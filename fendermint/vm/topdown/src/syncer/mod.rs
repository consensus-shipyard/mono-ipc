// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::observation::{LinearizedParentBlockView, Observation, ObservationConfig};
use crate::syncer::store::ParentViewStore;
use crate::{BlockHeight, Checkpoint};
use anyhow::anyhow;
use async_trait::async_trait;
use ipc_api::cross::IpcEnvelope;
use ipc_api::staking::StakingChangeRequest;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::select;
use tokio::sync::{broadcast, mpsc};

pub mod error;
pub mod payload;
pub mod poll;
pub mod store;

pub type QuorumCertContent = (Observation, Vec<IpcEnvelope>, Vec<StakingChangeRequest>);

#[derive(Clone, Debug)]
pub enum TopDownSyncEvent {
    /// The fendermint node is syncing with peers
    NodeSyncing,
    NewProposal(Box<Observation>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParentSyncerConfig {
    pub request_channel_size: usize,
    /// The event broadcast channel buffer size
    pub broadcast_channel_size: usize,
    /// The number of blocks to delay before reporting a height as final on the parent chain.
    /// To propose a certain number of epochs delayed from the latest height, we see to be
    /// conservative and avoid other from rejecting the proposal because they don't see the
    /// height as final yet.
    pub chain_head_delay: BlockHeight,
    /// Parent syncing cron period, in millis
    pub polling_interval_millis: Duration,
    /// Max number of requests to process in the reactor loop
    pub max_requests_per_loop: usize,
    /// Max number of un-finalized parent blocks that should be stored in the store
    pub max_store_blocks: BlockHeight,
    /// Attempts to sync as many block as possible till the finalized chain head
    pub sync_many: bool,

    pub observation: ObservationConfig,
}

#[derive(Clone)]
pub struct ParentSyncerReactorClient<S> {
    tx: mpsc::Sender<ParentSyncerRequest>,
    checkpoint: Arc<Mutex<Checkpoint>>,
    store: S,
}

impl<S: ParentViewStore + Send + Sync> ParentSyncerReactorClient<S> {
    pub fn new(
        request_channel_size: usize,
        store: S,
    ) -> (Self, mpsc::Receiver<ParentSyncerRequest>) {
        let (tx, rx) = mpsc::channel(request_channel_size);
        let checkpoint = Arc::new(Mutex::new(Checkpoint::v1(0, vec![], vec![])));
        (
            Self {
                tx,
                checkpoint,
                store,
            },
            rx,
        )
    }
}

pub fn start_polling_reactor<P: ParentPoller + Send + Sync + 'static>(
    mut rx: mpsc::Receiver<ParentSyncerRequest>,
    mut poller: P,
    config: ParentSyncerConfig,
) {
    let polling_interval = config.polling_interval_millis;
    tokio::spawn(async move {
        loop {
            select! {
                _ = tokio::time::sleep(polling_interval) => {
                    if let Err(e) = poller.try_poll().await {
                        tracing::error!(err = e.to_string(), "cannot sync with parent");
                    }
                }
                req = rx.recv() => {
                    let Some(req) = req else { break };
                    match req {
                        ParentSyncerRequest::Finalized(cp) => {
                            if let Err(e) = poller.finalize(cp) {
                                tracing::error!(err = e.to_string(), "cannot finalize syncer")
                            }
                        },
                    }
                }
            }
        }
    });
}

/// Polls the parent block view
#[async_trait]
pub trait ParentPoller {
    type Store: ParentViewStore + Send + Sync + 'static + Clone;

    fn subscribe(&self) -> broadcast::Receiver<TopDownSyncEvent>;

    fn store(&self) -> Self::Store;

    /// The target block height is finalized, purge all the parent view before the target height
    fn finalize(&mut self, checkpoint: Checkpoint) -> anyhow::Result<()>;

    /// Try to poll the next parent height
    async fn try_poll(&mut self) -> anyhow::Result<()>;
}

impl<S: ParentViewStore + Send + Sync + 'static> ParentSyncerReactorClient<S> {
    fn set_checkpoint(&self, cp: Checkpoint) {
        let mut checkpoint = self.checkpoint.lock().unwrap();
        *checkpoint = cp.clone();
    }
    /// Marks the height as finalized.
    /// There is no need to wait for ack from the reactor
    pub async fn finalize_parent_height(&self, cp: Checkpoint) -> anyhow::Result<()> {
        self.set_checkpoint(cp.clone());
        self.tx.send(ParentSyncerRequest::Finalized(cp)).await?;
        Ok(())
    }

    pub fn prepare_quorum_cert_content(
        &self,
        end_height: BlockHeight,
    ) -> anyhow::Result<QuorumCertContent> {
        let latest_checkpoint = self.checkpoint.lock().unwrap().clone();

        let mut xnet_msgs = vec![];
        let mut validator_changes = vec![];
        let mut linear = LinearizedParentBlockView::from(&latest_checkpoint);

        let start = latest_checkpoint.target_height() + 1;
        for h in start..=end_height {
            let Some(v) = self.store.get(h)? else {
                return Err(anyhow!("parent block view store does not have data at {h}"));
            };

            if let Err(e) = linear.append(v.clone()) {
                return Err(anyhow!("parent block view cannot be appended: {e}"));
            }

            if let Some(payload) = v.payload {
                xnet_msgs.extend(payload.xnet_msgs);
                validator_changes.extend(payload.validator_changes);
            }
        }

        let ob = linear
            .into_observation()
            .map_err(|e| anyhow!("cannot convert linearized parent view into observation: {e}"))?;

        Ok((ob, xnet_msgs, validator_changes))
    }
}

pub enum ParentSyncerRequest {
    /// A new parent height is finalized
    Finalized(Checkpoint),
}

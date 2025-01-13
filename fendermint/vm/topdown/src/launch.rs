// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::proxy::ParentQueryProxy;
use crate::syncer::store::InMemoryParentViewStore;
use crate::syncer::{
    start_polling_reactor, ParentPoller, ParentSyncerConfig, ParentSyncerReactorClient,
};
use crate::vote::gossip::{GossipReceiver, GossipSender};
use crate::vote::payload::PowerUpdates;
use crate::vote::store::InMemoryVoteStore;
use crate::vote::{StartVoteReactorParams, VoteReactorClient};
use crate::{BlockHeight, Checkpoint, Config, TopdownClient, TopdownProposal};
use anyhow::anyhow;
use cid::Cid;
use fendermint_crypto::SecretKey;
use fendermint_vm_genesis::{Power, Validator, ValidatorKey};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

/// Run the topdown checkpointing in the background. This consists of two processes:
/// - syncer:
///     - syncs with the parent through RPC endpoint to obtain:
///         - parent block hash/height
///         - topdown messages
///         - validator changes
///     - prepares for topdown observation to be braodcasted
/// - voting:
///     - signs/certifies and broadcast topdown observation to p2p peers
///     - listens to certified topdown observation from p2p
///     - aggregate peer certified observations into a quorum certificate for commitment in fendermint
pub async fn run_topdown<CheckpointQuery, GossipTx, GossipRx, Poller, ParentClient>(
    store: InMemoryParentViewStore,
    query: CheckpointQuery,
    config: Config,
    validator_key: SecretKey,
    gossip: (GossipTx, GossipRx),
    parent_client: ParentClient,
    create_poller_fn: impl FnOnce(&Checkpoint, ParentClient, ParentSyncerConfig) -> Poller
        + Send
        + 'static,
) -> anyhow::Result<TopdownClient>
where
    CheckpointQuery: LaunchQuery + Send + Sync + 'static,
    GossipTx: GossipSender + Send + Sync + 'static,
    GossipRx: GossipReceiver + Send + Sync + 'static,
    Poller: ParentPoller + Send + Sync + 'static,
    ParentClient: ParentQueryProxy + Send + Sync + 'static,
{
    let (syncer_client, syncer_rx) =
        ParentSyncerReactorClient::new(config.syncer.request_channel_size, store);
    let (voting_client, voting_rx) = VoteReactorClient::new(config.voting.req_channel_buffer_size);

    let syncer_client_cloned = syncer_client.clone();
    tokio::spawn(async move {
        let query = Arc::new(query);
        let checkpoint = query_starting_checkpoint(&query, &parent_client)
            .await
            .expect("should be able to query starting checkpoint");

        let power_table = query_starting_committee(&query)
            .await
            .expect("should be able to query starting committee");
        let power_table = power_table
            .into_iter()
            .map(|v| {
                let vk = ValidatorKey::new(v.public_key.0);
                let w = v.power.0;
                (vk, w)
            })
            .collect::<Vec<_>>();

        let poller = create_poller_fn(&checkpoint, parent_client, config.syncer.clone());
        let internal_event_rx = poller.subscribe();

        syncer_client_cloned
            .finalize_parent_height(checkpoint.clone())
            .await
            .expect("should be ok to set checkpoint");

        start_polling_reactor(syncer_rx, poller, config.syncer);
        VoteReactorClient::start_reactor(
            voting_rx,
            StartVoteReactorParams {
                config: config.voting,
                validator_key,
                power_table,
                last_finalized_height: checkpoint.target_height(),
                latest_child_block: query
                    .latest_chain_block()
                    .expect("should query latest chain block"),
                gossip_tx: gossip.0,
                gossip_rx: gossip.1,
                vote_store: InMemoryVoteStore::default(),
                internal_event_listener: internal_event_rx,
            },
        )
        .expect("cannot start vote reactor");

        tracing::info!(
            finality = checkpoint.to_string(),
            "launching parent syncer with last committed checkpoint"
        );
    });

    Ok(TopdownClient {
        syncer: syncer_client,
        voting: voting_client,
    })
}

/// Queries the starting finality for polling. First checks the committed finality, if none, that
/// means the chain has just started, then query from the parent to get the genesis epoch.
pub async fn query_starting_checkpoint<T, P>(
    query: &Arc<T>,
    parent_client: &P,
) -> anyhow::Result<Checkpoint>
where
    T: LaunchQuery + Send + Sync + 'static,
    P: ParentQueryProxy + Send + Sync + 'static,
{
    loop {
        let mut checkpoint = match query.get_latest_checkpoint() {
            Ok(Some(finality)) => finality,
            Ok(None) => {
                tracing::debug!("app not ready for query yet");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
            Err(e) => {
                tracing::warn!(error = e.to_string(), "cannot get committed finality");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };
        tracing::info!(
            checkpoint = checkpoint.to_string(),
            "latest checkpoint committed"
        );

        // this means there are no previous committed finality yet, we fetch from parent to get
        // the genesis epoch of the current subnet and its corresponding block hash.
        if checkpoint.target_height() == 0 {
            let genesis_epoch = parent_client.get_genesis_epoch().await?;
            tracing::debug!(genesis_epoch = genesis_epoch, "obtained genesis epoch");
            let r = parent_client.get_block_hash(genesis_epoch).await?;
            tracing::debug!(
                block_hash = hex::encode(&r.block_hash),
                "obtained genesis block hash",
            );

            checkpoint = Checkpoint::v1(genesis_epoch, r.block_hash, Cid::default().to_bytes());
            tracing::info!(
                genesis_checkpoint = checkpoint.to_string(),
                "no previous checkpoint committed, fetched from genesis epoch"
            );
        }

        return Ok(checkpoint);
    }
}

/// Queries the starting finality for polling. First checks the committed finality, if none, that
/// means the chain has just started, then query from the parent to get the genesis epoch.
pub async fn query_starting_committee<T>(query: &Arc<T>) -> anyhow::Result<Vec<Validator<Power>>>
where
    T: LaunchQuery + Send + Sync + 'static,
{
    loop {
        match query.get_power_table() {
            Ok(Some(power_table)) => return Ok(power_table),
            Ok(None) => {
                tracing::debug!("app not ready for query yet");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
            Err(e) => {
                tracing::warn!(error = e.to_string(), "cannot get comittee");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }
}

/// Query the chain for bootstrapping topdown
///
/// It returns `None` from queries until the ledger has been initialized.
pub trait LaunchQuery {
    /// Get the latest committed checkpoint from the state
    fn get_latest_checkpoint(&self) -> anyhow::Result<Option<Checkpoint>>;
    /// Get the current committee voting powers.
    fn get_power_table(&self) -> anyhow::Result<Option<Vec<Validator<Power>>>>;
    /// Get the latest blockchain height, the local/child subnet chain
    fn latest_chain_block(&self) -> anyhow::Result<BlockHeight>;
}

/// Toggle is needed for initialization because cyclic dependencies in fendermint bootstrap process.
/// Fendermint's App owns TopdownClient, but TopdownClient needs App for chain state.
/// Also Toggle is needed to handle non ipc enabled setups.
#[derive(Clone)]
pub struct Toggle<T> {
    inner: Option<T>,
}

impl<T> Toggle<T> {
    pub fn disable() -> Self {
        Self { inner: None }
    }

    pub fn enable(t: T) -> Self {
        Self { inner: Some(t) }
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_some()
    }

    async fn perform_or_err<
        'a,
        R,
        F: Future<Output = anyhow::Result<R>>,
        Fn: FnOnce(&'a T) -> F,
    >(
        &'a self,
        f: Fn,
    ) -> anyhow::Result<R> {
        let Some(ref inner) = self.inner else {
            return Err(anyhow!("topdown not enabled"));
        };
        f(inner).await
    }
}

impl Toggle<TopdownClient> {
    pub async fn validate_quorum_proposal(&self, proposal: TopdownProposal) -> anyhow::Result<()> {
        self.perform_or_err(|p| p.validate_quorum_proposal(proposal))
            .await
    }

    pub async fn find_topdown_proposal(&self) -> anyhow::Result<Option<TopdownProposal>> {
        self.perform_or_err(|p| p.find_topdown_proposal()).await
    }

    pub async fn parent_finalized(&self, checkpoint: Checkpoint) -> anyhow::Result<()> {
        self.perform_or_err(|p| p.parent_finalized(checkpoint))
            .await
    }

    pub async fn update_power_table(&self, updates: PowerUpdates) -> anyhow::Result<()> {
        self.perform_or_err(|p| p.update_power_table(updates)).await
    }
}

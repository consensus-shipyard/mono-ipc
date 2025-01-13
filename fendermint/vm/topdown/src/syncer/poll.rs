// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::observation::deduce_new_observation;
use crate::observe::ParentFinalityAcquired;
use crate::proxy::ParentQueryProxy;
use crate::syncer::error::Error;
use crate::syncer::payload::ParentBlockView;
use crate::syncer::store::ParentViewStore;
use crate::syncer::{ParentPoller, ParentSyncerConfig, TopDownSyncEvent};
use crate::{is_null_round_str, BlockHash, BlockHeight, Checkpoint};
use anyhow::anyhow;
use async_trait::async_trait;
use ipc_observability::emit;
use ipc_observability::serde::HexEncodableBlockHash;
use libp2p::futures::TryFutureExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tracing::instrument;

pub struct ParentPoll<P, S> {
    config: ParentSyncerConfig,
    parent_proxy: P,
    store: S,
    event_broadcast: broadcast::Sender<TopDownSyncEvent>,
    last_finalized: Checkpoint,
}

#[async_trait]
impl<P, S> ParentPoller for ParentPoll<P, S>
where
    S: ParentViewStore + Send + Sync + 'static + Clone,
    P: Send + Sync + 'static + ParentQueryProxy,
{
    type Store = S;

    fn subscribe(&self) -> Receiver<TopDownSyncEvent> {
        self.event_broadcast.subscribe()
    }

    fn store(&self) -> Self::Store {
        self.store.clone()
    }

    /// The target block height is finalized, purge all the parent view before the target height
    fn finalize(&mut self, checkpoint: Checkpoint) -> anyhow::Result<()> {
        let Some(min_height) = self.store.min_parent_view_height()? else {
            return Ok(());
        };
        for h in min_height..=checkpoint.target_height() {
            self.store.purge(h)?;
        }

        self.last_finalized = checkpoint;

        Ok(())
    }

    /// Insert the height into cache when we see a new non null block
    async fn try_poll(&mut self) -> anyhow::Result<()> {
        let Some(chain_head) = self.finalized_chain_head().await? else {
            return Ok(());
        };

        let (mut latest_height_fetched, mut first_non_null_parent_hash) =
            self.latest_nonnull_data()?;
        tracing::debug!(chain_head, latest_height_fetched, "syncing heights");

        if latest_height_fetched > chain_head {
            tracing::warn!(
                chain_head,
                latest_height_fetched,
                "chain head went backwards, potential reorg detected from height"
            );
            todo!("handle reorg, maybe just a warning???")
        }

        if latest_height_fetched == chain_head {
            tracing::debug!(
                chain_head,
                latest_height_fetched,
                "the parent has yet to produce a new block"
            );
            return Ok(());
        }

        loop {
            if self.store_full()? {
                tracing::debug!("exceeded cache size limit");
                break;
            }

            first_non_null_parent_hash = match self
                .poll_next(latest_height_fetched + 1, first_non_null_parent_hash)
                .await
            {
                Ok(h) => h,
                Err(Error::ParentChainReorgDetected) => {
                    tracing::warn!("potential reorg detected, clear cache and retry");
                    todo!();
                    // break;
                }
                Err(e) => return Err(anyhow!(e)),
            };

            latest_height_fetched += 1;

            if latest_height_fetched == chain_head {
                tracing::debug!("reached the tip of the chain");
                break;
            } else if !self.config.sync_many {
                break;
            }
        }

        Ok(())
    }
}

impl<P, S> ParentPoll<P, S>
where
    S: ParentViewStore + Send + Sync + 'static,
    P: Send + Sync + 'static + ParentQueryProxy,
{
    pub fn new(config: ParentSyncerConfig, proxy: P, store: S, last_finalized: Checkpoint) -> Self {
        let (tx, _) = broadcast::channel(config.broadcast_channel_size);
        Self {
            config,
            parent_proxy: proxy,
            store,
            event_broadcast: tx,
            last_finalized,
        }
    }

    /// Get the latest non null block data stored
    fn latest_nonnull_data(&self) -> anyhow::Result<(BlockHeight, BlockHash)> {
        let Some(latest_height) = self.store.max_parent_view_height()? else {
            return Ok((
                self.last_finalized.target_height(),
                self.last_finalized.target_hash().clone(),
            ));
        };

        let start = self.last_finalized.target_height() + 1;
        for h in (start..=latest_height).rev() {
            let Some(p) = self.store.get(h)? else {
                continue;
            };

            // if parent hash of the proposal is null, it means the
            let Some(p) = p.payload else {
                continue;
            };

            return Ok((h, p.parent_hash));
        }

        // this means the votes stored are all null blocks, return last committed finality
        Ok((
            self.last_finalized.target_height(),
            self.last_finalized.target_hash().clone(),
        ))
    }

    fn store_full(&self) -> anyhow::Result<bool> {
        let Some(h) = self.store.max_parent_view_height()? else {
            return Ok(false);
        };
        Ok(h - self.last_finalized.target_height() > self.config.max_store_blocks)
    }

    async fn finalized_chain_head(&self) -> anyhow::Result<Option<BlockHeight>> {
        let parent_chain_head_height = self.parent_proxy.get_chain_head_height().await?;
        // sanity check
        if parent_chain_head_height < self.config.chain_head_delay {
            tracing::debug!("latest height not more than the chain head delay");
            return Ok(None);
        }

        // we consider the chain head finalized only after the `chain_head_delay`
        Ok(Some(
            parent_chain_head_height - self.config.chain_head_delay,
        ))
    }

    /// Poll the next block height. Returns finalized and executed block data.
    async fn poll_next(
        &mut self,
        height: BlockHeight,
        parent_block_hash: BlockHash,
    ) -> Result<BlockHash, Error> {
        tracing::debug!(
            height,
            parent_block_hash = hex::encode(&parent_block_hash),
            "polling height with parent hash"
        );

        let block_hash_res = match self.parent_proxy.get_block_hash(height).await {
            Ok(res) => res,
            Err(e) => {
                let err = e.to_string();
                if is_null_round_str(&err) {
                    tracing::debug!(
                        height,
                        "detected null round at height, inserted None to cache"
                    );

                    self.store.store(ParentBlockView::null_block(height))?;

                    // self.store.store(ParentView::null_block(height))?;

                    emit(ParentFinalityAcquired {
                        source: "Parent syncer",
                        is_null: true,
                        block_height: height,
                        block_hash: None,
                        commitment_hash: None,
                        num_msgs: 0,
                        num_validator_changes: 0,
                    });

                    // Null block received, no block hash for the current height being polled.
                    // Return the previous parent hash as the non-null block hash.
                    return Ok(parent_block_hash);
                }
                return Err(Error::CannotQueryParent(
                    format!("get_block_hash: {e}"),
                    height,
                ));
            }
        };

        if block_hash_res.parent_block_hash != parent_block_hash {
            tracing::warn!(
                height,
                parent_hash = hex::encode(&block_hash_res.parent_block_hash),
                previous_hash = hex::encode(&parent_block_hash),
                "parent block hash diff than previous hash",
            );
            return Err(Error::ParentChainReorgDetected);
        }

        let view = fetch_data(&self.parent_proxy, height, block_hash_res.block_hash).await?;

        self.store.store(view.clone())?;
        let commitment =
            deduce_new_observation(&self.store, &self.last_finalized, &self.config.observation)?;
        // if there is an error, ignore, we can always try next loop
        let _ = self
            .event_broadcast
            .send(TopDownSyncEvent::NewProposal(Box::new(commitment)));

        let payload = view.payload.as_ref().unwrap();
        emit(ParentFinalityAcquired {
            source: "Parent syncer",
            is_null: false,
            block_height: height,
            block_hash: Some(HexEncodableBlockHash(payload.parent_hash.clone())),
            // TODO Karel, Willes - when we introduce commitment hash, we should add it here
            commitment_hash: None,
            num_msgs: payload.xnet_msgs.len(),
            num_validator_changes: payload.validator_changes.len(),
        });

        Ok(view.payload.unwrap().parent_hash)
    }
}

#[instrument(skip(parent_proxy))]
async fn fetch_data<P>(
    parent_proxy: &P,
    height: BlockHeight,
    block_hash: BlockHash,
) -> Result<ParentBlockView, Error>
where
    P: ParentQueryProxy + Send + Sync + 'static,
{
    let changes_res = parent_proxy
        .get_validator_changes(height)
        .map_err(|e| Error::CannotQueryParent(format!("get_validator_changes: {e}"), height));

    let topdown_msgs_res = parent_proxy
        .get_top_down_msgs(height)
        .map_err(|e| Error::CannotQueryParent(format!("get_top_down_msgs: {e}"), height));

    let (changes_res, topdown_msgs_res) = tokio::join!(changes_res, topdown_msgs_res);
    let (changes_res, topdown_msgs_res) = (changes_res?, topdown_msgs_res?);

    if changes_res.block_hash != block_hash {
        tracing::warn!(
            height,
            change_set_hash = hex::encode(&changes_res.block_hash),
            block_hash = hex::encode(&block_hash),
            "change set block hash does not equal block hash",
        );
        return Err(Error::ParentChainReorgDetected);
    }

    if topdown_msgs_res.block_hash != block_hash {
        tracing::warn!(
            height,
            topdown_msgs_hash = hex::encode(&topdown_msgs_res.block_hash),
            block_hash = hex::encode(&block_hash),
            "topdown messages block hash does not equal block hash",
        );
        return Err(Error::ParentChainReorgDetected);
    }

    Ok(ParentBlockView::nonnull_block(
        height,
        block_hash,
        topdown_msgs_res.value,
        changes_res.value,
    ))
}

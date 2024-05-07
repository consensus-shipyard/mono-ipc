// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::finality::{
    ensure_sequential, topdown_cross_msgs, validator_changes, ParentViewPayload,
};
use crate::{
    BlockHash, BlockHeight, CacheStore, Config, Error, IPCParentFinality, SequentialKeyCache,
};
use async_stm::{abort, atomically_or_err, Stm, StmError, StmResult, TVar};
use ipc_api::cross::IpcEnvelope;
use ipc_api::staking::StakingChangeRequest;
use std::cmp::min;

use fendermint_tracing::emit;
use fendermint_vm_event::ParentFinalityCommitted;

/// Finality provider that can handle null blocks
#[derive(Clone)]
pub struct FinalityWithNull {
    config: Config,
    genesis_epoch: BlockHeight,
    /// Cached data that always syncs with the latest parent chain proactively
    cached_data: TVar<SequentialKeyCache<BlockHeight, Option<ParentViewPayload>>>,
    /// This is a in memory view of the committed parent finality. We need this as a starting point
    /// for populating the cache
    last_committed_finality: TVar<Option<IPCParentFinality>>,
    cache_store: CacheStore,
}

impl FinalityWithNull {
    pub fn new(
        config: Config,
        genesis_epoch: BlockHeight,
        committed_finality: Option<IPCParentFinality>,
        cache_store: CacheStore,
    ) -> Self {
        Self {
            config,
            genesis_epoch,
            cached_data: TVar::new(SequentialKeyCache::sequential()),
            last_committed_finality: TVar::new(committed_finality),
            cache_store,
        }
    }

    pub fn genesis_epoch(&self) -> anyhow::Result<BlockHeight> {
        Ok(self.genesis_epoch)
    }

    pub async fn validator_changes(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<Option<Vec<StakingChangeRequest>>> {
        let r = atomically_or_err(|| self.handle_null_block(height, validator_changes, Vec::new))
            .await?;
        Ok(r)
    }

    pub async fn top_down_msgs(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<Option<Vec<IpcEnvelope>>> {
        let r = atomically_or_err(|| self.handle_null_block(height, topdown_cross_msgs, Vec::new))
            .await?;
        Ok(r)
    }

    pub fn last_committed_finality(&self) -> Stm<Option<IPCParentFinality>> {
        self.last_committed_finality.read_clone()
    }

    /// Clear the cache and set the committed finality to the provided value
    pub fn reset(&self, finality: IPCParentFinality) -> StmResult<(), Error> {
        self.cached_data.write(SequentialKeyCache::sequential())?;
        self.cache_store
            .delete_all()
            .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))?;
        tracing::info!("cache cleared");
        Ok(self.last_committed_finality.write(Some(finality))?)
    }

    pub fn new_parent_view(
        &self,
        height: BlockHeight,
        maybe_payload: Option<ParentViewPayload>,
    ) -> StmResult<(), Error> {
        if let Some((block_hash, validator_changes, top_down_msgs)) = maybe_payload {
            self.parent_block_filled(height, block_hash, validator_changes, top_down_msgs)
        } else {
            self.parent_null_round(height)
        }
    }

    pub fn next_proposal(&self) -> StmResult<Option<IPCParentFinality>, Error> {
        let height = if let Some(h) = self.propose_next_height()? {
            h
        } else {
            return Ok(None);
        };

        // safe to unwrap as we make sure null height will not be proposed
        let block_hash = self.block_hash_at_height(height)?.unwrap();

        let proposal = IPCParentFinality { height, block_hash };
        tracing::debug!(proposal = proposal.to_string(), "new proposal");
        Ok(Some(proposal))
    }

    pub fn check_proposal(&self, proposal: &IPCParentFinality) -> StmResult<bool, Error> {
        if !self.check_height(proposal)? {
            return Ok(false);
        }
        self.check_block_hash(proposal)
    }

    pub fn set_new_finality(
        &self,
        finality: IPCParentFinality,
        previous_finality: Option<IPCParentFinality>,
    ) -> StmResult<(), Error> {
        debug_assert!(previous_finality == self.last_committed_finality.read_clone()?);

        // the height to clear
        let height = finality.height;

        self.cached_data.update(|mut cache| {
            // only remove cache below height, but not at height, as we have delayed execution
            cache.remove_key_below(height);
            cache
        })?;
        self.cache_store
            .delete_key_below(height)
            .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))?;
        tracing::info!(height, "cache cleared below height");

        let hash = hex::encode(&finality.block_hash);

        self.last_committed_finality.write(Some(finality))?;

        // emit event only after successful write
        emit!(ParentFinalityCommitted {
            block_height: height,
            block_hash: &hash
        });

        Ok(())
    }
}

impl FinalityWithNull {
    /// Returns the number of blocks cached.
    pub(crate) fn cached_blocks(&self) -> StmResult<BlockHeight, Error> {
        let cache_size = self.cached_data.read()?.size();
        let store_size = self
            .cache_store
            .size()
            .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))?;
        tracing::info!(cache_size, store_size, "COMPARE cached_blocks");

        if cache_size != store_size {
            panic!("cached_blocks mismatch: {} !=  {}", cache_size, store_size);
        }

        Ok(cache_size as BlockHeight)
    }

    pub(crate) fn block_hash_at_height(
        &self,
        height: BlockHeight,
    ) -> StmResult<Option<BlockHash>, Error> {
        if let Some(f) = self.last_committed_finality.read()?.as_ref() {
            if f.height == height {
                return Ok(Some(f.block_hash.clone()));
            }
        }

        self.get_at_height(height, |i| i.0.clone())
    }

    pub(crate) fn latest_height_in_cache(&self) -> StmResult<Option<BlockHeight>, Error> {
        let cache_upper_bound = self.cached_data.read()?.upper_bound();
        let store_upper_bound = self
            .cache_store
            .upper_bound()
            .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))?;
        tracing::info!(
            cache_upper_bound,
            store_upper_bound,
            "COMPARE  latest_height_in_cache"
        );
        if cache_upper_bound != store_upper_bound {
            panic!(
                "latest_height_in_cache mismatch: {:?} !=  {:?}",
                cache_upper_bound, store_upper_bound
            );
        }
        Ok(cache_upper_bound)
    }

    /// Get the latest height tracked in the provider, includes both cache and last committed finality
    pub(crate) fn latest_height(&self) -> StmResult<Option<BlockHeight>, Error> {
        let h = if let Some(h) = self.latest_height_in_cache()? {
            h
        } else if let Some(p) = self.last_committed_finality()? {
            p.height
        } else {
            return Ok(None);
        };
        Ok(Some(h))
    }

    /// Get the first non-null block in the range of earliest cache block till the height specified, inclusive.
    pub(crate) fn first_non_null_block(
        &self,
        height: BlockHeight,
    ) -> StmResult<Option<BlockHeight>, Error> {
        let cache = self.cached_data.read()?;

        let mut cached_height = 0;

        let res = Ok(cache.lower_bound().and_then(|lower_bound| {
            for h in (lower_bound..=height).rev() {
                if let Some(Some(_)) = cache.get_value(h) {
                    cached_height = h;
                    return Some(h);
                }
            }
            None
        }));

        let mut stored_height = 0;

        let _res2: std::result::Result<std::option::Option<u64>, Vec<u8>> = Ok(self
            .cache_store
            .lower_bound()
            .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))?
            .and_then(|lower_bound| {
                for h in (lower_bound..=height).rev() {
                    if let Ok(Some(Some(_))) = self
                        .cache_store
                        .get_value(h)
                        .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))
                    {
                        stored_height = h;
                        return Some(h);
                    }
                }
                None
            }));

        tracing::info!(cached_height, stored_height, "COMPARE first_non_null_block");

        if cached_height != stored_height {
            panic!(
                "first_non_null_block mismatch: {} !=  {}",
                cached_height, stored_height
            );
        }

        res
    }
}

/// All the private functions
impl FinalityWithNull {
    fn propose_next_height(&self) -> StmResult<Option<BlockHeight>, Error> {
        let latest_height = if let Some(h) = self.latest_height_in_cache()? {
            h
        } else {
            tracing::debug!("no proposal yet as height not available");
            return Ok(None);
        };

        let last_committed_height = if let Some(h) = self.last_committed_finality.read_clone()? {
            h.height
        } else {
            unreachable!("last committed finality will be available at this point");
        };

        let max_proposal_height = last_committed_height + self.config.max_proposal_range();
        let candidate_height = min(max_proposal_height, latest_height);
        tracing::debug!(max_proposal_height, candidate_height, "propose heights");

        let first_non_null_height = if let Some(h) = self.first_non_null_block(candidate_height)? {
            h
        } else {
            tracing::debug!(height = candidate_height, "no non-null block found before");
            return Ok(None);
        };

        tracing::debug!(first_non_null_height, candidate_height);
        // an extra layer of delay
        let maybe_proposal_height =
            self.first_non_null_block(first_non_null_height - self.config.proposal_delay())?;
        tracing::debug!(
            delayed_height = maybe_proposal_height,
            delay = self.config.proposal_delay()
        );
        if let Some(proposal_height) = maybe_proposal_height {
            // this is possible due to delayed execution as the proposed height's data cannot be
            // executed because they have yet to be executed.
            return if last_committed_height == proposal_height {
                tracing::debug!(
                    last_committed_height,
                    proposal_height,
                    "no new blocks from cache, not proposing"
                );
                Ok(None)
            } else {
                tracing::debug!(proposal_height, "new proposal height");
                Ok(Some(proposal_height))
            };
        }

        tracing::debug!(last_committed_height, "no non-null block after delay");
        Ok(None)
    }

    fn handle_null_block<T, F: Fn(&ParentViewPayload) -> T, D: Fn() -> T>(
        &self,
        height: BlockHeight,
        f: F,
        d: D,
    ) -> StmResult<Option<T>, Error> {
        let cache = self.cached_data.read()?;

        let mut cache_value = None;

        let res = Ok(cache.get_value(height).map(|v| {
            if let Some(i) = v.as_ref() {
                cache_value = Some(i.clone());
                f(i)
            } else {
                tracing::debug!(height, "a null round detected, return default");
                d()
            }
        }));

        let mut stored_value = None;
        let _ = self
            .cache_store
            .get_value(height)
            .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))?
            .map(|v| {
                if let Some(i) = v.as_ref() {
                    stored_value = Some(i.clone());
                    f(i)
                } else {
                    tracing::debug!(height, "a null round detected, return default");
                    d()
                }
            });

        tracing::info!(?cache_value, ?stored_value, "COMPARE handle_null_block");

        if cache_value.is_some() || stored_value.is_some() {
            if cache_value.unwrap().2 != stored_value.unwrap().2 {
                panic!("handle_null_block mismatch");
            }
        }

        res
    }

    fn get_at_height<T, F: Fn(&ParentViewPayload) -> T>(
        &self,
        height: BlockHeight,
        f: F,
    ) -> StmResult<Option<T>, Error> {
        let cache = self.cached_data.read()?;

        let mut cache_value = None;
        let res = Ok(if let Some(Some(v)) = cache.get_value(height) {
            cache_value = Some(v.clone());
            Some(f(v))
        } else {
            None
        });

        let mut stored_value = None;
        if let Some(Some(v)) = self
            .cache_store
            .get_value(height)
            .map_err(|e| StmError::Abort(Error::CacheStoreError(e.to_string())))?
        {
            stored_value = Some(v.clone());
        }

        tracing::info!(?cache_value, ?stored_value, "COMPARE get_at_height");

        res
    }

    fn parent_block_filled(
        &self,
        height: BlockHeight,
        block_hash: BlockHash,
        validator_changes: Vec<StakingChangeRequest>,
        top_down_msgs: Vec<IpcEnvelope>,
    ) -> StmResult<(), Error> {
        if !top_down_msgs.is_empty() {
            // make sure incoming top down messages are ordered by nonce sequentially
            tracing::debug!(?top_down_msgs);
            ensure_sequential(&top_down_msgs, |msg| msg.nonce)?;
        };
        if !validator_changes.is_empty() {
            tracing::debug!(?validator_changes, "validator changes");
            ensure_sequential(&validator_changes, |change| change.configuration_number)?;
        }

        let r = self.cached_data.modify(|mut cache| {
            let r = cache
                .append(
                    height,
                    Some((
                        block_hash.clone(),
                        validator_changes.clone(),
                        top_down_msgs.clone(),
                    )),
                )
                .map_err(Error::NonSequentialParentViewInsert);
            (cache, r)
        })?;

        if let Err(e) = r {
            return abort(e);
        }

        let r2 = self
            .cache_store
            .append(height, Some((block_hash, validator_changes, top_down_msgs)))
            .map_err(|e| Error::CacheStoreError(e.to_string()));
        if let Err(e) = r2 {
            return abort(e);
        }

        Ok(())
    }

    /// When there is a new parent view, but it is actually a null round, call this function.
    fn parent_null_round(&self, height: BlockHeight) -> StmResult<(), Error> {
        let r = self.cached_data.modify(|mut cache| {
            let r = cache
                .append(height, None)
                .map_err(Error::NonSequentialParentViewInsert);
            (cache, r)
        })?;

        if let Err(e) = r {
            return abort(e);
        }

        let r2 = self
            .cache_store
            .append(height, None)
            .map_err(|e| Error::CacheStoreError(e.to_string()));
        if let Err(e) = r2 {
            return abort(e);
        }

        Ok(())
    }

    fn check_height(&self, proposal: &IPCParentFinality) -> StmResult<bool, Error> {
        let binding = self.last_committed_finality.read()?;
        // last committed finality is not ready yet, we don't vote, just reject
        let last_committed_finality = if let Some(f) = binding.as_ref() {
            f
        } else {
            return Ok(false);
        };

        // the incoming proposal has height already committed, reject
        if last_committed_finality.height >= proposal.height {
            tracing::debug!(
                last_committed = last_committed_finality.height,
                proposed = proposal.height,
                "proposed height already committed",
            );
            return Ok(false);
        }

        if let Some(latest_height) = self.latest_height_in_cache()? {
            let r = latest_height >= proposal.height;
            tracing::debug!(
                is_true = r,
                latest_height,
                proposal = proposal.height.to_string(),
                "incoming proposal height seen?"
            );
            // requires the incoming height cannot be more advanced than our trusted parent node
            Ok(r)
        } else {
            // latest height is not found, meaning we dont have any prefetched cache, we just be
            // strict and vote no simply because we don't know.
            tracing::debug!(
                proposal = proposal.height.to_string(),
                "reject proposal, no data in cache"
            );
            Ok(false)
        }
    }

    fn check_block_hash(&self, proposal: &IPCParentFinality) -> StmResult<bool, Error> {
        Ok(
            if let Some(block_hash) = self.block_hash_at_height(proposal.height)? {
                let r = block_hash == proposal.block_hash;
                tracing::debug!(proposal = proposal.to_string(), is_same = r, "same hash?");
                r
            } else {
                tracing::debug!(proposal = proposal.to_string(), "reject, hash not found");
                false
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::FinalityWithNull;
    use crate::finality::ParentViewPayload;
    use crate::{BlockHeight, CacheStore, Config, IPCParentFinality};
    use async_stm::{atomically, atomically_or_err};

    async fn new_provider(
        mut blocks: Vec<(BlockHeight, Option<ParentViewPayload>)>,
    ) -> FinalityWithNull {
        let config = Config {
            chain_head_delay: 2,
            polling_interval: Default::default(),
            exponential_back_off: Default::default(),
            exponential_retry_limit: 0,
            max_proposal_range: Some(6),
            max_cache_blocks: None,
            proposal_delay: Some(2),
        };
        let committed_finality = IPCParentFinality {
            height: blocks[0].0,
            block_hash: vec![0; 32],
        };

        blocks.remove(0);

        let cache_store = CacheStore::new_test("test".to_string()).unwrap();
        let f = FinalityWithNull::new(config, 1, Some(committed_finality), cache_store);
        for (h, p) in blocks {
            atomically_or_err(|| f.new_parent_view(h, p.clone()))
                .await
                .unwrap();
        }
        f
    }

    #[tokio::test]
    async fn test_happy_path() {
        // max_proposal_range is 6. proposal_delay is 2
        let parent_blocks = vec![
            (100, Some((vec![0; 32], vec![], vec![]))), // last committed block
            (101, Some((vec![1; 32], vec![], vec![]))), // cache start
            (102, Some((vec![2; 32], vec![], vec![]))),
            (103, Some((vec![3; 32], vec![], vec![]))),
            (104, Some((vec![4; 32], vec![], vec![]))), // final delayed height + proposal height
            (105, Some((vec![5; 32], vec![], vec![]))),
            (106, Some((vec![6; 32], vec![], vec![]))), // max proposal height (last committed + 6), first non null block
            (107, Some((vec![7; 32], vec![], vec![]))), // cache latest height
        ];
        let provider = new_provider(parent_blocks).await;

        let f = IPCParentFinality {
            height: 104,
            block_hash: vec![4; 32],
        };
        assert_eq!(
            atomically_or_err(|| provider.next_proposal())
                .await
                .unwrap(),
            Some(f.clone())
        );

        // Test set new finality
        atomically_or_err(|| {
            let last = provider.last_committed_finality.read_clone()?;
            provider.set_new_finality(f.clone(), last)
        })
        .await
        .unwrap();

        assert_eq!(
            atomically(|| provider.last_committed_finality()).await,
            Some(f.clone())
        );

        // this ensures sequential insertion is still valid
        atomically_or_err(|| provider.new_parent_view(108, None))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_not_enough_view() {
        // max_proposal_range is 6. proposal_delay is 2
        let parent_blocks = vec![
            (100, Some((vec![0; 32], vec![], vec![]))), // last committed block
            (101, Some((vec![1; 32], vec![], vec![]))),
            (102, Some((vec![2; 32], vec![], vec![]))),
            (103, Some((vec![3; 32], vec![], vec![]))), // delayed height + final height
            (104, Some((vec![4; 32], vec![], vec![]))),
            (105, Some((vec![4; 32], vec![], vec![]))), // cache latest height, first non null block
                                                        // max proposal height is 106
        ];
        let provider = new_provider(parent_blocks).await;

        assert_eq!(
            atomically_or_err(|| provider.next_proposal())
                .await
                .unwrap(),
            Some(IPCParentFinality {
                height: 103,
                block_hash: vec![3; 32]
            })
        );
    }

    #[tokio::test]
    async fn test_with_all_null_blocks() {
        // max_proposal_range is 10. proposal_delay is 2
        let parent_blocks = vec![
            (102, Some((vec![2; 32], vec![], vec![]))), // last committed block
            (103, None),
            (104, None),
            (105, None),
            (106, None),
            (107, None),
            (108, None),
            (109, None),
            (110, Some((vec![4; 32], vec![], vec![]))), // cache latest height
                                                        // max proposal height is 112
        ];
        let mut provider = new_provider(parent_blocks).await;
        provider.config.max_proposal_range = Some(8);

        assert_eq!(
            atomically_or_err(|| provider.next_proposal())
                .await
                .unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_with_partially_null_blocks_i() {
        // max_proposal_range is 10. proposal_delay is 2
        let parent_blocks = vec![
            (102, Some((vec![2; 32], vec![], vec![]))), // last committed block
            (103, None),
            (104, None), // we wont have a proposal because after delay, there is no more non-null proposal
            (105, None),
            (106, None),
            (107, None),
            (108, None), // delayed block
            (109, Some((vec![8; 32], vec![], vec![]))),
            (110, Some((vec![10; 32], vec![], vec![]))), // cache latest height, first non null block
                                                         // max proposal height is 112
        ];
        let mut provider = new_provider(parent_blocks).await;
        provider.config.max_proposal_range = Some(10);

        assert_eq!(
            atomically_or_err(|| provider.next_proposal())
                .await
                .unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_with_partially_null_blocks_ii() {
        // max_proposal_range is 10. proposal_delay is 2
        let parent_blocks = vec![
            (102, Some((vec![2; 32], vec![], vec![]))), // last committed block
            (103, Some((vec![3; 32], vec![], vec![]))),
            (104, None),
            (105, None),
            (106, None),
            (107, Some((vec![7; 32], vec![], vec![]))), // first non null after delay
            (108, None),                                // delayed block
            (109, None),
            (110, Some((vec![10; 32], vec![], vec![]))), // cache latest height, first non null block
                                                         // max proposal height is 112
        ];
        let mut provider = new_provider(parent_blocks).await;
        provider.config.max_proposal_range = Some(10);

        assert_eq!(
            atomically_or_err(|| provider.next_proposal())
                .await
                .unwrap(),
            Some(IPCParentFinality {
                height: 107,
                block_hash: vec![7; 32]
            })
        );
    }

    #[tokio::test]
    async fn test_with_partially_null_blocks_iii() {
        let parent_blocks = vec![
            (102, Some((vec![2; 32], vec![], vec![]))), // last committed block
            (103, Some((vec![3; 32], vec![], vec![]))),
            (104, None),
            (105, None),
            (106, None),
            (107, Some((vec![7; 32], vec![], vec![]))), // first non null delayed block, final
            (108, None),                                // delayed block
            (109, None),
            (110, Some((vec![10; 32], vec![], vec![]))), // first non null block
            (111, None),
            (112, None),
            // max proposal height is 122
        ];
        let mut provider = new_provider(parent_blocks).await;
        provider.config.max_proposal_range = Some(20);

        assert_eq!(
            atomically_or_err(|| provider.next_proposal())
                .await
                .unwrap(),
            Some(IPCParentFinality {
                height: 107,
                block_hash: vec![7; 32]
            })
        );
    }
}

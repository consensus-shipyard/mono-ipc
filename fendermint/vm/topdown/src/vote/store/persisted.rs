// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::vote::error::Error;
use crate::vote::payload::Vote;
use crate::vote::store::{InMemoryVoteStore, VoteAgg, VoteStore};
use crate::BlockHeight;
use fendermint_vm_genesis::ValidatorKey;
use rocksdb::{BoundColumnFamily, IteratorMode, OptimisticTransactionDB};
use std::sync::Arc;

/// The persisted store that stores the vote received from other peers.
/// Internally, it's making use of a in memory store for faster query response.
pub struct PersistedVoteStore {
    /// The in memory vote store for faster look ups
    in_memory_store: InMemoryVoteStore,
    /// The persisted vote store for storing votes in disk
    persisted_store: Arc<OptimisticTransactionDB>,
    /// The namespace for the persisted store
    ns: String,
}

impl PersistedVoteStore {
    pub fn new(db: Arc<OptimisticTransactionDB>, ns: String) -> Result<Self, Error> {
        // All namespaces are pre-created during open.
        if db.cf_handle(&ns).is_none() {
            return Err(Error::PersistentVoteStoreNoNamespace(ns.to_string()));
        }

        let mut memory = InMemoryVoteStore::default();

        let cf = get_cf(&db, &ns)?;
        let iter = db.iterator_cf(&cf, IteratorMode::Start);
        for item in iter {
            let (key, value) = item.map_err(|e| Error::PersistentVoteStore(Box::new(e)))?;
            let raw_key = key.get(0..8).ok_or_else(|| Error::PersistentVoteStoreInvalidKeyLength(hex::encode(key.as_ref())))?;
            let height = BlockHeight::from_be_bytes(
                raw_key
                    .try_into()
                    .map_err(|e| Error::PersistentVoteStore(Box::new(e)))?,
            );

            let vote = fvm_ipld_encoding::from_slice(value.as_ref()).map_err(|e| {
                Error::PersistentVoteStore(Box::new(e))
            })?;

            memory.store_vote(height, vote)?;
        }
        drop(cf);

        Ok(Self {
            in_memory_store: memory,
            persisted_store: db,
            ns,
        })
    }
}

impl VoteStore for PersistedVoteStore {
    fn earliest_vote_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.in_memory_store.earliest_vote_height()
    }

    fn latest_vote_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.in_memory_store.latest_vote_height()
    }

    fn store_vote(&mut self, height: BlockHeight, vote: Vote) -> Result<(), Error> {
        let bytes = fvm_ipld_encoding::to_vec(&vote)
            .map_err(|e| Error::PersistentVoteStore(Box::new(e)))?;

        let cf = get_cf(self.persisted_store.as_ref(), self.ns.as_str())?;
        self.persisted_store
            .put_cf(&cf, height.to_be_bytes(), bytes)
            .map_err(|e| Error::PersistentVoteStore(Box::new(e)))?;
        self.in_memory_store.store_vote(height, vote)
    }

    fn has_voted(&self, height: &BlockHeight, validator: &ValidatorKey) -> Result<bool, Error> {
        self.in_memory_store.has_voted(height, validator)
    }

    fn get_votes_at_height(&self, height: BlockHeight) -> Result<VoteAgg, Error> {
        self.in_memory_store.get_votes_at_height(height)
    }

    fn purge_votes_at_height(&mut self, height: BlockHeight) -> Result<(), Error> {
        let cf = get_cf(self.persisted_store.as_ref(), self.ns.as_str())?;
        self.persisted_store
            .delete_cf(&cf, height.to_be_bytes())
            .map_err(|e| Error::PersistentVoteStore(Box::new(e)))?;
        self.in_memory_store.purge_votes_at_height(height)
    }
}

pub(crate) fn get_cf<'a>(
    db: &'a OptimisticTransactionDB,
    ns: &'a str,
) -> Result<Arc<BoundColumnFamily<'a>>, Error> {
    db.cf_handle(ns)
        .ok_or_else(|| Error::PersistentVoteStoreNoNamespace(ns.to_string()))
}

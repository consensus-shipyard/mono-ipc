// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::vote::error::Error;
use crate::vote::payload::Vote;
use crate::vote::store::{InMemoryVoteStore, VoteAgg, VoteStore};
use crate::BlockHeight;
use fendermint_vm_genesis::ValidatorKey;
use rocksdb::{BoundColumnFamily, IteratorMode, OptimisticTransactionDB};
use std::sync::Arc;

pub struct PersistedVoteStore {
    cache: InMemoryVoteStore,
    db: Arc<OptimisticTransactionDB>,
    ns: String,
}

impl PersistedVoteStore {
    pub fn new(db: Arc<OptimisticTransactionDB>, ns: String) -> Result<Self, Error> {
        // All namespaces are pre-created during open.
        if db.cf_handle(&ns).is_none() {
            return Err(Error::PersistentVoteStore(format!(
                "namespace {ns} does not exist"
            )));
        }

        let mut memory = InMemoryVoteStore::default();

        let cf = get_cf(&db, &ns)?;
        let iter = db.iterator_cf(&cf, IteratorMode::Start);
        for item in iter {
            let (key, value) = item.map_err(|e| Error::PersistentVoteStore(format!("{e}")))?;
            let height = BlockHeight::from_be_bytes(
                key[0..8]
                    .try_into()
                    .map_err(|e| Error::PersistentVoteStore(format!("{e}")))?,
            );

            let vote = fvm_ipld_encoding::from_slice(value.as_ref()).map_err(|e| {
                Error::PersistentVoteStore(format!("cannot convert value to vote: {e}"))
            })?;

            memory.store_vote(height, vote)?;
        }
        drop(cf);

        Ok(Self {
            cache: memory,
            db,
            ns,
        })
    }
}

impl VoteStore for PersistedVoteStore {
    fn earliest_vote_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.cache.earliest_vote_height()
    }

    fn latest_vote_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.cache.latest_vote_height()
    }

    fn store_vote(&mut self, height: BlockHeight, vote: Vote) -> Result<(), Error> {
        let bytes = fvm_ipld_encoding::to_vec(&vote)
            .map_err(|e| Error::PersistentVoteStore(format!("{e}")))?;

        let cf = get_cf(self.db.as_ref(), self.ns.as_str())?;
        self.db
            .put_cf(&cf, height.to_be_bytes(), bytes)
            .map_err(|e| Error::PersistentVoteStore(format!("cannot store vote {e}")))?;
        self.cache.store_vote(height, vote)
    }

    fn has_voted(&self, height: &BlockHeight, validator: &ValidatorKey) -> Result<bool, Error> {
        self.cache.has_voted(height, validator)
    }

    fn get_votes_at_height(&self, height: BlockHeight) -> Result<VoteAgg, Error> {
        self.cache.get_votes_at_height(height)
    }

    fn purge_votes_at_height(&mut self, height: BlockHeight) -> Result<(), Error> {
        let cf = get_cf(self.db.as_ref(), self.ns.as_str())?;
        self.db
            .delete_cf(&cf, height.to_be_bytes())
            .map_err(|e| Error::PersistentVoteStore(format!("cannot remove block height {e}")))?;
        self.cache.purge_votes_at_height(height)
    }
}

pub(crate) fn get_cf<'a>(
    db: &'a OptimisticTransactionDB,
    ns: &'a str,
) -> Result<Arc<BoundColumnFamily<'a>>, Error> {
    db.cf_handle(ns)
        .ok_or_else(|| Error::PersistentVoteStore(format!("namespace {ns} does not exist")))
}

// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::syncer::error::Error;
use crate::syncer::payload::ParentBlockView;
use crate::syncer::store::{InMemoryParentViewStore, ParentViewStore};
use crate::BlockHeight;
use rocksdb::{BoundColumnFamily, IteratorMode, OptimisticTransactionDB};
use std::sync::Arc;

#[derive(Clone)]
pub struct PersistedParentViewStore {
    cache: InMemoryParentViewStore,
    db: Arc<OptimisticTransactionDB>,
    ns: String,
}

impl PersistedParentViewStore {
    pub fn new(db: Arc<OptimisticTransactionDB>, ns: String) -> Result<Self, Error> {
        // All namespaces are pre-created during open.
        if db.cf_handle(&ns).is_none() {
            return Err(Error::PersistentParentViewStore(format!(
                "namespace {ns} does not exist"
            )));
        }

        let memory = InMemoryParentViewStore::default();

        let cf = get_cf(&db, &ns)?;
        let iter = db.iterator_cf(&cf, IteratorMode::Start);
        for item in iter {
            let (_, value) = item.map_err(|e| Error::PersistentParentViewStore(format!("{e}")))?;

            let view = fvm_ipld_encoding::from_slice(value.as_ref()).map_err(|e| {
                Error::PersistentParentViewStore(format!("cannot convert value to vote: {e}"))
            })?;

            memory.store(view)?;
        }
        drop(cf);

        Ok(Self {
            cache: memory,
            db,
            ns,
        })
    }
}

impl ParentViewStore for PersistedParentViewStore {
    fn store(&self, view: ParentBlockView) -> Result<(), Error> {
        let height = view.parent_height;
        let bytes = fvm_ipld_encoding::to_vec(&view)
            .map_err(|e| Error::PersistentParentViewStore(format!("{e}")))?;

        let cf = get_cf(self.db.as_ref(), self.ns.as_str())?;
        self.db
            .put_cf(&cf, height.to_be_bytes(), bytes)
            .map_err(|e| {
                Error::PersistentParentViewStore(format!("cannot store parent view {e}"))
            })?;
        self.cache.store(view)
    }

    fn get(&self, height: BlockHeight) -> Result<Option<ParentBlockView>, Error> {
        self.cache.get(height)
    }

    fn purge(&self, height: BlockHeight) -> Result<(), Error> {
        let cf = get_cf(self.db.as_ref(), self.ns.as_str())?;
        self.db.delete_cf(&cf, height.to_be_bytes()).map_err(|e| {
            Error::PersistentParentViewStore(format!("cannot remove block height {e}"))
        })?;
        self.cache.purge(height)
    }

    fn min_parent_view_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.cache.min_parent_view_height()
    }

    fn max_parent_view_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.cache.max_parent_view_height()
    }
}

pub(crate) fn get_cf<'a>(
    db: &'a OptimisticTransactionDB,
    ns: &'a str,
) -> Result<Arc<BoundColumnFamily<'a>>, Error> {
    db.cf_handle(ns)
        .ok_or_else(|| Error::PersistentParentViewStore(format!("namespace {ns} does not exist")))
}

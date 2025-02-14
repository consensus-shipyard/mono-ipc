// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::syncer::error::Error;
use crate::syncer::payload::ParentBlockView;
use crate::syncer::store::{InMemoryParentViewStore, ParentViewStore};
use crate::BlockHeight;
use rocksdb::{BoundColumnFamily, IteratorMode, OptimisticTransactionDB};
use std::sync::Arc;

/// The persisted store that stores the parent view fetch from RPC nodes.
/// Internally, it's making use of a in memory store for faster get response.
#[derive(Clone)]
pub struct PersistedParentViewStore {
    in_memory_store: InMemoryParentViewStore,
    persisted_store: Arc<OptimisticTransactionDB>,
    /// The namespace for the persisted store
    ns: String,
}

impl PersistedParentViewStore {
    pub fn new(db: Arc<OptimisticTransactionDB>, ns: String) -> Result<Self, Error> {
        // All namespaces are pre-created during open.
        if db.cf_handle(&ns).is_none() {
            return Err(Error::StoreNamespaceDoesNotExist(ns.to_string()));
        }

        let memory = InMemoryParentViewStore::default();

        let cf = get_cf(&db, &ns)?;
        let iter = db.iterator_cf(&cf, IteratorMode::Start);
        for item in iter {
            let (_, value) = item.map_err(|e| Error::PersistentParentViewStore(Box::new(e)))?;

            let view = fvm_ipld_encoding::from_slice(value.as_ref()).map_err(|e| {
                Error::PersistentParentViewStore(Box::new(e))
            })?;

            memory.store(view)?;
        }
        drop(cf);

        Ok(Self {
            in_memory_store: memory,
            persisted_store: db,
            ns,
        })
    }
}

impl ParentViewStore for PersistedParentViewStore {
    fn store(&self, view: ParentBlockView) -> Result<(), Error> {
        let height = view.parent_height;
        let bytes = fvm_ipld_encoding::to_vec(&view)
            .map_err(|e| Error::PersistentParentViewStore(Box::new(e)))?;

        let cf = get_cf(self.persisted_store.as_ref(), self.ns.as_str())?;
        self.persisted_store
            .put_cf(&cf, height.to_be_bytes(), bytes)
            .map_err(|e| {
                Error::PersistentParentViewStore(Box::new(e))
            })?;
        self.in_memory_store.store(view)
    }

    fn get(&self, height: BlockHeight) -> Result<Option<ParentBlockView>, Error> {
        self.in_memory_store.get(height)
    }

    fn purge(&self, height: BlockHeight) -> Result<(), Error> {
        let cf = get_cf(self.persisted_store.as_ref(), self.ns.as_str())?;
        self.persisted_store.delete_cf(&cf, height.to_be_bytes()).map_err(|e| {
            Error::PersistentParentViewStore(Box::new(e))
        })?;
        self.in_memory_store.purge(height)
    }

    fn min_parent_view_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.in_memory_store.min_parent_view_height()
    }

    fn max_parent_view_height(&self) -> Result<Option<BlockHeight>, Error> {
        self.in_memory_store.max_parent_view_height()
    }
}

pub(crate) fn get_cf<'a>(
    db: &'a OptimisticTransactionDB,
    ns: &'a str,
) -> Result<Arc<BoundColumnFamily<'a>>, Error> {
    db.cf_handle(ns)
        .ok_or_else(|| Error::StoreNamespaceDoesNotExist(ns.to_string()))
}

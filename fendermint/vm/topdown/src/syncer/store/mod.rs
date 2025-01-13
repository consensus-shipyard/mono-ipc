// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

mod persisted;

use crate::syncer::error::Error;
use crate::syncer::payload::ParentBlockView;
use crate::{BlockHeight, SequentialKeyCache};
pub use persisted::PersistedParentViewStore;
use std::sync::{Arc, RwLock};

/// Stores the parent view observed of the current node
pub trait ParentViewStore: Clone {
    /// Store a newly observed parent view
    fn store(&self, view: ParentBlockView) -> Result<(), Error>;

    /// Get the parent view at the specified height
    fn get(&self, height: BlockHeight) -> Result<Option<ParentBlockView>, Error>;

    /// Purge the parent view at the target height
    fn purge(&self, height: BlockHeight) -> Result<(), Error>;

    fn min_parent_view_height(&self) -> Result<Option<BlockHeight>, Error>;

    fn max_parent_view_height(&self) -> Result<Option<BlockHeight>, Error>;
}

#[derive(Clone)]
pub struct InMemoryParentViewStore {
    inner: Arc<RwLock<SequentialKeyCache<BlockHeight, ParentBlockView>>>,
}

impl Default for InMemoryParentViewStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryParentViewStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(SequentialKeyCache::sequential())),
        }
    }
}

impl ParentViewStore for InMemoryParentViewStore {
    fn store(&self, view: ParentBlockView) -> Result<(), Error> {
        let mut inner = self.inner.write().unwrap();
        inner
            .append(view.parent_height, view)
            .map_err(|_| Error::NonSequentialParentViewInsert)
    }

    fn get(&self, height: BlockHeight) -> Result<Option<ParentBlockView>, Error> {
        let inner = self.inner.read().unwrap();
        Ok(inner.get_value(height).cloned())
    }

    fn purge(&self, height: BlockHeight) -> Result<(), Error> {
        let mut inner = self.inner.write().unwrap();
        inner.remove_key_below(height + 1);
        Ok(())
    }

    fn min_parent_view_height(&self) -> Result<Option<BlockHeight>, Error> {
        let inner = self.inner.read().unwrap();
        Ok(inner.lower_bound())
    }

    fn max_parent_view_height(&self) -> Result<Option<BlockHeight>, Error> {
        let inner = self.inner.read().unwrap();
        Ok(inner.upper_bound())
    }
}

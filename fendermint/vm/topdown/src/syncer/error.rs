// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::BlockHeight;
use thiserror::Error;

/// The errors for top down checkpointing
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum Error {
    #[error("Incoming items are not order sequentially")]
    NotSequential,
    #[error("The parent view update with block height is not sequential")]
    NonSequentialParentViewInsert,
    #[error("Parent chain reorg detected")]
    ParentChainReorgDetected,
    #[error("Cannot query parent at height {1}: {0}")]
    CannotQueryParent(String, BlockHeight),
    #[error("Parent block view store is empty")]
    BlockStoreEmpty,
    #[error("Committed block height not purged yet")]
    CommittedParentHeightNotPurged,
    #[error("Cannot serialize parent block view payload to bytes")]
    CannotSerializeParentBlockView,
    #[error("Cannot create commitment at null parent block {0}")]
    CannotCommitObservationAtNullBlock(BlockHeight),
    #[error("Missing block view at height {0} for target observation height {0}")]
    MissingBlockView(BlockHeight, BlockHeight),
    #[error("persistent parent view store error: {0}")]
    PersistentParentViewStore(String),
}

// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::syncer::error::Error;
use crate::{BlockHash, BlockHeight, Bytes};
use cid::Cid;
use fvm_ipld_encoding::DAG_CBOR;
use ipc_api::cross::IpcEnvelope;
use ipc_api::staking::StakingChangeRequest;
use multihash::Code;
use multihash::MultihashDigest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParentBlockViewPayload {
    pub parent_hash: BlockHash,
    /// Encodes cross-net messages.
    pub xnet_msgs: Vec<IpcEnvelope>,
    /// Encodes validator membership change commands.
    pub validator_changes: Vec<StakingChangeRequest>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParentBlockView {
    pub parent_height: BlockHeight,
    /// If the payload is None, this means the parent height is a null block
    pub payload: Option<ParentBlockViewPayload>,
}

impl ParentBlockView {
    pub fn null_block(h: BlockHeight) -> Self {
        Self {
            parent_height: h,
            payload: None,
        }
    }

    pub fn nonnull_block(
        h: BlockHeight,
        parent_hash: BlockHash,
        xnet_msgs: Vec<IpcEnvelope>,
        validator_changes: Vec<StakingChangeRequest>,
    ) -> Self {
        Self {
            parent_height: h,
            payload: Some(ParentBlockViewPayload {
                parent_hash,
                xnet_msgs,
                validator_changes,
            }),
        }
    }

    pub fn effects_commitment(&self) -> Result<Bytes, Error> {
        let Some(ref p) = self.payload else {
            return Ok(Cid::default().to_bytes());
        };

        let bytes =
            fvm_ipld_encoding::to_vec(&(&p.xnet_msgs, &p.validator_changes)).map_err(|e| {
                tracing::error!(err = e.to_string(), "cannot serialize parent block view");
                Error::CannotSerializeParentBlockView
            })?;
        let cid = Cid::new_v1(DAG_CBOR, Code::Blake2b256.digest(&bytes));
        Ok(cid.to_bytes())
    }
}

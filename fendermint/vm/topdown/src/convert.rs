// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
//! Handles the type conversion to ethers contract types

use crate::Checkpoint;
use anyhow::anyhow;
use ethers::types::{Bytes, U256};
use ipc_actors_abis::{gateway_getter_facet, top_down_finality_facet};

impl TryFrom<Checkpoint> for top_down_finality_facet::TopdownCheckpoint {
    type Error = anyhow::Error;

    fn try_from(value: Checkpoint) -> Result<Self, Self::Error> {
        if value.target_hash().len() != 32 {
            return Err(anyhow!("invalid block hash length, expecting 32"));
        }

        let mut block_hash = [0u8; 32];
        block_hash.copy_from_slice(&value.target_hash()[0..32]);

        Ok(Self {
            height: U256::from(value.target_height()),
            block_hash,
            effects_commitment: Bytes::from(value.cumulative_effects_comm().clone()),
        })
    }
}

impl From<gateway_getter_facet::TopdownCheckpoint> for Checkpoint {
    fn from(value: gateway_getter_facet::TopdownCheckpoint) -> Self {
        Checkpoint::v1(
            value.height.as_u64(),
            value.block_hash.to_vec(),
            value.effects_commitment.to_vec(),
        )
    }
}

impl From<top_down_finality_facet::TopdownCheckpoint> for Checkpoint {
    fn from(value: top_down_finality_facet::TopdownCheckpoint) -> Self {
        Checkpoint::v1(
            value.height.as_u64(),
            value.block_hash.to_vec(),
            value.effects_commitment.to_vec(),
        )
    }
}

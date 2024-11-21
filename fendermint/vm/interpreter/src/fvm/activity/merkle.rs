// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::Context;
use ipc_api::checkpoint::consensus::ValidatorData;
use ipc_api::evm::payload_to_evm_address;
use ipc_observability::lazy_static;
use merkle_tree_rs::format::Raw;
use merkle_tree_rs::standard::StandardMerkleTree;

pub type Hash = ethers::types::H256;

lazy_static!(
    /// ABI types of the Merkle tree which contains validator addresses and their voting power.
    pub static ref VALIDATOR_SUMMARY_FIELDS: Vec<String> = vec!["address".to_owned(), "uint64".to_owned()];
);

/// The merkle tree based proof verification to interact with solidity contracts
pub(crate) struct MerkleProofGen {
    tree: StandardMerkleTree<Raw>,
}

impl MerkleProofGen {
    pub fn root(&self) -> Hash {
        self.tree.root()
    }
}

impl MerkleProofGen {
    pub fn new(values: &[ValidatorData]) -> anyhow::Result<Self> {
        let values = values
            .iter()
            .map(|t| {
                payload_to_evm_address(t.validator.payload())
                    .map(|addr| vec![format!("{addr:?}"), t.stats.blocks_committed.to_string()])
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let tree = StandardMerkleTree::of(&values, &VALIDATOR_SUMMARY_FIELDS)
            .context("failed to construct Merkle tree")?;
        Ok(MerkleProofGen { tree })
    }
}

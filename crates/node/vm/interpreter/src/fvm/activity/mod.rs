// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

//! Tracks the current blockchain block mining activities and propagates to the parent subnet if
//! needed.

pub mod actor;

use fendermint_crypto::PublicKey;
use ipc_actors_abis::checkpointing_facet::{
    AggregatedStats, CompressedActivityRollup, CompressedSummary, FullActivityRollup, FullSummary,
    ValidatorData,
};
use ipc_api::checkpoint::VALIDATOR_REWARD_FIELDS;
use ipc_api::evm::payload_to_evm_address;
use ipc_api::merkle::MerkleGen;

/// Wrapper for FullActivityRollup with some utility functions
pub struct FullActivity(FullActivityRollup);

/// Tracks the validator activities in the current blockchain
pub trait ValidatorActivityTracker {
    /// Mark the validator has mined the target block.
    fn record_block_committed(&mut self, validator: PublicKey) -> anyhow::Result<()>;

    /// Get the validators activities summary since the checkpoint height
    fn commit_activity(&mut self) -> anyhow::Result<FullActivity>;
}

impl TryFrom<fendermint_actor_activity_tracker::types::FullActivityRollup> for FullActivity {
    type Error = anyhow::Error;

    fn try_from(
        value: fendermint_actor_activity_tracker::types::FullActivityRollup,
    ) -> Result<Self, Self::Error> {
        let stats = AggregatedStats {
            total_active_validators: value.consensus.stats.total_active_validators,
            total_num_blocks_committed: value.consensus.stats.total_num_blocks_committed,
        };
        let data = value
            .consensus
            .data
            .into_iter()
            .map(|(addr, data)| {
                let data = ValidatorData {
                    validator: payload_to_evm_address(addr.payload())?,
                    blocks_committed: data.blocks_committed,
                };
                Ok(data)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let consensus = FullSummary { stats, data };
        let f = FullActivityRollup { consensus };
        Ok(Self::new(f))
    }
}

impl FullActivity {
    pub fn new(mut full: FullActivityRollup) -> Self {
        full.consensus.data.sort_by(|a, b| {
            let cmp = a.validator.cmp(&b.validator);
            if cmp.is_eq() {
                // Address will be unique, do this just in case equal
                a.blocks_committed.cmp(&b.blocks_committed)
            } else {
                cmp
            }
        });
        Self(full)
    }

    pub fn compressed(&self) -> anyhow::Result<CompressedActivityRollup> {
        let gen = MerkleGen::new(
            |v| vec![format!("{:?}", v.validator), v.blocks_committed.to_string()],
            self.0.consensus.data.as_slice(),
            &VALIDATOR_REWARD_FIELDS,
        )?;
        Ok(CompressedActivityRollup {
            consensus: CompressedSummary {
                stats: self.0.consensus.stats.clone(),
                data_root_commitment: gen.root().to_fixed_bytes(),
            },
        })
    }

    pub fn into_inner(self) -> FullActivityRollup {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::fvm::activity::FullActivity;
    use ipc_actors_abis::checkpointing_facet::{
        AggregatedStats, FullActivityRollup, FullSummary, ValidatorData,
    };
    use rand::prelude::SliceRandom;
    use rand::thread_rng;
    use std::str::FromStr;

    #[test]
    fn test_commitment() {
        let mut v = vec![
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0xB29C00299756135ec5d6A140CA54Ec77790a99d6",
                )
                .unwrap(),
                blocks_committed: 1,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x28345a43c2fBae4412f0AbadFa06Bd8BA3f58867",
                )
                .unwrap(),
                blocks_committed: 2,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x1A79385eAd0e873FE0C441C034636D3Edf7014cC",
                )
                .unwrap(),
                blocks_committed: 10,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x76B9d5a35C46B1fFEb37aadf929f1CA63a26A829",
                )
                .unwrap(),
                blocks_committed: 4,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x3c5cc76b07cb02a372e647887bD6780513659527",
                )
                .unwrap(),
                blocks_committed: 3,
            },
        ];

        for _ in 0..10 {
            v.shuffle(&mut thread_rng());
            let full = FullActivityRollup {
                consensus: FullSummary {
                    stats: AggregatedStats {
                        total_active_validators: 1,
                        total_num_blocks_committed: 2,
                    },
                    data: v.clone(),
                },
            };
            let details = FullActivity::new(full);
            assert_eq!(
                hex::encode(details.compressed().unwrap().consensus.data_root_commitment),
                "5519955f33109df3338490473cb14458640efdccd4df05998c4c439738280ab0"
            );
        }
    }
}

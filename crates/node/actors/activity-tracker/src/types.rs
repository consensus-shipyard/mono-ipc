// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::address::Address;
use std::collections::HashMap;

#[derive(Deserialize_tuple, Serialize_tuple, Debug, Clone, PartialEq, Eq, Default)]
pub struct AggregatedStats {
    pub total_active_validators: u64,
    pub total_num_blocks_committed: u64,
}

#[derive(Deserialize_tuple, Serialize_tuple, Debug, Clone, PartialEq, Eq, Default)]
pub struct FullConsensusSummary {
    pub stats: AggregatedStats,
    pub data: HashMap<Address, ValidatorStats>,
}

#[derive(Deserialize_tuple, Serialize_tuple, Debug, Clone, PartialEq, Eq, Default)]
pub struct FullActivityRollup {
    pub consensus: FullConsensusSummary,
}

#[derive(Deserialize_tuple, Serialize_tuple, Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidatorStats {
    pub blocks_committed: u64,
}

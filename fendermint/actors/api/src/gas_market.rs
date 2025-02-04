// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use fil_actors_runtime::runtime::Runtime;
use fil_actors_runtime::ActorError;
use fvm_ipld_encoding::tuple::*;
use fvm_shared::econ::TokenAmount;
use num_derive::FromPrimitive;

pub type Gas = u64;

/// A reading of the current gas market state for use by consensus.
#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone)]
pub struct Reading {
    /// The current gas limit for the block.
    pub block_gas_limit: Gas,
    /// The current base fee for the block.
    pub base_fee: TokenAmount,
    /// The minimum allowable base fee.
    pub min_base_fee: TokenAmount,
}

/// The current utilization for the client to report to the gas market.
#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone)]
pub struct Utilization {
    /// The gas used by the current block, at the end of the block. To be invoked as an implicit
    /// message, so that gas metering for this message is disabled.
    pub block_gas_used: Gas,
}

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    CurrentReading = frc42_dispatch::method_hash!("CurrentReading"),
    UpdateUtilization = frc42_dispatch::method_hash!("UpdateUtilization"),
}

/// The trait to be implemented by a gas market actor, provided here for convenience,
/// using the standard Runtime libraries. Ready to be implemented as-is by an actor.
pub trait GasMarket {
    /// Returns the current gas market reading.
    fn current_reading(rt: &impl Runtime) -> Result<Reading, ActorError>;

    /// Updates the current utilization in the gas market, returning the reading after the update.
    fn update_utilization(
        rt: &impl Runtime,
        utilization: Utilization,
    ) -> Result<Reading, ActorError>;
}

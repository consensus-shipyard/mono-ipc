// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/consensus-shipyard/libp2p-bitswap with assumed MIT license
// as per Cargo.toml: https://github.com/consensus-shipyard/libp2p-bitswap/blob/7dd9cececda3e4a8f6e14c200a4b457159d8db33/Cargo.toml#L7
//
// License headers added post-fork.
//! Bitswap protocol implementation
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(clippy::derive_partial_eq_without_eq)]

mod behaviour;
#[cfg(feature = "compat")]
mod compat;
mod protocol;
mod query;
mod stats;

pub use crate::behaviour::{Bitswap, BitswapConfig, BitswapEvent, BitswapStore, Channel};
pub use crate::protocol::{BitswapRequest, BitswapResponse};
pub use crate::query::QueryId;

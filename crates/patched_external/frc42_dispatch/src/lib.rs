// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/filecoin-project/actors-utils with assumed MIT license
// as per Cargo.toml: https://github.com/filecoin-project/actors-utils/blob/7628cd8d39dafcc6035f28e350cdb0cccbea5ab4/frc42_dispatch/Cargo.toml#L5
//
// License headers added post-fork.
pub use frc42_hasher as hasher;
pub use frc42_hasher::hash;
pub use frc42_macros::method_hash;

pub mod match_method;
pub mod message;

#[cfg(test)]
mod tests {}

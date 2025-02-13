// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/filecoin-project/actors-utils with assumed MIT license
// as per Cargo.toml: https://github.com/filecoin-project/actors-utils/blob/7628cd8d39dafcc6035f28e350cdb0cccbea5ab4/frc42_dispatch/Cargo.toml#L5
//
// License headers added post-fork.
use blake2b_simd::blake2b;
use frc42_hasher::hash::Hasher;

pub struct Blake2bHasher {}
impl Hasher for Blake2bHasher {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        blake2b(bytes).as_bytes().to_vec()
    }
}

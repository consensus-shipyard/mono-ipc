// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/filecoin-project/actors-utils with assumed MIT license
// as per Cargo.toml: https://github.com/filecoin-project/actors-utils/blob/7628cd8d39dafcc6035f28e350cdb0cccbea5ab4/frc42_dispatch/Cargo.toml#L5
//
// License headers added post-fork.
use frc42_macros::method_hash;

fn main() {
    assert_eq!(method_hash!("Method"), 0xa20642fc);
    assert_eq!(method_hash!("_Method"), 0xeb9575aa);

    // method names from the example token actor
    // numbers are hashed by the python script included in the main dispatch crate
    assert_eq!(method_hash!("Name"), 0x02ea015c);
    assert_eq!(method_hash!("Symbol"), 0x7adab63e);
    assert_eq!(method_hash!("TotalSupply"), 0x06da7a35);
    assert_eq!(method_hash!("BalanceOf"), 0x8710e1ac);
    assert_eq!(method_hash!("Allowance"), 0xfaa45236);
    assert_eq!(method_hash!("IncreaseAllowance"), 0x69ecb918);
    assert_eq!(method_hash!("DecreaseAllowance"), 0x5b286f21);
    assert_eq!(method_hash!("RevokeAllowance"), 0xa4d840b1);
    assert_eq!(method_hash!("Burn"), 0x5584159a);
    assert_eq!(method_hash!("TransferFrom"), 0xd7d4deed);
    assert_eq!(method_hash!("Transfer"), 0x04cbf732);
    assert_eq!(method_hash!("Mint"), 0x06f84ab2);
}

// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/filecoin-project/actors-utils with assumed MIT license
// as per Cargo.toml: https://github.com/filecoin-project/actors-utils/blob/7628cd8d39dafcc6035f28e350cdb0cccbea5ab4/frc42_dispatch/Cargo.toml#L5
//
// License headers added post-fork.
use frc42_macros::method_hash;

fn main() {
    let str_hash = method_hash!("Method");
    println!("String hash: {str_hash:x}");

    // this one breaks naming rules and will fail to compile
    //println!("error hash: {}", method_hash!("some_function"));
}

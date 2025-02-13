// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
use fvm_shared::address::Address;
use ipc_api::subnet_id::SubnetID;
use quickcheck::Arbitrary;

#[derive(Clone, Debug)]
pub struct ArbSubnetID(pub SubnetID);

impl Arbitrary for ArbSubnetID {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let child_count = usize::arbitrary(g) % 4;

        let children = (0..child_count)
            .map(|_| {
                if bool::arbitrary(g) {
                    Address::new_id(u64::arbitrary(g))
                } else {
                    // Only expectign EAM managed delegated addresses.
                    let subaddr: [u8; 20] = std::array::from_fn(|_| Arbitrary::arbitrary(g));
                    Address::new_delegated(10, &subaddr).unwrap()
                }
            })
            .collect::<Vec<_>>();

        Self(SubnetID::new(u64::arbitrary(g), children))
    }
}

// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use hex;
use std::fmt;

/// Hex encodable block hash.
pub struct HexEncodableBlockHash(pub Vec<u8>);

impl fmt::Debug for HexEncodableBlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

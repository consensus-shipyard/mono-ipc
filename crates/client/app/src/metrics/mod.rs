// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

mod prometheus;

pub use prometheus::eth::register_metrics as register_eth_metrics;

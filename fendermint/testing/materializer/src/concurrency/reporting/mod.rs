// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

pub mod dataset;
pub mod summary;
pub mod tps;

use crate::bencher::Bencher;
use ethers::prelude::H256;

#[cfg(test)]
const FLOAT_TOLERANCE: f64 = 1e-6;

#[derive(Debug)]
pub struct TestResult {
    pub test_id: usize,
    pub step_id: usize,
    pub tx_hash: Option<H256>,
    pub bencher: Option<Bencher>,
    pub err: Option<anyhow::Error>,
}

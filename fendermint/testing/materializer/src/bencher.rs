// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct Bencher {
    pub start_time: Option<Instant>,
    pub latencies: HashMap<String, Duration>,
    pub block_inclusion: Option<u64>,
}

impl Bencher {
    pub fn new() -> Self {
        Self {
            start_time: None,
            latencies: HashMap::new(),
            block_inclusion: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn mempool(&mut self) {
        self.set_latency("mempool".to_string());
    }

    pub fn block_inclusion(&mut self, block_number: u64) {
        self.set_latency("block_inclusion".to_string());
        self.block_inclusion = Some(block_number);
    }

    fn set_latency(&mut self, label: String) {
        let duration = self.start_time.unwrap().elapsed();
        self.latencies.insert(label, duration);
    }
}

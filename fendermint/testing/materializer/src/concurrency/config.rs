// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Execution {
    pub steps: Vec<ExecutionStep>,
}

impl Execution {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(mut self, max_concurrency: usize, secs: u64) -> Self {
        self.steps.push(ExecutionStep {
            max_concurrency,
            duration: Duration::from_secs(secs),
        });
        self
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub max_concurrency: usize,
    pub duration: Duration,
}

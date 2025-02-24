// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

pub mod collect;
pub mod config;
pub mod nonce_manager;
pub mod reporting;
pub mod cancellation_flag;

use crate::bencher::Bencher;
use crate::concurrency::reporting::TestResult;
use ethers::types::H256;
use futures::FutureExt;
use std::future::Future;
use std::sync::Arc;
use std::time::Instant;
use futures::stream::FuturesUnordered;
use tokio::sync::{Semaphore};
use futures::{StreamExt};

#[derive(Debug)]
pub struct TestInput {
    pub test_id: usize,
    pub bencher: Bencher,
}

#[derive(Debug)]
pub struct TestOutput {
    pub bencher: Bencher,
    pub tx_hash: H256,
}

pub async fn execute<F, Fut>(cfg: config::Execution, test_factory: F) -> Vec<Vec<TestResult>>
where
    F: Fn(TestInput) -> Fut,
    Fut: Future<Output = anyhow::Result<TestOutput>> + Send + 'static,
{
    let mut test_id = 0;
    let mut results = Vec::new();
    for (step_id, step) in cfg.steps.iter().enumerate() {
        let semaphore = Arc::new(Semaphore::new(step.max_concurrency));
        let mut tasks = FuturesUnordered::new();
        let execution_start = Instant::now();
        loop {
            if execution_start.elapsed() > step.duration {
                break;
            }
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let bencher = Bencher::new();
            let test_input = TestInput { test_id, bencher };
            let task = test_factory(test_input).boxed();
            tasks.push(tokio::spawn(async move {
                let test_output = task.await;
                drop(permit);
                let (bencher, tx_hash, err) = match test_output {
                    Ok(test_output) => (Some(test_output.bencher), Some(test_output.tx_hash), None),
                    Err(err) => (None, None, Some(err)),
                };
                TestResult {
                    test_id,
                    step_id,
                    bencher,
                    tx_hash,
                    err,
                }
            }));
            test_id += 1;
        }

        // Collect results as tasks complete (unordered).
        let mut step_results = Vec::new();
        while let Some(Ok(result)) = tasks.next().await {
            step_results.push(result);
        }
        results.push(step_results);
    }
    results
}

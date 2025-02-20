// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

pub mod collect;
pub mod config;
pub mod nonce_manager;
pub mod reporting;
pub mod signal;

use crate::bencher::Bencher;
use crate::concurrency::reporting::TestResult;
use ethers::types::H256;
use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

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

pub async fn execute<F>(cfg: config::Execution, test_factory: F) -> Vec<Vec<TestResult>>
where
    F: Fn(TestInput) -> Pin<Box<dyn Future<Output = anyhow::Result<TestOutput>> + Send>>,
{
    let mut test_id = 0;
    let mut results = Vec::new();
    for (step_id, step) in cfg.steps.iter().enumerate() {
        let semaphore = Arc::new(Semaphore::new(step.max_concurrency));
        let mut handles = Vec::new();
        let step_results = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let execution_start = Instant::now();
        loop {
            if execution_start.elapsed() > step.duration {
                break;
            }
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let bencher = Bencher::new();
            let test_input = TestInput { test_id, bencher };
            let task = test_factory(test_input).boxed();
            let step_results = step_results.clone();
            let handle = tokio::spawn(async move {
                let test_output = task.await;
                let (bencher, tx_hash, err) = match test_output {
                    Ok(test_output) => (Some(test_output.bencher), Some(test_output.tx_hash), None),
                    Err(err) => (None, None, Some(err)),
                };
                step_results.lock().await.push(TestResult {
                    test_id,
                    step_id,
                    bencher,
                    tx_hash,
                    err,
                });
                drop(permit);
            });
            handles.push(handle);
            test_id += 1;
        }

        // Exhaust unfinished handles.
        for handle in handles {
            handle.await.unwrap();
        }

        let step_results = Arc::try_unwrap(step_results).unwrap().into_inner();
        results.push(step_results)
    }
    results
}

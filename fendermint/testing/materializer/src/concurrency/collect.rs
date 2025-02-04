// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::concurrency::signal::Signal;
use ethers::prelude::H256;
use ethers::providers::Http;
use ethers::providers::{Middleware, Provider};
use ethers::types::Block;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub async fn collect_blocks<F>(
    cancel: Arc<Signal>,
    provider: Provider<Http>,
    assert: F,
) -> anyhow::Result<HashMap<u64, Block<H256>>>
where
    F: Fn(&Block<H256>),
{
    let mut blocks = HashMap::new();
    loop {
        if cancel.received() {
            break;
        }

        // TODO: improve: use less calls, make sure blocks cannot be missed
        let block_number = provider.get_block_number().await?;
        let block = provider.get_block(block_number).await?.unwrap();
        assert(&block);
        blocks.insert(block_number.as_u64(), block);

        sleep(Duration::from_millis(100)).await;
    }
    Ok(blocks)
}

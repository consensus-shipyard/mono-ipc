// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use crate::make_testnet;
use anyhow::{anyhow, bail};
use ethers::{providers::Middleware, types::U64};
use fendermint_materializer::HasEthApi;
use std::time::Duration;

const MANIFEST: &str = "root-only.yaml";

#[serial_test::serial]
#[tokio::test]
async fn test_full_node_sync() -> Result<(), anyhow::Error> {
    let (testnet, cleanup) = make_testnet(MANIFEST, |_| {}).await?;

    let res = {
        // Allow a little bit of time for node-2 to catch up with node-1.
        tokio::time::sleep(Duration::from_secs(5)).await;
        // Check that node2 is following node1.
        let node2 = testnet.root().node("node-2");
        let dnode2 = testnet.node(&node2)?;

        let provider = dnode2
            .ethapi_http_provider()?
            .ok_or_else(|| anyhow!("node-2 has ethapi enabled"))?;

        let bn = provider.get_block_number().await?;

        if bn <= U64::one() {
            bail!("expected a block beyond genesis");
        }

        Ok(())
    };

    cleanup(res.is_err(), testnet).await;
    res
}

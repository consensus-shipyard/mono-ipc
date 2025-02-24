// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::make_testnet;
use anyhow::{anyhow, bail, Context};
use ethers::abi::Abi;
use ethers::contract::{abigen, ContractFactory};
use ethers::core::types::Bytes;
use ethers::prelude::{Block, H256};
use ethers::types::{Address, U256};
use ethers::{
    providers::{Middleware, PendingTransaction},
    types::{Eip1559TransactionRequest, H160},
};
use fendermint_materializer::concurrency::collect::collect_blocks;
use fendermint_materializer::concurrency::nonce_manager::NonceManager;
use fendermint_materializer::concurrency::reporting::summary::ExecutionSummary;
use fendermint_materializer::concurrency::cancellation_flag::CancellationFlag;
use fendermint_materializer::concurrency::TestOutput;
use fendermint_materializer::{
    concurrency::{self, config::Execution},
    HasEthApi,
};
use futures::FutureExt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::docker_tests::make_middleware;

const MANIFEST: &str = "benches.yaml";

#[serial_test::serial]
#[tokio::test]
async fn test_native_coin_transfer() -> Result<(), anyhow::Error> {
    let (testnet, cleanup) = make_testnet(MANIFEST, |_| {}).await?;

    let block_gas_limit = U256::from(10_000_000_000u64);
    let max_tx_gas_limit = U256::from(3_000_000u64);

    let pangea = testnet.node(&testnet.root().node("pangea"))?;
    let provider = pangea
        .ethapi_http_provider()?
        .expect("ethapi should be enabled");
    let chain_id = provider
        .get_chainid()
        .await
        .context("failed to get chain ID")?;

    let cancel = Arc::new(CancellationFlag::new());

    // Set up background blocks collector.
    let blocks_collector = {
        let cancel = cancel.clone();
        let provider = provider.clone();
        let assert = move |block: &Block<H256>| {
            // Make sure block gas limit isn't the bottleneck.
            let unused_gas_limit = block_gas_limit - block.gas_limit;
            assert!(unused_gas_limit >= max_tx_gas_limit);
        };
        tokio::spawn(collect_blocks(cancel, provider, assert))
    };

    // Drive concurrency.
    let cfg = Execution::new()
        .add_step(1, 5)
        .add_step(10, 5)
        .add_step(100, 5)
        .add_step(150, 5);
    let testnet = Arc::new(testnet);
    let testnet_clone = testnet.clone();
    let nonce_manager = Arc::new(NonceManager::new());

    let results = concurrency::execute(cfg.clone(), move |mut input| {
        let testnet = testnet_clone.clone();
        let nonce_manager = nonce_manager.clone();
        let provider = provider.clone();

        let test = async move {
            let sender = testnet.account_mod_nth(input.test_id);
            let recipient = testnet.account_mod_nth(input.test_id + 1);
            println!("running (test_id={})", input.test_id);

            let middleware = make_middleware(provider, sender, Some(chain_id))
                .await
                .context("make_middleware")?;

            let sender: H160 = sender.eth_addr().into();
            let nonce = nonce_manager.get_and_increment(sender).await;

            // Create the simplest transaction possible: send tokens between accounts.
            let to: H160 = recipient.eth_addr().into();
            let mut tx = Eip1559TransactionRequest::new()
                .to(to)
                .value(1)
                .nonce(nonce);

            let gas_estimation = middleware
                .estimate_gas(&tx.clone().into(), None)
                .await
                .unwrap();
            tx = tx.gas(gas_estimation);
            assert!(gas_estimation <= max_tx_gas_limit);

            input.bencher.start();

            let pending: PendingTransaction<_> = middleware
                .send_transaction(tx, None)
                .await
                .context("failed to send txn")?;
            let tx_hash = pending.tx_hash();
            println!("sent tx {:?} (test_id={})", tx_hash, input.test_id);

            // We expect that the transaction is pending, however it should not return an error.
            match middleware.get_transaction(tx_hash).await {
                Ok(Some(_)) => {}
                Ok(None) => bail!("pending transaction not found by eth hash"),
                Err(e) => {
                    bail!("failed to get pending transaction: {e}")
                }
            }
            input.bencher.mempool();

            let receipt = pending
                .interval(Duration::from_millis(50))
                .confirmations(1)
                .await?
                .ok_or(anyhow!("fd"))?;
            println!(
                "tx included in block {:?} (test_id={})",
                receipt.block_number, input.test_id
            );
            let block_number = receipt.block_number.unwrap().as_u64();
            input.bencher.block_inclusion(block_number);

            Ok(TestOutput {
                bencher: input.bencher,
                tx_hash,
            })
        };
        test.boxed()
    })
    .await;

    cancel.cancel();
    let blocks = blocks_collector.await??;
    let summary = ExecutionSummary::new(cfg.clone(), blocks, results);
    println!("summary:\n{}", summary);

    let res = summary.to_result();
    let Ok(testnet) = Arc::try_unwrap(testnet) else {
        bail!("Arc::try_unwrap(testnet)");
    };
    cleanup(res.is_err(), testnet).await;
    res
}

abigen!(SimpleCoin, "../../testing/contracts/SimpleCoin.abi");
const SIMPLECOIN_HEX: &str = include_str!("../../../../testing/contracts/SimpleCoin.bin");

#[serial_test::serial]
#[tokio::test]
async fn test_contract_deployment() -> Result<(), anyhow::Error> {
    let (testnet, cleanup) = make_testnet(MANIFEST, |_| {}).await?;

    let block_gas_limit = U256::from(10_000_000_000u64);
    let max_tx_gas_limit = U256::from(50_000_000u64);

    let pangea = testnet.node(&testnet.root().node("pangea"))?;
    let provider = pangea
        .ethapi_http_provider()?
        .expect("ethapi should be enabled");
    let chain_id = provider
        .get_chainid()
        .await
        .context("failed to get chain ID")?;

    let cancel = Arc::new(CancellationFlag::new());

    // Set up background blocks collector.
    let blocks_collector = {
        let cancel = cancel.clone();
        let provider = provider.clone();
        let assert = move |block: &Block<H256>| {
            // Make sure block gas limit isn't the bottleneck.
            let unused_gas_limit = block_gas_limit - block.gas_limit;
            assert!(unused_gas_limit >= max_tx_gas_limit);
        };
        tokio::spawn(collect_blocks(cancel, provider, assert))
    };

    // Drive concurrency.
    let cfg = Execution::new()
        .add_step(1, 5)
        .add_step(10, 5)
        .add_step(100, 5)
        .add_step(150, 5);
    let testnet = Arc::new(testnet);
    let testnet_clone = testnet.clone();
    let nonce_manager = Arc::new(NonceManager::new());

    let results = concurrency::execute(cfg.clone(), move |mut input| {
        let testnet = testnet_clone.clone();
        let nonce_manager = nonce_manager.clone();
        let provider = provider.clone();

        let test = async move {
            let sender = testnet.account_mod_nth(input.test_id);
            println!("running (test_id={})", input.test_id);

            let middleware = make_middleware(provider, sender, Some(chain_id))
                .await
                .context("make_middleware")?;
            let middleware = Arc::new(middleware);

            let sender: H160 = sender.eth_addr().into();
            let nonce = nonce_manager.get_and_increment(sender).await;

            let bytecode =
                Bytes::from(hex::decode(SIMPLECOIN_HEX).context("failed to decode contract hex")?);
            let abi: Abi = SIMPLECOIN_ABI.clone();
            let factory = ContractFactory::new(abi, bytecode.clone(), middleware.clone());
            let mut deploy_tx = factory.deploy(())?.tx;
            deploy_tx.set_nonce(nonce);

            let gas_estimation = middleware.estimate_gas(&deploy_tx, None).await.unwrap();
            deploy_tx.set_gas(gas_estimation);
            assert!(gas_estimation <= max_tx_gas_limit);

            input.bencher.start();

            let pending: PendingTransaction<_> = middleware
                .send_transaction(deploy_tx, None)
                .await
                .context("failed to send txn")?;
            let tx_hash = pending.tx_hash();
            println!("sent tx {:?} (test_id={})", tx_hash, input.test_id);

            // We expect that the transaction is pending, however it should not return an error.
            match middleware.get_transaction(tx_hash).await {
                Ok(Some(_)) => {}
                Ok(None) => bail!("pending transaction not found by eth hash"),
                Err(e) => {
                    bail!("failed to get pending transaction: {e}")
                }
            }
            input.bencher.mempool();

            let receipt = pending
                .interval(Duration::from_millis(50))
                .confirmations(1)
                .await?
                .ok_or(anyhow::anyhow!("contract not deployed"))?;
            let contract_address = receipt
                .contract_address
                .ok_or(anyhow::anyhow!("contract not deployed"))?;

            println!(
                "deploy tx included in block {:?}, contract address: {:?} (test_id={})",
                receipt.block_number, contract_address, input.test_id
            );
            let block_number = receipt.block_number.unwrap().as_u64();
            input.bencher.block_inclusion(block_number);

            Ok(TestOutput {
                bencher: input.bencher,
                tx_hash,
            })
        };
        test.boxed()
    })
    .await;

    cancel.cancel();
    let blocks = blocks_collector.await??;
    let summary = ExecutionSummary::new(cfg.clone(), blocks, results);
    println!("summary:\n{}", summary);

    let res = summary.to_result();
    let Ok(testnet) = Arc::try_unwrap(testnet) else {
        bail!("Arc::try_unwrap(testnet)");
    };
    cleanup(res.is_err(), testnet).await;
    res
}

#[serial_test::serial]
#[tokio::test]
async fn test_contract_call() -> Result<(), anyhow::Error> {
    let (testnet, cleanup) = make_testnet(MANIFEST, |_| {}).await?;

    let pangea = testnet.node(&testnet.root().node("pangea"))?;
    let provider = pangea
        .ethapi_http_provider()?
        .expect("ethapi should be enabled");
    let chain_id = provider
        .get_chainid()
        .await
        .context("failed to get chain ID")?;
    let nonce_manager = Arc::new(NonceManager::new());

    // Pre-test: deploy contract for each account.
    let contract_addresses = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<tokio::task::JoinHandle<Result<(), anyhow::Error>>> = Vec::new();
    for (_, account) in testnet.accounts().clone() {
        let provider = provider.clone();
        let nonce_manager = nonce_manager.clone();
        let contract_addresses = contract_addresses.clone();
        handles.push(tokio::spawn(async move {
            let middleware = make_middleware(provider, &account, Some(chain_id))
                .await
                .context("make_middleware")?;
            let middleware = Arc::new(middleware);
            let sender: H160 = account.eth_addr().into();
            let nonce = nonce_manager.get_and_increment(sender).await;

            let bytecode = Bytes(
                hex::decode(SIMPLECOIN_HEX)
                    .context("failed to decode contract hex")?
                    .into(),
            );
            let abi: Abi = SIMPLECOIN_ABI.clone();
            let factory = ContractFactory::new(abi, bytecode.clone(), middleware.clone());
            let mut deploy_tx = factory.deploy(())?.tx;
            deploy_tx.set_nonce(nonce);

            let gas_estimation = middleware.estimate_gas(&deploy_tx, None).await.unwrap();
            deploy_tx.set_gas(gas_estimation);

            let pending: PendingTransaction<_> = middleware
                .send_transaction(deploy_tx, None)
                .await
                .context("failed to send txn")?;
            let tx_hash = pending.tx_hash();
            println!("deploy tx {:?}", tx_hash);

            let receipt = pending
                .interval(Duration::from_millis(50))
                .confirmations(1)
                .await?
                .ok_or(anyhow::anyhow!("contract not deployed"))?;
            let contract_address = receipt
                .contract_address
                .ok_or(anyhow::anyhow!("contract not deployed"))?;

            println!(
                "deploy tx included in block {:?}, contract address: {:?}",
                receipt.block_number, contract_address
            );

            contract_addresses
                .lock()
                .unwrap()
                .insert(sender, contract_address);
            Ok(())
        }));
    }

    for handle in handles {
        handle.await??;
    }

    println!("pre-test contract deployments done");

    let contract_addresses: HashMap<Address, Address> = Arc::try_unwrap(contract_addresses)
        .unwrap()
        .into_inner()
        .unwrap();
    let block_gas_limit = U256::from(10_000_000_000u64);
    let max_tx_gas_limit = U256::from(3_000_000u64);
    let cancel = Arc::new(CancellationFlag::new());

    // Set up background blocks collector.
    let blocks_collector = {
        let cancel = cancel.clone();
        let provider = provider.clone();
        let assert = move |block: &Block<H256>| {
            // Make sure block gas limit isn't the bottleneck.
            let unused_gas_limit = block_gas_limit - block.gas_limit;
            assert!(unused_gas_limit >= max_tx_gas_limit);
        };
        tokio::spawn(collect_blocks(cancel, provider, assert))
    };

    // Drive concurrency.
    let cfg = Execution::new()
        .add_step(1, 5)
        .add_step(10, 5)
        .add_step(100, 5)
        .add_step(150, 5);
    let testnet = Arc::new(testnet);
    let testnet_clone = testnet.clone();
    let contract_addresses = Arc::new(contract_addresses);

    let results = concurrency::execute(cfg.clone(), move |mut input| {
        let testnet = testnet_clone.clone();
        let contract_addresses = contract_addresses.clone();
        let nonce_manager = nonce_manager.clone();
        let provider = provider.clone();

        let test = async move {
            let sender = testnet.account_mod_nth(input.test_id);
            let recipient = testnet.account_mod_nth(input.test_id + 1);

            println!("running (test_id={})", input.test_id);

            let middleware = make_middleware(provider, sender, Some(chain_id))
                .await
                .context("make_middleware")?;
            let middleware = Arc::new(middleware);

            let sender: H160 = sender.eth_addr().into();
            let nonce = nonce_manager.get_and_increment(sender).await;

            let contract_address = contract_addresses.get(&sender).unwrap();
            let contract = SimpleCoin::new(*contract_address, middleware.clone());

            let to: H160 = recipient.eth_addr().into();
            let amount = U256::from(1);
            let mut send_tx = contract.send_coin_or_revert(to, amount).tx;
            send_tx.set_nonce(nonce);

            input.bencher.start();

            let pending: PendingTransaction<_> = middleware
                .send_transaction(send_tx, None)
                .await
                .context("failed to send tx")?;

            let tx_hash = pending.tx_hash();
            println!("sent tx {:?} (test_id={})", tx_hash, input.test_id);

            // We expect that the transaction is pending, however it should not return an error.
            match middleware.get_transaction(tx_hash).await {
                Ok(Some(_)) => {}
                Ok(None) => bail!("pending transaction not found by eth hash"),
                Err(e) => {
                    bail!("failed to get pending transaction: {e}")
                }
            }
            input.bencher.mempool();

            let receipt = pending
                .interval(Duration::from_millis(50))
                .confirmations(1)
                .await?
                .ok_or(anyhow::anyhow!("failed to wait pending tx"))?;
            println!(
                "send tx included in block {:?} (test_id={})",
                receipt.block_number, input.test_id
            );
            let block_number = receipt.block_number.unwrap().as_u64();
            input.bencher.block_inclusion(block_number);

            Ok(TestOutput {
                bencher: input.bencher,
                tx_hash,
            })
        };
        test.boxed()
    })
    .await;

    cancel.cancel();
    let blocks = blocks_collector.await??;
    let summary = ExecutionSummary::new(cfg.clone(), blocks, results);
    println!("summary:\n{}", summary);

    let res = summary.to_result();
    let Ok(testnet) = Arc::try_unwrap(testnet) else {
        bail!("Arc::try_unwrap(testnet)");
    };
    cleanup(res.is_err(), testnet).await;
    res
}

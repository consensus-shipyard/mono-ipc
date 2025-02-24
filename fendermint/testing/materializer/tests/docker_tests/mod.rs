// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
//! These test modules are all imported by the top level `docker.rs` module,
//! so that they can be annotated with the `#[serial]` macro and run one by one,
//! sharing their materializer state.

use anyhow::{Context};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::prelude::{JsonRpcClient, Provider, Signer, Wallet};
use fendermint_materializer::materials::DefaultAccount;
use ethers::types::{U256};

// Tests using the manifest bearing their name.
pub mod benches;
pub mod layer2;
pub mod root_only;
pub mod standalone;

pub type TestMiddleware<C> = SignerMiddleware<Provider<C>, Wallet<SigningKey>>;

/// Create a middleware that will assign nonces and sign the message.
async fn make_middleware<C>(
    provider: Provider<C>,
    sender: &DefaultAccount,
    chain_id: Option<U256>,
) -> anyhow::Result<TestMiddleware<C>>
where
    C: JsonRpcClient,
{
    let chain_id = match chain_id {
        Some(id) => id,
        None => provider
            .get_chainid()
            .await
            .context("failed to get chain ID")?,
    };

    let wallet: Wallet<SigningKey> = Wallet::from_bytes(sender.secret_key().serialize().as_ref())?
        .with_chain_id(chain_id.as_u64());

    Ok(SignerMiddleware::new(provider, wallet))
}
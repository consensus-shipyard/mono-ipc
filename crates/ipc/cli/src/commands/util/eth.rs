// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//! Eth address util

use async_trait::async_trait;
use clap::Args;
use fvm_shared::address::Address;
use ipc_api::evm::payload_to_evm_address;
use std::fmt::Debug;
use std::str::FromStr;

use crate::{CommandLineHandler, GlobalArguments};

pub(crate) struct F4ToEthAddr;

#[async_trait]
impl CommandLineHandler for F4ToEthAddr {
    type Arguments = F4ToEthAddrArgs;

    async fn handle(_global: &GlobalArguments, arguments: &Self::Arguments) -> anyhow::Result<()> {
        let addr = Address::from_str(&arguments.addr)?;
        let eth_addr = payload_to_evm_address(addr.payload())?;
        log::info!("eth address: {:?}", eth_addr);
        Ok(())
    }
}

#[derive(Debug, Args)]
#[command(about = "Get Ethereum address for an F4")]
pub(crate) struct F4ToEthAddrArgs {
    #[arg(long, help = "F4 address to get the underlying Ethereum addr from")]
    pub addr: String,
}

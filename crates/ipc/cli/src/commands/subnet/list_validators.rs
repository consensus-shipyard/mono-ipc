// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//! List subnets cli command

use crate::{get_ipc_provider, CommandLineHandler, GlobalArguments};
use async_trait::async_trait;
use clap::Args;
use ipc_api::subnet_id::SubnetID;
use std::fmt::Debug;
use std::str::FromStr;

/// The command to create a new subnet actor.
pub(crate) struct ListValidators;

#[async_trait]
impl CommandLineHandler for ListValidators {
    type Arguments = ListValidatorsArgs;

    async fn handle(global: &GlobalArguments, arguments: &Self::Arguments) -> anyhow::Result<()> {
        log::debug!("list validators with args: {:?}", arguments);

        let provider = get_ipc_provider(global)?;
        let subnet = SubnetID::from_str(&arguments.subnet)?;

        let validators = provider.list_validators(&subnet).await?;

        for (addr, info) in validators {
            println!("{}: {}", addr, info);
        }
        Ok(())
    }
}

#[derive(Debug, Args)]
#[command(
    name = "list validators",
    about = "List the info of all the validators in the subnet, as viewed by the parent"
)]
pub(crate) struct ListValidatorsArgs {
    #[arg(long, help = "The target subnet to perform query")]
    pub subnet: String,
}

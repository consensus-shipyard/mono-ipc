// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT

use crate::commands::get_ipc_provider;
use crate::{CommandLineHandler, GlobalArguments};
use async_trait::async_trait;
use clap::Args;
use fvm_shared::{address::Address, clock::ChainEpoch};
use ipc_api::subnet_id::SubnetID;
use std::str::FromStr;

#[derive(Debug, Args)]
#[command(about = "validator list activities in a subnet")]
pub(crate) struct ListActivitiesArgs {
    #[arg(long, help = "The JSON RPC server url for ipc agent")]
    pub validator: String,
    #[arg(long, help = "The checkpoint height to claim from")]
    pub from: ChainEpoch,
    #[arg(long, help = "The checkpoint height to claim to")]
    pub to: ChainEpoch,
    #[arg(long, help = "The subnet to list activities from")]
    pub subnet: String,
}

pub(crate) struct ListActivities;

#[async_trait]
impl CommandLineHandler for ListActivities {
    type Arguments = ListActivitiesArgs;

    async fn handle(global: &GlobalArguments, arguments: &Self::Arguments) -> anyhow::Result<()> {
        log::debug!("list validator activities with args: {:?}", arguments);

        let provider = get_ipc_provider(global)?;
        let subnet = SubnetID::from_str(&arguments.subnet)?;
        let validator = Address::from_str(&arguments.validator)?;

        let r = provider
            .list_validator_activities(&subnet, &validator, arguments.from, arguments.to)
            .await?;

        println!("found total {} entries", r.len());
        for (checkpoint_height, v) in r {
            println!("  checkpoint height: {}", checkpoint_height);
            println!("  addr: {}", v.validator);
            println!("  locks_committed: {}", v.blocks_committed);
        }

        Ok(())
    }
}

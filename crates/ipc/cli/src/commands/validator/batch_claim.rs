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
#[command(about = "validator batch claim rewards for a target subnet")]
pub(crate) struct BatchClaimArgs {
    #[arg(long, help = "The JSON RPC server url for ipc agent")]
    pub validator: String,
    #[arg(long, help = "The checkpoint height to claim from")]
    pub from: ChainEpoch,
    #[arg(long, help = "The checkpoint height to claim to")]
    pub to: ChainEpoch,
    #[arg(long, help = "The source subnet that generated the reward")]
    pub reward_source_subnet: String,
    #[arg(long, help = "The subnet to claim reward from")]
    pub reward_claim_subnet: String,
}

pub(crate) struct BatchClaim;

#[async_trait]
impl CommandLineHandler for BatchClaim {
    type Arguments = BatchClaimArgs;

    async fn handle(global: &GlobalArguments, arguments: &Self::Arguments) -> anyhow::Result<()> {
        log::debug!("batch claim operation with args: {:?}", arguments);

        let provider = get_ipc_provider(global)?;

        let reward_source_subnet = SubnetID::from_str(&arguments.reward_source_subnet)?;
        let reward_claim_subnet = SubnetID::from_str(&arguments.reward_claim_subnet)?;
        let validator = Address::from_str(&arguments.validator)?;

        provider
            .batch_subnet_claim(
                &reward_claim_subnet,
                &reward_source_subnet,
                arguments.from,
                arguments.to,
                &validator,
            )
            .await?;

        println!("rewards claimed");

        Ok(())
    }
}

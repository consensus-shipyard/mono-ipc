// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//! List the bottom up checkpoint status

use std::fmt::Debug;
use std::str::FromStr;

use async_trait::async_trait;
use clap::Args;
use fvm_shared::clock::ChainEpoch;
use ipc_api::subnet_id::SubnetID;

use crate::commands::get_ipc_provider;
use crate::{CommandLineHandler, GlobalArguments};

const DEFAULT_LIMIT_UNSUBMITTED: usize = 10;

/// The command to list bottom up checkpoint status.
pub(crate) struct Status;

#[async_trait]
impl CommandLineHandler for Status {
    type Arguments = StatusArgs;

    async fn handle(global: &GlobalArguments, arguments: &Self::Arguments) -> anyhow::Result<()> {
        log::debug!("bottom up checkpoint status with args: {:?}", arguments);

        let provider = get_ipc_provider(global)?;
        let subnet = SubnetID::from_str(&arguments.subnet)?;

        let last_checkpointed_height = provider.last_bottom_up_checkpoint_height(&subnet).await?;
        let checkpoint = provider
            .get_bottom_up_bundle(&subnet, last_checkpointed_height)
            .await?;
        let period = provider.checkpoint_period(&subnet).await?;
        let chain_head = provider.get_chain_head_height(&subnet).await?;

        println!("subnet chain head height: {}", chain_head);
        println!(
            "last submitted checkpoint (@ subnet height {}): {:?}",
            last_checkpointed_height, checkpoint
        );

        let limit_unsubmitted = arguments
            .limit_unsubmitted
            .unwrap_or(DEFAULT_LIMIT_UNSUBMITTED);

        let start = last_checkpointed_height + 1;
        let ending = limit_unsubmitted as ChainEpoch * period + start;
        let mut checkpoints_ahead = 0;
        for h in start..=ending {
            let c = provider.get_bottom_up_bundle(&subnet, h).await?;
            if c.checkpoint.block_height != 0 {
                checkpoints_ahead += 1;
            }
        }
        println!(
            "there are at least {} unsubmitted checkpoints (limiting query to: {})",
            checkpoints_ahead, limit_unsubmitted
        );

        Ok(())
    }
}

#[derive(Debug, Args)]
#[command(about = "Summarise the bottom up checkpointing status of a subnet")]
pub(crate) struct StatusArgs {
    #[arg(long, help = "The subnet to inspect")]
    pub subnet: String,
    #[arg(
        long,
        help = "Limit unsubmitted checkpoints to print (looking forward from last submitted), default: 10"
    )]
    pub limit_unsubmitted: Option<usize>,
}

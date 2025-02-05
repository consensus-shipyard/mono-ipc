// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT

mod batch_claim;
mod list;

use crate::commands::validator::batch_claim::{BatchClaim, BatchClaimArgs};
use crate::commands::validator::list::{ListActivities, ListActivitiesArgs};
use crate::{CommandLineHandler, GlobalArguments};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(name = "validator", about = "validator reward related commands")]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct ValidatorCommandsArgs {
    #[command(subcommand)]
    command: Commands,
}

impl ValidatorCommandsArgs {
    pub async fn handle(&self, global: &GlobalArguments) -> anyhow::Result<()> {
        match &self.command {
            Commands::BatchClaim(args) => BatchClaim::handle(global, args).await,
            Commands::ListValidatorActivities(args) => ListActivities::handle(global, args).await,
        }
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    BatchClaim(BatchClaimArgs),
    ListValidatorActivities(ListActivitiesArgs),
}

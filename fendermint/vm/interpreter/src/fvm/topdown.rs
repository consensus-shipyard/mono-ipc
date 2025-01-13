// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

//! Topdown finality related util functions

use crate::fvm::state::ipc::GatewayCaller;
use crate::fvm::state::FvmExecState;
use crate::fvm::FvmApplyRet;
use anyhow::Context;
use fendermint_vm_topdown::Checkpoint;
use fvm_ipld_blockstore::Blockstore;
use ipc_api::cross::IpcEnvelope;

use super::state::ipc::tokens_to_mint;

/// Commit the topdown checkpoint. Returns the height that the previous parent checkpoint is committed and
/// the committed checkpoint itself. If there is no topdown checkpoint committed, genesis epoch is returned.
pub async fn commit_checkpoint<DB>(
    gateway_caller: &GatewayCaller<DB>,
    state: &mut FvmExecState<DB>,
    checkpoint: Checkpoint,
) -> anyhow::Result<Option<Checkpoint>>
where
    DB: Blockstore + Sync + Send + Clone + 'static,
{
    let prev_checkpoint = gateway_caller.commit_topdown_checkpoint(state, checkpoint)?;

    tracing::debug!("commit checkpoint parsed, prev_checkpoint: {prev_checkpoint:?}");

    Ok(prev_checkpoint)
}

/// Execute the top down messages implicitly. Before the execution, mint to the gateway of the funds
/// transferred in the messages, and increase the circulating supply with the incoming value.
pub async fn execute_topdown_msgs<DB>(
    gateway_caller: &GatewayCaller<DB>,
    state: &mut FvmExecState<DB>,
    messages: Vec<IpcEnvelope>,
) -> anyhow::Result<FvmApplyRet>
where
    DB: Blockstore + Sync + Send + Clone + 'static,
{
    let minted_tokens = tokens_to_mint(&messages);
    tracing::debug!(token = minted_tokens.to_string(), "tokens to mint in child");

    if !minted_tokens.is_zero() {
        gateway_caller
            .mint_to_gateway(state, minted_tokens.clone())
            .context("failed to mint to gateway")?;

        state.update_circ_supply(|circ_supply| {
            *circ_supply += minted_tokens;
        });
    }

    gateway_caller.apply_cross_messages(state, messages)
}

// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use crate::fvm::state::ipc::GatewayCaller;
use crate::fvm::store::ReadOnlyBlockstore;
use crate::fvm::{topdown, EndBlockOutput, FvmApplyRet};
use crate::selector::{GasLimitSelector, MessageSelector};
use crate::{
    fvm::state::FvmExecState,
    fvm::FvmMessage,
    signed::{SignedMessageApplyRes, SignedMessageCheckRes, SyntheticMessage, VerifiableMessage},
    CheckInterpreter, ExecInterpreter, ProposalInterpreter, QueryInterpreter,
};
use anyhow::{anyhow, Context};
use async_stm::atomically;
use async_trait::async_trait;
use fendermint_vm_actor_interface::ipc;
use fendermint_vm_genesis::ValidatorKey;
use fendermint_vm_message::{
    chain::ChainMessage,
    ipc::{BottomUpCheckpoint, CertifiedMessage, IpcMessage, SignedRelayedMessage},
};
use fendermint_vm_resolver::pool::{ResolveKey, ResolvePool};
use fendermint_vm_topdown::launch::Toggle;
use fendermint_vm_topdown::{Checkpoint, TopdownClient};
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::econ::TokenAmount;
use num_traits::Zero;
use std::sync::Arc;

/// A resolution pool for bottom-up and top-down checkpoints.
pub type CheckpointPool = ResolvePool<CheckpointPoolItem>;

/// These are the extra state items that the chain interpreter needs,
/// a sort of "environment" supporting IPC.
#[derive(Clone)]
pub struct ChainEnv {
    /// CID resolution pool.
    pub checkpoint_pool: CheckpointPool,
    /// The topdown checkpoint client
    pub topdown_client: Toggle<TopdownClient>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum CheckpointPoolItem {
    /// BottomUp checkpoints to be resolved from the originating subnet or the current one.
    BottomUp(CertifiedMessage<BottomUpCheckpoint>),
    // We can extend this to include top-down checkpoints as well, with slightly
    // different resolution semantics (resolving it from a trusted parent, and
    // awaiting finality before declaring it available).
}

impl From<&CheckpointPoolItem> for ResolveKey {
    fn from(value: &CheckpointPoolItem) -> Self {
        match value {
            CheckpointPoolItem::BottomUp(cp) => {
                (cp.message.subnet_id.clone(), cp.message.bottom_up_messages)
            }
        }
    }
}

/// A user sent a transaction which they are not allowed to do.
pub struct IllegalMessage;

// For now this is the only option, later we can expand.
pub enum ChainMessageApplyRet {
    Signed(SignedMessageApplyRes),
    /// The IPC chain message execution result
    Ipc(FvmApplyRet),
}

/// We only allow signed messages into the mempool.
pub type ChainMessageCheckRes = Result<SignedMessageCheckRes, IllegalMessage>;

/// Interpreter working on chain messages; in the future it will schedule
/// CID lookups to turn references into self-contained user or cross messages.
#[derive(Clone)]
pub struct ChainMessageInterpreter<I, DB> {
    inner: I,
    gateway_caller: GatewayCaller<DB>,
}

impl<I, DB> ChainMessageInterpreter<I, DB> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            gateway_caller: GatewayCaller::default(),
        }
    }
}

#[async_trait]
impl<I, DB> ProposalInterpreter for ChainMessageInterpreter<I, DB>
where
    DB: Blockstore + Clone + 'static + Send + Sync,
    I: Sync + Send,
{
    type State = (ChainEnv, FvmExecState<ReadOnlyBlockstore<Arc<DB>>>);
    type Message = ChainMessage;

    /// Check whether there are any "ready" messages in the IPLD resolution mempool which can be appended to the proposal.
    ///
    /// We could also use this to select the most profitable user transactions, within the gas limit. We can also take into
    /// account the transactions which are part of top-down or bottom-up checkpoints, to stay within gas limits.
    async fn prepare(
        &self,
        (chain_env, state): Self::State,
        mut msgs: Vec<Self::Message>,
    ) -> anyhow::Result<Vec<Self::Message>> {
        msgs = messages_selection(msgs, &state)?;

        // Collect resolved CIDs ready to be proposed from the pool.
        let ckpts = atomically(|| chain_env.checkpoint_pool.collect_resolved()).await;

        // Create transactions ready to be included on the chain.
        let ckpts = ckpts.into_iter().map(|ckpt| match ckpt {
            CheckpointPoolItem::BottomUp(ckpt) => ChainMessage::Ipc(IpcMessage::BottomUpExec(ckpt)),
        });

        match chain_env.topdown_client.find_topdown_proposal().await {
            Ok(Some(p)) => msgs.push(ChainMessage::Ipc(IpcMessage::TopDownExec(p))),
            Ok(None) => {}
            Err(e) => {
                tracing::error!(err = e.to_string(), "cannot find topdown proposal");
            }
        }

        // Append at the end - if we run out of block space, these are going to be reproposed in the next block.
        msgs.extend(ckpts);
        Ok(msgs)
    }

    /// Perform finality checks on top-down transactions and availability checks on bottom-up transactions.
    async fn process(
        &self,
        (chain_env, state): Self::State,
        msgs: Vec<Self::Message>,
    ) -> anyhow::Result<bool> {
        let mut block_gas_usage = 0;

        for msg in msgs {
            match msg {
                ChainMessage::Ipc(IpcMessage::BottomUpExec(msg)) => {
                    let item = CheckpointPoolItem::BottomUp(msg);

                    // We can just look in memory because when we start the application, we should retrieve any
                    // pending checkpoints (relayed but not executed) from the ledger, so they should be there.
                    // We don't have to validate the checkpoint here, because
                    // 1) we validated it when it was relayed, and
                    // 2) if a validator proposes something invalid, we can make them pay during execution.
                    let is_resolved =
                        atomically(|| match chain_env.checkpoint_pool.get_status(&item)? {
                            None => Ok(false),
                            Some(status) => status.is_resolved(),
                        })
                        .await;

                    if !is_resolved {
                        return Ok(false);
                    }
                }
                ChainMessage::Ipc(IpcMessage::TopDownExec(p)) => {
                    let proposal_height = p.cert.payload().parent_height();
                    match chain_env.topdown_client.validate_quorum_proposal(p).await {
                        Ok(_) => {
                            tracing::info!(proposal_height, "validated quorum proposal");
                        }
                        Err(e) => {
                            tracing::error!(err = e.to_string(), "cannot validate quorum proposal");
                            return Ok(false);
                        }
                    }
                }
                ChainMessage::Signed(signed) => {
                    block_gas_usage += signed.message.gas_limit;
                }
                _ => {}
            };
        }

        Ok(block_gas_usage <= state.block_gas_tracker().available())
    }
}

#[async_trait]
impl<I, DB> ExecInterpreter for ChainMessageInterpreter<I, DB>
where
    DB: Blockstore + Clone + 'static + Send + Sync + Clone,
    I: ExecInterpreter<
        Message = VerifiableMessage,
        DeliverOutput = SignedMessageApplyRes,
        State = FvmExecState<DB>,
        EndOutput = EndBlockOutput,
    >,
{
    // The state consists of the resolver pool, which this interpreter needs, and the rest of the
    // state which the inner interpreter uses. This is a technical solution because the pool doesn't
    // fit with the state we use for execution messages further down the stack, which depend on block
    // height and are used in queries as well.
    type State = (ChainEnv, I::State);
    type Message = ChainMessage;
    type BeginOutput = I::BeginOutput;
    type DeliverOutput = ChainMessageApplyRet;
    type EndOutput = I::EndOutput;

    async fn deliver(
        &self,
        (env, mut state): Self::State,
        msg: Self::Message,
    ) -> anyhow::Result<(Self::State, Self::DeliverOutput)> {
        match msg {
            ChainMessage::Signed(msg) => {
                let (state, ret) = self
                    .inner
                    .deliver(state, VerifiableMessage::Signed(msg))
                    .await?;
                Ok(((env, state), ChainMessageApplyRet::Signed(ret)))
            }
            ChainMessage::Ipc(msg) => match msg {
                IpcMessage::BottomUpResolve(msg) => {
                    let smsg = relayed_bottom_up_ckpt_to_fvm(&msg)
                        .context("failed to syntesize FVM message")?;

                    // Let the FVM validate the checkpoint quorum certificate and take note of the relayer for rewards.
                    let (state, ret) = self
                        .inner
                        .deliver(state, VerifiableMessage::Synthetic(smsg))
                        .await
                        .context("failed to deliver bottom up checkpoint")?;

                    // If successful, add the CID to the background resolution pool.
                    let is_success = match ret {
                        Ok(ref ret) => ret.fvm.apply_ret.msg_receipt.exit_code.is_success(),
                        Err(_) => false,
                    };

                    if is_success {
                        // For now try to get it from the child subnet. If the same comes up for execution, include own.
                        atomically(|| {
                            env.checkpoint_pool.add(
                                CheckpointPoolItem::BottomUp(msg.message.message.clone()),
                                false,
                            )
                        })
                        .await;
                    }

                    // We can use the same result type for now, it's isomorphic.
                    Ok(((env, state), ChainMessageApplyRet::Signed(ret)))
                }
                IpcMessage::BottomUpExec(_) => {
                    todo!("#197: implement BottomUp checkpoint execution")
                }
                IpcMessage::TopDownExec(p) => {
                    let checkpoint = Checkpoint::from(p.cert.payload());
                    tracing::debug!(
                        checkpoint = checkpoint.to_string(),
                        "chain interpreter received topdown exec proposal",
                    );

                    let prev_checkpoint = topdown::commit_checkpoint(
                        &self.gateway_caller,
                        &mut state,
                        checkpoint.clone(),
                    )
                    .await
                    .context("failed to commit finality")?;

                    tracing::debug!(
                        previous_committed_finality = prev_checkpoint
                            .as_ref()
                            .map(|f| format!("{f}"))
                            .unwrap_or_else(|| String::from("None")),
                        "chain interpreter committed topdown finality",
                    );

                    // error happens if we cannot get the validator set from ipc agent after retries
                    let validator_changes = p.effects.1;

                    tracing::debug!(
                        msgs = validator_changes.len(),
                        "chain interpreter received total validator changes"
                    );

                    self.gateway_caller
                        .store_validator_changes(&mut state, validator_changes)
                        .context("failed to store validator changes")?;

                    let msgs = p.effects.0;

                    tracing::debug!(
                        number_of_messages = msgs.len(),
                        "chain interpreter received topdown msgs",
                    );

                    let ret = topdown::execute_topdown_msgs(&self.gateway_caller, &mut state, msgs)
                        .await
                        .context("failed to execute top down messages")?;

                    tracing::debug!("chain interpreter applied topdown msgs");

                    env.topdown_client
                        .parent_finalized(checkpoint.clone())
                        .await?;

                    tracing::debug!(
                        checkpoint = checkpoint.to_string(),
                        "chain interpreter has set new topdown checkpoint"
                    );

                    Ok(((env, state), ChainMessageApplyRet::Ipc(ret)))
                }
            },
        }
    }

    async fn begin(
        &self,
        (env, state): Self::State,
    ) -> anyhow::Result<(Self::State, Self::BeginOutput)> {
        let (state, out) = self.inner.begin(state).await?;
        Ok(((env, state), out))
    }

    async fn end(
        &self,
        (env, state): Self::State,
    ) -> anyhow::Result<(Self::State, Self::EndOutput)> {
        let (state, out) = self.inner.end(state).await?;

        // Update any component that needs to know about changes in the power table.
        if !out.power_updates.0.is_empty() {
            let power_updates = out
                .power_updates
                .0
                .iter()
                .map(|v| {
                    let vk = ValidatorKey::new(v.public_key.0);
                    let w = v.power.0;
                    (vk, w)
                })
                .collect::<Vec<_>>();

            env.topdown_client.update_power_table(power_updates).await?;
        }

        Ok(((env, state), out))
    }
}

#[async_trait]
impl<I, DB> CheckInterpreter for ChainMessageInterpreter<I, DB>
where
    DB: Blockstore + Clone + 'static + Send + Sync,
    I: CheckInterpreter<Message = VerifiableMessage, Output = SignedMessageCheckRes>,
{
    type State = I::State;
    type Message = ChainMessage;
    type Output = ChainMessageCheckRes;

    async fn check(
        &self,
        state: Self::State,
        msg: Self::Message,
        is_recheck: bool,
    ) -> anyhow::Result<(Self::State, Self::Output)> {
        match msg {
            ChainMessage::Signed(msg) => {
                let (state, ret) = self
                    .inner
                    .check(state, VerifiableMessage::Signed(msg), is_recheck)
                    .await?;

                Ok((state, Ok(ret)))
            }
            ChainMessage::Ipc(msg) => {
                match msg {
                    IpcMessage::BottomUpResolve(msg) => {
                        let msg = relayed_bottom_up_ckpt_to_fvm(&msg)
                            .context("failed to syntesize FVM message")?;

                        let (state, ret) = self
                            .inner
                            .check(state, VerifiableMessage::Synthetic(msg), is_recheck)
                            .await
                            .context("failed to check bottom up resolve")?;

                        Ok((state, Ok(ret)))
                    }
                    IpcMessage::TopDownExec(_) | IpcMessage::BottomUpExec(_) => {
                        // Users cannot send these messages, only validators can propose them in blocks.
                        Ok((state, Err(IllegalMessage)))
                    }
                }
            }
        }
    }
}

#[async_trait]
impl<I, DB> QueryInterpreter for ChainMessageInterpreter<I, DB>
where
    DB: Blockstore + Clone + 'static + Send + Sync,
    I: QueryInterpreter,
{
    type State = I::State;
    type Query = I::Query;
    type Output = I::Output;

    async fn query(
        &self,
        state: Self::State,
        qry: Self::Query,
    ) -> anyhow::Result<(Self::State, Self::Output)> {
        self.inner.query(state, qry).await
    }
}

/// Convert a signed relayed bottom-up checkpoint to a syntetic message we can send to the FVM.
///
/// By mapping to an FVM message we invoke the right contract to validate the checkpoint,
/// and automatically charge the relayer gas for the execution of the check, but not the
/// execution of the cross-messages, which aren't part of the payload.
fn relayed_bottom_up_ckpt_to_fvm(
    relayed: &SignedRelayedMessage<CertifiedMessage<BottomUpCheckpoint>>,
) -> anyhow::Result<SyntheticMessage> {
    // TODO #192: Convert the checkpoint to what the actor expects.
    let params = RawBytes::default();

    let msg = FvmMessage {
        version: 0,
        from: relayed.message.relayer,
        to: ipc::GATEWAY_ACTOR_ADDR,
        sequence: relayed.message.sequence,
        value: TokenAmount::zero(),
        method_num: ipc::gateway::METHOD_INVOKE_CONTRACT,
        params,
        gas_limit: relayed.message.gas_limit,
        gas_fee_cap: relayed.message.gas_fee_cap.clone(),
        gas_premium: relayed.message.gas_premium.clone(),
    };

    let msg = SyntheticMessage::new(msg, &relayed.message, relayed.signature.clone())
        .context("failed to create syntetic message")?;

    Ok(msg)
}

/// Selects messages to be executed. Currently, this is a static function whose main purpose is to
/// coordinate various selectors. However, it does not have formal semantics for doing so, e.g.
/// do we daisy-chain selectors, do we parallelize, how do we treat rejections and acceptances?
/// It hasn't been well thought out yet. When we refactor the whole *Interpreter stack, we will
/// revisit this and make the selection function properly pluggable.
fn messages_selection<DB: Blockstore + Clone + 'static>(
    msgs: Vec<ChainMessage>,
    state: &FvmExecState<DB>,
) -> anyhow::Result<Vec<ChainMessage>> {
    let mut user_msgs = msgs
        .into_iter()
        .map(|msg| match msg {
            ChainMessage::Signed(inner) => Ok(inner),
            ChainMessage::Ipc(_) => Err(anyhow!("should not have ipc messages in user proposals")),
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Currently only one selector, we can potentially extend to more selectors
    // This selector enforces that the total cumulative gas limit of all messages is less than the
    // currently active block gas limit.
    let selectors = vec![GasLimitSelector {}];
    for s in selectors {
        user_msgs = s.select_messages(state, user_msgs)
    }

    Ok(user_msgs.into_iter().map(ChainMessage::Signed).collect())
}

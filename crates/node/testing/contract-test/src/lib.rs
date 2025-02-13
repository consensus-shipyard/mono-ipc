// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::{anyhow, Context, Result};
use byteorder::{BigEndian, WriteBytesExt};
use fendermint_vm_core::Timestamp;
use fvm_shared::clock::ChainEpoch;
use std::{future::Future, sync::Arc};

use fendermint_crypto::PublicKey;
use fendermint_vm_genesis::Genesis;
use fendermint_vm_interpreter::fvm::EndBlockOutput;
use fendermint_vm_interpreter::genesis::{create_test_genesis_state, GenesisOutput};
use fendermint_vm_interpreter::{
    fvm::{
        bundle::{bundle_path, contracts_path, custom_actors_bundle_path},
        state::{FvmExecState, FvmStateParams, FvmUpdatableParams},
        store::memory::MemoryBlockstore,
        FvmApplyRet, FvmMessage,
    },
    ExecInterpreter,
};
use fvm::engine::MultiEngine;

pub mod ipc;

pub async fn create_test_exec_state(
    genesis: Genesis,
) -> Result<(
    FvmExecState<MemoryBlockstore>,
    GenesisOutput,
    MemoryBlockstore,
)> {
    let bundle_path = bundle_path();
    let custom_actors_bundle_path = custom_actors_bundle_path();
    let artifacts_path = contracts_path();

    let (state, out) = create_test_genesis_state(
        bundle_path,
        custom_actors_bundle_path,
        artifacts_path,
        genesis,
    )
    .await?;
    let store = state.store().clone();
    Ok((
        state
            .into_exec_state()
            .map_err(|_| anyhow!("cannot parse state"))?,
        out,
        store,
    ))
}

pub struct Tester<I> {
    interpreter: Arc<I>,
    state_store: Arc<MemoryBlockstore>,
    multi_engine: Arc<MultiEngine>,
    exec_state: Arc<tokio::sync::Mutex<Option<FvmExecState<MemoryBlockstore>>>>,
    state_params: FvmStateParams,
}

impl<I> Tester<I>
where
    I: ExecInterpreter<
        State = FvmExecState<MemoryBlockstore>,
        Message = FvmMessage,
        BeginOutput = FvmApplyRet,
        DeliverOutput = FvmApplyRet,
        EndOutput = EndBlockOutput,
    >,
{
    pub async fn new(interpreter: I, genesis: Genesis) -> anyhow::Result<Self> {
        let (exec_state, out, store) = create_test_exec_state(genesis).await?;
        let (state_root, _, _) = exec_state
            .commit()
            .context("failed to commit genesis state")?;

        let state_params = FvmStateParams {
            state_root,
            timestamp: out.timestamp,
            network_version: out.network_version,
            base_fee: out.base_fee,
            circ_supply: out.circ_supply,
            chain_id: out.chain_id.into(),
            power_scale: out.power_scale,
            app_version: 0,
            consensus_params: None,
        };

        Ok(Self {
            interpreter: Arc::new(interpreter),
            state_store: Arc::new(store),
            multi_engine: Arc::new(MultiEngine::new(1)),
            exec_state: Arc::new(tokio::sync::Mutex::new(None)),
            state_params,
        })
    }

    /// Take the execution state, update it, put it back, return the output.
    pub async fn modify_exec_state<T, F, R>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(FvmExecState<MemoryBlockstore>) -> R,
        R: Future<Output = Result<(FvmExecState<MemoryBlockstore>, T)>>,
    {
        let mut guard = self.exec_state.lock().await;
        let state = guard.take().expect("exec state empty");

        let (state, ret) = f(state).await?;

        *guard = Some(state);

        Ok(ret)
    }

    /// Put the execution state during block execution. Has to be empty.
    async fn put_exec_state(&self, state: FvmExecState<MemoryBlockstore>) {
        let mut guard = self.exec_state.lock().await;
        assert!(guard.is_none(), "exec state not empty");
        *guard = Some(state);
    }

    /// Take the execution state during block execution. Has to be non-empty.
    async fn take_exec_state(&self) -> FvmExecState<MemoryBlockstore> {
        let mut guard = self.exec_state.lock().await;
        guard.take().expect("exec state empty")
    }

    pub async fn begin_block(&self, block_height: ChainEpoch, producer: PublicKey) -> Result<()> {
        let mut block_hash: [u8; 32] = [0; 32];
        let _ = block_hash.as_mut().write_i64::<BigEndian>(block_height);

        let db = self.state_store.as_ref().clone();
        let mut state_params = self.state_params.clone();
        state_params.timestamp = Timestamp(block_height as u64);

        let state = FvmExecState::new(db, self.multi_engine.as_ref(), block_height, state_params)
            .context("error creating new state")?
            .with_block_hash(block_hash)
            .with_block_producer(producer);

        self.put_exec_state(state).await;

        let _res = self
            .modify_exec_state(|s| self.interpreter.begin(s))
            .await?;

        Ok(())
    }

    pub async fn execute_msgs(&self, msgs: Vec<FvmMessage>) -> Result<()> {
        self.modify_exec_state(|mut s| async {
            for msg in msgs {
                let (a, out) = self.interpreter.deliver(s, msg).await?;
                if let Some(e) = out.apply_ret.failure_info {
                    println!("failed: {}", e);
                    return Err(anyhow!("err in msg deliver"));
                }
                s = a;
            }
            Ok((s, ()))
        })
        .await
        .context("execute msgs failed")
    }

    pub async fn end_block(&self, _block_height: ChainEpoch) -> Result<()> {
        let _ret = self
            .modify_exec_state(|s| self.interpreter.end(s))
            .await
            .context("end failed")?;

        Ok(())
    }

    pub async fn commit(&mut self) -> Result<()> {
        let exec_state = self.take_exec_state().await;

        let (
            state_root,
            FvmUpdatableParams {
                app_version,
                base_fee,
                circ_supply,
                power_scale,
            },
            _,
        ) = exec_state.commit().context("failed to commit FVM")?;

        self.state_params.state_root = state_root;
        self.state_params.app_version = app_version;
        self.state_params.base_fee = base_fee;
        self.state_params.circ_supply = circ_supply;
        self.state_params.power_scale = power_scale;

        eprintln!("self.state_params: {:?}", self.state_params);

        Ok(())
    }

    pub fn state_params(&self) -> FvmStateParams {
        self.state_params.clone()
    }
}

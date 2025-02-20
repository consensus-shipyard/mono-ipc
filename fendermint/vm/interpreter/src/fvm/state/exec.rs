// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::collections::{HashMap, HashSet};

use crate::fvm::activity::actor::ActorActivityTracker;
use crate::fvm::externs::FendermintExterns;
use crate::fvm::gas::BlockGasTracker;
use crate::fvm::state::priority::TxnPriorityCalculator;
use anyhow::Ok;
use cid::Cid;
use fendermint_actors_api::gas_market::Reading;
use fendermint_crypto::PublicKey;
use fendermint_vm_actor_interface::eam::EthAddress;
use fendermint_vm_core::{chainid::HasChainID, Timestamp};
use fendermint_vm_encoding::IsHumanReadable;
use fendermint_vm_genesis::PowerScale;
use fvm::executor::{ExecutionOptions, TxnGasHook};
use fvm::{
    call_manager::DefaultCallManager,
    engine::MultiEngine,
    executor::{ApplyFailure, ApplyKind, ApplyRet, DefaultExecutor, Executor},
    machine::{DefaultMachine, Machine, Manifest, NetworkConfig},
    state_tree::StateTree,
    DefaultKernel,
};
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::{
    address::Address, chainid::ChainID, clock::ChainEpoch, econ::TokenAmount, error::ExitCode,
    message::Message, receipt::Receipt, version::NetworkVersion, ActorID,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt;
use tendermint::consensus::params::Params as TendermintConsensusParams;

const ALWAYS_REVERT: bool = true;
pub type BlockHash = [u8; 32];

pub type ActorAddressMap = HashMap<ActorID, Address>;

/// The result of the message application bundled with any delegated addresses of event emitters.
pub type ExecResult = anyhow::Result<(ApplyRet, ActorAddressMap)>;

/// Parts of the state which evolve during the lifetime of the chain.
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct FvmStateParams {
    /// Root CID of the actor state map.
    #[serde_as(as = "IsHumanReadable")]
    pub state_root: Cid,
    /// Last applied block time stamp.
    pub timestamp: Timestamp,
    /// FVM network version.
    pub network_version: NetworkVersion,
    /// Base fee for contract execution.
    #[serde_as(as = "IsHumanReadable")]
    pub base_fee: TokenAmount,
    /// Current circulating supply; changes in the context of IPC.
    #[serde_as(as = "IsHumanReadable")]
    pub circ_supply: TokenAmount,
    /// The [`ChainID`] is stored here to hint at the possibility that
    /// a chain ID might change during the lifetime of a chain, in case
    /// there is a fork, or perhaps a subnet migration in IPC.
    ///
    /// How exactly that would be communicated is uknown at this point.
    pub chain_id: u64,
    /// Conversion from collateral to voting power.
    pub power_scale: PowerScale,
    /// The application protocol version.
    #[serde(default)]
    pub app_version: u64,
    /// Tendermint consensus params.
    pub consensus_params: Option<TendermintConsensusParams>,
}

/// Custom implementation of Debug to exclude `consensus_params` from the debug output
/// if it is `None`. This ensures consistency between the debug output and JSON/CBOR
/// serialization, which omits `None` values for `consensus_params`. See: fendermint/vm/interpreter/tests/golden.rs.
///
/// This implementation is temporary and should be removed once `consensus_params` is
/// no longer part of `FvmStateParams`.
///
/// @TODO: Remove this implementation when `consensus_params` is deprecated.
impl fmt::Debug for FvmStateParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("FvmStateParams");

        ds.field("state_root", &self.state_root)
            .field("timestamp", &self.timestamp)
            .field("network_version", &self.network_version)
            .field("base_fee", &self.base_fee)
            .field("circ_supply", &self.circ_supply)
            .field("chain_id", &self.chain_id)
            .field("power_scale", &self.power_scale)
            .field("app_version", &self.app_version);

        // Only include `consensus_params` in the debug output if it is `Some`.
        if let Some(ref params) = self.consensus_params {
            ds.field("consensus_params", params);
        }

        ds.finish()
    }
}

/// Parts of the state which can be updated by message execution, apart from the actor state.
///
/// This is just a technical thing to help us not forget about saving something.
///
/// TODO: `base_fee` should surely be here.
#[derive(Debug)]
pub struct FvmUpdatableParams {
    /// The application protocol version, which changes during upgrades.
    pub app_version: u64,
    /// The base fee has currently no automatic rules of being updated,
    /// but it's exposed to upgrades.
    pub base_fee: TokenAmount,
    /// The circulating supply changes if IPC is enabled and
    /// funds/releases are carried out with the parent.
    pub circ_supply: TokenAmount,
    /// Conversion between collateral and voting power.
    /// Doesn't change at the moment but in theory it could,
    /// and it doesn't have a place within the FVM.
    pub power_scale: PowerScale,
}

pub type MachineBlockstore<DB> = <DefaultMachine<DB, FendermintExterns<DB>> as Machine>::Blockstore;

/// A state we create for the execution of all the messages in a block.
pub struct FvmExecState<DB>
where
    DB: Blockstore + Clone + 'static,
{
    #[allow(clippy::type_complexity)]
    executor: DefaultExecutor<
        DefaultKernel<DefaultCallManager<DefaultMachine<DB, FendermintExterns<DB>>>>,
    >,
    /// Hash of the block currently being executed. For queries and checks this is empty.
    ///
    /// The main motivation to add it here was to make it easier to pass in data to the
    /// execution interpreter without having to add yet another piece to track at the app level.
    block_hash: Option<BlockHash>,
    /// Public key of the validator who created this block. For queries, checks, and proposal
    /// validations this is None.
    block_producer: Option<PublicKey>,
    /// Keeps track of block gas usage during execution, and takes care of updating
    /// the chosen gas market strategy (by default an on-chain actor delivering EIP-1559 behaviour).
    block_gas_tracker: BlockGasTracker,
    /// State of parameters that are outside the control of the FVM but can change and need to be persisted.
    params: FvmUpdatableParams,
    /// Indicate whether the parameters have been updated.
    params_dirty: bool,

    txn_priority: TxnPriorityCalculator,
}

impl<DB> FvmExecState<DB>
where
    DB: Blockstore + Clone + 'static,
{
    /// Create a new FVM execution environment.
    ///
    /// Calling this can be very slow unless we run in `--release` mode, because the [DefaultExecutor]
    /// pre-loads builtin-actor CIDs and wasm in debug mode is slow to instrument.
    pub fn new(
        blockstore: DB,
        multi_engine: &MultiEngine,
        block_height: ChainEpoch,
        params: FvmStateParams,
    ) -> anyhow::Result<Self> {
        let mut nc = NetworkConfig::new(params.network_version);
        nc.chain_id = ChainID::from(params.chain_id);

        // TODO: Configure:
        // * circ_supply; by default it's for Filecoin
        // * base_fee; by default it's zero
        let mut mc = nc.for_epoch(block_height, params.timestamp.0, params.state_root);
        mc.set_base_fee(params.base_fee.clone());
        mc.set_circulating_supply(params.circ_supply.clone());

        // Creating a new machine every time is prohibitively slow.
        // let ec = EngineConfig::from(&nc);
        // let engine = EnginePool::new_default(ec)?;

        let engine = multi_engine.get(&nc)?;
        let externs = FendermintExterns::new(blockstore.clone(), params.state_root);
        let machine = DefaultMachine::new(&mc, blockstore.clone(), externs)?;
        let mut executor = DefaultExecutor::new(engine.clone(), machine)?;

        let block_gas_tracker = BlockGasTracker::create(&mut executor)?;
        let base_fee = block_gas_tracker.base_fee().clone();

        Ok(Self {
            executor,
            block_hash: None,
            block_producer: None,
            block_gas_tracker,
            params: FvmUpdatableParams {
                app_version: params.app_version,
                base_fee: params.base_fee,
                circ_supply: params.circ_supply,
                power_scale: params.power_scale,
            },
            params_dirty: false,
            txn_priority: TxnPriorityCalculator::new(base_fee),
        })
    }

    /// Set the block hash during execution.
    pub fn with_block_hash(mut self, block_hash: BlockHash) -> Self {
        self.block_hash = Some(block_hash);
        self
    }

    /// Set the validator during execution.
    pub fn with_block_producer(mut self, pubkey: PublicKey) -> Self {
        self.block_producer = Some(pubkey);
        self
    }

    pub fn block_gas_tracker(&self) -> &BlockGasTracker {
        &self.block_gas_tracker
    }

    pub fn block_gas_tracker_mut(&mut self) -> &mut BlockGasTracker {
        &mut self.block_gas_tracker
    }

    pub fn read_gas_market(&mut self) -> anyhow::Result<Reading> {
        BlockGasTracker::read_gas_market(&mut self.executor)
    }

    /// Execute message implicitly.
    pub fn execute_implicit(&mut self, msg: Message) -> ExecResult {
        self.execute_message(msg, ApplyKind::Implicit)
    }

    pub fn execute_read_only(&mut self, msg: Message) -> ExecResult {
        if let Err(e) = msg.check() {
            return Ok(check_error(e));
        }

        let raw_length = message_raw_length(&msg)?;
        let ret = self.executor.execute_message_with_revert(
            msg,
            ApplyKind::Implicit,
            raw_length,
            ALWAYS_REVERT,
        )?;
        let addrs = self.emitter_delegated_addresses(&ret)?;

        Ok((ret, addrs))
    }

    /// Execute message implicitly but ensures the execution is successful and returns only the ApplyRet.
    pub fn execute_implicit_ok(&mut self, msg: Message) -> ExecResult {
        let r = self.execute_implicit(msg)?;
        if let Some(err) = &r.0.failure_info {
            anyhow::bail!("failed to apply message: {}", err)
        } else {
            Ok(r)
        }
    }

    /// Execute message explicitly.
    pub fn execute_explicit(&mut self, msg: Message) -> ExecResult {
        self.execute_message(msg, ApplyKind::Explicit)
    }

    pub fn execute_message(&mut self, msg: Message, kind: ApplyKind) -> ExecResult {
        if let Err(e) = msg.check() {
            return Ok(check_error(e));
        }

        // TODO: We could preserve the message length by changing the input type.
        let raw_length = message_raw_length(&msg)?;
        let ret = self.executor.execute_message(msg, kind, raw_length)?;
        let addrs = self.emitter_delegated_addresses(&ret)?;

        // Record the utilization of this message if the apply type was Explicit.
        if kind == ApplyKind::Explicit {
            self.block_gas_tracker.record_utilization(&ret);
        }

        Ok((ret, addrs))
    }

    /// Execute a function with the internal executor and return an arbitrary result.
    pub fn execute_with_executor<F, R>(&mut self, exec_func: F) -> anyhow::Result<R>
    where
        F: FnOnce(
            &mut DefaultExecutor<
                DefaultKernel<DefaultCallManager<DefaultMachine<DB, FendermintExterns<DB>>>>,
            >,
        ) -> anyhow::Result<R>,
    {
        exec_func(&mut self.executor)
    }

    /// Directly call the fvm execute with options
    pub fn execute_with_options<F: TxnGasHook>(
        &mut self,
        msg: Message,
        kind: ApplyKind,
        options: ExecutionOptions<F>,
    ) -> ExecResult {
        if let Err(e) = msg.check() {
            return Ok(check_error(e));
        }

        // TODO: We could preserve the message length by changing the input type.
        let raw_length = message_raw_length(&msg)?;
        let ret = self
            .executor
            .execute_message_with_options(msg, kind, raw_length, options)?;
        let addrs = self.emitter_delegated_addresses(&ret)?;

        // Record the utilization of this message if the apply type was Explicit.
        if kind == ApplyKind::Explicit {
            self.block_gas_tracker.record_utilization(&ret);
        }

        Ok((ret, addrs))
    }

    /// Commit the state. It must not fail, but we're returning a result so that error
    /// handling can be done in the application root.
    ///
    /// For now this is not part of the `Interpreter` because it's not clear what atomic
    /// semantics we can hope to provide if the middlewares call each other: did it go
    /// all the way down, or did it stop somewhere? Easier to have one commit of the state
    /// as a whole.
    pub fn commit(mut self) -> anyhow::Result<(Cid, FvmUpdatableParams, bool)> {
        let cid = self.executor.flush()?;
        Ok((cid, self.params, self.params_dirty))
    }

    /// The height of the currently executing block.
    pub fn block_height(&self) -> ChainEpoch {
        self.executor.context().epoch
    }

    /// Identity of the block being executed, if we are indeed executing any blocks.
    pub fn block_hash(&self) -> Option<BlockHash> {
        self.block_hash
    }

    /// Identity of the block producer, if we are indeed executing any blocks.
    pub fn block_producer(&self) -> Option<PublicKey> {
        self.block_producer
    }

    /// The timestamp of the currently executing block.
    pub fn timestamp(&self) -> Timestamp {
        Timestamp(self.executor.context().timestamp)
    }

    /// Conversion between collateral and voting power.
    pub fn power_scale(&self) -> PowerScale {
        self.params.power_scale
    }

    pub fn txn_priority_calculator(&self) -> &TxnPriorityCalculator {
        &self.txn_priority
    }

    pub fn app_version(&self) -> u64 {
        self.params.app_version
    }

    /// Get a mutable reference to the underlying [StateTree].
    pub fn state_tree_mut(&mut self) -> &mut StateTree<MachineBlockstore<DB>> {
        self.executor.state_tree_mut()
    }

    /// Get a reference to the underlying [StateTree].
    pub fn state_tree(&self) -> &StateTree<MachineBlockstore<DB>> {
        self.executor.state_tree()
    }

    /// Built-in actor manifest to inspect code CIDs.
    pub fn builtin_actors(&self) -> &Manifest {
        self.executor.builtin_actors()
    }

    /// The [ChainID] from the network configuration.
    pub fn chain_id(&self) -> ChainID {
        self.executor.context().network.chain_id
    }

    pub fn activity_tracker(&mut self) -> ActorActivityTracker<DB> {
        ActorActivityTracker { executor: self }
    }

    /// Collect all the event emitters' delegated addresses, for those who have any.
    fn emitter_delegated_addresses(&self, apply_ret: &ApplyRet) -> anyhow::Result<ActorAddressMap> {
        let emitter_ids = apply_ret
            .events
            .iter()
            .map(|e| e.emitter)
            .collect::<HashSet<_>>();

        let mut emitters = HashMap::default();

        for id in emitter_ids {
            if let Some(actor) = self.executor.state_tree().get_actor(id)? {
                if let Some(addr) = actor.delegated_address {
                    emitters.insert(id, addr);
                }
            }
        }

        Ok(emitters)
    }

    /// Update the application version.
    pub fn update_app_version<F>(&mut self, f: F)
    where
        F: FnOnce(&mut u64),
    {
        self.update_params(|p| f(&mut p.app_version))
    }

    /// Finalizes updates to the gas market based on the transactions processed by this instance.
    /// Returns the new base fee for the next height.
    pub fn finalize_gas_market(&mut self) -> anyhow::Result<Reading> {
        let premium_recipient = match self.block_producer {
            Some(pubkey) => Some(Address::from(EthAddress::new_secp256k1(
                &pubkey.serialize(),
            )?)),
            None => None,
        };

        self.block_gas_tracker
            .finalize(&mut self.executor, premium_recipient)
            .inspect(|reading| self.update_params(|p| p.base_fee = reading.base_fee.clone()))
    }

    /// Update the circulating supply, effective from the next block.
    pub fn update_circ_supply<F>(&mut self, f: F)
    where
        F: FnOnce(&mut TokenAmount),
    {
        self.update_params(|p| f(&mut p.circ_supply))
    }

    /// Update the parameters and mark them as dirty.
    fn update_params<F>(&mut self, f: F)
    where
        F: FnOnce(&mut FvmUpdatableParams),
    {
        f(&mut self.params);
        self.params_dirty = true;
    }
}

impl<DB> HasChainID for FvmExecState<DB>
where
    DB: Blockstore + Clone,
{
    fn chain_id(&self) -> ChainID {
        self.executor.context().network.chain_id
    }
}

/// The FVM would return an error from `DefaultExecutor::preflight_message` if it was called
/// with a message that doesn't pass basic checks, for example it has no gas limit, as opposed
/// to returning an `ApplyRet`. This would cause our application to fail.
/// I'm not sure if it's intentional, or how Lotus handles it, it's not desireable to crash
/// because such messages can be included by malicious validators or user queries. We could
/// use ABCI++ to filter out messages from blocks, but that doesn't affect queries, so we
/// might as well encode it as an error. To keep the types simpler, let's fabricate an `ApplyRet`.
fn check_error(e: anyhow::Error) -> (ApplyRet, ActorAddressMap) {
    let zero = TokenAmount::from_atto(0);
    let ret = ApplyRet {
        msg_receipt: Receipt {
            exit_code: ExitCode::SYS_ASSERTION_FAILED,
            return_data: RawBytes::default(),
            gas_used: 0,
            events_root: None,
        },
        penalty: zero.clone(),
        miner_tip: zero.clone(),
        base_fee_burn: zero.clone(),
        over_estimation_burn: zero.clone(),
        refund: zero,
        gas_refund: 0,
        gas_burned: 0,
        failure_info: Some(ApplyFailure::PreValidation(format!("{:#}", e))),
        exec_trace: Vec::new(),
        events: Vec::new(),
    };
    (ret, Default::default())
}

pub(crate) fn message_raw_length(msg: &Message) -> anyhow::Result<usize> {
    Ok(fvm_ipld_encoding::to_vec(msg).map(|bz| bz.len())?)
}

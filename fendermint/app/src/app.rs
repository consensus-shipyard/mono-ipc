// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use std::future::Future;
use std::sync::Arc;

use crate::observe::{
    BlockCommitted, BlockProposalEvaluated, BlockProposalReceived, BlockProposalSent, Message,
    MpoolReceived,
};
use crate::validators::ValidatorCache;
use crate::AppExitCode;
use crate::BlockHeight;
use crate::{tmconv::*, VERSION};
use anyhow::{anyhow, Context, Result};
use async_stm::{atomically, atomically_or_err};
use async_trait::async_trait;
use cid::Cid;
use fendermint_abci::util::take_until_max_size;
use fendermint_abci::{AbciResult, Application};
use fendermint_actors_api::gas_market::Reading;
use fendermint_crypto::PublicKey;
use fendermint_storage::{
    Codec, Encode, KVCollection, KVRead, KVReadable, KVStore, KVWritable, KVWrite,
};
use fendermint_vm_core::Timestamp;
use fendermint_vm_interpreter::bytes::{
    BytesMessageApplyRes, BytesMessageCheckRes, BytesMessageQuery, BytesMessageQueryRes,
};
use fendermint_vm_interpreter::chain::{ChainEnv, ChainMessageApplyRet, IllegalMessage};
use fendermint_vm_interpreter::fvm::state::{
    empty_state_tree, CheckStateRef, FvmExecState, FvmQueryState, FvmStateParams,
    FvmUpdatableParams,
};
use fendermint_vm_interpreter::fvm::store::ReadOnlyBlockstore;
use fendermint_vm_interpreter::fvm::{EndBlockOutput, FvmApplyRet};
use fendermint_vm_interpreter::genesis::{read_genesis_car, GenesisAppState};
use fendermint_vm_interpreter::signed::InvalidSignature;
use fendermint_vm_interpreter::{
    CheckInterpreter, ExecInterpreter, ProposalInterpreter, QueryInterpreter,
};
use fendermint_vm_message::query::FvmQueryHeight;
use fendermint_vm_snapshot::{SnapshotClient, SnapshotError};
use fendermint_vm_topdown::launch::Toggle;
use fendermint_vm_topdown::TopdownClient;
use fvm::engine::MultiEngine;
use fvm_ipld_blockstore::Blockstore;
use fvm_shared::chainid::ChainID;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::econ::TokenAmount;
use fvm_shared::version::NetworkVersion;
use ipc_observability::{emit, serde::HexEncodableBlockHash};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use tendermint::abci::request::CheckTxKind;
use tendermint::abci::{request, response};
use tendermint::consensus::params::Params as TendermintConsensusParams;
use tracing::instrument;

#[derive(Serialize)]
#[repr(u8)]
pub enum AppStoreKey {
    State,
}

// TODO: What range should we use for our own error codes? Should we shift FVM errors?
#[derive(Debug)]
#[repr(u32)]
pub enum AppError {
    /// Failed to deserialize the transaction.
    InvalidEncoding = 51,
    /// Failed to validate the user signature.
    InvalidSignature = 52,
    /// User sent a message they should not construct.
    IllegalMessage = 53,
    /// The genesis block hasn't been initialized yet.
    NotInitialized = 54,
}

/// The application state record we keep a history of in the database.
#[derive(Serialize, Deserialize)]
pub struct AppState {
    /// Last committed block height.
    block_height: BlockHeight,
    /// Oldest state hash height.
    oldest_state_height: BlockHeight,
    /// Last committed version of the evolving state of the FVM.
    state_params: FvmStateParams,
}

impl AppState {
    pub fn state_root(&self) -> Cid {
        self.state_params.state_root
    }

    pub fn chain_id(&self) -> ChainID {
        ChainID::from(self.state_params.chain_id)
    }

    pub fn app_hash(&self) -> tendermint::hash::AppHash {
        to_app_hash(&self.state_params)
    }

    /// The state is effective at the *next* block, that is, the effects of block N are visible in the header of block N+1,
    /// so the height of the state itself as a "post-state" is one higher than the block which we executed to create it.
    pub fn state_height(&self) -> BlockHeight {
        self.block_height + 1
    }
}

pub struct AppConfig<S: KVStore> {
    /// Namespace to store the current app state.
    pub app_namespace: S::Namespace,
    /// Namespace to store the app state history.
    pub state_hist_namespace: S::Namespace,
    /// Size of state history to keep; 0 means unlimited.
    pub state_hist_size: u64,
    /// Block height where we should gracefully stop the node
    pub halt_height: i64,
}

/// Handle ABCI requests.
#[derive(Clone)]
pub struct App<DB, SS, S, I>
where
    SS: Blockstore + Clone + 'static,
    S: KVStore,
{
    /// Database backing all key-value operations.
    db: Arc<DB>,
    /// State store, backing all the smart contracts.
    ///
    /// Must be kept separate from storage that can be influenced by network operations such as Bitswap;
    /// nodes must be able to run transactions deterministically. By contrast the Bitswap store should
    /// be able to read its own storage area as well as state storage, to serve content from both.
    state_store: Arc<SS>,
    /// Wasm engine cache.
    multi_engine: Arc<MultiEngine>,
    /// Block height where we should gracefully stop the node
    halt_height: i64,
    /// Namespace to store app state.
    namespace: S::Namespace,
    /// Collection of past state parameters.
    ///
    /// We store the state hash for the height of the block where it was committed,
    /// which is different from how Tendermint Core will refer to it in queries,
    /// shifted by one, because Tendermint Core will use the height where the hash
    /// *appeared*, which is in the block *after* the one which was committed.
    ///
    /// The state also contains things like timestamp and the network version,
    /// so that we can retrospectively execute FVM messages at past block heights
    /// in read-only mode.
    state_hist: KVCollection<S, BlockHeight, FvmStateParams>,
    /// Interpreter for block lifecycle events.
    interpreter: Arc<I>,
    /// Environment-like dependencies for the interpreter.
    chain_env: ChainEnv,
    /// Interface to the snapshotter, if enabled.
    snapshots: Option<SnapshotClient>,
    /// State accumulating changes during block execution.
    exec_state: Arc<tokio::sync::Mutex<Option<FvmExecState<SS>>>>,
    /// Projected (partial) state accumulating during transaction checks.
    check_state: CheckStateRef<SS>,
    /// How much history to keep.
    ///
    /// Zero means unlimited.
    state_hist_size: u64,
    /// Caches the validators.
    validators_cache: Arc<tokio::sync::Mutex<Option<ValidatorCache>>>,
}

impl<DB, SS, S, I> App<DB, SS, S, I>
where
    S: KVStore
        + Codec<AppState>
        + Encode<AppStoreKey>
        + Encode<BlockHeight>
        + Codec<FvmStateParams>,
    DB: KVWritable<S> + KVReadable<S> + Clone + 'static,
    SS: Blockstore + Clone + 'static,
{
    pub fn new(
        config: AppConfig<S>,
        db: DB,
        state_store: SS,
        interpreter: I,
        chain_env: ChainEnv,
        snapshots: Option<SnapshotClient>,
    ) -> Result<Self> {
        let app = Self {
            db: Arc::new(db),
            state_store: Arc::new(state_store),
            multi_engine: Arc::new(MultiEngine::new(1)),
            halt_height: config.halt_height,
            namespace: config.app_namespace,
            state_hist: KVCollection::new(config.state_hist_namespace),
            state_hist_size: config.state_hist_size,
            interpreter: Arc::new(interpreter),
            chain_env,
            snapshots,
            exec_state: Arc::new(tokio::sync::Mutex::new(None)),
            check_state: Arc::new(tokio::sync::Mutex::new(None)),
            validators_cache: Arc::new(tokio::sync::Mutex::new(None)),
        };
        app.init_committed_state()?;
        Ok(app)
    }
}

impl<DB, SS, S, I> App<DB, SS, S, I>
where
    S: KVStore
        + Codec<AppState>
        + Encode<AppStoreKey>
        + Encode<BlockHeight>
        + Codec<FvmStateParams>,
    DB: KVWritable<S> + KVReadable<S> + 'static + Clone,
    SS: Blockstore + 'static + Clone,
{
    /// Get an owned clone of the state store.
    fn state_store_clone(&self) -> SS {
        self.state_store.as_ref().clone()
    }

    /// Ensure the store has some initial state.
    fn init_committed_state(&self) -> Result<()> {
        if self.get_committed_state()?.is_none() {
            // We need to be careful never to run a query on this.
            let mut state_tree = empty_state_tree(self.state_store_clone())
                .context("failed to create empty state tree")?;
            let state_root = state_tree.flush()?;
            let state = AppState {
                block_height: 0,
                oldest_state_height: 0,
                state_params: FvmStateParams {
                    timestamp: Timestamp(0),
                    state_root,
                    network_version: NetworkVersion::V0,
                    base_fee: TokenAmount::zero(),
                    circ_supply: TokenAmount::zero(),
                    chain_id: 0,
                    power_scale: 0,
                    app_version: 0,
                    consensus_params: None,
                },
            };
            self.set_committed_state(state)?;
        }
        Ok(())
    }

    /// Get the last committed state, if exists.
    fn get_committed_state(&self) -> Result<Option<AppState>> {
        let tx = self.db.read();
        tx.get(&self.namespace, &AppStoreKey::State)
            .context("get failed")
    }

    /// Get the last committed state; return error if it doesn't exist.
    fn committed_state(&self) -> Result<AppState> {
        match self.get_committed_state()? {
            Some(state) => Ok(state),
            None => Err(anyhow!("app state not found")),
        }
    }

    /// Set the last committed state.
    fn set_committed_state(&self, mut state: AppState) -> Result<()> {
        self.db
            .with_write(|tx| {
                // Insert latest state history point at the `block_height + 1`,
                // to be consistent with how CometBFT queries are supposed to work.
                let state_height = state.state_height();

                self.state_hist
                    .put(tx, &state_height, &state.state_params)?;

                // Prune state history.
                if self.state_hist_size > 0 && state_height >= self.state_hist_size {
                    let prune_height = state_height.saturating_sub(self.state_hist_size);
                    while state.oldest_state_height <= prune_height {
                        self.state_hist.delete(tx, &state.oldest_state_height)?;
                        state.oldest_state_height += 1;
                    }
                }

                // Update the application state.
                tx.put(&self.namespace, &AppStoreKey::State, &state)?;

                Ok(())
            })
            .context("commit failed")
    }

    /// Diff our current consensus params with new values, and return Some with the final params
    /// if they differ (and therefore a consensus layer update is necessary).
    fn maybe_update_app_state(
        &self,
        gas_market: &Reading,
    ) -> Result<Option<TendermintConsensusParams>> {
        let mut state = self.committed_state()?;
        let current = state
            .state_params
            .consensus_params
            .ok_or_else(|| anyhow!("no current consensus params in state"))?;

        if current.block.max_gas == gas_market.block_gas_limit as i64 {
            return Ok(None); // No update necessary.
        }

        // Proceeding with update.
        let mut updated = current;
        updated.block.max_gas = gas_market.block_gas_limit as i64;
        state.state_params.consensus_params = Some(updated.clone());
        self.set_committed_state(state)?;

        Ok(Some(updated))
    }

    /// Put the execution state during block execution. Has to be empty.
    async fn put_exec_state(&self, state: FvmExecState<SS>) {
        let mut guard = self.exec_state.lock().await;
        assert!(guard.is_none(), "exec state not empty");
        *guard = Some(state);
    }

    /// Take the execution state during block execution. Has to be non-empty.
    async fn take_exec_state(&self) -> FvmExecState<SS> {
        let mut guard = self.exec_state.lock().await;
        guard.take().expect("exec state empty")
    }

    /// Take the execution state, update it, put it back, return the output.
    async fn modify_exec_state<T, F, R>(&self, f: F) -> Result<T>
    where
        F: FnOnce((ChainEnv, FvmExecState<SS>)) -> R,
        R: Future<Output = Result<((ChainEnv, FvmExecState<SS>), T)>>,
    {
        let mut guard = self.exec_state.lock().await;
        let state = guard.take().expect("exec state empty");

        // NOTE: There is no need to save the `ChainEnv`; it's shared with other, meant for cloning.
        let ((_env, state), ret) = f((self.chain_env.clone(), state)).await?;

        *guard = Some(state);

        Ok(ret)
    }

    pub fn enable_topdown(&mut self, topdown: TopdownClient) {
        self.chain_env.topdown_client = Toggle::enable(topdown);
    }

    /// Get a read-only view from the current FVM execution state, optionally passing a new BlockContext.
    /// This is useful to perform query commands targeting the latest state. Mutations from transactions
    /// will not be persisted.
    pub fn read_only_view(
        &self,
        height: Option<BlockHeight>,
    ) -> Result<Option<FvmExecState<ReadOnlyBlockstore<Arc<SS>>>>> {
        let app_state = match self.get_committed_state()? {
            Some(app_state) => app_state,
            None => return Ok(None),
        };

        let block_height = height.unwrap_or(app_state.block_height);
        let state_params = app_state.state_params;

        // wait for block production
        if !Self::can_query_state(block_height, &state_params) {
            return Ok(None);
        }

        let exec_state = FvmExecState::new(
            ReadOnlyBlockstore::new(self.state_store.clone()),
            self.multi_engine.as_ref(),
            block_height as ChainEpoch,
            state_params,
        )
        .context("error creating execution state")?;

        Ok(Some(exec_state))
    }

    /// Look up a past state at a particular height Tendermint Core is looking for.
    ///
    /// A height of zero means we are looking for the latest state.
    /// The genesis block state is saved under height 1.
    /// Under height 0 we saved the empty state, which we must not query,
    /// because it doesn't contain any initialized state for the actors.
    ///
    /// Returns the state params and the height of the block which committed it.
    fn state_params_at_height(
        &self,
        height: FvmQueryHeight,
    ) -> Result<(FvmStateParams, BlockHeight)> {
        if let FvmQueryHeight::Height(h) = height {
            let tx = self.db.read();
            let sh = self
                .state_hist
                .get(&tx, &h)
                .context("error looking up history")?;

            if let Some(p) = sh {
                return Ok((p, h));
            }
        }
        let state = self.committed_state()?;
        Ok((state.state_params, state.block_height))
    }

    /// Check whether the state has been initialized by genesis.
    ///
    /// We can't run queries on the initial empty state becase the actors haven't been inserted yet.
    fn can_query_state(height: BlockHeight, params: &FvmStateParams) -> bool {
        // It's really the empty state tree that would be the best indicator.
        !(height == 0 && params.timestamp.0 == 0 && params.network_version == NetworkVersion::V0)
    }

    fn parse_genesis_app_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
        // cometbft serves data in json format, convert from json string
        match serde_json::from_slice(bytes)? {
            serde_json::Value::String(s) => Ok(GenesisAppState::decode_and_decompress(&s)?),
            _ => Err(anyhow!("invalid app state json")),
        }
    }

    /// Replaces the current validators cache with a new one.
    async fn refresh_validators_cache(&self) -> Result<()> {
        // TODO: This should be read only state, but we can't use the read-only view here
        // because it hasn't been committed to state store yet.
        self.modify_exec_state(|mut s| async {
            let mut cache = self.validators_cache.lock().await;
            *cache = Some(ValidatorCache::new_from_state(&mut s.1)?);
            Ok((s, ()))
        })
        .await?;

        Ok(())
    }

    /// Retrieves a validator from the cache, initializing it if necessary.
    async fn get_validator_from_cache(&self, id: &tendermint::account::Id) -> Result<PublicKey> {
        let mut cache = self.validators_cache.lock().await;

        // If cache is not initialized, update it from the state
        if cache.is_none() {
            let mut state = self
                .read_only_view(None)?
                .ok_or_else(|| anyhow!("exec state should be present"))?;

            *cache = Some(ValidatorCache::new_from_state(&mut state)?);
        }

        // Retrieve the validator from the cache
        cache
            .as_ref()
            .context("Validator cache is not available")?
            .get_validator(id)
    }
}

// NOTE: The `Application` interface doesn't allow failures at the moment. The protobuf
// of `Response` actually has an `Exception` type, so in theory we could use that, and
// Tendermint would break up the connection. However, before the response could reach it,
// the `tower-abci` library would throw an exception when it tried to convert a
// `Response::Exception` into a `ConsensusResponse` for example.
#[async_trait]
impl<DB, SS, S, I> Application for App<DB, SS, S, I>
where
    S: KVStore
        + Codec<AppState>
        + Encode<AppStoreKey>
        + Encode<BlockHeight>
        + Codec<FvmStateParams>,
    S::Namespace: Sync + Send,
    DB: KVWritable<S> + KVReadable<S> + Clone + Send + Sync + 'static,
    SS: Blockstore + Clone + Send + Sync + 'static,
    I: ProposalInterpreter<
        State = (ChainEnv, FvmExecState<ReadOnlyBlockstore<Arc<SS>>>),
        Message = Vec<u8>,
    >,
    I: ExecInterpreter<
        State = (ChainEnv, FvmExecState<SS>),
        Message = Vec<u8>,
        BeginOutput = FvmApplyRet,
        DeliverOutput = BytesMessageApplyRes,
        EndOutput = EndBlockOutput,
    >,
    I: CheckInterpreter<
        State = FvmExecState<ReadOnlyBlockstore<SS>>,
        Message = Vec<u8>,
        Output = BytesMessageCheckRes,
    >,
    I: QueryInterpreter<
        State = FvmQueryState<SS>,
        Query = BytesMessageQuery,
        Output = BytesMessageQueryRes,
    >,
{
    /// Provide information about the ABCI application.
    async fn info(&self, _request: request::Info) -> AbciResult<response::Info> {
        let state = self.committed_state()?;

        let height = tendermint::block::Height::try_from(state.block_height)?;

        let info = response::Info {
            data: "fendermint".to_string(),
            version: VERSION.to_owned(),
            app_version: state.state_params.app_version,
            last_block_height: height,
            last_block_app_hash: state.app_hash(),
        };
        Ok(info)
    }

    /// Called once upon genesis.
    async fn init_chain(&self, request: request::InitChain) -> AbciResult<response::InitChain> {
        let genesis_bytes = Self::parse_genesis_app_bytes(&request.app_state_bytes)?;
        let genesis_hash =
            fendermint_vm_message::cid(&genesis_bytes).context("failed to compute genesis CID")?;

        // Make it easy to spot any discrepancies between nodes.
        tracing::info!(genesis_hash = genesis_hash.to_string(), "genesis");

        let (validators, mut state_params) =
            read_genesis_car(genesis_bytes, &self.state_store).await?;

        state_params.consensus_params = Some(request.consensus_params);

        let validators =
            to_validator_updates(validators).context("failed to convert validators")?;

        tracing::info!(state_params = serde_json::to_string(&state_params)?);

        // Let's pretend that the genesis state is that of a fictive block at height 0.
        // The record will be stored under height 1, and the record after the application
        // of the actual block 1 will be at height 2, so they are distinct records.
        // That is despite the fact that block 1 will share the timestamp with genesis,
        // however it seems like it goes through a `prepare_proposal` phase too, which
        // suggests it could have additional transactions affecting the state hash.
        // By keeping them separate we can actually run queries at height=1 as well as height=2,
        // to see the difference between `genesis.json` only and whatever else is in block 1.
        let height: u64 = request.initial_height.into();
        // Note that setting the `initial_height` to 0 doesn't seem to have an effect.
        let height = height - 1;

        let app_state = AppState {
            block_height: height,
            oldest_state_height: height,
            state_params,
        };

        let response = response::InitChain {
            consensus_params: None, // not updating the proposed consensus params
            validators,
            app_hash: app_state.app_hash(),
        };

        tracing::info!(
            height,
            state_root = app_state.state_root().to_string(),
            app_hash = app_state.app_hash().to_string(),
            timestamp = app_state.state_params.timestamp.0,
            chain_id = app_state.state_params.chain_id,
            "init chain"
        );

        self.set_committed_state(app_state)?;

        Ok(response)
    }

    /// Query the application for data at the current or past height.
    #[instrument(skip(self))]
    async fn query(&self, request: request::Query) -> AbciResult<response::Query> {
        let db = self.state_store_clone();
        let height = FvmQueryHeight::from(request.height.value());
        let (state_params, block_height) = self.state_params_at_height(height)?;

        tracing::debug!(
            query_height = request.height.value(),
            block_height,
            state_root = state_params.state_root.to_string(),
            "running query"
        );

        // Don't run queries on the empty state, they won't work.
        if !Self::can_query_state(block_height, &state_params) {
            return Ok(invalid_query(
                AppError::NotInitialized,
                "The app hasn't been initialized yet.".to_owned(),
            ));
        }

        let state = FvmQueryState::new(
            db,
            self.multi_engine.clone(),
            block_height.try_into()?,
            state_params,
            self.check_state.clone(),
            height == FvmQueryHeight::Pending,
        )
        .context("error creating query state")?;

        let qry = (request.path, request.data.to_vec());

        let (_, result) = self
            .interpreter
            .query(state, qry)
            .await
            .context("error running query")?;

        let response = match result {
            Err(e) => invalid_query(AppError::InvalidEncoding, e.description),
            Ok(result) => to_query(result, block_height)?,
        };
        Ok(response)
    }

    /// Check the given transaction before putting it into the local mempool.
    async fn check_tx(&self, request: request::CheckTx) -> AbciResult<response::CheckTx> {
        // Keep the guard through the check, so there can be only one at a time.
        let mut guard = self.check_state.lock().await;

        let state = match guard.take() {
            Some(state) => state,
            None => {
                let db = self.state_store_clone();
                let state = self.committed_state()?;

                // This would create a partial state, but some client scenarios need the full one.
                // FvmCheckState::new(db, state.state_root(), state.chain_id())
                //     .context("error creating check state")?

                FvmExecState::new(
                    ReadOnlyBlockstore::new(db),
                    self.multi_engine.as_ref(),
                    state.block_height.try_into()?,
                    state.state_params,
                )
                .context("error creating check state")?
            }
        };

        let (state, result) = self
            .interpreter
            .check(
                state,
                request.tx.to_vec(),
                request.kind == CheckTxKind::Recheck,
            )
            .await
            .context("error running check")?;

        let mut mpool_received_trace = MpoolReceived::default();

        let response = match result {
            Err(e) => invalid_check_tx(AppError::InvalidEncoding, e.description),
            Ok(result) => match result {
                Err(IllegalMessage) => invalid_check_tx(AppError::IllegalMessage, "".to_owned()),
                Ok(Err(InvalidSignature(d))) => invalid_check_tx(AppError::InvalidSignature, d),
                Ok(Ok(ret)) => {
                    mpool_received_trace.message = Some(Message::from(&ret.message));

                    let priority = state.txn_priority_calculator().priority(&ret.message);
                    to_check_tx(ret, priority)
                }
            },
        };

        // Update the check state.
        *guard = Some(state);

        mpool_received_trace.accept = response.code.is_ok();
        if !mpool_received_trace.accept {
            mpool_received_trace.reason = Some(format!("{:?} - {}", response.code, response.info));
        }

        emit(mpool_received_trace);
        Ok(response)
    }

    /// Amend which transactions to put into the next block proposal.
    async fn prepare_proposal(
        &self,
        request: request::PrepareProposal,
    ) -> AbciResult<response::PrepareProposal> {
        tracing::debug!(
            height = request.height.value(),
            time = request.time.to_string(),
            "prepare proposal"
        );
        let txs = request.txs.into_iter().map(|tx| tx.to_vec()).collect();

        let state = self
            .read_only_view(Some(request.height.value()))?
            .ok_or_else(|| anyhow!("exec state should be present"))?;

        let txs = self
            .interpreter
            .prepare((self.chain_env.clone(), state), txs)
            .await
            .context("failed to prepare proposal")?;

        let txs = txs.into_iter().map(bytes::Bytes::from).collect();
        // TODO: This seems leaky placed here, should be done in `interpreter`, that's where it's ipc
        // TODO: aware, here might have filtered more important messages.
        let (txs, size) = take_until_max_size(txs, request.max_tx_bytes.try_into().unwrap());

        emit(BlockProposalSent {
            validator: &request.proposer_address,
            height: request.height.value(),
            tx_count: txs.len(),
            size,
        });

        Ok(response::PrepareProposal { txs })
    }

    /// Inspect a proposal and decide whether to vote on it.
    async fn process_proposal(
        &self,
        request: request::ProcessProposal,
    ) -> AbciResult<response::ProcessProposal> {
        tracing::debug!(
            height = request.height.value(),
            time = request.time.to_string(),
            "process proposal"
        );
        let txs: Vec<_> = request.txs.into_iter().map(|tx| tx.to_vec()).collect();
        let size_txs = txs.iter().map(|tx| tx.len()).sum::<usize>();
        let num_txs = txs.len();

        let state = self
            .read_only_view(Some(request.height.value()))?
            .ok_or_else(|| anyhow!("exec state should be present"))?;

        let accept = self
            .interpreter
            .process((self.chain_env.clone(), state), txs)
            .await
            .context("failed to process proposal")?;

        emit(BlockProposalReceived {
            height: request.height.value(),
            hash: HexEncodableBlockHash(request.hash.into()),
            size: size_txs,
            tx_count: num_txs,
            validator: &request.proposer_address,
        });

        emit(BlockProposalEvaluated {
            height: request.height.value(),
            hash: HexEncodableBlockHash(request.hash.into()),
            size: size_txs,
            tx_count: num_txs,
            validator: &request.proposer_address,
            accept,
            reason: None,
        });

        if accept {
            Ok(response::ProcessProposal::Accept)
        } else {
            Ok(response::ProcessProposal::Reject)
        }
    }

    /// Signals the beginning of a new block, prior to any `DeliverTx` calls.
    async fn begin_block(&self, request: request::BeginBlock) -> AbciResult<response::BeginBlock> {
        let block_height = request.header.height.into();
        let block_hash = match request.hash {
            tendermint::Hash::Sha256(h) => h,
            tendermint::Hash::None => return Err(anyhow!("empty block hash").into()),
        };

        if self.halt_height != 0 && block_height == self.halt_height {
            tracing::info!(
                height = block_height,
                "Stopping node due to reaching halt height"
            );
            std::process::exit(AppExitCode::Halt as i32);
        }

        let db = self.state_store_clone();
        let state = self.committed_state()?;
        let mut state_params = state.state_params.clone();

        tracing::debug!(
            height = block_height,
            timestamp = request.header.time.unix_timestamp(),
            app_hash = request.header.app_hash.to_string(),
            //app_state_hash = to_app_hash(&state_params).to_string(), // should be the same as `app_hash`
            "begin block"
        );

        state_params.timestamp = to_timestamp(request.header.time);

        let validator = self
            .get_validator_from_cache(&request.header.proposer_address)
            .await?;

        let state = FvmExecState::new(db, self.multi_engine.as_ref(), block_height, state_params)
            .context("error creating new state")?
            .with_block_hash(block_hash)
            .with_block_producer(validator);

        tracing::debug!("initialized exec state");

        self.put_exec_state(state).await;

        let ret = self
            .modify_exec_state(|s| self.interpreter.begin(s))
            .await
            .context("begin failed")?;

        Ok(to_begin_block(ret))
    }

    /// Apply a transaction to the application's state.
    async fn deliver_tx(&self, request: request::DeliverTx) -> AbciResult<response::DeliverTx> {
        let msg = request.tx.to_vec();
        let (result, block_hash) = self
            .modify_exec_state(|s| async {
                let ((env, state), res) = self.interpreter.deliver(s, msg).await?;
                let block_hash = state.block_hash();
                Ok(((env, state), (res, block_hash)))
            })
            .await
            .context("deliver failed")?;

        let response = match result {
            Err(e) => invalid_deliver_tx(AppError::InvalidEncoding, e.description),
            Ok(ret) => match ret {
                ChainMessageApplyRet::Signed(Err(InvalidSignature(d))) => {
                    invalid_deliver_tx(AppError::InvalidSignature, d)
                }
                ChainMessageApplyRet::Signed(Ok(ret)) => {
                    to_deliver_tx(ret.fvm, ret.domain_hash, block_hash)
                }
                ChainMessageApplyRet::Ipc(ret) => to_deliver_tx(ret, None, block_hash),
            },
        };

        if response.code != 0.into() {
            tracing::info!(
                "deliver_tx failed: {:?} - {:?}",
                response.code,
                response.info
            );
        }

        Ok(response)
    }

    /// Signals the end of a block.
    async fn end_block(&self, request: request::EndBlock) -> AbciResult<response::EndBlock> {
        tracing::debug!(height = request.height, "end block");

        // End the interpreter for this block.
        let EndBlockOutput {
            power_updates,
            gas_market,
            events,
        } = self
            .modify_exec_state(|s| self.interpreter.end(s))
            .await
            .context("end failed")?;

        // Convert the incoming power updates to Tendermint validator updates.
        let validator_updates =
            to_validator_updates(power_updates.0).context("failed to convert validator updates")?;

        // Replace the validator cache if the validator set has changed.
        if !validator_updates.is_empty() {
            self.refresh_validators_cache().await?;
        }

        // Maybe update the app state with the new block gas limit.
        let consensus_param_updates = self
            .maybe_update_app_state(&gas_market)
            .context("failed to update block gas limit")?;

        let ret = response::EndBlock {
            validator_updates,
            consensus_param_updates,
            events: events
                .into_iter()
                .flat_map(|(stamped, emitters)| to_events("event", stamped, emitters))
                .collect::<Vec<_>>(),
        };

        Ok(ret)
    }

    /// Commit the current state at the current height.
    async fn commit(&self) -> AbciResult<response::Commit> {
        let exec_state = self.take_exec_state().await;

        // Commit the execution state to the datastore.
        let mut state = self.committed_state()?;
        state.block_height = exec_state.block_height().try_into()?;
        state.state_params.timestamp = exec_state.timestamp();

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

        state.state_params.state_root = state_root;
        state.state_params.app_version = app_version;
        state.state_params.base_fee = base_fee;
        state.state_params.circ_supply = circ_supply;
        state.state_params.power_scale = power_scale;

        let app_hash = state.app_hash();
        let block_height = state.block_height;

        // Tell CometBFT how much of the block history it can forget.
        let retain_height = if self.state_hist_size == 0 {
            Default::default()
        } else {
            block_height.saturating_sub(self.state_hist_size)
        };

        tracing::debug!(
            block_height,
            state_root = state_root.to_string(),
            app_hash = app_hash.to_string(),
            timestamp = state.state_params.timestamp.0,
            "commit state"
        );

        // TODO: We can defer committing changes the resolution pool to this point.
        // For example if a checkpoint is successfully executed, that's when we want to remove
        // that checkpoint from the pool, and not propose it to other validators again.
        // However, once Tendermint starts delivering the transactions, the commit will surely
        // follow at the end, so we can also remove these checkpoints from memory at the time
        // the transaction is delivered, rather than when the whole thing is committed.
        // It is only important to the persistent database changes as an atomic step in the
        // commit in case the block execution fails somewhere in the middle for uknown reasons.
        // But if that happened, we will have to restart the application again anyway, and
        // repopulate the in-memory checkpoints based on the last committed ledger.
        // So, while the pool is theoretically part of the evolving state and we can pass
        // it in and out, unless we want to defer commit to here (which the interpreters aren't
        // notified about), we could add it to the `ChainMessageInterpreter` as a constructor argument,
        // a sort of "ambient state", and not worry about in in the `App` at all.

        // Notify the snapshotter. It wasn't clear whether this should be done in `commit` or `begin_block`,
        // that is, whether the _height_ of the snapshot should be `block_height` or `block_height+1`.
        // When CometBFT calls `offer_snapshot` it sends an `app_hash` in it that we compare to the CID
        // of the `state_params`. Based on end-to-end testing it looks like it gives the `app_hash` from
        // the *next* block, so we have to do it here.
        // For example:
        // a) Notify in `begin_block`: say we are at committing block 899, then we notify in `begin_block`
        //    that block 900 has this state (so we use `block_height+1` in notification);
        //    CometBFT is going to offer it with the `app_hash` of block 901, which won't match, because
        //    by then the timestamp will be different in the state params after committing block 900.
        // b) Notify in `commit`: say we are committing block 900 and notify immediately that it has this state
        //    (even though this state will only be available to query from the next height);
        //    CometBFT is going to offer it with the `app_hash` of 901, but in this case that's good, because
        //    that hash reflects the changes made by block 900, which this state param is the result of.
        if let Some(ref snapshots) = self.snapshots {
            atomically(|| snapshots.notify(block_height, state.state_params.clone())).await;
        }

        // Commit app state to the datastore.
        self.set_committed_state(state)?;

        // Reset check state.
        let mut guard = self.check_state.lock().await;
        *guard = None;

        emit(BlockCommitted {
            height: block_height,
            app_hash: HexEncodableBlockHash(app_hash.clone().into()),
        });

        Ok(response::Commit {
            data: app_hash.into(),
            retain_height: retain_height.try_into().expect("height is valid"),
        })
    }

    /// List the snapshots available on this node to be served to remote peers.
    async fn list_snapshots(&self) -> AbciResult<response::ListSnapshots> {
        if let Some(ref client) = self.snapshots {
            let snapshots = atomically(|| client.list_snapshots()).await;
            tracing::info!(snapshot_count = snapshots.len(), "listing snaphots");
            Ok(to_snapshots(snapshots)?)
        } else {
            tracing::info!("listing snaphots disabled");
            Ok(Default::default())
        }
    }

    /// Load a particular snapshot chunk a remote peer is asking for.
    async fn load_snapshot_chunk(
        &self,
        request: request::LoadSnapshotChunk,
    ) -> AbciResult<response::LoadSnapshotChunk> {
        if let Some(ref client) = self.snapshots {
            if let Some(snapshot) =
                atomically(|| client.access_snapshot(request.height.value(), request.format)).await
            {
                match snapshot.load_chunk(request.chunk) {
                    Ok(chunk) => {
                        return Ok(response::LoadSnapshotChunk {
                            chunk: chunk.into(),
                        });
                    }
                    Err(e) => {
                        tracing::warn!("failed to load chunk: {e:#}");
                    }
                }
            }
        }
        Ok(Default::default())
    }

    /// Decide whether to start downloading a snapshot from peers.
    ///
    /// This method is also called when a download is aborted and a new snapshot is offered,
    /// so potentially we have to clean up previous resources and start a new one.
    async fn offer_snapshot(
        &self,
        request: request::OfferSnapshot,
    ) -> AbciResult<response::OfferSnapshot> {
        if let Some(ref client) = self.snapshots {
            tracing::info!(
                height = request.snapshot.height.value(),
                "received snapshot offer"
            );

            match from_snapshot(request).context("failed to parse snapshot") {
                Ok(manifest) => {
                    tracing::info!(?manifest, "received snapshot offer");
                    // We can look at the version but currently there's only one.
                    match atomically_or_err(|| client.offer_snapshot(manifest.clone())).await {
                        Ok(path) => {
                            tracing::info!(
                                download_dir = path.to_string_lossy().to_string(),
                                height = manifest.block_height,
                                size = manifest.size,
                                chunks = manifest.chunks,
                                "downloading snapshot"
                            );
                            return Ok(response::OfferSnapshot::Accept);
                        }
                        Err(SnapshotError::IncompatibleVersion(version)) => {
                            tracing::warn!(version, "rejecting offered snapshot version");
                            return Ok(response::OfferSnapshot::RejectFormat);
                        }
                        Err(e) => {
                            tracing::error!(error = ?e, "failed to start snapshot download");
                            return Ok(response::OfferSnapshot::Abort);
                        }
                    };
                }
                Err(e) => {
                    tracing::warn!("failed to parse snapshot offer: {e:#}");
                    return Ok(response::OfferSnapshot::Reject);
                }
            }
        }
        Ok(Default::default())
    }

    /// Apply the given snapshot chunk to the application's state.
    async fn apply_snapshot_chunk(
        &self,
        request: request::ApplySnapshotChunk,
    ) -> AbciResult<response::ApplySnapshotChunk> {
        tracing::debug!(chunk = request.index, "received snapshot chunk");
        let default = response::ApplySnapshotChunk::default();

        if let Some(ref client) = self.snapshots {
            match atomically_or_err(|| {
                client.save_chunk(request.index, request.chunk.clone().into())
            })
            .await
            {
                Ok(snapshot) => {
                    if let Some(snapshot) = snapshot {
                        tracing::info!(
                            download_dir = snapshot.snapshot_dir.to_string_lossy().to_string(),
                            height = snapshot.manifest.block_height,
                            "received all snapshot chunks",
                        );

                        // Ideally we would import into some isolated store then validate,
                        // but for now let's trust that all is well.
                        if let Err(e) = snapshot.import(self.state_store_clone(), true).await {
                            tracing::error!(error =? e, "failed to import snapshot");
                            return Ok(response::ApplySnapshotChunk {
                                result: response::ApplySnapshotChunkResult::RejectSnapshot,
                                ..default
                            });
                        }

                        tracing::info!(
                            height = snapshot.manifest.block_height,
                            "imported snapshot"
                        );

                        // Now insert the new state into the history.
                        let mut state = self.committed_state()?;

                        // The height reflects that it was produced in `commit`.
                        state.block_height = snapshot.manifest.block_height;
                        state.state_params = snapshot.manifest.state_params;
                        self.set_committed_state(state)?;

                        // TODO: We can remove the `current_download` from the STM
                        // state here which would cause it to get dropped from /tmp,
                        // but for now let's keep it just in case we need to investigate
                        // some problem.

                        // We could also move the files into our own snapshot directory
                        // so that we can offer it to others, but again let's hold on
                        // until we have done more robust validation.
                    }
                    return Ok(response::ApplySnapshotChunk {
                        result: response::ApplySnapshotChunkResult::Accept,
                        ..default
                    });
                }
                Err(SnapshotError::UnexpectedChunk(expected, got)) => {
                    tracing::warn!(got, expected, "unexpected snapshot chunk index");
                    return Ok(response::ApplySnapshotChunk {
                        result: response::ApplySnapshotChunkResult::Retry,
                        refetch_chunks: vec![expected],
                        ..default
                    });
                }
                Err(SnapshotError::WrongChecksum(expected, got)) => {
                    tracing::warn!(?got, ?expected, "wrong snapshot checksum");
                    // We could retry this snapshot, or try another one.
                    // If we retry, we have to tell which chunks to refetch.
                    return Ok(response::ApplySnapshotChunk {
                        result: response::ApplySnapshotChunkResult::RejectSnapshot,
                        ..default
                    });
                }
                Err(e) => {
                    tracing::error!(
                        chunk = request.index,
                        sender = request.sender,
                        error = ?e,
                        "failed to process snapshot chunk"
                    );
                }
            }
        }

        Ok(default)
    }
}

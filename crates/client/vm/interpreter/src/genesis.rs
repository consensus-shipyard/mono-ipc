// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::collections::{BTreeSet, HashMap};
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{anyhow, Context};
use base64::Engine;
use cid::Cid;
use ethers::abi::Tokenize;
use ethers::core::types as et;
use fendermint_actor_eam::PermissionModeParams;
use fendermint_eth_hardhat::{ContractSourceAndName, Hardhat, FQN};
use fendermint_vm_actor_interface::diamond::{EthContract, EthContractMap};
use fendermint_vm_actor_interface::eam::EthAddress;
use fendermint_vm_actor_interface::ipc::IPC_CONTRACTS;
use fendermint_vm_actor_interface::{
    account, activity, burntfunds, chainmetadata, cron, eam, gas_market, init, ipc, reward, system,
    EMPTY_ARR,
};
use fendermint_vm_core::Timestamp;
use fendermint_vm_genesis::{ActorMeta, Collateral, Genesis, Power, PowerScale, Validator};
use futures_util::io::Cursor;
use fvm::engine::MultiEngine;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_car::{load_car, CarHeader};
use fvm_ipld_encoding::CborStore;
use fvm_shared::chainid::ChainID;
use fvm_shared::econ::TokenAmount;
use fvm_shared::version::NetworkVersion;
use ipc_actors_abis::i_diamond::FacetCut;
use num_traits::Zero;

use crate::fvm::state::snapshot::{derive_cid, StateTreeStreamer};
use crate::fvm::state::{FvmGenesisState, FvmStateParams};
use crate::fvm::store::memory::MemoryBlockstore;
use fendermint_vm_genesis::ipc::{GatewayParams, IpcParams};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tokio_stream::StreamExt;
use tokio_util::compat::TokioAsyncWriteCompatExt;

/// The sealed genesis state metadata
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct GenesisMetadata {
    pub state_params: FvmStateParams,
    pub validators: Vec<Validator<Power>>,
}

impl GenesisMetadata {
    fn new(state_root: Cid, out: GenesisOutput) -> GenesisMetadata {
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

        GenesisMetadata {
            state_params,
            validators: out.validators,
        }
    }
}

/// Genesis app state wrapper for cometbft
#[repr(u8)]
pub enum GenesisAppState {
    V1(Vec<u8>) = 1,
}

impl GenesisAppState {
    pub fn v1(bytes: Vec<u8>) -> Self {
        Self::V1(bytes)
    }

    pub fn compress_and_encode(&self) -> anyhow::Result<String> {
        let bytes = match self {
            GenesisAppState::V1(ref bytes) => {
                let mut buf = {
                    let len = snap::raw::max_compress_len(bytes.len()) + 1; // +1 for the version discriminator
                    Vec::with_capacity(len)
                };

                // Write version discriminator uncompressed.
                buf.push(1);

                // Snappy compress the data.
                let mut wtr = snap::write::FrameEncoder::new(buf);
                wtr.write_all(bytes)?;
                wtr.into_inner()?
            }
        };

        Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    pub fn decode_and_decompress(raw: &str) -> anyhow::Result<Vec<u8>> {
        let bytes = base64::engine::general_purpose::STANDARD.decode(raw)?;
        if bytes.is_empty() {
            return Err(anyhow!("empty bytes for genesis app state"));
        }

        // Strip the version discriminator.
        let version = bytes[0];

        match version {
            1 => {
                let data = &bytes.as_slice()[1..];
                let len = snap::raw::decompress_len(data)
                    .context("failed to calculate length of decompressed app state")?;
                let mut buf = Vec::with_capacity(len);
                snap::read::FrameDecoder::new(data).read_to_end(&mut buf)?;
                Ok(buf)
            }
            _ => Err(anyhow!("unsupported schema version")),
        }
    }
}

pub async fn read_genesis_car<DB: Blockstore + 'static + Send + Sync>(
    bytes: Vec<u8>,
    store: &DB,
) -> anyhow::Result<(Vec<Validator<Power>>, FvmStateParams)> {
    let roots = load_car(store, Cursor::new(&bytes)).await?;

    let metadata_cid = roots
        .first()
        .ok_or_else(|| anyhow!("invalid genesis car, should have at least 1 root cid"))?;

    let metadata = store
        .get_cbor::<GenesisMetadata>(metadata_cid)?
        .ok_or_else(|| anyhow!("invalid genesis car, metadata not found"))?;

    Ok((metadata.validators, metadata.state_params))
}

/// The output of genesis creation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisOutput {
    pub chain_id: ChainID,
    pub timestamp: Timestamp,
    pub network_version: NetworkVersion,
    pub base_fee: TokenAmount,
    pub power_scale: PowerScale,
    pub circ_supply: TokenAmount,
    pub validators: Vec<Validator<Power>>,
}

pub struct GenesisBuilder {
    /// Hardhat like util to deploy ipc contracts
    hardhat: Hardhat,
    /// The built in actors bundle path
    builtin_actors_path: PathBuf,
    /// The custom actors bundle path
    custom_actors_path: PathBuf,

    /// Genesis params
    genesis_params: Genesis,
}

impl GenesisBuilder {
    pub fn new(
        builtin_actors_path: PathBuf,
        custom_actors_path: PathBuf,
        artifacts_path: PathBuf,
        genesis_params: Genesis,
    ) -> Self {
        Self {
            hardhat: Hardhat::new(artifacts_path),
            builtin_actors_path,
            custom_actors_path,
            genesis_params,
        }
    }

    /// Initialize actor states from the Genesis parameters and write the sealed genesis state to
    /// a CAR file specified by `out_path`
    pub async fn write_to(&self, out_path: PathBuf) -> anyhow::Result<()> {
        let mut state = self.init_state().await?;
        let genesis_state = self.populate_state(&mut state, self.genesis_params.clone())?;
        let (state_root, store) = state.finalize()?;
        self.write_car(state_root, genesis_state, out_path, store)
            .await
    }

    async fn write_car(
        &self,
        state_root: Cid,
        genesis_state: GenesisOutput,
        out_path: PathBuf,
        store: MemoryBlockstore,
    ) -> anyhow::Result<()> {
        let file = tokio::fs::File::create(&out_path).await?;

        tracing::info!(state_root = state_root.to_string(), "state root");

        let metadata = GenesisMetadata::new(state_root, genesis_state);

        let streamer = StateTreeStreamer::new(state_root, store);
        let (metadata_cid, metadata_bytes) = derive_cid(&metadata)?;
        tracing::info!("generated genesis metadata header cid: {}", metadata_cid);

        // create the target car header with the metadata cid as the only root
        let car = CarHeader::new(vec![metadata_cid], 1);

        // create the stream to stream all the data into the car file
        let mut streamer = tokio_stream::iter(vec![(metadata_cid, metadata_bytes)]).merge(streamer);

        let mut write = file.compat_write();
        car.write_stream_async(&mut Pin::new(&mut write), &mut streamer)
            .await?;

        tracing::info!("written sealed genesis state to file");

        Ok(())
    }

    async fn init_state(&self) -> anyhow::Result<FvmGenesisState<MemoryBlockstore>> {
        let bundle = std::fs::read(&self.builtin_actors_path).with_context(|| {
            format!(
                "failed to read builtin actors bundle: {}",
                self.builtin_actors_path.to_string_lossy()
            )
        })?;

        let custom_actors_bundle = std::fs::read(&self.custom_actors_path).with_context(|| {
            format!(
                "failed to read custom actors bundle: {}",
                self.custom_actors_path.to_string_lossy()
            )
        })?;

        let store = MemoryBlockstore::new();

        FvmGenesisState::new(
            store,
            Arc::new(MultiEngine::new(1)),
            &bundle,
            &custom_actors_bundle,
        )
        .await
        .context("failed to create genesis state")
    }

    fn populate_state(
        &self,
        state: &mut FvmGenesisState<MemoryBlockstore>,
        genesis: Genesis,
    ) -> anyhow::Result<GenesisOutput> {
        // NOTE: We could consider adding the chain ID to the interpreter
        //       and rejecting genesis if it doesn't match the expectation,
        //       but the Tendermint genesis file also has this field, and
        //       presumably Tendermint checks that its peers have the same.
        let chain_id = genesis.chain_id()?;

        // Convert validators to CometBFT power scale.
        let validators = genesis
            .validators
            .iter()
            .cloned()
            .map(|vc| vc.map_power(|c| c.into_power(genesis.power_scale)))
            .collect();

        // Currently we just pass them back as they are, but later we should
        // store them in the IPC actors; or in case of a snapshot restore them
        // from the state.
        let out = GenesisOutput {
            chain_id,
            timestamp: genesis.timestamp,
            network_version: genesis.network_version,
            circ_supply: circ_supply(&genesis),
            base_fee: genesis.base_fee,
            power_scale: genesis.power_scale,
            validators,
        };

        // STAGE 0: Declare the built-in EVM contracts we'll have to deploy.
        // ipc_entrypoints contains the external user facing contracts
        // all_ipc_contracts contains ipc_entrypoints + util contracts
        let (all_ipc_contracts, ipc_entrypoints) = self.collect_contracts()?;

        // STAGE 1: First we initialize native built-in actors.
        // System actor
        state
            .create_builtin_actor(
                system::SYSTEM_ACTOR_CODE_ID,
                system::SYSTEM_ACTOR_ID,
                &system::State {
                    builtin_actors: state.manifest_data_cid,
                },
                TokenAmount::zero(),
                None,
            )
            .context("failed to create system actor")?;

        // Init actor
        let (init_state, addr_to_id) = init::State::new(
            state.store(),
            genesis.chain_name.clone(),
            &genesis.accounts,
            &ipc_entrypoints
                .values()
                .map(|c| c.actor_id)
                .collect::<BTreeSet<_>>(),
            all_ipc_contracts.len() as u64,
        )
        .context("failed to create init state")?;

        state
            .create_builtin_actor(
                init::INIT_ACTOR_CODE_ID,
                init::INIT_ACTOR_ID,
                &init_state,
                TokenAmount::zero(),
                None,
            )
            .context("failed to create init actor")?;

        // Cron actor
        state
            .create_builtin_actor(
                cron::CRON_ACTOR_CODE_ID,
                cron::CRON_ACTOR_ID,
                &cron::State {
                    entries: vec![], // TODO: Maybe with the IPC.
                },
                TokenAmount::zero(),
                None,
            )
            .context("failed to create cron actor")?;

        // Ethereum Account Manager (EAM) actor
        state
            .create_builtin_actor(
                eam::EAM_ACTOR_CODE_ID,
                eam::EAM_ACTOR_ID,
                &EMPTY_ARR,
                TokenAmount::zero(),
                None,
            )
            .context("failed to create EAM actor")?;

        // Burnt funds actor (it's just an account).
        state
            .create_builtin_actor(
                account::ACCOUNT_ACTOR_CODE_ID,
                burntfunds::BURNT_FUNDS_ACTOR_ID,
                &account::State {
                    address: burntfunds::BURNT_FUNDS_ACTOR_ADDR,
                },
                TokenAmount::zero(),
                None,
            )
            .context("failed to create burnt funds actor")?;

        // A placeholder for the reward actor, beause I don't think
        // using the one in the builtin actors library would be appropriate.
        // This effectively burns the miner rewards. Better than panicking.
        state
            .create_builtin_actor(
                account::ACCOUNT_ACTOR_CODE_ID,
                reward::REWARD_ACTOR_ID,
                &account::State {
                    address: reward::REWARD_ACTOR_ADDR,
                },
                TokenAmount::zero(),
                None,
            )
            .context("failed to create reward actor")?;

        // STAGE 1b: Then we initialize the in-repo custom actors.

        // Initialize the chain metadata actor which handles saving metadata about the chain
        // (e.g. block hashes) which we can query.
        let chainmetadata_state = fendermint_actor_chainmetadata::State::new(
            &state.store(),
            fendermint_actor_chainmetadata::DEFAULT_LOOKBACK_LEN,
        )?;
        state
            .create_custom_actor(
                fendermint_actor_chainmetadata::CHAINMETADATA_ACTOR_NAME,
                chainmetadata::CHAINMETADATA_ACTOR_ID,
                &chainmetadata_state,
                TokenAmount::zero(),
                None,
            )
            .context("failed to create chainmetadata actor")?;

        let eam_state = fendermint_actor_eam::State::new(
            state.store(),
            PermissionModeParams::from(genesis.eam_permission_mode),
        )?;
        state
            .replace_builtin_actor(
                eam::EAM_ACTOR_NAME,
                eam::EAM_ACTOR_ID,
                fendermint_actor_eam::IPC_EAM_ACTOR_NAME,
                &eam_state,
                TokenAmount::zero(),
                None,
            )
            .context("failed to replace built in eam actor")?;

        // Currently hardcoded for now, once genesis V2 is implemented, should be taken
        // from genesis parameters.
        //
        // Default initial base fee equals minimum base fee in Filecoin.
        let initial_base_fee = TokenAmount::from_atto(100);
        // We construct the actor state here for simplicity, but for better decoupling we should
        // be invoking the constructor instead.
        let gas_market_state = fendermint_actor_gas_market_eip1559::State {
            base_fee: initial_base_fee,
            // If you need to customize the gas market constants, you can do so here.
            constants: fendermint_actor_gas_market_eip1559::Constants::default(),
        };
        state
            .create_custom_actor(
                fendermint_actor_gas_market_eip1559::ACTOR_NAME,
                gas_market::GAS_MARKET_ACTOR_ID,
                &gas_market_state,
                TokenAmount::zero(),
                None,
            )
            .context("failed to create default eip1559 gas market actor")?;

        let tracker_state = fendermint_actor_activity_tracker::State::new(state.store())?;
        state
            .create_custom_actor(
                fendermint_actor_activity_tracker::IPC_ACTIVITY_TRACKER_ACTOR_NAME,
                activity::ACTIVITY_TRACKER_ACTOR_ID,
                &tracker_state,
                TokenAmount::zero(),
                None,
            )
            .context("failed to create activity tracker actor")?;

        // STAGE 2: Create non-builtin accounts which do not have a fixed ID.

        // The next ID is going to be _after_ the accounts, which have already been assigned an ID by the `Init` actor.
        // The reason we aren't using the `init_state.next_id` is because that already accounted for the multisig accounts.
        let mut next_id = init::FIRST_NON_SINGLETON_ADDR + addr_to_id.len() as u64;

        for a in genesis.accounts {
            let balance = a.balance;
            match a.meta {
                ActorMeta::Account(acct) => {
                    state
                        .create_account_actor(acct, balance, &addr_to_id)
                        .context("failed to create account actor")?;
                }
                ActorMeta::Multisig(ms) => {
                    state
                        .create_multisig_actor(ms, balance, &addr_to_id, next_id)
                        .context("failed to create multisig actor")?;
                    next_id += 1;
                }
            }
        }

        // STAGE 3: Initialize the FVM and create built-in FEVM actors.

        state
            .init_exec_state(
                out.timestamp,
                out.network_version,
                out.base_fee.clone(),
                out.circ_supply.clone(),
                out.chain_id.into(),
                out.power_scale,
            )
            .context("failed to init exec state")?;

        // STAGE 4: Deploy the IPC system contracts.

        let config = DeployConfig {
            ipc_params: genesis.ipc.as_ref(),
            chain_id: out.chain_id,
            hardhat: &self.hardhat,
        };

        deploy_contracts(
            all_ipc_contracts,
            &ipc_entrypoints,
            genesis.validators,
            next_id,
            state,
            config,
        )?;

        Ok(out)
    }

    fn collect_contracts(&self) -> anyhow::Result<(Vec<ContractSourceAndName>, EthContractMap)> {
        let mut all_contracts = Vec::new();
        let mut top_level_contracts = EthContractMap::default();

        top_level_contracts.extend(IPC_CONTRACTS.clone());

        all_contracts.extend(top_level_contracts.keys());
        all_contracts.extend(
            top_level_contracts
                .values()
                .flat_map(|c| c.facets.iter().map(|f| f.name)),
        );
        // Collect dependencies of the main IPC actors.
        let mut eth_libs = self
            .hardhat
            .dependencies(
                &all_contracts
                    .iter()
                    .map(|n| (contract_src(n), *n))
                    .collect::<Vec<_>>(),
            )
            .context("failed to collect EVM contract dependencies")?;

        // Only keep library dependencies, not contracts with constructors.
        eth_libs.retain(|(_, d)| !top_level_contracts.contains_key(d.as_str()));
        Ok((eth_libs, top_level_contracts))
    }
}

// Configuration for deploying IPC contracts.
// This is to circumvent the arguments limit of the deploy_contracts function.
struct DeployConfig<'a> {
    ipc_params: Option<&'a IpcParams>,
    chain_id: ChainID,
    hardhat: &'a Hardhat,
}

fn deploy_contracts(
    ipc_contracts: Vec<ContractSourceAndName>,
    top_level_contracts: &EthContractMap,
    validators: Vec<Validator<Collateral>>,
    mut next_id: u64,
    state: &mut FvmGenesisState<MemoryBlockstore>,
    config: DeployConfig,
) -> anyhow::Result<()> {
    let mut deployer =
        ContractDeployer::<MemoryBlockstore>::new(config.hardhat, top_level_contracts);

    // Deploy Ethereum libraries.
    for (lib_src, lib_name) in ipc_contracts {
        deployer.deploy_library(state, &mut next_id, lib_src, &lib_name)?;
    }

    // IPC Gateway actor.
    let gateway_addr = {
        use ipc::gateway::ConstructorParameters;
        use ipc_api::subnet_id::SubnetID;

        let ipc_params = if let Some(p) = config.ipc_params {
            p.gateway.clone()
        } else {
            GatewayParams::new(SubnetID::new(config.chain_id.into(), vec![]))
        };

        let params = ConstructorParameters::new(ipc_params, validators)
            .context("failed to create gateway constructor")?;

        let facets = deployer
            .facets(ipc::gateway::CONTRACT_NAME)
            .context("failed to collect gateway facets")?;

        deployer.deploy_contract(state, ipc::gateway::CONTRACT_NAME, (facets, params))?
    };

    // IPC SubnetRegistry actor.
    {
        use ipc::registry::ConstructorParameters;

        let mut facets = deployer
            .facets(ipc::registry::CONTRACT_NAME)
            .context("failed to collect registry facets")?;

        let getter_facet = facets.remove(0);
        let manager_facet = facets.remove(0);
        let rewarder_facet = facets.remove(0);
        let checkpointer_facet = facets.remove(0);
        let pauser_facet = facets.remove(0);
        let diamond_loupe_facet = facets.remove(0);
        let diamond_cut_facet = facets.remove(0);
        let ownership_facet = facets.remove(0);
        let activity_facet = facets.remove(0);

        debug_assert_eq!(facets.len(), 2, "SubnetRegistry has 2 facets of its own");

        let params = ConstructorParameters {
            gateway: gateway_addr,
            getter_facet: getter_facet.facet_address,
            manager_facet: manager_facet.facet_address,
            rewarder_facet: rewarder_facet.facet_address,
            pauser_facet: pauser_facet.facet_address,
            checkpointer_facet: checkpointer_facet.facet_address,
            diamond_cut_facet: diamond_cut_facet.facet_address,
            diamond_loupe_facet: diamond_loupe_facet.facet_address,
            ownership_facet: ownership_facet.facet_address,
            activity_facet: activity_facet.facet_address,
            subnet_getter_selectors: getter_facet.function_selectors,
            subnet_manager_selectors: manager_facet.function_selectors,
            subnet_rewarder_selectors: rewarder_facet.function_selectors,
            subnet_checkpointer_selectors: checkpointer_facet.function_selectors,
            subnet_pauser_selectors: pauser_facet.function_selectors,
            subnet_actor_diamond_cut_selectors: diamond_cut_facet.function_selectors,
            subnet_actor_diamond_loupe_selectors: diamond_loupe_facet.function_selectors,
            subnet_actor_ownership_selectors: ownership_facet.function_selectors,
            subnet_actor_activity_selectors: activity_facet.function_selectors,
            creation_privileges: 0,
        };

        deployer.deploy_contract(state, ipc::registry::CONTRACT_NAME, (facets, params))?;
    }

    Ok(())
}

fn contract_src(name: &str) -> PathBuf {
    PathBuf::from(format!("{name}.sol"))
}

struct ContractDeployer<'a, DB> {
    hardhat: &'a Hardhat,
    top_contracts: &'a EthContractMap,
    // Assign dynamic ID addresses to libraries, but use fixed addresses for the top level contracts.
    lib_addrs: HashMap<FQN, et::Address>,
    phantom_db: PhantomData<DB>,
}

impl<'a, DB> ContractDeployer<'a, DB>
where
    DB: Blockstore + 'static + Clone,
{
    pub fn new(hardhat: &'a Hardhat, top_contracts: &'a EthContractMap) -> Self {
        Self {
            hardhat,
            top_contracts,
            lib_addrs: Default::default(),
            phantom_db: PhantomData,
        }
    }

    /// Deploy a library contract with a dynamic ID and no constructor.
    fn deploy_library(
        &mut self,
        state: &mut FvmGenesisState<DB>,
        next_id: &mut u64,
        lib_src: impl AsRef<Path>,
        lib_name: &str,
    ) -> anyhow::Result<()> {
        let fqn = self.hardhat.fqn(lib_src.as_ref(), lib_name);

        let bytecode = self
            .hardhat
            .bytecode(&lib_src, lib_name, &self.lib_addrs)
            .with_context(|| format!("failed to load library bytecode {fqn}"))?;

        let eth_addr = state
            .create_evm_actor(*next_id, bytecode)
            .with_context(|| format!("failed to create library actor {fqn}"))?;

        let id_addr = et::Address::from(EthAddress::from_id(*next_id).0);
        let eth_addr = et::Address::from(eth_addr.0);

        tracing::info!(
            actor_id = next_id,
            ?eth_addr,
            ?id_addr,
            fqn,
            "deployed Ethereum library"
        );

        // We can use the masked ID here or the delegated address.
        // Maybe the masked ID is quicker because it doesn't need to be resolved.
        self.lib_addrs.insert(fqn, id_addr);

        *next_id += 1;

        Ok(())
    }

    /// Construct the bytecode of a top-level contract and deploy it with some constructor parameters.
    fn deploy_contract<T>(
        &self,
        state: &mut FvmGenesisState<DB>,
        contract_name: &str,
        constructor_params: T,
    ) -> anyhow::Result<et::Address>
    where
        T: Tokenize,
    {
        let contract = self.top_contract(contract_name)?;
        let contract_id = contract.actor_id;
        let contract_src = contract_src(contract_name);

        let bytecode = self
            .hardhat
            .bytecode(contract_src, contract_name, &self.lib_addrs)
            .with_context(|| format!("failed to load {contract_name} bytecode"))?;

        let eth_addr = state
            .create_evm_actor_with_cons(contract_id, &contract.abi, bytecode, constructor_params)
            .with_context(|| format!("failed to create {contract_name} actor"))?;

        let id_addr = et::Address::from(EthAddress::from_id(contract_id).0);
        let eth_addr = et::Address::from(eth_addr.0);

        tracing::info!(
            actor_id = contract_id,
            ?eth_addr,
            ?id_addr,
            contract_name,
            "deployed Ethereum contract"
        );

        // The Ethereum address is more usable inside the EVM than the ID address.
        Ok(eth_addr)
    }

    /// Collect Facet Cuts for the diamond pattern, where the facet address comes from already deployed library facets.
    fn facets(&self, contract_name: &str) -> anyhow::Result<Vec<FacetCut>> {
        let contract = self.top_contract(contract_name)?;
        let mut facet_cuts = Vec::new();

        for facet in contract.facets.iter() {
            let facet_name = facet.name;
            let facet_src = contract_src(facet_name);
            let facet_fqn = self.hardhat.fqn(&facet_src, facet_name);

            let facet_addr = self
                .lib_addrs
                .get(&facet_fqn)
                .ok_or_else(|| anyhow!("facet {facet_name} has not been deployed"))?;

            let method_sigs = facet
                .abi
                .functions()
                .filter(|f| f.signature() != "init(bytes)")
                .map(|f| f.short_signature())
                .collect();

            let facet_cut = FacetCut {
                facet_address: *facet_addr,
                action: 0, // Add
                function_selectors: method_sigs,
            };

            facet_cuts.push(facet_cut);
        }

        Ok(facet_cuts)
    }

    fn top_contract(&self, contract_name: &str) -> anyhow::Result<&EthContract> {
        self.top_contracts
            .get(contract_name)
            .ok_or_else(|| anyhow!("unknown top contract name: {contract_name}"))
    }
}

/// Sum of balances in the genesis accounts.
fn circ_supply(g: &Genesis) -> TokenAmount {
    g.accounts
        .iter()
        .fold(TokenAmount::zero(), |s, a| s + a.balance.clone())
}

#[cfg(any(feature = "test-util", test))]
pub async fn create_test_genesis_state(
    bundle_path: PathBuf,
    custom_actors_bundle_path: PathBuf,
    ipc_path: PathBuf,
    genesis_params: Genesis,
) -> anyhow::Result<(FvmGenesisState<MemoryBlockstore>, GenesisOutput)> {
    let builder = GenesisBuilder::new(
        bundle_path,
        custom_actors_bundle_path,
        ipc_path,
        genesis_params,
    );

    let mut state = builder.init_state().await?;
    let out = builder.populate_state(&mut state, builder.genesis_params.clone())?;
    Ok((state, out))
}

#[cfg(test)]
mod tests {
    use crate::genesis::GenesisAppState;

    #[test]
    fn test_compression() {
        let bytes = (0..10000)
            .map(|_| rand::random::<u8>())
            .collect::<Vec<u8>>();

        let s = GenesisAppState::v1(bytes.clone())
            .compress_and_encode()
            .unwrap();
        let recovered = GenesisAppState::decode_and_decompress(&s).unwrap();

        assert_eq!(recovered, bytes);
    }
}

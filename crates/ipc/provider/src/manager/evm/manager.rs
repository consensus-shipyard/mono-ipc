// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT

use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use ethers_contract::{ContractError, EthLogDecode, LogMeta};
use ipc_actors_abis::{
    checkpointing_facet, gateway_getter_facet, gateway_manager_facet, lib_gateway, lib_quorum,
    lib_staking_change_log, register_subnet_facet, subnet_actor_activity_facet,
    subnet_actor_checkpointing_facet, subnet_actor_getter_facet, subnet_actor_manager_facet,
    subnet_actor_reward_facet,
};
use ipc_api::evm::{fil_to_eth_amount, payload_to_evm_address, subnet_id_to_evm_addresses};
use ipc_api::validator::from_contract_validators;
use reqwest::header::HeaderValue;
use reqwest::Client;
use std::net::{IpAddr, SocketAddr};

use ipc_api::subnet::{Asset, AssetKind, PermissionMode};
use ipc_api::{eth_to_fil_amount, ethers_address_to_fil_address};

use crate::config::subnet::SubnetConfig;
use crate::config::Subnet;
use crate::lotus::message::ipc::SubnetInfo;
use crate::manager::subnet::{
    BottomUpCheckpointRelayer, GetBlockHashResult, SubnetGenesisInfo, TopDownFinalityQuery,
    TopDownQueryPayload, ValidatorRewarder,
};

use crate::manager::{EthManager, SubnetManager};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use ethers::abi::Tokenizable;
use ethers::contract::abigen;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{Signer, SignerMiddleware};
use ethers::providers::{Authorization, Http, Provider};
use ethers::signers::{LocalWallet, Wallet};
use ethers::types::{Eip1559TransactionRequest, ValueOrArray, H256, U256};

use super::gas_estimator_middleware::Eip1559GasEstimatorMiddleware;
use ethers::middleware::Middleware;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::{address::Address, econ::TokenAmount};
use ipc_actors_abis::subnet_actor_activity_facet::ValidatorClaim;
use ipc_api::checkpoint::{
    consensus::ValidatorData, BottomUpCheckpoint, BottomUpCheckpointBundle, QuorumReachedEvent,
    Signature, VALIDATOR_REWARD_FIELDS,
};
use ipc_api::cross::IpcEnvelope;
use ipc_api::merkle::MerkleGen;
use ipc_api::staking::{StakingChangeRequest, ValidatorInfo, ValidatorStakingInfo};
use ipc_api::subnet::ConstructParams;
use ipc_api::subnet_id::SubnetID;
use ipc_observability::lazy_static;
use ipc_wallet::{EthKeyAddress, EvmKeyStore, PersistentKeyStore};
use num_traits::ToPrimitive;
use std::result;

pub type SignerWithFeeEstimatorMiddleware =
    Eip1559GasEstimatorMiddleware<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

/// Default polling time used by the Ethers provider to check for pending
/// transactions and events. Default is 7, and for our child subnets we
/// can reduce it to the block time (or potentially less)
const ETH_PROVIDER_POLLING_TIME: Duration = Duration::from_secs(1);
/// Maximum number of retries to fetch a transaction receipt.
/// The number of retries should ensure that for the block time
/// of the network the number of retires considering the polling
/// time above waits enough tie to get the transaction receipt.
/// We currently support a low polling time and high number of
/// retries so these numbers accommodate fast subnets with slow
/// roots (like Calibration and mainnet).
const TRANSACTION_RECEIPT_RETRIES: usize = 200;

/// The majority vote percentage for checkpoint submission when creating a subnet.
const SUBNET_MAJORITY_PERCENTAGE: u8 = 67;

pub struct EthSubnetManager {
    keystore: Option<Arc<RwLock<PersistentKeyStore<EthKeyAddress>>>>,
    ipc_contract_info: IPCContractInfo,
}

/// Keep track of the on chain information for the subnet manager
struct IPCContractInfo {
    gateway_addr: ethers::types::Address,
    registry_addr: ethers::types::Address,
    chain_id: u64,
    provider: Provider<Http>,
}

//TODO receive clarity on this implementation
abigen!(
    IERC20,
    r#"[
        function approve(address spender, uint256 amount) external returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#,
);

#[async_trait]
impl TopDownFinalityQuery for EthSubnetManager {
    async fn genesis_epoch(&self, subnet_id: &SubnetID) -> Result<ChainEpoch> {
        let address = contract_address_from_subnet(subnet_id)?;
        tracing::info!("querying genesis epoch in evm subnet contract: {address:}");

        let evm_subnet_id = gateway_getter_facet::SubnetID::try_from(subnet_id)?;

        let contract = gateway_getter_facet::GatewayGetterFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let (exists, subnet) = contract.get_subnet(evm_subnet_id).call().await?;
        if !exists {
            return Err(anyhow!("subnet: {} does not exists", subnet_id));
        }
        Ok(subnet.genesis_epoch.as_u64() as ChainEpoch)
    }

    async fn chain_head_height(&self) -> Result<ChainEpoch> {
        let block = self
            .ipc_contract_info
            .provider
            .get_block_number()
            .await
            .context("cannot get evm block number")?;
        Ok(block.as_u64() as ChainEpoch)
    }

    async fn get_top_down_msgs(
        &self,
        subnet_id: &SubnetID,
        epoch: ChainEpoch,
    ) -> Result<TopDownQueryPayload<Vec<IpcEnvelope>>> {
        let gateway_contract = gateway_manager_facet::GatewayManagerFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let topic1 = contract_address_from_subnet(subnet_id)?;
        tracing::debug!(
            "getting top down messages for subnet: {:?} with topic 1: {}",
            subnet_id,
            topic1,
        );

        let ev = gateway_contract
            .event::<lib_gateway::NewTopDownMessageFilter>()
            .from_block(epoch as u64)
            .to_block(epoch as u64)
            .topic1(topic1)
            .address(ValueOrArray::Value(gateway_contract.address()));

        let mut messages = vec![];
        let mut hash = None;
        for (event, meta) in query_with_meta(ev, gateway_contract.client()).await? {
            if let Some(h) = hash {
                if h != meta.block_hash {
                    return Err(anyhow!("block hash not equal"));
                }
            } else {
                hash = Some(meta.block_hash);
            }

            messages.push(IpcEnvelope::try_from(event.message)?);
        }

        let block_hash = if let Some(h) = hash {
            h.0.to_vec()
        } else {
            self.get_block_hash(epoch).await?.block_hash
        };
        Ok(TopDownQueryPayload {
            value: messages,
            block_hash,
        })
    }

    async fn get_block_hash(&self, height: ChainEpoch) -> Result<GetBlockHashResult> {
        let block = self
            .ipc_contract_info
            .provider
            .get_block(height as u64)
            .await?
            .ok_or_else(|| anyhow!("height does not exist"))?;

        Ok(GetBlockHashResult {
            parent_block_hash: block.parent_hash.to_fixed_bytes().to_vec(),
            block_hash: block
                .hash
                .ok_or_else(|| anyhow!("block hash is empty"))?
                .to_fixed_bytes()
                .to_vec(),
        })
    }

    async fn get_validator_changeset(
        &self,
        subnet_id: &SubnetID,
        epoch: ChainEpoch,
    ) -> Result<TopDownQueryPayload<Vec<StakingChangeRequest>>> {
        let address = contract_address_from_subnet(subnet_id)?;
        tracing::info!("querying validator changes in evm subnet contract: {address:}");

        let contract = subnet_actor_manager_facet::SubnetActorManagerFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let ev = contract
            .event::<lib_staking_change_log::NewStakingChangeRequestFilter>()
            .from_block(epoch as u64)
            .to_block(epoch as u64)
            .address(ValueOrArray::Value(contract.address()));

        let mut changes = vec![];
        let mut hash = None;
        for (event, meta) in query_with_meta(ev, contract.client()).await? {
            if let Some(h) = hash {
                if h != meta.block_hash {
                    return Err(anyhow!("block hash not equal"));
                }
            } else {
                hash = Some(meta.block_hash);
            }
            changes.push(StakingChangeRequest::try_from(event)?);
        }

        let block_hash = if let Some(h) = hash {
            h.0.to_vec()
        } else {
            self.get_block_hash(epoch).await?.block_hash
        };
        Ok(TopDownQueryPayload {
            value: changes,
            block_hash,
        })
    }

    async fn latest_parent_finality(&self) -> Result<ChainEpoch> {
        tracing::info!("querying latest parent finality ");

        let contract = gateway_getter_facet::GatewayGetterFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let finality = contract.get_latest_parent_finality().call().await?;
        Ok(finality.height.as_u64() as ChainEpoch)
    }
}

#[async_trait]
impl SubnetManager for EthSubnetManager {
    async fn create_subnet(&self, from: Address, params: ConstructParams) -> Result<Address> {
        self.ensure_same_gateway(&params.ipc_gateway_addr)?;

        let min_validator_stake = params
            .min_validator_stake
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid min validator stake"))?;

        tracing::debug!("calling create subnet for EVM manager");

        let route = subnet_id_to_evm_addresses(&params.parent)?;
        tracing::debug!("root SubnetID as Ethereum type: {route:?}");

        let params = register_subnet_facet::ConstructorParams {
            parent_id: register_subnet_facet::SubnetID {
                root: params.parent.root_id(),
                route,
            },
            ipc_gateway_addr: self.ipc_contract_info.gateway_addr,
            consensus: params.consensus as u64 as u8,
            min_activation_collateral: ethers::types::U256::from(min_validator_stake),
            min_validators: params.min_validators,
            bottom_up_check_period: params.bottomup_check_period as u64,
            majority_percentage: SUBNET_MAJORITY_PERCENTAGE,
            active_validators_limit: params.active_validators_limit,
            power_scale: 3,
            permission_mode: params.permission_mode as u8,
            supply_source: register_subnet_facet::Asset::try_from(params.supply_source)?,
            collateral_source: register_subnet_facet::Asset::try_from(params.collateral_source)?,
            validator_gater: payload_to_evm_address(params.validator_gater.payload())?,
            validator_rewarder: payload_to_evm_address(params.validator_rewarder.payload())?,
        };

        tracing::info!("creating subnet on evm with params: {params:?}");

        let signer = self.get_signer_with_fee_estimator(&from)?;
        let signer = Arc::new(signer);
        let registry_contract = register_subnet_facet::RegisterSubnetFacet::new(
            self.ipc_contract_info.registry_addr,
            signer.clone(),
        );

        let call =
            extend_call_with_pending_block(registry_contract.new_subnet_actor(params)).await?;
        // TODO: Edit call to get estimate premium
        let pending_tx = call.send().await?;
        // We need the retry to parse the deployment event. At the time of this writing, it's a bug
        // in current FEVM that without the retries, events are not picked up.
        // See https://github.com/filecoin-project/community/discussions/638 for more info and updates.
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        match receipt {
            Some(r) => {
                for log in r.logs {
                    tracing::debug!("log: {log:?}");

                    match ethers_contract::parse_log::<register_subnet_facet::SubnetDeployedFilter>(
                        log,
                    ) {
                        Ok(subnet_deploy) => {
                            let register_subnet_facet::SubnetDeployedFilter { subnet_addr } =
                                subnet_deploy;

                            tracing::debug!("subnet deployed at {subnet_addr:?}");
                            return ethers_address_to_fil_address(&subnet_addr);
                        }
                        Err(_) => {
                            tracing::debug!("no event for subnet actor published yet, continue");
                            continue;
                        }
                    }
                }
                Err(anyhow!("no logs receipt"))
            }
            None => Err(anyhow!("no receipt for event, txn not successful")),
        }
    }

    async fn join_subnet(
        &self,
        subnet: SubnetID,
        from: Address,
        collateral: TokenAmount,
        pub_key: Vec<u8>,
    ) -> Result<ChainEpoch> {
        let collateral = collateral
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid min validator stake"))?;

        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!(
            "interacting with evm subnet contract: {address:} with collateral: {collateral:}"
        );

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        let mut txn = contract.join(ethers::types::Bytes::from(pub_key), U256::from(collateral));
        txn = self.handle_txn_token(&subnet, txn, collateral, 0).await?;

        let txn = extend_call_with_pending_block(txn).await?;

        let pending_tx = txn.send().await?;
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        block_number_from_receipt(receipt)
    }

    async fn pre_fund(&self, subnet: SubnetID, from: Address, balance: TokenAmount) -> Result<()> {
        let balance = balance
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid initial balance"))?;

        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!("interacting with evm subnet contract: {address:} with balance: {balance:}");

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        let mut txn = contract.pre_fund(U256::from(balance));
        txn = self.handle_txn_token(&subnet, txn, 0, balance).await?;

        let txn = extend_call_with_pending_block(txn).await?;

        txn.send().await?;
        Ok(())
    }

    async fn pre_release(
        &self,
        subnet: SubnetID,
        from: Address,
        amount: TokenAmount,
    ) -> Result<()> {
        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!("pre-release funds from {subnet:} at contract: {address:}");

        let amount = amount
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid pre-release amount"))?;

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        extend_call_with_pending_block(contract.pre_release(amount.into()))
            .await?
            .send()
            .await?
            .await?;

        Ok(())
    }

    async fn stake(&self, subnet: SubnetID, from: Address, collateral: TokenAmount) -> Result<()> {
        let collateral = collateral
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid collateral amount"))?;

        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!(
            "interacting with evm subnet contract: {address:} with collateral: {collateral:}"
        );

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        let mut txn = contract.stake(U256::from(collateral));
        txn = self.handle_txn_token(&subnet, txn, collateral, 0).await?;

        let txn = extend_call_with_pending_block(txn).await?;

        txn.send().await?.await?;

        Ok(())
    }

    async fn unstake(
        &self,
        subnet: SubnetID,
        from: Address,
        collateral: TokenAmount,
    ) -> Result<()> {
        let collateral = collateral
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid collateral amount"))?;

        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!(
            "interacting with evm subnet contract: {address:} with collateral: {collateral:}"
        );

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        let txn = extend_call_with_pending_block(contract.unstake(collateral.into())).await?;
        txn.send().await?.await?;

        Ok(())
    }

    async fn leave_subnet(&self, subnet: SubnetID, from: Address) -> Result<()> {
        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!("leaving evm subnet: {subnet:} at contract: {address:}");

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        extend_call_with_pending_block(contract.leave())
            .await?
            .send()
            .await?
            .await?;

        Ok(())
    }

    async fn kill_subnet(&self, subnet: SubnetID, from: Address) -> Result<()> {
        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!("kill evm subnet: {subnet:} at contract: {address:}");

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        extend_call_with_pending_block(contract.kill())
            .await?
            .send()
            .await?
            .await?;

        Ok(())
    }

    async fn list_child_subnets(
        &self,
        gateway_addr: Address,
    ) -> Result<HashMap<SubnetID, SubnetInfo>> {
        self.ensure_same_gateway(&gateway_addr)?;

        let gateway_contract = gateway_getter_facet::GatewayGetterFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let mut s = HashMap::new();

        let evm_subnets = gateway_contract.list_subnets().call().await?;
        tracing::debug!("raw subnet: {evm_subnets:?}");

        for subnet in evm_subnets {
            let info = SubnetInfo::try_from(subnet)?;
            s.insert(info.id.clone(), info);
        }

        Ok(s)
    }

    async fn claim_collateral(&self, subnet: SubnetID, from: Address) -> Result<()> {
        let address = contract_address_from_subnet(&subnet)?;
        tracing::info!("claim collateral evm subnet: {subnet:} at contract: {address:}");

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let contract =
            subnet_actor_reward_facet::SubnetActorRewardFacet::new(address, signer.clone());

        extend_call_with_pending_block(contract.claim())
            .await?
            .send()
            .await?
            .await?;

        Ok(())
    }

    async fn fund(
        &self,
        subnet: SubnetID,
        gateway_addr: Address,
        from: Address,
        to: Address,
        amount: TokenAmount,
    ) -> Result<ChainEpoch> {
        self.ensure_same_gateway(&gateway_addr)?;

        let value = amount
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid value to fund"))?;

        tracing::info!("fund with evm gateway contract: {gateway_addr:} with value: {value:}, original: {amount:?}");

        let evm_subnet_id = gateway_manager_facet::SubnetID::try_from(&subnet)?;
        tracing::debug!("evm subnet id to fund: {evm_subnet_id:?}");

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let gateway_contract = gateway_manager_facet::GatewayManagerFacet::new(
            self.ipc_contract_info.gateway_addr,
            signer.clone(),
        );

        let mut txn = gateway_contract.fund(
            evm_subnet_id,
            gateway_manager_facet::FvmAddress::try_from(to)?,
        );
        txn.tx.set_value(value);
        let txn = extend_call_with_pending_block(txn).await?;

        let pending_tx = txn.send().await?;
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        block_number_from_receipt(receipt)
    }

    /// Approves the `from` address to use up to `amount` tokens from `token_address`.
    async fn approve_token(
        &self,
        subnet: SubnetID,
        from: Address,
        amount: TokenAmount,
    ) -> Result<ChainEpoch> {
        log::debug!("approve token, subnet: {subnet}, amount: {amount}, from: {from}");

        let value = fil_amount_to_eth_amount(&amount)?;

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);

        let subnet_supply_source = self.get_subnet_supply_source(&subnet).await?;
        if subnet_supply_source.kind != AssetKind::ERC20 {
            return Err(anyhow!("Invalid operation: Expected the subnet's supply source to be ERC20, but found a different kind."));
        }

        let token_address = payload_to_evm_address(
            subnet_supply_source
                .token_address
                .ok_or_else(|| anyhow!("zero adress not erc20"))?
                .payload(),
        )?;
        let token_contract = IERC20::new(token_address, signer.clone());

        let txn = token_contract.approve(self.ipc_contract_info.gateway_addr, value);
        let txn = extend_call_with_pending_block(txn).await?;

        let pending_tx = txn.send().await?;
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        block_number_from_receipt(receipt)
    }

    async fn fund_with_token(
        &self,
        subnet: SubnetID,
        from: Address,
        to: Address,
        amount: TokenAmount,
    ) -> Result<ChainEpoch> {
        tracing::debug!(
            "fund with token, subnet: {subnet}, amount: {amount}, from: {from}, to: {to}"
        );

        let value = fil_amount_to_eth_amount(&amount)?;
        let evm_subnet_id = gateway_manager_facet::SubnetID::try_from(&subnet)?;

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let gateway_contract = gateway_manager_facet::GatewayManagerFacet::new(
            self.ipc_contract_info.gateway_addr,
            signer.clone(),
        );

        let txn = gateway_contract.fund_with_token(
            evm_subnet_id,
            gateway_manager_facet::FvmAddress::try_from(to)?,
            value,
        );
        let txn = extend_call_with_pending_block(txn).await?;

        let pending_tx = txn.send().await?;
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        block_number_from_receipt(receipt)
    }

    async fn release(
        &self,
        gateway_addr: Address,
        from: Address,
        to: Address,
        amount: TokenAmount,
    ) -> Result<ChainEpoch> {
        self.ensure_same_gateway(&gateway_addr)?;

        let value = amount
            .atto()
            .to_u128()
            .ok_or_else(|| anyhow!("invalid value to fund"))?;

        tracing::info!("release with evm gateway contract: {gateway_addr:} with value: {value:}");

        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let gateway_contract = gateway_manager_facet::GatewayManagerFacet::new(
            self.ipc_contract_info.gateway_addr,
            signer.clone(),
        );
        let mut txn = gateway_contract.release(gateway_manager_facet::FvmAddress::try_from(to)?);
        txn.tx.set_value(value);
        let txn = extend_call_with_pending_block(txn).await?;

        let pending_tx = txn.send().await?;
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        block_number_from_receipt(receipt)
    }

    /// Send value between two addresses in a subnet
    async fn send_value(&self, from: Address, to: Address, amount: TokenAmount) -> Result<()> {
        let signer = Arc::new(self.get_signer_with_fee_estimator(&from)?);
        let tx = Eip1559TransactionRequest::new()
            .to(payload_to_evm_address(to.payload())?)
            .value(fil_to_eth_amount(&amount)?);

        let tx_pending = signer.send_transaction(tx, None).await?;

        tracing::info!(
            "sending FIL from {from:} to {to:} in tx {:?}",
            tx_pending.tx_hash()
        );
        tx_pending.await?;
        Ok(())
    }

    async fn wallet_balance(&self, address: &Address) -> Result<TokenAmount> {
        let balance = self
            .ipc_contract_info
            .provider
            .clone()
            .get_balance(payload_to_evm_address(address.payload())?, None)
            .await?;
        Ok(TokenAmount::from_atto(balance.as_u128()))
    }

    async fn get_chain_id(&self) -> Result<String> {
        Ok(self
            .ipc_contract_info
            .provider
            .get_chainid()
            .await?
            .to_string())
    }

    async fn get_commit_sha(&self) -> Result<[u8; 32]> {
        let gateway_contract = gateway_getter_facet::GatewayGetterFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        tracing::debug!(
            "gateway_contract address : {:?}",
            self.ipc_contract_info.gateway_addr
        );
        tracing::debug!(
            "gateway_contract_getter_facet address : {:?}",
            gateway_contract.address()
        );

        let commit_sha = gateway_contract
            .get_commit_sha()
            .call()
            .await
            .map_err(|e| anyhow!("cannot get commit sha due to: {e:}"))?;

        Ok(commit_sha)
    }

    async fn get_subnet_supply_source(&self, subnet: &SubnetID) -> Result<Asset> {
        let address = contract_address_from_subnet(subnet)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let raw = contract.supply_source().call().await?;
        Ok(Asset::try_from(raw)?)
    }

    async fn get_subnet_collateral_source(&self, subnet: &SubnetID) -> Result<Asset> {
        let address = contract_address_from_subnet(subnet)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let raw = contract.collateral_source().call().await?;
        Ok(Asset::try_from(raw)?)
    }

    async fn get_genesis_info(&self, subnet: &SubnetID) -> Result<SubnetGenesisInfo> {
        let address = contract_address_from_subnet(subnet)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let genesis_balances = contract.genesis_balances().await?;
        let bottom_up_checkpoint_period = contract.bottom_up_check_period().call().await?.as_u64();

        Ok(SubnetGenesisInfo {
            // Active validators limit set for the child subnet.
            active_validators_limit: contract.active_validators_limit().call().await?,
            // Bottom-up checkpoint period set in the subnet actor.
            bottom_up_checkpoint_period,
            // Genesis epoch when the subnet was bootstrapped in the parent.
            genesis_epoch: self.genesis_epoch(subnet).await?,
            // Majority percentage of
            majority_percentage: contract.majority_percentage().call().await?,
            // Minimum collateral required for subnets to register into the subnet
            min_collateral: eth_to_fil_amount(&contract.min_activation_collateral().call().await?)?,
            // Custom message fee that the child subnet wants to set for cross-net messages
            validators: from_contract_validators(contract.genesis_validators().call().await?)?,
            genesis_balances: into_genesis_balance_map(genesis_balances.0, genesis_balances.1)?,
            // TODO: fixme https://github.com/consensus-shipyard/ipc-monorepo/issues/496
            permission_mode: PermissionMode::Collateral,
            supply_source: Asset {
                kind: AssetKind::Native,
                token_address: None,
            },
        })
    }

    async fn add_bootstrap(
        &self,
        subnet: &SubnetID,
        from: &Address,
        endpoint: String,
    ) -> Result<()> {
        let address = contract_address_from_subnet(subnet)?;

        if is_valid_bootstrap_addr(&endpoint).is_none() {
            return Err(anyhow!("wrong format for bootstrap endpoint"));
        }

        let signer = Arc::new(self.get_signer_with_fee_estimator(from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        extend_call_with_pending_block(contract.add_bootstrap_node(endpoint))
            .await?
            .send()
            .await?
            .await?;

        Ok(())
    }

    async fn list_bootstrap_nodes(&self, subnet: &SubnetID) -> Result<Vec<String>> {
        let address = contract_address_from_subnet(subnet)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        Ok(contract.get_bootstrap_nodes().call().await?)
    }

    async fn get_validator_info(
        &self,
        subnet: &SubnetID,
        validator: &Address,
    ) -> Result<ValidatorInfo> {
        let address = contract_address_from_subnet(subnet)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let validator = payload_to_evm_address(validator.payload())?;

        let validator_info = contract.get_validator(validator).call().await?;
        let is_active = contract.is_active_validator(validator).call().await?;
        let is_waiting = contract.is_waiting_validator(validator).call().await?;

        Ok(ValidatorInfo {
            staking: ValidatorStakingInfo::try_from(validator_info)?,
            is_active,
            is_waiting,
        })
    }

    async fn list_validators(&self, subnet: &SubnetID) -> Result<Vec<(Address, ValidatorInfo)>> {
        let address = contract_address_from_subnet(subnet)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let mut addresses: Vec<Address> = vec![];
        let mut validators: Vec<ValidatorInfo> = vec![];

        let active = contract.get_active_validators().call().await?;
        addresses.extend(
            active
                .iter()
                .map(ethers_address_to_fil_address)
                .collect::<Result<Vec<_>, _>>()?,
        );
        for addr in active {
            let info = contract.get_validator(addr).call().await?;
            validators.push(ValidatorInfo {
                staking: ValidatorStakingInfo::try_from(info)?,
                is_active: true,
                is_waiting: false,
            });
        }
        let waiting = contract.get_waiting_validators().call().await?;
        addresses.extend(
            waiting
                .iter()
                .map(ethers_address_to_fil_address)
                .collect::<Result<Vec<_>, _>>()?,
        );
        for addr in waiting {
            let info = contract.get_validator(addr).call().await?;
            validators.push(ValidatorInfo {
                staking: ValidatorStakingInfo::try_from(info)?,
                is_active: false,
                is_waiting: true,
            });
        }

        Ok(addresses.into_iter().zip(validators).collect())
    }

    async fn set_federated_power(
        &self,
        from: &Address,
        subnet: &SubnetID,
        validators: &[Address],
        public_keys: &[Vec<u8>],
        federated_power: &[u128],
    ) -> Result<ChainEpoch> {
        let address = contract_address_from_subnet(subnet)?;
        tracing::info!("interacting with evm subnet contract: {address:}");

        let signer = Arc::new(self.get_signer_with_fee_estimator(from)?);
        let contract =
            subnet_actor_manager_facet::SubnetActorManagerFacet::new(address, signer.clone());

        let addresses: Vec<ethers::core::types::Address> = validators
            .iter()
            .map(|validator_address| payload_to_evm_address(validator_address.payload()).unwrap())
            .collect();
        tracing::debug!("converted addresses: {:?}", addresses);

        let pubkeys: Vec<ethers::core::types::Bytes> = public_keys
            .iter()
            .map(|key| ethers::core::types::Bytes::from(key.clone()))
            .collect();
        tracing::debug!("converted pubkeys: {:?}", pubkeys);

        let power_u256: Vec<ethers::core::types::U256> = federated_power
            .iter()
            .map(|power| ethers::core::types::U256::from(*power))
            .collect();
        tracing::debug!("converted power: {:?}", power_u256);

        tracing::debug!("from address: {:?}", from);

        let call = contract.set_federated_power(addresses, pubkeys, power_u256);
        let txn = extend_call_with_pending_block(call).await?;
        let pending_tx = txn.send().await?;
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        block_number_from_receipt(receipt)
    }
}

#[async_trait]
impl EthManager for EthSubnetManager {
    async fn current_epoch(&self) -> Result<ChainEpoch> {
        let block_number = self
            .ipc_contract_info
            .provider
            .get_block_number()
            .await?
            .as_u64();
        Ok(block_number as ChainEpoch)
    }

    async fn bottom_up_checkpoint(
        &self,
        epoch: ChainEpoch,
    ) -> Result<subnet_actor_checkpointing_facet::BottomUpCheckpoint> {
        let gateway_contract = gateway_getter_facet::GatewayGetterFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let checkpoint = gateway_contract
            .bottom_up_checkpoint(ethers::types::U256::from(epoch as u64))
            .call()
            .await?;
        tracing::debug!("raw bottom up checkpoint from gateway: {checkpoint:?}");
        let token = checkpoint.into_token();
        let c = subnet_actor_checkpointing_facet::BottomUpCheckpoint::from_token(token)?;
        Ok(c)
    }

    async fn get_applied_top_down_nonce(&self, subnet_id: &SubnetID) -> Result<u64> {
        let route = subnet_id_to_evm_addresses(subnet_id)?;
        tracing::debug!("getting applied top down nonce for route: {route:?}");

        let evm_subnet_id = gateway_getter_facet::SubnetID {
            root: subnet_id.root_id(),
            route,
        };

        let gateway_contract = gateway_getter_facet::GatewayGetterFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let (exists, nonce) = gateway_contract
            .get_top_down_nonce(evm_subnet_id)
            .call()
            .await
            .map_err(|e| anyhow!("cannot get applied top down nonce due to: {e:}"))?;

        if !exists {
            Err(anyhow!("subnet {:?} does not exists", subnet_id))
        } else {
            Ok(nonce)
        }
    }

    async fn subnet_bottom_up_checkpoint_period(&self, subnet_id: &SubnetID) -> Result<ChainEpoch> {
        let address = contract_address_from_subnet(subnet_id)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let period = contract.bottom_up_check_period().call().await?.as_u64();
        Ok(period as ChainEpoch)
    }

    async fn prev_bottom_up_checkpoint_hash(
        &self,
        subnet_id: &SubnetID,
        epoch: ChainEpoch,
    ) -> Result<[u8; 32]> {
        let address = contract_address_from_subnet(subnet_id)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let (exists, hash) = contract
            .bottom_up_checkpoint_hash_at_epoch(U256::from(epoch as u64))
            .await?;
        if !exists {
            return if epoch == 0 {
                // first ever epoch, return empty bytes
                return Ok([0u8; 32]);
            } else {
                Err(anyhow!("checkpoint does not exists: {epoch:}"))
            };
        }
        Ok(hash)
    }

    async fn min_validators(&self, subnet_id: &SubnetID) -> Result<u64> {
        let address = contract_address_from_subnet(subnet_id)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        Ok(contract.min_validators().call().await?)
    }
}

impl EthSubnetManager {
    pub fn new(
        gateway_addr: ethers::types::Address,
        registry_addr: ethers::types::Address,
        chain_id: u64,
        provider: Provider<Http>,
        keystore: Option<Arc<RwLock<PersistentKeyStore<EthKeyAddress>>>>,
    ) -> Self {
        Self {
            keystore,
            ipc_contract_info: IPCContractInfo {
                gateway_addr,
                registry_addr,
                chain_id,
                provider,
            },
        }
    }

    /// This method handles the "msg.value" based on different collateral/supply source
    /// asset kind.
    pub async fn handle_txn_token<B, D, M>(
        &self,
        subnet: &SubnetID,
        mut txn: ethers_contract::FunctionCall<B, D, M>,
        collateral: u128,
        balance: u128,
    ) -> anyhow::Result<ethers_contract::FunctionCall<B, D, M>>
    where
        B: std::borrow::Borrow<D>,
        M: ethers::abi::Detokenize,
    {
        let supply_source = self.get_subnet_supply_source(subnet).await?;
        let collateral_source = self.get_subnet_collateral_source(subnet).await?;

        match (supply_source.kind, collateral_source.kind) {
            (AssetKind::Native, AssetKind::Native) => _ = txn.tx.set_value(balance + collateral),
            (AssetKind::Native, AssetKind::ERC20) => _ = txn.tx.set_value(balance),
            (AssetKind::ERC20, AssetKind::Native) => _ = txn.tx.set_value(collateral),
            _ => {}
        }
        Ok(txn)
    }

    pub fn ensure_same_gateway(&self, gateway: &Address) -> Result<()> {
        let evm_gateway_addr = payload_to_evm_address(gateway.payload())?;
        if evm_gateway_addr != self.ipc_contract_info.gateway_addr {
            Err(anyhow!("Gateway address not matching with config"))
        } else {
            Ok(())
        }
    }

    pub fn keystore(&self) -> Result<Arc<RwLock<PersistentKeyStore<EthKeyAddress>>>> {
        self.keystore
            .clone()
            .ok_or(anyhow!("no evm keystore available"))
    }

    /// Get the ethers singer instance.
    /// We use filecoin addresses throughout our whole code-base
    /// and translate them to evm addresses when relevant.
    fn get_signer_with_fee_estimator(
        &self,
        addr: &Address,
    ) -> Result<SignerWithFeeEstimatorMiddleware> {
        // convert to its underlying eth address
        let addr = payload_to_evm_address(addr.payload())?;
        let keystore = self.keystore()?;
        let keystore = keystore.read().unwrap();
        let private_key = keystore
            .get(&addr.into())?
            .ok_or_else(|| anyhow!("address {addr:} does not have private key in key store"))?;
        let wallet = LocalWallet::from_bytes(private_key.private_key())?
            .with_chain_id(self.ipc_contract_info.chain_id);

        use super::gas_estimator_middleware::Eip1559GasEstimatorMiddleware;

        let signer = SignerMiddleware::new(self.ipc_contract_info.provider.clone(), wallet);
        Ok(Eip1559GasEstimatorMiddleware::new(signer))
    }

    pub fn from_subnet_with_wallet_store(
        subnet: &Subnet,
        keystore: Option<Arc<RwLock<PersistentKeyStore<EthKeyAddress>>>>,
    ) -> Result<Self> {
        let url = subnet.rpc_http().clone();
        let auth_token = subnet.auth_token();

        let SubnetConfig::Fevm(config) = &subnet.config;

        let mut client = Client::builder();

        if let Some(auth_token) = auth_token {
            let auth = Authorization::Bearer(auth_token);
            let mut auth_value = HeaderValue::from_str(&auth.to_string())?;
            auth_value.set_sensitive(true);

            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::AUTHORIZATION, auth_value);

            client = client.default_headers(headers);
        }

        if let Some(timeout) = subnet.rpc_timeout() {
            client = client.timeout(timeout);
        }

        let client = client.build()?;

        let provider = Http::new_with_client(url, client);

        let mut provider = Provider::new(provider);
        // set polling interval for provider to fit fast child subnets block times.
        // TODO: We may want to make it dynamic so it adjusts depending on the type of network
        // so we don't have a too slow or too fast polling for the underlying block times.
        provider.set_interval(ETH_PROVIDER_POLLING_TIME);
        let gateway_address = payload_to_evm_address(config.gateway_addr.payload())?;
        let registry_address = payload_to_evm_address(config.registry_addr.payload())?;

        Ok(Self::new(
            gateway_address,
            registry_address,
            subnet.id.chain_id(),
            provider,
            keystore,
        ))
    }
}

#[async_trait]
impl BottomUpCheckpointRelayer for EthSubnetManager {
    async fn submit_checkpoint(
        &self,
        submitter: &Address,
        checkpoint: BottomUpCheckpoint,
        signatures: Vec<Signature>,
        signatories: Vec<Address>,
    ) -> anyhow::Result<ChainEpoch> {
        let address = contract_address_from_subnet(&checkpoint.subnet_id)?;
        tracing::debug!(
            "submit bottom up checkpoint: {checkpoint:?} in evm subnet contract: {address:}"
        );

        let signatures = signatures
            .into_iter()
            .map(ethers::types::Bytes::from)
            .collect::<Vec<_>>();
        let signatories = signatories
            .into_iter()
            .map(|addr| payload_to_evm_address(addr.payload()))
            .collect::<result::Result<Vec<_>, _>>()?;

        let checkpoint =
            subnet_actor_checkpointing_facet::BottomUpCheckpoint::try_from(checkpoint)?;

        let signer = Arc::new(self.get_signer_with_fee_estimator(submitter)?);
        let contract = subnet_actor_checkpointing_facet::SubnetActorCheckpointingFacet::new(
            address,
            signer.clone(),
        );
        let call = contract.submit_checkpoint(checkpoint, signatories, signatures);
        let call = extend_call_with_pending_block(call).await?;

        let pending_tx = call.send().await?;
        let receipt = pending_tx.retries(TRANSACTION_RECEIPT_RETRIES).await?;
        block_number_from_receipt(receipt)
    }

    async fn last_bottom_up_checkpoint_height(
        &self,
        subnet_id: &SubnetID,
    ) -> anyhow::Result<ChainEpoch> {
        let address = contract_address_from_subnet(subnet_id)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let epoch = contract.last_bottom_up_checkpoint_height().call().await?;
        Ok(epoch.as_u64() as ChainEpoch)
    }

    async fn checkpoint_period(&self, subnet_id: &SubnetID) -> anyhow::Result<ChainEpoch> {
        let address = contract_address_from_subnet(subnet_id)?;
        let contract = subnet_actor_getter_facet::SubnetActorGetterFacet::new(
            address,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );
        let epoch = contract.bottom_up_check_period().call().await?;
        Ok(epoch.as_u64() as ChainEpoch)
    }

    async fn checkpoint_bundle_at(
        &self,
        height: ChainEpoch,
    ) -> anyhow::Result<Option<BottomUpCheckpointBundle>> {
        let contract = gateway_getter_facet::GatewayGetterFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let (checkpoint, _, signatories, signatures) = contract
            .get_checkpoint_signature_bundle(U256::from(height))
            .call()
            .await?;

        if checkpoint.block_height.as_u64() == 0 {
            return Ok(None);
        }

        let checkpoint = BottomUpCheckpoint::try_from(checkpoint)?;
        let signatories = signatories
            .into_iter()
            .map(|s| ethers_address_to_fil_address(&s))
            .collect::<Result<Vec<_>, _>>()?;
        let signatures = signatures
            .into_iter()
            .map(|s| s.to_vec())
            .collect::<Vec<_>>();

        Ok(Some(BottomUpCheckpointBundle {
            checkpoint,
            signatures,
            signatories,
        }))
    }

    async fn quorum_reached_events(&self, height: ChainEpoch) -> Result<Vec<QuorumReachedEvent>> {
        let contract = checkpointing_facet::CheckpointingFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let ev = contract
            .event::<lib_quorum::QuorumReachedFilter>()
            .from_block(height as u64)
            .to_block(height as u64)
            .address(ValueOrArray::Value(contract.address()));

        let mut events = vec![];
        for (event, _meta) in query_with_meta(ev, contract.client()).await? {
            events.push(QuorumReachedEvent {
                obj_kind: event.obj_kind,
                height: event.height.as_u64() as ChainEpoch,
                obj_hash: event.obj_hash.to_vec(),
                quorum_weight: eth_to_fil_amount(&event.quorum_weight)?,
            });
        }

        Ok(events)
    }
    async fn current_epoch(&self) -> Result<ChainEpoch> {
        let epoch = self
            .ipc_contract_info
            .provider
            .get_block_number()
            .await?
            .as_u64();
        Ok(epoch as ChainEpoch)
    }
}

lazy_static!(
    /// ABI types of the Merkle tree which contains validator addresses and their committed block count.
    pub static ref VALIDATOR_SUMMARY_FIELDS: Vec<String> = vec!["address".to_owned(), "uint64".to_owned()];
);

#[async_trait]
impl ValidatorRewarder for EthSubnetManager {
    /// Query validator claims, indexed by checkpoint height, to batch claim rewards.
    async fn query_reward_claims(
        &self,
        validator_addr: &Address,
        from_checkpoint: ChainEpoch,
        to_checkpoint: ChainEpoch,
    ) -> Result<Vec<(u64, ValidatorClaim)>> {
        let contract = checkpointing_facet::CheckpointingFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let ev = contract
            .event::<checkpointing_facet::ActivityRollupRecordedFilter>()
            .from_block(from_checkpoint as u64)
            .to_block(to_checkpoint as u64)
            .address(ValueOrArray::Value(contract.address()));

        let validator_eth_addr = payload_to_evm_address(validator_addr.payload())?;

        let mut claims = vec![];
        for (event, meta) in query_with_meta(ev, contract.client()).await? {
            tracing::debug!(
                "found activity bundle published at height: {}",
                meta.block_number
            );

            // Check if we have claims for this validator in this block.
            let our_data = event
                .rollup
                .consensus
                .data
                .iter()
                .find(|v| v.validator == validator_eth_addr);

            // If we don't, skip this block.
            let Some(data) = our_data else {
                tracing::info!(
                    "target validator address has no reward claims in epoch {}",
                    meta.block_number
                );
                continue;
            };

            let proof = gen_merkle_proof(&event.rollup.consensus.data, data)?;

            // Construct the claim and add it to the list.
            let claim = ValidatorClaim {
                // Even though it's the same struct but still need to do a mapping due to
                // different crate from ethers-rs
                data: subnet_actor_activity_facet::ValidatorData {
                    validator: data.validator,
                    blocks_committed: data.blocks_committed,
                },
                proof: proof.into_iter().map(|v| v.into()).collect(),
            };
            claims.push((event.checkpoint_height, claim));
        }

        Ok(claims)
    }

    /// Query validator rewards in the current subnet, without obtaining proofs.
    async fn query_validator_rewards(
        &self,
        validator_addr: &Address,
        from_checkpoint: ChainEpoch,
        to_checkpoint: ChainEpoch,
    ) -> Result<Vec<(u64, ValidatorData)>> {
        let contract = checkpointing_facet::CheckpointingFacet::new(
            self.ipc_contract_info.gateway_addr,
            Arc::new(self.ipc_contract_info.provider.clone()),
        );

        let ev = contract
            .event::<checkpointing_facet::ActivityRollupRecordedFilter>()
            .from_block(from_checkpoint as u64)
            .to_block(to_checkpoint as u64)
            .address(ValueOrArray::Value(contract.address()));

        let mut rewards = vec![];
        let validator_eth_addr = payload_to_evm_address(validator_addr.payload())?;

        for (event, meta) in query_with_meta(ev, contract.client()).await? {
            tracing::debug!(
                "found activity bundle published at height: {}",
                meta.block_number
            );

            // Check if we have rewards for this validator in this block.
            if let Some(data) = event
                .rollup
                .consensus
                .data
                .iter()
                .find(|v| v.validator == validator_eth_addr)
            {
                // TODO type conversion.
                let data = ValidatorData {
                    validator: *validator_addr,
                    blocks_committed: data.blocks_committed,
                };
                rewards.push((meta.block_number.as_u64(), data));
            }
        }

        Ok(rewards)
    }

    /// Claim validator rewards in a batch for the specified subnet.
    async fn batch_subnet_claim(
        &self,
        submitter: &Address,
        reward_claim_subnet: &SubnetID,
        reward_origin_subnet: &SubnetID,
        claims: Vec<(u64, ValidatorClaim)>,
    ) -> Result<()> {
        let signer = Arc::new(self.get_signer_with_fee_estimator(submitter)?);
        let contract = subnet_actor_activity_facet::SubnetActorActivityFacet::new(
            contract_address_from_subnet(reward_claim_subnet)?,
            signer.clone(),
        );

        // separate the Vec of tuples claims into two Vecs of Height and Claim
        let (heights, claims): (Vec<u64>, Vec<ValidatorClaim>) = claims.into_iter().unzip();

        let call = {
            let call =
                contract.batch_subnet_claim(reward_origin_subnet.try_into()?, heights, claims);
            extend_call_with_pending_block(call).await?
        };

        call.send().await?;

        Ok(())
    }
}

fn gen_merkle_proof(
    validator_data: &[checkpointing_facet::ValidatorData],
    validator: &checkpointing_facet::ValidatorData,
) -> anyhow::Result<Vec<H256>> {
    // Utilty function to pack validator data into a vector of strings for proof generation.
    let pack_validator_data = |v: &checkpointing_facet::ValidatorData| {
        vec![format!("{:?}", v.validator), v.blocks_committed.to_string()]
    };

    let leaves = order_validator_data(validator_data)?;
    let tree = MerkleGen::new(pack_validator_data, &leaves, &VALIDATOR_REWARD_FIELDS)?;

    tree.get_proof(validator)
}

fn order_validator_data(
    validator_data: &[checkpointing_facet::ValidatorData],
) -> anyhow::Result<Vec<checkpointing_facet::ValidatorData>> {
    let mut mapped = validator_data
        .iter()
        .map(|a| ethers_address_to_fil_address(&a.validator).map(|v| (v, a.blocks_committed)))
        .collect::<Result<Vec<_>, _>>()?;

    mapped.sort_by(|a, b| {
        let cmp = a.0.cmp(&b.0);
        if cmp.is_eq() {
            // Address will be unique, do this just in case equal
            a.1.cmp(&b.1)
        } else {
            cmp
        }
    });

    let back_to_eth = |(fvm_addr, blocks): (Address, u64)| {
        payload_to_evm_address(fvm_addr.payload()).map(|v| checkpointing_facet::ValidatorData {
            validator: v,
            blocks_committed: blocks,
        })
    };
    mapped
        .into_iter()
        .map(back_to_eth)
        .collect::<Result<Vec<_>, _>>()
}

/// Takes a `FunctionCall` input and returns a new instance with an estimated optimal `gas_premium`.
/// The function also uses the pending block number to help retrieve the latest nonce
/// via `get_transaction_count` with the `pending` parameter.
pub(crate) async fn extend_call_with_pending_block<B, D, M>(
    call: ethers_contract::FunctionCall<B, D, M>,
) -> Result<ethers_contract::FunctionCall<B, D, M>>
where
    B: std::borrow::Borrow<D>,
    M: ethers::abi::Detokenize,
{
    Ok(call.block(ethers::types::BlockNumber::Pending))
}

/// Get the block number from the transaction receipt
fn block_number_from_receipt(
    receipt: Option<ethers::types::TransactionReceipt>,
) -> Result<ChainEpoch> {
    match receipt {
        Some(r) => {
            let block_number = r
                .block_number
                .ok_or_else(|| anyhow!("cannot get block number"))?;
            Ok(block_number.as_u64() as ChainEpoch)
        }
        None => Err(anyhow!(
            "txn sent to network, but receipt cannot be obtained, please check scanner"
        )),
    }
}

fn is_valid_bootstrap_addr(input: &str) -> Option<(String, IpAddr, u16)> {
    let parts: Vec<&str> = input.split('@').collect();

    if parts.len() == 2 {
        let pubkey = parts[0].to_string();
        let addr_str = parts[1];

        if let Ok(addr) = addr_str.parse::<SocketAddr>() {
            return Some((pubkey, addr.ip(), addr.port()));
        }
    }

    None
}

/// This is a replacement for `Event::query_with_meta` in `ethers-contract`
/// because in that one we don't get access to the `reverted` field, which
/// we need to filteron in the currently deployed `1.25-rc4` version of Lotus.
async fn query_with_meta<B, M, D>(
    event: ethers::contract::Event<B, M, D>,
    client: B,
) -> Result<Vec<(D, LogMeta)>, ContractError<M>>
where
    B: Borrow<M>,
    M: Middleware,
    D: EthLogDecode,
{
    let logs = client
        .borrow()
        .get_logs(&event.filter)
        .await
        .map_err(ContractError::from_middleware_error)?;

    let events = logs
        .into_iter()
        .filter(|l| !l.removed.unwrap_or_default())
        .map(|log| {
            let meta = LogMeta::from(&log);
            let event = ethers::contract::parse_log::<D>(log)?;
            Ok((event, meta))
        })
        .collect::<Result<_, ContractError<M>>>()?;

    Ok(events)
}

fn into_genesis_balance_map(
    addrs: Vec<ethers::types::Address>,
    balances: Vec<ethers::types::U256>,
) -> Result<BTreeMap<Address, TokenAmount>> {
    let mut map = BTreeMap::new();
    for (a, b) in addrs.into_iter().zip(balances) {
        map.insert(ethers_address_to_fil_address(&a)?, eth_to_fil_amount(&b)?);
    }
    Ok(map)
}

pub(crate) fn fil_amount_to_eth_amount(amount: &TokenAmount) -> Result<ethers::types::U256> {
    let v = ethers::types::U256::from_dec_str(&amount.atto().to_string())?;
    Ok(v)
}

/// Convert the ipc SubnetID type to an evm address. It extracts the last address from the Subnet id
/// children and turns it into evm address.
pub(crate) fn contract_address_from_subnet(subnet: &SubnetID) -> Result<ethers::types::Address> {
    let children = subnet.children();
    let ipc_addr = children
        .last()
        .ok_or_else(|| anyhow!("{subnet:} has no child"))?;

    payload_to_evm_address(ipc_addr.payload())
}

impl TryFrom<gateway_getter_facet::Subnet> for SubnetInfo {
    type Error = anyhow::Error;

    fn try_from(value: gateway_getter_facet::Subnet) -> Result<Self, Self::Error> {
        Ok(SubnetInfo {
            id: SubnetID::try_from(value.id)?,
            stake: eth_to_fil_amount(&value.stake)?,
            circ_supply: eth_to_fil_amount(&value.circ_supply)?,
            genesis_epoch: value.genesis_epoch.as_u64() as ChainEpoch,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::evm::manager::contract_address_from_subnet;
    use ethers::core::rand::prelude::SliceRandom;
    use ethers::core::rand::{random, thread_rng};
    use fvm_shared::address::Address;
    use ipc_actors_abis::checkpointing_facet::{checkpointing_facet, ValidatorData};
    use ipc_api::checkpoint::VALIDATOR_REWARD_FIELDS;
    use ipc_api::merkle::MerkleGen;
    use ipc_api::subnet_id::SubnetID;
    use std::str::FromStr;

    #[test]
    fn test_agent_subnet_to_evm_address() {
        let addr = Address::from_str("f410ffzyuupbyl2uiucmzr3lu3mtf3luyknthaz4xsrq").unwrap();
        let id = SubnetID::new(0, vec![addr]);

        let eth = contract_address_from_subnet(&id).unwrap();
        assert_eq!(
            format!("{eth:?}"),
            "0x2e714a3c385ea88a09998ed74db265dae9853667"
        );
    }

    /// test case that makes sure the commitment created for various addresses and blocks committed
    /// are consistent
    #[test]
    fn test_validator_rewarder_claim_commitment() {
        let pack_validator_data = |v: &checkpointing_facet::ValidatorData| {
            vec![format!("{:?}", v.validator), v.blocks_committed.to_string()]
        };

        let mut random_validator_data = vec![
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0xB29C00299756135ec5d6A140CA54Ec77790a99d6",
                )
                .unwrap(),
                blocks_committed: 1,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x1A79385eAd0e873FE0C441C034636D3Edf7014cC",
                )
                .unwrap(),
                blocks_committed: 10,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x28345a43c2fBae4412f0AbadFa06Bd8BA3f58867",
                )
                .unwrap(),
                blocks_committed: 2,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x3c5cc76b07cb02a372e647887bD6780513659527",
                )
                .unwrap(),
                blocks_committed: 3,
            },
            ValidatorData {
                validator: ethers::types::Address::from_str(
                    "0x76B9d5a35C46B1fFEb37aadf929f1CA63a26A829",
                )
                .unwrap(),
                blocks_committed: 4,
            },
        ];
        random_validator_data.shuffle(&mut thread_rng());

        let root = MerkleGen::new(
            pack_validator_data,
            &random_validator_data,
            &VALIDATOR_REWARD_FIELDS,
        )
        .unwrap()
        .root();
        assert_eq!(
            hex::encode(root.0),
            "5519955f33109df3338490473cb14458640efdccd4df05998c4c439738280ab0"
        );
    }

    #[test]
    fn test_validator_rewarder_claim_commitment_ii() {
        let pack_validator_data = |v: &checkpointing_facet::ValidatorData| {
            vec![format!("{:?}", v.validator), v.blocks_committed.to_string()]
        };

        let mut random_validator_data = (0..100)
            .map(|_| ValidatorData {
                validator: ethers::types::Address::random(),
                blocks_committed: random::<u64>(),
            })
            .collect::<Vec<_>>();

        random_validator_data.shuffle(&mut thread_rng());
        let root = MerkleGen::new(
            pack_validator_data,
            &random_validator_data,
            &VALIDATOR_REWARD_FIELDS,
        )
        .unwrap()
        .root();

        random_validator_data.shuffle(&mut thread_rng());
        let new_root = MerkleGen::new(
            pack_validator_data,
            &random_validator_data,
            &VALIDATOR_REWARD_FIELDS,
        )
        .unwrap()
        .root();
        assert_eq!(new_root, root);

        random_validator_data.shuffle(&mut thread_rng());
        let new_root = MerkleGen::new(
            pack_validator_data,
            &random_validator_data,
            &VALIDATOR_REWARD_FIELDS,
        )
        .unwrap()
        .root();
        assert_eq!(new_root, root);
    }
}

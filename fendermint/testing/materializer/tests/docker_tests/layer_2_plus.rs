// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use anyhow::{anyhow, bail, Context};
use ethers::contract::{ContractError, ContractRevert};
use ethers::core::types as et;
use ethers::middleware::gas_oracle::middleware;
use ethers::types::transaction::response;
use ethers::types::{Filter, U256};
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, JsonRpcClient, Middleware, PendingTransaction, Provider},
    signers::{Signer, Wallet},
    types::{transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, H160, H256},
};

use ethers_contract::EthLogDecode;
use futures::FutureExt;
use ipc_provider::IpcProvider;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::event;

use fendermint_materializer::{HasEthApi, ResourceId};
use fendermint_vm_actor_interface::init::builtin_actor_eth_addr;
use fendermint_vm_actor_interface::ipc;
use fendermint_vm_message::conv::from_fvm::to_eth_address;
use ipc_actors_abis::gateway_getter_facet::{GatewayGetterFacet, ParentFinality};
use ipc_actors_abis::gateway_manager_facet::{GatewayManagerFacet, GatewayManagerFacetErrors};
use ipc_actors_abis::gateway_messenger_facet::{
    self, GatewayMessengerFacet, GatewayMessengerFacetErrors, GatewayMessengerFacetEvents,
    NewTopDownMessageFilter,
};

use ipc_actors_abis::subnet_actor_getter_facet::{SubnetActorGetterFacet, FvmAddress as FvmAddress2};

use ipc_actors_abis::cross_messenger::{
    self, CrossMessenger, FvmAddress, Ipcaddress, SubnetID as IPCSubnetID,
};

use fendermint_vm_message::{conv::from_eth::to_fvm_address};

use fvm_shared::econ::TokenAmount;

use ipc_api::address::IPCAddress;
use ipc_api::cross::{IpcEnvelope, IpcMsgKind};
use ipc_api::evm;
use ipc_api::subnet_id::SubnetID;
use ipc_api::ethers_address_to_fil_address;

use crate::with_testnet;

const MANIFEST: &str = "layer3.yaml";
const CHECKPOINT_PERIOD: u64 = 10;
const SLEEP_SECS: u64 = 5;
const MAX_RETRIES: u32 = 10;

use fvm_shared::address::{Address, Payload};

/// Convert the ipc SubnetID type to a vec of evm addresses. It extracts all the children addresses
/// in the subnet id and turns them as a vec of evm addresses.
pub fn subnet_id_to_evm_addresses(
    subnet: &SubnetID,
) -> anyhow::Result<Vec<ethers::types::Address>> {
    let children = subnet.children();
    children
        .iter()
        .map(|addr| payload_to_evm_address(addr.payload()))
        .collect::<anyhow::Result<_>>()
}

/// Util function to convert Fil address payload to evm address. Only delegated address is supported.
pub fn payload_to_evm_address(payload: &Payload) -> anyhow::Result<ethers::types::Address> {
    match payload {
        Payload::Delegated(delegated) => {
            let slice = delegated.subaddress();
            Ok(ethers::types::Address::from_slice(&slice[0..20]))
        }
        _ => Err(anyhow!("address provided is not delegated")),
    }
}

use fendermint_materializer::{manifest::Rootnet, materials::DefaultAccount};

pub type TestMiddleware<C> = SignerMiddleware<Provider<C>, Wallet<SigningKey>>;

async fn make_middleware<C>(
    provider: Provider<C>,
    sender: &DefaultAccount,
) -> anyhow::Result<TestMiddleware<C>>
where
    C: JsonRpcClient,
{
    let chain_id = provider
        .get_chainid()
        .await
        .context("failed to get chain ID")?;

    let wallet: Wallet<SigningKey> = Wallet::from_bytes(sender.secret_key().serialize().as_ref())?
        .with_chain_id(chain_id.as_u64());

    Ok(SignerMiddleware::new(provider, wallet))
}

#[serial_test::serial]
#[tokio::test]
async fn test_jedu_rajenka() {
    // cast logs -r "http://localhost:30945" --address 0x77aa40b105843728088c0132e43fc44348881da8
    let root_provider = Arc::new(Provider::<Http>::try_from("http://localhost:30845").unwrap());
    let england_provider = Arc::new(Provider::<Http>::try_from("http://localhost:30945").unwrap());
    let contract_address = H160::from_str("0x77aa40b105843728088c0132e43fc44348881da8").unwrap();

    let england_gateway_getter = GatewayGetterFacet::new(contract_address, england_provider.clone());

    // let finality: ParentFinality = england_gateway_getter
    //     .get_latest_parent_finality()
    //     .call()
    //     .await
    //     .unwrap();

    // let block_hash = hex::encode(finality.block_hash);
    // println!("England finality: {:?} {:?}", finality.height, block_hash);

    let england_gateway_messenger = GatewayMessengerFacet::new(
        contract_address,
        england_provider.clone(),
    );

    let filter = Filter::new().address(contract_address);

    // Query the logs
    let logs = england_provider.get_logs(&filter).await.unwrap();

    // Process each log by parsing it into the event struct
    for log in logs {
        println!("Historical Event: {:?}", log);
    }

    // let events = england_gateway_messenger
    //     .events()
    //     .from_block(1)
    //     .to_block(ethers::types::BlockNumber::Latest)
    //     .query()
    //     .await
    //     .unwrap();

    // println!("---- Events: {:?}", events.len());

    // events.iter().for_each(|event| {
    //     println!("---- Event: {:?}", event);
    // });

    // let root_gateway_messenger = GatewayMessengerFacet::new(
    //     contract_address,
    //     root_provider.clone(),
    // );

    // let events: Vec<NewTopDownMessageFilter> = root_gateway_messenger
    //     .new_top_down_message_filter()
    //     .from_block(1)
    //     .to_block(ethers::types::BlockNumber::Latest)
    //     .query()
    //     .await
    //     .unwrap();

    // println!("---- Events: {:?}", events.len());

    // events.iter().for_each(|event| {
    //     println!("---- Event: {:?}", event);
    // });
}

#[serial_test::serial]
#[tokio::test]
async fn test_cross_messages() {
    with_testnet(
        MANIFEST,
        |manifest| {
            // Try to make sure the bottom-up checkpoint period is quick enough for reasonable test runtime.
            let subnet = manifest
                .subnets
                .get_mut(&ResourceId::from("england"))
                .expect("subnet not found");

            subnet.bottom_up_checkpoint.period = CHECKPOINT_PERIOD;
        },
        |_, _, testnet| {
            let test = async {
                let england = testnet.subnet(&testnet.root().subnet("england"))?;

                let _bob = testnet.account("bob")?;

                let will = testnet.account("will")?;

                let england_subnet = testnet.subnet(&testnet.root().subnet("england"))?;
                let england_london_node =
                    testnet.node(&testnet.root().subnet("england").node("london"))?;

                let oxfordshire_subnet =
                    testnet.subnet(&testnet.root().subnet("england").subnet("oxfordshire"))?;

                let oxfordshire_oxford_node = testnet.node(
                    &testnet
                        .root()
                        .subnet("england")
                        .subnet("oxfordshire")
                        .node("oxford"),
                )?;

                let cross_messenger_contract_name = "cross_messenger".to_string();

                let root_net = testnet.subnet(&testnet.root())?;
                let root_brussels_node = testnet.node(&testnet.root().node("brussels"))?;
                let root_provider = root_brussels_node
                    .ethapi_http_provider()?
                    .expect("ethapi should be enabled");
                
                let root_provider_signer = Arc::new(
                    make_middleware(root_provider, will)
                        .await
                        .context("failed to set up signer middleware")?,
                );
    
                let root_cross_messenger_deployment = testnet
                .solidity_deployment(&root_net.name, cross_messenger_contract_name)?;

                let root_cross_messenger = CrossMessenger::new(
                    root_cross_messenger_deployment.address,
                    root_provider_signer.clone(),
                );

                root_cross_messenger
                    .set_gateway_address(builtin_actor_eth_addr(ipc::GATEWAY_ACTOR_ID).into())
                    .send()
                    .await?
                    .await?
                    .expect("set gateway address failed");

                let root_network = testnet.subnet(&testnet.root())?;
                let root_id = root_network.subnet_id.root_id();
                
                let addr = ethers::types::Address::from_str("0xc0ffee254729296a45a3885639AC7E10F9d54979")?;
                let fil_addr = ethers_address_to_fil_address(&addr)?;
                let fvm_addr_2 = FvmAddress2::try_from(fil_addr)?;

                let invoke_cross_message_call = root_cross_messenger.invoke_cross_message(
                    Ipcaddress {
                        subnet_id: IPCSubnetID {
                            root: root_id,
                            route: subnet_id_to_evm_addresses(&root_net.subnet_id)?,
                        },
                        raw_address: FvmAddress{
                            addr_type: fvm_addr_2.addr_type, // TODO Karel - add the cross message address and type
                            payload: fvm_addr_2.payload.clone(),
                        },
                    },
                    Ipcaddress {
                        subnet_id: IPCSubnetID {
                            root: root_id,
                            route: subnet_id_to_evm_addresses(&oxfordshire_subnet.subnet_id)?,
                        },
                        raw_address: FvmAddress{
                            addr_type: fvm_addr_2.addr_type,
                            payload: fvm_addr_2.payload,
                        },
                    },
                );

                let invoke_cross_message_receipt = invoke_cross_message_call
                    .value(3)
                    .send()
                    .await?
                    .await?
                    .expect("invoke cross message failed");
                
                println!(
                    "Invoke cross message response: {:?}",
                    invoke_cross_message_receipt
                );
                
                let gateway_address = builtin_actor_eth_addr(ipc::GATEWAY_ACTOR_ID);

                let london_provider = Arc::new(
                    england_london_node
                        .ethapi_http_provider()?
                        .expect("ethapi should be enabled"),
                );

                let london_subnet_gateway_getter =
                    GatewayGetterFacet::new(gateway_address, london_provider.clone());
                
                let london_gateway_messenger = GatewayMessengerFacet::new(
                    gateway_address,
                    london_provider.clone(),
                );

                let invoked_on_height = invoke_cross_message_receipt.block_number.unwrap().as_u64();

                // Query the latest committed parent finality and compare to the parent.
                {
                    let mut retry = 0;
                    loop {
                        let finality: ParentFinality = london_subnet_gateway_getter
                            .get_latest_parent_finality()
                            .call()
                            .await
                            .context("failed to get parent finality")?;

                        let finality_height = finality.height.as_u64();

                        // If the latest finality is not zero it means the syncer is working,
                        if finality_height == 0 || finality_height < invoked_on_height {
                            if retry < MAX_RETRIES {
                                eprintln!("waiting for syncing with the parent... finality_height: {}, invoked_on_height: {}", finality_height, invoked_on_height);
                                tokio::time::sleep(Duration::from_secs(SLEEP_SECS)).await;
                                retry += 1;
                                continue;
                            }
                            bail!("the parent finality is still zero");
                        }

                        // Check that the block hash of the parent is actually the same at that height.
                        let parent_block: Option<et::Block<_>> = root_provider_signer
                            .get_block(finality.height.as_u64())
                            .await
                            .context("failed to get parent block")?;

                        let Some(parent_block_hash) = parent_block.and_then(|b| b.hash) else {
                            bail!("cannot find parent block at final height");
                        };

                        if parent_block_hash.0 != finality.block_hash {
                            bail!("the finality block hash is different from the API");
                        }

                        let events: Vec<NewTopDownMessageFilter> = london_gateway_messenger
                            .new_top_down_message_filter()
                            .from_block(1)
                            .to_block(finality_height)
                            .query()
                            .await
                            .unwrap();

                        println!("---- Events: {:?}", events.len());

                        events.iter().for_each(|event| {
                            println!("---- Event: {:?}", event);
                        });
                        break;
                    }
                }


                let events = london_gateway_messenger
                    .events()
                    .from_block(1)
                    .to_block(ethers::types::BlockNumber::Latest)
                    .query()
                    .await
                    .unwrap();

                println!("---- after Events: {:?}", events.len());

                events.iter().for_each(|event| {
                    println!("---- after Event: {:?}", event);
                });

                // let parent_provider = Arc::new(
                //     brussels
                //         .ethapi_http_provider()?
                //         .ok_or_else(|| anyhow!("ethapi should be enabled"))?,
                // );

                // let _parent_gateway_mesenger = GatewayMessengerFacet::new(
                //     builtin_actor_eth_addr(ipc::GATEWAY_ACTOR_ID),
                //     parent_provider.clone(),
                // );

                // Gateway actor on the child
                // let subnet_gateway_getter = GatewayGetterFacet::new(
                //     builtin_actor_eth_addr(ipc::GATEWAY_ACTOR_ID),
                //     london_provider.clone(),
                // );

                // // Subnet actor on the parent
                // let subnet_actor_getter = SubnetActorGetterFacet::new(
                //     to_eth_address(&england.subnet_id.subnet_actor())
                //         .and_then(|a| a.ok_or_else(|| anyhow!("not an eth address")))?,
                //     parent_provider.clone(),
                // );

                // // Check that the parent knows about a checkpoint submitted from the child.
                // {
                //     let mut retry = 0;
                //     loop {
                //         // NOTE: The implementation of the following method seems like a nonsense;
                //         //       I don't know if there is a way to ask the gateway what the latest
                //         //       checkpoint is, so we'll just have to go to the parent directly.
                //         // let (has_checkpoint, epoch, _): (bool, et::U256, _) = england_gateway
                //         //     .get_current_bottom_up_checkpoint()
                //         //     .call()
                //         //     .await
                //         //     .context("failed to get current bottomup checkpoint")?;
                //         let ckpt_height: et::U256 = subnet_actor_getter
                //             .last_bottom_up_checkpoint_height()
                //             .call()
                //             .await
                //             .context("failed to query last checkpoint height")?;

                //         if !ckpt_height.is_zero() {
                //             break;
                //         }

                //         if retry < MAX_RETRIES {
                //             eprintln!("waiting for a checkpoint to be submitted...");
                //             tokio::time::sleep(Duration::from_secs(SLEEP_SECS)).await;
                //             retry += 1;
                //             continue;
                //         }

                //         bail!("hasn't submitted a bottom-up checkpoint");
                //     }
                // }

                Ok(())
            };

            test.boxed_local()
        },
    )
    .await
    .unwrap()
}

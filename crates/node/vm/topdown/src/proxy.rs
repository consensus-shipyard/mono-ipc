// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::observe::ParentRpcCalled;
use crate::BlockHeight;
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use fvm_shared::clock::ChainEpoch;
use ipc_api::cross::IpcEnvelope;
use ipc_api::staking::StakingChangeRequest;
use ipc_api::subnet_id::SubnetID;
use ipc_observability::emit;
use ipc_provider::manager::{GetBlockHashResult, TopDownQueryPayload};
use ipc_provider::IpcProvider;
use std::time::Instant;
use tracing::instrument;

/// The interface to querying state of the parent
#[async_trait]
pub trait ParentQueryProxy {
    /// Get the parent chain head block number or block height
    async fn get_chain_head_height(&self) -> anyhow::Result<BlockHeight>;

    /// Get the genesis epoch of the child subnet, i.e. the epoch that the subnet was created in
    /// the parent subnet.
    async fn get_genesis_epoch(&self) -> anyhow::Result<BlockHeight>;

    /// Getting the block hash at the target height.
    async fn get_block_hash(&self, height: BlockHeight) -> anyhow::Result<GetBlockHashResult>;

    /// Get the top down messages at epoch with the block hash at that height
    async fn get_top_down_msgs(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<TopDownQueryPayload<Vec<IpcEnvelope>>>;

    /// Get the validator set at the specified height
    async fn get_validator_changes(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<TopDownQueryPayload<Vec<StakingChangeRequest>>>;
}

/// The proxy to the subnet's parent
pub struct IPCProviderProxy {
    ipc_provider: IpcProvider,
    /// The parent subnet for the child subnet we are target. We can derive from child subnet,
    /// but storing it separately so that we dont have to derive every time.
    parent_subnet: SubnetID,
    /// The child subnet that this node belongs to.
    child_subnet: SubnetID,
}

impl IPCProviderProxy {
    pub fn new(ipc_provider: IpcProvider, target_subnet: SubnetID) -> anyhow::Result<Self> {
        let parent = target_subnet
            .parent()
            .ok_or_else(|| anyhow!("subnet does not have parent"))?;
        Ok(Self {
            ipc_provider,
            parent_subnet: parent,
            child_subnet: target_subnet,
        })
    }
}

#[async_trait]
impl ParentQueryProxy for IPCProviderProxy {
    async fn get_chain_head_height(&self) -> anyhow::Result<BlockHeight> {
        let height = self.ipc_provider.chain_head(&self.parent_subnet).await?;
        Ok(height as BlockHeight)
    }

    /// Get the genesis epoch of the child subnet, i.e. the epoch that the subnet was created in
    /// the parent subnet.
    async fn get_genesis_epoch(&self) -> anyhow::Result<BlockHeight> {
        let height = self.ipc_provider.genesis_epoch(&self.child_subnet).await?;
        Ok(height as BlockHeight)
    }

    /// Getting the block hash at the target height.
    async fn get_block_hash(&self, height: BlockHeight) -> anyhow::Result<GetBlockHashResult> {
        self.ipc_provider
            .get_block_hash(&self.parent_subnet, height as ChainEpoch)
            .await
    }

    /// Get the top down messages from the starting to the ending height.
    async fn get_top_down_msgs(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<TopDownQueryPayload<Vec<IpcEnvelope>>> {
        self.ipc_provider
            .get_top_down_msgs(&self.child_subnet, height as ChainEpoch)
            .await
            .map(|mut v| {
                // sort ascending, we dont assume the changes are ordered
                v.value.sort_by(|a, b| a.local_nonce.cmp(&b.local_nonce));
                v
            })
    }

    /// Get the validator set at the specified height.
    async fn get_validator_changes(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<TopDownQueryPayload<Vec<StakingChangeRequest>>> {
        self.ipc_provider
            .get_validator_changeset(&self.child_subnet, height as ChainEpoch)
            .await
            .map(|mut v| {
                // sort ascending, we dont assume the changes are ordered
                v.value
                    .sort_by(|a, b| a.configuration_number.cmp(&b.configuration_number));
                v
            })
    }
}

// TODO - create a macro for this
pub struct IPCProviderProxyWithLatency {
    inner: IPCProviderProxy,
}

impl IPCProviderProxyWithLatency {
    pub fn new(inner: IPCProviderProxy) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl ParentQueryProxy for IPCProviderProxyWithLatency {
    #[instrument(skip(self))]
    async fn get_chain_head_height(&self) -> anyhow::Result<BlockHeight> {
        emit_event_with_latency(
            &self.inner.parent_subnet.to_string(),
            "chain_head",
            || async { self.inner.get_chain_head_height().await },
        )
        .await
    }

    #[instrument(skip(self))]
    async fn get_genesis_epoch(&self) -> anyhow::Result<BlockHeight> {
        emit_event_with_latency(
            &self.inner.parent_subnet.to_string(),
            "genesis_epoch",
            || async { self.inner.get_genesis_epoch().await },
        )
        .await
    }

    #[instrument(skip(self))]
    async fn get_block_hash(&self, height: BlockHeight) -> anyhow::Result<GetBlockHashResult> {
        emit_event_with_latency(
            &self.inner.parent_subnet.to_string(),
            "get_block_hash",
            || async { self.inner.get_block_hash(height).await },
        )
        .await
    }

    #[instrument(skip(self))]
    async fn get_top_down_msgs(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<TopDownQueryPayload<Vec<IpcEnvelope>>> {
        emit_event_with_latency(
            &self.inner.parent_subnet.to_string(),
            "get_top_down_msgs",
            || async { self.inner.get_top_down_msgs(height).await },
        )
        .await
    }

    #[instrument(skip(self))]
    async fn get_validator_changes(
        &self,
        height: BlockHeight,
    ) -> anyhow::Result<TopDownQueryPayload<Vec<StakingChangeRequest>>> {
        emit_event_with_latency(
            &self.inner.parent_subnet.to_string(),
            "get_validator_changeset",
            || async { self.inner.get_validator_changes(height).await },
        )
        .await
    }
}

// TODO Karel - make it nicer. Perhaps use a macro?
async fn emit_event_with_latency<F, Fut, T>(json_rpc: &str, method: &str, func: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let start = Instant::now();
    let result = func().await;
    let latency = start.elapsed().as_secs_f64();

    emit(ParentRpcCalled {
        source: "IPC Provider Proxy",
        json_rpc,
        method,
        latency,
        status: match &result {
            Ok(_) => "success",
            Err(_) => "error",
        },
    });

    result
}

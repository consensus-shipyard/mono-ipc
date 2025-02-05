// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT

use async_trait::async_trait;
use ethers::{
    core::types::{transaction::eip2718::TypedTransaction, BlockId, U256},
    providers::{Middleware, MiddlewareError, PendingTransaction, ProviderError},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Eip1559GasEstimatorError<M: Middleware> {
    #[error("{0}")]
    MiddlewareError(M::Error),
    #[error("{0}")]
    FailedToEstimateGas(String),
}

impl<M: Middleware> Eip1559GasEstimatorError<M> {
    pub fn failed_to_estimate_gas_not_supported() -> Self {
        Eip1559GasEstimatorError::FailedToEstimateGas(
            "Only EIP-1559 transactions are supported".to_string(),
        )
    }
}

impl<M: Middleware> MiddlewareError for Eip1559GasEstimatorError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> Self {
        Eip1559GasEstimatorError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            Eip1559GasEstimatorError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}

pub struct GasFeeEstimate {
    pub max_priority_fee_per_gas: U256,
    pub max_fee_per_gas: U256,
}

#[derive(Debug)]
pub struct Eip1559GasEstimatorMiddleware<M: Middleware> {
    inner: M,
}

impl<M: Middleware> Eip1559GasEstimatorMiddleware<M> {
    pub fn new(inner: M) -> Self {
        Self { inner }
    }

    pub async fn max_priority_fee_per_gas(&self) -> Result<U256, ProviderError> {
        self.inner
            .provider()
            .request("eth_maxPriorityFeePerGas", json!([]))
            .await
    }

    pub async fn base_fee_per_gas(&self) -> Result<U256, ProviderError> {
        let latest_block = self
            .inner
            .provider()
            .get_block(ethers::types::BlockNumber::Latest)
            .await?
            .ok_or_else(|| ProviderError::CustomError("Latest block not found".to_string()))?;

        latest_block
            .base_fee_per_gas
            .ok_or_else(|| ProviderError::CustomError("EIP-1559 not activated".to_string()))
    }

    pub async fn estimate_gas_fees(&self) -> Result<GasFeeEstimate, Eip1559GasEstimatorError<M>> {
        let max_priority_fee_per_gas = self
            .max_priority_fee_per_gas()
            .await
            .map_err(|e| Eip1559GasEstimatorError::FailedToEstimateGas(e.to_string()))?;
        let base_fee_per_gas = self
            .base_fee_per_gas()
            .await
            .map_err(|e| Eip1559GasEstimatorError::FailedToEstimateGas(e.to_string()))?;

        // Buffer the base fee by multiplying by 2 to account for potential cumulative increases.
        let base_fee_per_gas_surged = base_fee_per_gas * 2;
        let max_fee_per_gas = max_priority_fee_per_gas + base_fee_per_gas_surged;

        Ok(GasFeeEstimate {
            max_priority_fee_per_gas,
            max_fee_per_gas,
        })
    }
}

#[async_trait]
impl<M: Middleware> Middleware for Eip1559GasEstimatorMiddleware<M> {
    type Error = Eip1559GasEstimatorError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.inner
    }

    /// Fills the transaction with EIP-1559 gas fees.
    async fn fill_transaction(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), Self::Error> {
        if let TypedTransaction::Eip1559(inner) = tx {
            let gas_fees = self.estimate_gas_fees().await?;

            // Set the gas fees directly on the transaction.
            let tx_req = inner
                .clone()
                .max_fee_per_gas(gas_fees.max_fee_per_gas)
                .max_priority_fee_per_gas(gas_fees.max_priority_fee_per_gas);

            *tx = TypedTransaction::Eip1559(tx_req);
        } else {
            return Err(Eip1559GasEstimatorError::failed_to_estimate_gas_not_supported());
        }

        // Delegate to the inner middleware for filling remaining transaction fields.
        self.inner()
            .fill_transaction(tx, block)
            .await
            .map_err(Eip1559GasEstimatorError::MiddlewareError)
    }

    /// Sends a transaction with EIP-1559 gas fees.
    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        let mut tx = tx.into();

        // Automatically fill EIP-1559 gas fees if they are not already set.
        if let TypedTransaction::Eip1559(ref mut inner) = tx {
            if inner.max_fee_per_gas.is_none() || inner.max_priority_fee_per_gas.is_none() {
                // Populate missing gas fees with `fill_transaction`.
                self.fill_transaction(&mut tx, block).await?;
            }
        } else {
            return Err(Eip1559GasEstimatorError::failed_to_estimate_gas_not_supported());
        }

        // Proceed to send the transaction with the inner middleware.
        self.inner()
            .send_transaction(tx, block)
            .await
            .map_err(Eip1559GasEstimatorError::MiddlewareError)
    }
}

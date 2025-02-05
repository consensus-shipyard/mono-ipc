// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::fvm::FvmMessage;
use anyhow::{bail, Context};

use fendermint_actors_api::gas_market::{Gas, Reading, Utilization};
use fendermint_vm_actor_interface::gas_market::GAS_MARKET_ACTOR_ADDR;
use fendermint_vm_actor_interface::{reward, system};
use fvm::executor::{ApplyKind, ApplyRet, Executor};
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::METHOD_SEND;
use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct BlockGasTracker {
    /// The current base fee.
    base_fee: TokenAmount,
    /// The current block gas limit.
    block_gas_limit: Gas,
    /// The cumulative gas premiums claimable by the block producer.
    cumul_gas_premium: TokenAmount,
    /// The accumulated gas usage throughout the block.
    cumul_gas_used: Gas,
}

impl BlockGasTracker {
    pub fn base_fee(&self) -> &TokenAmount {
        &self.base_fee
    }

    pub fn create<E: Executor>(executor: &mut E) -> anyhow::Result<BlockGasTracker> {
        let mut ret = Self {
            base_fee: Zero::zero(),
            block_gas_limit: Zero::zero(),
            cumul_gas_premium: Zero::zero(),
            cumul_gas_used: Zero::zero(),
        };

        let reading = Self::read_gas_market(executor)?;

        ret.base_fee = reading.base_fee;
        ret.block_gas_limit = reading.block_gas_limit;

        Ok(ret)
    }

    pub fn available(&self) -> Gas {
        self.block_gas_limit.saturating_sub(self.cumul_gas_used)
    }

    pub fn ensure_sufficient_gas(&self, msg: &FvmMessage) -> anyhow::Result<()> {
        let available_gas = self.available();
        if msg.gas_limit > available_gas {
            bail!("message gas limit exceed available block gas limit; consensus engine may be misbehaving; txn gas limit: {}, block gas available: {}",
                msg.gas_limit,
                available_gas
            );
        }
        Ok(())
    }

    pub fn record_utilization(&mut self, ret: &ApplyRet) {
        self.cumul_gas_premium += ret.miner_tip.clone();
        self.cumul_gas_used = self.cumul_gas_used.saturating_add(ret.msg_receipt.gas_used);

        // sanity check, should not happen; only trace if it does so we can debug later.
        if self.cumul_gas_used >= self.block_gas_limit {
            tracing::warn!("out of block gas; cumulative gas used exceeds block gas limit!");
        }
    }

    pub fn finalize<E: Executor>(
        &self,
        executor: &mut E,
        premium_recipient: Option<Address>,
    ) -> anyhow::Result<Reading> {
        if let Some(premium_recipient) = premium_recipient {
            self.distribute_premiums(executor, premium_recipient)?
        }
        self.commit_utilization(executor)
    }

    pub fn read_gas_market<E: Executor>(executor: &mut E) -> anyhow::Result<Reading> {
        let msg = FvmMessage {
            from: system::SYSTEM_ACTOR_ADDR,
            to: GAS_MARKET_ACTOR_ADDR,
            sequence: 0, // irrelevant for implicit executions.
            gas_limit: i64::MAX as u64,
            method_num: fendermint_actors_api::gas_market::Method::CurrentReading as u64,
            params: fvm_ipld_encoding::RawBytes::default(),
            value: Default::default(),
            version: Default::default(),
            gas_fee_cap: Default::default(),
            gas_premium: Default::default(),
        };

        let apply_ret = Self::apply_implicit_message(executor, msg)?;

        if let Some(err) = apply_ret.failure_info {
            bail!("failed to acquire gas market reading: {}", err);
        }

        fvm_ipld_encoding::from_slice::<Reading>(&apply_ret.msg_receipt.return_data)
            .context("failed to parse gas market reading")
    }

    fn commit_utilization<E: Executor>(&self, executor: &mut E) -> anyhow::Result<Reading> {
        let params = fvm_ipld_encoding::RawBytes::serialize(Utilization {
            block_gas_used: self.cumul_gas_used,
        })?;

        let msg = FvmMessage {
            from: system::SYSTEM_ACTOR_ADDR,
            to: GAS_MARKET_ACTOR_ADDR,
            sequence: 0, // irrelevant for implicit executions.
            gas_limit: i64::MAX as u64,
            method_num: fendermint_actors_api::gas_market::Method::UpdateUtilization as u64,
            params,
            value: Default::default(),
            version: Default::default(),
            gas_fee_cap: Default::default(),
            gas_premium: Default::default(),
        };

        let apply_ret = Self::apply_implicit_message(executor, msg)?;
        fvm_ipld_encoding::from_slice::<Reading>(&apply_ret.msg_receipt.return_data)
            .context("failed to parse gas utilization result")
    }

    fn distribute_premiums<E: Executor>(
        &self,
        executor: &mut E,
        premium_recipient: Address,
    ) -> anyhow::Result<()> {
        if self.cumul_gas_premium.is_zero() {
            return Ok(());
        }

        let msg = FvmMessage {
            from: reward::REWARD_ACTOR_ADDR,
            to: premium_recipient,
            sequence: 0, // irrelevant for implicit executions.
            gas_limit: i64::MAX as u64,
            method_num: METHOD_SEND,
            params: fvm_ipld_encoding::RawBytes::default(),
            value: self.cumul_gas_premium.clone(),
            version: Default::default(),
            gas_fee_cap: Default::default(),
            gas_premium: Default::default(),
        };
        Self::apply_implicit_message(executor, msg)?;

        Ok(())
    }

    fn apply_implicit_message<E: Executor>(
        executor: &mut E,
        msg: FvmMessage,
    ) -> anyhow::Result<ApplyRet> {
        let apply_ret = executor.execute_message(msg, ApplyKind::Implicit, 0)?;
        if let Some(err) = apply_ret.failure_info {
            bail!("failed to apply message: {}", err)
        }
        Ok(apply_ret)
    }
}

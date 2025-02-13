// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::fvm::activity::{FullActivity, ValidatorActivityTracker};
use crate::fvm::state::FvmExecState;
use crate::fvm::FvmMessage;
use anyhow::Context;
use fendermint_actor_activity_tracker::types::FullActivityRollup;
use fendermint_crypto::PublicKey;
use fendermint_vm_actor_interface::activity::ACTIVITY_TRACKER_ACTOR_ADDR;
use fendermint_vm_actor_interface::eam::EthAddress;
use fendermint_vm_actor_interface::system;
use fvm_ipld_blockstore::Blockstore;
use fvm_shared::address::Address;

pub struct ActorActivityTracker<'a, DB: Blockstore + Clone + 'static> {
    pub(crate) executor: &'a mut FvmExecState<DB>,
}

impl<'a, DB: Blockstore + Clone + 'static> ValidatorActivityTracker
    for ActorActivityTracker<'a, DB>
{
    fn record_block_committed(&mut self, validator: PublicKey) -> anyhow::Result<()> {
        let address: Address = EthAddress::from(validator).into();

        let msg = FvmMessage {
            from: system::SYSTEM_ACTOR_ADDR,
            to: ACTIVITY_TRACKER_ACTOR_ADDR,
            sequence: 0,                // irrelevant
            gas_limit: i64::MAX as u64, // exclude this from gas restriction
            method_num: fendermint_actor_activity_tracker::Method::RecordBlockCommitted as u64,
            params: fvm_ipld_encoding::RawBytes::serialize(address)?,
            value: Default::default(),
            version: Default::default(),
            gas_fee_cap: Default::default(),
            gas_premium: Default::default(),
        };

        self.executor.execute_implicit_ok(msg)?;
        Ok(())
    }

    fn commit_activity(&mut self) -> anyhow::Result<FullActivity> {
        let msg = FvmMessage {
            from: system::SYSTEM_ACTOR_ADDR,
            to: ACTIVITY_TRACKER_ACTOR_ADDR,
            sequence: 0,                // irrelevant
            gas_limit: i64::MAX as u64, // exclude this from gas restriction
            method_num: fendermint_actor_activity_tracker::Method::CommitActivity as u64,
            params: fvm_ipld_encoding::RawBytes::default(),
            value: Default::default(),
            version: Default::default(),
            gas_fee_cap: Default::default(),
            gas_premium: Default::default(),
        };

        let (apply_ret, _) = self.executor.execute_implicit_ok(msg)?;
        let r =
            fvm_ipld_encoding::from_slice::<FullActivityRollup>(&apply_ret.msg_receipt.return_data)
                .context("failed to parse validator activities")?;
        r.try_into()
    }
}

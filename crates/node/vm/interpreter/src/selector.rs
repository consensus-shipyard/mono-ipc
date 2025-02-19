// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

//! Gas related message selection

use crate::fvm::state::FvmExecState;
use fendermint_vm_message::signed::SignedMessage;
use fvm_ipld_blockstore::Blockstore;

/// Implement this trait to perform message selection
pub trait MessageSelector {
    fn select_messages<DB: Blockstore + Clone + 'static>(
        &self,
        state: &FvmExecState<DB>,
        msgs: Vec<SignedMessage>,
    ) -> Vec<SignedMessage>;
}

pub(crate) struct GasLimitSelector;

impl MessageSelector for GasLimitSelector {
    fn select_messages<DB: Blockstore + Clone + 'static>(
        &self,
        state: &FvmExecState<DB>,
        mut msgs: Vec<SignedMessage>,
    ) -> Vec<SignedMessage> {
        let total_gas_limit = state.block_gas_tracker().available();

        // Sort by gas limit descending
        msgs.sort_by(|a, b| b.message.gas_limit.cmp(&a.message.gas_limit));

        let mut total_gas_limit_consumed = 0;
        msgs.into_iter()
            .take_while(|msg| {
                let gas_limit = msg.message.gas_limit;
                let accepted = total_gas_limit_consumed + gas_limit <= total_gas_limit;
                if accepted {
                    total_gas_limit_consumed += gas_limit;
                }
                accepted
            })
            .collect()
    }
}

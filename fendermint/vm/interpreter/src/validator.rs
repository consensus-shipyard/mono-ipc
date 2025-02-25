// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::chain::ChainMessageApplyRet;
use crate::fvm::state::FvmExecState;
use crate::fvm::FvmApplyRet;
use crate::signed::InvalidSignature;
use ethers::contract::EthCall;
use fendermint_vm_message::signed::SignedMessage;
use fvm::executor::{default_gas_hook, ApplyKind, ExecutionOptions};
use fvm::gas::GasOutputs;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_encoding::BytesDe;
use fvm_shared::receipt::Receipt;
use fvm_shared::ActorID;

const SOLIDITY_SELECTOR_BYTES: usize = 4;

/// Executes the add bottom up checkpoint signature call from a validator.
pub(crate) fn execute_bottom_up_signature<DB: Blockstore + Clone + 'static>(
    state: &mut FvmExecState<DB>,
    signed: SignedMessage,
) -> anyhow::Result<ChainMessageApplyRet> {
    let chain_id = state.chain_id();
    if let Err(e) = signed.verify(&chain_id) {
        return Ok(ChainMessageApplyRet::Signed(Err(
            crate::signed::InvalidSignature(e.to_string()),
        )));
    }

    // technically we need to check the sender is actually a validator,
    // but the contract is checking it, delegates to the contract.
    let domain_hash = signed.domain_hash(&chain_id)?;
    let msg = signed.message;

    // check the message signature matches, that means we are actually calling submit signature
    let method_selector =
        ipc_actors_abis::checkpointing_facet::AddCheckpointSignatureCall::selector();
    let calldata = msg.params.deserialize::<BytesDe>()?.0;

    if calldata.len() < SOLIDITY_SELECTOR_BYTES
        || method_selector != calldata[0..SOLIDITY_SELECTOR_BYTES]
    {
        return Ok(ChainMessageApplyRet::Signed(Err(InvalidSignature(
            "not calling submitting bottom up signature".to_string(),
        ))));
    }

    let custom_hook = |sender_id: ActorID, receipt: &Receipt, gas_output: &GasOutputs| {
        // validator message execution ok, refund all gas to the validator
        if receipt.exit_code.is_success() {
            let total = &gas_output.base_fee_burn
                + &gas_output.miner_tip
                + &gas_output.over_estimation_burn
                + &gas_output.refund;
            return vec![(sender_id, total)];
        }

        // the execution fails for the validator, fallback to default gas hook from fvm
        default_gas_hook(sender_id, receipt, gas_output)
    };

    let options = ExecutionOptions {
        always_revert: false,
        txn_gas_hook: custom_hook,
    };

    let (apply_ret, emitters) =
        state.execute_with_options(msg.clone(), ApplyKind::Explicit, options)?;
    let ret = crate::signed::SignedMessageApplyRet {
        fvm: FvmApplyRet {
            apply_ret,
            from: msg.from,
            to: msg.to,
            method_num: msg.method_num,
            gas_limit: msg.gas_limit,
            emitters,
        },
        domain_hash,
    };
    Ok(ChainMessageApplyRet::Signed(Ok(ret)))
}

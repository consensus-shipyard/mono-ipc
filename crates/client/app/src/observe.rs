// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;

use fendermint_vm_interpreter::fvm::FvmMessage;
use ipc_observability::{
    impl_traceable, impl_traceables, lazy_static, register_metrics, serde::HexEncodableBlockHash,
    Recordable, TraceLevel, Traceable,
};
use prometheus::{register_counter_vec, register_int_gauge, CounterVec, IntGauge, Registry};
use tendermint::account::Id;

register_metrics! {
    CONSENSUS_BLOCK_PROPOSAL_RECEIVED: IntGauge
        = register_int_gauge!("consensus_block_proposal_received_height", "Block proposal received (last height)");
    CONSENSUS_BLOCK_PROPOSAL_SENT: IntGauge
        = register_int_gauge!("consensus_block_proposal_sent_height", "Block proposal sent (last height)");
    CONSENSUS_BLOCK_PROPOSAL_ACCEPTED: IntGauge
        = register_int_gauge!("consensus_block_proposal_accepted_height", "Block proposal accepted (last height)");
    CONSENSUS_BLOCK_PROPOSAL_REJECTED: IntGauge
        = register_int_gauge!("consensus_block_proposal_rejected_height", "Block proposal rejected (last height)");
    CONSENSUS_BLOCK_COMMITTED: IntGauge
        = register_int_gauge!("consensus_block_committed_height", "Block committed (last height)");
    MPOOL_RECEIVED: CounterVec = register_counter_vec!("mpool_received", "Message received in mpool", &["accept"]);
}

impl_traceables!(
    TraceLevel::Info,
    "Consensus",
    BlockProposalReceived<'a>,
    BlockProposalSent<'a>,
    BlockProposalEvaluated<'a>,
    BlockCommitted
);

impl_traceables!(TraceLevel::Info, "Mpool", MpoolReceived);

pub type BlockHeight = u64;

#[derive(Debug)]
pub struct BlockProposalReceived<'a> {
    pub height: BlockHeight,
    pub hash: HexEncodableBlockHash,
    pub size: usize,
    pub tx_count: usize,
    pub validator: &'a Id,
}

impl Recordable for BlockProposalReceived<'_> {
    fn record_metrics(&self) {
        CONSENSUS_BLOCK_PROPOSAL_RECEIVED.set(self.height as i64);
    }
}

#[derive(Debug)]
pub struct BlockProposalSent<'a> {
    pub validator: &'a Id,
    pub height: BlockHeight,
    pub size: usize,
    pub tx_count: usize,
}

impl Recordable for BlockProposalSent<'_> {
    fn record_metrics(&self) {
        CONSENSUS_BLOCK_PROPOSAL_SENT.set(self.height as i64)
    }
}

#[derive(Debug)]
pub struct BlockProposalEvaluated<'a> {
    pub height: BlockHeight,
    pub hash: HexEncodableBlockHash,
    pub size: usize,
    pub tx_count: usize,
    pub validator: &'a Id,
    pub accept: bool,
    pub reason: Option<&'a str>,
}

impl Recordable for BlockProposalEvaluated<'_> {
    fn record_metrics(&self) {
        if self.accept {
            CONSENSUS_BLOCK_PROPOSAL_ACCEPTED.set(self.height as i64);
        } else {
            CONSENSUS_BLOCK_PROPOSAL_REJECTED.set(self.height as i64);
        }
    }
}

#[derive(Debug)]
pub struct BlockCommitted {
    pub height: BlockHeight,
    pub app_hash: HexEncodableBlockHash,
}

impl Recordable for BlockCommitted {
    fn record_metrics(&self) {
        CONSENSUS_BLOCK_COMMITTED.set(self.height as i64)
    }
}

#[derive(Debug)]
pub struct Message {
    pub from: Address,
    pub to: Address,
    pub value: TokenAmount,
    pub gas_limit: u64,
    pub fee_cap: TokenAmount,
    pub premium: TokenAmount,
}

impl From<&FvmMessage> for Message {
    fn from(fvm_message: &FvmMessage) -> Self {
        Message {
            from: fvm_message.from,
            to: fvm_message.to,
            value: fvm_message.value.clone(),
            gas_limit: fvm_message.gas_limit,
            fee_cap: fvm_message.gas_fee_cap.clone(),
            premium: fvm_message.gas_premium.clone(),
        }
    }
}

#[derive(Debug, Default)]
pub struct MpoolReceived {
    // TODO - add cid later on
    // pub message_cid: &'a str,
    pub message: Option<Message>,
    pub accept: bool,
    pub reason: Option<String>,
}

impl Recordable for MpoolReceived {
    fn record_metrics(&self) {
        MPOOL_RECEIVED
            .with_label_values(&[&self.accept.to_string()])
            .inc();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipc_observability::emit;

    #[test]
    fn test_emit() {
        let id = Id::new([0x01; 20]);

        emit(BlockProposalReceived {
            height: 1,
            hash: HexEncodableBlockHash(vec![0x01, 0x02, 0x03]),
            size: 100,
            tx_count: 10,
            validator: &id,
        });

        emit(BlockProposalSent {
            height: 1,
            size: 100,
            tx_count: 10,
            validator: &id,
        });

        emit(BlockProposalEvaluated {
            height: 1,
            hash: HexEncodableBlockHash(vec![0x01, 0x02, 0x03]),
            size: 100,
            tx_count: 10,
            validator: &id,
            accept: true,
            reason: None,
        });

        emit(BlockProposalEvaluated {
            height: 1,
            hash: HexEncodableBlockHash(vec![0x01, 0x02, 0x03]),
            size: 100,
            tx_count: 10,
            validator: &id,
            accept: false,
            reason: None,
        });

        emit(BlockCommitted {
            height: 1,
            app_hash: HexEncodableBlockHash(vec![0x01, 0x02, 0x03]),
        });
    }
}

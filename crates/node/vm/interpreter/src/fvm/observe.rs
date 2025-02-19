// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use fvm_shared::address::Address;
use ipc_observability::{
    impl_traceable, impl_traceables, lazy_static, register_metrics, serde::HexEncodableBlockHash,
    Recordable, TraceLevel, Traceable,
};

use prometheus::{
    register_histogram, register_int_counter, register_int_gauge, register_int_gauge_vec,
    Histogram, IntCounter, IntGauge, IntGaugeVec, Registry,
};

use fvm_shared::message::Message;

register_metrics! {
    EXEC_FVM_CHECK_EXECUTION_TIME_SECS: Histogram
        = register_histogram!("exec_fvm_check_execution_time_secs", "Execution time of FVM check in seconds");
    EXEC_FVM_ESTIMATE_EXECUTION_TIME_SECS: Histogram
        = register_histogram!("exec_fvm_estimate_execution_time_secs", "Execution time of FVM estimate in seconds");
    EXEC_FVM_APPLY_EXECUTION_TIME_SECS: Histogram
        = register_histogram!("exec_fvm_apply_execution_time_secs", "Execution time of FVM apply in seconds");
    EXEC_FVM_CALL_EXECUTION_TIME_SECS: Histogram
        = register_histogram!("exec_fvm_call_execution_time_secs", "Execution time of FVM call in seconds");
    BOTTOMUP_CHECKPOINT_CREATED_TOTAL: IntCounter
        = register_int_counter!("bottomup_checkpoint_created_total", "Bottom-up checkpoint produced");
    BOTTOMUP_CHECKPOINT_CREATED_HEIGHT: IntGauge
        = register_int_gauge!("bottomup_checkpoint_created_height", "Height of the checkpoint created");
    BOTTOMUP_CHECKPOINT_CREATED_MSGCOUNT: IntGauge
        = register_int_gauge!("bottomup_checkpoint_created_msgcount", "Number of messages in the checkpoint created");
    BOTTOMUP_CHECKPOINT_CREATED_CONFIGNUM: IntGauge
        = register_int_gauge!("bottomup_checkpoint_created_confignum", "Configuration number of the checkpoint created");
    BOTTOMUP_CHECKPOINT_SIGNED_HEIGHT: IntGaugeVec = register_int_gauge_vec!(
        "bottomup_checkpoint_signed_height",
        "Height of the checkpoint signed",
        &["validator"]
    );
    BOTTOMUP_CHECKPOINT_FINALIZED_HEIGHT: IntGauge
        = register_int_gauge!("bottomup_checkpoint_finalized_height", "Height of the checkpoint finalized");
}

impl_traceables!(TraceLevel::Info, "Execution", MsgExec);

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MsgExecPurpose {
    Check,
    Apply,
    Estimate,
    Call,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MsgExec {
    pub purpose: MsgExecPurpose,
    pub message: Message,
    pub height: i64,
    pub duration: f64,
    pub exit_code: u32,
}

impl Recordable for MsgExec {
    fn record_metrics(&self) {
        match self.purpose {
            MsgExecPurpose::Check => EXEC_FVM_CHECK_EXECUTION_TIME_SECS.observe(self.duration),
            MsgExecPurpose::Estimate => {
                EXEC_FVM_ESTIMATE_EXECUTION_TIME_SECS.observe(self.duration)
            }
            MsgExecPurpose::Apply => EXEC_FVM_APPLY_EXECUTION_TIME_SECS.observe(self.duration),
            MsgExecPurpose::Call => EXEC_FVM_CALL_EXECUTION_TIME_SECS.observe(self.duration),
        }
    }
}

impl_traceables!(
    TraceLevel::Info,
    "Bottomup",
    CheckpointCreated,
    CheckpointSigned,
    CheckpointFinalized
);

#[derive(Debug)]
pub struct CheckpointCreated {
    pub height: u64,
    pub hash: HexEncodableBlockHash,
    pub msg_count: usize,
    pub config_number: u64,
}

impl Recordable for CheckpointCreated {
    fn record_metrics(&self) {
        BOTTOMUP_CHECKPOINT_CREATED_TOTAL.inc();
        BOTTOMUP_CHECKPOINT_CREATED_HEIGHT.set(self.height as i64);
        BOTTOMUP_CHECKPOINT_CREATED_MSGCOUNT.set(self.msg_count as i64);
        BOTTOMUP_CHECKPOINT_CREATED_CONFIGNUM.set(self.config_number as i64);
    }
}

#[derive(Debug)]
pub enum CheckpointSignedRole {
    Own,
    Peer,
}

#[derive(Debug)]
pub struct CheckpointSigned {
    pub role: CheckpointSignedRole,
    pub height: u64,
    pub hash: HexEncodableBlockHash,
    pub validator: Address,
}

impl Recordable for CheckpointSigned {
    fn record_metrics(&self) {
        BOTTOMUP_CHECKPOINT_SIGNED_HEIGHT
            .with_label_values(&[format!("{}", self.validator).as_str()])
            .set(self.height as i64);
    }
}

#[derive(Debug)]
pub struct CheckpointFinalized {
    pub height: i64,
    pub hash: HexEncodableBlockHash,
}

impl Recordable for CheckpointFinalized {
    fn record_metrics(&self) {
        BOTTOMUP_CHECKPOINT_FINALIZED_HEIGHT.set(self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipc_observability::emit;

    #[test]
    fn test_metrics() {
        let registry = Registry::new();
        register_metrics(&registry).unwrap();
    }

    #[test]
    fn test_emit() {
        use fvm_ipld_encoding::RawBytes;
        use fvm_shared::address::Address;
        use fvm_shared::econ::TokenAmount;

        let message = Message {
            version: 1,
            from: Address::new_id(1),
            to: Address::new_id(2),
            sequence: 1,
            value: TokenAmount::from_atto(1),
            method_num: 1,
            params: RawBytes::default(),
            gas_limit: 1,
            gas_fee_cap: TokenAmount::from_atto(1),
            gas_premium: TokenAmount::from_atto(1),
        };

        emit(MsgExec {
            purpose: MsgExecPurpose::Check,
            height: 1,
            duration: 1.0,
            exit_code: 1,
            message: message.clone(),
        });
        let hash = vec![0x01, 0x02, 0x03];

        emit(CheckpointCreated {
            height: 1,
            hash: HexEncodableBlockHash(hash.clone()),
            msg_count: 2,
            config_number: 3,
        });

        emit(CheckpointSigned {
            role: CheckpointSignedRole::Own,
            height: 1,
            hash: HexEncodableBlockHash(hash.clone()),
            validator: Address::new_id(1),
        });
    }
}

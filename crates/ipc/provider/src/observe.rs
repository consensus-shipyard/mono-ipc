// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use ipc_observability::{
    impl_traceable, impl_traceables, lazy_static, register_metrics, serde::HexEncodableBlockHash,
    Recordable, TraceLevel, Traceable,
};
use prometheus::{register_int_gauge, IntGauge, Registry};

register_metrics! {
    BOTTOMUP_CHECKPOINT_FINALIZED_HEIGHT: IntGauge
        = register_int_gauge!("bottomup_checkpoint_finalized_height", "Height of the checkpoint finalized");
}

impl_traceables!(TraceLevel::Info, "Bottomup", CheckpointSubmitted);

#[derive(Debug)]
pub struct CheckpointSubmitted {
    pub height: i64,
    pub hash: HexEncodableBlockHash,
}

impl Recordable for CheckpointSubmitted {
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
        let hash = vec![0x01, 0x02, 0x03];

        emit(CheckpointSubmitted {
            height: 1,
            hash: HexEncodableBlockHash(hash.clone()),
        });
    }
}

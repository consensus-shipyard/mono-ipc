// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::{
    impl_traceable, impl_traceables, lazy_static, register_metrics, Recordable, TraceLevel,
    Traceable,
};
use anyhow;
use prometheus::{register_int_counter_vec, IntCounterVec, Registry};

register_metrics! {
    TRACING_ERRORS: IntCounterVec
        = register_int_counter_vec!("tracing_errors", "Number of tracing errors", &["event"]);
}

impl_traceables!(TraceLevel::Error, "System", TracingError<'a>);

#[derive(Debug)]
pub struct TracingError<'a> {
    pub affected_event: &'a str,
    pub reason: String,
}

impl Recordable for TracingError<'_> {
    fn record_metrics(&self) {
        TRACING_ERRORS
            .with_label_values(&[self.affected_event])
            .inc();
    }
}

// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::config::{FileLayerSettings, TracingSettings};
use crate::tracing_layers::DomainEventFilterLayer;
use std::num::NonZeroUsize;
use tracing::Level;
pub use tracing_appender::non_blocking;
pub use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{fmt, fmt::Subscriber, layer::SubscriberExt, EnvFilter, Layer, Registry};

pub const TRACING_TARGET: &str = "tracing_event";

// Creates a temporary subscriber that logs all traces to stderr. Useful when global tracing is not set yet.
pub fn create_temporary_subscriber() -> Subscriber {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .finish()
}

// Sets the global tracing subscriber.
//
// All traces emitted through the tracing library will be routed to this subscriber.
//
// This subscriber bifurcates tracing events into two individual sinks: one for logs and one for
// structured traces. We also set up the console sink, if requested by the configuration.
//
// Returns a guard that can be used to drop the subscriber.
pub fn set_global_tracing_subscriber(config: &TracingSettings) -> Vec<WorkerGuard> {
    let console_layer = {
        let filter: EnvFilter = config
            .console
            .as_ref()
            .and_then(|c| c.level.clone())
            .unwrap_or_default()
            .into();

        // log all traces to stderr (reserving stdout for any actual output such as from the CLI commands)
        fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_filter(filter)
    };

    let (traces_layer, logs_layer, guards) = if let Some(file_settings) =
        config.file.as_ref().filter(|s| s.enabled)
    {
        //
        // Set up the traces layer.
        //
        let (traces_layer, traces_guard) = {
            let (appender, guard) = non_blocking(create_file_appender(file_settings, "traces.log"));

            // setup traces file layer - traces are logs that have the target set to TRACING_TARGET
            let layer = fmt::layer()
                .json()
                .with_writer(appender)
                .with_target(false)
                .with_file(false)
                .with_line_number(false);

            let domains = file_settings
                .domain_filter
                .as_ref()
                .map(|v| v.iter().map(|s| s.to_string()).collect());

            let events = file_settings
                .events_filter
                .as_ref()
                .map(|v| v.iter().map(|s| s.to_string()).collect());

            let level = &file_settings.level.clone().unwrap_or_default();

            let filter = EnvFilter::try_new(format!("{TRACING_TARGET}={}", level))
                .expect("invalid traces level");

            let filtered_layer =
                DomainEventFilterLayer::new(domains, events, layer).with_filter(filter);

            (filtered_layer, guard)
        };

        //
        // Set up the logs layer.
        //
        let (logs_layer, logs_guard) = {
            // setup logs file layer first - logs are traces that does not have the target set to TRACING_TARGET
            let (appender, guard) = non_blocking(create_file_appender(file_settings, "app.log"));

            let mut filter: EnvFilter = file_settings.level.clone().unwrap_or_default().into();
            filter = filter.add_directive(
                format!("{TRACING_TARGET}=off")
                    .parse()
                    .expect("invalid logs level"),
            );

            let layer = fmt::layer()
                .json()
                .with_writer(appender)
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .with_filter(filter);

            (layer, guard)
        };

        (
            Some(traces_layer),
            Some(logs_layer),
            vec![logs_guard, traces_guard],
        )
    } else {
        (None, None, Vec::new())
    };

    // Start with the base registry
    let subscriber = Registry::default()
        .with(console_layer)
        .with(traces_layer)
        .with(logs_layer);

    // Set the global subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global tracing subscriber");

    guards
}

fn create_file_appender(settings: &FileLayerSettings, suffix: &str) -> RollingFileAppender {
    let directory = settings
        .directory
        .as_deref()
        .expect("missing file log directory");
    let mut appender = RollingFileAppender::builder().filename_suffix(suffix);

    if let Some(max_log_files) = settings.max_log_files {
        appender = appender.max_log_files(
            NonZeroUsize::new(max_log_files)
                .expect("max_log_files must be greater than 0")
                .into(),
        );
    };

    let rotation: tracing_appender::rolling::Rotation = settings
        .rotation
        .as_ref()
        .map(|r| r.into())
        .unwrap_or(tracing_appender::rolling::Rotation::DAILY);

    appender
        .rotation(rotation)
        .build(directory)
        .expect("failed to create traces appender")
}

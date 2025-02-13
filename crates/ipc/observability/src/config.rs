// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use serde::Deserialize;
use serde_with::serde_as;
use std::path::PathBuf;
use strum;
use tracing_appender;
use tracing_appender::rolling::Rotation;

#[serde_as]
#[derive(Debug, Deserialize, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum RotationKind {
    Minutely,
    Hourly,
    Daily,
    Never,
}

impl From<&RotationKind> for tracing_appender::rolling::Rotation {
    fn from(kind: &RotationKind) -> tracing_appender::rolling::Rotation {
        match kind {
            RotationKind::Minutely => tracing_appender::rolling::Rotation::MINUTELY,
            RotationKind::Hourly => tracing_appender::rolling::Rotation::HOURLY,
            RotationKind::Daily => tracing_appender::rolling::Rotation::DAILY,
            RotationKind::Never => tracing_appender::rolling::Rotation::NEVER,
        }
    }
}

impl RotationKind {
    pub fn to_tracing_rotation(&self) -> Rotation {
        match self {
            RotationKind::Minutely => Rotation::DAILY,
            RotationKind::Hourly => Rotation::HOURLY,
            RotationKind::Daily => Rotation::DAILY,
            RotationKind::Never => Rotation::NEVER,
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct TracingSettings {
    pub console: Option<ConsoleLayerSettings>,
    pub file: Option<FileLayerSettings>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct ConsoleLayerSettings {
    pub level: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct FileLayerSettings {
    pub enabled: bool,
    pub level: Option<String>,
    pub directory: Option<PathBuf>,
    pub max_log_files: Option<usize>,
    pub rotation: Option<RotationKind>,
    pub domain_filter: Option<Vec<String>>,
    pub events_filter: Option<Vec<String>>,
}

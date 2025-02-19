// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use fvm_shared::econ::TokenAmount;
use ipc_provider::config::deserialize::{
    deserialize_cors_headers, deserialize_cors_methods, deserialize_cors_origins,
};
use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use std::time::Duration;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin};

use ipc_observability::config::TracingSettings;

use crate::{IsHumanReadable, MetricsSettings, SocketAddress};

/// Ethereum API facade settings.
#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct EthSettings {
    pub listen: SocketAddress,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub filter_timeout: Duration,
    pub cache_capacity: usize,
    pub gas: GasOpt,
    pub max_nonce_gap: u64,
    pub metrics: MetricsSettings,
    pub cors: CorsOpt,
    pub tracing: TracingSettings,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct GasOpt {
    /// Minimum gas fee in atto.
    #[serde_as(as = "IsHumanReadable")]
    pub min_gas_premium: TokenAmount,
    pub num_blocks_max_prio_fee: u64,
    pub max_fee_hist_size: u64,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct CorsOpt {
    #[serde(deserialize_with = "deserialize_cors_origins")]
    pub allowed_origins: AllowOrigin,
    #[serde(deserialize_with = "deserialize_cors_methods")]
    pub allowed_methods: AllowMethods,
    #[serde(deserialize_with = "deserialize_cors_headers")]
    pub allowed_headers: AllowHeaders,
}

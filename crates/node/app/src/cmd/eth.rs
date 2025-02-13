// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::time::Duration;

use anyhow::Context;
use fendermint_eth_api::HybridClient;
use tracing::info;

use crate::{
    cmd,
    options::eth::{EthArgs, EthCommands},
    settings::eth::EthSettings,
};

cmd! {
  EthArgs(self, settings: EthSettings) {
    match self.command.clone() {
      EthCommands::Run { ws_url, http_url, connect_retry_delay } => {
        let (client, driver) = HybridClient::new(http_url, ws_url, Duration::from_secs(connect_retry_delay)).context("failed to create HybridClient")?;

        let driver_handle = tokio::spawn(async move { driver.run().await });

        let result = run(settings, client).await;

        // Await the driver's termination to ensure proper connection closure.
        let _ = driver_handle.await;
        result
      }
    }
  }
}

/// Run the Ethereum API facade.
async fn run(settings: EthSettings, client: HybridClient) -> anyhow::Result<()> {
    if settings.metrics.enabled {
        info!("metrics enabled");

        let registry = prometheus::Registry::new();

        fendermint_app::metrics::register_eth_metrics(&registry)
            .context("failed to register metrics")?;

        info!(
            listen_addr = settings.metrics.listen.to_string(),
            "serving metrics"
        );
        let mut builder = prometheus_exporter::Builder::new(settings.metrics.listen.try_into()?);
        builder.with_registry(registry);
        let _ = builder.start().context("failed to start metrics server")?;
    } else {
        info!("metrics disabled");
    }

    let gas = fendermint_eth_api::GasOpt {
        min_gas_premium: settings.gas.min_gas_premium,
        num_blocks_max_prio_fee: settings.gas.num_blocks_max_prio_fee,
        max_fee_hist_size: settings.gas.max_fee_hist_size,
    };
    let cors = fendermint_eth_api::CorsOpt {
        allowed_origins: settings.cors.allowed_origins,
        allowed_methods: settings.cors.allowed_methods,
        allowed_headers: settings.cors.allowed_headers,
    };
    fendermint_eth_api::listen(
        settings.listen,
        client,
        settings.filter_timeout,
        settings.cache_capacity,
        settings.max_nonce_gap,
        gas,
        cors,
    )
    .await
}

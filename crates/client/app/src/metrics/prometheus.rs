// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
//! Prometheus metrics

/// Metrics emitted by the Ethereum API facade.
pub mod eth {
    // TODO - migrate these metrics to new observability architecture
    use fendermint_eth_api::apis::RPC_METHOD_CALL_LATENCY_SECONDS;

    pub fn register_metrics(registry: &prometheus::Registry) -> anyhow::Result<()> {
        registry.register(Box::new(RPC_METHOD_CALL_LATENCY_SECONDS.clone()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_register_eth_metrics() {
        let r = prometheus::Registry::new();
        super::eth::register_metrics(&r).unwrap();
    }
}

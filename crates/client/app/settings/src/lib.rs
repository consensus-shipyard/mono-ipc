// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::{anyhow, bail, Context};
use config::{Config, ConfigError, Environment, File};
use fvm_shared::address::Address;
use fvm_shared::bigint::Zero;
use fvm_shared::econ::TokenAmount;
use ipc_api::subnet_id::SubnetID;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};
use std::fmt::{Display, Formatter};
use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tendermint_rpc::Url;
use testing::TestingSettings;
use utils::EnvInterpol;

use fendermint_vm_encoding::{human_readable_delegate, human_readable_str};
use fendermint_vm_topdown::BlockHeight;

use self::eth::EthSettings;
use self::fvm::FvmSettings;
use self::resolver::ResolverSettings;
use ipc_observability::config::TracingSettings;
use ipc_provider::config::deserialize::deserialize_eth_address_from_str;

pub mod eth;
pub mod fvm;
pub mod resolver;
pub mod testing;
pub mod utils;

/// Marker to be used with the `#[serde_as(as = "IsHumanReadable")]` annotations.
///
/// We can't just import `fendermint_vm_encoding::IsHumanReadable` because we can't implement traits for it here,
/// however we can use the `human_readable_delegate!` macro to delegate from this to that for the types we need
/// and it will look the same.
struct IsHumanReadable;

human_readable_str!(SubnetID);
human_readable_delegate!(TokenAmount);

#[derive(Debug, Deserialize, Clone)]
pub struct SocketAddress {
    pub host: String,
    pub port: u32,
}

impl Display for SocketAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl std::net::ToSocketAddrs for SocketAddress {
    type Iter = <String as std::net::ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        self.to_string().to_socket_addrs()
    }
}

impl TryInto<std::net::SocketAddr> for SocketAddress {
    type Error = std::io::Error;

    fn try_into(self) -> Result<std::net::SocketAddr, Self::Error> {
        self.to_socket_addrs()?
            .next()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
/// Indicate the FVM account kind for generating addresses from a key.
pub enum AccountKind {
    /// Has an f1 address.
    Regular,
    /// Has an f410 address.
    Ethereum,
}

/// A Secp256k1 key used to sign transactions,
/// with the account kind showing if it's a regular or an ethereum key.
#[derive(Debug, Deserialize, Clone)]
pub struct SigningKey {
    path: PathBuf,
    pub kind: AccountKind,
}

home_relative!(SigningKey { path });

#[derive(Debug, Deserialize, Clone)]
pub struct AbciSettings {
    pub listen: SocketAddress,
    /// Queue size for each ABCI component.
    pub bound: usize,
    /// Maximum number of messages allowed in a block.
    pub block_max_msgs: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
/// Indicate the FVM account kind for generating addresses from a key.
///
/// See https://github.com/facebook/rocksdb/wiki/Compaction
pub enum DbCompaction {
    /// Good when most keys don't change.
    Level,
    Universal,
    Fifo,
    /// Auto-compaction disabled, has to be called manually.
    None,
}

impl Display for DbCompaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .map_err(|e| {
                    tracing::error!("cannot format DB compaction to json: {e}");
                    std::fmt::Error
                })?
                .as_str()
                .ok_or(std::fmt::Error)?
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbSettings {
    /// Length of the app state history to keep in the database before pruning; 0 means unlimited.
    ///
    /// This affects how long we can go back in state queries.
    pub state_hist_size: u64,
    /// How to compact the datastore.
    pub compaction_style: DbCompaction,
}

/// Settings affecting how we deal with failures in trying to send transactions to the local CometBFT node.
/// It is not expected to be unavailable, however we might get into race conditions about the nonce which
/// would need us to try creating a completely new transaction and try again.
#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct BroadcastSettings {
    /// Number of times to retry broadcasting a transaction.
    pub max_retries: u8,
    /// Time to wait between retries. This should roughly correspond to the block interval.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub retry_delay: Duration,
    /// Any over-estimation to apply on top of the estimate returned by the API.
    pub gas_overestimation_rate: f64,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct TopDownSettings {
    /// The number of blocks to delay before reporting a height as final on the parent chain.
    /// To propose a certain number of epochs delayed from the latest height, we see to be
    /// conservative and avoid other from rejecting the proposal because they don't see the
    /// height as final yet.
    pub chain_head_delay: BlockHeight,
    /// The number of blocks on top of `chain_head_delay` to wait before proposing a height
    /// as final on the parent chain, to avoid slight disagreements between validators whether
    /// a block is final, or not just yet.
    pub proposal_delay: BlockHeight,
    /// The max number of blocks one should make the topdown proposal
    pub max_proposal_range: BlockHeight,
    /// The max number of blocks to hold in memory for parent syncer
    pub max_cache_blocks: Option<BlockHeight>,
    /// Parent syncing cron period, in seconds
    #[serde_as(as = "DurationSeconds<u64>")]
    pub polling_interval: Duration,
    /// Top down exponential back off retry base
    #[serde_as(as = "DurationSeconds<u64>")]
    pub exponential_back_off: Duration,
    /// The max number of retries for exponential backoff before giving up
    pub exponential_retry_limit: usize,
    /// The parent rpc http endpoint
    pub parent_http_endpoint: Url,
    /// Timeout for calls to the parent Ethereum API.
    #[serde_as(as = "Option<DurationSeconds<u64>>")]
    pub parent_http_timeout: Option<Duration>,
    /// Bearer token for any Authorization header.
    pub parent_http_auth_token: Option<String>,
    /// The parent registry address
    #[serde(deserialize_with = "deserialize_eth_address_from_str")]
    pub parent_registry: Address,
    /// The parent gateway address
    #[serde(deserialize_with = "deserialize_eth_address_from_str")]
    pub parent_gateway: Address,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct IpcSettings {
    #[serde_as(as = "IsHumanReadable")]
    pub subnet_id: SubnetID,
    /// Interval with which votes can be gossiped.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub vote_interval: Duration,
    /// Timeout after which the last vote is re-published.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub vote_timeout: Duration,
    /// The config for top down checkpoint. It's None if subnet id is root or not activating
    /// any top down checkpoint related operations
    pub topdown: Option<TopDownSettings>,
}

impl IpcSettings {
    pub fn topdown_config(&self) -> anyhow::Result<&TopDownSettings> {
        let ret = self
            .topdown
            .as_ref()
            .ok_or_else(|| anyhow!("top down config missing"))?;

        if ret.chain_head_delay.is_zero() {
            bail!("unsafe top-down chain head delay: zero value not accepted")
        };

        Ok(ret)
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct SnapshotSettings {
    /// Enable the export and import of snapshots.
    pub enabled: bool,
    /// How often to attempt to export snapshots in terms of block height.
    pub block_interval: BlockHeight,
    /// Number of snapshots to keep before purging old ones.
    pub hist_size: usize,
    /// Target chunk size, in bytes.
    pub chunk_size_bytes: usize,
    /// How long to keep a snapshot from being purged after it has been requested by a peer.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub last_access_hold: Duration,
    /// How often to poll CometBFT to see whether it has caught up with the chain.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub sync_poll_interval: Duration,
    /// Temporary directory for downloads.
    download_dir: Option<PathBuf>,
}

impl SnapshotSettings {
    pub fn download_dir(&self) -> PathBuf {
        self.download_dir.clone().unwrap_or(std::env::temp_dir())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsSettings {
    /// Enable the export of metrics over HTTP.
    pub enabled: bool,
    /// HTTP listen address where Prometheus metrics are hosted.
    pub listen: SocketAddress,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    /// Home directory configured on the CLI, to which all paths in settings can be set relative.
    home_dir: PathBuf,
    /// Database files.
    data_dir: PathBuf,
    /// State snapshots.
    snapshots_dir: PathBuf,
    /// Solidity contracts.
    contracts_dir: PathBuf,

    /// Where to reach CometBFT for queries or broadcasting transactions.
    tendermint_rpc_url: Url,

    /// Block height where we should gracefully stop the node
    pub halt_height: i64,

    /// Secp256k1 private key used for signing transactions sent in the validator's name. Leave empty if not validating.
    pub validator_key: Option<SigningKey>,

    pub abci: AbciSettings,
    pub db: DbSettings,
    pub metrics: MetricsSettings,
    pub snapshots: SnapshotSettings,
    pub eth: EthSettings,
    pub fvm: FvmSettings,
    pub resolver: ResolverSettings,
    pub broadcast: BroadcastSettings,
    pub ipc: IpcSettings,
    pub testing: Option<TestingSettings>,
    pub tracing: TracingSettings,
}

impl Settings {
    home_relative!(data_dir, snapshots_dir, contracts_dir);

    /// Load the default configuration from a directory,
    /// then potential overrides specific to the run mode,
    /// then overrides from the local environment,
    /// finally parse it into the [Settings] type.
    pub fn new(config_dir: &Path, home_dir: &Path, run_mode: &str) -> Result<Self, ConfigError> {
        Self::config(config_dir, home_dir, run_mode).and_then(Self::parse)
    }

    /// Load the configuration into a generic data structure.
    fn config(config_dir: &Path, home_dir: &Path, run_mode: &str) -> Result<Config, ConfigError> {
        Config::builder()
            .add_source(EnvInterpol(File::from(config_dir.join("default"))))
            // Optional mode specific overrides, checked into git.
            .add_source(EnvInterpol(
                File::from(config_dir.join(run_mode)).required(false),
            ))
            // Optional local overrides, not checked into git.
            .add_source(EnvInterpol(
                File::from(config_dir.join("local")).required(false),
            ))
            // Add in settings from the environment (with a prefix of FM)
            // e.g. `FM_DB__DATA_DIR=./foo/bar ./target/app` would set the database location.
            .add_source(EnvInterpol(
                Environment::with_prefix("fm")
                    .prefix_separator("_")
                    .separator("__")
                    .ignore_empty(true) // otherwise "" will be parsed as a list item
                    .try_parsing(true) // required for list separator
                    .list_separator(",") // need to list keys explicitly below otherwise it can't pase simple `String` type
                    .with_list_parse_key("tracing.file.domain_filter")
                    .with_list_parse_key("tracing.file.events_filter")
                    .with_list_parse_key("resolver.connection.external_addresses")
                    .with_list_parse_key("resolver.discovery.static_addresses")
                    .with_list_parse_key("resolver.membership.static_subnets")
                    .with_list_parse_key("eth.cors.allowed_origins")
                    .with_list_parse_key("eth.cors.allowed_methods")
                    .with_list_parse_key("eth.cors.allowed_headers")
                    .with_list_parse_key("eth.tracing.file.domain_filter")
                    .with_list_parse_key("eth.tracing.file.events_filter"),
            ))
            // Set the home directory based on what was passed to the CLI,
            // so everything in the config can be relative to it.
            // The `home_dir` key is not added to `default.toml` so there is no confusion
            // about where it will be coming from.
            .set_override("home_dir", home_dir.to_string_lossy().as_ref())?
            .build()
    }

    /// Try to parse the config into [Settings].
    fn parse(config: Config) -> Result<Self, ConfigError> {
        // Deserialize (and thus freeze) the entire configuration.
        config.try_deserialize()
    }

    /// The configured home directory.
    pub fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    /// Tendermint RPC URL from the environment or the config file.
    pub fn tendermint_rpc_url(&self) -> anyhow::Result<Url> {
        // Prefer the "standard" env var used in the CLI.
        match std::env::var("TENDERMINT_RPC_URL").ok() {
            Some(url) => url.parse::<Url>().context("invalid Tendermint URL"),
            None => Ok(self.tendermint_rpc_url.clone()),
        }
    }

    /// Indicate whether we have configured the top-down syncer to run.
    pub fn topdown_enabled(&self) -> bool {
        !self.ipc.subnet_id.is_root() && self.ipc.topdown.is_some()
    }

    /// Indicate whether we have configured the IPLD Resolver to run.
    pub fn resolver_enabled(&self) -> bool {
        !self.resolver.connection.listen_addr.is_empty()
            && self.ipc.subnet_id != *ipc_api::subnet_id::UNDEF
    }
}

// Run these tests serially because some of them modify the environment.
#[serial_test::serial]
#[cfg(test)]
mod tests {
    use multiaddr::multiaddr;
    use std::path::PathBuf;

    use crate::utils::tests::with_env_vars;

    use crate::DbCompaction;

    use super::{ConfigError, Settings};

    fn try_parse_config(run_mode: &str) -> Result<Settings, config::ConfigError> {
        let current_dir = PathBuf::from(".");
        let default_dir = PathBuf::from("../config");
        let c = Settings::config(&default_dir, &current_dir, run_mode)?;
        // Trying to debug the following sporadic error on CI:
        // thread 'tests::parse_test_config' panicked at client/app/settings/src/lib.rs:315:36:
        // failed to parse Settings: failed to parse: invalid digit found in string
        // This turned out to be due to the environment variable manipulation below mixing with another test,
        // which is why `#[serial]` was moved to the top.
        eprintln!("CONFIG = {:?}", c.cache);
        Settings::parse(c)
    }

    fn parse_config(run_mode: &str) -> Settings {
        try_parse_config(run_mode).expect("failed to parse Settings")
    }

    #[test]
    fn parse_default_config() {
        let settings = parse_config("");
        assert!(!settings.resolver_enabled());
    }

    #[test]
    fn parse_test_config() {
        let settings = parse_config("test");
        assert!(settings.resolver_enabled());
    }

    #[test]
    fn compaction_to_string() {
        assert_eq!(DbCompaction::Level.to_string(), "level");
    }

    #[test]
    fn parse_comma_separated() {
        let settings = with_env_vars(vec![
                ("FM_RESOLVER__CONNECTION__EXTERNAL_ADDRESSES", "/ip4/198.51.100.0/tcp/4242/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N,/ip6/2604:1380:2000:7a00::1/udp/4001/quic/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"),
                ("FM_RESOLVER__DISCOVERY__STATIC_ADDRESSES", "/ip4/198.51.100.1/tcp/4242/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N,/ip6/2604:1380:2000:7a00::2/udp/4001/quic/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"),
                ("FM_RESOLVER__MEMBERSHIP__STATIC_SUBNETS", "/r314/f410fijl3evsntewwhqxy6cx5ijdq5qp5cjlocbgzgey,/r314/f410fwplxlims2wnigaha2gofgktue7hiusmttwridkq"),
                ("FM_ETH__CORS__ALLOWED_ORIGINS", "https://example.com,https://www.example.org"),
                ("FM_ETH__CORS__ALLOWED_METHODS", "GET,POST"),
                ("FM_ETH__CORS__ALLOWED_HEADERS", "Accept,Content-Type"),
                // Set a normal string key as well to make sure we have configured the library correctly and it doesn't try to parse everything as a list.
                ("FM_RESOLVER__NETWORK__NETWORK_NAME", "test"),
            ], || try_parse_config("")).unwrap();

        assert_eq!(settings.resolver.connection.external_addresses.len(), 2);
        assert_eq!(settings.resolver.discovery.static_addresses.len(), 2);
        assert_eq!(settings.resolver.membership.static_subnets.len(), 2);
        assert_eq!(
            format!("{:?}", settings.eth.cors.allowed_origins),
            "List([\"https://example.com\", \"https://www.example.org\"])"
        );
        assert_eq!(
            format!("{:?}", settings.eth.cors.allowed_methods),
            "Const(Some(\"GET,POST\"))"
        );
        assert_eq!(
            format!("{:?}", settings.eth.cors.allowed_headers),
            "Const(Some(\"accept,content-type\"))"
        );
    }

    #[test]
    fn parse_empty_comma_separated() {
        let settings = with_env_vars(
            vec![
                ("FM_RESOLVER__CONNECTION__EXTERNAL_ADDRESSES", ""),
                ("FM_RESOLVER__DISCOVERY__STATIC_ADDRESSES", ""),
                ("FM_RESOLVER__MEMBERSHIP__STATIC_SUBNETS", ""),
                ("FM_ETH__CORS__ALLOWED_ORIGINS", ""),
                ("FM_ETH__CORS__ALLOWED_METHODS", ""),
                ("FM_ETH__CORS__ALLOWED_HEADERS", ""),
            ],
            || try_parse_config(""),
        )
        .unwrap();

        assert_eq!(settings.resolver.connection.external_addresses.len(), 0);
        assert_eq!(settings.resolver.discovery.static_addresses.len(), 0);
        assert_eq!(settings.resolver.membership.static_subnets.len(), 0);
        assert_eq!(
            format!("{:?}", settings.eth.cors.allowed_origins),
            "List([])"
        );
        assert_eq!(
            format!("{:?}", settings.eth.cors.allowed_methods),
            "Const(None)"
        );
        assert_eq!(
            format!("{:?}", settings.eth.cors.allowed_headers),
            "Const(None)"
        );
    }

    #[test]
    fn parse_with_interpolation() {
        let settings = with_env_vars(
                vec![
                    ("FM_RESOLVER__DISCOVERY__STATIC_ADDRESSES", "/dns4/${SEED_1_HOST}/tcp/${SEED_1_PORT},/dns4/${SEED_2_HOST}/tcp/${SEED_2_PORT}"),
                    ("SEED_1_HOST", "foo.io"),
                    ("SEED_1_PORT", "1234"),
                    ("SEED_2_HOST", "bar.ai"),
                    ("SEED_2_PORT", "5678"),
                ],
                || try_parse_config(""),
            )
            .unwrap();

        assert_eq!(settings.resolver.discovery.static_addresses.len(), 2);
        assert_eq!(
            settings.resolver.discovery.static_addresses[0],
            multiaddr!(Dns4("foo.io"), Tcp(1234u16))
        );
        assert_eq!(
            settings.resolver.discovery.static_addresses[1],
            multiaddr!(Dns4("bar.ai"), Tcp(5678u16))
        );
    }

    #[test]
    fn parse_cors_origins_variants() {
        // relative URL without a base
        let settings = with_env_vars(
            vec![("FM_ETH__CORS__ALLOWED_ORIGINS", "example.com")],
            || try_parse_config(""),
        );
        assert!(
            matches!(settings, Err(ConfigError::Message(ref msg)) if msg == "relative URL without a base")
        );

        // opaque origin
        let settings = with_env_vars(
            vec![(
                "FM_ETH__CORS__ALLOWED_ORIGINS",
                "javascript:console.log(\"invalid origin\")",
            )],
            || try_parse_config(""),
        );
        assert!(
            matches!(settings, Err(ConfigError::Message(ref msg)) if msg == "opaque origins are not allowed")
        );

        // Allow all with "*"
        let settings = with_env_vars(vec![("FM_ETH__CORS__ALLOWED_ORIGINS", "*")], || {
            try_parse_config("")
        });
        assert!(settings.is_ok());

        // IPv4
        let settings = with_env_vars(
            vec![("FM_ETH__CORS__ALLOWED_ORIGINS", "http://192.0.2.1:1234")],
            || try_parse_config(""),
        );
        assert!(settings.is_ok());

        // IPv6
        let settings = with_env_vars(
            vec![(
                "FM_ETH__CORS__ALLOWED_ORIGINS",
                "http://[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:1234",
            )],
            || try_parse_config(""),
        );
        assert!(settings.is_ok());
    }
}

// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use config::ConfigArgs;
use debug::DebugArgs;
use fvm_shared::address::Network;
use lazy_static::lazy_static;

use self::{
    eth::EthArgs, genesis::GenesisArgs, key::KeyArgs, materializer::MaterializerArgs, rpc::RpcArgs,
    run::RunArgs,
};

pub mod config;
pub mod debug;
pub mod eth;
pub mod genesis;
pub mod key;
pub mod materializer;
pub mod rpc;
pub mod run;

mod parse;

use parse::parse_network;

lazy_static! {
    static ref ENV_ALIASES: Vec<(&'static str, Vec<&'static str>)> = vec![
        ("FM_NETWORK", vec!["IPC_NETWORK", "NETWORK"]),
        ("FM_LOG_LEVEL", vec!["LOG_LEVEL", "RUST_LOG"])
    ];
}

/// Parse the main arguments by:
/// 0. Detecting aliased env vars
/// 1. Parsing the [GlobalOptions]
/// 2. Setting any system wide parameters based on the globals
/// 3. Parsing and returning the final [Options]
pub fn parse() -> Options {
    set_env_from_aliases();
    let opts: GlobalOptions = GlobalOptions::parse();
    fvm_shared::address::set_current_network(opts.global.network);
    let opts: Options = Options::parse();
    opts
}

/// Assign value to env vars from aliases, if the canonic key doesn't exist but the alias does.
fn set_env_from_aliases() {
    'keys: for (key, aliases) in ENV_ALIASES.iter() {
        for alias in aliases {
            if let (Err(_), Ok(value)) = (std::env::var(key), std::env::var(alias)) {
                std::env::set_var(key, value);
                continue 'keys;
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Set the FVM Address Network. It's value affects whether `f` (main) or `t` (test) prefixed addresses are accepted.
    #[arg(short, long, default_value = "mainnet", env = "FM_NETWORK", value_parser = parse_network)]
    pub network: Network,
}

/// A version of options that does partial matching on the arguments, with its only interest
/// being the capture of global parameters that need to take effect first, before we parse [Options],
/// because their value affects how others arse parsed.
///
/// This one doesn't handle `--help` or `help` so that it is passed on to the next parser,
/// where the full set of commands and arguments can be printed properly.
#[derive(Parser, Debug)]
#[command(version, disable_help_flag = true)]
pub struct GlobalOptions {
    #[command(flatten)]
    pub global: GlobalArgs,

    /// Capture all the normal commands, basically to ingore them.
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    pub cmd: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Options {
    /// Set a custom directory for data and configuration files.
    #[arg(
        short = 'd',
        long,
        default_value = "~/.fendermint",
        env = "FM_HOME_DIR"
    )]
    pub home_dir: PathBuf,

    /// Set a custom directory for configuration files
    #[arg(long, env = "FM_CONFIG_DIR")]
    config_dir: Option<PathBuf>,

    /// Optionally override the default configuration.
    #[arg(short, long, default_value = "dev")]
    pub mode: String,

    /// Global options repeated here for discoverability, so they show up in `--help` among the others.
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

impl Options {
    /// Path to the configuration directories.
    ///
    /// If not specified then returns the default under the home directory.
    pub fn config_dir(&self) -> PathBuf {
        self.config_dir
            .as_ref()
            .cloned()
            .unwrap_or(self.home_dir.join("config"))
    }

    /// Check if metrics are supposed to be collected.
    pub fn metrics_enabled(&self) -> bool {
        matches!(self.command, Commands::Run(_) | Commands::Eth(_))
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Parse the configuration file and print it to the console.
    Config(ConfigArgs),
    /// Arbitrary commands that aid in debugging.
    Debug(DebugArgs),
    /// Run the `App`, listening to ABCI requests from Tendermint.
    Run(RunArgs),
    /// Subcommands related to the construction of signing keys.
    Key(KeyArgs),
    /// Subcommands related to the construction of Genesis files.
    Genesis(GenesisArgs),
    /// Subcommands related to sending JSON-RPC commands/queries to Tendermint.
    Rpc(RpcArgs),
    /// Subcommands related to the Ethereum API facade.
    Eth(EthArgs),
    /// Subcommands related to the Testnet Materializer.
    #[clap(aliases  = &["mat", "matr", "mate"])]
    Materializer(MaterializerArgs),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use clap::Parser;
    use fvm_shared::address::Network;

    /// Set some env vars, run a fallible piece of code, then unset the variables otherwise they would affect the next test.
    pub fn with_env_vars<F, T>(vars: &[(&str, &str)], f: F) -> T
    where
        F: FnOnce() -> T,
    {
        for (k, v) in vars.iter() {
            std::env::set_var(k, v);
        }
        let result = f();
        for (k, _) in vars {
            std::env::remove_var(k);
        }
        result
    }

    #[test]
    fn parse_global() {
        let cmd = "fendermint --network testnet genesis --genesis-file ./genesis.json ipc gateway --subnet-id /r123/t0456 -b 10 -t 10 -f 10 -m 65";
        let opts: GlobalOptions = GlobalOptions::parse_from(cmd.split_ascii_whitespace());
        assert_eq!(opts.global.network, Network::Testnet);
    }

    #[test]
    fn global_options_ignore_help() {
        let cmd = "fendermint --help";
        let _opts: GlobalOptions = GlobalOptions::parse_from(cmd.split_ascii_whitespace());
    }

    #[test]
    fn network_from_env() {
        for (key, _) in ENV_ALIASES.iter() {
            std::env::remove_var(key);
        }

        let examples = [
            (vec![], Network::Mainnet),
            (vec![("IPC_NETWORK", "testnet")], Network::Testnet),
            (vec![("NETWORK", "testnet")], Network::Testnet),
            (vec![("FM_NETWORK", "testnet")], Network::Testnet),
            (
                vec![("IPC_NETWORK", "testnet"), ("FM_NETWORK", "mainnet")],
                Network::Mainnet,
            ),
        ];

        for (i, (vars, network)) in examples.iter().enumerate() {
            let opts = with_env_vars(vars, || {
                set_env_from_aliases();
                let opts: GlobalOptions = GlobalOptions::parse_from(["fendermint", "run"]);
                opts
            });
            assert_eq!(opts.global.network, *network, "example {i}");
        }
    }

    #[test]
    fn options_handle_help() {
        let cmd = "fendermint --help";
        // This test would fail with a panic if we have a misconfiguration in our options.
        // On successfully parsing `--help` with `parse_from` the library would `.exit()` the test framework itself,
        // which is why we must use `try_parse_from`. An error results in a panic from `parse_from` and an `Err`
        // from this, but `--help` is not an `Ok`, since we aren't getting `Options`; it's an `Err` with a help message.
        let e = Options::try_parse_from(cmd.split_ascii_whitespace())
            .expect_err("--help is not Options");

        assert!(e.to_string().contains("Usage:"), "unexpected help: {e}");
    }

    #[test]
    fn parse_invalid_log_level() {
        // NOTE: `nonsense` in itself is interpreted as a target. Maybe we should mandate at least `=` in it?
        let cmd = "fendermint --log-level nonsense/123 run";
        Options::try_parse_from(cmd.split_ascii_whitespace()).expect_err("should not parse");
    }
}

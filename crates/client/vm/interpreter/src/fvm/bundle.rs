// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::path::PathBuf;

// TODO factor out into a test support crate
// Find the root of the workspace, not this crate, which is what `env!("CARGO_MANIFEST_DIR")` would return
fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;

    let cargo_path = PathBuf::from(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().parent().unwrap().to_path_buf()
}

/// Path to the Solidity contracts, indended to be used in tests.
pub fn contracts_path() -> PathBuf {
    let contracts_path = std::env::var("FM_CONTRACTS_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| workspace_dir().join("contracts").join("out"));

    contracts_path
}
/// Path to the builtin-actor bundle, indended to be used in tests.
pub fn bundle_path() -> PathBuf {
    let bundle_path = std::env::var("FM_BUILTIN_ACTORS_BUNDLE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            workspace_dir()
                .join("crates")
                .join("client/builtin-actors/output/bundle.car")
        });

    bundle_path
}

/// Path to the in-repo custom actor bundle, intended to be used in tests.
pub fn custom_actors_bundle_path() -> PathBuf {
    let custom_actors_bundle_path = std::env::var("FM_CUSTOM_ACTORS_BUNDLE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            workspace_dir()
                .join("crates")
                .join("client/actors/output/custom_actors_bundle.car")
        });

    custom_actors_bundle_path
}

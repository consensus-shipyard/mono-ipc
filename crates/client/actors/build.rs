// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::anyhow;
use fil_actor_bundler::Bundler;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use toml::Value;

fn parse_dependencies_for_wasm32() -> anyhow::Result<Vec<(String, String)>> {
    let manifest = std::fs::read_to_string("Cargo.toml")?;
    let document = manifest.parse::<Value>()?;

    let dependencies = document
        .get("target")
        .and_then(|t| t.get(r#"cfg(target_arch = "wasm32")"#))
        .and_then(|t| t.get("dependencies"))
        .and_then(Value::as_table)
        .ok_or_else(|| anyhow!("could not find wasm32 dependencies"))?;

    let mut ret = Vec::with_capacity(dependencies.len());
    for (name, details) in dependencies.iter() {
        let path = details
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("could not find path for a wasm32 dependency"))?;
        ret.push((name.clone(), path.to_string()));
    }

    Ok(ret)
}

const FILES_TO_WATCH: &[&str] = &["Cargo.toml", "src"];

fn main() -> Result<(), Box<dyn Error>> {
    // Cargo executable location.
    let cargo = std::env::var_os("CARGO").expect("no CARGO env var");

    let out_dir = std::env::var_os("OUT_DIR")
        .as_ref()
        .map(Path::new)
        .map(|p| p.join("bundle"))
        .expect("no OUT_DIR env var");
    println!("cargo:warning=out_dir: {:?}", &out_dir);

    let manifest_path =
        Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR unset"))
            .join("Cargo.toml");

    let actors = parse_dependencies_for_wasm32()?;
    let actor_files = actors
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();

    for file in [FILES_TO_WATCH, actor_files.as_slice()].concat() {
        println!("cargo:rerun-if-changed={}", file);
    }

    // Cargo build command for all test_actors at once.
    let mut cmd = Command::new(cargo);
    cmd.arg("build")
        .args(actors.iter().map(|(pkg, _)| "-p=".to_owned() + pkg))
        .arg("--target=wasm32-unknown-unknown")
        .arg("--profile=wasm")
        .arg("--features=fil-actor")
        .arg(format!("--manifest-path={}", manifest_path.display()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // We are supposed to only generate artifacts under OUT_DIR,
        // so set OUT_DIR as the target directory for this build.
        .env("CARGO_TARGET_DIR", &out_dir)
        // As we are being called inside a build-script, this env variable is set. However, we set
        // our own `RUSTFLAGS` and thus, we need to remove this. Otherwise cargo favors this
        // env variable.
        .env_remove("CARGO_ENCODED_RUSTFLAGS");

    // Print out the command line we're about to run.
    println!("cargo:warning=cmd={:?}", &cmd);

    // Launch the command.
    let mut child = cmd.spawn().expect("failed to launch cargo build");

    // Pipe the output as cargo warnings. Unfortunately this is the only way to
    // get cargo build to print the output.
    let stdout = child.stdout.take().expect("no stdout");
    let stderr = child.stderr.take().expect("no stderr");
    let j1 = thread::spawn(move || {
        for line in BufReader::new(stderr).lines() {
            println!("cargo:warning={:?}", line.unwrap());
        }
    });
    let j2 = thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            println!("cargo:warning={:?}", line.unwrap());
        }
    });

    j1.join().unwrap();
    j2.join().unwrap();

    let result = child.wait().expect("failed to wait for build to finish");
    if !result.success() {
        return Err("actor build failed".into());
    }

    // make sure the output dir exists
    std::fs::create_dir_all("output")
        .expect("failed to create output dir for the custom_actors_bundle.car file");

    let dst = Path::new("output/custom_actors_bundle.car");
    let mut bundler = Bundler::new(dst);
    for (pkg, id) in actors.iter().map(|(pkg, _)| pkg).zip(1u32..) {
        let bytecode_path = Path::new(&out_dir)
            .join("wasm32-unknown-unknown/wasm")
            .join(format!("{}.wasm", pkg));

        // This actor version doesn't force synthetic CIDs; it uses genuine
        // content-addressed CIDs.
        let forced_cid = None;

        let actor_name = pkg
            .to_owned()
            .strip_prefix("fendermint_actor_")
            .ok_or_else(|| {
                format!("expected fendermint_actor_ prefix in actor package name; got: {pkg}")
            })?
            .to_owned();

        let cid = bundler
            .add_from_file(id, actor_name, forced_cid, &bytecode_path)
            .unwrap_or_else(|err| {
                panic!(
                    "failed to add file {:?} to bundle for actor {}: {}",
                    bytecode_path, id, err
                )
            });
        println!(
            "cargo:warning=added {} ({}) to bundle with CID {}",
            pkg, id, cid
        );
    }
    bundler.finish().expect("failed to finish bundle");

    println!("cargo:warning=bundle={}", dst.display());

    Ok(())
}

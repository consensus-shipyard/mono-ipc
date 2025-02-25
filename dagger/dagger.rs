// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use std::sync::Arc;

use color_eyre::eyre::{bail, Result};
use dagger_sdk::{
    logging::StdLogger, Container, ContainerWithDirectoryOptsBuilder,
    ContainerWithEnvVariableOptsBuilder, ContainerWithExecOptsBuilder,
    ContainerWithFileOptsBuilder, DaggerConn, Directory, HostDirectoryOpts,
};
use fs_err as fs;

/// Execute the lazily prepared container definition and convert to a `Result`
async fn run(container: &Container) -> Result<()> {
    let out_fut = container.stdout();
    let err_fut = container.stderr();
    let exit_code_fut = container.exit_code();
    let (out, err, exit_code) = tokio::join!(out_fut, err_fut, exit_code_fut);
    let out = out.inspect_err(|e| eprintln!("dagger err(stdout): {}", e));
    let err = err.inspect_err(|e| eprintln!("dagger err(stderr): {}", e));
    let exit_code = exit_code?;
    out?;
    err?;
    if exit_code == 0 {
        Ok(())
    } else {
        bail!("Exit code was non zero: {exit_code}")
    }
}

/// Simplify execution, split at whitespace
///
/// Note: Does not consider nested `"` nor escaping `\"`!
fn cmd(s: impl AsRef<str>) -> Vec<String> {
    Vec::from_iter(s.as_ref().split_whitespace().map(|x| x.to_string()))
}

/// Simplify access to the CWD
///
/// TODO: Should be the cargo manifest directory!
fn host_repo_root_dir(client: &DaggerConn) -> Directory {
    client.host().directory_opts(
        ".",
        HostDirectoryOpts {
            exclude: Some(vec!["node_modules", "target"]),
            include: None,
        },
    )
}

/// Register these caches as early as possible
///
/// The ordering matters.
///
/// TODO: `forge`/`solc` still recompiles all solidity contracts.
fn with_caches(container: Container, client: &DaggerConn) -> Container {
    let cache_volume_aptititude = client.cache_volume("apt-cache");
    let cache_volume_var_cache = client.cache_volume("apt-lists");
    let cache_volume_cargo = client.cache_volume("cargo");
    let cache_volume_rustup = client.cache_volume("rustup");
    let cache_volume_target = client.cache_volume("target");
    let cache_volume_solidity = client.cache_volume("solidity");
    let cache_volume_node_modules = client.cache_volume("node_modules");
    let cache_volume_pnpm_store = client.cache_volume("pnpm");
    let cache_volume_npm_store = client.cache_volume("npm");

    let container = container.with_mounted_cache("/root/.npm", cache_volume_npm_store.clone());
    let container =
        container.with_mounted_cache("/workdir/.pnpm-store", cache_volume_pnpm_store.clone());
    let container =
        container.with_mounted_cache("/workdir/node_modules", cache_volume_node_modules.clone());
    let container = container.with_mounted_cache("/var/cache", cache_volume_var_cache.clone());
    let container = container.with_mounted_cache("/var/lib/apt/", cache_volume_aptititude.clone());
    let container = container.with_mounted_cache("/workdir/target", cache_volume_target.clone());
    let container = container.with_mounted_cache(
        "/workdir/_compiled_contracts",
        cache_volume_solidity.clone(),
    );
    let container = container.with_mounted_cache("/root/.cargo", cache_volume_cargo.clone());

    container.with_mounted_cache("/root/.rustup", cache_volume_rustup.clone())
}

/// Create a container definition which is able to compile the contracts
fn define_contracts_container(
    client: DaggerConn,
    hrrd: Directory,
    hccd: Directory,
) -> Result<Container> {
    let compiled_contracts_dir = "/workdir/compiled_contracts";

    let container = with_caches(client
    .container().from("docker.io/library/node:latest")
    , &client)
    .with_mounted_directory("/workdir", hrrd.clone())
    .with_mounted_directory(compiled_contracts_dir, hccd.clone())
    .with_exec(cmd("apt-get update -y"))
    .with_exec(cmd("apt-get install -y curl which"))
    .with_workdir("/workdir/contracts")
    .with_exec(cmd("ls -al"))
    .with_exec(cmd("npm install -g pnpm"))
    .with_exec(cmd("pnpm install"))
    .with_exec(vec!["sh", "-c", "curl -L https://foundry.paradigm.xyz | bash && /root/.foundry/bin/foundryup --install 0.3.0"])
    .with_exec(cmd("git submodule update --init --recursive"))
    .with_exec(cmd(format!("mkdir -p {compiled_contracts_dir}")))
    .with_env_variable_opts("PATH", "${PATH}:/root/.foundry/bin", ContainerWithEnvVariableOptsBuilder::default().expand(true).build()?)
    .with_exec(cmd("which forge"))
    // actually build the contracts
    // TODO investigate caching issue further
    .with_exec(cmd(format!("forge -vvv build -C ./src/ --lib-paths ./lib/ --via-ir --sizes --skip test --out={compiled_contracts_dir}")));

    Ok(container)
}

fn define_crates_container(
    client: DaggerConn,
    hrrd: Directory,
    hccd: Directory,
) -> Result<Container> {
    let container = with_caches(client.container().from("docker.io/rust:bookworm"), &client)
        .with_mounted_directory("/workdir", hrrd.clone())
        .with_mounted_directory("/workdir/_compiled_contracts", hccd)
        .with_workdir("/workdir")
        .with_exec(cmd("apt-get update"))
        .with_exec(cmd(
            "apt-get install -y build-essential clang cmake protobuf-compiler",
        ))
        // actually build the rust binaries
        .with_exec(cmd("cargo b -p fendermint_app -p ipc-cli"))
        // see what was created
        .with_exec(cmd("ls -al /workdir/target/debug"))
        .with_exec(cmd("ls -al output"));

    Ok(container)
}

async fn prepare_fendermint_two_stage_build(client: DaggerConn) -> Result<Container> {
    fs::create_dir_all("_compiled_contracts")?;
    let hrrd = host_repo_root_dir(&client);
    let hccd = hrrd.directory("fendermint/actors/output");

    let contracts_gen = define_contracts_container(client.clone(), hrrd.clone(), hccd.clone())?;
    run(&contracts_gen).await?;

    let crates_def = define_crates_container(client.clone(), hrrd.clone(), hccd.clone())?;
    run(&crates_def).await?;

    let f_fendermint = crates_def.file("/workdir/target/debug/fendermint");
    let f_ipc = crates_def.file("/workdir/target/debug/ipc-cli");

    let car_extra =
        contracts_gen.file("/workdir/fendermint/actors/output/custom_actors_bundle.car");

    // prepare the to-be-published "runner" container
    let runner = with_caches(client
    .container()
    .from("docker.io/debian:bookworm-slim"), &client)
    .with_file_opts(
            "/usr/local/bin/fendermint",
            f_fendermint,
            ContainerWithFileOptsBuilder::default()
                .permissions(0o755_isize)
                .build()?,
        )
        .with_file_opts(
            "/usr/local/bin/ipc-cli",
            f_ipc,
            ContainerWithFileOptsBuilder::default()
                .permissions(0o755_isize)
                .build()?,
        )
        .with_exec(cmd("ls -al"))
        .with_exec(vec!["sh", "-c", "apt-get update && \
            apt-get install -y libssl3 ca-certificates curl && \
            rm -rf /var/lib/apt/lists/*"])
        .with_env_variable("FM_HOME_DIR", "/fendermint")
        .with_exposed_port(26658)
        .with_exposed_port(8445)
        .with_exposed_port(9184)
        .with_entrypoint(cmd("docker-entry.sh"))
        .with_default_terminal_cmd(cmd("run"))
        // TODO STOPSIGNAL SIGTERM
        .with_env_variable("FM_ABCI__LISTEN__HOST", "0.0.0.0")
        .with_env_variable("FM_ETH__LISTEN__HOST", "0.0.0.0")
        .with_env_variable("FM_METRICS__LISTEN__HOST", "0.0.0.0")        
        .with_exec(cmd("mkdir -p /fendermint/logs"))
        .with_exec(cmd("chmod 777 /fendermint/logs"))
        .with_file("/usr/local/bin/docker-entry.sh", hrrd.file("fendermint/docker/docker-entry.sh"))
        .with_file("/fendermint/custom_actors_bundle.car", car_extra)
        // TODO FIXME - this is insane
        .with_exec(cmd("mkdir -p /fendermint/builtin-actors/output"))
        .with_exec_opts(cmd("curl -L -o /fendermint/builtin-actors/output/bundle.car https://github.com/filecoin-project/builtin-actors/releases/download/${BUILTIN_ACTORS_TAG}/builtin-actors-mainnet.car"), ContainerWithExecOptsBuilder::default().expand(true).build()?)
        .with_directory("/fendermint/contracts", hrrd.directory("contracts/out"))
        .with_directory_opts("/fendermint/config", hrrd.directory("fendermint/app/config"), ContainerWithDirectoryOptsBuilder::default().exclude(vec![".git", ".gitignore", ".*"]).build()?);

    run(&runner).await?;

    // TODO extract file and publish gh release IFF tagged
    // TODO
    // TODO runner.publish(address).await?;

    Ok(runner)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // dagger_sdk::logging::default_logging()?;

    dagger_sdk::connect_opts(
        dagger_sdk::Config {
            workdir_path: None,
            config_path: None,
            timeout_ms: 2000,
            execute_timeout_ms: None,
            logger: Some(Arc::new(StdLogger::default())),
        },
        |client| async move {
            let _fendermint_featherweight =
                prepare_fendermint_two_stage_build(client).await.unwrap();

            // let anvil = prepare_anvil_service(client).await?;
            // let client = prepare_ipc_cli(client).await?;

            Ok(())
        },
    )
    .await?;

    Ok(())
}

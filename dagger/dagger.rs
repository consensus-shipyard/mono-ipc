use std::path::Path;
use std::sync::Arc;

use color_eyre::eyre::{self, bail, eyre, Result};
use dagger_sdk::{
    logging::{StdLogger, TracingLogger},
    CacheVolume, Container, ContainerBuildOpts, ContainerBuildOptsBuilder,
    ContainerWithEnvVariableOptsBuilder, ContainerWithExecOpts, ContainerWithExecOptsBuilder,
    ContainerWithFileOpts, ContainerWithFileOptsBuilder, ContainerWithMountedCacheOpts,
    ContainerWithMountedCacheOptsBuilder, ContainerWithMountedDirectoryOpts, DaggerConn, Directory,
    File, HostDirectoryOpts, Service,
};
use fs_err as fs;
use rand::Rng;

async fn run(container: &Container) -> eyre::Result<()> {
    let out_fut = container.stdout();
    let err_fut = container.stderr();
    let exit_code_fut = container.exit_code();
    let (out, err, exit_code) = tokio::join!(out_fut, err_fut, exit_code_fut);
    let out = out.inspect_err(|e| eprintln!("dagger err(stdout): {}", e.to_string()));
    let err = err.inspect_err(|e| eprintln!("dagger err(stderr): {}", e.to_string()));
    let exit_code = exit_code?;
    out?;
    err?;
    if exit_code == 0 {
        Ok(())
    } else {
        bail!("Exit code was non zero: {exit_code}")
    }
}

async fn prepare_anvil_service(client: DaggerConn) -> eyre::Result<Service> {
    let context_dir = client.host().directory("./contracts/");

    let container = client.container().build_opts(
        context_dir,
        ContainerBuildOptsBuilder::default()
            .dockerfile("docker/Dockerfile")
            .build()?,
    );

    let out_fut = container.stdout();
    let err_fut = container.stderr();
    let exit_code_fut = container.exit_code();
    let (out, err, exit_code) = tokio::join!(out_fut, err_fut, exit_code_fut);
    let out = dbg!(out?);
    let err = dbg!(err?);

    Ok(container.as_service())
}

async fn prepare_ipc_cli(client: DaggerConn) -> eyre::Result<Container> {
    let context_dir = client.host().directory("fendermint/");

    let container = client.container().build_opts(
        context_dir,
        ContainerBuildOptsBuilder::default()
            .dockerfile("docker/runner.Dockerfile")
            .build()?,
    );

    Ok(container)
}

async fn test_service() -> eyre::Result<()> {
    todo!();
    Ok(())
}

async fn prepare_fendermint_container(client: DaggerConn) -> eyre::Result<Container> {
    let context_dir = client.host().directory("fendermint");

    let container = client.container().build_opts(
        context_dir,
        ContainerBuildOptsBuilder::default()
            .dockerfile("docker/runner.Dockerfile")
            .build()?,
    );

    Ok(container)
}

fn cmd(s: impl AsRef<str>) -> Vec<String> {
    Vec::from_iter(s.as_ref().split_whitespace().map(|x| x.to_string()))
}

fn host_repo_root_dir(client: &DaggerConn) -> Directory {
    let repo_root_dir = client.host().directory_opts(
        ".",
        HostDirectoryOpts {
            exclude: Some(vec!["node_modules", "target"]),
            include: None,
        },
    );
    repo_root_dir
}

fn with_caches(container: Container, client: &DaggerConn) -> Container {
    let cache_volume_aptititude = client.cache_volume("apt-cache");
    let cache_volume_var_cache = client.cache_volume("apt-lists");
    let cache_volume_cargo = client.cache_volume("cargo");
    let cache_volume_rustup = client.cache_volume("rustup");
    let cache_volume_target = client.cache_volume("target");
    let cache_volume_solidity = client.cache_volume("solidity");
    let cache_volume_node_modules = client.cache_volume("npm");
    let cache_volume_pnpm_store = client.cache_volume("pnpm");
    let cache_volume_npm_store = client.cache_volume("pnpm");

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
    let container = container.with_mounted_cache("/root/.rustup", cache_volume_rustup.clone());

    container
}

fn define_contracts_container(
    client: DaggerConn,
    hrrd: Directory,
    hccd: Directory,
) -> Result<Container> {
    let compiled_contracts_dir = "/workdir/compiled_contracts";

    let opts = ContainerBuildOptsBuilder::default()
        .dockerfile("contracts/docker/builder.Dockerfile")
        .build()?;

    let container =     with_caches(client
    .container().from("docker.io/library/node:latest")
    , &client)
    .with_mounted_directory("/workdir", hrrd.clone())
    .with_mounted_directory(compiled_contracts_dir, hccd.clone())
    .with_exec(cmd("apt-get update -y"))
    .with_exec(cmd("apt-get install -y curl which"))
    // .build_opts(d.clone(), opts)
    .with_workdir("/workdir/contracts")
    .with_exec(cmd("ls -al"))
    .with_exec(cmd("npm install -g pnpm"))
    .with_exec(cmd("pnpm install"))
    .with_exec(vec!["sh", "-c", "curl -L https://foundry.paradigm.xyz | bash && /root/.foundry/bin/foundryup --install 0.3.0"])
    .with_exec(cmd("git submodule update --init --recursive"))
    .with_exec(cmd(format!("mkdir -p {compiled_contracts_dir}")))
    .with_env_variable_opts("PATH", "${PATH}:/root/.foundry/bin", ContainerWithEnvVariableOptsBuilder::default().expand(true).build()?)
    .with_exec_opts(cmd("echo \"${PATH}\""), ContainerWithExecOptsBuilder::default().expand(true).build()?)
    .with_exec(cmd("which forge"))
    .with_exec(cmd(format!("/root/.foundry/bin/forge -vvv build -C ./src/ --lib-paths ./lib/ --via-ir --sizes --skip test --out={compiled_contracts_dir}")));

    Ok(container)
}

fn define_crates_container(
    client: DaggerConn,
    hrrd: Directory,
    hccd: Directory,
) -> Result<Container> {
    let opts = ContainerBuildOptsBuilder::default()
        .dockerfile("fendermint/docker/builder.local.Dockerfile")
        .build()?;

    let container = with_caches(client.container().from("docker.io/rust:bookworm"), &client)
        .with_mounted_directory("/workdir", hrrd.clone())
        .with_mounted_directory("/workdir/_compiled_contracts", hccd)
        .with_workdir("/workdir")
        .with_exec(cmd("apt-get update"))
        .with_exec(cmd(
            "apt-get install -y build-essential clang cmake protobuf-compiler",
        ))
        .with_exec(cmd("cargo b -p fendermint_app -p ipc-cli"))
        .with_exec(cmd("ls -al /workdir/target/debug"))
        .with_exec(cmd("ls -al output"));

    Ok(container)
    // .build_opts(d.clone(), opts))
}

async fn prepare_fendermint_two_stage_build(client: DaggerConn) -> eyre::Result<Container> {
    let fendermint_dir = client.host().directory("fendermint");

    fs::create_dir_all("_compiled_contracts")?;
    let hrrd = host_repo_root_dir(&client);
    let hccd = hrrd.directory("_compiled_contracts");

    let contracts_gen = define_contracts_container(client.clone(), hrrd.clone(), hccd.clone())?;
    run(&contracts_gen).await?;

    let crates_def = define_crates_container(client.clone(), hrrd.clone(), hccd.clone())?;
    run(&crates_def).await?;

    let f_fendermint = crates_def.file("/workdir/target/debug/fendermint");
    let f_ipc = crates_def.file("/workdir/target/debug/ipc-cli");

    let car_extra = contracts_gen.file("/workdir/_compiled_contracts/custom_extra_actors.car");
    let car_builtin = contracts_gen.file("/workdir/_compiled_contracts/output/builtin_actors.car");

    let container = client.container();
    let runner = client
        .container()
        .with_file_opts(
            "/usr/local/bin/fendermint",
            f_fendermint,
            ContainerWithFileOptsBuilder::default()
                .permissions(0755_isize)
                .build()?,
        )
        .with_file_opts(
            "/usr/local/bin/ipc-cli",
            f_ipc,
            ContainerWithFileOptsBuilder::default()
                .permissions(0755_isize)
                .build()?,
        )
        .with_exec(cmd("ls -al"))
        // .with_file("/workdir/extra.car", car_extra)
        // .with_file("/workdir/builtin.car", car_builtin)
        .build_opts(
            hrrd,
            ContainerBuildOptsBuilder::default()
                .dockerfile("fendermint/docker/runner.Dockerfile")
                .build()?,
        );
    run(&runner).await?;

    Ok(runner)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
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

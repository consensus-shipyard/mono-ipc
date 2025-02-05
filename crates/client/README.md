# Fendermint

Fendermint is an effort to implement [IPC with Tendermint Core](https://docs.google.com/document/d/1cFoTdoRuYgxmWJia6K-b5vmEj-4MvyHCNvShZpyconU/edit#). There is a preliminary [roadmap](https://docs.google.com/spreadsheets/d/1eVwkHEPGNg0js8DKRDIX7sugf5JqbI9zRBddIqzJFfI/edit#gid=0) that lays out the tasks towards implementing subnets that run IPLD and FVM under the Filecoin rootnet, sharing components with the Lotus/Eudico based implementation.

## Prerequisites

* Install the basic requirements for IPC (see [README](../README.md#Prerequisites))

## Quick Start

- [Local testnets](../docs/client/localnet.md)

## Docs

Please have a look in the [docs](../docs/client/README.md) to see an overview of the project, how to run the components, and previous demos.

## IPC

Fendermint is built with support for [IPC](https://github.com/consensus-shipyard/ipc) by design. If you are looking to deploy the infrastructure Fendermint-based IPC subnet, refer to the [IPC main repo](https://github.com/consensus-shipyard/ipc), or have a look at the [IPC infrastructure docs](../docs/client/ipc.md).

## Testing

The following command runs unit and integration tests:

```bash
make test
```

while the next command builds docker images and runs an end-to-end test using the
[SimpleCoin](./client/rpc/examples/simplecoin.rs) and the
[ethers](./client/eth/api/examples/ethers.rs) examples:

```bash
make e2e
```

## IPC Solidity Actors

We generate Rust bindings for the Solidity actors we need to invoke from the [contracts](../contracts/) folder, some of which are deployed during the genesis process. The bindings live in [contracts/binding/](../contracts/binding), and are generated automatically during the build, or with the following command:

```bash
make gen
```

To run it, you will have to install [forge](https://book.getfoundry.sh/getting-started/installation).

The list of contracts for which we generate Rust bindings are in [build.rs](../contracts/binding/build.rs) and needs to be maintained by hand, for example if a new "diamond facet" is added to a contract, it has to be added here. Diamond facets also have to be added manually in [ipc.rs](./vm/actor_interface/src/ipc.rs) where the contracts which need to be deployed during genesis are described. These facets cannot be divined from the ABI description, so they have to be maintained explicitly.

To test whether the genesis process works, we can run the following unit test:

```bash
cargo test --release -p fendermint_vm_interpreter load_genesis
```

## Docker Build

The tests above build the images as a dependency, but you can build them any time with the following command:

```bash
make docker-build
```

See the [docker docs](./docker/README.md) for more details about the build.


### Pre-built Docker Image

The CI build publishes a [Docker image](https://github.com/consensus-shipyard/client/pkgs/container/fendermint) to Github Container Registry upon a successful build on the `main` branch. This is the same image as the one used in the End-to-End tests; it contains the built-in actor bundle and IPC Solidity actors, ready to be deployed during genesis.

The image can be pulled with the following command:

```bash
docker pull ghcr.io/consensus-shipyard/fendermint:latest
```

## Configuration

The [settings](./app/settings/) contains the configuration options for the `fendermint` application, for which the defaults are in the [config](./app/config/) directory. The [options](./app/options/) crate contains CLI parameters only; out of these only the `run` and the `eth` commands use the configuration settings.

`fendermint` can be configured using either a configuration file, or using environmnet variables.

### Config files

The default configuration is in [default.toml](./app/config/default.toml). This file is copied into the `fendermint` docker image and should not be edited, so that any further releases can provide any new keys with default values necessary for the application to start; instead the operator can provide further partial configuration files to override the defaults.

The [Settings::config](./app/settings/src/lib.rs) method expects the following files in a configuration directory:
* `default.toml` with the settings for the keys that have meaningful default values
* `<mode>.toml` is an optional file correponding to the `--mode` CLI option (by default `dev`); these are files that could be checked into Github e.g. to have values for production or test environments
* `local.toml` is also optional with potential overrides specific to the developer's machine

An example of this override is the [test.toml](./app/config/test.toml) file which contains keys that do not have meaningful defaults, for the purpose of testing the parsing logic.

#### Config directory

The location of the configuration directory is determined by [Options::config_dir](./app/options/src/lib.rs) in the following way: The optional `--config-dir` CLI parameter can be used to set it directly. If it's missing, the default is the `config` directory under the `--home-dir` CLI parameter, which by default is `~/.fendermint`. The `FM_CONFIG_DIR` and `FM_HOME_DIR` env vars are also recognised.

The `--config-dir` can be used in combination with `--home-dir` and the pre-build docker image to:
1. set `--home-dir` to a custom mounted volume where the data files can persist
2. set `--config-dir` to `/client/config`, which is where the `runner.Dockerfile` places the `default.toml` file
3. mount any custom config files next to `default.toml` in the container

This way the `default.toml` that the image comes with is always active. Without setting `--config-dir` the operator would have to put it in the mounted `--home-dir`.

Alternatively individual paths for the relative directories inside the config can be mounted, e.g. `/client/data`, `/client/snapshots`, etc.

### Environment Variables

Every setting in the config file can be overridden by an environment variable using the `FM_` prefix and `__` path separator.

For example if the TOML file has a setting such as this:

```toml
[abci]
[abci.listen]
host = "127.0.0.1
```

Then the corresponding environment variable would be `FM_ABCI__LISTEN__HOST=0.0.0.0`. Basically every `.` becomes `__`. If a field name contains `_` that stays as it is, e.g. `FM_RESOLVER__DISCOVERY__STATIC_ADDRESSES`.

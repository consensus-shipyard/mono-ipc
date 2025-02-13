---
description: Tutorial to deploy your first custom IPC subnet
---

# Deploy a subnet

Ready to test the waters with your first subnet? This guide will deploy a subnet with three local validators orchestrated by `ipc-cli`. This subnet will be anchored to the public [Calibration testnet](https://docs.filecoin.io/networks/calibration/details/). This will be a minimal example and may not work on all systems. The full documentation provides more details on each step.

Several steps in this guide involve running long-lived processes. In each of these cases, the guide advises starting a new _session_. Depending on your set-up, you may do this using tools like `screen` or `tmux`, or, if using a graphical environment, by opening a new terminal tab, pane, or window.

### Step 1: Prepare your system

#### Install the basic requirements for IPC:

{% tabs %}
{% tab title="Linux" %}
* Install system packages: `sudo apt install build-essential clang cmake pkg-config libssl-dev protobuf-compiler git curl`.
* Install Rust. See [instructions](https://www.rust-lang.org/tools/install).
* Install cargo-make: `cargo install --force cargo-make`.
* Install Docker. See [instructions](https://docs.docker.com/engine/install/ubuntu/).
* Install Foundry. See [instructions](https://book.getfoundry.sh/getting-started/installation).

Also install the following dependencies ([details](https://lotus.filecoin.io/lotus/install/prerequisites/#supported-platforms))

```
sudo apt update && sudo apt install build-essential libssl-dev mesa-opencl-icd ocl-icd-opencl-dev gcc git bzr jq pkg-config curl clang hwloc libhwloc-dev wget ca-certificates gnupg -y
```
{% endtab %}

{% tab title="MacOS" %}
* Install Xcode from App Store or terminal: `xcode-select --install`
* Install Homebrew. See [instructions](https://brew.sh/).
* Install dependencies: `brew install jq`
* Install Rust. See [instructions](https://www.rust-lang.org/tools/install). (if you have homebrew installed rust, you may need to uninstall that if you get errors in the build)
* Install Cargo make: `cargo install --force cargo-make`
* Install docker. See [instructions](https://docs.docker.com/desktop/install/mac-install/).
* Install foundry. See [instructions](https://book.getfoundry.sh/getting-started/installation).
{% endtab %}
{% endtabs %}

#### Building:

{% hint style="info" %}
NOTE: this step may take a while to compile, depending on OS version and hardware build
{% endhint %}

{% tabs %}
{% tab title="Linux" %}
```
# make sure that rust has the wasm32 target & use stable version of rustc
rustup target add wasm32-unknown-unknown
rustup default stable

# add your user to the docker group
sudo usermod -aG docker $USER && newgrp docker

# clone this repo and build
git clone https://github.com/consensus-shipyard/ipc.git
cd ipc
make

# building will generate the following binaries
./target/release/ipc-cli --version
./target/release/fendermint --version
```
{% endtab %}

{% tab title="MacOS" %}
```
# make sure that rust has the wasm32 target & use stable version of rustc
rustup target add wasm32-unknown-unknown
rustup default stable

# clone this repo and build
git clone https://github.com/consensus-shipyard/ipc.git
cd ipc/contracts
make gen
cd ..
cargo build --release

# building will generate the following binaries
./target/release/ipc-cli --version
./target/release/fendermint --version
```
{% endtab %}
{% endtabs %}

### Step 2: Initialise your config

* Initialise the config

{% tabs %}
{% tab title="Linux/MacOS" %}
```
alias ipc-cli="cargo run -q -p ipc-cli --release --"
ipc-cli config init
```
{% endtab %}
{% endtabs %}

This should have populated a default config file with all the parameters required to connect to calibration at `~/.ipc/config.toml`. Feel free to update this configuration to fit your needs.

The IPC stack is changing rapidly. To make sure you use the latest contracts deployed on Filecoin Calibration:

* Run `nano ~/.ipc/config.toml` to see your configuration

```
keystore_path = "~/.ipc"

[[subnets]]
id = "/r314159"

[subnets.config]
network_type = "fevm"
provider_http = "https://api.calibration.node.glif.io/rpc/v1"
gateway_addr = "<GATEWAY_ADDR>"
registry_addr = "<REGISTRY_ADDR>"
```

*   **Replace** the `gateway_addr` and `registry_addr` with the following values. Click on the badges below to take you to the source to copy and paste them or go to [this link](https://github.com/consensus-shipyard/ipc/blob/cd/contracts/deployments/r314159.json).

    [![Gateway Address](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fconsensus-shipyard%2Fipc%2Fcd%2Fcontracts%2Fdeployments%2Fr314159.json\&query=%24.gateway\_addr\&label=Gateway%20Address)](https://github.com/consensus-shipyard/ipc/blob/cd/contracts/deployments/r314159.json)

    [![Registry Address](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fconsensus-shipyard%2Fipc%2Fcd%2Fcontracts%2Fdeployments%2Fr314159.json\&query=%24.registry\_addr\&label=Registry%20Address)](https://github.com/consensus-shipyard/ipc/blob/cd/contracts/deployments/r314159.json)

### Step 3: Set up your wallets

Since we are setting up a subnet with multiple validators, we will create a set of wallets to spawn and interact within the subnet.

{% hint style="info" %}
TIP: Note down wallet and subnet addresses and keys as you go along
{% endhint %}

* Create four different wallets (we recommend a minimum of 4 for BFT security)

```
ipc-cli wallet new --wallet-type evm
ipc-cli wallet new --wallet-type evm
ipc-cli wallet new --wallet-type evm
ipc-cli wallet new --wallet-type evm
```

* You can optionally set one of the wallets as your default so you don't have to use the `--from` flag explicitly in some of the commands:

```
ipc-cli wallet set-default --address <DEFAULT_ETH_ADDR> --wallet-type evm
```

* Go to the [Calibration faucet](https://faucet.calibnet.chainsafe-fil.io/) and get some funds sent to each of your addresses

{% hint style="info" %}
NOTE: you may hit faucet rate limits. In that case, wait a few minutes or continue with the guide and come back to this before step 9. Alternatively, you can send funds from your primary wallet to your owner wallets.
{% endhint %}

{% hint style="info" %}
TIP: If you'd like to import an EVM account into Metamask, you can use export the private key using `ipc-cli wallet export --wallet-type evm --address <ADDRESS>`. More information is available in the [EVM IPC agent support docs](https://github.com/consensus-shipyard/ipc/blob/main/docs/ipc/usage.md#key-management).
{% endhint %}

### Step 4: Create a child subnet

* The next step is to create a subnet under `/r314159` calibration. Remember to set a default wallet or explicitly specify the wallet from which you want to perform the action with the `--from` flag.

```
ipc-cli subnet create --parent /r314159 --min-validator-stake 1 --min-validators 4 --bottomup-check-period 300 --from <PLEASE PUT ACCOUNT ADDRESS> --permission-mode collateral --supply-source-kind native
```

This will output your subnet ID, similar to the following:

<pre><code><strong>/r314159/t410fx2xy6x6idpy6yfywiilp6uitq4eerhpdr72wtmi
</strong></code></pre>

Make a note of the address of the subnet you created because you will use it below.&#x20;

### Step 5: Join the subnet

Before we deploy the infrastructure for the subnet, we will have to bootstrap the subnet and join from our validators, putting some initial collateral into the subnet and giving our validator address some initial balance in the subnet. For this, we need to send a `join` command from each of our validators from their validator owner addresses.

```
ipc-cli subnet join --from=<PLEASE PUT ADDRESS 1> --subnet=<PLEASE PUT SUBNET ID> --collateral=10 --initial-balance 1
ipc-cli subnet join --from=<PLEASE PUT ADDRESS 2> --subnet=<PLEASE PUT SUBNET ID> --collateral=10 --initial-balance 1
ipc-cli subnet join --from=<PLEASE PUT ADDRESS 3> --subnet=<PLEASE PUT SUBNET ID> --collateral=10 --initial-balance 1
ipc-cli subnet join --from=<PLEASE PUT ADDRESS 3> --subnet=<PLEASE PUT SUBNET ID> --collateral=10 --initial-balance 1
```

### Step 6: Deploy the infrastructure

First, we need to export the validator private keys for all wallets into separate files which we will use to set up a validator node.

```
ipc-cli wallet export --wallet-type evm --address <PLEASE PUT ADDRESS 1> --hex > ~/.ipc/validator_1.sk
ipc-cli wallet export --wallet-type evm --address <PLEASE PUT ADDRESS 2> --hex > ~/.ipc/validator_2.sk
ipc-cli wallet export --wallet-type evm --address <PLEASE PUT ADDRESS 3> --hex > ~/.ipc/validator_3.sk
ipc-cli wallet export --wallet-type evm --address <PLEASE PUT ADDRESS 4> --hex > ~/.ipc/validator_4.sk
```

Let's start our first validator which the rest of the validators will bootstrap from. Make sure you have docker running before running this command.

```
cargo make --makefile infra/fendermint/Makefile.toml \
    -e NODE_NAME=validator-1 \
    -e PRIVATE_KEY_PATH=<PLEASE PUT FULL PATH TO validator_1.sk> \
    -e SUBNET_ID=<PLEASE PUT SUBNET ID> \
    -e CMT_P2P_HOST_PORT=26656 \
    -e CMT_RPC_HOST_PORT=26657 \
    -e ETHAPI_HOST_PORT=8545 \
    -e RESOLVER_HOST_PORT=26655 \
    -e PARENT_GATEWAY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.gateway_addr'` \
    -e PARENT_REGISTRY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.registry_addr'` \
    -e FM_PULL_SKIP=1 \
    child-validator
```

Once the first validator is up and running, it will print out the relative information for this validator.&#x20;

{% hint style="info" %}
TIP: Highly recommend documenting that information which will be useful to bootstrap other validators, connect to the IPC subnet on MetaMask, etc.
{% endhint %}

```
#################################
#                               #
# Subnet node ready! 🚀         #
#                               #
#################################

Subnet ID:
	/r314159/t410f6b2qto756ox3qfoonq4ii6pdrylxwyretgpixuy

Eth API:
	http://0.0.0.0:8545

Chain ID:
	3684170297508395

Fendermint API:
	http://localhost:26658

CometBFT API:
	http://0.0.0.0:26657

CometBFT node ID:
	ca644ac3194d39a2834f5d98e141d682772c149b

CometBFT P2P:
	http://0.0.0.0:26656

IPLD Resolver Multiaddress:
	/ip4/0.0.0.0/tcp/26655/p2p/16Uiu2HAkwhrWn9hYFQMR2QmW5Ky7HJKSGVkT8xKnQr1oUGCkqWms
```

You'll need the final component of the `IPLD Resolver Multiaddress` (the `peer ID`) and the `CometBFT node ID` for the next nodes to start.

*   _**BOOTSTRAPS**_: \<CometBFT node ID for validator1>@validator-1-cometbft:26656

    ```
    // An example
    ca644ac3194d39a2834f5d98e141d682772c149b@validator-1-cometbft:26656
    ```
*   _**RESOLVER\_BOOTSTRAPS**_: /dns/validator-1-fendermint/tcp/26655/p2p/\<Peer ID in IPLD Resolver Multiaddress>

    <pre><code>// An example
    <strong>/dns/validator-1-fendermint/tcp/26655/p2p/16Uiu2HAkwhrWn9hYFQMR2QmW5Ky7HJKSGVkT8xKnQr1oUGCkqWms
    </strong></code></pre>

Now, run the 2nd validator in a separate terminal.&#x20;

```
cargo make --makefile infra/fendermint/Makefile.toml \
    -e NODE_NAME=validator-2 \
    -e PRIVATE_KEY_PATH=<PLEASE PUT FULL PATH TO validator_2.sk> \
    -e SUBNET_ID=<PLEASE PUT SUBNET ID> \
    -e CMT_P2P_HOST_PORT=26756 \
    -e CMT_RPC_HOST_PORT=26757 \
    -e ETHAPI_HOST_PORT=8645 \
    -e RESOLVER_HOST_PORT=26755 \
    -e BOOTSTRAPS=<PLEASE PUT COMETBFT NODE ID of VALIDATOR-1>@validator-1-cometbft:26656 \
    -e RESOLVER_BOOTSTRAPS=/dns/validator-1-fendermint/tcp/26655/p2p/<PLEASE PUT PEER_ID of VALIDATOR-1> \
    -e PARENT_GATEWAY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.gateway_addr'` \
    -e PARENT_REGISTRY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.registry_addr'` \
    child-validator
```

Now, the 3rd:

```
cargo make --makefile infra/fendermint/Makefile.toml \
    -e NODE_NAME=validator-3 \
    -e PRIVATE_KEY_PATH=<PLEASE PUT FULL PATH TO validator_3.sk> \
    -e SUBNET_ID=<PLEASE PUT SUBNET ID> \
    -e CMT_P2P_HOST_PORT=26856 \
    -e CMT_RPC_HOST_PORT=26857 \
    -e ETHAPI_HOST_PORT=8745 \
    -e RESOLVER_HOST_PORT=26855 \
    -e BOOTSTRAPS=<PLEASE PUT COMETBFT NODE ID of VALIDATOR-1>@validator-1-cometbft:26656 \
    -e RESOLVER_BOOTSTRAPS=/dns/validator-1-fendermint/tcp/26655/p2p/<PLEASE PUT PEER_ID of VALIDATOR-1> \
    -e PARENT_GATEWAY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.gateway_addr'` \
    -e PARENT_REGISTRY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.registry_addr'` \
    child-validator
```

And finally, the 4th:

```
cargo make --makefile infra/fendermint/Makefile.toml \
    -e NODE_NAME=validator-4 \
    -e PRIVATE_KEY_PATH=<PLEASE PUT FULL PATH TO validator_4.sk> \
    -e SUBNET_ID=<PLEASE PUT SUBNET ID> \
    -e CMT_P2P_HOST_PORT=26956 \
    -e CMT_RPC_HOST_PORT=26957 \
    -e ETHAPI_HOST_PORT=8845 \
    -e RESOLVER_HOST_PORT=26955 \
    -e BOOTSTRAPS=<PLEASE PUT COMETBFT NODE ID of VALIDATOR-1>@validator-1-cometbft:26656 \
    -e RESOLVER_BOOTSTRAPS=/dns/validator-1-fendermint/tcp/26655/p2p/<PLEASE PUT PEER_ID of VALIDATOR-1> \
    -e PARENT_GATEWAY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.gateway_addr'` \
    -e PARENT_REGISTRY=`curl -s https://raw.githubusercontent.com/consensus-shipyard/ipc/cd/contracts/deployments/r314159.json | jq -r '.registry_addr'` \
    child-validator
```

{% hint style="info" %}
NOTE:

* Use full path to PRIVATE\_KEY\_PATH, don't path with "\~"
* Do not change values of any port from the ones provided unless you have to
* If you are deploying all validators on a single server, ports will need to be different, as shown in above examples. If you are deploying them from different servers, the ports can be similar.
{% endhint %}

### Step 7: Interact with your subnet using the IPC CLI

* Make sure `~/.ipc/config.toml` contains the configuration of your subnet in the "Subnet template" section. Uncomment the section and populate the corresponding fields

```
# Subnet template - uncomment and adjust before using
[[subnets]]
id = <PUT YOUR SUBNET ID>

[subnets.config]
network_type = "fevm"
provider_http = "http://localhost:8545/"
gateway_addr = "0x77aa40b105843728088c0132e43fc44348881da8"
registry_addr = "0x74539671a1d2f1c8f200826baba665179f53a1b7"
```

{% hint style="info" %}
NOTE: The ETH addresses for `gateway_addr` and `registry_addr` used when they are deployed in genesis in a child subnet by Fendermint are `0x77aa40b105843728088c0132e43fc44348881da8` and `0x74539671a1d2f1c8f200826baba665179f53a1b7`, respectively, so no need to change them.
{% endhint %}

* Fetch the balances of your wallets using the following command. The result should show the initial balance that you have included for your validator address in genesis:

```
ipc-cli wallet balances --wallet-type evm --subnet=<SUBNET_ID>
```

### Step 8: Run a relayer

IPC relies on the role of a specific type of peer on the network called the relayers that are responsible for submitting bottom-up checkpoints that have been finalized in a child subnet to its parent.

This process is key for the commitment of child subnet checkpoints in the parent, and the execution of bottom-up cross-net messages. Without relayers, cross-net messages will only flow from top levels of the hierarchy to the bottom, but not the other way around.

* Run the relayer process passing the 0x address of the submitter account:

```
ipc-cli checkpoint relayer --subnet <SUBNET_ID> --submitter <RELAYER_ADDR>
```

#!/bin/bash

# IPC Quick Start Script
# See also https://github.com/consensus-shipyard/ipc/blob/main/docs/ipc/quickstart-calibration.md

# Known issues:
# 1. Need to previously manual enable sudo without password on the host
# 2. You may need to rerun the script after docker installation for the first time

# Exit on any error
set -e

# Print commands as we execute
set -x

PREFIX='------'
IPC_FOLDER=${HOME}/ipc
IPC_CLI=${IPC_FOLDER}/target/release/ipc-cli
IPC_CONFIG_FOLDER=${HOME}/.ipc

wallet_addresses=()
address_pubkeys=()
CMT_P2P_HOST_PORTS=(26656 26756 26856)
CMT_RPC_HOST_PORTS=(26657 26757 26857)
ETHAPI_HOST_PORTS=(8545 8645 8745)
RESOLVER_HOST_PORTS=(26655 26755 26855)

if (($# != 1)); then
  echo "Arguments: <commit hash to checkout in the repo>"
  exit 1
fi

commit_hash=$1

# Step 1: Prepare system for building and running IPC

# Step 1.1: Install build dependencies
echo "${PREFIX} Installing build dependencies..."
sudo apt update && sudo apt install build-essential libssl-dev mesa-opencl-icd ocl-icd-opencl-dev gcc git bzr jq pkg-config curl clang hwloc libhwloc-dev wget ca-certificates gnupg -y

# Step 1.2: Install rust + cargo
echo "$PREFIX Check rustc & cargo..."
if which cargo ; then
  echo "$PREFIX rustc & cargo already installed."
else
  echo "$PREFIX Need to install rustc & cargo"
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  # Refresh env
  source ${HOME}/.bashrc
fi

# Step 1.3: Install cargo-make and toml-cli
# Install cargo make
echo "$PREFIX Installing cargo-make"
cargo install cargo-make
# Install toml-cli
echo "$PREFIX Installing toml-cli"
cargo install toml-cli

# Step 1.3: Install Foundry
echo "$PREFIX Check foundry..."
if which foundryup ; then
  echo "$PREFIX foundry is already installed."
else
  echo "$PREFIX Need to install foundry"
  curl -L https://foundry.paradigm.xyz | bash
  foundryup
fi

# Step 1.4: Install docker
echo "$PREFIX check docker"
if which docker ; then
  echo "$PREFIX docker is already installed."
else
  echo "$PREFIX Need to install docker"
  # Add Docker's official GPG key:
  sudo apt-get update
  sudo apt-get install ca-certificates curl
  sudo install -m 0755 -d /etc/apt/keyrings
  sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
  sudo chmod a+r /etc/apt/keyrings/docker.asc

  # Add the repository to Apt sources:
  echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
  sudo apt-get update
  sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

  # Remove the need to use sudo
  getent group docker || sudo groupadd docker
  sudo usermod -aG docker $USER
  newgrp docker

  # Test running docker without sudo
  docker ps
fi

# Make sure we re-read the latest env before finishing dependency instalation.
source ${HOME}/.bashrc

# Step 2: Prepare code repo and build ipc-cli
echo "$PREFIX Preparing ipc repo..."
cd $HOME
if ! ls $IPC_FOLDER ; then
  git clone https://github.com/consensus-shipyard/ipc.git
  git checkout $commit_hash
fi

echo "$PREFIX Building ipc contracts..."
cd ${IPC_FOLDER}/contracts
make build

echo "$PREFIX Building ipc-cli..."
cd ${IPC_FOLDER}/ipc
make build

# Step 3: Prepare wallet by creating new wallet
#echo "$PREFIX Creating 3 address in wallet..."
#for i in {1..3}
#do
#    addr=$($IPC_CLI wallet new --wallet-type evm | tr -d '"')
#    wallet_addresses+=($addr)
#    echo "Wallet $i address: $addr"
#done

# Step 3 (alternative): Prepare wallet by using existing wallet
echo "$PREFIX Using 3 address in wallet..."
for i in {0..2}
do
  addr=$(cat ${IPC_CONFIG_FOLDER}/evm_keystore.json | jq .[$i].address | tr -d '"')
  wallet_addresses+=($addr)
  echo "Wallet $i address: $addr"
done

default_wallet_address=${wallet_addresses[0]}
echo "Default wallet address: $default_wallet_address"

# Step 4: Create a subnet
#echo "$PREFIX Creating a child subnet..."
#create_subnet_output=$($IPC_CLI subnet create --parent /r314159 --min-validators 3 --min-validator-stake 1 --bottomup-check-period 30 --from $default_wallet_address --permission-mode 0 --supply-source-kind 0 2>&1)
#echo $create_subnet_output
#subnet_id=$(echo $create_subnet_output | sed 's/.*with id: \([^ ]*\).*/\1/')
#
#echo "Created subnet ID: $subnet_id"

# Step 4 (alternative): Use an already-created subnet
subnet_id=/r314159/t410fqmlmt6usaeewvxdj3slk6t57ti776ycqsjp2lsa

# Step 5: Generate pubkeys from addresses
echo "$PREFIX Generating pubkey for wallet addresses... $default_wallet_address"
for i in {0..2}
do
  pubkey=$($IPC_CLI wallet pub-key --wallet-type evm --address ${wallet_addresses[i]})
  echo "Wallet $i address's pubkey: $pubkey"
  address_pubkeys+=($pubkey)
done

# Step 6: Join subnet for addresses in wallet
#echo "$PREFIX Join subnet for addresses in wallet..."
#for i in {0..2}
#do
#  echo "Joining subnet ${subnet_id} for address ${wallet_address[i]}"
#  $IPC_CLI subnet join --from ${wallet_addresses[i]} --subnet $subnet_id --public-key ${address_pubkeys[i]} --initial-balance 1 --collateral 10
#done

# Step 6 (alternative): Assume we already let our addresses join in the subnet

# Step 7: Start validators
# Step 7.1: Export validator private keys into files
for i in {0..2}
do
  $IPC_CLI wallet export --wallet-type evm --address ${wallet_addresses[i]} --hex -o ${IPC_CONFIG_FOLDER}/validator_${i}.sk
  echo "Export private key for ${wallet_addresses[i]} to ${IPC_CONFIG_FOLDER}/validator_${i}.sk"
done

# Step 7.2 (optional): Rebuild fendermint docker
# cd ${IPC_FOLDER}/fendermint
# make docker-build

# Step 7.3: Read parent net gateway address and registry address
echo "$PREFIX Reading parent gateway and registry address"
parent_gateway_address=$(toml get ${IPC_CONFIG_FOLDER}/config.toml subnets[0].config.gateway_addr | tr -d '"')
parent_registry_address=$(toml get ${IPC_CONFIG_FOLDER}/config.toml subnets[0].config.registry_addr | tr -d '"')

# Step 7.4: Start the bootstrap validator node
echo "$PREFIX Start the bootstrap validator node"
cd ${IPC_FOLDER}
bootstrap_output=$(cargo make --makefile infra/fendermint/Makefile.toml \
        -e NODE_NAME=validator-1 \
        -e PRIVATE_KEY_PATH=${IPC_CONFIG_FOLDER}/validator_1.sk \
        -e SUBNET_ID=${subnet_id} \
        -e CMT_P2P_HOST_PORT=${CMT_P2P_HOST_PORTS[0]} \
        -e CMT_RPC_HOST_PORT=${CMT_RPC_HOST_PORTS[0]} \
        -e ETHAPI_HOST_PORT=${ETHAPI_HOST_PORTS[0]} \
        -e RESOLVER_HOST_PORT=${RESOLVER_HOST_PORTS[0]} \
        -e PARENT_REGISTRY=${parent_registry_address} \
        -e PARENT_GATEWAY=${parent_gateway_address} \
        -e FM_PULL_SKIP=1 \
        child-validator 2>&1)
echo "$bootstrap_output"
bootstrap_node_id=$(echo "$bootstrap_output" | sed -n '/CometBFT node ID:/ {n;p}' | tr -d "[:blank:]")
echo "Bootstrap node started. Node id ${bootstrap_node_id}"

# Step 7.5: Start other validator node
echo "$PREFIX Start the other validator nodes"
cd ${IPC_FOLDER}
for i in {1..2}
do
  cargo make --makefile infra/fendermint/Makefile.toml \
      -e NODE_NAME=validator-$(($i+1)) \
      -e PRIVATE_KEY_PATH=${IPC_CONFIG_FOLDER}/validator_1.sk \
      -e SUBNET_ID=${subnet_id} \
      -e CMT_P2P_HOST_PORT=${CMT_P2P_HOST_PORTS[i]} \
      -e CMT_RPC_HOST_PORT=${CMT_RPC_HOST_PORTS[i]} \
      -e ETHAPI_HOST_PORT=${ETHAPI_HOST_PORTS[i]} \
      -e RESOLVER_HOST_PORT=${RESOLVER_HOST_PORTS[i]} \
      -e BOOTSTRAPS=${bootstrap_node_id} \
      -e PARENT_REGISTRY=${parent_registry_address} \
      -e PARENT_GATEWAY=${parent_gateway_address} \
      -e FM_PULL_SKIP=1 \
      child-validator
done

# Step 8: Write down key properties into local file
echo $subnet_id > ~/running_subnet_id
echo $bootstrap_node_id > ~/running_bootstrap_node_id
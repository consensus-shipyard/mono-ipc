#!/usr/bin/env bash
set -euo pipefail

# Cleanup function: Terminates the Anvil process if still running.
cleanup() {
  echo "Cleaning up..."
  if ps -p "${ANVIL_PID:-0}" > /dev/null 2>&1; then
    echo "Terminating Anvil process (PID: $ANVIL_PID)"
    kill "$ANVIL_PID" || echo "Failed to kill Anvil process"
  fi
}
trap cleanup EXIT INT TERM

# Set MNEMONIC (from environment or default).
export MNEMONIC="${MNEMONIC:-test test test test test test test test test test test junk}"
echo "Using MNEMONIC: $MNEMONIC"

chain_id=31415926
port=8545

# Start Anvil with the fixed mnemonic and custom chain settings.
echo "Starting Anvil..."
anvil --host 0.0.0.0 --port "$port" --chain-id "$chain_id" --mnemonic "$MNEMONIC" --state /out/anvil_state.json 2>&1 | tee /tmp/anvil.log &
ANVIL_PID=$!
echo "Anvil started with PID: $ANVIL_PID"

# Wait until Anvil's RPC endpoint is ready (with a timeout).
echo "Waiting for Anvil RPC to be ready..."
TIMEOUT=30
while ! curl -s http://localhost:8545 > /dev/null; do
    sleep 1
    TIMEOUT=$((TIMEOUT - 1))
    if [ $TIMEOUT -le 0 ]; then
      echo "Error: Timeout waiting for Anvil RPC to be ready."
      exit 1
    fi
done
echo "Anvil is up!"

gateway_file="/out/GatewayAddress.txt"
subnet_file="/out/SubnetAddress.txt"

# Check if both files exist.
if [ -f "$gateway_file" ] && [ -f "$subnet_file" ]; then
  echo "Skipping deployment because $gateway_file and $subnet_file exists. Container will remain running as long as Anvil is active."
  wait "$ANVIL_PID"

  exit 0
fi


# Derive the first account's private key from the mnemonic using Node.js.
echo "Deriving private key from mnemonic..."
export PRIVATE_KEY=$(node <<'EOF'
const { ethers } = require('ethers');
if (!process.env.MNEMONIC) {
  console.error("MNEMONIC environment variable not set");
  process.exit(1);
}
const wallet = ethers.Wallet.fromMnemonic(process.env.MNEMONIC);
console.log(wallet.privateKey);
EOF
)
if [ -z "$PRIVATE_KEY" ]; then
  echo "Error: Failed to derive private key."
  exit 1
fi
echo "Derived Private Key: $PRIVATE_KEY"

export RPC_URL="http://localhost:$port"
export CHAIN_ID=$chain_id

# Deploy contracts using Hardhat.
# Disable exit-on-error around this command so that failure won't immediately kill the script.
echo "Deploying contracts with Hardhat..."
set +e
deployment_output=$(pnpm exec hardhat deploy-stack --network localnet 2>&1)
deploy_exit=$?
set -e

if [ $deploy_exit -ne 0 ]; then
    echo "Error: Hardhat deployment failed. Output:"
    echo "$deployment_output"
    exit 1
else
    # Attempt to extract the deployed contract addresses.
    gateway_address=$(echo "$deployment_output" | awk '/GatewayDiamond deployed at/ { print $NF }')
    subnet_address=$(echo "$deployment_output" | awk '/SubnetRegistryDiamond deployed at/ { print $NF }')
    if [ -z "$gateway_address" ] || [ -z "$subnet_address" ]; then
      echo "Warning: Could not extract deployed contract addresses. Full deployment output:"
      echo "$deployment_output"
      exit 1
    else
      echo "Deployment successful."
      echo "GatewayAddress: $gateway_address"
      echo "SubnetAddress: $subnet_address"
      mkdir -p /out
      echo "$gateway_address" > "$gateway_file"
      echo "$subnet_address" > "$subnet_file"
    fi
fi

# Keep the container running indefinitely by waiting on the Anvil process.
echo "Deployment complete. Container will remain running as long as Anvil is active."
wait "$ANVIL_PID"

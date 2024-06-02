#!/usr/bin/bash

set -euxo pipefail

DASHES='------'
IPC_FOLDER=${HOME}/ipc
IPC_CLI=${IPC_FOLDER}/target/release/ipc-cli
IPC_CONFIG_FOLDER=${HOME}/.ipc
LINKED_TOKEN_FOLDER=${HOME}/ipc/extras/linked-token
DOT_ENV_FILE=${HOME}/ipc/extras/linked-token/.env
DOT_ENV_TEMPLATE=${HOME}/ipc/extras/linked-token/.env.template

if (($# != 1)); then
  echo "Arguments: <branch_name>: Specify github remote branch name to use to deploy. Or use 'local' (without quote) to indicate using local repo instead. If not provided, will default to main branch"
  head_ref=main
  local_deploy=false
else
  if [ $1 = "local" ]; then
    local_deploy=true
  else
    local_deploy=false
    head_ref=$1
  fi
fi

# Step 1: Make sure dependencies are installed
echo "$DASHES Installing dependencies..."
cd $IPC_FOLDER/extras/tools/fvm-eth-address-converter
npm install

# Step 2: Prepare wallet address
echo "$DASHES Preparing wallet address..."
linked_token_private_key=$(cat ${IPC_CONFIG_FOLDER}/evm_keystore.json | jq -er .[3].private_key)
linked_token_addr=$(cast wallet address --private-key=$linked_token_private_key)
echo "wallet address for linked token test is:"
echo $linked_token_addr
default_wallet_address=$(cat ${IPC_CONFIG_FOLDER}/evm_keystore.json | jq -er .[0].address)
echo "default wallet address is:"
echo $default_wallet_address

# Step 3: Configure the dot env file
echo "$DASHES Configuring .env file for linked token deployment..."
cp $DOT_ENV_TEMPLATE $DOT_ENV_FILE
calib_net_gateway_address=$(toml get ~/.ipc/config.toml subnets[0].config.gateway_addr | tr -d '"')
subnet_id=$(toml get ~/.ipc/config.toml subnets[1].id | tr -d '"')
base_subnet_id=$(basename $subnet_id)
cd $IPC_FOLDER/extras/tools/fvm-eth-address-converter
subnet_id_as_eth_addr=$(npx ts-node fvm-addr-to-eth-addr.ts $base_subnet_id)
# Write config to dot env file
echo "export SUBNET_PRIVATE_KEY=$linked_token_private_key" >> $DOT_ENV_FILE
echo "export ORIGIN_NET_PRIVATE_KEY=$linked_token_private_key" >> $DOT_ENV_FILE
echo "export ORIGIN_NET_GATEWAY=$calib_net_gateway_address" >> $DOT_ENV_FILE
echo "export SUBNET_ROUTE_IN_ETH_FORMAT=$subnet_id_as_eth_addr" >> $DOT_ENV_FILE
# Preview the dot env file
echo "Final .env file:"
cat $DOT_ENV_FILE

# Step 4: Fund address in subnet
echo "$DASHES Funding address in subnet..."
$IPC_CLI cross-msg fund \
--subnet $subnet_id \
--from $default_wallet_address \
--to $linked_token_addr \
50


# Step 5: Reset Linked Token Config
cd $LINKED_TOKEN_FOLDER
cat config.json || true
rm -vrf config.json


# Step 6a: Deploy the USDCTest contract to calibration net
echo "$DASHES Deploying USDCTest contract to calibration net..."
cd $LINKED_TOKEN_FOLDER
make deploy-usdctest || true

cat config.json

# Step 6b: Mint USDCTest tokens on calibration net
echo "$DASHES Minting USDCTest tokens on calibration net..."
cd $LINKED_TOKEN_FOLDER
make mint-usdc || true

# Step 7: Check tokens has been minted
echo "$DASHES Checking token balance..."
sleep 10
cd $LINKED_TOKEN_FOLDER
for retry in {0..20}
do
  check_balance_output=$(make check-balance)
  balance=$(echo $check_balance_output | grep -oP '0x[\S]*')
  if [ $balance = '0x0000000000000000000000000000000000000000000000000000000000000000' ]; then
    if (( $retry < 20 )); then
      echo "Balance $balance is still zero. Will wait and retry...(attempt $retry)"
      sleep 10
    else
      echo "Balance $balance keeps at zero. Token minting failed."
      exit 1
    fi
  else
    echo "Token mint succeeded. Balance is $balance for addr $default_wallet_address"
    break
  fi
done

# Step 8a: Deploy token replica implementation contract to subnet
echo "$DASHES Deploying token replica implementation contract to subnet..."
sleep 30
make deploy-replica-implementation

# Step 8b: Deploy token controller implementation contract to calibration net
echo "$DASHES Deploying token controller implementation contract to calibration net..."
make deploy-controller-implementation || true


# Step 9a: Deploy token replica proxy contract to subnet
echo "$DASHES Deploying token replica proxy contract to subnet..."
sleep 30
make deploy-replica-proxy

# Step 9b: Deploy token controller proxy contract to calibration net
echo "$DASHES Deploying token controller proxy contract to calibration net..."
sleep 30
make deploy-controller-proxy

# Step 10: Initialize contracts
echo "$DASHES Linking replicat contract on subnet..."
make link-replica
echo "$DASHES Linking controller contract on calibration net..."
make link-controller || true

# Now all contracts have been deployed and initialized. We will now start testing
# the interaction with the contracts.

# Step 11: Approve the USDCTest token to be transferred.
echo "$DASHES Approving token (on calibration net) to be transferred..."
make approve-token || true

# Step 12: Transfer the USDCTest token from calibration to subnet.
echo "$DASHES Depositing token from calibration net to subnet..."
sleep 30
make deposit-token || true

# Step 13: Verify that token balance has been moved.
echo "$DASHES Checking token balance after deposit..."
sleep 10
for retry in {0..20}
do
  check_balance_output=$(make check-balance)
  balance=$(echo $check_balance_output | grep -oP '0x[\S]*')
  if [ $balance != '0x0000000000000000000000000000000000000000000000000000000000000000' ]; then
    if (( $retry < 20 )); then
      echo "Balance $balance has not been reduced. Will wait and retry...(attempt $retry)"
      sleep 10
    else
      echo "Balance $balance was not reduced. Token deposition failed."
      exit 1
    fi
  else
    echo "Token balance has reduced. Balance is now $balance for addr $default_wallet_address"
    break
  fi
done

# Step 14: Verify that token balance has been moved to replica contract.
# (This verifies top-down finality propagation from calibration net to subnet.)
echo "$DASHES Checking token balance on replica contract after deposit..."
sleep 10
for retry in {0..100}
do
  check_balance_output=$(make check-replica-balance)
  balance=$(echo $check_balance_output | grep -oP '0x[\S]*')
  if [ $balance = '0x0000000000000000000000000000000000000000000000000000000000000000' ]; then
    if (( $retry < 100 )); then
      echo "Balance $balance has not been increased in replica contract. Will wait and retry...(attempt $retry)"
      sleep 10
    else
      echo "Balance $balance was not increased in replica contract. Token deposition failed."
      exit 1
    fi
  else
    echo "Token balance has been increased in replica contract. Balance is now $balance"
    break
  fi
done

# TODO: Verify bottom-up checkpointing by withdrawing the USDCToken

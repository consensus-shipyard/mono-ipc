---
description: This page lists all the ipc-cli commands to interact with IPC subnets.
---

# ipc-cli Usage

{% hint style="info" %}
For background and setup information, make sure to start with the [README](https://github.com/consensus-shipyard/ipc/blob/main/README.md) of IPC on GitHub.
{% endhint %}

If you have IPC installed on your machine, you should be able to run `ipc-cli --help` to check all the available commands.

```sh
ipc-cli --help

The IPC agent command line tool

Usage: ipc-cli [OPTIONS] [COMMAND]

Commands:
  config      config related commands
  subnet      subnet related commands such as create, join and etc
  wallet      wallet related commands
  cross-msg   cross network messages related commands
  checkpoint  checkpoint related commands
  util        util commands
  help        Print this message or the help of the given subcommand(s)

```

### Configuration

#### IPC initialization

```sh
ipc-cli config init
```

This command will initialize a new empty config file under `~/.ipc` with all parameters required to connect to the IPC rootnet network.

### Subnet Management

#### List active subnets

```
ipc-cli subnet list --subnet=<PARENT_SUBBNET_ID>
```

You can check all the active child subnets on a specific parent subnet.

This command only shows subnets that have been registered to the gateway, i.e. that have provided enough collateral to participate in the IPC protocol and haven't been killed. It is not an exhaustive list of all of the subnet actors deployed over the network.

<pre class="language-sh"><code class="lang-sh"># Example execution
<strong>$ ipc-cli subnet list --subnet /r314159
</strong>/r314159/t410fmdbc3kcv4gody6drgztmgwnzs2ryzwiazjzu5pq - status: Active, collateral: 5.0 FIL, circ.supply: 21.0 FIL, genesis: 1069882
</code></pre>

#### Create a child subnet

{% code overflow="wrap" %}
```
ipc-cli subnet create
    --parent <parent-subnet-id>
    --min-validator <MIN_VALIDATORS>
    --min-validator-stake <MIN_VALIDATOR_STAK>
    --bottomup-check-period <BOTTOMUP_CHECK_PERIO>
```
{% endcode %}

This command will create a subnet and create a corresponding contract based on the parameters specified with it. Make a note of the subnet-id for the subnet just created.

```sh
# Example execution
$ ipc-cli subnet create --parent /r314159 --min-validators 3 --min-validator-stake 10 --bottomup-check-period 30
[ipc_cli::commands::subnet::create] created subnet actor with id: /r314159/t410fylyzufn7lfg3q6zxpt6cdvq4yyiwm4tkaged2oy
```

#### Join a subnet as a validator

{% code overflow="wrap" %}
```sh
ipc-cli subnet join
    --subnet <subnet-id>
    --collateral <collateral_amount>
    --public-key <public_key_validator_addr>
    --initial-balance <genesis-balance>
```
{% endcode %}

This command specifies the subnet to join, the amount of collateral to provide, and the public key of the `--from` address that is joining as a validator.

<pre class="language-sh"><code class="lang-sh"># Example execution
<strong>$ ipc-cli subnet join --subnet /r314159/t410fh4ywg4wvxcjzz4vsja3uh4f53johc2lf5bpjo6i --collateral 10 --public-key 043385c3b9ab8a697cd7bec6ca623cbdd0fea1293e8b464df825b104eb58a44cc8efacc6a3482b866b85ecdf734b5d4ef5495737deb348625ce6a35536142d2955
</strong></code></pre>

To join a subnet and also include some initial balance for the validator in the subnet, you can add the `--initial-balance` flag with the balance to be included in the genesis.

```sh
# Example execution
$ ipc-cli subnet join --subnet=/r314159/t410fh4ywg4wvxcjzz4vsja3uh4f53johc2lf5bpjo6i --collateral=1 --public-key=043385c3b9ab8a697cd7bec6ca623cbdd0fea1293e8b464df825b104eb58a44cc8efacc6a3482b866b85ecdf734b5d4ef5495737deb348625ce6a35536142d2955 --initial-balance 0.5
```

#### Leave a subnet

```sh
ipc-cli subnet leave --subnet <subnet-id>
```

Leaving a subnet will release the collateral for the validator and remove all the validation rights from its account. This means that if you have a validator running in that subnet, its validation process will immediately terminate.

<pre class="language-sh"><code class="lang-sh"><strong># Example execution
</strong>$ ipc-cli subnet leave --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq
</code></pre>

#### Stake more collateral as a validator

```
ipc-cli subnet stake --subnet <subnet-id> --collateral <collateral_amount>
```

After initially joining a subnet with specified collateral, a validator can stake more collaterals to the subnet.

```sh
# Example execution
$ ipc-cli subnet stake --subnet=/r314159/t410fh4ywg4wvxcjzz4vsja3uh4f53johc2lf5bpjo6i --collateral=10
ipc_provider::manager::evm::manager] interacting with evm subnet contract: 0x60c2…d900 with collateral: 10000000000000000000
```

#### Unstake collateral from a subnet

```sh
ipc-cli subnet unstake --subnet <subnet-id> --collateral <collateral_amount>

# Example execution
$ ipc-cli subnet unstake --subnet /r314159/t410fmdbc3kcv4gody6drgztmgwnzs2ryzwiazjzu5pq --collateral 2
```

#### Claim the unstaked collateral

```
ipc-cli subnet claim --subnet <subnet-id>
```

Validators need to claim their collateral after they reduce collateral in the subnet through `unstake`

```sh
# Example execution
$ ipc-cli subnet claim --subnet=/r314159/t410fh4ywg4wvxcjzz4vsja3uh4f53johc2lf5bpjo6i
```

{% hint style="info" %}
Changes in collateral and the power table are not reflected immediately in the parent. They need to be confirmed in the execution of the next bottom-up checkpoint, so until this happens, even if there has been a change in collateral, you may not be the change immediately when running `ipc-cli subnet list`. This impacts any change to the collateral of validators, i.e. `stake`, `unstake` and `leave` commands.

To inspect the changes to the power table that have been performed between two epochs you can use the following command:

```
ipc-cli checkpoint list-validator-changes --from-epoch=<START_EPOCH> --to-epoch=<END_EPOCH>
```
{% endhint %}

#### Transfer tokens within a subnet

{% code overflow="wrap" %}
```sh
ipc-cli subnet send-value
    --subnet <subnet-id>
    [--from <from-addr>]
    --to <to-addr>
    <value>
```
{% endcode %}

You can use this command to send tokens between addresses of the same subnet. If `--from` is not specified, `ipc-cli` will send tokens from the default wallet address.

```sh
# Example execution
$ ipc-cli subnet send-value --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq --to 0x406a7a1d002b71ece175cc7e067620ae5b58e9ec 10
```

### Wallet Key Management

The `ipc-cli` has an EVM-compatible wallet that it uses to sign transactions and interact with IPC on behalf of specific addresses. This wallet type can also be used with FEVM.

#### Create new address

```
ipc-cli wallet new -w <wallet-type>
```

This command will create a wallet, and store the key information in `~/.ipc/evm_keystore.json`. You can create an EVM-compatible wallet which can also used with FVM (Filecoin Virtual Machine).

```sh
# Sample execution
$ ipc-cli wallet new --wallet-type evm
"0x406a7a1d002b71ece175cc7e067620ae5b58e9ec"
```

#### Check wallet balance

```sh
ipc-cli wallet balances --wallet-type <wallet-type> --subnet <subnet-id>
```

You can check the token balance in your wallet addresses for any active subnet configured in the `ipc-cli`.

<pre class="language-sh"><code class="lang-sh"># Sample execution
<strong>$ ipc-cli wallet balances --wallet-type evm --subnet /r314159
</strong>0x406a7a1d002b71ece175cc7e067620ae5b58e9ec - Balance: 100
</code></pre>

#### Set default wallet address

```
ipc-cli wallet set-default --wallet-type <wallet-type> --address <EVM-ADDRESS>
```

You can set a default address for your wallet so it is always the one used when the `--from` flag is not explicitly set.

```sh
# Sample execution
$ ipc-cli wallet set-default --wallet-type evm --address 0x406a7a1d002b71ece175cc7e067620ae5b58e9ec
```

#### Get the default wallet address

```sh
ipc-cli wallet get-default --wallet-type <wallet-type>

# Sample execution
$ ipc-cli wallet set-default --wallet-type evm
"0x406a7a1d002b71ece175cc7e067620ae5b58e9ec"
```

#### Export a wallet key

```sh
ipc-cli wallet export --wallet-type <wallet-type> --address <EVM-ADDRESS> > <OUTPUT_FILE>
```

This command will exporte a wallet private key which is stored in the `ipc-cli` keystore `~/.ipc/evm_keystore.json`.

```sh
# Sample execution
$ ipc-cli wallet export --wallet-type evm --address 0x406a7a1d002b71ece175cc7e067620ae5b58e9ec > /tmp/priv.key
exported new wallet with address 0x406a7a1d002b71ece175cc7e067620ae5b58e9ec in file "/tmp/priv.key"
```

*   Export key encoded in based64 for Fendermint

    ```sh
    ipc-cli wallet export --wallet-type evm --address <EVM-ADDRESS> --fendermint > <OUTPUT_FILE>
    ```
*   Export key in HEX

    ```sh
    ipc-cli wallet export --wallet-type evm --address <EVM-ADDRESS> --hex > <OUTPUT_FILE>
    ```

#### Import a wallet

```
ipc-cli wallet import --wallet-type evm --path <INPUT_FILE_WITH_KEY> --private-key <PRIVATE_KEY>
```

This command will import a wallet from an EVM key file with this format `{“address”:,“private_key”:<PRIVATE_KEY>}`.

```sh
# Sample execution
$ ipc-cli wallet import --wallet-type evm --path=~/tmp/wallet.key
imported wallet with address "0x406a7a1d002b71ece175cc7e067620ae5b58e9ec"
```

Import a wallet from the private key.

```sh
# Sample execution
$ ipc-cli wallet import --wallet-type evm --private-key=0x405f50458008edd6e2eb2efc3bf34846db1d6689b89fe1a9f9ccfe7f6e301d8d
imported wallet with address "0x406a7a1d002b71ece175cc7e067620ae5b58e9ec"
```

### Cross subnet messages

At the moment, the `ipc-cli` only expose commands to perform the basic IPC interoperability primitives for cross-net communication, which is the exchange of FIL (the native token for IPC) between the same address of a subnet. Mainly:

* `fund`, which sends native tokens from one public key address to the address in the child subnet.
* `release` that moves native tokens from one account in a child subnet to its counterpart in the parent.

#### Fund tokens in a child subnet

```sh
ipc-cli cross-msg fund
    --subnet <subnet-id>
    [--from <from-addr>]
    [--to <to-addr>]
    <amount>
```

This command includes the cross-net message into the next top-down proof-of-finality. Once the top-down finality is committed in the child, the message will be executed and you should see the funds in your account of the child subnet. If the `--to` is not set explicitly, the funds are sent to the address of the `--from` in the subnet.

```sh
# Example execution
$ ipc-cli cross-msg fund --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq 100
fund performed in epoch 1030279
```

Alternatively, we can pass an additional parameter to send the funds to a specific address in the child subnet.

```sh
# Example execution
$ ipc-cli cross-msg fund --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq --to=0x406a7a1d002b71ece175cc7e067620ae5b58e9ec 100
fund performed in epoch 1030279
```

#### Pre-fund subnet address in genesis

```
ipc-cli cross-msg pre-fund
    --subnet <subnet-id>
    [--from <from-addr>]
    she<amount>
```

To fund your address in a child subnet genesis before it is bootstrapped, and include some funds on your address in the subnet in genesis, you can use the `pre-fund` command. This command can only be used before the subnet is bootstrapped and started.

```sh
# Example execution
$ ./bin/ipc-cli cross-msg pre-fund --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq 1
```

#### Release funds from a subnet

```sh
ipc-cli cross-msg release
    --subnet <subnet-id>
    [--from <from-addr>]
    [--to <to-addr>]
    <amount>
```

This command will release funds to the parent subnet from its subnet. To release funds from a subnet, your account must hold enough funds inside this subnet.

This command includes the cross-net message into a bottom-up checkpoint after the current epoch. Once the bottom-up checkpoint is committed in the parent, you should see the funds in your account in the parent.

```sh
# Example execution
$ ipc-cli cross-msg release --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq 100
release performed in epoch 1023
```

Alternatively, we can pass an additional parameter to release the funds to a specific address in the parent subnet by setting `--to` address.

```sh
# Example execution
$ ipc-cli cross-msg release --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq --to 0x406a7a1d002b71ece175cc7e067620ae5b58e9ec 100
release performed in epoch 1030
```

#### Release initial subnet funds

```sh
ipc-cli cross-msg pre-release
    --subnet <subnet-id>
    [--from <from-addr>]
    <amount>
```

This command will recover some (or all) of the funds that were sent to a subnet through `pre-fund` to be included as a genesis balance for your address.

```sh
# Example execution
$ ipc-cli cross-msg pre-release --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq 0.1
```

#### Check parent subnet finality

```
ipc-cli cross-msg parent-finality --subnet <SUBNET_ID>
```

The epoch in which the message is performed can give you a sense of the time the message will take to be propagated. You can check the current finality in a subnet and wait for the finality height that includes your message to be committed.

```sh
# Example execution
$ ipc-cli cross-msg parent-finality --subnet /r314159/t410fmdbc3kcv4gody6drgztmgwnzs2ryzwiazjzu5pq
ipc_provider::manager::evm::manager] querying latest parent finality
1070541
```

#### List top-down messages

```sh
ipc-cli cross-msg list-topdown-msgs --subnet=<SUBNET_ID> --epoch=<EPOCH>
```

This command will list the top-down messages sent for a subnet from a parent network for a specific epoch.

```sh
# Example execution
$ ipc-cli cross-msg list-topdown-msgs --subnet /r314159/t410fmdbc3kcv4gody6drgztmgwnzs2ryzwiazjzu5pq --epoch 100450
```

### CheckPoint

#### List checkpoints for a subnet

```sh
ipc-cli checkpoint list-bottomup
    --from-epoch <range-start>
    --to-epoch <range-end>
    --subnet <subnet-id>
```

Subnets are periodically committing checkpoints to their parent every `bottomup-check-period` (parameter defined when creating the subnet). You can use this command to inspect the information of a range of bottom-up checkpoints committed in the parent for a subnet.

```sh
# Example execution
$ ipc-cli checkpoint list-bottomup --from-epoch 0 --to-epoch 100 --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq
epoch 0 - prev_check={"/":"bafy2bzacedkoa623kvi5gfis2yks7xxjl73vg7xwbojz4tpq63dd5jpfz757i"}, cross_msgs=null, child_checks=null
epoch 10 - prev_check={"/":"bafy2bzacecsatvda6lodrorh7y7foxjt3a2dexxx5jiyvtl7gimrrvywb7l5m"}, cross_msgs=null, child_checks=null
epoch 30 - prev_check={"/":"bafy2bzaceauzdx22hna4e4cqf55jqmd64a4fx72sxprzj72qhrwuxhdl7zexu"}, cross_msgs=null, child_checks=null
```

#### Check quorum-reached bottom-up checkpoints

```
ipc-cli checkpoint quorum-reached-events
    --from-epoch <range-start>
    --to-epoch <range-end>
    --subnet <subnet-id>
```

As with bottom-up messages, you can get a sense of the time that your message will take to get to the parent by looking at the epoch in which your bottom-up message was triggered (the output of the command) and listing the latest bottom-up checkpoints to see how far it is from being propagated.

This command will list the bottom-up checkpoints populated, signed, and agreed on their validity by a majority of validators in the child subnet.

```sh
# Sample execution
$ ipc-cli checkpoint quorum-reached-events --from-epoch 600 --to-epoch 680 --subnet /r314159/t410ffumhfeppdjixhkxtgagowxkdu77j7xz5aaa52vy
```

#### Check if bottom-up checkpoints are submitted

<pre><code><strong>ipc-cli checkpoint has-submitted-bottomup-height
</strong>    --subnet &#x3C;SUBNET_ID>
    --submitter &#x3C;RELAYER_ADDR>
</code></pre>

This command can be used to check the state of the checkpoints submitted from a subnet relayer. Once subnet validators have agreed on the bottom-up checkpoint to be submitted in the parent for a specific epoch, relayers need to pick up the checkpoint and submit it in the parent.

```sh
# Sample execution
$ ipc-cli checkpoint has-submitted-bottomup-height --subnet /r314159/t410ffumhfeppdjixhkxtgagowxkdu77j7xz5aaa52vy --submitter  0x406a7a1d002b71ece175cc7e067620ae5b58e9ec
```

#### List submitted bottom-up checkpoints

```
ipc-cli checkpoint list-bottomup-bundle
    --subnet <SUBNET>
    --from-epoch <FROM_EPOCH>
    --to-epoch <TO_EPOCH>
```

This command can be used to check the list of the bundle of bottom-up checkpoints and signatures populated and already signed by a child subnet for their submission to the parent on a window of heights.

```sh
# Sample execution
$ ipc-cli checkpoint list-bottomup-bundle --subnet /r314159/t410ffumhfeppdjixhkxtgagowxkdu77j7xz5aaa52vy --from-epoch 600 --to-epoch 680
```

#### Run a relayer

```
ipc-cli checkpoint relayer --subnet <SUBNET_ID> --submitter <RELAYER_ADDR>
```

IPC relies on the role of a specific type of peer on the network called the **relayers** that are responsible for submitting bottom-up checkpoints that have been finalized in a child subnet to its parent. Without relayers, cross-net messages will only flow from the top levels of the hierarchy to the bottom, but not the other way around.

```sh
# Example execution
$ ipc-cli checkpoint relayer --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq
```

To run the relayer from a different address you can use the `--submitted` flag.

<pre class="language-sh"><code class="lang-sh"># Example execution
<strong>$ ipc-cli checkpoint relayer --subnet /r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq --submitter 0x406a7a1d002b71ece175cc7e067620ae5b58e9ec
</strong></code></pre>

Relayers are rewarded through cross-net message fees for the timely submission of bottom-up checkpoints to the parent. Relayers can claim the checkpointing rewards collected for a subnet.

```sh
# Example execution
$ ipc-cli subnet claim --subnet=/r31415926/t4xwzbdu7z5sam6hc57xxwkctciuaz7oe5omipwbq --reward
```

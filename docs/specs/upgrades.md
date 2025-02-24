# IPC Spec - Upgrades

As IPC evolves, upgrades are necessary to introduce new features, enhance security, improve scalability, and address new challenges. However, it is critical that all upgrades seamlessly transition without causing forks or consensus failures. This ensures that all nodes in the network remain synchronized.

In this document, we describe how IPC achieves full upgradability of all its components and ensures seamless progression of the blockchain network without compromising consensus.

# IPC Actor upgrades

The on-chain logic for IPC is implemented in two contracts, the IPC gateway and the subnet actor, as well as an auxiliary contract used for UX purposes, the registry.

The gateway is a singleton actor that lives in every IPC subnet and implements the common logic for IPC. It is responsible for managing the collateral of subnet, enforcing the firewall requirement, and the cross-net interactions.

The subnet actor is a user-defined actor that implements the specific logic of a subnet. This contract is deployed in the parent from which the child subnet wants to be deployed. There is one subnet-actor for each child subnet in the parent.

The subnet registry behaves as a subnet actor factory that offers users a convenient way of deploying instances of the reference implementation of the subnet actor in a network.

The IPC actors are implemented using the diamond pattern to make it easier to upgrade them. The following sections describes how to upgrade each of these actors.

## Upgrading the Subnet actor

Once you have [deployed your own subnet](https://docs.ipc.space/quickstarts/deploy-a-subnet) you might want to make changes to the subnet actor, for example make custom logic changes, add new functionality, fix bugs, etc.

The code for the subnet actor is located in `contracts/src/subnet` which you can directly edit to make your changes. Once ready, you run the following steps to upgrade your already deployed subnet actor:

1. First, you must know your subnet ID which was returned when you created a child subnet (using `ipc-cli subnet create`). Lets say when you created your subnet that your subnet ID was: `/r314159/f410fkp4r67rks3ok4bvbn2rjojhkbig2rwvp4nmor5q`
2. We must convert the subnet ID to an ETH address. To do this, visit the [Beryx address converter](https://beryx.zondax.ch/address_converter) and input the `f410` address in the Filecoin address input, in this example this would be `f410fkp4r67rks3ok4bvbn2rjojhkbig2rwvp4nmor5q`. Click `Convert to ETH` and it will compute the Ethereum address as `0x53f91f7e2a96dcae06a16ea29724ea0a0da8daaf`.
3. You must set the `RPC_URL` and `PRIVATE_KEY` environmental variables to point to your network provider and the private key of the address you want to use for the deployment, respectively

Now you should be able to upgrade the subnet actor by running the following command (change NETWORK if you have deployed your subnet on another network)

```bash
> cd contracts/
> make upgrade-sa-diamond SUBNET_ACTOR_ADDRESS=0x53f91f7e2a96dcae06a16ea29724ea0a0da8daaf NETWORK=calibrationnet
```

A successful output looks like:

```bash
Successfully generated 84 typings!
Compiled 14 Solidity files successfully (evm target: paris).

Facet Bytecode Not Found:
---------------------------------
Facet Name: SubnetActorGetterFacet
Libraries:
  - SubnetIDHelper: 0x1CaA8B5a18d0401f2089aC82063AaecA08142744
Address: 0xcb4EE21d081e0cBdd3D82D94114bEAa497f11C4a

Diamond Facet Upgrade:
-----------------------------------
Diamond Address: 0x53f91f7e2a96dcae06a16ea29724ea0a0da8daaf
Replacement Facet Name: SubnetActorGetterFacet

Deployment Status:
-------------------------
New replacement facet (SubnetActorGetterFacet) deployed.

Facet Bytecode Not Found:
---------------------------------
Facet Name: SubnetActorCheckpointingFacet
Libraries:

Address: 0xd5C97F52030DFE7595b6cD92868EcE5dC175C224

Diamond Facet Upgrade:
-----------------------------------
Diamond Address: 0x53f91f7e2a96dcae06a16ea29724ea0a0da8daaf
Replacement Facet Name: SubnetActorCheckpointingFacet

Deployment Status:
-------------------------
New replacement facet (SubnetActorCheckpointingFacet) deployed.
```

If you run the `make upgrade-sa-diamond` command again, you should see no output as the script skips upgrading the contract if its bytecode is the same to the one on-chain.

## Upgrading the Gateway actor

TODO

## Upgrading the Subnet Registry actor

TODO

# Fendermint upgrades

Fendermint incorporates a builtin `UpgradeScheduler`, enabling the execution of hardcoded `Upgrade` migrations at predetermined block heights to advance the on-chain state. Fendermint also supports functionality to halt at predetermined `halt_height` in order to switch binary versions.

## Upgrade scheduler

At the start of processing each block, Fendermint checks the `UpgradeScheduler` whether it contains an `Upgrade` for that block height. Only a single `Upgrade` can be scheduled for each block height. If an upgrade is found to be scheduled, it will execute the `migration` function that is associated with that `Upgrade`. If the upgrade migration returns an error it will crash the node.

The migration has access to the `state_tree` and `blockstore` which allows the user to write migrations such as patching state, sending messages, deploying solidity contracts, deploying new WASM actors, etc.

An `Upgrade` is defined as follows:

```rust
/// a function type for migration
pub type MigrationFunc = fn(state: &mut FvmExecState) -> anyhow::Result<()>;

pub struct Upgrade {
    chain_id: ChainID,
    block_height: BlockHeight,
    new_app_version: Option<u64>,
    migration: MigrationFunc,
}
```

Fields:

- `chain_id`: The chain id the upgrade should be applied to.
- `block_height`: The block height where the upgrade should be applied to and the `migration` function executed.
- `new_app_version`: Indicates a new application protocol version if the upgrade introduces backward-incompatible changes.
- `migration`: The migration function that is executed when applying the upgrade. The migration function is passed the `FvmExecState` which gives access to the `state_tree`, `block_store`, ability to send messages and more.

Fendermint supports scheduling multiple upgrades through its `UpgradeScheduler` API which is defined as follows:

```rust
struct UpgradeKey(ChainID, BlockHeight);

pub struct UpgradeScheduler {
  upgrades: BTreeMap<UpgradeKey, Upgrade>,
}

impl UpgradeScheduler {
  // add an upgrade to the upgrade schedule
  fn add(&mut self, upgrade: Upgrade<DB>) -> Result<()> {
    // implementation left out
  }

  // check if the current chain_id has a scheduled upgrade at the given height
  fn get(&self, chain_id: ChainID, height: BlockHeight) -> Option<&Upgrade<DB>> {
    // implementation left out
  }
}
```

As an example, if we want to create an `UpgradeScheduler` with a single `Upgrade`, we could write something like:

```rust
// create a new upgrade scheduler
let mut scheduler = UpgradeScheduler::new();

// add a new upgrade at predetermined block height
let upgrade_func = |state: &mut FvmExecState<DB>| {
  // implement our upgrade logic here
  Ok()
}
let upgrade = Upgrade::new(chain_name, block_height, app_version, upgrade_func);
scheduler.add(upgrade);

// when initializing the FvmMessageInterpreter, specify the upgrade schedule
let interpreter = FvmMessageInterpreter::<DB, _>::new(
  ...
  scheduler,
);
```

We now provide a few examples of using the `UpgradeScheduler` API.

### Example: Patching actor state

In this example, we show how you can write an upgrade migration which patches the `ActorState` of an existing actor stored on chain.

More specifically, in this example we want to patch the state of the `chainmetadata` actor which was deployed at genesis. This actor is used to store blockhashes of the previous blocks on chain. Internally, this actor has the following state:

```rust
// in fendermint/actors/chainmetadata/src/shared.rs
pub struct State {
    // the AMT root cid of blockhashes
    pub blockhashes: Cid,
    // the maximum size of blockhashes before removing the oldest epoch
    pub lookback_len: u64,
}
```

At genesis, this actor was deployed with `lookback_len` of 256. In this migration, we want to change the `lookback_len` to 512 to extend the lookback history.

Inside this migration function, we need to retrieve the actor state associated with the `chainmetadata` actor, update its `lookback_len` to 512, save the new state to the block store and then update the actor state in the state tree.

Our migration function is defined as follows:

```rust
pub fn patch_actor_state_func(state: &mut FvmExecState<NamespaceBlockstore>) -> anyhow::Result<()> {
    let state_tree = state.state_tree_mut();

    // get the ActorState from the state tree
    //
    let mut actor_state = match state_tree.get_actor(CHAINMETADATA_ACTOR_ID)? {
        Some(actor) => actor,
        None => {
            return Err(anyhow!("chainmetadata actor not found"));
        }
    };
    println!(
        "chainmetadata code_cid: {:?}, state_cid: {:?}",
        actor_state.code, actor_state.state
    );

    // retrieve the chainmetadata actor state from the blockstore
    //
    let mut chainmetadata_state: State = match state_tree.store().get_cbor(&actor_state.state)? {
        Some(v) => v,
        None => return Err(anyhow!("chain metadata actor state not found")),
    };
    println!(
        "chainmetadata lookback length: {}",
        chainmetadata_state.lookback_len
    );

    // lets patch the state, here we increase the lookback_len from the default (256) to 512
    //
    chainmetadata_state.lookback_len = 512;

    // store the updated state back to the blockstore and getsave new state cid
    //
    actor_state.new_state_cid = state_tree
        .store()
        .put_cbor(&chainmetadata_state, Code::Blake2b256)
        .map_err(|e| anyhow!("failed to put chain metadata actor state: {}", e))?;
    println!("new chainmetadata state_cid: {:?}", new_state_cid);

    // next we update the actor state in the state tree
    //
    state_tree.set_actor(
        CHAINMETADATA_ACTOR_ID,
        actor_state,
    );

    Ok(())
}
```

Once we have finished writing our `Upgrade` migration, we can add it to the `UpgradeScheduler`:

```rust
let mut scheduler = UpgradeScheduler::new();
let upgrade = Upgrade::new(chain_name, block_height, app_version, patch_actor_state_func);
scheduler.add(upgrade);

// when initializing the FvmMessageInterpreter, specify the upgrade schedule
let interpreter = FvmMessageInterpreter::<DB, _>::new(
  ...
  scheduler,
);
```

### Example: Upgrading Wasm actor

In this example, we show how you can write an upgrade migration which upgrades the code of an existing WASM actor that has been deployed on chain.

More specifically, in this example we want to replace the code of the `chainmetadata` actor which was deployed at genesis. This actor is used to store blockhashes of the previous blocks on chain. For instance, suppose we intend to enhance this actor to store additional information beyond block hashes. The specifics of the new version's functionality are irrelevant; we focus solely on replacing the actor's code.

Inside the migration function, we first must have access to the WASM binary of the new actor. Here, we simply copied the source code of the `chainmetadata` actor to a new location, made relevant changes to the source code of that new actor and compiled it to the `fendermint_actor_chainmetadata_v2.wasm` target.

To replace the existing `chainmetadata` actor that we deployed at genesis with this new v2 version, we need store the new WASM code in the blockstore, then update `code` of the actor state associated with the `chainmetadata` actor with `code_cid` of the new WASM actor.

Our migration function is defined as follows:

```rust
// The WASM binary of the new version of the chainmetadata actor.
static WASM_BIN: &[u8] = include_bytes!("../output/fendermint_actor_chainmetadata_v2.wasm");

pub fn upgrade_wasm_actor_func(
    state: &mut FvmExecState<NamespaceBlockstore>,
) -> anyhow::Result<()> {
    let state_tree = state.state_tree_mut();

    // get the ActorState from the state tree
    //
    let actor_state = match state_tree.get_actor(CHAINMETADATA_ACTOR_ID)? {
        Some(actor) => actor,
        None => {
            return Err(anyhow!("chainmetadata actor not found"));
        }
    };
    println!(
        "chainmetadata code_cid: {:?}, state_cid: {:?}",
        actor_state.code, actor_state.state
    );

    // store the new wasm code in the blockstore and get the new code cid
    //
    let new_code_cid = state_tree.store().put(
        Code::Blake2b256,
        &Block {
            codec: IPLD_RAW,
            data: WASM_BIN,
        },
    )?;
    println!("new chainmetadata code_cid: {:?}", new_code_cid);

    // next we update the actor state in the state tree
    //
    state_tree.set_actor(
        CHAINMETADATA_ACTOR_ID,
        ActorState {
            code: new_code_cid,
            state: actor_state.state,
            sequence: actor_state.sequence,
            balance: actor_state.balance,
            delegated_address: actor_state.delegated_address,
        },
    );

    Ok(())
}
```

Once we have finished writing our `Upgrade` migration, we can add it to the `UpgradeScheduler`:

```rust
let mut scheduler = UpgradeScheduler::new();
let upgrade = Upgrade::new(chain_name, block_height, app_version, upgrade_wasm_actor_func);
scheduler.add(upgrade);

// when initializing the FvmMessageInterpreter, specify the upgrade schedule
let interpreter = FvmMessageInterpreter::<DB, _>::new(
  ...
  scheduler,
);
```

## Halting at predetermined height

Although the `UpgradeScheduler` supports multiple different types of upgrades, it can not support non-state related changes such as upgrading Fendermint dependencies (such as FVM), adding new syscalls, etc.

To support these use cases, Fendermint includes a `halt_height` config which when set (is non zero), will halt and exit Fendermint with a specific exit code (`2`) once it reaches that block height.

This enables node operators to replace the Fendermint version at a predetermined future block height with a new Fendermint version containing the new upgrade. Optionally, the new version can specify a new `Upgrade` at the specified `halt_height` in case a state migration is also required.

This ensures that all nodes run the same Fendermint version for every block height which prevents accidental forking or consensus failures.

<aside>
💡 NOTE: We plan to add support for an orchistrator tool inspired by cosmovisor to make the upgrade process more automated in the future. There is currently a draft [PR](https://github.com/consensus-shipyard/ipc/pull/793) for preparing Fendermint for such tool.

</aside>

### **Instructions**

These instructions detail the steps for utilizing the halting mechanism to synchronize Fendermint application switching with newer versions.

<aside>
💡 NOTE: Node operators should always check the fendermint exit code. If fendermint exits with code 2, which indicates reaching the halting height, disable automatic restarts. Instead, follow steps 4 and 5 in this section.

</aside>

**1. Determine the future halt_height**

Node operators collaborate on which future block height should be used as `halt_height`. Selecting the halting height is totally up to the operators, but let say for example purposes that the halt_height was agreed to be set to `10000`.

**2. Update the halt_height config**

Now, each operator needs to edit the Fendermint config (`.fendermint/config/default.toml`) and make sure it contains the following line

```bash
> cat .fendermint/config/default.toml | grep halt_height
halt_height = 10000
```

Note that Fendermint needs to be restarted before the config changes take effect.

**3. Wait until halt_height**

Once Fendermint reaches the `halt_height`, it will exit and log it reached the halting height.

```bash
...
2024-03-19T13:57:52.904824Z  INFO fendermint/app/src/app.rs:677: event=ProposalProcessed is_accepted=true block_height=9999 block_hash="55E10F55140A995CEE4B540776EF5916CBC5C9D81137FED6788C82F2ACBEA28D" num_txs=0 proposer="BA9ADDB0F298912DB80757EA743C3672AE918FF8"
2024-03-19T13:57:53.006128Z  INFO fendermint/app/src/app.rs:786: event=NewBlock block_height=449
2024-03-19T13:57:53.980298Z  INFO fendermint/app/src/app.rs:677: event=ProposalProcessed is_accepted=true block_height=10000 block_hash="7B5830D4F138D4329909B64AE0A71296271AD65C83158DDED59DEB3C01DDC650" num_txs=0 proposer="BA9ADDB0F298912DB80757EA743C3672AE918FF8"
2024-03-19T13:57:54.036789Z  INFO fendermint/app/src/app.rs:701: Stopping node due to reaching halt height height=10000
```

The node operator can check if Fendermint exited due to reaching halting height by checking if its exit code is `2`.

```bash
> echo $?
2
```

**4. Reset the halt_height**

We must change the Fendermint `halt_height` to 0 (or some future block height if there is another upgrade planned).

```bash
> cat .fendermint/config/default.toml | grep halt_height
halt_height = 0
```

**5. Start the new Fendermint version**

We can now start the new version of Fendermint which contains the upgrade we need.

# UpgradeScheduler: Upgrade WASM Actor

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
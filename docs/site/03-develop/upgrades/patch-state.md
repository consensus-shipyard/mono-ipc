# UpgradeScheduler: Patch Actor State

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

    // store the updated state back to the blockstore and get the new state cid
    //
    let new_state_cid = state_tree
        .store()
        .put_cbor(&chainmetadata_state, Code::Blake2b256)
        .map_err(|e| anyhow!("failed to put chain metadata actor state: {}", e))?;
    println!("new chainmetadata state_cid: {:?}", new_state_cid);

    // next we update the actor state in the state tree
    //
    state_tree.set_actor(
        CHAINMETADATA_ACTOR_ID,
        ActorState {
            code: actor_state.code,
            state: new_state_cid,
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
let upgrade = Upgrade::new(chain_name, block_height, app_version, patch_actor_state_func);
scheduler.add(upgrade);

// when initializing the FvmMessageInterpreter, specify the upgrade schedule
let interpreter = FvmMessageInterpreter::<DB, _>::new(
  ...
  scheduler,
);
```
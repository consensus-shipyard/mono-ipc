# IPC spec - Customizability

IPC is uniquely hyper customizable as a scalability framework. Subnets are highly customizable and can be temporal, allowing subnet operators to spin up customized subnets for various needs, including modular consensus, gas option, configurable chain primitives, customized features via custom actors and pluggable syscalls.

# Custom Actors

For customizing chain functionality, fendermint employs a custom actor bundle file `custom_actors_bundle.car` which contains WASM actors that can be deployed at Genesis.

Currently, Fendermint contains two custom actors:

- `chainmetadata`: Stores the blockhashes of recent blocks so we can look them up through a custom syscall.
- `eam`: A custom version of the eam actor from the built-in actor repo which adds permissioning to who can deploy contracts.

## Adding a new custom actor

In order to add a new custom actor:

1. Implement your new actor inside the`fendermint/actors`. You can look at the `chainmetadata` and `eam` actor for how you organize your actor layout.rs.
2. Update the `fendermint/actors/build.rs` to include the new actor in the `custom_actors_bundle.car` file.
3. Define your actor ID in a new file for your actor in `fendermint/vm/actor_interface/src/`. This ID needs to be unique and not already used by another actor.
4. Deploy your custom actor at genesis in `fendermint/vm/interpreter/src/fvm/genesis.rs`.

# Pluggable Syscalls

In this section, we will focus on how to extend features to your IPC subnet by customizing syscalls that are 'pluggable' as needed.

IPC uses the [Filecoin Virtual Machine (FVM)](https://docs.filecoin.io/smart-contracts/fundamentals/the-fvm) as its execution layer, which is a WASM-based polyglot VM. FVM exposes all system features and information through syscalls, as part of its SDK. The FVM SDK is designed to be pluggable to enable user-defined custom features with syscalls, while implementing default features from FVM kernel. Use cases includes:

- Extending chain-specific syscalls once IPC supports more root chains. Because other chains may have their own special syscalls different as Filecoin (proof validation, etc.).
- Extending features to support better development tools. E.g. adding special debugging syscalls, adding randomness syscalls, and supporting more ECC curve, etc.

## **Instructions**

These instructions describe the steps required to create a new kernel which implements a new syscall along with an example custom actor that shows how you would call that syscall. Full example [here](https://github.com/consensus-shipyard/ipc/pull/630).

<aside>
💡 TIP: For clarity, the instructions may have skipped certain files (like long `Cargo.toml` files) so make sure to refer to the above full example, if you want to follow along and get this compiling on your machine.

</aside>

### **1. Define the custom syscall**

In this example, we will be creating a simple syscall which accesses the filesystem. Inside syscalls, you can run external processes, link to rust libraries, access network, call other syscalls, etc.

We’ll call this new syscall `my_custom_syscall`and its defined as follows:

```rust
pub trait CustomKernel: Kernel {
    fn my_custom_syscall(&self) -> Result<u64>;
}
```

[fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L23](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L23)

Define a struct `CustomKernelImpl` which extends `DefaultKernel` . We use the `ambassador` crate to automatically delegate calls which reduces the boilerplate code we need to write. Here we simply delegate all calls to existing syscall to the `DefaultKernel`.

```rust
#[derive(Delegate)]
#[delegate(IpldBlockOps, where = "C: CallManager")]
#[delegate(ActorOps, where = "C: CallManager")]
#[delegate(CryptoOps, where = "C: CallManager")]
#[delegate(DebugOps, where = "C: CallManager")]
#[delegate(EventOps, where = "C: CallManager")]
#[delegate(MessageOps, where = "C: CallManager")]
#[delegate(NetworkOps, where = "C: CallManager")]
#[delegate(RandomnessOps, where = "C: CallManager")]
#[delegate(SelfOps, where = "C: CallManager")]
#[delegate(SendOps<K>, generics = "K", where = "K: CustomKernel")]
#[delegate(UpgradeOps<K>, generics = "K", where = "K: CustomKernel")]
pub struct CustomKernelImpl<C>(pub DefaultKernel<C>);
```

[fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L27](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L27)

### **2. Implementing all necessary functions for the syscall**

Here is where we implement our `my_custom_syscall` custom syscall:

```rust
impl<C> CustomKernel for CustomKernelImpl<C>
where
    C: CallManager,
    CustomKernelImpl<C>: Kernel,
{
    fn my_custom_syscall(&self) -> Result<u64> {
        // Here we have access to the Kernel structure and can call
        // any of its methods, send messages, etc.

        // We can also run an external program, link to any rust library
        // access the network, etc.

        // In this example, lets access the file system and return
        // the number of paths in /
        let paths = std::fs::read_dir("/").unwrap();
        Ok(paths.count() as u64)
    }
}
```

[fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L42](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L42)

Next we need to implement the `Kernel` trait for the new `CustomKernelImpl`. You can treat this as boilerplate code and you can just copy it as is:

```rust
impl<C> Kernel for CustomKernelImpl<C>
where
    C: CallManager,
{
    type CallManager = C;
    type Limiter = <DefaultKernel<C> as Kernel>::Limiter;

    fn into_inner(self) -> (Self::CallManager, BlockRegistry)
    where
        Self: Sized,
    {
        self.0.into_inner()
    }

    fn new(
        mgr: C,
        blocks: BlockRegistry,
        caller: ActorID,
        actor_id: ActorID,
        method: MethodNum,
        value_received: TokenAmount,
        read_only: bool,
    ) -> Self {
        CustomKernelImpl(DefaultKernel::new(
            mgr,
            blocks,
            caller,
            actor_id,
            method,
            value_received,
            read_only,
        ))
    }

    fn machine(&self) -> &<Self::CallManager as CallManager>::Machine {
        self.0.machine()
    }

    fn limiter_mut(&mut self) -> &mut Self::Limiter {
        self.0.limiter_mut()
    }

    fn gas_available(&self) -> Gas {
        self.0.gas_available()
    }

    fn charge_gas(&self, name: &str, compute: Gas) -> Result<GasTimer> {
        self.0.charge_gas(name, compute)
    }
}
```

[fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L61](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L61)

### **3. Link syscalls to the kernel**

Next we need to implement the `SyscallHandler` trait for the `CustomKernelImpl` and link all the syscalls to that kernel. We need to explicitly list each of the syscall traits (`ActorOps`, `SendOps`, etc) manually here in addition to the `CustomKernel` trait. Then inside the `link_syscalls` method we plug in the actor invocation to the kernel function that should process that syscall. We can link all the existing syscalls using the `link_syscalls` on the `DefaultKernel` and then link our custom syscall.

```rust
impl<K> SyscallHandler<K> for CustomKernelImpl<K::CallManager>
where
    K: CustomKernel
        + ActorOps
        + SendOps
        + UpgradeOps
        + IpldBlockOps
        + CryptoOps
        + DebugOps
        + EventOps
        + MessageOps
        + NetworkOps
        + RandomnessOps
        + SelfOps,
{
	fn link_syscalls(linker: &mut Linker<K>) -> anyhow::Result<()> {
        DefaultKernel::<K::CallManager>::link_syscalls(linker)?;

        linker.link_syscall("my_custom_kernel", "my_custom_syscall", my_custom_syscall)?;

        Ok(())
    }
}
```

[fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L112](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L112)

### **4. Expose the customized syscall**

Once this function is linked to a syscall and exposed publicly, we can use this syscall by calling `my_custom_kernel.my_custom_syscall`

```rust
pub fn my_custom_syscall(
    context: fvm::syscalls::Context<'_, impl CustomKernel>
) -> Result<u64> {
    context.kernel.my_custom_syscall()
}
```

[fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L136](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/examples/mycustomkernel.rs#L136)

### **5. Replace existing IPC kernel with new custom kernel**

Since the customized syscall is implemented in a `CustomKernelImpl` which extends and implements all the behaviors for `DefaultKernel`, we can plug it into IPC instead of `DefaultKernel`.

To use this kernel in fendermint code, replace `DefaultKernel` with `CustomKernelImpl` for the `executor` declaration in `fendermint/vm/interpreter/src/fvm/state/exec.rs`

```rust
use crate::fvm::examples::mycustomkernel::CustomKernelImpl;

executor: DefaultExecutor<CustomKernelImpl<DefaultCallManager<DefaultMachine<DB, FendermintExterns>>>,
```

[fendermint/vm/interpreter/src/fvm/state/exec.rs#L86](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/state/exec.rs#L86)

### **6. Use syscall in your IPC subnet**

Now, we are all set to use the custom syscall in the IPC subnet. The custom syscall can be called in IPC actors to utilize the extended feature. For this tutorial, we can create a simple actor to demonstrate how to import and call the custom syscall and then confirm that its working correctly.

Let’s create a `customsyscall` folder in `ipc/fendermint/actors/` and then create a file called [actor.rs](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/actors/customsyscall/src/actor.rs) in that new folder. Here we want to create a very simple actor, which when invoked (received a message on its Invoke method) will call the new syscall and return its value:

```rust
fvm_sdk::sys::fvm_syscalls! {
    module = "my_custom_kernel";
    pub fn my_custom_syscall() -> Result<u64>;
}

pub struct Actor;
impl Actor {
    fn invoke(rt: &impl Runtime) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&SYSTEM_ACTOR_ADDR))?;

        unsafe {
            let value = my_custom_syscall().unwrap();
            Ok(value)
        }
    }
}

impl ActorCode for Actor {
    type Methods = Method;

    fn name() -> &'static str {
        CUSTOMSYSCALL_ACTOR_NAME
    }

    actor_dispatch! {
        Invoke => invoke,
    }
}
```

[fendermint/actors/customsyscall/src/actor.rs#L14](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/actors/customsyscall/src/actor.rs#L14)

Even though this is Rust code, IPC will compile it as a Wasm target and then run the compiled Wasm code inside FVM as an actor. However, we want to share some of the code between Wasm and IPC, such as the actor name `CUSTOMSYSCALL_ACTOR_NAME` and the `Invoke` method enum. We will define these in a separate file called `[shared.rs](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/actors/customsyscall/src/shared.rs)` as follows:

```rust
use num_derive::FromPrimitive;

pub const CUSTOMSYSCALL_ACTOR_NAME: &str = "customsyscall";

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Invoke = frc42_dispatch::method_hash!("Invoke"),
}
```

[fendermint/actors/customsyscall/src/shared.rs](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/actors/customsyscall/src/shared.rs#L1)

We next need to write a `[lib.rs](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/actors/customsyscall/src/lib.rs)` file which exports the shared code and only compiles `actor.rs` if we are building the Wasm actor:

```
#[cfg(feature = "fil-actor")]
mod actor;
mod shared;

pub use shared::*;
```

[fendermint/actors/customsyscall/src/lib.rs](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/actors/customsyscall/src/lib.rs#L1)

<aside>
💡 NOTE: There are several other files you need to change to compile this actor and package it with the other actors that IPC uses. Please refer to the full example [here](https://github.com/consensus-shipyard/ipc/pull/630) for the following other files you need to change:

- `fendermint/actors/customsyscall/Cargo.toml`: The package for your new actor and all its dependencies
- `fendermint/actors/Cargo.toml`: Add your new actor as a Wasm target
- `fendermint/actors/build.rs`: Include your new actor in the `ACTORS` array so it will get included in the bundle.
- `fendermint/actors/src/manifest.rs`: Add your new actor in the `REQUIRED_ACTORS` array so we can confirm it was correctly bundled on IPC startup
- `fendermint/vm/actor_interface/src/customsyscall.rs`: A macro which assigns an ID to your new actor and declares constants for accessing it by ID and Address
- `fendermint/vm/actor_interface/src/lib.rs`: export the constants to IPC
</aside>

### **7. Load and deploy actor at genesis**

We have so far created a new kernel and syscall, switched IPC to use that kernel and created an actor which calls the new syscall. However, in order to call this actor in IPC, we must load it from the custom_actors_bundle.

To do this open `fendermint/vm/interpreter/src/fvm/genesis.rs` file and in the `init` function add our customsyscall actor right after creating the `chainmetadata` actor:

```rust
// Initialize the customsyscall actor which gives an example of calling a custom syscall
state
    .create_custom_actor(
        fendermint_actor_customsyscall::CUSTOMSYSCALL_ACTOR_NAME,
        customsyscall::CUSTOMSYSCALL_ACTOR_ID,
        &EMPTY_ARR,
        TokenAmount::zero(),
        None,
    )
    .context("failed to create customsyscall actor")?;
```

[fendermint/vm/interpreter/src/fvm/genesis.rs#L251](https://github.com/consensus-shipyard/ipc/blob/98497363a10e08236325e6d5c52755b9fcd52958/fendermint/vm/interpreter/src/fvm/genesis.rs#L251)

Your actor has now been deployed and we should be able to send it messages!

# Genesis Parameters

The contents of the `genesis.json` file are essentially the `Genesis` structure defined in the [genesis](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/vm/genesis) crate. It has the following properties:

- `chain_name` is an arbitrary name for the chain, which will be hashed to become the numeric `chain_id`; in the context of IPC subnets the `chain_name` is expected to be the textual representation of the `SubnetID`
- `timestamp` is the UNIX timestamp of a moment when genesis was created by the first Validator
- `network_version` is used by the FVM to select gas pricing policy
- `base_fee` is measured in *atto* and represents the base price for gas
- `power_scale` is the number of decimals to take into account from the FIL token collateral balance when converting it into voting power expressed as `u64`, which is what CometBFT expects. For example if the scale is 0 then every 1 FIL gives 1 voting power, if it’s 3 then every 0.001 FIL does, and if it’s -1 then every 10 FIL. The power is rounded upwards, so that we don’t end up with 0 power, which would be rejected by CometBFT; for example if the scale is 1, then both 1.1 FIL and 1.9 FIL give 2 power.
- `validators` is a list of stakes of each genesis validator, given by the token collateral and their public key.
- `accounts` is a list of genesis balances, given by tokens and FVM addresses.
- `eam_permission_mode` defines who can deploy smart contracts, which could be anyone, or a set of whitelisted addresses.
- `ipc` is only enabled if we want Fendermint to participate in an IPC hierarchy:
    - `gateway` defines the parameters of the IPC `Gateway` actor:
        - `subnet_id` defines the full `SubnetID` from the root
        - `bottom_up_check_period` is the expected checkpoint frequency for the subnet. It is important that subnet nodes create checkpoints in the ledger at the same heights; doing otherwise would be a consensus failure.
        - `majority_percentage` is the checkpoint quorum size expressed as a number between 0 and 100; it should be at least 67 to be Byzantine fault tolerant.
        - `active_validators_limit` is the number of validators who can participate in the subnet consensus; it is important that both the parent and the child subnet agree on this value, so they select the same top validators the same way.

The `genesis.json` file is usually constructed in one of two ways:

- Using the `fendermint genesis` [CLI commands](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/app/options/src/genesis.rs) to incrementally add validators and accounts as described [here](https://github.com/consensus-shipyard/ipc/blob/specs/docs/fendermint/running.md#genesis).
- Using the `fenderming genesis ipc from-parent` command to fetch the parameters from the subnet actor on the parent subnet and export them to file.

# Gas policy

The gas price in the FVM is by default [determined](https://github.com/filecoin-project/ref-fvm/blob/c39d880d086aa2e771c7190163436e02715d80f3/fvm/src/machine/mod.rs#L156) by the network version when the `NetworkConfig` is created, but could further be customised by assigning to the `price_list` field. Should Fendermint have to do that, it would be in the [constructor](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/fvm/state/exec.rs#L132) of the `FvmExecState`.

The `base_fee` set in genesis is also part of the gas policy. Currently it’s a static value, but it’s part of the upgradable parts of the `FvmExecState` and `FvmStateParams`, with its history maintained by the `App`. The interpreters could have logic to change its value based on the blocks they process.

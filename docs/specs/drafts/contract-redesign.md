# Context

When a subnet is created, it has to register itself to a Gateway, submitting all the funds locked to the gateway. With the funds submitted to the gateway, the gateway will:

1. Route the cross network messages for the subnet
2. Tracks the checkpointing
3. Handles fund deposit and release
4. …

# Problems

There are a few problems associated with the above approach or the current implementations.

- First of all the funds of all subnets are locked in the gateway. If the gateway is misbehaving (it has an owner and can perform upgrades) or other subnets misbehaving, it might impact all subnets registered in the gateway.
- Any subnet can register with any gateway, there is no selection, no restriction and no checks.
- The gateway almost has 100% trust in the subnet registered, this is a big security risk. For example, the gateway fetches the `SupplySource` from the subnet without validation or any other restrictions. That means, the subnet can register with the gateway using SupplySourceA but then change to SupplySourceB without the gateway's approval. At the same time, if the supply source is erc20, there is no checks to see if the implementation is actually malicious. The risk should not have propagated into the gateway and affect all subnets.
- Currently federated power and collateral based permission modes are mixed together. It’s not easy to extend and modify. Also the initialisation of permission mode states in the child subnet is not fully synchronised.

# Phases / scope

- [ ]  Milestone 1: Security-focused refactor of contracts
    - Segregation of each `SubnetActor`
    - Gateway performs routing of messages
- [ ]  Milestone 2: Generalisation of permission modes
- [ ]  Milestone 3: Generalisation of subnet genesis
- [ ]  Milestone 4: Subnet and gateway upgrades

Milestone 4 is not fully covered yet as the upgrade path is not fully clear to me (please help enlighten me).

# Proposals

To separate the subnet actor and gateway so that:

1. Funds are managed by each subnet itself. Each subnet is segregated from another.
2. Gateway handles routing and high level subnet information display.

To clarify the definitions of `Gateway` and `Subnet`. `Subnet`is basically a representation of a network. Each `Subnet` will have a parent, except for the `root` subnet.  `Subnet` should have two parts:

1. A smart contract, which is called `SubnetActor`, that lives in the parent network.
2. A blockchain that is running separately, such as as `fendermint`. `Gateway` is a smart contract that lives in the blockchain, that interfaces with `IPC`.

For `SubnetActor`, as long as it implements the `SubnetActor` interface, it is considered as a potential `Subnet`. `IPC` should provide just templates and utils for `SubnetActor`'s actual implementation. The `Gateway` should be implemented mostly by `IPC`.

`PowerAllocationMode` controls how power is allocated, which can be driven by collateral, explicitly-assigned weights, or both (hybrid).

The `SubnetActor` interface is specified as (TO BE FILLED GRADUALLY):

```solidity
interface SubnetActor {
  // the token used
  function supplySource() external view returns(SupplySource memory);

  // the genesis bytes, child blockchain should parse the bytes accordingly
  function genesis() external view returns(bytes memory);

  function powerAllocationMode() external view 
	  returns(PowerAllocationMode memory);

  function consensus() external view returns(Consensus memory);

  // deposit funds into the subnet
  function deposit(FVMAddress to, uint256 amount) external emits IPCEnvolope;
  
  // route the cross network call from the gateway
  function routeXnetCall(IpcEnvelope msg) external onlyGateway;
}

enum PowerAllocationMode {
  Collateral,
  Federated,
}

enum Consensus {
  // proof of stake like consensus algorithm, could be stake or federated power
  ProofOfPower,
}
```

The `Gateway` interface (logically) contains several parts: `GatewayChildRegistry`, `GatewayTopdownFacet`, `GatewayBottomUpFacet`.  The `GatewayTopdownFacet` handles the requests from the parent. `GatewayBottomUpFacet` handles requests from the child to the parent. `GatewayChildRegistry` handles the registration of subnets in the parent.

```solidity
interface GatewayChildRegistry {
  // a subnet attempts to register itself to the gateway, only approved subnet
  // can register
  function register() onlyApproved external;

  // removes a subnet from the gateway
  function revoke(SubnetId subnet) onlyRole(SubnetAdmin) external;

  function approveRegister(SubnetId subnet) onlyRole(SubnetAdmin) extenral;

  function rejectRegister(SubnetId subnet) onlyRole(SubnetAdmin) external;
}
```

```solidity
interface GatewayBottomUpFacet {
  // withdraw the specified amount to the parent, amount is msg.value
  function withdraw(FVMAddress to) onlyOwner external emits IPCEnvolope;
  
  // for registered subnet to route a message to the parent
  function mail(IPCEnvelope envelope) onlyRegisterred external;
  
  // methods from existing `CheckpointingFacet`
  ...
}
```

The `GatewayTopdownFacet` is the same as existing `TopDownFinalityFacet`

The overall relationship is as follows:

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/75c9b610-402a-494d-9887-8258d6cc60b5/914d7a6e-5033-4c74-a2fd-f549141a9175/Untitled.png)

## **Subnet Lifecycle**

The lifecycle of a subnet happens both in the parent, i.e. through `SubnetActor`, and in the blockchain, i.e. `fendermint`.

- Creation: Subnet creation is just contract deployment, which currently is handled by the subnet registry or by the subnet owner. The creation of the subnet is not a concern here.
- Bootstrap: When the subnet has reached `PowerAllocationMode` thresholds, such as min collateral and min validator count reached, for collateral based mode. The su
    - Then each power allocation mode should have its own implementation.

`IPC` will provide several template implementations of different permission modes.

```
interface CollateralSubnet is SubnetActor {
  function consensus() external view override returns(Consensus memory) {
    return Consensus.ProofOfPower;
  }

  // ======= admin methods =======

  // sample setter for configuration, what can be: minValidators, 
  // minColallateral, ...
  function set(string what, uint256 value) onlyOwner external;

  // ======= open to public =======

  // for join, stake, unstake, leave, kill handling of pre
  function validatorJoin(
	  uint256 collateral,
	  bytes publicKey
	) external emits PowerChange[];

  function validatorStake(
	  uint256 collateral
	) external emits PowerChange[];

  function validatorUnstake(
	  uint256 collateral
	) external emits PowerChange[];

  function validatorLeave() external emits PowerChange[];

  function kill() external;

  // claim the collateral after collateral released
  function claim() external;

  // ===== getters =====
  function isActiveValidator(address addr) external returns(bool);

  function isWaitingValidator(address addr) external returns(bool);

  ...
}
```

```
interface FederatedSubnet is SubnetActor {
  function setPower(
        address[] calldata validators,
        bytes[] calldata publicKeys,
        uint256[] calldata powers
  ) external onlyOwner;

  function kill() external;

  // ===== getters =====
  function getPower(address addr) external returns(uint256);
}
```

For the above implementations, it will call into the existing `LibStaking` (probably rename to `LibPower` or any other better names) that handles the validator tracking.

For collateral based subnet, operations that deal with validator stakes will no longer send funds to the gateway contract. The funds will be managed by the subnet actor instead.

## XNet Messaging (L2 only)

The direct consequence of the change is cross messages execution. The biggest change is all cross message entrypoints are shifted to the `SubnetActor`, `Gateway` no longer plays a critical part in message execution. It exposes only a `mail` method to registered subnets and handles message routing.

For topdown messages, the funds should be locked in the child subnet. For example, the implementation of `fund` will be:

```jsx
contract SubnetActor {

  function fund(..., uint256 amount) {
	  SupplySource memory s = ...;
	  s.lock(amount);
	  
	  IPCEnvolope msg = ...
	  
	  // same as current implementation
    commitTopdownMsg(msg);
  }
}
```

The current `commitTopdownMsg` does not need to change. but only shift to the `SubnetActor`. The corresponding child gateway handling methods that executes the cross messages does not have to change. The execution of topdown messages in the child happens in `GatewayTopdownFacet`, if the message is targeting a grandchild subnet, then `GatewayTopdownFacet` will call the corresponding `SubnetActor` in the child network.

For `sendXnetMsg`, there are still some questions to be clarified, see [link](https://filecoinproject.slack.com/archives/C06KWC57DRA/p1713411693578479). But the design should be mostly similar to `fund`.

For bottom up checkpoint, there is no change to `CheckpointingFacet` as checkpoint creation and signature collection should still happen in the child gateway. When the relayer submits the checkpoint to the parent gateway, there is no need to call into gateway at `commitCheckpoint` , see [link](https://github.com/consensus-shipyard/ipc/blob/1b469edb840680297aa724683f481adfba529561/contracts/src/subnet/SubnetActorCheckpointingFacet.sol#L46). The execution of xnet messages should be shifted to `SubnetActor` , see [method](https://github.com/consensus-shipyard/ipc/blob/1b469edb840680297aa724683f481adfba529561/contracts/src/lib/LibGateway.sol#L357). Only when the target subnet is not the current subnet, then it call into the `Gateway` to route the message into the postbox.

```jsx
contract GatewayRoutingFacet {
  
  function route(IPCEnvolop msg) external onlyRegisterred {
    // design to be discussed, current system does not have this enabled.
    ...
  }
}
```

## Validator Changes Sync Simplification

Currently the validator changes are emitted as operations, that can be replayed in both child and parent. It’s not really necessary for the child to take operations as inputs to power calculation, only the final weight is required. As such, the parent will still maintain the list of top validators, but only emits the final weight to the child.

There are two approaches:

- The power still consists of two parts: totalPower and confirmedPower. The parent records each validator change and applies them to the `totalPower`, the total power is emitted to the child as validator changes, the child picks up the validator changes and applies the batch validator changes to its state, updating the power of each validator in the batch. The child then sends back the final configuration number to the parent in the bottom up checkpoint. Once the parent receives the bottom up checkpoint, it updates the `confirmedPower`  and propagates the changes to top validators.
- The power is just the a uint256. But with each validator change, the updated power is not immediately applied, but pushed to a queue. The queue is sequential respect to the configuration number. (It could be implemented with a circulation buffer, this could also be rated limited if the buffer is full). The change is actually propagated to the child sequentially according to the configuration number. The child applies the change just like the first approach. Once the parent receives the bottom up checkpoint, it pops the configuration changes from the queue and update the validator power accordingly and propagates the top validators.

The first approach requires slightly less change to existing system, but the second approach might be cleaner. Feedbacks needed!

## Generalisation of Subnet Genesis

Currently the subnet genesis is manually constructed, i.e. with each new component or functionality change, one needs to update the code in `fendermint`, `GatewayManager` and `SubnetActor` to capture the changes. At the same time, one needs to make sure the parent genesis information is correctly propagated to the child subnet gateway, otherwise it’s a bug(this happened before).  It’s very coupled. An automated way to track the genesis and propagate to the child subnet would be very helpful.

The idea is as follows, instead of currently creating a variable that tracks the subnet state, i.e. `bootstrapped` field in the `SubnetStorage` struct, one can generalise the genesis formation as concatenation an array of `IGenesisComponent` interfaces.

```solidity
/// @notice A interface that indicated the implementing facet contains a or multiple genesis settings.
interface IGenesisComponent {
    /// @notice Returns the id of the component
    function id() external view returns(bytes4);

    /// @notice Returns the actual bytes of the genesis component
    function genesis() external view returns(bytes memory);

    /// @notice Checks if the component is bootstrapped
    function bootstrapped() external view returns(bool);
}
```

Any `facet` that requires input to the genesis can just implement the `IGenesisComponent` interface. When the subnet is created and the facets are passed into the constructor, by simply checking if the facet support `IGenesisComponent`, one can automatically know which facets need to write data into the genesis.

As an example, a subnet implementation has two facets implementing `IGenesisComponent`:

- `SubnetActorFacet`: Tracks the information of subnet, i.e. bottom up checkpoint period. These metadata needs to propagate to the child subnet
- `FederatedPowerFacet`: Handles the power of the validators through federated mode. The initial validator information need to propagate to the child subnet.

We can create a `SubnetBootstrapFacet` , that holds `[SubnetActorFacet, FederatedPowerFacet]` in its storage. It has the following methods:

```solidity
contract SubnetBootstrapFacet {
    /// @notice Checks if the subnet is bootstrapped
    function bootstrapped() public view returns(bool) {
        /// loops [SubnetActorFacet, FederatedPowerFacet],
        /// to only returns true if the two facets are bootstrapped
    }

    function genesis() external view returns(bytes memory) {
        if (!bootstrapped()) {
            revert SubnetNotBootstrapped();
        }
        
        return Bytes.concat(
           SubnetActorFacet.genesis(),
           FederatedPowerFacet.genesis(),
        );
    }
}
```

The `SubnetActorFacet, FederatedPowerFacet` above can have their own bootstrap conditions. Caller of `SubnetBootstrapFacet.genesis` just need to parse the bytes to accordingly.

The `SubnetBootstrapFacet.genesis()` should be passed to the child `Gateway` , so that the gateway can streamline the genesis syncing process, without manually customization.
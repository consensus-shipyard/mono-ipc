# Subnet Lifecycle
A subnet is a new subsystem that a user can spawn from a parent subnet in a permissionless and on-demand way, depending on scalability requirements. Subnets have separate consensus algorithms and cryptoeconomic rules from their parent subnet. Subnets are firewalled from the parent network.

Subnets begin with a chosen "rootnet". In case of L2 subnets, “rootnets” refer to a layer 1 blockchain, such as Filecoin or Ethereum. Child subnets are spawned from the rootnet and the rootnet becomes the parent subnet.

Each subnet can have any number of child subnets, while each child subnet only has one parent subnet. Subnets can scale infinitely, to layer 2 and beyond. A single hierarchy tree begins at the chosen rootnet.

Subnets within a single hierarchy tree have native communication protocols and are able to transfer assets and state without a custom bridge.

## **Lifecycle**

The lifecycle of a subnet begins when it’s deployed and ends when the subnet is closed.

Before a subnet actor is deployed, the subnet creator must specify the validator power allocation mode, which is called the `PermissionMode`. There are three kinds of permission mode at the moment:

- `Collateral`: This means the power of the validator comes from the collateral staked. New validators can `join` the subnet, also `stake` more collateral, `unstake` collateral and finally `leave` the subnet
- `Federated`: The power of the validator is set by the owner of the subnet
- `Static`: The power of the validator is set when the subnet is created. This mode is used mainly for debugging and testing

At the time of subnet creation, a minimum validator count requirement is set by the subnet creator.  If the subnet’s permission mode is `Collateral`, the creator could also set the minimum collateral requirement for the subnet to be bootstrapped. A standard fee for the transaction on the parent network will be paid for the transaction that establishes the subnet.

Before the minimal requirements are met, the subnet is in a `preBootstrap` state, once those conditions are met, the subnet is in a `postBootstap` state. Any operations performed on the subnet in `preBootstrap` state is recorded in the subnet `genesis`.

Any address can perform:

- Prefund: Provide genesis balance to the subnet, i.e. providing some initial balance to certain reward account in the child subnet.
- Fund: Send fund from the parent to an address in the child subnet. The parent will track the total circulating supply of the subnet. For how the mapping of tokens between the parent and the child works, please refer to the `SupplySource` section.
- Release: Send fund from the child subnet to an address in the parent subnet
- Cross Message Call: Call another contract in another subnet

For validators in `Collateral` permission mode, they can:

- Join: Put some stake into the subnet and becomes a validator
- State: Add more stake into the subnet
- Unstake: Withdraw stake from the subnet
- Leave: Withdraw all stake from the subnet

For detailed explanation on how `Collateral` and `FederatedPower` affect the validator’s voting power, see `SubnetValidation`.

### Deployment

For ipc powered subnet, it’s deployed through `SubnetRegistry`. This contract should be deployed together with the gateway. One can simply call [newSubnetActor](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/subnetregistry/RegisterSubnetFacet.sol#L22) method to deploy a new subnet under the gateway contract. But do note that this method requires permission. The creator of this registry could limit the access on who can deploy new subnets or there are no restrictions at all, see [line](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/subnetregistry/RegisterSubnetFacet.sol#L95).

The states of a deployed subnet consists of:
- Prebootstrap: The minimal validator requirements are yet to be met
- Bootstrapped: The validator requirements are met
- Killed: All validators have left the subnet and no more circulation, `kill` method is called

### Genesis

The genesis state of the subnet is tracked in the [SubnetActorStorage](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/lib/LibSubnetActorStorage.sol#L10) struct of the subnet, any field that starts with `genesis*`.

For `Collateral` permission mode, validators’ stake can be updated and is directly reflected in the validator’s genesis weight until the subnet is bootstrapped. However, for `Federated` permission mode, the owner must make sure the minimum number of validators are met, see [check](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/lib/LibSubnetActor.sol#L82).

The child subnet blockchain, i.e. fendermint, can query the parent subnet actor to [obtain](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/cmd/genesis.rs#L33) the genesis information.

For detailed breakdown on the genesis file, see SubnetConfiguration.

### Bootstrap

The subnet is bootstrapped once the minimal validator requirement and, for `Collateral` permission mode, the minimal collateral requirement, are met.

The `bootstrapped` state of the subnet will become `true` and the `genesis` of the subnet will now be immutable. One need to note that once the subnet is bootstrapped, the `SubnetActor` will [register](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/gateway/GatewayManagerFacet.sol#L33) itself to the corresponding gateway contract. The `SubnetActor` will transfer its fund to the gateway and the gateway will manage the funds for each registered subnet.

This is the current trust model for IPC. The gateway is transparent, trustworthy and manages the routing/funding for each subnet.

### **Closing a Subnet**

The conditions for closing a subnet are as follows:

- A child subnet cannot be killed until its circulating supply is zero, which can be achieved when all users send their funds back to a parent.
- If all validators leave a subnet even when there are still users of the subnet, the users will have to either run their own validator or wait for a validator to return to the subnet.
- If a bug causes the subnet to fail, there is no way to recover funds in the subnet without a valid checkpoint signed by the latest validator committee.

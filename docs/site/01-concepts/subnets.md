# Subnets

## Definition

A subnet is a new subsystem that a user can spawn from a parent subnet in a permissionless and on-demand way, depending on scalability requirements. Subnets have separate consensus algorithms and cryptoeconomic rules from their parent subnet. Subnets are firewalled from the parent network.

## Hierarchy trees

Subnets begin with a chosen "rootnet". Rootnets refer to a layer 1 blockchain, such as Filecoin or Ethereum. Child subnets are spawned from the rootnet and the rootnet becomes the parent subnet.

Each subnet can have any number of child subnets, while each child subnet only has one parent subnet. Subnets can scale infinitely, to layer 2 and beyond. A single hierarchy tree begins at the chosen rootnet.

Subnets within a single hierarchy tree have native communication protocols and are able to transfer assets and state without a custom bridge.

## Lifecycle

The lifecycle of a subnet begins when it’s established and ends when the subnet is closed. 

At the time of subnet creation, a minimum collateral requirement is set by the subnet creator.  A standard fee for the transaction on the parent network will be paid for the transaction that establishes the subnet.

Conditions for closing a subnet include:

* A child subnet cannot be killed until its circulating supply is zero, which can be achieved when all users send their funds back to a parent.
* If all validators leave a subnet even when their are still users of the subnet, the users will have to either run their own validator or wait for a validator to return to the subnet.
* If a bug causes the subnet to fail, there is no way to recover funds in the subnet without a valid checkpoint signed by the latest validator committee.

## Staking

It’s likely that many IPC subnets will be subnets of proof-of-stake chains, or the subnets themselves will be proof of stake chains to other types of chains.  For this reason, IPC has native functionality that is intended to handle staking with respect to subnets.  These native functionalities include staking and releasing collateral associated with subnet validators and slashing collateral associated with a provably misbehaving subnet validator.

## Fees

### Establishing a subnet

There are a number of fees that are paid when a subnet is established:

* At the time of subnet creation, a minimum collateral requirement is set by the subnet creator.
* A standard fee for the transaction on the parent network will be paid for the transaction that establishes the subnet.
* When a participant or validator (other than the subnet creator) joins the subnet, initial funds for their participation in the subnet should be moved from their respective account in the parent by using the `join` command.  This also enables the signing of checkpoint transactions.

### Closing a Subnet

The conditions for closing a subnet are as follows:

* A child subnet cannot be killed untill its circulating supply is zero, which can be achieved when all users send their funds back to a parent.
* If all validators leave a subnet even when their are still users of the subnet, the users will have to either run their own validator or wait for a validator to return to the subnet.
* If a bug causes the subnet to fail, there is no way to recover funds in the subnet without a valid checkpoint signed by the latest validator committee.

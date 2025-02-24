# Circulating supply

## Supply sources

At the moment, there are 2 supply sources possible in IPC: _native_ and _ERC20_.

The native source means that funds in parent's native coin are moved to the subnet.

The ERC20 source means that there is an ERC20 contracts on the parent, which is configured as subnet's supply source. When funds are to be moved to the subnet, they are locked in ERC20 contract and then minted on the parent.

Other than this, the flows for depositing and withdrawing funds are equivalent.

## Depositing funds

The flow of depositing funds from the address on the parent to the address on the subnet consists of the following steps:

1. One of fund functions is called on the _parent's_ Gateway Actor
2. Funds are locked on the parent's subnet.
3. A top-down message is enqueued in the parent to move funds.
4. The subnet's validator fetches top-down messages and executes them, including the message related to depositing funds.
5. As part of the message execution funds are credited to the specified address in the subnet.

## Withdrawing funds

The flow of withdrawing funds from  the address on the subnet to the address on the parent consists of the following steps:

1. A release function is called on the _subnet's_ Gateway Actor.
2. The release function burns the funds which are going to be withdrawn.
3. A top-down message is enqueued in the subnet to move funds.
4. The relayer, which is responsible for submitting checkpoints containing bottom-up messages, propagates the messages to the parent, including the message related to withdrawing funds.
5. As part of the message execution funds are credited to the specified address in the parent.

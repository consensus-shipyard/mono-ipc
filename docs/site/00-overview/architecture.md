---
description: Architectural components in the IPC framework.
---

# Architecture

\
![](https://github.com/consensus-shipyard/docs/blob/main/assets/architecture.png?raw=true)

## Validator nodes

Operators of a subnet run a full validator node for both the parent and the child subnet. Requiring the nodes of child subnets to run the nodes of parents is a security parameter to ensure [checkpointed](../key-concepts/broken-reference/) states of the subnet are appropriately stored, at the right time in the parent.

The following components make up a node:

### Tendermint

[Tendermint Core](https://tendermint.com/) is a byzantine fault tolerant (BFT) consensus engine for blockchains. It acts as a generic [state machine replication](https://en.wikipedia.org/wiki/State\_machine\_replication) (SMR) in a subnet, talking to other Tendermint instances in the subnet and ensuring a consistent ledger is maintained across validator nodes. It talks to the Application in the node, via ABCI++.

### ABCI++

The [ABCI++](https://members.delphidigital.io/learn/abci) interface is implemented in order to handle the IPC ledger logic and transaction handling, using the [Filecoin Virtual Machine](https://docs.filecoin.io/smart-contracts/fundamentals/the-fvm) (or Ethereum-compatible FVM). The ABCI can pass [checkpointed](../key-concepts/broken-reference/) headers to the parent and use the ledger to gather relevant signatures.

An ABCI++ application can contact the [IPLD](https://docs.filecoin.io/basics/project-and-community/related-projects#ipld) [resolver & store](broken-reference) to read and write data so that it is IPLD addressable.

### Filecoin Virtual Machine (FVM)

The [FVM](https://docs.filecoin.io/smart-contracts/fundamentals/the-fvm) enables on-chain programmability and is built as a polyglot VM. It is currently compatible with Filecoin and Ethereum and has plans to support more chains in its [roadmap](https://fvm.filecoin.io/).

FVM is included as a transaction execution later in the subnet, allowing use cases enabled by smart contracts to be built on top of subnets.

You may want to build an dApp on FVM that requires a transaction throughput higher than Filecoin provides. You can deploy a subnet that has a higher transaction settling frequency, and deploy the dApp on the subnet with FVM.

### IPC Actors

Communication between subnets on IPC is done by two [actors](https://docs.filecoin.io/basics/the-blockchain/actors) instantiated in each subnet--the _IPC Subnet Actor (ISA)_ and the _IPC Gateway Actor (IGA)_.

The IGA is an actor that contains all IPC-related information and logic associated with a subnet that needs to be replicated within the subnet. The IGA implements the hierarchical consensus logic and enforces some level of security for account balances, e.g. by ensuring that it is not possible to withdraw more native tokens from the subnet than were used to fund the subnet.

The ISA is the IGA’s parent-side counterpart; that is, it is deployed to a subnet’s parent and contains all data and logic associated with the particular child subnet. For a subnet to be able to interact with its parent, it needs to have a registered subnet actor on the parent network.

Primitives enabling cross-subnet communication include those for transferring funds between accounts in different subnets, saving checkpoints of a child's state in a parent chain, and submitting transactions on one subnet based on inputs from actors on another subnet.

### Relayer

The role of relayers is to pass messages between parent and child subnets. They have to follow both the parent and the child consensus, subscribe to events, re-package the messages in the appropriate formats and resend them. Relayers can be incentivized. Both parent and child subnets can have an entirely different block structure and consensus, and only the relayers understand both, by being purposefully constructed to act between certain combinations.

### Lotus rootnet

[Lotus](https://lotus.filecoin.io/lotus/get-started/what-is-lotus/) currently serves as the rootnet for IPC subnets. It is the reference implementation for Filecoin nodes.

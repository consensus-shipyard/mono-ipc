---
description: >-
  Overview of how IPC works.
---

# How IPC Works

## **Interplanetary Consensus (IPC)**

IPC is designed to scale blockchains through the creation of child chains, each with its own unique consensus mechanism, known as a subnet. This scalable architecture is facilitated by a parent chain, which runs a set of Solidity smart contracts—referred to as [actors](https://docs.filecoin.io/basics/the-blockchain/actors)—that manage the creation, recording, and oversight of subnets along with their validator and staking mechanisms.

## **Initialization and Operation of Subnets**

Each subnet is deployed as a standalone chain utilizing [CometBFT](https://docs.cometbft.com/) for its consensus process. The chain is initially configured within the parent chain, where the base validators set and their stakes are also recorded. Following this initial setup, the chain nodes must be started and connected to the parent chain to begin operations. The subnet deploys its own actors (smart contracts) on the [Filecoin Virtual Machine (FVM)](https://docs.filecoin.io/smart-contracts/fundamentals/the-fvm), which manage subnet-specific operations. Additionally, subnets are fully [EVM (Ethereum Virtual Machine)](https://ethereum.org/en/developers/docs/evm/) compatible, allowing for seamless integration with existing Ethereum-based tools and systems. This design allows subnets to operate semi-independently while still being part of a larger network managed by the parent chain.

## **Communication Between Chains**

Communication between the parent chain and its subnets can occur in both directions—top-down (from parent to subnet) and bottom-up (from subnet to parent). This inter-chain communication is facilitated by a component known as the relayer, which transmits messages between actors on different chains. Typical communications include actor-to-actor messages, updates to the validator set, and periodic checkpoints sent from the subnet to the parent chain.

## **Data Handling with IPLD and IPFS**

Instead of transmitting actual data directly, IPC utilizes [InterPlanetary Linked Data (IPLD)](https://spec.filecoin.io/libraries/ipld/) to create data links. These links are then resolved using a resolver that fetches the actual data stored on the [InterPlanetary File System (IPFS)](https://docs.ipfs.tech/), ensuring efficient and secure data management across the network. Additionally, the CometBFT validators run a quorum to agree on the top-down messages from the parent chain, ensuring they can achieve consensus and end up with the same state.

## **Architecture**

For a detailed exploration of the IPC's underlying structure and design principles, please refer to the [Architecture section](overview/architecture.md). This section provides in-depth coverage of the technical framework and operational guidelines for IPC.

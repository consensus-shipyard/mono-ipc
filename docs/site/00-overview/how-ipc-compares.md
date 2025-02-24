---
description: >-
  IPC is a standout framework that strikes a considerable balance, to achieve
  breakthroughs in scaling.
---

# How IPC compares

## **Benefits of IPC Design**

The IPC framework offers several significant benefits:

- **Scalability**: By enabling the creation of subnets, IPC allows for on-demand horizontal scalability, effectively managing increased network load by distributing transactions across multiple chains.
- **Flexibility**: The ability to tailor stakeholder incentives per subnet caters to diverse application needs, optimizing performance and security. However, switching consensus mechanisms may not be straightforward at the current stage.
- **Interoperability**: Full EVM compatibility ensures that subnets can seamlessly integrate with the broader Ethereum ecosystem, leveraging existing development tools and community resources.
- **Decentralization and Security**: The hierarchical structure of subnets supports a robust security architecture while promoting greater decentralization, as subnets can operate independently but are still connected to the main network.

## **Highly customizable without compromising security**

Most L2 scaling solutions today either inherit the L1's security features but don't have their own consensus algorithms (e.g. rollups), or do the reverse (e.g. sidechains). They are also deployed in isolation and require custom bridges or protocols to transfer assets and state between L2s that share a common L1, which are vulnerable to attacks. In contrast, IPC subnets have their own consensus algorithms, inherit security features from the parent subnet and have native cross-net communication, eliminating the need for bridges.&#x20;

## **Multi-chain interoperability**&#x20;

- IPC uses [Tendermint Core](https://tendermint.com/core/) as a generic blockchain SMR system, without defaulting to the Cosmos SDK (written in Go). This allows IPC to plug in our own application logic regardless of what language it’s written in: it can be Go, Rust, Java, Haskell, Scheme, etc.
- IPC uses the [Filecoin Virtual Machine (FVM)](https://docs.filecoin.io/smart-contracts/fundamentals/the-fvm) as its transaction execution layer. The FVM is a WASM-based polyglot execution environment for IPLD data and is designed to support smart contracts written in any programming language, compiled to WebAssembly. This enables multi-chain support and gives developers the flexibility to build with familiar tools. Today, IPC is fully compatible with Filecoin and Ethereum and can use either as a rootnet, with more multi-chain support in the roadmap.

## **Compute-Storage Interoperability with Filecoin and more**&#x20;

IPC is designed to seamlessly integrate with Filecoin and EVM-compatible chains (with more to come), allowing developers to embed IPC subnets within these ecosystems. In particular, IPC unlocks new compute possibilities with the data-centric L1, [Filecoin](https://docs.filecoin.io/basics/what-is-filecoin), which is the largest decentralized storage network. IPC can leverage its storage primitives, like [IPLD](https://spec.filecoin.io/libraries/ipld/) data integration, to deliver enhanced solutions for data availability and more.

## **Increased performance**

IPC’s modular runtime enables the creation of truly flexible blockchains to increase throughput while managing gas efficiency. Developers can dynamically adjust their throughput by spawning and closing temporary subnets as needed.

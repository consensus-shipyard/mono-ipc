# Introduction

With IPC, dApps can reach planetary scale through recursively scalable subnets, sub-second transactions, robust compute workloads, and highly adaptable WebAssembly runtimes tailored to developer requirements.

## What is IPC?

[Interplanetary Consensus (IPC)](https://www.ipc.space/) is a framework that enables on-demand horizontal scalability of networks, by deploying "subnets" running different consensus algorithms depending on the application's requirements.

Let's break that down.

### What is horizontal scalability and why is it important for dApps?

[Horizontal scalability](https://en.wikipedia.org/wiki/Scalability#Horizontal\_or\_scale\_out) generally refers to the addition of nodes to a system, to increase its performance. For example, adding more nodes to a compute network helps distribute the effort needed to run a single compute task. This reduces cost per task and decreases latency, while improving overall throughput.&#x20;

In web3, horizontal scalability refers to  _scaling_ blockchains, for _desired_ performance. More specifically, _scaling_ the ability of a blockchain to process transactions and achieve consensus, across an increasing number of users, at _desired_ latencies and throughput. IPC is one such scaling solution, alongside other popular layer 2 solutions, like [sidechains](https://ethereum.org/en/developers/docs/scaling/sidechains/) and [rollups](https://ethereum.org/en/developers/docs/scaling/#rollups).&#x20;

For decentralized applications (dApps), there are several key motivations to adopt scaling - performance, decentralization, security. The challenge is that these factors are known to be conflicting goals.&#x20;

### How does IPC achieve horizontal scalability?&#x20;

IPC is a scaling solution intentionally designed to achieve considerable performance, decentralization and security for dApps.

It achieves scaling through the permission-less spawning of new blockchain sub-systems, which are composed of subnets.&#x20;

[Subnets](concepts/subnets/README.md) are organized in a hierarchy, with one parent subnet being able to spawn infinite child subnets. Within a hierarchical subsystem, subnets can seamlessly communicate with each other, reducing the need for cross-chain bridges.

Subnets also have their own specific consensus algorithms, whilst leveraging security features from parent subnets. This allows dApps to use subnets for hosting sets of applications or to [shard](https://en.wikipedia.org/wiki/Shard\_\(database\_architecture\)) a single application, according to its various cost or performance needs. \
\
IPC-powered networks will also be able to dynamically adjust their throughput by spawning and closing temporary subnets as needed.

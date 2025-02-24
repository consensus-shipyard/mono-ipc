# Ethereum JSON-RPC API

This document focuses on interacting with Fendermint through JSON-RPC interfaces.

# Deployments

The docs about running the application contain a [section](https://github.com/consensus-shipyard/ipc/blob/specs/docs/fendermint/running.md#run-eth-api) about the Ethereum API facade, depicting 3 docker containers running across 2 nodes:

- `fendermint` is the ABCI application, which doesn’t have a JSON-RPC interface dedicated to user queries, only expecting an ABCI connection from a co-located `cometbft` container (it does have another RPC server to serve Prometheus metrics, and also a P2P connection through the IPLD Resolver with other `fendermint` instances).
- `cometbft` is running consensus and driving `fendermint` through ABCI, and it has it own [JSON-RPC interface](https://docs.cometbft.com/v0.37/rpc/#/) for user interaction
- `ethapi` is a JSON-RPC and WebSockets server that connects to `cometbft` and presents an [Ethereum JSON-RPC](https://ethereum.org/en/developers/docs/apis/json-rpc/) facade to make Fendermint compatible with Ethereum tooling. It is stateless and doesn’t have to be deployed on the same machine as `cometbft` nor `fendermint`.

It’s worth noting that CometBFT recommends deploying [sentry nodes](https://docs.cometbft.com/v0.37/core/validators#local-configuration) to shield the validator node from direct user connections.

# RPC Client

As described above, we can interact with Fendermint through the CometBFT or the Ethereum API.

For the former we can use the [`tendermint-rs`](https://github.com/informalsystems/tendermint-rs/tree/main/rpc) library, which contains a JSON-RPC client. This client forms the basis of our own `[fendermint_rpc](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/rpc)` crate, which contains the following abstractions:

- `MessageFactory` and `SignedMessageFactory` to produce `ChainMessage` instances to be sent to using the following methods, bound to a particular account address and maintaining a `sequence`:
    - `transaction` constructs generic `Message` instances using `RawBytes` and `MethodNum`
    - `transfer` fills in the defaults for just sending tokens between native accounts
    - `fevm_create` fills in some defaults for deploying EVM bytecode, such as the address of the EAM actor, and takes care of correctly serializing the request
    - `fevm_invoke` fills in the defaults and serializes the calldata for invoking an EVM smart contract
- The `[response](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/rpc/src/response.rs)` module has some helper methods for decoding various responses from CometBFT, dealing with idiosyncrasies of Base64 encoding
- `TxClient` is an interface with methods to actually perform the actions in the `MessageFactory`
- `QueryClient` is an interface with methods corresponding to the `FvmQuery` variants, taking care of performing the query and decoding the results
- `FendermintClient` is a wrapper around the Tendermint JSON-RPC client; it’s not bound to an account, so it can only do queries, but it can be bound by giving it an address and a starting `sequence` number (nonce)
- `BoundFendermintClient` is wrapper around the Tendermint JSON-RPC client and a `SignedMessageFactory`

And example of using this client are [here](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/rpc/examples).

# Ethereum API

The RPC client above is used by the [`fendermint_eth_api`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/eth/api) crate to implement a JSON-RPC server.

That server is typically accessed using [`ethers`](https://github.com/gakonst/ethers-rs/) as a client. Examples of such are [here](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/eth/api/examples), which also form the basis of [end-to-end tests](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/testing/smoke-test).

The best way to get around this crate is to start with the [`api`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/eth/api/src/apis/mod.rs) module where the endpoints are registered. All JSON-RPC methods are organised in modules according to their namespace, with the `PascalCase` names turned into `snake_case` , e.g. `eth_blockNumber` is `eth::block_number`. This way we know that all the interesting methods can be found in the [`eth`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/eth/api/src/apis/eth.rs) module, and take it from there.

All methods are implemented by using `ethers` types, so the serialization and deserialization aspects are taken care of by that library.

The server handles both HTTP and WebSocket traffic. The common request and error handling can be found in the [`handlers`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/eth/api/src/handlers) module.

The entry point for registering routes and starting the servers is the [`listen`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/eth/api/src/lib.rs#L46) function.

## Subscriptions

Subscriptions to the WebSockets from Ethereum clients result in background subscriptions going out to Tendermint.

### Configuration

The default [configuration](https://docs.cometbft.com/v0.37/core/configuration) of CometBFT only allows 5 subscriptions per client, and a maximum 100 clients. These are assuming that users connect directly to CometBFT, rather than a proxies such as the Ethereum API facade. To make it work properly, these ratios should be reversed, e.g. 5 clients with 1000 subscriptions. Examples of where this is set are in the [materializer](https://github.com/consensus-shipyard/ipc/blob/67c1c1658fe73553bf1278b2c03ab66b67f86bc1/fendermint/testing/materializer/src/docker/node.rs#L344-L345) and the [infra scripts](https://github.com/consensus-shipyard/ipc/blob/67c1c1658fe73553bf1278b2c03ab66b67f86bc1/infra/fendermint/scripts/cometbft.toml#L30).

The high number of subscriptions is because of the possible combinatorial explosion that can result when Ethereum filter expressions are [converted](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/eth/api/src/filters.rs#L94) to CometBFT [subscriptions](https://docs.tendermint.com/v0.34/rpc/#/Websocket/subscribe). CometBFT only allows AND conditions, while Ethereum filters contain list of possible values per topic level and address.

### HybridClient

In our experience it is only a matter of time before these background WebSocket connections fail. For this reason our [`HybridClient`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/eth/api/src/client.rs) is a Tendermint client that:

- forwards HTTP requests as-is, with errors propagated back to the user
- handles WebSocket requests in a background queue, where if the background WebSocket client is lost, we re-connect and re-send the command

Note that two kinds of connections can go over WebSockets:

- normal requests which result in a single response sent, connected by `id`
- streaming requests, where a subscription is created which results in an unknown number of responses following (interspersed with other responses)

Out of the two the re-connection done by the `HybridClient` only affects the normal kind requests. When a subscriptions is made through the `HybridClient`, a regular `Subscription` is returned that is going to fail, if the underlying connection fails.  This failure will be propagated to the user (such as the browser), and it’s the responsibility of the user code to re-subscribe to the Ethereum filters it was interested in.

## State

The [`state`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/eth/api/src/state.rs) module contains the `JsonRpcState` structure which is passed to every request handler. It contains an instance of a `FendermintClient`, various caches, the web sockets and filters users subscribed to.

`JsonRpcState` has numerous helper methods to facilitate working with the Tendermint client, and convert the results to the data structures expected by Ethereum. It makes some notable choices to reconcile some of the differences in conventions:

- `latest_height` queries the latest committed height from CometBFT then returns *one less* than its value, ie. `H-1`. It does so because Ethereum expects that the blocks are immediately executed and their results are available (e.g. the post-state hash is part of the block header), while in CometBFT there is a time window between the latest committed block and subsequently its results becoming available through the API. For the latter the block needs to be executed, which happens soon after.
- `block_by_height` gets a block by number, or by some semantic label such as “latest” or “pending”. For the latter, for the same reason as before, a `H-1` is returned, so that if the client or API follows up with a request for results, we don’t get an error.
Another non-standard behaviour it has is that for height `0` it returns a special `BLOCK_ZERO` construct; some Ethereum clients expects block `0` to be the genesis block, however CometBFTs genesis block is at height `1`.
- `query_height` determines which block height to use in a state query; when we query by block number or hash, it applies `H+1`, which is where Fendermint stores the execution results, as per CometBFT conventions.

## Conversions

Converting between Ethereum, Tendermint and FVM types are spread across a number of modules with the module named after the source, and the functions within named after the target. For example `from_eth::to_fvm_message` would take an Ethereum `TransactionRequest` and turn it into an FVM `Message` type.

- [`fendermint_vm_message::conv::from_eth`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/message/src/conv/from_eth.rs) : transactions, addresses, tokens, signatures, to that Ethereum transaction requests can be mapped to what the chain actually expects
- [`fendermint_vm_message::conv::from_fvm`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/message/src/conv/from_fvm.rs) : the transactions as they appear on chain, converted back into what the Ethereum tools can display, with 100% rigour so that we can perform signature checks on the original
- [`fendermint_eth_api::conv::from_eth`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/eth/api/src/conv/from_eth.rs): same as above, with some additional API related conversions
- [`fendermint_eth_api::conv::from_tm`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/eth/api/src/conv/from_tm.rs): maps the responses returned by the CometBFT API to Ethereum types

## Indexing

In the current state of affairs, CometBFT is the only one maintaining a chain database, that is, the CometBFT API is the only source where we can look up blocks, transactions and receipts; Fendermint does not maintain its own indexer, it doesn’t save block results anywhere.

Note that there are ways that the indexing done by CometBFT can be [configured](https://docs.cometbft.com/v0.37/app-dev/indexing-transactions), for example to use different backends.

One issue with this is that some Ethereum clients like the `ethers-rs` library accept a transaction hash a return value of a transaction submission, while others expect that the transaction hash is the Keccak256 hash of the RLP-serialized transaction. This will not be the same as CometBFT hashes the binary contents of a transaction, which is an IPLD encoded `ChainMessage` hashed with SHA-2. The result is that if we use [`/tx`](https://docs.cometbft.com/v0.37/rpc/#/Info/tx) endpoint of the CometBFT API with the hashes submitted by the tools looking for the transaction, we won’t find the transaction.

The workaround we came up with was to use the [`/tx_search`](https://docs.cometbft.com/v0.37/rpc/#/Info/tx_search) endpoint and a special indexed field called `eth.hash`, which is produced by [`SignedMessage::domain_hash`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/message/src/signed.rs#L182) and indexed by [`tmconv::to_domain_hash_event`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/tmconv.rs#L228) during `App::deliver_tx`.

The problem is that this only happens after the transaction has been executed. Some Ethereum tools perform lookup by ID after submitting a transaction, while waiting for its receipt, and if the transaction is not found they conclude it must have been dropped from the mempool.

### Transaction Cache

To avoid this issue, [`JsonRpcState::tx_cache`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/eth/api/src/state.rs#L61-L62) contains an LRU `TransactionCache` where `eth::send_raw_tranaction` [inserts](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/eth/api/src/apis/eth.rs#L624-L627) submitted transactions, and `eth::get_transaction_by_hash` [looks up](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/eth/api/src/apis/eth.rs#L429-L430).

To remove the entries, the `listen` function [starts](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/eth/api/src/lib.rs#L63-L68) the `mpool::start_tx_cache_clearing` function in the background, which subscribes to new blocks and removes the transactions included in them from the cache.

### Sticky Sessions

The way the caching works assumes that either:

- the client is connecting directly to an Ethereum API facade and will not switch to another instance between submitting a transaction and trying to look it up, or
- if the operator uses Load Balancing between multiple Ethereum API facade instances then they also configured S*ticky Sessions* to ensure the same client is routed to the same API instance

<aside>
💡 The requirement for sticky sessions might be considered an inconvenience. For caching, the Ethereum API instances could use some external cache such as Redis or Memcached.

On the other hand we also saw deployments where there was a load balancer between the Ethereum API facades and the full nodes syncing with the network - in such a setup sticky sessions are a must, because each full node can have slightly different state, depending on their syncing progress.

</aside>

## Mempool

CometBFT has its own mempool; it gives the option through configuration to replace with a custom implementation, but currently Fendermint doesn’t have one.

When a transaction is sent to the CometBFT mempool, it first consults Fendermint through ABCI by calling `check_tx` to see if the transaction can be added to the mempool and gossiped to other CometBFT nodes. See more details in [IPS Spec - Executions](https://www.notion.so/IPS-Spec-Executions-ebf13d833d6845ec9c11b59bd514fcda?pvs=21).

The current implementation of `check_tx` is very conservative, only allowing in transactions which can already be executed, that is the account exists, the nonce is the next expected one, the balance is sufficient. All these are checked against the `check_state` , so multiple pending transactions building on each other (e.g. by incrementing the nonce) can be submitted.

There is a problem, however, if the Ethereum tools decide to send transactions that should follow each other by incremental nonces to different nodes, or if they send them to the same node in a randomized order. This can happen in an effort to parallelize transaction submission. The result is that the node rejects most of these transactions; this happened with `hardhat` unless the `--slow` parameter was used to deploy contracts which required dozens of transactions.

To help with this situation, the [`mpool`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/eth/api/src/mpool.rs) module contains a `TransactionBuffer` type which caches transactions that [failed submission](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/eth/api/src/apis/eth.rs#L659-L670) in `eth::send_raw_transaction` by a configurable maximum gap (by [default](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/config/default.toml#L119-L122) 10). The `mpool::tx_cache_clearing_loop` function subscribes to `NewBlock` events by CometBFT and looks for any transaction in the buffer that is *unblocked* by the ones included in the latest block. These unblocked transactions are re-sent by the function to the CometBFT API.

The re-submission can handle chains, so that multiple dependent transactions can go into the next block. The re-submission only happens once; if any error occurs, it won’t be propagated back to the user, just logged.

<aside>
💡 `TransactionBuffer` does not aim to be a fully fledged mempool by any means. The only goal is to help tools that apply parallelisation out of convenience, ending up sending transactions out-of-order unintentionally. This state is not expected to be a dragged out process, as all transactions are sent within milliseconds of each other. The client is not expected to rotate RPC endpoints during this operation.

A more sophisticated [pipelining](https://en.wikipedia.org/wiki/HTTP_pipelining) approach would send the requests and wait not for the receipts, but just the confirmation from CometBFT that the request has been received.

</aside>

### Sticky Sessions

When the operator uses Load Balancing between multiple Ethereum API facade instances, using Sticky Sessions should minimise the chance of unintended out-of-order transaction submission.

However, unlike with the in-memory `TransactionCache`, Sticky Sessions are not required for `TransactionBuffer` to function correctly, as any Ethereum API instance that experiences a rejection due to out-of-order nonces can buffer the transaction and re-send them as their predecessors appear in the blocks. The block time in Tendermint consensus can be as low as 1s, so while having all dependent transactions on a single instance can help to quickly unlock multiple ones in a row, having them scattered across multiple instances will work just as well, albeit slower.

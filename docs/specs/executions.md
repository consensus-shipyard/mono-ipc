# IPC Spec - Executions

This document explains on how Fendermint works at a high level. Fendermint is a CometBFT application at present (and therefore coupled with Ignite consensus). This document focuses on the implementation details of the state transition mechanics.
Note that while Fendermint is the current reference IPC client implementation, it is merely a stepping stone towards a modular, dynamic, embeddable, multi-network IPC client with fully customized consensus in the foreseeable future.

# ABCI

> Note: an upgrade to CometBFT v0.38.x and the ABCI v2 interface is underway.

Fendermint implements the ABCI v1 a.k.a. ABCI++ as [defined in CometBFT v0.37](https://github.com/cometbft/cometbft/tree/v0.37.x/abci).

To serve the ABCI requests in the expected Protobuf format over a TCP connection we use `[tower-abci](https://github.com/penumbra-zone/tower-abci)` with an [asynchronous adapter](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/abci).

Within `fendermint` the [`App`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/app/src/app.rs) implements all the ABCI related methods:

- maintaining the state
- delegating as much as possible to other components, generally living under the [`vm`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/vm) directory
- [converting](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/app/src/tmconv.rs) the results to the expected Tendermint types

The application initialisation happens in the [`run`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/app/src/cmd/run.rs) CLI command handler.

# Interpreters

Message handling is delegated by the `App` to an interpreter, which is known via [generic constraints](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L408-L430) to be able to handle the input and return the kind of output that the `App` knows how to convert to Tendermint types. This way the `App` is testable, although there are currently no unit tests for it.

The [interpreters](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/lib.rs) are designed to be stacked on top of each other as layers, where the input is progressively refined and changes type as they delegate the execution to their inner interpreters.

The interpreter methods are inspired by the [state monad](https://wiki.haskell.org/State_Monad), which is basically a function which some state and an input value, returning a modified state with an output value. The interpreters are supposed to be stateless, so that processing an input only causes changes in the state, not the interpreter - they can of course have their own dependencies and constructors to drive their behaviour.

<aside>
💡 The interpreter methods return a `anyhow::Result` , they don’t have a type specific `Error` type; instead any error is expected to be encoded in their `Output` type (if they have one). It is important to remember that returning an `Err` from these methods is considered a fatal error and will result in the `fendermint` application panicking, after logging the error. The only reason `Err` is returned is to have a uniform way of handling these, but any kind of *domain error* that we want to encode into the actual results sent back through ABCI should be returned as `Ok`, and then handled  in the `App`. `Err` should reserved for situation where we don’t know what to do, e.g. if the database is unavailable.

For this reason, be careful with the use of `?` in interpreters and make sure they are only used for operations that don’t fail due to user input.

</aside>

Every interpreter is a group of related methods from the ABCI spec, which all work on the same state - often this is just one method:

- `GenesisInterpreter` : Parses the app-data passed to `init_chain` and initializes the ledger.
- `ProposalInterpreter`: Parses transactions passed to `prepare_proposal` and `process_proposal` and decides which transactions to keep, any new proposals to add (such as checkpoints), whether to accept them.
- `ExecInterpreter`: Handles the block lifecycle methods `begin_block`, `deliver_tx` and `end_block`.
- `CheckInterpreter` : Parses and validates user transactions sent to the mempool.
- `QueryInterpreter`: Parses and executes user queries.

We have the following layers of interpreters:

- [`bytes`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/bytes.rs): Takes raw bytes as input values as they arrive from CometBFT. Responsible for parsing them from IPLD to the expected message types.
- [`chain`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/chain.rs): Takes [chain messages](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/message/src/chain.rs) as inputs, which are the transactions that can appear on the blockchain, which are generally either signed messages from the user or IPC messages created by the validators or relayers that need special handling by Fendermint. The chain interpreter handles these special messages (for example top-down message batches), and passes on the user messages to the inner interpreter. The idea was that this is a layer where CIDs can be turned into concrete payload.
- [`signed`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/signed.rs): Takes [signed messages](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/message/src/signed.rs), checks user signatures, forwards the signed FVM `[Message](https://github.com/filecoin-project/ref-fvm/blob/d10baf82857df234b37404507df8bbaa581990ac/shared/src/message.rs#L17)` (without the signature) to the inner interpreter
- [`fvm`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/vm/interpreter/src/fvm): This is the layer that generally interacts with the FVM message execution. The module contains separate parts for [`genesis`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/genesis.rs), [`exec`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/exec.rs), [`check`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/check.rs) and [`query`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/query.rs) interpreters.

Besides the transaction message types linked to above, every interpreter also has to deal with the [query messages](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/message/src/query.rs), which are just a payload sent over the CometBFT `abci_query` endpoint, forwarded to the `App` over ABCI.

<aside>
💡 Sometimes it seemed like an interpreter *tree* would be better than a *stack*, but that went against the grouping of methods: we can route a message left or right, but what about the methods that take no input; should we call both? I tried breaking up the interfaces further to make sense of this, but it didn’t seem to be worth the effort.

Another code smell was the number of empty forwarders: only the bytes and the FVM interpreter really has anything to do with queries, but the chain and signed message interpreters nevertheless have to send it along at the moment.

</aside>

## State

All the different state types taken by the interpreters are currently under the `[fvm::state](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/vm/interpreter/src/fvm/state)` module. The states usually wrap something from the `ref-fvm` library, e.g. a `StateTree` or an `Executor` , adding helper methods and extra pieces of parameters as needed.

Whether to put things into the interpreters or the state is a call the developer has to make:

- if it can change, put it in the state
- if it involves decision making, put it in the interpreter
- try to keep the state methods limited to accessing and manipulating the state itself, maintaining any invariants; add reusable general purpose methods

They follow the familiar trope:

- [`genesis`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/genesis.rs): the `FvmGenesisState` is special in that it can create actors directly by IDs in the `StateTree`, then evolve into an `Executor` to run smart contract constructors too
- [`exec`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/exec.rs): the `FvmExecState` is a workhorse used by multiple interpreters; it can execute `Message`s  as well as maintain e.g. the circulating supply
- [`check`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/check.rs): the `FvmCheckState` started out as simple interface to the `StateTree` , limited to doing simple balance and nonce checks, but later on this turned out to be insufficient, and the `FvmExecState` is used most of the time instead.
- [`query`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/query.rs): the `FvmQueryState` lazily loads loads `FvmExecState` depending on which query is being executed and how

Of a particular importance is the [`FvmStateParams`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/fvm/state/exec.rs#L43) construct, which encapsulates all the things that can change from block to block, such as:

- state root CID
- timestamp
- base fee
- circulating supply

This type is the input from the `FvmExecState`, and this is essentially what the [`AppState`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L73-L80) and the [state history](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L146-L156) contains.

## Genesis

The [genesis interpreter](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/genesis.rs) is a one-stop-shop for handling the [genesis message](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/genesis/src/lib.rs) to initialise the ledger from being empty to being ready to run the first block. It loads the builtin actor bundle, then from top to bottom it creates:

- the [`system`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/system) actor
- the [`init`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/init) actor
- the [`cron`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/cron) actor
- the [`eam`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/eam) actor (Ethereum Account Manager)
- the burnt funds actor
- the reward actor
- the [`chainmetadata`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/actors/chainmetadata) actor
- the [replacement EAM](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/actors/eam) actor (to handle permissioned deployments)
- all the [`account`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/account), [`ethaccount`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/ethaccount) and [`multisig`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/multisig) actors which have initial balances
- deploys [`evm`](https://github.com/filecoin-project/builtin-actors/tree/v12.0.0/actors/evm) actors for all the Solidity libraries used by the IPC Gateway and Registry actors

The interpreter returns an `FvmGenesisOutput` which contains the numeric `chain_id` ([derived](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/core/src/chainid.rs) from the `chain_name` in the original `Genesis`), as well as the initial set of validators which are returned to CometBFT.

<aside>
💡 This process should ideally be made available for developers who want to customize their ledger by installing further built-in actors, without having to fork and modify this module directly.

</aside>

## Check

The `App` contains a [`check_state`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L165-L166) which is mutated with every transaction passed to [`check_tx`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L585), until finally it’s [cleared](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L872-L874) in [`commit`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L791). For example every transaction that goes through the check increments the nonce of the account in the `check_state`, so that the client can line up multiple transactions in the right order.

Transactions are sent to `check_tx` as bytes, and are passed to the interpreter:

- The `[BytesMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/bytes.rs#L215)` tries to parse the content as IPLD encoded `ChainMessage`
- The `[ChainMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/chain.rs#L425)` inspects the type of message:
    - `Signed` messages are forwarded to the inner interpreter
    - `Ipc` messages are either:
        - rejected because they are not expected to come from users, instead they would be added to proposed blocks by a validator
        - validated as relayed bottom-up checkpoints, in which case they bear the signature of the relayer as well as the quorum from the subnet validators
- The `[SignedMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/signed.rs#L174)` validates the message signature, unless this is a re-check, in which case this has already been done before
- The `[FvmMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/fvm/check.rs#L25)` checks that:
    - the `from` exists as an actor
    - its `balance` is sufficient to cover the `gas_fee_cap * gas_limit`
    - its `sequence` is matches the one in the `Message`
    - if the `[exec_in_check](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/config/default.toml#L100-L105)` setting is `true` then it also executes the message and checks that the `exit_code` is successful
    - if all checks are passed then the `balance` and the `sequence` are modified in the state

`exec_in_check` is `true` by default so that we can support running queries against the `pending` state, but this creates a bottleneck in that checks will be blocked by queries acquiring an exclusive lock on the state.

The `App` returns the Tendermint response with a `code` based on the `exit_code` as long as the message reached the `FvmMessageInterpreter`. If it failed on an earlier layer, then one of the [`AppError`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L60-L69) values will be returned instead, which are the only Fendermint specific numeric error codes that appear in JSON-RPC responses.

The return value on a successful check result will contain the `[gas_wanted](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/tmconv.rs#L145)` field that CometBFT uses to figure out how many transactions it can put in a block. There is also a potentially useful `priority` field, which we don’t use.

## Proposal

These methods can be used to decide which transactions should appear in blocks. For example this would be the place to prioritise transactions with higher fees, or potentially to replace transactions with CIDs if there are other ways to acquire them.

### Prepare

`[prepare_proposal](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L635)` receives a list of bytes representing transactions in the mempool and has to return the ones that the validator wants to propose in the next block. This is only called on the node which is about the propose a block. The method needs to [respect block size limits](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L653), but otherwise it can reorder or replace the transactions as it sees fit:

- The `[BytesMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/bytes.rs#L73)` has a setting whether to pass the messages to the inner interpreter, or only prepend/append new messages returned from it; if it has to pass them then it parses them as `ChainMessage`, otherwise it just encodes the new messages as bytes on the way out. Finally it enforces are limit on the maximum number of messages that can go in a block.
- The `[ChainMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/chain.rs#L108-L112)` consults background components it was constructed with for IPC related messages that validators can include in a block:
    - any bottom-up checkpoints for which the CID has been successfully resolved and are ready for execution (not used at the moment)
    - any parent subnet finality that has received a quorum in the gossiped finality votes, and are ready for execution

The extra messages can be appended or prepended to the block, depending on how the interpreters are constructed. Currently it’s set to [prepend mode](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/cmd/run.rs#L147), so IPC messages take priority, but originally the idea was that we can append them, and if we hit a message size or message number limit, then these will be re-proposed in the next round.

### Process

`[process_proposal](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L659)` receives the transactions and some other metadata from the block, but only passes on the transactions to the interpreters. The output is an `Accept` or `Reject` :

- The `[BytesMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/bytes.rs#L127-L128)` parses the transactions and has to make a choice whether to reject blocks that contain transactions which cannot be parsed. The messages successfully parsed are forwarded to the inner interpreter. If the block were rejected then no invalid transaction appears in the final output; on the other hand if they are accepted then the proposing validator could be penalized for including them.
- The `[ChainMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/chain.rs#L177-L178)` is concerned about whether to vote for the IPC related messages by checking that:
    - the CID of a checkpoint in `BottomUpExec`  has been resolved (not used at the moment)
    - the block proposed in `TopDownExec` is final on the parent chain

Note that even if a particular node votes `Reject`, others can still `Accept` the block, in which case this node has no choice but to try and procure the data from the parent or child subnet, or its peers.

Rejecting blocks can have a negative impact on liveness, which is why we added vote gossiping so we only ever propose things for which we know a quorum can be achieved. In fact the CometBFT docs say that these methods [should be deterministic](https://docs.cometbft.com/v0.37/spec/abci/abci++_app_requirements), which in our case is not true as they depend on external factors, so we have to be careful.

## Execution

The following methods are part of the block lifecyle after it’s been finalized. The question whether CometBFT has “deferred execution” came up. The answer is “yes”:

1. The block proposer of the current round consults the ABCI app about which transactions to include.
2. The block at height H is proposed to other validators, who consult their ABCI apps whether to accept it.
3. Accepting votes are gossiped by CometBFT.
4. As soon as an individual node sees +2/3 votes, ie. a quorum, it considers the block H final, and executes it. A new app state hash is calculated during commitment.
5. The next block proposer puts the new app state hash into the header block H+1, along with the votes this particular proposer has received to form a quorum - CometBFT tries to include as many votes as possible, for which proposers can be rewarded by the app.
6. Nodes only vote on the block H+1 if their application hash matches.
7. To see the effect of the execution of block H, queries need to be sent with height H+1 (at least this is the CometBFT convention; the Ethereum API facade compensates to make it look like block H effects appear at height H, to conform to the expectations of Ethereum users).

Note that ABCI 2.0 will replace the following methods with a single `[FinalizeBlock](https://docs.cometbft.com/v0.38/spec/abci/abci++_methods#finalizeblock)` which will give the application a chance for example to execute transactions in parallel.

In the sections below we omit the interpreters that simply forward method calls to their inner interpreter.

### Begin

`[begin_block](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L693)` is the method called by CometBFT with the header of a newly finalized block; this is where we can take note of the new `timestamp`, `height` and the `block_hash` . The `App` checks if there is a `halt_height` configured (which facilitates upgrades). If not, it instantiates a new execution state and stores it in the `[exec_state](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L163-L164)` , which is the instance of `FvmExecState` that gets passed to the interpreters and returned in an altered state.

- The `[FvmMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/fvm/exec.rs#L51)` is the only one that handles `begin`:
    - Executes any upgrade scheduled for the current height. It is done in `begin` so that we don’t get into a situation where the transactions in the block have to be processed with old rules, and only at the `end` an upgrade is applied. Effectively the whole block is considered to be *after* the upgrade.
    - Calls the `cron` actor with the current height (CometBFT doesn’t have null rounds, so there is no need for a backfill like in Lotus).
    - Calls the `chainmetadata` actor with the current height and block hash, so that we can make the history available for the EVM.

### Deliver

[`deliver_tx`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L740) is called by CometBFT with individual transactions. On a successful result from the interpreter, the `App` calls `[tmconv::to_deliver_tx](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/tmconv.rs#L67)` to map the results to Tendermint types, which includes mapping all the events, and deciding which one gets indexed by CometBFT. This is the place where the Ethereum specific domain hash is appended to the events, for the sake of retrieving transactions from CometBFT with a non-native hash.

Note that the interpreters have to do the validations that `check_tx` has performed again, because Byzantine validators can propose anything they want - unless we make such checks part of `process_proposal` to disallow this.

- The `[BytesMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/bytes.rs#L177)` parses bytes into `ChainMessage`; if it fails, it could punish the validator for including them in the block.
- The `[ChainMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/chain.rs#L242)` has more or less to do, depending on whether the message is from a user, or part of IPC:
    - `Signed` messages are simply forwarded to the inner interpreter
    - `BottomUpResolve` messages are synhesized into an FVM `Message` and sent to the inner interpreter which should check that the relayed bottom-up checkpoint is legit, and remember to reward the relayer later; then, it schedules the resolution of the CID of the checkpoint contents from the child subnet. This isn’t used at the moment; checkpoints are sent as full-fat transaction payloads instead.
    - `BottomUpExec` is not yet implemented.
    - `TopDownExec` signals that a parent subnet finality has been agreed upon by the subnet validators. The execution of it involves updating the ledger, potentially fetching any data not already in the cache, adding validator changes and executing top-down messages, finally updating the syncer and voting subsystem with the newly finalized block identity. Note that the execution of messages happens using the state, rather than forwarding to the interpreter.
- The `[SignedMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/signed.rs#L132)` verifies the message signature, using the current chain ID, then forwards the `Message` without the signature.
- The `[FvmMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/fvm/exec.rs#L147)` simply executes the `Message`.

### End

`[end_block](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L776)` is called after all the transactions and it has a significant role because this is where the *power updates* are returned to CometBFT, which we use for validator rotation during checkpointing.

- The `[ChainMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/chain.rs#L395)` first calls the inner interpreter, then inspects the results for any power updates, and if there are some, then it updates the voting subsystem to let it know where to accept votes from. This step is here because the `ChainMessageInterpreter` receives in the `State` input a `[ChainEnv](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/chain.rs#L39-L48)` , which is a collection of components working independently in the background, using Software Transactional Memory to communicate data with the interpreter. These could technically be constructor dependencies for the interpreter, however they are instead managed by the `App` and part of the `State`, given that they are mutated during the execution - it perhaps makes it a bit easier to reason about what can change to consider them part of the state. However, these are not passed to the inner interpreter.
- The `[FvmMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/exec.rs#L185)` is responsible for the checkpointing logic:
    - If the current height is an end of an epoch, or we have enough bottom-up messages to hit a threshold, a checkpoint is created in the ledger and the previously stashed validator changes take effect, and the power updates are added to the output. This is done by all full nodes deterministically.
    - Then, if the current node is a validator, it kicks off a background process to broadcast transactions that adds its signature to all pending checkpoints.

### Commit

`[commit](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L791)` is called by CometBFT to signal to the application to persist the changes accumulated during the previous method calls. No interpreter is involved in this, as there are no further state changes:

- The `exec_state` is and flushed to the block store.
- The committed state in the `App` is updated.
- The application state history is extended, saving the state at `height + 1`; if configured with `[state_hist_size](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/config/default.toml#L52)` then older records are purged, making way for them to be garbage collected.
- The snapshot manager is notified about the newly committed height.
- The `check_state` is cleared, so on the next `check_tx` it is refreshed from the database.

The new state hash and the `retain_height` is returned to CometBFT. The latter can be used to allow CometBFT to forget older blocks and free up storage space.

## Queries

`fendermint` doesn’t have an RPC server for custom queries. Instead, these are sent to the `[abci_query](https://docs.cometbft.com/v0.37/rpc/#/ABCI/abci_query)` endpoint of CometBFT and are forwarded to the `[query](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L539)` method in the `App`.

As per the [ABCI specs](https://docs.cometbft.com/v0.37/spec/abci/abci++_methods#query), the `height` parameter is expected to be that of the block where the execution results are made available, that is, where the state hash is published in the header, as a result of the execution of the block at `height - 1`. This is *not* how the Ethereum API works. Since we are using ABCI, I thought it is better to implement queries in a way that conforms to the ABCI documentation, at least at this level.

The `[FvmQueryHeight](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/message/src/query.rs#L14-L28)` type sets out the possibilities of where to run a query:

- `Committed` means to run it on whatever the latest committed state is; this is used if the `height` in the request is `0`.
- `Pending` means to run it on the `check_state` of the app, which includes the effect of transactions which haven’t been included in a block yet; this is used if the `height` is greater or equal than `i64::MAX`. The maximum block height allowed by CometBFT has to fit into `i64`, so `i64::MAX` seemed like a good indicator of pending.
- `Height` indicates a particular value - once again it will *not* contain the effects of the block at the same height, but only the one preceding it.

The `App` figures out a height at which it can execute the query. The chosen height is part of the return value as well, so for example if `0` is requested, the client is informed what the latest height for this node was. If the node has already pruned the data, it runs the query on the latest state instead, so the client should check the value if it’s important for them.

The ABCI query consist of a `path` and some raw `data` bytes, which are both passed to the interpreters:

- The [`BytesMessageInterpreter`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/bytes.rs#L252) is the only one which looks at the `path`, and if it’s `"/store"`, it interprets the `data` as a CID to be looked up in the blockstore (as per the ABCI specs). If not, it is parsed as an [`FvmQuery`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/message/src/query.rs#L57)
- The `[FvmMessageInterpreter](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/fvm/query.rs#L46)` is the one executing all queries:
    - `Ipld` queries are returning raw data from the blockstore
    - `ActorState` looks up actor by its address and returns the state (which includes the balance and its state root CID)
    - `Call` executes a transaction in read-only mode
    - `EstimateGas` also executes a transaction, returning the gas used
    - `StateParams` returns things like the circulating supply of the subnet
    - `BuiltinActors` returns the code CID of the various actors

<aside>
💡 Ideally these should also be extensible for users to add their app-specific queries.

</aside>

# Gas

One of the fields in the CometBFT [genesis](https://docs.cometbft.com/v0.37/core/using-cometbft#fields) is `consensus_params.block.max_gas` . As mentioned under *Check* above, the results of `check_tx` include the `gas_wanted` field, which tells CometBFT how much gas a given transaction required. It uses this information to limit the number of transactions included in `prepare_proposal` to fit within this constraint.

Other than that, Fendermint uses the default gas configuration of the FVM.

# Actor Interface

There is a dedicated `[actor_interface](https://github.com/consensus-shipyard/ipc/tree/main/fendermint/vm/actor_interface)` crate under `vm` to collect all the DTOs and other helpers to interact with specific (builtin or custom) actors. Typical elements in these modules include:

- A macro call to define the well known ID and address of singleton actors, e.g. `SYSTEM_ACTOR_ID` and `SYSTEM_ACTOR_ADDR`.
- A macro call to define the code ID of the actor, e.g. `SYSTEM_ACTOR_CODE_ID`.
- A partial copy of their `State` implementations, which can be used during genesis to construct the state for example of the `Init` or `Multisig` actors, or later to read the state as well (deserialize it as IPLD).
- The `Method` enumeration with all the numeric IDs of different method calls that go in a `Message`.
- DTOs for inputs and outputs of methods.
- Helper constructs like the `EthAddress`
- The different facets of the diamond constructs of the IPC actors

Some actor like the `evm` and the `chainmetadata`have a `shared` library, so that we don’t have to copy DTOs but share them and compile them both to Wasm and native code. Even then, their ID is defined here, in this crate, so there is a common place to look it up.

The IPC Solidity actors enjoy some special treatment in that we have Rust `[binding](https://github.com/consensus-shipyard/ipc/tree/main/contracts/binding)` generated for them during the build, because of the frequent interaction from `fendermint` itself. To facilitate working with them through the `FvmExecState`, the `[ipc](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/ipc.rs)` module defines the `GatewayCaller` , which contains multiple instances of the `ContractCaller` defined in the [`fevm`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/fevm.rs) module to provide a unified interface over the facets defined in Solidity.

# IPLD

IPLD stands of InterPlanetary Linked Data, and it’s a JSON-like data structure [defined](https://github.com/ipld/libipld/blob/v0.16.0/core/src/ipld.rs) in [`libipld`](https://github.com/ipld/libipld). Notably it has a `Link` type with a `Cid` in it that can point at other IPLD structures, which makes it traversable.

IPLD data has various encodings, one of which is DAG-CBOR, which is used most often to encode data as bytes, and then put them into the `Blockstore` the FVM supports.

The `Blockstore` in our case is [implemented](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/rocksdb/src/blockstore.rs) on top of RocksDB. In particular we store the actor state in a [`NamespacedBlockstore`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/rocksdb/src/blockstore.rs#L45-L47), which means the data goes into a separate column family. The different column family names are defined in [`Namespaces`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/cmd/run.rs#L49-L57). The fact that different data go into different column families within the same RocksDB instance should be taken into account with any garbage collection implementation.

The application state is also encoded as IPLD, however these aren’t stored by CID in the `Blockstore`. Instead we use a different set of `KVStore` abstractions defined in `storage` to manage read and write transactions, also [implemented](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/rocksdb/src/kvstore.rs) for RocksDB. The generic encoding is defined by the [application](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/app/src/store.rs).

# Syscalls

The syscalls which the FVM needs the host to implement are defined in the [`externs`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/externs.rs) module.

Out of these the only one we implement is the `get_tipset_cid`, for which we return the block hash of the past blocks, looking them up in a data structure maintained by the `chainmetadata` actor in *Begin*, as described above. The reason this is added to the ledger, as opposed to looking it up in a “chain database” like Lotus does, is because Fendermint doesn’t maintain its own chain database, only CometBFT does. In theory we could look the data up using the CometBFT API, however in practice there are limitations:

- In our experience CometBFT doesn’t respond to API calls while it’s replaying the last finalized block to the application, so any call to CometBFT should not happen during block processing, as it can lead to a panic, or deadlock. For this reason these lookups happen asynchronously, on background tasks, with retries, and the block processing never relies on their results.
- The chain history would not be part of snapshots, which could lead to nodes which bootstrap themselves form a snapshot received from another being unable to deterministically run the same transactions, because they could not do historical lookups.

The [default history size](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/actors/chainmetadata/src/shared.rs#L79) is 256, as per the EVM specifications regarding the `BLOCKHASH` function which this functionality enables, and this is how we currently [instantiate](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/fvm/genesis.rs#L238) this actor in genesis.

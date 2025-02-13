# Subnet genesis v2

### Scope 

This document introduces a revised set of processes to conduct subnet genesis, codenamed "Subnet Genesis v2".  

### Context

_Genesis_ is a term widely used in the DLT space, and it can take various nuances depending on the technology/chain. 
In Bitcoin, genesis refers to the first block in the Bitcoin chain. In Tendermint, genesis refers to the specification
document used to bootstrap a chain.

In IPC, we use the term loosely to refer to the process of bootstrapping the subnet ledger following its activation on
the parent network. Because IPC is envisioned as a consensus-agnostic framework, our genesis-related assets are
merely intermediate representations that are translated to the specific artifacts required by the chosen consensus
engine. Think LLVM-IR for subnet bootstrapping.

### Overview

Once a subnet has been activated in the parent (i.e. all its collateral or minimum validator conditions are met), the
general steps for genesis are as follows:

1. Initialize a genesis specification from the parent.
2. Customize the specification.
3. Add binary assets (actor bytecode, system contracts) to the specification.
4. Seal the specification (generates the initial state tree).
5. Materialize the consensus-specific genesis assets.

### Data model 

Below is the schema for the IPC genesis specification, in a relaxed YAML notation. 

```yaml
## Schema version. Initial value: 1.
version: uint64

## Metadata about this genesis specification. Dropped for CID summarization.
meta:
  ## Status of this genesis specification. Values:
  ## - 'incomplete': genesis being parameterized.
  ## - 'sealed': genesis was fully parameterized and state tree has been generated.
  status: incomplete | sealed
  ## Timestamp when this specification was created, as an ISO8601 string.
  created: string
  ## Timestamp when this specification was last updated, as an ISO8601 string.
  last_updated: string

## Basic information about the subnet.
subnet:
  ## The FQN (fully qualified name) of the parent subnet in the IPC hierarchy, using standardised slash notation, e.g. /r314/f410.../f410...
  ## Or null if this subnet is standalone (unrooted).
  parent: string | null
  ## The FQN (fully qualified name) of this subnet in the IPC hierarchy, using standardised slash notation, e.g. /r314/f410.../f410...
  ## If this subnet is standalone (unrooted), this is a custom string.
  path: string
  ## A user-defined alias for this subnet.
  alias: string
  ## Genesis timestamp, as an ISO8601 string.
  genesis_timestamp: string
  ## Genesis network version.
  network_version: uint64 
  
## Consensus configuration for this subnet.
consensus:
  ## The consensus engine adopted by this subnet. The only value supported is 'tendermint', denoting that CometBFT will
  ## be used.
  engine: 'tendermint'
  ## Validation configuration.
  validation:
    ## The percentage of power/weight above which 
    majority_percentage: uint64
    ## Validator membership configuration.
    membership:
      ## Defines how membership is determined.
      ## - 'static': members and weights are defined upfront at subnet creation time, and cannot be changed after genesis.
      ## - 'collateral': members and weights are projected from collateral deposited at the parent (proof of stake style).
      ## - 'federated': members and weights are determined arbitrarily by an authority at the parent (proof of authority style).
      mode: static | collateral | federated
      ## The scaling factor that applies to the raw power communicated to the parent (raw * 1e-{power_scale}). 
      ## To be deprecated during contracts refactor project.
      power_scale: uint8
      ## The maximum number of active validators at a time.
      ## To be deprecated during contracts refactor project.
      max_active: uint64
    ## Initial validator set as an array of pubkeys and weights.
    initial_members:
      - pubkey: bytes
        weight: uint64
  
## Balances to be credited to the specified addresses at genesis, directly on the state tree,
## the precision unit of the circulating supply source.
balances:
  [address]: uint256
 
## Execution layer configuration. 
execution:
  ## Gas market configuration
  gas_market:
    ## Oscillation function. Values: eip1559 or static.
    kind: eip1559 | static
    ## Initial base fee.
    initial_base_fee: uint256 
  ## Configuration of FVM runtimes.
  runtimes:
    ## Filecoin EVM configuration.
    fevm:
      ## Contract deployment permissions.
      contract_deployment_permissions:
        ## Contract deployment permissioning mode. Values: 'unrestricted', or 'allowlist'.
        mode: unrestricted | allowlist
        ## If the mode is 'allowlist', specify a list of allowed addresses, or blank if none is allowed.
        allowlist:
          - addresses
         
## CIDs of actor manifests for built-in actors and for custom actors.
actor_bytecode:
  builtin: Cid
  custom: Cid | null

## The IPC system contracts, as a CID to the root of a UnixFS DAG stored in the inline blockstore.
system_contracts: Cid

## The initial state tree after
initial_state_tree: Cid

## Blockstore serialized as a compressed inline CAR.
blockstore: inline_bytes
```

### Initializing a genesis specification

A genesis specification can be initialized in one of two ways, depending on the kind of subnet.

For a child subnet, we use the `ipc genesis spec init-child` tool. This tool takes the parent subnet RPC, the gateway
address, and the subnet ID. It initializes a prefilled genesis, populating as much detail as possible from the on-chain
subnet definition.

For a standalone (unrooted) subnet, we use the `ipc genesis spec init-standalone --template=<name>` command. This tool
simply writes an empty genesis based on the specified template, for the user to adjust manually.

### Customizing a genesis specification

For the most part, developers customize the genesis specification by manually editing the file, except for the
following elements:

- actor_bytecode/builtin
- actor_bytecode/custom
- system_contracts
- initial_state_tree
- blockstore

Actor bytecode and system contracts are added using the `ipc genesis spec update` tool one or several of the following
flags:

- `--actor-bytecode-builtin=<path to bundle>`
- `--actor-bytecode-custom=<path to bundle>`
- `--system-contracts=<path to contract>`

These commands add the relevant blocks to the inline blockstore, and upsert the spec with the corresponding root CID(s)
in the appropriate field(s).

### Validating a genesis specification

`ipc genesis spec validate` validates that the genesis specification is syntatically and semantically correct.

### Generating the state tree

`ipc genesis spec seal` seals the specification by generating the initial state tree, copying it to the blockstore, and
setting the root CID in the `initial_state_tree` field. It also transitions the `meta.status` field to `sealed`.

### Materializing consensus-specific genesis assets

`ipc genesis materialize --spec-file=<path>` reads the specification file, ensures that it's sealed, and generates
the genesis assets required for the selected consensus engine type.
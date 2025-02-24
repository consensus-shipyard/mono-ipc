# Gas markets

In IPC, gas markets are algorithmic entities that determine key aspects of transaction pricing and dynamics within
subnets. Starting from the axon-r06 release, subnets can freely customize their gas market, as long as they adhere to
the standard actor interface defined below. This interface will evolve over time to accommodate new capabilities and
features.

## Capabilities

Starting from the axon-r06 release, gas markets can update the base fee and the block gas limit.

The **base fee** is applied during state transitions, and is used to calculate the network fee to execute a message
per gas unit. This portion of the transaction fee is burnt by sending it to the f099 burn address within the subnet.
Note that, today, these amounts remain at the f099 address in the subnet, and are counted towards the circulating
supply of the subnet (as seen by the parent). In the future, we plan to physically burn these amounts in the subnet
and notify the parent so that (a) the supply source is updated, and (b) these amounts are removed from the circulating
supply accounting.

The **block gas limit** is the maximum cumulative gas limit across all transactions in a block. This limit is enforced
by the consensus layer, so updating it via the gas market actor will cause us to notify the consensus layer
accordingly.

## Gas market actor

Gas markets are deployed as on-chain Wasm actors that implement a standard interface. The gas market is a singleton
actor deployed at ID address f098 on the subnet.

In axon-r06, the standard gas market interface is quite simple. It revolves around two interactions:

1. Acquiring the current reading of the gas market.
2. Updating the current utilization of the gas market (and returning the updated reading).

The following Rust code defines the standard interface for gas markets.

```rust
/// A reading of the current gas market state for use by consensus.
pub struct Reading {
    /// The current gas limit for the block.
    pub block_gas_limit: Gas,
    /// The current base fee for the block.
    pub base_fee: TokenAmount,
}

/// The current utilization for the client to report to the gas market.
pub struct Utilization {
    /// The gas used by the current block, at the end of the block. To be invoked as an implicit
    /// message, so that gas metering for this message is disabled.
    pub block_gas_used: Gas,
}

/// The trait to be implemented by a gas market actor, provided here for convenience,
/// using the standard Runtime libraries. Ready to be implemented as-is by an actor.
pub trait GasMarket {
    /// Returns the current gas market reading.
    #[frc42(method_name = "CurrentReading")]
    fn current_reading(rt: &impl Runtime) -> Result<Reading, ActorError>;

    /// Updates the current utilization in the gas market, returning the reading after the update.
    #[frc42(method_name = "UpdateUtilization")]
    fn update_utilization(
        rt: &impl Runtime,
        utilization: Utilization,
    ) -> Result<Reading, ActorError>;
}
```

## Default gas market: EIP-1559

IPC ships with EIP-1559 as the default gas market, with constants identical to Filecoin mainnet's.

Here are the constants, their roles in this algorithm, and their default values:

| Constant                          | Description                                                                                        | Default value |
|-----------------------------------|----------------------------------------------------------------------------------------------------|---------------|
| `block_gas_limit`                 | The block gas limit, as defined above.                                                             | 10M (^)       |
| `minimal_base_fee`                | The minimal base fee floor when gas utilization is low.                                            | 100 atto      |
| `elasticity_multiplier`           | Elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559).           | 2             |
| `base_fee_max_change_denominator` | Base fee max change denominator as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559). | 8             |

(^) Matching the Filecoin block gas limit. As defined
in [FIP-0032](https://github.com/filecoin-project/FIPs/blob/master/FIPS/fip-0032.md#gas-sizing-baseline), Filecoin sizes
gas by adopting a baseline of 10 gas/nanosecond of wall-clock time, or 1 milligas/10 picosecond. This baseline follows
from the need to sustain a 30s epoch by validating and applying unique messages in a tipset, with a block count
following a Poisson distribution (win rate), with a 10B block gas limit, assuming 6s of block propagation time. Since
IPC inherits the FVM, which encodes these assumptions, we keep the block gas limit equal. That said, it is worth to note
some differences: (a) Tendermint consensus produces a single block per height, while Filecoin produces multiple blocks
per epoch (usually oscillating between 4-10); (b) IPC subnets usually tick much faster than Filecoin (30s per epoch);
(c) IPC is impacted by lower execution overhead than Lotus (Filecoin's reference client) as it doesn't need to traverse
an FFI boundary to access the state store.

### Setting the constants

Developers can set the EIP-1559 constants through two mechanisms:

1. At genesis, currently
   by [patching the initial state tree construction](https://github.com/consensus-shipyard/ipc/blob/702d49b619623915772fe935a86e59a89744c3b1/fendermint/vm/interpreter/src/genesis.rs#L443).
   In the near future we intend to provide an API to set these constants, so that forking and patching is no longer
   necessary.
2. Updating the constants at runtime through an on-chain message. By default, this message can only be applied during an
   upgrade through the UpgradeScheduler, since the original implementation enforces the sender to be the system actor (
   f00). However, developers can set a different authorized addressed, such as an EOA, a multisig, or a smart
   contracts (e.g. DAO, timelock, etc.) This requires patching the actor code.

## Custom gas markets

Developers can deploy a custom gas market by implementing the `GasMarket` trait in a Wasm actor, and replacing the
default gas market at actor ID `f098`. This can be done at genesis, or in a network upgrade through the
UpgradeScheduler.

## Caveats

1. Updating the block gas limit in a block containing other transactions leads to undefined behaviour. Always perform
   this update in isolation, and at the end of the block. The new block gas limit is applied from the next block
   onwards.  

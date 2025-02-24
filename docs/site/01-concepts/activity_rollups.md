# Activity rollups

Activity rollups serve as a mechanism for synthesizing and exporting critical information about activity within a subnet
to parent or ancestor networks, potentially reaching the root network. This enables the triggering of external actions
such as reward distribution via token inflation, validator set rebalancing based on performance metrics, and other
systemic updates. These rollups are continuously aggregated during subnet execution and are periodically consolidated
and released through bottom-up checkpoints, ensuring efficient and structured communication across network layers.

## How activity rollups work

### General schema

**Summaries.** Activity rollups contain subject-scoped summaries. There are two classes of summaries:

- **Protocol-defined summaries:** part of the IPC protocol itself, receiving first-class support from the framework.
- **Application-defined summaries:** created by developers to carry application-specific information up the hierarchy.

**Representations.** Activity rollups can be represented in full and compressed forms:

- **Full activity rollups:** the original payload of the rollup as generated in the subnet, containing the entirety of
  the data in full form. They are published only locally within the subnet (as events), and are not transmitted
  outwards.
- **Compressed activity rollups:** the compressed version of the rollup, suitable for travelling in checkpoints.

This distinction is crucial to prevent checkpoints from being saturated with verbose activity data. By relying on
compressed representations, we significantly reduce the byte footprint, optimize gas consumption, and conserve space for
cross-network (xnet) messages, which are also transported within checkpoints—all without compromising security. The
compressed format generally includes succinct representations of the data along with cryptographic commitments to the
full data, enabling future claims to be verified through inclusion proofs.

```solidity
// Carries a set of reports summarising various aspects of the activity that took place in the subnet between the
// previous checkpoint and the checkpoint this summary is committed into. If this is the first checkpoint, the summary
// contains information about the subnet's activity since genesis.
// In the future we'll be having more kinds of activity reports here.
    struct FullActivityRollup {
        /// A report of consensus-level activity that took place in the subnet between the previous checkpoint
        /// and the checkpoint this summary is committed into.
        /// @dev If there is a configuration change applied at this checkpoint, this carries information
        /// about the _old_ validator set.
        Consensus.FullSummary consensus;
    }

// Compressed representation of the activity summary that can be embedded in checkpoints to propagate up the hierarchy.
    struct CompressedActivityRollup {
        Consensus.CompressedSummary consensus;
    }
```

### Tracking activity inside the subnet

**activity-tracker actor.** The activity-tracker actor is a built-in actor that exists in every IPC subnet at ID address
97 (f097). It acts as a live accumulator of activity data reported by various components.

Before emitting a checkpoint, the activity tracker finalizes the accumulated activity by constructing a complete
activity rollup, encompassing all supported summaries. Once the rollup is constructed, the accumulator's state is
cleared, allowing the tracker to resume collecting new data seamlessly. This process is currently implemented within the
`ABCI::end_block` method, though the design remains flexible and independent of the specific consensus engine, ensuring
adaptability across various implementations.

When a quorum is achieved within the child subnet, the checkpoint is finalized, and a relayer submits it—along with the
compressed activity rollup—to the parent network. This process is secured through the subnet's consensus mechanism and
validated by the parent network using the checkpoint's multi-signature scheme, ensuring integrity and trustworthiness in
the cross-network communication.

**Node responsibilities.** The node is currently responsible for:

- Compressing the full activity rollup into its compressed form, and embedding the result in the checkpoint.
- Emitting a local event in the subnet to publish the full activity rollup (although technically this happens in the
  gateway Solidity contract for convenience).

The emitted event confirms to the Solidity ABI:

```solidity
// Event to be emitted within the subnet when a new activity summary has been recorded.
    event ActivityRollupRecorded(uint64 checkpointHeight, FullActivityRollup rollup);
```

**Future direction.** In the future, both these tasks will be moved to actor space, in line with our policy of
offloading as much logic as possible to actor space. The activity-tracker will commit its compressed rollup into the
local gateway directly, and will emit the event containing the full activity rollup.

**Extensibility plans.** We plan to add first-class support for application-defined summaries in the future, and
eventually allow the user to define their own summaries in a pluggable way. The activity-tracker actor will likely turn
into an orchestrator and aggregator of other actors maintaining their own summaries.

**Actor interface.** The actor's public interface is as follows:

```rust

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    RecordBlockCommitted = frc42_dispatch::method_hash!("RecordBlockCommitted"),
    CommitActivity = frc42_dispatch::method_hash!("CommitActivity"),
    PendingActivity = frc42_dispatch::method_hash!("PendingActivity"),
}

trait ActivityTracker {
    /// Hook for the consensus layer to report that the validator committed a new block.
    fn record_block_committed(rt: &impl Runtime, validator: Address) -> Result<(), ActorError>;

    /// Commits the pending activity into an activity rollup.
    /// Currently, this constructs an activity rollup from the internal state, and then resets the internal state.
    /// In the future, this might actually write the activity rollup to the gateway directly, instead of relying on the client to move it around.
    /// Returns the activity rollup as a Solidity ABI-encoded type, in raw byte form.
    fn commit_activity(rt: &impl Runtime) -> Result<FullActivityRollup, ActorError>;

    /// Queries the activity that has been accumulated since the last commit, and is pending a flush.
    fn pending_activity(rt: &impl Runtime) -> Result<FullActivityRollup, ActorError>;
}
```

### Processing activity rollups at the parent

As checkpoints containing activity rollups are submitted to the parent network, the Subnet Actor dispatches the
summaries to their designated handlers. Currently, a protocol-defined handler processes consensus summaries (detailed
below). In future iterations, users will have the flexibility to define custom handlers tailored to their specific
summaries.

Alternatively, the parent network may choose to relay incoming activity rollups up the hierarchy, allowing ancestor
networks to process them. This relaying could occur recursively through multiple levels, potentially reaching the root
network. If the root network lacks configured handlers, any unprocessed activity rollups are simply discarded. This
relaying capability is scheduled for implementation in Phase 2 of this feature.

## Consensus summaries

Only one protocol-defined summary exists today: the **consensus summary**. This summary collects datapoints from the
consensus layer that are relevant to export/surface to ancestors.

### Data model

This summary currently exports the number of blocks committed per validator, and the total number of validators that
were active. This is sufficient information for subnet creators to reward validators at the rootnet, e.g. through token
inflation.

**Compressed form.** The compressed form of the consensus summary contains aggregated stats, and a commitment to the
full validator breakdown in the form of a Merkle root hash.

**Full schema.** Here's the full schema:

```solidity
/// Namespace for consensus-level activity summaries.
library Consensus {
    type MerkleHash is bytes32;

    // Aggregated stats for consensus-level activity.
    struct AggregatedStats {
        /// The total number of unique validators that have committed a block within this period.
        uint64 totalActiveValidators;
        /// The total number of blocks committed by all validators during this period.
        uint64 totalNumBlocksCommitted;
    }

    // The full activity summary for consensus-level activity.
    struct FullSummary {
        AggregatedStats stats;
        /// The breakdown of activity per validator.
        ValidatorData[] data;
    }

    // The compresed representation of the activity summary for consensus-level activity suitable for embedding in a checkpoint.
    struct CompressedSummary {
        AggregatedStats stats;
        /// The commitment for the validator details, so that we don't have to transmit them in full.
        MerkleHash dataRootCommitment;
    }

    struct ValidatorData {
        /// @dev The validator whose activity we're reporting about, identified by the Ethereum address corresponding
        /// to its secp256k1 pubkey.
        address validator;
        /// @dev The number of blocks committed by this validator during the summarised period.
        uint64 blocksCommitted;
    }

    /// The payload for validators to claim rewards
    struct ValidatorClaim {
        ValidatorData data;
        MerkleHash[] proof;
    }
}
```

**Future.** In the future, the consensus summary may be extended to include aspects like validator quality (e.g. rounds
missed per validator), validator fraud proofs, block proposal latency, and more. These data points may influence
slashing, rebalancing, or other actions. If you'd like to see particular datapoints included in the consensus summary,
please let us know by opening an issue.

### Rewarding validators

As checkpoints arrive, the subnet actor stores the compressed activity rollups and makes them available for handlers to
process. The `SubnetActorActivityFacet` defines two methods for validators to claim their consensus rewards. Note that
IPC does not disburse rewards, it simply facilitates the process by validating the claims and notifying a user-provided
`ValidatorRewarder` every time a valid claim is presented for the first time (preventing double claiming).

```solidity
contract SubnetActorActivityFacet is ReentrancyGuard, Pausable {
    // Entrypoint for validators to batch claim rewards in the parent subnet, for a given subnet,
    // against multiple checkpoints at once. Atomically succeeds or reverts.
    function batchSubnetClaim(
        SubnetID calldata subnet,
        uint64[] calldata checkpointHeights,
        Consensus.ValidatorClaim[] calldata claims
    ) external;

    /// Entrypoint for validators to claim their reward for doing work in the child subnet.
    function claim(
        SubnetID calldata subnet,
        uint64 checkpointHeight,
        Consensus.ValidatorData calldata data,
        Consensus.MerkleHash[] calldata proof
    ) external;
}
```

### Configuring a ValidatorRewarder to distribute rewards for a subnet

To distribute rewards to validators, deploy a custom contract that implements the `IValidatorRewarder` interface. This
contract could be a treasury of native FIL or a minter of ERC20 tokens; it could deliver rewards directly, or could
escrow them in a vault and release them after some lockup period; etc. How you manage these aspects is entirely up to
you.

```solidity
/// @dev Implement this interface and supply the address of the implementation contract at subnet creation to process
/// consensus activity summaries at this level, and disburse rewards to validators based on their block production
/// activities inside the subnet.
///
/// This interface will be called by the subnet actor when a validator presents a _valid_ proof of consensus activity,
/// via the SubnetActivityActivityFacet#claim method.
interface IValidatorRewarder {
    /// Called by the subnet actor when a validator presents a _valid_ proof of consensus activity, via
    /// SubnetActorActivityFacet#claim() or its batch equivalents.
    /// @dev This method should revert if the summary is invalid; this will cause the claim submission to be rejected.
    function notifyValidClaim(
        SubnetID calldata id,
        uint64 checkpointHeight,
        Consensus.ValidatorData calldata validatedData
    ) external;
}
```

In order to set the address of the ValidatorRewarder at subnet creation time, you can use the `--validator-rewarder`
flag when deploying the subnet actor:

```bash
ipc-cli subnet create --validator-rewarder <YOUR REWARDER ADDRESS>
```

#### Example ValidatorRewarder implementations

There are example implementations of this interface in the `examples/` folder to get you started quickly, along with
some [Hardhat scripts](https://github.com/consensus-shipyard/ipc/blob/main/contracts/tasks/validator-rewarder.ts) for
deployment:

- `ValidatorRewarderMap`: a simple implementation that tallies the blocks committed per validator in a mapping.
- `MintingValidatorRewarder`: mints ERC20 to reward block production.

### Claiming rewards

In order to automate the claiming of rewards, validators can use the `ipc-cli validator batch-claim` command.

This command takes:

- the address of the subnet the validator committed blocks to.
- the address of the ancestor subnet the rewards should be claimed from.
- the block ranges in the subnet we want to scan for eligible claims.
- the address of the validator we want to claim rewards for.

The command will when scan the subnet for eligible claims within the specified block ranges, and will then submit the
claim to the ancestor subnet using the batch claim method to amortize the gas cost.

```bash
$ ipc-cli validator batch-claim --help
validator batch claim rewards for a target subnet

Usage: ipc-cli validator batch-claim --validator <VALIDATOR> --from <FROM> --to <TO> --reward-source-subnet <REWARD_SOURCE_SUBNET> --reward-claim-subnet <REWARD_CLAIM_SUBNET>

Options:
      --validator <VALIDATOR>                        The JSON RPC server url for ipc agent
      --from <FROM>                                  The checkpoint height to claim from
      --to <TO>                                      The checkpoint height to claim to
      --reward-source-subnet <REWARD_SOURCE_SUBNET>  The source subnet that generated the reward
      --reward-claim-subnet <REWARD_CLAIM_SUBNET>    The subnet to claim reward from
```

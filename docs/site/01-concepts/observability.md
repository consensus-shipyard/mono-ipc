# Observability

## Overview

IPC's observability framework emits events throughout execution, which are recorded in a journal and transformed to Prometheus metrics.
This enables detailed monitoring and analysis of system behavior.
This is achieved through the use of the `ipc-observability` crate/library, which provides all the necessary helpers and tools to facilitate this process.

## Key features

1. **Events**: Specific events are defined and triggered throughout the codebase to capture significant occurrences or actions.
   These events encapsulate relevant data and context about what is happening within the system.

2. **Journal**: Events are recorded in a journal, which is a rotational ledger that records chronologically ordered, timestamped trace objects to log files on disk.
   The journal can also be emitted to console.

3. **Metrics**: Each event is associated with one or more Prometheus metrics.
   When an event is triggered, the corresponding metrics are updated to reflect the event's occurrence.
   This allows for real-time tracking and monitoring of various system activities and states through dashboards and alerts.

4. **Prometheus integration**: The metrics collected are designed to integrate seamlessly with Prometheus, a powerful monitoring and alerting toolkit.
   Prometheus collects and stores these metrics, enabling detailed analysis and visualization through its query language and dashboarding capabilities.

5. **ipc-observability crate**: This custom library encapsulates the logic and functionality required to define, trigger, and record events and metrics.
   It simplifies the process of adding observability to the codebase by providing ready-to-use macros, structs, and functions.

## Metrics

- `ipc_consensus_block_proposal_received_height` (IntGauge): Incremented when a block proposal is received.
- `ipc_consensus_block_proposal_sent_height` (IntGauge): Incremented when a block proposal is sent.
- `ipc_consensus_block_proposal_accepted_height` (IntGauge): Incremented if the block proposal is accepted.
- `ipc_consensus_block_proposal_rejected_height` (IntGauge): Incremented if the block proposal is rejected.
- `ipc_consensus_block_committed_height` (IntGauge): Incremented when a block is committed.
- `ipc_exec_fvm_check_execution_time_secs` (Histogram): Records the execution time of FVM check in seconds.
- `ipc_exec_fvm_estimate_execution_time_secs` (Histogram): Records the execution time of FVM estimate in seconds.
- `ipc_exec_fvm_apply_execution_time_secs` (Histogram): Records the execution time of FVM apply in seconds.
- `ipc_exec_fvm_call_execution_time_secs` (Histogram): Records the execution time of FVM call in seconds.
- `ipc_bottomup_checkpoint_created_total` (IntCounter): Incremented when a bottom-up checkpoint is created.
- `ipc_bottomup_checkpoint_created_height` (IntGauge): Sets the height of the created checkpoint.
- `ipc_bottomup_checkpoint_created_msgcount` (IntGauge): Sets the number of messages in the created checkpoint.
- `ipc_bottomup_checkpoint_created_confignum` (IntGauge): Sets the configuration number of the created checkpoint.
- `ipc_bottomup_checkpoint_signed_height` (IntGaugeVec): Sets the height of the signed checkpoint, labeled by validator.
- `ipc_bottomup_checkpoint_finalized_height` (IntGauge): Sets the height of the finalized checkpoint.
- `ipc_topdown_parent_rpc_call_total` (IntCounterVec): Incremented when a parent RPC call is made.
- `ipc_topdown_parent_rpc_call_latency_secs` (HistogramVec): Records the latency of parent RPC calls.
- `ipc_topdown_parent_finality_latest_acquired_height` (IntGaugeVec): Sets the height of the latest locally acquired parent finality.
- `ipc_topdown_parent_finality_voting_latest_received_height` (IntGaugeVec): Sets the height of the received parent finality peer vote.
- `ipc_topdown_parent_finality_voting_latest_sent_height` (IntGauge): Sets the height of the sent parent finality peer vote.
- `ipc_topdown_parent_finality_voting_quorum_height` (IntGauge): Sets the height of the parent finality quorum.
- `ipc_topdown_parent_finality_voting_quorum_weight` (IntGauge): Sets the weight of the parent finality quorum.
- `ipc_topdown_parent_finality_committed_height` (IntGauge): Sets the height of the committed parent finality.
- `ipld_resolver_ping_rtt` (Histogram): Records a ping roundtrip time.
- `ipld_resolver_ping_timeouts` (IntCounter): Incremented when a ping timed out.
- `ipld_resolver_ping_failure` (IntCounter): Incremented when a ping failed.
- `ipld_resolver_ping_success` (IntCounter): Incremented when a ping succeeded.
- `ipld_resolver_identify_failure` (IntCounter): Incremented when an identify failed.
- `ipld_resolver_identify_received` (IntCounter): Incremented when an identify info received.
- `ipld_resolver_discovery_background_lookup` (IntCounter): Incremented when a discovery background lookup started.
- `ipld_resolver_discovery_connected_peers` (IntGauge): Sets the number of discovery connected peers.
- `ipld_resolver_membership_skipped_peers` (IntCounter): Incremented when a membership provider skipped.
- `ipld_resolver_membership_routable_peers` (IntGauge): Sets the number of routable peers.
- `ipld_resolver_membership_provider_peers` (IntGauge): Sets the number of unique peers.
- `ipld_resolver_membership_unknown_topic` (IntCounter): Incremented when a membership of unknown topic received.
- `ipld_resolver_membership_invalid_message` (IntCounter): Incremented when a membership with invalid message received.
- `ipld_resolver_membership_publish_total` (IntCounter): Incremented when a membership published.
- `ipld_resolver_membership_publish_failure` (IntCounter): Incremented when a membership publish failed.
- `ipld_resolver_content_resolve_running` (IntGauge): Sets the number currently running content resolutions.
- `ipld_resolver_content_resolve_no_peers` (IntCounter): Incremented when a resolution had no peer.
- `ipld_resolver_content_resolve_success` (IntCounter): Incremented when a resolution succeeded.
- `ipld_resolver_content_resolve_failure` (IntCounter): Incremented when a resolution failed.
- `ipld_resolver_content_resolve_fallback` (IntCounter): Incremented when a resolution had a fallback.
- `ipld_resolver_content_resolve_peers` (Histogram): Records the number of peers found for a resolution from a subnet.
- `ipld_resolver_content_connected_peers` (Histogram): Records the number connected peers in a resolution.
- `ipld_resolver_content_rate_limited` (IntCounter): Incremented when a resolution was rate limited.
- `ipc_tracing_errors` (IntCounterVec): Increments the count of tracing errors for the affected event.

## Events and corresponding metrics

### BlockProposalReceived

**Description:**
Represents a block proposal received event.

**Fields:**

- `height`: The height of the block.
- `hash`: The hash of the block.
- `size`: The size of the block.
- `tx_count`: The transaction count in the block.
- `validator`: The validator that proposed the block.

**Affects metrics:**

- `ipc_consensus_block_proposal_received_height`

### BlockProposalSent

**Description:**
Represents a block proposal sent event.

**Fields:**

- `validator`: The validator that proposed the block.
- `height`: The height of the block.
- `size`: The size of the block.
- `tx_count`: The transaction count in the block.

**Affects metrics:**

- `ipc_consensus_block_proposal_sent_height`

### BlockProposalEvaluated

**Description:**
Represents the evaluation of a block proposal.

**Fields:**

- `height`: The height of the block.
- `hash`: The hash of the block.
- `size`: The size of the block.
- `tx_count`: The transaction count in the block.
- `validator`: The validator that proposed the block.
- `accept`: Whether the block proposal was accepted.
- `reason`: The reason for rejection, if any.

**Affects metrics:**

- `ipc_consensus_block_proposal_accepted_height`
- `ipc_consensus_block_proposal_rejected_height`

### BlockCommitted

**Description:**
Represents a block committed event.

**Fields:**

- `height`: The height of the block.
- `app_hash`: The application hash of the block.

**Affects metrics:**

- `ipc_consensus_block_committed_height`

### MsgExec

**Description:**
Represents an execution message for different purposes.

**Fields:**

- `purpose`: The purpose of the message execution (Check, Apply, Estimate, Call).
- `message`: The message being executed.
- `height`: The block height at which the message is executed.
- `duration`: The duration of the execution in seconds.
- `exit_code`: The exit code of the execution.

**Affects metrics:**

- `ipc_exec_fvm_check_execution_time_secs`
- `ipc_exec_fvm_estimate_execution_time_secs`
- `ipc_exec_fvm_apply_execution_time_secs`
- `ipc_exec_fvm_call_execution_time_secs`

### CheckpointCreated

**Description:**
Represents the creation of a bottom-up checkpoint.

**Fields:**

- `height`
- `hash`
- `msg_count`
- `config_number`

**Affects metrics:**

- `ipc_bottomup_checkpoint_created_total`
- `ipc_bottomup_checkpoint_created_height`
- `ipc_bottomup_checkpoint_created_msgcount`
- `ipc_bottomup_checkpoint_created_confignum`

### CheckpointSigned

**Description:**
Represents the signing of a bottom-up checkpoint by a validator.

**Fields:**

- `role`: The role of the signer (Own, Peer).
- `height`: The height of the checkpoint.
- `hash`: The hash of the checkpoint.
- `validator`: The public key of the validator who signed the checkpoint.

**Affects metrics:**

- `bottomup_checkpoint_signed_height`

### CheckpointFinalized

**Description:**
Represents the finalization of a bottom-up checkpoint.

**Fields:**

- `height`: The height of the checkpoint.
- `hash`: The hash of the checkpoint.

**Affects metrics:**

- `ipc_bottomup_checkpoint_finalized_height`

### ParentRpcCalled

**Description:**
Represents a parent RPC call.

**Fields:**

- `source`: The source of the RPC call.
- `json_rpc`: The JSON RPC method used.
- `method`: The method name of the RPC call.
- `status`: The status of the RPC call.
- `latency`: The latency of the RPC call in seconds.

**Affects metrics:**

- `ipc_topdown_parent_rpc_call_total`
- `ipc_topdown_parent_rpc_call_latency_secs`

### ParentFinalityAcquired

**Description:**
Represents the acquisition of parent finality.

**Fields:**

- `source`: The source of the finality.
- `is_null`: Indicates if the finality is null.
- `block_height`: The block height of the finality.
- `block_hash`: The block hash of the finality.
- `commitment_hash`: The commitment hash of the finality.
- `num_msgs`: The number of messages in the finality.
- `num_validator_changes`: The number of validator changes in the finality.

**Affects metrics:**

- `ipc_topdown_parent_finality_latest_acquired_height`

### ParentFinalityPeerVoteReceived

**Description:**
Represents the reception of a parent finality peer vote.

**Fields:**

- `validator`: The validator who voted.
- `block_height`: The block height of the vote.
- `block_hash`: The block hash of the vote.
- `commitment_hash`: The commitment hash of the vote.

**Affects metrics:**

- `ipc_topdown_parent_finality_voting_latest_received_height`

### ParentFinalityPeerVoteSent

**Description:**
Represents the sending of a parent finality peer vote.

**Fields:**

- `block_height`: The block height of the vote.
- `block_hash`: The block hash of the vote.
- `commitment_hash`: The commitment hash of the vote.

**Affects metrics:**

- `ipc_topdown_parent_finality_voting_latest_sent_height`

### ParentFinalityPeerQuorumReached

**Description:**
Represents the reaching of a parent finality quorum.

**Fields:**

- `block_height`: The block height of the quorum.
- `block_hash`: The block hash of the quorum.
- `commitment_hash`: The commitment hash of the quorum.
- `weight`: The weight of the quorum.

**Affects metrics:**

- `ipc_topdown_parent_finality_voting_quorum_height`
- `ipc_topdown_parent_finality_voting_quorum_weight`

### ParentFinalityCommitted

**Description:**
Represents the commitment of parent finality.

**Fields:**

- `parent_height`: The parent height of the committed finality.
- `block_hash`: The block hash of the committed finality.
- `local_height`: The local height of the committed finality.
- `proposer`: The proposer of the committed finality.

**Affects metrics:**

- `ipc_topdown_parent_finality_committed_height`

### PingEvent

**Variants and affected metrics:**

- `Success(PeerId, Duration)`: `ipld_resolver_ping_rtt`,`ipld_resolver_ping_success`

### PingFailureEvent

**Variants and affected metrics:**

- `Timeout(PeerId)`: `ipld_resolver_ping_timeouts`
- `Failure(PeerId, Duration)`: `ipld_resolver_ping_failure`

### IdentifyEvent

**Variants and affected metrics:**

- `Received(PeerId)`: `ipld_resolver_identify_received`

### IdentifyFailureEvent

**Variants and affected metrics:**

- `Failure(PeerId, String)`: `ipld_resolver_identify_failure`

### DiscoveryEvent

**Variants and affected metrics:**

- `BackgroundLookup(PeerId)`: `ipld_resolver_discovery_background_lookup`
- `ConnectionEstablished(PeerId)`: `ipld_resolver_discovery_connected_peers`
- `ConnectionClosed(PeerId)`: `ipld_resolver_discovery_connected_peers`

### MembershipEvent

**Variants and affected metrics:**

- `Added(PeerId)`: `ipld_resolver_membership_provider_peers`
- `Removed(PeerId)`: `ipld_resolver_membership_provider_peers`
- `Skipped(PeerId)`: `ipld_resolver_membership_skipped_peers`
- `PublishSuccess`: `ipld_resolver_membership_publish_total`
- `RoutablePeers(i64)`: `ipld_resolver_membership_routable_peers`

### MembershipFailureEvent

**Variants and affected metrics:**

- `PublishFailure(String)`: `ipld_resolver_membership_publish_failure`
- `GossipInvalidProviderRecord(Option<PeerId>, String)`: `ipld_resolver_membership_invalid_message`
- `GossipInvalidVoteRecord(Option<PeerId>, String)`: `ipld_resolver_membership_invalid_message`
- `GossipUnknownTopic(Option<PeerId>, TopicHash)`: `ipld_resolver_membership_unknown_topic`

### ResolveEvent

**Variants and affected metrics:**

- `Started(Cid)`: `ipld_resolver_content_resolve_running`
- `Success(Cid)`: `ipld_resolver_content_resolve_success`
- `Completed`: `ipld_resolver_content_resolve_running`
- `Peers(usize)`: `ipld_resolver_content_resolve_peers`
- `NoPeers`: `ipld_resolver_content_resolve_no_peers`
- `ConnectedPeers(usize)`: `ipld_resolver_content_connected_peers`

### ResolveFailureEvent

**Variants and affected metrics:**

- `Failure(Cid)`: `ipld_resolver_content_resolve_failure`
- `Fallback(Cid)`: `ipld_resolver_content_resolve_fallback`

### TracingError

**Description:**
Represents an error that occurs during tracing.

**Fields:**

- `affected_event`: The event affected by the error.
- `reason`: The reason for the error.

**Affects metrics:**

- `ipc_tracing_errors`

## Configuration

### Metrics configuration

The metrics can be configured via the `config.toml` configuration file for Fendermint. You can enable metrics and specify the listening host and port as follows:

```toml
[metrics]
enabled = true

[metrics.listen]
host = "127.0.0.1"
port = 9184
```

For Ethereum metrics, you can configure them similarly:

```toml
[eth.metrics]
enabled = true
```

## Tracing and journal configuration

> 🚧 Note: the event journal and general logs are currently output to the same file.
> We plan to segregate in the near future so that the event journal has its dedicated file.
> See this issue: https://github.com/consensus-shipyard/ipc/issues/1084.

Tracing can also be configured via the configuration file for Fendermint. You can set the tracing level and specify whether to log to console or file.

### Console tracing

Example config:

```toml
[tracing]

[tracing.console]
level = "trace" # Eg. "info,my_crate::module=trace" - https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
```

### File tracing

Example config:

```toml
[tracing.file]
enabled = true # Options: true, false
level = "trace" # Eg. "info,my_crate::module=trace" - https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
directory = "/path/to/log/directory"
max_log_files = 5 # Number of files to keep after rotation
rotation = "daily" # Options: minutely, hourly, daily, never
## Optional: filter events by domain
domain_filter = ["Bottomup", "Consensus", "Mpool", "Execution", "Topdown", "System"]
## Optional: filter events by event name
events_filter = ["ParentFinalityAcquired", "ParentRpcCalled"]
```

By configuring these options, you can control the behavior of metrics and tracing, enabling fine-grained monitoring and logging for your application.

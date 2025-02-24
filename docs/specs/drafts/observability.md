---
status: Implementing
authors: @raulk, @karlem
---

# Observability

## Model

Our mental model when designing our observability logic is as follows.

---

```
💥 An **event** happens at runtime
  ⇒ 📸 A trace object captures the fact, including key data about it
    ⇒ ✍️ The trace gets journaled (if the journal is enabled)
    ⇒ 📊 The trace is applied to mutate metrics
```

---

Once we capture the trace object (hopefully with negligible overhead), and we ensure it is a complete snapshot of the fact, we can call `ipc_observability::emit()`, which can send it to two sinks in parallel:

- The journal.
- The metrics.

| Done | Domain    | Subsystem      | Event                              | Trace                                                                                                          | Metrics                                                                      |
| ---- | --------- | -------------- | ---------------------------------- | -------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| x    | Topdown   | Parent syncer  | Parent RPC calls                   | ParentRpcCalled{source, json-rpc method, status, latency}                                                      | ipc_topdown_parent_rpc_call_total{source, method, status}++ (counter)        |
| x    |           |                |                                    |                                                                                                                | ipc_topdown_parent_rpc_call_latency_secs{source, method, status} (histogram) |
| x    |           |                | Parent finality locally acquired   | ParentFinalityAcquired{source, block height, block hash, commitment hash, num messages, num validator changes} | ipc_topdown_parent_finality_latest_acquired_height{source} (gauge)           |
| x    |           |                | Parent finality gossip received    | ParentFinalityPeerVoteReceived{validator, block height, block hash, commitment hash}                           | ipc_topdown_parent_finality_voting_latest_received_height{validator} (gauge) |
| x    |           |                | Parent finality peer gossip sent   | ParentFinalityPeerVoteSent{block height, block hash, commitment hash}                                          | ipc_topdown_parent_finality_voting_latest_sent_height (gauge)                |
| x    |           |                | Parent finality quorum reached     | ParentFinalityPeerQuorumReached{block height, block hash, commitment hash, weight}                             | ipc_topdown_parent_finality_voting_quorum_height (gauge)                     |
| x    |           |                |                                    |                                                                                                                | ipc_topdown_parent_finality_voting_quorum_weight (gauge)                     |
| x    |           |                | Parent finality committed on chain | ParentFinalityCommitted{parent height, block hash, local height, proposer}                                     | ipc_topdown_parent_finality_committed_height (gauge)                         |
| x    | Bottomup  | Checkpointing  | Checkpoint submitted               | CheckpointSubmitted{height, hash}                                                                              | ipc_bottomup_checkpoint_finalized_height (gauge)                             |
| x    |           |                | Checkpoint created                 | CheckpointCreated{height, hash, msg_count, config_number}                                                      | ipc_bottomup_checkpoint_created_total (counter)                              |
| x    |           |                |                                    |                                                                                                                | ipc_bottomup_checkpoint_created_height (gauge)                               |
| x    |           |                |                                    |                                                                                                                | ipc_bottomup_checkpoint_created_msgcount (gauge)                             |
| x    |           |                |                                    |                                                                                                                | ipc_bottomup_checkpoint_created_confignum (gauge)                            |
| x    |           |                | Checkpoint signed                  | CheckpointSigned{role, height, hash, validator}                                                                | ipc_bottomup_checkpoint_signed_height{validator} (gauge)                     |
| x    | Consensus | Block proposal | Block proposal received            | BlockProposalReceived{height, hash, size, tx_count, validator}                                                 | ipc_consensus_block_proposal_received_height (gauge)                         |
| x    |           |                | Block proposal sent                | BlockProposalSent{validator, height, size, tx_count}                                                           | ipc_consensus_block_proposal_sent_height (gauge)                             |
| x    |           |                | Block proposal evaluated           | BlockProposalEvaluated{height, hash, size, tx_count, validator, accept, reason}                                | ipc_consensus_block_proposal_accepted_height (gauge)                         |
| x    |           |                |                                    |                                                                                                                | ipc_consensus_block_proposal_rejected_height (gauge)                         |
| x    |           |                | Block proposal committed           | BlockCommitted{height, app_hash}                                                                               | ipc_consensus_block_committed_height (gauge)                                 |
| x    | Mpool     | Message pool   | Message received                   | MpoolReceived{message, accept, reason}                                                                         | ipc_mpool_received{accept} (counter)                                         |
| x    | Execution | VM execution   | Executing message                  | MsgExec{purpose, message, height, duration, exit_code}                                                         | ipc_exec_fvm_check_execution_time_secs (histogram)                           |
| x    |           |                |                                    |                                                                                                                | ipc_exec_fvm_estimate_execution_time_secs (histogram)                        |
| x    |           |                |                                    |                                                                                                                | ipc_exec_fvm_apply_execution_time_secs (histogram)                           |
| x    |           |                |                                    |                                                                                                                | ipc_exec_fvm_call_execution_time_secs (histogram)                            |
| x    | Tracing   | Errors         | Error while processing tracing     | TracingError{affected_event, reason}                                                                           | ipc_tracing_errors{event} (counter)                                          |

## Fine-grained VM metrics

It might be helpful for debugging/optimization workflows at development time to have a mode that uses the pre-existing FVM traces to populate actor-level call graph metrics on Prometheus. Imagine metrics like:

```promql
exec_fvm_call_stack_step_duration{message_cid, seq, from, to} <- duration (histogram)
```

**Considerations:**

- The cardinality of such a metric is huge, and the histogram type would be lossy. For this reason, we can't treat it as a production-worthy metric. But it might be useful to detect outliers or abnormalities during development.
- We’d need to provide a developer API operation to reset this metric.

## Tracing journal

The journal is a rotational ledger that records chronologically ordered, timestamped trace objects to log files on disk.
For ease of analysis and simplicity, we use JSON formatting for these objects (although we recognize that alternative binary formats such as Protocol Buffers would be more disk-efficient, but they would make the journals unreadable with the naked eye).
The tracing journal is configured via a `[tracing]` block within the Fendermint `config.toml` file. Ideally, we'd support dynamic reconfiguration, allowing for real-time application of changes to the journal settings. The configurable parameters include:

- Enable/disable the journal entirely.
- Filter by domains.
- Filter by specific events.
- Journal directory location.
- Journal file rotation policy (per-file size, retention window).

Implementation note: we're not reinventing the wheel here. It’s simply an abstraction over the `tracing` library and its appenders, all of which we already use. Refs:

- Tracing Appender: <https://docs.rs/tracing-appender/latest/tracing_appender/rolling/>
- EnvFilter Structure: <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html>

For a pretty decent reference, look at the Lotus journal: <https://github.com/filecoin-project/lotus/tree/master/journal>. We even managed to slap an alerting system on top of it without breaking everything.

## Tracing Configuration

Tracing can also be configured via the configuration file for `Fendermint`. You can set the tracing level and specify whether to log to console or file.

### Console Tracing

Example config:

```toml
[tracing]

[tracing.console]
level = "trace" # Eg. "info,my_crate::module=trace" - https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
```

### File Tracing

Example config:

```toml
[tracing.file]
enabled = true # Options: true, false
level = "trace" # Eg. "info,my_crate::module=trace" - https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
directory = "/path/to/log/directory"
max_log_files = 5 # Number of files to keep after rotation
rotation = "daily" # Options: minutely, hourly, daily, never
domain_filter = "Bottomup, Proposals, Mpool, Execution, Topdown, TracingError"
events_filter = "ParentFinalityAcquired, ParentRpcCalled"
```

By configuring these options, you can control the behavior of metrics and tracing, enabling fine-grained monitoring and logging for your application.

## Implementation

The existing metrics code paths are rather contorted and messy due to unnecessary indirection and excessive meta-programming. Read more commentary in the original pull request: <https://github.com/consensus-shipyard/ipc/pull/835>.

Take for example the `NewParentView` trace.

- Its struct is defined in the `fendermint_vm_event` crate.
- But it gets used in the `fendermint_vm_topdown` crate, where the event happens and the trace object gets emitted.
- But its metrics appliers are defined in the `fendermint_app` crate. Even worse: as a generic `tracing::Layer` which has lost all typing of the original event, so now it's force to introspect and match by string. Also, it only supports u64, and lacks support for labels (which would be hell to introduce on top of these macros).

We propose a radical simplification and cleanup.

- Every crate that produces traces has a conventional `observe` package. This package contains all trace structs and metric definitions. [This code was nice](https://github.com/consensus-shipyard/ipc/blob/a33cfee763cafaafdd00ad38a1367ee262e75fa4/ipld/resolver/src/stats.rs#L6).
- The `observe` package exposes a public `register_metrics(&Registry)` function so that the binary can register its metrics in its Prometheus Registry.
- An `ipc_observability` crate exposes a simple `Recordable` and `Traceable` traits. Every event needs to implement these traits either manually or with a help of provided macros.
- An `ipc_observability` exposes `emit` function that will take an event `Recordable` + `Traceable` and saves it to journal an updates metrics.

  ```rust
  trait Recordable {
      fn record_metrics();
  }
  ```

  ```rust
  pub trait Traceable {
    fn trace_level(&self) -> TraceLevel;
    fn domain(&self) -> &'static str;
    fn name() -> &'static str;
  }
  ```

- All trace structs implement `Recordable` (even if no-op), and they mutate the metrics within their crate only.
- When the emitting code performs `ipc_observability:emit(&trace);` and that records the metrics and sends them to the journal (potentially in parallel).

Note: the new `ipc_observability` crate contains: the `Recordable` trait, `emit` function,`tracing` related function configuration, general-purpose metrics utils and macros..

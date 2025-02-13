# IPC Spec - Snapshots

The snapshot mechanism in CometBFT allows a node to join a network and sync not from genesis, but from a recent snapshot offered by its peers.

# Snapshot methods in ABCI

The ABCI spec itself has a few methods dedicated for snapshots, all working over P2P. CometBFT cannot specify what a snapshot is, given it has no idea what the transactions or the ledger looks like. The snaphots itself is basically a blob that the application has to handle, but the requests and responses have some metadata that we can make use of in the [`Snapshot`](https://docs.cometbft.com/v0.37/spec/abci/abci++_methods#snapshot) type. Regardless of format, CometBFT expects that snapshots can be downloaded in chunks (e.g. 1-16MB pieces).

- [`ListSnapshots`](https://docs.cometbft.com/v0.37/spec/abci/abci++_methods#listsnapshots): Return a list of the metadata of recently made snapshots to a remote node that wants to join the network.
- [`OfferSnapshot`](https://docs.cometbft.com/v0.37/spec/abci/abci++_methods#offersnapshot): CometBFT offers to the application the choice whether to accept or reject a snapshot, based on its metadata such as its format or sender. If the offer is accepted, CometBFT will start downloading chunks.
- [`LoadSnapshotChunk`](https://docs.cometbft.com/v0.37/spec/abci/abci++_methods#loadsnapshotchunk): CometBFT is asking the application to retrieve a specific chunk of a specific snapshot (identified by `height` and `format`) and return the binary content.
- [`ApplySnapshotChunk`](https://docs.cometbft.com/v0.37/spec/abci/abci++_methods#applysnapshotchunk): CometBFT is passing a downloaded chunk to the application, along with its `sender`. Depending on the level of sophistication, the application can do incremental validation of the snapshot chunks (for example in the case of IPLD, every chunk, when loaded in order, should only contain blocks which are referenced by a previously visited CID), and if that fails it can ask CometBFT to re-download it, or blacklist the `sender`. When the last of the chunks has been handed to the application, it is expected to restore the state from them (either incrementally, or by combining all the chunks and loading them).

These are [implemented](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/src/app.rs#L882-L1053) by the `App`, not in the interpreters. The reason is that unlike the ABCI methods that participate in *Executions*, these do not change the ledger state, they do not react to messages like transactions, only handle pieces of metadata *about* the state, which doesn’t need an interpreter stack.

At the high level, if the snapshot functionality is enabled, it will run the snapshot background task at a fixed interval after a new block is committed. The snapshot background task simply captures the metadata, such as block height/hash, scans the entire blockchain state, starting from state root, bundles them into a `CAR` file.

# IPLD State Snapshots

By and large the snapshots include the actor state and the `FvmStateParams`, ie. the things needed for the FVM to process a block, nothing else.

There are two main components to consider:

- [`fendermint_vm_interpreter::fvm::state::snapshot`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/snapshot.rs) module contains the low-level constructs to export/import the actor state to/from [CAR](https://ipld.io/specs/transport/car/) files.
- [`fendermint_vm_snapshot`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/vm/snapshot) crate contains the [STM](https://crates.io/crates/async-stm) enabled machinery to produce and handle snapshots.

The former is associated with the state it is supposed to act on, while the latter is self-contained sub-system running in the background to create them, split them into chunks and recombine them, communicating with the `App` through STM.

The snapshot system has the following major components:

- [`SnapshotManifest`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/snapshot/src/manifest.rs) is essentially the metadata we keep about each snapshot and map onto the CometBFT type
- [`SnapshotManager`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/snapshot/src/manager.rs) runs in the background and maintains snapshot made from the local state. Its parameters include:
    - `snapshots_dir` is the file system directory to store snapshots at
    - `chunk_size` is the target size for snapshot chunks in bytes
    - `hist_size` is the number of snapshots to keep (2-3 is the recommendation)
    - `block_interval` is the number of blocks between snapshots, e.g. 10,000 means creating a block at every height divisible by 10,000.
- [`SnapshotState`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/snapshot/src/state.rs) holds the STM variables that constitute the state of the snapshoting system:
    - `snapshots` is the list of exports available on the file system. Each snapshot is in its own sub-directory, with chunks in individual files. The list contains `SnapshotItem` which is capable of loading individual chunks, or importing the whole snapshot into the `Blockstore` ; it also remembers the last time it was accessed, so we have some grace period before removing old snapshots if a client is actively downloading it.
    - `latest_params` is the last block height and hash the manager was notified about. If the manager is busy exporting the previous snapshot, it might skip some of these notifications, as it only reacts to them when it’s not already doing work.
    - `current_download` is the state of any snapshot currently being fed to the manager by CometBFT, which is being stored in a temporary directory until all chunks are available.
- [`SnapshotClient`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/snapshot/src/client.rs) shares its `SnapshotState` with the `SnapshotManager` and is the interface for the `App` to send notification and run queries over the state, as well as to save chunks coming from CometBFT to files and kick off the state restoration when all are present.
- [`car`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/vm/snapshot/src/car) module has utilities such as `ChunkWriter`, `BlockStreamer` and `split` to divide CAR files into chunks in a way that no IPLD `Block` is split across files.

The following methods are worth looking at:

- `SnapshotManager::run` to see how notifications are handled
- `SnapshotManager::create_snapshot` to see steps involved in creating a snapshot:
    - create a snapshot from a given block height and FVM parameters
    - export the snapshot CAR file to a temporary directory
    - record its size and create a checksum (these go into the manifest)
    - split the CAR file into chunks with the configured target size
    - create the manifest and export it to JSON
    - move the temporary directory to the final place under the manager
- `SnapshotClient::save_chunk` handles chunks from remote peers:
    - checks that it’s the next expected chunk (CometBFT loads them sequentially)
    - saves them to a part file
    - if it’s the last chunk (according to the manifest), it runs the checksum and returns the `SnapshotItem` ready to be loaded
- `App::apply_snapshot_chunk` calls `SnapshotClient::save_chunk` and if it gets back a `SnapshotItem`, it loads the contents into its `Blockstore` and finally sets the committed state.
- `App::commit` calls `SnapshotClient::notify` with the current block height and the committed state parameters.
- `tmconv::to_snapshot` maps the fields of a `SnapshotManifest` to the CometBFT type, in particular it encodes the `FvmStateParams` into the `metadata` field
- `tmconv::from_snapshot` tries to restore a `SnapshotManifest` from an offer, comparing the CID of the decoded `FvmStateParams` (which is the `AppHash` of Fendermint) to the `app_hash` in the offer, thus checking the integrity that the offer is coming from a remote which, according to the Light Client, has a consensus that agrees with the snapshot (unless they lie about the snapshot contents).

<aside>
💡 There are some comments in `App::commit` regarding how the state is related to block height. Remember that state which is the result of executing block `H` is only available to query at height `H+1`, which is reflected in `AppState::state_height` that we use to determine at which height the state is inserted into the state history.

When we do snapshots, however, we have to use height `H` with the state parameters that we save at `H+1`. The reason for this is because in `offer_snapshot` containing the snapshot we produced for `H` CometBFT will actually give us the `app_hash` at `H+1`, and this way things line up for validation.

In other words, a CometBFT `Snapshot` for height `H` contains the state which is the result of the execution of block `H`, which CometBFT allows to query with height `H+1`.

</aside>

# Snapshot Configuration

Snapshots are not enabled by default. To enabling them requires configuration in two places:

- in the [Fendermint config](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/app/config/default.toml#L68), e.g. by setting the `FMT_SNAPSHOT__ENABLED` env var
- in the [CometBFT config](https://docs.cometbft.com/v0.37/core/configuration) by setting multiple variables:
    - `CMT_STATESYNC_ENABLE` to allow CometBFT to bootstrap from others’ snapshots
    - `CMT_STATESYNC_RPC_SERVERS` with a list of at least 2 CometBFT peers where the Light Client can go and catch up with the state of the chain before CometBFT starts asking for snapshots
    - `CMT_STATESYNC_TRUST_HEIGHT` is a height of a recent block which the operator knows to be on the correct chain
    - `CMT_STATESYNC_TRUST_HASH` is the hash of that block

See the documentation of [State Sync](https://docs.cometbft.com/v0.37/core/state-sync) for how to obtain such trusted values.

The recommended practice by Tendermint is never to trust a block header older than the unbonding period of the Proof-of-Stake consensus, to protect against long range attacks. Within the unbonding period, if evidence of an equivocation arises, the offending validators can still be slashed, but outside that period, there is much less that can be done. The default trust period in CometBFT is 1 week.

The [`snapshot-test`](https://github.com/consensus-shipyard/ipc/tree/specs/fendermint/testing/snapshot-test) is an end-to-end test that demonstrates the system in action. The setup consists of the following nodes:

- A default standalone validator node, producing blocks.
- Two full nodes that connect directly to the validator node to sync from genesis, but with a very short retained block history size, so nobody can sync with them from genesis, and peer-exchange disabled, so nobody can discover the standalone validator node through them. This leaves anyone connecting to them to sync from snapshots.
- A fourth node that connects to both full nodes above, but *not* the validator node. The setup script uses `curl` to obtain a trusted height from the running full nodes, configure their addresses for `statesync` using the env vars above. The test verifies that the node is able to sync the chain, which it can only do using snapshots.

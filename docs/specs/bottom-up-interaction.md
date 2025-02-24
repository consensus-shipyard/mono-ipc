# Bottom Up Interactions
This document takes a closer look in the IPC mechanics involved in information flowing from the child to the parent subnet, a.k.a. bottom-up.

# Interactions

There are two user initiated interactions in IPC that result in bottom-up messages being sent:

- `release` sends tokens from a user account on the child subnet to another on the parent subnet
- general cross-net messages sent to, or through, the parent

The mechanism for propagating information from the child to the parent is through *checkpoints*.

# Checkpoints

The epic for implementing checkpointing is [here](https://github.com/consensus-shipyard/ipc/issues/211). The end-to-end workflow can be followed on this [diagram](https://github.com/consensus-shipyard/ipc/blob/main/docs/fendermint/checkpointing.md). The IPLD Resolver docs also present a [use case](https://github.com/consensus-shipyard/ipc/blob/specs/ipld/resolver/docs/README.md#checkpointing) for checkpointing.

## Contents

The original idea for a checkpoint was to contain the following information:

- `subnet_id`: to identify to the parent subnet which child the checkpoint is for, and to prevent any replay attacks across subnets run by the same validators
- `block_height` : the height of the child subnet blockchain at the time of checkpoint creation
- `block_hash`: the hash of the block whose execution results in a checkpoint being added to the ledger, to prevent long range attacks on the subnet by anchoring it to the parent chain
- `next_configuration_number`: this is the identifier of the validator set which is going to sign the *next* checkpoint; the current checkpoint is always going to be signed by the *current* validator set

To these would be added fields to carry the contents of the checkpoint, which would be either:

- `messages`, which would be a list of bottom-up cross-net messages, or
- `messages_cid`, which would be the CID of the messages, but not the payload itself, which would be procured by the [IPC Spec - IPLD Resolver](https://www.notion.so/IPC-Spec-IPLD-Resolver-7b4290a0d60c40cdba98cd6d3e66648b?pvs=21)

The CID based approach would only work with Fendermint, not Lotus running on rootnet, but in general a commitment based approach can work with Lotus too.

## Triggers

Originally checkpoints were supposed to be submitted at regular intervals, which was governed by the parameters with which the child subnet contract was created on the parent subnet, and made part of the `IpcParams` in [`genesis`](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/genesis/src/lib.rs#L227). However, this presented a problem with the number of messages that could be included in a checkpoint, which is why later the triggers for checkpoint creation were amended to be any of the following conditions:

- a fixed period in terms of block height
- the number of enqueued bottom-up messages being over a [limit](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/GatewayDiamond.sol#L68C37-L68C55)
- the number of enqueued bottom-up messages being under the immediate checkpoint limit, but having waited some maximum amount of time already

If any of these conditions are met, a checkpoint is added to the ledger.

One side effect of adding extra conditions is that it makes transactions irregular in their appearance, and so the parent cannot verify which is the next checkpoint to admit. For this reason, it is planned to add  a new `prev_checkpoint_height`, so that checkpoints can form a chain, and then the parent subnet will only accept the next checkpoint if it points at the last submitted one as its predecessor.

When the number of enqueued bottom-up messages exceeds the limit, a new message batch is created and committed at the current epoch. This will trigger a new checkpoint to be [created](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/lib/LibGateway.sol#L272).

## Creation

Where checkpoint creation fits into the process is explained in [IPS Spec - Executions](https://www.notion.so/IPS-Spec-Executions-ebf13d833d6845ec9c11b59bd514fcda?pvs=21).

Creating a checkpoint in the ledger is performed deterministically by every full node; they simply call the [`gateway`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/state/ipc.rs) contract with the following inputs:

- an unsigned `BottomUpCheckpoint`
- the root hash of a Merkle tree which consists of the current power table, ie. the public keys and powers of the current validator set
- the total power of the validators

## Signatures

After the checkpoint has been added to the ledger *and committed in a block*, those nodes which are currently validators broadcast transactions which add their signatures using the `broadcast_signature` function of the [`checkpoint`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/checkpoint.rs) module.

The reason we wait for the the change to be committed is so that the transactions that add the signatures don’t get rejected by `check_tx` because they are referring to a non-existing checkpoint.

The signing and sending of transactions happens in the [`broadcast`](https://github.com/consensus-shipyard/ipc/blob/specs/fendermint/vm/interpreter/src/fvm/broadcast.rs) module which fetches the current nonce of the validator, estimates the gas, performs retries, etc. Because it fetches the nonce for each submission, it cannot be used in parallel.

<aside>
💡 To submit transactions the validators of the subnet need to have an Ethereum account with sufficient tokens to cover the gas cost. They can use `fund` in order bring in tokens from the parent subnet. 

The fact that validators have to pay to submit transactions to the subnet they validate may seem harsh; intuitively these should be free! But remember that a subnet can contain Byzantine validators who might abuse their privileges of free transaction submission. However, there is no reason why Fendermint couldn’t contain extra logic to compensate successful submissions for their costs.

</aside>

The signature transactions are sent to the child ledger, where they accumulate until a quorum is reached, that is, more that 2/3 of the total power of validators have signed the checkpoint.

Note that by this time the CometBFT validators could be different, which is why signature submissions contain a Merkle proof that shows that the submitter was indeed part of the committee responsible for signing a past checkpoint.

## Relayers

Once a checkpoint has gathered a quorum of signatures in the child ledger, it can be picked up by a *relayer* and submitted in the form of a transaction to the parent subnet.

Ideally we would like some redundancy in the number of validators, so there is no single point of failure.

Relayers generally should be rewarded for their service. There are numerous opinions on how to implement rewards, each with their drawback:

- *Only the first submitter gets a reward.* To achieve redundancy the reward would have to be multiple of the cost. Depending on how likely it is to beat the fastest relayer (a parent validator might insert their own transaction to steal the rewards), it might make it unprofitable for multiple validators to operate.
- *The first N submitters get rewards.* It is easy for any relayer to submit N transactions in a Sybil attack to reap all rewards, hampering redundancy.
- *All submitters in a fixed time period get equal share of a fixed reward.* This takes out the competition aspect, and discourages Sybil attacks because the reward doesn’t grow. It should lead to a dynamic equilibrium of the number of relayers. However if the fixed time window is too wide, it encourages freeloaders who just repeat the first submission, which would make it look like there is redundancy where there isn’t.

<aside>
💡 Fendermint has a naive implementation of the fixed reward scheme divided between all relayers, however due to a vulnerability this was removed for now and relayers get no rewards.

</aside>

## Validation

The parent subnet contains a smart contract specific to the child subnet which can validate the contents of the checkpoint. Currently the checkpoint submission will contain the multisig of the validators as proof of quorum.

The parent subnet is the source of the validator power distribution, but it’s the child subnet that communicates through checkpointing how far ahead it has synchronised the changes in validator powers; this is signalled by the *configuration ID*, with different power tables being different configurations.

The parent knows what the last committed configuration ID of the subnet is, and it expects these validators to be the ones who sign the checkpoint, with sufficient weight to form a quorum.

Once the quorum has been verified, the `next_configuration_id` informs the parent how far ahead it can apply the pending validator updates on the active validator set, and thus know who to expect the *next* checkpoint to be signed by.

## Execution

When the checkpoint is submitted to the Lotus rootnet, it is currently expected to either contain all bottom-up messages or that they accompany the checkpoint in a different way, and only a commitment is in the checkpoint. However in both cases the bottom-up messages would be executed and their gas cost paid for by the relayer.

With a Fendermint parent network, the same thing works if that’s how the smart contracts are implemented. There was another way laid out it in the epic above, which involved the IPLD Resolver procuring the checkpoint payload from the subnet based on a CID, and executing the messages implicitly when the validators decide that they all have the data available. In this case the relayer would have only paid for the *validation* of the checkpoint, not its *execution*.

Both of these schemes suffer from the fact that the gas limit of the messages included in the checkpoint is unknown when the checkpoint is made (and it *cannot* be known, as by definition the gas cost depends on where the message will be executed):

- In the case where a relayer executes the messages, they first have to estimate the gas cost, so at least it is known before the checkpoint-bearing transaction is included in a rootnet block, and thus the block gas limit can be observed. However, if the gas spent by the checkpoint would exceed the block gas limit, the checkpoint will never be included, but at the same time no other checkpoint can be produced by the subnet, and thus checkpointing stalls.
- In the case where the validators execute messages implicitly, they can choose whether to include it in a block or not, but to do so they would need to estimate the cost at some point, and again it might exceed the limits. Implicit execution also makes it difficult to deal with errors, in particular there is no room for retries.

To overcome this issue, ideally cross messages would *not* be executed in the block where the checkpoint is included. Instead either just a commitment would be stored, or messages would be parked in inboxes (e.g. organised by sender account). The senders could come later and initiate their own transactions to kick off the execution of the messages delivered as part of the IPC consensus mechanism, at which point they can pay for the gas and retry as many times as they see fit if they run out of gas.

<aside>
💡 Because of the untractable nature or cross-net message gas limits, currently only `fund` and `release` messages are allowed.

</aside>

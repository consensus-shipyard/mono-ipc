# Subnet Validator Membership
Before proceeding, this doc assumes you have preliminary understanding of Subnet’s lifecycle.

At the time of this writing, the child subnet blockchain is running on top of [CometBFT](https://docs.cometbft.com/v0.37/), whose Ignite consensus is an optimised variant of PBFT. As such, the consensus is driven by a form of voting by a group of validators. Subnet validation deals with validator management in the child subnet, more specifically:

- **Validator power assignment**: how each validator obtains its voting power
- **Validator quorum**: how the validator agree on the child subnet changes
- **Validator set changes:** how each validator joins and exits the voting validators

## Validator Changes

The validators obtain their power from `SubnetActor` , mainly `FederatedPower`, where the power comes from the assignment of the subnet owner, and `Collateral`, where the power comes from the collateral staked in the subnet . This is described in detail in the lifecycle section. Do note that power assignment happens in the parent subnet, the power needs to be propagated down to the child subnet.

Validator membership changes are handled differently depending on the lifecycle stage the subnet is in:

1. If the subnet is yet to be bootstrapped, any changes applied to the parent will be tracked and merged onto the *staged* genesis for the subnet, inside the SubnetActor. The initial powers of the validators are tracked there, and ultimately committed onto a final genesis once the subnet is bootstrapped.
2. After the subnet is bootstrapped, the power propagation happens as a combination of topdown finality and bottom up checkpoint confirmation.

The high level flow of the process is as follows:

- The topdown syncer captures the validator membership changes and stores them in the child gateway contract.
- The bottom-up checkpoint applies these changes locally in the child subnet, i.e. update the validator powers and applies the changes to cometbft
- Relayer picks up the bottom up checkpoint once a checkpoint quorum is reached. The submitted bottom up checkpoint will trigger validator changes execution in the parent.

The reason for this round trip is because `fendermint` requires the existing child validator set to acknowledge the changes coming from the parent. A quorum must be formed in the child validators so that they know which validator set will be the next voting validators.

Once the validator power change is triggered, the parent will record down this [change](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/lib/LibStaking.sol#L516) as a configuration change. Each operation is assigned a monotonically increasing `u64` as its id, which is called `ConfigurationNumber`. The parent subnet will track the `lastConfigurationNumber` and the `nextConfigurationNumber`, every time a validator change is stored, `nextConfigurationNumber` is incremented, every time a validator change is confirmed from the child subnet, `lastConfigurationNumber` is incremented by 1. One can think of this as a `head` and `tail` in a `Deque`.

The topdown finality in `fendermint` will capture these configuration changes and [store](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/fendermint/vm/interpreter/src/chain.rs#L342) them in the child. The child also tracks the configuration number from the parent, so that the child and parent will align on the configuration change. The stored configuration changes will not be applied immediately but wait until a bottom up checkpoint is created. This is to batch the changes. Once a bottom up checkpoint is created, it will apply all the validator changes stored in the child subnet and obtain the configuration number of the last applied validator change. This configuration number will be included in the child bottom up checkpoint and sent to the parent. The validators in the child subnet must vote on the validator changes throw the bottom up checkpoint mechanism. They will form a quorum when the voting power of voted validators is greater than or equal to a certain threshold of the total voting power of all validators, currently this value is `0.67`. Once a quorum is formed, the bottom up checkpoint is sent to the parent and the validator changes are confirmed.

The parent will validate the signatures of bottom up checkpoint to ensure a quorum is actually reached. This implies that the parent also needs to track the current validator set in the child. Adding to the complexity is that `cometbft` can only support a certain number of validators. This means the parent must restrict the number of validators in the child. As such, validators are divided into `ActiveValidator` and `WaitingValidator`.

## Active Vs Waiting Validators

For `FederatedPower` permission mode, the number of validators is not expected to get very big. However, `Collateral` permission mode might have quite a large number of addresses staking as validator. There is a configuration parameter in the subnet: `maxValidatorLimit`. This controls the max number of validators that can vote at any point in time.

If the number of validators exceeds this limit, then some validators will have to wait for their turn. This is basically ranked by their power in descending order. Top `maxValidatorLimit` validators are active validators, they participant in child subnet voting. The rest of the validators are waiting validators. They will be promoted to active validators when existing validators leave the subnet or they obtain more power.

This is tracked by using a max priority queue for waiting validators and a min priority queue for active validators in the parent. The high level idea is that if there is a validator change, we will get the validator with lowest power from the min priority queue and getting the validator with max power in the max priority queue. If waiting validator’s power is greater than that of the active validator, the active validator will be popped from the active validator queue while the waiting validator will be popped from the waiting validator queue. The active validator will be inserted into the waiting queue and waiting validator will be inserted into the active queue.

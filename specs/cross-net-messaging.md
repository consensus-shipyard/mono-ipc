# IPC Cross-net Messaging

## Cross-net messages

All general-purpose cross-net messages are propagated wrapped into an `IpcEnvelope`. This means that when the gateway receives the a `CrossMsg` for execution, it won’t directly perform a `send` using the `method`, `to` and `params` specified in the `StorableMsg` , but it will directly forward the original `IpcEnvelope` being propagated to a specific method of the destination contract that will be responsible for unwrapping the cross-net message and conveniently handling it.

- 💡 This assumes that contracts need to be implemented to explicitly be able to unwrap and handle general-purpose cross-net messages. We can use for instance `abi.encode("IpcEntrypoint(IpcEnvelope)")` as the entrypoint that contracts need to expose where they handle the execution of general-purpose cross-net messages. In the future, we should provide a library or a SDK to make it easy for contracts to support IPC messages.

In order to simplify the implementation of the `IpcEntrypoint` function, we will add a new `kind` field in `IpcEnvelope` used to clearly identify the type of cross-net message being propagated.

- Both, `Call` and `Receipt` cross-net message types are propagated to the destination contract by calling the `IpcEntrypoint` with the full `IpcEnvelope` as an argument.
    
    ```solidity
    enum IpcMsgKind {
        Transfer, // use for fund, release and cross-net message that move funds (i.e. no data, so not a smart contract call
        Call, // for general-purpose messages
        Receipt,  // for ack messages
    }
    
    struct IpcEnvelope {
        IpcMsgKind kind;
    		// abi.encode of the underlying IpcMsgKind 
    		bytes message;
    		// outgoing nonce for the envelope.
    		// this is set by the gateway when committing the message for propagation.
        uint64 nonce;
        // fee and gas limit were used to pay for the execution
        // of messages in destination. Not currently used for the
        // the first implementation where the gas model has been descoped.
        uint256 fee;
        uint256 gasLimit;
    }
    
    // Used for both Call and Transfer messages
    struct IpcMsg {
        IPCAddress from;
        IPCAddress to;
        uint256 value;
        bytes4 method;
        bytes params;
    }
    
    struct ReceiptMsg {
        IPCAddress to;
    		bool success;   // we can optionally use an int flag instead of bool
        bytes32 id;     // id of the message being ack'ed. This id is the keccak of IpcMsg
    	  bytes reason;   // byte encoded failure reason (as in try-catch)
    }
    ```
    
- The `id` of an `Receipt` message is determined through the `keccak(IpcEnvelope)` of the message being acknowledged.
    - `sendXnetMessage` should be extended to return the keccak of the message, for the contract to retain and correlate the result with later
- Only the execution of `Call` messages trigger `Receipt`, for plain `Transfer` messages, i.e. `fund` and `release` , no `Ack` are needed (at least for now that we are subsidizing these).

## General Message Propagation and Relaying

A high-level overview of the process is shown in the following diagram:

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/75c9b610-402a-494d-9887-8258d6cc60b5/329c75e9-3642-4a7b-aa6a-bbd23f76db25/Untitled.png)

### Top-down flow

- A smart contract looking to send a top-down general-purpose cross-net message will call a new `sendXNetMessage`  method in the gateway actor (blue flow in the diagram).
- The gateway actor will fetch and apply the `SubnetPolicy` from the corresponding subnet actor to determine if the cross-net message is allowed to be propagated.
- If it is allowed, the subnet actor will commit the cross-net message in the gateway for propagation by calling what is now the `sendUserXnetMessage` method (that we should probably rename with a better name).
    - This function should be modified so from now on it only accept messages from the subnet actor of a registered subnet.
- The gateway then commits the top-down message for propagation that is propagated through a PoF.
- When the PoF is committed in the child, the execution of the cross-net message is performed by calling the `applyMsg` method in the gateway.
    - This method will apply the `SubnetPolicy` in the child to the message to see if it can be executed.
    - If it is allowed to be executed, the gateway will perform a smart contract call to the `abi.encode("IpcEntrypoint(CrossMsg)")` for the corresponding cross-net message.

### Bottom-up flow

The bottom-up flow is analogous to the one for top-down.

- A smart contract looking to send a bottom-up general-purpose cross-net message will call a new `sendXnetMessage` method in the gateway of the child subnet (red flow in the diagram).
- The gateway will apply its `SubnetPolicy` to determine if the cross-net message is allowed to be propagated.
    - If it is allowed, the gateway will commit the bottom-up message for propagation in the next message batch.
- When the message batch is propagated for execution by relayers calling `sendBottomUpMsgBatch` in the subnet actor at the parent, the `SubnetPolicy` is enforced over the general-purpose cross-net messages, and conveniently executed and forwarded to the relevant `abi.encode("IpcEntrypoint(CrossMsg)")` of the destination contract.

Remember that `sendXnetMessage` should include checks when committing the message that the `value` in the `IpcMsg` has been provided to the envelop and conveniently burnt or locked in the origin network.

## Message Routing

If the crossnet destination is NOT the current network (network where the gateway is running), we [add it to the postbox for further propagation](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/src/lib/LibGateway.sol#L401C5-L405C74). In this way messages can span multiple IPC networks in order to reach their final destination. 

## Message Failure

What happens if the execution of a message fails once it’s been propagated to the destination subnet, either because the execution policy rejects it, because it ran out of gas, etc.?

- When `applyMsg` is called in the gateway for both, top-down and bottom-up messages, if something fails in that function or when performing the method call to the relevant contract. We can use [Solidity try-catch](https://solidity-by-example.org/try-catch/) to catch the success or failure of the contract call. When the call fails, the `appliedNonce` for the subnet should be increased so subsequent messages can be accepted for execution.
- To notify the source contract about the success or failure of the cross-net message, a new general-purpose `Ack` cross-net message is committed for propagation in the gateway as a bottom-up or top-down message respectively.
    - This message is propagated as any other general-purpose cross-net message setting the `to` of the message to the `from` of the original cross-net message that triggered the ACK, with the relevant feedback about the execution in the `success` and `reason` fields.
    - As part of the `IpcEntrypoint` of the destination contract, handlers should be implemented for both: `Ack` and `Wrapped` cross-net messages. An `Ack` with `success = false` should compensate any side-effects that may have been triggered with the propagation of the cross-net message. Implementing the logic for this reversion is also the responsibility of the contract  developer, as it will be use case-specific.
    - Additional logic may be required in Fendermint to trigger the commitment of a new ACK when the execution of a message fails before the logic of `applyMsg` can even be triggered, or if the failure can't trigger the ACK logic. This will happen, for instance, when the execution of a message reaches the gas limit.

![Untitled](https://prod-files-secure.s3.us-west-2.amazonaws.com/75c9b610-402a-494d-9887-8258d6cc60b5/5438965d-53c5-4b1b-b3a7-b6906c8eb3ee/Untitled.png)

## Fees and Gas costs

Currently the relayer has the fees and rewards disabled. Currently, these are the gas costs incurred by a user when sending a general cross-net message.

- Top-down
    - Propagation and commitment: The caller of the contract triggering the cross-net message implicitly pays immediately to add their message to the queue for propagation, and its commitment in the origin network.
    - Execution: Top-down messages are executed implicitly in the child subnet when a top-down finality is committed. The execution of these message are, thus, implicitly paid by validators in the child.
    - ACK: They are triggered in the execution of top-down messages (which is done implicitly), while their execution is performed by relayers through the submission of a bottom-up batch.
- Bottom-up:
    - Propagation and commitment:  The caller of the contract triggering the cross-net message implicitly pays immediately for queuing the message for propagation.
    - Execution: Relayers pay the gas fees for the execution of these message through the submission of bottom-up batches. This execution also triggers the commitment of the corresponding ACKs.
    - ACKs are triggered through the execution of bottom-up messages in the parent by relayers, and their execution is implicit in the child.

So we see that without a clear gas model, the bulk of the cost model falls on the relayers and the child validators. Child validators implicitly pay by dedicating resources for the execution of messages for free, while relayers need to pay the execution of message explicitly in Filecoin.

A malicious looking to disrupt the system could launch an attack where it sends a lot of low-value general-purpose message to an allowed contract in the subnet or the parent (these messages do not necessarily need to be correct). This would lead to the corresponding execution and ACK propagation. By sending lots of these messages, the user could manage to drain the funds from relayers.

- If this happens we could implement an ad-hoc protection mechanism to allow the firewall policy (or relayers) to filter and skip messages from the offending user.

## IPC SDK for writing IPC-enabled contracts

In order for smart contracts to be IPC-enabled and be able to accept general cross-net message, they need to implement the `handleIpcMessage` that the gateway will call when `Call` and `Receipt` messages are forwarded to their address. 

We provide an IPC SDK for contracts to become “IPC-enabled”. Contracts that implement IPC General message passing extend the [IpcExchange contract](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/sdk/IpcContract.sol#L13) and implement their own overrides for `_handleIpcCall` and `_handleIpcResult`. Extending the [IpcExchange contract](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/contracts/sdk/IpcContract.sol#L13) also provides access to the method `performIpcCall` which allows for a contract to send a message to another IPC enabled contract.

## Sending a xnet message

### SDK Details

To send messages a contract calls the method `performIpcCall`that is defined in the SDK:

```solidity
abstract contract IpcExchange is IIpcHandler, Ownable, ReentrancyGuard {
    /// @notice Method the implementation of this contract can invoke to perform an IPC call.
    function performIpcCall(
        IPCAddress memory to,
        CallMsg memory callMsg,
        uint256 value
    ) internal nonReentrant returns (IpcEnvelope memory envelope) {
        // Queue the cross-net message for dispatch.
        envelope = IGateway(gatewayAddr).sendContractXnetMessage{value: value}(
            IpcEnvelope({
                kind: IpcMsgKind.Call,
                from: to, // TODO: will anyway be replaced by sendContractXnetMessage.
                to: to,
                nonce: 0, // TODO: will be replaced.
                value: value,
                message: abi.encode(callMsg)
            })
        );
        // Add the message to the list of inflights
        bytes32 id = envelope.toHash();
        inflightMsgs[id] = envelope;
    }
    function handleIpcMessage(IpcEnvelope calldata envelope) external payable onlyGateway returns (bytes memory) {
        // internal dispatch of the cross-net message to the right method.
        if (envelope.kind == IpcMsgKind.Call) {
            CallMsg memory call = abi.decode(envelope.message, (CallMsg));
            return _handleIpcCall(envelope, call);
        } else if (envelope.kind == IpcMsgKind.Result) {
            ResultMsg memory result = abi.decode(envelope.message, (ResultMsg));

            // Recover the original message.
            // If we were not tracking it, or if some details don't match, refuse to handle the receipt.
            IpcEnvelope storage orig = inflightMsgs[result.id];
            if (orig.message.length == 0 || keccak256(abi.encode(envelope.from)) != keccak256(abi.encode(orig.to))) {
                revert IIpcHandler.UnrecognizedResult();
            }

            /// Note: if the result handler reverts, we will
            _handleIpcResult(orig, envelope, result);
            delete inflightMsgs[result.id];
            return EMPTY_BYTES;
        }
        revert UnsupportedMsgKind();
    }

    /// @notice Function to be overridden by the child contract to handle incoming IPC calls.
    ///
    /// NOTE: It's fine for this method to revert. If that happens, IPC will carry the error to the caller.
    function _handleIpcCall(
        IpcEnvelope memory envelope,
        CallMsg memory callMsg
    ) internal virtual returns (bytes memory);

    /// @notice Function to be overridden by the child contract to handle results from previously performed IPC calls.
    ///
    /// NOTE: This must not revert as doing so will leave the correlation map in an inconsistent state.
    /// (IPC will consider the result delivery attempted, and will not repeat it again).
    function _handleIpcResult(
        IpcEnvelope storage original,
        IpcEnvelope memory result,
        ResultMsg memory resultMsg
    ) internal virtual;

}
```

In addition to calling performIpcCall, contracts that intend to send Ipc Messages using the SDK shall override the function `_handleIpcResult` which will process the message confirmation receipt. 

### Implementation Example

`performCall` is utilized in the example implementation for the [Linked Token Contract](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/extras/linked-token/src/LinkedToken.sol#L113) in the function linkedTransfer. This method is used to communicate to its linked contract that a token has been locked up and should be minted in the linked subnet. Additionally the contract uses this method to inform a linked contract that a token was burned and should be released on the corresponding subnet. The confirmation receipt method `_handleIpcResult` implementation validates the message, removes the transfer from the `_unconfirmedTransfers` map, and issues a token refund if the transaction has failed. 

```solidity
function _linkedTransfer(address recipient, uint256 amount) internal returns (IpcEnvelope memory committed) {
    _validateInitialized();

    // Validate that the transfer parameters are acceptable.
    _validateTransfer(recipient, amount);

    // Lock or burn, depending on concrete implementation.
    _captureTokens(msg.sender, amount);

    // Pack the message to send to the other side of the linked token.
    CallMsg memory message = CallMsg({
        method: abi.encodePacked(bytes4(keccak256("receiveLinked(address,uint256)"))),
        params: abi.encode(recipient, amount)
    });
    IPCAddress memory destination = IPCAddress({
        subnetId: _linkedSubnet,
        rawAddress: FvmAddressHelper.from(_linkedContract)
    });

    // Route through GMP.
    committed = performIpcCall(destination, message, 0);

    // Record the unconfirmed transfer.
    _addUnconfirmedTransfer(committed.toHash(), msg.sender, amount);

    emit LinkedTokensSent({
        underlying: address(_underlying),
        sender: msg.sender,
        recipient: recipient,
        id: committed.toHash(),
        nonce: committed.nonce,
        value: amount
    });
}

function _handleIpcResult(
    IpcEnvelope storage original,
    IpcEnvelope memory result,
    ResultMsg memory resultMsg
) internal override {
    _validateInitialized();
    _validateEnvelope(result);

    OutcomeType outcome = resultMsg.outcome;
    bool refund = outcome == OutcomeType.SystemErr || outcome == OutcomeType.ActorErr;

    _removeUnconfirmedTransfer({id: resultMsg.id, refund: refund});
}
```

### Receiving a xnet message

To receive properly implement the IpcExchange interface and receive Xnet messages, the method `_handleIpcCall` shall be overridden. Validation should be done on the message to ensure that it is originated from the expected subnet origin and contract address origin. In addition there is a field in the call message data that specifies a function selector that should be validated.

### Implementation Example

The implementation details of how the [Linked token example](https://github.com/consensus-shipyard/ipc/blob/7af25c4c860f5ab828e8177927a0f8b6b7a7cc74/extras/linked-token/src/LinkedToken.sol#L165) overrides the function `_handleIpcCall` is show below. The function initially starts by validated that the contract is initialized, the envelope containing the Ipc message has an expected origin contract, subnet and is targeted to the correct function selected. The call message data is decoded and then the appropriate internal function implementation `_receiveLinked` is called.

```
function _handleIpcCall(
    IpcEnvelope memory envelope,
    CallMsg memory callMsg
) internal override returns (bytes memory) {
    _validateInitialized();
    _validateEnvelope(envelope);
    _requireSelector(callMsg.method, "receiveLinked(address,uint256)");

    (address receiver, uint256 amount) = abi.decode(callMsg.params, (address, uint256));

    _receiveLinked(receiver, amount);
    return bytes("");
}

```

_handleIpcCall may raise an exception, for example if the validate methods fail. This result whether the method succeeds or fails is routed to the `_handleIpcResult` callback in the contract that originated the ipc message send. The `ResultMsg` data type that is passed in to `_handleIpcResult` has a field with type `OutcomeType` that contains one of three values: `Ok`, `SystemErr`, and `ActorErr`. The value `Ok` indicates that the transaction succeeded. The remaining two are error types. `SystemErr`  indicates the error was related to the Ipc system and `ActorErr` indicates that the error was due to the transaction reverting.

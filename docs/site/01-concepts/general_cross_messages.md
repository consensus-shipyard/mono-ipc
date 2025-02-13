## Overview

> :warning: **Caution**
> This document does not focus on simple transfers using the Transfer message kind (e.g., Gateway#release, an abstraction
> over Transfer). Instead, it covers the general message–building blocks implemented with the Call message kind, which enable
> communication across L2+ subnets and provide a foundation for higher-level communication abstractions.

General cross-net messages enable the transmission of arbitrary messages between IPC subnets. They are particularly useful for transferring tokens (when supported, more on this later) or transmitting arbitrary data. This is not a feature that allows users to trigger messages from wallets; rather, it is a framework for cross-subnet contract-to-contract communication.

Messages are transmitted across subnets using top-down and bottom-up mechanisms. For example, if a message is sent from a parent to a child subnet, the top-down mechanism is used, and vice versa. Similarly, messages between subnets not directly connected but sharing a common parent are routed automatically by the IPC system.

## Limitations

- Only contracts (not externally owned accounts, or EOAs) can invoke cross-net messages.
- Token transfers are only allowed if the source subnet, destination subnet, and all intermediate subnets share the same token.
- Since cross-net messages flow through the entire route between the source and destination subnets, multi-hop messages may take longer to be delivered.

## Usage

Messages are passed via the `Gateway` contract, which exposes the following method for triggering cross-net messages (only callable by another contract):

```solidity
function sendContractXnetMessage(
    IpcEnvelope calldata envelope
) external payable returns (IpcEnvelope memory committed);
```

The actual message is embedded within the IPC envelope as ABI-encoded bytes.

### IPC Envelope Structure

```solidity
struct IpcEnvelope {
    /// @dev type of message being propagated.
    IpcMsgKind kind;
    /// @dev outgoing nonce for the envelope.
    uint64 localNonce;
    /// @dev original nonce of the message from the source network.
    uint64 originalNonce;
    /// @dev Value being sent in the message.
    uint256 value;
    /// @dev destination of the message
    IPCAddress to;
    /// @dev address sending the message
    IPCAddress from;
    /// @dev abi.encoded message
    bytes message;
}

```

### Message Types of Interest

- **Call**: Represents a request sent to a destination contract. If the destination account is an externally owned account (EOA), the `Call` will fail and return an error in the `Result`.
- **Result**: Represents the response to a `Call` and `Transfer` messages, which could either be the result or an error report.

### Contracts overview

The source and destination contracts can be any contracts that implement the [IIpcHandler](../../contracts/sdk/interfaces/IIpcHandler.sol) interface.

```solidity
function handleIpcMessage(IpcEnvelope calldata envelope) external payable returns (bytes memory ret);
```

The primary method for handling messages in a contract is the `handleIpcMessage` function from the IIpcHandler interface. The `handleIpcMessage` method is triggered either by the execution of messages included in a bottom-up checkpoint or by the execution of a finalized top-down finality message. Its return value is an ABI-encoded message, automatically added to the message field of the `Result` IpcEnvelope. If `handleIpcMessage` reverts, the error is propagated back as a Result with the `ActorErr` outcome type, but does not revert the caller.

Although basic tasks such as decoding messages and handling errors must be implemented manually, you can simplify development by using the `IpcExchange` abstract contract from the [IPC SDK](../../contracts/sdk/IpcContract.sol). This contract provides convenience and ease of use for managing inter-process communication.

### IPC SDK

The IPC SDK simplifies development by providing an automatic implementation of the `handleIpcMessage` function and a convenient `performIpcCall` method.

- **performIpcCall**:
  This method calls `sendContractXnetMessage` on the Gateway and registers the message in an internal `inflightMsgs` map, indicating that a corresponding Result message is expected.
- **handleIpcMessage**:
  When a message arrives, `handleIpcMessage` checks whether it is a `Call` or a `Result`.
  For `Call` messages, it decodes the payload and invokes `_handleIpcCall`.
  For `Result` messages, it verifies that the message corresponds to a tracked inflight message in the `inflightMsgs` map, then calls `_handleIpcResult`. If no matching entry is found, the process reverts with `UnrecognizedResult`.

Both `_handleIpcCall` and `_handleIpcResult` must be implemented by the contract that extends the IPC SDK’s abstract `IpcExchange` contract. It is safe to revert within these functions: any revert will be returned in a `Result` message with the `ActorErr` outcome type, but will not revert the overall execution.

When designing these handlers, ensure the contract remains within reasonable gas usage and block size constraints. For example, you can:

- Offload heavy computations off-chain.
- Use “pull” over “push” patterns for transfers.
- Optimize Solidity code (e.g., minimize storage writes, use mappings).
- Break large tasks into smaller ones.

Following these practices helps keep function calls within practical limits and ensures they can be included in a block successfully.

The return value of `_handleIpcCall` becomes the return value of handleIpcMessage—an ABI-encoded message automatically added to the message field of the `Result` `IpcEnvelope`.

```solidity
function _handleIpcCall(
    IpcEnvelope memory envelope,
    CallMsg memory callMsg
) internal virtual returns (bytes memory);
```

```solidity
function _handleIpcResult(
    IpcEnvelope storage original,
    IpcEnvelope memory result,
    ResultMsg memory resultMsg
) internal virtual;
```

For an example, refer to [CrossMessengerCaller.sol](../../contracts/contracts/examples/CrossMessengerCaller.sol).

## Results

Results provide either error propagation or responses from the destination contract back to the caller. This creates a request-response mechanism where a `Call` or `Transfer` represents the request and a `Result` represents the response.

The IPC envelope will have a `kind` of `Result`, and the message will follow the standardized schema below (see [CrossNet.sol](../../contracts/contracts/structs/CrossNet.sol)):

```solidity
struct ResultMsg {
    /// @dev ID of the envelope the result belongs to.
    bytes32 id;
    /// @dev Outcome of the call (success or type of error).
    OutcomeType outcome;
    /// @dev ABI-encoded return value or failure reason.
    bytes ret;
}
```

### Outcome Types

- **Ok**: Message was delivered successfully, and the `ret` field contains the response from the called contract.
- **SystemErr**: An error occurred during message transmission. The return value indicates the `InvalidXnetMessageReason` (see [IPCErrors.sol](../../contracts/contracts/errors/IPCErrors.sol)).
- **ActorErr**: A custom error returned by the called contract.

### System Errors

During message propagation, certain `SystemErr` errors can occur. These errors are returned in the `Result` message (via the `ret` field in `ResultMsg`) and then propagated back to the caller. The possible system errors are:

- **Sender**: The message sender is an externally owned account (EOA), which is not permitted.
- **DstSubnet**: The destination subnet address is invalid or does not exist.
- **Nonce**: The message nonce does not match the expected nonce.
- **Value**: The transferred value is invalid (applicable only to Transfer messages, where value must not be zero).
- **Kind**: The message type (kind) is invalid.
- **ReflexiveSend**: The message’s source subnet matches its destination subnet (sending a message to itself).
- **NoRoute**: The message must travel upward, but there is no common parent subnet available.
- **IncompatibleSupplySource**: The supply source of the subnets is incompatible or mismatched.

## How It Works

1. A custom contract calls `sendContractXnetMessage` on the local `Gateway` with a message inside the IPC envelope (of kind `Call`). The envelope includes the destination subnet address, which is prefixed with the destination subnet path and is used for routing.
2. If the message passes validation, the gateway determines whether to send it upward (bottom-up) to a parent subnet or downward (top-down) to a child subnet. If it fails validation, the gateway rejects it.
3. If the message arrives at an intermediate subnet that is not the destination, it is stored in a postbox for further propagation.
4. Messages in the postbox are processed automatically. Based on their path, they are either sent to the parent via a bottom-up checkpoint or to the child subnet via a top-down message.
5. Steps 3 and 4 are repeated until the message either encounters an error or reaches the destination subnet. If an error occurs, a `Result` message is sent back to the source subnet.
6. Upon reaching the destination, the message is executed by invoking the handleIpcMessage method on the destination contract. A Result message is then sent back to the source subnet, which is possible because the IPC envelope includes the relayer’s address.
7. Steps 3 and 4 repeat until the `Result` message arrives at the source subnet, where it is executed on the original contract.

## Debugging

During message propagation, the following events are emitted by the subnet gateway, providing useful insights for debugging. Each event includes a unique message ID for cross-subnet tracking:

- **`NewTopDownMessage`**: Emitted when a message is sent downward to a child subnet.
- **`QueuedBottomUpMessage`**: Emitted when a message is prepared for inclusion in a bottom-up checkpoint.
- **`MessageStoredInPostbox`**: Emitted when a message is received by an intermediate subnet and stored for further propagation.
- **`MessagePropagatedFromPostbox`**: Emitted when a message is sent from the postbox to the next subnet.

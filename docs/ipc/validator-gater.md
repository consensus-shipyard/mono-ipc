# Validator Gater

## Overview

The Validator Gater feature allows the interception of validator-related actions, such as staking, unstaking, and explicit validator membership adjustments (federated membership), based on user-defined policies. By implementing a custom smart contract that adheres to the `IValidatorGater` interface, developers can enforce custom logic to either permit or deny these actions.

This feature is designed to support both federated and collateral-based networks, providing flexibility to manage validator permissions and validator power assignments through an external gating contract.

---

## Validator Gater Interface

The core of the Validator Gater feature is the `IValidatorGater` interface. It allows the interception of validator actions and defines custom logic to allow or reject actions such as staking, unstaking, or adjusting validator power. This extra layer ensures network participation aligns with the user-defined policy set in the contract.

### Interface Definition

```solidity
/// @title Validator Gater interface
/// @notice This interface introduces the ability to intercept validator power updates before execution.
/// @dev Power updates may result from staking, unstaking, or explicit validator membership adjustments (federated membership).
/// This interface introduces a mechanism to allow or deny validator actions, according to a user-defined policy.
interface IValidatorGater {
    /// This intercepts the power update call.
    /// @param id The identifier of the subnet.
    /// @param validator The address of the validator.
    /// @param prevPower The previous power of the validator.
    /// @param newPower The new power of the validator.
    /// @notice Reverts if the power update is not allowed.
    function interceptPowerDelta(SubnetID memory id, address validator, uint256 prevPower, uint256 newPower) external;
}
```

Any action that modifies validator power (whether through staking, unstaking, or explicit validator assignment) will trigger a call to this method, allowing the contract to validate or reject the action.

See [the interface definition](../../contracts/contracts/interfaces/IValidatorGater.sol) and an [example implementation](../../contracts/contracts/examples/SubnetValidatorGater.sol).

---

## Subnet Creation and Gater Contract Integration

When creating a subnet, users can provide a Validator Gater contract address to enforce custom policies. The `--validator-gater` parameter in the subnet creation command accepts a contract address, which the system will call every time a validator-related action occurs.

### Example Subnet Creation with Validator Gater

```bash
ipc-cli subnet create --validator-gater $CONTRACT_ADDRESS
```

### Callback Mechanism

Once a gater contract address is set, each validator-related action (staking, unstaking, or power adjustment) will trigger a callback to the gater contract. This allows the contract to determine whether the action is permitted based on the logic defined within it.

### Lifecycle Compatibility

The Validator Gater functionality applies throughout the entire lifecycle of the subnet, whether the subnet is in pre-activation or activated state. This ensures that any validator actions are checked consistently, regardless of the subnet's current status.

---

## Permission Modes

The Validator Gater feature supports two primary modes of operation:

### 1. Federated Permission Mode

In a federated network, the gater contract can directly manage validator power using the `setFederatedPower` method. This method is typically used when validator power is explicitly set by a central authority or policy.

#### Example Command

```bash
ipc-cli subnet set-federated-power
```

This allows for centralized control over which validators participate and with what power.

### 2. Collateral-based Permission Mode

For networks operating in collateral-based mode, the Validator Gater contract will intercept validator actions such as joining the network, staking, unstaking, and leaving.

#### Example Commands

```bash
ipc-cli subnet join
ipc-cli subnet stake
ipc-cli subnet unstake
ipc-cli subnet leave
```

These commands interact with the gater contract to ensure that only approved actions proceed, based on the policy defined in the gater contract.

---

## Implementation Guide

### 1. Implementing a Custom Gater Contract

Developers must implement the `IValidatorGater` interface to create a custom gater contract. This contract will define the rules and conditions under which validators can perform staking, unstaking, and power adjustment actions.

### 2. Deploying the Gater Contract

After implementing the gater contract, deploy it to the network. Then, during subnet creation, pass the contract address using the `--validator-gater` parameter.

### 3. Enforcing Validator Actions

Once the gater contract is set, it will automatically intercept validator-related actions and enforce the rules defined within the contract. This gives the subnet creator full control over who can participate and how the network is governed.

### 4. Example Implementation

Please see an example implementation [here]().

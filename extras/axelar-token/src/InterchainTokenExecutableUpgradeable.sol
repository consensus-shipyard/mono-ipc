// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import {IInterchainTokenExecutable} from "@axelar-network/interchain-token-service/interfaces/IInterchainTokenExecutable.sol";

/**
 * @title InterchainTokenExecutable
 * @notice Abstract contract that defines an interface for executing arbitrary logic
 * in the context of interchain token operations.
 * @dev This contract should be inherited by contracts that intend to execute custom
 * logic in response to interchain token actions such as transfers. This contract
 * will only be called by the interchain token service.
 */

abstract contract InterchainTokenExecutableUpgradeable is Initializable, IInterchainTokenExecutable {
    error NotService(address caller);

    address public interchainTokenService;

    bytes32 internal constant EXECUTE_SUCCESS = keccak256("its-execute-success");

    /**
     * @notice Creates a new InterchainTokenExecutable contract.
     * @param interchainTokenService_ The address of the interchain token service that will call this contract.
     */
    function __InterchainTokenExecutable_init(address interchainTokenService_) public onlyInitializing {
        interchainTokenService = interchainTokenService_;
    }

    /**
     * Modifier to restrict function execution to the interchain token service.
     */
    modifier onlyService() {
        if (msg.sender != interchainTokenService) revert NotService(msg.sender);
        _;
    }

    /**
     * @notice Executes logic in the context of an interchain token transfer.
     * @dev Only callable by the interchain token service.
     * @param commandId The unique message id.
     * @param sourceChain The source chain of the token transfer.
     * @param sourceAddress The source address of the token transfer.
     * @param data The data associated with the token transfer.
     * @param tokenId The token ID.
     * @param token The token address.
     * @param amount The amount of tokens being transferred.
     * @return bytes32 Hash indicating success of the execution.
     */
    function executeWithInterchainToken(
        bytes32 commandId,
        string calldata sourceChain,
        bytes calldata sourceAddress,
        bytes calldata data,
        bytes32 tokenId,
        address token,
        uint256 amount
    ) external virtual onlyService returns (bytes32) {
        _executeWithInterchainToken(commandId, sourceChain, sourceAddress, data, tokenId, token, amount);
        return EXECUTE_SUCCESS;
    }

    /**
     * @notice Internal function containing the logic to be executed with interchain token transfer.
     * @dev Logic must be implemented by derived contracts.
     * @param commandId The unique message id.
     * @param sourceChain The source chain of the token transfer.
     * @param sourceAddress The source address of the token transfer.
     * @param data The data associated with the token transfer.
     * @param tokenId The token ID.
     * @param token The token address.
     * @param amount The amount of tokens being transferred.
     */
    function _executeWithInterchainToken(
        bytes32 commandId,
        string calldata sourceChain,
        bytes calldata sourceAddress,
        bytes calldata data,
        bytes32 tokenId,
        address token,
        uint256 amount
    ) internal virtual;
}

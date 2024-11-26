// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.23;

import {FvmAddress} from "../structs/FvmAddress.sol";
import {SubnetID, IPCAddress} from "../structs/Subnet.sol";
import {IpcEnvelope, IpcMsgKind, CallMsg} from "../structs/CrossNet.sol";
import {IGateway} from "../interfaces/IGateway.sol";
import {EMPTY_BYTES, METHOD_SEND} from "../../contracts/constants/Constants.sol";

/// This is a simple helper contract for Materializer to test cross messages.
contract CrossMessenger {
    address internal gatewayAddr;

    function setGatewayAddress(address gateway) external {
        gatewayAddr = gateway;
    }

    function getGatewayAddress() external view returns (address) {
        return gatewayAddr;
    }

    function invokeCrossMessage(
        IPCAddress memory from,
        IPCAddress memory to
    ) external payable {
        CallMsg memory message = CallMsg({method: abi.encodePacked(METHOD_SEND), params: EMPTY_BYTES});
        IpcEnvelope memory envelope = IpcEnvelope({
            kind: IpcMsgKind.Call,
            from: from,
            to: to,
            value: msg.value,
            message: abi.encode(message),
            nonce: 0
        });

        IGateway(gatewayAddr).sendContractXnetMessage{value: msg.value}(envelope);
    }
}
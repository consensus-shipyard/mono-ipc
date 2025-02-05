// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

import {LinkedTokenReplicaV2} from "../contracts/v2/LinkedTokenReplicaV2.sol";

contract LinkedTokenReplicaV2Extension is LinkedTokenReplicaV2 {
    function newFunctionReturns8() public returns (uint256) {
        return 8;
    }
}

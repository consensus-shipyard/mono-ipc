// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.23;

import {LinkedTokenStorage} from "./lib/LibLinkedTokenStorage.sol";
import {LibLinkedToken} from "./lib/LibLinkedToken.sol";

import {IDiamond} from "@ipc/src/interfaces/IDiamond.sol";
import {IDiamondCut} from "@ipc/src/interfaces/IDiamondCut.sol";
import {IDiamondLoupe} from "@ipc/src/interfaces/IDiamondLoupe.sol";
import {IERC165} from "@ipc/src/interfaces/IERC165.sol";
import {LibDiamond} from "@ipc/src/lib/LibDiamond.sol";

import {SubnetID} from "@ipc/src/structs/Subnet.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";


error FunctionNotFound(bytes4 _functionSelector);

contract LinkedTokenDiamond {

    LinkedTokenStorage internal s;

    struct ConstructorParams {
        address gateway;
        address underlyingToken;
        SubnetID linkedSubnet;
    }


    constructor(IDiamond.FacetCut[] memory _diamondCut, ConstructorParams memory params) {

        LibDiamond.setContractOwner(msg.sender);
        LibDiamond.diamondCut({_diamondCut: _diamondCut, _init: address(0), _calldata: new bytes(0)});

        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        // adding ERC165 data
        ds.supportedInterfaces[type(IERC165).interfaceId] = true;
        ds.supportedInterfaces[type(IDiamondCut).interfaceId] = true;
        ds.supportedInterfaces[type(IDiamondLoupe).interfaceId] = true;

        LibLinkedToken.setUnderlyingToken(params.underlyingToken);
        LibLinkedToken.setGateway(params.gateway);
        LibLinkedToken.setLinkedSubnet(params.linkedSubnet);

    }

    function _fallback() internal {
        LibDiamond.DiamondStorage storage ds;
        bytes32 position = LibDiamond.DIAMOND_STORAGE_POSITION;
        // get diamond storage
        // slither-disable-next-line assembly
        assembly {
            ds.slot := position
        }
        // get facet from function selector
        address facet = ds.facetAddressAndSelectorPosition[msg.sig].facetAddress;
        if (facet == address(0)) {
            revert FunctionNotFound(msg.sig);
        }
        // Execute external function from facet using delegatecall and return any value.
        // slither-disable-next-line assembly
        assembly {
            // copy function selector and any arguments
            calldatacopy(0, 0, calldatasize())
            // execute function call using the facet
            let result := delegatecall(gas(), facet, 0, calldatasize(), 0, 0)
            // get any return value
            returndatacopy(0, 0, returndatasize())
            // return any return value or error back to the caller
            switch result
            case 0 {
                revert(0, returndatasize())
            }
            default {
                return(0, returndatasize())
            }
        }
    }

    /// @notice Will run when no functions matches call data
    fallback() external payable {
        _fallback();
    }

    /// @notice Same as fallback but called when calldata is empty
    receive() external payable {
        _fallback();
    }
}

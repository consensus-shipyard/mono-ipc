// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "openzeppelin-contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { Script, console2 as console } from "forge-std/Script.sol";
import "../src/IpcTokenHandler.sol";
import "../src/IpcTokenSender.sol";

contract Deploy is Script {
    function setUp() public {}

    function deployTokenHandlerImplementation() public {
        string memory network = vm.envString("DEST_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(network, "__PRIVATE_KEY"));

        console.log("deploying token handler implementation to %s...", network);

        vm.startBroadcast(privateKey);
        IpcTokenHandler initialImplementation = new IpcTokenHandler();
        vm.stopBroadcast();

        console.log("token handler implementation deployed on %s: %s", network, address(initialImplementation));
        string memory key = "out";
        vm.serializeString(key, "network", network);

        string memory path = getPath();
        string memory json = vm.serializeAddress(key, "token_handler_implementation", address(initialImplementation));
        vm.writeJson(json, path, ".dest");
    }

    function deployTokenHandler() public {

        string memory network = vm.envString("DEST_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(network, "__PRIVATE_KEY"));

        console.log("deploying token handler to %s...", network);
        checkPathExists();
        string memory path = getPath();

        console.log("loading handler implementation address...");
        string memory readJson = vm.readFile(path);
        address handlerAddrImplementation = vm.parseJsonAddress(readJson, ".dest.token_handler_implementation");
        console.log("handler implementation address: %s", handlerAddrImplementation);


        address axelarIts= vm.envAddress(string.concat(network, "__AXELAR_ITS_ADDRESS"));
        address ipcGateway= vm.envAddress(string.concat(network, "__IPC_GATEWAY_ADDRESS"));

        bytes memory initCall = abi.encodeCall(IpcTokenHandler.initialize, (axelarIts, ipcGateway));

        vm.startBroadcast(privateKey);
        TransparentUpgradeableProxy transparentProxy = new TransparentUpgradeableProxy(handlerAddrImplementation, address(msg.sender), initCall);
        vm.stopBroadcast();

        IpcTokenHandler handler = IpcTokenHandler(address(transparentProxy));

        console.log("token handler deployed on %s: %s", network, address(handler));
        string memory key = "out";
        vm.serializeString(key, "network", network);

        string memory json = vm.serializeAddress(key, "token_handler", address(handler));
        string memory finalJson = vm.serializeAddress(json, "token_handler_implementation", handlerAddrImplementation);
        vm.writeJson(finalJson, path, ".dest");
    }

    function getPath() public returns (string memory path) {
            path = string.concat(vm.projectRoot(), "/out/addresses.json");
            if (!vm.exists(path)) {
                vm.writeJson("{\"dest\":{\"token_handler\":{}, \"token_handler_implementation\":{} },\"src\":{\"token_sender\":{}, \"token_sender_implementation\":{}}}", path);
            }
    }

    function checkPathExists() public {
        string memory path = string.concat(vm.projectRoot(), "/out/addresses.json");
        require(vm.exists(path), "no addresses.json; please run DeployTokenHandler on the destination chain");
    }

    function deployTokenSenderImplementation() public {
    }
    function deployTokenSender() public {
        string memory originNetwork = vm.envString("ORIGIN_NETWORK");
        string memory destNetwork = vm.envString("DEST_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(originNetwork, "__PRIVATE_KEY"));
        checkPathExists();
        string memory path = getPath();

        console.log("loading handler address...");
        string memory json = vm.readFile(path);
        address handlerAddr = vm.parseJsonAddress(json, ".dest.token_handler");
        console.log("handler address: %s", handlerAddr);

        console.log("deploying token sender to %s...", originNetwork);

        // Deploy the sender on Mumbai.
        vm.startBroadcast(privateKey);


        address axelarIts= vm.envAddress(string.concat(destNetwork, "__AXELAR_ITS_ADDRESS"));
        string memory destinationChain= vm.envString(string.concat(destNetwork, "__AXELAR_CHAIN_NAME"));

        bytes memory initCall = abi.encodeCall(IpcTokenSender.initialize, (axelarIts, destinationChain, handlerAddr));

        IpcTokenSender initialImplementation = new IpcTokenSender();
        TransparentUpgradeableProxy transparentProxy = new TransparentUpgradeableProxy(address(initialImplementation), address(msg.sender), initCall);
        IpcTokenSender sender = IpcTokenSender(address(transparentProxy));

        vm.stopBroadcast();

        console.log("token sender deployed on %s: %s", originNetwork, address(sender));

        string memory key = "out";
        vm.serializeString(key, "network", originNetwork);
        json = vm.serializeAddress(key, "token_sender", address(sender));
        vm.writeJson(json, path, ".src");
    }
}

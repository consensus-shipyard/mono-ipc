// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "openzeppelin-contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Script, console2 as console} from "forge-std/Script.sol";
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

    function deployTokenHandlerProxy() public {
        string memory network = vm.envString("DEST_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(network, "__PRIVATE_KEY"));

        console.log("deploying token handler to %s...", network);
        checkPathExists();
        string memory path = getPath();

        console.log("loading handler implementation address...");
        string memory readJson = vm.readFile(path);
        address handlerAddrImplementation = vm.parseJsonAddress(readJson, ".dest.token_handler_implementation");
        console.log("handler implementation address: %s", handlerAddrImplementation);

        address axelarIts = vm.envAddress(string.concat(network, "__AXELAR_ITS_ADDRESS"));
        address ipcGateway = vm.envAddress(string.concat(network, "__IPC_GATEWAY_ADDRESS"));
        address handlerAdmin = vm.envAddress(string.concat(network, "__HANDLER_ADMIN_ADDRESS"));

        bytes memory initCall = abi.encodeCall(IpcTokenHandler.initialize, (axelarIts, ipcGateway, handlerAdmin));

        vm.startBroadcast(privateKey);
        TransparentUpgradeableProxy transparentProxy = new TransparentUpgradeableProxy(
            handlerAddrImplementation,
            handlerAdmin,
            initCall
        );
        vm.stopBroadcast();

        IpcTokenHandler handler = IpcTokenHandler(address(transparentProxy));

        console.log("token handler deployed on %s: %s", network, address(handler));
        string memory key = "out";
        vm.serializeString(key, "network", network);

        string memory json = vm.serializeAddress(key, "token_handler_proxy", address(handler));
        json = vm.serializeAddress(key, "token_handler_implementation", handlerAddrImplementation);
        vm.writeJson(json, path, ".dest");
    }

    function getPath() public returns (string memory path) {
        path = string.concat(vm.projectRoot(), "/out/addresses.json");
        if (!vm.exists(path)) {
            vm.writeJson(
                '{"dest":{"token_handler_proxy":{}, "token_handler_implementation":{}, "token_handler_implementation_v2":{} },"src":{"token_sender_proxy":{}, "token_sender_implementation":{}, "token_handler_implementation_v2":{}}}',
                path
            );
        }
    }

    function checkPathExists() public {
        string memory path = string.concat(vm.projectRoot(), "/out/addresses.json");
        require(vm.exists(path), "no addresses.json; please run DeployTokenHandler on the destination chain");
    }

    function deployTokenSenderImplementation() public {
        string memory originNetwork = vm.envString("ORIGIN_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(originNetwork, "__PRIVATE_KEY"));
        checkPathExists();
        string memory path = getPath();

        console.log("deploying token sender implementation to %s...", originNetwork);

        vm.startBroadcast(privateKey);
        IpcTokenSender initialImplementation = new IpcTokenSender();
        vm.stopBroadcast();

        console.log("token sender implementation deployed on %s: %s", originNetwork, address(initialImplementation));
        string memory key = "out";
        vm.serializeString(key, "network", originNetwork);

        string memory json = vm.serializeAddress(key, "token_sender_implementation", address(initialImplementation));
        vm.writeJson(json, path, ".src");
    }

    function deployTokenSenderProxy() public {
        string memory originNetwork = vm.envString("ORIGIN_NETWORK");
        string memory destNetwork = vm.envString("DEST_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(originNetwork, "__PRIVATE_KEY"));
        checkPathExists();
        string memory path = getPath();

        console.log("loading handler proxy address...");
        string memory json = vm.readFile(path);
        address handlerAddr = vm.parseJsonAddress(json, ".dest.token_handler_proxy");
        console.log("handler proxy address: %s", handlerAddr);

        console.log("loading sender implementation address...");
        address senderImplementationAddr = vm.parseJsonAddress(json, ".src.token_sender_implementation");
        console.log("sender implementation: %s", handlerAddr);

        console.log("deploying token sender to %s...", originNetwork);

        // Deploy the sender on Mumbai.
        vm.startBroadcast(privateKey);

        address axelarIts = vm.envAddress(string.concat(destNetwork, "__AXELAR_ITS_ADDRESS"));
        string memory destinationChain = vm.envString(string.concat(destNetwork, "__AXELAR_CHAIN_NAME"));
        address senderAdmin = vm.envAddress(string.concat(originNetwork, "__SENDER_ADMIN_ADDRESS"));

        bytes memory initCall = abi.encodeCall(
            IpcTokenSender.initialize,
            (axelarIts, destinationChain, handlerAddr, senderAdmin)
        );

        TransparentUpgradeableProxy sender = new TransparentUpgradeableProxy(
            address(senderImplementationAddr),
            senderAdmin,
            initCall
        );

        vm.stopBroadcast();

        console.log("token sender deployed on %s: %s", originNetwork, address(sender));

        string memory key = "out";
        vm.serializeString(key, "network", originNetwork);
        json = vm.serializeAddress(key, "token_sender_proxy", address(sender));
        json = vm.serializeAddress(key, "token_sender_implementation", senderImplementationAddr);
        vm.writeJson(json, path, ".src");
    }
}

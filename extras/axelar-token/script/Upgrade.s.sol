// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "openzeppelin-contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Script, console2 as console} from "forge-std/Script.sol";
import "../src/IpcTokenHandler.sol";
import "../src/IpcTokenSender.sol";

import "../src/v2/IpcTokenHandlerV2.sol";
import "../src/v2/IpcTokenSenderV2.sol";

contract Upgrade is Script {
    function setUp() public {}

    function deployTokenSenderV2Implementation() public {
        string memory originNetwork = vm.envString("ORIGIN_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(originNetwork, "__PRIVATE_KEY"));
        checkPathExists();
        string memory path = getPath();
        string memory readJson = vm.readFile(path);

        console.log("deploying token sender v2 implementation to %s...", originNetwork);

        vm.startBroadcast(privateKey);
        IpcTokenSenderV2 v2Implementation = new IpcTokenSenderV2();
        vm.stopBroadcast();


        console.log("token sender v2 implementation deployed on %s: %s", originNetwork, address(v2Implementation));
        string memory key = "out";
        vm.serializeString(key, "network", originNetwork);

        string memory json = vm.serializeAddress(key, "token_sender_implementation_v2", address(v2Implementation));
        json = vm.serializeAddress(key, "token_sender_proxy", vm.parseJsonAddress(readJson, ".src.token_sender_proxy"));
        json = vm.serializeAddress(key, "token_sender_implementation", vm.parseJsonAddress(readJson, ".src.token_sender_implementation"));
        vm.writeJson(json, path, ".src");
    }


    function upgradeTokenSenderProxy() public {
        string memory network = vm.envString("ORIGIN_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(network, "__PRIVATE_KEY"));

        console.log("upgrading token sender on %s...", network);
        checkPathExists();
        string memory path = getPath();

        console.log("loading sender implementation address...");
        string memory readJson = vm.readFile(path);
        address newSenderAddrImplementation = vm.parseJsonAddress(readJson, ".src.token_sender_implementation_v2");
        console.log("sender implementation address: %s", newSenderAddrImplementation);

        console.log("loading sender proxy address...");
        address senderAddr = vm.parseJsonAddress(readJson, ".src.token_sender_proxy");
        console.log("sender proxy address: %s", senderAddr);

        console.log("loading handler proxy address...");
        address handlerAddr = vm.parseJsonAddress(readJson, ".dest.token_handler_proxy");
        console.log("handler proxy address: %s", handlerAddr);

        address axelarIts = vm.envAddress(string.concat(network, "__AXELAR_ITS_ADDRESS"));
        string memory destinationChain = vm.envString(string.concat(network, "__AXELAR_CHAIN_NAME"));
        address senderAdmin = vm.envAddress(string.concat(network, "__SENDER_ADMIN_ADDRESS"));

        bytes memory initCall = abi.encodeCall(
            IpcTokenSenderV2.reinitialize,
            (axelarIts, destinationChain, handlerAddr, senderAdmin)
        );

        vm.startBroadcast(privateKey);
        IpcTokenSender sender = IpcTokenSender(address(senderAddr));
        sender.upgradeToAndCall(newSenderAddrImplementation, initCall);
        vm.stopBroadcast();
        console.log("token sender proxy upgraded on %s: %s", network, address(sender));
    }

    function deployTokenHandlerV2Implementation() public {
        string memory network = vm.envString("DEST_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(network, "__PRIVATE_KEY"));
        string memory path = getPath();
        string memory readJson = vm.readFile(path);

        console.log("deploying token handler v2 implementation to %s...", network);

        vm.startBroadcast(privateKey);
        IpcTokenHandlerV2 v2Implementation = new IpcTokenHandlerV2();
        vm.stopBroadcast();

        console.log("token handler v2 implementation deployed on %s: %s", network, address(v2Implementation));
        string memory key = "out";
        vm.serializeString(key, "network", network);

        string memory json = vm.serializeAddress(key, "token_handler_implementation_v2", address(v2Implementation));
        json = vm.serializeAddress(key, "token_handler_proxy", vm.parseJsonAddress(readJson, ".dest.token_handler_proxy"));
        json = vm.serializeAddress(key, "token_handler_implementation", vm.parseJsonAddress(readJson, ".dest.token_handler_implementation"));
        vm.writeJson(json, path, ".dest");
    }

    function upgradeTokenHandlerProxy() public {
        string memory network = vm.envString("DEST_NETWORK");
        uint256 privateKey = vm.envUint(string.concat(network, "__PRIVATE_KEY"));

        console.log("upgrading token handler on %s...", network);
        checkPathExists();
        string memory path = getPath();

        console.log("loading handler v2 implementation address...");
        string memory readJson = vm.readFile(path);
        address newHandlerAddrImplementation = vm.parseJsonAddress(readJson, ".dest.token_handler_implementation_v2");
        console.log("handler implementation address: %s", newHandlerAddrImplementation);

        console.log("loading handler proxy address...");
        address handlerAddr = vm.parseJsonAddress(readJson, ".dest.token_handler_proxy");
        console.log("handler proxy address: %s", handlerAddr);

        address axelarIts = vm.envAddress(string.concat(network, "__AXELAR_ITS_ADDRESS"));
        address ipcGateway = vm.envAddress(string.concat(network, "__IPC_GATEWAY_ADDRESS"));
        address handlerAdmin = vm.envAddress(string.concat(network, "__HANDLER_ADMIN_ADDRESS"));

        bytes memory initCall = abi.encodeCall(IpcTokenHandlerV2.reinitialize, (axelarIts, ipcGateway, handlerAdmin));

        vm.startBroadcast(privateKey);
        IpcTokenHandler handler = IpcTokenHandler(address(handlerAddr));
        handler.upgradeToAndCall(newHandlerAddrImplementation, initCall);
        vm.stopBroadcast();
        console.log("token handler proxy upgraded on %s: %s", network, address(handler));
    }

    function getPath() public returns (string memory path) {
        path = string.concat(vm.projectRoot(), "/out/addresses.json");
        if (!vm.exists(path)) {
            vm.writeJson(
                '{"dest":{"token_handler_proxy":{}, "token_handler_implementation":{} },"src":{"token_sender_proxy":{}, "token_sender_implementation":{}}}',
                path
            );
        }
    }

    function checkPathExists() public {
        string memory path = string.concat(vm.projectRoot(), "/out/addresses.json");
        require(vm.exists(path), "no addresses.json; please run DeployTokenHandler on the destination chain");
    }
}

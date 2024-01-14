// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.19;

library SelectorLibrary {
    function resolveSelectors(string memory facetName) public pure returns (bytes4[] memory facetSelectors) {
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("GatewayDiamond"))) {
            return
                abi.decode(
                    hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetActorDiamond"))) {
            return
                abi.decode(
                    hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetRegistryDiamond"))) {
            return
                abi.decode(
                    hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("DiamondCutFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000011f931c1c00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("DiamondLoupeFacet"))) {
            return
                abi.decode(
                    hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005cdffacc60000000000000000000000000000000000000000000000000000000052ef6b2c00000000000000000000000000000000000000000000000000000000adfca15e000000000000000000000000000000000000000000000000000000007a0ed6270000000000000000000000000000000000000000000000000000000001ffc9a700000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("GatewayGetterFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000258789f83b0000000000000000000000000000000000000000000000000000000006c46853000000000000000000000000000000000000000000000000000000002da5794a00000000000000000000000000000000000000000000000000000000dd81b5cf0000000000000000000000000000000000000000000000000000000069e737fd0000000000000000000000000000000000000000000000000000000041b6a2e80000000000000000000000000000000000000000000000000000000024729425000000000000000000000000000000000000000000000000000000009e530b57000000000000000000000000000000000000000000000000000000006547cd6400000000000000000000000000000000000000000000000000000000b9ee584200000000000000000000000000000000000000000000000000000000a9294bdd000000000000000000000000000000000000000000000000000000002218059400000000000000000000000000000000000000000000000000000000b3ab3f7400000000000000000000000000000000000000000000000000000000ac12d763000000000000000000000000000000000000000000000000000000004aa8f8a500000000000000000000000000000000000000000000000000000000ca41d5ce00000000000000000000000000000000000000000000000000000000d6c5c39700000000000000000000000000000000000000000000000000000000544dddff000000000000000000000000000000000000000000000000000000006ad21bb000000000000000000000000000000000000000000000000000000000a517218f000000000000000000000000000000000000000000000000000000009704276600000000000000000000000000000000000000000000000000000000767ee5f400000000000000000000000000000000000000000000000000000000335eb62a00000000000000000000000000000000000000000000000000000000b1ba49b000000000000000000000000000000000000000000000000000000000f3229131000000000000000000000000000000000000000000000000000000000338150f0000000000000000000000000000000000000000000000000000000094074b03000000000000000000000000000000000000000000000000000000007edeac920000000000000000000000000000000000000000000000000000000006572c1a00000000000000000000000000000000000000000000000000000000c66c66a1000000000000000000000000000000000000000000000000000000009d3070b5000000000000000000000000000000000000000000000000000000005d02968500000000000000000000000000000000000000000000000000000000599c7bd10000000000000000000000000000000000000000000000000000000005aff0b3000000000000000000000000000000000000000000000000000000008cfd78e70000000000000000000000000000000000000000000000000000000002e30f9a00000000000000000000000000000000000000000000000000000000a2b6715800000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("GatewayManagerFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000085a627dbc0000000000000000000000000000000000000000000000000000000018f44b70000000000000000000000000000000000000000000000000000000000517e1aa0000000000000000000000000000000000000000000000000000000041c0e1b500000000000000000000000000000000000000000000000000000000f207564e000000000000000000000000000000000000000000000000000000006b2c1eef00000000000000000000000000000000000000000000000000000000d8e255720000000000000000000000000000000000000000000000000000000045f5448500000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("GatewayMessengerFacet"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000225bf0db600000000000000000000000000000000000000000000000000000000210b944e00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("CheckpointingFacet"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000453b4e7bf0000000000000000000000000000000000000000000000000000000047dc9b4f000000000000000000000000000000000000000000000000000000007430377100000000000000000000000000000000000000000000000000000000ac81837900000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("BottomUpRouterFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000040db0f77c0000000000000000000000000000000000000000000000000000000032e7661f000000000000000000000000000000000000000000000000000000000bed761500000000000000000000000000000000000000000000000000000000bacc656d00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("TopDownFinalityFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000030df14461000000000000000000000000000000000000000000000000000000001119697400000000000000000000000000000000000000000000000000000000e49a547d00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("XnetMessagingFacet"))) {
            return
                abi.decode(
                    hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001c62eb4d500000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetActorGetterFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000253354c3e10000000000000000000000000000000000000000000000000000000035142c8c0000000000000000000000000000000000000000000000000000000006c46853000000000000000000000000000000000000000000000000000000004b27aa72000000000000000000000000000000000000000000000000000000004b0694e20000000000000000000000000000000000000000000000000000000069e737fd000000000000000000000000000000000000000000000000000000008ef3f761000000000000000000000000000000000000000000000000000000003da3324100000000000000000000000000000000000000000000000000000000903e693000000000000000000000000000000000000000000000000000000000948628a900000000000000000000000000000000000000000000000000000000d92e8f1200000000000000000000000000000000000000000000000000000000c7cda762000000000000000000000000000000000000000000000000000000009754b29e0000000000000000000000000000000000000000000000000000000038a210b30000000000000000000000000000000000000000000000000000000080f76021000000000000000000000000000000000000000000000000000000005dd9147c00000000000000000000000000000000000000000000000000000000b2bd295e00000000000000000000000000000000000000000000000000000000d6eb591000000000000000000000000000000000000000000000000000000000332a5ac9000000000000000000000000000000000000000000000000000000001597bf7e0000000000000000000000000000000000000000000000000000000052d182d1000000000000000000000000000000000000000000000000000000001904bb2e000000000000000000000000000000000000000000000000000000002bc31eb300000000000000000000000000000000000000000000000000000000f75499dc00000000000000000000000000000000000000000000000000000000cfca28240000000000000000000000000000000000000000000000000000000040550a1c00000000000000000000000000000000000000000000000000000000d081be03000000000000000000000000000000000000000000000000000000001f3a0e410000000000000000000000000000000000000000000000000000000072d0a0e00000000000000000000000000000000000000000000000000000000028d5551d00000000000000000000000000000000000000000000000000000000599c7bd1000000000000000000000000000000000000000000000000000000009e33bd02000000000000000000000000000000000000000000000000000000006704287c00000000000000000000000000000000000000000000000000000000c5ab224100000000000000000000000000000000000000000000000000000000f0cf6c9600000000000000000000000000000000000000000000000000000000ad81e4d60000000000000000000000000000000000000000000000000000000080875df700000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetActorManagerFacet"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000910fd4261000000000000000000000000000000000000000000000000000000006170b1620000000000000000000000000000000000000000000000000000000041c0e1b500000000000000000000000000000000000000000000000000000000d66d9e19000000000000000000000000000000000000000000000000000000000b7fbe600000000000000000000000000000000000000000000000000000000066783c9b00000000000000000000000000000000000000000000000000000000da5d09ee000000000000000000000000000000000000000000000000000000003a4b66f1000000000000000000000000000000000000000000000000000000002e17de7800000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetActorPauseFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000038456cb59000000000000000000000000000000000000000000000000000000005c975abb000000000000000000000000000000000000000000000000000000003f4ba83a00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetActorRewardFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000034e71d92d00000000000000000000000000000000000000000000000000000000ed7c4da1000000000000000000000000000000000000000000000000000000004c860af600000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetActorCheckpointingFacet"))) {
            return
                abi.decode(
                    hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000032681193600000000000000000000000000000000000000000000000000000000b9ee2bb900000000000000000000000000000000000000000000000000000000cc2dc2b900000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("RegisterSubnetFacet"))) {
            return
                abi.decode(
                    hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001aa4edbd700000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetGetterFacet"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000f42bf3cc10000000000000000000000000000000000000000000000000000000062c9d7fb00000000000000000000000000000000000000000000000000000000967ba537000000000000000000000000000000000000000000000000000000000be06111000000000000000000000000000000000000000000000000000000001b0766c300000000000000000000000000000000000000000000000000000000a372bf30000000000000000000000000000000000000000000000000000000000f5849d1000000000000000000000000000000000000000000000000000000004d7115140000000000000000000000000000000000000000000000000000000089bba29900000000000000000000000000000000000000000000000000000000540b5ad60000000000000000000000000000000000000000000000000000000054a4eddb000000000000000000000000000000000000000000000000000000009836b75f00000000000000000000000000000000000000000000000000000000030f6051000000000000000000000000000000000000000000000000000000001163dca500000000000000000000000000000000000000000000000000000000a46d044d00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("ERC20PresetFixedSupply"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000ddd62ed3e00000000000000000000000000000000000000000000000000000000095ea7b30000000000000000000000000000000000000000000000000000000070a082310000000000000000000000000000000000000000000000000000000042966c680000000000000000000000000000000000000000000000000000000079cc679000000000000000000000000000000000000000000000000000000000313ce56700000000000000000000000000000000000000000000000000000000a457c2d700000000000000000000000000000000000000000000000000000000395093510000000000000000000000000000000000000000000000000000000006fdde030000000000000000000000000000000000000000000000000000000095d89b410000000000000000000000000000000000000000000000000000000018160ddd00000000000000000000000000000000000000000000000000000000a9059cbb0000000000000000000000000000000000000000000000000000000023b872dd00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("NumberContractFacetEight"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000167e0badb00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("NumberContractFacetSeven"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000167e0badb00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SelectorLibrary"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000166e2898c00000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("TestUtils"))) {
            return
                abi.decode(
                    hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000008997da8d4000000000000000000000000000000000000000000000000000000005727dc5c0000000000000000000000000000000000000000000000000000000003a507be000000000000000000000000000000000000000000000000000000007a308a4c00000000000000000000000000000000000000000000000000000000eeeac01e00000000000000000000000000000000000000000000000000000000bc9e2bcf00000000000000000000000000000000000000000000000000000000f6caf0ac00000000000000000000000000000000000000000000000000000000573081a200000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        if (keccak256(abi.encodePacked(facetName)) == keccak256(abi.encodePacked("SubnetActorMock"))) {
            return
                abi.decode(
                    hex"0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000001410fd4261000000000000000000000000000000000000000000000000000000004e71d92d00000000000000000000000000000000000000000000000000000000ed7c4da100000000000000000000000000000000000000000000000000000000350a14bf00000000000000000000000000000000000000000000000000000000c7ebdaef000000000000000000000000000000000000000000000000000000004c860af6000000000000000000000000000000000000000000000000000000006170b1620000000000000000000000000000000000000000000000000000000041c0e1b500000000000000000000000000000000000000000000000000000000d66d9e19000000000000000000000000000000000000000000000000000000008456cb59000000000000000000000000000000000000000000000000000000005c975abb000000000000000000000000000000000000000000000000000000000b7fbe600000000000000000000000000000000000000000000000000000000066783c9b00000000000000000000000000000000000000000000000000000000da5d09ee000000000000000000000000000000000000000000000000000000003a4b66f1000000000000000000000000000000000000000000000000000000002681193600000000000000000000000000000000000000000000000000000000b9ee2bb9000000000000000000000000000000000000000000000000000000003f4ba83a000000000000000000000000000000000000000000000000000000002e17de7800000000000000000000000000000000000000000000000000000000cc2dc2b900000000000000000000000000000000000000000000000000000000",
                    (bytes4[])
                );
        }
        revert("Selector not found");
    }
}

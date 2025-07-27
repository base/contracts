// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Script} from "lib/forge-std/src/Script.sol";
import {ERC20PresetMinterPauser} from "@openzeppelin/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol";
import {ERC721PresetMinterPauserAutoId} from
    "@openzeppelin/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol";

// Deploys test token contracts on L1 to test Base Mainnet's bridging functionality
contract DeployTestTokenContracts is Script {
    function run(address tester) public {
        vm.startBroadcast(tester);
        ERC20PresetMinterPauser erc20 = new ERC20PresetMinterPauser({name: "L1 TEST ERC20", symbol: "L1T20"});
        console.log("TEST ERC20 deployed to: %s", address(erc20));

        ERC721PresetMinterPauserAutoId erc721 = new ERC721PresetMinterPauserAutoId({
            name: "L1 TEST ERC721",
            symbol: "L1T721",
            baseTokenURI: "not applicable"
        });
        console.log("TEST ERC721 deployed to: %s", address(erc721));

        erc20.mint({to: tester, amount: 1_000_000 ether});
        erc721.mint({to: tester});
        console.log("Minting to tester complete");

        vm.stopBroadcast();
    }
}

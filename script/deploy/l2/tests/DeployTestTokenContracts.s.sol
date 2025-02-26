// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// solhint-disable no-console
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";

import {Predeploys} from "@eth-optimism-bedrock/src/libraries/Predeploys.sol";
import {OptimismMintableERC20Factory} from "@eth-optimism-bedrock/src/universal/OptimismMintableERC20Factory.sol";
import {OptimismMintableERC721Factory} from "@eth-optimism-bedrock/src/universal/OptimismMintableERC721Factory.sol";

// Deploys test token contracts on L2 to test Base Mainnet functionality
contract DeployTestTokenContracts is Script {
    function run(address _tester, address _l1ERC20, address _l1ERC721) public {
        vm.startBroadcast(_tester);
        address ERC20 = OptimismMintableERC20Factory(Predeploys.OPTIMISM_MINTABLE_ERC20_FACTORY)
            .createOptimismMintableERC20(_l1ERC20, "L2 TEST ERC20", "L2T20");
        console.log("Bridged ERC20 deployed to: %s", address(ERC20));

        address ERC721 = OptimismMintableERC721Factory(payable(Predeploys.OPTIMISM_MINTABLE_ERC721_FACTORY))
            .createOptimismMintableERC721(_l1ERC721, "L2 TEST ERC721", "L1T721");
        console.log("Bridged ERC721 deployed to: %s", address(ERC721));

        vm.stopBroadcast();
    }
}

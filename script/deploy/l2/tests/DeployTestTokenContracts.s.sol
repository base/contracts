// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Script} from "lib/forge-std/src/Script.sol";
import {Predeploys} from "@eth-optimism-bedrock/src/libraries/Predeploys.sol";
import {OptimismMintableERC20Factory} from "@eth-optimism-bedrock/src/universal/OptimismMintableERC20Factory.sol";
import {OptimismMintableERC721Factory} from "@eth-optimism-bedrock/src/universal/OptimismMintableERC721Factory.sol";

// Deploys test token contracts on L2 to test Base Mainnet functionality
contract DeployTestTokenContracts is Script {
    function run(address tester, address l1erc20, address l1erc721) public {
        vm.startBroadcast(tester);
        address erc20 = OptimismMintableERC20Factory(Predeploys.OPTIMISM_MINTABLE_ERC20_FACTORY)
            .createOptimismMintableERC20({_remoteToken: l1erc20, _name: "L2 TEST ERC20", _symbol: "L2T20"});
        console.log("Bridged erc20 deployed to: %s", address(erc20));

        address erc721 = OptimismMintableERC721Factory(payable(Predeploys.OPTIMISM_MINTABLE_ERC721_FACTORY))
            .createOptimismMintableERC721({_remoteToken: l1erc721, _name: "L2 TEST ERC721", _symbol: "L2T721"});
        console.log("Bridged erc721 deployed to: %s", address(erc721));

        vm.stopBroadcast();
    }
}

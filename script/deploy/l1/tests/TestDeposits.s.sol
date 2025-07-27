// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Script} from "lib/forge-std/src/Script.sol";
import {ERC20PresetMinterPauser} from "@openzeppelin/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol";
import {ERC721PresetMinterPauserAutoId} from
    "@openzeppelin/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol";
import {L1ERC721Bridge} from "lib/optimism/packages/contracts-bedrock/src/L1/L1ERC721Bridge.sol";
import {L1StandardBridge} from "lib/optimism/packages/contracts-bedrock/src/L1/L1StandardBridge.sol";

// Deposits funds to Base Mainnet to test its functionality
contract DeployTestContracts is Script {
    function run(
        address tester,
        address payable l1StandardBridge,
        address l1erc721Bridge,
        address payable l1erc20,
        address l1erc721,
        address l2erc20,
        address l2erc721
    ) public {
        vm.startBroadcast(tester);
        ERC20PresetMinterPauser(l1erc20).approve({spender: l1StandardBridge, amount: 1_000_000 ether});
        ERC721PresetMinterPauserAutoId(l1erc721).approve({to: l1erc721Bridge, tokenId: 0});

        console.log("Approvals to bridge contracts complete");

        L1StandardBridge(l1StandardBridge).depositERC20({
            _l1Token: l1erc20,
            _l2Token: l2erc20,
            _amount: 1_000_000 ether,
            _minGasLimit: 200_000,
            _extraData: bytes("")
        });

        console.log("L1StandardBridge erc20 deposit complete");

        L1ERC721Bridge(l1erc721Bridge).bridgeERC721({
            _localToken: l1erc721,
            _remoteToken: l2erc721,
            _tokenId: 0,
            _minGasLimit: 200_000,
            _extraData: bytes("")
        });

        console.log("L1ERC721Bridge erc721 deposit complete");
    }
}

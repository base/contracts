// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Script} from "lib/forge-std/src/Script.sol";
import {Predeploys} from "lib/optimism/packages/contracts-bedrock/src/libraries/Predeploys.sol";
import {L2ERC721Bridge} from "lib/optimism/packages/contracts-bedrock/src/L2/L2ERC721Bridge.sol";
import {L2StandardBridge} from "lib/optimism/packages/contracts-bedrock/src/L2/L2StandardBridge.sol";

// Withdraws tokens from L2 to L1 to test Base Mainnet's bridging functionality
contract TestWithdraw is Script {
    function run(address tester, address l2erc20, address l1erc721, address l2erc721) public {
        vm.startBroadcast(tester);
        L2StandardBridge(payable(Predeploys.L2_STANDARD_BRIDGE)).withdraw({
            _l2Token: l2erc20,
            _amount: 10_000 ether,
            _minGasLimit: 200_000,
            _extraData: bytes("")
        });
        console.log("erc20 withdrawal initiated");

        L2ERC721Bridge(payable(Predeploys.L2_ERC721_BRIDGE)).bridgeERC721({
            _localToken: l2erc721,
            _remoteToken: l1erc721,
            _tokenId: 0,
            _minGasLimit: 200_000,
            _extraData: bytes("")
        });
        console.log("erc721 withdrawal initiated");

        vm.stopBroadcast();
    }
}

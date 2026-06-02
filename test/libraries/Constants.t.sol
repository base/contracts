// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";
import { Constants } from "src/libraries/Constants.sol";
import { IResourceMetering } from "interfaces/L1/IResourceMetering.sol";

contract Constants_Test is Test {
    function test_estimationAddress_succeeds() external pure {
        assertEq(Constants.ESTIMATION_ADDRESS, address(1));
    }

    function test_defaultL2Sender_succeeds() external pure {
        assertEq(Constants.DEFAULT_L2_SENDER, 0x000000000000000000000000000000000000dEaD);
    }

    function test_proxyImplementationAddress_succeeds() external pure {
        assertEq(
            bytes32(uint256(keccak256("eip1967.proxy.implementation")) - 1), Constants.PROXY_IMPLEMENTATION_ADDRESS
        );
    }

    function test_proxyOwnerAddress_succeeds() external pure {
        assertEq(bytes32(uint256(keccak256("eip1967.proxy.admin")) - 1), Constants.PROXY_OWNER_ADDRESS);
    }

    function test_guardStorageSlot_succeeds() external pure {
        assertEq(keccak256("guard_manager.guard.address"), Constants.GUARD_STORAGE_SLOT);
    }

    function test_ether_succeeds() external pure {
        assertEq(Constants.ETHER, 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE);
    }

    function test_depositorAccount_succeeds() external pure {
        assertEq(Constants.DEPOSITOR_ACCOUNT, 0xDeaDDEaDDeAdDeAdDEAdDEaddeAddEAdDEAd0001);
    }

    function test_defaultResourceConfig_succeeds() external pure {
        IResourceMetering.ResourceConfig memory config = Constants.DEFAULT_RESOURCE_CONFIG();
        assertEq(config.maxResourceLimit, 20_000_000);
        assertEq(config.elasticityMultiplier, 10);
        assertEq(config.baseFeeMaxChangeDenominator, 8);
        assertEq(config.minimumBaseFee, 1 gwei);
        assertEq(config.systemTxMaxGas, 1_000_000);
        assertEq(config.maximumBaseFee, type(uint128).max);
    }
}

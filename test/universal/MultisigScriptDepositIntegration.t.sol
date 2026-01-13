// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {CBMulticall} from "src/utils/CBMulticall.sol";
import {Test, console} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {MultisigScriptDeposit} from "script/universal/MultisigScriptDeposit.sol";
import {Simulation} from "script/universal/Simulation.sol";

/// @notice Integration test for L2 gas estimation
/// @dev Run with: L2_RPC_URL=<base-sepolia-rpc> forge test --match-contract MultisigScriptDepositIntegrationTest -vvv
contract MultisigScriptDepositIntegrationTest is Test, MultisigScriptDeposit {
    function _ownerSafe() internal pure override returns (address) {
        return address(1); // Dummy address for testing
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal pure override {}

    /// @notice Build simple L2 calls for testing estimation
    function _buildL2Calls() internal pure override returns (CBMulticall.Call3Value[] memory) {
        CBMulticall.Call3Value[] memory calls = new CBMulticall.Call3Value[](2);

        // Call 1: Simple ETH transfer (minimal gas)
        calls[0] = CBMulticall.Call3Value({target: address(0xdead), allowFailure: false, callData: "", value: 0});

        // Call 2: Another simple call
        calls[1] = CBMulticall.Call3Value({
            target: address(0xbeef),
            allowFailure: false,
            callData: abi.encodeWithSignature("nonExistentFunction()"),
            value: 0
        });

        return calls;
    }

    /// @notice Integration test that actually forks L2 and estimates gas
    /// @dev Requires L2_RPC_URL environment variable to be set
    ///      Run with: L2_RPC_URL=https://sepolia.base.org forge test --match-test test_integration_gasEstimation -vvv
    function test_integration_gasEstimation() external {
        // Skip if L2_RPC_URL is not set (allows CI to pass without network access)
        try vm.envString("L2_RPC_URL") returns (
            string memory
        ) {
        // L2_RPC_URL is set, continue with test
        }
        catch {
            vm.skip(true, "L2_RPC_URL not set - skipping integration test. Set L2_RPC_URL to run.");
        }

        console.log("Starting L2 gas estimation integration test...");
        console.log("L2_RPC_URL is set, proceeding with fork-based estimation");

        // Trigger estimation (will fork L2 and measure gas)
        _ensureL2GasLimitCached();

        // Get the estimated gas
        uint64 estimatedGas = _l2GasLimit();

        console.log("Estimated L2 gas limit:", estimatedGas);

        // Basic sanity checks
        assertGt(estimatedGas, 0, "Estimated gas should be > 0");
        assertLt(estimatedGas, 10_000_000, "Estimated gas should be reasonable (< 10M)");

        console.log("Integration test passed!");
    }
}


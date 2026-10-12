// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CommonTest } from "test/setup/CommonTest.sol";

/// @title ExtendedPause_Test
/// @notice These tests are somewhat redundant with tests in the SuperchainConfig and other
///         pausable contracts, however it is worthwhile to pull them into one location to ensure
///         that the behavior is consistent.
contract ExtendedPause_Test is CommonTest {
    /// @notice Tests that other contracts are paused when the superchain config is paused
    function test_pause_fullSystem_succeeds() external {
        _assertFullSystemPaused(false);

        _pauseSuperchain();

        _assertFullSystemPaused(true);
    }

    /// @notice Tests that other contracts are unpaused when the superchain config is paused and
    ///         then unpaused.
    function test_unpause_fullSystem_succeeds() external {
        _pauseSuperchain();

        vm.prank(superchainConfig.guardian());
        superchainConfig.unpause(address(0));

        _assertFullSystemPaused(false);
    }

    function _pauseSuperchain() internal {
        vm.prank(superchainConfig.guardian());
        superchainConfig.pause(address(0));
    }

    function _assertFullSystemPaused(bool _paused) internal view {
        assertEq(superchainConfig.paused(address(0)), _paused);
        assertEq(optimismPortal2.paused(), _paused);
        assertEq(l1CrossDomainMessenger.paused(), _paused);
        assertEq(l1StandardBridge.paused(), _paused);
        assertEq(l1ERC721Bridge.paused(), _paused);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";

// Libraries
import { Constants } from "src/libraries/Constants.sol";

// Interfaces
import { IBaseTime } from "interfaces/L2/IBaseTime.sol";

/// @title BaseTime_TestInit
/// @notice Reusable test initialization for BaseTime tests.
abstract contract BaseTime_TestInit is CommonTest {
    /// @notice Largest valid millisecond component.
    uint16 internal constant MAX_TIMESTAMP_MILLIS_PART = 800;

    /// @notice Interval the millisecond component must be aligned to.
    uint16 internal constant TIMESTAMP_MILLIS_PART_INTERVAL = 200;

    /// @notice Sets the millisecond component as the protocol depositor.
    function setTimestampMillisPart(uint16 _timestampMillisPart) internal {
        vm.prank(Constants.DEPOSITOR_ACCOUNT);
        baseTime.setTimestampMillisPart(_timestampMillisPart);
    }

    /// @notice Maps a fuzzed slot to a valid millisecond component.
    function validTimestampMillisPart(uint256 _slot) internal pure returns (uint16) {
        return uint16(
            bound(_slot, 0, MAX_TIMESTAMP_MILLIS_PART / TIMESTAMP_MILLIS_PART_INTERVAL) * TIMESTAMP_MILLIS_PART_INTERVAL
        );
    }

    /// @notice Returns whether a millisecond component is invalid.
    function isInvalidTimestampMillisPart(uint16 _timestampMillisPart) internal pure returns (bool) {
        return
            _timestampMillisPart > MAX_TIMESTAMP_MILLIS_PART
                || _timestampMillisPart % TIMESTAMP_MILLIS_PART_INTERVAL != 0;
    }
}

/// @title BaseTime_SetTimestampMillisPart_Test
/// @notice Tests updating BaseTime's millisecond component.
contract BaseTime_SetTimestampMillisPart_Test is BaseTime_TestInit {
    /// @notice Tests that callers other than the protocol depositor are rejected.
    function testFuzz_setTimestampMillisPart_notDepositor_reverts(address _caller, uint256 _slot) external {
        vm.assume(_caller != Constants.DEPOSITOR_ACCOUNT);
        uint16 part = validTimestampMillisPart(_slot);

        vm.expectRevert(IBaseTime.BaseTime_NotDepositor.selector);
        vm.prank(_caller);
        baseTime.setTimestampMillisPart(part);
    }

    /// @notice Tests that the depositor check runs before the value check.
    function testFuzz_setTimestampMillisPart_notDepositorInvalidValue_reverts(
        address _caller,
        uint16 _timestampMillisPart
    )
        external
    {
        vm.assume(_caller != Constants.DEPOSITOR_ACCOUNT);
        vm.assume(isInvalidTimestampMillisPart(_timestampMillisPart));

        vm.expectRevert(IBaseTime.BaseTime_NotDepositor.selector);
        vm.prank(_caller);
        baseTime.setTimestampMillisPart(_timestampMillisPart);
    }

    /// @notice Tests that an invalid millisecond component is rejected.
    function testFuzz_setTimestampMillisPart_invalidValue_reverts(uint16 _timestampMillisPart) external {
        vm.assume(isInvalidTimestampMillisPart(_timestampMillisPart));

        vm.expectRevert(IBaseTime.BaseTime_InvalidTimestampMillisPart.selector);
        vm.prank(Constants.DEPOSITOR_ACCOUNT);
        baseTime.setTimestampMillisPart(_timestampMillisPart);
    }

    /// @notice Tests that the depositor can set any valid millisecond component.
    function testFuzz_setTimestampMillisPart_validValue_succeeds(uint256 _slot) external {
        uint16 part = validTimestampMillisPart(_slot);

        setTimestampMillisPart(part);

        assertEq(baseTime.timestampMillisPart(), part);
    }

    /// @notice Tests that the millisecond component occupies slot zero as a uint16.
    function test_setTimestampMillisPart_usesSlotZero_succeeds() external {
        uint16 part = 600;

        setTimestampMillisPart(part);

        assertEq(vm.load(address(baseTime), bytes32(0)), bytes32(uint256(part)));
    }
}

/// @title BaseTime_TimestampMs_Test
/// @notice Tests BaseTime's full millisecond timestamp getter.
contract BaseTime_TimestampMs_Test is BaseTime_TestInit {
    /// @notice Tests that timestampMs combines block.timestamp with the millisecond component.
    function testFuzz_timestampMs_succeeds(uint256 _timestamp, uint256 _slot) external {
        uint256 timestamp = bound(_timestamp, 0, type(uint64).max / 1000 - 1);
        uint16 part = validTimestampMillisPart(_slot);
        vm.warp(timestamp);
        setTimestampMillisPart(part);

        assertEq(baseTime.timestampMs(), timestamp * 1000 + part);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { Test } from "forge-std/Test.sol";

// Libraries
import { Constants } from "src/libraries/Constants.sol";

// Target contract
import { BaseTime } from "src/L2/BaseTime.sol";
import { IBaseTime } from "interfaces/L2/IBaseTime.sol";

/// @title BaseTime_TestInit
/// @notice Reusable test initialization for BaseTime tests.
abstract contract BaseTime_TestInit is Test {
    BaseTime internal baseTime;

    /// @notice Sets up the test suite.
    function setUp() public {
        baseTime = new BaseTime();
    }

    /// @notice Sets the millisecond component as the protocol depositor.
    function setTimestampMillisPart(uint16 _timestampMillisPart) internal {
        vm.prank(Constants.DEPOSITOR_ACCOUNT);
        baseTime.setTimestampMillisPart(_timestampMillisPart);
    }
}

/// @title BaseTime_InitialValue_Test
/// @notice Tests BaseTime's initial value.
contract BaseTime_InitialValue_Test is BaseTime_TestInit {
    /// @notice Tests that the millisecond component initially equals zero.
    function test_initialValue_succeeds() external view {
        assertEq(baseTime.timestampMillisPart(), 0);
    }
}

/// @title BaseTime_SetTimestampMillisPart_Test
/// @notice Tests updating BaseTime's millisecond component.
contract BaseTime_SetTimestampMillisPart_Test is BaseTime_TestInit {
    /// @notice Tests every valid millisecond component.
    function test_setTimestampMillisPart_succeeds() external {
        uint16[5] memory validValues = [uint16(0), 200, 400, 600, 800];

        for (uint256 i; i < validValues.length; i++) {
            setTimestampMillisPart(validValues[i]);
            assertEq(baseTime.timestampMillisPart(), validValues[i]);
        }
    }

    /// @notice Tests that an invalid millisecond component is rejected.
    function testFuzz_setTimestampMillisPart_invalidValue_reverts(uint16 _timestampMillisPart) external {
        vm.assume(
            _timestampMillisPart != 0 && _timestampMillisPart != 200 && _timestampMillisPart != 400
                && _timestampMillisPart != 600 && _timestampMillisPart != 800
        );
        vm.expectRevert(IBaseTime.BaseTime_InvalidTimestampMillisPart.selector);
        vm.prank(Constants.DEPOSITOR_ACCOUNT);
        baseTime.setTimestampMillisPart(_timestampMillisPart);
    }

    /// @notice Tests that callers other than the protocol depositor are rejected.
    function testFuzz_setTimestampMillisPart_notDepositor_reverts(address _caller) external {
        vm.assume(_caller != Constants.DEPOSITOR_ACCOUNT);
        vm.expectRevert(IBaseTime.BaseTime_NotDepositor.selector);
        vm.prank(_caller);
        baseTime.setTimestampMillisPart(200);
    }

    /// @notice Tests that the millisecond component occupies slot zero as a uint16.
    function test_setTimestampMillisPart_usesSlotZero_succeeds() external {
        setTimestampMillisPart(600);

        assertEq(vm.load(address(baseTime), bytes32(0)), bytes32(uint256(600)));
    }
}

/// @title BaseTime_TimestampMs_Test
/// @notice Tests BaseTime's full millisecond timestamp getter.
contract BaseTime_TimestampMs_Test is BaseTime_TestInit {
    /// @notice Tests that timestampMs combines block.timestamp with the millisecond component.
    function test_timestampMs_succeeds() external {
        vm.warp(1725);
        setTimestampMillisPart(600);

        assertEq(baseTime.timestampMs(), 1_725_600);
    }
}

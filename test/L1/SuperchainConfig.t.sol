// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";

// Interfaces
import { ISuperchainConfig } from "interfaces/L1/ISuperchainConfig.sol";

/// @title SuperchainConfig_TestInit
/// @notice Initialization contract for SuperchainConfig tests.
abstract contract SuperchainConfig_TestInit is CommonTest {
    uint256 internal constant PAUSE_EXPIRY = 7_884_000;

    function setUp() public virtual override {
        super.setUp();
        skipIfForkTest("SuperchainConfig_TestInit: cannot test initialization on forked network");
    }

    function _pauseAsGuardian(address _identifier) internal {
        vm.prank(superchainConfig.guardian());
        superchainConfig.pause(_identifier);
    }
}

/// @title SuperchainConfig_Constructor_Test
/// @notice Test contract for SuperchainConfig constructor values.
contract SuperchainConfig_Constructor_Test is SuperchainConfig_TestInit {
    /// @notice Tests that constructor sets the correct values.
    function test_constructor_unpaused_succeeds() external view {
        assertFalse(superchainConfig.paused(address(this)));
        assertEq(superchainConfig.guardian(), deploy.cfg().superchainConfigGuardian());
    }
}

/// @title SuperchainConfig_PauseExpiry_Test
/// @notice Test contract for SuperchainConfig `pauseExpiry` function.
contract SuperchainConfig_PauseExpiry_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `pauseExpiry` returns the correct constant value.
    function test_pauseExpiry_succeeds() external view {
        assertEq(superchainConfig.pauseExpiry(), PAUSE_EXPIRY);
    }
}

/// @title SuperchainConfig_Paused_Test
/// @notice Test contract for SuperchainConfig `paused` function.
contract SuperchainConfig_Paused_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `paused` returns true when the specific identifier is paused.
    /// @param _identifier The identifier to test.
    /// @param _other The unpaused identifier to test.
    function testFuzz_paused_specificIdentifier_succeeds(address _identifier, address _other) external {
        vm.assume(_identifier != address(0));
        vm.assume(_other != _identifier);

        _pauseAsGuardian(_identifier);
        assertTrue(superchainConfig.paused(_identifier));
        assertFalse(superchainConfig.paused(_other));
    }

    /// @notice Tests that `paused` returns true when the global superchain system is paused.
    function test_paused_global_succeeds() external {
        _pauseAsGuardian(address(0));

        assertTrue(superchainConfig.paused());
        assertTrue(superchainConfig.paused(address(0)));
        assertFalse(superchainConfig.paused(address(1)));
    }

    /// @notice Tests that `paused` returns false after pause expires.
    /// @param _identifier The identifier to test.
    function testFuzz_paused_expired_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);
        assertTrue(superchainConfig.paused(_identifier));

        vm.warp(block.timestamp + PAUSE_EXPIRY + 1);
        assertFalse(superchainConfig.paused(_identifier));
    }

    /// @notice Tests that `paused` returns true just before expiry.
    /// @param _identifier The identifier to test.
    function testFuzz_paused_beforeExpiry_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);

        vm.warp(block.timestamp + PAUSE_EXPIRY - 1);
        assertTrue(superchainConfig.paused(_identifier));
    }
}

/// @title SuperchainConfig_Pause_Test
/// @notice Test contract for SuperchainConfig `pause` function.
contract SuperchainConfig_Pause_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `pause` successfully pauses when called by the guardian.
    /// @param _identifier The identifier to test.
    function testFuzz_pause_succeeds(address _identifier) external {
        assertFalse(superchainConfig.paused(_identifier));

        vm.expectEmit(address(superchainConfig));
        emit Paused(_identifier);

        vm.prank(superchainConfig.guardian());
        superchainConfig.pause(_identifier);

        assertTrue(superchainConfig.paused(_identifier));
    }

    /// @notice Tests that `pause` reverts when called by a non-guardian and non-incident-responder.
    /// @param _caller The unauthorized caller to test.
    function testFuzz_pause_notGuardianOrIncidentResponder_reverts(address _caller) external {
        vm.assume(_caller != superchainConfig.guardian() && _caller != superchainConfig.incidentResponder());

        vm.expectRevert(ISuperchainConfig.SuperchainConfig_OnlyGuardianOrIncidentResponder.selector);
        vm.prank(_caller);
        superchainConfig.pause(address(this));
    }

    /// @notice Tests that `pause` reverts when the identifier is already used.
    /// @param _identifier The identifier to test.
    function testFuzz_pause_alreadyUsed_reverts(address _identifier) external {
        _pauseAsGuardian(_identifier);

        vm.prank(superchainConfig.guardian());
        vm.expectRevert(abi.encodeWithSelector(ISuperchainConfig.SuperchainConfig_AlreadyPaused.selector, _identifier));
        superchainConfig.pause(_identifier);
    }
}

/// @title SuperchainConfig_Unpause_Test
/// @notice Test contract for SuperchainConfig `unpause` function.
contract SuperchainConfig_Unpause_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `unpause` successfully unpauses when called by the guardian.
    /// @param _identifier The identifier to test.
    function testFuzz_unpause_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);
        assertTrue(superchainConfig.paused(_identifier));

        vm.expectEmit(address(superchainConfig));
        emit Unpaused(_identifier);
        vm.prank(superchainConfig.guardian());
        superchainConfig.unpause(_identifier);

        assertFalse(superchainConfig.paused(_identifier));
    }

    /// @notice Tests that `unpause` reverts when called by a non-guardian.
    /// @param _caller The non-guardian caller to test.
    function testFuzz_unpause_notGuardian_reverts(address _caller) external {
        vm.assume(_caller != superchainConfig.guardian());

        _pauseAsGuardian(address(this));
        assertTrue(superchainConfig.paused(address(this)));

        vm.expectRevert(ISuperchainConfig.SuperchainConfig_OnlyGuardian.selector);
        vm.prank(_caller);
        superchainConfig.unpause(address(this));
    }
}

/// @title SuperchainConfig_Extend_Test
/// @notice Test contract for SuperchainConfig `extend` function.
contract SuperchainConfig_Extend_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `extend` successfully resets and re-pauses an identifier.
    /// @param _identifier The identifier to test.
    function testFuzz_extend_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);
        uint256 firstPauseTimestamp = block.timestamp;

        vm.warp(block.timestamp + 1);

        vm.expectEmit(address(superchainConfig));
        emit PauseExtended(_identifier);
        vm.prank(superchainConfig.guardian());
        superchainConfig.extend(_identifier);
        assertTrue(superchainConfig.pauseTimestamps(_identifier) > firstPauseTimestamp);
        assertTrue(superchainConfig.paused(_identifier));
    }

    /// @notice Tests that `extend` reverts when called by a non-guardian.
    /// @param _caller The non-guardian caller to test.
    function testFuzz_extend_notGuardian_reverts(address _caller) external {
        vm.assume(_caller != superchainConfig.guardian());

        _pauseAsGuardian(address(this));

        vm.expectRevert(ISuperchainConfig.SuperchainConfig_OnlyGuardian.selector);
        vm.prank(_caller);
        superchainConfig.extend(address(this));
    }

    /// @notice Tests that `extend` reverts when the identifier is not already paused.
    /// @param _identifier The identifier to test.
    function testFuzz_extend_notAlreadyPaused_reverts(address _identifier) external {
        vm.prank(superchainConfig.guardian());
        vm.expectRevert(
            abi.encodeWithSelector(ISuperchainConfig.SuperchainConfig_NotAlreadyPaused.selector, _identifier)
        );
        superchainConfig.extend(_identifier);
    }
}

/// @title SuperchainConfig_Pausable_Test
/// @notice Test contract for SuperchainConfig `pausable` function.
contract SuperchainConfig_Pausable_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `pausable` returns true when the identifier is not paused.
    /// @param _identifier The identifier to test.
    function testFuzz_pausable_notPaused_succeeds(address _identifier) external view {
        assertTrue(superchainConfig.pausable(_identifier));
    }

    /// @notice Tests that `pausable` returns false when the identifier is paused.
    /// @param _identifier The identifier to test.
    function testFuzz_pausable_paused_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);
        assertFalse(superchainConfig.pausable(_identifier));
    }

    /// @notice Tests that `pausable` returns false even after pause expires.
    /// @param _identifier The identifier to test.
    function testFuzz_pausable_expired_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);

        vm.warp(block.timestamp + PAUSE_EXPIRY + 1);

        // Expired pauses remain unpausable because the timestamp stays set.
        assertFalse(superchainConfig.pausable(_identifier));
        assertFalse(superchainConfig.paused(_identifier));
    }
}

/// @title SuperchainConfig_Guardian_Test
/// @notice Test contract for SuperchainConfig `guardian` getter function.
contract SuperchainConfig_Guardian_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `guardian` returns the correct guardian address.
    function test_guardian_succeeds() external view {
        assertEq(superchainConfig.guardian(), deploy.cfg().superchainConfigGuardian());
    }
}

/// @title SuperchainConfig_PauseTimestamps_Test
/// @notice Test contract for SuperchainConfig `pauseTimestamps` getter function.
contract SuperchainConfig_PauseTimestamps_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `pauseTimestamps` returns 0 for unpaused identifiers.
    /// @param _identifier The identifier to test.
    function testFuzz_pauseTimestamps_unpaused_succeeds(address _identifier) external view {
        assertEq(superchainConfig.pauseTimestamps(_identifier), 0);
    }

    /// @notice Tests that `pauseTimestamps` returns the correct timestamp for paused identifiers.
    /// @param _identifier The identifier to test.
    function testFuzz_pauseTimestamps_paused_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);
        assertEq(superchainConfig.pauseTimestamps(_identifier), block.timestamp);
    }

    /// @notice Tests that `pauseTimestamps` returns 0 after unpausing.
    /// @param _identifier The identifier to test.
    function testFuzz_pauseTimestamps_afterUnpause_succeeds(address _identifier) external {
        _pauseAsGuardian(_identifier);
        assertTrue(superchainConfig.pauseTimestamps(_identifier) != 0);

        vm.prank(superchainConfig.guardian());
        superchainConfig.unpause(_identifier);
        assertEq(superchainConfig.pauseTimestamps(_identifier), 0);
    }
}

/// @title SuperchainConfig_Version_Test
/// @notice Test contract for SuperchainConfig `version` getter function.
contract SuperchainConfig_Version_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `version` returns a version string.
    function test_version_succeeds() external view {
        assert(bytes(superchainConfig.version()).length > 0);
    }
}

/// @title SuperchainConfig_Expiration_Test
/// @notice Test contract for SuperchainConfig `expiration` function.
contract SuperchainConfig_Expiration_Test is SuperchainConfig_TestInit {
    /// @notice Tests that `expiration` returns 0 when the identifier is not paused.
    function test_expiration_notPaused_succeeds() external view {
        assertEq(superchainConfig.expiration(address(this)), 0);
    }

    /// @notice Tests that `expiration` returns the correct timestamp when the identifier is
    ///         paused.
    function test_expiration_paused_succeeds() external {
        _pauseAsGuardian(address(this));
        assertEq(superchainConfig.expiration(address(this)), block.timestamp + PAUSE_EXPIRY);
    }

    /// @notice Tests that `expiration` returns the updated timestamp after extending the pause.
    function test_expiration_afterExtend_succeeds() external {
        _pauseAsGuardian(address(this));
        vm.warp(block.timestamp + 100);

        vm.prank(superchainConfig.guardian());
        superchainConfig.extend(address(this));

        assertEq(superchainConfig.expiration(address(this)), block.timestamp + PAUSE_EXPIRY);
    }

    /// @notice Tests that `expiration` works correctly with fuzzed identifiers.
    /// @param _identifier The identifier to test.
    function testFuzz_expiration_succeeds(address _identifier) external {
        assertEq(superchainConfig.expiration(_identifier), 0);

        _pauseAsGuardian(_identifier);
        assertEq(superchainConfig.expiration(_identifier), block.timestamp + PAUSE_EXPIRY);
    }
}

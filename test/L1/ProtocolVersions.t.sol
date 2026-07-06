// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { Test } from "lib/forge-std/src/Test.sol";

// Contracts
import { ProtocolVersions } from "src/L1/ProtocolVersions.sol";
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";

// Interfaces
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";

// Mocks
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

/// @title ProtocolVersions_TestInit
/// @notice Reusable test initialization for ProtocolVersions tests.
abstract contract ProtocolVersions_TestInit is Test {
    event UpgradeRegistered(bytes32 indexed key, uint256 indexed index, string upgradeId, uint256 protocolVersion);
    event ChainTeamUpdated(address indexed previousChainTeam, address indexed newChainTeam);
    event TimestampSet(bytes32 indexed key, uint256 timestamp);


    address internal _owner = makeAddr("owner");
    address internal _nonOwner = makeAddr("non-owner");
    address internal _chainTeam = makeAddr("chain-team");
    ProtocolVersions internal protocolVersions;

    function setUp() public virtual {
        protocolVersions = new ProtocolVersions(8453);
        address mockProxyAdmin = makeAddr("proxy-admin");
        vm.mockCall(mockProxyAdmin, abi.encodeWithSignature("owner()"), abi.encode(_owner));
        EIP1967Helper.setAdmin(address(protocolVersions), mockProxyAdmin);
    }

    /// @dev Registers `canyon` and schedules it for block.timestamp + MIN_NOTICE + delay.
    function _scheduleCanyon(uint64 _delay) internal returns (uint64 ts_) {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);
        ts_ = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + _delay;
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts_);
    }
}

/// @title ProtocolVersions_Constructor_Test
/// @notice Test contract for the ProtocolVersions constructor.
contract ProtocolVersions_Constructor_Test is ProtocolVersions_TestInit {
    /// @notice Tests that the constructor sets the correct initial state.
    function test_constructor_setsInitialState_succeeds() external view {
        assertEq(protocolVersions.proxyAdminOwner(), _owner);
        assertNotEq(protocolVersions.scheduleId(), bytes32(0));
    }

    /// @notice Tests that the constructor reverts when the chain ID is zero.
    function test_constructor_zeroChainId_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidL2ChainId.selector);
        new ProtocolVersions(0);
    }
}

/// @title ProtocolVersions_Version_Test
/// @notice Test contract for the `version` function.
contract ProtocolVersions_Version_Test is ProtocolVersions_TestInit {
    /// @notice Tests that the `version` function returns the expected value.
    function test_version_succeeds() external view {
        assertEq(protocolVersions.version(), "1.0.0");
    }
}

/// @title ProtocolVersions_RegisterUpgrade_Test
/// @notice Test contract for the `registerUpgrade` function.
contract ProtocolVersions_RegisterUpgrade_Test is ProtocolVersions_TestInit {
    /// @notice Tests that registering an upgrade extends the scheduleId chain.
    function test_registerUpgrade_changesScheduleId_succeeds() external {
        bytes32 idBefore = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        assertNotEq(protocolVersions.scheduleId(), idBefore);
    }

    /// @notice Tests that `registerUpgrade` emits the `UpgradeRegistered` event with correct fields.
    function test_registerUpgrade_emitsEvent_succeeds() external {
        vm.expectEmit(true, true, true, true, address(protocolVersions));
        emit UpgradeRegistered(bytes32("canyon"), 0, "canyon", 1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.expectEmit(true, true, true, true, address(protocolVersions));
        emit UpgradeRegistered(bytes32("ecotone"), 1, "ecotone", 2);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("ecotone", 2);
    }

    /// @notice Tests that registering upgrades updates latestProtocolVersion to the most recent.
    function test_registerUpgrade_updatesLatestProtocolVersion_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 5);
        assertEq(protocolVersions.latestProtocolVersion(), 5);

        vm.prank(_owner);
        protocolVersions.registerUpgrade("ecotone", 9);
        assertEq(protocolVersions.latestProtocolVersion(), 9);
    }

    /// @notice Tests that registering a 32-byte name (the maximum) succeeds.
    function test_registerUpgrade_exactly32ByteName_succeeds() external {
        string memory name32 = "exactly-32-bytes-upgrade-name!!!";
        vm.prank(_owner);
        protocolVersions.registerUpgrade(name32, 7);
        assertEq(protocolVersions.latestProtocolVersion(), 7);
    }

    /// @notice Tests that registering the same upgrade twice reverts.
    function test_registerUpgrade_duplicate_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_UpgradeAlreadyRegistered.selector, bytes32("canyon")
            )
        );
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);
    }

    /// @notice Tests that registering an upgrade with a zero protocolVersion reverts.
    function test_registerUpgrade_zeroProtocolVersion_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidProtocolVersion.selector);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 0);
    }

    /// @notice Tests that registering an upgrade with an empty name reverts.
    function test_registerUpgrade_emptyName_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidUpgradeId.selector);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("", 1);
    }

    /// @notice Tests that registering an upgrade with a name longer than 32 bytes reverts.
    function test_registerUpgrade_nameTooLong_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidUpgradeId.selector);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("this-upgrade-name-is-definitely-too-long", 1);
    }

    /// @notice Tests that only the owner can call `registerUpgrade`.
    function test_registerUpgrade_callerNotOwner_reverts() external {
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.registerUpgrade("antares", 1);
    }
}

/// @title ProtocolVersions_SetTimestamp_Test
/// @notice Test contract for the `setTimestamp` function.
contract ProtocolVersions_SetTimestamp_Test is ProtocolVersions_TestInit {
    /// @notice Tests that setting a timestamp updates the stored value and extends the scheduleId.
    function test_setTimestamp_updatesTimestampAndScheduleId_succeeds() external {
        bytes32 initialScheduleId = protocolVersions.scheduleId();
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts);

        assertEq(protocolVersions.getTimestamp("canyon"), ts);
        assertNotEq(protocolVersions.scheduleId(), initialScheduleId);
    }

    /// @notice Tests that calling `setTimestamp` with the same value is a no-op for scheduleId.
    function test_setTimestamp_sameTimestamp_noScheduleIdChange_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts);

        bytes32 scheduleIdAfterSet = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts);

        assertEq(protocolVersions.getTimestamp("canyon"), ts);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterSet);
    }

    /// @notice Tests that passing 0 clears a scheduled timestamp, changes the scheduleId, and
    ///         restores it to the value it held immediately after registration (ts=0 link).
    function test_setTimestamp_clearTimestamp_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);
        bytes32 scheduleIdAfterRegister = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts);

        bytes32 scheduleIdAfterSet = protocolVersions.scheduleId();
        assertNotEq(scheduleIdAfterSet, scheduleIdAfterRegister);

        vm.roll(block.number + 1);
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", 0);

        assertEq(protocolVersions.getTimestamp("canyon"), 0);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterRegister);
    }

    /// @notice Tests that `setTimestamp` emits a `TimestampSet` event.
    function test_setTimestamp_emitsEvent_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit TimestampSet(bytes32("canyon"), ts);
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts);
    }

    /// @notice Tests that only the owner can call `setTimestamp`.
    function test_setTimestamp_callerNotOwner_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.setTimestamp("canyon", ts);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is in the past.
    function test_setTimestamp_timestampInPast_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.warp(1000);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, uint64(0), uint64(500))
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", 500);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is within MIN_NOTICE of now.
    function test_setTimestamp_insufficientNotice_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() - 1;
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, uint64(0), ts)
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts);
    }

    /// @notice Tests that `setTimestamp` reverts when the upgrade has already activated.
    function test_setTimestamp_afterActivation_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.warp(100);
        uint64 activationTs = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", activationTs);

        vm.warp(activationTs + 1);
        uint64 laterTs = activationTs + protocolVersions.MIN_NOTICE() + 100;
        bytes32 canyonKey = bytes32("canyon");
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, canyonKey, activationTs
            )
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", laterTs);
    }

    /// @notice Tests that `setTimestamp` reverts for an unregistered upgrade.
    function test_setTimestamp_unregisteredUpgrade_reverts() external {
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgradeName.selector, "antares")
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp("antares", ts);
    }

    /// @notice Tests that scheduleId is reproducible from (chainId, address, ordered keys, timestamps).
    function test_setTimestamp_scheduleIdReproducible_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("ecotone", 2);

        uint64 ts1 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 ts2 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 200;
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts1);
        vm.prank(_owner);
        protocolVersions.setTimestamp("ecotone", ts2);

        // Reproduce the chain from scratch.
        bytes32 seed = keccak256(abi.encode(uint256(8453), address(protocolVersions)));
        bytes32 link0 = keccak256(abi.encode(seed, bytes32("canyon"), ts1));
        bytes32 link1 = keccak256(abi.encode(link0, bytes32("ecotone"), ts2));

        assertEq(protocolVersions.scheduleId(), link1);
    }

    /// @notice Tests that `getTimestamp` reverts for an unknown upgrade name.
    function test_getTimestamp_unknownUpgrade_reverts() external {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgradeName.selector, "unknown")
        );
        protocolVersions.getTimestamp("unknown");
    }
}

/// @title ProtocolVersions_DelayTimestamp_Test
/// @notice Test contract for the `delayTimestamp` function.
contract ProtocolVersions_DelayTimestamp_Test is ProtocolVersions_TestInit {
    /// @notice Tests that `delayTimestamp` pushes the activation timestamp later and updates scheduleId.
    function test_delayTimestamp_pushesTimestampLater_succeeds() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        bytes32 scheduleIdBefore = protocolVersions.scheduleId();
        vm.roll(block.number + 1);

        uint64 later = ts + 50;
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp("canyon", later);

        assertEq(protocolVersions.getTimestamp("canyon"), later);
        assertNotEq(protocolVersions.scheduleId(), scheduleIdBefore);
    }

    /// @notice Tests that only the chainTeam can call `delayTimestamp`.
    function test_delayTimestamp_callerNotChainTeam_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.expectRevert(IProtocolVersions.ProtocolVersions_NotChainTeam.selector);
        vm.prank(_owner);
        protocolVersions.delayTimestamp("canyon", ts + 50);

        vm.expectRevert(IProtocolVersions.ProtocolVersions_NotChainTeam.selector);
        vm.prank(_nonOwner);
        protocolVersions.delayTimestamp("canyon", ts + 50);
    }

    /// @notice Tests that `delayTimestamp` reverts when the new timestamp is earlier than current.
    function test_delayTimestamp_earlierTimestamp_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, ts, ts - 10)
        );
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp("canyon", ts - 10);
    }

    /// @notice Tests that `delayTimestamp` reverts when the new timestamp equals the current one.
    function test_delayTimestamp_equalTimestamp_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, ts, ts));
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp("canyon", ts);
    }

    /// @notice Tests that `delayTimestamp` reverts when the upgrade has no scheduled timestamp.
    function test_delayTimestamp_notScheduled_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_NotScheduled.selector, bytes32("canyon"))
        );
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp("canyon", ts);
    }

    /// @notice Tests that `delayTimestamp` reverts when the upgrade has already activated.
    function test_delayTimestamp_afterActivation_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.warp(ts + 1);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, bytes32("canyon"), ts
            )
        );
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp("canyon", ts + 100);
    }

    /// @notice Tests that `delayTimestamp` reverts for an unregistered upgrade.
    function test_delayTimestamp_unregisteredUpgrade_reverts() external {
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgradeName.selector, "antares")
        );
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp("antares", ts);
    }
}

/// @title ProtocolVersions_ChainTeam_Test
/// @notice Test contract for the `setChainTeam` function and chainTeam role.
contract ProtocolVersions_ChainTeam_Test is ProtocolVersions_TestInit {
    /// @notice Tests that `chainTeam` starts as address(0).
    function test_chainTeam_startsUnset_succeeds() external view {
        assertEq(protocolVersions.chainTeam(), address(0));
    }

    /// @notice Tests that the owner can appoint a chainTeam address.
    function test_setChainTeam_setsAddress_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);
        assertEq(protocolVersions.chainTeam(), _chainTeam);
    }

    /// @notice Tests that only the owner can call `setChainTeam`.
    function test_setChainTeam_callerNotOwner_reverts() external {
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.setChainTeam(_chainTeam);
    }

    /// @notice Tests that `setChainTeam` emits a `ChainTeamUpdated` event.
    function test_setChainTeam_emitsEvent_succeeds() external {
        vm.expectEmit(true, true, false, false, address(protocolVersions));
        emit ChainTeamUpdated(address(0), _chainTeam);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);
    }

    /// @notice Tests that the owner can clear the chainTeam role by setting it to address(0).
    function test_setChainTeam_clear_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.expectEmit(true, true, false, false, address(protocolVersions));
        emit ChainTeamUpdated(_chainTeam, address(0));
        vm.prank(_owner);
        protocolVersions.setChainTeam(address(0));

        assertEq(protocolVersions.chainTeam(), address(0));
    }
}

/// @title ProtocolVersions_GetView_Test
/// @notice Test contract for view functions and the upgrade registry.
contract ProtocolVersions_GetView_Test is ProtocolVersions_TestInit {
    /// @notice Tests that the registry starts empty.
    function test_registry_startsEmpty_succeeds() external view {
        assertEq(protocolVersions.getSchedule().length, 0);
    }

    /// @notice Tests that `getSchedule` returns an empty array when no upgrades are registered.
    function test_getSchedule_empty_succeeds() external view {
        assertEq(protocolVersions.getSchedule().length, 0);
    }

    /// @notice Tests that `getSchedule` returns all upgrades in registration order with correct fields.
    function test_getSchedule_returnsFullSchedule_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade("canyon", 1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade("ecotone", 2);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp("canyon", ts);

        IProtocolVersions.Upgrade[] memory s = protocolVersions.getSchedule();

        assertEq(s.length, 2);

        assertEq(s[0].name, "canyon");
        assertEq(s[0].timestamp, ts);

        assertEq(s[1].name, "ecotone");
        assertEq(s[1].timestamp, 0);

        // The last entry's scheduleId is the contract's current scheduleId.
        assertEq(s[1].scheduleId, protocolVersions.scheduleId());
    }
}

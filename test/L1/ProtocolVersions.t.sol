// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";

// Contracts
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";
import { Proxy } from "src/universal/Proxy.sol";

// Interfaces
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";

/// @title ProtocolVersions_TestInit
/// @notice Reusable test initialization for ProtocolVersions tests. Runs against the
///         `protocolVersions` instance deployed by the standard SystemDeploy script.
abstract contract ProtocolVersions_TestInit is CommonTest {
    event UpgradeRegistered(uint256 indexed id);
    event MinimumProtocolVersionUpdated(uint256 indexed protocolVersion);
    event IncidentResponderUpdated(address indexed previousIncidentResponder, address indexed newIncidentResponder);
    event TimestampSet(uint256 indexed id, uint256 timestamp);

    /// @dev Ascending ids assigned by registration order in these tests.
    uint256 internal constant CANYON = 0;
    uint256 internal constant ECOTONE = 1;

    address internal _owner;
    address internal _nonOwner = makeAddr("non-owner");
    address internal _incidentResponder = makeAddr("incident-responder");

    function setUp() public virtual override {
        super.setUp();
        skipIfForkTest("ProtocolVersions_TestInit: cannot test on forked network");
        _owner = proxyAdminOwner;
    }

    /// @dev Registers the first upgrade (id CANYON) and schedules it for block.timestamp + MIN_NOTICE + delay.
    function _scheduleCanyon(uint64 _delay) internal returns (uint64 ts_) {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        ts_ = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + _delay;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts_);
    }

    /// @dev Deploys a fresh uninitialized proxy over the impl produced by SystemDeploy, for the
    ///      tests that genuinely need one: the initializer edge cases and any schedule that has to
    ///      be imported rather than registered. proxyAdminOwner() resolves by calling owner() on the
    ///      ProxyAdmin stored in the proxy slot, so the mock provides one.
    function _deployUninitializedProxy() internal returns (IProtocolVersions) {
        address proxyAdmin = makeAddr("proxy-admin");
        vm.mockCall(proxyAdmin, abi.encodeWithSignature("owner()"), abi.encode(_owner));
        Proxy proxy = new Proxy(proxyAdmin);
        address impl = EIP1967Helper.getImplementation(address(protocolVersions));
        vm.prank(proxyAdmin);
        proxy.upgradeTo(impl);
        return IProtocolVersions(address(proxy));
    }
}

/// @title ProtocolVersions_Initialize_Test
/// @notice Test contract for the ProtocolVersions initializer.
contract ProtocolVersions_Initialize_Test is ProtocolVersions_TestInit {
    /// @notice Tests that initialization sets the correct initial state. The seed is bytes32(0), so
    ///         the initial scheduleId is bytes32(0) until the first upgrade is registered.
    function test_initialize_setsInitialState_succeeds() external view {
        // The owner is inherited from the shared ProxyAdmin; initialize records the incident
        // responder from config and seeds the hash chain (scheduleId == the bytes32(0) seed).
        assertEq(protocolVersions.proxyAdminOwner(), proxyAdminOwner);
        assertEq(protocolVersions.incidentResponder(), deploy.cfg().superchainConfigIncidentResponder());
        assertEq(protocolVersions.scheduleId(), bytes32(0));
    }

    /// @notice Tests that initialization appoints the provided incidentResponder and emits the event.
    /// @dev Requires a fresh uninitialized proxy rather than the already-initialized shared instance.
    function test_initialize_setsIncidentResponder_succeeds() external {
        IProtocolVersions uninitialized = _deployUninitializedProxy();
        vm.expectEmit(true, true, false, false, address(uninitialized));
        emit IncidentResponderUpdated(address(0), _incidentResponder);
        vm.prank(EIP1967Helper.getAdmin(address(uninitialized)));
        uninitialized.initialize(_incidentResponder, new uint64[](0));
        assertEq(uninitialized.incidentResponder(), _incidentResponder);
    }

    /// @notice Tests that only the ProxyAdmin or its owner can initialize.
    /// @dev Requires a fresh uninitialized proxy rather than the already-initialized shared instance.
    function test_initialize_notProxyAdminOrOwner_reverts() external {
        IProtocolVersions uninitialized = _deployUninitializedProxy();
        vm.expectRevert(IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        uninitialized.initialize(_incidentResponder, new uint64[](0));
    }

    /// @notice Tests that the initializer imports a preexisting schedule, building the same hash
    ///         chain the equivalent sequence of registrations would have.
    function test_initialize_importsSchedule_succeeds() external {
        uint64[] memory schedule = new uint64[](3);
        schedule[0] = 10;
        schedule[1] = 0;
        schedule[2] = 30;

        IProtocolVersions imported = _deployUninitializedProxy();
        vm.prank(EIP1967Helper.getAdmin(address(imported)));
        imported.initialize(_incidentResponder, schedule);

        uint64[] memory stored = imported.getSchedule();
        assertEq(stored.length, schedule.length);
        assertEq(stored[0], schedule[0]);
        assertEq(stored[1], schedule[1]);
        assertEq(stored[2], schedule[2]);

        bytes32 link0 = keccak256(abi.encode(bytes32(0), uint256(0), uint64(10)));
        bytes32 link1 = keccak256(abi.encode(link0, uint256(1), uint64(0)));
        bytes32 link2 = keccak256(abi.encode(link1, uint256(2), uint64(30)));
        assertEq(imported.scheduleId(0), link0);
        assertEq(imported.scheduleId(), link2);
    }

    /// @notice Tests that an imported schedule is held to the same ordering rule as registration.
    function test_initialize_importUnorderedSchedule_reverts() external {
        uint64[] memory schedule = new uint64[](2);
        schedule[0] = 30;
        schedule[1] = 10;

        IProtocolVersions imported = _deployUninitializedProxy();
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_TimestampNotAfterPrevious.selector,
                ECOTONE,
                CANYON,
                uint64(30),
                uint64(10)
            )
        );
        vm.prank(EIP1967Helper.getAdmin(address(imported)));
        imported.initialize(address(0), schedule);
    }

    /// @notice Tests that the contract cannot be initialized twice.
    function test_initialize_alreadyInitialized_reverts() external {
        vm.expectRevert("Initializable: contract is already initialized");
        vm.prank(EIP1967Helper.getAdmin(address(protocolVersions)));
        protocolVersions.initialize(address(0), new uint64[](0));
    }

    /// @notice Tests that the implementation itself cannot be initialized (initializers disabled).
    function test_initialize_implementationDisabled_reverts() external {
        IProtocolVersions impl = IProtocolVersions(EIP1967Helper.getImplementation(address(protocolVersions)));
        vm.expectRevert("Initializable: contract is already initialized");
        impl.initialize(address(0), new uint64[](0));
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
        protocolVersions.registerUpgrade(0, 0);

        assertNotEq(protocolVersions.scheduleId(), idBefore);
    }

    /// @notice Tests that `registerUpgrade` assigns ascending ids and returns them.
    function test_registerUpgrade_returnsAscendingIds_succeeds() external {
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(0, 0), 0);
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(0, 0), 1);
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(0, 0), 2);
    }

    /// @notice Tests that `registerUpgrade` emits the `UpgradeRegistered` event with the assigned id.
    function test_registerUpgrade_emitsEvent_succeeds() external {
        vm.expectEmit(true, false, false, false, address(protocolVersions));
        emit UpgradeRegistered(0);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        vm.expectEmit(true, false, false, false, address(protocolVersions));
        emit UpgradeRegistered(1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
    }

    /// @notice Tests that only the owner can call `registerUpgrade`.
    function test_registerUpgrade_callerNotOwner_reverts() external {
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.registerUpgrade(0, 0);
    }

    /// @notice Tests that registering with a future timestamp schedules the upgrade in one call.
    function test_registerUpgrade_withTimestamp_succeeds() external {
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;

        vm.expectEmit(true, false, false, false, address(protocolVersions));
        emit UpgradeRegistered(CANYON);
        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit TimestampSet(CANYON, ts);
        vm.prank(_owner);
        uint256 id = protocolVersions.registerUpgrade(ts, 0);

        assertEq(id, CANYON);
        assertEq(protocolVersions.getSchedule()[CANYON], ts);
        assertNotEq(protocolVersions.scheduleId(), bytes32(0));
    }

    /// @notice Tests that registration is held to the same notice floor as `setTimestamp`. Appending
    ///         an activation that L1 is already close to would move `activatedScheduleId` for L2
    ///         timestamps the sequencer may already have produced, which is what FREEZE_WINDOW
    ///         prevents on the paths that change an existing activation.
    function test_registerUpgrade_insufficientNotice_reverts() external {
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() - 1;
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_InsufficientNotice.selector, ts));
        vm.prank(_owner);
        protocolVersions.registerUpgrade(ts, 0);
    }

    /// @notice Tests that registering without a timestamp stays unconstrained: a zero entry is
    ///         skipped by `activatedScheduleId`, so it cannot move an already-reachable answer.
    function test_registerUpgrade_zeroTimestampSkipsNotice_succeeds() external {
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(0, 0), CANYON);
        assertEq(protocolVersions.getSchedule()[CANYON], 0);
    }

    /// @notice Tests the property the notice floor exists to protect: an append cannot change the
    ///         commitment for an L2 timestamp the sequencer may already have produced, and an
    ///         append that clears the floor lands above every such timestamp.
    function test_registerUpgrade_cannotMoveReachableSchedule_reverts() external {
        uint64 reachable = uint64(block.timestamp) + protocolVersions.FREEZE_WINDOW();
        uint64 allowed = uint64(block.timestamp) + protocolVersions.MIN_NOTICE();
        bytes32 settled = protocolVersions.activatedScheduleId(reachable);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_InsufficientNotice.selector, reachable)
        );
        vm.prank(_owner);
        protocolVersions.registerUpgrade(reachable, 0);

        vm.prank(_owner);
        protocolVersions.registerUpgrade(allowed, 0);

        assertEq(protocolVersions.activatedScheduleId(reachable), settled);
    }

    /// @notice Tests that a scheduled registration may share the previous scheduled upgrade's timestamp.
    function test_registerUpgrade_timestampEqualToPrevious_succeeds() external {
        uint64 first = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 second = first;

        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(first, 0), CANYON);
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(second, 0), ECOTONE);

        uint64[] memory schedule = protocolVersions.getSchedule();
        assertEq(schedule[CANYON], first);
        assertEq(schedule[ECOTONE], second);
    }

    /// @notice Tests that zero remains the unscheduled/disabled value and is exempt from ordering.
    function test_registerUpgrade_zeroTimestampAfterScheduledPrevious_succeeds() external {
        uint64 first = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;

        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(first, 0), CANYON);
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(0, 0), ECOTONE);

        uint64[] memory schedule = protocolVersions.getSchedule();
        assertEq(schedule[CANYON], first);
        assertEq(schedule[ECOTONE], 0);
    }

    /// @notice Tests that scheduled registration scans past zero holes to the previous timestamp.
    function test_registerUpgrade_timestampAfterPreviousSkipsZeroHoles_succeeds() external {
        uint64 first = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 second = first + 1;

        vm.startPrank(_owner);
        assertEq(protocolVersions.registerUpgrade(first, 0), CANYON);
        assertEq(protocolVersions.registerUpgrade(0, 0), ECOTONE);
        assertEq(protocolVersions.registerUpgrade(second, 0), 2);
        vm.stopPrank();

        uint64[] memory schedule = protocolVersions.getSchedule();
        assertEq(schedule[CANYON], first);
        assertEq(schedule[ECOTONE], 0);
        assertEq(schedule[2], second);
    }

    /// @notice Tests that registering a timestamp before the previous scheduled upgrade reverts.
    function test_registerUpgrade_timestampNotAfterPrevious_reverts() external {
        uint64 first = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;

        vm.prank(_owner);
        protocolVersions.registerUpgrade(first, 0);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_TimestampNotAfterPrevious.selector, ECOTONE, CANYON, first, first - 1
            )
        );
        vm.prank(_owner);
        protocolVersions.registerUpgrade(first - 1, 0);
    }

    /// @notice Tests that a non-zero minProtocolVersion bumps the minimum during registration.
    function test_registerUpgrade_setsMinProtocolVersion_succeeds() external {
        vm.expectEmit(true, false, false, false, address(protocolVersions));
        emit MinimumProtocolVersionUpdated(42);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 42);

        assertEq(protocolVersions.minimumProtocolVersion(), 42);
    }

    /// @notice Tests that a zero minProtocolVersion leaves the current minimum unchanged.
    function test_registerUpgrade_zeroMinProtocolVersion_leavesUnchanged_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setMinimumProtocolVersion(7);

        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        assertEq(protocolVersions.minimumProtocolVersion(), 7);
    }

    /// @notice Tests that a minProtocolVersion exceeding 128 bits reverts.
    function test_registerUpgrade_minProtocolVersionTooLarge_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidProtocolVersion.selector);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, uint256(type(uint128).max) + 1);
    }

    /// @notice Tests that the maximum representable minProtocolVersion (128 bits set) succeeds.
    function test_registerUpgrade_minProtocolVersionMaxValue_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, type(uint128).max);
        assertEq(protocolVersions.minimumProtocolVersion(), type(uint128).max);
    }
}

/// @title ProtocolVersions_SetMinimumProtocolVersion_Test
/// @notice Test contract for the `setMinimumProtocolVersion` function.
contract ProtocolVersions_SetMinimumProtocolVersion_Test is ProtocolVersions_TestInit {
    /// @notice Tests that the owner can set the minimum protocol version.
    function test_setMinimumProtocolVersion_updates_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setMinimumProtocolVersion(42);
        assertEq(protocolVersions.minimumProtocolVersion(), 42);
    }

    /// @notice Tests that setting the minimum protocol version does not change the scheduleId.
    function test_setMinimumProtocolVersion_doesNotChangeScheduleId_succeeds() external {
        bytes32 scheduleIdBefore = protocolVersions.scheduleId();
        vm.prank(_owner);
        protocolVersions.setMinimumProtocolVersion(42);
        assertEq(protocolVersions.scheduleId(), scheduleIdBefore);
    }

    /// @notice Tests that `setMinimumProtocolVersion` emits the `MinimumProtocolVersionUpdated` event.
    function test_setMinimumProtocolVersion_emitsEvent_succeeds() external {
        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit MinimumProtocolVersionUpdated(42);
        vm.prank(_owner);
        protocolVersions.setMinimumProtocolVersion(42);
    }

    /// @notice Tests that setting a zero protocol version reverts.
    function test_setMinimumProtocolVersion_zero_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidProtocolVersion.selector);
        vm.prank(_owner);
        protocolVersions.setMinimumProtocolVersion(0);
    }

    /// @notice Tests that a protocol version exceeding 128 bits reverts.
    function test_setMinimumProtocolVersion_tooLarge_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidProtocolVersion.selector);
        vm.prank(_owner);
        protocolVersions.setMinimumProtocolVersion(uint256(type(uint128).max) + 1);
    }

    /// @notice Tests that the maximum representable protocol version (128 bits set) succeeds.
    function test_setMinimumProtocolVersion_maxValue_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setMinimumProtocolVersion(type(uint128).max);
        assertEq(protocolVersions.minimumProtocolVersion(), type(uint128).max);
    }

    /// @notice Tests that only the owner can call `setMinimumProtocolVersion`.
    function test_setMinimumProtocolVersion_callerNotOwner_reverts() external {
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.setMinimumProtocolVersion(42);
    }
}

/// @title ProtocolVersions_SetTimestamp_Test
/// @notice Test contract for the `setTimestamp` function.
contract ProtocolVersions_SetTimestamp_Test is ProtocolVersions_TestInit {
    /// @notice Tests that setting a timestamp updates the stored value and extends the scheduleId.
    function test_setTimestamp_updatesTimestampAndScheduleId_succeeds() external {
        bytes32 initialScheduleId = protocolVersions.scheduleId();
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        assertEq(protocolVersions.getSchedule()[CANYON], ts);
        assertNotEq(protocolVersions.scheduleId(), initialScheduleId);
    }

    /// @notice Tests that calling `setTimestamp` with the same value is a no-op for scheduleId.
    function test_setTimestamp_sameTimestamp_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        bytes32 scheduleIdAfterSet = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        assertEq(protocolVersions.getSchedule()[CANYON], ts);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterSet);
    }

    /// @notice Tests that passing 0 clears a scheduled timestamp, changes the scheduleId, and
    ///         restores it to the value it held immediately after registration (ts=0 link).
    function test_setTimestamp_clearTimestamp_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        bytes32 scheduleIdAfterRegister = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        bytes32 scheduleIdAfterSet = protocolVersions.scheduleId();
        assertNotEq(scheduleIdAfterSet, scheduleIdAfterRegister);

        vm.roll(block.number + 1);
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, 0);

        assertEq(protocolVersions.getSchedule()[CANYON], 0);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterRegister);
    }

    /// @notice Tests that `setTimestamp` emits a `TimestampSet` event.
    function test_setTimestamp_emitsEvent_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit TimestampSet(CANYON, ts);
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that only the owner can call `setTimestamp`.
    function test_setTimestamp_callerNotOwner_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is in the past.
    function test_setTimestamp_timestampInPast_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        vm.warp(1000);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_InsufficientNotice.selector, uint64(500))
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, 500);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is within MIN_NOTICE of now.
    function test_setTimestamp_insufficientNotice_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() - 1;
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_InsufficientNotice.selector, ts));
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that a zero-valued hole below a scheduled successor cannot be scheduled later.
    function test_setTimestamp_staticScheduleHole_reverts() external {
        uint64 successor = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 200;
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;

        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(successor, 0);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_StaticScheduleHole.selector, CANYON, ECOTONE)
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that static-hole detection scans past zero holes to the next scheduled upgrade.
    function test_setTimestamp_staticScheduleHoleSkipsZeroHoles_reverts() external {
        uint64 successor = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 200;
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;

        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(successor, 0);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_StaticScheduleHole.selector, CANYON, uint256(2))
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that `setTimestamp` may share the previous scheduled upgrade's timestamp.
    function test_setTimestamp_timestampEqualToPrevious_succeeds() external {
        uint64 previous = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;

        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(previous, 0);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.setTimestamp(ECOTONE, previous);
        vm.stopPrank();

        uint64[] memory schedule = protocolVersions.getSchedule();
        assertEq(schedule[CANYON], previous);
        assertEq(schedule[ECOTONE], previous);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is before the previous one.
    function test_setTimestamp_timestampNotAfterPrevious_reverts() external {
        uint64 previous = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;

        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(previous, 0);
        protocolVersions.registerUpgrade(0, 0);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_TimestampNotAfterPrevious.selector,
                ECOTONE,
                CANYON,
                previous,
                previous - 1
            )
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(ECOTONE, previous - 1);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is not before the next one.
    function test_setTimestamp_timestampNotBeforeNext_reverts() external {
        uint64 current = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 next = current + 100;

        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(current, 0);
        protocolVersions.registerUpgrade(next, 0);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_TimestampNotBeforeNext.selector, CANYON, ECOTONE, next, next
            )
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, next);
    }

    /// @notice Tests that `setTimestamp` reverts when the upgrade has already activated.
    function test_setTimestamp_afterActivation_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        vm.warp(100);
        uint64 activationTs = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, activationTs);

        vm.warp(activationTs + 1);
        uint64 laterTs = activationTs + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, CANYON, activationTs
            )
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, laterTs);
    }

    /// @notice Tests that a scheduled activation can no longer be cleared once L1 is within
    ///         FREEZE_WINDOW of it, which is the point from which an L2 block carrying that
    ///         activation may already exist.
    function test_setTimestamp_insideFreezeWindow_reverts() external {
        uint64 ts = _scheduleCanyon(100);

        vm.warp(ts - protocolVersions.FREEZE_WINDOW());
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_ActivationFrozen.selector, CANYON, ts)
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, 0);
    }

    /// @notice Tests that the freeze boundary is exact: one second before it, the activation is
    ///         still clearable.
    function test_setTimestamp_justBeforeFreezeWindow_succeeds() external {
        uint64 ts = _scheduleCanyon(100);

        vm.warp(ts - protocolVersions.FREEZE_WINDOW() - 1);
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, 0);

        assertEq(protocolVersions.getSchedule()[CANYON], 0);
    }

    /// @notice Tests that `setTimestamp` reverts for an unregistered upgrade.
    function test_setTimestamp_unregisteredUpgrade_reverts() external {
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgrade.selector, uint256(0)));
        vm.prank(_owner);
        protocolVersions.setTimestamp(0, ts);
    }

    /// @notice Tests that scheduleId is reproducible from (ascending ids, timestamps).
    function test_setTimestamp_scheduleIdReproducible_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts1 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 ts2 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 200;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts1);
        vm.prank(_owner);
        protocolVersions.setTimestamp(ECOTONE, ts2);

        // Reproduce the chain from scratch, starting from the bytes32(0) seed.
        bytes32 seed = bytes32(0);
        bytes32 link0 = keccak256(abi.encode(seed, uint256(0), ts1));
        bytes32 link1 = keccak256(abi.encode(link0, uint256(1), ts2));

        assertEq(protocolVersions.scheduleId(), link1);
    }
}

/// @title ProtocolVersions_DelayTimestamp_Test
/// @notice Test contract for the `delayTimestamp` function.
contract ProtocolVersions_DelayTimestamp_Test is ProtocolVersions_TestInit {
    /// @notice Tests that `delayTimestamp` pushes the activation timestamp later and updates scheduleId.
    function test_delayTimestamp_pushesTimestampLater_succeeds() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        bytes32 scheduleIdBefore = protocolVersions.scheduleId();
        vm.roll(block.number + 1);

        uint64 later = ts + 50;
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(CANYON, later);

        assertEq(protocolVersions.getSchedule()[CANYON], later);
        assertNotEq(protocolVersions.scheduleId(), scheduleIdBefore);
    }

    /// @notice Tests that only the incidentResponder can call `delayTimestamp`.
    function test_delayTimestamp_callerNotIncidentResponder_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        vm.expectRevert(IProtocolVersions.ProtocolVersions_NotIncidentResponder.selector);
        vm.prank(_owner);
        protocolVersions.delayTimestamp(CANYON, ts + 50);

        vm.expectRevert(IProtocolVersions.ProtocolVersions_NotIncidentResponder.selector);
        vm.prank(_nonOwner);
        protocolVersions.delayTimestamp(CANYON, ts + 50);
    }

    /// @notice Tests that `delayTimestamp` reverts when the new timestamp is earlier than current.
    function test_delayTimestamp_earlierTimestamp_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, ts, ts - 10)
        );
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(CANYON, ts - 10);
    }

    /// @notice Tests that `delayTimestamp` reverts when the new timestamp equals the current one.
    function test_delayTimestamp_equalTimestamp_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, ts, ts));
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(CANYON, ts);
    }

    /// @notice Tests that `delayTimestamp` cannot move an upgrade to or beyond its next scheduled successor.
    function test_delayTimestamp_timestampNotBeforeNext_reverts() external {
        uint64 current = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 next = current + 100;

        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(current, 0);
        protocolVersions.registerUpgrade(next, 0);
        protocolVersions.setIncidentResponder(_incidentResponder);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_TimestampNotBeforeNext.selector, CANYON, ECOTONE, next, next
            )
        );
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(CANYON, next);
    }

    /// @notice Tests that `delayTimestamp` reverts when the upgrade has no scheduled timestamp.
    function test_delayTimestamp_notScheduled_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_NotScheduled.selector, CANYON));
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(CANYON, ts);
    }

    /// @notice Tests that `delayTimestamp` reverts when the upgrade has already activated.
    function test_delayTimestamp_afterActivation_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        vm.warp(ts + 1);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, CANYON, ts)
        );
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(CANYON, ts + 100);
    }

    /// @notice Tests that the incidentResponder also loses the ability to move an activation once
    ///         L1 is within FREEZE_WINDOW of it.
    function test_delayTimestamp_insideFreezeWindow_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        uint64 later = ts + protocolVersions.MIN_NOTICE();
        vm.warp(ts - protocolVersions.FREEZE_WINDOW());
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_ActivationFrozen.selector, CANYON, ts)
        );
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(CANYON, later);
    }

    /// @notice Tests that `delayTimestamp` reverts for an unregistered upgrade.
    function test_delayTimestamp_unregisteredUpgrade_reverts() external {
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgrade.selector, uint256(0)));
        vm.prank(_incidentResponder);
        protocolVersions.delayTimestamp(0, ts);
    }
}

/// @title ProtocolVersions_IncidentResponder_Test
/// @notice Test contract for the `setIncidentResponder` function and incidentResponder role.
contract ProtocolVersions_IncidentResponder_Test is ProtocolVersions_TestInit {
    /// @notice Tests that `incidentResponder` starts as address(0).
    function test_incidentResponder_startsUnset_succeeds() external view {
        assertEq(protocolVersions.incidentResponder(), address(0));
    }

    /// @notice Tests that the owner can appoint a incidentResponder address.
    function test_setIncidentResponder_setsAddress_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);
        assertEq(protocolVersions.incidentResponder(), _incidentResponder);
    }

    /// @notice Tests that only the owner can call `setIncidentResponder`.
    function test_setIncidentResponder_callerNotOwner_reverts() external {
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.setIncidentResponder(_incidentResponder);
    }

    /// @notice Tests that `setIncidentResponder` emits a `IncidentResponderUpdated` event.
    function test_setIncidentResponder_emitsEvent_succeeds() external {
        vm.expectEmit(true, true, false, false, address(protocolVersions));
        emit IncidentResponderUpdated(address(0), _incidentResponder);
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);
    }

    /// @notice Tests that the owner can clear the incidentResponder role by setting it to address(0).
    function test_setIncidentResponder_clear_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(_incidentResponder);

        vm.expectEmit(true, true, false, false, address(protocolVersions));
        emit IncidentResponderUpdated(_incidentResponder, address(0));
        vm.prank(_owner);
        protocolVersions.setIncidentResponder(address(0));

        assertEq(protocolVersions.incidentResponder(), address(0));
    }
}

/// @title ProtocolVersions_Uncategorized_Test
/// @notice Test contract for view functions and the upgrade registry.
contract ProtocolVersions_Uncategorized_Test is ProtocolVersions_TestInit {
    /// @notice Tests that `getSchedule` returns an empty array when no upgrades are registered.
    function test_getSchedule_empty_succeeds() external view {
        assertEq(protocolVersions.getSchedule().length, 0);
    }

    /// @notice Tests that `getSchedule` returns all upgrades in registration order with correct fields.
    function test_getSchedule_returnsFullSchedule_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        uint64[] memory s = protocolVersions.getSchedule();

        assertEq(s.length, 2);
        assertEq(s[CANYON], ts);
        assertEq(s[ECOTONE], 0);
    }

    /// @notice Tests that `scheduleId(id)` returns each upgrade's cumulative commitment, and that
    ///         the last upgrade's commitment equals the current scheduleId.
    function test_scheduleId_byId_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        assertNotEq(protocolVersions.scheduleId(CANYON), protocolVersions.scheduleId(ECOTONE));
        assertEq(protocolVersions.scheduleId(ECOTONE), protocolVersions.scheduleId());
    }

    /// @notice Tests that `scheduleId(id)` reverts for an unregistered upgrade.
    function test_scheduleId_byId_unregistered_reverts() external {
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgrade.selector, uint256(0)));
        protocolVersions.scheduleId(0);
    }
}

/// @title ProtocolVersions_ActivatedScheduleId_Test
/// @notice Tests for commitments to the upgrades active at a supplied L2 timestamp.
contract ProtocolVersions_ActivatedScheduleId_Test is ProtocolVersions_TestInit {
    /// @notice An empty registry and a registry containing only inactive entries both commit to the
    ///         zero hash-chain seed.
    function test_activatedScheduleId_noneActivated_returnsSeed() external {
        assertEq(protocolVersions.activatedScheduleId(uint64(block.timestamp)), bytes32(0));

        uint64 future = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(future, 0);
        vm.stopPrank();

        assertEq(protocolVersions.activatedScheduleId(uint64(block.timestamp)), bytes32(0));
    }

    /// @notice Activation is inclusive at the supplied L2 timestamp.
    function test_activatedScheduleId_boundaryInclusive_succeeds() external {
        uint64[] memory schedule = new uint64[](1);
        schedule[0] = 100;
        IProtocolVersions imported = _importSchedule(schedule);

        assertEq(imported.activatedScheduleId(99), bytes32(0));
        assertEq(imported.activatedScheduleId(100), keccak256(abi.encode(bytes32(0), uint256(CANYON), uint64(100))));
    }

    /// @notice The commitment includes every registered entry through the highest active upgrade,
    ///         including a static zero-valued hole below it.
    function test_activatedScheduleId_commitsPrefixThroughHighestActive_succeeds() external {
        uint64[] memory schedule = new uint64[](3);
        schedule[0] = 10;
        schedule[1] = 0;
        schedule[2] = 30;
        IProtocolVersions imported = _importSchedule(schedule);

        bytes32 link0 = keccak256(abi.encode(bytes32(0), uint256(0), uint64(10)));
        bytes32 link1 = keccak256(abi.encode(link0, uint256(1), uint64(0)));
        bytes32 link2 = keccak256(abi.encode(link1, uint256(2), uint64(30)));

        assertEq(imported.activatedScheduleId(30), link2);
    }

    /// @notice Appending unscheduled or future upgrades above the active prefix cannot move the
    ///         activated commitment selected by that prefix.
    function test_activatedScheduleId_stableAcrossAppendsAboveActivePrefix_succeeds() external {
        uint64[] memory schedule = new uint64[](1);
        schedule[0] = 10;
        IProtocolVersions imported = _importSchedule(schedule);
        bytes32 pinned = imported.activatedScheduleId(10);

        uint64 future = uint64(block.timestamp) + imported.MIN_NOTICE();
        vm.startPrank(_owner);
        imported.registerUpgrade(0, 0);
        imported.registerUpgrade(future, 0);
        vm.stopPrank();

        assertEq(imported.activatedScheduleId(10), pinned);
        assertNotEq(imported.scheduleId(), pinned);
    }

    /// @notice Cross-implementation golden shared with Base's `ScheduleId` tests for the real Base
    ///         mainnet static schedule. The activation cutoff is Beryl, so the commitment contains
    ///         the prefix through id 11, including the static PectraBlobSchedule hole at id 7, and
    ///         excludes the unscheduled Cobalt placeholder at id 12.
    function test_activatedScheduleId_matchesBaseMainnetGoldenValue_succeeds() external {
        IProtocolVersions mainnet = _importBaseMainnetStaticSchedule();

        assertEq(
            mainnet.activatedScheduleId(BASE_MAINNET_BERYL_TIMESTAMP),
            0xadd4aa9bd3532969035a9543c16b8c7d71298e15836f0ac731fdd3eea552c6e2
        );
    }

    /// @notice Cross-implementation golden for the full real Base mainnet contract-backed schedule.
    ///         The live tail commits to all 13 entries, including PectraBlobSchedule and Cobalt as
    ///         unscheduled zero-timestamp entries.
    function test_scheduleId_matchesBaseMainnetFullScheduleGoldenValue_succeeds() external {
        IProtocolVersions mainnet = _importBaseMainnetStaticSchedule();

        assertEq(mainnet.scheduleId(), 0x5ee41f186b0a439783060587cfbb942f6f1d94ecc76376c9782580c943ff2b6d);
        assertEq(mainnet.scheduleId(12), mainnet.scheduleId());
    }

    uint64 private constant BASE_MAINNET_GENESIS_TIMESTAMP = 1_686_789_347;
    uint64 private constant BASE_MAINNET_CANYON_TIMESTAMP = 1_704_992_401;
    uint64 private constant BASE_MAINNET_DELTA_TIMESTAMP = 1_708_560_000;
    uint64 private constant BASE_MAINNET_ECOTONE_TIMESTAMP = 1_710_374_401;
    uint64 private constant BASE_MAINNET_FJORD_TIMESTAMP = 1_720_627_201;
    uint64 private constant BASE_MAINNET_GRANITE_TIMESTAMP = 1_726_070_401;
    uint64 private constant BASE_MAINNET_HOLOCENE_TIMESTAMP = 1_736_445_601;
    uint64 private constant BASE_MAINNET_ISTHMUS_TIMESTAMP = 1_746_806_401;
    uint64 private constant BASE_MAINNET_JOVIAN_TIMESTAMP = 1_764_691_201;
    uint64 private constant BASE_MAINNET_AZUL_TIMESTAMP = 1_779_991_200;
    uint64 private constant BASE_MAINNET_BERYL_TIMESTAMP = 1_782_410_400;

    /// @dev The real Base mainnet schedule is entirely in the past, so the initializer's import is
    ///      the only path that can enter it. The resulting chain must match what the equivalent
    ///      sequence of `registerUpgrade` calls would have produced, which these goldens pin.
    function _importBaseMainnetStaticSchedule() private returns (IProtocolVersions) {
        uint64[] memory schedule = new uint64[](13);
        schedule[0] = BASE_MAINNET_GENESIS_TIMESTAMP;
        schedule[1] = BASE_MAINNET_CANYON_TIMESTAMP;
        schedule[2] = BASE_MAINNET_DELTA_TIMESTAMP;
        schedule[3] = BASE_MAINNET_ECOTONE_TIMESTAMP;
        schedule[4] = BASE_MAINNET_FJORD_TIMESTAMP;
        schedule[5] = BASE_MAINNET_GRANITE_TIMESTAMP;
        schedule[6] = BASE_MAINNET_HOLOCENE_TIMESTAMP;
        schedule[7] = 0; // PectraBlobSchedule is unscheduled on Base mainnet.
        schedule[8] = BASE_MAINNET_ISTHMUS_TIMESTAMP;
        schedule[9] = BASE_MAINNET_JOVIAN_TIMESTAMP;
        schedule[10] = BASE_MAINNET_AZUL_TIMESTAMP;
        schedule[11] = BASE_MAINNET_BERYL_TIMESTAMP;
        schedule[12] = 0; // Cobalt is unscheduled on Base mainnet.

        return _importSchedule(schedule);
    }

    /// @dev These cases pin `activatedScheduleId` against activations that are already in the past,
    ///      which only the initializer's import can produce.
    function _importSchedule(uint64[] memory schedule) private returns (IProtocolVersions) {
        IProtocolVersions imported = _deployUninitializedProxy();
        vm.prank(EIP1967Helper.getAdmin(address(imported)));
        imported.initialize(address(0), schedule);
        return imported;
    }
}

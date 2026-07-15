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
        uninitialized.initialize(_incidentResponder);
        assertEq(uninitialized.incidentResponder(), _incidentResponder);
    }

    /// @notice Tests that only the ProxyAdmin or its owner can initialize.
    /// @dev Requires a fresh uninitialized proxy rather than the already-initialized shared instance.
    function test_initialize_notProxyAdminOrOwner_reverts() external {
        IProtocolVersions uninitialized = _deployUninitializedProxy();
        vm.expectRevert(IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        uninitialized.initialize(_incidentResponder);
    }

    /// @notice Tests that the contract cannot be initialized twice.
    function test_initialize_alreadyInitialized_reverts() external {
        vm.expectRevert("Initializable: contract is already initialized");
        vm.prank(EIP1967Helper.getAdmin(address(protocolVersions)));
        protocolVersions.initialize(address(0));
    }

    /// @notice Tests that the implementation itself cannot be initialized (initializers disabled).
    function test_initialize_implementationDisabled_reverts() external {
        IProtocolVersions impl = IProtocolVersions(EIP1967Helper.getImplementation(address(protocolVersions)));
        vm.expectRevert("Initializable: contract is already initialized");
        impl.initialize(address(0));
    }

    /// @dev Deploys a fresh uninitialized proxy over the impl produced by SystemDeploy for the two
    ///      initializer tests that genuinely need one. proxyAdminOwner() resolves by calling owner()
    ///      on the ProxyAdmin stored in the proxy slot, so the mock provides one.
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

    /// @notice Tests that registering with a timestamp inside the notice window succeeds — the
    ///         notice period is not enforced at registration, only via `setTimestamp`/`delayTimestamp`.
    function test_registerUpgrade_shortNotice_succeeds() external {
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() - 1;
        vm.prank(_owner);
        protocolVersions.registerUpgrade(ts, 0);
        assertEq(protocolVersions.getSchedule()[CANYON], ts);
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

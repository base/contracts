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
    event ScheduleIdUpdated(bytes32 indexed newScheduleId);

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
    /// @notice Tests that registering an upgrade without a timestamp leaves the scheduleId
    ///         unchanged: unscheduled upgrades carry the previous link forward, so the registry
    ///         and offchain provers can add upgrades asynchronously.
    function test_registerUpgrade_unscheduled_keepsScheduleId_succeeds() external {
        // Register-only from the empty registry: scheduleId stays at the bytes32(0) seed.
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        assertEq(protocolVersions.scheduleId(), bytes32(0));

        // Schedule the first upgrade, then register another without a timestamp: the new
        // unscheduled entry must not move the commitment.
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);
        bytes32 idBefore = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        assertEq(protocolVersions.scheduleId(), idBefore);
    }

    /// @notice Tests that register-only still emits `ScheduleIdUpdated`, carrying the unchanged
    ///         commitment. Pins the deliberate emit-always semantics so indexers can rely on the
    ///         event firing for every registration, scheduled or not.
    function test_registerUpgrade_unscheduled_emitsUnchangedScheduleId_succeeds() external {
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.registerUpgrade(ts, 0);
        bytes32 idBefore = protocolVersions.scheduleId();

        vm.expectEmit(true, false, false, false, address(protocolVersions));
        emit ScheduleIdUpdated(idBefore);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
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
    ///         restores it to the value it held immediately after registration (the cleared entry
    ///         carries the previous link forward again).
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

    /// @notice Tests that `setTimestamp` succeeds at exactly `block.timestamp + MIN_NOTICE`, the
    ///         boundary of the notice window.
    function test_setTimestamp_exactMinNotice_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE();
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        assertEq(protocolVersions.getSchedule()[CANYON], ts);
    }

    /// @notice Tests that `setTimestamp` emits `ScheduleIdUpdated` carrying the exact new
    ///         commitment tail.
    function test_setTimestamp_emitsScheduleIdUpdated_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        bytes32 expectedTail = keccak256(abi.encode(bytes32(0), uint256(0), ts));

        vm.expectEmit(true, false, false, false, address(protocolVersions));
        emit ScheduleIdUpdated(expectedTail);
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        assertEq(protocolVersions.scheduleId(), expectedTail);
    }

    /// @notice Tests that clearing a mid-chain upgrade recomputes all downstream links: the
    ///         cleared entry carries its predecessor's link forward and every later scheduled
    ///         entry is rehashed on top, leaving no stale link behind.
    function test_setTimestamp_clearMidChain_recomputesDownstreamLinks_succeeds() external {
        // Register and schedule three upgrades.
        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts1 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 ts2 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 200;
        uint64 ts3 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 300;
        protocolVersions.setTimestamp(0, ts1);
        protocolVersions.setTimestamp(1, ts2);
        protocolVersions.setTimestamp(2, ts3);

        // Clear the middle upgrade, punching a gap into a fully scheduled chain.
        protocolVersions.setTimestamp(1, 0);
        vm.stopPrank();

        // The chain now skips id 1: id 0 links directly to id 2.
        bytes32 link0 = keccak256(abi.encode(bytes32(0), uint256(0), ts1));
        bytes32 link2 = keccak256(abi.encode(link0, uint256(2), ts3));
        assertEq(protocolVersions.scheduleId(), link2);

        // Per-id commitments: the cleared id 1 carries id 0's link forward, and id 2 was
        // recomputed on top of it rather than left holding the stale ts2-inclusive link.
        assertEq(protocolVersions.scheduleId(0), link0);
        assertEq(protocolVersions.scheduleId(1), link0);
        assertEq(protocolVersions.scheduleId(2), link2);
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

    /// @notice Tests that an unscheduled upgrade in the middle of the registry is skipped by the
    ///         scheduleId chain: only scheduled (id, timestamp) pairs contribute links, so a
    ///         permanent gap (e.g. a fork that never activates on this chain) does not shift the
    ///         commitment.
    function test_setTimestamp_scheduleIdSkipsUnscheduledGap_succeeds() external {
        // Register three upgrades; only ids 0 and 2 get scheduled, id 1 stays unscheduled.
        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(0, 0);
        protocolVersions.registerUpgrade(0, 0);

        uint64 ts1 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 ts3 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 300;
        protocolVersions.setTimestamp(0, ts1);
        protocolVersions.setTimestamp(2, ts3);
        vm.stopPrank();

        // The chain links ids 0 and 2 directly; the unscheduled id 1 contributes no link but its
        // id still pins each scheduled entry to its registration position.
        bytes32 link0 = keccak256(abi.encode(bytes32(0), uint256(0), ts1));
        bytes32 link2 = keccak256(abi.encode(link0, uint256(2), ts3));

        assertEq(protocolVersions.scheduleId(), link2);
    }

    /// @notice Cross-implementation golden value shared with the offchain prover
    ///         (`ScheduleId::from_upgrades` in base's `crates/proof/proof/src/schedule_id.rs`):
    ///         ids 0 and 1 scheduled at 10 and 20, ids 2-9 unscheduled, id 10 scheduled at 30.
    ///         Both sides MUST derive the same commitment for this schedule.
    function test_scheduleId_matchesProverGoldenValue_succeeds() external {
        // registerUpgrade does not enforce the notice window, so historical timestamps are fine.
        vm.startPrank(_owner);
        protocolVersions.registerUpgrade(10, 0);
        protocolVersions.registerUpgrade(20, 0);
        for (uint256 i = 0; i < 8; i++) {
            protocolVersions.registerUpgrade(0, 0);
        }
        protocolVersions.registerUpgrade(30, 0);
        vm.stopPrank();

        assertEq(protocolVersions.scheduleId(), 0xa24ace1024856cce0f999daface9269cbe7c1a8d1069a0e75196635146ee2058);
    }

    /// @notice Cross-implementation golden value for the Base mainnet schedule as of Beryl,
    ///         shared with the offchain prover (`schedule_id_matches_mainnet_golden_value` in
    ///         base's `crates/proof/proof/src/schedule_id.rs`). Ids follow the node's
    ///         CONTRACT_VARIANTS registration order. The genesis-active regolith (timestamp 0),
    ///         the mainnet pectra_blob_schedule gap, and the unscheduled cobalt tail all
    ///         contribute no link.
    function test_scheduleId_matchesMainnetGoldenValue_succeeds() external {
        uint64[13] memory mainnetSchedule = [
            uint64(0), // regolith: genesis-active, encoded as 0 => no link
            1_704_992_401, // canyon
            1_708_560_000, // delta
            1_710_374_401, // ecotone
            1_720_627_201, // fjord
            1_726_070_401, // granite
            1_736_445_601, // holocene
            0, // pectra_blob_schedule: never activates on mainnet => no link
            1_746_806_401, // isthmus
            1_764_691_201, // jovian
            1_779_991_200, // azul
            1_782_410_400, // beryl
            0 // cobalt: not yet scheduled => no link
        ];

        vm.startPrank(_owner);
        for (uint256 i = 0; i < mainnetSchedule.length; i++) {
            protocolVersions.registerUpgrade(mainnetSchedule[i], 0);
        }
        vm.stopPrank();

        assertEq(protocolVersions.scheduleId(), 0xe7ed922ecb2a9d7704cf21e21c62313eabe90f345c212cad1a4706633dcf4efd);
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

    /// @notice Tests that `scheduleId(id)` returns each upgrade's cumulative commitment: scheduled
    ///         upgrades add a link, unscheduled upgrades carry the previous commitment forward, and
    ///         the last upgrade's commitment equals the current scheduleId.
    function test_scheduleId_byId_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0, 0);

        // Both unscheduled: each per-id commitment is the carried-forward bytes32(0) seed.
        assertEq(protocolVersions.scheduleId(CANYON), protocolVersions.scheduleId(ECOTONE));
        assertEq(protocolVersions.scheduleId(ECOTONE), bytes32(0));

        // Scheduling CANYON adds its link; the unscheduled ECOTONE carries it forward as the tail.
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        assertNotEq(protocolVersions.scheduleId(CANYON), bytes32(0));
        assertEq(protocolVersions.scheduleId(ECOTONE), protocolVersions.scheduleId(CANYON));
        assertEq(protocolVersions.scheduleId(ECOTONE), protocolVersions.scheduleId());
    }

    /// @notice Tests that `scheduleId(id)` reverts for an unregistered upgrade.
    function test_scheduleId_byId_unregistered_reverts() external {
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgrade.selector, uint256(0)));
        protocolVersions.scheduleId(0);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { Test } from "lib/forge-std/src/Test.sol";

// Contracts
import { ProtocolVersions } from "src/L1/ProtocolVersions.sol";
import { ProxyAdminOwnedBase } from "src/universal/ProxyAdminOwnedBase.sol";
import { Proxy } from "src/universal/Proxy.sol";

// Interfaces
import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";
import { IProxyAdminOwnedBase } from "interfaces/L1/IProxyAdminOwnedBase.sol";

/// @title ProtocolVersions_TestInit
/// @notice Reusable test initialization for ProtocolVersions tests.
abstract contract ProtocolVersions_TestInit is Test {
    event UpgradeRegistered(uint256 indexed id, uint256 protocolVersion);
    event LatestProtocolVersionUpdated(uint256 indexed protocolVersion);
    event ChainTeamUpdated(address indexed previousChainTeam, address indexed newChainTeam);
    event TimestampSet(uint256 indexed id, uint256 timestamp);

    /// @dev Ascending ids assigned by registration order in these tests.
    uint256 internal constant CANYON = 0;
    uint256 internal constant ECOTONE = 1;
    uint256 internal constant L2_CHAIN_ID = 8453;

    address internal _owner = makeAddr("owner");
    address internal _nonOwner = makeAddr("non-owner");
    address internal _chainTeam = makeAddr("chain-team");
    address internal _proxyAdmin = makeAddr("proxy-admin");
    ProtocolVersions internal _impl;
    ProtocolVersions internal protocolVersions;

    function setUp() public virtual {
        // proxyAdminOwner() resolves by calling owner() on the ProxyAdmin stored in the proxy slot.
        vm.mockCall(_proxyAdmin, abi.encodeWithSignature("owner()"), abi.encode(_owner));
        _impl = new ProtocolVersions();
        protocolVersions = ProtocolVersions(_deployInitializedProxy(L2_CHAIN_ID));
    }

    /// @dev Deploys a proxy (admin = _proxyAdmin) over the shared impl and initializes it via the proxy.
    function _deployInitializedProxy(uint256 _l2ChainId) internal returns (address proxy_) {
        Proxy proxy = new Proxy(_proxyAdmin);
        vm.prank(_proxyAdmin);
        proxy.upgradeToAndCall(address(_impl), abi.encodeCall(IProtocolVersions.initialize, (_l2ChainId)));
        proxy_ = address(proxy);
    }

    /// @dev Deploys an uninitialized proxy over the shared impl (for initializer tests).
    function _deployUninitializedProxy() internal returns (ProtocolVersions) {
        Proxy proxy = new Proxy(_proxyAdmin);
        vm.prank(_proxyAdmin);
        proxy.upgradeTo(address(_impl));
        return ProtocolVersions(address(proxy));
    }

    /// @dev Registers the first upgrade (id CANYON) and schedules it for block.timestamp + MIN_NOTICE + delay.
    function _scheduleCanyon(uint64 _delay) internal returns (uint64 ts_) {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);
        ts_ = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + _delay;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts_);
    }
}

/// @title ProtocolVersions_Initialize_Test
/// @notice Test contract for the ProtocolVersions initializer.
contract ProtocolVersions_Initialize_Test is ProtocolVersions_TestInit {
    /// @notice Tests that initialization sets the correct initial state.
    function test_initialize_setsInitialState_succeeds() external view {
        assertEq(protocolVersions.proxyAdminOwner(), _owner);
        assertNotEq(protocolVersions.scheduleId(), bytes32(0));
    }

    /// @notice Tests that the scheduleId seed binds to the proxy address, not the implementation.
    function test_initialize_seedBindsToProxyAddress_succeeds() external view {
        bytes32 expectedSeed = keccak256(abi.encode(L2_CHAIN_ID, address(protocolVersions)));
        assertEq(protocolVersions.scheduleId(), expectedSeed);
    }

    /// @notice Tests that initializing with a zero chain ID reverts.
    function test_initialize_zeroChainId_reverts() external {
        ProtocolVersions uninitialized = _deployUninitializedProxy();
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidL2ChainId.selector);
        vm.prank(_proxyAdmin);
        uninitialized.initialize(0);
    }

    /// @notice Tests that only the ProxyAdmin or its owner can initialize.
    function test_initialize_notProxyAdminOrOwner_reverts() external {
        ProtocolVersions uninitialized = _deployUninitializedProxy();
        vm.expectRevert(IProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        uninitialized.initialize(L2_CHAIN_ID);
    }

    /// @notice Tests that the contract cannot be initialized twice.
    function test_initialize_alreadyInitialized_reverts() external {
        vm.expectRevert("Initializable: contract is already initialized");
        vm.prank(_proxyAdmin);
        protocolVersions.initialize(L2_CHAIN_ID);
    }

    /// @notice Tests that the implementation itself cannot be initialized (initializers disabled).
    function test_initialize_implementationDisabled_reverts() external {
        vm.expectRevert("Initializable: contract is already initialized");
        vm.prank(_proxyAdmin);
        _impl.initialize(L2_CHAIN_ID);
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
        protocolVersions.registerUpgrade(1);

        assertNotEq(protocolVersions.scheduleId(), idBefore);
    }

    /// @notice Tests that `registerUpgrade` assigns ascending ids and returns them.
    function test_registerUpgrade_returnsAscendingIds_succeeds() external {
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(1), 0);
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(2), 1);
        vm.prank(_owner);
        assertEq(protocolVersions.registerUpgrade(3), 2);
    }

    /// @notice Tests that `registerUpgrade` emits the `UpgradeRegistered` event with correct fields.
    function test_registerUpgrade_emitsEvent_succeeds() external {
        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit UpgradeRegistered(0, 1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit UpgradeRegistered(1, 2);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(2);
    }

    /// @notice Tests that registering upgrades updates latestProtocolVersion to the most recent.
    function test_registerUpgrade_updatesLatestProtocolVersion_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(5);
        assertEq(protocolVersions.latestProtocolVersion(), 5);

        vm.prank(_owner);
        protocolVersions.registerUpgrade(9);
        assertEq(protocolVersions.latestProtocolVersion(), 9);
    }

    /// @notice Tests that registering an upgrade with a zero protocolVersion reverts.
    function test_registerUpgrade_zeroProtocolVersion_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidProtocolVersion.selector);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(0);
    }

    /// @notice Tests that only the owner can call `registerUpgrade`.
    function test_registerUpgrade_callerNotOwner_reverts() external {
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.registerUpgrade(1);
    }
}

/// @title ProtocolVersions_SetLatestProtocolVersion_Test
/// @notice Test contract for the `setLatestProtocolVersion` function.
contract ProtocolVersions_SetLatestProtocolVersion_Test is ProtocolVersions_TestInit {
    /// @notice Tests that the owner can set the latest protocol version directly.
    function test_setLatestProtocolVersion_updates_succeeds() external {
        vm.prank(_owner);
        protocolVersions.setLatestProtocolVersion(42);
        assertEq(protocolVersions.latestProtocolVersion(), 42);
    }

    /// @notice Tests that setting the latest protocol version does not change the scheduleId.
    function test_setLatestProtocolVersion_doesNotChangeScheduleId_succeeds() external {
        bytes32 scheduleIdBefore = protocolVersions.scheduleId();
        vm.prank(_owner);
        protocolVersions.setLatestProtocolVersion(42);
        assertEq(protocolVersions.scheduleId(), scheduleIdBefore);
    }

    /// @notice Tests that `setLatestProtocolVersion` emits the `LatestProtocolVersionUpdated` event.
    function test_setLatestProtocolVersion_emitsEvent_succeeds() external {
        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit LatestProtocolVersionUpdated(42);
        vm.prank(_owner);
        protocolVersions.setLatestProtocolVersion(42);
    }

    /// @notice Tests that setting a zero protocol version reverts.
    function test_setLatestProtocolVersion_zero_reverts() external {
        vm.expectRevert(IProtocolVersions.ProtocolVersions_InvalidProtocolVersion.selector);
        vm.prank(_owner);
        protocolVersions.setLatestProtocolVersion(0);
    }

    /// @notice Tests that only the owner can call `setLatestProtocolVersion`.
    function test_setLatestProtocolVersion_callerNotOwner_reverts() external {
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.setLatestProtocolVersion(42);
    }
}

/// @title ProtocolVersions_SetTimestamp_Test
/// @notice Test contract for the `setTimestamp` function.
contract ProtocolVersions_SetTimestamp_Test is ProtocolVersions_TestInit {
    /// @notice Tests that setting a timestamp updates the stored value and extends the scheduleId.
    function test_setTimestamp_updatesTimestampAndScheduleId_succeeds() external {
        bytes32 initialScheduleId = protocolVersions.scheduleId();
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        assertEq(protocolVersions.getSchedule()[CANYON].timestamp, ts);
        assertNotEq(protocolVersions.scheduleId(), initialScheduleId);
    }

    /// @notice Tests that calling `setTimestamp` with the same value is a no-op for scheduleId.
    function test_setTimestamp_sameTimestamp_noScheduleIdChange_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        bytes32 scheduleIdAfterSet = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        assertEq(protocolVersions.getSchedule()[CANYON].timestamp, ts);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterSet);
    }

    /// @notice Tests that passing 0 clears a scheduled timestamp, changes the scheduleId, and
    ///         restores it to the value it held immediately after registration (ts=0 link).
    function test_setTimestamp_clearTimestamp_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);
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

        assertEq(protocolVersions.getSchedule()[CANYON].timestamp, 0);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterRegister);
    }

    /// @notice Tests that `setTimestamp` emits a `TimestampSet` event.
    function test_setTimestamp_emitsEvent_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectEmit(true, false, false, true, address(protocolVersions));
        emit TimestampSet(CANYON, ts);
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that only the owner can call `setTimestamp`.
    function test_setTimestamp_callerNotOwner_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(ProxyAdminOwnedBase.ProxyAdminOwnedBase_NotProxyAdminOwner.selector);
        vm.prank(_nonOwner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is in the past.
    function test_setTimestamp_timestampInPast_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

        vm.warp(1000);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, uint64(0), uint64(500))
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, 500);
    }

    /// @notice Tests that `setTimestamp` reverts when the timestamp is within MIN_NOTICE of now.
    function test_setTimestamp_insufficientNotice_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() - 1;
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, uint64(0), ts)
        );
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);
    }

    /// @notice Tests that `setTimestamp` reverts when the upgrade has already activated.
    function test_setTimestamp_afterActivation_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);

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

    /// @notice Tests that scheduleId is reproducible from (chainId, address, ascending ids, timestamps).
    function test_setTimestamp_scheduleIdReproducible_succeeds() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(2);

        uint64 ts1 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        uint64 ts2 = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 200;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts1);
        vm.prank(_owner);
        protocolVersions.setTimestamp(ECOTONE, ts2);

        // Reproduce the chain from scratch.
        bytes32 seed = keccak256(abi.encode(uint256(8453), address(protocolVersions)));
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
        protocolVersions.setChainTeam(_chainTeam);

        bytes32 scheduleIdBefore = protocolVersions.scheduleId();
        vm.roll(block.number + 1);

        uint64 later = ts + 50;
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp(CANYON, later);

        assertEq(protocolVersions.getSchedule()[CANYON].timestamp, later);
        assertNotEq(protocolVersions.scheduleId(), scheduleIdBefore);
    }

    /// @notice Tests that only the chainTeam can call `delayTimestamp`.
    function test_delayTimestamp_callerNotChainTeam_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.expectRevert(IProtocolVersions.ProtocolVersions_NotChainTeam.selector);
        vm.prank(_owner);
        protocolVersions.delayTimestamp(CANYON, ts + 50);

        vm.expectRevert(IProtocolVersions.ProtocolVersions_NotChainTeam.selector);
        vm.prank(_nonOwner);
        protocolVersions.delayTimestamp(CANYON, ts + 50);
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
        protocolVersions.delayTimestamp(CANYON, ts - 10);
    }

    /// @notice Tests that `delayTimestamp` reverts when the new timestamp equals the current one.
    function test_delayTimestamp_equalTimestamp_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, ts, ts));
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp(CANYON, ts);
    }

    /// @notice Tests that `delayTimestamp` reverts when the upgrade has no scheduled timestamp.
    function test_delayTimestamp_notScheduled_reverts() external {
        vm.prank(_owner);
        protocolVersions.registerUpgrade(1);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_NotScheduled.selector, CANYON));
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp(CANYON, ts);
    }

    /// @notice Tests that `delayTimestamp` reverts when the upgrade has already activated.
    function test_delayTimestamp_afterActivation_reverts() external {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        vm.warp(ts + 1);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, CANYON, ts)
        );
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp(CANYON, ts + 100);
    }

    /// @notice Tests that `delayTimestamp` reverts for an unregistered upgrade.
    function test_delayTimestamp_unregisteredUpgrade_reverts() external {
        vm.prank(_owner);
        protocolVersions.setChainTeam(_chainTeam);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.expectRevert(abi.encodeWithSelector(IProtocolVersions.ProtocolVersions_UnknownUpgrade.selector, uint256(0)));
        vm.prank(_chainTeam);
        protocolVersions.delayTimestamp(0, ts);
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
        protocolVersions.registerUpgrade(1);
        vm.prank(_owner);
        protocolVersions.registerUpgrade(2);

        uint64 ts = uint64(block.timestamp) + protocolVersions.MIN_NOTICE() + 100;
        vm.prank(_owner);
        protocolVersions.setTimestamp(CANYON, ts);

        ProtocolVersions.Upgrade[] memory s = protocolVersions.getSchedule();

        assertEq(s.length, 2);

        assertEq(s[0].id, CANYON);
        assertEq(s[0].timestamp, ts);

        assertEq(s[1].id, ECOTONE);
        assertEq(s[1].timestamp, 0);

        // The last entry's scheduleId is the contract's current scheduleId.
        assertEq(s[1].scheduleId, protocolVersions.scheduleId());
    }
}

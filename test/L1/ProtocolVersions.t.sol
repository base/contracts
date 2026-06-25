// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { ProtocolVersions } from "src/L1/ProtocolVersions.sol";

contract ProtocolVersions_Test is Test {
    /// @dev Mirror of ProtocolVersions.UpgradeRegistered for use with vm.expectEmit.
    event UpgradeRegistered(bytes32 indexed key, uint256 indexed index, string upgradeId, uint256 protocolVersion);

    address internal owner = makeAddr("owner");
    address internal nonOwner = makeAddr("non-owner");
    address internal chainTeam = makeAddr("chain-team");
    ProtocolVersions internal protocolVersions;

    function setUp() public {
        protocolVersions = new ProtocolVersions(owner, 8453);
    }

    /// @dev Registers `canyon` and schedules it for `block.timestamp + delay`, returning that timestamp.
    function _scheduleCanyon(uint64 delay) internal returns (uint64 ts) {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);
        ts = uint64(block.timestamp + delay);
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", ts);
    }

    function testSetSignalUpdatesTimestampAndScheduleId() public {
        bytes32 initialScheduleId = protocolVersions.scheduleId();

        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp + 100);
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", ts);

        assertEq(protocolVersions.getTimestamp("canyon"), ts);
        assertEq(protocolVersions.lastUpdatedAtBlock(), block.number);
        assertNotEq(protocolVersions.scheduleId(), initialScheduleId);
    }

    function testSameTimestampDoesNotChangeScheduleId() public {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        uint64 ts = uint64(block.timestamp + 100);
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", ts);

        bytes32 scheduleIdAfterSet = protocolVersions.scheduleId();
        uint256 updateBlockAfterSet = protocolVersions.lastUpdatedAtBlock();

        vm.roll(block.number + 1);
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", ts);

        assertEq(protocolVersions.getTimestamp("canyon"), ts);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterSet);
        assertEq(protocolVersions.lastUpdatedAtBlock(), updateBlockAfterSet);
    }

    function testClearingSignalResetsTimestamp() public {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);
        bytes32 scheduleIdAfterRegister = protocolVersions.scheduleId();

        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 1);
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", uint64(block.timestamp + 100));

        vm.roll(block.number + 1);
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", 0);

        assertEq(protocolVersions.getTimestamp("canyon"), 0);
        assertEq(protocolVersions.scheduleId(), scheduleIdAfterRegister);
    }

    function testOnlyOwnerCanUpdateSignals() public {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(nonOwner);
        protocolVersions.setTimestamp("canyon", uint64(block.timestamp + 100));
    }

    function testSetSignalRejectsTimestampInPast() public {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.warp(1000);
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolVersions.ProtocolVersions_ActivationTimestampInPast.selector, uint64(500))
        );
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", 500);
    }

    function testSetSignalRejectsChangeAfterActivation() public {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.warp(100);
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", 200);

        vm.warp(300);
        bytes32 canyonId = bytes32("canyon");
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, canyonId, uint64(200)
            )
        );
        vm.prank(owner);
        protocolVersions.setTimestamp("canyon", 400);
    }

    function testGetTimestampRejectsUnknownUpgradeName() public {
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolVersions.ProtocolVersions_UnknownUpgradeName.selector, "unknown")
        );
        protocolVersions.getTimestamp("unknown");
    }

    function testRegistryStartsEmpty() public view {
        assertEq(protocolVersions.upgradeCount(), 0);
        assertEq(protocolVersions.upgradeIds().length, 0);
    }

    function testRegisterUpgradeExtendsScheduleAndChangesScheduleId() public {
        bytes32 initialScheduleId = protocolVersions.scheduleId();

        assertEq(protocolVersions.upgradeCount(), 0);

        vm.roll(block.number + 1);
        vm.prank(owner);
        protocolVersions.registerUpgrade("antares", 1);

        assertEq(protocolVersions.upgradeCount(), 1);
        assertEq(protocolVersions.upgradeIdAt(0), bytes32("antares"));
        assertNotEq(protocolVersions.scheduleId(), initialScheduleId);
        assertEq(protocolVersions.lastUpdatedAtBlock(), block.number);

        vm.warp(block.timestamp + 1);
        vm.roll(block.number + 1);
        vm.prank(owner);
        protocolVersions.setTimestamp("antares", uint64(block.timestamp + 100));
        assertEq(protocolVersions.getTimestamp("antares"), block.timestamp + 100);
    }

    function testUpgradeRegisteredEventEmitsKeyIndexNameAndVersion() public {
        // First registration: index 0, with its key, id string, and owner-chosen version.
        vm.expectEmit(true, true, true, true, address(protocolVersions));
        emit UpgradeRegistered(bytes32("canyon"), 0, "canyon", 1);
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);

        // Second registration: index increments to 1.
        vm.expectEmit(true, true, true, true, address(protocolVersions));
        emit UpgradeRegistered(bytes32("ecotone"), 1, "ecotone", 2);
        vm.prank(owner);
        protocolVersions.registerUpgrade("ecotone", 2);
    }

    function testRegisterUpgradeRejectsDuplicate() public {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);

        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolVersions.ProtocolVersions_UpgradeAlreadyRegistered.selector, bytes32("canyon")
            )
        );
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);
    }

    function testRegisterUpgradeRejectsEmptyName() public {
        vm.expectRevert(ProtocolVersions.ProtocolVersions_InvalidUpgradeId.selector);
        vm.prank(owner);
        protocolVersions.registerUpgrade("", 1);
    }

    function testRegisterUpgradeRejectsNameLongerThan32Bytes() public {
        vm.expectRevert(ProtocolVersions.ProtocolVersions_InvalidUpgradeId.selector);
        vm.prank(owner);
        protocolVersions.registerUpgrade("this-upgrade-name-is-definitely-too-long", 1);
    }

    function testOnlyOwnerCanRegisterUpgrade() public {
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(nonOwner);
        protocolVersions.registerUpgrade("antares", 1);
    }

    function testSetSignalRejectsUnregisteredUpgrade() public {
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolVersions.ProtocolVersions_UnknownUpgradeName.selector, "antares")
        );
        vm.prank(owner);
        protocolVersions.setTimestamp("antares", uint64(block.timestamp + 100));
    }

    function testRegisterUpgradeStoresOwnerChosenProtocolVersion() public {
        // Versions are owner-chosen at registration, not derived from registration order.
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 5);
        vm.prank(owner);
        protocolVersions.registerUpgrade("ecotone", 9);

        assertEq(protocolVersions.getProtocolVersion("canyon"), 5);
        assertEq(protocolVersions.getProtocolVersion("ecotone"), 9);
    }

    function testGetProtocolVersionRejectsUnknownUpgradeName() public {
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolVersions.ProtocolVersions_UnknownUpgradeName.selector, "unknown")
        );
        protocolVersions.getProtocolVersion("unknown");
    }

    function testChainTeamStartsUnset() public view {
        assertEq(protocolVersions.chainTeam(), address(0));
    }

    function testOwnerCanSetChainTeam() public {
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);
        assertEq(protocolVersions.chainTeam(), chainTeam);
    }

    function testOnlyOwnerCanSetChainTeam() public {
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(nonOwner);
        protocolVersions.setChainTeam(chainTeam);
    }

    function testDelaySignalPushesTimestampLater() public {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);

        bytes32 scheduleIdBefore = protocolVersions.scheduleId();
        vm.roll(block.number + 1);

        uint64 later = ts + 50;
        vm.prank(chainTeam);
        protocolVersions.delayTimestamp("canyon", later);

        assertEq(protocolVersions.getTimestamp("canyon"), later);
        assertNotEq(protocolVersions.scheduleId(), scheduleIdBefore);
        assertEq(protocolVersions.lastUpdatedAtBlock(), block.number);
    }

    function testOnlyChainTeamCanDelaySignal() public {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);

        // Not even the owner may call delayTimestamp — it is chainTeam-only.
        vm.expectRevert(ProtocolVersions.ProtocolVersions_NotChainTeam.selector);
        vm.prank(owner);
        protocolVersions.delayTimestamp("canyon", ts + 50);

        vm.expectRevert(ProtocolVersions.ProtocolVersions_NotChainTeam.selector);
        vm.prank(nonOwner);
        protocolVersions.delayTimestamp("canyon", ts + 50);
    }

    function testDelaySignalRejectsEarlierTimestamp() public {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);

        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, ts, ts - 10
            )
        );
        vm.prank(chainTeam);
        protocolVersions.delayTimestamp("canyon", ts - 10);
    }

    function testDelaySignalRejectsEqualTimestamp() public {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);

        vm.expectRevert(
            abi.encodeWithSelector(ProtocolVersions.ProtocolVersions_DelayMustBeLater.selector, ts, ts)
        );
        vm.prank(chainTeam);
        protocolVersions.delayTimestamp("canyon", ts);
    }

    function testDelaySignalRejectsUnscheduledUpgrade() public {
        vm.prank(owner);
        protocolVersions.registerUpgrade("canyon", 1);
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);

        vm.expectRevert(
            abi.encodeWithSelector(ProtocolVersions.ProtocolVersions_NotScheduled.selector, bytes32("canyon"))
        );
        vm.prank(chainTeam);
        protocolVersions.delayTimestamp("canyon", uint64(block.timestamp + 100));
    }

    function testDelaySignalRejectsAfterActivation() public {
        uint64 ts = _scheduleCanyon(100);
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);

        vm.warp(ts + 1);
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolVersions.ProtocolVersions_ActivationAlreadyPassed.selector, bytes32("canyon"), ts
            )
        );
        vm.prank(chainTeam);
        protocolVersions.delayTimestamp("canyon", ts + 100);
    }

    function testDelaySignalRejectsUnregisteredUpgrade() public {
        vm.prank(owner);
        protocolVersions.setChainTeam(chainTeam);

        vm.expectRevert(
            abi.encodeWithSelector(ProtocolVersions.ProtocolVersions_UnknownUpgradeName.selector, "antares")
        );
        vm.prank(chainTeam);
        protocolVersions.delayTimestamp("antares", uint64(block.timestamp + 100));
    }
}

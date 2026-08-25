// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { IProtocolVersions } from "interfaces/L1/IProtocolVersions.sol";
import { DeployConfig } from "scripts/deploy/DeployConfig.s.sol";
import { ProtocolVersionsConfig } from "src/libraries/ProtocolVersionsConfig.sol";

contract DeployConfigHarness is DeployConfig {
    function readSchedule(string memory _json) public {
        _readProtocolVersionsInitialSchedule(_json);
    }
}

contract DeployConfig_Test is Test {
    DeployConfigHarness internal cfg;

    function setUp() public {
        cfg = new DeployConfigHarness();
    }

    /// @notice The imported schedule is the only way a chain with existing hardfork history reaches
    ///         `ProtocolVersions.initialize`, so a key that silently fails to parse would leave that
    ///         chain permanently stuck with an empty registry.
    function test_readSchedule_parsesTimestampsInOrder_succeeds() public {
        cfg.readSchedule('{"protocolVersionsInitialSchedule":[1686789347,1704992401,0,1710374401,0,0,0,0,0,0,0,0,0,0]}');

        uint64[] memory schedule = cfg.protocolVersionsInitialSchedule();
        assertEq(schedule.length, ProtocolVersionsConfig.INITIAL_UPGRADE_COUNT);
        assertEq(schedule[0], 1_686_789_347);
        assertEq(schedule[1], 1_704_992_401);
        assertEq(schedule[2], 0);
        assertEq(schedule[3], 1_710_374_401);
    }

    function test_readSchedule_omitted_defaultsToEmpty_succeeds() public {
        cfg.readSchedule('{"l1ChainId":1}');

        assertEq(cfg.protocolVersionsInitialSchedule().length, 0);
    }

    /// @dev Config state is reused across reads, so a second read must replace rather than append.
    function test_readSchedule_twice_replacesSchedule_succeeds() public {
        cfg.readSchedule('{"protocolVersionsInitialSchedule":[1686789347,1704992401,0,0,0,0,0,0,0,0,0,0,0,0]}');
        cfg.readSchedule('{"protocolVersionsInitialSchedule":[1710374401,0,0,0,0,0,0,0,0,0,0,0,0,0]}');

        uint64[] memory schedule = cfg.protocolVersionsInitialSchedule();
        assertEq(schedule.length, ProtocolVersionsConfig.INITIAL_UPGRADE_COUNT);
        assertEq(schedule[0], 1_710_374_401);
        assertEq(schedule[1], 0);
    }

    function test_readSchedule_truncatedSchedule_reverts() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolVersions.ProtocolVersions_InvalidInitialScheduleLength.selector,
                uint256(4),
                ProtocolVersionsConfig.INITIAL_UPGRADE_COUNT
            )
        );
        cfg.readSchedule('{"protocolVersionsInitialSchedule":[1686789347,1704992401,1708560000,1710374401]}');
    }

    function test_readSchedule_timestampAboveUint64_reverts() public {
        vm.expectRevert("DeployConfig: initial schedule timestamp exceeds uint64");
        cfg.readSchedule('{"protocolVersionsInitialSchedule":[18446744073709551616]}');
    }

    /// @notice The shipped configs describe chains without a recorded history, so they must keep
    ///         producing an empty registry.
    function test_read_localConfig_leavesScheduleEmpty_succeeds() public {
        cfg.read("deploy-config/local.json");

        assertEq(cfg.protocolVersionsInitialSchedule().length, 0);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { Test } from "lib/forge-std/src/Test.sol";

// Contracts
import { ReinitializableBase } from "src/universal/ReinitializableBase.sol";

contract ReinitializableBase_Harness is ReinitializableBase {
    constructor(uint8 _initVersion) ReinitializableBase(_initVersion) { }
}

contract ReinitializableBase_Constructor_Test is Test {
    function test_constructor_zeroVersion_reverts() external {
        vm.expectRevert(ReinitializableBase.ReinitializableBase_ZeroInitVersion.selector);
        new ReinitializableBase_Harness(0);
    }

    function testFuzz_constructor_validVersion_succeeds(uint8 _initVersion) external {
        _initVersion = uint8(bound(_initVersion, 1, type(uint8).max));
        assertEq(new ReinitializableBase_Harness(_initVersion).initVersion(), _initVersion);
    }
}

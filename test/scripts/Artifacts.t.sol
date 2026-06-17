// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "lib/forge-std/src/Test.sol";
import { Artifacts } from "scripts/Artifacts.s.sol";

/// @title Artifacts_load_Test
/// @notice Covers the re-entrant (fresh-process) path: a script run such as
///         `registerAggregateVerifier` after L2 genesis must reload addresses written by an
///         earlier deploy, and a subsequent `save()` must append to rather than clobber the file.
contract Artifacts_load_Test is Test {
    Artifacts internal artifacts;
    string internal outfile;

    address internal constant FOO = address(0x1111111111111111111111111111111111111111);
    address internal constant BAR = address(0x2222222222222222222222222222222222222222);
    address internal constant BAZ = address(0x3333333333333333333333333333333333333333);

    function setUp() public {
        outfile = string.concat(vm.projectRoot(), "/deployments/artifacts-load-test.json");
        try vm.removeFile(outfile) { } catch { }
        vm.setEnv("DEPLOYMENT_OUTFILE", outfile);
        artifacts = new Artifacts();
        artifacts.setUp();
    }

    /// @dev Without load(), getAddress only knows predeploys, so reading a prior deployment would
    ///      revert; and save() does a whole-file writeJson, so it must not clobber existing keys.
    function test_load_reloadsThenAppends_succeeds() public {
        // Simulate an earlier deploy process having written the outfile.
        vm.writeFile(outfile, string.concat('{"Foo":"', vm.toString(FOO), '","Bar":"', vm.toString(BAR), '"}'));

        artifacts.load();

        assertEq(artifacts.mustGetAddress("Foo"), payable(FOO));
        assertEq(artifacts.mustGetAddress("Bar"), payable(BAR));

        artifacts.save("Baz", BAZ);

        string memory json = vm.readFile(outfile);
        assertEq(vm.parseJsonAddress(json, ".Foo"), FOO);
        assertEq(vm.parseJsonAddress(json, ".Bar"), BAR);
        assertEq(vm.parseJsonAddress(json, ".Baz"), BAZ);
        assertEq(vm.parseJsonKeys(json, "$").length, 3);

        vm.removeFile(outfile);
    }
}

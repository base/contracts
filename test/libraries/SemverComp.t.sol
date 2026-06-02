// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { JSONParserLib } from "lib/solady/src/utils/JSONParserLib.sol";
import { SemverComp } from "src/libraries/SemverComp.sol";

contract SemverComp_Harness {
    function parse(string memory _semver) external pure returns (uint256 major_, uint256 minor_, uint256 patch_) {
        SemverComp.Semver memory v = SemverComp.parse(_semver);
        return (v.major, v.minor, v.patch);
    }
}

abstract contract SemverComp_TestInit is Test {
    SemverComp_Harness internal harness;

    function setUp() public {
        harness = new SemverComp_Harness();
    }

    function assertParsedEq(string memory _semver, uint256 _major, uint256 _minor, uint256 _patch) internal view {
        (uint256 major, uint256 minor, uint256 patch) = harness.parse(_semver);
        assertEq(major, _major, "major mismatch");
        assertEq(minor, _minor, "minor mismatch");
        assertEq(patch, _patch, "patch mismatch");
    }

    function assertParseReverts(string memory _semver, bytes4 _selector) internal {
        vm.expectRevert(_selector);
        harness.parse(_semver);
    }
}

contract SemverComp_parse_Test is SemverComp_TestInit {
    function test_parse_basicZero_succeeds() external view {
        assertParsedEq("0.0.0", 0, 0, 0);
    }

    function test_parse_basic123_succeeds() external view {
        assertParsedEq("1.2.3", 1, 2, 3);
    }

    function test_parse_withPrerelease_succeeds() external view {
        assertParsedEq("1.2.3-alpha", 1, 2, 3);
        assertParsedEq("1.2.3-alpha.1", 1, 2, 3);
        assertParsedEq("10.20.30-rc.1", 10, 20, 30);
    }

    function test_parse_withBuildMetadataOnly_succeeds() external view {
        assertParsedEq("1.2.3+build.5", 1, 2, 3);
        assertParsedEq("1.2.3+20240101", 1, 2, 3);
    }

    function test_parse_withPrereleaseAndBuild_succeeds() external view {
        assertParsedEq("1.2.3-rc.1+build.5", 1, 2, 3);
        assertParsedEq("2.0.0-beta+exp.sha.5114f85", 2, 0, 0);
    }

    function test_parse_lessThanThreeParts_reverts() external {
        assertParseReverts("1.2", SemverComp.SemverComp_InvalidSemverParts.selector);
        assertParseReverts("1", SemverComp.SemverComp_InvalidSemverParts.selector);
        assertParseReverts("", SemverComp.SemverComp_InvalidSemverParts.selector);
    }

    function test_parse_extraDotComponents_succeeds() external view {
        assertParsedEq("1.2.3.4", 1, 2, 3);
        assertParsedEq("1.2.3.4.5", 1, 2, 3);
    }

    function test_parse_nonNumeric_reverts() external {
        assertParseReverts("a.b.c", JSONParserLib.ParsingFailed.selector);
        assertParseReverts("1.b.3", JSONParserLib.ParsingFailed.selector);
        assertParseReverts("1.2.c", JSONParserLib.ParsingFailed.selector);
    }

    function test_parse_malformedInputs_reverts() external {
        assertParseReverts(" 1.2.3", JSONParserLib.ParsingFailed.selector);
        assertParseReverts("1.2.3 ", JSONParserLib.ParsingFailed.selector);
        assertParseReverts("v1.2.3", JSONParserLib.ParsingFailed.selector);
    }
}

contract SemverComp_Eq_Test is Test {
    function test_eq_succeeds() external pure {
        assertTrue(SemverComp.eq("1.2.3", "1.2.3"));

        assertFalse(SemverComp.eq("1.2.3", "1.2.4"));
        assertFalse(SemverComp.eq("1.2.3", "1.3.3"));
        assertFalse(SemverComp.eq("1.2.3", "2.2.3"));
    }
}

contract SemverComp_Lt_Test is Test {
    function test_lt_succeeds() external pure {
        assertTrue(SemverComp.lt("1.2.3", "1.2.4"));
        assertTrue(SemverComp.lt("1.2.3", "1.3.0"));
        assertTrue(SemverComp.lt("1.2.3", "2.0.0"));

        assertFalse(SemverComp.lt("1.2.3", "1.2.3"));
        assertFalse(SemverComp.lt("1.2.3", "1.2.2"));
        assertFalse(SemverComp.lt("2.0.0", "1.9.9"));
    }
}

contract SemverComp_Lte_Test is Test {
    function test_lte_succeeds() external pure {
        assertTrue(SemverComp.lte("1.2.3", "1.2.3"));
        assertTrue(SemverComp.lte("1.2.3", "1.2.4"));
        assertTrue(SemverComp.lte("1.2.3", "1.3.0"));
        assertTrue(SemverComp.lte("1.2.3", "2.0.0"));

        assertFalse(SemverComp.lte("1.2.3", "1.2.2"));
        assertFalse(SemverComp.lte("2.0.0", "1.9.9"));
    }
}

contract SemverComp_Gt_Test is Test {
    function test_gt_succeeds() external pure {
        assertTrue(SemverComp.gt("1.2.4", "1.2.3"));
        assertTrue(SemverComp.gt("1.3.0", "1.2.3"));
        assertTrue(SemverComp.gt("2.0.0", "1.2.3"));

        assertFalse(SemverComp.gt("1.2.3", "1.2.3"));
        assertFalse(SemverComp.gt("1.2.2", "1.2.3"));
        assertFalse(SemverComp.gt("1.9.9", "2.0.0"));
    }
}

contract SemverComp_Gte_Test is Test {
    function test_gte_succeeds() external pure {
        assertTrue(SemverComp.gte("1.2.3", "1.2.3"));
        assertTrue(SemverComp.gte("1.2.4", "1.2.3"));
        assertTrue(SemverComp.gte("1.3.0", "1.2.3"));
        assertTrue(SemverComp.gte("2.0.0", "1.2.3"));

        assertFalse(SemverComp.gte("1.2.2", "1.2.3"));
        assertFalse(SemverComp.gte("1.9.9", "2.0.0"));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CommonTest } from "test/setup/CommonTest.sol";
import { SemverComp } from "src/libraries/SemverComp.sol";
import { ISemver } from "interfaces/universal/ISemver.sol";

contract WETH_Version_Test is CommonTest {
    function test_version_validFormat_succeeds() external view {
        SemverComp.parse(ISemver(address(weth)).version());
    }
}

contract WETH_Name_Test is CommonTest {
    function testFuzz_name_succeeds(string memory _gasPayingTokenName) external {
        vm.assume(bytes(_gasPayingTokenName).length <= 128);
        vm.mockCall(address(l1Block), abi.encodeCall(l1Block.gasPayingTokenName, ()), abi.encode(_gasPayingTokenName));

        assertEq(weth.name(), string.concat("Wrapped ", _gasPayingTokenName));
    }

    function test_name_defaultGasPayingToken_succeeds() external view {
        assertEq(weth.name(), string.concat("Wrapped ", l1Block.gasPayingTokenName()));
    }
}

contract WETH_Symbol_Test is CommonTest {
    function testFuzz_symbol_succeeds(string memory _gasPayingTokenSymbol) external {
        vm.assume(bytes(_gasPayingTokenSymbol).length <= 128);
        vm.mockCall(
            address(l1Block), abi.encodeCall(l1Block.gasPayingTokenSymbol, ()), abi.encode(_gasPayingTokenSymbol)
        );

        assertEq(weth.symbol(), string.concat("W", _gasPayingTokenSymbol));
    }

    function test_symbol_defaultGasPayingToken_succeeds() external view {
        assertEq(weth.symbol(), string.concat("W", l1Block.gasPayingTokenSymbol()));
    }
}

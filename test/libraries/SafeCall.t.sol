// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Scripts
import { Config } from "scripts/libraries/Config.sol";

// Forge
import { Test } from "lib/forge-std/src/Test.sol";
import { VmSafe } from "lib/forge-std/src/Vm.sol";
import { StdCheatsSafe } from "lib/forge-std/src/StdCheats.sol";

// Libraries
import { LibString } from "lib/solady/src/utils/LibString.sol";
import { SafeCall } from "src/libraries/SafeCall.sol";

contract SimpleSafeCaller {
    uint256 public a;

    function makeSafeCall(uint64 gas, uint64 minGas) external returns (bool) {
        return SafeCall.call(address(this), gas, 0, abi.encodeCall(this.makeSafeCallMinGas, (minGas)));
    }

    function makeSafeCallMinGas(uint64 minGas) external returns (bool) {
        return SafeCall.callWithMinGas(address(this), minGas, 0, abi.encodeCall(this.setA, (1)));
    }

    function setA(uint256 _a) external {
        a = _a;
    }
}

/// @title SafeCall_TestInit
/// @notice Reusable test initialization for `SafeCall` tests.
abstract contract SafeCall_TestInit is Test {
    struct CallBalances {
        uint256 from;
        uint256 to;
    }

    /// @notice Makes all assumptions required for these tests.
    function assumeNot(address _addr) internal {
        vm.assume(_addr != address(this));
        assumeAddressIsNot(_addr, StdCheatsSafe.AddressType.ForgeAddress, StdCheatsSafe.AddressType.Precompile);
        vm.deal(_addr, 0);
    }

    function prepareCall(
        address _from,
        address _to,
        uint256 _value
    )
        internal
        returns (CallBalances memory balancesBefore)
    {
        assumeNot(_from);
        assumeNot(_to);

        assertEq(_from.balance, 0, "from balance is 0");
        vm.deal(_from, _value);
        assertEq(_from.balance, _value, "from balance not dealt");

        balancesBefore = CallBalances({ from: _from.balance, to: _to.balance });
    }

    function assertCallBalances(
        address _from,
        address _to,
        uint256 _value,
        CallBalances memory _balancesBefore
    )
        internal
        view
    {
        if (_from == _to) {
            assertEq(_from.balance, _balancesBefore.from, "self-transfer did not preserve balance");
        } else {
            assertEq(_from.balance, _balancesBefore.from - _value, "from balance not drained");
            assertEq(_to.balance, _balancesBefore.to + _value, "to balance received");
        }
    }

    /// @notice Internal helper function for `send` tests
    function sendTest(address _from, address _to, uint64 _gas, uint256 _value) internal {
        CallBalances memory balancesBefore = prepareCall(_from, _to, _value);

        vm.expectCall(_to, _value, bytes(""));
        vm.prank(_from);
        bool success;
        if (_gas == 0) {
            success = SafeCall.send({ _target: _to, _value: _value });
        } else {
            success = SafeCall.send({ _target: _to, _gas: _gas, _value: _value });
        }

        assertTrue(success, "send not successful");
        assertCallBalances(_from, _to, _value, balancesBefore);
    }
}

/// @title SafeCall_Send_Test
/// @notice Tests the `send` function of the `SafeCall` contract.
contract SafeCall_Send_Test is SafeCall_TestInit {
    /// @notice Tests that the `send` function succeeds.
    function testFuzz_send_succeeds(address _from, address _to, uint256 _value) external {
        sendTest({ _from: _from, _to: _to, _gas: 0, _value: _value });
    }

    /// @notice Tests that the `send` function with value succeeds.
    function testFuzz_send_withGas_succeeds(address _from, address _to, uint64 _gas, uint256 _value) external {
        _gas = uint64(bound(_gas, 1, type(uint64).max));
        sendTest({ _from: _from, _to: _to, _gas: _gas, _value: _value });
    }
}

/// @title SafeCall_Call_Test
/// @notice Tests the `call` function of the `SafeCall` contract.
contract SafeCall_Call_Test is SafeCall_TestInit {
    /// @notice Tests that `call` succeeds.
    function testFuzz_call_succeeds(address from, address to, uint256 gas, uint64 value, bytes memory data) external {
        CallBalances memory balancesBefore = prepareCall(from, to, value);

        vm.expectCall(to, value, data);
        vm.prank(from);
        bool success = SafeCall.call(to, gas, value, data);

        assertTrue(success, "call not successful");
        assertCallBalances(from, to, value, balancesBefore);
    }
}

/// @title SafeCall_CallWithMinGas_Test
/// @notice Tests the `callWithMinGas` function of the `SafeCall` contract.
contract SafeCall_CallWithMinGas_Test is SafeCall_TestInit {
    /// @notice Tests that `callWithMinGas` succeeds with enough gas.
    function testFuzz_callWithMinGas_hasEnough_succeeds(
        address from,
        address to,
        uint64 minGas,
        uint64 value,
        bytes memory data
    )
        external
    {
        CallBalances memory balancesBefore = prepareCall(from, to, value);

        // Bound minGas to [0, l1_block_gas_limit]
        minGas = uint64(bound(minGas, 0, 30_000_000));

        vm.expectCallMinGas(to, value, minGas, data);
        vm.prank(from);
        bool success = SafeCall.callWithMinGas(to, minGas, value, data);

        assertTrue(success, "call not successful");
        assertCallBalances(from, to, value, balancesBefore);
    }

    /// @notice Tests that `callWithMinGas` succeeds for the lower gas bounds.
    function test_callWithMinGas_noLeakageLow_succeeds() external {
        SimpleSafeCaller caller = new SimpleSafeCaller();

        checkNoGasLeakage({
            _caller: caller,
            _startGas: 40_000,
            _endGas: 100_000,
            _minGas: 25_000,
            _expected: expectedSafeCallGas({ _coverageOrLiteGas: 66_290, _testGas: 65_922 }),
            _setACall: abi.encodeCall(caller.setA, (1))
        });
    }

    /// @notice Tests that `callWithMinGas` succeeds on the upper gas bounds.
    function test_callWithMinGas_noLeakageHigh_succeeds() external {
        SimpleSafeCaller caller = new SimpleSafeCaller();

        checkNoGasLeakage({
            _caller: caller,
            _startGas: 15_200_000,
            _endGas: 15_300_000,
            _minGas: 15_000_000,
            _expected: expectedSafeCallGas({ _coverageOrLiteGas: 15_278_989, _testGas: 15_278_621 }),
            _setACall: abi.encodeCall(caller.setA, (1))
        });
    }

    /// @notice Returns the gas threshold calibrated for the current forge context.
    /// @dev Thresholds are calibrated from the failing gas arg when running these tests with `-vvv`.
    function expectedSafeCallGas(uint256 _coverageOrLiteGas, uint256 _testGas) internal view returns (uint256) {
        if (vm.isContext(VmSafe.ForgeContext.Coverage) || LibString.eq(Config.foundryProfile(), "lite")) {
            return _coverageOrLiteGas;
        } else if (vm.isContext(VmSafe.ForgeContext.Test) || vm.isContext(VmSafe.ForgeContext.Snapshot)) {
            return _testGas;
        } else {
            revert("SafeCall_Test: unknown context");
        }
    }

    function checkNoGasLeakage(
        SimpleSafeCaller _caller,
        uint64 _startGas,
        uint64 _endGas,
        uint64 _minGas,
        uint256 _expected,
        bytes memory _setACall
    )
        internal
    {
        for (uint64 i = _startGas; i < _endGas; i++) {
            uint256 snapshot = vm.snapshotState();

            if (i < _expected) {
                assertFalse(_caller.makeSafeCall(i, _minGas));
            } else {
                vm.expectCallMinGas(address(_caller), 0, _minGas, _setACall);
                assertTrue(_caller.makeSafeCall(i, _minGas));
            }

            assertTrue(vm.revertToState(snapshot));
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { StdUtils } from "lib/forge-std/src/StdUtils.sol";
import { Test } from "lib/forge-std/src/Test.sol";
import { Vm } from "lib/forge-std/src/Vm.sol";
import { SafeCall } from "src/libraries/SafeCall.sol";

abstract contract SafeCall_Invariants is Test {
    SafeCaller_Actor actor;

    function _init(bool _shouldFail) internal {
        actor = new SafeCaller_Actor(vm, _shouldFail);
        targetSender(address(this));
        targetContract(address(actor));
        vm.deal(address(actor), type(uint128).max);
    }

    function performSafeCallMinGas(address to, uint64 minGas) external payable {
        SafeCall.callWithMinGas(to, minGas, msg.value, hex"");
    }
}

contract SafeCall_Succeeds_Invariants is SafeCall_Invariants {
    function setUp() public {
        _init(false);
    }

    /// @custom:invariant If `callWithMinGas` performs a call, then it must always
    ///                   provide at least the specified minimum gas limit to the subcontext.
    ///
    ///                   If the check for remaining gas in `SafeCall.callWithMinGas` passes, the
    ///                   subcontext of the call below it must be provided at least `minGas` gas.
    function invariant_callWithMinGas_alwaysForwardsMinGas_succeeds() public view {
        assertFalse(actor.badCallResult());
    }
}

contract SafeCall_Fails_Invariants is SafeCall_Invariants {
    function setUp() public {
        _init(true);
    }

    /// @custom:invariant `callWithMinGas` reverts if there is not enough gas to pass
    ///                   to the subcontext.
    ///
    ///                   If there is not enough gas in the callframe to ensure that
    ///                   `callWithMinGas` can provide the specified minimum gas limit
    ///                   to the subcontext of the call, then `callWithMinGas` must revert.
    function invariant_callWithMinGas_neverForwardsMinGas_reverts() public view {
        assertFalse(actor.badCallResult());
    }
}

contract SafeCaller_Actor is StdUtils {
    address internal constant CONSOLE = 0x000000000000000000636F6e736F6c652e6c6f67;
    uint64 internal constant MIN_MIN_GAS = 2_500;
    uint64 internal constant MAX_MIN_GAS = uint64(type(uint48).max);
    uint64 internal constant CALL_WITH_MIN_GAS_OVERHEAD = 40_000;
    uint64 internal constant SAFE_CALL_BUFFER = 1_000;

    Vm internal immutable vm;
    bool internal immutable shouldFail;

    bool public badCallResult;

    constructor(Vm _vm, bool _shouldFail) {
        vm = _vm;
        shouldFail = _shouldFail;
    }

    function performSafeCallMinGas(uint64 gas, uint64 minGas, address to, uint8 value) external {
        // Only send to EOAs - we exclude the console as it has no code but reverts when called
        // with a selector that doesn't exist due to the foundry hook.
        vm.assume(to.code.length == 0 && to != CONSOLE);

        minGas = uint64(bound(minGas, MIN_MIN_GAS, MAX_MIN_GAS));
        uint64 minCallGas = (minGas * 64) / 63;
        if (shouldFail) {
            gas = uint64(bound(gas, minGas, minCallGas));
        } else {
            gas = uint64(bound(gas, minCallGas + CALL_WITH_MIN_GAS_OVERHEAD + SAFE_CALL_BUFFER, type(uint64).max));
            vm.expectCallMinGas(to, value, minGas, hex"");
        }

        bool success = SafeCall.call(
            msg.sender, gas, value, abi.encodeCall(SafeCall_Invariants.performSafeCallMinGas, (to, minGas))
        );

        if (success == shouldFail) {
            badCallResult = true;
        }
    }
}

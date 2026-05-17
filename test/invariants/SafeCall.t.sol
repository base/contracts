// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";
import { SafeCall } from "src/libraries/SafeCall.sol";

abstract contract SafeCall_Invariants is Test {
    SafeCaller_Actor actor;

    function _init(bool _expectCallToFail) internal {
        actor = new SafeCaller_Actor(address(this), _expectCallToFail);
        targetContract(address(actor));

        bytes4[] memory selectors = new bytes4[](1);
        selectors[0] = actor.performSafeCallMinGas.selector;
        targetSelector(FuzzSelector({ addr: address(actor), selectors: selectors }));

        vm.deal(address(actor), type(uint128).max);
    }

    function callWithMinGas(address to, uint64 minGas) external payable {
        SafeCall.callWithMinGas(to, minGas, msg.value, hex"");
    }
}

contract SafeCall_Succeeds_Invariants is SafeCall_Invariants {
    function setUp() public {
        _init(false);
    }

    /// @custom:invariant Successful calls always forward at least the requested minimum gas.
    function invariant_callWithMinGas_alwaysForwardsMinGas_succeeds() external view {
        assertFalse(actor.badCallResult());
    }
}

contract SafeCall_Fails_Invariants is SafeCall_Invariants {
    function setUp() public {
        _init(true);
    }

    /// @custom:invariant Calls revert when the frame cannot provide the requested minimum gas.
    function invariant_callWithMinGas_neverForwardsMinGas_reverts() external view {
        assertFalse(actor.badCallResult());
    }
}

contract SafeCaller_Actor is Test {
    uint64 internal constant MIN_MIN_GAS = 2_500;

    // Keep the EIP-150 min-gas calculation inside uint64 bounds.
    uint64 internal constant MAX_MIN_GAS = type(uint64).max / 64;

    // `callWithMinGas` reserves 40k call overhead; the extra 1k covers the harness frame.
    uint64 internal constant CALL_WITH_MIN_GAS_BUFFER = 40_000 + 1_000;

    address internal immutable safeCallHarness;
    bool internal immutable expectCallToFail;

    // Invariant handlers ignore target-call reverts, so failures must persist after the call.
    bool public badCallResult;

    constructor(address _safeCallHarness, bool _expectCallToFail) {
        safeCallHarness = _safeCallHarness;
        expectCallToFail = _expectCallToFail;
    }

    function performSafeCallMinGas(uint64 gas, uint64 minGas, address to, uint8 value) external {
        assumeUnusedAddress(to);

        minGas = uint64(bound(minGas, MIN_MIN_GAS, MAX_MIN_GAS));
        uint64 minCallGas = (minGas * 64) / 63;
        if (expectCallToFail) {
            gas = uint64(bound(gas, minGas, minCallGas));
        } else {
            gas = uint64(bound(gas, minCallGas + CALL_WITH_MIN_GAS_BUFFER, type(uint64).max));
            vm.expectCallMinGas(to, value, minGas, hex"");
        }

        bool success = SafeCall.call(
            safeCallHarness, gas, value, abi.encodeCall(SafeCall_Invariants.callWithMinGas, (to, minGas))
        );

        bool expectedSuccess = !expectCallToFail;
        if (success != expectedSuccess) {
            badCallResult = true;
        }
    }
}

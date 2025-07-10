// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {MultisigScript} from "./MultisigScript.sol";

/// @title DoubleNestedMultisigBuilder
/// @custom:deprecated Use `MultisigScript` instead.
abstract contract DoubleNestedMultisigBuilder is MultisigScript {
    /// @custom:deprecated Use `sign(address[] memory safes)` instead.
    function sign(address signerSafe, address intermediateSafe) external {
        sign({safes: _toArray(signerSafe, intermediateSafe)});
    }

    /// @custom:deprecated Use `approve(address[] memory safes, bytes memory signatures)` instead.
    function approveOnBehalfOfSignerSafe(address signerSafe, address intermediateSafe, bytes memory signatures)
        public
    {
        approve({safes: _toArray(signerSafe, intermediateSafe), signatures: signatures});
    }

    /// @custom:deprecated Use `approve(address[] memory safes, bytes memory signatures)` instead.
    function approveOnBehalfOfIntermediateSafe(address intermediateSafe) public {
        approve({safes: _toArray(intermediateSafe), signatures: ""});
    }

    /// @custom:deprecated Use `simulate(bytes memory signatures)` instead, with empty `signatures`.
    function simulate() public {
        simulate({signatures: ""});
    }

    /// @custom:deprecated Use `run(bytes memory signatures)` instead, with empty `signatures`.
    function run() public {
        run({signatures: ""});
    }

    /// @custom:deprecated Use `verify(address[] memory safes, bytes memory signatures)` instead.
    function verify(address signerSafe, address intermediateSafe, bytes memory signatures) public view {
        verify({safes: _toArray(signerSafe, intermediateSafe), signatures: signatures});
    }
}

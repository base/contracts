// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {MultisigScript} from "./MultisigScript.sol";

/// @title NestedMultisigBuilder
/// @custom:deprecated Use `MultisigScript` instead.
abstract contract NestedMultisigBuilder is MultisigScript {
    /// @custom:deprecated Use `sign(address[] memory safes)` instead.
    function sign(address signerSafe) external {
        sign({safes: _toArray(signerSafe)});
    }

    /// @custom:deprecated Use `approve(address[] memory safes, bytes memory signatures)` instead.
    function approve(address signerSafe, bytes memory signatures) public {
        approve({safes: _toArray(signerSafe), signatures: signatures});
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
    function verify(address signerSafe, bytes memory signatures) public view {
        verify({safes: _toArray(signerSafe), signatures: signatures});
    }
}

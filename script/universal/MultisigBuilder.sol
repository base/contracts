// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {MultisigScript} from "./MultisigScript.sol";

/// @title MultisigBuilder
/// @custom:deprecated Use `MultisigScript` instead.
abstract contract MultisigBuilder is MultisigScript {
    /// @custom:deprecated Use `sign(address[] memory safes)` instead, with an empty array.
    function sign() external {
        sign({safes: new address[](0)});
    }

    /// @custom:deprecated Use `verify(address[] memory safes, bytes memory signatures)` instead, with an empty array.
    function verify(bytes memory signatures) external view {
        verify({safes: new address[](0), signatures: signatures});
    }
}

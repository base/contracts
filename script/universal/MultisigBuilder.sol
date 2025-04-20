// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {MultisigScript} from "./MultisigScript.sol";

/**
 * @title MultisigBuilder
 * @custom:deprecated Use `MultisigScript` instead.
 */
abstract contract MultisigBuilder is MultisigScript {
    /*
     * @custom:deprecated Use `sign(address[] memory _safes)` instead, with an empty array.
     */
    function sign() public {
        sign(new address[](0));
    }

    /*
     * @custom:deprecated Use `verify(address[] memory _safes, bytes memory _signatures)` instead,
     *                    with an empty array.
     */
    function verify(bytes memory _signatures) public view {
        verify(new address[](0), _signatures);
    }
}

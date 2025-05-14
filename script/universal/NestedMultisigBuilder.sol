// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {MultisigScript} from "./MultisigScript.sol";

/**
 * @title NestedMultisigBuilder
 * @custom:deprecated Use `MultisigScript` instead.
 */
abstract contract NestedMultisigBuilder is MultisigScript {
    /*
     * @custom:deprecated Use `sign(address[] memory _safes)` instead.
     */
    function sign(address _signerSafe) external {
        sign(_toArray(_signerSafe));
    }

    /*
     * @custom:deprecated Use `verify(address[] memory _safes, bytes memory _signatures)` instead.
     */
    function verify(address _signerSafe, bytes memory _signatures) public view {
        verify(_toArray(_signerSafe), _signatures);
    }

    /*
     * @custom:deprecated Use `approve(address[] memory _safes, bytes memory _signatures)` instead.
     */
    function approve(address _signerSafe, bytes memory _signatures) public {
        approve(_toArray(_signerSafe), _signatures);
    }

    /*
     * @custom:deprecated Use `simulate(bytes memory _signatures)` instead, with empty `_signatures`.
     */
    function simulate() public {
        simulate("");
    }

    /*
     * @custom:deprecated Use `run(bytes memory _signatures)` instead, with empty `_signatures`.
     */
    function run() public {
        run("");
    }
}

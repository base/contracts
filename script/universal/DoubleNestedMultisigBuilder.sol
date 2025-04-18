// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {MultisigScript} from "./MultisigScript.sol";

/**
 * @title DoubleNestedMultisigBuilder
 * @custom:deprecated Use `MultisigScript` instead.
 */
abstract contract DoubleNestedMultisigBuilder is MultisigScript {
    /*
     * @custom:deprecated Use `sign(address[] memory _safes)` instead.
     */
    function sign(address _signerSafe, address _intermediateSafe) public {
        sign(_toArray(_signerSafe, _intermediateSafe));
    }

    /*
     * @custom:deprecated Use `verify(address[] memory _safes, bytes memory _signatures)` instead.
     */
    function verify(address _signerSafe, address _intermediateSafe, bytes memory _signatures) public view {
        verify(_toArray(_signerSafe, _intermediateSafe), _signatures);
    }

    /*
     * @custom:deprecated Use `approve(address[] memory _safes, bytes memory _signatures)` instead.
     */
    function approveOnBehalfOfSignerSafe(address _signerSafe, address _intermediateSafe, bytes memory _signatures)
        public
    {
        approve(_toArray(_signerSafe, _intermediateSafe), _signatures);
    }

    /*
     * @custom:deprecated Use `approve(address[] memory _safes, bytes memory _signatures)` instead.
     */
    function approveOnBehalfOfIntermediateSafe(address _intermediateSafe) public {
        approve(_toArray(_intermediateSafe), "");
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

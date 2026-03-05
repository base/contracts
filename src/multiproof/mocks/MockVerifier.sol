// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Verifier } from "../Verifier.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";

contract MockVerifier is Verifier {
    constructor(IAnchorStateRegistry anchorStateRegistry) Verifier(anchorStateRegistry) {}

    function verify(bytes calldata, bytes32, bytes32) external view override notNullified returns (bool) {
        return true;
    }
}

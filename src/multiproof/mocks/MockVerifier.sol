// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

<<<<<<< simplify-and-modularize-aggregate-verifier
import { Verifier } from "../Verifier.sol";
import { IAnchorStateRegistry } from "interfaces/dispute/IAnchorStateRegistry.sol";

contract MockVerifier is Verifier {
    constructor(IAnchorStateRegistry anchorStateRegistry) Verifier(anchorStateRegistry) { }

    function verify(bytes calldata, bytes32, bytes32) external view override notNullified returns (bool) {
=======
import { IVerifier } from "interfaces/multiproof/IVerifier.sol";

contract MockVerifier is IVerifier {
    function verify(bytes calldata, bytes32, bytes32) external pure returns (bool) {
>>>>>>> main
        return true;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {ITEEVerifier} from "../interfaces/ITEEVerifier.sol";
import {Claim} from "optimism/src/dispute/lib/Types.sol";

contract MockTEEVerifier is ITEEVerifier {
    function verify(bytes calldata , Claim , uint256 ) external pure returns (bool) {
        return true;
    }
}
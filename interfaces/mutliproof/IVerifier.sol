// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Claim } from "optimism/src/dispute/lib/Types.sol";

interface IVerifier {
    function verify(bytes calldata proofBytes, bytes32 journal) external view returns (bool);
}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Claim } from "optimism/src/dispute/lib/Types.sol";

interface IZKVerifier {
    function verify(bytes calldata proofBytes, Claim rootClaim, uint256 l2SequenceNumber) external view returns (bool);
}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

interface IVerifier {
    function verify(bytes calldata proofBytes, bytes32 imageId, bytes32 journal) external view returns (bool);
}

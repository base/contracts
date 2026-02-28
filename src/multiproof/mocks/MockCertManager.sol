// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { ICertManager } from "@nitro-validator/ICertManager.sol";

/// @title MockCertManager
/// @notice Mock CertManager for testing SystemConfigGlobal.
contract MockCertManager is ICertManager {
    function verifyCACert(bytes memory, bytes32) external pure returns (bytes32) {
        return bytes32(0);
    }

    function verifyClientCert(bytes memory, bytes32) external pure returns (VerifiedCert memory) {
        return VerifiedCert({ ca: false, notAfter: 0, maxPathLen: 0, subjectHash: bytes32(0), pubKey: "" });
    }
}

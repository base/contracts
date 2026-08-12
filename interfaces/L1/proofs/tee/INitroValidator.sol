// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { CborElement } from "lib/nitro-validator/src/CborDecode.sol";

interface INitroValidator {
    /// @notice Pointers to fields in a validated Nitro attestation's TBS bytes.
    struct Ptrs {
        CborElement moduleID;
        uint64 timestamp;
        CborElement digest;
        CborElement[] pcrs;
        CborElement cert;
        CborElement[] cabundle;
        CborElement publicKey;
        CborElement userData;
        CborElement nonce;
    }

    /// @notice Validates a Nitro attestation using offchain inverse hints.
    /// @param attestationTbs The COSE Sign1 to-be-signed bytes.
    /// @param signature The 96-byte P-384 attestation signature.
    /// @param attestationSigHints Offchain inverse hints for the attestation signature.
    /// @return ptrs Pointers to the validated attestation fields.
    function validateAttestationWithHints(
        bytes calldata attestationTbs,
        bytes calldata signature,
        bytes calldata attestationSigHints
    )
        external
        returns (Ptrs memory ptrs);
}

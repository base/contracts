// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import { ITDXVerifier, TDXVerificationResult, TDXVerifierJournal } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

/// @title TDXVerifier
/// @notice Onchain policy verifier for Confidential Space Intel TDX signer registration.
/// @dev The RISC Zero guest verifies the Google Cloud Attestation PKI token and its workload claims.
contract TDXVerifier is ITDXVerifier, ISemver {
    /// @notice Maximum accepted age of a Confidential Space token, in seconds.
    uint64 public immutable maxTimeDiff;

    /// @notice Keccak256 hash of the trusted Google Confidential Space root CA.
    bytes32 public immutable rootCaHash;

    /// @notice Keccak256 hash of the required custom token audience.
    bytes32 public immutable audienceHash;

    /// @notice RISC Zero verifier router.
    IRiscZeroVerifier public immutable riscZeroVerifier;

    /// @notice RISC Zero image ID for the Confidential Space verifier guest.
    bytes32 public immutable verifierId;

    /// @notice Keccak256 hash of the Google Cloud Attestation Intel TDX hardware model claim.
    bytes32 public constant GCP_INTEL_TDX_HASH = keccak256("GCP_INTEL_TDX");

    error ZeroInput();
    error RootCaHashMismatch(bytes32 expected, bytes32 actual);
    error AudienceHashMismatch(bytes32 expected, bytes32 actual);
    error VerificationFailed(TDXVerificationResult result);
    error InvalidTokenTime(uint64 issuedAt, uint64 expiration, uint256 currentTimestamp);
    error HardwareModelMismatch(bytes32 expected, bytes32 actual);
    error SecureBootRequired();
    error DebugImageNotAllowed();
    error LaunchOverrideNotAllowed();
    error InvalidPublicKey();
    error SignerMismatch(address expected, address actual);
    error ChainIdMismatch(uint64 expected, uint64 actual);
    error RegistryAddressMismatch(address expected, address actual);

    constructor(
        uint64 initialMaxTimeDiff,
        bytes32 initialRootCaHash,
        bytes32 initialAudienceHash,
        address initialRiscZeroVerifier,
        bytes32 initialVerifierId
    ) {
        if (
            initialMaxTimeDiff == 0 || initialRootCaHash == bytes32(0) || initialAudienceHash == bytes32(0)
                || initialRiscZeroVerifier == address(0) || initialVerifierId == bytes32(0)
        ) revert ZeroInput();
        maxTimeDiff = initialMaxTimeDiff;
        rootCaHash = initialRootCaHash;
        audienceHash = initialAudienceHash;
        riscZeroVerifier = IRiscZeroVerifier(initialRiscZeroVerifier);
        verifierId = initialVerifierId;
    }

    /// @inheritdoc ITDXVerifier
    function verify(
        bytes calldata output,
        bytes calldata proofBytes
    )
        external
        view
        returns (address signer, bytes32 imageHash)
    {
        riscZeroVerifier.verify(proofBytes, verifierId, sha256(output));
        TDXVerifierJournal memory journal = abi.decode(output, (TDXVerifierJournal));

        if (journal.result != TDXVerificationResult.Success) revert VerificationFailed(journal.result);
        if (journal.rootCaHash != rootCaHash) revert RootCaHashMismatch(rootCaHash, journal.rootCaHash);
        if (journal.audienceHash != audienceHash) revert AudienceHashMismatch(audienceHash, journal.audienceHash);
        if (
            journal.issuedAt > block.timestamp || journal.issuedAt + maxTimeDiff <= block.timestamp
                || journal.expiration <= block.timestamp
        ) revert InvalidTokenTime(journal.issuedAt, journal.expiration, block.timestamp);
        if (journal.hardwareModelHash != GCP_INTEL_TDX_HASH) {
            revert HardwareModelMismatch(GCP_INTEL_TDX_HASH, journal.hardwareModelHash);
        }
        if (!journal.secureBoot) revert SecureBootRequired();
        if (!journal.debugDisabled) revert DebugImageNotAllowed();
        if (journal.commandOverride || journal.environmentOverride) revert LaunchOverrideNotAllowed();
        if (journal.chainId != block.chainid) revert ChainIdMismatch(uint64(block.chainid), journal.chainId);
        if (journal.registryAddress != msg.sender) revert RegistryAddressMismatch(msg.sender, journal.registryAddress);
        if (journal.publicKey.length != 65 || journal.publicKey[0] != bytes1(0x04)) revert InvalidPublicKey();

        bytes32 publicKeyHash;
        bytes memory publicKey = journal.publicKey;
        assembly {
            publicKeyHash := keccak256(add(publicKey, 0x21), 64)
        }
        address derivedSigner = address(uint160(uint256(publicKeyHash)));
        if (journal.signer != derivedSigner) revert SignerMismatch(derivedSigner, journal.signer);

        return (derivedSigner, journal.imageHash);
    }

    /// @notice Semantic version.
    /// @custom:semver 0.5.0
    function version() public pure virtual returns (string memory) {
        return "0.5.0";
    }
}

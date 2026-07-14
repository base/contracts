// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

import { ISemver } from "interfaces/universal/ISemver.sol";
import {
    ITDXVerifier,
    TDXTcbStatus,
    TDXVerificationResult,
    TDXVerifierJournal
} from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

/// @title TDXVerifier
/// @notice Production-shape Intel TDX DCAP verifier for multiproof signer registration.
/// @dev The heavy TDX work is expected to happen in a ZK guest: quote signature verification, PCK chain
///      validation, TCB info validation, QE identity validation, CRL checks, and extraction of TDREPORT fields.
///      This contract verifies the ZK proof and enforces onchain policy over the verified journal.
contract TDXVerifier is ITDXVerifier, ISemver {
    /// @notice Maximum accepted age of a TDX quote, in seconds.
    uint64 public immutable maxTimeDiff;

    /// @notice Hash of the trusted Intel root CA used by the ZK verifier guest.
    bytes32 public immutable rootCaHash;

    /// @notice RISC Zero verifier router.
    IRiscZeroVerifier public immutable riscZeroVerifier;

    /// @notice RISC Zero image ID for the TDX DCAP verifier guest.
    bytes32 public immutable verifierId;

    error ZeroInput();
    error RootCaHashMismatch(bytes32 expected, bytes32 actual);
    error VerificationFailed(TDXVerificationResult result);
    error TcbStatusNotAllowed(TDXTcbStatus status);
    error CollateralExpired(uint64 collateralExpiration);
    error InvalidTimestamp(uint64 timestampSeconds, uint256 currentTimestamp);
    error InvalidPublicKey();
    error ReportDataMismatch(bytes32 expected, bytes32 actual);
    error SignerMismatch(address expected, address actual);
    error DebugTdNotAllowed(uint64 tdAttributes);
    error ChainIdMismatch(uint64 expected, uint64 actual);
    error RegistryAddressMismatch(address expected, address actual);

    constructor(
        uint64 initialMaxTimeDiff,
        bytes32 initialRootCaHash,
        address initialRiscZeroVerifier,
        bytes32 initialVerifierId
    ) {
        if (
            initialMaxTimeDiff == 0 || initialRootCaHash == bytes32(0) || initialRiscZeroVerifier == address(0)
                || initialVerifierId == bytes32(0)
        ) revert ZeroInput();
        maxTimeDiff = initialMaxTimeDiff;
        rootCaHash = initialRootCaHash;
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
        if (journal.tcbStatus != TDXTcbStatus.UpToDate && journal.tcbStatus != TDXTcbStatus.SwHardeningNeeded) {
            revert TcbStatusNotAllowed(journal.tcbStatus);
        }
        if (journal.collateralExpiration <= block.timestamp) revert CollateralExpired(journal.collateralExpiration);

        uint64 timestamp = journal.timestamp / 1000;
        if (timestamp + maxTimeDiff <= block.timestamp || timestamp >= block.timestamp) {
            revert InvalidTimestamp(timestamp, block.timestamp);
        }

        if (journal.tdAttributes & 1 != 0) revert DebugTdNotAllowed(journal.tdAttributes);
        if (journal.chainId != block.chainid) revert ChainIdMismatch(uint64(block.chainid), journal.chainId);
        if (journal.registryAddress != msg.sender) revert RegistryAddressMismatch(msg.sender, journal.registryAddress);
        if (journal.publicKey.length != 65 || journal.publicKey[0] != bytes1(0x04)) revert InvalidPublicKey();

        bytes32 publicKeyHash;
        bytes memory publicKey = journal.publicKey;
        assembly {
            publicKeyHash := keccak256(add(publicKey, 0x21), 64)
        }
        if (journal.reportDataPrefix != publicKeyHash) {
            revert ReportDataMismatch(publicKeyHash, journal.reportDataPrefix);
        }

        address derivedSigner = address(uint160(uint256(publicKeyHash)));
        if (journal.signer != derivedSigner) revert SignerMismatch(derivedSigner, journal.signer);

        return (derivedSigner, journal.imageHash);
    }

    /// @notice Semantic version.
    /// @custom:semver 0.4.0
    function version() public pure virtual returns (string memory) {
        return "0.4.0";
    }
}

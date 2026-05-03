// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { EnumerableSetLib } from "@solady-v0.0.245/utils/EnumerableSetLib.sol";

import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier, TDXVerifierJournal, ZkCoProcessorType } from "interfaces/multiproof/tee/ITDXVerifier.sol";

import { TEEProverRegistry } from "./TEEProverRegistry.sol";

/// @title TDXTEEProverRegistry
/// @notice TEE prover registry extension that can register signers from Intel TDX quotes.
/// @dev Nitro registration remains available through TEEProverRegistry.registerSigner(). This contract adds a
///      separate TDX path so existing Nitro deployments and verifier integrations do not need to change.
contract TDXTEEProverRegistry is TEEProverRegistry {
    using EnumerableSetLib for EnumerableSetLib.AddressSet;

    /// @notice The TDX verifier used to validate ZK-proven TD quote verification journals.
    ITDXVerifier public immutable TDX_VERIFIER;

    /// @notice Emitted when a TDX signer is registered.
    event TDXSignerRegistered(address indexed signer, bytes32 indexed imageHash, bytes32 reportDataSuffix);

    /// @notice Thrown when the TDX verifier is not configured.
    error TDXVerifierNotSet();

    constructor(
        INitroEnclaveVerifier nitroVerifier,
        ITDXVerifier tdxVerifier,
        IDisputeGameFactory factory
    )
        TEEProverRegistry(nitroVerifier, factory)
    {
        if (address(tdxVerifier) == address(0)) revert TDXVerifierNotSet();
        TDX_VERIFIER = tdxVerifier;
    }

    /// @notice Registers a signer using a ZK proof of Intel TDX DCAP quote verification.
    /// @param output ABI-encoded TDXVerifierJournal public values from the ZK verifier guest.
    /// @param proofBytes ZK proof bytes.
    function registerTDXSigner(bytes calldata output, bytes calldata proofBytes) external onlyOwnerOrManager {
        TDXVerifierJournal memory journal = TDX_VERIFIER.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        isRegisteredSigner[journal.signer] = true;
        signerImageHash[journal.signer] = journal.imageHash;
        _registeredSigners.add(journal.signer);

        emit SignerRegistered(journal.signer);
        emit TDXSignerRegistered(journal.signer, journal.imageHash, journal.reportDataSuffix);
    }

    /// @notice Semantic version.
    /// @custom:semver 0.1.0
    function version() public pure override returns (string memory) {
        return "0.1.0";
    }
}

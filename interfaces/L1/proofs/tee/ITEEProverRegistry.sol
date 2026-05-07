// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

interface ITEEProverRegistry {
    function NITRO_VERIFIER() external view returns (INitroEnclaveVerifier);
    function DISPUTE_GAME_FACTORY() external view returns (IDisputeGameFactory);
    function gameType() external view returns (GameType);
    function isRegisteredSigner(address signer) external view returns (bool);
    function signerImageHash(address signer) external view returns (bytes32);
    function isValidProposer(address proposer) external view returns (bool);
    function owner() external view returns (address);
    function manager() external view returns (address);

    function setProposer(address proposer, bool isValid) external;
    function setGameType(GameType gameType_) external;
    function registerSigner(bytes calldata output, bytes calldata proofBytes) external;
    function deregisterSigner(address signer) external;
    function isValidSigner(address signer) external view returns (bool);
    function getRegisteredSigners() external view returns (address[] memory);
    function getExpectedImageHash() external view returns (bytes32);
    function initialize(
        address initialOwner,
        address initialManager,
        address[] memory initialProposers,
        GameType gameType_
    )
        external;
    function version() external pure returns (string memory);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @notice Development deployment using the TDX signer-registration path.
contract DeployDevWithTDX is DeployDevBase {
    address internal nitroEnclaveVerifierAddr;
    address internal tdxVerifierAddr;
    address internal tdxRegistrationManager;

    function run(
        address nitroEnclaveVerifier,
        address tdxVerifier,
        address registrationManager,
        bytes32 asrStartingOutputRoot,
        uint256 asrStartingBlockNumber
    )
        public
    {
        nitroEnclaveVerifierAddr = nitroEnclaveVerifier;
        tdxVerifierAddr = tdxVerifier;
        tdxRegistrationManager = registrationManager;
        run(asrStartingOutputRoot, asrStartingBlockNumber);
    }

    function _outputSuffix() internal pure override returns (string memory) {
        return "-dev-with-tdx.json";
    }

    function _preflight() internal view override {
        require(tdxVerifierAddr != address(0), "tdxVerifier must be non-zero");
        require(nitroEnclaveVerifierAddr != address(0), "nitroEnclaveVerifier must be non-zero");
        require(tdxRegistrationManager != address(0), "registrationManager must be non-zero");
    }

    function _nitroEnclaveVerifier() internal view override returns (address) {
        return nitroEnclaveVerifierAddr;
    }

    function _tdxVerifier() internal view override returns (address) {
        return tdxVerifierAddr;
    }

    function _teeRegistrationManager() internal view override returns (address) {
        return tdxRegistrationManager;
    }

    function _serializeExtra(string memory key) internal override {
        vm.serializeAddress(key, "NitroEnclaveVerifier", nitroEnclaveVerifierAddr);
        vm.serializeAddress(key, "TDXVerifier", tdxVerifierAddr);
        vm.serializeAddress(key, "TDXRegistrationManager", tdxRegistrationManager);
    }
}

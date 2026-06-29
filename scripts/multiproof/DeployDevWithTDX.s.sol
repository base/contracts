// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @notice Development deployment using the TDX signer-registration path.
contract DeployDevWithTDX is DeployDevBase {
    function run() public pure override {
        revert("use parameterized run");
    }

    function run(bytes32, uint256) public pure override {
        revert("use parameterized run");
    }

    function run(
        address nitroEnclaveVerifier,
        address tdxVerifier,
        address registrationManager,
        bytes32 asrStartingOutputRoot,
        uint256 asrStartingBlockNumber
    )
        public
    {
        require(nitroEnclaveVerifier != address(0), "nitroEnclaveVerifier must be non-zero");
        _run(asrStartingOutputRoot, asrStartingBlockNumber, nitroEnclaveVerifier, tdxVerifier, registrationManager);
    }

    function _outputSuffix() internal pure override returns (string memory) {
        return "-dev-with-tdx.json";
    }
}

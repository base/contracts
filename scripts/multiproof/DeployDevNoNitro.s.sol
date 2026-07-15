// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { DeployDevBase } from "./DeployDevBase.s.sol";

/// @title DeployDevNoNitro
/// @notice Development deployment using DevTEEProverRegistry, which bypasses AWS Nitro attestation
///         validation. See scripts/multiproof/README.md for usage. Not for production.
contract DeployDevNoNitro is DeployDevBase {
    function run() public {
        run(cfg.multiproofGenesisOutputRoot(), cfg.multiproofGenesisBlockNumber());
    }

    function run(bytes32 asrStartingOutputRoot, uint256 asrStartingBlockNumber) public {
        _run(
            asrStartingOutputRoot,
            asrStartingBlockNumber,
            address(0),
            cfg.tdxVerifier(),
            cfg.finalSystemOwner(),
            0.001 ether,
            "-dev-no-nitro.json"
        );
    }
}

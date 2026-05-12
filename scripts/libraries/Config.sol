// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Vm, VmSafe } from "lib/forge-std/src/Vm.sol";

/// @notice Enum of forks available for selection when generating genesis allocs.
enum Fork {
    NONE,
    DELTA,
    ECOTONE,
    FJORD,
    GRANITE,
    HOLOCENE,
    ISTHMUS,
    JOVIAN,
    INTEROP
}

Fork constant LATEST_FORK = Fork.INTEROP;

library ForkUtils {
    function toString(Fork _fork) internal pure returns (string memory) {
        if (_fork == Fork.NONE) {
            return "none";
        } else if (_fork == Fork.DELTA) {
            return "delta";
        } else if (_fork == Fork.ECOTONE) {
            return "ecotone";
        } else if (_fork == Fork.FJORD) {
            return "fjord";
        } else if (_fork == Fork.GRANITE) {
            return "granite";
        } else if (_fork == Fork.HOLOCENE) {
            return "holocene";
        } else if (_fork == Fork.ISTHMUS) {
            return "isthmus";
        } else if (_fork == Fork.JOVIAN) {
            return "jovian";
        } else {
            return "unknown";
        }
    }
}

/// @title Config
/// @notice Contains shared env var based config used by scripts and tests.
library Config {
    /// @notice Foundry cheatcode VM.
    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Returns the path on the local filesystem where the deployment artifact is
    ///         written to disk after doing a deployment.
    function deploymentOutfile() internal view returns (string memory env_) {
        env_ = vm.envOr(
            "DEPLOYMENT_OUTFILE",
            string.concat(vm.projectRoot(), "/deployments/", vm.toString(block.chainid), "-deploy.json")
        );
    }

    /// @notice Returns the path on the local filesystem where the deploy config is
    function deployConfigPath() internal view returns (string memory env_) {
        if (vm.isContext(VmSafe.ForgeContext.TestGroup)) {
            env_ = string.concat(vm.projectRoot(), "/deploy-config/local.json");
        } else {
            env_ = vm.envOr("DEPLOY_CONFIG_PATH", string(""));
            require(bytes(env_).length > 0, "Config: must set DEPLOY_CONFIG_PATH to filesystem path of deploy config");
        }
    }

    /// @notice Returns the chainid from the EVM context or the value of the CHAIN_ID env var as
    ///         an override.
    function chainID() internal view returns (uint256 env_) {
        env_ = vm.envOr("CHAIN_ID", block.chainid);
    }

    /// @notice Returns the string identifier of the OP chain use for forking.
    ///         If not set, "op" is returned.
    function forkOpChain() internal view returns (string memory) {
        return vm.envOr("FORK_OP_CHAIN", string("op"));
    }

    /// @notice Returns the RPC URL to use for forking.
    function forkRpcUrl() internal view returns (string memory) {
        return vm.envString("FORK_RPC_URL");
    }

    /// @notice Returns the block number to use for forking.
    function forkBlockNumber() internal view returns (uint256) {
        return vm.envUint("FORK_BLOCK_NUMBER");
    }

    /// @notice Returns the profile to use for the foundry commands.
    ///         If not set, "default" is returned.
    function foundryProfile() internal view returns (string memory) {
        return vm.envOr("FOUNDRY_PROFILE", string("default"));
    }

    /// @notice Returns the path to the superchain ops allocs.
    function superchainOpsAllocsPath() internal view returns (string memory) {
        return vm.envOr("SUPERCHAIN_OPS_ALLOCS_PATH", string(""));
    }

    /// @notice Returns true if the fork is a test fork.
    function forkTest() internal view returns (bool) {
        return vm.envOr("FORK_TEST", false);
    }
}

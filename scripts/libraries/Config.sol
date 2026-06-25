// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Vm, VmSafe } from "lib/forge-std/src/Vm.sol";

/// @notice Enum of forks available for selection when generating genesis allocs.
/// @dev Keep `ForkUtils.toString` and `LATEST_FORK` in sync with this enum's order and length.
enum Fork {
    NONE,
    DELTA,
    ECOTONE,
    FJORD,
    ISTHMUS,
    JOVIAN,
    INTEROP
}

Fork constant LATEST_FORK = Fork.INTEROP;

library ForkUtils {
    function toString(Fork _fork) internal pure returns (string memory) {
        string[7] memory names = ["none", "delta", "ecotone", "fjord", "isthmus", "jovian", "interop"];
        return names[uint8(_fork)];
    }
}

/// @title Config
/// @notice Contains shared env var based config used by scripts and tests.
library Config {
    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    function deploymentOutfile() internal view returns (string memory) {
        return vm.envOr(
            "DEPLOYMENT_OUTFILE",
            string.concat(vm.projectRoot(), "/deployments/", vm.toString(block.chainid), "-deploy.json")
        );
    }

    /// @dev In test contexts, defaults to deploy-config/local.json; otherwise DEPLOY_CONFIG_PATH must be set.
    function deployConfigPath() internal view returns (string memory) {
        if (vm.isContext(VmSafe.ForgeContext.TestGroup)) {
            return string.concat(vm.projectRoot(), "/deploy-config/local.json");
        }
        string memory path = vm.envOr("DEPLOY_CONFIG_PATH", string(""));
        require(bytes(path).length > 0, "Config: must set DEPLOY_CONFIG_PATH to filesystem path of deploy config");
        return path;
    }

    function chainID() internal view returns (uint256) {
        return vm.envOr("CHAIN_ID", block.chainid);
    }

    function forkOpChain() internal view returns (string memory) {
        return vm.envOr("FORK_OP_CHAIN", string("op"));
    }

    function forkRpcUrl() internal view returns (string memory) {
        return vm.envString("FORK_RPC_URL");
    }

    function forkBlockNumber() internal view returns (uint256) {
        return vm.envUint("FORK_BLOCK_NUMBER");
    }

    function foundryProfile() internal view returns (string memory) {
        return vm.envOr("FOUNDRY_PROFILE", string("default"));
    }

    function superchainOpsAllocsPath() internal view returns (string memory) {
        return vm.envOr("SUPERCHAIN_OPS_ALLOCS_PATH", string(""));
    }

    function forkTest() internal view returns (bool) {
        return vm.envOr("FORK_TEST", false);
    }
}

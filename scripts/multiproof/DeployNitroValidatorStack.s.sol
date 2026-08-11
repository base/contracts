// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import { Script } from "lib/forge-std/src/Script.sol";
import { VmSafe } from "lib/forge-std/src/Vm.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";

import { CertManager } from "lib/nitro-validator/src/CertManager.sol";
import { NitroValidator } from "lib/nitro-validator/src/NitroValidator.sol";
import { P384Verifier } from "lib/nitro-validator/src/P384Verifier.sol";

/// @title DeployNitroValidatorStack
/// @notice Deploys the hinted Nitro attestation validator stack.
contract DeployNitroValidatorStack is Script {
    /// @notice Addresses of a deployed hinted Nitro validator stack.
    struct Deployment {
        P384Verifier p384Verifier;
        CertManager certManager;
        NitroValidator nitroValidator;
    }

    /// @notice Thrown when the initial CertManager owner is the zero address.
    error InvalidOwner();

    /// @notice Thrown when the initial CertManager revoker is the zero address.
    error InvalidRevoker();

    /// @notice Deploys P384Verifier, CertManager, and NitroValidator in dependency order.
    ///
    /// @param _owner Initial CertManager owner.
    /// @param _revoker Initial CertManager revoker.
    ///
    /// @return deployment Addresses of the deployed Nitro validator stack.
    function run(address _owner, address _revoker) public returns (Deployment memory) {
        if (_owner == address(0)) revert InvalidOwner();
        if (_revoker == address(0)) revert InvalidRevoker();

        Deployment memory deployment;
        vm.startBroadcast();
        deployment.p384Verifier = new P384Verifier();
        deployment.certManager = new CertManager(deployment.p384Verifier, _owner, _revoker);
        deployment.nitroValidator = new NitroValidator(deployment.certManager, deployment.p384Verifier);
        vm.stopBroadcast();

        console.log("P384Verifier:", address(deployment.p384Verifier));
        console.log("CertManager:", address(deployment.certManager));
        console.log("NitroValidator:", address(deployment.nitroValidator));

        if (!vm.isContext(VmSafe.ForgeContext.TestGroup)) {
            string memory key = "deployment";
            vm.serializeAddress(key, "P384Verifier", address(deployment.p384Verifier));
            vm.serializeAddress(key, "CertManager", address(deployment.certManager));
            string memory json = vm.serializeAddress(key, "NitroValidator", address(deployment.nitroValidator));
            vm.writeJson(json, string.concat("deployments/", vm.toString(block.chainid), "-nitro-validator.json"));
        }

        return deployment;
    }
}

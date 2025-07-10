// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script, console} from "forge-std/Script.sol";
import {SafeProxyFactory} from "lib/safe-smart-account/contracts/proxies/SafeProxyFactory.sol";
import {SafeL2} from "lib/safe-smart-account/contracts/SafeL2.sol";
import {Safe} from "lib/safe-smart-account/contracts/Safe.sol";
import {SafeProxy} from "lib/safe-smart-account/contracts/proxies/SafeProxy.sol";

/**
 * @title MultisigDeployScript
 * @notice Deploys a hierarchy of Safe multisig wallets where later safes can reference earlier ones as owners
 *
 * @dev This script enables deployment of nested/hierarchical multisig structures for complex governance systems.
 *      Safes are deployed in array order, allowing later safes to use previously deployed safes as owners.
 *
 * EXAMPLE JSON CONFIGURATION (config/safes-nested.json):
 * {
 *   "safeCount": 3,
 *   "safes": [
 *     {
 *       "label": "Treasury",
 *       "threshold": 2,
 *       "owners": [
 *         "0x1234567890123456789012345678901234567890",
 *         "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
 *       ],
 *       "ownerRefIndices": []
 *     },
 *     {
 *       "label": "Operations",
 *       "threshold": 1,
 *       "owners": [
 *         "0x9876543210987654321098765432109876543210"
 *       ],
 *       "ownerRefIndices": [0]
 *     },
 *     {
 *       "label": "Governance",
 *       "threshold": 2,
 *       "owners": [],
 *       "ownerRefIndices": [0, 1]
 *     }
 *   ]
 * }
 *
 * CONFIGURATION FIELDS:
 * - label: Human-readable name for the safe
 * - threshold: Number of signatures required for transactions
 * - owners: Array of direct address owners (EOAs or other contracts)
 * - ownerRefIndices: Array of indices referencing previously deployed safes as owners
 *
 * DEPLOYMENT ORDER MATTERS:
 * - Safes must be ordered so that any referenced safe (via ownerRefIndices) appears earlier in the array
 * - This ensures referenced safes are already deployed when needed as owners
 */
contract MultisigDeployScript is Script {
    // Safe v1.4.1-3 Addresses
    address public constant SINGLETON = 0x29fcB43b46531BcA003ddC8FCB67FFE91900C762;
    address public constant FACTORY_PROXY = 0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67;
    address public constant COMPATBILITY_FALLBACK_HANDLER = 0xfd0732Dc9E303f09fCEf3a7388Ad10A83459Ec99;

    struct SafeWallet {
        string label;
        uint256 threshold;
        address[] owners;
        uint256[] ownerRefIndices;
    }

    // Track deployed safes and their predicted addresses
    mapping(string => address) public deployedSafes;

    SafeWallet[] public safes;

    function setUp() public {
        // Read configs from JSON
        string memory configPath = vm.envString("MULTISIG_CONFIG_PATH");
        string memory json = vm.readFile(configPath);
        console.log("Using config path:", configPath);

        // Read safeCount directly from JSON
        uint256 safeCount = vm.parseJsonUint(json, ".safeCount");
        console.log("Reading", safeCount, "safes from configuration");

        // Parse each safe individually field by field
        for (uint256 i = 0; i < safeCount; i++) {
            string memory basePath = string(abi.encodePacked(".safes[", vm.toString(i), "]"));

            safes.push();

            // Parse simple fields (these work reliably)
            safes[i].label = vm.parseJsonString(json, string(abi.encodePacked(basePath, ".label")));
            safes[i].threshold = vm.parseJsonUint(json, string(abi.encodePacked(basePath, ".threshold")));

            // Parse arrays (these are more reliable when done individually)
            safes[i].owners = vm.parseJsonAddressArray(json, string(abi.encodePacked(basePath, ".owners")));
            safes[i].ownerRefIndices =
                vm.parseJsonUintArray(json, string(abi.encodePacked(basePath, ".ownerRefIndices")));
        }

        // Print out the config to verify parsing worked
        for (uint256 i = 0; i < safes.length; i++) {
            console.log("Safe:", safes[i].label);
            console.log("  Owners:", safes[i].owners.length);
            console.log("  OwnerRefIndices:", safes[i].ownerRefIndices.length);
            console.log("  Threshold:", safes[i].threshold);
        }

        // Print out first safe owners for verification
        console.log("First safe owners:");
        for (uint256 j = 0; j < safes[0].owners.length; j++) {
            console.log("  Owner", j, ":", safes[0].owners[j]);
        }
    }

    function run() public {
        SafeProxyFactory factory = SafeProxyFactory(FACTORY_PROXY);

        // Start broadcasting transactions
        vm.startBroadcast();

        console.log("Deploying", safes.length, "Safe(s) in sequence");
        console.log("--------------------");

        uint256 baseNonce = block.timestamp;
        console.log("Base nonce:", baseNonce);

        // Deploy each Safe with its configuration in array order
        for (uint256 i = 0; i < safes.length; i++) {
            SafeWallet memory config = safes[i];
            uint256 saltNonce = baseNonce + i;

            console.log("Deploying Safe:", config.label);
            console.log("  Index:", i);
            console.log("  Salt Nonce:", saltNonce);

            // Resolve owner addresses (combine direct owners + referenced safe addresses)
            address[] memory resolvedOwners = resolveOwnerAddresses(config);

            console.log("  Total Owners:", resolvedOwners.length);
            console.log("  Direct Owners:", config.owners.length);
            console.log("  Safe References:", config.ownerRefIndices.length);
            console.log("  Threshold:", config.threshold);

            // Compose initializer data with resolved owners
            bytes memory initializer = abi.encodeCall(
                Safe.setup,
                (
                    resolvedOwners,
                    config.threshold,
                    address(0), // to
                    hex"", // data
                    COMPATBILITY_FALLBACK_HANDLER,
                    address(0), // payment token
                    0, // payment
                    payable(address(0)) // payment receiver
                )
            );

            // Deploy Safe with calculated nonce
            SafeProxy safe = factory.createProxyWithNonce(SINGLETON, initializer, saltNonce);

            // Store deployed address
            deployedSafes[config.label] = address(safe);

            console.log("  Deployed at:", address(safe));

            // Log resolved owners
            for (uint256 k = 0; k < resolvedOwners.length; k++) {
                console.log("    Owner", k, ":", resolvedOwners[k]);
            }
            console.log("--------------------");
        }

        vm.stopBroadcast();

        // Verify all deployments
        console.log("Deployment Summary:");
        console.log("==================");
        for (uint256 i = 0; i < safes.length; i++) {
            SafeWallet memory config = safes[i];
            console.log("Safe:", config.label);
            console.log("  Address:", deployedSafes[config.label]);
            console.log("  Salt Nonce:", baseNonce + i);
            console.log("--------------------");
        }
    }

    function resolveOwnerAddresses(SafeWallet memory config) internal view returns (address[] memory) {
        uint256 totalOwners = config.owners.length + config.ownerRefIndices.length;
        address[] memory resolved = new address[](totalOwners);

        // Add direct address owners
        for (uint256 i = 0; i < config.owners.length; i++) {
            resolved[i] = config.owners[i];
        }

        // Add referenced safe addresses (they must already be deployed due to array order)
        for (uint256 i = 0; i < config.ownerRefIndices.length; i++) {
            uint256 refIndex = config.ownerRefIndices[i];
            string memory refLabel = safes[refIndex].label;
            address refAddr = deployedSafes[refLabel];
            require(refAddr != address(0), string(abi.encodePacked("Reference not deployed: ", refLabel)));
            resolved[config.owners.length + i] = refAddr;
        }

        return resolved;
    }
}

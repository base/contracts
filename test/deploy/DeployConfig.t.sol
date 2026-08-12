// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { DeployConfig } from "scripts/deploy/DeployConfig.s.sol";

/// @title DeployConfig_ResourceConfigMinimumBaseFee_Test
/// @notice Covers DeployConfig JSON parsing for the optional resource minimum base fee.
contract DeployConfig_ResourceConfigMinimumBaseFee_Test is Test {
    DeployConfig internal config;
    string internal localConfigPath;
    string internal overrideConfigPath;

    function setUp() public {
        config = new DeployConfig();
        localConfigPath = string.concat(vm.projectRoot(), "/deploy-config/local.json");
        overrideConfigPath =
            string.concat(vm.projectRoot(), "/deployments/deploy-config-resource-minimum-override.json");
        vm.createDir(string.concat(vm.projectRoot(), "/deployments"), true);
    }

    function test_read_resourceConfigMinimumBaseFee_default_succeeds() public {
        config.read(localConfigPath);
        assertEq(config.resourceConfigMinimumBaseFee(), uint32(1 gwei));
    }

    function test_read_resourceConfigMinimumBaseFee_override_succeeds() public {
        string memory json = _withResourceConfigMinimumBaseFee(vm.readFile(localConfigPath), 10_000_000);
        vm.writeFile(overrideConfigPath, json);

        config.read(overrideConfigPath);
        assertEq(config.resourceConfigMinimumBaseFee(), 10_000_000);

        vm.removeFile(overrideConfigPath);
    }

    function _withResourceConfigMinimumBaseFee(
        string memory _json,
        uint256 _value
    )
        internal
        pure
        returns (string memory json_)
    {
        require(_value == 10_000_000, "DeployConfig test: unsupported fixture value");

        bytes memory jsonBytes = bytes(_json);
        uint256 end = jsonBytes.length;
        while (end > 0 && (jsonBytes[end - 1] == "\n" || jsonBytes[end - 1] == "\r" || jsonBytes[end - 1] == " ")) {
            end--;
        }
        require(end > 0 && jsonBytes[end - 1] == "}", "DeployConfig test: expected object");

        bytes memory suffix = bytes(',\n  "resourceConfigMinimumBaseFee": 10000000\n}');
        json_ = new string(end - 1 + suffix.length);

        bytes memory result = bytes(json_);
        for (uint256 i; i < end - 1; ++i) {
            result[i] = jsonBytes[i];
        }
        for (uint256 i; i < suffix.length; ++i) {
            result[end - 1 + i] = suffix[i];
        }
    }
}

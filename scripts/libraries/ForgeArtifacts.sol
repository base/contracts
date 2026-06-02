// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Vm } from "lib/forge-std/src/Vm.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";

/// @notice Contains information about a storage slot. Mirrors the layout of the storage
///         slot object in Forge artifacts so that we can deserialize JSON into this struct.
///         Field order matches the alphabetical JSON key order produced by `vm.parseJson`.
struct ForgeStorageSlot {
    uint256 astId;
    string _contract;
    string label;
    uint256 offset;
    string slot;
    string _type;
}

/// @notice Minimal storage slot information consumed by tests.
struct StorageSlot {
    uint256 offset;
    uint256 slot;
}

/// @title ForgeArtifacts
/// @notice Library for interacting with the forge artifacts.
library ForgeArtifacts {
    /// @notice Foundry cheatcode VM.
    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Forge artifact output directory. Must match `out` in foundry.toml.
    string private constant OUT_DIR = "forge-artifacts";

    /// @notice Removes the semantic versioning suffix from a contract name. The semver appears
    ///         when the contract is compiled more than once with different solc versions.
    function _stripSemver(string memory _name) private pure returns (string memory) {
        return vm.split(_name, ".")[0];
    }

    /// @notice Returns the abi from the forge artifact.
    function getAbi(string memory _name) internal returns (string memory abi_) {
        abi_ = _bash(string.concat("jq -r '.abi' < ", _getForgeArtifactPath(_name)));
    }

    /// @notice Returns the kind of contract (i.e. library, contract, or interface).
    /// @param _name The name of the contract to get the kind of.
    /// @return kind_ The kind of contract ("library", "contract", or "interface").
    function getContractKind(string memory _name) internal returns (string memory kind_) {
        kind_ = _bash(
            string.concat(
                "jq -r '.ast.nodes[] | select(.nodeType == \"ContractDefinition\") | .contractKind' < ",
                _getForgeArtifactPath(_name)
            )
        );
    }

    /// @notice Returns whether or not a contract is proxied. Heuristic based on the
    ///         custom:proxied devdoc tag; deployment script would be a more reliable source.
    function isProxiedContract(string memory _name) internal returns (bool) {
        return _hasDevdocTag(_name, "custom:proxied");
    }

    /// @notice Returns whether or not a contract is predeployed. Heuristic based on the
    ///         custom:predeploy devdoc tag; deployment script would be a more reliable source.
    function isPredeployedContract(string memory _name) internal returns (bool) {
        return _hasDevdocTag(_name, "custom:predeploy");
    }

    function _hasDevdocTag(string memory _name, string memory _tag) private returns (bool) {
        string memory res = _bash(
            string.concat(
                "jq -r '.rawMetadata | fromjson | .output.devdoc | has(\"", _tag, "\")' ", _getForgeArtifactPath(_name)
            )
        );
        return stdJson.readBool(res, "");
    }

    function _getForgeArtifactDirectory(string memory _name) private view returns (string memory) {
        return string.concat(vm.projectRoot(), "/", OUT_DIR, "/", _stripSemver(_name), ".sol");
    }

    /// @notice Returns the filesystem path to the artifact path. If the contract was compiled
    ///         with multiple solidity versions then return the first entry in the directory.
    function _getForgeArtifactPath(string memory _name) private view returns (string memory) {
        string memory directory = _getForgeArtifactDirectory(_name);
        string memory path = string.concat(directory, "/", _name, ".json");
        if (vm.exists(path)) return path;
        return vm.readDir(directory)[0].path;
    }

    /// @notice Returns the storage slot for a given contract and slot name.
    function getSlot(
        string memory _contractName,
        string memory _slotName
    )
        internal
        view
        returns (StorageSlot memory slot_)
    {
        string memory artifact = vm.readFile(_getForgeArtifactPath(_contractName));
        bytes memory raw = vm.parseJson(artifact, ".storageLayout.storage");
        ForgeStorageSlot[] memory slots = abi.decode(raw, (ForgeStorageSlot[]));
        bytes32 wantHash = keccak256(bytes(_slotName));
        for (uint256 i = 0; i < slots.length; i++) {
            if (keccak256(bytes(slots[i].label)) == wantHash) {
                return StorageSlot({ offset: slots[i].offset, slot: vm.parseUint(slots[i].slot) });
            }
        }
        revert(string.concat("ForgeArtifacts: slot not found for ", _contractName, ".", _slotName));
    }

    /// @notice Returns whether or not a contract is initialized (OZ v4 layout).
    function isInitialized(string memory _name, address _address) internal view returns (bool) {
        StorageSlot memory slot = getSlot(_name, "_initialized");
        bytes32 slotVal = vm.load(_address, bytes32(slot.slot));
        return uint8((uint256(slotVal) >> (slot.offset * 8)) & 0xFF) != 0;
    }

    /// @notice Returns whether or not a contract is initialized using the OZ v5 namespaced
    ///         Initializable storage slot:
    ///         keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.Initializable")) - 1)) &
    /// ~bytes32(uint256(0xff))
    function isInitializedV5(address _addr) internal view returns (bool) {
        bytes32 INITIALIZABLE_STORAGE_SLOT = 0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
        bytes32 slotVal = vm.load(_addr, INITIALIZABLE_STORAGE_SLOT);
        return uint8(uint256(slotVal) & 0xFF) != 0;
    }

    /// @notice Returns the names of all contracts in a given directory.
    /// @param _path The path to search for contracts.
    /// @param _pathExcludes An array of paths to exclude from the search.
    /// @return contractNames_ An array of contract names.
    function getContractNames(
        string memory _path,
        string[] memory _pathExcludes
    )
        internal
        returns (string[] memory contractNames_)
    {
        string memory pathExcludesPat;
        for (uint256 i = 0; i < _pathExcludes.length; i++) {
            if (i > 0) pathExcludesPat = string.concat(pathExcludesPat, " -o ");
            pathExcludesPat = string.concat(pathExcludesPat, " -path \"", _pathExcludes[i], "\"");
        }

        contractNames_ = abi.decode(
            vm.parseJson(
                _bash(
                    string.concat(
                        "find ",
                        _path,
                        bytes(pathExcludesPat).length > 0 ? string.concat(" ! \\( ", pathExcludesPat, " \\)") : "",
                        " -type f -exec basename {} \\; | sed 's/\\.[^.]*$//' | jq -R -s 'split(\"\n\")[:-1]'"
                    )
                )
            ),
            (string[])
        );
    }

    /// @notice Accepts a filepath and then ensures that the directory
    ///         exists for the file to live in.
    function ensurePath(string memory _path) internal {
        string[] memory outputs = vm.split(_path, "/");
        string memory dir = "";
        for (uint256 i = 0; i < outputs.length - 1; i++) {
            dir = string.concat(dir, outputs[i], "/");
        }
        vm.createDir(dir, true);
    }

    function _bash(string memory _command) private returns (string memory stdout_) {
        string[] memory command = new string[](3);
        command[0] = "bash";
        command[1] = "-c";
        command[2] = _command;
        stdout_ = string(vm.ffi(command));
    }
}

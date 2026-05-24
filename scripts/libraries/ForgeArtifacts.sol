// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Vm } from "lib/forge-std/src/Vm.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";

/// @notice Contains information about a storage slot. Mirrors the layout of the storage
///         slot object in Forge artifacts so that we can deserialize JSON into this struct.
///         Field order matches the alphabetical JSON key order produced by `vm.parseJson`.
///         IMPORTANT: Do not reorder fields — Foundry's `vm.parseJson` deserializes JSON
///         objects in alphabetical key order, so the struct field order must match to
///         avoid silent decoding errors.
struct ForgeStorageSlot {
    uint256 astId;
    string _contract;
    string label;
    uint256 offset;
    string slot;
    string _type;
}

/// @notice Minimal storage slot information consumed by tests.
///         Unlike ForgeStorageSlot, this uses uint256 for the slot field to simplify
///         comparisons and arithmetic in test assertions.
struct StorageSlot {
    uint256 offset;
    uint256 slot;
}

/// @title ForgeArtifacts
/// @notice Library for interacting with the forge artifacts.
///         Provides utilities to read compiled contract metadata (ABI, storage layout,
///         devdoc tags) from the `forge-artifacts/` output directory. These helpers are
///         primarily used in integration tests and deployment scripts to verify contract
///         state, check initialization status, and discover contract names.
library ForgeArtifacts {
    /// @notice Foundry cheatcode VM. Used for filesystem operations (vm.exists,
    ///         vm.readFile, vm.readDir) and state inspection (vm.load).
    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Forge artifact output directory. Must match `out` in foundry.toml.
    ///         This directory contains per-contract JSON files produced by `forge build`.
    string private constant OUT_DIR = "forge-artifacts";

    /// @notice Removes the semantic versioning suffix from a contract name. The semver appears
    ///         when the contract is compiled more than once with different solc versions.
    ///         For example, "L1StandardBridge.v0.8.20" becomes "L1StandardBridge".
    ///         This is needed because Foundry appends the compiler version to the artifact
    ///         directory name when multiple solc versions are used, but we always want the
    ///         canonical contract name for lookups.
    /// @param _name The contract name, potentially with a semver suffix.
    /// @return The contract name with the semver suffix removed.
    function _stripSemver(string memory _name) private pure returns (string memory) {
        return vm.split(_name, ".")[0];
    }

    /// @notice Returns the abi from the forge artifact.
    ///         Uses `jq` to extract the `.abi` field from the compiled artifact JSON.
    /// @param _name The name of the contract.
    /// @return abi_ The ABI as a JSON string.
    function getAbi(string memory _name) internal returns (string memory abi_) {
        abi_ = _bash(string.concat("jq -r '.abi' < ", _getForgeArtifactPath(_name)));
    }

    /// @notice Returns the kind of contract (i.e. library, contract, or interface).
    ///         Extracts the `contractKind` field from the AST's ContractDefinition node.
    ///         This is useful for filtering contracts by type in deployment scripts.
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
    ///         A "proxied" contract is one that is deployed behind a proxy (e.g., UPGRADEABLE_PROXY).
    ///         This tag is set in the contract's NatSpec as `@custom:proxied`.
    function isProxiedContract(string memory _name) internal returns (bool) {
        return _hasDevdocTag(_name, "custom:proxied");
    }

    /// @notice Returns whether or not a contract is predeployed. Heuristic based on the
    ///         custom:predeploy devdoc tag; deployment script would be a more reliable source.
    ///         A "predeployed" contract is one that is pre-installed at a fixed address on L2
    ///         (e.g., the L2StandardBridge at address 0x420...). This tag is set in the
    ///         contract's NatSpec as `@custom:predeploy`.
    function isPredeployedContract(string memory _name) internal returns (bool) {
        return _hasDevdocTag(_name, "custom:predeploy");
    }

    /// @notice Checks whether a given devdoc tag exists in the contract's metadata.
    ///         Parses the `rawMetadata` JSON field, decodes the embedded devdoc object,
    ///         and checks for the presence of the specified key.
    /// @param _name The contract name to check.
    /// @param _tag The devdoc tag key to look for (e.g., "custom:proxied").
    /// @return True if the tag exists in the contract's devdoc.
    function _hasDevdocTag(string memory _name, string memory _tag) private returns (bool) {
        string memory res = _bash(
            string.concat(
                "jq -r '.rawMetadata | fromjson | .output.devdoc | has(\"", _tag, "\")' ", _getForgeArtifactPath(_name)
            )
        );
        return stdJson.readBool(res, "");
    }

    /// @notice Returns the directory containing forge artifacts for a given contract.
    ///         The directory path follows the pattern: <projectRoot>/forge-artifacts/<ContractName>.sol/
    /// @param _name The contract name used to construct the directory path.
    /// @return The filesystem path to the artifact directory.
    function _getForgeArtifactDirectory(string memory _name) private view returns (string memory) {
        return string.concat(vm.projectRoot(), "/", OUT_DIR, "/", _stripSemver(_name), ".sol");
    }

    /// @notice Returns the filesystem path to the artifact JSON file. If the contract was compiled
    ///         with multiple solidity versions then return the first entry in the directory.
    ///         When multiple solc versions are used, Foundry creates separate artifact files
    ///         (e.g., "ContractName.v0.8.20.json", "ContractName.v0.8.25.json"). We prefer
    ///         the exact match first, then fall back to the first file in the directory.
    /// @param _name The contract name.
    /// @return The path to the artifact JSON file.
    function _getForgeArtifactPath(string memory _name) private view returns (string memory) {
        string memory directory = _getForgeArtifactDirectory(_name);
        string memory path = string.concat(directory, "/", _name, ".json");
        if (vm.exists(path)) return path;
        // Fallback: use the first artifact file in the directory (for multi-solc builds).
        return vm.readDir(directory)[0].path;
    }

    /// @notice Returns the storage slot for a given contract and slot name.
    ///         Reads the storage layout from the compiled artifact and searches for a slot
    ///         whose label matches `_slotName`. This is useful for tests that need to read
    ///         or verify storage values at specific slots (e.g., checking initialization state).
    /// @param _contractName The name of the compiled contract.
    /// @param _slotName The storage variable name to find (e.g., "_initialized", "_owner").
    /// @return slot_ The StorageSlot containing the offset (in bytes within the slot) and
    ///         the slot number (as a uint256).
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
        // Compare slot labels by hash to avoid unbounded string comparison.
        bytes32 wantHash = keccak256(bytes(_slotName));
        for (uint256 i = 0; i < slots.length; i++) {
            if (keccak256(bytes(slots[i].label)) == wantHash) {
                return StorageSlot({ offset: slots[i].offset, slot: vm.parseUint(slots[i].slot) });
            }
        }
        revert(string.concat("ForgeArtifacts: slot not found for ", _contractName, ".", _slotName));
    }

    /// @notice Returns whether or not a contract is initialized (OZ v4 layout).
    ///         Checks the `_initialized` storage slot using the OpenZeppelin v4 storage layout,
    ///         where `_initialized` is packed with `_initializing` in slot 0.
    ///         The value is a uint8 stored at the given offset within the slot.
    /// @param _name The contract name (used to look up the storage slot).
    /// @param _address The contract address to inspect.
    /// @return True if the contract's `_initialized` flag is non-zero.
    function isInitialized(string memory _name, address _address) internal view returns (bool) {
        StorageSlot memory slot = getSlot(_name, "_initialized");
        // Read the raw 32-byte storage slot value.
        bytes32 slotVal = vm.load(_address, bytes32(slot.slot));
        // Extract the uint8 at the byte offset. OZ v4 packs `_initialized` and
        // `_initializing` into the same 32-byte slot, so we must shift and mask.
        return uint8((uint256(slotVal) >> (slot.offset * 8)) & 0xFF) != 0;
    }

    /// @notice Returns whether or not a contract is initialized using the OZ v5 namespaced
    ///         Initializable storage slot:
    ///         keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.Initializable")) - 1)) &
    ///         ~bytes32(uint256(0xff))
    ///         In OZ v5, storage is namespaced to avoid collisions in upgradeable proxies.
    ///         The `_initialized` value occupies the least-significant byte of this slot.
    /// @param _addr The contract address to inspect.
    /// @return True if the contract's `_initialized` flag is non-zero.
    function isInitializedV5(address _addr) internal view returns (bool) {
        // This is the deterministic storage slot computed by OZ v5's namespaced layout.
        // See: https://docs.openzeppelin.com/contracts/5.x/upgradeable#storage_gaps
        bytes32 INITIALIZABLE_STORAGE_SLOT = 0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
        bytes32 slotVal = vm.load(_addr, INITIALIZABLE_STORAGE_SLOT);
        // The `_initialized` flag is stored in the least-significant byte.
        return uint8(uint256(slotVal) & 0xFF) != 0;
    }

    /// @notice Returns the names of all contracts in a given directory.
    ///         Uses `find` to locate Solidity files, then strips extensions with `sed`,
    ///         and formats the result as a JSON array using `jq`.
    ///         This is useful for iterating over all contracts in deployment scripts.
    /// @param _path The path to search for contracts.
    /// @param _pathExcludes An array of paths to exclude from the search.
    /// @return contractNames_ An array of contract names (without file extensions).
    function getContractNames(
        string memory _path,
        string[] memory _pathExcludes
    )
        internal
        returns (string[] memory contractNames_)
    {
        // Build a `find` exclusion pattern from the provided paths.
        // Each exclude becomes a `-path "<path>"` clause joined by `-o` (OR).
        string memory pathExcludesPat;
        for (uint256 i = 0; i < _pathExcludes.length; i++) {
            if (i > 0) pathExcludesPat = string.concat(pathExcludesPat, " -o ");
            pathExcludesPat = string.concat(pathExcludesPat, " -path \"", _pathExcludes[i], "\"");
        }

        // find <path> [! \( -path X -o -path Y \)] -type f -exec basename {} \;
        //   | sed 's/\.[^.]*$//'   — strip file extension
        //   | jq -R -s 'split("\n")[:-1]' — convert lines to JSON array
        contractNames_ = abi.decode(
            vm.parseJson(
                _bash(
                    string.concat(
                        "find ",
                        _path,
                        bytes(pathExcludesPat).length > 0 ? string.concat(" ! \\ ( ", pathExcludesPat, " \\ )") : "",
                        " -type f -exec basename {} \\; | sed 's/\\.[^.]*$//' | jq -R -s 'split(\"\\n\")[:-1]'"
                    )
                )
            ),
            (string[])
        );
    }

    /// @notice Accepts a filepath and then ensures that the directory
    ///         exists for the file to live in. Creates intermediate directories
    ///         as needed (similar to `mkdir -p`). This is useful before writing
    ///         artifact files to ensure the target directory structure exists.
    /// @param _path The full file path (the directory portion will be created).
    function ensurePath(string memory _path) internal {
        string[] memory outputs = vm.split(_path, "/");
        string memory dir = "";
        // Reconstruct the directory path from all path segments except the last
        // (which is the filename itself).
        for (uint256 i = 0; i < outputs.length - 1; i++) {
            dir = string.concat(dir, outputs[i], "/");
        }
        vm.createDir(dir, true); // recursive = true
    }

    /// @notice Executes a shell command via Foundry's `vm.ffi` and returns stdout.
    ///         This is a convenience wrapper that prefixes the command with `bash -c`
    ///         to enable shell features like pipes and redirects.
    ///         WARNING: FFI is inherently unsafe and should only be used in test scripts.
    /// @param _command The shell command to execute.
    /// @return stdout_ The command's standard output.
    function _bash(string memory _command) private returns (string memory stdout_) {
        string[] memory command = new string[](3);
        command[0] = "bash";
        command[1] = "-c";
        command[2] = _command;
        stdout_ = string(vm.ffi(command));
    }
}

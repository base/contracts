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
/// @dev Provides utilities for reading contract ABIs, storage layouts, and metadata
///      from Foundry's build output directory. Uses FFI to invoke shell commands
///      for JSON parsing, since Foundry's cheatcodes alone cannot extract arbitrary
///      fields from artifact JSON files.
library ForgeArtifacts {
    /// @notice Foundry cheatcode VM.
    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Forge artifact output directory. Must match `out` in foundry.toml.
    string private constant OUT_DIR = "forge-artifacts";

    /// @notice Removes the semantic versioning suffix from a contract name. The semver appears
    ///         when the contract is compiled more than once with different solc versions.
    ///         For example, "L1StandardBridge.0.8.25" becomes "L1StandardBridge".
    /// @param _name The contract name, potentially with a semver suffix (e.g. "MyContract.0.8.25").
    /// @return The contract name with the semver suffix stripped.
    function _stripSemver(string memory _name) private pure returns (string memory) {
        return vm.split(_name, ".")[0];
    }

    /// @notice Returns the abi from the forge artifact.
    /// @dev Uses jq to extract the ".abi" field from the artifact JSON file.
    /// @param _name The name of the contract to get the ABI for.
    /// @return abi_ The JSON-encoded ABI string.
    function getAbi(string memory _name) internal returns (string memory abi_) {
        abi_ = _bash(string.concat("jq -r '.abi' < ", _getForgeArtifactPath(_name)));
    }

    /// @notice Returns the kind of contract (i.e. library, contract, or interface).
    /// @param _name The name of the contract to get the kind of.
    /// @return kind_ The kind of contract ("library", "contract", or "interface").
    /// @dev Queries the AST nodes in the artifact JSON to find the ContractDefinition
    ///      node and extracts its contractKind field.
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
    /// @dev This relies on the contract's natspec devdoc containing a `@custom:proxied`
    ///      tag. If the tag is missing, this will return false even if the contract
    ///      is actually deployed behind a proxy. Prefer checking deployment scripts
    ///      for authoritative proxy status.
    function isProxiedContract(string memory _name) internal returns (bool) {
        return _hasDevdocTag(_name, "custom:proxied");
    }

    /// @notice Returns whether or not a contract is predeployed. Heuristic based on the
    ///         custom:predeploy devdoc tag; deployment script would be a more reliable source.
    /// @dev This relies on the contract's natspec devdoc containing a `@custom:predeploy`
    ///      tag. If the tag is missing, this will return false even if the contract
    ///      is actually a predeploy. Prefer checking deployment scripts for authoritative
    ///      predeploy status.
    function isPredeployedContract(string memory _name) internal returns (bool) {
        return _hasDevdocTag(_name, "custom:predeploy");
    }

    /// @notice Checks whether a given devdoc tag exists in the contract's metadata.
    /// @dev Parses the rawMetadata JSON field from the artifact and checks if the
    ///      specified tag key exists in the devdoc section. Returns false if the
    ///      tag is absent or if the metadata cannot be parsed.
    /// @param _name The name of the contract.
    /// @param _tag The devdoc tag to check for (e.g. "custom:proxied").
    /// @return Whether the tag exists in the contract's devdoc.
    function _hasDevdocTag(string memory _name, string memory _tag) private returns (bool) {
        string memory res = _bash(
            string.concat(
                "jq -r '.rawMetadata | fromjson | .output.devdoc | has(\"", _tag, "\")' ", _getForgeArtifactPath(_name)
            )
        );
        return stdJson.readBool(res, "");
    }

    /// @notice Returns the directory containing forge artifacts for a given contract name.
    /// @dev Constructs the path as `<projectRoot>/<OUT_DIR>/<strippedName>.sol`.
    ///      The semver suffix is stripped from the contract name to form the directory name.
    function _getForgeArtifactDirectory(string memory _name) private view returns (string memory) {
        return string.concat(vm.projectRoot(), "/", OUT_DIR, "/", _stripSemver(_name), ".sol");
    }

    /// @notice Returns the filesystem path to the artifact JSON file. If the contract was compiled
    ///         with multiple solidity versions then return the first entry in the directory.
    /// @dev First attempts to find an exact match at `<dir>/<name>.json`. If that file does
    ///      not exist (which happens when multiple solc versions produce artifacts), falls
    ///      back to the first file listed in the artifact directory. This fallback is
    ///      nondeterministic if multiple versions exist, but in practice the first entry
    ///      is typically sufficient for test purposes.
    function _getForgeArtifactPath(string memory _name) private view returns (string memory) {
        string memory directory = _getForgeArtifactDirectory(_name);
        string memory path = string.concat(directory, "/", _name, ".json");
        if (vm.exists(path)) return path;
        // Fallback: return the first artifact file in the directory when the exact
        // path doesn't exist (e.g., due to multiple solc version compilations).
        return vm.readDir(directory)[0].path;
    }

    /// @notice Returns the storage slot for a given contract and slot name.
    /// @dev Iterates through all storage slots in the artifact's storage layout and
    ///      matches by label using keccak256 comparison. Reverts if the slot is not found.
    ///      The keccak256 comparison avoids string comparison issues and is gas-efficient.
    /// @param _contractName The name of the contract to look up the slot for.
    /// @param _slotName The variable name (label) of the storage slot.
    /// @return slot_ The StorageSlot struct containing offset and slot number.
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
    /// @dev Reads the `_initialized` storage slot from the contract at `_address` and checks
    ///      if the value is non-zero. Uses the OZ v4 storage layout where `_initialized`
    ///      is packed with `_initializing` in the same slot.
    ///      The bit shift extracts the byte at the given offset: `(value >> (offset * 8)) & 0xFF`
    ///      masks out all bits except the lowest byte, which holds the initialization version.
    /// @param _name The contract name, used to look up the storage slot layout.
    /// @param _address The deployed address of the contract to check.
    /// @return Whether the contract has been initialized (version > 0).
    function isInitialized(string memory _name, address _address) internal view returns (bool) {
        StorageSlot memory slot = getSlot(_name, "_initialized");
        bytes32 slotVal = vm.load(_address, bytes32(slot.slot));
        // Extract the byte at the packed offset and check if it's non-zero.
        // OZ v4 packs _initialized (uint8) and _initializing (bool) into the same slot.
        return uint8((uint256(slotVal) >> (slot.offset * 8)) & 0xFF) != 0;
    }

    /// @notice Returns whether or not a contract is initialized using the OZ v5 namespaced
    ///         Initializable storage slot:
    ///         keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.Initializable")) - 1)) &
    ///         ~bytes32(uint256(0xff))
    /// @dev OZ v5 uses a namespaced storage pattern to avoid slot collisions in upgradeable
    ///      contracts. The slot is computed deterministically from the namespace string.
    ///      The `& ~0xff` clears the bottom byte, which is where the `_initialized` version
    ///      counter is stored. A non-zero bottom byte indicates the contract has been initialized.
    /// @param _addr The deployed address of the contract to check.
    /// @return Whether the contract has been initialized (version > 0).
    function isInitializedV5(address _addr) internal view returns (bool) {
        bytes32 INITIALIZABLE_STORAGE_SLOT = 0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
        bytes32 slotVal = vm.load(_addr, INITIALIZABLE_STORAGE_SLOT);
        // The lowest byte of the namespaced slot holds the initialization version (uint8).
        // If it's non-zero, the contract has been initialized at least once.
        return uint8(uint256(slotVal) & 0xFF) != 0;
    }

    /// @notice Returns the names of all contracts in a given directory.
    /// @param _path The path to search for contracts.
    /// @param _pathExcludes An array of paths to exclude from the search.
    /// @return contractNames_ An array of contract names (without file extensions).
    /// @dev Uses `find` to locate Solidity files, then strips extensions with `sed`,
    ///      and converts the result to a JSON array with `jq`. Paths in `_pathExcludes`
    ///      are combined into a single `find ! \( -path ... -o -path ... \)` expression.
    function getContractNames(
        string memory _path,
        string[] memory _pathExcludes
    )
        internal
        returns (string[] memory contractNames_)
    {
        // Build a find-compatible exclude pattern from the array of paths.
        // Each excluded path becomes a `-path "<path>"` clause, joined with `-o` (OR).
        string memory pathExcludesPat;
        for (uint256 i = 0; i < _pathExcludes.length; i++) {
            if (i > 0) pathExcludesPat = string.concat(pathExcludesPat, " -o ");
            pathExcludesPat = string.concat(pathExcludesPat, " -path \"", _pathExcludes[i], "\"");
        }

        // Find all files in _path (excluding _pathExcludes), strip extensions,
        // and convert the newline-separated list into a JSON array via jq.
        contractNames_ = abi.decode(
            vm.parseJson(
                _bash(
                    string.concat(
                        "find ",
                        _path,
                        bytes(pathExcludesPat).length > 0 ? string.concat(" ! \\( ", pathExcludesPat, " \\)" ) : "",
                        " -type f -exec basename {} \\; | sed 's/\\.[^.]*$//' | jq -R -s 'split(\"\\n\")[:-1]'"
                    )
                )
            ),
            (string[])
        );
    }

    /// @notice Accepts a filepath and then ensures that the directory
    ///         exists for the file to live in.
    /// @dev Creates all intermediate directories if they don't already exist.
    ///      The `true` parameter to `vm.createDir` makes it recursive (like `mkdir -p`).
    function ensurePath(string memory _path) internal {
        string[] memory outputs = vm.split(_path, "/");
        string memory dir = "";
        for (uint256 i = 0; i < outputs.length - 1; i++) {
            dir = string.concat(dir, outputs[i], "/");
        }
        vm.createDir(dir, true);
    }

    /// @notice Executes a shell command via Foundry's `vm.ffi` cheatcode.
    /// @dev Invokes `bash -c <command>` and returns the stdout output as a string.
    ///      This requires `ffi = true` in the Foundry configuration. Shell commands
    ///      are used here because Foundry's built-in JSON parsing cheatcodes cannot
    ///      extract arbitrary nested fields or run complex queries like `jq`.
    /// @param _command The shell command to execute.
    /// @return stdout_ The stdout output of the command.
    function _bash(string memory _command) private returns (string memory stdout_) {
        string[] memory command = new string[](3);
        command[0] = "bash";
        command[1] = "-c";
        command[2] = _command;
        stdout_ = string(vm.ffi(command));
    }
}

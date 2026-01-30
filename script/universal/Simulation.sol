// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Vm} from "lib/forge-std/src/Vm.sol";

import {IGnosisSafe} from "./IGnosisSafe.sol";

/// @title Simulation
///
/// @notice Library for simulating multisig transactions with state overrides in Foundry
///
/// @dev This library provides utilities for:
///      - Simulating multisig transactions before execution
///      - Overriding contract storage states for testing scenarios
///      - Generating Tenderly simulation links for external transaction analysis
///      - Managing Gnosis Safe parameters (threshold, nonce, approvals) during simulation
library Simulation {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                          Safe Storage Slot Constants                           ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Storage slot for Safe's signature threshold
    /// @dev Slot 4 in Safe's storage layout. The threshold determines how many owner signatures
    ///      are required to execute a transaction.
    ///      Valid for: Safe v1.3.0 - v1.4.1
    /// @custom:warning These storage slots are internal implementation details of Safe contracts
    ///                 and may change in future Safe versions. Verify compatibility before use.
    bytes32 internal constant SAFE_THRESHOLD_SLOT = bytes32(uint256(4));

    /// @notice Storage slot for Safe's transaction nonce
    /// @dev Slot 5 in Safe's storage layout. The nonce is incremented after each successful
    ///      transaction execution to prevent replay attacks.
    ///      Valid for: Safe v1.3.0 - v1.4.1
    /// @custom:warning These storage slots are internal implementation details of Safe contracts
    ///                 and may change in future Safe versions. Verify compatibility before use.
    bytes32 internal constant SAFE_NONCE_SLOT = bytes32(uint256(5));

    /// @notice Base storage slot for Safe's approvedHashes mapping
    /// @dev Slot 8 in Safe's storage layout. The approvedHashes mapping stores pre-approved
    ///      transaction hashes per owner: mapping(address => mapping(bytes32 => uint256))
    ///      Valid for: Safe v1.3.0 - v1.4.1
    /// @custom:warning These storage slots are internal implementation details of Safe contracts
    ///                 and may change in future Safe versions. Verify compatibility before use.
    uint256 internal constant SAFE_APPROVED_HASHES_SLOT = 8;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Structs                                          ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Represents state overrides for a specific contract during simulation. Used to modify contract storage
    ///         slots temporarily for testing purposes
    struct StateOverride {
        /// @dev The address of the contract whose state will be overridden
        address contractAddress;
        /// @dev Array of storage slot overrides to apply to this contract
        StorageOverride[] overrides;
    }

    /// @notice Represents a single storage slot override. Maps a storage slot key to a new value during simulation
    struct StorageOverride {
        /// @dev The storage slot key
        bytes32 key;
        /// @dev The new value to store in the slot during simulation
        bytes32 value;
    }

    /// @notice Contains all parameters needed to execute a simulation. Encapsulates transaction data and state
    ///         modifications for simulation execution
    struct Payload {
        /// @dev Address that will appear as the transaction sender
        address from;
        /// @dev Target contract address for the transaction
        address to;
        /// @dev Encoded transaction data to execute
        bytes data;
        /// @dev Array of state overrides to apply before simulation
        StateOverride[] stateOverrides;
    }

    /// @notice Foundry VM instance for state manipulation during simulations
    Vm internal constant VM = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    //////////////////////////////////////////////////////////////////////////////////////
    ///                          Storage Slot Helpers                                  ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Computes the storage slot key for a Safe's approvedHashes mapping entry
    ///
    /// @dev Safe's approvedHashes is a nested mapping: mapping(address => mapping(bytes32 => uint256))
    ///      For Solidity mappings, the storage slot is computed as:
    ///        slot(mapping[key]) = keccak256(abi.encode(key, baseSlot))
    ///      For nested mappings, this is applied recursively:
    ///        slot(mapping[owner][dataHash]) = keccak256(abi.encode(dataHash, keccak256(abi.encode(owner, baseSlot))))
    ///
    ///      The formula breaks down as:
    ///        1. innerSlot = keccak256(abi.encode(owner, SAFE_APPROVED_HASHES_SLOT))
    ///           This gives the slot for the inner mapping (owner's approved hashes)
    ///        2. finalSlot = keccak256(abi.encode(dataHash, innerSlot))
    ///           This gives the slot for the specific dataHash within owner's mapping
    ///
    /// @param owner    The address of the Safe owner who approved the hash
    /// @param dataHash The transaction hash that was approved
    ///
    /// @return The storage slot key where the approval value (0 or 1) is stored
    function computeApprovedHashSlot(address owner, bytes32 dataHash) internal pure returns (bytes32) {
        bytes32 innerMappingSlot = keccak256(abi.encode(owner, SAFE_APPROVED_HASHES_SLOT));
        return keccak256(abi.encode(dataHash, innerMappingSlot));
    }

    /// @notice Executes a simulation using the provided payload and returns state changes
    ///
    /// @dev This is the core simulation function that applies state overrides and executes the transaction
    ///
    /// @param simPayload The simulation payload containing transaction data and state overrides
    ///
    /// @return accesses Array of account access records showing all state changes during simulation
    function simulateFromSimPayload(Payload memory simPayload) internal returns (Vm.AccountAccess[] memory) {
        // solhint-disable-next-line max-line-length
        require(simPayload.from != address(0), "Simulator::simulateFromSimPayload: from address cannot be zero address");
        require(simPayload.to != address(0), "Simulator::simulateFromSimPayload: to address cannot be zero address");

        // Apply state overrides.
        StateOverride[] memory stateOverrides = simPayload.stateOverrides;
        for (uint256 i; i < stateOverrides.length; i++) {
            StateOverride memory stateOverride = stateOverrides[i];
            StorageOverride[] memory storageOverrides = stateOverride.overrides;
            for (uint256 j; j < storageOverrides.length; j++) {
                StorageOverride memory storageOverride = storageOverrides[j];
                VM.store({
                    target: stateOverride.contractAddress, slot: storageOverride.key, value: storageOverride.value
                });
            }
        }

        // Execute the call in forge and return the state diff.
        VM.startStateDiffRecording();
        VM.prank({msgSender: simPayload.from});
        (bool ok, bytes memory returnData) = address(simPayload.to).call(simPayload.data);
        Vm.AccountAccess[] memory accesses = VM.stopAndReturnStateDiff();
        require(ok, string.concat("Simulator::simulateFromSimPayload failed: ", VM.toString({value: returnData})));
        require(accesses.length > 0, "Simulator::simulateFromSimPayload: No state changes");
        return accesses;
    }

    /// @notice Creates a comprehensive state override for a Gnosis Safe including threshold, nonce, and approval
    ///
    /// @dev Combines multiple overrides: sets threshold to 1, updates nonce, and pre-approves transaction hash
    ///
    /// @param safe     The address of the Gnosis Safe to override
    /// @param nonce    The nonce value to set for the safe
    /// @param owner    The owner address that should appear to have approved the transaction
    /// @param dataHash The transaction hash that should appear as pre-approved
    ///
    /// @return state StateOverride struct containing all the necessary storage overrides
    function overrideSafeThresholdApprovalAndNonce(address safe, uint256 nonce, address owner, bytes32 dataHash)
        internal
        view
        returns (StateOverride memory)
    {
        // solhint-disable-next-line max-line-length
        StateOverride memory state = StateOverride({contractAddress: safe, overrides: new StorageOverride[](0)});
        state = addThresholdOverride({safe: safe, state: state});
        state = addNonceOverride({safe: safe, state: state, nonce: nonce});
        state = addApprovalOverride({state: state, owner: owner, dataHash: dataHash});
        return state;
    }

    /// @notice Creates a state override for a Gnosis Safe's threshold and nonce only
    ///
    /// @dev Sets the safe's threshold to 1 and updates its nonce for simulation purposes
    ///
    /// @param safe  The address of the Gnosis Safe to override
    /// @param nonce The nonce value to set for the safe
    ///
    /// @return state StateOverride struct containing threshold and nonce overrides
    function overrideSafeThresholdAndNonce(address safe, uint256 nonce) internal view returns (StateOverride memory) {
        StateOverride memory state = StateOverride({contractAddress: safe, overrides: new StorageOverride[](0)});
        state = addThresholdOverride({safe: safe, state: state});
        state = addNonceOverride({safe: safe, state: state, nonce: nonce});
        return state;
    }

    /// @notice Adds a transaction approval override to the state
    ///
    /// @dev Simulates that the specified owner has already approved the given transaction hash
    ///
    /// @param state    The existing state override to modify
    /// @param owner    The address of the owner who should appear to have approved
    /// @param dataHash The transaction hash that should appear as approved
    ///
    /// @return _ StateOverride struct with the approval override added
    function addApprovalOverride(StateOverride memory state, address owner, bytes32 dataHash)
        internal
        pure
        returns (StateOverride memory)
    {
        return addOverride({
            state: state,
            storageOverride: StorageOverride({
                key: computeApprovedHashSlot({owner: owner, dataHash: dataHash}),
                value: bytes32(uint256(0x1))
            })
        });
    }

    /// @notice Adds a threshold override to set the safe's signature threshold to 1
    ///
    /// @dev Only adds the override if the current threshold is not already 1
    ///
    /// @param safe  The address of the Gnosis Safe to check and potentially override
    /// @param state The existing state override to modify
    ///
    /// @return _ StateOverride struct with threshold override added (if needed)
    function addThresholdOverride(address safe, StateOverride memory state)
        internal
        view
        returns (StateOverride memory)
    {
        // get the threshold and check if we need to override it
        if (IGnosisSafe(safe).getThreshold() == 1) return state;

        // set the threshold to 1
        return addOverride({
            state: state, storageOverride: StorageOverride({key: SAFE_THRESHOLD_SLOT, value: bytes32(uint256(0x1))})
        });
    }

    /// @notice Adds a nonce override to set the safe's transaction nonce
    ///
    /// @dev Only adds the override if the current nonce differs from the desired value
    ///
    /// @param safe  The address of the Gnosis Safe to check and potentially override
    /// @param state The existing state override to modify
    /// @param nonce The nonce value to set for the safe
    ///
    /// @return _ StateOverride struct with nonce override added (if needed)
    function addNonceOverride(address safe, StateOverride memory state, uint256 nonce)
        internal
        view
        returns (StateOverride memory)
    {
        // get the nonce and check if we need to override it
        if (IGnosisSafe(safe).nonce() == nonce) return state;

        // set the nonce to the desired value
        return addOverride({
            state: state, storageOverride: StorageOverride({key: SAFE_NONCE_SLOT, value: bytes32(nonce)})
        });
    }

    /// @notice Appends a new storage override to an existing state override
    ///
    /// @dev Creates a new array with the additional override appended
    ///
    /// @param state           The existing state override to extend
    /// @param storageOverride The new storage override to add
    ///
    /// @return _ StateOverride struct with the new override added to the array
    function addOverride(StateOverride memory state, StorageOverride memory storageOverride)
        internal
        pure
        returns (StateOverride memory)
    {
        StorageOverride[] memory overrides = new StorageOverride[](state.overrides.length + 1);
        for (uint256 i; i < state.overrides.length; i++) {
            overrides[i] = state.overrides[i];
        }
        overrides[state.overrides.length] = storageOverride;
        return StateOverride({contractAddress: state.contractAddress, overrides: overrides});
    }

    /// @notice Generates and logs a Tenderly simulation link without state overrides
    ///
    /// @dev Convenience function that calls the full logSimulationLink with empty overrides
    ///
    /// @param to   The target contract address for the simulation
    /// @param data The transaction data to simulate
    /// @param from The address that will appear as the transaction sender
    function logSimulationLink(address to, bytes memory data, address from) internal view {
        logSimulationLink({to: to, data: data, from: from, overrides: new StateOverride[](0)});
    }

    /// @notice Generates and logs a Tenderly simulation link with state overrides
    ///
    /// @dev Creates a properly formatted URL for Tenderly's transaction simulator with state modifications
    ///
    /// @param to        The target contract address for the simulation
    /// @param data      The transaction data to simulate
    /// @param from      The address that will appear as the transaction sender
    /// @param overrides Array of state overrides to apply during simulation
    function logSimulationLink(address to, bytes memory data, address from, StateOverride[] memory overrides)
        internal
        view
    {
        string memory proj = VM.envOr({name: "TENDERLY_PROJECT", defaultValue: string("TENDERLY_PROJECT")});
        string memory username = VM.envOr({name: "TENDERLY_USERNAME", defaultValue: string("TENDERLY_USERNAME")});
        bool includeOverrides;

        // the following characters are url encoded: []{}
        string memory stateOverrides = "%5B";
        for (uint256 i; i < overrides.length; i++) {
            StateOverride memory _override = overrides[i];

            if (_override.overrides.length == 0) {
                continue;
            }

            includeOverrides = true;

            if (i > 0) stateOverrides = string.concat(stateOverrides, ",");
            stateOverrides = string.concat(
                stateOverrides,
                "%7B\"contractAddress\":\"",
                VM.toString({value: _override.contractAddress}),
                "\",\"storage\":%5B"
            );
            for (uint256 j; j < _override.overrides.length; j++) {
                if (j > 0) stateOverrides = string.concat(stateOverrides, ",");
                stateOverrides = string.concat(
                    stateOverrides,
                    "%7B\"key\":\"",
                    VM.toString({value: _override.overrides[j].key}),
                    "\",\"value\":\"",
                    VM.toString({value: _override.overrides[j].value}),
                    "\"%7D"
                );
            }
            stateOverrides = string.concat(stateOverrides, "%5D%7D");
        }
        stateOverrides = string.concat(stateOverrides, "%5D");

        string memory str = string.concat(
            "https://dashboard.tenderly.co/",
            username,
            "/",
            proj,
            "/simulator/new?network=",
            VM.toString({value: block.chainid}),
            "&contractAddress=",
            VM.toString({value: to}),
            "&from=",
            VM.toString({value: from})
        );

        if (includeOverrides) {
            str = string.concat(str, "&stateOverrides=", stateOverrides);
        }

        if (bytes(str).length + data.length * 2 > 7980) {
            // tenderly's nginx has issues with long URLs, so print the raw input data separately
            str = string.concat(str, "\nInsert the following hex into the 'Raw input data' field:");
            console.log(str);
            console.log(VM.toString({value: data}));
        } else {
            str = string.concat(str, "&rawFunctionInput=", VM.toString({value: data}));
            console.log(str);
        }
    }
}

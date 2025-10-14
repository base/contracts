// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Vm} from "lib/forge-std/src/Vm.sol";

import {Simulation} from "./Simulation.sol";

library StateDiff {
    struct MappingParent {
        bytes32 slot;
        bytes32 parent;
        bytes32 key;
    }

    struct CollectStateDiffOpts {
        Vm.AccountAccess[] accesses;
        Simulation.Payload simPayload;
    }

    /// @notice Foundry VM instance for state manipulation during simulations
    Vm internal constant VM = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    string internal constant OBJ = "root";

    function collectStateDiff(CollectStateDiffOpts memory opts)
        internal
        returns (MappingParent[] memory, string memory)
    {
        bytes memory encodedStateDiff = abi.encode(opts.accesses);
        string memory json = VM.serializeBytes(OBJ, "stateDiff", encodedStateDiff);
        json = VM.serializeBytes(OBJ, "overrides", abi.encode(opts.simPayload));

        MappingParent[] memory parents = new MappingParent[](1);
        // Account for the msg.sender approval override
        parents[0] = MappingParent({
            slot: keccak256(abi.encode(msg.sender, uint256(8))),
            parent: bytes32(uint256(8)),
            key: bytes32(bytes20(msg.sender))
        });

        for (uint256 i; i < opts.accesses.length; i++) {
            for (uint256 j; j < opts.accesses[i].storageAccesses.length; j++) {
                (bool found, bytes32 key, bytes32 parent) = VM.getMappingKeyAndParentOf(
                    opts.accesses[i].storageAccesses[j].account, opts.accesses[i].storageAccesses[j].slot
                );
                if (found) {
                    parents = _appendToParents(
                        parents,
                        MappingParent({slot: opts.accesses[i].storageAccesses[j].slot, parent: parent, key: key})
                    );
                }
            }
        }

        return (parents, json);
    }

    function recordStateDiff(
        string memory json,
        MappingParent[] memory parents,
        bytes memory txData,
        address targetSafe
    ) internal {
        json = VM.serializeBytes(OBJ, "preimages", abi.encode(parents));
        json = VM.serializeBytes(OBJ, "dataToSign", txData);
        json = VM.serializeAddress(OBJ, "targetSafe", targetSafe);

        bool shouldWrite = VM.envOr("RECORD_STATE_DIFF", false);
        if (shouldWrite) {
            VM.writeJson(json, "stateDiff.json");
        }
    }

    function _appendToParents(MappingParent[] memory parents, MappingParent memory newParent)
        private
        pure
        returns (MappingParent[] memory)
    {
        MappingParent[] memory newArr = new MappingParent[](parents.length + 1);
        for (uint256 i; i < parents.length; i++) {
            newArr[i] = parents[i];
        }
        newArr[parents.length] = newParent;
        return newArr;
    }
}

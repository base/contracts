// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Vm} from "lib/forge-std/src/Vm.sol";

import {IGnosisSafe} from "./IGnosisSafe.sol";

library Simulation {
    address internal constant VM_ADDRESS = address(uint160(uint256(keccak256("hevm cheat code"))));
    Vm internal constant VM = Vm(VM_ADDRESS);

    struct StateOverride {
        address contractAddress;
        StorageOverride[] overrides;
    }

    struct StorageOverride {
        bytes32 key;
        bytes32 value;
    }

    struct Payload {
        address from;
        address to;
        bytes data;
        StateOverride[] stateOverrides;
    }

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
                    target: stateOverride.contractAddress,
                    slot: storageOverride.key,
                    value: storageOverride.value
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

    function overrideSafeThresholdAndNonce(address safe, uint256 nonce) internal view returns (StateOverride memory) {
        StateOverride memory state = StateOverride({contractAddress: safe, overrides: new StorageOverride[](0)});
        state = addThresholdOverride({safe: safe, state: state});
        state = addNonceOverride({safe: safe, state: state, nonce: nonce});
        return state;
    }

    function addApprovalOverride(StateOverride memory state, address owner, bytes32 dataHash)
        internal
        pure
        returns (StateOverride memory)
    {
        return addOverride({
            state: state,
            storageOverride: StorageOverride({
                key: keccak256(abi.encode(dataHash, keccak256(abi.encode(owner, uint256(8))))),
                value: bytes32(uint256(0x1))
            })
        });
    }

    function addThresholdOverride(address safe, StateOverride memory state)
        internal
        view
        returns (StateOverride memory)
    {
        // get the threshold and check if we need to override it
        if (IGnosisSafe(safe).getThreshold() == 1) return state;

        // set the threshold (slot 4) to 1
        return addOverride({
            state: state,
            storageOverride: StorageOverride({key: bytes32(uint256(0x4)), value: bytes32(uint256(0x1))})
        });
    }

    function addOwnerOverride(address safe, StateOverride memory state, address owner)
        internal
        view
        returns (StateOverride memory)
    {
        // get the owners and check if owner is an owner
        address[] memory owners = IGnosisSafe(safe).getOwners();
        for (uint256 i; i < owners.length; i++) {
            if (owners[i] == owner) return state;
        }

        // set the ownerCount (slot 3) to 1
        state = addOverride({
            state: state,
            storageOverride: StorageOverride({key: bytes32(uint256(0x3)), value: bytes32(uint256(0x1))})
        });
        // override the owner mapping (slot 2), which requires two key/value pairs: { 0x1: owner, owner: 0x1 }
        state = addOverride({
            state: state,
            storageOverride: StorageOverride({
                key: bytes32(0xe90b7bceb6e7df5418fb78d8ee546e97c83a08bbccc01a0644d599ccd2a7c2e0), // keccak256(1 || 2)
                value: bytes32(uint256(uint160(owner)))
            })
        });
        return addOverride({
            state: state,
            storageOverride: StorageOverride({key: keccak256(abi.encode(owner, uint256(2))), value: bytes32(uint256(0x1))})
        });
    }

    function addNonceOverride(address safe, StateOverride memory state, uint256 nonce)
        internal
        view
        returns (StateOverride memory)
    {
        // get the nonce and check if we need to override it
        if (IGnosisSafe(safe).nonce() == nonce) return state;

        // set the nonce (slot 5) to the desired value
        return addOverride({
            state: state,
            storageOverride: StorageOverride({key: bytes32(uint256(0x5)), value: bytes32(nonce)})
        });
    }

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

    function logSimulationLink(address to, bytes memory data, address from) internal view {
        logSimulationLink({to: to, data: data, from: from, overrides: new StateOverride[](0)});
    }

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

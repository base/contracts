// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Bytes} from "lib/optimism/packages/contracts-bedrock/src/libraries/Bytes.sol";
import {LibSort} from "@solady/utils/LibSort.sol";

import {IGnosisSafe} from "./IGnosisSafe.sol";

/// @title Signatures - Gnosis Safe Signature Processing Library
library Signatures {
    /// @dev Custom errors for gas efficiency and better debugging
    error InsufficientSignatures(uint256 found, uint256 required);

    function prepareSignatures(address safe, bytes32 hash, bytes memory signatures)
        internal
        view
        returns (bytes memory)
    {
        address[] memory approvers = getApprovers({safeAddr: safe, hash: hash});
        bytes memory prevalidatedSignatures = genPrevalidatedSignatures({addresses: approvers});
        signatures = bytes.concat(prevalidatedSignatures, signatures);

        return sortUniqueSignatures({
            safe: safe,
            signatures: signatures,
            dataHash: hash,
            threshold: IGnosisSafe(safe).getThreshold(),
            dynamicOffset: prevalidatedSignatures.length
        });
    }

    function genPrevalidatedSignatures(address[] memory addresses) internal pure returns (bytes memory) {
        LibSort.sort({a: addresses});
        bytes memory signatures;
        uint256 len = addresses.length;
        for (uint256 i; i < len; ) {
            signatures = bytes.concat(signatures, genPrevalidatedSignature({addr: addresses[i]}));
            unchecked { ++i; }
        }
        return signatures;
    }

    function genPrevalidatedSignature(address addr) internal pure returns (bytes memory) {
        uint8 v = 1;
        bytes32 s = bytes32(0);
        bytes32 r = bytes32(uint256(uint160(addr)));
        return abi.encodePacked(r, s, v);
    }

    function getApprovers(address safeAddr, bytes32 hash) internal view returns (address[] memory) {
        IGnosisSafe safe = IGnosisSafe(safeAddr);
        uint256 threshold = safe.getThreshold();
        address[] memory owners = safe.getOwners();
        address[] memory approvers = new address[](threshold);
        uint256 approverIndex;
        uint256 ownersLen = owners.length;

        for (uint256 i; i < ownersLen; ) {
            address owner = owners[i];
            if (safe.approvedHashes(owner, hash) == 1) {
                approvers[approverIndex] = owner;
                unchecked { approverIndex++; }
                if (approverIndex == threshold) return approvers;
            }
            unchecked { ++i; }
        }

        address[] memory subset = new address[](approverIndex);
        for (uint256 i; i < approverIndex; ) {
            subset[i] = approvers[i];
            unchecked { ++i; }
        }
        return subset;
    }

    function sortUniqueSignatures(
        address safe,
        bytes memory signatures,
        bytes32 dataHash,
        uint256 threshold,
        uint256 dynamicOffset
    ) internal view returns (bytes memory) {
        bytes memory sorted;
        uint256 count = signatures.length / 0x41;
        uint256[] memory addressesAndIndexes = new uint256[](threshold);
        address[] memory uniqueAddresses = new address[](threshold);
        uint256 j;

        for (uint256 i; i < count; ) {
            (address owner, bool isOwner) = extractOwner({safe: safe, signatures: signatures, dataHash: dataHash, i: i});
            
            if (isOwner) {
                uint256 k;
                for (; k < j; ) {
                    if (uniqueAddresses[k] == owner) break;
                    unchecked { ++k; }
                }

                if (k == j) { // Only if not a duplicate
                    uniqueAddresses[j] = owner;
                    addressesAndIndexes[j] = uint256(uint256(uint160(owner)) << 0x60 | i);
                    unchecked { j++; }
                }
            }

            if (j == threshold) break;
            unchecked { ++i; }
        }

        if (j != threshold) revert InsufficientSignatures(j, threshold);

        LibSort.sort({a: addressesAndIndexes});
        for (uint256 i; i < threshold; ) {
            uint256 index = addressesAndIndexes[i] & 0xffffffff;
            (uint8 v, bytes32 r, bytes32 s) = signatureSplit({signatures: signatures, pos: index});
            if (v == 0) {
                s = bytes32(uint256(s) + dynamicOffset);
            }
            sorted = bytes.concat(sorted, abi.encodePacked(r, s, v));
            unchecked { ++i; }
        }

        return appendRemainingBytes({a1: sorted, a2: signatures});
    }

    function extractOwner(address safe, bytes memory signatures, bytes32 dataHash, uint256 i)
        internal
        view
        returns (address, bool)
    {
        (uint8 v, bytes32 r, bytes32 s) = signatureSplit({signatures: signatures, pos: i});
        address owner = extractOwner({dataHash: dataHash, r: r, s: s, v: v});
        bool isOwner = IGnosisSafe(safe).isOwner({owner: owner});
        if (!isOwner) {
            console.log("---\nSkipping signature recovered to non-owner address %s:", owner);
            console.logBytes(abi.encodePacked(r, s, v));
        }
        return (owner, isOwner);
    }

    function extractOwner(bytes32 dataHash, bytes32 r, bytes32 s, uint8 v) internal pure returns (address) {
        if (v <= 1) return address(uint160(uint256(r)));
        if (v > 30) {
            return ecrecover(keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", dataHash)), v - 4, r, s);
        }
        return ecrecover(dataHash, v, r, s);
    }

    function signatureSplit(bytes memory signatures, uint256 pos)
        internal
        pure
        returns (uint8 v, bytes32 r, bytes32 s)
    {
        assembly {
            let signaturePos := mul(0x41, pos)
            r := mload(add(signatures, add(signaturePos, 0x20)))
            s := mload(add(signatures, add(signaturePos, 0x40)))
            v := and(mload(add(signatures, add(signaturePos, 0x41))), 0xff)
        }
    }

    function appendRemainingBytes(bytes memory a1, bytes memory a2) internal pure returns (bytes memory) {
        if (a2.length > a1.length) {
            a1 = bytes.concat(a1, Bytes.slice(a2, a1.length, a2.length - a1.length));
        }
        return a1;
    }
}

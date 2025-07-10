// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// solhint-disable no-console
import {console} from "lib/forge-std/src/console.sol";
import {Bytes} from "@eth-optimism-bedrock/src/libraries/Bytes.sol";
import {LibSort} from "@solady/utils/LibSort.sol";

import {IGnosisSafe} from "./IGnosisSafe.sol";

library Signatures {
    function prepareSignatures(address safe, bytes32 hash, bytes memory signatures)
        internal
        view
        returns (bytes memory)
    {
        // prepend the prevalidated signatures to the signatures
        address[] memory approvers = getApprovers({safeAddr: safe, hash: hash});
        bytes memory prevalidatedSignatures = genPrevalidatedSignatures({addresses: approvers});
        signatures = bytes.concat(prevalidatedSignatures, signatures);

        // safe requires all signatures to be unique, and sorted ascending by public key
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
        for (uint256 i; i < addresses.length; i++) {
            signatures = bytes.concat(signatures, genPrevalidatedSignature({addr: addresses[i]}));
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
        // get a list of owners that have approved this transaction
        IGnosisSafe safe = IGnosisSafe(safeAddr);
        uint256 threshold = safe.getThreshold();
        address[] memory owners = safe.getOwners();
        address[] memory approvers = new address[](threshold);
        uint256 approverIndex;
        for (uint256 i; i < owners.length; i++) {
            address owner = owners[i];
            uint256 approved = safe.approvedHashes(owner, hash);
            if (approved == 1) {
                approvers[approverIndex] = owner;
                approverIndex++;
                if (approverIndex == threshold) {
                    return approvers;
                }
            }
        }
        address[] memory subset = new address[](approverIndex);
        for (uint256 i; i < approverIndex; i++) {
            subset[i] = approvers[i];
        }
        return subset;
    }

    // solhint-disable max-line-length
    /// @notice Sorts the signatures in ascending order of the signer's address, and removes any duplicates.
    ///
    /// @dev see https://github.com/safe-global/safe-smart-account/blob/1ed486bb148fe40c26be58d1b517cec163980027/contracts/Safe.sol#L265-L334
    /// @dev `signatures` can be packed ECDSA signature ({bytes32 r}{bytes32 s}{uint8 v}), contract signature (EIP-1271) or approved hash.
    /// @dev `signatures` can be suffixed with EIP-1271 signatures after threshold*65 bytes.
    ///
    /// @param safe          Address of the Safe that should verify the signatures.
    /// @param signatures    Signature data that should be verified.
    /// @param dataHash      Hash that is signed.
    /// @param threshold     Number of signatures required to approve the transaction.
    /// @param dynamicOffset Offset to add to the `s` value of any EIP-1271 signature.
    ///                      Can be used to accommodate any additional signatures prepended to the array.
    ///                      If prevalidated signatures were prepended, this should be the length of those signatures.
    function sortUniqueSignatures(
        address safe,
        bytes memory signatures,
        bytes32 dataHash,
        uint256 threshold,
        uint256 dynamicOffset
    ) internal view returns (bytes memory) {
        bytes memory sorted;
        uint256 count = uint256(signatures.length / 0x41);
        uint256[] memory addressesAndIndexes = new uint256[](threshold);
        address[] memory uniqueAddresses = new address[](threshold);
        uint256 j;
        for (uint256 i; i < count; i++) {
            (address owner, bool isOwner) = extractOwner({safe: safe, signatures: signatures, dataHash: dataHash, i: i});
            if (!isOwner) {
                continue;
            }

            // skip duplicate owners
            uint256 k;
            for (; k < j; k++) {
                if (uniqueAddresses[k] == owner) break;
            }
            if (k < j) continue;

            uniqueAddresses[j] = owner;
            addressesAndIndexes[j] = uint256(uint256(uint160(owner)) << 0x60 | i); // address in first 160 bits, index in second 96 bits
            j++;

            // we have enough signatures to reach the threshold
            if (j == threshold) break;
        }
        require(j == threshold, "not enough signatures");

        LibSort.sort({a: addressesAndIndexes});
        for (uint256 i; i < count; i++) {
            uint256 index = addressesAndIndexes[i] & 0xffffffff;
            (uint8 v, bytes32 r, bytes32 s) = signatureSplit({signatures: signatures, pos: index});
            if (v == 0) {
                // The `s` value is used by safe as a lookup into the signature bytes.
                // Increment by the offset so that the lookup location remains correct.
                s = bytes32(uint256(s) + dynamicOffset);
            }
            sorted = bytes.concat(sorted, abi.encodePacked(r, s, v));
        }

        // append the non-static part of the signatures (can contain EIP-1271 signatures if contracts are signers)
        // if there were any duplicates detected above, they will be safely ignored by Safe's checkNSignatures method
        sorted = appendRemainingBytes({a1: sorted, a2: signatures});

        return sorted;
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
            console.log("---\nSkipping the following signature, which was recovered to a non-owner address %s:", owner);
            console.logBytes(abi.encodePacked(r, s, v));
        }
        return (owner, isOwner);
    }

    function extractOwner(bytes32 dataHash, bytes32 r, bytes32 s, uint8 v) internal pure returns (address) {
        if (v <= 1) {
            return address(uint160(uint256(r)));
        }
        if (v > 30) {
            return ecrecover(keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", dataHash)), v - 4, r, s);
        }
        return ecrecover(dataHash, v, r, s);
    }

    // see https://github.com/safe-global/safe-contracts/blob/1ed486bb148fe40c26be58d1b517cec163980027/contracts/common/SignatureDecoder.sol
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

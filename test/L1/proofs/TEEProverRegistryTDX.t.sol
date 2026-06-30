// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

contract TEEProverRegistryTDXTest is Test {
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");

    function testRegisterTDXSignerStoresImageHash() public {
        address signer = makeAddr("signer");
        address tdxVerifier = makeAddr("tdx-verifier");

        vm.etch(tdxVerifier, hex"00");
        vm.mockCall(
            tdxVerifier, abi.encodeCall(ITDXVerifier.verify, (bytes(""), bytes(""))), abi.encode(signer, IMAGE_HASH)
        );

        TEEProverRegistry registry = new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(tdxVerifier), IDisputeGameFactory(makeAddr("factory"))
        );

        vm.prank(address(0xdEaD));
        registry.registerTDXSigner("", "");

        assertTrue(registry.isRegisteredSigner(signer));
        assertEq(registry.signerImageHash(signer), IMAGE_HASH);
        assertEq(uint8(registry.signerTEEType(signer)), uint8(TEEProverRegistry.TEEType.TDX));
    }

    function testConstructorRevertsIfTDXVerifierNotSet() public {
        vm.expectRevert(TEEProverRegistry.TDXVerifierNotSet.selector);
        new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(0)), IDisputeGameFactory(makeAddr("factory"))
        );
    }
}

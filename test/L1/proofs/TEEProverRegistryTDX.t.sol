// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

contract TEEProverRegistryTDXTest is Test {
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    GameType internal constant TEST_GAME_TYPE = GameType.wrap(0);

    function testRegisterTDXSignerStoresImageHash() public {
        address signer = makeAddr("signer");
        address tdxVerifier = makeAddr("tdx-verifier");
        address aggregateVerifier = makeAddr("aggregate-verifier");
        address factory = makeAddr("factory");

        vm.etch(tdxVerifier, hex"00");
        vm.mockCall(
            tdxVerifier, abi.encodeCall(ITDXVerifier.verify, (bytes(""), bytes(""))), abi.encode(signer, IMAGE_HASH)
        );

        vm.etch(aggregateVerifier, hex"00");
        vm.mockCall(aggregateVerifier, abi.encodeWithSignature("TEE_NITRO_IMAGE_HASH()"), abi.encode(IMAGE_HASH));
        vm.mockCall(aggregateVerifier, abi.encodeWithSignature("TEE_TDX_IMAGE_HASH()"), abi.encode(IMAGE_HASH));

        vm.etch(factory, hex"00");
        vm.mockCall(
            factory, abi.encodeCall(IDisputeGameFactory.gameImpls, (TEST_GAME_TYPE)), abi.encode(aggregateVerifier)
        );

        TEEProverRegistry registry = new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(tdxVerifier), IDisputeGameFactory(factory)
        );

        vm.prank(address(0xdEaD));
        registry.registerTDXSigner("", "");

        assertTrue(registry.isRegisteredSigner(signer));
        assertEq(registry.signerImageHash(signer), IMAGE_HASH);
        assertEq(uint8(registry.signerTEEType(signer)), uint8(TEEProverRegistry.TEEType.TDX));
        assertTrue(registry.isValidSigner(signer));
    }

    function testConstructorRevertsIfTDXVerifierNotSet() public {
        vm.expectRevert(TEEProverRegistry.TDXVerifierNotSet.selector);
        new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(0)), IDisputeGameFactory(makeAddr("factory"))
        );
    }
}

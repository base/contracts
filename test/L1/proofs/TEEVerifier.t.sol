// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import {
    TransparentUpgradeableProxy
} from "lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { MockAnchorStateRegistry } from "scripts/multiproof/mocks/MockAnchorStateRegistry.sol";
import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

contract MockAggregateVerifierForVerifier {
    bytes32 public immutable TEE_IMAGE_HASH;

    constructor(bytes32 imageHash) {
        TEE_IMAGE_HASH = imageHash;
    }
}

contract MockDisputeGameFactoryForVerifier {
    IDisputeGame internal immutable _impl;

    constructor(address impl) {
        _impl = IDisputeGame(impl);
    }

    function gameImpls(GameType) external view returns (IDisputeGame) {
        return _impl;
    }
}

contract TEEVerifierTest is Test {
    TEEVerifier public verifier;
    DevTEEProverRegistry public teeProverRegistry;

    uint256 internal constant SIGNER_PRIVATE_KEY = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    bytes32 internal constant IMAGE_ID = keccak256("test-image-id");
    bytes32 internal constant TEST_JOURNAL = keccak256("test-journal");
    GameType internal constant TEST_GAME_TYPE = GameType.wrap(621);
    address internal immutable PROPOSER = makeAddr("proposer");

    function setUp() public {
        address signerAddress = vm.addr(SIGNER_PRIVATE_KEY);
        MockAggregateVerifierForVerifier mockVerifier = new MockAggregateVerifierForVerifier(IMAGE_ID);
        MockDisputeGameFactoryForVerifier mockFactory = new MockDisputeGameFactoryForVerifier(address(mockVerifier));

        // DevTEEProverRegistry keeps these tests focused on verifier behavior without Nitro attestation setup.
        DevTEEProverRegistry impl =
            new DevTEEProverRegistry(INitroEnclaveVerifier(address(0)), IDisputeGameFactory(address(mockFactory)));

        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl),
            makeAddr("proxy-admin"),
            abi.encodeCall(
                TEEProverRegistry.initialize,
                (address(this), address(this), new address[](0), TEST_GAME_TYPE)
            )
        );

        teeProverRegistry = DevTEEProverRegistry(address(proxy));
        teeProverRegistry.addDevSigner(signerAddress, IMAGE_ID);
        teeProverRegistry.setProposer(PROPOSER, true);

        MockAnchorStateRegistry anchorStateRegistry = new MockAnchorStateRegistry();
        verifier = new TEEVerifier(
            TEEProverRegistry(address(teeProverRegistry)), IAnchorStateRegistry(address(anchorStateRegistry))
        );
    }

    function testVerifyValidSignature() public view {
        assertTrue(verifier.verify(_proofFor(PROPOSER, SIGNER_PRIVATE_KEY, TEST_JOURNAL), IMAGE_ID, TEST_JOURNAL));
    }

    function testVerifyFailsWithInvalidSignature() public {
        bytes memory proofBytes = abi.encodePacked(PROPOSER, bytes32(0), bytes32(0), uint8(27));

        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(proofBytes, IMAGE_ID, TEST_JOURNAL);
    }

    function testVerifyFailsWithInvalidProposer() public {
        bytes memory proofBytes = _proofFor(address(0), SIGNER_PRIVATE_KEY, TEST_JOURNAL);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(proofBytes, IMAGE_ID, TEST_JOURNAL);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        uint256 unregisteredKey = 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef;
        address unregisteredSigner = vm.addr(unregisteredKey);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(_proofFor(PROPOSER, unregisteredKey, TEST_JOURNAL), IMAGE_ID, TEST_JOURNAL);
    }

    function testVerifyFailsWithImageIdMismatch() public {
        bytes32 wrongImageId = keccak256("different-image");
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.ImageIdMismatch.selector, IMAGE_ID, wrongImageId));
        verifier.verify(_proofFor(PROPOSER, SIGNER_PRIVATE_KEY, TEST_JOURNAL), wrongImageId, TEST_JOURNAL);
    }

    function testVerifyFailsWithInvalidProofFormat() public {
        bytes memory shortProof = new bytes(50);

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(shortProof, IMAGE_ID, TEST_JOURNAL);
    }

    function testConstants() public view {
        assertEq(address(verifier.TEE_PROVER_REGISTRY()), address(teeProverRegistry));
    }

    function _proofFor(address proposer, uint256 privateKey, bytes32 journal) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, journal);
        return abi.encodePacked(proposer, r, s, v);
    }
}

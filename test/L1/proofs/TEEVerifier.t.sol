// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "src/universal/ProxyAdmin.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { MockAnchorStateRegistry } from "scripts/multiproof/mocks/MockAnchorStateRegistry.sol";
import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";
import { TEEVerifier } from "src/L1/proofs/tee/TEEVerifier.sol";

/// @notice Mock AggregateVerifier that returns a configurable TEE_IMAGE_HASH.
contract MockAggregateVerifierForVerifier {
    bytes32 public TEE_IMAGE_HASH;

    constructor(bytes32 imageHash) {
        TEE_IMAGE_HASH = imageHash;
    }
}

/// @notice Mock DisputeGameFactory that returns a fixed game implementation.
contract MockDisputeGameFactoryForVerifier {
    mapping(uint32 => address) internal _impls;

    function setImpl(uint32 gameType_, address impl) external {
        _impls[gameType_] = impl;
    }

    function gameImpls(GameType gameType_) external view returns (IDisputeGame) {
        return IDisputeGame(_impls[GameType.unwrap(gameType_)]);
    }
}

contract TEEVerifierTest is Test {
    TEEVerifier public verifier;
    DevTEEProverRegistry public teeProverRegistry;
    ProxyAdmin public proxyAdmin;
    MockAnchorStateRegistry public anchorStateRegistry;

    // Test signer - we'll derive address from private key
    uint256 internal constant SIGNER_PRIVATE_KEY = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    address internal signerAddress;

    bytes32 internal constant IMAGE_ID = keccak256("test-image-id");
    uint32 internal constant TEST_GAME_TYPE = 621;
    address internal immutable PROPOSER = makeAddr("proposer");

    address internal owner;

    function setUp() public {
        owner = address(this);

        // Derive signer address from private key
        signerAddress = vm.addr(SIGNER_PRIVATE_KEY);

        // Deploy mock factory and verifier
        MockAggregateVerifierForVerifier mockVerifier = new MockAggregateVerifierForVerifier(IMAGE_ID);
        MockDisputeGameFactoryForVerifier mockFactory = new MockDisputeGameFactoryForVerifier();
        mockFactory.setImpl(TEST_GAME_TYPE, address(mockVerifier));

        // Deploy implementation (NitroEnclaveVerifier not needed for dev signer tests)
        DevTEEProverRegistry impl =
            new DevTEEProverRegistry(INitroEnclaveVerifier(address(0)), IDisputeGameFactory(address(mockFactory)));

        // Deploy proxy admin
        proxyAdmin = new ProxyAdmin(address(this));

        // Deploy proxy
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(impl),
            address(proxyAdmin),
            abi.encodeCall(
                TEEProverRegistry.initialize, (owner, owner, new address[](0), GameType.wrap(TEST_GAME_TYPE))
            )
        );

        teeProverRegistry = DevTEEProverRegistry(address(proxy));

        // Register the signer with the correct image hash
        teeProverRegistry.addDevSigner(signerAddress, IMAGE_ID);

        // Set the proposer as valid
        teeProverRegistry.setProposer(PROPOSER, true);

        // Deploy TEEVerifier
        anchorStateRegistry = new MockAnchorStateRegistry();
        verifier = new TEEVerifier(
            TEEProverRegistry(address(teeProverRegistry)), IAnchorStateRegistry(address(anchorStateRegistry))
        );
    }

    function testVerifyValidSignature() public view {
        // Create a journal hash
        bytes32 journal = keccak256("test-journal");

        // Sign the journal with the signer's private key
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Construct proof: proposer(20) + signature(65) = 85 bytes
        bytes memory proofBytes = abi.encodePacked(PROPOSER, signature);

        // Verify should return true regardless of imageId (enforced via journal hash, not registry)
        bool result = verifier.verify(proofBytes, IMAGE_ID, journal);
        assertTrue(result);
    }

    function testVerifyFailsWithInvalidSignature() public {
        bytes32 journal = keccak256("test-journal");

        // Create an invalid signature (all zeros except v)
        bytes memory invalidSignature = new bytes(65);
        invalidSignature[64] = bytes1(uint8(27)); // Set v to 27

        bytes memory proofBytes = abi.encodePacked(PROPOSER, invalidSignature);

        vm.expectRevert(TEEVerifier.InvalidSignature.selector);
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithInvalidProposer() public {
        // Create a journal hash
        bytes32 journal = keccak256("test-journal");

        // Sign the journal with the signer's private key
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        // Construct proof: proposer(20) + signature(65) = 85 bytes
        bytes memory proofBytes = abi.encodePacked(address(0), signature);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidProposer.selector, address(0)));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithUnregisteredSigner() public {
        // Use a different private key that's not registered
        uint256 unregisteredKey = 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef;
        address unregisteredSigner = vm.addr(unregisteredKey);

        bytes32 journal = keccak256("test-journal");

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(unregisteredKey, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        bytes memory proofBytes = abi.encodePacked(PROPOSER, signature);

        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.InvalidSigner.selector, unregisteredSigner));
        verifier.verify(proofBytes, IMAGE_ID, journal);
    }

    function testVerifyFailsWithImageIdMismatch() public {
        bytes32 journal = keccak256("test-journal");

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(SIGNER_PRIVATE_KEY, journal);
        bytes memory signature = abi.encodePacked(r, s, v);

        bytes memory proofBytes = abi.encodePacked(PROPOSER, signature);

        // Different imageId should fail — signer was registered with IMAGE_ID
        bytes32 wrongImageId = keccak256("different-image");
        vm.expectRevert(abi.encodeWithSelector(TEEVerifier.ImageIdMismatch.selector, IMAGE_ID, wrongImageId));
        verifier.verify(proofBytes, wrongImageId, journal);
    }

    function testVerifyFailsWithInvalidProofFormat() public {
        bytes32 journal = keccak256("test-journal");

        // Proof too short (less than 85 bytes)
        bytes memory shortProof = new bytes(50);

        vm.expectRevert(TEEVerifier.InvalidProofFormat.selector);
        verifier.verify(shortProof, IMAGE_ID, journal);
    }

    function testConstants() public view {
        assertEq(address(verifier.TEE_PROVER_REGISTRY()), address(teeProverRegistry));
    }
}

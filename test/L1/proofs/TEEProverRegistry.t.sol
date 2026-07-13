// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Proxy } from "src/universal/Proxy.sol";

import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { DevTEEProverRegistry } from "test/mocks/MockDevTEEProverRegistry.sol";
import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

bytes32 constant TEST_IMAGE_HASH = keccak256("test-image-hash");

/// @dev Uses DevTEEProverRegistry because production signer registration requires a Nitro attestation proof.
contract TEEProverRegistryTest is Test {
    DevTEEProverRegistry internal teeProverRegistry;

    address internal immutable owner = makeAddr("owner");
    address internal immutable manager = makeAddr("manager");

    GameType internal constant TEST_GAME_TYPE = GameType.wrap(621);

    function setUp() public {
        teeProverRegistry = _deployRegistry(new address[](0));
    }

    function _deployRegistry(address[] memory proposers) internal returns (DevTEEProverRegistry) {
        address verifier = makeAddr("verifier");
        vm.mockCall(verifier, abi.encodeWithSignature("TEE_NITRO_IMAGE_HASH()"), abi.encode(TEST_IMAGE_HASH));

        address factory = makeAddr("factory");
        vm.mockCall(factory, abi.encodeCall(IDisputeGameFactory.gameImpls, (TEST_GAME_TYPE)), abi.encode(verifier));

        DevTEEProverRegistry impl = new DevTEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(1)), IDisputeGameFactory(factory)
        );

        address proxyAdmin = makeAddr("proxy-admin");
        Proxy proxy = new Proxy(proxyAdmin);
        vm.prank(proxyAdmin);
        proxy.upgradeToAndCall(
            address(impl), abi.encodeCall(TEEProverRegistry.initialize, (owner, manager, proposers, TEST_GAME_TYPE))
        );

        return DevTEEProverRegistry(address(proxy));
    }

    function _addDevSigner(address signer) internal {
        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH, TEEProverRegistry.TEEType.NITRO);
    }

    function testInitialization() public view {
        assertEq(teeProverRegistry.owner(), owner);
        assertEq(teeProverRegistry.manager(), manager);
        assertEq(teeProverRegistry.version(), "0.7.0");
    }

    function testInitializationWithProposers() public {
        address[] memory proposers = new address[](1);
        proposers[0] = makeAddr("proposer");

        DevTEEProverRegistry registry2 = _deployRegistry(proposers);
        assertTrue(registry2.isValidProposer(proposers[0]));
    }

    function testDeregisterSignerAsOwnerOrManager() public {
        address ownerSigner = makeAddr("owner-signer");
        address managerSigner = makeAddr("manager-signer");

        _addDevSigner(ownerSigner);
        _addDevSigner(managerSigner);

        vm.prank(owner);
        teeProverRegistry.deregisterSigner(ownerSigner);
        vm.prank(manager);
        teeProverRegistry.deregisterSigner(managerSigner);

        assertFalse(teeProverRegistry.isValidSigner(ownerSigner));
        assertFalse(teeProverRegistry.isValidSigner(managerSigner));
    }

    function testDeregisterSignerFailsIfUnauthorized() public {
        vm.prank(makeAddr("unauthorized"));
        vm.expectRevert(bytes("OwnableManaged: caller is not the owner or the manager"));
        teeProverRegistry.deregisterSigner(makeAddr("signer"));
    }

    function testSetProposer() public {
        address newProposer = makeAddr("proposer");

        vm.prank(owner);
        teeProverRegistry.setProposer(newProposer, true);

        assertTrue(teeProverRegistry.isValidProposer(newProposer));
    }

    function testSetProposerFailsIfNotOwner() public {
        vm.prank(manager);
        vm.expectRevert(bytes("OwnableManaged: caller is not the owner"));
        teeProverRegistry.setProposer(makeAddr("proposer"), true);
    }

    function testRegisteringSameSignerWithDifferentTEETypeOverwritesTEEType() public {
        address signer = makeAddr("dev-signer");

        _addDevSigner(signer);

        vm.prank(owner);
        teeProverRegistry.addDevSigner(signer, TEST_IMAGE_HASH, TEEProverRegistry.TEEType.TDX);

        assertTrue(teeProverRegistry.signerTEEType(signer) == TEEProverRegistry.TEEType.TDX);
    }

    function testRegisterTDXSignerStoresImageHash() public {
        address signer = makeAddr("tdx-signer");
        address tdxVerifier = makeAddr("tdx-verifier");

        vm.mockCall(
            tdxVerifier,
            abi.encodeCall(ITDXVerifier.verify, (bytes(""), bytes(""))),
            abi.encode(signer, TEST_IMAGE_HASH)
        );

        TEEProverRegistry registry = new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(tdxVerifier), IDisputeGameFactory(makeAddr("factory"))
        );

        vm.prank(address(0xdEaD));
        registry.registerTDXSigner("", "");

        assertTrue(registry.isRegisteredSigner(signer));
        assertEq(registry.signerImageHash(signer), TEST_IMAGE_HASH);
        assertTrue(registry.signerTEEType(signer) == TEEProverRegistry.TEEType.TDX);
    }

    function testConstructorRevertsIfTDXVerifierNotSet() public {
        vm.expectRevert(TEEProverRegistry.TDXVerifierNotSet.selector);
        new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(0)), IDisputeGameFactory(makeAddr("factory"))
        );
    }

    function testAddDevSigner() public {
        address signer = makeAddr("dev-signer");

        _addDevSigner(signer);

        assertTrue(teeProverRegistry.isValidSigner(signer));
        assertTrue(teeProverRegistry.signerTEEType(signer) == TEEProverRegistry.TEEType.NITRO);
    }

    function testAddDevSignerFailsIfNotOwner() public {
        vm.prank(manager);
        vm.expectRevert(bytes("OwnableManaged: caller is not the owner"));
        teeProverRegistry.addDevSigner(makeAddr("dev-signer"), TEST_IMAGE_HASH, TEEProverRegistry.TEEType.NITRO);
    }

    function testGetRegisteredSignersConsistencyAfterMixedOperations() public {
        address signer1 = makeAddr("signer-1");
        address signer2 = makeAddr("signer-2");

        _addDevSigner(signer1);
        _addDevSigner(signer2);

        vm.prank(manager);
        teeProverRegistry.deregisterSigner(signer2);

        address[] memory signers = teeProverRegistry.getRegisteredSigners();
        assertEq(signers.length, 1);
        assertEq(signers[0], signer1);
        assertFalse(teeProverRegistry.isValidSigner(signer2));
    }
}

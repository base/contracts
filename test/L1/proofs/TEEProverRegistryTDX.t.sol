// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { IDisputeGame } from "interfaces/L1/proofs/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITDXVerifier } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";
import { GameType } from "src/libraries/bridge/Types.sol";

import { TEEProverRegistry } from "src/L1/proofs/tee/TEEProverRegistry.sol";

/// @notice Mock AggregateVerifier that returns configurable TEE image hashes.
contract MockAggregateVerifierForTDXRegistry {
    bytes32 public TEE_NITRO_IMAGE_HASH;
    bytes32 public TEE_TDX_IMAGE_HASH;

    constructor(bytes32 imageHash) {
        TEE_NITRO_IMAGE_HASH = imageHash;
        TEE_TDX_IMAGE_HASH = imageHash;
    }
}

/// @notice Mock DisputeGameFactory that returns a fixed game implementation.
contract MockDisputeGameFactoryForTDXRegistry {
    mapping(uint32 => address) internal _impls;

    function setImpl(uint32 gameType, address impl) external {
        _impls[gameType] = impl;
    }

    function gameImpls(GameType gameType) external view returns (IDisputeGame) {
        return IDisputeGame(_impls[GameType.unwrap(gameType)]);
    }
}

contract MockTDXVerifierForRegistry is ITDXVerifier {
    address internal _signer;
    bytes32 internal _imageHash;
    bytes32 internal _reportDataSuffix;

    function setResult(address signer, bytes32 imageHash, bytes32 reportDataSuffix) external {
        _signer = signer;
        _imageHash = imageHash;
        _reportDataSuffix = reportDataSuffix;
    }

    function verify(bytes calldata, bytes calldata) external view returns (address, bytes32, bytes32) {
        return (_signer, _imageHash, _reportDataSuffix);
    }
}

contract TEEProverRegistryTDXTest is Test {
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    bytes32 internal constant REPORT_DATA_SUFFIX = keccak256("multiproof-tdx-poc");

    function testRegisterTDXSignerStoresImageHash() public {
        address signer = address(0x1234);
        MockTDXVerifierForRegistry verifier = new MockTDXVerifierForRegistry();
        verifier.setResult(signer, IMAGE_HASH, REPORT_DATA_SUFFIX);

        MockDisputeGameFactoryForTDXRegistry factory = new MockDisputeGameFactoryForTDXRegistry();
        factory.setImpl(0, address(new MockAggregateVerifierForTDXRegistry(IMAGE_HASH)));

        TEEProverRegistry registry = new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(verifier)), IDisputeGameFactory(address(factory))
        );

        vm.prank(address(0xdEaD));
        registry.registerTDXSigner("", "");

        assertTrue(registry.isRegisteredSigner(signer));
        assertEq(registry.signerImageHash(signer), IMAGE_HASH);
        assertEq(uint8(registry.signerTEEType(signer)), uint8(TEEProverRegistry.TEEType.TDX));
        assertTrue(registry.isValidSigner(signer));
    }

    function testConstructorRevertsIfTDXVerifierNotSet() public {
        MockDisputeGameFactoryForTDXRegistry factory = new MockDisputeGameFactoryForTDXRegistry();
        factory.setImpl(0, address(new MockAggregateVerifierForTDXRegistry(IMAGE_HASH)));

        vm.expectRevert(TEEProverRegistry.TDXVerifierNotSet.selector);
        new TEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(0)), IDisputeGameFactory(address(factory))
        );
    }
}

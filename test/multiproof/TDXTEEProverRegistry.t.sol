// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "forge-std/Test.sol";

import { IDisputeGame } from "interfaces/dispute/IDisputeGame.sol";
import { IDisputeGameFactory } from "interfaces/dispute/IDisputeGameFactory.sol";
import { INitroEnclaveVerifier } from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import {
    ITDXVerifier,
    TDXTcbStatus,
    TDXVerificationResult,
    TDXVerifierJournal
} from "interfaces/multiproof/tee/ITDXVerifier.sol";
import { ZkCoProcessorConfig, ZkCoProcessorType } from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";
import { GameType } from "src/dispute/lib/Types.sol";

import { TDXTEEProverRegistry } from "src/multiproof/tee/TDXTEEProverRegistry.sol";

/// @notice Mock AggregateVerifier that returns a configurable TEE_IMAGE_HASH.
contract MockAggregateVerifierForTDXRegistry {
    bytes32 public TEE_IMAGE_HASH;

    constructor(bytes32 imageHash) {
        TEE_IMAGE_HASH = imageHash;
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
    TDXVerifierJournal internal _journal;

    function setJournal(TDXVerifierJournal memory journal) external {
        _journal = journal;
    }

    function verify(
        bytes calldata,
        ZkCoProcessorType,
        bytes calldata
    )
        external
        view
        returns (TDXVerifierJournal memory)
    {
        return _journal;
    }

    function getZkConfig(ZkCoProcessorType) external pure returns (ZkCoProcessorConfig memory) {
        return ZkCoProcessorConfig({ verifierId: bytes32(0), aggregatorId: bytes32(0), zkVerifier: address(0) });
    }

    function allowedTcbStatuses(TDXTcbStatus) external pure returns (bool) {
        return true;
    }
}

contract TDXTEEProverRegistryTest is Test {
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    bytes32 internal constant REPORT_DATA_SUFFIX = keccak256("multiproof-tdx-poc");

    function testRegisterTDXSignerStoresImageHash() public {
        TDXVerifierJournal memory journal = _successJournal();
        MockTDXVerifierForRegistry verifier = new MockTDXVerifierForRegistry();
        verifier.setJournal(journal);

        MockDisputeGameFactoryForTDXRegistry factory = new MockDisputeGameFactoryForTDXRegistry();
        factory.setImpl(0, address(new MockAggregateVerifierForTDXRegistry(IMAGE_HASH)));

        TDXTEEProverRegistry registry = new TDXTEEProverRegistry(
            INitroEnclaveVerifier(address(0)), ITDXVerifier(address(verifier)), IDisputeGameFactory(address(factory))
        );

        vm.prank(address(0xdEaD));
        registry.registerTDXSigner("", ZkCoProcessorType.Succinct, "");

        assertTrue(registry.isRegisteredSigner(journal.signer));
        assertEq(registry.signerImageHash(journal.signer), IMAGE_HASH);
        assertTrue(registry.isValidSigner(journal.signer));
    }

    function _successJournal() internal pure returns (TDXVerifierJournal memory journal) {
        address signer = address(0x1234);

        journal = TDXVerifierJournal({
            result: TDXVerificationResult.Success,
            tcbStatus: TDXTcbStatus.UpToDate,
            timestamp: 0,
            collateralExpiration: 0,
            rootCaHash: bytes32(0),
            pckCertHash: bytes32(0),
            tcbInfoHash: bytes32(0),
            qeIdentityHash: bytes32(0),
            publicKey: "",
            signer: signer,
            imageHash: IMAGE_HASH,
            mrTdHash: bytes32(0),
            reportDataPrefix: bytes32(0),
            reportDataSuffix: REPORT_DATA_SUFFIX
        });
    }
}

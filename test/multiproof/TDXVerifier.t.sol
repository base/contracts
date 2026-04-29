// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "forge-std/Test.sol";

import { TDXTcbStatus, TDXVerificationResult, TDXVerifierJournal } from "interfaces/multiproof/tee/ITDXVerifier.sol";
import { ZkCoProcessorConfig, ZkCoProcessorType } from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";

import { TDXVerifier } from "src/multiproof/tee/TDXVerifier.sol";

contract TDXVerifierTest is Test {
    TDXVerifier internal verifier;

    address internal owner;
    address internal proofSubmitter;
    address internal mockRiscZeroVerifier;
    address internal mockSP1Verifier;

    bytes32 internal constant ROOT_CA_HASH = keccak256("intel-root-ca");
    bytes32 internal constant WRONG_ROOT_CA_HASH = keccak256("wrong-root-ca");
    bytes32 internal constant VERIFIER_ID = keccak256("tdx-verifier-id");
    bytes32 internal constant AGGREGATOR_ID = keccak256("tdx-aggregator-id");
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    bytes32 internal constant MRTD_HASH = keccak256("mrtd");
    bytes32 internal constant REPORT_DATA_SUFFIX = keccak256("multiproof-tdx-poc");

    uint64 internal constant MAX_TIME_DIFF = 3600;
    uint256 internal constant NOW = 1_700_000_000;

    function setUp() public {
        vm.warp(NOW);

        owner = address(this);
        proofSubmitter = address(this);
        mockRiscZeroVerifier = makeAddr("mock-risc-zero");
        mockSP1Verifier = makeAddr("mock-sp1");

        TDXTcbStatus[] memory allowedStatuses = new TDXTcbStatus[](2);
        allowedStatuses[0] = TDXTcbStatus.UpToDate;
        allowedStatuses[1] = TDXTcbStatus.SwHardeningNeeded;

        verifier = new TDXVerifier(
            owner,
            MAX_TIME_DIFF,
            ROOT_CA_HASH,
            proofSubmitter,
            ZkCoProcessorType.Succinct,
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockSP1Verifier }),
            allowedStatuses
        );
    }

    function testVerifySucceedsWithSP1ProofAndAllowedJournal() public {
        TDXVerifierJournal memory journal = _successJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        TDXVerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);

        assertEq(result.signer, journal.signer);
        assertEq(result.imageHash, IMAGE_HASH);
        assertEq(uint256(result.tcbStatus), uint256(TDXTcbStatus.UpToDate));
    }

    function testVerifySucceedsWithRiscZeroProof() public {
        ZkCoProcessorConfig memory config = ZkCoProcessorConfig({
            verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockRiscZeroVerifier
        });
        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config);

        TDXVerifierJournal memory journal = _successJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"5678";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        TDXVerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(result.signer, journal.signer);
        assertEq(result.imageHash, IMAGE_HASH);
    }

    function testVerifyRevertsIfNotProofSubmitter() public {
        bytes memory output = abi.encode(_successJournal());

        vm.prank(makeAddr("not-submitter"));
        vm.expectRevert(TDXVerifier.CallerNotProofSubmitter.selector);
        verifier.verify(output, ZkCoProcessorType.Succinct, "");
    }

    function testVerifyRevertsIfZkVerifierNotConfigured() public {
        bytes memory output = abi.encode(_successJournal());

        vm.expectRevert(
            abi.encodeWithSelector(TDXVerifier.ZkVerifierNotConfigured.selector, ZkCoProcessorType.RiscZero)
        );
        verifier.verify(output, ZkCoProcessorType.RiscZero, "");
    }

    function testVerifyRevertsForUnknownCoprocessor() public {
        ZkCoProcessorConfig memory config =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockSP1Verifier });
        verifier.setZkConfiguration(ZkCoProcessorType.Unknown, config);

        bytes memory output = abi.encode(_successJournal());

        vm.expectRevert(TDXVerifier.UnknownZkCoprocessor.selector);
        verifier.verify(output, ZkCoProcessorType.Unknown, "");
    }

    function testVerifyRevertsWhenGuestReportsFailure() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.result = TDXVerificationResult.PckCertChainInvalid;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.TDXVerificationFailed.selector, journal.result));
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function testVerifyRevertsWhenRootCaHashMismatches() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.rootCaHash = WRONG_ROOT_CA_HASH;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(TDXVerifier.RootCaHashMismatch.selector, ROOT_CA_HASH, WRONG_ROOT_CA_HASH)
        );
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function testVerifyRevertsWhenTcbStatusIsNotAllowed() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.tcbStatus = TDXTcbStatus.ConfigurationNeeded;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.TcbStatusNotAllowed.selector, journal.tcbStatus));
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function testOwnerCanAllowAdditionalTcbStatus() public {
        verifier.setTcbStatusAllowed(TDXTcbStatus.ConfigurationNeeded, true);

        TDXVerifierJournal memory journal = _successJournal();
        journal.tcbStatus = TDXTcbStatus.ConfigurationNeeded;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        TDXVerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);

        assertEq(uint256(result.tcbStatus), uint256(TDXTcbStatus.ConfigurationNeeded));
    }

    function testVerifyRevertsWhenCollateralExpired() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.collateralExpiration = uint64(block.timestamp);
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.CollateralExpired.selector, journal.collateralExpiration));
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function testVerifyRevertsWhenTimestampTooOld() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp - MAX_TIME_DIFF) * 1000;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(
                TDXVerifier.InvalidTimestamp.selector, uint64(block.timestamp - MAX_TIME_DIFF), block.timestamp
            )
        );
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function testVerifyRevertsWhenTimestampIsFromFuture() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp) * 1000;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(TDXVerifier.InvalidTimestamp.selector, uint64(block.timestamp), block.timestamp)
        );
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function testVerifyRevertsWhenReportDataDoesNotBindPublicKey() public {
        TDXVerifierJournal memory journal = _successJournal();
        bytes32 expected = journal.reportDataPrefix;
        journal.reportDataPrefix = keccak256("wrong-report-data");
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(TDXVerifier.ReportDataMismatch.selector, expected, journal.reportDataPrefix)
        );
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function testVerifyRevertsWhenSignerDoesNotMatchPublicKey() public {
        TDXVerifierJournal memory journal = _successJournal();
        address expected = journal.signer;
        journal.signer = makeAddr("wrong-signer");
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.SignerMismatch.selector, expected, journal.signer));
        verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function _successJournal() internal view returns (TDXVerifierJournal memory journal) {
        bytes memory publicKey = _publicKey();
        bytes32 publicKeyHash;
        assembly {
            publicKeyHash := keccak256(add(publicKey, 0x21), 64)
        }

        journal = TDXVerifierJournal({
            result: TDXVerificationResult.Success,
            tcbStatus: TDXTcbStatus.UpToDate,
            timestamp: uint64(block.timestamp - 1) * 1000,
            collateralExpiration: uint64(block.timestamp + 1 days),
            rootCaHash: ROOT_CA_HASH,
            pckCertHash: keccak256("pck-cert"),
            tcbInfoHash: keccak256("tcb-info"),
            qeIdentityHash: keccak256("qe-identity"),
            publicKey: publicKey,
            signer: address(uint160(uint256(publicKeyHash))),
            imageHash: IMAGE_HASH,
            mrTdHash: MRTD_HASH,
            reportDataPrefix: publicKeyHash,
            reportDataSuffix: REPORT_DATA_SUFFIX
        });
    }

    function _publicKey() internal pure returns (bytes memory publicKey) {
        publicKey = new bytes(65);
        publicKey[0] = 0x04;
        for (uint256 i = 1; i < publicKey.length; i++) {
            publicKey[i] = bytes1(uint8(i));
        }
    }

    function _mockRiscZeroVerify(bytes32 programId, bytes memory output, bytes memory proofBytes) internal {
        vm.mockCall(
            mockRiscZeroVerifier,
            abi.encodeWithSelector(
                bytes4(keccak256("verify(bytes,bytes32,bytes32)")), proofBytes, programId, sha256(output)
            ),
            ""
        );
    }

    function _mockSP1Verify(bytes32 programId, bytes memory output, bytes memory proofBytes) internal {
        vm.mockCall(
            mockSP1Verifier,
            abi.encodeWithSelector(
                bytes4(keccak256("verifyProof(bytes32,bytes,bytes)")), programId, output, proofBytes
            ),
            ""
        );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "forge-std/Test.sol";

import { TDXTcbStatus, TDXVerificationResult, TDXVerifierJournal } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract TDXVerifierTest is Test {
    TDXVerifier internal verifier;

    address internal proofSubmitter;
    address internal mockRiscZeroVerifier;

    bytes32 internal constant ROOT_CA_HASH = keccak256("intel-root-ca");
    bytes32 internal constant WRONG_ROOT_CA_HASH = keccak256("wrong-root-ca");
    bytes32 internal constant VERIFIER_ID = keccak256("tdx-verifier-id");
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    bytes32 internal constant REPORT_DATA_SUFFIX = keccak256("multiproof-tdx-poc");

    uint64 internal constant MAX_TIME_DIFF = 3600;
    uint256 internal constant NOW = 1_700_000_000;

    function setUp() public {
        vm.warp(NOW);

        proofSubmitter = address(this);
        mockRiscZeroVerifier = makeAddr("mock-risc-zero");

        verifier = new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, proofSubmitter, mockRiscZeroVerifier, VERIFIER_ID);
    }

    function testVerifySucceedsWithRiscZeroProofAndAllowedJournal() public {
        TDXVerifierJournal memory journal = _successJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        TDXVerifierJournal memory result = verifier.verify(output, proofBytes);

        assertEq(result.signer, journal.signer);
        assertEq(result.imageHash, IMAGE_HASH);
        assertEq(uint256(result.tcbStatus), uint256(TDXTcbStatus.UpToDate));
    }

    function testVerifyRevertsIfNotProofSubmitter() public {
        bytes memory output = abi.encode(_successJournal());

        vm.prank(makeAddr("not-submitter"));
        vm.expectRevert(TDXVerifier.CallerNotProofSubmitter.selector);
        verifier.verify(output, "");
    }

    function testConstructorRevertsIfZeroRiscZeroVerifier() public {
        vm.expectRevert(TDXVerifier.ZeroRiscZeroVerifier.selector);
        new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, proofSubmitter, address(0), VERIFIER_ID);
    }

    function testConstructorRevertsIfZeroProofSubmitter() public {
        vm.expectRevert(TDXVerifier.ZeroProofSubmitter.selector);
        new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, address(0), mockRiscZeroVerifier, VERIFIER_ID);
    }

    function testVerifyRevertsWhenGuestReportsFailure() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.result = TDXVerificationResult.PckCertChainInvalid;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.TDXVerificationFailed.selector, journal.result));
        verifier.verify(output, proofBytes);
    }

    function testVerifyRevertsWhenRootCaHashMismatches() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.rootCaHash = WRONG_ROOT_CA_HASH;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(TDXVerifier.RootCaHashMismatch.selector, ROOT_CA_HASH, WRONG_ROOT_CA_HASH)
        );
        verifier.verify(output, proofBytes);
    }

    function testVerifyRevertsWhenTcbStatusIsNotAllowed() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.tcbStatus = TDXTcbStatus.ConfigurationNeeded;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.TcbStatusNotAllowed.selector, journal.tcbStatus));
        verifier.verify(output, proofBytes);
    }

    function testVerifyRevertsWhenCollateralExpired() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.collateralExpiration = uint64(block.timestamp);
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.CollateralExpired.selector, journal.collateralExpiration));
        verifier.verify(output, proofBytes);
    }

    function testVerifyRevertsWhenTimestampTooOld() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp - MAX_TIME_DIFF) * 1000;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(
                TDXVerifier.InvalidTimestamp.selector, uint64(block.timestamp - MAX_TIME_DIFF), block.timestamp
            )
        );
        verifier.verify(output, proofBytes);
    }

    function testVerifyRevertsWhenTimestampIsFromFuture() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp) * 1000;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(TDXVerifier.InvalidTimestamp.selector, uint64(block.timestamp), block.timestamp)
        );
        verifier.verify(output, proofBytes);
    }

    function testVerifyRevertsWhenReportDataDoesNotBindPublicKey() public {
        TDXVerifierJournal memory journal = _successJournal();
        bytes32 expected = journal.reportDataPrefix;
        journal.reportDataPrefix = keccak256("wrong-report-data");
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(
            abi.encodeWithSelector(TDXVerifier.ReportDataMismatch.selector, expected, journal.reportDataPrefix)
        );
        verifier.verify(output, proofBytes);
    }

    function testVerifyRevertsWhenSignerDoesNotMatchPublicKey() public {
        TDXVerifierJournal memory journal = _successJournal();
        address expected = journal.signer;
        journal.signer = makeAddr("wrong-signer");
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(abi.encodeWithSelector(TDXVerifier.SignerMismatch.selector, expected, journal.signer));
        verifier.verify(output, proofBytes);
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
            publicKey: publicKey,
            signer: address(uint160(uint256(publicKeyHash))),
            imageHash: IMAGE_HASH,
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
        _mockRiscZeroVerify(mockRiscZeroVerifier, programId, output, proofBytes);
    }

    function _mockRiscZeroVerify(
        address verifierAddress,
        bytes32 programId,
        bytes memory output,
        bytes memory proofBytes
    )
        internal
    {
        vm.mockCall(
            verifierAddress,
            abi.encodeWithSelector(
                bytes4(keccak256("verify(bytes,bytes32,bytes32)")), proofBytes, programId, sha256(output)
            ),
            ""
        );
    }
}

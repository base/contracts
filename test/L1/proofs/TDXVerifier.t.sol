// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "forge-std/Test.sol";

import { TDXTcbStatus, TDXVerifierJournal } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract TDXVerifierTest is Test {
    TDXVerifier internal verifier;

    address internal mockRiscZeroVerifier;

    bytes32 internal constant ROOT_CA_HASH = keccak256("intel-root-ca");
    bytes32 internal constant WRONG_ROOT_CA_HASH = keccak256("wrong-root-ca");
    bytes32 internal constant VERIFIER_ID = keccak256("tdx-verifier-id");
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    bytes32 internal constant REPORT_DATA_SUFFIX = keccak256("multiproof-tdx-poc");
    bytes32 internal constant PUBLIC_KEY_X = hex"0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
    bytes32 internal constant PUBLIC_KEY_Y = hex"2122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f40";

    uint64 internal constant MAX_TIME_DIFF = 3600;
    uint256 internal constant NOW = 1_700_000_000;

    function setUp() public {
        vm.warp(NOW);

        mockRiscZeroVerifier = makeAddr("mock-risc-zero");

        verifier = new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, mockRiscZeroVerifier, VERIFIER_ID);
    }

    function testVerifySucceedsWithRiscZeroProofAndAllowedJournal() public {
        TDXVerifierJournal memory journal = _successJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        (address signer, bytes32 imageHash, bytes32 reportDataSuffix) = verifier.verify(output, proofBytes);

        assertEq(signer, _signer());
        assertEq(imageHash, IMAGE_HASH);
        assertEq(reportDataSuffix, REPORT_DATA_SUFFIX);
    }

    function testConstructorRevertsIfZeroInput() public {
        vm.expectRevert(TDXVerifier.ZeroInput.selector);
        new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, address(0), VERIFIER_ID);
    }

    function testVerifyRevertsWhenGuestReportsFailure() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.success = false;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = hex"1234";
        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.expectRevert(TDXVerifier.TDXVerificationFailed.selector);
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

    function _successJournal() internal view returns (TDXVerifierJournal memory journal) {
        bytes32 publicKeyHash = _publicKeyHash();

        journal = TDXVerifierJournal({
            success: true,
            tcbStatus: TDXTcbStatus.UpToDate,
            timestamp: uint64(block.timestamp - 1) * 1000,
            collateralExpiration: uint64(block.timestamp + 1 days),
            rootCaHash: ROOT_CA_HASH,
            publicKeyX: PUBLIC_KEY_X,
            publicKeyY: PUBLIC_KEY_Y,
            imageHash: IMAGE_HASH,
            reportDataPrefix: publicKeyHash,
            reportDataSuffix: REPORT_DATA_SUFFIX
        });
    }

    function _publicKeyHash() internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(PUBLIC_KEY_X, PUBLIC_KEY_Y));
    }

    function _signer() internal pure returns (address) {
        return address(uint160(uint256(_publicKeyHash())));
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

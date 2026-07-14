// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "forge-std/Test.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

import { TDXTcbStatus, TDXVerificationResult, TDXVerifierJournal } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract TDXVerifierTest is Test {
    TDXVerifier internal verifier;

    address internal constant MOCK_RISC_ZERO_VERIFIER = address(0x1234);

    bytes32 internal constant ROOT_CA_HASH = keccak256("intel-root-ca");
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    bytes32 internal constant PUBLIC_KEY_X = bytes32(uint256(1));
    bytes32 internal constant PUBLIC_KEY_Y = bytes32(uint256(2));

    uint64 internal constant MAX_TIME_DIFF = 3600;

    function setUp() public {
        vm.warp(1_700_000_000);

        verifier = new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, MOCK_RISC_ZERO_VERIFIER, keccak256("tdx-verifier-id"));
        vm.mockCall(MOCK_RISC_ZERO_VERIFIER, abi.encodeWithSelector(IRiscZeroVerifier.verify.selector), "");
    }

    function testVerifySucceedsWithRiscZeroProofAndAllowedJournal() public view {
        (address signer, bytes32 imageHash) = verifier.verify(abi.encode(_successJournal()), hex"1234");

        assertEq(signer, address(uint160(uint256(_publicKeyHash()))));
        assertEq(imageHash, IMAGE_HASH);
    }

    function testConstructorRevertsIfZeroInput() public {
        vm.expectRevert(TDXVerifier.ZeroInput.selector);
        new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, address(0), keccak256("tdx-verifier-id"));
    }

    function testVerifyRevertsWhenRootCaHashMismatches() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.rootCaHash = keccak256("wrong-root-ca");

        _expectVerifyRevert(journal, TDXVerifier.RootCaHashMismatch.selector);
    }

    function testVerifyRevertsWhenGuestReportedFailure() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.result = TDXVerificationResult.InvalidQuote;

        _expectVerifyRevert(journal, TDXVerifier.VerificationFailed.selector);
    }

    function testVerifyRevertsWhenTcbStatusIsNotAllowed() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.tcbStatus = TDXTcbStatus.ConfigurationNeeded;

        _expectVerifyRevert(journal, TDXVerifier.TcbStatusNotAllowed.selector);
    }

    function testVerifyRevertsWhenCollateralExpired() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.collateralExpiration = uint64(block.timestamp);

        _expectVerifyRevert(journal, TDXVerifier.CollateralExpired.selector);
    }

    function testVerifyRevertsWhenTimestampTooOld() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp - MAX_TIME_DIFF) * 1000;

        _expectVerifyRevert(journal, TDXVerifier.InvalidTimestamp.selector);
    }

    function testVerifyRevertsWhenTimestampIsFromFuture() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp) * 1000;

        _expectVerifyRevert(journal, TDXVerifier.InvalidTimestamp.selector);
    }

    function testVerifyRevertsWhenReportDataDoesNotBindPublicKey() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.reportDataPrefix = keccak256("wrong-report-data");

        _expectVerifyRevert(journal, TDXVerifier.ReportDataMismatch.selector);
    }

    function testVerifyRevertsForDebugTd() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.tdAttributes = 1;

        _expectVerifyRevert(journal, TDXVerifier.DebugTdNotAllowed.selector);
    }

    function testVerifyRevertsWhenRegistrationContextDoesNotMatch() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.chainId = uint64(block.chainid + 1);

        _expectVerifyRevert(journal, TDXVerifier.ChainIdMismatch.selector);

        journal = _successJournal();
        journal.registryAddress = makeAddr("wrong-registry");

        _expectVerifyRevert(journal, TDXVerifier.RegistryAddressMismatch.selector);
    }

    function _successJournal() internal view returns (TDXVerifierJournal memory journal) {
        journal = TDXVerifierJournal({
            result: TDXVerificationResult.Success,
            tcbStatus: TDXTcbStatus.UpToDate,
            timestamp: uint64(block.timestamp - 1) * 1000,
            collateralExpiration: uint64(block.timestamp + 1 days),
            rootCaHash: ROOT_CA_HASH,
            pckCertHash: keccak256("pck-cert"),
            tcbInfoHash: keccak256("tcb-info"),
            qeIdentityHash: keccak256("qe-identity"),
            publicKey: abi.encodePacked(bytes1(0x04), PUBLIC_KEY_X, PUBLIC_KEY_Y),
            signer: address(uint160(uint256(_publicKeyHash()))),
            imageHash: IMAGE_HASH,
            mrTdHash: keccak256("mrtd"),
            reportDataPrefix: _publicKeyHash(),
            reportDataSuffix: keccak256("report-data"),
            tdAttributes: 0,
            chainId: uint64(block.chainid),
            registryAddress: address(this)
        });
    }

    function _publicKeyHash() internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(PUBLIC_KEY_X, PUBLIC_KEY_Y));
    }

    function _expectVerifyRevert(TDXVerifierJournal memory journal, bytes4 expectedRevert) internal {
        vm.expectPartialRevert(expectedRevert);
        verifier.verify(abi.encode(journal), hex"1234");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "forge-std/Test.sol";

import { IRiscZeroVerifier } from "lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

import { TDXTcbStatus, TDXVerifierJournal } from "interfaces/L1/proofs/tee/ITDXVerifier.sol";

import { TDXVerifier } from "src/L1/proofs/tee/TDXVerifier.sol";

contract TDXVerifierTest is Test {
    TDXVerifier internal verifier;

    address internal constant MOCK_RISC_ZERO_VERIFIER = address(0x1234);

    bytes32 internal constant ROOT_CA_HASH = keccak256("intel-root-ca");
    bytes32 internal constant VERIFIER_ID = keccak256("tdx-verifier-id");
    bytes32 internal constant IMAGE_HASH = keccak256("tdx-image");
    bytes32 internal constant PUBLIC_KEY_X = hex"0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
    bytes32 internal constant PUBLIC_KEY_Y = hex"2122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f40";

    uint64 internal constant MAX_TIME_DIFF = 3600;
    uint256 internal constant NOW = 1_700_000_000;

    function setUp() public {
        vm.warp(NOW);

        verifier = new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, MOCK_RISC_ZERO_VERIFIER, VERIFIER_ID);
        vm.mockCall(MOCK_RISC_ZERO_VERIFIER, abi.encodeWithSelector(IRiscZeroVerifier.verify.selector), "");
    }

    function testVerifySucceedsWithRiscZeroProofAndAllowedJournal() public view {
        (address signer, bytes32 imageHash) = _verify(_successJournal());

        assertEq(signer, address(uint160(uint256(_publicKeyHash()))));
        assertEq(imageHash, IMAGE_HASH);
    }

    function testConstructorRevertsIfZeroInput() public {
        vm.expectRevert(TDXVerifier.ZeroInput.selector);
        new TDXVerifier(MAX_TIME_DIFF, ROOT_CA_HASH, address(0), VERIFIER_ID);
    }

    function testVerifyRevertsWhenRootCaHashMismatches() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.rootCaHash = keccak256("wrong-root-ca");

        _expectVerifyRevert(
            journal, abi.encodeWithSelector(TDXVerifier.RootCaHashMismatch.selector, ROOT_CA_HASH, journal.rootCaHash)
        );
    }

    function testVerifyRevertsWhenTcbStatusIsNotAllowed() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.tcbStatus = TDXTcbStatus.ConfigurationNeeded;

        _expectVerifyRevert(
            journal, abi.encodeWithSelector(TDXVerifier.TcbStatusNotAllowed.selector, journal.tcbStatus)
        );
    }

    function testVerifyRevertsWhenCollateralExpired() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.collateralExpiration = uint64(block.timestamp);

        _expectVerifyRevert(
            journal, abi.encodeWithSelector(TDXVerifier.CollateralExpired.selector, journal.collateralExpiration)
        );
    }

    function testVerifyRevertsWhenTimestampTooOld() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp - MAX_TIME_DIFF) * 1000;

        _expectVerifyRevert(
            journal,
            abi.encodeWithSelector(
                TDXVerifier.InvalidTimestamp.selector, uint64(block.timestamp - MAX_TIME_DIFF), block.timestamp
            )
        );
    }

    function testVerifyRevertsWhenTimestampIsFromFuture() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.timestamp = uint64(block.timestamp) * 1000;

        _expectVerifyRevert(
            journal,
            abi.encodeWithSelector(TDXVerifier.InvalidTimestamp.selector, uint64(block.timestamp), block.timestamp)
        );
    }

    function testVerifyRevertsWhenReportDataDoesNotBindPublicKey() public {
        TDXVerifierJournal memory journal = _successJournal();
        journal.reportDataPrefix = keccak256("wrong-report-data");

        _expectVerifyRevert(
            journal,
            abi.encodeWithSelector(TDXVerifier.ReportDataMismatch.selector, _publicKeyHash(), journal.reportDataPrefix)
        );
    }

    function _successJournal() internal view returns (TDXVerifierJournal memory journal) {
        journal = TDXVerifierJournal({
            tcbStatus: TDXTcbStatus.UpToDate,
            timestamp: uint64(block.timestamp - 1) * 1000,
            collateralExpiration: uint64(block.timestamp + 1 days),
            rootCaHash: ROOT_CA_HASH,
            publicKeyX: PUBLIC_KEY_X,
            publicKeyY: PUBLIC_KEY_Y,
            imageHash: IMAGE_HASH,
            reportDataPrefix: _publicKeyHash()
        });
    }

    function _publicKeyHash() internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(PUBLIC_KEY_X, PUBLIC_KEY_Y));
    }

    function _verify(TDXVerifierJournal memory journal) internal view returns (address signer, bytes32 imageHash) {
        return verifier.verify(abi.encode(journal), hex"1234");
    }

    function _expectVerifyRevert(TDXVerifierJournal memory journal, bytes memory expectedRevert) internal {
        vm.expectRevert(expectedRevert);
        verifier.verify(abi.encode(journal), hex"1234");
    }
}

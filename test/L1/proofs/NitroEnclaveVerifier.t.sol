// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test } from "lib/forge-std/src/Test.sol";
import { Ownable } from "lib/solady/src/auth/Ownable.sol";

import {
    ZkCoProcessorType,
    ZkCoProcessorConfig,
    VerifierJournal,
    BatchVerifierJournal,
    VerificationResult,
    Pcr
} from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";

import { NitroEnclaveVerifier } from "src/L1/proofs/tee/NitroEnclaveVerifier.sol";

contract NitroEnclaveVerifierTest is Test {
    NitroEnclaveVerifier internal verifier;

    address internal owner;
    address internal submitter;
    address internal revokerAddr;
    address internal mockRiscZeroVerifier;
    address internal mockSP1Verifier;

    bytes32 internal constant ROOT_CERT = keccak256("root-cert");
    bytes32 internal constant INTERMEDIATE_CERT_1 = keccak256("intermediate-cert-1");
    bytes32 internal constant INTERMEDIATE_CERT_2 = keccak256("intermediate-cert-2");
    bytes32 internal constant VERIFIER_ID = keccak256("verifier-id");
    bytes32 internal constant AGGREGATOR_ID = keccak256("aggregator-id");
    bytes32 internal constant VERIFIER_PROOF_ID = keccak256("verifier-proof-id");
    bytes4 internal constant DEFAULT_PROOF_SELECTOR = bytes4(0);
    bytes4 internal constant NON_DEFAULT_PROOF_SELECTOR = 0x00000001;
    bytes4 internal constant TEST_ROUTE_SELECTOR = bytes4(keccak256("test"));
    bytes4 internal constant RISC_ZERO_VERIFY_SELECTOR = bytes4(keccak256("verify(bytes,bytes32,bytes32)"));
    bytes4 internal constant SP1_VERIFY_PROOF_SELECTOR = bytes4(keccak256("verifyProof(bytes32,bytes,bytes)"));
    address internal constant FROZEN_ROUTE_SENTINEL = address(0xdead);

    uint64 internal constant MAX_TIME_DIFF = 3600; // 1 hour

    // Realistic timestamp so timestamp validation tests work correctly
    uint256 internal constant REALISTIC_TIMESTAMP = 1_700_000_000;

    // Expiry timestamps for test certs (well after REALISTIC_TIMESTAMP)
    uint64 internal constant INTERMEDIATE_CERT_1_EXPIRY = 1_800_000_000; // ~2027
    uint64 internal constant ROOT_CERT_EXPIRY = 1_900_000_000;
    uint64 internal constant NEW_LEAF_CERT_EXPIRY = 1_700_100_000; // ~28 hours after REALISTIC_TIMESTAMP

    function setUp() public {
        vm.warp(REALISTIC_TIMESTAMP);

        owner = address(this);
        submitter = makeAddr("submitter");
        revokerAddr = makeAddr("revoker");
        mockRiscZeroVerifier = makeAddr("mock-riscZero-verifier");
        mockSP1Verifier = makeAddr("mock-sp1-verifier");

        bytes32[] memory trustedCerts = new bytes32[](1);
        trustedCerts[0] = INTERMEDIATE_CERT_1;

        uint64[] memory trustedCertExpiries = new uint64[](1);
        trustedCertExpiries[0] = INTERMEDIATE_CERT_1_EXPIRY;

        ZkCoProcessorConfig memory zkCfg = _zkConfig(mockSP1Verifier);

        verifier = new NitroEnclaveVerifier(
            owner,
            MAX_TIME_DIFF,
            trustedCerts,
            trustedCertExpiries,
            ROOT_CERT,
            submitter,
            revokerAddr,
            ZkCoProcessorType.Succinct,
            zkCfg,
            VERIFIER_PROOF_ID
        );
    }

    // ============ Constructor Tests ============

    function testConstructorSetsOwner() public view {
        assertEq(verifier.owner(), owner);
    }

    function testConstructorSetsMaxTimeDiff() public view {
        assertEq(verifier.maxTimeDiff(), MAX_TIME_DIFF);
    }

    function testConstructorSetsTrustedCerts() public view {
        assertEq(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1), INTERMEDIATE_CERT_1_EXPIRY);
        assertEq(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_2), 0);
    }

    function testConstructorRevertsIfZeroMaxTimeDiff() public {
        bytes32[] memory certs = new bytes32[](0);
        uint64[] memory expiries = new uint64[](0);
        ZkCoProcessorConfig memory zkCfg =
            ZkCoProcessorConfig({ verifierId: bytes32(0), aggregatorId: bytes32(0), zkVerifier: address(0) });
        vm.expectRevert(NitroEnclaveVerifier.ZeroMaxTimeDiff.selector);
        new NitroEnclaveVerifier(
            owner, 0, certs, expiries, bytes32(0), submitter, address(0), ZkCoProcessorType.Succinct, zkCfg, bytes32(0)
        );
    }

    function testConstructorRevertsIfCertExpiriesLengthMismatch() public {
        bytes32[] memory certs = new bytes32[](1);
        certs[0] = INTERMEDIATE_CERT_1;
        uint64[] memory expiries = new uint64[](0);
        ZkCoProcessorConfig memory zkCfg =
            ZkCoProcessorConfig({ verifierId: bytes32(0), aggregatorId: bytes32(0), zkVerifier: address(0) });
        vm.expectRevert(abi.encodeWithSelector(NitroEnclaveVerifier.CertExpiriesLengthMismatch.selector, 1, 0));
        new NitroEnclaveVerifier(
            owner,
            MAX_TIME_DIFF,
            certs,
            expiries,
            bytes32(0),
            submitter,
            address(0),
            ZkCoProcessorType.Succinct,
            zkCfg,
            bytes32(0)
        );
    }

    // ============ setRootCert Tests ============

    function testSetRootCert() public {
        bytes32 newRoot = keccak256("new-root");
        verifier.setRootCert(newRoot);
        assertEq(verifier.rootCert(), newRoot);
    }

    function testSetRootCertRevertsIfNotOwner() public {
        _expectNotOwnerRevert(submitter);
        verifier.setRootCert(keccak256("bad"));
    }

    // ============ setMaxTimeDiff Tests ============

    function testSetMaxTimeDiff() public {
        uint64 newTimeDiff = 7200;
        verifier.setMaxTimeDiff(newTimeDiff);
        assertEq(verifier.maxTimeDiff(), newTimeDiff);
    }

    function testSetMaxTimeDiffRevertsIfZero() public {
        vm.expectRevert(NitroEnclaveVerifier.ZeroMaxTimeDiff.selector);
        verifier.setMaxTimeDiff(0);
    }

    function testSetMaxTimeDiffRevertsIfNotOwner() public {
        _expectNotOwnerRevert(submitter);
        verifier.setMaxTimeDiff(7200);
    }

    // ============ setProofSubmitter Tests ============

    function testSetProofSubmitter() public {
        address newSubmitter = makeAddr("new-submitter");
        verifier.setProofSubmitter(newSubmitter);
        assertEq(verifier.proofSubmitter(), newSubmitter);
    }

    function testSetProofSubmitterRevertsIfZeroAddress() public {
        vm.expectRevert(NitroEnclaveVerifier.ZeroProofSubmitter.selector);
        verifier.setProofSubmitter(address(0));
    }

    function testSetProofSubmitterRevertsIfNotOwner() public {
        _expectNotOwnerRevert(submitter);
        verifier.setProofSubmitter(makeAddr("anyone"));
    }

    // ============ setZkConfiguration Tests ============

    function testSetZkConfiguration() public {
        ZkCoProcessorConfig memory config = _zkConfig(mockRiscZeroVerifier);

        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);

        ZkCoProcessorConfig memory stored = verifier.getZkConfig(ZkCoProcessorType.RiscZero);
        assertEq(stored.verifierId, VERIFIER_ID);
        assertEq(stored.aggregatorId, AGGREGATOR_ID);
        assertEq(stored.zkVerifier, mockRiscZeroVerifier);

        assertEq(verifier.getVerifierProofId(ZkCoProcessorType.RiscZero), VERIFIER_PROOF_ID);
    }

    function testSetZkConfigurationRevertsIfNotOwner() public {
        ZkCoProcessorConfig memory config = _zkConfig(mockRiscZeroVerifier);

        _expectNotOwnerRevert(submitter);
        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);
    }

    // ============ revokeCert Tests ============

    function testRevokeCert() public {
        assertGt(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1), 0);
        verifier.revokeCert(INTERMEDIATE_CERT_1);
        assertEq(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1), 0);
    }

    function testRevokeCertRevertsIfNotTrusted() public {
        bytes32 unknown = keccak256("unknown-cert");
        vm.expectRevert(abi.encodeWithSelector(NitroEnclaveVerifier.CertificateNotFound.selector, unknown));
        verifier.revokeCert(unknown);
    }

    function testRevokeCertRevertsIfNotOwnerOrRevoker() public {
        vm.prank(submitter);
        vm.expectRevert(NitroEnclaveVerifier.CallerNotOwnerOrRevoker.selector);
        verifier.revokeCert(INTERMEDIATE_CERT_1);
    }

    // ============ Revoker Role Tests ============

    function testConstructorSetsRevoker() public view {
        assertEq(verifier.revoker(), revokerAddr);
    }

    function testConstructorAcceptsZeroRevoker() public {
        bytes32[] memory certs = new bytes32[](0);
        uint64[] memory expiries = new uint64[](0);
        ZkCoProcessorConfig memory zkCfg = _zkConfig(mockSP1Verifier);
        NitroEnclaveVerifier v = new NitroEnclaveVerifier(
            owner,
            MAX_TIME_DIFF,
            certs,
            expiries,
            ROOT_CERT,
            submitter,
            address(0),
            ZkCoProcessorType.Succinct,
            zkCfg,
            VERIFIER_PROOF_ID
        );
        assertEq(v.revoker(), address(0));
    }

    function testRevokerCanRevokeCert() public {
        assertGt(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1), 0);
        vm.prank(revokerAddr);
        verifier.revokeCert(INTERMEDIATE_CERT_1);
        assertEq(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1), 0);
    }

    function testSetRevoker() public {
        address newRevoker = makeAddr("new-revoker");
        verifier.setRevoker(newRevoker);
        assertEq(verifier.revoker(), newRevoker);
    }

    function testSetRevokerToZeroDisablesRole() public {
        verifier.setRevoker(address(0));
        assertEq(verifier.revoker(), address(0));

        vm.prank(revokerAddr);
        vm.expectRevert(NitroEnclaveVerifier.CallerNotOwnerOrRevoker.selector);
        verifier.revokeCert(INTERMEDIATE_CERT_1);
    }

    function testSetRevokerEmitsEvent() public {
        address newRevoker = makeAddr("new-revoker");
        vm.expectEmit(true, false, false, false);
        emit NitroEnclaveVerifier.RevokerUpdated(newRevoker);
        verifier.setRevoker(newRevoker);
    }

    function testSetRevokerRevertsIfNotOwner() public {
        _expectNotOwnerRevert(submitter);
        verifier.setRevoker(makeAddr("anyone"));
    }

    function testSetRevokerRevertsIfCalledByRevoker() public {
        _expectNotOwnerRevert(revokerAddr);
        verifier.setRevoker(makeAddr("anyone"));
    }

    // ============ updateVerifierId Tests ============

    function testUpdateVerifierId() public {
        _setUpRiscZeroConfig();

        bytes32 newVerifierId = keccak256("new-verifier-id");
        bytes32 newVerifierProofId = keccak256("new-verifier-proof-id");
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, newVerifierId, newVerifierProofId);

        ZkCoProcessorConfig memory config = verifier.getZkConfig(ZkCoProcessorType.RiscZero);
        assertEq(config.verifierId, newVerifierId);
        assertEq(verifier.getVerifierProofId(ZkCoProcessorType.RiscZero), newVerifierProofId);
    }

    function testUpdateVerifierIdRevertsIfZero() public {
        _setUpRiscZeroConfig();
        vm.expectRevert(NitroEnclaveVerifier.ZeroProgramId.selector);
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, bytes32(0), bytes32(0));
    }

    function testUpdateVerifierIdRevertsIfSame() public {
        _setUpRiscZeroConfig();
        vm.expectRevert(
            abi.encodeWithSelector(
                NitroEnclaveVerifier.ProgramIdAlreadyLatest.selector, ZkCoProcessorType.RiscZero, VERIFIER_ID
            )
        );
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, VERIFIER_ID, VERIFIER_PROOF_ID);
    }

    function testUpdateVerifierIdRevertsIfNotOwner() public {
        _setUpRiscZeroConfig();
        _expectNotOwnerRevert(submitter);
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, keccak256("new"), keccak256("proof"));
    }

    // ============ updateAggregatorId Tests ============

    function testUpdateAggregatorId() public {
        _setUpRiscZeroConfig();

        bytes32 newAggregatorId = keccak256("new-aggregator-id");
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, newAggregatorId);

        ZkCoProcessorConfig memory config = verifier.getZkConfig(ZkCoProcessorType.RiscZero);
        assertEq(config.aggregatorId, newAggregatorId);
    }

    function testUpdateAggregatorIdRevertsIfZero() public {
        _setUpRiscZeroConfig();
        vm.expectRevert(NitroEnclaveVerifier.ZeroProgramId.selector);
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, bytes32(0));
    }

    function testUpdateAggregatorIdRevertsIfSame() public {
        _setUpRiscZeroConfig();
        vm.expectRevert(
            abi.encodeWithSelector(
                NitroEnclaveVerifier.ProgramIdAlreadyLatest.selector, ZkCoProcessorType.RiscZero, AGGREGATOR_ID
            )
        );
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, AGGREGATOR_ID);
    }

    function testUpdateAggregatorIdRevertsIfNotOwner() public {
        _setUpRiscZeroConfig();
        _expectNotOwnerRevert(submitter);
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, keccak256("new"));
    }

    // ============ addVerifyRoute / freezeVerifyRoute Tests ============

    function testAddVerifyRoute() public {
        address routeVerifier = makeAddr("route-verifier");

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR, routeVerifier);
        assertEq(verifier.getZkVerifier(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR), routeVerifier);
    }

    function testAddVerifyRouteRevertsIfZeroAddress() public {
        vm.expectRevert(NitroEnclaveVerifier.ZeroVerifierAddress.selector);
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, NON_DEFAULT_PROOF_SELECTOR, address(0));
    }

    function testAddVerifyRouteRevertsIfFrozenSentinel() public {
        vm.expectRevert(NitroEnclaveVerifier.InvalidVerifierAddress.selector);
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, NON_DEFAULT_PROOF_SELECTOR, FROZEN_ROUTE_SENTINEL);
    }

    function testAddVerifyRouteRevertsIfNotOwner() public {
        _expectNotOwnerRevert(submitter);
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR, makeAddr("v"));
    }

    function testFreezeVerifyRoute() public {
        address routeVerifier = makeAddr("route-verifier");

        _addAndFreezeVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR, routeVerifier);

        _expectZkRouteFrozenRevert(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR);
        verifier.getZkVerifier(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR);
    }

    function testAddVerifyRouteRevertsIfFrozen() public {
        address routeVerifier = makeAddr("route-verifier");

        _addAndFreezeVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR, routeVerifier);

        _expectZkRouteFrozenRevert(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR);
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR, routeVerifier);
    }

    function testFreezeVerifyRouteRevertsIfAlreadyFrozen() public {
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR, makeAddr("v"));
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR);

        _expectZkRouteFrozenRevert(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR);
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR);
    }

    function testFreezeVerifyRouteRevertsIfNotOwner() public {
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR, makeAddr("v"));

        _expectNotOwnerRevert(submitter);
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, TEST_ROUTE_SELECTOR);
    }

    // ============ getZkVerifier Tests ============

    function testGetZkVerifierFallsBackToDefault() public {
        _setUpRiscZeroConfig();

        bytes4 unknownSelector = bytes4(0xdeadbeef);
        assertEq(verifier.getZkVerifier(ZkCoProcessorType.RiscZero, unknownSelector), mockRiscZeroVerifier);
    }

    function testGetZkVerifierReturnsRouteSpecific() public {
        _setUpRiscZeroConfig();

        bytes4 selector = bytes4(keccak256("special"));
        address routeVerifier = makeAddr("route-verifier");
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);

        assertEq(verifier.getZkVerifier(ZkCoProcessorType.RiscZero, selector), routeVerifier);
    }

    // ============ checkTrustedIntermediateCerts Tests ============

    function testCheckTrustedIntermediateCerts() public view {
        bytes32[][] memory reportCerts = new bytes32[][](1);
        reportCerts[0] = new bytes32[](3);
        reportCerts[0][0] = ROOT_CERT;
        reportCerts[0][1] = INTERMEDIATE_CERT_1;
        reportCerts[0][2] = INTERMEDIATE_CERT_2;

        uint8[] memory results = verifier.checkTrustedIntermediateCerts(reportCerts);
        assertEq(results[0], 2); // root + 1 intermediate trusted
    }

    function testCheckTrustedIntermediateCertsRevertsIfWrongRoot() public {
        bytes32 wrongRoot = keccak256("wrong-root");
        bytes32[][] memory reportCerts = new bytes32[][](1);
        reportCerts[0] = new bytes32[](1);
        reportCerts[0][0] = wrongRoot;

        vm.expectRevert(abi.encodeWithSelector(NitroEnclaveVerifier.RootCertMismatch.selector, ROOT_CERT, wrongRoot));
        verifier.checkTrustedIntermediateCerts(reportCerts);
    }

    // ============ verify — access control ============

    function testVerifyRevertsIfNotProofSubmitter() public {
        vm.expectRevert(NitroEnclaveVerifier.CallerNotProofSubmitter.selector);
        verifier.verify("", ZkCoProcessorType.RiscZero, "");
    }

    // ============ verify — ZkVerifierNotConfigured ============

    function testVerifyRevertsIfZkVerifierNotConfigured() public {
        ZkCoProcessorConfig memory config = _zkConfig(address(0));
        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);

        bytes memory proofBytes = _proofBytes();

        vm.prank(submitter);
        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkVerifierNotConfigured.selector, ZkCoProcessorType.RiscZero)
        );
        verifier.verify("", ZkCoProcessorType.RiscZero, proofBytes);
    }

    // ============ verify — Unknown_Zk_Coprocessor ============

    function testVerifyRevertsForUnknownCoprocessor() public {
        ZkCoProcessorConfig memory config = _zkConfig(mockRiscZeroVerifier);
        verifier.setZkConfiguration(ZkCoProcessorType.Unknown, config, VERIFIER_PROOF_ID);

        bytes memory proofBytes = _proofBytes();

        vm.prank(submitter);
        vm.expectRevert(NitroEnclaveVerifier.Unknown_Zk_Coprocessor.selector);
        verifier.verify("", ZkCoProcessorType.Unknown, proofBytes);
    }

    // ============ verify — ZkRouteFrozen during verify() ============

    function testVerifyRevertsIfRouteFrozen() public {
        _setUpRiscZeroConfig();

        bytes4 selector = DEFAULT_PROOF_SELECTOR;
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, makeAddr("route-v"));
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        bytes memory proofBytes = _proofBytes();

        vm.prank(submitter);
        _expectZkRouteFrozenRevert(ZkCoProcessorType.RiscZero, selector);
        verifier.verify("", ZkCoProcessorType.RiscZero, proofBytes);
    }

    // ============ verify — RiscZero happy path ============

    function testVerifySuccessfulJournal() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory result = _verifyRiscZeroJournal(_createSuccessJournal());

        _assertVerificationResult(result, VerificationResult.Success);
    }

    function testVerifyJournalRootCertNotTrusted() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        journal.certs[0] = keccak256("wrong-root");
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.RootCertNotTrusted);
    }

    function testVerifyJournalRootCertNotTrustedZeroPrefixLen() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        journal.trustedCertsPrefixLen = 0;
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.RootCertNotTrusted);
    }

    function testVerifyJournalIntermediateCertNotTrusted() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        // Replace trusted intermediate with untrusted one, but keep trustedCertsPrefixLen = 2
        journal.certs[1] = keccak256("untrusted-intermediate");
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.IntermediateCertsNotTrusted);
    }

    function testVerifyJournalInvalidTimestampTooOld() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        // Set timestamp far in the past — more than maxTimeDiff seconds ago (in ms)
        journal.timestamp = uint64(block.timestamp - MAX_TIME_DIFF - 1) * 1000;
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.InvalidTimestamp);
    }

    function testVerifyJournalInvalidTimestampFuture() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        // Set timestamp in the future (converted to ms)
        journal.timestamp = uint64(block.timestamp + 100) * 1000;
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.InvalidTimestamp);
    }

    function testVerifyCachesNewCerts() public {
        _setUpRiscZeroConfig();

        bytes32 newCert = keccak256("new-leaf-cert");
        assertEq(verifier.trustedIntermediateCerts(newCert), 0);

        VerifierJournal memory journal = _createSuccessJournalWithLeaf(newCert, NEW_LEAF_CERT_EXPIRY);
        _verifyRiscZeroJournal(journal);

        assertEq(verifier.trustedIntermediateCerts(newCert), NEW_LEAF_CERT_EXPIRY);
    }

    function testVerifyJournalPassesThroughFailedResult() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        journal.result = VerificationResult.IntermediateCertsNotTrusted;
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.IntermediateCertsNotTrusted);
    }

    // ============ verify — Succinct SP1 happy path ============

    function testVerifySuccessfulJournalSP1() public {
        VerifierJournal memory result = _verifySP1Journal(_createSuccessJournal());

        _assertVerificationResult(result, VerificationResult.Success);
    }

    function testVerifyRevertsIfZkVerifierNotConfiguredSP1() public {
        ZkCoProcessorConfig memory config = _zkConfig(address(0));
        verifier.setZkConfiguration(ZkCoProcessorType.Succinct, config, VERIFIER_PROOF_ID);

        bytes memory proofBytes = _proofBytes();

        vm.prank(submitter);
        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkVerifierNotConfigured.selector, ZkCoProcessorType.Succinct)
        );
        verifier.verify("", ZkCoProcessorType.Succinct, proofBytes);
    }

    // ============ batchVerify Tests ============

    function testBatchVerifyRevertsIfNotProofSubmitter() public {
        vm.expectRevert(NitroEnclaveVerifier.CallerNotProofSubmitter.selector);
        verifier.batchVerify("", ZkCoProcessorType.RiscZero, "");
    }

    function testBatchVerifySuccess() public {
        _setUpRiscZeroConfig();

        VerifierJournal[] memory results = _batchVerifyRiscZero(_createBatchJournal(VERIFIER_PROOF_ID, 2));

        assertEq(results.length, 2);
        _assertVerificationResult(results[0], VerificationResult.Success);
        _assertVerificationResult(results[1], VerificationResult.Success);
    }

    function testBatchVerifyRevertsIfVerifierVkMismatch() public {
        _setUpRiscZeroConfig();

        bytes32 wrongVk = keccak256("wrong-vk");
        BatchVerifierJournal memory batchJournal = _createBatchJournal(wrongVk, 1);
        (bytes memory output, bytes memory proofBytes) = _mockRiscZeroBatchVerify(batchJournal);

        vm.prank(submitter);
        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.VerifierVkMismatch.selector, VERIFIER_PROOF_ID, wrongVk)
        );
        verifier.batchVerify(output, ZkCoProcessorType.RiscZero, proofBytes);
    }

    function testBatchVerifySuccessSP1() public {
        VerifierJournal[] memory results = _batchVerifySP1(_createBatchJournal(VERIFIER_PROOF_ID, 1));

        assertEq(results.length, 1);
        _assertVerificationResult(results[0], VerificationResult.Success);
    }

    // ============ Revoked Cert Invalidates Journal ============

    function testRevokedCertInvalidatesVerification() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();

        verifier.revokeCert(INTERMEDIATE_CERT_1);

        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.IntermediateCertsNotTrusted);
    }

    // ============ Expiry-Aware Caching Tests ============

    function testExpiredCachedCertFailsVerification() public {
        _setUpRiscZeroConfig();

        vm.warp(INTERMEDIATE_CERT_1_EXPIRY + 1);

        VerifierJournal memory journal = _createSuccessJournal();
        journal.timestamp = uint64(block.timestamp - 1) * 1000;
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.IntermediateCertsNotTrusted);
    }

    function testNonExpiredCachedCertPassesVerification() public {
        _setUpRiscZeroConfig();

        vm.warp(INTERMEDIATE_CERT_1_EXPIRY - 1);

        VerifierJournal memory journal = _createSuccessJournal();
        journal.timestamp = uint64(block.timestamp - 1) * 1000;
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.Success);
    }

    // Untrusted chain certs (past trustedCertsPrefixLen): expired journal notAfter => InvalidTimestamp
    function testVerifyJournalInvalidTimestampExpiredUntrustedCertInChain() public {
        _setUpRiscZeroConfig();

        bytes32 expiredLeaf = keccak256("expired-untrusted-leaf");

        VerifierJournal memory journal = _createSuccessJournalWithLeaf(expiredLeaf, uint64(block.timestamp - 1));
        VerifierJournal memory result = _verifyRiscZeroJournal(journal);

        _assertVerificationResult(result, VerificationResult.InvalidTimestamp);
        assertEq(verifier.trustedIntermediateCerts(expiredLeaf), 0);
    }

    function testCheckTrustedIntermediateCertsStopsAtExpiredCert() public {
        vm.warp(INTERMEDIATE_CERT_1_EXPIRY + 1);

        bytes32[][] memory reportCerts = new bytes32[][](1);
        reportCerts[0] = new bytes32[](2);
        reportCerts[0][0] = ROOT_CERT;
        reportCerts[0][1] = INTERMEDIATE_CERT_1;

        uint8[] memory results = verifier.checkTrustedIntermediateCerts(reportCerts);
        assertEq(results[0], 1); // only root counted, expired intermediate skipped
    }

    function testCacheNewCertStoresCorrectExpiry() public {
        _setUpRiscZeroConfig();

        bytes32 newCert = keccak256("brand-new-cert");
        uint64 newCertExpiry = uint64(REALISTIC_TIMESTAMP + 86_400); // 1 day from now

        VerifierJournal memory journal = _createSuccessJournalWithLeaf(newCert, newCertExpiry);
        _verifyRiscZeroJournal(journal);

        assertEq(verifier.trustedIntermediateCerts(newCert), newCertExpiry);
    }

    // ============ Helpers ============

    function _setUpRiscZeroConfig() internal {
        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, _zkConfig(mockRiscZeroVerifier), VERIFIER_PROOF_ID);
    }

    function _zkConfig(address zkVerifier) internal pure returns (ZkCoProcessorConfig memory) {
        return ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: zkVerifier });
    }

    function _expectNotOwnerRevert(address caller) internal {
        vm.prank(caller);
        vm.expectRevert(Ownable.Unauthorized.selector);
    }

    function _expectZkRouteFrozenRevert(ZkCoProcessorType zkType, bytes4 selector) internal {
        vm.expectRevert(abi.encodeWithSelector(NitroEnclaveVerifier.ZkRouteFrozen.selector, zkType, selector));
    }

    function _addAndFreezeVerifyRoute(ZkCoProcessorType zkType, bytes4 selector, address routeVerifier) internal {
        verifier.addVerifyRoute(zkType, selector, routeVerifier);
        verifier.freezeVerifyRoute(zkType, selector);
    }

    function _assertVerificationResult(VerifierJournal memory journal, VerificationResult expected) internal pure {
        assertEq(uint8(journal.result), uint8(expected));
    }

    function _verifyRiscZeroJournal(VerifierJournal memory journal) internal returns (VerifierJournal memory) {
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = _proofBytes();

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        return verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);
    }

    function _verifySP1Journal(VerifierJournal memory journal) internal returns (VerifierJournal memory) {
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = _proofBytes();

        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        return verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function _createBatchJournal(
        bytes32 verifierVk,
        uint256 outputCount
    )
        internal
        view
        returns (BatchVerifierJournal memory)
    {
        VerifierJournal[] memory outputs = new VerifierJournal[](outputCount);
        VerifierJournal memory journal = _createSuccessJournal();
        for (uint256 i; i < outputCount; ++i) {
            outputs[i] = journal;
        }

        return BatchVerifierJournal({ verifierVk: verifierVk, outputs: outputs });
    }

    function _batchVerifyRiscZero(BatchVerifierJournal memory batchJournal)
        internal
        returns (VerifierJournal[] memory)
    {
        (bytes memory output, bytes memory proofBytes) = _mockRiscZeroBatchVerify(batchJournal);

        vm.prank(submitter);
        return verifier.batchVerify(output, ZkCoProcessorType.RiscZero, proofBytes);
    }

    function _batchVerifySP1(BatchVerifierJournal memory batchJournal) internal returns (VerifierJournal[] memory) {
        bytes memory output = abi.encode(batchJournal);
        bytes memory proofBytes = _proofBytes();

        _mockSP1Verify(AGGREGATOR_ID, output, proofBytes);

        vm.prank(submitter);
        return verifier.batchVerify(output, ZkCoProcessorType.Succinct, proofBytes);
    }

    function _mockRiscZeroBatchVerify(BatchVerifierJournal memory batchJournal)
        internal
        returns (bytes memory output, bytes memory proofBytes)
    {
        output = abi.encode(batchJournal);
        proofBytes = _proofBytes();

        _mockRiscZeroVerify(AGGREGATOR_ID, output, proofBytes);
    }

    function _proofBytes() internal pure returns (bytes memory) {
        return abi.encodePacked(DEFAULT_PROOF_SELECTOR, bytes32(0));
    }

    function _createSuccessJournal() internal view returns (VerifierJournal memory) {
        bytes32[] memory certs = new bytes32[](2);
        certs[0] = ROOT_CERT;
        certs[1] = INTERMEDIATE_CERT_1;

        uint64[] memory expiries = new uint64[](2);
        expiries[0] = ROOT_CERT_EXPIRY;
        expiries[1] = INTERMEDIATE_CERT_1_EXPIRY;

        return _successJournal(certs, expiries);
    }

    function _createSuccessJournalWithLeaf(
        bytes32 leafCert,
        uint64 leafExpiry
    )
        internal
        view
        returns (VerifierJournal memory)
    {
        bytes32[] memory certs = new bytes32[](3);
        certs[0] = ROOT_CERT;
        certs[1] = INTERMEDIATE_CERT_1;
        certs[2] = leafCert;

        uint64[] memory expiries = new uint64[](3);
        expiries[0] = ROOT_CERT_EXPIRY;
        expiries[1] = INTERMEDIATE_CERT_1_EXPIRY;
        expiries[2] = leafExpiry;

        return _successJournal(certs, expiries);
    }

    function _successJournal(
        bytes32[] memory certs,
        uint64[] memory expiries
    )
        internal
        view
        returns (VerifierJournal memory)
    {
        Pcr[] memory pcrs = new Pcr[](0);

        return VerifierJournal({
            result: VerificationResult.Success,
            trustedCertsPrefixLen: 2,
            timestamp: uint64(block.timestamp - 1) * 1000,
            certs: certs,
            certExpiries: expiries,
            userData: "",
            nonce: "",
            publicKey: "",
            pcrs: pcrs,
            moduleId: "test-module"
        });
    }

    function _mockRiscZeroVerify(bytes32 programId, bytes memory output, bytes memory proofBytes) internal {
        vm.mockCall(
            mockRiscZeroVerifier,
            abi.encodeWithSelector(RISC_ZERO_VERIFY_SELECTOR, proofBytes, programId, sha256(output)),
            ""
        );
    }

    function _mockSP1Verify(bytes32 programId, bytes memory output, bytes memory proofBytes) internal {
        vm.mockCall(
            mockSP1Verifier, abi.encodeWithSelector(SP1_VERIFY_PROOF_SELECTOR, programId, output, proofBytes), ""
        );
    }
}

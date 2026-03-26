// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import { Test } from "forge-std/Test.sol";

import {
    ZkCoProcessorType,
    ZkCoProcessorConfig,
    VerifierJournal,
    BatchVerifierJournal,
    VerificationResult,
    Pcr
} from "interfaces/multiproof/tee/INitroEnclaveVerifier.sol";

import { NitroEnclaveVerifier } from "src/multiproof/tee/NitroEnclaveVerifier.sol";

contract NitroEnclaveVerifierTest is Test {
    NitroEnclaveVerifier public verifier;

    address public owner;
    address public submitter;
    address public mockRiscZeroVerifier;
    address public mockSP1Verifier;

    bytes32 public constant ROOT_CERT = keccak256("root-cert");
    bytes32 public constant INTERMEDIATE_CERT_1 = keccak256("intermediate-cert-1");
    bytes32 public constant INTERMEDIATE_CERT_2 = keccak256("intermediate-cert-2");
    bytes32 public constant VERIFIER_ID = keccak256("verifier-id");
    bytes32 public constant AGGREGATOR_ID = keccak256("aggregator-id");
    bytes32 public constant VERIFIER_PROOF_ID = keccak256("verifier-proof-id");

    uint64 public constant MAX_TIME_DIFF = 3600; // 1 hour

    // Realistic timestamp so timestamp validation tests work correctly
    uint256 internal constant REALISTIC_TIMESTAMP = 1_700_000_000;

    function setUp() public {
        vm.warp(REALISTIC_TIMESTAMP);

        owner = address(this);
        submitter = makeAddr("submitter");
        mockRiscZeroVerifier = makeAddr("mock-riscZero-verifier");
        mockSP1Verifier = makeAddr("mock-sp1-verifier");

        bytes32[] memory trustedCerts = new bytes32[](1);
        trustedCerts[0] = INTERMEDIATE_CERT_1;

        ZkCoProcessorConfig memory zkCfg =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockSP1Verifier });

        verifier = new NitroEnclaveVerifier(
            owner,
            MAX_TIME_DIFF,
            trustedCerts,
            ROOT_CERT,
            submitter,
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
        assertTrue(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1));
        assertFalse(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_2));
    }

    function testConstructorRevertsIfZeroMaxTimeDiff() public {
        bytes32[] memory certs = new bytes32[](0);
        ZkCoProcessorConfig memory zkCfg =
            ZkCoProcessorConfig({ verifierId: bytes32(0), aggregatorId: bytes32(0), zkVerifier: address(0) });
        vm.expectRevert(NitroEnclaveVerifier.ZeroMaxTimeDiff.selector);
        new NitroEnclaveVerifier(owner, 0, certs, bytes32(0), submitter, ZkCoProcessorType.Succinct, zkCfg, bytes32(0));
    }

    // ============ setRootCert Tests ============

    function testSetRootCert() public {
        bytes32 newRoot = keccak256("new-root");
        verifier.setRootCert(newRoot);
        assertEq(verifier.rootCert(), newRoot);
    }

    function testSetRootCertRevertsIfNotOwner() public {
        vm.prank(submitter);
        vm.expectRevert();
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
        vm.prank(submitter);
        vm.expectRevert();
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
        vm.prank(submitter);
        vm.expectRevert();
        verifier.setProofSubmitter(makeAddr("anyone"));
    }

    // ============ setZkConfiguration Tests ============

    function testSetZkConfiguration() public {
        ZkCoProcessorConfig memory config = ZkCoProcessorConfig({
            verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockRiscZeroVerifier
        });

        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);

        ZkCoProcessorConfig memory stored = verifier.getZkConfig(ZkCoProcessorType.RiscZero);
        assertEq(stored.verifierId, VERIFIER_ID);
        assertEq(stored.aggregatorId, AGGREGATOR_ID);
        assertEq(stored.zkVerifier, mockRiscZeroVerifier);

        assertEq(verifier.getVerifierProofId(ZkCoProcessorType.RiscZero), VERIFIER_PROOF_ID);
    }

    function testSetZkConfigurationRevertsIfNotOwner() public {
        ZkCoProcessorConfig memory config = ZkCoProcessorConfig({
            verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockRiscZeroVerifier
        });

        vm.prank(submitter);
        vm.expectRevert();
        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);
    }

    // ============ revokeCert Tests ============

    function testRevokeCert() public {
        assertTrue(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1));
        verifier.revokeCert(INTERMEDIATE_CERT_1);
        assertFalse(verifier.trustedIntermediateCerts(INTERMEDIATE_CERT_1));
    }

    function testRevokeCertRevertsIfNotTrusted() public {
        bytes32 unknown = keccak256("unknown-cert");
        vm.expectRevert(abi.encodeWithSelector(NitroEnclaveVerifier.CertificateNotFound.selector, unknown));
        verifier.revokeCert(unknown);
    }

    function testRevokeCertRevertsIfNotOwner() public {
        vm.prank(submitter);
        vm.expectRevert();
        verifier.revokeCert(INTERMEDIATE_CERT_1);
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
        vm.prank(submitter);
        vm.expectRevert();
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
        vm.prank(submitter);
        vm.expectRevert();
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, keccak256("new"));
    }

    // ============ addVerifyRoute / freezeVerifyRoute Tests ============

    function testAddVerifyRoute() public {
        bytes4 selector = bytes4(keccak256("test"));
        address routeVerifier = makeAddr("route-verifier");

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
        assertEq(verifier.getZkVerifier(ZkCoProcessorType.RiscZero, selector), routeVerifier);
    }

    function testAddVerifyRouteRevertsIfZeroAddress() public {
        vm.expectRevert(NitroEnclaveVerifier.ZeroVerifierAddress.selector);
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, bytes4(uint32(0x01)), address(0));
    }

    function testAddVerifyRouteRevertsIfFrozenSentinel() public {
        vm.expectRevert(NitroEnclaveVerifier.InvalidVerifierAddress.selector);
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, bytes4(uint32(0x01)), address(0xdead));
    }

    function testAddVerifyRouteRevertsIfNotOwner() public {
        vm.prank(submitter);
        vm.expectRevert();
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, bytes4(keccak256("test")), makeAddr("v"));
    }

    function testFreezeVerifyRoute() public {
        bytes4 selector = bytes4(keccak256("test"));
        address routeVerifier = makeAddr("route-verifier");

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkRouteFrozen.selector, ZkCoProcessorType.RiscZero, selector)
        );
        verifier.getZkVerifier(ZkCoProcessorType.RiscZero, selector);
    }

    function testAddVerifyRouteRevertsIfFrozen() public {
        bytes4 selector = bytes4(keccak256("test"));
        address routeVerifier = makeAddr("route-verifier");

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkRouteFrozen.selector, ZkCoProcessorType.RiscZero, selector)
        );
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
    }

    function testFreezeVerifyRouteRevertsIfAlreadyFrozen() public {
        bytes4 selector = bytes4(keccak256("test"));
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, makeAddr("v"));
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkRouteFrozen.selector, ZkCoProcessorType.RiscZero, selector)
        );
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);
    }

    function testFreezeVerifyRouteRevertsIfNotOwner() public {
        bytes4 selector = bytes4(keccak256("test"));
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, makeAddr("v"));

        vm.prank(submitter);
        vm.expectRevert();
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);
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
        reportCerts[0][2] = INTERMEDIATE_CERT_2; // not trusted

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
        // Set up config WITHOUT a zkVerifier address (zero)
        ZkCoProcessorConfig memory config =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: address(0) });
        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);

        VerifierJournal memory journal = _createSuccessJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        vm.prank(submitter);
        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkVerifierNotConfigured.selector, ZkCoProcessorType.RiscZero)
        );
        verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);
    }

    // ============ verify — Unknown_Zk_Coprocessor ============

    function testVerifyRevertsForUnknownCoprocessor() public {
        // Use ZkCoProcessorType.Unknown (0) — not RiscZero or Succinct
        ZkCoProcessorConfig memory config = ZkCoProcessorConfig({
            verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockRiscZeroVerifier
        });
        verifier.setZkConfiguration(ZkCoProcessorType.Unknown, config, VERIFIER_PROOF_ID);

        VerifierJournal memory journal = _createSuccessJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        vm.prank(submitter);
        vm.expectRevert(NitroEnclaveVerifier.Unknown_Zk_Coprocessor.selector);
        verifier.verify(output, ZkCoProcessorType.Unknown, proofBytes);
    }

    // ============ verify — ZkRouteFrozen during verify() ============

    function testVerifyRevertsIfRouteFrozen() public {
        _setUpRiscZeroConfig();

        bytes4 selector = bytes4(0); // matches the selector in our proofBytes
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, makeAddr("route-v"));
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        VerifierJournal memory journal = _createSuccessJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        vm.prank(submitter);
        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkRouteFrozen.selector, ZkCoProcessorType.RiscZero, selector)
        );
        verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);
    }

    // ============ verify — RiscZero happy path ============

    function testVerifySuccessfulJournal() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.Success));
    }

    function testVerifyJournalRootCertNotTrusted() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        journal.certs[0] = keccak256("wrong-root");
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.RootCertNotTrusted));
    }

    function testVerifyJournalRootCertNotTrustedZeroPrefixLen() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        journal.trustedCertsPrefixLen = 0;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.RootCertNotTrusted));
    }

    function testVerifyJournalIntermediateCertNotTrusted() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        // Replace trusted intermediate with untrusted one, but keep trustedCertsPrefixLen = 2
        journal.certs[1] = keccak256("untrusted-intermediate");
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.IntermediateCertsNotTrusted));
    }

    function testVerifyJournalInvalidTimestampTooOld() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        // Set timestamp far in the past — more than maxTimeDiff seconds ago (in ms)
        journal.timestamp = uint64(block.timestamp - MAX_TIME_DIFF - 1) * 1000;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.InvalidTimestamp));
    }

    function testVerifyJournalInvalidTimestampFuture() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        // Set timestamp in the future (converted to ms)
        journal.timestamp = uint64(block.timestamp + 100) * 1000;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.InvalidTimestamp));
    }

    function testVerifyCachesNewCerts() public {
        _setUpRiscZeroConfig();

        bytes32 newCert = keccak256("new-leaf-cert");
        assertFalse(verifier.trustedIntermediateCerts(newCert));

        VerifierJournal memory journal = _createSuccessJournal();
        // Add a new cert beyond the trusted prefix that will get cached
        bytes32[] memory certs = new bytes32[](3);
        certs[0] = ROOT_CERT;
        certs[1] = INTERMEDIATE_CERT_1;
        certs[2] = newCert;
        journal.certs = certs;
        journal.trustedCertsPrefixLen = 2; // only root + 1 intermediate are pre-trusted

        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertTrue(verifier.trustedIntermediateCerts(newCert));
    }

    function testVerifyJournalPassesThroughFailedResult() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        journal.result = VerificationResult.IntermediateCertsNotTrusted;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.IntermediateCertsNotTrusted));
    }

    // ============ verify — Succinct SP1 happy path ============

    function testVerifySuccessfulJournalSP1() public {
        _setUpSP1Config();

        VerifierJournal memory journal = _createSuccessJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockSP1Verify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.Succinct, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.Success));
    }

    function testVerifyRevertsIfNotProofSubmitterSP1() public {
        vm.expectRevert(NitroEnclaveVerifier.CallerNotProofSubmitter.selector);
        verifier.verify("", ZkCoProcessorType.Succinct, "");
    }

    function testVerifyRevertsIfZkVerifierNotConfiguredSP1() public {
        ZkCoProcessorConfig memory config =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: address(0) });
        verifier.setZkConfiguration(ZkCoProcessorType.Succinct, config, VERIFIER_PROOF_ID);

        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        vm.prank(submitter);
        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.ZkVerifierNotConfigured.selector, ZkCoProcessorType.Succinct)
        );
        verifier.verify(abi.encode(_createSuccessJournal()), ZkCoProcessorType.Succinct, proofBytes);
    }

    // ============ batchVerify Tests ============

    function testBatchVerifyRevertsIfNotProofSubmitter() public {
        vm.expectRevert(NitroEnclaveVerifier.CallerNotProofSubmitter.selector);
        verifier.batchVerify("", ZkCoProcessorType.RiscZero, "");
    }

    function testBatchVerifySuccess() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        VerifierJournal[] memory outputs = new VerifierJournal[](2);
        outputs[0] = journal;
        outputs[1] = journal;

        BatchVerifierJournal memory batchJournal =
            BatchVerifierJournal({ verifierVk: VERIFIER_PROOF_ID, outputs: outputs });

        bytes memory output = abi.encode(batchJournal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(AGGREGATOR_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal[] memory results = verifier.batchVerify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(results.length, 2);
        assertEq(uint8(results[0].result), uint8(VerificationResult.Success));
        assertEq(uint8(results[1].result), uint8(VerificationResult.Success));
    }

    function testBatchVerifyRevertsIfVerifierVkMismatch() public {
        _setUpRiscZeroConfig();

        bytes32 wrongVk = keccak256("wrong-vk");
        VerifierJournal[] memory outputs = new VerifierJournal[](1);
        outputs[0] = _createSuccessJournal();

        BatchVerifierJournal memory batchJournal = BatchVerifierJournal({ verifierVk: wrongVk, outputs: outputs });

        bytes memory output = abi.encode(batchJournal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(AGGREGATOR_ID, output, proofBytes);

        vm.prank(submitter);
        vm.expectRevert(
            abi.encodeWithSelector(NitroEnclaveVerifier.VerifierVkMismatch.selector, VERIFIER_PROOF_ID, wrongVk)
        );
        verifier.batchVerify(output, ZkCoProcessorType.RiscZero, proofBytes);
    }

    function testBatchVerifySuccessSP1() public {
        _setUpSP1Config();

        VerifierJournal memory journal = _createSuccessJournal();
        VerifierJournal[] memory outputs = new VerifierJournal[](1);
        outputs[0] = journal;

        BatchVerifierJournal memory batchJournal =
            BatchVerifierJournal({ verifierVk: VERIFIER_PROOF_ID, outputs: outputs });

        bytes memory output = abi.encode(batchJournal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockSP1Verify(AGGREGATOR_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal[] memory results = verifier.batchVerify(output, ZkCoProcessorType.Succinct, proofBytes);

        assertEq(results.length, 1);
        assertEq(uint8(results[0].result), uint8(VerificationResult.Success));
    }

    // ============ Revoked Cert Invalidates Journal ============

    function testRevokedCertInvalidatesVerification() public {
        _setUpRiscZeroConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        // Revoke the intermediate cert before verification
        verifier.revokeCert(INTERMEDIATE_CERT_1);

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.IntermediateCertsNotTrusted));
    }

    // ============ Helpers ============

    function _setUpRiscZeroConfig() internal {
        ZkCoProcessorConfig memory config = ZkCoProcessorConfig({
            verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockRiscZeroVerifier
        });
        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);
    }

    function _setUpSP1Config() internal {
        ZkCoProcessorConfig memory config =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockSP1Verifier });
        verifier.setZkConfiguration(ZkCoProcessorType.Succinct, config, VERIFIER_PROOF_ID);
    }

    function _createSuccessJournal() internal view returns (VerifierJournal memory) {
        bytes32[] memory certs = new bytes32[](2);
        certs[0] = ROOT_CERT;
        certs[1] = INTERMEDIATE_CERT_1;

        Pcr[] memory pcrs = new Pcr[](0);

        return VerifierJournal({
            result: VerificationResult.Success,
            trustedCertsPrefixLen: 2,
            timestamp: uint64(block.timestamp - 1) * 1000,
            certs: certs,
            userData: "",
            nonce: "",
            publicKey: "",
            pcrs: pcrs,
            moduleId: "test-module"
        });
    }

    function _mockRiscZeroVerify(bytes32 programId, bytes memory output, bytes memory proofBytes) internal {
        // IRiscZeroVerifier.verify(proofBytes, programId, sha256(output))
        vm.mockCall(
            mockRiscZeroVerifier,
            abi.encodeWithSelector(
                bytes4(keccak256("verify(bytes,bytes32,bytes32)")), proofBytes, programId, sha256(output)
            ),
            ""
        );
    }

    function _mockSP1Verify(bytes32 programId, bytes memory output, bytes memory proofBytes) internal {
        // ISP1Verifier.verifyProof(programVKey, publicValues, proofBytes)
        vm.mockCall(
            mockSP1Verifier,
            abi.encodeWithSelector(
                bytes4(keccak256("verifyProof(bytes32,bytes,bytes)")), programId, output, proofBytes
            ),
            ""
        );
    }
}

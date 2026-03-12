// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import { Test } from "forge-std/Test.sol";

import {
    INitroEnclaveVerifier,
    ZkCoProcessorType,
    ZkCoProcessorConfig,
    VerifierJournal,
    BatchVerifierJournal,
    VerificationResult,
    Pcr,
    Bytes48
} from "lib/aws-nitro-enclave-attestation/contracts/src/interfaces/INitroEnclaveVerifier.sol";

import { NitroEnclaveVerifier } from "src/multiproof/tee/NitroEnclaveVerifier.sol";

contract NitroEnclaveVerifierTest is Test {
    NitroEnclaveVerifier public verifier;

    address public owner;
    address public submitter;
    address public mockZkVerifier;

    bytes32 public constant ROOT_CERT = keccak256("root-cert");
    bytes32 public constant INTERMEDIATE_CERT_1 = keccak256("intermediate-cert-1");
    bytes32 public constant INTERMEDIATE_CERT_2 = keccak256("intermediate-cert-2");
    bytes32 public constant VERIFIER_ID = keccak256("verifier-id");
    bytes32 public constant AGGREGATOR_ID = keccak256("aggregator-id");
    bytes32 public constant VERIFIER_PROOF_ID = keccak256("verifier-proof-id");

    uint64 public constant MAX_TIME_DIFF = 3600; // 1 hour

    function setUp() public {
        owner = address(this);
        submitter = makeAddr("submitter");
        mockZkVerifier = makeAddr("mock-zk-verifier");

        bytes32[] memory trustedCerts = new bytes32[](1);
        trustedCerts[0] = INTERMEDIATE_CERT_1;

        verifier = new NitroEnclaveVerifier(owner, MAX_TIME_DIFF, trustedCerts);
        verifier.setRootCert(ROOT_CERT);
        verifier.setProofSubmitter(submitter);
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

    // ============ setProofSubmitter Tests ============

    function testSetProofSubmitter() public {
        address newSubmitter = makeAddr("new-submitter");
        verifier.setProofSubmitter(newSubmitter);
        assertEq(verifier.proofSubmitter(), newSubmitter);
    }

    function testSetProofSubmitterRevertsIfNotOwner() public {
        vm.prank(submitter);
        vm.expectRevert();
        verifier.setProofSubmitter(address(0));
    }

    // ============ setZkConfiguration Tests ============

    function testSetZkConfiguration() public {
        ZkCoProcessorConfig memory config =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockZkVerifier });

        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);

        ZkCoProcessorConfig memory stored = verifier.getZkConfig(ZkCoProcessorType.RiscZero);
        assertEq(stored.verifierId, VERIFIER_ID);
        assertEq(stored.aggregatorId, AGGREGATOR_ID);
        assertEq(stored.zkVerifier, mockZkVerifier);

        assertTrue(verifier.isVerifierIdSupported(ZkCoProcessorType.RiscZero, VERIFIER_ID));
        assertTrue(verifier.isAggregatorIdSupported(ZkCoProcessorType.RiscZero, AGGREGATOR_ID));
        assertEq(verifier.getVerifierProofId(ZkCoProcessorType.RiscZero, VERIFIER_ID), VERIFIER_PROOF_ID);
    }

    function testSetZkConfigurationRevertsIfNotOwner() public {
        ZkCoProcessorConfig memory config =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockZkVerifier });

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
        vm.expectRevert("Certificate not found in trusted certs");
        verifier.revokeCert(keccak256("unknown-cert"));
    }

    function testRevokeCertRevertsIfNotOwner() public {
        vm.prank(submitter);
        vm.expectRevert();
        verifier.revokeCert(INTERMEDIATE_CERT_1);
    }

    // ============ updateVerifierId Tests ============

    function testUpdateVerifierId() public {
        _setUpZkConfig();

        bytes32 newVerifierId = keccak256("new-verifier-id");
        bytes32 newVerifierProofId = keccak256("new-verifier-proof-id");
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, newVerifierId, newVerifierProofId);

        ZkCoProcessorConfig memory config = verifier.getZkConfig(ZkCoProcessorType.RiscZero);
        assertEq(config.verifierId, newVerifierId);
        assertTrue(verifier.isVerifierIdSupported(ZkCoProcessorType.RiscZero, newVerifierId));
        assertTrue(verifier.isVerifierIdSupported(ZkCoProcessorType.RiscZero, VERIFIER_ID));
        assertEq(verifier.getVerifierProofId(ZkCoProcessorType.RiscZero, newVerifierId), newVerifierProofId);
    }

    function testUpdateVerifierIdRevertsIfZero() public {
        _setUpZkConfig();
        vm.expectRevert("Verifier ID cannot be zero");
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, bytes32(0), bytes32(0));
    }

    function testUpdateVerifierIdRevertsIfSame() public {
        _setUpZkConfig();
        vm.expectRevert("Verifier ID is already the latest");
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, VERIFIER_ID, VERIFIER_PROOF_ID);
    }

    // ============ updateAggregatorId Tests ============

    function testUpdateAggregatorId() public {
        _setUpZkConfig();

        bytes32 newAggregatorId = keccak256("new-aggregator-id");
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, newAggregatorId);

        ZkCoProcessorConfig memory config = verifier.getZkConfig(ZkCoProcessorType.RiscZero);
        assertEq(config.aggregatorId, newAggregatorId);
        assertTrue(verifier.isAggregatorIdSupported(ZkCoProcessorType.RiscZero, newAggregatorId));
        assertTrue(verifier.isAggregatorIdSupported(ZkCoProcessorType.RiscZero, AGGREGATOR_ID));
    }

    function testUpdateAggregatorIdRevertsIfZero() public {
        _setUpZkConfig();
        vm.expectRevert("Aggregator ID cannot be zero");
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, bytes32(0));
    }

    function testUpdateAggregatorIdRevertsIfSame() public {
        _setUpZkConfig();
        vm.expectRevert("Aggregator ID is already the latest");
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, AGGREGATOR_ID);
    }

    // ============ removeVerifierId Tests ============

    function testRemoveVerifierId() public {
        _setUpZkConfig();

        bytes32 newId = keccak256("new-verifier-id");
        verifier.updateVerifierId(ZkCoProcessorType.RiscZero, newId, keccak256("proof"));

        verifier.removeVerifierId(ZkCoProcessorType.RiscZero, VERIFIER_ID);
        assertFalse(verifier.isVerifierIdSupported(ZkCoProcessorType.RiscZero, VERIFIER_ID));
        assertTrue(verifier.isVerifierIdSupported(ZkCoProcessorType.RiscZero, newId));
    }

    function testRemoveVerifierIdRevertsIfLatest() public {
        _setUpZkConfig();

        vm.expectRevert(
            abi.encodeWithSelector(
                INitroEnclaveVerifier.CannotRemoveLatestProgramId.selector, ZkCoProcessorType.RiscZero, VERIFIER_ID
            )
        );
        verifier.removeVerifierId(ZkCoProcessorType.RiscZero, VERIFIER_ID);
    }

    function testRemoveVerifierIdRevertsIfNotExists() public {
        _setUpZkConfig();
        vm.expectRevert("Verifier ID does not exist");
        verifier.removeVerifierId(ZkCoProcessorType.RiscZero, keccak256("nonexistent"));
    }

    // ============ removeAggregatorId Tests ============

    function testRemoveAggregatorId() public {
        _setUpZkConfig();

        bytes32 newId = keccak256("new-aggregator-id");
        verifier.updateAggregatorId(ZkCoProcessorType.RiscZero, newId);

        verifier.removeAggregatorId(ZkCoProcessorType.RiscZero, AGGREGATOR_ID);
        assertFalse(verifier.isAggregatorIdSupported(ZkCoProcessorType.RiscZero, AGGREGATOR_ID));
        assertTrue(verifier.isAggregatorIdSupported(ZkCoProcessorType.RiscZero, newId));
    }

    function testRemoveAggregatorIdRevertsIfLatest() public {
        _setUpZkConfig();

        vm.expectRevert(
            abi.encodeWithSelector(
                INitroEnclaveVerifier.CannotRemoveLatestProgramId.selector, ZkCoProcessorType.RiscZero, AGGREGATOR_ID
            )
        );
        verifier.removeAggregatorId(ZkCoProcessorType.RiscZero, AGGREGATOR_ID);
    }

    function testRemoveAggregatorIdRevertsIfNotExists() public {
        _setUpZkConfig();
        vm.expectRevert("Aggregator ID does not exist");
        verifier.removeAggregatorId(ZkCoProcessorType.RiscZero, keccak256("nonexistent"));
    }

    // ============ addVerifyRoute / freezeVerifyRoute Tests ============

    function testAddVerifyRoute() public {
        bytes4 selector = bytes4(keccak256("test"));
        address routeVerifier = makeAddr("route-verifier");

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
        assertEq(verifier.getZkVerifier(ZkCoProcessorType.RiscZero, selector), routeVerifier);
    }

    function testAddVerifyRouteRevertsIfZeroAddress() public {
        vm.expectRevert("Verifier cannot be zero address");
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, bytes4(uint32(0x01)), address(0));
    }

    function testFreezeVerifyRoute() public {
        bytes4 selector = bytes4(keccak256("test"));
        address routeVerifier = makeAddr("route-verifier");

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        vm.expectRevert(
            abi.encodeWithSelector(INitroEnclaveVerifier.ZkRouteFrozen.selector, ZkCoProcessorType.RiscZero, selector)
        );
        verifier.getZkVerifier(ZkCoProcessorType.RiscZero, selector);
    }

    function testAddVerifyRouteRevertsIfFrozen() public {
        bytes4 selector = bytes4(keccak256("test"));
        address routeVerifier = makeAddr("route-verifier");

        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        vm.expectRevert(
            abi.encodeWithSelector(INitroEnclaveVerifier.ZkRouteFrozen.selector, ZkCoProcessorType.RiscZero, selector)
        );
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, routeVerifier);
    }

    function testFreezeVerifyRouteRevertsIfAlreadyFrozen() public {
        bytes4 selector = bytes4(keccak256("test"));
        verifier.addVerifyRoute(ZkCoProcessorType.RiscZero, selector, makeAddr("v"));
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);

        vm.expectRevert(
            abi.encodeWithSelector(INitroEnclaveVerifier.ZkRouteFrozen.selector, ZkCoProcessorType.RiscZero, selector)
        );
        verifier.freezeVerifyRoute(ZkCoProcessorType.RiscZero, selector);
    }

    // ============ getZkVerifier Tests ============

    function testGetZkVerifierFallsBackToDefault() public {
        _setUpZkConfig();

        bytes4 unknownSelector = bytes4(0xdeadbeef);
        assertEq(verifier.getZkVerifier(ZkCoProcessorType.RiscZero, unknownSelector), mockZkVerifier);
    }

    function testGetZkVerifierReturnsRouteSpecific() public {
        _setUpZkConfig();

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
        bytes32[][] memory reportCerts = new bytes32[][](1);
        reportCerts[0] = new bytes32[](1);
        reportCerts[0][0] = keccak256("wrong-root");

        vm.expectRevert("First certificate must be the root certificate");
        verifier.checkTrustedIntermediateCerts(reportCerts);
    }

    // ============ verify Tests ============

    function testVerifyRevertsIfNotProofSubmitter() public {
        vm.expectRevert("Only the proof submitter can verify proofs");
        verifier.verify("", ZkCoProcessorType.RiscZero, "");
    }

    function testVerifySuccessfulJournal() public {
        _setUpZkConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.Success));
    }

    function testVerifyJournalRootCertNotTrusted() public {
        _setUpZkConfig();

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
        _setUpZkConfig();

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
        _setUpZkConfig();

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
        _setUpZkConfig();

        vm.warp(100_000);

        VerifierJournal memory journal = _createSuccessJournal();
        // Set timestamp far in the past — exceeds maxTimeDiff
        journal.timestamp = 1000; // 1 second in ms, way too old relative to block.timestamp=100000
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.InvalidTimestamp));
    }

    function testVerifyJournalInvalidTimestampFuture() public {
        _setUpZkConfig();

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
        _setUpZkConfig();

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
        _setUpZkConfig();

        VerifierJournal memory journal = _createSuccessJournal();
        journal.result = VerificationResult.IntermediateCertsNotTrusted;
        bytes memory output = abi.encode(journal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(VERIFIER_ID, output, proofBytes);

        vm.prank(submitter);
        VerifierJournal memory result = verifier.verify(output, ZkCoProcessorType.RiscZero, proofBytes);

        assertEq(uint8(result.result), uint8(VerificationResult.IntermediateCertsNotTrusted));
    }

    // ============ batchVerify Tests ============

    function testBatchVerifyRevertsIfNotProofSubmitter() public {
        vm.expectRevert("Only the proof submitter can verify proofs");
        verifier.batchVerify("", ZkCoProcessorType.RiscZero, "");
    }

    function testBatchVerifySuccess() public {
        _setUpZkConfig();

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
        _setUpZkConfig();

        VerifierJournal[] memory outputs = new VerifierJournal[](1);
        outputs[0] = _createSuccessJournal();

        BatchVerifierJournal memory batchJournal =
            BatchVerifierJournal({ verifierVk: keccak256("wrong-vk"), outputs: outputs });

        bytes memory output = abi.encode(batchJournal);
        bytes memory proofBytes = abi.encodePacked(bytes4(0), bytes32(0));

        _mockRiscZeroVerify(AGGREGATOR_ID, output, proofBytes);

        vm.prank(submitter);
        vm.expectRevert("Verifier VK does not match the expected verifier proof ID");
        verifier.batchVerify(output, ZkCoProcessorType.RiscZero, proofBytes);
    }

    // ============ verifyWithProgramId / batchVerifyWithProgramId Tests ============

    function testVerifyWithProgramIdReverts() public {
        vm.expectRevert("Not implemented");
        verifier.verifyWithProgramId("", ZkCoProcessorType.RiscZero, bytes32(0), "");
    }

    function testBatchVerifyWithProgramIdReverts() public {
        vm.expectRevert("Not implemented");
        verifier.batchVerifyWithProgramId("", ZkCoProcessorType.RiscZero, bytes32(0), bytes32(0), "");
    }

    // ============ Revoked Cert Invalidates Journal ============

    function testRevokedCertInvalidatesVerification() public {
        _setUpZkConfig();

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

    function _setUpZkConfig() internal {
        ZkCoProcessorConfig memory config =
            ZkCoProcessorConfig({ verifierId: VERIFIER_ID, aggregatorId: AGGREGATOR_ID, zkVerifier: mockZkVerifier });

        verifier.setZkConfiguration(ZkCoProcessorType.RiscZero, config, VERIFIER_PROOF_ID);
    }

    function _createSuccessJournal() internal view returns (VerifierJournal memory) {
        bytes32[] memory certs = new bytes32[](2);
        certs[0] = ROOT_CERT;
        certs[1] = INTERMEDIATE_CERT_1;

        Pcr[] memory pcrs = new Pcr[](0);

        return VerifierJournal({
            result: VerificationResult.Success,
            trustedCertsPrefixLen: 2,
            timestamp: uint64(block.timestamp) * 1000,
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
            mockZkVerifier,
            abi.encodeWithSelector(
                bytes4(keccak256("verify(bytes,bytes32,bytes32)")), proofBytes, programId, sha256(output)
            ),
            ""
        );
    }
}

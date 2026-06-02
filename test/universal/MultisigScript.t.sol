// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { Test } from "lib/forge-std/src/Test.sol";
import { Vm } from "lib/forge-std/src/Vm.sol";
import { Preinstalls } from "src/libraries/Preinstalls.sol";

import { MultisigScript } from "scripts/universal/MultisigScript.sol";
import { Simulation } from "scripts/universal/Simulation.sol";
import { IGnosisSafe, Enum } from "scripts/universal/IGnosisSafe.sol";

import { Counter } from "test/universal/Counter.sol";
import { LibString } from "lib/solady/src/utils/LibString.sol";

import { CBMulticall } from "src/universal/CBMulticall.sol";

contract MultisigScriptTest is Test, MultisigScript {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");
    Vm.Wallet internal wallet3 = vm.createWallet("3");

    address internal safe = address(1001);
    Counter internal counter = new Counter(safe);

    /// @dev Controls whether to use hash-based or EIP-712 JSON output. True by default.
    bool internal _useDataHashes = true;

    function setUp() public {
        vm.etch(safe, Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid));
        deployCodeTo("CBMulticall.sol", "", CB_MULTICALL);
        vm.deal(safe, 10 ether);

        // Multisig ownership tree:
        //
        //  ┌───────┐ ┌───────┐ ┌───────┐
        //  │wallet1│ │wallet2│ │wallet3│
        //  └───┬───┘ └───┬───┘ └───┬───┘
        //      └────────┼────────┘
        //          ┌────▽────┐
        //          │  safe   │ (threshold: 2/3)
        //          └────┬────┘
        //          ┌────▽────┐
        //          │ counter │
        //          └─────────┘

        address[] memory owners = new address[](3);
        owners[0] = wallet1.addr;
        owners[1] = wallet2.addr;
        owners[2] = wallet3.addr;
        IGnosisSafe(safe).setup(owners, 2, address(0), "", address(0), address(0), 0, address(0));
    }

    /// @inheritdoc MultisigScript
    ///
    /// @dev Verifies counter was incremented 6 times and received 3 ether
    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        uint256 counterValue = counter.count();
        assertEq(counterValue, 6, "Counter value is not 6");

        uint256 counterBalance = address(counter).balance;
        assertEq(counterBalance, 3 ether, "Counter balance is not 3 ether");
    }

    /// @inheritdoc MultisigScript
    ///
    /// @dev Builds a mix of calls to test different operation types:
    ///      - 1 regular increment call
    ///      - 1 delegatecall with 2 increments via multicall
    ///      - 1 payable increment call (1 ether)
    ///      - 1 delegatecall with 2 payable increments via multicall (2 ether)
    ///      Total: 6 increments, 3 ether sent
    function _buildCalls() internal view override returns (Call[] memory) {
        Call memory counterIncrementCall = Call({
            operation: Enum.Operation.Call,
            target: address(counter),
            data: abi.encodeCall(Counter.increment, ()),
            value: 0
        });

        Call memory counterIncrementCallPayable = Call({
            operation: Enum.Operation.Call,
            target: address(counter),
            data: abi.encodeCall(Counter.incrementPayable, ()),
            value: 1 ether
        });

        Call[] memory counterIncrementCalls = new Call[](2);
        counterIncrementCalls[0] = counterIncrementCall;
        counterIncrementCalls[1] = counterIncrementCall;

        Call[] memory counterIncrementCallsPayable = new Call[](2);
        counterIncrementCallsPayable[0] = counterIncrementCallPayable;
        counterIncrementCallsPayable[1] = counterIncrementCallPayable;

        Call[] memory calls = new Call[](4);

        calls[0] = counterIncrementCall;

        // Use multicall to test the delegatecall use case
        calls[1] = Call({
            operation: Enum.Operation.DelegateCall,
            target: CB_MULTICALL,
            data: abi.encodeCall(CBMulticall.aggregate3, (_toCall3s(counterIncrementCalls))),
            value: 0
        });

        calls[2] = counterIncrementCallPayable;

        calls[3] = Call({
            operation: Enum.Operation.DelegateCall,
            target: CB_MULTICALL,
            data: abi.encodeCall(CBMulticall.aggregate3Value, (_toCall3Values(counterIncrementCallsPayable))),
            value: 0
        });

        return calls;
    }

    /// @inheritdoc MultisigScript
    function _ownerSafe() internal view override returns (address) {
        return safe;
    }

    /// @inheritdoc MultisigScript
    function _printDataHashes() internal view override returns (bool) {
        return _useDataHashes;
    }

    /// @notice Helper to compute the expected transaction data for signing
    ///
    /// @return The encoded transaction data that signers need to sign
    function _expectedTxDataForCurrentBuildCalls() internal view returns (bytes memory) {
        return _encodeTransactionData(_ownerSafe(), _buildAggregatedScriptCall({ scriptCalls: _buildCalls() }));
    }

    function _signDigest(Vm.Wallet memory wallet, bytes32 digest) internal returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wallet, digest);
        return abi.encodePacked(r, s, v);
    }

    function _lastLoggedBytes() internal returns (bytes memory) {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        return abi.decode(logs[logs.length - 1].data, (bytes));
    }

    /// @notice Tests that sign() emits the correct data to sign
    function test_sign() external {
        vm.recordLogs();

        vm.prank(wallet1.addr);
        this.sign(new address[](0));

        bytes memory logged = _lastLoggedBytes();
        bytes memory expected = _expectedTxDataForCurrentBuildCalls();

        assertEq(logged, expected);
    }

    /// @notice Tests that verify() accepts valid 2-of-2 signatures
    function test_verify_valid_signatures() external {
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        bytes memory signatures = abi.encodePacked(_signDigest(wallet1, digest), _signDigest(wallet2, digest));
        verify(new address[](0), signatures);
    }

    /// @notice Tests that verify() reverts when given an invalid signature
    function test_verify_reverts_with_invalid_signature() external {
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        bytes memory signatures = abi.encodePacked(_signDigest(wallet1, digest), bytes32(0), bytes32(0), uint8(27));

        vm.expectRevert(); // nosemgrep: sol-safety-expectrevert-no-args
        this.verify(new address[](0), signatures);
    }

    /// @notice Tests that simulate() executes the transaction without broadcasting
    function test_simulate_only() external {
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        bytes memory signatures = abi.encodePacked(_signDigest(wallet1, digest), _signDigest(wallet2, digest));

        simulate(signatures);
    }

    /// @notice Tests that a Safe can execute with more signatures than the threshold requires
    ///
    /// @dev Safe is 2/3, but we provide all 3 signatures
    function test_run_with_more_signatures_than_threshold() external {
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        bytes memory signatures =
            abi.encodePacked(_signDigest(wallet1, digest), _signDigest(wallet2, digest), _signDigest(wallet3, digest));
        run(signatures);
    }

    /// @notice Tests that sign() emits EIP-712 JSON formatted data
    ///
    /// @dev Verifies the output contains expected EIP-712 structure fields
    function test_sign_eip712() external {
        _useDataHashes = false;

        vm.recordLogs();

        vm.prank(wallet1.addr);
        this.sign(new address[](0));

        bytes memory logged = _lastLoggedBytes();

        string memory loggedStr = string(logged);
        assertTrue(LibString.contains(loggedStr, "EIP712Domain"), "EIP-712 output should contain EIP712Domain");
        assertTrue(LibString.contains(loggedStr, "SafeTx"), "EIP-712 output should contain SafeTx type");
        assertTrue(LibString.contains(loggedStr, "primaryType"), "EIP-712 output should contain primaryType");
        assertTrue(LibString.contains(loggedStr, "domain"), "EIP-712 output should contain domain");
        assertTrue(LibString.contains(loggedStr, "message"), "EIP-712 output should contain message");
    }
}

abstract contract MultisigScriptNestedBase is Test, MultisigScript {
    Vm.Wallet internal nestedWallet1 = vm.createWallet("nested-1");
    Vm.Wallet internal nestedWallet2 = vm.createWallet("nested-2");

    address internal safe1 = address(1001);
    address internal safe2 = address(1002);
    address internal safe3 = address(1003);
    address internal safe4 = address(1004);

    Counter internal nestedCounter;

    function _setupSafe(address safeAddr, address[] memory owners, uint256 threshold) internal {
        IGnosisSafe(safeAddr).setup(owners, threshold, address(0), "", address(0), address(0), 0, address(0));
    }

    function _setupLeafSafes() internal {
        _setupSafe(safe1, _toArray(nestedWallet1.addr), 1);
        _setupSafe(safe2, _toArray(nestedWallet2.addr), 1);
    }

    /// @inheritdoc MultisigScript
    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        assertEq(nestedCounter.count(), 1, "Counter value is not 1");
    }

    /// @inheritdoc MultisigScript
    function _buildCalls() internal view override returns (Call[] memory) {
        Call[] memory calls = new Call[](1);
        calls[0] = Call({
            target: address(nestedCounter),
            operation: Enum.Operation.Call,
            data: abi.encodeCall(Counter.increment, ()),
            value: 0
        });

        return calls;
    }

    function _getSignerData(address signerSafe)
        internal
        view
        returns (address[] memory safes, bytes memory dataToSign)
    {
        safes = _signerSafes(signerSafe);

        Call[] memory callsChain = _buildCallsChain({ safes: _appendOwnerSafe(safes) });
        dataToSign = _encodeTransactionData({ safe: signerSafe, call: callsChain[0] });
    }

    function _assertSignOutput(address[] memory safes, bytes memory dataToSign) internal {
        this.sign(safes);

        assertEq(_lastLoggedBytes(), dataToSign);
    }

    function _signData(Vm.Wallet memory wallet, bytes memory dataToSign) internal returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wallet, keccak256(dataToSign));
        return abi.encodePacked(r, s, v);
    }

    function _approveFrom(address signerSafe, Vm.Wallet memory wallet) internal {
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(signerSafe);
        approve(safes, _signData(wallet, dataToSign));
    }

    function _lastLoggedBytes() internal returns (bytes memory) {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        return abi.decode(logs[logs.length - 1].data, (bytes));
    }

    function _signerSafes(address signerSafe) internal view virtual returns (address[] memory safes);
}

contract MultisigScriptNestedTest is MultisigScriptNestedBase {
    function setUp() public {
        bytes memory safeCode = Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid);
        deployCodeTo("CBMulticall.sol", "", CB_MULTICALL);
        vm.etch(safe1, safeCode);
        vm.etch(safe2, safeCode);
        vm.etch(safe3, safeCode);

        nestedCounter = new Counter(safe3);

        _setupLeafSafes();

        address[] memory owners3 = new address[](2);
        owners3[0] = safe1;
        owners3[1] = safe2;
        _setupSafe(safe3, owners3, 2);
    }

    /// @inheritdoc MultisigScript
    function _ownerSafe() internal view override returns (address) {
        return safe3;
    }

    function _signerSafes(address signerSafe) internal pure override returns (address[] memory safes) {
        safes = new address[](1);
        safes[0] = signerSafe;
    }

    /// @notice Tests that sign() emits the correct data to sign for safe1
    function test_sign_safe1() external {
        vm.recordLogs();
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(safe1);

        vm.prank(nestedWallet1.addr);
        _assertSignOutput(safes, dataToSign);
    }

    /// @notice Tests that sign() emits the correct data to sign for safe2
    function test_sign_safe2() external {
        vm.recordLogs();
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(safe2);

        vm.prank(nestedWallet2.addr);
        _assertSignOutput(safes, dataToSign);
    }

    /// @notice Tests that approve() succeeds with valid signature from safe1
    function test_approve_safe1() external {
        _approveFrom(safe1, nestedWallet1);
    }

    /// @notice Tests that approve() succeeds with valid signature from safe2
    function test_approve_safe2() external {
        _approveFrom(safe2, nestedWallet2);
    }

    /// @notice Tests that approve() fails when signature doesn't match the safe
    function test_approve_notOwner() external {
        (, bytes memory dataToSign) = _getSignerData(safe1);
        bytes memory signature = _signData(nestedWallet1, dataToSign);

        vm.expectRevert("not enough signatures");
        this.approve(_signerSafes(safe2), signature);
    }

    /// @notice Tests the full flow: approve from both safes, then run
    function test_run() external {
        _approveFrom(safe1, nestedWallet1);
        _approveFrom(safe2, nestedWallet2);

        run("");
    }

    /// @notice Tests that run() fails when not all nested safes have approved
    function test_run_notApproved() external {
        _approveFrom(safe1, nestedWallet1);

        vm.expectRevert("not enough signatures");
        this.run("");
    }
}

contract MultisigScriptDoubleNestedTest is MultisigScriptNestedBase {
    function setUp() public {
        bytes memory safeCode = Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid);
        deployCodeTo("CBMulticall.sol", "", CB_MULTICALL);
        vm.etch(safe1, safeCode);
        vm.etch(safe2, safeCode);
        vm.etch(safe3, safeCode);
        vm.etch(safe4, safeCode);

        nestedCounter = new Counter(safe4);

        _setupLeafSafes();

        address[] memory owners3 = new address[](2);
        owners3[0] = safe1;
        owners3[1] = safe2;
        _setupSafe(safe3, owners3, 2);

        _setupSafe(safe4, _toArray(safe3), 1);
    }

    /// @inheritdoc MultisigScript
    function _ownerSafe() internal view override returns (address) {
        return safe4;
    }

    function _signerSafes(address signerSafe) internal view override returns (address[] memory safes) {
        safes = new address[](2);
        safes[0] = signerSafe;
        safes[1] = safe3;
    }

    /// @notice Tests that sign() emits the correct data to sign for a double-nested safe
    function test_sign_double_nested() external {
        vm.recordLogs();
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(safe1);

        vm.prank(nestedWallet1.addr);
        _assertSignOutput(safes, dataToSign);
    }

    /// @notice Tests the intermediate approval flow through nested safes
    function test_approve_double_nested() external {
        _approveFrom(safe1, nestedWallet1);
        _approveFrom(safe2, nestedWallet2);

        approve(_toArray(safe3), "");
    }

    /// @notice Tests that intermediate approve fails when not all leaf safes have approved
    function test_runInit_double_nested_notApproved() external {
        _approveFrom(safe1, nestedWallet1);

        vm.expectRevert("not enough signatures");
        this.approve(_toArray(safe3), "");
    }

    /// @notice Tests the full flow: approve from all nested safes, then run
    function test_run_double_nested() external {
        _approveFrom(safe1, nestedWallet1);
        _approveFrom(safe2, nestedWallet2);
        approve(_toArray(safe3), "");

        run("");
    }
}

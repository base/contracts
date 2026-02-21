// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {Preinstalls} from "src/libraries/Preinstalls.sol";

import {MultisigScript} from "script/universal/MultisigScript.sol";
import {Simulation} from "script/universal/Simulation.sol";
import {IGnosisSafe, Enum} from "script/universal/IGnosisSafe.sol";

import {Counter} from "test/universal/Counter.sol";
import {LibString} from "lib/solady/src/utils/LibString.sol";

import {CBMulticall} from "src/utils/CBMulticall.sol";

contract MultisigScriptTest is Test, MultisigScript {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");
    Vm.Wallet internal wallet3 = vm.createWallet("3");

    address internal safe = address(1001);
    Counter internal counter = new Counter(address(safe));

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
        assertEq(counterBalance, 3 ether, "Counter balance is not 1 ether");
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

        calls[0] = Call({
            operation: Enum.Operation.Call,
            target: address(counter),
            data: abi.encodeCall(Counter.increment, ()),
            value: 0
        });

        // Use multicall to test the delegatecall use case
        calls[1] = Call({
            operation: Enum.Operation.DelegateCall,
            target: CB_MULTICALL,
            data: abi.encodeCall(CBMulticall.aggregate3, (_toCall3s(counterIncrementCalls))),
            value: 0
        });

        calls[2] = Call({
            operation: Enum.Operation.Call,
            target: address(counter),
            data: abi.encodeCall(Counter.incrementPayable, ()),
            value: 1 ether
        });

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
        return address(safe);
    }

    /// @inheritdoc MultisigScript
    ///
    /// @dev Returns `_useDataHashes` which is true by default (hash-based signing).
    function _printDataHashes() internal view override returns (bool) {
        return _useDataHashes;
    }

    /// @notice Helper to compute the expected transaction data for signing
    ///
    /// @return The encoded transaction data that signers need to sign
    function _expectedTxDataForCurrentBuildCalls() internal view returns (bytes memory) {
        return _encodeTransactionData(_ownerSafe(), _buildAggregatedScriptCall({scriptCalls: _buildCalls()}));
    }

    /// @notice Tests that sign() emits the correct data to sign
    function test_sign() external {
        vm.recordLogs();

        vm.prank(wallet1.addr);
        this.sign(new address[](0));

        Vm.Log[] memory logs = vm.getRecordedLogs();
        bytes memory logged = abi.decode(logs[logs.length - 1].data, (bytes));
        bytes memory expected = _expectedTxDataForCurrentBuildCalls();

        assertEq(keccak256(logged), keccak256(expected));
    }

    /// @notice Tests that verify() accepts valid 2-of-2 signatures
    function test_verify_valid_signatures() external {
        // Two-of-two signatures over the encoded transaction data should verify
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, digest);
        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2);
        verify(new address[](0), signatures);
    }

    /// @notice Tests that verify() reverts when given an invalid signature
    function test_verify_reverts_with_invalid_signature() external {
        // One valid, one invalid should revert
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        bytes memory signatures = abi.encodePacked(r1, s1, v1, bytes32(0), bytes32(0), uint8(27));
        bytes memory callData = abi.encodeCall(this.verify, (new address[](0), signatures));
        (bool success, bytes memory ret) = address(this).call(callData);
        assertFalse(success);
        assertTrue(ret.length > 0);
    }

    /// @notice Tests that simulate() executes the transaction without broadcasting
    function test_simulate_only() external {
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, digest);
        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2);

        // Simulate should execute successfully and satisfy _postCheck
        simulate(signatures);
    }

    /// @notice Tests that a Safe can execute with more signatures than the threshold requires
    ///
    /// @dev Safe is 2/3, but we provide all 3 signatures
    function test_run_with_more_signatures_than_threshold() external {
        // Sign with all 3 owners (threshold is 2, but we provide 3)
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, digest);
        (uint8 v3, bytes32 r3, bytes32 s3) = vm.sign(wallet3, digest);

        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2, r3, s3, v3);
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

        Vm.Log[] memory logs = vm.getRecordedLogs();
        bytes memory logged = abi.decode(logs[logs.length - 1].data, (bytes));

        // Verify the logged data contains EIP-712 JSON structure markers
        string memory loggedStr = string(logged);
        assertTrue(LibString.contains(loggedStr, "EIP712Domain"), "EIP-712 output should contain EIP712Domain");
        assertTrue(LibString.contains(loggedStr, "SafeTx"), "EIP-712 output should contain SafeTx type");
        assertTrue(LibString.contains(loggedStr, "primaryType"), "EIP-712 output should contain primaryType");
        assertTrue(LibString.contains(loggedStr, "domain"), "EIP-712 output should contain domain");
        assertTrue(LibString.contains(loggedStr, "message"), "EIP-712 output should contain message");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {Preinstalls} from "src/libraries/Preinstalls.sol";

import {MultisigScript} from "script/universal/MultisigScript.sol";
import {Simulation} from "script/universal/Simulation.sol";
import {IGnosisSafe, Enum} from "script/universal/IGnosisSafe.sol";
import {Counter} from "test/universal/Counter.sol";

contract MultisigScriptNestedTest is Test, MultisigScript {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");

    address internal safe1 = address(1001);
    address internal safe2 = address(1002);
    address internal safe3 = address(1003);
    Counter internal counter = new Counter(address(safe3));

    function setUp() public {
        bytes memory safeCode = Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid);
        deployCodeTo("CBMulticall.sol", "", CB_MULTICALL);
        vm.etch(safe1, safeCode);
        vm.etch(safe2, safeCode);
        vm.etch(safe3, safeCode);

        // Multisig ownership tree:
        //
        //      ┌───────┐ ┌───────┐
        //      │wallet1│ │wallet2│
        //      └───┬───┘ └───┬───┘
        //      ┌───▽───┐ ┌───▽───┐
        //      │ safe1 │ │ safe2 │  (threshold: 1/1 each)
        //      └───┬───┘ └───┬───┘
        //          └────┬────┘
        //          ┌────▽────┐
        //          │  safe3  │      (threshold: 2/2)
        //          └────┬────┘
        //          ┌────▽────┐
        //          │ counter │
        //          └─────────┘

        address[] memory owners1 = new address[](1);
        owners1[0] = wallet1.addr;
        IGnosisSafe(safe1).setup(owners1, 1, address(0), "", address(0), address(0), 0, address(0));

        address[] memory owners2 = new address[](1);
        owners2[0] = wallet2.addr;
        IGnosisSafe(safe2).setup(owners2, 1, address(0), "", address(0), address(0), 0, address(0));

        address[] memory owners3 = new address[](2);
        owners3[0] = safe1;
        owners3[1] = safe2;
        IGnosisSafe(safe3).setup(owners3, 2, address(0), "", address(0), address(0), 0, address(0));
    }

    /// @inheritdoc MultisigScript
    ///
    /// @dev Verifies counter was incremented once
    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        uint256 counterValue = counter.count();
        require(counterValue == 1, "Counter value is not 1");
    }

    /// @inheritdoc MultisigScript
    function _buildCalls() internal view override returns (Call[] memory) {
        Call[] memory calls = new Call[](1);

        calls[0] = Call({
            target: address(counter),
            operation: Enum.Operation.Call,
            data: abi.encodeCall(Counter.increment, ()),
            value: 0
        });

        return calls;
    }

    /// @inheritdoc MultisigScript
    function _ownerSafe() internal view override returns (address) {
        return address(safe3);
    }

    /// @notice Gets the safes array and data to sign for a given signer safe
    ///
    /// @param signerSafe The address of the signer's Safe (safe1 or safe2)
    ///
    /// @return safes The array of safes to pass to approve()
    /// @return dataToSign The data that needs to be signed
    function _getSignerData(address signerSafe)
        internal
        view
        returns (address[] memory safes, bytes memory dataToSign)
    {
        safes = new address[](1);
        safes[0] = signerSafe;

        Call[] memory callsChain = _buildCallsChain({safes: _appendOwnerSafe(safes)});
        dataToSign = _encodeTransactionData({safe: signerSafe, call: callsChain[0]});
    }

    /// @notice Tests that sign() emits the correct data to sign for safe1
    function test_sign_safe1() external {
        vm.recordLogs();
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(safe1);

        vm.prank(wallet1.addr);
        bytes memory txData = abi.encodeWithSelector(this.sign.selector, safes);
        (bool success,) = address(this).call(txData);
        assertTrue(success);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(keccak256(logs[logs.length - 1].data), keccak256(abi.encode(dataToSign)));
    }

    /// @notice Tests that sign() emits the correct data to sign for safe2
    function test_sign_safe2() external {
        vm.recordLogs();
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(safe2);

        vm.prank(wallet2.addr);
        bytes memory txData = abi.encodeWithSelector(this.sign.selector, safes);
        (bool success,) = address(this).call(txData);
        assertTrue(success);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(keccak256(logs[logs.length - 1].data), keccak256(abi.encode(dataToSign)));
    }

    /// @notice Tests that approve() succeeds with valid signature from safe1
    function test_approve_safe1() external {
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(safe1);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wallet1, keccak256(dataToSign));
        approve(safes, abi.encodePacked(r, s, v));
    }

    /// @notice Tests that approve() succeeds with valid signature from safe2
    function test_approve_safe2() external {
        (address[] memory safes, bytes memory dataToSign) = _getSignerData(safe2);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wallet2, keccak256(dataToSign));
        approve(safes, abi.encodePacked(r, s, v));
    }

    /// @notice Tests that approve() fails when signature doesn't match the safe
    function test_approve_notOwner() external {
        // Sign with wallet1 for safe1
        (, bytes memory dataToSign) = _getSignerData(safe1);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wallet1, keccak256(dataToSign));

        // But try to approve for safe2 (should fail)
        (address[] memory safes2,) = _getSignerData(safe2);

        bytes memory data = abi.encodeCall(this.approve, (safes2, abi.encodePacked(r, s, v)));
        (bool success, bytes memory result) = address(this).call(data);
        assertFalse(success);
        assertEq(result, abi.encodeWithSignature("Error(string)", "not enough signatures"));
    }

    /// @notice Tests the full flow: approve from both safes, then run
    function test_run() external {
        // Prepare and sign for wallet1/safe1
        (address[] memory safes1, bytes memory dataToSign1) = _getSignerData(safe1);
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, keccak256(dataToSign1));

        // Prepare and sign for wallet2/safe2
        (address[] memory safes2, bytes memory dataToSign2) = _getSignerData(safe2);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, keccak256(dataToSign2));

        // Approve for safe1 and safe2
        approve(safes1, abi.encodePacked(r1, s1, v1));
        approve(safes2, abi.encodePacked(r2, s2, v2));

        // Execute the final transaction
        run("");
    }

    /// @notice Tests that run() fails when not all nested safes have approved
    function test_run_notApproved() external {
        // Prepare and sign for wallet1/safe1 only
        (address[] memory safes1, bytes memory dataToSign) = _getSignerData(safe1);
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, keccak256(dataToSign));

        // Approve only for safe1
        approve(safes1, abi.encodePacked(r1, s1, v1));

        // Try to run without safe2 approval (should fail)
        bytes memory data = abi.encodeCall(this.run, (""));
        (bool success, bytes memory result) = address(this).call(data);
        assertFalse(success);
        assertEq(result, abi.encodeWithSignature("Error(string)", "not enough signatures"));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {Preinstalls} from "lib/optimism/packages/contracts-bedrock/src/libraries/Preinstalls.sol";

import {MultisigScript} from "script/universal/MultisigScript.sol";
import {Simulation} from "script/universal/Simulation.sol";
import {IGnosisSafe, Enum} from "script/universal/IGnosisSafe.sol";
import {Signatures} from "script/universal/Signatures.sol";

import {Counter} from "test/universal/Counter.sol";

contract MultisigScriptTest is Test, MultisigScript {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");
    Vm.Wallet internal wallet3 = vm.createWallet("3");

    address internal safe = address(1001);
    Counter internal counter = new Counter(address(safe));

    function() internal view returns (IMulticall3.Call3Value[] memory) buildCallsInternal;

    bytes internal dataToSign3of2 =
    // solhint-disable-next-line max-line-length
    hex"190132640243d7aade8c72f3d90d2dbf359e9897feba5fce1453bc8d9e7ba10d1715e6bf78f25eeee432952e1453c1b0d0bd867a1d4c4c859aa07ec7e2ef9cb87bc7";

    function setUp() public {
        vm.etch(safe, Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid));
        vm.etch(Preinstalls.MultiCall3, Preinstalls.getDeployedCode(Preinstalls.MultiCall3, block.chainid));
        vm.deal(safe, 10 ether);

        address[] memory owners = new address[](2);
        owners[0] = wallet1.addr;
        owners[1] = wallet2.addr;
        IGnosisSafe(safe).setup(owners, 2, address(0), "", address(0), address(0), 0, address(0));
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        uint256 counterValue = counter.count();
        require(counterValue == 1, "Counter value is not 1");
    }

    function _buildCalls() internal view override returns (IMulticall3.Call3Value[] memory) {
        return buildCallsInternal();
    }

    function _ownerSafe() internal view override returns (address) {
        return address(safe);
    }

    function _expectedTxDataForCurrentBuildCalls() internal view returns (bytes memory) {
        IMulticall3.Call3Value[] memory calls = _buildCalls();
        uint256 value;
        for (uint256 i; i < calls.length; i++) {
            value += calls[i].value;
        }

        // Non-nested case: single owner safe, last call is the aggregate call.
        bytes memory data = abi.encodeCall(IMulticall3.aggregate3Value, (calls));
        return _encodeTransactionData(_ownerSafe(), data, value);
    }

    function test_sign_no_value() external {
        buildCallsInternal = _buildCallsNoValue;

        vm.recordLogs();
        bytes memory txData = abi.encodeWithSelector(this.sign.selector, new address[](0));
        vm.prank(wallet1.addr);
        (bool success,) = address(this).call(txData);
        vm.assertTrue(success);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        bytes memory logged = abi.decode(logs[logs.length - 1].data, (bytes));
        bytes memory expected = _expectedTxDataForCurrentBuildCalls();
        assertEq(keccak256(logged), keccak256(expected));
    }

    function test_sign_with_value() external {
        buildCallsInternal = _buildCallsWithValue;

        vm.recordLogs();
        bytes memory txData = abi.encodeWithSelector(this.sign.selector, new address[](0));
        vm.prank(wallet1.addr);
        (bool success,) = address(this).call(txData);
        vm.assertTrue(success);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        bytes memory logged = abi.decode(logs[logs.length - 1].data, (bytes));
        bytes memory expected = _expectedTxDataForCurrentBuildCalls();
        assertEq(keccak256(logged), keccak256(expected));
    }

    function test_verify_valid_signatures() external {
        buildCallsInternal = _buildCallsNoValue;
        // Two-of-two signatures over the encoded transaction data should verify
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, digest);
        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2);
        verify(new address[](0), signatures);
    }

    function test_verify_reverts_with_invalid_signature() external {
        buildCallsInternal = _buildCallsNoValue;
        // One valid, one invalid should revert
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        bytes memory signatures = abi.encodePacked(r1, s1, v1, bytes32(0), bytes32(0), uint8(27));
        bytes memory callData = abi.encodeCall(this.verify, (new address[](0), signatures));
        (bool success, bytes memory ret) = address(this).call(callData);
        assertFalse(success);
        assertTrue(ret.length > 0);
    }

    function test_simulate_only() external {
        buildCallsInternal = _buildCallsNoValue;
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, digest);
        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2);

        // Simulate should execute successfully and satisfy _postCheck
        simulate(signatures);
    }

    function test_run_with_more_signatures_than_threshold() external {
        // Create a safe with 3 owners but threshold of 2
        address safe3of2 = address(1002);
        vm.etch(safe3of2, Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid));
        vm.deal(safe3of2, 10 ether);

        address[] memory owners = new address[](3);
        owners[0] = wallet1.addr;
        owners[1] = wallet2.addr;
        owners[2] = wallet3.addr;
        IGnosisSafe(safe3of2).setup(owners, 2, address(0), "", address(0), address(0), 0, address(0));

        Counter counter3of2 = new Counter(safe3of2);
        bytes32 hash = keccak256(dataToSign3of2);

        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, hash);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, hash);
        (uint8 v3, bytes32 r3, bytes32 s3) = vm.sign(wallet3, hash);

        bytes memory sigs = abi.encodePacked(r1, s1, v1, r2, s2, v2, r3, s3, v3);
        sigs = Signatures.prepareSignatures({safe: safe3of2, hash: hash, signatures: sigs});

        bool success = IGnosisSafe(safe3of2)
            .execTransaction({
                to: address(counter3of2),
                value: 0,
                data: abi.encodeCall(Counter.increment, ()),
                operation: Enum.Operation.Call,
                safeTxGas: 0,
                baseGas: 0,
                gasPrice: 0,
                gasToken: address(0),
                refundReceiver: payable(address(0)),
                signatures: sigs
            });

        assertTrue(success, "Should succeed with extra signatures");
        assertEq(counter3of2.count(), 1, "Counter should be incremented");
    }

    function _buildCallsNoValue() internal view returns (IMulticall3.Call3Value[] memory) {
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);

        calls[0] = IMulticall3.Call3Value({
            target: address(counter), allowFailure: false, callData: abi.encodeCall(Counter.increment, ()), value: 0
        });

        return calls;
    }

    function _buildCallsWithValue() internal view returns (IMulticall3.Call3Value[] memory) {
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);

        calls[0] = IMulticall3.Call3Value({
            target: address(counter),
            allowFailure: false,
            callData: abi.encodeCall(Counter.incrementPayable, ()),
            value: 1 ether
        });

        return calls;
    }
}

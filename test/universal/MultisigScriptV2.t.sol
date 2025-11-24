// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {Preinstalls} from "lib/optimism/packages/contracts-bedrock/src/libraries/Preinstalls.sol";

import {MultisigScriptV2} from "script/universal/MultisigScriptV2.sol";
import {Simulation} from "script/universal/Simulation.sol";
import {IGnosisSafe, Enum} from "script/universal/IGnosisSafe.sol";
import {Signatures} from "script/universal/Signatures.sol";

import {Counter} from "test/universal/Counter.sol";

import {CBMulticall} from "src/utils/CBMulticall.sol";

contract MultisigScriptV2Test is Test, MultisigScriptV2 {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");
    Vm.Wallet internal wallet3 = vm.createWallet("3");

    address internal safe = address(1001);
    Counter internal counter = new Counter(address(safe));

    bytes internal dataToSign3of2 =
    // solhint-disable-next-line max-line-length
    hex"190132640243d7aade8c72f3d90d2dbf359e9897feba5fce1453bc8d9e7ba10d1715e6bf78f25eeee432952e1453c1b0d0bd867a1d4c4c859aa07ec7e2ef9cb87bc7";

    function setUp() public {
        vm.etch(safe, Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid));
        deployCodeTo("CBMulticall.sol", "", CB_MULTICALL);
        vm.deal(safe, 10 ether);

        address[] memory owners = new address[](2);
        owners[0] = wallet1.addr;
        owners[1] = wallet2.addr;
        IGnosisSafe(safe).setup(owners, 2, address(0), "", address(0), address(0), 0, address(0));
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        uint256 counterValue = counter.count();
        assertEq(counterValue, 6, "Counter value is not 6");

        uint256 counterBalance = address(counter).balance;
        assertEq(counterBalance, 3 ether, "Counter balance is not 1 ether");
    }

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

    function _ownerSafe() internal view override returns (address) {
        return address(safe);
    }

    function _expectedTxDataForCurrentBuildCalls() internal view returns (bytes memory) {
        return _encodeTransactionData(_ownerSafe(), _buildAggregatedScriptCall({scriptCalls: _buildCalls()}));
    }

    function test_sign() external {
        vm.recordLogs();

        vm.prank(wallet1.addr);
        this.sign(new address[](0));

        Vm.Log[] memory logs = vm.getRecordedLogs();
        bytes memory logged = abi.decode(logs[logs.length - 1].data, (bytes));
        bytes memory expected = _expectedTxDataForCurrentBuildCalls();

        assertEq(keccak256(logged), keccak256(expected));
    }

    function test_verify_valid_signatures() external {
        // Two-of-two signatures over the encoded transaction data should verify
        bytes32 digest = keccak256(_expectedTxDataForCurrentBuildCalls());
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, digest);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, digest);
        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2);
        verify(new address[](0), signatures);
    }

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

    function test_simulate_only() external {
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
}

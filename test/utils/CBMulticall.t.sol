// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {CBMulticall} from "src/utils/CBMulticall.sol";
import {CommonTest} from "test/CommonTest.t.sol";
import {MockReceiver} from "test/mocks/MockReceiver.sol";

contract CBMulticallTest is CommonTest {
    CBMulticall mc;
    MockReceiver target;

    function setUp() public override {
        super.setUp();
        mc = new CBMulticall();
        target = new MockReceiver();
    }

    function test_aggregate_returnsBlockNumberAndData() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 41)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 1)
        });

        (uint256 bn, bytes[] memory rdata) = mc.aggregate(calls);
        assertEq(bn, block.number);
        assertEq(abi.decode(rdata[0], (uint256)), 42);
        assertEq(abi.decode(rdata[1], (uint256)), 2);
    }

    function test_aggregate_revertsOnFailedCall() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.aggregate(calls);
    }

    function test_tryAggregate_noRequire_returnsResults() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 1)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });

        CBMulticall.Result[] memory results = mc.tryAggregate(false, calls);
        assertEq(results.length, 2);
        assertTrue(results[0].success);
        assertEq(abi.decode(results[0].returnData, (uint256)), 2);
        assertFalse(results[1].success);
    }

    function test_tryAggregate_requireSuccess_revertsOnFailure() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.tryAggregate(true, calls);
    }

    function test_tryBlockAndAggregate_noRequire_returnsBlockInfoAndResults() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });

        (uint256 bn, bytes32 bh, CBMulticall.Result[] memory res) = mc.tryBlockAndAggregate(false, calls);
        assertEq(bn, block.number);
        assertEq(bh, blockhash(block.number));
        assertTrue(res[0].success);
        assertEq(abi.decode(res[0].returnData, (uint256)), 1);
        assertFalse(res[1].success);
    }

    function test_blockAndAggregate_allSuccess_returnsResults() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 1)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 2)
        });
        (uint256 bn, bytes32 bh, CBMulticall.Result[] memory res) = mc.blockAndAggregate(calls);
        assertEq(bn, block.number);
        assertEq(bh, blockhash(block.number));
        assertEq(res.length, 2);
        assertEq(abi.decode(res[1].returnData, (uint256)), 3);
    }

    function test_blockAndAggregate_revertsOnFailure() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.blockAndAggregate(calls);
    }

    function test_tryBlockAndAggregate_requireSuccess_revertsOnFailure() external {
        CBMulticall.Call[] memory calls = new CBMulticall.Call[](2);
        calls[0] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0)
        });
        calls[1] = CBMulticall.Call({
            target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.tryBlockAndAggregate(true, calls);
    }

    function test_aggregate3_success() external {
        CBMulticall.Call3[] memory calls3 = new CBMulticall.Call3[](1);
        calls3[0] = CBMulticall.Call3({
            target: address(target),
            allowFailure: false,
            callData: abi.encodeWithSelector(MockReceiver.bump.selector, 4)
        });
        CBMulticall.Result[] memory ret3 = mc.aggregate3(calls3);
        assertTrue(ret3[0].success);
        assertEq(abi.decode(ret3[0].returnData, (uint256)), 5);
    }

    function test_aggregate3_allowedFailure_returnsFalse() external {
        CBMulticall.Call3[] memory calls3 = new CBMulticall.Call3[](1);
        calls3[0] = CBMulticall.Call3({
            target: address(target),
            allowFailure: true,
            callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        CBMulticall.Result[] memory ret3 = mc.aggregate3(calls3);
        assertFalse(ret3[0].success);
    }

    function test_aggregate3_revertsOnNonAllowedFailure() external {
        CBMulticall.Call3[] memory calls3 = new CBMulticall.Call3[](1);
        calls3[0] = CBMulticall.Call3({
            target: address(target),
            allowFailure: false,
            callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.aggregate3(calls3);
    }

    function test_aggregate3Value_success_usesContractBalance() external {
        vm.deal(address(mc), 1 ether);
        CBMulticall.Call3Value[] memory callsV = new CBMulticall.Call3Value[](1);
        callsV[0] = CBMulticall.Call3Value({
            target: address(target),
            allowFailure: false,
            value: 0.5 ether,
            callData: abi.encodeWithSelector(MockReceiver.payAndEcho.selector, 7)
        });
        CBMulticall.Result[] memory retV = mc.aggregate3Value(callsV);
        (uint256 x, uint256 v) = abi.decode(retV[0].returnData, (uint256, uint256));
        assertEq(x, 7);
        assertEq(v, 0.5 ether);
        assertEq(address(target).balance, 0.5 ether);
    }

    function test_aggregate3Value_revertsOnNonAllowedFailure() external {
        CBMulticall.Call3Value[] memory callsV = new CBMulticall.Call3Value[](1);
        callsV[0] = CBMulticall.Call3Value({
            target: address(target),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.aggregate3Value(callsV);
    }

    function test_getBlockNumber() external view {
        assertEq(mc.getBlockNumber(), block.number);
    }

    function test_getBlockHash() external view {
        assertEq(mc.getBlockHash(block.number), blockhash(block.number));
    }

    function test_getCurrentBlockCoinbase() external view {
        assertEq(mc.getCurrentBlockCoinbase(), block.coinbase);
    }

    function test_getCurrentBlockDifficulty() external view {
        assertEq(mc.getCurrentBlockDifficulty(), block.difficulty);
    }

    function test_getCurrentBlockGasLimit() external view {
        assertEq(mc.getCurrentBlockGasLimit(), block.gaslimit);
    }

    function test_getCurrentBlockTimestamp() external view {
        assertEq(mc.getCurrentBlockTimestamp(), block.timestamp);
    }

    function test_getEthBalance() external view {
        assertEq(mc.getEthBalance(address(target)), address(target).balance);
    }

    function test_getLastBlockHash() external view {
        assertEq(mc.getLastBlockHash(), blockhash(block.number - 1));
    }

    function test_getBasefee() external view {
        assertEq(mc.getBasefee(), block.basefee);
    }

    function test_getChainId() external view {
        assertEq(mc.getChainId(), block.chainid);
    }
}

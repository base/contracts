// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { CBMulticall, Call, Call3, Call3Value, Result } from "src/utils/CBMulticall.sol";
import { CommonTest } from "test/CommonTest.t.sol";
import { MockReceiver } from "test/mocks/MockReceiver.sol";

/// @dev Helper contract used to invoke `aggregateDelegateCalls` via `delegatecall`.
///      This simulates the intended multisig usage pattern where the multicall
///      logic is executed in the context of another contract.
contract CBMulticallDelegateCaller {
    CBMulticall public mc;

    constructor(CBMulticall _mc) {
        mc = _mc;
    }

    function aggregateDelegateCalls(Call3[] calldata calls) external returns (Result[] memory) {
        (, bytes memory data) =
            address(mc).delegatecall(abi.encodeWithSelector(CBMulticall.aggregateDelegateCalls.selector, calls));
        return abi.decode(data, (Result[]));
    }
}

contract CBMulticallTest is CommonTest {
    CBMulticall mc;
    MockReceiver target;
    CBMulticallDelegateCaller delegateCaller;

    function setUp() public override {
        super.setUp();
        mc = new CBMulticall();
        target = new MockReceiver();
        delegateCaller = new CBMulticallDelegateCaller(mc);
    }

    function test_aggregate_returnsBlockNumberAndData() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 41) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 1) });

        (uint256 bn, bytes[] memory rdata) = mc.aggregate(calls);
        assertEq(bn, block.number);
        assertEq(abi.decode(rdata[0], (uint256)), 42);
        assertEq(abi.decode(rdata[1], (uint256)), 2);
    }

    function test_aggregate_revertsOnFailedCall() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector) });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.aggregate(calls);
    }

    function test_tryAggregate_noRequire_returnsResults() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 1) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector) });

        Result[] memory results = mc.tryAggregate(false, calls);
        assertEq(results.length, 2);
        assertTrue(results[0].success);
        assertEq(abi.decode(results[0].returnData, (uint256)), 2);
        assertFalse(results[1].success);
    }

    function test_tryAggregate_requireSuccess_revertsOnFailure() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector) });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.tryAggregate(true, calls);
    }

    function test_tryBlockAndAggregate_noRequire_returnsBlockInfoAndResults() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector) });

        (uint256 bn, bytes32 bh, Result[] memory res) = mc.tryBlockAndAggregate(false, calls);
        assertEq(bn, block.number);
        assertEq(bh, blockhash(block.number));
        assertTrue(res[0].success);
        assertEq(abi.decode(res[0].returnData, (uint256)), 1);
        assertFalse(res[1].success);
    }

    function test_blockAndAggregate_allSuccess_returnsResults() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 1) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 2) });
        (uint256 bn, bytes32 bh, Result[] memory res) = mc.blockAndAggregate(calls);
        assertEq(bn, block.number);
        assertEq(bh, blockhash(block.number));
        assertEq(res.length, 2);
        assertEq(abi.decode(res[1].returnData, (uint256)), 3);
    }

    function test_blockAndAggregate_revertsOnFailure() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector) });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.blockAndAggregate(calls);
    }

    function test_tryBlockAndAggregate_requireSuccess_revertsOnFailure() external {
        Call[] memory calls = new Call[](2);
        calls[0] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.bump.selector, 0) });
        calls[1] = Call({ target: address(target), callData: abi.encodeWithSelector(MockReceiver.willRevert.selector) });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.tryBlockAndAggregate(true, calls);
    }

    function test_aggregate3_success() external {
        Call3[] memory calls3 = new Call3[](1);
        calls3[0] = Call3({
            target: address(target),
            allowFailure: false,
            callData: abi.encodeWithSelector(MockReceiver.bump.selector, 4)
        });
        Result[] memory ret3 = mc.aggregate3(calls3);
        assertTrue(ret3[0].success);
        assertEq(abi.decode(ret3[0].returnData, (uint256)), 5);
    }

    function test_aggregate3_allowedFailure_returnsFalse() external {
        Call3[] memory calls3 = new Call3[](1);
        calls3[0] = Call3({
            target: address(target),
            allowFailure: true,
            callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        Result[] memory ret3 = mc.aggregate3(calls3);
        assertFalse(ret3[0].success);
    }

    function test_aggregate3_revertsOnNonAllowedFailure() external {
        Call3[] memory calls3 = new Call3[](1);
        calls3[0] = Call3({
            target: address(target),
            allowFailure: false,
            callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert(bytes("Multicall3: call failed"));
        mc.aggregate3(calls3);
    }

    function test_aggregateDelegateCalls_success() external {
        Call3[] memory calls3 = new Call3[](1);
        calls3[0] = Call3({
            target: address(target),
            allowFailure: false,
            callData: abi.encodeWithSelector(MockReceiver.bump.selector, 4)
        });
        Result[] memory ret3 = delegateCaller.aggregateDelegateCalls(calls3);
        assertTrue(ret3[0].success);
        assertEq(abi.decode(ret3[0].returnData, (uint256)), 5);
    }

    function test_aggregateDelegateCalls_allowedFailure_returnsFalse() external {
        Call3[] memory calls3 = new Call3[](1);
        calls3[0] = Call3({
            target: address(target),
            allowFailure: true,
            callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        Result[] memory ret3 = delegateCaller.aggregateDelegateCalls(calls3);
        assertFalse(ret3[0].success);
    }

    function test_aggregateDelegateCalls_revertsOnNonAllowedFailure() external {
        Call3[] memory calls3 = new Call3[](1);
        calls3[0] = Call3({
            target: address(target),
            allowFailure: false,
            callData: abi.encodeWithSelector(MockReceiver.willRevert.selector)
        });
        vm.expectRevert();
        delegateCaller.aggregateDelegateCalls(calls3);
    }

    function test_aggregateDelegateCalls_directCall_revertsWithMustDelegateCall() external {
        Call3[] memory calls3 = new Call3[](1);
        calls3[0] = Call3({
            target: address(target),
            allowFailure: false,
            callData: abi.encodeWithSelector(MockReceiver.bump.selector, 1)
        });

        vm.expectRevert(CBMulticall.MustDelegateCall.selector);
        mc.aggregateDelegateCalls(calls3);
    }

    function test_aggregate3Value_success_usesContractBalance() external {
        vm.deal(address(mc), 1 ether);
        Call3Value[] memory callsV = new Call3Value[](1);
        callsV[0] = Call3Value({
            target: address(target),
            allowFailure: false,
            value: 0.5 ether,
            callData: abi.encodeWithSelector(MockReceiver.payAndEcho.selector, 7)
        });
        Result[] memory retV = mc.aggregate3Value(callsV);
        (uint256 x, uint256 v) = abi.decode(retV[0].returnData, (uint256, uint256));
        assertEq(x, 7);
        assertEq(v, 0.5 ether);
        assertEq(address(target).balance, 0.5 ether);
    }

    function test_aggregate3Value_revertsOnNonAllowedFailure() external {
        Call3Value[] memory callsV = new Call3Value[](1);
        callsV[0] = Call3Value({
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

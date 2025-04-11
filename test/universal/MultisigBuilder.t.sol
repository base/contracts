// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {Preinstalls} from "@eth-optimism-bedrock/src/libraries/Preinstalls.sol";

import {MultisigBuilder} from "../../script/universal/MultisigBuilder.sol";
import {Simulation} from "../../script/universal/Simulation.sol";
import {IGnosisSafe} from "../../script/universal/IGnosisSafe.sol";
import {Counter} from "./Counter.sol";

contract MultisigBuilderTest is Test, MultisigBuilder {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");

    address internal safe = address(1001);
    Counter internal counter = new Counter(address(safe));

    bytes internal dataToSign =
    // solhint-disable-next-line max-line-length
        hex"1901d4bb33110137810c444c1d9617abe97df097d587ecde64e6fcb38d7f49e1280cd0722aa57d06d71497c199147817c38ae160e5b355d3fb5ccbe34c3dbadeae6d";

    bool allowEthTransfer;
    function() view returns (IMulticall3.Call3Value[] memory) buildCallsFn;

    function setUp() public {
        vm.etch(safe, Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid));
        vm.etch(Preinstalls.MultiCall3, Preinstalls.getDeployedCode(Preinstalls.MultiCall3, block.chainid));

        address[] memory owners = new address[](2);
        owners[0] = wallet1.addr;
        owners[1] = wallet2.addr;
        IGnosisSafe(safe).setup(owners, 2, address(0), "", address(0), address(0), 0, address(0));

        allowEthTransfer = false;
        buildCallsFn = _buildCallsNoValue;
    }

    function _allowEthTransfer() internal view override returns (bool) {
        return allowEthTransfer;
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        // Check that the counter has been incremented
        uint256 counterValue = counter.count();
        require(counterValue == 1, "Counter value is not 1");
    }

    function _buildCalls() internal view override returns (IMulticall3.Call3Value[] memory) {
        return buildCallsFn();
    }

    function _ownerSafe() internal view override returns (address) {
        return address(safe);
    }

    function test_sign() external {
        vm.recordLogs();
        sign();
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(keccak256(logs[logs.length - 1].data), keccak256(abi.encode(dataToSign)));
    }

    function test_run() external {
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, keccak256(dataToSign));
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, keccak256(dataToSign));
        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2);
        run(signatures);
    }

    function testRevert_buildCalls() external {
        allowEthTransfer = false;
        buildCallsFn = _buildCallsWithValue;
        _test_buildCalls("_buildCallsChecked: ETH transfer not allowed");

        allowEthTransfer = true;
        buildCallsFn = _buildCallsNoValue;
        _test_buildCalls("_buildCallsChecked: ETH transfer not necessary");
    }

    function _buildCallsWithValue() private view returns (IMulticall3.Call3Value[] memory) {
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);

        calls[0] = IMulticall3.Call3Value({
            target: address(counter),
            allowFailure: false,
            callData: abi.encodeCall(Counter.increment, ()),
            value: 1
        });

        return calls;
    }

    function _buildCallsNoValue() private view returns (IMulticall3.Call3Value[] memory) {
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);

        calls[0] = IMulticall3.Call3Value({
            target: address(counter),
            allowFailure: false,
            callData: abi.encodeCall(Counter.increment, ()),
            value: 0
        });

        return calls;
    }

    function _test_buildCalls(bytes memory revertData) internal {
        vm.expectRevert(revertData);
        this.sign();

        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(wallet1, keccak256(dataToSign));
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(wallet2, keccak256(dataToSign));
        bytes memory signatures = abi.encodePacked(r1, s1, v1, r2, s2, v2);
        vm.expectRevert(revertData);
        this.run(signatures);
    }
}

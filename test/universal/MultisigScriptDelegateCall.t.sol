// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {Preinstalls} from "lib/optimism/packages/contracts-bedrock/src/libraries/Preinstalls.sol";

import {MultisigScript} from "script/universal/MultisigScript.sol";
import {Simulation} from "script/universal/Simulation.sol";
import {IGnosisSafe, Enum} from "script/universal/IGnosisSafe.sol";

import {CBMulticall} from "src/utils/CBMulticall.sol";
import {Counter} from "test/universal/Counter.sol";

/// @dev Variant of `MultisigScript` that always uses delegatecall via `CBMulticall`.
///      Used to assert the delegatecall path wiring and encoded data.
contract MultisigScriptDelegateCallTest is Test, MultisigScript {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");

    address internal safe = address(1101);
    Counter internal counter = new Counter(address(safe));

    function() internal view returns (IMulticall3.Call3Value[] memory) buildCallsInternal;

    function setUp() public {
        // Deploy Safe and Multicall contracts.
        vm.etch(safe, Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid));

        // Ensure there is code at the standard Multicall3 preinstall for simulation paths
        // even though this test primarily exercises the CBMulticall delegatecall wiring.
        vm.etch(Preinstalls.MultiCall3, Preinstalls.getDeployedCode(Preinstalls.MultiCall3, block.chainid));

        // Deploy CBMulticall and map its code to the hard-coded CB_MULTICALL address
        CBMulticall mc = new CBMulticall();
        vm.etch(CB_MULTICALL, address(mc).code);

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

    /// @dev Force delegatecall mode for this test contract.
    function _useDelegateCall() internal view override returns (bool) {
        return true;
    }

    function test_delegatecall_mode_rejects_non_zero_call_value() external {
        buildCallsInternal = _buildCallsWithValue;

        // In delegatecall mode, per-call value is not supported and `_toCall3Array`
        // should enforce this invariant.
        vm.expectRevert(bytes("MultisigScript: delegatecall mode does not support call value"));
        bytes memory txData = abi.encodeWithSelector(this.sign.selector, new address[](0));
        vm.prank(wallet1.addr);
        // This should bubble up the revert from `_toCall3Array`.
        address(this).call(txData);
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

    /// @dev Expose `multicallAddress` for test assertions.
    function _getMulticallAddress() internal view returns (address) {
        return multicallAddress;
    }

    /// @dev Expose `CB_MULTICALL` for test assertions.
    function _getCbMulticallConstant() internal pure returns (address) {
        return CB_MULTICALL;
    }
}



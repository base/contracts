// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Vm } from "lib/forge-std/src/Vm.sol";
import { SystemConfig } from "src/L1/SystemConfig.sol";

import { Enum } from "../universal/IGnosisSafe.sol";
import { MultisigScript, Simulation } from "../universal/MultisigScript.sol";

/// @title SetGasLimit
///
/// @notice A script for updating the gas limit parameter on the Base L1 SystemConfig contract.
///
/// @dev This script assumes the SystemConfig contract is governed by a multisig and thus facilitates the collection of
///      signer approvals & execution. The gas limit parameter controls the maximum amount of gas that can be consumed
///      in a single L2 block.
contract SetGasLimit is MultisigScript {
    /// @notice Storage slot of `gasLimit` on `SystemConfig`.
    bytes32 internal constant GAS_LIMIT_SLOT = bytes32(uint256(0x68));

    address internal SYSTEM_CONFIG_OWNER = vm.envAddress("SYSTEM_CONFIG_OWNER");
    address internal L1_SYSTEM_CONFIG = vm.envAddress("L1_SYSTEM_CONFIG_ADDRESS");

    function _fromGasLimit() internal view returns (uint64) {
        return uint64(vm.envUint("FROM_GAS_LIMIT"));
    }

    function _toGasLimit() internal view returns (uint64) {
        return uint64(vm.envUint("TO_GAS_LIMIT"));
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        require(
            SystemConfig(L1_SYSTEM_CONFIG).gasLimit() == _toGasLimit(),
            "SetGasLimit::_postCheck: gas limit was not updated"
        );
    }

    function _buildCalls() internal view override returns (Call[] memory) {
        Call[] memory calls = new Call[](1);

        calls[0] = Call({
            operation: Enum.Operation.Call,
            target: L1_SYSTEM_CONFIG,
            data: abi.encodeCall(SystemConfig.setGasLimit, (_toGasLimit())),
            value: 0
        });

        return calls;
    }

    function _ownerSafe() internal view override returns (address) {
        return SYSTEM_CONFIG_OWNER;
    }

    // Pin the simulation's starting gas limit to FROM_GAS_LIMIT so the simulated transition
    // matches the intended FROM -> TO change even if production state has since drifted.
    function _simulationOverrides() internal view override returns (Simulation.StateOverride[] memory) {
        Simulation.StateOverride[] memory stateOverrides = new Simulation.StateOverride[](1);
        Simulation.StorageOverride[] memory storageOverrides = new Simulation.StorageOverride[](1);
        storageOverrides[0] =
            Simulation.StorageOverride({ key: GAS_LIMIT_SLOT, value: bytes32(uint256(_fromGasLimit())) });
        // solhint-disable-next-line max-line-length
        stateOverrides[0] = Simulation.StateOverride({ contractAddress: L1_SYSTEM_CONFIG, overrides: storageOverrides });
        return stateOverrides;
    }
}

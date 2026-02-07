// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {Vm} from "lib/forge-std/src/Vm.sol";
import {SystemConfig} from "lib/optimism/packages/contracts-bedrock/src/L1/SystemConfig.sol";

import {Enum} from "../../universal/IGnosisSafe.sol";
import {MultisigScript, Simulation} from "../../universal/MultisigScript.sol";

/// @title SetGasLimit
///
/// @notice A script for updating the gas limit parameter on the Base L1 SystemConfig contract.
///
/// @dev This script assumes the SystemConfig contract is governed by a multisig and thus facilitates the collection of
///      signer approvals & execution. The gas limit parameter controls the maximum amount of gas that can be consumed
///      in a single L2 block.
contract SetGasLimit is MultisigScript {
    address internal SYSTEM_CONFIG_OWNER = vm.envAddress("SYSTEM_CONFIG_OWNER");
    address internal L1_SYSTEM_CONFIG = vm.envAddress("L1_SYSTEM_CONFIG_ADDRESS");

    /**
     * -----------------------------------------------------------
     * Implemented Functions
     * -----------------------------------------------------------
     */
    function _fromGasLimit() internal view returns (uint64) {
        return uint64(vm.envUint("FROM_GAS_LIMIT"));
    }

    function _toGasLimit() internal view returns (uint64) {
        return uint64(vm.envUint("TO_GAS_LIMIT"));
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal view override {
        assert(SystemConfig(L1_SYSTEM_CONFIG).gasLimit() == _toGasLimit());
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

    // We need to expect that the gas limit will have been updated previously in our simulation
    // Use this override to specifically set the gas limit to the expected update value.
    function _simulationOverrides() internal view override returns (Simulation.StateOverride[] memory) {
        Simulation.StateOverride[] memory stateOverrides = new Simulation.StateOverride[](1);
        Simulation.StorageOverride[] memory storageOverrides = new Simulation.StorageOverride[](1);
        storageOverrides[0] = Simulation.StorageOverride({
            key: 0x0000000000000000000000000000000000000000000000000000000000000068, // slot of gas limit
            value: bytes32(uint256(_fromGasLimit()))
        });
        // solhint-disable-next-line max-line-length
        stateOverrides[0] = Simulation.StateOverride({contractAddress: L1_SYSTEM_CONFIG, overrides: storageOverrides});
        return stateOverrides;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { Test } from "lib/forge-std/src/Test.sol";
import { Initializable } from "lib/openzeppelin-contracts-v5/contracts/proxy/utils/Initializable.sol";
import { DeployUtils } from "scripts/libraries/DeployUtils.sol";
import { IFeeVault } from "interfaces/L2/IFeeVault.sol";
import { Types } from "src/libraries/Types.sol";

/// @title InitializerOZv5_Test
/// @dev Ensures that the `initialize()` function on contracts cannot be called more than
///      once. Tests the contracts inheriting from `Initializable` from OpenZeppelin Contracts v5.

contract InitializerOZv5_Test is Test {
    /// @notice The storage slot of the `initialized` flag in the `Initializable` contract from OZ v5.
    /// keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.Initializable")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant INITIALIZABLE_STORAGE = 0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;

    struct InitializableContract {
        address target;
        bytes initCalldata;
    }

    InitializableContract[] private contracts;

    function setUp() public {
        bytes memory constructorArgs = DeployUtils.encodeConstructor(abi.encodeCall(IFeeVault.__constructor__, ()));
        bytes memory initCalldata = abi.encodeCall(IFeeVault.initialize, (address(0), 0, Types.WithdrawalNetwork.L1));

        contracts.push(
            InitializableContract({
                target: address(DeployUtils.create1({ _name: "BaseFeeVault", _args: constructorArgs })),
                initCalldata: initCalldata
            })
        );

        contracts.push(
            InitializableContract({
                target: address(DeployUtils.create1({ _name: "OperatorFeeVault", _args: constructorArgs })),
                initCalldata: initCalldata
            })
        );

        contracts.push(
            InitializableContract({
                target: address(DeployUtils.create1({ _name: "SequencerFeeVault", _args: constructorArgs })),
                initCalldata: initCalldata
            })
        );

        contracts.push(
            InitializableContract({
                target: address(DeployUtils.create1({ _name: "L1FeeVault", _args: constructorArgs })),
                initCalldata: initCalldata
            })
        );
    }

    /// @notice Ensures OZ v5 initializers are disabled on deployed FeeVault contracts.
    function test_cannotReinitialize_succeeds() public {
        for (uint256 i; i < contracts.length; i++) {
            InitializableContract memory _contract = contracts[i];

            bytes32 slotVal = vm.load(_contract.target, INITIALIZABLE_STORAGE);
            uint64 initialized = uint64(uint256(slotVal));
            assertEq(initialized, type(uint64).max);

            (bool success, bytes memory returnData) = _contract.target.call(_contract.initCalldata);
            assertFalse(success);
            assertEq(bytes4(returnData), Initializable.InvalidInitialization.selector);
        }
    }
}

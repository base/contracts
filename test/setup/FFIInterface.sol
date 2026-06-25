// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Types } from "src/libraries/Types.sol";
import { Vm } from "lib/forge-std/src/Vm.sol";
import { Strings } from "lib/openzeppelin-contracts/contracts/utils/Strings.sol";

/// @title FFIInterface
/// @notice This contract is set into state using `etch` and therefore must not have constructor logic.
///         It also MUST be compiled with `0.8.15` because `vm.getDeployedCode` will break if there
///         are multiple artifacts for different compiler versions.
contract FFIInterface {
    Vm internal constant vm = Vm(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);
    string internal constant GO_FFI = "scripts/go-ffi/go-ffi";
    string internal constant DIFF_MODE = "diff";

    function getProveWithdrawalTransactionInputs(Types.WithdrawalTransaction memory _tx)
        external
        returns (bytes32, bytes32, bytes32, bytes32, bytes[] memory)
    {
        string[] memory cmds = _newDiffCommand("getProveWithdrawalTransactionInputs", 6);
        _setCrossDomainArgs(cmds, _tx.nonce, _tx.sender, _tx.target, _tx.value, _tx.gasLimit, _tx.data);

        return abi.decode(vm.ffi(cmds), (bytes32, bytes32, bytes32, bytes32, bytes[]));
    }

    function hashCrossDomainMessage(
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
        returns (bytes32)
    {
        string[] memory cmds = _newDiffCommand("hashCrossDomainMessage", 6);
        _setCrossDomainArgs(cmds, _nonce, _sender, _target, _value, _gasLimit, _data);

        return abi.decode(vm.ffi(cmds), (bytes32));
    }

    function hashWithdrawal(
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
        returns (bytes32)
    {
        string[] memory cmds = _newDiffCommand("hashWithdrawal", 6);
        _setCrossDomainArgs(cmds, _nonce, _sender, _target, _value, _gasLimit, _data);

        return abi.decode(vm.ffi(cmds), (bytes32));
    }

    function hashOutputRootProof(
        bytes32 _version,
        bytes32 _stateRoot,
        bytes32 _messagePasserStorageRoot,
        bytes32 _latestBlockhash
    )
        external
        returns (bytes32)
    {
        string[] memory cmds = _newDiffCommand("hashOutputRootProof", 4);
        cmds[3] = Strings.toHexString(uint256(_version));
        cmds[4] = Strings.toHexString(uint256(_stateRoot));
        cmds[5] = Strings.toHexString(uint256(_messagePasserStorageRoot));
        cmds[6] = Strings.toHexString(uint256(_latestBlockhash));

        return abi.decode(vm.ffi(cmds), (bytes32));
    }

    function hashDepositTransaction(
        address _from,
        address _to,
        uint256 _mint,
        uint256 _value,
        uint64 _gas,
        bytes memory _data,
        uint64 _logIndex
    )
        external
        returns (bytes32)
    {
        string[] memory cmds = _newDiffCommand("hashDepositTransaction", 8);
        cmds[3] = "0x0000000000000000000000000000000000000000000000000000000000000000";
        cmds[4] = vm.toString(_logIndex);
        cmds[5] = vm.toString(_from);
        cmds[6] = vm.toString(_to);
        cmds[7] = vm.toString(_mint);
        cmds[8] = vm.toString(_value);
        cmds[9] = vm.toString(_gas);
        cmds[10] = vm.toString(_data);

        return abi.decode(vm.ffi(cmds), (bytes32));
    }

    function encodeDepositTransaction(Types.UserDepositTransaction calldata txn) external returns (bytes memory) {
        string[] memory cmds = _newDiffCommand("encodeDepositTransaction", 9);
        cmds[3] = vm.toString(txn.from);
        cmds[4] = vm.toString(txn.to);
        cmds[5] = vm.toString(txn.value);
        cmds[6] = vm.toString(txn.mint);
        cmds[7] = vm.toString(txn.gasLimit);
        cmds[8] = vm.toString(txn.isCreation);
        cmds[9] = vm.toString(txn.data);
        cmds[10] = vm.toString(txn.l1BlockHash);
        cmds[11] = vm.toString(txn.logIndex);

        return abi.decode(vm.ffi(cmds), (bytes));
    }

    function encodeCrossDomainMessage(
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        external
        returns (bytes memory)
    {
        string[] memory cmds = _newDiffCommand("encodeCrossDomainMessage", 6);
        _setCrossDomainArgs(cmds, _nonce, _sender, _target, _value, _gasLimit, _data);

        return abi.decode(vm.ffi(cmds), (bytes));
    }

    function encodeSuperRootProof(Types.SuperRootProof calldata proof) external returns (bytes memory) {
        string[] memory cmds = _newDiffCommand("encodeSuperRootProof", 1);
        cmds[3] = vm.toString(abi.encode(proof));

        return abi.decode(vm.ffi(cmds), (bytes));
    }

    function hashSuperRootProof(Types.SuperRootProof calldata proof) external returns (bytes32) {
        string[] memory cmds = _newDiffCommand("hashSuperRootProof", 1);
        cmds[3] = vm.toString(abi.encode(proof));

        return abi.decode(vm.ffi(cmds), (bytes32));
    }

    function decodeVersionedNonce(uint256 nonce) external returns (uint256, uint256) {
        string[] memory cmds = _newDiffCommand("decodeVersionedNonce", 1);
        cmds[3] = vm.toString(nonce);

        return abi.decode(vm.ffi(cmds), (uint256, uint256));
    }

    function getMerkleTrieFuzzCase(string memory variant)
        external
        returns (bytes32, bytes memory, bytes memory, bytes[] memory)
    {
        string[] memory cmds = _newCommand("trie", variant, 0);

        return abi.decode(vm.ffi(cmds), (bytes32, bytes, bytes, bytes[]));
    }

    function encodeScalarEcotone(uint32 _basefeeScalar, uint32 _blobbasefeeScalar) external returns (bytes32) {
        string[] memory cmds = _newDiffCommand("encodeScalarEcotone", 2);
        cmds[3] = vm.toString(_basefeeScalar);
        cmds[4] = vm.toString(_blobbasefeeScalar);

        return abi.decode(vm.ffi(cmds), (bytes32));
    }

    function decodeScalarEcotone(bytes32 _scalar) external returns (uint32, uint32) {
        string[] memory cmds = _newDiffCommand("decodeScalarEcotone", 1);
        cmds[3] = vm.toString(_scalar);

        return abi.decode(vm.ffi(cmds), (uint32, uint32));
    }

    function _newDiffCommand(string memory _variant, uint256 _argCount) private pure returns (string[] memory cmds) {
        return _newCommand(DIFF_MODE, _variant, _argCount);
    }

    function _newCommand(
        string memory _mode,
        string memory _variant,
        uint256 _argCount
    )
        private
        pure
        returns (string[] memory cmds)
    {
        cmds = new string[](3 + _argCount);
        cmds[0] = GO_FFI;
        cmds[1] = _mode;
        cmds[2] = _variant;
    }

    function _setCrossDomainArgs(
        string[] memory _cmds,
        uint256 _nonce,
        address _sender,
        address _target,
        uint256 _value,
        uint256 _gasLimit,
        bytes memory _data
    )
        private
        pure
    {
        _cmds[3] = vm.toString(_nonce);
        _cmds[4] = vm.toString(_sender);
        _cmds[5] = vm.toString(_target);
        _cmds[6] = vm.toString(_value);
        _cmds[7] = vm.toString(_gasLimit);
        _cmds[8] = vm.toString(_data);
    }
}

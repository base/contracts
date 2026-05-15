// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Vm } from "lib/forge-std/src/Vm.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { CommonTest } from "test/setup/CommonTest.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { Constants } from "src/libraries/Constants.sol";
import { Encoding } from "src/libraries/Encoding.sol";
import { Hashing } from "src/libraries/Hashing.sol";
import { ForgeArtifacts } from "scripts/libraries/ForgeArtifacts.sol";

contract RelayActor {
    address internal constant IDENTITY_PRECOMPILE = address(0x04);

    bool public reverted;
    bool public unexpectedMessageStatus;

    IOptimismPortal2 internal immutable op;
    IL1CrossDomainMessenger internal immutable xdm;
    Vm internal immutable vm;
    bool internal immutable doFail;

    constructor(IOptimismPortal2 _op, IL1CrossDomainMessenger _xdm, Vm _vm, bool _doFail) {
        op = _op;
        xdm = _xdm;
        vm = _vm;
        doFail = _doFail;

        // Set op.l2Sender() once to the L2 Cross Domain Messenger. Nothing in the fuzzed
        // surface modifies this slot, so we don't need to re-write it on every relay.
        uint256 senderSlotIndex = ForgeArtifacts.getSlot("OptimismPortal2", "l2Sender").slot;
        vm.store(address(_op), bytes32(senderSlotIndex), bytes32(abi.encode(Predeploys.L2_CROSS_DOMAIN_MESSENGER)));
    }

    function _hashRelayMessage(
        uint256 _nonce,
        uint256 _value,
        uint256 _minGasLimit,
        bytes memory _message
    )
        internal
        pure
        returns (bytes32)
    {
        return Hashing.hashCrossDomainMessageV1({
            _nonce: _nonce,
            _sender: Predeploys.L2_CROSS_DOMAIN_MESSENGER,
            _target: IDENTITY_PRECOMPILE,
            _value: _value,
            _gasLimit: _minGasLimit,
            _data: _message
        });
    }

    /// @notice Relays a fuzzed message to the `L1CrossDomainMessenger`.
    function relay(uint8 _version, uint8 _value, bytes memory _message) external {
        // Vary value between 0 and 1 to exercise the with/without-value paths; the ID
        // precompile accepts value.
        _version = _version % 2;
        _value = _value % 2;

        // ID precompile gas cost: 15 + 3 * data_word_length.
        uint32 minGasLimit = uint32(15 + 3 * ((_message.length + 31) / 32));

        // For the failure case, we use an impossibly large minGasLimit so that the hasMinGas
        // check always fails regardless of available gas. We provide baseGas-level gas (enough
        // for relayMessage's overhead) to avoid OOG reverts. Limiting gas directly is fragile
        // because the proxy-to-proxy call overhead (SystemConfig → SuperchainConfig,
        // OptimismPortal) leaves a razor-thin window between "enough to not OOG" and
        // "not enough for hasMinGas to pass".
        uint32 relayMinGasLimit = doFail ? type(uint32).max : minGasLimit;
        uint256 gas = xdm.baseGas(_message, minGasLimit);

        // `relayMessage` always re-encodes as a v1 hash after checking the v0 hash hasn't been
        // relayed, so the v1 hash is what we track.
        uint256 nonce = Encoding.encodeVersionedNonce({ _nonce: 0, _version: _version });

        bytes32 relayMessageHash = _hashRelayMessage(nonce, _value, relayMinGasLimit, _message);
        vm.assume(!xdm.successfulMessages(relayMessageHash) && !xdm.failedMessages(relayMessageHash));

        vm.startPrank(address(op));
        if (!doFail) {
            vm.expectCallMinGas(IDENTITY_PRECOMPILE, _value, minGasLimit, _message);
        }
        try xdm.relayMessage{ gas: gas, value: _value }(
            nonce, Predeploys.L2_CROSS_DOMAIN_MESSENGER, IDENTITY_PRECOMPILE, _value, relayMinGasLimit, _message
        ) { }
        catch {
            // Forge's invariant fuzzer ignores reverted target calls, so we surface the failure
            // by flipping a flag the invariant asserts on.
            reverted = true;
        }
        vm.stopPrank();

        unexpectedMessageStatus = unexpectedMessageStatus || xdm.successfulMessages(relayMessageHash) == doFail
            || xdm.failedMessages(relayMessageHash) != doFail;
    }
}

contract XDM_MinGasLimits is CommonTest {
    RelayActor actor;

    function init(bool doFail) internal {
        super.setUp();

        actor = new RelayActor(optimismPortal2, l1CrossDomainMessenger, vm, doFail);

        // Give the portal some ether to send to `relayMessage`.
        vm.deal(address(optimismPortal2), type(uint128).max);

        targetContract(address(actor));

        excludeSender(Constants.ESTIMATION_ADDRESS);

        // Don't allow the predeploys to be the senders.
        uint160 prefix = uint160(0x420) << 148;
        for (uint256 i = 0; i < Predeploys.PREDEPLOY_COUNT; i++) {
            excludeContract(address(prefix | uint160(i)));
        }

        bytes4[] memory selectors = new bytes4[](1);
        selectors[0] = actor.relay.selector;
        targetSelector(FuzzSelector({ addr: address(actor), selectors: selectors }));
    }

    /// @dev The actor records any relay that reverts or lands in the wrong message-status mapping.
    function _assertRelayResults() internal view {
        assertFalse(actor.unexpectedMessageStatus());
        assertFalse(actor.reverted());
    }
}

contract XDM_MinGasLimits_Succeeds is XDM_MinGasLimits {
    function setUp() public override {
        super.init(false);
    }

    /// @custom:invariant `relayMessage` should succeed when the outer call has base gas and the
    ///                   target can receive the inner minimum gas limit.
    function invariant_relayMessage_forwardsMinGas_succeeds() external view {
        _assertRelayResults();
    }
}

contract XDM_MinGasLimits_Reverts is XDM_MinGasLimits {
    function setUp() public override {
        super.init(true);
    }

    /// @custom:invariant `relayMessage` should mark the message failed when the inner minimum gas
    ///                   limit is too large to forward to the target.
    function invariant_relayMessage_insufficientMinGas_fails() external view {
        _assertRelayResults();
    }
}

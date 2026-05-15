// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { StdUtils } from "lib/forge-std/src/StdUtils.sol";
import { Vm } from "lib/forge-std/src/Vm.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { CommonTest } from "test/setup/CommonTest.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { Constants } from "src/libraries/Constants.sol";
import { Encoding } from "src/libraries/Encoding.sol";
import { Hashing } from "src/libraries/Hashing.sol";
import { ForgeArtifacts } from "scripts/libraries/ForgeArtifacts.sol";

contract RelayActor is StdUtils {
    address internal constant IDENTITY_PRECOMPILE = address(0x04);

    bytes32[] public hashes;
    bool public reverted;

    IOptimismPortal2 op;
    IL1CrossDomainMessenger xdm;
    Vm vm;
    bool doFail;

    constructor(IOptimismPortal2 _op, IL1CrossDomainMessenger _xdm, Vm _vm, bool _doFail) {
        op = _op;
        xdm = _xdm;
        vm = _vm;
        doFail = _doFail;

        // Set op.l2Sender() once to the L2 Cross Domain Messenger. Nothing in the fuzzed
        // surface modifies this slot, so we don't need to re-write it on every relay.
        uint256 senderSlotIndex = ForgeArtifacts.getSlot("OptimismPortal2", "l2Sender").slot;
        vm.store(
            address(_op), bytes32(senderSlotIndex), bytes32(uint256(uint160(Predeploys.L2_CROSS_DOMAIN_MESSENGER)))
        );
    }

    function hashesLength() external view returns (uint256) {
        return hashes.length;
    }

    /// @notice Relays a message to the `L1CrossDomainMessenger` with a random `version`,
    ///         and `_message`.
    function relay(uint8 _version, uint8 _value, bytes memory _message) external {
        // Restrict version to [0, 1] and value to [0, 1] (variance with/without value;
        // the ID precompile accepts value).
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
        uint256 nonce = Encoding.encodeVersionedNonce(0, _version);
        address sender = Predeploys.L2_CROSS_DOMAIN_MESSENGER;
        bytes32 _hash =
            Hashing.hashCrossDomainMessageV1(nonce, sender, IDENTITY_PRECOMPILE, _value, relayMinGasLimit, _message);
        hashes.push(_hash);

        // Make sure we've got a fresh message.
        vm.assume(xdm.successfulMessages(_hash) == false && xdm.failedMessages(_hash) == false);

        vm.startPrank(address(op));
        if (!doFail) {
            vm.expectCallMinGas(IDENTITY_PRECOMPILE, _value, minGasLimit, _message);
        }
        try xdm.relayMessage{ gas: gas, value: _value }(
            nonce, sender, IDENTITY_PRECOMPILE, _value, relayMinGasLimit, _message
        ) { }
        catch {
            // Forge's invariant fuzzer ignores reverted target calls, so we surface the failure
            // by flipping a flag the invariant asserts on.
            reverted = true;
        }
        vm.stopPrank();
    }
}

contract XDM_MinGasLimits is CommonTest {
    RelayActor actor;

    function init(bool doFail) public virtual {
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

    /// @dev Asserts every relayed hash landed in the expected mapping. `expectSuccess = true`
    ///      checks `successfulMessages`; `false` checks `failedMessages`.
    function _assertHashes(bool expectSuccess) internal view {
        uint256 length = actor.hashesLength();
        for (uint256 i = 0; i < length; ++i) {
            bytes32 hash = actor.hashes(i);
            assertEq(l1CrossDomainMessenger.successfulMessages(hash), expectSuccess);
            assertEq(l1CrossDomainMessenger.failedMessages(hash), !expectSuccess);
        }
        assertFalse(actor.reverted());
    }
}

contract XDM_MinGasLimits_Succeeds is XDM_MinGasLimits {
    function setUp() public override {
        super.init(false);
    }

    /// @custom:invariant A call to `relayMessage` should succeed if at least the minimum gas limit
    ///                   can be supplied to the target context, there is enough gas to complete
    ///                   execution of `relayMessage` after the target context's execution is
    ///                   finished, and the target context did not revert.
    ///
    ///                   There are two minimum gas limits here:
    ///
    ///                   - The outer min gas limit is for the call from the `OptimismPortal` to the
    ///                     `L1CrossDomainMessenger`,  and it can be retrieved by calling the xdm's
    ///                     `baseGas` function with the `message` and inner limit.
    ///
    ///                   - The inner min gas limit is for the call from the
    ///                     `L1CrossDomainMessenger` to the target contract.
    function invariant_minGasLimits() external view {
        _assertHashes(true);
    }
}

contract XDM_MinGasLimits_Reverts is XDM_MinGasLimits {
    function setUp() public override {
        super.init(true);
    }

    /// @custom:invariant A call to `relayMessage` should assign the message hash to the
    ///                   `failedMessages` mapping if not enough gas is supplied to forward
    ///                   `minGasLimit` to the target context or if there is not enough gas to
    ///                   complete execution of `relayMessage` after the target context's execution
    ///                   is finished.
    ///
    ///                   There are two minimum gas limits here:
    ///
    ///                   - The outer min gas limit is for the call from the `OptimismPortal` to the
    ///                     `L1CrossDomainMessenger`,  and it can be retrieved by calling the xdm's
    ///                     `baseGas` function with the `message` and inner limit.
    ///
    ///                   - The inner min gas limit is for the call from the
    ///                     `L1CrossDomainMessenger` to the target contract.
    function invariant_minGasLimits() external view {
        _assertHashes(false);
    }
}

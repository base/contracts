// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Vm } from "lib/forge-std/src/Vm.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { CommonTest } from "test/setup/CommonTest.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { Encoding } from "src/libraries/Encoding.sol";
import { Hashing } from "src/libraries/Hashing.sol";
import { ForgeArtifacts, StorageSlot } from "scripts/libraries/ForgeArtifacts.sol";

contract RelayActor {
    address internal constant IDENTITY_PRECOMPILE = address(0x04);
    uint256 internal constant MAX_MESSAGE_SIZE = 1000;

    // Invariant handlers ignore target-call reverts, so failures must persist after relay returns.
    bool public badRelayResult;

    address internal immutable op;
    IL1CrossDomainMessenger internal immutable xdm;
    Vm internal immutable vm;
    bool internal immutable shouldFail;

    constructor(address _op, IL1CrossDomainMessenger _xdm, Vm _vm, bool _shouldFail) {
        op = _op;
        xdm = _xdm;
        vm = _vm;
        shouldFail = _shouldFail;
    }

    function relay(uint8 _version, uint8 _value, bytes memory _message) external {
        vm.assume(_message.length <= MAX_MESSAGE_SIZE);

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
        uint32 relayMinGasLimit = shouldFail ? type(uint32).max : minGasLimit;

        // `relayMessage` always re-encodes as a v1 hash after checking the v0 hash hasn't been
        // relayed, so the v1 hash is what we track.
        uint256 nonce = Encoding.encodeVersionedNonce({ _nonce: 0, _version: _version });

        bytes32 relayMessageHash = Hashing.hashCrossDomainMessageV1({
            _nonce: nonce,
            _sender: Predeploys.L2_CROSS_DOMAIN_MESSENGER,
            _target: IDENTITY_PRECOMPILE,
            _value: _value,
            _gasLimit: relayMinGasLimit,
            _data: _message
        });
        vm.assume(!xdm.successfulMessages(relayMessageHash) && !xdm.failedMessages(relayMessageHash));

        uint256 gas = xdm.baseGas(_message, minGasLimit);

        if (!shouldFail) {
            vm.expectCallMinGas(IDENTITY_PRECOMPILE, _value, minGasLimit, _message);
        }
        vm.prank(op);
        try xdm.relayMessage{ gas: gas, value: _value }(
            nonce, Predeploys.L2_CROSS_DOMAIN_MESSENGER, IDENTITY_PRECOMPILE, _value, relayMinGasLimit, _message
        ) { }
        catch {
            // Forge's invariant fuzzer ignores reverted target calls, so we surface the failure
            // by flipping a flag the invariant asserts on.
            badRelayResult = true;
        }

        bool relaySucceeded = xdm.successfulMessages(relayMessageHash);
        bool relayFailed = xdm.failedMessages(relayMessageHash);
        if (relaySucceeded == shouldFail || relayFailed != shouldFail) {
            badRelayResult = true;
        }
    }
}

contract XDM_MinGasLimits is CommonTest {
    RelayActor actor;

    function _init(bool _shouldFail) internal {
        super.setUp();

        StorageSlot memory l2SenderSlot = ForgeArtifacts.getSlot("OptimismPortal2", "l2Sender");
        vm.store(
            address(optimismPortal2),
            bytes32(l2SenderSlot.slot),
            bytes32(abi.encode(Predeploys.L2_CROSS_DOMAIN_MESSENGER))
        );
        actor = new RelayActor(address(optimismPortal2), l1CrossDomainMessenger, vm, _shouldFail);

        vm.deal(address(optimismPortal2), type(uint128).max);

        targetContract(address(actor));

        bytes4[] memory selectors = new bytes4[](1);
        selectors[0] = actor.relay.selector;
        targetSelector(FuzzSelector({ addr: address(actor), selectors: selectors }));
    }

    function _assertRelayResults() internal view {
        assertFalse(actor.badRelayResult());
    }
}

contract XDM_MinGasLimits_Succeeds is XDM_MinGasLimits {
    function setUp() public override {
        super._init(false);
    }

    /// @custom:invariant `relayMessage` should succeed when the outer call has base gas and the
    ///                   target can receive the inner minimum gas limit.
    function invariant_relayMessage_forwardsMinGas_succeeds() external view {
        _assertRelayResults();
    }
}

contract XDM_MinGasLimits_Reverts is XDM_MinGasLimits {
    function setUp() public override {
        super._init(true);
    }

    /// @custom:invariant `relayMessage` should mark the message failed when the inner minimum gas
    ///                   limit is too large to forward to the target.
    function invariant_relayMessage_insufficientMinGas_fails() external view {
        _assertRelayResults();
    }
}

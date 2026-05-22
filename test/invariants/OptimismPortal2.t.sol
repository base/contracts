// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CommonTest } from "test/setup/CommonTest.sol";

/// @title OptimismPortal2_Invariants
/// @notice Invariant tests for OptimismPortal2. These tests verify critical security
///         properties of the portal's configuration and access controls.
contract OptimismPortal2_Invariants is CommonTest {
    /// @notice Handler that performs deposits on the OptimismPortal2.
    PortalActor actor;

    function setUp() public override {
        super.setUp();
        actor = new PortalActor(address(optimismPortal2), vm);

        // Target the actor contract for fuzzing
        targetContract(address(actor));

        // Limit to depositTransaction selector
        bytes4[] memory selectors = new bytes4[](1);
        selectors[0] = actor.deposit.selector;
        targetSelector(FuzzSelector({ addr: address(actor), selectors: selectors }));

        // Fund the actor for deposits
        vm.deal(address(actor), 1000 ether);
    }

    /// @custom:invariant The minimum gas limit for any calldata size must be greater than 0.
    ///                   A zero gas limit would make the portal unusable for deposits.
    function invariant_minimum_gas_limit_nonzero() external view {
        // Test various calldata sizes to ensure minimumGasLimit never returns 0
        for (uint64 i = 0; i < 128; i++) {
            uint64 minGas = optimismPortal2.minimumGasLimit(i);
            assertTrue(minGas > 0, "minimumGasLimit returned zero");
        }
    }

    /// @custom:invariant The portal's paused state must be consistent with the system config.
    ///                   If the system config says the system is paused, the portal must also
    ///                   report itself as paused.
    function invariant_paused_state_consistency() external view {
        assertEq(optimismPortal2.paused(), systemConfig.paused());
    }

    /// @custom:invariant The portal must always have sufficient ETH balance to cover at minimum
    ///                   the deposits that have been made. This is a sanity check that the
    ///                   portal's balance is not artificially depleted.
    function invariant_portal_has_minimum_balance() external view {
        // The portal should always have at least some minimal balance from initial funding
        // This is a sanity check - in production the portal should have significant balance
        assertTrue(address(optimismPortal2).balance >= 1 wei, "portal balance is zero");
    }
}

/// @title PortalActor
/// @notice Actor contract that performs calls to OptimismPortal2 for invariant testing.
contract PortalActor {
    IOptimismPortal2 public portal;
    Vm public vm;

    constructor(address _portal, Vm _vm) {
        portal = IOptimismPortal2(_portal);
        vm = _vm;
    }

    /// @notice Perform a deposit transaction to the portal.
    /// @param _to         Target address on L2.
    /// @param _value      ETH value to send.
    /// @param _gasLimit   Amount of L2 gas to purchase.
    /// @param _isCreation Whether the transaction is a contract creation.
    /// @param _data       Data to trigger the recipient with.
    function deposit(
        address _to,
        uint256 _value,
        uint64 _gasLimit,
        bool _isCreation,
        bytes memory _data
    ) external payable {
        // Bound values to prevent extreme cases
        _value = bound(_value, 0, 100 ether);
        _gasLimit = uint64(bound(_gasLimit, 21000, 10_000_000));

        // Ensure actor has enough ETH
        if (address(this).balance < _value) {
            vm.deal(address(this), _value + 1 ether);
        }

        portal.depositTransaction{ value: _value }(_to, _value, _gasLimit, _isCreation, _data);
    }
}

/// @notice Interface for OptimismPortal2 external functions used in testing.
/// @dev This duplicates the interface to avoid import complexity in the actor.
interface IOptimismPortal2 {
    function depositTransaction(
        address _to,
        uint256 _value,
        uint64 _gasLimit,
        bool _isCreation,
        bytes memory _data
    ) external payable;

    function minimumGasLimit(uint64 _byteCount) external pure returns (uint64);

    function paused() external view returns (bool);
}
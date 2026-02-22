// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import { ICBMulticall, Call3Value } from "src/utils/ICBMulticall.sol";
import { Test } from "forge-std/Test.sol";
import { Vm } from "forge-std/Vm.sol";
import { Preinstalls } from "src/libraries/Preinstalls.sol";

import { MultisigScriptDeposit } from "script/universal/MultisigScriptDeposit.sol";
import { Simulation } from "script/universal/Simulation.sol";
import { IGnosisSafe } from "script/universal/IGnosisSafe.sol";

import { Counter } from "test/universal/Counter.sol";

/// @notice Mock OptimismPortal for testing deposit transactions
contract MockOptimismPortal {
    event TransactionDeposited(address indexed from, address indexed to, uint256 value, bytes data);

    /// @notice Records the deposit transaction parameters for verification
    address public lastTo;
    uint256 public lastValue;
    uint64 public lastGasLimit;
    bool public lastIsCreation;
    bytes public lastData;
    uint256 public depositCount;

    function depositTransaction(
        address _to,
        uint256 _value,
        uint64 _gasLimit,
        bool _isCreation,
        bytes memory _data
    )
        external
        payable
    {
        require(msg.value == _value, "MockPortal: value mismatch");

        lastTo = _to;
        lastValue = _value;
        lastGasLimit = _gasLimit;
        lastIsCreation = _isCreation;
        lastData = _data;
        depositCount++;

        emit TransactionDeposited(msg.sender, _to, _value, _data);
    }
}

contract MultisigScriptDepositTest is Test, MultisigScriptDeposit {
    Vm.Wallet internal wallet1 = vm.createWallet("1");
    Vm.Wallet internal wallet2 = vm.createWallet("2");

    address internal safe = address(1001);
    MockOptimismPortal internal portal;
    Counter internal l2Counter;

    // Test configuration
    address internal testL2Target;
    uint64 internal testGasLimit = 200_000;

    function() internal view returns (Call3Value[] memory) buildL2CallsInternal;

    function setUp() public {
        // Deploy mock portal
        portal = new MockOptimismPortal();

        // Deploy a counter to use as L2 target (for calldata encoding)
        l2Counter = new Counter(address(this));
        testL2Target = address(l2Counter);

        // Setup Safe
        vm.etch(safe, Preinstalls.getDeployedCode(Preinstalls.Safe_v130, block.chainid));
        vm.etch(Preinstalls.MultiCall3, Preinstalls.getDeployedCode(Preinstalls.MultiCall3, block.chainid));
        vm.deal(safe, 100 ether);

        address[] memory owners = new address[](2);
        owners[0] = wallet1.addr;
        owners[1] = wallet2.addr;
        IGnosisSafe(safe).setup(owners, 2, address(0), "", address(0), address(0), 0, address(0));
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                        MultisigScriptDeposit Overrides                         ///
    //////////////////////////////////////////////////////////////////////////////////////

    function _optimismPortal() internal view override returns (address) {
        return address(portal);
    }

    function _l2GasLimit() internal view override returns (uint64) {
        return testGasLimit;
    }

    function _ownerSafe() internal view override returns (address) {
        return safe;
    }

    function _buildL2Calls() internal view override returns (Call3Value[] memory) {
        return buildL2CallsInternal();
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal pure override {
        // No-op for tests - the deposit is simulated but not executed during sign()
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                    Tests                                       ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Test that a single L2 call is correctly wrapped in depositTransaction
    function test_buildCalls_singleL2Call_noValue() external {
        buildL2CallsInternal = _buildSingleL2CallNoValue;

        Call[] memory calls = _buildCalls();

        // Should produce exactly one L1 call to the portal
        assertEq(calls.length, 1, "Should have one L1 call");
        assertEq(calls[0].target, address(portal), "Target should be portal");
        assertEq(calls[0].value, 0, "Value should be 0");

        // Decode the depositTransaction call
        (address to, uint256 value, uint64 gasLimit, bool isCreation, bytes memory data) =
            _decodeDepositTransaction(calls[0].data);

        assertEq(to, CB_MULTICALL, "L2 target should be CB_MULTICALL");
        assertEq(value, 0, "Bridged value should be 0");
        assertEq(gasLimit, testGasLimit, "Gas limit should match");
        assertFalse(isCreation, "Should not be creation");
        assertTrue(data.length > 0, "Data should not be empty");

        // Verify the L2 data is an aggregate3Value call
        bytes4 selector = bytes4(data);
        assertEq(selector, ICBMulticall.aggregate3Value.selector, "Should be aggregate3Value call");
    }

    /// @notice Test that multiple L2 calls are batched correctly
    function test_buildCalls_multipleL2Calls_noValue() external {
        buildL2CallsInternal = _buildMultipleL2CallsNoValue;

        Call[] memory calls = _buildCalls();

        // Should still produce exactly one L1 call
        assertEq(calls.length, 1, "Should have one L1 call");

        // Decode and verify
        (address to, uint256 value, uint64 gasLimit, bool isCreation, bytes memory data) =
            _decodeDepositTransaction(calls[0].data);

        assertEq(to, CB_MULTICALL, "L2 target should be CB_MULTICALL");
        assertEq(value, 0, "Bridged value should be 0");
        assertEq(gasLimit, testGasLimit, "Gas limit should match");
        assertFalse(isCreation, "Should not be creation");

        // Decode the aggregate3Value call to verify multiple L2 calls are included
        Call3Value[] memory l2Calls = abi.decode(_stripSelector(data), (Call3Value[]));
        assertEq(l2Calls.length, 3, "Should have 3 L2 calls");

        // Verify call parameters are preserved through the wrapping
        assertEq(l2Calls[0].target, testL2Target, "First call target should be preserved");
        assertEq(l2Calls[1].target, testL2Target, "Second call target should be preserved");
        assertEq(l2Calls[2].target, testL2Target, "Third call target should be preserved");

        // Verify allowFailure flags are preserved (first two are false, third is true)
        assertFalse(l2Calls[0].allowFailure, "First call allowFailure should be false");
        assertFalse(l2Calls[1].allowFailure, "Second call allowFailure should be false");
        assertTrue(l2Calls[2].allowFailure, "Third call allowFailure should be true (preserved)");
    }

    /// @notice Test that ETH values are correctly summed and bridged
    function test_buildCalls_withValue() external {
        buildL2CallsInternal = _buildL2CallsWithValue;

        Call[] memory calls = _buildCalls();

        // Value should be sum of all L2 call values (1 + 2 + 0.5 = 3.5 ether)
        assertEq(calls[0].value, 3.5 ether, "L1 call value should be sum of L2 values");

        // Decode and verify bridged value
        (, uint256 value,,,) = _decodeDepositTransaction(calls[0].data);
        assertEq(value, 3.5 ether, "Bridged value should be 3.5 ether");
    }

    /// @notice Test that single L2 call still goes through multicall (consistent behavior)
    function test_buildCalls_singleCallStillUsesMulticall() external {
        buildL2CallsInternal = _buildSingleL2CallNoValue;

        Call[] memory calls = _buildCalls();
        (,,,, bytes memory data) = _decodeDepositTransaction(calls[0].data);

        // Even single calls should be wrapped in aggregate3Value for consistency
        bytes4 selector = bytes4(data);
        assertEq(selector, ICBMulticall.aggregate3Value.selector, "Single call should still use aggregate3Value");
    }

    /// @notice Test the full sign flow with deposit transaction
    function test_sign_depositTransaction() external {
        buildL2CallsInternal = _buildSingleL2CallNoValue;

        vm.recordLogs();
        bytes memory txData = abi.encodeWithSelector(this.sign.selector, new address[](0));
        vm.prank(wallet1.addr);
        (bool success,) = address(this).call(txData);
        assertTrue(success, "Sign should succeed");

        // Verify DataToSign event was emitted
        Vm.Log[] memory logs = vm.getRecordedLogs();
        bool foundDataToSign = false;
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics[0] == keccak256("DataToSign(bytes)")) {
                foundDataToSign = true;
                break;
            }
        }
        assertTrue(foundDataToSign, "DataToSign event should be emitted");
    }

    /// @notice Test sign flow with ETH value
    function test_sign_depositTransaction_withValue() external {
        buildL2CallsInternal = _buildL2CallsWithValue;

        vm.recordLogs();
        bytes memory txData = abi.encodeWithSelector(this.sign.selector, new address[](0));
        vm.prank(wallet1.addr);
        (bool success,) = address(this).call(txData);
        assertTrue(success, "Sign with value should succeed");
    }

    /// @notice Test default _optimismPortal() returns correct address for mainnet
    function test_optimismPortal_mainnet() external {
        // Create a separate test contract that uses default _optimismPortal()
        vm.chainId(1);
        DefaultPortalTest defaultTest = new DefaultPortalTest();
        assertEq(
            defaultTest.getOptimismPortal(), 0x49048044D57e1C92A77f79988d21Fa8fAF74E97e, "Should return mainnet portal"
        );
    }

    /// @notice Test default _optimismPortal() returns correct address for sepolia
    function test_optimismPortal_sepolia() external {
        vm.chainId(11155111);
        DefaultPortalTest defaultTest = new DefaultPortalTest();
        assertEq(
            defaultTest.getOptimismPortal(), 0x49f53e41452C74589E85cA1677426Ba426459e85, "Should return sepolia portal"
        );
    }

    /// @notice Test default _optimismPortal() reverts for unknown chain
    function test_optimismPortal_unknownChain_reverts() external {
        vm.chainId(999999);
        DefaultPortalTest defaultTest = new DefaultPortalTest();
        vm.expectRevert("MultisigScriptDeposit: unsupported chain, override _optimismPortal()");
        defaultTest.getOptimismPortal();
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                              Helper Functions                                  ///
    //////////////////////////////////////////////////////////////////////////////////////

    function _buildSingleL2CallNoValue() internal view returns (Call3Value[] memory) {
        Call3Value[] memory calls = new Call3Value[](1);
        calls[0] = Call3Value({
            target: testL2Target, allowFailure: false, callData: abi.encodeCall(Counter.increment, ()), value: 0
        });
        return calls;
    }

    function _buildMultipleL2CallsNoValue() internal view returns (Call3Value[] memory) {
        Call3Value[] memory calls = new Call3Value[](3);
        calls[0] = Call3Value({
            target: testL2Target, allowFailure: false, callData: abi.encodeCall(Counter.increment, ()), value: 0
        });
        calls[1] = Call3Value({
            target: testL2Target, allowFailure: false, callData: abi.encodeCall(Counter.increment, ()), value: 0
        });
        calls[2] = Call3Value({
            target: testL2Target,
            allowFailure: true, // Test allowFailure flag preservation
            callData: abi.encodeCall(Counter.increment, ()),
            value: 0
        });
        return calls;
    }

    function _buildL2CallsWithValue() internal view returns (Call3Value[] memory) {
        Call3Value[] memory calls = new Call3Value[](3);
        calls[0] = Call3Value({
            target: testL2Target,
            allowFailure: false,
            callData: abi.encodeCall(Counter.incrementPayable, ()),
            value: 1 ether
        });
        calls[1] = Call3Value({
            target: testL2Target,
            allowFailure: false,
            callData: abi.encodeCall(Counter.incrementPayable, ()),
            value: 2 ether
        });
        calls[2] = Call3Value({
            target: testL2Target,
            allowFailure: false,
            callData: abi.encodeCall(Counter.incrementPayable, ()),
            value: 0.5 ether
        });
        return calls;
    }

    /// @notice Decode depositTransaction calldata
    function _decodeDepositTransaction(bytes memory callData)
        internal
        pure
        returns (address to, uint256 value, uint64 gasLimit, bool isCreation, bytes memory data)
    {
        // Skip the 4-byte selector
        bytes memory params = _stripSelector(callData);
        (to, value, gasLimit, isCreation, data) = abi.decode(params, (address, uint256, uint64, bool, bytes));
    }

    /// @notice Strip the 4-byte function selector from calldata
    function _stripSelector(bytes memory data) internal pure returns (bytes memory) {
        require(data.length >= 4, "Data too short");
        bytes memory result = new bytes(data.length - 4);
        for (uint256 i = 4; i < data.length; i++) {
            result[i - 4] = data[i];
        }
        return result;
    }
}

/// @notice Helper contract to test default _optimismPortal() implementation
/// @dev This contract does NOT override _optimismPortal(), so it uses the default chain-based logic
contract DefaultPortalTest is MultisigScriptDeposit {
    function _ownerSafe() internal pure override returns (address) {
        return address(1);
    }

    function _buildL2Calls() internal pure override returns (Call3Value[] memory) {
        return new Call3Value[](0);
    }

    function _postCheck(Vm.AccountAccess[] memory, Simulation.Payload memory) internal pure override { }

    /// @notice Expose _optimismPortal() for testing
    function getOptimismPortal() external view returns (address) {
        return _optimismPortal();
    }
}

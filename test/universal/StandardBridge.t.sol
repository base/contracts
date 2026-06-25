// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { StandardBridge } from "src/universal/StandardBridge.sol";
import { CommonTest } from "test/setup/CommonTest.sol";
import { OptimismMintableERC20 } from "src/universal/OptimismMintableERC20.sol";
import { ILegacyMintableERC20 } from "interfaces/legacy/ILegacyMintableERC20.sol";
import { ERC20 } from "lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import { IERC165 } from "lib/openzeppelin-contracts/contracts/utils/introspection/IERC165.sol";

/// @title StandardBridgeTester
/// @notice Simple wrapper around the StandardBridge contract that exposes
///         internal functions so they can be more easily tested directly.
contract StandardBridgeTester is StandardBridge {
    function isOptimismMintableERC20(address _token) external view returns (bool) {
        return _isOptimismMintableERC20(_token);
    }

    function isCorrectTokenPair(address _mintableToken, address _otherToken) external view returns (bool) {
        return _isCorrectTokenPair(_mintableToken, _otherToken);
    }

    receive() external payable override { }
}

/// @title LegacyMintable
/// @notice Simple implementation of the legacy OptimismMintableERC20.
contract LegacyMintable is ERC20 {
    constructor(string memory _name, string memory _ticker) ERC20(_name, _ticker) { }

    function l1Token() external pure returns (address) {
        return address(0);
    }

    function mint(address _to, uint256 _amount) external pure { }

    function burn(address _from, uint256 _amount) external pure { }

    /// @notice Matches the deployed legacy mintable token's ERC165 support.
    function supportsInterface(bytes4 _interfaceId) external pure returns (bool) {
        return _interfaceId == type(IERC165).interfaceId || _interfaceId == type(ILegacyMintableERC20).interfaceId;
    }
}

/// @title StandardBridge_TestInit
/// @notice Reusable test initialization for `StandardBridge` tests.
/// @dev This setup is primarily for tests focusing on internal stateless logic or default states
///      of the `StandardBridge` contract.
abstract contract StandardBridge_TestInit is CommonTest {
    address internal constant OTHER_TOKEN = address(0x20);

    StandardBridgeTester internal bridge;
    OptimismMintableERC20 internal mintable;
    LegacyMintable internal legacy;

    function setUp() public override {
        super.setUp();

        bridge = new StandardBridgeTester();

        mintable = new OptimismMintableERC20({
            _bridge: address(0), _remoteToken: address(0), _name: "Stonks", _symbol: "STONK", _decimals: 18
        });

        legacy = new LegacyMintable("Legacy", "LEG");
    }
}

/// @title StandardBridge_Paused_Test
/// @notice Tests the `paused` function of the `StandardBridge` contract.
contract StandardBridge_Paused_Test is StandardBridge_TestInit {
    /// @notice The bridge by default should be unpaused.
    function test_paused_succeeds() external view {
        assertFalse(bridge.paused());
    }
}

/// @title StandardBridge_IsOptimismMintableERC20_Test
/// @notice Tests the `_isOptimismMintableERC20` internal function of `StandardBridge`.
contract StandardBridge_IsOptimismMintableERC20_Test is StandardBridge_TestInit {
    function test_isOptimismMintableERC20_succeeds() external view {
        assertTrue(bridge.isOptimismMintableERC20(address(mintable)));
        assertTrue(bridge.isOptimismMintableERC20(address(legacy)));
        assertFalse(bridge.isOptimismMintableERC20(address(L1Token)));

        assertEq(OTHER_TOKEN.code.length, 0);
        assertFalse(bridge.isOptimismMintableERC20(OTHER_TOKEN));
    }
}

/// @title StandardBridge_IsCorrectTokenPair_Test
/// @notice Tests the `_isCorrectTokenPair` internal function of `StandardBridge`.
contract StandardBridge_IsCorrectTokenPair_Test is StandardBridge_TestInit {
    function test_isCorrectTokenPair_succeeds() external {
        assertTrue(bridge.isCorrectTokenPair(address(mintable), mintable.remoteToken()));
        assertTrue(bridge.isCorrectTokenPair(address(mintable), mintable.l1Token()));
        assertFalse(bridge.isCorrectTokenPair(address(mintable), OTHER_TOKEN));

        assertTrue(bridge.isCorrectTokenPair(address(legacy), legacy.l1Token()));
        assertFalse(bridge.isCorrectTokenPair(address(legacy), OTHER_TOKEN));

        vm.expectRevert(); // nosemgrep: sol-safety-expectrevert-no-args
        bridge.isCorrectTokenPair(address(L1Token), address(1));
    }
}

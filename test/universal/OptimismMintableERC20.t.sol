// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CommonTest } from "test/setup/CommonTest.sol";
import { IOptimismMintableERC20 } from "interfaces/universal/IOptimismMintableERC20.sol";
import { ILegacyMintableERC20 } from "interfaces/legacy/ILegacyMintableERC20.sol";
import { IERC165 } from "lib/openzeppelin-contracts/contracts/utils/introspection/IERC165.sol";

/// @title OptimismMintableERC20_TestInit
/// @notice Reusable test initialization for `OptimismMintableERC20` tests.
abstract contract OptimismMintableERC20_TestInit is CommonTest {
    event Mint(address indexed account, uint256 amount);
    event Burn(address indexed account, uint256 amount);

    function _bridgeMint(address _to, uint256 _amount) internal {
        vm.prank(address(l2StandardBridge));
        L2Token.mint(_to, _amount);
    }
}

/// @title OptimismMintableERC20_Permit2_Test
/// @notice Tests the `permit2` function of the `OptimismMintableERC20` contract.
contract OptimismMintableERC20_Permit2_Test is OptimismMintableERC20_TestInit {
    function test_permit2_transferFrom_succeeds() external {
        _bridgeMint(alice, 100);

        vm.prank(L2Token.PERMIT2());
        L2Token.transferFrom(alice, bob, 100);
        assertEq(L2Token.balanceOf(bob), 100);
    }
}

/// @title OptimismMintableERC20_Allowance_Test
/// @notice Tests the `allowance` function of the `OptimismMintableERC20` contract.
contract OptimismMintableERC20_Allowance_Test is OptimismMintableERC20_TestInit {
    function test_allowance_permit2Max_works() external view {
        assertEq(L2Token.allowance(alice, L2Token.PERMIT2()), type(uint256).max);
    }
}

/// @title OptimismMintableERC20_Mint_Test
/// @notice Tests the `mint` function of the `OptimismMintableERC20` contract.
contract OptimismMintableERC20_Mint_Test is OptimismMintableERC20_TestInit {
    function test_mint_succeeds() external {
        vm.expectEmit(true, true, true, true);
        emit Mint(alice, 100);

        _bridgeMint(alice, 100);

        assertEq(L2Token.balanceOf(alice), 100);
    }

    function test_mint_notBridge_reverts() external {
        vm.expectRevert("OptimismMintableERC20: only bridge can mint and burn");
        vm.prank(address(alice));
        L2Token.mint(alice, 100);
    }
}

/// @title OptimismMintableERC20_Burn_Test
/// @notice Tests the `burn` function of the `OptimismMintableERC20` contract.
contract OptimismMintableERC20_Burn_Test is OptimismMintableERC20_TestInit {
    function test_burn_succeeds() external {
        _bridgeMint(alice, 100);

        vm.expectEmit(true, true, true, true);
        emit Burn(alice, 100);

        vm.prank(address(l2StandardBridge));
        L2Token.burn(alice, 100);

        assertEq(L2Token.balanceOf(alice), 0);
    }

    function test_burn_notBridge_reverts() external {
        vm.expectRevert("OptimismMintableERC20: only bridge can mint and burn");
        vm.prank(address(alice));
        L2Token.burn(alice, 100);
    }
}

/// @title OptimismMintableERC20_SupportsInterface_Test
/// @notice Tests the `supportsInterface` function of the `OptimismMintableERC20` contract.
contract OptimismMintableERC20_SupportsInterface_Test is OptimismMintableERC20_TestInit {
    function test_erc165_supportsInterface_succeeds() external view {
        assertTrue(L2Token.supportsInterface(type(IERC165).interfaceId));
        assertTrue(L2Token.supportsInterface(type(ILegacyMintableERC20).interfaceId));
        assertTrue(L2Token.supportsInterface(type(IOptimismMintableERC20).interfaceId));
    }
}

/// @title OptimismMintableERC20_Getters_Test
/// @notice Tests getter and legacy getter functions of the `OptimismMintableERC20` contract.
contract OptimismMintableERC20_Getters_Test is OptimismMintableERC20_TestInit {
    function test_getters_succeeds() external view {
        assertEq(L2Token.REMOTE_TOKEN(), address(L1Token));
        assertEq(L2Token.remoteToken(), address(L1Token));
        assertEq(L2Token.l1Token(), address(L1Token));
        assertEq(L2Token.BRIDGE(), address(l2StandardBridge));
        assertEq(L2Token.bridge(), address(l2StandardBridge));
        assertEq(L2Token.l2Bridge(), address(l2StandardBridge));
    }
}

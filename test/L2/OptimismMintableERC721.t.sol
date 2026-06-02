// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IERC721 } from "lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import { IERC721Enumerable } from "lib/openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import { IERC165 } from "lib/openzeppelin-contracts/contracts/utils/introspection/IERC165.sol";
import { Strings } from "lib/openzeppelin-contracts/contracts/utils/Strings.sol";
import { CommonTest } from "test/setup/CommonTest.sol";
import { OptimismMintableERC721, IOptimismMintableERC721 } from "src/L2/OptimismMintableERC721.sol";

/// @title OptimismMintableERC721_TestInit
/// @notice Reusable test initialization for `OptimismMintableERC721` tests.
abstract contract OptimismMintableERC721_TestInit is CommonTest {
    uint256 internal constant remoteChainId = 1;
    uint256 internal constant tokenId = 1;

    address internal L1NFT;
    OptimismMintableERC721 internal L2NFT;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);

    event Mint(address indexed account, uint256 tokenId);

    event Burn(address indexed account, uint256 tokenId);

    function setUp() public override {
        super.setUp();

        L1NFT = makeAddr("L1NFT");
        L2NFT = new OptimismMintableERC721(address(l2ERC721Bridge), remoteChainId, L1NFT, "L2NFT", "L2T");

        vm.label(L1NFT, "L1ERC721Token");
        vm.label(address(L2NFT), "L2ERC721Token");
    }

    function _bridgeMint(address _to) internal {
        vm.prank(address(l2ERC721Bridge));
        L2NFT.safeMint(_to, tokenId);
    }
}

/// @title OptimismMintableERC721_Constructor_Test
/// @notice Tests the `constructor` of the `OptimismMintableERC721` contract.
contract OptimismMintableERC721_Constructor_Test is OptimismMintableERC721_TestInit {
    /// @notice Tests that the constructor initializes state variables correctly with valid inputs.
    function test_constructor_succeeds() external view {
        assertEq(L2NFT.name(), "L2NFT");
        assertEq(L2NFT.symbol(), "L2T");
        assertEq(L2NFT.remoteToken(), L1NFT);
        assertEq(L2NFT.bridge(), address(l2ERC721Bridge));
        assertEq(L2NFT.remoteChainId(), remoteChainId);
        assertEq(L2NFT.REMOTE_TOKEN(), L1NFT);
        assertEq(L2NFT.BRIDGE(), address(l2ERC721Bridge));
        assertEq(L2NFT.REMOTE_CHAIN_ID(), remoteChainId);
    }

    /// @notice Tests that the constructor reverts when the bridge address is zero.
    function test_constructor_bridgeAsAddress0_reverts() external {
        vm.expectRevert("OptimismMintableERC721: bridge cannot be address(0)");
        L2NFT = new OptimismMintableERC721(address(0), remoteChainId, L1NFT, "L2NFT", "L2T");
    }

    /// @notice Tests that the constructor reverts when the remote chain ID is zero.
    function test_constructor_remoteChainId0_reverts() external {
        vm.expectRevert("OptimismMintableERC721: remote chain id cannot be zero");
        L2NFT = new OptimismMintableERC721(address(l2ERC721Bridge), 0, L1NFT, "L2NFT", "L2T");
    }

    /// @notice Tests that the constructor reverts when the remote token address is zero.
    function test_constructor_remoteTokenAsAddress0_reverts() external {
        vm.expectRevert("OptimismMintableERC721: remote token cannot be address(0)");
        L2NFT = new OptimismMintableERC721(address(l2ERC721Bridge), remoteChainId, address(0), "L2NFT", "L2T");
    }
}

/// @title OptimismMintableERC721_SafeMint_Test
/// @notice Tests the `safeMint` function of the `OptimismMintableERC721` contract.
contract OptimismMintableERC721_SafeMint_Test is OptimismMintableERC721_TestInit {
    /// @notice Tests that the `safeMint` function successfully mints a token when called by the
    ///         bridge.
    function test_safeMint_succeeds() external {
        vm.expectEmit(address(L2NFT));
        emit Transfer(address(0), alice, tokenId);

        vm.expectEmit(address(L2NFT));
        emit Mint(alice, tokenId);

        _bridgeMint(alice);

        assertEq(L2NFT.ownerOf(tokenId), alice);
    }

    /// @notice Tests that the `safeMint` function reverts when called by an address other than the bridge.
    function test_safeMint_notBridge_reverts() external {
        vm.expectRevert("OptimismMintableERC721: only bridge can call this function");
        vm.prank(address(alice));
        L2NFT.safeMint(alice, tokenId);
    }
}

/// @title OptimismMintableERC721_Burn_Test
/// @notice Tests the `burn` function of the `OptimismMintableERC721` contract.
contract OptimismMintableERC721_Burn_Test is OptimismMintableERC721_TestInit {
    /// @notice Tests that the `burn` function successfully burns a token when called by the
    ///         bridge.
    function test_burn_succeeds() external {
        _bridgeMint(alice);

        vm.expectEmit(address(L2NFT));
        emit Transfer(alice, address(0), tokenId);

        vm.expectEmit(address(L2NFT));
        emit Burn(alice, tokenId);

        vm.prank(address(l2ERC721Bridge));
        L2NFT.burn(alice, tokenId);

        vm.expectRevert("ERC721: invalid token ID");
        L2NFT.ownerOf(tokenId);
    }

    /// @notice Tests that the `burn` function reverts when called by an address other than the
    ///         bridge.
    function test_burn_notBridge_reverts() external {
        _bridgeMint(alice);

        vm.expectRevert("OptimismMintableERC721: only bridge can call this function");
        vm.prank(address(alice));
        L2NFT.burn(alice, tokenId);
    }
}

/// @title OptimismMintableERC721_SupportsInterface_Test
/// @notice Tests the `supportsInterface` function of the `OptimismMintableERC721` contract.
contract OptimismMintableERC721_SupportsInterface_Test is OptimismMintableERC721_TestInit {
    /// @notice Tests that the `supportsInterface` function returns true for
    ///         IOptimismMintableERC721, IERC721Enumerable, IERC721 and IERC165 interfaces.
    function test_supportsInterface_succeeds() external view {
        assertTrue(L2NFT.supportsInterface(type(IOptimismMintableERC721).interfaceId));
        assertTrue(L2NFT.supportsInterface(type(IERC721Enumerable).interfaceId));
        assertTrue(L2NFT.supportsInterface(type(IERC721).interfaceId));
        assertTrue(L2NFT.supportsInterface(type(IERC165).interfaceId));
    }
}

/// @title OptimismMintableERC721_Uncategorized_Test
/// @notice General tests that are not testing any function directly of the
///         `OptimismMintableERC721` contract.
contract OptimismMintableERC721_Uncategorized_Test is OptimismMintableERC721_TestInit {
    /// @notice Tests that the `tokenURI` function returns the correct URI for a minted token.
    function test_tokenURI_succeeds() external {
        _bridgeMint(alice);

        assertEq(
            L2NFT.tokenURI(tokenId),
            string.concat(
                "ethereum:",
                Strings.toHexString(uint160(L1NFT), 20),
                "@",
                Strings.toString(remoteChainId),
                "/tokenURI?uint256=",
                Strings.toString(tokenId)
            )
        );
    }
}

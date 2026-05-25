// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Testing
import { CommonTest } from "test/setup/CommonTest.sol";

// Contracts
import { OptimismMintableERC721 } from "src/L2/OptimismMintableERC721.sol";

// Interfaces
import { IL1ERC721Bridge } from "interfaces/L1/IL1ERC721Bridge.sol";
import { ICrossDomainMessenger } from "interfaces/universal/ICrossDomainMessenger.sol";

/// @title TestMintableERC721
/// @notice A test OptimismMintableERC721 token used for `L2ERC721Bridge` tests.
contract L2ERC721Bridge_TestMintableERC721_Harness is OptimismMintableERC721 {
    constructor(address _bridge, address _remoteToken)
        OptimismMintableERC721(_bridge, 1, _remoteToken, "Test", "TST")
    { }

    function mint(address to, uint256 tokenId) public {
        _mint(to, tokenId);
    }
}

/// @title NonCompliantERC721
/// @notice A non-compliant ERC721 token that does not implement the full ERC721 interface.
///         This is used to test that the bridge will revert if the token does not claim to
///         support the ERC721 interface.
contract L2ERC721Bridge_NonCompliantERC721_Harness {
    address internal immutable owner;

    constructor(address _owner) {
        owner = _owner;
    }

    function ownerOf(uint256) external view returns (address) {
        return owner;
    }

    function remoteToken() external pure returns (address) {
        return address(0x01);
    }

    function burn(address, uint256) external { }

    function supportsInterface(bytes4) external pure returns (bool) {
        return false;
    }
}

/// @title L2ERC721Bridge_TestInit
/// @notice Reusable test initialization for `L2ERC721Bridge` tests.
abstract contract L2ERC721Bridge_TestInit is CommonTest {
    uint256 internal constant tokenId = 1;
    uint32 internal constant DEFAULT_MIN_GAS_LIMIT = 1234;
    bytes internal constant EXTRA_DATA = hex"5678";
    address internal constant remoteToken = address(0x01);

    event ERC721BridgeInitiated(
        address indexed localToken,
        address indexed remoteToken,
        address indexed from,
        address to,
        uint256 tokenId,
        bytes extraData
    );

    event ERC721BridgeFinalized(
        address indexed localToken,
        address indexed remoteToken,
        address indexed from,
        address to,
        uint256 tokenId,
        bytes extraData
    );
}

abstract contract L2ERC721Bridge_Bridge_TestInit is L2ERC721Bridge_TestInit {
    L2ERC721Bridge_TestMintableERC721_Harness internal localToken;

    /// @notice Sets up the test suite.
    function setUp() public virtual override {
        super.setUp();

        localToken = new L2ERC721Bridge_TestMintableERC721_Harness(address(l2ERC721Bridge), remoteToken);

        localToken.mint(alice, tokenId);

        vm.prank(alice);
        localToken.approve(address(l2ERC721Bridge), tokenId);
    }

    function _expectBridgeMessage(address _to) internal {
        bytes memory message = abi.encodeCall(
            IL1ERC721Bridge.finalizeBridgeERC721, (remoteToken, address(localToken), alice, _to, tokenId, EXTRA_DATA)
        );

        vm.expectCall(
            address(l2ERC721Bridge.messenger()),
            abi.encodeCall(
                ICrossDomainMessenger.sendMessage,
                (address(l2ERC721Bridge.otherBridge()), message, DEFAULT_MIN_GAS_LIMIT)
            )
        );
    }

    function _expectBridgeInitiated(address _to) internal {
        vm.expectEmit(address(l2ERC721Bridge));
        emit ERC721BridgeInitiated(address(localToken), remoteToken, alice, _to, tokenId, EXTRA_DATA);
    }

    function _mockXDomainMessageSender(address _sender) internal {
        vm.mockCall(
            address(l2ERC721Bridge.messenger()),
            abi.encodeCall(ICrossDomainMessenger.xDomainMessageSender, ()),
            abi.encode(_sender)
        );
    }

    function _mockOtherBridge() internal {
        _mockXDomainMessageSender(address(l2ERC721Bridge.otherBridge()));
    }

    function _bridgeToken() internal {
        vm.prank(alice, alice);
        l2ERC721Bridge.bridgeERC721(address(localToken), remoteToken, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);
    }

    function _assertTokenBurned() internal {
        vm.expectRevert("ERC721: invalid token ID");
        localToken.ownerOf(tokenId);
    }

    function _assertTokenOwnedBy(address _owner) internal view {
        assertEq(localToken.ownerOf(tokenId), _owner);
    }
}

/// @title L2ERC721Bridge_Test_Constructor
/// @notice Tests the `constructor` of the `L2ERC721Bridge` contract.
contract L2ERC721Bridge_Constructor_Test is L2ERC721Bridge_TestInit {
    /// @notice Tests that the constructor sets the correct variables.
    function test_constructor_succeeds() public view {
        assertEq(address(l2ERC721Bridge.MESSENGER()), address(l2CrossDomainMessenger));
        assertEq(address(l2ERC721Bridge.OTHER_BRIDGE()), address(l1ERC721Bridge));
        assertEq(address(l2ERC721Bridge.messenger()), address(l2CrossDomainMessenger));
        assertEq(address(l2ERC721Bridge.otherBridge()), address(l1ERC721Bridge));
    }
}

/// @title L2ERC721Bridge_FinalizeBridgeERC721_Test
/// @notice Tests the `finalizeBridgeERC721` function of the `L2ERC721Bridge` contract.
contract L2ERC721Bridge_FinalizeBridgeERC721_Test is L2ERC721Bridge_Bridge_TestInit {
    /// @notice Tests that `finalizeBridgeERC721` correctly finalizes a bridged token.
    function test_finalizeBridgeERC721_succeeds() external {
        _bridgeToken();

        vm.expectEmit(address(l2ERC721Bridge));
        emit ERC721BridgeFinalized(address(localToken), remoteToken, alice, alice, tokenId, EXTRA_DATA);

        _mockOtherBridge();
        vm.prank(address(l2ERC721Bridge.messenger()));
        l2ERC721Bridge.finalizeBridgeERC721(address(localToken), remoteToken, alice, alice, tokenId, EXTRA_DATA);

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `finalizeBridgeERC721` reverts if the token is not compliant with the
    ///         `IOptimismMintableERC721` interface.
    function test_finalizeBridgeERC721_interfaceNotCompliant_reverts() external {
        L2ERC721Bridge_NonCompliantERC721_Harness nonCompliantToken =
            new L2ERC721Bridge_NonCompliantERC721_Harness(alice);

        vm.prank(alice, alice);
        l2ERC721Bridge.bridgeERC721(address(nonCompliantToken), remoteToken, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _mockOtherBridge();
        vm.prank(address(l2ERC721Bridge.messenger()));
        vm.expectRevert("L2ERC721Bridge: local token interface is not compliant");
        l2ERC721Bridge.finalizeBridgeERC721(address(nonCompliantToken), remoteToken, alice, alice, tokenId, EXTRA_DATA);
    }

    /// @notice Tests that `finalizeBridgeERC721` reverts when not called by the remote bridge.
    function test_finalizeBridgeERC721_notViaLocalMessenger_reverts() external {
        vm.prank(alice);
        vm.expectRevert("ERC721Bridge: function can only be called from the other bridge");
        l2ERC721Bridge.finalizeBridgeERC721(address(localToken), remoteToken, alice, alice, tokenId, EXTRA_DATA);
    }

    /// @notice Tests that `finalizeBridgeERC721` reverts when not called by the remote bridge.
    function test_finalizeBridgeERC721_notFromRemoteMessenger_reverts() external {
        _mockXDomainMessageSender(alice);
        vm.prank(address(l2ERC721Bridge.messenger()));
        vm.expectRevert("ERC721Bridge: function can only be called from the other bridge");
        l2ERC721Bridge.finalizeBridgeERC721(address(localToken), remoteToken, alice, alice, tokenId, EXTRA_DATA);
    }

    /// @notice Tests that `finalizeBridgeERC721` reverts when the local token is the address of
    ///         the bridge itself.
    function test_finalizeBridgeERC721_selfToken_reverts() external {
        _mockOtherBridge();
        vm.prank(address(l2ERC721Bridge.messenger()));
        vm.expectRevert("L2ERC721Bridge: local token cannot be self");
        l2ERC721Bridge.finalizeBridgeERC721(address(l2ERC721Bridge), remoteToken, alice, alice, tokenId, EXTRA_DATA);
    }

    /// @notice Tests that `finalizeBridgeERC721` reverts when already finalized.
    function test_finalizeBridgeERC721_alreadyExists_reverts() external {
        _mockOtherBridge();
        vm.prank(address(l2ERC721Bridge.messenger()));
        vm.expectRevert("ERC721: token already minted");
        l2ERC721Bridge.finalizeBridgeERC721(address(localToken), remoteToken, alice, alice, tokenId, EXTRA_DATA);
    }
}

/// @title L2ERC721Bridge_Paused_Test
/// @notice Tests the `paused` function of the `L2ERC721Bridge` contract.
contract L2ERC721Bridge_Paused_Test is L2ERC721Bridge_TestInit {
    /// @notice Ensures that the L2ERC721Bridge is always not paused. The pausability happens on L1
    ///         and not L2.
    function test_paused_succeeds() external view {
        assertFalse(l2ERC721Bridge.paused());
    }
}

/// @title L2ERC721Bridge_BridgeERC721_Test
/// @notice Tests the `bridgeERC721` and `bridgeERC721To` functions of the `L2ERC721Bridge` contract.
contract L2ERC721Bridge_BridgeERC721_Test is L2ERC721Bridge_Bridge_TestInit {
    /// @notice Tests that `bridgeERC721` correctly bridges a token and burns it on the origin
    ///         chain.
    function test_bridgeERC721_succeeds() public {
        _expectBridgeMessage(alice);
        _expectBridgeInitiated(alice);

        _bridgeToken();

        _assertTokenBurned();
    }

    /// @notice Tests that `bridgeERC721` reverts if the owner is not an EOA.
    function test_bridgeERC721_fromContract_reverts() external {
        vm.etch(alice, hex"01");
        vm.prank(alice);
        vm.expectRevert("ERC721Bridge: account is not externally owned");
        l2ERC721Bridge.bridgeERC721(address(localToken), remoteToken, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `bridgeERC721` reverts if the local token is the zero address.
    function test_bridgeERC721_localTokenZeroAddress_reverts() external {
        vm.prank(alice, alice);
        vm.expectRevert(); // nosemgrep: sol-safety-expectrevert-no-args
        l2ERC721Bridge.bridgeERC721(address(0), remoteToken, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `bridgeERC721` reverts if the remote token is the zero address.
    function test_bridgeERC721_remoteTokenZeroAddress_reverts() external {
        vm.prank(alice, alice);
        vm.expectRevert("L2ERC721Bridge: remote token cannot be address(0)");
        l2ERC721Bridge.bridgeERC721(address(localToken), address(0), tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `bridgeERC721` reverts if the caller is not the token owner.
    function test_bridgeERC721_wrongOwner_reverts() external {
        vm.prank(bob, bob);
        vm.expectRevert("L2ERC721Bridge: Withdrawal is not being initiated by NFT owner");
        l2ERC721Bridge.bridgeERC721(address(localToken), remoteToken, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `bridgeERC721To` correctly bridges a token and burns it on the origin
    ///         chain.
    function test_bridgeERC721To_succeeds() external {
        _expectBridgeMessage(bob);
        _expectBridgeInitiated(bob);

        vm.prank(alice);
        l2ERC721Bridge.bridgeERC721To(address(localToken), remoteToken, bob, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _assertTokenBurned();
    }

    /// @notice Tests that `bridgeERC721To` reverts if the local token is the zero address.
    function test_bridgeERC721To_localTokenZeroAddress_reverts() external {
        vm.prank(alice);
        vm.expectRevert(); // nosemgrep: sol-safety-expectrevert-no-args
        l2ERC721Bridge.bridgeERC721To(
            address(0), address(l1ERC721Bridge), bob, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA
        );

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `bridgeERC721To` reverts if the remote token is the zero address.
    function test_bridgeERC721To_remoteTokenZeroAddress_reverts() external {
        vm.prank(alice);
        vm.expectRevert("L2ERC721Bridge: remote token cannot be address(0)");
        l2ERC721Bridge.bridgeERC721To(address(localToken), address(0), bob, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `bridgeERC721To` reverts if the caller is not the token owner.
    function test_bridgeERC721To_wrongOwner_reverts() external {
        vm.prank(bob);
        vm.expectRevert("L2ERC721Bridge: Withdrawal is not being initiated by NFT owner");
        l2ERC721Bridge.bridgeERC721To(address(localToken), remoteToken, bob, tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA);

        _assertTokenOwnedBy(alice);
    }

    /// @notice Tests that `bridgeERC721To` reverts if the to address is the zero address.
    function test_bridgeERC721To_toZeroAddress_reverts() external {
        vm.prank(bob);
        vm.expectRevert("ERC721Bridge: nft recipient cannot be address(0)");
        l2ERC721Bridge.bridgeERC721To(
            address(localToken), remoteToken, address(0), tokenId, DEFAULT_MIN_GAS_LIMIT, EXTRA_DATA
        );
    }
}

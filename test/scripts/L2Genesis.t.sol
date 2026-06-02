// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";
import { EIP1967Helper } from "test/mocks/EIP1967Helper.sol";
import { L2Genesis } from "scripts/L2Genesis.s.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { LATEST_FORK } from "scripts/libraries/Config.sol";
import { IOptimismMintableERC20Factory } from "interfaces/universal/IOptimismMintableERC20Factory.sol";
import { IOptimismMintableERC721Factory } from "interfaces/L2/IOptimismMintableERC721Factory.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { IGasPriceOracle } from "interfaces/L2/IGasPriceOracle.sol";
import { IFeeVault } from "interfaces/L2/IFeeVault.sol";
import { Types } from "src/libraries/Types.sol";

/// @title L2Genesis_TestInit
/// @notice Reusable test initialization for `L2Genesis` tests.
abstract contract L2Genesis_TestInit is Test {
    L2Genesis.Input internal input;

    L2Genesis internal genesis;

    function setUp() public virtual {
        genesis = new L2Genesis();
    }

    function _assertProxyAdmin() internal view {
        assertEq(input.opChainProxyAdminOwner, IProxyAdmin(Predeploys.PROXY_ADMIN).owner());

        address proxyAdminImpl = Predeploys.predeployToCodeNamespace(Predeploys.PROXY_ADMIN);
        assertEq(
            input.opChainProxyAdminOwner,
            IProxyAdmin(proxyAdminImpl).owner(),
            "ProxyAdmin implementation owner should match expected"
        );
    }

    function _assertPredeploys() internal view {
        uint160 prefix = uint160(0x420) << 148;

        for (uint256 i = 0; i < Predeploys.PREDEPLOY_COUNT; i++) {
            address addr = address(prefix | uint160(i));
            if (Predeploys.notProxied(addr)) {
                continue;
            }

            assertGt(addr.code.length, 0);
            assertEq(Predeploys.PROXY_ADMIN, EIP1967Helper.getAdmin(addr));

            if (!Predeploys.isSupportedPredeploy(addr)) {
                continue;
            }

            address impl = Predeploys.predeployToCodeNamespace(addr);
            assertEq(impl, EIP1967Helper.getImplementation(addr));
            assertGt(impl.code.length, 0);
        }

        assertGt(Predeploys.WETH.code.length, 0);
    }

    function _assertFeeVaultsWithoutRevenueShare() internal view {
        _assertFeeVault(
            Predeploys.BASE_FEE_VAULT,
            input.baseFeeVaultRecipient,
            input.baseFeeVaultMinimumWithdrawalAmount,
            input.baseFeeVaultWithdrawalNetwork
        );
        _assertFeeVault(
            Predeploys.L1_FEE_VAULT,
            input.l1FeeVaultRecipient,
            input.l1FeeVaultMinimumWithdrawalAmount,
            input.l1FeeVaultWithdrawalNetwork
        );
        _assertFeeVault(
            Predeploys.SEQUENCER_FEE_WALLET,
            input.sequencerFeeVaultRecipient,
            input.sequencerFeeVaultMinimumWithdrawalAmount,
            input.sequencerFeeVaultWithdrawalNetwork
        );
        _assertFeeVault(
            Predeploys.OPERATOR_FEE_VAULT,
            input.operatorFeeVaultRecipient,
            input.operatorFeeVaultMinimumWithdrawalAmount,
            input.operatorFeeVaultWithdrawalNetwork
        );
    }

    function _assertFeeVault(
        address _vault,
        address _recipient,
        uint256 _minWithdrawalAmount,
        uint256 _withdrawalNetwork
    )
        internal
        view
    {
        IFeeVault vault = IFeeVault(payable(_vault));

        assertEq(vault.RECIPIENT(), _recipient);
        assertEq(vault.recipient(), _recipient);
        assertEq(vault.MIN_WITHDRAWAL_AMOUNT(), _minWithdrawalAmount);
        assertEq(vault.minWithdrawalAmount(), _minWithdrawalAmount);
        assertEq(uint256(vault.WITHDRAWAL_NETWORK()), _withdrawalNetwork);
        assertEq(uint256(vault.withdrawalNetwork()), _withdrawalNetwork);
    }

    function _assertFactories() internal view {
        IOptimismMintableERC20Factory erc20Factory =
            IOptimismMintableERC20Factory(payable(Predeploys.OPTIMISM_MINTABLE_ERC20_FACTORY));
        IOptimismMintableERC721Factory erc721Factory =
            IOptimismMintableERC721Factory(payable(Predeploys.OPTIMISM_MINTABLE_ERC721_FACTORY));

        assertEq(erc20Factory.bridge(), Predeploys.L2_STANDARD_BRIDGE);
        assertEq(erc721Factory.bridge(), Predeploys.L2_ERC721_BRIDGE);
        assertEq(erc721Factory.remoteChainID(), input.l1ChainID);
    }

    function _assertForks() internal view {
        IGasPriceOracle gasPriceOracle = IGasPriceOracle(payable(Predeploys.GAS_PRICE_ORACLE));
        assertTrue(gasPriceOracle.isEcotone());
        assertTrue(gasPriceOracle.isFjord());
        assertTrue(gasPriceOracle.isIsthmus());
        assertTrue(gasPriceOracle.isJovian());
    }
}

/// @title L2Genesis_Run_Test
/// @notice Tests the `run` function of the `L2Genesis` contract.
contract L2Genesis_Run_Test is L2Genesis_TestInit {
    function setUp() public override {
        super.setUp();

        input = L2Genesis.Input({
            l1ChainID: 1,
            l2ChainID: 2,
            l1CrossDomainMessengerProxy: payable(makeAddr("L1CrossDomainMessengerProxy")),
            l1StandardBridgeProxy: payable(makeAddr("L1StandardBridgeProxy")),
            l1ERC721BridgeProxy: payable(makeAddr("L1ERC721BridgeProxy")),
            opChainProxyAdminOwner: makeAddr("ProxyAdminOwner"),
            sequencerFeeVaultRecipient: makeAddr("SequencerFeeVaultRecipient"),
            sequencerFeeVaultMinimumWithdrawalAmount: 1,
            sequencerFeeVaultWithdrawalNetwork: uint256(Types.WithdrawalNetwork.L2),
            baseFeeVaultRecipient: makeAddr("BaseFeeVaultRecipient"),
            baseFeeVaultMinimumWithdrawalAmount: 1,
            baseFeeVaultWithdrawalNetwork: uint256(Types.WithdrawalNetwork.L2),
            l1FeeVaultRecipient: makeAddr("L1FeeVaultRecipient"),
            l1FeeVaultMinimumWithdrawalAmount: 1,
            l1FeeVaultWithdrawalNetwork: uint256(Types.WithdrawalNetwork.L2),
            operatorFeeVaultRecipient: makeAddr("OperatorFeeVaultRecipient"),
            operatorFeeVaultMinimumWithdrawalAmount: 1,
            operatorFeeVaultWithdrawalNetwork: uint256(Types.WithdrawalNetwork.L2),
            fork: uint256(LATEST_FORK),
            fundDevAccounts: true
        });
    }

    function test_run_succeeds() external {
        genesis.run(input);

        _assertProxyAdmin();
        _assertPredeploys();
        _assertFeeVaultsWithoutRevenueShare();
        _assertFactories();
        _assertForks();
    }
}

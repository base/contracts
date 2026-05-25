// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { IFeeVault } from "interfaces/L2/IFeeVault.sol";

import { Predeploys } from "src/libraries/Predeploys.sol";
import { FeeVault_Uncategorized_Test } from "test/L2/FeeVault.t.sol";
import { Types } from "src/libraries/Types.sol";

contract SequencerFeeVault_Uncategorized_Test is FeeVault_Uncategorized_Test {
    function setUp() public virtual override {
        super.setUp();
        recipient = deploy.cfg().sequencerFeeVaultRecipient();
        minWithdrawalAmount = deploy.cfg().sequencerFeeVaultMinimumWithdrawalAmount();
        feeVault = IFeeVault(payable(Predeploys.SEQUENCER_FEE_WALLET));
        withdrawalNetwork = Types.WithdrawalNetwork(deploy.cfg().sequencerFeeVaultWithdrawalNetwork());
    }

    function test_l1FeeWallet_matchesRecipient_succeeds() external view {
        assertEq(sequencerFeeVault.l1FeeWallet(), recipient);
    }
}

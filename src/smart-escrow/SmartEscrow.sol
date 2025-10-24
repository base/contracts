// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {AccessControlDefaultAdminRules} from "@openzeppelin/contracts/access/AccessControlDefaultAdminRules.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title SmartEscrow contract
///
/// @notice Contract to handle payment of OP tokens over a period of vesting with the ability to terminate the contract.
///         This contract is inspired by OpenZeppelin's VestingWallet contract, but had sufficiently different
///         requirements to where inheriting did not make sense.
contract SmartEscrow is AccessControlDefaultAdminRules {
    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Constants                                    ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice OP token contract.
    IERC20 public constant OP_TOKEN = IERC20(0x4200000000000000000000000000000000000042);

    /// @notice Role which can update benefactor address.
    bytes32 public constant BENEFACTOR_OWNER_ROLE = keccak256("smartescrow.roles.benefactorowner");

    /// @notice Role which can update beneficiary address.
    bytes32 public constant BENEFICIARY_OWNER_ROLE = keccak256("smartescrow.roles.beneficiaryowner");

    /// @notice Role which can update call terminate.
    bytes32 public constant TERMINATOR_ROLE = keccak256("smartescrow.roles.terminator");

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                   Immutables                                   ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Timestamp of the start of vesting period.
    uint256 public immutable start;

    /// @notice Timestamp of the cliff.
    uint256 public immutable cliffStart;

    /// @notice Timestamp of the end of the vesting period.
    uint256 public immutable end;

    /// @notice Period of time between each vesting event in seconds.
    uint256 public immutable vestingPeriod;

    /// @notice Number of OP tokens which vest at start time.
    uint256 public immutable initialTokens;

    /// @notice Number of OP tokens which vest upon each vesting event.
    uint256 public immutable vestingEventTokens;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                    Storage                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Address which receives funds back in case of contract termination.
    address public benefactor;

    /// @notice Address which receives tokens that have vested.
    address public beneficiary;

    /// @notice Number of OP tokens which have been released to the beneficiary.
    uint256 public released;

    /// @notice Flag for whether the contract is terminated or active.
    bool public contractTerminated;

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                     Events                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Event emitted when tokens are withdrawn from the contract.
    ///
    /// @param benefactor The address which received the withdrawn tokens.
    /// @param amount     The amount of tokens withdrawn.
    event TokensWithdrawn(address indexed benefactor, uint256 amount);

    /// @notice Event emitted when tokens are released to the beneficiary.
    ///
    /// @param beneficiary The address which received the released tokens.
    /// @param amount      The amount of tokens released.
    event TokensReleased(address indexed beneficiary, uint256 amount);

    /// @notice Event emitted when the benefactor is updated.
    ///
    /// @param oldBenefactor The address of the old benefactor.
    /// @param newBenefactor The address of the new benefactor.
    event BenefactorUpdated(address indexed oldBenefactor, address indexed newBenefactor);

    /// @notice Event emitted when the beneficiary is updated.
    ///
    /// @param oldBeneficiary The address of the old beneficiary.
    /// @param newBeneficiary The address of the new beneficiary.
    event BeneficiaryUpdated(address indexed oldBeneficiary, address indexed newBeneficiary);

    /// @notice Event emitted when the contract is terminated.
    event ContractTerminated();

    /// @notice Event emitted when the contract was terminated and is no longer.
    event ContractResumed();

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                     Errors                                     ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice The error is thrown when an address is not set.
    error AddressIsZeroAddress();

    /// @notice The error is thrown when the start timestamp is greater than the end timestamp.
    ///
    /// @param startTimestamp The provided start time of the contract.
    /// @param endTimestamp   The provided end time of the contract.
    error StartTimeAfterEndTime(uint256 startTimestamp, uint256 endTimestamp);

    /// @notice The error is thrown when the cliffStart timestamp is less than the start time.
    ///
    /// @param cliffStartTimestamp The provided start time of the contract.
    /// @param startTime           The start time
    error CliffStartTimeInvalid(uint256 cliffStartTimestamp, uint256 startTime);

    /// @notice The error is thrown when the cliffStart timestamp is greater than the end timestamp.
    ///
    /// @param cliffStartTimestamp The provided start time of the contract.
    /// @param endTimestamp        The provided end time of the contract.
    error CliffStartTimeAfterEndTime(uint256 cliffStartTimestamp, uint256 endTimestamp);

    /// @notice The error is thrown when the vesting period is zero.
    error VestingPeriodIsZeroSeconds();

    /// @notice The error is thrown when the number of vesting event tokens is zero.
    error VestingEventTokensIsZero();

    /// @notice The error is thrown when vesting period is longer than the contract duration.
    ///
    /// @param vestingPeriodSeconds The provided vesting period in seconds.
    error VestingPeriodExceedsContractDuration(uint256 vestingPeriodSeconds);

    /// @notice The error is thrown when the vesting period does not evenly divide the contract duration.
    ///
    /// @param vestingPeriodSeconds The provided vesting period in seconds.
    /// @param startTimestamp       The provided start time of the contract.
    /// @param endTimestamp         The provided end time of the contract.
    error UnevenVestingPeriod(uint256 vestingPeriodSeconds, uint256 startTimestamp, uint256 endTimestamp);

    /// @notice The error is thrown when the contract is terminated, when it should not be.
    error ContractIsTerminated();

    /// @notice The error is thrown when the contract is not terminated, when it should be.
    error ContractIsNotTerminated();

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                  Constructor                                   ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Set initial parameters.
    ///
    /// @param benefactor_           Address which receives tokens back in case of contract termination.
    /// @param beneficiary_          Address which receives tokens that have vested.
    /// @param benefactorOwner       Address which represents the benefactor entity.
    /// @param beneficiaryOwner      Address which represents the beneficiary entity.
    /// @param escrowOwner           Address which represents both the benefactor and the beneficiary entities.
    /// @param start_                Timestamp of the start of vesting period.
    /// @param cliffStart_           Timestamp when tokens start to vest (must be >= start_).
    /// @param end_                  Timestamp of the end of the vesting period.
    /// @param vestingPeriodSeconds_ Period of time between each vesting event in seconds.
    /// @param initialTokens_        Number of OP tokens which vest at start time.
    /// @param vestingEventTokens_   Number of OP tokens which vest upon each vesting event.
    constructor(
        address benefactor_,
        address beneficiary_,
        address benefactorOwner,
        address beneficiaryOwner,
        address escrowOwner,
        uint256 start_,
        uint256 cliffStart_,
        uint256 end_,
        uint256 vestingPeriodSeconds_,
        uint256 initialTokens_,
        uint256 vestingEventTokens_
    ) AccessControlDefaultAdminRules(5 days, escrowOwner) {
        if (
            benefactor_ == address(0) || beneficiary_ == address(0) || beneficiaryOwner == address(0)
                || benefactorOwner == address(0)
        ) {
            revert AddressIsZeroAddress();
        }
        if (start_ >= end_) revert StartTimeAfterEndTime({startTimestamp: start_, endTimestamp: end_});
        if (cliffStart_ < start_) revert CliffStartTimeInvalid({cliffStartTimestamp: cliffStart_, startTime: start_});
        if (cliffStart_ >= end_) {
            revert CliffStartTimeAfterEndTime({cliffStartTimestamp: cliffStart_, endTimestamp: end_});
        }
        if (vestingPeriodSeconds_ == 0) revert VestingPeriodIsZeroSeconds();
        if (vestingEventTokens_ == 0) revert VestingEventTokensIsZero();
        if ((end_ - start_) < vestingPeriodSeconds_) {
            revert VestingPeriodExceedsContractDuration({vestingPeriodSeconds: vestingPeriodSeconds_});
        }
        if ((end_ - start_) % vestingPeriodSeconds_ != 0) {
            revert UnevenVestingPeriod({
                vestingPeriodSeconds: vestingPeriodSeconds_, startTimestamp: start_, endTimestamp: end_
            });
        }

        benefactor = benefactor_;
        beneficiary = beneficiary_;
        start = start_;
        cliffStart = cliffStart_;
        end = end_;
        vestingPeriod = vestingPeriodSeconds_;
        initialTokens = initialTokens_;
        vestingEventTokens = vestingEventTokens_;

        _grantRole({role: BENEFACTOR_OWNER_ROLE, account: benefactorOwner});
        _grantRole({role: TERMINATOR_ROLE, account: benefactorOwner});
        _grantRole({role: BENEFICIARY_OWNER_ROLE, account: beneficiaryOwner});
        _grantRole({role: TERMINATOR_ROLE, account: beneficiaryOwner});
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               External Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Terminates the contract if called by address with TERMINATOR_ROLE.
    ///
    /// @dev Releases any vested token to the beneficiary before terminating.
    function terminate() external onlyRole(TERMINATOR_ROLE) {
        release();
        contractTerminated = true;
        emit ContractTerminated();
    }

    /// @notice Resumes the contract on the original vesting schedule.
    ///
    /// @dev Must be called by address with DEFAULT_ADMIN_ROLE role.
    function resume() external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (!contractTerminated) revert ContractIsNotTerminated();
        contractTerminated = false;
        emit ContractResumed();
    }

    /// @notice Allow benefactor owner to update benefactor address.
    ///
    /// @dev This method does not adjust the BENEFACTOR_OWNER_ROLE. Ensure to pair calling this with a role change by
    ///      DEFAULT_ADMIN if this is the desired outcome.
    ///
    /// @param newBenefactor New benefactor address.
    function updateBenefactor(address newBenefactor) external onlyRole(BENEFACTOR_OWNER_ROLE) {
        if (newBenefactor == address(0)) revert AddressIsZeroAddress();
        address oldBenefactor = benefactor;
        if (oldBenefactor != newBenefactor) {
            benefactor = newBenefactor;
            emit BenefactorUpdated({oldBenefactor: oldBenefactor, newBenefactor: newBenefactor});
        }
    }

    /// @notice Allow beneficiary owner to update beneficiary address.
    ///
    /// @dev This method does not adjust the BENEFICIARY_OWNER_ROLE. Ensure to pair calling this with a role change by
    ///      DEFAULT_ADMIN if this is the desired outcome.
    ///
    /// @param newBeneficiary New beneficiary address.
    function updateBeneficiary(address newBeneficiary) external onlyRole(BENEFICIARY_OWNER_ROLE) {
        if (newBeneficiary == address(0)) revert AddressIsZeroAddress();
        address oldBeneficiary = beneficiary;
        if (oldBeneficiary != newBeneficiary) {
            beneficiary = newBeneficiary;
            emit BeneficiaryUpdated({oldBeneficiary: oldBeneficiary, newBeneficiary: newBeneficiary});
        }
    }

    /// @notice Allow withdrawal of remaining tokens to benefactor address if contract is terminated.
    function withdrawUnvestedTokens() external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (!contractTerminated) revert ContractIsNotTerminated();
        uint256 amount = OP_TOKEN.balanceOf({account: address(this)});
        if (amount > 0) {
            OP_TOKEN.transfer({to: benefactor, amount: amount});
            emit TokensWithdrawn({benefactor: benefactor, amount: amount});
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                                Public Functions                                ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Release OP tokens that have already vested.
    function release() public {
        if (contractTerminated) revert ContractIsTerminated();
        uint256 amount = releasable();
        if (amount > 0) {
            released += amount;
            OP_TOKEN.transfer({to: beneficiary, amount: amount});
            emit TokensReleased({beneficiary: beneficiary, amount: amount});
        }
    }

    /// @notice Getter for the amount of releasable OP.
    function releasable() public view returns (uint256) {
        return vestedAmount({timestamp: block.timestamp}) - released;
    }

    /// @notice Calculates the amount of OP that has already vested.
    ///
    /// @param timestamp The timestamp to at which to get the vested amount
    function vestedAmount(uint256 timestamp) public view returns (uint256) {
        return _vestingSchedule({timestamp: timestamp});
    }

    //////////////////////////////////////////////////////////////////////////////////////
    ///                               Internal Functions                               ///
    //////////////////////////////////////////////////////////////////////////////////////

    /// @notice Returns the amount vested as a function of time.
    ///
    /// @param timestamp The timestamp to at which to get the vested amount
    function _vestingSchedule(uint256 timestamp) internal view returns (uint256) {
        if (timestamp < cliffStart) {
            return 0;
        } else if (timestamp > end) {
            return OP_TOKEN.balanceOf({account: address(this)}) + released;
        } else {
            return initialTokens + ((timestamp - start) / vestingPeriod) * vestingEventTokens;
        }
    }
}

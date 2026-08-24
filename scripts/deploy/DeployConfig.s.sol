// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { console } from "lib/forge-std/src/console.sol";
import { Script } from "lib/forge-std/src/Script.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";

/// @title DeployConfig
/// @notice Represents the configuration required to deploy the system, read from a JSON file.
contract DeployConfig is Script {
    using stdJson for string;

    address public baseFeeVaultRecipient;
    address public batchSenderAddress;
    address public finalSystemOwner;
    address public l1FeeVaultRecipient;
    address public nitroValidator;
    address public operatorFeeVaultRecipient;
    address public p2pSequencerAddress;
    address public proxyAdminOwner;
    address public sequencerFeeVaultRecipient;
    address public sp1Verifier;
    address public superchainConfigGuardian;
    address public superchainConfigIncidentResponder;
    address public teeChallenger;
    address public teeProposer;

    bool public fundDevAccounts;
    bool public useUpgradedFork;

    bytes32 public multiproofConfigHash;
    bytes32 public multiproofGenesisOutputRoot;
    bytes32 public teeImageHash;
    bytes32 public zkAggregationHash;
    bytes32 public zkRangeHash;

    uint32 public basefeeScalar;
    uint32 public blobbasefeeScalar;

    uint256 public baseFeeVaultMinimumWithdrawalAmount;
    uint256 public baseFeeVaultWithdrawalNetwork;
    uint256 public disputeGameFinalityDelaySeconds;
    uint256 public delayedWETHWithdrawalDelay;
    uint256 public l1ChainId;
    uint256 public l1FeeVaultMinimumWithdrawalAmount;
    uint256 public l1FeeVaultWithdrawalNetwork;
    uint256 public l2BlockTime;
    uint256 public l2ChainId;
    uint256 public l2GenesisBlockGasLimit;
    uint256 public l2GenesisBlockNumber;
    uint256 public l2GenesisTimestamp;
    uint256 public l2OutputOracleStartingBlockNumber;
    uint256 public l2OutputOracleStartingTimestamp;
    uint256 public multiproofBlockInterval;
    uint256 public multiproofGameType;
    uint256 public multiproofGenesisBlockNumber;
    uint256 public multiproofIntermediateBlockInterval;
    uint256 public operatorFeeVaultMinimumWithdrawalAmount;
    uint256 public operatorFeeVaultWithdrawalNetwork;
    uint256 public proofMaturityDelaySeconds;
    uint256 public protocolVersionsInitialMinimumVersion;
    uint256 public respectedGameType;
    uint256 public sequencerFeeVaultMinimumWithdrawalAmount;
    uint256 public sequencerFeeVaultWithdrawalNetwork;

    uint64[] internal _protocolVersionsInitialSchedule;

    /// @notice The chain's existing hardfork activation timestamps, used to seed `ProtocolVersions`.
    /// @dev Chains with no upgrade history leave this unset and start with an empty registry.
    function protocolVersionsInitialSchedule() public view returns (uint64[] memory) {
        return _protocolVersionsInitialSchedule;
    }

    function read(string memory _path) public {
        console.log("DeployConfig: reading file %s", _path);
        string memory _json = vm.readFile(_path);

        baseFeeVaultRecipient = _json.readAddress("$.baseFeeVaultRecipient");
        batchSenderAddress = _json.readAddress("$.batchSenderAddress");
        finalSystemOwner = _json.readAddress("$.finalSystemOwner");
        l1FeeVaultRecipient = _json.readAddress("$.l1FeeVaultRecipient");
        nitroValidator = _json.readAddress("$.nitroValidator");
        operatorFeeVaultRecipient = _json.readAddress("$.operatorFeeVaultRecipient");
        p2pSequencerAddress = _json.readAddress("$.p2pSequencerAddress");
        proxyAdminOwner = _json.readAddress("$.proxyAdminOwner");
        sequencerFeeVaultRecipient = _json.readAddress("$.sequencerFeeVaultRecipient");
        sp1Verifier = _json.readAddress("$.sp1Verifier");
        superchainConfigGuardian = _json.readAddress("$.superchainConfigGuardian");
        superchainConfigIncidentResponder = _json.readAddress("$.superchainConfigIncidentResponder");
        teeChallenger = _json.readAddress("$.teeChallenger");
        teeProposer = _json.readAddress("$.teeProposer");

        fundDevAccounts = _json.readBoolOr("$.fundDevAccounts", false);

        multiproofConfigHash = _json.readBytes32("$.multiproofConfigHash");
        multiproofGenesisOutputRoot = _json.readBytes32("$.multiproofGenesisOutputRoot");
        teeImageHash = _json.readBytes32("$.teeImageHash");
        zkAggregationHash = _json.readBytes32("$.zkAggregationHash");
        zkRangeHash = _json.readBytes32("$.zkRangeHash");

        basefeeScalar = uint32(_json.readUint("$.gasPriceOracleBaseFeeScalar"));
        blobbasefeeScalar = uint32(_json.readUint("$.gasPriceOracleBlobBaseFeeScalar"));

        baseFeeVaultMinimumWithdrawalAmount = _json.readUint("$.baseFeeVaultMinimumWithdrawalAmount");
        baseFeeVaultWithdrawalNetwork = _json.readUint("$.baseFeeVaultWithdrawalNetwork");
        disputeGameFinalityDelaySeconds = _json.readUint("$.disputeGameFinalityDelaySeconds");
        delayedWETHWithdrawalDelay = _json.readUint("$.delayedWETHWithdrawalDelay");
        l1ChainId = _json.readUint("$.l1ChainId");
        l1FeeVaultMinimumWithdrawalAmount = _json.readUint("$.l1FeeVaultMinimumWithdrawalAmount");
        l1FeeVaultWithdrawalNetwork = _json.readUint("$.l1FeeVaultWithdrawalNetwork");
        l2BlockTime = _json.readUintOr("$.l2BlockTime", 0);
        l2ChainId = _json.readUint("$.l2ChainId");
        l2GenesisBlockGasLimit = _json.readUint("$.l2GenesisBlockGasLimit");
        l2OutputOracleStartingBlockNumber = _json.readUint("$.l2OutputOracleStartingBlockNumber");
        l2OutputOracleStartingTimestamp = _json.readUint("$.l2OutputOracleStartingTimestamp");
        l2GenesisBlockNumber = _json.readUintOr("$.l2GenesisBlockNumber", 0);
        l2GenesisTimestamp = _json.readUintOr("$.l2GenesisTimestamp", 0);
        multiproofBlockInterval = _json.readUintOr("$.multiproofBlockInterval", 100);
        multiproofGameType = _json.readUintOr("$.multiproofGameType", 621);
        multiproofGenesisBlockNumber = _json.readUintOr("$.multiproofGenesisBlockNumber", 0);
        multiproofIntermediateBlockInterval = _json.readUintOr("$.multiproofIntermediateBlockInterval", 10);
        operatorFeeVaultMinimumWithdrawalAmount = _json.readUint("$.operatorFeeVaultMinimumWithdrawalAmount");
        operatorFeeVaultWithdrawalNetwork = _json.readUint("$.operatorFeeVaultWithdrawalNetwork");
        proofMaturityDelaySeconds = _json.readUintOr("$.proofMaturityDelaySeconds", 0);
        _readProtocolVersionsInitialMinimumVersion(_json);
        respectedGameType = _json.readUintOr("$.respectedGameType", 0);
        sequencerFeeVaultMinimumWithdrawalAmount = _json.readUint("$.sequencerFeeVaultMinimumWithdrawalAmount");
        sequencerFeeVaultWithdrawalNetwork = _json.readUint("$.sequencerFeeVaultWithdrawalNetwork");

        _readProtocolVersionsInitialSchedule(_json);
    }

    function _readProtocolVersionsInitialMinimumVersion(string memory _json) internal {
        uint256 minimumVersion = _json.readUintOr("$.protocolVersionsInitialMinimumVersion", 0);
        require(minimumVersion <= type(uint128).max, "DeployConfig: initial minimum protocol version exceeds uint128");
        protocolVersionsInitialMinimumVersion = minimumVersion;
    }

    /// @dev Read separately so a rerun of `read` replaces the previous schedule rather than appending
    ///      to it, and so an out-of-range timestamp fails loudly instead of silently truncating.
    function _readProtocolVersionsInitialSchedule(string memory _json) internal {
        uint256[] memory schedule = _json.readUintArrayOr("$.protocolVersionsInitialSchedule", new uint256[](0));

        delete _protocolVersionsInitialSchedule;
        for (uint256 i = 0; i < schedule.length; i++) {
            require(schedule[i] <= type(uint64).max, "DeployConfig: initial schedule timestamp exceeds uint64");
            _protocolVersionsInitialSchedule.push(uint64(schedule[i]));
        }
    }

    /// @notice Allow the `useUpgradedFork` config to be overridden in testing environments
    /// @dev When true, the forked system WILL be upgraded in setUp().
    ///      When false, the forked system WILL NOT be upgraded in setUp().
    ///      This function does nothing when not testing in a forked environment.
    ///      Generally the only time you should call this function is if you want to
    ///      call the upgrade script in the test itself, rather than have the upgraded
    ///      system be deployed in setUp().
    function setUseUpgradedFork(bool _useUpgradedFork) public {
        useUpgradedFork = _useUpgradedFork;
    }
}

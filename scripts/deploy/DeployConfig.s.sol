// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Script } from "lib/forge-std/src/Script.sol";
import { console } from "lib/forge-std/src/console.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";
import { Process } from "scripts/libraries/Process.sol";

/// @title DeployConfig
/// @notice Represents the configuration required to deploy the system, read from a JSON file.
contract DeployConfig is Script {
    using stdJson for string;

    string internal _json;

    address public baseFeeVaultRecipient;
    address public batchInboxAddress;
    address public batchSenderAddress;
    address public finalSystemOwner;
    address public l1FeeVaultRecipient;
    address public l2OutputOracleProposer;
    address public l2OutputOracleChallenger;
    address public nitroEnclaveVerifier;
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

    bytes32 public faultGameGenesisOutputRoot;
    bytes32 public multiproofConfigHash;
    bytes32 public multiproofGenesisOutputRoot;
    bytes32 public teeImageHash;
    bytes32 public zkAggregationHash;
    bytes32 public zkRangeHash;

    int256 internal _l2OutputOracleStartingTimestamp;

    uint32 public basefeeScalar;
    uint32 public blobbasefeeScalar;

    uint256 public baseFeeVaultMinimumWithdrawalAmount;
    uint256 public baseFeeVaultWithdrawalNetwork;
    uint256 public disputeGameFinalityDelaySeconds;
    uint256 public faultGameAbsolutePrestate;
    uint256 public faultGameClockExtension;
    uint256 public faultGameGenesisBlock;
    uint256 public faultGameMaxClockDuration;
    uint256 public faultGameMaxDepth;
    uint256 public faultGameSplitDepth;
    uint256 public faultGameV2ClockExtension;
    uint256 public faultGameV2MaxClockDuration;
    uint256 public faultGameV2MaxGameDepth;
    uint256 public faultGameV2SplitDepth;
    uint256 public faultGameWithdrawalDelay;
    uint256 public l1ChainId;
    uint256 public l1FeeVaultMinimumWithdrawalAmount;
    uint256 public l1FeeVaultWithdrawalNetwork;
    uint256 public l2ChainId;
    uint256 public l2GenesisBlockGasLimit;
    uint256 public l2OutputOracleStartingBlockNumber;
    uint256 public multiproofBlockInterval;
    uint256 public multiproofGameType;
    uint256 public multiproofGenesisBlockNumber;
    uint256 public multiproofIntermediateBlockInterval;
    uint256 public operatorFeeVaultMinimumWithdrawalAmount;
    uint256 public operatorFeeVaultWithdrawalNetwork;
    uint256 public preimageOracleChallengePeriod;
    uint256 public preimageOracleMinProposalSize;
    uint256 public proofMaturityDelaySeconds;
    uint256 public respectedGameType;
    uint256 public sequencerFeeVaultMinimumWithdrawalAmount;
    uint256 public sequencerFeeVaultWithdrawalNetwork;
    uint256 public systemConfigStartBlock;

    function read(string memory _path) public {
        console.log("DeployConfig: reading file %s", _path);
        try vm.readFile(_path) returns (string memory data_) {
            _json = data_;
        } catch {
            revert(string.concat("DeployConfig: cannot find deploy config file at ", _path));
        }

        baseFeeVaultRecipient = _json.readAddress("$.baseFeeVaultRecipient");
        batchInboxAddress = _json.readAddress("$.batchInboxAddress");
        batchSenderAddress = _json.readAddress("$.batchSenderAddress");
        finalSystemOwner = _json.readAddress("$.finalSystemOwner");
        l1FeeVaultRecipient = _json.readAddress("$.l1FeeVaultRecipient");
        l2OutputOracleChallenger = _json.readAddress("$.l2OutputOracleChallenger");
        l2OutputOracleProposer = _json.readAddress("$.l2OutputOracleProposer");
        nitroEnclaveVerifier = _json.readAddress("$.nitroEnclaveVerifier");
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

        faultGameGenesisOutputRoot = _json.readBytes32("$.faultGameGenesisOutputRoot");
        multiproofConfigHash = _json.readBytes32("$.multiproofConfigHash");
        multiproofGenesisOutputRoot = _json.readBytes32("$.multiproofGenesisOutputRoot");
        teeImageHash = _json.readBytes32("$.teeImageHash");
        zkAggregationHash = _json.readBytes32("$.zkAggregationHash");
        zkRangeHash = _json.readBytes32("$.zkRangeHash");

        _l2OutputOracleStartingTimestamp = _json.readInt("$.l2OutputOracleStartingTimestamp");

        basefeeScalar = uint32(_json.readUint("$.gasPriceOracleBaseFeeScalar"));
        blobbasefeeScalar = uint32(_json.readUint("$.gasPriceOracleBlobBaseFeeScalar"));

        baseFeeVaultMinimumWithdrawalAmount = _json.readUint("$.baseFeeVaultMinimumWithdrawalAmount");
        baseFeeVaultWithdrawalNetwork = _json.readUint("$.baseFeeVaultWithdrawalNetwork");
        disputeGameFinalityDelaySeconds = _json.readUint("$.disputeGameFinalityDelaySeconds");
        faultGameAbsolutePrestate = _json.readUint("$.faultGameAbsolutePrestate");
        faultGameClockExtension = _json.readUint("$.faultGameClockExtension");
        faultGameGenesisBlock = _json.readUint("$.faultGameGenesisBlock");
        faultGameMaxClockDuration = _json.readUint("$.faultGameMaxClockDuration");
        faultGameMaxDepth = _json.readUint("$.faultGameMaxDepth");
        faultGameSplitDepth = _json.readUint("$.faultGameSplitDepth");
        faultGameV2ClockExtension = _json.readUintOr("$.faultGameV2ClockExtension", 10800);
        faultGameV2MaxClockDuration = _json.readUintOr("$.faultGameV2MaxClockDuration", 302400);
        faultGameV2MaxGameDepth = _json.readUintOr("$.faultGameV2MaxGameDepth", 73);
        faultGameV2SplitDepth = _json.readUintOr("$.faultGameV2SplitDepth", 30);
        faultGameWithdrawalDelay = _json.readUint("$.faultGameWithdrawalDelay");
        l1ChainId = _json.readUint("$.l1ChainId");
        l1FeeVaultMinimumWithdrawalAmount = _json.readUint("$.l1FeeVaultMinimumWithdrawalAmount");
        l1FeeVaultWithdrawalNetwork = _json.readUint("$.l1FeeVaultWithdrawalNetwork");
        l2ChainId = _json.readUint("$.l2ChainId");
        l2GenesisBlockGasLimit = _json.readUint("$.l2GenesisBlockGasLimit");
        l2OutputOracleStartingBlockNumber = _json.readUint("$.l2OutputOracleStartingBlockNumber");
        multiproofBlockInterval = _json.readUintOr("$.multiproofBlockInterval", 100);
        multiproofGameType = _json.readUintOr("$.multiproofGameType", 621);
        multiproofGenesisBlockNumber = _json.readUintOr("$.multiproofGenesisBlockNumber", 0);
        multiproofIntermediateBlockInterval = _json.readUintOr("$.multiproofIntermediateBlockInterval", 10);
        operatorFeeVaultMinimumWithdrawalAmount = _json.readUint("$.operatorFeeVaultMinimumWithdrawalAmount");
        operatorFeeVaultWithdrawalNetwork = _json.readUint("$.operatorFeeVaultWithdrawalNetwork");
        preimageOracleChallengePeriod = _json.readUint("$.preimageOracleChallengePeriod");
        preimageOracleMinProposalSize = _json.readUint("$.preimageOracleMinProposalSize");
        proofMaturityDelaySeconds = _json.readUintOr("$.proofMaturityDelaySeconds", 0);
        respectedGameType = _json.readUintOr("$.respectedGameType", 0);
        sequencerFeeVaultMinimumWithdrawalAmount = _json.readUint("$.sequencerFeeVaultMinimumWithdrawalAmount");
        sequencerFeeVaultWithdrawalNetwork = _json.readUint("$.sequencerFeeVaultWithdrawalNetwork");
        systemConfigStartBlock = _json.readUint("$.systemConfigStartBlock");
    }

    function l1StartingBlockTag() public returns (bytes32) {
        try vm.parseJsonBytes32(_json, "$.l1StartingBlockTag") returns (bytes32 tag_) {
            return tag_;
        } catch { }
        try vm.parseJsonString(_json, "$.l1StartingBlockTag") returns (string memory tag_) {
            return _getBlockByTag(tag_);
        } catch { }
        try vm.parseJsonUint(_json, "$.l1StartingBlockTag") returns (uint256 tag_) {
            return _getBlockByTag(vm.toString(tag_));
        } catch { }
        revert("DeployConfig: l1StartingBlockTag missing or not a bytes32/string/uint256");
    }

    function l2OutputOracleStartingTimestamp() public returns (uint256) {
        if (_l2OutputOracleStartingTimestamp < 0) {
            bytes32 tag = l1StartingBlockTag();
            string memory cmd = string.concat("cast block ", vm.toString(tag), " --json | jq .timestamp");
            string memory res = Process.bash(cmd);
            return stdJson.readUint(res, "");
        }
        return uint256(_l2OutputOracleStartingTimestamp);
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

    function _getBlockByTag(string memory _tag) internal returns (bytes32) {
        string memory cmd = string.concat("cast block ", _tag, " --json | jq .hash");
        return stdJson.readBytes32(Process.bash(cmd), "");
    }
}

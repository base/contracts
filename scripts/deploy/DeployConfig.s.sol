// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Script } from "lib/forge-std/src/Script.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";
import { Process } from "scripts/libraries/Process.sol";

/// @title DeployConfig
/// @notice Represents the configuration required to deploy the system, read from a JSON file.
contract DeployConfig is Script {
    using stdJson for string;

    string internal _json;

    address public finalSystemOwner;
    address public superchainConfigGuardian;
    address public superchainConfigIncidentResponder;
    uint256 public l1ChainID;
    uint256 public l2ChainID;
    address public p2pSequencerAddress;
    address public batchInboxAddress;
    address public batchSenderAddress;
    int256 internal _l2OutputOracleStartingTimestamp;
    uint256 public l2OutputOracleStartingBlockNumber;
    address public l2OutputOracleProposer;
    address public l2OutputOracleChallenger;
    bool public fundDevAccounts;
    address public proxyAdminOwner;
    address public baseFeeVaultRecipient;
    uint256 public baseFeeVaultMinimumWithdrawalAmount;
    uint256 public baseFeeVaultWithdrawalNetwork;
    address public l1FeeVaultRecipient;
    uint256 public l1FeeVaultMinimumWithdrawalAmount;
    uint256 public l1FeeVaultWithdrawalNetwork;
    address public sequencerFeeVaultRecipient;
    uint256 public sequencerFeeVaultMinimumWithdrawalAmount;
    uint256 public sequencerFeeVaultWithdrawalNetwork;
    address public operatorFeeVaultRecipient;
    uint256 public operatorFeeVaultMinimumWithdrawalAmount;
    uint256 public operatorFeeVaultWithdrawalNetwork;
    address public governanceTokenOwner;
    uint256 public l2GenesisBlockGasLimit;
    uint32 public basefeeScalar;
    uint32 public blobbasefeeScalar;
    bool public enableGovernance;
    uint256 public faultGameAbsolutePrestate;
    uint256 public faultGameGenesisBlock;
    bytes32 public faultGameGenesisOutputRoot;
    uint256 public faultGameMaxDepth;
    uint256 public faultGameSplitDepth;
    uint256 public faultGameClockExtension;
    uint256 public faultGameMaxClockDuration;
    uint256 public faultGameWithdrawalDelay;
    uint256 public preimageOracleMinProposalSize;
    uint256 public preimageOracleChallengePeriod;
    uint256 public systemConfigStartBlock;
    uint256 public proofMaturityDelaySeconds;
    uint256 public disputeGameFinalityDelaySeconds;
    uint256 public respectedGameType;

    uint256 public faultGameV2MaxGameDepth;
    uint256 public faultGameV2SplitDepth;
    uint256 public faultGameV2ClockExtension;
    uint256 public faultGameV2MaxClockDuration;

    bytes32 public teeImageHash;
    bytes32 public multiproofConfigHash;
    uint256 public multiproofGameType;
    address public teeProposer;
    address public teeChallenger;
    bytes32 public zkRangeHash;
    bytes32 public zkAggregationHash;
    address public nitroEnclaveVerifier;
    bytes32 public multiproofGenesisOutputRoot;
    uint256 public multiproofGenesisBlockNumber;
    uint256 public multiproofBlockInterval;
    uint256 public multiproofIntermediateBlockInterval;
    address public sp1Verifier;

    bool public useInterop;
    bool public useUpgradedFork;
    bytes32 public devFeatureBitmap;

    bool public useRevenueShare;
    address public chainFeesRecipient;
    /// @notice This is not read from JSON because it is hardcoded in the deployer. It is overwritten with its setter
    ///         for testing.
    address public l1FeesDepositor;

    function read(string memory _path) public {
        console.log("DeployConfig: reading file %s", _path);
        try vm.readFile(_path) returns (string memory data_) {
            _json = data_;
        } catch {
            revert(string.concat("DeployConfig: cannot find deploy config file at ", _path));
        }

        finalSystemOwner = _json.readAddress("$.finalSystemOwner");
        superchainConfigGuardian = _json.readAddress("$.superchainConfigGuardian");
        superchainConfigIncidentResponder = _json.readAddressOr("$.superchainConfigIncidentResponder", address(0));
        l1ChainID = _json.readUint("$.l1ChainID");
        l2ChainID = _json.readUint("$.l2ChainID");

        p2pSequencerAddress = _json.readAddress("$.p2pSequencerAddress");
        batchInboxAddress = _json.readAddress("$.batchInboxAddress");
        batchSenderAddress = _json.readAddress("$.batchSenderAddress");
        _l2OutputOracleStartingTimestamp = _json.readInt("$.l2OutputOracleStartingTimestamp");
        l2OutputOracleStartingBlockNumber = _json.readUint("$.l2OutputOracleStartingBlockNumber");
        l2OutputOracleProposer = _json.readAddress("$.l2OutputOracleProposer");
        l2OutputOracleChallenger = _json.readAddress("$.l2OutputOracleChallenger");
        fundDevAccounts = _json.readBoolOr("$.fundDevAccounts", false);
        proxyAdminOwner = _json.readAddress("$.proxyAdminOwner");
        baseFeeVaultRecipient = _json.readAddress("$.baseFeeVaultRecipient");
        baseFeeVaultMinimumWithdrawalAmount = _json.readUint("$.baseFeeVaultMinimumWithdrawalAmount");
        baseFeeVaultWithdrawalNetwork = _json.readUint("$.baseFeeVaultWithdrawalNetwork");
        l1FeeVaultRecipient = _json.readAddress("$.l1FeeVaultRecipient");
        l1FeeVaultMinimumWithdrawalAmount = _json.readUint("$.l1FeeVaultMinimumWithdrawalAmount");
        l1FeeVaultWithdrawalNetwork = _json.readUint("$.l1FeeVaultWithdrawalNetwork");
        sequencerFeeVaultRecipient = _json.readAddress("$.sequencerFeeVaultRecipient");
        sequencerFeeVaultMinimumWithdrawalAmount = _json.readUint("$.sequencerFeeVaultMinimumWithdrawalAmount");
        sequencerFeeVaultWithdrawalNetwork = _json.readUint("$.sequencerFeeVaultWithdrawalNetwork");
        operatorFeeVaultRecipient = _json.readAddress("$.operatorFeeVaultRecipient");
        operatorFeeVaultMinimumWithdrawalAmount = _json.readUint("$.operatorFeeVaultMinimumWithdrawalAmount");
        operatorFeeVaultWithdrawalNetwork = _json.readUint("$.operatorFeeVaultWithdrawalNetwork");
        governanceTokenOwner = _json.readAddress("$.governanceTokenOwner");
        l2GenesisBlockGasLimit = _json.readUint("$.l2GenesisBlockGasLimit");
        basefeeScalar = uint32(_json.readUintOr("$.gasPriceOracleBaseFeeScalar", 1368));
        blobbasefeeScalar = uint32(_json.readUintOr("$.gasPriceOracleBlobBaseFeeScalar", 810949));

        enableGovernance = _json.readBoolOr("$.enableGovernance", false);
        systemConfigStartBlock = _json.readUint("$.systemConfigStartBlock");

        proofMaturityDelaySeconds = _json.readUintOr("$.proofMaturityDelaySeconds", 0);
        disputeGameFinalityDelaySeconds = _json.readUintOr("$.disputeGameFinalityDelaySeconds", 0);
        respectedGameType = _json.readUintOr("$.respectedGameType", 0);

        faultGameAbsolutePrestate = _json.readUint("$.faultGameAbsolutePrestate");
        faultGameMaxDepth = _json.readUint("$.faultGameMaxDepth");
        faultGameSplitDepth = _json.readUint("$.faultGameSplitDepth");
        faultGameClockExtension = _json.readUint("$.faultGameClockExtension");
        faultGameMaxClockDuration = _json.readUint("$.faultGameMaxClockDuration");
        faultGameGenesisBlock = _json.readUint("$.faultGameGenesisBlock");
        faultGameGenesisOutputRoot = _json.readBytes32("$.faultGameGenesisOutputRoot");
        faultGameWithdrawalDelay = _json.readUint("$.faultGameWithdrawalDelay");

        preimageOracleMinProposalSize = _json.readUint("$.preimageOracleMinProposalSize");
        preimageOracleChallengePeriod = _json.readUint("$.preimageOracleChallengePeriod");

        useInterop = _json.readBoolOr("$.useInterop", false);
        devFeatureBitmap = _json.readBytes32Or("$.devFeatureBitmap", bytes32(0));
        useRevenueShare = _json.readBoolOr("$.useRevenueShare", false);
        chainFeesRecipient = _json.readAddressOr("$.chainFeesRecipient", address(0));
        faultGameV2MaxGameDepth = _json.readUintOr("$.faultGameV2MaxGameDepth", 73);
        faultGameV2SplitDepth = _json.readUintOr("$.faultGameV2SplitDepth", 30);
        faultGameV2ClockExtension = _json.readUintOr("$.faultGameV2ClockExtension", 10800);
        faultGameV2MaxClockDuration = _json.readUintOr("$.faultGameV2MaxClockDuration", 302400);
        teeImageHash = _json.readBytes32Or("$.teeImageHash", bytes32(0));
        multiproofConfigHash = _json.readBytes32Or("$.multiproofConfigHash", bytes32(0));
        multiproofGameType = _json.readUintOr("$.multiproofGameType", 621);
        teeProposer = _json.readAddressOr("$.teeProposer", address(0));
        teeChallenger = _json.readAddressOr("$.teeChallenger", address(0));
        zkRangeHash = _json.readBytes32Or("$.zkRangeHash", bytes32(0));
        zkAggregationHash = _json.readBytes32Or("$.zkAggregationHash", bytes32(0));
        nitroEnclaveVerifier = _json.readAddressOr("$.nitroEnclaveVerifier", address(0));
        multiproofGenesisOutputRoot = _json.readBytes32Or("$.multiproofGenesisOutputRoot", bytes32(uint256(1)));
        multiproofGenesisBlockNumber = _json.readUintOr("$.multiproofGenesisBlockNumber", 0);
        multiproofBlockInterval = _json.readUintOr("$.multiproofBlockInterval", 100);
        multiproofIntermediateBlockInterval = _json.readUintOr("$.multiproofIntermediateBlockInterval", 10);
        sp1Verifier = _json.readAddressOr("$.sp1Verifier", address(0));
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
        revert(
            "DeployConfig: l1StartingBlockTag must be a bytes32, string or uint256 or cannot fetch l1StartingBlockTag"
        );
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

    function setUseInterop(bool _useInterop) public {
        useInterop = _useInterop;
    }

    function setUseRevenueShare(bool _useRevenueShare) public {
        useRevenueShare = _useRevenueShare;
    }

    function setL1FeesDepositor(address _l1FeesDepositor) public {
        l1FeesDepositor = _l1FeesDepositor;
    }

    function setChainFeesRecipient(address _chainFeesRecipient) public {
        chainFeesRecipient = _chainFeesRecipient;
    }

    function setDevFeatureBitmap(bytes32 _devFeatureBitmap) public {
        devFeatureBitmap = _devFeatureBitmap;
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
        string memory cmd = string.concat("cast block ", _tag, " --json | jq -r .hash");
        bytes memory res = bytes(Process.bash(cmd));
        return abi.decode(res, (bytes32));
    }
}

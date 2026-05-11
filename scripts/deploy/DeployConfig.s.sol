// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Script } from "lib/forge-std/src/Script.sol";
import { console2 as console } from "lib/forge-std/src/console2.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";
import { Process } from "scripts/libraries/Process.sol";

/// @title DeployConfig
/// @notice Represents the configuration required to deploy the system. It is expected
///         to read the file from JSON. A future improvement would be to have fallback
///         values if they are not defined in the JSON themselves.
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

    // V2 Dispute Game Configuration
    uint256 public faultGameV2MaxGameDepth;
    uint256 public faultGameV2SplitDepth;
    uint256 public faultGameV2ClockExtension;
    uint256 public faultGameV2MaxClockDuration;

    // Multiproof Configuration
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
            require(false, string.concat("DeployConfig: cannot find deploy config file at ", _path));
        }

        finalSystemOwner = stdJson.readAddress(_json, "$.finalSystemOwner");
        superchainConfigGuardian = stdJson.readAddress(_json, "$.superchainConfigGuardian");
        superchainConfigIncidentResponder = _readOr(_json, "$.superchainConfigIncidentResponder", address(0));
        l1ChainID = stdJson.readUint(_json, "$.l1ChainID");
        l2ChainID = stdJson.readUint(_json, "$.l2ChainID");

        p2pSequencerAddress = stdJson.readAddress(_json, "$.p2pSequencerAddress");
        batchInboxAddress = stdJson.readAddress(_json, "$.batchInboxAddress");
        batchSenderAddress = stdJson.readAddress(_json, "$.batchSenderAddress");
        _l2OutputOracleStartingTimestamp = stdJson.readInt(_json, "$.l2OutputOracleStartingTimestamp");
        l2OutputOracleStartingBlockNumber = stdJson.readUint(_json, "$.l2OutputOracleStartingBlockNumber");
        l2OutputOracleProposer = stdJson.readAddress(_json, "$.l2OutputOracleProposer");
        l2OutputOracleChallenger = stdJson.readAddress(_json, "$.l2OutputOracleChallenger");
        fundDevAccounts = _readOr(_json, "$.fundDevAccounts", false);
        proxyAdminOwner = stdJson.readAddress(_json, "$.proxyAdminOwner");
        baseFeeVaultRecipient = stdJson.readAddress(_json, "$.baseFeeVaultRecipient");
        baseFeeVaultMinimumWithdrawalAmount = stdJson.readUint(_json, "$.baseFeeVaultMinimumWithdrawalAmount");
        baseFeeVaultWithdrawalNetwork = stdJson.readUint(_json, "$.baseFeeVaultWithdrawalNetwork");
        l1FeeVaultRecipient = stdJson.readAddress(_json, "$.l1FeeVaultRecipient");
        l1FeeVaultMinimumWithdrawalAmount = stdJson.readUint(_json, "$.l1FeeVaultMinimumWithdrawalAmount");
        l1FeeVaultWithdrawalNetwork = stdJson.readUint(_json, "$.l1FeeVaultWithdrawalNetwork");
        sequencerFeeVaultRecipient = stdJson.readAddress(_json, "$.sequencerFeeVaultRecipient");
        sequencerFeeVaultMinimumWithdrawalAmount = stdJson.readUint(_json, "$.sequencerFeeVaultMinimumWithdrawalAmount");
        sequencerFeeVaultWithdrawalNetwork = stdJson.readUint(_json, "$.sequencerFeeVaultWithdrawalNetwork");
        operatorFeeVaultRecipient = stdJson.readAddress(_json, "$.operatorFeeVaultRecipient");
        operatorFeeVaultMinimumWithdrawalAmount = stdJson.readUint(_json, "$.operatorFeeVaultMinimumWithdrawalAmount");
        operatorFeeVaultWithdrawalNetwork = stdJson.readUint(_json, "$.operatorFeeVaultWithdrawalNetwork");
        governanceTokenOwner = stdJson.readAddress(_json, "$.governanceTokenOwner");
        l2GenesisBlockGasLimit = stdJson.readUint(_json, "$.l2GenesisBlockGasLimit");
        basefeeScalar = uint32(_readOr(_json, "$.gasPriceOracleBaseFeeScalar", 1368));
        blobbasefeeScalar = uint32(_readOr(_json, "$.gasPriceOracleBlobBaseFeeScalar", 810949));

        enableGovernance = _readOr(_json, "$.enableGovernance", false);
        systemConfigStartBlock = stdJson.readUint(_json, "$.systemConfigStartBlock");

        proofMaturityDelaySeconds = _readOr(_json, "$.proofMaturityDelaySeconds", 0);
        disputeGameFinalityDelaySeconds = _readOr(_json, "$.disputeGameFinalityDelaySeconds", 0);
        respectedGameType = _readOr(_json, "$.respectedGameType", 0);

        faultGameAbsolutePrestate = stdJson.readUint(_json, "$.faultGameAbsolutePrestate");
        faultGameMaxDepth = stdJson.readUint(_json, "$.faultGameMaxDepth");
        faultGameSplitDepth = stdJson.readUint(_json, "$.faultGameSplitDepth");
        faultGameClockExtension = stdJson.readUint(_json, "$.faultGameClockExtension");
        faultGameMaxClockDuration = stdJson.readUint(_json, "$.faultGameMaxClockDuration");
        faultGameGenesisBlock = stdJson.readUint(_json, "$.faultGameGenesisBlock");
        faultGameGenesisOutputRoot = stdJson.readBytes32(_json, "$.faultGameGenesisOutputRoot");
        faultGameWithdrawalDelay = stdJson.readUint(_json, "$.faultGameWithdrawalDelay");

        preimageOracleMinProposalSize = stdJson.readUint(_json, "$.preimageOracleMinProposalSize");
        preimageOracleChallengePeriod = stdJson.readUint(_json, "$.preimageOracleChallengePeriod");

        useInterop = _readOr(_json, "$.useInterop", false);
        devFeatureBitmap = bytes32(_readOr(_json, "$.devFeatureBitmap", 0));
        useRevenueShare = _readOr(_json, "$.useRevenueShare", false);
        chainFeesRecipient = _readOr(_json, "$.chainFeesRecipient", address(0));
        faultGameV2MaxGameDepth = _readOr(_json, "$.faultGameV2MaxGameDepth", 73);
        faultGameV2SplitDepth = _readOr(_json, "$.faultGameV2SplitDepth", 30);
        faultGameV2ClockExtension = _readOr(_json, "$.faultGameV2ClockExtension", 10800);
        faultGameV2MaxClockDuration = _readOr(_json, "$.faultGameV2MaxClockDuration", 302400);
        teeImageHash = bytes32(_readOr(_json, "$.teeImageHash", 0));
        multiproofConfigHash = bytes32(_readOr(_json, "$.multiproofConfigHash", 0));
        multiproofGameType = _readOr(_json, "$.multiproofGameType", 621);
        teeProposer = _readOr(_json, "$.teeProposer", address(0));
        teeChallenger = _readOr(_json, "$.teeChallenger", address(0));
        zkRangeHash = bytes32(_readOr(_json, "$.zkRangeHash", 0));
        zkAggregationHash = bytes32(_readOr(_json, "$.zkAggregationHash", 0));
        nitroEnclaveVerifier = _readOr(_json, "$.nitroEnclaveVerifier", address(0));
        multiproofGenesisOutputRoot = bytes32(_readOr(_json, "$.multiproofGenesisOutputRoot", uint256(1)));
        multiproofGenesisBlockNumber = _readOr(_json, "$.multiproofGenesisBlockNumber", 0);
        multiproofBlockInterval = _readOr(_json, "$.multiproofBlockInterval", 100);
        multiproofIntermediateBlockInterval = _readOr(_json, "$.multiproofIntermediateBlockInterval", 10);
        sp1Verifier = _readOr(_json, "$.sp1Verifier", address(0));
    }

    function l1StartingBlockTag() public returns (bytes32) {
        try vm.parseJsonBytes32(_json, "$.l1StartingBlockTag") returns (bytes32 tag_) {
            return tag_;
        } catch {
            try vm.parseJsonString(_json, "$.l1StartingBlockTag") returns (string memory tag_) {
                return _getBlockByTag(tag_);
            } catch {
                try vm.parseJsonUint(_json, "$.l1StartingBlockTag") returns (uint256 tag_) {
                    return _getBlockByTag(vm.toString(tag_));
                } catch { }
            }
        }
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

    /// @notice Allow the `useInterop` config to be overridden in testing environments
    function setUseInterop(bool _useInterop) public {
        useInterop = _useInterop;
    }

    /// @notice Allow the `useRevenueShare` config to be overridden in testing environments
    function setUseRevenueShare(bool _useRevenueShare) public {
        useRevenueShare = _useRevenueShare;
    }

    /// @notice Allow the `l1FeesDepositor` config to be overridden in testing environments
    function setL1FeesDepositor(address _l1FeesDepositor) public {
        l1FeesDepositor = _l1FeesDepositor;
    }

    /// @notice Allow the `chainFeesRecipient` config to be overridden in testing environments
    function setChainFeesRecipient(address _chainFeesRecipient) public {
        chainFeesRecipient = _chainFeesRecipient;
    }

    /// @notice Allow the `devFeatureBitmap` config to be overridden in testing environments
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

    function _readOr(string memory _jsonInp, string memory _key, bool _defaultValue) internal view returns (bool) {
        return _jsonInp.readBoolOr(_key, _defaultValue);
    }

    function _readOr(string memory _jsonInp, string memory _key, uint256 _defaultValue)
        internal
        view
        returns (uint256)
    {
        return (vm.keyExistsJson(_jsonInp, _key) && !_isNull(_json, _key)) ? _jsonInp.readUint(_key) : _defaultValue;
    }

    function _readOr(string memory _jsonInp, string memory _key, address _defaultValue)
        internal
        view
        returns (address)
    {
        return _jsonInp.readAddressOr(_key, _defaultValue);
    }

    function _isNull(string memory _jsonInp, string memory _key) internal pure returns (bool) {
        string memory value = _jsonInp.readString(_key);
        return (keccak256(bytes(value)) == keccak256(bytes("null")));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {Script} from "forge-std/Script.sol";
import {console2 as console} from "forge-std/console2.sol";
import {stdJson} from "forge-std/StdJson.sol";

// TEE contracts
import {CertManager} from "@nitro-validator/CertManager.sol";
import {SystemConfigGlobal} from "../src/tee/SystemConfigGlobal.sol";
import {TEEVerifier} from "../src/tee/TEEVerifier.sol";

// Dispute game
import {AggregateVerifier} from "../src/AggregateVerifier.sol";
import {IVerifier} from "../src/interfaces/IVerifier.sol";
import {IAnchorStateRegistry} from "optimism/interfaces/dispute/IAnchorStateRegistry.sol";
import {IDelayedWETH} from "optimism/interfaces/dispute/IDelayedWETH.sol";
import {IDisputeGame} from "optimism/interfaces/dispute/IDisputeGame.sol";
import {GameType, Hash} from "optimism/src/dispute/lib/Types.sol";

// Mocks
import {MockVerifier} from "../src/mocks/MockVerifier.sol";

// Proxy - using OpenZeppelin for simpler deployment
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

/// @title MockAnchorStateRegistry
/// @notice Minimal mock for testing - stores anchor state and factory reference
/// @dev We use a mock instead of the real AnchorStateRegistry because:
///      1. The real contract requires deploying the entire Optimism L1 stack
///         (SystemConfig, SuperchainConfig, ProxyAdmin, Guardian roles, etc.)
///      2. The real contract has "stack too deep" compilation issues that require
///         special compiler settings (via-ir) which significantly slow builds
///      3. For TEE prover testing, we only need getAnchorRoot() and setAnchorState()
contract MockAnchorStateRegistry {
    Hash public anchorRoot;
    uint256 public anchorL2BlockNumber;
    address public factory;
    GameType public respectedGameType;

    function initialize(address _factory, Hash _anchorRoot, uint256 _anchorL2BlockNumber, GameType _gameType) external {
        factory = _factory;
        anchorRoot = _anchorRoot;
        anchorL2BlockNumber = _anchorL2BlockNumber;
        respectedGameType = _gameType;
    }

    // This is the key function AggregateVerifier calls
    function getAnchorRoot() external view returns (Hash, uint256) {
        return (anchorRoot, anchorL2BlockNumber);
    }

    function disputeGameFactory() external view returns (address) {
        return factory;
    }

    function setRespectedGameType(GameType _gameType) external {
        respectedGameType = _gameType;
    }

    /// @notice Update the anchor state (for testing purposes)
    function setAnchorState(Hash _anchorRoot, uint256 _anchorL2BlockNumber) external {
        anchorRoot = _anchorRoot;
        anchorL2BlockNumber = _anchorL2BlockNumber;
    }

    // Stub implementations that AggregateVerifier may call
    function isGameRegistered(IDisputeGame) external pure returns (bool) {
        return true;
    }

    function isGameBlacklisted(IDisputeGame) external pure returns (bool) {
        return false;
    }

    function isGameRetired(IDisputeGame) external pure returns (bool) {
        return false;
    }

    function isGameRespected(IDisputeGame) external pure returns (bool) {
        return true;
    }
}

/// @title MockDelayedWETH
/// @notice Minimal mock for testing - implements the IDelayedWETH interface
/// @dev For testing purposes only. The real DelayedWETH handles bond deposits and withdrawals.
contract MockDelayedWETH {
    /// @notice Accepts ETH deposits (no-op for testing)
    function deposit() external payable {}

    /// @notice Mock unlock - no-op for testing
    function unlock(address, uint256) external {}

    /// @notice Mock withdraw - transfers ETH back
    function withdraw(address recipient, uint256 amount) external {
        payable(recipient).transfer(amount);
    }

    /// @notice Allow contract to receive ETH
    receive() external payable {}
}

// Import the REAL DisputeGameFactory
import {DisputeGameFactory} from "optimism/src/dispute/DisputeGameFactory.sol";

/// @title MinimalProxyAdmin
/// @notice Minimal contract to satisfy DisputeGameFactory's proxy admin check
/// @dev DisputeGameFactory.initialize() requires msg.sender == proxyAdmin() or proxyAdminOwner()
///      We deploy this minimal contract and set it as the proxy admin via vm.store
contract MinimalProxyAdmin {
    address public owner;

    constructor(address _owner) {
        owner = _owner;
    }
}

/// @title DeployAllForTesting
/// @notice Deploys everything needed for e2e testing, using mocks for optimism contracts.
contract DeployAllForTesting is Script {
    using stdJson for string;

    uint256 public constant BLOCK_INTERVAL = 100;
    uint256 public constant INIT_BOND = 0.001 ether;

    // Config struct to reduce stack variables
    struct DeployConfig {
        address owner;
        bytes32 teeImageHash;
        address teeProposer;
        GameType gameType;
        uint256 gameTypeRaw;
        bytes32 genesisOutputRoot;
        uint256 genesisBlockNumber;
        bytes32 configHash;
    }

    // Deployed addresses
    address public certManager;
    address public systemConfigGlobalProxy;
    address public teeVerifier;
    address public disputeGameFactory;
    address public mockAnchorRegistry;
    address public mockDelayedWETH;
    address public aggregateVerifier;

    function _loadConfig() internal view returns (DeployConfig memory cfg) {
        string memory configPath = vm.envOr("DEPLOY_CONFIG_PATH", string("deploy-config/sepolia.json"));
        string memory config = vm.readFile(configPath);

        cfg.owner = config.readAddress(".finalSystemOwner");
        cfg.teeImageHash = config.readBytes32(".teeImageHash");
        cfg.teeProposer = config.readAddressOr(".teeProposer", cfg.owner);
        cfg.gameTypeRaw = config.readUintOr(".gameType", 621);
        cfg.gameType = GameType.wrap(uint32(cfg.gameTypeRaw));
        cfg.genesisOutputRoot = config.readBytes32Or(".genesisOutputRoot", bytes32(uint256(1)));
        cfg.genesisBlockNumber = config.readUintOr(".genesisBlockNumber", 0);
        cfg.configHash = config.readBytes32Or(".configHash", bytes32(0));
    }

    function run() public {
        DeployConfig memory cfg = _loadConfig();

        console.log("=== Deploying Complete Test Infrastructure ===");
        console.log("Chain ID:", block.chainid);
        console.log("Owner:", cfg.owner);
        console.log("TEE Proposer:", cfg.teeProposer);
        console.log("Game Type:", cfg.gameTypeRaw);

        vm.startBroadcast();

        _deployTEEContracts(cfg.owner);
        _deployInfrastructure(cfg);
        _deployAggregateVerifier(cfg);

        vm.stopBroadcast();

        _printSummary(cfg.teeImageHash, cfg.gameTypeRaw, cfg.configHash);
        _writeOutput();
    }

    function _deployTEEContracts(address owner) internal {
        // 1. CertManager
        certManager = address(new CertManager());
        console.log("CertManager:", certManager);

        // 2. SystemConfigGlobal with proxy
        address scgImpl = address(new SystemConfigGlobal(CertManager(certManager)));
        systemConfigGlobalProxy = address(
            new TransparentUpgradeableProxy(
                scgImpl,
                address(0xdead), // Non-upgradeable for testing
                abi.encodeCall(SystemConfigGlobal.initialize, (owner, owner))
            )
        );
        console.log("SystemConfigGlobal:", systemConfigGlobalProxy);

        // 3. TEEVerifier
        teeVerifier = address(new TEEVerifier(SystemConfigGlobal(systemConfigGlobalProxy)));
        console.log("TEEVerifier:", teeVerifier);
    }

    // Constant from Optimism's Constants.sol - the storage slot for proxy admin
    bytes32 constant PROXY_OWNER_ADDRESS = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;

    function _deployInfrastructure(DeployConfig memory cfg) internal {
        // 4. REAL DisputeGameFactory (behind proxy)
        // The constructor calls _disableInitializers(), so we must use a proxy
        address factoryImpl = address(new DisputeGameFactory());

        // Deploy a minimal proxy admin that returns cfg.owner as owner
        MinimalProxyAdmin proxyAdmin = new MinimalProxyAdmin(cfg.owner);

        // Deploy proxy - but DON'T initialize yet (empty initData)
        // We need to set the proxy admin storage slot first
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            factoryImpl,
            address(proxyAdmin), // Use our MinimalProxyAdmin as the admin
            "" // Don't call initialize yet
        );

        // Set the PROXY_OWNER_ADDRESS slot on the proxy so initialize() passes the access check
        vm.store(address(proxy), PROXY_OWNER_ADDRESS, bytes32(uint256(uint160(address(proxyAdmin)))));

        // Now initialize - caller is checked against proxyAdmin or proxyAdminOwner
        DisputeGameFactory(address(proxy)).initialize(cfg.owner);

        disputeGameFactory = address(proxy);
        console.log("DisputeGameFactory (REAL):", disputeGameFactory);

        // 5. Mock AnchorStateRegistry - still mocked because:
        //    - The real one needs SystemConfig, SuperchainConfig, etc.
        //    - We only need getAnchorRoot() for the prover
        //    - Manual setAnchorState() is useful for testing
        MockAnchorStateRegistry asr = new MockAnchorStateRegistry();
        mockAnchorRegistry = address(asr);
        asr.initialize(disputeGameFactory, Hash.wrap(cfg.genesisOutputRoot), cfg.genesisBlockNumber, cfg.gameType);
        console.log("AnchorStateRegistry (mock):", mockAnchorRegistry);
    }

    function _deployAggregateVerifier(DeployConfig memory cfg) internal {
        // 6. Mock ZK Verifier
        address zkVerifier = address(new MockVerifier());
        console.log("MockVerifier (ZK):", zkVerifier);

        // 6.5. Mock DelayedWETH for bond handling
        mockDelayedWETH = address(new MockDelayedWETH());
        console.log("MockDelayedWETH:", mockDelayedWETH);

        // 7. AggregateVerifier
        aggregateVerifier = address(
            new AggregateVerifier(
                cfg.gameType,
                IAnchorStateRegistry(mockAnchorRegistry),
                IDelayedWETH(payable(mockDelayedWETH)),
                IVerifier(teeVerifier),
                IVerifier(zkVerifier),
                cfg.teeImageHash,
                bytes32(0), // zkImageHash (unused for testing)
                cfg.configHash,
                cfg.teeProposer,
                8453, // l2ChainId (Base mainnet)
                BLOCK_INTERVAL
            )
        );
        console.log("AggregateVerifier:", aggregateVerifier);
        console.log("ConfigHash:", vm.toString(cfg.configHash));

        // 8. Register AggregateVerifier with the real factory
        DisputeGameFactory(disputeGameFactory).setImplementation(cfg.gameType, IDisputeGame(aggregateVerifier));
        DisputeGameFactory(disputeGameFactory).setInitBond(cfg.gameType, INIT_BOND);
        console.log("Registered AggregateVerifier with factory");
    }

    function _printSummary(bytes32 teeImageHash, uint256 gameType, bytes32 configHash) internal view {
        console.log("\n========================================");
        console.log("         DEPLOYMENT COMPLETE");
        console.log("========================================");
        console.log("\nTEE Contracts:");
        console.log("  CertManager:", certManager);
        console.log("  SystemConfigGlobal:", systemConfigGlobalProxy);
        console.log("  TEEVerifier:", teeVerifier);
        console.log("\nInfrastructure:");
        console.log("  DisputeGameFactory (real):", disputeGameFactory);
        console.log("  AnchorStateRegistry (mock):", mockAnchorRegistry);
        console.log("  DelayedWETH (mock):", mockDelayedWETH);
        console.log("\nGame:");
        console.log("  AggregateVerifier:", aggregateVerifier);
        console.log("  Game Type:", gameType);
        console.log("  ConfigHash:", vm.toString(configHash));
        console.log("========================================");
        console.log("\nNEXT STEP - Register dev signer:");
        console.log("cast send", systemConfigGlobalProxy);
        console.log('  "addDevSigner(address,bytes32)" <SIGNER_ADDRESS>');
        console.log(" ", vm.toString(teeImageHash));
    }

    function _writeOutput() internal {
        string memory outPath = string.concat("deployments/", vm.toString(block.chainid), "-all.json");
        string memory output = string.concat(
            '{"CertManager":"',
            vm.toString(certManager),
            '","SystemConfigGlobal":"',
            vm.toString(systemConfigGlobalProxy),
            '","TEEVerifier":"',
            vm.toString(teeVerifier),
            '","DisputeGameFactory":"',
            vm.toString(disputeGameFactory),
            '","AnchorStateRegistry":"',
            vm.toString(mockAnchorRegistry),
            '","AggregateVerifier":"',
            vm.toString(aggregateVerifier),
            '"}'
        );
        vm.writeFile(outPath, output);
        console.log("\nDeployment saved to:", outPath);
    }
}

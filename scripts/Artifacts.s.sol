// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { console2 as console } from "lib/forge-std/src/console2.sol";
import { stdJson } from "lib/forge-std/src/StdJson.sol";
import { Vm } from "lib/forge-std/src/Vm.sol";
import { Predeploys } from "src/libraries/Predeploys.sol";
import { Config } from "scripts/libraries/Config.sol";
import { ForgeArtifacts } from "scripts/libraries/ForgeArtifacts.sol";

/// @title Artifacts
/// @notice Useful for accessing deployment artifacts from within scripts.
///         When a contract is deployed, call the `save` function to write its name and
///         contract address to disk. Inspired by `forge-deploy`.
contract Artifacts {
    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    error DeploymentDoesNotExist(string);
    error InvalidDeployment(string);

    mapping(string => address payable) internal _namedDeployments;
    mapping(bytes32 => address payable) internal _predeploys;
    string internal deploymentOutfile;

    function setUp() public virtual {
        deploymentOutfile = Config.deploymentOutfile();
        console.log("Writing artifact to %s", deploymentOutfile);
        ForgeArtifacts.ensurePath(deploymentOutfile);

        uint256 chainId = Config.chainID();
        console.log("Connected to network with chainid %s", chainId);

        _predeploys[keccak256("L2CrossDomainMessenger")] = payable(Predeploys.L2_CROSS_DOMAIN_MESSENGER);
        _predeploys[keccak256("L2ToL1MessagePasser")] = payable(Predeploys.L2_TO_L1_MESSAGE_PASSER);
        _predeploys[keccak256("L2StandardBridge")] = payable(Predeploys.L2_STANDARD_BRIDGE);
        _predeploys[keccak256("L2ERC721Bridge")] = payable(Predeploys.L2_ERC721_BRIDGE);
        _predeploys[keccak256("SequencerFeeVault")] = payable(Predeploys.SEQUENCER_FEE_WALLET);
        _predeploys[keccak256("OptimismMintableERC20Factory")] = payable(Predeploys.OPTIMISM_MINTABLE_ERC20_FACTORY);
        _predeploys[keccak256("OptimismMintableERC721Factory")] = payable(Predeploys.OPTIMISM_MINTABLE_ERC721_FACTORY);
        _predeploys[keccak256("L1Block")] = payable(Predeploys.L1_BLOCK_ATTRIBUTES);
        _predeploys[keccak256("GasPriceOracle")] = payable(Predeploys.GAS_PRICE_ORACLE);
        _predeploys[keccak256("L1MessageSender")] = payable(Predeploys.L1_MESSAGE_SENDER);
        _predeploys[keccak256("WETH")] = payable(Predeploys.WETH);
        _predeploys[keccak256("LegacyERC20ETH")] = payable(Predeploys.LEGACY_ERC20_ETH);
        _predeploys[keccak256("ProxyAdmin")] = payable(Predeploys.PROXY_ADMIN);
        _predeploys[keccak256("BaseFeeVault")] = payable(Predeploys.BASE_FEE_VAULT);
        _predeploys[keccak256("L1FeeVault")] = payable(Predeploys.L1_FEE_VAULT);
        _predeploys[keccak256("OperatorFeeVault")] = payable(Predeploys.OPERATOR_FEE_VAULT);
        _predeploys[keccak256("SchemaRegistry")] = payable(Predeploys.SCHEMA_REGISTRY);
        _predeploys[keccak256("EAS")] = payable(Predeploys.EAS);
    }

    /// @notice Returns the address of a deployment. Also handles the predeploys.
    /// @param _name The name of the deployment.
    /// @return The address of the deployment. May be `address(0)` if the deployment does not
    ///         exist.
    function getAddress(string memory _name) public view returns (address payable) {
        address payable existing = _namedDeployments[_name];
        if (existing != address(0)) {
            return existing;
        }
        return _predeploys[keccak256(bytes(_name))];
    }

    /// @notice Returns the address of a deployment and reverts if the deployment
    ///         does not exist.
    /// @return The address of the deployment.
    function mustGetAddress(string memory _name) public view returns (address payable) {
        address payable addr = getAddress(_name);
        if (addr == address(0)) {
            revert DeploymentDoesNotExist(_name);
        }
        return addr;
    }

    /// @notice Appends a deployment to disk as a JSON deploy artifact.
    /// @param _name The name of the deployment.
    /// @param _deployed The address of the deployment.
    function save(string memory _name, address _deployed) public {
        console.log("Saving %s: %s", _name, _deployed);
        if (bytes(_name).length == 0) {
            revert InvalidDeployment("EmptyName");
        }
        address payable existing = _namedDeployments[_name];
        if (existing != address(0)) {
            console.log("Warning: Deployment already exists for %s.", _name);
            console.log("Overwriting %s with %s", existing, _deployed);
        }

        _namedDeployments[_name] = payable(_deployed);
        vm.writeJson({ json: stdJson.serialize("", _name, _deployed), path: deploymentOutfile });
        vm.label(_deployed, _name);
    }
}

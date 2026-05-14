// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { IDelayedWETH } from "interfaces/L1/proofs/IDelayedWETH.sol";
import { IAnchorStateRegistry } from "interfaces/L1/proofs/IAnchorStateRegistry.sol";
import { IVerifier } from "interfaces/L1/proofs/IVerifier.sol";
import { INitroEnclaveVerifier } from "interfaces/L1/proofs/tee/INitroEnclaveVerifier.sol";
import { ITEEProverRegistry } from "interfaces/L1/proofs/tee/ITEEProverRegistry.sol";
import { ISP1Verifier } from "interfaces/L1/proofs/zk/ISP1Verifier.sol";
import { IAddressManager } from "interfaces/legacy/IAddressManager.sol";
import { IProxyAdmin } from "interfaces/universal/IProxyAdmin.sol";
import { IDisputeGameFactory } from "interfaces/L1/proofs/IDisputeGameFactory.sol";
import { IOptimismPortal2 } from "interfaces/L1/IOptimismPortal2.sol";
import { ISystemConfig } from "interfaces/L1/ISystemConfig.sol";
import { IL1CrossDomainMessenger } from "interfaces/L1/IL1CrossDomainMessenger.sol";
import { IL1ERC721Bridge } from "interfaces/L1/IL1ERC721Bridge.sol";
import { IL1StandardBridge } from "interfaces/L1/IL1StandardBridge.sol";
import { IOptimismMintableERC20Factory } from "interfaces/universal/IOptimismMintableERC20Factory.sol";
import { IETHLockbox } from "interfaces/L1/IETHLockbox.sol";

import { GameType, Proposal } from "src/libraries/bridge/Types.sol";
import { Duration, Claim } from "src/libraries/bridge/LibUDT.sol";

library Types {
    /// @notice Represents the roles that can be set when deploying a standard OP Stack chain.
    struct Roles {
        address opChainProxyAdminOwner;
        address systemConfigOwner;
        address batcher;
        address unsafeBlockSigner;
    }

    /// @notice The full set of inputs to deploy a new OP Stack chain.
    struct DeployInput {
        Roles roles;
        uint32 basefeeScalar;
        uint32 blobBasefeeScalar;
        uint256 l2ChainId;
        Proposal startingAnchorRoot;
        string saltMixer;
        uint64 gasLimit;
        GameType disputeGameType;
    }

    /// @notice The full set of outputs from deploying a new OP Stack chain.
    struct DeployOutput {
        IProxyAdmin opChainProxyAdmin;
        IAddressManager addressManager;
        IL1ERC721Bridge l1ERC721BridgeProxy;
        ISystemConfig systemConfigProxy;
        IOptimismMintableERC20Factory optimismMintableERC20FactoryProxy;
        IL1StandardBridge l1StandardBridgeProxy;
        IL1CrossDomainMessenger l1CrossDomainMessengerProxy;
        IETHLockbox ethLockboxProxy;
        IOptimismPortal2 optimismPortalProxy;
        IDisputeGameFactory disputeGameFactoryProxy;
        IAnchorStateRegistry anchorStateRegistryProxy;
        IDelayedWETH delayedWETHPermissionlessGameProxy;
        IVerifier aggregateVerifier;
        ITEEProverRegistry teeProverRegistryProxy;
        IVerifier teeVerifier;
        IVerifier zkVerifier;
        INitroEnclaveVerifier nitroEnclaveVerifier;
        ISP1Verifier sp1Verifier;
    }

    /// @notice The latest implementation contracts for the OP Stack.
    struct Implementations {
        address superchainConfigImpl;
        address l1ERC721BridgeImpl;
        address optimismPortalImpl;
        address ethLockboxImpl;
        address systemConfigImpl;
        address optimismMintableERC20FactoryImpl;
        address l1CrossDomainMessengerImpl;
        address l1StandardBridgeImpl;
        address disputeGameFactoryImpl;
        address anchorStateRegistryImpl;
        address delayedWETHImpl;
        address aggregateVerifierImpl;
        address teeProverRegistryImpl;
        address teeVerifierImpl;
        address zkVerifierImpl;
    }

    /// @notice Maps an L2 chain ID to the standard L1 batch inbox address.
    /// @dev The convention is `versionByte || keccak256(bytes32(chainId))[:19]`.
    function chainIdToBatchInboxAddress(uint256 _l2ChainId) internal pure returns (address) {
        bytes1 versionByte = 0x00;
        bytes32 hashedChainId = keccak256(bytes.concat(bytes32(_l2ChainId)));
        bytes19 first19Bytes = bytes19(hashedChainId);
        return address(uint160(bytes20(bytes.concat(versionByte, first19Bytes))));
    }
}

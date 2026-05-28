// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { CommonTest } from "test/setup/CommonTest.sol";
import { Preinstalls } from "src/libraries/Preinstalls.sol";
import { Bytes } from "src/libraries/Bytes.sol";
import { IEIP712 } from "interfaces/universal/IEIP712.sol";

/// @title Preinstalls_TestInit
/// @notice Reusable test initialization for `Preinstalls` tests.
abstract contract Preinstalls_TestInit is CommonTest {
    uint256 internal constant PERMIT2_CHAIN_ID_OFFSET = 6945;
    uint256 internal constant PERMIT2_DOMAIN_SEPARATOR_OFFSET = 6983;
    uint256 internal constant PERMIT2_DEPLOYMENT_CHAIN_ID = 901;

    function assertPreinstall(address _addr, bytes memory _code) internal view {
        assertPreinstall(_addr, _code, block.chainid);
    }

    function assertPreinstall(address _addr, bytes memory _code, uint256 _chainId) internal view {
        bytes memory deployedCode = _addr.code;

        assertNotEq(_code.length, 0, "must have code");
        assertNotEq(deployedCode.length, 0, "deployed preinstall account must have code");
        assertEq(deployedCode, _code, "equal code must be deployed");
        assertEq(Preinstalls.getDeployedCode(_addr, _chainId), _code, "deployed-code getter must match");
        assertNotEq(Preinstalls.getName(_addr), "", "must have a name");
        if (_addr != Preinstalls.DeterministicDeploymentProxy) {
            assertEq(vm.getNonce(_addr), 1, "preinstall account must have 1 nonce");
        }
    }

    function assertPermit2Code(
        uint256 _chainId,
        bytes32 _expectedDomainSeparator
    )
        internal
        pure
        returns (bytes memory code_)
    {
        code_ = Preinstalls.getPermit2Code(_chainId);
        assertNotEq(code_.length, 0, "must have code");
        assertEq(uint256(bytes32(Bytes.slice(code_, PERMIT2_CHAIN_ID_OFFSET, 32))), _chainId, "expecting chain ID");
        assertEq(
            bytes32(Bytes.slice(code_, PERMIT2_DOMAIN_SEPARATOR_OFFSET, 32)),
            _expectedDomainSeparator,
            "expecting domain separator"
        );
    }
}

/// @title Preinstalls_GetPermit2Code_Test
/// @notice Tests the `getPermit2Code` function of the `Preinstalls` library.
contract Preinstalls_GetPermit2Code_Test is Preinstalls_TestInit {
    function test_getPermit2Code_templating_works() external pure {
        assertPermit2Code(1234, bytes32(0x6cda538cafce36292a6ef27740629597f85f6716f5694d26d5c59fc1d07cfd95));

        bytes memory defaultCode =
            assertPermit2Code(1, bytes32(0x866a5aba21966af95d6c7ab78eb2b2fc913915c28be3b9aa07cc04ff903e3f28));
        assertEq(defaultCode, Preinstalls.Permit2TemplateCode, "template is using chain ID 1");
    }
}

/// @title Preinstalls_Uncategorized_Test
/// @notice General tests that are not testing any function directly of the `Preinstalls` contract
///         or are testing multiple functions at once.
contract Preinstalls_Uncategorized_Test is Preinstalls_TestInit {
    /// @notice The domain separator commits to the chainid of the chain
    function test_preinstall_permit2DomainSeparator_works() external view {
        bytes32 domainSeparator = IEIP712(Preinstalls.Permit2).DOMAIN_SEPARATOR();
        bytes32 typeHash =
            keccak256(abi.encodePacked("EIP712Domain(string name,uint256 chainId,address verifyingContract)"));
        bytes32 nameHash = keccak256(abi.encodePacked("Permit2"));
        uint256 chainId = block.chainid;
        bytes32 expectedDomainSeparator = keccak256(abi.encode(typeHash, nameHash, chainId, Preinstalls.Permit2));
        assertEq(domainSeparator, expectedDomainSeparator, "Domain separator mismatch");
        assertEq(chainId, PERMIT2_DEPLOYMENT_CHAIN_ID); // uses devnet config
        assertEq(domainSeparator, bytes32(0x48deb34b39fb4b41f5c195008940d5ef510cdd7853eba5807b2fa08dfd586475));
        // Warning the Permit2 domain separator as cached in the DeployPermit2.sol bytecode is
        // incorrect.
    }

    function test_preinstall_multicall3_succeeds() external view {
        assertPreinstall(Preinstalls.MultiCall3, Preinstalls.MultiCall3Code);
    }

    function test_preinstall_create2Deployer_succeeds() external view {
        assertPreinstall(Preinstalls.Create2Deployer, Preinstalls.Create2DeployerCode);
    }

    function test_preinstall_safev130_succeeds() external view {
        assertPreinstall(Preinstalls.Safe_v130, Preinstalls.Safe_v130Code);
    }

    function test_preinstall_safeL2v130_succeeds() external view {
        assertPreinstall(Preinstalls.SafeL2_v130, Preinstalls.SafeL2_v130Code);
    }

    function test_preinstall_multisendCallOnlyv130_succeeds() external view {
        assertPreinstall(Preinstalls.MultiSendCallOnly_v130, Preinstalls.MultiSendCallOnly_v130Code);
    }

    function test_preinstall_safeSingletonFactory_succeeds() external view {
        assertPreinstall(Preinstalls.SafeSingletonFactory, Preinstalls.SafeSingletonFactoryCode);
    }

    function test_preinstall_deterministicDeploymentProxy_succeeds() external view {
        assertPreinstall(Preinstalls.DeterministicDeploymentProxy, Preinstalls.DeterministicDeploymentProxyCode);
    }

    function test_preinstall_multisendv130_succeeds() external view {
        assertPreinstall(Preinstalls.MultiSend_v130, Preinstalls.MultiSend_v130Code);
    }

    function test_preinstall_permit2_succeeds() external view {
        assertPreinstall(
            Preinstalls.Permit2, Preinstalls.getPermit2Code(PERMIT2_DEPLOYMENT_CHAIN_ID), PERMIT2_DEPLOYMENT_CHAIN_ID
        );
    }

    function test_preinstall_senderCreatorv060_succeeds() external view {
        assertPreinstall(Preinstalls.SenderCreator_v060, Preinstalls.SenderCreator_v060Code);
    }

    function test_preinstall_entrypointv060_succeeds() external view {
        assertPreinstall(Preinstalls.EntryPoint_v060, Preinstalls.EntryPoint_v060Code);
    }

    function test_preinstall_senderCreatorv070_succeeds() external view {
        assertPreinstall(Preinstalls.SenderCreator_v070, Preinstalls.SenderCreator_v070Code);
    }

    function test_preinstall_entrypointv070_succeeds() external view {
        assertPreinstall(Preinstalls.EntryPoint_v070, Preinstalls.EntryPoint_v070Code);
    }

    function test_preinstall_beaconBlockRoots_succeeds() external view {
        assertPreinstall(Preinstalls.BeaconBlockRoots, Preinstalls.BeaconBlockRootsCode);
        assertEq(vm.getNonce(Preinstalls.BeaconBlockRootsSender), 1, "4788 sender must have nonce=1");
    }

    function test_preinstall_historyStorage_succeeds() external view {
        assertPreinstall(Preinstalls.HistoryStorage, Preinstalls.HistoryStorageCode);
        assertEq(vm.getNonce(Preinstalls.HistoryStorageSender), 1, "2935 sender must have nonce=1");
    }

    function test_preinstall_createX_succeeds() external view {
        assertPreinstall(Preinstalls.CreateX, Preinstalls.CreateXCode);
    }

    function test_createX_runtimeBytecodeHash_works() external view {
        bytes memory createXRuntimeBytecode = Preinstalls.CreateX.code;
        bytes32 createXRuntimeBytecodeHash = keccak256(createXRuntimeBytecode);

        assertEq(
            createXRuntimeBytecodeHash,
            0xbd8a7ea8cfca7b4e5f5041d7d4b17bc317c5ce42cfbc42066a00cf26b43eb53f,
            "CreateX runtime bytecode hash mismatch"
        );
    }
}

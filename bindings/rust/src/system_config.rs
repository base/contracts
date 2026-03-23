///Module containing a contract's types and functions.
/**

```solidity
library IResourceMetering {
    struct ResourceConfig { uint32 maxResourceLimit; uint8 elasticityMultiplier; uint8 baseFeeMaxChangeDenominator; uint32 minimumBaseFee; uint32 systemTxMaxGas; uint128 maximumBaseFee; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IResourceMetering {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ResourceConfig { uint32 maxResourceLimit; uint8 elasticityMultiplier; uint8 baseFeeMaxChangeDenominator; uint32 minimumBaseFee; uint32 systemTxMaxGas; uint128 maximumBaseFee; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ResourceConfig {
        #[allow(missing_docs)]
        pub maxResourceLimit: u32,
        #[allow(missing_docs)]
        pub elasticityMultiplier: u8,
        #[allow(missing_docs)]
        pub baseFeeMaxChangeDenominator: u8,
        #[allow(missing_docs)]
        pub minimumBaseFee: u32,
        #[allow(missing_docs)]
        pub systemTxMaxGas: u32,
        #[allow(missing_docs)]
        pub maximumBaseFee: u128,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, u8, u8, u32, u32, u128);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ResourceConfig> for UnderlyingRustTuple<'_> {
            fn from(value: ResourceConfig) -> Self {
                (
                    value.maxResourceLimit,
                    value.elasticityMultiplier,
                    value.baseFeeMaxChangeDenominator,
                    value.minimumBaseFee,
                    value.systemTxMaxGas,
                    value.maximumBaseFee,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ResourceConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxResourceLimit: tuple.0,
                    elasticityMultiplier: tuple.1,
                    baseFeeMaxChangeDenominator: tuple.2,
                    minimumBaseFee: tuple.3,
                    systemTxMaxGas: tuple.4,
                    maximumBaseFee: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ResourceConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ResourceConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxResourceLimit),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.elasticityMultiplier),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.baseFeeMaxChangeDenominator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumBaseFee),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.systemTxMaxGas),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.maximumBaseFee),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ResourceConfig {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ResourceConfig {
            const NAME: &'static str = "ResourceConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ResourceConfig(uint32 maxResourceLimit,uint8 elasticityMultiplier,uint8 baseFeeMaxChangeDenominator,uint32 minimumBaseFee,uint32 systemTxMaxGas,uint128 maximumBaseFee)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxResourceLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.elasticityMultiplier,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.baseFeeMaxChangeDenominator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.minimumBaseFee,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.systemTxMaxGas,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maximumBaseFee,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ResourceConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxResourceLimit,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.elasticityMultiplier,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseFeeMaxChangeDenominator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minimumBaseFee,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.systemTxMaxGas,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maximumBaseFee,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxResourceLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.elasticityMultiplier,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.baseFeeMaxChangeDenominator,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minimumBaseFee,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.systemTxMaxGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maximumBaseFee,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IResourceMetering`](self) contract instance.

See the [wrapper's documentation](`IResourceMeteringInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IResourceMeteringInstance<P, N> {
        IResourceMeteringInstance::<P, N>::new(address, __provider)
    }
    /**A [`IResourceMetering`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IResourceMetering`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IResourceMeteringInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IResourceMeteringInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IResourceMeteringInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IResourceMeteringInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IResourceMetering`](self) contract instance.

See the [wrapper's documentation](`IResourceMeteringInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IResourceMeteringInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IResourceMeteringInstance<P, N> {
            IResourceMeteringInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IResourceMeteringInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IResourceMeteringInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IResourceMetering {
    struct ResourceConfig {
        uint32 maxResourceLimit;
        uint8 elasticityMultiplier;
        uint8 baseFeeMaxChangeDenominator;
        uint32 minimumBaseFee;
        uint32 systemTxMaxGas;
        uint128 maximumBaseFee;
    }
}

interface SystemConfig {
    type UpdateType is uint8;
    struct Addresses {
        address l1CrossDomainMessenger;
        address l1ERC721Bridge;
        address l1StandardBridge;
        address optimismPortal;
        address optimismMintableERC20Factory;
        address delayedWETH;
    }

    error ProxyAdminOwnedBase_NotProxyAdmin();
    error ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner();
    error ProxyAdminOwnedBase_NotProxyAdminOwner();
    error ProxyAdminOwnedBase_NotResolvedDelegateProxy();
    error ProxyAdminOwnedBase_NotSharedProxyAdminOwner();
    error ProxyAdminOwnedBase_ProxyAdminNotFound();
    error ReinitializableBase_ZeroInitVersion();
    error SystemConfig_InvalidFeatureState();

    event ConfigUpdate(uint256 indexed version, UpdateType indexed updateType, bytes data);
    event FeatureSet(bytes32 indexed feature, bool indexed enabled);
    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    constructor();

    function BATCH_INBOX_SLOT() external view returns (bytes32);
    function DELAYED_WETH_SLOT() external view returns (bytes32);
    function L1_CROSS_DOMAIN_MESSENGER_SLOT() external view returns (bytes32);
    function L1_ERC_721_BRIDGE_SLOT() external view returns (bytes32);
    function L1_STANDARD_BRIDGE_SLOT() external view returns (bytes32);
    function OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT() external view returns (bytes32);
    function OPTIMISM_PORTAL_SLOT() external view returns (bytes32);
    function START_BLOCK_SLOT() external view returns (bytes32);
    function UNSAFE_BLOCK_SIGNER_SLOT() external view returns (bytes32);
    function VERSION() external view returns (uint256);
    function basefeeScalar() external view returns (uint32);
    function batchInbox() external view returns (address addr_);
    function batcherHash() external view returns (bytes32);
    function blobbasefeeScalar() external view returns (uint32);
    function daFootprintGasScalar() external view returns (uint16);
    function delayedWETH() external view returns (address addr_);
    function disputeGameFactory() external view returns (address addr_);
    function eip1559Denominator() external view returns (uint32);
    function eip1559Elasticity() external view returns (uint32);
    function gasLimit() external view returns (uint64);
    function getAddresses() external view returns (Addresses memory);
    function guardian() external view returns (address);
    function initVersion() external view returns (uint8);
    function initialize(address _owner, uint32 _basefeeScalar, uint32 _blobbasefeeScalar, bytes32 _batcherHash, uint64 _gasLimit, address _unsafeBlockSigner, IResourceMetering.ResourceConfig memory _config, address _batchInbox, Addresses memory _addresses, uint256 _l2ChainId, address _superchainConfig) external;
    function isCustomGasToken() external view returns (bool);
    function isFeatureEnabled(bytes32) external view returns (bool);
    function l1CrossDomainMessenger() external view returns (address addr_);
    function l1ERC721Bridge() external view returns (address addr_);
    function l1StandardBridge() external view returns (address addr_);
    function l2ChainId() external view returns (uint256);
    function maximumGasLimit() external pure returns (uint64);
    function minBaseFee() external view returns (uint64);
    function minimumGasLimit() external view returns (uint64);
    function operatorFeeConstant() external view returns (uint64);
    function operatorFeeScalar() external view returns (uint32);
    function optimismMintableERC20Factory() external view returns (address addr_);
    function optimismPortal() external view returns (address addr_);
    function overhead() external view returns (uint256);
    function owner() external view returns (address);
    function paused() external view returns (bool);
    function proxyAdmin() external view returns (address);
    function proxyAdminOwner() external view returns (address);
    function renounceOwnership() external;
    function resourceConfig() external view returns (IResourceMetering.ResourceConfig memory);
    function scalar() external view returns (uint256);
    function setBatcherHash(address _batcher) external;
    function setBatcherHash(bytes32 _batcherHash) external;
    function setDAFootprintGasScalar(uint16 _daFootprintGasScalar) external;
    function setEIP1559Params(uint32 _denominator, uint32 _elasticity) external;
    function setFeature(bytes32 _feature, bool _enabled) external;
    function setGasConfig(uint256 _overhead, uint256 _scalar) external;
    function setGasConfigEcotone(uint32 _basefeeScalar, uint32 _blobbasefeeScalar) external;
    function setGasLimit(uint64 _gasLimit) external;
    function setMinBaseFee(uint64 _minBaseFee) external;
    function setOperatorFeeScalars(uint32 _operatorFeeScalar, uint64 _operatorFeeConstant) external;
    function setUnsafeBlockSigner(address _unsafeBlockSigner) external;
    function startBlock() external view returns (uint256 startBlock_);
    function superchainConfig() external view returns (address);
    function transferOwnership(address newOwner) external;
    function unsafeBlockSigner() external view returns (address addr_);
    function version() external pure returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "BATCH_INBOX_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DELAYED_WETH_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "L1_CROSS_DOMAIN_MESSENGER_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "L1_ERC_721_BRIDGE_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "L1_STANDARD_BRIDGE_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "OPTIMISM_PORTAL_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "START_BLOCK_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UNSAFE_BLOCK_SIGNER_SLOT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "basefeeScalar",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "batchInbox",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "batcherHash",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "blobbasefeeScalar",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "daFootprintGasScalar",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delayedWETH",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "disputeGameFactory",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eip1559Denominator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eip1559Elasticity",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "gasLimit",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAddresses",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct SystemConfig.Addresses",
        "components": [
          {
            "name": "l1CrossDomainMessenger",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1ERC721Bridge",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1StandardBridge",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "optimismPortal",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "optimismMintableERC20Factory",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delayedWETH",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "guardian",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initVersion",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_basefeeScalar",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_blobbasefeeScalar",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_batcherHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_gasLimit",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_unsafeBlockSigner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_config",
        "type": "tuple",
        "internalType": "struct IResourceMetering.ResourceConfig",
        "components": [
          {
            "name": "maxResourceLimit",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "elasticityMultiplier",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "baseFeeMaxChangeDenominator",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "minimumBaseFee",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "systemTxMaxGas",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "maximumBaseFee",
            "type": "uint128",
            "internalType": "uint128"
          }
        ]
      },
      {
        "name": "_batchInbox",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_addresses",
        "type": "tuple",
        "internalType": "struct SystemConfig.Addresses",
        "components": [
          {
            "name": "l1CrossDomainMessenger",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1ERC721Bridge",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1StandardBridge",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "optimismPortal",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "optimismMintableERC20Factory",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delayedWETH",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "_l2ChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_superchainConfig",
        "type": "address",
        "internalType": "contract ISuperchainConfig"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isCustomGasToken",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isFeatureEnabled",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l1CrossDomainMessenger",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l1ERC721Bridge",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l1StandardBridge",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l2ChainId",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "maximumGasLimit",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "minBaseFee",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minimumGasLimit",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorFeeConstant",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorFeeScalar",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "optimismMintableERC20Factory",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "optimismPortal",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "overhead",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proxyAdmin",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IProxyAdmin"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proxyAdminOwner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resourceConfig",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IResourceMetering.ResourceConfig",
        "components": [
          {
            "name": "maxResourceLimit",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "elasticityMultiplier",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "baseFeeMaxChangeDenominator",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "minimumBaseFee",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "systemTxMaxGas",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "maximumBaseFee",
            "type": "uint128",
            "internalType": "uint128"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "scalar",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setBatcherHash",
    "inputs": [
      {
        "name": "_batcher",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setBatcherHash",
    "inputs": [
      {
        "name": "_batcherHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setDAFootprintGasScalar",
    "inputs": [
      {
        "name": "_daFootprintGasScalar",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEIP1559Params",
    "inputs": [
      {
        "name": "_denominator",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_elasticity",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setFeature",
    "inputs": [
      {
        "name": "_feature",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_enabled",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setGasConfig",
    "inputs": [
      {
        "name": "_overhead",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_scalar",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setGasConfigEcotone",
    "inputs": [
      {
        "name": "_basefeeScalar",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_blobbasefeeScalar",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setGasLimit",
    "inputs": [
      {
        "name": "_gasLimit",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMinBaseFee",
    "inputs": [
      {
        "name": "_minBaseFee",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setOperatorFeeScalars",
    "inputs": [
      {
        "name": "_operatorFeeScalar",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_operatorFeeConstant",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setUnsafeBlockSigner",
    "inputs": [
      {
        "name": "_unsafeBlockSigner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "startBlock",
    "inputs": [],
    "outputs": [
      {
        "name": "startBlock_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "superchainConfig",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISuperchainConfig"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unsafeBlockSigner",
    "inputs": [],
    "outputs": [
      {
        "name": "addr_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "version",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "event",
    "name": "ConfigUpdate",
    "inputs": [
      {
        "name": "version",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "updateType",
        "type": "uint8",
        "indexed": true,
        "internalType": "enum SystemConfig.UpdateType"
      },
      {
        "name": "data",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FeatureSet",
    "inputs": [
      {
        "name": "feature",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "enabled",
        "type": "bool",
        "indexed": true,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "ProxyAdminOwnedBase_NotProxyAdmin",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProxyAdminOwnedBase_NotProxyAdminOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProxyAdminOwnedBase_NotResolvedDelegateProxy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProxyAdminOwnedBase_NotSharedProxyAdminOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProxyAdminOwnedBase_ProxyAdminNotFound",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReinitializableBase_ZeroInitVersion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SystemConfig_InvalidFeatureState",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod SystemConfig {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523480156200001157600080fd5b5060046080526200005f6200004860017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a062000135565b60001b6000196200006f60201b620019b81760201c565b6200006962000073565b6200015b565b9055565b600054610100900460ff1615620000e05760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000133576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6000828210156200015657634e487b7160e01b600052601160045260246000fd5b500390565b608051612d396200017e600039600081816105480152610af70152612d396000f3fe608060405234801561001057600080fd5b50600436106103af5760003560e01c80637e54b8ad116101f4578063d220a9e01161011a578063f2c4bc9e116100ad578063f8c68de01161007c578063f8c68de01461099e578063fd32aa0f146109a6578063fe3d5710146109ae578063ffa1ad74146109e757600080fd5b8063f2c4bc9e1461095b578063f2fde38b1461096e578063f45e65d814610981578063f68016b71461098a57600080fd5b8063e0e2016d116100e9578063e0e2016d14610922578063e81b2c6d1461092a578063ec70751714610933578063f2b4e6171461095357600080fd5b8063d220a9e0146108f9578063d6ae3cd514610909578063dac6e63a14610912578063dad544e01461091a57600080fd5b8063b40a817c11610192578063c4e8ddfa11610161578063c4e8ddfa14610792578063c9b26f611461079a578063c9ff2d16146107ad578063cc731b02146107c557600080fd5b8063b40a817c14610748578063bc49ce5f1461075b578063bfb14fb714610763578063c0fd4b411461077f57600080fd5b80639b7d7f0a116101ce5780639b7d7f0a146106a3578063a39fac12146106ab578063a62611a214610714578063a71198691461074057600080fd5b80637e54b8ad146106775780638da5cb5b1461067f578063935f029e1461069057600080fd5b806321d7fde5116102d95780634add321d116102775780635c975abb116102465780635c975abb1461064c5780635d73369c14610654578063715018a61461065c5780637616f0e81461066457600080fd5b80634add321d146105ad5780634d5d9a2a146105b55780634f16540b146105e657806354fd4d501461060d57600080fd5b80633e47158c116102b35780633e47158c14610572578063452a93201461057a57806347af267b1461058257806348cd4cb1146105a557600080fd5b806321d7fde51461051b57806335e80ab31461052e57806338d38c971461054157600080fd5b806316d3bc7f116103515780631fd19ee1116103205780631fd19ee11461049957806320f06fdc146104a157806321326849146104b4578063215b7a1c1461051357600080fd5b806316d3bc7f1461044757806318d139181461046b57806319f5cea81461047e5780631edd50981461048657600080fd5b80630a49cb031161038d5780630a49cb03146104045780630ae14b1b1461040c5780630c18c1621461042b578063155b6c6f1461043457600080fd5b806306c92657146103b4578063078f29cf146103cf5780630a2ca2a9146103ef575b600080fd5b6103bc6109ef565b6040519081526020015b60405180910390f35b6103d7610a1d565b6040516001600160a01b0390911681526020016103c6565b6104026103fd366004612742565b610a56565b005b6103d7610a73565b631dcd65005b60405167ffffffffffffffff90911681526020016103c6565b6103bc60655481565b610402610442366004612792565b610aa3565b606a54610412906c01000000000000000000000000900467ffffffffffffffff1681565b610402610479366004612742565b610ab9565b6103bc610aca565b6104026104943660046128fc565b610af5565b6103d7610e6a565b6104026104af366004612a6c565b610e94565b7f435553544f4d5f4741535f544f4b454e00000000000000000000000000000000600052606d6020527f3ec9a18bd22a834e2a13465cc2aa2a9aebb161ffdebf39cdb0028dbb1b5394b45460ff165b60405190151581526020016103c6565b6103d7610ea5565b610402610529366004612a90565b610ed5565b606c546103d7906001600160a01b031681565b60405160ff7f00000000000000000000000000000000000000000000000000000000000000001681526020016103c6565b6103d7610ee7565b6103d76110cb565b610503610590366004612aba565b606d6020526000908152604090205460ff1681565b6103bc611152565b610412611182565b606a546105d19068010000000000000000900463ffffffff1681565b60405163ffffffff90911681526020016103c6565b6103bc7f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c0881565b604080518082018252600681527f332e31332e320000000000000000000000000000000000000000000000000000602082015290516103c69190612b20565b6105036111a8565b6103bc61138e565b6104026113b9565b610402610672366004612b33565b6113cd565b6103bc6113de565b6033546001600160a01b03166103d7565b61040261069e366004612b4e565b611409565b6103d761141b565b6106b361144b565b6040516103c69190600060c0820190506001600160a01b038084511683528060208501511660208401528060408501511660408401528060608501511660608401528060808501511660808401528060a08501511660a08401525092915050565b606c546104129074010000000000000000000000000000000000000000900467ffffffffffffffff1681565b6103d761150e565b610402610756366004612b33565b61153e565b6103bc61154f565b6068546105d19068010000000000000000900463ffffffff1681565b61040261078d366004612a90565b61157a565b6103d761158c565b6104026107a8366004612aba565b6115bc565b606a546105d190640100000000900463ffffffff1681565b6108896040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a0810191909152506040805160c08101825260695463ffffffff8082168352640100000000820460ff9081166020850152650100000000008304169383019390935266010000000000008104831660608301526a0100000000000000000000810490921660808201526e0100000000000000000000000000009091046fffffffffffffffffffffffffffffffff1660a082015290565b6040516103c69190600060c08201905063ffffffff80845116835260ff602085015116602084015260ff6040850151166040840152806060850151166060840152806080850151166080840152506fffffffffffffffffffffffffffffffff60a08401511660a083015292915050565b606a546105d19063ffffffff1681565b6103bc606b5481565b6103d76115cd565b6103d76115fd565b6103bc611644565b6103bc60675481565b6068546105d1906c01000000000000000000000000900463ffffffff1681565b6103d761166f565b610402610969366004612b7e565b6116de565b61040261097c366004612742565b6118d5565b6103bc60665481565b6068546104129067ffffffffffffffff1681565b6103bc611962565b6103bc61198d565b606a546109d49074010000000000000000000000000000000000000000900461ffff1681565b60405161ffff90911681526020016103c6565b6103bc600081565b610a1a60017fa04c5bb938ca6fc46d95553abf0a76345ce3e722a30bf4f74928b8e7d852320d612bdd565b81565b6000610a51610a4d60017f9904ba90dde5696cda05c9e0dab5cbaa0fea005ace4d11218a02ac668dad6377612bdd565b5490565b905090565b610a5e6119bc565b610a706001600160a01b038216611a16565b50565b6000610a51610a4d60017f4b6c74f9e688cb39801f2112c14a8c57232a3fc5202e1444126d4bce86eb19ad612bdd565b610aab6119bc565b610ab58282611a77565b5050565b610ac16119bc565b610a7081611b69565b610a1a60017f46adcbebc6be8ce551740c29c47c8798210f23f7f4086c41752944352568d5a8612bdd565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff16158015610b35575060005460ff8083169116105b610bac5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a656400000000000000000000000000000000000060648201526084015b60405180910390fd5b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff831617610100179055610be5611bc2565b610bed611c29565b610bf68c6118d5565b610bff89611a16565b610c098b8b611cae565b610c1288611d95565b610c3b7f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c08889055565b610c6e610c6960017f71ac12829d66ee73d8d95bff50b3589745ce57edae70a3fb111a2342464dc598612bdd565b869055565b610ca2610c9c60017f383f291819e6d54073bc9a648251d97421076bdd101933c0c022219ce9580637612bdd565b85519055565b610cd9610cd060017f46adcbebc6be8ce551740c29c47c8798210f23f7f4086c41752944352568d5a8612bdd565b60208601519055565b610d10610d0760017f9904ba90dde5696cda05c9e0dab5cbaa0fea005ace4d11218a02ac668dad6377612bdd565b60408601519055565b610d47610d3e60017f4b6c74f9e688cb39801f2112c14a8c57232a3fc5202e1444126d4bce86eb19ad612bdd565b60608601519055565b610d7e610d7560017fa04c5bb938ca6fc46d95553abf0a76345ce3e722a30bf4f74928b8e7d852320d612bdd565b60808601519055565b610db5610dac60017f51547f31a231e1007dca33017faa3da20d959b95087c588a7768bfb922fd5900612bdd565b60a08601519055565b610dbd611eb7565b610dc686611f1f565b606b839055606c80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b038416179055600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050505050505050505050565b6000610a517f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c085490565b610e9c6119bc565b610a7081612311565b6000610a51610a4d60017f51547f31a231e1007dca33017faa3da20d959b95087c588a7768bfb922fd5900612bdd565b610edd6119bc565b610ab58282611cae565b600080610f127fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035490565b90506001600160a01b03811615610f2857919050565b6040518060400160405280601a81526020017f4f564d5f4c3143726f7373446f6d61696e4d657373656e676572000000000000815250516002610f6b9190612bf4565b604080513060208201526000918101919091527f4f564d5f4c3143726f7373446f6d61696e4d657373656e6765720000000000009190911790610fc6906060015b604051602081830303815290604052805190602001205490565b14610ffd576040517f54e433cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805130602082015260019181019190915260009061101f90606001610fac565b90506001600160a01b0381161561109957806001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561106e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110929190612c31565b9250505090565b6040517f332144db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b606c54604080517f452a932000000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163452a93209160048083019260209291908290030181865afa15801561112e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a519190612c31565b6000610a51610a4d60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b606954600090610a519063ffffffff6a0100000000000000000000820481169116612c4e565b7f4554485f4c4f434b424f580000000000000000000000000000000000000000006000908152606d6020527f58e88e949bd180ff86d6c072735c3d8d5a05a543c16130176ec0bc0adf3fd80654819060ff1661120b57611206610a73565b611274565b611213610a73565b6001600160a01b031663b682c4446040518163ffffffff1660e01b8152600401602060405180830381865afa158015611250573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112749190612c31565b606c546040517f2e48152c000000000000000000000000000000000000000000000000000000008152600060048201529192506001600160a01b031690632e48152c90602401602060405180830381865afa1580156112d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112fb9190612c7a565b806113885750606c546040517f2e48152c0000000000000000000000000000000000000000000000000000000081526001600160a01b03838116600483015290911690632e48152c90602401602060405180830381865afa158015611364573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113889190612c7a565b91505090565b610a1a60017f383f291819e6d54073bc9a648251d97421076bdd101933c0c022219ce9580637612bdd565b6113c16119bc565b6113cb600061237e565b565b6113d56119bc565b610a70816123e8565b610a1a60017f51547f31a231e1007dca33017faa3da20d959b95087c588a7768bfb922fd5900612bdd565b6114116119bc565b610ab5828261245b565b6000610a51610a4d60017fa04c5bb938ca6fc46d95553abf0a76345ce3e722a30bf4f74928b8e7d852320d612bdd565b6040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a08101919091526040518060c0016040528061149061150e565b6001600160a01b031681526020016114a661158c565b6001600160a01b031681526020016114bc610a1d565b6001600160a01b031681526020016114d2610a73565b6001600160a01b031681526020016114e861141b565b6001600160a01b031681526020016114fe610ea5565b6001600160a01b03169052919050565b6000610a51610a4d60017f383f291819e6d54073bc9a648251d97421076bdd101933c0c022219ce9580637612bdd565b6115466119bc565b610a7081611d95565b610a1a60017f71ac12829d66ee73d8d95bff50b3589745ce57edae70a3fb111a2342464dc598612bdd565b6115826119bc565b610ab58282612517565b6000610a51610a4d60017f46adcbebc6be8ce551740c29c47c8798210f23f7f4086c41752944352568d5a8612bdd565b6115c46119bc565b610a7081611a16565b6000610a51610a4d60017f71ac12829d66ee73d8d95bff50b3589745ce57edae70a3fb111a2342464dc598612bdd565b6000611607610ee7565b6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561112e573d6000803e3d6000fd5b610a1a60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b60008061167a610a73565b9050806001600160a01b031663f2b4e6176040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116ba573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113889190612c31565b6116e6611bc2565b6000828152606d602052604090205460ff16151581151503611734576040517ff5828b0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f4554485f4c4f434b424f58000000000000000000000000000000000000000000820361186b576000828152606d602052604090205460ff168015611777575080155b80156117f557506000611788610a73565b6001600160a01b031663b682c4446040518163ffffffff1660e01b8152600401602060405180830381865afa1580156117c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117e99190612c31565b6001600160a01b031614155b1561182c576040517ff5828b0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6118346111a8565b1561186b576040517ff5828b0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000828152606d602052604080822080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00168415159081179091559051909184917fb876f6594132c89891d2fd198e925e999be741ec809abb58bfe9b966876cc06c9190a35050565b6118dd6119bc565b6001600160a01b0381166119595760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152608401610ba3565b610a708161237e565b610a1a60017f9904ba90dde5696cda05c9e0dab5cbaa0fea005ace4d11218a02ac668dad6377612bdd565b610a1a60017f4b6c74f9e688cb39801f2112c14a8c57232a3fc5202e1444126d4bce86eb19ad612bdd565b9055565b6033546001600160a01b031633146113cb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610ba3565b60678190556040805160208082018490528251808303909101815290820190915260005b60007f1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be83604051611a6b9190612b20565b60405180910390a35050565b606a80547fffffffffffffffffffffffff000000000000000000000000ffffffffffffffff166801000000000000000063ffffffff8516027fffffffffffffffffffffffff0000000000000000ffffffffffffffffffffffff16176c0100000000000000000000000067ffffffffffffffff841690810291909117909155604080516bffffffff000000000000000085831b1690921760208301526000910160408051601f19818403018152919052905060055b60007f1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be83604051611b5c9190612b20565b60405180910390a3505050565b611b927f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c08829055565b604080516001600160a01b03831660208201526000910160408051601f1981840301815291905290506003611a3a565b33611bcb610ee7565b6001600160a01b031614158015611bf2575033611be66115fd565b6001600160a01b031614155b156113cb576040517fc4050a2600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600054610100900460ff16611ca65760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401610ba3565b6113cb612697565b606880547fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff166801000000000000000063ffffffff8581169182027fffffffffffffffffffffffffffffffff00000000ffffffffffffffffffffffff16929092176c0100000000000000000000000092851692909202919091179091557f0100000000000000000000000000000000000000000000000000000000000000602083811b67ffffffff000000001690921717606681905560655460408051938401919091528201526000906060015b60408051601f1981840301815291905290506001611b2b565b611d9d611182565b67ffffffffffffffff168167ffffffffffffffff161015611e005760405162461bcd60e51b815260206004820152601f60248201527f53797374656d436f6e6669673a20676173206c696d697420746f6f206c6f77006044820152606401610ba3565b631dcd650067ffffffffffffffff82161115611e5e5760405162461bcd60e51b815260206004820181905260248201527f53797374656d436f6e6669673a20676173206c696d697420746f6f20686967686044820152606401610ba3565b606880547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff83169081179091556040805160208082019390935281518082039093018352810190526002611a3a565b611ee5610a4d60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b6000036113cb576113cb611f1a60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b439055565b8060a001516fffffffffffffffffffffffffffffffff16816060015163ffffffff161115611fb55760405162461bcd60e51b815260206004820152603560248201527f53797374656d436f6e6669673a206d696e206261736520666565206d7573742060448201527f6265206c657373207468616e206d6178206261736500000000000000000000006064820152608401610ba3565b6001816040015160ff16116120325760405162461bcd60e51b815260206004820152602f60248201527f53797374656d436f6e6669673a2064656e6f6d696e61746f72206d757374206260448201527f65206c6172676572207468616e203100000000000000000000000000000000006064820152608401610ba3565b6068546080820151825167ffffffffffffffff909216916120539190612c97565b63ffffffff1611156120a75760405162461bcd60e51b815260206004820152601f60248201527f53797374656d436f6e6669673a20676173206c696d697420746f6f206c6f77006044820152606401610ba3565b6000816020015160ff16116121245760405162461bcd60e51b815260206004820152602f60248201527f53797374656d436f6e6669673a20656c6173746963697479206d756c7469706c60448201527f6965722063616e6e6f74206265203000000000000000000000000000000000006064820152608401610ba3565b8051602082015163ffffffff82169160ff90911690612144908290612cb6565b61214e9190612d00565b63ffffffff16146121c75760405162461bcd60e51b815260206004820152603760248201527f53797374656d436f6e6669673a20707265636973696f6e206c6f73732077697460448201527f6820746172676574207265736f75726365206c696d69740000000000000000006064820152608401610ba3565b805160698054602084015160408501516060860151608087015160a09097015163ffffffff9687167fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009095169490941764010000000060ff94851602177fffffffffffffffffffffffffffffffffffffffffffff0000000000ffffffffff166501000000000093909216929092027fffffffffffffffffffffffffffffffffffffffffffff00000000ffffffffffff1617660100000000000091851691909102177fffff0000000000000000000000000000000000000000ffffffffffffffffffff166a010000000000000000000093909416929092027fffff00000000000000000000000000000000ffffffffffffffffffffffffffff16929092176e0100000000000000000000000000006fffffffffffffffffffffffffffffffff90921691909102179055565b606a80547fffffffffffffffffffff0000ffffffffffffffffffffffffffffffffffffffff167401000000000000000000000000000000000000000061ffff8416908102919091179091556040805160208082019390935281518082039093018352810190526007611a3a565b603380546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b606c80547fffffffff0000000000000000ffffffffffffffffffffffffffffffffffffffff167401000000000000000000000000000000000000000067ffffffffffffffff8416908102919091179091556040805160208082019390935281518082039093018352810190526006611a3a565b7fff000000000000000000000000000000000000000000000000000000000000008116156124f15760405162461bcd60e51b815260206004820152602160248201527f53797374656d436f6e6669673a207363616c61722065786365656473206d617860448201527f2e000000000000000000000000000000000000000000000000000000000000006064820152608401610ba3565b606582905560668190556040805160208101849052908101829052600090606001611d7c565b60018263ffffffff1610156125945760405162461bcd60e51b815260206004820152602660248201527f53797374656d436f6e6669673a2064656e6f6d696e61746f72206d757374206260448201527f65203e3d203100000000000000000000000000000000000000000000000000006064820152608401610ba3565b60018163ffffffff1610156126115760405162461bcd60e51b815260206004820152602560248201527f53797374656d436f6e6669673a20656c6173746963697479206d75737420626560448201527f203e3d20310000000000000000000000000000000000000000000000000000006064820152608401610ba3565b606a805463ffffffff83811664010000000081027fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000909316918616919091179190911790915560405160009161267e91602086811b67ffffffff0000000016909217910190815260200190565b60408051601f1981840301815291905290506004611b2b565b600054610100900460ff166127145760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401610ba3565b6113cb3361237e565b6001600160a01b0381168114610a7057600080fd5b803561273d8161271d565b919050565b60006020828403121561275457600080fd5b813561275f8161271d565b9392505050565b803563ffffffff8116811461273d57600080fd5b803567ffffffffffffffff8116811461273d57600080fd5b600080604083850312156127a557600080fd5b6127ae83612766565b91506127bc6020840161277a565b90509250929050565b60405160c0810167ffffffffffffffff8111828210171561280f577f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60405290565b803560ff8116811461273d57600080fd5b600060c0828403121561283857600080fd5b60405160c0810181811067ffffffffffffffff82111715612882577f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60405290508082356128938161271d565b815260208301356128a38161271d565b602082015260408301356128b68161271d565b604082015260608301356128c98161271d565b606082015260808301356128dc8161271d565b608082015260a08301356128ef8161271d565b60a0919091015292915050565b60008060008060008060008060008060008b8d036102a081121561291f57600080fd5b8c3561292a8161271d565b9b5061293860208e01612766565b9a5061294660408e01612766565b995060608d0135985061295b60808e0161277a565b975060a08d013561296b8161271d565b965060c07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408201121561299d57600080fd5b506129a66127c5565b6129b260c08e01612766565b81526129c060e08e01612815565b60208201526129d26101008e01612815565b60408201526129e46101208e01612766565b60608201526129f66101408e01612766565b60808201526101608d01356fffffffffffffffffffffffffffffffff81168114612a1f57600080fd5b60a08201529450612a336101808d01612732565b9350612a438d6101a08e01612826565b92506102608c01359150612a5a6102808d01612732565b90509295989b509295989b9093969950565b600060208284031215612a7e57600080fd5b813561ffff8116811461275f57600080fd5b60008060408385031215612aa357600080fd5b612aac83612766565b91506127bc60208401612766565b600060208284031215612acc57600080fd5b5035919050565b6000815180845260005b81811015612af957602081850181015186830182015201612add565b81811115612b0b576000602083870101525b50601f01601f19169290920160200192915050565b60208152600061275f6020830184612ad3565b600060208284031215612b4557600080fd5b61275f8261277a565b60008060408385031215612b6157600080fd5b50508035926020909101359150565b8015158114610a7057600080fd5b60008060408385031215612b9157600080fd5b823591506020830135612ba381612b70565b809150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600082821015612bef57612bef612bae565b500390565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0483118215151615612c2c57612c2c612bae565b500290565b600060208284031215612c4357600080fd5b815161275f8161271d565b600067ffffffffffffffff808316818516808303821115612c7157612c71612bae565b01949350505050565b600060208284031215612c8c57600080fd5b815161275f81612b70565b600063ffffffff808316818516808303821115612c7157612c71612bae565b600063ffffffff80841680612cf4577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b92169190910492915050565b600063ffffffff80831681851681830481118215151615612d2357612d23612bae565b0294935050505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x04`\x80Rb\0\0_b\0\0H`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0b\0\x015V[`\0\x1B`\0\x19b\0\0o` \x1Bb\0\x19\xB8\x17` \x1CV[b\0\0ib\0\0sV[b\0\x01[V[\x90UV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x013W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0\x82\x82\x10\x15b\0\x01VWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[`\x80Qa-9b\0\x01~`\09`\0\x81\x81a\x05H\x01Ra\n\xF7\x01Ra-9`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xAFW`\x005`\xE0\x1C\x80c~T\xB8\xAD\x11a\x01\xF4W\x80c\xD2 \xA9\xE0\x11a\x01\x1AW\x80c\xF2\xC4\xBC\x9E\x11a\0\xADW\x80c\xF8\xC6\x8D\xE0\x11a\0|W\x80c\xF8\xC6\x8D\xE0\x14a\t\x9EW\x80c\xFD2\xAA\x0F\x14a\t\xA6W\x80c\xFE=W\x10\x14a\t\xAEW\x80c\xFF\xA1\xADt\x14a\t\xE7W`\0\x80\xFD[\x80c\xF2\xC4\xBC\x9E\x14a\t[W\x80c\xF2\xFD\xE3\x8B\x14a\tnW\x80c\xF4^e\xD8\x14a\t\x81W\x80c\xF6\x80\x16\xB7\x14a\t\x8AW`\0\x80\xFD[\x80c\xE0\xE2\x01m\x11a\0\xE9W\x80c\xE0\xE2\x01m\x14a\t\"W\x80c\xE8\x1B,m\x14a\t*W\x80c\xECpu\x17\x14a\t3W\x80c\xF2\xB4\xE6\x17\x14a\tSW`\0\x80\xFD[\x80c\xD2 \xA9\xE0\x14a\x08\xF9W\x80c\xD6\xAE<\xD5\x14a\t\tW\x80c\xDA\xC6\xE6:\x14a\t\x12W\x80c\xDA\xD5D\xE0\x14a\t\x1AW`\0\x80\xFD[\x80c\xB4\n\x81|\x11a\x01\x92W\x80c\xC4\xE8\xDD\xFA\x11a\x01aW\x80c\xC4\xE8\xDD\xFA\x14a\x07\x92W\x80c\xC9\xB2oa\x14a\x07\x9AW\x80c\xC9\xFF-\x16\x14a\x07\xADW\x80c\xCCs\x1B\x02\x14a\x07\xC5W`\0\x80\xFD[\x80c\xB4\n\x81|\x14a\x07HW\x80c\xBCI\xCE_\x14a\x07[W\x80c\xBF\xB1O\xB7\x14a\x07cW\x80c\xC0\xFDKA\x14a\x07\x7FW`\0\x80\xFD[\x80c\x9B}\x7F\n\x11a\x01\xCEW\x80c\x9B}\x7F\n\x14a\x06\xA3W\x80c\xA3\x9F\xAC\x12\x14a\x06\xABW\x80c\xA6&\x11\xA2\x14a\x07\x14W\x80c\xA7\x11\x98i\x14a\x07@W`\0\x80\xFD[\x80c~T\xB8\xAD\x14a\x06wW\x80c\x8D\xA5\xCB[\x14a\x06\x7FW\x80c\x93_\x02\x9E\x14a\x06\x90W`\0\x80\xFD[\x80c!\xD7\xFD\xE5\x11a\x02\xD9W\x80cJ\xDD2\x1D\x11a\x02wW\x80c\\\x97Z\xBB\x11a\x02FW\x80c\\\x97Z\xBB\x14a\x06LW\x80c]s6\x9C\x14a\x06TW\x80cqP\x18\xA6\x14a\x06\\W\x80cv\x16\xF0\xE8\x14a\x06dW`\0\x80\xFD[\x80cJ\xDD2\x1D\x14a\x05\xADW\x80cM]\x9A*\x14a\x05\xB5W\x80cO\x16T\x0B\x14a\x05\xE6W\x80cT\xFDMP\x14a\x06\rW`\0\x80\xFD[\x80c>G\x15\x8C\x11a\x02\xB3W\x80c>G\x15\x8C\x14a\x05rW\x80cE*\x93 \x14a\x05zW\x80cG\xAF&{\x14a\x05\x82W\x80cH\xCDL\xB1\x14a\x05\xA5W`\0\x80\xFD[\x80c!\xD7\xFD\xE5\x14a\x05\x1BW\x80c5\xE8\n\xB3\x14a\x05.W\x80c8\xD3\x8C\x97\x14a\x05AW`\0\x80\xFD[\x80c\x16\xD3\xBC\x7F\x11a\x03QW\x80c\x1F\xD1\x9E\xE1\x11a\x03 W\x80c\x1F\xD1\x9E\xE1\x14a\x04\x99W\x80c \xF0o\xDC\x14a\x04\xA1W\x80c!2hI\x14a\x04\xB4W\x80c![z\x1C\x14a\x05\x13W`\0\x80\xFD[\x80c\x16\xD3\xBC\x7F\x14a\x04GW\x80c\x18\xD19\x18\x14a\x04kW\x80c\x19\xF5\xCE\xA8\x14a\x04~W\x80c\x1E\xDDP\x98\x14a\x04\x86W`\0\x80\xFD[\x80c\nI\xCB\x03\x11a\x03\x8DW\x80c\nI\xCB\x03\x14a\x04\x04W\x80c\n\xE1K\x1B\x14a\x04\x0CW\x80c\x0C\x18\xC1b\x14a\x04+W\x80c\x15[lo\x14a\x044W`\0\x80\xFD[\x80c\x06\xC9&W\x14a\x03\xB4W\x80c\x07\x8F)\xCF\x14a\x03\xCFW\x80c\n,\xA2\xA9\x14a\x03\xEFW[`\0\x80\xFD[a\x03\xBCa\t\xEFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xD7a\n\x1DV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x04\x02a\x03\xFD6`\x04a'BV[a\nVV[\0[a\x03\xD7a\nsV[c\x1D\xCDe\0[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x03\xBC`eT\x81V[a\x04\x02a\x04B6`\x04a'\x92V[a\n\xA3V[`jTa\x04\x12\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x02a\x04y6`\x04a'BV[a\n\xB9V[a\x03\xBCa\n\xCAV[a\x04\x02a\x04\x946`\x04a(\xFCV[a\n\xF5V[a\x03\xD7a\x0EjV[a\x04\x02a\x04\xAF6`\x04a*lV[a\x0E\x94V[\x7FCUSTOM_GAS_TOKEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`m` R\x7F>\xC9\xA1\x8B\xD2*\x83N*\x13F\\\xC2\xAA*\x9A\xEB\xB1a\xFF\xDE\xBF9\xCD\xB0\x02\x8D\xBB\x1BS\x94\xB4T`\xFF\x16[`@Q\x90\x15\x15\x81R` \x01a\x03\xC6V[a\x03\xD7a\x0E\xA5V[a\x04\x02a\x05)6`\x04a*\x90V[a\x0E\xD5V[`lTa\x03\xD7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x03\xC6V[a\x03\xD7a\x0E\xE7V[a\x03\xD7a\x10\xCBV[a\x05\x03a\x05\x906`\x04a*\xBAV[`m` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\xBCa\x11RV[a\x04\x12a\x11\x82V[`jTa\x05\xD1\x90h\x01\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x03\xBC\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x81V[`@\x80Q\x80\x82\x01\x82R`\x06\x81R\x7F3.13.2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x03\xC6\x91\x90a+ V[a\x05\x03a\x11\xA8V[a\x03\xBCa\x13\x8EV[a\x04\x02a\x13\xB9V[a\x04\x02a\x06r6`\x04a+3V[a\x13\xCDV[a\x03\xBCa\x13\xDEV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD7V[a\x04\x02a\x06\x9E6`\x04a+NV[a\x14\tV[a\x03\xD7a\x14\x1BV[a\x06\xB3a\x14KV[`@Qa\x03\xC6\x91\x90`\0`\xC0\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01R\x80`\xA0\x85\x01Q\x16`\xA0\x84\x01RP\x92\x91PPV[`lTa\x04\x12\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xD7a\x15\x0EV[a\x04\x02a\x07V6`\x04a+3V[a\x15>V[a\x03\xBCa\x15OV[`hTa\x05\xD1\x90h\x01\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x02a\x07\x8D6`\x04a*\x90V[a\x15zV[a\x03\xD7a\x15\x8CV[a\x04\x02a\x07\xA86`\x04a*\xBAV[a\x15\xBCV[`jTa\x05\xD1\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x08\x89`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`iTc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[`@Qa\x03\xC6\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`jTa\x05\xD1\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xBC`kT\x81V[a\x03\xD7a\x15\xCDV[a\x03\xD7a\x15\xFDV[a\x03\xBCa\x16DV[a\x03\xBC`gT\x81V[`hTa\x05\xD1\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xD7a\x16oV[a\x04\x02a\ti6`\x04a+~V[a\x16\xDEV[a\x04\x02a\t|6`\x04a'BV[a\x18\xD5V[a\x03\xBC`fT\x81V[`hTa\x04\x12\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xBCa\x19bV[a\x03\xBCa\x19\x8DV[`jTa\t\xD4\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x03\xBC`\0\x81V[a\n\x1A`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra+\xDDV[\x81V[`\0a\nQa\nM`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa+\xDDV[T\x90V[\x90P\x90V[a\n^a\x19\xBCV[a\np`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1A\x16V[PV[`\0a\nQa\nM`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa+\xDDV[a\n\xABa\x19\xBCV[a\n\xB5\x82\x82a\x1AwV[PPV[a\n\xC1a\x19\xBCV[a\np\x81a\x1BiV[a\n\x1A`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a+\xDDV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x0B5WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x0B\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x0B\xE5a\x1B\xC2V[a\x0B\xEDa\x1C)V[a\x0B\xF6\x8Ca\x18\xD5V[a\x0B\xFF\x89a\x1A\x16V[a\x0C\t\x8B\x8Ba\x1C\xAEV[a\x0C\x12\x88a\x1D\x95V[a\x0C;\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x88\x90UV[a\x0Cna\x0Ci`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a+\xDDV[\x86\x90UV[a\x0C\xA2a\x0C\x9C`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a+\xDDV[\x85Q\x90UV[a\x0C\xD9a\x0C\xD0`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a+\xDDV[` \x86\x01Q\x90UV[a\r\x10a\r\x07`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa+\xDDV[`@\x86\x01Q\x90UV[a\rGa\r>`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa+\xDDV[``\x86\x01Q\x90UV[a\r~a\ru`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra+\xDDV[`\x80\x86\x01Q\x90UV[a\r\xB5a\r\xAC`\x01\x7FQT\x7F1\xA21\xE1\0}\xCA3\x01\x7F\xAA=\xA2\r\x95\x9B\x95\x08|X\x8Awh\xBF\xB9\"\xFDY\0a+\xDDV[`\xA0\x86\x01Q\x90UV[a\r\xBDa\x1E\xB7V[a\r\xC6\x86a\x1F\x1FV[`k\x83\x90U`l\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPPV[`\0a\nQ\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08T\x90V[a\x0E\x9Ca\x19\xBCV[a\np\x81a#\x11V[`\0a\nQa\nM`\x01\x7FQT\x7F1\xA21\xE1\0}\xCA3\x01\x7F\xAA=\xA2\r\x95\x9B\x95\x08|X\x8Awh\xBF\xB9\"\xFDY\0a+\xDDV[a\x0E\xDDa\x19\xBCV[a\n\xB5\x82\x82a\x1C\xAEV[`\0\x80a\x0F\x12\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0F(W\x91\x90PV[`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x81RPQ`\x02a\x0Fk\x91\x90a+\xF4V[`@\x80Q0` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x91\x90\x91\x17\x90a\x0F\xC6\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 T\x90V[\x14a\x0F\xFDW`@Q\x7FT\xE43\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R`\0\x90a\x10\x1F\x90``\x01a\x0F\xACV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x10\x99W\x80`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x92\x91\x90a,1V[\x92PPP\x90V[`@Q\x7F3!D\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`lT`@\x80Q\x7FE*\x93 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cE*\x93 \x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x11.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nQ\x91\x90a,1V[`\0a\nQa\nM`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[`iT`\0\x90a\nQ\x90c\xFF\xFF\xFF\xFFj\x01\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16a,NV[\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x90\x81R`m` R\x7FX\xE8\x8E\x94\x9B\xD1\x80\xFF\x86\xD6\xC0rs\\=\x8DZ\x05\xA5C\xC1a0\x17n\xC0\xBC\n\xDF?\xD8\x06T\x81\x90`\xFF\x16a\x12\x0BWa\x12\x06a\nsV[a\x12tV[a\x12\x13a\nsV[`\x01`\x01`\xA0\x1B\x03\x16c\xB6\x82\xC4D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12t\x91\x90a,1V[`lT`@Q\x7F.H\x15,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c.H\x15,\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFB\x91\x90a,zV[\x80a\x13\x88WP`lT`@Q\x7F.H\x15,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c.H\x15,\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x88\x91\x90a,zV[\x91PP\x90V[a\n\x1A`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a+\xDDV[a\x13\xC1a\x19\xBCV[a\x13\xCB`\0a#~V[V[a\x13\xD5a\x19\xBCV[a\np\x81a#\xE8V[a\n\x1A`\x01\x7FQT\x7F1\xA21\xE1\0}\xCA3\x01\x7F\xAA=\xA2\r\x95\x9B\x95\x08|X\x8Awh\xBF\xB9\"\xFDY\0a+\xDDV[a\x14\x11a\x19\xBCV[a\n\xB5\x82\x82a$[V[`\0a\nQa\nM`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra+\xDDV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R`@Q\x80`\xC0\x01`@R\x80a\x14\x90a\x15\x0EV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xA6a\x15\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xBCa\n\x1DV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xD2a\nsV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xE8a\x14\x1BV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xFEa\x0E\xA5V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x91\x90PV[`\0a\nQa\nM`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a+\xDDV[a\x15Fa\x19\xBCV[a\np\x81a\x1D\x95V[a\n\x1A`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a+\xDDV[a\x15\x82a\x19\xBCV[a\n\xB5\x82\x82a%\x17V[`\0a\nQa\nM`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a+\xDDV[a\x15\xC4a\x19\xBCV[a\np\x81a\x1A\x16V[`\0a\nQa\nM`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a+\xDDV[`\0a\x16\x07a\x0E\xE7V[`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11.W=`\0\x80>=`\0\xFD[a\n\x1A`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[`\0\x80a\x16za\nsV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xB4\xE6\x17`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x88\x91\x90a,1V[a\x16\xE6a\x1B\xC2V[`\0\x82\x81R`m` R`@\x90 T`\xFF\x16\x15\x15\x81\x15\x15\x03a\x174W`@Q\x7F\xF5\x82\x8B\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x18kW`\0\x82\x81R`m` R`@\x90 T`\xFF\x16\x80\x15a\x17wWP\x80\x15[\x80\x15a\x17\xF5WP`\0a\x17\x88a\nsV[`\x01`\x01`\xA0\x1B\x03\x16c\xB6\x82\xC4D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE9\x91\x90a,1V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x18,W`@Q\x7F\xF5\x82\x8B\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x184a\x11\xA8V[\x15a\x18kW`@Q\x7F\xF5\x82\x8B\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`m` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x84\x15\x15\x90\x81\x17\x90\x91U\x90Q\x90\x91\x84\x91\x7F\xB8v\xF6YA2\xC8\x98\x91\xD2\xFD\x19\x8E\x92^\x99\x9B\xE7A\xEC\x80\x9A\xBBX\xBF\xE9\xB9f\x87l\xC0l\x91\x90\xA3PPV[a\x18\xDDa\x19\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[a\np\x81a#~V[a\n\x1A`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa+\xDDV[a\n\x1A`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa+\xDDV[\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\xA3V[`g\x81\x90U`@\x80Q` \x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x90\x82\x01\x90\x91R`\0[`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x1Ak\x91\x90a+ V[`@Q\x80\x91\x03\x90\xA3PPV[`j\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x85\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@\x80Qk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x90\x92\x17` \x83\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x05[`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x1B\\\x91\x90a+ V[`@Q\x80\x91\x03\x90\xA3PPPV[a\x1B\x92\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x82\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x03a\x1A:V[3a\x1B\xCBa\x0E\xE7V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x1B\xF2WP3a\x1B\xE6a\x15\xFDV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x13\xCBW`@Q\x7F\xC4\x05\n&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1C\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[a\x13\xCBa&\x97V[`h\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x85\x81\x16\x91\x82\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x92\x85\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90\x91U\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x81\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x90\x92\x17\x17`f\x81\x90U`eT`@\x80Q\x93\x84\x01\x91\x90\x91R\x82\x01R`\0\x90``\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x01a\x1B+V[a\x1D\x9Da\x11\x82V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x0B\xA3V[c\x1D\xCDe\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a\x1E^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FSystemConfig: gas limit too high`D\x82\x01R`d\x01a\x0B\xA3V[`h\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x02a\x1A:V[a\x1E\xE5a\nM`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[`\0\x03a\x13\xCBWa\x13\xCBa\x1F\x1A`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[C\x90UV[\x80`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FSystemConfig: min base fee must `D\x82\x01R\x7Fbe less than max base\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`\x01\x81`@\x01Q`\xFF\x16\x11a 2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: denominator must b`D\x82\x01R\x7Fe larger than 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`hT`\x80\x82\x01Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91a S\x91\x90a,\x97V[c\xFF\xFF\xFF\xFF\x16\x11\x15a \xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x0B\xA3V[`\0\x81` \x01Q`\xFF\x16\x11a!$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: elasticity multipl`D\x82\x01R\x7Fier cannot be 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[\x80Q` \x82\x01Qc\xFF\xFF\xFF\xFF\x82\x16\x91`\xFF\x90\x91\x16\x90a!D\x90\x82\x90a,\xB6V[a!N\x91\x90a-\0V[c\xFF\xFF\xFF\xFF\x16\x14a!\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FSystemConfig: precision loss wit`D\x82\x01R\x7Fh target resource limit\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[\x80Q`i\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x02\x17\x90UV[`j\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x07a\x1A:V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`l\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x06a\x1A:V[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x15a$\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FSystemConfig: scalar exceeds max`D\x82\x01R\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`e\x82\x90U`f\x81\x90U`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\0\x90``\x01a\x1D|V[`\x01\x82c\xFF\xFF\xFF\xFF\x16\x10\x15a%\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSystemConfig: denominator must b`D\x82\x01R\x7Fe >= 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`\x01\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a&\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FSystemConfig: elasticity must be`D\x82\x01R\x7F >= 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`j\x80Tc\xFF\xFF\xFF\xFF\x83\x81\x16d\x01\0\0\0\0\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x90\x93\x16\x91\x86\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90\x91U`@Q`\0\x91a&~\x91` \x86\x81\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x90\x92\x17\x91\x01\x90\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x04a\x1B+V[`\0Ta\x01\0\x90\x04`\xFF\x16a'\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[a\x13\xCB3a#~V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\npW`\0\x80\xFD[\x805a'=\x81a'\x1DV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a'TW`\0\x80\xFD[\x815a'_\x81a'\x1DV[\x93\x92PPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'=W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'=W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'\xA5W`\0\x80\xFD[a'\xAE\x83a'fV[\x91Pa'\xBC` \x84\x01a'zV[\x90P\x92P\x92\x90PV[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(\x0FW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805`\xFF\x81\x16\x81\x14a'=W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a(8W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a(\x82W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80\x825a(\x93\x81a'\x1DV[\x81R` \x83\x015a(\xA3\x81a'\x1DV[` \x82\x01R`@\x83\x015a(\xB6\x81a'\x1DV[`@\x82\x01R``\x83\x015a(\xC9\x81a'\x1DV[``\x82\x01R`\x80\x83\x015a(\xDC\x81a'\x1DV[`\x80\x82\x01R`\xA0\x83\x015a(\xEF\x81a'\x1DV[`\xA0\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8B\x8D\x03a\x02\xA0\x81\x12\x15a)\x1FW`\0\x80\xFD[\x8C5a)*\x81a'\x1DV[\x9BPa)8` \x8E\x01a'fV[\x9APa)F`@\x8E\x01a'fV[\x99P``\x8D\x015\x98Pa)[`\x80\x8E\x01a'zV[\x97P`\xA0\x8D\x015a)k\x81a'\x1DV[\x96P`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a)\x9DW`\0\x80\xFD[Pa)\xA6a'\xC5V[a)\xB2`\xC0\x8E\x01a'fV[\x81Ra)\xC0`\xE0\x8E\x01a(\x15V[` \x82\x01Ra)\xD2a\x01\0\x8E\x01a(\x15V[`@\x82\x01Ra)\xE4a\x01 \x8E\x01a'fV[``\x82\x01Ra)\xF6a\x01@\x8E\x01a'fV[`\x80\x82\x01Ra\x01`\x8D\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a*\x1FW`\0\x80\xFD[`\xA0\x82\x01R\x94Pa*3a\x01\x80\x8D\x01a'2V[\x93Pa*C\x8Da\x01\xA0\x8E\x01a(&V[\x92Pa\x02`\x8C\x015\x91Pa*Za\x02\x80\x8D\x01a'2V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0` \x82\x84\x03\x12\x15a*~W`\0\x80\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a'_W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a*\xA3W`\0\x80\xFD[a*\xAC\x83a'fV[\x91Pa'\xBC` \x84\x01a'fV[`\0` \x82\x84\x03\x12\x15a*\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a*\xF9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a*\xDDV[\x81\x81\x11\x15a+\x0BW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a'_` \x83\x01\x84a*\xD3V[`\0` \x82\x84\x03\x12\x15a+EW`\0\x80\xFD[a'_\x82a'zV[`\0\x80`@\x83\x85\x03\x12\x15a+aW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\npW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a+\x91W`\0\x80\xFD[\x825\x91P` \x83\x015a+\xA3\x81a+pV[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a+\xEFWa+\xEFa+\xAEV[P\x03\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a,,Wa,,a+\xAEV[P\x02\x90V[`\0` \x82\x84\x03\x12\x15a,CW`\0\x80\xFD[\x81Qa'_\x81a'\x1DV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a,qWa,qa+\xAEV[\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a,\x8CW`\0\x80\xFD[\x81Qa'_\x81a+pV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a,qWa,qa+\xAEV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a,\xF4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a-#Wa-#a+\xAEV[\x02\x94\x93PPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106103af5760003560e01c80637e54b8ad116101f4578063d220a9e01161011a578063f2c4bc9e116100ad578063f8c68de01161007c578063f8c68de01461099e578063fd32aa0f146109a6578063fe3d5710146109ae578063ffa1ad74146109e757600080fd5b8063f2c4bc9e1461095b578063f2fde38b1461096e578063f45e65d814610981578063f68016b71461098a57600080fd5b8063e0e2016d116100e9578063e0e2016d14610922578063e81b2c6d1461092a578063ec70751714610933578063f2b4e6171461095357600080fd5b8063d220a9e0146108f9578063d6ae3cd514610909578063dac6e63a14610912578063dad544e01461091a57600080fd5b8063b40a817c11610192578063c4e8ddfa11610161578063c4e8ddfa14610792578063c9b26f611461079a578063c9ff2d16146107ad578063cc731b02146107c557600080fd5b8063b40a817c14610748578063bc49ce5f1461075b578063bfb14fb714610763578063c0fd4b411461077f57600080fd5b80639b7d7f0a116101ce5780639b7d7f0a146106a3578063a39fac12146106ab578063a62611a214610714578063a71198691461074057600080fd5b80637e54b8ad146106775780638da5cb5b1461067f578063935f029e1461069057600080fd5b806321d7fde5116102d95780634add321d116102775780635c975abb116102465780635c975abb1461064c5780635d73369c14610654578063715018a61461065c5780637616f0e81461066457600080fd5b80634add321d146105ad5780634d5d9a2a146105b55780634f16540b146105e657806354fd4d501461060d57600080fd5b80633e47158c116102b35780633e47158c14610572578063452a93201461057a57806347af267b1461058257806348cd4cb1146105a557600080fd5b806321d7fde51461051b57806335e80ab31461052e57806338d38c971461054157600080fd5b806316d3bc7f116103515780631fd19ee1116103205780631fd19ee11461049957806320f06fdc146104a157806321326849146104b4578063215b7a1c1461051357600080fd5b806316d3bc7f1461044757806318d139181461046b57806319f5cea81461047e5780631edd50981461048657600080fd5b80630a49cb031161038d5780630a49cb03146104045780630ae14b1b1461040c5780630c18c1621461042b578063155b6c6f1461043457600080fd5b806306c92657146103b4578063078f29cf146103cf5780630a2ca2a9146103ef575b600080fd5b6103bc6109ef565b6040519081526020015b60405180910390f35b6103d7610a1d565b6040516001600160a01b0390911681526020016103c6565b6104026103fd366004612742565b610a56565b005b6103d7610a73565b631dcd65005b60405167ffffffffffffffff90911681526020016103c6565b6103bc60655481565b610402610442366004612792565b610aa3565b606a54610412906c01000000000000000000000000900467ffffffffffffffff1681565b610402610479366004612742565b610ab9565b6103bc610aca565b6104026104943660046128fc565b610af5565b6103d7610e6a565b6104026104af366004612a6c565b610e94565b7f435553544f4d5f4741535f544f4b454e00000000000000000000000000000000600052606d6020527f3ec9a18bd22a834e2a13465cc2aa2a9aebb161ffdebf39cdb0028dbb1b5394b45460ff165b60405190151581526020016103c6565b6103d7610ea5565b610402610529366004612a90565b610ed5565b606c546103d7906001600160a01b031681565b60405160ff7f00000000000000000000000000000000000000000000000000000000000000001681526020016103c6565b6103d7610ee7565b6103d76110cb565b610503610590366004612aba565b606d6020526000908152604090205460ff1681565b6103bc611152565b610412611182565b606a546105d19068010000000000000000900463ffffffff1681565b60405163ffffffff90911681526020016103c6565b6103bc7f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c0881565b604080518082018252600681527f332e31332e320000000000000000000000000000000000000000000000000000602082015290516103c69190612b20565b6105036111a8565b6103bc61138e565b6104026113b9565b610402610672366004612b33565b6113cd565b6103bc6113de565b6033546001600160a01b03166103d7565b61040261069e366004612b4e565b611409565b6103d761141b565b6106b361144b565b6040516103c69190600060c0820190506001600160a01b038084511683528060208501511660208401528060408501511660408401528060608501511660608401528060808501511660808401528060a08501511660a08401525092915050565b606c546104129074010000000000000000000000000000000000000000900467ffffffffffffffff1681565b6103d761150e565b610402610756366004612b33565b61153e565b6103bc61154f565b6068546105d19068010000000000000000900463ffffffff1681565b61040261078d366004612a90565b61157a565b6103d761158c565b6104026107a8366004612aba565b6115bc565b606a546105d190640100000000900463ffffffff1681565b6108896040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a0810191909152506040805160c08101825260695463ffffffff8082168352640100000000820460ff9081166020850152650100000000008304169383019390935266010000000000008104831660608301526a0100000000000000000000810490921660808201526e0100000000000000000000000000009091046fffffffffffffffffffffffffffffffff1660a082015290565b6040516103c69190600060c08201905063ffffffff80845116835260ff602085015116602084015260ff6040850151166040840152806060850151166060840152806080850151166080840152506fffffffffffffffffffffffffffffffff60a08401511660a083015292915050565b606a546105d19063ffffffff1681565b6103bc606b5481565b6103d76115cd565b6103d76115fd565b6103bc611644565b6103bc60675481565b6068546105d1906c01000000000000000000000000900463ffffffff1681565b6103d761166f565b610402610969366004612b7e565b6116de565b61040261097c366004612742565b6118d5565b6103bc60665481565b6068546104129067ffffffffffffffff1681565b6103bc611962565b6103bc61198d565b606a546109d49074010000000000000000000000000000000000000000900461ffff1681565b60405161ffff90911681526020016103c6565b6103bc600081565b610a1a60017fa04c5bb938ca6fc46d95553abf0a76345ce3e722a30bf4f74928b8e7d852320d612bdd565b81565b6000610a51610a4d60017f9904ba90dde5696cda05c9e0dab5cbaa0fea005ace4d11218a02ac668dad6377612bdd565b5490565b905090565b610a5e6119bc565b610a706001600160a01b038216611a16565b50565b6000610a51610a4d60017f4b6c74f9e688cb39801f2112c14a8c57232a3fc5202e1444126d4bce86eb19ad612bdd565b610aab6119bc565b610ab58282611a77565b5050565b610ac16119bc565b610a7081611b69565b610a1a60017f46adcbebc6be8ce551740c29c47c8798210f23f7f4086c41752944352568d5a8612bdd565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff16158015610b35575060005460ff8083169116105b610bac5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a656400000000000000000000000000000000000060648201526084015b60405180910390fd5b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff831617610100179055610be5611bc2565b610bed611c29565b610bf68c6118d5565b610bff89611a16565b610c098b8b611cae565b610c1288611d95565b610c3b7f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c08889055565b610c6e610c6960017f71ac12829d66ee73d8d95bff50b3589745ce57edae70a3fb111a2342464dc598612bdd565b869055565b610ca2610c9c60017f383f291819e6d54073bc9a648251d97421076bdd101933c0c022219ce9580637612bdd565b85519055565b610cd9610cd060017f46adcbebc6be8ce551740c29c47c8798210f23f7f4086c41752944352568d5a8612bdd565b60208601519055565b610d10610d0760017f9904ba90dde5696cda05c9e0dab5cbaa0fea005ace4d11218a02ac668dad6377612bdd565b60408601519055565b610d47610d3e60017f4b6c74f9e688cb39801f2112c14a8c57232a3fc5202e1444126d4bce86eb19ad612bdd565b60608601519055565b610d7e610d7560017fa04c5bb938ca6fc46d95553abf0a76345ce3e722a30bf4f74928b8e7d852320d612bdd565b60808601519055565b610db5610dac60017f51547f31a231e1007dca33017faa3da20d959b95087c588a7768bfb922fd5900612bdd565b60a08601519055565b610dbd611eb7565b610dc686611f1f565b606b839055606c80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b038416179055600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050505050505050505050565b6000610a517f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c085490565b610e9c6119bc565b610a7081612311565b6000610a51610a4d60017f51547f31a231e1007dca33017faa3da20d959b95087c588a7768bfb922fd5900612bdd565b610edd6119bc565b610ab58282611cae565b600080610f127fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035490565b90506001600160a01b03811615610f2857919050565b6040518060400160405280601a81526020017f4f564d5f4c3143726f7373446f6d61696e4d657373656e676572000000000000815250516002610f6b9190612bf4565b604080513060208201526000918101919091527f4f564d5f4c3143726f7373446f6d61696e4d657373656e6765720000000000009190911790610fc6906060015b604051602081830303815290604052805190602001205490565b14610ffd576040517f54e433cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805130602082015260019181019190915260009061101f90606001610fac565b90506001600160a01b0381161561109957806001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561106e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110929190612c31565b9250505090565b6040517f332144db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b606c54604080517f452a932000000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163452a93209160048083019260209291908290030181865afa15801561112e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a519190612c31565b6000610a51610a4d60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b606954600090610a519063ffffffff6a0100000000000000000000820481169116612c4e565b7f4554485f4c4f434b424f580000000000000000000000000000000000000000006000908152606d6020527f58e88e949bd180ff86d6c072735c3d8d5a05a543c16130176ec0bc0adf3fd80654819060ff1661120b57611206610a73565b611274565b611213610a73565b6001600160a01b031663b682c4446040518163ffffffff1660e01b8152600401602060405180830381865afa158015611250573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112749190612c31565b606c546040517f2e48152c000000000000000000000000000000000000000000000000000000008152600060048201529192506001600160a01b031690632e48152c90602401602060405180830381865afa1580156112d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112fb9190612c7a565b806113885750606c546040517f2e48152c0000000000000000000000000000000000000000000000000000000081526001600160a01b03838116600483015290911690632e48152c90602401602060405180830381865afa158015611364573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113889190612c7a565b91505090565b610a1a60017f383f291819e6d54073bc9a648251d97421076bdd101933c0c022219ce9580637612bdd565b6113c16119bc565b6113cb600061237e565b565b6113d56119bc565b610a70816123e8565b610a1a60017f51547f31a231e1007dca33017faa3da20d959b95087c588a7768bfb922fd5900612bdd565b6114116119bc565b610ab5828261245b565b6000610a51610a4d60017fa04c5bb938ca6fc46d95553abf0a76345ce3e722a30bf4f74928b8e7d852320d612bdd565b6040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a08101919091526040518060c0016040528061149061150e565b6001600160a01b031681526020016114a661158c565b6001600160a01b031681526020016114bc610a1d565b6001600160a01b031681526020016114d2610a73565b6001600160a01b031681526020016114e861141b565b6001600160a01b031681526020016114fe610ea5565b6001600160a01b03169052919050565b6000610a51610a4d60017f383f291819e6d54073bc9a648251d97421076bdd101933c0c022219ce9580637612bdd565b6115466119bc565b610a7081611d95565b610a1a60017f71ac12829d66ee73d8d95bff50b3589745ce57edae70a3fb111a2342464dc598612bdd565b6115826119bc565b610ab58282612517565b6000610a51610a4d60017f46adcbebc6be8ce551740c29c47c8798210f23f7f4086c41752944352568d5a8612bdd565b6115c46119bc565b610a7081611a16565b6000610a51610a4d60017f71ac12829d66ee73d8d95bff50b3589745ce57edae70a3fb111a2342464dc598612bdd565b6000611607610ee7565b6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561112e573d6000803e3d6000fd5b610a1a60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b60008061167a610a73565b9050806001600160a01b031663f2b4e6176040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116ba573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113889190612c31565b6116e6611bc2565b6000828152606d602052604090205460ff16151581151503611734576040517ff5828b0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f4554485f4c4f434b424f58000000000000000000000000000000000000000000820361186b576000828152606d602052604090205460ff168015611777575080155b80156117f557506000611788610a73565b6001600160a01b031663b682c4446040518163ffffffff1660e01b8152600401602060405180830381865afa1580156117c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117e99190612c31565b6001600160a01b031614155b1561182c576040517ff5828b0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6118346111a8565b1561186b576040517ff5828b0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000828152606d602052604080822080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00168415159081179091559051909184917fb876f6594132c89891d2fd198e925e999be741ec809abb58bfe9b966876cc06c9190a35050565b6118dd6119bc565b6001600160a01b0381166119595760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152608401610ba3565b610a708161237e565b610a1a60017f9904ba90dde5696cda05c9e0dab5cbaa0fea005ace4d11218a02ac668dad6377612bdd565b610a1a60017f4b6c74f9e688cb39801f2112c14a8c57232a3fc5202e1444126d4bce86eb19ad612bdd565b9055565b6033546001600160a01b031633146113cb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610ba3565b60678190556040805160208082018490528251808303909101815290820190915260005b60007f1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be83604051611a6b9190612b20565b60405180910390a35050565b606a80547fffffffffffffffffffffffff000000000000000000000000ffffffffffffffff166801000000000000000063ffffffff8516027fffffffffffffffffffffffff0000000000000000ffffffffffffffffffffffff16176c0100000000000000000000000067ffffffffffffffff841690810291909117909155604080516bffffffff000000000000000085831b1690921760208301526000910160408051601f19818403018152919052905060055b60007f1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be83604051611b5c9190612b20565b60405180910390a3505050565b611b927f65a7ed542fb37fe237fdfbdd70b31598523fe5b32879e307bae27a0bd9581c08829055565b604080516001600160a01b03831660208201526000910160408051601f1981840301815291905290506003611a3a565b33611bcb610ee7565b6001600160a01b031614158015611bf2575033611be66115fd565b6001600160a01b031614155b156113cb576040517fc4050a2600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600054610100900460ff16611ca65760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401610ba3565b6113cb612697565b606880547fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff166801000000000000000063ffffffff8581169182027fffffffffffffffffffffffffffffffff00000000ffffffffffffffffffffffff16929092176c0100000000000000000000000092851692909202919091179091557f0100000000000000000000000000000000000000000000000000000000000000602083811b67ffffffff000000001690921717606681905560655460408051938401919091528201526000906060015b60408051601f1981840301815291905290506001611b2b565b611d9d611182565b67ffffffffffffffff168167ffffffffffffffff161015611e005760405162461bcd60e51b815260206004820152601f60248201527f53797374656d436f6e6669673a20676173206c696d697420746f6f206c6f77006044820152606401610ba3565b631dcd650067ffffffffffffffff82161115611e5e5760405162461bcd60e51b815260206004820181905260248201527f53797374656d436f6e6669673a20676173206c696d697420746f6f20686967686044820152606401610ba3565b606880547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff83169081179091556040805160208082019390935281518082039093018352810190526002611a3a565b611ee5610a4d60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b6000036113cb576113cb611f1a60017fa11ee3ab75b40e88a0105e935d17cd36c8faee0138320d776c411291bdbbb1a0612bdd565b439055565b8060a001516fffffffffffffffffffffffffffffffff16816060015163ffffffff161115611fb55760405162461bcd60e51b815260206004820152603560248201527f53797374656d436f6e6669673a206d696e206261736520666565206d7573742060448201527f6265206c657373207468616e206d6178206261736500000000000000000000006064820152608401610ba3565b6001816040015160ff16116120325760405162461bcd60e51b815260206004820152602f60248201527f53797374656d436f6e6669673a2064656e6f6d696e61746f72206d757374206260448201527f65206c6172676572207468616e203100000000000000000000000000000000006064820152608401610ba3565b6068546080820151825167ffffffffffffffff909216916120539190612c97565b63ffffffff1611156120a75760405162461bcd60e51b815260206004820152601f60248201527f53797374656d436f6e6669673a20676173206c696d697420746f6f206c6f77006044820152606401610ba3565b6000816020015160ff16116121245760405162461bcd60e51b815260206004820152602f60248201527f53797374656d436f6e6669673a20656c6173746963697479206d756c7469706c60448201527f6965722063616e6e6f74206265203000000000000000000000000000000000006064820152608401610ba3565b8051602082015163ffffffff82169160ff90911690612144908290612cb6565b61214e9190612d00565b63ffffffff16146121c75760405162461bcd60e51b815260206004820152603760248201527f53797374656d436f6e6669673a20707265636973696f6e206c6f73732077697460448201527f6820746172676574207265736f75726365206c696d69740000000000000000006064820152608401610ba3565b805160698054602084015160408501516060860151608087015160a09097015163ffffffff9687167fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009095169490941764010000000060ff94851602177fffffffffffffffffffffffffffffffffffffffffffff0000000000ffffffffff166501000000000093909216929092027fffffffffffffffffffffffffffffffffffffffffffff00000000ffffffffffff1617660100000000000091851691909102177fffff0000000000000000000000000000000000000000ffffffffffffffffffff166a010000000000000000000093909416929092027fffff00000000000000000000000000000000ffffffffffffffffffffffffffff16929092176e0100000000000000000000000000006fffffffffffffffffffffffffffffffff90921691909102179055565b606a80547fffffffffffffffffffff0000ffffffffffffffffffffffffffffffffffffffff167401000000000000000000000000000000000000000061ffff8416908102919091179091556040805160208082019390935281518082039093018352810190526007611a3a565b603380546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b606c80547fffffffff0000000000000000ffffffffffffffffffffffffffffffffffffffff167401000000000000000000000000000000000000000067ffffffffffffffff8416908102919091179091556040805160208082019390935281518082039093018352810190526006611a3a565b7fff000000000000000000000000000000000000000000000000000000000000008116156124f15760405162461bcd60e51b815260206004820152602160248201527f53797374656d436f6e6669673a207363616c61722065786365656473206d617860448201527f2e000000000000000000000000000000000000000000000000000000000000006064820152608401610ba3565b606582905560668190556040805160208101849052908101829052600090606001611d7c565b60018263ffffffff1610156125945760405162461bcd60e51b815260206004820152602660248201527f53797374656d436f6e6669673a2064656e6f6d696e61746f72206d757374206260448201527f65203e3d203100000000000000000000000000000000000000000000000000006064820152608401610ba3565b60018163ffffffff1610156126115760405162461bcd60e51b815260206004820152602560248201527f53797374656d436f6e6669673a20656c6173746963697479206d75737420626560448201527f203e3d20310000000000000000000000000000000000000000000000000000006064820152608401610ba3565b606a805463ffffffff83811664010000000081027fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000909316918616919091179190911790915560405160009161267e91602086811b67ffffffff0000000016909217910190815260200190565b60408051601f1981840301815291905290506004611b2b565b600054610100900460ff166127145760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401610ba3565b6113cb3361237e565b6001600160a01b0381168114610a7057600080fd5b803561273d8161271d565b919050565b60006020828403121561275457600080fd5b813561275f8161271d565b9392505050565b803563ffffffff8116811461273d57600080fd5b803567ffffffffffffffff8116811461273d57600080fd5b600080604083850312156127a557600080fd5b6127ae83612766565b91506127bc6020840161277a565b90509250929050565b60405160c0810167ffffffffffffffff8111828210171561280f577f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60405290565b803560ff8116811461273d57600080fd5b600060c0828403121561283857600080fd5b60405160c0810181811067ffffffffffffffff82111715612882577f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60405290508082356128938161271d565b815260208301356128a38161271d565b602082015260408301356128b68161271d565b604082015260608301356128c98161271d565b606082015260808301356128dc8161271d565b608082015260a08301356128ef8161271d565b60a0919091015292915050565b60008060008060008060008060008060008b8d036102a081121561291f57600080fd5b8c3561292a8161271d565b9b5061293860208e01612766565b9a5061294660408e01612766565b995060608d0135985061295b60808e0161277a565b975060a08d013561296b8161271d565b965060c07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408201121561299d57600080fd5b506129a66127c5565b6129b260c08e01612766565b81526129c060e08e01612815565b60208201526129d26101008e01612815565b60408201526129e46101208e01612766565b60608201526129f66101408e01612766565b60808201526101608d01356fffffffffffffffffffffffffffffffff81168114612a1f57600080fd5b60a08201529450612a336101808d01612732565b9350612a438d6101a08e01612826565b92506102608c01359150612a5a6102808d01612732565b90509295989b509295989b9093969950565b600060208284031215612a7e57600080fd5b813561ffff8116811461275f57600080fd5b60008060408385031215612aa357600080fd5b612aac83612766565b91506127bc60208401612766565b600060208284031215612acc57600080fd5b5035919050565b6000815180845260005b81811015612af957602081850181015186830182015201612add565b81811115612b0b576000602083870101525b50601f01601f19169290920160200192915050565b60208152600061275f6020830184612ad3565b600060208284031215612b4557600080fd5b61275f8261277a565b60008060408385031215612b6157600080fd5b50508035926020909101359150565b8015158114610a7057600080fd5b60008060408385031215612b9157600080fd5b823591506020830135612ba381612b70565b809150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600082821015612bef57612bef612bae565b500390565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0483118215151615612c2c57612c2c612bae565b500290565b600060208284031215612c4357600080fd5b815161275f8161271d565b600067ffffffffffffffff808316818516808303821115612c7157612c71612bae565b01949350505050565b600060208284031215612c8c57600080fd5b815161275f81612b70565b600063ffffffff808316818516808303821115612c7157612c71612bae565b600063ffffffff80841680612cf4577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b92169190910492915050565b600063ffffffff80831681851681830481118215151615612d2357612d23612bae565b0294935050505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xAFW`\x005`\xE0\x1C\x80c~T\xB8\xAD\x11a\x01\xF4W\x80c\xD2 \xA9\xE0\x11a\x01\x1AW\x80c\xF2\xC4\xBC\x9E\x11a\0\xADW\x80c\xF8\xC6\x8D\xE0\x11a\0|W\x80c\xF8\xC6\x8D\xE0\x14a\t\x9EW\x80c\xFD2\xAA\x0F\x14a\t\xA6W\x80c\xFE=W\x10\x14a\t\xAEW\x80c\xFF\xA1\xADt\x14a\t\xE7W`\0\x80\xFD[\x80c\xF2\xC4\xBC\x9E\x14a\t[W\x80c\xF2\xFD\xE3\x8B\x14a\tnW\x80c\xF4^e\xD8\x14a\t\x81W\x80c\xF6\x80\x16\xB7\x14a\t\x8AW`\0\x80\xFD[\x80c\xE0\xE2\x01m\x11a\0\xE9W\x80c\xE0\xE2\x01m\x14a\t\"W\x80c\xE8\x1B,m\x14a\t*W\x80c\xECpu\x17\x14a\t3W\x80c\xF2\xB4\xE6\x17\x14a\tSW`\0\x80\xFD[\x80c\xD2 \xA9\xE0\x14a\x08\xF9W\x80c\xD6\xAE<\xD5\x14a\t\tW\x80c\xDA\xC6\xE6:\x14a\t\x12W\x80c\xDA\xD5D\xE0\x14a\t\x1AW`\0\x80\xFD[\x80c\xB4\n\x81|\x11a\x01\x92W\x80c\xC4\xE8\xDD\xFA\x11a\x01aW\x80c\xC4\xE8\xDD\xFA\x14a\x07\x92W\x80c\xC9\xB2oa\x14a\x07\x9AW\x80c\xC9\xFF-\x16\x14a\x07\xADW\x80c\xCCs\x1B\x02\x14a\x07\xC5W`\0\x80\xFD[\x80c\xB4\n\x81|\x14a\x07HW\x80c\xBCI\xCE_\x14a\x07[W\x80c\xBF\xB1O\xB7\x14a\x07cW\x80c\xC0\xFDKA\x14a\x07\x7FW`\0\x80\xFD[\x80c\x9B}\x7F\n\x11a\x01\xCEW\x80c\x9B}\x7F\n\x14a\x06\xA3W\x80c\xA3\x9F\xAC\x12\x14a\x06\xABW\x80c\xA6&\x11\xA2\x14a\x07\x14W\x80c\xA7\x11\x98i\x14a\x07@W`\0\x80\xFD[\x80c~T\xB8\xAD\x14a\x06wW\x80c\x8D\xA5\xCB[\x14a\x06\x7FW\x80c\x93_\x02\x9E\x14a\x06\x90W`\0\x80\xFD[\x80c!\xD7\xFD\xE5\x11a\x02\xD9W\x80cJ\xDD2\x1D\x11a\x02wW\x80c\\\x97Z\xBB\x11a\x02FW\x80c\\\x97Z\xBB\x14a\x06LW\x80c]s6\x9C\x14a\x06TW\x80cqP\x18\xA6\x14a\x06\\W\x80cv\x16\xF0\xE8\x14a\x06dW`\0\x80\xFD[\x80cJ\xDD2\x1D\x14a\x05\xADW\x80cM]\x9A*\x14a\x05\xB5W\x80cO\x16T\x0B\x14a\x05\xE6W\x80cT\xFDMP\x14a\x06\rW`\0\x80\xFD[\x80c>G\x15\x8C\x11a\x02\xB3W\x80c>G\x15\x8C\x14a\x05rW\x80cE*\x93 \x14a\x05zW\x80cG\xAF&{\x14a\x05\x82W\x80cH\xCDL\xB1\x14a\x05\xA5W`\0\x80\xFD[\x80c!\xD7\xFD\xE5\x14a\x05\x1BW\x80c5\xE8\n\xB3\x14a\x05.W\x80c8\xD3\x8C\x97\x14a\x05AW`\0\x80\xFD[\x80c\x16\xD3\xBC\x7F\x11a\x03QW\x80c\x1F\xD1\x9E\xE1\x11a\x03 W\x80c\x1F\xD1\x9E\xE1\x14a\x04\x99W\x80c \xF0o\xDC\x14a\x04\xA1W\x80c!2hI\x14a\x04\xB4W\x80c![z\x1C\x14a\x05\x13W`\0\x80\xFD[\x80c\x16\xD3\xBC\x7F\x14a\x04GW\x80c\x18\xD19\x18\x14a\x04kW\x80c\x19\xF5\xCE\xA8\x14a\x04~W\x80c\x1E\xDDP\x98\x14a\x04\x86W`\0\x80\xFD[\x80c\nI\xCB\x03\x11a\x03\x8DW\x80c\nI\xCB\x03\x14a\x04\x04W\x80c\n\xE1K\x1B\x14a\x04\x0CW\x80c\x0C\x18\xC1b\x14a\x04+W\x80c\x15[lo\x14a\x044W`\0\x80\xFD[\x80c\x06\xC9&W\x14a\x03\xB4W\x80c\x07\x8F)\xCF\x14a\x03\xCFW\x80c\n,\xA2\xA9\x14a\x03\xEFW[`\0\x80\xFD[a\x03\xBCa\t\xEFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xD7a\n\x1DV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x04\x02a\x03\xFD6`\x04a'BV[a\nVV[\0[a\x03\xD7a\nsV[c\x1D\xCDe\0[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x03\xBC`eT\x81V[a\x04\x02a\x04B6`\x04a'\x92V[a\n\xA3V[`jTa\x04\x12\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x02a\x04y6`\x04a'BV[a\n\xB9V[a\x03\xBCa\n\xCAV[a\x04\x02a\x04\x946`\x04a(\xFCV[a\n\xF5V[a\x03\xD7a\x0EjV[a\x04\x02a\x04\xAF6`\x04a*lV[a\x0E\x94V[\x7FCUSTOM_GAS_TOKEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`m` R\x7F>\xC9\xA1\x8B\xD2*\x83N*\x13F\\\xC2\xAA*\x9A\xEB\xB1a\xFF\xDE\xBF9\xCD\xB0\x02\x8D\xBB\x1BS\x94\xB4T`\xFF\x16[`@Q\x90\x15\x15\x81R` \x01a\x03\xC6V[a\x03\xD7a\x0E\xA5V[a\x04\x02a\x05)6`\x04a*\x90V[a\x0E\xD5V[`lTa\x03\xD7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x03\xC6V[a\x03\xD7a\x0E\xE7V[a\x03\xD7a\x10\xCBV[a\x05\x03a\x05\x906`\x04a*\xBAV[`m` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\xBCa\x11RV[a\x04\x12a\x11\x82V[`jTa\x05\xD1\x90h\x01\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x03\xBC\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x81V[`@\x80Q\x80\x82\x01\x82R`\x06\x81R\x7F3.13.2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x03\xC6\x91\x90a+ V[a\x05\x03a\x11\xA8V[a\x03\xBCa\x13\x8EV[a\x04\x02a\x13\xB9V[a\x04\x02a\x06r6`\x04a+3V[a\x13\xCDV[a\x03\xBCa\x13\xDEV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD7V[a\x04\x02a\x06\x9E6`\x04a+NV[a\x14\tV[a\x03\xD7a\x14\x1BV[a\x06\xB3a\x14KV[`@Qa\x03\xC6\x91\x90`\0`\xC0\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01R\x80`\xA0\x85\x01Q\x16`\xA0\x84\x01RP\x92\x91PPV[`lTa\x04\x12\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xD7a\x15\x0EV[a\x04\x02a\x07V6`\x04a+3V[a\x15>V[a\x03\xBCa\x15OV[`hTa\x05\xD1\x90h\x01\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x02a\x07\x8D6`\x04a*\x90V[a\x15zV[a\x03\xD7a\x15\x8CV[a\x04\x02a\x07\xA86`\x04a*\xBAV[a\x15\xBCV[`jTa\x05\xD1\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x08\x89`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`iTc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[`@Qa\x03\xC6\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`jTa\x05\xD1\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xBC`kT\x81V[a\x03\xD7a\x15\xCDV[a\x03\xD7a\x15\xFDV[a\x03\xBCa\x16DV[a\x03\xBC`gT\x81V[`hTa\x05\xD1\x90l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xD7a\x16oV[a\x04\x02a\ti6`\x04a+~V[a\x16\xDEV[a\x04\x02a\t|6`\x04a'BV[a\x18\xD5V[a\x03\xBC`fT\x81V[`hTa\x04\x12\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xBCa\x19bV[a\x03\xBCa\x19\x8DV[`jTa\t\xD4\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xC6V[a\x03\xBC`\0\x81V[a\n\x1A`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra+\xDDV[\x81V[`\0a\nQa\nM`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa+\xDDV[T\x90V[\x90P\x90V[a\n^a\x19\xBCV[a\np`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1A\x16V[PV[`\0a\nQa\nM`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa+\xDDV[a\n\xABa\x19\xBCV[a\n\xB5\x82\x82a\x1AwV[PPV[a\n\xC1a\x19\xBCV[a\np\x81a\x1BiV[a\n\x1A`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a+\xDDV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x0B5WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x0B\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x0B\xE5a\x1B\xC2V[a\x0B\xEDa\x1C)V[a\x0B\xF6\x8Ca\x18\xD5V[a\x0B\xFF\x89a\x1A\x16V[a\x0C\t\x8B\x8Ba\x1C\xAEV[a\x0C\x12\x88a\x1D\x95V[a\x0C;\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x88\x90UV[a\x0Cna\x0Ci`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a+\xDDV[\x86\x90UV[a\x0C\xA2a\x0C\x9C`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a+\xDDV[\x85Q\x90UV[a\x0C\xD9a\x0C\xD0`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a+\xDDV[` \x86\x01Q\x90UV[a\r\x10a\r\x07`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa+\xDDV[`@\x86\x01Q\x90UV[a\rGa\r>`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa+\xDDV[``\x86\x01Q\x90UV[a\r~a\ru`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra+\xDDV[`\x80\x86\x01Q\x90UV[a\r\xB5a\r\xAC`\x01\x7FQT\x7F1\xA21\xE1\0}\xCA3\x01\x7F\xAA=\xA2\r\x95\x9B\x95\x08|X\x8Awh\xBF\xB9\"\xFDY\0a+\xDDV[`\xA0\x86\x01Q\x90UV[a\r\xBDa\x1E\xB7V[a\r\xC6\x86a\x1F\x1FV[`k\x83\x90U`l\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPPV[`\0a\nQ\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08T\x90V[a\x0E\x9Ca\x19\xBCV[a\np\x81a#\x11V[`\0a\nQa\nM`\x01\x7FQT\x7F1\xA21\xE1\0}\xCA3\x01\x7F\xAA=\xA2\r\x95\x9B\x95\x08|X\x8Awh\xBF\xB9\"\xFDY\0a+\xDDV[a\x0E\xDDa\x19\xBCV[a\n\xB5\x82\x82a\x1C\xAEV[`\0\x80a\x0F\x12\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0F(W\x91\x90PV[`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x81RPQ`\x02a\x0Fk\x91\x90a+\xF4V[`@\x80Q0` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x91\x90\x91\x17\x90a\x0F\xC6\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 T\x90V[\x14a\x0F\xFDW`@Q\x7FT\xE43\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R`\0\x90a\x10\x1F\x90``\x01a\x0F\xACV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x10\x99W\x80`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x92\x91\x90a,1V[\x92PPP\x90V[`@Q\x7F3!D\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`lT`@\x80Q\x7FE*\x93 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cE*\x93 \x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x11.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nQ\x91\x90a,1V[`\0a\nQa\nM`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[`iT`\0\x90a\nQ\x90c\xFF\xFF\xFF\xFFj\x01\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16a,NV[\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x90\x81R`m` R\x7FX\xE8\x8E\x94\x9B\xD1\x80\xFF\x86\xD6\xC0rs\\=\x8DZ\x05\xA5C\xC1a0\x17n\xC0\xBC\n\xDF?\xD8\x06T\x81\x90`\xFF\x16a\x12\x0BWa\x12\x06a\nsV[a\x12tV[a\x12\x13a\nsV[`\x01`\x01`\xA0\x1B\x03\x16c\xB6\x82\xC4D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12t\x91\x90a,1V[`lT`@Q\x7F.H\x15,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c.H\x15,\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFB\x91\x90a,zV[\x80a\x13\x88WP`lT`@Q\x7F.H\x15,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c.H\x15,\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x88\x91\x90a,zV[\x91PP\x90V[a\n\x1A`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a+\xDDV[a\x13\xC1a\x19\xBCV[a\x13\xCB`\0a#~V[V[a\x13\xD5a\x19\xBCV[a\np\x81a#\xE8V[a\n\x1A`\x01\x7FQT\x7F1\xA21\xE1\0}\xCA3\x01\x7F\xAA=\xA2\r\x95\x9B\x95\x08|X\x8Awh\xBF\xB9\"\xFDY\0a+\xDDV[a\x14\x11a\x19\xBCV[a\n\xB5\x82\x82a$[V[`\0a\nQa\nM`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra+\xDDV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R`@Q\x80`\xC0\x01`@R\x80a\x14\x90a\x15\x0EV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xA6a\x15\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xBCa\n\x1DV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xD2a\nsV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xE8a\x14\x1BV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x14\xFEa\x0E\xA5V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x91\x90PV[`\0a\nQa\nM`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a+\xDDV[a\x15Fa\x19\xBCV[a\np\x81a\x1D\x95V[a\n\x1A`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a+\xDDV[a\x15\x82a\x19\xBCV[a\n\xB5\x82\x82a%\x17V[`\0a\nQa\nM`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a+\xDDV[a\x15\xC4a\x19\xBCV[a\np\x81a\x1A\x16V[`\0a\nQa\nM`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a+\xDDV[`\0a\x16\x07a\x0E\xE7V[`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11.W=`\0\x80>=`\0\xFD[a\n\x1A`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[`\0\x80a\x16za\nsV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xB4\xE6\x17`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x88\x91\x90a,1V[a\x16\xE6a\x1B\xC2V[`\0\x82\x81R`m` R`@\x90 T`\xFF\x16\x15\x15\x81\x15\x15\x03a\x174W`@Q\x7F\xF5\x82\x8B\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x18kW`\0\x82\x81R`m` R`@\x90 T`\xFF\x16\x80\x15a\x17wWP\x80\x15[\x80\x15a\x17\xF5WP`\0a\x17\x88a\nsV[`\x01`\x01`\xA0\x1B\x03\x16c\xB6\x82\xC4D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE9\x91\x90a,1V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x18,W`@Q\x7F\xF5\x82\x8B\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x184a\x11\xA8V[\x15a\x18kW`@Q\x7F\xF5\x82\x8B\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`m` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x84\x15\x15\x90\x81\x17\x90\x91U\x90Q\x90\x91\x84\x91\x7F\xB8v\xF6YA2\xC8\x98\x91\xD2\xFD\x19\x8E\x92^\x99\x9B\xE7A\xEC\x80\x9A\xBBX\xBF\xE9\xB9f\x87l\xC0l\x91\x90\xA3PPV[a\x18\xDDa\x19\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[a\np\x81a#~V[a\n\x1A`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa+\xDDV[a\n\x1A`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa+\xDDV[\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\xA3V[`g\x81\x90U`@\x80Q` \x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x90\x82\x01\x90\x91R`\0[`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x1Ak\x91\x90a+ V[`@Q\x80\x91\x03\x90\xA3PPV[`j\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x85\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@\x80Qk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x85\x83\x1B\x16\x90\x92\x17` \x83\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x05[`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x1B\\\x91\x90a+ V[`@Q\x80\x91\x03\x90\xA3PPPV[a\x1B\x92\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x82\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x03a\x1A:V[3a\x1B\xCBa\x0E\xE7V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x1B\xF2WP3a\x1B\xE6a\x15\xFDV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x13\xCBW`@Q\x7F\xC4\x05\n&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1C\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[a\x13\xCBa&\x97V[`h\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x85\x81\x16\x91\x82\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x92\x85\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90\x91U\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x81\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x90\x92\x17\x17`f\x81\x90U`eT`@\x80Q\x93\x84\x01\x91\x90\x91R\x82\x01R`\0\x90``\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x01a\x1B+V[a\x1D\x9Da\x11\x82V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x0B\xA3V[c\x1D\xCDe\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a\x1E^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FSystemConfig: gas limit too high`D\x82\x01R`d\x01a\x0B\xA3V[`h\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x02a\x1A:V[a\x1E\xE5a\nM`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[`\0\x03a\x13\xCBWa\x13\xCBa\x1F\x1A`\x01\x7F\xA1\x1E\xE3\xABu\xB4\x0E\x88\xA0\x10^\x93]\x17\xCD6\xC8\xFA\xEE\x0182\rwlA\x12\x91\xBD\xBB\xB1\xA0a+\xDDV[C\x90UV[\x80`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FSystemConfig: min base fee must `D\x82\x01R\x7Fbe less than max base\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`\x01\x81`@\x01Q`\xFF\x16\x11a 2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: denominator must b`D\x82\x01R\x7Fe larger than 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`hT`\x80\x82\x01Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91a S\x91\x90a,\x97V[c\xFF\xFF\xFF\xFF\x16\x11\x15a \xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x0B\xA3V[`\0\x81` \x01Q`\xFF\x16\x11a!$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: elasticity multipl`D\x82\x01R\x7Fier cannot be 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[\x80Q` \x82\x01Qc\xFF\xFF\xFF\xFF\x82\x16\x91`\xFF\x90\x91\x16\x90a!D\x90\x82\x90a,\xB6V[a!N\x91\x90a-\0V[c\xFF\xFF\xFF\xFF\x16\x14a!\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FSystemConfig: precision loss wit`D\x82\x01R\x7Fh target resource limit\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[\x80Q`i\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x02\x17\x90UV[`j\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x07a\x1A:V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`l\x80T\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x06a\x1A:V[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x15a$\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FSystemConfig: scalar exceeds max`D\x82\x01R\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`e\x82\x90U`f\x81\x90U`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\0\x90``\x01a\x1D|V[`\x01\x82c\xFF\xFF\xFF\xFF\x16\x10\x15a%\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSystemConfig: denominator must b`D\x82\x01R\x7Fe >= 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`\x01\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a&\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FSystemConfig: elasticity must be`D\x82\x01R\x7F >= 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`j\x80Tc\xFF\xFF\xFF\xFF\x83\x81\x16d\x01\0\0\0\0\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x90\x93\x16\x91\x86\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90\x91U`@Q`\0\x91a&~\x91` \x86\x81\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x90\x92\x17\x91\x01\x90\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x04a\x1B+V[`\0Ta\x01\0\x90\x04`\xFF\x16a'\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[a\x13\xCB3a#~V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\npW`\0\x80\xFD[\x805a'=\x81a'\x1DV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a'TW`\0\x80\xFD[\x815a'_\x81a'\x1DV[\x93\x92PPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'=W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'=W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'\xA5W`\0\x80\xFD[a'\xAE\x83a'fV[\x91Pa'\xBC` \x84\x01a'zV[\x90P\x92P\x92\x90PV[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(\x0FW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805`\xFF\x81\x16\x81\x14a'=W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a(8W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a(\x82W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80\x825a(\x93\x81a'\x1DV[\x81R` \x83\x015a(\xA3\x81a'\x1DV[` \x82\x01R`@\x83\x015a(\xB6\x81a'\x1DV[`@\x82\x01R``\x83\x015a(\xC9\x81a'\x1DV[``\x82\x01R`\x80\x83\x015a(\xDC\x81a'\x1DV[`\x80\x82\x01R`\xA0\x83\x015a(\xEF\x81a'\x1DV[`\xA0\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x8B\x8D\x03a\x02\xA0\x81\x12\x15a)\x1FW`\0\x80\xFD[\x8C5a)*\x81a'\x1DV[\x9BPa)8` \x8E\x01a'fV[\x9APa)F`@\x8E\x01a'fV[\x99P``\x8D\x015\x98Pa)[`\x80\x8E\x01a'zV[\x97P`\xA0\x8D\x015a)k\x81a'\x1DV[\x96P`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a)\x9DW`\0\x80\xFD[Pa)\xA6a'\xC5V[a)\xB2`\xC0\x8E\x01a'fV[\x81Ra)\xC0`\xE0\x8E\x01a(\x15V[` \x82\x01Ra)\xD2a\x01\0\x8E\x01a(\x15V[`@\x82\x01Ra)\xE4a\x01 \x8E\x01a'fV[``\x82\x01Ra)\xF6a\x01@\x8E\x01a'fV[`\x80\x82\x01Ra\x01`\x8D\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a*\x1FW`\0\x80\xFD[`\xA0\x82\x01R\x94Pa*3a\x01\x80\x8D\x01a'2V[\x93Pa*C\x8Da\x01\xA0\x8E\x01a(&V[\x92Pa\x02`\x8C\x015\x91Pa*Za\x02\x80\x8D\x01a'2V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0` \x82\x84\x03\x12\x15a*~W`\0\x80\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a'_W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a*\xA3W`\0\x80\xFD[a*\xAC\x83a'fV[\x91Pa'\xBC` \x84\x01a'fV[`\0` \x82\x84\x03\x12\x15a*\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a*\xF9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a*\xDDV[\x81\x81\x11\x15a+\x0BW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a'_` \x83\x01\x84a*\xD3V[`\0` \x82\x84\x03\x12\x15a+EW`\0\x80\xFD[a'_\x82a'zV[`\0\x80`@\x83\x85\x03\x12\x15a+aW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\npW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a+\x91W`\0\x80\xFD[\x825\x91P` \x83\x015a+\xA3\x81a+pV[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a+\xEFWa+\xEFa+\xAEV[P\x03\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a,,Wa,,a+\xAEV[P\x02\x90V[`\0` \x82\x84\x03\x12\x15a,CW`\0\x80\xFD[\x81Qa'_\x81a'\x1DV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a,qWa,qa+\xAEV[\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a,\x8CW`\0\x80\xFD[\x81Qa'_\x81a+pV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a,qWa,qa+\xAEV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a,\xF4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a-#Wa-#a+\xAEV[\x02\x94\x93PPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UpdateType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<UpdateType> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl UpdateType {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for UpdateType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<UpdateType> for u8 {
            fn from(value: UpdateType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for UpdateType {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UpdateType {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Addresses { address l1CrossDomainMessenger; address l1ERC721Bridge; address l1StandardBridge; address optimismPortal; address optimismMintableERC20Factory; address delayedWETH; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Addresses {
        #[allow(missing_docs)]
        pub l1CrossDomainMessenger: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1ERC721Bridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1StandardBridge: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub optimismPortal: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub optimismMintableERC20Factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayedWETH: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Addresses> for UnderlyingRustTuple<'_> {
            fn from(value: Addresses) -> Self {
                (
                    value.l1CrossDomainMessenger,
                    value.l1ERC721Bridge,
                    value.l1StandardBridge,
                    value.optimismPortal,
                    value.optimismMintableERC20Factory,
                    value.delayedWETH,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Addresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    l1CrossDomainMessenger: tuple.0,
                    l1ERC721Bridge: tuple.1,
                    l1StandardBridge: tuple.2,
                    optimismPortal: tuple.3,
                    optimismMintableERC20Factory: tuple.4,
                    delayedWETH: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Addresses {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Addresses {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1CrossDomainMessenger,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1ERC721Bridge,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1StandardBridge,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.optimismPortal,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.optimismMintableERC20Factory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWETH,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Addresses {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Addresses {
            const NAME: &'static str = "Addresses";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Addresses(address l1CrossDomainMessenger,address l1ERC721Bridge,address l1StandardBridge,address optimismPortal,address optimismMintableERC20Factory,address delayedWETH)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1CrossDomainMessenger,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1ERC721Bridge,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1StandardBridge,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.optimismPortal,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.optimismMintableERC20Factory,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedWETH,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Addresses {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1CrossDomainMessenger,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1ERC721Bridge,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1StandardBridge,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimismPortal,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimismMintableERC20Factory,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWETH,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1CrossDomainMessenger,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1ERC721Bridge,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1StandardBridge,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.optimismPortal,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.optimismMintableERC20Factory,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedWETH,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProxyAdminOwnedBase_NotProxyAdmin()` and selector `0xe818dcc3`.
```solidity
error ProxyAdminOwnedBase_NotProxyAdmin();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProxyAdminOwnedBase_NotProxyAdmin;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProxyAdminOwnedBase_NotProxyAdmin>
        for UnderlyingRustTuple<'_> {
            fn from(value: ProxyAdminOwnedBase_NotProxyAdmin) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ProxyAdminOwnedBase_NotProxyAdmin {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProxyAdminOwnedBase_NotProxyAdmin {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProxyAdminOwnedBase_NotProxyAdmin()";
            const SELECTOR: [u8; 4] = [232u8, 24u8, 220u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner()` and selector `0xc4050a26`.
```solidity
error ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError
        for ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner()";
            const SELECTOR: [u8; 4] = [196u8, 5u8, 10u8, 38u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProxyAdminOwnedBase_NotProxyAdminOwner()` and selector `0x7f12c64b`.
```solidity
error ProxyAdminOwnedBase_NotProxyAdminOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProxyAdminOwnedBase_NotProxyAdminOwner;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProxyAdminOwnedBase_NotProxyAdminOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: ProxyAdminOwnedBase_NotProxyAdminOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ProxyAdminOwnedBase_NotProxyAdminOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProxyAdminOwnedBase_NotProxyAdminOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProxyAdminOwnedBase_NotProxyAdminOwner()";
            const SELECTOR: [u8; 4] = [127u8, 18u8, 198u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProxyAdminOwnedBase_NotResolvedDelegateProxy()` and selector `0x54e433cd`.
```solidity
error ProxyAdminOwnedBase_NotResolvedDelegateProxy();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProxyAdminOwnedBase_NotResolvedDelegateProxy;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProxyAdminOwnedBase_NotResolvedDelegateProxy>
        for UnderlyingRustTuple<'_> {
            fn from(value: ProxyAdminOwnedBase_NotResolvedDelegateProxy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ProxyAdminOwnedBase_NotResolvedDelegateProxy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProxyAdminOwnedBase_NotResolvedDelegateProxy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProxyAdminOwnedBase_NotResolvedDelegateProxy()";
            const SELECTOR: [u8; 4] = [84u8, 228u8, 51u8, 205u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProxyAdminOwnedBase_NotSharedProxyAdminOwner()` and selector `0x075c4314`.
```solidity
error ProxyAdminOwnedBase_NotSharedProxyAdminOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProxyAdminOwnedBase_NotSharedProxyAdminOwner;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProxyAdminOwnedBase_NotSharedProxyAdminOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: ProxyAdminOwnedBase_NotSharedProxyAdminOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ProxyAdminOwnedBase_NotSharedProxyAdminOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProxyAdminOwnedBase_NotSharedProxyAdminOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProxyAdminOwnedBase_NotSharedProxyAdminOwner()";
            const SELECTOR: [u8; 4] = [7u8, 92u8, 67u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProxyAdminOwnedBase_ProxyAdminNotFound()` and selector `0x332144db`.
```solidity
error ProxyAdminOwnedBase_ProxyAdminNotFound();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProxyAdminOwnedBase_ProxyAdminNotFound;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProxyAdminOwnedBase_ProxyAdminNotFound>
        for UnderlyingRustTuple<'_> {
            fn from(value: ProxyAdminOwnedBase_ProxyAdminNotFound) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ProxyAdminOwnedBase_ProxyAdminNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProxyAdminOwnedBase_ProxyAdminNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProxyAdminOwnedBase_ProxyAdminNotFound()";
            const SELECTOR: [u8; 4] = [51u8, 33u8, 68u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ReinitializableBase_ZeroInitVersion()` and selector `0x9b01afed`.
```solidity
error ReinitializableBase_ZeroInitVersion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReinitializableBase_ZeroInitVersion;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReinitializableBase_ZeroInitVersion>
        for UnderlyingRustTuple<'_> {
            fn from(value: ReinitializableBase_ZeroInitVersion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ReinitializableBase_ZeroInitVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReinitializableBase_ZeroInitVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReinitializableBase_ZeroInitVersion()";
            const SELECTOR: [u8; 4] = [155u8, 1u8, 175u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `SystemConfig_InvalidFeatureState()` and selector `0xf5828b04`.
```solidity
error SystemConfig_InvalidFeatureState();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SystemConfig_InvalidFeatureState;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SystemConfig_InvalidFeatureState>
        for UnderlyingRustTuple<'_> {
            fn from(value: SystemConfig_InvalidFeatureState) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SystemConfig_InvalidFeatureState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SystemConfig_InvalidFeatureState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SystemConfig_InvalidFeatureState()";
            const SELECTOR: [u8; 4] = [245u8, 130u8, 139u8, 4u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ConfigUpdate(uint256,uint8,bytes)` and selector `0x1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be`.
```solidity
event ConfigUpdate(uint256 indexed version, UpdateType indexed updateType, bytes data);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ConfigUpdate {
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub updateType: <UpdateType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ConfigUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                UpdateType,
            );
            const SIGNATURE: &'static str = "ConfigUpdate(uint256,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                29u8, 43u8, 11u8, 218u8, 33u8, 213u8, 107u8, 139u8, 209u8, 45u8, 79u8,
                148u8, 235u8, 172u8, 255u8, 223u8, 179u8, 95u8, 94u8, 34u8, 111u8, 132u8,
                180u8, 97u8, 16u8, 59u8, 184u8, 190u8, 171u8, 99u8, 83u8, 190u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    version: topics.1,
                    updateType: topics.2,
                    data: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.version.clone(),
                    self.updateType.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.version);
                out[2usize] = <UpdateType as alloy_sol_types::EventTopic>::encode_topic(
                    &self.updateType,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ConfigUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ConfigUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ConfigUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `FeatureSet(bytes32,bool)` and selector `0xb876f6594132c89891d2fd198e925e999be741ec809abb58bfe9b966876cc06c`.
```solidity
event FeatureSet(bytes32 indexed feature, bool indexed enabled);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FeatureSet {
        #[allow(missing_docs)]
        pub feature: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub enabled: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for FeatureSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            const SIGNATURE: &'static str = "FeatureSet(bytes32,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                184u8, 118u8, 246u8, 89u8, 65u8, 50u8, 200u8, 152u8, 145u8, 210u8, 253u8,
                25u8, 142u8, 146u8, 94u8, 153u8, 155u8, 231u8, 65u8, 236u8, 128u8, 154u8,
                187u8, 88u8, 191u8, 233u8, 185u8, 102u8, 135u8, 108u8, 192u8, 108u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    feature: topics.1,
                    enabled: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.feature.clone(), self.enabled.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.feature);
                out[2usize] = <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic(
                    &self.enabled,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FeatureSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FeatureSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FeatureSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `BATCH_INBOX_SLOT()` and selector `0xbc49ce5f`.
```solidity
function BATCH_INBOX_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BATCH_INBOX_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`BATCH_INBOX_SLOT()`](BATCH_INBOX_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BATCH_INBOX_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BATCH_INBOX_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: BATCH_INBOX_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BATCH_INBOX_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BATCH_INBOX_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: BATCH_INBOX_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BATCH_INBOX_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BATCH_INBOX_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BATCH_INBOX_SLOT()";
            const SELECTOR: [u8; 4] = [188u8, 73u8, 206u8, 95u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: BATCH_INBOX_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: BATCH_INBOX_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DELAYED_WETH_SLOT()` and selector `0x7e54b8ad`.
```solidity
function DELAYED_WETH_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELAYED_WETH_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DELAYED_WETH_SLOT()`](DELAYED_WETH_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELAYED_WETH_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DELAYED_WETH_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DELAYED_WETH_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DELAYED_WETH_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DELAYED_WETH_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DELAYED_WETH_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DELAYED_WETH_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DELAYED_WETH_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DELAYED_WETH_SLOT()";
            const SELECTOR: [u8; 4] = [126u8, 84u8, 184u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DELAYED_WETH_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DELAYED_WETH_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `L1_CROSS_DOMAIN_MESSENGER_SLOT()` and selector `0x5d73369c`.
```solidity
function L1_CROSS_DOMAIN_MESSENGER_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1_CROSS_DOMAIN_MESSENGER_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`L1_CROSS_DOMAIN_MESSENGER_SLOT()`](L1_CROSS_DOMAIN_MESSENGER_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1_CROSS_DOMAIN_MESSENGER_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<L1_CROSS_DOMAIN_MESSENGER_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1_CROSS_DOMAIN_MESSENGER_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1_CROSS_DOMAIN_MESSENGER_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<L1_CROSS_DOMAIN_MESSENGER_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1_CROSS_DOMAIN_MESSENGER_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1_CROSS_DOMAIN_MESSENGER_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for L1_CROSS_DOMAIN_MESSENGER_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L1_CROSS_DOMAIN_MESSENGER_SLOT()";
            const SELECTOR: [u8; 4] = [93u8, 115u8, 54u8, 156u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: L1_CROSS_DOMAIN_MESSENGER_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: L1_CROSS_DOMAIN_MESSENGER_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `L1_ERC_721_BRIDGE_SLOT()` and selector `0x19f5cea8`.
```solidity
function L1_ERC_721_BRIDGE_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1_ERC_721_BRIDGE_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`L1_ERC_721_BRIDGE_SLOT()`](L1_ERC_721_BRIDGE_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1_ERC_721_BRIDGE_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<L1_ERC_721_BRIDGE_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1_ERC_721_BRIDGE_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1_ERC_721_BRIDGE_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<L1_ERC_721_BRIDGE_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1_ERC_721_BRIDGE_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1_ERC_721_BRIDGE_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for L1_ERC_721_BRIDGE_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L1_ERC_721_BRIDGE_SLOT()";
            const SELECTOR: [u8; 4] = [25u8, 245u8, 206u8, 168u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: L1_ERC_721_BRIDGE_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: L1_ERC_721_BRIDGE_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `L1_STANDARD_BRIDGE_SLOT()` and selector `0xf8c68de0`.
```solidity
function L1_STANDARD_BRIDGE_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1_STANDARD_BRIDGE_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`L1_STANDARD_BRIDGE_SLOT()`](L1_STANDARD_BRIDGE_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L1_STANDARD_BRIDGE_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<L1_STANDARD_BRIDGE_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1_STANDARD_BRIDGE_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1_STANDARD_BRIDGE_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<L1_STANDARD_BRIDGE_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: L1_STANDARD_BRIDGE_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for L1_STANDARD_BRIDGE_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for L1_STANDARD_BRIDGE_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L1_STANDARD_BRIDGE_SLOT()";
            const SELECTOR: [u8; 4] = [248u8, 198u8, 141u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: L1_STANDARD_BRIDGE_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: L1_STANDARD_BRIDGE_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT()` and selector `0x06c92657`.
```solidity
function OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT()`](OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT()";
            const SELECTOR: [u8; 4] = [6u8, 201u8, 38u8, 87u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `OPTIMISM_PORTAL_SLOT()` and selector `0xfd32aa0f`.
```solidity
function OPTIMISM_PORTAL_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPTIMISM_PORTAL_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`OPTIMISM_PORTAL_SLOT()`](OPTIMISM_PORTAL_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPTIMISM_PORTAL_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPTIMISM_PORTAL_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPTIMISM_PORTAL_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPTIMISM_PORTAL_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPTIMISM_PORTAL_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPTIMISM_PORTAL_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPTIMISM_PORTAL_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPTIMISM_PORTAL_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPTIMISM_PORTAL_SLOT()";
            const SELECTOR: [u8; 4] = [253u8, 50u8, 170u8, 15u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: OPTIMISM_PORTAL_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: OPTIMISM_PORTAL_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `START_BLOCK_SLOT()` and selector `0xe0e2016d`.
```solidity
function START_BLOCK_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct START_BLOCK_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`START_BLOCK_SLOT()`](START_BLOCK_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct START_BLOCK_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<START_BLOCK_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: START_BLOCK_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for START_BLOCK_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<START_BLOCK_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: START_BLOCK_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for START_BLOCK_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for START_BLOCK_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "START_BLOCK_SLOT()";
            const SELECTOR: [u8; 4] = [224u8, 226u8, 1u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: START_BLOCK_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: START_BLOCK_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `UNSAFE_BLOCK_SIGNER_SLOT()` and selector `0x4f16540b`.
```solidity
function UNSAFE_BLOCK_SIGNER_SLOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UNSAFE_BLOCK_SIGNER_SLOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`UNSAFE_BLOCK_SIGNER_SLOT()`](UNSAFE_BLOCK_SIGNER_SLOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UNSAFE_BLOCK_SIGNER_SLOTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UNSAFE_BLOCK_SIGNER_SLOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UNSAFE_BLOCK_SIGNER_SLOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UNSAFE_BLOCK_SIGNER_SLOTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UNSAFE_BLOCK_SIGNER_SLOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UNSAFE_BLOCK_SIGNER_SLOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UNSAFE_BLOCK_SIGNER_SLOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UNSAFE_BLOCK_SIGNER_SLOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UNSAFE_BLOCK_SIGNER_SLOT()";
            const SELECTOR: [u8; 4] = [79u8, 22u8, 84u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: UNSAFE_BLOCK_SIGNER_SLOTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: UNSAFE_BLOCK_SIGNER_SLOTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `VERSION()` and selector `0xffa1ad74`.
```solidity
function VERSION() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`VERSION()`](VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<VERSIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<VERSIONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VERSION()";
            const SELECTOR: [u8; 4] = [255u8, 161u8, 173u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: VERSIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `basefeeScalar()` and selector `0xbfb14fb7`.
```solidity
function basefeeScalar() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct basefeeScalarCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`basefeeScalar()`](basefeeScalarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct basefeeScalarReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<basefeeScalarCall> for UnderlyingRustTuple<'_> {
                fn from(value: basefeeScalarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for basefeeScalarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<basefeeScalarReturn> for UnderlyingRustTuple<'_> {
                fn from(value: basefeeScalarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for basefeeScalarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for basefeeScalarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "basefeeScalar()";
            const SELECTOR: [u8; 4] = [191u8, 177u8, 79u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: basefeeScalarReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: basefeeScalarReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `batchInbox()` and selector `0xdac6e63a`.
```solidity
function batchInbox() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchInboxCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`batchInbox()`](batchInboxCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchInboxReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<batchInboxCall> for UnderlyingRustTuple<'_> {
                fn from(value: batchInboxCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for batchInboxCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<batchInboxReturn> for UnderlyingRustTuple<'_> {
                fn from(value: batchInboxReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for batchInboxReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for batchInboxCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "batchInbox()";
            const SELECTOR: [u8; 4] = [218u8, 198u8, 230u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: batchInboxReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: batchInboxReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `batcherHash()` and selector `0xe81b2c6d`.
```solidity
function batcherHash() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batcherHashCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`batcherHash()`](batcherHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batcherHashReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<batcherHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: batcherHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for batcherHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<batcherHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: batcherHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for batcherHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for batcherHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "batcherHash()";
            const SELECTOR: [u8; 4] = [232u8, 27u8, 44u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: batcherHashReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: batcherHashReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `blobbasefeeScalar()` and selector `0xec707517`.
```solidity
function blobbasefeeScalar() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blobbasefeeScalarCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`blobbasefeeScalar()`](blobbasefeeScalarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blobbasefeeScalarReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blobbasefeeScalarCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: blobbasefeeScalarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blobbasefeeScalarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blobbasefeeScalarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: blobbasefeeScalarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blobbasefeeScalarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blobbasefeeScalarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blobbasefeeScalar()";
            const SELECTOR: [u8; 4] = [236u8, 112u8, 117u8, 23u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: blobbasefeeScalarReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: blobbasefeeScalarReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `daFootprintGasScalar()` and selector `0xfe3d5710`.
```solidity
function daFootprintGasScalar() external view returns (uint16);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct daFootprintGasScalarCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`daFootprintGasScalar()`](daFootprintGasScalarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct daFootprintGasScalarReturn {
        #[allow(missing_docs)]
        pub _0: u16,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<daFootprintGasScalarCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: daFootprintGasScalarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for daFootprintGasScalarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<daFootprintGasScalarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: daFootprintGasScalarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for daFootprintGasScalarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for daFootprintGasScalarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u16;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "daFootprintGasScalar()";
            const SELECTOR: [u8; 4] = [254u8, 61u8, 87u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: daFootprintGasScalarReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: daFootprintGasScalarReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delayedWETH()` and selector `0x215b7a1c`.
```solidity
function delayedWETH() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedWETHCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`delayedWETH()`](delayedWETHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedWETHReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delayedWETHCall> for UnderlyingRustTuple<'_> {
                fn from(value: delayedWETHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delayedWETHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delayedWETHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delayedWETHReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delayedWETHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delayedWETHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delayedWETH()";
            const SELECTOR: [u8; 4] = [33u8, 91u8, 122u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: delayedWETHReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: delayedWETHReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disputeGameFactory()` and selector `0xf2b4e617`.
```solidity
function disputeGameFactory() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameFactoryCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`disputeGameFactory()`](disputeGameFactoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameFactoryReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<disputeGameFactoryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameFactoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameFactoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<disputeGameFactoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameFactoryReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameFactoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disputeGameFactoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disputeGameFactory()";
            const SELECTOR: [u8; 4] = [242u8, 180u8, 230u8, 23u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: disputeGameFactoryReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: disputeGameFactoryReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eip1559Denominator()` and selector `0xd220a9e0`.
```solidity
function eip1559Denominator() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip1559DenominatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eip1559Denominator()`](eip1559DenominatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip1559DenominatorReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip1559DenominatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: eip1559DenominatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eip1559DenominatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip1559DenominatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eip1559DenominatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eip1559DenominatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip1559DenominatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip1559Denominator()";
            const SELECTOR: [u8; 4] = [210u8, 32u8, 169u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: eip1559DenominatorReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: eip1559DenominatorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eip1559Elasticity()` and selector `0xc9ff2d16`.
```solidity
function eip1559Elasticity() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip1559ElasticityCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eip1559Elasticity()`](eip1559ElasticityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip1559ElasticityReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip1559ElasticityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: eip1559ElasticityCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eip1559ElasticityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip1559ElasticityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eip1559ElasticityReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eip1559ElasticityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip1559ElasticityCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip1559Elasticity()";
            const SELECTOR: [u8; 4] = [201u8, 255u8, 45u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: eip1559ElasticityReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: eip1559ElasticityReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `gasLimit()` and selector `0xf68016b7`.
```solidity
function gasLimit() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasLimitCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`gasLimit()`](gasLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasLimitReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gasLimitCall> for UnderlyingRustTuple<'_> {
                fn from(value: gasLimitCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gasLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<gasLimitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gasLimitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gasLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gasLimitCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gasLimit()";
            const SELECTOR: [u8; 4] = [246u8, 128u8, 22u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: gasLimitReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: gasLimitReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAddresses()` and selector `0xa39fac12`.
```solidity
function getAddresses() external view returns (Addresses memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAddressesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAddresses()`](getAddressesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAddressesReturn {
        #[allow(missing_docs)]
        pub _0: <Addresses as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAddressesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAddressesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAddressesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Addresses,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Addresses as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAddressesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAddressesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAddressesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAddressesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Addresses as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Addresses,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAddresses()";
            const SELECTOR: [u8; 4] = [163u8, 159u8, 172u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Addresses as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getAddressesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getAddressesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `guardian()` and selector `0x452a9320`.
```solidity
function guardian() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct guardianCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`guardian()`](guardianCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct guardianReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<guardianCall> for UnderlyingRustTuple<'_> {
                fn from(value: guardianCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for guardianCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<guardianReturn> for UnderlyingRustTuple<'_> {
                fn from(value: guardianReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for guardianReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for guardianCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "guardian()";
            const SELECTOR: [u8; 4] = [69u8, 42u8, 147u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: guardianReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: guardianReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initVersion()` and selector `0x38d38c97`.
```solidity
function initVersion() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initVersionCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`initVersion()`](initVersionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initVersionReturn {
        #[allow(missing_docs)]
        pub _0: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initVersionCall> for UnderlyingRustTuple<'_> {
                fn from(value: initVersionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initVersionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initVersionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initVersionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initVersionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initVersionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initVersion()";
            const SELECTOR: [u8; 4] = [56u8, 211u8, 140u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: initVersionReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: initVersionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address,uint32,uint32,bytes32,uint64,address,(uint32,uint8,uint8,uint32,uint32,uint128),address,(address,address,address,address,address,address),uint256,address)` and selector `0x1edd5098`.
```solidity
function initialize(address _owner, uint32 _basefeeScalar, uint32 _blobbasefeeScalar, bytes32 _batcherHash, uint64 _gasLimit, address _unsafeBlockSigner, IResourceMetering.ResourceConfig memory _config, address _batchInbox, Addresses memory _addresses, uint256 _l2ChainId, address _superchainConfig) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _basefeeScalar: u32,
        #[allow(missing_docs)]
        pub _blobbasefeeScalar: u32,
        #[allow(missing_docs)]
        pub _batcherHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _gasLimit: u64,
        #[allow(missing_docs)]
        pub _unsafeBlockSigner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _config: <IResourceMetering::ResourceConfig as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _batchInbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _addresses: <Addresses as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _l2ChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _superchainConfig: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,uint32,uint32,bytes32,uint64,address,(uint32,uint8,uint8,uint32,uint32,uint128),address,(address,address,address,address,address,address),uint256,address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                IResourceMetering::ResourceConfig,
                alloy::sol_types::sol_data::Address,
                Addresses,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                u64,
                alloy::sol_types::private::Address,
                <IResourceMetering::ResourceConfig as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                <Addresses as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value._owner,
                        value._basefeeScalar,
                        value._blobbasefeeScalar,
                        value._batcherHash,
                        value._gasLimit,
                        value._unsafeBlockSigner,
                        value._config,
                        value._batchInbox,
                        value._addresses,
                        value._l2ChainId,
                        value._superchainConfig,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _owner: tuple.0,
                        _basefeeScalar: tuple.1,
                        _blobbasefeeScalar: tuple.2,
                        _batcherHash: tuple.3,
                        _gasLimit: tuple.4,
                        _unsafeBlockSigner: tuple.5,
                        _config: tuple.6,
                        _batchInbox: tuple.7,
                        _addresses: tuple.8,
                        _l2ChainId: tuple.9,
                        _superchainConfig: tuple.10,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeReturn {
            fn _tokenize(
                &self,
            ) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                IResourceMetering::ResourceConfig,
                alloy::sol_types::sol_data::Address,
                Addresses,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint32,uint32,bytes32,uint64,address,(uint32,uint8,uint8,uint32,uint32,uint128),address,(address,address,address,address,address,address),uint256,address)";
            const SELECTOR: [u8; 4] = [30u8, 221u8, 80u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._basefeeScalar),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blobbasefeeScalar),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._batcherHash),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._gasLimit),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._unsafeBlockSigner,
                    ),
                    <IResourceMetering::ResourceConfig as alloy_sol_types::SolType>::tokenize(
                        &self._config,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._batchInbox,
                    ),
                    <Addresses as alloy_sol_types::SolType>::tokenize(&self._addresses),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l2ChainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._superchainConfig,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isCustomGasToken()` and selector `0x21326849`.
```solidity
function isCustomGasToken() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCustomGasTokenCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCustomGasToken()`](isCustomGasTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCustomGasTokenReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isCustomGasTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCustomGasTokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCustomGasTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isCustomGasTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCustomGasTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCustomGasTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCustomGasTokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCustomGasToken()";
            const SELECTOR: [u8; 4] = [33u8, 50u8, 104u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isCustomGasTokenReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isCustomGasTokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isFeatureEnabled(bytes32)` and selector `0x47af267b`.
```solidity
function isFeatureEnabled(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isFeatureEnabledCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isFeatureEnabled(bytes32)`](isFeatureEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isFeatureEnabledReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isFeatureEnabledCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isFeatureEnabledCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isFeatureEnabledCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isFeatureEnabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isFeatureEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isFeatureEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isFeatureEnabledCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isFeatureEnabled(bytes32)";
            const SELECTOR: [u8; 4] = [71u8, 175u8, 38u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isFeatureEnabledReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isFeatureEnabledReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l1CrossDomainMessenger()` and selector `0xa7119869`.
```solidity
function l1CrossDomainMessenger() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1CrossDomainMessengerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l1CrossDomainMessenger()`](l1CrossDomainMessengerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1CrossDomainMessengerReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1CrossDomainMessengerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l1CrossDomainMessengerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l1CrossDomainMessengerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1CrossDomainMessengerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l1CrossDomainMessengerReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l1CrossDomainMessengerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l1CrossDomainMessengerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l1CrossDomainMessenger()";
            const SELECTOR: [u8; 4] = [167u8, 17u8, 152u8, 105u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l1CrossDomainMessengerReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l1CrossDomainMessengerReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l1ERC721Bridge()` and selector `0xc4e8ddfa`.
```solidity
function l1ERC721Bridge() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1ERC721BridgeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l1ERC721Bridge()`](l1ERC721BridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1ERC721BridgeReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1ERC721BridgeCall> for UnderlyingRustTuple<'_> {
                fn from(value: l1ERC721BridgeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l1ERC721BridgeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1ERC721BridgeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l1ERC721BridgeReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l1ERC721BridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l1ERC721BridgeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l1ERC721Bridge()";
            const SELECTOR: [u8; 4] = [196u8, 232u8, 221u8, 250u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l1ERC721BridgeReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l1ERC721BridgeReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l1StandardBridge()` and selector `0x078f29cf`.
```solidity
function l1StandardBridge() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1StandardBridgeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l1StandardBridge()`](l1StandardBridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1StandardBridgeReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1StandardBridgeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l1StandardBridgeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l1StandardBridgeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l1StandardBridgeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l1StandardBridgeReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l1StandardBridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l1StandardBridgeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l1StandardBridge()";
            const SELECTOR: [u8; 4] = [7u8, 143u8, 41u8, 207u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l1StandardBridgeReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l1StandardBridgeReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l2ChainId()` and selector `0xd6ae3cd5`.
```solidity
function l2ChainId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2ChainIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2ChainId()`](l2ChainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2ChainIdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2ChainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2ChainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2ChainIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<l2ChainIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2ChainIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2ChainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2ChainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2ChainId()";
            const SELECTOR: [u8; 4] = [214u8, 174u8, 60u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l2ChainIdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: l2ChainIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `maximumGasLimit()` and selector `0x0ae14b1b`.
```solidity
function maximumGasLimit() external pure returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maximumGasLimitCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`maximumGasLimit()`](maximumGasLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maximumGasLimitReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maximumGasLimitCall> for UnderlyingRustTuple<'_> {
                fn from(value: maximumGasLimitCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maximumGasLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maximumGasLimitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: maximumGasLimitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maximumGasLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maximumGasLimitCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maximumGasLimit()";
            const SELECTOR: [u8; 4] = [10u8, 225u8, 75u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: maximumGasLimitReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: maximumGasLimitReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `minBaseFee()` and selector `0xa62611a2`.
```solidity
function minBaseFee() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minBaseFeeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`minBaseFee()`](minBaseFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minBaseFeeReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minBaseFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: minBaseFeeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minBaseFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minBaseFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minBaseFeeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minBaseFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minBaseFeeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minBaseFee()";
            const SELECTOR: [u8; 4] = [166u8, 38u8, 17u8, 162u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: minBaseFeeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: minBaseFeeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `minimumGasLimit()` and selector `0x4add321d`.
```solidity
function minimumGasLimit() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumGasLimitCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`minimumGasLimit()`](minimumGasLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumGasLimitReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumGasLimitCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumGasLimitCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumGasLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumGasLimitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: minimumGasLimitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minimumGasLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumGasLimitCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumGasLimit()";
            const SELECTOR: [u8; 4] = [74u8, 221u8, 50u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: minimumGasLimitReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: minimumGasLimitReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `operatorFeeConstant()` and selector `0x16d3bc7f`.
```solidity
function operatorFeeConstant() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorFeeConstantCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`operatorFeeConstant()`](operatorFeeConstantCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorFeeConstantReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorFeeConstantCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorFeeConstantCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorFeeConstantCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorFeeConstantReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorFeeConstantReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorFeeConstantReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorFeeConstantCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorFeeConstant()";
            const SELECTOR: [u8; 4] = [22u8, 211u8, 188u8, 127u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: operatorFeeConstantReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: operatorFeeConstantReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `operatorFeeScalar()` and selector `0x4d5d9a2a`.
```solidity
function operatorFeeScalar() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorFeeScalarCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`operatorFeeScalar()`](operatorFeeScalarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorFeeScalarReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorFeeScalarCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorFeeScalarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorFeeScalarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorFeeScalarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorFeeScalarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorFeeScalarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorFeeScalarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorFeeScalar()";
            const SELECTOR: [u8; 4] = [77u8, 93u8, 154u8, 42u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: operatorFeeScalarReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: operatorFeeScalarReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `optimismMintableERC20Factory()` and selector `0x9b7d7f0a`.
```solidity
function optimismMintableERC20Factory() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimismMintableERC20FactoryCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`optimismMintableERC20Factory()`](optimismMintableERC20FactoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimismMintableERC20FactoryReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<optimismMintableERC20FactoryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: optimismMintableERC20FactoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for optimismMintableERC20FactoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<optimismMintableERC20FactoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: optimismMintableERC20FactoryReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for optimismMintableERC20FactoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for optimismMintableERC20FactoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "optimismMintableERC20Factory()";
            const SELECTOR: [u8; 4] = [155u8, 125u8, 127u8, 10u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: optimismMintableERC20FactoryReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: optimismMintableERC20FactoryReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `optimismPortal()` and selector `0x0a49cb03`.
```solidity
function optimismPortal() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimismPortalCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`optimismPortal()`](optimismPortalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct optimismPortalReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<optimismPortalCall> for UnderlyingRustTuple<'_> {
                fn from(value: optimismPortalCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for optimismPortalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<optimismPortalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: optimismPortalReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for optimismPortalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for optimismPortalCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "optimismPortal()";
            const SELECTOR: [u8; 4] = [10u8, 73u8, 203u8, 3u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: optimismPortalReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: optimismPortalReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `overhead()` and selector `0x0c18c162`.
```solidity
function overhead() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct overheadCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`overhead()`](overheadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct overheadReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<overheadCall> for UnderlyingRustTuple<'_> {
                fn from(value: overheadCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for overheadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<overheadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: overheadReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for overheadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for overheadCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "overhead()";
            const SELECTOR: [u8; 4] = [12u8, 24u8, 193u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: overheadReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: overheadReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proxyAdmin()` and selector `0x3e47158c`.
```solidity
function proxyAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxyAdminCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proxyAdmin()`](proxyAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxyAdminReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxyAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxyAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxyAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxyAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxyAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxyAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxyAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxyAdmin()";
            const SELECTOR: [u8; 4] = [62u8, 71u8, 21u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: proxyAdminReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: proxyAdminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proxyAdminOwner()` and selector `0xdad544e0`.
```solidity
function proxyAdminOwner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxyAdminOwnerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proxyAdminOwner()`](proxyAdminOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxyAdminOwnerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxyAdminOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxyAdminOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxyAdminOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxyAdminOwnerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proxyAdminOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proxyAdminOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxyAdminOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxyAdminOwner()";
            const SELECTOR: [u8; 4] = [218u8, 213u8, 68u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: proxyAdminOwnerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: proxyAdminOwnerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resourceConfig()` and selector `0xcc731b02`.
```solidity
function resourceConfig() external view returns (IResourceMetering.ResourceConfig memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resourceConfigCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`resourceConfig()`](resourceConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resourceConfigReturn {
        #[allow(missing_docs)]
        pub _0: <IResourceMetering::ResourceConfig as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resourceConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: resourceConfigCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resourceConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IResourceMetering::ResourceConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IResourceMetering::ResourceConfig as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resourceConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resourceConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resourceConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resourceConfigCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IResourceMetering::ResourceConfig as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IResourceMetering::ResourceConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resourceConfig()";
            const SELECTOR: [u8; 4] = [204u8, 115u8, 27u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IResourceMetering::ResourceConfig as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: resourceConfigReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: resourceConfigReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `scalar()` and selector `0xf45e65d8`.
```solidity
function scalar() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scalarCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`scalar()`](scalarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scalarReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<scalarCall> for UnderlyingRustTuple<'_> {
                fn from(value: scalarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scalarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<scalarReturn> for UnderlyingRustTuple<'_> {
                fn from(value: scalarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scalarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scalarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scalar()";
            const SELECTOR: [u8; 4] = [244u8, 94u8, 101u8, 216u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: scalarReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: scalarReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setBatcherHash(address)` and selector `0x0a2ca2a9`.
```solidity
function setBatcherHash(address _batcher) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBatcherHash_0Call {
        #[allow(missing_docs)]
        pub _batcher: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setBatcherHash(address)`](setBatcherHash_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBatcherHash_0Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBatcherHash_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBatcherHash_0Call) -> Self {
                    (value._batcher,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBatcherHash_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _batcher: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBatcherHash_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBatcherHash_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBatcherHash_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setBatcherHash_0Return {
            fn _tokenize(
                &self,
            ) -> <setBatcherHash_0Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBatcherHash_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBatcherHash_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBatcherHash(address)";
            const SELECTOR: [u8; 4] = [10u8, 44u8, 162u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._batcher,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setBatcherHash_0Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setBatcherHash(bytes32)` and selector `0xc9b26f61`.
```solidity
function setBatcherHash(bytes32 _batcherHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBatcherHash_1Call {
        #[allow(missing_docs)]
        pub _batcherHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`setBatcherHash(bytes32)`](setBatcherHash_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBatcherHash_1Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBatcherHash_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBatcherHash_1Call) -> Self {
                    (value._batcherHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBatcherHash_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _batcherHash: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBatcherHash_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: setBatcherHash_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setBatcherHash_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setBatcherHash_1Return {
            fn _tokenize(
                &self,
            ) -> <setBatcherHash_1Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBatcherHash_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBatcherHash_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBatcherHash(bytes32)";
            const SELECTOR: [u8; 4] = [201u8, 178u8, 111u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._batcherHash),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setBatcherHash_1Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setDAFootprintGasScalar(uint16)` and selector `0x20f06fdc`.
```solidity
function setDAFootprintGasScalar(uint16 _daFootprintGasScalar) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setDAFootprintGasScalarCall {
        #[allow(missing_docs)]
        pub _daFootprintGasScalar: u16,
    }
    ///Container type for the return parameters of the [`setDAFootprintGasScalar(uint16)`](setDAFootprintGasScalarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setDAFootprintGasScalarReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setDAFootprintGasScalarCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setDAFootprintGasScalarCall) -> Self {
                    (value._daFootprintGasScalar,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setDAFootprintGasScalarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _daFootprintGasScalar: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setDAFootprintGasScalarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setDAFootprintGasScalarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setDAFootprintGasScalarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setDAFootprintGasScalarReturn {
            fn _tokenize(
                &self,
            ) -> <setDAFootprintGasScalarCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setDAFootprintGasScalarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setDAFootprintGasScalarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setDAFootprintGasScalar(uint16)";
            const SELECTOR: [u8; 4] = [32u8, 240u8, 111u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._daFootprintGasScalar,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setDAFootprintGasScalarReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setEIP1559Params(uint32,uint32)` and selector `0xc0fd4b41`.
```solidity
function setEIP1559Params(uint32 _denominator, uint32 _elasticity) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEIP1559ParamsCall {
        #[allow(missing_docs)]
        pub _denominator: u32,
        #[allow(missing_docs)]
        pub _elasticity: u32,
    }
    ///Container type for the return parameters of the [`setEIP1559Params(uint32,uint32)`](setEIP1559ParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEIP1559ParamsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEIP1559ParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setEIP1559ParamsCall) -> Self {
                    (value._denominator, value._elasticity)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setEIP1559ParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _denominator: tuple.0,
                        _elasticity: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEIP1559ParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setEIP1559ParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setEIP1559ParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setEIP1559ParamsReturn {
            fn _tokenize(
                &self,
            ) -> <setEIP1559ParamsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEIP1559ParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEIP1559ParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEIP1559Params(uint32,uint32)";
            const SELECTOR: [u8; 4] = [192u8, 253u8, 75u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._denominator),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._elasticity),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setEIP1559ParamsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setFeature(bytes32,bool)` and selector `0xf2c4bc9e`.
```solidity
function setFeature(bytes32 _feature, bool _enabled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFeatureCall {
        #[allow(missing_docs)]
        pub _feature: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _enabled: bool,
    }
    ///Container type for the return parameters of the [`setFeature(bytes32,bool)`](setFeatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFeatureReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setFeatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: setFeatureCall) -> Self {
                    (value._feature, value._enabled)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFeatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _feature: tuple.0,
                        _enabled: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setFeatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setFeatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFeatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setFeatureReturn {
            fn _tokenize(
                &self,
            ) -> <setFeatureCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setFeatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setFeatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setFeature(bytes32,bool)";
            const SELECTOR: [u8; 4] = [242u8, 196u8, 188u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._feature),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._enabled,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setFeatureReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setGasConfig(uint256,uint256)` and selector `0x935f029e`.
```solidity
function setGasConfig(uint256 _overhead, uint256 _scalar) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGasConfigCall {
        #[allow(missing_docs)]
        pub _overhead: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _scalar: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setGasConfig(uint256,uint256)`](setGasConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGasConfigReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setGasConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: setGasConfigCall) -> Self {
                    (value._overhead, value._scalar)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGasConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _overhead: tuple.0,
                        _scalar: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setGasConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setGasConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGasConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setGasConfigReturn {
            fn _tokenize(
                &self,
            ) -> <setGasConfigCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setGasConfigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setGasConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setGasConfig(uint256,uint256)";
            const SELECTOR: [u8; 4] = [147u8, 95u8, 2u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._overhead),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._scalar),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setGasConfigReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setGasConfigEcotone(uint32,uint32)` and selector `0x21d7fde5`.
```solidity
function setGasConfigEcotone(uint32 _basefeeScalar, uint32 _blobbasefeeScalar) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGasConfigEcotoneCall {
        #[allow(missing_docs)]
        pub _basefeeScalar: u32,
        #[allow(missing_docs)]
        pub _blobbasefeeScalar: u32,
    }
    ///Container type for the return parameters of the [`setGasConfigEcotone(uint32,uint32)`](setGasConfigEcotoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGasConfigEcotoneReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setGasConfigEcotoneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setGasConfigEcotoneCall) -> Self {
                    (value._basefeeScalar, value._blobbasefeeScalar)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setGasConfigEcotoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _basefeeScalar: tuple.0,
                        _blobbasefeeScalar: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setGasConfigEcotoneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setGasConfigEcotoneReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setGasConfigEcotoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setGasConfigEcotoneReturn {
            fn _tokenize(
                &self,
            ) -> <setGasConfigEcotoneCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setGasConfigEcotoneCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setGasConfigEcotoneReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setGasConfigEcotone(uint32,uint32)";
            const SELECTOR: [u8; 4] = [33u8, 215u8, 253u8, 229u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._basefeeScalar),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blobbasefeeScalar),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setGasConfigEcotoneReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setGasLimit(uint64)` and selector `0xb40a817c`.
```solidity
function setGasLimit(uint64 _gasLimit) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGasLimitCall {
        #[allow(missing_docs)]
        pub _gasLimit: u64,
    }
    ///Container type for the return parameters of the [`setGasLimit(uint64)`](setGasLimitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGasLimitReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setGasLimitCall> for UnderlyingRustTuple<'_> {
                fn from(value: setGasLimitCall) -> Self {
                    (value._gasLimit,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGasLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _gasLimit: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setGasLimitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setGasLimitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGasLimitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setGasLimitReturn {
            fn _tokenize(
                &self,
            ) -> <setGasLimitCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setGasLimitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setGasLimitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setGasLimit(uint64)";
            const SELECTOR: [u8; 4] = [180u8, 10u8, 129u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._gasLimit),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setGasLimitReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setMinBaseFee(uint64)` and selector `0x7616f0e8`.
```solidity
function setMinBaseFee(uint64 _minBaseFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinBaseFeeCall {
        #[allow(missing_docs)]
        pub _minBaseFee: u64,
    }
    ///Container type for the return parameters of the [`setMinBaseFee(uint64)`](setMinBaseFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinBaseFeeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMinBaseFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setMinBaseFeeCall) -> Self {
                    (value._minBaseFee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinBaseFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _minBaseFee: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMinBaseFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setMinBaseFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinBaseFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setMinBaseFeeReturn {
            fn _tokenize(
                &self,
            ) -> <setMinBaseFeeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMinBaseFeeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMinBaseFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMinBaseFee(uint64)";
            const SELECTOR: [u8; 4] = [118u8, 22u8, 240u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._minBaseFee),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setMinBaseFeeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setOperatorFeeScalars(uint32,uint64)` and selector `0x155b6c6f`.
```solidity
function setOperatorFeeScalars(uint32 _operatorFeeScalar, uint64 _operatorFeeConstant) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorFeeScalarsCall {
        #[allow(missing_docs)]
        pub _operatorFeeScalar: u32,
        #[allow(missing_docs)]
        pub _operatorFeeConstant: u64,
    }
    ///Container type for the return parameters of the [`setOperatorFeeScalars(uint32,uint64)`](setOperatorFeeScalarsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorFeeScalarsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorFeeScalarsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorFeeScalarsCall) -> Self {
                    (value._operatorFeeScalar, value._operatorFeeConstant)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorFeeScalarsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operatorFeeScalar: tuple.0,
                        _operatorFeeConstant: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorFeeScalarsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorFeeScalarsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorFeeScalarsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setOperatorFeeScalarsReturn {
            fn _tokenize(
                &self,
            ) -> <setOperatorFeeScalarsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorFeeScalarsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorFeeScalarsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorFeeScalars(uint32,uint64)";
            const SELECTOR: [u8; 4] = [21u8, 91u8, 108u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._operatorFeeScalar),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._operatorFeeConstant),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setOperatorFeeScalarsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setUnsafeBlockSigner(address)` and selector `0x18d13918`.
```solidity
function setUnsafeBlockSigner(address _unsafeBlockSigner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUnsafeBlockSignerCall {
        #[allow(missing_docs)]
        pub _unsafeBlockSigner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setUnsafeBlockSigner(address)`](setUnsafeBlockSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUnsafeBlockSignerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUnsafeBlockSignerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUnsafeBlockSignerCall) -> Self {
                    (value._unsafeBlockSigner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUnsafeBlockSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _unsafeBlockSigner: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUnsafeBlockSignerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUnsafeBlockSignerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUnsafeBlockSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setUnsafeBlockSignerReturn {
            fn _tokenize(
                &self,
            ) -> <setUnsafeBlockSignerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUnsafeBlockSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUnsafeBlockSignerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUnsafeBlockSigner(address)";
            const SELECTOR: [u8; 4] = [24u8, 209u8, 57u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._unsafeBlockSigner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setUnsafeBlockSignerReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `startBlock()` and selector `0x48cd4cb1`.
```solidity
function startBlock() external view returns (uint256 startBlock_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startBlockCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`startBlock()`](startBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startBlockReturn {
        #[allow(missing_docs)]
        pub startBlock_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: startBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startBlockReturn) -> Self {
                    (value.startBlock_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { startBlock_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startBlock()";
            const SELECTOR: [u8; 4] = [72u8, 205u8, 76u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: startBlockReturn = r.into();
                        r.startBlock_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: startBlockReturn = r.into();
                        r.startBlock_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `superchainConfig()` and selector `0x35e80ab3`.
```solidity
function superchainConfig() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct superchainConfigCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`superchainConfig()`](superchainConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct superchainConfigReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<superchainConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: superchainConfigCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for superchainConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<superchainConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: superchainConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for superchainConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for superchainConfigCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "superchainConfig()";
            const SELECTOR: [u8; 4] = [53u8, 232u8, 10u8, 179u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: superchainConfigReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: superchainConfigReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `unsafeBlockSigner()` and selector `0x1fd19ee1`.
```solidity
function unsafeBlockSigner() external view returns (address addr_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unsafeBlockSignerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`unsafeBlockSigner()`](unsafeBlockSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unsafeBlockSignerReturn {
        #[allow(missing_docs)]
        pub addr_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unsafeBlockSignerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: unsafeBlockSignerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unsafeBlockSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unsafeBlockSignerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unsafeBlockSignerReturn) -> Self {
                    (value.addr_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unsafeBlockSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { addr_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unsafeBlockSignerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unsafeBlockSigner()";
            const SELECTOR: [u8; 4] = [31u8, 209u8, 158u8, 225u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: unsafeBlockSignerReturn = r.into();
                        r.addr_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: unsafeBlockSignerReturn = r.into();
                        r.addr_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `version()` and selector `0x54fd4d50`.
```solidity
function version() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`version()`](versionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionCall> for UnderlyingRustTuple<'_> {
                fn from(value: versionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: versionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for versionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "version()";
            const SELECTOR: [u8; 4] = [84u8, 253u8, 77u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: versionReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: versionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`SystemConfig`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SystemConfigCalls {
        #[allow(missing_docs)]
        BATCH_INBOX_SLOT(BATCH_INBOX_SLOTCall),
        #[allow(missing_docs)]
        DELAYED_WETH_SLOT(DELAYED_WETH_SLOTCall),
        #[allow(missing_docs)]
        L1_CROSS_DOMAIN_MESSENGER_SLOT(L1_CROSS_DOMAIN_MESSENGER_SLOTCall),
        #[allow(missing_docs)]
        L1_ERC_721_BRIDGE_SLOT(L1_ERC_721_BRIDGE_SLOTCall),
        #[allow(missing_docs)]
        L1_STANDARD_BRIDGE_SLOT(L1_STANDARD_BRIDGE_SLOTCall),
        #[allow(missing_docs)]
        OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT(OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall),
        #[allow(missing_docs)]
        OPTIMISM_PORTAL_SLOT(OPTIMISM_PORTAL_SLOTCall),
        #[allow(missing_docs)]
        START_BLOCK_SLOT(START_BLOCK_SLOTCall),
        #[allow(missing_docs)]
        UNSAFE_BLOCK_SIGNER_SLOT(UNSAFE_BLOCK_SIGNER_SLOTCall),
        #[allow(missing_docs)]
        VERSION(VERSIONCall),
        #[allow(missing_docs)]
        basefeeScalar(basefeeScalarCall),
        #[allow(missing_docs)]
        batchInbox(batchInboxCall),
        #[allow(missing_docs)]
        batcherHash(batcherHashCall),
        #[allow(missing_docs)]
        blobbasefeeScalar(blobbasefeeScalarCall),
        #[allow(missing_docs)]
        daFootprintGasScalar(daFootprintGasScalarCall),
        #[allow(missing_docs)]
        delayedWETH(delayedWETHCall),
        #[allow(missing_docs)]
        disputeGameFactory(disputeGameFactoryCall),
        #[allow(missing_docs)]
        eip1559Denominator(eip1559DenominatorCall),
        #[allow(missing_docs)]
        eip1559Elasticity(eip1559ElasticityCall),
        #[allow(missing_docs)]
        gasLimit(gasLimitCall),
        #[allow(missing_docs)]
        getAddresses(getAddressesCall),
        #[allow(missing_docs)]
        guardian(guardianCall),
        #[allow(missing_docs)]
        initVersion(initVersionCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isCustomGasToken(isCustomGasTokenCall),
        #[allow(missing_docs)]
        isFeatureEnabled(isFeatureEnabledCall),
        #[allow(missing_docs)]
        l1CrossDomainMessenger(l1CrossDomainMessengerCall),
        #[allow(missing_docs)]
        l1ERC721Bridge(l1ERC721BridgeCall),
        #[allow(missing_docs)]
        l1StandardBridge(l1StandardBridgeCall),
        #[allow(missing_docs)]
        l2ChainId(l2ChainIdCall),
        #[allow(missing_docs)]
        maximumGasLimit(maximumGasLimitCall),
        #[allow(missing_docs)]
        minBaseFee(minBaseFeeCall),
        #[allow(missing_docs)]
        minimumGasLimit(minimumGasLimitCall),
        #[allow(missing_docs)]
        operatorFeeConstant(operatorFeeConstantCall),
        #[allow(missing_docs)]
        operatorFeeScalar(operatorFeeScalarCall),
        #[allow(missing_docs)]
        optimismMintableERC20Factory(optimismMintableERC20FactoryCall),
        #[allow(missing_docs)]
        optimismPortal(optimismPortalCall),
        #[allow(missing_docs)]
        overhead(overheadCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        proxyAdmin(proxyAdminCall),
        #[allow(missing_docs)]
        proxyAdminOwner(proxyAdminOwnerCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        resourceConfig(resourceConfigCall),
        #[allow(missing_docs)]
        scalar(scalarCall),
        #[allow(missing_docs)]
        setBatcherHash_0(setBatcherHash_0Call),
        #[allow(missing_docs)]
        setBatcherHash_1(setBatcherHash_1Call),
        #[allow(missing_docs)]
        setDAFootprintGasScalar(setDAFootprintGasScalarCall),
        #[allow(missing_docs)]
        setEIP1559Params(setEIP1559ParamsCall),
        #[allow(missing_docs)]
        setFeature(setFeatureCall),
        #[allow(missing_docs)]
        setGasConfig(setGasConfigCall),
        #[allow(missing_docs)]
        setGasConfigEcotone(setGasConfigEcotoneCall),
        #[allow(missing_docs)]
        setGasLimit(setGasLimitCall),
        #[allow(missing_docs)]
        setMinBaseFee(setMinBaseFeeCall),
        #[allow(missing_docs)]
        setOperatorFeeScalars(setOperatorFeeScalarsCall),
        #[allow(missing_docs)]
        setUnsafeBlockSigner(setUnsafeBlockSignerCall),
        #[allow(missing_docs)]
        startBlock(startBlockCall),
        #[allow(missing_docs)]
        superchainConfig(superchainConfigCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unsafeBlockSigner(unsafeBlockSignerCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl SystemConfigCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 201u8, 38u8, 87u8],
            [7u8, 143u8, 41u8, 207u8],
            [10u8, 44u8, 162u8, 169u8],
            [10u8, 73u8, 203u8, 3u8],
            [10u8, 225u8, 75u8, 27u8],
            [12u8, 24u8, 193u8, 98u8],
            [21u8, 91u8, 108u8, 111u8],
            [22u8, 211u8, 188u8, 127u8],
            [24u8, 209u8, 57u8, 24u8],
            [25u8, 245u8, 206u8, 168u8],
            [30u8, 221u8, 80u8, 152u8],
            [31u8, 209u8, 158u8, 225u8],
            [32u8, 240u8, 111u8, 220u8],
            [33u8, 50u8, 104u8, 73u8],
            [33u8, 91u8, 122u8, 28u8],
            [33u8, 215u8, 253u8, 229u8],
            [53u8, 232u8, 10u8, 179u8],
            [56u8, 211u8, 140u8, 151u8],
            [62u8, 71u8, 21u8, 140u8],
            [69u8, 42u8, 147u8, 32u8],
            [71u8, 175u8, 38u8, 123u8],
            [72u8, 205u8, 76u8, 177u8],
            [74u8, 221u8, 50u8, 29u8],
            [77u8, 93u8, 154u8, 42u8],
            [79u8, 22u8, 84u8, 11u8],
            [84u8, 253u8, 77u8, 80u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 115u8, 54u8, 156u8],
            [113u8, 80u8, 24u8, 166u8],
            [118u8, 22u8, 240u8, 232u8],
            [126u8, 84u8, 184u8, 173u8],
            [141u8, 165u8, 203u8, 91u8],
            [147u8, 95u8, 2u8, 158u8],
            [155u8, 125u8, 127u8, 10u8],
            [163u8, 159u8, 172u8, 18u8],
            [166u8, 38u8, 17u8, 162u8],
            [167u8, 17u8, 152u8, 105u8],
            [180u8, 10u8, 129u8, 124u8],
            [188u8, 73u8, 206u8, 95u8],
            [191u8, 177u8, 79u8, 183u8],
            [192u8, 253u8, 75u8, 65u8],
            [196u8, 232u8, 221u8, 250u8],
            [201u8, 178u8, 111u8, 97u8],
            [201u8, 255u8, 45u8, 22u8],
            [204u8, 115u8, 27u8, 2u8],
            [210u8, 32u8, 169u8, 224u8],
            [214u8, 174u8, 60u8, 213u8],
            [218u8, 198u8, 230u8, 58u8],
            [218u8, 213u8, 68u8, 224u8],
            [224u8, 226u8, 1u8, 109u8],
            [232u8, 27u8, 44u8, 109u8],
            [236u8, 112u8, 117u8, 23u8],
            [242u8, 180u8, 230u8, 23u8],
            [242u8, 196u8, 188u8, 158u8],
            [242u8, 253u8, 227u8, 139u8],
            [244u8, 94u8, 101u8, 216u8],
            [246u8, 128u8, 22u8, 183u8],
            [248u8, 198u8, 141u8, 224u8],
            [253u8, 50u8, 170u8, 15u8],
            [254u8, 61u8, 87u8, 16u8],
            [255u8, 161u8, 173u8, 116u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT),
            ::core::stringify!(l1StandardBridge),
            ::core::stringify!(setBatcherHash_0),
            ::core::stringify!(optimismPortal),
            ::core::stringify!(maximumGasLimit),
            ::core::stringify!(overhead),
            ::core::stringify!(setOperatorFeeScalars),
            ::core::stringify!(operatorFeeConstant),
            ::core::stringify!(setUnsafeBlockSigner),
            ::core::stringify!(L1_ERC_721_BRIDGE_SLOT),
            ::core::stringify!(initialize),
            ::core::stringify!(unsafeBlockSigner),
            ::core::stringify!(setDAFootprintGasScalar),
            ::core::stringify!(isCustomGasToken),
            ::core::stringify!(delayedWETH),
            ::core::stringify!(setGasConfigEcotone),
            ::core::stringify!(superchainConfig),
            ::core::stringify!(initVersion),
            ::core::stringify!(proxyAdmin),
            ::core::stringify!(guardian),
            ::core::stringify!(isFeatureEnabled),
            ::core::stringify!(startBlock),
            ::core::stringify!(minimumGasLimit),
            ::core::stringify!(operatorFeeScalar),
            ::core::stringify!(UNSAFE_BLOCK_SIGNER_SLOT),
            ::core::stringify!(version),
            ::core::stringify!(paused),
            ::core::stringify!(L1_CROSS_DOMAIN_MESSENGER_SLOT),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(setMinBaseFee),
            ::core::stringify!(DELAYED_WETH_SLOT),
            ::core::stringify!(owner),
            ::core::stringify!(setGasConfig),
            ::core::stringify!(optimismMintableERC20Factory),
            ::core::stringify!(getAddresses),
            ::core::stringify!(minBaseFee),
            ::core::stringify!(l1CrossDomainMessenger),
            ::core::stringify!(setGasLimit),
            ::core::stringify!(BATCH_INBOX_SLOT),
            ::core::stringify!(basefeeScalar),
            ::core::stringify!(setEIP1559Params),
            ::core::stringify!(l1ERC721Bridge),
            ::core::stringify!(setBatcherHash_1),
            ::core::stringify!(eip1559Elasticity),
            ::core::stringify!(resourceConfig),
            ::core::stringify!(eip1559Denominator),
            ::core::stringify!(l2ChainId),
            ::core::stringify!(batchInbox),
            ::core::stringify!(proxyAdminOwner),
            ::core::stringify!(START_BLOCK_SLOT),
            ::core::stringify!(batcherHash),
            ::core::stringify!(blobbasefeeScalar),
            ::core::stringify!(disputeGameFactory),
            ::core::stringify!(setFeature),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(scalar),
            ::core::stringify!(gasLimit),
            ::core::stringify!(L1_STANDARD_BRIDGE_SLOT),
            ::core::stringify!(OPTIMISM_PORTAL_SLOT),
            ::core::stringify!(daFootprintGasScalar),
            ::core::stringify!(VERSION),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l1StandardBridgeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setBatcherHash_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <optimismPortalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maximumGasLimitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <overheadCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setOperatorFeeScalarsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <operatorFeeConstantCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setUnsafeBlockSignerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <L1_ERC_721_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <unsafeBlockSignerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setDAFootprintGasScalarCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isCustomGasTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <delayedWETHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setGasConfigEcotoneCall as alloy_sol_types::SolCall>::SIGNATURE,
            <superchainConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxyAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <guardianCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isFeatureEnabledCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <minimumGasLimitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <operatorFeeScalarCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UNSAFE_BLOCK_SIGNER_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <L1_CROSS_DOMAIN_MESSENGER_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setMinBaseFeeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DELAYED_WETH_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setGasConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <optimismMintableERC20FactoryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getAddressesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <minBaseFeeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l1CrossDomainMessengerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setGasLimitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <BATCH_INBOX_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <basefeeScalarCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setEIP1559ParamsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l1ERC721BridgeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setBatcherHash_1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <eip1559ElasticityCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resourceConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eip1559DenominatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2ChainIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <batchInboxCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxyAdminOwnerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <START_BLOCK_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <batcherHashCall as alloy_sol_types::SolCall>::SIGNATURE,
            <blobbasefeeScalarCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeGameFactoryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setFeatureCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scalarCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gasLimitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <L1_STANDARD_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <OPTIMISM_PORTAL_SLOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <daFootprintGasScalarCall as alloy_sol_types::SolCall>::SIGNATURE,
            <VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SystemConfigCalls {
        const NAME: &'static str = "SystemConfigCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 61usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BATCH_INBOX_SLOT(_) => {
                    <BATCH_INBOX_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DELAYED_WETH_SLOT(_) => {
                    <DELAYED_WETH_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::L1_CROSS_DOMAIN_MESSENGER_SLOT(_) => {
                    <L1_CROSS_DOMAIN_MESSENGER_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::L1_ERC_721_BRIDGE_SLOT(_) => {
                    <L1_ERC_721_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::L1_STANDARD_BRIDGE_SLOT(_) => {
                    <L1_STANDARD_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT(_) => {
                    <OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OPTIMISM_PORTAL_SLOT(_) => {
                    <OPTIMISM_PORTAL_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::START_BLOCK_SLOT(_) => {
                    <START_BLOCK_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UNSAFE_BLOCK_SIGNER_SLOT(_) => {
                    <UNSAFE_BLOCK_SIGNER_SLOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::VERSION(_) => <VERSIONCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::basefeeScalar(_) => {
                    <basefeeScalarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::batchInbox(_) => {
                    <batchInboxCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::batcherHash(_) => {
                    <batcherHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blobbasefeeScalar(_) => {
                    <blobbasefeeScalarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::daFootprintGasScalar(_) => {
                    <daFootprintGasScalarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delayedWETH(_) => {
                    <delayedWETHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disputeGameFactory(_) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip1559Denominator(_) => {
                    <eip1559DenominatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip1559Elasticity(_) => {
                    <eip1559ElasticityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasLimit(_) => <gasLimitCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getAddresses(_) => {
                    <getAddressesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::guardian(_) => <guardianCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initVersion(_) => {
                    <initVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCustomGasToken(_) => {
                    <isCustomGasTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isFeatureEnabled(_) => {
                    <isFeatureEnabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l1CrossDomainMessenger(_) => {
                    <l1CrossDomainMessengerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l1ERC721Bridge(_) => {
                    <l1ERC721BridgeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l1StandardBridge(_) => {
                    <l1StandardBridgeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2ChainId(_) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maximumGasLimit(_) => {
                    <maximumGasLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minBaseFee(_) => {
                    <minBaseFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumGasLimit(_) => {
                    <minimumGasLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorFeeConstant(_) => {
                    <operatorFeeConstantCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorFeeScalar(_) => {
                    <operatorFeeScalarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::optimismMintableERC20Factory(_) => {
                    <optimismMintableERC20FactoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::optimismPortal(_) => {
                    <optimismPortalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::overhead(_) => <overheadCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxyAdmin(_) => {
                    <proxyAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxyAdminOwner(_) => {
                    <proxyAdminOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resourceConfig(_) => {
                    <resourceConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scalar(_) => <scalarCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setBatcherHash_0(_) => {
                    <setBatcherHash_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBatcherHash_1(_) => {
                    <setBatcherHash_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setDAFootprintGasScalar(_) => {
                    <setDAFootprintGasScalarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEIP1559Params(_) => {
                    <setEIP1559ParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setFeature(_) => {
                    <setFeatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setGasConfig(_) => {
                    <setGasConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setGasConfigEcotone(_) => {
                    <setGasConfigEcotoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setGasLimit(_) => {
                    <setGasLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMinBaseFee(_) => {
                    <setMinBaseFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorFeeScalars(_) => {
                    <setOperatorFeeScalarsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUnsafeBlockSigner(_) => {
                    <setUnsafeBlockSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startBlock(_) => {
                    <startBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::superchainConfig(_) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unsafeBlockSigner(_) => {
                    <unsafeBlockSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SystemConfigCalls>] = &[
                {
                    fn OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT)
                    }
                    OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT
                },
                {
                    fn l1StandardBridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l1StandardBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::l1StandardBridge)
                    }
                    l1StandardBridge
                },
                {
                    fn setBatcherHash_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setBatcherHash_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setBatcherHash_0)
                    }
                    setBatcherHash_0
                },
                {
                    fn optimismPortal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <optimismPortalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::optimismPortal)
                    }
                    optimismPortal
                },
                {
                    fn maximumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <maximumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::maximumGasLimit)
                    }
                    maximumGasLimit
                },
                {
                    fn overhead(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <overheadCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::overhead)
                    }
                    overhead
                },
                {
                    fn setOperatorFeeScalars(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setOperatorFeeScalarsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setOperatorFeeScalars)
                    }
                    setOperatorFeeScalars
                },
                {
                    fn operatorFeeConstant(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <operatorFeeConstantCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::operatorFeeConstant)
                    }
                    operatorFeeConstant
                },
                {
                    fn setUnsafeBlockSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setUnsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setUnsafeBlockSigner)
                    }
                    setUnsafeBlockSigner
                },
                {
                    fn L1_ERC_721_BRIDGE_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <L1_ERC_721_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::L1_ERC_721_BRIDGE_SLOT)
                    }
                    L1_ERC_721_BRIDGE_SLOT
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::initialize)
                    }
                    initialize
                },
                {
                    fn unsafeBlockSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <unsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::unsafeBlockSigner)
                    }
                    unsafeBlockSigner
                },
                {
                    fn setDAFootprintGasScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setDAFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setDAFootprintGasScalar)
                    }
                    setDAFootprintGasScalar
                },
                {
                    fn isCustomGasToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <isCustomGasTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::isCustomGasToken)
                    }
                    isCustomGasToken
                },
                {
                    fn delayedWETH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <delayedWETHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::delayedWETH)
                    }
                    delayedWETH
                },
                {
                    fn setGasConfigEcotone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setGasConfigEcotoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setGasConfigEcotone)
                    }
                    setGasConfigEcotone
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn initVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <initVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::initVersion)
                    }
                    initVersion
                },
                {
                    fn proxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <proxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::proxyAdmin)
                    }
                    proxyAdmin
                },
                {
                    fn guardian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <guardianCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::guardian)
                    }
                    guardian
                },
                {
                    fn isFeatureEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <isFeatureEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::isFeatureEnabled)
                    }
                    isFeatureEnabled
                },
                {
                    fn startBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <startBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::startBlock)
                    }
                    startBlock
                },
                {
                    fn minimumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::minimumGasLimit)
                    }
                    minimumGasLimit
                },
                {
                    fn operatorFeeScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <operatorFeeScalarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::operatorFeeScalar)
                    }
                    operatorFeeScalar
                },
                {
                    fn UNSAFE_BLOCK_SIGNER_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <UNSAFE_BLOCK_SIGNER_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::UNSAFE_BLOCK_SIGNER_SLOT)
                    }
                    UNSAFE_BLOCK_SIGNER_SLOT
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::version)
                    }
                    version
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::paused)
                    }
                    paused
                },
                {
                    fn L1_CROSS_DOMAIN_MESSENGER_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <L1_CROSS_DOMAIN_MESSENGER_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::L1_CROSS_DOMAIN_MESSENGER_SLOT)
                    }
                    L1_CROSS_DOMAIN_MESSENGER_SLOT
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn setMinBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setMinBaseFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setMinBaseFee)
                    }
                    setMinBaseFee
                },
                {
                    fn DELAYED_WETH_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <DELAYED_WETH_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::DELAYED_WETH_SLOT)
                    }
                    DELAYED_WETH_SLOT
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::owner)
                    }
                    owner
                },
                {
                    fn setGasConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setGasConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setGasConfig)
                    }
                    setGasConfig
                },
                {
                    fn optimismMintableERC20Factory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <optimismMintableERC20FactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::optimismMintableERC20Factory)
                    }
                    optimismMintableERC20Factory
                },
                {
                    fn getAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <getAddressesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::getAddresses)
                    }
                    getAddresses
                },
                {
                    fn minBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <minBaseFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::minBaseFee)
                    }
                    minBaseFee
                },
                {
                    fn l1CrossDomainMessenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l1CrossDomainMessengerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::l1CrossDomainMessenger)
                    }
                    l1CrossDomainMessenger
                },
                {
                    fn setGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setGasLimit)
                    }
                    setGasLimit
                },
                {
                    fn BATCH_INBOX_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <BATCH_INBOX_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::BATCH_INBOX_SLOT)
                    }
                    BATCH_INBOX_SLOT
                },
                {
                    fn basefeeScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <basefeeScalarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::basefeeScalar)
                    }
                    basefeeScalar
                },
                {
                    fn setEIP1559Params(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setEIP1559ParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setEIP1559Params)
                    }
                    setEIP1559Params
                },
                {
                    fn l1ERC721Bridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l1ERC721BridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::l1ERC721Bridge)
                    }
                    l1ERC721Bridge
                },
                {
                    fn setBatcherHash_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setBatcherHash_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setBatcherHash_1)
                    }
                    setBatcherHash_1
                },
                {
                    fn eip1559Elasticity(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <eip1559ElasticityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::eip1559Elasticity)
                    }
                    eip1559Elasticity
                },
                {
                    fn resourceConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <resourceConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::resourceConfig)
                    }
                    resourceConfig
                },
                {
                    fn eip1559Denominator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <eip1559DenominatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::eip1559Denominator)
                    }
                    eip1559Denominator
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn batchInbox(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <batchInboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::batchInbox)
                    }
                    batchInbox
                },
                {
                    fn proxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::proxyAdminOwner)
                    }
                    proxyAdminOwner
                },
                {
                    fn START_BLOCK_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <START_BLOCK_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::START_BLOCK_SLOT)
                    }
                    START_BLOCK_SLOT
                },
                {
                    fn batcherHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <batcherHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::batcherHash)
                    }
                    batcherHash
                },
                {
                    fn blobbasefeeScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <blobbasefeeScalarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::blobbasefeeScalar)
                    }
                    blobbasefeeScalar
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::disputeGameFactory)
                    }
                    disputeGameFactory
                },
                {
                    fn setFeature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setFeatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::setFeature)
                    }
                    setFeature
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn scalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <scalarCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::scalar)
                    }
                    scalar
                },
                {
                    fn gasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <gasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::gasLimit)
                    }
                    gasLimit
                },
                {
                    fn L1_STANDARD_BRIDGE_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <L1_STANDARD_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::L1_STANDARD_BRIDGE_SLOT)
                    }
                    L1_STANDARD_BRIDGE_SLOT
                },
                {
                    fn OPTIMISM_PORTAL_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <OPTIMISM_PORTAL_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::OPTIMISM_PORTAL_SLOT)
                    }
                    OPTIMISM_PORTAL_SLOT
                },
                {
                    fn daFootprintGasScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <daFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigCalls::daFootprintGasScalar)
                    }
                    daFootprintGasScalar
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SystemConfigCalls::VERSION)
                    }
                    VERSION
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SystemConfigCalls>] = &[
                {
                    fn OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT)
                    }
                    OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT
                },
                {
                    fn l1StandardBridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l1StandardBridgeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::l1StandardBridge)
                    }
                    l1StandardBridge
                },
                {
                    fn setBatcherHash_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setBatcherHash_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setBatcherHash_0)
                    }
                    setBatcherHash_0
                },
                {
                    fn optimismPortal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <optimismPortalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::optimismPortal)
                    }
                    optimismPortal
                },
                {
                    fn maximumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <maximumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::maximumGasLimit)
                    }
                    maximumGasLimit
                },
                {
                    fn overhead(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <overheadCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::overhead)
                    }
                    overhead
                },
                {
                    fn setOperatorFeeScalars(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setOperatorFeeScalarsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setOperatorFeeScalars)
                    }
                    setOperatorFeeScalars
                },
                {
                    fn operatorFeeConstant(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <operatorFeeConstantCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::operatorFeeConstant)
                    }
                    operatorFeeConstant
                },
                {
                    fn setUnsafeBlockSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setUnsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setUnsafeBlockSigner)
                    }
                    setUnsafeBlockSigner
                },
                {
                    fn L1_ERC_721_BRIDGE_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <L1_ERC_721_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::L1_ERC_721_BRIDGE_SLOT)
                    }
                    L1_ERC_721_BRIDGE_SLOT
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::initialize)
                    }
                    initialize
                },
                {
                    fn unsafeBlockSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <unsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::unsafeBlockSigner)
                    }
                    unsafeBlockSigner
                },
                {
                    fn setDAFootprintGasScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setDAFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setDAFootprintGasScalar)
                    }
                    setDAFootprintGasScalar
                },
                {
                    fn isCustomGasToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <isCustomGasTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::isCustomGasToken)
                    }
                    isCustomGasToken
                },
                {
                    fn delayedWETH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <delayedWETHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::delayedWETH)
                    }
                    delayedWETH
                },
                {
                    fn setGasConfigEcotone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setGasConfigEcotoneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setGasConfigEcotone)
                    }
                    setGasConfigEcotone
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn initVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <initVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::initVersion)
                    }
                    initVersion
                },
                {
                    fn proxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <proxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::proxyAdmin)
                    }
                    proxyAdmin
                },
                {
                    fn guardian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <guardianCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::guardian)
                    }
                    guardian
                },
                {
                    fn isFeatureEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <isFeatureEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::isFeatureEnabled)
                    }
                    isFeatureEnabled
                },
                {
                    fn startBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <startBlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::startBlock)
                    }
                    startBlock
                },
                {
                    fn minimumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::minimumGasLimit)
                    }
                    minimumGasLimit
                },
                {
                    fn operatorFeeScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <operatorFeeScalarCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::operatorFeeScalar)
                    }
                    operatorFeeScalar
                },
                {
                    fn UNSAFE_BLOCK_SIGNER_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <UNSAFE_BLOCK_SIGNER_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::UNSAFE_BLOCK_SIGNER_SLOT)
                    }
                    UNSAFE_BLOCK_SIGNER_SLOT
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::version)
                    }
                    version
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::paused)
                    }
                    paused
                },
                {
                    fn L1_CROSS_DOMAIN_MESSENGER_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <L1_CROSS_DOMAIN_MESSENGER_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::L1_CROSS_DOMAIN_MESSENGER_SLOT)
                    }
                    L1_CROSS_DOMAIN_MESSENGER_SLOT
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn setMinBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setMinBaseFeeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setMinBaseFee)
                    }
                    setMinBaseFee
                },
                {
                    fn DELAYED_WETH_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <DELAYED_WETH_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::DELAYED_WETH_SLOT)
                    }
                    DELAYED_WETH_SLOT
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::owner)
                    }
                    owner
                },
                {
                    fn setGasConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setGasConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setGasConfig)
                    }
                    setGasConfig
                },
                {
                    fn optimismMintableERC20Factory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <optimismMintableERC20FactoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::optimismMintableERC20Factory)
                    }
                    optimismMintableERC20Factory
                },
                {
                    fn getAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <getAddressesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::getAddresses)
                    }
                    getAddresses
                },
                {
                    fn minBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <minBaseFeeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::minBaseFee)
                    }
                    minBaseFee
                },
                {
                    fn l1CrossDomainMessenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l1CrossDomainMessengerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::l1CrossDomainMessenger)
                    }
                    l1CrossDomainMessenger
                },
                {
                    fn setGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setGasLimit)
                    }
                    setGasLimit
                },
                {
                    fn BATCH_INBOX_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <BATCH_INBOX_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::BATCH_INBOX_SLOT)
                    }
                    BATCH_INBOX_SLOT
                },
                {
                    fn basefeeScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <basefeeScalarCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::basefeeScalar)
                    }
                    basefeeScalar
                },
                {
                    fn setEIP1559Params(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setEIP1559ParamsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setEIP1559Params)
                    }
                    setEIP1559Params
                },
                {
                    fn l1ERC721Bridge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l1ERC721BridgeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::l1ERC721Bridge)
                    }
                    l1ERC721Bridge
                },
                {
                    fn setBatcherHash_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setBatcherHash_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setBatcherHash_1)
                    }
                    setBatcherHash_1
                },
                {
                    fn eip1559Elasticity(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <eip1559ElasticityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::eip1559Elasticity)
                    }
                    eip1559Elasticity
                },
                {
                    fn resourceConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <resourceConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::resourceConfig)
                    }
                    resourceConfig
                },
                {
                    fn eip1559Denominator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <eip1559DenominatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::eip1559Denominator)
                    }
                    eip1559Denominator
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn batchInbox(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <batchInboxCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::batchInbox)
                    }
                    batchInbox
                },
                {
                    fn proxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::proxyAdminOwner)
                    }
                    proxyAdminOwner
                },
                {
                    fn START_BLOCK_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <START_BLOCK_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::START_BLOCK_SLOT)
                    }
                    START_BLOCK_SLOT
                },
                {
                    fn batcherHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <batcherHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::batcherHash)
                    }
                    batcherHash
                },
                {
                    fn blobbasefeeScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <blobbasefeeScalarCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::blobbasefeeScalar)
                    }
                    blobbasefeeScalar
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::disputeGameFactory)
                    }
                    disputeGameFactory
                },
                {
                    fn setFeature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <setFeatureCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::setFeature)
                    }
                    setFeature
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn scalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <scalarCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::scalar)
                    }
                    scalar
                },
                {
                    fn gasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <gasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::gasLimit)
                    }
                    gasLimit
                },
                {
                    fn L1_STANDARD_BRIDGE_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <L1_STANDARD_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::L1_STANDARD_BRIDGE_SLOT)
                    }
                    L1_STANDARD_BRIDGE_SLOT
                },
                {
                    fn OPTIMISM_PORTAL_SLOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <OPTIMISM_PORTAL_SLOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::OPTIMISM_PORTAL_SLOT)
                    }
                    OPTIMISM_PORTAL_SLOT
                },
                {
                    fn daFootprintGasScalar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <daFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::daFootprintGasScalar)
                    }
                    daFootprintGasScalar
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigCalls::VERSION)
                    }
                    VERSION
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BATCH_INBOX_SLOT(inner) => {
                    <BATCH_INBOX_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DELAYED_WETH_SLOT(inner) => {
                    <DELAYED_WETH_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::L1_CROSS_DOMAIN_MESSENGER_SLOT(inner) => {
                    <L1_CROSS_DOMAIN_MESSENGER_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::L1_ERC_721_BRIDGE_SLOT(inner) => {
                    <L1_ERC_721_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::L1_STANDARD_BRIDGE_SLOT(inner) => {
                    <L1_STANDARD_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT(inner) => {
                    <OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OPTIMISM_PORTAL_SLOT(inner) => {
                    <OPTIMISM_PORTAL_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::START_BLOCK_SLOT(inner) => {
                    <START_BLOCK_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UNSAFE_BLOCK_SIGNER_SLOT(inner) => {
                    <UNSAFE_BLOCK_SIGNER_SLOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::basefeeScalar(inner) => {
                    <basefeeScalarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::batchInbox(inner) => {
                    <batchInboxCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::batcherHash(inner) => {
                    <batcherHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blobbasefeeScalar(inner) => {
                    <blobbasefeeScalarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::daFootprintGasScalar(inner) => {
                    <daFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delayedWETH(inner) => {
                    <delayedWETHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disputeGameFactory(inner) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip1559Denominator(inner) => {
                    <eip1559DenominatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip1559Elasticity(inner) => {
                    <eip1559ElasticityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gasLimit(inner) => {
                    <gasLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getAddresses(inner) => {
                    <getAddressesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::guardian(inner) => {
                    <guardianCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initVersion(inner) => {
                    <initVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isCustomGasToken(inner) => {
                    <isCustomGasTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isFeatureEnabled(inner) => {
                    <isFeatureEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l1CrossDomainMessenger(inner) => {
                    <l1CrossDomainMessengerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l1ERC721Bridge(inner) => {
                    <l1ERC721BridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l1StandardBridge(inner) => {
                    <l1StandardBridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2ChainId(inner) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::maximumGasLimit(inner) => {
                    <maximumGasLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minBaseFee(inner) => {
                    <minBaseFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::minimumGasLimit(inner) => {
                    <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorFeeConstant(inner) => {
                    <operatorFeeConstantCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorFeeScalar(inner) => {
                    <operatorFeeScalarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::optimismMintableERC20Factory(inner) => {
                    <optimismMintableERC20FactoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::optimismPortal(inner) => {
                    <optimismPortalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::overhead(inner) => {
                    <overheadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxyAdmin(inner) => {
                    <proxyAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxyAdminOwner(inner) => {
                    <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resourceConfig(inner) => {
                    <resourceConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scalar(inner) => {
                    <scalarCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setBatcherHash_0(inner) => {
                    <setBatcherHash_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setBatcherHash_1(inner) => {
                    <setBatcherHash_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setDAFootprintGasScalar(inner) => {
                    <setDAFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEIP1559Params(inner) => {
                    <setEIP1559ParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setFeature(inner) => {
                    <setFeatureCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setGasConfig(inner) => {
                    <setGasConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setGasConfigEcotone(inner) => {
                    <setGasConfigEcotoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setGasLimit(inner) => {
                    <setGasLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMinBaseFee(inner) => {
                    <setMinBaseFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperatorFeeScalars(inner) => {
                    <setOperatorFeeScalarsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setUnsafeBlockSigner(inner) => {
                    <setUnsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startBlock(inner) => {
                    <startBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::superchainConfig(inner) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unsafeBlockSigner(inner) => {
                    <unsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BATCH_INBOX_SLOT(inner) => {
                    <BATCH_INBOX_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DELAYED_WETH_SLOT(inner) => {
                    <DELAYED_WETH_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::L1_CROSS_DOMAIN_MESSENGER_SLOT(inner) => {
                    <L1_CROSS_DOMAIN_MESSENGER_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::L1_ERC_721_BRIDGE_SLOT(inner) => {
                    <L1_ERC_721_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::L1_STANDARD_BRIDGE_SLOT(inner) => {
                    <L1_STANDARD_BRIDGE_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT(inner) => {
                    <OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OPTIMISM_PORTAL_SLOT(inner) => {
                    <OPTIMISM_PORTAL_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::START_BLOCK_SLOT(inner) => {
                    <START_BLOCK_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UNSAFE_BLOCK_SIGNER_SLOT(inner) => {
                    <UNSAFE_BLOCK_SIGNER_SLOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::basefeeScalar(inner) => {
                    <basefeeScalarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::batchInbox(inner) => {
                    <batchInboxCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::batcherHash(inner) => {
                    <batcherHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blobbasefeeScalar(inner) => {
                    <blobbasefeeScalarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::daFootprintGasScalar(inner) => {
                    <daFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delayedWETH(inner) => {
                    <delayedWETHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disputeGameFactory(inner) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip1559Denominator(inner) => {
                    <eip1559DenominatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip1559Elasticity(inner) => {
                    <eip1559ElasticityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gasLimit(inner) => {
                    <gasLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAddresses(inner) => {
                    <getAddressesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::guardian(inner) => {
                    <guardianCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initVersion(inner) => {
                    <initVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isCustomGasToken(inner) => {
                    <isCustomGasTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isFeatureEnabled(inner) => {
                    <isFeatureEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l1CrossDomainMessenger(inner) => {
                    <l1CrossDomainMessengerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l1ERC721Bridge(inner) => {
                    <l1ERC721BridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l1StandardBridge(inner) => {
                    <l1StandardBridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2ChainId(inner) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maximumGasLimit(inner) => {
                    <maximumGasLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minBaseFee(inner) => {
                    <minBaseFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumGasLimit(inner) => {
                    <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorFeeConstant(inner) => {
                    <operatorFeeConstantCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorFeeScalar(inner) => {
                    <operatorFeeScalarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::optimismMintableERC20Factory(inner) => {
                    <optimismMintableERC20FactoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::optimismPortal(inner) => {
                    <optimismPortalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::overhead(inner) => {
                    <overheadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proxyAdmin(inner) => {
                    <proxyAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proxyAdminOwner(inner) => {
                    <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resourceConfig(inner) => {
                    <resourceConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scalar(inner) => {
                    <scalarCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setBatcherHash_0(inner) => {
                    <setBatcherHash_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setBatcherHash_1(inner) => {
                    <setBatcherHash_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setDAFootprintGasScalar(inner) => {
                    <setDAFootprintGasScalarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEIP1559Params(inner) => {
                    <setEIP1559ParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setFeature(inner) => {
                    <setFeatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setGasConfig(inner) => {
                    <setGasConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setGasConfigEcotone(inner) => {
                    <setGasConfigEcotoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setGasLimit(inner) => {
                    <setGasLimitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMinBaseFee(inner) => {
                    <setMinBaseFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorFeeScalars(inner) => {
                    <setOperatorFeeScalarsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setUnsafeBlockSigner(inner) => {
                    <setUnsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startBlock(inner) => {
                    <startBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::superchainConfig(inner) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unsafeBlockSigner(inner) => {
                    <unsafeBlockSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SystemConfig`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SystemConfigErrors {
        #[allow(missing_docs)]
        ProxyAdminOwnedBase_NotProxyAdmin(ProxyAdminOwnedBase_NotProxyAdmin),
        #[allow(missing_docs)]
        ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(
            ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner,
        ),
        #[allow(missing_docs)]
        ProxyAdminOwnedBase_NotProxyAdminOwner(ProxyAdminOwnedBase_NotProxyAdminOwner),
        #[allow(missing_docs)]
        ProxyAdminOwnedBase_NotResolvedDelegateProxy(
            ProxyAdminOwnedBase_NotResolvedDelegateProxy,
        ),
        #[allow(missing_docs)]
        ProxyAdminOwnedBase_NotSharedProxyAdminOwner(
            ProxyAdminOwnedBase_NotSharedProxyAdminOwner,
        ),
        #[allow(missing_docs)]
        ProxyAdminOwnedBase_ProxyAdminNotFound(ProxyAdminOwnedBase_ProxyAdminNotFound),
        #[allow(missing_docs)]
        ReinitializableBase_ZeroInitVersion(ReinitializableBase_ZeroInitVersion),
        #[allow(missing_docs)]
        SystemConfig_InvalidFeatureState(SystemConfig_InvalidFeatureState),
    }
    impl SystemConfigErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 92u8, 67u8, 20u8],
            [51u8, 33u8, 68u8, 219u8],
            [84u8, 228u8, 51u8, 205u8],
            [127u8, 18u8, 198u8, 75u8],
            [155u8, 1u8, 175u8, 237u8],
            [196u8, 5u8, 10u8, 38u8],
            [232u8, 24u8, 220u8, 195u8],
            [245u8, 130u8, 139u8, 4u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ProxyAdminOwnedBase_NotSharedProxyAdminOwner),
            ::core::stringify!(ProxyAdminOwnedBase_ProxyAdminNotFound),
            ::core::stringify!(ProxyAdminOwnedBase_NotResolvedDelegateProxy),
            ::core::stringify!(ProxyAdminOwnedBase_NotProxyAdminOwner),
            ::core::stringify!(ReinitializableBase_ZeroInitVersion),
            ::core::stringify!(ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner),
            ::core::stringify!(ProxyAdminOwnedBase_NotProxyAdmin),
            ::core::stringify!(SystemConfig_InvalidFeatureState),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::SIGNATURE,
            <SystemConfig_InvalidFeatureState as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SystemConfigErrors {
        const NAME: &'static str = "SystemConfigErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ProxyAdminOwnedBase_NotProxyAdmin(_) => {
                    <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(_) => {
                    <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProxyAdminOwnedBase_NotProxyAdminOwner(_) => {
                    <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProxyAdminOwnedBase_NotResolvedDelegateProxy(_) => {
                    <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProxyAdminOwnedBase_NotSharedProxyAdminOwner(_) => {
                    <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProxyAdminOwnedBase_ProxyAdminNotFound(_) => {
                    <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReinitializableBase_ZeroInitVersion(_) => {
                    <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SystemConfig_InvalidFeatureState(_) => {
                    <SystemConfig_InvalidFeatureState as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SystemConfigErrors>] = &[
                {
                    fn ProxyAdminOwnedBase_NotSharedProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotSharedProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotSharedProxyAdminOwner
                },
                {
                    fn ProxyAdminOwnedBase_ProxyAdminNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_ProxyAdminNotFound,
                            )
                    }
                    ProxyAdminOwnedBase_ProxyAdminNotFound
                },
                {
                    fn ProxyAdminOwnedBase_NotResolvedDelegateProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotResolvedDelegateProxy,
                            )
                    }
                    ProxyAdminOwnedBase_NotResolvedDelegateProxy
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOwner
                },
                {
                    fn ReinitializableBase_ZeroInitVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigErrors::ReinitializableBase_ZeroInitVersion)
                    }
                    ReinitializableBase_ZeroInitVersion
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigErrors::ProxyAdminOwnedBase_NotProxyAdmin)
                    }
                    ProxyAdminOwnedBase_NotProxyAdmin
                },
                {
                    fn SystemConfig_InvalidFeatureState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <SystemConfig_InvalidFeatureState as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SystemConfigErrors::SystemConfig_InvalidFeatureState)
                    }
                    SystemConfig_InvalidFeatureState
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SystemConfigErrors>] = &[
                {
                    fn ProxyAdminOwnedBase_NotSharedProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotSharedProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotSharedProxyAdminOwner
                },
                {
                    fn ProxyAdminOwnedBase_ProxyAdminNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_ProxyAdminNotFound,
                            )
                    }
                    ProxyAdminOwnedBase_ProxyAdminNotFound
                },
                {
                    fn ProxyAdminOwnedBase_NotResolvedDelegateProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotResolvedDelegateProxy,
                            )
                    }
                    ProxyAdminOwnedBase_NotResolvedDelegateProxy
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOwner
                },
                {
                    fn ReinitializableBase_ZeroInitVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigErrors::ReinitializableBase_ZeroInitVersion)
                    }
                    ReinitializableBase_ZeroInitVersion
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SystemConfigErrors::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigErrors::ProxyAdminOwnedBase_NotProxyAdmin)
                    }
                    ProxyAdminOwnedBase_NotProxyAdmin
                },
                {
                    fn SystemConfig_InvalidFeatureState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SystemConfigErrors> {
                        <SystemConfig_InvalidFeatureState as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SystemConfigErrors::SystemConfig_InvalidFeatureState)
                    }
                    SystemConfig_InvalidFeatureState
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::ProxyAdminOwnedBase_NotProxyAdmin(inner) => {
                    <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(inner) => {
                    <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProxyAdminOwnedBase_NotProxyAdminOwner(inner) => {
                    <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProxyAdminOwnedBase_NotResolvedDelegateProxy(inner) => {
                    <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProxyAdminOwnedBase_NotSharedProxyAdminOwner(inner) => {
                    <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProxyAdminOwnedBase_ProxyAdminNotFound(inner) => {
                    <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReinitializableBase_ZeroInitVersion(inner) => {
                    <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SystemConfig_InvalidFeatureState(inner) => {
                    <SystemConfig_InvalidFeatureState as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ProxyAdminOwnedBase_NotProxyAdmin(inner) => {
                    <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(inner) => {
                    <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProxyAdminOwnedBase_NotProxyAdminOwner(inner) => {
                    <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProxyAdminOwnedBase_NotResolvedDelegateProxy(inner) => {
                    <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProxyAdminOwnedBase_NotSharedProxyAdminOwner(inner) => {
                    <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProxyAdminOwnedBase_ProxyAdminNotFound(inner) => {
                    <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReinitializableBase_ZeroInitVersion(inner) => {
                    <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SystemConfig_InvalidFeatureState(inner) => {
                    <SystemConfig_InvalidFeatureState as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SystemConfig`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SystemConfigEvents {
        #[allow(missing_docs)]
        ConfigUpdate(ConfigUpdate),
        #[allow(missing_docs)]
        FeatureSet(FeatureSet),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
    }
    impl SystemConfigEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                29u8, 43u8, 11u8, 218u8, 33u8, 213u8, 107u8, 139u8, 209u8, 45u8, 79u8,
                148u8, 235u8, 172u8, 255u8, 223u8, 179u8, 95u8, 94u8, 34u8, 111u8, 132u8,
                180u8, 97u8, 16u8, 59u8, 184u8, 190u8, 171u8, 99u8, 83u8, 190u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                184u8, 118u8, 246u8, 89u8, 65u8, 50u8, 200u8, 152u8, 145u8, 210u8, 253u8,
                25u8, 142u8, 146u8, 94u8, 153u8, 155u8, 231u8, 65u8, 236u8, 128u8, 154u8,
                187u8, 88u8, 191u8, 233u8, 185u8, 102u8, 135u8, 108u8, 192u8, 108u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ConfigUpdate),
            ::core::stringify!(Initialized),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(FeatureSet),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ConfigUpdate as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <FeatureSet as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SystemConfigEvents {
        const NAME: &'static str = "SystemConfigEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ConfigUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ConfigUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ConfigUpdate)
                }
                Some(<FeatureSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <FeatureSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::FeatureSet)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for SystemConfigEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ConfigUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FeatureSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ConfigUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FeatureSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SystemConfig`](self) contract instance.

See the [wrapper's documentation](`SystemConfigInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> SystemConfigInstance<P, N> {
        SystemConfigInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SystemConfigInstance<P, N>>,
    > {
        SystemConfigInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        SystemConfigInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`SystemConfig`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SystemConfig`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SystemConfigInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SystemConfigInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SystemConfigInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SystemConfigInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SystemConfig`](self) contract instance.

See the [wrapper's documentation](`SystemConfigInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<SystemConfigInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> SystemConfigInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SystemConfigInstance<P, N> {
            SystemConfigInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SystemConfigInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`BATCH_INBOX_SLOT`] function.
        pub fn BATCH_INBOX_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, BATCH_INBOX_SLOTCall, N> {
            self.call_builder(&BATCH_INBOX_SLOTCall)
        }
        ///Creates a new call builder for the [`DELAYED_WETH_SLOT`] function.
        pub fn DELAYED_WETH_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DELAYED_WETH_SLOTCall, N> {
            self.call_builder(&DELAYED_WETH_SLOTCall)
        }
        ///Creates a new call builder for the [`L1_CROSS_DOMAIN_MESSENGER_SLOT`] function.
        pub fn L1_CROSS_DOMAIN_MESSENGER_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, L1_CROSS_DOMAIN_MESSENGER_SLOTCall, N> {
            self.call_builder(&L1_CROSS_DOMAIN_MESSENGER_SLOTCall)
        }
        ///Creates a new call builder for the [`L1_ERC_721_BRIDGE_SLOT`] function.
        pub fn L1_ERC_721_BRIDGE_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, L1_ERC_721_BRIDGE_SLOTCall, N> {
            self.call_builder(&L1_ERC_721_BRIDGE_SLOTCall)
        }
        ///Creates a new call builder for the [`L1_STANDARD_BRIDGE_SLOT`] function.
        pub fn L1_STANDARD_BRIDGE_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, L1_STANDARD_BRIDGE_SLOTCall, N> {
            self.call_builder(&L1_STANDARD_BRIDGE_SLOTCall)
        }
        ///Creates a new call builder for the [`OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT`] function.
        pub fn OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall,
            N,
        > {
            self.call_builder(&OPTIMISM_MINTABLE_ERC20_FACTORY_SLOTCall)
        }
        ///Creates a new call builder for the [`OPTIMISM_PORTAL_SLOT`] function.
        pub fn OPTIMISM_PORTAL_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, OPTIMISM_PORTAL_SLOTCall, N> {
            self.call_builder(&OPTIMISM_PORTAL_SLOTCall)
        }
        ///Creates a new call builder for the [`START_BLOCK_SLOT`] function.
        pub fn START_BLOCK_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, START_BLOCK_SLOTCall, N> {
            self.call_builder(&START_BLOCK_SLOTCall)
        }
        ///Creates a new call builder for the [`UNSAFE_BLOCK_SIGNER_SLOT`] function.
        pub fn UNSAFE_BLOCK_SIGNER_SLOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UNSAFE_BLOCK_SIGNER_SLOTCall, N> {
            self.call_builder(&UNSAFE_BLOCK_SIGNER_SLOTCall)
        }
        ///Creates a new call builder for the [`VERSION`] function.
        pub fn VERSION(&self) -> alloy_contract::SolCallBuilder<&P, VERSIONCall, N> {
            self.call_builder(&VERSIONCall)
        }
        ///Creates a new call builder for the [`basefeeScalar`] function.
        pub fn basefeeScalar(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, basefeeScalarCall, N> {
            self.call_builder(&basefeeScalarCall)
        }
        ///Creates a new call builder for the [`batchInbox`] function.
        pub fn batchInbox(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, batchInboxCall, N> {
            self.call_builder(&batchInboxCall)
        }
        ///Creates a new call builder for the [`batcherHash`] function.
        pub fn batcherHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, batcherHashCall, N> {
            self.call_builder(&batcherHashCall)
        }
        ///Creates a new call builder for the [`blobbasefeeScalar`] function.
        pub fn blobbasefeeScalar(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, blobbasefeeScalarCall, N> {
            self.call_builder(&blobbasefeeScalarCall)
        }
        ///Creates a new call builder for the [`daFootprintGasScalar`] function.
        pub fn daFootprintGasScalar(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, daFootprintGasScalarCall, N> {
            self.call_builder(&daFootprintGasScalarCall)
        }
        ///Creates a new call builder for the [`delayedWETH`] function.
        pub fn delayedWETH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, delayedWETHCall, N> {
            self.call_builder(&delayedWETHCall)
        }
        ///Creates a new call builder for the [`disputeGameFactory`] function.
        pub fn disputeGameFactory(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, disputeGameFactoryCall, N> {
            self.call_builder(&disputeGameFactoryCall)
        }
        ///Creates a new call builder for the [`eip1559Denominator`] function.
        pub fn eip1559Denominator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip1559DenominatorCall, N> {
            self.call_builder(&eip1559DenominatorCall)
        }
        ///Creates a new call builder for the [`eip1559Elasticity`] function.
        pub fn eip1559Elasticity(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip1559ElasticityCall, N> {
            self.call_builder(&eip1559ElasticityCall)
        }
        ///Creates a new call builder for the [`gasLimit`] function.
        pub fn gasLimit(&self) -> alloy_contract::SolCallBuilder<&P, gasLimitCall, N> {
            self.call_builder(&gasLimitCall)
        }
        ///Creates a new call builder for the [`getAddresses`] function.
        pub fn getAddresses(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getAddressesCall, N> {
            self.call_builder(&getAddressesCall)
        }
        ///Creates a new call builder for the [`guardian`] function.
        pub fn guardian(&self) -> alloy_contract::SolCallBuilder<&P, guardianCall, N> {
            self.call_builder(&guardianCall)
        }
        ///Creates a new call builder for the [`initVersion`] function.
        pub fn initVersion(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, initVersionCall, N> {
            self.call_builder(&initVersionCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _owner: alloy::sol_types::private::Address,
            _basefeeScalar: u32,
            _blobbasefeeScalar: u32,
            _batcherHash: alloy::sol_types::private::FixedBytes<32>,
            _gasLimit: u64,
            _unsafeBlockSigner: alloy::sol_types::private::Address,
            _config: <IResourceMetering::ResourceConfig as alloy::sol_types::SolType>::RustType,
            _batchInbox: alloy::sol_types::private::Address,
            _addresses: <Addresses as alloy::sol_types::SolType>::RustType,
            _l2ChainId: alloy::sol_types::private::primitives::aliases::U256,
            _superchainConfig: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _owner,
                    _basefeeScalar,
                    _blobbasefeeScalar,
                    _batcherHash,
                    _gasLimit,
                    _unsafeBlockSigner,
                    _config,
                    _batchInbox,
                    _addresses,
                    _l2ChainId,
                    _superchainConfig,
                },
            )
        }
        ///Creates a new call builder for the [`isCustomGasToken`] function.
        pub fn isCustomGasToken(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, isCustomGasTokenCall, N> {
            self.call_builder(&isCustomGasTokenCall)
        }
        ///Creates a new call builder for the [`isFeatureEnabled`] function.
        pub fn isFeatureEnabled(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isFeatureEnabledCall, N> {
            self.call_builder(&isFeatureEnabledCall(_0))
        }
        ///Creates a new call builder for the [`l1CrossDomainMessenger`] function.
        pub fn l1CrossDomainMessenger(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l1CrossDomainMessengerCall, N> {
            self.call_builder(&l1CrossDomainMessengerCall)
        }
        ///Creates a new call builder for the [`l1ERC721Bridge`] function.
        pub fn l1ERC721Bridge(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l1ERC721BridgeCall, N> {
            self.call_builder(&l1ERC721BridgeCall)
        }
        ///Creates a new call builder for the [`l1StandardBridge`] function.
        pub fn l1StandardBridge(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l1StandardBridgeCall, N> {
            self.call_builder(&l1StandardBridgeCall)
        }
        ///Creates a new call builder for the [`l2ChainId`] function.
        pub fn l2ChainId(&self) -> alloy_contract::SolCallBuilder<&P, l2ChainIdCall, N> {
            self.call_builder(&l2ChainIdCall)
        }
        ///Creates a new call builder for the [`maximumGasLimit`] function.
        pub fn maximumGasLimit(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, maximumGasLimitCall, N> {
            self.call_builder(&maximumGasLimitCall)
        }
        ///Creates a new call builder for the [`minBaseFee`] function.
        pub fn minBaseFee(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, minBaseFeeCall, N> {
            self.call_builder(&minBaseFeeCall)
        }
        ///Creates a new call builder for the [`minimumGasLimit`] function.
        pub fn minimumGasLimit(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, minimumGasLimitCall, N> {
            self.call_builder(&minimumGasLimitCall)
        }
        ///Creates a new call builder for the [`operatorFeeConstant`] function.
        pub fn operatorFeeConstant(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, operatorFeeConstantCall, N> {
            self.call_builder(&operatorFeeConstantCall)
        }
        ///Creates a new call builder for the [`operatorFeeScalar`] function.
        pub fn operatorFeeScalar(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, operatorFeeScalarCall, N> {
            self.call_builder(&operatorFeeScalarCall)
        }
        ///Creates a new call builder for the [`optimismMintableERC20Factory`] function.
        pub fn optimismMintableERC20Factory(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, optimismMintableERC20FactoryCall, N> {
            self.call_builder(&optimismMintableERC20FactoryCall)
        }
        ///Creates a new call builder for the [`optimismPortal`] function.
        pub fn optimismPortal(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, optimismPortalCall, N> {
            self.call_builder(&optimismPortalCall)
        }
        ///Creates a new call builder for the [`overhead`] function.
        pub fn overhead(&self) -> alloy_contract::SolCallBuilder<&P, overheadCall, N> {
            self.call_builder(&overheadCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`proxyAdmin`] function.
        pub fn proxyAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxyAdminCall, N> {
            self.call_builder(&proxyAdminCall)
        }
        ///Creates a new call builder for the [`proxyAdminOwner`] function.
        pub fn proxyAdminOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxyAdminOwnerCall, N> {
            self.call_builder(&proxyAdminOwnerCall)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`resourceConfig`] function.
        pub fn resourceConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, resourceConfigCall, N> {
            self.call_builder(&resourceConfigCall)
        }
        ///Creates a new call builder for the [`scalar`] function.
        pub fn scalar(&self) -> alloy_contract::SolCallBuilder<&P, scalarCall, N> {
            self.call_builder(&scalarCall)
        }
        ///Creates a new call builder for the [`setBatcherHash_0`] function.
        pub fn setBatcherHash_0(
            &self,
            _batcher: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setBatcherHash_0Call, N> {
            self.call_builder(&setBatcherHash_0Call { _batcher })
        }
        ///Creates a new call builder for the [`setBatcherHash_1`] function.
        pub fn setBatcherHash_1(
            &self,
            _batcherHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, setBatcherHash_1Call, N> {
            self.call_builder(
                &setBatcherHash_1Call {
                    _batcherHash,
                },
            )
        }
        ///Creates a new call builder for the [`setDAFootprintGasScalar`] function.
        pub fn setDAFootprintGasScalar(
            &self,
            _daFootprintGasScalar: u16,
        ) -> alloy_contract::SolCallBuilder<&P, setDAFootprintGasScalarCall, N> {
            self.call_builder(
                &setDAFootprintGasScalarCall {
                    _daFootprintGasScalar,
                },
            )
        }
        ///Creates a new call builder for the [`setEIP1559Params`] function.
        pub fn setEIP1559Params(
            &self,
            _denominator: u32,
            _elasticity: u32,
        ) -> alloy_contract::SolCallBuilder<&P, setEIP1559ParamsCall, N> {
            self.call_builder(
                &setEIP1559ParamsCall {
                    _denominator,
                    _elasticity,
                },
            )
        }
        ///Creates a new call builder for the [`setFeature`] function.
        pub fn setFeature(
            &self,
            _feature: alloy::sol_types::private::FixedBytes<32>,
            _enabled: bool,
        ) -> alloy_contract::SolCallBuilder<&P, setFeatureCall, N> {
            self.call_builder(
                &setFeatureCall {
                    _feature,
                    _enabled,
                },
            )
        }
        ///Creates a new call builder for the [`setGasConfig`] function.
        pub fn setGasConfig(
            &self,
            _overhead: alloy::sol_types::private::primitives::aliases::U256,
            _scalar: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setGasConfigCall, N> {
            self.call_builder(
                &setGasConfigCall {
                    _overhead,
                    _scalar,
                },
            )
        }
        ///Creates a new call builder for the [`setGasConfigEcotone`] function.
        pub fn setGasConfigEcotone(
            &self,
            _basefeeScalar: u32,
            _blobbasefeeScalar: u32,
        ) -> alloy_contract::SolCallBuilder<&P, setGasConfigEcotoneCall, N> {
            self.call_builder(
                &setGasConfigEcotoneCall {
                    _basefeeScalar,
                    _blobbasefeeScalar,
                },
            )
        }
        ///Creates a new call builder for the [`setGasLimit`] function.
        pub fn setGasLimit(
            &self,
            _gasLimit: u64,
        ) -> alloy_contract::SolCallBuilder<&P, setGasLimitCall, N> {
            self.call_builder(&setGasLimitCall { _gasLimit })
        }
        ///Creates a new call builder for the [`setMinBaseFee`] function.
        pub fn setMinBaseFee(
            &self,
            _minBaseFee: u64,
        ) -> alloy_contract::SolCallBuilder<&P, setMinBaseFeeCall, N> {
            self.call_builder(&setMinBaseFeeCall { _minBaseFee })
        }
        ///Creates a new call builder for the [`setOperatorFeeScalars`] function.
        pub fn setOperatorFeeScalars(
            &self,
            _operatorFeeScalar: u32,
            _operatorFeeConstant: u64,
        ) -> alloy_contract::SolCallBuilder<&P, setOperatorFeeScalarsCall, N> {
            self.call_builder(
                &setOperatorFeeScalarsCall {
                    _operatorFeeScalar,
                    _operatorFeeConstant,
                },
            )
        }
        ///Creates a new call builder for the [`setUnsafeBlockSigner`] function.
        pub fn setUnsafeBlockSigner(
            &self,
            _unsafeBlockSigner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setUnsafeBlockSignerCall, N> {
            self.call_builder(
                &setUnsafeBlockSignerCall {
                    _unsafeBlockSigner,
                },
            )
        }
        ///Creates a new call builder for the [`startBlock`] function.
        pub fn startBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startBlockCall, N> {
            self.call_builder(&startBlockCall)
        }
        ///Creates a new call builder for the [`superchainConfig`] function.
        pub fn superchainConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, superchainConfigCall, N> {
            self.call_builder(&superchainConfigCall)
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unsafeBlockSigner`] function.
        pub fn unsafeBlockSigner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, unsafeBlockSignerCall, N> {
            self.call_builder(&unsafeBlockSignerCall)
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SystemConfigInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ConfigUpdate`] event.
        pub fn ConfigUpdate_filter(&self) -> alloy_contract::Event<&P, ConfigUpdate, N> {
            self.event_filter::<ConfigUpdate>()
        }
        ///Creates a new event filter for the [`FeatureSet`] event.
        pub fn FeatureSet_filter(&self) -> alloy_contract::Event<&P, FeatureSet, N> {
            self.event_filter::<FeatureSet>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
    }
}

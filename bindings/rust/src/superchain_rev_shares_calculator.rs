///Module containing a contract's types and functions.
/**

```solidity
library ISharesCalculator {
    struct ShareInfo { address recipient; uint256 amount; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISharesCalculator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ShareInfo { address recipient; uint256 amount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ShareInfo {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ShareInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ShareInfo) -> Self {
                (value.recipient, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ShareInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    recipient: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ShareInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ShareInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::SolType for ShareInfo {
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
        impl alloy_sol_types::SolStruct for ShareInfo {
            const NAME: &'static str = "ShareInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ShareInfo(address recipient,uint256 amount)",
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
                            &self.recipient,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ShareInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.recipient,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
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
                    &rust.recipient,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
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
    /**Creates a new wrapper around an on-chain [`ISharesCalculator`](self) contract instance.

See the [wrapper's documentation](`ISharesCalculatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ISharesCalculatorInstance<P, N> {
        ISharesCalculatorInstance::<P, N>::new(address, __provider)
    }
    /**A [`ISharesCalculator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISharesCalculator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISharesCalculatorInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ISharesCalculatorInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISharesCalculatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ISharesCalculatorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ISharesCalculator`](self) contract instance.

See the [wrapper's documentation](`ISharesCalculatorInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> ISharesCalculatorInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISharesCalculatorInstance<P, N> {
            ISharesCalculatorInstance {
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
    > ISharesCalculatorInstance<P, N> {
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
    > ISharesCalculatorInstance<P, N> {
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
library ISharesCalculator {
    struct ShareInfo {
        address payable recipient;
        uint256 amount;
    }
}

interface SuperchainRevSharesCalculator {
    error SharesCalculator_OnlyProxyAdminOwner();
    error SharesCalculator_ZeroGrossShare();

    event RemainderRecipientUpdated(address indexed oldRemainderRecipient, address indexed newRemainderRecipient);
    event ShareRecipientUpdated(address indexed oldShareRecipient, address indexed newShareRecipient);

    constructor(address payable _shareRecipient, address payable _remainderRecipient);

    function BASIS_POINT_SCALE() external view returns (uint32);
    function GROSS_SHARE_BPS() external view returns (uint32);
    function NET_SHARE_BPS() external view returns (uint32);
    function getRecipientsAndAmounts(uint256 _sequencerFeeRevenue, uint256 _baseFeeRevenue, uint256 _operatorFeeRevenue, uint256 _l1FeeRevenue) external view returns (ISharesCalculator.ShareInfo[] memory shareInfo_);
    function remainderRecipient() external view returns (address payable);
    function setRemainderRecipient(address payable _newRemainderRecipient) external;
    function setShareRecipient(address payable _newShareRecipient) external;
    function shareRecipient() external view returns (address payable);
    function version() external view returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_shareRecipient",
        "type": "address",
        "internalType": "address payable"
      },
      {
        "name": "_remainderRecipient",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "BASIS_POINT_SCALE",
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
    "name": "GROSS_SHARE_BPS",
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
    "name": "NET_SHARE_BPS",
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
    "name": "getRecipientsAndAmounts",
    "inputs": [
      {
        "name": "_sequencerFeeRevenue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_baseFeeRevenue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_operatorFeeRevenue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_l1FeeRevenue",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "shareInfo_",
        "type": "tuple[]",
        "internalType": "struct ISharesCalculator.ShareInfo[]",
        "components": [
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address payable"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "remainderRecipient",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setRemainderRecipient",
    "inputs": [
      {
        "name": "_newRemainderRecipient",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setShareRecipient",
    "inputs": [
      {
        "name": "_newShareRecipient",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "shareRecipient",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address payable"
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
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "RemainderRecipientUpdated",
    "inputs": [
      {
        "name": "oldRemainderRecipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newRemainderRecipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ShareRecipientUpdated",
    "inputs": [
      {
        "name": "oldShareRecipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newShareRecipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "SharesCalculator_OnlyProxyAdminOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SharesCalculator_ZeroGrossShare",
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
pub mod SuperchainRevSharesCalculator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f80fd5b50604051610955380380610955833981016040819052602b916074565b5f80546001600160a01b039384166001600160a01b0319918216179091556001805492909316911617905560a0565b80516001600160a01b0381168114606f575f80fd5b919050565b5f80604083850312156084575f80fd5b608b83605a565b9150609760208401605a565b90509250929050565b6108a8806100ad5f395ff3fe608060405234801561000f575f80fd5b506004361061009f575f3560e01c80635b201d83116100725780637b42d6c2116100585780637b42d6c21461019157806396d842be1461019a5780639c662fdd146101b9575f80fd5b80635b201d831461016b578063712d7bb814610189575f80fd5b806329f33ec8146100a35780634b012e9a146100ed57806354e7f42d1461010257806354fd4d5014610122575b5f80fd5b6001546100c39073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b6101006100fb3660046106a3565b6101cc565b005b6101156101103660046106c5565b610325565b6040516100e491906106f4565b61015e6040518060400160405280600581526020017f312e302e3000000000000000000000000000000000000000000000000000000081525081565b6040516100e49190610758565b61017461271081565b60405163ffffffff90911681526020016100e4565b61017460fa81565b6101746105dc81565b5f546100c39073ffffffffffffffffffffffffffffffffffffffff1681565b6101006101c73660046106a3565b610524565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610229573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061024d91906107ab565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146102b1576040517f67102c4c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917fc7b7459ad1ab31862524bf0c4a5e6a8518c60f27607a5c483a58ea27e30c214c9190a35050565b6040805160028082526060828101909352816020015b604080518082019091525f808252602082015281526020019060019003908161033b575050604080518082019091525f805473ffffffffffffffffffffffffffffffffffffffff16825260208201819052825192935090918391906103a2576103a26107c6565b6020026020010181905250604051806040016040528060015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f8152508160018151811061040c5761040c6107c6565b60209081029190910101525f82846104248789610820565b61042e9190610820565b6104389190610820565b90505f61271061044960fa84610839565b6104539190610850565b9050805f0361048e576040517f0174e42600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6104998584610888565b90505f6127106104ab6105dc84610839565b6104b59190610850565b90505f8184116104c557816104c7565b835b905080865f815181106104dc576104dc6107c6565b60209081029190910181015101526104f48186610888565b86600181518110610507576105076107c6565b602002602001015160200181815250505050505050949350505050565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610581573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105a591906107ab565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610609576040517f67102c4c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681179093556040519116919082907f28c1e0a919cd3b91d6d516417dedc06a2bd7954ad8960315347250b1aee2a4f1905f90a35050565b73ffffffffffffffffffffffffffffffffffffffff811681146106a0575f80fd5b50565b5f602082840312156106b3575f80fd5b81356106be8161067f565b9392505050565b5f805f80608085870312156106d8575f80fd5b5050823594602084013594506040840135936060013592509050565b602080825282518282018190525f919060409081850190868401855b8281101561074b578151805173ffffffffffffffffffffffffffffffffffffffff168552860151868501529284019290850190600101610710565b5091979650505050505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f602082840312156107bb575f80fd5b81516106be8161067f565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610833576108336107f3565b92915050565b8082028115828204841417610833576108336107f3565b5f82610883577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b81810381811115610833576108336107f356fea164736f6c6343000819000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`@Qa\tU8\x03\x80a\tU\x839\x81\x01`@\x81\x90R`+\x91`tV[_\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x92\x90\x93\x16\x91\x16\x17\x90U`\xA0V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`oW_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15`\x84W_\x80\xFD[`\x8B\x83`ZV[\x91P`\x97` \x84\x01`ZV[\x90P\x92P\x92\x90PV[a\x08\xA8\x80a\0\xAD_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x9FW_5`\xE0\x1C\x80c[ \x1D\x83\x11a\0rW\x80c{B\xD6\xC2\x11a\0XW\x80c{B\xD6\xC2\x14a\x01\x91W\x80c\x96\xD8B\xBE\x14a\x01\x9AW\x80c\x9Cf/\xDD\x14a\x01\xB9W_\x80\xFD[\x80c[ \x1D\x83\x14a\x01kW\x80cq-{\xB8\x14a\x01\x89W_\x80\xFD[\x80c)\xF3>\xC8\x14a\0\xA3W\x80cK\x01.\x9A\x14a\0\xEDW\x80cT\xE7\xF4-\x14a\x01\x02W\x80cT\xFDMP\x14a\x01\"W[_\x80\xFD[`\x01Ta\0\xC3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\0a\0\xFB6`\x04a\x06\xA3V[a\x01\xCCV[\0[a\x01\x15a\x01\x106`\x04a\x06\xC5V[a\x03%V[`@Qa\0\xE4\x91\x90a\x06\xF4V[a\x01^`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xE4\x91\x90a\x07XV[a\x01ta'\x10\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE4V[a\x01t`\xFA\x81V[a\x01ta\x05\xDC\x81V[_Ta\0\xC3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\0a\x01\xC76`\x04a\x06\xA3V[a\x05$V[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02)W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02M\x91\x90a\x07\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x02\xB1W`@Q\x7Fg\x10,L\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\xC7\xB7E\x9A\xD1\xAB1\x86%$\xBF\x0CJ^j\x85\x18\xC6\x0F'`z\\H:X\xEA'\xE3\x0C!L\x91\x90\xA3PPV[`@\x80Q`\x02\x80\x82R``\x82\x81\x01\x90\x93R\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03;WPP`@\x80Q\x80\x82\x01\x90\x91R_\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x82\x01\x81\x90R\x82Q\x92\x93P\x90\x91\x83\x91\x90a\x03\xA2Wa\x03\xA2a\x07\xC6V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81RP\x81`\x01\x81Q\x81\x10a\x04\x0CWa\x04\x0Ca\x07\xC6V[` \x90\x81\x02\x91\x90\x91\x01\x01R_\x82\x84a\x04$\x87\x89a\x08 V[a\x04.\x91\x90a\x08 V[a\x048\x91\x90a\x08 V[\x90P_a'\x10a\x04I`\xFA\x84a\x089V[a\x04S\x91\x90a\x08PV[\x90P\x80_\x03a\x04\x8EW`@Q\x7F\x01t\xE4&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04\x99\x85\x84a\x08\x88V[\x90P_a'\x10a\x04\xABa\x05\xDC\x84a\x089V[a\x04\xB5\x91\x90a\x08PV[\x90P_\x81\x84\x11a\x04\xC5W\x81a\x04\xC7V[\x83[\x90P\x80\x86_\x81Q\x81\x10a\x04\xDCWa\x04\xDCa\x07\xC6V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01Ra\x04\xF4\x81\x86a\x08\x88V[\x86`\x01\x81Q\x81\x10a\x05\x07Wa\x05\x07a\x07\xC6V[` \x02` \x01\x01Q` \x01\x81\x81RPPPPPPP\x94\x93PPPPV[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA5\x91\x90a\x07\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\tW`@Q\x7Fg\x10,L\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F(\xC1\xE0\xA9\x19\xCD;\x91\xD6\xD5\x16A}\xED\xC0j+\xD7\x95J\xD8\x96\x03\x154rP\xB1\xAE\xE2\xA4\xF1\x90_\x90\xA3PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xA0W_\x80\xFD[PV[_` \x82\x84\x03\x12\x15a\x06\xB3W_\x80\xFD[\x815a\x06\xBE\x81a\x06\x7FV[\x93\x92PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x06\xD8W_\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x07KW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x07\x10V[P\x91\x97\x96PPPPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x07\xBBW_\x80\xFD[\x81Qa\x06\xBE\x81a\x06\x7FV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x083Wa\x083a\x07\xF3V[\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x083Wa\x083a\x07\xF3V[_\x82a\x08\x83W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x083Wa\x083a\x07\xF3V\xFE\xA1dsolcC\0\x08\x19\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506004361061009f575f3560e01c80635b201d83116100725780637b42d6c2116100585780637b42d6c21461019157806396d842be1461019a5780639c662fdd146101b9575f80fd5b80635b201d831461016b578063712d7bb814610189575f80fd5b806329f33ec8146100a35780634b012e9a146100ed57806354e7f42d1461010257806354fd4d5014610122575b5f80fd5b6001546100c39073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b6101006100fb3660046106a3565b6101cc565b005b6101156101103660046106c5565b610325565b6040516100e491906106f4565b61015e6040518060400160405280600581526020017f312e302e3000000000000000000000000000000000000000000000000000000081525081565b6040516100e49190610758565b61017461271081565b60405163ffffffff90911681526020016100e4565b61017460fa81565b6101746105dc81565b5f546100c39073ffffffffffffffffffffffffffffffffffffffff1681565b6101006101c73660046106a3565b610524565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610229573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061024d91906107ab565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146102b1576040517f67102c4c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917fc7b7459ad1ab31862524bf0c4a5e6a8518c60f27607a5c483a58ea27e30c214c9190a35050565b6040805160028082526060828101909352816020015b604080518082019091525f808252602082015281526020019060019003908161033b575050604080518082019091525f805473ffffffffffffffffffffffffffffffffffffffff16825260208201819052825192935090918391906103a2576103a26107c6565b6020026020010181905250604051806040016040528060015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f8152508160018151811061040c5761040c6107c6565b60209081029190910101525f82846104248789610820565b61042e9190610820565b6104389190610820565b90505f61271061044960fa84610839565b6104539190610850565b9050805f0361048e576040517f0174e42600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6104998584610888565b90505f6127106104ab6105dc84610839565b6104b59190610850565b90505f8184116104c557816104c7565b835b905080865f815181106104dc576104dc6107c6565b60209081029190910181015101526104f48186610888565b86600181518110610507576105076107c6565b602002602001015160200181815250505050505050949350505050565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610581573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105a591906107ab565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610609576040517f67102c4c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681179093556040519116919082907f28c1e0a919cd3b91d6d516417dedc06a2bd7954ad8960315347250b1aee2a4f1905f90a35050565b73ffffffffffffffffffffffffffffffffffffffff811681146106a0575f80fd5b50565b5f602082840312156106b3575f80fd5b81356106be8161067f565b9392505050565b5f805f80608085870312156106d8575f80fd5b5050823594602084013594506040840135936060013592509050565b602080825282518282018190525f919060409081850190868401855b8281101561074b578151805173ffffffffffffffffffffffffffffffffffffffff168552860151868501529284019290850190600101610710565b5091979650505050505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f602082840312156107bb575f80fd5b81516106be8161067f565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610833576108336107f3565b92915050565b8082028115828204841417610833576108336107f3565b5f82610883577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b81810381811115610833576108336107f356fea164736f6c6343000819000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x9FW_5`\xE0\x1C\x80c[ \x1D\x83\x11a\0rW\x80c{B\xD6\xC2\x11a\0XW\x80c{B\xD6\xC2\x14a\x01\x91W\x80c\x96\xD8B\xBE\x14a\x01\x9AW\x80c\x9Cf/\xDD\x14a\x01\xB9W_\x80\xFD[\x80c[ \x1D\x83\x14a\x01kW\x80cq-{\xB8\x14a\x01\x89W_\x80\xFD[\x80c)\xF3>\xC8\x14a\0\xA3W\x80cK\x01.\x9A\x14a\0\xEDW\x80cT\xE7\xF4-\x14a\x01\x02W\x80cT\xFDMP\x14a\x01\"W[_\x80\xFD[`\x01Ta\0\xC3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\0a\0\xFB6`\x04a\x06\xA3V[a\x01\xCCV[\0[a\x01\x15a\x01\x106`\x04a\x06\xC5V[a\x03%V[`@Qa\0\xE4\x91\x90a\x06\xF4V[a\x01^`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xE4\x91\x90a\x07XV[a\x01ta'\x10\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE4V[a\x01t`\xFA\x81V[a\x01ta\x05\xDC\x81V[_Ta\0\xC3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\0a\x01\xC76`\x04a\x06\xA3V[a\x05$V[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02)W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02M\x91\x90a\x07\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x02\xB1W`@Q\x7Fg\x10,L\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\xC7\xB7E\x9A\xD1\xAB1\x86%$\xBF\x0CJ^j\x85\x18\xC6\x0F'`z\\H:X\xEA'\xE3\x0C!L\x91\x90\xA3PPV[`@\x80Q`\x02\x80\x82R``\x82\x81\x01\x90\x93R\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03;WPP`@\x80Q\x80\x82\x01\x90\x91R_\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x82\x01\x81\x90R\x82Q\x92\x93P\x90\x91\x83\x91\x90a\x03\xA2Wa\x03\xA2a\x07\xC6V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81RP\x81`\x01\x81Q\x81\x10a\x04\x0CWa\x04\x0Ca\x07\xC6V[` \x90\x81\x02\x91\x90\x91\x01\x01R_\x82\x84a\x04$\x87\x89a\x08 V[a\x04.\x91\x90a\x08 V[a\x048\x91\x90a\x08 V[\x90P_a'\x10a\x04I`\xFA\x84a\x089V[a\x04S\x91\x90a\x08PV[\x90P\x80_\x03a\x04\x8EW`@Q\x7F\x01t\xE4&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04\x99\x85\x84a\x08\x88V[\x90P_a'\x10a\x04\xABa\x05\xDC\x84a\x089V[a\x04\xB5\x91\x90a\x08PV[\x90P_\x81\x84\x11a\x04\xC5W\x81a\x04\xC7V[\x83[\x90P\x80\x86_\x81Q\x81\x10a\x04\xDCWa\x04\xDCa\x07\xC6V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01Ra\x04\xF4\x81\x86a\x08\x88V[\x86`\x01\x81Q\x81\x10a\x05\x07Wa\x05\x07a\x07\xC6V[` \x02` \x01\x01Q` \x01\x81\x81RPPPPPPP\x94\x93PPPPV[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA5\x91\x90a\x07\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\tW`@Q\x7Fg\x10,L\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F(\xC1\xE0\xA9\x19\xCD;\x91\xD6\xD5\x16A}\xED\xC0j+\xD7\x95J\xD8\x96\x03\x154rP\xB1\xAE\xE2\xA4\xF1\x90_\x90\xA3PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xA0W_\x80\xFD[PV[_` \x82\x84\x03\x12\x15a\x06\xB3W_\x80\xFD[\x815a\x06\xBE\x81a\x06\x7FV[\x93\x92PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x06\xD8W_\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x07KW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x07\x10V[P\x91\x97\x96PPPPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x07\xBBW_\x80\xFD[\x81Qa\x06\xBE\x81a\x06\x7FV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x083Wa\x083a\x07\xF3V[\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x083Wa\x083a\x07\xF3V[_\x82a\x08\x83W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x083Wa\x083a\x07\xF3V\xFE\xA1dsolcC\0\x08\x19\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `SharesCalculator_OnlyProxyAdminOwner()` and selector `0x67102c4c`.
```solidity
error SharesCalculator_OnlyProxyAdminOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SharesCalculator_OnlyProxyAdminOwner;
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
        impl ::core::convert::From<SharesCalculator_OnlyProxyAdminOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: SharesCalculator_OnlyProxyAdminOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SharesCalculator_OnlyProxyAdminOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SharesCalculator_OnlyProxyAdminOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SharesCalculator_OnlyProxyAdminOwner()";
            const SELECTOR: [u8; 4] = [103u8, 16u8, 44u8, 76u8];
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
    /**Custom error with signature `SharesCalculator_ZeroGrossShare()` and selector `0x0174e426`.
```solidity
error SharesCalculator_ZeroGrossShare();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SharesCalculator_ZeroGrossShare;
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
        impl ::core::convert::From<SharesCalculator_ZeroGrossShare>
        for UnderlyingRustTuple<'_> {
            fn from(value: SharesCalculator_ZeroGrossShare) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SharesCalculator_ZeroGrossShare {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SharesCalculator_ZeroGrossShare {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SharesCalculator_ZeroGrossShare()";
            const SELECTOR: [u8; 4] = [1u8, 116u8, 228u8, 38u8];
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
    /**Event with signature `RemainderRecipientUpdated(address,address)` and selector `0x28c1e0a919cd3b91d6d516417dedc06a2bd7954ad8960315347250b1aee2a4f1`.
```solidity
event RemainderRecipientUpdated(address indexed oldRemainderRecipient, address indexed newRemainderRecipient);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RemainderRecipientUpdated {
        #[allow(missing_docs)]
        pub oldRemainderRecipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRemainderRecipient: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RemainderRecipientUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RemainderRecipientUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8, 193u8, 224u8, 169u8, 25u8, 205u8, 59u8, 145u8, 214u8, 213u8, 22u8,
                65u8, 125u8, 237u8, 192u8, 106u8, 43u8, 215u8, 149u8, 74u8, 216u8, 150u8,
                3u8, 21u8, 52u8, 114u8, 80u8, 177u8, 174u8, 226u8, 164u8, 241u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldRemainderRecipient: topics.1,
                    newRemainderRecipient: topics.2,
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
                    self.oldRemainderRecipient.clone(),
                    self.newRemainderRecipient.clone(),
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
                    &self.oldRemainderRecipient,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newRemainderRecipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RemainderRecipientUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RemainderRecipientUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RemainderRecipientUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ShareRecipientUpdated(address,address)` and selector `0xc7b7459ad1ab31862524bf0c4a5e6a8518c60f27607a5c483a58ea27e30c214c`.
```solidity
event ShareRecipientUpdated(address indexed oldShareRecipient, address indexed newShareRecipient);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ShareRecipientUpdated {
        #[allow(missing_docs)]
        pub oldShareRecipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newShareRecipient: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ShareRecipientUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ShareRecipientUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8, 183u8, 69u8, 154u8, 209u8, 171u8, 49u8, 134u8, 37u8, 36u8, 191u8,
                12u8, 74u8, 94u8, 106u8, 133u8, 24u8, 198u8, 15u8, 39u8, 96u8, 122u8,
                92u8, 72u8, 58u8, 88u8, 234u8, 39u8, 227u8, 12u8, 33u8, 76u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldShareRecipient: topics.1,
                    newShareRecipient: topics.2,
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
                    self.oldShareRecipient.clone(),
                    self.newShareRecipient.clone(),
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
                    &self.oldShareRecipient,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newShareRecipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ShareRecipientUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ShareRecipientUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ShareRecipientUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _shareRecipient, address _remainderRecipient);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _shareRecipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _remainderRecipient: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._shareRecipient, value._remainderRecipient)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _shareRecipient: tuple.0,
                        _remainderRecipient: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._shareRecipient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._remainderRecipient,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `BASIS_POINT_SCALE()` and selector `0x5b201d83`.
```solidity
function BASIS_POINT_SCALE() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BASIS_POINT_SCALECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`BASIS_POINT_SCALE()`](BASIS_POINT_SCALECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BASIS_POINT_SCALEReturn {
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
            impl ::core::convert::From<BASIS_POINT_SCALECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: BASIS_POINT_SCALECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BASIS_POINT_SCALECall {
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
            impl ::core::convert::From<BASIS_POINT_SCALEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: BASIS_POINT_SCALEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BASIS_POINT_SCALEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BASIS_POINT_SCALECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BASIS_POINT_SCALE()";
            const SELECTOR: [u8; 4] = [91u8, 32u8, 29u8, 131u8];
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
                        let r: BASIS_POINT_SCALEReturn = r.into();
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
                        let r: BASIS_POINT_SCALEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `GROSS_SHARE_BPS()` and selector `0x712d7bb8`.
```solidity
function GROSS_SHARE_BPS() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GROSS_SHARE_BPSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`GROSS_SHARE_BPS()`](GROSS_SHARE_BPSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GROSS_SHARE_BPSReturn {
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
            impl ::core::convert::From<GROSS_SHARE_BPSCall> for UnderlyingRustTuple<'_> {
                fn from(value: GROSS_SHARE_BPSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GROSS_SHARE_BPSCall {
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
            impl ::core::convert::From<GROSS_SHARE_BPSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: GROSS_SHARE_BPSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for GROSS_SHARE_BPSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GROSS_SHARE_BPSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GROSS_SHARE_BPS()";
            const SELECTOR: [u8; 4] = [113u8, 45u8, 123u8, 184u8];
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
                        let r: GROSS_SHARE_BPSReturn = r.into();
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
                        let r: GROSS_SHARE_BPSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `NET_SHARE_BPS()` and selector `0x7b42d6c2`.
```solidity
function NET_SHARE_BPS() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NET_SHARE_BPSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`NET_SHARE_BPS()`](NET_SHARE_BPSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NET_SHARE_BPSReturn {
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
            impl ::core::convert::From<NET_SHARE_BPSCall> for UnderlyingRustTuple<'_> {
                fn from(value: NET_SHARE_BPSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NET_SHARE_BPSCall {
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
            impl ::core::convert::From<NET_SHARE_BPSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: NET_SHARE_BPSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NET_SHARE_BPSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for NET_SHARE_BPSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NET_SHARE_BPS()";
            const SELECTOR: [u8; 4] = [123u8, 66u8, 214u8, 194u8];
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
                        let r: NET_SHARE_BPSReturn = r.into();
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
                        let r: NET_SHARE_BPSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRecipientsAndAmounts(uint256,uint256,uint256,uint256)` and selector `0x54e7f42d`.
```solidity
function getRecipientsAndAmounts(uint256 _sequencerFeeRevenue, uint256 _baseFeeRevenue, uint256 _operatorFeeRevenue, uint256 _l1FeeRevenue) external view returns (ISharesCalculator.ShareInfo[] memory shareInfo_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRecipientsAndAmountsCall {
        #[allow(missing_docs)]
        pub _sequencerFeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _baseFeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _operatorFeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _l1FeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRecipientsAndAmounts(uint256,uint256,uint256,uint256)`](getRecipientsAndAmountsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRecipientsAndAmountsReturn {
        #[allow(missing_docs)]
        pub shareInfo_: alloy::sol_types::private::Vec<
            <ISharesCalculator::ShareInfo as alloy::sol_types::SolType>::RustType,
        >,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getRecipientsAndAmountsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRecipientsAndAmountsCall) -> Self {
                    (
                        value._sequencerFeeRevenue,
                        value._baseFeeRevenue,
                        value._operatorFeeRevenue,
                        value._l1FeeRevenue,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRecipientsAndAmountsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _sequencerFeeRevenue: tuple.0,
                        _baseFeeRevenue: tuple.1,
                        _operatorFeeRevenue: tuple.2,
                        _l1FeeRevenue: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ISharesCalculator::ShareInfo>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ISharesCalculator::ShareInfo as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<getRecipientsAndAmountsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRecipientsAndAmountsReturn) -> Self {
                    (value.shareInfo_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRecipientsAndAmountsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { shareInfo_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRecipientsAndAmountsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <ISharesCalculator::ShareInfo as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ISharesCalculator::ShareInfo>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRecipientsAndAmounts(uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [84u8, 231u8, 244u8, 45u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._sequencerFeeRevenue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._baseFeeRevenue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._operatorFeeRevenue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._l1FeeRevenue),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        ISharesCalculator::ShareInfo,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRecipientsAndAmountsReturn = r.into();
                        r.shareInfo_
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
                        let r: getRecipientsAndAmountsReturn = r.into();
                        r.shareInfo_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `remainderRecipient()` and selector `0x29f33ec8`.
```solidity
function remainderRecipient() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct remainderRecipientCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`remainderRecipient()`](remainderRecipientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct remainderRecipientReturn {
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
            impl ::core::convert::From<remainderRecipientCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: remainderRecipientCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for remainderRecipientCall {
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
            impl ::core::convert::From<remainderRecipientReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: remainderRecipientReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for remainderRecipientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for remainderRecipientCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "remainderRecipient()";
            const SELECTOR: [u8; 4] = [41u8, 243u8, 62u8, 200u8];
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
                        let r: remainderRecipientReturn = r.into();
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
                        let r: remainderRecipientReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setRemainderRecipient(address)` and selector `0x9c662fdd`.
```solidity
function setRemainderRecipient(address _newRemainderRecipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRemainderRecipientCall {
        #[allow(missing_docs)]
        pub _newRemainderRecipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRemainderRecipient(address)`](setRemainderRecipientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRemainderRecipientReturn {}
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
            impl ::core::convert::From<setRemainderRecipientCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRemainderRecipientCall) -> Self {
                    (value._newRemainderRecipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRemainderRecipientCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newRemainderRecipient: tuple.0,
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
            impl ::core::convert::From<setRemainderRecipientReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRemainderRecipientReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRemainderRecipientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setRemainderRecipientReturn {
            fn _tokenize(
                &self,
            ) -> <setRemainderRecipientCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRemainderRecipientCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRemainderRecipientReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRemainderRecipient(address)";
            const SELECTOR: [u8; 4] = [156u8, 102u8, 47u8, 221u8];
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
                        &self._newRemainderRecipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setRemainderRecipientReturn::_tokenize(ret)
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
    /**Function with signature `setShareRecipient(address)` and selector `0x4b012e9a`.
```solidity
function setShareRecipient(address _newShareRecipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setShareRecipientCall {
        #[allow(missing_docs)]
        pub _newShareRecipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setShareRecipient(address)`](setShareRecipientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setShareRecipientReturn {}
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
            impl ::core::convert::From<setShareRecipientCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setShareRecipientCall) -> Self {
                    (value._newShareRecipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setShareRecipientCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newShareRecipient: tuple.0,
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
            impl ::core::convert::From<setShareRecipientReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setShareRecipientReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setShareRecipientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setShareRecipientReturn {
            fn _tokenize(
                &self,
            ) -> <setShareRecipientCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setShareRecipientCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setShareRecipientReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setShareRecipient(address)";
            const SELECTOR: [u8; 4] = [75u8, 1u8, 46u8, 154u8];
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
                        &self._newShareRecipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setShareRecipientReturn::_tokenize(ret)
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
    /**Function with signature `shareRecipient()` and selector `0x96d842be`.
```solidity
function shareRecipient() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareRecipientCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`shareRecipient()`](shareRecipientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareRecipientReturn {
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
            impl ::core::convert::From<shareRecipientCall> for UnderlyingRustTuple<'_> {
                fn from(value: shareRecipientCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for shareRecipientCall {
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
            impl ::core::convert::From<shareRecipientReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: shareRecipientReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for shareRecipientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for shareRecipientCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "shareRecipient()";
            const SELECTOR: [u8; 4] = [150u8, 216u8, 66u8, 190u8];
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
                        let r: shareRecipientReturn = r.into();
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
                        let r: shareRecipientReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `version()` and selector `0x54fd4d50`.
```solidity
function version() external view returns (string memory);
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
    ///Container for all the [`SuperchainRevSharesCalculator`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SuperchainRevSharesCalculatorCalls {
        #[allow(missing_docs)]
        BASIS_POINT_SCALE(BASIS_POINT_SCALECall),
        #[allow(missing_docs)]
        GROSS_SHARE_BPS(GROSS_SHARE_BPSCall),
        #[allow(missing_docs)]
        NET_SHARE_BPS(NET_SHARE_BPSCall),
        #[allow(missing_docs)]
        getRecipientsAndAmounts(getRecipientsAndAmountsCall),
        #[allow(missing_docs)]
        remainderRecipient(remainderRecipientCall),
        #[allow(missing_docs)]
        setRemainderRecipient(setRemainderRecipientCall),
        #[allow(missing_docs)]
        setShareRecipient(setShareRecipientCall),
        #[allow(missing_docs)]
        shareRecipient(shareRecipientCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl SuperchainRevSharesCalculatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [41u8, 243u8, 62u8, 200u8],
            [75u8, 1u8, 46u8, 154u8],
            [84u8, 231u8, 244u8, 45u8],
            [84u8, 253u8, 77u8, 80u8],
            [91u8, 32u8, 29u8, 131u8],
            [113u8, 45u8, 123u8, 184u8],
            [123u8, 66u8, 214u8, 194u8],
            [150u8, 216u8, 66u8, 190u8],
            [156u8, 102u8, 47u8, 221u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(remainderRecipient),
            ::core::stringify!(setShareRecipient),
            ::core::stringify!(getRecipientsAndAmounts),
            ::core::stringify!(version),
            ::core::stringify!(BASIS_POINT_SCALE),
            ::core::stringify!(GROSS_SHARE_BPS),
            ::core::stringify!(NET_SHARE_BPS),
            ::core::stringify!(shareRecipient),
            ::core::stringify!(setRemainderRecipient),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <remainderRecipientCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setShareRecipientCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRecipientsAndAmountsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <BASIS_POINT_SCALECall as alloy_sol_types::SolCall>::SIGNATURE,
            <GROSS_SHARE_BPSCall as alloy_sol_types::SolCall>::SIGNATURE,
            <NET_SHARE_BPSCall as alloy_sol_types::SolCall>::SIGNATURE,
            <shareRecipientCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setRemainderRecipientCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SuperchainRevSharesCalculatorCalls {
        const NAME: &'static str = "SuperchainRevSharesCalculatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 9usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BASIS_POINT_SCALE(_) => {
                    <BASIS_POINT_SCALECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::GROSS_SHARE_BPS(_) => {
                    <GROSS_SHARE_BPSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::NET_SHARE_BPS(_) => {
                    <NET_SHARE_BPSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRecipientsAndAmounts(_) => {
                    <getRecipientsAndAmountsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::remainderRecipient(_) => {
                    <remainderRecipientCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRemainderRecipient(_) => {
                    <setRemainderRecipientCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setShareRecipient(_) => {
                    <setShareRecipientCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::shareRecipient(_) => {
                    <shareRecipientCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls>] = &[
                {
                    fn remainderRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <remainderRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::remainderRecipient)
                    }
                    remainderRecipient
                },
                {
                    fn setShareRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <setShareRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::setShareRecipient)
                    }
                    setShareRecipient
                },
                {
                    fn getRecipientsAndAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <getRecipientsAndAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorCalls::getRecipientsAndAmounts,
                            )
                    }
                    getRecipientsAndAmounts
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperchainRevSharesCalculatorCalls::version)
                    }
                    version
                },
                {
                    fn BASIS_POINT_SCALE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <BASIS_POINT_SCALECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::BASIS_POINT_SCALE)
                    }
                    BASIS_POINT_SCALE
                },
                {
                    fn GROSS_SHARE_BPS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <GROSS_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::GROSS_SHARE_BPS)
                    }
                    GROSS_SHARE_BPS
                },
                {
                    fn NET_SHARE_BPS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <NET_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::NET_SHARE_BPS)
                    }
                    NET_SHARE_BPS
                },
                {
                    fn shareRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <shareRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::shareRecipient)
                    }
                    shareRecipient
                },
                {
                    fn setRemainderRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <setRemainderRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorCalls::setRemainderRecipient,
                            )
                    }
                    setRemainderRecipient
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
            ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls>] = &[
                {
                    fn remainderRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <remainderRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::remainderRecipient)
                    }
                    remainderRecipient
                },
                {
                    fn setShareRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <setShareRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::setShareRecipient)
                    }
                    setShareRecipient
                },
                {
                    fn getRecipientsAndAmounts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <getRecipientsAndAmountsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorCalls::getRecipientsAndAmounts,
                            )
                    }
                    getRecipientsAndAmounts
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::version)
                    }
                    version
                },
                {
                    fn BASIS_POINT_SCALE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <BASIS_POINT_SCALECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::BASIS_POINT_SCALE)
                    }
                    BASIS_POINT_SCALE
                },
                {
                    fn GROSS_SHARE_BPS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <GROSS_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::GROSS_SHARE_BPS)
                    }
                    GROSS_SHARE_BPS
                },
                {
                    fn NET_SHARE_BPS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <NET_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::NET_SHARE_BPS)
                    }
                    NET_SHARE_BPS
                },
                {
                    fn shareRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <shareRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperchainRevSharesCalculatorCalls::shareRecipient)
                    }
                    shareRecipient
                },
                {
                    fn setRemainderRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorCalls> {
                        <setRemainderRecipientCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorCalls::setRemainderRecipient,
                            )
                    }
                    setRemainderRecipient
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
                Self::BASIS_POINT_SCALE(inner) => {
                    <BASIS_POINT_SCALECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GROSS_SHARE_BPS(inner) => {
                    <GROSS_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NET_SHARE_BPS(inner) => {
                    <NET_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRecipientsAndAmounts(inner) => {
                    <getRecipientsAndAmountsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::remainderRecipient(inner) => {
                    <remainderRecipientCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRemainderRecipient(inner) => {
                    <setRemainderRecipientCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setShareRecipient(inner) => {
                    <setShareRecipientCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::shareRecipient(inner) => {
                    <shareRecipientCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::BASIS_POINT_SCALE(inner) => {
                    <BASIS_POINT_SCALECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GROSS_SHARE_BPS(inner) => {
                    <GROSS_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NET_SHARE_BPS(inner) => {
                    <NET_SHARE_BPSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRecipientsAndAmounts(inner) => {
                    <getRecipientsAndAmountsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::remainderRecipient(inner) => {
                    <remainderRecipientCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRemainderRecipient(inner) => {
                    <setRemainderRecipientCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setShareRecipient(inner) => {
                    <setShareRecipientCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::shareRecipient(inner) => {
                    <shareRecipientCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`SuperchainRevSharesCalculator`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SuperchainRevSharesCalculatorErrors {
        #[allow(missing_docs)]
        SharesCalculator_OnlyProxyAdminOwner(SharesCalculator_OnlyProxyAdminOwner),
        #[allow(missing_docs)]
        SharesCalculator_ZeroGrossShare(SharesCalculator_ZeroGrossShare),
    }
    impl SuperchainRevSharesCalculatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 116u8, 228u8, 38u8],
            [103u8, 16u8, 44u8, 76u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(SharesCalculator_ZeroGrossShare),
            ::core::stringify!(SharesCalculator_OnlyProxyAdminOwner),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <SharesCalculator_ZeroGrossShare as alloy_sol_types::SolError>::SIGNATURE,
            <SharesCalculator_OnlyProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SuperchainRevSharesCalculatorErrors {
        const NAME: &'static str = "SuperchainRevSharesCalculatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 2usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SharesCalculator_OnlyProxyAdminOwner(_) => {
                    <SharesCalculator_OnlyProxyAdminOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SharesCalculator_ZeroGrossShare(_) => {
                    <SharesCalculator_ZeroGrossShare as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorErrors>] = &[
                {
                    fn SharesCalculator_ZeroGrossShare(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorErrors> {
                        <SharesCalculator_ZeroGrossShare as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorErrors::SharesCalculator_ZeroGrossShare,
                            )
                    }
                    SharesCalculator_ZeroGrossShare
                },
                {
                    fn SharesCalculator_OnlyProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorErrors> {
                        <SharesCalculator_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorErrors::SharesCalculator_OnlyProxyAdminOwner,
                            )
                    }
                    SharesCalculator_OnlyProxyAdminOwner
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
            ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorErrors>] = &[
                {
                    fn SharesCalculator_ZeroGrossShare(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorErrors> {
                        <SharesCalculator_ZeroGrossShare as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorErrors::SharesCalculator_ZeroGrossShare,
                            )
                    }
                    SharesCalculator_ZeroGrossShare
                },
                {
                    fn SharesCalculator_OnlyProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperchainRevSharesCalculatorErrors> {
                        <SharesCalculator_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperchainRevSharesCalculatorErrors::SharesCalculator_OnlyProxyAdminOwner,
                            )
                    }
                    SharesCalculator_OnlyProxyAdminOwner
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
                Self::SharesCalculator_OnlyProxyAdminOwner(inner) => {
                    <SharesCalculator_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SharesCalculator_ZeroGrossShare(inner) => {
                    <SharesCalculator_ZeroGrossShare as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::SharesCalculator_OnlyProxyAdminOwner(inner) => {
                    <SharesCalculator_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SharesCalculator_ZeroGrossShare(inner) => {
                    <SharesCalculator_ZeroGrossShare as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SuperchainRevSharesCalculator`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SuperchainRevSharesCalculatorEvents {
        #[allow(missing_docs)]
        RemainderRecipientUpdated(RemainderRecipientUpdated),
        #[allow(missing_docs)]
        ShareRecipientUpdated(ShareRecipientUpdated),
    }
    impl SuperchainRevSharesCalculatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                40u8, 193u8, 224u8, 169u8, 25u8, 205u8, 59u8, 145u8, 214u8, 213u8, 22u8,
                65u8, 125u8, 237u8, 192u8, 106u8, 43u8, 215u8, 149u8, 74u8, 216u8, 150u8,
                3u8, 21u8, 52u8, 114u8, 80u8, 177u8, 174u8, 226u8, 164u8, 241u8,
            ],
            [
                199u8, 183u8, 69u8, 154u8, 209u8, 171u8, 49u8, 134u8, 37u8, 36u8, 191u8,
                12u8, 74u8, 94u8, 106u8, 133u8, 24u8, 198u8, 15u8, 39u8, 96u8, 122u8,
                92u8, 72u8, 58u8, 88u8, 234u8, 39u8, 227u8, 12u8, 33u8, 76u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(RemainderRecipientUpdated),
            ::core::stringify!(ShareRecipientUpdated),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <RemainderRecipientUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ShareRecipientUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for SuperchainRevSharesCalculatorEvents {
        const NAME: &'static str = "SuperchainRevSharesCalculatorEvents";
        const COUNT: usize = 2usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <RemainderRecipientUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RemainderRecipientUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RemainderRecipientUpdated)
                }
                Some(
                    <ShareRecipientUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ShareRecipientUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ShareRecipientUpdated)
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
    impl alloy_sol_types::private::IntoLogData for SuperchainRevSharesCalculatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::RemainderRecipientUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ShareRecipientUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::RemainderRecipientUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ShareRecipientUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SuperchainRevSharesCalculator`](self) contract instance.

See the [wrapper's documentation](`SuperchainRevSharesCalculatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> SuperchainRevSharesCalculatorInstance<P, N> {
        SuperchainRevSharesCalculatorInstance::<P, N>::new(address, __provider)
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
        _shareRecipient: alloy::sol_types::private::Address,
        _remainderRecipient: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SuperchainRevSharesCalculatorInstance<P, N>>,
    > {
        SuperchainRevSharesCalculatorInstance::<
            P,
            N,
        >::deploy(__provider, _shareRecipient, _remainderRecipient)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        _shareRecipient: alloy::sol_types::private::Address,
        _remainderRecipient: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        SuperchainRevSharesCalculatorInstance::<
            P,
            N,
        >::deploy_builder(__provider, _shareRecipient, _remainderRecipient)
    }
    /**A [`SuperchainRevSharesCalculator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SuperchainRevSharesCalculator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SuperchainRevSharesCalculatorInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SuperchainRevSharesCalculatorInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SuperchainRevSharesCalculatorInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SuperchainRevSharesCalculatorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SuperchainRevSharesCalculator`](self) contract instance.

See the [wrapper's documentation](`SuperchainRevSharesCalculatorInstance`) for more details.*/
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
            _shareRecipient: alloy::sol_types::private::Address,
            _remainderRecipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SuperchainRevSharesCalculatorInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                _shareRecipient,
                _remainderRecipient,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            _shareRecipient: alloy::sol_types::private::Address,
            _remainderRecipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _shareRecipient,
                            _remainderRecipient,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
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
    impl<P: ::core::clone::Clone, N> SuperchainRevSharesCalculatorInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> SuperchainRevSharesCalculatorInstance<P, N> {
            SuperchainRevSharesCalculatorInstance {
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
    > SuperchainRevSharesCalculatorInstance<P, N> {
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
        ///Creates a new call builder for the [`BASIS_POINT_SCALE`] function.
        pub fn BASIS_POINT_SCALE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, BASIS_POINT_SCALECall, N> {
            self.call_builder(&BASIS_POINT_SCALECall)
        }
        ///Creates a new call builder for the [`GROSS_SHARE_BPS`] function.
        pub fn GROSS_SHARE_BPS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, GROSS_SHARE_BPSCall, N> {
            self.call_builder(&GROSS_SHARE_BPSCall)
        }
        ///Creates a new call builder for the [`NET_SHARE_BPS`] function.
        pub fn NET_SHARE_BPS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, NET_SHARE_BPSCall, N> {
            self.call_builder(&NET_SHARE_BPSCall)
        }
        ///Creates a new call builder for the [`getRecipientsAndAmounts`] function.
        pub fn getRecipientsAndAmounts(
            &self,
            _sequencerFeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
            _baseFeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
            _operatorFeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
            _l1FeeRevenue: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getRecipientsAndAmountsCall, N> {
            self.call_builder(
                &getRecipientsAndAmountsCall {
                    _sequencerFeeRevenue,
                    _baseFeeRevenue,
                    _operatorFeeRevenue,
                    _l1FeeRevenue,
                },
            )
        }
        ///Creates a new call builder for the [`remainderRecipient`] function.
        pub fn remainderRecipient(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, remainderRecipientCall, N> {
            self.call_builder(&remainderRecipientCall)
        }
        ///Creates a new call builder for the [`setRemainderRecipient`] function.
        pub fn setRemainderRecipient(
            &self,
            _newRemainderRecipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setRemainderRecipientCall, N> {
            self.call_builder(
                &setRemainderRecipientCall {
                    _newRemainderRecipient,
                },
            )
        }
        ///Creates a new call builder for the [`setShareRecipient`] function.
        pub fn setShareRecipient(
            &self,
            _newShareRecipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setShareRecipientCall, N> {
            self.call_builder(
                &setShareRecipientCall {
                    _newShareRecipient,
                },
            )
        }
        ///Creates a new call builder for the [`shareRecipient`] function.
        pub fn shareRecipient(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, shareRecipientCall, N> {
            self.call_builder(&shareRecipientCall)
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
    > SuperchainRevSharesCalculatorInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`RemainderRecipientUpdated`] event.
        pub fn RemainderRecipientUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, RemainderRecipientUpdated, N> {
            self.event_filter::<RemainderRecipientUpdated>()
        }
        ///Creates a new event filter for the [`ShareRecipientUpdated`] event.
        pub fn ShareRecipientUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ShareRecipientUpdated, N> {
            self.event_filter::<ShareRecipientUpdated>()
        }
    }
}

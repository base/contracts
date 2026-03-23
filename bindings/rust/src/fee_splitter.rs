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

interface FeeSplitter {
    error FeeSplitter_DisbursementIntervalNotReached();
    error FeeSplitter_ExceedsMaxFeeDisbursementTime();
    error FeeSplitter_FailedToSendToRevenueShareRecipient();
    error FeeSplitter_FeeDisbursementIntervalCannotBeZero();
    error FeeSplitter_FeeShareInfoEmpty();
    error FeeSplitter_FeeVaultMustWithdrawToFeeSplitter();
    error FeeSplitter_FeeVaultMustWithdrawToL2();
    error FeeSplitter_FeeVaultWithdrawalAmountMismatch();
    error FeeSplitter_NoFeesCollected();
    error FeeSplitter_OnlyProxyAdminOwner();
    error FeeSplitter_SenderNotCurrentVault();
    error FeeSplitter_SharesCalculatorCannotBeZero();
    error FeeSplitter_SharesCalculatorMalformedOutput();

    event FeeDisbursementIntervalUpdated(uint128 oldFeeDisbursementInterval, uint128 newFeeDisbursementInterval);
    event FeesDisbursed(ISharesCalculator.ShareInfo[] shareInfo, uint256 grossRevenue);
    event FeesReceived(address indexed sender, uint256 amount, uint256 newBalance);
    event Initialized(uint8 version);
    event SharesCalculatorUpdated(address oldSharesCalculator, address newSharesCalculator);

    constructor();

    receive() external payable;

    function MAX_DISBURSEMENT_INTERVAL() external view returns (uint128);
    function disburseFees() external;
    function feeDisbursementInterval() external view returns (uint128);
    function initialize(address _sharesCalculator) external;
    function lastDisbursementTime() external view returns (uint128);
    function setFeeDisbursementInterval(uint128 _newFeeDisbursementInterval) external;
    function setSharesCalculator(address _newSharesCalculator) external;
    function sharesCalculator() external view returns (address);
    function version() external view returns (string memory);
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
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "MAX_DISBURSEMENT_INTERVAL",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "disburseFees",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "feeDisbursementInterval",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_sharesCalculator",
        "type": "address",
        "internalType": "contract ISharesCalculator"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lastDisbursementTime",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setFeeDisbursementInterval",
    "inputs": [
      {
        "name": "_newFeeDisbursementInterval",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setSharesCalculator",
    "inputs": [
      {
        "name": "_newSharesCalculator",
        "type": "address",
        "internalType": "contract ISharesCalculator"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "sharesCalculator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISharesCalculator"
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
    "name": "FeeDisbursementIntervalUpdated",
    "inputs": [
      {
        "name": "oldFeeDisbursementInterval",
        "type": "uint128",
        "indexed": false,
        "internalType": "uint128"
      },
      {
        "name": "newFeeDisbursementInterval",
        "type": "uint128",
        "indexed": false,
        "internalType": "uint128"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FeesDisbursed",
    "inputs": [
      {
        "name": "shareInfo",
        "type": "tuple[]",
        "indexed": false,
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
      },
      {
        "name": "grossRevenue",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FeesReceived",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "SharesCalculatorUpdated",
    "inputs": [
      {
        "name": "oldSharesCalculator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newSharesCalculator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "FeeSplitter_DisbursementIntervalNotReached",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_ExceedsMaxFeeDisbursementTime",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_FailedToSendToRevenueShareRecipient",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_FeeDisbursementIntervalCannotBeZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_FeeShareInfoEmpty",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_FeeVaultMustWithdrawToFeeSplitter",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_FeeVaultMustWithdrawToL2",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_FeeVaultWithdrawalAmountMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_NoFeesCollected",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_OnlyProxyAdminOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_SenderNotCurrentVault",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_SharesCalculatorCannotBeZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeSplitter_SharesCalculatorMalformedOutput",
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
pub mod FeeSplitter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f80fd5b5060156019565b60d4565b5f54610100900460ff161560835760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff908116101560d2575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b611308806100e15f395ff3fe608060405260043610610096575f3560e01c80637dfbd04911610066578063b87ea8d41161004c578063b87ea8d414610284578063c4d66de814610298578063d61a398b146102b7575f80fd5b80637dfbd0491461024e5780637fc81bb714610265575f80fd5b80630a7617b31461014e5780630c0544a31461016f578063394d2731146101d157806354fd4d50146101f9575f80fd5b3661014a573373ffffffffffffffffffffffffffffffffffffffff7f21346dddac42cc163a6523eefc19df981df7352c870dc3b0b17a6a92fc6fe8135c161461010b576040517f14885cf900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805134815247602082018190529133917f213e72af0d3613bd643cff3059f872c1015e6541624e37872bf95eefbaf220a8910160405180910390a2005b5f80fd5b348015610159575f80fd5b5061016d610168366004610f49565b61030d565b005b34801561017a575f80fd5b506001546101ab9070010000000000000000000000000000000090046fffffffffffffffffffffffffffffffff1681565b6040516fffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b3480156101dc575f80fd5b506001546101ab906fffffffffffffffffffffffffffffffff1681565b348015610204575f80fd5b506102416040518060400160405280600581526020017f312e302e3000000000000000000000000000000000000000000000000000000081525081565b6040516101c89190610f64565b348015610259575f80fd5b506101ab6301e1338081565b348015610270575f80fd5b5061016d61027f366004610fb7565b6104cf565b34801561028f575f80fd5b5061016d6106c2565b3480156102a3575f80fd5b5061016d6102b2366004610f49565b610a9a565b3480156102c2575f80fd5b505f546102e89062010000900473ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020016101c8565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561036a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038e9190610fe6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146103f2576040517f38bac74200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811661043f576040517f99c6ec0800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805473ffffffffffffffffffffffffffffffffffffffff838116620100008181027fffffffffffffffffffff0000000000000000000000000000000000000000ffff85161790945560408051949093049091168084526020840191909152917f16417cc372deec0caee5f52e2ad77a5f07b4591fd56b4ff31b6e20f817d4daeb91015b60405180910390a15050565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561052c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105509190610fe6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146105b4576040517f38bac74200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b806fffffffffffffffffffffffffffffffff165f036105ff576040517fcf85916100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6301e133806fffffffffffffffffffffffffffffffff8216111561064f576040517f30b9f35e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600180546fffffffffffffffffffffffffffffffff8381167001000000000000000000000000000000008181028385161790945560408051949093049091168084526020840191909152917f4492086b630ed3846eec0979dd87a71c814ceb1c6dab80ab81e3450b21e4de2891016104c3565b6001546106f7906fffffffffffffffffffffffffffffffff70010000000000000000000000000000000082048116911661102e565b6fffffffffffffffffffffffffffffffff16421015610742576040517f1e4a9f3a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600180547fffffffffffffffffffffffffffffffff0000000000000000000000000000000016426fffffffffffffffffffffffffffffffff161790555f61079c734200000000000000000000000000000000000011610c8e565b90505f6107bc734200000000000000000000000000000000000019610c8e565b90505f6107dc73420000000000000000000000000000000000001a610c8e565b90505f6107fc73420000000000000000000000000000000000001b610c8e565b90506108075f610edb565b5f8282610814868861105e565b61081e919061105e565b610828919061105e565b9050805f03610863576040517fc8972e5200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80546040517f54e7f42d000000000000000000000000000000000000000000000000000000008152600481018890526024810187905260448101859052606481018690526201000090910473ffffffffffffffffffffffffffffffffffffffff16906354e7f42d906084015f60405180830381865afa1580156108e9573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261092e919081019061111c565b80519091505f81900361096d576040517f763970d600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805b82811015610a1d575f84828151811061098b5761098b6111eb565b6020026020010151602001519050805f036109a65750610a15565b5f6109cd8684815181106109bc576109bc6111eb565b60200260200101515f015183610f01565b905080610a06576040517fd68d1b1800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a10828561105e565b935050505b600101610970565b50838114610a57576040517f9c01eac000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f73f9a13241a1848ec157967f3a85601709353e616f1f2605d818c0f2d21774df8385604051610a88929190611218565b60405180910390a15050505050505050565b5f54610100900460ff1615808015610ab857505f54600160ff909116105b80610ad15750303b158015610ad157505f5460ff166001145b610b61576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a6564000000000000000000000000000000000000606482015260840160405180910390fd5b5f80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790558015610bbd575f80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff166101001790555b5f80547fffffffffffffffffffff0000000000000000000000000000000000000000ffff166201000073ffffffffffffffffffffffffffffffffffffffff85160217905572015180000000000000000000000000000000006fffffffffffffffffffffffffffffffff4216176001558015610c8a575f80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020016104c3565b5050565b5f60018273ffffffffffffffffffffffffffffffffffffffff166382356d8a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610cda573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cfe91906112b3565b6001811115610d0f57610d0f611286565b14610d46576040517fb4726cbe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff166366d003ac6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610da6573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dca9190610fe6565b73ffffffffffffffffffffffffffffffffffffffff1614610e17576040517fc3380cef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b47610e2183610edb565b8273ffffffffffffffffffffffffffffffffffffffff16633ccfd60b6040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610e6b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e8f91906112d1565b91504782610e9d83836112e8565b14610ed4576040517f87c91c5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050919050565b807f21346dddac42cc163a6523eefc19df981df7352c870dc3b0b17a6a92fc6fe8135d50565b5f610f0d835a84610f14565b9392505050565b5f805f805f858888f1949350505050565b73ffffffffffffffffffffffffffffffffffffffff81168114610f46575f80fd5b50565b5f60208284031215610f59575f80fd5b8135610f0d81610f25565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f60208284031215610fc7575f80fd5b81356fffffffffffffffffffffffffffffffff81168114610f0d575f80fd5b5f60208284031215610ff6575f80fd5b8151610f0d81610f25565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6fffffffffffffffffffffffffffffffff81811683821601908082111561105757611057611001565b5092915050565b8082018082111561107157611071611001565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040805190810167ffffffffffffffff811182821017156110c7576110c7611077565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff8111828210171561111457611114611077565b604052919050565b5f602080838503121561112d575f80fd5b825167ffffffffffffffff80821115611144575f80fd5b818501915085601f830112611157575f80fd5b81518181111561116957611169611077565b611177848260051b016110cd565b818152848101925060069190911b830184019087821115611196575f80fd5b928401925b818410156111e057604084890312156111b2575f80fd5b6111ba6110a4565b84516111c581610f25565b8152848601518682015283526040909301929184019161119b565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b604080825283518282018190525f91906020906060850190828801855b82811015611270578151805173ffffffffffffffffffffffffffffffffffffffff168552850151858501529285019290840190600101611235565b5050508093505050508260208301529392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f602082840312156112c3575f80fd5b815160028110610f0d575f80fd5b5f602082840312156112e1575f80fd5b5051919050565b818103818111156110715761107161100156fea164736f6c6343000819000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`\x15`\x19V[`\xD4V[_Ta\x01\0\x90\x04`\xFF\x16\x15`\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x10\x15`\xD2W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[a\x13\x08\x80a\0\xE1_9_\xF3\xFE`\x80`@R`\x046\x10a\0\x96W_5`\xE0\x1C\x80c}\xFB\xD0I\x11a\0fW\x80c\xB8~\xA8\xD4\x11a\0LW\x80c\xB8~\xA8\xD4\x14a\x02\x84W\x80c\xC4\xD6m\xE8\x14a\x02\x98W\x80c\xD6\x1A9\x8B\x14a\x02\xB7W_\x80\xFD[\x80c}\xFB\xD0I\x14a\x02NW\x80c\x7F\xC8\x1B\xB7\x14a\x02eW_\x80\xFD[\x80c\nv\x17\xB3\x14a\x01NW\x80c\x0C\x05D\xA3\x14a\x01oW\x80c9M'1\x14a\x01\xD1W\x80cT\xFDMP\x14a\x01\xF9W_\x80\xFD[6a\x01JW3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F!4m\xDD\xACB\xCC\x16:e#\xEE\xFC\x19\xDF\x98\x1D\xF75,\x87\r\xC3\xB0\xB1zj\x92\xFCo\xE8\x13\\\x16\x14a\x01\x0BW`@Q\x7F\x14\x88\\\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q4\x81RG` \x82\x01\x81\x90R\x913\x91\x7F!>r\xAF\r6\x13\xBDd<\xFF0Y\xF8r\xC1\x01^eAbN7\x87+\xF9^\xEF\xBA\xF2 \xA8\x91\x01`@Q\x80\x91\x03\x90\xA2\0[_\x80\xFD[4\x80\x15a\x01YW_\x80\xFD[Pa\x01ma\x01h6`\x04a\x0FIV[a\x03\rV[\0[4\x80\x15a\x01zW_\x80\xFD[P`\x01Ta\x01\xAB\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDCW_\x80\xFD[P`\x01Ta\x01\xAB\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\x04W_\x80\xFD[Pa\x02A`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xC8\x91\x90a\x0FdV[4\x80\x15a\x02YW_\x80\xFD[Pa\x01\xABc\x01\xE13\x80\x81V[4\x80\x15a\x02pW_\x80\xFD[Pa\x01ma\x02\x7F6`\x04a\x0F\xB7V[a\x04\xCFV[4\x80\x15a\x02\x8FW_\x80\xFD[Pa\x01ma\x06\xC2V[4\x80\x15a\x02\xA3W_\x80\xFD[Pa\x01ma\x02\xB26`\x04a\x0FIV[a\n\x9AV[4\x80\x15a\x02\xC2W_\x80\xFD[P_Ta\x02\xE8\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xC8V[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03jW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8E\x91\x90a\x0F\xE6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03\xF2W`@Q\x7F8\xBA\xC7B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x04?W`@Q\x7F\x99\xC6\xEC\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16b\x01\0\0\x81\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\x85\x16\x17\x90\x94U`@\x80Q\x94\x90\x93\x04\x90\x91\x16\x80\x84R` \x84\x01\x91\x90\x91R\x91\x7F\x16A|\xC3r\xDE\xEC\x0C\xAE\xE5\xF5.*\xD7z_\x07\xB4Y\x1F\xD5kO\xF3\x1Bn \xF8\x17\xD4\xDA\xEB\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05,W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05P\x91\x90a\x0F\xE6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05\xB4W`@Q\x7F8\xBA\xC7B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x05\xFFW`@Q\x7F\xCF\x85\x91a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\x01\xE13\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a\x06OW`@Q\x7F0\xB9\xF3^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x81\x02\x83\x85\x16\x17\x90\x94U`@\x80Q\x94\x90\x93\x04\x90\x91\x16\x80\x84R` \x84\x01\x91\x90\x91R\x91\x7FD\x92\x08kc\x0E\xD3\x84n\xEC\ty\xDD\x87\xA7\x1C\x81L\xEB\x1Cm\xAB\x80\xAB\x81\xE3E\x0B!\xE4\xDE(\x91\x01a\x04\xC3V[`\x01Ta\x06\xF7\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16a\x10.V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x10\x15a\x07BW`@Q\x7F\x1EJ\x9F:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90U_a\x07\x9CsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11a\x0C\x8EV[\x90P_a\x07\xBCsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x19a\x0C\x8EV[\x90P_a\x07\xDCsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1Aa\x0C\x8EV[\x90P_a\x07\xFCsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1Ba\x0C\x8EV[\x90Pa\x08\x07_a\x0E\xDBV[_\x82\x82a\x08\x14\x86\x88a\x10^V[a\x08\x1E\x91\x90a\x10^V[a\x08(\x91\x90a\x10^V[\x90P\x80_\x03a\x08cW`@Q\x7F\xC8\x97.R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`@Q\x7FT\xE7\xF4-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x87\x90R`D\x81\x01\x85\x90R`d\x81\x01\x86\x90Rb\x01\0\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cT\xE7\xF4-\x90`\x84\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE9W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\t.\x91\x90\x81\x01\x90a\x11\x1CV[\x80Q\x90\x91P_\x81\x90\x03a\tmW`@Q\x7Fv9p\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82\x81\x10\x15a\n\x1DW_\x84\x82\x81Q\x81\x10a\t\x8BWa\t\x8Ba\x11\xEBV[` \x02` \x01\x01Q` \x01Q\x90P\x80_\x03a\t\xA6WPa\n\x15V[_a\t\xCD\x86\x84\x81Q\x81\x10a\t\xBCWa\t\xBCa\x11\xEBV[` \x02` \x01\x01Q_\x01Q\x83a\x0F\x01V[\x90P\x80a\n\x06W`@Q\x7F\xD6\x8D\x1B\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x10\x82\x85a\x10^V[\x93PPP[`\x01\x01a\tpV[P\x83\x81\x14a\nWW`@Q\x7F\x9C\x01\xEA\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fs\xF9\xA12A\xA1\x84\x8E\xC1W\x96\x7F:\x85`\x17\t5>ao\x1F&\x05\xD8\x18\xC0\xF2\xD2\x17t\xDF\x83\x85`@Qa\n\x88\x92\x91\x90a\x12\x18V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\n\xB8WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\n\xD1WP0;\x15\x80\x15a\n\xD1WP_T`\xFF\x16`\x01\x14[a\x0BaW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x0B\xBDW_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\x16b\x01\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x02\x17\x90Ur\x01Q\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x17`\x01U\x80\x15a\x0C\x8AW_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x04\xC3V[PPV[_`\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x825m\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFE\x91\x90a\x12\xB3V[`\x01\x81\x11\x15a\r\x0FWa\r\x0Fa\x12\x86V[\x14a\rFW`@Q\x7F\xB4rl\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cf\xD0\x03\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCA\x91\x90a\x0F\xE6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0E\x17W`@Q\x7F\xC38\x0C\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Ga\x0E!\x83a\x0E\xDBV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\xCF\xD6\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0EkW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8F\x91\x90a\x12\xD1V[\x91PG\x82a\x0E\x9D\x83\x83a\x12\xE8V[\x14a\x0E\xD4W`@Q\x7F\x87\xC9\x1C\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x91\x90PV[\x80\x7F!4m\xDD\xACB\xCC\x16:e#\xEE\xFC\x19\xDF\x98\x1D\xF75,\x87\r\xC3\xB0\xB1zj\x92\xFCo\xE8\x13]PV[_a\x0F\r\x83Z\x84a\x0F\x14V[\x93\x92PPPV[_\x80_\x80_\x85\x88\x88\xF1\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0FFW_\x80\xFD[PV[_` \x82\x84\x03\x12\x15a\x0FYW_\x80\xFD[\x815a\x0F\r\x81a\x0F%V[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\xC7W_\x80\xFD[\x815o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\rW_\x80\xFD[_` \x82\x84\x03\x12\x15a\x0F\xF6W_\x80\xFD[\x81Qa\x0F\r\x81a\x0F%V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x10WWa\x10Wa\x10\x01V[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x10qWa\x10qa\x10\x01V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC7Wa\x10\xC7a\x10wV[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\x14Wa\x11\x14a\x10wV[`@R\x91\x90PV[_` \x80\x83\x85\x03\x12\x15a\x11-W_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11DW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x11WW_\x80\xFD[\x81Q\x81\x81\x11\x15a\x11iWa\x11ia\x10wV[a\x11w\x84\x82`\x05\x1B\x01a\x10\xCDV[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x11\x96W_\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x11\xE0W`@\x84\x89\x03\x12\x15a\x11\xB2W_\x80\xFD[a\x11\xBAa\x10\xA4V[\x84Qa\x11\xC5\x81a\x0F%V[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\x11\x9BV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@\x80\x82R\x83Q\x82\x82\x01\x81\x90R_\x91\x90` \x90``\x85\x01\x90\x82\x88\x01\x85[\x82\x81\x10\x15a\x12pW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x85\x01Q\x85\x85\x01R\x92\x85\x01\x92\x90\x84\x01\x90`\x01\x01a\x125V[PPP\x80\x93PPPP\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x12\xC3W_\x80\xFD[\x81Q`\x02\x81\x10a\x0F\rW_\x80\xFD[_` \x82\x84\x03\x12\x15a\x12\xE1W_\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x10qWa\x10qa\x10\x01V\xFE\xA1dsolcC\0\x08\x19\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610096575f3560e01c80637dfbd04911610066578063b87ea8d41161004c578063b87ea8d414610284578063c4d66de814610298578063d61a398b146102b7575f80fd5b80637dfbd0491461024e5780637fc81bb714610265575f80fd5b80630a7617b31461014e5780630c0544a31461016f578063394d2731146101d157806354fd4d50146101f9575f80fd5b3661014a573373ffffffffffffffffffffffffffffffffffffffff7f21346dddac42cc163a6523eefc19df981df7352c870dc3b0b17a6a92fc6fe8135c161461010b576040517f14885cf900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805134815247602082018190529133917f213e72af0d3613bd643cff3059f872c1015e6541624e37872bf95eefbaf220a8910160405180910390a2005b5f80fd5b348015610159575f80fd5b5061016d610168366004610f49565b61030d565b005b34801561017a575f80fd5b506001546101ab9070010000000000000000000000000000000090046fffffffffffffffffffffffffffffffff1681565b6040516fffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b3480156101dc575f80fd5b506001546101ab906fffffffffffffffffffffffffffffffff1681565b348015610204575f80fd5b506102416040518060400160405280600581526020017f312e302e3000000000000000000000000000000000000000000000000000000081525081565b6040516101c89190610f64565b348015610259575f80fd5b506101ab6301e1338081565b348015610270575f80fd5b5061016d61027f366004610fb7565b6104cf565b34801561028f575f80fd5b5061016d6106c2565b3480156102a3575f80fd5b5061016d6102b2366004610f49565b610a9a565b3480156102c2575f80fd5b505f546102e89062010000900473ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020016101c8565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561036a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038e9190610fe6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146103f2576040517f38bac74200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811661043f576040517f99c6ec0800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805473ffffffffffffffffffffffffffffffffffffffff838116620100008181027fffffffffffffffffffff0000000000000000000000000000000000000000ffff85161790945560408051949093049091168084526020840191909152917f16417cc372deec0caee5f52e2ad77a5f07b4591fd56b4ff31b6e20f817d4daeb91015b60405180910390a15050565b73420000000000000000000000000000000000001873ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561052c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105509190610fe6565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146105b4576040517f38bac74200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b806fffffffffffffffffffffffffffffffff165f036105ff576040517fcf85916100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6301e133806fffffffffffffffffffffffffffffffff8216111561064f576040517f30b9f35e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600180546fffffffffffffffffffffffffffffffff8381167001000000000000000000000000000000008181028385161790945560408051949093049091168084526020840191909152917f4492086b630ed3846eec0979dd87a71c814ceb1c6dab80ab81e3450b21e4de2891016104c3565b6001546106f7906fffffffffffffffffffffffffffffffff70010000000000000000000000000000000082048116911661102e565b6fffffffffffffffffffffffffffffffff16421015610742576040517f1e4a9f3a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600180547fffffffffffffffffffffffffffffffff0000000000000000000000000000000016426fffffffffffffffffffffffffffffffff161790555f61079c734200000000000000000000000000000000000011610c8e565b90505f6107bc734200000000000000000000000000000000000019610c8e565b90505f6107dc73420000000000000000000000000000000000001a610c8e565b90505f6107fc73420000000000000000000000000000000000001b610c8e565b90506108075f610edb565b5f8282610814868861105e565b61081e919061105e565b610828919061105e565b9050805f03610863576040517fc8972e5200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80546040517f54e7f42d000000000000000000000000000000000000000000000000000000008152600481018890526024810187905260448101859052606481018690526201000090910473ffffffffffffffffffffffffffffffffffffffff16906354e7f42d906084015f60405180830381865afa1580156108e9573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261092e919081019061111c565b80519091505f81900361096d576040517f763970d600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805b82811015610a1d575f84828151811061098b5761098b6111eb565b6020026020010151602001519050805f036109a65750610a15565b5f6109cd8684815181106109bc576109bc6111eb565b60200260200101515f015183610f01565b905080610a06576040517fd68d1b1800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a10828561105e565b935050505b600101610970565b50838114610a57576040517f9c01eac000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f73f9a13241a1848ec157967f3a85601709353e616f1f2605d818c0f2d21774df8385604051610a88929190611218565b60405180910390a15050505050505050565b5f54610100900460ff1615808015610ab857505f54600160ff909116105b80610ad15750303b158015610ad157505f5460ff166001145b610b61576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a6564000000000000000000000000000000000000606482015260840160405180910390fd5b5f80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790558015610bbd575f80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff166101001790555b5f80547fffffffffffffffffffff0000000000000000000000000000000000000000ffff166201000073ffffffffffffffffffffffffffffffffffffffff85160217905572015180000000000000000000000000000000006fffffffffffffffffffffffffffffffff4216176001558015610c8a575f80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020016104c3565b5050565b5f60018273ffffffffffffffffffffffffffffffffffffffff166382356d8a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610cda573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cfe91906112b3565b6001811115610d0f57610d0f611286565b14610d46576040517fb4726cbe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff166366d003ac6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610da6573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dca9190610fe6565b73ffffffffffffffffffffffffffffffffffffffff1614610e17576040517fc3380cef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b47610e2183610edb565b8273ffffffffffffffffffffffffffffffffffffffff16633ccfd60b6040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610e6b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e8f91906112d1565b91504782610e9d83836112e8565b14610ed4576040517f87c91c5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050919050565b807f21346dddac42cc163a6523eefc19df981df7352c870dc3b0b17a6a92fc6fe8135d50565b5f610f0d835a84610f14565b9392505050565b5f805f805f858888f1949350505050565b73ffffffffffffffffffffffffffffffffffffffff81168114610f46575f80fd5b50565b5f60208284031215610f59575f80fd5b8135610f0d81610f25565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f60208284031215610fc7575f80fd5b81356fffffffffffffffffffffffffffffffff81168114610f0d575f80fd5b5f60208284031215610ff6575f80fd5b8151610f0d81610f25565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6fffffffffffffffffffffffffffffffff81811683821601908082111561105757611057611001565b5092915050565b8082018082111561107157611071611001565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040805190810167ffffffffffffffff811182821017156110c7576110c7611077565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff8111828210171561111457611114611077565b604052919050565b5f602080838503121561112d575f80fd5b825167ffffffffffffffff80821115611144575f80fd5b818501915085601f830112611157575f80fd5b81518181111561116957611169611077565b611177848260051b016110cd565b818152848101925060069190911b830184019087821115611196575f80fd5b928401925b818410156111e057604084890312156111b2575f80fd5b6111ba6110a4565b84516111c581610f25565b8152848601518682015283526040909301929184019161119b565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b604080825283518282018190525f91906020906060850190828801855b82811015611270578151805173ffffffffffffffffffffffffffffffffffffffff168552850151858501529285019290840190600101611235565b5050508093505050508260208301529392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f602082840312156112c3575f80fd5b815160028110610f0d575f80fd5b5f602082840312156112e1575f80fd5b5051919050565b818103818111156110715761107161100156fea164736f6c6343000819000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x96W_5`\xE0\x1C\x80c}\xFB\xD0I\x11a\0fW\x80c\xB8~\xA8\xD4\x11a\0LW\x80c\xB8~\xA8\xD4\x14a\x02\x84W\x80c\xC4\xD6m\xE8\x14a\x02\x98W\x80c\xD6\x1A9\x8B\x14a\x02\xB7W_\x80\xFD[\x80c}\xFB\xD0I\x14a\x02NW\x80c\x7F\xC8\x1B\xB7\x14a\x02eW_\x80\xFD[\x80c\nv\x17\xB3\x14a\x01NW\x80c\x0C\x05D\xA3\x14a\x01oW\x80c9M'1\x14a\x01\xD1W\x80cT\xFDMP\x14a\x01\xF9W_\x80\xFD[6a\x01JW3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F!4m\xDD\xACB\xCC\x16:e#\xEE\xFC\x19\xDF\x98\x1D\xF75,\x87\r\xC3\xB0\xB1zj\x92\xFCo\xE8\x13\\\x16\x14a\x01\x0BW`@Q\x7F\x14\x88\\\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q4\x81RG` \x82\x01\x81\x90R\x913\x91\x7F!>r\xAF\r6\x13\xBDd<\xFF0Y\xF8r\xC1\x01^eAbN7\x87+\xF9^\xEF\xBA\xF2 \xA8\x91\x01`@Q\x80\x91\x03\x90\xA2\0[_\x80\xFD[4\x80\x15a\x01YW_\x80\xFD[Pa\x01ma\x01h6`\x04a\x0FIV[a\x03\rV[\0[4\x80\x15a\x01zW_\x80\xFD[P`\x01Ta\x01\xAB\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDCW_\x80\xFD[P`\x01Ta\x01\xAB\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\x04W_\x80\xFD[Pa\x02A`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xC8\x91\x90a\x0FdV[4\x80\x15a\x02YW_\x80\xFD[Pa\x01\xABc\x01\xE13\x80\x81V[4\x80\x15a\x02pW_\x80\xFD[Pa\x01ma\x02\x7F6`\x04a\x0F\xB7V[a\x04\xCFV[4\x80\x15a\x02\x8FW_\x80\xFD[Pa\x01ma\x06\xC2V[4\x80\x15a\x02\xA3W_\x80\xFD[Pa\x01ma\x02\xB26`\x04a\x0FIV[a\n\x9AV[4\x80\x15a\x02\xC2W_\x80\xFD[P_Ta\x02\xE8\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xC8V[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03jW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8E\x91\x90a\x0F\xE6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03\xF2W`@Q\x7F8\xBA\xC7B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x04?W`@Q\x7F\x99\xC6\xEC\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16b\x01\0\0\x81\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\x85\x16\x17\x90\x94U`@\x80Q\x94\x90\x93\x04\x90\x91\x16\x80\x84R` \x84\x01\x91\x90\x91R\x91\x7F\x16A|\xC3r\xDE\xEC\x0C\xAE\xE5\xF5.*\xD7z_\x07\xB4Y\x1F\xD5kO\xF3\x1Bn \xF8\x17\xD4\xDA\xEB\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05,W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05P\x91\x90a\x0F\xE6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05\xB4W`@Q\x7F8\xBA\xC7B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x05\xFFW`@Q\x7F\xCF\x85\x91a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\x01\xE13\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a\x06OW`@Q\x7F0\xB9\xF3^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x81\x02\x83\x85\x16\x17\x90\x94U`@\x80Q\x94\x90\x93\x04\x90\x91\x16\x80\x84R` \x84\x01\x91\x90\x91R\x91\x7FD\x92\x08kc\x0E\xD3\x84n\xEC\ty\xDD\x87\xA7\x1C\x81L\xEB\x1Cm\xAB\x80\xAB\x81\xE3E\x0B!\xE4\xDE(\x91\x01a\x04\xC3V[`\x01Ta\x06\xF7\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16a\x10.V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x10\x15a\x07BW`@Q\x7F\x1EJ\x9F:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90U_a\x07\x9CsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11a\x0C\x8EV[\x90P_a\x07\xBCsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x19a\x0C\x8EV[\x90P_a\x07\xDCsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1Aa\x0C\x8EV[\x90P_a\x07\xFCsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1Ba\x0C\x8EV[\x90Pa\x08\x07_a\x0E\xDBV[_\x82\x82a\x08\x14\x86\x88a\x10^V[a\x08\x1E\x91\x90a\x10^V[a\x08(\x91\x90a\x10^V[\x90P\x80_\x03a\x08cW`@Q\x7F\xC8\x97.R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`@Q\x7FT\xE7\xF4-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x81\x01\x87\x90R`D\x81\x01\x85\x90R`d\x81\x01\x86\x90Rb\x01\0\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cT\xE7\xF4-\x90`\x84\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE9W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\t.\x91\x90\x81\x01\x90a\x11\x1CV[\x80Q\x90\x91P_\x81\x90\x03a\tmW`@Q\x7Fv9p\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82\x81\x10\x15a\n\x1DW_\x84\x82\x81Q\x81\x10a\t\x8BWa\t\x8Ba\x11\xEBV[` \x02` \x01\x01Q` \x01Q\x90P\x80_\x03a\t\xA6WPa\n\x15V[_a\t\xCD\x86\x84\x81Q\x81\x10a\t\xBCWa\t\xBCa\x11\xEBV[` \x02` \x01\x01Q_\x01Q\x83a\x0F\x01V[\x90P\x80a\n\x06W`@Q\x7F\xD6\x8D\x1B\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x10\x82\x85a\x10^V[\x93PPP[`\x01\x01a\tpV[P\x83\x81\x14a\nWW`@Q\x7F\x9C\x01\xEA\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fs\xF9\xA12A\xA1\x84\x8E\xC1W\x96\x7F:\x85`\x17\t5>ao\x1F&\x05\xD8\x18\xC0\xF2\xD2\x17t\xDF\x83\x85`@Qa\n\x88\x92\x91\x90a\x12\x18V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\n\xB8WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\n\xD1WP0;\x15\x80\x15a\n\xD1WP_T`\xFF\x16`\x01\x14[a\x0BaW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x0B\xBDW_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\x16b\x01\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x02\x17\x90Ur\x01Q\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x17`\x01U\x80\x15a\x0C\x8AW_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x04\xC3V[PPV[_`\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x825m\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFE\x91\x90a\x12\xB3V[`\x01\x81\x11\x15a\r\x0FWa\r\x0Fa\x12\x86V[\x14a\rFW`@Q\x7F\xB4rl\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cf\xD0\x03\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCA\x91\x90a\x0F\xE6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0E\x17W`@Q\x7F\xC38\x0C\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Ga\x0E!\x83a\x0E\xDBV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\xCF\xD6\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0EkW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8F\x91\x90a\x12\xD1V[\x91PG\x82a\x0E\x9D\x83\x83a\x12\xE8V[\x14a\x0E\xD4W`@Q\x7F\x87\xC9\x1C\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x91\x90PV[\x80\x7F!4m\xDD\xACB\xCC\x16:e#\xEE\xFC\x19\xDF\x98\x1D\xF75,\x87\r\xC3\xB0\xB1zj\x92\xFCo\xE8\x13]PV[_a\x0F\r\x83Z\x84a\x0F\x14V[\x93\x92PPPV[_\x80_\x80_\x85\x88\x88\xF1\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0FFW_\x80\xFD[PV[_` \x82\x84\x03\x12\x15a\x0FYW_\x80\xFD[\x815a\x0F\r\x81a\x0F%V[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\xC7W_\x80\xFD[\x815o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\rW_\x80\xFD[_` \x82\x84\x03\x12\x15a\x0F\xF6W_\x80\xFD[\x81Qa\x0F\r\x81a\x0F%V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x10WWa\x10Wa\x10\x01V[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x10qWa\x10qa\x10\x01V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC7Wa\x10\xC7a\x10wV[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\x14Wa\x11\x14a\x10wV[`@R\x91\x90PV[_` \x80\x83\x85\x03\x12\x15a\x11-W_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11DW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x11WW_\x80\xFD[\x81Q\x81\x81\x11\x15a\x11iWa\x11ia\x10wV[a\x11w\x84\x82`\x05\x1B\x01a\x10\xCDV[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x11\x96W_\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x11\xE0W`@\x84\x89\x03\x12\x15a\x11\xB2W_\x80\xFD[a\x11\xBAa\x10\xA4V[\x84Qa\x11\xC5\x81a\x0F%V[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\x11\x9BV[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@\x80\x82R\x83Q\x82\x82\x01\x81\x90R_\x91\x90` \x90``\x85\x01\x90\x82\x88\x01\x85[\x82\x81\x10\x15a\x12pW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x85\x01Q\x85\x85\x01R\x92\x85\x01\x92\x90\x84\x01\x90`\x01\x01a\x125V[PPP\x80\x93PPPP\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x12\xC3W_\x80\xFD[\x81Q`\x02\x81\x10a\x0F\rW_\x80\xFD[_` \x82\x84\x03\x12\x15a\x12\xE1W_\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x10qWa\x10qa\x10\x01V\xFE\xA1dsolcC\0\x08\x19\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FeeSplitter_DisbursementIntervalNotReached()` and selector `0x1e4a9f3a`.
```solidity
error FeeSplitter_DisbursementIntervalNotReached();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_DisbursementIntervalNotReached;
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
        impl ::core::convert::From<FeeSplitter_DisbursementIntervalNotReached>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_DisbursementIntervalNotReached) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_DisbursementIntervalNotReached {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_DisbursementIntervalNotReached {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_DisbursementIntervalNotReached()";
            const SELECTOR: [u8; 4] = [30u8, 74u8, 159u8, 58u8];
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
    /**Custom error with signature `FeeSplitter_ExceedsMaxFeeDisbursementTime()` and selector `0x30b9f35e`.
```solidity
error FeeSplitter_ExceedsMaxFeeDisbursementTime();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_ExceedsMaxFeeDisbursementTime;
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
        impl ::core::convert::From<FeeSplitter_ExceedsMaxFeeDisbursementTime>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_ExceedsMaxFeeDisbursementTime) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_ExceedsMaxFeeDisbursementTime {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_ExceedsMaxFeeDisbursementTime {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_ExceedsMaxFeeDisbursementTime()";
            const SELECTOR: [u8; 4] = [48u8, 185u8, 243u8, 94u8];
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
    /**Custom error with signature `FeeSplitter_FailedToSendToRevenueShareRecipient()` and selector `0xd68d1b18`.
```solidity
error FeeSplitter_FailedToSendToRevenueShareRecipient();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_FailedToSendToRevenueShareRecipient;
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
        impl ::core::convert::From<FeeSplitter_FailedToSendToRevenueShareRecipient>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_FailedToSendToRevenueShareRecipient) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_FailedToSendToRevenueShareRecipient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError
        for FeeSplitter_FailedToSendToRevenueShareRecipient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_FailedToSendToRevenueShareRecipient()";
            const SELECTOR: [u8; 4] = [214u8, 141u8, 27u8, 24u8];
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
    /**Custom error with signature `FeeSplitter_FeeDisbursementIntervalCannotBeZero()` and selector `0xcf859161`.
```solidity
error FeeSplitter_FeeDisbursementIntervalCannotBeZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_FeeDisbursementIntervalCannotBeZero;
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
        impl ::core::convert::From<FeeSplitter_FeeDisbursementIntervalCannotBeZero>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_FeeDisbursementIntervalCannotBeZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_FeeDisbursementIntervalCannotBeZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError
        for FeeSplitter_FeeDisbursementIntervalCannotBeZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_FeeDisbursementIntervalCannotBeZero()";
            const SELECTOR: [u8; 4] = [207u8, 133u8, 145u8, 97u8];
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
    /**Custom error with signature `FeeSplitter_FeeShareInfoEmpty()` and selector `0x763970d6`.
```solidity
error FeeSplitter_FeeShareInfoEmpty();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_FeeShareInfoEmpty;
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
        impl ::core::convert::From<FeeSplitter_FeeShareInfoEmpty>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_FeeShareInfoEmpty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_FeeShareInfoEmpty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_FeeShareInfoEmpty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_FeeShareInfoEmpty()";
            const SELECTOR: [u8; 4] = [118u8, 57u8, 112u8, 214u8];
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
    /**Custom error with signature `FeeSplitter_FeeVaultMustWithdrawToFeeSplitter()` and selector `0xc3380cef`.
```solidity
error FeeSplitter_FeeVaultMustWithdrawToFeeSplitter();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_FeeVaultMustWithdrawToFeeSplitter;
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
        impl ::core::convert::From<FeeSplitter_FeeVaultMustWithdrawToFeeSplitter>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_FeeVaultMustWithdrawToFeeSplitter) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_FeeVaultMustWithdrawToFeeSplitter {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError
        for FeeSplitter_FeeVaultMustWithdrawToFeeSplitter {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_FeeVaultMustWithdrawToFeeSplitter()";
            const SELECTOR: [u8; 4] = [195u8, 56u8, 12u8, 239u8];
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
    /**Custom error with signature `FeeSplitter_FeeVaultMustWithdrawToL2()` and selector `0xb4726cbe`.
```solidity
error FeeSplitter_FeeVaultMustWithdrawToL2();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_FeeVaultMustWithdrawToL2;
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
        impl ::core::convert::From<FeeSplitter_FeeVaultMustWithdrawToL2>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_FeeVaultMustWithdrawToL2) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_FeeVaultMustWithdrawToL2 {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_FeeVaultMustWithdrawToL2 {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_FeeVaultMustWithdrawToL2()";
            const SELECTOR: [u8; 4] = [180u8, 114u8, 108u8, 190u8];
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
    /**Custom error with signature `FeeSplitter_FeeVaultWithdrawalAmountMismatch()` and selector `0x87c91c5c`.
```solidity
error FeeSplitter_FeeVaultWithdrawalAmountMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_FeeVaultWithdrawalAmountMismatch;
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
        impl ::core::convert::From<FeeSplitter_FeeVaultWithdrawalAmountMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_FeeVaultWithdrawalAmountMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_FeeVaultWithdrawalAmountMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_FeeVaultWithdrawalAmountMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_FeeVaultWithdrawalAmountMismatch()";
            const SELECTOR: [u8; 4] = [135u8, 201u8, 28u8, 92u8];
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
    /**Custom error with signature `FeeSplitter_NoFeesCollected()` and selector `0xc8972e52`.
```solidity
error FeeSplitter_NoFeesCollected();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_NoFeesCollected;
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
        impl ::core::convert::From<FeeSplitter_NoFeesCollected>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_NoFeesCollected) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_NoFeesCollected {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_NoFeesCollected {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_NoFeesCollected()";
            const SELECTOR: [u8; 4] = [200u8, 151u8, 46u8, 82u8];
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
    /**Custom error with signature `FeeSplitter_OnlyProxyAdminOwner()` and selector `0x38bac742`.
```solidity
error FeeSplitter_OnlyProxyAdminOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_OnlyProxyAdminOwner;
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
        impl ::core::convert::From<FeeSplitter_OnlyProxyAdminOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_OnlyProxyAdminOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_OnlyProxyAdminOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_OnlyProxyAdminOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_OnlyProxyAdminOwner()";
            const SELECTOR: [u8; 4] = [56u8, 186u8, 199u8, 66u8];
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
    /**Custom error with signature `FeeSplitter_SenderNotCurrentVault()` and selector `0x14885cf9`.
```solidity
error FeeSplitter_SenderNotCurrentVault();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_SenderNotCurrentVault;
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
        impl ::core::convert::From<FeeSplitter_SenderNotCurrentVault>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_SenderNotCurrentVault) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_SenderNotCurrentVault {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_SenderNotCurrentVault {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_SenderNotCurrentVault()";
            const SELECTOR: [u8; 4] = [20u8, 136u8, 92u8, 249u8];
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
    /**Custom error with signature `FeeSplitter_SharesCalculatorCannotBeZero()` and selector `0x99c6ec08`.
```solidity
error FeeSplitter_SharesCalculatorCannotBeZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_SharesCalculatorCannotBeZero;
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
        impl ::core::convert::From<FeeSplitter_SharesCalculatorCannotBeZero>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_SharesCalculatorCannotBeZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_SharesCalculatorCannotBeZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_SharesCalculatorCannotBeZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_SharesCalculatorCannotBeZero()";
            const SELECTOR: [u8; 4] = [153u8, 198u8, 236u8, 8u8];
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
    /**Custom error with signature `FeeSplitter_SharesCalculatorMalformedOutput()` and selector `0x9c01eac0`.
```solidity
error FeeSplitter_SharesCalculatorMalformedOutput();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeSplitter_SharesCalculatorMalformedOutput;
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
        impl ::core::convert::From<FeeSplitter_SharesCalculatorMalformedOutput>
        for UnderlyingRustTuple<'_> {
            fn from(value: FeeSplitter_SharesCalculatorMalformedOutput) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FeeSplitter_SharesCalculatorMalformedOutput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeSplitter_SharesCalculatorMalformedOutput {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeSplitter_SharesCalculatorMalformedOutput()";
            const SELECTOR: [u8; 4] = [156u8, 1u8, 234u8, 192u8];
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
    /**Event with signature `FeeDisbursementIntervalUpdated(uint128,uint128)` and selector `0x4492086b630ed3846eec0979dd87a71c814ceb1c6dab80ab81e3450b21e4de28`.
```solidity
event FeeDisbursementIntervalUpdated(uint128 oldFeeDisbursementInterval, uint128 newFeeDisbursementInterval);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FeeDisbursementIntervalUpdated {
        #[allow(missing_docs)]
        pub oldFeeDisbursementInterval: u128,
        #[allow(missing_docs)]
        pub newFeeDisbursementInterval: u128,
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
        impl alloy_sol_types::SolEvent for FeeDisbursementIntervalUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Uint<128>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "FeeDisbursementIntervalUpdated(uint128,uint128)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                68u8, 146u8, 8u8, 107u8, 99u8, 14u8, 211u8, 132u8, 110u8, 236u8, 9u8,
                121u8, 221u8, 135u8, 167u8, 28u8, 129u8, 76u8, 235u8, 28u8, 109u8, 171u8,
                128u8, 171u8, 129u8, 227u8, 69u8, 11u8, 33u8, 228u8, 222u8, 40u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldFeeDisbursementInterval: data.0,
                    newFeeDisbursementInterval: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.oldFeeDisbursementInterval,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newFeeDisbursementInterval,
                    ),
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
        impl alloy_sol_types::private::IntoLogData for FeeDisbursementIntervalUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FeeDisbursementIntervalUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &FeeDisbursementIntervalUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `FeesDisbursed((address,uint256)[],uint256)` and selector `0x73f9a13241a1848ec157967f3a85601709353e616f1f2605d818c0f2d21774df`.
```solidity
event FeesDisbursed(ISharesCalculator.ShareInfo[] shareInfo, uint256 grossRevenue);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FeesDisbursed {
        #[allow(missing_docs)]
        pub shareInfo: alloy::sol_types::private::Vec<
            <ISharesCalculator::ShareInfo as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub grossRevenue: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for FeesDisbursed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<ISharesCalculator::ShareInfo>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "FeesDisbursed((address,uint256)[],uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                115u8, 249u8, 161u8, 50u8, 65u8, 161u8, 132u8, 142u8, 193u8, 87u8, 150u8,
                127u8, 58u8, 133u8, 96u8, 23u8, 9u8, 53u8, 62u8, 97u8, 111u8, 31u8, 38u8,
                5u8, 216u8, 24u8, 192u8, 242u8, 210u8, 23u8, 116u8, 223u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    shareInfo: data.0,
                    grossRevenue: data.1,
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
                    <alloy::sol_types::sol_data::Array<
                        ISharesCalculator::ShareInfo,
                    > as alloy_sol_types::SolType>::tokenize(&self.shareInfo),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.grossRevenue),
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
        impl alloy_sol_types::private::IntoLogData for FeesDisbursed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FeesDisbursed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FeesDisbursed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `FeesReceived(address,uint256,uint256)` and selector `0x213e72af0d3613bd643cff3059f872c1015e6541624e37872bf95eefbaf220a8`.
```solidity
event FeesReceived(address indexed sender, uint256 amount, uint256 newBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FeesReceived {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newBalance: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for FeesReceived {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "FeesReceived(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                33u8, 62u8, 114u8, 175u8, 13u8, 54u8, 19u8, 189u8, 100u8, 60u8, 255u8,
                48u8, 89u8, 248u8, 114u8, 193u8, 1u8, 94u8, 101u8, 65u8, 98u8, 78u8,
                55u8, 135u8, 43u8, 249u8, 94u8, 239u8, 186u8, 242u8, 32u8, 168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: topics.1,
                    amount: data.0,
                    newBalance: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
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
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FeesReceived {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FeesReceived> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FeesReceived) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `SharesCalculatorUpdated(address,address)` and selector `0x16417cc372deec0caee5f52e2ad77a5f07b4591fd56b4ff31b6e20f817d4daeb`.
```solidity
event SharesCalculatorUpdated(address oldSharesCalculator, address newSharesCalculator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SharesCalculatorUpdated {
        #[allow(missing_docs)]
        pub oldSharesCalculator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newSharesCalculator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SharesCalculatorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SharesCalculatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                22u8, 65u8, 124u8, 195u8, 114u8, 222u8, 236u8, 12u8, 174u8, 229u8, 245u8,
                46u8, 42u8, 215u8, 122u8, 95u8, 7u8, 180u8, 89u8, 31u8, 213u8, 107u8,
                79u8, 243u8, 27u8, 110u8, 32u8, 248u8, 23u8, 212u8, 218u8, 235u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldSharesCalculator: data.0,
                    newSharesCalculator: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldSharesCalculator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newSharesCalculator,
                    ),
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
        impl alloy_sol_types::private::IntoLogData for SharesCalculatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SharesCalculatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SharesCalculatorUpdated,
            ) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `MAX_DISBURSEMENT_INTERVAL()` and selector `0x7dfbd049`.
```solidity
function MAX_DISBURSEMENT_INTERVAL() external view returns (uint128);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_DISBURSEMENT_INTERVALCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_DISBURSEMENT_INTERVAL()`](MAX_DISBURSEMENT_INTERVALCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_DISBURSEMENT_INTERVALReturn {
        #[allow(missing_docs)]
        pub _0: u128,
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
            impl ::core::convert::From<MAX_DISBURSEMENT_INTERVALCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_DISBURSEMENT_INTERVALCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_DISBURSEMENT_INTERVALCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<MAX_DISBURSEMENT_INTERVALReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_DISBURSEMENT_INTERVALReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_DISBURSEMENT_INTERVALReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_DISBURSEMENT_INTERVALCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u128;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_DISBURSEMENT_INTERVAL()";
            const SELECTOR: [u8; 4] = [125u8, 251u8, 208u8, 73u8];
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
                        128,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MAX_DISBURSEMENT_INTERVALReturn = r.into();
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
                        let r: MAX_DISBURSEMENT_INTERVALReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disburseFees()` and selector `0xb87ea8d4`.
```solidity
function disburseFees() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disburseFeesCall;
    ///Container type for the return parameters of the [`disburseFees()`](disburseFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disburseFeesReturn {}
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
            impl ::core::convert::From<disburseFeesCall> for UnderlyingRustTuple<'_> {
                fn from(value: disburseFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disburseFeesCall {
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
            impl ::core::convert::From<disburseFeesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disburseFeesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disburseFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl disburseFeesReturn {
            fn _tokenize(
                &self,
            ) -> <disburseFeesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disburseFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disburseFeesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disburseFees()";
            const SELECTOR: [u8; 4] = [184u8, 126u8, 168u8, 212u8];
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
                disburseFeesReturn::_tokenize(ret)
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
    /**Function with signature `feeDisbursementInterval()` and selector `0x0c0544a3`.
```solidity
function feeDisbursementInterval() external view returns (uint128);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct feeDisbursementIntervalCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`feeDisbursementInterval()`](feeDisbursementIntervalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct feeDisbursementIntervalReturn {
        #[allow(missing_docs)]
        pub _0: u128,
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
            impl ::core::convert::From<feeDisbursementIntervalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: feeDisbursementIntervalCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for feeDisbursementIntervalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<feeDisbursementIntervalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: feeDisbursementIntervalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for feeDisbursementIntervalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for feeDisbursementIntervalCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u128;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "feeDisbursementInterval()";
            const SELECTOR: [u8; 4] = [12u8, 5u8, 68u8, 163u8];
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
                        128,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: feeDisbursementIntervalReturn = r.into();
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
                        let r: feeDisbursementIntervalReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
```solidity
function initialize(address _sharesCalculator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _sharesCalculator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._sharesCalculator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _sharesCalculator: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
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
                        &self._sharesCalculator,
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
    /**Function with signature `lastDisbursementTime()` and selector `0x394d2731`.
```solidity
function lastDisbursementTime() external view returns (uint128);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastDisbursementTimeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastDisbursementTime()`](lastDisbursementTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastDisbursementTimeReturn {
        #[allow(missing_docs)]
        pub _0: u128,
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
            impl ::core::convert::From<lastDisbursementTimeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastDisbursementTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastDisbursementTimeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<lastDisbursementTimeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastDisbursementTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastDisbursementTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastDisbursementTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u128;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastDisbursementTime()";
            const SELECTOR: [u8; 4] = [57u8, 77u8, 39u8, 49u8];
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
                        128,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lastDisbursementTimeReturn = r.into();
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
                        let r: lastDisbursementTimeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setFeeDisbursementInterval(uint128)` and selector `0x7fc81bb7`.
```solidity
function setFeeDisbursementInterval(uint128 _newFeeDisbursementInterval) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFeeDisbursementIntervalCall {
        #[allow(missing_docs)]
        pub _newFeeDisbursementInterval: u128,
    }
    ///Container type for the return parameters of the [`setFeeDisbursementInterval(uint128)`](setFeeDisbursementIntervalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFeeDisbursementIntervalReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<setFeeDisbursementIntervalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setFeeDisbursementIntervalCall) -> Self {
                    (value._newFeeDisbursementInterval,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setFeeDisbursementIntervalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newFeeDisbursementInterval: tuple.0,
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
            impl ::core::convert::From<setFeeDisbursementIntervalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setFeeDisbursementIntervalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setFeeDisbursementIntervalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setFeeDisbursementIntervalReturn {
            fn _tokenize(
                &self,
            ) -> <setFeeDisbursementIntervalCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setFeeDisbursementIntervalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setFeeDisbursementIntervalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setFeeDisbursementInterval(uint128)";
            const SELECTOR: [u8; 4] = [127u8, 200u8, 27u8, 183u8];
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
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._newFeeDisbursementInterval,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setFeeDisbursementIntervalReturn::_tokenize(ret)
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
    /**Function with signature `setSharesCalculator(address)` and selector `0x0a7617b3`.
```solidity
function setSharesCalculator(address _newSharesCalculator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSharesCalculatorCall {
        #[allow(missing_docs)]
        pub _newSharesCalculator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setSharesCalculator(address)`](setSharesCalculatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSharesCalculatorReturn {}
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
            impl ::core::convert::From<setSharesCalculatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSharesCalculatorCall) -> Self {
                    (value._newSharesCalculator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSharesCalculatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newSharesCalculator: tuple.0,
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
            impl ::core::convert::From<setSharesCalculatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSharesCalculatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSharesCalculatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setSharesCalculatorReturn {
            fn _tokenize(
                &self,
            ) -> <setSharesCalculatorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSharesCalculatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSharesCalculatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSharesCalculator(address)";
            const SELECTOR: [u8; 4] = [10u8, 118u8, 23u8, 179u8];
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
                        &self._newSharesCalculator,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setSharesCalculatorReturn::_tokenize(ret)
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
    /**Function with signature `sharesCalculator()` and selector `0xd61a398b`.
```solidity
function sharesCalculator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sharesCalculatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`sharesCalculator()`](sharesCalculatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sharesCalculatorReturn {
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
            impl ::core::convert::From<sharesCalculatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: sharesCalculatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sharesCalculatorCall {
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
            impl ::core::convert::From<sharesCalculatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sharesCalculatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sharesCalculatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sharesCalculatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sharesCalculator()";
            const SELECTOR: [u8; 4] = [214u8, 26u8, 57u8, 139u8];
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
                        let r: sharesCalculatorReturn = r.into();
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
                        let r: sharesCalculatorReturn = r.into();
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
    ///Container for all the [`FeeSplitter`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum FeeSplitterCalls {
        #[allow(missing_docs)]
        MAX_DISBURSEMENT_INTERVAL(MAX_DISBURSEMENT_INTERVALCall),
        #[allow(missing_docs)]
        disburseFees(disburseFeesCall),
        #[allow(missing_docs)]
        feeDisbursementInterval(feeDisbursementIntervalCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        lastDisbursementTime(lastDisbursementTimeCall),
        #[allow(missing_docs)]
        setFeeDisbursementInterval(setFeeDisbursementIntervalCall),
        #[allow(missing_docs)]
        setSharesCalculator(setSharesCalculatorCall),
        #[allow(missing_docs)]
        sharesCalculator(sharesCalculatorCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl FeeSplitterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 118u8, 23u8, 179u8],
            [12u8, 5u8, 68u8, 163u8],
            [57u8, 77u8, 39u8, 49u8],
            [84u8, 253u8, 77u8, 80u8],
            [125u8, 251u8, 208u8, 73u8],
            [127u8, 200u8, 27u8, 183u8],
            [184u8, 126u8, 168u8, 212u8],
            [196u8, 214u8, 109u8, 232u8],
            [214u8, 26u8, 57u8, 139u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(setSharesCalculator),
            ::core::stringify!(feeDisbursementInterval),
            ::core::stringify!(lastDisbursementTime),
            ::core::stringify!(version),
            ::core::stringify!(MAX_DISBURSEMENT_INTERVAL),
            ::core::stringify!(setFeeDisbursementInterval),
            ::core::stringify!(disburseFees),
            ::core::stringify!(initialize),
            ::core::stringify!(sharesCalculator),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <setSharesCalculatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <feeDisbursementIntervalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lastDisbursementTimeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_DISBURSEMENT_INTERVALCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setFeeDisbursementIntervalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disburseFeesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <sharesCalculatorCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for FeeSplitterCalls {
        const NAME: &'static str = "FeeSplitterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 9usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MAX_DISBURSEMENT_INTERVAL(_) => {
                    <MAX_DISBURSEMENT_INTERVALCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disburseFees(_) => {
                    <disburseFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::feeDisbursementInterval(_) => {
                    <feeDisbursementIntervalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastDisbursementTime(_) => {
                    <lastDisbursementTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setFeeDisbursementInterval(_) => {
                    <setFeeDisbursementIntervalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSharesCalculator(_) => {
                    <setSharesCalculatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sharesCalculator(_) => {
                    <sharesCalculatorCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<FeeSplitterCalls>] = &[
                {
                    fn setSharesCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <setSharesCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::setSharesCalculator)
                    }
                    setSharesCalculator
                },
                {
                    fn feeDisbursementInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <feeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::feeDisbursementInterval)
                    }
                    feeDisbursementInterval
                },
                {
                    fn lastDisbursementTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <lastDisbursementTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::lastDisbursementTime)
                    }
                    lastDisbursementTime
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FeeSplitterCalls::version)
                    }
                    version
                },
                {
                    fn MAX_DISBURSEMENT_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <MAX_DISBURSEMENT_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::MAX_DISBURSEMENT_INTERVAL)
                    }
                    MAX_DISBURSEMENT_INTERVAL
                },
                {
                    fn setFeeDisbursementInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <setFeeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::setFeeDisbursementInterval)
                    }
                    setFeeDisbursementInterval
                },
                {
                    fn disburseFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <disburseFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::disburseFees)
                    }
                    disburseFees
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn sharesCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <sharesCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterCalls::sharesCalculator)
                    }
                    sharesCalculator
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
            ) -> alloy_sol_types::Result<FeeSplitterCalls>] = &[
                {
                    fn setSharesCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <setSharesCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::setSharesCalculator)
                    }
                    setSharesCalculator
                },
                {
                    fn feeDisbursementInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <feeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::feeDisbursementInterval)
                    }
                    feeDisbursementInterval
                },
                {
                    fn lastDisbursementTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <lastDisbursementTimeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::lastDisbursementTime)
                    }
                    lastDisbursementTime
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::version)
                    }
                    version
                },
                {
                    fn MAX_DISBURSEMENT_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <MAX_DISBURSEMENT_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::MAX_DISBURSEMENT_INTERVAL)
                    }
                    MAX_DISBURSEMENT_INTERVAL
                },
                {
                    fn setFeeDisbursementInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <setFeeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::setFeeDisbursementInterval)
                    }
                    setFeeDisbursementInterval
                },
                {
                    fn disburseFees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <disburseFeesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::disburseFees)
                    }
                    disburseFees
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn sharesCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterCalls> {
                        <sharesCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterCalls::sharesCalculator)
                    }
                    sharesCalculator
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
                Self::MAX_DISBURSEMENT_INTERVAL(inner) => {
                    <MAX_DISBURSEMENT_INTERVALCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disburseFees(inner) => {
                    <disburseFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::feeDisbursementInterval(inner) => {
                    <feeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastDisbursementTime(inner) => {
                    <lastDisbursementTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setFeeDisbursementInterval(inner) => {
                    <setFeeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setSharesCalculator(inner) => {
                    <setSharesCalculatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::sharesCalculator(inner) => {
                    <sharesCalculatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::MAX_DISBURSEMENT_INTERVAL(inner) => {
                    <MAX_DISBURSEMENT_INTERVALCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disburseFees(inner) => {
                    <disburseFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::feeDisbursementInterval(inner) => {
                    <feeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::lastDisbursementTime(inner) => {
                    <lastDisbursementTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setFeeDisbursementInterval(inner) => {
                    <setFeeDisbursementIntervalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setSharesCalculator(inner) => {
                    <setSharesCalculatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sharesCalculator(inner) => {
                    <sharesCalculatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`FeeSplitter`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum FeeSplitterErrors {
        #[allow(missing_docs)]
        FeeSplitter_DisbursementIntervalNotReached(
            FeeSplitter_DisbursementIntervalNotReached,
        ),
        #[allow(missing_docs)]
        FeeSplitter_ExceedsMaxFeeDisbursementTime(
            FeeSplitter_ExceedsMaxFeeDisbursementTime,
        ),
        #[allow(missing_docs)]
        FeeSplitter_FailedToSendToRevenueShareRecipient(
            FeeSplitter_FailedToSendToRevenueShareRecipient,
        ),
        #[allow(missing_docs)]
        FeeSplitter_FeeDisbursementIntervalCannotBeZero(
            FeeSplitter_FeeDisbursementIntervalCannotBeZero,
        ),
        #[allow(missing_docs)]
        FeeSplitter_FeeShareInfoEmpty(FeeSplitter_FeeShareInfoEmpty),
        #[allow(missing_docs)]
        FeeSplitter_FeeVaultMustWithdrawToFeeSplitter(
            FeeSplitter_FeeVaultMustWithdrawToFeeSplitter,
        ),
        #[allow(missing_docs)]
        FeeSplitter_FeeVaultMustWithdrawToL2(FeeSplitter_FeeVaultMustWithdrawToL2),
        #[allow(missing_docs)]
        FeeSplitter_FeeVaultWithdrawalAmountMismatch(
            FeeSplitter_FeeVaultWithdrawalAmountMismatch,
        ),
        #[allow(missing_docs)]
        FeeSplitter_NoFeesCollected(FeeSplitter_NoFeesCollected),
        #[allow(missing_docs)]
        FeeSplitter_OnlyProxyAdminOwner(FeeSplitter_OnlyProxyAdminOwner),
        #[allow(missing_docs)]
        FeeSplitter_SenderNotCurrentVault(FeeSplitter_SenderNotCurrentVault),
        #[allow(missing_docs)]
        FeeSplitter_SharesCalculatorCannotBeZero(
            FeeSplitter_SharesCalculatorCannotBeZero,
        ),
        #[allow(missing_docs)]
        FeeSplitter_SharesCalculatorMalformedOutput(
            FeeSplitter_SharesCalculatorMalformedOutput,
        ),
    }
    impl FeeSplitterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [20u8, 136u8, 92u8, 249u8],
            [30u8, 74u8, 159u8, 58u8],
            [48u8, 185u8, 243u8, 94u8],
            [56u8, 186u8, 199u8, 66u8],
            [118u8, 57u8, 112u8, 214u8],
            [135u8, 201u8, 28u8, 92u8],
            [153u8, 198u8, 236u8, 8u8],
            [156u8, 1u8, 234u8, 192u8],
            [180u8, 114u8, 108u8, 190u8],
            [195u8, 56u8, 12u8, 239u8],
            [200u8, 151u8, 46u8, 82u8],
            [207u8, 133u8, 145u8, 97u8],
            [214u8, 141u8, 27u8, 24u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(FeeSplitter_SenderNotCurrentVault),
            ::core::stringify!(FeeSplitter_DisbursementIntervalNotReached),
            ::core::stringify!(FeeSplitter_ExceedsMaxFeeDisbursementTime),
            ::core::stringify!(FeeSplitter_OnlyProxyAdminOwner),
            ::core::stringify!(FeeSplitter_FeeShareInfoEmpty),
            ::core::stringify!(FeeSplitter_FeeVaultWithdrawalAmountMismatch),
            ::core::stringify!(FeeSplitter_SharesCalculatorCannotBeZero),
            ::core::stringify!(FeeSplitter_SharesCalculatorMalformedOutput),
            ::core::stringify!(FeeSplitter_FeeVaultMustWithdrawToL2),
            ::core::stringify!(FeeSplitter_FeeVaultMustWithdrawToFeeSplitter),
            ::core::stringify!(FeeSplitter_NoFeesCollected),
            ::core::stringify!(FeeSplitter_FeeDisbursementIntervalCannotBeZero),
            ::core::stringify!(FeeSplitter_FailedToSendToRevenueShareRecipient),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <FeeSplitter_SenderNotCurrentVault as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_DisbursementIntervalNotReached as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_ExceedsMaxFeeDisbursementTime as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_OnlyProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_FeeShareInfoEmpty as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_FeeVaultWithdrawalAmountMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_SharesCalculatorCannotBeZero as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_SharesCalculatorMalformedOutput as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_FeeVaultMustWithdrawToL2 as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_FeeVaultMustWithdrawToFeeSplitter as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_NoFeesCollected as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_FeeDisbursementIntervalCannotBeZero as alloy_sol_types::SolError>::SIGNATURE,
            <FeeSplitter_FailedToSendToRevenueShareRecipient as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for FeeSplitterErrors {
        const NAME: &'static str = "FeeSplitterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::FeeSplitter_DisbursementIntervalNotReached(_) => {
                    <FeeSplitter_DisbursementIntervalNotReached as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_ExceedsMaxFeeDisbursementTime(_) => {
                    <FeeSplitter_ExceedsMaxFeeDisbursementTime as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_FailedToSendToRevenueShareRecipient(_) => {
                    <FeeSplitter_FailedToSendToRevenueShareRecipient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_FeeDisbursementIntervalCannotBeZero(_) => {
                    <FeeSplitter_FeeDisbursementIntervalCannotBeZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_FeeShareInfoEmpty(_) => {
                    <FeeSplitter_FeeShareInfoEmpty as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_FeeVaultMustWithdrawToFeeSplitter(_) => {
                    <FeeSplitter_FeeVaultMustWithdrawToFeeSplitter as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_FeeVaultMustWithdrawToL2(_) => {
                    <FeeSplitter_FeeVaultMustWithdrawToL2 as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_FeeVaultWithdrawalAmountMismatch(_) => {
                    <FeeSplitter_FeeVaultWithdrawalAmountMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_NoFeesCollected(_) => {
                    <FeeSplitter_NoFeesCollected as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_OnlyProxyAdminOwner(_) => {
                    <FeeSplitter_OnlyProxyAdminOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_SenderNotCurrentVault(_) => {
                    <FeeSplitter_SenderNotCurrentVault as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_SharesCalculatorCannotBeZero(_) => {
                    <FeeSplitter_SharesCalculatorCannotBeZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeSplitter_SharesCalculatorMalformedOutput(_) => {
                    <FeeSplitter_SharesCalculatorMalformedOutput as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<FeeSplitterErrors>] = &[
                {
                    fn FeeSplitter_SenderNotCurrentVault(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_SenderNotCurrentVault as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_SenderNotCurrentVault)
                    }
                    FeeSplitter_SenderNotCurrentVault
                },
                {
                    fn FeeSplitter_DisbursementIntervalNotReached(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_DisbursementIntervalNotReached as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_DisbursementIntervalNotReached,
                            )
                    }
                    FeeSplitter_DisbursementIntervalNotReached
                },
                {
                    fn FeeSplitter_ExceedsMaxFeeDisbursementTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_ExceedsMaxFeeDisbursementTime as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_ExceedsMaxFeeDisbursementTime,
                            )
                    }
                    FeeSplitter_ExceedsMaxFeeDisbursementTime
                },
                {
                    fn FeeSplitter_OnlyProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_OnlyProxyAdminOwner)
                    }
                    FeeSplitter_OnlyProxyAdminOwner
                },
                {
                    fn FeeSplitter_FeeShareInfoEmpty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeShareInfoEmpty as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_FeeShareInfoEmpty)
                    }
                    FeeSplitter_FeeShareInfoEmpty
                },
                {
                    fn FeeSplitter_FeeVaultWithdrawalAmountMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeVaultWithdrawalAmountMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FeeVaultWithdrawalAmountMismatch,
                            )
                    }
                    FeeSplitter_FeeVaultWithdrawalAmountMismatch
                },
                {
                    fn FeeSplitter_SharesCalculatorCannotBeZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_SharesCalculatorCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_SharesCalculatorCannotBeZero,
                            )
                    }
                    FeeSplitter_SharesCalculatorCannotBeZero
                },
                {
                    fn FeeSplitter_SharesCalculatorMalformedOutput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_SharesCalculatorMalformedOutput as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_SharesCalculatorMalformedOutput,
                            )
                    }
                    FeeSplitter_SharesCalculatorMalformedOutput
                },
                {
                    fn FeeSplitter_FeeVaultMustWithdrawToL2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeVaultMustWithdrawToL2 as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_FeeVaultMustWithdrawToL2)
                    }
                    FeeSplitter_FeeVaultMustWithdrawToL2
                },
                {
                    fn FeeSplitter_FeeVaultMustWithdrawToFeeSplitter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeVaultMustWithdrawToFeeSplitter as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FeeVaultMustWithdrawToFeeSplitter,
                            )
                    }
                    FeeSplitter_FeeVaultMustWithdrawToFeeSplitter
                },
                {
                    fn FeeSplitter_NoFeesCollected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_NoFeesCollected as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_NoFeesCollected)
                    }
                    FeeSplitter_NoFeesCollected
                },
                {
                    fn FeeSplitter_FeeDisbursementIntervalCannotBeZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeDisbursementIntervalCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FeeDisbursementIntervalCannotBeZero,
                            )
                    }
                    FeeSplitter_FeeDisbursementIntervalCannotBeZero
                },
                {
                    fn FeeSplitter_FailedToSendToRevenueShareRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FailedToSendToRevenueShareRecipient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FailedToSendToRevenueShareRecipient,
                            )
                    }
                    FeeSplitter_FailedToSendToRevenueShareRecipient
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
            ) -> alloy_sol_types::Result<FeeSplitterErrors>] = &[
                {
                    fn FeeSplitter_SenderNotCurrentVault(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_SenderNotCurrentVault as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_SenderNotCurrentVault)
                    }
                    FeeSplitter_SenderNotCurrentVault
                },
                {
                    fn FeeSplitter_DisbursementIntervalNotReached(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_DisbursementIntervalNotReached as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_DisbursementIntervalNotReached,
                            )
                    }
                    FeeSplitter_DisbursementIntervalNotReached
                },
                {
                    fn FeeSplitter_ExceedsMaxFeeDisbursementTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_ExceedsMaxFeeDisbursementTime as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_ExceedsMaxFeeDisbursementTime,
                            )
                    }
                    FeeSplitter_ExceedsMaxFeeDisbursementTime
                },
                {
                    fn FeeSplitter_OnlyProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_OnlyProxyAdminOwner)
                    }
                    FeeSplitter_OnlyProxyAdminOwner
                },
                {
                    fn FeeSplitter_FeeShareInfoEmpty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeShareInfoEmpty as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_FeeShareInfoEmpty)
                    }
                    FeeSplitter_FeeShareInfoEmpty
                },
                {
                    fn FeeSplitter_FeeVaultWithdrawalAmountMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeVaultWithdrawalAmountMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FeeVaultWithdrawalAmountMismatch,
                            )
                    }
                    FeeSplitter_FeeVaultWithdrawalAmountMismatch
                },
                {
                    fn FeeSplitter_SharesCalculatorCannotBeZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_SharesCalculatorCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_SharesCalculatorCannotBeZero,
                            )
                    }
                    FeeSplitter_SharesCalculatorCannotBeZero
                },
                {
                    fn FeeSplitter_SharesCalculatorMalformedOutput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_SharesCalculatorMalformedOutput as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_SharesCalculatorMalformedOutput,
                            )
                    }
                    FeeSplitter_SharesCalculatorMalformedOutput
                },
                {
                    fn FeeSplitter_FeeVaultMustWithdrawToL2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeVaultMustWithdrawToL2 as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_FeeVaultMustWithdrawToL2)
                    }
                    FeeSplitter_FeeVaultMustWithdrawToL2
                },
                {
                    fn FeeSplitter_FeeVaultMustWithdrawToFeeSplitter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeVaultMustWithdrawToFeeSplitter as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FeeVaultMustWithdrawToFeeSplitter,
                            )
                    }
                    FeeSplitter_FeeVaultMustWithdrawToFeeSplitter
                },
                {
                    fn FeeSplitter_NoFeesCollected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_NoFeesCollected as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FeeSplitterErrors::FeeSplitter_NoFeesCollected)
                    }
                    FeeSplitter_NoFeesCollected
                },
                {
                    fn FeeSplitter_FeeDisbursementIntervalCannotBeZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FeeDisbursementIntervalCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FeeDisbursementIntervalCannotBeZero,
                            )
                    }
                    FeeSplitter_FeeDisbursementIntervalCannotBeZero
                },
                {
                    fn FeeSplitter_FailedToSendToRevenueShareRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FeeSplitterErrors> {
                        <FeeSplitter_FailedToSendToRevenueShareRecipient as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                FeeSplitterErrors::FeeSplitter_FailedToSendToRevenueShareRecipient,
                            )
                    }
                    FeeSplitter_FailedToSendToRevenueShareRecipient
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
                Self::FeeSplitter_DisbursementIntervalNotReached(inner) => {
                    <FeeSplitter_DisbursementIntervalNotReached as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_ExceedsMaxFeeDisbursementTime(inner) => {
                    <FeeSplitter_ExceedsMaxFeeDisbursementTime as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_FailedToSendToRevenueShareRecipient(inner) => {
                    <FeeSplitter_FailedToSendToRevenueShareRecipient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_FeeDisbursementIntervalCannotBeZero(inner) => {
                    <FeeSplitter_FeeDisbursementIntervalCannotBeZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_FeeShareInfoEmpty(inner) => {
                    <FeeSplitter_FeeShareInfoEmpty as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_FeeVaultMustWithdrawToFeeSplitter(inner) => {
                    <FeeSplitter_FeeVaultMustWithdrawToFeeSplitter as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_FeeVaultMustWithdrawToL2(inner) => {
                    <FeeSplitter_FeeVaultMustWithdrawToL2 as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_FeeVaultWithdrawalAmountMismatch(inner) => {
                    <FeeSplitter_FeeVaultWithdrawalAmountMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_NoFeesCollected(inner) => {
                    <FeeSplitter_NoFeesCollected as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_OnlyProxyAdminOwner(inner) => {
                    <FeeSplitter_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_SenderNotCurrentVault(inner) => {
                    <FeeSplitter_SenderNotCurrentVault as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_SharesCalculatorCannotBeZero(inner) => {
                    <FeeSplitter_SharesCalculatorCannotBeZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeSplitter_SharesCalculatorMalformedOutput(inner) => {
                    <FeeSplitter_SharesCalculatorMalformedOutput as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::FeeSplitter_DisbursementIntervalNotReached(inner) => {
                    <FeeSplitter_DisbursementIntervalNotReached as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_ExceedsMaxFeeDisbursementTime(inner) => {
                    <FeeSplitter_ExceedsMaxFeeDisbursementTime as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_FailedToSendToRevenueShareRecipient(inner) => {
                    <FeeSplitter_FailedToSendToRevenueShareRecipient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_FeeDisbursementIntervalCannotBeZero(inner) => {
                    <FeeSplitter_FeeDisbursementIntervalCannotBeZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_FeeShareInfoEmpty(inner) => {
                    <FeeSplitter_FeeShareInfoEmpty as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_FeeVaultMustWithdrawToFeeSplitter(inner) => {
                    <FeeSplitter_FeeVaultMustWithdrawToFeeSplitter as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_FeeVaultMustWithdrawToL2(inner) => {
                    <FeeSplitter_FeeVaultMustWithdrawToL2 as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_FeeVaultWithdrawalAmountMismatch(inner) => {
                    <FeeSplitter_FeeVaultWithdrawalAmountMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_NoFeesCollected(inner) => {
                    <FeeSplitter_NoFeesCollected as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_OnlyProxyAdminOwner(inner) => {
                    <FeeSplitter_OnlyProxyAdminOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_SenderNotCurrentVault(inner) => {
                    <FeeSplitter_SenderNotCurrentVault as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_SharesCalculatorCannotBeZero(inner) => {
                    <FeeSplitter_SharesCalculatorCannotBeZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeSplitter_SharesCalculatorMalformedOutput(inner) => {
                    <FeeSplitter_SharesCalculatorMalformedOutput as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`FeeSplitter`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum FeeSplitterEvents {
        #[allow(missing_docs)]
        FeeDisbursementIntervalUpdated(FeeDisbursementIntervalUpdated),
        #[allow(missing_docs)]
        FeesDisbursed(FeesDisbursed),
        #[allow(missing_docs)]
        FeesReceived(FeesReceived),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        SharesCalculatorUpdated(SharesCalculatorUpdated),
    }
    impl FeeSplitterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                22u8, 65u8, 124u8, 195u8, 114u8, 222u8, 236u8, 12u8, 174u8, 229u8, 245u8,
                46u8, 42u8, 215u8, 122u8, 95u8, 7u8, 180u8, 89u8, 31u8, 213u8, 107u8,
                79u8, 243u8, 27u8, 110u8, 32u8, 248u8, 23u8, 212u8, 218u8, 235u8,
            ],
            [
                33u8, 62u8, 114u8, 175u8, 13u8, 54u8, 19u8, 189u8, 100u8, 60u8, 255u8,
                48u8, 89u8, 248u8, 114u8, 193u8, 1u8, 94u8, 101u8, 65u8, 98u8, 78u8,
                55u8, 135u8, 43u8, 249u8, 94u8, 239u8, 186u8, 242u8, 32u8, 168u8,
            ],
            [
                68u8, 146u8, 8u8, 107u8, 99u8, 14u8, 211u8, 132u8, 110u8, 236u8, 9u8,
                121u8, 221u8, 135u8, 167u8, 28u8, 129u8, 76u8, 235u8, 28u8, 109u8, 171u8,
                128u8, 171u8, 129u8, 227u8, 69u8, 11u8, 33u8, 228u8, 222u8, 40u8,
            ],
            [
                115u8, 249u8, 161u8, 50u8, 65u8, 161u8, 132u8, 142u8, 193u8, 87u8, 150u8,
                127u8, 58u8, 133u8, 96u8, 23u8, 9u8, 53u8, 62u8, 97u8, 111u8, 31u8, 38u8,
                5u8, 216u8, 24u8, 192u8, 242u8, 210u8, 23u8, 116u8, 223u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(SharesCalculatorUpdated),
            ::core::stringify!(FeesReceived),
            ::core::stringify!(FeeDisbursementIntervalUpdated),
            ::core::stringify!(FeesDisbursed),
            ::core::stringify!(Initialized),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <SharesCalculatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <FeesReceived as alloy_sol_types::SolEvent>::SIGNATURE,
            <FeeDisbursementIntervalUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <FeesDisbursed as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for FeeSplitterEvents {
        const NAME: &'static str = "FeeSplitterEvents";
        const COUNT: usize = 5usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <FeeDisbursementIntervalUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <FeeDisbursementIntervalUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::FeeDisbursementIntervalUpdated)
                }
                Some(<FeesDisbursed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <FeesDisbursed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::FeesDisbursed)
                }
                Some(<FeesReceived as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <FeesReceived as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::FeesReceived)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <SharesCalculatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SharesCalculatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SharesCalculatorUpdated)
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
    impl alloy_sol_types::private::IntoLogData for FeeSplitterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::FeeDisbursementIntervalUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FeesDisbursed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FeesReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SharesCalculatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::FeeDisbursementIntervalUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FeesDisbursed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FeesReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SharesCalculatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`FeeSplitter`](self) contract instance.

See the [wrapper's documentation](`FeeSplitterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> FeeSplitterInstance<P, N> {
        FeeSplitterInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<FeeSplitterInstance<P, N>>,
    > {
        FeeSplitterInstance::<P, N>::deploy(__provider)
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
        FeeSplitterInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`FeeSplitter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`FeeSplitter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct FeeSplitterInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for FeeSplitterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("FeeSplitterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FeeSplitterInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`FeeSplitter`](self) contract instance.

See the [wrapper's documentation](`FeeSplitterInstance`) for more details.*/
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
        ) -> alloy_contract::Result<FeeSplitterInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> FeeSplitterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> FeeSplitterInstance<P, N> {
            FeeSplitterInstance {
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
    > FeeSplitterInstance<P, N> {
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
        ///Creates a new call builder for the [`MAX_DISBURSEMENT_INTERVAL`] function.
        pub fn MAX_DISBURSEMENT_INTERVAL(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_DISBURSEMENT_INTERVALCall, N> {
            self.call_builder(&MAX_DISBURSEMENT_INTERVALCall)
        }
        ///Creates a new call builder for the [`disburseFees`] function.
        pub fn disburseFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, disburseFeesCall, N> {
            self.call_builder(&disburseFeesCall)
        }
        ///Creates a new call builder for the [`feeDisbursementInterval`] function.
        pub fn feeDisbursementInterval(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, feeDisbursementIntervalCall, N> {
            self.call_builder(&feeDisbursementIntervalCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _sharesCalculator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _sharesCalculator,
                },
            )
        }
        ///Creates a new call builder for the [`lastDisbursementTime`] function.
        pub fn lastDisbursementTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lastDisbursementTimeCall, N> {
            self.call_builder(&lastDisbursementTimeCall)
        }
        ///Creates a new call builder for the [`setFeeDisbursementInterval`] function.
        pub fn setFeeDisbursementInterval(
            &self,
            _newFeeDisbursementInterval: u128,
        ) -> alloy_contract::SolCallBuilder<&P, setFeeDisbursementIntervalCall, N> {
            self.call_builder(
                &setFeeDisbursementIntervalCall {
                    _newFeeDisbursementInterval,
                },
            )
        }
        ///Creates a new call builder for the [`setSharesCalculator`] function.
        pub fn setSharesCalculator(
            &self,
            _newSharesCalculator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setSharesCalculatorCall, N> {
            self.call_builder(
                &setSharesCalculatorCall {
                    _newSharesCalculator,
                },
            )
        }
        ///Creates a new call builder for the [`sharesCalculator`] function.
        pub fn sharesCalculator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, sharesCalculatorCall, N> {
            self.call_builder(&sharesCalculatorCall)
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
    > FeeSplitterInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`FeeDisbursementIntervalUpdated`] event.
        pub fn FeeDisbursementIntervalUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, FeeDisbursementIntervalUpdated, N> {
            self.event_filter::<FeeDisbursementIntervalUpdated>()
        }
        ///Creates a new event filter for the [`FeesDisbursed`] event.
        pub fn FeesDisbursed_filter(
            &self,
        ) -> alloy_contract::Event<&P, FeesDisbursed, N> {
            self.event_filter::<FeesDisbursed>()
        }
        ///Creates a new event filter for the [`FeesReceived`] event.
        pub fn FeesReceived_filter(&self) -> alloy_contract::Event<&P, FeesReceived, N> {
            self.event_filter::<FeesReceived>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`SharesCalculatorUpdated`] event.
        pub fn SharesCalculatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, SharesCalculatorUpdated, N> {
            self.event_filter::<SharesCalculatorUpdated>()
        }
    }
}

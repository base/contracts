///Module containing a contract's types and functions.
/**

```solidity
library Enum {
    type Operation is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Enum {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operation(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Operation> for u8 {
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
        impl Operation {
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
        impl From<u8> for Operation {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Operation> for u8 {
            fn from(value: Operation) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Operation {
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
        impl alloy_sol_types::EventTopic for Operation {
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, __provider: P) -> EnumInstance<P, N> {
        EnumInstance::<P, N>::new(address, __provider)
    }
    /**A [`Enum`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Enum`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EnumInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for EnumInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EnumInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > EnumInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> EnumInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EnumInstance<P, N> {
            EnumInstance {
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
    > EnumInstance<P, N> {
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
    > EnumInstance<P, N> {
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
library Enum {
    type Operation is uint8;
}

interface LivenessGuard {
    event OwnerRecorded(address owner);

    constructor(address _safe);

    function checkAfterExecution(bytes32, bool) external;
    function checkTransaction(address _to, uint256 _value, bytes memory _data, Enum.Operation _operation, uint256 _safeTxGas, uint256 _baseGas, uint256 _gasPrice, address _gasToken, address payable _refundReceiver, bytes memory _signatures, address _msgSender) external;
    function lastLive(address) external view returns (uint256);
    function safe() external view returns (address safe_);
    function showLiveness() external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
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
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkAfterExecution",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkTransaction",
    "inputs": [
      {
        "name": "_to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_operation",
        "type": "uint8",
        "internalType": "enum Enum.Operation"
      },
      {
        "name": "_safeTxGas",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_baseGas",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_gasPrice",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_gasToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_refundReceiver",
        "type": "address",
        "internalType": "address payable"
      },
      {
        "name": "_signatures",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_msgSender",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lastLive",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
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
    "name": "safe",
    "inputs": [],
    "outputs": [
      {
        "name": "safe_",
        "type": "address",
        "internalType": "contract Safe"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "showLiveness",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
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
    "name": "OwnerRecorded",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
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
pub mod LivenessGuard {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523480156200001157600080fd5b506040516200177138038062001771833981016040819052620000349162000170565b806001600160a01b03166080816001600160a01b0316815250506000816001600160a01b031663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa1580156200008f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052620000b99190810190620001bf565b905060005b81518110156200014e576000828281518110620000df57620000df62000291565b6020908102919091018101516001600160a01b0381166000818152808452604090819020429055519081529092507f833bc129023866d52116d61e94b791eb8be46f05709362e0bcf1fe7c1a8c225c910160405180910390a150806200014581620002a7565b915050620000be565b505050620002cf565b6001600160a01b03811681146200016d57600080fd5b50565b6000602082840312156200018357600080fd5b8151620001908162000157565b9392505050565b634e487b7160e01b600052604160045260246000fd5b8051620001ba8162000157565b919050565b60006020808385031215620001d357600080fd5b82516001600160401b0380821115620001eb57600080fd5b818501915085601f8301126200020057600080fd5b81518181111562000215576200021562000197565b8060051b604051601f19603f830116810181811085821117156200023d576200023d62000197565b6040529182528482019250838101850191888311156200025c57600080fd5b938501935b8285101562000285576200027585620001ad565b8452938501939285019262000261565b98975050505050505050565b634e487b7160e01b600052603260045260246000fd5b600060018201620002c857634e487b7160e01b600052601160045260246000fd5b5060010190565b6080516114566200031b6000396000818160c40152818161025c015281816103bf015281816104c1015281816105080152818161060b015281816107ad015261098501526114566000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c806354fd4d501161005b57806354fd4d50146100f857806375f0bb52146101415780639327136814610154578063e458779b1461016757600080fd5b806301ffc9a714610082578063186f0354146100aa5780634c205d0d146100ee575b600080fd5b610095610090366004610e47565b610195565b60405190151581526020015b60405180910390f35b60405173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016100a1565b6100f661022e565b005b6101346040518060400160405280600581526020017f312e312e3000000000000000000000000000000000000000000000000000000081525081565b6040516100a19190610ef4565b6100f661014f366004611057565b6103b3565b6100f661016236600461114a565b6107a1565b61018761017536600461117a565b60006020819052908152604090205481565b6040519081526020016100a1565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167fe6d7a83a00000000000000000000000000000000000000000000000000000000148061022857507fffffffff0000000000000000000000000000000000000000000000000000000082167f01ffc9a700000000000000000000000000000000000000000000000000000000145b92915050565b6040517f2f54bf6e0000000000000000000000000000000000000000000000000000000081523360048201527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1690632f54bf6e90602401602060405180830381865afa1580156102b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102dc9190611197565b61036d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4c6976656e65737347756172643a206f6e6c792053616665206f776e6572732060448201527f6d61792064656d6f6e737472617465206c6976656e657373000000000000000060648201526084015b60405180910390fd5b336000818152602081815260409182902042905590519182527f833bc129023866d52116d61e94b791eb8be46f05709362e0bcf1fe7c1a8c225c910160405180910390a1565b6103bb61096d565b60007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015610428573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261046e91908101906111b4565b905060005b81518110156104bc576104a982828151811061049157610491611266565b60200260200101516001610a3490919063ffffffff16565b50806104b4816112c4565b915050610473565b5060007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663d8d11f788e8e8e8e8e8e8e8e8e60017f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663affed0e06040518163ffffffff1660e01b8152600401602060405180830381865afa158015610571573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061059591906112fc565b61059f9190611315565b6040518b63ffffffff1660e01b81526004016105c49a9998979695949392919061132c565b602060405180830381865afa1580156105e1573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061060591906112fc565b905060007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663e75235b86040518163ffffffff1660e01b8152600401602060405180830381865afa158015610674573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061069891906112fc565b905060006106a7838784610a5d565b905060005b815181101561078f57426000808484815181106106cb576106cb611266565b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055507f833bc129023866d52116d61e94b791eb8be46f05709362e0bcf1fe7c1a8c225c82828151811061074457610744611266565b6020026020010151604051610775919073ffffffffffffffffffffffffffffffffffffffff91909116815260200190565b60405180910390a180610787816112c4565b9150506106ac565b50505050505050505050505050505050565b6107a961096d565b60007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015610816573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261085c91908101906111b4565b905060005b81518110156108e157600082828151811061087e5761087e611266565b6020026020010151905061089c816001610c7a90919063ffffffff16565b15156000036108ce5773ffffffffffffffffffffffffffffffffffffffff811660009081526020819052604090204290555b50806108d9816112c4565b915050610861565b5060006108ee6001610c9c565b905060005b815181101561096657600082828151811061091057610910611266565b60209081029190910181015173ffffffffffffffffffffffffffffffffffffffff811660009081529182905260408220919091559050610951600182610c7a565b5050808061095e906112c4565b9150506108f3565b5050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610a32576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602f60248201527f4c6976656e65737347756172643a206f6e6c7920536166652063616e2063616c60448201527f6c20746869732066756e6374696f6e00000000000000000000000000000000006064820152608401610364565b565b6000610a568373ffffffffffffffffffffffffffffffffffffffff8416610ca9565b9392505050565b60608167ffffffffffffffff811115610a7857610a78610f3c565b604051908082528060200260200182016040528015610aa1578160200160208202803683370190505b50905060008060008060005b86811015610c6e576041818102890160208101516040820151919092015160ff16955090935091506000849003610ae9578260001c9450610c24565b8360ff16600103610aff578260001c9450610c24565b601e8460ff161115610bc4576040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018a9052600190605c0160405160208183030381529060405280519060200120600486610b6491906113f7565b6040805160008152602081018083529390935260ff90911690820152606081018590526080810184905260a0016020604051602081039080840390855afa158015610bb3573d6000803e3d6000fd5b505050602060405103519450610c24565b6040805160008152602081018083528b905260ff861691810191909152606081018490526080810183905260019060a0016020604051602081039080840390855afa158015610c17573d6000803e3d6000fd5b5050506020604051035194505b84868281518110610c3757610c37611266565b73ffffffffffffffffffffffffffffffffffffffff9092166020928302919091019091015280610c66816112c4565b915050610aad565b50505050509392505050565b6000610a568373ffffffffffffffffffffffffffffffffffffffff8416610cf8565b60606000610a5683610deb565b6000818152600183016020526040812054610cf057508154600181810184556000848152602080822090930184905584548482528286019093526040902091909155610228565b506000610228565b60008181526001830160205260408120548015610de1576000610d1c600183611315565b8554909150600090610d3090600190611315565b9050818114610d95576000866000018281548110610d5057610d50611266565b9060005260206000200154905080876000018481548110610d7357610d73611266565b6000918252602080832090910192909255918252600188019052604090208390555b8554869080610da657610da661141a565b600190038181906000526020600020016000905590558560010160008681526020019081526020016000206000905560019350505050610228565b6000915050610228565b606081600001805480602002602001604051908101604052809291908181526020018280548015610e3b57602002820191906000526020600020905b815481526020019060010190808311610e27575b50505050509050919050565b600060208284031215610e5957600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114610a5657600080fd5b6000815180845260005b81811015610eaf57602081850181015186830182015201610e93565b81811115610ec1576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b602081526000610a566020830184610e89565b73ffffffffffffffffffffffffffffffffffffffff81168114610f2957600080fd5b50565b8035610f3781610f07565b919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715610fb257610fb2610f3c565b604052919050565b600082601f830112610fcb57600080fd5b813567ffffffffffffffff811115610fe557610fe5610f3c565b61101660207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601610f6b565b81815284602083860101111561102b57600080fd5b816020850160208301376000918101602001919091529392505050565b803560028110610f3757600080fd5b60008060008060008060008060008060006101608c8e03121561107957600080fd5b6110828c610f2c565b9a5060208c0135995067ffffffffffffffff8060408e013511156110a557600080fd5b6110b58e60408f01358f01610fba565b99506110c360608e01611048565b985060808d0135975060a08d0135965060c08d013595506110e660e08e01610f2c565b94506110f56101008e01610f2c565b9350806101208e0135111561110957600080fd5b5061111b8d6101208e01358e01610fba565b915061112a6101408d01610f2c565b90509295989b509295989b9093969950565b8015158114610f2957600080fd5b6000806040838503121561115d57600080fd5b82359150602083013561116f8161113c565b809150509250929050565b60006020828403121561118c57600080fd5b8135610a5681610f07565b6000602082840312156111a957600080fd5b8151610a568161113c565b600060208083850312156111c757600080fd5b825167ffffffffffffffff808211156111df57600080fd5b818501915085601f8301126111f357600080fd5b81518181111561120557611205610f3c565b8060051b9150611216848301610f6b565b818152918301840191848101908884111561123057600080fd5b938501935b8385101561125a578451925061124a83610f07565b8282529385019390850190611235565b98975050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036112f5576112f5611295565b5060010190565b60006020828403121561130e57600080fd5b5051919050565b60008282101561132757611327611295565b500390565b600061014073ffffffffffffffffffffffffffffffffffffffff808e1684528c60208501528160408501526113638285018d610e89565b925060028b1061139c577f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b8a60608501528960808501528860a08501528760c085015280871660e085015250506113e161010083018573ffffffffffffffffffffffffffffffffffffffff169052565b826101208301529b9a5050505050505050505050565b600060ff821660ff84168082101561141157611411611295565b90039392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603160045260246000fdfea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x17q8\x03\x80b\0\x17q\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01pV[\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\0\xB9\x91\x90\x81\x01\x90b\0\x01\xBFV[\x90P`\0[\x81Q\x81\x10\x15b\0\x01NW`\0\x82\x82\x81Q\x81\x10b\0\0\xDFWb\0\0\xDFb\0\x02\x91V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R\x80\x84R`@\x90\x81\x90 B\x90UQ\x90\x81R\x90\x92P\x7F\x83;\xC1)\x028f\xD5!\x16\xD6\x1E\x94\xB7\x91\xEB\x8B\xE4o\x05p\x93b\xE0\xBC\xF1\xFE|\x1A\x8C\"\\\x91\x01`@Q\x80\x91\x03\x90\xA1P\x80b\0\x01E\x81b\0\x02\xA7V[\x91PPb\0\0\xBEV[PPPb\0\x02\xCFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01mW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15b\0\x01\x83W`\0\x80\xFD[\x81Qb\0\x01\x90\x81b\0\x01WV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qb\0\x01\xBA\x81b\0\x01WV[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15b\0\x01\xD3W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01\xEBW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x02\0W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x02\x15Wb\0\x02\x15b\0\x01\x97V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15b\0\x02=Wb\0\x02=b\0\x01\x97V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15b\0\x02\\W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x02\x85Wb\0\x02u\x85b\0\x01\xADV[\x84R\x93\x85\x01\x93\x92\x85\x01\x92b\0\x02aV[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\0\x02\xC8WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\x80Qa\x14Vb\0\x03\x1B`\09`\0\x81\x81`\xC4\x01R\x81\x81a\x02\\\x01R\x81\x81a\x03\xBF\x01R\x81\x81a\x04\xC1\x01R\x81\x81a\x05\x08\x01R\x81\x81a\x06\x0B\x01R\x81\x81a\x07\xAD\x01Ra\t\x85\x01Ra\x14V`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0[W\x80cT\xFDMP\x14a\0\xF8W\x80cu\xF0\xBBR\x14a\x01AW\x80c\x93'\x13h\x14a\x01TW\x80c\xE4Xw\x9B\x14a\x01gW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\x82W\x80c\x18o\x03T\x14a\0\xAAW\x80cL ]\r\x14a\0\xEEW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x0EGV[a\x01\x95V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\0\xA1V[a\0\xF6a\x02.V[\0[a\x014`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.1.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xA1\x91\x90a\x0E\xF4V[a\0\xF6a\x01O6`\x04a\x10WV[a\x03\xB3V[a\0\xF6a\x01b6`\x04a\x11JV[a\x07\xA1V[a\x01\x87a\x01u6`\x04a\x11zV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xA1V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE6\xD7\xA8:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x02(WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x92\x91PPV[`@Q\x7F/T\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c/T\xBFn\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDC\x91\x90a\x11\x97V[a\x03mW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FLivenessGuard: only Safe owners `D\x82\x01R\x7Fmay demonstrate liveness\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 B\x90U\x90Q\x91\x82R\x7F\x83;\xC1)\x028f\xD5!\x16\xD6\x1E\x94\xB7\x91\xEB\x8B\xE4o\x05p\x93b\xE0\xBC\xF1\xFE|\x1A\x8C\"\\\x91\x01`@Q\x80\x91\x03\x90\xA1V[a\x03\xBBa\tmV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04n\x91\x90\x81\x01\x90a\x11\xB4V[\x90P`\0[\x81Q\x81\x10\x15a\x04\xBCWa\x04\xA9\x82\x82\x81Q\x81\x10a\x04\x91Wa\x04\x91a\x12fV[` \x02` \x01\x01Q`\x01a\n4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80a\x04\xB4\x81a\x12\xC4V[\x91PPa\x04sV[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAF\xFE\xD0\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x95\x91\x90a\x12\xFCV[a\x05\x9F\x91\x90a\x13\x15V[`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xC4\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x13,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x05\x91\x90a\x12\xFCV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x98\x91\x90a\x12\xFCV[\x90P`\0a\x06\xA7\x83\x87\x84a\n]V[\x90P`\0[\x81Q\x81\x10\x15a\x07\x8FWB`\0\x80\x84\x84\x81Q\x81\x10a\x06\xCBWa\x06\xCBa\x12fV[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F\x83;\xC1)\x028f\xD5!\x16\xD6\x1E\x94\xB7\x91\xEB\x8B\xE4o\x05p\x93b\xE0\xBC\xF1\xFE|\x1A\x8C\"\\\x82\x82\x81Q\x81\x10a\x07DWa\x07Da\x12fV[` \x02` \x01\x01Q`@Qa\x07u\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x80a\x07\x87\x81a\x12\xC4V[\x91PPa\x06\xACV[PPPPPPPPPPPPPPPPV[a\x07\xA9a\tmV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\\\x91\x90\x81\x01\x90a\x11\xB4V[\x90P`\0[\x81Q\x81\x10\x15a\x08\xE1W`\0\x82\x82\x81Q\x81\x10a\x08~Wa\x08~a\x12fV[` \x02` \x01\x01Q\x90Pa\x08\x9C\x81`\x01a\x0Cz\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x15\x15`\0\x03a\x08\xCEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 B\x90U[P\x80a\x08\xD9\x81a\x12\xC4V[\x91PPa\x08aV[P`\0a\x08\xEE`\x01a\x0C\x9CV[\x90P`\0[\x81Q\x81\x10\x15a\tfW`\0\x82\x82\x81Q\x81\x10a\t\x10Wa\t\x10a\x12fV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x91\x82\x90R`@\x82 \x91\x90\x91U\x90Pa\tQ`\x01\x82a\x0CzV[PP\x80\x80a\t^\x90a\x12\xC4V[\x91PPa\x08\xF3V[PPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FLivenessGuard: only Safe can cal`D\x82\x01R\x7Fl this function\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03dV[V[`\0a\nV\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x0C\xA9V[\x93\x92PPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nxWa\nxa\x0F<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xA1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\0\x80`\0[\x86\x81\x10\x15a\x0CnW`A\x81\x81\x02\x89\x01` \x81\x01Q`@\x82\x01Q\x91\x90\x92\x01Q`\xFF\x16\x95P\x90\x93P\x91P`\0\x84\x90\x03a\n\xE9W\x82`\0\x1C\x94Pa\x0C$V[\x83`\xFF\x16`\x01\x03a\n\xFFW\x82`\0\x1C\x94Pa\x0C$V[`\x1E\x84`\xFF\x16\x11\x15a\x0B\xC4W`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x8A\x90R`\x01\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04\x86a\x0Bd\x91\x90a\x13\xF7V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x93\x90\x93R`\xFF\x90\x91\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\xB3W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94Pa\x0C$V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x86\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0C\x17W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94P[\x84\x86\x82\x81Q\x81\x10a\x0C7Wa\x0C7a\x12fV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0Cf\x81a\x12\xC4V[\x91PPa\n\xADV[PPPPP\x93\x92PPPV[`\0a\nV\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x0C\xF8V[```\0a\nV\x83a\r\xEBV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x0C\xF0WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02(V[P`\0a\x02(V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\r\xE1W`\0a\r\x1C`\x01\x83a\x13\x15V[\x85T\x90\x91P`\0\x90a\r0\x90`\x01\x90a\x13\x15V[\x90P\x81\x81\x14a\r\x95W`\0\x86`\0\x01\x82\x81T\x81\x10a\rPWa\rPa\x12fV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\rsWa\rsa\x12fV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\r\xA6Wa\r\xA6a\x14\x1AV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02(V[`\0\x91PPa\x02(V[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E;W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E'W[PPPPP\x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0EYW`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\nVW`\0\x80\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0E\xAFW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0E\x93V[\x81\x81\x11\x15a\x0E\xC1W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\nV` \x83\x01\x84a\x0E\x89V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F)W`\0\x80\xFD[PV[\x805a\x0F7\x81a\x0F\x07V[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xB2Wa\x0F\xB2a\x0F<V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0F\xCBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xE5Wa\x0F\xE5a\x0F<V[a\x10\x16` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x0FkV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x10+W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x02\x81\x10a\x0F7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a\x10yW`\0\x80\xFD[a\x10\x82\x8Ca\x0F,V[\x9AP` \x8C\x015\x99Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@\x8E\x015\x11\x15a\x10\xA5W`\0\x80\xFD[a\x10\xB5\x8E`@\x8F\x015\x8F\x01a\x0F\xBAV[\x99Pa\x10\xC3``\x8E\x01a\x10HV[\x98P`\x80\x8D\x015\x97P`\xA0\x8D\x015\x96P`\xC0\x8D\x015\x95Pa\x10\xE6`\xE0\x8E\x01a\x0F,V[\x94Pa\x10\xF5a\x01\0\x8E\x01a\x0F,V[\x93P\x80a\x01 \x8E\x015\x11\x15a\x11\tW`\0\x80\xFD[Pa\x11\x1B\x8Da\x01 \x8E\x015\x8E\x01a\x0F\xBAV[\x91Pa\x11*a\x01@\x8D\x01a\x0F,V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[\x80\x15\x15\x81\x14a\x0F)W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x11]W`\0\x80\xFD[\x825\x91P` \x83\x015a\x11o\x81a\x11<V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\x8CW`\0\x80\xFD[\x815a\nV\x81a\x0F\x07V[`\0` \x82\x84\x03\x12\x15a\x11\xA9W`\0\x80\xFD[\x81Qa\nV\x81a\x11<V[`\0` \x80\x83\x85\x03\x12\x15a\x11\xC7W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\xDFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x11\xF3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x12\x05Wa\x12\x05a\x0F<V[\x80`\x05\x1B\x91Pa\x12\x16\x84\x83\x01a\x0FkV[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x120W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x12ZW\x84Q\x92Pa\x12J\x83a\x0F\x07V[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x125V[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x12\xF5Wa\x12\xF5a\x12\x95V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x13\x0EW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a\x13'Wa\x13'a\x12\x95V[P\x03\x90V[`\0a\x01@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8E\x16\x84R\x8C` \x85\x01R\x81`@\x85\x01Ra\x13c\x82\x85\x01\x8Da\x0E\x89V[\x92P`\x02\x8B\x10a\x13\x9CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x8A``\x85\x01R\x89`\x80\x85\x01R\x88`\xA0\x85\x01R\x87`\xC0\x85\x01R\x80\x87\x16`\xE0\x85\x01RPPa\x13\xE1a\x01\0\x83\x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x82a\x01 \x83\x01R\x9B\x9APPPPPPPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a\x14\x11Wa\x14\x11a\x12\x95V[\x90\x03\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061007d5760003560e01c806354fd4d501161005b57806354fd4d50146100f857806375f0bb52146101415780639327136814610154578063e458779b1461016757600080fd5b806301ffc9a714610082578063186f0354146100aa5780634c205d0d146100ee575b600080fd5b610095610090366004610e47565b610195565b60405190151581526020015b60405180910390f35b60405173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016100a1565b6100f661022e565b005b6101346040518060400160405280600581526020017f312e312e3000000000000000000000000000000000000000000000000000000081525081565b6040516100a19190610ef4565b6100f661014f366004611057565b6103b3565b6100f661016236600461114a565b6107a1565b61018761017536600461117a565b60006020819052908152604090205481565b6040519081526020016100a1565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167fe6d7a83a00000000000000000000000000000000000000000000000000000000148061022857507fffffffff0000000000000000000000000000000000000000000000000000000082167f01ffc9a700000000000000000000000000000000000000000000000000000000145b92915050565b6040517f2f54bf6e0000000000000000000000000000000000000000000000000000000081523360048201527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1690632f54bf6e90602401602060405180830381865afa1580156102b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102dc9190611197565b61036d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4c6976656e65737347756172643a206f6e6c792053616665206f776e6572732060448201527f6d61792064656d6f6e737472617465206c6976656e657373000000000000000060648201526084015b60405180910390fd5b336000818152602081815260409182902042905590519182527f833bc129023866d52116d61e94b791eb8be46f05709362e0bcf1fe7c1a8c225c910160405180910390a1565b6103bb61096d565b60007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015610428573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261046e91908101906111b4565b905060005b81518110156104bc576104a982828151811061049157610491611266565b60200260200101516001610a3490919063ffffffff16565b50806104b4816112c4565b915050610473565b5060007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663d8d11f788e8e8e8e8e8e8e8e8e60017f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663affed0e06040518163ffffffff1660e01b8152600401602060405180830381865afa158015610571573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061059591906112fc565b61059f9190611315565b6040518b63ffffffff1660e01b81526004016105c49a9998979695949392919061132c565b602060405180830381865afa1580156105e1573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061060591906112fc565b905060007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663e75235b86040518163ffffffff1660e01b8152600401602060405180830381865afa158015610674573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061069891906112fc565b905060006106a7838784610a5d565b905060005b815181101561078f57426000808484815181106106cb576106cb611266565b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055507f833bc129023866d52116d61e94b791eb8be46f05709362e0bcf1fe7c1a8c225c82828151811061074457610744611266565b6020026020010151604051610775919073ffffffffffffffffffffffffffffffffffffffff91909116815260200190565b60405180910390a180610787816112c4565b9150506106ac565b50505050505050505050505050505050565b6107a961096d565b60007f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015610816573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261085c91908101906111b4565b905060005b81518110156108e157600082828151811061087e5761087e611266565b6020026020010151905061089c816001610c7a90919063ffffffff16565b15156000036108ce5773ffffffffffffffffffffffffffffffffffffffff811660009081526020819052604090204290555b50806108d9816112c4565b915050610861565b5060006108ee6001610c9c565b905060005b815181101561096657600082828151811061091057610910611266565b60209081029190910181015173ffffffffffffffffffffffffffffffffffffffff811660009081529182905260408220919091559050610951600182610c7a565b5050808061095e906112c4565b9150506108f3565b5050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610a32576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602f60248201527f4c6976656e65737347756172643a206f6e6c7920536166652063616e2063616c60448201527f6c20746869732066756e6374696f6e00000000000000000000000000000000006064820152608401610364565b565b6000610a568373ffffffffffffffffffffffffffffffffffffffff8416610ca9565b9392505050565b60608167ffffffffffffffff811115610a7857610a78610f3c565b604051908082528060200260200182016040528015610aa1578160200160208202803683370190505b50905060008060008060005b86811015610c6e576041818102890160208101516040820151919092015160ff16955090935091506000849003610ae9578260001c9450610c24565b8360ff16600103610aff578260001c9450610c24565b601e8460ff161115610bc4576040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018a9052600190605c0160405160208183030381529060405280519060200120600486610b6491906113f7565b6040805160008152602081018083529390935260ff90911690820152606081018590526080810184905260a0016020604051602081039080840390855afa158015610bb3573d6000803e3d6000fd5b505050602060405103519450610c24565b6040805160008152602081018083528b905260ff861691810191909152606081018490526080810183905260019060a0016020604051602081039080840390855afa158015610c17573d6000803e3d6000fd5b5050506020604051035194505b84868281518110610c3757610c37611266565b73ffffffffffffffffffffffffffffffffffffffff9092166020928302919091019091015280610c66816112c4565b915050610aad565b50505050509392505050565b6000610a568373ffffffffffffffffffffffffffffffffffffffff8416610cf8565b60606000610a5683610deb565b6000818152600183016020526040812054610cf057508154600181810184556000848152602080822090930184905584548482528286019093526040902091909155610228565b506000610228565b60008181526001830160205260408120548015610de1576000610d1c600183611315565b8554909150600090610d3090600190611315565b9050818114610d95576000866000018281548110610d5057610d50611266565b9060005260206000200154905080876000018481548110610d7357610d73611266565b6000918252602080832090910192909255918252600188019052604090208390555b8554869080610da657610da661141a565b600190038181906000526020600020016000905590558560010160008681526020019081526020016000206000905560019350505050610228565b6000915050610228565b606081600001805480602002602001604051908101604052809291908181526020018280548015610e3b57602002820191906000526020600020905b815481526020019060010190808311610e27575b50505050509050919050565b600060208284031215610e5957600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114610a5657600080fd5b6000815180845260005b81811015610eaf57602081850181015186830182015201610e93565b81811115610ec1576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b602081526000610a566020830184610e89565b73ffffffffffffffffffffffffffffffffffffffff81168114610f2957600080fd5b50565b8035610f3781610f07565b919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715610fb257610fb2610f3c565b604052919050565b600082601f830112610fcb57600080fd5b813567ffffffffffffffff811115610fe557610fe5610f3c565b61101660207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601610f6b565b81815284602083860101111561102b57600080fd5b816020850160208301376000918101602001919091529392505050565b803560028110610f3757600080fd5b60008060008060008060008060008060006101608c8e03121561107957600080fd5b6110828c610f2c565b9a5060208c0135995067ffffffffffffffff8060408e013511156110a557600080fd5b6110b58e60408f01358f01610fba565b99506110c360608e01611048565b985060808d0135975060a08d0135965060c08d013595506110e660e08e01610f2c565b94506110f56101008e01610f2c565b9350806101208e0135111561110957600080fd5b5061111b8d6101208e01358e01610fba565b915061112a6101408d01610f2c565b90509295989b509295989b9093969950565b8015158114610f2957600080fd5b6000806040838503121561115d57600080fd5b82359150602083013561116f8161113c565b809150509250929050565b60006020828403121561118c57600080fd5b8135610a5681610f07565b6000602082840312156111a957600080fd5b8151610a568161113c565b600060208083850312156111c757600080fd5b825167ffffffffffffffff808211156111df57600080fd5b818501915085601f8301126111f357600080fd5b81518181111561120557611205610f3c565b8060051b9150611216848301610f6b565b818152918301840191848101908884111561123057600080fd5b938501935b8385101561125a578451925061124a83610f07565b8282529385019390850190611235565b98975050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036112f5576112f5611295565b5060010190565b60006020828403121561130e57600080fd5b5051919050565b60008282101561132757611327611295565b500390565b600061014073ffffffffffffffffffffffffffffffffffffffff808e1684528c60208501528160408501526113638285018d610e89565b925060028b1061139c577f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b8a60608501528960808501528860a08501528760c085015280871660e085015250506113e161010083018573ffffffffffffffffffffffffffffffffffffffff169052565b826101208301529b9a5050505050505050505050565b600060ff821660ff84168082101561141157611411611295565b90039392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603160045260246000fdfea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0[W\x80cT\xFDMP\x14a\0\xF8W\x80cu\xF0\xBBR\x14a\x01AW\x80c\x93'\x13h\x14a\x01TW\x80c\xE4Xw\x9B\x14a\x01gW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\x82W\x80c\x18o\x03T\x14a\0\xAAW\x80cL ]\r\x14a\0\xEEW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x0EGV[a\x01\x95V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\0\xA1V[a\0\xF6a\x02.V[\0[a\x014`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.1.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xA1\x91\x90a\x0E\xF4V[a\0\xF6a\x01O6`\x04a\x10WV[a\x03\xB3V[a\0\xF6a\x01b6`\x04a\x11JV[a\x07\xA1V[a\x01\x87a\x01u6`\x04a\x11zV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xA1V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE6\xD7\xA8:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x02(WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x92\x91PPV[`@Q\x7F/T\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c/T\xBFn\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDC\x91\x90a\x11\x97V[a\x03mW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FLivenessGuard: only Safe owners `D\x82\x01R\x7Fmay demonstrate liveness\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 B\x90U\x90Q\x91\x82R\x7F\x83;\xC1)\x028f\xD5!\x16\xD6\x1E\x94\xB7\x91\xEB\x8B\xE4o\x05p\x93b\xE0\xBC\xF1\xFE|\x1A\x8C\"\\\x91\x01`@Q\x80\x91\x03\x90\xA1V[a\x03\xBBa\tmV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04n\x91\x90\x81\x01\x90a\x11\xB4V[\x90P`\0[\x81Q\x81\x10\x15a\x04\xBCWa\x04\xA9\x82\x82\x81Q\x81\x10a\x04\x91Wa\x04\x91a\x12fV[` \x02` \x01\x01Q`\x01a\n4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80a\x04\xB4\x81a\x12\xC4V[\x91PPa\x04sV[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAF\xFE\xD0\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x95\x91\x90a\x12\xFCV[a\x05\x9F\x91\x90a\x13\x15V[`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xC4\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x13,V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x05\x91\x90a\x12\xFCV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x98\x91\x90a\x12\xFCV[\x90P`\0a\x06\xA7\x83\x87\x84a\n]V[\x90P`\0[\x81Q\x81\x10\x15a\x07\x8FWB`\0\x80\x84\x84\x81Q\x81\x10a\x06\xCBWa\x06\xCBa\x12fV[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F\x83;\xC1)\x028f\xD5!\x16\xD6\x1E\x94\xB7\x91\xEB\x8B\xE4o\x05p\x93b\xE0\xBC\xF1\xFE|\x1A\x8C\"\\\x82\x82\x81Q\x81\x10a\x07DWa\x07Da\x12fV[` \x02` \x01\x01Q`@Qa\x07u\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x80a\x07\x87\x81a\x12\xC4V[\x91PPa\x06\xACV[PPPPPPPPPPPPPPPPV[a\x07\xA9a\tmV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\\\x91\x90\x81\x01\x90a\x11\xB4V[\x90P`\0[\x81Q\x81\x10\x15a\x08\xE1W`\0\x82\x82\x81Q\x81\x10a\x08~Wa\x08~a\x12fV[` \x02` \x01\x01Q\x90Pa\x08\x9C\x81`\x01a\x0Cz\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x15\x15`\0\x03a\x08\xCEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 B\x90U[P\x80a\x08\xD9\x81a\x12\xC4V[\x91PPa\x08aV[P`\0a\x08\xEE`\x01a\x0C\x9CV[\x90P`\0[\x81Q\x81\x10\x15a\tfW`\0\x82\x82\x81Q\x81\x10a\t\x10Wa\t\x10a\x12fV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x91\x82\x90R`@\x82 \x91\x90\x91U\x90Pa\tQ`\x01\x82a\x0CzV[PP\x80\x80a\t^\x90a\x12\xC4V[\x91PPa\x08\xF3V[PPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FLivenessGuard: only Safe can cal`D\x82\x01R\x7Fl this function\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03dV[V[`\0a\nV\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x0C\xA9V[\x93\x92PPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nxWa\nxa\x0F<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xA1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\0\x80`\0[\x86\x81\x10\x15a\x0CnW`A\x81\x81\x02\x89\x01` \x81\x01Q`@\x82\x01Q\x91\x90\x92\x01Q`\xFF\x16\x95P\x90\x93P\x91P`\0\x84\x90\x03a\n\xE9W\x82`\0\x1C\x94Pa\x0C$V[\x83`\xFF\x16`\x01\x03a\n\xFFW\x82`\0\x1C\x94Pa\x0C$V[`\x1E\x84`\xFF\x16\x11\x15a\x0B\xC4W`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x8A\x90R`\x01\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04\x86a\x0Bd\x91\x90a\x13\xF7V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x93\x90\x93R`\xFF\x90\x91\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\xB3W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94Pa\x0C$V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x86\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0C\x17W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94P[\x84\x86\x82\x81Q\x81\x10a\x0C7Wa\x0C7a\x12fV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0Cf\x81a\x12\xC4V[\x91PPa\n\xADV[PPPPP\x93\x92PPPV[`\0a\nV\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x0C\xF8V[```\0a\nV\x83a\r\xEBV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x0C\xF0WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02(V[P`\0a\x02(V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\r\xE1W`\0a\r\x1C`\x01\x83a\x13\x15V[\x85T\x90\x91P`\0\x90a\r0\x90`\x01\x90a\x13\x15V[\x90P\x81\x81\x14a\r\x95W`\0\x86`\0\x01\x82\x81T\x81\x10a\rPWa\rPa\x12fV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\rsWa\rsa\x12fV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\r\xA6Wa\r\xA6a\x14\x1AV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02(V[`\0\x91PPa\x02(V[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E;W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E'W[PPPPP\x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0EYW`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\nVW`\0\x80\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0E\xAFW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0E\x93V[\x81\x81\x11\x15a\x0E\xC1W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\nV` \x83\x01\x84a\x0E\x89V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F)W`\0\x80\xFD[PV[\x805a\x0F7\x81a\x0F\x07V[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xB2Wa\x0F\xB2a\x0F<V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0F\xCBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xE5Wa\x0F\xE5a\x0F<V[a\x10\x16` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x0FkV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x10+W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x02\x81\x10a\x0F7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a\x10yW`\0\x80\xFD[a\x10\x82\x8Ca\x0F,V[\x9AP` \x8C\x015\x99Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@\x8E\x015\x11\x15a\x10\xA5W`\0\x80\xFD[a\x10\xB5\x8E`@\x8F\x015\x8F\x01a\x0F\xBAV[\x99Pa\x10\xC3``\x8E\x01a\x10HV[\x98P`\x80\x8D\x015\x97P`\xA0\x8D\x015\x96P`\xC0\x8D\x015\x95Pa\x10\xE6`\xE0\x8E\x01a\x0F,V[\x94Pa\x10\xF5a\x01\0\x8E\x01a\x0F,V[\x93P\x80a\x01 \x8E\x015\x11\x15a\x11\tW`\0\x80\xFD[Pa\x11\x1B\x8Da\x01 \x8E\x015\x8E\x01a\x0F\xBAV[\x91Pa\x11*a\x01@\x8D\x01a\x0F,V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[\x80\x15\x15\x81\x14a\x0F)W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x11]W`\0\x80\xFD[\x825\x91P` \x83\x015a\x11o\x81a\x11<V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\x8CW`\0\x80\xFD[\x815a\nV\x81a\x0F\x07V[`\0` \x82\x84\x03\x12\x15a\x11\xA9W`\0\x80\xFD[\x81Qa\nV\x81a\x11<V[`\0` \x80\x83\x85\x03\x12\x15a\x11\xC7W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\xDFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x11\xF3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x12\x05Wa\x12\x05a\x0F<V[\x80`\x05\x1B\x91Pa\x12\x16\x84\x83\x01a\x0FkV[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\x120W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x12ZW\x84Q\x92Pa\x12J\x83a\x0F\x07V[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x125V[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x12\xF5Wa\x12\xF5a\x12\x95V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x13\x0EW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a\x13'Wa\x13'a\x12\x95V[P\x03\x90V[`\0a\x01@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8E\x16\x84R\x8C` \x85\x01R\x81`@\x85\x01Ra\x13c\x82\x85\x01\x8Da\x0E\x89V[\x92P`\x02\x8B\x10a\x13\x9CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x8A``\x85\x01R\x89`\x80\x85\x01R\x88`\xA0\x85\x01R\x87`\xC0\x85\x01R\x80\x87\x16`\xE0\x85\x01RPPa\x13\xE1a\x01\0\x83\x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x82a\x01 \x83\x01R\x9B\x9APPPPPPPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a\x14\x11Wa\x14\x11a\x12\x95V[\x90\x03\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnerRecorded(address)` and selector `0x833bc129023866d52116d61e94b791eb8be46f05709362e0bcf1fe7c1a8c225c`.
```solidity
event OwnerRecorded(address owner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnerRecorded {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnerRecorded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OwnerRecorded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                131u8, 59u8, 193u8, 41u8, 2u8, 56u8, 102u8, 213u8, 33u8, 22u8, 214u8,
                30u8, 148u8, 183u8, 145u8, 235u8, 139u8, 228u8, 111u8, 5u8, 112u8, 147u8,
                98u8, 224u8, 188u8, 241u8, 254u8, 124u8, 26u8, 140u8, 34u8, 92u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { owner: data.0 }
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
                        &self.owner,
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
        impl alloy_sol_types::private::IntoLogData for OwnerRecorded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnerRecorded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnerRecorded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _safe);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                        &self._safe,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkAfterExecution(bytes32,bool)` and selector `0x93271368`.
```solidity
function checkAfterExecution(bytes32, bool) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAfterExecutionCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _1: bool,
    }
    ///Container type for the return parameters of the [`checkAfterExecution(bytes32,bool)`](checkAfterExecutionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAfterExecutionReturn {}
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
            impl ::core::convert::From<checkAfterExecutionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAfterExecutionCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAfterExecutionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<checkAfterExecutionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAfterExecutionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAfterExecutionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkAfterExecutionReturn {
            fn _tokenize(
                &self,
            ) -> <checkAfterExecutionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkAfterExecutionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkAfterExecutionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkAfterExecution(bytes32,bool)";
            const SELECTOR: [u8; 4] = [147u8, 39u8, 19u8, 104u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkAfterExecutionReturn::_tokenize(ret)
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
    /**Function with signature `checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)` and selector `0x75f0bb52`.
```solidity
function checkTransaction(address _to, uint256 _value, bytes memory _data, Enum.Operation _operation, uint256 _safeTxGas, uint256 _baseGas, uint256 _gasPrice, address _gasToken, address _refundReceiver, bytes memory _signatures, address _msgSender) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkTransactionCall {
        #[allow(missing_docs)]
        pub _to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _safeTxGas: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _baseGas: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _gasPrice: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _gasToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _refundReceiver: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _signatures: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _msgSender: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)`](checkTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkTransactionReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<checkTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkTransactionCall) -> Self {
                    (
                        value._to,
                        value._value,
                        value._data,
                        value._operation,
                        value._safeTxGas,
                        value._baseGas,
                        value._gasPrice,
                        value._gasToken,
                        value._refundReceiver,
                        value._signatures,
                        value._msgSender,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _to: tuple.0,
                        _value: tuple.1,
                        _data: tuple.2,
                        _operation: tuple.3,
                        _safeTxGas: tuple.4,
                        _baseGas: tuple.5,
                        _gasPrice: tuple.6,
                        _gasToken: tuple.7,
                        _refundReceiver: tuple.8,
                        _signatures: tuple.9,
                        _msgSender: tuple.10,
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
            impl ::core::convert::From<checkTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkTransactionReturn {
            fn _tokenize(
                &self,
            ) -> <checkTransactionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkTransactionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)";
            const SELECTOR: [u8; 4] = [117u8, 240u8, 187u8, 82u8];
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
                        &self._to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self._operation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._safeTxGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._baseGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._gasPrice),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._gasToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._refundReceiver,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatures,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._msgSender,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkTransactionReturn::_tokenize(ret)
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
    /**Function with signature `lastLive(address)` and selector `0xe458779b`.
```solidity
function lastLive(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastLiveCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastLive(address)`](lastLiveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastLiveReturn {
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
            impl ::core::convert::From<lastLiveCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastLiveCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastLiveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<lastLiveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastLiveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastLiveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastLiveCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastLive(address)";
            const SELECTOR: [u8; 4] = [228u8, 88u8, 119u8, 155u8];
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
                        &self.0,
                    ),
                )
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
                        let r: lastLiveReturn = r.into();
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
                        let r: lastLiveReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `safe()` and selector `0x186f0354`.
```solidity
function safe() external view returns (address safe_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`safe()`](safeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeReturn {
        #[allow(missing_docs)]
        pub safe_: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<safeCall> for UnderlyingRustTuple<'_> {
                fn from(value: safeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeCall {
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
            impl ::core::convert::From<safeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: safeReturn) -> Self {
                    (value.safe_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { safe_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for safeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "safe()";
            const SELECTOR: [u8; 4] = [24u8, 111u8, 3u8, 84u8];
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
                        let r: safeReturn = r.into();
                        r.safe_
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
                        let r: safeReturn = r.into();
                        r.safe_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `showLiveness()` and selector `0x4c205d0d`.
```solidity
function showLiveness() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct showLivenessCall;
    ///Container type for the return parameters of the [`showLiveness()`](showLivenessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct showLivenessReturn {}
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
            impl ::core::convert::From<showLivenessCall> for UnderlyingRustTuple<'_> {
                fn from(value: showLivenessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for showLivenessCall {
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
            impl ::core::convert::From<showLivenessReturn> for UnderlyingRustTuple<'_> {
                fn from(value: showLivenessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for showLivenessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl showLivenessReturn {
            fn _tokenize(
                &self,
            ) -> <showLivenessCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for showLivenessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = showLivenessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "showLiveness()";
            const SELECTOR: [u8; 4] = [76u8, 32u8, 93u8, 13u8];
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
                showLivenessReturn::_tokenize(ret)
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
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
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
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
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
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
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
                        let r: supportsInterfaceReturn = r.into();
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
                        let r: supportsInterfaceReturn = r.into();
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
    ///Container for all the [`LivenessGuard`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum LivenessGuardCalls {
        #[allow(missing_docs)]
        checkAfterExecution(checkAfterExecutionCall),
        #[allow(missing_docs)]
        checkTransaction(checkTransactionCall),
        #[allow(missing_docs)]
        lastLive(lastLiveCall),
        #[allow(missing_docs)]
        safe(safeCall),
        #[allow(missing_docs)]
        showLiveness(showLivenessCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl LivenessGuardCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [24u8, 111u8, 3u8, 84u8],
            [76u8, 32u8, 93u8, 13u8],
            [84u8, 253u8, 77u8, 80u8],
            [117u8, 240u8, 187u8, 82u8],
            [147u8, 39u8, 19u8, 104u8],
            [228u8, 88u8, 119u8, 155u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(safe),
            ::core::stringify!(showLiveness),
            ::core::stringify!(version),
            ::core::stringify!(checkTransaction),
            ::core::stringify!(checkAfterExecution),
            ::core::stringify!(lastLive),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <safeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <showLivenessCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkAfterExecutionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lastLiveCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for LivenessGuardCalls {
        const NAME: &'static str = "LivenessGuardCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 7usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::checkAfterExecution(_) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkTransaction(_) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastLive(_) => <lastLiveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::safe(_) => <safeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::showLiveness(_) => {
                    <showLivenessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<LivenessGuardCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(LivenessGuardCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn safe(data: &[u8]) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <safeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(LivenessGuardCalls::safe)
                    }
                    safe
                },
                {
                    fn showLiveness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <showLivenessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(LivenessGuardCalls::showLiveness)
                    }
                    showLiveness
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(LivenessGuardCalls::version)
                    }
                    version
                },
                {
                    fn checkTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <checkTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(LivenessGuardCalls::checkTransaction)
                    }
                    checkTransaction
                },
                {
                    fn checkAfterExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(LivenessGuardCalls::checkAfterExecution)
                    }
                    checkAfterExecution
                },
                {
                    fn lastLive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <lastLiveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(LivenessGuardCalls::lastLive)
                    }
                    lastLive
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
            ) -> alloy_sol_types::Result<LivenessGuardCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(LivenessGuardCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn safe(data: &[u8]) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <safeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(LivenessGuardCalls::safe)
                    }
                    safe
                },
                {
                    fn showLiveness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <showLivenessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(LivenessGuardCalls::showLiveness)
                    }
                    showLiveness
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(LivenessGuardCalls::version)
                    }
                    version
                },
                {
                    fn checkTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <checkTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(LivenessGuardCalls::checkTransaction)
                    }
                    checkTransaction
                },
                {
                    fn checkAfterExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(LivenessGuardCalls::checkAfterExecution)
                    }
                    checkAfterExecution
                },
                {
                    fn lastLive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<LivenessGuardCalls> {
                        <lastLiveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(LivenessGuardCalls::lastLive)
                    }
                    lastLive
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
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastLive(inner) => {
                    <lastLiveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::showLiveness(inner) => {
                    <showLivenessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastLive(inner) => {
                    <lastLiveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::showLiveness(inner) => {
                    <showLivenessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`LivenessGuard`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum LivenessGuardEvents {
        #[allow(missing_docs)]
        OwnerRecorded(OwnerRecorded),
    }
    impl LivenessGuardEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                131u8, 59u8, 193u8, 41u8, 2u8, 56u8, 102u8, 213u8, 33u8, 22u8, 214u8,
                30u8, 148u8, 183u8, 145u8, 235u8, 139u8, 228u8, 111u8, 5u8, 112u8, 147u8,
                98u8, 224u8, 188u8, 241u8, 254u8, 124u8, 26u8, 140u8, 34u8, 92u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(OwnerRecorded),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <OwnerRecorded as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for LivenessGuardEvents {
        const NAME: &'static str = "LivenessGuardEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<OwnerRecorded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnerRecorded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnerRecorded)
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
    impl alloy_sol_types::private::IntoLogData for LivenessGuardEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OwnerRecorded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OwnerRecorded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`LivenessGuard`](self) contract instance.

See the [wrapper's documentation](`LivenessGuardInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> LivenessGuardInstance<P, N> {
        LivenessGuardInstance::<P, N>::new(address, __provider)
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
        _safe: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<LivenessGuardInstance<P, N>>,
    > {
        LivenessGuardInstance::<P, N>::deploy(__provider, _safe)
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
        _safe: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        LivenessGuardInstance::<P, N>::deploy_builder(__provider, _safe)
    }
    /**A [`LivenessGuard`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LivenessGuard`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LivenessGuardInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for LivenessGuardInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LivenessGuardInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > LivenessGuardInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`LivenessGuard`](self) contract instance.

See the [wrapper's documentation](`LivenessGuardInstance`) for more details.*/
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
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<LivenessGuardInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, _safe);
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
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _safe },
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
    impl<P: ::core::clone::Clone, N> LivenessGuardInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LivenessGuardInstance<P, N> {
            LivenessGuardInstance {
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
    > LivenessGuardInstance<P, N> {
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
        ///Creates a new call builder for the [`checkAfterExecution`] function.
        pub fn checkAfterExecution(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: bool,
        ) -> alloy_contract::SolCallBuilder<&P, checkAfterExecutionCall, N> {
            self.call_builder(&checkAfterExecutionCall { _0, _1 })
        }
        ///Creates a new call builder for the [`checkTransaction`] function.
        pub fn checkTransaction(
            &self,
            _to: alloy::sol_types::private::Address,
            _value: alloy::sol_types::private::primitives::aliases::U256,
            _data: alloy::sol_types::private::Bytes,
            _operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
            _safeTxGas: alloy::sol_types::private::primitives::aliases::U256,
            _baseGas: alloy::sol_types::private::primitives::aliases::U256,
            _gasPrice: alloy::sol_types::private::primitives::aliases::U256,
            _gasToken: alloy::sol_types::private::Address,
            _refundReceiver: alloy::sol_types::private::Address,
            _signatures: alloy::sol_types::private::Bytes,
            _msgSender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, checkTransactionCall, N> {
            self.call_builder(
                &checkTransactionCall {
                    _to,
                    _value,
                    _data,
                    _operation,
                    _safeTxGas,
                    _baseGas,
                    _gasPrice,
                    _gasToken,
                    _refundReceiver,
                    _signatures,
                    _msgSender,
                },
            )
        }
        ///Creates a new call builder for the [`lastLive`] function.
        pub fn lastLive(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, lastLiveCall, N> {
            self.call_builder(&lastLiveCall(_0))
        }
        ///Creates a new call builder for the [`safe`] function.
        pub fn safe(&self) -> alloy_contract::SolCallBuilder<&P, safeCall, N> {
            self.call_builder(&safeCall)
        }
        ///Creates a new call builder for the [`showLiveness`] function.
        pub fn showLiveness(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, showLivenessCall, N> {
            self.call_builder(&showLivenessCall)
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<&P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
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
    > LivenessGuardInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`OwnerRecorded`] event.
        pub fn OwnerRecorded_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnerRecorded, N> {
            self.event_filter::<OwnerRecorded>()
        }
    }
}

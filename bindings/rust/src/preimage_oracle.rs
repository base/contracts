///Module containing a contract's types and functions.
/**

```solidity
library LibKeccak {
    struct StateMatrix { uint64[25] state; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod LibKeccak {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct StateMatrix { uint64[25] state; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StateMatrix {
        #[allow(missing_docs)]
        pub state: [u64; 25usize],
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
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<64>,
                25usize,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ([u64; 25usize],);
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
        impl ::core::convert::From<StateMatrix> for UnderlyingRustTuple<'_> {
            fn from(value: StateMatrix) -> Self {
                (value.state,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StateMatrix {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { state: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StateMatrix {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StateMatrix {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        25usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.state),
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
        impl alloy_sol_types::SolType for StateMatrix {
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
        impl alloy_sol_types::SolStruct for StateMatrix {
            const NAME: &'static str = "StateMatrix";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("StateMatrix(uint64[25] state)")
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
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<64>,
                    25usize,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.state)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StateMatrix {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        25usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.state)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<64>,
                    25usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.state,
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
    /**Creates a new wrapper around an on-chain [`LibKeccak`](self) contract instance.

See the [wrapper's documentation](`LibKeccakInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> LibKeccakInstance<P, N> {
        LibKeccakInstance::<P, N>::new(address, __provider)
    }
    /**A [`LibKeccak`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LibKeccak`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LibKeccakInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for LibKeccakInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LibKeccakInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > LibKeccakInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`LibKeccak`](self) contract instance.

See the [wrapper's documentation](`LibKeccakInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> LibKeccakInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LibKeccakInstance<P, N> {
            LibKeccakInstance {
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
    > LibKeccakInstance<P, N> {
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
    > LibKeccakInstance<P, N> {
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
library LibKeccak {
    struct StateMatrix {
        uint64[25] state;
    }
}

interface PreimageOracle {
    type LPPMetaData is bytes32;
    struct Leaf {
        bytes input;
        uint256 index;
        bytes32 stateCommitment;
    }

    error ActiveProposal();
    error AlreadyFinalized();
    error AlreadyInitialized();
    error BadProposal();
    error BondTransferFailed();
    error InsufficientBond();
    error InvalidInputSize();
    error InvalidPreimage();
    error InvalidProof();
    error NotEOA();
    error NotInitialized();
    error PartOffsetOOB();
    error PostStateMatches();
    error StatesNotContiguous();
    error TreeSizeOverflow();
    error WrongStartingBlock();

    constructor(uint256 _minProposalSize, uint256 _challengePeriod);

    function KECCAK_TREE_DEPTH() external view returns (uint256);
    function MAX_LEAF_COUNT() external view returns (uint256);
    function MIN_BOND_SIZE() external view returns (uint256);
    function PRECOMPILE_CALL_RESERVED_GAS() external view returns (uint256);
    function addLeavesLPP(uint256 _uuid, uint256 _inputStartBlock, bytes memory _input, bytes32[] memory _stateCommitments, bool _finalize) external;
    function challengeFirstLPP(address _claimant, uint256 _uuid, Leaf memory _postState, bytes32[] memory _postStateProof) external;
    function challengeLPP(address _claimant, uint256 _uuid, LibKeccak.StateMatrix memory _stateMatrix, Leaf memory _preState, bytes32[] memory _preStateProof, Leaf memory _postState, bytes32[] memory _postStateProof) external;
    function challengePeriod() external view returns (uint256 challengePeriod_);
    function getTreeRootLPP(address _owner, uint256 _uuid) external view returns (bytes32 treeRoot_);
    function initLPP(uint256 _uuid, uint32 _partOffset, uint32 _claimedSize) external payable;
    function loadBlobPreimagePart(uint256 _z, uint256 _y, bytes memory _commitment, bytes memory _proof, uint256 _partOffset) external;
    function loadKeccak256PreimagePart(uint256 _partOffset, bytes memory _preimage) external;
    function loadLocalData(uint256 _ident, bytes32 _localContext, bytes32 _word, uint256 _size, uint256 _partOffset) external returns (bytes32 key_);
    function loadPrecompilePreimagePart(uint256 _partOffset, address _precompile, uint64 _requiredGas, bytes memory _input) external;
    function loadSha256PreimagePart(uint256 _partOffset, bytes memory _preimage) external;
    function minProposalSize() external view returns (uint256 minProposalSize_);
    function preimageLengths(bytes32) external view returns (uint256);
    function preimagePartOk(bytes32, uint256) external view returns (bool);
    function preimageParts(bytes32, uint256) external view returns (bytes32);
    function proposalBlocks(address, uint256, uint256) external view returns (uint64);
    function proposalBlocksLen(address _claimant, uint256 _uuid) external view returns (uint256 len_);
    function proposalBonds(address, uint256) external view returns (uint256);
    function proposalBranches(address, uint256, uint256) external view returns (bytes32);
    function proposalCount() external view returns (uint256 count_);
    function proposalMetadata(address, uint256) external view returns (LPPMetaData);
    function proposalParts(address, uint256) external view returns (bytes32);
    function proposals(uint256) external view returns (address claimant, uint256 uuid);
    function readPreimage(bytes32 _key, uint256 _offset) external view returns (bytes32 dat_, uint256 datLen_);
    function squeezeLPP(address _claimant, uint256 _uuid, LibKeccak.StateMatrix memory _stateMatrix, Leaf memory _preState, bytes32[] memory _preStateProof, Leaf memory _postState, bytes32[] memory _postStateProof) external;
    function version() external view returns (string memory);
    function zeroHashes(uint256) external view returns (bytes32);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_minProposalSize",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_challengePeriod",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "KECCAK_TREE_DEPTH",
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
    "name": "MAX_LEAF_COUNT",
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
    "name": "MIN_BOND_SIZE",
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
    "name": "PRECOMPILE_CALL_RESERVED_GAS",
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
    "name": "addLeavesLPP",
    "inputs": [
      {
        "name": "_uuid",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_inputStartBlock",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_input",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_stateCommitments",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "_finalize",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "challengeFirstLPP",
    "inputs": [
      {
        "name": "_claimant",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_uuid",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_postState",
        "type": "tuple",
        "internalType": "struct PreimageOracle.Leaf",
        "components": [
          {
            "name": "input",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "index",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "stateCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "_postStateProof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "challengeLPP",
    "inputs": [
      {
        "name": "_claimant",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_uuid",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_stateMatrix",
        "type": "tuple",
        "internalType": "struct LibKeccak.StateMatrix",
        "components": [
          {
            "name": "state",
            "type": "uint64[25]",
            "internalType": "uint64[25]"
          }
        ]
      },
      {
        "name": "_preState",
        "type": "tuple",
        "internalType": "struct PreimageOracle.Leaf",
        "components": [
          {
            "name": "input",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "index",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "stateCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "_preStateProof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "_postState",
        "type": "tuple",
        "internalType": "struct PreimageOracle.Leaf",
        "components": [
          {
            "name": "input",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "index",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "stateCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "_postStateProof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "challengePeriod",
    "inputs": [],
    "outputs": [
      {
        "name": "challengePeriod_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTreeRootLPP",
    "inputs": [
      {
        "name": "_owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_uuid",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "treeRoot_",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initLPP",
    "inputs": [
      {
        "name": "_uuid",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_partOffset",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_claimedSize",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "loadBlobPreimagePart",
    "inputs": [
      {
        "name": "_z",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_y",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_commitment",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_proof",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_partOffset",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "loadKeccak256PreimagePart",
    "inputs": [
      {
        "name": "_partOffset",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_preimage",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "loadLocalData",
    "inputs": [
      {
        "name": "_ident",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_localContext",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_word",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_size",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_partOffset",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "key_",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "loadPrecompilePreimagePart",
    "inputs": [
      {
        "name": "_partOffset",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_precompile",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_requiredGas",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_input",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "loadSha256PreimagePart",
    "inputs": [
      {
        "name": "_partOffset",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_preimage",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "minProposalSize",
    "inputs": [],
    "outputs": [
      {
        "name": "minProposalSize_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "preimageLengths",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "preimagePartOk",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "preimageParts",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "proposalBlocks",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "proposalBlocksLen",
    "inputs": [
      {
        "name": "_claimant",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_uuid",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "len_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalBonds",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "proposalBranches",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "proposalCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalMetadata",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "LPPMetaData"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalParts",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "proposals",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "claimant",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "uuid",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "readPreimage",
    "inputs": [
      {
        "name": "_key",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_offset",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "dat_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "datLen_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "squeezeLPP",
    "inputs": [
      {
        "name": "_claimant",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_uuid",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_stateMatrix",
        "type": "tuple",
        "internalType": "struct LibKeccak.StateMatrix",
        "components": [
          {
            "name": "state",
            "type": "uint64[25]",
            "internalType": "uint64[25]"
          }
        ]
      },
      {
        "name": "_preState",
        "type": "tuple",
        "internalType": "struct PreimageOracle.Leaf",
        "components": [
          {
            "name": "input",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "index",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "stateCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "_preStateProof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "_postState",
        "type": "tuple",
        "internalType": "struct PreimageOracle.Leaf",
        "components": [
          {
            "name": "input",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "index",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "stateCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "_postStateProof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "type": "function",
    "name": "zeroHashes",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "type": "error",
    "name": "ActiveProposal",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AlreadyFinalized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AlreadyInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BadProposal",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BondTransferFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientBond",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInputSize",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPreimage",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotEOA",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PartOffsetOOB",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PostStateMatches",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StatesNotContiguous",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TreeSizeOverflow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WrongStartingBlock",
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
pub mod PreimageOracle {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c06040523480156200001157600080fd5b50604051620039ec380380620039ec833981016040819052620000349162000162565b60a082905260808190526001600160401b03811115620000ad5760405162461bcd60e51b815260206004820152602a60248201527f507265696d6167654f7261636c653a206368616c6c656e676520706572696f6460448201526920746f6f206c6172676560b01b606482015260840160405180910390fd5b60005b620000be600160106200019d565b811015620001595760038160108110620000dc57620000dc620001b7565b015460038260108110620000f457620000f4620001b7565b01546040805160208101939093528201526060016040516020818303038152906040528051906020012060038260016200012f9190620001cd565b60108110620001425762000142620001b7565b0155806200015081620001e8565b915050620000b0565b50505062000204565b600080604083850312156200017657600080fd5b505080516020909101519092909150565b634e487b7160e01b600052601160045260246000fd5b600082821015620001b257620001b262000187565b500390565b634e487b7160e01b600052603260045260246000fd5b60008219821115620001e357620001e362000187565b500190565b600060018201620001fd57620001fd62000187565b5060010190565b60805160a0516137b462000238600039600081816105b00152611dea0152600081816106b001526114f301526137b46000f3fe6080604052600436106101d85760003560e01c80639d53a64811610102578063ddcd58de11610095578063ec5efcbc11610064578063ec5efcbc14610681578063f3f480d9146106a1578063faf37bc7146106d4578063fef2b4ed146106e757600080fd5b8063ddcd58de146105d4578063e03110e11461060c578063e159261114610641578063ea7139501461066157600080fd5b8063b5e7154c116100d1578063b5e7154c14610555578063d18534b51461056c578063da35c6641461058c578063dd24f9bf146105a157600080fd5b80639d53a6481461048e5780639d7e8769146104dd578063b2e67ba8146104fd578063b4801e611461053557600080fd5b806361238bde1161017a5780637ac54767116101495780637ac54767146103ca5780638542cf50146103ea578063882856ef146104355780638dc4be111461046e57600080fd5b806361238bde1461031e5780636551927b146103565780637051472e1461038e5780637917de1d146103aa57600080fd5b80633909af5c116101b65780633909af5c146102715780634d52b4c91461029357806352f0f3ad146102a857806354fd4d50146102c857600080fd5b8063013cf08b146101dd5780630359a5631461022e5780632055b36b1461025c575b600080fd5b3480156101e957600080fd5b506101fd6101f8366004612ca2565b610714565b6040805173ffffffffffffffffffffffffffffffffffffffff90931683526020830191909152015b60405180910390f35b34801561023a57600080fd5b5061024e610249366004612ce4565b610759565b604051908152602001610225565b34801561026857600080fd5b5061024e601081565b34801561027d57600080fd5b5061029161028c366004612eec565b610891565b005b34801561029f57600080fd5b5061024e610ae8565b3480156102b457600080fd5b5061024e6102c3366004612fd8565b610b03565b3480156102d457600080fd5b506103116040518060400160405280600581526020017f312e312e3400000000000000000000000000000000000000000000000000000081525081565b604051610225919061303f565b34801561032a57600080fd5b5061024e610339366004613090565b600160209081526000928352604080842090915290825290205481565b34801561036257600080fd5b5061024e610371366004612ce4565b601560209081526000928352604080842090915290825290205481565b34801561039a57600080fd5b5061024e6703782dace9d9000081565b3480156103b657600080fd5b506102916103c53660046130f4565b610bd9565b3480156103d657600080fd5b5061024e6103e5366004612ca2565b6110dc565b3480156103f657600080fd5b50610425610405366004613090565b600260209081526000928352604080842090915290825290205460ff1681565b6040519015158152602001610225565b34801561044157600080fd5b50610455610450366004613190565b6110f3565b60405167ffffffffffffffff9091168152602001610225565b34801561047a57600080fd5b506102916104893660046131c3565b61114d565b34801561049a57600080fd5b5061024e6104a9366004612ce4565b73ffffffffffffffffffffffffffffffffffffffff9091166000908152601860209081526040808320938352929052205490565b3480156104e957600080fd5b506102916104f836600461320f565b611248565b34801561050957600080fd5b5061024e610518366004612ce4565b601760209081526000928352604080842090915290825290205481565b34801561054157600080fd5b5061024e610550366004613190565b6113ff565b34801561056157600080fd5b5061024e620186a081565b34801561057857600080fd5b50610291610587366004612eec565b611431565b34801561059857600080fd5b5060135461024e565b3480156105ad57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061024e565b3480156105e057600080fd5b5061024e6105ef366004612ce4565b601660209081526000928352604080842090915290825290205481565b34801561061857600080fd5b5061062c610627366004613090565b611840565b60408051928352602083019190915201610225565b34801561064d57600080fd5b5061029161065c3660046131c3565b611931565b34801561066d57600080fd5b5061029161067c36600461329b565b611a39565b34801561068d57600080fd5b5061029161069c36600461330a565b611b98565b3480156106ad57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061024e565b6102916106e2366004613392565b611d1e565b3480156106f357600080fd5b5061024e610702366004612ca2565b60006020819052908152604090205481565b6013818154811061072457600080fd5b60009182526020909120600290910201805460019091015473ffffffffffffffffffffffffffffffffffffffff909116915082565b73ffffffffffffffffffffffffffffffffffffffff82166000908152601560209081526040808320848452909152812054819061079c9060601c63ffffffff1690565b63ffffffff16905060005b6010811015610889578160011660010361082f5773ffffffffffffffffffffffffffffffffffffffff85166000908152601460209081526040808320878452909152902081601081106107fc576107fc6133ce565b01546040805160208101929092528101849052606001604051602081830303815290604052805190602001209250610870565b8260038260108110610843576108436133ce565b01546040805160208101939093528201526060016040516020818303038152906040528051906020012092505b60019190911c90806108818161342c565b9150506107a7565b505092915050565b600061089d8a8a610759565b90506108c086868360208b01356108bb6108b68d613464565b611fea565b61202a565b80156108de57506108de83838360208801356108bb6108b68a613464565b610914576040517f09bde33900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b86604001358860405160200161092a9190613533565b6040516020818303038152906040528051906020012014610977576040517f1968a90200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b83602001358760200135600161098d9190613571565b146109c4576040517f9a3b119900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a0c886109d28680613589565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061208b92505050565b610a15886121e6565b836040013588604051602001610a2b9190613533565b6040516020818303038152906040528051906020012003610a78576040517f9843145b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8a1660009081526015602090815260408083208c8452909152902080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000166001179055610adc8a8a33612800565b50505050505050505050565b6001610af660106002613710565b610b00919061371c565b81565b6000610b0f86866128c0565b9050610b1c836008613571565b82101580610b2a5750602083115b15610b61576040517ffe25498700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000602081815260c085901b82526008959095528251828252600286526040808320858452875280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660019081179091558484528752808320948352938652838220558181529384905292205592915050565b60608115610bf257610beb868661296d565b9050610c2c565b85858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509293505050505b3360009081526014602090815260408083208b845290915280822081516102008101928390529160109082845b815481526020019060010190808311610c5957505050505090506000601560003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008b81526020019081526020016000205490506000610cda8260601c63ffffffff1690565b63ffffffff169050333214610d1b576040517fba092d1600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d2b8260801c63ffffffff1690565b63ffffffff16600003610d6a576040517f87138d5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d748260c01c90565b67ffffffffffffffff1615610db5576040517f475a253500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b898114610dee576040517f60f95d5a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610dfb89898d88866129e6565b83516020850160888204881415608883061715610e20576307b1daf16000526004601cfd5b60405160c8810160405260005b83811015610ed0578083018051835260208101516020840152604081015160408401526060810151606084015260808101516080840152508460888301526088810460051b8b013560a883015260c882206001860195508560005b610200811015610ec5576001821615610ea55782818b0152610ec5565b8981015160009081526020938452604090209260019290921c9101610e88565b505050608801610e2d565b50505050600160106002610ee49190613710565b610eee919061371c565b811115610f27576040517f6229572300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610f9c610f3a8360401c63ffffffff1690565b610f4a9063ffffffff168a613571565b60401b7fffffffffffffffffffffffffffffffffffffffff00000000ffffffffffffffff606084901b167fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff8516171790565b915084156110295777ffffffffffffffffffffffffffffffffffffffffffffffff82164260c01b179150610fd68260801c63ffffffff1690565b63ffffffff16610fec8360401c63ffffffff1690565b63ffffffff1614611029576040517f7b1dafd100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3360009081526014602090815260408083208e8452909152902061104f90846010612c18565b503360008181526018602090815260408083208f8452825280832080546001810182559084528284206004820401805460039092166008026101000a67ffffffffffffffff818102199093164390931602919091179055838352601582528083208f8452909152812084905560609190911b81523690601437366014016000a05050505050505050505050565b600381601081106110ec57600080fd5b0154905081565b6018602052826000526040600020602052816000526040600020818154811061111b57600080fd5b906000526020600020906004918282040191900660080292509250509054906101000a900467ffffffffffffffff1681565b60443560008060088301861061116b5763fe2549876000526004601cfd5b60c083901b60805260888386823786600882030151915060206000858360025afa90508061119857600080fd5b50600080517effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0400000000000000000000000000000000000000000000000000000000000000178082526002602090815260408084208a8552825280842080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660019081179091558385528252808420998452988152888320939093558152908190529490942055505050565b600080603087600037602060006030600060025afa806112705763f91129696000526004601cfd5b6000517effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f010000000000000000000000000000000000000000000000000000000000000017608081815260a08c905260c08b905260308a60e037603088609083013760008060c083600a5afa9250826112f2576309bde3396000526004601cfd5b602886106113085763fe2549876000526004601cfd5b6000602882015278200000000000000000000000000000000000000000000000008152600881018b905285810151935060308a8237603081019b909b52505060509098207effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0500000000000000000000000000000000000000000000000000000000000000176000818152600260209081526040808320868452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600190811790915584845282528083209583529481528482209a909a559081528089529190912096909655505050505050565b6014602052826000526040600020602052816000526040600020816010811061142757600080fd5b0154925083915050565b73ffffffffffffffffffffffffffffffffffffffff891660009081526015602090815260408083208b845290915290205467ffffffffffffffff8116156114a4576040517fc334f06900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6114ae8160c01c90565b67ffffffffffffffff166000036114f1576040517f55d4cbf900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f000000000000000000000000000000000000000000000000000000000000000061151c8260c01c90565b6115309067ffffffffffffffff164261371c565b11611567576040517f55d4cbf900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006115738b8b610759565b905061158c87878360208c01356108bb6108b68e613464565b80156115aa57506115aa84848360208901356108bb6108b68b613464565b6115e0576040517f09bde33900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8760400135896040516020016115f69190613533565b6040516020818303038152906040528051906020012014611643576040517f1968a90200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8460200135886020013560016116599190613571565b14158061168b575060016116738360601c63ffffffff1690565b61167d9190613733565b63ffffffff16856020013514155b156116c2576040517f9a3b119900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6116d0896109d28780613589565b6116d9896121e6565b60006116e48a612b39565b7effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0200000000000000000000000000000000000000000000000000000000000000179050600061173b8460a01c63ffffffff1690565b67ffffffffffffffff169050600160026000848152602001908152602001600020600083815260200190815260200160002060006101000a81548160ff021916908315150217905550601760008e73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008d8152602001908152602001600020546001600084815260200190815260200160002060008381526020019081526020016000208190555061180d8460801c63ffffffff1690565b600083815260208190526040902063ffffffff9190911690556118318d8d81612800565b50505050505050505050505050565b6000828152600260209081526040808320848452909152812054819060ff166118c9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f7072652d696d616765206d757374206578697374000000000000000000000000604482015260640160405180910390fd5b50600083815260208181526040909120546118e5816008613571565b6118f0856020613571565b1061190e5783611901826008613571565b61190b919061371c565b91505b506000938452600160209081526040808620948652939052919092205492909150565b60443560008060088301861061194f5763fe2549876000526004601cfd5b60c083901b6080526088838682378087017ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80151908490207effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f02000000000000000000000000000000000000000000000000000000000000001760008181526002602090815260408083208b8452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600190811790915584845282528083209a83529981528982209390935590815290819052959095209190915550505050565b60008060008060808860601b81528760c01b6014820152858782601c0137601c860181207effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0600000000000000000000000000000000000000000000000000000000000000179350604088026260216001603f5a021015611ac35763dd629f866000526004601cfd5b6000808783601c018c5afa94503d6001019150600882018a10611aee5763fe2549876000526004601cfd5b60c082901b81526008018481533d6000600183013e89017ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8015160008481526002602090815260408083208d8452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600190811790915587845282528083209c83529b81528b8220929092559384528390529790912096909655505050505050565b6000611ba48686610759565b9050611bbd83838360208801356108bb6108b68a613464565b611bf3576040517f09bde33900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602084013515611c2f576040517f9a3b119900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611c37612c56565b611c45816109d28780613589565b611c4e816121e6565b846040013581604051602001611c649190613533565b6040516020818303038152906040528051906020012003611cb1576040517f9843145b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff87166000908152601560209081526040808320898452909152902080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000166001179055611d15878733612800565b50505050505050565b6703782dace9d90000341015611d60576040517fe92c469f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b333214611d99576040517fba092d1600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611da4816008613758565b63ffffffff168263ffffffff1610611de8576040517ffe25498700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000008163ffffffff161015611e48576040517f7b1dafd100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000908152601560209081526040808320868452909152902054611e738160801c63ffffffff1690565b63ffffffff1615611eb0576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b608082901b7fffffffffffffffffffffffff00000000ffffffffffffffffffffffffffffffff60a085901b167fffffffffffffffff0000000000000000ffffffffffffffffffffffffffffffff83161717336000818152601560209081526040808320898452825280832094909455835180850185528381528082018981526013805460018101825590855291517f66de8ffda797e3de9c05e8fc57b3bf0ec28a930d40b0d285d93c06501cf6a090600290930292830180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909216919091179055517f66de8ffda797e3de9c05e8fc57b3bf0ec28a930d40b0d285d93c06501cf6a0919091015591815260168252828120968152959052909320349055505050565b600081600001518260200151836040015160405160200161200d93929190613780565b604051602081830303815290604052805190602001209050919050565b60008160005b601081101561207e578060051b880135600186831c16600181146120635760008481526020839052604090209350612074565b600082815260208590526040902093505b5050600101612030565b5090931495945050505050565b608881511461209957600080fd5b602081016020830161211a565b8260031b8201518060001a8160011a60081b178160021a60101b8260031a60181b17178160041a60201b8260051a60281b178260061a60301b8360071a60381b1717179050612114816120ff868560059190911b015190565b1867ffffffffffffffff16600586901b840152565b50505050565b612126600083836120a6565b612132600183836120a6565b61213e600283836120a6565b61214a600383836120a6565b612156600483836120a6565b612162600583836120a6565b61216e600683836120a6565b61217a600783836120a6565b612186600883836120a6565b612192600983836120a6565b61219e600a83836120a6565b6121aa600b83836120a6565b6121b6600c83836120a6565b6121c2600d83836120a6565b6121ce600e83836120a6565b6121da600f83836120a6565b612114601083836120a6565b6040805178010000000000008082800000000000808a8000000080008000602082015279808b00000000800000018000000080008081800000000000800991810191909152788a00000000000000880000000080008009000000008000000a60608201527b8000808b800000000000008b8000000000008089800000000000800360808201527f80000000000080028000000000000080000000000000800a800000008000000a60a08201527f800000008000808180000000000080800000000080000001800000008000800860c082015260009060e001604051602081830303815290604052905060208201602082016126e0565b600583901b8101518518604085900381901c90851b1867ffffffffffffffff8116600584901b8301525b505050505050565b6102808101516101e082015161014083015160a0840151845118189118186102a082015161020083015161016084015160c0850151602086015118189118186102c083015161022084015161018085015160e0860151604087015118189118186102e08401516102408501516101a0860151610100870151606088015118189118186103008501516102608601516101c0870151610120880151608089015118189118188084603f1c6123cb8660011b67ffffffffffffffff1690565b18188584603f1c6123e68660011b67ffffffffffffffff1690565b18188584603f1c6124018660011b67ffffffffffffffff1690565b181895508483603f1c61241e8560011b67ffffffffffffffff1690565b181894508387603f1c61243b8960011b67ffffffffffffffff1690565b60208b01518b51861867ffffffffffffffff168c5291189190911897508118600181901b603f9190911c18935087925061247b81602c60066001876122dc565b61248b87601460096006876122dc565b61249b86603d60166009876122dc565b6124ab876027600e6016876122dc565b6124bb8260126014600e876122dc565b6124cb86603e60026014876122dc565b6124db86602b600c6002876122dc565b6124eb856019600d600c876122dc565b6124fb8760086013600d876122dc565b61250b85603860176013876122dc565b61251b826029600f6017876122dc565b61252b87601b6004600f876122dc565b61253b87600e60186004876122dc565b61254b81600260156018876122dc565b61255b85603760086015876122dc565b61256b81602d60106008876122dc565b61257b82602460056010876122dc565b61258b85601c60036005876122dc565b61259b85601560126003876122dc565b6125ab86600f60116012876122dc565b6125bb81600a600b6011876122dc565b506125cc8560066007600b866122dc565b6125dc816003600a6007866122dc565b5067ffffffffffffffff8216610140820152612306565b600582811b8201805160018501831b8401805160028701851b8601805160038901871b8801805160048b0190981b8901805167ffffffffffffffff861985168918811690995283198a16861889169096528819861683188816909352841986168818871690528419831684189095169052919391929190611d15565b61267a6000826125f3565b6126856005826125f3565b612690600a826125f3565b61269b600f826125f3565b6126a66014826125f3565b50565b6126b28161230e565b6126bb8161266f565b600383901b820151815160c09190911c9061211490821867ffffffffffffffff168352565b6126ec600082846126a9565b6126f8600182846126a9565b612704600282846126a9565b612710600382846126a9565b61271c600482846126a9565b612728600582846126a9565b612734600682846126a9565b612740600782846126a9565b61274c600882846126a9565b612758600982846126a9565b612764600a82846126a9565b612770600b82846126a9565b61277c600c82846126a9565b612788600d82846126a9565b612794600e82846126a9565b6127a0600f82846126a9565b6127ac601082846126a9565b6127b8601182846126a9565b6127c4601282846126a9565b6127d0601382846126a9565b6127dc601482846126a9565b6127e8601582846126a9565b6127f4601682846126a9565b612114601782846126a9565b73ffffffffffffffffffffffffffffffffffffffff83811660009081526016602090815260408083208684529091528082208054908390559051909284169083908381818185875af1925050503d8060008114612879576040519150601f19603f3d011682016040523d82523d6000602084013e61287e565b606091505b50509050806128b9576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050505050565b7f01000000000000000000000000000000000000000000000000000000000000007effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831617612966818360408051600093845233602052918152606090922091527effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01000000000000000000000000000000000000000000000000000000000000001790565b9392505050565b60606040519050816020820181810182868337608883068080156129b65760888290038501848101848103803687375060806001820353506001845160001a17845386526129cd565b608836843760018353608060878401536088850186525b5050505050601f19603f82510116810160405292915050565b60006129f88260a01c63ffffffff1690565b67ffffffffffffffff1690506000612a168360801c63ffffffff1690565b63ffffffff1690506000612a308460401c63ffffffff1690565b63ffffffff169050600883108015612a46575080155b15612a7a5760c082901b6000908152883560085283513382526017602090815260408084208a855290915290912055612b2f565b60088310158015612a98575080612a9260088561371c565b93508310155b8015612aac5750612aa98782613571565b83105b15612b2f576000612abd828561371c565b905087612acb826020613571565b10158015612ad7575085155b15612b0e576040517ffe25498700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3360009081526017602090815260408083208a845290915290209089013590555b5050505050505050565b6000612bbc565b66ff00ff00ff00ff8160081c1667ff00ff00ff00ff00612b6a8360081b67ffffffffffffffff1690565b1617905065ffff0000ffff8160101c1667ffff0000ffff0000612b978360101b67ffffffffffffffff1690565b1617905060008160201c612bb58360201b67ffffffffffffffff1690565b1792915050565b60808201516020830190612bd490612b40565b612b40565b6040820151612be290612b40565b60401b17612bfa612bcf60018460059190911b015190565b825160809190911b90612c0c90612b40565b60c01b17179392505050565b8260108101928215612c46579160200282015b82811115612c46578251825591602001919060010190612c2b565b50612c52929150612c6e565b5090565b6040518060200160405280612c69612c83565b905290565b5b80821115612c525760008155600101612c6f565b6040518061032001604052806019906020820280368337509192915050565b600060208284031215612cb457600080fd5b5035919050565b803573ffffffffffffffffffffffffffffffffffffffff81168114612cdf57600080fd5b919050565b60008060408385031215612cf757600080fd5b612d0083612cbb565b946020939093013593505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051610320810167ffffffffffffffff81118282101715612d6157612d61612d0e565b60405290565b6040516060810167ffffffffffffffff81118282101715612d6157612d61612d0e565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715612dd157612dd1612d0e565b604052919050565b803567ffffffffffffffff81168114612cdf57600080fd5b6000610320808385031215612e0557600080fd5b604051602080820182811067ffffffffffffffff82111715612e2957612e29612d0e565b806040525081935085601f860112612e4057600080fd5b612e48612d3d565b928501928087851115612e5a57600080fd5b865b85811015612e7a57612e6d81612dd9565b8352918301918301612e5c565b509092525091949350505050565b600060608284031215612e9a57600080fd5b50919050565b60008083601f840112612eb257600080fd5b50813567ffffffffffffffff811115612eca57600080fd5b6020830191508360208260051b8501011115612ee557600080fd5b9250929050565b60008060008060008060008060006103e08a8c031215612f0b57600080fd5b612f148a612cbb565b985060208a01359750612f2a8b60408c01612df1565b96506103608a013567ffffffffffffffff80821115612f4857600080fd5b612f548d838e01612e88565b97506103808c0135915080821115612f6b57600080fd5b612f778d838e01612ea0565b90975095506103a08c0135915080821115612f9157600080fd5b612f9d8d838e01612e88565b94506103c08c0135915080821115612fb457600080fd5b50612fc18c828d01612ea0565b915080935050809150509295985092959850929598565b600080600080600060a08688031215612ff057600080fd5b505083359560208501359550604085013594606081013594506080013592509050565b60005b8381101561302e578181015183820152602001613016565b838111156121145750506000910152565b602081526000825180602084015261305e816040850160208701613013565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160400192915050565b600080604083850312156130a357600080fd5b50508035926020909101359150565b60008083601f8401126130c457600080fd5b50813567ffffffffffffffff8111156130dc57600080fd5b602083019150836020828501011115612ee557600080fd5b600080600080600080600060a0888a03121561310f57600080fd5b8735965060208801359550604088013567ffffffffffffffff8082111561313557600080fd5b6131418b838c016130b2565b909750955060608a013591508082111561315a57600080fd5b506131678a828b01612ea0565b9094509250506080880135801515811461318057600080fd5b8091505092959891949750929550565b6000806000606084860312156131a557600080fd5b6131ae84612cbb565b95602085013595506040909401359392505050565b6000806000604084860312156131d857600080fd5b83359250602084013567ffffffffffffffff8111156131f657600080fd5b613202868287016130b2565b9497909650939450505050565b600080600080600080600060a0888a03121561322a57600080fd5b8735965060208801359550604088013567ffffffffffffffff8082111561325057600080fd5b61325c8b838c016130b2565b909750955060608a013591508082111561327557600080fd5b506132828a828b016130b2565b989b979a50959894979596608090950135949350505050565b6000806000806000608086880312156132b357600080fd5b853594506132c360208701612cbb565b93506132d160408701612dd9565b9250606086013567ffffffffffffffff8111156132ed57600080fd5b6132f9888289016130b2565b969995985093965092949392505050565b60008060008060006080868803121561332257600080fd5b61332b86612cbb565b945060208601359350604086013567ffffffffffffffff8082111561334f57600080fd5b61335b89838a01612e88565b9450606088013591508082111561337157600080fd5b506132f988828901612ea0565b803563ffffffff81168114612cdf57600080fd5b6000806000606084860312156133a757600080fd5b833592506133b76020850161337e565b91506133c56040850161337e565b90509250925092565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361345d5761345d6133fd565b5060010190565b60006060823603121561347657600080fd5b61347e612d67565b823567ffffffffffffffff8082111561349657600080fd5b9084019036601f8301126134a957600080fd5b81356020828211156134bd576134bd612d0e565b6134ed817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f85011601612d8a565b9250818352368183860101111561350357600080fd5b81818501828501376000918301810191909152908352848101359083015250604092830135928101929092525090565b81516103208201908260005b601981101561356857825167ffffffffffffffff1682526020928301929091019060010161353f565b50505092915050565b60008219821115613584576135846133fd565b500190565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126135be57600080fd5b83018035915067ffffffffffffffff8211156135d957600080fd5b602001915036819003821315612ee557600080fd5b600181815b8085111561364757817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0482111561362d5761362d6133fd565b8085161561363a57918102915b93841c93908002906135f3565b509250929050565b60008261365e5750600161370a565b8161366b5750600061370a565b8160018114613681576002811461368b576136a7565b600191505061370a565b60ff84111561369c5761369c6133fd565b50506001821b61370a565b5060208310610133831016604e8410600b84101617156136ca575081810a61370a565b6136d483836135ee565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821115613706576137066133fd565b0290505b92915050565b6000612966838361364f565b60008282101561372e5761372e6133fd565b500390565b600063ffffffff83811690831681811015613750576137506133fd565b039392505050565b600063ffffffff808316818516808303821115613777576137776133fd565b01949350505050565b60008451613792818460208901613013565b9190910192835250602082015260400191905056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\09\xEC8\x03\x80b\09\xEC\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01bV[`\xA0\x82\x90R`\x80\x81\x90R`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\0\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FPreimageOracle: challenge period`D\x82\x01Ri too large`\xB0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0[b\0\0\xBE`\x01`\x10b\0\x01\x9DV[\x81\x10\x15b\0\x01YW`\x03\x81`\x10\x81\x10b\0\0\xDCWb\0\0\xDCb\0\x01\xB7V[\x01T`\x03\x82`\x10\x81\x10b\0\0\xF4Wb\0\0\xF4b\0\x01\xB7V[\x01T`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x03\x82`\x01b\0\x01/\x91\x90b\0\x01\xCDV[`\x10\x81\x10b\0\x01BWb\0\x01Bb\0\x01\xB7V[\x01U\x80b\0\x01P\x81b\0\x01\xE8V[\x91PPb\0\0\xB0V[PPPb\0\x02\x04V[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01vW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15b\0\x01\xB2Wb\0\x01\xB2b\0\x01\x87V[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15b\0\x01\xE3Wb\0\x01\xE3b\0\x01\x87V[P\x01\x90V[`\0`\x01\x82\x01b\0\x01\xFDWb\0\x01\xFDb\0\x01\x87V[P`\x01\x01\x90V[`\x80Q`\xA0Qa7\xB4b\0\x028`\09`\0\x81\x81a\x05\xB0\x01Ra\x1D\xEA\x01R`\0\x81\x81a\x06\xB0\x01Ra\x14\xF3\x01Ra7\xB4`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xD8W`\x005`\xE0\x1C\x80c\x9DS\xA6H\x11a\x01\x02W\x80c\xDD\xCDX\xDE\x11a\0\x95W\x80c\xEC^\xFC\xBC\x11a\0dW\x80c\xEC^\xFC\xBC\x14a\x06\x81W\x80c\xF3\xF4\x80\xD9\x14a\x06\xA1W\x80c\xFA\xF3{\xC7\x14a\x06\xD4W\x80c\xFE\xF2\xB4\xED\x14a\x06\xE7W`\0\x80\xFD[\x80c\xDD\xCDX\xDE\x14a\x05\xD4W\x80c\xE01\x10\xE1\x14a\x06\x0CW\x80c\xE1Y&\x11\x14a\x06AW\x80c\xEAq9P\x14a\x06aW`\0\x80\xFD[\x80c\xB5\xE7\x15L\x11a\0\xD1W\x80c\xB5\xE7\x15L\x14a\x05UW\x80c\xD1\x854\xB5\x14a\x05lW\x80c\xDA5\xC6d\x14a\x05\x8CW\x80c\xDD$\xF9\xBF\x14a\x05\xA1W`\0\x80\xFD[\x80c\x9DS\xA6H\x14a\x04\x8EW\x80c\x9D~\x87i\x14a\x04\xDDW\x80c\xB2\xE6{\xA8\x14a\x04\xFDW\x80c\xB4\x80\x1Ea\x14a\x055W`\0\x80\xFD[\x80ca#\x8B\xDE\x11a\x01zW\x80cz\xC5Gg\x11a\x01IW\x80cz\xC5Gg\x14a\x03\xCAW\x80c\x85B\xCFP\x14a\x03\xEAW\x80c\x88(V\xEF\x14a\x045W\x80c\x8D\xC4\xBE\x11\x14a\x04nW`\0\x80\xFD[\x80ca#\x8B\xDE\x14a\x03\x1EW\x80ceQ\x92{\x14a\x03VW\x80cpQG.\x14a\x03\x8EW\x80cy\x17\xDE\x1D\x14a\x03\xAAW`\0\x80\xFD[\x80c9\t\xAF\\\x11a\x01\xB6W\x80c9\t\xAF\\\x14a\x02qW\x80cMR\xB4\xC9\x14a\x02\x93W\x80cR\xF0\xF3\xAD\x14a\x02\xA8W\x80cT\xFDMP\x14a\x02\xC8W`\0\x80\xFD[\x80c\x01<\xF0\x8B\x14a\x01\xDDW\x80c\x03Y\xA5c\x14a\x02.W\x80c U\xB3k\x14a\x02\\W[`\0\x80\xFD[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01\xFDa\x01\xF86`\x04a,\xA2V[a\x07\x14V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02:W`\0\x80\xFD[Pa\x02Na\x02I6`\x04a,\xE4V[a\x07YV[`@Q\x90\x81R` \x01a\x02%V[4\x80\x15a\x02hW`\0\x80\xFD[Pa\x02N`\x10\x81V[4\x80\x15a\x02}W`\0\x80\xFD[Pa\x02\x91a\x02\x8C6`\x04a.\xECV[a\x08\x91V[\0[4\x80\x15a\x02\x9FW`\0\x80\xFD[Pa\x02Na\n\xE8V[4\x80\x15a\x02\xB4W`\0\x80\xFD[Pa\x02Na\x02\xC36`\x04a/\xD8V[a\x0B\x03V[4\x80\x15a\x02\xD4W`\0\x80\xFD[Pa\x03\x11`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.1.4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x02%\x91\x90a0?V[4\x80\x15a\x03*W`\0\x80\xFD[Pa\x02Na\x0396`\x04a0\x90V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x03bW`\0\x80\xFD[Pa\x02Na\x03q6`\x04a,\xE4V[`\x15` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x02Ng\x03x-\xAC\xE9\xD9\0\0\x81V[4\x80\x15a\x03\xB6W`\0\x80\xFD[Pa\x02\x91a\x03\xC56`\x04a0\xF4V[a\x0B\xD9V[4\x80\x15a\x03\xD6W`\0\x80\xFD[Pa\x02Na\x03\xE56`\x04a,\xA2V[a\x10\xDCV[4\x80\x15a\x03\xF6W`\0\x80\xFD[Pa\x04%a\x04\x056`\x04a0\x90V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02%V[4\x80\x15a\x04AW`\0\x80\xFD[Pa\x04Ua\x04P6`\x04a1\x90V[a\x10\xF3V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02%V[4\x80\x15a\x04zW`\0\x80\xFD[Pa\x02\x91a\x04\x896`\x04a1\xC3V[a\x11MV[4\x80\x15a\x04\x9AW`\0\x80\xFD[Pa\x02Na\x04\xA96`\x04a,\xE4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x18` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[4\x80\x15a\x04\xE9W`\0\x80\xFD[Pa\x02\x91a\x04\xF86`\x04a2\x0FV[a\x12HV[4\x80\x15a\x05\tW`\0\x80\xFD[Pa\x02Na\x05\x186`\x04a,\xE4V[`\x17` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02Na\x05P6`\x04a1\x90V[a\x13\xFFV[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02Nb\x01\x86\xA0\x81V[4\x80\x15a\x05xW`\0\x80\xFD[Pa\x02\x91a\x05\x876`\x04a.\xECV[a\x141V[4\x80\x15a\x05\x98W`\0\x80\xFD[P`\x13Ta\x02NV[4\x80\x15a\x05\xADW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02NV[4\x80\x15a\x05\xE0W`\0\x80\xFD[Pa\x02Na\x05\xEF6`\x04a,\xE4V[`\x16` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x06\x18W`\0\x80\xFD[Pa\x06,a\x06'6`\x04a0\x90V[a\x18@V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02%V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x02\x91a\x06\\6`\x04a1\xC3V[a\x191V[4\x80\x15a\x06mW`\0\x80\xFD[Pa\x02\x91a\x06|6`\x04a2\x9BV[a\x1A9V[4\x80\x15a\x06\x8DW`\0\x80\xFD[Pa\x02\x91a\x06\x9C6`\x04a3\nV[a\x1B\x98V[4\x80\x15a\x06\xADW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02NV[a\x02\x91a\x06\xE26`\x04a3\x92V[a\x1D\x1EV[4\x80\x15a\x06\xF3W`\0\x80\xFD[Pa\x02Na\x07\x026`\x04a,\xA2V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`\x13\x81\x81T\x81\x10a\x07$W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91P\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81\x90a\x07\x9C\x90``\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P`\0[`\x10\x81\x10\x15a\x08\x89W\x81`\x01\x16`\x01\x03a\x08/Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x14` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 \x81`\x10\x81\x10a\x07\xFCWa\x07\xFCa3\xCEV[\x01T`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x84\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92Pa\x08pV[\x82`\x03\x82`\x10\x81\x10a\x08CWa\x08Ca3\xCEV[\x01T`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P[`\x01\x91\x90\x91\x1C\x90\x80a\x08\x81\x81a4,V[\x91PPa\x07\xA7V[PP\x92\x91PPV[`\0a\x08\x9D\x8A\x8Aa\x07YV[\x90Pa\x08\xC0\x86\x86\x83` \x8B\x015a\x08\xBBa\x08\xB6\x8Da4dV[a\x1F\xEAV[a *V[\x80\x15a\x08\xDEWPa\x08\xDE\x83\x83\x83` \x88\x015a\x08\xBBa\x08\xB6\x8Aa4dV[a\t\x14W`@Q\x7F\t\xBD\xE39\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86`@\x015\x88`@Q` \x01a\t*\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\twW`@Q\x7F\x19h\xA9\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83` \x015\x87` \x015`\x01a\t\x8D\x91\x90a5qV[\x14a\t\xC4W`@Q\x7F\x9A;\x11\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x0C\x88a\t\xD2\x86\x80a5\x89V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \x8B\x92PPPV[a\n\x15\x88a!\xE6V[\x83`@\x015\x88`@Q` \x01a\n+\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\nxW`@Q\x7F\x98C\x14[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x8C\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x90Ua\n\xDC\x8A\x8A3a(\0V[PPPPPPPPPPV[`\x01a\n\xF6`\x10`\x02a7\x10V[a\x0B\0\x91\x90a7\x1CV[\x81V[`\0a\x0B\x0F\x86\x86a(\xC0V[\x90Pa\x0B\x1C\x83`\x08a5qV[\x82\x10\x15\x80a\x0B*WP` \x83\x11[\x15a\x0BaW`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x81\x81R`\xC0\x85\x90\x1B\x82R`\x08\x95\x90\x95R\x82Q\x82\x82R`\x02\x86R`@\x80\x83 \x85\x84R\x87R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x87R\x80\x83 \x94\x83R\x93\x86R\x83\x82 U\x81\x81R\x93\x84\x90R\x92 U\x92\x91PPV[``\x81\x15a\x0B\xF2Wa\x0B\xEB\x86\x86a)mV[\x90Pa\x0C,V[\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x93PPPP[3`\0\x90\x81R`\x14` \x90\x81R`@\x80\x83 \x8B\x84R\x90\x91R\x80\x82 \x81Qa\x02\0\x81\x01\x92\x83\x90R\x91`\x10\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0CYWPPPPP\x90P`\0`\x15`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8B\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x0C\xDA\x82``\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P32\x14a\r\x1BW`@Q\x7F\xBA\t-\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r+\x82`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\rjW`@Q\x7F\x87\x13\x8D\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rt\x82`\xC0\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\r\xB5W`@Q\x7FGZ%5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x81\x14a\r\xEEW`@Q\x7F`\xF9]Z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xFB\x89\x89\x8D\x88\x86a)\xE6V[\x83Q` \x85\x01`\x88\x82\x04\x88\x14\x15`\x88\x83\x06\x17\x15a\x0E Wc\x07\xB1\xDA\xF1`\0R`\x04`\x1C\xFD[`@Q`\xC8\x81\x01`@R`\0[\x83\x81\x10\x15a\x0E\xD0W\x80\x83\x01\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01RP\x84`\x88\x83\x01R`\x88\x81\x04`\x05\x1B\x8B\x015`\xA8\x83\x01R`\xC8\x82 `\x01\x86\x01\x95P\x85`\0[a\x02\0\x81\x10\x15a\x0E\xC5W`\x01\x82\x16\x15a\x0E\xA5W\x82\x81\x8B\x01Ra\x0E\xC5V[\x89\x81\x01Q`\0\x90\x81R` \x93\x84R`@\x90 \x92`\x01\x92\x90\x92\x1C\x91\x01a\x0E\x88V[PPP`\x88\x01a\x0E-V[PPPP`\x01`\x10`\x02a\x0E\xE4\x91\x90a7\x10V[a\x0E\xEE\x91\x90a7\x1CV[\x81\x11\x15a\x0F'W`@Q\x7Fb)W#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x9Ca\x0F:\x83`@\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[a\x0FJ\x90c\xFF\xFF\xFF\xFF\x16\x8Aa5qV[`@\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x84\x90\x1B\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x17\x17\x90V[\x91P\x84\x15a\x10)Ww\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16B`\xC0\x1B\x17\x91Pa\x0F\xD6\x82`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16a\x0F\xEC\x83`@\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x14a\x10)W`@Q\x7F{\x1D\xAF\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x14` \x90\x81R`@\x80\x83 \x8E\x84R\x90\x91R\x90 a\x10O\x90\x84`\x10a,\x18V[P3`\0\x81\x81R`\x18` \x90\x81R`@\x80\x83 \x8F\x84R\x82R\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x82\x84 `\x04\x82\x04\x01\x80T`\x03\x90\x92\x16`\x08\x02a\x01\0\ng\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x02\x19\x90\x93\x16C\x90\x93\x16\x02\x91\x90\x91\x17\x90U\x83\x83R`\x15\x82R\x80\x83 \x8F\x84R\x90\x91R\x81 \x84\x90U``\x91\x90\x91\x1B\x81R6\x90`\x1476`\x14\x01`\0\xA0PPPPPPPPPPPV[`\x03\x81`\x10\x81\x10a\x10\xECW`\0\x80\xFD[\x01T\x90P\x81V[`\x18` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x11\x1BW`\0\x80\xFD[\x90`\0R` `\0 \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x92P\x92PP\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`D5`\0\x80`\x08\x83\x01\x86\x10a\x11kWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x83\x90\x1B`\x80R`\x88\x83\x86\x827\x86`\x08\x82\x03\x01Q\x91P` `\0\x85\x83`\x02Z\xFA\x90P\x80a\x11\x98W`\0\x80\xFD[P`\0\x80Q~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x80\x82R`\x02` \x90\x81R`@\x80\x84 \x8A\x85R\x82R\x80\x84 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x83\x85R\x82R\x80\x84 \x99\x84R\x98\x81R\x88\x83 \x93\x90\x93U\x81R\x90\x81\x90R\x94\x90\x94 UPPPV[`\0\x80`0\x87`\x007` `\0`0`\0`\x02Z\xFA\x80a\x12pWc\xF9\x11)i`\0R`\x04`\x1C\xFD[`\0Q~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\x80\x81\x81R`\xA0\x8C\x90R`\xC0\x8B\x90R`0\x8A`\xE07`0\x88`\x90\x83\x017`\0\x80`\xC0\x83`\nZ\xFA\x92P\x82a\x12\xF2Wc\t\xBD\xE39`\0R`\x04`\x1C\xFD[`(\x86\x10a\x13\x08Wc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\0`(\x82\x01Rx \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x08\x81\x01\x8B\x90R\x85\x81\x01Q\x93P`0\x8A\x827`0\x81\x01\x9B\x90\x9BRPP`P\x90\x98 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x82R\x80\x83 \x95\x83R\x94\x81R\x84\x82 \x9A\x90\x9AU\x90\x81R\x80\x89R\x91\x90\x91 \x96\x90\x96UPPPPPPV[`\x14` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81`\x10\x81\x10a\x14'W`\0\x80\xFD[\x01T\x92P\x83\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x8B\x84R\x90\x91R\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\x14\xA4W`@Q\x7F\xC34\xF0i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xAE\x81`\xC0\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x14\xF1W`@Q\x7FU\xD4\xCB\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x15\x1C\x82`\xC0\x1C\x90V[a\x150\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba7\x1CV[\x11a\x15gW`@Q\x7FU\xD4\xCB\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15s\x8B\x8Ba\x07YV[\x90Pa\x15\x8C\x87\x87\x83` \x8C\x015a\x08\xBBa\x08\xB6\x8Ea4dV[\x80\x15a\x15\xAAWPa\x15\xAA\x84\x84\x83` \x89\x015a\x08\xBBa\x08\xB6\x8Ba4dV[a\x15\xE0W`@Q\x7F\t\xBD\xE39\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`@\x015\x89`@Q` \x01a\x15\xF6\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x16CW`@Q\x7F\x19h\xA9\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84` \x015\x88` \x015`\x01a\x16Y\x91\x90a5qV[\x14\x15\x80a\x16\x8BWP`\x01a\x16s\x83``\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[a\x16}\x91\x90a73V[c\xFF\xFF\xFF\xFF\x16\x85` \x015\x14\x15[\x15a\x16\xC2W`@Q\x7F\x9A;\x11\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\xD0\x89a\t\xD2\x87\x80a5\x89V[a\x16\xD9\x89a!\xE6V[`\0a\x16\xE4\x8Aa+9V[~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90P`\0a\x17;\x84`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x01`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x17`\0\x8Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 T`\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\x18\r\x84`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[`\0\x83\x81R` \x81\x90R`@\x90 c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90Ua\x181\x8D\x8D\x81a(\0V[PPPPPPPPPPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81\x90`\xFF\x16a\x18\xC9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fpre-image must exist\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P`\0\x83\x81R` \x81\x81R`@\x90\x91 Ta\x18\xE5\x81`\x08a5qV[a\x18\xF0\x85` a5qV[\x10a\x19\x0EW\x83a\x19\x01\x82`\x08a5qV[a\x19\x0B\x91\x90a7\x1CV[\x91P[P`\0\x93\x84R`\x01` \x90\x81R`@\x80\x86 \x94\x86R\x93\x90R\x91\x90\x92 T\x92\x90\x91PV[`D5`\0\x80`\x08\x83\x01\x86\x10a\x19OWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x83\x90\x1B`\x80R`\x88\x83\x86\x827\x80\x87\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01Q\x90\x84\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x8B\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x82R\x80\x83 \x9A\x83R\x99\x81R\x89\x82 \x93\x90\x93U\x90\x81R\x90\x81\x90R\x95\x90\x95 \x91\x90\x91UPPPPV[`\0\x80`\0\x80`\x80\x88``\x1B\x81R\x87`\xC0\x1B`\x14\x82\x01R\x85\x87\x82`\x1C\x017`\x1C\x86\x01\x81 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x93P`@\x88\x02b`!`\x01`?Z\x02\x10\x15a\x1A\xC3Wc\xDDb\x9F\x86`\0R`\x04`\x1C\xFD[`\0\x80\x87\x83`\x1C\x01\x8CZ\xFA\x94P=`\x01\x01\x91P`\x08\x82\x01\x8A\x10a\x1A\xEEWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x82\x90\x1B\x81R`\x08\x01\x84\x81S=`\0`\x01\x83\x01>\x89\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01Q`\0\x84\x81R`\x02` \x90\x81R`@\x80\x83 \x8D\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x87\x84R\x82R\x80\x83 \x9C\x83R\x9B\x81R\x8B\x82 \x92\x90\x92U\x93\x84R\x83\x90R\x97\x90\x91 \x96\x90\x96UPPPPPPV[`\0a\x1B\xA4\x86\x86a\x07YV[\x90Pa\x1B\xBD\x83\x83\x83` \x88\x015a\x08\xBBa\x08\xB6\x8Aa4dV[a\x1B\xF3W`@Q\x7F\t\xBD\xE39\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x015\x15a\x1C/W`@Q\x7F\x9A;\x11\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C7a,VV[a\x1CE\x81a\t\xD2\x87\x80a5\x89V[a\x1CN\x81a!\xE6V[\x84`@\x015\x81`@Q` \x01a\x1Cd\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x1C\xB1W`@Q\x7F\x98C\x14[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x90Ua\x1D\x15\x87\x873a(\0V[PPPPPPPV[g\x03x-\xAC\xE9\xD9\0\x004\x10\x15a\x1D`W`@Q\x7F\xE9,F\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[32\x14a\x1D\x99W`@Q\x7F\xBA\t-\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xA4\x81`\x08a7XV[c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1D\xE8W`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1EHW`@Q\x7F{\x1D\xAF\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 Ta\x1Es\x81`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x15a\x1E\xB0W`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1B\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17\x173`\0\x81\x81R`\x15` \x90\x81R`@\x80\x83 \x89\x84R\x82R\x80\x83 \x94\x90\x94U\x83Q\x80\x85\x01\x85R\x83\x81R\x80\x82\x01\x89\x81R`\x13\x80T`\x01\x81\x01\x82U\x90\x85R\x91Q\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x90`\x02\x90\x93\x02\x92\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UQ\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x91\x90\x91\x01U\x91\x81R`\x16\x82R\x82\x81 \x96\x81R\x95\x90R\x90\x93 4\x90UPPPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a \r\x93\x92\x91\x90a7\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0[`\x10\x81\x10\x15a ~W\x80`\x05\x1B\x88\x015`\x01\x86\x83\x1C\x16`\x01\x81\x14a cW`\0\x84\x81R` \x83\x90R`@\x90 \x93Pa tV[`\0\x82\x81R` \x85\x90R`@\x90 \x93P[PP`\x01\x01a 0V[P\x90\x93\x14\x95\x94PPPPPV[`\x88\x81Q\x14a \x99W`\0\x80\xFD[` \x81\x01` \x83\x01a!\x1AV[\x82`\x03\x1B\x82\x01Q\x80`\0\x1A\x81`\x01\x1A`\x08\x1B\x17\x81`\x02\x1A`\x10\x1B\x82`\x03\x1A`\x18\x1B\x17\x17\x81`\x04\x1A` \x1B\x82`\x05\x1A`(\x1B\x17\x82`\x06\x1A`0\x1B\x83`\x07\x1A`8\x1B\x17\x17\x17\x90Pa!\x14\x81a \xFF\x86\x85`\x05\x91\x90\x91\x1B\x01Q\x90V[\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05\x86\x90\x1B\x84\x01RV[PPPPV[a!&`\0\x83\x83a \xA6V[a!2`\x01\x83\x83a \xA6V[a!>`\x02\x83\x83a \xA6V[a!J`\x03\x83\x83a \xA6V[a!V`\x04\x83\x83a \xA6V[a!b`\x05\x83\x83a \xA6V[a!n`\x06\x83\x83a \xA6V[a!z`\x07\x83\x83a \xA6V[a!\x86`\x08\x83\x83a \xA6V[a!\x92`\t\x83\x83a \xA6V[a!\x9E`\n\x83\x83a \xA6V[a!\xAA`\x0B\x83\x83a \xA6V[a!\xB6`\x0C\x83\x83a \xA6V[a!\xC2`\r\x83\x83a \xA6V[a!\xCE`\x0E\x83\x83a \xA6V[a!\xDA`\x0F\x83\x83a \xA6V[a!\x14`\x10\x83\x83a \xA6V[`@\x80Qx\x01\0\0\0\0\0\0\x80\x82\x80\0\0\0\0\0\x80\x8A\x80\0\0\0\x80\0\x80\0` \x82\x01Ry\x80\x8B\0\0\0\0\x80\0\0\x01\x80\0\0\0\x80\0\x80\x81\x80\0\0\0\0\0\x80\t\x91\x81\x01\x91\x90\x91Rx\x8A\0\0\0\0\0\0\0\x88\0\0\0\0\x80\0\x80\t\0\0\0\0\x80\0\0\n``\x82\x01R{\x80\0\x80\x8B\x80\0\0\0\0\0\0\x8B\x80\0\0\0\0\0\x80\x89\x80\0\0\0\0\0\x80\x03`\x80\x82\x01R\x7F\x80\0\0\0\0\0\x80\x02\x80\0\0\0\0\0\0\x80\0\0\0\0\0\0\x80\n\x80\0\0\0\x80\0\0\n`\xA0\x82\x01R\x7F\x80\0\0\0\x80\0\x80\x81\x80\0\0\0\0\0\x80\x80\0\0\0\0\x80\0\0\x01\x80\0\0\0\x80\0\x80\x08`\xC0\x82\x01R`\0\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x82\x01` \x82\x01a&\xE0V[`\x05\x83\x90\x1B\x81\x01Q\x85\x18`@\x85\x90\x03\x81\x90\x1C\x90\x85\x1B\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x05\x84\x90\x1B\x83\x01R[PPPPPPV[a\x02\x80\x81\x01Qa\x01\xE0\x82\x01Qa\x01@\x83\x01Q`\xA0\x84\x01Q\x84Q\x18\x18\x91\x18\x18a\x02\xA0\x82\x01Qa\x02\0\x83\x01Qa\x01`\x84\x01Q`\xC0\x85\x01Q` \x86\x01Q\x18\x18\x91\x18\x18a\x02\xC0\x83\x01Qa\x02 \x84\x01Qa\x01\x80\x85\x01Q`\xE0\x86\x01Q`@\x87\x01Q\x18\x18\x91\x18\x18a\x02\xE0\x84\x01Qa\x02@\x85\x01Qa\x01\xA0\x86\x01Qa\x01\0\x87\x01Q``\x88\x01Q\x18\x18\x91\x18\x18a\x03\0\x85\x01Qa\x02`\x86\x01Qa\x01\xC0\x87\x01Qa\x01 \x88\x01Q`\x80\x89\x01Q\x18\x18\x91\x18\x18\x80\x84`?\x1Ca#\xCB\x86`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x85\x84`?\x1Ca#\xE6\x86`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x85\x84`?\x1Ca$\x01\x86`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x95P\x84\x83`?\x1Ca$\x1E\x85`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x94P\x83\x87`?\x1Ca$;\x89`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[` \x8B\x01Q\x8BQ\x86\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8CR\x91\x18\x91\x90\x91\x18\x97P\x81\x18`\x01\x81\x90\x1B`?\x91\x90\x91\x1C\x18\x93P\x87\x92Pa${\x81`,`\x06`\x01\x87a\"\xDCV[a$\x8B\x87`\x14`\t`\x06\x87a\"\xDCV[a$\x9B\x86`=`\x16`\t\x87a\"\xDCV[a$\xAB\x87`'`\x0E`\x16\x87a\"\xDCV[a$\xBB\x82`\x12`\x14`\x0E\x87a\"\xDCV[a$\xCB\x86`>`\x02`\x14\x87a\"\xDCV[a$\xDB\x86`+`\x0C`\x02\x87a\"\xDCV[a$\xEB\x85`\x19`\r`\x0C\x87a\"\xDCV[a$\xFB\x87`\x08`\x13`\r\x87a\"\xDCV[a%\x0B\x85`8`\x17`\x13\x87a\"\xDCV[a%\x1B\x82`)`\x0F`\x17\x87a\"\xDCV[a%+\x87`\x1B`\x04`\x0F\x87a\"\xDCV[a%;\x87`\x0E`\x18`\x04\x87a\"\xDCV[a%K\x81`\x02`\x15`\x18\x87a\"\xDCV[a%[\x85`7`\x08`\x15\x87a\"\xDCV[a%k\x81`-`\x10`\x08\x87a\"\xDCV[a%{\x82`$`\x05`\x10\x87a\"\xDCV[a%\x8B\x85`\x1C`\x03`\x05\x87a\"\xDCV[a%\x9B\x85`\x15`\x12`\x03\x87a\"\xDCV[a%\xAB\x86`\x0F`\x11`\x12\x87a\"\xDCV[a%\xBB\x81`\n`\x0B`\x11\x87a\"\xDCV[Pa%\xCC\x85`\x06`\x07`\x0B\x86a\"\xDCV[a%\xDC\x81`\x03`\n`\x07\x86a\"\xDCV[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x01@\x82\x01Ra#\x06V[`\x05\x82\x81\x1B\x82\x01\x80Q`\x01\x85\x01\x83\x1B\x84\x01\x80Q`\x02\x87\x01\x85\x1B\x86\x01\x80Q`\x03\x89\x01\x87\x1B\x88\x01\x80Q`\x04\x8B\x01\x90\x98\x1B\x89\x01\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x19\x85\x16\x89\x18\x81\x16\x90\x99R\x83\x19\x8A\x16\x86\x18\x89\x16\x90\x96R\x88\x19\x86\x16\x83\x18\x88\x16\x90\x93R\x84\x19\x86\x16\x88\x18\x87\x16\x90R\x84\x19\x83\x16\x84\x18\x90\x95\x16\x90R\x91\x93\x91\x92\x91\x90a\x1D\x15V[a&z`\0\x82a%\xF3V[a&\x85`\x05\x82a%\xF3V[a&\x90`\n\x82a%\xF3V[a&\x9B`\x0F\x82a%\xF3V[a&\xA6`\x14\x82a%\xF3V[PV[a&\xB2\x81a#\x0EV[a&\xBB\x81a&oV[`\x03\x83\x90\x1B\x82\x01Q\x81Q`\xC0\x91\x90\x91\x1C\x90a!\x14\x90\x82\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83RV[a&\xEC`\0\x82\x84a&\xA9V[a&\xF8`\x01\x82\x84a&\xA9V[a'\x04`\x02\x82\x84a&\xA9V[a'\x10`\x03\x82\x84a&\xA9V[a'\x1C`\x04\x82\x84a&\xA9V[a'(`\x05\x82\x84a&\xA9V[a'4`\x06\x82\x84a&\xA9V[a'@`\x07\x82\x84a&\xA9V[a'L`\x08\x82\x84a&\xA9V[a'X`\t\x82\x84a&\xA9V[a'd`\n\x82\x84a&\xA9V[a'p`\x0B\x82\x84a&\xA9V[a'|`\x0C\x82\x84a&\xA9V[a'\x88`\r\x82\x84a&\xA9V[a'\x94`\x0E\x82\x84a&\xA9V[a'\xA0`\x0F\x82\x84a&\xA9V[a'\xAC`\x10\x82\x84a&\xA9V[a'\xB8`\x11\x82\x84a&\xA9V[a'\xC4`\x12\x82\x84a&\xA9V[a'\xD0`\x13\x82\x84a&\xA9V[a'\xDC`\x14\x82\x84a&\xA9V[a'\xE8`\x15\x82\x84a&\xA9V[a'\xF4`\x16\x82\x84a&\xA9V[a!\x14`\x17\x82\x84a&\xA9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x16` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T\x90\x83\x90U\x90Q\x90\x92\x84\x16\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a(yW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a(~V[``\x91P[PP\x90P\x80a(\xB9W`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17a)f\x81\x83`@\x80Q`\0\x93\x84R3` R\x91\x81R``\x90\x92 \x91R~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x93\x92PPPV[```@Q\x90P\x81` \x82\x01\x81\x81\x01\x82\x86\x837`\x88\x83\x06\x80\x80\x15a)\xB6W`\x88\x82\x90\x03\x85\x01\x84\x81\x01\x84\x81\x03\x806\x877P`\x80`\x01\x82\x03SP`\x01\x84Q`\0\x1A\x17\x84S\x86Ra)\xCDV[`\x886\x847`\x01\x83S`\x80`\x87\x84\x01S`\x88\x85\x01\x86R[PPPPP`\x1F\x19`?\x82Q\x01\x16\x81\x01`@R\x92\x91PPV[`\0a)\xF8\x82`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a*\x16\x83`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P`\0a*0\x84`@\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P`\x08\x83\x10\x80\x15a*FWP\x80\x15[\x15a*zW`\xC0\x82\x90\x1B`\0\x90\x81R\x885`\x08R\x83Q3\x82R`\x17` \x90\x81R`@\x80\x84 \x8A\x85R\x90\x91R\x90\x91 Ua+/V[`\x08\x83\x10\x15\x80\x15a*\x98WP\x80a*\x92`\x08\x85a7\x1CV[\x93P\x83\x10\x15[\x80\x15a*\xACWPa*\xA9\x87\x82a5qV[\x83\x10[\x15a+/W`\0a*\xBD\x82\x85a7\x1CV[\x90P\x87a*\xCB\x82` a5qV[\x10\x15\x80\x15a*\xD7WP\x85\x15[\x15a+\x0EW`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x17` \x90\x81R`@\x80\x83 \x8A\x84R\x90\x91R\x90 \x90\x89\x015\x90U[PPPPPPPPV[`\0a+\xBCV[f\xFF\0\xFF\0\xFF\0\xFF\x81`\x08\x1C\x16g\xFF\0\xFF\0\xFF\0\xFF\0a+j\x83`\x08\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x17\x90Pe\xFF\xFF\0\0\xFF\xFF\x81`\x10\x1C\x16g\xFF\xFF\0\0\xFF\xFF\0\0a+\x97\x83`\x10\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x17\x90P`\0\x81` \x1Ca+\xB5\x83` \x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x17\x92\x91PPV[`\x80\x82\x01Q` \x83\x01\x90a+\xD4\x90a+@V[a+@V[`@\x82\x01Qa+\xE2\x90a+@V[`@\x1B\x17a+\xFAa+\xCF`\x01\x84`\x05\x91\x90\x91\x1B\x01Q\x90V[\x82Q`\x80\x91\x90\x91\x1B\x90a,\x0C\x90a+@V[`\xC0\x1B\x17\x17\x93\x92PPPV[\x82`\x10\x81\x01\x92\x82\x15a,FW\x91` \x02\x82\x01[\x82\x81\x11\x15a,FW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a,+V[Pa,R\x92\x91Pa,nV[P\x90V[`@Q\x80` \x01`@R\x80a,ia,\x83V[\x90R\x90V[[\x80\x82\x11\x15a,RW`\0\x81U`\x01\x01a,oV[`@Q\x80a\x03 \x01`@R\x80`\x19\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a,\xB4W`\0\x80\xFD[P5\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xDFW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xF7W`\0\x80\xFD[a-\0\x83a,\xBBV[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Qa\x03 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-aWa-aa-\x0EV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-aWa-aa-\x0EV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-\xD1Wa-\xD1a-\x0EV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xDFW`\0\x80\xFD[`\0a\x03 \x80\x83\x85\x03\x12\x15a.\x05W`\0\x80\xFD[`@Q` \x80\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.)Wa.)a-\x0EV[\x80`@RP\x81\x93P\x85`\x1F\x86\x01\x12a.@W`\0\x80\xFD[a.Ha-=V[\x92\x85\x01\x92\x80\x87\x85\x11\x15a.ZW`\0\x80\xFD[\x86[\x85\x81\x10\x15a.zWa.m\x81a-\xD9V[\x83R\x91\x83\x01\x91\x83\x01a.\\V[P\x90\x92RP\x91\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a.\x9AW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a.\xB2W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xCAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a.\xE5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x03\xE0\x8A\x8C\x03\x12\x15a/\x0BW`\0\x80\xFD[a/\x14\x8Aa,\xBBV[\x98P` \x8A\x015\x97Pa/*\x8B`@\x8C\x01a-\xF1V[\x96Pa\x03`\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a/HW`\0\x80\xFD[a/T\x8D\x83\x8E\x01a.\x88V[\x97Pa\x03\x80\x8C\x015\x91P\x80\x82\x11\x15a/kW`\0\x80\xFD[a/w\x8D\x83\x8E\x01a.\xA0V[\x90\x97P\x95Pa\x03\xA0\x8C\x015\x91P\x80\x82\x11\x15a/\x91W`\0\x80\xFD[a/\x9D\x8D\x83\x8E\x01a.\x88V[\x94Pa\x03\xC0\x8C\x015\x91P\x80\x82\x11\x15a/\xB4W`\0\x80\xFD[Pa/\xC1\x8C\x82\x8D\x01a.\xA0V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a/\xF0W`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0[\x83\x81\x10\x15a0.W\x81\x81\x01Q\x83\x82\x01R` \x01a0\x16V[\x83\x81\x11\x15a!\x14WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra0^\x81`@\x85\x01` \x87\x01a0\x13V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a0\xA3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a0\xC4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xDCW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a.\xE5W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a1\x0FW`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a15W`\0\x80\xFD[a1A\x8B\x83\x8C\x01a0\xB2V[\x90\x97P\x95P``\x8A\x015\x91P\x80\x82\x11\x15a1ZW`\0\x80\xFD[Pa1g\x8A\x82\x8B\x01a.\xA0V[\x90\x94P\x92PP`\x80\x88\x015\x80\x15\x15\x81\x14a1\x80W`\0\x80\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\xA5W`\0\x80\xFD[a1\xAE\x84a,\xBBV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xD8W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xF6W`\0\x80\xFD[a2\x02\x86\x82\x87\x01a0\xB2V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a2*W`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a2PW`\0\x80\xFD[a2\\\x8B\x83\x8C\x01a0\xB2V[\x90\x97P\x95P``\x8A\x015\x91P\x80\x82\x11\x15a2uW`\0\x80\xFD[Pa2\x82\x8A\x82\x8B\x01a0\xB2V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96`\x80\x90\x95\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a2\xB3W`\0\x80\xFD[\x855\x94Pa2\xC3` \x87\x01a,\xBBV[\x93Pa2\xD1`@\x87\x01a-\xD9V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xEDW`\0\x80\xFD[a2\xF9\x88\x82\x89\x01a0\xB2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a3\"W`\0\x80\xFD[a3+\x86a,\xBBV[\x94P` \x86\x015\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a3OW`\0\x80\xFD[a3[\x89\x83\x8A\x01a.\x88V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a3qW`\0\x80\xFD[Pa2\xF9\x88\x82\x89\x01a.\xA0V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xDFW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xA7W`\0\x80\xFD[\x835\x92Pa3\xB7` \x85\x01a3~V[\x91Pa3\xC5`@\x85\x01a3~V[\x90P\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a4]Wa4]a3\xFDV[P`\x01\x01\x90V[`\0``\x826\x03\x12\x15a4vW`\0\x80\xFD[a4~a-gV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4\x96W`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12a4\xA9W`\0\x80\xFD[\x815` \x82\x82\x11\x15a4\xBDWa4\xBDa-\x0EV[a4\xED\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01a-\x8AV[\x92P\x81\x83R6\x81\x83\x86\x01\x01\x11\x15a5\x03W`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x91\x83\x01\x81\x01\x91\x90\x91R\x90\x83R\x84\x81\x015\x90\x83\x01RP`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[\x81Qa\x03 \x82\x01\x90\x82`\0[`\x19\x81\x10\x15a5hW\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a5?V[PPP\x92\x91PPV[`\0\x82\x19\x82\x11\x15a5\x84Wa5\x84a3\xFDV[P\x01\x90V[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a5\xBEW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a5\xD9W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a.\xE5W`\0\x80\xFD[`\x01\x81\x81[\x80\x85\x11\x15a6GW\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a6-Wa6-a3\xFDV[\x80\x85\x16\x15a6:W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a5\xF3V[P\x92P\x92\x90PV[`\0\x82a6^WP`\x01a7\nV[\x81a6kWP`\0a7\nV[\x81`\x01\x81\x14a6\x81W`\x02\x81\x14a6\x8BWa6\xA7V[`\x01\x91PPa7\nV[`\xFF\x84\x11\x15a6\x9CWa6\x9Ca3\xFDV[PP`\x01\x82\x1Ba7\nV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a6\xCAWP\x81\x81\na7\nV[a6\xD4\x83\x83a5\xEEV[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a7\x06Wa7\x06a3\xFDV[\x02\x90P[\x92\x91PPV[`\0a)f\x83\x83a6OV[`\0\x82\x82\x10\x15a7.Wa7.a3\xFDV[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a7PWa7Pa3\xFDV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a7wWa7wa3\xFDV[\x01\x94\x93PPPPV[`\0\x84Qa7\x92\x81\x84` \x89\x01a0\x13V[\x91\x90\x91\x01\x92\x83RP` \x82\x01R`@\x01\x91\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101d85760003560e01c80639d53a64811610102578063ddcd58de11610095578063ec5efcbc11610064578063ec5efcbc14610681578063f3f480d9146106a1578063faf37bc7146106d4578063fef2b4ed146106e757600080fd5b8063ddcd58de146105d4578063e03110e11461060c578063e159261114610641578063ea7139501461066157600080fd5b8063b5e7154c116100d1578063b5e7154c14610555578063d18534b51461056c578063da35c6641461058c578063dd24f9bf146105a157600080fd5b80639d53a6481461048e5780639d7e8769146104dd578063b2e67ba8146104fd578063b4801e611461053557600080fd5b806361238bde1161017a5780637ac54767116101495780637ac54767146103ca5780638542cf50146103ea578063882856ef146104355780638dc4be111461046e57600080fd5b806361238bde1461031e5780636551927b146103565780637051472e1461038e5780637917de1d146103aa57600080fd5b80633909af5c116101b65780633909af5c146102715780634d52b4c91461029357806352f0f3ad146102a857806354fd4d50146102c857600080fd5b8063013cf08b146101dd5780630359a5631461022e5780632055b36b1461025c575b600080fd5b3480156101e957600080fd5b506101fd6101f8366004612ca2565b610714565b6040805173ffffffffffffffffffffffffffffffffffffffff90931683526020830191909152015b60405180910390f35b34801561023a57600080fd5b5061024e610249366004612ce4565b610759565b604051908152602001610225565b34801561026857600080fd5b5061024e601081565b34801561027d57600080fd5b5061029161028c366004612eec565b610891565b005b34801561029f57600080fd5b5061024e610ae8565b3480156102b457600080fd5b5061024e6102c3366004612fd8565b610b03565b3480156102d457600080fd5b506103116040518060400160405280600581526020017f312e312e3400000000000000000000000000000000000000000000000000000081525081565b604051610225919061303f565b34801561032a57600080fd5b5061024e610339366004613090565b600160209081526000928352604080842090915290825290205481565b34801561036257600080fd5b5061024e610371366004612ce4565b601560209081526000928352604080842090915290825290205481565b34801561039a57600080fd5b5061024e6703782dace9d9000081565b3480156103b657600080fd5b506102916103c53660046130f4565b610bd9565b3480156103d657600080fd5b5061024e6103e5366004612ca2565b6110dc565b3480156103f657600080fd5b50610425610405366004613090565b600260209081526000928352604080842090915290825290205460ff1681565b6040519015158152602001610225565b34801561044157600080fd5b50610455610450366004613190565b6110f3565b60405167ffffffffffffffff9091168152602001610225565b34801561047a57600080fd5b506102916104893660046131c3565b61114d565b34801561049a57600080fd5b5061024e6104a9366004612ce4565b73ffffffffffffffffffffffffffffffffffffffff9091166000908152601860209081526040808320938352929052205490565b3480156104e957600080fd5b506102916104f836600461320f565b611248565b34801561050957600080fd5b5061024e610518366004612ce4565b601760209081526000928352604080842090915290825290205481565b34801561054157600080fd5b5061024e610550366004613190565b6113ff565b34801561056157600080fd5b5061024e620186a081565b34801561057857600080fd5b50610291610587366004612eec565b611431565b34801561059857600080fd5b5060135461024e565b3480156105ad57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061024e565b3480156105e057600080fd5b5061024e6105ef366004612ce4565b601660209081526000928352604080842090915290825290205481565b34801561061857600080fd5b5061062c610627366004613090565b611840565b60408051928352602083019190915201610225565b34801561064d57600080fd5b5061029161065c3660046131c3565b611931565b34801561066d57600080fd5b5061029161067c36600461329b565b611a39565b34801561068d57600080fd5b5061029161069c36600461330a565b611b98565b3480156106ad57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061024e565b6102916106e2366004613392565b611d1e565b3480156106f357600080fd5b5061024e610702366004612ca2565b60006020819052908152604090205481565b6013818154811061072457600080fd5b60009182526020909120600290910201805460019091015473ffffffffffffffffffffffffffffffffffffffff909116915082565b73ffffffffffffffffffffffffffffffffffffffff82166000908152601560209081526040808320848452909152812054819061079c9060601c63ffffffff1690565b63ffffffff16905060005b6010811015610889578160011660010361082f5773ffffffffffffffffffffffffffffffffffffffff85166000908152601460209081526040808320878452909152902081601081106107fc576107fc6133ce565b01546040805160208101929092528101849052606001604051602081830303815290604052805190602001209250610870565b8260038260108110610843576108436133ce565b01546040805160208101939093528201526060016040516020818303038152906040528051906020012092505b60019190911c90806108818161342c565b9150506107a7565b505092915050565b600061089d8a8a610759565b90506108c086868360208b01356108bb6108b68d613464565b611fea565b61202a565b80156108de57506108de83838360208801356108bb6108b68a613464565b610914576040517f09bde33900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b86604001358860405160200161092a9190613533565b6040516020818303038152906040528051906020012014610977576040517f1968a90200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b83602001358760200135600161098d9190613571565b146109c4576040517f9a3b119900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a0c886109d28680613589565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061208b92505050565b610a15886121e6565b836040013588604051602001610a2b9190613533565b6040516020818303038152906040528051906020012003610a78576040517f9843145b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8a1660009081526015602090815260408083208c8452909152902080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000166001179055610adc8a8a33612800565b50505050505050505050565b6001610af660106002613710565b610b00919061371c565b81565b6000610b0f86866128c0565b9050610b1c836008613571565b82101580610b2a5750602083115b15610b61576040517ffe25498700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000602081815260c085901b82526008959095528251828252600286526040808320858452875280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660019081179091558484528752808320948352938652838220558181529384905292205592915050565b60608115610bf257610beb868661296d565b9050610c2c565b85858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509293505050505b3360009081526014602090815260408083208b845290915280822081516102008101928390529160109082845b815481526020019060010190808311610c5957505050505090506000601560003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008b81526020019081526020016000205490506000610cda8260601c63ffffffff1690565b63ffffffff169050333214610d1b576040517fba092d1600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d2b8260801c63ffffffff1690565b63ffffffff16600003610d6a576040517f87138d5c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d748260c01c90565b67ffffffffffffffff1615610db5576040517f475a253500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b898114610dee576040517f60f95d5a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610dfb89898d88866129e6565b83516020850160888204881415608883061715610e20576307b1daf16000526004601cfd5b60405160c8810160405260005b83811015610ed0578083018051835260208101516020840152604081015160408401526060810151606084015260808101516080840152508460888301526088810460051b8b013560a883015260c882206001860195508560005b610200811015610ec5576001821615610ea55782818b0152610ec5565b8981015160009081526020938452604090209260019290921c9101610e88565b505050608801610e2d565b50505050600160106002610ee49190613710565b610eee919061371c565b811115610f27576040517f6229572300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610f9c610f3a8360401c63ffffffff1690565b610f4a9063ffffffff168a613571565b60401b7fffffffffffffffffffffffffffffffffffffffff00000000ffffffffffffffff606084901b167fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff8516171790565b915084156110295777ffffffffffffffffffffffffffffffffffffffffffffffff82164260c01b179150610fd68260801c63ffffffff1690565b63ffffffff16610fec8360401c63ffffffff1690565b63ffffffff1614611029576040517f7b1dafd100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3360009081526014602090815260408083208e8452909152902061104f90846010612c18565b503360008181526018602090815260408083208f8452825280832080546001810182559084528284206004820401805460039092166008026101000a67ffffffffffffffff818102199093164390931602919091179055838352601582528083208f8452909152812084905560609190911b81523690601437366014016000a05050505050505050505050565b600381601081106110ec57600080fd5b0154905081565b6018602052826000526040600020602052816000526040600020818154811061111b57600080fd5b906000526020600020906004918282040191900660080292509250509054906101000a900467ffffffffffffffff1681565b60443560008060088301861061116b5763fe2549876000526004601cfd5b60c083901b60805260888386823786600882030151915060206000858360025afa90508061119857600080fd5b50600080517effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0400000000000000000000000000000000000000000000000000000000000000178082526002602090815260408084208a8552825280842080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660019081179091558385528252808420998452988152888320939093558152908190529490942055505050565b600080603087600037602060006030600060025afa806112705763f91129696000526004601cfd5b6000517effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f010000000000000000000000000000000000000000000000000000000000000017608081815260a08c905260c08b905260308a60e037603088609083013760008060c083600a5afa9250826112f2576309bde3396000526004601cfd5b602886106113085763fe2549876000526004601cfd5b6000602882015278200000000000000000000000000000000000000000000000008152600881018b905285810151935060308a8237603081019b909b52505060509098207effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0500000000000000000000000000000000000000000000000000000000000000176000818152600260209081526040808320868452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600190811790915584845282528083209583529481528482209a909a559081528089529190912096909655505050505050565b6014602052826000526040600020602052816000526040600020816010811061142757600080fd5b0154925083915050565b73ffffffffffffffffffffffffffffffffffffffff891660009081526015602090815260408083208b845290915290205467ffffffffffffffff8116156114a4576040517fc334f06900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6114ae8160c01c90565b67ffffffffffffffff166000036114f1576040517f55d4cbf900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f000000000000000000000000000000000000000000000000000000000000000061151c8260c01c90565b6115309067ffffffffffffffff164261371c565b11611567576040517f55d4cbf900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006115738b8b610759565b905061158c87878360208c01356108bb6108b68e613464565b80156115aa57506115aa84848360208901356108bb6108b68b613464565b6115e0576040517f09bde33900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8760400135896040516020016115f69190613533565b6040516020818303038152906040528051906020012014611643576040517f1968a90200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8460200135886020013560016116599190613571565b14158061168b575060016116738360601c63ffffffff1690565b61167d9190613733565b63ffffffff16856020013514155b156116c2576040517f9a3b119900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6116d0896109d28780613589565b6116d9896121e6565b60006116e48a612b39565b7effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0200000000000000000000000000000000000000000000000000000000000000179050600061173b8460a01c63ffffffff1690565b67ffffffffffffffff169050600160026000848152602001908152602001600020600083815260200190815260200160002060006101000a81548160ff021916908315150217905550601760008e73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008d8152602001908152602001600020546001600084815260200190815260200160002060008381526020019081526020016000208190555061180d8460801c63ffffffff1690565b600083815260208190526040902063ffffffff9190911690556118318d8d81612800565b50505050505050505050505050565b6000828152600260209081526040808320848452909152812054819060ff166118c9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f7072652d696d616765206d757374206578697374000000000000000000000000604482015260640160405180910390fd5b50600083815260208181526040909120546118e5816008613571565b6118f0856020613571565b1061190e5783611901826008613571565b61190b919061371c565b91505b506000938452600160209081526040808620948652939052919092205492909150565b60443560008060088301861061194f5763fe2549876000526004601cfd5b60c083901b6080526088838682378087017ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80151908490207effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f02000000000000000000000000000000000000000000000000000000000000001760008181526002602090815260408083208b8452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600190811790915584845282528083209a83529981528982209390935590815290819052959095209190915550505050565b60008060008060808860601b81528760c01b6014820152858782601c0137601c860181207effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0600000000000000000000000000000000000000000000000000000000000000179350604088026260216001603f5a021015611ac35763dd629f866000526004601cfd5b6000808783601c018c5afa94503d6001019150600882018a10611aee5763fe2549876000526004601cfd5b60c082901b81526008018481533d6000600183013e89017ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8015160008481526002602090815260408083208d8452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600190811790915587845282528083209c83529b81528b8220929092559384528390529790912096909655505050505050565b6000611ba48686610759565b9050611bbd83838360208801356108bb6108b68a613464565b611bf3576040517f09bde33900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602084013515611c2f576040517f9a3b119900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611c37612c56565b611c45816109d28780613589565b611c4e816121e6565b846040013581604051602001611c649190613533565b6040516020818303038152906040528051906020012003611cb1576040517f9843145b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff87166000908152601560209081526040808320898452909152902080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000166001179055611d15878733612800565b50505050505050565b6703782dace9d90000341015611d60576040517fe92c469f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b333214611d99576040517fba092d1600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611da4816008613758565b63ffffffff168263ffffffff1610611de8576040517ffe25498700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000008163ffffffff161015611e48576040517f7b1dafd100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000908152601560209081526040808320868452909152902054611e738160801c63ffffffff1690565b63ffffffff1615611eb0576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b608082901b7fffffffffffffffffffffffff00000000ffffffffffffffffffffffffffffffff60a085901b167fffffffffffffffff0000000000000000ffffffffffffffffffffffffffffffff83161717336000818152601560209081526040808320898452825280832094909455835180850185528381528082018981526013805460018101825590855291517f66de8ffda797e3de9c05e8fc57b3bf0ec28a930d40b0d285d93c06501cf6a090600290930292830180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff909216919091179055517f66de8ffda797e3de9c05e8fc57b3bf0ec28a930d40b0d285d93c06501cf6a0919091015591815260168252828120968152959052909320349055505050565b600081600001518260200151836040015160405160200161200d93929190613780565b604051602081830303815290604052805190602001209050919050565b60008160005b601081101561207e578060051b880135600186831c16600181146120635760008481526020839052604090209350612074565b600082815260208590526040902093505b5050600101612030565b5090931495945050505050565b608881511461209957600080fd5b602081016020830161211a565b8260031b8201518060001a8160011a60081b178160021a60101b8260031a60181b17178160041a60201b8260051a60281b178260061a60301b8360071a60381b1717179050612114816120ff868560059190911b015190565b1867ffffffffffffffff16600586901b840152565b50505050565b612126600083836120a6565b612132600183836120a6565b61213e600283836120a6565b61214a600383836120a6565b612156600483836120a6565b612162600583836120a6565b61216e600683836120a6565b61217a600783836120a6565b612186600883836120a6565b612192600983836120a6565b61219e600a83836120a6565b6121aa600b83836120a6565b6121b6600c83836120a6565b6121c2600d83836120a6565b6121ce600e83836120a6565b6121da600f83836120a6565b612114601083836120a6565b6040805178010000000000008082800000000000808a8000000080008000602082015279808b00000000800000018000000080008081800000000000800991810191909152788a00000000000000880000000080008009000000008000000a60608201527b8000808b800000000000008b8000000000008089800000000000800360808201527f80000000000080028000000000000080000000000000800a800000008000000a60a08201527f800000008000808180000000000080800000000080000001800000008000800860c082015260009060e001604051602081830303815290604052905060208201602082016126e0565b600583901b8101518518604085900381901c90851b1867ffffffffffffffff8116600584901b8301525b505050505050565b6102808101516101e082015161014083015160a0840151845118189118186102a082015161020083015161016084015160c0850151602086015118189118186102c083015161022084015161018085015160e0860151604087015118189118186102e08401516102408501516101a0860151610100870151606088015118189118186103008501516102608601516101c0870151610120880151608089015118189118188084603f1c6123cb8660011b67ffffffffffffffff1690565b18188584603f1c6123e68660011b67ffffffffffffffff1690565b18188584603f1c6124018660011b67ffffffffffffffff1690565b181895508483603f1c61241e8560011b67ffffffffffffffff1690565b181894508387603f1c61243b8960011b67ffffffffffffffff1690565b60208b01518b51861867ffffffffffffffff168c5291189190911897508118600181901b603f9190911c18935087925061247b81602c60066001876122dc565b61248b87601460096006876122dc565b61249b86603d60166009876122dc565b6124ab876027600e6016876122dc565b6124bb8260126014600e876122dc565b6124cb86603e60026014876122dc565b6124db86602b600c6002876122dc565b6124eb856019600d600c876122dc565b6124fb8760086013600d876122dc565b61250b85603860176013876122dc565b61251b826029600f6017876122dc565b61252b87601b6004600f876122dc565b61253b87600e60186004876122dc565b61254b81600260156018876122dc565b61255b85603760086015876122dc565b61256b81602d60106008876122dc565b61257b82602460056010876122dc565b61258b85601c60036005876122dc565b61259b85601560126003876122dc565b6125ab86600f60116012876122dc565b6125bb81600a600b6011876122dc565b506125cc8560066007600b866122dc565b6125dc816003600a6007866122dc565b5067ffffffffffffffff8216610140820152612306565b600582811b8201805160018501831b8401805160028701851b8601805160038901871b8801805160048b0190981b8901805167ffffffffffffffff861985168918811690995283198a16861889169096528819861683188816909352841986168818871690528419831684189095169052919391929190611d15565b61267a6000826125f3565b6126856005826125f3565b612690600a826125f3565b61269b600f826125f3565b6126a66014826125f3565b50565b6126b28161230e565b6126bb8161266f565b600383901b820151815160c09190911c9061211490821867ffffffffffffffff168352565b6126ec600082846126a9565b6126f8600182846126a9565b612704600282846126a9565b612710600382846126a9565b61271c600482846126a9565b612728600582846126a9565b612734600682846126a9565b612740600782846126a9565b61274c600882846126a9565b612758600982846126a9565b612764600a82846126a9565b612770600b82846126a9565b61277c600c82846126a9565b612788600d82846126a9565b612794600e82846126a9565b6127a0600f82846126a9565b6127ac601082846126a9565b6127b8601182846126a9565b6127c4601282846126a9565b6127d0601382846126a9565b6127dc601482846126a9565b6127e8601582846126a9565b6127f4601682846126a9565b612114601782846126a9565b73ffffffffffffffffffffffffffffffffffffffff83811660009081526016602090815260408083208684529091528082208054908390559051909284169083908381818185875af1925050503d8060008114612879576040519150601f19603f3d011682016040523d82523d6000602084013e61287e565b606091505b50509050806128b9576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050505050565b7f01000000000000000000000000000000000000000000000000000000000000007effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831617612966818360408051600093845233602052918152606090922091527effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01000000000000000000000000000000000000000000000000000000000000001790565b9392505050565b60606040519050816020820181810182868337608883068080156129b65760888290038501848101848103803687375060806001820353506001845160001a17845386526129cd565b608836843760018353608060878401536088850186525b5050505050601f19603f82510116810160405292915050565b60006129f88260a01c63ffffffff1690565b67ffffffffffffffff1690506000612a168360801c63ffffffff1690565b63ffffffff1690506000612a308460401c63ffffffff1690565b63ffffffff169050600883108015612a46575080155b15612a7a5760c082901b6000908152883560085283513382526017602090815260408084208a855290915290912055612b2f565b60088310158015612a98575080612a9260088561371c565b93508310155b8015612aac5750612aa98782613571565b83105b15612b2f576000612abd828561371c565b905087612acb826020613571565b10158015612ad7575085155b15612b0e576040517ffe25498700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3360009081526017602090815260408083208a845290915290209089013590555b5050505050505050565b6000612bbc565b66ff00ff00ff00ff8160081c1667ff00ff00ff00ff00612b6a8360081b67ffffffffffffffff1690565b1617905065ffff0000ffff8160101c1667ffff0000ffff0000612b978360101b67ffffffffffffffff1690565b1617905060008160201c612bb58360201b67ffffffffffffffff1690565b1792915050565b60808201516020830190612bd490612b40565b612b40565b6040820151612be290612b40565b60401b17612bfa612bcf60018460059190911b015190565b825160809190911b90612c0c90612b40565b60c01b17179392505050565b8260108101928215612c46579160200282015b82811115612c46578251825591602001919060010190612c2b565b50612c52929150612c6e565b5090565b6040518060200160405280612c69612c83565b905290565b5b80821115612c525760008155600101612c6f565b6040518061032001604052806019906020820280368337509192915050565b600060208284031215612cb457600080fd5b5035919050565b803573ffffffffffffffffffffffffffffffffffffffff81168114612cdf57600080fd5b919050565b60008060408385031215612cf757600080fd5b612d0083612cbb565b946020939093013593505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051610320810167ffffffffffffffff81118282101715612d6157612d61612d0e565b60405290565b6040516060810167ffffffffffffffff81118282101715612d6157612d61612d0e565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715612dd157612dd1612d0e565b604052919050565b803567ffffffffffffffff81168114612cdf57600080fd5b6000610320808385031215612e0557600080fd5b604051602080820182811067ffffffffffffffff82111715612e2957612e29612d0e565b806040525081935085601f860112612e4057600080fd5b612e48612d3d565b928501928087851115612e5a57600080fd5b865b85811015612e7a57612e6d81612dd9565b8352918301918301612e5c565b509092525091949350505050565b600060608284031215612e9a57600080fd5b50919050565b60008083601f840112612eb257600080fd5b50813567ffffffffffffffff811115612eca57600080fd5b6020830191508360208260051b8501011115612ee557600080fd5b9250929050565b60008060008060008060008060006103e08a8c031215612f0b57600080fd5b612f148a612cbb565b985060208a01359750612f2a8b60408c01612df1565b96506103608a013567ffffffffffffffff80821115612f4857600080fd5b612f548d838e01612e88565b97506103808c0135915080821115612f6b57600080fd5b612f778d838e01612ea0565b90975095506103a08c0135915080821115612f9157600080fd5b612f9d8d838e01612e88565b94506103c08c0135915080821115612fb457600080fd5b50612fc18c828d01612ea0565b915080935050809150509295985092959850929598565b600080600080600060a08688031215612ff057600080fd5b505083359560208501359550604085013594606081013594506080013592509050565b60005b8381101561302e578181015183820152602001613016565b838111156121145750506000910152565b602081526000825180602084015261305e816040850160208701613013565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160400192915050565b600080604083850312156130a357600080fd5b50508035926020909101359150565b60008083601f8401126130c457600080fd5b50813567ffffffffffffffff8111156130dc57600080fd5b602083019150836020828501011115612ee557600080fd5b600080600080600080600060a0888a03121561310f57600080fd5b8735965060208801359550604088013567ffffffffffffffff8082111561313557600080fd5b6131418b838c016130b2565b909750955060608a013591508082111561315a57600080fd5b506131678a828b01612ea0565b9094509250506080880135801515811461318057600080fd5b8091505092959891949750929550565b6000806000606084860312156131a557600080fd5b6131ae84612cbb565b95602085013595506040909401359392505050565b6000806000604084860312156131d857600080fd5b83359250602084013567ffffffffffffffff8111156131f657600080fd5b613202868287016130b2565b9497909650939450505050565b600080600080600080600060a0888a03121561322a57600080fd5b8735965060208801359550604088013567ffffffffffffffff8082111561325057600080fd5b61325c8b838c016130b2565b909750955060608a013591508082111561327557600080fd5b506132828a828b016130b2565b989b979a50959894979596608090950135949350505050565b6000806000806000608086880312156132b357600080fd5b853594506132c360208701612cbb565b93506132d160408701612dd9565b9250606086013567ffffffffffffffff8111156132ed57600080fd5b6132f9888289016130b2565b969995985093965092949392505050565b60008060008060006080868803121561332257600080fd5b61332b86612cbb565b945060208601359350604086013567ffffffffffffffff8082111561334f57600080fd5b61335b89838a01612e88565b9450606088013591508082111561337157600080fd5b506132f988828901612ea0565b803563ffffffff81168114612cdf57600080fd5b6000806000606084860312156133a757600080fd5b833592506133b76020850161337e565b91506133c56040850161337e565b90509250925092565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361345d5761345d6133fd565b5060010190565b60006060823603121561347657600080fd5b61347e612d67565b823567ffffffffffffffff8082111561349657600080fd5b9084019036601f8301126134a957600080fd5b81356020828211156134bd576134bd612d0e565b6134ed817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f85011601612d8a565b9250818352368183860101111561350357600080fd5b81818501828501376000918301810191909152908352848101359083015250604092830135928101929092525090565b81516103208201908260005b601981101561356857825167ffffffffffffffff1682526020928301929091019060010161353f565b50505092915050565b60008219821115613584576135846133fd565b500190565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126135be57600080fd5b83018035915067ffffffffffffffff8211156135d957600080fd5b602001915036819003821315612ee557600080fd5b600181815b8085111561364757817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0482111561362d5761362d6133fd565b8085161561363a57918102915b93841c93908002906135f3565b509250929050565b60008261365e5750600161370a565b8161366b5750600061370a565b8160018114613681576002811461368b576136a7565b600191505061370a565b60ff84111561369c5761369c6133fd565b50506001821b61370a565b5060208310610133831016604e8410600b84101617156136ca575081810a61370a565b6136d483836135ee565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821115613706576137066133fd565b0290505b92915050565b6000612966838361364f565b60008282101561372e5761372e6133fd565b500390565b600063ffffffff83811690831681811015613750576137506133fd565b039392505050565b600063ffffffff808316818516808303821115613777576137776133fd565b01949350505050565b60008451613792818460208901613013565b9190910192835250602082015260400191905056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xD8W`\x005`\xE0\x1C\x80c\x9DS\xA6H\x11a\x01\x02W\x80c\xDD\xCDX\xDE\x11a\0\x95W\x80c\xEC^\xFC\xBC\x11a\0dW\x80c\xEC^\xFC\xBC\x14a\x06\x81W\x80c\xF3\xF4\x80\xD9\x14a\x06\xA1W\x80c\xFA\xF3{\xC7\x14a\x06\xD4W\x80c\xFE\xF2\xB4\xED\x14a\x06\xE7W`\0\x80\xFD[\x80c\xDD\xCDX\xDE\x14a\x05\xD4W\x80c\xE01\x10\xE1\x14a\x06\x0CW\x80c\xE1Y&\x11\x14a\x06AW\x80c\xEAq9P\x14a\x06aW`\0\x80\xFD[\x80c\xB5\xE7\x15L\x11a\0\xD1W\x80c\xB5\xE7\x15L\x14a\x05UW\x80c\xD1\x854\xB5\x14a\x05lW\x80c\xDA5\xC6d\x14a\x05\x8CW\x80c\xDD$\xF9\xBF\x14a\x05\xA1W`\0\x80\xFD[\x80c\x9DS\xA6H\x14a\x04\x8EW\x80c\x9D~\x87i\x14a\x04\xDDW\x80c\xB2\xE6{\xA8\x14a\x04\xFDW\x80c\xB4\x80\x1Ea\x14a\x055W`\0\x80\xFD[\x80ca#\x8B\xDE\x11a\x01zW\x80cz\xC5Gg\x11a\x01IW\x80cz\xC5Gg\x14a\x03\xCAW\x80c\x85B\xCFP\x14a\x03\xEAW\x80c\x88(V\xEF\x14a\x045W\x80c\x8D\xC4\xBE\x11\x14a\x04nW`\0\x80\xFD[\x80ca#\x8B\xDE\x14a\x03\x1EW\x80ceQ\x92{\x14a\x03VW\x80cpQG.\x14a\x03\x8EW\x80cy\x17\xDE\x1D\x14a\x03\xAAW`\0\x80\xFD[\x80c9\t\xAF\\\x11a\x01\xB6W\x80c9\t\xAF\\\x14a\x02qW\x80cMR\xB4\xC9\x14a\x02\x93W\x80cR\xF0\xF3\xAD\x14a\x02\xA8W\x80cT\xFDMP\x14a\x02\xC8W`\0\x80\xFD[\x80c\x01<\xF0\x8B\x14a\x01\xDDW\x80c\x03Y\xA5c\x14a\x02.W\x80c U\xB3k\x14a\x02\\W[`\0\x80\xFD[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01\xFDa\x01\xF86`\x04a,\xA2V[a\x07\x14V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02:W`\0\x80\xFD[Pa\x02Na\x02I6`\x04a,\xE4V[a\x07YV[`@Q\x90\x81R` \x01a\x02%V[4\x80\x15a\x02hW`\0\x80\xFD[Pa\x02N`\x10\x81V[4\x80\x15a\x02}W`\0\x80\xFD[Pa\x02\x91a\x02\x8C6`\x04a.\xECV[a\x08\x91V[\0[4\x80\x15a\x02\x9FW`\0\x80\xFD[Pa\x02Na\n\xE8V[4\x80\x15a\x02\xB4W`\0\x80\xFD[Pa\x02Na\x02\xC36`\x04a/\xD8V[a\x0B\x03V[4\x80\x15a\x02\xD4W`\0\x80\xFD[Pa\x03\x11`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.1.4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x02%\x91\x90a0?V[4\x80\x15a\x03*W`\0\x80\xFD[Pa\x02Na\x0396`\x04a0\x90V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x03bW`\0\x80\xFD[Pa\x02Na\x03q6`\x04a,\xE4V[`\x15` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x02Ng\x03x-\xAC\xE9\xD9\0\0\x81V[4\x80\x15a\x03\xB6W`\0\x80\xFD[Pa\x02\x91a\x03\xC56`\x04a0\xF4V[a\x0B\xD9V[4\x80\x15a\x03\xD6W`\0\x80\xFD[Pa\x02Na\x03\xE56`\x04a,\xA2V[a\x10\xDCV[4\x80\x15a\x03\xF6W`\0\x80\xFD[Pa\x04%a\x04\x056`\x04a0\x90V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02%V[4\x80\x15a\x04AW`\0\x80\xFD[Pa\x04Ua\x04P6`\x04a1\x90V[a\x10\xF3V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02%V[4\x80\x15a\x04zW`\0\x80\xFD[Pa\x02\x91a\x04\x896`\x04a1\xC3V[a\x11MV[4\x80\x15a\x04\x9AW`\0\x80\xFD[Pa\x02Na\x04\xA96`\x04a,\xE4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x18` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[4\x80\x15a\x04\xE9W`\0\x80\xFD[Pa\x02\x91a\x04\xF86`\x04a2\x0FV[a\x12HV[4\x80\x15a\x05\tW`\0\x80\xFD[Pa\x02Na\x05\x186`\x04a,\xE4V[`\x17` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x02Na\x05P6`\x04a1\x90V[a\x13\xFFV[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02Nb\x01\x86\xA0\x81V[4\x80\x15a\x05xW`\0\x80\xFD[Pa\x02\x91a\x05\x876`\x04a.\xECV[a\x141V[4\x80\x15a\x05\x98W`\0\x80\xFD[P`\x13Ta\x02NV[4\x80\x15a\x05\xADW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02NV[4\x80\x15a\x05\xE0W`\0\x80\xFD[Pa\x02Na\x05\xEF6`\x04a,\xE4V[`\x16` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x06\x18W`\0\x80\xFD[Pa\x06,a\x06'6`\x04a0\x90V[a\x18@V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02%V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x02\x91a\x06\\6`\x04a1\xC3V[a\x191V[4\x80\x15a\x06mW`\0\x80\xFD[Pa\x02\x91a\x06|6`\x04a2\x9BV[a\x1A9V[4\x80\x15a\x06\x8DW`\0\x80\xFD[Pa\x02\x91a\x06\x9C6`\x04a3\nV[a\x1B\x98V[4\x80\x15a\x06\xADW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02NV[a\x02\x91a\x06\xE26`\x04a3\x92V[a\x1D\x1EV[4\x80\x15a\x06\xF3W`\0\x80\xFD[Pa\x02Na\x07\x026`\x04a,\xA2V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`\x13\x81\x81T\x81\x10a\x07$W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91P\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81\x90a\x07\x9C\x90``\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P`\0[`\x10\x81\x10\x15a\x08\x89W\x81`\x01\x16`\x01\x03a\x08/Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x14` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 \x81`\x10\x81\x10a\x07\xFCWa\x07\xFCa3\xCEV[\x01T`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x84\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92Pa\x08pV[\x82`\x03\x82`\x10\x81\x10a\x08CWa\x08Ca3\xCEV[\x01T`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P[`\x01\x91\x90\x91\x1C\x90\x80a\x08\x81\x81a4,V[\x91PPa\x07\xA7V[PP\x92\x91PPV[`\0a\x08\x9D\x8A\x8Aa\x07YV[\x90Pa\x08\xC0\x86\x86\x83` \x8B\x015a\x08\xBBa\x08\xB6\x8Da4dV[a\x1F\xEAV[a *V[\x80\x15a\x08\xDEWPa\x08\xDE\x83\x83\x83` \x88\x015a\x08\xBBa\x08\xB6\x8Aa4dV[a\t\x14W`@Q\x7F\t\xBD\xE39\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86`@\x015\x88`@Q` \x01a\t*\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\twW`@Q\x7F\x19h\xA9\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83` \x015\x87` \x015`\x01a\t\x8D\x91\x90a5qV[\x14a\t\xC4W`@Q\x7F\x9A;\x11\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x0C\x88a\t\xD2\x86\x80a5\x89V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \x8B\x92PPPV[a\n\x15\x88a!\xE6V[\x83`@\x015\x88`@Q` \x01a\n+\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\nxW`@Q\x7F\x98C\x14[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x8C\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x90Ua\n\xDC\x8A\x8A3a(\0V[PPPPPPPPPPV[`\x01a\n\xF6`\x10`\x02a7\x10V[a\x0B\0\x91\x90a7\x1CV[\x81V[`\0a\x0B\x0F\x86\x86a(\xC0V[\x90Pa\x0B\x1C\x83`\x08a5qV[\x82\x10\x15\x80a\x0B*WP` \x83\x11[\x15a\x0BaW`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x81\x81R`\xC0\x85\x90\x1B\x82R`\x08\x95\x90\x95R\x82Q\x82\x82R`\x02\x86R`@\x80\x83 \x85\x84R\x87R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x87R\x80\x83 \x94\x83R\x93\x86R\x83\x82 U\x81\x81R\x93\x84\x90R\x92 U\x92\x91PPV[``\x81\x15a\x0B\xF2Wa\x0B\xEB\x86\x86a)mV[\x90Pa\x0C,V[\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x93PPPP[3`\0\x90\x81R`\x14` \x90\x81R`@\x80\x83 \x8B\x84R\x90\x91R\x80\x82 \x81Qa\x02\0\x81\x01\x92\x83\x90R\x91`\x10\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0CYWPPPPP\x90P`\0`\x15`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8B\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x0C\xDA\x82``\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P32\x14a\r\x1BW`@Q\x7F\xBA\t-\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r+\x82`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\rjW`@Q\x7F\x87\x13\x8D\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rt\x82`\xC0\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\r\xB5W`@Q\x7FGZ%5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x81\x14a\r\xEEW`@Q\x7F`\xF9]Z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xFB\x89\x89\x8D\x88\x86a)\xE6V[\x83Q` \x85\x01`\x88\x82\x04\x88\x14\x15`\x88\x83\x06\x17\x15a\x0E Wc\x07\xB1\xDA\xF1`\0R`\x04`\x1C\xFD[`@Q`\xC8\x81\x01`@R`\0[\x83\x81\x10\x15a\x0E\xD0W\x80\x83\x01\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01RP\x84`\x88\x83\x01R`\x88\x81\x04`\x05\x1B\x8B\x015`\xA8\x83\x01R`\xC8\x82 `\x01\x86\x01\x95P\x85`\0[a\x02\0\x81\x10\x15a\x0E\xC5W`\x01\x82\x16\x15a\x0E\xA5W\x82\x81\x8B\x01Ra\x0E\xC5V[\x89\x81\x01Q`\0\x90\x81R` \x93\x84R`@\x90 \x92`\x01\x92\x90\x92\x1C\x91\x01a\x0E\x88V[PPP`\x88\x01a\x0E-V[PPPP`\x01`\x10`\x02a\x0E\xE4\x91\x90a7\x10V[a\x0E\xEE\x91\x90a7\x1CV[\x81\x11\x15a\x0F'W`@Q\x7Fb)W#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x9Ca\x0F:\x83`@\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[a\x0FJ\x90c\xFF\xFF\xFF\xFF\x16\x8Aa5qV[`@\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x84\x90\x1B\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x17\x17\x90V[\x91P\x84\x15a\x10)Ww\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16B`\xC0\x1B\x17\x91Pa\x0F\xD6\x82`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16a\x0F\xEC\x83`@\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x14a\x10)W`@Q\x7F{\x1D\xAF\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x14` \x90\x81R`@\x80\x83 \x8E\x84R\x90\x91R\x90 a\x10O\x90\x84`\x10a,\x18V[P3`\0\x81\x81R`\x18` \x90\x81R`@\x80\x83 \x8F\x84R\x82R\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x82\x84 `\x04\x82\x04\x01\x80T`\x03\x90\x92\x16`\x08\x02a\x01\0\ng\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x02\x19\x90\x93\x16C\x90\x93\x16\x02\x91\x90\x91\x17\x90U\x83\x83R`\x15\x82R\x80\x83 \x8F\x84R\x90\x91R\x81 \x84\x90U``\x91\x90\x91\x1B\x81R6\x90`\x1476`\x14\x01`\0\xA0PPPPPPPPPPPV[`\x03\x81`\x10\x81\x10a\x10\xECW`\0\x80\xFD[\x01T\x90P\x81V[`\x18` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x11\x1BW`\0\x80\xFD[\x90`\0R` `\0 \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x92P\x92PP\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`D5`\0\x80`\x08\x83\x01\x86\x10a\x11kWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x83\x90\x1B`\x80R`\x88\x83\x86\x827\x86`\x08\x82\x03\x01Q\x91P` `\0\x85\x83`\x02Z\xFA\x90P\x80a\x11\x98W`\0\x80\xFD[P`\0\x80Q~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x80\x82R`\x02` \x90\x81R`@\x80\x84 \x8A\x85R\x82R\x80\x84 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x83\x85R\x82R\x80\x84 \x99\x84R\x98\x81R\x88\x83 \x93\x90\x93U\x81R\x90\x81\x90R\x94\x90\x94 UPPPV[`\0\x80`0\x87`\x007` `\0`0`\0`\x02Z\xFA\x80a\x12pWc\xF9\x11)i`\0R`\x04`\x1C\xFD[`\0Q~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\x80\x81\x81R`\xA0\x8C\x90R`\xC0\x8B\x90R`0\x8A`\xE07`0\x88`\x90\x83\x017`\0\x80`\xC0\x83`\nZ\xFA\x92P\x82a\x12\xF2Wc\t\xBD\xE39`\0R`\x04`\x1C\xFD[`(\x86\x10a\x13\x08Wc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\0`(\x82\x01Rx \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x08\x81\x01\x8B\x90R\x85\x81\x01Q\x93P`0\x8A\x827`0\x81\x01\x9B\x90\x9BRPP`P\x90\x98 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x82R\x80\x83 \x95\x83R\x94\x81R\x84\x82 \x9A\x90\x9AU\x90\x81R\x80\x89R\x91\x90\x91 \x96\x90\x96UPPPPPPV[`\x14` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81`\x10\x81\x10a\x14'W`\0\x80\xFD[\x01T\x92P\x83\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x8B\x84R\x90\x91R\x90 Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\x14\xA4W`@Q\x7F\xC34\xF0i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xAE\x81`\xC0\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x14\xF1W`@Q\x7FU\xD4\xCB\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x15\x1C\x82`\xC0\x1C\x90V[a\x150\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba7\x1CV[\x11a\x15gW`@Q\x7FU\xD4\xCB\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15s\x8B\x8Ba\x07YV[\x90Pa\x15\x8C\x87\x87\x83` \x8C\x015a\x08\xBBa\x08\xB6\x8Ea4dV[\x80\x15a\x15\xAAWPa\x15\xAA\x84\x84\x83` \x89\x015a\x08\xBBa\x08\xB6\x8Ba4dV[a\x15\xE0W`@Q\x7F\t\xBD\xE39\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`@\x015\x89`@Q` \x01a\x15\xF6\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x16CW`@Q\x7F\x19h\xA9\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84` \x015\x88` \x015`\x01a\x16Y\x91\x90a5qV[\x14\x15\x80a\x16\x8BWP`\x01a\x16s\x83``\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[a\x16}\x91\x90a73V[c\xFF\xFF\xFF\xFF\x16\x85` \x015\x14\x15[\x15a\x16\xC2W`@Q\x7F\x9A;\x11\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\xD0\x89a\t\xD2\x87\x80a5\x89V[a\x16\xD9\x89a!\xE6V[`\0a\x16\xE4\x8Aa+9V[~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90P`\0a\x17;\x84`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x01`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x17`\0\x8Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 T`\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPa\x18\r\x84`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[`\0\x83\x81R` \x81\x90R`@\x90 c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90Ua\x181\x8D\x8D\x81a(\0V[PPPPPPPPPPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81\x90`\xFF\x16a\x18\xC9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fpre-image must exist\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P`\0\x83\x81R` \x81\x81R`@\x90\x91 Ta\x18\xE5\x81`\x08a5qV[a\x18\xF0\x85` a5qV[\x10a\x19\x0EW\x83a\x19\x01\x82`\x08a5qV[a\x19\x0B\x91\x90a7\x1CV[\x91P[P`\0\x93\x84R`\x01` \x90\x81R`@\x80\x86 \x94\x86R\x93\x90R\x91\x90\x92 T\x92\x90\x91PV[`D5`\0\x80`\x08\x83\x01\x86\x10a\x19OWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x83\x90\x1B`\x80R`\x88\x83\x86\x827\x80\x87\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01Q\x90\x84\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x8B\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x82R\x80\x83 \x9A\x83R\x99\x81R\x89\x82 \x93\x90\x93U\x90\x81R\x90\x81\x90R\x95\x90\x95 \x91\x90\x91UPPPPV[`\0\x80`\0\x80`\x80\x88``\x1B\x81R\x87`\xC0\x1B`\x14\x82\x01R\x85\x87\x82`\x1C\x017`\x1C\x86\x01\x81 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x93P`@\x88\x02b`!`\x01`?Z\x02\x10\x15a\x1A\xC3Wc\xDDb\x9F\x86`\0R`\x04`\x1C\xFD[`\0\x80\x87\x83`\x1C\x01\x8CZ\xFA\x94P=`\x01\x01\x91P`\x08\x82\x01\x8A\x10a\x1A\xEEWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x82\x90\x1B\x81R`\x08\x01\x84\x81S=`\0`\x01\x83\x01>\x89\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01Q`\0\x84\x81R`\x02` \x90\x81R`@\x80\x83 \x8D\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x87\x84R\x82R\x80\x83 \x9C\x83R\x9B\x81R\x8B\x82 \x92\x90\x92U\x93\x84R\x83\x90R\x97\x90\x91 \x96\x90\x96UPPPPPPV[`\0a\x1B\xA4\x86\x86a\x07YV[\x90Pa\x1B\xBD\x83\x83\x83` \x88\x015a\x08\xBBa\x08\xB6\x8Aa4dV[a\x1B\xF3W`@Q\x7F\t\xBD\xE39\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x015\x15a\x1C/W`@Q\x7F\x9A;\x11\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C7a,VV[a\x1CE\x81a\t\xD2\x87\x80a5\x89V[a\x1CN\x81a!\xE6V[\x84`@\x015\x81`@Q` \x01a\x1Cd\x91\x90a53V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x1C\xB1W`@Q\x7F\x98C\x14[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x90Ua\x1D\x15\x87\x873a(\0V[PPPPPPPV[g\x03x-\xAC\xE9\xD9\0\x004\x10\x15a\x1D`W`@Q\x7F\xE9,F\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[32\x14a\x1D\x99W`@Q\x7F\xBA\t-\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xA4\x81`\x08a7XV[c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x10a\x1D\xE8W`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1EHW`@Q\x7F{\x1D\xAF\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x15` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 Ta\x1Es\x81`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x15a\x1E\xB0W`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1B\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17\x173`\0\x81\x81R`\x15` \x90\x81R`@\x80\x83 \x89\x84R\x82R\x80\x83 \x94\x90\x94U\x83Q\x80\x85\x01\x85R\x83\x81R\x80\x82\x01\x89\x81R`\x13\x80T`\x01\x81\x01\x82U\x90\x85R\x91Q\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x90`\x02\x90\x93\x02\x92\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UQ\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x91\x90\x91\x01U\x91\x81R`\x16\x82R\x82\x81 \x96\x81R\x95\x90R\x90\x93 4\x90UPPPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a \r\x93\x92\x91\x90a7\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0[`\x10\x81\x10\x15a ~W\x80`\x05\x1B\x88\x015`\x01\x86\x83\x1C\x16`\x01\x81\x14a cW`\0\x84\x81R` \x83\x90R`@\x90 \x93Pa tV[`\0\x82\x81R` \x85\x90R`@\x90 \x93P[PP`\x01\x01a 0V[P\x90\x93\x14\x95\x94PPPPPV[`\x88\x81Q\x14a \x99W`\0\x80\xFD[` \x81\x01` \x83\x01a!\x1AV[\x82`\x03\x1B\x82\x01Q\x80`\0\x1A\x81`\x01\x1A`\x08\x1B\x17\x81`\x02\x1A`\x10\x1B\x82`\x03\x1A`\x18\x1B\x17\x17\x81`\x04\x1A` \x1B\x82`\x05\x1A`(\x1B\x17\x82`\x06\x1A`0\x1B\x83`\x07\x1A`8\x1B\x17\x17\x17\x90Pa!\x14\x81a \xFF\x86\x85`\x05\x91\x90\x91\x1B\x01Q\x90V[\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05\x86\x90\x1B\x84\x01RV[PPPPV[a!&`\0\x83\x83a \xA6V[a!2`\x01\x83\x83a \xA6V[a!>`\x02\x83\x83a \xA6V[a!J`\x03\x83\x83a \xA6V[a!V`\x04\x83\x83a \xA6V[a!b`\x05\x83\x83a \xA6V[a!n`\x06\x83\x83a \xA6V[a!z`\x07\x83\x83a \xA6V[a!\x86`\x08\x83\x83a \xA6V[a!\x92`\t\x83\x83a \xA6V[a!\x9E`\n\x83\x83a \xA6V[a!\xAA`\x0B\x83\x83a \xA6V[a!\xB6`\x0C\x83\x83a \xA6V[a!\xC2`\r\x83\x83a \xA6V[a!\xCE`\x0E\x83\x83a \xA6V[a!\xDA`\x0F\x83\x83a \xA6V[a!\x14`\x10\x83\x83a \xA6V[`@\x80Qx\x01\0\0\0\0\0\0\x80\x82\x80\0\0\0\0\0\x80\x8A\x80\0\0\0\x80\0\x80\0` \x82\x01Ry\x80\x8B\0\0\0\0\x80\0\0\x01\x80\0\0\0\x80\0\x80\x81\x80\0\0\0\0\0\x80\t\x91\x81\x01\x91\x90\x91Rx\x8A\0\0\0\0\0\0\0\x88\0\0\0\0\x80\0\x80\t\0\0\0\0\x80\0\0\n``\x82\x01R{\x80\0\x80\x8B\x80\0\0\0\0\0\0\x8B\x80\0\0\0\0\0\x80\x89\x80\0\0\0\0\0\x80\x03`\x80\x82\x01R\x7F\x80\0\0\0\0\0\x80\x02\x80\0\0\0\0\0\0\x80\0\0\0\0\0\0\x80\n\x80\0\0\0\x80\0\0\n`\xA0\x82\x01R\x7F\x80\0\0\0\x80\0\x80\x81\x80\0\0\0\0\0\x80\x80\0\0\0\0\x80\0\0\x01\x80\0\0\0\x80\0\x80\x08`\xC0\x82\x01R`\0\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x82\x01` \x82\x01a&\xE0V[`\x05\x83\x90\x1B\x81\x01Q\x85\x18`@\x85\x90\x03\x81\x90\x1C\x90\x85\x1B\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x05\x84\x90\x1B\x83\x01R[PPPPPPV[a\x02\x80\x81\x01Qa\x01\xE0\x82\x01Qa\x01@\x83\x01Q`\xA0\x84\x01Q\x84Q\x18\x18\x91\x18\x18a\x02\xA0\x82\x01Qa\x02\0\x83\x01Qa\x01`\x84\x01Q`\xC0\x85\x01Q` \x86\x01Q\x18\x18\x91\x18\x18a\x02\xC0\x83\x01Qa\x02 \x84\x01Qa\x01\x80\x85\x01Q`\xE0\x86\x01Q`@\x87\x01Q\x18\x18\x91\x18\x18a\x02\xE0\x84\x01Qa\x02@\x85\x01Qa\x01\xA0\x86\x01Qa\x01\0\x87\x01Q``\x88\x01Q\x18\x18\x91\x18\x18a\x03\0\x85\x01Qa\x02`\x86\x01Qa\x01\xC0\x87\x01Qa\x01 \x88\x01Q`\x80\x89\x01Q\x18\x18\x91\x18\x18\x80\x84`?\x1Ca#\xCB\x86`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x85\x84`?\x1Ca#\xE6\x86`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x85\x84`?\x1Ca$\x01\x86`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x95P\x84\x83`?\x1Ca$\x1E\x85`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x18\x18\x94P\x83\x87`?\x1Ca$;\x89`\x01\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[` \x8B\x01Q\x8BQ\x86\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8CR\x91\x18\x91\x90\x91\x18\x97P\x81\x18`\x01\x81\x90\x1B`?\x91\x90\x91\x1C\x18\x93P\x87\x92Pa${\x81`,`\x06`\x01\x87a\"\xDCV[a$\x8B\x87`\x14`\t`\x06\x87a\"\xDCV[a$\x9B\x86`=`\x16`\t\x87a\"\xDCV[a$\xAB\x87`'`\x0E`\x16\x87a\"\xDCV[a$\xBB\x82`\x12`\x14`\x0E\x87a\"\xDCV[a$\xCB\x86`>`\x02`\x14\x87a\"\xDCV[a$\xDB\x86`+`\x0C`\x02\x87a\"\xDCV[a$\xEB\x85`\x19`\r`\x0C\x87a\"\xDCV[a$\xFB\x87`\x08`\x13`\r\x87a\"\xDCV[a%\x0B\x85`8`\x17`\x13\x87a\"\xDCV[a%\x1B\x82`)`\x0F`\x17\x87a\"\xDCV[a%+\x87`\x1B`\x04`\x0F\x87a\"\xDCV[a%;\x87`\x0E`\x18`\x04\x87a\"\xDCV[a%K\x81`\x02`\x15`\x18\x87a\"\xDCV[a%[\x85`7`\x08`\x15\x87a\"\xDCV[a%k\x81`-`\x10`\x08\x87a\"\xDCV[a%{\x82`$`\x05`\x10\x87a\"\xDCV[a%\x8B\x85`\x1C`\x03`\x05\x87a\"\xDCV[a%\x9B\x85`\x15`\x12`\x03\x87a\"\xDCV[a%\xAB\x86`\x0F`\x11`\x12\x87a\"\xDCV[a%\xBB\x81`\n`\x0B`\x11\x87a\"\xDCV[Pa%\xCC\x85`\x06`\x07`\x0B\x86a\"\xDCV[a%\xDC\x81`\x03`\n`\x07\x86a\"\xDCV[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x01@\x82\x01Ra#\x06V[`\x05\x82\x81\x1B\x82\x01\x80Q`\x01\x85\x01\x83\x1B\x84\x01\x80Q`\x02\x87\x01\x85\x1B\x86\x01\x80Q`\x03\x89\x01\x87\x1B\x88\x01\x80Q`\x04\x8B\x01\x90\x98\x1B\x89\x01\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x19\x85\x16\x89\x18\x81\x16\x90\x99R\x83\x19\x8A\x16\x86\x18\x89\x16\x90\x96R\x88\x19\x86\x16\x83\x18\x88\x16\x90\x93R\x84\x19\x86\x16\x88\x18\x87\x16\x90R\x84\x19\x83\x16\x84\x18\x90\x95\x16\x90R\x91\x93\x91\x92\x91\x90a\x1D\x15V[a&z`\0\x82a%\xF3V[a&\x85`\x05\x82a%\xF3V[a&\x90`\n\x82a%\xF3V[a&\x9B`\x0F\x82a%\xF3V[a&\xA6`\x14\x82a%\xF3V[PV[a&\xB2\x81a#\x0EV[a&\xBB\x81a&oV[`\x03\x83\x90\x1B\x82\x01Q\x81Q`\xC0\x91\x90\x91\x1C\x90a!\x14\x90\x82\x18g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83RV[a&\xEC`\0\x82\x84a&\xA9V[a&\xF8`\x01\x82\x84a&\xA9V[a'\x04`\x02\x82\x84a&\xA9V[a'\x10`\x03\x82\x84a&\xA9V[a'\x1C`\x04\x82\x84a&\xA9V[a'(`\x05\x82\x84a&\xA9V[a'4`\x06\x82\x84a&\xA9V[a'@`\x07\x82\x84a&\xA9V[a'L`\x08\x82\x84a&\xA9V[a'X`\t\x82\x84a&\xA9V[a'd`\n\x82\x84a&\xA9V[a'p`\x0B\x82\x84a&\xA9V[a'|`\x0C\x82\x84a&\xA9V[a'\x88`\r\x82\x84a&\xA9V[a'\x94`\x0E\x82\x84a&\xA9V[a'\xA0`\x0F\x82\x84a&\xA9V[a'\xAC`\x10\x82\x84a&\xA9V[a'\xB8`\x11\x82\x84a&\xA9V[a'\xC4`\x12\x82\x84a&\xA9V[a'\xD0`\x13\x82\x84a&\xA9V[a'\xDC`\x14\x82\x84a&\xA9V[a'\xE8`\x15\x82\x84a&\xA9V[a'\xF4`\x16\x82\x84a&\xA9V[a!\x14`\x17\x82\x84a&\xA9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x16` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T\x90\x83\x90U\x90Q\x90\x92\x84\x16\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a(yW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a(~V[``\x91P[PP\x90P\x80a(\xB9W`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17a)f\x81\x83`@\x80Q`\0\x93\x84R3` R\x91\x81R``\x90\x92 \x91R~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x93\x92PPPV[```@Q\x90P\x81` \x82\x01\x81\x81\x01\x82\x86\x837`\x88\x83\x06\x80\x80\x15a)\xB6W`\x88\x82\x90\x03\x85\x01\x84\x81\x01\x84\x81\x03\x806\x877P`\x80`\x01\x82\x03SP`\x01\x84Q`\0\x1A\x17\x84S\x86Ra)\xCDV[`\x886\x847`\x01\x83S`\x80`\x87\x84\x01S`\x88\x85\x01\x86R[PPPPP`\x1F\x19`?\x82Q\x01\x16\x81\x01`@R\x92\x91PPV[`\0a)\xF8\x82`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0a*\x16\x83`\x80\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P`\0a*0\x84`@\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90P`\x08\x83\x10\x80\x15a*FWP\x80\x15[\x15a*zW`\xC0\x82\x90\x1B`\0\x90\x81R\x885`\x08R\x83Q3\x82R`\x17` \x90\x81R`@\x80\x84 \x8A\x85R\x90\x91R\x90\x91 Ua+/V[`\x08\x83\x10\x15\x80\x15a*\x98WP\x80a*\x92`\x08\x85a7\x1CV[\x93P\x83\x10\x15[\x80\x15a*\xACWPa*\xA9\x87\x82a5qV[\x83\x10[\x15a+/W`\0a*\xBD\x82\x85a7\x1CV[\x90P\x87a*\xCB\x82` a5qV[\x10\x15\x80\x15a*\xD7WP\x85\x15[\x15a+\x0EW`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x17` \x90\x81R`@\x80\x83 \x8A\x84R\x90\x91R\x90 \x90\x89\x015\x90U[PPPPPPPPV[`\0a+\xBCV[f\xFF\0\xFF\0\xFF\0\xFF\x81`\x08\x1C\x16g\xFF\0\xFF\0\xFF\0\xFF\0a+j\x83`\x08\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x17\x90Pe\xFF\xFF\0\0\xFF\xFF\x81`\x10\x1C\x16g\xFF\xFF\0\0\xFF\xFF\0\0a+\x97\x83`\x10\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x17\x90P`\0\x81` \x1Ca+\xB5\x83` \x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x17\x92\x91PPV[`\x80\x82\x01Q` \x83\x01\x90a+\xD4\x90a+@V[a+@V[`@\x82\x01Qa+\xE2\x90a+@V[`@\x1B\x17a+\xFAa+\xCF`\x01\x84`\x05\x91\x90\x91\x1B\x01Q\x90V[\x82Q`\x80\x91\x90\x91\x1B\x90a,\x0C\x90a+@V[`\xC0\x1B\x17\x17\x93\x92PPPV[\x82`\x10\x81\x01\x92\x82\x15a,FW\x91` \x02\x82\x01[\x82\x81\x11\x15a,FW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a,+V[Pa,R\x92\x91Pa,nV[P\x90V[`@Q\x80` \x01`@R\x80a,ia,\x83V[\x90R\x90V[[\x80\x82\x11\x15a,RW`\0\x81U`\x01\x01a,oV[`@Q\x80a\x03 \x01`@R\x80`\x19\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a,\xB4W`\0\x80\xFD[P5\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xDFW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xF7W`\0\x80\xFD[a-\0\x83a,\xBBV[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Qa\x03 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-aWa-aa-\x0EV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-aWa-aa-\x0EV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a-\xD1Wa-\xD1a-\x0EV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xDFW`\0\x80\xFD[`\0a\x03 \x80\x83\x85\x03\x12\x15a.\x05W`\0\x80\xFD[`@Q` \x80\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.)Wa.)a-\x0EV[\x80`@RP\x81\x93P\x85`\x1F\x86\x01\x12a.@W`\0\x80\xFD[a.Ha-=V[\x92\x85\x01\x92\x80\x87\x85\x11\x15a.ZW`\0\x80\xFD[\x86[\x85\x81\x10\x15a.zWa.m\x81a-\xD9V[\x83R\x91\x83\x01\x91\x83\x01a.\\V[P\x90\x92RP\x91\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a.\x9AW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a.\xB2W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xCAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a.\xE5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x03\xE0\x8A\x8C\x03\x12\x15a/\x0BW`\0\x80\xFD[a/\x14\x8Aa,\xBBV[\x98P` \x8A\x015\x97Pa/*\x8B`@\x8C\x01a-\xF1V[\x96Pa\x03`\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a/HW`\0\x80\xFD[a/T\x8D\x83\x8E\x01a.\x88V[\x97Pa\x03\x80\x8C\x015\x91P\x80\x82\x11\x15a/kW`\0\x80\xFD[a/w\x8D\x83\x8E\x01a.\xA0V[\x90\x97P\x95Pa\x03\xA0\x8C\x015\x91P\x80\x82\x11\x15a/\x91W`\0\x80\xFD[a/\x9D\x8D\x83\x8E\x01a.\x88V[\x94Pa\x03\xC0\x8C\x015\x91P\x80\x82\x11\x15a/\xB4W`\0\x80\xFD[Pa/\xC1\x8C\x82\x8D\x01a.\xA0V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a/\xF0W`\0\x80\xFD[PP\x835\x95` \x85\x015\x95P`@\x85\x015\x94``\x81\x015\x94P`\x80\x015\x92P\x90PV[`\0[\x83\x81\x10\x15a0.W\x81\x81\x01Q\x83\x82\x01R` \x01a0\x16V[\x83\x81\x11\x15a!\x14WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra0^\x81`@\x85\x01` \x87\x01a0\x13V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a0\xA3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a0\xC4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xDCW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a.\xE5W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a1\x0FW`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a15W`\0\x80\xFD[a1A\x8B\x83\x8C\x01a0\xB2V[\x90\x97P\x95P``\x8A\x015\x91P\x80\x82\x11\x15a1ZW`\0\x80\xFD[Pa1g\x8A\x82\x8B\x01a.\xA0V[\x90\x94P\x92PP`\x80\x88\x015\x80\x15\x15\x81\x14a1\x80W`\0\x80\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\xA5W`\0\x80\xFD[a1\xAE\x84a,\xBBV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xD8W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xF6W`\0\x80\xFD[a2\x02\x86\x82\x87\x01a0\xB2V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a2*W`\0\x80\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a2PW`\0\x80\xFD[a2\\\x8B\x83\x8C\x01a0\xB2V[\x90\x97P\x95P``\x8A\x015\x91P\x80\x82\x11\x15a2uW`\0\x80\xFD[Pa2\x82\x8A\x82\x8B\x01a0\xB2V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96`\x80\x90\x95\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a2\xB3W`\0\x80\xFD[\x855\x94Pa2\xC3` \x87\x01a,\xBBV[\x93Pa2\xD1`@\x87\x01a-\xD9V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xEDW`\0\x80\xFD[a2\xF9\x88\x82\x89\x01a0\xB2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a3\"W`\0\x80\xFD[a3+\x86a,\xBBV[\x94P` \x86\x015\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a3OW`\0\x80\xFD[a3[\x89\x83\x8A\x01a.\x88V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a3qW`\0\x80\xFD[Pa2\xF9\x88\x82\x89\x01a.\xA0V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xDFW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xA7W`\0\x80\xFD[\x835\x92Pa3\xB7` \x85\x01a3~V[\x91Pa3\xC5`@\x85\x01a3~V[\x90P\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a4]Wa4]a3\xFDV[P`\x01\x01\x90V[`\0``\x826\x03\x12\x15a4vW`\0\x80\xFD[a4~a-gV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4\x96W`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12a4\xA9W`\0\x80\xFD[\x815` \x82\x82\x11\x15a4\xBDWa4\xBDa-\x0EV[a4\xED\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01a-\x8AV[\x92P\x81\x83R6\x81\x83\x86\x01\x01\x11\x15a5\x03W`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x91\x83\x01\x81\x01\x91\x90\x91R\x90\x83R\x84\x81\x015\x90\x83\x01RP`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[\x81Qa\x03 \x82\x01\x90\x82`\0[`\x19\x81\x10\x15a5hW\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a5?V[PPP\x92\x91PPV[`\0\x82\x19\x82\x11\x15a5\x84Wa5\x84a3\xFDV[P\x01\x90V[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a5\xBEW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a5\xD9W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a.\xE5W`\0\x80\xFD[`\x01\x81\x81[\x80\x85\x11\x15a6GW\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a6-Wa6-a3\xFDV[\x80\x85\x16\x15a6:W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a5\xF3V[P\x92P\x92\x90PV[`\0\x82a6^WP`\x01a7\nV[\x81a6kWP`\0a7\nV[\x81`\x01\x81\x14a6\x81W`\x02\x81\x14a6\x8BWa6\xA7V[`\x01\x91PPa7\nV[`\xFF\x84\x11\x15a6\x9CWa6\x9Ca3\xFDV[PP`\x01\x82\x1Ba7\nV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a6\xCAWP\x81\x81\na7\nV[a6\xD4\x83\x83a5\xEEV[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a7\x06Wa7\x06a3\xFDV[\x02\x90P[\x92\x91PPV[`\0a)f\x83\x83a6OV[`\0\x82\x82\x10\x15a7.Wa7.a3\xFDV[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a7PWa7Pa3\xFDV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a7wWa7wa3\xFDV[\x01\x94\x93PPPPV[`\0\x84Qa7\x92\x81\x84` \x89\x01a0\x13V[\x91\x90\x91\x01\x92\x83RP` \x82\x01R`@\x01\x91\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LPPMetaData(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<LPPMetaData>
        for alloy::sol_types::private::FixedBytes<32> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl LPPMetaData {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::FixedBytes<32>,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::FixedBytes<32> {
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
        impl From<alloy::sol_types::private::FixedBytes<32>> for LPPMetaData {
            fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<LPPMetaData> for alloy::sol_types::private::FixedBytes<32> {
            fn from(value: LPPMetaData) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for LPPMetaData {
            type RustType = alloy::sol_types::private::FixedBytes<32>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for LPPMetaData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Leaf { bytes input; uint256 index; bytes32 stateCommitment; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Leaf {
        #[allow(missing_docs)]
        pub input: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub stateCommitment: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<Leaf> for UnderlyingRustTuple<'_> {
            fn from(value: Leaf) -> Self {
                (value.input, value.index, value.stateCommitment)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Leaf {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    input: tuple.0,
                    index: tuple.1,
                    stateCommitment: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Leaf {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Leaf {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.input,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.stateCommitment),
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
        impl alloy_sol_types::SolType for Leaf {
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
        impl alloy_sol_types::SolStruct for Leaf {
            const NAME: &'static str = "Leaf";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Leaf(bytes input,uint256 index,bytes32 stateCommitment)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.input,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.index)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stateCommitment,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Leaf {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.input,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.index)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stateCommitment,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.input,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.index,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stateCommitment,
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
    /**Custom error with signature `ActiveProposal()` and selector `0x55d4cbf9`.
```solidity
error ActiveProposal();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ActiveProposal;
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
        impl ::core::convert::From<ActiveProposal> for UnderlyingRustTuple<'_> {
            fn from(value: ActiveProposal) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ActiveProposal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ActiveProposal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ActiveProposal()";
            const SELECTOR: [u8; 4] = [85u8, 212u8, 203u8, 249u8];
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
    /**Custom error with signature `AlreadyFinalized()` and selector `0x475a2535`.
```solidity
error AlreadyFinalized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyFinalized;
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
        impl ::core::convert::From<AlreadyFinalized> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyFinalized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyFinalized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyFinalized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyFinalized()";
            const SELECTOR: [u8; 4] = [71u8, 90u8, 37u8, 53u8];
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
    /**Custom error with signature `AlreadyInitialized()` and selector `0x0dc149f0`.
```solidity
error AlreadyInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInitialized;
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
        impl ::core::convert::From<AlreadyInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyInitialized()";
            const SELECTOR: [u8; 4] = [13u8, 193u8, 73u8, 240u8];
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
    /**Custom error with signature `BadProposal()` and selector `0xc334f069`.
```solidity
error BadProposal();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BadProposal;
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
        impl ::core::convert::From<BadProposal> for UnderlyingRustTuple<'_> {
            fn from(value: BadProposal) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BadProposal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BadProposal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BadProposal()";
            const SELECTOR: [u8; 4] = [195u8, 52u8, 240u8, 105u8];
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
    /**Custom error with signature `BondTransferFailed()` and selector `0x83e6cc6b`.
```solidity
error BondTransferFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BondTransferFailed;
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
        impl ::core::convert::From<BondTransferFailed> for UnderlyingRustTuple<'_> {
            fn from(value: BondTransferFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BondTransferFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BondTransferFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BondTransferFailed()";
            const SELECTOR: [u8; 4] = [131u8, 230u8, 204u8, 107u8];
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
    /**Custom error with signature `InsufficientBond()` and selector `0xe92c469f`.
```solidity
error InsufficientBond();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientBond;
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
        impl ::core::convert::From<InsufficientBond> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientBond) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientBond {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientBond {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientBond()";
            const SELECTOR: [u8; 4] = [233u8, 44u8, 70u8, 159u8];
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
    /**Custom error with signature `InvalidInputSize()` and selector `0x7b1dafd1`.
```solidity
error InvalidInputSize();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInputSize;
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
        impl ::core::convert::From<InvalidInputSize> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInputSize) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInputSize {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInputSize {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInputSize()";
            const SELECTOR: [u8; 4] = [123u8, 29u8, 175u8, 209u8];
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
    /**Custom error with signature `InvalidPreimage()` and selector `0x1968a902`.
```solidity
error InvalidPreimage();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPreimage;
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
        impl ::core::convert::From<InvalidPreimage> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPreimage) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPreimage {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPreimage {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPreimage()";
            const SELECTOR: [u8; 4] = [25u8, 104u8, 169u8, 2u8];
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
    /**Custom error with signature `InvalidProof()` and selector `0x09bde339`.
```solidity
error InvalidProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidProof;
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
        impl ::core::convert::From<InvalidProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidProof()";
            const SELECTOR: [u8; 4] = [9u8, 189u8, 227u8, 57u8];
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
    /**Custom error with signature `NotEOA()` and selector `0xba092d16`.
```solidity
error NotEOA();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotEOA;
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
        impl ::core::convert::From<NotEOA> for UnderlyingRustTuple<'_> {
            fn from(value: NotEOA) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotEOA {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotEOA {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotEOA()";
            const SELECTOR: [u8; 4] = [186u8, 9u8, 45u8, 22u8];
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
    /**Custom error with signature `NotInitialized()` and selector `0x87138d5c`.
```solidity
error NotInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitialized;
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
        impl ::core::convert::From<NotInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: NotInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitialized()";
            const SELECTOR: [u8; 4] = [135u8, 19u8, 141u8, 92u8];
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
    /**Custom error with signature `PartOffsetOOB()` and selector `0xfe254987`.
```solidity
error PartOffsetOOB();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PartOffsetOOB;
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
        impl ::core::convert::From<PartOffsetOOB> for UnderlyingRustTuple<'_> {
            fn from(value: PartOffsetOOB) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PartOffsetOOB {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PartOffsetOOB {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PartOffsetOOB()";
            const SELECTOR: [u8; 4] = [254u8, 37u8, 73u8, 135u8];
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
    /**Custom error with signature `PostStateMatches()` and selector `0x9843145b`.
```solidity
error PostStateMatches();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PostStateMatches;
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
        impl ::core::convert::From<PostStateMatches> for UnderlyingRustTuple<'_> {
            fn from(value: PostStateMatches) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PostStateMatches {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PostStateMatches {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PostStateMatches()";
            const SELECTOR: [u8; 4] = [152u8, 67u8, 20u8, 91u8];
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
    /**Custom error with signature `StatesNotContiguous()` and selector `0x9a3b1199`.
```solidity
error StatesNotContiguous();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StatesNotContiguous;
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
        impl ::core::convert::From<StatesNotContiguous> for UnderlyingRustTuple<'_> {
            fn from(value: StatesNotContiguous) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StatesNotContiguous {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StatesNotContiguous {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StatesNotContiguous()";
            const SELECTOR: [u8; 4] = [154u8, 59u8, 17u8, 153u8];
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
    /**Custom error with signature `TreeSizeOverflow()` and selector `0x62295723`.
```solidity
error TreeSizeOverflow();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TreeSizeOverflow;
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
        impl ::core::convert::From<TreeSizeOverflow> for UnderlyingRustTuple<'_> {
            fn from(value: TreeSizeOverflow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TreeSizeOverflow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TreeSizeOverflow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TreeSizeOverflow()";
            const SELECTOR: [u8; 4] = [98u8, 41u8, 87u8, 35u8];
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
    /**Custom error with signature `WrongStartingBlock()` and selector `0x60f95d5a`.
```solidity
error WrongStartingBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongStartingBlock;
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
        impl ::core::convert::From<WrongStartingBlock> for UnderlyingRustTuple<'_> {
            fn from(value: WrongStartingBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongStartingBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongStartingBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongStartingBlock()";
            const SELECTOR: [u8; 4] = [96u8, 249u8, 93u8, 90u8];
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
    /**Constructor`.
```solidity
constructor(uint256 _minProposalSize, uint256 _challengePeriod);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _minProposalSize: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _challengePeriod: alloy::sol_types::private::primitives::aliases::U256,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._minProposalSize, value._challengePeriod)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _minProposalSize: tuple.0,
                        _challengePeriod: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._minProposalSize),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._challengePeriod),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `KECCAK_TREE_DEPTH()` and selector `0x2055b36b`.
```solidity
function KECCAK_TREE_DEPTH() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KECCAK_TREE_DEPTHCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`KECCAK_TREE_DEPTH()`](KECCAK_TREE_DEPTHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KECCAK_TREE_DEPTHReturn {
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
            impl ::core::convert::From<KECCAK_TREE_DEPTHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: KECCAK_TREE_DEPTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for KECCAK_TREE_DEPTHCall {
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
            impl ::core::convert::From<KECCAK_TREE_DEPTHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: KECCAK_TREE_DEPTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for KECCAK_TREE_DEPTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for KECCAK_TREE_DEPTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KECCAK_TREE_DEPTH()";
            const SELECTOR: [u8; 4] = [32u8, 85u8, 179u8, 107u8];
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
                        let r: KECCAK_TREE_DEPTHReturn = r.into();
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
                        let r: KECCAK_TREE_DEPTHReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MAX_LEAF_COUNT()` and selector `0x4d52b4c9`.
```solidity
function MAX_LEAF_COUNT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_LEAF_COUNTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_LEAF_COUNT()`](MAX_LEAF_COUNTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_LEAF_COUNTReturn {
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
            impl ::core::convert::From<MAX_LEAF_COUNTCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_LEAF_COUNTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_LEAF_COUNTCall {
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
            impl ::core::convert::From<MAX_LEAF_COUNTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_LEAF_COUNTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_LEAF_COUNTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_LEAF_COUNTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_LEAF_COUNT()";
            const SELECTOR: [u8; 4] = [77u8, 82u8, 180u8, 201u8];
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
                        let r: MAX_LEAF_COUNTReturn = r.into();
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
                        let r: MAX_LEAF_COUNTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MIN_BOND_SIZE()` and selector `0x7051472e`.
```solidity
function MIN_BOND_SIZE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_BOND_SIZECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MIN_BOND_SIZE()`](MIN_BOND_SIZECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_BOND_SIZEReturn {
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
            impl ::core::convert::From<MIN_BOND_SIZECall> for UnderlyingRustTuple<'_> {
                fn from(value: MIN_BOND_SIZECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MIN_BOND_SIZECall {
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
            impl ::core::convert::From<MIN_BOND_SIZEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MIN_BOND_SIZEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MIN_BOND_SIZEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MIN_BOND_SIZECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MIN_BOND_SIZE()";
            const SELECTOR: [u8; 4] = [112u8, 81u8, 71u8, 46u8];
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
                        let r: MIN_BOND_SIZEReturn = r.into();
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
                        let r: MIN_BOND_SIZEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `PRECOMPILE_CALL_RESERVED_GAS()` and selector `0xb5e7154c`.
```solidity
function PRECOMPILE_CALL_RESERVED_GAS() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRECOMPILE_CALL_RESERVED_GASCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`PRECOMPILE_CALL_RESERVED_GAS()`](PRECOMPILE_CALL_RESERVED_GASCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRECOMPILE_CALL_RESERVED_GASReturn {
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
            impl ::core::convert::From<PRECOMPILE_CALL_RESERVED_GASCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: PRECOMPILE_CALL_RESERVED_GASCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PRECOMPILE_CALL_RESERVED_GASCall {
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
            impl ::core::convert::From<PRECOMPILE_CALL_RESERVED_GASReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PRECOMPILE_CALL_RESERVED_GASReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PRECOMPILE_CALL_RESERVED_GASReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PRECOMPILE_CALL_RESERVED_GASCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PRECOMPILE_CALL_RESERVED_GAS()";
            const SELECTOR: [u8; 4] = [181u8, 231u8, 21u8, 76u8];
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
                        let r: PRECOMPILE_CALL_RESERVED_GASReturn = r.into();
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
                        let r: PRECOMPILE_CALL_RESERVED_GASReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addLeavesLPP(uint256,uint256,bytes,bytes32[],bool)` and selector `0x7917de1d`.
```solidity
function addLeavesLPP(uint256 _uuid, uint256 _inputStartBlock, bytes memory _input, bytes32[] memory _stateCommitments, bool _finalize) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLeavesLPPCall {
        #[allow(missing_docs)]
        pub _uuid: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _inputStartBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _input: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _stateCommitments: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub _finalize: bool,
    }
    ///Container type for the return parameters of the [`addLeavesLPP(uint256,uint256,bytes,bytes32[],bool)`](addLeavesLPPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLeavesLPPReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
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
            impl ::core::convert::From<addLeavesLPPCall> for UnderlyingRustTuple<'_> {
                fn from(value: addLeavesLPPCall) -> Self {
                    (
                        value._uuid,
                        value._inputStartBlock,
                        value._input,
                        value._stateCommitments,
                        value._finalize,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLeavesLPPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _uuid: tuple.0,
                        _inputStartBlock: tuple.1,
                        _input: tuple.2,
                        _stateCommitments: tuple.3,
                        _finalize: tuple.4,
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
            impl ::core::convert::From<addLeavesLPPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addLeavesLPPReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLeavesLPPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addLeavesLPPReturn {
            fn _tokenize(
                &self,
            ) -> <addLeavesLPPCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addLeavesLPPCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addLeavesLPPReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addLeavesLPP(uint256,uint256,bytes,bytes32[],bool)";
            const SELECTOR: [u8; 4] = [121u8, 23u8, 222u8, 29u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._uuid),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._inputStartBlock),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._input,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._stateCommitments),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._finalize,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addLeavesLPPReturn::_tokenize(ret)
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
    /**Function with signature `challengeFirstLPP(address,uint256,(bytes,uint256,bytes32),bytes32[])` and selector `0xec5efcbc`.
```solidity
function challengeFirstLPP(address _claimant, uint256 _uuid, Leaf memory _postState, bytes32[] memory _postStateProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeFirstLPPCall {
        #[allow(missing_docs)]
        pub _claimant: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _uuid: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _postState: <Leaf as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _postStateProof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`challengeFirstLPP(address,uint256,(bytes,uint256,bytes32),bytes32[])`](challengeFirstLPPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeFirstLPPReturn {}
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
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <Leaf as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<challengeFirstLPPCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeFirstLPPCall) -> Self {
                    (
                        value._claimant,
                        value._uuid,
                        value._postState,
                        value._postStateProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeFirstLPPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimant: tuple.0,
                        _uuid: tuple.1,
                        _postState: tuple.2,
                        _postStateProof: tuple.3,
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
            impl ::core::convert::From<challengeFirstLPPReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeFirstLPPReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeFirstLPPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl challengeFirstLPPReturn {
            fn _tokenize(
                &self,
            ) -> <challengeFirstLPPCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeFirstLPPCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = challengeFirstLPPReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengeFirstLPP(address,uint256,(bytes,uint256,bytes32),bytes32[])";
            const SELECTOR: [u8; 4] = [236u8, 94u8, 252u8, 188u8];
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
                        &self._claimant,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._uuid),
                    <Leaf as alloy_sol_types::SolType>::tokenize(&self._postState),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._postStateProof),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                challengeFirstLPPReturn::_tokenize(ret)
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
    /**Function with signature `challengeLPP(address,uint256,(uint64[25]),(bytes,uint256,bytes32),bytes32[],(bytes,uint256,bytes32),bytes32[])` and selector `0x3909af5c`.
```solidity
function challengeLPP(address _claimant, uint256 _uuid, LibKeccak.StateMatrix memory _stateMatrix, Leaf memory _preState, bytes32[] memory _preStateProof, Leaf memory _postState, bytes32[] memory _postStateProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeLPPCall {
        #[allow(missing_docs)]
        pub _claimant: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _uuid: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _stateMatrix: <LibKeccak::StateMatrix as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _preState: <Leaf as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _preStateProof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub _postState: <Leaf as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _postStateProof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`challengeLPP(address,uint256,(uint64[25]),(bytes,uint256,bytes32),bytes32[],(bytes,uint256,bytes32),bytes32[])`](challengeLPPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeLPPReturn {}
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
                LibKeccak::StateMatrix,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <LibKeccak::StateMatrix as alloy::sol_types::SolType>::RustType,
                <Leaf as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                <Leaf as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<challengeLPPCall> for UnderlyingRustTuple<'_> {
                fn from(value: challengeLPPCall) -> Self {
                    (
                        value._claimant,
                        value._uuid,
                        value._stateMatrix,
                        value._preState,
                        value._preStateProof,
                        value._postState,
                        value._postStateProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengeLPPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimant: tuple.0,
                        _uuid: tuple.1,
                        _stateMatrix: tuple.2,
                        _preState: tuple.3,
                        _preStateProof: tuple.4,
                        _postState: tuple.5,
                        _postStateProof: tuple.6,
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
            impl ::core::convert::From<challengeLPPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: challengeLPPReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengeLPPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl challengeLPPReturn {
            fn _tokenize(
                &self,
            ) -> <challengeLPPCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeLPPCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                LibKeccak::StateMatrix,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = challengeLPPReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengeLPP(address,uint256,(uint64[25]),(bytes,uint256,bytes32),bytes32[],(bytes,uint256,bytes32),bytes32[])";
            const SELECTOR: [u8; 4] = [57u8, 9u8, 175u8, 92u8];
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
                        &self._claimant,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._uuid),
                    <LibKeccak::StateMatrix as alloy_sol_types::SolType>::tokenize(
                        &self._stateMatrix,
                    ),
                    <Leaf as alloy_sol_types::SolType>::tokenize(&self._preState),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._preStateProof),
                    <Leaf as alloy_sol_types::SolType>::tokenize(&self._postState),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._postStateProof),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                challengeLPPReturn::_tokenize(ret)
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
    /**Function with signature `challengePeriod()` and selector `0xf3f480d9`.
```solidity
function challengePeriod() external view returns (uint256 challengePeriod_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengePeriodCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`challengePeriod()`](challengePeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengePeriodReturn {
        #[allow(missing_docs)]
        pub challengePeriod_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<challengePeriodCall> for UnderlyingRustTuple<'_> {
                fn from(value: challengePeriodCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengePeriodCall {
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
            impl ::core::convert::From<challengePeriodReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengePeriodReturn) -> Self {
                    (value.challengePeriod_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengePeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { challengePeriod_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengePeriodCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengePeriod()";
            const SELECTOR: [u8; 4] = [243u8, 244u8, 128u8, 217u8];
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
                        let r: challengePeriodReturn = r.into();
                        r.challengePeriod_
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
                        let r: challengePeriodReturn = r.into();
                        r.challengePeriod_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTreeRootLPP(address,uint256)` and selector `0x0359a563`.
```solidity
function getTreeRootLPP(address _owner, uint256 _uuid) external view returns (bytes32 treeRoot_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTreeRootLPPCall {
        #[allow(missing_docs)]
        pub _owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _uuid: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTreeRootLPP(address,uint256)`](getTreeRootLPPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTreeRootLPPReturn {
        #[allow(missing_docs)]
        pub treeRoot_: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getTreeRootLPPCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTreeRootLPPCall) -> Self {
                    (value._owner, value._uuid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTreeRootLPPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _owner: tuple.0,
                        _uuid: tuple.1,
                    }
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
            impl ::core::convert::From<getTreeRootLPPReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTreeRootLPPReturn) -> Self {
                    (value.treeRoot_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTreeRootLPPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { treeRoot_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTreeRootLPPCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTreeRootLPP(address,uint256)";
            const SELECTOR: [u8; 4] = [3u8, 89u8, 165u8, 99u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._uuid),
                )
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
                        let r: getTreeRootLPPReturn = r.into();
                        r.treeRoot_
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
                        let r: getTreeRootLPPReturn = r.into();
                        r.treeRoot_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initLPP(uint256,uint32,uint32)` and selector `0xfaf37bc7`.
```solidity
function initLPP(uint256 _uuid, uint32 _partOffset, uint32 _claimedSize) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initLPPCall {
        #[allow(missing_docs)]
        pub _uuid: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _partOffset: u32,
        #[allow(missing_docs)]
        pub _claimedSize: u32,
    }
    ///Container type for the return parameters of the [`initLPP(uint256,uint32,uint32)`](initLPPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initLPPReturn {}
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u32,
                u32,
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
            impl ::core::convert::From<initLPPCall> for UnderlyingRustTuple<'_> {
                fn from(value: initLPPCall) -> Self {
                    (value._uuid, value._partOffset, value._claimedSize)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initLPPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _uuid: tuple.0,
                        _partOffset: tuple.1,
                        _claimedSize: tuple.2,
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
            impl ::core::convert::From<initLPPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initLPPReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initLPPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initLPPReturn {
            fn _tokenize(
                &self,
            ) -> <initLPPCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initLPPCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initLPPReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initLPP(uint256,uint32,uint32)";
            const SELECTOR: [u8; 4] = [250u8, 243u8, 123u8, 199u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._uuid),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._claimedSize),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initLPPReturn::_tokenize(ret)
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
    /**Function with signature `loadBlobPreimagePart(uint256,uint256,bytes,bytes,uint256)` and selector `0x9d7e8769`.
```solidity
function loadBlobPreimagePart(uint256 _z, uint256 _y, bytes memory _commitment, bytes memory _proof, uint256 _partOffset) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadBlobPreimagePartCall {
        #[allow(missing_docs)]
        pub _z: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _y: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _commitment: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _proof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _partOffset: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`loadBlobPreimagePart(uint256,uint256,bytes,bytes,uint256)`](loadBlobPreimagePartCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadBlobPreimagePartReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<loadBlobPreimagePartCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadBlobPreimagePartCall) -> Self {
                    (
                        value._z,
                        value._y,
                        value._commitment,
                        value._proof,
                        value._partOffset,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadBlobPreimagePartCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _z: tuple.0,
                        _y: tuple.1,
                        _commitment: tuple.2,
                        _proof: tuple.3,
                        _partOffset: tuple.4,
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
            impl ::core::convert::From<loadBlobPreimagePartReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadBlobPreimagePartReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadBlobPreimagePartReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl loadBlobPreimagePartReturn {
            fn _tokenize(
                &self,
            ) -> <loadBlobPreimagePartCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for loadBlobPreimagePartCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = loadBlobPreimagePartReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "loadBlobPreimagePart(uint256,uint256,bytes,bytes,uint256)";
            const SELECTOR: [u8; 4] = [157u8, 126u8, 135u8, 105u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._z),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._y),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._commitment,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._proof,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                loadBlobPreimagePartReturn::_tokenize(ret)
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
    /**Function with signature `loadKeccak256PreimagePart(uint256,bytes)` and selector `0xe1592611`.
```solidity
function loadKeccak256PreimagePart(uint256 _partOffset, bytes memory _preimage) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadKeccak256PreimagePartCall {
        #[allow(missing_docs)]
        pub _partOffset: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _preimage: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`loadKeccak256PreimagePart(uint256,bytes)`](loadKeccak256PreimagePartCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadKeccak256PreimagePartReturn {}
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<loadKeccak256PreimagePartCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadKeccak256PreimagePartCall) -> Self {
                    (value._partOffset, value._preimage)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadKeccak256PreimagePartCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _partOffset: tuple.0,
                        _preimage: tuple.1,
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
            impl ::core::convert::From<loadKeccak256PreimagePartReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadKeccak256PreimagePartReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadKeccak256PreimagePartReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl loadKeccak256PreimagePartReturn {
            fn _tokenize(
                &self,
            ) -> <loadKeccak256PreimagePartCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for loadKeccak256PreimagePartCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = loadKeccak256PreimagePartReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "loadKeccak256PreimagePart(uint256,bytes)";
            const SELECTOR: [u8; 4] = [225u8, 89u8, 38u8, 17u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._preimage,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                loadKeccak256PreimagePartReturn::_tokenize(ret)
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
    /**Function with signature `loadLocalData(uint256,bytes32,bytes32,uint256,uint256)` and selector `0x52f0f3ad`.
```solidity
function loadLocalData(uint256 _ident, bytes32 _localContext, bytes32 _word, uint256 _size, uint256 _partOffset) external returns (bytes32 key_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadLocalDataCall {
        #[allow(missing_docs)]
        pub _ident: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _localContext: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _word: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _size: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _partOffset: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`loadLocalData(uint256,bytes32,bytes32,uint256,uint256)`](loadLocalDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadLocalDataReturn {
        #[allow(missing_docs)]
        pub key_: alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<loadLocalDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: loadLocalDataCall) -> Self {
                    (
                        value._ident,
                        value._localContext,
                        value._word,
                        value._size,
                        value._partOffset,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for loadLocalDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ident: tuple.0,
                        _localContext: tuple.1,
                        _word: tuple.2,
                        _size: tuple.3,
                        _partOffset: tuple.4,
                    }
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
            impl ::core::convert::From<loadLocalDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: loadLocalDataReturn) -> Self {
                    (value.key_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for loadLocalDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { key_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for loadLocalDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "loadLocalData(uint256,bytes32,bytes32,uint256,uint256)";
            const SELECTOR: [u8; 4] = [82u8, 240u8, 243u8, 173u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._ident),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._localContext),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._word),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._size),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                )
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
                        let r: loadLocalDataReturn = r.into();
                        r.key_
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
                        let r: loadLocalDataReturn = r.into();
                        r.key_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `loadPrecompilePreimagePart(uint256,address,uint64,bytes)` and selector `0xea713950`.
```solidity
function loadPrecompilePreimagePart(uint256 _partOffset, address _precompile, uint64 _requiredGas, bytes memory _input) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadPrecompilePreimagePartCall {
        #[allow(missing_docs)]
        pub _partOffset: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _precompile: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _requiredGas: u64,
        #[allow(missing_docs)]
        pub _input: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`loadPrecompilePreimagePart(uint256,address,uint64,bytes)`](loadPrecompilePreimagePartCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadPrecompilePreimagePartReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                u64,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<loadPrecompilePreimagePartCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadPrecompilePreimagePartCall) -> Self {
                    (
                        value._partOffset,
                        value._precompile,
                        value._requiredGas,
                        value._input,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadPrecompilePreimagePartCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _partOffset: tuple.0,
                        _precompile: tuple.1,
                        _requiredGas: tuple.2,
                        _input: tuple.3,
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
            impl ::core::convert::From<loadPrecompilePreimagePartReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadPrecompilePreimagePartReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadPrecompilePreimagePartReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl loadPrecompilePreimagePartReturn {
            fn _tokenize(
                &self,
            ) -> <loadPrecompilePreimagePartCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for loadPrecompilePreimagePartCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = loadPrecompilePreimagePartReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "loadPrecompilePreimagePart(uint256,address,uint64,bytes)";
            const SELECTOR: [u8; 4] = [234u8, 113u8, 57u8, 80u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._precompile,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._requiredGas),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._input,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                loadPrecompilePreimagePartReturn::_tokenize(ret)
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
    /**Function with signature `loadSha256PreimagePart(uint256,bytes)` and selector `0x8dc4be11`.
```solidity
function loadSha256PreimagePart(uint256 _partOffset, bytes memory _preimage) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadSha256PreimagePartCall {
        #[allow(missing_docs)]
        pub _partOffset: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _preimage: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`loadSha256PreimagePart(uint256,bytes)`](loadSha256PreimagePartCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loadSha256PreimagePartReturn {}
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<loadSha256PreimagePartCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadSha256PreimagePartCall) -> Self {
                    (value._partOffset, value._preimage)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadSha256PreimagePartCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _partOffset: tuple.0,
                        _preimage: tuple.1,
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
            impl ::core::convert::From<loadSha256PreimagePartReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: loadSha256PreimagePartReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loadSha256PreimagePartReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl loadSha256PreimagePartReturn {
            fn _tokenize(
                &self,
            ) -> <loadSha256PreimagePartCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for loadSha256PreimagePartCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = loadSha256PreimagePartReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "loadSha256PreimagePart(uint256,bytes)";
            const SELECTOR: [u8; 4] = [141u8, 196u8, 190u8, 17u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._preimage,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                loadSha256PreimagePartReturn::_tokenize(ret)
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
    /**Function with signature `minProposalSize()` and selector `0xdd24f9bf`.
```solidity
function minProposalSize() external view returns (uint256 minProposalSize_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minProposalSizeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`minProposalSize()`](minProposalSizeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minProposalSizeReturn {
        #[allow(missing_docs)]
        pub minProposalSize_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<minProposalSizeCall> for UnderlyingRustTuple<'_> {
                fn from(value: minProposalSizeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minProposalSizeCall {
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
            impl ::core::convert::From<minProposalSizeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: minProposalSizeReturn) -> Self {
                    (value.minProposalSize_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minProposalSizeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { minProposalSize_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minProposalSizeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minProposalSize()";
            const SELECTOR: [u8; 4] = [221u8, 36u8, 249u8, 191u8];
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
                        let r: minProposalSizeReturn = r.into();
                        r.minProposalSize_
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
                        let r: minProposalSizeReturn = r.into();
                        r.minProposalSize_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `preimageLengths(bytes32)` and selector `0xfef2b4ed`.
```solidity
function preimageLengths(bytes32) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preimageLengthsCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`preimageLengths(bytes32)`](preimageLengthsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preimageLengthsReturn {
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
            impl ::core::convert::From<preimageLengthsCall> for UnderlyingRustTuple<'_> {
                fn from(value: preimageLengthsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for preimageLengthsCall {
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
            impl ::core::convert::From<preimageLengthsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: preimageLengthsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for preimageLengthsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for preimageLengthsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "preimageLengths(bytes32)";
            const SELECTOR: [u8; 4] = [254u8, 242u8, 180u8, 237u8];
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
                        let r: preimageLengthsReturn = r.into();
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
                        let r: preimageLengthsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `preimagePartOk(bytes32,uint256)` and selector `0x8542cf50`.
```solidity
function preimagePartOk(bytes32, uint256) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preimagePartOkCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`preimagePartOk(bytes32,uint256)`](preimagePartOkCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preimagePartOkReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<preimagePartOkCall> for UnderlyingRustTuple<'_> {
                fn from(value: preimagePartOkCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for preimagePartOkCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<preimagePartOkReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: preimagePartOkReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for preimagePartOkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for preimagePartOkCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "preimagePartOk(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [133u8, 66u8, 207u8, 80u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
                        let r: preimagePartOkReturn = r.into();
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
                        let r: preimagePartOkReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `preimageParts(bytes32,uint256)` and selector `0x61238bde`.
```solidity
function preimageParts(bytes32, uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preimagePartsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`preimageParts(bytes32,uint256)`](preimagePartsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preimagePartsReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<preimagePartsCall> for UnderlyingRustTuple<'_> {
                fn from(value: preimagePartsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for preimagePartsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<preimagePartsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: preimagePartsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for preimagePartsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for preimagePartsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "preimageParts(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [97u8, 35u8, 139u8, 222u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
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
                        let r: preimagePartsReturn = r.into();
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
                        let r: preimagePartsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposalBlocks(address,uint256,uint256)` and selector `0x882856ef`.
```solidity
function proposalBlocks(address, uint256, uint256) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBlocksCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposalBlocks(address,uint256,uint256)`](proposalBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBlocksReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<proposalBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposalBlocksCall) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
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
            impl ::core::convert::From<proposalBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalBlocksCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalBlocks(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [136u8, 40u8, 86u8, 239u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                )
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
                        let r: proposalBlocksReturn = r.into();
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
                        let r: proposalBlocksReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposalBlocksLen(address,uint256)` and selector `0x9d53a648`.
```solidity
function proposalBlocksLen(address _claimant, uint256 _uuid) external view returns (uint256 len_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBlocksLenCall {
        #[allow(missing_docs)]
        pub _claimant: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _uuid: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposalBlocksLen(address,uint256)`](proposalBlocksLenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBlocksLenReturn {
        #[allow(missing_docs)]
        pub len_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<proposalBlocksLenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalBlocksLenCall) -> Self {
                    (value._claimant, value._uuid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalBlocksLenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimant: tuple.0,
                        _uuid: tuple.1,
                    }
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
            impl ::core::convert::From<proposalBlocksLenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalBlocksLenReturn) -> Self {
                    (value.len_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalBlocksLenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { len_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalBlocksLenCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalBlocksLen(address,uint256)";
            const SELECTOR: [u8; 4] = [157u8, 83u8, 166u8, 72u8];
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
                        &self._claimant,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._uuid),
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
                        let r: proposalBlocksLenReturn = r.into();
                        r.len_
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
                        let r: proposalBlocksLenReturn = r.into();
                        r.len_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposalBonds(address,uint256)` and selector `0xddcd58de`.
```solidity
function proposalBonds(address, uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBondsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposalBonds(address,uint256)`](proposalBondsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBondsReturn {
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
            impl ::core::convert::From<proposalBondsCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposalBondsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalBondsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<proposalBondsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposalBondsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalBondsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalBondsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalBonds(address,uint256)";
            const SELECTOR: [u8; 4] = [221u8, 205u8, 88u8, 222u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
                        let r: proposalBondsReturn = r.into();
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
                        let r: proposalBondsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposalBranches(address,uint256,uint256)` and selector `0xb4801e61`.
```solidity
function proposalBranches(address, uint256, uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBranchesCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposalBranches(address,uint256,uint256)`](proposalBranchesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalBranchesReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<proposalBranchesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalBranchesCall) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalBranchesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
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
            impl ::core::convert::From<proposalBranchesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalBranchesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalBranchesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalBranchesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalBranches(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [180u8, 128u8, 30u8, 97u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                )
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
                        let r: proposalBranchesReturn = r.into();
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
                        let r: proposalBranchesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposalCount()` and selector `0xda35c664`.
```solidity
function proposalCount() external view returns (uint256 count_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalCountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposalCount()`](proposalCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalCountReturn {
        #[allow(missing_docs)]
        pub count_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<proposalCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposalCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalCountCall {
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
            impl ::core::convert::From<proposalCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposalCountReturn) -> Self {
                    (value.count_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalCount()";
            const SELECTOR: [u8; 4] = [218u8, 53u8, 198u8, 100u8];
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
                        let r: proposalCountReturn = r.into();
                        r.count_
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
                        let r: proposalCountReturn = r.into();
                        r.count_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposalMetadata(address,uint256)` and selector `0x6551927b`.
```solidity
function proposalMetadata(address, uint256) external view returns (LPPMetaData);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalMetadataCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposalMetadata(address,uint256)`](proposalMetadataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalMetadataReturn {
        #[allow(missing_docs)]
        pub _0: <LPPMetaData as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<proposalMetadataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalMetadataCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalMetadataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (LPPMetaData,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LPPMetaData as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<proposalMetadataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalMetadataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalMetadataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalMetadataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <LPPMetaData as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (LPPMetaData,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalMetadata(address,uint256)";
            const SELECTOR: [u8; 4] = [101u8, 81u8, 146u8, 123u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<LPPMetaData as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: proposalMetadataReturn = r.into();
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
                        let r: proposalMetadataReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposalParts(address,uint256)` and selector `0xb2e67ba8`.
```solidity
function proposalParts(address, uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalPartsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposalParts(address,uint256)`](proposalPartsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalPartsReturn {
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
            impl ::core::convert::From<proposalPartsCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposalPartsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalPartsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<proposalPartsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposalPartsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalPartsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalPartsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalParts(address,uint256)";
            const SELECTOR: [u8; 4] = [178u8, 230u8, 123u8, 168u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
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
                        let r: proposalPartsReturn = r.into();
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
                        let r: proposalPartsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proposals(uint256)` and selector `0x013cf08b`.
```solidity
function proposals(uint256) external view returns (address claimant, uint256 uuid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalsCall(pub alloy::sol_types::private::primitives::aliases::U256);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposals(uint256)`](proposalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalsReturn {
        #[allow(missing_docs)]
        pub claimant: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub uuid: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<proposalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposalsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
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
            impl ::core::convert::From<proposalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposalsReturn) -> Self {
                    (value.claimant, value.uuid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        claimant: tuple.0,
                        uuid: tuple.1,
                    }
                }
            }
        }
        impl proposalsReturn {
            fn _tokenize(
                &self,
            ) -> <proposalsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.claimant,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.uuid),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposals(uint256)";
            const SELECTOR: [u8; 4] = [1u8, 60u8, 240u8, 139u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                proposalsReturn::_tokenize(ret)
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
    /**Function with signature `readPreimage(bytes32,uint256)` and selector `0xe03110e1`.
```solidity
function readPreimage(bytes32 _key, uint256 _offset) external view returns (bytes32 dat_, uint256 datLen_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct readPreimageCall {
        #[allow(missing_docs)]
        pub _key: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _offset: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`readPreimage(bytes32,uint256)`](readPreimageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct readPreimageReturn {
        #[allow(missing_docs)]
        pub dat_: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub datLen_: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<readPreimageCall> for UnderlyingRustTuple<'_> {
                fn from(value: readPreimageCall) -> Self {
                    (value._key, value._offset)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for readPreimageCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _key: tuple.0,
                        _offset: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<readPreimageReturn> for UnderlyingRustTuple<'_> {
                fn from(value: readPreimageReturn) -> Self {
                    (value.dat_, value.datLen_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for readPreimageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dat_: tuple.0,
                        datLen_: tuple.1,
                    }
                }
            }
        }
        impl readPreimageReturn {
            fn _tokenize(
                &self,
            ) -> <readPreimageCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.dat_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.datLen_),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for readPreimageCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = readPreimageReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "readPreimage(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [224u8, 49u8, 16u8, 225u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._key),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._offset),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                readPreimageReturn::_tokenize(ret)
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
    /**Function with signature `squeezeLPP(address,uint256,(uint64[25]),(bytes,uint256,bytes32),bytes32[],(bytes,uint256,bytes32),bytes32[])` and selector `0xd18534b5`.
```solidity
function squeezeLPP(address _claimant, uint256 _uuid, LibKeccak.StateMatrix memory _stateMatrix, Leaf memory _preState, bytes32[] memory _preStateProof, Leaf memory _postState, bytes32[] memory _postStateProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct squeezeLPPCall {
        #[allow(missing_docs)]
        pub _claimant: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _uuid: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _stateMatrix: <LibKeccak::StateMatrix as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _preState: <Leaf as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _preStateProof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub _postState: <Leaf as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _postStateProof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`squeezeLPP(address,uint256,(uint64[25]),(bytes,uint256,bytes32),bytes32[],(bytes,uint256,bytes32),bytes32[])`](squeezeLPPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct squeezeLPPReturn {}
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
                LibKeccak::StateMatrix,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <LibKeccak::StateMatrix as alloy::sol_types::SolType>::RustType,
                <Leaf as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                <Leaf as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<squeezeLPPCall> for UnderlyingRustTuple<'_> {
                fn from(value: squeezeLPPCall) -> Self {
                    (
                        value._claimant,
                        value._uuid,
                        value._stateMatrix,
                        value._preState,
                        value._preStateProof,
                        value._postState,
                        value._postStateProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for squeezeLPPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimant: tuple.0,
                        _uuid: tuple.1,
                        _stateMatrix: tuple.2,
                        _preState: tuple.3,
                        _preStateProof: tuple.4,
                        _postState: tuple.5,
                        _postStateProof: tuple.6,
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
            impl ::core::convert::From<squeezeLPPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: squeezeLPPReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for squeezeLPPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl squeezeLPPReturn {
            fn _tokenize(
                &self,
            ) -> <squeezeLPPCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for squeezeLPPCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                LibKeccak::StateMatrix,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                Leaf,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = squeezeLPPReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "squeezeLPP(address,uint256,(uint64[25]),(bytes,uint256,bytes32),bytes32[],(bytes,uint256,bytes32),bytes32[])";
            const SELECTOR: [u8; 4] = [209u8, 133u8, 52u8, 181u8];
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
                        &self._claimant,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._uuid),
                    <LibKeccak::StateMatrix as alloy_sol_types::SolType>::tokenize(
                        &self._stateMatrix,
                    ),
                    <Leaf as alloy_sol_types::SolType>::tokenize(&self._preState),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._preStateProof),
                    <Leaf as alloy_sol_types::SolType>::tokenize(&self._postState),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._postStateProof),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                squeezeLPPReturn::_tokenize(ret)
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `zeroHashes(uint256)` and selector `0x7ac54767`.
```solidity
function zeroHashes(uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct zeroHashesCall(pub alloy::sol_types::private::primitives::aliases::U256);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`zeroHashes(uint256)`](zeroHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct zeroHashesReturn {
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
            impl ::core::convert::From<zeroHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: zeroHashesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for zeroHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<zeroHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: zeroHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for zeroHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for zeroHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "zeroHashes(uint256)";
            const SELECTOR: [u8; 4] = [122u8, 197u8, 71u8, 103u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
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
                        let r: zeroHashesReturn = r.into();
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
                        let r: zeroHashesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`PreimageOracle`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum PreimageOracleCalls {
        #[allow(missing_docs)]
        KECCAK_TREE_DEPTH(KECCAK_TREE_DEPTHCall),
        #[allow(missing_docs)]
        MAX_LEAF_COUNT(MAX_LEAF_COUNTCall),
        #[allow(missing_docs)]
        MIN_BOND_SIZE(MIN_BOND_SIZECall),
        #[allow(missing_docs)]
        PRECOMPILE_CALL_RESERVED_GAS(PRECOMPILE_CALL_RESERVED_GASCall),
        #[allow(missing_docs)]
        addLeavesLPP(addLeavesLPPCall),
        #[allow(missing_docs)]
        challengeFirstLPP(challengeFirstLPPCall),
        #[allow(missing_docs)]
        challengeLPP(challengeLPPCall),
        #[allow(missing_docs)]
        challengePeriod(challengePeriodCall),
        #[allow(missing_docs)]
        getTreeRootLPP(getTreeRootLPPCall),
        #[allow(missing_docs)]
        initLPP(initLPPCall),
        #[allow(missing_docs)]
        loadBlobPreimagePart(loadBlobPreimagePartCall),
        #[allow(missing_docs)]
        loadKeccak256PreimagePart(loadKeccak256PreimagePartCall),
        #[allow(missing_docs)]
        loadLocalData(loadLocalDataCall),
        #[allow(missing_docs)]
        loadPrecompilePreimagePart(loadPrecompilePreimagePartCall),
        #[allow(missing_docs)]
        loadSha256PreimagePart(loadSha256PreimagePartCall),
        #[allow(missing_docs)]
        minProposalSize(minProposalSizeCall),
        #[allow(missing_docs)]
        preimageLengths(preimageLengthsCall),
        #[allow(missing_docs)]
        preimagePartOk(preimagePartOkCall),
        #[allow(missing_docs)]
        preimageParts(preimagePartsCall),
        #[allow(missing_docs)]
        proposalBlocks(proposalBlocksCall),
        #[allow(missing_docs)]
        proposalBlocksLen(proposalBlocksLenCall),
        #[allow(missing_docs)]
        proposalBonds(proposalBondsCall),
        #[allow(missing_docs)]
        proposalBranches(proposalBranchesCall),
        #[allow(missing_docs)]
        proposalCount(proposalCountCall),
        #[allow(missing_docs)]
        proposalMetadata(proposalMetadataCall),
        #[allow(missing_docs)]
        proposalParts(proposalPartsCall),
        #[allow(missing_docs)]
        proposals(proposalsCall),
        #[allow(missing_docs)]
        readPreimage(readPreimageCall),
        #[allow(missing_docs)]
        squeezeLPP(squeezeLPPCall),
        #[allow(missing_docs)]
        version(versionCall),
        #[allow(missing_docs)]
        zeroHashes(zeroHashesCall),
    }
    impl PreimageOracleCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 60u8, 240u8, 139u8],
            [3u8, 89u8, 165u8, 99u8],
            [32u8, 85u8, 179u8, 107u8],
            [57u8, 9u8, 175u8, 92u8],
            [77u8, 82u8, 180u8, 201u8],
            [82u8, 240u8, 243u8, 173u8],
            [84u8, 253u8, 77u8, 80u8],
            [97u8, 35u8, 139u8, 222u8],
            [101u8, 81u8, 146u8, 123u8],
            [112u8, 81u8, 71u8, 46u8],
            [121u8, 23u8, 222u8, 29u8],
            [122u8, 197u8, 71u8, 103u8],
            [133u8, 66u8, 207u8, 80u8],
            [136u8, 40u8, 86u8, 239u8],
            [141u8, 196u8, 190u8, 17u8],
            [157u8, 83u8, 166u8, 72u8],
            [157u8, 126u8, 135u8, 105u8],
            [178u8, 230u8, 123u8, 168u8],
            [180u8, 128u8, 30u8, 97u8],
            [181u8, 231u8, 21u8, 76u8],
            [209u8, 133u8, 52u8, 181u8],
            [218u8, 53u8, 198u8, 100u8],
            [221u8, 36u8, 249u8, 191u8],
            [221u8, 205u8, 88u8, 222u8],
            [224u8, 49u8, 16u8, 225u8],
            [225u8, 89u8, 38u8, 17u8],
            [234u8, 113u8, 57u8, 80u8],
            [236u8, 94u8, 252u8, 188u8],
            [243u8, 244u8, 128u8, 217u8],
            [250u8, 243u8, 123u8, 199u8],
            [254u8, 242u8, 180u8, 237u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(proposals),
            ::core::stringify!(getTreeRootLPP),
            ::core::stringify!(KECCAK_TREE_DEPTH),
            ::core::stringify!(challengeLPP),
            ::core::stringify!(MAX_LEAF_COUNT),
            ::core::stringify!(loadLocalData),
            ::core::stringify!(version),
            ::core::stringify!(preimageParts),
            ::core::stringify!(proposalMetadata),
            ::core::stringify!(MIN_BOND_SIZE),
            ::core::stringify!(addLeavesLPP),
            ::core::stringify!(zeroHashes),
            ::core::stringify!(preimagePartOk),
            ::core::stringify!(proposalBlocks),
            ::core::stringify!(loadSha256PreimagePart),
            ::core::stringify!(proposalBlocksLen),
            ::core::stringify!(loadBlobPreimagePart),
            ::core::stringify!(proposalParts),
            ::core::stringify!(proposalBranches),
            ::core::stringify!(PRECOMPILE_CALL_RESERVED_GAS),
            ::core::stringify!(squeezeLPP),
            ::core::stringify!(proposalCount),
            ::core::stringify!(minProposalSize),
            ::core::stringify!(proposalBonds),
            ::core::stringify!(readPreimage),
            ::core::stringify!(loadKeccak256PreimagePart),
            ::core::stringify!(loadPrecompilePreimagePart),
            ::core::stringify!(challengeFirstLPP),
            ::core::stringify!(challengePeriod),
            ::core::stringify!(initLPP),
            ::core::stringify!(preimageLengths),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <proposalsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getTreeRootLPPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <KECCAK_TREE_DEPTHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <challengeLPPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_LEAF_COUNTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <loadLocalDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <preimagePartsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposalMetadataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MIN_BOND_SIZECall as alloy_sol_types::SolCall>::SIGNATURE,
            <addLeavesLPPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <zeroHashesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <preimagePartOkCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposalBlocksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <loadSha256PreimagePartCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposalBlocksLenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <loadBlobPreimagePartCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposalPartsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposalBranchesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PRECOMPILE_CALL_RESERVED_GASCall as alloy_sol_types::SolCall>::SIGNATURE,
            <squeezeLPPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposalCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <minProposalSizeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposalBondsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <readPreimageCall as alloy_sol_types::SolCall>::SIGNATURE,
            <loadKeccak256PreimagePartCall as alloy_sol_types::SolCall>::SIGNATURE,
            <loadPrecompilePreimagePartCall as alloy_sol_types::SolCall>::SIGNATURE,
            <challengeFirstLPPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <challengePeriodCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initLPPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <preimageLengthsCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for PreimageOracleCalls {
        const NAME: &'static str = "PreimageOracleCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 31usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::KECCAK_TREE_DEPTH(_) => {
                    <KECCAK_TREE_DEPTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_LEAF_COUNT(_) => {
                    <MAX_LEAF_COUNTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MIN_BOND_SIZE(_) => {
                    <MIN_BOND_SIZECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PRECOMPILE_CALL_RESERVED_GAS(_) => {
                    <PRECOMPILE_CALL_RESERVED_GASCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addLeavesLPP(_) => {
                    <addLeavesLPPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challengeFirstLPP(_) => {
                    <challengeFirstLPPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challengeLPP(_) => {
                    <challengeLPPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challengePeriod(_) => {
                    <challengePeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTreeRootLPP(_) => {
                    <getTreeRootLPPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initLPP(_) => <initLPPCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::loadBlobPreimagePart(_) => {
                    <loadBlobPreimagePartCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::loadKeccak256PreimagePart(_) => {
                    <loadKeccak256PreimagePartCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::loadLocalData(_) => {
                    <loadLocalDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::loadPrecompilePreimagePart(_) => {
                    <loadPrecompilePreimagePartCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::loadSha256PreimagePart(_) => {
                    <loadSha256PreimagePartCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minProposalSize(_) => {
                    <minProposalSizeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::preimageLengths(_) => {
                    <preimageLengthsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::preimagePartOk(_) => {
                    <preimagePartOkCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::preimageParts(_) => {
                    <preimagePartsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalBlocks(_) => {
                    <proposalBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalBlocksLen(_) => {
                    <proposalBlocksLenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalBonds(_) => {
                    <proposalBondsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalBranches(_) => {
                    <proposalBranchesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalCount(_) => {
                    <proposalCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalMetadata(_) => {
                    <proposalMetadataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalParts(_) => {
                    <proposalPartsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposals(_) => {
                    <proposalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::readPreimage(_) => {
                    <readPreimageCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::squeezeLPP(_) => {
                    <squeezeLPPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::zeroHashes(_) => {
                    <zeroHashesCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<PreimageOracleCalls>] = &[
                {
                    fn proposals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PreimageOracleCalls::proposals)
                    }
                    proposals
                },
                {
                    fn getTreeRootLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <getTreeRootLPPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::getTreeRootLPP)
                    }
                    getTreeRootLPP
                },
                {
                    fn KECCAK_TREE_DEPTH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <KECCAK_TREE_DEPTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::KECCAK_TREE_DEPTH)
                    }
                    KECCAK_TREE_DEPTH
                },
                {
                    fn challengeLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <challengeLPPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::challengeLPP)
                    }
                    challengeLPP
                },
                {
                    fn MAX_LEAF_COUNT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <MAX_LEAF_COUNTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::MAX_LEAF_COUNT)
                    }
                    MAX_LEAF_COUNT
                },
                {
                    fn loadLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::loadLocalData)
                    }
                    loadLocalData
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PreimageOracleCalls::version)
                    }
                    version
                },
                {
                    fn preimageParts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <preimagePartsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::preimageParts)
                    }
                    preimageParts
                },
                {
                    fn proposalMetadata(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalMetadataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalMetadata)
                    }
                    proposalMetadata
                },
                {
                    fn MIN_BOND_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <MIN_BOND_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::MIN_BOND_SIZE)
                    }
                    MIN_BOND_SIZE
                },
                {
                    fn addLeavesLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <addLeavesLPPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::addLeavesLPP)
                    }
                    addLeavesLPP
                },
                {
                    fn zeroHashes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <zeroHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::zeroHashes)
                    }
                    zeroHashes
                },
                {
                    fn preimagePartOk(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <preimagePartOkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::preimagePartOk)
                    }
                    preimagePartOk
                },
                {
                    fn proposalBlocks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBlocks)
                    }
                    proposalBlocks
                },
                {
                    fn loadSha256PreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadSha256PreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::loadSha256PreimagePart)
                    }
                    loadSha256PreimagePart
                },
                {
                    fn proposalBlocksLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBlocksLenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBlocksLen)
                    }
                    proposalBlocksLen
                },
                {
                    fn loadBlobPreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadBlobPreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::loadBlobPreimagePart)
                    }
                    loadBlobPreimagePart
                },
                {
                    fn proposalParts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalPartsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalParts)
                    }
                    proposalParts
                },
                {
                    fn proposalBranches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBranchesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBranches)
                    }
                    proposalBranches
                },
                {
                    fn PRECOMPILE_CALL_RESERVED_GAS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <PRECOMPILE_CALL_RESERVED_GASCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::PRECOMPILE_CALL_RESERVED_GAS)
                    }
                    PRECOMPILE_CALL_RESERVED_GAS
                },
                {
                    fn squeezeLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <squeezeLPPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::squeezeLPP)
                    }
                    squeezeLPP
                },
                {
                    fn proposalCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalCount)
                    }
                    proposalCount
                },
                {
                    fn minProposalSize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <minProposalSizeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::minProposalSize)
                    }
                    minProposalSize
                },
                {
                    fn proposalBonds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBonds)
                    }
                    proposalBonds
                },
                {
                    fn readPreimage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <readPreimageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::readPreimage)
                    }
                    readPreimage
                },
                {
                    fn loadKeccak256PreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadKeccak256PreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::loadKeccak256PreimagePart)
                    }
                    loadKeccak256PreimagePart
                },
                {
                    fn loadPrecompilePreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadPrecompilePreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::loadPrecompilePreimagePart)
                    }
                    loadPrecompilePreimagePart
                },
                {
                    fn challengeFirstLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <challengeFirstLPPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::challengeFirstLPP)
                    }
                    challengeFirstLPP
                },
                {
                    fn challengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <challengePeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::challengePeriod)
                    }
                    challengePeriod
                },
                {
                    fn initLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <initLPPCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PreimageOracleCalls::initLPP)
                    }
                    initLPP
                },
                {
                    fn preimageLengths(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <preimageLengthsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleCalls::preimageLengths)
                    }
                    preimageLengths
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
            ) -> alloy_sol_types::Result<PreimageOracleCalls>] = &[
                {
                    fn proposals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposals)
                    }
                    proposals
                },
                {
                    fn getTreeRootLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <getTreeRootLPPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::getTreeRootLPP)
                    }
                    getTreeRootLPP
                },
                {
                    fn KECCAK_TREE_DEPTH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <KECCAK_TREE_DEPTHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::KECCAK_TREE_DEPTH)
                    }
                    KECCAK_TREE_DEPTH
                },
                {
                    fn challengeLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <challengeLPPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::challengeLPP)
                    }
                    challengeLPP
                },
                {
                    fn MAX_LEAF_COUNT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <MAX_LEAF_COUNTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::MAX_LEAF_COUNT)
                    }
                    MAX_LEAF_COUNT
                },
                {
                    fn loadLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::loadLocalData)
                    }
                    loadLocalData
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::version)
                    }
                    version
                },
                {
                    fn preimageParts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <preimagePartsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::preimageParts)
                    }
                    preimageParts
                },
                {
                    fn proposalMetadata(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalMetadataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalMetadata)
                    }
                    proposalMetadata
                },
                {
                    fn MIN_BOND_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <MIN_BOND_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::MIN_BOND_SIZE)
                    }
                    MIN_BOND_SIZE
                },
                {
                    fn addLeavesLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <addLeavesLPPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::addLeavesLPP)
                    }
                    addLeavesLPP
                },
                {
                    fn zeroHashes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <zeroHashesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::zeroHashes)
                    }
                    zeroHashes
                },
                {
                    fn preimagePartOk(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <preimagePartOkCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::preimagePartOk)
                    }
                    preimagePartOk
                },
                {
                    fn proposalBlocks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBlocks)
                    }
                    proposalBlocks
                },
                {
                    fn loadSha256PreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadSha256PreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::loadSha256PreimagePart)
                    }
                    loadSha256PreimagePart
                },
                {
                    fn proposalBlocksLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBlocksLenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBlocksLen)
                    }
                    proposalBlocksLen
                },
                {
                    fn loadBlobPreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadBlobPreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::loadBlobPreimagePart)
                    }
                    loadBlobPreimagePart
                },
                {
                    fn proposalParts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalPartsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalParts)
                    }
                    proposalParts
                },
                {
                    fn proposalBranches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBranchesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBranches)
                    }
                    proposalBranches
                },
                {
                    fn PRECOMPILE_CALL_RESERVED_GAS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <PRECOMPILE_CALL_RESERVED_GASCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::PRECOMPILE_CALL_RESERVED_GAS)
                    }
                    PRECOMPILE_CALL_RESERVED_GAS
                },
                {
                    fn squeezeLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <squeezeLPPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::squeezeLPP)
                    }
                    squeezeLPP
                },
                {
                    fn proposalCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalCount)
                    }
                    proposalCount
                },
                {
                    fn minProposalSize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <minProposalSizeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::minProposalSize)
                    }
                    minProposalSize
                },
                {
                    fn proposalBonds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <proposalBondsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::proposalBonds)
                    }
                    proposalBonds
                },
                {
                    fn readPreimage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <readPreimageCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::readPreimage)
                    }
                    readPreimage
                },
                {
                    fn loadKeccak256PreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadKeccak256PreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::loadKeccak256PreimagePart)
                    }
                    loadKeccak256PreimagePart
                },
                {
                    fn loadPrecompilePreimagePart(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <loadPrecompilePreimagePartCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::loadPrecompilePreimagePart)
                    }
                    loadPrecompilePreimagePart
                },
                {
                    fn challengeFirstLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <challengeFirstLPPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::challengeFirstLPP)
                    }
                    challengeFirstLPP
                },
                {
                    fn challengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <challengePeriodCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::challengePeriod)
                    }
                    challengePeriod
                },
                {
                    fn initLPP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <initLPPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::initLPP)
                    }
                    initLPP
                },
                {
                    fn preimageLengths(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleCalls> {
                        <preimageLengthsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleCalls::preimageLengths)
                    }
                    preimageLengths
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
                Self::KECCAK_TREE_DEPTH(inner) => {
                    <KECCAK_TREE_DEPTHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_LEAF_COUNT(inner) => {
                    <MAX_LEAF_COUNTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MIN_BOND_SIZE(inner) => {
                    <MIN_BOND_SIZECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PRECOMPILE_CALL_RESERVED_GAS(inner) => {
                    <PRECOMPILE_CALL_RESERVED_GASCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addLeavesLPP(inner) => {
                    <addLeavesLPPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challengeFirstLPP(inner) => {
                    <challengeFirstLPPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challengeLPP(inner) => {
                    <challengeLPPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challengePeriod(inner) => {
                    <challengePeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTreeRootLPP(inner) => {
                    <getTreeRootLPPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initLPP(inner) => {
                    <initLPPCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::loadBlobPreimagePart(inner) => {
                    <loadBlobPreimagePartCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::loadKeccak256PreimagePart(inner) => {
                    <loadKeccak256PreimagePartCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::loadLocalData(inner) => {
                    <loadLocalDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::loadPrecompilePreimagePart(inner) => {
                    <loadPrecompilePreimagePartCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::loadSha256PreimagePart(inner) => {
                    <loadSha256PreimagePartCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minProposalSize(inner) => {
                    <minProposalSizeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::preimageLengths(inner) => {
                    <preimageLengthsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::preimagePartOk(inner) => {
                    <preimagePartOkCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::preimageParts(inner) => {
                    <preimagePartsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalBlocks(inner) => {
                    <proposalBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalBlocksLen(inner) => {
                    <proposalBlocksLenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalBonds(inner) => {
                    <proposalBondsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalBranches(inner) => {
                    <proposalBranchesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalCount(inner) => {
                    <proposalCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalMetadata(inner) => {
                    <proposalMetadataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalParts(inner) => {
                    <proposalPartsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposals(inner) => {
                    <proposalsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::readPreimage(inner) => {
                    <readPreimageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::squeezeLPP(inner) => {
                    <squeezeLPPCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::zeroHashes(inner) => {
                    <zeroHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::KECCAK_TREE_DEPTH(inner) => {
                    <KECCAK_TREE_DEPTHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_LEAF_COUNT(inner) => {
                    <MAX_LEAF_COUNTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MIN_BOND_SIZE(inner) => {
                    <MIN_BOND_SIZECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PRECOMPILE_CALL_RESERVED_GAS(inner) => {
                    <PRECOMPILE_CALL_RESERVED_GASCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addLeavesLPP(inner) => {
                    <addLeavesLPPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challengeFirstLPP(inner) => {
                    <challengeFirstLPPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challengeLPP(inner) => {
                    <challengeLPPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challengePeriod(inner) => {
                    <challengePeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTreeRootLPP(inner) => {
                    <getTreeRootLPPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initLPP(inner) => {
                    <initLPPCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::loadBlobPreimagePart(inner) => {
                    <loadBlobPreimagePartCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::loadKeccak256PreimagePart(inner) => {
                    <loadKeccak256PreimagePartCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::loadLocalData(inner) => {
                    <loadLocalDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::loadPrecompilePreimagePart(inner) => {
                    <loadPrecompilePreimagePartCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::loadSha256PreimagePart(inner) => {
                    <loadSha256PreimagePartCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minProposalSize(inner) => {
                    <minProposalSizeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::preimageLengths(inner) => {
                    <preimageLengthsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::preimagePartOk(inner) => {
                    <preimagePartOkCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::preimageParts(inner) => {
                    <preimagePartsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalBlocks(inner) => {
                    <proposalBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalBlocksLen(inner) => {
                    <proposalBlocksLenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalBonds(inner) => {
                    <proposalBondsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalBranches(inner) => {
                    <proposalBranchesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalCount(inner) => {
                    <proposalCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalMetadata(inner) => {
                    <proposalMetadataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalParts(inner) => {
                    <proposalPartsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposals(inner) => {
                    <proposalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::readPreimage(inner) => {
                    <readPreimageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::squeezeLPP(inner) => {
                    <squeezeLPPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::zeroHashes(inner) => {
                    <zeroHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PreimageOracle`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum PreimageOracleErrors {
        #[allow(missing_docs)]
        ActiveProposal(ActiveProposal),
        #[allow(missing_docs)]
        AlreadyFinalized(AlreadyFinalized),
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        BadProposal(BadProposal),
        #[allow(missing_docs)]
        BondTransferFailed(BondTransferFailed),
        #[allow(missing_docs)]
        InsufficientBond(InsufficientBond),
        #[allow(missing_docs)]
        InvalidInputSize(InvalidInputSize),
        #[allow(missing_docs)]
        InvalidPreimage(InvalidPreimage),
        #[allow(missing_docs)]
        InvalidProof(InvalidProof),
        #[allow(missing_docs)]
        NotEOA(NotEOA),
        #[allow(missing_docs)]
        NotInitialized(NotInitialized),
        #[allow(missing_docs)]
        PartOffsetOOB(PartOffsetOOB),
        #[allow(missing_docs)]
        PostStateMatches(PostStateMatches),
        #[allow(missing_docs)]
        StatesNotContiguous(StatesNotContiguous),
        #[allow(missing_docs)]
        TreeSizeOverflow(TreeSizeOverflow),
        #[allow(missing_docs)]
        WrongStartingBlock(WrongStartingBlock),
    }
    impl PreimageOracleErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 189u8, 227u8, 57u8],
            [13u8, 193u8, 73u8, 240u8],
            [25u8, 104u8, 169u8, 2u8],
            [71u8, 90u8, 37u8, 53u8],
            [85u8, 212u8, 203u8, 249u8],
            [96u8, 249u8, 93u8, 90u8],
            [98u8, 41u8, 87u8, 35u8],
            [123u8, 29u8, 175u8, 209u8],
            [131u8, 230u8, 204u8, 107u8],
            [135u8, 19u8, 141u8, 92u8],
            [152u8, 67u8, 20u8, 91u8],
            [154u8, 59u8, 17u8, 153u8],
            [186u8, 9u8, 45u8, 22u8],
            [195u8, 52u8, 240u8, 105u8],
            [233u8, 44u8, 70u8, 159u8],
            [254u8, 37u8, 73u8, 135u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(InvalidProof),
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(InvalidPreimage),
            ::core::stringify!(AlreadyFinalized),
            ::core::stringify!(ActiveProposal),
            ::core::stringify!(WrongStartingBlock),
            ::core::stringify!(TreeSizeOverflow),
            ::core::stringify!(InvalidInputSize),
            ::core::stringify!(BondTransferFailed),
            ::core::stringify!(NotInitialized),
            ::core::stringify!(PostStateMatches),
            ::core::stringify!(StatesNotContiguous),
            ::core::stringify!(NotEOA),
            ::core::stringify!(BadProposal),
            ::core::stringify!(InsufficientBond),
            ::core::stringify!(PartOffsetOOB),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <InvalidProof as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidPreimage as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyFinalized as alloy_sol_types::SolError>::SIGNATURE,
            <ActiveProposal as alloy_sol_types::SolError>::SIGNATURE,
            <WrongStartingBlock as alloy_sol_types::SolError>::SIGNATURE,
            <TreeSizeOverflow as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInputSize as alloy_sol_types::SolError>::SIGNATURE,
            <BondTransferFailed as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <PostStateMatches as alloy_sol_types::SolError>::SIGNATURE,
            <StatesNotContiguous as alloy_sol_types::SolError>::SIGNATURE,
            <NotEOA as alloy_sol_types::SolError>::SIGNATURE,
            <BadProposal as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientBond as alloy_sol_types::SolError>::SIGNATURE,
            <PartOffsetOOB as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for PreimageOracleErrors {
        const NAME: &'static str = "PreimageOracleErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 16usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ActiveProposal(_) => {
                    <ActiveProposal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadyFinalized(_) => {
                    <AlreadyFinalized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BadProposal(_) => {
                    <BadProposal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BondTransferFailed(_) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientBond(_) => {
                    <InsufficientBond as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInputSize(_) => {
                    <InvalidInputSize as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPreimage(_) => {
                    <InvalidPreimage as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProof(_) => {
                    <InvalidProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotEOA(_) => <NotEOA as alloy_sol_types::SolError>::SELECTOR,
                Self::NotInitialized(_) => {
                    <NotInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PartOffsetOOB(_) => {
                    <PartOffsetOOB as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PostStateMatches(_) => {
                    <PostStateMatches as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StatesNotContiguous(_) => {
                    <StatesNotContiguous as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TreeSizeOverflow(_) => {
                    <TreeSizeOverflow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongStartingBlock(_) => {
                    <WrongStartingBlock as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<PreimageOracleErrors>] = &[
                {
                    fn InvalidProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InvalidProof as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PreimageOracleErrors::InvalidProof)
                    }
                    InvalidProof
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn InvalidPreimage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InvalidPreimage as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::InvalidPreimage)
                    }
                    InvalidPreimage
                },
                {
                    fn AlreadyFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <AlreadyFinalized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::AlreadyFinalized)
                    }
                    AlreadyFinalized
                },
                {
                    fn ActiveProposal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <ActiveProposal as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::ActiveProposal)
                    }
                    ActiveProposal
                },
                {
                    fn WrongStartingBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <WrongStartingBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::WrongStartingBlock)
                    }
                    WrongStartingBlock
                },
                {
                    fn TreeSizeOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <TreeSizeOverflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::TreeSizeOverflow)
                    }
                    TreeSizeOverflow
                },
                {
                    fn InvalidInputSize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InvalidInputSize as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::InvalidInputSize)
                    }
                    InvalidInputSize
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn NotInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <NotInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::NotInitialized)
                    }
                    NotInitialized
                },
                {
                    fn PostStateMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <PostStateMatches as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::PostStateMatches)
                    }
                    PostStateMatches
                },
                {
                    fn StatesNotContiguous(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <StatesNotContiguous as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::StatesNotContiguous)
                    }
                    StatesNotContiguous
                },
                {
                    fn NotEOA(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <NotEOA as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PreimageOracleErrors::NotEOA)
                    }
                    NotEOA
                },
                {
                    fn BadProposal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <BadProposal as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PreimageOracleErrors::BadProposal)
                    }
                    BadProposal
                },
                {
                    fn InsufficientBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InsufficientBond as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::InsufficientBond)
                    }
                    InsufficientBond
                },
                {
                    fn PartOffsetOOB(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <PartOffsetOOB as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PreimageOracleErrors::PartOffsetOOB)
                    }
                    PartOffsetOOB
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
            ) -> alloy_sol_types::Result<PreimageOracleErrors>] = &[
                {
                    fn InvalidProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InvalidProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::InvalidProof)
                    }
                    InvalidProof
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn InvalidPreimage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InvalidPreimage as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::InvalidPreimage)
                    }
                    InvalidPreimage
                },
                {
                    fn AlreadyFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <AlreadyFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::AlreadyFinalized)
                    }
                    AlreadyFinalized
                },
                {
                    fn ActiveProposal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <ActiveProposal as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::ActiveProposal)
                    }
                    ActiveProposal
                },
                {
                    fn WrongStartingBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <WrongStartingBlock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::WrongStartingBlock)
                    }
                    WrongStartingBlock
                },
                {
                    fn TreeSizeOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <TreeSizeOverflow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::TreeSizeOverflow)
                    }
                    TreeSizeOverflow
                },
                {
                    fn InvalidInputSize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InvalidInputSize as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::InvalidInputSize)
                    }
                    InvalidInputSize
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn NotInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <NotInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::NotInitialized)
                    }
                    NotInitialized
                },
                {
                    fn PostStateMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <PostStateMatches as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::PostStateMatches)
                    }
                    PostStateMatches
                },
                {
                    fn StatesNotContiguous(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <StatesNotContiguous as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::StatesNotContiguous)
                    }
                    StatesNotContiguous
                },
                {
                    fn NotEOA(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <NotEOA as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::NotEOA)
                    }
                    NotEOA
                },
                {
                    fn BadProposal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <BadProposal as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::BadProposal)
                    }
                    BadProposal
                },
                {
                    fn InsufficientBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <InsufficientBond as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::InsufficientBond)
                    }
                    InsufficientBond
                },
                {
                    fn PartOffsetOOB(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PreimageOracleErrors> {
                        <PartOffsetOOB as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PreimageOracleErrors::PartOffsetOOB)
                    }
                    PartOffsetOOB
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
                Self::ActiveProposal(inner) => {
                    <ActiveProposal as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AlreadyFinalized(inner) => {
                    <AlreadyFinalized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BadProposal(inner) => {
                    <BadProposal as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BondTransferFailed(inner) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientBond(inner) => {
                    <InsufficientBond as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInputSize(inner) => {
                    <InvalidInputSize as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPreimage(inner) => {
                    <InvalidPreimage as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotEOA(inner) => {
                    <NotEOA as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotInitialized(inner) => {
                    <NotInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PartOffsetOOB(inner) => {
                    <PartOffsetOOB as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::PostStateMatches(inner) => {
                    <PostStateMatches as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StatesNotContiguous(inner) => {
                    <StatesNotContiguous as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TreeSizeOverflow(inner) => {
                    <TreeSizeOverflow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongStartingBlock(inner) => {
                    <WrongStartingBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ActiveProposal(inner) => {
                    <ActiveProposal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadyFinalized(inner) => {
                    <AlreadyFinalized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BadProposal(inner) => {
                    <BadProposal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BondTransferFailed(inner) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientBond(inner) => {
                    <InsufficientBond as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInputSize(inner) => {
                    <InvalidInputSize as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPreimage(inner) => {
                    <InvalidPreimage as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotEOA(inner) => {
                    <NotEOA as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotInitialized(inner) => {
                    <NotInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PartOffsetOOB(inner) => {
                    <PartOffsetOOB as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PostStateMatches(inner) => {
                    <PostStateMatches as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StatesNotContiguous(inner) => {
                    <StatesNotContiguous as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TreeSizeOverflow(inner) => {
                    <TreeSizeOverflow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongStartingBlock(inner) => {
                    <WrongStartingBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PreimageOracle`](self) contract instance.

See the [wrapper's documentation](`PreimageOracleInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> PreimageOracleInstance<P, N> {
        PreimageOracleInstance::<P, N>::new(address, __provider)
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
        _minProposalSize: alloy::sol_types::private::primitives::aliases::U256,
        _challengePeriod: alloy::sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PreimageOracleInstance<P, N>>,
    > {
        PreimageOracleInstance::<
            P,
            N,
        >::deploy(__provider, _minProposalSize, _challengePeriod)
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
        _minProposalSize: alloy::sol_types::private::primitives::aliases::U256,
        _challengePeriod: alloy::sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        PreimageOracleInstance::<
            P,
            N,
        >::deploy_builder(__provider, _minProposalSize, _challengePeriod)
    }
    /**A [`PreimageOracle`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PreimageOracle`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PreimageOracleInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for PreimageOracleInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PreimageOracleInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > PreimageOracleInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`PreimageOracle`](self) contract instance.

See the [wrapper's documentation](`PreimageOracleInstance`) for more details.*/
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
            _minProposalSize: alloy::sol_types::private::primitives::aliases::U256,
            _challengePeriod: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::Result<PreimageOracleInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                _minProposalSize,
                _challengePeriod,
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
            _minProposalSize: alloy::sol_types::private::primitives::aliases::U256,
            _challengePeriod: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _minProposalSize,
                            _challengePeriod,
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
    impl<P: ::core::clone::Clone, N> PreimageOracleInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PreimageOracleInstance<P, N> {
            PreimageOracleInstance {
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
    > PreimageOracleInstance<P, N> {
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
        ///Creates a new call builder for the [`KECCAK_TREE_DEPTH`] function.
        pub fn KECCAK_TREE_DEPTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, KECCAK_TREE_DEPTHCall, N> {
            self.call_builder(&KECCAK_TREE_DEPTHCall)
        }
        ///Creates a new call builder for the [`MAX_LEAF_COUNT`] function.
        pub fn MAX_LEAF_COUNT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_LEAF_COUNTCall, N> {
            self.call_builder(&MAX_LEAF_COUNTCall)
        }
        ///Creates a new call builder for the [`MIN_BOND_SIZE`] function.
        pub fn MIN_BOND_SIZE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MIN_BOND_SIZECall, N> {
            self.call_builder(&MIN_BOND_SIZECall)
        }
        ///Creates a new call builder for the [`PRECOMPILE_CALL_RESERVED_GAS`] function.
        pub fn PRECOMPILE_CALL_RESERVED_GAS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, PRECOMPILE_CALL_RESERVED_GASCall, N> {
            self.call_builder(&PRECOMPILE_CALL_RESERVED_GASCall)
        }
        ///Creates a new call builder for the [`addLeavesLPP`] function.
        pub fn addLeavesLPP(
            &self,
            _uuid: alloy::sol_types::private::primitives::aliases::U256,
            _inputStartBlock: alloy::sol_types::private::primitives::aliases::U256,
            _input: alloy::sol_types::private::Bytes,
            _stateCommitments: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            _finalize: bool,
        ) -> alloy_contract::SolCallBuilder<&P, addLeavesLPPCall, N> {
            self.call_builder(
                &addLeavesLPPCall {
                    _uuid,
                    _inputStartBlock,
                    _input,
                    _stateCommitments,
                    _finalize,
                },
            )
        }
        ///Creates a new call builder for the [`challengeFirstLPP`] function.
        pub fn challengeFirstLPP(
            &self,
            _claimant: alloy::sol_types::private::Address,
            _uuid: alloy::sol_types::private::primitives::aliases::U256,
            _postState: <Leaf as alloy::sol_types::SolType>::RustType,
            _postStateProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, challengeFirstLPPCall, N> {
            self.call_builder(
                &challengeFirstLPPCall {
                    _claimant,
                    _uuid,
                    _postState,
                    _postStateProof,
                },
            )
        }
        ///Creates a new call builder for the [`challengeLPP`] function.
        pub fn challengeLPP(
            &self,
            _claimant: alloy::sol_types::private::Address,
            _uuid: alloy::sol_types::private::primitives::aliases::U256,
            _stateMatrix: <LibKeccak::StateMatrix as alloy::sol_types::SolType>::RustType,
            _preState: <Leaf as alloy::sol_types::SolType>::RustType,
            _preStateProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            _postState: <Leaf as alloy::sol_types::SolType>::RustType,
            _postStateProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, challengeLPPCall, N> {
            self.call_builder(
                &challengeLPPCall {
                    _claimant,
                    _uuid,
                    _stateMatrix,
                    _preState,
                    _preStateProof,
                    _postState,
                    _postStateProof,
                },
            )
        }
        ///Creates a new call builder for the [`challengePeriod`] function.
        pub fn challengePeriod(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, challengePeriodCall, N> {
            self.call_builder(&challengePeriodCall)
        }
        ///Creates a new call builder for the [`getTreeRootLPP`] function.
        pub fn getTreeRootLPP(
            &self,
            _owner: alloy::sol_types::private::Address,
            _uuid: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getTreeRootLPPCall, N> {
            self.call_builder(
                &getTreeRootLPPCall {
                    _owner,
                    _uuid,
                },
            )
        }
        ///Creates a new call builder for the [`initLPP`] function.
        pub fn initLPP(
            &self,
            _uuid: alloy::sol_types::private::primitives::aliases::U256,
            _partOffset: u32,
            _claimedSize: u32,
        ) -> alloy_contract::SolCallBuilder<&P, initLPPCall, N> {
            self.call_builder(
                &initLPPCall {
                    _uuid,
                    _partOffset,
                    _claimedSize,
                },
            )
        }
        ///Creates a new call builder for the [`loadBlobPreimagePart`] function.
        pub fn loadBlobPreimagePart(
            &self,
            _z: alloy::sol_types::private::primitives::aliases::U256,
            _y: alloy::sol_types::private::primitives::aliases::U256,
            _commitment: alloy::sol_types::private::Bytes,
            _proof: alloy::sol_types::private::Bytes,
            _partOffset: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, loadBlobPreimagePartCall, N> {
            self.call_builder(
                &loadBlobPreimagePartCall {
                    _z,
                    _y,
                    _commitment,
                    _proof,
                    _partOffset,
                },
            )
        }
        ///Creates a new call builder for the [`loadKeccak256PreimagePart`] function.
        pub fn loadKeccak256PreimagePart(
            &self,
            _partOffset: alloy::sol_types::private::primitives::aliases::U256,
            _preimage: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, loadKeccak256PreimagePartCall, N> {
            self.call_builder(
                &loadKeccak256PreimagePartCall {
                    _partOffset,
                    _preimage,
                },
            )
        }
        ///Creates a new call builder for the [`loadLocalData`] function.
        pub fn loadLocalData(
            &self,
            _ident: alloy::sol_types::private::primitives::aliases::U256,
            _localContext: alloy::sol_types::private::FixedBytes<32>,
            _word: alloy::sol_types::private::FixedBytes<32>,
            _size: alloy::sol_types::private::primitives::aliases::U256,
            _partOffset: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, loadLocalDataCall, N> {
            self.call_builder(
                &loadLocalDataCall {
                    _ident,
                    _localContext,
                    _word,
                    _size,
                    _partOffset,
                },
            )
        }
        ///Creates a new call builder for the [`loadPrecompilePreimagePart`] function.
        pub fn loadPrecompilePreimagePart(
            &self,
            _partOffset: alloy::sol_types::private::primitives::aliases::U256,
            _precompile: alloy::sol_types::private::Address,
            _requiredGas: u64,
            _input: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, loadPrecompilePreimagePartCall, N> {
            self.call_builder(
                &loadPrecompilePreimagePartCall {
                    _partOffset,
                    _precompile,
                    _requiredGas,
                    _input,
                },
            )
        }
        ///Creates a new call builder for the [`loadSha256PreimagePart`] function.
        pub fn loadSha256PreimagePart(
            &self,
            _partOffset: alloy::sol_types::private::primitives::aliases::U256,
            _preimage: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, loadSha256PreimagePartCall, N> {
            self.call_builder(
                &loadSha256PreimagePartCall {
                    _partOffset,
                    _preimage,
                },
            )
        }
        ///Creates a new call builder for the [`minProposalSize`] function.
        pub fn minProposalSize(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, minProposalSizeCall, N> {
            self.call_builder(&minProposalSizeCall)
        }
        ///Creates a new call builder for the [`preimageLengths`] function.
        pub fn preimageLengths(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, preimageLengthsCall, N> {
            self.call_builder(&preimageLengthsCall(_0))
        }
        ///Creates a new call builder for the [`preimagePartOk`] function.
        pub fn preimagePartOk(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, preimagePartOkCall, N> {
            self.call_builder(&preimagePartOkCall { _0, _1 })
        }
        ///Creates a new call builder for the [`preimageParts`] function.
        pub fn preimageParts(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, preimagePartsCall, N> {
            self.call_builder(&preimagePartsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`proposalBlocks`] function.
        pub fn proposalBlocks(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
            _2: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposalBlocksCall, N> {
            self.call_builder(&proposalBlocksCall { _0, _1, _2 })
        }
        ///Creates a new call builder for the [`proposalBlocksLen`] function.
        pub fn proposalBlocksLen(
            &self,
            _claimant: alloy::sol_types::private::Address,
            _uuid: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposalBlocksLenCall, N> {
            self.call_builder(
                &proposalBlocksLenCall {
                    _claimant,
                    _uuid,
                },
            )
        }
        ///Creates a new call builder for the [`proposalBonds`] function.
        pub fn proposalBonds(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposalBondsCall, N> {
            self.call_builder(&proposalBondsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`proposalBranches`] function.
        pub fn proposalBranches(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
            _2: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposalBranchesCall, N> {
            self.call_builder(&proposalBranchesCall { _0, _1, _2 })
        }
        ///Creates a new call builder for the [`proposalCount`] function.
        pub fn proposalCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proposalCountCall, N> {
            self.call_builder(&proposalCountCall)
        }
        ///Creates a new call builder for the [`proposalMetadata`] function.
        pub fn proposalMetadata(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposalMetadataCall, N> {
            self.call_builder(&proposalMetadataCall { _0, _1 })
        }
        ///Creates a new call builder for the [`proposalParts`] function.
        pub fn proposalParts(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposalPartsCall, N> {
            self.call_builder(&proposalPartsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`proposals`] function.
        pub fn proposals(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proposalsCall, N> {
            self.call_builder(&proposalsCall(_0))
        }
        ///Creates a new call builder for the [`readPreimage`] function.
        pub fn readPreimage(
            &self,
            _key: alloy::sol_types::private::FixedBytes<32>,
            _offset: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, readPreimageCall, N> {
            self.call_builder(&readPreimageCall { _key, _offset })
        }
        ///Creates a new call builder for the [`squeezeLPP`] function.
        pub fn squeezeLPP(
            &self,
            _claimant: alloy::sol_types::private::Address,
            _uuid: alloy::sol_types::private::primitives::aliases::U256,
            _stateMatrix: <LibKeccak::StateMatrix as alloy::sol_types::SolType>::RustType,
            _preState: <Leaf as alloy::sol_types::SolType>::RustType,
            _preStateProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            _postState: <Leaf as alloy::sol_types::SolType>::RustType,
            _postStateProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, squeezeLPPCall, N> {
            self.call_builder(
                &squeezeLPPCall {
                    _claimant,
                    _uuid,
                    _stateMatrix,
                    _preState,
                    _preStateProof,
                    _postState,
                    _postStateProof,
                },
            )
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
        ///Creates a new call builder for the [`zeroHashes`] function.
        pub fn zeroHashes(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, zeroHashesCall, N> {
            self.call_builder(&zeroHashesCall(_0))
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > PreimageOracleInstance<P, N> {
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

///Module containing a contract's types and functions.
/**

```solidity
library Types {
    struct OutputRootProof { bytes32 version; bytes32 stateRoot; bytes32 messagePasserStorageRoot; bytes32 latestBlockhash; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Types {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct OutputRootProof { bytes32 version; bytes32 stateRoot; bytes32 messagePasserStorageRoot; bytes32 latestBlockhash; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutputRootProof {
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub stateRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub messagePasserStorageRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub latestBlockhash: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<OutputRootProof> for UnderlyingRustTuple<'_> {
            fn from(value: OutputRootProof) -> Self {
                (
                    value.version,
                    value.stateRoot,
                    value.messagePasserStorageRoot,
                    value.latestBlockhash,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutputRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    version: tuple.0,
                    stateRoot: tuple.1,
                    messagePasserStorageRoot: tuple.2,
                    latestBlockhash: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OutputRootProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OutputRootProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.stateRoot),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.messagePasserStorageRoot,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.latestBlockhash),
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
        impl alloy_sol_types::SolType for OutputRootProof {
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
        impl alloy_sol_types::SolStruct for OutputRootProof {
            const NAME: &'static str = "OutputRootProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OutputRootProof(bytes32 version,bytes32 stateRoot,bytes32 messagePasserStorageRoot,bytes32 latestBlockhash)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.version)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stateRoot)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.messagePasserStorageRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.latestBlockhash,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OutputRootProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.version,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stateRoot,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.messagePasserStorageRoot,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.latestBlockhash,
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.version,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stateRoot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.messagePasserStorageRoot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.latestBlockhash,
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
    /**Creates a new wrapper around an on-chain [`Types`](self) contract instance.

See the [wrapper's documentation](`TypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, __provider: P) -> TypesInstance<P, N> {
        TypesInstance::<P, N>::new(address, __provider)
    }
    /**A [`Types`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Types`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for TypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TypesInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Types`](self) contract instance.

See the [wrapper's documentation](`TypesInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> TypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TypesInstance<P, N> {
            TypesInstance {
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
    > TypesInstance<P, N> {
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
    > TypesInstance<P, N> {
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
library Types {
    struct OutputRootProof {
        bytes32 version;
        bytes32 stateRoot;
        bytes32 messagePasserStorageRoot;
        bytes32 latestBlockhash;
    }
}

interface FaultDisputeGame {
    type BondDistributionMode is uint8;
    type GameStatus is uint8;
    type Claim is bytes32;
    type Clock is uint128;
    type Duration is uint64;
    type GameType is uint32;
    type Hash is bytes32;
    type Position is uint128;
    type Timestamp is uint64;
    struct GameConstructorParams {
        GameType gameType;
        Claim absolutePrestate;
        uint256 maxGameDepth;
        uint256 splitDepth;
        Duration clockExtension;
        Duration maxClockDuration;
        address vm;
        address weth;
        address anchorStateRegistry;
        uint256 l2ChainId;
    }

    error AlreadyInitialized();
    error AnchorRootNotFound();
    error BadExtraData();
    error BlockNumberMatches();
    error BondTransferFailed();
    error CannotDefendRootClaim();
    error ClaimAboveSplit();
    error ClaimAlreadyExists();
    error ClaimAlreadyResolved();
    error ClockNotExpired();
    error ClockTimeExceeded();
    error ContentLengthMismatch();
    error DuplicateStep();
    error EmptyItem();
    error GameDepthExceeded();
    error GameNotFinalized();
    error GameNotInProgress();
    error GameNotResolved();
    error GamePaused();
    error IncorrectBondAmount();
    error InvalidBondDistributionMode();
    error InvalidChallengePeriod();
    error InvalidClockExtension();
    error InvalidDataRemainder();
    error InvalidDisputedClaimIndex();
    error InvalidHeader();
    error InvalidHeaderRLP();
    error InvalidLocalIdent();
    error InvalidOutputRootProof();
    error InvalidParent();
    error InvalidPrestate();
    error InvalidSplitDepth();
    error L2BlockNumberChallenged();
    error MaxDepthTooLarge();
    error NoCreditToClaim();
    error OutOfOrderResolution();
    error ReservedGameType();
    error UnexpectedList();
    error UnexpectedRootClaim(Claim rootClaim);
    error UnexpectedString();
    error ValidStep();

    event GameClosed(BondDistributionMode bondDistributionMode);
    event Move(uint256 indexed parentIndex, Claim indexed claim, address indexed claimant);
    event Resolved(GameStatus indexed status);

    constructor(GameConstructorParams _params);

    function absolutePrestate() external view returns (Claim absolutePrestate_);
    function addLocalData(uint256 _ident, uint256 _execLeafIdx, uint256 _partOffset) external;
    function anchorStateRegistry() external view returns (address registry_);
    function attack(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
    function bondDistributionMode() external view returns (BondDistributionMode);
    function challengeRootL2Block(Types.OutputRootProof memory _outputRootProof, bytes memory _headerRLP) external;
    function claimCredit(address _recipient) external;
    function claimData(uint256) external view returns (uint32 parentIndex, address counteredBy, address claimant, uint128 bond, Claim claim, Position position, Clock clock);
    function claimDataLen() external view returns (uint256 len_);
    function claims(Hash) external view returns (bool);
    function clockExtension() external view returns (Duration clockExtension_);
    function closeGame() external;
    function createdAt() external view returns (Timestamp);
    function credit(address _recipient) external view returns (uint256 credit_);
    function defend(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
    function extraData() external pure returns (bytes memory extraData_);
    function gameCreator() external pure returns (address creator_);
    function gameData() external view returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_);
    function gameType() external view returns (GameType gameType_);
    function getChallengerDuration(uint256 _claimIndex) external view returns (Duration duration_);
    function getNumToResolve(uint256 _claimIndex) external view returns (uint256 numRemainingChildren_);
    function getRequiredBond(Position _position) external view returns (uint256 requiredBond_);
    function hasUnlockedCredit(address) external view returns (bool);
    function initialize() external payable;
    function l1Head() external pure returns (Hash l1Head_);
    function l2BlockNumber() external pure returns (uint256 l2BlockNumber_);
    function l2BlockNumberChallenged() external view returns (bool);
    function l2BlockNumberChallenger() external view returns (address);
    function l2ChainId() external view returns (uint256 l2ChainId_);
    function l2SequenceNumber() external pure returns (uint256 l2SequenceNumber_);
    function maxClockDuration() external view returns (Duration maxClockDuration_);
    function maxGameDepth() external view returns (uint256 maxGameDepth_);
    function move(Claim _disputed, uint256 _challengeIndex, Claim _claim, bool _isAttack) external payable;
    function normalModeCredit(address) external view returns (uint256);
    function refundModeCredit(address) external view returns (uint256);
    function resolutionCheckpoints(uint256) external view returns (bool initialCheckpointComplete, uint32 subgameIndex, Position leftmostPosition, address counteredBy);
    function resolve() external returns (GameStatus status_);
    function resolveClaim(uint256 _claimIndex, uint256 _numToResolve) external;
    function resolvedAt() external view returns (Timestamp);
    function resolvedSubgames(uint256) external view returns (bool);
    function rootClaim() external pure returns (Claim rootClaim_);
    function splitDepth() external view returns (uint256 splitDepth_);
    function startingBlockNumber() external view returns (uint256 startingBlockNumber_);
    function startingOutputRoot() external view returns (Hash root, uint256 l2SequenceNumber);
    function startingRootHash() external view returns (Hash startingRootHash_);
    function status() external view returns (GameStatus);
    function step(uint256 _claimIndex, bool _isAttack, bytes memory _stateData, bytes memory _proof) external;
    function subgames(uint256, uint256) external view returns (uint256);
    function version() external pure returns (string memory);
    function vm() external view returns (address vm_);
    function wasRespectedGameTypeWhenCreated() external view returns (bool);
    function weth() external view returns (address weth_);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_params",
        "type": "tuple",
        "internalType": "struct FaultDisputeGame.GameConstructorParams",
        "components": [
          {
            "name": "gameType",
            "type": "uint32",
            "internalType": "GameType"
          },
          {
            "name": "absolutePrestate",
            "type": "bytes32",
            "internalType": "Claim"
          },
          {
            "name": "maxGameDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "splitDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "clockExtension",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "maxClockDuration",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "vm",
            "type": "address",
            "internalType": "contract IBigStepper"
          },
          {
            "name": "weth",
            "type": "address",
            "internalType": "contract IDelayedWETH"
          },
          {
            "name": "anchorStateRegistry",
            "type": "address",
            "internalType": "contract IAnchorStateRegistry"
          },
          {
            "name": "l2ChainId",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "absolutePrestate",
    "inputs": [],
    "outputs": [
      {
        "name": "absolutePrestate_",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addLocalData",
    "inputs": [
      {
        "name": "_ident",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_execLeafIdx",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "anchorStateRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "registry_",
        "type": "address",
        "internalType": "contract IAnchorStateRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "attack",
    "inputs": [
      {
        "name": "_disputed",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_parentIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_claim",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "bondDistributionMode",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum BondDistributionMode"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "challengeRootL2Block",
    "inputs": [
      {
        "name": "_outputRootProof",
        "type": "tuple",
        "internalType": "struct Types.OutputRootProof",
        "components": [
          {
            "name": "version",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stateRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "messagePasserStorageRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "latestBlockhash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "_headerRLP",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimCredit",
    "inputs": [
      {
        "name": "_recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimData",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "parentIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "counteredBy",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "claimant",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "bond",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "claim",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "position",
        "type": "uint128",
        "internalType": "Position"
      },
      {
        "name": "clock",
        "type": "uint128",
        "internalType": "Clock"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "claimDataLen",
    "inputs": [],
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
    "name": "claims",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "Hash"
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
    "name": "clockExtension",
    "inputs": [],
    "outputs": [
      {
        "name": "clockExtension_",
        "type": "uint64",
        "internalType": "Duration"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "closeGame",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createdAt",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "Timestamp"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "credit",
    "inputs": [
      {
        "name": "_recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "credit_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "defend",
    "inputs": [
      {
        "name": "_disputed",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_parentIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_claim",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "extraData",
    "inputs": [],
    "outputs": [
      {
        "name": "extraData_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "gameCreator",
    "inputs": [],
    "outputs": [
      {
        "name": "creator_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "gameData",
    "inputs": [],
    "outputs": [
      {
        "name": "gameType_",
        "type": "uint32",
        "internalType": "GameType"
      },
      {
        "name": "rootClaim_",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "extraData_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "gameType",
    "inputs": [],
    "outputs": [
      {
        "name": "gameType_",
        "type": "uint32",
        "internalType": "GameType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getChallengerDuration",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "duration_",
        "type": "uint64",
        "internalType": "Duration"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getNumToResolve",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "numRemainingChildren_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRequiredBond",
    "inputs": [
      {
        "name": "_position",
        "type": "uint128",
        "internalType": "Position"
      }
    ],
    "outputs": [
      {
        "name": "requiredBond_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "hasUnlockedCredit",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "l1Head",
    "inputs": [],
    "outputs": [
      {
        "name": "l1Head_",
        "type": "bytes32",
        "internalType": "Hash"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "l2BlockNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "l2BlockNumber_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "l2BlockNumberChallenged",
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
    "name": "l2BlockNumberChallenger",
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
    "name": "l2ChainId",
    "inputs": [],
    "outputs": [
      {
        "name": "l2ChainId_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "l2SequenceNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "l2SequenceNumber_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "maxClockDuration",
    "inputs": [],
    "outputs": [
      {
        "name": "maxClockDuration_",
        "type": "uint64",
        "internalType": "Duration"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "maxGameDepth",
    "inputs": [],
    "outputs": [
      {
        "name": "maxGameDepth_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "move",
    "inputs": [
      {
        "name": "_disputed",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_challengeIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_claim",
        "type": "bytes32",
        "internalType": "Claim"
      },
      {
        "name": "_isAttack",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "normalModeCredit",
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
    "name": "refundModeCredit",
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
    "name": "resolutionCheckpoints",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "initialCheckpointComplete",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "subgameIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "leftmostPosition",
        "type": "uint128",
        "internalType": "Position"
      },
      {
        "name": "counteredBy",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "resolve",
    "inputs": [],
    "outputs": [
      {
        "name": "status_",
        "type": "uint8",
        "internalType": "enum GameStatus"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resolveClaim",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_numToResolve",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resolvedAt",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "Timestamp"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "resolvedSubgames",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "rootClaim",
    "inputs": [],
    "outputs": [
      {
        "name": "rootClaim_",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "splitDepth",
    "inputs": [],
    "outputs": [
      {
        "name": "splitDepth_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "startingBlockNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "startingBlockNumber_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "startingOutputRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "Hash"
      },
      {
        "name": "l2SequenceNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "startingRootHash",
    "inputs": [],
    "outputs": [
      {
        "name": "startingRootHash_",
        "type": "bytes32",
        "internalType": "Hash"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "status",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum GameStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "step",
    "inputs": [
      {
        "name": "_claimIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_isAttack",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_stateData",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "_proof",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "subgames",
    "inputs": [
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
        "type": "uint256",
        "internalType": "uint256"
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
    "type": "function",
    "name": "vm",
    "inputs": [],
    "outputs": [
      {
        "name": "vm_",
        "type": "address",
        "internalType": "contract IBigStepper"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "wasRespectedGameTypeWhenCreated",
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
    "name": "weth",
    "inputs": [],
    "outputs": [
      {
        "name": "weth_",
        "type": "address",
        "internalType": "contract IDelayedWETH"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "GameClosed",
    "inputs": [
      {
        "name": "bondDistributionMode",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum BondDistributionMode"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Move",
    "inputs": [
      {
        "name": "parentIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "claim",
        "type": "bytes32",
        "indexed": true,
        "internalType": "Claim"
      },
      {
        "name": "claimant",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Resolved",
    "inputs": [
      {
        "name": "status",
        "type": "uint8",
        "indexed": true,
        "internalType": "enum GameStatus"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AnchorRootNotFound",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BadExtraData",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BlockNumberMatches",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BondTransferFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotDefendRootClaim",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClaimAboveSplit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClaimAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClaimAlreadyResolved",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClockNotExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ClockTimeExceeded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ContentLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DuplicateStep",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyItem",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameDepthExceeded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameNotFinalized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameNotInProgress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GameNotResolved",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GamePaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "IncorrectBondAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBondDistributionMode",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidChallengePeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidClockExtension",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDataRemainder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDisputedClaimIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHeader",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHeaderRLP",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLocalIdent",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOutputRootProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidParent",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPrestate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSplitDepth",
    "inputs": []
  },
  {
    "type": "error",
    "name": "L2BlockNumberChallenged",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MaxDepthTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoCreditToClaim",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfOrderResolution",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReservedGameType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnexpectedList",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnexpectedRootClaim",
    "inputs": [
      {
        "name": "rootClaim",
        "type": "bytes32",
        "internalType": "Claim"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnexpectedString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidStep",
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
pub mod FaultDisputeGame {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101c06040523480156200001257600080fd5b50604051620062933803806200629383398101604081905262000035916200052e565b620000436001607e62000601565b60ff16816040015111156200006b57604051633beff19960e11b815260040160405180910390fd5b60001981606001511480620000955750604081015160608201516200009290600162000627565b10155b15620000b45760405163e62ccf3960e01b815260040160405180910390fd5b600281606001511015620000db5760405163e62ccf3960e01b815260040160405180910390fd5b6001600160401b0380168160c001516001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000128573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200014e919062000642565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200018c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001b2919062000662565b1115620001d25760405163b4e1243360e01b815260040160405180910390fd5b6000620001f782608001516001600160401b03166200048060201b62000c701760201c565b6200020d906001600160401b031660026200067c565b905060008260c001516001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000254573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200027a919062000642565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015620002b8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002de919062000662565b6200030184608001516001600160401b03166200048060201b62000c701760201c565b6001600160401b031662000316919062000627565b905060006200033183836200048360201b62003aa61760201c565b90506001600160401b038111156200035c5760405163235dfb2b60e21b815260040160405180910390fd5b6200037f8460a001516001600160401b03166200048060201b62000c701760201c565b6001600160401b0316816001600160401b03161115620003b25760405163235dfb2b60e21b815260040160405180910390fd5b63ffffffff8016620003d9856000015163ffffffff166200048060201b62000c701760201c565b63ffffffff1603620003fe5760405163073ed16760e31b815260040160405180910390fd5b5050815163ffffffff1661012090815260208301516080908152604084015160a0908152606085015160c0908152918501516001600160401b039081166101a052908501511660e0908152908401516001600160a01b03908116610100908152918501518116610140529084015116610160529091015161018052506200069e565b90565b60008183101562000495578162000497565b825b9392505050565b60405161014081016001600160401b0381118282101715620004d057634e487b7160e01b600052604160045260246000fd5b60405290565b805163ffffffff81168114620004eb57600080fd5b919050565b80516001600160401b0381168114620004eb57600080fd5b6001600160a01b03811681146200051e57600080fd5b50565b8051620004eb8162000508565b600061014082840312156200054257600080fd5b6200054c6200049e565b6200055783620004d6565b81526020830151602082015260408301516040820152606083015160608201526200058560808401620004f0565b60808201526200059860a08401620004f0565b60a0820152620005ab60c0840162000521565b60c0820152620005be60e0840162000521565b60e0820152610100620005d381850162000521565b90820152610120928301519281019290925250919050565b634e487b7160e01b600052601160045260246000fd5b600060ff821660ff8416808210156200061e576200061e620005eb565b90039392505050565b600082198211156200063d576200063d620005eb565b500190565b6000602082840312156200065557600080fd5b8151620004978162000508565b6000602082840312156200067557600080fd5b5051919050565b6000816000190483118215151615620006995762000699620005eb565b500290565b60805160a05160c05160e05161010051610120516101405161016051610180516101a051615a53620008406000396000818161072f01528181611e7c01528181611ee70152611f1a015260008181610a4901526139e40152600081816106a5015281816123c60152818161250a015281816125e401528181612673015281816127b50152612c68015260008181610593015281816117aa015281816118a6015281816122830152612b6f0152600081816108cc01528181612c3f0152613a6601526000818161053001528181611d950152818161341501526136e6015260008181610a9c01528181610f0301528181611d0501528181611f4a01528181611fa901528181612e2b0152612e6d015260008181610acf01528181611b4e01528181611c7401528181611eb80152818161328c0152818161388a01528181613ea50152818161457e0152818161469a01528181614779015261482c015260008181610b7601528181611c1701528181611d6901528181612f1901528181612f9f015281816131a401526132ad0152600081816107f101526133300152615a536000f3fe60806040526004361061033f5760003560e01c806370872aa5116101b0578063c395e1ca116100ec578063dabd396d11610095578063f8f43ff61161006f578063f8f43ff614610b23578063fa24f74314610b43578063fa315aa914610b67578063fe2bbeb214610b9a57600080fd5b8063dabd396d14610a8d578063ec5e630814610ac0578063eff0f59214610af357600080fd5b8063d5d44d80116100c6578063d5d44d8014610a1a578063d6ae3cd514610a3a578063d8cc1a3c14610a6d57600080fd5b8063c395e1ca14610965578063c6f0308c14610985578063cf09e0d0146109f957600080fd5b80638d450a9511610159578063bbdc02db11610133578063bbdc02db146108b5578063bcef3b55146108f6578063bd8da95614610918578063c0d8bb741461093857600080fd5b80638d450a95146107e257806399735e32146107c0578063a445ece61461081557600080fd5b80638129fc1c1161018a5780638129fc1c146107a35780638980e0cc146107ab5780638b85902b146107c057600080fd5b806370872aa514610766578063786b844b1461077b5780637b0f0adc1461079057600080fd5b80633e3ac9121161027f5780635a5fa2d91161022857806360e274641161020257806360e27464146106de5780636361506d146106fe5780636b6716c0146107205780636f0344091461075357600080fd5b80635a5fa2d9146106765780635c0cba3314610696578063609d3334146106c957600080fd5b8063529d6a8c11610259578063529d6a8c146105ca57806354fd4d50146105f757806357da950e1461064657600080fd5b80633e3ac912146105545780633fc8cef314610584578063472777c6146105b757600080fd5b806325fc2ace116102ec57806330dbe570116102c657806330dbe570146104ad578063378dd48c146104e557806337b1b229146104ff5780633a7684631461052157600080fd5b806325fc2ace146104595780632810e1d6146104785780632ad69aeb1461048d57600080fd5b8063200d2ed21161031d578063200d2ed2146103d1578063222abf45146103ff578063250e69bd1461043f57600080fd5b8063019351301461034457806303c2924d1461036657806319effeb414610386575b600080fd5b34801561035057600080fd5b5061036461035f3660046152c3565b610bca565b005b34801561037257600080fd5b5061036461038136600461531e565b610e89565b34801561039257600080fd5b506000546103b39068010000000000000000900467ffffffffffffffff1681565b60405167ffffffffffffffff90911681526020015b60405180910390f35b3480156103dd57600080fd5b506000546103f290600160801b900460ff1681565b6040516103c89190615377565b34801561040b57600080fd5b5061042f61041a36600461539f565b600c6020526000908152604090205460ff1681565b60405190151581526020016103c8565b34801561044b57600080fd5b50600a5461042f9060ff1681565b34801561046557600080fd5b506008545b6040519081526020016103c8565b34801561048457600080fd5b506103f2611411565b34801561049957600080fd5b5061046a6104a836600461531e565b6115e9565b3480156104b957600080fd5b506001546104cd906001600160a01b031681565b6040516001600160a01b0390911681526020016103c8565b3480156104f157600080fd5b50600d546103f29060ff1681565b34801561050b57600080fd5b503660011981013560f01c90033560601c6104cd565b34801561052d57600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006104cd565b34801561056057600080fd5b5060005461042f907201000000000000000000000000000000000000900460ff1681565b34801561059057600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006104cd565b6103646105c53660046153bc565b61161f565b3480156105d657600080fd5b5061046a6105e536600461539f565b60036020526000908152604090205481565b34801561060357600080fd5b5060408051808201909152600581527f312e382e3000000000000000000000000000000000000000000000000000000060208201525b6040516103c89190615453565b34801561065257600080fd5b50600854600954610661919082565b604080519283526020830191909152016103c8565b34801561068257600080fd5b5061046a610691366004615466565b611631565b3480156106a257600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006104cd565b3480156106d557600080fd5b5061063961166b565b3480156106ea57600080fd5b506103646106f936600461539f565b611679565b34801561070a57600080fd5b503660011981013560f01c90036034013561046a565b34801561072c57600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103b3565b61036461076136600461548d565b611991565b34801561077257600080fd5b5060095461046a565b34801561078757600080fd5b50610364612332565b61036461079e3660046153bc565b612751565b61036461275e565b3480156107b757600080fd5b5060025461046a565b3480156107cc57600080fd5b503660011981013560f01c90036054013561046a565b3480156107ee57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b34801561082157600080fd5b50610877610830366004615466565b6007602052600090815260409020805460019091015460ff821691610100810463ffffffff1691650100000000009091046001600160801b0316906001600160a01b031684565b60408051941515855263ffffffff90931660208501526001600160801b03909116918301919091526001600160a01b031660608201526080016103c8565b3480156108c157600080fd5b5060405163ffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016103c8565b34801561090257600080fd5b503660011981013560f01c90036014013561046a565b34801561092457600080fd5b506103b3610933366004615466565b612cfb565b34801561094457600080fd5b5061046a61095336600461539f565b600b6020526000908152604090205481565b34801561097157600080fd5b5061046a6109803660046154ce565b612e95565b34801561099157600080fd5b506109a56109a0366004615466565b613056565b6040805163ffffffff90981688526001600160a01b03968716602089015295909416948601949094526001600160801b039182166060860152608085015291821660a08401521660c082015260e0016103c8565b348015610a0557600080fd5b506000546103b39067ffffffffffffffff1681565b348015610a2657600080fd5b5061046a610a3536600461539f565b6130ca565b348015610a4657600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b348015610a7957600080fd5b50610364610a883660046154f7565b613122565b348015610a9957600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103b3565b348015610acc57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b348015610aff57600080fd5b5061042f610b0e366004615466565b60046020526000908152604090205460ff1681565b348015610b2f57600080fd5b50610364610b3e3660046153bc565b61367e565b348015610b4f57600080fd5b50610b58613a64565b6040516103c893929190615583565b348015610b7357600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b348015610ba657600080fd5b5061042f610bb5366004615466565b60066020526000908152604090205460ff1681565b60008054600160801b900460ff166002811115610be957610be9615340565b14610c075760405163067fe19560e41b815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff1615610c5a576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610c733660011981013560f01c90036014013590565b90565b610c8a610c85368690038601866155be565b613ac1565b14610cc1576040517f9cc00b5b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b82606001358282604051610cd6929190615632565b604051809103902014610d15576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610d5e610d5984848080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250613b1d92505050565b613b8a565b90506000610d8582600881518110610d7857610d78615642565b6020026020010151613d40565b9050602081511115610dc3576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602081810151825190910360031b1c3660011981013560f01c9003605401358103610e1a576040517fb8ed883000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050600180547fffffffffffffffffffffffff000000000000000000000000000000000000000016331790555050600080547fffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffff1672010000000000000000000000000000000000001790555050565b60008054600160801b900460ff166002811115610ea857610ea8615340565b14610ec65760405163067fe19560e41b815260040160405180910390fd5b600060028381548110610edb57610edb615642565b906000526020600020906005020190506000610ef684612cfb565b905067ffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081169082161015610f5f576040517ff2440b5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008481526006602052604090205460ff1615610fa8576040517ff1a9458100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000848152600560205260409020805480158015610fc557508515155b1561102857835464010000000090046001600160a01b031660008115610feb5781610ffa565b60018601546001600160a01b03165b90506110068187613df4565b505050600094855250506006602052505060409020805460ff19166001179055565b6000868152600760209081526040918290208251608081018452815460ff81161515808352610100820463ffffffff16948301949094526501000000000090046001600160801b031693810193909352600101546001600160a01b031660608301526110ac576001600160801b0360408201526001815260008690036110ac578195505b600086826020015163ffffffff166110c4919061566e565b905060008382116110d557816110d7565b835b602084015190915063ffffffff165b818110156111f757600086828154811061110257611102615642565b6000918252602080832090910154808352600690915260409091205490915060ff1661115a576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006002828154811061116f5761116f615642565b60009182526020909120600590910201805490915064010000000090046001600160a01b03161580156111b65750600481015460408701516001600160801b039182169116115b156111e25760018101546001600160a01b0316606087015260048101546001600160801b031660408701525b505080806111ef90615686565b9150506110e6565b5063ffffffff818116602085810191825260008c81526007909152604090819020865181549351928801517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009094169015157fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000ff161761010092909416918202939093177fffffffffffffffffffffff00000000000000000000000000000000ffffffffff16650100000000006001600160801b03909316929092029190911782556060850151600190920180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b039093169290921790915584900361140657606083015160008a8152600660205260409020805460ff191660011790558915801561134357506000547201000000000000000000000000000000000000900460ff165b1561139e576001546001600160a01b031661135e818a613df4565b88546001600160a01b03909116640100000000027fffffffffffffffff0000000000000000000000000000000000000000ffffffff909116178855611404565b6113cb6001600160a01b038216156113b657816113c5565b60018901546001600160a01b03165b89613df4565b87547fffffffffffffffff0000000000000000000000000000000000000000ffffffff166401000000006001600160a01b038316021788555b505b505050505050505050565b600080600054600160801b900460ff16600281111561143257611432615340565b146114505760405163067fe19560e41b815260040160405180910390fd5b6000805260066020527f54cdd369e4e8a8515e52ca72ec816c2101831ad1f18bf44102ed171459c9b4f85460ff166114b4576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001600160a01b031660026000815481106114d3576114d3615642565b600091825260209091206005909102015464010000000090046001600160a01b031614611501576001611504565b60025b6000805467ffffffffffffffff421668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff82168117835592935083927fffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffff167fffffffffffffffffffffffffffffff000000000000000000ffffffffffffffff90911617600160801b8360028111156115a8576115a8615340565b0217905560028111156115bd576115bd615340565b6040517f5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da6090600090a290565b6005602052816000526040600020818154811061160557600080fd5b90600052602060002001600091509150505481565b905090565b61162c8383836001611991565b505050565b6000818152600760209081526040808320600590925282208054825461166290610100900463ffffffff16826156a0565b95945050505050565b606061161a60546020613e36565b611681612332565b60006002600d5460ff16600281111561169c5761169c615340565b036116c057506001600160a01b0381166000908152600b602052604090205461172f565b6001600d5460ff1660028111156116d9576116d9615340565b036116fd57506001600160a01b03811660009081526003602052604090205461172f565b6040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0382166000908152600c602052604090205460ff1661180c576001600160a01b038281166000818152600c602052604090819020805460ff19166001179055517f7eee288d0000000000000000000000000000000000000000000000000000000081526004810191909152602481018390527f000000000000000000000000000000000000000000000000000000000000000090911690637eee288d90604401600060405180830381600087803b1580156117f057600080fd5b505af1158015611804573d6000803e3d6000fd5b505050505050565b80600003611846576040517f17bfe5f700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b038281166000818152600b60209081526040808320839055600390915280822091909155517ff3fef3a30000000000000000000000000000000000000000000000000000000081526004810191909152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063f3fef3a390604401600060405180830381600087803b1580156118ec57600080fd5b505af1158015611900573d6000803e3d6000fd5b505050506000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114611951576040519150601f19603f3d011682016040523d82523d6000602084013e611956565b606091505b505090508061162c576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008054600160801b900460ff1660028111156119b0576119b0615340565b146119ce5760405163067fe19560e41b815260040160405180910390fd5b6000600284815481106119e3576119e3615642565b60009182526020918290206040805160e0810182526005909302909101805463ffffffff811684526001600160a01b0364010000000090910481169484019490945260018101549093169082015260028201546001600160801b03908116606083015260038301546080830181905260049093015480821660a0840152600160801b90041660c082015291508514611aa7576040517f3014033200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60a0810151600083156001600160801b0383161760011b90506000611b3c826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050861580611b775750611b747f0000000000000000000000000000000000000000000000000000000000000000600261566e565b81145b8015611b81575084155b15611bb8576040517fa42637bc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff168015611bde575086155b15611c15576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000811115611c6f576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611c9a7f0000000000000000000000000000000000000000000000000000000000000000600161566e565b8103611cac57611cac86888588613e6a565b34611cb683612e95565b14611ced576040517f8620aa1900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611cf888612cfb565b905067ffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000811690821603611d60576040517f3381d11400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611d8d60017f00000000000000000000000000000000000000000000000000000000000000006156a0565b8303611eb1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015611df1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e1591906156b7565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015611e52573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e7691906156d4565b611eaa907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166156ed565b9050611f44565b611edc60017f00000000000000000000000000000000000000000000000000000000000000006156a0565b8303611f1757611eaa7f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615719565b507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff165b611f78817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615749565b67ffffffffffffffff16611f938367ffffffffffffffff1690565b67ffffffffffffffff161115611fda57611fd7817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615749565b91505b6000604083901b421760008a8152608087901b6001600160801b038d1617602052604081209192509060008181526004602052604090205490915060ff161561204f576040517f80497e3b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60016004600083815260200190815260200160002060006101000a81548160ff02191690831515021790555060026040518060e001604052808d63ffffffff16815260200160006001600160a01b03168152602001336001600160a01b03168152602001346001600160801b031681526020018c8152602001886001600160801b03168152602001846001600160801b0316815250908060018154018082558091505060019003906000526020600020906005020160009091909190915060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a8154816001600160a01b0302191690836001600160a01b0316021790555060408201518160010160006101000a8154816001600160a01b0302191690836001600160a01b0316021790555060608201518160020160006101000a8154816001600160801b0302191690836001600160801b031602179055506080820151816003015560a08201518160040160006101000a8154816001600160801b0302191690836001600160801b0316021790555060c08201518160040160106101000a8154816001600160801b0302191690836001600160801b031602179055505050600560008c8152602001908152602001600020600160028054905061224691906156a0565b81546001810183556000928352602080842090910191909155338252600b905260408120805434929061227a90849061566e565b925050819055507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156122dc57600080fd5b505af11580156122f0573d6000803e3d6000fd5b50506040513393508d92508e91507f9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be90600090a4505050505050505050505050565b6002600d5460ff16600281111561234b5761234b615340565b148061236d57506001600d5460ff16600281111561236b5761236b615340565b145b1561237457565b6000600d5460ff16600281111561238d5761238d615340565b146123c4576040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316635c975abb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612422573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124469190615772565b1561247d576040517f379a7ed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60005468010000000000000000900467ffffffffffffffff1667ffffffffffffffff166000036124d9576040517fc105260a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f0314d2b30000000000000000000000000000000000000000000000000000000081523060048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690630314d2b390602401602060405180830381865afa158015612559573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061257d9190615772565b9050806125b6576040517f4851bd9b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f17cf21a90000000000000000000000000000000000000000000000000000000081523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906317cf21a990602401600060405180830381600087803b15801561263057600080fd5b505af1925050508015612641575060015b506040517f496b9c160000000000000000000000000000000000000000000000000000000081523060048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063496b9c1690602401602060405180830381865afa1580156126c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126e69190615772565b9050801561270057600d805460ff1916600117905561270e565b600d805460ff191660021790555b600d546040517f9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f916127459160ff90911690615377565b60405180910390a15050565b61162c8383836000611991565b60005471010000000000000000000000000000000000900460ff16156127b0576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d83ef2676040518163ffffffff1660e01b81526004016040805180830381865afa158015612810573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612834919061578f565b909250905081612870576040517f6a6bc3b200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820190915282815260200181905260088290556009819055607a36146128c7576040517f9824bdab00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b803660011981013560f01c90036054013511612925576040517ff40239db0000000000000000000000000000000000000000000000000000000081523660011981013560f01c90036014013560048201526024015b60405180910390fd5b6040805160e08101825263ffffffff808252600060208084018281523660011981013560f01c90038035606090811c8789018181526001600160801b0334818116948b0194855260149095013560808b01908152600160a08c0181815242841660c08e019081526002805493840181558c529c517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace600590930292830180549a5191909d167fffffffffffffffff000000000000000000000000000000000000000000000000909a16999099176401000000006001600160a01b039a8b160217909b5592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5acf840180547fffffffffffffffffffffffff000000000000000000000000000000000000000016919098161790965592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad0820180547fffffffffffffffffffffffffffffffff000000000000000000000000000000001691851691909117905593517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad185015595519651968116600160801b9790911696909602959095177f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad2909101558154710100000000000000000000000000000000007fffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffff909116178255918152600b909152918220805491929091612b6690849061566e565b925050819055507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b158015612bc857600080fd5b505af1158015612bdc573d6000803e3d6000fd5b5050600080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000164267ffffffffffffffff161790555050604080517f3c9f397c00000000000000000000000000000000000000000000000000000000815290517f000000000000000000000000000000000000000000000000000000000000000063ffffffff1692507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691633c9f397c9160048083019260209291908290030181865afa158015612cbb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612cdf91906157b3565b600a805460ff191663ffffffff92909216929092141790555050565b600080600054600160801b900460ff166002811115612d1c57612d1c615340565b14612d3a5760405163067fe19560e41b815260040160405180910390fd5b600060028381548110612d4f57612d4f615642565b600091825260208220600590910201805490925063ffffffff90811614612db557815460028054909163ffffffff16908110612d8d57612d8d615642565b906000526020600020906005020160040160109054906101000a90046001600160801b031690505b6004820154600090612de090600160801b900467ffffffffffffffff165b67ffffffffffffffff1690565b612df49067ffffffffffffffff16426156a0565b612e0a612dd3846001600160801b031660401c90565b67ffffffffffffffff16612e1e919061566e565b905067ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff1611612e6b5780611662565b7f000000000000000000000000000000000000000000000000000000000000000095945050505050565b600080612f12836001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690507f0000000000000000000000000000000000000000000000000000000000000000811115612f71576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b642e90edd00062061a806311e1a3006000612f8c83836157ef565b9050670de0b6b3a76400006000612fc3827f0000000000000000000000000000000000000000000000000000000000000000615803565b90506000612fe1612fdc670de0b6b3a764000086615803565b613ff9565b90506000612fef848461424b565b90506000612ffd838361429a565b9050600061300a826142c8565b9050600061302982613024670de0b6b3a76400008f615803565b6144b0565b905060006130378b8361429a565b9050613043818d615803565b9f9e505050505050505050505050505050565b6002818154811061306657600080fd5b60009182526020909120600590910201805460018201546002830154600384015460049094015463ffffffff841695506401000000009093046001600160a01b03908116949216926001600160801b03918216929180821691600160801b90041687565b60006002600d5460ff1660028111156130e5576130e5615340565b0361310657506001600160a01b03166000908152600b602052604090205490565b506001600160a01b031660009081526003602052604090205490565b60008054600160801b900460ff16600281111561314157613141615340565b1461315f5760405163067fe19560e41b815260040160405180910390fd5b60006002878154811061317457613174615642565b6000918252602082206005919091020160048101549092506001600160801b0316908715821760011b90506131ca7f0000000000000000000000000000000000000000000000000000000000000000600161566e565b613244826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff161461327e576040517f5f53dd9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600080891561335a576132d17f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006156a0565b6001901b6132e7846001600160801b03166144e1565b6001600160801b03166132fa9190615822565b1561332e5761332561331660016001600160801b038716615836565b865463ffffffff166000614567565b60030154613350565b7f00000000000000000000000000000000000000000000000000000000000000005b915084905061337b565b600385015491506133786133166001600160801b0386166001615856565b90505b600882901b60088a8a604051613392929190615632565b6040518091039020901b146133d3576040517f696550ff00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006133de8c614630565b905060006133ed836003015490565b6040517fe14ced320000000000000000000000000000000000000000000000000000000081527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e14ced329061345a908f908f908f908f908a906004016158c1565b6020604051808303816000875af1158015613479573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061349d91906156d4565b600485015491149150600090600290613526906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6135a0896001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6135aa91906158fb565b6135b4919061591e565b60ff1615905081151581036135f5576040517ffb4e40dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b875464010000000090046001600160a01b03161561363f576040517f9071e6af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505085547fffffffffffffffff0000000000000000000000000000000000000000ffffffff163364010000000002179095555050505050505050505050565b60008054600160801b900460ff16600281111561369d5761369d615340565b146136bb5760405163067fe19560e41b815260040160405180910390fd5b6000806000806136ca8661465f565b935093509350935060006136e0858585856149b8565b905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015613742573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061376691906156b7565b905060018903613833576001600160a01b0381166352f0f3ad8a846137973660011981013560f01c90036034013590565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815260048101939093526024830191909152604482015260206064820152608481018a905260a4015b6020604051808303816000875af1158015613809573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061382d91906156d4565b50611406565b60028903613852576001600160a01b0381166352f0f3ad8a8489613797565b60038903613871576001600160a01b0381166352f0f3ad8a8487613797565b600489036139a65760006138ae6001600160801b0385167f0000000000000000000000000000000000000000000000000000000000000000614a57565b6009546138bb919061566e565b6138c690600161566e565b90503660011981013560f01c90036054013581106138f3573660011981013560f01c9003605401356138f5565b805b90506001600160a01b0382166352f0f3ad8b8560405160e084901b7fffffffff000000000000000000000000000000000000000000000000000000001681526004810192909252602482015260c084901b604482015260086064820152608481018b905260a4016020604051808303816000875af115801561397b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061399f91906156d4565b5050611406565b60058903613a32576040517f52f0f3ad000000000000000000000000000000000000000000000000000000008152600481018a9052602481018390527f000000000000000000000000000000000000000000000000000000000000000060c01b604482015260086064820152608481018890526001600160a01b038216906352f0f3ad9060a4016137ea565b6040517fff137e6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000003660011981013560f01c9003601401356060613a9f61166b565b9050909192565b600081831015613ab65781613ab8565b825b90505b92915050565b60008160000151826020015183604001518460600151604051602001613b00949392919093845260208401929092526040830152606082015260800190565b604051602081830303815290604052805190602001209050919050565b60408051808201909152600080825260208201528151600003613b6c576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b60606000806000613b9a85614aec565b919450925090506001816001811115613bb557613bb5615340565b14613bec576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8451613bf8838561566e565b14613c2f576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b6040805180820190915260008082526020820152815260200190600190039081613c465790505093506000835b8651811015613d3457600080613cb96040518060400160405280858c60000151613c9d91906156a0565b8152602001858c60200151613cb2919061566e565b9052614aec565b509150915060405180604001604052808383613cd5919061566e565b8152602001848b60200151613cea919061566e565b815250888581518110613cff57613cff615642565b6020908102919091010152613d1560018561566e565b9350613d21818361566e565b613d2b908461566e565b92505050613c73565b50845250919392505050565b60606000806000613d5085614aec565b919450925090506000816001811115613d6b57613d6b615340565b14613da2576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613dac828461566e565b855114613de5576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61166285602001518484614f8a565b60028101546001600160a01b038316600090815260036020526040812080546001600160801b0390931692909190613e2d90849061566e565b90915550505050565b6040518181523660011981013560f01c90038284820160208401378260208301016000815260208101604052505092915050565b6000613e806001600160801b0384166001615856565b90506000613e9082866001614567565b9050600086901a8380613f5a5750613ec960027f0000000000000000000000000000000000000000000000000000000000000000615822565b6004830154600290613f4b906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b613f55919061591e565b60ff16145b15613fb25760ff811660011480613f74575060ff81166002145b613fad576040517ff40239db0000000000000000000000000000000000000000000000000000000081526004810188905260240161291c565b613ff0565b60ff811615613ff0576040517ff40239db0000000000000000000000000000000000000000000000000000000081526004810188905260240161291c565b50505050505050565b6001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b176000821361404f57631615e6386000526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f218311670de0b6b3a76400000215820261428857637c5f487d6000526004601cfd5b50670de0b6b3a7640000919091020490565b6000816000190483118202156142b85763bac65e5b6000526004601cfd5b50670de0b6b3a764000091020490565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d782136142f657919050565b680755bf798b4a1bf1e582126143145763a37bfec96000526004601cfd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b6000613ab8670de0b6b3a7640000836144c886613ff9565b6144d29190615940565b6144dc91906159fc565b6142c8565b600080614555837e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b600160ff919091161b90920392915050565b600080826145a7576145a26001600160801b0386167f000000000000000000000000000000000000000000000000000000000000000061501f565b6145b9565b6145b9856001600160801b031661515e565b9050600284815481106145ce576145ce615642565b906000526020600020906005020191505b60048201546001600160801b0382811691161461462857815460028054909163ffffffff1690811061461357614613615642565b906000526020600020906005020191506145df565b509392505050565b60008060008060006146418661465f565b9350935093509350614655848484846149b8565b9695505050505050565b600080600080600085905060006002828154811061467f5761467f615642565b600091825260209091206004600590920201908101549091507f000000000000000000000000000000000000000000000000000000000000000090614734906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff161161476e576040517fb34b5c2200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000815b60048301547f000000000000000000000000000000000000000000000000000000000000000090614813906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16925082111561488857825463ffffffff166148527f0000000000000000000000000000000000000000000000000000000000000000600161566e565b830361485c578391505b6002818154811061486f5761486f615642565b9060005260206000209060050201935080945050614772565b600481810154908401546001600160801b0391821691166000816001600160801b03166148cd6148c1856001600160801b031660011c90565b6001600160801b031690565b6001600160801b031614905080156149665760006148f3836001600160801b03166144e1565b6001600160801b0316111561494357600061492361491b60016001600160801b038616615836565b896001614567565b6003810154600490910154909c506001600160801b03169a506149499050565b6008549a505b600386015460048701549099506001600160801b031697506149aa565b600061497f61491b6001600160801b0385166001615856565b6003808901546004808b015492840154930154909e506001600160801b039182169d50919b50169850505b505050505050509193509193565b60006001600160801b03841615614a135760408051602081018790526001600160801b038087169282019290925260608101859052908316608082015260a00160405160208183030381529060405280519060200120611662565b8282604051602001614a389291909182526001600160801b0316602082015260400190565b6040516020818303038152906040528051906020012095945050505050565b600080614acb847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690508083036001841b600180831b0386831b17039250505092915050565b60008060008360000151600003614b2f576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f8111614b54576000600160009450945094505050614f83565b60b78111614c6a576000614b696080836156a0565b905080876000015111614ba8576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff00000000000000000000000000000000000000000000000000000000000000169082148015614c2057507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b15614c57576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5060019550935060009250614f83915050565b60bf8111614dc8576000614c7f60b7836156a0565b905080876000015111614cbe576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614d20576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614d68576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614d72818461566e565b895111614dab576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614db683600161566e565b9750955060009450614f839350505050565b60f78111614e2d576000614ddd60c0836156a0565b905080876000015111614e1c576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600195509350849250614f83915050565b6000614e3a60f7836156a0565b905080876000015111614e79576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614edb576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614f23576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614f2d818461566e565b895111614f66576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614f7183600161566e565b9750955060019450614f839350505050565b9193909250565b60608167ffffffffffffffff811115614fa557614fa56155a8565b6040519080825280601f01601f191660200182016040528015614fcf576020820181803683370190505b5090508115615018576000614fe4848661566e565b90506020820160005b84811015615005578281015182820152602001614fed565b84811115615014576000858301525b5050505b9392505050565b60008161509c846001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116150b25763b34b5c226000526004601cfd5b6150bb8361515e565b905081615138826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1611613abb57613ab861514e83600161566e565b6001600160801b038316906151ea565b600081196001830116816151d9827e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169390931c8015179392505050565b60008061525e847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050808303600180821b0385821b179250505092915050565b60008083601f84011261528c57600080fd5b50813567ffffffffffffffff8111156152a457600080fd5b6020830191508360208285010111156152bc57600080fd5b9250929050565b600080600083850360a08112156152d957600080fd5b60808112156152e757600080fd5b50839250608084013567ffffffffffffffff81111561530557600080fd5b6153118682870161527a565b9497909650939450505050565b6000806040838503121561533157600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b6003811061537457634e487b7160e01b600052602160045260246000fd5b50565b6020810161538483615356565b91905290565b6001600160a01b038116811461537457600080fd5b6000602082840312156153b157600080fd5b81356150188161538a565b6000806000606084860312156153d157600080fd5b505081359360208301359350604090920135919050565b6000815180845260005b8181101561540e576020818501810151868301820152016153f2565b81811115615420576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b602081526000613ab860208301846153e8565b60006020828403121561547857600080fd5b5035919050565b801515811461537457600080fd5b600080600080608085870312156154a357600080fd5b84359350602085013592506040850135915060608501356154c38161547f565b939692955090935050565b6000602082840312156154e057600080fd5b81356001600160801b038116811461501857600080fd5b6000806000806000806080878903121561551057600080fd5b8635955060208701356155228161547f565b9450604087013567ffffffffffffffff8082111561553f57600080fd5b61554b8a838b0161527a565b9096509450606089013591508082111561556457600080fd5b5061557189828a0161527a565b979a9699509497509295939492505050565b63ffffffff8416815282602082015260606040820152600061166260608301846153e8565b634e487b7160e01b600052604160045260246000fd5b6000608082840312156155d057600080fd5b6040516080810181811067ffffffffffffffff8211171561560157634e487b7160e01b600052604160045260246000fd5b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b8183823760009101908152919050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561568157615681615658565b500190565b6000600019820361569957615699615658565b5060010190565b6000828210156156b2576156b2615658565b500390565b6000602082840312156156c957600080fd5b81516150188161538a565b6000602082840312156156e657600080fd5b5051919050565b600067ffffffffffffffff80831681851680830382111561571057615710615658565b01949350505050565b600067ffffffffffffffff8083168185168183048111821515161561574057615740615658565b02949350505050565b600067ffffffffffffffff8381169083168181101561576a5761576a615658565b039392505050565b60006020828403121561578457600080fd5b81516150188161547f565b600080604083850312156157a257600080fd5b505080516020909101519092909150565b6000602082840312156157c557600080fd5b815163ffffffff8116811461501857600080fd5b634e487b7160e01b600052601260045260246000fd5b6000826157fe576157fe6157d9565b500490565b600081600019048311821515161561581d5761581d615658565b500290565b600082615831576158316157d9565b500690565b60006001600160801b038381169083168181101561576a5761576a615658565b60006001600160801b0380831681851680830382111561571057615710615658565b8183528181602085013750600060208284010152600060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b6060815260006158d5606083018789615878565b82810360208401526158e8818688615878565b9150508260408301529695505050505050565b600060ff821660ff84168082101561591557615915615658565b90039392505050565b600060ff831680615931576159316157d9565b8060ff84160691505092915050565b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60008413600084138583048511828216161561598157615981615658565b7f800000000000000000000000000000000000000000000000000000000000000060008712868205881281841616156159bc576159bc615658565b600087129250878205871284841616156159d8576159d8615658565b878505871281841616156159ee576159ee615658565b505050929093029392505050565b600082615a0b57615a0b6157d9565b60001983147f800000000000000000000000000000000000000000000000000000000000000083141615615a4157615a41615658565b50059056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\xC0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0b\x938\x03\x80b\0b\x93\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x05.V[b\0\0C`\x01`~b\0\x06\x01V[`\xFF\x16\x81`@\x01Q\x11\x15b\0\0kW`@Qc;\xEF\xF1\x99`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x19\x81``\x01Q\x14\x80b\0\0\x95WP`@\x81\x01Q``\x82\x01Qb\0\0\x92\x90`\x01b\0\x06'V[\x10\x15[\x15b\0\0\xB4W`@Qc\xE6,\xCF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x81``\x01Q\x10\x15b\0\0\xDBW`@Qc\xE6,\xCF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16\x81`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01N\x91\x90b\0\x06BV[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xB2\x91\x90b\0\x06bV[\x11\x15b\0\x01\xD2W`@Qc\xB4\xE1$3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0b\0\x01\xF7\x82`\x80\x01Q`\x01`\x01`@\x1B\x03\x16b\0\x04\x80` \x1Bb\0\x0Cp\x17` \x1CV[b\0\x02\r\x90`\x01`\x01`@\x1B\x03\x16`\x02b\0\x06|V[\x90P`\0\x82`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x02TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02z\x91\x90b\0\x06BV[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x02\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02\xDE\x91\x90b\0\x06bV[b\0\x03\x01\x84`\x80\x01Q`\x01`\x01`@\x1B\x03\x16b\0\x04\x80` \x1Bb\0\x0Cp\x17` \x1CV[`\x01`\x01`@\x1B\x03\x16b\0\x03\x16\x91\x90b\0\x06'V[\x90P`\0b\0\x031\x83\x83b\0\x04\x83` \x1Bb\0:\xA6\x17` \x1CV[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\\W`@Qc#]\xFB+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\0\x03\x7F\x84`\xA0\x01Q`\x01`\x01`@\x1B\x03\x16b\0\x04\x80` \x1Bb\0\x0Cp\x17` \x1CV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15b\0\x03\xB2W`@Qc#]\xFB+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x80\x16b\0\x03\xD9\x85`\0\x01Qc\xFF\xFF\xFF\xFF\x16b\0\x04\x80` \x1Bb\0\x0Cp\x17` \x1CV[c\xFF\xFF\xFF\xFF\x16\x03b\0\x03\xFEW`@Qc\x07>\xD1g`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x81Qc\xFF\xFF\xFF\xFF\x16a\x01 \x90\x81R` \x83\x01Q`\x80\x90\x81R`@\x84\x01Q`\xA0\x90\x81R``\x85\x01Q`\xC0\x90\x81R\x91\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16a\x01\xA0R\x90\x85\x01Q\x16`\xE0\x90\x81R\x90\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\0\x90\x81R\x91\x85\x01Q\x81\x16a\x01@R\x90\x84\x01Q\x16a\x01`R\x90\x91\x01Qa\x01\x80RPb\0\x06\x9EV[\x90V[`\0\x81\x83\x10\x15b\0\x04\x95W\x81b\0\x04\x97V[\x82[\x93\x92PPPV[`@Qa\x01@\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x04\xD0WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x04\xEBW`\0\x80\xFD[\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x04\xEBW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x05\x1EW`\0\x80\xFD[PV[\x80Qb\0\x04\xEB\x81b\0\x05\x08V[`\0a\x01@\x82\x84\x03\x12\x15b\0\x05BW`\0\x80\xFD[b\0\x05Lb\0\x04\x9EV[b\0\x05W\x83b\0\x04\xD6V[\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01Rb\0\x05\x85`\x80\x84\x01b\0\x04\xF0V[`\x80\x82\x01Rb\0\x05\x98`\xA0\x84\x01b\0\x04\xF0V[`\xA0\x82\x01Rb\0\x05\xAB`\xC0\x84\x01b\0\x05!V[`\xC0\x82\x01Rb\0\x05\xBE`\xE0\x84\x01b\0\x05!V[`\xE0\x82\x01Ra\x01\0b\0\x05\xD3\x81\x85\x01b\0\x05!V[\x90\x82\x01Ra\x01 \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\0\x06\x1EWb\0\x06\x1Eb\0\x05\xEBV[\x90\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15b\0\x06=Wb\0\x06=b\0\x05\xEBV[P\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\x06UW`\0\x80\xFD[\x81Qb\0\x04\x97\x81b\0\x05\x08V[`\0` \x82\x84\x03\x12\x15b\0\x06uW`\0\x80\xFD[PQ\x91\x90PV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\0\x06\x99Wb\0\x06\x99b\0\x05\xEBV[P\x02\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0QaZSb\0\x08@`\09`\0\x81\x81a\x07/\x01R\x81\x81a\x1E|\x01R\x81\x81a\x1E\xE7\x01Ra\x1F\x1A\x01R`\0\x81\x81a\nI\x01Ra9\xE4\x01R`\0\x81\x81a\x06\xA5\x01R\x81\x81a#\xC6\x01R\x81\x81a%\n\x01R\x81\x81a%\xE4\x01R\x81\x81a&s\x01R\x81\x81a'\xB5\x01Ra,h\x01R`\0\x81\x81a\x05\x93\x01R\x81\x81a\x17\xAA\x01R\x81\x81a\x18\xA6\x01R\x81\x81a\"\x83\x01Ra+o\x01R`\0\x81\x81a\x08\xCC\x01R\x81\x81a,?\x01Ra:f\x01R`\0\x81\x81a\x050\x01R\x81\x81a\x1D\x95\x01R\x81\x81a4\x15\x01Ra6\xE6\x01R`\0\x81\x81a\n\x9C\x01R\x81\x81a\x0F\x03\x01R\x81\x81a\x1D\x05\x01R\x81\x81a\x1FJ\x01R\x81\x81a\x1F\xA9\x01R\x81\x81a.+\x01Ra.m\x01R`\0\x81\x81a\n\xCF\x01R\x81\x81a\x1BN\x01R\x81\x81a\x1Ct\x01R\x81\x81a\x1E\xB8\x01R\x81\x81a2\x8C\x01R\x81\x81a8\x8A\x01R\x81\x81a>\xA5\x01R\x81\x81aE~\x01R\x81\x81aF\x9A\x01R\x81\x81aGy\x01RaH,\x01R`\0\x81\x81a\x0Bv\x01R\x81\x81a\x1C\x17\x01R\x81\x81a\x1Di\x01R\x81\x81a/\x19\x01R\x81\x81a/\x9F\x01R\x81\x81a1\xA4\x01Ra2\xAD\x01R`\0\x81\x81a\x07\xF1\x01Ra30\x01RaZS`\0\xF3\xFE`\x80`@R`\x046\x10a\x03?W`\x005`\xE0\x1C\x80cp\x87*\xA5\x11a\x01\xB0W\x80c\xC3\x95\xE1\xCA\x11a\0\xECW\x80c\xDA\xBD9m\x11a\0\x95W\x80c\xF8\xF4?\xF6\x11a\0oW\x80c\xF8\xF4?\xF6\x14a\x0B#W\x80c\xFA$\xF7C\x14a\x0BCW\x80c\xFA1Z\xA9\x14a\x0BgW\x80c\xFE+\xBE\xB2\x14a\x0B\x9AW`\0\x80\xFD[\x80c\xDA\xBD9m\x14a\n\x8DW\x80c\xEC^c\x08\x14a\n\xC0W\x80c\xEF\xF0\xF5\x92\x14a\n\xF3W`\0\x80\xFD[\x80c\xD5\xD4M\x80\x11a\0\xC6W\x80c\xD5\xD4M\x80\x14a\n\x1AW\x80c\xD6\xAE<\xD5\x14a\n:W\x80c\xD8\xCC\x1A<\x14a\nmW`\0\x80\xFD[\x80c\xC3\x95\xE1\xCA\x14a\teW\x80c\xC6\xF00\x8C\x14a\t\x85W\x80c\xCF\t\xE0\xD0\x14a\t\xF9W`\0\x80\xFD[\x80c\x8DE\n\x95\x11a\x01YW\x80c\xBB\xDC\x02\xDB\x11a\x013W\x80c\xBB\xDC\x02\xDB\x14a\x08\xB5W\x80c\xBC\xEF;U\x14a\x08\xF6W\x80c\xBD\x8D\xA9V\x14a\t\x18W\x80c\xC0\xD8\xBBt\x14a\t8W`\0\x80\xFD[\x80c\x8DE\n\x95\x14a\x07\xE2W\x80c\x99s^2\x14a\x07\xC0W\x80c\xA4E\xEC\xE6\x14a\x08\x15W`\0\x80\xFD[\x80c\x81)\xFC\x1C\x11a\x01\x8AW\x80c\x81)\xFC\x1C\x14a\x07\xA3W\x80c\x89\x80\xE0\xCC\x14a\x07\xABW\x80c\x8B\x85\x90+\x14a\x07\xC0W`\0\x80\xFD[\x80cp\x87*\xA5\x14a\x07fW\x80cxk\x84K\x14a\x07{W\x80c{\x0F\n\xDC\x14a\x07\x90W`\0\x80\xFD[\x80c>:\xC9\x12\x11a\x02\x7FW\x80cZ_\xA2\xD9\x11a\x02(W\x80c`\xE2td\x11a\x02\x02W\x80c`\xE2td\x14a\x06\xDEW\x80ccaPm\x14a\x06\xFEW\x80ckg\x16\xC0\x14a\x07 W\x80co\x03D\t\x14a\x07SW`\0\x80\xFD[\x80cZ_\xA2\xD9\x14a\x06vW\x80c\\\x0C\xBA3\x14a\x06\x96W\x80c`\x9D34\x14a\x06\xC9W`\0\x80\xFD[\x80cR\x9Dj\x8C\x11a\x02YW\x80cR\x9Dj\x8C\x14a\x05\xCAW\x80cT\xFDMP\x14a\x05\xF7W\x80cW\xDA\x95\x0E\x14a\x06FW`\0\x80\xFD[\x80c>:\xC9\x12\x14a\x05TW\x80c?\xC8\xCE\xF3\x14a\x05\x84W\x80cG'w\xC6\x14a\x05\xB7W`\0\x80\xFD[\x80c%\xFC*\xCE\x11a\x02\xECW\x80c0\xDB\xE5p\x11a\x02\xC6W\x80c0\xDB\xE5p\x14a\x04\xADW\x80c7\x8D\xD4\x8C\x14a\x04\xE5W\x80c7\xB1\xB2)\x14a\x04\xFFW\x80c:v\x84c\x14a\x05!W`\0\x80\xFD[\x80c%\xFC*\xCE\x14a\x04YW\x80c(\x10\xE1\xD6\x14a\x04xW\x80c*\xD6\x9A\xEB\x14a\x04\x8DW`\0\x80\xFD[\x80c \r.\xD2\x11a\x03\x1DW\x80c \r.\xD2\x14a\x03\xD1W\x80c\"*\xBFE\x14a\x03\xFFW\x80c%\x0Ei\xBD\x14a\x04?W`\0\x80\xFD[\x80c\x01\x93Q0\x14a\x03DW\x80c\x03\xC2\x92M\x14a\x03fW\x80c\x19\xEF\xFE\xB4\x14a\x03\x86W[`\0\x80\xFD[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x03da\x03_6`\x04aR\xC3V[a\x0B\xCAV[\0[4\x80\x15a\x03rW`\0\x80\xFD[Pa\x03da\x03\x816`\x04aS\x1EV[a\x0E\x89V[4\x80\x15a\x03\x92W`\0\x80\xFD[P`\0Ta\x03\xB3\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xDDW`\0\x80\xFD[P`\0Ta\x03\xF2\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x03\xC8\x91\x90aSwV[4\x80\x15a\x04\x0BW`\0\x80\xFD[Pa\x04/a\x04\x1A6`\x04aS\x9FV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xC8V[4\x80\x15a\x04KW`\0\x80\xFD[P`\nTa\x04/\x90`\xFF\x16\x81V[4\x80\x15a\x04eW`\0\x80\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x03\xC8V[4\x80\x15a\x04\x84W`\0\x80\xFD[Pa\x03\xF2a\x14\x11V[4\x80\x15a\x04\x99W`\0\x80\xFD[Pa\x04ja\x04\xA86`\x04aS\x1EV[a\x15\xE9V[4\x80\x15a\x04\xB9W`\0\x80\xFD[P`\x01Ta\x04\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xC8V[4\x80\x15a\x04\xF1W`\0\x80\xFD[P`\rTa\x03\xF2\x90`\xFF\x16\x81V[4\x80\x15a\x05\x0BW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x035``\x1Ca\x04\xCDV[4\x80\x15a\x05-W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xCDV[4\x80\x15a\x05`W`\0\x80\xFD[P`\0Ta\x04/\x90r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x05\x90W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xCDV[a\x03da\x05\xC56`\x04aS\xBCV[a\x16\x1FV[4\x80\x15a\x05\xD6W`\0\x80\xFD[Pa\x04ja\x05\xE56`\x04aS\x9FV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x06\x03W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F1.8.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03\xC8\x91\x90aTSV[4\x80\x15a\x06RW`\0\x80\xFD[P`\x08T`\tTa\x06a\x91\x90\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\xC8V[4\x80\x15a\x06\x82W`\0\x80\xFD[Pa\x04ja\x06\x916`\x04aTfV[a\x161V[4\x80\x15a\x06\xA2W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xCDV[4\x80\x15a\x06\xD5W`\0\x80\xFD[Pa\x069a\x16kV[4\x80\x15a\x06\xEAW`\0\x80\xFD[Pa\x03da\x06\xF96`\x04aS\x9FV[a\x16yV[4\x80\x15a\x07\nW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015a\x04jV[4\x80\x15a\x07,W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xB3V[a\x03da\x07a6`\x04aT\x8DV[a\x19\x91V[4\x80\x15a\x07rW`\0\x80\xFD[P`\tTa\x04jV[4\x80\x15a\x07\x87W`\0\x80\xFD[Pa\x03da#2V[a\x03da\x07\x9E6`\x04aS\xBCV[a'QV[a\x03da'^V[4\x80\x15a\x07\xB7W`\0\x80\xFD[P`\x02Ta\x04jV[4\x80\x15a\x07\xCCW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015a\x04jV[4\x80\x15a\x07\xEEW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\x08!W`\0\x80\xFD[Pa\x08wa\x0806`\x04aTfV[`\x07` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x82\x16\x91a\x01\0\x81\x04c\xFF\xFF\xFF\xFF\x16\x91e\x01\0\0\0\0\0\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q\x94\x15\x15\x85Rc\xFF\xFF\xFF\xFF\x90\x93\x16` \x85\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01a\x03\xC8V[4\x80\x15a\x08\xC1W`\0\x80\xFD[P`@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x03\xC8V[4\x80\x15a\t\x02W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015a\x04jV[4\x80\x15a\t$W`\0\x80\xFD[Pa\x03\xB3a\t36`\x04aTfV[a,\xFBV[4\x80\x15a\tDW`\0\x80\xFD[Pa\x04ja\tS6`\x04aS\x9FV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\tqW`\0\x80\xFD[Pa\x04ja\t\x806`\x04aT\xCEV[a.\x95V[4\x80\x15a\t\x91W`\0\x80\xFD[Pa\t\xA5a\t\xA06`\x04aTfV[a0VV[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x98\x16\x88R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16` \x89\x01R\x95\x90\x94\x16\x94\x86\x01\x94\x90\x94R`\x01`\x01`\x80\x1B\x03\x91\x82\x16``\x86\x01R`\x80\x85\x01R\x91\x82\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\xC8V[4\x80\x15a\n\x05W`\0\x80\xFD[P`\0Ta\x03\xB3\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\n&W`\0\x80\xFD[Pa\x04ja\n56`\x04aS\x9FV[a0\xCAV[4\x80\x15a\nFW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\nyW`\0\x80\xFD[Pa\x03da\n\x886`\x04aT\xF7V[a1\"V[4\x80\x15a\n\x99W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xB3V[4\x80\x15a\n\xCCW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\n\xFFW`\0\x80\xFD[Pa\x04/a\x0B\x0E6`\x04aTfV[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B/W`\0\x80\xFD[Pa\x03da\x0B>6`\x04aS\xBCV[a6~V[4\x80\x15a\x0BOW`\0\x80\xFD[Pa\x0BXa:dV[`@Qa\x03\xC8\x93\x92\x91\x90aU\x83V[4\x80\x15a\x0BsW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\x0B\xA6W`\0\x80\xFD[Pa\x04/a\x0B\xB56`\x04aTfV[`\x06` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0B\xE9Wa\x0B\xE9aS@V[\x14a\x0C\x07W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x0CZW`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cs6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015\x90V[\x90V[a\x0C\x8Aa\x0C\x856\x86\x90\x03\x86\x01\x86aU\xBEV[a:\xC1V[\x14a\x0C\xC1W`@Q\x7F\x9C\xC0\x0B[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82``\x015\x82\x82`@Qa\x0C\xD6\x92\x91\x90aV2V[`@Q\x80\x91\x03\x90 \x14a\r\x15W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r^a\rY\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\x1D\x92PPPV[a;\x8AV[\x90P`\0a\r\x85\x82`\x08\x81Q\x81\x10a\rxWa\rxaVBV[` \x02` \x01\x01Qa=@V[\x90P` \x81Q\x11\x15a\r\xC3W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x81\x01Q\x82Q\x90\x91\x03`\x03\x1B\x1C6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015\x81\x03a\x0E\x1AW`@Q\x7F\xB8\xED\x880\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UPP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UPPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0E\xA8Wa\x0E\xA8aS@V[\x14a\x0E\xC6W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a\x0E\xDBWa\x0E\xDBaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x90P`\0a\x0E\xF6\x84a,\xFBV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x10\x15a\x0F_W`@Q\x7F\xF2D\x0BS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x06` R`@\x90 T`\xFF\x16\x15a\x0F\xA8W`@Q\x7F\xF1\xA9E\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x05` R`@\x90 \x80T\x80\x15\x80\x15a\x0F\xC5WP\x85\x15\x15[\x15a\x10(W\x83Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0\x81\x15a\x0F\xEBW\x81a\x0F\xFAV[`\x01\x86\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x90Pa\x10\x06\x81\x87a=\xF4V[PPP`\0\x94\x85RPP`\x06` RPP`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0\x86\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x82\x04c\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x93\x81\x01\x93\x90\x93R`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01Ra\x10\xACW`\x01`\x01`\x80\x1B\x03`@\x82\x01R`\x01\x81R`\0\x86\x90\x03a\x10\xACW\x81\x95P[`\0\x86\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a\x10\xC4\x91\x90aVnV[\x90P`\0\x83\x82\x11a\x10\xD5W\x81a\x10\xD7V[\x83[` \x84\x01Q\x90\x91Pc\xFF\xFF\xFF\xFF\x16[\x81\x81\x10\x15a\x11\xF7W`\0\x86\x82\x81T\x81\x10a\x11\x02Wa\x11\x02aVBV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83R`\x06\x90\x91R`@\x90\x91 T\x90\x91P`\xFF\x16a\x11ZW`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x82\x81T\x81\x10a\x11oWa\x11oaVBV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T\x90\x91Pd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x11\xB6WP`\x04\x81\x01T`@\x87\x01Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11[\x15a\x11\xE2W`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x87\x01R`\x04\x81\x01T`\x01`\x01`\x80\x1B\x03\x16`@\x87\x01R[PP\x80\x80a\x11\xEF\x90aV\x86V[\x91PPa\x10\xE6V[Pc\xFF\xFF\xFF\xFF\x81\x81\x16` \x85\x81\x01\x91\x82R`\0\x8C\x81R`\x07\x90\x91R`@\x90\x81\x90 \x86Q\x81T\x93Q\x92\x88\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x94\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\x16\x17a\x01\0\x92\x90\x94\x16\x91\x82\x02\x93\x90\x93\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x82U``\x85\x01Q`\x01\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x84\x90\x03a\x14\x06W``\x83\x01Q`\0\x8A\x81R`\x06` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x89\x15\x80\x15a\x13CWP`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16[\x15a\x13\x9EW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x13^\x81\x8Aa=\xF4V[\x88T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x88Ua\x14\x04V[a\x13\xCB`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x13\xB6W\x81a\x13\xC5V[`\x01\x89\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x89a=\xF4V[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x83\x16\x02\x17\x88U[P[PPPPPPPPPV[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x142Wa\x142aS@V[\x14a\x14PW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80R`\x06` R\x7FT\xCD\xD3i\xE4\xE8\xA8Q^R\xCAr\xEC\x81l!\x01\x83\x1A\xD1\xF1\x8B\xF4A\x02\xED\x17\x14Y\xC9\xB4\xF8T`\xFF\x16a\x14\xB4W`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x02`\0\x81T\x81\x10a\x14\xD3Wa\x14\xD3aVBV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\x01W`\x01a\x15\x04V[`\x02[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x17\x83U\x92\x93P\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17`\x01`\x80\x1B\x83`\x02\x81\x11\x15a\x15\xA8Wa\x15\xA8aS@V[\x02\x17\x90U`\x02\x81\x11\x15a\x15\xBDWa\x15\xBDaS@V[`@Q\x7F^\x18o\t\xB9\xC94\x91\xF1N'~\xEA\x7F\xAA]\xE6\xA2\xD4\xBD\xA7Zy\xAFz6\x84\xFB\xFBB\xDA`\x90`\0\x90\xA2\x90V[`\x05` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x16\x05W`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[\x90P\x90V[a\x16,\x83\x83\x83`\x01a\x19\x91V[PPPV[`\0\x81\x81R`\x07` \x90\x81R`@\x80\x83 `\x05\x90\x92R\x82 \x80T\x82Ta\x16b\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aV\xA0V[\x95\x94PPPPPV[``a\x16\x1A`T` a>6V[a\x16\x81a#2V[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\x9CWa\x16\x9CaS@V[\x03a\x16\xC0WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x90 Ta\x17/V[`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\xD9Wa\x16\xD9aS@V[\x03a\x16\xFDWP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 Ta\x17/V[`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x18\x0CW`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x0C` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F~\xEE(\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c~\xEE(\x8D\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x04W=`\0\x80>=`\0\xFD[PPPPPPV[\x80`\0\x03a\x18FW`@Q\x7F\x17\xBF\xE5\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x0B` \x90\x81R`@\x80\x83 \x83\x90U`\x03\x90\x91R\x80\x82 \x91\x90\x91UQ\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\0W=`\0\x80>=`\0\xFD[PPPP`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19QW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19VV[``\x91P[PP\x90P\x80a\x16,W`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x19\xB0Wa\x19\xB0aS@V[\x14a\x19\xCEW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x84\x81T\x81\x10a\x19\xE3Wa\x19\xE3aVBV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x81\x16\x84R`\x01`\x01`\xA0\x1B\x03d\x01\0\0\0\0\x90\x91\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01\x81\x01T\x90\x93\x16\x90\x82\x01R`\x02\x82\x01T`\x01`\x01`\x80\x1B\x03\x90\x81\x16``\x83\x01R`\x03\x83\x01T`\x80\x83\x01\x81\x90R`\x04\x90\x93\x01T\x80\x82\x16`\xA0\x84\x01R`\x01`\x80\x1B\x90\x04\x16`\xC0\x82\x01R\x91P\x85\x14a\x1A\xA7W`@Q\x7F0\x14\x032\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0\x81\x01Q`\0\x83\x15`\x01`\x01`\x80\x1B\x03\x83\x16\x17`\x01\x1B\x90P`\0a\x1B<\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x86\x15\x80a\x1BwWPa\x1Bt\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02aVnV[\x81\x14[\x80\x15a\x1B\x81WP\x84\x15[\x15a\x1B\xB8W`@Q\x7F\xA4&7\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x80\x15a\x1B\xDEWP\x86\x15[\x15a\x1C\x15W`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\x1CoW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aVnV[\x81\x03a\x1C\xACWa\x1C\xAC\x86\x88\x85\x88a>jV[4a\x1C\xB6\x83a.\x95V[\x14a\x1C\xEDW`@Q\x7F\x86 \xAA\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\xF8\x88a,\xFBV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a\x1D`W`@Q\x7F3\x81\xD1\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1D\x8D`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aV\xA0V[\x83\x03a\x1E\xB1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x15\x91\x90aV\xB7V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ERW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ev\x91\x90aV\xD4V[a\x1E\xAA\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aV\xEDV[\x90Pa\x1FDV[a\x1E\xDC`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aV\xA0V[\x83\x03a\x1F\x17Wa\x1E\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aW\x19V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[a\x1Fx\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aWIV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\x93\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xDAWa\x1F\xD7\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aWIV[\x91P[`\0`@\x83\x90\x1BB\x17`\0\x8A\x81R`\x80\x87\x90\x1B`\x01`\x01`\x80\x1B\x03\x8D\x16\x17` R`@\x81 \x91\x92P\x90`\0\x81\x81R`\x04` R`@\x90 T\x90\x91P`\xFF\x16\x15a OW`@Q\x7F\x80I~;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x04`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x02`@Q\x80`\xE0\x01`@R\x80\x8Dc\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x014`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\x80\x1B\x03\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`\x05`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\x01`\x02\x80T\x90Pa\"F\x91\x90aV\xA0V[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x91\x01\x91\x90\x91U3\x82R`\x0B\x90R`@\x81 \x80T4\x92\x90a\"z\x90\x84\x90aVnV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\"\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\xF0W=`\0\x80>=`\0\xFD[PP`@Q3\x93P\x8D\x92P\x8E\x91P\x7F\x9B2Et\x0E\xC3\xB1U\t\x8AU\xBE\x84\x95zM\xA1>\xAF\x7F\x14\xA8\xBCoS\x12l\x0B\x93P\xF2\xBE\x90`\0\x90\xA4PPPPPPPPPPPPV[`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a#KWa#KaS@V[\x14\x80a#mWP`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a#kWa#kaS@V[\x14[\x15a#tWV[`\0`\rT`\xFF\x16`\x02\x81\x11\x15a#\x8DWa#\x8DaS@V[\x14a#\xC4W`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$F\x91\x90aWrV[\x15a$}W`@Q\x7F7\x9A~\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a$\xD9W`@Q\x7F\xC1\x05&\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x03\x14\xD2\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x03\x14\xD2\xB3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%}\x91\x90aWrV[\x90P\x80a%\xB6W`@Q\x7FHQ\xBD\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x17\xCF!\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x17\xCF!\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&0W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a&AWP`\x01[P`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xE6\x91\x90aWrV[\x90P\x80\x15a'\0W`\r\x80T`\xFF\x19\x16`\x01\x17\x90Ua'\x0EV[`\r\x80T`\xFF\x19\x16`\x02\x17\x90U[`\rT`@Q\x7F\x99\x08\xEA\xAC\x06E\xDF\x9D\x07\x04\xD0j\xDC\x9E\x073|\x95\x1D\xE2\xF0k_(6\x15\x1DH\xD5\xE4r/\x91a'E\x91`\xFF\x90\x91\x16\x90aSwV[`@Q\x80\x91\x03\x90\xA1PPV[a\x16,\x83\x83\x83`\0a\x19\x91V[`\0Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a'\xB0W`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD8>\xF2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(4\x91\x90aW\x8FV[\x90\x92P\x90P\x81a(pW`@Q\x7Fjk\xC3\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x01\x81\x90R`\x08\x82\x90U`\t\x81\x90U`z6\x14a(\xC7W`@Q\x7F\x98$\xBD\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x806`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015\x11a)%W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x80\x82R`\0` \x80\x84\x01\x82\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x805``\x90\x81\x1C\x87\x89\x01\x81\x81R`\x01`\x01`\x80\x1B\x034\x81\x81\x16\x94\x8B\x01\x94\x85R`\x14\x90\x95\x015`\x80\x8B\x01\x90\x81R`\x01`\xA0\x8C\x01\x81\x81RB\x84\x16`\xC0\x8E\x01\x90\x81R`\x02\x80T\x93\x84\x01\x81U\x8CR\x9CQ\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE`\x05\x90\x93\x02\x92\x83\x01\x80T\x9AQ\x91\x90\x9D\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x9A\x16\x99\x90\x99\x17d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x9A\x8B\x16\x02\x17\x90\x9BU\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCF\x84\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x98\x16\x17\x90\x96U\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD0\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD1\x85\x01U\x95Q\x96Q\x96\x81\x16`\x01`\x80\x1B\x97\x90\x91\x16\x96\x90\x96\x02\x95\x90\x95\x17\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD2\x90\x91\x01U\x81Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U\x91\x81R`\x0B\x90\x91R\x91\x82 \x80T\x91\x92\x90\x91a+f\x90\x84\x90aVnV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a+\xC8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a+\xDCW=`\0\x80>=`\0\xFD[PP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPP`@\x80Q\x7F<\x9F9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91c<\x9F9|\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xDF\x91\x90aW\xB3V[`\n\x80T`\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x92\x90\x92\x14\x17\x90UPPV[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a-\x1CWa-\x1CaS@V[\x14a-:W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a-OWa-OaVBV[`\0\x91\x82R` \x82 `\x05\x90\x91\x02\x01\x80T\x90\x92Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x14a-\xB5W\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10a-\x8DWa-\x8DaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01`\x04\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90P[`\x04\x82\x01T`\0\x90a-\xE0\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a-\xF4\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaV\xA0V[a.\na-\xD3\x84`\x01`\x01`\x80\x1B\x03\x16`@\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a.\x1E\x91\x90aVnV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a.kW\x80a\x16bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`\0\x80a/\x12\x83`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a/qW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d.\x90\xED\xD0\0b\x06\x1A\x80c\x11\xE1\xA3\0`\0a/\x8C\x83\x83aW\xEFV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0a/\xC3\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aX\x03V[\x90P`\0a/\xE1a/\xDCg\r\xE0\xB6\xB3\xA7d\0\0\x86aX\x03V[a?\xF9V[\x90P`\0a/\xEF\x84\x84aBKV[\x90P`\0a/\xFD\x83\x83aB\x9AV[\x90P`\0a0\n\x82aB\xC8V[\x90P`\0a0)\x82a0$g\r\xE0\xB6\xB3\xA7d\0\0\x8FaX\x03V[aD\xB0V[\x90P`\0a07\x8B\x83aB\x9AV[\x90Pa0C\x81\x8DaX\x03V[\x9F\x9EPPPPPPPPPPPPPPPV[`\x02\x81\x81T\x81\x10a0fW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01Tc\xFF\xFF\xFF\xFF\x84\x16\x95Pd\x01\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x92\x16\x92`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x87V[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a0\xE5Wa0\xE5aS@V[\x03a1\x06WP`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0B` R`@\x90 T\x90V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a1AWa1AaS@V[\x14a1_W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x87\x81T\x81\x10a1tWa1taVBV[`\0\x91\x82R` \x82 `\x05\x91\x90\x91\x02\x01`\x04\x81\x01T\x90\x92P`\x01`\x01`\x80\x1B\x03\x16\x90\x87\x15\x82\x17`\x01\x1B\x90Pa1\xCA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aVnV[a2D\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x14a2~W`@Q\x7F_S\xDD\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x89\x15a3ZWa2\xD1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aV\xA0V[`\x01\x90\x1Ba2\xE7\x84`\x01`\x01`\x80\x1B\x03\x16aD\xE1V[`\x01`\x01`\x80\x1B\x03\x16a2\xFA\x91\x90aX\"V[\x15a3.Wa3%a3\x16`\x01`\x01`\x01`\x80\x1B\x03\x87\x16aX6V[\x86Tc\xFF\xFF\xFF\xFF\x16`\0aEgV[`\x03\x01Ta3PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x91P\x84\x90Pa3{V[`\x03\x85\x01T\x91Pa3xa3\x16`\x01`\x01`\x80\x1B\x03\x86\x16`\x01aXVV[\x90P[`\x08\x82\x90\x1B`\x08\x8A\x8A`@Qa3\x92\x92\x91\x90aV2V[`@Q\x80\x91\x03\x90 \x90\x1B\x14a3\xD3W`@Q\x7FieP\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a3\xDE\x8CaF0V[\x90P`\0a3\xED\x83`\x03\x01T\x90V[`@Q\x7F\xE1L\xED2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE1L\xED2\x90a4Z\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8A\x90`\x04\x01aX\xC1V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a4yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\x9D\x91\x90aV\xD4V[`\x04\x85\x01T\x91\x14\x91P`\0\x90`\x02\x90a5&\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a5\xA0\x89`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a5\xAA\x91\x90aX\xFBV[a5\xB4\x91\x90aY\x1EV[`\xFF\x16\x15\x90P\x81\x15\x15\x81\x03a5\xF5W`@Q\x7F\xFBN@\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a6?W`@Q\x7F\x90q\xE6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x163d\x01\0\0\0\0\x02\x17\x90\x95UPPPPPPPPPPPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a6\x9DWa6\x9DaS@V[\x14a6\xBBW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80a6\xCA\x86aF_V[\x93P\x93P\x93P\x93P`\0a6\xE0\x85\x85\x85\x85aI\xB8V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7f\x91\x90aV\xB7V[\x90P`\x01\x89\x03a83W`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84a7\x976`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R`D\x82\x01R` `d\x82\x01R`\x84\x81\x01\x8A\x90R`\xA4\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a8\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8-\x91\x90aV\xD4V[Pa\x14\x06V[`\x02\x89\x03a8RW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x89a7\x97V[`\x03\x89\x03a8qW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x87a7\x97V[`\x04\x89\x03a9\xA6W`\0a8\xAE`\x01`\x01`\x80\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aJWV[`\tTa8\xBB\x91\x90aVnV[a8\xC6\x90`\x01aVnV[\x90P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015\x81\x10a8\xF3W6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015a8\xF5V[\x80[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16cR\xF0\xF3\xAD\x8B\x85`@Q`\xE0\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`\xC0\x84\x90\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x8B\x90R`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a9{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x9F\x91\x90aV\xD4V[PPa\x14\x06V[`\x05\x89\x03a:2W`@Q\x7FR\xF0\xF3\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cR\xF0\xF3\xAD\x90`\xA4\x01a7\xEAV[`@Q\x7F\xFF\x13~e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015``a:\x9Fa\x16kV[\x90P\x90\x91\x92V[`\0\x81\x83\x10\x15a:\xB6W\x81a:\xB8V[\x82[\x90P[\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a;\0\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03a;lW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0a;\x9A\x85aJ\xECV[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15a;\xB5Wa;\xB5aS@V[\x14a;\xECW`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa;\xF8\x83\x85aVnV[\x14a</W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a<FW\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15a=4W`\0\x80a<\xB9`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01Qa<\x9D\x91\x90aV\xA0V[\x81R` \x01\x85\x8C` \x01Qa<\xB2\x91\x90aVnV[\x90RaJ\xECV[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a<\xD5\x91\x90aVnV[\x81R` \x01\x84\x8B` \x01Qa<\xEA\x91\x90aVnV[\x81RP\x88\x85\x81Q\x81\x10a<\xFFWa<\xFFaVBV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra=\x15`\x01\x85aVnV[\x93Pa=!\x81\x83aVnV[a=+\x90\x84aVnV[\x92PPPa<sV[P\x84RP\x91\x93\x92PPPV[```\0\x80`\0a=P\x85aJ\xECV[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15a=kWa=kaS@V[\x14a=\xA2W`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\xAC\x82\x84aVnV[\x85Q\x14a=\xE5W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16b\x85` \x01Q\x84\x84aO\x8AV[`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x91\x90a>-\x90\x84\x90aVnV[\x90\x91UPPPPV[`@Q\x81\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x82\x84\x82\x01` \x84\x017\x82` \x83\x01\x01`\0\x81R` \x81\x01`@RPP\x92\x91PPV[`\0a>\x80`\x01`\x01`\x80\x1B\x03\x84\x16`\x01aXVV[\x90P`\0a>\x90\x82\x86`\x01aEgV[\x90P`\0\x86\x90\x1A\x83\x80a?ZWPa>\xC9`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aX\"V[`\x04\x83\x01T`\x02\x90a?K\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a?U\x91\x90aY\x1EV[`\xFF\x16\x14[\x15a?\xB2W`\xFF\x81\x16`\x01\x14\x80a?tWP`\xFF\x81\x16`\x02\x14[a?\xADW`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a)\x1CV[a?\xF0V[`\xFF\x81\x16\x15a?\xF0W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a)\x1CV[PPPPPPPV[`\x01`\x01`\x80\x1B\x03\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17`\0\x82\x13a@OWc\x16\x15\xE68`\0R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[`\0x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x83\x11g\r\xE0\xB6\xB3\xA7d\0\0\x02\x15\x82\x02aB\x88Wc|_H}`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x02\x15aB\xB8Wc\xBA\xC6^[`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13aB\xF6W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12aC\x14Wc\xA3{\xFE\xC9`\0R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a:\xB8g\r\xE0\xB6\xB3\xA7d\0\0\x83aD\xC8\x86a?\xF9V[aD\xD2\x91\x90aY@V[aD\xDC\x91\x90aY\xFCV[aB\xC8V[`\0\x80aEU\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01`\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x80\x82aE\xA7WaE\xA2`\x01`\x01`\x80\x1B\x03\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aP\x1FV[aE\xB9V[aE\xB9\x85`\x01`\x01`\x80\x1B\x03\x16aQ^V[\x90P`\x02\x84\x81T\x81\x10aE\xCEWaE\xCEaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91P[`\x04\x82\x01T`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x91\x16\x14aF(W\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10aF\x13WaF\x13aVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91PaE\xDFV[P\x93\x92PPPV[`\0\x80`\0\x80`\0aFA\x86aF_V[\x93P\x93P\x93P\x93PaFU\x84\x84\x84\x84aI\xB8V[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x85\x90P`\0`\x02\x82\x81T\x81\x10aF\x7FWaF\x7FaVBV[`\0\x91\x82R` \x90\x91 `\x04`\x05\x90\x92\x02\x01\x90\x81\x01T\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aG4\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aGnW`@Q\x7F\xB3K\\\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[`\x04\x83\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aH\x13\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x92P\x82\x11\x15aH\x88W\x82Tc\xFF\xFF\xFF\xFF\x16aHR\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aVnV[\x83\x03aH\\W\x83\x91P[`\x02\x81\x81T\x81\x10aHoWaHoaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x93P\x80\x94PPaGrV[`\x04\x81\x81\x01T\x90\x84\x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16`\0\x81`\x01`\x01`\x80\x1B\x03\x16aH\xCDaH\xC1\x85`\x01`\x01`\x80\x1B\x03\x16`\x01\x1C\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x14\x90P\x80\x15aIfW`\0aH\xF3\x83`\x01`\x01`\x80\x1B\x03\x16aD\xE1V[`\x01`\x01`\x80\x1B\x03\x16\x11\x15aICW`\0aI#aI\x1B`\x01`\x01`\x01`\x80\x1B\x03\x86\x16aX6V[\x89`\x01aEgV[`\x03\x81\x01T`\x04\x90\x91\x01T\x90\x9CP`\x01`\x01`\x80\x1B\x03\x16\x9APaII\x90PV[`\x08T\x9AP[`\x03\x86\x01T`\x04\x87\x01T\x90\x99P`\x01`\x01`\x80\x1B\x03\x16\x97PaI\xAAV[`\0aI\x7FaI\x1B`\x01`\x01`\x80\x1B\x03\x85\x16`\x01aXVV[`\x03\x80\x89\x01T`\x04\x80\x8B\x01T\x92\x84\x01T\x93\x01T\x90\x9EP`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x9DP\x91\x9BP\x16\x98PP[PPPPPPP\x91\x93P\x91\x93V[`\0`\x01`\x01`\x80\x1B\x03\x84\x16\x15aJ\x13W`@\x80Q` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R\x90\x83\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x16bV[\x82\x82`@Q` \x01aJ8\x92\x91\x90\x91\x82R`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95\x94PPPPPV[`\0\x80aJ\xCB\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[`\0\x80`\0\x83`\0\x01Q`\0\x03aK/W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11aKTW`\0`\x01`\0\x94P\x94P\x94PPPaO\x83V[`\xB7\x81\x11aLjW`\0aKi`\x80\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aK\xA8W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15aL WP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15aLWW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92PaO\x83\x91PPV[`\xBF\x81\x11aM\xC8W`\0aL\x7F`\xB7\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aL\xBEW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aM W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aMhW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aMr\x81\x84aVnV[\x89Q\x11aM\xABW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aM\xB6\x83`\x01aVnV[\x97P\x95P`\0\x94PaO\x83\x93PPPPV[`\xF7\x81\x11aN-W`\0aM\xDD`\xC0\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aN\x1CW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92PaO\x83\x91PPV[`\0aN:`\xF7\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aNyW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aN\xDBW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aO#W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aO-\x81\x84aVnV[\x89Q\x11aOfW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aOq\x83`\x01aVnV[\x97P\x95P`\x01\x94PaO\x83\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\xA5WaO\xA5aU\xA8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aO\xCFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15aP\x18W`\0aO\xE4\x84\x86aVnV[\x90P` \x82\x01`\0[\x84\x81\x10\x15aP\x05W\x82\x81\x01Q\x82\x82\x01R` \x01aO\xEDV[\x84\x81\x11\x15aP\x14W`\0\x85\x83\x01R[PPP[\x93\x92PPPV[`\0\x81aP\x9C\x84`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aP\xB2Wc\xB3K\\\"`\0R`\x04`\x1C\xFD[aP\xBB\x83aQ^V[\x90P\x81aQ8\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11a:\xBBWa:\xB8aQN\x83`\x01aVnV[`\x01`\x01`\x80\x1B\x03\x83\x16\x90aQ\xEAV[`\0\x81\x19`\x01\x83\x01\x16\x81aQ\xD9\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`\0\x80aR^\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x80\x82\x1B\x03\x85\x82\x1B\x17\x92PPP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aR\x8CW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\xA4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aR\xBCW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15aR\xD9W`\0\x80\xFD[`\x80\x81\x12\x15aR\xE7W`\0\x80\xFD[P\x83\x92P`\x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\x05W`\0\x80\xFD[aS\x11\x86\x82\x87\x01aRzV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15aS1W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aStWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01aS\x84\x83aSVV[\x91\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aStW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aS\xB1W`\0\x80\xFD[\x815aP\x18\x81aS\x8AV[`\0\x80`\0``\x84\x86\x03\x12\x15aS\xD1W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aT\x0EW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aS\xF2V[\x81\x81\x11\x15aT W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a:\xB8` \x83\x01\x84aS\xE8V[`\0` \x82\x84\x03\x12\x15aTxW`\0\x80\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14aStW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\xA3W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aT\xC3\x81aT\x7FV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aT\xE0W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aP\x18W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aU\x10W`\0\x80\xFD[\x865\x95P` \x87\x015aU\"\x81aT\x7FV[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aU?W`\0\x80\xFD[aUK\x8A\x83\x8B\x01aRzV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aUdW`\0\x80\xFD[PaUq\x89\x82\x8A\x01aRzV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x16b``\x83\x01\x84aS\xE8V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15aU\xD0W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aV\x01WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aV\x81WaV\x81aVXV[P\x01\x90V[`\0`\0\x19\x82\x03aV\x99WaV\x99aVXV[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aV\xB2WaV\xB2aVXV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aV\xC9W`\0\x80\xFD[\x81QaP\x18\x81aS\x8AV[`\0` \x82\x84\x03\x12\x15aV\xE6W`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aW\x10WaW\x10aVXV[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aW@WaW@aVXV[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aWjWaWjaVXV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aW\x84W`\0\x80\xFD[\x81QaP\x18\x81aT\x7FV[`\0\x80`@\x83\x85\x03\x12\x15aW\xA2W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15aW\xC5W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14aP\x18W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aW\xFEWaW\xFEaW\xD9V[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aX\x1DWaX\x1DaVXV[P\x02\x90V[`\0\x82aX1WaX1aW\xD9V[P\x06\x90V[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aWjWaWjaVXV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aW\x10WaW\x10aVXV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0aX\xD5``\x83\x01\x87\x89aXxV[\x82\x81\x03` \x84\x01RaX\xE8\x81\x86\x88aXxV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15aY\x15WaY\x15aVXV[\x90\x03\x93\x92PPPV[`\0`\xFF\x83\x16\x80aY1WaY1aW\xD9V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15aY\x81WaY\x81aVXV[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15aY\xBCWaY\xBCaVXV[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15aY\xD8WaY\xD8aVXV[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15aY\xEEWaY\xEEaVXV[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82aZ\x0BWaZ\x0BaW\xD9V[`\0\x19\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aZAWaZAaVXV[P\x05\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061033f5760003560e01c806370872aa5116101b0578063c395e1ca116100ec578063dabd396d11610095578063f8f43ff61161006f578063f8f43ff614610b23578063fa24f74314610b43578063fa315aa914610b67578063fe2bbeb214610b9a57600080fd5b8063dabd396d14610a8d578063ec5e630814610ac0578063eff0f59214610af357600080fd5b8063d5d44d80116100c6578063d5d44d8014610a1a578063d6ae3cd514610a3a578063d8cc1a3c14610a6d57600080fd5b8063c395e1ca14610965578063c6f0308c14610985578063cf09e0d0146109f957600080fd5b80638d450a9511610159578063bbdc02db11610133578063bbdc02db146108b5578063bcef3b55146108f6578063bd8da95614610918578063c0d8bb741461093857600080fd5b80638d450a95146107e257806399735e32146107c0578063a445ece61461081557600080fd5b80638129fc1c1161018a5780638129fc1c146107a35780638980e0cc146107ab5780638b85902b146107c057600080fd5b806370872aa514610766578063786b844b1461077b5780637b0f0adc1461079057600080fd5b80633e3ac9121161027f5780635a5fa2d91161022857806360e274641161020257806360e27464146106de5780636361506d146106fe5780636b6716c0146107205780636f0344091461075357600080fd5b80635a5fa2d9146106765780635c0cba3314610696578063609d3334146106c957600080fd5b8063529d6a8c11610259578063529d6a8c146105ca57806354fd4d50146105f757806357da950e1461064657600080fd5b80633e3ac912146105545780633fc8cef314610584578063472777c6146105b757600080fd5b806325fc2ace116102ec57806330dbe570116102c657806330dbe570146104ad578063378dd48c146104e557806337b1b229146104ff5780633a7684631461052157600080fd5b806325fc2ace146104595780632810e1d6146104785780632ad69aeb1461048d57600080fd5b8063200d2ed21161031d578063200d2ed2146103d1578063222abf45146103ff578063250e69bd1461043f57600080fd5b8063019351301461034457806303c2924d1461036657806319effeb414610386575b600080fd5b34801561035057600080fd5b5061036461035f3660046152c3565b610bca565b005b34801561037257600080fd5b5061036461038136600461531e565b610e89565b34801561039257600080fd5b506000546103b39068010000000000000000900467ffffffffffffffff1681565b60405167ffffffffffffffff90911681526020015b60405180910390f35b3480156103dd57600080fd5b506000546103f290600160801b900460ff1681565b6040516103c89190615377565b34801561040b57600080fd5b5061042f61041a36600461539f565b600c6020526000908152604090205460ff1681565b60405190151581526020016103c8565b34801561044b57600080fd5b50600a5461042f9060ff1681565b34801561046557600080fd5b506008545b6040519081526020016103c8565b34801561048457600080fd5b506103f2611411565b34801561049957600080fd5b5061046a6104a836600461531e565b6115e9565b3480156104b957600080fd5b506001546104cd906001600160a01b031681565b6040516001600160a01b0390911681526020016103c8565b3480156104f157600080fd5b50600d546103f29060ff1681565b34801561050b57600080fd5b503660011981013560f01c90033560601c6104cd565b34801561052d57600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006104cd565b34801561056057600080fd5b5060005461042f907201000000000000000000000000000000000000900460ff1681565b34801561059057600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006104cd565b6103646105c53660046153bc565b61161f565b3480156105d657600080fd5b5061046a6105e536600461539f565b60036020526000908152604090205481565b34801561060357600080fd5b5060408051808201909152600581527f312e382e3000000000000000000000000000000000000000000000000000000060208201525b6040516103c89190615453565b34801561065257600080fd5b50600854600954610661919082565b604080519283526020830191909152016103c8565b34801561068257600080fd5b5061046a610691366004615466565b611631565b3480156106a257600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006104cd565b3480156106d557600080fd5b5061063961166b565b3480156106ea57600080fd5b506103646106f936600461539f565b611679565b34801561070a57600080fd5b503660011981013560f01c90036034013561046a565b34801561072c57600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103b3565b61036461076136600461548d565b611991565b34801561077257600080fd5b5060095461046a565b34801561078757600080fd5b50610364612332565b61036461079e3660046153bc565b612751565b61036461275e565b3480156107b757600080fd5b5060025461046a565b3480156107cc57600080fd5b503660011981013560f01c90036054013561046a565b3480156107ee57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b34801561082157600080fd5b50610877610830366004615466565b6007602052600090815260409020805460019091015460ff821691610100810463ffffffff1691650100000000009091046001600160801b0316906001600160a01b031684565b60408051941515855263ffffffff90931660208501526001600160801b03909116918301919091526001600160a01b031660608201526080016103c8565b3480156108c157600080fd5b5060405163ffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016103c8565b34801561090257600080fd5b503660011981013560f01c90036014013561046a565b34801561092457600080fd5b506103b3610933366004615466565b612cfb565b34801561094457600080fd5b5061046a61095336600461539f565b600b6020526000908152604090205481565b34801561097157600080fd5b5061046a6109803660046154ce565b612e95565b34801561099157600080fd5b506109a56109a0366004615466565b613056565b6040805163ffffffff90981688526001600160a01b03968716602089015295909416948601949094526001600160801b039182166060860152608085015291821660a08401521660c082015260e0016103c8565b348015610a0557600080fd5b506000546103b39067ffffffffffffffff1681565b348015610a2657600080fd5b5061046a610a3536600461539f565b6130ca565b348015610a4657600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b348015610a7957600080fd5b50610364610a883660046154f7565b613122565b348015610a9957600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103b3565b348015610acc57600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b348015610aff57600080fd5b5061042f610b0e366004615466565b60046020526000908152604090205460ff1681565b348015610b2f57600080fd5b50610364610b3e3660046153bc565b61367e565b348015610b4f57600080fd5b50610b58613a64565b6040516103c893929190615583565b348015610b7357600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061046a565b348015610ba657600080fd5b5061042f610bb5366004615466565b60066020526000908152604090205460ff1681565b60008054600160801b900460ff166002811115610be957610be9615340565b14610c075760405163067fe19560e41b815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff1615610c5a576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610c733660011981013560f01c90036014013590565b90565b610c8a610c85368690038601866155be565b613ac1565b14610cc1576040517f9cc00b5b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b82606001358282604051610cd6929190615632565b604051809103902014610d15576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610d5e610d5984848080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250613b1d92505050565b613b8a565b90506000610d8582600881518110610d7857610d78615642565b6020026020010151613d40565b9050602081511115610dc3576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602081810151825190910360031b1c3660011981013560f01c9003605401358103610e1a576040517fb8ed883000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050600180547fffffffffffffffffffffffff000000000000000000000000000000000000000016331790555050600080547fffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffff1672010000000000000000000000000000000000001790555050565b60008054600160801b900460ff166002811115610ea857610ea8615340565b14610ec65760405163067fe19560e41b815260040160405180910390fd5b600060028381548110610edb57610edb615642565b906000526020600020906005020190506000610ef684612cfb565b905067ffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081169082161015610f5f576040517ff2440b5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008481526006602052604090205460ff1615610fa8576040517ff1a9458100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000848152600560205260409020805480158015610fc557508515155b1561102857835464010000000090046001600160a01b031660008115610feb5781610ffa565b60018601546001600160a01b03165b90506110068187613df4565b505050600094855250506006602052505060409020805460ff19166001179055565b6000868152600760209081526040918290208251608081018452815460ff81161515808352610100820463ffffffff16948301949094526501000000000090046001600160801b031693810193909352600101546001600160a01b031660608301526110ac576001600160801b0360408201526001815260008690036110ac578195505b600086826020015163ffffffff166110c4919061566e565b905060008382116110d557816110d7565b835b602084015190915063ffffffff165b818110156111f757600086828154811061110257611102615642565b6000918252602080832090910154808352600690915260409091205490915060ff1661115a576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006002828154811061116f5761116f615642565b60009182526020909120600590910201805490915064010000000090046001600160a01b03161580156111b65750600481015460408701516001600160801b039182169116115b156111e25760018101546001600160a01b0316606087015260048101546001600160801b031660408701525b505080806111ef90615686565b9150506110e6565b5063ffffffff818116602085810191825260008c81526007909152604090819020865181549351928801517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009094169015157fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000ff161761010092909416918202939093177fffffffffffffffffffffff00000000000000000000000000000000ffffffffff16650100000000006001600160801b03909316929092029190911782556060850151600190920180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b039093169290921790915584900361140657606083015160008a8152600660205260409020805460ff191660011790558915801561134357506000547201000000000000000000000000000000000000900460ff165b1561139e576001546001600160a01b031661135e818a613df4565b88546001600160a01b03909116640100000000027fffffffffffffffff0000000000000000000000000000000000000000ffffffff909116178855611404565b6113cb6001600160a01b038216156113b657816113c5565b60018901546001600160a01b03165b89613df4565b87547fffffffffffffffff0000000000000000000000000000000000000000ffffffff166401000000006001600160a01b038316021788555b505b505050505050505050565b600080600054600160801b900460ff16600281111561143257611432615340565b146114505760405163067fe19560e41b815260040160405180910390fd5b6000805260066020527f54cdd369e4e8a8515e52ca72ec816c2101831ad1f18bf44102ed171459c9b4f85460ff166114b4576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001600160a01b031660026000815481106114d3576114d3615642565b600091825260209091206005909102015464010000000090046001600160a01b031614611501576001611504565b60025b6000805467ffffffffffffffff421668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff82168117835592935083927fffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffff167fffffffffffffffffffffffffffffff000000000000000000ffffffffffffffff90911617600160801b8360028111156115a8576115a8615340565b0217905560028111156115bd576115bd615340565b6040517f5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da6090600090a290565b6005602052816000526040600020818154811061160557600080fd5b90600052602060002001600091509150505481565b905090565b61162c8383836001611991565b505050565b6000818152600760209081526040808320600590925282208054825461166290610100900463ffffffff16826156a0565b95945050505050565b606061161a60546020613e36565b611681612332565b60006002600d5460ff16600281111561169c5761169c615340565b036116c057506001600160a01b0381166000908152600b602052604090205461172f565b6001600d5460ff1660028111156116d9576116d9615340565b036116fd57506001600160a01b03811660009081526003602052604090205461172f565b6040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0382166000908152600c602052604090205460ff1661180c576001600160a01b038281166000818152600c602052604090819020805460ff19166001179055517f7eee288d0000000000000000000000000000000000000000000000000000000081526004810191909152602481018390527f000000000000000000000000000000000000000000000000000000000000000090911690637eee288d90604401600060405180830381600087803b1580156117f057600080fd5b505af1158015611804573d6000803e3d6000fd5b505050505050565b80600003611846576040517f17bfe5f700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b038281166000818152600b60209081526040808320839055600390915280822091909155517ff3fef3a30000000000000000000000000000000000000000000000000000000081526004810191909152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063f3fef3a390604401600060405180830381600087803b1580156118ec57600080fd5b505af1158015611900573d6000803e3d6000fd5b505050506000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114611951576040519150601f19603f3d011682016040523d82523d6000602084013e611956565b606091505b505090508061162c576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008054600160801b900460ff1660028111156119b0576119b0615340565b146119ce5760405163067fe19560e41b815260040160405180910390fd5b6000600284815481106119e3576119e3615642565b60009182526020918290206040805160e0810182526005909302909101805463ffffffff811684526001600160a01b0364010000000090910481169484019490945260018101549093169082015260028201546001600160801b03908116606083015260038301546080830181905260049093015480821660a0840152600160801b90041660c082015291508514611aa7576040517f3014033200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60a0810151600083156001600160801b0383161760011b90506000611b3c826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050861580611b775750611b747f0000000000000000000000000000000000000000000000000000000000000000600261566e565b81145b8015611b81575084155b15611bb8576040517fa42637bc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff168015611bde575086155b15611c15576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000811115611c6f576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611c9a7f0000000000000000000000000000000000000000000000000000000000000000600161566e565b8103611cac57611cac86888588613e6a565b34611cb683612e95565b14611ced576040517f8620aa1900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611cf888612cfb565b905067ffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000811690821603611d60576040517f3381d11400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611d8d60017f00000000000000000000000000000000000000000000000000000000000000006156a0565b8303611eb1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015611df1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e1591906156b7565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015611e52573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e7691906156d4565b611eaa907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166156ed565b9050611f44565b611edc60017f00000000000000000000000000000000000000000000000000000000000000006156a0565b8303611f1757611eaa7f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615719565b507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff165b611f78817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615749565b67ffffffffffffffff16611f938367ffffffffffffffff1690565b67ffffffffffffffff161115611fda57611fd7817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615749565b91505b6000604083901b421760008a8152608087901b6001600160801b038d1617602052604081209192509060008181526004602052604090205490915060ff161561204f576040517f80497e3b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60016004600083815260200190815260200160002060006101000a81548160ff02191690831515021790555060026040518060e001604052808d63ffffffff16815260200160006001600160a01b03168152602001336001600160a01b03168152602001346001600160801b031681526020018c8152602001886001600160801b03168152602001846001600160801b0316815250908060018154018082558091505060019003906000526020600020906005020160009091909190915060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a8154816001600160a01b0302191690836001600160a01b0316021790555060408201518160010160006101000a8154816001600160a01b0302191690836001600160a01b0316021790555060608201518160020160006101000a8154816001600160801b0302191690836001600160801b031602179055506080820151816003015560a08201518160040160006101000a8154816001600160801b0302191690836001600160801b0316021790555060c08201518160040160106101000a8154816001600160801b0302191690836001600160801b031602179055505050600560008c8152602001908152602001600020600160028054905061224691906156a0565b81546001810183556000928352602080842090910191909155338252600b905260408120805434929061227a90849061566e565b925050819055507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156122dc57600080fd5b505af11580156122f0573d6000803e3d6000fd5b50506040513393508d92508e91507f9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be90600090a4505050505050505050505050565b6002600d5460ff16600281111561234b5761234b615340565b148061236d57506001600d5460ff16600281111561236b5761236b615340565b145b1561237457565b6000600d5460ff16600281111561238d5761238d615340565b146123c4576040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316635c975abb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612422573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124469190615772565b1561247d576040517f379a7ed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60005468010000000000000000900467ffffffffffffffff1667ffffffffffffffff166000036124d9576040517fc105260a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f0314d2b30000000000000000000000000000000000000000000000000000000081523060048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690630314d2b390602401602060405180830381865afa158015612559573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061257d9190615772565b9050806125b6576040517f4851bd9b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f17cf21a90000000000000000000000000000000000000000000000000000000081523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906317cf21a990602401600060405180830381600087803b15801561263057600080fd5b505af1925050508015612641575060015b506040517f496b9c160000000000000000000000000000000000000000000000000000000081523060048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063496b9c1690602401602060405180830381865afa1580156126c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126e69190615772565b9050801561270057600d805460ff1916600117905561270e565b600d805460ff191660021790555b600d546040517f9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f916127459160ff90911690615377565b60405180910390a15050565b61162c8383836000611991565b60005471010000000000000000000000000000000000900460ff16156127b0576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d83ef2676040518163ffffffff1660e01b81526004016040805180830381865afa158015612810573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612834919061578f565b909250905081612870576040517f6a6bc3b200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820190915282815260200181905260088290556009819055607a36146128c7576040517f9824bdab00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b803660011981013560f01c90036054013511612925576040517ff40239db0000000000000000000000000000000000000000000000000000000081523660011981013560f01c90036014013560048201526024015b60405180910390fd5b6040805160e08101825263ffffffff808252600060208084018281523660011981013560f01c90038035606090811c8789018181526001600160801b0334818116948b0194855260149095013560808b01908152600160a08c0181815242841660c08e019081526002805493840181558c529c517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace600590930292830180549a5191909d167fffffffffffffffff000000000000000000000000000000000000000000000000909a16999099176401000000006001600160a01b039a8b160217909b5592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5acf840180547fffffffffffffffffffffffff000000000000000000000000000000000000000016919098161790965592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad0820180547fffffffffffffffffffffffffffffffff000000000000000000000000000000001691851691909117905593517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad185015595519651968116600160801b9790911696909602959095177f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad2909101558154710100000000000000000000000000000000007fffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffff909116178255918152600b909152918220805491929091612b6690849061566e565b925050819055507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b158015612bc857600080fd5b505af1158015612bdc573d6000803e3d6000fd5b5050600080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000164267ffffffffffffffff161790555050604080517f3c9f397c00000000000000000000000000000000000000000000000000000000815290517f000000000000000000000000000000000000000000000000000000000000000063ffffffff1692507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691633c9f397c9160048083019260209291908290030181865afa158015612cbb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612cdf91906157b3565b600a805460ff191663ffffffff92909216929092141790555050565b600080600054600160801b900460ff166002811115612d1c57612d1c615340565b14612d3a5760405163067fe19560e41b815260040160405180910390fd5b600060028381548110612d4f57612d4f615642565b600091825260208220600590910201805490925063ffffffff90811614612db557815460028054909163ffffffff16908110612d8d57612d8d615642565b906000526020600020906005020160040160109054906101000a90046001600160801b031690505b6004820154600090612de090600160801b900467ffffffffffffffff165b67ffffffffffffffff1690565b612df49067ffffffffffffffff16426156a0565b612e0a612dd3846001600160801b031660401c90565b67ffffffffffffffff16612e1e919061566e565b905067ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff1611612e6b5780611662565b7f000000000000000000000000000000000000000000000000000000000000000095945050505050565b600080612f12836001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690507f0000000000000000000000000000000000000000000000000000000000000000811115612f71576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b642e90edd00062061a806311e1a3006000612f8c83836157ef565b9050670de0b6b3a76400006000612fc3827f0000000000000000000000000000000000000000000000000000000000000000615803565b90506000612fe1612fdc670de0b6b3a764000086615803565b613ff9565b90506000612fef848461424b565b90506000612ffd838361429a565b9050600061300a826142c8565b9050600061302982613024670de0b6b3a76400008f615803565b6144b0565b905060006130378b8361429a565b9050613043818d615803565b9f9e505050505050505050505050505050565b6002818154811061306657600080fd5b60009182526020909120600590910201805460018201546002830154600384015460049094015463ffffffff841695506401000000009093046001600160a01b03908116949216926001600160801b03918216929180821691600160801b90041687565b60006002600d5460ff1660028111156130e5576130e5615340565b0361310657506001600160a01b03166000908152600b602052604090205490565b506001600160a01b031660009081526003602052604090205490565b60008054600160801b900460ff16600281111561314157613141615340565b1461315f5760405163067fe19560e41b815260040160405180910390fd5b60006002878154811061317457613174615642565b6000918252602082206005919091020160048101549092506001600160801b0316908715821760011b90506131ca7f0000000000000000000000000000000000000000000000000000000000000000600161566e565b613244826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff161461327e576040517f5f53dd9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600080891561335a576132d17f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006156a0565b6001901b6132e7846001600160801b03166144e1565b6001600160801b03166132fa9190615822565b1561332e5761332561331660016001600160801b038716615836565b865463ffffffff166000614567565b60030154613350565b7f00000000000000000000000000000000000000000000000000000000000000005b915084905061337b565b600385015491506133786133166001600160801b0386166001615856565b90505b600882901b60088a8a604051613392929190615632565b6040518091039020901b146133d3576040517f696550ff00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006133de8c614630565b905060006133ed836003015490565b6040517fe14ced320000000000000000000000000000000000000000000000000000000081527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e14ced329061345a908f908f908f908f908a906004016158c1565b6020604051808303816000875af1158015613479573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061349d91906156d4565b600485015491149150600090600290613526906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6135a0896001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6135aa91906158fb565b6135b4919061591e565b60ff1615905081151581036135f5576040517ffb4e40dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b875464010000000090046001600160a01b03161561363f576040517f9071e6af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505085547fffffffffffffffff0000000000000000000000000000000000000000ffffffff163364010000000002179095555050505050505050505050565b60008054600160801b900460ff16600281111561369d5761369d615340565b146136bb5760405163067fe19560e41b815260040160405180910390fd5b6000806000806136ca8661465f565b935093509350935060006136e0858585856149b8565b905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015613742573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061376691906156b7565b905060018903613833576001600160a01b0381166352f0f3ad8a846137973660011981013560f01c90036034013590565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815260048101939093526024830191909152604482015260206064820152608481018a905260a4015b6020604051808303816000875af1158015613809573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061382d91906156d4565b50611406565b60028903613852576001600160a01b0381166352f0f3ad8a8489613797565b60038903613871576001600160a01b0381166352f0f3ad8a8487613797565b600489036139a65760006138ae6001600160801b0385167f0000000000000000000000000000000000000000000000000000000000000000614a57565b6009546138bb919061566e565b6138c690600161566e565b90503660011981013560f01c90036054013581106138f3573660011981013560f01c9003605401356138f5565b805b90506001600160a01b0382166352f0f3ad8b8560405160e084901b7fffffffff000000000000000000000000000000000000000000000000000000001681526004810192909252602482015260c084901b604482015260086064820152608481018b905260a4016020604051808303816000875af115801561397b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061399f91906156d4565b5050611406565b60058903613a32576040517f52f0f3ad000000000000000000000000000000000000000000000000000000008152600481018a9052602481018390527f000000000000000000000000000000000000000000000000000000000000000060c01b604482015260086064820152608481018890526001600160a01b038216906352f0f3ad9060a4016137ea565b6040517fff137e6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000003660011981013560f01c9003601401356060613a9f61166b565b9050909192565b600081831015613ab65781613ab8565b825b90505b92915050565b60008160000151826020015183604001518460600151604051602001613b00949392919093845260208401929092526040830152606082015260800190565b604051602081830303815290604052805190602001209050919050565b60408051808201909152600080825260208201528151600003613b6c576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b60606000806000613b9a85614aec565b919450925090506001816001811115613bb557613bb5615340565b14613bec576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8451613bf8838561566e565b14613c2f576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b6040805180820190915260008082526020820152815260200190600190039081613c465790505093506000835b8651811015613d3457600080613cb96040518060400160405280858c60000151613c9d91906156a0565b8152602001858c60200151613cb2919061566e565b9052614aec565b509150915060405180604001604052808383613cd5919061566e565b8152602001848b60200151613cea919061566e565b815250888581518110613cff57613cff615642565b6020908102919091010152613d1560018561566e565b9350613d21818361566e565b613d2b908461566e565b92505050613c73565b50845250919392505050565b60606000806000613d5085614aec565b919450925090506000816001811115613d6b57613d6b615340565b14613da2576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613dac828461566e565b855114613de5576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61166285602001518484614f8a565b60028101546001600160a01b038316600090815260036020526040812080546001600160801b0390931692909190613e2d90849061566e565b90915550505050565b6040518181523660011981013560f01c90038284820160208401378260208301016000815260208101604052505092915050565b6000613e806001600160801b0384166001615856565b90506000613e9082866001614567565b9050600086901a8380613f5a5750613ec960027f0000000000000000000000000000000000000000000000000000000000000000615822565b6004830154600290613f4b906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b613f55919061591e565b60ff16145b15613fb25760ff811660011480613f74575060ff81166002145b613fad576040517ff40239db0000000000000000000000000000000000000000000000000000000081526004810188905260240161291c565b613ff0565b60ff811615613ff0576040517ff40239db0000000000000000000000000000000000000000000000000000000081526004810188905260240161291c565b50505050505050565b6001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b176000821361404f57631615e6386000526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f218311670de0b6b3a76400000215820261428857637c5f487d6000526004601cfd5b50670de0b6b3a7640000919091020490565b6000816000190483118202156142b85763bac65e5b6000526004601cfd5b50670de0b6b3a764000091020490565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d782136142f657919050565b680755bf798b4a1bf1e582126143145763a37bfec96000526004601cfd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b6000613ab8670de0b6b3a7640000836144c886613ff9565b6144d29190615940565b6144dc91906159fc565b6142c8565b600080614555837e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b600160ff919091161b90920392915050565b600080826145a7576145a26001600160801b0386167f000000000000000000000000000000000000000000000000000000000000000061501f565b6145b9565b6145b9856001600160801b031661515e565b9050600284815481106145ce576145ce615642565b906000526020600020906005020191505b60048201546001600160801b0382811691161461462857815460028054909163ffffffff1690811061461357614613615642565b906000526020600020906005020191506145df565b509392505050565b60008060008060006146418661465f565b9350935093509350614655848484846149b8565b9695505050505050565b600080600080600085905060006002828154811061467f5761467f615642565b600091825260209091206004600590920201908101549091507f000000000000000000000000000000000000000000000000000000000000000090614734906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff161161476e576040517fb34b5c2200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000815b60048301547f000000000000000000000000000000000000000000000000000000000000000090614813906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16925082111561488857825463ffffffff166148527f0000000000000000000000000000000000000000000000000000000000000000600161566e565b830361485c578391505b6002818154811061486f5761486f615642565b9060005260206000209060050201935080945050614772565b600481810154908401546001600160801b0391821691166000816001600160801b03166148cd6148c1856001600160801b031660011c90565b6001600160801b031690565b6001600160801b031614905080156149665760006148f3836001600160801b03166144e1565b6001600160801b0316111561494357600061492361491b60016001600160801b038616615836565b896001614567565b6003810154600490910154909c506001600160801b03169a506149499050565b6008549a505b600386015460048701549099506001600160801b031697506149aa565b600061497f61491b6001600160801b0385166001615856565b6003808901546004808b015492840154930154909e506001600160801b039182169d50919b50169850505b505050505050509193509193565b60006001600160801b03841615614a135760408051602081018790526001600160801b038087169282019290925260608101859052908316608082015260a00160405160208183030381529060405280519060200120611662565b8282604051602001614a389291909182526001600160801b0316602082015260400190565b6040516020818303038152906040528051906020012095945050505050565b600080614acb847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690508083036001841b600180831b0386831b17039250505092915050565b60008060008360000151600003614b2f576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f8111614b54576000600160009450945094505050614f83565b60b78111614c6a576000614b696080836156a0565b905080876000015111614ba8576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff00000000000000000000000000000000000000000000000000000000000000169082148015614c2057507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b15614c57576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5060019550935060009250614f83915050565b60bf8111614dc8576000614c7f60b7836156a0565b905080876000015111614cbe576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614d20576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614d68576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614d72818461566e565b895111614dab576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614db683600161566e565b9750955060009450614f839350505050565b60f78111614e2d576000614ddd60c0836156a0565b905080876000015111614e1c576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600195509350849250614f83915050565b6000614e3a60f7836156a0565b905080876000015111614e79576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614edb576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614f23576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614f2d818461566e565b895111614f66576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614f7183600161566e565b9750955060019450614f839350505050565b9193909250565b60608167ffffffffffffffff811115614fa557614fa56155a8565b6040519080825280601f01601f191660200182016040528015614fcf576020820181803683370190505b5090508115615018576000614fe4848661566e565b90506020820160005b84811015615005578281015182820152602001614fed565b84811115615014576000858301525b5050505b9392505050565b60008161509c846001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116150b25763b34b5c226000526004601cfd5b6150bb8361515e565b905081615138826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1611613abb57613ab861514e83600161566e565b6001600160801b038316906151ea565b600081196001830116816151d9827e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169390931c8015179392505050565b60008061525e847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050808303600180821b0385821b179250505092915050565b60008083601f84011261528c57600080fd5b50813567ffffffffffffffff8111156152a457600080fd5b6020830191508360208285010111156152bc57600080fd5b9250929050565b600080600083850360a08112156152d957600080fd5b60808112156152e757600080fd5b50839250608084013567ffffffffffffffff81111561530557600080fd5b6153118682870161527a565b9497909650939450505050565b6000806040838503121561533157600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b6003811061537457634e487b7160e01b600052602160045260246000fd5b50565b6020810161538483615356565b91905290565b6001600160a01b038116811461537457600080fd5b6000602082840312156153b157600080fd5b81356150188161538a565b6000806000606084860312156153d157600080fd5b505081359360208301359350604090920135919050565b6000815180845260005b8181101561540e576020818501810151868301820152016153f2565b81811115615420576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b602081526000613ab860208301846153e8565b60006020828403121561547857600080fd5b5035919050565b801515811461537457600080fd5b600080600080608085870312156154a357600080fd5b84359350602085013592506040850135915060608501356154c38161547f565b939692955090935050565b6000602082840312156154e057600080fd5b81356001600160801b038116811461501857600080fd5b6000806000806000806080878903121561551057600080fd5b8635955060208701356155228161547f565b9450604087013567ffffffffffffffff8082111561553f57600080fd5b61554b8a838b0161527a565b9096509450606089013591508082111561556457600080fd5b5061557189828a0161527a565b979a9699509497509295939492505050565b63ffffffff8416815282602082015260606040820152600061166260608301846153e8565b634e487b7160e01b600052604160045260246000fd5b6000608082840312156155d057600080fd5b6040516080810181811067ffffffffffffffff8211171561560157634e487b7160e01b600052604160045260246000fd5b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b8183823760009101908152919050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561568157615681615658565b500190565b6000600019820361569957615699615658565b5060010190565b6000828210156156b2576156b2615658565b500390565b6000602082840312156156c957600080fd5b81516150188161538a565b6000602082840312156156e657600080fd5b5051919050565b600067ffffffffffffffff80831681851680830382111561571057615710615658565b01949350505050565b600067ffffffffffffffff8083168185168183048111821515161561574057615740615658565b02949350505050565b600067ffffffffffffffff8381169083168181101561576a5761576a615658565b039392505050565b60006020828403121561578457600080fd5b81516150188161547f565b600080604083850312156157a257600080fd5b505080516020909101519092909150565b6000602082840312156157c557600080fd5b815163ffffffff8116811461501857600080fd5b634e487b7160e01b600052601260045260246000fd5b6000826157fe576157fe6157d9565b500490565b600081600019048311821515161561581d5761581d615658565b500290565b600082615831576158316157d9565b500690565b60006001600160801b038381169083168181101561576a5761576a615658565b60006001600160801b0380831681851680830382111561571057615710615658565b8183528181602085013750600060208284010152600060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b6060815260006158d5606083018789615878565b82810360208401526158e8818688615878565b9150508260408301529695505050505050565b600060ff821660ff84168082101561591557615915615658565b90039392505050565b600060ff831680615931576159316157d9565b8060ff84160691505092915050565b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60008413600084138583048511828216161561598157615981615658565b7f800000000000000000000000000000000000000000000000000000000000000060008712868205881281841616156159bc576159bc615658565b600087129250878205871284841616156159d8576159d8615658565b878505871281841616156159ee576159ee615658565b505050929093029392505050565b600082615a0b57615a0b6157d9565b60001983147f800000000000000000000000000000000000000000000000000000000000000083141615615a4157615a41615658565b50059056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x03?W`\x005`\xE0\x1C\x80cp\x87*\xA5\x11a\x01\xB0W\x80c\xC3\x95\xE1\xCA\x11a\0\xECW\x80c\xDA\xBD9m\x11a\0\x95W\x80c\xF8\xF4?\xF6\x11a\0oW\x80c\xF8\xF4?\xF6\x14a\x0B#W\x80c\xFA$\xF7C\x14a\x0BCW\x80c\xFA1Z\xA9\x14a\x0BgW\x80c\xFE+\xBE\xB2\x14a\x0B\x9AW`\0\x80\xFD[\x80c\xDA\xBD9m\x14a\n\x8DW\x80c\xEC^c\x08\x14a\n\xC0W\x80c\xEF\xF0\xF5\x92\x14a\n\xF3W`\0\x80\xFD[\x80c\xD5\xD4M\x80\x11a\0\xC6W\x80c\xD5\xD4M\x80\x14a\n\x1AW\x80c\xD6\xAE<\xD5\x14a\n:W\x80c\xD8\xCC\x1A<\x14a\nmW`\0\x80\xFD[\x80c\xC3\x95\xE1\xCA\x14a\teW\x80c\xC6\xF00\x8C\x14a\t\x85W\x80c\xCF\t\xE0\xD0\x14a\t\xF9W`\0\x80\xFD[\x80c\x8DE\n\x95\x11a\x01YW\x80c\xBB\xDC\x02\xDB\x11a\x013W\x80c\xBB\xDC\x02\xDB\x14a\x08\xB5W\x80c\xBC\xEF;U\x14a\x08\xF6W\x80c\xBD\x8D\xA9V\x14a\t\x18W\x80c\xC0\xD8\xBBt\x14a\t8W`\0\x80\xFD[\x80c\x8DE\n\x95\x14a\x07\xE2W\x80c\x99s^2\x14a\x07\xC0W\x80c\xA4E\xEC\xE6\x14a\x08\x15W`\0\x80\xFD[\x80c\x81)\xFC\x1C\x11a\x01\x8AW\x80c\x81)\xFC\x1C\x14a\x07\xA3W\x80c\x89\x80\xE0\xCC\x14a\x07\xABW\x80c\x8B\x85\x90+\x14a\x07\xC0W`\0\x80\xFD[\x80cp\x87*\xA5\x14a\x07fW\x80cxk\x84K\x14a\x07{W\x80c{\x0F\n\xDC\x14a\x07\x90W`\0\x80\xFD[\x80c>:\xC9\x12\x11a\x02\x7FW\x80cZ_\xA2\xD9\x11a\x02(W\x80c`\xE2td\x11a\x02\x02W\x80c`\xE2td\x14a\x06\xDEW\x80ccaPm\x14a\x06\xFEW\x80ckg\x16\xC0\x14a\x07 W\x80co\x03D\t\x14a\x07SW`\0\x80\xFD[\x80cZ_\xA2\xD9\x14a\x06vW\x80c\\\x0C\xBA3\x14a\x06\x96W\x80c`\x9D34\x14a\x06\xC9W`\0\x80\xFD[\x80cR\x9Dj\x8C\x11a\x02YW\x80cR\x9Dj\x8C\x14a\x05\xCAW\x80cT\xFDMP\x14a\x05\xF7W\x80cW\xDA\x95\x0E\x14a\x06FW`\0\x80\xFD[\x80c>:\xC9\x12\x14a\x05TW\x80c?\xC8\xCE\xF3\x14a\x05\x84W\x80cG'w\xC6\x14a\x05\xB7W`\0\x80\xFD[\x80c%\xFC*\xCE\x11a\x02\xECW\x80c0\xDB\xE5p\x11a\x02\xC6W\x80c0\xDB\xE5p\x14a\x04\xADW\x80c7\x8D\xD4\x8C\x14a\x04\xE5W\x80c7\xB1\xB2)\x14a\x04\xFFW\x80c:v\x84c\x14a\x05!W`\0\x80\xFD[\x80c%\xFC*\xCE\x14a\x04YW\x80c(\x10\xE1\xD6\x14a\x04xW\x80c*\xD6\x9A\xEB\x14a\x04\x8DW`\0\x80\xFD[\x80c \r.\xD2\x11a\x03\x1DW\x80c \r.\xD2\x14a\x03\xD1W\x80c\"*\xBFE\x14a\x03\xFFW\x80c%\x0Ei\xBD\x14a\x04?W`\0\x80\xFD[\x80c\x01\x93Q0\x14a\x03DW\x80c\x03\xC2\x92M\x14a\x03fW\x80c\x19\xEF\xFE\xB4\x14a\x03\x86W[`\0\x80\xFD[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x03da\x03_6`\x04aR\xC3V[a\x0B\xCAV[\0[4\x80\x15a\x03rW`\0\x80\xFD[Pa\x03da\x03\x816`\x04aS\x1EV[a\x0E\x89V[4\x80\x15a\x03\x92W`\0\x80\xFD[P`\0Ta\x03\xB3\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xDDW`\0\x80\xFD[P`\0Ta\x03\xF2\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x03\xC8\x91\x90aSwV[4\x80\x15a\x04\x0BW`\0\x80\xFD[Pa\x04/a\x04\x1A6`\x04aS\x9FV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xC8V[4\x80\x15a\x04KW`\0\x80\xFD[P`\nTa\x04/\x90`\xFF\x16\x81V[4\x80\x15a\x04eW`\0\x80\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x03\xC8V[4\x80\x15a\x04\x84W`\0\x80\xFD[Pa\x03\xF2a\x14\x11V[4\x80\x15a\x04\x99W`\0\x80\xFD[Pa\x04ja\x04\xA86`\x04aS\x1EV[a\x15\xE9V[4\x80\x15a\x04\xB9W`\0\x80\xFD[P`\x01Ta\x04\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xC8V[4\x80\x15a\x04\xF1W`\0\x80\xFD[P`\rTa\x03\xF2\x90`\xFF\x16\x81V[4\x80\x15a\x05\x0BW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x035``\x1Ca\x04\xCDV[4\x80\x15a\x05-W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xCDV[4\x80\x15a\x05`W`\0\x80\xFD[P`\0Ta\x04/\x90r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x05\x90W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xCDV[a\x03da\x05\xC56`\x04aS\xBCV[a\x16\x1FV[4\x80\x15a\x05\xD6W`\0\x80\xFD[Pa\x04ja\x05\xE56`\x04aS\x9FV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x06\x03W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F1.8.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03\xC8\x91\x90aTSV[4\x80\x15a\x06RW`\0\x80\xFD[P`\x08T`\tTa\x06a\x91\x90\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\xC8V[4\x80\x15a\x06\x82W`\0\x80\xFD[Pa\x04ja\x06\x916`\x04aTfV[a\x161V[4\x80\x15a\x06\xA2W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xCDV[4\x80\x15a\x06\xD5W`\0\x80\xFD[Pa\x069a\x16kV[4\x80\x15a\x06\xEAW`\0\x80\xFD[Pa\x03da\x06\xF96`\x04aS\x9FV[a\x16yV[4\x80\x15a\x07\nW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015a\x04jV[4\x80\x15a\x07,W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xB3V[a\x03da\x07a6`\x04aT\x8DV[a\x19\x91V[4\x80\x15a\x07rW`\0\x80\xFD[P`\tTa\x04jV[4\x80\x15a\x07\x87W`\0\x80\xFD[Pa\x03da#2V[a\x03da\x07\x9E6`\x04aS\xBCV[a'QV[a\x03da'^V[4\x80\x15a\x07\xB7W`\0\x80\xFD[P`\x02Ta\x04jV[4\x80\x15a\x07\xCCW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015a\x04jV[4\x80\x15a\x07\xEEW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\x08!W`\0\x80\xFD[Pa\x08wa\x0806`\x04aTfV[`\x07` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x82\x16\x91a\x01\0\x81\x04c\xFF\xFF\xFF\xFF\x16\x91e\x01\0\0\0\0\0\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q\x94\x15\x15\x85Rc\xFF\xFF\xFF\xFF\x90\x93\x16` \x85\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01a\x03\xC8V[4\x80\x15a\x08\xC1W`\0\x80\xFD[P`@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x03\xC8V[4\x80\x15a\t\x02W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015a\x04jV[4\x80\x15a\t$W`\0\x80\xFD[Pa\x03\xB3a\t36`\x04aTfV[a,\xFBV[4\x80\x15a\tDW`\0\x80\xFD[Pa\x04ja\tS6`\x04aS\x9FV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\tqW`\0\x80\xFD[Pa\x04ja\t\x806`\x04aT\xCEV[a.\x95V[4\x80\x15a\t\x91W`\0\x80\xFD[Pa\t\xA5a\t\xA06`\x04aTfV[a0VV[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x98\x16\x88R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16` \x89\x01R\x95\x90\x94\x16\x94\x86\x01\x94\x90\x94R`\x01`\x01`\x80\x1B\x03\x91\x82\x16``\x86\x01R`\x80\x85\x01R\x91\x82\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\xC8V[4\x80\x15a\n\x05W`\0\x80\xFD[P`\0Ta\x03\xB3\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\n&W`\0\x80\xFD[Pa\x04ja\n56`\x04aS\x9FV[a0\xCAV[4\x80\x15a\nFW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\nyW`\0\x80\xFD[Pa\x03da\n\x886`\x04aT\xF7V[a1\"V[4\x80\x15a\n\x99W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xB3V[4\x80\x15a\n\xCCW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\n\xFFW`\0\x80\xFD[Pa\x04/a\x0B\x0E6`\x04aTfV[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B/W`\0\x80\xFD[Pa\x03da\x0B>6`\x04aS\xBCV[a6~V[4\x80\x15a\x0BOW`\0\x80\xFD[Pa\x0BXa:dV[`@Qa\x03\xC8\x93\x92\x91\x90aU\x83V[4\x80\x15a\x0BsW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04jV[4\x80\x15a\x0B\xA6W`\0\x80\xFD[Pa\x04/a\x0B\xB56`\x04aTfV[`\x06` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0B\xE9Wa\x0B\xE9aS@V[\x14a\x0C\x07W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x0CZW`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cs6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015\x90V[\x90V[a\x0C\x8Aa\x0C\x856\x86\x90\x03\x86\x01\x86aU\xBEV[a:\xC1V[\x14a\x0C\xC1W`@Q\x7F\x9C\xC0\x0B[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82``\x015\x82\x82`@Qa\x0C\xD6\x92\x91\x90aV2V[`@Q\x80\x91\x03\x90 \x14a\r\x15W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r^a\rY\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\x1D\x92PPPV[a;\x8AV[\x90P`\0a\r\x85\x82`\x08\x81Q\x81\x10a\rxWa\rxaVBV[` \x02` \x01\x01Qa=@V[\x90P` \x81Q\x11\x15a\r\xC3W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x81\x01Q\x82Q\x90\x91\x03`\x03\x1B\x1C6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015\x81\x03a\x0E\x1AW`@Q\x7F\xB8\xED\x880\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UPP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UPPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0E\xA8Wa\x0E\xA8aS@V[\x14a\x0E\xC6W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a\x0E\xDBWa\x0E\xDBaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x90P`\0a\x0E\xF6\x84a,\xFBV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x10\x15a\x0F_W`@Q\x7F\xF2D\x0BS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x06` R`@\x90 T`\xFF\x16\x15a\x0F\xA8W`@Q\x7F\xF1\xA9E\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x05` R`@\x90 \x80T\x80\x15\x80\x15a\x0F\xC5WP\x85\x15\x15[\x15a\x10(W\x83Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0\x81\x15a\x0F\xEBW\x81a\x0F\xFAV[`\x01\x86\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x90Pa\x10\x06\x81\x87a=\xF4V[PPP`\0\x94\x85RPP`\x06` RPP`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0\x86\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x82\x04c\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x93\x81\x01\x93\x90\x93R`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01Ra\x10\xACW`\x01`\x01`\x80\x1B\x03`@\x82\x01R`\x01\x81R`\0\x86\x90\x03a\x10\xACW\x81\x95P[`\0\x86\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a\x10\xC4\x91\x90aVnV[\x90P`\0\x83\x82\x11a\x10\xD5W\x81a\x10\xD7V[\x83[` \x84\x01Q\x90\x91Pc\xFF\xFF\xFF\xFF\x16[\x81\x81\x10\x15a\x11\xF7W`\0\x86\x82\x81T\x81\x10a\x11\x02Wa\x11\x02aVBV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83R`\x06\x90\x91R`@\x90\x91 T\x90\x91P`\xFF\x16a\x11ZW`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x82\x81T\x81\x10a\x11oWa\x11oaVBV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T\x90\x91Pd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x11\xB6WP`\x04\x81\x01T`@\x87\x01Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11[\x15a\x11\xE2W`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x87\x01R`\x04\x81\x01T`\x01`\x01`\x80\x1B\x03\x16`@\x87\x01R[PP\x80\x80a\x11\xEF\x90aV\x86V[\x91PPa\x10\xE6V[Pc\xFF\xFF\xFF\xFF\x81\x81\x16` \x85\x81\x01\x91\x82R`\0\x8C\x81R`\x07\x90\x91R`@\x90\x81\x90 \x86Q\x81T\x93Q\x92\x88\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x94\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\x16\x17a\x01\0\x92\x90\x94\x16\x91\x82\x02\x93\x90\x93\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x82U``\x85\x01Q`\x01\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x84\x90\x03a\x14\x06W``\x83\x01Q`\0\x8A\x81R`\x06` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x89\x15\x80\x15a\x13CWP`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16[\x15a\x13\x9EW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x13^\x81\x8Aa=\xF4V[\x88T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x88Ua\x14\x04V[a\x13\xCB`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x13\xB6W\x81a\x13\xC5V[`\x01\x89\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x89a=\xF4V[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x83\x16\x02\x17\x88U[P[PPPPPPPPPV[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x142Wa\x142aS@V[\x14a\x14PW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80R`\x06` R\x7FT\xCD\xD3i\xE4\xE8\xA8Q^R\xCAr\xEC\x81l!\x01\x83\x1A\xD1\xF1\x8B\xF4A\x02\xED\x17\x14Y\xC9\xB4\xF8T`\xFF\x16a\x14\xB4W`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x02`\0\x81T\x81\x10a\x14\xD3Wa\x14\xD3aVBV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\x01W`\x01a\x15\x04V[`\x02[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x17\x83U\x92\x93P\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17`\x01`\x80\x1B\x83`\x02\x81\x11\x15a\x15\xA8Wa\x15\xA8aS@V[\x02\x17\x90U`\x02\x81\x11\x15a\x15\xBDWa\x15\xBDaS@V[`@Q\x7F^\x18o\t\xB9\xC94\x91\xF1N'~\xEA\x7F\xAA]\xE6\xA2\xD4\xBD\xA7Zy\xAFz6\x84\xFB\xFBB\xDA`\x90`\0\x90\xA2\x90V[`\x05` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x16\x05W`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[\x90P\x90V[a\x16,\x83\x83\x83`\x01a\x19\x91V[PPPV[`\0\x81\x81R`\x07` \x90\x81R`@\x80\x83 `\x05\x90\x92R\x82 \x80T\x82Ta\x16b\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aV\xA0V[\x95\x94PPPPPV[``a\x16\x1A`T` a>6V[a\x16\x81a#2V[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\x9CWa\x16\x9CaS@V[\x03a\x16\xC0WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x90 Ta\x17/V[`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\xD9Wa\x16\xD9aS@V[\x03a\x16\xFDWP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 Ta\x17/V[`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x18\x0CW`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x0C` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F~\xEE(\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c~\xEE(\x8D\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x04W=`\0\x80>=`\0\xFD[PPPPPPV[\x80`\0\x03a\x18FW`@Q\x7F\x17\xBF\xE5\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x0B` \x90\x81R`@\x80\x83 \x83\x90U`\x03\x90\x91R\x80\x82 \x91\x90\x91UQ\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\0W=`\0\x80>=`\0\xFD[PPPP`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19QW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19VV[``\x91P[PP\x90P\x80a\x16,W`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x19\xB0Wa\x19\xB0aS@V[\x14a\x19\xCEW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x84\x81T\x81\x10a\x19\xE3Wa\x19\xE3aVBV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x81\x16\x84R`\x01`\x01`\xA0\x1B\x03d\x01\0\0\0\0\x90\x91\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01\x81\x01T\x90\x93\x16\x90\x82\x01R`\x02\x82\x01T`\x01`\x01`\x80\x1B\x03\x90\x81\x16``\x83\x01R`\x03\x83\x01T`\x80\x83\x01\x81\x90R`\x04\x90\x93\x01T\x80\x82\x16`\xA0\x84\x01R`\x01`\x80\x1B\x90\x04\x16`\xC0\x82\x01R\x91P\x85\x14a\x1A\xA7W`@Q\x7F0\x14\x032\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0\x81\x01Q`\0\x83\x15`\x01`\x01`\x80\x1B\x03\x83\x16\x17`\x01\x1B\x90P`\0a\x1B<\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x86\x15\x80a\x1BwWPa\x1Bt\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02aVnV[\x81\x14[\x80\x15a\x1B\x81WP\x84\x15[\x15a\x1B\xB8W`@Q\x7F\xA4&7\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x80\x15a\x1B\xDEWP\x86\x15[\x15a\x1C\x15W`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\x1CoW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aVnV[\x81\x03a\x1C\xACWa\x1C\xAC\x86\x88\x85\x88a>jV[4a\x1C\xB6\x83a.\x95V[\x14a\x1C\xEDW`@Q\x7F\x86 \xAA\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\xF8\x88a,\xFBV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a\x1D`W`@Q\x7F3\x81\xD1\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1D\x8D`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aV\xA0V[\x83\x03a\x1E\xB1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x15\x91\x90aV\xB7V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ERW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ev\x91\x90aV\xD4V[a\x1E\xAA\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aV\xEDV[\x90Pa\x1FDV[a\x1E\xDC`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aV\xA0V[\x83\x03a\x1F\x17Wa\x1E\xAA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aW\x19V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[a\x1Fx\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aWIV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\x93\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xDAWa\x1F\xD7\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aWIV[\x91P[`\0`@\x83\x90\x1BB\x17`\0\x8A\x81R`\x80\x87\x90\x1B`\x01`\x01`\x80\x1B\x03\x8D\x16\x17` R`@\x81 \x91\x92P\x90`\0\x81\x81R`\x04` R`@\x90 T\x90\x91P`\xFF\x16\x15a OW`@Q\x7F\x80I~;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x04`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x02`@Q\x80`\xE0\x01`@R\x80\x8Dc\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x014`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\x80\x1B\x03\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`\x05`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\x01`\x02\x80T\x90Pa\"F\x91\x90aV\xA0V[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x91\x01\x91\x90\x91U3\x82R`\x0B\x90R`@\x81 \x80T4\x92\x90a\"z\x90\x84\x90aVnV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\"\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\xF0W=`\0\x80>=`\0\xFD[PP`@Q3\x93P\x8D\x92P\x8E\x91P\x7F\x9B2Et\x0E\xC3\xB1U\t\x8AU\xBE\x84\x95zM\xA1>\xAF\x7F\x14\xA8\xBCoS\x12l\x0B\x93P\xF2\xBE\x90`\0\x90\xA4PPPPPPPPPPPPV[`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a#KWa#KaS@V[\x14\x80a#mWP`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a#kWa#kaS@V[\x14[\x15a#tWV[`\0`\rT`\xFF\x16`\x02\x81\x11\x15a#\x8DWa#\x8DaS@V[\x14a#\xC4W`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$F\x91\x90aWrV[\x15a$}W`@Q\x7F7\x9A~\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a$\xD9W`@Q\x7F\xC1\x05&\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x03\x14\xD2\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x03\x14\xD2\xB3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%}\x91\x90aWrV[\x90P\x80a%\xB6W`@Q\x7FHQ\xBD\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x17\xCF!\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x17\xCF!\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&0W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a&AWP`\x01[P`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xE6\x91\x90aWrV[\x90P\x80\x15a'\0W`\r\x80T`\xFF\x19\x16`\x01\x17\x90Ua'\x0EV[`\r\x80T`\xFF\x19\x16`\x02\x17\x90U[`\rT`@Q\x7F\x99\x08\xEA\xAC\x06E\xDF\x9D\x07\x04\xD0j\xDC\x9E\x073|\x95\x1D\xE2\xF0k_(6\x15\x1DH\xD5\xE4r/\x91a'E\x91`\xFF\x90\x91\x16\x90aSwV[`@Q\x80\x91\x03\x90\xA1PPV[a\x16,\x83\x83\x83`\0a\x19\x91V[`\0Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a'\xB0W`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD8>\xF2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(4\x91\x90aW\x8FV[\x90\x92P\x90P\x81a(pW`@Q\x7Fjk\xC3\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x01\x81\x90R`\x08\x82\x90U`\t\x81\x90U`z6\x14a(\xC7W`@Q\x7F\x98$\xBD\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x806`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015\x11a)%W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x80\x82R`\0` \x80\x84\x01\x82\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x805``\x90\x81\x1C\x87\x89\x01\x81\x81R`\x01`\x01`\x80\x1B\x034\x81\x81\x16\x94\x8B\x01\x94\x85R`\x14\x90\x95\x015`\x80\x8B\x01\x90\x81R`\x01`\xA0\x8C\x01\x81\x81RB\x84\x16`\xC0\x8E\x01\x90\x81R`\x02\x80T\x93\x84\x01\x81U\x8CR\x9CQ\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE`\x05\x90\x93\x02\x92\x83\x01\x80T\x9AQ\x91\x90\x9D\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x9A\x16\x99\x90\x99\x17d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x9A\x8B\x16\x02\x17\x90\x9BU\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCF\x84\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x98\x16\x17\x90\x96U\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD0\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD1\x85\x01U\x95Q\x96Q\x96\x81\x16`\x01`\x80\x1B\x97\x90\x91\x16\x96\x90\x96\x02\x95\x90\x95\x17\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD2\x90\x91\x01U\x81Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U\x91\x81R`\x0B\x90\x91R\x91\x82 \x80T\x91\x92\x90\x91a+f\x90\x84\x90aVnV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a+\xC8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a+\xDCW=`\0\x80>=`\0\xFD[PP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPP`@\x80Q\x7F<\x9F9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91c<\x9F9|\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xDF\x91\x90aW\xB3V[`\n\x80T`\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x92\x90\x92\x14\x17\x90UPPV[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a-\x1CWa-\x1CaS@V[\x14a-:W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a-OWa-OaVBV[`\0\x91\x82R` \x82 `\x05\x90\x91\x02\x01\x80T\x90\x92Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x14a-\xB5W\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10a-\x8DWa-\x8DaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01`\x04\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90P[`\x04\x82\x01T`\0\x90a-\xE0\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a-\xF4\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaV\xA0V[a.\na-\xD3\x84`\x01`\x01`\x80\x1B\x03\x16`@\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a.\x1E\x91\x90aVnV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a.kW\x80a\x16bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`\0\x80a/\x12\x83`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a/qW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d.\x90\xED\xD0\0b\x06\x1A\x80c\x11\xE1\xA3\0`\0a/\x8C\x83\x83aW\xEFV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0a/\xC3\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aX\x03V[\x90P`\0a/\xE1a/\xDCg\r\xE0\xB6\xB3\xA7d\0\0\x86aX\x03V[a?\xF9V[\x90P`\0a/\xEF\x84\x84aBKV[\x90P`\0a/\xFD\x83\x83aB\x9AV[\x90P`\0a0\n\x82aB\xC8V[\x90P`\0a0)\x82a0$g\r\xE0\xB6\xB3\xA7d\0\0\x8FaX\x03V[aD\xB0V[\x90P`\0a07\x8B\x83aB\x9AV[\x90Pa0C\x81\x8DaX\x03V[\x9F\x9EPPPPPPPPPPPPPPPV[`\x02\x81\x81T\x81\x10a0fW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01Tc\xFF\xFF\xFF\xFF\x84\x16\x95Pd\x01\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x92\x16\x92`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x87V[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a0\xE5Wa0\xE5aS@V[\x03a1\x06WP`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0B` R`@\x90 T\x90V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a1AWa1AaS@V[\x14a1_W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x87\x81T\x81\x10a1tWa1taVBV[`\0\x91\x82R` \x82 `\x05\x91\x90\x91\x02\x01`\x04\x81\x01T\x90\x92P`\x01`\x01`\x80\x1B\x03\x16\x90\x87\x15\x82\x17`\x01\x1B\x90Pa1\xCA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aVnV[a2D\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x14a2~W`@Q\x7F_S\xDD\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x89\x15a3ZWa2\xD1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aV\xA0V[`\x01\x90\x1Ba2\xE7\x84`\x01`\x01`\x80\x1B\x03\x16aD\xE1V[`\x01`\x01`\x80\x1B\x03\x16a2\xFA\x91\x90aX\"V[\x15a3.Wa3%a3\x16`\x01`\x01`\x01`\x80\x1B\x03\x87\x16aX6V[\x86Tc\xFF\xFF\xFF\xFF\x16`\0aEgV[`\x03\x01Ta3PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x91P\x84\x90Pa3{V[`\x03\x85\x01T\x91Pa3xa3\x16`\x01`\x01`\x80\x1B\x03\x86\x16`\x01aXVV[\x90P[`\x08\x82\x90\x1B`\x08\x8A\x8A`@Qa3\x92\x92\x91\x90aV2V[`@Q\x80\x91\x03\x90 \x90\x1B\x14a3\xD3W`@Q\x7FieP\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a3\xDE\x8CaF0V[\x90P`\0a3\xED\x83`\x03\x01T\x90V[`@Q\x7F\xE1L\xED2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE1L\xED2\x90a4Z\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8A\x90`\x04\x01aX\xC1V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a4yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\x9D\x91\x90aV\xD4V[`\x04\x85\x01T\x91\x14\x91P`\0\x90`\x02\x90a5&\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a5\xA0\x89`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a5\xAA\x91\x90aX\xFBV[a5\xB4\x91\x90aY\x1EV[`\xFF\x16\x15\x90P\x81\x15\x15\x81\x03a5\xF5W`@Q\x7F\xFBN@\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a6?W`@Q\x7F\x90q\xE6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x163d\x01\0\0\0\0\x02\x17\x90\x95UPPPPPPPPPPPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a6\x9DWa6\x9DaS@V[\x14a6\xBBW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80a6\xCA\x86aF_V[\x93P\x93P\x93P\x93P`\0a6\xE0\x85\x85\x85\x85aI\xB8V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7f\x91\x90aV\xB7V[\x90P`\x01\x89\x03a83W`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84a7\x976`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R`D\x82\x01R` `d\x82\x01R`\x84\x81\x01\x8A\x90R`\xA4\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a8\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8-\x91\x90aV\xD4V[Pa\x14\x06V[`\x02\x89\x03a8RW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x89a7\x97V[`\x03\x89\x03a8qW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x87a7\x97V[`\x04\x89\x03a9\xA6W`\0a8\xAE`\x01`\x01`\x80\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aJWV[`\tTa8\xBB\x91\x90aVnV[a8\xC6\x90`\x01aVnV[\x90P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015\x81\x10a8\xF3W6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015a8\xF5V[\x80[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16cR\xF0\xF3\xAD\x8B\x85`@Q`\xE0\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`\xC0\x84\x90\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x8B\x90R`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a9{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x9F\x91\x90aV\xD4V[PPa\x14\x06V[`\x05\x89\x03a:2W`@Q\x7FR\xF0\xF3\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cR\xF0\xF3\xAD\x90`\xA4\x01a7\xEAV[`@Q\x7F\xFF\x13~e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015``a:\x9Fa\x16kV[\x90P\x90\x91\x92V[`\0\x81\x83\x10\x15a:\xB6W\x81a:\xB8V[\x82[\x90P[\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a;\0\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03a;lW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0a;\x9A\x85aJ\xECV[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15a;\xB5Wa;\xB5aS@V[\x14a;\xECW`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa;\xF8\x83\x85aVnV[\x14a</W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a<FW\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15a=4W`\0\x80a<\xB9`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01Qa<\x9D\x91\x90aV\xA0V[\x81R` \x01\x85\x8C` \x01Qa<\xB2\x91\x90aVnV[\x90RaJ\xECV[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a<\xD5\x91\x90aVnV[\x81R` \x01\x84\x8B` \x01Qa<\xEA\x91\x90aVnV[\x81RP\x88\x85\x81Q\x81\x10a<\xFFWa<\xFFaVBV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra=\x15`\x01\x85aVnV[\x93Pa=!\x81\x83aVnV[a=+\x90\x84aVnV[\x92PPPa<sV[P\x84RP\x91\x93\x92PPPV[```\0\x80`\0a=P\x85aJ\xECV[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15a=kWa=kaS@V[\x14a=\xA2W`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\xAC\x82\x84aVnV[\x85Q\x14a=\xE5W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16b\x85` \x01Q\x84\x84aO\x8AV[`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x91\x90a>-\x90\x84\x90aVnV[\x90\x91UPPPPV[`@Q\x81\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x82\x84\x82\x01` \x84\x017\x82` \x83\x01\x01`\0\x81R` \x81\x01`@RPP\x92\x91PPV[`\0a>\x80`\x01`\x01`\x80\x1B\x03\x84\x16`\x01aXVV[\x90P`\0a>\x90\x82\x86`\x01aEgV[\x90P`\0\x86\x90\x1A\x83\x80a?ZWPa>\xC9`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aX\"V[`\x04\x83\x01T`\x02\x90a?K\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a?U\x91\x90aY\x1EV[`\xFF\x16\x14[\x15a?\xB2W`\xFF\x81\x16`\x01\x14\x80a?tWP`\xFF\x81\x16`\x02\x14[a?\xADW`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a)\x1CV[a?\xF0V[`\xFF\x81\x16\x15a?\xF0W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a)\x1CV[PPPPPPPV[`\x01`\x01`\x80\x1B\x03\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17`\0\x82\x13a@OWc\x16\x15\xE68`\0R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[`\0x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x83\x11g\r\xE0\xB6\xB3\xA7d\0\0\x02\x15\x82\x02aB\x88Wc|_H}`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x02\x15aB\xB8Wc\xBA\xC6^[`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13aB\xF6W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12aC\x14Wc\xA3{\xFE\xC9`\0R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a:\xB8g\r\xE0\xB6\xB3\xA7d\0\0\x83aD\xC8\x86a?\xF9V[aD\xD2\x91\x90aY@V[aD\xDC\x91\x90aY\xFCV[aB\xC8V[`\0\x80aEU\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01`\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x80\x82aE\xA7WaE\xA2`\x01`\x01`\x80\x1B\x03\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aP\x1FV[aE\xB9V[aE\xB9\x85`\x01`\x01`\x80\x1B\x03\x16aQ^V[\x90P`\x02\x84\x81T\x81\x10aE\xCEWaE\xCEaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91P[`\x04\x82\x01T`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x91\x16\x14aF(W\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10aF\x13WaF\x13aVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91PaE\xDFV[P\x93\x92PPPV[`\0\x80`\0\x80`\0aFA\x86aF_V[\x93P\x93P\x93P\x93PaFU\x84\x84\x84\x84aI\xB8V[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x85\x90P`\0`\x02\x82\x81T\x81\x10aF\x7FWaF\x7FaVBV[`\0\x91\x82R` \x90\x91 `\x04`\x05\x90\x92\x02\x01\x90\x81\x01T\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aG4\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aGnW`@Q\x7F\xB3K\\\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[`\x04\x83\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aH\x13\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x92P\x82\x11\x15aH\x88W\x82Tc\xFF\xFF\xFF\xFF\x16aHR\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aVnV[\x83\x03aH\\W\x83\x91P[`\x02\x81\x81T\x81\x10aHoWaHoaVBV[\x90`\0R` `\0 \x90`\x05\x02\x01\x93P\x80\x94PPaGrV[`\x04\x81\x81\x01T\x90\x84\x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16`\0\x81`\x01`\x01`\x80\x1B\x03\x16aH\xCDaH\xC1\x85`\x01`\x01`\x80\x1B\x03\x16`\x01\x1C\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x14\x90P\x80\x15aIfW`\0aH\xF3\x83`\x01`\x01`\x80\x1B\x03\x16aD\xE1V[`\x01`\x01`\x80\x1B\x03\x16\x11\x15aICW`\0aI#aI\x1B`\x01`\x01`\x01`\x80\x1B\x03\x86\x16aX6V[\x89`\x01aEgV[`\x03\x81\x01T`\x04\x90\x91\x01T\x90\x9CP`\x01`\x01`\x80\x1B\x03\x16\x9APaII\x90PV[`\x08T\x9AP[`\x03\x86\x01T`\x04\x87\x01T\x90\x99P`\x01`\x01`\x80\x1B\x03\x16\x97PaI\xAAV[`\0aI\x7FaI\x1B`\x01`\x01`\x80\x1B\x03\x85\x16`\x01aXVV[`\x03\x80\x89\x01T`\x04\x80\x8B\x01T\x92\x84\x01T\x93\x01T\x90\x9EP`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x9DP\x91\x9BP\x16\x98PP[PPPPPPP\x91\x93P\x91\x93V[`\0`\x01`\x01`\x80\x1B\x03\x84\x16\x15aJ\x13W`@\x80Q` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R\x90\x83\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x16bV[\x82\x82`@Q` \x01aJ8\x92\x91\x90\x91\x82R`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95\x94PPPPPV[`\0\x80aJ\xCB\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[`\0\x80`\0\x83`\0\x01Q`\0\x03aK/W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11aKTW`\0`\x01`\0\x94P\x94P\x94PPPaO\x83V[`\xB7\x81\x11aLjW`\0aKi`\x80\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aK\xA8W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15aL WP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15aLWW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92PaO\x83\x91PPV[`\xBF\x81\x11aM\xC8W`\0aL\x7F`\xB7\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aL\xBEW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aM W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aMhW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aMr\x81\x84aVnV[\x89Q\x11aM\xABW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aM\xB6\x83`\x01aVnV[\x97P\x95P`\0\x94PaO\x83\x93PPPPV[`\xF7\x81\x11aN-W`\0aM\xDD`\xC0\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aN\x1CW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92PaO\x83\x91PPV[`\0aN:`\xF7\x83aV\xA0V[\x90P\x80\x87`\0\x01Q\x11aNyW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aN\xDBW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aO#W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aO-\x81\x84aVnV[\x89Q\x11aOfW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aOq\x83`\x01aVnV[\x97P\x95P`\x01\x94PaO\x83\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\xA5WaO\xA5aU\xA8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aO\xCFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15aP\x18W`\0aO\xE4\x84\x86aVnV[\x90P` \x82\x01`\0[\x84\x81\x10\x15aP\x05W\x82\x81\x01Q\x82\x82\x01R` \x01aO\xEDV[\x84\x81\x11\x15aP\x14W`\0\x85\x83\x01R[PPP[\x93\x92PPPV[`\0\x81aP\x9C\x84`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aP\xB2Wc\xB3K\\\"`\0R`\x04`\x1C\xFD[aP\xBB\x83aQ^V[\x90P\x81aQ8\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11a:\xBBWa:\xB8aQN\x83`\x01aVnV[`\x01`\x01`\x80\x1B\x03\x83\x16\x90aQ\xEAV[`\0\x81\x19`\x01\x83\x01\x16\x81aQ\xD9\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`\0\x80aR^\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x80\x82\x1B\x03\x85\x82\x1B\x17\x92PPP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aR\x8CW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\xA4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aR\xBCW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15aR\xD9W`\0\x80\xFD[`\x80\x81\x12\x15aR\xE7W`\0\x80\xFD[P\x83\x92P`\x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\x05W`\0\x80\xFD[aS\x11\x86\x82\x87\x01aRzV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15aS1W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aStWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01aS\x84\x83aSVV[\x91\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aStW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aS\xB1W`\0\x80\xFD[\x815aP\x18\x81aS\x8AV[`\0\x80`\0``\x84\x86\x03\x12\x15aS\xD1W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aT\x0EW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aS\xF2V[\x81\x81\x11\x15aT W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a:\xB8` \x83\x01\x84aS\xE8V[`\0` \x82\x84\x03\x12\x15aTxW`\0\x80\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14aStW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\xA3W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aT\xC3\x81aT\x7FV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aT\xE0W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aP\x18W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aU\x10W`\0\x80\xFD[\x865\x95P` \x87\x015aU\"\x81aT\x7FV[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aU?W`\0\x80\xFD[aUK\x8A\x83\x8B\x01aRzV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aUdW`\0\x80\xFD[PaUq\x89\x82\x8A\x01aRzV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x16b``\x83\x01\x84aS\xE8V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15aU\xD0W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aV\x01WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aV\x81WaV\x81aVXV[P\x01\x90V[`\0`\0\x19\x82\x03aV\x99WaV\x99aVXV[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aV\xB2WaV\xB2aVXV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aV\xC9W`\0\x80\xFD[\x81QaP\x18\x81aS\x8AV[`\0` \x82\x84\x03\x12\x15aV\xE6W`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aW\x10WaW\x10aVXV[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aW@WaW@aVXV[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aWjWaWjaVXV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aW\x84W`\0\x80\xFD[\x81QaP\x18\x81aT\x7FV[`\0\x80`@\x83\x85\x03\x12\x15aW\xA2W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15aW\xC5W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14aP\x18W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aW\xFEWaW\xFEaW\xD9V[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aX\x1DWaX\x1DaVXV[P\x02\x90V[`\0\x82aX1WaX1aW\xD9V[P\x06\x90V[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aWjWaWjaVXV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aW\x10WaW\x10aVXV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0aX\xD5``\x83\x01\x87\x89aXxV[\x82\x81\x03` \x84\x01RaX\xE8\x81\x86\x88aXxV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15aY\x15WaY\x15aVXV[\x90\x03\x93\x92PPPV[`\0`\xFF\x83\x16\x80aY1WaY1aW\xD9V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15aY\x81WaY\x81aVXV[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15aY\xBCWaY\xBCaVXV[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15aY\xD8WaY\xD8aVXV[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15aY\xEEWaY\xEEaVXV[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82aZ\x0BWaZ\x0BaW\xD9V[`\0\x19\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aZAWaZAaVXV[P\x05\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BondDistributionMode(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BondDistributionMode> for u8 {
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
        impl BondDistributionMode {
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
        impl From<u8> for BondDistributionMode {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<BondDistributionMode> for u8 {
            fn from(value: BondDistributionMode) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for BondDistributionMode {
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
        impl alloy_sol_types::EventTopic for BondDistributionMode {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<GameStatus> for u8 {
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
        impl GameStatus {
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
        impl From<u8> for GameStatus {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<GameStatus> for u8 {
            fn from(value: GameStatus) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GameStatus {
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
        impl alloy_sol_types::EventTopic for GameStatus {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Claim(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Claim>
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
        impl Claim {
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
        impl From<alloy::sol_types::private::FixedBytes<32>> for Claim {
            fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Claim> for alloy::sol_types::private::FixedBytes<32> {
            fn from(value: Claim) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Claim {
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
        impl alloy_sol_types::EventTopic for Claim {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Clock(u128);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Clock> for u128 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<128>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Clock {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u128) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u128 {
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
        impl From<u128> for Clock {
            fn from(value: u128) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Clock> for u128 {
            fn from(value: Clock) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Clock {
            type RustType = u128;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Clock {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Duration(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Duration> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Duration {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u64 {
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
        impl From<u64> for Duration {
            fn from(value: u64) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Duration> for u64 {
            fn from(value: Duration) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Duration {
            type RustType = u64;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Duration {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameType(u32);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<GameType> for u32 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl GameType {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u32) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u32 {
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
        impl From<u32> for GameType {
            fn from(value: u32) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<GameType> for u32 {
            fn from(value: GameType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GameType {
            type RustType = u32;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GameType {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Hash(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Hash>
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
        impl Hash {
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
        impl From<alloy::sol_types::private::FixedBytes<32>> for Hash {
            fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Hash> for alloy::sol_types::private::FixedBytes<32> {
            fn from(value: Hash) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Hash {
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
        impl alloy_sol_types::EventTopic for Hash {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Position(u128);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Position> for u128 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<128>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Position {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u128) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u128 {
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
        impl From<u128> for Position {
            fn from(value: u128) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Position> for u128 {
            fn from(value: Position) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Position {
            type RustType = u128;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                128,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Position {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Timestamp(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Timestamp> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Timestamp {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u64 {
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
        impl From<u64> for Timestamp {
            fn from(value: u64) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Timestamp> for u64 {
            fn from(value: Timestamp) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Timestamp {
            type RustType = u64;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Timestamp {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct GameConstructorParams { GameType gameType; Claim absolutePrestate; uint256 maxGameDepth; uint256 splitDepth; Duration clockExtension; Duration maxClockDuration; address vm; address weth; address anchorStateRegistry; uint256 l2ChainId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameConstructorParams {
        #[allow(missing_docs)]
        pub gameType: <GameType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub absolutePrestate: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub maxGameDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub splitDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub clockExtension: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub maxClockDuration: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub vm: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub weth: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub anchorStateRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l2ChainId: alloy::sol_types::private::primitives::aliases::U256,
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
            GameType,
            Claim,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            Duration,
            Duration,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <GameType as alloy::sol_types::SolType>::RustType,
            <Claim as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <Duration as alloy::sol_types::SolType>::RustType,
            <Duration as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<GameConstructorParams> for UnderlyingRustTuple<'_> {
            fn from(value: GameConstructorParams) -> Self {
                (
                    value.gameType,
                    value.absolutePrestate,
                    value.maxGameDepth,
                    value.splitDepth,
                    value.clockExtension,
                    value.maxClockDuration,
                    value.vm,
                    value.weth,
                    value.anchorStateRegistry,
                    value.l2ChainId,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameConstructorParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    gameType: tuple.0,
                    absolutePrestate: tuple.1,
                    maxGameDepth: tuple.2,
                    splitDepth: tuple.3,
                    clockExtension: tuple.4,
                    maxClockDuration: tuple.5,
                    vm: tuple.6,
                    weth: tuple.7,
                    anchorStateRegistry: tuple.8,
                    l2ChainId: tuple.9,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GameConstructorParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GameConstructorParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <GameType as alloy_sol_types::SolType>::tokenize(&self.gameType),
                    <Claim as alloy_sol_types::SolType>::tokenize(
                        &self.absolutePrestate,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxGameDepth),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.splitDepth),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.clockExtension,
                    ),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.maxClockDuration,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.vm,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.weth,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.anchorStateRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2ChainId),
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
        impl alloy_sol_types::SolType for GameConstructorParams {
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
        impl alloy_sol_types::SolStruct for GameConstructorParams {
            const NAME: &'static str = "GameConstructorParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GameConstructorParams(uint32 gameType,bytes32 absolutePrestate,uint256 maxGameDepth,uint256 splitDepth,uint64 clockExtension,uint64 maxClockDuration,address vm,address weth,address anchorStateRegistry,uint256 l2ChainId)",
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
                    <GameType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.gameType,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.absolutePrestate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxGameDepth)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.splitDepth)
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.clockExtension,
                        )
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxClockDuration,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.vm,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.weth,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.anchorStateRegistry,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2ChainId)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GameConstructorParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <GameType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gameType,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.absolutePrestate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxGameDepth,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.splitDepth,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.clockExtension,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxClockDuration,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.vm,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.weth,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.anchorStateRegistry,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2ChainId,
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
                <GameType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gameType,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.absolutePrestate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxGameDepth,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.splitDepth,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.clockExtension,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxClockDuration,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.vm,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.weth,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.anchorStateRegistry,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2ChainId,
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
    /**Custom error with signature `AnchorRootNotFound()` and selector `0x6a6bc3b2`.
```solidity
error AnchorRootNotFound();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AnchorRootNotFound;
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
        impl ::core::convert::From<AnchorRootNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: AnchorRootNotFound) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AnchorRootNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AnchorRootNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AnchorRootNotFound()";
            const SELECTOR: [u8; 4] = [106u8, 107u8, 195u8, 178u8];
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
    /**Custom error with signature `BadExtraData()` and selector `0x9824bdab`.
```solidity
error BadExtraData();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BadExtraData;
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
        impl ::core::convert::From<BadExtraData> for UnderlyingRustTuple<'_> {
            fn from(value: BadExtraData) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BadExtraData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BadExtraData {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BadExtraData()";
            const SELECTOR: [u8; 4] = [152u8, 36u8, 189u8, 171u8];
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
    /**Custom error with signature `BlockNumberMatches()` and selector `0xb8ed8830`.
```solidity
error BlockNumberMatches();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BlockNumberMatches;
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
        impl ::core::convert::From<BlockNumberMatches> for UnderlyingRustTuple<'_> {
            fn from(value: BlockNumberMatches) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BlockNumberMatches {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BlockNumberMatches {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BlockNumberMatches()";
            const SELECTOR: [u8; 4] = [184u8, 237u8, 136u8, 48u8];
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
    /**Custom error with signature `CannotDefendRootClaim()` and selector `0xa42637bc`.
```solidity
error CannotDefendRootClaim();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotDefendRootClaim;
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
        impl ::core::convert::From<CannotDefendRootClaim> for UnderlyingRustTuple<'_> {
            fn from(value: CannotDefendRootClaim) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotDefendRootClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotDefendRootClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotDefendRootClaim()";
            const SELECTOR: [u8; 4] = [164u8, 38u8, 55u8, 188u8];
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
    /**Custom error with signature `ClaimAboveSplit()` and selector `0xb34b5c22`.
```solidity
error ClaimAboveSplit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClaimAboveSplit;
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
        impl ::core::convert::From<ClaimAboveSplit> for UnderlyingRustTuple<'_> {
            fn from(value: ClaimAboveSplit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClaimAboveSplit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClaimAboveSplit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClaimAboveSplit()";
            const SELECTOR: [u8; 4] = [179u8, 75u8, 92u8, 34u8];
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
    /**Custom error with signature `ClaimAlreadyExists()` and selector `0x80497e3b`.
```solidity
error ClaimAlreadyExists();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClaimAlreadyExists;
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
        impl ::core::convert::From<ClaimAlreadyExists> for UnderlyingRustTuple<'_> {
            fn from(value: ClaimAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClaimAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClaimAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClaimAlreadyExists()";
            const SELECTOR: [u8; 4] = [128u8, 73u8, 126u8, 59u8];
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
    /**Custom error with signature `ClaimAlreadyResolved()` and selector `0xf1a94581`.
```solidity
error ClaimAlreadyResolved();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClaimAlreadyResolved;
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
        impl ::core::convert::From<ClaimAlreadyResolved> for UnderlyingRustTuple<'_> {
            fn from(value: ClaimAlreadyResolved) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClaimAlreadyResolved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClaimAlreadyResolved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClaimAlreadyResolved()";
            const SELECTOR: [u8; 4] = [241u8, 169u8, 69u8, 129u8];
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
    /**Custom error with signature `ClockNotExpired()` and selector `0xf2440b53`.
```solidity
error ClockNotExpired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClockNotExpired;
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
        impl ::core::convert::From<ClockNotExpired> for UnderlyingRustTuple<'_> {
            fn from(value: ClockNotExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClockNotExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClockNotExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClockNotExpired()";
            const SELECTOR: [u8; 4] = [242u8, 68u8, 11u8, 83u8];
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
    /**Custom error with signature `ClockTimeExceeded()` and selector `0x3381d114`.
```solidity
error ClockTimeExceeded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClockTimeExceeded;
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
        impl ::core::convert::From<ClockTimeExceeded> for UnderlyingRustTuple<'_> {
            fn from(value: ClockTimeExceeded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ClockTimeExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClockTimeExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClockTimeExceeded()";
            const SELECTOR: [u8; 4] = [51u8, 129u8, 209u8, 20u8];
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
    /**Custom error with signature `ContentLengthMismatch()` and selector `0x66c94485`.
```solidity
error ContentLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContentLengthMismatch;
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
        impl ::core::convert::From<ContentLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: ContentLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContentLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContentLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContentLengthMismatch()";
            const SELECTOR: [u8; 4] = [102u8, 201u8, 68u8, 133u8];
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
    /**Custom error with signature `DuplicateStep()` and selector `0x9071e6af`.
```solidity
error DuplicateStep();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DuplicateStep;
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
        impl ::core::convert::From<DuplicateStep> for UnderlyingRustTuple<'_> {
            fn from(value: DuplicateStep) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DuplicateStep {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DuplicateStep {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DuplicateStep()";
            const SELECTOR: [u8; 4] = [144u8, 113u8, 230u8, 175u8];
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
    /**Custom error with signature `EmptyItem()` and selector `0x5ab458fb`.
```solidity
error EmptyItem();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyItem;
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
        impl ::core::convert::From<EmptyItem> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyItem) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyItem {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyItem {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyItem()";
            const SELECTOR: [u8; 4] = [90u8, 180u8, 88u8, 251u8];
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
    /**Custom error with signature `GameDepthExceeded()` and selector `0x56f57b2b`.
```solidity
error GameDepthExceeded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameDepthExceeded;
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
        impl ::core::convert::From<GameDepthExceeded> for UnderlyingRustTuple<'_> {
            fn from(value: GameDepthExceeded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameDepthExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameDepthExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameDepthExceeded()";
            const SELECTOR: [u8; 4] = [86u8, 245u8, 123u8, 43u8];
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
    /**Custom error with signature `GameNotFinalized()` and selector `0x4851bd9b`.
```solidity
error GameNotFinalized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameNotFinalized;
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
        impl ::core::convert::From<GameNotFinalized> for UnderlyingRustTuple<'_> {
            fn from(value: GameNotFinalized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameNotFinalized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameNotFinalized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameNotFinalized()";
            const SELECTOR: [u8; 4] = [72u8, 81u8, 189u8, 155u8];
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
    /**Custom error with signature `GameNotInProgress()` and selector `0x67fe1950`.
```solidity
error GameNotInProgress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameNotInProgress;
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
        impl ::core::convert::From<GameNotInProgress> for UnderlyingRustTuple<'_> {
            fn from(value: GameNotInProgress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameNotInProgress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameNotInProgress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameNotInProgress()";
            const SELECTOR: [u8; 4] = [103u8, 254u8, 25u8, 80u8];
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
    /**Custom error with signature `GameNotResolved()` and selector `0xc105260a`.
```solidity
error GameNotResolved();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameNotResolved;
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
        impl ::core::convert::From<GameNotResolved> for UnderlyingRustTuple<'_> {
            fn from(value: GameNotResolved) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameNotResolved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GameNotResolved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GameNotResolved()";
            const SELECTOR: [u8; 4] = [193u8, 5u8, 38u8, 10u8];
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
    /**Custom error with signature `GamePaused()` and selector `0x379a7ed9`.
```solidity
error GamePaused();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GamePaused;
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
        impl ::core::convert::From<GamePaused> for UnderlyingRustTuple<'_> {
            fn from(value: GamePaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GamePaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GamePaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GamePaused()";
            const SELECTOR: [u8; 4] = [55u8, 154u8, 126u8, 217u8];
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
    /**Custom error with signature `IncorrectBondAmount()` and selector `0x8620aa19`.
```solidity
error IncorrectBondAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IncorrectBondAmount;
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
        impl ::core::convert::From<IncorrectBondAmount> for UnderlyingRustTuple<'_> {
            fn from(value: IncorrectBondAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IncorrectBondAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IncorrectBondAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IncorrectBondAmount()";
            const SELECTOR: [u8; 4] = [134u8, 32u8, 170u8, 25u8];
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
    /**Custom error with signature `InvalidBondDistributionMode()` and selector `0x078a3df4`.
```solidity
error InvalidBondDistributionMode();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBondDistributionMode;
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
        impl ::core::convert::From<InvalidBondDistributionMode>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBondDistributionMode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidBondDistributionMode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBondDistributionMode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBondDistributionMode()";
            const SELECTOR: [u8; 4] = [7u8, 138u8, 61u8, 244u8];
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
    /**Custom error with signature `InvalidChallengePeriod()` and selector `0xb4e12433`.
```solidity
error InvalidChallengePeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidChallengePeriod;
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
        impl ::core::convert::From<InvalidChallengePeriod> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidChallengePeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidChallengePeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidChallengePeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidChallengePeriod()";
            const SELECTOR: [u8; 4] = [180u8, 225u8, 36u8, 51u8];
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
    /**Custom error with signature `InvalidClockExtension()` and selector `0x8d77ecac`.
```solidity
error InvalidClockExtension();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidClockExtension;
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
        impl ::core::convert::From<InvalidClockExtension> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidClockExtension) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidClockExtension {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidClockExtension {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidClockExtension()";
            const SELECTOR: [u8; 4] = [141u8, 119u8, 236u8, 172u8];
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
    /**Custom error with signature `InvalidDataRemainder()` and selector `0x5c5537b8`.
```solidity
error InvalidDataRemainder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDataRemainder;
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
        impl ::core::convert::From<InvalidDataRemainder> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDataRemainder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidDataRemainder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDataRemainder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDataRemainder()";
            const SELECTOR: [u8; 4] = [92u8, 85u8, 55u8, 184u8];
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
    /**Custom error with signature `InvalidDisputedClaimIndex()` and selector `0x30140332`.
```solidity
error InvalidDisputedClaimIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDisputedClaimIndex;
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
        impl ::core::convert::From<InvalidDisputedClaimIndex>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDisputedClaimIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidDisputedClaimIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDisputedClaimIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDisputedClaimIndex()";
            const SELECTOR: [u8; 4] = [48u8, 20u8, 3u8, 50u8];
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
    /**Custom error with signature `InvalidHeader()` and selector `0xbabb01dd`.
```solidity
error InvalidHeader();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHeader;
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
        impl ::core::convert::From<InvalidHeader> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHeader) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHeader {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHeader {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHeader()";
            const SELECTOR: [u8; 4] = [186u8, 187u8, 1u8, 221u8];
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
    /**Custom error with signature `InvalidHeaderRLP()` and selector `0xd81d583b`.
```solidity
error InvalidHeaderRLP();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHeaderRLP;
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
        impl ::core::convert::From<InvalidHeaderRLP> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHeaderRLP) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHeaderRLP {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHeaderRLP {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHeaderRLP()";
            const SELECTOR: [u8; 4] = [216u8, 29u8, 88u8, 59u8];
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
    /**Custom error with signature `InvalidLocalIdent()` and selector `0xff137e65`.
```solidity
error InvalidLocalIdent();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLocalIdent;
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
        impl ::core::convert::From<InvalidLocalIdent> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLocalIdent) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLocalIdent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLocalIdent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLocalIdent()";
            const SELECTOR: [u8; 4] = [255u8, 19u8, 126u8, 101u8];
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
    /**Custom error with signature `InvalidOutputRootProof()` and selector `0x9cc00b5b`.
```solidity
error InvalidOutputRootProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOutputRootProof;
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
        impl ::core::convert::From<InvalidOutputRootProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOutputRootProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOutputRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOutputRootProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOutputRootProof()";
            const SELECTOR: [u8; 4] = [156u8, 192u8, 11u8, 91u8];
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
    /**Custom error with signature `InvalidParent()` and selector `0x5f53dd98`.
```solidity
error InvalidParent();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidParent;
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
        impl ::core::convert::From<InvalidParent> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidParent) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidParent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidParent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidParent()";
            const SELECTOR: [u8; 4] = [95u8, 83u8, 221u8, 152u8];
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
    /**Custom error with signature `InvalidPrestate()` and selector `0x696550ff`.
```solidity
error InvalidPrestate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPrestate;
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
        impl ::core::convert::From<InvalidPrestate> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPrestate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPrestate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPrestate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPrestate()";
            const SELECTOR: [u8; 4] = [105u8, 101u8, 80u8, 255u8];
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
    /**Custom error with signature `InvalidSplitDepth()` and selector `0xe62ccf39`.
```solidity
error InvalidSplitDepth();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSplitDepth;
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
        impl ::core::convert::From<InvalidSplitDepth> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSplitDepth) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSplitDepth {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSplitDepth {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSplitDepth()";
            const SELECTOR: [u8; 4] = [230u8, 44u8, 207u8, 57u8];
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
    /**Custom error with signature `L2BlockNumberChallenged()` and selector `0x0ea2e752`.
```solidity
error L2BlockNumberChallenged();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct L2BlockNumberChallenged;
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
        impl ::core::convert::From<L2BlockNumberChallenged> for UnderlyingRustTuple<'_> {
            fn from(value: L2BlockNumberChallenged) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for L2BlockNumberChallenged {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for L2BlockNumberChallenged {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "L2BlockNumberChallenged()";
            const SELECTOR: [u8; 4] = [14u8, 162u8, 231u8, 82u8];
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
    /**Custom error with signature `MaxDepthTooLarge()` and selector `0x77dfe332`.
```solidity
error MaxDepthTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxDepthTooLarge;
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
        impl ::core::convert::From<MaxDepthTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: MaxDepthTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxDepthTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxDepthTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxDepthTooLarge()";
            const SELECTOR: [u8; 4] = [119u8, 223u8, 227u8, 50u8];
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
    /**Custom error with signature `NoCreditToClaim()` and selector `0x17bfe5f7`.
```solidity
error NoCreditToClaim();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoCreditToClaim;
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
        impl ::core::convert::From<NoCreditToClaim> for UnderlyingRustTuple<'_> {
            fn from(value: NoCreditToClaim) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoCreditToClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoCreditToClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoCreditToClaim()";
            const SELECTOR: [u8; 4] = [23u8, 191u8, 229u8, 247u8];
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
    /**Custom error with signature `OutOfOrderResolution()` and selector `0x9a076646`.
```solidity
error OutOfOrderResolution();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfOrderResolution;
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
        impl ::core::convert::From<OutOfOrderResolution> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfOrderResolution) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfOrderResolution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfOrderResolution {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfOrderResolution()";
            const SELECTOR: [u8; 4] = [154u8, 7u8, 102u8, 70u8];
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
    /**Custom error with signature `ReservedGameType()` and selector `0x39f68b38`.
```solidity
error ReservedGameType();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReservedGameType;
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
        impl ::core::convert::From<ReservedGameType> for UnderlyingRustTuple<'_> {
            fn from(value: ReservedGameType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReservedGameType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReservedGameType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReservedGameType()";
            const SELECTOR: [u8; 4] = [57u8, 246u8, 139u8, 56u8];
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
    /**Custom error with signature `UnexpectedList()` and selector `0x1ff9b2e4`.
```solidity
error UnexpectedList();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnexpectedList;
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
        impl ::core::convert::From<UnexpectedList> for UnderlyingRustTuple<'_> {
            fn from(value: UnexpectedList) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedList {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnexpectedList {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnexpectedList()";
            const SELECTOR: [u8; 4] = [31u8, 249u8, 178u8, 228u8];
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
    /**Custom error with signature `UnexpectedRootClaim(bytes32)` and selector `0xf40239db`.
```solidity
error UnexpectedRootClaim(Claim rootClaim);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnexpectedRootClaim {
        #[allow(missing_docs)]
        pub rootClaim: <Claim as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (Claim,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (<Claim as alloy::sol_types::SolType>::RustType,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnexpectedRootClaim> for UnderlyingRustTuple<'_> {
            fn from(value: UnexpectedRootClaim) -> Self {
                (value.rootClaim,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedRootClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { rootClaim: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnexpectedRootClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnexpectedRootClaim(bytes32)";
            const SELECTOR: [u8; 4] = [244u8, 2u8, 57u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Claim as alloy_sol_types::SolType>::tokenize(&self.rootClaim),)
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
    /**Custom error with signature `UnexpectedString()` and selector `0x4b9c6abe`.
```solidity
error UnexpectedString();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnexpectedString;
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
        impl ::core::convert::From<UnexpectedString> for UnderlyingRustTuple<'_> {
            fn from(value: UnexpectedString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnexpectedString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnexpectedString()";
            const SELECTOR: [u8; 4] = [75u8, 156u8, 106u8, 190u8];
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
    /**Custom error with signature `ValidStep()` and selector `0xfb4e40dd`.
```solidity
error ValidStep();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidStep;
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
        impl ::core::convert::From<ValidStep> for UnderlyingRustTuple<'_> {
            fn from(value: ValidStep) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidStep {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidStep {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidStep()";
            const SELECTOR: [u8; 4] = [251u8, 78u8, 64u8, 221u8];
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
    /**Event with signature `GameClosed(uint8)` and selector `0x9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f`.
```solidity
event GameClosed(BondDistributionMode bondDistributionMode);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GameClosed {
        #[allow(missing_docs)]
        pub bondDistributionMode: <BondDistributionMode as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for GameClosed {
            type DataTuple<'a> = (BondDistributionMode,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GameClosed(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                153u8, 8u8, 234u8, 172u8, 6u8, 69u8, 223u8, 157u8, 7u8, 4u8, 208u8,
                106u8, 220u8, 158u8, 7u8, 51u8, 124u8, 149u8, 29u8, 226u8, 240u8, 107u8,
                95u8, 40u8, 54u8, 21u8, 29u8, 72u8, 213u8, 228u8, 114u8, 47u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    bondDistributionMode: data.0,
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
                    <BondDistributionMode as alloy_sol_types::SolType>::tokenize(
                        &self.bondDistributionMode,
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
        impl alloy_sol_types::private::IntoLogData for GameClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GameClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GameClosed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Move(uint256,bytes32,address)` and selector `0x9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be`.
```solidity
event Move(uint256 indexed parentIndex, Claim indexed claim, address indexed claimant);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Move {
        #[allow(missing_docs)]
        pub parentIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub claim: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub claimant: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Move {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Move(uint256,bytes32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                155u8, 50u8, 69u8, 116u8, 14u8, 195u8, 177u8, 85u8, 9u8, 138u8, 85u8,
                190u8, 132u8, 149u8, 122u8, 77u8, 161u8, 62u8, 175u8, 127u8, 20u8, 168u8,
                188u8, 111u8, 83u8, 18u8, 108u8, 11u8, 147u8, 80u8, 242u8, 190u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    parentIndex: topics.1,
                    claim: topics.2,
                    claimant: topics.3,
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
                    self.parentIndex.clone(),
                    self.claim.clone(),
                    self.claimant.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.parentIndex);
                out[2usize] = <Claim as alloy_sol_types::EventTopic>::encode_topic(
                    &self.claim,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.claimant,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Move {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Move> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Move) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Resolved(uint8)` and selector `0x5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da60`.
```solidity
event Resolved(GameStatus indexed status);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Resolved {
        #[allow(missing_docs)]
        pub status: <GameStatus as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for Resolved {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, GameStatus);
            const SIGNATURE: &'static str = "Resolved(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                94u8, 24u8, 111u8, 9u8, 185u8, 201u8, 52u8, 145u8, 241u8, 78u8, 39u8,
                126u8, 234u8, 127u8, 170u8, 93u8, 230u8, 162u8, 212u8, 189u8, 167u8,
                90u8, 121u8, 175u8, 122u8, 54u8, 132u8, 251u8, 251u8, 66u8, 218u8, 96u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { status: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.status.clone())
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
                out[1usize] = <GameStatus as alloy_sol_types::EventTopic>::encode_topic(
                    &self.status,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Resolved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Resolved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Resolved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(GameConstructorParams _params);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _params: <GameConstructorParams as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameConstructorParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameConstructorParams as alloy::sol_types::SolType>::RustType,
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
                    (value._params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _params: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (GameConstructorParams,);
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
                    <GameConstructorParams as alloy_sol_types::SolType>::tokenize(
                        &self._params,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `absolutePrestate()` and selector `0x8d450a95`.
```solidity
function absolutePrestate() external view returns (Claim absolutePrestate_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct absolutePrestateCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`absolutePrestate()`](absolutePrestateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct absolutePrestateReturn {
        #[allow(missing_docs)]
        pub absolutePrestate_: <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<absolutePrestateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: absolutePrestateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for absolutePrestateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Claim,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<absolutePrestateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: absolutePrestateReturn) -> Self {
                    (value.absolutePrestate_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for absolutePrestateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { absolutePrestate_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for absolutePrestateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Claim as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Claim,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "absolutePrestate()";
            const SELECTOR: [u8; 4] = [141u8, 69u8, 10u8, 149u8];
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
                (<Claim as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: absolutePrestateReturn = r.into();
                        r.absolutePrestate_
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
                        let r: absolutePrestateReturn = r.into();
                        r.absolutePrestate_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addLocalData(uint256,uint256,uint256)` and selector `0xf8f43ff6`.
```solidity
function addLocalData(uint256 _ident, uint256 _execLeafIdx, uint256 _partOffset) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLocalDataCall {
        #[allow(missing_docs)]
        pub _ident: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _execLeafIdx: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _partOffset: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addLocalData(uint256,uint256,uint256)`](addLocalDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLocalDataReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<addLocalDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: addLocalDataCall) -> Self {
                    (value._ident, value._execLeafIdx, value._partOffset)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLocalDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ident: tuple.0,
                        _execLeafIdx: tuple.1,
                        _partOffset: tuple.2,
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
            impl ::core::convert::From<addLocalDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addLocalDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLocalDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addLocalDataReturn {
            fn _tokenize(
                &self,
            ) -> <addLocalDataCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addLocalDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addLocalDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addLocalData(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [248u8, 244u8, 63u8, 246u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._execLeafIdx),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._partOffset),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addLocalDataReturn::_tokenize(ret)
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
    /**Function with signature `anchorStateRegistry()` and selector `0x5c0cba33`.
```solidity
function anchorStateRegistry() external view returns (address registry_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct anchorStateRegistryCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`anchorStateRegistry()`](anchorStateRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct anchorStateRegistryReturn {
        #[allow(missing_docs)]
        pub registry_: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<anchorStateRegistryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: anchorStateRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for anchorStateRegistryCall {
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
            impl ::core::convert::From<anchorStateRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: anchorStateRegistryReturn) -> Self {
                    (value.registry_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for anchorStateRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { registry_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for anchorStateRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "anchorStateRegistry()";
            const SELECTOR: [u8; 4] = [92u8, 12u8, 186u8, 51u8];
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
                        let r: anchorStateRegistryReturn = r.into();
                        r.registry_
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
                        let r: anchorStateRegistryReturn = r.into();
                        r.registry_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `attack(bytes32,uint256,bytes32)` and selector `0x472777c6`.
```solidity
function attack(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct attackCall {
        #[allow(missing_docs)]
        pub _disputed: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _claim: <Claim as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`attack(bytes32,uint256,bytes32)`](attackCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct attackReturn {}
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
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<attackCall> for UnderlyingRustTuple<'_> {
                fn from(value: attackCall) -> Self {
                    (value._disputed, value._parentIndex, value._claim)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for attackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _disputed: tuple.0,
                        _parentIndex: tuple.1,
                        _claim: tuple.2,
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
            impl ::core::convert::From<attackReturn> for UnderlyingRustTuple<'_> {
                fn from(value: attackReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for attackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl attackReturn {
            fn _tokenize(
                &self,
            ) -> <attackCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for attackCall {
            type Parameters<'a> = (Claim, alloy::sol_types::sol_data::Uint<256>, Claim);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = attackReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "attack(bytes32,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [71u8, 39u8, 119u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._disputed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._parentIndex),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._claim),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                attackReturn::_tokenize(ret)
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
    /**Function with signature `bondDistributionMode()` and selector `0x378dd48c`.
```solidity
function bondDistributionMode() external view returns (BondDistributionMode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bondDistributionModeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`bondDistributionMode()`](bondDistributionModeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bondDistributionModeReturn {
        #[allow(missing_docs)]
        pub _0: <BondDistributionMode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<bondDistributionModeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: bondDistributionModeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bondDistributionModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (BondDistributionMode,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BondDistributionMode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<bondDistributionModeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: bondDistributionModeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bondDistributionModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bondDistributionModeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <BondDistributionMode as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (BondDistributionMode,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bondDistributionMode()";
            const SELECTOR: [u8; 4] = [55u8, 141u8, 212u8, 140u8];
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
                (<BondDistributionMode as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: bondDistributionModeReturn = r.into();
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
                        let r: bondDistributionModeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `challengeRootL2Block((bytes32,bytes32,bytes32,bytes32),bytes)` and selector `0x01935130`.
```solidity
function challengeRootL2Block(Types.OutputRootProof memory _outputRootProof, bytes memory _headerRLP) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeRootL2BlockCall {
        #[allow(missing_docs)]
        pub _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _headerRLP: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`challengeRootL2Block((bytes32,bytes32,bytes32,bytes32),bytes)`](challengeRootL2BlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeRootL2BlockReturn {}
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
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<challengeRootL2BlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeRootL2BlockCall) -> Self {
                    (value._outputRootProof, value._headerRLP)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeRootL2BlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _outputRootProof: tuple.0,
                        _headerRLP: tuple.1,
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
            impl ::core::convert::From<challengeRootL2BlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeRootL2BlockReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeRootL2BlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl challengeRootL2BlockReturn {
            fn _tokenize(
                &self,
            ) -> <challengeRootL2BlockCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeRootL2BlockCall {
            type Parameters<'a> = (
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = challengeRootL2BlockReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengeRootL2Block((bytes32,bytes32,bytes32,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [1u8, 147u8, 81u8, 48u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Types::OutputRootProof as alloy_sol_types::SolType>::tokenize(
                        &self._outputRootProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._headerRLP,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                challengeRootL2BlockReturn::_tokenize(ret)
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
    /**Function with signature `claimCredit(address)` and selector `0x60e27464`.
```solidity
function claimCredit(address _recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimCreditCall {
        #[allow(missing_docs)]
        pub _recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`claimCredit(address)`](claimCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimCreditReturn {}
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
            impl ::core::convert::From<claimCreditCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimCreditCall) -> Self {
                    (value._recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCreditCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _recipient: tuple.0 }
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
            impl ::core::convert::From<claimCreditReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimCreditReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl claimCreditReturn {
            fn _tokenize(
                &self,
            ) -> <claimCreditCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimCreditReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimCredit(address)";
            const SELECTOR: [u8; 4] = [96u8, 226u8, 116u8, 100u8];
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
                        &self._recipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                claimCreditReturn::_tokenize(ret)
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
    /**Function with signature `claimData(uint256)` and selector `0xc6f0308c`.
```solidity
function claimData(uint256) external view returns (uint32 parentIndex, address counteredBy, address claimant, uint128 bond, Claim claim, Position position, Clock clock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataCall(pub alloy::sol_types::private::primitives::aliases::U256);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`claimData(uint256)`](claimDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataReturn {
        #[allow(missing_docs)]
        pub parentIndex: u32,
        #[allow(missing_docs)]
        pub counteredBy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub claimant: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub bond: u128,
        #[allow(missing_docs)]
        pub claim: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub position: <Position as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub clock: <Clock as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<claimDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
                Claim,
                Position,
                Clock,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u128,
                <Claim as alloy::sol_types::SolType>::RustType,
                <Position as alloy::sol_types::SolType>::RustType,
                <Clock as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<claimDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataReturn) -> Self {
                    (
                        value.parentIndex,
                        value.counteredBy,
                        value.claimant,
                        value.bond,
                        value.claim,
                        value.position,
                        value.clock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        parentIndex: tuple.0,
                        counteredBy: tuple.1,
                        claimant: tuple.2,
                        bond: tuple.3,
                        claim: tuple.4,
                        position: tuple.5,
                        clock: tuple.6,
                    }
                }
            }
        }
        impl claimDataReturn {
            fn _tokenize(
                &self,
            ) -> <claimDataCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.parentIndex),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.counteredBy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.claimant,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.bond),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self.claim),
                    <Position as alloy_sol_types::SolType>::tokenize(&self.position),
                    <Clock as alloy_sol_types::SolType>::tokenize(&self.clock),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimDataCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimDataReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
                Claim,
                Position,
                Clock,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimData(uint256)";
            const SELECTOR: [u8; 4] = [198u8, 240u8, 48u8, 140u8];
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
                claimDataReturn::_tokenize(ret)
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
    /**Function with signature `claimDataLen()` and selector `0x8980e0cc`.
```solidity
function claimDataLen() external view returns (uint256 len_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataLenCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`claimDataLen()`](claimDataLenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDataLenReturn {
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
            impl ::core::convert::From<claimDataLenCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataLenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataLenCall {
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
            impl ::core::convert::From<claimDataLenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimDataLenReturn) -> Self {
                    (value.len_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimDataLenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { len_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimDataLenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimDataLen()";
            const SELECTOR: [u8; 4] = [137u8, 128u8, 224u8, 204u8];
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
                        let r: claimDataLenReturn = r.into();
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
                        let r: claimDataLenReturn = r.into();
                        r.len_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `claims(bytes32)` and selector `0xeff0f592`.
```solidity
function claims(Hash) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimsCall(pub <Hash as alloy::sol_types::SolType>::RustType);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`claims(bytes32)`](claimsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimsReturn {
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
            type UnderlyingSolTuple<'a> = (Hash,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<claimsCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimsCall {
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
            impl ::core::convert::From<claimsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimsCall {
            type Parameters<'a> = (Hash,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claims(bytes32)";
            const SELECTOR: [u8; 4] = [239u8, 240u8, 245u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Hash as alloy_sol_types::SolType>::tokenize(&self.0),)
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
                        let r: claimsReturn = r.into();
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
                        let r: claimsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `clockExtension()` and selector `0x6b6716c0`.
```solidity
function clockExtension() external view returns (Duration clockExtension_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockExtensionCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`clockExtension()`](clockExtensionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockExtensionReturn {
        #[allow(missing_docs)]
        pub clockExtension_: <Duration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<clockExtensionCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockExtensionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockExtensionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Duration,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Duration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<clockExtensionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: clockExtensionReturn) -> Self {
                    (value.clockExtension_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clockExtensionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { clockExtension_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockExtensionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Duration as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Duration,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clockExtension()";
            const SELECTOR: [u8; 4] = [107u8, 103u8, 22u8, 192u8];
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
                (<Duration as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: clockExtensionReturn = r.into();
                        r.clockExtension_
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
                        let r: clockExtensionReturn = r.into();
                        r.clockExtension_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `closeGame()` and selector `0x786b844b`.
```solidity
function closeGame() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeGameCall;
    ///Container type for the return parameters of the [`closeGame()`](closeGameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeGameReturn {}
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
            impl ::core::convert::From<closeGameCall> for UnderlyingRustTuple<'_> {
                fn from(value: closeGameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeGameCall {
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
            impl ::core::convert::From<closeGameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: closeGameReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeGameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl closeGameReturn {
            fn _tokenize(
                &self,
            ) -> <closeGameCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeGameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeGameReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "closeGame()";
            const SELECTOR: [u8; 4] = [120u8, 107u8, 132u8, 75u8];
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
                closeGameReturn::_tokenize(ret)
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
    /**Function with signature `createdAt()` and selector `0xcf09e0d0`.
```solidity
function createdAt() external view returns (Timestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createdAtCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createdAt()`](createdAtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createdAtReturn {
        #[allow(missing_docs)]
        pub _0: <Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createdAtCall> for UnderlyingRustTuple<'_> {
                fn from(value: createdAtCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createdAtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Timestamp,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createdAtReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createdAtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createdAtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createdAtCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Timestamp as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Timestamp,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createdAt()";
            const SELECTOR: [u8; 4] = [207u8, 9u8, 224u8, 208u8];
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
                (<Timestamp as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: createdAtReturn = r.into();
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
                        let r: createdAtReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `credit(address)` and selector `0xd5d44d80`.
```solidity
function credit(address _recipient) external view returns (uint256 credit_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creditCall {
        #[allow(missing_docs)]
        pub _recipient: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`credit(address)`](creditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creditReturn {
        #[allow(missing_docs)]
        pub credit_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<creditCall> for UnderlyingRustTuple<'_> {
                fn from(value: creditCall) -> Self {
                    (value._recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creditCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _recipient: tuple.0 }
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
            impl ::core::convert::From<creditReturn> for UnderlyingRustTuple<'_> {
                fn from(value: creditReturn) -> Self {
                    (value.credit_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { credit_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for creditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "credit(address)";
            const SELECTOR: [u8; 4] = [213u8, 212u8, 77u8, 128u8];
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
                        &self._recipient,
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
                        let r: creditReturn = r.into();
                        r.credit_
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
                        let r: creditReturn = r.into();
                        r.credit_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `defend(bytes32,uint256,bytes32)` and selector `0x7b0f0adc`.
```solidity
function defend(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defendCall {
        #[allow(missing_docs)]
        pub _disputed: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _claim: <Claim as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`defend(bytes32,uint256,bytes32)`](defendCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defendReturn {}
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
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<defendCall> for UnderlyingRustTuple<'_> {
                fn from(value: defendCall) -> Self {
                    (value._disputed, value._parentIndex, value._claim)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defendCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _disputed: tuple.0,
                        _parentIndex: tuple.1,
                        _claim: tuple.2,
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
            impl ::core::convert::From<defendReturn> for UnderlyingRustTuple<'_> {
                fn from(value: defendReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defendReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl defendReturn {
            fn _tokenize(
                &self,
            ) -> <defendCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defendCall {
            type Parameters<'a> = (Claim, alloy::sol_types::sol_data::Uint<256>, Claim);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defendReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defend(bytes32,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [123u8, 15u8, 10u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._disputed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._parentIndex),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._claim),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                defendReturn::_tokenize(ret)
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
    /**Function with signature `extraData()` and selector `0x609d3334`.
```solidity
function extraData() external pure returns (bytes memory extraData_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct extraDataCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`extraData()`](extraDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct extraDataReturn {
        #[allow(missing_docs)]
        pub extraData_: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<extraDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: extraDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extraDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extraDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: extraDataReturn) -> Self {
                    (value.extraData_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extraDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { extraData_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extraDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extraData()";
            const SELECTOR: [u8; 4] = [96u8, 157u8, 51u8, 52u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
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
                        let r: extraDataReturn = r.into();
                        r.extraData_
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
                        let r: extraDataReturn = r.into();
                        r.extraData_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `gameCreator()` and selector `0x37b1b229`.
```solidity
function gameCreator() external pure returns (address creator_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameCreatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`gameCreator()`](gameCreatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameCreatorReturn {
        #[allow(missing_docs)]
        pub creator_: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<gameCreatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: gameCreatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameCreatorCall {
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
            impl ::core::convert::From<gameCreatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gameCreatorReturn) -> Self {
                    (value.creator_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameCreatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { creator_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gameCreatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gameCreator()";
            const SELECTOR: [u8; 4] = [55u8, 177u8, 178u8, 41u8];
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
                        let r: gameCreatorReturn = r.into();
                        r.creator_
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
                        let r: gameCreatorReturn = r.into();
                        r.creator_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `gameData()` and selector `0xfa24f743`.
```solidity
function gameData() external view returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameDataCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`gameData()`](gameDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameDataReturn {
        #[allow(missing_docs)]
        pub gameType_: <GameType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub rootClaim_: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub extraData_: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<gameDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: gameDataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                GameType,
                Claim,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameType as alloy::sol_types::SolType>::RustType,
                <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<gameDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gameDataReturn) -> Self {
                    (value.gameType_, value.rootClaim_, value.extraData_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        gameType_: tuple.0,
                        rootClaim_: tuple.1,
                        extraData_: tuple.2,
                    }
                }
            }
        }
        impl gameDataReturn {
            fn _tokenize(
                &self,
            ) -> <gameDataCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <GameType as alloy_sol_types::SolType>::tokenize(&self.gameType_),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self.rootClaim_),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData_,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gameDataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = gameDataReturn;
            type ReturnTuple<'a> = (GameType, Claim, alloy::sol_types::sol_data::Bytes);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gameData()";
            const SELECTOR: [u8; 4] = [250u8, 36u8, 247u8, 67u8];
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
                gameDataReturn::_tokenize(ret)
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
    /**Function with signature `gameType()` and selector `0xbbdc02db`.
```solidity
function gameType() external view returns (GameType gameType_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameTypeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`gameType()`](gameTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gameTypeReturn {
        #[allow(missing_docs)]
        pub gameType_: <GameType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<gameTypeCall> for UnderlyingRustTuple<'_> {
                fn from(value: gameTypeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<gameTypeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gameTypeReturn) -> Self {
                    (value.gameType_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gameTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { gameType_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gameTypeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GameType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GameType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gameType()";
            const SELECTOR: [u8; 4] = [187u8, 220u8, 2u8, 219u8];
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
                (<GameType as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: gameTypeReturn = r.into();
                        r.gameType_
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
                        let r: gameTypeReturn = r.into();
                        r.gameType_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getChallengerDuration(uint256)` and selector `0xbd8da956`.
```solidity
function getChallengerDuration(uint256 _claimIndex) external view returns (Duration duration_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChallengerDurationCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getChallengerDuration(uint256)`](getChallengerDurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChallengerDurationReturn {
        #[allow(missing_docs)]
        pub duration_: <Duration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getChallengerDurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getChallengerDurationCall) -> Self {
                    (value._claimIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getChallengerDurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _claimIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Duration,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Duration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getChallengerDurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getChallengerDurationReturn) -> Self {
                    (value.duration_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getChallengerDurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { duration_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getChallengerDurationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Duration as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Duration,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getChallengerDuration(uint256)";
            const SELECTOR: [u8; 4] = [189u8, 141u8, 169u8, 86u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Duration as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getChallengerDurationReturn = r.into();
                        r.duration_
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
                        let r: getChallengerDurationReturn = r.into();
                        r.duration_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getNumToResolve(uint256)` and selector `0x5a5fa2d9`.
```solidity
function getNumToResolve(uint256 _claimIndex) external view returns (uint256 numRemainingChildren_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumToResolveCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getNumToResolve(uint256)`](getNumToResolveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumToResolveReturn {
        #[allow(missing_docs)]
        pub numRemainingChildren_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getNumToResolveCall> for UnderlyingRustTuple<'_> {
                fn from(value: getNumToResolveCall) -> Self {
                    (value._claimIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNumToResolveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _claimIndex: tuple.0 }
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
            impl ::core::convert::From<getNumToResolveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getNumToResolveReturn) -> Self {
                    (value.numRemainingChildren_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getNumToResolveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        numRemainingChildren_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNumToResolveCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getNumToResolve(uint256)";
            const SELECTOR: [u8; 4] = [90u8, 95u8, 162u8, 217u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
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
                        let r: getNumToResolveReturn = r.into();
                        r.numRemainingChildren_
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
                        let r: getNumToResolveReturn = r.into();
                        r.numRemainingChildren_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRequiredBond(uint128)` and selector `0xc395e1ca`.
```solidity
function getRequiredBond(Position _position) external view returns (uint256 requiredBond_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequiredBondCall {
        #[allow(missing_docs)]
        pub _position: <Position as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRequiredBond(uint128)`](getRequiredBondCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequiredBondReturn {
        #[allow(missing_docs)]
        pub requiredBond_: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (Position,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Position as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRequiredBondCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRequiredBondCall) -> Self {
                    (value._position,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRequiredBondCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _position: tuple.0 }
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
            impl ::core::convert::From<getRequiredBondReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRequiredBondReturn) -> Self {
                    (value.requiredBond_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRequiredBondReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requiredBond_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRequiredBondCall {
            type Parameters<'a> = (Position,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRequiredBond(uint128)";
            const SELECTOR: [u8; 4] = [195u8, 149u8, 225u8, 202u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Position as alloy_sol_types::SolType>::tokenize(&self._position),)
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
                        let r: getRequiredBondReturn = r.into();
                        r.requiredBond_
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
                        let r: getRequiredBondReturn = r.into();
                        r.requiredBond_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `hasUnlockedCredit(address)` and selector `0x222abf45`.
```solidity
function hasUnlockedCredit(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasUnlockedCreditCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasUnlockedCredit(address)`](hasUnlockedCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasUnlockedCreditReturn {
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
            impl ::core::convert::From<hasUnlockedCreditCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: hasUnlockedCreditCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hasUnlockedCreditCall {
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
            impl ::core::convert::From<hasUnlockedCreditReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hasUnlockedCreditReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hasUnlockedCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasUnlockedCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasUnlockedCredit(address)";
            const SELECTOR: [u8; 4] = [34u8, 42u8, 191u8, 69u8];
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
                        let r: hasUnlockedCreditReturn = r.into();
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
                        let r: hasUnlockedCreditReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize()` and selector `0x8129fc1c`.
```solidity
function initialize() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall;
    ///Container type for the return parameters of the [`initialize()`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
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
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize()";
            const SELECTOR: [u8; 4] = [129u8, 41u8, 252u8, 28u8];
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
    /**Function with signature `l1Head()` and selector `0x6361506d`.
```solidity
function l1Head() external pure returns (Hash l1Head_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1HeadCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l1Head()`](l1HeadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l1HeadReturn {
        #[allow(missing_docs)]
        pub l1Head_: <Hash as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<l1HeadCall> for UnderlyingRustTuple<'_> {
                fn from(value: l1HeadCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l1HeadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Hash,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<l1HeadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l1HeadReturn) -> Self {
                    (value.l1Head_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l1HeadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l1Head_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l1HeadCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Hash as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Hash,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l1Head()";
            const SELECTOR: [u8; 4] = [99u8, 97u8, 80u8, 109u8];
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
                (<Hash as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: l1HeadReturn = r.into();
                        r.l1Head_
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
                        let r: l1HeadReturn = r.into();
                        r.l1Head_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l2BlockNumber()` and selector `0x8b85902b`.
```solidity
function l2BlockNumber() external pure returns (uint256 l2BlockNumber_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2BlockNumber()`](l2BlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberReturn {
        #[allow(missing_docs)]
        pub l2BlockNumber_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<l2BlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2BlockNumberCall {
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
            impl ::core::convert::From<l2BlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberReturn) -> Self {
                    (value.l2BlockNumber_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2BlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l2BlockNumber_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2BlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2BlockNumber()";
            const SELECTOR: [u8; 4] = [139u8, 133u8, 144u8, 43u8];
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
                        let r: l2BlockNumberReturn = r.into();
                        r.l2BlockNumber_
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
                        let r: l2BlockNumberReturn = r.into();
                        r.l2BlockNumber_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l2BlockNumberChallenged()` and selector `0x3e3ac912`.
```solidity
function l2BlockNumberChallenged() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2BlockNumberChallenged()`](l2BlockNumberChallengedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengedReturn {
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
            impl ::core::convert::From<l2BlockNumberChallengedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengedCall {
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
            impl ::core::convert::From<l2BlockNumberChallengedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2BlockNumberChallengedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2BlockNumberChallenged()";
            const SELECTOR: [u8; 4] = [62u8, 58u8, 201u8, 18u8];
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
                        let r: l2BlockNumberChallengedReturn = r.into();
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
                        let r: l2BlockNumberChallengedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l2BlockNumberChallenger()` and selector `0x30dbe570`.
```solidity
function l2BlockNumberChallenger() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2BlockNumberChallenger()`](l2BlockNumberChallengerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2BlockNumberChallengerReturn {
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
            impl ::core::convert::From<l2BlockNumberChallengerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengerCall {
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
            impl ::core::convert::From<l2BlockNumberChallengerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2BlockNumberChallengerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2BlockNumberChallengerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2BlockNumberChallengerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2BlockNumberChallenger()";
            const SELECTOR: [u8; 4] = [48u8, 219u8, 229u8, 112u8];
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
                        let r: l2BlockNumberChallengerReturn = r.into();
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
                        let r: l2BlockNumberChallengerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l2ChainId()` and selector `0xd6ae3cd5`.
```solidity
function l2ChainId() external view returns (uint256 l2ChainId_);
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
        pub l2ChainId_: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.l2ChainId_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2ChainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l2ChainId_: tuple.0 }
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
                        r.l2ChainId_
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
                        r.l2ChainId_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `l2SequenceNumber()` and selector `0x99735e32`.
```solidity
function l2SequenceNumber() external pure returns (uint256 l2SequenceNumber_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2SequenceNumberCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2SequenceNumber()`](l2SequenceNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2SequenceNumberReturn {
        #[allow(missing_docs)]
        pub l2SequenceNumber_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<l2SequenceNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2SequenceNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2SequenceNumberCall {
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
            impl ::core::convert::From<l2SequenceNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: l2SequenceNumberReturn) -> Self {
                    (value.l2SequenceNumber_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for l2SequenceNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l2SequenceNumber_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2SequenceNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2SequenceNumber()";
            const SELECTOR: [u8; 4] = [153u8, 115u8, 94u8, 50u8];
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
                        let r: l2SequenceNumberReturn = r.into();
                        r.l2SequenceNumber_
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
                        let r: l2SequenceNumberReturn = r.into();
                        r.l2SequenceNumber_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `maxClockDuration()` and selector `0xdabd396d`.
```solidity
function maxClockDuration() external view returns (Duration maxClockDuration_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxClockDurationCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`maxClockDuration()`](maxClockDurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxClockDurationReturn {
        #[allow(missing_docs)]
        pub maxClockDuration_: <Duration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<maxClockDurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxClockDurationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxClockDurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Duration,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Duration as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<maxClockDurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxClockDurationReturn) -> Self {
                    (value.maxClockDuration_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxClockDurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { maxClockDuration_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxClockDurationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Duration as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Duration,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxClockDuration()";
            const SELECTOR: [u8; 4] = [218u8, 189u8, 57u8, 109u8];
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
                (<Duration as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: maxClockDurationReturn = r.into();
                        r.maxClockDuration_
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
                        let r: maxClockDurationReturn = r.into();
                        r.maxClockDuration_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `maxGameDepth()` and selector `0xfa315aa9`.
```solidity
function maxGameDepth() external view returns (uint256 maxGameDepth_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxGameDepthCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`maxGameDepth()`](maxGameDepthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxGameDepthReturn {
        #[allow(missing_docs)]
        pub maxGameDepth_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<maxGameDepthCall> for UnderlyingRustTuple<'_> {
                fn from(value: maxGameDepthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxGameDepthCall {
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
            impl ::core::convert::From<maxGameDepthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: maxGameDepthReturn) -> Self {
                    (value.maxGameDepth_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxGameDepthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { maxGameDepth_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxGameDepthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxGameDepth()";
            const SELECTOR: [u8; 4] = [250u8, 49u8, 90u8, 169u8];
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
                        let r: maxGameDepthReturn = r.into();
                        r.maxGameDepth_
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
                        let r: maxGameDepthReturn = r.into();
                        r.maxGameDepth_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `move(bytes32,uint256,bytes32,bool)` and selector `0x6f034409`.
```solidity
function r#move(Claim _disputed, uint256 _challengeIndex, Claim _claim, bool _isAttack) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct moveCall {
        #[allow(missing_docs)]
        pub _disputed: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _challengeIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _claim: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _isAttack: bool,
    }
    ///Container type for the return parameters of the [`move(bytes32,uint256,bytes32,bool)`](moveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct moveReturn {}
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
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<moveCall> for UnderlyingRustTuple<'_> {
                fn from(value: moveCall) -> Self {
                    (
                        value._disputed,
                        value._challengeIndex,
                        value._claim,
                        value._isAttack,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for moveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _disputed: tuple.0,
                        _challengeIndex: tuple.1,
                        _claim: tuple.2,
                        _isAttack: tuple.3,
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
            impl ::core::convert::From<moveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: moveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for moveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl moveReturn {
            fn _tokenize(
                &self,
            ) -> <moveCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for moveCall {
            type Parameters<'a> = (
                Claim,
                alloy::sol_types::sol_data::Uint<256>,
                Claim,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = moveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "move(bytes32,uint256,bytes32,bool)";
            const SELECTOR: [u8; 4] = [111u8, 3u8, 68u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._disputed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._challengeIndex),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self._claim),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._isAttack,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                moveReturn::_tokenize(ret)
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
    /**Function with signature `normalModeCredit(address)` and selector `0x529d6a8c`.
```solidity
function normalModeCredit(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct normalModeCreditCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`normalModeCredit(address)`](normalModeCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct normalModeCreditReturn {
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
            impl ::core::convert::From<normalModeCreditCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: normalModeCreditCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for normalModeCreditCall {
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
            impl ::core::convert::From<normalModeCreditReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: normalModeCreditReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for normalModeCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for normalModeCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "normalModeCredit(address)";
            const SELECTOR: [u8; 4] = [82u8, 157u8, 106u8, 140u8];
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
                        let r: normalModeCreditReturn = r.into();
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
                        let r: normalModeCreditReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `refundModeCredit(address)` and selector `0xc0d8bb74`.
```solidity
function refundModeCredit(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct refundModeCreditCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`refundModeCredit(address)`](refundModeCreditCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct refundModeCreditReturn {
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
            impl ::core::convert::From<refundModeCreditCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: refundModeCreditCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for refundModeCreditCall {
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
            impl ::core::convert::From<refundModeCreditReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: refundModeCreditReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for refundModeCreditReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for refundModeCreditCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "refundModeCredit(address)";
            const SELECTOR: [u8; 4] = [192u8, 216u8, 187u8, 116u8];
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
                        let r: refundModeCreditReturn = r.into();
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
                        let r: refundModeCreditReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resolutionCheckpoints(uint256)` and selector `0xa445ece6`.
```solidity
function resolutionCheckpoints(uint256) external view returns (bool initialCheckpointComplete, uint32 subgameIndex, Position leftmostPosition, address counteredBy);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolutionCheckpointsCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`resolutionCheckpoints(uint256)`](resolutionCheckpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolutionCheckpointsReturn {
        #[allow(missing_docs)]
        pub initialCheckpointComplete: bool,
        #[allow(missing_docs)]
        pub subgameIndex: u32,
        #[allow(missing_docs)]
        pub leftmostPosition: <Position as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub counteredBy: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<resolutionCheckpointsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolutionCheckpointsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolutionCheckpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
                Position,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                u32,
                <Position as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<resolutionCheckpointsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolutionCheckpointsReturn) -> Self {
                    (
                        value.initialCheckpointComplete,
                        value.subgameIndex,
                        value.leftmostPosition,
                        value.counteredBy,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolutionCheckpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialCheckpointComplete: tuple.0,
                        subgameIndex: tuple.1,
                        leftmostPosition: tuple.2,
                        counteredBy: tuple.3,
                    }
                }
            }
        }
        impl resolutionCheckpointsReturn {
            fn _tokenize(
                &self,
            ) -> <resolutionCheckpointsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.initialCheckpointComplete,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.subgameIndex),
                    <Position as alloy_sol_types::SolType>::tokenize(
                        &self.leftmostPosition,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.counteredBy,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolutionCheckpointsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resolutionCheckpointsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
                Position,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolutionCheckpoints(uint256)";
            const SELECTOR: [u8; 4] = [164u8, 69u8, 236u8, 230u8];
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
                resolutionCheckpointsReturn::_tokenize(ret)
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
    /**Function with signature `resolve()` and selector `0x2810e1d6`.
```solidity
function resolve() external returns (GameStatus status_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`resolve()`](resolveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveReturn {
        #[allow(missing_docs)]
        pub status_: <GameStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<resolveCall> for UnderlyingRustTuple<'_> {
                fn from(value: resolveCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<resolveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: resolveReturn) -> Self {
                    (value.status_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { status_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolveCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GameStatus as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GameStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolve()";
            const SELECTOR: [u8; 4] = [40u8, 16u8, 225u8, 214u8];
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
                (<GameStatus as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: resolveReturn = r.into();
                        r.status_
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
                        let r: resolveReturn = r.into();
                        r.status_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resolveClaim(uint256,uint256)` and selector `0x03c2924d`.
```solidity
function resolveClaim(uint256 _claimIndex, uint256 _numToResolve) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveClaimCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _numToResolve: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`resolveClaim(uint256,uint256)`](resolveClaimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveClaimReturn {}
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
            impl ::core::convert::From<resolveClaimCall> for UnderlyingRustTuple<'_> {
                fn from(value: resolveClaimCall) -> Self {
                    (value._claimIndex, value._numToResolve)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveClaimCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimIndex: tuple.0,
                        _numToResolve: tuple.1,
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
            impl ::core::convert::From<resolveClaimReturn> for UnderlyingRustTuple<'_> {
                fn from(value: resolveClaimReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveClaimReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl resolveClaimReturn {
            fn _tokenize(
                &self,
            ) -> <resolveClaimCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolveClaimCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resolveClaimReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolveClaim(uint256,uint256)";
            const SELECTOR: [u8; 4] = [3u8, 194u8, 146u8, 77u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._numToResolve),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                resolveClaimReturn::_tokenize(ret)
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
    /**Function with signature `resolvedAt()` and selector `0x19effeb4`.
```solidity
function resolvedAt() external view returns (Timestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedAtCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`resolvedAt()`](resolvedAtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedAtReturn {
        #[allow(missing_docs)]
        pub _0: <Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<resolvedAtCall> for UnderlyingRustTuple<'_> {
                fn from(value: resolvedAtCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolvedAtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Timestamp,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<resolvedAtReturn> for UnderlyingRustTuple<'_> {
                fn from(value: resolvedAtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolvedAtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolvedAtCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Timestamp as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Timestamp,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolvedAt()";
            const SELECTOR: [u8; 4] = [25u8, 239u8, 254u8, 180u8];
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
                (<Timestamp as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: resolvedAtReturn = r.into();
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
                        let r: resolvedAtReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resolvedSubgames(uint256)` and selector `0xfe2bbeb2`.
```solidity
function resolvedSubgames(uint256) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedSubgamesCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`resolvedSubgames(uint256)`](resolvedSubgamesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolvedSubgamesReturn {
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
            impl ::core::convert::From<resolvedSubgamesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolvedSubgamesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolvedSubgamesCall {
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
            impl ::core::convert::From<resolvedSubgamesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolvedSubgamesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolvedSubgamesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolvedSubgamesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolvedSubgames(uint256)";
            const SELECTOR: [u8; 4] = [254u8, 43u8, 190u8, 178u8];
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
                        let r: resolvedSubgamesReturn = r.into();
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
                        let r: resolvedSubgamesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `rootClaim()` and selector `0xbcef3b55`.
```solidity
function rootClaim() external pure returns (Claim rootClaim_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootClaimCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`rootClaim()`](rootClaimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rootClaimReturn {
        #[allow(missing_docs)]
        pub rootClaim_: <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<rootClaimCall> for UnderlyingRustTuple<'_> {
                fn from(value: rootClaimCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootClaimCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Claim,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Claim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<rootClaimReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rootClaimReturn) -> Self {
                    (value.rootClaim_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rootClaimReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { rootClaim_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rootClaimCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Claim as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Claim,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rootClaim()";
            const SELECTOR: [u8; 4] = [188u8, 239u8, 59u8, 85u8];
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
                (<Claim as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: rootClaimReturn = r.into();
                        r.rootClaim_
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
                        let r: rootClaimReturn = r.into();
                        r.rootClaim_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `splitDepth()` and selector `0xec5e6308`.
```solidity
function splitDepth() external view returns (uint256 splitDepth_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct splitDepthCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`splitDepth()`](splitDepthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct splitDepthReturn {
        #[allow(missing_docs)]
        pub splitDepth_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<splitDepthCall> for UnderlyingRustTuple<'_> {
                fn from(value: splitDepthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for splitDepthCall {
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
            impl ::core::convert::From<splitDepthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: splitDepthReturn) -> Self {
                    (value.splitDepth_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for splitDepthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { splitDepth_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for splitDepthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "splitDepth()";
            const SELECTOR: [u8; 4] = [236u8, 94u8, 99u8, 8u8];
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
                        let r: splitDepthReturn = r.into();
                        r.splitDepth_
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
                        let r: splitDepthReturn = r.into();
                        r.splitDepth_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `startingBlockNumber()` and selector `0x70872aa5`.
```solidity
function startingBlockNumber() external view returns (uint256 startingBlockNumber_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingBlockNumberCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`startingBlockNumber()`](startingBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingBlockNumberReturn {
        #[allow(missing_docs)]
        pub startingBlockNumber_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<startingBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingBlockNumberCall {
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
            impl ::core::convert::From<startingBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingBlockNumberReturn) -> Self {
                    (value.startingBlockNumber_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startingBlockNumber_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingBlockNumber()";
            const SELECTOR: [u8; 4] = [112u8, 135u8, 42u8, 165u8];
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
                        let r: startingBlockNumberReturn = r.into();
                        r.startingBlockNumber_
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
                        let r: startingBlockNumberReturn = r.into();
                        r.startingBlockNumber_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `startingOutputRoot()` and selector `0x57da950e`.
```solidity
function startingOutputRoot() external view returns (Hash root, uint256 l2SequenceNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingOutputRootCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`startingOutputRoot()`](startingOutputRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingOutputRootReturn {
        #[allow(missing_docs)]
        pub root: <Hash as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub l2SequenceNumber: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<startingOutputRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingOutputRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingOutputRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Hash, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<startingOutputRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingOutputRootReturn) -> Self {
                    (value.root, value.l2SequenceNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingOutputRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        root: tuple.0,
                        l2SequenceNumber: tuple.1,
                    }
                }
            }
        }
        impl startingOutputRootReturn {
            fn _tokenize(
                &self,
            ) -> <startingOutputRootCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <Hash as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2SequenceNumber),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingOutputRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startingOutputRootReturn;
            type ReturnTuple<'a> = (Hash, alloy::sol_types::sol_data::Uint<256>);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingOutputRoot()";
            const SELECTOR: [u8; 4] = [87u8, 218u8, 149u8, 14u8];
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
                startingOutputRootReturn::_tokenize(ret)
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
    /**Function with signature `startingRootHash()` and selector `0x25fc2ace`.
```solidity
function startingRootHash() external view returns (Hash startingRootHash_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingRootHashCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`startingRootHash()`](startingRootHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingRootHashReturn {
        #[allow(missing_docs)]
        pub startingRootHash_: <Hash as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<startingRootHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingRootHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingRootHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Hash,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Hash as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<startingRootHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingRootHashReturn) -> Self {
                    (value.startingRootHash_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingRootHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { startingRootHash_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingRootHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Hash as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Hash,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingRootHash()";
            const SELECTOR: [u8; 4] = [37u8, 252u8, 42u8, 206u8];
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
                (<Hash as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: startingRootHashReturn = r.into();
                        r.startingRootHash_
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
                        let r: startingRootHashReturn = r.into();
                        r.startingRootHash_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `status()` and selector `0x200d2ed2`.
```solidity
function status() external view returns (GameStatus);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct statusCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`status()`](statusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct statusReturn {
        #[allow(missing_docs)]
        pub _0: <GameStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<statusCall> for UnderlyingRustTuple<'_> {
                fn from(value: statusCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for statusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GameStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GameStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<statusReturn> for UnderlyingRustTuple<'_> {
                fn from(value: statusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for statusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for statusCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GameStatus as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GameStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "status()";
            const SELECTOR: [u8; 4] = [32u8, 13u8, 46u8, 210u8];
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
                (<GameStatus as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: statusReturn = r.into();
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
                        let r: statusReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `step(uint256,bool,bytes,bytes)` and selector `0xd8cc1a3c`.
```solidity
function step(uint256 _claimIndex, bool _isAttack, bytes memory _stateData, bytes memory _proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stepCall {
        #[allow(missing_docs)]
        pub _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _isAttack: bool,
        #[allow(missing_docs)]
        pub _stateData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _proof: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`step(uint256,bool,bytes,bytes)`](stepCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stepReturn {}
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
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<stepCall> for UnderlyingRustTuple<'_> {
                fn from(value: stepCall) -> Self {
                    (value._claimIndex, value._isAttack, value._stateData, value._proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stepCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _claimIndex: tuple.0,
                        _isAttack: tuple.1,
                        _stateData: tuple.2,
                        _proof: tuple.3,
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
            impl ::core::convert::From<stepReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stepReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stepReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl stepReturn {
            fn _tokenize(
                &self,
            ) -> <stepCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stepCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stepReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "step(uint256,bool,bytes,bytes)";
            const SELECTOR: [u8; 4] = [216u8, 204u8, 26u8, 60u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._claimIndex),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._isAttack,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._stateData,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._proof,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                stepReturn::_tokenize(ret)
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
    /**Function with signature `subgames(uint256,uint256)` and selector `0x2ad69aeb`.
```solidity
function subgames(uint256, uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subgamesCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`subgames(uint256,uint256)`](subgamesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subgamesReturn {
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
            impl ::core::convert::From<subgamesCall> for UnderlyingRustTuple<'_> {
                fn from(value: subgamesCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for subgamesCall {
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
            impl ::core::convert::From<subgamesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: subgamesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for subgamesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for subgamesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
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
            const SIGNATURE: &'static str = "subgames(uint256,uint256)";
            const SELECTOR: [u8; 4] = [42u8, 214u8, 154u8, 235u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
                        let r: subgamesReturn = r.into();
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
                        let r: subgamesReturn = r.into();
                        r._0
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `vm()` and selector `0x3a768463`.
```solidity
function vm() external view returns (address vm_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct vmCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`vm()`](vmCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct vmReturn {
        #[allow(missing_docs)]
        pub vm_: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<vmCall> for UnderlyingRustTuple<'_> {
                fn from(value: vmCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for vmCall {
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
            impl ::core::convert::From<vmReturn> for UnderlyingRustTuple<'_> {
                fn from(value: vmReturn) -> Self {
                    (value.vm_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for vmReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { vm_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for vmCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "vm()";
            const SELECTOR: [u8; 4] = [58u8, 118u8, 132u8, 99u8];
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
                        let r: vmReturn = r.into();
                        r.vm_
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
                        let r: vmReturn = r.into();
                        r.vm_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `wasRespectedGameTypeWhenCreated()` and selector `0x250e69bd`.
```solidity
function wasRespectedGameTypeWhenCreated() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wasRespectedGameTypeWhenCreatedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`wasRespectedGameTypeWhenCreated()`](wasRespectedGameTypeWhenCreatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wasRespectedGameTypeWhenCreatedReturn {
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
            impl ::core::convert::From<wasRespectedGameTypeWhenCreatedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: wasRespectedGameTypeWhenCreatedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for wasRespectedGameTypeWhenCreatedCall {
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
            impl ::core::convert::From<wasRespectedGameTypeWhenCreatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: wasRespectedGameTypeWhenCreatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for wasRespectedGameTypeWhenCreatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wasRespectedGameTypeWhenCreatedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "wasRespectedGameTypeWhenCreated()";
            const SELECTOR: [u8; 4] = [37u8, 14u8, 105u8, 189u8];
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
                        let r: wasRespectedGameTypeWhenCreatedReturn = r.into();
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
                        let r: wasRespectedGameTypeWhenCreatedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `weth()` and selector `0x3fc8cef3`.
```solidity
function weth() external view returns (address weth_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`weth()`](wethCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethReturn {
        #[allow(missing_docs)]
        pub weth_: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<wethCall> for UnderlyingRustTuple<'_> {
                fn from(value: wethCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethCall {
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
            impl ::core::convert::From<wethReturn> for UnderlyingRustTuple<'_> {
                fn from(value: wethReturn) -> Self {
                    (value.weth_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { weth_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wethCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "weth()";
            const SELECTOR: [u8; 4] = [63u8, 200u8, 206u8, 243u8];
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
                        let r: wethReturn = r.into();
                        r.weth_
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
                        let r: wethReturn = r.into();
                        r.weth_
                    })
            }
        }
    };
    ///Container for all the [`FaultDisputeGame`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum FaultDisputeGameCalls {
        #[allow(missing_docs)]
        absolutePrestate(absolutePrestateCall),
        #[allow(missing_docs)]
        addLocalData(addLocalDataCall),
        #[allow(missing_docs)]
        anchorStateRegistry(anchorStateRegistryCall),
        #[allow(missing_docs)]
        attack(attackCall),
        #[allow(missing_docs)]
        bondDistributionMode(bondDistributionModeCall),
        #[allow(missing_docs)]
        challengeRootL2Block(challengeRootL2BlockCall),
        #[allow(missing_docs)]
        claimCredit(claimCreditCall),
        #[allow(missing_docs)]
        claimData(claimDataCall),
        #[allow(missing_docs)]
        claimDataLen(claimDataLenCall),
        #[allow(missing_docs)]
        claims(claimsCall),
        #[allow(missing_docs)]
        clockExtension(clockExtensionCall),
        #[allow(missing_docs)]
        closeGame(closeGameCall),
        #[allow(missing_docs)]
        createdAt(createdAtCall),
        #[allow(missing_docs)]
        credit(creditCall),
        #[allow(missing_docs)]
        defend(defendCall),
        #[allow(missing_docs)]
        extraData(extraDataCall),
        #[allow(missing_docs)]
        gameCreator(gameCreatorCall),
        #[allow(missing_docs)]
        gameData(gameDataCall),
        #[allow(missing_docs)]
        gameType(gameTypeCall),
        #[allow(missing_docs)]
        getChallengerDuration(getChallengerDurationCall),
        #[allow(missing_docs)]
        getNumToResolve(getNumToResolveCall),
        #[allow(missing_docs)]
        getRequiredBond(getRequiredBondCall),
        #[allow(missing_docs)]
        hasUnlockedCredit(hasUnlockedCreditCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        l1Head(l1HeadCall),
        #[allow(missing_docs)]
        l2BlockNumber(l2BlockNumberCall),
        #[allow(missing_docs)]
        l2BlockNumberChallenged(l2BlockNumberChallengedCall),
        #[allow(missing_docs)]
        l2BlockNumberChallenger(l2BlockNumberChallengerCall),
        #[allow(missing_docs)]
        l2ChainId(l2ChainIdCall),
        #[allow(missing_docs)]
        l2SequenceNumber(l2SequenceNumberCall),
        #[allow(missing_docs)]
        maxClockDuration(maxClockDurationCall),
        #[allow(missing_docs)]
        maxGameDepth(maxGameDepthCall),
        #[allow(missing_docs)]
        r#move(moveCall),
        #[allow(missing_docs)]
        normalModeCredit(normalModeCreditCall),
        #[allow(missing_docs)]
        refundModeCredit(refundModeCreditCall),
        #[allow(missing_docs)]
        resolutionCheckpoints(resolutionCheckpointsCall),
        #[allow(missing_docs)]
        resolve(resolveCall),
        #[allow(missing_docs)]
        resolveClaim(resolveClaimCall),
        #[allow(missing_docs)]
        resolvedAt(resolvedAtCall),
        #[allow(missing_docs)]
        resolvedSubgames(resolvedSubgamesCall),
        #[allow(missing_docs)]
        rootClaim(rootClaimCall),
        #[allow(missing_docs)]
        splitDepth(splitDepthCall),
        #[allow(missing_docs)]
        startingBlockNumber(startingBlockNumberCall),
        #[allow(missing_docs)]
        startingOutputRoot(startingOutputRootCall),
        #[allow(missing_docs)]
        startingRootHash(startingRootHashCall),
        #[allow(missing_docs)]
        status(statusCall),
        #[allow(missing_docs)]
        step(stepCall),
        #[allow(missing_docs)]
        subgames(subgamesCall),
        #[allow(missing_docs)]
        version(versionCall),
        #[allow(missing_docs)]
        vm(vmCall),
        #[allow(missing_docs)]
        wasRespectedGameTypeWhenCreated(wasRespectedGameTypeWhenCreatedCall),
        #[allow(missing_docs)]
        weth(wethCall),
    }
    impl FaultDisputeGameCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 147u8, 81u8, 48u8],
            [3u8, 194u8, 146u8, 77u8],
            [25u8, 239u8, 254u8, 180u8],
            [32u8, 13u8, 46u8, 210u8],
            [34u8, 42u8, 191u8, 69u8],
            [37u8, 14u8, 105u8, 189u8],
            [37u8, 252u8, 42u8, 206u8],
            [40u8, 16u8, 225u8, 214u8],
            [42u8, 214u8, 154u8, 235u8],
            [48u8, 219u8, 229u8, 112u8],
            [55u8, 141u8, 212u8, 140u8],
            [55u8, 177u8, 178u8, 41u8],
            [58u8, 118u8, 132u8, 99u8],
            [62u8, 58u8, 201u8, 18u8],
            [63u8, 200u8, 206u8, 243u8],
            [71u8, 39u8, 119u8, 198u8],
            [82u8, 157u8, 106u8, 140u8],
            [84u8, 253u8, 77u8, 80u8],
            [87u8, 218u8, 149u8, 14u8],
            [90u8, 95u8, 162u8, 217u8],
            [92u8, 12u8, 186u8, 51u8],
            [96u8, 157u8, 51u8, 52u8],
            [96u8, 226u8, 116u8, 100u8],
            [99u8, 97u8, 80u8, 109u8],
            [107u8, 103u8, 22u8, 192u8],
            [111u8, 3u8, 68u8, 9u8],
            [112u8, 135u8, 42u8, 165u8],
            [120u8, 107u8, 132u8, 75u8],
            [123u8, 15u8, 10u8, 220u8],
            [129u8, 41u8, 252u8, 28u8],
            [137u8, 128u8, 224u8, 204u8],
            [139u8, 133u8, 144u8, 43u8],
            [141u8, 69u8, 10u8, 149u8],
            [153u8, 115u8, 94u8, 50u8],
            [164u8, 69u8, 236u8, 230u8],
            [187u8, 220u8, 2u8, 219u8],
            [188u8, 239u8, 59u8, 85u8],
            [189u8, 141u8, 169u8, 86u8],
            [192u8, 216u8, 187u8, 116u8],
            [195u8, 149u8, 225u8, 202u8],
            [198u8, 240u8, 48u8, 140u8],
            [207u8, 9u8, 224u8, 208u8],
            [213u8, 212u8, 77u8, 128u8],
            [214u8, 174u8, 60u8, 213u8],
            [216u8, 204u8, 26u8, 60u8],
            [218u8, 189u8, 57u8, 109u8],
            [236u8, 94u8, 99u8, 8u8],
            [239u8, 240u8, 245u8, 146u8],
            [248u8, 244u8, 63u8, 246u8],
            [250u8, 36u8, 247u8, 67u8],
            [250u8, 49u8, 90u8, 169u8],
            [254u8, 43u8, 190u8, 178u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(challengeRootL2Block),
            ::core::stringify!(resolveClaim),
            ::core::stringify!(resolvedAt),
            ::core::stringify!(status),
            ::core::stringify!(hasUnlockedCredit),
            ::core::stringify!(wasRespectedGameTypeWhenCreated),
            ::core::stringify!(startingRootHash),
            ::core::stringify!(resolve),
            ::core::stringify!(subgames),
            ::core::stringify!(l2BlockNumberChallenger),
            ::core::stringify!(bondDistributionMode),
            ::core::stringify!(gameCreator),
            ::core::stringify!(vm),
            ::core::stringify!(l2BlockNumberChallenged),
            ::core::stringify!(weth),
            ::core::stringify!(attack),
            ::core::stringify!(normalModeCredit),
            ::core::stringify!(version),
            ::core::stringify!(startingOutputRoot),
            ::core::stringify!(getNumToResolve),
            ::core::stringify!(anchorStateRegistry),
            ::core::stringify!(extraData),
            ::core::stringify!(claimCredit),
            ::core::stringify!(l1Head),
            ::core::stringify!(clockExtension),
            ::core::stringify!(r#move),
            ::core::stringify!(startingBlockNumber),
            ::core::stringify!(closeGame),
            ::core::stringify!(defend),
            ::core::stringify!(initialize),
            ::core::stringify!(claimDataLen),
            ::core::stringify!(l2BlockNumber),
            ::core::stringify!(absolutePrestate),
            ::core::stringify!(l2SequenceNumber),
            ::core::stringify!(resolutionCheckpoints),
            ::core::stringify!(gameType),
            ::core::stringify!(rootClaim),
            ::core::stringify!(getChallengerDuration),
            ::core::stringify!(refundModeCredit),
            ::core::stringify!(getRequiredBond),
            ::core::stringify!(claimData),
            ::core::stringify!(createdAt),
            ::core::stringify!(credit),
            ::core::stringify!(l2ChainId),
            ::core::stringify!(step),
            ::core::stringify!(maxClockDuration),
            ::core::stringify!(splitDepth),
            ::core::stringify!(claims),
            ::core::stringify!(addLocalData),
            ::core::stringify!(gameData),
            ::core::stringify!(maxGameDepth),
            ::core::stringify!(resolvedSubgames),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <challengeRootL2BlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolveClaimCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolvedAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <statusCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasUnlockedCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingRootHashCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <subgamesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bondDistributionModeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameCreatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <vmCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <wethCall as alloy_sol_types::SolCall>::SIGNATURE,
            <attackCall as alloy_sol_types::SolCall>::SIGNATURE,
            <normalModeCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingOutputRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getNumToResolveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <anchorStateRegistryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <extraDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l1HeadCall as alloy_sol_types::SolCall>::SIGNATURE,
            <clockExtensionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <moveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingBlockNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <closeGameCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defendCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimDataLenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2BlockNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <absolutePrestateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2SequenceNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolutionCheckpointsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <rootClaimCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getChallengerDurationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <refundModeCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRequiredBondCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createdAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2ChainIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <stepCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxClockDurationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <splitDepthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addLocalDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxGameDepthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolvedSubgamesCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for FaultDisputeGameCalls {
        const NAME: &'static str = "FaultDisputeGameCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 52usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::absolutePrestate(_) => {
                    <absolutePrestateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addLocalData(_) => {
                    <addLocalDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::anchorStateRegistry(_) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::attack(_) => <attackCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bondDistributionMode(_) => {
                    <bondDistributionModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challengeRootL2Block(_) => {
                    <challengeRootL2BlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimCredit(_) => {
                    <claimCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimData(_) => {
                    <claimDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimDataLen(_) => {
                    <claimDataLenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claims(_) => <claimsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::clockExtension(_) => {
                    <clockExtensionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::closeGame(_) => {
                    <closeGameCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createdAt(_) => {
                    <createdAtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::credit(_) => <creditCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::defend(_) => <defendCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::extraData(_) => {
                    <extraDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gameCreator(_) => {
                    <gameCreatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gameData(_) => <gameDataCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::gameType(_) => <gameTypeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getChallengerDuration(_) => {
                    <getChallengerDurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getNumToResolve(_) => {
                    <getNumToResolveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRequiredBond(_) => {
                    <getRequiredBondCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasUnlockedCredit(_) => {
                    <hasUnlockedCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l1Head(_) => <l1HeadCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::l2BlockNumber(_) => {
                    <l2BlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2BlockNumberChallenged(_) => {
                    <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2BlockNumberChallenger(_) => {
                    <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2ChainId(_) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2SequenceNumber(_) => {
                    <l2SequenceNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxClockDuration(_) => {
                    <maxClockDurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxGameDepth(_) => {
                    <maxGameDepthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::r#move(_) => <moveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::normalModeCredit(_) => {
                    <normalModeCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::refundModeCredit(_) => {
                    <refundModeCreditCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolutionCheckpoints(_) => {
                    <resolutionCheckpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolve(_) => <resolveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::resolveClaim(_) => {
                    <resolveClaimCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolvedAt(_) => {
                    <resolvedAtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolvedSubgames(_) => {
                    <resolvedSubgamesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rootClaim(_) => {
                    <rootClaimCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::splitDepth(_) => {
                    <splitDepthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingBlockNumber(_) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingOutputRoot(_) => {
                    <startingOutputRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingRootHash(_) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::status(_) => <statusCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::step(_) => <stepCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::subgames(_) => <subgamesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::vm(_) => <vmCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::wasRespectedGameTypeWhenCreated(_) => {
                    <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::weth(_) => <wethCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<FaultDisputeGameCalls>] = &[
                {
                    fn challengeRootL2Block(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::challengeRootL2Block)
                    }
                    challengeRootL2Block
                },
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::wasRespectedGameTypeWhenCreated)
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::subgames)
                    }
                    subgames
                },
                {
                    fn l2BlockNumberChallenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2BlockNumberChallenger)
                    }
                    l2BlockNumberChallenger
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::vm)
                    }
                    vm
                },
                {
                    fn l2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2BlockNumberChallenged)
                    }
                    l2BlockNumberChallenged
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::version)
                    }
                    version
                },
                {
                    fn startingOutputRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <startingOutputRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::startingOutputRoot)
                    }
                    startingOutputRoot
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::r#move)
                    }
                    r#move
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn l2BlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2BlockNumber)
                    }
                    l2BlockNumber
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolutionCheckpoints)
                    }
                    resolutionCheckpoints
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::getChallengerDuration)
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::credit)
                    }
                    credit
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FaultDisputeGameCalls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolvedSubgames)
                    }
                    resolvedSubgames
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
            ) -> alloy_sol_types::Result<FaultDisputeGameCalls>] = &[
                {
                    fn challengeRootL2Block(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::challengeRootL2Block)
                    }
                    challengeRootL2Block
                },
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::wasRespectedGameTypeWhenCreated)
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::subgames)
                    }
                    subgames
                },
                {
                    fn l2BlockNumberChallenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2BlockNumberChallenger)
                    }
                    l2BlockNumberChallenger
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::vm)
                    }
                    vm
                },
                {
                    fn l2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2BlockNumberChallenged)
                    }
                    l2BlockNumberChallenged
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::version)
                    }
                    version
                },
                {
                    fn startingOutputRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <startingOutputRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::startingOutputRoot)
                    }
                    startingOutputRoot
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::r#move)
                    }
                    r#move
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn l2BlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2BlockNumber)
                    }
                    l2BlockNumber
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolutionCheckpoints)
                    }
                    resolutionCheckpoints
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::getChallengerDuration)
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::credit)
                    }
                    credit
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameCalls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameCalls::resolvedSubgames)
                    }
                    resolvedSubgames
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
                Self::absolutePrestate(inner) => {
                    <absolutePrestateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addLocalData(inner) => {
                    <addLocalDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::anchorStateRegistry(inner) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::attack(inner) => {
                    <attackCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bondDistributionMode(inner) => {
                    <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challengeRootL2Block(inner) => {
                    <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimCredit(inner) => {
                    <claimCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimData(inner) => {
                    <claimDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::claimDataLen(inner) => {
                    <claimDataLenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claims(inner) => {
                    <claimsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::clockExtension(inner) => {
                    <clockExtensionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::closeGame(inner) => {
                    <closeGameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::createdAt(inner) => {
                    <createdAtCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::credit(inner) => {
                    <creditCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::defend(inner) => {
                    <defendCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extraData(inner) => {
                    <extraDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::gameCreator(inner) => {
                    <gameCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gameData(inner) => {
                    <gameDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::gameType(inner) => {
                    <gameTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getChallengerDuration(inner) => {
                    <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getNumToResolve(inner) => {
                    <getNumToResolveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRequiredBond(inner) => {
                    <getRequiredBondCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hasUnlockedCredit(inner) => {
                    <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::l1Head(inner) => {
                    <l1HeadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::l2BlockNumber(inner) => {
                    <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2BlockNumberChallenged(inner) => {
                    <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2BlockNumberChallenger(inner) => {
                    <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::l2ChainId(inner) => {
                    <l2ChainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::l2SequenceNumber(inner) => {
                    <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxClockDuration(inner) => {
                    <maxClockDurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxGameDepth(inner) => {
                    <maxGameDepthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::r#move(inner) => {
                    <moveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::normalModeCredit(inner) => {
                    <normalModeCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::refundModeCredit(inner) => {
                    <refundModeCreditCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resolutionCheckpoints(inner) => {
                    <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resolve(inner) => {
                    <resolveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::resolveClaim(inner) => {
                    <resolveClaimCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resolvedAt(inner) => {
                    <resolvedAtCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::resolvedSubgames(inner) => {
                    <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rootClaim(inner) => {
                    <rootClaimCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::splitDepth(inner) => {
                    <splitDepthCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startingBlockNumber(inner) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingOutputRoot(inner) => {
                    <startingOutputRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingRootHash(inner) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::status(inner) => {
                    <statusCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::step(inner) => {
                    <stepCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::subgames(inner) => {
                    <subgamesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::vm(inner) => {
                    <vmCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::wasRespectedGameTypeWhenCreated(inner) => {
                    <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::absolutePrestate(inner) => {
                    <absolutePrestateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addLocalData(inner) => {
                    <addLocalDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::anchorStateRegistry(inner) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::attack(inner) => {
                    <attackCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bondDistributionMode(inner) => {
                    <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challengeRootL2Block(inner) => {
                    <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimCredit(inner) => {
                    <claimCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimData(inner) => {
                    <claimDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimDataLen(inner) => {
                    <claimDataLenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claims(inner) => {
                    <claimsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::clockExtension(inner) => {
                    <clockExtensionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::closeGame(inner) => {
                    <closeGameCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createdAt(inner) => {
                    <createdAtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::credit(inner) => {
                    <creditCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::defend(inner) => {
                    <defendCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::extraData(inner) => {
                    <extraDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gameCreator(inner) => {
                    <gameCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gameData(inner) => {
                    <gameDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gameType(inner) => {
                    <gameTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getChallengerDuration(inner) => {
                    <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getNumToResolve(inner) => {
                    <getNumToResolveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRequiredBond(inner) => {
                    <getRequiredBondCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasUnlockedCredit(inner) => {
                    <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::l1Head(inner) => {
                    <l1HeadCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::l2BlockNumber(inner) => {
                    <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2BlockNumberChallenged(inner) => {
                    <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::l2BlockNumberChallenger(inner) => {
                    <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::l2SequenceNumber(inner) => {
                    <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxClockDuration(inner) => {
                    <maxClockDurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxGameDepth(inner) => {
                    <maxGameDepthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::r#move(inner) => {
                    <moveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::normalModeCredit(inner) => {
                    <normalModeCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::refundModeCredit(inner) => {
                    <refundModeCreditCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolutionCheckpoints(inner) => {
                    <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolve(inner) => {
                    <resolveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::resolveClaim(inner) => {
                    <resolveClaimCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolvedAt(inner) => {
                    <resolvedAtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolvedSubgames(inner) => {
                    <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rootClaim(inner) => {
                    <rootClaimCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::splitDepth(inner) => {
                    <splitDepthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startingBlockNumber(inner) => {
                    <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startingOutputRoot(inner) => {
                    <startingOutputRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startingRootHash(inner) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::status(inner) => {
                    <statusCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::step(inner) => {
                    <stepCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::subgames(inner) => {
                    <subgamesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::vm(inner) => {
                    <vmCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::wasRespectedGameTypeWhenCreated(inner) => {
                    <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`FaultDisputeGame`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum FaultDisputeGameErrors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        AnchorRootNotFound(AnchorRootNotFound),
        #[allow(missing_docs)]
        BadExtraData(BadExtraData),
        #[allow(missing_docs)]
        BlockNumberMatches(BlockNumberMatches),
        #[allow(missing_docs)]
        BondTransferFailed(BondTransferFailed),
        #[allow(missing_docs)]
        CannotDefendRootClaim(CannotDefendRootClaim),
        #[allow(missing_docs)]
        ClaimAboveSplit(ClaimAboveSplit),
        #[allow(missing_docs)]
        ClaimAlreadyExists(ClaimAlreadyExists),
        #[allow(missing_docs)]
        ClaimAlreadyResolved(ClaimAlreadyResolved),
        #[allow(missing_docs)]
        ClockNotExpired(ClockNotExpired),
        #[allow(missing_docs)]
        ClockTimeExceeded(ClockTimeExceeded),
        #[allow(missing_docs)]
        ContentLengthMismatch(ContentLengthMismatch),
        #[allow(missing_docs)]
        DuplicateStep(DuplicateStep),
        #[allow(missing_docs)]
        EmptyItem(EmptyItem),
        #[allow(missing_docs)]
        GameDepthExceeded(GameDepthExceeded),
        #[allow(missing_docs)]
        GameNotFinalized(GameNotFinalized),
        #[allow(missing_docs)]
        GameNotInProgress(GameNotInProgress),
        #[allow(missing_docs)]
        GameNotResolved(GameNotResolved),
        #[allow(missing_docs)]
        GamePaused(GamePaused),
        #[allow(missing_docs)]
        IncorrectBondAmount(IncorrectBondAmount),
        #[allow(missing_docs)]
        InvalidBondDistributionMode(InvalidBondDistributionMode),
        #[allow(missing_docs)]
        InvalidChallengePeriod(InvalidChallengePeriod),
        #[allow(missing_docs)]
        InvalidClockExtension(InvalidClockExtension),
        #[allow(missing_docs)]
        InvalidDataRemainder(InvalidDataRemainder),
        #[allow(missing_docs)]
        InvalidDisputedClaimIndex(InvalidDisputedClaimIndex),
        #[allow(missing_docs)]
        InvalidHeader(InvalidHeader),
        #[allow(missing_docs)]
        InvalidHeaderRLP(InvalidHeaderRLP),
        #[allow(missing_docs)]
        InvalidLocalIdent(InvalidLocalIdent),
        #[allow(missing_docs)]
        InvalidOutputRootProof(InvalidOutputRootProof),
        #[allow(missing_docs)]
        InvalidParent(InvalidParent),
        #[allow(missing_docs)]
        InvalidPrestate(InvalidPrestate),
        #[allow(missing_docs)]
        InvalidSplitDepth(InvalidSplitDepth),
        #[allow(missing_docs)]
        L2BlockNumberChallenged(L2BlockNumberChallenged),
        #[allow(missing_docs)]
        MaxDepthTooLarge(MaxDepthTooLarge),
        #[allow(missing_docs)]
        NoCreditToClaim(NoCreditToClaim),
        #[allow(missing_docs)]
        OutOfOrderResolution(OutOfOrderResolution),
        #[allow(missing_docs)]
        ReservedGameType(ReservedGameType),
        #[allow(missing_docs)]
        UnexpectedList(UnexpectedList),
        #[allow(missing_docs)]
        UnexpectedRootClaim(UnexpectedRootClaim),
        #[allow(missing_docs)]
        UnexpectedString(UnexpectedString),
        #[allow(missing_docs)]
        ValidStep(ValidStep),
    }
    impl FaultDisputeGameErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 138u8, 61u8, 244u8],
            [13u8, 193u8, 73u8, 240u8],
            [14u8, 162u8, 231u8, 82u8],
            [23u8, 191u8, 229u8, 247u8],
            [31u8, 249u8, 178u8, 228u8],
            [48u8, 20u8, 3u8, 50u8],
            [51u8, 129u8, 209u8, 20u8],
            [55u8, 154u8, 126u8, 217u8],
            [57u8, 246u8, 139u8, 56u8],
            [72u8, 81u8, 189u8, 155u8],
            [75u8, 156u8, 106u8, 190u8],
            [86u8, 245u8, 123u8, 43u8],
            [90u8, 180u8, 88u8, 251u8],
            [92u8, 85u8, 55u8, 184u8],
            [95u8, 83u8, 221u8, 152u8],
            [102u8, 201u8, 68u8, 133u8],
            [103u8, 254u8, 25u8, 80u8],
            [105u8, 101u8, 80u8, 255u8],
            [106u8, 107u8, 195u8, 178u8],
            [119u8, 223u8, 227u8, 50u8],
            [128u8, 73u8, 126u8, 59u8],
            [131u8, 230u8, 204u8, 107u8],
            [134u8, 32u8, 170u8, 25u8],
            [141u8, 119u8, 236u8, 172u8],
            [144u8, 113u8, 230u8, 175u8],
            [152u8, 36u8, 189u8, 171u8],
            [154u8, 7u8, 102u8, 70u8],
            [156u8, 192u8, 11u8, 91u8],
            [164u8, 38u8, 55u8, 188u8],
            [179u8, 75u8, 92u8, 34u8],
            [180u8, 225u8, 36u8, 51u8],
            [184u8, 237u8, 136u8, 48u8],
            [186u8, 187u8, 1u8, 221u8],
            [193u8, 5u8, 38u8, 10u8],
            [216u8, 29u8, 88u8, 59u8],
            [230u8, 44u8, 207u8, 57u8],
            [241u8, 169u8, 69u8, 129u8],
            [242u8, 68u8, 11u8, 83u8],
            [244u8, 2u8, 57u8, 219u8],
            [251u8, 78u8, 64u8, 221u8],
            [255u8, 19u8, 126u8, 101u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(InvalidBondDistributionMode),
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(L2BlockNumberChallenged),
            ::core::stringify!(NoCreditToClaim),
            ::core::stringify!(UnexpectedList),
            ::core::stringify!(InvalidDisputedClaimIndex),
            ::core::stringify!(ClockTimeExceeded),
            ::core::stringify!(GamePaused),
            ::core::stringify!(ReservedGameType),
            ::core::stringify!(GameNotFinalized),
            ::core::stringify!(UnexpectedString),
            ::core::stringify!(GameDepthExceeded),
            ::core::stringify!(EmptyItem),
            ::core::stringify!(InvalidDataRemainder),
            ::core::stringify!(InvalidParent),
            ::core::stringify!(ContentLengthMismatch),
            ::core::stringify!(GameNotInProgress),
            ::core::stringify!(InvalidPrestate),
            ::core::stringify!(AnchorRootNotFound),
            ::core::stringify!(MaxDepthTooLarge),
            ::core::stringify!(ClaimAlreadyExists),
            ::core::stringify!(BondTransferFailed),
            ::core::stringify!(IncorrectBondAmount),
            ::core::stringify!(InvalidClockExtension),
            ::core::stringify!(DuplicateStep),
            ::core::stringify!(BadExtraData),
            ::core::stringify!(OutOfOrderResolution),
            ::core::stringify!(InvalidOutputRootProof),
            ::core::stringify!(CannotDefendRootClaim),
            ::core::stringify!(ClaimAboveSplit),
            ::core::stringify!(InvalidChallengePeriod),
            ::core::stringify!(BlockNumberMatches),
            ::core::stringify!(InvalidHeader),
            ::core::stringify!(GameNotResolved),
            ::core::stringify!(InvalidHeaderRLP),
            ::core::stringify!(InvalidSplitDepth),
            ::core::stringify!(ClaimAlreadyResolved),
            ::core::stringify!(ClockNotExpired),
            ::core::stringify!(UnexpectedRootClaim),
            ::core::stringify!(ValidStep),
            ::core::stringify!(InvalidLocalIdent),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <InvalidBondDistributionMode as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <L2BlockNumberChallenged as alloy_sol_types::SolError>::SIGNATURE,
            <NoCreditToClaim as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedList as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::SIGNATURE,
            <ClockTimeExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <GamePaused as alloy_sol_types::SolError>::SIGNATURE,
            <ReservedGameType as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotFinalized as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedString as alloy_sol_types::SolError>::SIGNATURE,
            <GameDepthExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyItem as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDataRemainder as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidParent as alloy_sol_types::SolError>::SIGNATURE,
            <ContentLengthMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotInProgress as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidPrestate as alloy_sol_types::SolError>::SIGNATURE,
            <AnchorRootNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <MaxDepthTooLarge as alloy_sol_types::SolError>::SIGNATURE,
            <ClaimAlreadyExists as alloy_sol_types::SolError>::SIGNATURE,
            <BondTransferFailed as alloy_sol_types::SolError>::SIGNATURE,
            <IncorrectBondAmount as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidClockExtension as alloy_sol_types::SolError>::SIGNATURE,
            <DuplicateStep as alloy_sol_types::SolError>::SIGNATURE,
            <BadExtraData as alloy_sol_types::SolError>::SIGNATURE,
            <OutOfOrderResolution as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidOutputRootProof as alloy_sol_types::SolError>::SIGNATURE,
            <CannotDefendRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <ClaimAboveSplit as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidChallengePeriod as alloy_sol_types::SolError>::SIGNATURE,
            <BlockNumberMatches as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHeader as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotResolved as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHeaderRLP as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSplitDepth as alloy_sol_types::SolError>::SIGNATURE,
            <ClaimAlreadyResolved as alloy_sol_types::SolError>::SIGNATURE,
            <ClockNotExpired as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <ValidStep as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidLocalIdent as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for FaultDisputeGameErrors {
        const NAME: &'static str = "FaultDisputeGameErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 41usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AnchorRootNotFound(_) => {
                    <AnchorRootNotFound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BadExtraData(_) => {
                    <BadExtraData as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BlockNumberMatches(_) => {
                    <BlockNumberMatches as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BondTransferFailed(_) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotDefendRootClaim(_) => {
                    <CannotDefendRootClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClaimAboveSplit(_) => {
                    <ClaimAboveSplit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClaimAlreadyExists(_) => {
                    <ClaimAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClaimAlreadyResolved(_) => {
                    <ClaimAlreadyResolved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClockNotExpired(_) => {
                    <ClockNotExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ClockTimeExceeded(_) => {
                    <ClockTimeExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContentLengthMismatch(_) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DuplicateStep(_) => {
                    <DuplicateStep as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyItem(_) => <EmptyItem as alloy_sol_types::SolError>::SELECTOR,
                Self::GameDepthExceeded(_) => {
                    <GameDepthExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GameNotFinalized(_) => {
                    <GameNotFinalized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GameNotInProgress(_) => {
                    <GameNotInProgress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GameNotResolved(_) => {
                    <GameNotResolved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GamePaused(_) => {
                    <GamePaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IncorrectBondAmount(_) => {
                    <IncorrectBondAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBondDistributionMode(_) => {
                    <InvalidBondDistributionMode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidChallengePeriod(_) => {
                    <InvalidChallengePeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidClockExtension(_) => {
                    <InvalidClockExtension as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDataRemainder(_) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDisputedClaimIndex(_) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHeader(_) => {
                    <InvalidHeader as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHeaderRLP(_) => {
                    <InvalidHeaderRLP as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLocalIdent(_) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOutputRootProof(_) => {
                    <InvalidOutputRootProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidParent(_) => {
                    <InvalidParent as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPrestate(_) => {
                    <InvalidPrestate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSplitDepth(_) => {
                    <InvalidSplitDepth as alloy_sol_types::SolError>::SELECTOR
                }
                Self::L2BlockNumberChallenged(_) => {
                    <L2BlockNumberChallenged as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDepthTooLarge(_) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoCreditToClaim(_) => {
                    <NoCreditToClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfOrderResolution(_) => {
                    <OutOfOrderResolution as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReservedGameType(_) => {
                    <ReservedGameType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedList(_) => {
                    <UnexpectedList as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedRootClaim(_) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedString(_) => {
                    <UnexpectedString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidStep(_) => <ValidStep as alloy_sol_types::SolError>::SELECTOR,
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
            ) -> alloy_sol_types::Result<FaultDisputeGameErrors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidBondDistributionMode)
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn L2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::L2BlockNumberChallenged)
                    }
                    L2BlockNumberChallenged
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidDisputedClaimIndex)
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FaultDisputeGameErrors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn ReservedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ReservedGameType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ReservedGameType)
                    }
                    ReservedGameType
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FaultDisputeGameErrors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidClockExtension)
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FaultDisputeGameErrors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::OutOfOrderResolution)
                    }
                    OutOfOrderResolution
                },
                {
                    fn InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidOutputRootProof)
                    }
                    InvalidOutputRootProof
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::CannotDefendRootClaim)
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidChallengePeriod)
                    }
                    InvalidChallengePeriod
                },
                {
                    fn BlockNumberMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <BlockNumberMatches as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::BlockNumberMatches)
                    }
                    BlockNumberMatches
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn InvalidHeaderRLP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidHeaderRLP)
                    }
                    InvalidHeaderRLP
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClaimAlreadyResolved)
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FaultDisputeGameErrors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidLocalIdent)
                    }
                    InvalidLocalIdent
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
            ) -> alloy_sol_types::Result<FaultDisputeGameErrors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidBondDistributionMode)
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn L2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::L2BlockNumberChallenged)
                    }
                    L2BlockNumberChallenged
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidDisputedClaimIndex)
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn ReservedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ReservedGameType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ReservedGameType)
                    }
                    ReservedGameType
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidClockExtension)
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::OutOfOrderResolution)
                    }
                    OutOfOrderResolution
                },
                {
                    fn InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidOutputRootProof)
                    }
                    InvalidOutputRootProof
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::CannotDefendRootClaim)
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidChallengePeriod)
                    }
                    InvalidChallengePeriod
                },
                {
                    fn BlockNumberMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <BlockNumberMatches as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::BlockNumberMatches)
                    }
                    BlockNumberMatches
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn InvalidHeaderRLP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidHeaderRLP)
                    }
                    InvalidHeaderRLP
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClaimAlreadyResolved)
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FaultDisputeGameErrors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FaultDisputeGameErrors::InvalidLocalIdent)
                    }
                    InvalidLocalIdent
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
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AnchorRootNotFound(inner) => {
                    <AnchorRootNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BadExtraData(inner) => {
                    <BadExtraData as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BlockNumberMatches(inner) => {
                    <BlockNumberMatches as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BondTransferFailed(inner) => {
                    <BondTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotDefendRootClaim(inner) => {
                    <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClaimAboveSplit(inner) => {
                    <ClaimAboveSplit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClaimAlreadyExists(inner) => {
                    <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClaimAlreadyResolved(inner) => {
                    <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClockNotExpired(inner) => {
                    <ClockNotExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ClockTimeExceeded(inner) => {
                    <ClockTimeExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContentLengthMismatch(inner) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DuplicateStep(inner) => {
                    <DuplicateStep as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyItem(inner) => {
                    <EmptyItem as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::GameDepthExceeded(inner) => {
                    <GameDepthExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GameNotFinalized(inner) => {
                    <GameNotFinalized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GameNotInProgress(inner) => {
                    <GameNotInProgress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GameNotResolved(inner) => {
                    <GameNotResolved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GamePaused(inner) => {
                    <GamePaused as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::IncorrectBondAmount(inner) => {
                    <IncorrectBondAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBondDistributionMode(inner) => {
                    <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidChallengePeriod(inner) => {
                    <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidClockExtension(inner) => {
                    <InvalidClockExtension as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidDataRemainder(inner) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidDisputedClaimIndex(inner) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHeader(inner) => {
                    <InvalidHeader as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidHeaderRLP(inner) => {
                    <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLocalIdent(inner) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOutputRootProof(inner) => {
                    <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidParent(inner) => {
                    <InvalidParent as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidPrestate(inner) => {
                    <InvalidPrestate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSplitDepth(inner) => {
                    <InvalidSplitDepth as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::L2BlockNumberChallenged(inner) => {
                    <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxDepthTooLarge(inner) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoCreditToClaim(inner) => {
                    <NoCreditToClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfOrderResolution(inner) => {
                    <OutOfOrderResolution as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReservedGameType(inner) => {
                    <ReservedGameType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedList(inner) => {
                    <UnexpectedList as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedRootClaim(inner) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedString(inner) => {
                    <UnexpectedString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidStep(inner) => {
                    <ValidStep as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AnchorRootNotFound(inner) => {
                    <AnchorRootNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BadExtraData(inner) => {
                    <BadExtraData as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BlockNumberMatches(inner) => {
                    <BlockNumberMatches as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::CannotDefendRootClaim(inner) => {
                    <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClaimAboveSplit(inner) => {
                    <ClaimAboveSplit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClaimAlreadyExists(inner) => {
                    <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClaimAlreadyResolved(inner) => {
                    <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClockNotExpired(inner) => {
                    <ClockNotExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ClockTimeExceeded(inner) => {
                    <ClockTimeExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContentLengthMismatch(inner) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DuplicateStep(inner) => {
                    <DuplicateStep as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyItem(inner) => {
                    <EmptyItem as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::GameDepthExceeded(inner) => {
                    <GameDepthExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GameNotFinalized(inner) => {
                    <GameNotFinalized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GameNotInProgress(inner) => {
                    <GameNotInProgress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GameNotResolved(inner) => {
                    <GameNotResolved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GamePaused(inner) => {
                    <GamePaused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::IncorrectBondAmount(inner) => {
                    <IncorrectBondAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBondDistributionMode(inner) => {
                    <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidChallengePeriod(inner) => {
                    <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidClockExtension(inner) => {
                    <InvalidClockExtension as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDataRemainder(inner) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDisputedClaimIndex(inner) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHeader(inner) => {
                    <InvalidHeader as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHeaderRLP(inner) => {
                    <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLocalIdent(inner) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidOutputRootProof(inner) => {
                    <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidParent(inner) => {
                    <InvalidParent as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPrestate(inner) => {
                    <InvalidPrestate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSplitDepth(inner) => {
                    <InvalidSplitDepth as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::L2BlockNumberChallenged(inner) => {
                    <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxDepthTooLarge(inner) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoCreditToClaim(inner) => {
                    <NoCreditToClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfOrderResolution(inner) => {
                    <OutOfOrderResolution as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReservedGameType(inner) => {
                    <ReservedGameType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnexpectedList(inner) => {
                    <UnexpectedList as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnexpectedRootClaim(inner) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnexpectedString(inner) => {
                    <UnexpectedString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidStep(inner) => {
                    <ValidStep as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`FaultDisputeGame`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum FaultDisputeGameEvents {
        #[allow(missing_docs)]
        GameClosed(GameClosed),
        #[allow(missing_docs)]
        Move(Move),
        #[allow(missing_docs)]
        Resolved(Resolved),
    }
    impl FaultDisputeGameEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                94u8, 24u8, 111u8, 9u8, 185u8, 201u8, 52u8, 145u8, 241u8, 78u8, 39u8,
                126u8, 234u8, 127u8, 170u8, 93u8, 230u8, 162u8, 212u8, 189u8, 167u8,
                90u8, 121u8, 175u8, 122u8, 54u8, 132u8, 251u8, 251u8, 66u8, 218u8, 96u8,
            ],
            [
                153u8, 8u8, 234u8, 172u8, 6u8, 69u8, 223u8, 157u8, 7u8, 4u8, 208u8,
                106u8, 220u8, 158u8, 7u8, 51u8, 124u8, 149u8, 29u8, 226u8, 240u8, 107u8,
                95u8, 40u8, 54u8, 21u8, 29u8, 72u8, 213u8, 228u8, 114u8, 47u8,
            ],
            [
                155u8, 50u8, 69u8, 116u8, 14u8, 195u8, 177u8, 85u8, 9u8, 138u8, 85u8,
                190u8, 132u8, 149u8, 122u8, 77u8, 161u8, 62u8, 175u8, 127u8, 20u8, 168u8,
                188u8, 111u8, 83u8, 18u8, 108u8, 11u8, 147u8, 80u8, 242u8, 190u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(Resolved),
            ::core::stringify!(GameClosed),
            ::core::stringify!(Move),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <Resolved as alloy_sol_types::SolEvent>::SIGNATURE,
            <GameClosed as alloy_sol_types::SolEvent>::SIGNATURE,
            <Move as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for FaultDisputeGameEvents {
        const NAME: &'static str = "FaultDisputeGameEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<GameClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <GameClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GameClosed)
                }
                Some(<Move as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Move as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Move)
                }
                Some(<Resolved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Resolved as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Resolved)
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
    impl alloy_sol_types::private::IntoLogData for FaultDisputeGameEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GameClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Move(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Resolved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GameClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Move(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Resolved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`FaultDisputeGame`](self) contract instance.

See the [wrapper's documentation](`FaultDisputeGameInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> FaultDisputeGameInstance<P, N> {
        FaultDisputeGameInstance::<P, N>::new(address, __provider)
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
        _params: <GameConstructorParams as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<FaultDisputeGameInstance<P, N>>,
    > {
        FaultDisputeGameInstance::<P, N>::deploy(__provider, _params)
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
        _params: <GameConstructorParams as alloy::sol_types::SolType>::RustType,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        FaultDisputeGameInstance::<P, N>::deploy_builder(__provider, _params)
    }
    /**A [`FaultDisputeGame`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`FaultDisputeGame`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct FaultDisputeGameInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for FaultDisputeGameInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("FaultDisputeGameInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FaultDisputeGameInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`FaultDisputeGame`](self) contract instance.

See the [wrapper's documentation](`FaultDisputeGameInstance`) for more details.*/
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
            _params: <GameConstructorParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<FaultDisputeGameInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, _params);
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
            _params: <GameConstructorParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _params },
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
    impl<P: ::core::clone::Clone, N> FaultDisputeGameInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> FaultDisputeGameInstance<P, N> {
            FaultDisputeGameInstance {
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
    > FaultDisputeGameInstance<P, N> {
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
        ///Creates a new call builder for the [`absolutePrestate`] function.
        pub fn absolutePrestate(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, absolutePrestateCall, N> {
            self.call_builder(&absolutePrestateCall)
        }
        ///Creates a new call builder for the [`addLocalData`] function.
        pub fn addLocalData(
            &self,
            _ident: alloy::sol_types::private::primitives::aliases::U256,
            _execLeafIdx: alloy::sol_types::private::primitives::aliases::U256,
            _partOffset: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, addLocalDataCall, N> {
            self.call_builder(
                &addLocalDataCall {
                    _ident,
                    _execLeafIdx,
                    _partOffset,
                },
            )
        }
        ///Creates a new call builder for the [`anchorStateRegistry`] function.
        pub fn anchorStateRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, anchorStateRegistryCall, N> {
            self.call_builder(&anchorStateRegistryCall)
        }
        ///Creates a new call builder for the [`attack`] function.
        pub fn attack(
            &self,
            _disputed: <Claim as alloy::sol_types::SolType>::RustType,
            _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
            _claim: <Claim as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, attackCall, N> {
            self.call_builder(
                &attackCall {
                    _disputed,
                    _parentIndex,
                    _claim,
                },
            )
        }
        ///Creates a new call builder for the [`bondDistributionMode`] function.
        pub fn bondDistributionMode(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, bondDistributionModeCall, N> {
            self.call_builder(&bondDistributionModeCall)
        }
        ///Creates a new call builder for the [`challengeRootL2Block`] function.
        pub fn challengeRootL2Block(
            &self,
            _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
            _headerRLP: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, challengeRootL2BlockCall, N> {
            self.call_builder(
                &challengeRootL2BlockCall {
                    _outputRootProof,
                    _headerRLP,
                },
            )
        }
        ///Creates a new call builder for the [`claimCredit`] function.
        pub fn claimCredit(
            &self,
            _recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, claimCreditCall, N> {
            self.call_builder(&claimCreditCall { _recipient })
        }
        ///Creates a new call builder for the [`claimData`] function.
        pub fn claimData(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, claimDataCall, N> {
            self.call_builder(&claimDataCall(_0))
        }
        ///Creates a new call builder for the [`claimDataLen`] function.
        pub fn claimDataLen(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, claimDataLenCall, N> {
            self.call_builder(&claimDataLenCall)
        }
        ///Creates a new call builder for the [`claims`] function.
        pub fn claims(
            &self,
            _0: <Hash as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, claimsCall, N> {
            self.call_builder(&claimsCall(_0))
        }
        ///Creates a new call builder for the [`clockExtension`] function.
        pub fn clockExtension(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, clockExtensionCall, N> {
            self.call_builder(&clockExtensionCall)
        }
        ///Creates a new call builder for the [`closeGame`] function.
        pub fn closeGame(&self) -> alloy_contract::SolCallBuilder<&P, closeGameCall, N> {
            self.call_builder(&closeGameCall)
        }
        ///Creates a new call builder for the [`createdAt`] function.
        pub fn createdAt(&self) -> alloy_contract::SolCallBuilder<&P, createdAtCall, N> {
            self.call_builder(&createdAtCall)
        }
        ///Creates a new call builder for the [`credit`] function.
        pub fn credit(
            &self,
            _recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, creditCall, N> {
            self.call_builder(&creditCall { _recipient })
        }
        ///Creates a new call builder for the [`defend`] function.
        pub fn defend(
            &self,
            _disputed: <Claim as alloy::sol_types::SolType>::RustType,
            _parentIndex: alloy::sol_types::private::primitives::aliases::U256,
            _claim: <Claim as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, defendCall, N> {
            self.call_builder(
                &defendCall {
                    _disputed,
                    _parentIndex,
                    _claim,
                },
            )
        }
        ///Creates a new call builder for the [`extraData`] function.
        pub fn extraData(&self) -> alloy_contract::SolCallBuilder<&P, extraDataCall, N> {
            self.call_builder(&extraDataCall)
        }
        ///Creates a new call builder for the [`gameCreator`] function.
        pub fn gameCreator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, gameCreatorCall, N> {
            self.call_builder(&gameCreatorCall)
        }
        ///Creates a new call builder for the [`gameData`] function.
        pub fn gameData(&self) -> alloy_contract::SolCallBuilder<&P, gameDataCall, N> {
            self.call_builder(&gameDataCall)
        }
        ///Creates a new call builder for the [`gameType`] function.
        pub fn gameType(&self) -> alloy_contract::SolCallBuilder<&P, gameTypeCall, N> {
            self.call_builder(&gameTypeCall)
        }
        ///Creates a new call builder for the [`getChallengerDuration`] function.
        pub fn getChallengerDuration(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getChallengerDurationCall, N> {
            self.call_builder(
                &getChallengerDurationCall {
                    _claimIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getNumToResolve`] function.
        pub fn getNumToResolve(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getNumToResolveCall, N> {
            self.call_builder(&getNumToResolveCall { _claimIndex })
        }
        ///Creates a new call builder for the [`getRequiredBond`] function.
        pub fn getRequiredBond(
            &self,
            _position: <Position as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getRequiredBondCall, N> {
            self.call_builder(&getRequiredBondCall { _position })
        }
        ///Creates a new call builder for the [`hasUnlockedCredit`] function.
        pub fn hasUnlockedCredit(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasUnlockedCreditCall, N> {
            self.call_builder(&hasUnlockedCreditCall(_0))
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall)
        }
        ///Creates a new call builder for the [`l1Head`] function.
        pub fn l1Head(&self) -> alloy_contract::SolCallBuilder<&P, l1HeadCall, N> {
            self.call_builder(&l1HeadCall)
        }
        ///Creates a new call builder for the [`l2BlockNumber`] function.
        pub fn l2BlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2BlockNumberCall, N> {
            self.call_builder(&l2BlockNumberCall)
        }
        ///Creates a new call builder for the [`l2BlockNumberChallenged`] function.
        pub fn l2BlockNumberChallenged(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2BlockNumberChallengedCall, N> {
            self.call_builder(&l2BlockNumberChallengedCall)
        }
        ///Creates a new call builder for the [`l2BlockNumberChallenger`] function.
        pub fn l2BlockNumberChallenger(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2BlockNumberChallengerCall, N> {
            self.call_builder(&l2BlockNumberChallengerCall)
        }
        ///Creates a new call builder for the [`l2ChainId`] function.
        pub fn l2ChainId(&self) -> alloy_contract::SolCallBuilder<&P, l2ChainIdCall, N> {
            self.call_builder(&l2ChainIdCall)
        }
        ///Creates a new call builder for the [`l2SequenceNumber`] function.
        pub fn l2SequenceNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, l2SequenceNumberCall, N> {
            self.call_builder(&l2SequenceNumberCall)
        }
        ///Creates a new call builder for the [`maxClockDuration`] function.
        pub fn maxClockDuration(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, maxClockDurationCall, N> {
            self.call_builder(&maxClockDurationCall)
        }
        ///Creates a new call builder for the [`maxGameDepth`] function.
        pub fn maxGameDepth(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, maxGameDepthCall, N> {
            self.call_builder(&maxGameDepthCall)
        }
        ///Creates a new call builder for the [`r#move`] function.
        pub fn r#move(
            &self,
            _disputed: <Claim as alloy::sol_types::SolType>::RustType,
            _challengeIndex: alloy::sol_types::private::primitives::aliases::U256,
            _claim: <Claim as alloy::sol_types::SolType>::RustType,
            _isAttack: bool,
        ) -> alloy_contract::SolCallBuilder<&P, moveCall, N> {
            self.call_builder(
                &moveCall {
                    _disputed,
                    _challengeIndex,
                    _claim,
                    _isAttack,
                },
            )
        }
        ///Creates a new call builder for the [`normalModeCredit`] function.
        pub fn normalModeCredit(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, normalModeCreditCall, N> {
            self.call_builder(&normalModeCreditCall(_0))
        }
        ///Creates a new call builder for the [`refundModeCredit`] function.
        pub fn refundModeCredit(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, refundModeCreditCall, N> {
            self.call_builder(&refundModeCreditCall(_0))
        }
        ///Creates a new call builder for the [`resolutionCheckpoints`] function.
        pub fn resolutionCheckpoints(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resolutionCheckpointsCall, N> {
            self.call_builder(&resolutionCheckpointsCall(_0))
        }
        ///Creates a new call builder for the [`resolve`] function.
        pub fn resolve(&self) -> alloy_contract::SolCallBuilder<&P, resolveCall, N> {
            self.call_builder(&resolveCall)
        }
        ///Creates a new call builder for the [`resolveClaim`] function.
        pub fn resolveClaim(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
            _numToResolve: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resolveClaimCall, N> {
            self.call_builder(
                &resolveClaimCall {
                    _claimIndex,
                    _numToResolve,
                },
            )
        }
        ///Creates a new call builder for the [`resolvedAt`] function.
        pub fn resolvedAt(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, resolvedAtCall, N> {
            self.call_builder(&resolvedAtCall)
        }
        ///Creates a new call builder for the [`resolvedSubgames`] function.
        pub fn resolvedSubgames(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resolvedSubgamesCall, N> {
            self.call_builder(&resolvedSubgamesCall(_0))
        }
        ///Creates a new call builder for the [`rootClaim`] function.
        pub fn rootClaim(&self) -> alloy_contract::SolCallBuilder<&P, rootClaimCall, N> {
            self.call_builder(&rootClaimCall)
        }
        ///Creates a new call builder for the [`splitDepth`] function.
        pub fn splitDepth(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, splitDepthCall, N> {
            self.call_builder(&splitDepthCall)
        }
        ///Creates a new call builder for the [`startingBlockNumber`] function.
        pub fn startingBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingBlockNumberCall, N> {
            self.call_builder(&startingBlockNumberCall)
        }
        ///Creates a new call builder for the [`startingOutputRoot`] function.
        pub fn startingOutputRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingOutputRootCall, N> {
            self.call_builder(&startingOutputRootCall)
        }
        ///Creates a new call builder for the [`startingRootHash`] function.
        pub fn startingRootHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingRootHashCall, N> {
            self.call_builder(&startingRootHashCall)
        }
        ///Creates a new call builder for the [`status`] function.
        pub fn status(&self) -> alloy_contract::SolCallBuilder<&P, statusCall, N> {
            self.call_builder(&statusCall)
        }
        ///Creates a new call builder for the [`step`] function.
        pub fn step(
            &self,
            _claimIndex: alloy::sol_types::private::primitives::aliases::U256,
            _isAttack: bool,
            _stateData: alloy::sol_types::private::Bytes,
            _proof: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, stepCall, N> {
            self.call_builder(
                &stepCall {
                    _claimIndex,
                    _isAttack,
                    _stateData,
                    _proof,
                },
            )
        }
        ///Creates a new call builder for the [`subgames`] function.
        pub fn subgames(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, subgamesCall, N> {
            self.call_builder(&subgamesCall { _0, _1 })
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
        ///Creates a new call builder for the [`vm`] function.
        pub fn vm(&self) -> alloy_contract::SolCallBuilder<&P, vmCall, N> {
            self.call_builder(&vmCall)
        }
        ///Creates a new call builder for the [`wasRespectedGameTypeWhenCreated`] function.
        pub fn wasRespectedGameTypeWhenCreated(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, wasRespectedGameTypeWhenCreatedCall, N> {
            self.call_builder(&wasRespectedGameTypeWhenCreatedCall)
        }
        ///Creates a new call builder for the [`weth`] function.
        pub fn weth(&self) -> alloy_contract::SolCallBuilder<&P, wethCall, N> {
            self.call_builder(&wethCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FaultDisputeGameInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`GameClosed`] event.
        pub fn GameClosed_filter(&self) -> alloy_contract::Event<&P, GameClosed, N> {
            self.event_filter::<GameClosed>()
        }
        ///Creates a new event filter for the [`Move`] event.
        pub fn Move_filter(&self) -> alloy_contract::Event<&P, Move, N> {
            self.event_filter::<Move>()
        }
        ///Creates a new event filter for the [`Resolved`] event.
        pub fn Resolved_filter(&self) -> alloy_contract::Event<&P, Resolved, N> {
            self.event_filter::<Resolved>()
        }
    }
}

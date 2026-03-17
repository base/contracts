///Module containing a contract's types and functions.
/**

```solidity
library SuperFaultDisputeGame {
    struct GameConstructorParams { uint256 maxGameDepth; uint256 splitDepth; Duration clockExtension; Duration maxClockDuration; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod SuperFaultDisputeGame {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**```solidity
struct GameConstructorParams { uint256 maxGameDepth; uint256 splitDepth; Duration clockExtension; Duration maxClockDuration; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameConstructorParams {
        #[allow(missing_docs)]
        pub maxGameDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub splitDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub clockExtension: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub maxClockDuration: <Duration as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            Duration,
            Duration,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <Duration as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<GameConstructorParams> for UnderlyingRustTuple<'_> {
            fn from(value: GameConstructorParams) -> Self {
                (
                    value.maxGameDepth,
                    value.splitDepth,
                    value.clockExtension,
                    value.maxClockDuration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameConstructorParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxGameDepth: tuple.0,
                    splitDepth: tuple.1,
                    clockExtension: tuple.2,
                    maxClockDuration: tuple.3,
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
                    "GameConstructorParams(uint256 maxGameDepth,uint256 splitDepth,Duration clockExtension,Duration maxClockDuration)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(<Duration as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <Duration as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(<Duration as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <Duration as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
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
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GameConstructorParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
    /**Creates a new wrapper around an on-chain [`SuperFaultDisputeGame`](self) contract instance.

See the [wrapper's documentation](`SuperFaultDisputeGameInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> SuperFaultDisputeGameInstance<P, N> {
        SuperFaultDisputeGameInstance::<P, N>::new(address, __provider)
    }
    /**A [`SuperFaultDisputeGame`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SuperFaultDisputeGame`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SuperFaultDisputeGameInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SuperFaultDisputeGameInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SuperFaultDisputeGameInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SuperFaultDisputeGameInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SuperFaultDisputeGame`](self) contract instance.

See the [wrapper's documentation](`SuperFaultDisputeGameInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> SuperFaultDisputeGameInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SuperFaultDisputeGameInstance<P, N> {
            SuperFaultDisputeGameInstance {
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
    > SuperFaultDisputeGameInstance<P, N> {
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
    > SuperFaultDisputeGameInstance<P, N> {
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
library SuperFaultDisputeGame {
    struct GameConstructorParams {
        uint256 maxGameDepth;
        uint256 splitDepth;
        Duration clockExtension;
        Duration maxClockDuration;
    }
}

interface SuperPermissionedDisputeGame {
    type BondDistributionMode is uint8;
    type GameStatus is uint8;
    type Claim is bytes32;
    type Clock is uint128;
    type Duration is uint64;
    type GameType is uint32;
    type Hash is bytes32;
    type Position is uint128;
    type Timestamp is uint64;

    error AlreadyInitialized();
    error AnchorRootNotFound();
    error BadAuth();
    error BadExtraData();
    error BondTransferFailed();
    error CannotDefendRootClaim();
    error ClaimAboveSplit();
    error ClaimAlreadyExists();
    error ClaimAlreadyResolved();
    error ClockNotExpired();
    error ClockTimeExceeded();
    error DuplicateStep();
    error GameDepthExceeded();
    error GameNotFinalized();
    error GameNotInProgress();
    error GameNotResolved();
    error GamePaused();
    error IncorrectBondAmount();
    error InvalidBondDistributionMode();
    error InvalidChallengePeriod();
    error InvalidClockExtension();
    error InvalidDisputedClaimIndex();
    error InvalidLocalIdent();
    error InvalidParent();
    error InvalidPrestate();
    error InvalidSplitDepth();
    error MaxDepthTooLarge();
    error NoChainIdNeeded();
    error NoCreditToClaim();
    error OutOfOrderResolution();
    error SuperFaultDisputeGameInvalidRootClaim();
    error UnexpectedRootClaim(Claim rootClaim);
    error ValidStep();

    event GameClosed(BondDistributionMode bondDistributionMode);
    event Move(uint256 indexed parentIndex, Claim indexed claim, address indexed claimant);
    event Resolved(GameStatus indexed status);

    constructor(SuperFaultDisputeGame.GameConstructorParams _params);

    function absolutePrestate() external pure returns (Claim absolutePrestate_);
    function addLocalData(uint256 _ident, uint256 _execLeafIdx, uint256 _partOffset) external;
    function anchorStateRegistry() external pure returns (address registry_);
    function attack(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
    function bondDistributionMode() external view returns (BondDistributionMode);
    function challenger() external pure returns (address challenger_);
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
    function gameData() external pure returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_);
    function gameType() external pure returns (GameType gameType_);
    function getChallengerDuration(uint256 _claimIndex) external view returns (Duration duration_);
    function getNumToResolve(uint256 _claimIndex) external view returns (uint256 numRemainingChildren_);
    function getRequiredBond(Position _position) external view returns (uint256 requiredBond_);
    function hasUnlockedCredit(address) external view returns (bool);
    function initialize() external payable;
    function l1Head() external pure returns (Hash l1Head_);
    function l2SequenceNumber() external pure returns (uint256 l2SequenceNumber_);
    function maxClockDuration() external view returns (Duration maxClockDuration_);
    function maxGameDepth() external view returns (uint256 maxGameDepth_);
    function move(Claim _disputed, uint256 _challengeIndex, Claim _claim, bool _isAttack) external payable;
    function normalModeCredit(address) external view returns (uint256);
    function proposer() external pure returns (address proposer_);
    function refundModeCredit(address) external view returns (uint256);
    function resolutionCheckpoints(uint256) external view returns (bool initialCheckpointComplete, uint32 subgameIndex, Position leftmostPosition, address counteredBy);
    function resolve() external returns (GameStatus status_);
    function resolveClaim(uint256 _claimIndex, uint256 _numToResolve) external;
    function resolvedAt() external view returns (Timestamp);
    function resolvedSubgames(uint256) external view returns (bool);
    function rootClaim() external pure returns (Claim rootClaim_);
    function splitDepth() external view returns (uint256 splitDepth_);
    function startingProposal() external view returns (Hash root, uint256 l2SequenceNumber);
    function startingRootHash() external view returns (Hash startingRootHash_);
    function startingSequenceNumber() external view returns (uint256 startingSequenceNumber_);
    function status() external view returns (GameStatus);
    function step(uint256 _claimIndex, bool _isAttack, bytes memory _stateData, bytes memory _proof) external;
    function subgames(uint256, uint256) external view returns (uint256);
    function version() external pure returns (string memory);
    function vm() external pure returns (address vm_);
    function wasRespectedGameTypeWhenCreated() external view returns (bool);
    function weth() external pure returns (address weth_);
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
        "internalType": "struct SuperFaultDisputeGame.GameConstructorParams",
        "components": [
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
    "stateMutability": "pure"
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
    "stateMutability": "pure"
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
    "name": "challenger",
    "inputs": [],
    "outputs": [
      {
        "name": "challenger_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "pure"
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
    "stateMutability": "pure"
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
    "stateMutability": "pure"
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
    "name": "proposer",
    "inputs": [],
    "outputs": [
      {
        "name": "proposer_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "pure"
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
    "name": "startingProposal",
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
    "name": "startingSequenceNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "startingSequenceNumber_",
        "type": "uint256",
        "internalType": "uint256"
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
    "stateMutability": "pure"
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
    "stateMutability": "pure"
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
    "name": "BadAuth",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BadExtraData",
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
    "name": "DuplicateStep",
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
    "name": "InvalidDisputedClaimIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLocalIdent",
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
    "name": "MaxDepthTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoChainIdNeeded",
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
    "name": "SuperFaultDisputeGameInvalidRootClaim",
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
pub mod SuperPermissionedDisputeGame {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101006040523480156200001257600080fd5b5060405162006124380380620061248339810160408190526200003591620001e7565b80620000446001607e62000281565b60ff16816000015111156200006c57604051633beff19960e11b815260040160405180910390fd5b600019816020015114806200009357508051602082015162000090906001620002a7565b10155b15620000b25760405163e62ccf3960e01b815260040160405180910390fd5b600281602001511015620000d95760405163e62ccf3960e01b815260040160405180910390fd5b6000620000fe82604001516001600160401b0316620001c760201b620026941760201c565b62000114906001600160401b03166002620002c2565b90506001600160401b038111156200013f5760405163235dfb2b60e21b815260040160405180910390fd5b6200016282606001516001600160401b0316620001c760201b620026941760201c565b6001600160401b0316816001600160401b03161115620001955760405163235dfb2b60e21b815260040160405180910390fd5b508051608052602081015160a05260408101516001600160401b0390811660e0526060909101511660c05250620002e4565b90565b80516001600160401b0381168114620001e257600080fd5b919050565b600060808284031215620001fa57600080fd5b604051608081016001600160401b03811182821017156200022b57634e487b7160e01b600052604160045260246000fd5b806040525082518152602083015160208201526200024c60408401620001ca565b60408201526200025f60608401620001ca565b60608201529392505050565b634e487b7160e01b600052601160045260246000fd5b600060ff821660ff8416808210156200029e576200029e6200026b565b90039392505050565b60008219821115620002bd57620002bd6200026b565b500190565b6000816000190483118215151615620002df57620002df6200026b565b500290565b60805160a05160c05160e051615d49620003db6000396000818161071a01528181612e8201528181612eed01528181612f200152818161376701526138ae015260008181610b4701528181610d15015281816121030152818161214501528181612ce101528181612f5001528181612faf0152613939015260008181610b7a01528181612b8701528181612c5001528181612ebe0152818161459101528181614a4701528181614b4801528181614c1d01528181614f130152615174015260008181610c21015281816122130152818161229901528181612bf301528181612d450152818161448701526145b20152615d496000f3fe60806040526004361061031e5760003560e01c8063786b844b116101a5578063c0d8bb74116100ec578063dabd396d11610095578063f8f43ff61161006f578063f8f43ff614610bce578063fa24f74314610bee578063fa315aa914610c12578063fe2bbeb214610c4557600080fd5b8063dabd396d14610b38578063ec5e630814610b6b578063eff0f59214610b9e57600080fd5b8063cf09e0d0116100c6578063cf09e0d014610ad7578063d5d44d8014610af8578063d8cc1a3c14610b1857600080fd5b8063c0d8bb7414610a00578063c395e1ca14610a2d578063c6f0308c14610a4d57600080fd5b806399735e321161014e578063bbdc02db11610128578063bbdc02db14610955578063bcef3b55146109a0578063bd8da956146109e057600080fd5b806399735e3214610806578063a445ece614610846578063a8e4fb901461091257600080fd5b80638980e0cc1161017f5780638980e0cc146107815780638d450a9514610796578063938d689a146107d657600080fd5b8063786b844b146107515780637b0f0adc146107665780638129fc1c1461077957600080fd5b8063472777c6116102695780635c0cba33116102125780636361506d116101ec5780636361506d146106cb5780636b6716c01461070b5780636f0344091461073e57600080fd5b80635c0cba3314610653578063609d33341461069657806360e27464146106ab57600080fd5b806354fd4d501161024357806354fd4d50146105cf57806359cebe091461061e5780635a5fa2d91461063357600080fd5b8063472777c61461057a578063529d6a8c1461058d578063534db0e2146105ba57600080fd5b80632810e1d6116102cb57806337b1b229116102a557806337b1b229146104935780633a768463146104f45780633fc8cef31461053757600080fd5b80632810e1d6146104445780632ad69aeb14610459578063378dd48c1461047957600080fd5b8063222abf45116102fc578063222abf45146103cb578063250e69bd1461040b57806325fc2ace1461042557600080fd5b806303c2924d1461032357806319effeb414610345578063200d2ed214610390575b600080fd5b34801561032f57600080fd5b5061034361033e36600461554f565b610c75565b005b34801561035157600080fd5b506000546103729068010000000000000000900467ffffffffffffffff1681565b60405167ffffffffffffffff90911681526020015b60405180910390f35b34801561039c57600080fd5b506000546103be90700100000000000000000000000000000000900460ff1681565b60405161038791906155da565b3480156103d757600080fd5b506103fb6103e636600461560f565b600b6020526000908152604090205460ff1681565b6040519015158152602001610387565b34801561041757600080fd5b506009546103fb9060ff1681565b34801561043157600080fd5b506007545b604051908152602001610387565b34801561045057600080fd5b506103be611281565b34801561046557600080fd5b5061043661047436600461554f565b6114a6565b34801561048557600080fd5b50600c546103be9060ff1681565b34801561049f57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90033560601c5b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610387565b34801561050057600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c6104cf565b34801561054357600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c6104cf565b610343610588366004615633565b6114dc565b34801561059957600080fd5b506104366105a836600461560f565b60026020526000908152604090205481565b3480156105c657600080fd5b506104cf6114ee565b3480156105db57600080fd5b5060408051808201909152600581527f302e362e3000000000000000000000000000000000000000000000000000000060208201525b60405161038791906156ca565b34801561062a57600080fd5b50600854610436565b34801561063f57600080fd5b5061043661064e3660046156dd565b611531565b34801561065f57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6104cf565b3480156106a257600080fd5b5061061161156b565b3480156106b757600080fd5b506103436106c636600461560f565b611579565b3480156106d757600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360340135610436565b34801561071757600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610372565b61034361074c366004615704565b61194c565b34801561075d57600080fd5b50610343611a04565b610343610774366004615633565b611eda565b610343611ee7565b34801561078d57600080fd5b50600154610436565b3480156107a257600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360780135610436565b3480156107e257600080fd5b506007546008546107f1919082565b60408051928352602083019190915201610387565b34801561081257600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360580135610436565b34801561085257600080fd5b506108be6108613660046156dd565b6006602052600090815260409020805460019091015460ff821691610100810463ffffffff1691650100000000009091046fffffffffffffffffffffffffffffffff169073ffffffffffffffffffffffffffffffffffffffff1684565b60408051941515855263ffffffff90931660208501526fffffffffffffffffffffffffffffffff9091169183019190915273ffffffffffffffffffffffffffffffffffffffff166060820152608001610387565b34801561091e57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c6104cf565b34801561096157600080fd5b50604051367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036054013560e01c8152602001610387565b3480156109ac57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360140135610436565b3480156109ec57600080fd5b506103726109fb3660046156dd565b611f8e565b348015610a0c57600080fd5b50610436610a1b36600461560f565b600a6020526000908152604090205481565b348015610a3957600080fd5b50610436610a48366004615745565b61216d565b348015610a5957600080fd5b50610a6d610a683660046156dd565b612350565b6040805163ffffffff909816885273ffffffffffffffffffffffffffffffffffffffff968716602089015295909416948601949094526fffffffffffffffffffffffffffffffff9182166060860152608085015291821660a08401521660c082015260e001610387565b348015610ae357600080fd5b506000546103729067ffffffffffffffff1681565b348015610b0457600080fd5b50610436610b1336600461560f565b6123e7565b348015610b2457600080fd5b50610343610b333660046157c0565b612459565b348015610b4457600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610372565b348015610b7757600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610436565b348015610baa57600080fd5b506103fb610bb93660046156dd565b60036020526000908152604090205460ff1681565b348015610bda57600080fd5b50610343610be9366004615633565b61250d565b348015610bfa57600080fd5b50610c03612863565b6040516103879392919061584c565b348015610c1e57600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610436565b348015610c5157600080fd5b506103fb610c603660046156dd565b60056020526000908152604090205460ff1681565b60008054700100000000000000000000000000000000900460ff166002811115610ca157610ca1615571565b14610cd8576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600060018381548110610ced57610ced615871565b906000526020600020906005020190506000610d0884611f8e565b905067ffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081169082161015610d71576040517ff2440b5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008481526005602052604090205460ff1615610dba576040517ff1a9458100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000848152600460205260409020805480158015610dd757508515155b15610e72578354640100000000900473ffffffffffffffffffffffffffffffffffffffff1660008115610e0a5781610e26565b600186015473ffffffffffffffffffffffffffffffffffffffff165b9050610e3281876128ab565b50505060009485525050600560205250506040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055565b6000868152600660209081526040918290208251608081018452815460ff81161515808352610100820463ffffffff16948301949094526501000000000090046fffffffffffffffffffffffffffffffff16938101939093526001015473ffffffffffffffffffffffffffffffffffffffff166060830152610f15576fffffffffffffffffffffffffffffffff6040820152600181526000869003610f15578195505b600086826020015163ffffffff16610f2d91906158cf565b90506000838211610f3e5781610f40565b835b602084015190915063ffffffff165b8181101561108c576000868281548110610f6b57610f6b615871565b6000918252602080832090910154808352600590915260409091205490915060ff16610fc3576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600060018281548110610fd857610fd8615871565b600091825260209091206005909102018054909150640100000000900473ffffffffffffffffffffffffffffffffffffffff161580156110355750600481015460408701516fffffffffffffffffffffffffffffffff9182169116115b1561107757600181015473ffffffffffffffffffffffffffffffffffffffff16606087015260048101546fffffffffffffffffffffffffffffffff1660408701525b50508080611084906158e7565b915050610f4f565b5063ffffffff818116602085810191825260008c81526006909152604090819020865181549351928801517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009094169015157fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000ff161761010092909416918202939093177fffffffffffffffffffffff00000000000000000000000000000000ffffffffff16650100000000006fffffffffffffffffffffffffffffffff909316929092029190911782556060850151600190920180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff9093169290921790915584900361127657606083015160008a815260056020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905561122d73ffffffffffffffffffffffffffffffffffffffff82161561120b5781611227565b600189015473ffffffffffffffffffffffffffffffffffffffff165b896128ab565b875473ffffffffffffffffffffffffffffffffffffffff909116640100000000027fffffffffffffffff0000000000000000000000000000000000000000ffffffff9091161787555b505050505050505050565b600080600054700100000000000000000000000000000000900460ff1660028111156112af576112af615571565b146112e6576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000805260056020527f05b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc5460ff1661134a576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff16600160008154811061137657611376615871565b6000918252602090912060059091020154640100000000900473ffffffffffffffffffffffffffffffffffffffff16146113b15760016113b4565b60025b6000805467ffffffffffffffff421668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff82168117835592935083927fffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffff167fffffffffffffffffffffffffffffff000000000000000000ffffffffffffffff9091161770010000000000000000000000000000000083600281111561146557611465615571565b02179055600281111561147a5761147a615571565b6040517f5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da6090600090a290565b600460205281600052604060002081815481106114c257600080fd5b90600052602060002001600091509150505481565b905090565b6114e9838383600161194c565b505050565b60006114d76114ff60f460146158cf565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c9003013560601c90565b6000818152600660209081526040808320600490925282208054825461156290610100900463ffffffff168261591f565b95945050505050565b60606114d760586020612904565b611581611a04565b60006002600c5460ff16600281111561159c5761159c615571565b036115cd575073ffffffffffffffffffffffffffffffffffffffff81166000908152600a6020526040902054611649565b6001600c5460ff1660028111156115e6576115e6615571565b03611617575073ffffffffffffffffffffffffffffffffffffffff8116600090815260026020526040902054611649565b6040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff82166000908152600b602052604090205460ff1661178a5773ffffffffffffffffffffffffffffffffffffffff82166000908152600b6020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790556116fc60c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe369081013560f01c9003013560601c90565b6040517f7eee288d00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8481166004830152602482018490529190911690637eee288d90604401600060405180830381600087803b15801561176e57600080fd5b505af1158015611782573d6000803e3d6000fd5b505050505050565b806000036117c4576040517f17bfe5f700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff82166000908152600a602090815260408083208390556002909152812055367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c6040517ff3fef3a300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff848116600483015260248201849052919091169063f3fef3a390604401600060405180830381600087803b15801561189a57600080fd5b505af11580156118ae573d6000803e3d6000fd5b5050505060008273ffffffffffffffffffffffffffffffffffffffff168260405160006040518083038185875af1925050503d806000811461190c576040519150601f19603f3d011682016040523d82523d6000602084013e611911565b606091505b50509050806114e9576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c3314806119bc575061198d6114ee565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b6119f2576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6119fe84848484612956565b50505050565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c73ffffffffffffffffffffffffffffffffffffffff16635c975abb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a7f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611aa39190615936565b15611ada576040517f379a7ed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002600c5460ff166002811115611af357611af3615571565b1480611b1557506001600c5460ff166002811115611b1357611b13615571565b145b15611b1c57565b6000600c5460ff166002811115611b3557611b35615571565b14611b6c576040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60005468010000000000000000900467ffffffffffffffff1667ffffffffffffffff16600003611bc8576040517fc105260a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6040517f0314d2b300000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff9190911690630314d2b390602401602060405180830381865afa158015611c67573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c8b9190615936565b905080611cc4576040517f4851bd9b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6040517f17cf21a900000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff91909116906317cf21a990602401600060405180830381600087803b158015611d5e57600080fd5b505af1925050508015611d6f575060015b506000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6040517f496b9c1600000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff919091169063496b9c1690602401602060405180830381865afa158015611e0f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e339190615936565b90508015611e6b57600c80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055611e97565b600c80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660021790555b600c546040517f9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f91611ece9160ff909116906155da565b60405180910390a15050565b6114e9838383600061194c565b611eef6133fa565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c3214611f58576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036054013560e01c90565b600080600054700100000000000000000000000000000000900460ff166002811115611fbc57611fbc615571565b14611ff3576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001838154811061200857612008615871565b600091825260208220600590910201805490925063ffffffff9081161461207757815460018054909163ffffffff1690811061204657612046615871565b906000526020600020906005020160040160109054906101000a90046fffffffffffffffffffffffffffffffff1690505b60048201546000906120af90700100000000000000000000000000000000900467ffffffffffffffff165b67ffffffffffffffff1690565b6120c39067ffffffffffffffff164261591f565b6120e26120a2846fffffffffffffffffffffffffffffffff1660401c90565b67ffffffffffffffff166120f691906158cf565b905067ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff16116121435780611562565b7f000000000000000000000000000000000000000000000000000000000000000095945050505050565b60008061220c836fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690507f000000000000000000000000000000000000000000000000000000000000000081111561226b576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b642e90edd00062061a806311e1a30060006122868383615982565b9050670de0b6b3a764000060006122bd827f0000000000000000000000000000000000000000000000000000000000000000615996565b905060006122db6122d6670de0b6b3a764000086615996565b613edc565b905060006122e98484614137565b905060006122f78383614186565b90506000612304826141b4565b905060006123238261231e670de0b6b3a76400008f615996565b61439c565b905060006123318b83614186565b905061233d818d615996565b9f9e505050505050505050505050505050565b6001818154811061236057600080fd5b60009182526020909120600590910201805460018201546002830154600384015460049094015463ffffffff8416955064010000000090930473ffffffffffffffffffffffffffffffffffffffff908116949216926fffffffffffffffffffffffffffffffff91821692918082169170010000000000000000000000000000000090041687565b60006002600c5460ff16600281111561240257612402615571565b03612430575073ffffffffffffffffffffffffffffffffffffffff166000908152600a602052604090205490565b5073ffffffffffffffffffffffffffffffffffffffff1660009081526002602052604090205490565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c3314806124c9575061249a6114ee565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b6124ff576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117828686868686866143d6565b60008054700100000000000000000000000000000000900460ff16600281111561253957612539615571565b14612570576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008060008061257f86614a0c565b9350935093509350600061259585858585614e15565b90506000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015612614573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061263891906159d3565b9050600189036127335773ffffffffffffffffffffffffffffffffffffffff81166352f0f3ad8a84612697367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036034013590565b90565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815260048101939093526024830191909152604482015260206064820152608481018a905260a4015b6020604051808303816000875af1158015612709573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061272d91906159f0565b50611276565b6002890361275f5773ffffffffffffffffffffffffffffffffffffffff81166352f0f3ad8a8489612697565b6003890361278b5773ffffffffffffffffffffffffffffffffffffffff81166352f0f3ad8a8487612697565b60048903612831576040517f52f0f3ad000000000000000000000000000000000000000000000000000000008152600481018a905260248101839052367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036058013560c01b6044820152600860648201526084810188905273ffffffffffffffffffffffffffffffffffffffff8216906352f0f3ad9060a4016126ea565b6040517fff137e6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c9003605481013560e01c906014013560606128a461156b565b9050909192565b60028082015473ffffffffffffffffffffffffffffffffffffffff841660009081526020929092526040822080546fffffffffffffffffffffffffffffffff9092169290916128fb9084906158cf565b90915550505050565b604051818152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90038284820160208401378260208301016000815260208101604052505092915050565b60008054700100000000000000000000000000000000900460ff16600281111561298257612982615571565b146129b9576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000600184815481106129ce576129ce615871565b60009182526020918290206040805160e0810182526005909302909101805463ffffffff8116845273ffffffffffffffffffffffffffffffffffffffff64010000000090910481169484019490945260018101549093169082015260028201546fffffffffffffffffffffffffffffffff908116606083015260038301546080830181905260049093015480821660a084015270010000000000000000000000000000000090041660c082015291508514612ab5576040517f3014033200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60a0810151600083156fffffffffffffffffffffffffffffffff83161760011b90506000612b75826fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050861580612bb05750612bad7f000000000000000000000000000000000000000000000000000000000000000060026158cf565b81145b8015612bba575084155b15612bf1576040517fa42637bc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000811115612c4b576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612c767f000000000000000000000000000000000000000000000000000000000000000060016158cf565b8103612c8857612c8886888588614ecf565b34612c928361216d565b14612cc9576040517f8620aa1900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612cd488611f8e565b905067ffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000811690821603612d3c576040517f3381d11400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612d6960017f000000000000000000000000000000000000000000000000000000000000000061591f565b8303612eb757367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015612dea573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e0e91906159d3565b73ffffffffffffffffffffffffffffffffffffffff1663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e58573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e7c91906159f0565b612eb0907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615a09565b9050612f4a565b612ee260017f000000000000000000000000000000000000000000000000000000000000000061591f565b8303612f1d57612eb07f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615a35565b507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff165b612f7e817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615a65565b67ffffffffffffffff16612f998367ffffffffffffffff1690565b67ffffffffffffffff161115612fe057612fdd817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615a65565b91505b6000604083901b421760008a8152608087901b6fffffffffffffffffffffffffffffffff8d1617602052604081209192509060008181526003602052604090205490915060ff161561305e576040517f80497e3b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60016003600083815260200190815260200160002060006101000a81548160ff02191690831515021790555060016040518060e001604052808d63ffffffff168152602001600073ffffffffffffffffffffffffffffffffffffffff1681526020013373ffffffffffffffffffffffffffffffffffffffff168152602001346fffffffffffffffffffffffffffffffff1681526020018c8152602001886fffffffffffffffffffffffffffffffff168152602001846fffffffffffffffffffffffffffffffff16815250908060018154018082558091505060019003906000526020600020906005020160009091909190915060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060408201518160010160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060608201518160020160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055506080820151816003015560a08201518160040160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060c08201518160040160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055505050600460008c8152602001908152602001600020600180805490506132f3919061591f565b81546001810183556000928352602080842090910191909155338252600a90526040812080543492906133279084906158cf565b9091555050367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c73ffffffffffffffffffffffffffffffffffffffff1663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156133a457600080fd5b505af11580156133b8573d6000803e3d6000fd5b50506040513393508d92508e91507f9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be90600090a4505050505050505050505050565b60005471010000000000000000000000000000000000900460ff161561344c576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613454615089565b361461348c576040517f9824bdab00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600080367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c73ffffffffffffffffffffffffffffffffffffffff1663d83ef2676040518163ffffffff1660e01b81526004016040805180830381865afa158015613509573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061352d9190615a8e565b909250905081613569576040517f6a6bc3b200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7fffd7db0f9d5cdeb49c4c9eba649d4dc6d852d64671e65488e57f58584992ac686135be367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036014013590565b036135f5576040517f2cfac08200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080518082019091528281526020018190526007829055600881905567ffffffffffffffff367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015613696573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906136ba91906159d3565b73ffffffffffffffffffffffffffffffffffffffff1663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613704573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061372891906159f0565b1115613760576040517fb4e1243300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006137977f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615996565b90506000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015613816573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061383a91906159d3565b73ffffffffffffffffffffffffffffffffffffffff1663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613884573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906138a891906159f0565b6138dc907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166158cf565b905060006138ea838361509e565b905067ffffffffffffffff81111561392e576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff1611156139a6576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360d4013515613a0b576040517f223db39400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b83367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036058013511613aa5576040517ff40239db000000000000000000000000000000000000000000000000000000008152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036014013560048201526024015b60405180910390fd5b6040805160e08101825263ffffffff80825260006020808401828152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90038035606090811c8789018181526fffffffffffffffffffffffffffffffff34818116948b0194855260149095013560808b01908152600160a08c0181815242841660c08e0190815282548084018455928c529c517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6600590930292830180549a5191909d167fffffffffffffffff000000000000000000000000000000000000000000000000909a169990991764010000000073ffffffffffffffffffffffffffffffffffffffff9a8b160217909b5592517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf7840180547fffffffffffffffffffffffff000000000000000000000000000000000000000016919098161790965592517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf8820180547fffffffffffffffffffffffffffffffff000000000000000000000000000000001691851691909117905593517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf9850155955196519681167001000000000000000000000000000000009790911696909602959095177fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cfa909101558154710100000000000000000000000000000000007fffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffff909116178255918152600a909152918220805491929091613d269084906158cf565b9091555050367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c73ffffffffffffffffffffffffffffffffffffffff1663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b158015613da357600080fd5b505af1158015613db7573d6000803e3d6000fd5b5050600080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000164267ffffffffffffffff1617905550613dfa9150611f5a9050565b63ffffffff16367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c73ffffffffffffffffffffffffffffffffffffffff16633c9f397c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613e7b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613e9f9190615ab2565b600980547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001663ffffffff92909216929092141790555050505050565b6fffffffffffffffffffffffffffffffff811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b1760008213613f3b57631615e6386000526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f218311670de0b6b3a76400000215820261417457637c5f487d6000526004601cfd5b50670de0b6b3a7640000919091020490565b6000816000190483118202156141a45763bac65e5b6000526004601cfd5b50670de0b6b3a764000091020490565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d782136141e257919050565b680755bf798b4a1bf1e582126142005763a37bfec96000526004601cfd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b60006143cd670de0b6b3a7640000836143b486613edc565b6143be9190615ad8565b6143c89190615b94565b6141b4565b90505b92915050565b60008054700100000000000000000000000000000000900460ff16600281111561440257614402615571565b14614439576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001878154811061444e5761444e615871565b6000918252602082206005919091020160048101549092506fffffffffffffffffffffffffffffffff16908715821760011b90506144ad7f000000000000000000000000000000000000000000000000000000000000000060016158cf565b614549826fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1614614583576040517f5f53dd9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000808915614687576145d67f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000061591f565b6001901b6145f5846fffffffffffffffffffffffffffffffff166150b5565b6fffffffffffffffffffffffffffffffff166146119190615bfc565b1561464e5761464561463660016fffffffffffffffffffffffffffffffff8716615c10565b865463ffffffff166000615154565b6003015461467d565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c9003607801355b91508490506146b1565b600385015491506146ae6146366fffffffffffffffffffffffffffffffff86166001615c39565b90505b600882901b60088a8a6040516146c8929190615c64565b6040518091039020901b14614709576040517f696550ff00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006147148c615238565b90506000614723836003015490565b6040517fe14ced32000000000000000000000000000000000000000000000000000000008152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c9063e14ced3290614797908f908f908f908f908a90600401615cbd565b6020604051808303816000875af11580156147b6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906147da91906159f0565b600485015491149150600090600290614885906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b614921896fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b61492b9190615cf7565b6149359190615d1a565b60ff161590508115158103614976576040517ffb4e40dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8754640100000000900473ffffffffffffffffffffffffffffffffffffffff16156149cd576040517f9071e6af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505085547fffffffffffffffff0000000000000000000000000000000000000000ffffffff163364010000000002179095555050505050505050505050565b6000806000806000859050600060018281548110614a2c57614a2c615871565b600091825260209091206004600590920201908101549091507f000000000000000000000000000000000000000000000000000000000000000090614b03906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1611614b3d576040517fb34b5c2200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000815b60048301547f000000000000000000000000000000000000000000000000000000000000000090614c04906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169250821115614c7957825463ffffffff16614c437f000000000000000000000000000000000000000000000000000000000000000060016158cf565b8303614c4d578391505b60018181548110614c6057614c60615871565b9060005260206000209060050201935080945050614b41565b600481810154908401546fffffffffffffffffffffffffffffffff91821691166000816fffffffffffffffffffffffffffffffff16614ce2614ccd856fffffffffffffffffffffffffffffffff1660011c90565b6fffffffffffffffffffffffffffffffff1690565b6fffffffffffffffffffffffffffffffff161490508015614db1576000614d1a836fffffffffffffffffffffffffffffffff166150b5565b6fffffffffffffffffffffffffffffffff161115614d85576000614d5c614d5460016fffffffffffffffffffffffffffffffff8616615c10565b896001615154565b6003810154600490910154909c506fffffffffffffffffffffffffffffffff169a50614d8b9050565b6007549a505b600386015460048701549099506fffffffffffffffffffffffffffffffff169750614e07565b6000614dd3614d546fffffffffffffffffffffffffffffffff85166001615c39565b6003808901546004808b015492840154930154909e506fffffffffffffffffffffffffffffffff9182169d50919b50169850505b505050505050509193509193565b60006fffffffffffffffffffffffffffffffff841615614e825760408051602081018790526fffffffffffffffffffffffffffffffff8087169282019290925260608101859052908316608082015260a00160405160208183030381529060405280519060200120611562565b8282604051602001614eb09291909182526fffffffffffffffffffffffffffffffff16602082015260400190565b6040516020818303038152906040528051906020012095945050505050565b6000614eee6fffffffffffffffffffffffffffffffff84166001615c39565b90506000614efe82866001615154565b9050600086901a8380614fea5750614f3760027f0000000000000000000000000000000000000000000000000000000000000000615bfc565b6004830154600290614fdb906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b614fe59190615d1a565b60ff16145b156150425760ff811660011480615004575060ff81166002145b61503d576040517ff40239db00000000000000000000000000000000000000000000000000000000815260048101889052602401613a9c565b615080565b60ff811615615080576040517ff40239db00000000000000000000000000000000000000000000000000000000815260048101889052602401613a9c565b50505050505050565b6000615093615267565b6114d79060066158cf565b6000818310156150ae57816143cd565b5090919050565b600080615142837e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b600160ff919091161b90920392915050565b6000808261519d576151986fffffffffffffffffffffffffffffffff86167f0000000000000000000000000000000000000000000000000000000000000000615275565b6151b8565b6151b8856fffffffffffffffffffffffffffffffff16615401565b9050600184815481106151cd576151cd615871565b906000526020600020906005020191505b60048201546fffffffffffffffffffffffffffffffff82811691161461523057815460018054909163ffffffff1690811061521b5761521b615871565b906000526020600020906005020191506151de565b509392505050565b600080600080600061524986614a0c565b935093509350935061525d84848484614e15565b9695505050505050565b60006114d760f460286158cf565b600081615314846fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff161161532a5763b34b5c226000526004601cfd5b61533383615401565b9050816153d2826fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116143d0576143cd6153e88360016158cf565b6fffffffffffffffffffffffffffffffff8316906154a6565b60008119600183011681615495827e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169390931c8015179392505050565b600080615533847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050808303600180821b0385821b179250505092915050565b6000806040838503121561556257600080fd5b50508035926020909101359150565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b600381106155d7577f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b50565b602081016155e7836155a0565b91905290565b73ffffffffffffffffffffffffffffffffffffffff811681146155d757600080fd5b60006020828403121561562157600080fd5b813561562c816155ed565b9392505050565b60008060006060848603121561564857600080fd5b505081359360208301359350604090920135919050565b6000815180845260005b8181101561568557602081850181015186830182015201615669565b81811115615697576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b6020815260006143cd602083018461565f565b6000602082840312156156ef57600080fd5b5035919050565b80151581146155d757600080fd5b6000806000806080858703121561571a57600080fd5b843593506020850135925060408501359150606085013561573a816156f6565b939692955090935050565b60006020828403121561575757600080fd5b81356fffffffffffffffffffffffffffffffff8116811461562c57600080fd5b60008083601f84011261578957600080fd5b50813567ffffffffffffffff8111156157a157600080fd5b6020830191508360208285010111156157b957600080fd5b9250929050565b600080600080600080608087890312156157d957600080fd5b8635955060208701356157eb816156f6565b9450604087013567ffffffffffffffff8082111561580857600080fd5b6158148a838b01615777565b9096509450606089013591508082111561582d57600080fd5b5061583a89828a01615777565b979a9699509497509295939492505050565b63ffffffff84168152826020820152606060408201526000611562606083018461565f565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600082198211156158e2576158e26158a0565b500190565b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203615918576159186158a0565b5060010190565b600082821015615931576159316158a0565b500390565b60006020828403121561594857600080fd5b815161562c816156f6565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b60008261599157615991615953565b500490565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156159ce576159ce6158a0565b500290565b6000602082840312156159e557600080fd5b815161562c816155ed565b600060208284031215615a0257600080fd5b5051919050565b600067ffffffffffffffff808316818516808303821115615a2c57615a2c6158a0565b01949350505050565b600067ffffffffffffffff80831681851681830481118215151615615a5c57615a5c6158a0565b02949350505050565b600067ffffffffffffffff83811690831681811015615a8657615a866158a0565b039392505050565b60008060408385031215615aa157600080fd5b505080516020909101519092909150565b600060208284031215615ac457600080fd5b815163ffffffff8116811461562c57600080fd5b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615615b1957615b196158a0565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615615b5457615b546158a0565b60008712925087820587128484161615615b7057615b706158a0565b87850587128184161615615b8657615b866158a0565b505050929093029392505050565b600082615ba357615ba3615953565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83147f800000000000000000000000000000000000000000000000000000000000000083141615615bf757615bf76158a0565b500590565b600082615c0b57615c0b615953565b500690565b60006fffffffffffffffffffffffffffffffff83811690831681811015615a8657615a866158a0565b60006fffffffffffffffffffffffffffffffff808316818516808303821115615a2c57615a2c6158a0565b8183823760009101908152919050565b8183528181602085013750600060208284010152600060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b606081526000615cd1606083018789615c74565b8281036020840152615ce4818688615c74565b9150508260408301529695505050505050565b600060ff821660ff841680821015615d1157615d116158a0565b90039392505050565b600060ff831680615d2d57615d2d615953565b8060ff8416069150509291505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0a$8\x03\x80b\0a$\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01\xE7V[\x80b\0\0D`\x01`~b\0\x02\x81V[`\xFF\x16\x81`\0\x01Q\x11\x15b\0\0lW`@Qc;\xEF\xF1\x99`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x19\x81` \x01Q\x14\x80b\0\0\x93WP\x80Q` \x82\x01Qb\0\0\x90\x90`\x01b\0\x02\xA7V[\x10\x15[\x15b\0\0\xB2W`@Qc\xE6,\xCF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x81` \x01Q\x10\x15b\0\0\xD9W`@Qc\xE6,\xCF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0b\0\0\xFE\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16b\0\x01\xC7` \x1Bb\0&\x94\x17` \x1CV[b\0\x01\x14\x90`\x01`\x01`@\x1B\x03\x16`\x02b\0\x02\xC2V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01?W`@Qc#]\xFB+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\0\x01b\x82``\x01Q`\x01`\x01`@\x1B\x03\x16b\0\x01\xC7` \x1Bb\0&\x94\x17` \x1CV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15b\0\x01\x95W`@Qc#]\xFB+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q`\x80R` \x81\x01Q`\xA0R`@\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\xE0R``\x90\x91\x01Q\x16`\xC0RPb\0\x02\xE4V[\x90V[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x01\xE2W`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15b\0\x01\xFAW`\0\x80\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02+WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01Rb\0\x02L`@\x84\x01b\0\x01\xCAV[`@\x82\x01Rb\0\x02_``\x84\x01b\0\x01\xCAV[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\0\x02\x9EWb\0\x02\x9Eb\0\x02kV[\x90\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15b\0\x02\xBDWb\0\x02\xBDb\0\x02kV[P\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\0\x02\xDFWb\0\x02\xDFb\0\x02kV[P\x02\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa]Ib\0\x03\xDB`\09`\0\x81\x81a\x07\x1A\x01R\x81\x81a.\x82\x01R\x81\x81a.\xED\x01R\x81\x81a/ \x01R\x81\x81a7g\x01Ra8\xAE\x01R`\0\x81\x81a\x0BG\x01R\x81\x81a\r\x15\x01R\x81\x81a!\x03\x01R\x81\x81a!E\x01R\x81\x81a,\xE1\x01R\x81\x81a/P\x01R\x81\x81a/\xAF\x01Ra99\x01R`\0\x81\x81a\x0Bz\x01R\x81\x81a+\x87\x01R\x81\x81a,P\x01R\x81\x81a.\xBE\x01R\x81\x81aE\x91\x01R\x81\x81aJG\x01R\x81\x81aKH\x01R\x81\x81aL\x1D\x01R\x81\x81aO\x13\x01RaQt\x01R`\0\x81\x81a\x0C!\x01R\x81\x81a\"\x13\x01R\x81\x81a\"\x99\x01R\x81\x81a+\xF3\x01R\x81\x81a-E\x01R\x81\x81aD\x87\x01RaE\xB2\x01Ra]I`\0\xF3\xFE`\x80`@R`\x046\x10a\x03\x1EW`\x005`\xE0\x1C\x80cxk\x84K\x11a\x01\xA5W\x80c\xC0\xD8\xBBt\x11a\0\xECW\x80c\xDA\xBD9m\x11a\0\x95W\x80c\xF8\xF4?\xF6\x11a\0oW\x80c\xF8\xF4?\xF6\x14a\x0B\xCEW\x80c\xFA$\xF7C\x14a\x0B\xEEW\x80c\xFA1Z\xA9\x14a\x0C\x12W\x80c\xFE+\xBE\xB2\x14a\x0CEW`\0\x80\xFD[\x80c\xDA\xBD9m\x14a\x0B8W\x80c\xEC^c\x08\x14a\x0BkW\x80c\xEF\xF0\xF5\x92\x14a\x0B\x9EW`\0\x80\xFD[\x80c\xCF\t\xE0\xD0\x11a\0\xC6W\x80c\xCF\t\xE0\xD0\x14a\n\xD7W\x80c\xD5\xD4M\x80\x14a\n\xF8W\x80c\xD8\xCC\x1A<\x14a\x0B\x18W`\0\x80\xFD[\x80c\xC0\xD8\xBBt\x14a\n\0W\x80c\xC3\x95\xE1\xCA\x14a\n-W\x80c\xC6\xF00\x8C\x14a\nMW`\0\x80\xFD[\x80c\x99s^2\x11a\x01NW\x80c\xBB\xDC\x02\xDB\x11a\x01(W\x80c\xBB\xDC\x02\xDB\x14a\tUW\x80c\xBC\xEF;U\x14a\t\xA0W\x80c\xBD\x8D\xA9V\x14a\t\xE0W`\0\x80\xFD[\x80c\x99s^2\x14a\x08\x06W\x80c\xA4E\xEC\xE6\x14a\x08FW\x80c\xA8\xE4\xFB\x90\x14a\t\x12W`\0\x80\xFD[\x80c\x89\x80\xE0\xCC\x11a\x01\x7FW\x80c\x89\x80\xE0\xCC\x14a\x07\x81W\x80c\x8DE\n\x95\x14a\x07\x96W\x80c\x93\x8Dh\x9A\x14a\x07\xD6W`\0\x80\xFD[\x80cxk\x84K\x14a\x07QW\x80c{\x0F\n\xDC\x14a\x07fW\x80c\x81)\xFC\x1C\x14a\x07yW`\0\x80\xFD[\x80cG'w\xC6\x11a\x02iW\x80c\\\x0C\xBA3\x11a\x02\x12W\x80ccaPm\x11a\x01\xECW\x80ccaPm\x14a\x06\xCBW\x80ckg\x16\xC0\x14a\x07\x0BW\x80co\x03D\t\x14a\x07>W`\0\x80\xFD[\x80c\\\x0C\xBA3\x14a\x06SW\x80c`\x9D34\x14a\x06\x96W\x80c`\xE2td\x14a\x06\xABW`\0\x80\xFD[\x80cT\xFDMP\x11a\x02CW\x80cT\xFDMP\x14a\x05\xCFW\x80cY\xCE\xBE\t\x14a\x06\x1EW\x80cZ_\xA2\xD9\x14a\x063W`\0\x80\xFD[\x80cG'w\xC6\x14a\x05zW\x80cR\x9Dj\x8C\x14a\x05\x8DW\x80cSM\xB0\xE2\x14a\x05\xBAW`\0\x80\xFD[\x80c(\x10\xE1\xD6\x11a\x02\xCBW\x80c7\xB1\xB2)\x11a\x02\xA5W\x80c7\xB1\xB2)\x14a\x04\x93W\x80c:v\x84c\x14a\x04\xF4W\x80c?\xC8\xCE\xF3\x14a\x057W`\0\x80\xFD[\x80c(\x10\xE1\xD6\x14a\x04DW\x80c*\xD6\x9A\xEB\x14a\x04YW\x80c7\x8D\xD4\x8C\x14a\x04yW`\0\x80\xFD[\x80c\"*\xBFE\x11a\x02\xFCW\x80c\"*\xBFE\x14a\x03\xCBW\x80c%\x0Ei\xBD\x14a\x04\x0BW\x80c%\xFC*\xCE\x14a\x04%W`\0\x80\xFD[\x80c\x03\xC2\x92M\x14a\x03#W\x80c\x19\xEF\xFE\xB4\x14a\x03EW\x80c \r.\xD2\x14a\x03\x90W[`\0\x80\xFD[4\x80\x15a\x03/W`\0\x80\xFD[Pa\x03Ca\x03>6`\x04aUOV[a\x0CuV[\0[4\x80\x15a\x03QW`\0\x80\xFD[P`\0Ta\x03r\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x9CW`\0\x80\xFD[P`\0Ta\x03\xBE\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Qa\x03\x87\x91\x90aU\xDAV[4\x80\x15a\x03\xD7W`\0\x80\xFD[Pa\x03\xFBa\x03\xE66`\x04aV\x0FV[`\x0B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\x87V[4\x80\x15a\x04\x17W`\0\x80\xFD[P`\tTa\x03\xFB\x90`\xFF\x16\x81V[4\x80\x15a\x041W`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01a\x03\x87V[4\x80\x15a\x04PW`\0\x80\xFD[Pa\x03\xBEa\x12\x81V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x046a\x04t6`\x04aUOV[a\x14\xA6V[4\x80\x15a\x04\x85W`\0\x80\xFD[P`\x0CTa\x03\xBE\x90`\xFF\x16\x81V[4\x80\x15a\x04\x9FW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x035``\x1C[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\x87V[4\x80\x15a\x05\0W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Ca\x04\xCFV[4\x80\x15a\x05CW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Ca\x04\xCFV[a\x03Ca\x05\x886`\x04aV3V[a\x14\xDCV[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x046a\x05\xA86`\x04aV\x0FV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\xC6W`\0\x80\xFD[Pa\x04\xCFa\x14\xEEV[4\x80\x15a\x05\xDBW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F0.6.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03\x87\x91\x90aV\xCAV[4\x80\x15a\x06*W`\0\x80\xFD[P`\x08Ta\x046V[4\x80\x15a\x06?W`\0\x80\xFD[Pa\x046a\x06N6`\x04aV\xDDV[a\x151V[4\x80\x15a\x06_W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Ca\x04\xCFV[4\x80\x15a\x06\xA2W`\0\x80\xFD[Pa\x06\x11a\x15kV[4\x80\x15a\x06\xB7W`\0\x80\xFD[Pa\x03Ca\x06\xC66`\x04aV\x0FV[a\x15yV[4\x80\x15a\x06\xD7W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`4\x015a\x046V[4\x80\x15a\x07\x17W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03rV[a\x03Ca\x07L6`\x04aW\x04V[a\x19LV[4\x80\x15a\x07]W`\0\x80\xFD[Pa\x03Ca\x1A\x04V[a\x03Ca\x07t6`\x04aV3V[a\x1E\xDAV[a\x03Ca\x1E\xE7V[4\x80\x15a\x07\x8DW`\0\x80\xFD[P`\x01Ta\x046V[4\x80\x15a\x07\xA2W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`x\x015a\x046V[4\x80\x15a\x07\xE2W`\0\x80\xFD[P`\x07T`\x08Ta\x07\xF1\x91\x90\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\x87V[4\x80\x15a\x08\x12W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`X\x015a\x046V[4\x80\x15a\x08RW`\0\x80\xFD[Pa\x08\xBEa\x08a6`\x04aV\xDDV[`\x06` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x82\x16\x91a\x01\0\x81\x04c\xFF\xFF\xFF\xFF\x16\x91e\x01\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84V[`@\x80Q\x94\x15\x15\x85Rc\xFF\xFF\xFF\xFF\x90\x93\x16` \x85\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91\x83\x01\x91\x90\x91Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x80\x01a\x03\x87V[4\x80\x15a\t\x1EW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1Ca\x04\xCFV[4\x80\x15a\taW`\0\x80\xFD[P`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x81R` \x01a\x03\x87V[4\x80\x15a\t\xACW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x14\x015a\x046V[4\x80\x15a\t\xECW`\0\x80\xFD[Pa\x03ra\t\xFB6`\x04aV\xDDV[a\x1F\x8EV[4\x80\x15a\n\x0CW`\0\x80\xFD[Pa\x046a\n\x1B6`\x04aV\x0FV[`\n` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\n9W`\0\x80\xFD[Pa\x046a\nH6`\x04aWEV[a!mV[4\x80\x15a\nYW`\0\x80\xFD[Pa\nma\nh6`\x04aV\xDDV[a#PV[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x98\x16\x88Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16` \x89\x01R\x95\x90\x94\x16\x94\x86\x01\x94\x90\x94Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16``\x86\x01R`\x80\x85\x01R\x91\x82\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\x87V[4\x80\x15a\n\xE3W`\0\x80\xFD[P`\0Ta\x03r\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x0B\x04W`\0\x80\xFD[Pa\x046a\x0B\x136`\x04aV\x0FV[a#\xE7V[4\x80\x15a\x0B$W`\0\x80\xFD[Pa\x03Ca\x0B36`\x04aW\xC0V[a$YV[4\x80\x15a\x0BDW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03rV[4\x80\x15a\x0BwW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x046V[4\x80\x15a\x0B\xAAW`\0\x80\xFD[Pa\x03\xFBa\x0B\xB96`\x04aV\xDDV[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\xDAW`\0\x80\xFD[Pa\x03Ca\x0B\xE96`\x04aV3V[a%\rV[4\x80\x15a\x0B\xFAW`\0\x80\xFD[Pa\x0C\x03a(cV[`@Qa\x03\x87\x93\x92\x91\x90aXLV[4\x80\x15a\x0C\x1EW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x046V[4\x80\x15a\x0CQW`\0\x80\xFD[Pa\x03\xFBa\x0C`6`\x04aV\xDDV[`\x05` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0C\xA1Wa\x0C\xA1aUqV[\x14a\x0C\xD8W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x83\x81T\x81\x10a\x0C\xEDWa\x0C\xEDaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x90P`\0a\r\x08\x84a\x1F\x8EV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x10\x15a\rqW`@Q\x7F\xF2D\x0BS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x05` R`@\x90 T`\xFF\x16\x15a\r\xBAW`@Q\x7F\xF1\xA9E\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x04` R`@\x90 \x80T\x80\x15\x80\x15a\r\xD7WP\x85\x15\x15[\x15a\x0ErW\x83Td\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15a\x0E\nW\x81a\x0E&V[`\x01\x86\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x90Pa\x0E2\x81\x87a(\xABV[PPP`\0\x94\x85RPP`\x05` RPP`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UV[`\0\x86\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x82\x04c\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x81\x01\x93\x90\x93R`\x01\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x83\x01Ra\x0F\x15Wo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01R`\x01\x81R`\0\x86\x90\x03a\x0F\x15W\x81\x95P[`\0\x86\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a\x0F-\x91\x90aX\xCFV[\x90P`\0\x83\x82\x11a\x0F>W\x81a\x0F@V[\x83[` \x84\x01Q\x90\x91Pc\xFF\xFF\xFF\xFF\x16[\x81\x81\x10\x15a\x10\x8CW`\0\x86\x82\x81T\x81\x10a\x0FkWa\x0FkaXqV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83R`\x05\x90\x91R`@\x90\x91 T\x90\x91P`\xFF\x16a\x0F\xC3W`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x82\x81T\x81\x10a\x0F\xD8Wa\x0F\xD8aXqV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T\x90\x91Pd\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x105WP`\x04\x81\x01T`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11[\x15a\x10wW`\x01\x81\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01R`\x04\x81\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x87\x01R[PP\x80\x80a\x10\x84\x90aX\xE7V[\x91PPa\x0FOV[Pc\xFF\xFF\xFF\xFF\x81\x81\x16` \x85\x81\x01\x91\x82R`\0\x8C\x81R`\x06\x90\x91R`@\x90\x81\x90 \x86Q\x81T\x93Q\x92\x88\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x94\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\x16\x17a\x01\0\x92\x90\x94\x16\x91\x82\x02\x93\x90\x93\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x82U``\x85\x01Q`\x01\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x84\x90\x03a\x12vW``\x83\x01Q`\0\x8A\x81R`\x05` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x12-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a\x12\x0BW\x81a\x12'V[`\x01\x89\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x89a(\xABV[\x87Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x87U[PPPPPPPPPV[`\0\x80`\0Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x12\xAFWa\x12\xAFaUqV[\x14a\x12\xE6W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80R`\x05` R\x7F\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBCT`\xFF\x16a\x13JW`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\0\x81T\x81\x10a\x13vWa\x13vaXqV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01Td\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\xB1W`\x01a\x13\xB4V[`\x02[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x17\x83U\x92\x93P\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\x02\x81\x11\x15a\x14eWa\x14eaUqV[\x02\x17\x90U`\x02\x81\x11\x15a\x14zWa\x14zaUqV[`@Q\x7F^\x18o\t\xB9\xC94\x91\xF1N'~\xEA\x7F\xAA]\xE6\xA2\xD4\xBD\xA7Zy\xAFz6\x84\xFB\xFBB\xDA`\x90`\0\x90\xA2\x90V[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x14\xC2W`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[\x90P\x90V[a\x14\xE9\x83\x83\x83`\x01a\x19LV[PPPV[`\0a\x14\xD7a\x14\xFF`\xF4`\x14aX\xCFV[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`\0\x81\x81R`\x06` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x80T\x82Ta\x15b\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aY\x1FV[\x95\x94PPPPPV[``a\x14\xD7`X` a)\x04V[a\x15\x81a\x1A\x04V[`\0`\x02`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x15\x9CWa\x15\x9CaUqV[\x03a\x15\xCDWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\n` R`@\x90 Ta\x16IV[`\x01`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x15\xE6Wa\x15\xE6aUqV[\x03a\x16\x17WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x16IV[`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16a\x17\x8AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x16\xFC`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE6\x90\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`@Q\x7F~\xEE(\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c~\xEE(\x8D\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x82W=`\0\x80>=`\0\xFD[PPPPPPV[\x80`\0\x03a\x17\xC4W`@Q\x7F\x17\xBF\xE5\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x83\x90U`\x02\x90\x91R\x81 U6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x9AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xAEW=`\0\x80>=`\0\xFD[PPPP`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19\x0CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\x11V[``\x91P[PP\x90P\x80a\x14\xE9W`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a\x19\xBCWPa\x19\x8Da\x14\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x19\xF2W`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\xFE\x84\x84\x84\x84a)VV[PPPPV[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA3\x91\x90aY6V[\x15a\x1A\xDAW`@Q\x7F7\x9A~\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x1A\xF3Wa\x1A\xF3aUqV[\x14\x80a\x1B\x15WP`\x01`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x1B\x13Wa\x1B\x13aUqV[\x14[\x15a\x1B\x1CWV[`\0`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x1B5Wa\x1B5aUqV[\x14a\x1BlW`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B\xC8W`@Q\x7F\xC1\x05&\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x03\x14\xD2\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90c\x03\x14\xD2\xB3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x8B\x91\x90aY6V[\x90P\x80a\x1C\xC4W`@Q\x7FHQ\xBD\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x17\xCF!\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90c\x17\xCF!\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D^W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x1DoWP`\x01[P`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E3\x91\x90aY6V[\x90P\x80\x15a\x1EkW`\x0C\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x1E\x97V[`\x0C\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x02\x17\x90U[`\x0CT`@Q\x7F\x99\x08\xEA\xAC\x06E\xDF\x9D\x07\x04\xD0j\xDC\x9E\x073|\x95\x1D\xE2\xF0k_(6\x15\x1DH\xD5\xE4r/\x91a\x1E\xCE\x91`\xFF\x90\x91\x16\x90aU\xDAV[`@Q\x80\x91\x03\x90\xA1PPV[a\x14\xE9\x83\x83\x83`\0a\x19LV[a\x1E\xEFa3\xFAV[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C2\x14a\x1FXW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x90V[`\0\x80`\0Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1F\xBCWa\x1F\xBCaUqV[\x14a\x1F\xF3W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x83\x81T\x81\x10a \x08Wa \x08aXqV[`\0\x91\x82R` \x82 `\x05\x90\x91\x02\x01\x80T\x90\x92Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x14a wW\x81T`\x01\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10a FWa FaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01`\x04\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\x04\x82\x01T`\0\x90a \xAF\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a \xC3\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaY\x1FV[a \xE2a \xA2\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a \xF6\x91\x90aX\xCFV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a!CW\x80a\x15bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`\0\x80a\"\x0C\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\"kW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d.\x90\xED\xD0\0b\x06\x1A\x80c\x11\xE1\xA3\0`\0a\"\x86\x83\x83aY\x82V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0a\"\xBD\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x96V[\x90P`\0a\"\xDBa\"\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x86aY\x96V[a>\xDCV[\x90P`\0a\"\xE9\x84\x84aA7V[\x90P`\0a\"\xF7\x83\x83aA\x86V[\x90P`\0a#\x04\x82aA\xB4V[\x90P`\0a##\x82a#\x1Eg\r\xE0\xB6\xB3\xA7d\0\0\x8FaY\x96V[aC\x9CV[\x90P`\0a#1\x8B\x83aA\x86V[\x90Pa#=\x81\x8DaY\x96V[\x9F\x9EPPPPPPPPPPPPPPPV[`\x01\x81\x81T\x81\x10a#`W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01Tc\xFF\xFF\xFF\xFF\x84\x16\x95Pd\x01\0\0\0\0\x90\x93\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x92\x16\x92o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x80\x82\x16\x91p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x87V[`\0`\x02`\x0CT`\xFF\x16`\x02\x81\x11\x15a$\x02Wa$\x02aUqV[\x03a$0WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\n` R`@\x90 T\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a$\xC9WPa$\x9Aa\x14\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a$\xFFW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\x82\x86\x86\x86\x86\x86\x86aC\xD6V[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a%9Wa%9aUqV[\x14a%pW`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80a%\x7F\x86aJ\x0CV[\x93P\x93P\x93P\x93P`\0a%\x95\x85\x85\x85\x85aN\x15V[\x90P`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&8\x91\x90aY\xD3V[\x90P`\x01\x89\x03a'3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16cR\xF0\xF3\xAD\x8A\x84a&\x976\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`4\x015\x90V[\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R`D\x82\x01R` `d\x82\x01R`\x84\x81\x01\x8A\x90R`\xA4\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a'\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'-\x91\x90aY\xF0V[Pa\x12vV[`\x02\x89\x03a'_Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16cR\xF0\xF3\xAD\x8A\x84\x89a&\x97V[`\x03\x89\x03a'\x8BWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16cR\xF0\xF3\xAD\x8A\x84\x87a&\x97V[`\x04\x89\x03a(1W`@Q\x7FR\xF0\xF3\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`X\x015`\xC0\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90cR\xF0\xF3\xAD\x90`\xA4\x01a&\xEAV[`@Q\x7F\xFF\x13~e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`T\x81\x015`\xE0\x1C\x90`\x14\x015``a(\xA4a\x15kV[\x90P\x90\x91\x92V[`\x02\x80\x82\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R` \x92\x90\x92R`@\x82 \x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92\x90\x91a(\xFB\x90\x84\x90aX\xCFV[\x90\x91UPPPPV[`@Q\x81\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x82\x84\x82\x01` \x84\x017\x82` \x83\x01\x01`\0\x81R` \x81\x01`@RPP\x92\x91PPV[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a)\x82Wa)\x82aUqV[\x14a)\xB9W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x84\x81T\x81\x10a)\xCEWa)\xCEaXqV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x81\x16\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFd\x01\0\0\0\0\x90\x91\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01\x81\x01T\x90\x93\x16\x90\x82\x01R`\x02\x82\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x01R`\x03\x83\x01T`\x80\x83\x01\x81\x90R`\x04\x90\x93\x01T\x80\x82\x16`\xA0\x84\x01Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16`\xC0\x82\x01R\x91P\x85\x14a*\xB5W`@Q\x7F0\x14\x032\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0\x81\x01Q`\0\x83\x15o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17`\x01\x1B\x90P`\0a+u\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x86\x15\x80a+\xB0WPa+\xAD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02aX\xCFV[\x81\x14[\x80\x15a+\xBAWP\x84\x15[\x15a+\xF1W`@Q\x7F\xA4&7\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a,KW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aX\xCFV[\x81\x03a,\x88Wa,\x88\x86\x88\x85\x88aN\xCFV[4a,\x92\x83a!mV[\x14a,\xC9W`@Q\x7F\x86 \xAA\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a,\xD4\x88a\x1F\x8EV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a-<W`@Q\x7F3\x81\xD1\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a-i`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x1FV[\x83\x03a.\xB7W6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x0E\x91\x90aY\xD3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.|\x91\x90aY\xF0V[a.\xB0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZ\tV[\x90Pa/JV[a.\xE2`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x1FV[\x83\x03a/\x1DWa.\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aZ5V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[a/~\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZeV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a/\x99\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a/\xE0Wa/\xDD\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZeV[\x91P[`\0`@\x83\x90\x1BB\x17`\0\x8A\x81R`\x80\x87\x90\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x16\x17` R`@\x81 \x91\x92P\x90`\0\x81\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16\x15a0^W`@Q\x7F\x80I~;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`@Q\x80`\xE0\x01`@R\x80\x8Dc\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x013s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x014o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8C\x81R` \x01\x88o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP`\x04`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\x01\x80\x80T\x90Pa2\xF3\x91\x90aY\x1FV[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x91\x01\x91\x90\x91U3\x82R`\n\x90R`@\x81 \x80T4\x92\x90a3'\x90\x84\x90aX\xCFV[\x90\x91UPP6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a3\xA4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a3\xB8W=`\0\x80>=`\0\xFD[PP`@Q3\x93P\x8D\x92P\x8E\x91P\x7F\x9B2Et\x0E\xC3\xB1U\t\x8AU\xBE\x84\x95zM\xA1>\xAF\x7F\x14\xA8\xBCoS\x12l\x0B\x93P\xF2\xBE\x90`\0\x90\xA4PPPPPPPPPPPPV[`\0Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a4LW`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a4TaP\x89V[6\x14a4\x8CW`@Q\x7F\x98$\xBD\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x806\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8>\xF2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5-\x91\x90aZ\x8EV[\x90\x92P\x90P\x81a5iW`@Q\x7Fjk\xC3\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xFF\xD7\xDB\x0F\x9D\\\xDE\xB4\x9CL\x9E\xBAd\x9DM\xC6\xD8R\xD6Fq\xE6T\x88\xE5\x7FXXI\x92\xACha5\xBE6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x14\x015\x90V[\x03a5\xF5W`@Q\x7F,\xFA\xC0\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x01\x81\x90R`\x07\x82\x90U`\x08\x81\x90Ug\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xBA\x91\x90aY\xD3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7(\x91\x90aY\xF0V[\x11\x15a7`W`@Q\x7F\xB4\xE1$3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a7\x97\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aY\x96V[\x90P`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8:\x91\x90aY\xD3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xA8\x91\x90aY\xF0V[a8\xDC\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aX\xCFV[\x90P`\0a8\xEA\x83\x83aP\x9EV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9.W`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a9\xA6W`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xD4\x015\x15a:\x0BW`@Q\x7F\"=\xB3\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x836\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`X\x015\x11a:\xA5W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x14\x015`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x80\x82R`\0` \x80\x84\x01\x82\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x805``\x90\x81\x1C\x87\x89\x01\x81\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF4\x81\x81\x16\x94\x8B\x01\x94\x85R`\x14\x90\x95\x015`\x80\x8B\x01\x90\x81R`\x01`\xA0\x8C\x01\x81\x81RB\x84\x16`\xC0\x8E\x01\x90\x81R\x82T\x80\x84\x01\x84U\x92\x8CR\x9CQ\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6`\x05\x90\x93\x02\x92\x83\x01\x80T\x9AQ\x91\x90\x9D\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x9A\x16\x99\x90\x99\x17d\x01\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9A\x8B\x16\x02\x17\x90\x9BU\x92Q\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x84\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x98\x16\x17\x90\x96U\x92Q\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF8\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF9\x85\x01U\x95Q\x96Q\x96\x81\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x90\x91\x16\x96\x90\x96\x02\x95\x90\x95\x17\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xFA\x90\x91\x01U\x81Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U\x91\x81R`\n\x90\x91R\x91\x82 \x80T\x91\x92\x90\x91a=&\x90\x84\x90aX\xCFV[\x90\x91UPP6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a=\xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a=\xB7W=`\0\x80>=`\0\xFD[PP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPa=\xFA\x91Pa\x1FZ\x90PV[c\xFF\xFF\xFF\xFF\x166\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x9F9|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\x9F\x91\x90aZ\xB2V[`\t\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x92\x90\x92\x14\x17\x90UPPPPPV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17`\0\x82\x13a?;Wc\x16\x15\xE68`\0R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[`\0x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x83\x11g\r\xE0\xB6\xB3\xA7d\0\0\x02\x15\x82\x02aAtWc|_H}`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x02\x15aA\xA4Wc\xBA\xC6^[`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13aA\xE2W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12aB\0Wc\xA3{\xFE\xC9`\0R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0aC\xCDg\r\xE0\xB6\xB3\xA7d\0\0\x83aC\xB4\x86a>\xDCV[aC\xBE\x91\x90aZ\xD8V[aC\xC8\x91\x90a[\x94V[aA\xB4V[\x90P[\x92\x91PPV[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15aD\x02WaD\x02aUqV[\x14aD9W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x87\x81T\x81\x10aDNWaDNaXqV[`\0\x91\x82R` \x82 `\x05\x91\x90\x91\x02\x01`\x04\x81\x01T\x90\x92Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x87\x15\x82\x17`\x01\x1B\x90PaD\xAD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aX\xCFV[aEI\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x14aE\x83W`@Q\x7F_S\xDD\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x89\x15aF\x87WaE\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x1FV[`\x01\x90\x1BaE\xF5\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aP\xB5V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aF\x11\x91\x90a[\xFCV[\x15aFNWaFEaF6`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16a\\\x10V[\x86Tc\xFF\xFF\xFF\xFF\x16`\0aQTV[`\x03\x01TaF}V[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`x\x015[\x91P\x84\x90PaF\xB1V[`\x03\x85\x01T\x91PaF\xAEaF6o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x01a\\9V[\x90P[`\x08\x82\x90\x1B`\x08\x8A\x8A`@QaF\xC8\x92\x91\x90a\\dV[`@Q\x80\x91\x03\x90 \x90\x1B\x14aG\tW`@Q\x7FieP\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0aG\x14\x8CaR8V[\x90P`\0aG#\x83`\x03\x01T\x90V[`@Q\x7F\xE1L\xED2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C\x90c\xE1L\xED2\x90aG\x97\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8A\x90`\x04\x01a\\\xBDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aG\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xDA\x91\x90aY\xF0V[`\x04\x85\x01T\x91\x14\x91P`\0\x90`\x02\x90aH\x85\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aI!\x89o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aI+\x91\x90a\\\xF7V[aI5\x91\x90a]\x1AV[`\xFF\x16\x15\x90P\x81\x15\x15\x81\x03aIvW`@Q\x7F\xFBN@\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87Td\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15aI\xCDW`@Q\x7F\x90q\xE6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x163d\x01\0\0\0\0\x02\x17\x90\x95UPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x85\x90P`\0`\x01\x82\x81T\x81\x10aJ,WaJ,aXqV[`\0\x91\x82R` \x90\x91 `\x04`\x05\x90\x92\x02\x01\x90\x81\x01T\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aK\x03\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aK=W`@Q\x7F\xB3K\\\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[`\x04\x83\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aL\x04\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x92P\x82\x11\x15aLyW\x82Tc\xFF\xFF\xFF\xFF\x16aLC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aX\xCFV[\x83\x03aLMW\x83\x91P[`\x01\x81\x81T\x81\x10aL`WaL`aXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x93P\x80\x94PPaKAV[`\x04\x81\x81\x01T\x90\x84\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16`\0\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aL\xE2aL\xCD\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x1C\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x90P\x80\x15aM\xB1W`\0aM\x1A\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aP\xB5V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aM\x85W`\0aM\\aMT`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16a\\\x10V[\x89`\x01aQTV[`\x03\x81\x01T`\x04\x90\x91\x01T\x90\x9CPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9APaM\x8B\x90PV[`\x07T\x9AP[`\x03\x86\x01T`\x04\x87\x01T\x90\x99Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x97PaN\x07V[`\0aM\xD3aMTo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x01a\\9V[`\x03\x80\x89\x01T`\x04\x80\x8B\x01T\x92\x84\x01T\x93\x01T\x90\x9EPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x9DP\x91\x9BP\x16\x98PP[PPPPPPP\x91\x93P\x91\x93V[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15aN\x82W`@\x80Q` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R\x90\x83\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x15bV[\x82\x82`@Q` \x01aN\xB0\x92\x91\x90\x91\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95\x94PPPPPV[`\0aN\xEEo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x01a\\9V[\x90P`\0aN\xFE\x82\x86`\x01aQTV[\x90P`\0\x86\x90\x1A\x83\x80aO\xEAWPaO7`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a[\xFCV[`\x04\x83\x01T`\x02\x90aO\xDB\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aO\xE5\x91\x90a]\x1AV[`\xFF\x16\x14[\x15aPBW`\xFF\x81\x16`\x01\x14\x80aP\x04WP`\xFF\x81\x16`\x02\x14[aP=W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a:\x9CV[aP\x80V[`\xFF\x81\x16\x15aP\x80W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a:\x9CV[PPPPPPPV[`\0aP\x93aRgV[a\x14\xD7\x90`\x06aX\xCFV[`\0\x81\x83\x10\x15aP\xAEW\x81aC\xCDV[P\x90\x91\x90PV[`\0\x80aQB\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01`\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x80\x82aQ\x9DWaQ\x98o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aRuV[aQ\xB8V[aQ\xB8\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aT\x01V[\x90P`\x01\x84\x81T\x81\x10aQ\xCDWaQ\xCDaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91P[`\x04\x82\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x91\x16\x14aR0W\x81T`\x01\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10aR\x1BWaR\x1BaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91PaQ\xDEV[P\x93\x92PPPV[`\0\x80`\0\x80`\0aRI\x86aJ\x0CV[\x93P\x93P\x93P\x93PaR]\x84\x84\x84\x84aN\x15V[\x96\x95PPPPPPV[`\0a\x14\xD7`\xF4`(aX\xCFV[`\0\x81aS\x14\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aS*Wc\xB3K\\\"`\0R`\x04`\x1C\xFD[aS3\x83aT\x01V[\x90P\x81aS\xD2\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aC\xD0WaC\xCDaS\xE8\x83`\x01aX\xCFV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90aT\xA6V[`\0\x81\x19`\x01\x83\x01\x16\x81aT\x95\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`\0\x80aU3\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x80\x82\x1B\x03\x85\x82\x1B\x17\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aUbW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aU\xD7W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01aU\xE7\x83aU\xA0V[\x91\x90R\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aU\xD7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aV!W`\0\x80\xFD[\x815aV,\x81aU\xEDV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aVHW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aV\x85W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aViV[\x81\x81\x11\x15aV\x97W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0aC\xCD` \x83\x01\x84aV_V[`\0` \x82\x84\x03\x12\x15aV\xEFW`\0\x80\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14aU\xD7W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aW\x1AW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aW:\x81aV\xF6V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aWWW`\0\x80\xFD[\x815o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aV,W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aW\x89W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xA1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aW\xB9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aW\xD9W`\0\x80\xFD[\x865\x95P` \x87\x015aW\xEB\x81aV\xF6V[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aX\x08W`\0\x80\xFD[aX\x14\x8A\x83\x8B\x01aWwV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aX-W`\0\x80\xFD[PaX:\x89\x82\x8A\x01aWwV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x15b``\x83\x01\x84aV_V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aX\xE2WaX\xE2aX\xA0V[P\x01\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aY\x18WaY\x18aX\xA0V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aY1WaY1aX\xA0V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aYHW`\0\x80\xFD[\x81QaV,\x81aV\xF6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aY\x91WaY\x91aYSV[P\x04\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aY\xCEWaY\xCEaX\xA0V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aY\xE5W`\0\x80\xFD[\x81QaV,\x81aU\xEDV[`\0` \x82\x84\x03\x12\x15aZ\x02W`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aZ,WaZ,aX\xA0V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aZ\\WaZ\\aX\xA0V[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aZ\x86WaZ\x86aX\xA0V[\x03\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aZ\xA1W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15aZ\xC4W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14aV,W`\0\x80\xFD[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a[\x19Wa[\x19aX\xA0V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a[TWa[TaX\xA0V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a[pWa[paX\xA0V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a[\x86Wa[\x86aX\xA0V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82a[\xA3Wa[\xA3aYSV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a[\xF7Wa[\xF7aX\xA0V[P\x05\x90V[`\0\x82a\\\x0BWa\\\x0BaYSV[P\x06\x90V[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aZ\x86WaZ\x86aX\xA0V[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aZ,WaZ,aX\xA0V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0a\\\xD1``\x83\x01\x87\x89a\\tV[\x82\x81\x03` \x84\x01Ra\\\xE4\x81\x86\x88a\\tV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a]\x11Wa]\x11aX\xA0V[\x90\x03\x93\x92PPPV[`\0`\xFF\x83\x16\x80a]-Wa]-aYSV[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061031e5760003560e01c8063786b844b116101a5578063c0d8bb74116100ec578063dabd396d11610095578063f8f43ff61161006f578063f8f43ff614610bce578063fa24f74314610bee578063fa315aa914610c12578063fe2bbeb214610c4557600080fd5b8063dabd396d14610b38578063ec5e630814610b6b578063eff0f59214610b9e57600080fd5b8063cf09e0d0116100c6578063cf09e0d014610ad7578063d5d44d8014610af8578063d8cc1a3c14610b1857600080fd5b8063c0d8bb7414610a00578063c395e1ca14610a2d578063c6f0308c14610a4d57600080fd5b806399735e321161014e578063bbdc02db11610128578063bbdc02db14610955578063bcef3b55146109a0578063bd8da956146109e057600080fd5b806399735e3214610806578063a445ece614610846578063a8e4fb901461091257600080fd5b80638980e0cc1161017f5780638980e0cc146107815780638d450a9514610796578063938d689a146107d657600080fd5b8063786b844b146107515780637b0f0adc146107665780638129fc1c1461077957600080fd5b8063472777c6116102695780635c0cba33116102125780636361506d116101ec5780636361506d146106cb5780636b6716c01461070b5780636f0344091461073e57600080fd5b80635c0cba3314610653578063609d33341461069657806360e27464146106ab57600080fd5b806354fd4d501161024357806354fd4d50146105cf57806359cebe091461061e5780635a5fa2d91461063357600080fd5b8063472777c61461057a578063529d6a8c1461058d578063534db0e2146105ba57600080fd5b80632810e1d6116102cb57806337b1b229116102a557806337b1b229146104935780633a768463146104f45780633fc8cef31461053757600080fd5b80632810e1d6146104445780632ad69aeb14610459578063378dd48c1461047957600080fd5b8063222abf45116102fc578063222abf45146103cb578063250e69bd1461040b57806325fc2ace1461042557600080fd5b806303c2924d1461032357806319effeb414610345578063200d2ed214610390575b600080fd5b34801561032f57600080fd5b5061034361033e36600461554f565b610c75565b005b34801561035157600080fd5b506000546103729068010000000000000000900467ffffffffffffffff1681565b60405167ffffffffffffffff90911681526020015b60405180910390f35b34801561039c57600080fd5b506000546103be90700100000000000000000000000000000000900460ff1681565b60405161038791906155da565b3480156103d757600080fd5b506103fb6103e636600461560f565b600b6020526000908152604090205460ff1681565b6040519015158152602001610387565b34801561041757600080fd5b506009546103fb9060ff1681565b34801561043157600080fd5b506007545b604051908152602001610387565b34801561045057600080fd5b506103be611281565b34801561046557600080fd5b5061043661047436600461554f565b6114a6565b34801561048557600080fd5b50600c546103be9060ff1681565b34801561049f57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90033560601c5b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610387565b34801561050057600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c6104cf565b34801561054357600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c6104cf565b610343610588366004615633565b6114dc565b34801561059957600080fd5b506104366105a836600461560f565b60026020526000908152604090205481565b3480156105c657600080fd5b506104cf6114ee565b3480156105db57600080fd5b5060408051808201909152600581527f302e362e3000000000000000000000000000000000000000000000000000000060208201525b60405161038791906156ca565b34801561062a57600080fd5b50600854610436565b34801561063f57600080fd5b5061043661064e3660046156dd565b611531565b34801561065f57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6104cf565b3480156106a257600080fd5b5061061161156b565b3480156106b757600080fd5b506103436106c636600461560f565b611579565b3480156106d757600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360340135610436565b34801561071757600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610372565b61034361074c366004615704565b61194c565b34801561075d57600080fd5b50610343611a04565b610343610774366004615633565b611eda565b610343611ee7565b34801561078d57600080fd5b50600154610436565b3480156107a257600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360780135610436565b3480156107e257600080fd5b506007546008546107f1919082565b60408051928352602083019190915201610387565b34801561081257600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360580135610436565b34801561085257600080fd5b506108be6108613660046156dd565b6006602052600090815260409020805460019091015460ff821691610100810463ffffffff1691650100000000009091046fffffffffffffffffffffffffffffffff169073ffffffffffffffffffffffffffffffffffffffff1684565b60408051941515855263ffffffff90931660208501526fffffffffffffffffffffffffffffffff9091169183019190915273ffffffffffffffffffffffffffffffffffffffff166060820152608001610387565b34801561091e57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c6104cf565b34801561096157600080fd5b50604051367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036054013560e01c8152602001610387565b3480156109ac57600080fd5b50367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360140135610436565b3480156109ec57600080fd5b506103726109fb3660046156dd565b611f8e565b348015610a0c57600080fd5b50610436610a1b36600461560f565b600a6020526000908152604090205481565b348015610a3957600080fd5b50610436610a48366004615745565b61216d565b348015610a5957600080fd5b50610a6d610a683660046156dd565b612350565b6040805163ffffffff909816885273ffffffffffffffffffffffffffffffffffffffff968716602089015295909416948601949094526fffffffffffffffffffffffffffffffff9182166060860152608085015291821660a08401521660c082015260e001610387565b348015610ae357600080fd5b506000546103729067ffffffffffffffff1681565b348015610b0457600080fd5b50610436610b1336600461560f565b6123e7565b348015610b2457600080fd5b50610343610b333660046157c0565b612459565b348015610b4457600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610372565b348015610b7757600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610436565b348015610baa57600080fd5b506103fb610bb93660046156dd565b60036020526000908152604090205460ff1681565b348015610bda57600080fd5b50610343610be9366004615633565b61250d565b348015610bfa57600080fd5b50610c03612863565b6040516103879392919061584c565b348015610c1e57600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610436565b348015610c5157600080fd5b506103fb610c603660046156dd565b60056020526000908152604090205460ff1681565b60008054700100000000000000000000000000000000900460ff166002811115610ca157610ca1615571565b14610cd8576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600060018381548110610ced57610ced615871565b906000526020600020906005020190506000610d0884611f8e565b905067ffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081169082161015610d71576040517ff2440b5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008481526005602052604090205460ff1615610dba576040517ff1a9458100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000848152600460205260409020805480158015610dd757508515155b15610e72578354640100000000900473ffffffffffffffffffffffffffffffffffffffff1660008115610e0a5781610e26565b600186015473ffffffffffffffffffffffffffffffffffffffff165b9050610e3281876128ab565b50505060009485525050600560205250506040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055565b6000868152600660209081526040918290208251608081018452815460ff81161515808352610100820463ffffffff16948301949094526501000000000090046fffffffffffffffffffffffffffffffff16938101939093526001015473ffffffffffffffffffffffffffffffffffffffff166060830152610f15576fffffffffffffffffffffffffffffffff6040820152600181526000869003610f15578195505b600086826020015163ffffffff16610f2d91906158cf565b90506000838211610f3e5781610f40565b835b602084015190915063ffffffff165b8181101561108c576000868281548110610f6b57610f6b615871565b6000918252602080832090910154808352600590915260409091205490915060ff16610fc3576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600060018281548110610fd857610fd8615871565b600091825260209091206005909102018054909150640100000000900473ffffffffffffffffffffffffffffffffffffffff161580156110355750600481015460408701516fffffffffffffffffffffffffffffffff9182169116115b1561107757600181015473ffffffffffffffffffffffffffffffffffffffff16606087015260048101546fffffffffffffffffffffffffffffffff1660408701525b50508080611084906158e7565b915050610f4f565b5063ffffffff818116602085810191825260008c81526006909152604090819020865181549351928801517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009094169015157fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000ff161761010092909416918202939093177fffffffffffffffffffffff00000000000000000000000000000000ffffffffff16650100000000006fffffffffffffffffffffffffffffffff909316929092029190911782556060850151600190920180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff9093169290921790915584900361127657606083015160008a815260056020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905561122d73ffffffffffffffffffffffffffffffffffffffff82161561120b5781611227565b600189015473ffffffffffffffffffffffffffffffffffffffff165b896128ab565b875473ffffffffffffffffffffffffffffffffffffffff909116640100000000027fffffffffffffffff0000000000000000000000000000000000000000ffffffff9091161787555b505050505050505050565b600080600054700100000000000000000000000000000000900460ff1660028111156112af576112af615571565b146112e6576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000805260056020527f05b8ccbb9d4d8fb16ea74ce3c29a41f1b461fbdaff4714a0d9a8eb05499746bc5460ff1661134a576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff16600160008154811061137657611376615871565b6000918252602090912060059091020154640100000000900473ffffffffffffffffffffffffffffffffffffffff16146113b15760016113b4565b60025b6000805467ffffffffffffffff421668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff82168117835592935083927fffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffff167fffffffffffffffffffffffffffffff000000000000000000ffffffffffffffff9091161770010000000000000000000000000000000083600281111561146557611465615571565b02179055600281111561147a5761147a615571565b6040517f5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da6090600090a290565b600460205281600052604060002081815481106114c257600080fd5b90600052602060002001600091509150505481565b905090565b6114e9838383600161194c565b505050565b60006114d76114ff60f460146158cf565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c9003013560601c90565b6000818152600660209081526040808320600490925282208054825461156290610100900463ffffffff168261591f565b95945050505050565b60606114d760586020612904565b611581611a04565b60006002600c5460ff16600281111561159c5761159c615571565b036115cd575073ffffffffffffffffffffffffffffffffffffffff81166000908152600a6020526040902054611649565b6001600c5460ff1660028111156115e6576115e6615571565b03611617575073ffffffffffffffffffffffffffffffffffffffff8116600090815260026020526040902054611649565b6040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff82166000908152600b602052604090205460ff1661178a5773ffffffffffffffffffffffffffffffffffffffff82166000908152600b6020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790556116fc60c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe369081013560f01c9003013560601c90565b6040517f7eee288d00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8481166004830152602482018490529190911690637eee288d90604401600060405180830381600087803b15801561176e57600080fd5b505af1158015611782573d6000803e3d6000fd5b505050505050565b806000036117c4576040517f17bfe5f700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff82166000908152600a602090815260408083208390556002909152812055367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c6040517ff3fef3a300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff848116600483015260248201849052919091169063f3fef3a390604401600060405180830381600087803b15801561189a57600080fd5b505af11580156118ae573d6000803e3d6000fd5b5050505060008273ffffffffffffffffffffffffffffffffffffffff168260405160006040518083038185875af1925050503d806000811461190c576040519150601f19603f3d011682016040523d82523d6000602084013e611911565b606091505b50509050806114e9576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c3314806119bc575061198d6114ee565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b6119f2576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6119fe84848484612956565b50505050565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c73ffffffffffffffffffffffffffffffffffffffff16635c975abb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a7f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611aa39190615936565b15611ada576040517f379a7ed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002600c5460ff166002811115611af357611af3615571565b1480611b1557506001600c5460ff166002811115611b1357611b13615571565b145b15611b1c57565b6000600c5460ff166002811115611b3557611b35615571565b14611b6c576040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60005468010000000000000000900467ffffffffffffffff1667ffffffffffffffff16600003611bc8576040517fc105260a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6040517f0314d2b300000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff9190911690630314d2b390602401602060405180830381865afa158015611c67573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c8b9190615936565b905080611cc4576040517f4851bd9b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6040517f17cf21a900000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff91909116906317cf21a990602401600060405180830381600087803b158015611d5e57600080fd5b505af1925050508015611d6f575060015b506000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c6040517f496b9c1600000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff919091169063496b9c1690602401602060405180830381865afa158015611e0f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611e339190615936565b90508015611e6b57600c80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055611e97565b600c80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660021790555b600c546040517f9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f91611ece9160ff909116906155da565b60405180910390a15050565b6114e9838383600061194c565b611eef6133fa565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c3214611f58576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036054013560e01c90565b600080600054700100000000000000000000000000000000900460ff166002811115611fbc57611fbc615571565b14611ff3576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001838154811061200857612008615871565b600091825260208220600590910201805490925063ffffffff9081161461207757815460018054909163ffffffff1690811061204657612046615871565b906000526020600020906005020160040160109054906101000a90046fffffffffffffffffffffffffffffffff1690505b60048201546000906120af90700100000000000000000000000000000000900467ffffffffffffffff165b67ffffffffffffffff1690565b6120c39067ffffffffffffffff164261591f565b6120e26120a2846fffffffffffffffffffffffffffffffff1660401c90565b67ffffffffffffffff166120f691906158cf565b905067ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff16116121435780611562565b7f000000000000000000000000000000000000000000000000000000000000000095945050505050565b60008061220c836fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690507f000000000000000000000000000000000000000000000000000000000000000081111561226b576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b642e90edd00062061a806311e1a30060006122868383615982565b9050670de0b6b3a764000060006122bd827f0000000000000000000000000000000000000000000000000000000000000000615996565b905060006122db6122d6670de0b6b3a764000086615996565b613edc565b905060006122e98484614137565b905060006122f78383614186565b90506000612304826141b4565b905060006123238261231e670de0b6b3a76400008f615996565b61439c565b905060006123318b83614186565b905061233d818d615996565b9f9e505050505050505050505050505050565b6001818154811061236057600080fd5b60009182526020909120600590910201805460018201546002830154600384015460049094015463ffffffff8416955064010000000090930473ffffffffffffffffffffffffffffffffffffffff908116949216926fffffffffffffffffffffffffffffffff91821692918082169170010000000000000000000000000000000090041687565b60006002600c5460ff16600281111561240257612402615571565b03612430575073ffffffffffffffffffffffffffffffffffffffff166000908152600a602052604090205490565b5073ffffffffffffffffffffffffffffffffffffffff1660009081526002602052604090205490565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360f4013560601c3314806124c9575061249a6114ee565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b6124ff576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117828686868686866143d6565b60008054700100000000000000000000000000000000900460ff16600281111561253957612539615571565b14612570576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008060008061257f86614a0c565b9350935093509350600061259585858585614e15565b90506000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015612614573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061263891906159d3565b9050600189036127335773ffffffffffffffffffffffffffffffffffffffff81166352f0f3ad8a84612697367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036034013590565b90565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815260048101939093526024830191909152604482015260206064820152608481018a905260a4015b6020604051808303816000875af1158015612709573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061272d91906159f0565b50611276565b6002890361275f5773ffffffffffffffffffffffffffffffffffffffff81166352f0f3ad8a8489612697565b6003890361278b5773ffffffffffffffffffffffffffffffffffffffff81166352f0f3ad8a8487612697565b60048903612831576040517f52f0f3ad000000000000000000000000000000000000000000000000000000008152600481018a905260248101839052367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036058013560c01b6044820152600860648201526084810188905273ffffffffffffffffffffffffffffffffffffffff8216906352f0f3ad9060a4016126ea565b6040517fff137e6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c9003605481013560e01c906014013560606128a461156b565b9050909192565b60028082015473ffffffffffffffffffffffffffffffffffffffff841660009081526020929092526040822080546fffffffffffffffffffffffffffffffff9092169290916128fb9084906158cf565b90915550505050565b604051818152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90038284820160208401378260208301016000815260208101604052505092915050565b60008054700100000000000000000000000000000000900460ff16600281111561298257612982615571565b146129b9576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000600184815481106129ce576129ce615871565b60009182526020918290206040805160e0810182526005909302909101805463ffffffff8116845273ffffffffffffffffffffffffffffffffffffffff64010000000090910481169484019490945260018101549093169082015260028201546fffffffffffffffffffffffffffffffff908116606083015260038301546080830181905260049093015480821660a084015270010000000000000000000000000000000090041660c082015291508514612ab5576040517f3014033200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60a0810151600083156fffffffffffffffffffffffffffffffff83161760011b90506000612b75826fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050861580612bb05750612bad7f000000000000000000000000000000000000000000000000000000000000000060026158cf565b81145b8015612bba575084155b15612bf1576040517fa42637bc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000811115612c4b576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612c767f000000000000000000000000000000000000000000000000000000000000000060016158cf565b8103612c8857612c8886888588614ecf565b34612c928361216d565b14612cc9576040517f8620aa1900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612cd488611f8e565b905067ffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000811690821603612d3c576040517f3381d11400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612d6960017f000000000000000000000000000000000000000000000000000000000000000061591f565b8303612eb757367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015612dea573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e0e91906159d3565b73ffffffffffffffffffffffffffffffffffffffff1663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e58573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e7c91906159f0565b612eb0907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615a09565b9050612f4a565b612ee260017f000000000000000000000000000000000000000000000000000000000000000061591f565b8303612f1d57612eb07f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615a35565b507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff165b612f7e817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615a65565b67ffffffffffffffff16612f998367ffffffffffffffff1690565b67ffffffffffffffff161115612fe057612fdd817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615a65565b91505b6000604083901b421760008a8152608087901b6fffffffffffffffffffffffffffffffff8d1617602052604081209192509060008181526003602052604090205490915060ff161561305e576040517f80497e3b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60016003600083815260200190815260200160002060006101000a81548160ff02191690831515021790555060016040518060e001604052808d63ffffffff168152602001600073ffffffffffffffffffffffffffffffffffffffff1681526020013373ffffffffffffffffffffffffffffffffffffffff168152602001346fffffffffffffffffffffffffffffffff1681526020018c8152602001886fffffffffffffffffffffffffffffffff168152602001846fffffffffffffffffffffffffffffffff16815250908060018154018082558091505060019003906000526020600020906005020160009091909190915060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060408201518160010160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060608201518160020160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055506080820151816003015560a08201518160040160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060c08201518160040160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055505050600460008c8152602001908152602001600020600180805490506132f3919061591f565b81546001810183556000928352602080842090910191909155338252600a90526040812080543492906133279084906158cf565b9091555050367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c73ffffffffffffffffffffffffffffffffffffffff1663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156133a457600080fd5b505af11580156133b8573d6000803e3d6000fd5b50506040513393508d92508e91507f9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be90600090a4505050505050505050505050565b60005471010000000000000000000000000000000000900460ff161561344c576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613454615089565b361461348c576040517f9824bdab00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600080367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c73ffffffffffffffffffffffffffffffffffffffff1663d83ef2676040518163ffffffff1660e01b81526004016040805180830381865afa158015613509573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061352d9190615a8e565b909250905081613569576040517f6a6bc3b200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7fffd7db0f9d5cdeb49c4c9eba649d4dc6d852d64671e65488e57f58584992ac686135be367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036014013590565b036135f5576040517f2cfac08200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080518082019091528281526020018190526007829055600881905567ffffffffffffffff367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015613696573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906136ba91906159d3565b73ffffffffffffffffffffffffffffffffffffffff1663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613704573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061372891906159f0565b1115613760576040517fb4e1243300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006137977f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615996565b90506000367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c73ffffffffffffffffffffffffffffffffffffffff16637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015613816573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061383a91906159d3565b73ffffffffffffffffffffffffffffffffffffffff1663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613884573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906138a891906159f0565b6138dc907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166158cf565b905060006138ea838361509e565b905067ffffffffffffffff81111561392e576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff1611156139a6576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360d4013515613a0b576040517f223db39400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b83367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036058013511613aa5576040517ff40239db000000000000000000000000000000000000000000000000000000008152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036014013560048201526024015b60405180910390fd5b6040805160e08101825263ffffffff80825260006020808401828152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90038035606090811c8789018181526fffffffffffffffffffffffffffffffff34818116948b0194855260149095013560808b01908152600160a08c0181815242841660c08e0190815282548084018455928c529c517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6600590930292830180549a5191909d167fffffffffffffffff000000000000000000000000000000000000000000000000909a169990991764010000000073ffffffffffffffffffffffffffffffffffffffff9a8b160217909b5592517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf7840180547fffffffffffffffffffffffff000000000000000000000000000000000000000016919098161790965592517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf8820180547fffffffffffffffffffffffffffffffff000000000000000000000000000000001691851691909117905593517fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf9850155955196519681167001000000000000000000000000000000009790911696909602959095177fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cfa909101558154710100000000000000000000000000000000007fffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffff909116178255918152600a909152918220805491929091613d269084906158cf565b9091555050367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360c0013560601c73ffffffffffffffffffffffffffffffffffffffff1663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b158015613da357600080fd5b505af1158015613db7573d6000803e3d6000fd5b5050600080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000164267ffffffffffffffff1617905550613dfa9150611f5a9050565b63ffffffff16367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c900360ac013560601c73ffffffffffffffffffffffffffffffffffffffff16633c9f397c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613e7b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613e9f9190615ab2565b600980547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001663ffffffff92909216929092141790555050505050565b6fffffffffffffffffffffffffffffffff811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b1760008213613f3b57631615e6386000526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f218311670de0b6b3a76400000215820261417457637c5f487d6000526004601cfd5b50670de0b6b3a7640000919091020490565b6000816000190483118202156141a45763bac65e5b6000526004601cfd5b50670de0b6b3a764000091020490565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d782136141e257919050565b680755bf798b4a1bf1e582126142005763a37bfec96000526004601cfd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b60006143cd670de0b6b3a7640000836143b486613edc565b6143be9190615ad8565b6143c89190615b94565b6141b4565b90505b92915050565b60008054700100000000000000000000000000000000900460ff16600281111561440257614402615571565b14614439576040517f67fe195000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001878154811061444e5761444e615871565b6000918252602082206005919091020160048101549092506fffffffffffffffffffffffffffffffff16908715821760011b90506144ad7f000000000000000000000000000000000000000000000000000000000000000060016158cf565b614549826fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1614614583576040517f5f53dd9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000808915614687576145d67f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000061591f565b6001901b6145f5846fffffffffffffffffffffffffffffffff166150b5565b6fffffffffffffffffffffffffffffffff166146119190615bfc565b1561464e5761464561463660016fffffffffffffffffffffffffffffffff8716615c10565b865463ffffffff166000615154565b6003015461467d565b367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c9003607801355b91508490506146b1565b600385015491506146ae6146366fffffffffffffffffffffffffffffffff86166001615c39565b90505b600882901b60088a8a6040516146c8929190615c64565b6040518091039020901b14614709576040517f696550ff00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006147148c615238565b90506000614723836003015490565b6040517fe14ced32000000000000000000000000000000000000000000000000000000008152367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90036098013560601c9063e14ced3290614797908f908f908f908f908a90600401615cbd565b6020604051808303816000875af11580156147b6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906147da91906159f0565b600485015491149150600090600290614885906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b614921896fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b61492b9190615cf7565b6149359190615d1a565b60ff161590508115158103614976576040517ffb4e40dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8754640100000000900473ffffffffffffffffffffffffffffffffffffffff16156149cd576040517f9071e6af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505085547fffffffffffffffff0000000000000000000000000000000000000000ffffffff163364010000000002179095555050505050505050505050565b6000806000806000859050600060018281548110614a2c57614a2c615871565b600091825260209091206004600590920201908101549091507f000000000000000000000000000000000000000000000000000000000000000090614b03906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1611614b3d576040517fb34b5c2200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000815b60048301547f000000000000000000000000000000000000000000000000000000000000000090614c04906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169250821115614c7957825463ffffffff16614c437f000000000000000000000000000000000000000000000000000000000000000060016158cf565b8303614c4d578391505b60018181548110614c6057614c60615871565b9060005260206000209060050201935080945050614b41565b600481810154908401546fffffffffffffffffffffffffffffffff91821691166000816fffffffffffffffffffffffffffffffff16614ce2614ccd856fffffffffffffffffffffffffffffffff1660011c90565b6fffffffffffffffffffffffffffffffff1690565b6fffffffffffffffffffffffffffffffff161490508015614db1576000614d1a836fffffffffffffffffffffffffffffffff166150b5565b6fffffffffffffffffffffffffffffffff161115614d85576000614d5c614d5460016fffffffffffffffffffffffffffffffff8616615c10565b896001615154565b6003810154600490910154909c506fffffffffffffffffffffffffffffffff169a50614d8b9050565b6007549a505b600386015460048701549099506fffffffffffffffffffffffffffffffff169750614e07565b6000614dd3614d546fffffffffffffffffffffffffffffffff85166001615c39565b6003808901546004808b015492840154930154909e506fffffffffffffffffffffffffffffffff9182169d50919b50169850505b505050505050509193509193565b60006fffffffffffffffffffffffffffffffff841615614e825760408051602081018790526fffffffffffffffffffffffffffffffff8087169282019290925260608101859052908316608082015260a00160405160208183030381529060405280519060200120611562565b8282604051602001614eb09291909182526fffffffffffffffffffffffffffffffff16602082015260400190565b6040516020818303038152906040528051906020012095945050505050565b6000614eee6fffffffffffffffffffffffffffffffff84166001615c39565b90506000614efe82866001615154565b9050600086901a8380614fea5750614f3760027f0000000000000000000000000000000000000000000000000000000000000000615bfc565b6004830154600290614fdb906fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b614fe59190615d1a565b60ff16145b156150425760ff811660011480615004575060ff81166002145b61503d576040517ff40239db00000000000000000000000000000000000000000000000000000000815260048101889052602401613a9c565b615080565b60ff811615615080576040517ff40239db00000000000000000000000000000000000000000000000000000000815260048101889052602401613a9c565b50505050505050565b6000615093615267565b6114d79060066158cf565b6000818310156150ae57816143cd565b5090919050565b600080615142837e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b600160ff919091161b90920392915050565b6000808261519d576151986fffffffffffffffffffffffffffffffff86167f0000000000000000000000000000000000000000000000000000000000000000615275565b6151b8565b6151b8856fffffffffffffffffffffffffffffffff16615401565b9050600184815481106151cd576151cd615871565b906000526020600020906005020191505b60048201546fffffffffffffffffffffffffffffffff82811691161461523057815460018054909163ffffffff1690811061521b5761521b615871565b906000526020600020906005020191506151de565b509392505050565b600080600080600061524986614a0c565b935093509350935061525d84848484614e15565b9695505050505050565b60006114d760f460286158cf565b600081615314846fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff161161532a5763b34b5c226000526004601cfd5b61533383615401565b9050816153d2826fffffffffffffffffffffffffffffffff167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116143d0576143cd6153e88360016158cf565b6fffffffffffffffffffffffffffffffff8316906154a6565b60008119600183011681615495827e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169390931c8015179392505050565b600080615533847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f7f07c4acdd0000000000000000000000000000000000000000000000000000000067ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050808303600180821b0385821b179250505092915050565b6000806040838503121561556257600080fd5b50508035926020909101359150565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b600381106155d7577f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b50565b602081016155e7836155a0565b91905290565b73ffffffffffffffffffffffffffffffffffffffff811681146155d757600080fd5b60006020828403121561562157600080fd5b813561562c816155ed565b9392505050565b60008060006060848603121561564857600080fd5b505081359360208301359350604090920135919050565b6000815180845260005b8181101561568557602081850181015186830182015201615669565b81811115615697576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b6020815260006143cd602083018461565f565b6000602082840312156156ef57600080fd5b5035919050565b80151581146155d757600080fd5b6000806000806080858703121561571a57600080fd5b843593506020850135925060408501359150606085013561573a816156f6565b939692955090935050565b60006020828403121561575757600080fd5b81356fffffffffffffffffffffffffffffffff8116811461562c57600080fd5b60008083601f84011261578957600080fd5b50813567ffffffffffffffff8111156157a157600080fd5b6020830191508360208285010111156157b957600080fd5b9250929050565b600080600080600080608087890312156157d957600080fd5b8635955060208701356157eb816156f6565b9450604087013567ffffffffffffffff8082111561580857600080fd5b6158148a838b01615777565b9096509450606089013591508082111561582d57600080fd5b5061583a89828a01615777565b979a9699509497509295939492505050565b63ffffffff84168152826020820152606060408201526000611562606083018461565f565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600082198211156158e2576158e26158a0565b500190565b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203615918576159186158a0565b5060010190565b600082821015615931576159316158a0565b500390565b60006020828403121561594857600080fd5b815161562c816156f6565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b60008261599157615991615953565b500490565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156159ce576159ce6158a0565b500290565b6000602082840312156159e557600080fd5b815161562c816155ed565b600060208284031215615a0257600080fd5b5051919050565b600067ffffffffffffffff808316818516808303821115615a2c57615a2c6158a0565b01949350505050565b600067ffffffffffffffff80831681851681830481118215151615615a5c57615a5c6158a0565b02949350505050565b600067ffffffffffffffff83811690831681811015615a8657615a866158a0565b039392505050565b60008060408385031215615aa157600080fd5b505080516020909101519092909150565b600060208284031215615ac457600080fd5b815163ffffffff8116811461562c57600080fd5b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615615b1957615b196158a0565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615615b5457615b546158a0565b60008712925087820587128484161615615b7057615b706158a0565b87850587128184161615615b8657615b866158a0565b505050929093029392505050565b600082615ba357615ba3615953565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83147f800000000000000000000000000000000000000000000000000000000000000083141615615bf757615bf76158a0565b500590565b600082615c0b57615c0b615953565b500690565b60006fffffffffffffffffffffffffffffffff83811690831681811015615a8657615a866158a0565b60006fffffffffffffffffffffffffffffffff808316818516808303821115615a2c57615a2c6158a0565b8183823760009101908152919050565b8183528181602085013750600060208284010152600060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b606081526000615cd1606083018789615c74565b8281036020840152615ce4818688615c74565b9150508260408301529695505050505050565b600060ff821660ff841680821015615d1157615d116158a0565b90039392505050565b600060ff831680615d2d57615d2d615953565b8060ff8416069150509291505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x03\x1EW`\x005`\xE0\x1C\x80cxk\x84K\x11a\x01\xA5W\x80c\xC0\xD8\xBBt\x11a\0\xECW\x80c\xDA\xBD9m\x11a\0\x95W\x80c\xF8\xF4?\xF6\x11a\0oW\x80c\xF8\xF4?\xF6\x14a\x0B\xCEW\x80c\xFA$\xF7C\x14a\x0B\xEEW\x80c\xFA1Z\xA9\x14a\x0C\x12W\x80c\xFE+\xBE\xB2\x14a\x0CEW`\0\x80\xFD[\x80c\xDA\xBD9m\x14a\x0B8W\x80c\xEC^c\x08\x14a\x0BkW\x80c\xEF\xF0\xF5\x92\x14a\x0B\x9EW`\0\x80\xFD[\x80c\xCF\t\xE0\xD0\x11a\0\xC6W\x80c\xCF\t\xE0\xD0\x14a\n\xD7W\x80c\xD5\xD4M\x80\x14a\n\xF8W\x80c\xD8\xCC\x1A<\x14a\x0B\x18W`\0\x80\xFD[\x80c\xC0\xD8\xBBt\x14a\n\0W\x80c\xC3\x95\xE1\xCA\x14a\n-W\x80c\xC6\xF00\x8C\x14a\nMW`\0\x80\xFD[\x80c\x99s^2\x11a\x01NW\x80c\xBB\xDC\x02\xDB\x11a\x01(W\x80c\xBB\xDC\x02\xDB\x14a\tUW\x80c\xBC\xEF;U\x14a\t\xA0W\x80c\xBD\x8D\xA9V\x14a\t\xE0W`\0\x80\xFD[\x80c\x99s^2\x14a\x08\x06W\x80c\xA4E\xEC\xE6\x14a\x08FW\x80c\xA8\xE4\xFB\x90\x14a\t\x12W`\0\x80\xFD[\x80c\x89\x80\xE0\xCC\x11a\x01\x7FW\x80c\x89\x80\xE0\xCC\x14a\x07\x81W\x80c\x8DE\n\x95\x14a\x07\x96W\x80c\x93\x8Dh\x9A\x14a\x07\xD6W`\0\x80\xFD[\x80cxk\x84K\x14a\x07QW\x80c{\x0F\n\xDC\x14a\x07fW\x80c\x81)\xFC\x1C\x14a\x07yW`\0\x80\xFD[\x80cG'w\xC6\x11a\x02iW\x80c\\\x0C\xBA3\x11a\x02\x12W\x80ccaPm\x11a\x01\xECW\x80ccaPm\x14a\x06\xCBW\x80ckg\x16\xC0\x14a\x07\x0BW\x80co\x03D\t\x14a\x07>W`\0\x80\xFD[\x80c\\\x0C\xBA3\x14a\x06SW\x80c`\x9D34\x14a\x06\x96W\x80c`\xE2td\x14a\x06\xABW`\0\x80\xFD[\x80cT\xFDMP\x11a\x02CW\x80cT\xFDMP\x14a\x05\xCFW\x80cY\xCE\xBE\t\x14a\x06\x1EW\x80cZ_\xA2\xD9\x14a\x063W`\0\x80\xFD[\x80cG'w\xC6\x14a\x05zW\x80cR\x9Dj\x8C\x14a\x05\x8DW\x80cSM\xB0\xE2\x14a\x05\xBAW`\0\x80\xFD[\x80c(\x10\xE1\xD6\x11a\x02\xCBW\x80c7\xB1\xB2)\x11a\x02\xA5W\x80c7\xB1\xB2)\x14a\x04\x93W\x80c:v\x84c\x14a\x04\xF4W\x80c?\xC8\xCE\xF3\x14a\x057W`\0\x80\xFD[\x80c(\x10\xE1\xD6\x14a\x04DW\x80c*\xD6\x9A\xEB\x14a\x04YW\x80c7\x8D\xD4\x8C\x14a\x04yW`\0\x80\xFD[\x80c\"*\xBFE\x11a\x02\xFCW\x80c\"*\xBFE\x14a\x03\xCBW\x80c%\x0Ei\xBD\x14a\x04\x0BW\x80c%\xFC*\xCE\x14a\x04%W`\0\x80\xFD[\x80c\x03\xC2\x92M\x14a\x03#W\x80c\x19\xEF\xFE\xB4\x14a\x03EW\x80c \r.\xD2\x14a\x03\x90W[`\0\x80\xFD[4\x80\x15a\x03/W`\0\x80\xFD[Pa\x03Ca\x03>6`\x04aUOV[a\x0CuV[\0[4\x80\x15a\x03QW`\0\x80\xFD[P`\0Ta\x03r\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x9CW`\0\x80\xFD[P`\0Ta\x03\xBE\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Qa\x03\x87\x91\x90aU\xDAV[4\x80\x15a\x03\xD7W`\0\x80\xFD[Pa\x03\xFBa\x03\xE66`\x04aV\x0FV[`\x0B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\x87V[4\x80\x15a\x04\x17W`\0\x80\xFD[P`\tTa\x03\xFB\x90`\xFF\x16\x81V[4\x80\x15a\x041W`\0\x80\xFD[P`\x07T[`@Q\x90\x81R` \x01a\x03\x87V[4\x80\x15a\x04PW`\0\x80\xFD[Pa\x03\xBEa\x12\x81V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x046a\x04t6`\x04aUOV[a\x14\xA6V[4\x80\x15a\x04\x85W`\0\x80\xFD[P`\x0CTa\x03\xBE\x90`\xFF\x16\x81V[4\x80\x15a\x04\x9FW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x035``\x1C[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\x87V[4\x80\x15a\x05\0W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Ca\x04\xCFV[4\x80\x15a\x05CW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Ca\x04\xCFV[a\x03Ca\x05\x886`\x04aV3V[a\x14\xDCV[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x046a\x05\xA86`\x04aV\x0FV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\xC6W`\0\x80\xFD[Pa\x04\xCFa\x14\xEEV[4\x80\x15a\x05\xDBW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F0.6.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03\x87\x91\x90aV\xCAV[4\x80\x15a\x06*W`\0\x80\xFD[P`\x08Ta\x046V[4\x80\x15a\x06?W`\0\x80\xFD[Pa\x046a\x06N6`\x04aV\xDDV[a\x151V[4\x80\x15a\x06_W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Ca\x04\xCFV[4\x80\x15a\x06\xA2W`\0\x80\xFD[Pa\x06\x11a\x15kV[4\x80\x15a\x06\xB7W`\0\x80\xFD[Pa\x03Ca\x06\xC66`\x04aV\x0FV[a\x15yV[4\x80\x15a\x06\xD7W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`4\x015a\x046V[4\x80\x15a\x07\x17W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03rV[a\x03Ca\x07L6`\x04aW\x04V[a\x19LV[4\x80\x15a\x07]W`\0\x80\xFD[Pa\x03Ca\x1A\x04V[a\x03Ca\x07t6`\x04aV3V[a\x1E\xDAV[a\x03Ca\x1E\xE7V[4\x80\x15a\x07\x8DW`\0\x80\xFD[P`\x01Ta\x046V[4\x80\x15a\x07\xA2W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`x\x015a\x046V[4\x80\x15a\x07\xE2W`\0\x80\xFD[P`\x07T`\x08Ta\x07\xF1\x91\x90\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\x87V[4\x80\x15a\x08\x12W`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`X\x015a\x046V[4\x80\x15a\x08RW`\0\x80\xFD[Pa\x08\xBEa\x08a6`\x04aV\xDDV[`\x06` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x82\x16\x91a\x01\0\x81\x04c\xFF\xFF\xFF\xFF\x16\x91e\x01\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84V[`@\x80Q\x94\x15\x15\x85Rc\xFF\xFF\xFF\xFF\x90\x93\x16` \x85\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91\x83\x01\x91\x90\x91Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x80\x01a\x03\x87V[4\x80\x15a\t\x1EW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1Ca\x04\xCFV[4\x80\x15a\taW`\0\x80\xFD[P`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x81R` \x01a\x03\x87V[4\x80\x15a\t\xACW`\0\x80\xFD[P6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x14\x015a\x046V[4\x80\x15a\t\xECW`\0\x80\xFD[Pa\x03ra\t\xFB6`\x04aV\xDDV[a\x1F\x8EV[4\x80\x15a\n\x0CW`\0\x80\xFD[Pa\x046a\n\x1B6`\x04aV\x0FV[`\n` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\n9W`\0\x80\xFD[Pa\x046a\nH6`\x04aWEV[a!mV[4\x80\x15a\nYW`\0\x80\xFD[Pa\nma\nh6`\x04aV\xDDV[a#PV[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x98\x16\x88Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16` \x89\x01R\x95\x90\x94\x16\x94\x86\x01\x94\x90\x94Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16``\x86\x01R`\x80\x85\x01R\x91\x82\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\x87V[4\x80\x15a\n\xE3W`\0\x80\xFD[P`\0Ta\x03r\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x0B\x04W`\0\x80\xFD[Pa\x046a\x0B\x136`\x04aV\x0FV[a#\xE7V[4\x80\x15a\x0B$W`\0\x80\xFD[Pa\x03Ca\x0B36`\x04aW\xC0V[a$YV[4\x80\x15a\x0BDW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03rV[4\x80\x15a\x0BwW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x046V[4\x80\x15a\x0B\xAAW`\0\x80\xFD[Pa\x03\xFBa\x0B\xB96`\x04aV\xDDV[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\xDAW`\0\x80\xFD[Pa\x03Ca\x0B\xE96`\x04aV3V[a%\rV[4\x80\x15a\x0B\xFAW`\0\x80\xFD[Pa\x0C\x03a(cV[`@Qa\x03\x87\x93\x92\x91\x90aXLV[4\x80\x15a\x0C\x1EW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x046V[4\x80\x15a\x0CQW`\0\x80\xFD[Pa\x03\xFBa\x0C`6`\x04aV\xDDV[`\x05` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0C\xA1Wa\x0C\xA1aUqV[\x14a\x0C\xD8W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x83\x81T\x81\x10a\x0C\xEDWa\x0C\xEDaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x90P`\0a\r\x08\x84a\x1F\x8EV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x10\x15a\rqW`@Q\x7F\xF2D\x0BS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x05` R`@\x90 T`\xFF\x16\x15a\r\xBAW`@Q\x7F\xF1\xA9E\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x04` R`@\x90 \x80T\x80\x15\x80\x15a\r\xD7WP\x85\x15\x15[\x15a\x0ErW\x83Td\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15a\x0E\nW\x81a\x0E&V[`\x01\x86\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x90Pa\x0E2\x81\x87a(\xABV[PPP`\0\x94\x85RPP`\x05` RPP`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UV[`\0\x86\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x82\x04c\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x81\x01\x93\x90\x93R`\x01\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x83\x01Ra\x0F\x15Wo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01R`\x01\x81R`\0\x86\x90\x03a\x0F\x15W\x81\x95P[`\0\x86\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a\x0F-\x91\x90aX\xCFV[\x90P`\0\x83\x82\x11a\x0F>W\x81a\x0F@V[\x83[` \x84\x01Q\x90\x91Pc\xFF\xFF\xFF\xFF\x16[\x81\x81\x10\x15a\x10\x8CW`\0\x86\x82\x81T\x81\x10a\x0FkWa\x0FkaXqV[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83R`\x05\x90\x91R`@\x90\x91 T\x90\x91P`\xFF\x16a\x0F\xC3W`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x82\x81T\x81\x10a\x0F\xD8Wa\x0F\xD8aXqV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T\x90\x91Pd\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x105WP`\x04\x81\x01T`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x11[\x15a\x10wW`\x01\x81\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01R`\x04\x81\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x87\x01R[PP\x80\x80a\x10\x84\x90aX\xE7V[\x91PPa\x0FOV[Pc\xFF\xFF\xFF\xFF\x81\x81\x16` \x85\x81\x01\x91\x82R`\0\x8C\x81R`\x06\x90\x91R`@\x90\x81\x90 \x86Q\x81T\x93Q\x92\x88\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x94\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\x16\x17a\x01\0\x92\x90\x94\x16\x91\x82\x02\x93\x90\x93\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x82U``\x85\x01Q`\x01\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x84\x90\x03a\x12vW``\x83\x01Q`\0\x8A\x81R`\x05` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x12-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a\x12\x0BW\x81a\x12'V[`\x01\x89\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[\x89a(\xABV[\x87Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x87U[PPPPPPPPPV[`\0\x80`\0Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x12\xAFWa\x12\xAFaUqV[\x14a\x12\xE6W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80R`\x05` R\x7F\x05\xB8\xCC\xBB\x9DM\x8F\xB1n\xA7L\xE3\xC2\x9AA\xF1\xB4a\xFB\xDA\xFFG\x14\xA0\xD9\xA8\xEB\x05I\x97F\xBCT`\xFF\x16a\x13JW`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\0\x81T\x81\x10a\x13vWa\x13vaXqV[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01Td\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\xB1W`\x01a\x13\xB4V[`\x02[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x17\x83U\x92\x93P\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\x02\x81\x11\x15a\x14eWa\x14eaUqV[\x02\x17\x90U`\x02\x81\x11\x15a\x14zWa\x14zaUqV[`@Q\x7F^\x18o\t\xB9\xC94\x91\xF1N'~\xEA\x7F\xAA]\xE6\xA2\xD4\xBD\xA7Zy\xAFz6\x84\xFB\xFBB\xDA`\x90`\0\x90\xA2\x90V[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x14\xC2W`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[\x90P\x90V[a\x14\xE9\x83\x83\x83`\x01a\x19LV[PPPV[`\0a\x14\xD7a\x14\xFF`\xF4`\x14aX\xCFV[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`\0\x81\x81R`\x06` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x80T\x82Ta\x15b\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aY\x1FV[\x95\x94PPPPPV[``a\x14\xD7`X` a)\x04V[a\x15\x81a\x1A\x04V[`\0`\x02`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x15\x9CWa\x15\x9CaUqV[\x03a\x15\xCDWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\n` R`@\x90 Ta\x16IV[`\x01`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x15\xE6Wa\x15\xE6aUqV[\x03a\x16\x17WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x16IV[`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16a\x17\x8AWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x16\xFC`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE6\x90\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`@Q\x7F~\xEE(\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c~\xEE(\x8D\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x82W=`\0\x80>=`\0\xFD[PPPPPPV[\x80`\0\x03a\x17\xC4W`@Q\x7F\x17\xBF\xE5\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x83\x90U`\x02\x90\x91R\x81 U6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x9AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xAEW=`\0\x80>=`\0\xFD[PPPP`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19\x0CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19\x11V[``\x91P[PP\x90P\x80a\x14\xE9W`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a\x19\xBCWPa\x19\x8Da\x14\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x19\xF2W`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\xFE\x84\x84\x84\x84a)VV[PPPPV[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA3\x91\x90aY6V[\x15a\x1A\xDAW`@Q\x7F7\x9A~\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x1A\xF3Wa\x1A\xF3aUqV[\x14\x80a\x1B\x15WP`\x01`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x1B\x13Wa\x1B\x13aUqV[\x14[\x15a\x1B\x1CWV[`\0`\x0CT`\xFF\x16`\x02\x81\x11\x15a\x1B5Wa\x1B5aUqV[\x14a\x1BlW`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B\xC8W`@Q\x7F\xC1\x05&\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x03\x14\xD2\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90c\x03\x14\xD2\xB3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x8B\x91\x90aY6V[\x90P\x80a\x1C\xC4W`@Q\x7FHQ\xBD\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x17\xCF!\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90c\x17\xCF!\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D^W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x1DoWP`\x01[P`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E3\x91\x90aY6V[\x90P\x80\x15a\x1EkW`\x0C\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\x1E\x97V[`\x0C\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x02\x17\x90U[`\x0CT`@Q\x7F\x99\x08\xEA\xAC\x06E\xDF\x9D\x07\x04\xD0j\xDC\x9E\x073|\x95\x1D\xE2\xF0k_(6\x15\x1DH\xD5\xE4r/\x91a\x1E\xCE\x91`\xFF\x90\x91\x16\x90aU\xDAV[`@Q\x80\x91\x03\x90\xA1PPV[a\x14\xE9\x83\x83\x83`\0a\x19LV[a\x1E\xEFa3\xFAV[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C2\x14a\x1FXW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x90V[`\0\x80`\0Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1F\xBCWa\x1F\xBCaUqV[\x14a\x1F\xF3W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x83\x81T\x81\x10a \x08Wa \x08aXqV[`\0\x91\x82R` \x82 `\x05\x90\x91\x02\x01\x80T\x90\x92Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x14a wW\x81T`\x01\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10a FWa FaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01`\x04\x01`\x10\x90T\x90a\x01\0\n\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\x04\x82\x01T`\0\x90a \xAF\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a \xC3\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaY\x1FV[a \xE2a \xA2\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a \xF6\x91\x90aX\xCFV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a!CW\x80a\x15bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`\0\x80a\"\x0C\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a\"kW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d.\x90\xED\xD0\0b\x06\x1A\x80c\x11\xE1\xA3\0`\0a\"\x86\x83\x83aY\x82V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0a\"\xBD\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x96V[\x90P`\0a\"\xDBa\"\xD6g\r\xE0\xB6\xB3\xA7d\0\0\x86aY\x96V[a>\xDCV[\x90P`\0a\"\xE9\x84\x84aA7V[\x90P`\0a\"\xF7\x83\x83aA\x86V[\x90P`\0a#\x04\x82aA\xB4V[\x90P`\0a##\x82a#\x1Eg\r\xE0\xB6\xB3\xA7d\0\0\x8FaY\x96V[aC\x9CV[\x90P`\0a#1\x8B\x83aA\x86V[\x90Pa#=\x81\x8DaY\x96V[\x9F\x9EPPPPPPPPPPPPPPPV[`\x01\x81\x81T\x81\x10a#`W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01Tc\xFF\xFF\xFF\xFF\x84\x16\x95Pd\x01\0\0\0\0\x90\x93\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x94\x92\x16\x92o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92\x91\x80\x82\x16\x91p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x87V[`\0`\x02`\x0CT`\xFF\x16`\x02\x81\x11\x15a$\x02Wa$\x02aUqV[\x03a$0WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\n` R`@\x90 T\x90V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a$\xC9WPa$\x9Aa\x14\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a$\xFFW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\x82\x86\x86\x86\x86\x86\x86aC\xD6V[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a%9Wa%9aUqV[\x14a%pW`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80a%\x7F\x86aJ\x0CV[\x93P\x93P\x93P\x93P`\0a%\x95\x85\x85\x85\x85aN\x15V[\x90P`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&8\x91\x90aY\xD3V[\x90P`\x01\x89\x03a'3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16cR\xF0\xF3\xAD\x8A\x84a&\x976\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`4\x015\x90V[\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R`D\x82\x01R` `d\x82\x01R`\x84\x81\x01\x8A\x90R`\xA4\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a'\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'-\x91\x90aY\xF0V[Pa\x12vV[`\x02\x89\x03a'_Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16cR\xF0\xF3\xAD\x8A\x84\x89a&\x97V[`\x03\x89\x03a'\x8BWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16cR\xF0\xF3\xAD\x8A\x84\x87a&\x97V[`\x04\x89\x03a(1W`@Q\x7FR\xF0\xF3\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`X\x015`\xC0\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90cR\xF0\xF3\xAD\x90`\xA4\x01a&\xEAV[`@Q\x7F\xFF\x13~e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`T\x81\x015`\xE0\x1C\x90`\x14\x015``a(\xA4a\x15kV[\x90P\x90\x91\x92V[`\x02\x80\x82\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R` \x92\x90\x92R`@\x82 \x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92\x90\x91a(\xFB\x90\x84\x90aX\xCFV[\x90\x91UPPPPV[`@Q\x81\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x82\x84\x82\x01` \x84\x017\x82` \x83\x01\x01`\0\x81R` \x81\x01`@RPP\x92\x91PPV[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a)\x82Wa)\x82aUqV[\x14a)\xB9W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x84\x81T\x81\x10a)\xCEWa)\xCEaXqV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x81\x16\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFd\x01\0\0\0\0\x90\x91\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01\x81\x01T\x90\x93\x16\x90\x82\x01R`\x02\x82\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x01R`\x03\x83\x01T`\x80\x83\x01\x81\x90R`\x04\x90\x93\x01T\x80\x82\x16`\xA0\x84\x01Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16`\xC0\x82\x01R\x91P\x85\x14a*\xB5W`@Q\x7F0\x14\x032\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0\x81\x01Q`\0\x83\x15o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17`\x01\x1B\x90P`\0a+u\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x86\x15\x80a+\xB0WPa+\xAD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02aX\xCFV[\x81\x14[\x80\x15a+\xBAWP\x84\x15[\x15a+\xF1W`@Q\x7F\xA4&7\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a,KW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aX\xCFV[\x81\x03a,\x88Wa,\x88\x86\x88\x85\x88aN\xCFV[4a,\x92\x83a!mV[\x14a,\xC9W`@Q\x7F\x86 \xAA\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a,\xD4\x88a\x1F\x8EV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a-<W`@Q\x7F3\x81\xD1\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a-i`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x1FV[\x83\x03a.\xB7W6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x0E\x91\x90aY\xD3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.|\x91\x90aY\xF0V[a.\xB0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZ\tV[\x90Pa/JV[a.\xE2`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x1FV[\x83\x03a/\x1DWa.\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aZ5V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[a/~\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZeV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a/\x99\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a/\xE0Wa/\xDD\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZeV[\x91P[`\0`@\x83\x90\x1BB\x17`\0\x8A\x81R`\x80\x87\x90\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x16\x17` R`@\x81 \x91\x92P\x90`\0\x81\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16\x15a0^W`@Q\x7F\x80I~;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`@Q\x80`\xE0\x01`@R\x80\x8Dc\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x013s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x014o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8C\x81R` \x01\x88o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP`\x04`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\x01\x80\x80T\x90Pa2\xF3\x91\x90aY\x1FV[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x91\x01\x91\x90\x91U3\x82R`\n\x90R`@\x81 \x80T4\x92\x90a3'\x90\x84\x90aX\xCFV[\x90\x91UPP6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a3\xA4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a3\xB8W=`\0\x80>=`\0\xFD[PP`@Q3\x93P\x8D\x92P\x8E\x91P\x7F\x9B2Et\x0E\xC3\xB1U\t\x8AU\xBE\x84\x95zM\xA1>\xAF\x7F\x14\xA8\xBCoS\x12l\x0B\x93P\xF2\xBE\x90`\0\x90\xA4PPPPPPPPPPPPV[`\0Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a4LW`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a4TaP\x89V[6\x14a4\x8CW`@Q\x7F\x98$\xBD\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x806\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8>\xF2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5-\x91\x90aZ\x8EV[\x90\x92P\x90P\x81a5iW`@Q\x7Fjk\xC3\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xFF\xD7\xDB\x0F\x9D\\\xDE\xB4\x9CL\x9E\xBAd\x9DM\xC6\xD8R\xD6Fq\xE6T\x88\xE5\x7FXXI\x92\xACha5\xBE6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x14\x015\x90V[\x03a5\xF5W`@Q\x7F,\xFA\xC0\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x01\x81\x90R`\x07\x82\x90U`\x08\x81\x90Ug\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xBA\x91\x90aY\xD3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7(\x91\x90aY\xF0V[\x11\x15a7`W`@Q\x7F\xB4\xE1$3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a7\x97\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aY\x96V[\x90P`\x006\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8:\x91\x90aY\xD3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xA8\x91\x90aY\xF0V[a8\xDC\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aX\xCFV[\x90P`\0a8\xEA\x83\x83aP\x9EV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9.W`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a9\xA6W`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xD4\x015\x15a:\x0BW`@Q\x7F\"=\xB3\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x836\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`X\x015\x11a:\xA5W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x14\x015`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x80\x82R`\0` \x80\x84\x01\x82\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x805``\x90\x81\x1C\x87\x89\x01\x81\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF4\x81\x81\x16\x94\x8B\x01\x94\x85R`\x14\x90\x95\x015`\x80\x8B\x01\x90\x81R`\x01`\xA0\x8C\x01\x81\x81RB\x84\x16`\xC0\x8E\x01\x90\x81R\x82T\x80\x84\x01\x84U\x92\x8CR\x9CQ\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6`\x05\x90\x93\x02\x92\x83\x01\x80T\x9AQ\x91\x90\x9D\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x9A\x16\x99\x90\x99\x17d\x01\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9A\x8B\x16\x02\x17\x90\x9BU\x92Q\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF7\x84\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x98\x16\x17\x90\x96U\x92Q\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF8\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF9\x85\x01U\x95Q\x96Q\x96\x81\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x90\x91\x16\x96\x90\x96\x02\x95\x90\x95\x17\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xFA\x90\x91\x01U\x81Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U\x91\x81R`\n\x90\x91R\x91\x82 \x80T\x91\x92\x90\x91a=&\x90\x84\x90aX\xCFV[\x90\x91UPP6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a=\xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a=\xB7W=`\0\x80>=`\0\xFD[PP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPa=\xFA\x91Pa\x1FZ\x90PV[c\xFF\xFF\xFF\xFF\x166\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c<\x9F9|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\x9F\x91\x90aZ\xB2V[`\t\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x92\x90\x92\x14\x17\x90UPPPPPV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17`\0\x82\x13a?;Wc\x16\x15\xE68`\0R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[`\0x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x83\x11g\r\xE0\xB6\xB3\xA7d\0\0\x02\x15\x82\x02aAtWc|_H}`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x02\x15aA\xA4Wc\xBA\xC6^[`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13aA\xE2W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12aB\0Wc\xA3{\xFE\xC9`\0R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0aC\xCDg\r\xE0\xB6\xB3\xA7d\0\0\x83aC\xB4\x86a>\xDCV[aC\xBE\x91\x90aZ\xD8V[aC\xC8\x91\x90a[\x94V[aA\xB4V[\x90P[\x92\x91PPV[`\0\x80Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`\x02\x81\x11\x15aD\x02WaD\x02aUqV[\x14aD9W`@Q\x7Fg\xFE\x19P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x87\x81T\x81\x10aDNWaDNaXqV[`\0\x91\x82R` \x82 `\x05\x91\x90\x91\x02\x01`\x04\x81\x01T\x90\x92Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x87\x15\x82\x17`\x01\x1B\x90PaD\xAD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aX\xCFV[aEI\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x14aE\x83W`@Q\x7F_S\xDD\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x89\x15aF\x87WaE\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aY\x1FV[`\x01\x90\x1BaE\xF5\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aP\xB5V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aF\x11\x91\x90a[\xFCV[\x15aFNWaFEaF6`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16a\\\x10V[\x86Tc\xFF\xFF\xFF\xFF\x16`\0aQTV[`\x03\x01TaF}V[6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`x\x015[\x91P\x84\x90PaF\xB1V[`\x03\x85\x01T\x91PaF\xAEaF6o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x01a\\9V[\x90P[`\x08\x82\x90\x1B`\x08\x8A\x8A`@QaF\xC8\x92\x91\x90a\\dV[`@Q\x80\x91\x03\x90 \x90\x1B\x14aG\tW`@Q\x7FieP\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0aG\x14\x8CaR8V[\x90P`\0aG#\x83`\x03\x01T\x90V[`@Q\x7F\xE1L\xED2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C\x90c\xE1L\xED2\x90aG\x97\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8A\x90`\x04\x01a\\\xBDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aG\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xDA\x91\x90aY\xF0V[`\x04\x85\x01T\x91\x14\x91P`\0\x90`\x02\x90aH\x85\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aI!\x89o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aI+\x91\x90a\\\xF7V[aI5\x91\x90a]\x1AV[`\xFF\x16\x15\x90P\x81\x15\x15\x81\x03aIvW`@Q\x7F\xFBN@\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87Td\x01\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15aI\xCDW`@Q\x7F\x90q\xE6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x163d\x01\0\0\0\0\x02\x17\x90\x95UPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x85\x90P`\0`\x01\x82\x81T\x81\x10aJ,WaJ,aXqV[`\0\x91\x82R` \x90\x91 `\x04`\x05\x90\x92\x02\x01\x90\x81\x01T\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aK\x03\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aK=W`@Q\x7F\xB3K\\\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[`\x04\x83\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aL\x04\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x92P\x82\x11\x15aLyW\x82Tc\xFF\xFF\xFF\xFF\x16aLC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aX\xCFV[\x83\x03aLMW\x83\x91P[`\x01\x81\x81T\x81\x10aL`WaL`aXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x93P\x80\x94PPaKAV[`\x04\x81\x81\x01T\x90\x84\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16`\0\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aL\xE2aL\xCD\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x1C\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x90P\x80\x15aM\xB1W`\0aM\x1A\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aP\xB5V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aM\x85W`\0aM\\aMT`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16a\\\x10V[\x89`\x01aQTV[`\x03\x81\x01T`\x04\x90\x91\x01T\x90\x9CPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9APaM\x8B\x90PV[`\x07T\x9AP[`\x03\x86\x01T`\x04\x87\x01T\x90\x99Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x97PaN\x07V[`\0aM\xD3aMTo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x01a\\9V[`\x03\x80\x89\x01T`\x04\x80\x8B\x01T\x92\x84\x01T\x93\x01T\x90\x9EPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x9DP\x91\x9BP\x16\x98PP[PPPPPPP\x91\x93P\x91\x93V[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15aN\x82W`@\x80Q` \x81\x01\x87\x90Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R\x90\x83\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x15bV[\x82\x82`@Q` \x01aN\xB0\x92\x91\x90\x91\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95\x94PPPPPV[`\0aN\xEEo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x01a\\9V[\x90P`\0aN\xFE\x82\x86`\x01aQTV[\x90P`\0\x86\x90\x1A\x83\x80aO\xEAWPaO7`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a[\xFCV[`\x04\x83\x01T`\x02\x90aO\xDB\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aO\xE5\x91\x90a]\x1AV[`\xFF\x16\x14[\x15aPBW`\xFF\x81\x16`\x01\x14\x80aP\x04WP`\xFF\x81\x16`\x02\x14[aP=W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a:\x9CV[aP\x80V[`\xFF\x81\x16\x15aP\x80W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a:\x9CV[PPPPPPPV[`\0aP\x93aRgV[a\x14\xD7\x90`\x06aX\xCFV[`\0\x81\x83\x10\x15aP\xAEW\x81aC\xCDV[P\x90\x91\x90PV[`\0\x80aQB\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01`\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x80\x82aQ\x9DWaQ\x98o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aRuV[aQ\xB8V[aQ\xB8\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aT\x01V[\x90P`\x01\x84\x81T\x81\x10aQ\xCDWaQ\xCDaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91P[`\x04\x82\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x91\x16\x14aR0W\x81T`\x01\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10aR\x1BWaR\x1BaXqV[\x90`\0R` `\0 \x90`\x05\x02\x01\x91PaQ\xDEV[P\x93\x92PPPV[`\0\x80`\0\x80`\0aRI\x86aJ\x0CV[\x93P\x93P\x93P\x93PaR]\x84\x84\x84\x84aN\x15V[\x96\x95PPPPPPV[`\0a\x14\xD7`\xF4`(aX\xCFV[`\0\x81aS\x14\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aS*Wc\xB3K\\\"`\0R`\x04`\x1C\xFD[aS3\x83aT\x01V[\x90P\x81aS\xD2\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aC\xD0WaC\xCDaS\xE8\x83`\x01aX\xCFV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90aT\xA6V[`\0\x81\x19`\x01\x83\x01\x16\x81aT\x95\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`\0\x80aU3\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x80\x82\x1B\x03\x85\x82\x1B\x17\x92PPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aUbW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aU\xD7W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01aU\xE7\x83aU\xA0V[\x91\x90R\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aU\xD7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aV!W`\0\x80\xFD[\x815aV,\x81aU\xEDV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aVHW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aV\x85W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aViV[\x81\x81\x11\x15aV\x97W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0aC\xCD` \x83\x01\x84aV_V[`\0` \x82\x84\x03\x12\x15aV\xEFW`\0\x80\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14aU\xD7W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aW\x1AW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aW:\x81aV\xF6V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aWWW`\0\x80\xFD[\x815o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aV,W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aW\x89W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xA1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aW\xB9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aW\xD9W`\0\x80\xFD[\x865\x95P` \x87\x015aW\xEB\x81aV\xF6V[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aX\x08W`\0\x80\xFD[aX\x14\x8A\x83\x8B\x01aWwV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aX-W`\0\x80\xFD[PaX:\x89\x82\x8A\x01aWwV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x15b``\x83\x01\x84aV_V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aX\xE2WaX\xE2aX\xA0V[P\x01\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aY\x18WaY\x18aX\xA0V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aY1WaY1aX\xA0V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aYHW`\0\x80\xFD[\x81QaV,\x81aV\xF6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aY\x91WaY\x91aYSV[P\x04\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aY\xCEWaY\xCEaX\xA0V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aY\xE5W`\0\x80\xFD[\x81QaV,\x81aU\xEDV[`\0` \x82\x84\x03\x12\x15aZ\x02W`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aZ,WaZ,aX\xA0V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aZ\\WaZ\\aX\xA0V[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aZ\x86WaZ\x86aX\xA0V[\x03\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aZ\xA1W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15aZ\xC4W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14aV,W`\0\x80\xFD[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a[\x19Wa[\x19aX\xA0V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a[TWa[TaX\xA0V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a[pWa[paX\xA0V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a[\x86Wa[\x86aX\xA0V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82a[\xA3Wa[\xA3aYSV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a[\xF7Wa[\xF7aX\xA0V[P\x05\x90V[`\0\x82a\\\x0BWa\\\x0BaYSV[P\x06\x90V[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aZ\x86WaZ\x86aX\xA0V[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aZ,WaZ,aX\xA0V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0a\\\xD1``\x83\x01\x87\x89a\\tV[\x82\x81\x03` \x84\x01Ra\\\xE4\x81\x86\x88a\\tV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a]\x11Wa]\x11aX\xA0V[\x90\x03\x93\x92PPPV[`\0`\xFF\x83\x16\x80a]-Wa]-aYSV[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
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
    /**Custom error with signature `BadAuth()` and selector `0xd386ef3e`.
```solidity
error BadAuth();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BadAuth;
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
        impl ::core::convert::From<BadAuth> for UnderlyingRustTuple<'_> {
            fn from(value: BadAuth) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BadAuth {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BadAuth {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BadAuth()";
            const SELECTOR: [u8; 4] = [211u8, 134u8, 239u8, 62u8];
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
    /**Custom error with signature `NoChainIdNeeded()` and selector `0x223db394`.
```solidity
error NoChainIdNeeded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoChainIdNeeded;
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
        impl ::core::convert::From<NoChainIdNeeded> for UnderlyingRustTuple<'_> {
            fn from(value: NoChainIdNeeded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoChainIdNeeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoChainIdNeeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoChainIdNeeded()";
            const SELECTOR: [u8; 4] = [34u8, 61u8, 179u8, 148u8];
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
    /**Custom error with signature `SuperFaultDisputeGameInvalidRootClaim()` and selector `0x2cfac082`.
```solidity
error SuperFaultDisputeGameInvalidRootClaim();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SuperFaultDisputeGameInvalidRootClaim;
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
        impl ::core::convert::From<SuperFaultDisputeGameInvalidRootClaim>
        for UnderlyingRustTuple<'_> {
            fn from(value: SuperFaultDisputeGameInvalidRootClaim) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SuperFaultDisputeGameInvalidRootClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SuperFaultDisputeGameInvalidRootClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SuperFaultDisputeGameInvalidRootClaim()";
            const SELECTOR: [u8; 4] = [44u8, 250u8, 192u8, 130u8];
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
constructor(SuperFaultDisputeGame.GameConstructorParams _params);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _params: <SuperFaultDisputeGame::GameConstructorParams as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                SuperFaultDisputeGame::GameConstructorParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <SuperFaultDisputeGame::GameConstructorParams as alloy::sol_types::SolType>::RustType,
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
            type Parameters<'a> = (SuperFaultDisputeGame::GameConstructorParams,);
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
                    <SuperFaultDisputeGame::GameConstructorParams as alloy_sol_types::SolType>::tokenize(
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
function absolutePrestate() external pure returns (Claim absolutePrestate_);
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
function anchorStateRegistry() external pure returns (address registry_);
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
    /**Function with signature `challenger()` and selector `0x534db0e2`.
```solidity
function challenger() external pure returns (address challenger_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`challenger()`](challengerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengerReturn {
        #[allow(missing_docs)]
        pub challenger_: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<challengerCall> for UnderlyingRustTuple<'_> {
                fn from(value: challengerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengerCall {
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
            impl ::core::convert::From<challengerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: challengerReturn) -> Self {
                    (value.challenger_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { challenger_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challenger()";
            const SELECTOR: [u8; 4] = [83u8, 77u8, 176u8, 226u8];
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
                        let r: challengerReturn = r.into();
                        r.challenger_
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
                        let r: challengerReturn = r.into();
                        r.challenger_
                    })
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
function gameData() external pure returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_);
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
function gameType() external pure returns (GameType gameType_);
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
    /**Function with signature `proposer()` and selector `0xa8e4fb90`.
```solidity
function proposer() external pure returns (address proposer_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proposer()`](proposerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposerReturn {
        #[allow(missing_docs)]
        pub proposer_: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<proposerCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposerCall {
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
            impl ::core::convert::From<proposerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposerReturn) -> Self {
                    (value.proposer_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposer_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposer()";
            const SELECTOR: [u8; 4] = [168u8, 228u8, 251u8, 144u8];
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
                        let r: proposerReturn = r.into();
                        r.proposer_
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
                        let r: proposerReturn = r.into();
                        r.proposer_
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
    /**Function with signature `startingProposal()` and selector `0x938d689a`.
```solidity
function startingProposal() external view returns (Hash root, uint256 l2SequenceNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingProposalCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`startingProposal()`](startingProposalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingProposalReturn {
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
            impl ::core::convert::From<startingProposalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingProposalCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingProposalCall {
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
            impl ::core::convert::From<startingProposalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingProposalReturn) -> Self {
                    (value.root, value.l2SequenceNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingProposalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        root: tuple.0,
                        l2SequenceNumber: tuple.1,
                    }
                }
            }
        }
        impl startingProposalReturn {
            fn _tokenize(
                &self,
            ) -> <startingProposalCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <Hash as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2SequenceNumber),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingProposalCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startingProposalReturn;
            type ReturnTuple<'a> = (Hash, alloy::sol_types::sol_data::Uint<256>);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingProposal()";
            const SELECTOR: [u8; 4] = [147u8, 141u8, 104u8, 154u8];
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
                startingProposalReturn::_tokenize(ret)
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
    /**Function with signature `startingSequenceNumber()` and selector `0x59cebe09`.
```solidity
function startingSequenceNumber() external view returns (uint256 startingSequenceNumber_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingSequenceNumberCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`startingSequenceNumber()`](startingSequenceNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startingSequenceNumberReturn {
        #[allow(missing_docs)]
        pub startingSequenceNumber_: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<startingSequenceNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingSequenceNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingSequenceNumberCall {
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
            impl ::core::convert::From<startingSequenceNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startingSequenceNumberReturn) -> Self {
                    (value.startingSequenceNumber_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startingSequenceNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startingSequenceNumber_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startingSequenceNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startingSequenceNumber()";
            const SELECTOR: [u8; 4] = [89u8, 206u8, 190u8, 9u8];
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
                        let r: startingSequenceNumberReturn = r.into();
                        r.startingSequenceNumber_
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
                        let r: startingSequenceNumberReturn = r.into();
                        r.startingSequenceNumber_
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
function vm() external pure returns (address vm_);
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
function weth() external pure returns (address weth_);
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
    ///Container for all the [`SuperPermissionedDisputeGame`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SuperPermissionedDisputeGameCalls {
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
        challenger(challengerCall),
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
        proposer(proposerCall),
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
        startingProposal(startingProposalCall),
        #[allow(missing_docs)]
        startingRootHash(startingRootHashCall),
        #[allow(missing_docs)]
        startingSequenceNumber(startingSequenceNumberCall),
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
    impl SuperPermissionedDisputeGameCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 194u8, 146u8, 77u8],
            [25u8, 239u8, 254u8, 180u8],
            [32u8, 13u8, 46u8, 210u8],
            [34u8, 42u8, 191u8, 69u8],
            [37u8, 14u8, 105u8, 189u8],
            [37u8, 252u8, 42u8, 206u8],
            [40u8, 16u8, 225u8, 214u8],
            [42u8, 214u8, 154u8, 235u8],
            [55u8, 141u8, 212u8, 140u8],
            [55u8, 177u8, 178u8, 41u8],
            [58u8, 118u8, 132u8, 99u8],
            [63u8, 200u8, 206u8, 243u8],
            [71u8, 39u8, 119u8, 198u8],
            [82u8, 157u8, 106u8, 140u8],
            [83u8, 77u8, 176u8, 226u8],
            [84u8, 253u8, 77u8, 80u8],
            [89u8, 206u8, 190u8, 9u8],
            [90u8, 95u8, 162u8, 217u8],
            [92u8, 12u8, 186u8, 51u8],
            [96u8, 157u8, 51u8, 52u8],
            [96u8, 226u8, 116u8, 100u8],
            [99u8, 97u8, 80u8, 109u8],
            [107u8, 103u8, 22u8, 192u8],
            [111u8, 3u8, 68u8, 9u8],
            [120u8, 107u8, 132u8, 75u8],
            [123u8, 15u8, 10u8, 220u8],
            [129u8, 41u8, 252u8, 28u8],
            [137u8, 128u8, 224u8, 204u8],
            [141u8, 69u8, 10u8, 149u8],
            [147u8, 141u8, 104u8, 154u8],
            [153u8, 115u8, 94u8, 50u8],
            [164u8, 69u8, 236u8, 230u8],
            [168u8, 228u8, 251u8, 144u8],
            [187u8, 220u8, 2u8, 219u8],
            [188u8, 239u8, 59u8, 85u8],
            [189u8, 141u8, 169u8, 86u8],
            [192u8, 216u8, 187u8, 116u8],
            [195u8, 149u8, 225u8, 202u8],
            [198u8, 240u8, 48u8, 140u8],
            [207u8, 9u8, 224u8, 208u8],
            [213u8, 212u8, 77u8, 128u8],
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
            ::core::stringify!(resolveClaim),
            ::core::stringify!(resolvedAt),
            ::core::stringify!(status),
            ::core::stringify!(hasUnlockedCredit),
            ::core::stringify!(wasRespectedGameTypeWhenCreated),
            ::core::stringify!(startingRootHash),
            ::core::stringify!(resolve),
            ::core::stringify!(subgames),
            ::core::stringify!(bondDistributionMode),
            ::core::stringify!(gameCreator),
            ::core::stringify!(vm),
            ::core::stringify!(weth),
            ::core::stringify!(attack),
            ::core::stringify!(normalModeCredit),
            ::core::stringify!(challenger),
            ::core::stringify!(version),
            ::core::stringify!(startingSequenceNumber),
            ::core::stringify!(getNumToResolve),
            ::core::stringify!(anchorStateRegistry),
            ::core::stringify!(extraData),
            ::core::stringify!(claimCredit),
            ::core::stringify!(l1Head),
            ::core::stringify!(clockExtension),
            ::core::stringify!(r#move),
            ::core::stringify!(closeGame),
            ::core::stringify!(defend),
            ::core::stringify!(initialize),
            ::core::stringify!(claimDataLen),
            ::core::stringify!(absolutePrestate),
            ::core::stringify!(startingProposal),
            ::core::stringify!(l2SequenceNumber),
            ::core::stringify!(resolutionCheckpoints),
            ::core::stringify!(proposer),
            ::core::stringify!(gameType),
            ::core::stringify!(rootClaim),
            ::core::stringify!(getChallengerDuration),
            ::core::stringify!(refundModeCredit),
            ::core::stringify!(getRequiredBond),
            ::core::stringify!(claimData),
            ::core::stringify!(createdAt),
            ::core::stringify!(credit),
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
            <resolveClaimCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolvedAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <statusCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasUnlockedCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingRootHashCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <subgamesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bondDistributionModeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameCreatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <vmCall as alloy_sol_types::SolCall>::SIGNATURE,
            <wethCall as alloy_sol_types::SolCall>::SIGNATURE,
            <attackCall as alloy_sol_types::SolCall>::SIGNATURE,
            <normalModeCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <challengerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingSequenceNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getNumToResolveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <anchorStateRegistryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <extraDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l1HeadCall as alloy_sol_types::SolCall>::SIGNATURE,
            <clockExtensionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <moveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <closeGameCall as alloy_sol_types::SolCall>::SIGNATURE,
            <defendCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimDataLenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <absolutePrestateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startingProposalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2SequenceNumberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolutionCheckpointsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proposerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <gameTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <rootClaimCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getChallengerDurationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <refundModeCreditCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRequiredBondCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createdAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creditCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SuperPermissionedDisputeGameCalls {
        const NAME: &'static str = "SuperPermissionedDisputeGameCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 49usize;
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
                Self::challenger(_) => {
                    <challengerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::proposer(_) => <proposerCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::startingProposal(_) => {
                    <startingProposalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingRootHash(_) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startingSequenceNumber(_) => {
                    <startingSequenceNumberCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls>] = &[
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::wasRespectedGameTypeWhenCreated,
                            )
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::subgames)
                    }
                    subgames
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::vm)
                    }
                    vm
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn challenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <challengerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::challenger)
                    }
                    challenger
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::version)
                    }
                    version
                },
                {
                    fn startingSequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <startingSequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::startingSequenceNumber,
                            )
                    }
                    startingSequenceNumber
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::r#move)
                    }
                    r#move
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn startingProposal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <startingProposalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::startingProposal)
                    }
                    startingProposal
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::resolutionCheckpoints,
                            )
                    }
                    resolutionCheckpoints
                },
                {
                    fn proposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <proposerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::proposer)
                    }
                    proposer
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::getChallengerDuration,
                            )
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::credit)
                    }
                    credit
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameCalls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::resolvedSubgames)
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
            ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls>] = &[
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::wasRespectedGameTypeWhenCreated,
                            )
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::subgames)
                    }
                    subgames
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::vm)
                    }
                    vm
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn challenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <challengerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::challenger)
                    }
                    challenger
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::version)
                    }
                    version
                },
                {
                    fn startingSequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <startingSequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::startingSequenceNumber,
                            )
                    }
                    startingSequenceNumber
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::r#move)
                    }
                    r#move
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn startingProposal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <startingProposalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::startingProposal)
                    }
                    startingProposal
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::resolutionCheckpoints,
                            )
                    }
                    resolutionCheckpoints
                },
                {
                    fn proposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <proposerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::proposer)
                    }
                    proposer
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameCalls::getChallengerDuration,
                            )
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::credit)
                    }
                    credit
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameCalls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameCalls::resolvedSubgames)
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
                Self::challenger(inner) => {
                    <challengerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::proposer(inner) => {
                    <proposerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::startingProposal(inner) => {
                    <startingProposalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingRootHash(inner) => {
                    <startingRootHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startingSequenceNumber(inner) => {
                    <startingSequenceNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::challenger(inner) => {
                    <challengerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::proposer(inner) => {
                    <proposerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::startingProposal(inner) => {
                    <startingProposalCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::startingSequenceNumber(inner) => {
                    <startingSequenceNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`SuperPermissionedDisputeGame`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SuperPermissionedDisputeGameErrors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        AnchorRootNotFound(AnchorRootNotFound),
        #[allow(missing_docs)]
        BadAuth(BadAuth),
        #[allow(missing_docs)]
        BadExtraData(BadExtraData),
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
        DuplicateStep(DuplicateStep),
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
        InvalidDisputedClaimIndex(InvalidDisputedClaimIndex),
        #[allow(missing_docs)]
        InvalidLocalIdent(InvalidLocalIdent),
        #[allow(missing_docs)]
        InvalidParent(InvalidParent),
        #[allow(missing_docs)]
        InvalidPrestate(InvalidPrestate),
        #[allow(missing_docs)]
        InvalidSplitDepth(InvalidSplitDepth),
        #[allow(missing_docs)]
        MaxDepthTooLarge(MaxDepthTooLarge),
        #[allow(missing_docs)]
        NoChainIdNeeded(NoChainIdNeeded),
        #[allow(missing_docs)]
        NoCreditToClaim(NoCreditToClaim),
        #[allow(missing_docs)]
        OutOfOrderResolution(OutOfOrderResolution),
        #[allow(missing_docs)]
        SuperFaultDisputeGameInvalidRootClaim(SuperFaultDisputeGameInvalidRootClaim),
        #[allow(missing_docs)]
        UnexpectedRootClaim(UnexpectedRootClaim),
        #[allow(missing_docs)]
        ValidStep(ValidStep),
    }
    impl SuperPermissionedDisputeGameErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 138u8, 61u8, 244u8],
            [13u8, 193u8, 73u8, 240u8],
            [23u8, 191u8, 229u8, 247u8],
            [34u8, 61u8, 179u8, 148u8],
            [44u8, 250u8, 192u8, 130u8],
            [48u8, 20u8, 3u8, 50u8],
            [51u8, 129u8, 209u8, 20u8],
            [55u8, 154u8, 126u8, 217u8],
            [72u8, 81u8, 189u8, 155u8],
            [86u8, 245u8, 123u8, 43u8],
            [95u8, 83u8, 221u8, 152u8],
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
            [164u8, 38u8, 55u8, 188u8],
            [179u8, 75u8, 92u8, 34u8],
            [180u8, 225u8, 36u8, 51u8],
            [193u8, 5u8, 38u8, 10u8],
            [211u8, 134u8, 239u8, 62u8],
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
            ::core::stringify!(NoCreditToClaim),
            ::core::stringify!(NoChainIdNeeded),
            ::core::stringify!(SuperFaultDisputeGameInvalidRootClaim),
            ::core::stringify!(InvalidDisputedClaimIndex),
            ::core::stringify!(ClockTimeExceeded),
            ::core::stringify!(GamePaused),
            ::core::stringify!(GameNotFinalized),
            ::core::stringify!(GameDepthExceeded),
            ::core::stringify!(InvalidParent),
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
            ::core::stringify!(CannotDefendRootClaim),
            ::core::stringify!(ClaimAboveSplit),
            ::core::stringify!(InvalidChallengePeriod),
            ::core::stringify!(GameNotResolved),
            ::core::stringify!(BadAuth),
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
            <NoCreditToClaim as alloy_sol_types::SolError>::SIGNATURE,
            <NoChainIdNeeded as alloy_sol_types::SolError>::SIGNATURE,
            <SuperFaultDisputeGameInvalidRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::SIGNATURE,
            <ClockTimeExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <GamePaused as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotFinalized as alloy_sol_types::SolError>::SIGNATURE,
            <GameDepthExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidParent as alloy_sol_types::SolError>::SIGNATURE,
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
            <CannotDefendRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <ClaimAboveSplit as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidChallengePeriod as alloy_sol_types::SolError>::SIGNATURE,
            <GameNotResolved as alloy_sol_types::SolError>::SIGNATURE,
            <BadAuth as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SuperPermissionedDisputeGameErrors {
        const NAME: &'static str = "SuperPermissionedDisputeGameErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AnchorRootNotFound(_) => {
                    <AnchorRootNotFound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BadAuth(_) => <BadAuth as alloy_sol_types::SolError>::SELECTOR,
                Self::BadExtraData(_) => {
                    <BadExtraData as alloy_sol_types::SolError>::SELECTOR
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
                Self::DuplicateStep(_) => {
                    <DuplicateStep as alloy_sol_types::SolError>::SELECTOR
                }
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
                Self::InvalidDisputedClaimIndex(_) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLocalIdent(_) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::SELECTOR
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
                Self::MaxDepthTooLarge(_) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoChainIdNeeded(_) => {
                    <NoChainIdNeeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoCreditToClaim(_) => {
                    <NoCreditToClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfOrderResolution(_) => {
                    <OutOfOrderResolution as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SuperFaultDisputeGameInvalidRootClaim(_) => {
                    <SuperFaultDisputeGameInvalidRootClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedRootClaim(_) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidBondDistributionMode,
                            )
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn NoChainIdNeeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <NoChainIdNeeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::NoChainIdNeeded)
                    }
                    NoChainIdNeeded
                },
                {
                    fn SuperFaultDisputeGameInvalidRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <SuperFaultDisputeGameInvalidRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::SuperFaultDisputeGameInvalidRootClaim,
                            )
                    }
                    SuperFaultDisputeGameInvalidRootClaim
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidDisputedClaimIndex,
                            )
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameErrors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidClockExtension,
                            )
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameErrors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::OutOfOrderResolution,
                            )
                    }
                    OutOfOrderResolution
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::CannotDefendRootClaim,
                            )
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidChallengePeriod,
                            )
                    }
                    InvalidChallengePeriod
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn BadAuth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <BadAuth as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameErrors::BadAuth)
                    }
                    BadAuth
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::ClaimAlreadyResolved,
                            )
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SuperPermissionedDisputeGameErrors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidLocalIdent)
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
            ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidBondDistributionMode,
                            )
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn NoChainIdNeeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <NoChainIdNeeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::NoChainIdNeeded)
                    }
                    NoChainIdNeeded
                },
                {
                    fn SuperFaultDisputeGameInvalidRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <SuperFaultDisputeGameInvalidRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::SuperFaultDisputeGameInvalidRootClaim,
                            )
                    }
                    SuperFaultDisputeGameInvalidRootClaim
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidDisputedClaimIndex,
                            )
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidClockExtension,
                            )
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::OutOfOrderResolution,
                            )
                    }
                    OutOfOrderResolution
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::CannotDefendRootClaim,
                            )
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::InvalidChallengePeriod,
                            )
                    }
                    InvalidChallengePeriod
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn BadAuth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <BadAuth as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::BadAuth)
                    }
                    BadAuth
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SuperPermissionedDisputeGameErrors::ClaimAlreadyResolved,
                            )
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SuperPermissionedDisputeGameErrors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SuperPermissionedDisputeGameErrors::InvalidLocalIdent)
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
                Self::BadAuth(inner) => {
                    <BadAuth as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BadExtraData(inner) => {
                    <BadExtraData as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::DuplicateStep(inner) => {
                    <DuplicateStep as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidDisputedClaimIndex(inner) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLocalIdent(inner) => {
                    <InvalidLocalIdent as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::MaxDepthTooLarge(inner) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoChainIdNeeded(inner) => {
                    <NoChainIdNeeded as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::SuperFaultDisputeGameInvalidRootClaim(inner) => {
                    <SuperFaultDisputeGameInvalidRootClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedRootClaim(inner) => {
                    <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::BadAuth(inner) => {
                    <BadAuth as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::BadExtraData(inner) => {
                    <BadExtraData as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::DuplicateStep(inner) => {
                    <DuplicateStep as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::InvalidDisputedClaimIndex(inner) => {
                    <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::MaxDepthTooLarge(inner) => {
                    <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoChainIdNeeded(inner) => {
                    <NoChainIdNeeded as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::SuperFaultDisputeGameInvalidRootClaim(inner) => {
                    <SuperFaultDisputeGameInvalidRootClaim as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ValidStep(inner) => {
                    <ValidStep as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SuperPermissionedDisputeGame`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SuperPermissionedDisputeGameEvents {
        #[allow(missing_docs)]
        GameClosed(GameClosed),
        #[allow(missing_docs)]
        Move(Move),
        #[allow(missing_docs)]
        Resolved(Resolved),
    }
    impl SuperPermissionedDisputeGameEvents {
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
    impl alloy_sol_types::SolEventInterface for SuperPermissionedDisputeGameEvents {
        const NAME: &'static str = "SuperPermissionedDisputeGameEvents";
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
    impl alloy_sol_types::private::IntoLogData for SuperPermissionedDisputeGameEvents {
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
    /**Creates a new wrapper around an on-chain [`SuperPermissionedDisputeGame`](self) contract instance.

See the [wrapper's documentation](`SuperPermissionedDisputeGameInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> SuperPermissionedDisputeGameInstance<P, N> {
        SuperPermissionedDisputeGameInstance::<P, N>::new(address, __provider)
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
        _params: <SuperFaultDisputeGame::GameConstructorParams as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SuperPermissionedDisputeGameInstance<P, N>>,
    > {
        SuperPermissionedDisputeGameInstance::<P, N>::deploy(__provider, _params)
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
        _params: <SuperFaultDisputeGame::GameConstructorParams as alloy::sol_types::SolType>::RustType,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        SuperPermissionedDisputeGameInstance::<P, N>::deploy_builder(__provider, _params)
    }
    /**A [`SuperPermissionedDisputeGame`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SuperPermissionedDisputeGame`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SuperPermissionedDisputeGameInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SuperPermissionedDisputeGameInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SuperPermissionedDisputeGameInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SuperPermissionedDisputeGameInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SuperPermissionedDisputeGame`](self) contract instance.

See the [wrapper's documentation](`SuperPermissionedDisputeGameInstance`) for more details.*/
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
            _params: <SuperFaultDisputeGame::GameConstructorParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<SuperPermissionedDisputeGameInstance<P, N>> {
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
            _params: <SuperFaultDisputeGame::GameConstructorParams as alloy::sol_types::SolType>::RustType,
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
    impl<P: ::core::clone::Clone, N> SuperPermissionedDisputeGameInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SuperPermissionedDisputeGameInstance<P, N> {
            SuperPermissionedDisputeGameInstance {
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
    > SuperPermissionedDisputeGameInstance<P, N> {
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
        ///Creates a new call builder for the [`challenger`] function.
        pub fn challenger(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, challengerCall, N> {
            self.call_builder(&challengerCall)
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
        ///Creates a new call builder for the [`proposer`] function.
        pub fn proposer(&self) -> alloy_contract::SolCallBuilder<&P, proposerCall, N> {
            self.call_builder(&proposerCall)
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
        ///Creates a new call builder for the [`startingProposal`] function.
        pub fn startingProposal(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingProposalCall, N> {
            self.call_builder(&startingProposalCall)
        }
        ///Creates a new call builder for the [`startingRootHash`] function.
        pub fn startingRootHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingRootHashCall, N> {
            self.call_builder(&startingRootHashCall)
        }
        ///Creates a new call builder for the [`startingSequenceNumber`] function.
        pub fn startingSequenceNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startingSequenceNumberCall, N> {
            self.call_builder(&startingSequenceNumberCall)
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
    > SuperPermissionedDisputeGameInstance<P, N> {
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

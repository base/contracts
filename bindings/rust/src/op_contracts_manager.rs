///Module containing a contract's types and functions.
/**

```solidity
library OPContractsManagerInteropMigrator {
    struct GameParameters { address proposer; address challenger; uint256 maxGameDepth; uint256 splitDepth; uint256 initBond; Duration clockExtension; Duration maxClockDuration; }
    struct MigrateInput { bool usePermissionlessGame; Proposal startingAnchorRoot; GameParameters gameParameters; OPContractsManager.OpChainConfig[] opChainConfigs; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod OPContractsManagerInteropMigrator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**```solidity
struct GameParameters { address proposer; address challenger; uint256 maxGameDepth; uint256 splitDepth; uint256 initBond; Duration clockExtension; Duration maxClockDuration; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GameParameters {
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxGameDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub splitDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initBond: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            Duration,
            Duration,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<GameParameters> for UnderlyingRustTuple<'_> {
            fn from(value: GameParameters) -> Self {
                (
                    value.proposer,
                    value.challenger,
                    value.maxGameDepth,
                    value.splitDepth,
                    value.initBond,
                    value.clockExtension,
                    value.maxClockDuration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GameParameters {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    proposer: tuple.0,
                    challenger: tuple.1,
                    maxGameDepth: tuple.2,
                    splitDepth: tuple.3,
                    initBond: tuple.4,
                    clockExtension: tuple.5,
                    maxClockDuration: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GameParameters {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GameParameters {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challenger,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxGameDepth),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.splitDepth),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initBond),
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
        impl alloy_sol_types::SolType for GameParameters {
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
        impl alloy_sol_types::SolStruct for GameParameters {
            const NAME: &'static str = "GameParameters";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GameParameters(address proposer,address challenger,uint256 maxGameDepth,uint256 splitDepth,uint256 initBond,Duration clockExtension,Duration maxClockDuration)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.challenger,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.initBond)
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
        impl alloy_sol_types::EventTopic for GameParameters {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposer,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.challenger,
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
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.initBond,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposer,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.challenger,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.initBond,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**```solidity
struct MigrateInput { bool usePermissionlessGame; Proposal startingAnchorRoot; GameParameters gameParameters; OPContractsManager.OpChainConfig[] opChainConfigs; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MigrateInput {
        #[allow(missing_docs)]
        pub usePermissionlessGame: bool,
        #[allow(missing_docs)]
        pub startingAnchorRoot: <Proposal as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub gameParameters: <GameParameters as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub opChainConfigs: alloy::sol_types::private::Vec<
            <OPContractsManager::OpChainConfig as alloy::sol_types::SolType>::RustType,
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
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bool,
            Proposal,
            GameParameters,
            alloy::sol_types::sol_data::Array<OPContractsManager::OpChainConfig>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            bool,
            <Proposal as alloy::sol_types::SolType>::RustType,
            <GameParameters as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<
                <OPContractsManager::OpChainConfig as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<MigrateInput> for UnderlyingRustTuple<'_> {
            fn from(value: MigrateInput) -> Self {
                (
                    value.usePermissionlessGame,
                    value.startingAnchorRoot,
                    value.gameParameters,
                    value.opChainConfigs,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MigrateInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    usePermissionlessGame: tuple.0,
                    startingAnchorRoot: tuple.1,
                    gameParameters: tuple.2,
                    opChainConfigs: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MigrateInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MigrateInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.usePermissionlessGame,
                    ),
                    <Proposal as alloy_sol_types::SolType>::tokenize(
                        &self.startingAnchorRoot,
                    ),
                    <GameParameters as alloy_sol_types::SolType>::tokenize(
                        &self.gameParameters,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        OPContractsManager::OpChainConfig,
                    > as alloy_sol_types::SolType>::tokenize(&self.opChainConfigs),
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
        impl alloy_sol_types::SolType for MigrateInput {
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
        impl alloy_sol_types::SolStruct for MigrateInput {
            const NAME: &'static str = "MigrateInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MigrateInput(bool usePermissionlessGame,Proposal startingAnchorRoot,GameParameters gameParameters,OPContractsManager.OpChainConfig[] opChainConfigs)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
                components
                    .push(<Proposal as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <Proposal as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <GameParameters as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <GameParameters as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <OPContractsManager::OpChainConfig as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OPContractsManager::OpChainConfig as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.usePermissionlessGame,
                        )
                        .0,
                    <Proposal as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startingAnchorRoot,
                        )
                        .0,
                    <GameParameters as alloy_sol_types::SolType>::eip712_data_word(
                            &self.gameParameters,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OPContractsManager::OpChainConfig,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.opChainConfigs,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MigrateInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.usePermissionlessGame,
                    )
                    + <Proposal as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startingAnchorRoot,
                    )
                    + <GameParameters as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gameParameters,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OPContractsManager::OpChainConfig,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.opChainConfigs,
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
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.usePermissionlessGame,
                    out,
                );
                <Proposal as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startingAnchorRoot,
                    out,
                );
                <GameParameters as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gameParameters,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OPContractsManager::OpChainConfig,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.opChainConfigs,
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
    /**Creates a new wrapper around an on-chain [`OPContractsManagerInteropMigrator`](self) contract instance.

See the [wrapper's documentation](`OPContractsManagerInteropMigratorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OPContractsManagerInteropMigratorInstance<P, N> {
        OPContractsManagerInteropMigratorInstance::<P, N>::new(address, __provider)
    }
    /**A [`OPContractsManagerInteropMigrator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OPContractsManagerInteropMigrator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OPContractsManagerInteropMigratorInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OPContractsManagerInteropMigratorInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OPContractsManagerInteropMigratorInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OPContractsManagerInteropMigratorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OPContractsManagerInteropMigrator`](self) contract instance.

See the [wrapper's documentation](`OPContractsManagerInteropMigratorInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> OPContractsManagerInteropMigratorInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> OPContractsManagerInteropMigratorInstance<P, N> {
            OPContractsManagerInteropMigratorInstance {
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
    > OPContractsManagerInteropMigratorInstance<P, N> {
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
    > OPContractsManagerInteropMigratorInstance<P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library OPContractsManagerStandardValidator {
    struct ValidationInput { address sysCfg; bytes32 absolutePrestate; uint256 l2ChainID; address proposer; }
    struct ValidationInputDev { address sysCfg; bytes32 cannonPrestate; bytes32 cannonKonaPrestate; uint256 l2ChainID; address proposer; }
    struct ValidationOverrides { address l1PAOMultisig; address challenger; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod OPContractsManagerStandardValidator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ValidationInput { address sysCfg; bytes32 absolutePrestate; uint256 l2ChainID; address proposer; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationInput {
        #[allow(missing_docs)]
        pub sysCfg: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub absolutePrestate: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub l2ChainID: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<ValidationInput> for UnderlyingRustTuple<'_> {
            fn from(value: ValidationInput) -> Self {
                (value.sysCfg, value.absolutePrestate, value.l2ChainID, value.proposer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidationInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sysCfg: tuple.0,
                    absolutePrestate: tuple.1,
                    l2ChainID: tuple.2,
                    proposer: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ValidationInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ValidationInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sysCfg,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.absolutePrestate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2ChainID),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
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
        impl alloy_sol_types::SolType for ValidationInput {
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
        impl alloy_sol_types::SolStruct for ValidationInput {
            const NAME: &'static str = "ValidationInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidationInput(address sysCfg,bytes32 absolutePrestate,uint256 l2ChainID,address proposer)",
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
                            &self.sysCfg,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.absolutePrestate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2ChainID)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposer,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ValidationInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sysCfg,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.absolutePrestate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2ChainID,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposer,
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
                    &rust.sysCfg,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.absolutePrestate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2ChainID,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposer,
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
    /**```solidity
struct ValidationInputDev { address sysCfg; bytes32 cannonPrestate; bytes32 cannonKonaPrestate; uint256 l2ChainID; address proposer; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationInputDev {
        #[allow(missing_docs)]
        pub sysCfg: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub cannonPrestate: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub cannonKonaPrestate: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub l2ChainID: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<ValidationInputDev> for UnderlyingRustTuple<'_> {
            fn from(value: ValidationInputDev) -> Self {
                (
                    value.sysCfg,
                    value.cannonPrestate,
                    value.cannonKonaPrestate,
                    value.l2ChainID,
                    value.proposer,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidationInputDev {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sysCfg: tuple.0,
                    cannonPrestate: tuple.1,
                    cannonKonaPrestate: tuple.2,
                    l2ChainID: tuple.3,
                    proposer: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ValidationInputDev {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ValidationInputDev {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sysCfg,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.cannonPrestate),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.cannonKonaPrestate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2ChainID),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
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
        impl alloy_sol_types::SolType for ValidationInputDev {
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
        impl alloy_sol_types::SolStruct for ValidationInputDev {
            const NAME: &'static str = "ValidationInputDev";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidationInputDev(address sysCfg,bytes32 cannonPrestate,bytes32 cannonKonaPrestate,uint256 l2ChainID,address proposer)",
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
                            &self.sysCfg,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cannonPrestate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cannonKonaPrestate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2ChainID)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposer,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ValidationInputDev {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sysCfg,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cannonPrestate,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cannonKonaPrestate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2ChainID,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposer,
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
                    &rust.sysCfg,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cannonPrestate,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cannonKonaPrestate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2ChainID,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposer,
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
    /**```solidity
struct ValidationOverrides { address l1PAOMultisig; address challenger; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationOverrides {
        #[allow(missing_docs)]
        pub l1PAOMultisig: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ValidationOverrides> for UnderlyingRustTuple<'_> {
            fn from(value: ValidationOverrides) -> Self {
                (value.l1PAOMultisig, value.challenger)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidationOverrides {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    l1PAOMultisig: tuple.0,
                    challenger: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ValidationOverrides {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ValidationOverrides {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1PAOMultisig,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challenger,
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
        impl alloy_sol_types::SolType for ValidationOverrides {
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
        impl alloy_sol_types::SolStruct for ValidationOverrides {
            const NAME: &'static str = "ValidationOverrides";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidationOverrides(address l1PAOMultisig,address challenger)",
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
                            &self.l1PAOMultisig,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.challenger,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ValidationOverrides {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1PAOMultisig,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.challenger,
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
                    &rust.l1PAOMultisig,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.challenger,
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
    /**Creates a new wrapper around an on-chain [`OPContractsManagerStandardValidator`](self) contract instance.

See the [wrapper's documentation](`OPContractsManagerStandardValidatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OPContractsManagerStandardValidatorInstance<P, N> {
        OPContractsManagerStandardValidatorInstance::<P, N>::new(address, __provider)
    }
    /**A [`OPContractsManagerStandardValidator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OPContractsManagerStandardValidator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OPContractsManagerStandardValidatorInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OPContractsManagerStandardValidatorInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OPContractsManagerStandardValidatorInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OPContractsManagerStandardValidatorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OPContractsManagerStandardValidator`](self) contract instance.

See the [wrapper's documentation](`OPContractsManagerStandardValidatorInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> OPContractsManagerStandardValidatorInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> OPContractsManagerStandardValidatorInstance<P, N> {
            OPContractsManagerStandardValidatorInstance {
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
    > OPContractsManagerStandardValidatorInstance<P, N> {
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
    > OPContractsManagerStandardValidatorInstance<P, N> {
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
library OPContractsManagerInteropMigrator {
    struct GameParameters {
        address proposer;
        address challenger;
        uint256 maxGameDepth;
        uint256 splitDepth;
        uint256 initBond;
        Duration clockExtension;
        Duration maxClockDuration;
    }
    struct MigrateInput {
        bool usePermissionlessGame;
        Proposal startingAnchorRoot;
        GameParameters gameParameters;
        OPContractsManager.OpChainConfig[] opChainConfigs;
    }
}

library OPContractsManagerStandardValidator {
    struct ValidationInput {
        address sysCfg;
        bytes32 absolutePrestate;
        uint256 l2ChainID;
        address proposer;
    }
    struct ValidationInputDev {
        address sysCfg;
        bytes32 cannonPrestate;
        bytes32 cannonKonaPrestate;
        uint256 l2ChainID;
        address proposer;
    }
    struct ValidationOverrides {
        address l1PAOMultisig;
        address challenger;
    }
}

interface OPContractsManager {
    type Claim is bytes32;
    type Duration is uint64;
    type GameType is uint32;
    type Hash is bytes32;
    struct AddGameInput {
        string saltMixer;
        address systemConfig;
        address delayedWETH;
        GameType disputeGameType;
        Claim disputeAbsolutePrestate;
        uint256 disputeMaxGameDepth;
        uint256 disputeSplitDepth;
        Duration disputeClockExtension;
        Duration disputeMaxClockDuration;
        uint256 initialBond;
        address vm;
        bool permissioned;
    }
    struct AddGameOutput {
        address delayedWETH;
        address faultDisputeGame;
    }
    struct Blueprints {
        address addressManager;
        address proxy;
        address proxyAdmin;
        address l1ChugSplashProxy;
        address resolvedDelegateProxy;
    }
    struct DeployInput {
        Roles roles;
        uint32 basefeeScalar;
        uint32 blobBasefeeScalar;
        uint256 l2ChainId;
        bytes startingAnchorRoot;
        string saltMixer;
        uint64 gasLimit;
        GameType disputeGameType;
        Claim disputeAbsolutePrestate;
        uint256 disputeMaxGameDepth;
        uint256 disputeSplitDepth;
        Duration disputeClockExtension;
        Duration disputeMaxClockDuration;
        bool useCustomGasToken;
    }
    struct DeployOutput {
        address opChainProxyAdmin;
        address addressManager;
        address l1ERC721BridgeProxy;
        address systemConfigProxy;
        address optimismMintableERC20FactoryProxy;
        address l1StandardBridgeProxy;
        address l1CrossDomainMessengerProxy;
        address ethLockboxProxy;
        address optimismPortalProxy;
        address disputeGameFactoryProxy;
        address anchorStateRegistryProxy;
        address faultDisputeGame;
        address permissionedDisputeGame;
        address delayedWETHPermissionedGameProxy;
        address delayedWETHPermissionlessGameProxy;
    }
    struct Implementations {
        address superchainConfigImpl;
        address protocolVersionsImpl;
        address l1ERC721BridgeImpl;
        address optimismPortalImpl;
        address optimismPortalInteropImpl;
        address ethLockboxImpl;
        address systemConfigImpl;
        address optimismMintableERC20FactoryImpl;
        address l1CrossDomainMessengerImpl;
        address l1StandardBridgeImpl;
        address disputeGameFactoryImpl;
        address anchorStateRegistryImpl;
        address delayedWETHImpl;
        address mipsImpl;
        address faultDisputeGameV2Impl;
        address permissionedDisputeGameV2Impl;
        address superFaultDisputeGameImpl;
        address superPermissionedDisputeGameImpl;
    }
    struct OpChainConfig {
        address systemConfigProxy;
        Claim cannonPrestate;
        Claim cannonKonaPrestate;
    }
    struct Proposal {
        Hash root;
        uint256 l2SequenceNumber;
    }
    struct Roles {
        address opChainProxyAdminOwner;
        address systemConfigOwner;
        address batcher;
        address unsafeBlockSigner;
        address proposer;
        address challenger;
    }
    struct UpdatePrestateInput {
        address systemConfigProxy;
        Claim cannonPrestate;
        Claim cannonKonaPrestate;
    }

    error AddressHasNoCode(address who);
    error AddressNotFound(address who);
    error AlreadyReleased();
    error InvalidChainId();
    error InvalidDevFeatureAccess(bytes32 devFeature);
    error InvalidGameConfigs();
    error InvalidRoleAddress(string role);
    error InvalidStartingAnchorRoot();
    error LatestReleaseNotSet();
    error OnlyDelegatecall();
    error PrestateNotSet();
    error PrestateRequired();
    error SuperchainConfigMismatch(address systemConfig);
    error SuperchainProxyAdminMismatch();

    constructor(address _opcmGameTypeAdder, address _opcmDeployer, address _opcmUpgrader, address _opcmInteropMigrator, address _opcmStandardValidator, address _superchainConfig, address _protocolVersions);

    function addGameType(AddGameInput[] memory _gameConfigs) external returns (AddGameOutput[] memory);
    function blueprints() external view returns (Blueprints memory);
    function chainIdToBatchInboxAddress(uint256 _l2ChainId) external view returns (address);
    function deploy(DeployInput memory _input) external returns (DeployOutput memory);
    function devFeatureBitmap() external view returns (bytes32);
    function implementations() external view returns (Implementations memory);
    function isDevFeatureEnabled(bytes32 _feature) external view returns (bool);
    function migrate(OPContractsManagerInteropMigrator.MigrateInput memory _input) external;
    function opcmDeployer() external view returns (address);
    function opcmGameTypeAdder() external view returns (address);
    function opcmInteropMigrator() external view returns (address);
    function opcmStandardValidator() external view returns (address);
    function opcmUpgrader() external view returns (address);
    function protocolVersions() external view returns (address);
    function superchainConfig() external view returns (address);
    function updatePrestate(UpdatePrestateInput[] memory _prestateUpdateInputs) external;
    function upgrade(OpChainConfig[] memory _opChainConfigs) external;
    function upgradeSuperchainConfig(address _superchainConfig) external;
    function validate(OPContractsManagerStandardValidator.ValidationInput memory _input, bool _allowFailure) external view returns (string memory);
    function validate(OPContractsManagerStandardValidator.ValidationInputDev memory _input, bool _allowFailure) external view returns (string memory);
    function validateWithOverrides(OPContractsManagerStandardValidator.ValidationInput memory _input, bool _allowFailure, OPContractsManagerStandardValidator.ValidationOverrides memory _overrides) external view returns (string memory);
    function validateWithOverrides(OPContractsManagerStandardValidator.ValidationInputDev memory _input, bool _allowFailure, OPContractsManagerStandardValidator.ValidationOverrides memory _overrides) external view returns (string memory);
    function version() external pure returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_opcmGameTypeAdder",
        "type": "address",
        "internalType": "contract OPContractsManagerGameTypeAdder"
      },
      {
        "name": "_opcmDeployer",
        "type": "address",
        "internalType": "contract OPContractsManagerDeployer"
      },
      {
        "name": "_opcmUpgrader",
        "type": "address",
        "internalType": "contract OPContractsManagerUpgrader"
      },
      {
        "name": "_opcmInteropMigrator",
        "type": "address",
        "internalType": "contract OPContractsManagerInteropMigrator"
      },
      {
        "name": "_opcmStandardValidator",
        "type": "address",
        "internalType": "contract OPContractsManagerStandardValidator"
      },
      {
        "name": "_superchainConfig",
        "type": "address",
        "internalType": "contract ISuperchainConfig"
      },
      {
        "name": "_protocolVersions",
        "type": "address",
        "internalType": "contract IProtocolVersions"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addGameType",
    "inputs": [
      {
        "name": "_gameConfigs",
        "type": "tuple[]",
        "internalType": "struct OPContractsManager.AddGameInput[]",
        "components": [
          {
            "name": "saltMixer",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "systemConfig",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "delayedWETH",
            "type": "address",
            "internalType": "contract IDelayedWETH"
          },
          {
            "name": "disputeGameType",
            "type": "uint32",
            "internalType": "GameType"
          },
          {
            "name": "disputeAbsolutePrestate",
            "type": "bytes32",
            "internalType": "Claim"
          },
          {
            "name": "disputeMaxGameDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "disputeSplitDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "disputeClockExtension",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "disputeMaxClockDuration",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "initialBond",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "vm",
            "type": "address",
            "internalType": "contract IBigStepper"
          },
          {
            "name": "permissioned",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct OPContractsManager.AddGameOutput[]",
        "components": [
          {
            "name": "delayedWETH",
            "type": "address",
            "internalType": "contract IDelayedWETH"
          },
          {
            "name": "faultDisputeGame",
            "type": "address",
            "internalType": "contract IFaultDisputeGame"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "blueprints",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OPContractsManager.Blueprints",
        "components": [
          {
            "name": "addressManager",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "proxy",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "proxyAdmin",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1ChugSplashProxy",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "resolvedDelegateProxy",
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
    "name": "chainIdToBatchInboxAddress",
    "inputs": [
      {
        "name": "_l2ChainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "deploy",
    "inputs": [
      {
        "name": "_input",
        "type": "tuple",
        "internalType": "struct OPContractsManager.DeployInput",
        "components": [
          {
            "name": "roles",
            "type": "tuple",
            "internalType": "struct OPContractsManager.Roles",
            "components": [
              {
                "name": "opChainProxyAdminOwner",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "systemConfigOwner",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "batcher",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "unsafeBlockSigner",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "proposer",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "challenger",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "basefeeScalar",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "blobBasefeeScalar",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "l2ChainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startingAnchorRoot",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "saltMixer",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "gasLimit",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "disputeGameType",
            "type": "uint32",
            "internalType": "GameType"
          },
          {
            "name": "disputeAbsolutePrestate",
            "type": "bytes32",
            "internalType": "Claim"
          },
          {
            "name": "disputeMaxGameDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "disputeSplitDepth",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "disputeClockExtension",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "disputeMaxClockDuration",
            "type": "uint64",
            "internalType": "Duration"
          },
          {
            "name": "useCustomGasToken",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OPContractsManager.DeployOutput",
        "components": [
          {
            "name": "opChainProxyAdmin",
            "type": "address",
            "internalType": "contract IProxyAdmin"
          },
          {
            "name": "addressManager",
            "type": "address",
            "internalType": "contract IAddressManager"
          },
          {
            "name": "l1ERC721BridgeProxy",
            "type": "address",
            "internalType": "contract IL1ERC721Bridge"
          },
          {
            "name": "systemConfigProxy",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "optimismMintableERC20FactoryProxy",
            "type": "address",
            "internalType": "contract IOptimismMintableERC20Factory"
          },
          {
            "name": "l1StandardBridgeProxy",
            "type": "address",
            "internalType": "contract IL1StandardBridge"
          },
          {
            "name": "l1CrossDomainMessengerProxy",
            "type": "address",
            "internalType": "contract IL1CrossDomainMessenger"
          },
          {
            "name": "ethLockboxProxy",
            "type": "address",
            "internalType": "contract IETHLockbox"
          },
          {
            "name": "optimismPortalProxy",
            "type": "address",
            "internalType": "contract IOptimismPortal2"
          },
          {
            "name": "disputeGameFactoryProxy",
            "type": "address",
            "internalType": "contract IDisputeGameFactory"
          },
          {
            "name": "anchorStateRegistryProxy",
            "type": "address",
            "internalType": "contract IAnchorStateRegistry"
          },
          {
            "name": "faultDisputeGame",
            "type": "address",
            "internalType": "contract IFaultDisputeGame"
          },
          {
            "name": "permissionedDisputeGame",
            "type": "address",
            "internalType": "contract IPermissionedDisputeGame"
          },
          {
            "name": "delayedWETHPermissionedGameProxy",
            "type": "address",
            "internalType": "contract IDelayedWETH"
          },
          {
            "name": "delayedWETHPermissionlessGameProxy",
            "type": "address",
            "internalType": "contract IDelayedWETH"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "devFeatureBitmap",
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
    "name": "implementations",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OPContractsManager.Implementations",
        "components": [
          {
            "name": "superchainConfigImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "protocolVersionsImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1ERC721BridgeImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "optimismPortalImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "optimismPortalInteropImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ethLockboxImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "systemConfigImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "optimismMintableERC20FactoryImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1CrossDomainMessengerImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "l1StandardBridgeImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "disputeGameFactoryImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "anchorStateRegistryImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delayedWETHImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "mipsImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "faultDisputeGameV2Impl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "permissionedDisputeGameV2Impl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "superFaultDisputeGameImpl",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "superPermissionedDisputeGameImpl",
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
    "name": "isDevFeatureEnabled",
    "inputs": [
      {
        "name": "_feature",
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
    "name": "migrate",
    "inputs": [
      {
        "name": "_input",
        "type": "tuple",
        "internalType": "struct OPContractsManagerInteropMigrator.MigrateInput",
        "components": [
          {
            "name": "usePermissionlessGame",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "startingAnchorRoot",
            "type": "tuple",
            "internalType": "struct Proposal",
            "components": [
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
            ]
          },
          {
            "name": "gameParameters",
            "type": "tuple",
            "internalType": "struct OPContractsManagerInteropMigrator.GameParameters",
            "components": [
              {
                "name": "proposer",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "challenger",
                "type": "address",
                "internalType": "address"
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
                "name": "initBond",
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
          },
          {
            "name": "opChainConfigs",
            "type": "tuple[]",
            "internalType": "struct OPContractsManager.OpChainConfig[]",
            "components": [
              {
                "name": "systemConfigProxy",
                "type": "address",
                "internalType": "contract ISystemConfig"
              },
              {
                "name": "cannonPrestate",
                "type": "bytes32",
                "internalType": "Claim"
              },
              {
                "name": "cannonKonaPrestate",
                "type": "bytes32",
                "internalType": "Claim"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "opcmDeployer",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract OPContractsManagerDeployer"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "opcmGameTypeAdder",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract OPContractsManagerGameTypeAdder"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "opcmInteropMigrator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract OPContractsManagerInteropMigrator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "opcmStandardValidator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract OPContractsManagerStandardValidator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "opcmUpgrader",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract OPContractsManagerUpgrader"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "protocolVersions",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IProtocolVersions"
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
    "name": "updatePrestate",
    "inputs": [
      {
        "name": "_prestateUpdateInputs",
        "type": "tuple[]",
        "internalType": "struct OPContractsManager.UpdatePrestateInput[]",
        "components": [
          {
            "name": "systemConfigProxy",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "cannonPrestate",
            "type": "bytes32",
            "internalType": "Claim"
          },
          {
            "name": "cannonKonaPrestate",
            "type": "bytes32",
            "internalType": "Claim"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "upgrade",
    "inputs": [
      {
        "name": "_opChainConfigs",
        "type": "tuple[]",
        "internalType": "struct OPContractsManager.OpChainConfig[]",
        "components": [
          {
            "name": "systemConfigProxy",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "cannonPrestate",
            "type": "bytes32",
            "internalType": "Claim"
          },
          {
            "name": "cannonKonaPrestate",
            "type": "bytes32",
            "internalType": "Claim"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "upgradeSuperchainConfig",
    "inputs": [
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
    "name": "validate",
    "inputs": [
      {
        "name": "_input",
        "type": "tuple",
        "internalType": "struct OPContractsManagerStandardValidator.ValidationInput",
        "components": [
          {
            "name": "sysCfg",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "absolutePrestate",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l2ChainID",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "proposer",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "_allowFailure",
        "type": "bool",
        "internalType": "bool"
      }
    ],
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
    "name": "validate",
    "inputs": [
      {
        "name": "_input",
        "type": "tuple",
        "internalType": "struct OPContractsManagerStandardValidator.ValidationInputDev",
        "components": [
          {
            "name": "sysCfg",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "cannonPrestate",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "cannonKonaPrestate",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l2ChainID",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "proposer",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "_allowFailure",
        "type": "bool",
        "internalType": "bool"
      }
    ],
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
    "name": "validateWithOverrides",
    "inputs": [
      {
        "name": "_input",
        "type": "tuple",
        "internalType": "struct OPContractsManagerStandardValidator.ValidationInput",
        "components": [
          {
            "name": "sysCfg",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "absolutePrestate",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l2ChainID",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "proposer",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "_allowFailure",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_overrides",
        "type": "tuple",
        "internalType": "struct OPContractsManagerStandardValidator.ValidationOverrides",
        "components": [
          {
            "name": "l1PAOMultisig",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "challenger",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
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
    "name": "validateWithOverrides",
    "inputs": [
      {
        "name": "_input",
        "type": "tuple",
        "internalType": "struct OPContractsManagerStandardValidator.ValidationInputDev",
        "components": [
          {
            "name": "sysCfg",
            "type": "address",
            "internalType": "contract ISystemConfig"
          },
          {
            "name": "cannonPrestate",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "cannonKonaPrestate",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "l2ChainID",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "proposer",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "_allowFailure",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_overrides",
        "type": "tuple",
        "internalType": "struct OPContractsManagerStandardValidator.ValidationOverrides",
        "components": [
          {
            "name": "l1PAOMultisig",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "challenger",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
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
    "type": "error",
    "name": "AddressHasNoCode",
    "inputs": [
      {
        "name": "who",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "AddressNotFound",
    "inputs": [
      {
        "name": "who",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "AlreadyReleased",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidChainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDevFeatureAccess",
    "inputs": [
      {
        "name": "devFeature",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidGameConfigs",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRoleAddress",
    "inputs": [
      {
        "name": "role",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidStartingAnchorRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LatestReleaseNotSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyDelegatecall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PrestateNotSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PrestateRequired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SuperchainConfigMismatch",
    "inputs": [
      {
        "name": "systemConfig",
        "type": "address",
        "internalType": "contract ISystemConfig"
      }
    ]
  },
  {
    "type": "error",
    "name": "SuperchainProxyAdminMismatch",
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
pub mod OPContractsManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101806040523480156200001257600080fd5b5060405162002d9138038062002d91833981016040819052620000359162000306565b60405163b6a4cd2160e01b81526001600160a01b03838116600483015287169063b6a4cd219060240160006040518083038186803b1580156200007757600080fd5b505afa1580156200008c573d6000803e3d6000fd5b505060405163b6a4cd2160e01b81526001600160a01b0384811660048301528916925063b6a4cd21915060240160006040518083038186803b158015620000d257600080fd5b505afa158015620000e7573d6000803e3d6000fd5b505060405163b6a4cd2160e01b81526001600160a01b038a811660048301528916925063b6a4cd21915060240160006040518083038186803b1580156200012d57600080fd5b505afa15801562000142573d6000803e3d6000fd5b505060405163b6a4cd2160e01b81526001600160a01b03891660048201819052925063b6a4cd21915060240160006040518083038186803b1580156200018757600080fd5b505afa1580156200019c573d6000803e3d6000fd5b505060405163b6a4cd2160e01b81526001600160a01b0388811660048301528916925063b6a4cd21915060240160006040518083038186803b158015620001e257600080fd5b505afa158015620001f7573d6000803e3d6000fd5b505060405163b6a4cd2160e01b81526001600160a01b0387811660048301528916925063b6a4cd21915060240160006040518083038186803b1580156200023d57600080fd5b505afa15801562000252573d6000803e3d6000fd5b505060405163b6a4cd2160e01b81526001600160a01b0386811660048301528916925063b6a4cd21915060240160006040518083038186803b1580156200029857600080fd5b505afa158015620002ad573d6000803e3d6000fd5b5050506001600160a01b039788166080525094861660a05292851660c05290841660e05283166101005282166101205216610140523061016052620003b1565b6001600160a01b03811681146200030357600080fd5b50565b600080600080600080600060e0888a0312156200032257600080fd5b87516200032f81620002ed565b60208901519097506200034281620002ed565b60408901519096506200035581620002ed565b60608901519095506200036881620002ed565b60808901519094506200037b81620002ed565b60a08901519093506200038e81620002ed565b60c0890151909250620003a181620002ed565b8091505092959891949750929550565b60805160a05160c05160e051610100516101205161014051610160516128ee620004a3600039600081816108590152818161095901528181610cbd01528181610e630152610f610152600061032f0152600081816102600152610b34015260008181610416015281816104cb015281816107cc01528181610c7a01526110830152600081816101fb015261092301526000818161019701528181610f32015261102b015260008181610308015281816105550152818161066d0152818161072001528181610afd01528181610bd40152610dd901526000818161043d01528181610a250152610d8701526128ee6000f3fe608060405234801561001057600080fd5b506004361061018d5760003560e01c8063622d56f1116100e3578063b51f9c2b1161008c578063c993f27c11610066578063c993f27c1461045f578063cbeda5a714610472578063f3edcbe11461048557600080fd5b8063b51f9c2b146103ba578063ba7903db14610411578063becbdf4a1461043857600080fd5b806378ecabce116100bd57806378ecabce146103715780638970ac4414610394578063b23cc044146103a757600080fd5b8063622d56f1146103035780636624856a1461032a5780636ab5f6611461035157600080fd5b8063318b1b801161014557806354fd4d501161011f57806354fd4d501461029557806358084273146102ce578063604aa628146102e357600080fd5b8063318b1b801461024857806335e80ab31461025b57806341fe53851461028257600080fd5b80631481a724116101765780631481a724146101f65780631d8a4e921461021d57806330e9012c1461023357600080fd5b806303dbe68c146101925780630e9d5cb9146101d6575b600080fd5b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6101e96101e436600461131a565b610498565b6040516101cd91906113bd565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b610225610551565b6040519081526020016101cd565b61023b6105da565b6040516101cd91906113d0565b6101b9610256366004611538565b6106ee565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6101e9610290366004611551565b610799565b60408051808201909152600581527f362e302e3000000000000000000000000000000000000000000000000000000060208201526101e9565b6102e16102dc366004611589565b61084f565b005b6102f66102f136600461169c565b61094d565b6040516101cd9190611831565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b61036461035f36600461188d565b610a60565b6040516101cd91906118c9565b61038461037f366004611538565b610ba2565b60405190151581526020016101cd565b6101e96103a2366004611a89565b610c47565b6102e16103b5366004611b6d565b610cb3565b6103c2610dac565b6040516101cd919081516001600160a01b039081168252602080840151821690830152604080840151821690830152606080840151821690830152608092830151169181019190915260a00190565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6102e161046d366004611bb6565b610e59565b6102e1610480366004611b6d565b610f57565b6101e9610493366004611bd3565b611050565b6040517f0e9d5cb90000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690630e9d5cb99061050490879087908790600401611c00565b600060405180830381865afa158015610521573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526105499190810190611c62565b949350505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631d8a4e926040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105b1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105d59190611cd9565b905090565b6040805161024081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101829052610160810182905261018081018290526101a081018290526101c081018290526101e0810182905261020081018290526102208101919091527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166330e9012c6040518163ffffffff1660e01b815260040161024060405180830381865afa1580156106ca573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105d59190611cfd565b6040517f318b1b80000000000000000000000000000000000000000000000000000000008152600481018290526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063318b1b8090602401602060405180830381865afa15801561076f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107939190611e55565b92915050565b6040517f41fe53850000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906341fe5385906108039086908690600401611e72565b600060405180830381865afa158015610820573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526108489190810190611c62565b9392505050565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630036108b1576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000816040516024016108c49190611f84565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f580842730000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b505050565b60606001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630036109b1576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000826040516024016109c49190612077565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f604aa6280000000000000000000000000000000000000000000000000000000017905290506000610a4a7f0000000000000000000000000000000000000000000000000000000000000000836110ba565b90508080602001905181019061054991906121ac565b604080516101e081018252600080825260208201819052818301819052606082018190526080820181905260a0820181905260c0820181905260e08201819052610100820181905261012082018190526101408201819052610160820181905261018082018190526101a082018190526101c082015290517fcefe12230000000000000000000000000000000000000000000000000000000081527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063cefe122390610b5e9085907f0000000000000000000000000000000000000000000000000000000000000000903390600401612395565b6101e0604051808303816000875af1158015610b7e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107939190612569565b6040517f78ecabce000000000000000000000000000000000000000000000000000000008152600481018290526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906378ecabce90602401602060405180830381865afa158015610c23573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107939190612680565b6040517f8970ac440000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690638970ac44906105049087908790879060040161269d565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003610d15576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600081604051602401610d28919061270e565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb23cc0440000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b6040805160a0810182526000808252602082018190529181018290526060810182905260808101919091527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b51f9c2b6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610e35573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105d59190612779565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003610ebb576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040516001600160a01b038216602482015260009060440160408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fc993f27c0000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003610fb9576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600081604051602401610fcc9190612811565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fcbeda5a70000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b6040517ff3edcbe10000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f3edcbe1906108039086908690600401612870565b6060600080846001600160a01b0316846040516110d791906128c5565b600060405180830381855af49150503d8060008114611112576040519150601f19603f3d011682016040523d82523d6000602084013e611117565b606091505b50915091508161054957805160208201fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040805190810167ffffffffffffffff8111828210171561117b5761117b611129565b60405290565b604051610180810167ffffffffffffffff8111828210171561117b5761117b611129565b604051610240810167ffffffffffffffff8111828210171561117b5761117b611129565b6040516101e0810167ffffffffffffffff8111828210171561117b5761117b611129565b604051601f8201601f1916810167ffffffffffffffff8111828210171561121657611216611129565b604052919050565b6001600160a01b038116811461123357600080fd5b50565b80356112418161121e565b919050565b60006080828403121561125857600080fd5b6040516080810181811067ffffffffffffffff8211171561127b5761127b611129565b604052905080823561128c8161121e565b80825250602083013560208201526040830135604082015260608301356112b28161121e565b6060919091015292915050565b801515811461123357600080fd5b8035611241816112bf565b6000604082840312156112ea57600080fd5b6112f2611158565b905081356112ff8161121e565b8152602082013561130f8161121e565b602082015292915050565b600080600060e0848603121561132f57600080fd5b6113398585611246565b92506080840135611349816112bf565b91506113588560a086016112d8565b90509250925092565b60005b8381101561137c578181015183820152602001611364565b8381111561138b576000848401525b50505050565b600081518084526113a9816020860160208601611361565b601f01601f19169290920160200192915050565b6020815260006108486020830184611391565b81516001600160a01b03168152610240810160208301516113fc60208401826001600160a01b03169052565b50604083015161141760408401826001600160a01b03169052565b50606083015161143260608401826001600160a01b03169052565b50608083015161144d60808401826001600160a01b03169052565b5060a083015161146860a08401826001600160a01b03169052565b5060c083015161148360c08401826001600160a01b03169052565b5060e083015161149e60e08401826001600160a01b03169052565b50610100838101516001600160a01b0390811691840191909152610120808501518216908401526101408085015182169084015261016080850151821690840152610180808501518216908401526101a0808501518216908401526101c0808501518216908401526101e080850151821690840152610200808501518216908401526102208085015191821681850152905b505092915050565b60006020828403121561154a57600080fd5b5035919050565b60008060a0838503121561156457600080fd5b61156e8484611246565b9150608083013561157e816112bf565b809150509250929050565b60006020828403121561159b57600080fd5b813567ffffffffffffffff8111156115b257600080fd5b8201610160818503121561084857600080fd5b600067ffffffffffffffff8211156115df576115df611129565b5060051b60200190565b600067ffffffffffffffff82111561160357611603611129565b50601f01601f191660200190565b600082601f83011261162257600080fd5b8135611635611630826115e9565b6111ed565b81815284602083860101111561164a57600080fd5b816020850160208301376000918101602001919091529392505050565b803563ffffffff8116811461124157600080fd5b67ffffffffffffffff8116811461123357600080fd5b80356112418161167b565b600060208083850312156116af57600080fd5b823567ffffffffffffffff808211156116c757600080fd5b818501915085601f8301126116db57600080fd5b81356116e9611630826115c5565b81815260059190911b8301840190848101908883111561170857600080fd5b8585015b83811015611824578035858111156117245760008081fd5b8601610180818c03601f190181131561173d5760008081fd5b611745611181565b89830135888111156117575760008081fd5b6117658e8c83870101611611565b8252506040611775818501611236565b8b8301526060611786818601611236565b8284015260809150611799828601611667565b818401525060a0808501358284015260c0915081850135818401525060e0808501358284015261010091506117cf828601611691565b908301526101206117e1858201611691565b828401526101409150818501358184015250610160611801818601611236565b828401526118108486016112cd565b90830152508552505091860191860161170c565b5098975050505050505050565b602080825282518282018190526000919060409081850190868401855b8281101561188057815180516001600160a01b039081168652908701511686850152928401929085019060010161184e565b5091979650505050505050565b60006020828403121561189f57600080fd5b813567ffffffffffffffff8111156118b657600080fd5b8201610260818503121561084857600080fd5b81516001600160a01b031681526101e0810160208301516118f560208401826001600160a01b03169052565b50604083015161191060408401826001600160a01b03169052565b50606083015161192b60608401826001600160a01b03169052565b50608083015161194660808401826001600160a01b03169052565b5060a083015161196160a08401826001600160a01b03169052565b5060c083015161197c60c08401826001600160a01b03169052565b5060e083015161199760e08401826001600160a01b03169052565b50610100838101516001600160a01b0390811691840191909152610120808501518216908401526101408085015182169084015261016080850151821690840152610180808501518216908401526101a0808501518216908401526101c0808501519182168185015290611530565b600060a08284031215611a1857600080fd5b60405160a0810181811067ffffffffffffffff82111715611a3b57611a3b611129565b6040529050808235611a4c8161121e565b808252506020830135602082015260408301356040820152606083013560608201526080830135611a7c8161121e565b6080919091015292915050565b60008060006101008486031215611a9f57600080fd5b611aa98585611a06565b925060a0840135611ab9816112bf565b91506113588560c086016112d8565b6000611ad6611630846115c5565b83815290506020808201906060808602850187811115611af557600080fd5b855b81811015611b615782818a031215611b0f5760008081fd5b6040805184810181811067ffffffffffffffff82111715611b3257611b32611129565b82528235611b3f8161121e565b8152828601358682015281830135918101919091528552938301938201611af7565b50505050509392505050565b600060208284031215611b7f57600080fd5b813567ffffffffffffffff811115611b9657600080fd5b8201601f81018413611ba757600080fd5b61054984823560208401611ac8565b600060208284031215611bc857600080fd5b81356108488161121e565b60008060c08385031215611be657600080fd5b611bf08484611a06565b915060a083013561157e816112bf565b60e08101611c38828680516001600160a01b039081168352602080830151908401526040808301519084015260609182015116910152565b831515608083015282516001600160a01b0390811660a084015260208401511660c0830152610549565b600060208284031215611c7457600080fd5b815167ffffffffffffffff811115611c8b57600080fd5b8201601f81018413611c9c57600080fd5b8051611caa611630826115e9565b818152856020838501011115611cbf57600080fd5b611cd0826020830160208601611361565b95945050505050565b600060208284031215611ceb57600080fd5b5051919050565b80516112418161121e565b60006102408284031215611d1057600080fd5b611d186111a5565b611d2183611cf2565b8152611d2f60208401611cf2565b6020820152611d4060408401611cf2565b6040820152611d5160608401611cf2565b6060820152611d6260808401611cf2565b6080820152611d7360a08401611cf2565b60a0820152611d8460c08401611cf2565b60c0820152611d9560e08401611cf2565b60e0820152610100611da8818501611cf2565b90820152610120611dba848201611cf2565b90820152610140611dcc848201611cf2565b90820152610160611dde848201611cf2565b90820152610180611df0848201611cf2565b908201526101a0611e02848201611cf2565b908201526101c0611e14848201611cf2565b908201526101e0611e26848201611cf2565b90820152610200611e38848201611cf2565b90820152610220611e4a848201611cf2565b908201529392505050565b600060208284031215611e6757600080fd5b81516108488161121e565b60a08101611eaa828580516001600160a01b039081168352602080830151908401526040808301519084015260609182015116910152565b82151560808301529392505050565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112611eee57600080fd5b830160208101925035905067ffffffffffffffff811115611f0e57600080fd5b606081023603821315611f2057600080fd5b9250929050565b8183526000602080850194508260005b85811015611f79578135611f4a8161121e565b6001600160a01b0316875281830135838801526040808301359088015260609687019690910190600101611f37565b509495945050505050565b6020815260008235611f95816112bf565b80151560208401525060208301356040830152604083013560608301526060830135611fc08161121e565b6001600160a01b03808216608085015260808501359150611fe08261121e565b80821660a0850152505060a083013560c083015260c083013560e083015261010060e0840135818401528084013590506120198161167b565b61012067ffffffffffffffff821681850152612036818601611691565b9150506101406120518185018367ffffffffffffffff169052565b61205d81860186611eb9565b6101608681015292509050611cd061018085018383611f27565b60006020808301818452808551808352604092508286019150828160051b87010184880160005b8381101561219e577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0898403018552815161018081518186526120e382870182611391565b915050888201516120fe8a8701826001600160a01b03169052565b50878201516001600160a01b03908116868a015260608084015163ffffffff16908701526080808401519087015260a0808401519087015260c0808401519087015260e08084015167ffffffffffffffff908116918801919091526101008085015190911690870152610120808401519087015261014080840151909116908601526101609182015115159190940152938601939086019060010161209e565b509098975050505050505050565b600060208083850312156121bf57600080fd5b825167ffffffffffffffff8111156121d657600080fd5b8301601f810185136121e757600080fd5b80516121f5611630826115c5565b81815260069190911b8201830190838101908783111561221457600080fd5b928401925b8284101561226a57604084890312156122325760008081fd5b61223a611158565b84516122458161121e565b8152848601516122548161121e565b8187015282526040939093019290840190612219565b979650505050505050565b80356122808161121e565b6001600160a01b03908116835260208201359061229c8261121e565b90811660208401526040820135906122b38261121e565b90811660408401526060820135906122ca8261121e565b90811660608401526080820135906122e18261121e565b908116608084015260a0820135906122f88261121e565b80821660a085015250505050565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe184360301811261233b57600080fd5b830160208101925035905067ffffffffffffffff81111561235b57600080fd5b803603821315611f2057600080fd5b818352818160208501375060006020828401015260006020601f19601f840116840101905092915050565b606081526123a66060820185612275565b60006123b460c08601611667565b6101206123c88185018363ffffffff169052565b6123d460e08801611667565b91506101406123ea8186018463ffffffff169052565b61016092506101008801358386015261240582890189612306565b925061026061018081818901526124216102c08901868561236a565b945061242f848c018c612306565b945092506101a07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa089870301818a015261246a86868661236a565b9550612477878d01611691565b96506101c09450612493858a018867ffffffffffffffff169052565b61249e828d01611667565b96506101e093506124b6848a018863ffffffff169052565b6102009650808c0135878a01525050610220838b0135818901526102409350828b0135848901526124e8868c01611691565b67ffffffffffffffff8116898401529550612504818c01611691565b955050505061252061028086018467ffffffffffffffff169052565b61252b8189016112cd565b92505061253d6102a085018315159052565b6001600160a01b038616602085015291506125559050565b6001600160a01b0383166040830152610549565b60006101e0828403121561257c57600080fd5b6125846111c9565b61258d83611cf2565b815261259b60208401611cf2565b60208201526125ac60408401611cf2565b60408201526125bd60608401611cf2565b60608201526125ce60808401611cf2565b60808201526125df60a08401611cf2565b60a08201526125f060c08401611cf2565b60c082015261260160e08401611cf2565b60e0820152610100612614818501611cf2565b90820152610120612626848201611cf2565b90820152610140612638848201611cf2565b9082015261016061264a848201611cf2565b9082015261018061265c848201611cf2565b908201526101a061266e848201611cf2565b908201526101c0611e4a848201611cf2565b60006020828403121561269257600080fd5b8151610848816112bf565b61010081016126e482866001600160a01b03808251168352602082015160208401526040820151604084015260608201516060840152806080830151166080840152505050565b83151560a083015282516001600160a01b0390811660c084015260208401511660e0830152610549565b6020808252825182820181905260009190848201906040850190845b8181101561276d5761275a83855180516001600160a01b0316825260208082015190830152604090810151910152565b928401926060929092019160010161272a565b50909695505050505050565b600060a0828403121561278b57600080fd5b60405160a0810181811067ffffffffffffffff821117156127ae576127ae611129565b60405282516127bc8161121e565b815260208301516127cc8161121e565b602082015260408301516127df8161121e565b604082015260608301516127f28161121e565b606082015260808301516128058161121e565b60808201529392505050565b6020808252825182820181905260009190848201906040850190845b8181101561276d5761285d83855180516001600160a01b0316825260208082015190830152604090810151910152565b928401926060929092019160010161282d565b60c081016128b682856001600160a01b03808251168352602082015160208401526040820151604084015260608201516060840152806080830151166080840152505050565b82151560a08301529392505050565b600082516128d7818460208701611361565b919091019291505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0-\x918\x03\x80b\0-\x91\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x03\x06V[`@Qc\xB6\xA4\xCD!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x87\x16\x90c\xB6\xA4\xCD!\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\0wW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\0\x8CW=`\0\x80>=`\0\xFD[PP`@Qc\xB6\xA4\xCD!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x89\x16\x92Pc\xB6\xA4\xCD!\x91P`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\0\xD2W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\0\xE7W=`\0\x80>=`\0\xFD[PP`@Qc\xB6\xA4\xCD!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x16\x92Pc\xB6\xA4\xCD!\x91P`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x01-W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x01BW=`\0\x80>=`\0\xFD[PP`@Qc\xB6\xA4\xCD!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16`\x04\x82\x01\x81\x90R\x92Pc\xB6\xA4\xCD!\x91P`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x01\x87W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x01\x9CW=`\0\x80>=`\0\xFD[PP`@Qc\xB6\xA4\xCD!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x89\x16\x92Pc\xB6\xA4\xCD!\x91P`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x01\xE2W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x01\xF7W=`\0\x80>=`\0\xFD[PP`@Qc\xB6\xA4\xCD!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x89\x16\x92Pc\xB6\xA4\xCD!\x91P`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x02=W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x02RW=`\0\x80>=`\0\xFD[PP`@Qc\xB6\xA4\xCD!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x89\x16\x92Pc\xB6\xA4\xCD!\x91P`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x02\x98W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x02\xADW=`\0\x80>=`\0\xFD[PPP`\x01`\x01`\xA0\x1B\x03\x97\x88\x16`\x80RP\x94\x86\x16`\xA0R\x92\x85\x16`\xC0R\x90\x84\x16`\xE0R\x83\x16a\x01\0R\x82\x16a\x01 R\x16a\x01@R0a\x01`Rb\0\x03\xB1V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03\x03W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15b\0\x03\"W`\0\x80\xFD[\x87Qb\0\x03/\x81b\0\x02\xEDV[` \x89\x01Q\x90\x97Pb\0\x03B\x81b\0\x02\xEDV[`@\x89\x01Q\x90\x96Pb\0\x03U\x81b\0\x02\xEDV[``\x89\x01Q\x90\x95Pb\0\x03h\x81b\0\x02\xEDV[`\x80\x89\x01Q\x90\x94Pb\0\x03{\x81b\0\x02\xEDV[`\xA0\x89\x01Q\x90\x93Pb\0\x03\x8E\x81b\0\x02\xEDV[`\xC0\x89\x01Q\x90\x92Pb\0\x03\xA1\x81b\0\x02\xEDV[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa(\xEEb\0\x04\xA3`\09`\0\x81\x81a\x08Y\x01R\x81\x81a\tY\x01R\x81\x81a\x0C\xBD\x01R\x81\x81a\x0Ec\x01Ra\x0Fa\x01R`\0a\x03/\x01R`\0\x81\x81a\x02`\x01Ra\x0B4\x01R`\0\x81\x81a\x04\x16\x01R\x81\x81a\x04\xCB\x01R\x81\x81a\x07\xCC\x01R\x81\x81a\x0Cz\x01Ra\x10\x83\x01R`\0\x81\x81a\x01\xFB\x01Ra\t#\x01R`\0\x81\x81a\x01\x97\x01R\x81\x81a\x0F2\x01Ra\x10+\x01R`\0\x81\x81a\x03\x08\x01R\x81\x81a\x05U\x01R\x81\x81a\x06m\x01R\x81\x81a\x07 \x01R\x81\x81a\n\xFD\x01R\x81\x81a\x0B\xD4\x01Ra\r\xD9\x01R`\0\x81\x81a\x04=\x01R\x81\x81a\n%\x01Ra\r\x87\x01Ra(\xEE`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8DW`\x005`\xE0\x1C\x80cb-V\xF1\x11a\0\xE3W\x80c\xB5\x1F\x9C+\x11a\0\x8CW\x80c\xC9\x93\xF2|\x11a\0fW\x80c\xC9\x93\xF2|\x14a\x04_W\x80c\xCB\xED\xA5\xA7\x14a\x04rW\x80c\xF3\xED\xCB\xE1\x14a\x04\x85W`\0\x80\xFD[\x80c\xB5\x1F\x9C+\x14a\x03\xBAW\x80c\xBAy\x03\xDB\x14a\x04\x11W\x80c\xBE\xCB\xDFJ\x14a\x048W`\0\x80\xFD[\x80cx\xEC\xAB\xCE\x11a\0\xBDW\x80cx\xEC\xAB\xCE\x14a\x03qW\x80c\x89p\xACD\x14a\x03\x94W\x80c\xB2<\xC0D\x14a\x03\xA7W`\0\x80\xFD[\x80cb-V\xF1\x14a\x03\x03W\x80cf$\x85j\x14a\x03*W\x80cj\xB5\xF6a\x14a\x03QW`\0\x80\xFD[\x80c1\x8B\x1B\x80\x11a\x01EW\x80cT\xFDMP\x11a\x01\x1FW\x80cT\xFDMP\x14a\x02\x95W\x80cX\x08Bs\x14a\x02\xCEW\x80c`J\xA6(\x14a\x02\xE3W`\0\x80\xFD[\x80c1\x8B\x1B\x80\x14a\x02HW\x80c5\xE8\n\xB3\x14a\x02[W\x80cA\xFES\x85\x14a\x02\x82W`\0\x80\xFD[\x80c\x14\x81\xA7$\x11a\x01vW\x80c\x14\x81\xA7$\x14a\x01\xF6W\x80c\x1D\x8AN\x92\x14a\x02\x1DW\x80c0\xE9\x01,\x14a\x023W`\0\x80\xFD[\x80c\x03\xDB\xE6\x8C\x14a\x01\x92W\x80c\x0E\x9D\\\xB9\x14a\x01\xD6W[`\0\x80\xFD[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE9a\x01\xE46`\x04a\x13\x1AV[a\x04\x98V[`@Qa\x01\xCD\x91\x90a\x13\xBDV[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02%a\x05QV[`@Q\x90\x81R` \x01a\x01\xCDV[a\x02;a\x05\xDAV[`@Qa\x01\xCD\x91\x90a\x13\xD0V[a\x01\xB9a\x02V6`\x04a\x158V[a\x06\xEEV[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xE9a\x02\x906`\x04a\x15QV[a\x07\x99V[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F6.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x01\xE9V[a\x02\xE1a\x02\xDC6`\x04a\x15\x89V[a\x08OV[\0[a\x02\xF6a\x02\xF16`\x04a\x16\x9CV[a\tMV[`@Qa\x01\xCD\x91\x90a\x181V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03da\x03_6`\x04a\x18\x8DV[a\n`V[`@Qa\x01\xCD\x91\x90a\x18\xC9V[a\x03\x84a\x03\x7F6`\x04a\x158V[a\x0B\xA2V[`@Q\x90\x15\x15\x81R` \x01a\x01\xCDV[a\x01\xE9a\x03\xA26`\x04a\x1A\x89V[a\x0CGV[a\x02\xE1a\x03\xB56`\x04a\x1BmV[a\x0C\xB3V[a\x03\xC2a\r\xACV[`@Qa\x01\xCD\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE1a\x04m6`\x04a\x1B\xB6V[a\x0EYV[a\x02\xE1a\x04\x806`\x04a\x1BmV[a\x0FWV[a\x01\xE9a\x04\x936`\x04a\x1B\xD3V[a\x10PV[`@Q\x7F\x0E\x9D\\\xB9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x0E\x9D\\\xB9\x90a\x05\x04\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x1C\0V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05I\x91\x90\x81\x01\x90a\x1CbV[\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1D\x8AN\x92`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x1C\xD9V[\x90P\x90V[`@\x80Qa\x02@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Ra\x01`\x81\x01\x82\x90Ra\x01\x80\x81\x01\x82\x90Ra\x01\xA0\x81\x01\x82\x90Ra\x01\xC0\x81\x01\x82\x90Ra\x01\xE0\x81\x01\x82\x90Ra\x02\0\x81\x01\x82\x90Ra\x02 \x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0\xE9\x01,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x1C\xFDV[`@Q\x7F1\x8B\x1B\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c1\x8B\x1B\x80\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x93\x91\x90a\x1EUV[\x92\x91PPV[`@Q\x7FA\xFES\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cA\xFES\x85\x90a\x08\x03\x90\x86\x90\x86\x90`\x04\x01a\x1ErV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08H\x91\x90\x81\x01\x90a\x1CbV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x08\xB1W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@Q`$\x01a\x08\xC4\x91\x90a\x1F\x84V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FX\x08Bs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[PPPV[```\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\t\xB1W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`@Q`$\x01a\t\xC4\x91\x90a wV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F`J\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P`\0a\nJ\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x10\xBAV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x05I\x91\x90a!\xACV[`@\x80Qa\x01\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x81\x83\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01\x81\x90R`\xE0\x82\x01\x81\x90Ra\x01\0\x82\x01\x81\x90Ra\x01 \x82\x01\x81\x90Ra\x01@\x82\x01\x81\x90Ra\x01`\x82\x01\x81\x90Ra\x01\x80\x82\x01\x81\x90Ra\x01\xA0\x82\x01\x81\x90Ra\x01\xC0\x82\x01R\x90Q\x7F\xCE\xFE\x12#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\xFE\x12#\x90a\x0B^\x90\x85\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x903\x90`\x04\x01a#\x95V[a\x01\xE0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x93\x91\x90a%iV[`@Q\x7Fx\xEC\xAB\xCE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cx\xEC\xAB\xCE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x93\x91\x90a&\x80V[`@Q\x7F\x89p\xACD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89p\xACD\x90a\x05\x04\x90\x87\x90\x87\x90\x87\x90`\x04\x01a&\x9DV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\r\x15W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@Q`$\x01a\r(\x91\x90a'\x0EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB2<\xC0D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB5\x1F\x9C+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a'yV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0E\xBBW`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`\0\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xC9\x93\xF2|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0F\xB9W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@Q`$\x01a\x0F\xCC\x91\x90a(\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xED\xA5\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[`@Q\x7F\xF3\xED\xCB\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3\xED\xCB\xE1\x90a\x08\x03\x90\x86\x90\x86\x90`\x04\x01a(pV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x10\xD7\x91\x90a(\xC5V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x11\x12W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\x17V[``\x91P[P\x91P\x91P\x81a\x05IW\x80Q` \x82\x01\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@R\x90V[`@Qa\x01\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@Qa\x02@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x12\x16Wa\x12\x16a\x11)V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x123W`\0\x80\xFD[PV[\x805a\x12A\x81a\x12\x1EV[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x12XW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12{Wa\x12{a\x11)V[`@R\x90P\x80\x825a\x12\x8C\x81a\x12\x1EV[\x80\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015a\x12\xB2\x81a\x12\x1EV[``\x91\x90\x91\x01R\x92\x91PPV[\x80\x15\x15\x81\x14a\x123W`\0\x80\xFD[\x805a\x12A\x81a\x12\xBFV[`\0`@\x82\x84\x03\x12\x15a\x12\xEAW`\0\x80\xFD[a\x12\xF2a\x11XV[\x90P\x815a\x12\xFF\x81a\x12\x1EV[\x81R` \x82\x015a\x13\x0F\x81a\x12\x1EV[` \x82\x01R\x92\x91PPV[`\0\x80`\0`\xE0\x84\x86\x03\x12\x15a\x13/W`\0\x80\xFD[a\x139\x85\x85a\x12FV[\x92P`\x80\x84\x015a\x13I\x81a\x12\xBFV[\x91Pa\x13X\x85`\xA0\x86\x01a\x12\xD8V[\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x13|W\x81\x81\x01Q\x83\x82\x01R` \x01a\x13dV[\x83\x81\x11\x15a\x13\x8BW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x13\xA9\x81` \x86\x01` \x86\x01a\x13aV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x08H` \x83\x01\x84a\x13\x91V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81Ra\x02@\x81\x01` \x83\x01Qa\x13\xFC` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa\x14\x17`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa\x142``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa\x14M`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa\x14h`\xA0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x83\x01Qa\x14\x83`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x83\x01Qa\x14\x9E`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01 \x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01@\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01`\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\x80\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xC0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xE0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x02\0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x02 \x80\x85\x01Q\x91\x82\x16\x81\x85\x01R\x90[PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15JW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a\x15dW`\0\x80\xFD[a\x15n\x84\x84a\x12FV[\x91P`\x80\x83\x015a\x15~\x81a\x12\xBFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\x9BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xB2W`\0\x80\xFD[\x82\x01a\x01`\x81\x85\x03\x12\x15a\x08HW`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\xDFWa\x15\xDFa\x11)V[P`\x05\x1B` \x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x16\x03Wa\x16\x03a\x11)V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x16\"W`\0\x80\xFD[\x815a\x165a\x160\x82a\x15\xE9V[a\x11\xEDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x16JW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12AW`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x123W`\0\x80\xFD[\x805a\x12A\x81a\x16{V[`\0` \x80\x83\x85\x03\x12\x15a\x16\xAFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xC7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x16\xDBW`\0\x80\xFD[\x815a\x16\xE9a\x160\x82a\x15\xC5V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x17\x08W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x18$W\x805\x85\x81\x11\x15a\x17$W`\0\x80\x81\xFD[\x86\x01a\x01\x80\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x17=W`\0\x80\x81\xFD[a\x17Ea\x11\x81V[\x89\x83\x015\x88\x81\x11\x15a\x17WW`\0\x80\x81\xFD[a\x17e\x8E\x8C\x83\x87\x01\x01a\x16\x11V[\x82RP`@a\x17u\x81\x85\x01a\x126V[\x8B\x83\x01R``a\x17\x86\x81\x86\x01a\x126V[\x82\x84\x01R`\x80\x91Pa\x17\x99\x82\x86\x01a\x16gV[\x81\x84\x01RP`\xA0\x80\x85\x015\x82\x84\x01R`\xC0\x91P\x81\x85\x015\x81\x84\x01RP`\xE0\x80\x85\x015\x82\x84\x01Ra\x01\0\x91Pa\x17\xCF\x82\x86\x01a\x16\x91V[\x90\x83\x01Ra\x01 a\x17\xE1\x85\x82\x01a\x16\x91V[\x82\x84\x01Ra\x01@\x91P\x81\x85\x015\x81\x84\x01RPa\x01`a\x18\x01\x81\x86\x01a\x126V[\x82\x84\x01Ra\x18\x10\x84\x86\x01a\x12\xCDV[\x90\x83\x01RP\x85RPP\x91\x86\x01\x91\x86\x01a\x17\x0CV[P\x98\x97PPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x18\x80W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x87\x01Q\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x18NV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x18\x9FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xB6W`\0\x80\xFD[\x82\x01a\x02`\x81\x85\x03\x12\x15a\x08HW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81Ra\x01\xE0\x81\x01` \x83\x01Qa\x18\xF5` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa\x19\x10`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa\x19+``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa\x19F`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa\x19a`\xA0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x83\x01Qa\x19|`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x83\x01Qa\x19\x97`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01 \x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01@\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01`\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\x80\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xC0\x80\x85\x01Q\x91\x82\x16\x81\x85\x01R\x90a\x150V[`\0`\xA0\x82\x84\x03\x12\x15a\x1A\x18W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1A;Wa\x1A;a\x11)V[`@R\x90P\x80\x825a\x1AL\x81a\x12\x1EV[\x80\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015a\x1A|\x81a\x12\x1EV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a\x1A\x9FW`\0\x80\xFD[a\x1A\xA9\x85\x85a\x1A\x06V[\x92P`\xA0\x84\x015a\x1A\xB9\x81a\x12\xBFV[\x91Pa\x13X\x85`\xC0\x86\x01a\x12\xD8V[`\0a\x1A\xD6a\x160\x84a\x15\xC5V[\x83\x81R\x90P` \x80\x82\x01\x90``\x80\x86\x02\x85\x01\x87\x81\x11\x15a\x1A\xF5W`\0\x80\xFD[\x85[\x81\x81\x10\x15a\x1BaW\x82\x81\x8A\x03\x12\x15a\x1B\x0FW`\0\x80\x81\xFD[`@\x80Q\x84\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B2Wa\x1B2a\x11)V[\x82R\x825a\x1B?\x81a\x12\x1EV[\x81R\x82\x86\x015\x86\x82\x01R\x81\x83\x015\x91\x81\x01\x91\x90\x91R\x85R\x93\x83\x01\x93\x82\x01a\x1A\xF7V[PPPPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1B\x7FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x96W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1B\xA7W`\0\x80\xFD[a\x05I\x84\x825` \x84\x01a\x1A\xC8V[`\0` \x82\x84\x03\x12\x15a\x1B\xC8W`\0\x80\xFD[\x815a\x08H\x81a\x12\x1EV[`\0\x80`\xC0\x83\x85\x03\x12\x15a\x1B\xE6W`\0\x80\xFD[a\x1B\xF0\x84\x84a\x1A\x06V[\x91P`\xA0\x83\x015a\x15~\x81a\x12\xBFV[`\xE0\x81\x01a\x1C8\x82\x86\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x90\x84\x01R`@\x80\x83\x01Q\x90\x84\x01R``\x91\x82\x01Q\x16\x91\x01RV[\x83\x15\x15`\x80\x83\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x84\x01R` \x84\x01Q\x16`\xC0\x83\x01Ra\x05IV[`\0` \x82\x84\x03\x12\x15a\x1CtW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x8BW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1C\x9CW`\0\x80\xFD[\x80Qa\x1C\xAAa\x160\x82a\x15\xE9V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x1C\xBFW`\0\x80\xFD[a\x1C\xD0\x82` \x83\x01` \x86\x01a\x13aV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xEBW`\0\x80\xFD[PQ\x91\x90PV[\x80Qa\x12A\x81a\x12\x1EV[`\0a\x02@\x82\x84\x03\x12\x15a\x1D\x10W`\0\x80\xFD[a\x1D\x18a\x11\xA5V[a\x1D!\x83a\x1C\xF2V[\x81Ra\x1D/` \x84\x01a\x1C\xF2V[` \x82\x01Ra\x1D@`@\x84\x01a\x1C\xF2V[`@\x82\x01Ra\x1DQ``\x84\x01a\x1C\xF2V[``\x82\x01Ra\x1Db`\x80\x84\x01a\x1C\xF2V[`\x80\x82\x01Ra\x1Ds`\xA0\x84\x01a\x1C\xF2V[`\xA0\x82\x01Ra\x1D\x84`\xC0\x84\x01a\x1C\xF2V[`\xC0\x82\x01Ra\x1D\x95`\xE0\x84\x01a\x1C\xF2V[`\xE0\x82\x01Ra\x01\0a\x1D\xA8\x81\x85\x01a\x1C\xF2V[\x90\x82\x01Ra\x01 a\x1D\xBA\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01@a\x1D\xCC\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01`a\x1D\xDE\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\x80a\x1D\xF0\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xA0a\x1E\x02\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xC0a\x1E\x14\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xE0a\x1E&\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x02\0a\x1E8\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x02 a\x1EJ\x84\x82\x01a\x1C\xF2V[\x90\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1EgW`\0\x80\xFD[\x81Qa\x08H\x81a\x12\x1EV[`\xA0\x81\x01a\x1E\xAA\x82\x85\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x90\x84\x01R`@\x80\x83\x01Q\x90\x84\x01R``\x91\x82\x01Q\x16\x91\x01RV[\x82\x15\x15`\x80\x83\x01R\x93\x92PPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x1E\xEEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x0EW`\0\x80\xFD[``\x81\x026\x03\x82\x13\x15a\x1F W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\x1FyW\x815a\x1FJ\x81a\x12\x1EV[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015\x83\x88\x01R`@\x80\x83\x015\x90\x88\x01R``\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a\x1F7V[P\x94\x95\x94PPPPPV[` \x81R`\0\x825a\x1F\x95\x81a\x12\xBFV[\x80\x15\x15` \x84\x01RP` \x83\x015`@\x83\x01R`@\x83\x015``\x83\x01R``\x83\x015a\x1F\xC0\x81a\x12\x1EV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\x80\x85\x01R`\x80\x85\x015\x91Pa\x1F\xE0\x82a\x12\x1EV[\x80\x82\x16`\xA0\x85\x01RPP`\xA0\x83\x015`\xC0\x83\x01R`\xC0\x83\x015`\xE0\x83\x01Ra\x01\0`\xE0\x84\x015\x81\x84\x01R\x80\x84\x015\x90Pa \x19\x81a\x16{V[a\x01 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x85\x01Ra 6\x81\x86\x01a\x16\x91V[\x91PPa\x01@a Q\x81\x85\x01\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a ]\x81\x86\x01\x86a\x1E\xB9V[a\x01`\x86\x81\x01R\x92P\x90Pa\x1C\xD0a\x01\x80\x85\x01\x83\x83a\x1F'V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a!\x9EW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x89\x84\x03\x01\x85R\x81Qa\x01\x80\x81Q\x81\x86Ra \xE3\x82\x87\x01\x82a\x13\x91V[\x91PP\x88\x82\x01Qa \xFE\x8A\x87\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P\x87\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86\x8A\x01R``\x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x87\x01R`\x80\x80\x84\x01Q\x90\x87\x01R`\xA0\x80\x84\x01Q\x90\x87\x01R`\xC0\x80\x84\x01Q\x90\x87\x01R`\xE0\x80\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x88\x01\x91\x90\x91Ra\x01\0\x80\x85\x01Q\x90\x91\x16\x90\x87\x01Ra\x01 \x80\x84\x01Q\x90\x87\x01Ra\x01@\x80\x84\x01Q\x90\x91\x16\x90\x86\x01Ra\x01`\x91\x82\x01Q\x15\x15\x91\x90\x94\x01R\x93\x86\x01\x93\x90\x86\x01\x90`\x01\x01a \x9EV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a!\xBFW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xD6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a!\xE7W`\0\x80\xFD[\x80Qa!\xF5a\x160\x82a\x15\xC5V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\"\x14W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"jW`@\x84\x89\x03\x12\x15a\"2W`\0\x80\x81\xFD[a\":a\x11XV[\x84Qa\"E\x81a\x12\x1EV[\x81R\x84\x86\x01Qa\"T\x81a\x12\x1EV[\x81\x87\x01R\x82R`@\x93\x90\x93\x01\x92\x90\x84\x01\x90a\"\x19V[\x97\x96PPPPPPPV[\x805a\"\x80\x81a\x12\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x82\x015\x90a\"\x9C\x82a\x12\x1EV[\x90\x81\x16` \x84\x01R`@\x82\x015\x90a\"\xB3\x82a\x12\x1EV[\x90\x81\x16`@\x84\x01R``\x82\x015\x90a\"\xCA\x82a\x12\x1EV[\x90\x81\x16``\x84\x01R`\x80\x82\x015\x90a\"\xE1\x82a\x12\x1EV[\x90\x81\x16`\x80\x84\x01R`\xA0\x82\x015\x90a\"\xF8\x82a\x12\x1EV[\x80\x82\x16`\xA0\x85\x01RPPPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#;W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#[W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x1F W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81Ra#\xA6``\x82\x01\x85a\"uV[`\0a#\xB4`\xC0\x86\x01a\x16gV[a\x01 a#\xC8\x81\x85\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[a#\xD4`\xE0\x88\x01a\x16gV[\x91Pa\x01@a#\xEA\x81\x86\x01\x84c\xFF\xFF\xFF\xFF\x16\x90RV[a\x01`\x92Pa\x01\0\x88\x015\x83\x86\x01Ra$\x05\x82\x89\x01\x89a#\x06V[\x92Pa\x02`a\x01\x80\x81\x81\x89\x01Ra$!a\x02\xC0\x89\x01\x86\x85a#jV[\x94Pa$/\x84\x8C\x01\x8Ca#\x06V[\x94P\x92Pa\x01\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x87\x03\x01\x81\x8A\x01Ra$j\x86\x86\x86a#jV[\x95Pa$w\x87\x8D\x01a\x16\x91V[\x96Pa\x01\xC0\x94Pa$\x93\x85\x8A\x01\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a$\x9E\x82\x8D\x01a\x16gV[\x96Pa\x01\xE0\x93Pa$\xB6\x84\x8A\x01\x88c\xFF\xFF\xFF\xFF\x16\x90RV[a\x02\0\x96P\x80\x8C\x015\x87\x8A\x01RPPa\x02 \x83\x8B\x015\x81\x89\x01Ra\x02@\x93P\x82\x8B\x015\x84\x89\x01Ra$\xE8\x86\x8C\x01a\x16\x91V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x89\x84\x01R\x95Pa%\x04\x81\x8C\x01a\x16\x91V[\x95PPPPa% a\x02\x80\x86\x01\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a%+\x81\x89\x01a\x12\xCDV[\x92PPa%=a\x02\xA0\x85\x01\x83\x15\x15\x90RV[`\x01`\x01`\xA0\x1B\x03\x86\x16` \x85\x01R\x91Pa%U\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`@\x83\x01Ra\x05IV[`\0a\x01\xE0\x82\x84\x03\x12\x15a%|W`\0\x80\xFD[a%\x84a\x11\xC9V[a%\x8D\x83a\x1C\xF2V[\x81Ra%\x9B` \x84\x01a\x1C\xF2V[` \x82\x01Ra%\xAC`@\x84\x01a\x1C\xF2V[`@\x82\x01Ra%\xBD``\x84\x01a\x1C\xF2V[``\x82\x01Ra%\xCE`\x80\x84\x01a\x1C\xF2V[`\x80\x82\x01Ra%\xDF`\xA0\x84\x01a\x1C\xF2V[`\xA0\x82\x01Ra%\xF0`\xC0\x84\x01a\x1C\xF2V[`\xC0\x82\x01Ra&\x01`\xE0\x84\x01a\x1C\xF2V[`\xE0\x82\x01Ra\x01\0a&\x14\x81\x85\x01a\x1C\xF2V[\x90\x82\x01Ra\x01 a&&\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01@a&8\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01`a&J\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\x80a&\\\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xA0a&n\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xC0a\x1EJ\x84\x82\x01a\x1C\xF2V[`\0` \x82\x84\x03\x12\x15a&\x92W`\0\x80\xFD[\x81Qa\x08H\x81a\x12\xBFV[a\x01\0\x81\x01a&\xE4\x82\x86`\x01`\x01`\xA0\x1B\x03\x80\x82Q\x16\x83R` \x82\x01Q` \x84\x01R`@\x82\x01Q`@\x84\x01R``\x82\x01Q``\x84\x01R\x80`\x80\x83\x01Q\x16`\x80\x84\x01RPPPV[\x83\x15\x15`\xA0\x83\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R` \x84\x01Q\x16`\xE0\x83\x01Ra\x05IV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a'mWa'Z\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a'*V[P\x90\x96\x95PPPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a'\x8BW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a'\xAEWa'\xAEa\x11)V[`@R\x82Qa'\xBC\x81a\x12\x1EV[\x81R` \x83\x01Qa'\xCC\x81a\x12\x1EV[` \x82\x01R`@\x83\x01Qa'\xDF\x81a\x12\x1EV[`@\x82\x01R``\x83\x01Qa'\xF2\x81a\x12\x1EV[``\x82\x01R`\x80\x83\x01Qa(\x05\x81a\x12\x1EV[`\x80\x82\x01R\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a'mWa(]\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a(-V[`\xC0\x81\x01a(\xB6\x82\x85`\x01`\x01`\xA0\x1B\x03\x80\x82Q\x16\x83R` \x82\x01Q` \x84\x01R`@\x82\x01Q`@\x84\x01R``\x82\x01Q``\x84\x01R\x80`\x80\x83\x01Q\x16`\x80\x84\x01RPPPV[\x82\x15\x15`\xA0\x83\x01R\x93\x92PPPV[`\0\x82Qa(\xD7\x81\x84` \x87\x01a\x13aV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061018d5760003560e01c8063622d56f1116100e3578063b51f9c2b1161008c578063c993f27c11610066578063c993f27c1461045f578063cbeda5a714610472578063f3edcbe11461048557600080fd5b8063b51f9c2b146103ba578063ba7903db14610411578063becbdf4a1461043857600080fd5b806378ecabce116100bd57806378ecabce146103715780638970ac4414610394578063b23cc044146103a757600080fd5b8063622d56f1146103035780636624856a1461032a5780636ab5f6611461035157600080fd5b8063318b1b801161014557806354fd4d501161011f57806354fd4d501461029557806358084273146102ce578063604aa628146102e357600080fd5b8063318b1b801461024857806335e80ab31461025b57806341fe53851461028257600080fd5b80631481a724116101765780631481a724146101f65780631d8a4e921461021d57806330e9012c1461023357600080fd5b806303dbe68c146101925780630e9d5cb9146101d6575b600080fd5b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6101e96101e436600461131a565b610498565b6040516101cd91906113bd565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b610225610551565b6040519081526020016101cd565b61023b6105da565b6040516101cd91906113d0565b6101b9610256366004611538565b6106ee565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6101e9610290366004611551565b610799565b60408051808201909152600581527f362e302e3000000000000000000000000000000000000000000000000000000060208201526101e9565b6102e16102dc366004611589565b61084f565b005b6102f66102f136600461169c565b61094d565b6040516101cd9190611831565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b61036461035f36600461188d565b610a60565b6040516101cd91906118c9565b61038461037f366004611538565b610ba2565b60405190151581526020016101cd565b6101e96103a2366004611a89565b610c47565b6102e16103b5366004611b6d565b610cb3565b6103c2610dac565b6040516101cd919081516001600160a01b039081168252602080840151821690830152604080840151821690830152606080840151821690830152608092830151169181019190915260a00190565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6101b97f000000000000000000000000000000000000000000000000000000000000000081565b6102e161046d366004611bb6565b610e59565b6102e1610480366004611b6d565b610f57565b6101e9610493366004611bd3565b611050565b6040517f0e9d5cb90000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690630e9d5cb99061050490879087908790600401611c00565b600060405180830381865afa158015610521573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526105499190810190611c62565b949350505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631d8a4e926040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105b1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105d59190611cd9565b905090565b6040805161024081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101829052610160810182905261018081018290526101a081018290526101c081018290526101e0810182905261020081018290526102208101919091527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166330e9012c6040518163ffffffff1660e01b815260040161024060405180830381865afa1580156106ca573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105d59190611cfd565b6040517f318b1b80000000000000000000000000000000000000000000000000000000008152600481018290526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063318b1b8090602401602060405180830381865afa15801561076f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107939190611e55565b92915050565b6040517f41fe53850000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906341fe5385906108039086908690600401611e72565b600060405180830381865afa158015610820573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526108489190810190611c62565b9392505050565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630036108b1576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000816040516024016108c49190611f84565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f580842730000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b505050565b60606001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630036109b1576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000826040516024016109c49190612077565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f604aa6280000000000000000000000000000000000000000000000000000000017905290506000610a4a7f0000000000000000000000000000000000000000000000000000000000000000836110ba565b90508080602001905181019061054991906121ac565b604080516101e081018252600080825260208201819052818301819052606082018190526080820181905260a0820181905260c0820181905260e08201819052610100820181905261012082018190526101408201819052610160820181905261018082018190526101a082018190526101c082015290517fcefe12230000000000000000000000000000000000000000000000000000000081527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063cefe122390610b5e9085907f0000000000000000000000000000000000000000000000000000000000000000903390600401612395565b6101e0604051808303816000875af1158015610b7e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107939190612569565b6040517f78ecabce000000000000000000000000000000000000000000000000000000008152600481018290526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906378ecabce90602401602060405180830381865afa158015610c23573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107939190612680565b6040517f8970ac440000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690638970ac44906105049087908790879060040161269d565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003610d15576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600081604051602401610d28919061270e565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb23cc0440000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b6040805160a0810182526000808252602082018190529181018290526060810182905260808101919091527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b51f9c2b6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610e35573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105d59190612779565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003610ebb576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040516001600160a01b038216602482015260009060440160408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fc993f27c0000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003610fb9576040517f0a57d61d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600081604051602401610fcc9190612811565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fcbeda5a70000000000000000000000000000000000000000000000000000000017905290506109487f0000000000000000000000000000000000000000000000000000000000000000826110ba565b6040517ff3edcbe10000000000000000000000000000000000000000000000000000000081526060906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f3edcbe1906108039086908690600401612870565b6060600080846001600160a01b0316846040516110d791906128c5565b600060405180830381855af49150503d8060008114611112576040519150601f19603f3d011682016040523d82523d6000602084013e611117565b606091505b50915091508161054957805160208201fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040805190810167ffffffffffffffff8111828210171561117b5761117b611129565b60405290565b604051610180810167ffffffffffffffff8111828210171561117b5761117b611129565b604051610240810167ffffffffffffffff8111828210171561117b5761117b611129565b6040516101e0810167ffffffffffffffff8111828210171561117b5761117b611129565b604051601f8201601f1916810167ffffffffffffffff8111828210171561121657611216611129565b604052919050565b6001600160a01b038116811461123357600080fd5b50565b80356112418161121e565b919050565b60006080828403121561125857600080fd5b6040516080810181811067ffffffffffffffff8211171561127b5761127b611129565b604052905080823561128c8161121e565b80825250602083013560208201526040830135604082015260608301356112b28161121e565b6060919091015292915050565b801515811461123357600080fd5b8035611241816112bf565b6000604082840312156112ea57600080fd5b6112f2611158565b905081356112ff8161121e565b8152602082013561130f8161121e565b602082015292915050565b600080600060e0848603121561132f57600080fd5b6113398585611246565b92506080840135611349816112bf565b91506113588560a086016112d8565b90509250925092565b60005b8381101561137c578181015183820152602001611364565b8381111561138b576000848401525b50505050565b600081518084526113a9816020860160208601611361565b601f01601f19169290920160200192915050565b6020815260006108486020830184611391565b81516001600160a01b03168152610240810160208301516113fc60208401826001600160a01b03169052565b50604083015161141760408401826001600160a01b03169052565b50606083015161143260608401826001600160a01b03169052565b50608083015161144d60808401826001600160a01b03169052565b5060a083015161146860a08401826001600160a01b03169052565b5060c083015161148360c08401826001600160a01b03169052565b5060e083015161149e60e08401826001600160a01b03169052565b50610100838101516001600160a01b0390811691840191909152610120808501518216908401526101408085015182169084015261016080850151821690840152610180808501518216908401526101a0808501518216908401526101c0808501518216908401526101e080850151821690840152610200808501518216908401526102208085015191821681850152905b505092915050565b60006020828403121561154a57600080fd5b5035919050565b60008060a0838503121561156457600080fd5b61156e8484611246565b9150608083013561157e816112bf565b809150509250929050565b60006020828403121561159b57600080fd5b813567ffffffffffffffff8111156115b257600080fd5b8201610160818503121561084857600080fd5b600067ffffffffffffffff8211156115df576115df611129565b5060051b60200190565b600067ffffffffffffffff82111561160357611603611129565b50601f01601f191660200190565b600082601f83011261162257600080fd5b8135611635611630826115e9565b6111ed565b81815284602083860101111561164a57600080fd5b816020850160208301376000918101602001919091529392505050565b803563ffffffff8116811461124157600080fd5b67ffffffffffffffff8116811461123357600080fd5b80356112418161167b565b600060208083850312156116af57600080fd5b823567ffffffffffffffff808211156116c757600080fd5b818501915085601f8301126116db57600080fd5b81356116e9611630826115c5565b81815260059190911b8301840190848101908883111561170857600080fd5b8585015b83811015611824578035858111156117245760008081fd5b8601610180818c03601f190181131561173d5760008081fd5b611745611181565b89830135888111156117575760008081fd5b6117658e8c83870101611611565b8252506040611775818501611236565b8b8301526060611786818601611236565b8284015260809150611799828601611667565b818401525060a0808501358284015260c0915081850135818401525060e0808501358284015261010091506117cf828601611691565b908301526101206117e1858201611691565b828401526101409150818501358184015250610160611801818601611236565b828401526118108486016112cd565b90830152508552505091860191860161170c565b5098975050505050505050565b602080825282518282018190526000919060409081850190868401855b8281101561188057815180516001600160a01b039081168652908701511686850152928401929085019060010161184e565b5091979650505050505050565b60006020828403121561189f57600080fd5b813567ffffffffffffffff8111156118b657600080fd5b8201610260818503121561084857600080fd5b81516001600160a01b031681526101e0810160208301516118f560208401826001600160a01b03169052565b50604083015161191060408401826001600160a01b03169052565b50606083015161192b60608401826001600160a01b03169052565b50608083015161194660808401826001600160a01b03169052565b5060a083015161196160a08401826001600160a01b03169052565b5060c083015161197c60c08401826001600160a01b03169052565b5060e083015161199760e08401826001600160a01b03169052565b50610100838101516001600160a01b0390811691840191909152610120808501518216908401526101408085015182169084015261016080850151821690840152610180808501518216908401526101a0808501518216908401526101c0808501519182168185015290611530565b600060a08284031215611a1857600080fd5b60405160a0810181811067ffffffffffffffff82111715611a3b57611a3b611129565b6040529050808235611a4c8161121e565b808252506020830135602082015260408301356040820152606083013560608201526080830135611a7c8161121e565b6080919091015292915050565b60008060006101008486031215611a9f57600080fd5b611aa98585611a06565b925060a0840135611ab9816112bf565b91506113588560c086016112d8565b6000611ad6611630846115c5565b83815290506020808201906060808602850187811115611af557600080fd5b855b81811015611b615782818a031215611b0f5760008081fd5b6040805184810181811067ffffffffffffffff82111715611b3257611b32611129565b82528235611b3f8161121e565b8152828601358682015281830135918101919091528552938301938201611af7565b50505050509392505050565b600060208284031215611b7f57600080fd5b813567ffffffffffffffff811115611b9657600080fd5b8201601f81018413611ba757600080fd5b61054984823560208401611ac8565b600060208284031215611bc857600080fd5b81356108488161121e565b60008060c08385031215611be657600080fd5b611bf08484611a06565b915060a083013561157e816112bf565b60e08101611c38828680516001600160a01b039081168352602080830151908401526040808301519084015260609182015116910152565b831515608083015282516001600160a01b0390811660a084015260208401511660c0830152610549565b600060208284031215611c7457600080fd5b815167ffffffffffffffff811115611c8b57600080fd5b8201601f81018413611c9c57600080fd5b8051611caa611630826115e9565b818152856020838501011115611cbf57600080fd5b611cd0826020830160208601611361565b95945050505050565b600060208284031215611ceb57600080fd5b5051919050565b80516112418161121e565b60006102408284031215611d1057600080fd5b611d186111a5565b611d2183611cf2565b8152611d2f60208401611cf2565b6020820152611d4060408401611cf2565b6040820152611d5160608401611cf2565b6060820152611d6260808401611cf2565b6080820152611d7360a08401611cf2565b60a0820152611d8460c08401611cf2565b60c0820152611d9560e08401611cf2565b60e0820152610100611da8818501611cf2565b90820152610120611dba848201611cf2565b90820152610140611dcc848201611cf2565b90820152610160611dde848201611cf2565b90820152610180611df0848201611cf2565b908201526101a0611e02848201611cf2565b908201526101c0611e14848201611cf2565b908201526101e0611e26848201611cf2565b90820152610200611e38848201611cf2565b90820152610220611e4a848201611cf2565b908201529392505050565b600060208284031215611e6757600080fd5b81516108488161121e565b60a08101611eaa828580516001600160a01b039081168352602080830151908401526040808301519084015260609182015116910152565b82151560808301529392505050565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112611eee57600080fd5b830160208101925035905067ffffffffffffffff811115611f0e57600080fd5b606081023603821315611f2057600080fd5b9250929050565b8183526000602080850194508260005b85811015611f79578135611f4a8161121e565b6001600160a01b0316875281830135838801526040808301359088015260609687019690910190600101611f37565b509495945050505050565b6020815260008235611f95816112bf565b80151560208401525060208301356040830152604083013560608301526060830135611fc08161121e565b6001600160a01b03808216608085015260808501359150611fe08261121e565b80821660a0850152505060a083013560c083015260c083013560e083015261010060e0840135818401528084013590506120198161167b565b61012067ffffffffffffffff821681850152612036818601611691565b9150506101406120518185018367ffffffffffffffff169052565b61205d81860186611eb9565b6101608681015292509050611cd061018085018383611f27565b60006020808301818452808551808352604092508286019150828160051b87010184880160005b8381101561219e577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0898403018552815161018081518186526120e382870182611391565b915050888201516120fe8a8701826001600160a01b03169052565b50878201516001600160a01b03908116868a015260608084015163ffffffff16908701526080808401519087015260a0808401519087015260c0808401519087015260e08084015167ffffffffffffffff908116918801919091526101008085015190911690870152610120808401519087015261014080840151909116908601526101609182015115159190940152938601939086019060010161209e565b509098975050505050505050565b600060208083850312156121bf57600080fd5b825167ffffffffffffffff8111156121d657600080fd5b8301601f810185136121e757600080fd5b80516121f5611630826115c5565b81815260069190911b8201830190838101908783111561221457600080fd5b928401925b8284101561226a57604084890312156122325760008081fd5b61223a611158565b84516122458161121e565b8152848601516122548161121e565b8187015282526040939093019290840190612219565b979650505050505050565b80356122808161121e565b6001600160a01b03908116835260208201359061229c8261121e565b90811660208401526040820135906122b38261121e565b90811660408401526060820135906122ca8261121e565b90811660608401526080820135906122e18261121e565b908116608084015260a0820135906122f88261121e565b80821660a085015250505050565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe184360301811261233b57600080fd5b830160208101925035905067ffffffffffffffff81111561235b57600080fd5b803603821315611f2057600080fd5b818352818160208501375060006020828401015260006020601f19601f840116840101905092915050565b606081526123a66060820185612275565b60006123b460c08601611667565b6101206123c88185018363ffffffff169052565b6123d460e08801611667565b91506101406123ea8186018463ffffffff169052565b61016092506101008801358386015261240582890189612306565b925061026061018081818901526124216102c08901868561236a565b945061242f848c018c612306565b945092506101a07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa089870301818a015261246a86868661236a565b9550612477878d01611691565b96506101c09450612493858a018867ffffffffffffffff169052565b61249e828d01611667565b96506101e093506124b6848a018863ffffffff169052565b6102009650808c0135878a01525050610220838b0135818901526102409350828b0135848901526124e8868c01611691565b67ffffffffffffffff8116898401529550612504818c01611691565b955050505061252061028086018467ffffffffffffffff169052565b61252b8189016112cd565b92505061253d6102a085018315159052565b6001600160a01b038616602085015291506125559050565b6001600160a01b0383166040830152610549565b60006101e0828403121561257c57600080fd5b6125846111c9565b61258d83611cf2565b815261259b60208401611cf2565b60208201526125ac60408401611cf2565b60408201526125bd60608401611cf2565b60608201526125ce60808401611cf2565b60808201526125df60a08401611cf2565b60a08201526125f060c08401611cf2565b60c082015261260160e08401611cf2565b60e0820152610100612614818501611cf2565b90820152610120612626848201611cf2565b90820152610140612638848201611cf2565b9082015261016061264a848201611cf2565b9082015261018061265c848201611cf2565b908201526101a061266e848201611cf2565b908201526101c0611e4a848201611cf2565b60006020828403121561269257600080fd5b8151610848816112bf565b61010081016126e482866001600160a01b03808251168352602082015160208401526040820151604084015260608201516060840152806080830151166080840152505050565b83151560a083015282516001600160a01b0390811660c084015260208401511660e0830152610549565b6020808252825182820181905260009190848201906040850190845b8181101561276d5761275a83855180516001600160a01b0316825260208082015190830152604090810151910152565b928401926060929092019160010161272a565b50909695505050505050565b600060a0828403121561278b57600080fd5b60405160a0810181811067ffffffffffffffff821117156127ae576127ae611129565b60405282516127bc8161121e565b815260208301516127cc8161121e565b602082015260408301516127df8161121e565b604082015260608301516127f28161121e565b606082015260808301516128058161121e565b60808201529392505050565b6020808252825182820181905260009190848201906040850190845b8181101561276d5761285d83855180516001600160a01b0316825260208082015190830152604090810151910152565b928401926060929092019160010161282d565b60c081016128b682856001600160a01b03808251168352602082015160208401526040820151604084015260608201516060840152806080830151166080840152505050565b82151560a08301529392505050565b600082516128d7818460208701611361565b919091019291505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8DW`\x005`\xE0\x1C\x80cb-V\xF1\x11a\0\xE3W\x80c\xB5\x1F\x9C+\x11a\0\x8CW\x80c\xC9\x93\xF2|\x11a\0fW\x80c\xC9\x93\xF2|\x14a\x04_W\x80c\xCB\xED\xA5\xA7\x14a\x04rW\x80c\xF3\xED\xCB\xE1\x14a\x04\x85W`\0\x80\xFD[\x80c\xB5\x1F\x9C+\x14a\x03\xBAW\x80c\xBAy\x03\xDB\x14a\x04\x11W\x80c\xBE\xCB\xDFJ\x14a\x048W`\0\x80\xFD[\x80cx\xEC\xAB\xCE\x11a\0\xBDW\x80cx\xEC\xAB\xCE\x14a\x03qW\x80c\x89p\xACD\x14a\x03\x94W\x80c\xB2<\xC0D\x14a\x03\xA7W`\0\x80\xFD[\x80cb-V\xF1\x14a\x03\x03W\x80cf$\x85j\x14a\x03*W\x80cj\xB5\xF6a\x14a\x03QW`\0\x80\xFD[\x80c1\x8B\x1B\x80\x11a\x01EW\x80cT\xFDMP\x11a\x01\x1FW\x80cT\xFDMP\x14a\x02\x95W\x80cX\x08Bs\x14a\x02\xCEW\x80c`J\xA6(\x14a\x02\xE3W`\0\x80\xFD[\x80c1\x8B\x1B\x80\x14a\x02HW\x80c5\xE8\n\xB3\x14a\x02[W\x80cA\xFES\x85\x14a\x02\x82W`\0\x80\xFD[\x80c\x14\x81\xA7$\x11a\x01vW\x80c\x14\x81\xA7$\x14a\x01\xF6W\x80c\x1D\x8AN\x92\x14a\x02\x1DW\x80c0\xE9\x01,\x14a\x023W`\0\x80\xFD[\x80c\x03\xDB\xE6\x8C\x14a\x01\x92W\x80c\x0E\x9D\\\xB9\x14a\x01\xD6W[`\0\x80\xFD[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE9a\x01\xE46`\x04a\x13\x1AV[a\x04\x98V[`@Qa\x01\xCD\x91\x90a\x13\xBDV[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02%a\x05QV[`@Q\x90\x81R` \x01a\x01\xCDV[a\x02;a\x05\xDAV[`@Qa\x01\xCD\x91\x90a\x13\xD0V[a\x01\xB9a\x02V6`\x04a\x158V[a\x06\xEEV[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xE9a\x02\x906`\x04a\x15QV[a\x07\x99V[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F6.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01Ra\x01\xE9V[a\x02\xE1a\x02\xDC6`\x04a\x15\x89V[a\x08OV[\0[a\x02\xF6a\x02\xF16`\x04a\x16\x9CV[a\tMV[`@Qa\x01\xCD\x91\x90a\x181V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03da\x03_6`\x04a\x18\x8DV[a\n`V[`@Qa\x01\xCD\x91\x90a\x18\xC9V[a\x03\x84a\x03\x7F6`\x04a\x158V[a\x0B\xA2V[`@Q\x90\x15\x15\x81R` \x01a\x01\xCDV[a\x01\xE9a\x03\xA26`\x04a\x1A\x89V[a\x0CGV[a\x02\xE1a\x03\xB56`\x04a\x1BmV[a\x0C\xB3V[a\x03\xC2a\r\xACV[`@Qa\x01\xCD\x91\x90\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE1a\x04m6`\x04a\x1B\xB6V[a\x0EYV[a\x02\xE1a\x04\x806`\x04a\x1BmV[a\x0FWV[a\x01\xE9a\x04\x936`\x04a\x1B\xD3V[a\x10PV[`@Q\x7F\x0E\x9D\\\xB9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x0E\x9D\\\xB9\x90a\x05\x04\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x1C\0V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05I\x91\x90\x81\x01\x90a\x1CbV[\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1D\x8AN\x92`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x1C\xD9V[\x90P\x90V[`@\x80Qa\x02@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Ra\x01`\x81\x01\x82\x90Ra\x01\x80\x81\x01\x82\x90Ra\x01\xA0\x81\x01\x82\x90Ra\x01\xC0\x81\x01\x82\x90Ra\x01\xE0\x81\x01\x82\x90Ra\x02\0\x81\x01\x82\x90Ra\x02 \x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0\xE9\x01,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x1C\xFDV[`@Q\x7F1\x8B\x1B\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c1\x8B\x1B\x80\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x93\x91\x90a\x1EUV[\x92\x91PPV[`@Q\x7FA\xFES\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cA\xFES\x85\x90a\x08\x03\x90\x86\x90\x86\x90`\x04\x01a\x1ErV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08H\x91\x90\x81\x01\x90a\x1CbV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x08\xB1W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@Q`$\x01a\x08\xC4\x91\x90a\x1F\x84V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FX\x08Bs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[PPPV[```\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\t\xB1W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`@Q`$\x01a\t\xC4\x91\x90a wV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F`J\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P`\0a\nJ\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x10\xBAV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x05I\x91\x90a!\xACV[`@\x80Qa\x01\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x81\x83\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01\x81\x90R`\xE0\x82\x01\x81\x90Ra\x01\0\x82\x01\x81\x90Ra\x01 \x82\x01\x81\x90Ra\x01@\x82\x01\x81\x90Ra\x01`\x82\x01\x81\x90Ra\x01\x80\x82\x01\x81\x90Ra\x01\xA0\x82\x01\x81\x90Ra\x01\xC0\x82\x01R\x90Q\x7F\xCE\xFE\x12#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\xFE\x12#\x90a\x0B^\x90\x85\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x903\x90`\x04\x01a#\x95V[a\x01\xE0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x93\x91\x90a%iV[`@Q\x7Fx\xEC\xAB\xCE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cx\xEC\xAB\xCE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x93\x91\x90a&\x80V[`@Q\x7F\x89p\xACD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x89p\xACD\x90a\x05\x04\x90\x87\x90\x87\x90\x87\x90`\x04\x01a&\x9DV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\r\x15W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@Q`$\x01a\r(\x91\x90a'\x0EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB2<\xC0D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB5\x1F\x9C+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a'yV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0E\xBBW`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`\0\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xC9\x93\xF2|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0F\xB9W`@Q\x7F\nW\xD6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@Q`$\x01a\x0F\xCC\x91\x90a(\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xED\xA5\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Pa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\xBAV[`@Q\x7F\xF3\xED\xCB\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3\xED\xCB\xE1\x90a\x08\x03\x90\x86\x90\x86\x90`\x04\x01a(pV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x10\xD7\x91\x90a(\xC5V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x11\x12W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\x17V[``\x91P[P\x91P\x91P\x81a\x05IW\x80Q` \x82\x01\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@R\x90V[`@Qa\x01\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@Qa\x02@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11{Wa\x11{a\x11)V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x12\x16Wa\x12\x16a\x11)V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x123W`\0\x80\xFD[PV[\x805a\x12A\x81a\x12\x1EV[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x12XW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12{Wa\x12{a\x11)V[`@R\x90P\x80\x825a\x12\x8C\x81a\x12\x1EV[\x80\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015a\x12\xB2\x81a\x12\x1EV[``\x91\x90\x91\x01R\x92\x91PPV[\x80\x15\x15\x81\x14a\x123W`\0\x80\xFD[\x805a\x12A\x81a\x12\xBFV[`\0`@\x82\x84\x03\x12\x15a\x12\xEAW`\0\x80\xFD[a\x12\xF2a\x11XV[\x90P\x815a\x12\xFF\x81a\x12\x1EV[\x81R` \x82\x015a\x13\x0F\x81a\x12\x1EV[` \x82\x01R\x92\x91PPV[`\0\x80`\0`\xE0\x84\x86\x03\x12\x15a\x13/W`\0\x80\xFD[a\x139\x85\x85a\x12FV[\x92P`\x80\x84\x015a\x13I\x81a\x12\xBFV[\x91Pa\x13X\x85`\xA0\x86\x01a\x12\xD8V[\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x13|W\x81\x81\x01Q\x83\x82\x01R` \x01a\x13dV[\x83\x81\x11\x15a\x13\x8BW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x13\xA9\x81` \x86\x01` \x86\x01a\x13aV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x08H` \x83\x01\x84a\x13\x91V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81Ra\x02@\x81\x01` \x83\x01Qa\x13\xFC` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa\x14\x17`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa\x142``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa\x14M`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa\x14h`\xA0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x83\x01Qa\x14\x83`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x83\x01Qa\x14\x9E`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01 \x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01@\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01`\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\x80\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xC0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xE0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x02\0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x02 \x80\x85\x01Q\x91\x82\x16\x81\x85\x01R\x90[PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15JW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a\x15dW`\0\x80\xFD[a\x15n\x84\x84a\x12FV[\x91P`\x80\x83\x015a\x15~\x81a\x12\xBFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\x9BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xB2W`\0\x80\xFD[\x82\x01a\x01`\x81\x85\x03\x12\x15a\x08HW`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\xDFWa\x15\xDFa\x11)V[P`\x05\x1B` \x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x16\x03Wa\x16\x03a\x11)V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x16\"W`\0\x80\xFD[\x815a\x165a\x160\x82a\x15\xE9V[a\x11\xEDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x16JW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12AW`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x123W`\0\x80\xFD[\x805a\x12A\x81a\x16{V[`\0` \x80\x83\x85\x03\x12\x15a\x16\xAFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xC7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x16\xDBW`\0\x80\xFD[\x815a\x16\xE9a\x160\x82a\x15\xC5V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x17\x08W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x18$W\x805\x85\x81\x11\x15a\x17$W`\0\x80\x81\xFD[\x86\x01a\x01\x80\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x17=W`\0\x80\x81\xFD[a\x17Ea\x11\x81V[\x89\x83\x015\x88\x81\x11\x15a\x17WW`\0\x80\x81\xFD[a\x17e\x8E\x8C\x83\x87\x01\x01a\x16\x11V[\x82RP`@a\x17u\x81\x85\x01a\x126V[\x8B\x83\x01R``a\x17\x86\x81\x86\x01a\x126V[\x82\x84\x01R`\x80\x91Pa\x17\x99\x82\x86\x01a\x16gV[\x81\x84\x01RP`\xA0\x80\x85\x015\x82\x84\x01R`\xC0\x91P\x81\x85\x015\x81\x84\x01RP`\xE0\x80\x85\x015\x82\x84\x01Ra\x01\0\x91Pa\x17\xCF\x82\x86\x01a\x16\x91V[\x90\x83\x01Ra\x01 a\x17\xE1\x85\x82\x01a\x16\x91V[\x82\x84\x01Ra\x01@\x91P\x81\x85\x015\x81\x84\x01RPa\x01`a\x18\x01\x81\x86\x01a\x126V[\x82\x84\x01Ra\x18\x10\x84\x86\x01a\x12\xCDV[\x90\x83\x01RP\x85RPP\x91\x86\x01\x91\x86\x01a\x17\x0CV[P\x98\x97PPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x18\x80W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x90\x87\x01Q\x16\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x18NV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x18\x9FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xB6W`\0\x80\xFD[\x82\x01a\x02`\x81\x85\x03\x12\x15a\x08HW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81Ra\x01\xE0\x81\x01` \x83\x01Qa\x18\xF5` \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x83\x01Qa\x19\x10`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Qa\x19+``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x83\x01Qa\x19F`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa\x19a`\xA0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x83\x01Qa\x19|`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x83\x01Qa\x19\x97`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91Ra\x01 \x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01@\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01`\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\x80\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xA0\x80\x85\x01Q\x82\x16\x90\x84\x01Ra\x01\xC0\x80\x85\x01Q\x91\x82\x16\x81\x85\x01R\x90a\x150V[`\0`\xA0\x82\x84\x03\x12\x15a\x1A\x18W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1A;Wa\x1A;a\x11)V[`@R\x90P\x80\x825a\x1AL\x81a\x12\x1EV[\x80\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015a\x1A|\x81a\x12\x1EV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a\x1A\x9FW`\0\x80\xFD[a\x1A\xA9\x85\x85a\x1A\x06V[\x92P`\xA0\x84\x015a\x1A\xB9\x81a\x12\xBFV[\x91Pa\x13X\x85`\xC0\x86\x01a\x12\xD8V[`\0a\x1A\xD6a\x160\x84a\x15\xC5V[\x83\x81R\x90P` \x80\x82\x01\x90``\x80\x86\x02\x85\x01\x87\x81\x11\x15a\x1A\xF5W`\0\x80\xFD[\x85[\x81\x81\x10\x15a\x1BaW\x82\x81\x8A\x03\x12\x15a\x1B\x0FW`\0\x80\x81\xFD[`@\x80Q\x84\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B2Wa\x1B2a\x11)V[\x82R\x825a\x1B?\x81a\x12\x1EV[\x81R\x82\x86\x015\x86\x82\x01R\x81\x83\x015\x91\x81\x01\x91\x90\x91R\x85R\x93\x83\x01\x93\x82\x01a\x1A\xF7V[PPPPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1B\x7FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x96W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1B\xA7W`\0\x80\xFD[a\x05I\x84\x825` \x84\x01a\x1A\xC8V[`\0` \x82\x84\x03\x12\x15a\x1B\xC8W`\0\x80\xFD[\x815a\x08H\x81a\x12\x1EV[`\0\x80`\xC0\x83\x85\x03\x12\x15a\x1B\xE6W`\0\x80\xFD[a\x1B\xF0\x84\x84a\x1A\x06V[\x91P`\xA0\x83\x015a\x15~\x81a\x12\xBFV[`\xE0\x81\x01a\x1C8\x82\x86\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x90\x84\x01R`@\x80\x83\x01Q\x90\x84\x01R``\x91\x82\x01Q\x16\x91\x01RV[\x83\x15\x15`\x80\x83\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x84\x01R` \x84\x01Q\x16`\xC0\x83\x01Ra\x05IV[`\0` \x82\x84\x03\x12\x15a\x1CtW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x8BW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1C\x9CW`\0\x80\xFD[\x80Qa\x1C\xAAa\x160\x82a\x15\xE9V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x1C\xBFW`\0\x80\xFD[a\x1C\xD0\x82` \x83\x01` \x86\x01a\x13aV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xEBW`\0\x80\xFD[PQ\x91\x90PV[\x80Qa\x12A\x81a\x12\x1EV[`\0a\x02@\x82\x84\x03\x12\x15a\x1D\x10W`\0\x80\xFD[a\x1D\x18a\x11\xA5V[a\x1D!\x83a\x1C\xF2V[\x81Ra\x1D/` \x84\x01a\x1C\xF2V[` \x82\x01Ra\x1D@`@\x84\x01a\x1C\xF2V[`@\x82\x01Ra\x1DQ``\x84\x01a\x1C\xF2V[``\x82\x01Ra\x1Db`\x80\x84\x01a\x1C\xF2V[`\x80\x82\x01Ra\x1Ds`\xA0\x84\x01a\x1C\xF2V[`\xA0\x82\x01Ra\x1D\x84`\xC0\x84\x01a\x1C\xF2V[`\xC0\x82\x01Ra\x1D\x95`\xE0\x84\x01a\x1C\xF2V[`\xE0\x82\x01Ra\x01\0a\x1D\xA8\x81\x85\x01a\x1C\xF2V[\x90\x82\x01Ra\x01 a\x1D\xBA\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01@a\x1D\xCC\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01`a\x1D\xDE\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\x80a\x1D\xF0\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xA0a\x1E\x02\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xC0a\x1E\x14\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xE0a\x1E&\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x02\0a\x1E8\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x02 a\x1EJ\x84\x82\x01a\x1C\xF2V[\x90\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1EgW`\0\x80\xFD[\x81Qa\x08H\x81a\x12\x1EV[`\xA0\x81\x01a\x1E\xAA\x82\x85\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x90\x84\x01R`@\x80\x83\x01Q\x90\x84\x01R``\x91\x82\x01Q\x16\x91\x01RV[\x82\x15\x15`\x80\x83\x01R\x93\x92PPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x1E\xEEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x0EW`\0\x80\xFD[``\x81\x026\x03\x82\x13\x15a\x1F W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\x1FyW\x815a\x1FJ\x81a\x12\x1EV[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015\x83\x88\x01R`@\x80\x83\x015\x90\x88\x01R``\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a\x1F7V[P\x94\x95\x94PPPPPV[` \x81R`\0\x825a\x1F\x95\x81a\x12\xBFV[\x80\x15\x15` \x84\x01RP` \x83\x015`@\x83\x01R`@\x83\x015``\x83\x01R``\x83\x015a\x1F\xC0\x81a\x12\x1EV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\x80\x85\x01R`\x80\x85\x015\x91Pa\x1F\xE0\x82a\x12\x1EV[\x80\x82\x16`\xA0\x85\x01RPP`\xA0\x83\x015`\xC0\x83\x01R`\xC0\x83\x015`\xE0\x83\x01Ra\x01\0`\xE0\x84\x015\x81\x84\x01R\x80\x84\x015\x90Pa \x19\x81a\x16{V[a\x01 g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x85\x01Ra 6\x81\x86\x01a\x16\x91V[\x91PPa\x01@a Q\x81\x85\x01\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a ]\x81\x86\x01\x86a\x1E\xB9V[a\x01`\x86\x81\x01R\x92P\x90Pa\x1C\xD0a\x01\x80\x85\x01\x83\x83a\x1F'V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a!\x9EW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x89\x84\x03\x01\x85R\x81Qa\x01\x80\x81Q\x81\x86Ra \xE3\x82\x87\x01\x82a\x13\x91V[\x91PP\x88\x82\x01Qa \xFE\x8A\x87\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P\x87\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86\x8A\x01R``\x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x87\x01R`\x80\x80\x84\x01Q\x90\x87\x01R`\xA0\x80\x84\x01Q\x90\x87\x01R`\xC0\x80\x84\x01Q\x90\x87\x01R`\xE0\x80\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x88\x01\x91\x90\x91Ra\x01\0\x80\x85\x01Q\x90\x91\x16\x90\x87\x01Ra\x01 \x80\x84\x01Q\x90\x87\x01Ra\x01@\x80\x84\x01Q\x90\x91\x16\x90\x86\x01Ra\x01`\x91\x82\x01Q\x15\x15\x91\x90\x94\x01R\x93\x86\x01\x93\x90\x86\x01\x90`\x01\x01a \x9EV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a!\xBFW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xD6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a!\xE7W`\0\x80\xFD[\x80Qa!\xF5a\x160\x82a\x15\xC5V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\"\x14W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"jW`@\x84\x89\x03\x12\x15a\"2W`\0\x80\x81\xFD[a\":a\x11XV[\x84Qa\"E\x81a\x12\x1EV[\x81R\x84\x86\x01Qa\"T\x81a\x12\x1EV[\x81\x87\x01R\x82R`@\x93\x90\x93\x01\x92\x90\x84\x01\x90a\"\x19V[\x97\x96PPPPPPPV[\x805a\"\x80\x81a\x12\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x82\x015\x90a\"\x9C\x82a\x12\x1EV[\x90\x81\x16` \x84\x01R`@\x82\x015\x90a\"\xB3\x82a\x12\x1EV[\x90\x81\x16`@\x84\x01R``\x82\x015\x90a\"\xCA\x82a\x12\x1EV[\x90\x81\x16``\x84\x01R`\x80\x82\x015\x90a\"\xE1\x82a\x12\x1EV[\x90\x81\x16`\x80\x84\x01R`\xA0\x82\x015\x90a\"\xF8\x82a\x12\x1EV[\x80\x82\x16`\xA0\x85\x01RPPPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#;W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#[W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x1F W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81Ra#\xA6``\x82\x01\x85a\"uV[`\0a#\xB4`\xC0\x86\x01a\x16gV[a\x01 a#\xC8\x81\x85\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[a#\xD4`\xE0\x88\x01a\x16gV[\x91Pa\x01@a#\xEA\x81\x86\x01\x84c\xFF\xFF\xFF\xFF\x16\x90RV[a\x01`\x92Pa\x01\0\x88\x015\x83\x86\x01Ra$\x05\x82\x89\x01\x89a#\x06V[\x92Pa\x02`a\x01\x80\x81\x81\x89\x01Ra$!a\x02\xC0\x89\x01\x86\x85a#jV[\x94Pa$/\x84\x8C\x01\x8Ca#\x06V[\x94P\x92Pa\x01\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x87\x03\x01\x81\x8A\x01Ra$j\x86\x86\x86a#jV[\x95Pa$w\x87\x8D\x01a\x16\x91V[\x96Pa\x01\xC0\x94Pa$\x93\x85\x8A\x01\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a$\x9E\x82\x8D\x01a\x16gV[\x96Pa\x01\xE0\x93Pa$\xB6\x84\x8A\x01\x88c\xFF\xFF\xFF\xFF\x16\x90RV[a\x02\0\x96P\x80\x8C\x015\x87\x8A\x01RPPa\x02 \x83\x8B\x015\x81\x89\x01Ra\x02@\x93P\x82\x8B\x015\x84\x89\x01Ra$\xE8\x86\x8C\x01a\x16\x91V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x89\x84\x01R\x95Pa%\x04\x81\x8C\x01a\x16\x91V[\x95PPPPa% a\x02\x80\x86\x01\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a%+\x81\x89\x01a\x12\xCDV[\x92PPa%=a\x02\xA0\x85\x01\x83\x15\x15\x90RV[`\x01`\x01`\xA0\x1B\x03\x86\x16` \x85\x01R\x91Pa%U\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`@\x83\x01Ra\x05IV[`\0a\x01\xE0\x82\x84\x03\x12\x15a%|W`\0\x80\xFD[a%\x84a\x11\xC9V[a%\x8D\x83a\x1C\xF2V[\x81Ra%\x9B` \x84\x01a\x1C\xF2V[` \x82\x01Ra%\xAC`@\x84\x01a\x1C\xF2V[`@\x82\x01Ra%\xBD``\x84\x01a\x1C\xF2V[``\x82\x01Ra%\xCE`\x80\x84\x01a\x1C\xF2V[`\x80\x82\x01Ra%\xDF`\xA0\x84\x01a\x1C\xF2V[`\xA0\x82\x01Ra%\xF0`\xC0\x84\x01a\x1C\xF2V[`\xC0\x82\x01Ra&\x01`\xE0\x84\x01a\x1C\xF2V[`\xE0\x82\x01Ra\x01\0a&\x14\x81\x85\x01a\x1C\xF2V[\x90\x82\x01Ra\x01 a&&\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01@a&8\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01`a&J\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\x80a&\\\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xA0a&n\x84\x82\x01a\x1C\xF2V[\x90\x82\x01Ra\x01\xC0a\x1EJ\x84\x82\x01a\x1C\xF2V[`\0` \x82\x84\x03\x12\x15a&\x92W`\0\x80\xFD[\x81Qa\x08H\x81a\x12\xBFV[a\x01\0\x81\x01a&\xE4\x82\x86`\x01`\x01`\xA0\x1B\x03\x80\x82Q\x16\x83R` \x82\x01Q` \x84\x01R`@\x82\x01Q`@\x84\x01R``\x82\x01Q``\x84\x01R\x80`\x80\x83\x01Q\x16`\x80\x84\x01RPPPV[\x83\x15\x15`\xA0\x83\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x84\x01R` \x84\x01Q\x16`\xE0\x83\x01Ra\x05IV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a'mWa'Z\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a'*V[P\x90\x96\x95PPPPPPV[`\0`\xA0\x82\x84\x03\x12\x15a'\x8BW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a'\xAEWa'\xAEa\x11)V[`@R\x82Qa'\xBC\x81a\x12\x1EV[\x81R` \x83\x01Qa'\xCC\x81a\x12\x1EV[` \x82\x01R`@\x83\x01Qa'\xDF\x81a\x12\x1EV[`@\x82\x01R``\x83\x01Qa'\xF2\x81a\x12\x1EV[``\x82\x01R`\x80\x83\x01Qa(\x05\x81a\x12\x1EV[`\x80\x82\x01R\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a'mWa(]\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a(-V[`\xC0\x81\x01a(\xB6\x82\x85`\x01`\x01`\xA0\x1B\x03\x80\x82Q\x16\x83R` \x82\x01Q` \x84\x01R`@\x82\x01Q`@\x84\x01R``\x82\x01Q``\x84\x01R\x80`\x80\x83\x01Q\x16`\x80\x84\x01RPPPV[\x82\x15\x15`\xA0\x83\x01R\x93\x92PPPV[`\0\x82Qa(\xD7\x81\x84` \x87\x01a\x13aV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
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
    /**```solidity
struct AddGameInput { string saltMixer; address systemConfig; address delayedWETH; GameType disputeGameType; Claim disputeAbsolutePrestate; uint256 disputeMaxGameDepth; uint256 disputeSplitDepth; Duration disputeClockExtension; Duration disputeMaxClockDuration; uint256 initialBond; address vm; bool permissioned; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddGameInput {
        #[allow(missing_docs)]
        pub saltMixer: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub systemConfig: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayedWETH: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub disputeGameType: <GameType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub disputeAbsolutePrestate: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub disputeMaxGameDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub disputeSplitDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub disputeClockExtension: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub disputeMaxClockDuration: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub initialBond: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub vm: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permissioned: bool,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            GameType,
            Claim,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            Duration,
            Duration,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            <GameType as alloy::sol_types::SolType>::RustType,
            <Claim as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <Duration as alloy::sol_types::SolType>::RustType,
            <Duration as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AddGameInput> for UnderlyingRustTuple<'_> {
            fn from(value: AddGameInput) -> Self {
                (
                    value.saltMixer,
                    value.systemConfig,
                    value.delayedWETH,
                    value.disputeGameType,
                    value.disputeAbsolutePrestate,
                    value.disputeMaxGameDepth,
                    value.disputeSplitDepth,
                    value.disputeClockExtension,
                    value.disputeMaxClockDuration,
                    value.initialBond,
                    value.vm,
                    value.permissioned,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddGameInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    saltMixer: tuple.0,
                    systemConfig: tuple.1,
                    delayedWETH: tuple.2,
                    disputeGameType: tuple.3,
                    disputeAbsolutePrestate: tuple.4,
                    disputeMaxGameDepth: tuple.5,
                    disputeSplitDepth: tuple.6,
                    disputeClockExtension: tuple.7,
                    disputeMaxClockDuration: tuple.8,
                    initialBond: tuple.9,
                    vm: tuple.10,
                    permissioned: tuple.11,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AddGameInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AddGameInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.saltMixer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.systemConfig,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWETH,
                    ),
                    <GameType as alloy_sol_types::SolType>::tokenize(
                        &self.disputeGameType,
                    ),
                    <Claim as alloy_sol_types::SolType>::tokenize(
                        &self.disputeAbsolutePrestate,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.disputeMaxGameDepth),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.disputeSplitDepth),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.disputeClockExtension,
                    ),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.disputeMaxClockDuration,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialBond),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.vm,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.permissioned,
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
        impl alloy_sol_types::SolType for AddGameInput {
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
        impl alloy_sol_types::SolStruct for AddGameInput {
            const NAME: &'static str = "AddGameInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AddGameInput(string saltMixer,address systemConfig,address delayedWETH,uint32 disputeGameType,bytes32 disputeAbsolutePrestate,uint256 disputeMaxGameDepth,uint256 disputeSplitDepth,uint64 disputeClockExtension,uint64 disputeMaxClockDuration,uint256 initialBond,address vm,bool permissioned)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.saltMixer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.systemConfig,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedWETH,
                        )
                        .0,
                    <GameType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeGameType,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeAbsolutePrestate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeMaxGameDepth,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeSplitDepth,
                        )
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeClockExtension,
                        )
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeMaxClockDuration,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.initialBond)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.vm,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.permissioned,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AddGameInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.saltMixer,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.systemConfig,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWETH,
                    )
                    + <GameType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeGameType,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeAbsolutePrestate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeMaxGameDepth,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeSplitDepth,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeClockExtension,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeMaxClockDuration,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.initialBond,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.vm,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.permissioned,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.saltMixer,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.systemConfig,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedWETH,
                    out,
                );
                <GameType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeGameType,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeAbsolutePrestate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeMaxGameDepth,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeSplitDepth,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeClockExtension,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeMaxClockDuration,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.initialBond,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.vm,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.permissioned,
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
    /**```solidity
struct AddGameOutput { address delayedWETH; address faultDisputeGame; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddGameOutput {
        #[allow(missing_docs)]
        pub delayedWETH: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub faultDisputeGame: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AddGameOutput> for UnderlyingRustTuple<'_> {
            fn from(value: AddGameOutput) -> Self {
                (value.delayedWETH, value.faultDisputeGame)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddGameOutput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delayedWETH: tuple.0,
                    faultDisputeGame: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AddGameOutput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AddGameOutput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWETH,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.faultDisputeGame,
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
        impl alloy_sol_types::SolType for AddGameOutput {
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
        impl alloy_sol_types::SolStruct for AddGameOutput {
            const NAME: &'static str = "AddGameOutput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AddGameOutput(address delayedWETH,address faultDisputeGame)",
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
                            &self.delayedWETH,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.faultDisputeGame,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AddGameOutput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWETH,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.faultDisputeGame,
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
                    &rust.delayedWETH,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.faultDisputeGame,
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
    /**```solidity
struct Blueprints { address addressManager; address proxy; address proxyAdmin; address l1ChugSplashProxy; address resolvedDelegateProxy; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Blueprints {
        #[allow(missing_docs)]
        pub addressManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub proxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub proxyAdmin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1ChugSplashProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub resolvedDelegateProxy: alloy::sol_types::private::Address,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<Blueprints> for UnderlyingRustTuple<'_> {
            fn from(value: Blueprints) -> Self {
                (
                    value.addressManager,
                    value.proxy,
                    value.proxyAdmin,
                    value.l1ChugSplashProxy,
                    value.resolvedDelegateProxy,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Blueprints {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addressManager: tuple.0,
                    proxy: tuple.1,
                    proxyAdmin: tuple.2,
                    l1ChugSplashProxy: tuple.3,
                    resolvedDelegateProxy: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Blueprints {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Blueprints {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addressManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proxyAdmin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1ChugSplashProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.resolvedDelegateProxy,
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
        impl alloy_sol_types::SolType for Blueprints {
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
        impl alloy_sol_types::SolStruct for Blueprints {
            const NAME: &'static str = "Blueprints";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Blueprints(address addressManager,address proxy,address proxyAdmin,address l1ChugSplashProxy,address resolvedDelegateProxy)",
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
                            &self.addressManager,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proxyAdmin,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1ChugSplashProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.resolvedDelegateProxy,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Blueprints {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addressManager,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proxyAdmin,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1ChugSplashProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.resolvedDelegateProxy,
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
                    &rust.addressManager,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proxyAdmin,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1ChugSplashProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.resolvedDelegateProxy,
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
    /**```solidity
struct DeployInput { Roles roles; uint32 basefeeScalar; uint32 blobBasefeeScalar; uint256 l2ChainId; bytes startingAnchorRoot; string saltMixer; uint64 gasLimit; GameType disputeGameType; Claim disputeAbsolutePrestate; uint256 disputeMaxGameDepth; uint256 disputeSplitDepth; Duration disputeClockExtension; Duration disputeMaxClockDuration; bool useCustomGasToken; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeployInput {
        #[allow(missing_docs)]
        pub roles: <Roles as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub basefeeScalar: u32,
        #[allow(missing_docs)]
        pub blobBasefeeScalar: u32,
        #[allow(missing_docs)]
        pub l2ChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startingAnchorRoot: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub saltMixer: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub gasLimit: u64,
        #[allow(missing_docs)]
        pub disputeGameType: <GameType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub disputeAbsolutePrestate: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub disputeMaxGameDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub disputeSplitDepth: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub disputeClockExtension: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub disputeMaxClockDuration: <Duration as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub useCustomGasToken: bool,
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
            Roles,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Uint<64>,
            GameType,
            Claim,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            Duration,
            Duration,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Roles as alloy::sol_types::SolType>::RustType,
            u32,
            u32,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::String,
            u64,
            <GameType as alloy::sol_types::SolType>::RustType,
            <Claim as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <Duration as alloy::sol_types::SolType>::RustType,
            <Duration as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<DeployInput> for UnderlyingRustTuple<'_> {
            fn from(value: DeployInput) -> Self {
                (
                    value.roles,
                    value.basefeeScalar,
                    value.blobBasefeeScalar,
                    value.l2ChainId,
                    value.startingAnchorRoot,
                    value.saltMixer,
                    value.gasLimit,
                    value.disputeGameType,
                    value.disputeAbsolutePrestate,
                    value.disputeMaxGameDepth,
                    value.disputeSplitDepth,
                    value.disputeClockExtension,
                    value.disputeMaxClockDuration,
                    value.useCustomGasToken,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DeployInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    roles: tuple.0,
                    basefeeScalar: tuple.1,
                    blobBasefeeScalar: tuple.2,
                    l2ChainId: tuple.3,
                    startingAnchorRoot: tuple.4,
                    saltMixer: tuple.5,
                    gasLimit: tuple.6,
                    disputeGameType: tuple.7,
                    disputeAbsolutePrestate: tuple.8,
                    disputeMaxGameDepth: tuple.9,
                    disputeSplitDepth: tuple.10,
                    disputeClockExtension: tuple.11,
                    disputeMaxClockDuration: tuple.12,
                    useCustomGasToken: tuple.13,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DeployInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DeployInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Roles as alloy_sol_types::SolType>::tokenize(&self.roles),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.basefeeScalar),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blobBasefeeScalar),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2ChainId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.startingAnchorRoot,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.saltMixer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasLimit),
                    <GameType as alloy_sol_types::SolType>::tokenize(
                        &self.disputeGameType,
                    ),
                    <Claim as alloy_sol_types::SolType>::tokenize(
                        &self.disputeAbsolutePrestate,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.disputeMaxGameDepth),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.disputeSplitDepth),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.disputeClockExtension,
                    ),
                    <Duration as alloy_sol_types::SolType>::tokenize(
                        &self.disputeMaxClockDuration,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.useCustomGasToken,
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
        impl alloy_sol_types::SolType for DeployInput {
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
        impl alloy_sol_types::SolStruct for DeployInput {
            const NAME: &'static str = "DeployInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DeployInput(Roles roles,uint32 basefeeScalar,uint32 blobBasefeeScalar,uint256 l2ChainId,bytes startingAnchorRoot,string saltMixer,uint64 gasLimit,uint32 disputeGameType,bytes32 disputeAbsolutePrestate,uint256 disputeMaxGameDepth,uint256 disputeSplitDepth,uint64 disputeClockExtension,uint64 disputeMaxClockDuration,bool useCustomGasToken)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Roles as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Roles as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Roles as alloy_sol_types::SolType>::eip712_data_word(&self.roles).0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.basefeeScalar)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.blobBasefeeScalar,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l2ChainId)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startingAnchorRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.saltMixer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gasLimit)
                        .0,
                    <GameType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeGameType,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeAbsolutePrestate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeMaxGameDepth,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeSplitDepth,
                        )
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeClockExtension,
                        )
                        .0,
                    <Duration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeMaxClockDuration,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.useCustomGasToken,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DeployInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Roles as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.roles,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.basefeeScalar,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blobBasefeeScalar,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2ChainId,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startingAnchorRoot,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.saltMixer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gasLimit,
                    )
                    + <GameType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeGameType,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeAbsolutePrestate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeMaxGameDepth,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeSplitDepth,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeClockExtension,
                    )
                    + <Duration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeMaxClockDuration,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.useCustomGasToken,
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
                <Roles as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.roles,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.basefeeScalar,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blobBasefeeScalar,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2ChainId,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startingAnchorRoot,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.saltMixer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gasLimit,
                    out,
                );
                <GameType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeGameType,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeAbsolutePrestate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeMaxGameDepth,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeSplitDepth,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeClockExtension,
                    out,
                );
                <Duration as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeMaxClockDuration,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.useCustomGasToken,
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
    /**```solidity
struct DeployOutput { address opChainProxyAdmin; address addressManager; address l1ERC721BridgeProxy; address systemConfigProxy; address optimismMintableERC20FactoryProxy; address l1StandardBridgeProxy; address l1CrossDomainMessengerProxy; address ethLockboxProxy; address optimismPortalProxy; address disputeGameFactoryProxy; address anchorStateRegistryProxy; address faultDisputeGame; address permissionedDisputeGame; address delayedWETHPermissionedGameProxy; address delayedWETHPermissionlessGameProxy; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeployOutput {
        #[allow(missing_docs)]
        pub opChainProxyAdmin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub addressManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1ERC721BridgeProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub systemConfigProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub optimismMintableERC20FactoryProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1StandardBridgeProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1CrossDomainMessengerProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ethLockboxProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub optimismPortalProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub disputeGameFactoryProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub anchorStateRegistryProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub faultDisputeGame: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permissionedDisputeGame: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayedWETHPermissionedGameProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayedWETHPermissionlessGameProxy: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
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
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<DeployOutput> for UnderlyingRustTuple<'_> {
            fn from(value: DeployOutput) -> Self {
                (
                    value.opChainProxyAdmin,
                    value.addressManager,
                    value.l1ERC721BridgeProxy,
                    value.systemConfigProxy,
                    value.optimismMintableERC20FactoryProxy,
                    value.l1StandardBridgeProxy,
                    value.l1CrossDomainMessengerProxy,
                    value.ethLockboxProxy,
                    value.optimismPortalProxy,
                    value.disputeGameFactoryProxy,
                    value.anchorStateRegistryProxy,
                    value.faultDisputeGame,
                    value.permissionedDisputeGame,
                    value.delayedWETHPermissionedGameProxy,
                    value.delayedWETHPermissionlessGameProxy,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DeployOutput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    opChainProxyAdmin: tuple.0,
                    addressManager: tuple.1,
                    l1ERC721BridgeProxy: tuple.2,
                    systemConfigProxy: tuple.3,
                    optimismMintableERC20FactoryProxy: tuple.4,
                    l1StandardBridgeProxy: tuple.5,
                    l1CrossDomainMessengerProxy: tuple.6,
                    ethLockboxProxy: tuple.7,
                    optimismPortalProxy: tuple.8,
                    disputeGameFactoryProxy: tuple.9,
                    anchorStateRegistryProxy: tuple.10,
                    faultDisputeGame: tuple.11,
                    permissionedDisputeGame: tuple.12,
                    delayedWETHPermissionedGameProxy: tuple.13,
                    delayedWETHPermissionlessGameProxy: tuple.14,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DeployOutput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DeployOutput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.opChainProxyAdmin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addressManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1ERC721BridgeProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.systemConfigProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.optimismMintableERC20FactoryProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1StandardBridgeProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1CrossDomainMessengerProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ethLockboxProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.optimismPortalProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.disputeGameFactoryProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.anchorStateRegistryProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.faultDisputeGame,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.permissionedDisputeGame,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWETHPermissionedGameProxy,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWETHPermissionlessGameProxy,
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
        impl alloy_sol_types::SolType for DeployOutput {
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
        impl alloy_sol_types::SolStruct for DeployOutput {
            const NAME: &'static str = "DeployOutput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DeployOutput(address opChainProxyAdmin,address addressManager,address l1ERC721BridgeProxy,address systemConfigProxy,address optimismMintableERC20FactoryProxy,address l1StandardBridgeProxy,address l1CrossDomainMessengerProxy,address ethLockboxProxy,address optimismPortalProxy,address disputeGameFactoryProxy,address anchorStateRegistryProxy,address faultDisputeGame,address permissionedDisputeGame,address delayedWETHPermissionedGameProxy,address delayedWETHPermissionlessGameProxy)",
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
                            &self.opChainProxyAdmin,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addressManager,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1ERC721BridgeProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.systemConfigProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.optimismMintableERC20FactoryProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1StandardBridgeProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1CrossDomainMessengerProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ethLockboxProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.optimismPortalProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeGameFactoryProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.anchorStateRegistryProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.faultDisputeGame,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.permissionedDisputeGame,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedWETHPermissionedGameProxy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedWETHPermissionlessGameProxy,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DeployOutput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.opChainProxyAdmin,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addressManager,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1ERC721BridgeProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.systemConfigProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimismMintableERC20FactoryProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1StandardBridgeProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1CrossDomainMessengerProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ethLockboxProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimismPortalProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeGameFactoryProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.anchorStateRegistryProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.faultDisputeGame,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.permissionedDisputeGame,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWETHPermissionedGameProxy,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWETHPermissionlessGameProxy,
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
                    &rust.opChainProxyAdmin,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addressManager,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1ERC721BridgeProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.systemConfigProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.optimismMintableERC20FactoryProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1StandardBridgeProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1CrossDomainMessengerProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ethLockboxProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.optimismPortalProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeGameFactoryProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.anchorStateRegistryProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.faultDisputeGame,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.permissionedDisputeGame,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedWETHPermissionedGameProxy,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedWETHPermissionlessGameProxy,
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
    /**```solidity
struct Implementations { address superchainConfigImpl; address protocolVersionsImpl; address l1ERC721BridgeImpl; address optimismPortalImpl; address optimismPortalInteropImpl; address ethLockboxImpl; address systemConfigImpl; address optimismMintableERC20FactoryImpl; address l1CrossDomainMessengerImpl; address l1StandardBridgeImpl; address disputeGameFactoryImpl; address anchorStateRegistryImpl; address delayedWETHImpl; address mipsImpl; address faultDisputeGameV2Impl; address permissionedDisputeGameV2Impl; address superFaultDisputeGameImpl; address superPermissionedDisputeGameImpl; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Implementations {
        #[allow(missing_docs)]
        pub superchainConfigImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub protocolVersionsImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1ERC721BridgeImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub optimismPortalImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub optimismPortalInteropImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ethLockboxImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub systemConfigImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub optimismMintableERC20FactoryImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1CrossDomainMessengerImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub l1StandardBridgeImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub disputeGameFactoryImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub anchorStateRegistryImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delayedWETHImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub mipsImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub faultDisputeGameV2Impl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permissionedDisputeGameV2Impl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub superFaultDisputeGameImpl: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub superPermissionedDisputeGameImpl: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
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
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<Implementations> for UnderlyingRustTuple<'_> {
            fn from(value: Implementations) -> Self {
                (
                    value.superchainConfigImpl,
                    value.protocolVersionsImpl,
                    value.l1ERC721BridgeImpl,
                    value.optimismPortalImpl,
                    value.optimismPortalInteropImpl,
                    value.ethLockboxImpl,
                    value.systemConfigImpl,
                    value.optimismMintableERC20FactoryImpl,
                    value.l1CrossDomainMessengerImpl,
                    value.l1StandardBridgeImpl,
                    value.disputeGameFactoryImpl,
                    value.anchorStateRegistryImpl,
                    value.delayedWETHImpl,
                    value.mipsImpl,
                    value.faultDisputeGameV2Impl,
                    value.permissionedDisputeGameV2Impl,
                    value.superFaultDisputeGameImpl,
                    value.superPermissionedDisputeGameImpl,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Implementations {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    superchainConfigImpl: tuple.0,
                    protocolVersionsImpl: tuple.1,
                    l1ERC721BridgeImpl: tuple.2,
                    optimismPortalImpl: tuple.3,
                    optimismPortalInteropImpl: tuple.4,
                    ethLockboxImpl: tuple.5,
                    systemConfigImpl: tuple.6,
                    optimismMintableERC20FactoryImpl: tuple.7,
                    l1CrossDomainMessengerImpl: tuple.8,
                    l1StandardBridgeImpl: tuple.9,
                    disputeGameFactoryImpl: tuple.10,
                    anchorStateRegistryImpl: tuple.11,
                    delayedWETHImpl: tuple.12,
                    mipsImpl: tuple.13,
                    faultDisputeGameV2Impl: tuple.14,
                    permissionedDisputeGameV2Impl: tuple.15,
                    superFaultDisputeGameImpl: tuple.16,
                    superPermissionedDisputeGameImpl: tuple.17,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Implementations {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Implementations {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.superchainConfigImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.protocolVersionsImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1ERC721BridgeImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.optimismPortalImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.optimismPortalInteropImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ethLockboxImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.systemConfigImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.optimismMintableERC20FactoryImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1CrossDomainMessengerImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.l1StandardBridgeImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.disputeGameFactoryImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.anchorStateRegistryImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWETHImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.mipsImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.faultDisputeGameV2Impl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.permissionedDisputeGameV2Impl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.superFaultDisputeGameImpl,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.superPermissionedDisputeGameImpl,
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
        impl alloy_sol_types::SolType for Implementations {
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
        impl alloy_sol_types::SolStruct for Implementations {
            const NAME: &'static str = "Implementations";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Implementations(address superchainConfigImpl,address protocolVersionsImpl,address l1ERC721BridgeImpl,address optimismPortalImpl,address optimismPortalInteropImpl,address ethLockboxImpl,address systemConfigImpl,address optimismMintableERC20FactoryImpl,address l1CrossDomainMessengerImpl,address l1StandardBridgeImpl,address disputeGameFactoryImpl,address anchorStateRegistryImpl,address delayedWETHImpl,address mipsImpl,address faultDisputeGameV2Impl,address permissionedDisputeGameV2Impl,address superFaultDisputeGameImpl,address superPermissionedDisputeGameImpl)",
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
                            &self.superchainConfigImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.protocolVersionsImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1ERC721BridgeImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.optimismPortalImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.optimismPortalInteropImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ethLockboxImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.systemConfigImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.optimismMintableERC20FactoryImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1CrossDomainMessengerImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1StandardBridgeImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disputeGameFactoryImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.anchorStateRegistryImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedWETHImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.mipsImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.faultDisputeGameV2Impl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.permissionedDisputeGameV2Impl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.superFaultDisputeGameImpl,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.superPermissionedDisputeGameImpl,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Implementations {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.superchainConfigImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.protocolVersionsImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1ERC721BridgeImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimismPortalImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimismPortalInteropImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ethLockboxImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.systemConfigImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimismMintableERC20FactoryImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1CrossDomainMessengerImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1StandardBridgeImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disputeGameFactoryImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.anchorStateRegistryImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWETHImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.mipsImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.faultDisputeGameV2Impl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.permissionedDisputeGameV2Impl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.superFaultDisputeGameImpl,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.superPermissionedDisputeGameImpl,
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
                    &rust.superchainConfigImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.protocolVersionsImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1ERC721BridgeImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.optimismPortalImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.optimismPortalInteropImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ethLockboxImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.systemConfigImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.optimismMintableERC20FactoryImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1CrossDomainMessengerImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1StandardBridgeImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disputeGameFactoryImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.anchorStateRegistryImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedWETHImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.mipsImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.faultDisputeGameV2Impl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.permissionedDisputeGameV2Impl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.superFaultDisputeGameImpl,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.superPermissionedDisputeGameImpl,
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
    /**```solidity
struct OpChainConfig { address systemConfigProxy; Claim cannonPrestate; Claim cannonKonaPrestate; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OpChainConfig {
        #[allow(missing_docs)]
        pub systemConfigProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub cannonPrestate: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cannonKonaPrestate: <Claim as alloy::sol_types::SolType>::RustType,
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
            Claim,
            Claim,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            <Claim as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<OpChainConfig> for UnderlyingRustTuple<'_> {
            fn from(value: OpChainConfig) -> Self {
                (value.systemConfigProxy, value.cannonPrestate, value.cannonKonaPrestate)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OpChainConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    systemConfigProxy: tuple.0,
                    cannonPrestate: tuple.1,
                    cannonKonaPrestate: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OpChainConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OpChainConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.systemConfigProxy,
                    ),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self.cannonPrestate),
                    <Claim as alloy_sol_types::SolType>::tokenize(
                        &self.cannonKonaPrestate,
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
        impl alloy_sol_types::SolType for OpChainConfig {
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
        impl alloy_sol_types::SolStruct for OpChainConfig {
            const NAME: &'static str = "OpChainConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OpChainConfig(address systemConfigProxy,bytes32 cannonPrestate,bytes32 cannonKonaPrestate)",
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
                            &self.systemConfigProxy,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cannonPrestate,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cannonKonaPrestate,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OpChainConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.systemConfigProxy,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cannonPrestate,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cannonKonaPrestate,
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
                    &rust.systemConfigProxy,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cannonPrestate,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cannonKonaPrestate,
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
    /**```solidity
struct Proposal { Hash root; uint256 l2SequenceNumber; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Proposal {
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
        impl ::core::convert::From<Proposal> for UnderlyingRustTuple<'_> {
            fn from(value: Proposal) -> Self {
                (value.root, value.l2SequenceNumber)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Proposal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    root: tuple.0,
                    l2SequenceNumber: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Proposal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Proposal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Hash as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.l2SequenceNumber),
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
        impl alloy_sol_types::SolType for Proposal {
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
        impl alloy_sol_types::SolStruct for Proposal {
            const NAME: &'static str = "Proposal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Proposal(bytes32 root,uint256 l2SequenceNumber)",
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
                    <Hash as alloy_sol_types::SolType>::eip712_data_word(&self.root).0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l2SequenceNumber,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Proposal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Hash as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.root,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l2SequenceNumber,
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
                <Hash as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.root,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l2SequenceNumber,
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
    /**```solidity
struct Roles { address opChainProxyAdminOwner; address systemConfigOwner; address batcher; address unsafeBlockSigner; address proposer; address challenger; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Roles {
        #[allow(missing_docs)]
        pub opChainProxyAdminOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub systemConfigOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub batcher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub unsafeBlockSigner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<Roles> for UnderlyingRustTuple<'_> {
            fn from(value: Roles) -> Self {
                (
                    value.opChainProxyAdminOwner,
                    value.systemConfigOwner,
                    value.batcher,
                    value.unsafeBlockSigner,
                    value.proposer,
                    value.challenger,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Roles {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    opChainProxyAdminOwner: tuple.0,
                    systemConfigOwner: tuple.1,
                    batcher: tuple.2,
                    unsafeBlockSigner: tuple.3,
                    proposer: tuple.4,
                    challenger: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Roles {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Roles {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.opChainProxyAdminOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.systemConfigOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.batcher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.unsafeBlockSigner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challenger,
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
        impl alloy_sol_types::SolType for Roles {
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
        impl alloy_sol_types::SolStruct for Roles {
            const NAME: &'static str = "Roles";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Roles(address opChainProxyAdminOwner,address systemConfigOwner,address batcher,address unsafeBlockSigner,address proposer,address challenger)",
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
                            &self.opChainProxyAdminOwner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.systemConfigOwner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.batcher,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.unsafeBlockSigner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.challenger,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Roles {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.opChainProxyAdminOwner,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.systemConfigOwner,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.batcher,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.unsafeBlockSigner,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposer,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.challenger,
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
                    &rust.opChainProxyAdminOwner,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.systemConfigOwner,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.batcher,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.unsafeBlockSigner,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposer,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.challenger,
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
    /**```solidity
struct UpdatePrestateInput { address systemConfigProxy; Claim cannonPrestate; Claim cannonKonaPrestate; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UpdatePrestateInput {
        #[allow(missing_docs)]
        pub systemConfigProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub cannonPrestate: <Claim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cannonKonaPrestate: <Claim as alloy::sol_types::SolType>::RustType,
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
            Claim,
            Claim,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            <Claim as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<UpdatePrestateInput> for UnderlyingRustTuple<'_> {
            fn from(value: UpdatePrestateInput) -> Self {
                (value.systemConfigProxy, value.cannonPrestate, value.cannonKonaPrestate)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UpdatePrestateInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    systemConfigProxy: tuple.0,
                    cannonPrestate: tuple.1,
                    cannonKonaPrestate: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UpdatePrestateInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UpdatePrestateInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.systemConfigProxy,
                    ),
                    <Claim as alloy_sol_types::SolType>::tokenize(&self.cannonPrestate),
                    <Claim as alloy_sol_types::SolType>::tokenize(
                        &self.cannonKonaPrestate,
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
        impl alloy_sol_types::SolType for UpdatePrestateInput {
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
        impl alloy_sol_types::SolStruct for UpdatePrestateInput {
            const NAME: &'static str = "UpdatePrestateInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UpdatePrestateInput(address systemConfigProxy,bytes32 cannonPrestate,bytes32 cannonKonaPrestate)",
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
                            &self.systemConfigProxy,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cannonPrestate,
                        )
                        .0,
                    <Claim as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cannonKonaPrestate,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UpdatePrestateInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.systemConfigProxy,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cannonPrestate,
                    )
                    + <Claim as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cannonKonaPrestate,
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
                    &rust.systemConfigProxy,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cannonPrestate,
                    out,
                );
                <Claim as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cannonKonaPrestate,
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
    /**Custom error with signature `AddressHasNoCode(address)` and selector `0x86bb51b8`.
```solidity
error AddressHasNoCode(address who);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressHasNoCode {
        #[allow(missing_docs)]
        pub who: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AddressHasNoCode> for UnderlyingRustTuple<'_> {
            fn from(value: AddressHasNoCode) -> Self {
                (value.who,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressHasNoCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { who: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressHasNoCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressHasNoCode(address)";
            const SELECTOR: [u8; 4] = [134u8, 187u8, 81u8, 184u8];
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
                        &self.who,
                    ),
                )
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
    /**Custom error with signature `AddressNotFound(address)` and selector `0x70de3231`.
```solidity
error AddressNotFound(address who);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressNotFound {
        #[allow(missing_docs)]
        pub who: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AddressNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: AddressNotFound) -> Self {
                (value.who,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { who: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressNotFound(address)";
            const SELECTOR: [u8; 4] = [112u8, 222u8, 50u8, 49u8];
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
                        &self.who,
                    ),
                )
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
    /**Custom error with signature `AlreadyReleased()` and selector `0x63b4904e`.
```solidity
error AlreadyReleased();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyReleased;
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
        impl ::core::convert::From<AlreadyReleased> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyReleased) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyReleased {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyReleased {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyReleased()";
            const SELECTOR: [u8; 4] = [99u8, 180u8, 144u8, 78u8];
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
    /**Custom error with signature `InvalidChainId()` and selector `0x7a47c9a2`.
```solidity
error InvalidChainId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidChainId;
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
        impl ::core::convert::From<InvalidChainId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidChainId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidChainId()";
            const SELECTOR: [u8; 4] = [122u8, 71u8, 201u8, 162u8];
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
    /**Custom error with signature `InvalidDevFeatureAccess(bytes32)` and selector `0x228ff8f2`.
```solidity
error InvalidDevFeatureAccess(bytes32 devFeature);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDevFeatureAccess {
        #[allow(missing_docs)]
        pub devFeature: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<InvalidDevFeatureAccess> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDevFeatureAccess) -> Self {
                (value.devFeature,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidDevFeatureAccess {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { devFeature: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDevFeatureAccess {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDevFeatureAccess(bytes32)";
            const SELECTOR: [u8; 4] = [34u8, 143u8, 248u8, 242u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.devFeature),
                )
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
    /**Custom error with signature `InvalidGameConfigs()` and selector `0xea116472`.
```solidity
error InvalidGameConfigs();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidGameConfigs;
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
        impl ::core::convert::From<InvalidGameConfigs> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidGameConfigs) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidGameConfigs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidGameConfigs {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidGameConfigs()";
            const SELECTOR: [u8; 4] = [234u8, 17u8, 100u8, 114u8];
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
    /**Custom error with signature `InvalidRoleAddress(string)` and selector `0xe646e043`.
```solidity
error InvalidRoleAddress(string role);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRoleAddress {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::String,
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
        impl ::core::convert::From<InvalidRoleAddress> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRoleAddress) -> Self {
                (value.role,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRoleAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { role: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRoleAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRoleAddress(string)";
            const SELECTOR: [u8; 4] = [230u8, 70u8, 224u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.role,
                    ),
                )
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
    /**Custom error with signature `InvalidStartingAnchorRoot()` and selector `0x89e0acdf`.
```solidity
error InvalidStartingAnchorRoot();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidStartingAnchorRoot;
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
        impl ::core::convert::From<InvalidStartingAnchorRoot>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidStartingAnchorRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidStartingAnchorRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidStartingAnchorRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidStartingAnchorRoot()";
            const SELECTOR: [u8; 4] = [137u8, 224u8, 172u8, 223u8];
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
    /**Custom error with signature `LatestReleaseNotSet()` and selector `0xe7a227dc`.
```solidity
error LatestReleaseNotSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LatestReleaseNotSet;
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
        impl ::core::convert::From<LatestReleaseNotSet> for UnderlyingRustTuple<'_> {
            fn from(value: LatestReleaseNotSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LatestReleaseNotSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LatestReleaseNotSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LatestReleaseNotSet()";
            const SELECTOR: [u8; 4] = [231u8, 162u8, 39u8, 220u8];
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
    /**Custom error with signature `OnlyDelegatecall()` and selector `0x0a57d61d`.
```solidity
error OnlyDelegatecall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyDelegatecall;
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
        impl ::core::convert::From<OnlyDelegatecall> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyDelegatecall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyDelegatecall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyDelegatecall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyDelegatecall()";
            const SELECTOR: [u8; 4] = [10u8, 87u8, 214u8, 29u8];
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
    /**Custom error with signature `PrestateNotSet()` and selector `0xefc878f2`.
```solidity
error PrestateNotSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PrestateNotSet;
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
        impl ::core::convert::From<PrestateNotSet> for UnderlyingRustTuple<'_> {
            fn from(value: PrestateNotSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PrestateNotSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PrestateNotSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PrestateNotSet()";
            const SELECTOR: [u8; 4] = [239u8, 200u8, 120u8, 242u8];
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
    /**Custom error with signature `PrestateRequired()` and selector `0xb30c83ab`.
```solidity
error PrestateRequired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PrestateRequired;
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
        impl ::core::convert::From<PrestateRequired> for UnderlyingRustTuple<'_> {
            fn from(value: PrestateRequired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PrestateRequired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PrestateRequired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PrestateRequired()";
            const SELECTOR: [u8; 4] = [179u8, 12u8, 131u8, 171u8];
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
    /**Custom error with signature `SuperchainConfigMismatch(address)` and selector `0xc358e75a`.
```solidity
error SuperchainConfigMismatch(address systemConfig);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SuperchainConfigMismatch {
        #[allow(missing_docs)]
        pub systemConfig: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<SuperchainConfigMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: SuperchainConfigMismatch) -> Self {
                (value.systemConfig,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SuperchainConfigMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { systemConfig: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SuperchainConfigMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SuperchainConfigMismatch(address)";
            const SELECTOR: [u8; 4] = [195u8, 88u8, 231u8, 90u8];
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
                        &self.systemConfig,
                    ),
                )
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
    /**Custom error with signature `SuperchainProxyAdminMismatch()` and selector `0x83e442ff`.
```solidity
error SuperchainProxyAdminMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SuperchainProxyAdminMismatch;
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
        impl ::core::convert::From<SuperchainProxyAdminMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: SuperchainProxyAdminMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SuperchainProxyAdminMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SuperchainProxyAdminMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SuperchainProxyAdminMismatch()";
            const SELECTOR: [u8; 4] = [131u8, 228u8, 66u8, 255u8];
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
constructor(address _opcmGameTypeAdder, address _opcmDeployer, address _opcmUpgrader, address _opcmInteropMigrator, address _opcmStandardValidator, address _superchainConfig, address _protocolVersions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _opcmGameTypeAdder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _opcmDeployer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _opcmUpgrader: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _opcmInteropMigrator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _opcmStandardValidator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _superchainConfig: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _protocolVersions: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
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
                    (
                        value._opcmGameTypeAdder,
                        value._opcmDeployer,
                        value._opcmUpgrader,
                        value._opcmInteropMigrator,
                        value._opcmStandardValidator,
                        value._superchainConfig,
                        value._protocolVersions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _opcmGameTypeAdder: tuple.0,
                        _opcmDeployer: tuple.1,
                        _opcmUpgrader: tuple.2,
                        _opcmInteropMigrator: tuple.3,
                        _opcmStandardValidator: tuple.4,
                        _superchainConfig: tuple.5,
                        _protocolVersions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
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
                        &self._opcmGameTypeAdder,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._opcmDeployer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._opcmUpgrader,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._opcmInteropMigrator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._opcmStandardValidator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._superchainConfig,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._protocolVersions,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addGameType((string,address,address,uint32,bytes32,uint256,uint256,uint64,uint64,uint256,address,bool)[])` and selector `0x604aa628`.
```solidity
function addGameType(AddGameInput[] memory _gameConfigs) external returns (AddGameOutput[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addGameTypeCall {
        #[allow(missing_docs)]
        pub _gameConfigs: alloy::sol_types::private::Vec<
            <AddGameInput as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`addGameType((string,address,address,uint32,bytes32,uint256,uint256,uint64,uint64,uint256,address,bool)[])`](addGameTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addGameTypeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <AddGameOutput as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<AddGameInput>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <AddGameInput as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addGameTypeCall> for UnderlyingRustTuple<'_> {
                fn from(value: addGameTypeCall) -> Self {
                    (value._gameConfigs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addGameTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _gameConfigs: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<AddGameOutput>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <AddGameOutput as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addGameTypeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addGameTypeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addGameTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addGameTypeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Array<AddGameInput>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <AddGameOutput as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<AddGameOutput>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addGameType((string,address,address,uint32,bytes32,uint256,uint256,uint64,uint64,uint256,address,bool)[])";
            const SELECTOR: [u8; 4] = [96u8, 74u8, 166u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        AddGameInput,
                    > as alloy_sol_types::SolType>::tokenize(&self._gameConfigs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        AddGameOutput,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: addGameTypeReturn = r.into();
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
                        let r: addGameTypeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `blueprints()` and selector `0xb51f9c2b`.
```solidity
function blueprints() external view returns (Blueprints memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blueprintsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`blueprints()`](blueprintsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blueprintsReturn {
        #[allow(missing_docs)]
        pub _0: <Blueprints as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<blueprintsCall> for UnderlyingRustTuple<'_> {
                fn from(value: blueprintsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blueprintsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Blueprints,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Blueprints as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<blueprintsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: blueprintsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blueprintsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blueprintsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Blueprints as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Blueprints,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blueprints()";
            const SELECTOR: [u8; 4] = [181u8, 31u8, 156u8, 43u8];
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
                (<Blueprints as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: blueprintsReturn = r.into();
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
                        let r: blueprintsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `chainIdToBatchInboxAddress(uint256)` and selector `0x318b1b80`.
```solidity
function chainIdToBatchInboxAddress(uint256 _l2ChainId) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainIdToBatchInboxAddressCall {
        #[allow(missing_docs)]
        pub _l2ChainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`chainIdToBatchInboxAddress(uint256)`](chainIdToBatchInboxAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainIdToBatchInboxAddressReturn {
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
            impl ::core::convert::From<chainIdToBatchInboxAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: chainIdToBatchInboxAddressCall) -> Self {
                    (value._l2ChainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for chainIdToBatchInboxAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _l2ChainId: tuple.0 }
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
            impl ::core::convert::From<chainIdToBatchInboxAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: chainIdToBatchInboxAddressReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for chainIdToBatchInboxAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for chainIdToBatchInboxAddressCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "chainIdToBatchInboxAddress(uint256)";
            const SELECTOR: [u8; 4] = [49u8, 139u8, 27u8, 128u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._l2ChainId),
                )
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
                        let r: chainIdToBatchInboxAddressReturn = r.into();
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
                        let r: chainIdToBatchInboxAddressReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `deploy(((address,address,address,address,address,address),uint32,uint32,uint256,bytes,string,uint64,uint32,bytes32,uint256,uint256,uint64,uint64,bool))` and selector `0x6ab5f661`.
```solidity
function deploy(DeployInput memory _input) external returns (DeployOutput memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deployCall {
        #[allow(missing_docs)]
        pub _input: <DeployInput as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`deploy(((address,address,address,address,address,address),uint32,uint32,uint256,bytes,string,uint64,uint32,bytes32,uint256,uint256,uint64,uint64,bool))`](deployCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deployReturn {
        #[allow(missing_docs)]
        pub _0: <DeployOutput as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (DeployInput,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DeployInput as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<deployCall> for UnderlyingRustTuple<'_> {
                fn from(value: deployCall) -> Self {
                    (value._input,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deployCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _input: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (DeployOutput,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DeployOutput as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<deployReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deployReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deployReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deployCall {
            type Parameters<'a> = (DeployInput,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <DeployOutput as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (DeployOutput,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deploy(((address,address,address,address,address,address),uint32,uint32,uint256,bytes,string,uint64,uint32,bytes32,uint256,uint256,uint64,uint64,bool))";
            const SELECTOR: [u8; 4] = [106u8, 181u8, 246u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<DeployInput as alloy_sol_types::SolType>::tokenize(&self._input),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<DeployOutput as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: deployReturn = r.into();
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
                        let r: deployReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `devFeatureBitmap()` and selector `0x1d8a4e92`.
```solidity
function devFeatureBitmap() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct devFeatureBitmapCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`devFeatureBitmap()`](devFeatureBitmapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct devFeatureBitmapReturn {
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
            impl ::core::convert::From<devFeatureBitmapCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: devFeatureBitmapCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for devFeatureBitmapCall {
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
            impl ::core::convert::From<devFeatureBitmapReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: devFeatureBitmapReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for devFeatureBitmapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for devFeatureBitmapCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "devFeatureBitmap()";
            const SELECTOR: [u8; 4] = [29u8, 138u8, 78u8, 146u8];
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
                        let r: devFeatureBitmapReturn = r.into();
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
                        let r: devFeatureBitmapReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `implementations()` and selector `0x30e9012c`.
```solidity
function implementations() external view returns (Implementations memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct implementationsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`implementations()`](implementationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct implementationsReturn {
        #[allow(missing_docs)]
        pub _0: <Implementations as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<implementationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: implementationsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for implementationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Implementations,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Implementations as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<implementationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: implementationsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for implementationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for implementationsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Implementations as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Implementations,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "implementations()";
            const SELECTOR: [u8; 4] = [48u8, 233u8, 1u8, 44u8];
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
                (<Implementations as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: implementationsReturn = r.into();
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
                        let r: implementationsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isDevFeatureEnabled(bytes32)` and selector `0x78ecabce`.
```solidity
function isDevFeatureEnabled(bytes32 _feature) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDevFeatureEnabledCall {
        #[allow(missing_docs)]
        pub _feature: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isDevFeatureEnabled(bytes32)`](isDevFeatureEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDevFeatureEnabledReturn {
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
            impl ::core::convert::From<isDevFeatureEnabledCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDevFeatureEnabledCall) -> Self {
                    (value._feature,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDevFeatureEnabledCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _feature: tuple.0 }
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
            impl ::core::convert::From<isDevFeatureEnabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDevFeatureEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDevFeatureEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDevFeatureEnabledCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDevFeatureEnabled(bytes32)";
            const SELECTOR: [u8; 4] = [120u8, 236u8, 171u8, 206u8];
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
                        let r: isDevFeatureEnabledReturn = r.into();
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
                        let r: isDevFeatureEnabledReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Function with signature `migrate((bool,(bytes32,uint256),(address,address,uint256,uint256,uint256,uint64,uint64),(address,bytes32,bytes32)[]))` and selector `0x58084273`.
```solidity
function migrate(OPContractsManagerInteropMigrator.MigrateInput memory _input) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateCall {
        #[allow(missing_docs)]
        pub _input: <OPContractsManagerInteropMigrator::MigrateInput as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`migrate((bool,(bytes32,uint256),(address,address,uint256,uint256,uint256,uint64,uint64),(address,bytes32,bytes32)[]))`](migrateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateReturn {}
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
                OPContractsManagerInteropMigrator::MigrateInput,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OPContractsManagerInteropMigrator::MigrateInput as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<migrateCall> for UnderlyingRustTuple<'_> {
                fn from(value: migrateCall) -> Self {
                    (value._input,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for migrateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _input: tuple.0 }
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
            impl ::core::convert::From<migrateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: migrateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for migrateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl migrateReturn {
            fn _tokenize(
                &self,
            ) -> <migrateCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateCall {
            type Parameters<'a> = (OPContractsManagerInteropMigrator::MigrateInput,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrate((bool,(bytes32,uint256),(address,address,uint256,uint256,uint256,uint64,uint64),(address,bytes32,bytes32)[]))";
            const SELECTOR: [u8; 4] = [88u8, 8u8, 66u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OPContractsManagerInteropMigrator::MigrateInput as alloy_sol_types::SolType>::tokenize(
                        &self._input,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                migrateReturn::_tokenize(ret)
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
    /**Function with signature `opcmDeployer()` and selector `0x622d56f1`.
```solidity
function opcmDeployer() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmDeployerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`opcmDeployer()`](opcmDeployerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmDeployerReturn {
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
            impl ::core::convert::From<opcmDeployerCall> for UnderlyingRustTuple<'_> {
                fn from(value: opcmDeployerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for opcmDeployerCall {
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
            impl ::core::convert::From<opcmDeployerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: opcmDeployerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for opcmDeployerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for opcmDeployerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "opcmDeployer()";
            const SELECTOR: [u8; 4] = [98u8, 45u8, 86u8, 241u8];
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
                        let r: opcmDeployerReturn = r.into();
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
                        let r: opcmDeployerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `opcmGameTypeAdder()` and selector `0xbecbdf4a`.
```solidity
function opcmGameTypeAdder() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmGameTypeAdderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`opcmGameTypeAdder()`](opcmGameTypeAdderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmGameTypeAdderReturn {
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
            impl ::core::convert::From<opcmGameTypeAdderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: opcmGameTypeAdderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opcmGameTypeAdderCall {
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
            impl ::core::convert::From<opcmGameTypeAdderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: opcmGameTypeAdderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opcmGameTypeAdderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for opcmGameTypeAdderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "opcmGameTypeAdder()";
            const SELECTOR: [u8; 4] = [190u8, 203u8, 223u8, 74u8];
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
                        let r: opcmGameTypeAdderReturn = r.into();
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
                        let r: opcmGameTypeAdderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `opcmInteropMigrator()` and selector `0x1481a724`.
```solidity
function opcmInteropMigrator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmInteropMigratorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`opcmInteropMigrator()`](opcmInteropMigratorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmInteropMigratorReturn {
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
            impl ::core::convert::From<opcmInteropMigratorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: opcmInteropMigratorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opcmInteropMigratorCall {
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
            impl ::core::convert::From<opcmInteropMigratorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: opcmInteropMigratorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opcmInteropMigratorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for opcmInteropMigratorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "opcmInteropMigrator()";
            const SELECTOR: [u8; 4] = [20u8, 129u8, 167u8, 36u8];
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
                        let r: opcmInteropMigratorReturn = r.into();
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
                        let r: opcmInteropMigratorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `opcmStandardValidator()` and selector `0xba7903db`.
```solidity
function opcmStandardValidator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmStandardValidatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`opcmStandardValidator()`](opcmStandardValidatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmStandardValidatorReturn {
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
            impl ::core::convert::From<opcmStandardValidatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: opcmStandardValidatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opcmStandardValidatorCall {
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
            impl ::core::convert::From<opcmStandardValidatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: opcmStandardValidatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for opcmStandardValidatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for opcmStandardValidatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "opcmStandardValidator()";
            const SELECTOR: [u8; 4] = [186u8, 121u8, 3u8, 219u8];
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
                        let r: opcmStandardValidatorReturn = r.into();
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
                        let r: opcmStandardValidatorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `opcmUpgrader()` and selector `0x03dbe68c`.
```solidity
function opcmUpgrader() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmUpgraderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`opcmUpgrader()`](opcmUpgraderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct opcmUpgraderReturn {
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
            impl ::core::convert::From<opcmUpgraderCall> for UnderlyingRustTuple<'_> {
                fn from(value: opcmUpgraderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for opcmUpgraderCall {
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
            impl ::core::convert::From<opcmUpgraderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: opcmUpgraderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for opcmUpgraderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for opcmUpgraderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "opcmUpgrader()";
            const SELECTOR: [u8; 4] = [3u8, 219u8, 230u8, 140u8];
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
                        let r: opcmUpgraderReturn = r.into();
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
                        let r: opcmUpgraderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `protocolVersions()` and selector `0x6624856a`.
```solidity
function protocolVersions() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct protocolVersionsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`protocolVersions()`](protocolVersionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct protocolVersionsReturn {
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
            impl ::core::convert::From<protocolVersionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolVersionsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolVersionsCall {
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
            impl ::core::convert::From<protocolVersionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolVersionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolVersionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for protocolVersionsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "protocolVersions()";
            const SELECTOR: [u8; 4] = [102u8, 36u8, 133u8, 106u8];
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
                        let r: protocolVersionsReturn = r.into();
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
                        let r: protocolVersionsReturn = r.into();
                        r._0
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
    /**Function with signature `updatePrestate((address,bytes32,bytes32)[])` and selector `0xb23cc044`.
```solidity
function updatePrestate(UpdatePrestateInput[] memory _prestateUpdateInputs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePrestateCall {
        #[allow(missing_docs)]
        pub _prestateUpdateInputs: alloy::sol_types::private::Vec<
            <UpdatePrestateInput as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`updatePrestate((address,bytes32,bytes32)[])`](updatePrestateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePrestateReturn {}
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
                alloy::sol_types::sol_data::Array<UpdatePrestateInput>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <UpdatePrestateInput as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updatePrestateCall> for UnderlyingRustTuple<'_> {
                fn from(value: updatePrestateCall) -> Self {
                    (value._prestateUpdateInputs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updatePrestateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _prestateUpdateInputs: tuple.0,
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
            impl ::core::convert::From<updatePrestateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updatePrestateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updatePrestateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updatePrestateReturn {
            fn _tokenize(
                &self,
            ) -> <updatePrestateCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updatePrestateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<UpdatePrestateInput>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updatePrestateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updatePrestate((address,bytes32,bytes32)[])";
            const SELECTOR: [u8; 4] = [178u8, 60u8, 192u8, 68u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        UpdatePrestateInput,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._prestateUpdateInputs,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updatePrestateReturn::_tokenize(ret)
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
    /**Function with signature `upgrade((address,bytes32,bytes32)[])` and selector `0xcbeda5a7`.
```solidity
function upgrade(OpChainConfig[] memory _opChainConfigs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeCall {
        #[allow(missing_docs)]
        pub _opChainConfigs: alloy::sol_types::private::Vec<
            <OpChainConfig as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`upgrade((address,bytes32,bytes32)[])`](upgradeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeReturn {}
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
                alloy::sol_types::sol_data::Array<OpChainConfig>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OpChainConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<upgradeCall> for UnderlyingRustTuple<'_> {
                fn from(value: upgradeCall) -> Self {
                    (value._opChainConfigs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for upgradeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _opChainConfigs: tuple.0 }
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
            impl ::core::convert::From<upgradeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: upgradeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for upgradeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl upgradeReturn {
            fn _tokenize(
                &self,
            ) -> <upgradeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Array<OpChainConfig>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgrade((address,bytes32,bytes32)[])";
            const SELECTOR: [u8; 4] = [203u8, 237u8, 165u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        OpChainConfig,
                    > as alloy_sol_types::SolType>::tokenize(&self._opChainConfigs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                upgradeReturn::_tokenize(ret)
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
    /**Function with signature `upgradeSuperchainConfig(address)` and selector `0xc993f27c`.
```solidity
function upgradeSuperchainConfig(address _superchainConfig) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeSuperchainConfigCall {
        #[allow(missing_docs)]
        pub _superchainConfig: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`upgradeSuperchainConfig(address)`](upgradeSuperchainConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeSuperchainConfigReturn {}
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
            impl ::core::convert::From<upgradeSuperchainConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeSuperchainConfigCall) -> Self {
                    (value._superchainConfig,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeSuperchainConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _superchainConfig: tuple.0 }
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
            impl ::core::convert::From<upgradeSuperchainConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeSuperchainConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeSuperchainConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl upgradeSuperchainConfigReturn {
            fn _tokenize(
                &self,
            ) -> <upgradeSuperchainConfigCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeSuperchainConfigCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeSuperchainConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgradeSuperchainConfig(address)";
            const SELECTOR: [u8; 4] = [201u8, 147u8, 242u8, 124u8];
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
                        &self._superchainConfig,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                upgradeSuperchainConfigReturn::_tokenize(ret)
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
    /**Function with signature `validate((address,bytes32,uint256,address),bool)` and selector `0x41fe5385`.
```solidity
function validate(OPContractsManagerStandardValidator.ValidationInput memory _input, bool _allowFailure) external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validate_0Call {
        #[allow(missing_docs)]
        pub _input: <OPContractsManagerStandardValidator::ValidationInput as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _allowFailure: bool,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`validate((address,bytes32,uint256,address),bool)`](validate_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validate_0Return {
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
            type UnderlyingSolTuple<'a> = (
                OPContractsManagerStandardValidator::ValidationInput,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OPContractsManagerStandardValidator::ValidationInput as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validate_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: validate_0Call) -> Self {
                    (value._input, value._allowFailure)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validate_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _input: tuple.0,
                        _allowFailure: tuple.1,
                    }
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
            impl ::core::convert::From<validate_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: validate_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validate_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validate_0Call {
            type Parameters<'a> = (
                OPContractsManagerStandardValidator::ValidationInput,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validate((address,bytes32,uint256,address),bool)";
            const SELECTOR: [u8; 4] = [65u8, 254u8, 83u8, 133u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OPContractsManagerStandardValidator::ValidationInput as alloy_sol_types::SolType>::tokenize(
                        &self._input,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._allowFailure,
                    ),
                )
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
                        let r: validate_0Return = r.into();
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
                        let r: validate_0Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validate((address,bytes32,bytes32,uint256,address),bool)` and selector `0xf3edcbe1`.
```solidity
function validate(OPContractsManagerStandardValidator.ValidationInputDev memory _input, bool _allowFailure) external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validate_1Call {
        #[allow(missing_docs)]
        pub _input: <OPContractsManagerStandardValidator::ValidationInputDev as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _allowFailure: bool,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`validate((address,bytes32,bytes32,uint256,address),bool)`](validate_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validate_1Return {
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
            type UnderlyingSolTuple<'a> = (
                OPContractsManagerStandardValidator::ValidationInputDev,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OPContractsManagerStandardValidator::ValidationInputDev as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validate_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: validate_1Call) -> Self {
                    (value._input, value._allowFailure)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validate_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _input: tuple.0,
                        _allowFailure: tuple.1,
                    }
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
            impl ::core::convert::From<validate_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: validate_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validate_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validate_1Call {
            type Parameters<'a> = (
                OPContractsManagerStandardValidator::ValidationInputDev,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validate((address,bytes32,bytes32,uint256,address),bool)";
            const SELECTOR: [u8; 4] = [243u8, 237u8, 203u8, 225u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OPContractsManagerStandardValidator::ValidationInputDev as alloy_sol_types::SolType>::tokenize(
                        &self._input,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._allowFailure,
                    ),
                )
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
                        let r: validate_1Return = r.into();
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
                        let r: validate_1Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validateWithOverrides((address,bytes32,uint256,address),bool,(address,address))` and selector `0x0e9d5cb9`.
```solidity
function validateWithOverrides(OPContractsManagerStandardValidator.ValidationInput memory _input, bool _allowFailure, OPContractsManagerStandardValidator.ValidationOverrides memory _overrides) external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateWithOverrides_0Call {
        #[allow(missing_docs)]
        pub _input: <OPContractsManagerStandardValidator::ValidationInput as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _allowFailure: bool,
        #[allow(missing_docs)]
        pub _overrides: <OPContractsManagerStandardValidator::ValidationOverrides as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`validateWithOverrides((address,bytes32,uint256,address),bool,(address,address))`](validateWithOverrides_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateWithOverrides_0Return {
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
            type UnderlyingSolTuple<'a> = (
                OPContractsManagerStandardValidator::ValidationInput,
                alloy::sol_types::sol_data::Bool,
                OPContractsManagerStandardValidator::ValidationOverrides,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OPContractsManagerStandardValidator::ValidationInput as alloy::sol_types::SolType>::RustType,
                bool,
                <OPContractsManagerStandardValidator::ValidationOverrides as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validateWithOverrides_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateWithOverrides_0Call) -> Self {
                    (value._input, value._allowFailure, value._overrides)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateWithOverrides_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _input: tuple.0,
                        _allowFailure: tuple.1,
                        _overrides: tuple.2,
                    }
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
            impl ::core::convert::From<validateWithOverrides_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateWithOverrides_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateWithOverrides_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateWithOverrides_0Call {
            type Parameters<'a> = (
                OPContractsManagerStandardValidator::ValidationInput,
                alloy::sol_types::sol_data::Bool,
                OPContractsManagerStandardValidator::ValidationOverrides,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateWithOverrides((address,bytes32,uint256,address),bool,(address,address))";
            const SELECTOR: [u8; 4] = [14u8, 157u8, 92u8, 185u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OPContractsManagerStandardValidator::ValidationInput as alloy_sol_types::SolType>::tokenize(
                        &self._input,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._allowFailure,
                    ),
                    <OPContractsManagerStandardValidator::ValidationOverrides as alloy_sol_types::SolType>::tokenize(
                        &self._overrides,
                    ),
                )
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
                        let r: validateWithOverrides_0Return = r.into();
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
                        let r: validateWithOverrides_0Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validateWithOverrides((address,bytes32,bytes32,uint256,address),bool,(address,address))` and selector `0x8970ac44`.
```solidity
function validateWithOverrides(OPContractsManagerStandardValidator.ValidationInputDev memory _input, bool _allowFailure, OPContractsManagerStandardValidator.ValidationOverrides memory _overrides) external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateWithOverrides_1Call {
        #[allow(missing_docs)]
        pub _input: <OPContractsManagerStandardValidator::ValidationInputDev as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _allowFailure: bool,
        #[allow(missing_docs)]
        pub _overrides: <OPContractsManagerStandardValidator::ValidationOverrides as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`validateWithOverrides((address,bytes32,bytes32,uint256,address),bool,(address,address))`](validateWithOverrides_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateWithOverrides_1Return {
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
            type UnderlyingSolTuple<'a> = (
                OPContractsManagerStandardValidator::ValidationInputDev,
                alloy::sol_types::sol_data::Bool,
                OPContractsManagerStandardValidator::ValidationOverrides,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OPContractsManagerStandardValidator::ValidationInputDev as alloy::sol_types::SolType>::RustType,
                bool,
                <OPContractsManagerStandardValidator::ValidationOverrides as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validateWithOverrides_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateWithOverrides_1Call) -> Self {
                    (value._input, value._allowFailure, value._overrides)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateWithOverrides_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _input: tuple.0,
                        _allowFailure: tuple.1,
                        _overrides: tuple.2,
                    }
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
            impl ::core::convert::From<validateWithOverrides_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateWithOverrides_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateWithOverrides_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateWithOverrides_1Call {
            type Parameters<'a> = (
                OPContractsManagerStandardValidator::ValidationInputDev,
                alloy::sol_types::sol_data::Bool,
                OPContractsManagerStandardValidator::ValidationOverrides,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateWithOverrides((address,bytes32,bytes32,uint256,address),bool,(address,address))";
            const SELECTOR: [u8; 4] = [137u8, 112u8, 172u8, 68u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OPContractsManagerStandardValidator::ValidationInputDev as alloy_sol_types::SolType>::tokenize(
                        &self._input,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._allowFailure,
                    ),
                    <OPContractsManagerStandardValidator::ValidationOverrides as alloy_sol_types::SolType>::tokenize(
                        &self._overrides,
                    ),
                )
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
                        let r: validateWithOverrides_1Return = r.into();
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
                        let r: validateWithOverrides_1Return = r.into();
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
    ///Container for all the [`OPContractsManager`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum OPContractsManagerCalls {
        #[allow(missing_docs)]
        addGameType(addGameTypeCall),
        #[allow(missing_docs)]
        blueprints(blueprintsCall),
        #[allow(missing_docs)]
        chainIdToBatchInboxAddress(chainIdToBatchInboxAddressCall),
        #[allow(missing_docs)]
        deploy(deployCall),
        #[allow(missing_docs)]
        devFeatureBitmap(devFeatureBitmapCall),
        #[allow(missing_docs)]
        implementations(implementationsCall),
        #[allow(missing_docs)]
        isDevFeatureEnabled(isDevFeatureEnabledCall),
        #[allow(missing_docs)]
        migrate(migrateCall),
        #[allow(missing_docs)]
        opcmDeployer(opcmDeployerCall),
        #[allow(missing_docs)]
        opcmGameTypeAdder(opcmGameTypeAdderCall),
        #[allow(missing_docs)]
        opcmInteropMigrator(opcmInteropMigratorCall),
        #[allow(missing_docs)]
        opcmStandardValidator(opcmStandardValidatorCall),
        #[allow(missing_docs)]
        opcmUpgrader(opcmUpgraderCall),
        #[allow(missing_docs)]
        protocolVersions(protocolVersionsCall),
        #[allow(missing_docs)]
        superchainConfig(superchainConfigCall),
        #[allow(missing_docs)]
        updatePrestate(updatePrestateCall),
        #[allow(missing_docs)]
        upgrade(upgradeCall),
        #[allow(missing_docs)]
        upgradeSuperchainConfig(upgradeSuperchainConfigCall),
        #[allow(missing_docs)]
        validate_0(validate_0Call),
        #[allow(missing_docs)]
        validate_1(validate_1Call),
        #[allow(missing_docs)]
        validateWithOverrides_0(validateWithOverrides_0Call),
        #[allow(missing_docs)]
        validateWithOverrides_1(validateWithOverrides_1Call),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl OPContractsManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 219u8, 230u8, 140u8],
            [14u8, 157u8, 92u8, 185u8],
            [20u8, 129u8, 167u8, 36u8],
            [29u8, 138u8, 78u8, 146u8],
            [48u8, 233u8, 1u8, 44u8],
            [49u8, 139u8, 27u8, 128u8],
            [53u8, 232u8, 10u8, 179u8],
            [65u8, 254u8, 83u8, 133u8],
            [84u8, 253u8, 77u8, 80u8],
            [88u8, 8u8, 66u8, 115u8],
            [96u8, 74u8, 166u8, 40u8],
            [98u8, 45u8, 86u8, 241u8],
            [102u8, 36u8, 133u8, 106u8],
            [106u8, 181u8, 246u8, 97u8],
            [120u8, 236u8, 171u8, 206u8],
            [137u8, 112u8, 172u8, 68u8],
            [178u8, 60u8, 192u8, 68u8],
            [181u8, 31u8, 156u8, 43u8],
            [186u8, 121u8, 3u8, 219u8],
            [190u8, 203u8, 223u8, 74u8],
            [201u8, 147u8, 242u8, 124u8],
            [203u8, 237u8, 165u8, 167u8],
            [243u8, 237u8, 203u8, 225u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(opcmUpgrader),
            ::core::stringify!(validateWithOverrides_0),
            ::core::stringify!(opcmInteropMigrator),
            ::core::stringify!(devFeatureBitmap),
            ::core::stringify!(implementations),
            ::core::stringify!(chainIdToBatchInboxAddress),
            ::core::stringify!(superchainConfig),
            ::core::stringify!(validate_0),
            ::core::stringify!(version),
            ::core::stringify!(migrate),
            ::core::stringify!(addGameType),
            ::core::stringify!(opcmDeployer),
            ::core::stringify!(protocolVersions),
            ::core::stringify!(deploy),
            ::core::stringify!(isDevFeatureEnabled),
            ::core::stringify!(validateWithOverrides_1),
            ::core::stringify!(updatePrestate),
            ::core::stringify!(blueprints),
            ::core::stringify!(opcmStandardValidator),
            ::core::stringify!(opcmGameTypeAdder),
            ::core::stringify!(upgradeSuperchainConfig),
            ::core::stringify!(upgrade),
            ::core::stringify!(validate_1),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <opcmUpgraderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validateWithOverrides_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <opcmInteropMigratorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <devFeatureBitmapCall as alloy_sol_types::SolCall>::SIGNATURE,
            <implementationsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <chainIdToBatchInboxAddressCall as alloy_sol_types::SolCall>::SIGNATURE,
            <superchainConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validate_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <migrateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addGameTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <opcmDeployerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <protocolVersionsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <deployCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isDevFeatureEnabledCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validateWithOverrides_1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <updatePrestateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <blueprintsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <opcmStandardValidatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <opcmGameTypeAdderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeSuperchainConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validate_1Call as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OPContractsManagerCalls {
        const NAME: &'static str = "OPContractsManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addGameType(_) => {
                    <addGameTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blueprints(_) => {
                    <blueprintsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::chainIdToBatchInboxAddress(_) => {
                    <chainIdToBatchInboxAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deploy(_) => <deployCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::devFeatureBitmap(_) => {
                    <devFeatureBitmapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::implementations(_) => {
                    <implementationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDevFeatureEnabled(_) => {
                    <isDevFeatureEnabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrate(_) => <migrateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::opcmDeployer(_) => {
                    <opcmDeployerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::opcmGameTypeAdder(_) => {
                    <opcmGameTypeAdderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::opcmInteropMigrator(_) => {
                    <opcmInteropMigratorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::opcmStandardValidator(_) => {
                    <opcmStandardValidatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::opcmUpgrader(_) => {
                    <opcmUpgraderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::protocolVersions(_) => {
                    <protocolVersionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::superchainConfig(_) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updatePrestate(_) => {
                    <updatePrestateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgrade(_) => <upgradeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::upgradeSuperchainConfig(_) => {
                    <upgradeSuperchainConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validate_0(_) => {
                    <validate_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validate_1(_) => {
                    <validate_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateWithOverrides_0(_) => {
                    <validateWithOverrides_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateWithOverrides_1(_) => {
                    <validateWithOverrides_1Call as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OPContractsManagerCalls>] = &[
                {
                    fn opcmUpgrader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmUpgraderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmUpgrader)
                    }
                    opcmUpgrader
                },
                {
                    fn validateWithOverrides_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validateWithOverrides_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::validateWithOverrides_0)
                    }
                    validateWithOverrides_0
                },
                {
                    fn opcmInteropMigrator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmInteropMigratorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmInteropMigrator)
                    }
                    opcmInteropMigrator
                },
                {
                    fn devFeatureBitmap(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <devFeatureBitmapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::devFeatureBitmap)
                    }
                    devFeatureBitmap
                },
                {
                    fn implementations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <implementationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::implementations)
                    }
                    implementations
                },
                {
                    fn chainIdToBatchInboxAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <chainIdToBatchInboxAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::chainIdToBatchInboxAddress)
                    }
                    chainIdToBatchInboxAddress
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn validate_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validate_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::validate_0)
                    }
                    validate_0
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPContractsManagerCalls::version)
                    }
                    version
                },
                {
                    fn migrate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <migrateCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPContractsManagerCalls::migrate)
                    }
                    migrate
                },
                {
                    fn addGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <addGameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::addGameType)
                    }
                    addGameType
                },
                {
                    fn opcmDeployer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmDeployerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmDeployer)
                    }
                    opcmDeployer
                },
                {
                    fn protocolVersions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <protocolVersionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::protocolVersions)
                    }
                    protocolVersions
                },
                {
                    fn deploy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <deployCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPContractsManagerCalls::deploy)
                    }
                    deploy
                },
                {
                    fn isDevFeatureEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <isDevFeatureEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::isDevFeatureEnabled)
                    }
                    isDevFeatureEnabled
                },
                {
                    fn validateWithOverrides_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validateWithOverrides_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::validateWithOverrides_1)
                    }
                    validateWithOverrides_1
                },
                {
                    fn updatePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <updatePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::updatePrestate)
                    }
                    updatePrestate
                },
                {
                    fn blueprints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <blueprintsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::blueprints)
                    }
                    blueprints
                },
                {
                    fn opcmStandardValidator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmStandardValidatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmStandardValidator)
                    }
                    opcmStandardValidator
                },
                {
                    fn opcmGameTypeAdder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmGameTypeAdderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmGameTypeAdder)
                    }
                    opcmGameTypeAdder
                },
                {
                    fn upgradeSuperchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <upgradeSuperchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::upgradeSuperchainConfig)
                    }
                    upgradeSuperchainConfig
                },
                {
                    fn upgrade(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <upgradeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OPContractsManagerCalls::upgrade)
                    }
                    upgrade
                },
                {
                    fn validate_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validate_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerCalls::validate_1)
                    }
                    validate_1
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
            ) -> alloy_sol_types::Result<OPContractsManagerCalls>] = &[
                {
                    fn opcmUpgrader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmUpgraderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmUpgrader)
                    }
                    opcmUpgrader
                },
                {
                    fn validateWithOverrides_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validateWithOverrides_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::validateWithOverrides_0)
                    }
                    validateWithOverrides_0
                },
                {
                    fn opcmInteropMigrator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmInteropMigratorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmInteropMigrator)
                    }
                    opcmInteropMigrator
                },
                {
                    fn devFeatureBitmap(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <devFeatureBitmapCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::devFeatureBitmap)
                    }
                    devFeatureBitmap
                },
                {
                    fn implementations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <implementationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::implementations)
                    }
                    implementations
                },
                {
                    fn chainIdToBatchInboxAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <chainIdToBatchInboxAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::chainIdToBatchInboxAddress)
                    }
                    chainIdToBatchInboxAddress
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn validate_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validate_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::validate_0)
                    }
                    validate_0
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::version)
                    }
                    version
                },
                {
                    fn migrate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <migrateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::migrate)
                    }
                    migrate
                },
                {
                    fn addGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <addGameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::addGameType)
                    }
                    addGameType
                },
                {
                    fn opcmDeployer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmDeployerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmDeployer)
                    }
                    opcmDeployer
                },
                {
                    fn protocolVersions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <protocolVersionsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::protocolVersions)
                    }
                    protocolVersions
                },
                {
                    fn deploy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <deployCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::deploy)
                    }
                    deploy
                },
                {
                    fn isDevFeatureEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <isDevFeatureEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::isDevFeatureEnabled)
                    }
                    isDevFeatureEnabled
                },
                {
                    fn validateWithOverrides_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validateWithOverrides_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::validateWithOverrides_1)
                    }
                    validateWithOverrides_1
                },
                {
                    fn updatePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <updatePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::updatePrestate)
                    }
                    updatePrestate
                },
                {
                    fn blueprints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <blueprintsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::blueprints)
                    }
                    blueprints
                },
                {
                    fn opcmStandardValidator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmStandardValidatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmStandardValidator)
                    }
                    opcmStandardValidator
                },
                {
                    fn opcmGameTypeAdder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <opcmGameTypeAdderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::opcmGameTypeAdder)
                    }
                    opcmGameTypeAdder
                },
                {
                    fn upgradeSuperchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <upgradeSuperchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::upgradeSuperchainConfig)
                    }
                    upgradeSuperchainConfig
                },
                {
                    fn upgrade(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <upgradeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::upgrade)
                    }
                    upgrade
                },
                {
                    fn validate_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerCalls> {
                        <validate_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerCalls::validate_1)
                    }
                    validate_1
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
                Self::addGameType(inner) => {
                    <addGameTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blueprints(inner) => {
                    <blueprintsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::chainIdToBatchInboxAddress(inner) => {
                    <chainIdToBatchInboxAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deploy(inner) => {
                    <deployCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::devFeatureBitmap(inner) => {
                    <devFeatureBitmapCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::implementations(inner) => {
                    <implementationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isDevFeatureEnabled(inner) => {
                    <isDevFeatureEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::migrate(inner) => {
                    <migrateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::opcmDeployer(inner) => {
                    <opcmDeployerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::opcmGameTypeAdder(inner) => {
                    <opcmGameTypeAdderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::opcmInteropMigrator(inner) => {
                    <opcmInteropMigratorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::opcmStandardValidator(inner) => {
                    <opcmStandardValidatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::opcmUpgrader(inner) => {
                    <opcmUpgraderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::protocolVersions(inner) => {
                    <protocolVersionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::superchainConfig(inner) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updatePrestate(inner) => {
                    <updatePrestateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgrade(inner) => {
                    <upgradeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::upgradeSuperchainConfig(inner) => {
                    <upgradeSuperchainConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validate_0(inner) => {
                    <validate_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::validate_1(inner) => {
                    <validate_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::validateWithOverrides_0(inner) => {
                    <validateWithOverrides_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validateWithOverrides_1(inner) => {
                    <validateWithOverrides_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::addGameType(inner) => {
                    <addGameTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blueprints(inner) => {
                    <blueprintsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::chainIdToBatchInboxAddress(inner) => {
                    <chainIdToBatchInboxAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deploy(inner) => {
                    <deployCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::devFeatureBitmap(inner) => {
                    <devFeatureBitmapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::implementations(inner) => {
                    <implementationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isDevFeatureEnabled(inner) => {
                    <isDevFeatureEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::migrate(inner) => {
                    <migrateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::opcmDeployer(inner) => {
                    <opcmDeployerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::opcmGameTypeAdder(inner) => {
                    <opcmGameTypeAdderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::opcmInteropMigrator(inner) => {
                    <opcmInteropMigratorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::opcmStandardValidator(inner) => {
                    <opcmStandardValidatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::opcmUpgrader(inner) => {
                    <opcmUpgraderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::protocolVersions(inner) => {
                    <protocolVersionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updatePrestate(inner) => {
                    <updatePrestateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::upgrade(inner) => {
                    <upgradeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::upgradeSuperchainConfig(inner) => {
                    <upgradeSuperchainConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validate_0(inner) => {
                    <validate_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validate_1(inner) => {
                    <validate_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateWithOverrides_0(inner) => {
                    <validateWithOverrides_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateWithOverrides_1(inner) => {
                    <validateWithOverrides_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`OPContractsManager`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OPContractsManagerErrors {
        #[allow(missing_docs)]
        AddressHasNoCode(AddressHasNoCode),
        #[allow(missing_docs)]
        AddressNotFound(AddressNotFound),
        #[allow(missing_docs)]
        AlreadyReleased(AlreadyReleased),
        #[allow(missing_docs)]
        InvalidChainId(InvalidChainId),
        #[allow(missing_docs)]
        InvalidDevFeatureAccess(InvalidDevFeatureAccess),
        #[allow(missing_docs)]
        InvalidGameConfigs(InvalidGameConfigs),
        #[allow(missing_docs)]
        InvalidRoleAddress(InvalidRoleAddress),
        #[allow(missing_docs)]
        InvalidStartingAnchorRoot(InvalidStartingAnchorRoot),
        #[allow(missing_docs)]
        LatestReleaseNotSet(LatestReleaseNotSet),
        #[allow(missing_docs)]
        OnlyDelegatecall(OnlyDelegatecall),
        #[allow(missing_docs)]
        PrestateNotSet(PrestateNotSet),
        #[allow(missing_docs)]
        PrestateRequired(PrestateRequired),
        #[allow(missing_docs)]
        SuperchainConfigMismatch(SuperchainConfigMismatch),
        #[allow(missing_docs)]
        SuperchainProxyAdminMismatch(SuperchainProxyAdminMismatch),
    }
    impl OPContractsManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 87u8, 214u8, 29u8],
            [34u8, 143u8, 248u8, 242u8],
            [99u8, 180u8, 144u8, 78u8],
            [112u8, 222u8, 50u8, 49u8],
            [122u8, 71u8, 201u8, 162u8],
            [131u8, 228u8, 66u8, 255u8],
            [134u8, 187u8, 81u8, 184u8],
            [137u8, 224u8, 172u8, 223u8],
            [179u8, 12u8, 131u8, 171u8],
            [195u8, 88u8, 231u8, 90u8],
            [230u8, 70u8, 224u8, 67u8],
            [231u8, 162u8, 39u8, 220u8],
            [234u8, 17u8, 100u8, 114u8],
            [239u8, 200u8, 120u8, 242u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(OnlyDelegatecall),
            ::core::stringify!(InvalidDevFeatureAccess),
            ::core::stringify!(AlreadyReleased),
            ::core::stringify!(AddressNotFound),
            ::core::stringify!(InvalidChainId),
            ::core::stringify!(SuperchainProxyAdminMismatch),
            ::core::stringify!(AddressHasNoCode),
            ::core::stringify!(InvalidStartingAnchorRoot),
            ::core::stringify!(PrestateRequired),
            ::core::stringify!(SuperchainConfigMismatch),
            ::core::stringify!(InvalidRoleAddress),
            ::core::stringify!(LatestReleaseNotSet),
            ::core::stringify!(InvalidGameConfigs),
            ::core::stringify!(PrestateNotSet),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <OnlyDelegatecall as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDevFeatureAccess as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyReleased as alloy_sol_types::SolError>::SIGNATURE,
            <AddressNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidChainId as alloy_sol_types::SolError>::SIGNATURE,
            <SuperchainProxyAdminMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <AddressHasNoCode as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidStartingAnchorRoot as alloy_sol_types::SolError>::SIGNATURE,
            <PrestateRequired as alloy_sol_types::SolError>::SIGNATURE,
            <SuperchainConfigMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidRoleAddress as alloy_sol_types::SolError>::SIGNATURE,
            <LatestReleaseNotSet as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidGameConfigs as alloy_sol_types::SolError>::SIGNATURE,
            <PrestateNotSet as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OPContractsManagerErrors {
        const NAME: &'static str = "OPContractsManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 14usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressHasNoCode(_) => {
                    <AddressHasNoCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AddressNotFound(_) => {
                    <AddressNotFound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadyReleased(_) => {
                    <AlreadyReleased as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidChainId(_) => {
                    <InvalidChainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDevFeatureAccess(_) => {
                    <InvalidDevFeatureAccess as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidGameConfigs(_) => {
                    <InvalidGameConfigs as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRoleAddress(_) => {
                    <InvalidRoleAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidStartingAnchorRoot(_) => {
                    <InvalidStartingAnchorRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LatestReleaseNotSet(_) => {
                    <LatestReleaseNotSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyDelegatecall(_) => {
                    <OnlyDelegatecall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PrestateNotSet(_) => {
                    <PrestateNotSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PrestateRequired(_) => {
                    <PrestateRequired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SuperchainConfigMismatch(_) => {
                    <SuperchainConfigMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SuperchainProxyAdminMismatch(_) => {
                    <SuperchainProxyAdminMismatch as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<OPContractsManagerErrors>] = &[
                {
                    fn OnlyDelegatecall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <OnlyDelegatecall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::OnlyDelegatecall)
                    }
                    OnlyDelegatecall
                },
                {
                    fn InvalidDevFeatureAccess(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidDevFeatureAccess as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidDevFeatureAccess)
                    }
                    InvalidDevFeatureAccess
                },
                {
                    fn AlreadyReleased(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <AlreadyReleased as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::AlreadyReleased)
                    }
                    AlreadyReleased
                },
                {
                    fn AddressNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <AddressNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::AddressNotFound)
                    }
                    AddressNotFound
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn SuperchainProxyAdminMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <SuperchainProxyAdminMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::SuperchainProxyAdminMismatch)
                    }
                    SuperchainProxyAdminMismatch
                },
                {
                    fn AddressHasNoCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <AddressHasNoCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::AddressHasNoCode)
                    }
                    AddressHasNoCode
                },
                {
                    fn InvalidStartingAnchorRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidStartingAnchorRoot as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidStartingAnchorRoot)
                    }
                    InvalidStartingAnchorRoot
                },
                {
                    fn PrestateRequired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <PrestateRequired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::PrestateRequired)
                    }
                    PrestateRequired
                },
                {
                    fn SuperchainConfigMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <SuperchainConfigMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::SuperchainConfigMismatch)
                    }
                    SuperchainConfigMismatch
                },
                {
                    fn InvalidRoleAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidRoleAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidRoleAddress)
                    }
                    InvalidRoleAddress
                },
                {
                    fn LatestReleaseNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <LatestReleaseNotSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::LatestReleaseNotSet)
                    }
                    LatestReleaseNotSet
                },
                {
                    fn InvalidGameConfigs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidGameConfigs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidGameConfigs)
                    }
                    InvalidGameConfigs
                },
                {
                    fn PrestateNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <PrestateNotSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OPContractsManagerErrors::PrestateNotSet)
                    }
                    PrestateNotSet
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
            ) -> alloy_sol_types::Result<OPContractsManagerErrors>] = &[
                {
                    fn OnlyDelegatecall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <OnlyDelegatecall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::OnlyDelegatecall)
                    }
                    OnlyDelegatecall
                },
                {
                    fn InvalidDevFeatureAccess(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidDevFeatureAccess as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidDevFeatureAccess)
                    }
                    InvalidDevFeatureAccess
                },
                {
                    fn AlreadyReleased(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <AlreadyReleased as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::AlreadyReleased)
                    }
                    AlreadyReleased
                },
                {
                    fn AddressNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <AddressNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::AddressNotFound)
                    }
                    AddressNotFound
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn SuperchainProxyAdminMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <SuperchainProxyAdminMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::SuperchainProxyAdminMismatch)
                    }
                    SuperchainProxyAdminMismatch
                },
                {
                    fn AddressHasNoCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <AddressHasNoCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::AddressHasNoCode)
                    }
                    AddressHasNoCode
                },
                {
                    fn InvalidStartingAnchorRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidStartingAnchorRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidStartingAnchorRoot)
                    }
                    InvalidStartingAnchorRoot
                },
                {
                    fn PrestateRequired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <PrestateRequired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::PrestateRequired)
                    }
                    PrestateRequired
                },
                {
                    fn SuperchainConfigMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <SuperchainConfigMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::SuperchainConfigMismatch)
                    }
                    SuperchainConfigMismatch
                },
                {
                    fn InvalidRoleAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidRoleAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidRoleAddress)
                    }
                    InvalidRoleAddress
                },
                {
                    fn LatestReleaseNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <LatestReleaseNotSet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::LatestReleaseNotSet)
                    }
                    LatestReleaseNotSet
                },
                {
                    fn InvalidGameConfigs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <InvalidGameConfigs as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::InvalidGameConfigs)
                    }
                    InvalidGameConfigs
                },
                {
                    fn PrestateNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OPContractsManagerErrors> {
                        <PrestateNotSet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OPContractsManagerErrors::PrestateNotSet)
                    }
                    PrestateNotSet
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
                Self::AddressHasNoCode(inner) => {
                    <AddressHasNoCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AddressNotFound(inner) => {
                    <AddressNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AlreadyReleased(inner) => {
                    <AlreadyReleased as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidDevFeatureAccess(inner) => {
                    <InvalidDevFeatureAccess as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidGameConfigs(inner) => {
                    <InvalidGameConfigs as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidRoleAddress(inner) => {
                    <InvalidRoleAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidStartingAnchorRoot(inner) => {
                    <InvalidStartingAnchorRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LatestReleaseNotSet(inner) => {
                    <LatestReleaseNotSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyDelegatecall(inner) => {
                    <OnlyDelegatecall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PrestateNotSet(inner) => {
                    <PrestateNotSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PrestateRequired(inner) => {
                    <PrestateRequired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SuperchainConfigMismatch(inner) => {
                    <SuperchainConfigMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SuperchainProxyAdminMismatch(inner) => {
                    <SuperchainProxyAdminMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AddressHasNoCode(inner) => {
                    <AddressHasNoCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AddressNotFound(inner) => {
                    <AddressNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadyReleased(inner) => {
                    <AlreadyReleased as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDevFeatureAccess(inner) => {
                    <InvalidDevFeatureAccess as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidGameConfigs(inner) => {
                    <InvalidGameConfigs as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRoleAddress(inner) => {
                    <InvalidRoleAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidStartingAnchorRoot(inner) => {
                    <InvalidStartingAnchorRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LatestReleaseNotSet(inner) => {
                    <LatestReleaseNotSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyDelegatecall(inner) => {
                    <OnlyDelegatecall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PrestateNotSet(inner) => {
                    <PrestateNotSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PrestateRequired(inner) => {
                    <PrestateRequired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SuperchainConfigMismatch(inner) => {
                    <SuperchainConfigMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SuperchainProxyAdminMismatch(inner) => {
                    <SuperchainProxyAdminMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OPContractsManager`](self) contract instance.

See the [wrapper's documentation](`OPContractsManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OPContractsManagerInstance<P, N> {
        OPContractsManagerInstance::<P, N>::new(address, __provider)
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
        _opcmGameTypeAdder: alloy::sol_types::private::Address,
        _opcmDeployer: alloy::sol_types::private::Address,
        _opcmUpgrader: alloy::sol_types::private::Address,
        _opcmInteropMigrator: alloy::sol_types::private::Address,
        _opcmStandardValidator: alloy::sol_types::private::Address,
        _superchainConfig: alloy::sol_types::private::Address,
        _protocolVersions: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OPContractsManagerInstance<P, N>>,
    > {
        OPContractsManagerInstance::<
            P,
            N,
        >::deploy(
            __provider,
            _opcmGameTypeAdder,
            _opcmDeployer,
            _opcmUpgrader,
            _opcmInteropMigrator,
            _opcmStandardValidator,
            _superchainConfig,
            _protocolVersions,
        )
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
        _opcmGameTypeAdder: alloy::sol_types::private::Address,
        _opcmDeployer: alloy::sol_types::private::Address,
        _opcmUpgrader: alloy::sol_types::private::Address,
        _opcmInteropMigrator: alloy::sol_types::private::Address,
        _opcmStandardValidator: alloy::sol_types::private::Address,
        _superchainConfig: alloy::sol_types::private::Address,
        _protocolVersions: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        OPContractsManagerInstance::<
            P,
            N,
        >::deploy_builder(
            __provider,
            _opcmGameTypeAdder,
            _opcmDeployer,
            _opcmUpgrader,
            _opcmInteropMigrator,
            _opcmStandardValidator,
            _superchainConfig,
            _protocolVersions,
        )
    }
    /**A [`OPContractsManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OPContractsManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OPContractsManagerInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OPContractsManagerInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OPContractsManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OPContractsManagerInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OPContractsManager`](self) contract instance.

See the [wrapper's documentation](`OPContractsManagerInstance`) for more details.*/
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
            _opcmGameTypeAdder: alloy::sol_types::private::Address,
            _opcmDeployer: alloy::sol_types::private::Address,
            _opcmUpgrader: alloy::sol_types::private::Address,
            _opcmInteropMigrator: alloy::sol_types::private::Address,
            _opcmStandardValidator: alloy::sol_types::private::Address,
            _superchainConfig: alloy::sol_types::private::Address,
            _protocolVersions: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<OPContractsManagerInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                _opcmGameTypeAdder,
                _opcmDeployer,
                _opcmUpgrader,
                _opcmInteropMigrator,
                _opcmStandardValidator,
                _superchainConfig,
                _protocolVersions,
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
            _opcmGameTypeAdder: alloy::sol_types::private::Address,
            _opcmDeployer: alloy::sol_types::private::Address,
            _opcmUpgrader: alloy::sol_types::private::Address,
            _opcmInteropMigrator: alloy::sol_types::private::Address,
            _opcmStandardValidator: alloy::sol_types::private::Address,
            _superchainConfig: alloy::sol_types::private::Address,
            _protocolVersions: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _opcmGameTypeAdder,
                            _opcmDeployer,
                            _opcmUpgrader,
                            _opcmInteropMigrator,
                            _opcmStandardValidator,
                            _superchainConfig,
                            _protocolVersions,
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
    impl<P: ::core::clone::Clone, N> OPContractsManagerInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OPContractsManagerInstance<P, N> {
            OPContractsManagerInstance {
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
    > OPContractsManagerInstance<P, N> {
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
        ///Creates a new call builder for the [`addGameType`] function.
        pub fn addGameType(
            &self,
            _gameConfigs: alloy::sol_types::private::Vec<
                <AddGameInput as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, addGameTypeCall, N> {
            self.call_builder(&addGameTypeCall { _gameConfigs })
        }
        ///Creates a new call builder for the [`blueprints`] function.
        pub fn blueprints(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, blueprintsCall, N> {
            self.call_builder(&blueprintsCall)
        }
        ///Creates a new call builder for the [`chainIdToBatchInboxAddress`] function.
        pub fn chainIdToBatchInboxAddress(
            &self,
            _l2ChainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, chainIdToBatchInboxAddressCall, N> {
            self.call_builder(
                &chainIdToBatchInboxAddressCall {
                    _l2ChainId,
                },
            )
        }
        ///Creates a new call builder for the [`deploy_call`] function.
        pub fn deploy_call(
            &self,
            _input: <DeployInput as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, deployCall, N> {
            self.call_builder(&deployCall { _input })
        }
        ///Creates a new call builder for the [`devFeatureBitmap`] function.
        pub fn devFeatureBitmap(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, devFeatureBitmapCall, N> {
            self.call_builder(&devFeatureBitmapCall)
        }
        ///Creates a new call builder for the [`implementations`] function.
        pub fn implementations(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, implementationsCall, N> {
            self.call_builder(&implementationsCall)
        }
        ///Creates a new call builder for the [`isDevFeatureEnabled`] function.
        pub fn isDevFeatureEnabled(
            &self,
            _feature: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isDevFeatureEnabledCall, N> {
            self.call_builder(
                &isDevFeatureEnabledCall {
                    _feature,
                },
            )
        }
        ///Creates a new call builder for the [`migrate`] function.
        pub fn migrate(
            &self,
            _input: <OPContractsManagerInteropMigrator::MigrateInput as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, migrateCall, N> {
            self.call_builder(&migrateCall { _input })
        }
        ///Creates a new call builder for the [`opcmDeployer`] function.
        pub fn opcmDeployer(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, opcmDeployerCall, N> {
            self.call_builder(&opcmDeployerCall)
        }
        ///Creates a new call builder for the [`opcmGameTypeAdder`] function.
        pub fn opcmGameTypeAdder(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, opcmGameTypeAdderCall, N> {
            self.call_builder(&opcmGameTypeAdderCall)
        }
        ///Creates a new call builder for the [`opcmInteropMigrator`] function.
        pub fn opcmInteropMigrator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, opcmInteropMigratorCall, N> {
            self.call_builder(&opcmInteropMigratorCall)
        }
        ///Creates a new call builder for the [`opcmStandardValidator`] function.
        pub fn opcmStandardValidator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, opcmStandardValidatorCall, N> {
            self.call_builder(&opcmStandardValidatorCall)
        }
        ///Creates a new call builder for the [`opcmUpgrader`] function.
        pub fn opcmUpgrader(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, opcmUpgraderCall, N> {
            self.call_builder(&opcmUpgraderCall)
        }
        ///Creates a new call builder for the [`protocolVersions`] function.
        pub fn protocolVersions(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, protocolVersionsCall, N> {
            self.call_builder(&protocolVersionsCall)
        }
        ///Creates a new call builder for the [`superchainConfig`] function.
        pub fn superchainConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, superchainConfigCall, N> {
            self.call_builder(&superchainConfigCall)
        }
        ///Creates a new call builder for the [`updatePrestate`] function.
        pub fn updatePrestate(
            &self,
            _prestateUpdateInputs: alloy::sol_types::private::Vec<
                <UpdatePrestateInput as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, updatePrestateCall, N> {
            self.call_builder(
                &updatePrestateCall {
                    _prestateUpdateInputs,
                },
            )
        }
        ///Creates a new call builder for the [`upgrade`] function.
        pub fn upgrade(
            &self,
            _opChainConfigs: alloy::sol_types::private::Vec<
                <OpChainConfig as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, upgradeCall, N> {
            self.call_builder(&upgradeCall { _opChainConfigs })
        }
        ///Creates a new call builder for the [`upgradeSuperchainConfig`] function.
        pub fn upgradeSuperchainConfig(
            &self,
            _superchainConfig: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, upgradeSuperchainConfigCall, N> {
            self.call_builder(
                &upgradeSuperchainConfigCall {
                    _superchainConfig,
                },
            )
        }
        ///Creates a new call builder for the [`validate_0`] function.
        pub fn validate_0(
            &self,
            _input: <OPContractsManagerStandardValidator::ValidationInput as alloy::sol_types::SolType>::RustType,
            _allowFailure: bool,
        ) -> alloy_contract::SolCallBuilder<&P, validate_0Call, N> {
            self.call_builder(
                &validate_0Call {
                    _input,
                    _allowFailure,
                },
            )
        }
        ///Creates a new call builder for the [`validate_1`] function.
        pub fn validate_1(
            &self,
            _input: <OPContractsManagerStandardValidator::ValidationInputDev as alloy::sol_types::SolType>::RustType,
            _allowFailure: bool,
        ) -> alloy_contract::SolCallBuilder<&P, validate_1Call, N> {
            self.call_builder(
                &validate_1Call {
                    _input,
                    _allowFailure,
                },
            )
        }
        ///Creates a new call builder for the [`validateWithOverrides_0`] function.
        pub fn validateWithOverrides_0(
            &self,
            _input: <OPContractsManagerStandardValidator::ValidationInput as alloy::sol_types::SolType>::RustType,
            _allowFailure: bool,
            _overrides: <OPContractsManagerStandardValidator::ValidationOverrides as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, validateWithOverrides_0Call, N> {
            self.call_builder(
                &validateWithOverrides_0Call {
                    _input,
                    _allowFailure,
                    _overrides,
                },
            )
        }
        ///Creates a new call builder for the [`validateWithOverrides_1`] function.
        pub fn validateWithOverrides_1(
            &self,
            _input: <OPContractsManagerStandardValidator::ValidationInputDev as alloy::sol_types::SolType>::RustType,
            _allowFailure: bool,
            _overrides: <OPContractsManagerStandardValidator::ValidationOverrides as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, validateWithOverrides_1Call, N> {
            self.call_builder(
                &validateWithOverrides_1Call {
                    _input,
                    _allowFailure,
                    _overrides,
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
    > OPContractsManagerInstance<P, N> {
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

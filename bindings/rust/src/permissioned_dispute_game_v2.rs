///Module containing a contract's types and functions.
/**

```solidity
library FaultDisputeGameV2 {
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
pub mod FaultDisputeGameV2 {
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
    /**Creates a new wrapper around an on-chain [`FaultDisputeGameV2`](self) contract instance.

See the [wrapper's documentation](`FaultDisputeGameV2Instance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> FaultDisputeGameV2Instance<P, N> {
        FaultDisputeGameV2Instance::<P, N>::new(address, __provider)
    }
    /**A [`FaultDisputeGameV2`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`FaultDisputeGameV2`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct FaultDisputeGameV2Instance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for FaultDisputeGameV2Instance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("FaultDisputeGameV2Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FaultDisputeGameV2Instance<P, N> {
        /**Creates a new wrapper around an on-chain [`FaultDisputeGameV2`](self) contract instance.

See the [wrapper's documentation](`FaultDisputeGameV2Instance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> FaultDisputeGameV2Instance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> FaultDisputeGameV2Instance<P, N> {
            FaultDisputeGameV2Instance {
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
    > FaultDisputeGameV2Instance<P, N> {
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
    > FaultDisputeGameV2Instance<P, N> {
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
library FaultDisputeGameV2 {
    struct GameConstructorParams {
        uint256 maxGameDepth;
        uint256 splitDepth;
        Duration clockExtension;
        Duration maxClockDuration;
    }
}

library Types {
    struct OutputRootProof {
        bytes32 version;
        bytes32 stateRoot;
        bytes32 messagePasserStorageRoot;
        bytes32 latestBlockhash;
    }
}

interface PermissionedDisputeGameV2 {
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
    error UnexpectedList();
    error UnexpectedRootClaim(Claim rootClaim);
    error UnexpectedString();
    error ValidStep();

    event GameClosed(BondDistributionMode bondDistributionMode);
    event Move(uint256 indexed parentIndex, Claim indexed claim, address indexed claimant);
    event Resolved(GameStatus indexed status);

    constructor(FaultDisputeGameV2.GameConstructorParams _params);

    function absolutePrestate() external pure returns (Claim absolutePrestate_);
    function addLocalData(uint256 _ident, uint256 _execLeafIdx, uint256 _partOffset) external;
    function anchorStateRegistry() external pure returns (address registry_);
    function attack(Claim _disputed, uint256 _parentIndex, Claim _claim) external payable;
    function bondDistributionMode() external view returns (BondDistributionMode);
    function challengeRootL2Block(Types.OutputRootProof memory _outputRootProof, bytes memory _headerRLP) external;
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
    function l2BlockNumber() external pure returns (uint256 l2BlockNumber_);
    function l2BlockNumberChallenged() external view returns (bool);
    function l2BlockNumberChallenger() external view returns (address);
    function l2ChainId() external pure returns (uint256 l2ChainId_);
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
    function startingBlockNumber() external view returns (uint256 startingBlockNumber_);
    function startingOutputRoot() external view returns (Hash root, uint256 l2SequenceNumber);
    function startingRootHash() external view returns (Hash startingRootHash_);
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
        "internalType": "struct FaultDisputeGameV2.GameConstructorParams",
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
pub mod PermissionedDisputeGameV2 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101006040523480156200001257600080fd5b5060405162006208380380620062088339810160408190526200003591620001e7565b80620000446001607e62000281565b60ff16816000015111156200006c57604051633beff19960e11b815260040160405180910390fd5b600019816020015114806200009357508051602082015162000090906001620002a7565b10155b15620000b25760405163e62ccf3960e01b815260040160405180910390fd5b600281602001511015620000d95760405163e62ccf3960e01b815260040160405180910390fd5b6000620000fe82604001516001600160401b0316620001c760201b62000c601760201c565b62000114906001600160401b03166002620002c2565b90506001600160401b038111156200013f5760405163235dfb2b60e21b815260040160405180910390fd5b6200016282606001516001600160401b0316620001c760201b62000c601760201c565b6001600160401b0316816001600160401b03161115620001955760405163235dfb2b60e21b815260040160405180910390fd5b508051608052602081015160a05260408101516001600160401b0390811660e0526060909101511660c05250620002e4565b90565b80516001600160401b0381168114620001e257600080fd5b919050565b600060808284031215620001fa57600080fd5b604051608081016001600160401b03811182821017156200022b57634e487b7160e01b600052604160045260246000fd5b806040525082518152602083015160208201526200024c60408401620001ca565b60408201526200025f60608401620001ca565b60608201529392505050565b634e487b7160e01b600052601160045260246000fd5b600060ff821660ff8416808210156200029e576200029e6200026b565b90039392505050565b60008219821115620002bd57620002bd6200026b565b500190565b6000816000190483118215151615620002df57620002df6200026b565b500290565b60805160a05160c05160e051615e26620003e26000396000818161073001528181612f9f0152818161300a0152818161303d015281816137210152613839015260008181610a8c01528181610ef301528181611fb501528181611ff701528181612e360152818161306d015281816130cc01526138d6015260008181610abf0152818161252601528181612c7f01528181612da501528181612fdb015281816143210152818161472601528181614805015281816148b8015281816150e60152615303015260008181610b66015281816120a30152818161212901528181612d4801528181612e9a0152818161423901526143420152615e266000f3fe6080604052600436106103555760003560e01c806370872aa5116101bb578063c0d8bb74116100f7578063dabd396d11610095578063f8f43ff61161006f578063f8f43ff614610b13578063fa24f74314610b33578063fa315aa914610b57578063fe2bbeb214610b8a57600080fd5b8063dabd396d14610a7d578063ec5e630814610ab0578063eff0f59214610ae357600080fd5b8063cf09e0d0116100d1578063cf09e0d0146109fa578063d5d44d8014610a1b578063d6ae3cd514610a3b578063d8cc1a3c14610a5d57600080fd5b8063c0d8bb7414610939578063c395e1ca14610966578063c6f0308c1461098657600080fd5b80638d450a9511610164578063a8e4fb901161013e578063a8e4fb90146108a5578063bbdc02db146108ca578063bcef3b55146108f7578063bd8da9561461091957600080fd5b80638d450a95146107e357806399735e32146107c1578063a445ece61461080557600080fd5b80638129fc1c116101955780638129fc1c146107a45780638980e0cc146107ac5780638b85902b146107c157600080fd5b806370872aa514610767578063786b844b1461077c5780637b0f0adc1461079157600080fd5b80633e3ac912116102955780635a5fa2d91161023357806360e274641161020d57806360e27464146106df5780636361506d146106ff5780636b6716c0146107215780636f0344091461075457600080fd5b80635a5fa2d9146106855780635c0cba33146106a5578063609d3334146106ca57600080fd5b8063529d6a8c1161026f578063529d6a8c146105c4578063534db0e2146105f157806354fd4d501461060657806357da950e1461065557600080fd5b80633e3ac9121461055c5780633fc8cef31461058c578063472777c6146105b157600080fd5b806325fc2ace1161030257806330dbe570116102dc57806330dbe570146104c3578063378dd48c146104fb57806337b1b229146105155780633a7684631461053757600080fd5b806325fc2ace1461046f5780632810e1d61461048e5780632ad69aeb146104a357600080fd5b8063200d2ed211610333578063200d2ed2146103e7578063222abf4514610415578063250e69bd1461045557600080fd5b8063019351301461035a57806303c2924d1461037c57806319effeb41461039c575b600080fd5b34801561036657600080fd5b5061037a610375366004615696565b610bba565b005b34801561038857600080fd5b5061037a6103973660046156f1565b610e79565b3480156103a857600080fd5b506000546103c99068010000000000000000900467ffffffffffffffff1681565b60405167ffffffffffffffff90911681526020015b60405180910390f35b3480156103f357600080fd5b5060005461040890600160801b900460ff1681565b6040516103de919061574a565b34801561042157600080fd5b50610445610430366004615772565b600c6020526000908152604090205460ff1681565b60405190151581526020016103de565b34801561046157600080fd5b50600a546104459060ff1681565b34801561047b57600080fd5b506008545b6040519081526020016103de565b34801561049a57600080fd5b50610408611401565b3480156104af57600080fd5b506104806104be3660046156f1565b6115d9565b3480156104cf57600080fd5b506001546104e3906001600160a01b031681565b6040516001600160a01b0390911681526020016103de565b34801561050757600080fd5b50600d546104089060ff1681565b34801561052157600080fd5b503660011981013560f01c90033560601c6104e3565b34801561054357600080fd5b503660011981013560f01c90036098013560601c6104e3565b34801561056857600080fd5b50600054610445907201000000000000000000000000000000000000900460ff1681565b34801561059857600080fd5b503660011981013560f01c900360c0013560601c6104e3565b61037a6105bf36600461578f565b61160f565b3480156105d057600080fd5b506104806105df366004615772565b60036020526000908152604090205481565b3480156105fd57600080fd5b506104e3611621565b34801561061257600080fd5b5060408051808201909152600581527f322e322e3000000000000000000000000000000000000000000000000000000060208201525b6040516103de9190615826565b34801561066157600080fd5b50600854600954610670919082565b604080519283526020830191909152016103de565b34801561069157600080fd5b506104806106a0366004615839565b611646565b3480156106b157600080fd5b503660011981013560f01c900360ac013560601c6104e3565b3480156106d657600080fd5b50610648611680565b3480156106eb57600080fd5b5061037a6106fa366004615772565b61168e565b34801561070b57600080fd5b503660011981013560f01c900360340135610480565b34801561072d57600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103c9565b61037a610762366004615860565b61199f565b34801561077357600080fd5b50600954610480565b34801561078857600080fd5b5061037a611a1f565b61037a61079f36600461578f565b611e0d565b61037a611e1a565b3480156107b857600080fd5b50600254610480565b3480156107cd57600080fd5b503660011981013560f01c900360580135610480565b3480156107ef57600080fd5b503660011981013560f01c900360780135610480565b34801561081157600080fd5b50610867610820366004615839565b6007602052600090815260409020805460019091015460ff821691610100810463ffffffff1691650100000000009091046001600160801b0316906001600160a01b031684565b60408051941515855263ffffffff90931660208501526001600160801b03909116918301919091526001600160a01b031660608201526080016103de565b3480156108b157600080fd5b503660011981013560f01c900360f4013560601c6104e3565b3480156108d657600080fd5b506040513660011981013560f01c90036054013560e01c81526020016103de565b34801561090357600080fd5b503660011981013560f01c900360140135610480565b34801561092557600080fd5b506103c9610934366004615839565b611e85565b34801561094557600080fd5b50610480610954366004615772565b600b6020526000908152604090205481565b34801561097257600080fd5b506104806109813660046158a1565b61201f565b34801561099257600080fd5b506109a66109a1366004615839565b6121e0565b6040805163ffffffff90981688526001600160a01b03968716602089015295909416948601949094526001600160801b039182166060860152608085015291821660a08401521660c082015260e0016103de565b348015610a0657600080fd5b506000546103c99067ffffffffffffffff1681565b348015610a2757600080fd5b50610480610a36366004615772565b612254565b348015610a4757600080fd5b503660011981013560f01c900360d40135610480565b348015610a6957600080fd5b5061037a610a783660046158ca565b6122ac565b348015610a8957600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103c9565b348015610abc57600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610480565b348015610aef57600080fd5b50610445610afe366004615839565b60046020526000908152604090205460ff1681565b348015610b1f57600080fd5b5061037a610b2e36600461578f565b612328565b348015610b3f57600080fd5b50610b486126ef565b6040516103de93929190615956565b348015610b6357600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610480565b348015610b9657600080fd5b50610445610ba5366004615839565b60066020526000908152604090205460ff1681565b60008054600160801b900460ff166002811115610bd957610bd9615713565b14610bf75760405163067fe19560e41b815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff1615610c4a576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610c633660011981013560f01c90036014013590565b90565b610c7a610c7536869003860186615991565b612719565b14610cb1576040517f9cc00b5b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b82606001358282604051610cc6929190615a05565b604051809103902014610d05576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610d4e610d4984848080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061277592505050565b6127e2565b90506000610d7582600881518110610d6857610d68615a15565b6020026020010151612998565b9050602081511115610db3576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602081810151825190910360031b1c3660011981013560f01c9003605801358103610e0a576040517fb8ed883000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050600180547fffffffffffffffffffffffff000000000000000000000000000000000000000016331790555050600080547fffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffff1672010000000000000000000000000000000000001790555050565b60008054600160801b900460ff166002811115610e9857610e98615713565b14610eb65760405163067fe19560e41b815260040160405180910390fd5b600060028381548110610ecb57610ecb615a15565b906000526020600020906005020190506000610ee684611e85565b905067ffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081169082161015610f4f576040517ff2440b5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008481526006602052604090205460ff1615610f98576040517ff1a9458100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000848152600560205260409020805480158015610fb557508515155b1561101857835464010000000090046001600160a01b031660008115610fdb5781610fea565b60018601546001600160a01b03165b9050610ff68187612a4c565b505050600094855250506006602052505060409020805460ff19166001179055565b6000868152600760209081526040918290208251608081018452815460ff81161515808352610100820463ffffffff16948301949094526501000000000090046001600160801b031693810193909352600101546001600160a01b0316606083015261109c576001600160801b03604082015260018152600086900361109c578195505b600086826020015163ffffffff166110b49190615a41565b905060008382116110c557816110c7565b835b602084015190915063ffffffff165b818110156111e75760008682815481106110f2576110f2615a15565b6000918252602080832090910154808352600690915260409091205490915060ff1661114a576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006002828154811061115f5761115f615a15565b60009182526020909120600590910201805490915064010000000090046001600160a01b03161580156111a65750600481015460408701516001600160801b039182169116115b156111d25760018101546001600160a01b0316606087015260048101546001600160801b031660408701525b505080806111df90615a59565b9150506110d6565b5063ffffffff818116602085810191825260008c81526007909152604090819020865181549351928801517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009094169015157fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000ff161761010092909416918202939093177fffffffffffffffffffffff00000000000000000000000000000000ffffffffff16650100000000006001600160801b03909316929092029190911782556060850151600190920180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b03909316929092179091558490036113f657606083015160008a8152600660205260409020805460ff191660011790558915801561133357506000547201000000000000000000000000000000000000900460ff165b1561138e576001546001600160a01b031661134e818a612a4c565b88546001600160a01b03909116640100000000027fffffffffffffffff0000000000000000000000000000000000000000ffffffff9091161788556113f4565b6113bb6001600160a01b038216156113a657816113b5565b60018901546001600160a01b03165b89612a4c565b87547fffffffffffffffff0000000000000000000000000000000000000000ffffffff166401000000006001600160a01b038316021788555b505b505050505050505050565b600080600054600160801b900460ff16600281111561142257611422615713565b146114405760405163067fe19560e41b815260040160405180910390fd5b6000805260066020527f54cdd369e4e8a8515e52ca72ec816c2101831ad1f18bf44102ed171459c9b4f85460ff166114a4576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001600160a01b031660026000815481106114c3576114c3615a15565b600091825260209091206005909102015464010000000090046001600160a01b0316146114f15760016114f4565b60025b6000805467ffffffffffffffff421668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff82168117835592935083927fffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffff167fffffffffffffffffffffffffffffff000000000000000000ffffffffffffffff90911617600160801b83600281111561159857611598615713565b0217905560028111156115ad576115ad615713565b6040517f5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da6090600090a290565b600560205281600052604060002081815481106115f557600080fd5b90600052602060002001600091509150505481565b905090565b61161c838383600161199f565b505050565b600061160a61163260f46014615a41565b3660011981013560f01c9003013560601c90565b6000818152600760209081526040808320600590925282208054825461167790610100900463ffffffff1682615a73565b95945050505050565b606061160a60586020612a8e565b611696611a1f565b60006002600d5460ff1660028111156116b1576116b1615713565b036116d557506001600160a01b0381166000908152600b6020526040902054611744565b6001600d5460ff1660028111156116ee576116ee615713565b0361171257506001600160a01b038116600090815260036020526040902054611744565b6040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0382166000908152600c602052604090205460ff16611822576001600160a01b0382166000908152600c60205260409020805460ff191660011790556117a160c0600119369081013560f01c9003013560601c90565b6040517f7eee288d0000000000000000000000000000000000000000000000000000000081526001600160a01b038481166004830152602482018490529190911690637eee288d90604401600060405180830381600087803b15801561180657600080fd5b505af115801561181a573d6000803e3d6000fd5b505050505050565b8060000361185c576040517f17bfe5f700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0382166000908152600b6020908152604080832083905560039091528120553660011981013560f01c900360c0013560601c6040517ff3fef3a30000000000000000000000000000000000000000000000000000000081526001600160a01b03848116600483015260248201849052919091169063f3fef3a390604401600060405180830381600087803b1580156118fa57600080fd5b505af115801561190e573d6000803e3d6000fd5b505050506000826001600160a01b03168260405160006040518083038185875af1925050503d806000811461195f576040519150601f19603f3d011682016040523d82523d6000602084013e611964565b606091505b505090508061161c576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c900360f4013560601c3314806119d757506119c2611621565b6001600160a01b0316336001600160a01b0316145b611a0d576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611a1984848484612ac2565b50505050565b6002600d5460ff166002811115611a3857611a38615713565b1480611a5a57506001600d5460ff166002811115611a5857611a58615713565b145b15611a6157565b6000600d5460ff166002811115611a7a57611a7a615713565b14611ab1576040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c900360ac013560601c6001600160a01b0316635c975abb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b01573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b259190615a8a565b15611b5c576040517f379a7ed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60005468010000000000000000900467ffffffffffffffff1667ffffffffffffffff16600003611bb8576040517fc105260a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60003660011981013560f01c900360ac013560601c6040517f0314d2b30000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b039190911690630314d2b390602401602060405180830381865afa158015611c2c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c509190615a8a565b905080611c89576040517f4851bd9b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c900360ac013560601c6040517f17cf21a90000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b0391909116906317cf21a990602401600060405180830381600087803b158015611cf857600080fd5b505af1925050508015611d09575060015b5060003660011981013560f01c900360ac013560601c6040517f496b9c160000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b03919091169063496b9c1690602401602060405180830381865afa158015611d7e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611da29190615a8a565b90508015611dbc57600d805460ff19166001179055611dca565b600d805460ff191660021790555b600d546040517f9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f91611e019160ff9091169061574a565b60405180910390a15050565b61161c838383600061199f565b611e22613445565b3660011981013560f01c900360f4013560601c3214611e6d576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b3660011981013560f01c90036054013560e01c90565b600080600054600160801b900460ff166002811115611ea657611ea6615713565b14611ec45760405163067fe19560e41b815260040160405180910390fd5b600060028381548110611ed957611ed9615a15565b600091825260208220600590910201805490925063ffffffff90811614611f3f57815460028054909163ffffffff16908110611f1757611f17615a15565b906000526020600020906005020160040160109054906101000a90046001600160801b031690505b6004820154600090611f6a90600160801b900467ffffffffffffffff165b67ffffffffffffffff1690565b611f7e9067ffffffffffffffff1642615a73565b611f94611f5d846001600160801b031660401c90565b67ffffffffffffffff16611fa89190615a41565b905067ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff1611611ff55780611677565b7f000000000000000000000000000000000000000000000000000000000000000095945050505050565b60008061209c836001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690507f00000000000000000000000000000000000000000000000000000000000000008111156120fb576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b642e90edd00062061a806311e1a30060006121168383615abd565b9050670de0b6b3a7640000600061214d827f0000000000000000000000000000000000000000000000000000000000000000615ad1565b9050600061216b612166670de0b6b3a764000086615ad1565b613cc6565b905060006121798484613f18565b905060006121878383613f67565b9050600061219482613f95565b905060006121b3826121ae670de0b6b3a76400008f615ad1565b61417d565b905060006121c18b83613f67565b90506121cd818d615ad1565b9f9e505050505050505050505050505050565b600281815481106121f057600080fd5b60009182526020909120600590910201805460018201546002830154600384015460049094015463ffffffff841695506401000000009093046001600160a01b03908116949216926001600160801b03918216929180821691600160801b90041687565b60006002600d5460ff16600281111561226f5761226f615713565b0361229057506001600160a01b03166000908152600b602052604090205490565b506001600160a01b031660009081526003602052604090205490565b3660011981013560f01c900360f4013560601c3314806122e457506122cf611621565b6001600160a01b0316336001600160a01b0316145b61231a576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61181a8686868686866141b7565b60008054600160801b900460ff16600281111561234757612347615713565b146123655760405163067fe19560e41b815260040160405180910390fd5b600080600080612374866146eb565b9350935093509350600061238a85858585614a44565b905060003660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156123de573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124029190615af0565b9050600189036124cf576001600160a01b0381166352f0f3ad8a846124333660011981013560f01c90036034013590565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815260048101939093526024830191909152604482015260206064820152608481018a905260a4015b6020604051808303816000875af11580156124a5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124c99190615b0d565b506113f6565b600289036124ee576001600160a01b0381166352f0f3ad8a8489612433565b6003890361250d576001600160a01b0381166352f0f3ad8a8487612433565b6004890361264257600061254a6001600160801b0385167f0000000000000000000000000000000000000000000000000000000000000000614ae3565b6009546125579190615a41565b612562906001615a41565b90503660011981013560f01c900360580135811061258f573660011981013560f01c900360580135612591565b805b90506001600160a01b0382166352f0f3ad8b8560405160e084901b7fffffffff000000000000000000000000000000000000000000000000000000001681526004810192909252602482015260c084901b604482015260086064820152608481018b905260a4016020604051808303816000875af1158015612617573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061263b9190615b0d565b50506113f6565b600589036126bd576040517f52f0f3ad000000000000000000000000000000000000000000000000000000008152600481018a9052602481018390523660011981013560f01c900360d4013560c01b604482015260086064820152608481018890526001600160a01b038216906352f0f3ad9060a401612486565b6040517fff137e6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c9003605481013560e01c90601401356060612712611680565b9050909192565b60008160000151826020015183604001518460600151604051602001612758949392919093845260208401929092526040830152606082015260800190565b604051602081830303815290604052805190602001209050919050565b604080518082019091526000808252602082015281516000036127c4576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b606060008060006127f285614b78565b91945092509050600181600181111561280d5761280d615713565b14612844576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b84516128508385615a41565b14612887576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b604080518082019091526000808252602082015281526020019060019003908161289e5790505093506000835b865181101561298c576000806129116040518060400160405280858c600001516128f59190615a73565b8152602001858c6020015161290a9190615a41565b9052614b78565b50915091506040518060400160405280838361292d9190615a41565b8152602001848b602001516129429190615a41565b81525088858151811061295757612957615a15565b602090810291909101015261296d600185615a41565b93506129798183615a41565b6129839084615a41565b925050506128cb565b50845250919392505050565b606060008060006129a885614b78565b9194509250905060008160018111156129c3576129c3615713565b146129fa576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612a048284615a41565b855114612a3d576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61167785602001518484615016565b60028101546001600160a01b038316600090815260036020526040812080546001600160801b0390931692909190612a85908490615a41565b90915550505050565b6040518181523660011981013560f01c90038284820160208401378260208301016000815260208101604052505092915050565b60008054600160801b900460ff166002811115612ae157612ae1615713565b14612aff5760405163067fe19560e41b815260040160405180910390fd5b600060028481548110612b1457612b14615a15565b60009182526020918290206040805160e0810182526005909302909101805463ffffffff811684526001600160a01b0364010000000090910481169484019490945260018101549093169082015260028201546001600160801b03908116606083015260038301546080830181905260049093015480821660a0840152600160801b90041660c082015291508514612bd8576040517f3014033200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60a0810151600083156001600160801b0383161760011b90506000612c6d826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050861580612ca85750612ca57f00000000000000000000000000000000000000000000000000000000000000006002615a41565b81145b8015612cb2575084155b15612ce9576040517fa42637bc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff168015612d0f575086155b15612d46576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000811115612da0576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612dcb7f00000000000000000000000000000000000000000000000000000000000000006001615a41565b8103612ddd57612ddd868885886150ab565b34612de78361201f565b14612e1e576040517f8620aa1900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612e2988611e85565b905067ffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000811690821603612e91576040517f3381d11400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612ebe60017f0000000000000000000000000000000000000000000000000000000000000000615a73565b8303612fd4573660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015612f14573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f389190615af0565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015612f75573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f999190615b0d565b612fcd907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615b26565b9050613067565b612fff60017f0000000000000000000000000000000000000000000000000000000000000000615a73565b830361303a57612fcd7f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615b52565b507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff165b61309b817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615b82565b67ffffffffffffffff166130b68367ffffffffffffffff1690565b67ffffffffffffffff1611156130fd576130fa817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615b82565b91505b6000604083901b421760008a8152608087901b6001600160801b038d1617602052604081209192509060008181526004602052604090205490915060ff1615613172576040517f80497e3b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60016004600083815260200190815260200160002060006101000a81548160ff02191690831515021790555060026040518060e001604052808d63ffffffff16815260200160006001600160a01b03168152602001336001600160a01b03168152602001346001600160801b031681526020018c8152602001886001600160801b03168152602001846001600160801b0316815250908060018154018082558091505060019003906000526020600020906005020160009091909190915060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a8154816001600160a01b0302191690836001600160a01b0316021790555060408201518160010160006101000a8154816001600160a01b0302191690836001600160a01b0316021790555060608201518160020160006101000a8154816001600160801b0302191690836001600160801b031602179055506080820151816003015560a08201518160040160006101000a8154816001600160801b0302191690836001600160801b0316021790555060c08201518160040160106101000a8154816001600160801b0302191690836001600160801b031602179055505050600560008c815260200190815260200160002060016002805490506133699190615a73565b81546001810183556000928352602080842090910191909155338252600b905260408120805434929061339d908490615a41565b90915550503660011981013560f01c900360c0013560601c6001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156133ef57600080fd5b505af1158015613403573d6000803e3d6000fd5b50506040513393508d92508e91507f9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be90600090a4505050505050505050505050565b60005471010000000000000000000000000000000000900460ff1615613497576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61349f61523a565b36146134d7576040517f9824bdab00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000803660011981013560f01c900360ac013560601c6001600160a01b031663d83ef2676040518163ffffffff1660e01b81526004016040805180830381865afa158015613529573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061354d9190615bab565b909250905081613589576040517f6a6bc3b200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820190915282815260200181905260088290556009819055803660011981013560f01c90036058013511613604576040517ff40239db0000000000000000000000000000000000000000000000000000000081523660011981013560f01c90036014013560048201526024015b60405180910390fd5b67ffffffffffffffff3660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561365d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906136819190615af0565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156136be573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906136e29190615b0d565b111561371a576040517fb4e1243300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006137517f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615ad1565b905060003660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156137a5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906137c99190615af0565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613806573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061382a9190615b0d565b67ffffffffffffffff166138657f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff1690565b67ffffffffffffffff166138799190615a41565b90506000613887838361524f565b905067ffffffffffffffff8111156138cb576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff161115613943576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805160e08101825263ffffffff808252600060208084018281523660011981013560f01c90038035606090811c8789018181526001600160801b0334818116948b0194855260149095013560808b01908152600160a08c0181815242841660c08e019081526002805493840181558c529c517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace600590930292830180549a5191909d167fffffffffffffffff000000000000000000000000000000000000000000000000909a16999099176401000000006001600160a01b039a8b160217909b5592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5acf840180547fffffffffffffffffffffffff000000000000000000000000000000000000000016919098161790965592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad0820180547fffffffffffffffffffffffffffffffff000000000000000000000000000000001691851691909117905593517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad185015595519651968116600160801b9790911696909602959095177f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad2909101558154710100000000000000000000000000000000007fffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffff909116178255918152600b909152918220805491929091613b84908490615a41565b90915550503660011981013560f01c900360c0013560601c6001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b158015613bd657600080fd5b505af1158015613bea573d6000803e3d6000fd5b5050600080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000164267ffffffffffffffff1617905550613c2d9150611e6f9050565b63ffffffff163660011981013560f01c900360ac013560601c6001600160a01b0316633c9f397c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613c83573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613ca79190615bcf565b600a805460ff191663ffffffff92909216929092141790555050505050565b6001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b1760008213613d1c57631615e6386000526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f218311670de0b6b3a764000002158202613f5557637c5f487d6000526004601cfd5b50670de0b6b3a7640000919091020490565b600081600019048311820215613f855763bac65e5b6000526004601cfd5b50670de0b6b3a764000091020490565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d78213613fc357919050565b680755bf798b4a1bf1e58212613fe15763a37bfec96000526004601cfd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b60006141ae670de0b6b3a76400008361419586613cc6565b61419f9190615bf5565b6141a99190615cb1565b613f95565b90505b92915050565b60008054600160801b900460ff1660028111156141d6576141d6615713565b146141f45760405163067fe19560e41b815260040160405180910390fd5b60006002878154811061420957614209615a15565b6000918252602082206005919091020160048101549092506001600160801b0316908715821760011b905061425f7f00000000000000000000000000000000000000000000000000000000000000006001615a41565b6142d9826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1614614313576040517f5f53dd9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008089156143de576143667f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000615a73565b6001901b61437c846001600160801b0316615266565b6001600160801b031661438f9190615cfb565b156143c3576143ba6143ab60016001600160801b038716615d0f565b865463ffffffff1660006152ec565b600301546143d4565b3660011981013560f01c9003607801355b91508490506143ff565b600385015491506143fc6143ab6001600160801b0386166001615d2f565b90505b600882901b60088a8a604051614416929190615a05565b6040518091039020901b14614457576040517f696550ff00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006144628c6153b5565b90506000614471836003015490565b6040517fe14ced320000000000000000000000000000000000000000000000000000000081523660011981013560f01c90036098013560601c9063e14ced32906144c7908f908f908f908f908a90600401615d9a565b6020604051808303816000875af11580156144e6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061450a9190615b0d565b600485015491149150600090600290614593906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b61460d896001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6146179190615dd4565b6146219190615df7565b60ff161590508115158103614662576040517ffb4e40dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b875464010000000090046001600160a01b0316156146ac576040517f9071e6af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505085547fffffffffffffffff0000000000000000000000000000000000000000ffffffff163364010000000002179095555050505050505050505050565b600080600080600085905060006002828154811061470b5761470b615a15565b600091825260209091206004600590920201908101549091507f0000000000000000000000000000000000000000000000000000000000000000906147c0906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116147fa576040517fb34b5c2200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000815b60048301547f00000000000000000000000000000000000000000000000000000000000000009061489f906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16925082111561491457825463ffffffff166148de7f00000000000000000000000000000000000000000000000000000000000000006001615a41565b83036148e8578391505b600281815481106148fb576148fb615a15565b90600052602060002090600502019350809450506147fe565b600481810154908401546001600160801b0391821691166000816001600160801b031661495961494d856001600160801b031660011c90565b6001600160801b031690565b6001600160801b031614905080156149f257600061497f836001600160801b0316615266565b6001600160801b031611156149cf5760006149af6149a760016001600160801b038616615d0f565b8960016152ec565b6003810154600490910154909c506001600160801b03169a506149d59050565b6008549a505b600386015460048701549099506001600160801b03169750614a36565b6000614a0b6149a76001600160801b0385166001615d2f565b6003808901546004808b015492840154930154909e506001600160801b039182169d50919b50169850505b505050505050509193509193565b60006001600160801b03841615614a9f5760408051602081018790526001600160801b038087169282019290925260608101859052908316608082015260a00160405160208183030381529060405280519060200120611677565b8282604051602001614ac49291909182526001600160801b0316602082015260400190565b6040516020818303038152906040528051906020012095945050505050565b600080614b57847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690508083036001841b600180831b0386831b17039250505092915050565b60008060008360000151600003614bbb576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f8111614be057600060016000945094509450505061500f565b60b78111614cf6576000614bf5608083615a73565b905080876000015111614c34576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff00000000000000000000000000000000000000000000000000000000000000169082148015614cac57507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b15614ce3576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b506001955093506000925061500f915050565b60bf8111614e54576000614d0b60b783615a73565b905080876000015111614d4a576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614dac576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614df4576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614dfe8184615a41565b895111614e37576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614e42836001615a41565b975095506000945061500f9350505050565b60f78111614eb9576000614e6960c083615a73565b905080876000015111614ea8576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60019550935084925061500f915050565b6000614ec660f783615a73565b905080876000015111614f05576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614f67576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614faf576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614fb98184615a41565b895111614ff2576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614ffd836001615a41565b975095506001945061500f9350505050565b9193909250565b60608167ffffffffffffffff8111156150315761503161597b565b6040519080825280601f01601f19166020018201604052801561505b576020820181803683370190505b50905081156150a45760006150708486615a41565b90506020820160005b84811015615091578281015182820152602001615079565b848111156150a0576000858301525b5050505b9392505050565b60006150c16001600160801b0384166001615d2f565b905060006150d1828660016152ec565b9050600086901a838061519b575061510a60027f0000000000000000000000000000000000000000000000000000000000000000615cfb565b600483015460029061518c906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6151969190615df7565b60ff16145b156151f35760ff8116600114806151b5575060ff81166002145b6151ee576040517ff40239db000000000000000000000000000000000000000000000000000000008152600481018890526024016135fb565b615231565b60ff811615615231576040517ff40239db000000000000000000000000000000000000000000000000000000008152600481018890526024016135fb565b50505050505050565b60006152446153e4565b61160a906006615a41565b60008183101561525f57816141ae565b5090919050565b6000806152da837e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b600160ff919091161b90920392915050565b6000808261532c576153276001600160801b0386167f00000000000000000000000000000000000000000000000000000000000000006153f2565b61533e565b61533e856001600160801b0316615531565b90506002848154811061535357615353615a15565b906000526020600020906005020191505b60048201546001600160801b038281169116146153ad57815460028054909163ffffffff1690811061539857615398615a15565b90600052602060002090600502019150615364565b509392505050565b60008060008060006153c6866146eb565b93509350935093506153da84848484614a44565b9695505050505050565b600061160a60f46028615a41565b60008161546f846001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116154855763b34b5c226000526004601cfd5b61548e83615531565b90508161550b826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116141b1576141ae615521836001615a41565b6001600160801b038316906155bd565b600081196001830116816155ac827e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169390931c8015179392505050565b600080615631847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050808303600180821b0385821b179250505092915050565b60008083601f84011261565f57600080fd5b50813567ffffffffffffffff81111561567757600080fd5b60208301915083602082850101111561568f57600080fd5b9250929050565b600080600083850360a08112156156ac57600080fd5b60808112156156ba57600080fd5b50839250608084013567ffffffffffffffff8111156156d857600080fd5b6156e48682870161564d565b9497909650939450505050565b6000806040838503121561570457600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b6003811061574757634e487b7160e01b600052602160045260246000fd5b50565b6020810161575783615729565b91905290565b6001600160a01b038116811461574757600080fd5b60006020828403121561578457600080fd5b81356150a48161575d565b6000806000606084860312156157a457600080fd5b505081359360208301359350604090920135919050565b6000815180845260005b818110156157e1576020818501810151868301820152016157c5565b818111156157f3576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b6020815260006141ae60208301846157bb565b60006020828403121561584b57600080fd5b5035919050565b801515811461574757600080fd5b6000806000806080858703121561587657600080fd5b843593506020850135925060408501359150606085013561589681615852565b939692955090935050565b6000602082840312156158b357600080fd5b81356001600160801b03811681146150a457600080fd5b600080600080600080608087890312156158e357600080fd5b8635955060208701356158f581615852565b9450604087013567ffffffffffffffff8082111561591257600080fd5b61591e8a838b0161564d565b9096509450606089013591508082111561593757600080fd5b5061594489828a0161564d565b979a9699509497509295939492505050565b63ffffffff8416815282602082015260606040820152600061167760608301846157bb565b634e487b7160e01b600052604160045260246000fd5b6000608082840312156159a357600080fd5b6040516080810181811067ffffffffffffffff821117156159d457634e487b7160e01b600052604160045260246000fd5b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b8183823760009101908152919050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60008219821115615a5457615a54615a2b565b500190565b60006000198203615a6c57615a6c615a2b565b5060010190565b600082821015615a8557615a85615a2b565b500390565b600060208284031215615a9c57600080fd5b81516150a481615852565b634e487b7160e01b600052601260045260246000fd5b600082615acc57615acc615aa7565b500490565b6000816000190483118215151615615aeb57615aeb615a2b565b500290565b600060208284031215615b0257600080fd5b81516150a48161575d565b600060208284031215615b1f57600080fd5b5051919050565b600067ffffffffffffffff808316818516808303821115615b4957615b49615a2b565b01949350505050565b600067ffffffffffffffff80831681851681830481118215151615615b7957615b79615a2b565b02949350505050565b600067ffffffffffffffff83811690831681811015615ba357615ba3615a2b565b039392505050565b60008060408385031215615bbe57600080fd5b505080516020909101519092909150565b600060208284031215615be157600080fd5b815163ffffffff811681146150a457600080fd5b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615615c3657615c36615a2b565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615615c7157615c71615a2b565b60008712925087820587128484161615615c8d57615c8d615a2b565b87850587128184161615615ca357615ca3615a2b565b505050929093029392505050565b600082615cc057615cc0615aa7565b60001983147f800000000000000000000000000000000000000000000000000000000000000083141615615cf657615cf6615a2b565b500590565b600082615d0a57615d0a615aa7565b500690565b60006001600160801b0383811690831681811015615ba357615ba3615a2b565b60006001600160801b03808316818516808303821115615b4957615b49615a2b565b8183528181602085013750600060208284010152600060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b606081526000615dae606083018789615d51565b8281036020840152615dc1818688615d51565b9150508260408301529695505050505050565b600060ff821660ff841680821015615dee57615dee615a2b565b90039392505050565b600060ff831680615e0a57615e0a615aa7565b8060ff8416069150509291505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0b\x088\x03\x80b\0b\x08\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01\xE7V[\x80b\0\0D`\x01`~b\0\x02\x81V[`\xFF\x16\x81`\0\x01Q\x11\x15b\0\0lW`@Qc;\xEF\xF1\x99`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x19\x81` \x01Q\x14\x80b\0\0\x93WP\x80Q` \x82\x01Qb\0\0\x90\x90`\x01b\0\x02\xA7V[\x10\x15[\x15b\0\0\xB2W`@Qc\xE6,\xCF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x81` \x01Q\x10\x15b\0\0\xD9W`@Qc\xE6,\xCF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0b\0\0\xFE\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16b\0\x01\xC7` \x1Bb\0\x0C`\x17` \x1CV[b\0\x01\x14\x90`\x01`\x01`@\x1B\x03\x16`\x02b\0\x02\xC2V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01?W`@Qc#]\xFB+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\0\x01b\x82``\x01Q`\x01`\x01`@\x1B\x03\x16b\0\x01\xC7` \x1Bb\0\x0C`\x17` \x1CV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15b\0\x01\x95W`@Qc#]\xFB+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Q`\x80R` \x81\x01Q`\xA0R`@\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\xE0R``\x90\x91\x01Q\x16`\xC0RPb\0\x02\xE4V[\x90V[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x01\xE2W`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15b\0\x01\xFAW`\0\x80\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02+WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01Rb\0\x02L`@\x84\x01b\0\x01\xCAV[`@\x82\x01Rb\0\x02_``\x84\x01b\0\x01\xCAV[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\0\x02\x9EWb\0\x02\x9Eb\0\x02kV[\x90\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15b\0\x02\xBDWb\0\x02\xBDb\0\x02kV[P\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\0\x02\xDFWb\0\x02\xDFb\0\x02kV[P\x02\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa^&b\0\x03\xE2`\09`\0\x81\x81a\x070\x01R\x81\x81a/\x9F\x01R\x81\x81a0\n\x01R\x81\x81a0=\x01R\x81\x81a7!\x01Ra89\x01R`\0\x81\x81a\n\x8C\x01R\x81\x81a\x0E\xF3\x01R\x81\x81a\x1F\xB5\x01R\x81\x81a\x1F\xF7\x01R\x81\x81a.6\x01R\x81\x81a0m\x01R\x81\x81a0\xCC\x01Ra8\xD6\x01R`\0\x81\x81a\n\xBF\x01R\x81\x81a%&\x01R\x81\x81a,\x7F\x01R\x81\x81a-\xA5\x01R\x81\x81a/\xDB\x01R\x81\x81aC!\x01R\x81\x81aG&\x01R\x81\x81aH\x05\x01R\x81\x81aH\xB8\x01R\x81\x81aP\xE6\x01RaS\x03\x01R`\0\x81\x81a\x0Bf\x01R\x81\x81a \xA3\x01R\x81\x81a!)\x01R\x81\x81a-H\x01R\x81\x81a.\x9A\x01R\x81\x81aB9\x01RaCB\x01Ra^&`\0\xF3\xFE`\x80`@R`\x046\x10a\x03UW`\x005`\xE0\x1C\x80cp\x87*\xA5\x11a\x01\xBBW\x80c\xC0\xD8\xBBt\x11a\0\xF7W\x80c\xDA\xBD9m\x11a\0\x95W\x80c\xF8\xF4?\xF6\x11a\0oW\x80c\xF8\xF4?\xF6\x14a\x0B\x13W\x80c\xFA$\xF7C\x14a\x0B3W\x80c\xFA1Z\xA9\x14a\x0BWW\x80c\xFE+\xBE\xB2\x14a\x0B\x8AW`\0\x80\xFD[\x80c\xDA\xBD9m\x14a\n}W\x80c\xEC^c\x08\x14a\n\xB0W\x80c\xEF\xF0\xF5\x92\x14a\n\xE3W`\0\x80\xFD[\x80c\xCF\t\xE0\xD0\x11a\0\xD1W\x80c\xCF\t\xE0\xD0\x14a\t\xFAW\x80c\xD5\xD4M\x80\x14a\n\x1BW\x80c\xD6\xAE<\xD5\x14a\n;W\x80c\xD8\xCC\x1A<\x14a\n]W`\0\x80\xFD[\x80c\xC0\xD8\xBBt\x14a\t9W\x80c\xC3\x95\xE1\xCA\x14a\tfW\x80c\xC6\xF00\x8C\x14a\t\x86W`\0\x80\xFD[\x80c\x8DE\n\x95\x11a\x01dW\x80c\xA8\xE4\xFB\x90\x11a\x01>W\x80c\xA8\xE4\xFB\x90\x14a\x08\xA5W\x80c\xBB\xDC\x02\xDB\x14a\x08\xCAW\x80c\xBC\xEF;U\x14a\x08\xF7W\x80c\xBD\x8D\xA9V\x14a\t\x19W`\0\x80\xFD[\x80c\x8DE\n\x95\x14a\x07\xE3W\x80c\x99s^2\x14a\x07\xC1W\x80c\xA4E\xEC\xE6\x14a\x08\x05W`\0\x80\xFD[\x80c\x81)\xFC\x1C\x11a\x01\x95W\x80c\x81)\xFC\x1C\x14a\x07\xA4W\x80c\x89\x80\xE0\xCC\x14a\x07\xACW\x80c\x8B\x85\x90+\x14a\x07\xC1W`\0\x80\xFD[\x80cp\x87*\xA5\x14a\x07gW\x80cxk\x84K\x14a\x07|W\x80c{\x0F\n\xDC\x14a\x07\x91W`\0\x80\xFD[\x80c>:\xC9\x12\x11a\x02\x95W\x80cZ_\xA2\xD9\x11a\x023W\x80c`\xE2td\x11a\x02\rW\x80c`\xE2td\x14a\x06\xDFW\x80ccaPm\x14a\x06\xFFW\x80ckg\x16\xC0\x14a\x07!W\x80co\x03D\t\x14a\x07TW`\0\x80\xFD[\x80cZ_\xA2\xD9\x14a\x06\x85W\x80c\\\x0C\xBA3\x14a\x06\xA5W\x80c`\x9D34\x14a\x06\xCAW`\0\x80\xFD[\x80cR\x9Dj\x8C\x11a\x02oW\x80cR\x9Dj\x8C\x14a\x05\xC4W\x80cSM\xB0\xE2\x14a\x05\xF1W\x80cT\xFDMP\x14a\x06\x06W\x80cW\xDA\x95\x0E\x14a\x06UW`\0\x80\xFD[\x80c>:\xC9\x12\x14a\x05\\W\x80c?\xC8\xCE\xF3\x14a\x05\x8CW\x80cG'w\xC6\x14a\x05\xB1W`\0\x80\xFD[\x80c%\xFC*\xCE\x11a\x03\x02W\x80c0\xDB\xE5p\x11a\x02\xDCW\x80c0\xDB\xE5p\x14a\x04\xC3W\x80c7\x8D\xD4\x8C\x14a\x04\xFBW\x80c7\xB1\xB2)\x14a\x05\x15W\x80c:v\x84c\x14a\x057W`\0\x80\xFD[\x80c%\xFC*\xCE\x14a\x04oW\x80c(\x10\xE1\xD6\x14a\x04\x8EW\x80c*\xD6\x9A\xEB\x14a\x04\xA3W`\0\x80\xFD[\x80c \r.\xD2\x11a\x033W\x80c \r.\xD2\x14a\x03\xE7W\x80c\"*\xBFE\x14a\x04\x15W\x80c%\x0Ei\xBD\x14a\x04UW`\0\x80\xFD[\x80c\x01\x93Q0\x14a\x03ZW\x80c\x03\xC2\x92M\x14a\x03|W\x80c\x19\xEF\xFE\xB4\x14a\x03\x9CW[`\0\x80\xFD[4\x80\x15a\x03fW`\0\x80\xFD[Pa\x03za\x03u6`\x04aV\x96V[a\x0B\xBAV[\0[4\x80\x15a\x03\x88W`\0\x80\xFD[Pa\x03za\x03\x976`\x04aV\xF1V[a\x0EyV[4\x80\x15a\x03\xA8W`\0\x80\xFD[P`\0Ta\x03\xC9\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xF3W`\0\x80\xFD[P`\0Ta\x04\x08\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x03\xDE\x91\x90aWJV[4\x80\x15a\x04!W`\0\x80\xFD[Pa\x04Ea\x0406`\x04aWrV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xDEV[4\x80\x15a\x04aW`\0\x80\xFD[P`\nTa\x04E\x90`\xFF\x16\x81V[4\x80\x15a\x04{W`\0\x80\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x03\xDEV[4\x80\x15a\x04\x9AW`\0\x80\xFD[Pa\x04\x08a\x14\x01V[4\x80\x15a\x04\xAFW`\0\x80\xFD[Pa\x04\x80a\x04\xBE6`\x04aV\xF1V[a\x15\xD9V[4\x80\x15a\x04\xCFW`\0\x80\xFD[P`\x01Ta\x04\xE3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xDEV[4\x80\x15a\x05\x07W`\0\x80\xFD[P`\rTa\x04\x08\x90`\xFF\x16\x81V[4\x80\x15a\x05!W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x035``\x1Ca\x04\xE3V[4\x80\x15a\x05CW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Ca\x04\xE3V[4\x80\x15a\x05hW`\0\x80\xFD[P`\0Ta\x04E\x90r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x05\x98W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Ca\x04\xE3V[a\x03za\x05\xBF6`\x04aW\x8FV[a\x16\x0FV[4\x80\x15a\x05\xD0W`\0\x80\xFD[Pa\x04\x80a\x05\xDF6`\x04aWrV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\xFDW`\0\x80\xFD[Pa\x04\xE3a\x16!V[4\x80\x15a\x06\x12W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F2.2.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03\xDE\x91\x90aX&V[4\x80\x15a\x06aW`\0\x80\xFD[P`\x08T`\tTa\x06p\x91\x90\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\xDEV[4\x80\x15a\x06\x91W`\0\x80\xFD[Pa\x04\x80a\x06\xA06`\x04aX9V[a\x16FV[4\x80\x15a\x06\xB1W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Ca\x04\xE3V[4\x80\x15a\x06\xD6W`\0\x80\xFD[Pa\x06Ha\x16\x80V[4\x80\x15a\x06\xEBW`\0\x80\xFD[Pa\x03za\x06\xFA6`\x04aWrV[a\x16\x8EV[4\x80\x15a\x07\x0BW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015a\x04\x80V[4\x80\x15a\x07-W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xC9V[a\x03za\x07b6`\x04aX`V[a\x19\x9FV[4\x80\x15a\x07sW`\0\x80\xFD[P`\tTa\x04\x80V[4\x80\x15a\x07\x88W`\0\x80\xFD[Pa\x03za\x1A\x1FV[a\x03za\x07\x9F6`\x04aW\x8FV[a\x1E\rV[a\x03za\x1E\x1AV[4\x80\x15a\x07\xB8W`\0\x80\xFD[P`\x02Ta\x04\x80V[4\x80\x15a\x07\xCDW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015a\x04\x80V[4\x80\x15a\x07\xEFW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`x\x015a\x04\x80V[4\x80\x15a\x08\x11W`\0\x80\xFD[Pa\x08ga\x08 6`\x04aX9V[`\x07` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x82\x16\x91a\x01\0\x81\x04c\xFF\xFF\xFF\xFF\x16\x91e\x01\0\0\0\0\0\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q\x94\x15\x15\x85Rc\xFF\xFF\xFF\xFF\x90\x93\x16` \x85\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01a\x03\xDEV[4\x80\x15a\x08\xB1W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1Ca\x04\xE3V[4\x80\x15a\x08\xD6W`\0\x80\xFD[P`@Q6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x81R` \x01a\x03\xDEV[4\x80\x15a\t\x03W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015a\x04\x80V[4\x80\x15a\t%W`\0\x80\xFD[Pa\x03\xC9a\t46`\x04aX9V[a\x1E\x85V[4\x80\x15a\tEW`\0\x80\xFD[Pa\x04\x80a\tT6`\x04aWrV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\trW`\0\x80\xFD[Pa\x04\x80a\t\x816`\x04aX\xA1V[a \x1FV[4\x80\x15a\t\x92W`\0\x80\xFD[Pa\t\xA6a\t\xA16`\x04aX9V[a!\xE0V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x98\x16\x88R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16` \x89\x01R\x95\x90\x94\x16\x94\x86\x01\x94\x90\x94R`\x01`\x01`\x80\x1B\x03\x91\x82\x16``\x86\x01R`\x80\x85\x01R\x91\x82\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\xDEV[4\x80\x15a\n\x06W`\0\x80\xFD[P`\0Ta\x03\xC9\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\n'W`\0\x80\xFD[Pa\x04\x80a\n66`\x04aWrV[a\"TV[4\x80\x15a\nGW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xD4\x015a\x04\x80V[4\x80\x15a\niW`\0\x80\xFD[Pa\x03za\nx6`\x04aX\xCAV[a\"\xACV[4\x80\x15a\n\x89W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xC9V[4\x80\x15a\n\xBCW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x80V[4\x80\x15a\n\xEFW`\0\x80\xFD[Pa\x04Ea\n\xFE6`\x04aX9V[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\x1FW`\0\x80\xFD[Pa\x03za\x0B.6`\x04aW\x8FV[a#(V[4\x80\x15a\x0B?W`\0\x80\xFD[Pa\x0BHa&\xEFV[`@Qa\x03\xDE\x93\x92\x91\x90aYVV[4\x80\x15a\x0BcW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x80V[4\x80\x15a\x0B\x96W`\0\x80\xFD[Pa\x04Ea\x0B\xA56`\x04aX9V[`\x06` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0B\xD9Wa\x0B\xD9aW\x13V[\x14a\x0B\xF7W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x0CJW`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cc6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015\x90V[\x90V[a\x0Cza\x0Cu6\x86\x90\x03\x86\x01\x86aY\x91V[a'\x19V[\x14a\x0C\xB1W`@Q\x7F\x9C\xC0\x0B[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82``\x015\x82\x82`@Qa\x0C\xC6\x92\x91\x90aZ\x05V[`@Q\x80\x91\x03\x90 \x14a\r\x05W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\rNa\rI\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa'u\x92PPPV[a'\xE2V[\x90P`\0a\ru\x82`\x08\x81Q\x81\x10a\rhWa\rhaZ\x15V[` \x02` \x01\x01Qa)\x98V[\x90P` \x81Q\x11\x15a\r\xB3W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x81\x01Q\x82Q\x90\x91\x03`\x03\x1B\x1C6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015\x81\x03a\x0E\nW`@Q\x7F\xB8\xED\x880\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UPP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UPPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0E\x98Wa\x0E\x98aW\x13V[\x14a\x0E\xB6W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a\x0E\xCBWa\x0E\xCBaZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x90P`\0a\x0E\xE6\x84a\x1E\x85V[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x10\x15a\x0FOW`@Q\x7F\xF2D\x0BS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x06` R`@\x90 T`\xFF\x16\x15a\x0F\x98W`@Q\x7F\xF1\xA9E\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x05` R`@\x90 \x80T\x80\x15\x80\x15a\x0F\xB5WP\x85\x15\x15[\x15a\x10\x18W\x83Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0\x81\x15a\x0F\xDBW\x81a\x0F\xEAV[`\x01\x86\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x90Pa\x0F\xF6\x81\x87a*LV[PPP`\0\x94\x85RPP`\x06` RPP`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0\x86\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x82\x04c\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x93\x81\x01\x93\x90\x93R`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01Ra\x10\x9CW`\x01`\x01`\x80\x1B\x03`@\x82\x01R`\x01\x81R`\0\x86\x90\x03a\x10\x9CW\x81\x95P[`\0\x86\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a\x10\xB4\x91\x90aZAV[\x90P`\0\x83\x82\x11a\x10\xC5W\x81a\x10\xC7V[\x83[` \x84\x01Q\x90\x91Pc\xFF\xFF\xFF\xFF\x16[\x81\x81\x10\x15a\x11\xE7W`\0\x86\x82\x81T\x81\x10a\x10\xF2Wa\x10\xF2aZ\x15V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83R`\x06\x90\x91R`@\x90\x91 T\x90\x91P`\xFF\x16a\x11JW`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x82\x81T\x81\x10a\x11_Wa\x11_aZ\x15V[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T\x90\x91Pd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x11\xA6WP`\x04\x81\x01T`@\x87\x01Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11[\x15a\x11\xD2W`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x87\x01R`\x04\x81\x01T`\x01`\x01`\x80\x1B\x03\x16`@\x87\x01R[PP\x80\x80a\x11\xDF\x90aZYV[\x91PPa\x10\xD6V[Pc\xFF\xFF\xFF\xFF\x81\x81\x16` \x85\x81\x01\x91\x82R`\0\x8C\x81R`\x07\x90\x91R`@\x90\x81\x90 \x86Q\x81T\x93Q\x92\x88\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x94\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\x16\x17a\x01\0\x92\x90\x94\x16\x91\x82\x02\x93\x90\x93\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x82U``\x85\x01Q`\x01\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x84\x90\x03a\x13\xF6W``\x83\x01Q`\0\x8A\x81R`\x06` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x89\x15\x80\x15a\x133WP`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16[\x15a\x13\x8EW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x13N\x81\x8Aa*LV[\x88T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x88Ua\x13\xF4V[a\x13\xBB`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x13\xA6W\x81a\x13\xB5V[`\x01\x89\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x89a*LV[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x83\x16\x02\x17\x88U[P[PPPPPPPPPV[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14\"Wa\x14\"aW\x13V[\x14a\x14@W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80R`\x06` R\x7FT\xCD\xD3i\xE4\xE8\xA8Q^R\xCAr\xEC\x81l!\x01\x83\x1A\xD1\xF1\x8B\xF4A\x02\xED\x17\x14Y\xC9\xB4\xF8T`\xFF\x16a\x14\xA4W`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x02`\0\x81T\x81\x10a\x14\xC3Wa\x14\xC3aZ\x15V[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\xF1W`\x01a\x14\xF4V[`\x02[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x17\x83U\x92\x93P\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17`\x01`\x80\x1B\x83`\x02\x81\x11\x15a\x15\x98Wa\x15\x98aW\x13V[\x02\x17\x90U`\x02\x81\x11\x15a\x15\xADWa\x15\xADaW\x13V[`@Q\x7F^\x18o\t\xB9\xC94\x91\xF1N'~\xEA\x7F\xAA]\xE6\xA2\xD4\xBD\xA7Zy\xAFz6\x84\xFB\xFBB\xDA`\x90`\0\x90\xA2\x90V[`\x05` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15\xF5W`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[\x90P\x90V[a\x16\x1C\x83\x83\x83`\x01a\x19\x9FV[PPPV[`\0a\x16\na\x162`\xF4`\x14aZAV[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`\0\x81\x81R`\x07` \x90\x81R`@\x80\x83 `\x05\x90\x92R\x82 \x80T\x82Ta\x16w\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aZsV[\x95\x94PPPPPV[``a\x16\n`X` a*\x8EV[a\x16\x96a\x1A\x1FV[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\xB1Wa\x16\xB1aW\x13V[\x03a\x16\xD5WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x90 Ta\x17DV[`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\xEEWa\x16\xEEaW\x13V[\x03a\x17\x12WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 Ta\x17DV[`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x18\"W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x17\xA1`\xC0`\x01\x196\x90\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`@Q\x7F~\xEE(\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c~\xEE(\x8D\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x06W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x1AW=`\0\x80>=`\0\xFD[PPPPPPV[\x80`\0\x03a\x18\\W`@Q\x7F\x17\xBF\xE5\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x83\x90U`\x03\x90\x91R\x81 U6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xFAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x0EW=`\0\x80>=`\0\xFD[PPPP`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19dV[``\x91P[PP\x90P\x80a\x16\x1CW`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a\x19\xD7WPa\x19\xC2a\x16!V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x1A\rW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1A\x19\x84\x84\x84\x84a*\xC2V[PPPPV[`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\x1A8Wa\x1A8aW\x13V[\x14\x80a\x1AZWP`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a\x1AXWa\x1AXaW\x13V[\x14[\x15a\x1AaWV[`\0`\rT`\xFF\x16`\x02\x81\x11\x15a\x1AzWa\x1AzaW\x13V[\x14a\x1A\xB1W`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B%\x91\x90aZ\x8AV[\x15a\x1B\\W`@Q\x7F7\x9A~\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B\xB8W`@Q\x7F\xC1\x05&\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x03\x14\xD2\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x03\x14\xD2\xB3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CP\x91\x90aZ\x8AV[\x90P\x80a\x1C\x89W`@Q\x7FHQ\xBD\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x17\xCF!\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x17\xCF!\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xF8W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x1D\tWP`\x01[P`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA2\x91\x90aZ\x8AV[\x90P\x80\x15a\x1D\xBCW`\r\x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1D\xCAV[`\r\x80T`\xFF\x19\x16`\x02\x17\x90U[`\rT`@Q\x7F\x99\x08\xEA\xAC\x06E\xDF\x9D\x07\x04\xD0j\xDC\x9E\x073|\x95\x1D\xE2\xF0k_(6\x15\x1DH\xD5\xE4r/\x91a\x1E\x01\x91`\xFF\x90\x91\x16\x90aWJV[`@Q\x80\x91\x03\x90\xA1PPV[a\x16\x1C\x83\x83\x83`\0a\x19\x9FV[a\x1E\"a4EV[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C2\x14a\x1EmW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x90V[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1E\xA6Wa\x1E\xA6aW\x13V[\x14a\x1E\xC4W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a\x1E\xD9Wa\x1E\xD9aZ\x15V[`\0\x91\x82R` \x82 `\x05\x90\x91\x02\x01\x80T\x90\x92Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\x1F?W\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10a\x1F\x17Wa\x1F\x17aZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01`\x04\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90P[`\x04\x82\x01T`\0\x90a\x1Fj\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x1F~\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaZsV[a\x1F\x94a\x1F]\x84`\x01`\x01`\x80\x1B\x03\x16`@\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\xA8\x91\x90aZAV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1F\xF5W\x80a\x16wV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`\0\x80a \x9C\x83`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a \xFBW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d.\x90\xED\xD0\0b\x06\x1A\x80c\x11\xE1\xA3\0`\0a!\x16\x83\x83aZ\xBDV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0a!M\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZ\xD1V[\x90P`\0a!ka!fg\r\xE0\xB6\xB3\xA7d\0\0\x86aZ\xD1V[a<\xC6V[\x90P`\0a!y\x84\x84a?\x18V[\x90P`\0a!\x87\x83\x83a?gV[\x90P`\0a!\x94\x82a?\x95V[\x90P`\0a!\xB3\x82a!\xAEg\r\xE0\xB6\xB3\xA7d\0\0\x8FaZ\xD1V[aA}V[\x90P`\0a!\xC1\x8B\x83a?gV[\x90Pa!\xCD\x81\x8DaZ\xD1V[\x9F\x9EPPPPPPPPPPPPPPPV[`\x02\x81\x81T\x81\x10a!\xF0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01Tc\xFF\xFF\xFF\xFF\x84\x16\x95Pd\x01\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x92\x16\x92`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x87V[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\"oWa\"oaW\x13V[\x03a\"\x90WP`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0B` R`@\x90 T\x90V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a\"\xE4WPa\"\xCFa\x16!V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a#\x1AW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\x1A\x86\x86\x86\x86\x86\x86aA\xB7V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a#GWa#GaW\x13V[\x14a#eW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80a#t\x86aF\xEBV[\x93P\x93P\x93P\x93P`\0a#\x8A\x85\x85\x85\x85aJDV[\x90P`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x02\x91\x90aZ\xF0V[\x90P`\x01\x89\x03a$\xCFW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84a$36`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R`D\x82\x01R` `d\x82\x01R`\x84\x81\x01\x8A\x90R`\xA4\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a$\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xC9\x91\x90a[\rV[Pa\x13\xF6V[`\x02\x89\x03a$\xEEW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x89a$3V[`\x03\x89\x03a%\rW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x87a$3V[`\x04\x89\x03a&BW`\0a%J`\x01`\x01`\x80\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aJ\xE3V[`\tTa%W\x91\x90aZAV[a%b\x90`\x01aZAV[\x90P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015\x81\x10a%\x8FW6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015a%\x91V[\x80[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16cR\xF0\xF3\xAD\x8B\x85`@Q`\xE0\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`\xC0\x84\x90\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x8B\x90R`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&;\x91\x90a[\rV[PPa\x13\xF6V[`\x05\x89\x03a&\xBDW`@Q\x7FR\xF0\xF3\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xD4\x015`\xC0\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cR\xF0\xF3\xAD\x90`\xA4\x01a$\x86V[`@Q\x7F\xFF\x13~e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x81\x015`\xE0\x1C\x90`\x14\x015``a'\x12a\x16\x80V[\x90P\x90\x91\x92V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a'X\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03a'\xC4W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0a'\xF2\x85aKxV[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15a(\rWa(\raW\x13V[\x14a(DW`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa(P\x83\x85aZAV[\x14a(\x87W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a(\x9EW\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15a)\x8CW`\0\x80a)\x11`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01Qa(\xF5\x91\x90aZsV[\x81R` \x01\x85\x8C` \x01Qa)\n\x91\x90aZAV[\x90RaKxV[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a)-\x91\x90aZAV[\x81R` \x01\x84\x8B` \x01Qa)B\x91\x90aZAV[\x81RP\x88\x85\x81Q\x81\x10a)WWa)WaZ\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra)m`\x01\x85aZAV[\x93Pa)y\x81\x83aZAV[a)\x83\x90\x84aZAV[\x92PPPa(\xCBV[P\x84RP\x91\x93\x92PPPV[```\0\x80`\0a)\xA8\x85aKxV[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15a)\xC3Wa)\xC3aW\x13V[\x14a)\xFAW`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\x04\x82\x84aZAV[\x85Q\x14a*=W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16w\x85` \x01Q\x84\x84aP\x16V[`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x91\x90a*\x85\x90\x84\x90aZAV[\x90\x91UPPPPV[`@Q\x81\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x82\x84\x82\x01` \x84\x017\x82` \x83\x01\x01`\0\x81R` \x81\x01`@RPP\x92\x91PPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a*\xE1Wa*\xE1aW\x13V[\x14a*\xFFW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x84\x81T\x81\x10a+\x14Wa+\x14aZ\x15V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x81\x16\x84R`\x01`\x01`\xA0\x1B\x03d\x01\0\0\0\0\x90\x91\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01\x81\x01T\x90\x93\x16\x90\x82\x01R`\x02\x82\x01T`\x01`\x01`\x80\x1B\x03\x90\x81\x16``\x83\x01R`\x03\x83\x01T`\x80\x83\x01\x81\x90R`\x04\x90\x93\x01T\x80\x82\x16`\xA0\x84\x01R`\x01`\x80\x1B\x90\x04\x16`\xC0\x82\x01R\x91P\x85\x14a+\xD8W`@Q\x7F0\x14\x032\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0\x81\x01Q`\0\x83\x15`\x01`\x01`\x80\x1B\x03\x83\x16\x17`\x01\x1B\x90P`\0a,m\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x86\x15\x80a,\xA8WPa,\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02aZAV[\x81\x14[\x80\x15a,\xB2WP\x84\x15[\x15a,\xE9W`@Q\x7F\xA4&7\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x80\x15a-\x0FWP\x86\x15[\x15a-FW`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a-\xA0W`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aZAV[\x81\x03a-\xDDWa-\xDD\x86\x88\x85\x88aP\xABV[4a-\xE7\x83a \x1FV[\x14a.\x1EW`@Q\x7F\x86 \xAA\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.)\x88a\x1E\x85V[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a.\x91W`@Q\x7F3\x81\xD1\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.\xBE`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZsV[\x83\x03a/\xD4W6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/8\x91\x90aZ\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x99\x91\x90a[\rV[a/\xCD\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[&V[\x90Pa0gV[a/\xFF`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZsV[\x83\x03a0:Wa/\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02a[RV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[a0\x9B\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\x82V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a0\xB6\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a0\xFDWa0\xFA\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\x82V[\x91P[`\0`@\x83\x90\x1BB\x17`\0\x8A\x81R`\x80\x87\x90\x1B`\x01`\x01`\x80\x1B\x03\x8D\x16\x17` R`@\x81 \x91\x92P\x90`\0\x81\x81R`\x04` R`@\x90 T\x90\x91P`\xFF\x16\x15a1rW`@Q\x7F\x80I~;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x04`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x02`@Q\x80`\xE0\x01`@R\x80\x8Dc\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x014`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\x80\x1B\x03\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`\x05`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\x01`\x02\x80T\x90Pa3i\x91\x90aZsV[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x91\x01\x91\x90\x91U3\x82R`\x0B\x90R`@\x81 \x80T4\x92\x90a3\x9D\x90\x84\x90aZAV[\x90\x91UPP6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a3\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\x03W=`\0\x80>=`\0\xFD[PP`@Q3\x93P\x8D\x92P\x8E\x91P\x7F\x9B2Et\x0E\xC3\xB1U\t\x8AU\xBE\x84\x95zM\xA1>\xAF\x7F\x14\xA8\xBCoS\x12l\x0B\x93P\xF2\xBE\x90`\0\x90\xA4PPPPPPPPPPPPV[`\0Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a4\x97W`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a4\x9FaR:V[6\x14a4\xD7W`@Q\x7F\x98$\xBD\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x806`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD8>\xF2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5M\x91\x90a[\xABV[\x90\x92P\x90P\x81a5\x89W`@Q\x7Fjk\xC3\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x01\x81\x90R`\x08\x82\x90U`\t\x81\x90U\x806`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015\x11a6\x04W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x81\x91\x90aZ\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE2\x91\x90a[\rV[\x11\x15a7\x1AW`@Q\x7F\xB4\xE1$3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a7Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aZ\xD1V[\x90P`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xC9\x91\x90aZ\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8*\x91\x90a[\rV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a8e\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a8y\x91\x90aZAV[\x90P`\0a8\x87\x83\x83aROV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xCBW`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a9CW`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x80\x82R`\0` \x80\x84\x01\x82\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x805``\x90\x81\x1C\x87\x89\x01\x81\x81R`\x01`\x01`\x80\x1B\x034\x81\x81\x16\x94\x8B\x01\x94\x85R`\x14\x90\x95\x015`\x80\x8B\x01\x90\x81R`\x01`\xA0\x8C\x01\x81\x81RB\x84\x16`\xC0\x8E\x01\x90\x81R`\x02\x80T\x93\x84\x01\x81U\x8CR\x9CQ\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE`\x05\x90\x93\x02\x92\x83\x01\x80T\x9AQ\x91\x90\x9D\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x9A\x16\x99\x90\x99\x17d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x9A\x8B\x16\x02\x17\x90\x9BU\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCF\x84\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x98\x16\x17\x90\x96U\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD0\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD1\x85\x01U\x95Q\x96Q\x96\x81\x16`\x01`\x80\x1B\x97\x90\x91\x16\x96\x90\x96\x02\x95\x90\x95\x17\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD2\x90\x91\x01U\x81Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U\x91\x81R`\x0B\x90\x91R\x91\x82 \x80T\x91\x92\x90\x91a;\x84\x90\x84\x90aZAV[\x90\x91UPP6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a;\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a;\xEAW=`\0\x80>=`\0\xFD[PP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPa<-\x91Pa\x1Eo\x90PV[c\xFF\xFF\xFF\xFF\x166`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c<\x9F9|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xA7\x91\x90a[\xCFV[`\n\x80T`\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x92\x90\x92\x14\x17\x90UPPPPPV[`\x01`\x01`\x80\x1B\x03\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17`\0\x82\x13a=\x1CWc\x16\x15\xE68`\0R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[`\0x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x83\x11g\r\xE0\xB6\xB3\xA7d\0\0\x02\x15\x82\x02a?UWc|_H}`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x02\x15a?\x85Wc\xBA\xC6^[`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13a?\xC3W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a?\xE1Wc\xA3{\xFE\xC9`\0R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0aA\xAEg\r\xE0\xB6\xB3\xA7d\0\0\x83aA\x95\x86a<\xC6V[aA\x9F\x91\x90a[\xF5V[aA\xA9\x91\x90a\\\xB1V[a?\x95V[\x90P[\x92\x91PPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15aA\xD6WaA\xD6aW\x13V[\x14aA\xF4W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x87\x81T\x81\x10aB\tWaB\taZ\x15V[`\0\x91\x82R` \x82 `\x05\x91\x90\x91\x02\x01`\x04\x81\x01T\x90\x92P`\x01`\x01`\x80\x1B\x03\x16\x90\x87\x15\x82\x17`\x01\x1B\x90PaB_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aZAV[aB\xD9\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x14aC\x13W`@Q\x7F_S\xDD\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x89\x15aC\xDEWaCf\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZsV[`\x01\x90\x1BaC|\x84`\x01`\x01`\x80\x1B\x03\x16aRfV[`\x01`\x01`\x80\x1B\x03\x16aC\x8F\x91\x90a\\\xFBV[\x15aC\xC3WaC\xBAaC\xAB`\x01`\x01`\x01`\x80\x1B\x03\x87\x16a]\x0FV[\x86Tc\xFF\xFF\xFF\xFF\x16`\0aR\xECV[`\x03\x01TaC\xD4V[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`x\x015[\x91P\x84\x90PaC\xFFV[`\x03\x85\x01T\x91PaC\xFCaC\xAB`\x01`\x01`\x80\x1B\x03\x86\x16`\x01a]/V[\x90P[`\x08\x82\x90\x1B`\x08\x8A\x8A`@QaD\x16\x92\x91\x90aZ\x05V[`@Q\x80\x91\x03\x90 \x90\x1B\x14aDWW`@Q\x7FieP\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0aDb\x8CaS\xB5V[\x90P`\0aDq\x83`\x03\x01T\x90V[`@Q\x7F\xE1L\xED2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C\x90c\xE1L\xED2\x90aD\xC7\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8A\x90`\x04\x01a]\x9AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aD\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\n\x91\x90a[\rV[`\x04\x85\x01T\x91\x14\x91P`\0\x90`\x02\x90aE\x93\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aF\r\x89`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aF\x17\x91\x90a]\xD4V[aF!\x91\x90a]\xF7V[`\xFF\x16\x15\x90P\x81\x15\x15\x81\x03aFbW`@Q\x7F\xFBN@\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15aF\xACW`@Q\x7F\x90q\xE6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x163d\x01\0\0\0\0\x02\x17\x90\x95UPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x85\x90P`\0`\x02\x82\x81T\x81\x10aG\x0BWaG\x0BaZ\x15V[`\0\x91\x82R` \x90\x91 `\x04`\x05\x90\x92\x02\x01\x90\x81\x01T\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aG\xC0\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aG\xFAW`@Q\x7F\xB3K\\\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[`\x04\x83\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aH\x9F\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x92P\x82\x11\x15aI\x14W\x82Tc\xFF\xFF\xFF\xFF\x16aH\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aZAV[\x83\x03aH\xE8W\x83\x91P[`\x02\x81\x81T\x81\x10aH\xFBWaH\xFBaZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x93P\x80\x94PPaG\xFEV[`\x04\x81\x81\x01T\x90\x84\x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16`\0\x81`\x01`\x01`\x80\x1B\x03\x16aIYaIM\x85`\x01`\x01`\x80\x1B\x03\x16`\x01\x1C\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x14\x90P\x80\x15aI\xF2W`\0aI\x7F\x83`\x01`\x01`\x80\x1B\x03\x16aRfV[`\x01`\x01`\x80\x1B\x03\x16\x11\x15aI\xCFW`\0aI\xAFaI\xA7`\x01`\x01`\x01`\x80\x1B\x03\x86\x16a]\x0FV[\x89`\x01aR\xECV[`\x03\x81\x01T`\x04\x90\x91\x01T\x90\x9CP`\x01`\x01`\x80\x1B\x03\x16\x9APaI\xD5\x90PV[`\x08T\x9AP[`\x03\x86\x01T`\x04\x87\x01T\x90\x99P`\x01`\x01`\x80\x1B\x03\x16\x97PaJ6V[`\0aJ\x0BaI\xA7`\x01`\x01`\x80\x1B\x03\x85\x16`\x01a]/V[`\x03\x80\x89\x01T`\x04\x80\x8B\x01T\x92\x84\x01T\x93\x01T\x90\x9EP`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x9DP\x91\x9BP\x16\x98PP[PPPPPPP\x91\x93P\x91\x93V[`\0`\x01`\x01`\x80\x1B\x03\x84\x16\x15aJ\x9FW`@\x80Q` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R\x90\x83\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x16wV[\x82\x82`@Q` \x01aJ\xC4\x92\x91\x90\x91\x82R`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95\x94PPPPPV[`\0\x80aKW\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[`\0\x80`\0\x83`\0\x01Q`\0\x03aK\xBBW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11aK\xE0W`\0`\x01`\0\x94P\x94P\x94PPPaP\x0FV[`\xB7\x81\x11aL\xF6W`\0aK\xF5`\x80\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aL4W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15aL\xACWP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15aL\xE3W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92PaP\x0F\x91PPV[`\xBF\x81\x11aNTW`\0aM\x0B`\xB7\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aMJW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aM\xACW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aM\xF4W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aM\xFE\x81\x84aZAV[\x89Q\x11aN7W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aNB\x83`\x01aZAV[\x97P\x95P`\0\x94PaP\x0F\x93PPPPV[`\xF7\x81\x11aN\xB9W`\0aNi`\xC0\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aN\xA8W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92PaP\x0F\x91PPV[`\0aN\xC6`\xF7\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aO\x05W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aOgW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aO\xAFW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aO\xB9\x81\x84aZAV[\x89Q\x11aO\xF2W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aO\xFD\x83`\x01aZAV[\x97P\x95P`\x01\x94PaP\x0F\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP1WaP1aY{V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aP[W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15aP\xA4W`\0aPp\x84\x86aZAV[\x90P` \x82\x01`\0[\x84\x81\x10\x15aP\x91W\x82\x81\x01Q\x82\x82\x01R` \x01aPyV[\x84\x81\x11\x15aP\xA0W`\0\x85\x83\x01R[PPP[\x93\x92PPPV[`\0aP\xC1`\x01`\x01`\x80\x1B\x03\x84\x16`\x01a]/V[\x90P`\0aP\xD1\x82\x86`\x01aR\xECV[\x90P`\0\x86\x90\x1A\x83\x80aQ\x9BWPaQ\n`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\\\xFBV[`\x04\x83\x01T`\x02\x90aQ\x8C\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aQ\x96\x91\x90a]\xF7V[`\xFF\x16\x14[\x15aQ\xF3W`\xFF\x81\x16`\x01\x14\x80aQ\xB5WP`\xFF\x81\x16`\x02\x14[aQ\xEEW`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a5\xFBV[aR1V[`\xFF\x81\x16\x15aR1W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a5\xFBV[PPPPPPPV[`\0aRDaS\xE4V[a\x16\n\x90`\x06aZAV[`\0\x81\x83\x10\x15aR_W\x81aA\xAEV[P\x90\x91\x90PV[`\0\x80aR\xDA\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01`\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x80\x82aS,WaS'`\x01`\x01`\x80\x1B\x03\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aS\xF2V[aS>V[aS>\x85`\x01`\x01`\x80\x1B\x03\x16aU1V[\x90P`\x02\x84\x81T\x81\x10aSSWaSSaZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x91P[`\x04\x82\x01T`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x91\x16\x14aS\xADW\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10aS\x98WaS\x98aZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x91PaSdV[P\x93\x92PPPV[`\0\x80`\0\x80`\0aS\xC6\x86aF\xEBV[\x93P\x93P\x93P\x93PaS\xDA\x84\x84\x84\x84aJDV[\x96\x95PPPPPPV[`\0a\x16\n`\xF4`(aZAV[`\0\x81aTo\x84`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aT\x85Wc\xB3K\\\"`\0R`\x04`\x1C\xFD[aT\x8E\x83aU1V[\x90P\x81aU\x0B\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aA\xB1WaA\xAEaU!\x83`\x01aZAV[`\x01`\x01`\x80\x1B\x03\x83\x16\x90aU\xBDV[`\0\x81\x19`\x01\x83\x01\x16\x81aU\xAC\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`\0\x80aV1\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x80\x82\x1B\x03\x85\x82\x1B\x17\x92PPP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aV_W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aVwW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aV\x8FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15aV\xACW`\0\x80\xFD[`\x80\x81\x12\x15aV\xBAW`\0\x80\xFD[P\x83\x92P`\x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xD8W`\0\x80\xFD[aV\xE4\x86\x82\x87\x01aVMV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15aW\x04W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aWGWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01aWW\x83aW)V[\x91\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aWGW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aW\x84W`\0\x80\xFD[\x815aP\xA4\x81aW]V[`\0\x80`\0``\x84\x86\x03\x12\x15aW\xA4W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aW\xE1W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aW\xC5V[\x81\x81\x11\x15aW\xF3W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0aA\xAE` \x83\x01\x84aW\xBBV[`\0` \x82\x84\x03\x12\x15aXKW`\0\x80\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14aWGW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aXvW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aX\x96\x81aXRV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aX\xB3W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aP\xA4W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aX\xE3W`\0\x80\xFD[\x865\x95P` \x87\x015aX\xF5\x81aXRV[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aY\x12W`\0\x80\xFD[aY\x1E\x8A\x83\x8B\x01aVMV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aY7W`\0\x80\xFD[PaYD\x89\x82\x8A\x01aVMV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x16w``\x83\x01\x84aW\xBBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15aY\xA3W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aY\xD4WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aZTWaZTaZ+V[P\x01\x90V[`\0`\0\x19\x82\x03aZlWaZlaZ+V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aZ\x85WaZ\x85aZ+V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aZ\x9CW`\0\x80\xFD[\x81QaP\xA4\x81aXRV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aZ\xCCWaZ\xCCaZ\xA7V[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aZ\xEBWaZ\xEBaZ+V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15a[\x02W`\0\x80\xFD[\x81QaP\xA4\x81aW]V[`\0` \x82\x84\x03\x12\x15a[\x1FW`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a[IWa[IaZ+V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a[yWa[yaZ+V[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a[\xA3Wa[\xA3aZ+V[\x03\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a[\xBEW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a[\xE1W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14aP\xA4W`\0\x80\xFD[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\\6Wa\\6aZ+V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\\qWa\\qaZ+V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\\\x8DWa\\\x8DaZ+V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\\\xA3Wa\\\xA3aZ+V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82a\\\xC0Wa\\\xC0aZ\xA7V[`\0\x19\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\\\xF6Wa\\\xF6aZ+V[P\x05\x90V[`\0\x82a]\nWa]\naZ\xA7V[P\x06\x90V[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a[\xA3Wa[\xA3aZ+V[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a[IWa[IaZ+V[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0a]\xAE``\x83\x01\x87\x89a]QV[\x82\x81\x03` \x84\x01Ra]\xC1\x81\x86\x88a]QV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a]\xEEWa]\xEEaZ+V[\x90\x03\x93\x92PPPV[`\0`\xFF\x83\x16\x80a^\nWa^\naZ\xA7V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106103555760003560e01c806370872aa5116101bb578063c0d8bb74116100f7578063dabd396d11610095578063f8f43ff61161006f578063f8f43ff614610b13578063fa24f74314610b33578063fa315aa914610b57578063fe2bbeb214610b8a57600080fd5b8063dabd396d14610a7d578063ec5e630814610ab0578063eff0f59214610ae357600080fd5b8063cf09e0d0116100d1578063cf09e0d0146109fa578063d5d44d8014610a1b578063d6ae3cd514610a3b578063d8cc1a3c14610a5d57600080fd5b8063c0d8bb7414610939578063c395e1ca14610966578063c6f0308c1461098657600080fd5b80638d450a9511610164578063a8e4fb901161013e578063a8e4fb90146108a5578063bbdc02db146108ca578063bcef3b55146108f7578063bd8da9561461091957600080fd5b80638d450a95146107e357806399735e32146107c1578063a445ece61461080557600080fd5b80638129fc1c116101955780638129fc1c146107a45780638980e0cc146107ac5780638b85902b146107c157600080fd5b806370872aa514610767578063786b844b1461077c5780637b0f0adc1461079157600080fd5b80633e3ac912116102955780635a5fa2d91161023357806360e274641161020d57806360e27464146106df5780636361506d146106ff5780636b6716c0146107215780636f0344091461075457600080fd5b80635a5fa2d9146106855780635c0cba33146106a5578063609d3334146106ca57600080fd5b8063529d6a8c1161026f578063529d6a8c146105c4578063534db0e2146105f157806354fd4d501461060657806357da950e1461065557600080fd5b80633e3ac9121461055c5780633fc8cef31461058c578063472777c6146105b157600080fd5b806325fc2ace1161030257806330dbe570116102dc57806330dbe570146104c3578063378dd48c146104fb57806337b1b229146105155780633a7684631461053757600080fd5b806325fc2ace1461046f5780632810e1d61461048e5780632ad69aeb146104a357600080fd5b8063200d2ed211610333578063200d2ed2146103e7578063222abf4514610415578063250e69bd1461045557600080fd5b8063019351301461035a57806303c2924d1461037c57806319effeb41461039c575b600080fd5b34801561036657600080fd5b5061037a610375366004615696565b610bba565b005b34801561038857600080fd5b5061037a6103973660046156f1565b610e79565b3480156103a857600080fd5b506000546103c99068010000000000000000900467ffffffffffffffff1681565b60405167ffffffffffffffff90911681526020015b60405180910390f35b3480156103f357600080fd5b5060005461040890600160801b900460ff1681565b6040516103de919061574a565b34801561042157600080fd5b50610445610430366004615772565b600c6020526000908152604090205460ff1681565b60405190151581526020016103de565b34801561046157600080fd5b50600a546104459060ff1681565b34801561047b57600080fd5b506008545b6040519081526020016103de565b34801561049a57600080fd5b50610408611401565b3480156104af57600080fd5b506104806104be3660046156f1565b6115d9565b3480156104cf57600080fd5b506001546104e3906001600160a01b031681565b6040516001600160a01b0390911681526020016103de565b34801561050757600080fd5b50600d546104089060ff1681565b34801561052157600080fd5b503660011981013560f01c90033560601c6104e3565b34801561054357600080fd5b503660011981013560f01c90036098013560601c6104e3565b34801561056857600080fd5b50600054610445907201000000000000000000000000000000000000900460ff1681565b34801561059857600080fd5b503660011981013560f01c900360c0013560601c6104e3565b61037a6105bf36600461578f565b61160f565b3480156105d057600080fd5b506104806105df366004615772565b60036020526000908152604090205481565b3480156105fd57600080fd5b506104e3611621565b34801561061257600080fd5b5060408051808201909152600581527f322e322e3000000000000000000000000000000000000000000000000000000060208201525b6040516103de9190615826565b34801561066157600080fd5b50600854600954610670919082565b604080519283526020830191909152016103de565b34801561069157600080fd5b506104806106a0366004615839565b611646565b3480156106b157600080fd5b503660011981013560f01c900360ac013560601c6104e3565b3480156106d657600080fd5b50610648611680565b3480156106eb57600080fd5b5061037a6106fa366004615772565b61168e565b34801561070b57600080fd5b503660011981013560f01c900360340135610480565b34801561072d57600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103c9565b61037a610762366004615860565b61199f565b34801561077357600080fd5b50600954610480565b34801561078857600080fd5b5061037a611a1f565b61037a61079f36600461578f565b611e0d565b61037a611e1a565b3480156107b857600080fd5b50600254610480565b3480156107cd57600080fd5b503660011981013560f01c900360580135610480565b3480156107ef57600080fd5b503660011981013560f01c900360780135610480565b34801561081157600080fd5b50610867610820366004615839565b6007602052600090815260409020805460019091015460ff821691610100810463ffffffff1691650100000000009091046001600160801b0316906001600160a01b031684565b60408051941515855263ffffffff90931660208501526001600160801b03909116918301919091526001600160a01b031660608201526080016103de565b3480156108b157600080fd5b503660011981013560f01c900360f4013560601c6104e3565b3480156108d657600080fd5b506040513660011981013560f01c90036054013560e01c81526020016103de565b34801561090357600080fd5b503660011981013560f01c900360140135610480565b34801561092557600080fd5b506103c9610934366004615839565b611e85565b34801561094557600080fd5b50610480610954366004615772565b600b6020526000908152604090205481565b34801561097257600080fd5b506104806109813660046158a1565b61201f565b34801561099257600080fd5b506109a66109a1366004615839565b6121e0565b6040805163ffffffff90981688526001600160a01b03968716602089015295909416948601949094526001600160801b039182166060860152608085015291821660a08401521660c082015260e0016103de565b348015610a0657600080fd5b506000546103c99067ffffffffffffffff1681565b348015610a2757600080fd5b50610480610a36366004615772565b612254565b348015610a4757600080fd5b503660011981013560f01c900360d40135610480565b348015610a6957600080fd5b5061037a610a783660046158ca565b6122ac565b348015610a8957600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103c9565b348015610abc57600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610480565b348015610aef57600080fd5b50610445610afe366004615839565b60046020526000908152604090205460ff1681565b348015610b1f57600080fd5b5061037a610b2e36600461578f565b612328565b348015610b3f57600080fd5b50610b486126ef565b6040516103de93929190615956565b348015610b6357600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610480565b348015610b9657600080fd5b50610445610ba5366004615839565b60066020526000908152604090205460ff1681565b60008054600160801b900460ff166002811115610bd957610bd9615713565b14610bf75760405163067fe19560e41b815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff1615610c4a576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610c633660011981013560f01c90036014013590565b90565b610c7a610c7536869003860186615991565b612719565b14610cb1576040517f9cc00b5b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b82606001358282604051610cc6929190615a05565b604051809103902014610d05576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610d4e610d4984848080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061277592505050565b6127e2565b90506000610d7582600881518110610d6857610d68615a15565b6020026020010151612998565b9050602081511115610db3576040517fd81d583b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602081810151825190910360031b1c3660011981013560f01c9003605801358103610e0a576040517fb8ed883000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050600180547fffffffffffffffffffffffff000000000000000000000000000000000000000016331790555050600080547fffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffff1672010000000000000000000000000000000000001790555050565b60008054600160801b900460ff166002811115610e9857610e98615713565b14610eb65760405163067fe19560e41b815260040160405180910390fd5b600060028381548110610ecb57610ecb615a15565b906000526020600020906005020190506000610ee684611e85565b905067ffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081169082161015610f4f576040517ff2440b5300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008481526006602052604090205460ff1615610f98576040517ff1a9458100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000848152600560205260409020805480158015610fb557508515155b1561101857835464010000000090046001600160a01b031660008115610fdb5781610fea565b60018601546001600160a01b03165b9050610ff68187612a4c565b505050600094855250506006602052505060409020805460ff19166001179055565b6000868152600760209081526040918290208251608081018452815460ff81161515808352610100820463ffffffff16948301949094526501000000000090046001600160801b031693810193909352600101546001600160a01b0316606083015261109c576001600160801b03604082015260018152600086900361109c578195505b600086826020015163ffffffff166110b49190615a41565b905060008382116110c557816110c7565b835b602084015190915063ffffffff165b818110156111e75760008682815481106110f2576110f2615a15565b6000918252602080832090910154808352600690915260409091205490915060ff1661114a576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006002828154811061115f5761115f615a15565b60009182526020909120600590910201805490915064010000000090046001600160a01b03161580156111a65750600481015460408701516001600160801b039182169116115b156111d25760018101546001600160a01b0316606087015260048101546001600160801b031660408701525b505080806111df90615a59565b9150506110d6565b5063ffffffff818116602085810191825260008c81526007909152604090819020865181549351928801517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009094169015157fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000ff161761010092909416918202939093177fffffffffffffffffffffff00000000000000000000000000000000ffffffffff16650100000000006001600160801b03909316929092029190911782556060850151600190920180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b03909316929092179091558490036113f657606083015160008a8152600660205260409020805460ff191660011790558915801561133357506000547201000000000000000000000000000000000000900460ff165b1561138e576001546001600160a01b031661134e818a612a4c565b88546001600160a01b03909116640100000000027fffffffffffffffff0000000000000000000000000000000000000000ffffffff9091161788556113f4565b6113bb6001600160a01b038216156113a657816113b5565b60018901546001600160a01b03165b89612a4c565b87547fffffffffffffffff0000000000000000000000000000000000000000ffffffff166401000000006001600160a01b038316021788555b505b505050505050505050565b600080600054600160801b900460ff16600281111561142257611422615713565b146114405760405163067fe19560e41b815260040160405180910390fd5b6000805260066020527f54cdd369e4e8a8515e52ca72ec816c2101831ad1f18bf44102ed171459c9b4f85460ff166114a4576040517f9a07664600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001600160a01b031660026000815481106114c3576114c3615a15565b600091825260209091206005909102015464010000000090046001600160a01b0316146114f15760016114f4565b60025b6000805467ffffffffffffffff421668010000000000000000027fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff82168117835592935083927fffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffff167fffffffffffffffffffffffffffffff000000000000000000ffffffffffffffff90911617600160801b83600281111561159857611598615713565b0217905560028111156115ad576115ad615713565b6040517f5e186f09b9c93491f14e277eea7faa5de6a2d4bda75a79af7a3684fbfb42da6090600090a290565b600560205281600052604060002081815481106115f557600080fd5b90600052602060002001600091509150505481565b905090565b61161c838383600161199f565b505050565b600061160a61163260f46014615a41565b3660011981013560f01c9003013560601c90565b6000818152600760209081526040808320600590925282208054825461167790610100900463ffffffff1682615a73565b95945050505050565b606061160a60586020612a8e565b611696611a1f565b60006002600d5460ff1660028111156116b1576116b1615713565b036116d557506001600160a01b0381166000908152600b6020526040902054611744565b6001600d5460ff1660028111156116ee576116ee615713565b0361171257506001600160a01b038116600090815260036020526040902054611744565b6040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0382166000908152600c602052604090205460ff16611822576001600160a01b0382166000908152600c60205260409020805460ff191660011790556117a160c0600119369081013560f01c9003013560601c90565b6040517f7eee288d0000000000000000000000000000000000000000000000000000000081526001600160a01b038481166004830152602482018490529190911690637eee288d90604401600060405180830381600087803b15801561180657600080fd5b505af115801561181a573d6000803e3d6000fd5b505050505050565b8060000361185c576040517f17bfe5f700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0382166000908152600b6020908152604080832083905560039091528120553660011981013560f01c900360c0013560601c6040517ff3fef3a30000000000000000000000000000000000000000000000000000000081526001600160a01b03848116600483015260248201849052919091169063f3fef3a390604401600060405180830381600087803b1580156118fa57600080fd5b505af115801561190e573d6000803e3d6000fd5b505050506000826001600160a01b03168260405160006040518083038185875af1925050503d806000811461195f576040519150601f19603f3d011682016040523d82523d6000602084013e611964565b606091505b505090508061161c576040517f83e6cc6b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c900360f4013560601c3314806119d757506119c2611621565b6001600160a01b0316336001600160a01b0316145b611a0d576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611a1984848484612ac2565b50505050565b6002600d5460ff166002811115611a3857611a38615713565b1480611a5a57506001600d5460ff166002811115611a5857611a58615713565b145b15611a6157565b6000600d5460ff166002811115611a7a57611a7a615713565b14611ab1576040517f078a3df400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c900360ac013560601c6001600160a01b0316635c975abb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b01573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b259190615a8a565b15611b5c576040517f379a7ed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60005468010000000000000000900467ffffffffffffffff1667ffffffffffffffff16600003611bb8576040517fc105260a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60003660011981013560f01c900360ac013560601c6040517f0314d2b30000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b039190911690630314d2b390602401602060405180830381865afa158015611c2c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c509190615a8a565b905080611c89576040517f4851bd9b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c900360ac013560601c6040517f17cf21a90000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b0391909116906317cf21a990602401600060405180830381600087803b158015611cf857600080fd5b505af1925050508015611d09575060015b5060003660011981013560f01c900360ac013560601c6040517f496b9c160000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b03919091169063496b9c1690602401602060405180830381865afa158015611d7e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611da29190615a8a565b90508015611dbc57600d805460ff19166001179055611dca565b600d805460ff191660021790555b600d546040517f9908eaac0645df9d0704d06adc9e07337c951de2f06b5f2836151d48d5e4722f91611e019160ff9091169061574a565b60405180910390a15050565b61161c838383600061199f565b611e22613445565b3660011981013560f01c900360f4013560601c3214611e6d576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b3660011981013560f01c90036054013560e01c90565b600080600054600160801b900460ff166002811115611ea657611ea6615713565b14611ec45760405163067fe19560e41b815260040160405180910390fd5b600060028381548110611ed957611ed9615a15565b600091825260208220600590910201805490925063ffffffff90811614611f3f57815460028054909163ffffffff16908110611f1757611f17615a15565b906000526020600020906005020160040160109054906101000a90046001600160801b031690505b6004820154600090611f6a90600160801b900467ffffffffffffffff165b67ffffffffffffffff1690565b611f7e9067ffffffffffffffff1642615a73565b611f94611f5d846001600160801b031660401c90565b67ffffffffffffffff16611fa89190615a41565b905067ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff1611611ff55780611677565b7f000000000000000000000000000000000000000000000000000000000000000095945050505050565b60008061209c836001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690507f00000000000000000000000000000000000000000000000000000000000000008111156120fb576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b642e90edd00062061a806311e1a30060006121168383615abd565b9050670de0b6b3a7640000600061214d827f0000000000000000000000000000000000000000000000000000000000000000615ad1565b9050600061216b612166670de0b6b3a764000086615ad1565b613cc6565b905060006121798484613f18565b905060006121878383613f67565b9050600061219482613f95565b905060006121b3826121ae670de0b6b3a76400008f615ad1565b61417d565b905060006121c18b83613f67565b90506121cd818d615ad1565b9f9e505050505050505050505050505050565b600281815481106121f057600080fd5b60009182526020909120600590910201805460018201546002830154600384015460049094015463ffffffff841695506401000000009093046001600160a01b03908116949216926001600160801b03918216929180821691600160801b90041687565b60006002600d5460ff16600281111561226f5761226f615713565b0361229057506001600160a01b03166000908152600b602052604090205490565b506001600160a01b031660009081526003602052604090205490565b3660011981013560f01c900360f4013560601c3314806122e457506122cf611621565b6001600160a01b0316336001600160a01b0316145b61231a576040517fd386ef3e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61181a8686868686866141b7565b60008054600160801b900460ff16600281111561234757612347615713565b146123655760405163067fe19560e41b815260040160405180910390fd5b600080600080612374866146eb565b9350935093509350600061238a85858585614a44565b905060003660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156123de573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124029190615af0565b9050600189036124cf576001600160a01b0381166352f0f3ad8a846124333660011981013560f01c90036034013590565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815260048101939093526024830191909152604482015260206064820152608481018a905260a4015b6020604051808303816000875af11580156124a5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124c99190615b0d565b506113f6565b600289036124ee576001600160a01b0381166352f0f3ad8a8489612433565b6003890361250d576001600160a01b0381166352f0f3ad8a8487612433565b6004890361264257600061254a6001600160801b0385167f0000000000000000000000000000000000000000000000000000000000000000614ae3565b6009546125579190615a41565b612562906001615a41565b90503660011981013560f01c900360580135811061258f573660011981013560f01c900360580135612591565b805b90506001600160a01b0382166352f0f3ad8b8560405160e084901b7fffffffff000000000000000000000000000000000000000000000000000000001681526004810192909252602482015260c084901b604482015260086064820152608481018b905260a4016020604051808303816000875af1158015612617573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061263b9190615b0d565b50506113f6565b600589036126bd576040517f52f0f3ad000000000000000000000000000000000000000000000000000000008152600481018a9052602481018390523660011981013560f01c900360d4013560c01b604482015260086064820152608481018890526001600160a01b038216906352f0f3ad9060a401612486565b6040517fff137e6500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3660011981013560f01c9003605481013560e01c90601401356060612712611680565b9050909192565b60008160000151826020015183604001518460600151604051602001612758949392919093845260208401929092526040830152606082015260800190565b604051602081830303815290604052805190602001209050919050565b604080518082019091526000808252602082015281516000036127c4576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b606060008060006127f285614b78565b91945092509050600181600181111561280d5761280d615713565b14612844576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b84516128508385615a41565b14612887576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b604080518082019091526000808252602082015281526020019060019003908161289e5790505093506000835b865181101561298c576000806129116040518060400160405280858c600001516128f59190615a73565b8152602001858c6020015161290a9190615a41565b9052614b78565b50915091506040518060400160405280838361292d9190615a41565b8152602001848b602001516129429190615a41565b81525088858151811061295757612957615a15565b602090810291909101015261296d600185615a41565b93506129798183615a41565b6129839084615a41565b925050506128cb565b50845250919392505050565b606060008060006129a885614b78565b9194509250905060008160018111156129c3576129c3615713565b146129fa576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612a048284615a41565b855114612a3d576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61167785602001518484615016565b60028101546001600160a01b038316600090815260036020526040812080546001600160801b0390931692909190612a85908490615a41565b90915550505050565b6040518181523660011981013560f01c90038284820160208401378260208301016000815260208101604052505092915050565b60008054600160801b900460ff166002811115612ae157612ae1615713565b14612aff5760405163067fe19560e41b815260040160405180910390fd5b600060028481548110612b1457612b14615a15565b60009182526020918290206040805160e0810182526005909302909101805463ffffffff811684526001600160a01b0364010000000090910481169484019490945260018101549093169082015260028201546001600160801b03908116606083015260038301546080830181905260049093015480821660a0840152600160801b90041660c082015291508514612bd8576040517f3014033200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60a0810151600083156001600160801b0383161760011b90506000612c6d826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050861580612ca85750612ca57f00000000000000000000000000000000000000000000000000000000000000006002615a41565b81145b8015612cb2575084155b15612ce9576040517fa42637bc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000547201000000000000000000000000000000000000900460ff168015612d0f575086155b15612d46576040517f0ea2e75200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000811115612da0576040517f56f57b2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612dcb7f00000000000000000000000000000000000000000000000000000000000000006001615a41565b8103612ddd57612ddd868885886150ab565b34612de78361201f565b14612e1e576040517f8620aa1900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612e2988611e85565b905067ffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000811690821603612e91576040517f3381d11400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612ebe60017f0000000000000000000000000000000000000000000000000000000000000000615a73565b8303612fd4573660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa158015612f14573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f389190615af0565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015612f75573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f999190615b0d565b612fcd907f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615b26565b9050613067565b612fff60017f0000000000000000000000000000000000000000000000000000000000000000615a73565b830361303a57612fcd7f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615b52565b507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff165b61309b817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615b82565b67ffffffffffffffff166130b68367ffffffffffffffff1690565b67ffffffffffffffff1611156130fd576130fa817f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff16615b82565b91505b6000604083901b421760008a8152608087901b6001600160801b038d1617602052604081209192509060008181526004602052604090205490915060ff1615613172576040517f80497e3b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60016004600083815260200190815260200160002060006101000a81548160ff02191690831515021790555060026040518060e001604052808d63ffffffff16815260200160006001600160a01b03168152602001336001600160a01b03168152602001346001600160801b031681526020018c8152602001886001600160801b03168152602001846001600160801b0316815250908060018154018082558091505060019003906000526020600020906005020160009091909190915060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a8154816001600160a01b0302191690836001600160a01b0316021790555060408201518160010160006101000a8154816001600160a01b0302191690836001600160a01b0316021790555060608201518160020160006101000a8154816001600160801b0302191690836001600160801b031602179055506080820151816003015560a08201518160040160006101000a8154816001600160801b0302191690836001600160801b0316021790555060c08201518160040160106101000a8154816001600160801b0302191690836001600160801b031602179055505050600560008c815260200190815260200160002060016002805490506133699190615a73565b81546001810183556000928352602080842090910191909155338252600b905260408120805434929061339d908490615a41565b90915550503660011981013560f01c900360c0013560601c6001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156133ef57600080fd5b505af1158015613403573d6000803e3d6000fd5b50506040513393508d92508e91507f9b3245740ec3b155098a55be84957a4da13eaf7f14a8bc6f53126c0b9350f2be90600090a4505050505050505050505050565b60005471010000000000000000000000000000000000900460ff1615613497576040517f0dc149f000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61349f61523a565b36146134d7576040517f9824bdab00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000803660011981013560f01c900360ac013560601c6001600160a01b031663d83ef2676040518163ffffffff1660e01b81526004016040805180830381865afa158015613529573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061354d9190615bab565b909250905081613589576040517f6a6bc3b200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820190915282815260200181905260088290556009819055803660011981013560f01c90036058013511613604576040517ff40239db0000000000000000000000000000000000000000000000000000000081523660011981013560f01c90036014013560048201526024015b60405180910390fd5b67ffffffffffffffff3660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561365d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906136819190615af0565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156136be573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906136e29190615b0d565b111561371a576040517fb4e1243300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006137517f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff166002615ad1565b905060003660011981013560f01c90036098013560601c6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156137a5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906137c99190615af0565b6001600160a01b031663f3f480d96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613806573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061382a9190615b0d565b67ffffffffffffffff166138657f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff1690565b67ffffffffffffffff166138799190615a41565b90506000613887838361524f565b905067ffffffffffffffff8111156138cb576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001667ffffffffffffffff168167ffffffffffffffff161115613943576040517f8d77ecac00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805160e08101825263ffffffff808252600060208084018281523660011981013560f01c90038035606090811c8789018181526001600160801b0334818116948b0194855260149095013560808b01908152600160a08c0181815242841660c08e019081526002805493840181558c529c517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace600590930292830180549a5191909d167fffffffffffffffff000000000000000000000000000000000000000000000000909a16999099176401000000006001600160a01b039a8b160217909b5592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5acf840180547fffffffffffffffffffffffff000000000000000000000000000000000000000016919098161790965592517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad0820180547fffffffffffffffffffffffffffffffff000000000000000000000000000000001691851691909117905593517f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad185015595519651968116600160801b9790911696909602959095177f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ad2909101558154710100000000000000000000000000000000007fffffffffffffffffffffffffffff00ffffffffffffffffffffffffffffffffff909116178255918152600b909152918220805491929091613b84908490615a41565b90915550503660011981013560f01c900360c0013560601c6001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b158015613bd657600080fd5b505af1158015613bea573d6000803e3d6000fd5b5050600080547fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000164267ffffffffffffffff1617905550613c2d9150611e6f9050565b63ffffffff163660011981013560f01c900360ac013560601c6001600160a01b0316633c9f397c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613c83573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613ca79190615bcf565b600a805460ff191663ffffffff92909216929092141790555050505050565b6001600160801b03811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b1760008213613d1c57631615e6386000526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b60007812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f218311670de0b6b3a764000002158202613f5557637c5f487d6000526004601cfd5b50670de0b6b3a7640000919091020490565b600081600019048311820215613f855763bac65e5b6000526004601cfd5b50670de0b6b3a764000091020490565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d78213613fc357919050565b680755bf798b4a1bf1e58212613fe15763a37bfec96000526004601cfd5b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b60006141ae670de0b6b3a76400008361419586613cc6565b61419f9190615bf5565b6141a99190615cb1565b613f95565b90505b92915050565b60008054600160801b900460ff1660028111156141d6576141d6615713565b146141f45760405163067fe19560e41b815260040160405180910390fd5b60006002878154811061420957614209615a15565b6000918252602082206005919091020160048101549092506001600160801b0316908715821760011b905061425f7f00000000000000000000000000000000000000000000000000000000000000006001615a41565b6142d9826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1614614313576040517f5f53dd9800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008089156143de576143667f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000615a73565b6001901b61437c846001600160801b0316615266565b6001600160801b031661438f9190615cfb565b156143c3576143ba6143ab60016001600160801b038716615d0f565b865463ffffffff1660006152ec565b600301546143d4565b3660011981013560f01c9003607801355b91508490506143ff565b600385015491506143fc6143ab6001600160801b0386166001615d2f565b90505b600882901b60088a8a604051614416929190615a05565b6040518091039020901b14614457576040517f696550ff00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006144628c6153b5565b90506000614471836003015490565b6040517fe14ced320000000000000000000000000000000000000000000000000000000081523660011981013560f01c90036098013560601c9063e14ced32906144c7908f908f908f908f908a90600401615d9a565b6020604051808303816000875af11580156144e6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061450a9190615b0d565b600485015491149150600090600290614593906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b61460d896001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6146179190615dd4565b6146219190615df7565b60ff161590508115158103614662576040517ffb4e40dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b875464010000000090046001600160a01b0316156146ac576040517f9071e6af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505085547fffffffffffffffff0000000000000000000000000000000000000000ffffffff163364010000000002179095555050505050505050505050565b600080600080600085905060006002828154811061470b5761470b615a15565b600091825260209091206004600590920201908101549091507f0000000000000000000000000000000000000000000000000000000000000000906147c0906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116147fa576040517fb34b5c2200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000815b60048301547f00000000000000000000000000000000000000000000000000000000000000009061489f906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16925082111561491457825463ffffffff166148de7f00000000000000000000000000000000000000000000000000000000000000006001615a41565b83036148e8578391505b600281815481106148fb576148fb615a15565b90600052602060002090600502019350809450506147fe565b600481810154908401546001600160801b0391821691166000816001600160801b031661495961494d856001600160801b031660011c90565b6001600160801b031690565b6001600160801b031614905080156149f257600061497f836001600160801b0316615266565b6001600160801b031611156149cf5760006149af6149a760016001600160801b038616615d0f565b8960016152ec565b6003810154600490910154909c506001600160801b03169a506149d59050565b6008549a505b600386015460048701549099506001600160801b03169750614a36565b6000614a0b6149a76001600160801b0385166001615d2f565b6003808901546004808b015492840154930154909e506001600160801b039182169d50919b50169850505b505050505050509193509193565b60006001600160801b03841615614a9f5760408051602081018790526001600160801b038087169282019290925260608101859052908316608082015260a00160405160208183030381529060405280519060200120611677565b8282604051602001614ac49291909182526001600160801b0316602082015260400190565b6040516020818303038152906040528051906020012095945050505050565b600080614b57847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff1690508083036001841b600180831b0386831b17039250505092915050565b60008060008360000151600003614bbb576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f8111614be057600060016000945094509450505061500f565b60b78111614cf6576000614bf5608083615a73565b905080876000015111614c34576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff00000000000000000000000000000000000000000000000000000000000000169082148015614cac57507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b15614ce3576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b506001955093506000925061500f915050565b60bf8111614e54576000614d0b60b783615a73565b905080876000015111614d4a576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614dac576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614df4576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614dfe8184615a41565b895111614e37576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614e42836001615a41565b975095506000945061500f9350505050565b60f78111614eb9576000614e6960c083615a73565b905080876000015111614ea8576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60019550935084925061500f915050565b6000614ec660f783615a73565b905080876000015111614f05576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614f67576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614faf576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614fb98184615a41565b895111614ff2576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614ffd836001615a41565b975095506001945061500f9350505050565b9193909250565b60608167ffffffffffffffff8111156150315761503161597b565b6040519080825280601f01601f19166020018201604052801561505b576020820181803683370190505b50905081156150a45760006150708486615a41565b90506020820160005b84811015615091578281015182820152602001615079565b848111156150a0576000858301525b5050505b9392505050565b60006150c16001600160801b0384166001615d2f565b905060006150d1828660016152ec565b9050600086901a838061519b575061510a60027f0000000000000000000000000000000000000000000000000000000000000000615cfb565b600483015460029061518c906001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b6151969190615df7565b60ff16145b156151f35760ff8116600114806151b5575060ff81166002145b6151ee576040517ff40239db000000000000000000000000000000000000000000000000000000008152600481018890526024016135fb565b615231565b60ff811615615231576040517ff40239db000000000000000000000000000000000000000000000000000000008152600481018890526024016135fb565b50505050505050565b60006152446153e4565b61160a906006615a41565b60008183101561525f57816141ae565b5090919050565b6000806152da837e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b600160ff919091161b90920392915050565b6000808261532c576153276001600160801b0386167f00000000000000000000000000000000000000000000000000000000000000006153f2565b61533e565b61533e856001600160801b0316615531565b90506002848154811061535357615353615a15565b906000526020600020906005020191505b60048201546001600160801b038281169116146153ad57815460028054909163ffffffff1690811061539857615398615a15565b90600052602060002090600502019150615364565b509392505050565b60008060008060006153c6866146eb565b93509350935093506153da84848484614a44565b9695505050505050565b600061160a60f46028615a41565b60008161546f846001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116154855763b34b5c226000526004601cfd5b61548e83615531565b90508161550b826001600160801b03167e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff16116141b1576141ae615521836001615a41565b6001600160801b038316906155bd565b600081196001830116816155ac827e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169390931c8015179392505050565b600080615631847e09010a0d15021d0b0e10121619031e080c141c0f111807131b17061a05041f6307c4acdd60e01b67ffffffffffffffff831160061b83811c63ffffffff1060051b1792831c600181901c17600281901c17600481901c17600881901c17601081901c170260fb1c1a1790565b60ff169050808303600180821b0385821b179250505092915050565b60008083601f84011261565f57600080fd5b50813567ffffffffffffffff81111561567757600080fd5b60208301915083602082850101111561568f57600080fd5b9250929050565b600080600083850360a08112156156ac57600080fd5b60808112156156ba57600080fd5b50839250608084013567ffffffffffffffff8111156156d857600080fd5b6156e48682870161564d565b9497909650939450505050565b6000806040838503121561570457600080fd5b50508035926020909101359150565b634e487b7160e01b600052602160045260246000fd5b6003811061574757634e487b7160e01b600052602160045260246000fd5b50565b6020810161575783615729565b91905290565b6001600160a01b038116811461574757600080fd5b60006020828403121561578457600080fd5b81356150a48161575d565b6000806000606084860312156157a457600080fd5b505081359360208301359350604090920135919050565b6000815180845260005b818110156157e1576020818501810151868301820152016157c5565b818111156157f3576000602083870101525b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b6020815260006141ae60208301846157bb565b60006020828403121561584b57600080fd5b5035919050565b801515811461574757600080fd5b6000806000806080858703121561587657600080fd5b843593506020850135925060408501359150606085013561589681615852565b939692955090935050565b6000602082840312156158b357600080fd5b81356001600160801b03811681146150a457600080fd5b600080600080600080608087890312156158e357600080fd5b8635955060208701356158f581615852565b9450604087013567ffffffffffffffff8082111561591257600080fd5b61591e8a838b0161564d565b9096509450606089013591508082111561593757600080fd5b5061594489828a0161564d565b979a9699509497509295939492505050565b63ffffffff8416815282602082015260606040820152600061167760608301846157bb565b634e487b7160e01b600052604160045260246000fd5b6000608082840312156159a357600080fd5b6040516080810181811067ffffffffffffffff821117156159d457634e487b7160e01b600052604160045260246000fd5b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b8183823760009101908152919050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60008219821115615a5457615a54615a2b565b500190565b60006000198203615a6c57615a6c615a2b565b5060010190565b600082821015615a8557615a85615a2b565b500390565b600060208284031215615a9c57600080fd5b81516150a481615852565b634e487b7160e01b600052601260045260246000fd5b600082615acc57615acc615aa7565b500490565b6000816000190483118215151615615aeb57615aeb615a2b565b500290565b600060208284031215615b0257600080fd5b81516150a48161575d565b600060208284031215615b1f57600080fd5b5051919050565b600067ffffffffffffffff808316818516808303821115615b4957615b49615a2b565b01949350505050565b600067ffffffffffffffff80831681851681830481118215151615615b7957615b79615a2b565b02949350505050565b600067ffffffffffffffff83811690831681811015615ba357615ba3615a2b565b039392505050565b60008060408385031215615bbe57600080fd5b505080516020909101519092909150565b600060208284031215615be157600080fd5b815163ffffffff811681146150a457600080fd5b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615615c3657615c36615a2b565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615615c7157615c71615a2b565b60008712925087820587128484161615615c8d57615c8d615a2b565b87850587128184161615615ca357615ca3615a2b565b505050929093029392505050565b600082615cc057615cc0615aa7565b60001983147f800000000000000000000000000000000000000000000000000000000000000083141615615cf657615cf6615a2b565b500590565b600082615d0a57615d0a615aa7565b500690565b60006001600160801b0383811690831681811015615ba357615ba3615a2b565b60006001600160801b03808316818516808303821115615b4957615b49615a2b565b8183528181602085013750600060208284010152600060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b606081526000615dae606083018789615d51565b8281036020840152615dc1818688615d51565b9150508260408301529695505050505050565b600060ff821660ff841680821015615dee57615dee615a2b565b90039392505050565b600060ff831680615e0a57615e0a615aa7565b8060ff8416069150509291505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x03UW`\x005`\xE0\x1C\x80cp\x87*\xA5\x11a\x01\xBBW\x80c\xC0\xD8\xBBt\x11a\0\xF7W\x80c\xDA\xBD9m\x11a\0\x95W\x80c\xF8\xF4?\xF6\x11a\0oW\x80c\xF8\xF4?\xF6\x14a\x0B\x13W\x80c\xFA$\xF7C\x14a\x0B3W\x80c\xFA1Z\xA9\x14a\x0BWW\x80c\xFE+\xBE\xB2\x14a\x0B\x8AW`\0\x80\xFD[\x80c\xDA\xBD9m\x14a\n}W\x80c\xEC^c\x08\x14a\n\xB0W\x80c\xEF\xF0\xF5\x92\x14a\n\xE3W`\0\x80\xFD[\x80c\xCF\t\xE0\xD0\x11a\0\xD1W\x80c\xCF\t\xE0\xD0\x14a\t\xFAW\x80c\xD5\xD4M\x80\x14a\n\x1BW\x80c\xD6\xAE<\xD5\x14a\n;W\x80c\xD8\xCC\x1A<\x14a\n]W`\0\x80\xFD[\x80c\xC0\xD8\xBBt\x14a\t9W\x80c\xC3\x95\xE1\xCA\x14a\tfW\x80c\xC6\xF00\x8C\x14a\t\x86W`\0\x80\xFD[\x80c\x8DE\n\x95\x11a\x01dW\x80c\xA8\xE4\xFB\x90\x11a\x01>W\x80c\xA8\xE4\xFB\x90\x14a\x08\xA5W\x80c\xBB\xDC\x02\xDB\x14a\x08\xCAW\x80c\xBC\xEF;U\x14a\x08\xF7W\x80c\xBD\x8D\xA9V\x14a\t\x19W`\0\x80\xFD[\x80c\x8DE\n\x95\x14a\x07\xE3W\x80c\x99s^2\x14a\x07\xC1W\x80c\xA4E\xEC\xE6\x14a\x08\x05W`\0\x80\xFD[\x80c\x81)\xFC\x1C\x11a\x01\x95W\x80c\x81)\xFC\x1C\x14a\x07\xA4W\x80c\x89\x80\xE0\xCC\x14a\x07\xACW\x80c\x8B\x85\x90+\x14a\x07\xC1W`\0\x80\xFD[\x80cp\x87*\xA5\x14a\x07gW\x80cxk\x84K\x14a\x07|W\x80c{\x0F\n\xDC\x14a\x07\x91W`\0\x80\xFD[\x80c>:\xC9\x12\x11a\x02\x95W\x80cZ_\xA2\xD9\x11a\x023W\x80c`\xE2td\x11a\x02\rW\x80c`\xE2td\x14a\x06\xDFW\x80ccaPm\x14a\x06\xFFW\x80ckg\x16\xC0\x14a\x07!W\x80co\x03D\t\x14a\x07TW`\0\x80\xFD[\x80cZ_\xA2\xD9\x14a\x06\x85W\x80c\\\x0C\xBA3\x14a\x06\xA5W\x80c`\x9D34\x14a\x06\xCAW`\0\x80\xFD[\x80cR\x9Dj\x8C\x11a\x02oW\x80cR\x9Dj\x8C\x14a\x05\xC4W\x80cSM\xB0\xE2\x14a\x05\xF1W\x80cT\xFDMP\x14a\x06\x06W\x80cW\xDA\x95\x0E\x14a\x06UW`\0\x80\xFD[\x80c>:\xC9\x12\x14a\x05\\W\x80c?\xC8\xCE\xF3\x14a\x05\x8CW\x80cG'w\xC6\x14a\x05\xB1W`\0\x80\xFD[\x80c%\xFC*\xCE\x11a\x03\x02W\x80c0\xDB\xE5p\x11a\x02\xDCW\x80c0\xDB\xE5p\x14a\x04\xC3W\x80c7\x8D\xD4\x8C\x14a\x04\xFBW\x80c7\xB1\xB2)\x14a\x05\x15W\x80c:v\x84c\x14a\x057W`\0\x80\xFD[\x80c%\xFC*\xCE\x14a\x04oW\x80c(\x10\xE1\xD6\x14a\x04\x8EW\x80c*\xD6\x9A\xEB\x14a\x04\xA3W`\0\x80\xFD[\x80c \r.\xD2\x11a\x033W\x80c \r.\xD2\x14a\x03\xE7W\x80c\"*\xBFE\x14a\x04\x15W\x80c%\x0Ei\xBD\x14a\x04UW`\0\x80\xFD[\x80c\x01\x93Q0\x14a\x03ZW\x80c\x03\xC2\x92M\x14a\x03|W\x80c\x19\xEF\xFE\xB4\x14a\x03\x9CW[`\0\x80\xFD[4\x80\x15a\x03fW`\0\x80\xFD[Pa\x03za\x03u6`\x04aV\x96V[a\x0B\xBAV[\0[4\x80\x15a\x03\x88W`\0\x80\xFD[Pa\x03za\x03\x976`\x04aV\xF1V[a\x0EyV[4\x80\x15a\x03\xA8W`\0\x80\xFD[P`\0Ta\x03\xC9\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xF3W`\0\x80\xFD[P`\0Ta\x04\x08\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x03\xDE\x91\x90aWJV[4\x80\x15a\x04!W`\0\x80\xFD[Pa\x04Ea\x0406`\x04aWrV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xDEV[4\x80\x15a\x04aW`\0\x80\xFD[P`\nTa\x04E\x90`\xFF\x16\x81V[4\x80\x15a\x04{W`\0\x80\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x03\xDEV[4\x80\x15a\x04\x9AW`\0\x80\xFD[Pa\x04\x08a\x14\x01V[4\x80\x15a\x04\xAFW`\0\x80\xFD[Pa\x04\x80a\x04\xBE6`\x04aV\xF1V[a\x15\xD9V[4\x80\x15a\x04\xCFW`\0\x80\xFD[P`\x01Ta\x04\xE3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xDEV[4\x80\x15a\x05\x07W`\0\x80\xFD[P`\rTa\x04\x08\x90`\xFF\x16\x81V[4\x80\x15a\x05!W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x035``\x1Ca\x04\xE3V[4\x80\x15a\x05CW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1Ca\x04\xE3V[4\x80\x15a\x05hW`\0\x80\xFD[P`\0Ta\x04E\x90r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x05\x98W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1Ca\x04\xE3V[a\x03za\x05\xBF6`\x04aW\x8FV[a\x16\x0FV[4\x80\x15a\x05\xD0W`\0\x80\xFD[Pa\x04\x80a\x05\xDF6`\x04aWrV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\xFDW`\0\x80\xFD[Pa\x04\xE3a\x16!V[4\x80\x15a\x06\x12W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81R\x7F2.2.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x03\xDE\x91\x90aX&V[4\x80\x15a\x06aW`\0\x80\xFD[P`\x08T`\tTa\x06p\x91\x90\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\xDEV[4\x80\x15a\x06\x91W`\0\x80\xFD[Pa\x04\x80a\x06\xA06`\x04aX9V[a\x16FV[4\x80\x15a\x06\xB1W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1Ca\x04\xE3V[4\x80\x15a\x06\xD6W`\0\x80\xFD[Pa\x06Ha\x16\x80V[4\x80\x15a\x06\xEBW`\0\x80\xFD[Pa\x03za\x06\xFA6`\x04aWrV[a\x16\x8EV[4\x80\x15a\x07\x0BW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015a\x04\x80V[4\x80\x15a\x07-W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xC9V[a\x03za\x07b6`\x04aX`V[a\x19\x9FV[4\x80\x15a\x07sW`\0\x80\xFD[P`\tTa\x04\x80V[4\x80\x15a\x07\x88W`\0\x80\xFD[Pa\x03za\x1A\x1FV[a\x03za\x07\x9F6`\x04aW\x8FV[a\x1E\rV[a\x03za\x1E\x1AV[4\x80\x15a\x07\xB8W`\0\x80\xFD[P`\x02Ta\x04\x80V[4\x80\x15a\x07\xCDW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015a\x04\x80V[4\x80\x15a\x07\xEFW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`x\x015a\x04\x80V[4\x80\x15a\x08\x11W`\0\x80\xFD[Pa\x08ga\x08 6`\x04aX9V[`\x07` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x82\x16\x91a\x01\0\x81\x04c\xFF\xFF\xFF\xFF\x16\x91e\x01\0\0\0\0\0\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q\x94\x15\x15\x85Rc\xFF\xFF\xFF\xFF\x90\x93\x16` \x85\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01a\x03\xDEV[4\x80\x15a\x08\xB1W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1Ca\x04\xE3V[4\x80\x15a\x08\xD6W`\0\x80\xFD[P`@Q6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x81R` \x01a\x03\xDEV[4\x80\x15a\t\x03W`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015a\x04\x80V[4\x80\x15a\t%W`\0\x80\xFD[Pa\x03\xC9a\t46`\x04aX9V[a\x1E\x85V[4\x80\x15a\tEW`\0\x80\xFD[Pa\x04\x80a\tT6`\x04aWrV[`\x0B` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\trW`\0\x80\xFD[Pa\x04\x80a\t\x816`\x04aX\xA1V[a \x1FV[4\x80\x15a\t\x92W`\0\x80\xFD[Pa\t\xA6a\t\xA16`\x04aX9V[a!\xE0V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x98\x16\x88R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16` \x89\x01R\x95\x90\x94\x16\x94\x86\x01\x94\x90\x94R`\x01`\x01`\x80\x1B\x03\x91\x82\x16``\x86\x01R`\x80\x85\x01R\x91\x82\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\xDEV[4\x80\x15a\n\x06W`\0\x80\xFD[P`\0Ta\x03\xC9\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\n'W`\0\x80\xFD[Pa\x04\x80a\n66`\x04aWrV[a\"TV[4\x80\x15a\nGW`\0\x80\xFD[P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xD4\x015a\x04\x80V[4\x80\x15a\niW`\0\x80\xFD[Pa\x03za\nx6`\x04aX\xCAV[a\"\xACV[4\x80\x15a\n\x89W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xC9V[4\x80\x15a\n\xBCW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x80V[4\x80\x15a\n\xEFW`\0\x80\xFD[Pa\x04Ea\n\xFE6`\x04aX9V[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\x1FW`\0\x80\xFD[Pa\x03za\x0B.6`\x04aW\x8FV[a#(V[4\x80\x15a\x0B?W`\0\x80\xFD[Pa\x0BHa&\xEFV[`@Qa\x03\xDE\x93\x92\x91\x90aYVV[4\x80\x15a\x0BcW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x80V[4\x80\x15a\x0B\x96W`\0\x80\xFD[Pa\x04Ea\x0B\xA56`\x04aX9V[`\x06` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0B\xD9Wa\x0B\xD9aW\x13V[\x14a\x0B\xF7W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x0CJW`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cc6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015\x90V[\x90V[a\x0Cza\x0Cu6\x86\x90\x03\x86\x01\x86aY\x91V[a'\x19V[\x14a\x0C\xB1W`@Q\x7F\x9C\xC0\x0B[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82``\x015\x82\x82`@Qa\x0C\xC6\x92\x91\x90aZ\x05V[`@Q\x80\x91\x03\x90 \x14a\r\x05W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\rNa\rI\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa'u\x92PPPV[a'\xE2V[\x90P`\0a\ru\x82`\x08\x81Q\x81\x10a\rhWa\rhaZ\x15V[` \x02` \x01\x01Qa)\x98V[\x90P` \x81Q\x11\x15a\r\xB3W`@Q\x7F\xD8\x1DX;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x81\x01Q\x82Q\x90\x91\x03`\x03\x1B\x1C6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015\x81\x03a\x0E\nW`@Q\x7F\xB8\xED\x880\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UPP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16r\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UPPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0E\x98Wa\x0E\x98aW\x13V[\x14a\x0E\xB6W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a\x0E\xCBWa\x0E\xCBaZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x90P`\0a\x0E\xE6\x84a\x1E\x85V[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x10\x15a\x0FOW`@Q\x7F\xF2D\x0BS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x06` R`@\x90 T`\xFF\x16\x15a\x0F\x98W`@Q\x7F\xF1\xA9E\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x05` R`@\x90 \x80T\x80\x15\x80\x15a\x0F\xB5WP\x85\x15\x15[\x15a\x10\x18W\x83Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0\x81\x15a\x0F\xDBW\x81a\x0F\xEAV[`\x01\x86\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x90Pa\x0F\xF6\x81\x87a*LV[PPP`\0\x94\x85RPP`\x06` RPP`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0\x86\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x82\x04c\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x93\x81\x01\x93\x90\x93R`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01Ra\x10\x9CW`\x01`\x01`\x80\x1B\x03`@\x82\x01R`\x01\x81R`\0\x86\x90\x03a\x10\x9CW\x81\x95P[`\0\x86\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a\x10\xB4\x91\x90aZAV[\x90P`\0\x83\x82\x11a\x10\xC5W\x81a\x10\xC7V[\x83[` \x84\x01Q\x90\x91Pc\xFF\xFF\xFF\xFF\x16[\x81\x81\x10\x15a\x11\xE7W`\0\x86\x82\x81T\x81\x10a\x10\xF2Wa\x10\xF2aZ\x15V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x80\x83R`\x06\x90\x91R`@\x90\x91 T\x90\x91P`\xFF\x16a\x11JW`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x82\x81T\x81\x10a\x11_Wa\x11_aZ\x15V[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T\x90\x91Pd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x11\xA6WP`\x04\x81\x01T`@\x87\x01Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11[\x15a\x11\xD2W`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x87\x01R`\x04\x81\x01T`\x01`\x01`\x80\x1B\x03\x16`@\x87\x01R[PP\x80\x80a\x11\xDF\x90aZYV[\x91PPa\x10\xD6V[Pc\xFF\xFF\xFF\xFF\x81\x81\x16` \x85\x81\x01\x91\x82R`\0\x8C\x81R`\x07\x90\x91R`@\x90\x81\x90 \x86Q\x81T\x93Q\x92\x88\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x94\x16\x90\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\x16\x17a\x01\0\x92\x90\x94\x16\x91\x82\x02\x93\x90\x93\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x82U``\x85\x01Q`\x01\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x84\x90\x03a\x13\xF6W``\x83\x01Q`\0\x8A\x81R`\x06` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x89\x15\x80\x15a\x133WP`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16[\x15a\x13\x8EW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x13N\x81\x8Aa*LV[\x88T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x88Ua\x13\xF4V[a\x13\xBB`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x13\xA6W\x81a\x13\xB5V[`\x01\x89\x01T`\x01`\x01`\xA0\x1B\x03\x16[\x89a*LV[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x83\x16\x02\x17\x88U[P[PPPPPPPPPV[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14\"Wa\x14\"aW\x13V[\x14a\x14@W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80R`\x06` R\x7FT\xCD\xD3i\xE4\xE8\xA8Q^R\xCAr\xEC\x81l!\x01\x83\x1A\xD1\xF1\x8B\xF4A\x02\xED\x17\x14Y\xC9\xB4\xF8T`\xFF\x16a\x14\xA4W`@Q\x7F\x9A\x07fF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x16`\x02`\0\x81T\x81\x10a\x14\xC3Wa\x14\xC3aZ\x15V[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\xF1W`\x01a\x14\xF4V[`\x02[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x17\x83U\x92\x93P\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17`\x01`\x80\x1B\x83`\x02\x81\x11\x15a\x15\x98Wa\x15\x98aW\x13V[\x02\x17\x90U`\x02\x81\x11\x15a\x15\xADWa\x15\xADaW\x13V[`@Q\x7F^\x18o\t\xB9\xC94\x91\xF1N'~\xEA\x7F\xAA]\xE6\xA2\xD4\xBD\xA7Zy\xAFz6\x84\xFB\xFBB\xDA`\x90`\0\x90\xA2\x90V[`\x05` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15\xF5W`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[\x90P\x90V[a\x16\x1C\x83\x83\x83`\x01a\x19\x9FV[PPPV[`\0a\x16\na\x162`\xF4`\x14aZAV[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`\0\x81\x81R`\x07` \x90\x81R`@\x80\x83 `\x05\x90\x92R\x82 \x80T\x82Ta\x16w\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aZsV[\x95\x94PPPPPV[``a\x16\n`X` a*\x8EV[a\x16\x96a\x1A\x1FV[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\xB1Wa\x16\xB1aW\x13V[\x03a\x16\xD5WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x90 Ta\x17DV[`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a\x16\xEEWa\x16\xEEaW\x13V[\x03a\x17\x12WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 Ta\x17DV[`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x18\"W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x17\xA1`\xC0`\x01\x196\x90\x81\x015`\xF0\x1C\x90\x03\x015``\x1C\x90V[`@Q\x7F~\xEE(\x8D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c~\xEE(\x8D\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x06W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x1AW=`\0\x80>=`\0\xFD[PPPPPPV[\x80`\0\x03a\x18\\W`@Q\x7F\x17\xBF\xE5\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x83\x90U`\x03\x90\x91R\x81 U6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x91\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xFAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x0EW=`\0\x80>=`\0\xFD[PPPP`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19dV[``\x91P[PP\x90P\x80a\x16\x1CW`@Q\x7F\x83\xE6\xCCk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a\x19\xD7WPa\x19\xC2a\x16!V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x1A\rW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1A\x19\x84\x84\x84\x84a*\xC2V[PPPPV[`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\x1A8Wa\x1A8aW\x13V[\x14\x80a\x1AZWP`\x01`\rT`\xFF\x16`\x02\x81\x11\x15a\x1AXWa\x1AXaW\x13V[\x14[\x15a\x1AaWV[`\0`\rT`\xFF\x16`\x02\x81\x11\x15a\x1AzWa\x1AzaW\x13V[\x14a\x1A\xB1W`@Q\x7F\x07\x8A=\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\\\x97Z\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B%\x91\x90aZ\x8AV[\x15a\x1B\\W`@Q\x7F7\x9A~\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Th\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B\xB8W`@Q\x7F\xC1\x05&\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x03\x14\xD2\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x03\x14\xD2\xB3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CP\x91\x90aZ\x8AV[\x90P\x80a\x1C\x89W`@Q\x7FHQ\xBD\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7F\x17\xCF!\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x17\xCF!\xA9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xF8W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x1D\tWP`\x01[P`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA2\x91\x90aZ\x8AV[\x90P\x80\x15a\x1D\xBCW`\r\x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1D\xCAV[`\r\x80T`\xFF\x19\x16`\x02\x17\x90U[`\rT`@Q\x7F\x99\x08\xEA\xAC\x06E\xDF\x9D\x07\x04\xD0j\xDC\x9E\x073|\x95\x1D\xE2\xF0k_(6\x15\x1DH\xD5\xE4r/\x91a\x1E\x01\x91`\xFF\x90\x91\x16\x90aWJV[`@Q\x80\x91\x03\x90\xA1PPV[a\x16\x1C\x83\x83\x83`\0a\x19\x9FV[a\x1E\"a4EV[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C2\x14a\x1EmW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x015`\xE0\x1C\x90V[`\0\x80`\0T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1E\xA6Wa\x1E\xA6aW\x13V[\x14a\x1E\xC4W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x83\x81T\x81\x10a\x1E\xD9Wa\x1E\xD9aZ\x15V[`\0\x91\x82R` \x82 `\x05\x90\x91\x02\x01\x80T\x90\x92Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\x1F?W\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10a\x1F\x17Wa\x1F\x17aZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01`\x04\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90P[`\x04\x82\x01T`\0\x90a\x1Fj\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x1F~\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaZsV[a\x1F\x94a\x1F]\x84`\x01`\x01`\x80\x1B\x03\x16`@\x1C\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\xA8\x91\x90aZAV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1F\xF5W\x80a\x16wV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`\0\x80a \x9C\x83`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a \xFBW`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d.\x90\xED\xD0\0b\x06\x1A\x80c\x11\xE1\xA3\0`\0a!\x16\x83\x83aZ\xBDV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0a!M\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZ\xD1V[\x90P`\0a!ka!fg\r\xE0\xB6\xB3\xA7d\0\0\x86aZ\xD1V[a<\xC6V[\x90P`\0a!y\x84\x84a?\x18V[\x90P`\0a!\x87\x83\x83a?gV[\x90P`\0a!\x94\x82a?\x95V[\x90P`\0a!\xB3\x82a!\xAEg\r\xE0\xB6\xB3\xA7d\0\0\x8FaZ\xD1V[aA}V[\x90P`\0a!\xC1\x8B\x83a?gV[\x90Pa!\xCD\x81\x8DaZ\xD1V[\x9F\x9EPPPPPPPPPPPPPPPV[`\x02\x81\x81T\x81\x10a!\xF0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01Tc\xFF\xFF\xFF\xFF\x84\x16\x95Pd\x01\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x92\x16\x92`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x92\x91\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x87V[`\0`\x02`\rT`\xFF\x16`\x02\x81\x11\x15a\"oWa\"oaW\x13V[\x03a\"\x90WP`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0B` R`@\x90 T\x90V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xF4\x015``\x1C3\x14\x80a\"\xE4WPa\"\xCFa\x16!V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a#\x1AW`@Q\x7F\xD3\x86\xEF>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\x1A\x86\x86\x86\x86\x86\x86aA\xB7V[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a#GWa#GaW\x13V[\x14a#eW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80a#t\x86aF\xEBV[\x93P\x93P\x93P\x93P`\0a#\x8A\x85\x85\x85\x85aJDV[\x90P`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x02\x91\x90aZ\xF0V[\x90P`\x01\x89\x03a$\xCFW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84a$36`\x01\x19\x81\x015`\xF0\x1C\x90\x03`4\x015\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R`D\x82\x01R` `d\x82\x01R`\x84\x81\x01\x8A\x90R`\xA4\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a$\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xC9\x91\x90a[\rV[Pa\x13\xF6V[`\x02\x89\x03a$\xEEW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x89a$3V[`\x03\x89\x03a%\rW`\x01`\x01`\xA0\x1B\x03\x81\x16cR\xF0\xF3\xAD\x8A\x84\x87a$3V[`\x04\x89\x03a&BW`\0a%J`\x01`\x01`\x80\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aJ\xE3V[`\tTa%W\x91\x90aZAV[a%b\x90`\x01aZAV[\x90P6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015\x81\x10a%\x8FW6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015a%\x91V[\x80[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16cR\xF0\xF3\xAD\x8B\x85`@Q`\xE0\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`\xC0\x84\x90\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x8B\x90R`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&;\x91\x90a[\rV[PPa\x13\xF6V[`\x05\x89\x03a&\xBDW`@Q\x7FR\xF0\xF3\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8A\x90R`$\x81\x01\x83\x90R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xD4\x015`\xC0\x1B`D\x82\x01R`\x08`d\x82\x01R`\x84\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cR\xF0\xF3\xAD\x90`\xA4\x01a$\x86V[`@Q\x7F\xFF\x13~e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`T\x81\x015`\xE0\x1C\x90`\x14\x015``a'\x12a\x16\x80V[\x90P\x90\x91\x92V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a'X\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03a'\xC4W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0a'\xF2\x85aKxV[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15a(\rWa(\raW\x13V[\x14a(DW`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa(P\x83\x85aZAV[\x14a(\x87W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a(\x9EW\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15a)\x8CW`\0\x80a)\x11`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01Qa(\xF5\x91\x90aZsV[\x81R` \x01\x85\x8C` \x01Qa)\n\x91\x90aZAV[\x90RaKxV[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a)-\x91\x90aZAV[\x81R` \x01\x84\x8B` \x01Qa)B\x91\x90aZAV[\x81RP\x88\x85\x81Q\x81\x10a)WWa)WaZ\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra)m`\x01\x85aZAV[\x93Pa)y\x81\x83aZAV[a)\x83\x90\x84aZAV[\x92PPPa(\xCBV[P\x84RP\x91\x93\x92PPPV[```\0\x80`\0a)\xA8\x85aKxV[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15a)\xC3Wa)\xC3aW\x13V[\x14a)\xFAW`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\x04\x82\x84aZAV[\x85Q\x14a*=W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16w\x85` \x01Q\x84\x84aP\x16V[`\x02\x81\x01T`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x03` R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x91\x90a*\x85\x90\x84\x90aZAV[\x90\x91UPPPPV[`@Q\x81\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x82\x84\x82\x01` \x84\x017\x82` \x83\x01\x01`\0\x81R` \x81\x01`@RPP\x92\x91PPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a*\xE1Wa*\xE1aW\x13V[\x14a*\xFFW`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x84\x81T\x81\x10a+\x14Wa+\x14aZ\x15V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x81\x16\x84R`\x01`\x01`\xA0\x1B\x03d\x01\0\0\0\0\x90\x91\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01\x81\x01T\x90\x93\x16\x90\x82\x01R`\x02\x82\x01T`\x01`\x01`\x80\x1B\x03\x90\x81\x16``\x83\x01R`\x03\x83\x01T`\x80\x83\x01\x81\x90R`\x04\x90\x93\x01T\x80\x82\x16`\xA0\x84\x01R`\x01`\x80\x1B\x90\x04\x16`\xC0\x82\x01R\x91P\x85\x14a+\xD8W`@Q\x7F0\x14\x032\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xA0\x81\x01Q`\0\x83\x15`\x01`\x01`\x80\x1B\x03\x83\x16\x17`\x01\x1B\x90P`\0a,m\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x86\x15\x80a,\xA8WPa,\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02aZAV[\x81\x14[\x80\x15a,\xB2WP\x84\x15[\x15a,\xE9W`@Q\x7F\xA4&7\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Tr\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x80\x15a-\x0FWP\x86\x15[\x15a-FW`@Q\x7F\x0E\xA2\xE7R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x11\x15a-\xA0W`@Q\x7FV\xF5{+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aZAV[\x81\x03a-\xDDWa-\xDD\x86\x88\x85\x88aP\xABV[4a-\xE7\x83a \x1FV[\x14a.\x1EW`@Q\x7F\x86 \xAA\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.)\x88a\x1E\x85V[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a.\x91W`@Q\x7F3\x81\xD1\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a.\xBE`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZsV[\x83\x03a/\xD4W6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/8\x91\x90aZ\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x99\x91\x90a[\rV[a/\xCD\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[&V[\x90Pa0gV[a/\xFF`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZsV[\x83\x03a0:Wa/\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02a[RV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[a0\x9B\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\x82V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a0\xB6\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a0\xFDWa0\xFA\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\x82V[\x91P[`\0`@\x83\x90\x1BB\x17`\0\x8A\x81R`\x80\x87\x90\x1B`\x01`\x01`\x80\x1B\x03\x8D\x16\x17` R`@\x81 \x91\x92P\x90`\0\x81\x81R`\x04` R`@\x90 T\x90\x91P`\xFF\x16\x15a1rW`@Q\x7F\x80I~;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x04`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x02`@Q\x80`\xE0\x01`@R\x80\x8Dc\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x014`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\x80\x1B\x03\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`\x05`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\x01`\x02\x80T\x90Pa3i\x91\x90aZsV[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x80\x84 \x90\x91\x01\x91\x90\x91U3\x82R`\x0B\x90R`@\x81 \x80T4\x92\x90a3\x9D\x90\x84\x90aZAV[\x90\x91UPP6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a3\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\x03W=`\0\x80>=`\0\xFD[PP`@Q3\x93P\x8D\x92P\x8E\x91P\x7F\x9B2Et\x0E\xC3\xB1U\t\x8AU\xBE\x84\x95zM\xA1>\xAF\x7F\x14\xA8\xBCoS\x12l\x0B\x93P\xF2\xBE\x90`\0\x90\xA4PPPPPPPPPPPPV[`\0Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a4\x97W`@Q\x7F\r\xC1I\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a4\x9FaR:V[6\x14a4\xD7W`@Q\x7F\x98$\xBD\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x806`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD8>\xF2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5M\x91\x90a[\xABV[\x90\x92P\x90P\x81a5\x89W`@Q\x7Fjk\xC3\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x01\x81\x90R`\x08\x82\x90U`\t\x81\x90U\x806`\x01\x19\x81\x015`\xF0\x1C\x90\x03`X\x015\x11a6\x04W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x14\x015`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x81\x91\x90aZ\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE2\x91\x90a[\rV[\x11\x15a7\x1AW`@Q\x7F\xB4\xE1$3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a7Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02aZ\xD1V[\x90P`\x006`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c}\xC0\xD1\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xC9\x91\x90aZ\xF0V[`\x01`\x01`\xA0\x1B\x03\x16c\xF3\xF4\x80\xD9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8*\x91\x90a[\rV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a8e\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a8y\x91\x90aZAV[\x90P`\0a8\x87\x83\x83aROV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xCBW`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a9CW`@Q\x7F\x8Dw\xEC\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x80\x82R`\0` \x80\x84\x01\x82\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03\x805``\x90\x81\x1C\x87\x89\x01\x81\x81R`\x01`\x01`\x80\x1B\x034\x81\x81\x16\x94\x8B\x01\x94\x85R`\x14\x90\x95\x015`\x80\x8B\x01\x90\x81R`\x01`\xA0\x8C\x01\x81\x81RB\x84\x16`\xC0\x8E\x01\x90\x81R`\x02\x80T\x93\x84\x01\x81U\x8CR\x9CQ\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE`\x05\x90\x93\x02\x92\x83\x01\x80T\x9AQ\x91\x90\x9D\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x9A\x16\x99\x90\x99\x17d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x9A\x8B\x16\x02\x17\x90\x9BU\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCF\x84\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x98\x16\x17\x90\x96U\x92Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD0\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD1\x85\x01U\x95Q\x96Q\x96\x81\x16`\x01`\x80\x1B\x97\x90\x91\x16\x96\x90\x96\x02\x95\x90\x95\x17\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xD2\x90\x91\x01U\x81Tq\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82U\x91\x81R`\x0B\x90\x91R\x91\x82 \x80T\x91\x92\x90\x91a;\x84\x90\x84\x90aZAV[\x90\x91UPP6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xC0\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a;\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a;\xEAW=`\0\x80>=`\0\xFD[PP`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UPa<-\x91Pa\x1Eo\x90PV[c\xFF\xFF\xFF\xFF\x166`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\xAC\x015``\x1C`\x01`\x01`\xA0\x1B\x03\x16c<\x9F9|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xA7\x91\x90a[\xCFV[`\n\x80T`\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x92\x90\x92\x14\x17\x90UPPPPPV[`\x01`\x01`\x80\x1B\x03\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17`\0\x82\x13a=\x1CWc\x16\x15\xE68`\0R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[`\0x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F!\x83\x11g\r\xE0\xB6\xB3\xA7d\0\0\x02\x15\x82\x02a?UWc|_H}`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x02\x15a?\x85Wc\xBA\xC6^[`\0R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13a?\xC3W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a?\xE1Wc\xA3{\xFE\xC9`\0R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0aA\xAEg\r\xE0\xB6\xB3\xA7d\0\0\x83aA\x95\x86a<\xC6V[aA\x9F\x91\x90a[\xF5V[aA\xA9\x91\x90a\\\xB1V[a?\x95V[\x90P[\x92\x91PPV[`\0\x80T`\x01`\x80\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15aA\xD6WaA\xD6aW\x13V[\x14aA\xF4W`@Qc\x06\x7F\xE1\x95`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x87\x81T\x81\x10aB\tWaB\taZ\x15V[`\0\x91\x82R` \x82 `\x05\x91\x90\x91\x02\x01`\x04\x81\x01T\x90\x92P`\x01`\x01`\x80\x1B\x03\x16\x90\x87\x15\x82\x17`\x01\x1B\x90PaB_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aZAV[aB\xD9\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x14aC\x13W`@Q\x7F_S\xDD\x98\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x89\x15aC\xDEWaCf\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aZsV[`\x01\x90\x1BaC|\x84`\x01`\x01`\x80\x1B\x03\x16aRfV[`\x01`\x01`\x80\x1B\x03\x16aC\x8F\x91\x90a\\\xFBV[\x15aC\xC3WaC\xBAaC\xAB`\x01`\x01`\x01`\x80\x1B\x03\x87\x16a]\x0FV[\x86Tc\xFF\xFF\xFF\xFF\x16`\0aR\xECV[`\x03\x01TaC\xD4V[6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`x\x015[\x91P\x84\x90PaC\xFFV[`\x03\x85\x01T\x91PaC\xFCaC\xAB`\x01`\x01`\x80\x1B\x03\x86\x16`\x01a]/V[\x90P[`\x08\x82\x90\x1B`\x08\x8A\x8A`@QaD\x16\x92\x91\x90aZ\x05V[`@Q\x80\x91\x03\x90 \x90\x1B\x14aDWW`@Q\x7FieP\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0aDb\x8CaS\xB5V[\x90P`\0aDq\x83`\x03\x01T\x90V[`@Q\x7F\xE1L\xED2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R6`\x01\x19\x81\x015`\xF0\x1C\x90\x03`\x98\x015``\x1C\x90c\xE1L\xED2\x90aD\xC7\x90\x8F\x90\x8F\x90\x8F\x90\x8F\x90\x8A\x90`\x04\x01a]\x9AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aD\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\n\x91\x90a[\rV[`\x04\x85\x01T\x91\x14\x91P`\0\x90`\x02\x90aE\x93\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aF\r\x89`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aF\x17\x91\x90a]\xD4V[aF!\x91\x90a]\xF7V[`\xFF\x16\x15\x90P\x81\x15\x15\x81\x03aFbW`@Q\x7F\xFBN@\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15aF\xACW`@Q\x7F\x90q\xE6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x85T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x163d\x01\0\0\0\0\x02\x17\x90\x95UPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x85\x90P`\0`\x02\x82\x81T\x81\x10aG\x0BWaG\x0BaZ\x15V[`\0\x91\x82R` \x90\x91 `\x04`\x05\x90\x92\x02\x01\x90\x81\x01T\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aG\xC0\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aG\xFAW`@Q\x7F\xB3K\\\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[`\x04\x83\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90aH\x9F\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x92P\x82\x11\x15aI\x14W\x82Tc\xFF\xFF\xFF\xFF\x16aH\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01aZAV[\x83\x03aH\xE8W\x83\x91P[`\x02\x81\x81T\x81\x10aH\xFBWaH\xFBaZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x93P\x80\x94PPaG\xFEV[`\x04\x81\x81\x01T\x90\x84\x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16`\0\x81`\x01`\x01`\x80\x1B\x03\x16aIYaIM\x85`\x01`\x01`\x80\x1B\x03\x16`\x01\x1C\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x14\x90P\x80\x15aI\xF2W`\0aI\x7F\x83`\x01`\x01`\x80\x1B\x03\x16aRfV[`\x01`\x01`\x80\x1B\x03\x16\x11\x15aI\xCFW`\0aI\xAFaI\xA7`\x01`\x01`\x01`\x80\x1B\x03\x86\x16a]\x0FV[\x89`\x01aR\xECV[`\x03\x81\x01T`\x04\x90\x91\x01T\x90\x9CP`\x01`\x01`\x80\x1B\x03\x16\x9APaI\xD5\x90PV[`\x08T\x9AP[`\x03\x86\x01T`\x04\x87\x01T\x90\x99P`\x01`\x01`\x80\x1B\x03\x16\x97PaJ6V[`\0aJ\x0BaI\xA7`\x01`\x01`\x80\x1B\x03\x85\x16`\x01a]/V[`\x03\x80\x89\x01T`\x04\x80\x8B\x01T\x92\x84\x01T\x93\x01T\x90\x9EP`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x9DP\x91\x9BP\x16\x98PP[PPPPPPP\x91\x93P\x91\x93V[`\0`\x01`\x01`\x80\x1B\x03\x84\x16\x15aJ\x9FW`@\x80Q` \x81\x01\x87\x90R`\x01`\x01`\x80\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R\x90\x83\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x16wV[\x82\x82`@Q` \x01aJ\xC4\x92\x91\x90\x91\x82R`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x95\x94PPPPPV[`\0\x80aKW\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[`\0\x80`\0\x83`\0\x01Q`\0\x03aK\xBBW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11aK\xE0W`\0`\x01`\0\x94P\x94P\x94PPPaP\x0FV[`\xB7\x81\x11aL\xF6W`\0aK\xF5`\x80\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aL4W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15aL\xACWP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15aL\xE3W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92PaP\x0F\x91PPV[`\xBF\x81\x11aNTW`\0aM\x0B`\xB7\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aMJW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aM\xACW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aM\xF4W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aM\xFE\x81\x84aZAV[\x89Q\x11aN7W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aNB\x83`\x01aZAV[\x97P\x95P`\0\x94PaP\x0F\x93PPPPV[`\xF7\x81\x11aN\xB9W`\0aNi`\xC0\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aN\xA8W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92PaP\x0F\x91PPV[`\0aN\xC6`\xF7\x83aZsV[\x90P\x80\x87`\0\x01Q\x11aO\x05W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aOgW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aO\xAFW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aO\xB9\x81\x84aZAV[\x89Q\x11aO\xF2W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aO\xFD\x83`\x01aZAV[\x97P\x95P`\x01\x94PaP\x0F\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP1WaP1aY{V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aP[W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15aP\xA4W`\0aPp\x84\x86aZAV[\x90P` \x82\x01`\0[\x84\x81\x10\x15aP\x91W\x82\x81\x01Q\x82\x82\x01R` \x01aPyV[\x84\x81\x11\x15aP\xA0W`\0\x85\x83\x01R[PPP[\x93\x92PPPV[`\0aP\xC1`\x01`\x01`\x80\x1B\x03\x84\x16`\x01a]/V[\x90P`\0aP\xD1\x82\x86`\x01aR\xECV[\x90P`\0\x86\x90\x1A\x83\x80aQ\x9BWPaQ\n`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\\\xFBV[`\x04\x83\x01T`\x02\x90aQ\x8C\x90`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[aQ\x96\x91\x90a]\xF7V[`\xFF\x16\x14[\x15aQ\xF3W`\xFF\x81\x16`\x01\x14\x80aQ\xB5WP`\xFF\x81\x16`\x02\x14[aQ\xEEW`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a5\xFBV[aR1V[`\xFF\x81\x16\x15aR1W`@Q\x7F\xF4\x029\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x88\x90R`$\x01a5\xFBV[PPPPPPPV[`\0aRDaS\xE4V[a\x16\n\x90`\x06aZAV[`\0\x81\x83\x10\x15aR_W\x81aA\xAEV[P\x90\x91\x90PV[`\0\x80aR\xDA\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01`\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x80\x82aS,WaS'`\x01`\x01`\x80\x1B\x03\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aS\xF2V[aS>V[aS>\x85`\x01`\x01`\x80\x1B\x03\x16aU1V[\x90P`\x02\x84\x81T\x81\x10aSSWaSSaZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x91P[`\x04\x82\x01T`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x91\x16\x14aS\xADW\x81T`\x02\x80T\x90\x91c\xFF\xFF\xFF\xFF\x16\x90\x81\x10aS\x98WaS\x98aZ\x15V[\x90`\0R` `\0 \x90`\x05\x02\x01\x91PaSdV[P\x93\x92PPPV[`\0\x80`\0\x80`\0aS\xC6\x86aF\xEBV[\x93P\x93P\x93P\x93PaS\xDA\x84\x84\x84\x84aJDV[\x96\x95PPPPPPV[`\0a\x16\n`\xF4`(aZAV[`\0\x81aTo\x84`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aT\x85Wc\xB3K\\\"`\0R`\x04`\x1C\xFD[aT\x8E\x83aU1V[\x90P\x81aU\x0B\x82`\x01`\x01`\x80\x1B\x03\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x11aA\xB1WaA\xAEaU!\x83`\x01aZAV[`\x01`\x01`\x80\x1B\x03\x83\x16\x90aU\xBDV[`\0\x81\x19`\x01\x83\x01\x16\x81aU\xAC\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`\0\x80aV1\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1Fc\x07\xC4\xAC\xDD`\xE0\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\xFF\x16\x90P\x80\x83\x03`\x01\x80\x82\x1B\x03\x85\x82\x1B\x17\x92PPP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aV_W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aVwW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aV\x8FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15aV\xACW`\0\x80\xFD[`\x80\x81\x12\x15aV\xBAW`\0\x80\xFD[P\x83\x92P`\x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xD8W`\0\x80\xFD[aV\xE4\x86\x82\x87\x01aVMV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15aW\x04W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aWGWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01aWW\x83aW)V[\x91\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aWGW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aW\x84W`\0\x80\xFD[\x815aP\xA4\x81aW]V[`\0\x80`\0``\x84\x86\x03\x12\x15aW\xA4W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aW\xE1W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aW\xC5V[\x81\x81\x11\x15aW\xF3W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0aA\xAE` \x83\x01\x84aW\xBBV[`\0` \x82\x84\x03\x12\x15aXKW`\0\x80\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14aWGW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aXvW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aX\x96\x81aXRV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aX\xB3W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aP\xA4W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aX\xE3W`\0\x80\xFD[\x865\x95P` \x87\x015aX\xF5\x81aXRV[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aY\x12W`\0\x80\xFD[aY\x1E\x8A\x83\x8B\x01aVMV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aY7W`\0\x80\xFD[PaYD\x89\x82\x8A\x01aVMV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x16w``\x83\x01\x84aW\xBBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15aY\xA3W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aY\xD4WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aZTWaZTaZ+V[P\x01\x90V[`\0`\0\x19\x82\x03aZlWaZlaZ+V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aZ\x85WaZ\x85aZ+V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aZ\x9CW`\0\x80\xFD[\x81QaP\xA4\x81aXRV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aZ\xCCWaZ\xCCaZ\xA7V[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aZ\xEBWaZ\xEBaZ+V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15a[\x02W`\0\x80\xFD[\x81QaP\xA4\x81aW]V[`\0` \x82\x84\x03\x12\x15a[\x1FW`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a[IWa[IaZ+V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a[yWa[yaZ+V[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a[\xA3Wa[\xA3aZ+V[\x03\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a[\xBEW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a[\xE1W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14aP\xA4W`\0\x80\xFD[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\\6Wa\\6aZ+V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\\qWa\\qaZ+V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\\\x8DWa\\\x8DaZ+V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\\\xA3Wa\\\xA3aZ+V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82a\\\xC0Wa\\\xC0aZ\xA7V[`\0\x19\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\\\xF6Wa\\\xF6aZ+V[P\x05\x90V[`\0\x82a]\nWa]\naZ\xA7V[P\x06\x90V[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a[\xA3Wa[\xA3aZ+V[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a[IWa[IaZ+V[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0a]\xAE``\x83\x01\x87\x89a]QV[\x82\x81\x03` \x84\x01Ra]\xC1\x81\x86\x88a]QV[\x91PP\x82`@\x83\x01R\x96\x95PPPPPPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a]\xEEWa]\xEEaZ+V[\x90\x03\x93\x92PPPV[`\0`\xFF\x83\x16\x80a^\nWa^\naZ\xA7V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
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
constructor(FaultDisputeGameV2.GameConstructorParams _params);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _params: <FaultDisputeGameV2::GameConstructorParams as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (FaultDisputeGameV2::GameConstructorParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <FaultDisputeGameV2::GameConstructorParams as alloy::sol_types::SolType>::RustType,
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
            type Parameters<'a> = (FaultDisputeGameV2::GameConstructorParams,);
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
                    <FaultDisputeGameV2::GameConstructorParams as alloy_sol_types::SolType>::tokenize(
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
function l2ChainId() external pure returns (uint256 l2ChainId_);
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
    ///Container for all the [`PermissionedDisputeGameV2`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum PermissionedDisputeGameV2Calls {
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
    impl PermissionedDisputeGameV2Calls {
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
            [83u8, 77u8, 176u8, 226u8],
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
            [168u8, 228u8, 251u8, 144u8],
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
            ::core::stringify!(challenger),
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
            ::core::stringify!(proposer),
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
            <challengerCall as alloy_sol_types::SolCall>::SIGNATURE,
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
            <proposerCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for PermissionedDisputeGameV2Calls {
        const NAME: &'static str = "PermissionedDisputeGameV2Calls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 54usize;
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
            ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls>] = &[
                {
                    fn challengeRootL2Block(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::challengeRootL2Block)
                    }
                    challengeRootL2Block
                },
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Calls::wasRespectedGameTypeWhenCreated,
                            )
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::subgames)
                    }
                    subgames
                },
                {
                    fn l2BlockNumberChallenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2BlockNumberChallenger)
                    }
                    l2BlockNumberChallenger
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::vm)
                    }
                    vm
                },
                {
                    fn l2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2BlockNumberChallenged)
                    }
                    l2BlockNumberChallenged
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn challenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <challengerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::challenger)
                    }
                    challenger
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::version)
                    }
                    version
                },
                {
                    fn startingOutputRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <startingOutputRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::startingOutputRoot)
                    }
                    startingOutputRoot
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::r#move)
                    }
                    r#move
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn l2BlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2BlockNumber)
                    }
                    l2BlockNumber
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolutionCheckpoints)
                    }
                    resolutionCheckpoints
                },
                {
                    fn proposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <proposerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::proposer)
                    }
                    proposer
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::getChallengerDuration)
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::credit)
                    }
                    credit
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Calls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolvedSubgames)
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
            ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls>] = &[
                {
                    fn challengeRootL2Block(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <challengeRootL2BlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::challengeRootL2Block)
                    }
                    challengeRootL2Block
                },
                {
                    fn resolveClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolveClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolveClaim)
                    }
                    resolveClaim
                },
                {
                    fn resolvedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolvedAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolvedAt)
                    }
                    resolvedAt
                },
                {
                    fn status(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <statusCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::status)
                    }
                    status
                },
                {
                    fn hasUnlockedCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <hasUnlockedCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::hasUnlockedCredit)
                    }
                    hasUnlockedCredit
                },
                {
                    fn wasRespectedGameTypeWhenCreated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <wasRespectedGameTypeWhenCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Calls::wasRespectedGameTypeWhenCreated,
                            )
                    }
                    wasRespectedGameTypeWhenCreated
                },
                {
                    fn startingRootHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <startingRootHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::startingRootHash)
                    }
                    startingRootHash
                },
                {
                    fn resolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolve)
                    }
                    resolve
                },
                {
                    fn subgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <subgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::subgames)
                    }
                    subgames
                },
                {
                    fn l2BlockNumberChallenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2BlockNumberChallengerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2BlockNumberChallenger)
                    }
                    l2BlockNumberChallenger
                },
                {
                    fn bondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <bondDistributionModeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::bondDistributionMode)
                    }
                    bondDistributionMode
                },
                {
                    fn gameCreator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <gameCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::gameCreator)
                    }
                    gameCreator
                },
                {
                    fn vm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <vmCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::vm)
                    }
                    vm
                },
                {
                    fn l2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2BlockNumberChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2BlockNumberChallenged)
                    }
                    l2BlockNumberChallenged
                },
                {
                    fn weth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::weth)
                    }
                    weth
                },
                {
                    fn attack(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <attackCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::attack)
                    }
                    attack
                },
                {
                    fn normalModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <normalModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::normalModeCredit)
                    }
                    normalModeCredit
                },
                {
                    fn challenger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <challengerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::challenger)
                    }
                    challenger
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::version)
                    }
                    version
                },
                {
                    fn startingOutputRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <startingOutputRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::startingOutputRoot)
                    }
                    startingOutputRoot
                },
                {
                    fn getNumToResolve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <getNumToResolveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::getNumToResolve)
                    }
                    getNumToResolve
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn extraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <extraDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::extraData)
                    }
                    extraData
                },
                {
                    fn claimCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::claimCredit)
                    }
                    claimCredit
                },
                {
                    fn l1Head(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l1HeadCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l1Head)
                    }
                    l1Head
                },
                {
                    fn clockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <clockExtensionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::clockExtension)
                    }
                    clockExtension
                },
                {
                    fn r#move(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <moveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::r#move)
                    }
                    r#move
                },
                {
                    fn startingBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <startingBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::startingBlockNumber)
                    }
                    startingBlockNumber
                },
                {
                    fn closeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <closeGameCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::closeGame)
                    }
                    closeGame
                },
                {
                    fn defend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <defendCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::defend)
                    }
                    defend
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::initialize)
                    }
                    initialize
                },
                {
                    fn claimDataLen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimDataLenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::claimDataLen)
                    }
                    claimDataLen
                },
                {
                    fn l2BlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2BlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2BlockNumber)
                    }
                    l2BlockNumber
                },
                {
                    fn absolutePrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <absolutePrestateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::absolutePrestate)
                    }
                    absolutePrestate
                },
                {
                    fn l2SequenceNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2SequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2SequenceNumber)
                    }
                    l2SequenceNumber
                },
                {
                    fn resolutionCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolutionCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolutionCheckpoints)
                    }
                    resolutionCheckpoints
                },
                {
                    fn proposer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <proposerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::proposer)
                    }
                    proposer
                },
                {
                    fn gameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <gameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::gameType)
                    }
                    gameType
                },
                {
                    fn rootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <rootClaimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::rootClaim)
                    }
                    rootClaim
                },
                {
                    fn getChallengerDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <getChallengerDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::getChallengerDuration)
                    }
                    getChallengerDuration
                },
                {
                    fn refundModeCredit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <refundModeCreditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::refundModeCredit)
                    }
                    refundModeCredit
                },
                {
                    fn getRequiredBond(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <getRequiredBondCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::getRequiredBond)
                    }
                    getRequiredBond
                },
                {
                    fn claimData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::claimData)
                    }
                    claimData
                },
                {
                    fn createdAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <createdAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::createdAt)
                    }
                    createdAt
                },
                {
                    fn credit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <creditCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::credit)
                    }
                    credit
                },
                {
                    fn l2ChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <l2ChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::l2ChainId)
                    }
                    l2ChainId
                },
                {
                    fn step(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <stepCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::step)
                    }
                    step
                },
                {
                    fn maxClockDuration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <maxClockDurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::maxClockDuration)
                    }
                    maxClockDuration
                },
                {
                    fn splitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <splitDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::splitDepth)
                    }
                    splitDepth
                },
                {
                    fn claims(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <claimsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::claims)
                    }
                    claims
                },
                {
                    fn addLocalData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <addLocalDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::addLocalData)
                    }
                    addLocalData
                },
                {
                    fn gameData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <gameDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::gameData)
                    }
                    gameData
                },
                {
                    fn maxGameDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <maxGameDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::maxGameDepth)
                    }
                    maxGameDepth
                },
                {
                    fn resolvedSubgames(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Calls> {
                        <resolvedSubgamesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Calls::resolvedSubgames)
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
    ///Container for all the [`PermissionedDisputeGameV2`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum PermissionedDisputeGameV2Errors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        AnchorRootNotFound(AnchorRootNotFound),
        #[allow(missing_docs)]
        BadAuth(BadAuth),
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
        UnexpectedList(UnexpectedList),
        #[allow(missing_docs)]
        UnexpectedRootClaim(UnexpectedRootClaim),
        #[allow(missing_docs)]
        UnexpectedString(UnexpectedString),
        #[allow(missing_docs)]
        ValidStep(ValidStep),
    }
    impl PermissionedDisputeGameV2Errors {
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
            [211u8, 134u8, 239u8, 62u8],
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
            ::core::stringify!(BadAuth),
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
            <BadAuth as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for PermissionedDisputeGameV2Errors {
        const NAME: &'static str = "PermissionedDisputeGameV2Errors";
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
                Self::BadAuth(_) => <BadAuth as alloy_sol_types::SolError>::SELECTOR,
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
            ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Errors::InvalidBondDistributionMode,
                            )
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn L2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Errors::L2BlockNumberChallenged,
                            )
                    }
                    L2BlockNumberChallenged
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Errors::InvalidDisputedClaimIndex,
                            )
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Errors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Errors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidClockExtension)
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Errors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::OutOfOrderResolution)
                    }
                    OutOfOrderResolution
                },
                {
                    fn InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidOutputRootProof)
                    }
                    InvalidOutputRootProof
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::CannotDefendRootClaim)
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidChallengePeriod)
                    }
                    InvalidChallengePeriod
                },
                {
                    fn BlockNumberMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BlockNumberMatches as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::BlockNumberMatches)
                    }
                    BlockNumberMatches
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn BadAuth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BadAuth as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Errors::BadAuth)
                    }
                    BadAuth
                },
                {
                    fn InvalidHeaderRLP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidHeaderRLP)
                    }
                    InvalidHeaderRLP
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClaimAlreadyResolved)
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(PermissionedDisputeGameV2Errors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidLocalIdent)
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
            ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors>] = &[
                {
                    fn InvalidBondDistributionMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidBondDistributionMode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Errors::InvalidBondDistributionMode,
                            )
                    }
                    InvalidBondDistributionMode
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn L2BlockNumberChallenged(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <L2BlockNumberChallenged as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Errors::L2BlockNumberChallenged,
                            )
                    }
                    L2BlockNumberChallenged
                },
                {
                    fn NoCreditToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <NoCreditToClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::NoCreditToClaim)
                    }
                    NoCreditToClaim
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn InvalidDisputedClaimIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidDisputedClaimIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                PermissionedDisputeGameV2Errors::InvalidDisputedClaimIndex,
                            )
                    }
                    InvalidDisputedClaimIndex
                },
                {
                    fn ClockTimeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClockTimeExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClockTimeExceeded)
                    }
                    ClockTimeExceeded
                },
                {
                    fn GamePaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GamePaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GamePaused)
                    }
                    GamePaused
                },
                {
                    fn GameNotFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameNotFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameNotFinalized)
                    }
                    GameNotFinalized
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn GameDepthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameDepthExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameDepthExceeded)
                    }
                    GameDepthExceeded
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn InvalidParent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidParent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidParent)
                    }
                    InvalidParent
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn GameNotInProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameNotInProgress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameNotInProgress)
                    }
                    GameNotInProgress
                },
                {
                    fn InvalidPrestate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidPrestate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidPrestate)
                    }
                    InvalidPrestate
                },
                {
                    fn AnchorRootNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <AnchorRootNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::AnchorRootNotFound)
                    }
                    AnchorRootNotFound
                },
                {
                    fn MaxDepthTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <MaxDepthTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::MaxDepthTooLarge)
                    }
                    MaxDepthTooLarge
                },
                {
                    fn ClaimAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClaimAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClaimAlreadyExists)
                    }
                    ClaimAlreadyExists
                },
                {
                    fn BondTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BondTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::BondTransferFailed)
                    }
                    BondTransferFailed
                },
                {
                    fn IncorrectBondAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <IncorrectBondAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::IncorrectBondAmount)
                    }
                    IncorrectBondAmount
                },
                {
                    fn InvalidClockExtension(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidClockExtension as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidClockExtension)
                    }
                    InvalidClockExtension
                },
                {
                    fn DuplicateStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <DuplicateStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::DuplicateStep)
                    }
                    DuplicateStep
                },
                {
                    fn BadExtraData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BadExtraData as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::BadExtraData)
                    }
                    BadExtraData
                },
                {
                    fn OutOfOrderResolution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <OutOfOrderResolution as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::OutOfOrderResolution)
                    }
                    OutOfOrderResolution
                },
                {
                    fn InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidOutputRootProof)
                    }
                    InvalidOutputRootProof
                },
                {
                    fn CannotDefendRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <CannotDefendRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::CannotDefendRootClaim)
                    }
                    CannotDefendRootClaim
                },
                {
                    fn ClaimAboveSplit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClaimAboveSplit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClaimAboveSplit)
                    }
                    ClaimAboveSplit
                },
                {
                    fn InvalidChallengePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidChallengePeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidChallengePeriod)
                    }
                    InvalidChallengePeriod
                },
                {
                    fn BlockNumberMatches(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BlockNumberMatches as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::BlockNumberMatches)
                    }
                    BlockNumberMatches
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn GameNotResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <GameNotResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::GameNotResolved)
                    }
                    GameNotResolved
                },
                {
                    fn BadAuth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <BadAuth as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::BadAuth)
                    }
                    BadAuth
                },
                {
                    fn InvalidHeaderRLP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidHeaderRLP as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidHeaderRLP)
                    }
                    InvalidHeaderRLP
                },
                {
                    fn InvalidSplitDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidSplitDepth as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidSplitDepth)
                    }
                    InvalidSplitDepth
                },
                {
                    fn ClaimAlreadyResolved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClaimAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClaimAlreadyResolved)
                    }
                    ClaimAlreadyResolved
                },
                {
                    fn ClockNotExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ClockNotExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ClockNotExpired)
                    }
                    ClockNotExpired
                },
                {
                    fn UnexpectedRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <UnexpectedRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::UnexpectedRootClaim)
                    }
                    UnexpectedRootClaim
                },
                {
                    fn ValidStep(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <ValidStep as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::ValidStep)
                    }
                    ValidStep
                },
                {
                    fn InvalidLocalIdent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PermissionedDisputeGameV2Errors> {
                        <InvalidLocalIdent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PermissionedDisputeGameV2Errors::InvalidLocalIdent)
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
                Self::BadAuth(inner) => {
                    <BadAuth as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
    ///Container for all the [`PermissionedDisputeGameV2`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum PermissionedDisputeGameV2Events {
        #[allow(missing_docs)]
        GameClosed(GameClosed),
        #[allow(missing_docs)]
        Move(Move),
        #[allow(missing_docs)]
        Resolved(Resolved),
    }
    impl PermissionedDisputeGameV2Events {
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
    impl alloy_sol_types::SolEventInterface for PermissionedDisputeGameV2Events {
        const NAME: &'static str = "PermissionedDisputeGameV2Events";
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
    impl alloy_sol_types::private::IntoLogData for PermissionedDisputeGameV2Events {
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
    /**Creates a new wrapper around an on-chain [`PermissionedDisputeGameV2`](self) contract instance.

See the [wrapper's documentation](`PermissionedDisputeGameV2Instance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> PermissionedDisputeGameV2Instance<P, N> {
        PermissionedDisputeGameV2Instance::<P, N>::new(address, __provider)
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
        _params: <FaultDisputeGameV2::GameConstructorParams as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PermissionedDisputeGameV2Instance<P, N>>,
    > {
        PermissionedDisputeGameV2Instance::<P, N>::deploy(__provider, _params)
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
        _params: <FaultDisputeGameV2::GameConstructorParams as alloy::sol_types::SolType>::RustType,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        PermissionedDisputeGameV2Instance::<P, N>::deploy_builder(__provider, _params)
    }
    /**A [`PermissionedDisputeGameV2`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PermissionedDisputeGameV2`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PermissionedDisputeGameV2Instance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for PermissionedDisputeGameV2Instance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PermissionedDisputeGameV2Instance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > PermissionedDisputeGameV2Instance<P, N> {
        /**Creates a new wrapper around an on-chain [`PermissionedDisputeGameV2`](self) contract instance.

See the [wrapper's documentation](`PermissionedDisputeGameV2Instance`) for more details.*/
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
            _params: <FaultDisputeGameV2::GameConstructorParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<PermissionedDisputeGameV2Instance<P, N>> {
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
            _params: <FaultDisputeGameV2::GameConstructorParams as alloy::sol_types::SolType>::RustType,
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
    impl<P: ::core::clone::Clone, N> PermissionedDisputeGameV2Instance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PermissionedDisputeGameV2Instance<P, N> {
            PermissionedDisputeGameV2Instance {
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
    > PermissionedDisputeGameV2Instance<P, N> {
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
    > PermissionedDisputeGameV2Instance<P, N> {
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

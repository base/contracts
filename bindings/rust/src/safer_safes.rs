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
///Module containing a contract's types and functions.
/**

```solidity
library LivenessModule2 {
    struct ModuleConfig { uint256 livenessResponsePeriod; address fallbackOwner; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod LivenessModule2 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ModuleConfig { uint256 livenessResponsePeriod; address fallbackOwner; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ModuleConfig {
        #[allow(missing_docs)]
        pub livenessResponsePeriod: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fallbackOwner: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<ModuleConfig> for UnderlyingRustTuple<'_> {
            fn from(value: ModuleConfig) -> Self {
                (value.livenessResponsePeriod, value.fallbackOwner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ModuleConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    livenessResponsePeriod: tuple.0,
                    fallbackOwner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ModuleConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ModuleConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.livenessResponsePeriod,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.fallbackOwner,
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
        impl alloy_sol_types::SolType for ModuleConfig {
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
        impl alloy_sol_types::SolStruct for ModuleConfig {
            const NAME: &'static str = "ModuleConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ModuleConfig(uint256 livenessResponsePeriod,address fallbackOwner)",
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.livenessResponsePeriod,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.fallbackOwner,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ModuleConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.livenessResponsePeriod,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fallbackOwner,
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
                    &rust.livenessResponsePeriod,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fallbackOwner,
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
    /**Creates a new wrapper around an on-chain [`LivenessModule2`](self) contract instance.

See the [wrapper's documentation](`LivenessModule2Instance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> LivenessModule2Instance<P, N> {
        LivenessModule2Instance::<P, N>::new(address, __provider)
    }
    /**A [`LivenessModule2`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LivenessModule2`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LivenessModule2Instance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for LivenessModule2Instance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LivenessModule2Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > LivenessModule2Instance<P, N> {
        /**Creates a new wrapper around an on-chain [`LivenessModule2`](self) contract instance.

See the [wrapper's documentation](`LivenessModule2Instance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> LivenessModule2Instance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LivenessModule2Instance<P, N> {
            LivenessModule2Instance {
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
    > LivenessModule2Instance<P, N> {
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
    > LivenessModule2Instance<P, N> {
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
library TimelockGuard {
    type TransactionState is uint8;
    struct ExecTransactionParams { address to; uint256 value; bytes data; Enum.Operation operation; uint256 safeTxGas; uint256 baseGas; uint256 gasPrice; address gasToken; address refundReceiver; }
    struct ScheduledTransaction { bytes32 txHash; uint256 executionTime; TransactionState state; ExecTransactionParams params; uint256 nonce; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod TimelockGuard {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransactionState(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<TransactionState> for u8 {
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
        impl TransactionState {
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
        impl From<u8> for TransactionState {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<TransactionState> for u8 {
            fn from(value: TransactionState) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TransactionState {
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
        impl alloy_sol_types::EventTopic for TransactionState {
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
struct ExecTransactionParams { address to; uint256 value; bytes data; Enum.Operation operation; uint256 safeTxGas; uint256 baseGas; uint256 gasPrice; address gasToken; address refundReceiver; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExecTransactionParams {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub safeTxGas: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseGas: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasPrice: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub refundReceiver: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Bytes,
            Enum::Operation,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
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
        impl ::core::convert::From<ExecTransactionParams> for UnderlyingRustTuple<'_> {
            fn from(value: ExecTransactionParams) -> Self {
                (
                    value.to,
                    value.value,
                    value.data,
                    value.operation,
                    value.safeTxGas,
                    value.baseGas,
                    value.gasPrice,
                    value.gasToken,
                    value.refundReceiver,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExecTransactionParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    to: tuple.0,
                    value: tuple.1,
                    data: tuple.2,
                    operation: tuple.3,
                    safeTxGas: tuple.4,
                    baseGas: tuple.5,
                    gasPrice: tuple.6,
                    gasToken: tuple.7,
                    refundReceiver: tuple.8,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ExecTransactionParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ExecTransactionParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self.operation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.safeTxGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasPrice),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.refundReceiver,
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
        impl alloy_sol_types::SolType for ExecTransactionParams {
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
        impl alloy_sol_types::SolStruct for ExecTransactionParams {
            const NAME: &'static str = "ExecTransactionParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ExecTransactionParams(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 baseGas,uint256 gasPrice,address gasToken,address refundReceiver)",
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
                            &self.to,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.value)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                    <Enum::Operation as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operation,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.safeTxGas)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.baseGas)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gasPrice)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.gasToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.refundReceiver,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ExecTransactionParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.to,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.value)
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
                    + <Enum::Operation as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operation,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.safeTxGas,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseGas,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gasPrice,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gasToken,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.refundReceiver,
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
                    &rust.to,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.value,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
                <Enum::Operation as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operation,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.safeTxGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.baseGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gasPrice,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gasToken,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.refundReceiver,
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
struct ScheduledTransaction { bytes32 txHash; uint256 executionTime; TransactionState state; ExecTransactionParams params; uint256 nonce; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ScheduledTransaction {
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub executionTime: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub state: <TransactionState as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub params: <ExecTransactionParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            TransactionState,
            ExecTransactionParams,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            <TransactionState as alloy::sol_types::SolType>::RustType,
            <ExecTransactionParams as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<ScheduledTransaction> for UnderlyingRustTuple<'_> {
            fn from(value: ScheduledTransaction) -> Self {
                (
                    value.txHash,
                    value.executionTime,
                    value.state,
                    value.params,
                    value.nonce,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ScheduledTransaction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txHash: tuple.0,
                    executionTime: tuple.1,
                    state: tuple.2,
                    params: tuple.3,
                    nonce: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ScheduledTransaction {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ScheduledTransaction {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.txHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.executionTime),
                    <TransactionState as alloy_sol_types::SolType>::tokenize(
                        &self.state,
                    ),
                    <ExecTransactionParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
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
        impl alloy_sol_types::SolType for ScheduledTransaction {
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
        impl alloy_sol_types::SolStruct for ScheduledTransaction {
            const NAME: &'static str = "ScheduledTransaction";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ScheduledTransaction(bytes32 txHash,uint256 executionTime,uint8 state,ExecTransactionParams params,uint256 nonce)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <ExecTransactionParams as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExecTransactionParams as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.txHash)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.executionTime)
                        .0,
                    <TransactionState as alloy_sol_types::SolType>::eip712_data_word(
                            &self.state,
                        )
                        .0,
                    <ExecTransactionParams as alloy_sol_types::SolType>::eip712_data_word(
                            &self.params,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ScheduledTransaction {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txHash,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.executionTime,
                    )
                    + <TransactionState as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.state,
                    )
                    + <ExecTransactionParams as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.params,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
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
                    &rust.txHash,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.executionTime,
                    out,
                );
                <TransactionState as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.state,
                    out,
                );
                <ExecTransactionParams as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.params,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
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
    /**Creates a new wrapper around an on-chain [`TimelockGuard`](self) contract instance.

See the [wrapper's documentation](`TimelockGuardInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> TimelockGuardInstance<P, N> {
        TimelockGuardInstance::<P, N>::new(address, __provider)
    }
    /**A [`TimelockGuard`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`TimelockGuard`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TimelockGuardInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for TimelockGuardInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TimelockGuardInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > TimelockGuardInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`TimelockGuard`](self) contract instance.

See the [wrapper's documentation](`TimelockGuardInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> TimelockGuardInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TimelockGuardInstance<P, N> {
            TimelockGuardInstance {
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
    > TimelockGuardInstance<P, N> {
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
    > TimelockGuardInstance<P, N> {
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

library LivenessModule2 {
    struct ModuleConfig {
        uint256 livenessResponsePeriod;
        address fallbackOwner;
    }
}

library TimelockGuard {
    type TransactionState is uint8;
    struct ExecTransactionParams {
        address to;
        uint256 value;
        bytes data;
        Enum.Operation operation;
        uint256 safeTxGas;
        uint256 baseGas;
        uint256 gasPrice;
        address gasToken;
        address payable refundReceiver;
    }
    struct ScheduledTransaction {
        bytes32 txHash;
        uint256 executionTime;
        TransactionState state;
        ExecTransactionParams params;
        uint256 nonce;
    }
}

interface SaferSafes {
    error LivenessModule2_ChallengeAlreadyExists();
    error LivenessModule2_ChallengeDoesNotExist();
    error LivenessModule2_InvalidFallbackOwner();
    error LivenessModule2_InvalidResponsePeriod();
    error LivenessModule2_InvalidVersion();
    error LivenessModule2_ModuleNotConfigured();
    error LivenessModule2_ModuleNotEnabled();
    error LivenessModule2_ModuleStillEnabled();
    error LivenessModule2_OwnershipTransferFailed();
    error LivenessModule2_ResponsePeriodActive();
    error LivenessModule2_ResponsePeriodEnded();
    error LivenessModule2_UnauthorizedCaller();
    error SaferSafes_InsufficientLivenessResponsePeriod();
    error SemverComp_InvalidSemverParts();
    error TimelockGuard_GuardNotConfigured();
    error TimelockGuard_GuardNotEnabled();
    error TimelockGuard_GuardStillEnabled();
    error TimelockGuard_InvalidTimelockDelay();
    error TimelockGuard_InvalidVersion();
    error TimelockGuard_NotOwner();
    error TimelockGuard_TransactionAlreadyCancelled();
    error TimelockGuard_TransactionAlreadyExecuted();
    error TimelockGuard_TransactionAlreadyScheduled();
    error TimelockGuard_TransactionNotReady();
    error TimelockGuard_TransactionNotScheduled();

    event CancellationThresholdUpdated(address indexed safe, uint256 oldThreshold, uint256 newThreshold);
    event ChallengeCancelled(address indexed safe);
    event ChallengeStarted(address indexed safe, uint256 challengeStartTime);
    event ChallengeSucceeded(address indexed safe, address fallbackOwner);
    event GuardConfigured(address indexed safe, uint256 timelockDelay);
    event Message(string message);
    event ModuleCleared(address indexed safe);
    event ModuleConfigured(address indexed safe, uint256 livenessResponsePeriod, address fallbackOwner);
    event TransactionCancelled(address indexed safe, bytes32 indexed txHash);
    event TransactionExecuted(address indexed safe, bytes32 indexed txHash);
    event TransactionScheduled(address indexed safe, bytes32 indexed txHash, uint256 executionTime);

    function cancelTransaction(address _safe, bytes32 _txHash, uint256 _nonce, bytes memory _signatures) external;
    function cancellationThreshold(address _safe) external view returns (uint256);
    function challenge(address _safe) external;
    function challengeStartTime(address) external view returns (uint256);
    function changeOwnershipToFallback(address _safe) external;
    function checkAfterExecution(bytes32 _txHash, bool _success) external;
    function checkTransaction(address _to, uint256 _value, bytes memory _data, Enum.Operation _operation, uint256 _safeTxGas, uint256 _baseGas, uint256 _gasPrice, address _gasToken, address payable _refundReceiver, bytes memory, address _msgSender) external;
    function clearLivenessModule() external;
    function clearTimelockGuard() external;
    function configureLivenessModule(LivenessModule2.ModuleConfig memory _config) external;
    function configureTimelockGuard(uint256 _timelockDelay) external;
    function getChallengePeriodEnd(address _safe) external view returns (uint256);
    function livenessSafeConfiguration(address _safe) external view returns (LivenessModule2.ModuleConfig memory);
    function maxCancellationThreshold(address _safe) external view returns (uint256);
    function pendingTransactions(address _safe) external view returns (TimelockGuard.ScheduledTransaction[] memory);
    function respond() external;
    function scheduleTransaction(address _safe, uint256 _nonce, TimelockGuard.ExecTransactionParams memory _params, bytes memory _signatures) external;
    function scheduledTransaction(address _safe, bytes32 _txHash) external view returns (TimelockGuard.ScheduledTransaction memory);
    function signCancellation(bytes32) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function timelockDelay(address _safe) external view returns (uint256);
    function version() external view returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "cancelTransaction",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      },
      {
        "name": "_txHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_signatures",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cancellationThreshold",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
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
    "name": "challenge",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "challengeStartTime",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract Safe"
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
    "name": "changeOwnershipToFallback",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkAfterExecution",
    "inputs": [
      {
        "name": "_txHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_success",
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
        "name": "",
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
    "name": "clearLivenessModule",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "clearTimelockGuard",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "configureLivenessModule",
    "inputs": [
      {
        "name": "_config",
        "type": "tuple",
        "internalType": "struct LivenessModule2.ModuleConfig",
        "components": [
          {
            "name": "livenessResponsePeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fallbackOwner",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "configureTimelockGuard",
    "inputs": [
      {
        "name": "_timelockDelay",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getChallengePeriodEnd",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
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
    "name": "livenessSafeConfiguration",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct LivenessModule2.ModuleConfig",
        "components": [
          {
            "name": "livenessResponsePeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fallbackOwner",
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
    "name": "maxCancellationThreshold",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
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
    "name": "pendingTransactions",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct TimelockGuard.ScheduledTransaction[]",
        "components": [
          {
            "name": "txHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "executionTime",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "state",
            "type": "uint8",
            "internalType": "enum TimelockGuard.TransactionState"
          },
          {
            "name": "params",
            "type": "tuple",
            "internalType": "struct TimelockGuard.ExecTransactionParams",
            "components": [
              {
                "name": "to",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "value",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "data",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "operation",
                "type": "uint8",
                "internalType": "enum Enum.Operation"
              },
              {
                "name": "safeTxGas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "baseGas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "gasPrice",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "gasToken",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "refundReceiver",
                "type": "address",
                "internalType": "address payable"
              }
            ]
          },
          {
            "name": "nonce",
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
    "name": "respond",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scheduleTransaction",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      },
      {
        "name": "_nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_params",
        "type": "tuple",
        "internalType": "struct TimelockGuard.ExecTransactionParams",
        "components": [
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "value",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "operation",
            "type": "uint8",
            "internalType": "enum Enum.Operation"
          },
          {
            "name": "safeTxGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "baseGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasPrice",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "refundReceiver",
            "type": "address",
            "internalType": "address payable"
          }
        ]
      },
      {
        "name": "_signatures",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scheduledTransaction",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
      },
      {
        "name": "_txHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct TimelockGuard.ScheduledTransaction",
        "components": [
          {
            "name": "txHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "executionTime",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "state",
            "type": "uint8",
            "internalType": "enum TimelockGuard.TransactionState"
          },
          {
            "name": "params",
            "type": "tuple",
            "internalType": "struct TimelockGuard.ExecTransactionParams",
            "components": [
              {
                "name": "to",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "value",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "data",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "operation",
                "type": "uint8",
                "internalType": "enum Enum.Operation"
              },
              {
                "name": "safeTxGas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "baseGas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "gasPrice",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "gasToken",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "refundReceiver",
                "type": "address",
                "internalType": "address payable"
              }
            ]
          },
          {
            "name": "nonce",
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
    "name": "signCancellation",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
    "name": "timelockDelay",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "contract Safe"
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
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "CancellationThresholdUpdated",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "contract Safe"
      },
      {
        "name": "oldThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChallengeCancelled",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChallengeStarted",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "challengeStartTime",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChallengeSucceeded",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "fallbackOwner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "GuardConfigured",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "contract Safe"
      },
      {
        "name": "timelockDelay",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Message",
    "inputs": [
      {
        "name": "message",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ModuleCleared",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ModuleConfigured",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "livenessResponsePeriod",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "fallbackOwner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionCancelled",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "contract Safe"
      },
      {
        "name": "txHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionExecuted",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "contract Safe"
      },
      {
        "name": "txHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionScheduled",
    "inputs": [
      {
        "name": "safe",
        "type": "address",
        "indexed": true,
        "internalType": "contract Safe"
      },
      {
        "name": "txHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "executionTime",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "LivenessModule2_ChallengeAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_ChallengeDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_InvalidFallbackOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_InvalidResponsePeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_InvalidVersion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_ModuleNotConfigured",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_ModuleNotEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_ModuleStillEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_OwnershipTransferFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_ResponsePeriodActive",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_ResponsePeriodEnded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LivenessModule2_UnauthorizedCaller",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SaferSafes_InsufficientLivenessResponsePeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SemverComp_InvalidSemverParts",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_GuardNotConfigured",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_GuardNotEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_GuardStillEnabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_InvalidTimelockDelay",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_InvalidVersion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_NotOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_TransactionAlreadyCancelled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_TransactionAlreadyExecuted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_TransactionAlreadyScheduled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_TransactionNotReady",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimelockGuard_TransactionNotScheduled",
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
pub mod SaferSafes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50614657806100206000396000f3fe608060405234801561001057600080fd5b50600436106101825760003560e01c806365cf50ec116100d8578063bceb8eda1161008c578063db16aab911610066578063db16aab91461040f578063e10ffc9214610417578063e647dee11461042a57600080fd5b8063bceb8eda146103c9578063c127fd39146103dc578063c9713656146103fc57600080fd5b806375f0bb52116100bd57806375f0bb5214610384578063932713681461039757806396de45a4146103a957600080fd5b806365cf50ec1461035e57806372fb97031461037157600080fd5b806312492d971161013a5780635698b16a116101145780635698b16a146102f85780635c3b45101461034e5780636092b3181461035657600080fd5b806312492d97146101fd57806347c032231461021d57806354fd4d50146102af57600080fd5b806306d8b7e21161016b57806306d8b7e2146101c4578063096e01f7146101d75780630e40beec146101ea57600080fd5b806301ffc9a71461018757806305ccf606146101af575b600080fd5b61019a610195366004613918565b610475565b60405190151581526020015b60405180910390f35b6101c26101bd366004613a34565b61050e565b005b6101c26101d2366004613a8a565b6107b5565b6101c26101e5366004613b3f565b61086b565b6101c26101f8366004613bb1565b610dc3565b61021061020b366004613cc1565b6113e3565b6040516101a69190613eca565b61027e61022b366004613edd565b6040805180820182526000808252602091820181905273ffffffffffffffffffffffffffffffffffffffff9384168152808252829020825180840190935280548352600101549092169181019190915290565b604080518251815260209283015173ffffffffffffffffffffffffffffffffffffffff1692810192909252016101a6565b6102eb6040518060400160405280600681526020017f312e31302e31000000000000000000000000000000000000000000000000000081525081565b6040516101a69190613efa565b610340610306366004613edd565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602090815260408083206003835281842054845290915290205490565b6040519081526020016101a6565b6101c2611614565b6101c261168e565b6101c261036c366004613a8a565b61173f565b6101c261037f366004613edd565b6118de565b6101c2610392366004613f0d565b611a11565b6101c26103a5366004614000565b5050565b6103406103b7366004613edd565b60016020526000908152604090205481565b6101c26103d7366004613edd565b611e69565b6103ef6103ea366004613edd565b6126a0565b6040516101a69190614030565b61034061040a366004613edd565b6129ac565b6101c2612a44565b610340610425366004613edd565b612ac0565b610340610438366004613edd565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602090815260408083206003835281842054845290915290206001015490565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167fe6d7a83a00000000000000000000000000000000000000000000000000000000148061050857507fffffffff0000000000000000000000000000000000000000000000000000000082167f01ffc9a700000000000000000000000000000000000000000000000000000000145b92915050565b8051339060000361054b576040517f74a7e96e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602082015173ffffffffffffffffffffffffffffffffffffffff1615806105a157508073ffffffffffffffffffffffffffffffffffffffff16826020015173ffffffffffffffffffffffffffffffffffffffff16145b156105d8576040517f552dd1b700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6106a78173ffffffffffffffffffffffffffffffffffffffff1663ffa1ad746040518163ffffffff1660e01b8152600401600060405180830381865afa158015610626573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261066c91908101906140e0565b6040518060400160405280600581526020017f312e342e31000000000000000000000000000000000000000000000000000000815250612b27565b6106dd576040517fda21aed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6106e681612b77565b73ffffffffffffffffffffffffffffffffffffffff8181166000908152602081815260409091208451815590840151600190910180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169190921617905561074e81612c3e565b8151602083015160405133927f47e2ddae3ece2c77f60d8e9e5a89a50f1d1374a87fb11956d27a6b4986bba17e926107a49291825273ffffffffffffffffffffffffffffffffffffffff16602082015260400190565b60405180910390a26103a581612cbe565b7f51a7f65c6325882f237d4aeb43228179cfad48b868511d508e24b4437a8191376040516108609060208082526050908201527f546869732066756e6374696f6e206973206e6f74206d65616e7420746f20626560408201527f2063616c6c65642c2064696420796f75206d65616e20746f2063616c6c20636160608201527f6e63656c5472616e73616374696f6e3f00000000000000000000000000000000608082015260a00190565b60405180910390a150565b600273ffffffffffffffffffffffffffffffffffffffff851660009081526002602090815260408083206003835281842054845290915290206000858152600291820160205260409020015460ff1660038111156108cb576108cb613ced565b03610902576040517f3e8b838900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8416600090815260026020818152604080842060038084528286205486529083528185208886528401909252909220015460ff168181111561095b5761095b613ced565b03610992576040517f183ca43100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff841660009081526002602090815260408083206003835281842054845290915281206000858152600291820160205260409020015460ff1660038111156109f0576109f0613ced565b03610a27576040517f03c8597300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60003073ffffffffffffffffffffffffffffffffffffffff166306d8b7e285604051602401610a5891815260200190565b604051602081830303815290604052915060e01b6020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008573ffffffffffffffffffffffffffffffffffffffff1663e86637db306000856000806000806000808e6040518b63ffffffff1660e01b8152600401610aed9a99989796959493929190614129565b600060405180830381865afa158015610b0a573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610b5091908101906140e0565b905060008673ffffffffffffffffffffffffffffffffffffffff1663d8d11f78306000866000806000806000808f6040518b63ffffffff1660e01b8152600401610ba39a99989796959493929190614129565b602060405180830381865afa158015610bc0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610be491906141a9565b90508673ffffffffffffffffffffffffffffffffffffffff166312fb68e0828487610c428c73ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b600101546040518563ffffffff1660e01b8152600401610c6594939291906141c2565b60006040518083038186803b158015610c7d57600080fd5b505afa158015610c91573d6000803e3d6000fd5b505050506002610cd48873ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b60008881526002918201602052604090200180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001836003811115610d1e57610d1e613ced565b0217905550610d6d86610d648973ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b60030190612e08565b50610d7787612e1b565b604051869073ffffffffffffffffffffffffffffffffffffffff8916907fa42fd857b47d3d04f5b29f35cb05343f66b317633d2dc2910726bd4bca1a162590600090a350505050505050565b610dcc84612ee1565b610e02576040517f3f4b296600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8416600090815260026020908152604080832060038352818420548452909152902054600003610e73576040517f0832dd6900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008473ffffffffffffffffffffffffffffffffffffffff1663e86637db846000015185602001518660400151876060015188608001518960a001518a60c001518b60e001518c61010001518e6040518b63ffffffff1660e01b8152600401610ee59a99989796959493929190614129565b600060405180830381865afa158015610f02573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610f4891908101906140e0565b905060008573ffffffffffffffffffffffffffffffffffffffff1663d8d11f78856000015186602001518760400151886060015189608001518a60a001518b60c001518c60e001518d61010001518f6040518b63ffffffff1660e01b8152600401610fbc9a99989796959493929190614129565b602060405180830381865afa158015610fd9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ffd91906141a9565b905061103c8673ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b6000828152600291909101602052604090206001015415611089576040517f80394de600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f934f3a1100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff87169063934f3a11906110df908490869088906004016141ff565b60006040518083038186803b1580156110f757600080fd5b505afa15801561110b573d6000803e3d6000fd5b50505050600061114e8773ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b546111599042614263565b6040805160a08101825284815260208082018481526001838501818152606085018c9052608085018d905273ffffffffffffffffffffffffffffffffffffffff8e166000908152600280865287822060038088528984205484529087528883208c84528201909652969020855181559251838301555194820180549697509395919493927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0090921691849081111561121357611213613ced565b0217905550606082015180516003830180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff90921691909117815560208201516004840155604082015160058401906112839082614316565b5060608201516003820180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600183818111156112c4576112c4613ced565b0217905550608082810151600483015560a0830151600583015560c0830151600683015560e08301516007830180547fffffffffffffffffffffffff000000000000000000000000000000000000000090811673ffffffffffffffffffffffffffffffffffffffff93841617909155610100909401516008909301805490941692811692909217909255920151600c909101558716600090815260026020908152604080832060038352818420548452909152902061138890839060030190612fef565b50818773ffffffffffffffffffffffffffffffffffffffff167f653f8f6fce2a503b2dfca34e95b3e4902254a11765d2658c0e5af1d64ab276cf836040516113d291815260200190565b60405180910390a350505050505050565b6113eb613844565b73ffffffffffffffffffffffffffffffffffffffff8316600090815260026020818152604080842060038084528286205486529083528185208786528401835293819020815160a081018352815481526001820154938101939093529283015491939084019160ff169081111561146457611464613ced565b600381111561147557611475613ced565b815260200160038201604051806101200160405290816000820160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015481526020016002820180546114fa9061427b565b80601f01602080910402602001604051908101604052809291908181526020018280546115269061427b565b80156115735780601f1061154857610100808354040283529160200191611573565b820191906000526020600020905b81548152906001019060200180831161155657829003601f168201915b5050509183525050600382015460209091019060ff16600181111561159a5761159a613ced565b60018111156115ab576115ab613ced565b815260048201546020808301919091526005830154604083015260068301546060830152600783015473ffffffffffffffffffffffffffffffffffffffff908116608084015260089093015490921660a090910152908252600c92909201549101529392505050565b3361161e81612ee1565b15611655576040517fa4d234cb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600090815260036020526040812080549161168683614430565b919050555050565b3361169881612ffb565b6116a18161305f565b73ffffffffffffffffffffffffffffffffffffffff8116600090815260208190526040812090815560010180547fffffffffffffffffffffffff00000000000000000000000000000000000000001690556116fb81612c3e565b60405173ffffffffffffffffffffffffffffffffffffffff8216907f9c6ff02e684f8a81389ed942fea1147c16b8cc5fe79f2bfbc520d44e214aed4b90600090a250565b60003390506117928173ffffffffffffffffffffffffffffffffffffffff1663ffa1ad746040518163ffffffff1660e01b8152600401600060405180830381865afa158015610626573d6000803e3d6000fd5b6117c8576040517f9e2f7c4b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117d181612ee1565b611807576040517f3f4b296600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81158061181757506301e1338082115b1561184e576040517fa0ce228b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8161188c8273ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b5561189681613124565b8073ffffffffffffffffffffffffffffffffffffffff167fa48d13ee8fad9974fa901cfb88a02d39c5361efbab13bb9b3aa7caa3f6d6b786836040516107a491815260200190565b6118e781612ffb565b6118f081612b77565b73ffffffffffffffffffffffffffffffffffffffff818116600090815260208190526040902060010154163314611953576040517fa1f2082000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600090815260016020526040902054156119b0576040517f9cd9090400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020908152604091829020429081905591519182527f3eb9241ca06793ab672590e858d0977206f824e7367806f94a90af391d275d33910160405180910390a250565b33600081815260026020908152604080832060038352818420548452909152902054600003611a405750611e5c565b6040517f2f54bf6e00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8381166004830152821690632f54bf6e90602401602060405180830381865afa158015611aac573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ad09190614468565b611b06576040517f5874e94f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600060018273ffffffffffffffffffffffffffffffffffffffff1663affed0e06040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b55573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b7991906141a9565b611b839190614485565b905060008273ffffffffffffffffffffffffffffffffffffffff1663d8d11f788f8f8f8f8f8f8f8f8f8c6040518b63ffffffff1660e01b8152600401611bd29a99989796959493929190614129565b602060405180830381865afa158015611bef573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c1391906141a9565b90506000611c548473ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b60008381526002918201602052604090209150600282015460ff166003811115611c8057611c80613ced565b03611cb7576040517f3e8b838900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6003600282015460ff166003811115611cd257611cd2613ced565b03611d09576040517f183ca43100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000600282015460ff166003811115611d2457611d24613ced565b03611d5b576040517f03c8597300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b4281600101541115611d99576040517f503c42c400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611da284613124565b600281810180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600390811790915573ffffffffffffffffffffffffffffffffffffffff8616600090815260209283526040808220928452808220548252919092529020611e13908390610d64565b50604051829073ffffffffffffffffffffffffffffffffffffffff8616907fdd4b9b318b98162cb1e7b52752a3fd110d5b7966f3b50884c1cd3bd04058e5c790600090a3505050505b5050505050505050505050565b611e7281612ffb565b611e7b81612b77565b73ffffffffffffffffffffffffffffffffffffffff818116600090815260208190526040902060010154163314611ede576040517fa1f2082000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660009081526001602052604081205490819003611f3e576040517f09f9592000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611f4782612ac0565b421015611f80576040517f279e3d2400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600160008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000206000905560008273ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612010573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052612056919081019061449c565b90505b600181511115612261578273ffffffffffffffffffffffffffffffffffffffff1663468721a78460006001856000815181106120975761209761454e565b602090810291909101015160405173ffffffffffffffffffffffffffffffffffffffff92831660248201529116604482015260016064820152608401604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167ff8dc5dd900000000000000000000000000000000000000000000000000000000179052517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b1681526121859392919060009060040161457d565b6020604051808303816000875af11580156121a4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121c89190614468565b508273ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612214573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261225a919081019061449c565b9050612059565b8273ffffffffffffffffffffffffffffffffffffffff1663468721a78460006001856000815181106122955761229561454e565b60209081029190910181015173ffffffffffffffffffffffffffffffffffffffff8a811660009081529283905260409283902060010154925193811660248501529081166044840152166064820152608401604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe318b52b00000000000000000000000000000000000000000000000000000000179052517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b1681526123999392919060009060040161457d565b6020604051808303816000875af11580156123b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123dc9190614468565b5060008373ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa15801561242a573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052612470919081019061449c565b9050805160011415806124dd575073ffffffffffffffffffffffffffffffffffffffff80851660009081526020819052604081206001015483519216918391906124bc576124bc61454e565b602002602001015173ffffffffffffffffffffffffffffffffffffffff1614155b15612514576040517fe330ac0500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405160006024820181905273ffffffffffffffffffffffffffffffffffffffff86169163468721a7918791604401604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe19a9dd900000000000000000000000000000000000000000000000000000000179052517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b1681526125f59392919060009060040161457d565b6020604051808303816000875af1158015612614573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126389190614468565b5073ffffffffffffffffffffffffffffffffffffffff8481166000818152602081815260409182902060010154915191909316815290917fdfdecb71eb0580c9263c867bc6de9dd6f859cc6a6ee33d47e505904f1d5601c9910160405180910390a250505050565b606060006126e18373ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b905060006126f1826003016131a1565b90506000815167ffffffffffffffff81111561270f5761270f61395a565b60405190808252806020026020018201604052801561274857816020015b612735613844565b81526020019060019003908161272d5790505b50905060005b82518110156129a35783600201600084838151811061276f5761276f61454e565b602002602001015181526020019081526020016000206040518060a001604052908160008201548152602001600182015481526020016002820160009054906101000a900460ff1660038111156127c8576127c8613ced565b60038111156127d9576127d9613ced565b815260200160038201604051806101200160405290816000820160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016001820154815260200160028201805461285e9061427b565b80601f016020809104026020016040519081016040528092919081815260200182805461288a9061427b565b80156128d75780601f106128ac576101008083540402835291602001916128d7565b820191906000526020600020905b8154815290600101906020018083116128ba57829003601f168201915b5050509183525050600382015460209091019060ff1660018111156128fe576128fe613ced565b600181111561290f5761290f613ced565b815260048201546020808301919091526005830154604083015260068301546060830152600783015473ffffffffffffffffffffffffffffffffffffffff908116608084015260089093015490921660a090910152908252600c929092015491015282518390839081106129855761298561454e565b6020026020010181905250808061299b90614430565b91505061274e565b50949350505050565b6000806129b8836131ac565b905060008373ffffffffffffffffffffffffffffffffffffffff1663e75235b86040518163ffffffff1660e01b8152600401602060405180830381865afa158015612a07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a2b91906141a9565b9050808210612a3a5780612a3c565b815b949350505050565b33612a4e81612ffb565b612a5781612b77565b73ffffffffffffffffffffffffffffffffffffffff811660009081526001602052604081205490819003612ab7576040517f09f9592000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6103a582612c3e565b73ffffffffffffffffffffffffffffffffffffffff8116600090815260016020526040812054808203612af65750600092915050565b73ffffffffffffffffffffffffffffffffffffffff831660009081526020819052604090208054612a3c9083614263565b600080612b33846132c4565b90506000612b40846132c4565b80518351919250148015612b5b575080602001518260200151145b8015612b6e575080604001518260400151145b95945050505050565b6040517f2d9ad53d00000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff821690632d9ad53d90602401602060405180830381865afa158015612be1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c059190614468565b612c3b576040517f9fdada3100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b73ffffffffffffffffffffffffffffffffffffffff81166000908152600160205260408120549003612c6d5750565b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020526040808220829055517fadfa6511c5a9b5ac1adc5e4e0ca637c465f24ab56c1bd255a083e0023e3b7c429190a250565b612cc781612ee1565b8015612d5c57506040517f2d9ad53d00000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff821690632d9ad53d90602401602060405180830381865afa158015612d38573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d5c9190614468565b612d635750565b73ffffffffffffffffffffffffffffffffffffffff81166000818152600260209081526040808320600383528184205484528252808320549383529082905281205490829003612db257505050565b80600003612dbf57505050565b612dca8260026145c1565b811015612e03576040517f47dca1ce00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050565b6000612e148383613490565b9392505050565b73ffffffffffffffffffffffffffffffffffffffff81166000908152600260209081526040808320600383528184205484529091529020612e5b826129ac565b816001015410156103a5576001810180549081906000612e7a83614430565b91905055508273ffffffffffffffffffffffffffffffffffffffff167f4eda179760b4e68650058376d4acef78e953221317450d29b4920e6d2836944c828460010154604051612ed4929190918252602082015260400190565b60405180910390a2505050565b6040517f5624b25b0000000000000000000000000000000000000000000000000000000081527f4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c8600482015260016024820152600090819073ffffffffffffffffffffffffffffffffffffffff841690635624b25b90604401600060405180830381865afa158015612f77573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052612fbd91908101906140e0565b806020019051810190612fd091906145fe565b73ffffffffffffffffffffffffffffffffffffffff1630149392505050565b6000612e14838361358a565b73ffffffffffffffffffffffffffffffffffffffff808216600090815260208190526040902060018101549091166103a5576040517fa0fe939b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f2d9ad53d00000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff821690632d9ad53d90602401602060405180830381865afa1580156130c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906130ed9190614468565b15612c3b576040517f70a3809400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660008181526002602090815260408083206003835281842054845282529182902060018082018054908290558451818152938401919091529093909290917f4eda179760b4e68650058376d4acef78e953221317450d29b4920e6d2836944c9101612ed4565b6060610508826135d9565b60008173ffffffffffffffffffffffffffffffffffffffff1663e75235b86040518163ffffffff1660e01b8152600401602060405180830381865afa1580156131f9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061321d91906141a9565b8273ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015613268573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526132ae919081019061449c565b516132b99190614485565b610508906001614263565b6132e860405180606001604052806000815260200160008152602001600081525090565b6000613329836040518060400160405280600181526020017f2e00000000000000000000000000000000000000000000000000000000000000815250613635565b9050600381511015613367576040517f9eda858c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006133c28260028151811061337f5761337f61454e565b60200260200101516040518060400160405280600181526020017f2d00000000000000000000000000000000000000000000000000000000000000815250613635565b9050600061341f826000815181106133dc576133dc61454e565b60200260200101516040518060400160405280600181526020017f2b00000000000000000000000000000000000000000000000000000000000000815250613635565b9050604051806060016040528061344f856000815181106134425761344261454e565b60200260200101516136df565b815260200161346a856001815181106134425761344261454e565b8152602001613485836000815181106134425761344261454e565b905295945050505050565b600081815260018301602052604081205480156135795760006134b4600183614485565b85549091506000906134c890600190614485565b905081811461352d5760008660000182815481106134e8576134e861454e565b906000526020600020015490508087600001848154811061350b5761350b61454e565b6000918252602080832090910192909255918252600188019052604090208390555b855486908061353e5761353e61461b565b600190038181906000526020600020016000905590558560010160008681526020019081526020016000206000905560019350505050610508565b6000915050610508565b5092915050565b60008181526001830160205260408120546135d157508154600181810184556000848152602080822090930184905584548482528286019093526040902091909155610508565b506000610508565b60608160000180548060200260200160405190810160405280929190818152602001828054801561362957602002820191906000526020600020905b815481526020019060010190808311613615575b50505050509050919050565b606060006136438484613757565b9050601f1960208201600183510160051b81018651838201526001845101845260005b8251606084528181146136ab5760405182820380825286601f8201165b8b8501810151838201528701806136835750600082820160200152603f018616810160405284525b8751602094909401930190508183106136665750505050809150825161358357602081019150600281510382525092915050565b80516000907f1999999999999999999999999999999999999999999999999999999999999999825b600181019050603060ff82870151160382851185600a028281019650600983118188108317171586029550505050828110613707575050806137515763101827966000526004601cfd5b50919050565b60608251825181811161383c57602085019450602084019350602060405101925084600182848801030160006020841061379057508286205b601f841660200360031b87515b8951818118831c6137f25783156137d05783878c20146137d05760018b019a50848b106137ca5750613801565b5061379d565b858b0389529986019960209098019786156137f257848b106137ca5750613801565b5060018a019950838a1061379d575b5050604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08189030160051c8152602090970190525050505b505092915050565b6040805160a0810182526000808252602082018190529091820190815260200161386c613879565b8152602001600081525090565b604051806101200160405280600073ffffffffffffffffffffffffffffffffffffffff1681526020016000815260200160608152602001600060018111156138c3576138c3613ced565b8152602001600081526020016000815260200160008152602001600073ffffffffffffffffffffffffffffffffffffffff168152602001600073ffffffffffffffffffffffffffffffffffffffff1681525090565b60006020828403121561392a57600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114612e1457600080fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051610120810167ffffffffffffffff811182821017156139ad576139ad61395a565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156139fa576139fa61395a565b604052919050565b73ffffffffffffffffffffffffffffffffffffffff81168114612c3b57600080fd5b8035613a2f81613a02565b919050565b600060408284031215613a4657600080fd5b6040516040810181811067ffffffffffffffff82111715613a6957613a6961395a565b604052823581526020830135613a7e81613a02565b60208201529392505050565b600060208284031215613a9c57600080fd5b5035919050565b600067ffffffffffffffff821115613abd57613abd61395a565b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b600082601f830112613afa57600080fd5b8135613b0d613b0882613aa3565b6139b3565b818152846020838601011115613b2257600080fd5b816020850160208301376000918101602001919091529392505050565b60008060008060808587031215613b5557600080fd5b8435613b6081613a02565b93506020850135925060408501359150606085013567ffffffffffffffff811115613b8a57600080fd5b613b9687828801613ae9565b91505092959194509250565b803560028110613a2f57600080fd5b60008060008060808587031215613bc757600080fd5b8435613bd281613a02565b935060208501359250604085013567ffffffffffffffff80821115613bf657600080fd5b908601906101208289031215613c0b57600080fd5b613c13613989565b613c1c83613a24565b815260208301356020820152604083013582811115613c3a57600080fd5b613c468a828601613ae9565b604083015250613c5860608401613ba2565b60608201526080830135608082015260a083013560a082015260c083013560c0820152613c8760e08401613a24565b60e0820152610100613c9a818501613a24565b9082015293506060870135915080821115613cb457600080fd5b50613b9687828801613ae9565b60008060408385031215613cd457600080fd5b8235613cdf81613a02565b946020939093013593505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b60005b83811015613d37578181015183820152602001613d1f565b83811115613d46576000848401525b50505050565b60008151808452613d64816020860160208601613d1c565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b60028110613da657613da6613ced565b9052565b80518252602081015160208301526000604082015160048110613dcf57613dcf613ced565b80604085015250606082015160a06060850152613e0560a08501825173ffffffffffffffffffffffffffffffffffffffff169052565b602081015160c085015260408101516101208060e0870152613e2b6101c0870183613d4c565b91506060830151610100613e4181890183613d96565b60808501518389015260a085015161014089015260c085015161016089015260e08501519250613e8a61018089018473ffffffffffffffffffffffffffffffffffffffff169052565b939093015173ffffffffffffffffffffffffffffffffffffffff81166101a08801529250613eb59050565b60808401516080860152809250505092915050565b602081526000612e146020830184613daa565b600060208284031215613eef57600080fd5b8135612e1481613a02565b602081526000612e146020830184613d4c565b60008060008060008060008060008060006101608c8e031215613f2f57600080fd5b613f388c613a24565b9a5060208c0135995067ffffffffffffffff8060408e01351115613f5b57600080fd5b613f6b8e60408f01358f01613ae9565b9950613f7960608e01613ba2565b985060808d0135975060a08d0135965060c08d01359550613f9c60e08e01613a24565b9450613fab6101008e01613a24565b9350806101208e01351115613fbf57600080fd5b50613fd18d6101208e01358e01613ae9565b9150613fe06101408d01613a24565b90509295989b509295989b9093969950565b8015158114612c3b57600080fd5b6000806040838503121561401357600080fd5b82359150602083013561402581613ff2565b809150509250929050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b828110156140a3577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0888603018452614091858351613daa565b94509285019290850190600101614057565b5092979650505050505050565b60006140be613b0884613aa3565b90508281528383830111156140d257600080fd5b612e14836020830184613d1c565b6000602082840312156140f257600080fd5b815167ffffffffffffffff81111561410957600080fd5b8201601f8101841361411a57600080fd5b612a3c848251602084016140b0565b600061014073ffffffffffffffffffffffffffffffffffffffff808e1684528c60208501528160408501526141608285018d613d4c565b925061416f606085018c613d96565b60808401999099525060a082019690965260c081019490945291851660e08401529093166101008201526101200191909152949350505050565b6000602082840312156141bb57600080fd5b5051919050565b8481526080602082015260006141db6080830186613d4c565b82810360408401526141ed8186613d4c565b91505082606083015295945050505050565b8381526060602082015260006142186060830185613d4c565b828103604084015261422a8185613d4c565b9695505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6000821982111561427657614276614234565b500190565b600181811c9082168061428f57607f821691505b602082108103613751577f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b601f821115612e0357600081815260208120601f850160051c810160208610156142ef5750805b601f850160051c820191505b8181101561430e578281556001016142fb565b505050505050565b815167ffffffffffffffff8111156143305761433061395a565b6143448161433e845461427b565b846142c8565b602080601f83116001811461439757600084156143615750858301515b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600386901b1c1916600185901b17855561430e565b6000858152602081207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08616915b828110156143e4578886015182559484019460019091019084016143c5565b508582101561442057878501517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600388901b60f8161c191681555b5050505050600190811b01905550565b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361446157614461614234565b5060010190565b60006020828403121561447a57600080fd5b8151612e1481613ff2565b60008282101561449757614497614234565b500390565b600060208083850312156144af57600080fd5b825167ffffffffffffffff808211156144c757600080fd5b818501915085601f8301126144db57600080fd5b8151818111156144ed576144ed61395a565b8060051b91506144fe8483016139b3565b818152918301840191848101908884111561451857600080fd5b938501935b83851015614542578451925061453283613a02565b828252938501939085019061451d565b98975050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b73ffffffffffffffffffffffffffffffffffffffff851681528360208201526080604082015260006145b26080830185613d4c565b9050612b6e6060830184613d96565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156145f9576145f9614234565b500290565b60006020828403121561461057600080fd5b8151612e1481613a02565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603160045260246000fdfea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaFW\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x82W`\x005`\xE0\x1C\x80ce\xCFP\xEC\x11a\0\xD8W\x80c\xBC\xEB\x8E\xDA\x11a\0\x8CW\x80c\xDB\x16\xAA\xB9\x11a\0fW\x80c\xDB\x16\xAA\xB9\x14a\x04\x0FW\x80c\xE1\x0F\xFC\x92\x14a\x04\x17W\x80c\xE6G\xDE\xE1\x14a\x04*W`\0\x80\xFD[\x80c\xBC\xEB\x8E\xDA\x14a\x03\xC9W\x80c\xC1'\xFD9\x14a\x03\xDCW\x80c\xC9q6V\x14a\x03\xFCW`\0\x80\xFD[\x80cu\xF0\xBBR\x11a\0\xBDW\x80cu\xF0\xBBR\x14a\x03\x84W\x80c\x93'\x13h\x14a\x03\x97W\x80c\x96\xDEE\xA4\x14a\x03\xA9W`\0\x80\xFD[\x80ce\xCFP\xEC\x14a\x03^W\x80cr\xFB\x97\x03\x14a\x03qW`\0\x80\xFD[\x80c\x12I-\x97\x11a\x01:W\x80cV\x98\xB1j\x11a\x01\x14W\x80cV\x98\xB1j\x14a\x02\xF8W\x80c\\;E\x10\x14a\x03NW\x80c`\x92\xB3\x18\x14a\x03VW`\0\x80\xFD[\x80c\x12I-\x97\x14a\x01\xFDW\x80cG\xC02#\x14a\x02\x1DW\x80cT\xFDMP\x14a\x02\xAFW`\0\x80\xFD[\x80c\x06\xD8\xB7\xE2\x11a\x01kW\x80c\x06\xD8\xB7\xE2\x14a\x01\xC4W\x80c\tn\x01\xF7\x14a\x01\xD7W\x80c\x0E@\xBE\xEC\x14a\x01\xEAW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x87W\x80c\x05\xCC\xF6\x06\x14a\x01\xAFW[`\0\x80\xFD[a\x01\x9Aa\x01\x956`\x04a9\x18V[a\x04uV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xC2a\x01\xBD6`\x04a:4V[a\x05\x0EV[\0[a\x01\xC2a\x01\xD26`\x04a:\x8AV[a\x07\xB5V[a\x01\xC2a\x01\xE56`\x04a;?V[a\x08kV[a\x01\xC2a\x01\xF86`\x04a;\xB1V[a\r\xC3V[a\x02\x10a\x02\x0B6`\x04a<\xC1V[a\x13\xE3V[`@Qa\x01\xA6\x91\x90a>\xCAV[a\x02~a\x02+6`\x04a>\xDDV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01\x81\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x80\x82R\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xA6V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7F1.10.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xA6\x91\x90a>\xFAV[a\x03@a\x03\x066`\x04a>\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 T\x90V[`@Q\x90\x81R` \x01a\x01\xA6V[a\x01\xC2a\x16\x14V[a\x01\xC2a\x16\x8EV[a\x01\xC2a\x03l6`\x04a:\x8AV[a\x17?V[a\x01\xC2a\x03\x7F6`\x04a>\xDDV[a\x18\xDEV[a\x01\xC2a\x03\x926`\x04a?\rV[a\x1A\x11V[a\x01\xC2a\x03\xA56`\x04a@\0V[PPV[a\x03@a\x03\xB76`\x04a>\xDDV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xC2a\x03\xD76`\x04a>\xDDV[a\x1EiV[a\x03\xEFa\x03\xEA6`\x04a>\xDDV[a&\xA0V[`@Qa\x01\xA6\x91\x90a@0V[a\x03@a\x04\n6`\x04a>\xDDV[a)\xACV[a\x01\xC2a*DV[a\x03@a\x04%6`\x04a>\xDDV[a*\xC0V[a\x03@a\x0486`\x04a>\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 `\x01\x01T\x90V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE6\xD7\xA8:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x05\x08WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x92\x91PPV[\x80Q3\x90`\0\x03a\x05KW`@Q\x7Ft\xA7\xE9n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80a\x05\xA1WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x15a\x05\xD8W`@Q\x7FU-\xD1\xB7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\xA7\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xA1\xADt`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x06l\x91\x90\x81\x01\x90a@\xE0V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.4.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa+'V[a\x06\xDDW`@Q\x7F\xDA!\xAE\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\xE6\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16`\0\x90\x81R` \x81\x81R`@\x90\x91 \x84Q\x81U\x90\x84\x01Q`\x01\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x92\x16\x17\x90Ua\x07N\x81a,>V[\x81Q` \x83\x01Q`@Q3\x92\x7FG\xE2\xDD\xAE>\xCE,w\xF6\r\x8E\x9EZ\x89\xA5\x0F\x1D\x13t\xA8\x7F\xB1\x19V\xD2zkI\x86\xBB\xA1~\x92a\x07\xA4\x92\x91\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x03\xA5\x81a,\xBEV[\x7FQ\xA7\xF6\\c%\x88/#}J\xEBC\"\x81y\xCF\xADH\xB8hQ\x1DP\x8E$\xB4Cz\x81\x917`@Qa\x08`\x90` \x80\x82R`P\x90\x82\x01R\x7FThis function is not meant to be`@\x82\x01R\x7F called, did you mean to call ca``\x82\x01R\x7FncelTransaction?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1PV[`\x02s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 `\0\x85\x81R`\x02\x91\x82\x01` R`@\x90 \x01T`\xFF\x16`\x03\x81\x11\x15a\x08\xCBWa\x08\xCBa<\xEDV[\x03a\t\x02W`@Q\x7F>\x8B\x83\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 `\x03\x80\x84R\x82\x86 T\x86R\x90\x83R\x81\x85 \x88\x86R\x84\x01\x90\x92R\x90\x92 \x01T`\xFF\x16\x81\x81\x11\x15a\t[Wa\t[a<\xEDV[\x03a\t\x92W`@Q\x7F\x18<\xA41\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x81 `\0\x85\x81R`\x02\x91\x82\x01` R`@\x90 \x01T`\xFF\x16`\x03\x81\x11\x15a\t\xF0Wa\t\xF0a<\xEDV[\x03a\n'W`@Q\x7F\x03\xC8Ys\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x06\xD8\xB7\xE2\x85`@Q`$\x01a\nX\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP\x90P`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE8f7\xDB0`\0\x85`\0\x80`\0\x80`\0\x80\x8E`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xED\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0BP\x91\x90\x81\x01\x90a@\xE0V[\x90P`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx0`\0\x86`\0\x80`\0\x80`\0\x80\x8F`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xA3\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE4\x91\x90aA\xA9V[\x90P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x12\xFBh\xE0\x82\x84\x87a\x0CB\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\x01\x01T`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ce\x94\x93\x92\x91\x90aA\xC2V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C}W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0C\x91W=`\0\x80>=`\0\xFD[PPPP`\x02a\x0C\xD4\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\0\x88\x81R`\x02\x91\x82\x01` R`@\x90 \x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x03\x81\x11\x15a\r\x1EWa\r\x1Ea<\xEDV[\x02\x17\x90UPa\rm\x86a\rd\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\x03\x01\x90a.\x08V[Pa\rw\x87a.\x1BV[`@Q\x86\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90\x7F\xA4/\xD8W\xB4}=\x04\xF5\xB2\x9F5\xCB\x054?f\xB3\x17c=-\xC2\x91\x07&\xBDK\xCA\x1A\x16%\x90`\0\x90\xA3PPPPPPPV[a\r\xCC\x84a.\xE1V[a\x0E\x02W`@Q\x7F?K)f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 T`\0\x03a\x0EsW`@Q\x7F\x082\xDDi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE8f7\xDB\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q\x8A`\xC0\x01Q\x8B`\xE0\x01Q\x8Ca\x01\0\x01Q\x8E`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xE5\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0FH\x91\x90\x81\x01\x90a@\xE0V[\x90P`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x88``\x01Q\x89`\x80\x01Q\x8A`\xA0\x01Q\x8B`\xC0\x01Q\x8C`\xE0\x01Q\x8Da\x01\0\x01Q\x8F`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xBC\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFD\x91\x90aA\xA9V[\x90Pa\x10<\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\0\x82\x81R`\x02\x91\x90\x91\x01` R`@\x90 `\x01\x01T\x15a\x10\x89W`@Q\x7F\x809M\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x93O:\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90c\x93O:\x11\x90a\x10\xDF\x90\x84\x90\x86\x90\x88\x90`\x04\x01aA\xFFV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xF7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x11\x0BW=`\0\x80>=`\0\xFD[PPPP`\0a\x11N\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[Ta\x11Y\x90BaBcV[`@\x80Q`\xA0\x81\x01\x82R\x84\x81R` \x80\x82\x01\x84\x81R`\x01\x83\x85\x01\x81\x81R``\x85\x01\x8C\x90R`\x80\x85\x01\x8D\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8E\x16`\0\x90\x81R`\x02\x80\x86R\x87\x82 `\x03\x80\x88R\x89\x84 T\x84R\x90\x87R\x88\x83 \x8C\x84R\x82\x01\x90\x96R\x96\x90 \x85Q\x81U\x92Q\x83\x83\x01UQ\x94\x82\x01\x80T\x96\x97P\x93\x95\x91\x94\x93\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x92\x16\x91\x84\x90\x81\x11\x15a\x12\x13Wa\x12\x13a<\xEDV[\x02\x17\x90UP``\x82\x01Q\x80Q`\x03\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x81U` \x82\x01Q`\x04\x84\x01U`@\x82\x01Q`\x05\x84\x01\x90a\x12\x83\x90\x82aC\x16V[P``\x82\x01Q`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83\x81\x81\x11\x15a\x12\xC4Wa\x12\xC4a<\xEDV[\x02\x17\x90UP`\x80\x82\x81\x01Q`\x04\x83\x01U`\xA0\x83\x01Q`\x05\x83\x01U`\xC0\x83\x01Q`\x06\x83\x01U`\xE0\x83\x01Q`\x07\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x17\x90\x91Ua\x01\0\x90\x94\x01Q`\x08\x90\x93\x01\x80T\x90\x94\x16\x92\x81\x16\x92\x90\x92\x17\x90\x92U\x92\x01Q`\x0C\x90\x91\x01U\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 a\x13\x88\x90\x83\x90`\x03\x01\x90a/\xEFV[P\x81\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fe?\x8Fo\xCE*P;-\xFC\xA3N\x95\xB3\xE4\x90\"T\xA1\x17e\xD2e\x8C\x0EZ\xF1\xD6J\xB2v\xCF\x83`@Qa\x13\xD2\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[a\x13\xEBa8DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 `\x03\x80\x84R\x82\x86 T\x86R\x90\x83R\x81\x85 \x87\x86R\x84\x01\x83R\x93\x81\x90 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R\x92\x83\x01T\x91\x93\x90\x84\x01\x91`\xFF\x16\x90\x81\x11\x15a\x14dWa\x14da<\xEDV[`\x03\x81\x11\x15a\x14uWa\x14ua<\xEDV[\x81R` \x01`\x03\x82\x01`@Q\x80a\x01 \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x14\xFA\x90aB{V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15&\x90aB{V[\x80\x15a\x15sW\x80`\x1F\x10a\x15HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15sV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15VW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T` \x90\x91\x01\x90`\xFF\x16`\x01\x81\x11\x15a\x15\x9AWa\x15\x9Aa<\xEDV[`\x01\x81\x11\x15a\x15\xABWa\x15\xABa<\xEDV[\x81R`\x04\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x05\x83\x01T`@\x83\x01R`\x06\x83\x01T``\x83\x01R`\x07\x83\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x08\x90\x93\x01T\x90\x92\x16`\xA0\x90\x91\x01R\x90\x82R`\x0C\x92\x90\x92\x01T\x91\x01R\x93\x92PPPV[3a\x16\x1E\x81a.\xE1V[\x15a\x16UW`@Q\x7F\xA4\xD24\xCB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x91a\x16\x86\x83aD0V[\x91\x90PUPPV[3a\x16\x98\x81a/\xFBV[a\x16\xA1\x81a0_V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 \x90\x81U`\x01\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90Ua\x16\xFB\x81a,>V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\x9Co\xF0.hO\x8A\x818\x9E\xD9B\xFE\xA1\x14|\x16\xB8\xCC_\xE7\x9F+\xFB\xC5 \xD4N!J\xEDK\x90`\0\x90\xA2PV[`\x003\x90Pa\x17\x92\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xA1\xADt`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06&W=`\0\x80>=`\0\xFD[a\x17\xC8W`@Q\x7F\x9E/|K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xD1\x81a.\xE1V[a\x18\x07W`@Q\x7F?K)f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15\x80a\x18\x17WPc\x01\xE13\x80\x82\x11[\x15a\x18NW`@Q\x7F\xA0\xCE\"\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x18\x8C\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[Ua\x18\x96\x81a1$V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xA4\x8D\x13\xEE\x8F\xAD\x99t\xFA\x90\x1C\xFB\x88\xA0-9\xC56\x1E\xFB\xAB\x13\xBB\x9B:\xA7\xCA\xA3\xF6\xD6\xB7\x86\x83`@Qa\x07\xA4\x91\x81R` \x01\x90V[a\x18\xE7\x81a/\xFBV[a\x18\xF0\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x163\x14a\x19SW`@Q\x7F\xA1\xF2\x08 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x19\xB0W`@Q\x7F\x9C\xD9\t\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 B\x90\x81\x90U\x91Q\x91\x82R\x7F>\xB9$\x1C\xA0g\x93\xABg%\x90\xE8X\xD0\x97r\x06\xF8$\xE76x\x06\xF9J\x90\xAF9\x1D']3\x91\x01`@Q\x80\x91\x03\x90\xA2PV[3`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 T`\0\x03a\x1A@WPa\x1E\\V[`@Q\x7F/T\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x82\x16\x90c/T\xBFn\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xD0\x91\x90aDhV[a\x1B\x06W`@Q\x7FXt\xE9O\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAF\xFE\xD0\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1By\x91\x90aA\xA9V[a\x1B\x83\x91\x90aD\x85V[\x90P`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8C`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xD2\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x13\x91\x90aA\xA9V[\x90P`\0a\x1CT\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\0\x83\x81R`\x02\x91\x82\x01` R`@\x90 \x91P`\x02\x82\x01T`\xFF\x16`\x03\x81\x11\x15a\x1C\x80Wa\x1C\x80a<\xEDV[\x03a\x1C\xB7W`@Q\x7F>\x8B\x83\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\x02\x82\x01T`\xFF\x16`\x03\x81\x11\x15a\x1C\xD2Wa\x1C\xD2a<\xEDV[\x03a\x1D\tW`@Q\x7F\x18<\xA41\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x82\x01T`\xFF\x16`\x03\x81\x11\x15a\x1D$Wa\x1D$a<\xEDV[\x03a\x1D[W`@Q\x7F\x03\xC8Ys\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`\x01\x01T\x11\x15a\x1D\x99W`@Q\x7FP<B\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xA2\x84a1$V[`\x02\x81\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x03\x90\x81\x17\x90\x91Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R` \x92\x83R`@\x80\x82 \x92\x84R\x80\x82 T\x82R\x91\x90\x92R\x90 a\x1E\x13\x90\x83\x90a\rdV[P`@Q\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x7F\xDDK\x9B1\x8B\x98\x16,\xB1\xE7\xB5'R\xA3\xFD\x11\r[yf\xF3\xB5\x08\x84\xC1\xCD;\xD0@X\xE5\xC7\x90`\0\x90\xA3PPPP[PPPPPPPPPPPV[a\x1Er\x81a/\xFBV[a\x1E{\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x163\x14a\x1E\xDEW`@Q\x7F\xA1\xF2\x08 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x81\x90\x03a\x1F>W`@Q\x7F\t\xF9Y \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1FG\x82a*\xC0V[B\x10\x15a\x1F\x80W`@Q\x7F'\x9E=$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra V\x91\x90\x81\x01\x90aD\x9CV[\x90P[`\x01\x81Q\x11\x15a\"aW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\x87!\xA7\x84`\0`\x01\x85`\0\x81Q\x81\x10a \x97Wa \x97aENV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`$\x82\x01R\x91\x16`D\x82\x01R`\x01`d\x82\x01R`\x84\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xF8\xDC]\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Ra!\x85\x93\x92\x91\x90`\0\x90`\x04\x01aE}V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xC8\x91\x90aDhV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\"Z\x91\x90\x81\x01\x90aD\x9CV[\x90Pa YV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\x87!\xA7\x84`\0`\x01\x85`\0\x81Q\x81\x10a\"\x95Wa\"\x95aENV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16`\0\x90\x81R\x92\x83\x90R`@\x92\x83\x90 `\x01\x01T\x92Q\x93\x81\x16`$\x85\x01R\x90\x81\x16`D\x84\x01R\x16`d\x82\x01R`\x84\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE3\x18\xB5+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Ra#\x99\x93\x92\x91\x90`\0\x90`\x04\x01aE}V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a#\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xDC\x91\x90aDhV[P`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra$p\x91\x90\x81\x01\x90aD\x9CV[\x90P\x80Q`\x01\x14\x15\x80a$\xDDWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x81 `\x01\x01T\x83Q\x92\x16\x91\x83\x91\x90a$\xBCWa$\xBCaENV[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a%\x14W`@Q\x7F\xE30\xAC\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\0`$\x82\x01\x81\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91cF\x87!\xA7\x91\x87\x91`D\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE1\x9A\x9D\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Ra%\xF5\x93\x92\x91\x90`\0\x90`\x04\x01aE}V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&8\x91\x90aDhV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 `\x01\x01T\x91Q\x91\x90\x93\x16\x81R\x90\x91\x7F\xDF\xDE\xCBq\xEB\x05\x80\xC9&<\x86{\xC6\xDE\x9D\xD6\xF8Y\xCCjn\xE3=G\xE5\x05\x90O\x1DV\x01\xC9\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[```\0a&\xE1\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[\x90P`\0a&\xF1\x82`\x03\x01a1\xA1V[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x0FWa'\x0Fa9ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'HW\x81` \x01[a'5a8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a'-W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a)\xA3W\x83`\x02\x01`\0\x84\x83\x81Q\x81\x10a'oWa'oaENV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\xA0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a'\xC8Wa'\xC8a<\xEDV[`\x03\x81\x11\x15a'\xD9Wa'\xD9a<\xEDV[\x81R` \x01`\x03\x82\x01`@Q\x80a\x01 \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta(^\x90aB{V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x8A\x90aB{V[\x80\x15a(\xD7W\x80`\x1F\x10a(\xACWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xD7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xBAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T` \x90\x91\x01\x90`\xFF\x16`\x01\x81\x11\x15a(\xFEWa(\xFEa<\xEDV[`\x01\x81\x11\x15a)\x0FWa)\x0Fa<\xEDV[\x81R`\x04\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x05\x83\x01T`@\x83\x01R`\x06\x83\x01T``\x83\x01R`\x07\x83\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x08\x90\x93\x01T\x90\x92\x16`\xA0\x90\x91\x01R\x90\x82R`\x0C\x92\x90\x92\x01T\x91\x01R\x82Q\x83\x90\x83\x90\x81\x10a)\x85Wa)\x85aENV[` \x02` \x01\x01\x81\x90RP\x80\x80a)\x9B\x90aD0V[\x91PPa'NV[P\x94\x93PPPPV[`\0\x80a)\xB8\x83a1\xACV[\x90P`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*+\x91\x90aA\xA9V[\x90P\x80\x82\x10a*:W\x80a*<V[\x81[\x94\x93PPPPV[3a*N\x81a/\xFBV[a*W\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x81\x90\x03a*\xB7W`@Q\x7F\t\xF9Y \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA5\x82a,>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x80\x82\x03a*\xF6WP`\0\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80Ta*<\x90\x83aBcV[`\0\x80a+3\x84a2\xC4V[\x90P`\0a+@\x84a2\xC4V[\x80Q\x83Q\x91\x92P\x14\x80\x15a+[WP\x80` \x01Q\x82` \x01Q\x14[\x80\x15a+nWP\x80`@\x01Q\x82`@\x01Q\x14[\x95\x94PPPPPV[`@Q\x7F-\x9A\xD5=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c-\x9A\xD5=\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x05\x91\x90aDhV[a,;W`@Q\x7F\x9F\xDA\xDA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x03a,mWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x01` R`@\x80\x82 \x82\x90UQ\x7F\xAD\xFAe\x11\xC5\xA9\xB5\xAC\x1A\xDC^N\x0C\xA67\xC4e\xF2J\xB5l\x1B\xD2U\xA0\x83\xE0\x02>;|B\x91\x90\xA2PV[a,\xC7\x81a.\xE1V[\x80\x15a-\\WP`@Q\x7F-\x9A\xD5=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c-\x9A\xD5=\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\\\x91\x90aDhV[a-cWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x82R\x80\x83 T\x93\x83R\x90\x82\x90R\x81 T\x90\x82\x90\x03a-\xB2WPPPV[\x80`\0\x03a-\xBFWPPPV[a-\xCA\x82`\x02aE\xC1V[\x81\x10\x15a.\x03W`@Q\x7FG\xDC\xA1\xCE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0a.\x14\x83\x83a4\x90V[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 a.[\x82a)\xACV[\x81`\x01\x01T\x10\x15a\x03\xA5W`\x01\x81\x01\x80T\x90\x81\x90`\0a.z\x83aD0V[\x91\x90PUP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FN\xDA\x17\x97`\xB4\xE6\x86P\x05\x83v\xD4\xAC\xEFx\xE9S\"\x13\x17E\r)\xB4\x92\x0Em(6\x94L\x82\x84`\x01\x01T`@Qa.\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[`@Q\x7FV$\xB2[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8`\x04\x82\x01R`\x01`$\x82\x01R`\0\x90\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90cV$\xB2[\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra/\xBD\x91\x90\x81\x01\x90a@\xE0V[\x80` \x01\x90Q\x81\x01\x90a/\xD0\x91\x90aE\xFEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14\x93\x92PPPV[`\0a.\x14\x83\x83a5\x8AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x90\x91\x16a\x03\xA5W`@Q\x7F\xA0\xFE\x93\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F-\x9A\xD5=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c-\x9A\xD5=\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xED\x91\x90aDhV[\x15a,;W`@Q\x7Fp\xA3\x80\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x82R\x91\x82\x90 `\x01\x80\x82\x01\x80T\x90\x82\x90U\x84Q\x81\x81R\x93\x84\x01\x91\x90\x91R\x90\x93\x90\x92\x90\x91\x7FN\xDA\x17\x97`\xB4\xE6\x86P\x05\x83v\xD4\xAC\xEFx\xE9S\"\x13\x17E\r)\xB4\x92\x0Em(6\x94L\x91\x01a.\xD4V[``a\x05\x08\x82a5\xD9V[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x1D\x91\x90aA\xA9V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra2\xAE\x91\x90\x81\x01\x90aD\x9CV[Qa2\xB9\x91\x90aD\x85V[a\x05\x08\x90`\x01aBcV[a2\xE8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a3)\x83`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa65V[\x90P`\x03\x81Q\x10\x15a3gW`@Q\x7F\x9E\xDA\x85\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a3\xC2\x82`\x02\x81Q\x81\x10a3\x7FWa3\x7FaENV[` \x02` \x01\x01Q`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa65V[\x90P`\0a4\x1F\x82`\0\x81Q\x81\x10a3\xDCWa3\xDCaENV[` \x02` \x01\x01Q`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa65V[\x90P`@Q\x80``\x01`@R\x80a4O\x85`\0\x81Q\x81\x10a4BWa4BaENV[` \x02` \x01\x01Qa6\xDFV[\x81R` \x01a4j\x85`\x01\x81Q\x81\x10a4BWa4BaENV[\x81R` \x01a4\x85\x83`\0\x81Q\x81\x10a4BWa4BaENV[\x90R\x95\x94PPPPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a5yW`\0a4\xB4`\x01\x83aD\x85V[\x85T\x90\x91P`\0\x90a4\xC8\x90`\x01\x90aD\x85V[\x90P\x81\x81\x14a5-W`\0\x86`\0\x01\x82\x81T\x81\x10a4\xE8Wa4\xE8aENV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a5\x0BWa5\x0BaENV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a5>Wa5>aF\x1BV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x05\x08V[`\0\x91PPa\x05\x08V[P\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta5\xD1WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05\x08V[P`\0a\x05\x08V[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a6)W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a6\x15W[PPPPP\x90P\x91\x90PV[```\0a6C\x84\x84a7WV[\x90P`\x1F\x19` \x82\x01`\x01\x83Q\x01`\x05\x1B\x81\x01\x86Q\x83\x82\x01R`\x01\x84Q\x01\x84R`\0[\x82Q``\x84R\x81\x81\x14a6\xABW`@Q\x82\x82\x03\x80\x82R\x86`\x1F\x82\x01\x16[\x8B\x85\x01\x81\x01Q\x83\x82\x01R\x87\x01\x80a6\x83WP`\0\x82\x82\x01` \x01R`?\x01\x86\x16\x81\x01`@R\x84R[\x87Q` \x94\x90\x94\x01\x93\x01\x90P\x81\x83\x10a6fWPPPP\x80\x91P\x82Qa5\x83W` \x81\x01\x91P`\x02\x81Q\x03\x82RP\x92\x91PPV[\x80Q`\0\x90\x7F\x19\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x82[`\x01\x81\x01\x90P`0`\xFF\x82\x87\x01Q\x16\x03\x82\x85\x11\x85`\n\x02\x82\x81\x01\x96P`\t\x83\x11\x81\x88\x10\x83\x17\x17\x15\x86\x02\x95PPPP\x82\x81\x10a7\x07WPP\x80a7QWc\x10\x18'\x96`\0R`\x04`\x1C\xFD[P\x91\x90PV[``\x82Q\x82Q\x81\x81\x11a8<W` \x85\x01\x94P` \x84\x01\x93P` `@Q\x01\x92P\x84`\x01\x82\x84\x88\x01\x03\x01`\0` \x84\x10a7\x90WP\x82\x86 [`\x1F\x84\x16` \x03`\x03\x1B\x87Q[\x89Q\x81\x81\x18\x83\x1Ca7\xF2W\x83\x15a7\xD0W\x83\x87\x8C \x14a7\xD0W`\x01\x8B\x01\x9AP\x84\x8B\x10a7\xCAWPa8\x01V[Pa7\x9DV[\x85\x8B\x03\x89R\x99\x86\x01\x99` \x90\x98\x01\x97\x86\x15a7\xF2W\x84\x8B\x10a7\xCAWPa8\x01V[P`\x01\x8A\x01\x99P\x83\x8A\x10a7\x9DW[PP`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x89\x03\x01`\x05\x1C\x81R` \x90\x97\x01\x90RPPP[PP\x92\x91PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x90\x91\x82\x01\x90\x81R` \x01a8la8yV[\x81R` \x01`\0\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0`\x01\x81\x11\x15a8\xC3Wa8\xC3a<\xEDV[\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0` \x82\x84\x03\x12\x15a9*W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a.\x14W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a9\xADWa9\xADa9ZV[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a9\xFAWa9\xFAa9ZV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,;W`\0\x80\xFD[\x805a:/\x81a:\x02V[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a:FW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a:iWa:ia9ZV[`@R\x825\x81R` \x83\x015a:~\x81a:\x02V[` \x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a:\x9CW`\0\x80\xFD[P5\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a:\xBDWa:\xBDa9ZV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a:\xFAW`\0\x80\xFD[\x815a;\ra;\x08\x82a:\xA3V[a9\xB3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a;\"W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a;UW`\0\x80\xFD[\x845a;`\x81a:\x02V[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x8AW`\0\x80\xFD[a;\x96\x87\x82\x88\x01a:\xE9V[\x91PP\x92\x95\x91\x94P\x92PV[\x805`\x02\x81\x10a:/W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a;\xC7W`\0\x80\xFD[\x845a;\xD2\x81a:\x02V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a;\xF6W`\0\x80\xFD[\x90\x86\x01\x90a\x01 \x82\x89\x03\x12\x15a<\x0BW`\0\x80\xFD[a<\x13a9\x89V[a<\x1C\x83a:$V[\x81R` \x83\x015` \x82\x01R`@\x83\x015\x82\x81\x11\x15a<:W`\0\x80\xFD[a<F\x8A\x82\x86\x01a:\xE9V[`@\x83\x01RPa<X``\x84\x01a;\xA2V[``\x82\x01R`\x80\x83\x015`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01Ra<\x87`\xE0\x84\x01a:$V[`\xE0\x82\x01Ra\x01\0a<\x9A\x81\x85\x01a:$V[\x90\x82\x01R\x93P``\x87\x015\x91P\x80\x82\x11\x15a<\xB4W`\0\x80\xFD[Pa;\x96\x87\x82\x88\x01a:\xE9V[`\0\x80`@\x83\x85\x03\x12\x15a<\xD4W`\0\x80\xFD[\x825a<\xDF\x81a:\x02V[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a=7W\x81\x81\x01Q\x83\x82\x01R` \x01a=\x1FV[\x83\x81\x11\x15a=FW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra=d\x81` \x86\x01` \x86\x01a=\x1CV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x02\x81\x10a=\xA6Wa=\xA6a<\xEDV[\x90RV[\x80Q\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x04\x81\x10a=\xCFWa=\xCFa<\xEDV[\x80`@\x85\x01RP``\x82\x01Q`\xA0``\x85\x01Ra>\x05`\xA0\x85\x01\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[` \x81\x01Q`\xC0\x85\x01R`@\x81\x01Qa\x01 \x80`\xE0\x87\x01Ra>+a\x01\xC0\x87\x01\x83a=LV[\x91P``\x83\x01Qa\x01\0a>A\x81\x89\x01\x83a=\x96V[`\x80\x85\x01Q\x83\x89\x01R`\xA0\x85\x01Qa\x01@\x89\x01R`\xC0\x85\x01Qa\x01`\x89\x01R`\xE0\x85\x01Q\x92Pa>\x8Aa\x01\x80\x89\x01\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x93\x90\x93\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x01\xA0\x88\x01R\x92Pa>\xB5\x90PV[`\x80\x84\x01Q`\x80\x86\x01R\x80\x92PPP\x92\x91PPV[` \x81R`\0a.\x14` \x83\x01\x84a=\xAAV[`\0` \x82\x84\x03\x12\x15a>\xEFW`\0\x80\xFD[\x815a.\x14\x81a:\x02V[` \x81R`\0a.\x14` \x83\x01\x84a=LV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a?/W`\0\x80\xFD[a?8\x8Ca:$V[\x9AP` \x8C\x015\x99Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@\x8E\x015\x11\x15a?[W`\0\x80\xFD[a?k\x8E`@\x8F\x015\x8F\x01a:\xE9V[\x99Pa?y``\x8E\x01a;\xA2V[\x98P`\x80\x8D\x015\x97P`\xA0\x8D\x015\x96P`\xC0\x8D\x015\x95Pa?\x9C`\xE0\x8E\x01a:$V[\x94Pa?\xABa\x01\0\x8E\x01a:$V[\x93P\x80a\x01 \x8E\x015\x11\x15a?\xBFW`\0\x80\xFD[Pa?\xD1\x8Da\x01 \x8E\x015\x8E\x01a:\xE9V[\x91Pa?\xE0a\x01@\x8D\x01a:$V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[\x80\x15\x15\x81\x14a,;W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a@\x13W`\0\x80\xFD[\x825\x91P` \x83\x015a@%\x81a?\xF2V[\x80\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a@\xA3W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra@\x91\x85\x83Qa=\xAAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a@WV[P\x92\x97\x96PPPPPPPV[`\0a@\xBEa;\x08\x84a:\xA3V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a@\xD2W`\0\x80\xFD[a.\x14\x83` \x83\x01\x84a=\x1CV[`\0` \x82\x84\x03\x12\x15a@\xF2W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\tW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aA\x1AW`\0\x80\xFD[a*<\x84\x82Q` \x84\x01a@\xB0V[`\0a\x01@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8E\x16\x84R\x8C` \x85\x01R\x81`@\x85\x01RaA`\x82\x85\x01\x8Da=LV[\x92PaAo``\x85\x01\x8Ca=\x96V[`\x80\x84\x01\x99\x90\x99RP`\xA0\x82\x01\x96\x90\x96R`\xC0\x81\x01\x94\x90\x94R\x91\x85\x16`\xE0\x84\x01R\x90\x93\x16a\x01\0\x82\x01Ra\x01 \x01\x91\x90\x91R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aA\xBBW`\0\x80\xFD[PQ\x91\x90PV[\x84\x81R`\x80` \x82\x01R`\0aA\xDB`\x80\x83\x01\x86a=LV[\x82\x81\x03`@\x84\x01RaA\xED\x81\x86a=LV[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[\x83\x81R``` \x82\x01R`\0aB\x18``\x83\x01\x85a=LV[\x82\x81\x03`@\x84\x01RaB*\x81\x85a=LV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aBvWaBvaB4V[P\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aB\x8FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a7QW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a.\x03W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aB\xEFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aC\x0EW\x82\x81U`\x01\x01aB\xFBV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC0WaC0a9ZV[aCD\x81aC>\x84TaB{V[\x84aB\xC8V[` \x80`\x1F\x83\x11`\x01\x81\x14aC\x97W`\0\x84\x15aCaWP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaC\x0EV[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15aC\xE4W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aC\xC5V[P\x85\x82\x10\x15aD W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aDaWaDaaB4V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aDzW`\0\x80\xFD[\x81Qa.\x14\x81a?\xF2V[`\0\x82\x82\x10\x15aD\x97WaD\x97aB4V[P\x03\x90V[`\0` \x80\x83\x85\x03\x12\x15aD\xAFW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aD\xC7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aD\xDBW`\0\x80\xFD[\x81Q\x81\x81\x11\x15aD\xEDWaD\xEDa9ZV[\x80`\x05\x1B\x91PaD\xFE\x84\x83\x01a9\xB3V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15aE\x18W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aEBW\x84Q\x92PaE2\x83a:\x02V[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90aE\x1DV[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aE\xB2`\x80\x83\x01\x85a=LV[\x90Pa+n``\x83\x01\x84a=\x96V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aE\xF9WaE\xF9aB4V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aF\x10W`\0\x80\xFD[\x81Qa.\x14\x81a:\x02V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101825760003560e01c806365cf50ec116100d8578063bceb8eda1161008c578063db16aab911610066578063db16aab91461040f578063e10ffc9214610417578063e647dee11461042a57600080fd5b8063bceb8eda146103c9578063c127fd39146103dc578063c9713656146103fc57600080fd5b806375f0bb52116100bd57806375f0bb5214610384578063932713681461039757806396de45a4146103a957600080fd5b806365cf50ec1461035e57806372fb97031461037157600080fd5b806312492d971161013a5780635698b16a116101145780635698b16a146102f85780635c3b45101461034e5780636092b3181461035657600080fd5b806312492d97146101fd57806347c032231461021d57806354fd4d50146102af57600080fd5b806306d8b7e21161016b57806306d8b7e2146101c4578063096e01f7146101d75780630e40beec146101ea57600080fd5b806301ffc9a71461018757806305ccf606146101af575b600080fd5b61019a610195366004613918565b610475565b60405190151581526020015b60405180910390f35b6101c26101bd366004613a34565b61050e565b005b6101c26101d2366004613a8a565b6107b5565b6101c26101e5366004613b3f565b61086b565b6101c26101f8366004613bb1565b610dc3565b61021061020b366004613cc1565b6113e3565b6040516101a69190613eca565b61027e61022b366004613edd565b6040805180820182526000808252602091820181905273ffffffffffffffffffffffffffffffffffffffff9384168152808252829020825180840190935280548352600101549092169181019190915290565b604080518251815260209283015173ffffffffffffffffffffffffffffffffffffffff1692810192909252016101a6565b6102eb6040518060400160405280600681526020017f312e31302e31000000000000000000000000000000000000000000000000000081525081565b6040516101a69190613efa565b610340610306366004613edd565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602090815260408083206003835281842054845290915290205490565b6040519081526020016101a6565b6101c2611614565b6101c261168e565b6101c261036c366004613a8a565b61173f565b6101c261037f366004613edd565b6118de565b6101c2610392366004613f0d565b611a11565b6101c26103a5366004614000565b5050565b6103406103b7366004613edd565b60016020526000908152604090205481565b6101c26103d7366004613edd565b611e69565b6103ef6103ea366004613edd565b6126a0565b6040516101a69190614030565b61034061040a366004613edd565b6129ac565b6101c2612a44565b610340610425366004613edd565b612ac0565b610340610438366004613edd565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602090815260408083206003835281842054845290915290206001015490565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167fe6d7a83a00000000000000000000000000000000000000000000000000000000148061050857507fffffffff0000000000000000000000000000000000000000000000000000000082167f01ffc9a700000000000000000000000000000000000000000000000000000000145b92915050565b8051339060000361054b576040517f74a7e96e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b602082015173ffffffffffffffffffffffffffffffffffffffff1615806105a157508073ffffffffffffffffffffffffffffffffffffffff16826020015173ffffffffffffffffffffffffffffffffffffffff16145b156105d8576040517f552dd1b700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6106a78173ffffffffffffffffffffffffffffffffffffffff1663ffa1ad746040518163ffffffff1660e01b8152600401600060405180830381865afa158015610626573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261066c91908101906140e0565b6040518060400160405280600581526020017f312e342e31000000000000000000000000000000000000000000000000000000815250612b27565b6106dd576040517fda21aed900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6106e681612b77565b73ffffffffffffffffffffffffffffffffffffffff8181166000908152602081815260409091208451815590840151600190910180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169190921617905561074e81612c3e565b8151602083015160405133927f47e2ddae3ece2c77f60d8e9e5a89a50f1d1374a87fb11956d27a6b4986bba17e926107a49291825273ffffffffffffffffffffffffffffffffffffffff16602082015260400190565b60405180910390a26103a581612cbe565b7f51a7f65c6325882f237d4aeb43228179cfad48b868511d508e24b4437a8191376040516108609060208082526050908201527f546869732066756e6374696f6e206973206e6f74206d65616e7420746f20626560408201527f2063616c6c65642c2064696420796f75206d65616e20746f2063616c6c20636160608201527f6e63656c5472616e73616374696f6e3f00000000000000000000000000000000608082015260a00190565b60405180910390a150565b600273ffffffffffffffffffffffffffffffffffffffff851660009081526002602090815260408083206003835281842054845290915290206000858152600291820160205260409020015460ff1660038111156108cb576108cb613ced565b03610902576040517f3e8b838900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8416600090815260026020818152604080842060038084528286205486529083528185208886528401909252909220015460ff168181111561095b5761095b613ced565b03610992576040517f183ca43100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff841660009081526002602090815260408083206003835281842054845290915281206000858152600291820160205260409020015460ff1660038111156109f0576109f0613ced565b03610a27576040517f03c8597300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60003073ffffffffffffffffffffffffffffffffffffffff166306d8b7e285604051602401610a5891815260200190565b604051602081830303815290604052915060e01b6020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008573ffffffffffffffffffffffffffffffffffffffff1663e86637db306000856000806000806000808e6040518b63ffffffff1660e01b8152600401610aed9a99989796959493929190614129565b600060405180830381865afa158015610b0a573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610b5091908101906140e0565b905060008673ffffffffffffffffffffffffffffffffffffffff1663d8d11f78306000866000806000806000808f6040518b63ffffffff1660e01b8152600401610ba39a99989796959493929190614129565b602060405180830381865afa158015610bc0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610be491906141a9565b90508673ffffffffffffffffffffffffffffffffffffffff166312fb68e0828487610c428c73ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b600101546040518563ffffffff1660e01b8152600401610c6594939291906141c2565b60006040518083038186803b158015610c7d57600080fd5b505afa158015610c91573d6000803e3d6000fd5b505050506002610cd48873ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b60008881526002918201602052604090200180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001836003811115610d1e57610d1e613ced565b0217905550610d6d86610d648973ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b60030190612e08565b50610d7787612e1b565b604051869073ffffffffffffffffffffffffffffffffffffffff8916907fa42fd857b47d3d04f5b29f35cb05343f66b317633d2dc2910726bd4bca1a162590600090a350505050505050565b610dcc84612ee1565b610e02576040517f3f4b296600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8416600090815260026020908152604080832060038352818420548452909152902054600003610e73576040517f0832dd6900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008473ffffffffffffffffffffffffffffffffffffffff1663e86637db846000015185602001518660400151876060015188608001518960a001518a60c001518b60e001518c61010001518e6040518b63ffffffff1660e01b8152600401610ee59a99989796959493929190614129565b600060405180830381865afa158015610f02573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610f4891908101906140e0565b905060008573ffffffffffffffffffffffffffffffffffffffff1663d8d11f78856000015186602001518760400151886060015189608001518a60a001518b60c001518c60e001518d61010001518f6040518b63ffffffff1660e01b8152600401610fbc9a99989796959493929190614129565b602060405180830381865afa158015610fd9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ffd91906141a9565b905061103c8673ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b6000828152600291909101602052604090206001015415611089576040517f80394de600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f934f3a1100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff87169063934f3a11906110df908490869088906004016141ff565b60006040518083038186803b1580156110f757600080fd5b505afa15801561110b573d6000803e3d6000fd5b50505050600061114e8773ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b546111599042614263565b6040805160a08101825284815260208082018481526001838501818152606085018c9052608085018d905273ffffffffffffffffffffffffffffffffffffffff8e166000908152600280865287822060038088528984205484529087528883208c84528201909652969020855181559251838301555194820180549697509395919493927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0090921691849081111561121357611213613ced565b0217905550606082015180516003830180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff90921691909117815560208201516004840155604082015160058401906112839082614316565b5060608201516003820180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600183818111156112c4576112c4613ced565b0217905550608082810151600483015560a0830151600583015560c0830151600683015560e08301516007830180547fffffffffffffffffffffffff000000000000000000000000000000000000000090811673ffffffffffffffffffffffffffffffffffffffff93841617909155610100909401516008909301805490941692811692909217909255920151600c909101558716600090815260026020908152604080832060038352818420548452909152902061138890839060030190612fef565b50818773ffffffffffffffffffffffffffffffffffffffff167f653f8f6fce2a503b2dfca34e95b3e4902254a11765d2658c0e5af1d64ab276cf836040516113d291815260200190565b60405180910390a350505050505050565b6113eb613844565b73ffffffffffffffffffffffffffffffffffffffff8316600090815260026020818152604080842060038084528286205486529083528185208786528401835293819020815160a081018352815481526001820154938101939093529283015491939084019160ff169081111561146457611464613ced565b600381111561147557611475613ced565b815260200160038201604051806101200160405290816000820160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015481526020016002820180546114fa9061427b565b80601f01602080910402602001604051908101604052809291908181526020018280546115269061427b565b80156115735780601f1061154857610100808354040283529160200191611573565b820191906000526020600020905b81548152906001019060200180831161155657829003601f168201915b5050509183525050600382015460209091019060ff16600181111561159a5761159a613ced565b60018111156115ab576115ab613ced565b815260048201546020808301919091526005830154604083015260068301546060830152600783015473ffffffffffffffffffffffffffffffffffffffff908116608084015260089093015490921660a090910152908252600c92909201549101529392505050565b3361161e81612ee1565b15611655576040517fa4d234cb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600090815260036020526040812080549161168683614430565b919050555050565b3361169881612ffb565b6116a18161305f565b73ffffffffffffffffffffffffffffffffffffffff8116600090815260208190526040812090815560010180547fffffffffffffffffffffffff00000000000000000000000000000000000000001690556116fb81612c3e565b60405173ffffffffffffffffffffffffffffffffffffffff8216907f9c6ff02e684f8a81389ed942fea1147c16b8cc5fe79f2bfbc520d44e214aed4b90600090a250565b60003390506117928173ffffffffffffffffffffffffffffffffffffffff1663ffa1ad746040518163ffffffff1660e01b8152600401600060405180830381865afa158015610626573d6000803e3d6000fd5b6117c8576040517f9e2f7c4b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117d181612ee1565b611807576040517f3f4b296600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81158061181757506301e1338082115b1561184e576040517fa0ce228b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8161188c8273ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b5561189681613124565b8073ffffffffffffffffffffffffffffffffffffffff167fa48d13ee8fad9974fa901cfb88a02d39c5361efbab13bb9b3aa7caa3f6d6b786836040516107a491815260200190565b6118e781612ffb565b6118f081612b77565b73ffffffffffffffffffffffffffffffffffffffff818116600090815260208190526040902060010154163314611953576040517fa1f2082000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600090815260016020526040902054156119b0576040517f9cd9090400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020908152604091829020429081905591519182527f3eb9241ca06793ab672590e858d0977206f824e7367806f94a90af391d275d33910160405180910390a250565b33600081815260026020908152604080832060038352818420548452909152902054600003611a405750611e5c565b6040517f2f54bf6e00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8381166004830152821690632f54bf6e90602401602060405180830381865afa158015611aac573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ad09190614468565b611b06576040517f5874e94f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600060018273ffffffffffffffffffffffffffffffffffffffff1663affed0e06040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b55573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b7991906141a9565b611b839190614485565b905060008273ffffffffffffffffffffffffffffffffffffffff1663d8d11f788f8f8f8f8f8f8f8f8f8c6040518b63ffffffff1660e01b8152600401611bd29a99989796959493929190614129565b602060405180830381865afa158015611bef573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c1391906141a9565b90506000611c548473ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b60008381526002918201602052604090209150600282015460ff166003811115611c8057611c80613ced565b03611cb7576040517f3e8b838900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6003600282015460ff166003811115611cd257611cd2613ced565b03611d09576040517f183ca43100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000600282015460ff166003811115611d2457611d24613ced565b03611d5b576040517f03c8597300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b4281600101541115611d99576040517f503c42c400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611da284613124565b600281810180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600390811790915573ffffffffffffffffffffffffffffffffffffffff8616600090815260209283526040808220928452808220548252919092529020611e13908390610d64565b50604051829073ffffffffffffffffffffffffffffffffffffffff8616907fdd4b9b318b98162cb1e7b52752a3fd110d5b7966f3b50884c1cd3bd04058e5c790600090a3505050505b5050505050505050505050565b611e7281612ffb565b611e7b81612b77565b73ffffffffffffffffffffffffffffffffffffffff818116600090815260208190526040902060010154163314611ede576040517fa1f2082000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660009081526001602052604081205490819003611f3e576040517f09f9592000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611f4782612ac0565b421015611f80576040517f279e3d2400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600160008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000206000905560008273ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612010573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052612056919081019061449c565b90505b600181511115612261578273ffffffffffffffffffffffffffffffffffffffff1663468721a78460006001856000815181106120975761209761454e565b602090810291909101015160405173ffffffffffffffffffffffffffffffffffffffff92831660248201529116604482015260016064820152608401604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167ff8dc5dd900000000000000000000000000000000000000000000000000000000179052517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b1681526121859392919060009060040161457d565b6020604051808303816000875af11580156121a4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121c89190614468565b508273ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015612214573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261225a919081019061449c565b9050612059565b8273ffffffffffffffffffffffffffffffffffffffff1663468721a78460006001856000815181106122955761229561454e565b60209081029190910181015173ffffffffffffffffffffffffffffffffffffffff8a811660009081529283905260409283902060010154925193811660248501529081166044840152166064820152608401604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe318b52b00000000000000000000000000000000000000000000000000000000179052517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b1681526123999392919060009060040161457d565b6020604051808303816000875af11580156123b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123dc9190614468565b5060008373ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa15801561242a573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052612470919081019061449c565b9050805160011415806124dd575073ffffffffffffffffffffffffffffffffffffffff80851660009081526020819052604081206001015483519216918391906124bc576124bc61454e565b602002602001015173ffffffffffffffffffffffffffffffffffffffff1614155b15612514576040517fe330ac0500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405160006024820181905273ffffffffffffffffffffffffffffffffffffffff86169163468721a7918791604401604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe19a9dd900000000000000000000000000000000000000000000000000000000179052517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b1681526125f59392919060009060040161457d565b6020604051808303816000875af1158015612614573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126389190614468565b5073ffffffffffffffffffffffffffffffffffffffff8481166000818152602081815260409182902060010154915191909316815290917fdfdecb71eb0580c9263c867bc6de9dd6f859cc6a6ee33d47e505904f1d5601c9910160405180910390a250505050565b606060006126e18373ffffffffffffffffffffffffffffffffffffffff16600090815260026020908152604080832060038352818420548452909152902090565b905060006126f1826003016131a1565b90506000815167ffffffffffffffff81111561270f5761270f61395a565b60405190808252806020026020018201604052801561274857816020015b612735613844565b81526020019060019003908161272d5790505b50905060005b82518110156129a35783600201600084838151811061276f5761276f61454e565b602002602001015181526020019081526020016000206040518060a001604052908160008201548152602001600182015481526020016002820160009054906101000a900460ff1660038111156127c8576127c8613ced565b60038111156127d9576127d9613ced565b815260200160038201604051806101200160405290816000820160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016001820154815260200160028201805461285e9061427b565b80601f016020809104026020016040519081016040528092919081815260200182805461288a9061427b565b80156128d75780601f106128ac576101008083540402835291602001916128d7565b820191906000526020600020905b8154815290600101906020018083116128ba57829003601f168201915b5050509183525050600382015460209091019060ff1660018111156128fe576128fe613ced565b600181111561290f5761290f613ced565b815260048201546020808301919091526005830154604083015260068301546060830152600783015473ffffffffffffffffffffffffffffffffffffffff908116608084015260089093015490921660a090910152908252600c929092015491015282518390839081106129855761298561454e565b6020026020010181905250808061299b90614430565b91505061274e565b50949350505050565b6000806129b8836131ac565b905060008373ffffffffffffffffffffffffffffffffffffffff1663e75235b86040518163ffffffff1660e01b8152600401602060405180830381865afa158015612a07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a2b91906141a9565b9050808210612a3a5780612a3c565b815b949350505050565b33612a4e81612ffb565b612a5781612b77565b73ffffffffffffffffffffffffffffffffffffffff811660009081526001602052604081205490819003612ab7576040517f09f9592000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6103a582612c3e565b73ffffffffffffffffffffffffffffffffffffffff8116600090815260016020526040812054808203612af65750600092915050565b73ffffffffffffffffffffffffffffffffffffffff831660009081526020819052604090208054612a3c9083614263565b600080612b33846132c4565b90506000612b40846132c4565b80518351919250148015612b5b575080602001518260200151145b8015612b6e575080604001518260400151145b95945050505050565b6040517f2d9ad53d00000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff821690632d9ad53d90602401602060405180830381865afa158015612be1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c059190614468565b612c3b576040517f9fdada3100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b73ffffffffffffffffffffffffffffffffffffffff81166000908152600160205260408120549003612c6d5750565b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020526040808220829055517fadfa6511c5a9b5ac1adc5e4e0ca637c465f24ab56c1bd255a083e0023e3b7c429190a250565b612cc781612ee1565b8015612d5c57506040517f2d9ad53d00000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff821690632d9ad53d90602401602060405180830381865afa158015612d38573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d5c9190614468565b612d635750565b73ffffffffffffffffffffffffffffffffffffffff81166000818152600260209081526040808320600383528184205484528252808320549383529082905281205490829003612db257505050565b80600003612dbf57505050565b612dca8260026145c1565b811015612e03576040517f47dca1ce00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050565b6000612e148383613490565b9392505050565b73ffffffffffffffffffffffffffffffffffffffff81166000908152600260209081526040808320600383528184205484529091529020612e5b826129ac565b816001015410156103a5576001810180549081906000612e7a83614430565b91905055508273ffffffffffffffffffffffffffffffffffffffff167f4eda179760b4e68650058376d4acef78e953221317450d29b4920e6d2836944c828460010154604051612ed4929190918252602082015260400190565b60405180910390a2505050565b6040517f5624b25b0000000000000000000000000000000000000000000000000000000081527f4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c8600482015260016024820152600090819073ffffffffffffffffffffffffffffffffffffffff841690635624b25b90604401600060405180830381865afa158015612f77573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052612fbd91908101906140e0565b806020019051810190612fd091906145fe565b73ffffffffffffffffffffffffffffffffffffffff1630149392505050565b6000612e14838361358a565b73ffffffffffffffffffffffffffffffffffffffff808216600090815260208190526040902060018101549091166103a5576040517fa0fe939b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f2d9ad53d00000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff821690632d9ad53d90602401602060405180830381865afa1580156130c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906130ed9190614468565b15612c3b576040517f70a3809400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660008181526002602090815260408083206003835281842054845282529182902060018082018054908290558451818152938401919091529093909290917f4eda179760b4e68650058376d4acef78e953221317450d29b4920e6d2836944c9101612ed4565b6060610508826135d9565b60008173ffffffffffffffffffffffffffffffffffffffff1663e75235b86040518163ffffffff1660e01b8152600401602060405180830381865afa1580156131f9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061321d91906141a9565b8273ffffffffffffffffffffffffffffffffffffffff1663a0e67e2b6040518163ffffffff1660e01b8152600401600060405180830381865afa158015613268573d6000803e3d6000fd5b505050506040513d6000823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526132ae919081019061449c565b516132b99190614485565b610508906001614263565b6132e860405180606001604052806000815260200160008152602001600081525090565b6000613329836040518060400160405280600181526020017f2e00000000000000000000000000000000000000000000000000000000000000815250613635565b9050600381511015613367576040517f9eda858c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006133c28260028151811061337f5761337f61454e565b60200260200101516040518060400160405280600181526020017f2d00000000000000000000000000000000000000000000000000000000000000815250613635565b9050600061341f826000815181106133dc576133dc61454e565b60200260200101516040518060400160405280600181526020017f2b00000000000000000000000000000000000000000000000000000000000000815250613635565b9050604051806060016040528061344f856000815181106134425761344261454e565b60200260200101516136df565b815260200161346a856001815181106134425761344261454e565b8152602001613485836000815181106134425761344261454e565b905295945050505050565b600081815260018301602052604081205480156135795760006134b4600183614485565b85549091506000906134c890600190614485565b905081811461352d5760008660000182815481106134e8576134e861454e565b906000526020600020015490508087600001848154811061350b5761350b61454e565b6000918252602080832090910192909255918252600188019052604090208390555b855486908061353e5761353e61461b565b600190038181906000526020600020016000905590558560010160008681526020019081526020016000206000905560019350505050610508565b6000915050610508565b5092915050565b60008181526001830160205260408120546135d157508154600181810184556000848152602080822090930184905584548482528286019093526040902091909155610508565b506000610508565b60608160000180548060200260200160405190810160405280929190818152602001828054801561362957602002820191906000526020600020905b815481526020019060010190808311613615575b50505050509050919050565b606060006136438484613757565b9050601f1960208201600183510160051b81018651838201526001845101845260005b8251606084528181146136ab5760405182820380825286601f8201165b8b8501810151838201528701806136835750600082820160200152603f018616810160405284525b8751602094909401930190508183106136665750505050809150825161358357602081019150600281510382525092915050565b80516000907f1999999999999999999999999999999999999999999999999999999999999999825b600181019050603060ff82870151160382851185600a028281019650600983118188108317171586029550505050828110613707575050806137515763101827966000526004601cfd5b50919050565b60608251825181811161383c57602085019450602084019350602060405101925084600182848801030160006020841061379057508286205b601f841660200360031b87515b8951818118831c6137f25783156137d05783878c20146137d05760018b019a50848b106137ca5750613801565b5061379d565b858b0389529986019960209098019786156137f257848b106137ca5750613801565b5060018a019950838a1061379d575b5050604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08189030160051c8152602090970190525050505b505092915050565b6040805160a0810182526000808252602082018190529091820190815260200161386c613879565b8152602001600081525090565b604051806101200160405280600073ffffffffffffffffffffffffffffffffffffffff1681526020016000815260200160608152602001600060018111156138c3576138c3613ced565b8152602001600081526020016000815260200160008152602001600073ffffffffffffffffffffffffffffffffffffffff168152602001600073ffffffffffffffffffffffffffffffffffffffff1681525090565b60006020828403121561392a57600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114612e1457600080fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051610120810167ffffffffffffffff811182821017156139ad576139ad61395a565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156139fa576139fa61395a565b604052919050565b73ffffffffffffffffffffffffffffffffffffffff81168114612c3b57600080fd5b8035613a2f81613a02565b919050565b600060408284031215613a4657600080fd5b6040516040810181811067ffffffffffffffff82111715613a6957613a6961395a565b604052823581526020830135613a7e81613a02565b60208201529392505050565b600060208284031215613a9c57600080fd5b5035919050565b600067ffffffffffffffff821115613abd57613abd61395a565b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b600082601f830112613afa57600080fd5b8135613b0d613b0882613aa3565b6139b3565b818152846020838601011115613b2257600080fd5b816020850160208301376000918101602001919091529392505050565b60008060008060808587031215613b5557600080fd5b8435613b6081613a02565b93506020850135925060408501359150606085013567ffffffffffffffff811115613b8a57600080fd5b613b9687828801613ae9565b91505092959194509250565b803560028110613a2f57600080fd5b60008060008060808587031215613bc757600080fd5b8435613bd281613a02565b935060208501359250604085013567ffffffffffffffff80821115613bf657600080fd5b908601906101208289031215613c0b57600080fd5b613c13613989565b613c1c83613a24565b815260208301356020820152604083013582811115613c3a57600080fd5b613c468a828601613ae9565b604083015250613c5860608401613ba2565b60608201526080830135608082015260a083013560a082015260c083013560c0820152613c8760e08401613a24565b60e0820152610100613c9a818501613a24565b9082015293506060870135915080821115613cb457600080fd5b50613b9687828801613ae9565b60008060408385031215613cd457600080fd5b8235613cdf81613a02565b946020939093013593505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b60005b83811015613d37578181015183820152602001613d1f565b83811115613d46576000848401525b50505050565b60008151808452613d64816020860160208601613d1c565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b60028110613da657613da6613ced565b9052565b80518252602081015160208301526000604082015160048110613dcf57613dcf613ced565b80604085015250606082015160a06060850152613e0560a08501825173ffffffffffffffffffffffffffffffffffffffff169052565b602081015160c085015260408101516101208060e0870152613e2b6101c0870183613d4c565b91506060830151610100613e4181890183613d96565b60808501518389015260a085015161014089015260c085015161016089015260e08501519250613e8a61018089018473ffffffffffffffffffffffffffffffffffffffff169052565b939093015173ffffffffffffffffffffffffffffffffffffffff81166101a08801529250613eb59050565b60808401516080860152809250505092915050565b602081526000612e146020830184613daa565b600060208284031215613eef57600080fd5b8135612e1481613a02565b602081526000612e146020830184613d4c565b60008060008060008060008060008060006101608c8e031215613f2f57600080fd5b613f388c613a24565b9a5060208c0135995067ffffffffffffffff8060408e01351115613f5b57600080fd5b613f6b8e60408f01358f01613ae9565b9950613f7960608e01613ba2565b985060808d0135975060a08d0135965060c08d01359550613f9c60e08e01613a24565b9450613fab6101008e01613a24565b9350806101208e01351115613fbf57600080fd5b50613fd18d6101208e01358e01613ae9565b9150613fe06101408d01613a24565b90509295989b509295989b9093969950565b8015158114612c3b57600080fd5b6000806040838503121561401357600080fd5b82359150602083013561402581613ff2565b809150509250929050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b828110156140a3577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0888603018452614091858351613daa565b94509285019290850190600101614057565b5092979650505050505050565b60006140be613b0884613aa3565b90508281528383830111156140d257600080fd5b612e14836020830184613d1c565b6000602082840312156140f257600080fd5b815167ffffffffffffffff81111561410957600080fd5b8201601f8101841361411a57600080fd5b612a3c848251602084016140b0565b600061014073ffffffffffffffffffffffffffffffffffffffff808e1684528c60208501528160408501526141608285018d613d4c565b925061416f606085018c613d96565b60808401999099525060a082019690965260c081019490945291851660e08401529093166101008201526101200191909152949350505050565b6000602082840312156141bb57600080fd5b5051919050565b8481526080602082015260006141db6080830186613d4c565b82810360408401526141ed8186613d4c565b91505082606083015295945050505050565b8381526060602082015260006142186060830185613d4c565b828103604084015261422a8185613d4c565b9695505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6000821982111561427657614276614234565b500190565b600181811c9082168061428f57607f821691505b602082108103613751577f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b601f821115612e0357600081815260208120601f850160051c810160208610156142ef5750805b601f850160051c820191505b8181101561430e578281556001016142fb565b505050505050565b815167ffffffffffffffff8111156143305761433061395a565b6143448161433e845461427b565b846142c8565b602080601f83116001811461439757600084156143615750858301515b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600386901b1c1916600185901b17855561430e565b6000858152602081207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08616915b828110156143e4578886015182559484019460019091019084016143c5565b508582101561442057878501517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600388901b60f8161c191681555b5050505050600190811b01905550565b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361446157614461614234565b5060010190565b60006020828403121561447a57600080fd5b8151612e1481613ff2565b60008282101561449757614497614234565b500390565b600060208083850312156144af57600080fd5b825167ffffffffffffffff808211156144c757600080fd5b818501915085601f8301126144db57600080fd5b8151818111156144ed576144ed61395a565b8060051b91506144fe8483016139b3565b818152918301840191848101908884111561451857600080fd5b938501935b83851015614542578451925061453283613a02565b828252938501939085019061451d565b98975050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b73ffffffffffffffffffffffffffffffffffffffff851681528360208201526080604082015260006145b26080830185613d4c565b9050612b6e6060830184613d96565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156145f9576145f9614234565b500290565b60006020828403121561461057600080fd5b8151612e1481613a02565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603160045260246000fdfea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x82W`\x005`\xE0\x1C\x80ce\xCFP\xEC\x11a\0\xD8W\x80c\xBC\xEB\x8E\xDA\x11a\0\x8CW\x80c\xDB\x16\xAA\xB9\x11a\0fW\x80c\xDB\x16\xAA\xB9\x14a\x04\x0FW\x80c\xE1\x0F\xFC\x92\x14a\x04\x17W\x80c\xE6G\xDE\xE1\x14a\x04*W`\0\x80\xFD[\x80c\xBC\xEB\x8E\xDA\x14a\x03\xC9W\x80c\xC1'\xFD9\x14a\x03\xDCW\x80c\xC9q6V\x14a\x03\xFCW`\0\x80\xFD[\x80cu\xF0\xBBR\x11a\0\xBDW\x80cu\xF0\xBBR\x14a\x03\x84W\x80c\x93'\x13h\x14a\x03\x97W\x80c\x96\xDEE\xA4\x14a\x03\xA9W`\0\x80\xFD[\x80ce\xCFP\xEC\x14a\x03^W\x80cr\xFB\x97\x03\x14a\x03qW`\0\x80\xFD[\x80c\x12I-\x97\x11a\x01:W\x80cV\x98\xB1j\x11a\x01\x14W\x80cV\x98\xB1j\x14a\x02\xF8W\x80c\\;E\x10\x14a\x03NW\x80c`\x92\xB3\x18\x14a\x03VW`\0\x80\xFD[\x80c\x12I-\x97\x14a\x01\xFDW\x80cG\xC02#\x14a\x02\x1DW\x80cT\xFDMP\x14a\x02\xAFW`\0\x80\xFD[\x80c\x06\xD8\xB7\xE2\x11a\x01kW\x80c\x06\xD8\xB7\xE2\x14a\x01\xC4W\x80c\tn\x01\xF7\x14a\x01\xD7W\x80c\x0E@\xBE\xEC\x14a\x01\xEAW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x87W\x80c\x05\xCC\xF6\x06\x14a\x01\xAFW[`\0\x80\xFD[a\x01\x9Aa\x01\x956`\x04a9\x18V[a\x04uV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xC2a\x01\xBD6`\x04a:4V[a\x05\x0EV[\0[a\x01\xC2a\x01\xD26`\x04a:\x8AV[a\x07\xB5V[a\x01\xC2a\x01\xE56`\x04a;?V[a\x08kV[a\x01\xC2a\x01\xF86`\x04a;\xB1V[a\r\xC3V[a\x02\x10a\x02\x0B6`\x04a<\xC1V[a\x13\xE3V[`@Qa\x01\xA6\x91\x90a>\xCAV[a\x02~a\x02+6`\x04a>\xDDV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01\x81\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x80\x82R\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01a\x01\xA6V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7F1.10.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xA6\x91\x90a>\xFAV[a\x03@a\x03\x066`\x04a>\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 T\x90V[`@Q\x90\x81R` \x01a\x01\xA6V[a\x01\xC2a\x16\x14V[a\x01\xC2a\x16\x8EV[a\x01\xC2a\x03l6`\x04a:\x8AV[a\x17?V[a\x01\xC2a\x03\x7F6`\x04a>\xDDV[a\x18\xDEV[a\x01\xC2a\x03\x926`\x04a?\rV[a\x1A\x11V[a\x01\xC2a\x03\xA56`\x04a@\0V[PPV[a\x03@a\x03\xB76`\x04a>\xDDV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xC2a\x03\xD76`\x04a>\xDDV[a\x1EiV[a\x03\xEFa\x03\xEA6`\x04a>\xDDV[a&\xA0V[`@Qa\x01\xA6\x91\x90a@0V[a\x03@a\x04\n6`\x04a>\xDDV[a)\xACV[a\x01\xC2a*DV[a\x03@a\x04%6`\x04a>\xDDV[a*\xC0V[a\x03@a\x0486`\x04a>\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 `\x01\x01T\x90V[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE6\xD7\xA8:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x05\x08WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x92\x91PPV[\x80Q3\x90`\0\x03a\x05KW`@Q\x7Ft\xA7\xE9n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80a\x05\xA1WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x15a\x05\xD8W`@Q\x7FU-\xD1\xB7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\xA7\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xA1\xADt`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x06l\x91\x90\x81\x01\x90a@\xE0V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.4.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa+'V[a\x06\xDDW`@Q\x7F\xDA!\xAE\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\xE6\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16`\0\x90\x81R` \x81\x81R`@\x90\x91 \x84Q\x81U\x90\x84\x01Q`\x01\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x92\x16\x17\x90Ua\x07N\x81a,>V[\x81Q` \x83\x01Q`@Q3\x92\x7FG\xE2\xDD\xAE>\xCE,w\xF6\r\x8E\x9EZ\x89\xA5\x0F\x1D\x13t\xA8\x7F\xB1\x19V\xD2zkI\x86\xBB\xA1~\x92a\x07\xA4\x92\x91\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x03\xA5\x81a,\xBEV[\x7FQ\xA7\xF6\\c%\x88/#}J\xEBC\"\x81y\xCF\xADH\xB8hQ\x1DP\x8E$\xB4Cz\x81\x917`@Qa\x08`\x90` \x80\x82R`P\x90\x82\x01R\x7FThis function is not meant to be`@\x82\x01R\x7F called, did you mean to call ca``\x82\x01R\x7FncelTransaction?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1PV[`\x02s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 `\0\x85\x81R`\x02\x91\x82\x01` R`@\x90 \x01T`\xFF\x16`\x03\x81\x11\x15a\x08\xCBWa\x08\xCBa<\xEDV[\x03a\t\x02W`@Q\x7F>\x8B\x83\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 `\x03\x80\x84R\x82\x86 T\x86R\x90\x83R\x81\x85 \x88\x86R\x84\x01\x90\x92R\x90\x92 \x01T`\xFF\x16\x81\x81\x11\x15a\t[Wa\t[a<\xEDV[\x03a\t\x92W`@Q\x7F\x18<\xA41\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x81 `\0\x85\x81R`\x02\x91\x82\x01` R`@\x90 \x01T`\xFF\x16`\x03\x81\x11\x15a\t\xF0Wa\t\xF0a<\xEDV[\x03a\n'W`@Q\x7F\x03\xC8Ys\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x06\xD8\xB7\xE2\x85`@Q`$\x01a\nX\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP\x90P`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE8f7\xDB0`\0\x85`\0\x80`\0\x80`\0\x80\x8E`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xED\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0BP\x91\x90\x81\x01\x90a@\xE0V[\x90P`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx0`\0\x86`\0\x80`\0\x80`\0\x80\x8F`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xA3\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE4\x91\x90aA\xA9V[\x90P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x12\xFBh\xE0\x82\x84\x87a\x0CB\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\x01\x01T`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ce\x94\x93\x92\x91\x90aA\xC2V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C}W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0C\x91W=`\0\x80>=`\0\xFD[PPPP`\x02a\x0C\xD4\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\0\x88\x81R`\x02\x91\x82\x01` R`@\x90 \x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x03\x81\x11\x15a\r\x1EWa\r\x1Ea<\xEDV[\x02\x17\x90UPa\rm\x86a\rd\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\x03\x01\x90a.\x08V[Pa\rw\x87a.\x1BV[`@Q\x86\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90\x7F\xA4/\xD8W\xB4}=\x04\xF5\xB2\x9F5\xCB\x054?f\xB3\x17c=-\xC2\x91\x07&\xBDK\xCA\x1A\x16%\x90`\0\x90\xA3PPPPPPPV[a\r\xCC\x84a.\xE1V[a\x0E\x02W`@Q\x7F?K)f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 T`\0\x03a\x0EsW`@Q\x7F\x082\xDDi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE8f7\xDB\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q\x8A`\xC0\x01Q\x8B`\xE0\x01Q\x8Ca\x01\0\x01Q\x8E`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xE5\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0FH\x91\x90\x81\x01\x90a@\xE0V[\x90P`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x88``\x01Q\x89`\x80\x01Q\x8A`\xA0\x01Q\x8B`\xC0\x01Q\x8C`\xE0\x01Q\x8Da\x01\0\x01Q\x8F`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xBC\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFD\x91\x90aA\xA9V[\x90Pa\x10<\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\0\x82\x81R`\x02\x91\x90\x91\x01` R`@\x90 `\x01\x01T\x15a\x10\x89W`@Q\x7F\x809M\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x93O:\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90c\x93O:\x11\x90a\x10\xDF\x90\x84\x90\x86\x90\x88\x90`\x04\x01aA\xFFV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xF7W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x11\x0BW=`\0\x80>=`\0\xFD[PPPP`\0a\x11N\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[Ta\x11Y\x90BaBcV[`@\x80Q`\xA0\x81\x01\x82R\x84\x81R` \x80\x82\x01\x84\x81R`\x01\x83\x85\x01\x81\x81R``\x85\x01\x8C\x90R`\x80\x85\x01\x8D\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8E\x16`\0\x90\x81R`\x02\x80\x86R\x87\x82 `\x03\x80\x88R\x89\x84 T\x84R\x90\x87R\x88\x83 \x8C\x84R\x82\x01\x90\x96R\x96\x90 \x85Q\x81U\x92Q\x83\x83\x01UQ\x94\x82\x01\x80T\x96\x97P\x93\x95\x91\x94\x93\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x92\x16\x91\x84\x90\x81\x11\x15a\x12\x13Wa\x12\x13a<\xEDV[\x02\x17\x90UP``\x82\x01Q\x80Q`\x03\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x81U` \x82\x01Q`\x04\x84\x01U`@\x82\x01Q`\x05\x84\x01\x90a\x12\x83\x90\x82aC\x16V[P``\x82\x01Q`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83\x81\x81\x11\x15a\x12\xC4Wa\x12\xC4a<\xEDV[\x02\x17\x90UP`\x80\x82\x81\x01Q`\x04\x83\x01U`\xA0\x83\x01Q`\x05\x83\x01U`\xC0\x83\x01Q`\x06\x83\x01U`\xE0\x83\x01Q`\x07\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x17\x90\x91Ua\x01\0\x90\x94\x01Q`\x08\x90\x93\x01\x80T\x90\x94\x16\x92\x81\x16\x92\x90\x92\x17\x90\x92U\x92\x01Q`\x0C\x90\x91\x01U\x87\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 a\x13\x88\x90\x83\x90`\x03\x01\x90a/\xEFV[P\x81\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fe?\x8Fo\xCE*P;-\xFC\xA3N\x95\xB3\xE4\x90\"T\xA1\x17e\xD2e\x8C\x0EZ\xF1\xD6J\xB2v\xCF\x83`@Qa\x13\xD2\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[a\x13\xEBa8DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 `\x03\x80\x84R\x82\x86 T\x86R\x90\x83R\x81\x85 \x87\x86R\x84\x01\x83R\x93\x81\x90 \x81Q`\xA0\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R\x92\x83\x01T\x91\x93\x90\x84\x01\x91`\xFF\x16\x90\x81\x11\x15a\x14dWa\x14da<\xEDV[`\x03\x81\x11\x15a\x14uWa\x14ua<\xEDV[\x81R` \x01`\x03\x82\x01`@Q\x80a\x01 \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x14\xFA\x90aB{V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15&\x90aB{V[\x80\x15a\x15sW\x80`\x1F\x10a\x15HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15sV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15VW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T` \x90\x91\x01\x90`\xFF\x16`\x01\x81\x11\x15a\x15\x9AWa\x15\x9Aa<\xEDV[`\x01\x81\x11\x15a\x15\xABWa\x15\xABa<\xEDV[\x81R`\x04\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x05\x83\x01T`@\x83\x01R`\x06\x83\x01T``\x83\x01R`\x07\x83\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x08\x90\x93\x01T\x90\x92\x16`\xA0\x90\x91\x01R\x90\x82R`\x0C\x92\x90\x92\x01T\x91\x01R\x93\x92PPPV[3a\x16\x1E\x81a.\xE1V[\x15a\x16UW`@Q\x7F\xA4\xD24\xCB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x91a\x16\x86\x83aD0V[\x91\x90PUPPV[3a\x16\x98\x81a/\xFBV[a\x16\xA1\x81a0_V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 \x90\x81U`\x01\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90Ua\x16\xFB\x81a,>V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\x9Co\xF0.hO\x8A\x818\x9E\xD9B\xFE\xA1\x14|\x16\xB8\xCC_\xE7\x9F+\xFB\xC5 \xD4N!J\xEDK\x90`\0\x90\xA2PV[`\x003\x90Pa\x17\x92\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xA1\xADt`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06&W=`\0\x80>=`\0\xFD[a\x17\xC8W`@Q\x7F\x9E/|K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xD1\x81a.\xE1V[a\x18\x07W`@Q\x7F?K)f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15\x80a\x18\x17WPc\x01\xE13\x80\x82\x11[\x15a\x18NW`@Q\x7F\xA0\xCE\"\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x18\x8C\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[Ua\x18\x96\x81a1$V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xA4\x8D\x13\xEE\x8F\xAD\x99t\xFA\x90\x1C\xFB\x88\xA0-9\xC56\x1E\xFB\xAB\x13\xBB\x9B:\xA7\xCA\xA3\xF6\xD6\xB7\x86\x83`@Qa\x07\xA4\x91\x81R` \x01\x90V[a\x18\xE7\x81a/\xFBV[a\x18\xF0\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x163\x14a\x19SW`@Q\x7F\xA1\xF2\x08 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x19\xB0W`@Q\x7F\x9C\xD9\t\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 B\x90\x81\x90U\x91Q\x91\x82R\x7F>\xB9$\x1C\xA0g\x93\xABg%\x90\xE8X\xD0\x97r\x06\xF8$\xE76x\x06\xF9J\x90\xAF9\x1D']3\x91\x01`@Q\x80\x91\x03\x90\xA2PV[3`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 T`\0\x03a\x1A@WPa\x1E\\V[`@Q\x7F/T\xBFn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x82\x16\x90c/T\xBFn\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xD0\x91\x90aDhV[a\x1B\x06W`@Q\x7FXt\xE9O\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAF\xFE\xD0\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1By\x91\x90aA\xA9V[a\x1B\x83\x91\x90aD\x85V[\x90P`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD8\xD1\x1Fx\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8C`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xD2\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA)V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x13\x91\x90aA\xA9V[\x90P`\0a\x1CT\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[`\0\x83\x81R`\x02\x91\x82\x01` R`@\x90 \x91P`\x02\x82\x01T`\xFF\x16`\x03\x81\x11\x15a\x1C\x80Wa\x1C\x80a<\xEDV[\x03a\x1C\xB7W`@Q\x7F>\x8B\x83\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\x02\x82\x01T`\xFF\x16`\x03\x81\x11\x15a\x1C\xD2Wa\x1C\xD2a<\xEDV[\x03a\x1D\tW`@Q\x7F\x18<\xA41\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x82\x01T`\xFF\x16`\x03\x81\x11\x15a\x1D$Wa\x1D$a<\xEDV[\x03a\x1D[W`@Q\x7F\x03\xC8Ys\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`\x01\x01T\x11\x15a\x1D\x99W`@Q\x7FP<B\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xA2\x84a1$V[`\x02\x81\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x03\x90\x81\x17\x90\x91Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R` \x92\x83R`@\x80\x82 \x92\x84R\x80\x82 T\x82R\x91\x90\x92R\x90 a\x1E\x13\x90\x83\x90a\rdV[P`@Q\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x7F\xDDK\x9B1\x8B\x98\x16,\xB1\xE7\xB5'R\xA3\xFD\x11\r[yf\xF3\xB5\x08\x84\xC1\xCD;\xD0@X\xE5\xC7\x90`\0\x90\xA3PPPP[PPPPPPPPPPPV[a\x1Er\x81a/\xFBV[a\x1E{\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x163\x14a\x1E\xDEW`@Q\x7F\xA1\xF2\x08 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x81\x90\x03a\x1F>W`@Q\x7F\t\xF9Y \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1FG\x82a*\xC0V[B\x10\x15a\x1F\x80W`@Q\x7F'\x9E=$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra V\x91\x90\x81\x01\x90aD\x9CV[\x90P[`\x01\x81Q\x11\x15a\"aW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\x87!\xA7\x84`\0`\x01\x85`\0\x81Q\x81\x10a \x97Wa \x97aENV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`$\x82\x01R\x91\x16`D\x82\x01R`\x01`d\x82\x01R`\x84\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xF8\xDC]\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Ra!\x85\x93\x92\x91\x90`\0\x90`\x04\x01aE}V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xC8\x91\x90aDhV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\"Z\x91\x90\x81\x01\x90aD\x9CV[\x90Pa YV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\x87!\xA7\x84`\0`\x01\x85`\0\x81Q\x81\x10a\"\x95Wa\"\x95aENV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16`\0\x90\x81R\x92\x83\x90R`@\x92\x83\x90 `\x01\x01T\x92Q\x93\x81\x16`$\x85\x01R\x90\x81\x16`D\x84\x01R\x16`d\x82\x01R`\x84\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE3\x18\xB5+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Ra#\x99\x93\x92\x91\x90`\0\x90`\x04\x01aE}V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a#\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xDC\x91\x90aDhV[P`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra$p\x91\x90\x81\x01\x90aD\x9CV[\x90P\x80Q`\x01\x14\x15\x80a$\xDDWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x81 `\x01\x01T\x83Q\x92\x16\x91\x83\x91\x90a$\xBCWa$\xBCaENV[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a%\x14W`@Q\x7F\xE30\xAC\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\0`$\x82\x01\x81\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91cF\x87!\xA7\x91\x87\x91`D\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE1\x9A\x9D\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Ra%\xF5\x93\x92\x91\x90`\0\x90`\x04\x01aE}V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&8\x91\x90aDhV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 `\x01\x01T\x91Q\x91\x90\x93\x16\x81R\x90\x91\x7F\xDF\xDE\xCBq\xEB\x05\x80\xC9&<\x86{\xC6\xDE\x9D\xD6\xF8Y\xCCjn\xE3=G\xE5\x05\x90O\x1DV\x01\xC9\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[```\0a&\xE1\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 \x90V[\x90P`\0a&\xF1\x82`\x03\x01a1\xA1V[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x0FWa'\x0Fa9ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'HW\x81` \x01[a'5a8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a'-W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a)\xA3W\x83`\x02\x01`\0\x84\x83\x81Q\x81\x10a'oWa'oaENV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\xA0\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a'\xC8Wa'\xC8a<\xEDV[`\x03\x81\x11\x15a'\xD9Wa'\xD9a<\xEDV[\x81R` \x01`\x03\x82\x01`@Q\x80a\x01 \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta(^\x90aB{V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x8A\x90aB{V[\x80\x15a(\xD7W\x80`\x1F\x10a(\xACWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xD7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xBAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T` \x90\x91\x01\x90`\xFF\x16`\x01\x81\x11\x15a(\xFEWa(\xFEa<\xEDV[`\x01\x81\x11\x15a)\x0FWa)\x0Fa<\xEDV[\x81R`\x04\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x05\x83\x01T`@\x83\x01R`\x06\x83\x01T``\x83\x01R`\x07\x83\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x80\x84\x01R`\x08\x90\x93\x01T\x90\x92\x16`\xA0\x90\x91\x01R\x90\x82R`\x0C\x92\x90\x92\x01T\x91\x01R\x82Q\x83\x90\x83\x90\x81\x10a)\x85Wa)\x85aENV[` \x02` \x01\x01\x81\x90RP\x80\x80a)\x9B\x90aD0V[\x91PPa'NV[P\x94\x93PPPPV[`\0\x80a)\xB8\x83a1\xACV[\x90P`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*+\x91\x90aA\xA9V[\x90P\x80\x82\x10a*:W\x80a*<V[\x81[\x94\x93PPPPV[3a*N\x81a/\xFBV[a*W\x81a+wV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x81\x90\x03a*\xB7W`@Q\x7F\t\xF9Y \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA5\x82a,>V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x80\x82\x03a*\xF6WP`\0\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80Ta*<\x90\x83aBcV[`\0\x80a+3\x84a2\xC4V[\x90P`\0a+@\x84a2\xC4V[\x80Q\x83Q\x91\x92P\x14\x80\x15a+[WP\x80` \x01Q\x82` \x01Q\x14[\x80\x15a+nWP\x80`@\x01Q\x82`@\x01Q\x14[\x95\x94PPPPPV[`@Q\x7F-\x9A\xD5=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c-\x9A\xD5=\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x05\x91\x90aDhV[a,;W`@Q\x7F\x9F\xDA\xDA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90\x03a,mWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x01` R`@\x80\x82 \x82\x90UQ\x7F\xAD\xFAe\x11\xC5\xA9\xB5\xAC\x1A\xDC^N\x0C\xA67\xC4e\xF2J\xB5l\x1B\xD2U\xA0\x83\xE0\x02>;|B\x91\x90\xA2PV[a,\xC7\x81a.\xE1V[\x80\x15a-\\WP`@Q\x7F-\x9A\xD5=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c-\x9A\xD5=\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\\\x91\x90aDhV[a-cWPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x82R\x80\x83 T\x93\x83R\x90\x82\x90R\x81 T\x90\x82\x90\x03a-\xB2WPPPV[\x80`\0\x03a-\xBFWPPPV[a-\xCA\x82`\x02aE\xC1V[\x81\x10\x15a.\x03W`@Q\x7FG\xDC\xA1\xCE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0a.\x14\x83\x83a4\x90V[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x90\x91R\x90 a.[\x82a)\xACV[\x81`\x01\x01T\x10\x15a\x03\xA5W`\x01\x81\x01\x80T\x90\x81\x90`\0a.z\x83aD0V[\x91\x90PUP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FN\xDA\x17\x97`\xB4\xE6\x86P\x05\x83v\xD4\xAC\xEFx\xE9S\"\x13\x17E\r)\xB4\x92\x0Em(6\x94L\x82\x84`\x01\x01T`@Qa.\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[`@Q\x7FV$\xB2[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8`\x04\x82\x01R`\x01`$\x82\x01R`\0\x90\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90cV$\xB2[\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra/\xBD\x91\x90\x81\x01\x90a@\xE0V[\x80` \x01\x90Q\x81\x01\x90a/\xD0\x91\x90aE\xFEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14\x93\x92PPPV[`\0a.\x14\x83\x83a5\x8AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x90\x91\x16a\x03\xA5W`@Q\x7F\xA0\xFE\x93\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F-\x9A\xD5=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90c-\x9A\xD5=\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xED\x91\x90aDhV[\x15a,;W`@Q\x7Fp\xA3\x80\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 `\x03\x83R\x81\x84 T\x84R\x82R\x91\x82\x90 `\x01\x80\x82\x01\x80T\x90\x82\x90U\x84Q\x81\x81R\x93\x84\x01\x91\x90\x91R\x90\x93\x90\x92\x90\x91\x7FN\xDA\x17\x97`\xB4\xE6\x86P\x05\x83v\xD4\xAC\xEFx\xE9S\"\x13\x17E\r)\xB4\x92\x0Em(6\x94L\x91\x01a.\xD4V[``a\x05\x08\x82a5\xD9V[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7R5\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x1D\x91\x90aA\xA9V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0\xE6~+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra2\xAE\x91\x90\x81\x01\x90aD\x9CV[Qa2\xB9\x91\x90aD\x85V[a\x05\x08\x90`\x01aBcV[a2\xE8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a3)\x83`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa65V[\x90P`\x03\x81Q\x10\x15a3gW`@Q\x7F\x9E\xDA\x85\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a3\xC2\x82`\x02\x81Q\x81\x10a3\x7FWa3\x7FaENV[` \x02` \x01\x01Q`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa65V[\x90P`\0a4\x1F\x82`\0\x81Q\x81\x10a3\xDCWa3\xDCaENV[` \x02` \x01\x01Q`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa65V[\x90P`@Q\x80``\x01`@R\x80a4O\x85`\0\x81Q\x81\x10a4BWa4BaENV[` \x02` \x01\x01Qa6\xDFV[\x81R` \x01a4j\x85`\x01\x81Q\x81\x10a4BWa4BaENV[\x81R` \x01a4\x85\x83`\0\x81Q\x81\x10a4BWa4BaENV[\x90R\x95\x94PPPPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a5yW`\0a4\xB4`\x01\x83aD\x85V[\x85T\x90\x91P`\0\x90a4\xC8\x90`\x01\x90aD\x85V[\x90P\x81\x81\x14a5-W`\0\x86`\0\x01\x82\x81T\x81\x10a4\xE8Wa4\xE8aENV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a5\x0BWa5\x0BaENV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a5>Wa5>aF\x1BV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x05\x08V[`\0\x91PPa\x05\x08V[P\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta5\xD1WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x05\x08V[P`\0a\x05\x08V[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a6)W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a6\x15W[PPPPP\x90P\x91\x90PV[```\0a6C\x84\x84a7WV[\x90P`\x1F\x19` \x82\x01`\x01\x83Q\x01`\x05\x1B\x81\x01\x86Q\x83\x82\x01R`\x01\x84Q\x01\x84R`\0[\x82Q``\x84R\x81\x81\x14a6\xABW`@Q\x82\x82\x03\x80\x82R\x86`\x1F\x82\x01\x16[\x8B\x85\x01\x81\x01Q\x83\x82\x01R\x87\x01\x80a6\x83WP`\0\x82\x82\x01` \x01R`?\x01\x86\x16\x81\x01`@R\x84R[\x87Q` \x94\x90\x94\x01\x93\x01\x90P\x81\x83\x10a6fWPPPP\x80\x91P\x82Qa5\x83W` \x81\x01\x91P`\x02\x81Q\x03\x82RP\x92\x91PPV[\x80Q`\0\x90\x7F\x19\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x99\x82[`\x01\x81\x01\x90P`0`\xFF\x82\x87\x01Q\x16\x03\x82\x85\x11\x85`\n\x02\x82\x81\x01\x96P`\t\x83\x11\x81\x88\x10\x83\x17\x17\x15\x86\x02\x95PPPP\x82\x81\x10a7\x07WPP\x80a7QWc\x10\x18'\x96`\0R`\x04`\x1C\xFD[P\x91\x90PV[``\x82Q\x82Q\x81\x81\x11a8<W` \x85\x01\x94P` \x84\x01\x93P` `@Q\x01\x92P\x84`\x01\x82\x84\x88\x01\x03\x01`\0` \x84\x10a7\x90WP\x82\x86 [`\x1F\x84\x16` \x03`\x03\x1B\x87Q[\x89Q\x81\x81\x18\x83\x1Ca7\xF2W\x83\x15a7\xD0W\x83\x87\x8C \x14a7\xD0W`\x01\x8B\x01\x9AP\x84\x8B\x10a7\xCAWPa8\x01V[Pa7\x9DV[\x85\x8B\x03\x89R\x99\x86\x01\x99` \x90\x98\x01\x97\x86\x15a7\xF2W\x84\x8B\x10a7\xCAWPa8\x01V[P`\x01\x8A\x01\x99P\x83\x8A\x10a7\x9DW[PP`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x89\x03\x01`\x05\x1C\x81R` \x90\x97\x01\x90RPPP[PP\x92\x91PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x90\x91\x82\x01\x90\x81R` \x01a8la8yV[\x81R` \x01`\0\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0`\x01\x81\x11\x15a8\xC3Wa8\xC3a<\xEDV[\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`\0` \x82\x84\x03\x12\x15a9*W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a.\x14W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a9\xADWa9\xADa9ZV[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a9\xFAWa9\xFAa9ZV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,;W`\0\x80\xFD[\x805a:/\x81a:\x02V[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a:FW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a:iWa:ia9ZV[`@R\x825\x81R` \x83\x015a:~\x81a:\x02V[` \x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a:\x9CW`\0\x80\xFD[P5\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a:\xBDWa:\xBDa9ZV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a:\xFAW`\0\x80\xFD[\x815a;\ra;\x08\x82a:\xA3V[a9\xB3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a;\"W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a;UW`\0\x80\xFD[\x845a;`\x81a:\x02V[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x8AW`\0\x80\xFD[a;\x96\x87\x82\x88\x01a:\xE9V[\x91PP\x92\x95\x91\x94P\x92PV[\x805`\x02\x81\x10a:/W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a;\xC7W`\0\x80\xFD[\x845a;\xD2\x81a:\x02V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a;\xF6W`\0\x80\xFD[\x90\x86\x01\x90a\x01 \x82\x89\x03\x12\x15a<\x0BW`\0\x80\xFD[a<\x13a9\x89V[a<\x1C\x83a:$V[\x81R` \x83\x015` \x82\x01R`@\x83\x015\x82\x81\x11\x15a<:W`\0\x80\xFD[a<F\x8A\x82\x86\x01a:\xE9V[`@\x83\x01RPa<X``\x84\x01a;\xA2V[``\x82\x01R`\x80\x83\x015`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01Ra<\x87`\xE0\x84\x01a:$V[`\xE0\x82\x01Ra\x01\0a<\x9A\x81\x85\x01a:$V[\x90\x82\x01R\x93P``\x87\x015\x91P\x80\x82\x11\x15a<\xB4W`\0\x80\xFD[Pa;\x96\x87\x82\x88\x01a:\xE9V[`\0\x80`@\x83\x85\x03\x12\x15a<\xD4W`\0\x80\xFD[\x825a<\xDF\x81a:\x02V[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a=7W\x81\x81\x01Q\x83\x82\x01R` \x01a=\x1FV[\x83\x81\x11\x15a=FW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra=d\x81` \x86\x01` \x86\x01a=\x1CV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x02\x81\x10a=\xA6Wa=\xA6a<\xEDV[\x90RV[\x80Q\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x04\x81\x10a=\xCFWa=\xCFa<\xEDV[\x80`@\x85\x01RP``\x82\x01Q`\xA0``\x85\x01Ra>\x05`\xA0\x85\x01\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[` \x81\x01Q`\xC0\x85\x01R`@\x81\x01Qa\x01 \x80`\xE0\x87\x01Ra>+a\x01\xC0\x87\x01\x83a=LV[\x91P``\x83\x01Qa\x01\0a>A\x81\x89\x01\x83a=\x96V[`\x80\x85\x01Q\x83\x89\x01R`\xA0\x85\x01Qa\x01@\x89\x01R`\xC0\x85\x01Qa\x01`\x89\x01R`\xE0\x85\x01Q\x92Pa>\x8Aa\x01\x80\x89\x01\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x93\x90\x93\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x01\xA0\x88\x01R\x92Pa>\xB5\x90PV[`\x80\x84\x01Q`\x80\x86\x01R\x80\x92PPP\x92\x91PPV[` \x81R`\0a.\x14` \x83\x01\x84a=\xAAV[`\0` \x82\x84\x03\x12\x15a>\xEFW`\0\x80\xFD[\x815a.\x14\x81a:\x02V[` \x81R`\0a.\x14` \x83\x01\x84a=LV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a?/W`\0\x80\xFD[a?8\x8Ca:$V[\x9AP` \x8C\x015\x99Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@\x8E\x015\x11\x15a?[W`\0\x80\xFD[a?k\x8E`@\x8F\x015\x8F\x01a:\xE9V[\x99Pa?y``\x8E\x01a;\xA2V[\x98P`\x80\x8D\x015\x97P`\xA0\x8D\x015\x96P`\xC0\x8D\x015\x95Pa?\x9C`\xE0\x8E\x01a:$V[\x94Pa?\xABa\x01\0\x8E\x01a:$V[\x93P\x80a\x01 \x8E\x015\x11\x15a?\xBFW`\0\x80\xFD[Pa?\xD1\x8Da\x01 \x8E\x015\x8E\x01a:\xE9V[\x91Pa?\xE0a\x01@\x8D\x01a:$V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[\x80\x15\x15\x81\x14a,;W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a@\x13W`\0\x80\xFD[\x825\x91P` \x83\x015a@%\x81a?\xF2V[\x80\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a@\xA3W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra@\x91\x85\x83Qa=\xAAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a@WV[P\x92\x97\x96PPPPPPPV[`\0a@\xBEa;\x08\x84a:\xA3V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a@\xD2W`\0\x80\xFD[a.\x14\x83` \x83\x01\x84a=\x1CV[`\0` \x82\x84\x03\x12\x15a@\xF2W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\tW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aA\x1AW`\0\x80\xFD[a*<\x84\x82Q` \x84\x01a@\xB0V[`\0a\x01@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8E\x16\x84R\x8C` \x85\x01R\x81`@\x85\x01RaA`\x82\x85\x01\x8Da=LV[\x92PaAo``\x85\x01\x8Ca=\x96V[`\x80\x84\x01\x99\x90\x99RP`\xA0\x82\x01\x96\x90\x96R`\xC0\x81\x01\x94\x90\x94R\x91\x85\x16`\xE0\x84\x01R\x90\x93\x16a\x01\0\x82\x01Ra\x01 \x01\x91\x90\x91R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aA\xBBW`\0\x80\xFD[PQ\x91\x90PV[\x84\x81R`\x80` \x82\x01R`\0aA\xDB`\x80\x83\x01\x86a=LV[\x82\x81\x03`@\x84\x01RaA\xED\x81\x86a=LV[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[\x83\x81R``` \x82\x01R`\0aB\x18``\x83\x01\x85a=LV[\x82\x81\x03`@\x84\x01RaB*\x81\x85a=LV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aBvWaBvaB4V[P\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aB\x8FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a7QW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a.\x03W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aB\xEFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aC\x0EW\x82\x81U`\x01\x01aB\xFBV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC0WaC0a9ZV[aCD\x81aC>\x84TaB{V[\x84aB\xC8V[` \x80`\x1F\x83\x11`\x01\x81\x14aC\x97W`\0\x84\x15aCaWP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaC\x0EV[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15aC\xE4W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aC\xC5V[P\x85\x82\x10\x15aD W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aDaWaDaaB4V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aDzW`\0\x80\xFD[\x81Qa.\x14\x81a?\xF2V[`\0\x82\x82\x10\x15aD\x97WaD\x97aB4V[P\x03\x90V[`\0` \x80\x83\x85\x03\x12\x15aD\xAFW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aD\xC7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aD\xDBW`\0\x80\xFD[\x81Q\x81\x81\x11\x15aD\xEDWaD\xEDa9ZV[\x80`\x05\x1B\x91PaD\xFE\x84\x83\x01a9\xB3V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15aE\x18W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aEBW\x84Q\x92PaE2\x83a:\x02V[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90aE\x1DV[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aE\xB2`\x80\x83\x01\x85a=LV[\x90Pa+n``\x83\x01\x84a=\x96V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aE\xF9WaE\xF9aB4V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aF\x10W`\0\x80\xFD[\x81Qa.\x14\x81a:\x02V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `LivenessModule2_ChallengeAlreadyExists()` and selector `0x9cd90904`.
```solidity
error LivenessModule2_ChallengeAlreadyExists();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_ChallengeAlreadyExists;
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
        impl ::core::convert::From<LivenessModule2_ChallengeAlreadyExists>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_ChallengeAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_ChallengeAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_ChallengeAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_ChallengeAlreadyExists()";
            const SELECTOR: [u8; 4] = [156u8, 217u8, 9u8, 4u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_ChallengeDoesNotExist()` and selector `0x09f95920`.
```solidity
error LivenessModule2_ChallengeDoesNotExist();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_ChallengeDoesNotExist;
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
        impl ::core::convert::From<LivenessModule2_ChallengeDoesNotExist>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_ChallengeDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_ChallengeDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_ChallengeDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_ChallengeDoesNotExist()";
            const SELECTOR: [u8; 4] = [9u8, 249u8, 89u8, 32u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_InvalidFallbackOwner()` and selector `0x552dd1b7`.
```solidity
error LivenessModule2_InvalidFallbackOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_InvalidFallbackOwner;
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
        impl ::core::convert::From<LivenessModule2_InvalidFallbackOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_InvalidFallbackOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_InvalidFallbackOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_InvalidFallbackOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_InvalidFallbackOwner()";
            const SELECTOR: [u8; 4] = [85u8, 45u8, 209u8, 183u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_InvalidResponsePeriod()` and selector `0x74a7e96e`.
```solidity
error LivenessModule2_InvalidResponsePeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_InvalidResponsePeriod;
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
        impl ::core::convert::From<LivenessModule2_InvalidResponsePeriod>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_InvalidResponsePeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_InvalidResponsePeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_InvalidResponsePeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_InvalidResponsePeriod()";
            const SELECTOR: [u8; 4] = [116u8, 167u8, 233u8, 110u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_InvalidVersion()` and selector `0xda21aed9`.
```solidity
error LivenessModule2_InvalidVersion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_InvalidVersion;
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
        impl ::core::convert::From<LivenessModule2_InvalidVersion>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_InvalidVersion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_InvalidVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_InvalidVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_InvalidVersion()";
            const SELECTOR: [u8; 4] = [218u8, 33u8, 174u8, 217u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_ModuleNotConfigured()` and selector `0xa0fe939b`.
```solidity
error LivenessModule2_ModuleNotConfigured();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_ModuleNotConfigured;
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
        impl ::core::convert::From<LivenessModule2_ModuleNotConfigured>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_ModuleNotConfigured) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_ModuleNotConfigured {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_ModuleNotConfigured {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_ModuleNotConfigured()";
            const SELECTOR: [u8; 4] = [160u8, 254u8, 147u8, 155u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_ModuleNotEnabled()` and selector `0x9fdada31`.
```solidity
error LivenessModule2_ModuleNotEnabled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_ModuleNotEnabled;
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
        impl ::core::convert::From<LivenessModule2_ModuleNotEnabled>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_ModuleNotEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_ModuleNotEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_ModuleNotEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_ModuleNotEnabled()";
            const SELECTOR: [u8; 4] = [159u8, 218u8, 218u8, 49u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_ModuleStillEnabled()` and selector `0x70a38094`.
```solidity
error LivenessModule2_ModuleStillEnabled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_ModuleStillEnabled;
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
        impl ::core::convert::From<LivenessModule2_ModuleStillEnabled>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_ModuleStillEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_ModuleStillEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_ModuleStillEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_ModuleStillEnabled()";
            const SELECTOR: [u8; 4] = [112u8, 163u8, 128u8, 148u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_OwnershipTransferFailed()` and selector `0xe330ac05`.
```solidity
error LivenessModule2_OwnershipTransferFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_OwnershipTransferFailed;
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
        impl ::core::convert::From<LivenessModule2_OwnershipTransferFailed>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_OwnershipTransferFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_OwnershipTransferFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_OwnershipTransferFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_OwnershipTransferFailed()";
            const SELECTOR: [u8; 4] = [227u8, 48u8, 172u8, 5u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_ResponsePeriodActive()` and selector `0x279e3d24`.
```solidity
error LivenessModule2_ResponsePeriodActive();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_ResponsePeriodActive;
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
        impl ::core::convert::From<LivenessModule2_ResponsePeriodActive>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_ResponsePeriodActive) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_ResponsePeriodActive {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_ResponsePeriodActive {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_ResponsePeriodActive()";
            const SELECTOR: [u8; 4] = [39u8, 158u8, 61u8, 36u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_ResponsePeriodEnded()` and selector `0xeb911af0`.
```solidity
error LivenessModule2_ResponsePeriodEnded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_ResponsePeriodEnded;
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
        impl ::core::convert::From<LivenessModule2_ResponsePeriodEnded>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_ResponsePeriodEnded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_ResponsePeriodEnded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_ResponsePeriodEnded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_ResponsePeriodEnded()";
            const SELECTOR: [u8; 4] = [235u8, 145u8, 26u8, 240u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `LivenessModule2_UnauthorizedCaller()` and selector `0xa1f20820`.
```solidity
error LivenessModule2_UnauthorizedCaller();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LivenessModule2_UnauthorizedCaller;
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
        impl ::core::convert::From<LivenessModule2_UnauthorizedCaller>
        for UnderlyingRustTuple<'_> {
            fn from(value: LivenessModule2_UnauthorizedCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LivenessModule2_UnauthorizedCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LivenessModule2_UnauthorizedCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LivenessModule2_UnauthorizedCaller()";
            const SELECTOR: [u8; 4] = [161u8, 242u8, 8u8, 32u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `SaferSafes_InsufficientLivenessResponsePeriod()` and selector `0x47dca1ce`.
```solidity
error SaferSafes_InsufficientLivenessResponsePeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SaferSafes_InsufficientLivenessResponsePeriod;
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
        impl ::core::convert::From<SaferSafes_InsufficientLivenessResponsePeriod>
        for UnderlyingRustTuple<'_> {
            fn from(value: SaferSafes_InsufficientLivenessResponsePeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SaferSafes_InsufficientLivenessResponsePeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError
        for SaferSafes_InsufficientLivenessResponsePeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SaferSafes_InsufficientLivenessResponsePeriod()";
            const SELECTOR: [u8; 4] = [71u8, 220u8, 161u8, 206u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `SemverComp_InvalidSemverParts()` and selector `0x9eda858c`.
```solidity
error SemverComp_InvalidSemverParts();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SemverComp_InvalidSemverParts;
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
        impl ::core::convert::From<SemverComp_InvalidSemverParts>
        for UnderlyingRustTuple<'_> {
            fn from(value: SemverComp_InvalidSemverParts) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SemverComp_InvalidSemverParts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SemverComp_InvalidSemverParts {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SemverComp_InvalidSemverParts()";
            const SELECTOR: [u8; 4] = [158u8, 218u8, 133u8, 140u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_GuardNotConfigured()` and selector `0x0832dd69`.
```solidity
error TimelockGuard_GuardNotConfigured();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_GuardNotConfigured;
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
        impl ::core::convert::From<TimelockGuard_GuardNotConfigured>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_GuardNotConfigured) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_GuardNotConfigured {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_GuardNotConfigured {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_GuardNotConfigured()";
            const SELECTOR: [u8; 4] = [8u8, 50u8, 221u8, 105u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_GuardNotEnabled()` and selector `0x3f4b2966`.
```solidity
error TimelockGuard_GuardNotEnabled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_GuardNotEnabled;
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
        impl ::core::convert::From<TimelockGuard_GuardNotEnabled>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_GuardNotEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_GuardNotEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_GuardNotEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_GuardNotEnabled()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 41u8, 102u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_GuardStillEnabled()` and selector `0xa4d234cb`.
```solidity
error TimelockGuard_GuardStillEnabled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_GuardStillEnabled;
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
        impl ::core::convert::From<TimelockGuard_GuardStillEnabled>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_GuardStillEnabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_GuardStillEnabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_GuardStillEnabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_GuardStillEnabled()";
            const SELECTOR: [u8; 4] = [164u8, 210u8, 52u8, 203u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_InvalidTimelockDelay()` and selector `0xa0ce228b`.
```solidity
error TimelockGuard_InvalidTimelockDelay();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_InvalidTimelockDelay;
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
        impl ::core::convert::From<TimelockGuard_InvalidTimelockDelay>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_InvalidTimelockDelay) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_InvalidTimelockDelay {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_InvalidTimelockDelay {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_InvalidTimelockDelay()";
            const SELECTOR: [u8; 4] = [160u8, 206u8, 34u8, 139u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_InvalidVersion()` and selector `0x9e2f7c4b`.
```solidity
error TimelockGuard_InvalidVersion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_InvalidVersion;
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
        impl ::core::convert::From<TimelockGuard_InvalidVersion>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_InvalidVersion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_InvalidVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_InvalidVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_InvalidVersion()";
            const SELECTOR: [u8; 4] = [158u8, 47u8, 124u8, 75u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_NotOwner()` and selector `0x5874e94f`.
```solidity
error TimelockGuard_NotOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_NotOwner;
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
        impl ::core::convert::From<TimelockGuard_NotOwner> for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_NotOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TimelockGuard_NotOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_NotOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_NotOwner()";
            const SELECTOR: [u8; 4] = [88u8, 116u8, 233u8, 79u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_TransactionAlreadyCancelled()` and selector `0x3e8b8389`.
```solidity
error TimelockGuard_TransactionAlreadyCancelled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_TransactionAlreadyCancelled;
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
        impl ::core::convert::From<TimelockGuard_TransactionAlreadyCancelled>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_TransactionAlreadyCancelled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_TransactionAlreadyCancelled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_TransactionAlreadyCancelled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_TransactionAlreadyCancelled()";
            const SELECTOR: [u8; 4] = [62u8, 139u8, 131u8, 137u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_TransactionAlreadyExecuted()` and selector `0x183ca431`.
```solidity
error TimelockGuard_TransactionAlreadyExecuted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_TransactionAlreadyExecuted;
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
        impl ::core::convert::From<TimelockGuard_TransactionAlreadyExecuted>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_TransactionAlreadyExecuted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_TransactionAlreadyExecuted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_TransactionAlreadyExecuted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_TransactionAlreadyExecuted()";
            const SELECTOR: [u8; 4] = [24u8, 60u8, 164u8, 49u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_TransactionAlreadyScheduled()` and selector `0x80394de6`.
```solidity
error TimelockGuard_TransactionAlreadyScheduled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_TransactionAlreadyScheduled;
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
        impl ::core::convert::From<TimelockGuard_TransactionAlreadyScheduled>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_TransactionAlreadyScheduled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_TransactionAlreadyScheduled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_TransactionAlreadyScheduled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_TransactionAlreadyScheduled()";
            const SELECTOR: [u8; 4] = [128u8, 57u8, 77u8, 230u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_TransactionNotReady()` and selector `0x503c42c4`.
```solidity
error TimelockGuard_TransactionNotReady();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_TransactionNotReady;
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
        impl ::core::convert::From<TimelockGuard_TransactionNotReady>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_TransactionNotReady) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_TransactionNotReady {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_TransactionNotReady {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_TransactionNotReady()";
            const SELECTOR: [u8; 4] = [80u8, 60u8, 66u8, 196u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `TimelockGuard_TransactionNotScheduled()` and selector `0x03c85973`.
```solidity
error TimelockGuard_TransactionNotScheduled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockGuard_TransactionNotScheduled;
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
        impl ::core::convert::From<TimelockGuard_TransactionNotScheduled>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockGuard_TransactionNotScheduled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockGuard_TransactionNotScheduled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockGuard_TransactionNotScheduled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockGuard_TransactionNotScheduled()";
            const SELECTOR: [u8; 4] = [3u8, 200u8, 89u8, 115u8];
            #[inline]
            fn new<'a>(
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
    /**Event with signature `CancellationThresholdUpdated(address,uint256,uint256)` and selector `0x4eda179760b4e68650058376d4acef78e953221317450d29b4920e6d2836944c`.
```solidity
event CancellationThresholdUpdated(address indexed safe, uint256 oldThreshold, uint256 newThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CancellationThresholdUpdated {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for CancellationThresholdUpdated {
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
            const SIGNATURE: &'static str = "CancellationThresholdUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                78u8, 218u8, 23u8, 151u8, 96u8, 180u8, 230u8, 134u8, 80u8, 5u8, 131u8,
                118u8, 212u8, 172u8, 239u8, 120u8, 233u8, 83u8, 34u8, 19u8, 23u8, 69u8,
                13u8, 41u8, 180u8, 146u8, 14u8, 109u8, 40u8, 54u8, 148u8, 76u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    oldThreshold: data.0,
                    newThreshold: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.oldThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newThreshold),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.safe.clone())
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
                    &self.safe,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CancellationThresholdUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CancellationThresholdUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CancellationThresholdUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChallengeCancelled(address)` and selector `0xadfa6511c5a9b5ac1adc5e4e0ca637c465f24ab56c1bd255a083e0023e3b7c42`.
```solidity
event ChallengeCancelled(address indexed safe);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChallengeCancelled {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ChallengeCancelled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ChallengeCancelled(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                173u8, 250u8, 101u8, 17u8, 197u8, 169u8, 181u8, 172u8, 26u8, 220u8, 94u8,
                78u8, 12u8, 166u8, 55u8, 196u8, 101u8, 242u8, 74u8, 181u8, 108u8, 27u8,
                210u8, 85u8, 160u8, 131u8, 224u8, 2u8, 62u8, 59u8, 124u8, 66u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { safe: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.safe.clone())
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
                    &self.safe,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChallengeCancelled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChallengeCancelled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChallengeCancelled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChallengeStarted(address,uint256)` and selector `0x3eb9241ca06793ab672590e858d0977206f824e7367806f94a90af391d275d33`.
```solidity
event ChallengeStarted(address indexed safe, uint256 challengeStartTime);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChallengeStarted {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub challengeStartTime: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ChallengeStarted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ChallengeStarted(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                62u8, 185u8, 36u8, 28u8, 160u8, 103u8, 147u8, 171u8, 103u8, 37u8, 144u8,
                232u8, 88u8, 208u8, 151u8, 114u8, 6u8, 248u8, 36u8, 231u8, 54u8, 120u8,
                6u8, 249u8, 74u8, 144u8, 175u8, 57u8, 29u8, 39u8, 93u8, 51u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    challengeStartTime: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.challengeStartTime),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.safe.clone())
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
                    &self.safe,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChallengeStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChallengeStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChallengeStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChallengeSucceeded(address,address)` and selector `0xdfdecb71eb0580c9263c867bc6de9dd6f859cc6a6ee33d47e505904f1d5601c9`.
```solidity
event ChallengeSucceeded(address indexed safe, address fallbackOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChallengeSucceeded {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fallbackOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ChallengeSucceeded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ChallengeSucceeded(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                223u8, 222u8, 203u8, 113u8, 235u8, 5u8, 128u8, 201u8, 38u8, 60u8, 134u8,
                123u8, 198u8, 222u8, 157u8, 214u8, 248u8, 89u8, 204u8, 106u8, 110u8,
                227u8, 61u8, 71u8, 229u8, 5u8, 144u8, 79u8, 29u8, 86u8, 1u8, 201u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    fallbackOwner: data.0,
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
                        &self.fallbackOwner,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.safe.clone())
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
                    &self.safe,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChallengeSucceeded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChallengeSucceeded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChallengeSucceeded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `GuardConfigured(address,uint256)` and selector `0xa48d13ee8fad9974fa901cfb88a02d39c5361efbab13bb9b3aa7caa3f6d6b786`.
```solidity
event GuardConfigured(address indexed safe, uint256 timelockDelay);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GuardConfigured {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timelockDelay: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for GuardConfigured {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "GuardConfigured(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 141u8, 19u8, 238u8, 143u8, 173u8, 153u8, 116u8, 250u8, 144u8,
                28u8, 251u8, 136u8, 160u8, 45u8, 57u8, 197u8, 54u8, 30u8, 251u8, 171u8,
                19u8, 187u8, 155u8, 58u8, 167u8, 202u8, 163u8, 246u8, 214u8, 183u8, 134u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    timelockDelay: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.timelockDelay),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.safe.clone())
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
                    &self.safe,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for GuardConfigured {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GuardConfigured> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GuardConfigured) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Message(string)` and selector `0x51a7f65c6325882f237d4aeb43228179cfad48b868511d508e24b4437a819137`.
```solidity
event Message(string message);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Message {
        #[allow(missing_docs)]
        pub message: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for Message {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Message(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                81u8, 167u8, 246u8, 92u8, 99u8, 37u8, 136u8, 47u8, 35u8, 125u8, 74u8,
                235u8, 67u8, 34u8, 129u8, 121u8, 207u8, 173u8, 72u8, 184u8, 104u8, 81u8,
                29u8, 80u8, 142u8, 36u8, 180u8, 67u8, 122u8, 129u8, 145u8, 55u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { message: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.message,
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
        impl alloy_sol_types::private::IntoLogData for Message {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Message> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Message) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ModuleCleared(address)` and selector `0x9c6ff02e684f8a81389ed942fea1147c16b8cc5fe79f2bfbc520d44e214aed4b`.
```solidity
event ModuleCleared(address indexed safe);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ModuleCleared {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ModuleCleared {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ModuleCleared(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 111u8, 240u8, 46u8, 104u8, 79u8, 138u8, 129u8, 56u8, 158u8, 217u8,
                66u8, 254u8, 161u8, 20u8, 124u8, 22u8, 184u8, 204u8, 95u8, 231u8, 159u8,
                43u8, 251u8, 197u8, 32u8, 212u8, 78u8, 33u8, 74u8, 237u8, 75u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { safe: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.safe.clone())
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
                    &self.safe,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ModuleCleared {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ModuleCleared> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ModuleCleared) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ModuleConfigured(address,uint256,address)` and selector `0x47e2ddae3ece2c77f60d8e9e5a89a50f1d1374a87fb11956d27a6b4986bba17e`.
```solidity
event ModuleConfigured(address indexed safe, uint256 livenessResponsePeriod, address fallbackOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ModuleConfigured {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub livenessResponsePeriod: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fallbackOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ModuleConfigured {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ModuleConfigured(address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                71u8, 226u8, 221u8, 174u8, 62u8, 206u8, 44u8, 119u8, 246u8, 13u8, 142u8,
                158u8, 90u8, 137u8, 165u8, 15u8, 29u8, 19u8, 116u8, 168u8, 127u8, 177u8,
                25u8, 86u8, 210u8, 122u8, 107u8, 73u8, 134u8, 187u8, 161u8, 126u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    livenessResponsePeriod: data.0,
                    fallbackOwner: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.livenessResponsePeriod,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.fallbackOwner,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.safe.clone())
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
                    &self.safe,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ModuleConfigured {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ModuleConfigured> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ModuleConfigured) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TransactionCancelled(address,bytes32)` and selector `0xa42fd857b47d3d04f5b29f35cb05343f66b317633d2dc2910726bd4bca1a1625`.
```solidity
event TransactionCancelled(address indexed safe, bytes32 indexed txHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionCancelled {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for TransactionCancelled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TransactionCancelled(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 47u8, 216u8, 87u8, 180u8, 125u8, 61u8, 4u8, 245u8, 178u8, 159u8,
                53u8, 203u8, 5u8, 52u8, 63u8, 102u8, 179u8, 23u8, 99u8, 61u8, 45u8,
                194u8, 145u8, 7u8, 38u8, 189u8, 75u8, 202u8, 26u8, 22u8, 37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    txHash: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.safe.clone(), self.txHash.clone())
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
                    &self.safe,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.txHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionCancelled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionCancelled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionCancelled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TransactionExecuted(address,bytes32)` and selector `0xdd4b9b318b98162cb1e7b52752a3fd110d5b7966f3b50884c1cd3bd04058e5c7`.
```solidity
event TransactionExecuted(address indexed safe, bytes32 indexed txHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionExecuted {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for TransactionExecuted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TransactionExecuted(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8, 75u8, 155u8, 49u8, 139u8, 152u8, 22u8, 44u8, 177u8, 231u8, 181u8,
                39u8, 82u8, 163u8, 253u8, 17u8, 13u8, 91u8, 121u8, 102u8, 243u8, 181u8,
                8u8, 132u8, 193u8, 205u8, 59u8, 208u8, 64u8, 88u8, 229u8, 199u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    txHash: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self.safe.clone(), self.txHash.clone())
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
                    &self.safe,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.txHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TransactionScheduled(address,bytes32,uint256)` and selector `0x653f8f6fce2a503b2dfca34e95b3e4902254a11765d2658c0e5af1d64ab276cf`.
```solidity
event TransactionScheduled(address indexed safe, bytes32 indexed txHash, uint256 executionTime);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionScheduled {
        #[allow(missing_docs)]
        pub safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub executionTime: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for TransactionScheduled {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TransactionScheduled(address,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                101u8, 63u8, 143u8, 111u8, 206u8, 42u8, 80u8, 59u8, 45u8, 252u8, 163u8,
                78u8, 149u8, 179u8, 228u8, 144u8, 34u8, 84u8, 161u8, 23u8, 101u8, 210u8,
                101u8, 140u8, 14u8, 90u8, 241u8, 214u8, 74u8, 178u8, 118u8, 207u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    safe: topics.1,
                    txHash: topics.2,
                    executionTime: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.executionTime),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.safe.clone(), self.txHash.clone())
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
                    &self.safe,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.txHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionScheduled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionScheduled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionScheduled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelTransaction(address,bytes32,uint256,bytes)` and selector `0x096e01f7`.
```solidity
function cancelTransaction(address _safe, bytes32 _txHash, uint256 _nonce, bytes memory _signatures) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelTransactionCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _signatures: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`cancelTransaction(address,bytes32,uint256,bytes)`](cancelTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelTransactionReturn {}
    #[allow(
        non_camel_case_types,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<cancelTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelTransactionCall) -> Self {
                    (value._safe, value._txHash, value._nonce, value._signatures)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _safe: tuple.0,
                        _txHash: tuple.1,
                        _nonce: tuple.2,
                        _signatures: tuple.3,
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
            impl ::core::convert::From<cancelTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelTransactionReturn {
            fn _tokenize(
                &self,
            ) -> <cancelTransactionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelTransactionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelTransaction(address,bytes32,uint256,bytes)";
            const SELECTOR: [u8; 4] = [9u8, 110u8, 1u8, 247u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._txHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._nonce),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatures,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelTransactionReturn::_tokenize(ret)
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
    /**Function with signature `cancellationThreshold(address)` and selector `0xe647dee1`.
```solidity
function cancellationThreshold(address _safe) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancellationThresholdCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`cancellationThreshold(address)`](cancellationThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancellationThresholdReturn {
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
            impl ::core::convert::From<cancellationThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancellationThresholdCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancellationThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
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
            impl ::core::convert::From<cancellationThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancellationThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancellationThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancellationThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancellationThreshold(address)";
            const SELECTOR: [u8; 4] = [230u8, 71u8, 222u8, 225u8];
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
                        let r: cancellationThresholdReturn = r.into();
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
                        let r: cancellationThresholdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `challenge(address)` and selector `0x72fb9703`.
```solidity
function challenge(address _safe) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`challenge(address)`](challengeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<challengeCall> for UnderlyingRustTuple<'_> {
                fn from(value: challengeCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
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
            impl ::core::convert::From<challengeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: challengeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl challengeReturn {
            fn _tokenize(
                &self,
            ) -> <challengeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = challengeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challenge(address)";
            const SELECTOR: [u8; 4] = [114u8, 251u8, 151u8, 3u8];
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
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                challengeReturn::_tokenize(ret)
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
    /**Function with signature `challengeStartTime(address)` and selector `0x96de45a4`.
```solidity
function challengeStartTime(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeStartTimeCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`challengeStartTime(address)`](challengeStartTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeStartTimeReturn {
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
            impl ::core::convert::From<challengeStartTimeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeStartTimeCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeStartTimeCall {
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
            impl ::core::convert::From<challengeStartTimeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeStartTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeStartTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeStartTimeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengeStartTime(address)";
            const SELECTOR: [u8; 4] = [150u8, 222u8, 69u8, 164u8];
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
                        let r: challengeStartTimeReturn = r.into();
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
                        let r: challengeStartTimeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `changeOwnershipToFallback(address)` and selector `0xbceb8eda`.
```solidity
function changeOwnershipToFallback(address _safe) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct changeOwnershipToFallbackCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`changeOwnershipToFallback(address)`](changeOwnershipToFallbackCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct changeOwnershipToFallbackReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<changeOwnershipToFallbackCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: changeOwnershipToFallbackCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for changeOwnershipToFallbackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
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
            impl ::core::convert::From<changeOwnershipToFallbackReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: changeOwnershipToFallbackReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for changeOwnershipToFallbackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl changeOwnershipToFallbackReturn {
            fn _tokenize(
                &self,
            ) -> <changeOwnershipToFallbackCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for changeOwnershipToFallbackCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = changeOwnershipToFallbackReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "changeOwnershipToFallback(address)";
            const SELECTOR: [u8; 4] = [188u8, 235u8, 142u8, 218u8];
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
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                changeOwnershipToFallbackReturn::_tokenize(ret)
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
    /**Function with signature `checkAfterExecution(bytes32,bool)` and selector `0x93271368`.
```solidity
function checkAfterExecution(bytes32 _txHash, bool _success) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAfterExecutionCall {
        #[allow(missing_docs)]
        pub _txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _success: bool,
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
                    (value._txHash, value._success)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAfterExecutionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _txHash: tuple.0,
                        _success: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self._txHash),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._success,
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
function checkTransaction(address _to, uint256 _value, bytes memory _data, Enum.Operation _operation, uint256 _safeTxGas, uint256 _baseGas, uint256 _gasPrice, address _gasToken, address _refundReceiver, bytes memory, address _msgSender) external;
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
        pub _9: alloy::sol_types::private::Bytes,
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
                        value._9,
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
                        _9: tuple.9,
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
                        &self._9,
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
    /**Function with signature `clearLivenessModule()` and selector `0x6092b318`.
```solidity
function clearLivenessModule() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearLivenessModuleCall;
    ///Container type for the return parameters of the [`clearLivenessModule()`](clearLivenessModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearLivenessModuleReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<clearLivenessModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearLivenessModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearLivenessModuleCall {
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
            impl ::core::convert::From<clearLivenessModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearLivenessModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearLivenessModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl clearLivenessModuleReturn {
            fn _tokenize(
                &self,
            ) -> <clearLivenessModuleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clearLivenessModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearLivenessModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clearLivenessModule()";
            const SELECTOR: [u8; 4] = [96u8, 146u8, 179u8, 24u8];
            #[inline]
            fn new<'a>(
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
                clearLivenessModuleReturn::_tokenize(ret)
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
    /**Function with signature `clearTimelockGuard()` and selector `0x5c3b4510`.
```solidity
function clearTimelockGuard() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearTimelockGuardCall;
    ///Container type for the return parameters of the [`clearTimelockGuard()`](clearTimelockGuardCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearTimelockGuardReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<clearTimelockGuardCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearTimelockGuardCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearTimelockGuardCall {
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
            impl ::core::convert::From<clearTimelockGuardReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearTimelockGuardReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearTimelockGuardReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl clearTimelockGuardReturn {
            fn _tokenize(
                &self,
            ) -> <clearTimelockGuardCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clearTimelockGuardCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearTimelockGuardReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clearTimelockGuard()";
            const SELECTOR: [u8; 4] = [92u8, 59u8, 69u8, 16u8];
            #[inline]
            fn new<'a>(
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
                clearTimelockGuardReturn::_tokenize(ret)
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
    /**Function with signature `configureLivenessModule((uint256,address))` and selector `0x05ccf606`.
```solidity
function configureLivenessModule(LivenessModule2.ModuleConfig memory _config) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureLivenessModuleCall {
        #[allow(missing_docs)]
        pub _config: <LivenessModule2::ModuleConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`configureLivenessModule((uint256,address))`](configureLivenessModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureLivenessModuleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (LivenessModule2::ModuleConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LivenessModule2::ModuleConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<configureLivenessModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureLivenessModuleCall) -> Self {
                    (value._config,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureLivenessModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _config: tuple.0 }
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
            impl ::core::convert::From<configureLivenessModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureLivenessModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureLivenessModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl configureLivenessModuleReturn {
            fn _tokenize(
                &self,
            ) -> <configureLivenessModuleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configureLivenessModuleCall {
            type Parameters<'a> = (LivenessModule2::ModuleConfig,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = configureLivenessModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configureLivenessModule((uint256,address))";
            const SELECTOR: [u8; 4] = [5u8, 204u8, 246u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LivenessModule2::ModuleConfig as alloy_sol_types::SolType>::tokenize(
                        &self._config,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                configureLivenessModuleReturn::_tokenize(ret)
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
    /**Function with signature `configureTimelockGuard(uint256)` and selector `0x65cf50ec`.
```solidity
function configureTimelockGuard(uint256 _timelockDelay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureTimelockGuardCall {
        #[allow(missing_docs)]
        pub _timelockDelay: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`configureTimelockGuard(uint256)`](configureTimelockGuardCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureTimelockGuardReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<configureTimelockGuardCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureTimelockGuardCall) -> Self {
                    (value._timelockDelay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureTimelockGuardCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _timelockDelay: tuple.0 }
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
            impl ::core::convert::From<configureTimelockGuardReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureTimelockGuardReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureTimelockGuardReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl configureTimelockGuardReturn {
            fn _tokenize(
                &self,
            ) -> <configureTimelockGuardCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configureTimelockGuardCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = configureTimelockGuardReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configureTimelockGuard(uint256)";
            const SELECTOR: [u8; 4] = [101u8, 207u8, 80u8, 236u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._timelockDelay),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                configureTimelockGuardReturn::_tokenize(ret)
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
    /**Function with signature `getChallengePeriodEnd(address)` and selector `0xe10ffc92`.
```solidity
function getChallengePeriodEnd(address _safe) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChallengePeriodEndCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getChallengePeriodEnd(address)`](getChallengePeriodEndCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChallengePeriodEndReturn {
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
            impl ::core::convert::From<getChallengePeriodEndCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getChallengePeriodEndCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getChallengePeriodEndCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
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
            impl ::core::convert::From<getChallengePeriodEndReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getChallengePeriodEndReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getChallengePeriodEndReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getChallengePeriodEndCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getChallengePeriodEnd(address)";
            const SELECTOR: [u8; 4] = [225u8, 15u8, 252u8, 146u8];
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
                        let r: getChallengePeriodEndReturn = r.into();
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
                        let r: getChallengePeriodEndReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `livenessSafeConfiguration(address)` and selector `0x47c03223`.
```solidity
function livenessSafeConfiguration(address _safe) external view returns (LivenessModule2.ModuleConfig memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct livenessSafeConfigurationCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`livenessSafeConfiguration(address)`](livenessSafeConfigurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct livenessSafeConfigurationReturn {
        #[allow(missing_docs)]
        pub _0: <LivenessModule2::ModuleConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<livenessSafeConfigurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: livenessSafeConfigurationCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for livenessSafeConfigurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (LivenessModule2::ModuleConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LivenessModule2::ModuleConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<livenessSafeConfigurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: livenessSafeConfigurationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for livenessSafeConfigurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for livenessSafeConfigurationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <LivenessModule2::ModuleConfig as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (LivenessModule2::ModuleConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "livenessSafeConfiguration(address)";
            const SELECTOR: [u8; 4] = [71u8, 192u8, 50u8, 35u8];
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
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <LivenessModule2::ModuleConfig as alloy_sol_types::SolType>::tokenize(
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
                        let r: livenessSafeConfigurationReturn = r.into();
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
                        let r: livenessSafeConfigurationReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `maxCancellationThreshold(address)` and selector `0xc9713656`.
```solidity
function maxCancellationThreshold(address _safe) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxCancellationThresholdCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`maxCancellationThreshold(address)`](maxCancellationThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxCancellationThresholdReturn {
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
            impl ::core::convert::From<maxCancellationThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxCancellationThresholdCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxCancellationThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
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
            impl ::core::convert::From<maxCancellationThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxCancellationThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxCancellationThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxCancellationThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxCancellationThreshold(address)";
            const SELECTOR: [u8; 4] = [201u8, 113u8, 54u8, 86u8];
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
                        let r: maxCancellationThresholdReturn = r.into();
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
                        let r: maxCancellationThresholdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pendingTransactions(address)` and selector `0xc127fd39`.
```solidity
function pendingTransactions(address _safe) external view returns (TimelockGuard.ScheduledTransaction[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingTransactionsCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`pendingTransactions(address)`](pendingTransactionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingTransactionsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <TimelockGuard::ScheduledTransaction as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<pendingTransactionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingTransactionsCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingTransactionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<TimelockGuard::ScheduledTransaction>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <TimelockGuard::ScheduledTransaction as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<pendingTransactionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingTransactionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingTransactionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingTransactionsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <TimelockGuard::ScheduledTransaction as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<TimelockGuard::ScheduledTransaction>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingTransactions(address)";
            const SELECTOR: [u8; 4] = [193u8, 39u8, 253u8, 57u8];
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
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        TimelockGuard::ScheduledTransaction,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: pendingTransactionsReturn = r.into();
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
                        let r: pendingTransactionsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `respond()` and selector `0xdb16aab9`.
```solidity
function respond() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondCall;
    ///Container type for the return parameters of the [`respond()`](respondCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<respondCall> for UnderlyingRustTuple<'_> {
                fn from(value: respondCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondCall {
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
            impl ::core::convert::From<respondReturn> for UnderlyingRustTuple<'_> {
                fn from(value: respondReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl respondReturn {
            fn _tokenize(
                &self,
            ) -> <respondCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respond()";
            const SELECTOR: [u8; 4] = [219u8, 22u8, 170u8, 185u8];
            #[inline]
            fn new<'a>(
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
                respondReturn::_tokenize(ret)
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
    /**Function with signature `scheduleTransaction(address,uint256,(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address),bytes)` and selector `0x0e40beec`.
```solidity
function scheduleTransaction(address _safe, uint256 _nonce, TimelockGuard.ExecTransactionParams memory _params, bytes memory _signatures) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleTransactionCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _params: <TimelockGuard::ExecTransactionParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _signatures: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`scheduleTransaction(address,uint256,(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address),bytes)`](scheduleTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleTransactionReturn {}
    #[allow(
        non_camel_case_types,
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
                TimelockGuard::ExecTransactionParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <TimelockGuard::ExecTransactionParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<scheduleTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleTransactionCall) -> Self {
                    (value._safe, value._nonce, value._params, value._signatures)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _safe: tuple.0,
                        _nonce: tuple.1,
                        _params: tuple.2,
                        _signatures: tuple.3,
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
            impl ::core::convert::From<scheduleTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scheduleTransactionReturn {
            fn _tokenize(
                &self,
            ) -> <scheduleTransactionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduleTransactionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                TimelockGuard::ExecTransactionParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scheduleTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scheduleTransaction(address,uint256,(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address),bytes)";
            const SELECTOR: [u8; 4] = [14u8, 64u8, 190u8, 236u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._nonce),
                    <TimelockGuard::ExecTransactionParams as alloy_sol_types::SolType>::tokenize(
                        &self._params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatures,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scheduleTransactionReturn::_tokenize(ret)
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
    /**Function with signature `scheduledTransaction(address,bytes32)` and selector `0x12492d97`.
```solidity
function scheduledTransaction(address _safe, bytes32 _txHash) external view returns (TimelockGuard.ScheduledTransaction memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduledTransactionCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _txHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`scheduledTransaction(address,bytes32)`](scheduledTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduledTransactionReturn {
        #[allow(missing_docs)]
        pub _0: <TimelockGuard::ScheduledTransaction as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<scheduledTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduledTransactionCall) -> Self {
                    (value._safe, value._txHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduledTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _safe: tuple.0,
                        _txHash: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (TimelockGuard::ScheduledTransaction,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <TimelockGuard::ScheduledTransaction as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<scheduledTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduledTransactionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduledTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduledTransactionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <TimelockGuard::ScheduledTransaction as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (TimelockGuard::ScheduledTransaction,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scheduledTransaction(address,bytes32)";
            const SELECTOR: [u8; 4] = [18u8, 73u8, 45u8, 151u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._txHash),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <TimelockGuard::ScheduledTransaction as alloy_sol_types::SolType>::tokenize(
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
                        let r: scheduledTransactionReturn = r.into();
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
                        let r: scheduledTransactionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `signCancellation(bytes32)` and selector `0x06d8b7e2`.
```solidity
function signCancellation(bytes32) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signCancellationCall(pub alloy::sol_types::private::FixedBytes<32>);
    ///Container type for the return parameters of the [`signCancellation(bytes32)`](signCancellationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signCancellationReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<signCancellationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: signCancellationCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for signCancellationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<signCancellationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: signCancellationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for signCancellationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl signCancellationReturn {
            fn _tokenize(
                &self,
            ) -> <signCancellationCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for signCancellationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = signCancellationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "signCancellation(bytes32)";
            const SELECTOR: [u8; 4] = [6u8, 216u8, 183u8, 226u8];
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
                signCancellationReturn::_tokenize(ret)
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
    /**Function with signature `timelockDelay(address)` and selector `0x5698b16a`.
```solidity
function timelockDelay(address _safe) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct timelockDelayCall {
        #[allow(missing_docs)]
        pub _safe: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`timelockDelay(address)`](timelockDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct timelockDelayReturn {
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
            impl ::core::convert::From<timelockDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: timelockDelayCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for timelockDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
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
            impl ::core::convert::From<timelockDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: timelockDelayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for timelockDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for timelockDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "timelockDelay(address)";
            const SELECTOR: [u8; 4] = [86u8, 152u8, 177u8, 106u8];
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
                        let r: timelockDelayReturn = r.into();
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
                        let r: timelockDelayReturn = r.into();
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
    ///Container for all the [`SaferSafes`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SaferSafesCalls {
        #[allow(missing_docs)]
        cancelTransaction(cancelTransactionCall),
        #[allow(missing_docs)]
        cancellationThreshold(cancellationThresholdCall),
        #[allow(missing_docs)]
        challenge(challengeCall),
        #[allow(missing_docs)]
        challengeStartTime(challengeStartTimeCall),
        #[allow(missing_docs)]
        changeOwnershipToFallback(changeOwnershipToFallbackCall),
        #[allow(missing_docs)]
        checkAfterExecution(checkAfterExecutionCall),
        #[allow(missing_docs)]
        checkTransaction(checkTransactionCall),
        #[allow(missing_docs)]
        clearLivenessModule(clearLivenessModuleCall),
        #[allow(missing_docs)]
        clearTimelockGuard(clearTimelockGuardCall),
        #[allow(missing_docs)]
        configureLivenessModule(configureLivenessModuleCall),
        #[allow(missing_docs)]
        configureTimelockGuard(configureTimelockGuardCall),
        #[allow(missing_docs)]
        getChallengePeriodEnd(getChallengePeriodEndCall),
        #[allow(missing_docs)]
        livenessSafeConfiguration(livenessSafeConfigurationCall),
        #[allow(missing_docs)]
        maxCancellationThreshold(maxCancellationThresholdCall),
        #[allow(missing_docs)]
        pendingTransactions(pendingTransactionsCall),
        #[allow(missing_docs)]
        respond(respondCall),
        #[allow(missing_docs)]
        scheduleTransaction(scheduleTransactionCall),
        #[allow(missing_docs)]
        scheduledTransaction(scheduledTransactionCall),
        #[allow(missing_docs)]
        signCancellation(signCancellationCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        timelockDelay(timelockDelayCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl SaferSafesCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [5u8, 204u8, 246u8, 6u8],
            [6u8, 216u8, 183u8, 226u8],
            [9u8, 110u8, 1u8, 247u8],
            [14u8, 64u8, 190u8, 236u8],
            [18u8, 73u8, 45u8, 151u8],
            [71u8, 192u8, 50u8, 35u8],
            [84u8, 253u8, 77u8, 80u8],
            [86u8, 152u8, 177u8, 106u8],
            [92u8, 59u8, 69u8, 16u8],
            [96u8, 146u8, 179u8, 24u8],
            [101u8, 207u8, 80u8, 236u8],
            [114u8, 251u8, 151u8, 3u8],
            [117u8, 240u8, 187u8, 82u8],
            [147u8, 39u8, 19u8, 104u8],
            [150u8, 222u8, 69u8, 164u8],
            [188u8, 235u8, 142u8, 218u8],
            [193u8, 39u8, 253u8, 57u8],
            [201u8, 113u8, 54u8, 86u8],
            [219u8, 22u8, 170u8, 185u8],
            [225u8, 15u8, 252u8, 146u8],
            [230u8, 71u8, 222u8, 225u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(configureLivenessModule),
            ::core::stringify!(signCancellation),
            ::core::stringify!(cancelTransaction),
            ::core::stringify!(scheduleTransaction),
            ::core::stringify!(scheduledTransaction),
            ::core::stringify!(livenessSafeConfiguration),
            ::core::stringify!(version),
            ::core::stringify!(timelockDelay),
            ::core::stringify!(clearTimelockGuard),
            ::core::stringify!(clearLivenessModule),
            ::core::stringify!(configureTimelockGuard),
            ::core::stringify!(challenge),
            ::core::stringify!(checkTransaction),
            ::core::stringify!(checkAfterExecution),
            ::core::stringify!(challengeStartTime),
            ::core::stringify!(changeOwnershipToFallback),
            ::core::stringify!(pendingTransactions),
            ::core::stringify!(maxCancellationThreshold),
            ::core::stringify!(respond),
            ::core::stringify!(getChallengePeriodEnd),
            ::core::stringify!(cancellationThreshold),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <configureLivenessModuleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <signCancellationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scheduleTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scheduledTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <livenessSafeConfigurationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <timelockDelayCall as alloy_sol_types::SolCall>::SIGNATURE,
            <clearTimelockGuardCall as alloy_sol_types::SolCall>::SIGNATURE,
            <clearLivenessModuleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <configureTimelockGuardCall as alloy_sol_types::SolCall>::SIGNATURE,
            <challengeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkAfterExecutionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <challengeStartTimeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <changeOwnershipToFallbackCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pendingTransactionsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxCancellationThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <respondCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getChallengePeriodEndCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancellationThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SaferSafesCalls {
        const NAME: &'static str = "SaferSafesCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::cancelTransaction(_) => {
                    <cancelTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancellationThreshold(_) => {
                    <cancellationThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challenge(_) => {
                    <challengeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challengeStartTime(_) => {
                    <challengeStartTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::changeOwnershipToFallback(_) => {
                    <changeOwnershipToFallbackCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkAfterExecution(_) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkTransaction(_) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clearLivenessModule(_) => {
                    <clearLivenessModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clearTimelockGuard(_) => {
                    <clearTimelockGuardCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configureLivenessModule(_) => {
                    <configureLivenessModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configureTimelockGuard(_) => {
                    <configureTimelockGuardCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getChallengePeriodEnd(_) => {
                    <getChallengePeriodEndCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::livenessSafeConfiguration(_) => {
                    <livenessSafeConfigurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxCancellationThreshold(_) => {
                    <maxCancellationThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pendingTransactions(_) => {
                    <pendingTransactionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respond(_) => <respondCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::scheduleTransaction(_) => {
                    <scheduleTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scheduledTransaction(_) => {
                    <scheduledTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::signCancellation(_) => {
                    <signCancellationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::timelockDelay(_) => {
                    <timelockDelayCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SaferSafesCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn configureLivenessModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <configureLivenessModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::configureLivenessModule)
                    }
                    configureLivenessModule
                },
                {
                    fn signCancellation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <signCancellationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::signCancellation)
                    }
                    signCancellation
                },
                {
                    fn cancelTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <cancelTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::cancelTransaction)
                    }
                    cancelTransaction
                },
                {
                    fn scheduleTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <scheduleTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::scheduleTransaction)
                    }
                    scheduleTransaction
                },
                {
                    fn scheduledTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <scheduledTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::scheduledTransaction)
                    }
                    scheduledTransaction
                },
                {
                    fn livenessSafeConfiguration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <livenessSafeConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::livenessSafeConfiguration)
                    }
                    livenessSafeConfiguration
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SaferSafesCalls::version)
                    }
                    version
                },
                {
                    fn timelockDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <timelockDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::timelockDelay)
                    }
                    timelockDelay
                },
                {
                    fn clearTimelockGuard(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <clearTimelockGuardCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::clearTimelockGuard)
                    }
                    clearTimelockGuard
                },
                {
                    fn clearLivenessModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <clearLivenessModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::clearLivenessModule)
                    }
                    clearLivenessModule
                },
                {
                    fn configureTimelockGuard(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <configureTimelockGuardCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::configureTimelockGuard)
                    }
                    configureTimelockGuard
                },
                {
                    fn challenge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <challengeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SaferSafesCalls::challenge)
                    }
                    challenge
                },
                {
                    fn checkTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <checkTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::checkTransaction)
                    }
                    checkTransaction
                },
                {
                    fn checkAfterExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::checkAfterExecution)
                    }
                    checkAfterExecution
                },
                {
                    fn challengeStartTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <challengeStartTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::challengeStartTime)
                    }
                    challengeStartTime
                },
                {
                    fn changeOwnershipToFallback(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <changeOwnershipToFallbackCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::changeOwnershipToFallback)
                    }
                    changeOwnershipToFallback
                },
                {
                    fn pendingTransactions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <pendingTransactionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::pendingTransactions)
                    }
                    pendingTransactions
                },
                {
                    fn maxCancellationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <maxCancellationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::maxCancellationThreshold)
                    }
                    maxCancellationThreshold
                },
                {
                    fn respond(data: &[u8]) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <respondCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SaferSafesCalls::respond)
                    }
                    respond
                },
                {
                    fn getChallengePeriodEnd(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <getChallengePeriodEndCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::getChallengePeriodEnd)
                    }
                    getChallengePeriodEnd
                },
                {
                    fn cancellationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <cancellationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesCalls::cancellationThreshold)
                    }
                    cancellationThreshold
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
            ) -> alloy_sol_types::Result<SaferSafesCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn configureLivenessModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <configureLivenessModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::configureLivenessModule)
                    }
                    configureLivenessModule
                },
                {
                    fn signCancellation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <signCancellationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::signCancellation)
                    }
                    signCancellation
                },
                {
                    fn cancelTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <cancelTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::cancelTransaction)
                    }
                    cancelTransaction
                },
                {
                    fn scheduleTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <scheduleTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::scheduleTransaction)
                    }
                    scheduleTransaction
                },
                {
                    fn scheduledTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <scheduledTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::scheduledTransaction)
                    }
                    scheduledTransaction
                },
                {
                    fn livenessSafeConfiguration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <livenessSafeConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::livenessSafeConfiguration)
                    }
                    livenessSafeConfiguration
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::version)
                    }
                    version
                },
                {
                    fn timelockDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <timelockDelayCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::timelockDelay)
                    }
                    timelockDelay
                },
                {
                    fn clearTimelockGuard(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <clearTimelockGuardCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::clearTimelockGuard)
                    }
                    clearTimelockGuard
                },
                {
                    fn clearLivenessModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <clearLivenessModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::clearLivenessModule)
                    }
                    clearLivenessModule
                },
                {
                    fn configureTimelockGuard(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <configureTimelockGuardCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::configureTimelockGuard)
                    }
                    configureTimelockGuard
                },
                {
                    fn challenge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <challengeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::challenge)
                    }
                    challenge
                },
                {
                    fn checkTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <checkTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::checkTransaction)
                    }
                    checkTransaction
                },
                {
                    fn checkAfterExecution(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::checkAfterExecution)
                    }
                    checkAfterExecution
                },
                {
                    fn challengeStartTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <challengeStartTimeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::challengeStartTime)
                    }
                    challengeStartTime
                },
                {
                    fn changeOwnershipToFallback(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <changeOwnershipToFallbackCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::changeOwnershipToFallback)
                    }
                    changeOwnershipToFallback
                },
                {
                    fn pendingTransactions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <pendingTransactionsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::pendingTransactions)
                    }
                    pendingTransactions
                },
                {
                    fn maxCancellationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <maxCancellationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::maxCancellationThreshold)
                    }
                    maxCancellationThreshold
                },
                {
                    fn respond(data: &[u8]) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <respondCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::respond)
                    }
                    respond
                },
                {
                    fn getChallengePeriodEnd(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <getChallengePeriodEndCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::getChallengePeriodEnd)
                    }
                    getChallengePeriodEnd
                },
                {
                    fn cancellationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesCalls> {
                        <cancellationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesCalls::cancellationThreshold)
                    }
                    cancellationThreshold
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
                Self::cancelTransaction(inner) => {
                    <cancelTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancellationThreshold(inner) => {
                    <cancellationThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challenge(inner) => {
                    <challengeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::challengeStartTime(inner) => {
                    <challengeStartTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::changeOwnershipToFallback(inner) => {
                    <changeOwnershipToFallbackCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::clearLivenessModule(inner) => {
                    <clearLivenessModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clearTimelockGuard(inner) => {
                    <clearTimelockGuardCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::configureLivenessModule(inner) => {
                    <configureLivenessModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::configureTimelockGuard(inner) => {
                    <configureTimelockGuardCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getChallengePeriodEnd(inner) => {
                    <getChallengePeriodEndCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::livenessSafeConfiguration(inner) => {
                    <livenessSafeConfigurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxCancellationThreshold(inner) => {
                    <maxCancellationThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pendingTransactions(inner) => {
                    <pendingTransactionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respond(inner) => {
                    <respondCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::scheduleTransaction(inner) => {
                    <scheduleTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scheduledTransaction(inner) => {
                    <scheduledTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::signCancellation(inner) => {
                    <signCancellationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::timelockDelay(inner) => {
                    <timelockDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::cancelTransaction(inner) => {
                    <cancelTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancellationThreshold(inner) => {
                    <cancellationThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challenge(inner) => {
                    <challengeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challengeStartTime(inner) => {
                    <challengeStartTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::changeOwnershipToFallback(inner) => {
                    <changeOwnershipToFallbackCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::clearLivenessModule(inner) => {
                    <clearLivenessModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::clearTimelockGuard(inner) => {
                    <clearTimelockGuardCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::configureLivenessModule(inner) => {
                    <configureLivenessModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::configureTimelockGuard(inner) => {
                    <configureTimelockGuardCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getChallengePeriodEnd(inner) => {
                    <getChallengePeriodEndCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::livenessSafeConfiguration(inner) => {
                    <livenessSafeConfigurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxCancellationThreshold(inner) => {
                    <maxCancellationThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pendingTransactions(inner) => {
                    <pendingTransactionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::respond(inner) => {
                    <respondCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::scheduleTransaction(inner) => {
                    <scheduleTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scheduledTransaction(inner) => {
                    <scheduledTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::signCancellation(inner) => {
                    <signCancellationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::timelockDelay(inner) => {
                    <timelockDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`SaferSafes`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SaferSafesErrors {
        #[allow(missing_docs)]
        LivenessModule2_ChallengeAlreadyExists(LivenessModule2_ChallengeAlreadyExists),
        #[allow(missing_docs)]
        LivenessModule2_ChallengeDoesNotExist(LivenessModule2_ChallengeDoesNotExist),
        #[allow(missing_docs)]
        LivenessModule2_InvalidFallbackOwner(LivenessModule2_InvalidFallbackOwner),
        #[allow(missing_docs)]
        LivenessModule2_InvalidResponsePeriod(LivenessModule2_InvalidResponsePeriod),
        #[allow(missing_docs)]
        LivenessModule2_InvalidVersion(LivenessModule2_InvalidVersion),
        #[allow(missing_docs)]
        LivenessModule2_ModuleNotConfigured(LivenessModule2_ModuleNotConfigured),
        #[allow(missing_docs)]
        LivenessModule2_ModuleNotEnabled(LivenessModule2_ModuleNotEnabled),
        #[allow(missing_docs)]
        LivenessModule2_ModuleStillEnabled(LivenessModule2_ModuleStillEnabled),
        #[allow(missing_docs)]
        LivenessModule2_OwnershipTransferFailed(LivenessModule2_OwnershipTransferFailed),
        #[allow(missing_docs)]
        LivenessModule2_ResponsePeriodActive(LivenessModule2_ResponsePeriodActive),
        #[allow(missing_docs)]
        LivenessModule2_ResponsePeriodEnded(LivenessModule2_ResponsePeriodEnded),
        #[allow(missing_docs)]
        LivenessModule2_UnauthorizedCaller(LivenessModule2_UnauthorizedCaller),
        #[allow(missing_docs)]
        SaferSafes_InsufficientLivenessResponsePeriod(
            SaferSafes_InsufficientLivenessResponsePeriod,
        ),
        #[allow(missing_docs)]
        SemverComp_InvalidSemverParts(SemverComp_InvalidSemverParts),
        #[allow(missing_docs)]
        TimelockGuard_GuardNotConfigured(TimelockGuard_GuardNotConfigured),
        #[allow(missing_docs)]
        TimelockGuard_GuardNotEnabled(TimelockGuard_GuardNotEnabled),
        #[allow(missing_docs)]
        TimelockGuard_GuardStillEnabled(TimelockGuard_GuardStillEnabled),
        #[allow(missing_docs)]
        TimelockGuard_InvalidTimelockDelay(TimelockGuard_InvalidTimelockDelay),
        #[allow(missing_docs)]
        TimelockGuard_InvalidVersion(TimelockGuard_InvalidVersion),
        #[allow(missing_docs)]
        TimelockGuard_NotOwner(TimelockGuard_NotOwner),
        #[allow(missing_docs)]
        TimelockGuard_TransactionAlreadyCancelled(
            TimelockGuard_TransactionAlreadyCancelled,
        ),
        #[allow(missing_docs)]
        TimelockGuard_TransactionAlreadyExecuted(
            TimelockGuard_TransactionAlreadyExecuted,
        ),
        #[allow(missing_docs)]
        TimelockGuard_TransactionAlreadyScheduled(
            TimelockGuard_TransactionAlreadyScheduled,
        ),
        #[allow(missing_docs)]
        TimelockGuard_TransactionNotReady(TimelockGuard_TransactionNotReady),
        #[allow(missing_docs)]
        TimelockGuard_TransactionNotScheduled(TimelockGuard_TransactionNotScheduled),
    }
    impl SaferSafesErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 200u8, 89u8, 115u8],
            [8u8, 50u8, 221u8, 105u8],
            [9u8, 249u8, 89u8, 32u8],
            [24u8, 60u8, 164u8, 49u8],
            [39u8, 158u8, 61u8, 36u8],
            [62u8, 139u8, 131u8, 137u8],
            [63u8, 75u8, 41u8, 102u8],
            [71u8, 220u8, 161u8, 206u8],
            [80u8, 60u8, 66u8, 196u8],
            [85u8, 45u8, 209u8, 183u8],
            [88u8, 116u8, 233u8, 79u8],
            [112u8, 163u8, 128u8, 148u8],
            [116u8, 167u8, 233u8, 110u8],
            [128u8, 57u8, 77u8, 230u8],
            [156u8, 217u8, 9u8, 4u8],
            [158u8, 47u8, 124u8, 75u8],
            [158u8, 218u8, 133u8, 140u8],
            [159u8, 218u8, 218u8, 49u8],
            [160u8, 206u8, 34u8, 139u8],
            [160u8, 254u8, 147u8, 155u8],
            [161u8, 242u8, 8u8, 32u8],
            [164u8, 210u8, 52u8, 203u8],
            [218u8, 33u8, 174u8, 217u8],
            [227u8, 48u8, 172u8, 5u8],
            [235u8, 145u8, 26u8, 240u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(TimelockGuard_TransactionNotScheduled),
            ::core::stringify!(TimelockGuard_GuardNotConfigured),
            ::core::stringify!(LivenessModule2_ChallengeDoesNotExist),
            ::core::stringify!(TimelockGuard_TransactionAlreadyExecuted),
            ::core::stringify!(LivenessModule2_ResponsePeriodActive),
            ::core::stringify!(TimelockGuard_TransactionAlreadyCancelled),
            ::core::stringify!(TimelockGuard_GuardNotEnabled),
            ::core::stringify!(SaferSafes_InsufficientLivenessResponsePeriod),
            ::core::stringify!(TimelockGuard_TransactionNotReady),
            ::core::stringify!(LivenessModule2_InvalidFallbackOwner),
            ::core::stringify!(TimelockGuard_NotOwner),
            ::core::stringify!(LivenessModule2_ModuleStillEnabled),
            ::core::stringify!(LivenessModule2_InvalidResponsePeriod),
            ::core::stringify!(TimelockGuard_TransactionAlreadyScheduled),
            ::core::stringify!(LivenessModule2_ChallengeAlreadyExists),
            ::core::stringify!(TimelockGuard_InvalidVersion),
            ::core::stringify!(SemverComp_InvalidSemverParts),
            ::core::stringify!(LivenessModule2_ModuleNotEnabled),
            ::core::stringify!(TimelockGuard_InvalidTimelockDelay),
            ::core::stringify!(LivenessModule2_ModuleNotConfigured),
            ::core::stringify!(LivenessModule2_UnauthorizedCaller),
            ::core::stringify!(TimelockGuard_GuardStillEnabled),
            ::core::stringify!(LivenessModule2_InvalidVersion),
            ::core::stringify!(LivenessModule2_OwnershipTransferFailed),
            ::core::stringify!(LivenessModule2_ResponsePeriodEnded),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <TimelockGuard_TransactionNotScheduled as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_GuardNotConfigured as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_ChallengeDoesNotExist as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_TransactionAlreadyExecuted as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_ResponsePeriodActive as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_TransactionAlreadyCancelled as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_GuardNotEnabled as alloy_sol_types::SolError>::SIGNATURE,
            <SaferSafes_InsufficientLivenessResponsePeriod as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_TransactionNotReady as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_InvalidFallbackOwner as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_NotOwner as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_ModuleStillEnabled as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_InvalidResponsePeriod as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_TransactionAlreadyScheduled as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_ChallengeAlreadyExists as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_InvalidVersion as alloy_sol_types::SolError>::SIGNATURE,
            <SemverComp_InvalidSemverParts as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_ModuleNotEnabled as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_InvalidTimelockDelay as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_ModuleNotConfigured as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_UnauthorizedCaller as alloy_sol_types::SolError>::SIGNATURE,
            <TimelockGuard_GuardStillEnabled as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_InvalidVersion as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_OwnershipTransferFailed as alloy_sol_types::SolError>::SIGNATURE,
            <LivenessModule2_ResponsePeriodEnded as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for SaferSafesErrors {
        const NAME: &'static str = "SaferSafesErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::LivenessModule2_ChallengeAlreadyExists(_) => {
                    <LivenessModule2_ChallengeAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_ChallengeDoesNotExist(_) => {
                    <LivenessModule2_ChallengeDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_InvalidFallbackOwner(_) => {
                    <LivenessModule2_InvalidFallbackOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_InvalidResponsePeriod(_) => {
                    <LivenessModule2_InvalidResponsePeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_InvalidVersion(_) => {
                    <LivenessModule2_InvalidVersion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_ModuleNotConfigured(_) => {
                    <LivenessModule2_ModuleNotConfigured as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_ModuleNotEnabled(_) => {
                    <LivenessModule2_ModuleNotEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_ModuleStillEnabled(_) => {
                    <LivenessModule2_ModuleStillEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_OwnershipTransferFailed(_) => {
                    <LivenessModule2_OwnershipTransferFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_ResponsePeriodActive(_) => {
                    <LivenessModule2_ResponsePeriodActive as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_ResponsePeriodEnded(_) => {
                    <LivenessModule2_ResponsePeriodEnded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LivenessModule2_UnauthorizedCaller(_) => {
                    <LivenessModule2_UnauthorizedCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SaferSafes_InsufficientLivenessResponsePeriod(_) => {
                    <SaferSafes_InsufficientLivenessResponsePeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SemverComp_InvalidSemverParts(_) => {
                    <SemverComp_InvalidSemverParts as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_GuardNotConfigured(_) => {
                    <TimelockGuard_GuardNotConfigured as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_GuardNotEnabled(_) => {
                    <TimelockGuard_GuardNotEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_GuardStillEnabled(_) => {
                    <TimelockGuard_GuardStillEnabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_InvalidTimelockDelay(_) => {
                    <TimelockGuard_InvalidTimelockDelay as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_InvalidVersion(_) => {
                    <TimelockGuard_InvalidVersion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_NotOwner(_) => {
                    <TimelockGuard_NotOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_TransactionAlreadyCancelled(_) => {
                    <TimelockGuard_TransactionAlreadyCancelled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_TransactionAlreadyExecuted(_) => {
                    <TimelockGuard_TransactionAlreadyExecuted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_TransactionAlreadyScheduled(_) => {
                    <TimelockGuard_TransactionAlreadyScheduled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_TransactionNotReady(_) => {
                    <TimelockGuard_TransactionNotReady as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockGuard_TransactionNotScheduled(_) => {
                    <TimelockGuard_TransactionNotScheduled as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SaferSafesErrors>] = &[
                {
                    fn TimelockGuard_TransactionNotScheduled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionNotScheduled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_TransactionNotScheduled)
                    }
                    TimelockGuard_TransactionNotScheduled
                },
                {
                    fn TimelockGuard_GuardNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_GuardNotConfigured as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_GuardNotConfigured)
                    }
                    TimelockGuard_GuardNotConfigured
                },
                {
                    fn LivenessModule2_ChallengeDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ChallengeDoesNotExist)
                    }
                    LivenessModule2_ChallengeDoesNotExist
                },
                {
                    fn TimelockGuard_TransactionAlreadyExecuted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionAlreadyExecuted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SaferSafesErrors::TimelockGuard_TransactionAlreadyExecuted,
                            )
                    }
                    TimelockGuard_TransactionAlreadyExecuted
                },
                {
                    fn LivenessModule2_ResponsePeriodActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ResponsePeriodActive as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ResponsePeriodActive)
                    }
                    LivenessModule2_ResponsePeriodActive
                },
                {
                    fn TimelockGuard_TransactionAlreadyCancelled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionAlreadyCancelled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SaferSafesErrors::TimelockGuard_TransactionAlreadyCancelled,
                            )
                    }
                    TimelockGuard_TransactionAlreadyCancelled
                },
                {
                    fn TimelockGuard_GuardNotEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_GuardNotEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_GuardNotEnabled)
                    }
                    TimelockGuard_GuardNotEnabled
                },
                {
                    fn SaferSafes_InsufficientLivenessResponsePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <SaferSafes_InsufficientLivenessResponsePeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SaferSafesErrors::SaferSafes_InsufficientLivenessResponsePeriod,
                            )
                    }
                    SaferSafes_InsufficientLivenessResponsePeriod
                },
                {
                    fn TimelockGuard_TransactionNotReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionNotReady as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_TransactionNotReady)
                    }
                    TimelockGuard_TransactionNotReady
                },
                {
                    fn LivenessModule2_InvalidFallbackOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_InvalidFallbackOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_InvalidFallbackOwner)
                    }
                    LivenessModule2_InvalidFallbackOwner
                },
                {
                    fn TimelockGuard_NotOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_NotOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_NotOwner)
                    }
                    TimelockGuard_NotOwner
                },
                {
                    fn LivenessModule2_ModuleStillEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ModuleStillEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ModuleStillEnabled)
                    }
                    LivenessModule2_ModuleStillEnabled
                },
                {
                    fn LivenessModule2_InvalidResponsePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_InvalidResponsePeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_InvalidResponsePeriod)
                    }
                    LivenessModule2_InvalidResponsePeriod
                },
                {
                    fn TimelockGuard_TransactionAlreadyScheduled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionAlreadyScheduled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SaferSafesErrors::TimelockGuard_TransactionAlreadyScheduled,
                            )
                    }
                    TimelockGuard_TransactionAlreadyScheduled
                },
                {
                    fn LivenessModule2_ChallengeAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SaferSafesErrors::LivenessModule2_ChallengeAlreadyExists,
                            )
                    }
                    LivenessModule2_ChallengeAlreadyExists
                },
                {
                    fn TimelockGuard_InvalidVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_InvalidVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_InvalidVersion)
                    }
                    TimelockGuard_InvalidVersion
                },
                {
                    fn SemverComp_InvalidSemverParts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <SemverComp_InvalidSemverParts as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::SemverComp_InvalidSemverParts)
                    }
                    SemverComp_InvalidSemverParts
                },
                {
                    fn LivenessModule2_ModuleNotEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ModuleNotEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ModuleNotEnabled)
                    }
                    LivenessModule2_ModuleNotEnabled
                },
                {
                    fn TimelockGuard_InvalidTimelockDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_InvalidTimelockDelay as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_InvalidTimelockDelay)
                    }
                    TimelockGuard_InvalidTimelockDelay
                },
                {
                    fn LivenessModule2_ModuleNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ModuleNotConfigured as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ModuleNotConfigured)
                    }
                    LivenessModule2_ModuleNotConfigured
                },
                {
                    fn LivenessModule2_UnauthorizedCaller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_UnauthorizedCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_UnauthorizedCaller)
                    }
                    LivenessModule2_UnauthorizedCaller
                },
                {
                    fn TimelockGuard_GuardStillEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_GuardStillEnabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_GuardStillEnabled)
                    }
                    TimelockGuard_GuardStillEnabled
                },
                {
                    fn LivenessModule2_InvalidVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_InvalidVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_InvalidVersion)
                    }
                    LivenessModule2_InvalidVersion
                },
                {
                    fn LivenessModule2_OwnershipTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_OwnershipTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                SaferSafesErrors::LivenessModule2_OwnershipTransferFailed,
                            )
                    }
                    LivenessModule2_OwnershipTransferFailed
                },
                {
                    fn LivenessModule2_ResponsePeriodEnded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ResponsePeriodEnded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ResponsePeriodEnded)
                    }
                    LivenessModule2_ResponsePeriodEnded
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
            ) -> alloy_sol_types::Result<SaferSafesErrors>] = &[
                {
                    fn TimelockGuard_TransactionNotScheduled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionNotScheduled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_TransactionNotScheduled)
                    }
                    TimelockGuard_TransactionNotScheduled
                },
                {
                    fn TimelockGuard_GuardNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_GuardNotConfigured as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_GuardNotConfigured)
                    }
                    TimelockGuard_GuardNotConfigured
                },
                {
                    fn LivenessModule2_ChallengeDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ChallengeDoesNotExist)
                    }
                    LivenessModule2_ChallengeDoesNotExist
                },
                {
                    fn TimelockGuard_TransactionAlreadyExecuted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionAlreadyExecuted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SaferSafesErrors::TimelockGuard_TransactionAlreadyExecuted,
                            )
                    }
                    TimelockGuard_TransactionAlreadyExecuted
                },
                {
                    fn LivenessModule2_ResponsePeriodActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ResponsePeriodActive as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ResponsePeriodActive)
                    }
                    LivenessModule2_ResponsePeriodActive
                },
                {
                    fn TimelockGuard_TransactionAlreadyCancelled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionAlreadyCancelled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SaferSafesErrors::TimelockGuard_TransactionAlreadyCancelled,
                            )
                    }
                    TimelockGuard_TransactionAlreadyCancelled
                },
                {
                    fn TimelockGuard_GuardNotEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_GuardNotEnabled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_GuardNotEnabled)
                    }
                    TimelockGuard_GuardNotEnabled
                },
                {
                    fn SaferSafes_InsufficientLivenessResponsePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <SaferSafes_InsufficientLivenessResponsePeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SaferSafesErrors::SaferSafes_InsufficientLivenessResponsePeriod,
                            )
                    }
                    SaferSafes_InsufficientLivenessResponsePeriod
                },
                {
                    fn TimelockGuard_TransactionNotReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionNotReady as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_TransactionNotReady)
                    }
                    TimelockGuard_TransactionNotReady
                },
                {
                    fn LivenessModule2_InvalidFallbackOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_InvalidFallbackOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_InvalidFallbackOwner)
                    }
                    LivenessModule2_InvalidFallbackOwner
                },
                {
                    fn TimelockGuard_NotOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_NotOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_NotOwner)
                    }
                    TimelockGuard_NotOwner
                },
                {
                    fn LivenessModule2_ModuleStillEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ModuleStillEnabled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ModuleStillEnabled)
                    }
                    LivenessModule2_ModuleStillEnabled
                },
                {
                    fn LivenessModule2_InvalidResponsePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_InvalidResponsePeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_InvalidResponsePeriod)
                    }
                    LivenessModule2_InvalidResponsePeriod
                },
                {
                    fn TimelockGuard_TransactionAlreadyScheduled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_TransactionAlreadyScheduled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SaferSafesErrors::TimelockGuard_TransactionAlreadyScheduled,
                            )
                    }
                    TimelockGuard_TransactionAlreadyScheduled
                },
                {
                    fn LivenessModule2_ChallengeAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SaferSafesErrors::LivenessModule2_ChallengeAlreadyExists,
                            )
                    }
                    LivenessModule2_ChallengeAlreadyExists
                },
                {
                    fn TimelockGuard_InvalidVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_InvalidVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_InvalidVersion)
                    }
                    TimelockGuard_InvalidVersion
                },
                {
                    fn SemverComp_InvalidSemverParts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <SemverComp_InvalidSemverParts as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::SemverComp_InvalidSemverParts)
                    }
                    SemverComp_InvalidSemverParts
                },
                {
                    fn LivenessModule2_ModuleNotEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ModuleNotEnabled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ModuleNotEnabled)
                    }
                    LivenessModule2_ModuleNotEnabled
                },
                {
                    fn TimelockGuard_InvalidTimelockDelay(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_InvalidTimelockDelay as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_InvalidTimelockDelay)
                    }
                    TimelockGuard_InvalidTimelockDelay
                },
                {
                    fn LivenessModule2_ModuleNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ModuleNotConfigured as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ModuleNotConfigured)
                    }
                    LivenessModule2_ModuleNotConfigured
                },
                {
                    fn LivenessModule2_UnauthorizedCaller(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_UnauthorizedCaller as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_UnauthorizedCaller)
                    }
                    LivenessModule2_UnauthorizedCaller
                },
                {
                    fn TimelockGuard_GuardStillEnabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <TimelockGuard_GuardStillEnabled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::TimelockGuard_GuardStillEnabled)
                    }
                    TimelockGuard_GuardStillEnabled
                },
                {
                    fn LivenessModule2_InvalidVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_InvalidVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_InvalidVersion)
                    }
                    LivenessModule2_InvalidVersion
                },
                {
                    fn LivenessModule2_OwnershipTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_OwnershipTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                SaferSafesErrors::LivenessModule2_OwnershipTransferFailed,
                            )
                    }
                    LivenessModule2_OwnershipTransferFailed
                },
                {
                    fn LivenessModule2_ResponsePeriodEnded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SaferSafesErrors> {
                        <LivenessModule2_ResponsePeriodEnded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SaferSafesErrors::LivenessModule2_ResponsePeriodEnded)
                    }
                    LivenessModule2_ResponsePeriodEnded
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
                Self::LivenessModule2_ChallengeAlreadyExists(inner) => {
                    <LivenessModule2_ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_ChallengeDoesNotExist(inner) => {
                    <LivenessModule2_ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_InvalidFallbackOwner(inner) => {
                    <LivenessModule2_InvalidFallbackOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_InvalidResponsePeriod(inner) => {
                    <LivenessModule2_InvalidResponsePeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_InvalidVersion(inner) => {
                    <LivenessModule2_InvalidVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_ModuleNotConfigured(inner) => {
                    <LivenessModule2_ModuleNotConfigured as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_ModuleNotEnabled(inner) => {
                    <LivenessModule2_ModuleNotEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_ModuleStillEnabled(inner) => {
                    <LivenessModule2_ModuleStillEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_OwnershipTransferFailed(inner) => {
                    <LivenessModule2_OwnershipTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_ResponsePeriodActive(inner) => {
                    <LivenessModule2_ResponsePeriodActive as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_ResponsePeriodEnded(inner) => {
                    <LivenessModule2_ResponsePeriodEnded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LivenessModule2_UnauthorizedCaller(inner) => {
                    <LivenessModule2_UnauthorizedCaller as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SaferSafes_InsufficientLivenessResponsePeriod(inner) => {
                    <SaferSafes_InsufficientLivenessResponsePeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SemverComp_InvalidSemverParts(inner) => {
                    <SemverComp_InvalidSemverParts as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_GuardNotConfigured(inner) => {
                    <TimelockGuard_GuardNotConfigured as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_GuardNotEnabled(inner) => {
                    <TimelockGuard_GuardNotEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_GuardStillEnabled(inner) => {
                    <TimelockGuard_GuardStillEnabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_InvalidTimelockDelay(inner) => {
                    <TimelockGuard_InvalidTimelockDelay as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_InvalidVersion(inner) => {
                    <TimelockGuard_InvalidVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_NotOwner(inner) => {
                    <TimelockGuard_NotOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_TransactionAlreadyCancelled(inner) => {
                    <TimelockGuard_TransactionAlreadyCancelled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_TransactionAlreadyExecuted(inner) => {
                    <TimelockGuard_TransactionAlreadyExecuted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_TransactionAlreadyScheduled(inner) => {
                    <TimelockGuard_TransactionAlreadyScheduled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_TransactionNotReady(inner) => {
                    <TimelockGuard_TransactionNotReady as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockGuard_TransactionNotScheduled(inner) => {
                    <TimelockGuard_TransactionNotScheduled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::LivenessModule2_ChallengeAlreadyExists(inner) => {
                    <LivenessModule2_ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_ChallengeDoesNotExist(inner) => {
                    <LivenessModule2_ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_InvalidFallbackOwner(inner) => {
                    <LivenessModule2_InvalidFallbackOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_InvalidResponsePeriod(inner) => {
                    <LivenessModule2_InvalidResponsePeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_InvalidVersion(inner) => {
                    <LivenessModule2_InvalidVersion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_ModuleNotConfigured(inner) => {
                    <LivenessModule2_ModuleNotConfigured as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_ModuleNotEnabled(inner) => {
                    <LivenessModule2_ModuleNotEnabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_ModuleStillEnabled(inner) => {
                    <LivenessModule2_ModuleStillEnabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_OwnershipTransferFailed(inner) => {
                    <LivenessModule2_OwnershipTransferFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_ResponsePeriodActive(inner) => {
                    <LivenessModule2_ResponsePeriodActive as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_ResponsePeriodEnded(inner) => {
                    <LivenessModule2_ResponsePeriodEnded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LivenessModule2_UnauthorizedCaller(inner) => {
                    <LivenessModule2_UnauthorizedCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SaferSafes_InsufficientLivenessResponsePeriod(inner) => {
                    <SaferSafes_InsufficientLivenessResponsePeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SemverComp_InvalidSemverParts(inner) => {
                    <SemverComp_InvalidSemverParts as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_GuardNotConfigured(inner) => {
                    <TimelockGuard_GuardNotConfigured as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_GuardNotEnabled(inner) => {
                    <TimelockGuard_GuardNotEnabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_GuardStillEnabled(inner) => {
                    <TimelockGuard_GuardStillEnabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_InvalidTimelockDelay(inner) => {
                    <TimelockGuard_InvalidTimelockDelay as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_InvalidVersion(inner) => {
                    <TimelockGuard_InvalidVersion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_NotOwner(inner) => {
                    <TimelockGuard_NotOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_TransactionAlreadyCancelled(inner) => {
                    <TimelockGuard_TransactionAlreadyCancelled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_TransactionAlreadyExecuted(inner) => {
                    <TimelockGuard_TransactionAlreadyExecuted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_TransactionAlreadyScheduled(inner) => {
                    <TimelockGuard_TransactionAlreadyScheduled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_TransactionNotReady(inner) => {
                    <TimelockGuard_TransactionNotReady as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockGuard_TransactionNotScheduled(inner) => {
                    <TimelockGuard_TransactionNotScheduled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SaferSafes`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SaferSafesEvents {
        #[allow(missing_docs)]
        CancellationThresholdUpdated(CancellationThresholdUpdated),
        #[allow(missing_docs)]
        ChallengeCancelled(ChallengeCancelled),
        #[allow(missing_docs)]
        ChallengeStarted(ChallengeStarted),
        #[allow(missing_docs)]
        ChallengeSucceeded(ChallengeSucceeded),
        #[allow(missing_docs)]
        GuardConfigured(GuardConfigured),
        #[allow(missing_docs)]
        Message(Message),
        #[allow(missing_docs)]
        ModuleCleared(ModuleCleared),
        #[allow(missing_docs)]
        ModuleConfigured(ModuleConfigured),
        #[allow(missing_docs)]
        TransactionCancelled(TransactionCancelled),
        #[allow(missing_docs)]
        TransactionExecuted(TransactionExecuted),
        #[allow(missing_docs)]
        TransactionScheduled(TransactionScheduled),
    }
    impl SaferSafesEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                62u8, 185u8, 36u8, 28u8, 160u8, 103u8, 147u8, 171u8, 103u8, 37u8, 144u8,
                232u8, 88u8, 208u8, 151u8, 114u8, 6u8, 248u8, 36u8, 231u8, 54u8, 120u8,
                6u8, 249u8, 74u8, 144u8, 175u8, 57u8, 29u8, 39u8, 93u8, 51u8,
            ],
            [
                71u8, 226u8, 221u8, 174u8, 62u8, 206u8, 44u8, 119u8, 246u8, 13u8, 142u8,
                158u8, 90u8, 137u8, 165u8, 15u8, 29u8, 19u8, 116u8, 168u8, 127u8, 177u8,
                25u8, 86u8, 210u8, 122u8, 107u8, 73u8, 134u8, 187u8, 161u8, 126u8,
            ],
            [
                78u8, 218u8, 23u8, 151u8, 96u8, 180u8, 230u8, 134u8, 80u8, 5u8, 131u8,
                118u8, 212u8, 172u8, 239u8, 120u8, 233u8, 83u8, 34u8, 19u8, 23u8, 69u8,
                13u8, 41u8, 180u8, 146u8, 14u8, 109u8, 40u8, 54u8, 148u8, 76u8,
            ],
            [
                81u8, 167u8, 246u8, 92u8, 99u8, 37u8, 136u8, 47u8, 35u8, 125u8, 74u8,
                235u8, 67u8, 34u8, 129u8, 121u8, 207u8, 173u8, 72u8, 184u8, 104u8, 81u8,
                29u8, 80u8, 142u8, 36u8, 180u8, 67u8, 122u8, 129u8, 145u8, 55u8,
            ],
            [
                101u8, 63u8, 143u8, 111u8, 206u8, 42u8, 80u8, 59u8, 45u8, 252u8, 163u8,
                78u8, 149u8, 179u8, 228u8, 144u8, 34u8, 84u8, 161u8, 23u8, 101u8, 210u8,
                101u8, 140u8, 14u8, 90u8, 241u8, 214u8, 74u8, 178u8, 118u8, 207u8,
            ],
            [
                156u8, 111u8, 240u8, 46u8, 104u8, 79u8, 138u8, 129u8, 56u8, 158u8, 217u8,
                66u8, 254u8, 161u8, 20u8, 124u8, 22u8, 184u8, 204u8, 95u8, 231u8, 159u8,
                43u8, 251u8, 197u8, 32u8, 212u8, 78u8, 33u8, 74u8, 237u8, 75u8,
            ],
            [
                164u8, 47u8, 216u8, 87u8, 180u8, 125u8, 61u8, 4u8, 245u8, 178u8, 159u8,
                53u8, 203u8, 5u8, 52u8, 63u8, 102u8, 179u8, 23u8, 99u8, 61u8, 45u8,
                194u8, 145u8, 7u8, 38u8, 189u8, 75u8, 202u8, 26u8, 22u8, 37u8,
            ],
            [
                164u8, 141u8, 19u8, 238u8, 143u8, 173u8, 153u8, 116u8, 250u8, 144u8,
                28u8, 251u8, 136u8, 160u8, 45u8, 57u8, 197u8, 54u8, 30u8, 251u8, 171u8,
                19u8, 187u8, 155u8, 58u8, 167u8, 202u8, 163u8, 246u8, 214u8, 183u8, 134u8,
            ],
            [
                173u8, 250u8, 101u8, 17u8, 197u8, 169u8, 181u8, 172u8, 26u8, 220u8, 94u8,
                78u8, 12u8, 166u8, 55u8, 196u8, 101u8, 242u8, 74u8, 181u8, 108u8, 27u8,
                210u8, 85u8, 160u8, 131u8, 224u8, 2u8, 62u8, 59u8, 124u8, 66u8,
            ],
            [
                221u8, 75u8, 155u8, 49u8, 139u8, 152u8, 22u8, 44u8, 177u8, 231u8, 181u8,
                39u8, 82u8, 163u8, 253u8, 17u8, 13u8, 91u8, 121u8, 102u8, 243u8, 181u8,
                8u8, 132u8, 193u8, 205u8, 59u8, 208u8, 64u8, 88u8, 229u8, 199u8,
            ],
            [
                223u8, 222u8, 203u8, 113u8, 235u8, 5u8, 128u8, 201u8, 38u8, 60u8, 134u8,
                123u8, 198u8, 222u8, 157u8, 214u8, 248u8, 89u8, 204u8, 106u8, 110u8,
                227u8, 61u8, 71u8, 229u8, 5u8, 144u8, 79u8, 29u8, 86u8, 1u8, 201u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ChallengeStarted),
            ::core::stringify!(ModuleConfigured),
            ::core::stringify!(CancellationThresholdUpdated),
            ::core::stringify!(Message),
            ::core::stringify!(TransactionScheduled),
            ::core::stringify!(ModuleCleared),
            ::core::stringify!(TransactionCancelled),
            ::core::stringify!(GuardConfigured),
            ::core::stringify!(ChallengeCancelled),
            ::core::stringify!(TransactionExecuted),
            ::core::stringify!(ChallengeSucceeded),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ChallengeStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ModuleConfigured as alloy_sol_types::SolEvent>::SIGNATURE,
            <CancellationThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <Message as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionScheduled as alloy_sol_types::SolEvent>::SIGNATURE,
            <ModuleCleared as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionCancelled as alloy_sol_types::SolEvent>::SIGNATURE,
            <GuardConfigured as alloy_sol_types::SolEvent>::SIGNATURE,
            <ChallengeCancelled as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ChallengeSucceeded as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for SaferSafesEvents {
        const NAME: &'static str = "SaferSafesEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <CancellationThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CancellationThresholdUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CancellationThresholdUpdated)
                }
                Some(
                    <ChallengeCancelled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ChallengeCancelled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChallengeCancelled)
                }
                Some(<ChallengeStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChallengeStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChallengeStarted)
                }
                Some(
                    <ChallengeSucceeded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ChallengeSucceeded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChallengeSucceeded)
                }
                Some(<GuardConfigured as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <GuardConfigured as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GuardConfigured)
                }
                Some(<Message as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Message as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Message)
                }
                Some(<ModuleCleared as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ModuleCleared as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ModuleCleared)
                }
                Some(<ModuleConfigured as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ModuleConfigured as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ModuleConfigured)
                }
                Some(
                    <TransactionCancelled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionCancelled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TransactionCancelled)
                }
                Some(
                    <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TransactionExecuted)
                }
                Some(
                    <TransactionScheduled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionScheduled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TransactionScheduled)
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
    impl alloy_sol_types::private::IntoLogData for SaferSafesEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CancellationThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChallengeCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChallengeStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChallengeSucceeded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GuardConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Message(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ModuleCleared(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ModuleConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CancellationThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChallengeCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChallengeStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChallengeSucceeded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GuardConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Message(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ModuleCleared(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ModuleConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SaferSafes`](self) contract instance.

See the [wrapper's documentation](`SaferSafesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> SaferSafesInstance<P, N> {
        SaferSafesInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<SaferSafesInstance<P, N>>,
    > {
        SaferSafesInstance::<P, N>::deploy(__provider)
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
        SaferSafesInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`SaferSafes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SaferSafes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SaferSafesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SaferSafesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SaferSafesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SaferSafesInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SaferSafes`](self) contract instance.

See the [wrapper's documentation](`SaferSafesInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SaferSafesInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> SaferSafesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SaferSafesInstance<P, N> {
            SaferSafesInstance {
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
    > SaferSafesInstance<P, N> {
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
        ///Creates a new call builder for the [`cancelTransaction`] function.
        pub fn cancelTransaction(
            &self,
            _safe: alloy::sol_types::private::Address,
            _txHash: alloy::sol_types::private::FixedBytes<32>,
            _nonce: alloy::sol_types::private::primitives::aliases::U256,
            _signatures: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, cancelTransactionCall, N> {
            self.call_builder(
                &cancelTransactionCall {
                    _safe,
                    _txHash,
                    _nonce,
                    _signatures,
                },
            )
        }
        ///Creates a new call builder for the [`cancellationThreshold`] function.
        pub fn cancellationThreshold(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, cancellationThresholdCall, N> {
            self.call_builder(&cancellationThresholdCall { _safe })
        }
        ///Creates a new call builder for the [`challenge`] function.
        pub fn challenge(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, challengeCall, N> {
            self.call_builder(&challengeCall { _safe })
        }
        ///Creates a new call builder for the [`challengeStartTime`] function.
        pub fn challengeStartTime(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, challengeStartTimeCall, N> {
            self.call_builder(&challengeStartTimeCall(_0))
        }
        ///Creates a new call builder for the [`changeOwnershipToFallback`] function.
        pub fn changeOwnershipToFallback(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, changeOwnershipToFallbackCall, N> {
            self.call_builder(
                &changeOwnershipToFallbackCall {
                    _safe,
                },
            )
        }
        ///Creates a new call builder for the [`checkAfterExecution`] function.
        pub fn checkAfterExecution(
            &self,
            _txHash: alloy::sol_types::private::FixedBytes<32>,
            _success: bool,
        ) -> alloy_contract::SolCallBuilder<&P, checkAfterExecutionCall, N> {
            self.call_builder(
                &checkAfterExecutionCall {
                    _txHash,
                    _success,
                },
            )
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
            _9: alloy::sol_types::private::Bytes,
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
                    _9,
                    _msgSender,
                },
            )
        }
        ///Creates a new call builder for the [`clearLivenessModule`] function.
        pub fn clearLivenessModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, clearLivenessModuleCall, N> {
            self.call_builder(&clearLivenessModuleCall)
        }
        ///Creates a new call builder for the [`clearTimelockGuard`] function.
        pub fn clearTimelockGuard(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, clearTimelockGuardCall, N> {
            self.call_builder(&clearTimelockGuardCall)
        }
        ///Creates a new call builder for the [`configureLivenessModule`] function.
        pub fn configureLivenessModule(
            &self,
            _config: <LivenessModule2::ModuleConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, configureLivenessModuleCall, N> {
            self.call_builder(
                &configureLivenessModuleCall {
                    _config,
                },
            )
        }
        ///Creates a new call builder for the [`configureTimelockGuard`] function.
        pub fn configureTimelockGuard(
            &self,
            _timelockDelay: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, configureTimelockGuardCall, N> {
            self.call_builder(
                &configureTimelockGuardCall {
                    _timelockDelay,
                },
            )
        }
        ///Creates a new call builder for the [`getChallengePeriodEnd`] function.
        pub fn getChallengePeriodEnd(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getChallengePeriodEndCall, N> {
            self.call_builder(&getChallengePeriodEndCall { _safe })
        }
        ///Creates a new call builder for the [`livenessSafeConfiguration`] function.
        pub fn livenessSafeConfiguration(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, livenessSafeConfigurationCall, N> {
            self.call_builder(
                &livenessSafeConfigurationCall {
                    _safe,
                },
            )
        }
        ///Creates a new call builder for the [`maxCancellationThreshold`] function.
        pub fn maxCancellationThreshold(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, maxCancellationThresholdCall, N> {
            self.call_builder(
                &maxCancellationThresholdCall {
                    _safe,
                },
            )
        }
        ///Creates a new call builder for the [`pendingTransactions`] function.
        pub fn pendingTransactions(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, pendingTransactionsCall, N> {
            self.call_builder(&pendingTransactionsCall { _safe })
        }
        ///Creates a new call builder for the [`respond`] function.
        pub fn respond(&self) -> alloy_contract::SolCallBuilder<&P, respondCall, N> {
            self.call_builder(&respondCall)
        }
        ///Creates a new call builder for the [`scheduleTransaction`] function.
        pub fn scheduleTransaction(
            &self,
            _safe: alloy::sol_types::private::Address,
            _nonce: alloy::sol_types::private::primitives::aliases::U256,
            _params: <TimelockGuard::ExecTransactionParams as alloy::sol_types::SolType>::RustType,
            _signatures: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, scheduleTransactionCall, N> {
            self.call_builder(
                &scheduleTransactionCall {
                    _safe,
                    _nonce,
                    _params,
                    _signatures,
                },
            )
        }
        ///Creates a new call builder for the [`scheduledTransaction`] function.
        pub fn scheduledTransaction(
            &self,
            _safe: alloy::sol_types::private::Address,
            _txHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, scheduledTransactionCall, N> {
            self.call_builder(
                &scheduledTransactionCall {
                    _safe,
                    _txHash,
                },
            )
        }
        ///Creates a new call builder for the [`signCancellation`] function.
        pub fn signCancellation(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, signCancellationCall, N> {
            self.call_builder(&signCancellationCall(_0))
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
        ///Creates a new call builder for the [`timelockDelay`] function.
        pub fn timelockDelay(
            &self,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, timelockDelayCall, N> {
            self.call_builder(&timelockDelayCall { _safe })
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
    > SaferSafesInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`CancellationThresholdUpdated`] event.
        pub fn CancellationThresholdUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, CancellationThresholdUpdated, N> {
            self.event_filter::<CancellationThresholdUpdated>()
        }
        ///Creates a new event filter for the [`ChallengeCancelled`] event.
        pub fn ChallengeCancelled_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChallengeCancelled, N> {
            self.event_filter::<ChallengeCancelled>()
        }
        ///Creates a new event filter for the [`ChallengeStarted`] event.
        pub fn ChallengeStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChallengeStarted, N> {
            self.event_filter::<ChallengeStarted>()
        }
        ///Creates a new event filter for the [`ChallengeSucceeded`] event.
        pub fn ChallengeSucceeded_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChallengeSucceeded, N> {
            self.event_filter::<ChallengeSucceeded>()
        }
        ///Creates a new event filter for the [`GuardConfigured`] event.
        pub fn GuardConfigured_filter(
            &self,
        ) -> alloy_contract::Event<&P, GuardConfigured, N> {
            self.event_filter::<GuardConfigured>()
        }
        ///Creates a new event filter for the [`Message`] event.
        pub fn Message_filter(&self) -> alloy_contract::Event<&P, Message, N> {
            self.event_filter::<Message>()
        }
        ///Creates a new event filter for the [`ModuleCleared`] event.
        pub fn ModuleCleared_filter(
            &self,
        ) -> alloy_contract::Event<&P, ModuleCleared, N> {
            self.event_filter::<ModuleCleared>()
        }
        ///Creates a new event filter for the [`ModuleConfigured`] event.
        pub fn ModuleConfigured_filter(
            &self,
        ) -> alloy_contract::Event<&P, ModuleConfigured, N> {
            self.event_filter::<ModuleConfigured>()
        }
        ///Creates a new event filter for the [`TransactionCancelled`] event.
        pub fn TransactionCancelled_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionCancelled, N> {
            self.event_filter::<TransactionCancelled>()
        }
        ///Creates a new event filter for the [`TransactionExecuted`] event.
        pub fn TransactionExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionExecuted, N> {
            self.event_filter::<TransactionExecuted>()
        }
        ///Creates a new event filter for the [`TransactionScheduled`] event.
        pub fn TransactionScheduled_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionScheduled, N> {
            self.event_filter::<TransactionScheduled>()
        }
    }
}

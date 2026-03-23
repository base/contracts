///Module containing a contract's types and functions.
/**

```solidity
library Types {
    struct OutputRootProof { bytes32 version; bytes32 stateRoot; bytes32 messagePasserStorageRoot; bytes32 latestBlockhash; }
    struct WithdrawalTransaction { uint256 nonce; address sender; address target; uint256 value; uint256 gasLimit; bytes data; }
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct WithdrawalTransaction { uint256 nonce; address sender; address target; uint256 value; uint256 gasLimit; bytes data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalTransaction {
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasLimit: alloy::sol_types::private::primitives::aliases::U256,
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
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<WithdrawalTransaction> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalTransaction) -> Self {
                (
                    value.nonce,
                    value.sender,
                    value.target,
                    value.value,
                    value.gasLimit,
                    value.data,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalTransaction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonce: tuple.0,
                    sender: tuple.1,
                    target: tuple.2,
                    value: tuple.3,
                    gasLimit: tuple.4,
                    data: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for WithdrawalTransaction {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for WithdrawalTransaction {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasLimit),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
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
        impl alloy_sol_types::SolType for WithdrawalTransaction {
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
        impl alloy_sol_types::SolStruct for WithdrawalTransaction {
            const NAME: &'static str = "WithdrawalTransaction";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "WithdrawalTransaction(uint256 nonce,address sender,address target,uint256 value,uint256 gasLimit,bytes data)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sender,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.target,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.value)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gasLimit)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for WithdrawalTransaction {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sender,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.target,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.value)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gasLimit,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
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
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sender,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.target,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.value,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
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
    struct WithdrawalTransaction {
        uint256 nonce;
        address sender;
        address target;
        uint256 value;
        uint256 gasLimit;
        bytes data;
    }
}

interface OptimismPortal2 {
    type GameType is uint32;

    error ContentLengthMismatch();
    error EmptyItem();
    error InvalidDataRemainder();
    error InvalidHeader();
    error OptimismPortal_AlreadyFinalized();
    error OptimismPortal_BadTarget();
    error OptimismPortal_CallPaused();
    error OptimismPortal_CalldataTooLarge();
    error OptimismPortal_GasEstimation();
    error OptimismPortal_GasLimitTooLow();
    error OptimismPortal_ImproperDisputeGame();
    error OptimismPortal_InvalidDisputeGame();
    error OptimismPortal_InvalidLockboxState();
    error OptimismPortal_InvalidMerkleProof();
    error OptimismPortal_InvalidOutputRootProof();
    error OptimismPortal_InvalidProofTimestamp();
    error OptimismPortal_InvalidRootClaim();
    error OptimismPortal_NoReentrancy();
    error OptimismPortal_NotAllowedOnCGTMode();
    error OptimismPortal_ProofNotOldEnough();
    error OptimismPortal_Unproven();
    error OutOfGas();
    error ProxyAdminOwnedBase_NotProxyAdmin();
    error ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner();
    error ProxyAdminOwnedBase_NotProxyAdminOwner();
    error ProxyAdminOwnedBase_NotResolvedDelegateProxy();
    error ProxyAdminOwnedBase_NotSharedProxyAdminOwner();
    error ProxyAdminOwnedBase_ProxyAdminNotFound();
    error ReinitializableBase_ZeroInitVersion();
    error UnexpectedList();
    error UnexpectedString();

    event Initialized(uint8 version);
    event TransactionDeposited(address indexed from, address indexed to, uint256 indexed version, bytes opaqueData);
    event WithdrawalFinalized(bytes32 indexed withdrawalHash, bool success);
    event WithdrawalProven(bytes32 indexed withdrawalHash, address indexed from, address indexed to);
    event WithdrawalProvenExtension1(bytes32 indexed withdrawalHash, address indexed proofSubmitter);

    constructor(uint256 _proofMaturityDelaySeconds);

    receive() external payable;

    function anchorStateRegistry() external view returns (address);
    function checkWithdrawal(bytes32 _withdrawalHash, address _proofSubmitter) external view;
    function depositTransaction(address _to, uint256 _value, uint64 _gasLimit, bool _isCreation, bytes memory _data) external payable;
    function disputeGameBlacklist(address _disputeGame) external view returns (bool);
    function disputeGameFactory() external view returns (address);
    function disputeGameFinalityDelaySeconds() external view returns (uint256);
    function donateETH() external payable;
    function ethLockbox() external view returns (address);
    function finalizeWithdrawalTransaction(Types.WithdrawalTransaction memory _tx) external;
    function finalizeWithdrawalTransactionExternalProof(Types.WithdrawalTransaction memory _tx, address _proofSubmitter) external;
    function finalizedWithdrawals(bytes32) external view returns (bool);
    function guardian() external view returns (address);
    function initVersion() external view returns (uint8);
    function initialize(address _systemConfig, address _anchorStateRegistry) external;
    function l2Sender() external view returns (address);
    function minimumGasLimit(uint64 _byteCount) external pure returns (uint64);
    function numProofSubmitters(bytes32 _withdrawalHash) external view returns (uint256);
    function params() external view returns (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum);
    function paused() external view returns (bool);
    function proofMaturityDelaySeconds() external view returns (uint256);
    function proofSubmitters(bytes32, uint256) external view returns (address);
    function proveWithdrawalTransaction(Types.WithdrawalTransaction memory _tx, uint256 _disputeGameIndex, Types.OutputRootProof memory _outputRootProof, bytes[] memory _withdrawalProof) external;
    function provenWithdrawals(bytes32, address) external view returns (address disputeGameProxy, uint64 timestamp);
    function proxyAdmin() external view returns (address);
    function proxyAdminOwner() external view returns (address);
    function respectedGameType() external view returns (GameType);
    function respectedGameTypeUpdatedAt() external view returns (uint64);
    function superchainConfig() external view returns (address);
    function systemConfig() external view returns (address);
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
        "name": "_proofMaturityDelaySeconds",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "anchorStateRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAnchorStateRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkWithdrawal",
    "inputs": [
      {
        "name": "_withdrawalHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_proofSubmitter",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "depositTransaction",
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
        "name": "_gasLimit",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_isCreation",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "_data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "disputeGameBlacklist",
    "inputs": [
      {
        "name": "_disputeGame",
        "type": "address",
        "internalType": "contract IDisputeGame"
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
    "name": "disputeGameFactory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDisputeGameFactory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "disputeGameFinalityDelaySeconds",
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
    "name": "donateETH",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "ethLockbox",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IETHLockbox"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "finalizeWithdrawalTransaction",
    "inputs": [
      {
        "name": "_tx",
        "type": "tuple",
        "internalType": "struct Types.WithdrawalTransaction",
        "components": [
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "target",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "value",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "finalizeWithdrawalTransactionExternalProof",
    "inputs": [
      {
        "name": "_tx",
        "type": "tuple",
        "internalType": "struct Types.WithdrawalTransaction",
        "components": [
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "target",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "value",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "_proofSubmitter",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "finalizedWithdrawals",
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
        "name": "_systemConfig",
        "type": "address",
        "internalType": "contract ISystemConfig"
      },
      {
        "name": "_anchorStateRegistry",
        "type": "address",
        "internalType": "contract IAnchorStateRegistry"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "l2Sender",
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
    "name": "minimumGasLimit",
    "inputs": [
      {
        "name": "_byteCount",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
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
    "name": "numProofSubmitters",
    "inputs": [
      {
        "name": "_withdrawalHash",
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
    "name": "params",
    "inputs": [],
    "outputs": [
      {
        "name": "prevBaseFee",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "prevBoughtGas",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "prevBlockNum",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "proofMaturityDelaySeconds",
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
    "name": "proofSubmitters",
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
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proveWithdrawalTransaction",
    "inputs": [
      {
        "name": "_tx",
        "type": "tuple",
        "internalType": "struct Types.WithdrawalTransaction",
        "components": [
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "target",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "value",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "_disputeGameIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
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
        "name": "_withdrawalProof",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "provenWithdrawals",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "disputeGameProxy",
        "type": "address",
        "internalType": "contract IDisputeGame"
      },
      {
        "name": "timestamp",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "respectedGameType",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "GameType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "respectedGameTypeUpdatedAt",
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
    "name": "systemConfig",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISystemConfig"
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
    "name": "TransactionDeposited",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "version",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "opaqueData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalFinalized",
    "inputs": [
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "success",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalProven",
    "inputs": [
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalProvenExtension1",
    "inputs": [
      {
        "name": "withdrawalHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "proofSubmitter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "ContentLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyItem",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDataRemainder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHeader",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_AlreadyFinalized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_BadTarget",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_CallPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_CalldataTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_GasEstimation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_GasLimitTooLow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_ImproperDisputeGame",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidDisputeGame",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidLockboxState",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidMerkleProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidOutputRootProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidProofTimestamp",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidRootClaim",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_NoReentrancy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_NotAllowedOnCGTMode",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_ProofNotOldEnough",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_Unproven",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfGas",
    "inputs": []
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
    "name": "UnexpectedList",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnexpectedString",
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
pub mod OptimismPortal2 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c06040523480156200001157600080fd5b506040516200516938038062005169833981016040819052620000349162000111565b600360805260a0819052620000486200004f565b506200012b565b600054610100900460ff1615620000bc5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156200010f576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6000602082840312156200012457600080fd5b5051919050565b60805160a05161500a6200015f600039600081816105fc0152611bce015260008181610263015261109b015261500a6000f3fe6080604052600436106101d15760003560e01c806371c1566e116100f7578063a3860f4811610095578063cff0ab9611610064578063cff0ab9614610620578063dad544e0146106c1578063e9e05c42146106d6578063f2b4e617146106e957600080fd5b8063a3860f4814610520578063b682c44414610540578063bb2c727e14610560578063bf653a5c146105ed57600080fd5b8063952b2797116100d1578063952b27971461049b5780639bf62d82146104b0578063a14238e7146104d0578063a35d99df1461050057600080fd5b806371c1566e1461045b5780638b4c40b0146101f65780638c3152e91461047b57600080fd5b806345884d321161016f578063513747ab1161013e578063513747ab1461039f57806354fd4d50146103da5780635c0cba33146104265780635c975abb1461044657600080fd5b806345884d3214610301578063485cc955146103315780634870496f146103515780634fd0434c1461037157600080fd5b80633c9f397c116101ab5780633c9f397c1461028d5780633e47158c146102b757806343ca1c50146102cc578063452a9320146102ec57600080fd5b806333d7e2bd146101fd57806335e80ab31461023a57806338d38c971461024f57600080fd5b366101f8576101f63334620186a06000604051806020016040528060008152506106fe565b005b600080fd5b34801561020957600080fd5b5060375461021d906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561024657600080fd5b5061021d61095b565b34801561025b57600080fd5b5060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152602001610231565b34801561029957600080fd5b506102a26109e7565b60405163ffffffff9091168152602001610231565b3480156102c357600080fd5b5061021d610a6e565b3480156102d857600080fd5b506101f66102e736600461457d565b610c52565b3480156102f857600080fd5b5061021d610fa8565b34801561030d57600080fd5b5061032161031c3660046145cf565b61100b565b6040519015158152602001610231565b34801561033d57600080fd5b506101f661034c3660046145ec565b611099565b34801561035d57600080fd5b506101f661036c36600461461a565b611281565b34801561037d57600080fd5b50610386611943565b60405167ffffffffffffffff9091168152602001610231565b3480156103ab57600080fd5b506103cc6103ba3660046146f6565b6000908152603c602052604090205490565b604051908152602001610231565b3480156103e657600080fd5b50604080518082018252600581527f352e322e30000000000000000000000000000000000000000000000000000000602082015290516102319190614767565b34801561043257600080fd5b50603e5461021d906001600160a01b031681565b34801561045257600080fd5b506103216119ca565b34801561046757600080fd5b506101f661047636600461477a565b611a51565b34801561048757600080fd5b506101f661049636600461479f565b611cfd565b3480156104a757600080fd5b506103cc611d0a565b3480156104bc57600080fd5b5060325461021d906001600160a01b031681565b3480156104dc57600080fd5b506103216104eb3660046146f6565b60336020526000908152604090205460ff1681565b34801561050c57600080fd5b5061038661051b3660046147f2565b611d91565b34801561052c57600080fd5b5061021d61053b36600461480f565b611daa565b34801561054c57600080fd5b50603f5461021d906001600160a01b031681565b34801561056c57600080fd5b506105c561057b36600461477a565b60396020908152600092835260408084209091529082529020546001600160a01b0381169074010000000000000000000000000000000000000000900467ffffffffffffffff1682565b604080516001600160a01b03909316835267ffffffffffffffff909116602083015201610231565b3480156105f957600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103cc565b34801561062c57600080fd5b50600154610688906fffffffffffffffffffffffffffffffff81169067ffffffffffffffff7001000000000000000000000000000000008204811691780100000000000000000000000000000000000000000000000090041683565b604080516fffffffffffffffffffffffffffffffff909416845267ffffffffffffffff9283166020850152911690820152606001610231565b3480156106cd57600080fd5b5061021d611de2565b6101f66106e436600461483f565b6106fe565b3480156106f557600080fd5b5061021d611e29565b8260005a905061070c611e8c565b15610749573415610749576040517fbd58e0a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610751611f0e565b156107c65734156107c657603f60009054906101000a90046001600160a01b03166001600160a01b0316631ee116bf346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156107ac57600080fd5b505af11580156107c0573d6000803e3d6000fd5b50505050505b8380156107db57506001600160a01b03871615155b15610812576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61081c8351611d91565b67ffffffffffffffff168567ffffffffffffffff161015610869576040517f70c8bdbd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6201d4c0835111156108a7576040517f5aa3bac900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336108b0611fcd565b6108cd575033731111000000000000000000000000000000001111015b600034888888886040516020016108e89594939291906148be565b60405160208183030381529060405290506000896001600160a01b0316836001600160a01b03167fb3813568d9991fc951961fcb4c784893574240a28925604d09fc577c55bb7c328460405161093e9190614767565b60405180910390a45050610952828261200a565b50505050505050565b603754604080517f35e80ab300000000000000000000000000000000000000000000000000000000815290516000926001600160a01b0316916335e80ab39160048083019260209291908290030181865afa1580156109be573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614923565b905090565b603e54604080517f3c9f397c00000000000000000000000000000000000000000000000000000000815290516000926001600160a01b031691633c9f397c9160048083019260209291908290030181865afa158015610a4a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614952565b600080610a997fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035490565b90506001600160a01b03811615610aaf57919050565b6040518060400160405280601a81526020017f4f564d5f4c3143726f7373446f6d61696e4d657373656e676572000000000000815250516002610af2919061499e565b604080513060208201526000918101919091527f4f564d5f4c3143726f7373446f6d61696e4d657373656e6765720000000000009190911790610b4d906060015b604051602081830303815290604052805190602001205490565b14610b84576040517f54e433cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408051306020820152600191810191909152600090610ba690606001610b33565b90506001600160a01b03811615610c2057806001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bf5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c199190614923565b9250505090565b6040517f332144db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610c5a6122e1565b610c62611e8c565b15610ca357606082015115610ca3576040517fbd58e0a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6032546001600160a01b031661dead14610ce9576040517fdfeaaeb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610cf68260400151612320565b15610d2d576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610d3883612349565b9050610d448183611a51565b600081815260336020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055610d83611f0e565b15610e1557606083015115610e1557603f5460608401516040517f8d445bd00000000000000000000000000000000000000000000000000000000081526001600160a01b0390921691638d445bd091610de29160040190815260200190565b600060405180830381600087803b158015610dfc57600080fd5b505af1158015610e10573d6000803e3d6000fd5b505050505b8260200151603260006101000a8154816001600160a01b0302191690836001600160a01b031602179055506000610e5e8460400151856080015186606001518760a00151612396565b603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead17905560405190915082907fdb5c7652857aa163daadd670e116628fb42e869d8ac4251ef8971d9e5727df1b90610ec390841515815260200190565b60405180910390a2610ed3611f0e565b15610f5d5780158015610eea575060008460600151115b15610f5d57603f60009054906101000a90046001600160a01b03166001600160a01b0316631ee116bf85606001516040518263ffffffff1660e01b81526004016000604051808303818588803b158015610f4357600080fd5b505af1158015610f57573d6000803e3d6000fd5b50505050505b80158015610f6b5750326001145b15610fa2576040517fab58103600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505050565b603754604080517f452a932000000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163452a93209160048083019260209291908290030181865afa1580156109be573d6000803e3d6000fd5b603e546040517f45884d320000000000000000000000000000000000000000000000000000000081526001600160a01b03838116600483015260009216906345884d3290602401602060405180830381865afa15801561106f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061109391906149bd565b92915050565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff161580156110d9575060005460ff8083169116105b6111505760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a656400000000000000000000000000000000000060648201526084015b60405180910390fd5b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff8316176101001790556111896123f4565b603780546001600160a01b038086167fffffffffffffffffffffffff000000000000000000000000000000000000000092831617909255603e8054928516929091169190911790556111d961245b565b6032546001600160a01b031661121657603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead1790555b61121e61260e565b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b6112896122e1565b6112968560400151612320565b156112cd576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6112d5611e8c565b1561131657606085015115611316576040517fbd58e0a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611320611e29565b6001600160a01b031663bb8aa1fc866040518263ffffffff1660e01b815260040161134d91815260200190565b606060405180830381865afa15801561136a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061138e91906149da565b603e546040517f496b9c160000000000000000000000000000000000000000000000000000000081526001600160a01b0380841660048301529295509116925063496b9c169150602401602060405180830381865afa1580156113f5573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061141991906149bd565b61144f576040517ff395240e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f04e50fed0000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152909116906304e50fed90602401602060405180830381865afa1580156114b2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114d691906149bd565b61150c576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001816001600160a01b031663200d2ed26040518163ffffffff1660e01b8152600401602060405180830381865afa15801561154c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115709190614a56565b600281111561158157611581614a27565b036115b8576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61162a816001600160a01b031663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115f9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061161d9190614a77565b67ffffffffffffffff1690565b67ffffffffffffffff16421161166c576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61168361167e36869003860186614a94565b612707565b6116eb826001600160a01b031663bcef3b556040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116e89190614afa565b90565b14611722576040517f426149af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600061172d87612349565b9050600081600060405160200161174e929190918252602082015260400190565b60408051601f19818403018152828252805160209182012090830181905292506117c5910160408051601f19818403018152828201909152600182527f01000000000000000000000000000000000000000000000000000000000000006020830152906117bb8789614b13565b8960400135612746565b1515600003611800576040517f2e57ef3a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820182526001600160a01b03808616825267ffffffffffffffff42811660208085019182526000888152603982528681203380835290835287822096518754945190951674010000000000000000000000000000000000000000027fffffffff000000000000000000000000000000000000000000000000000000009094169486169490941792909217909455868152603c845284812080546001810182559082528482200180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169092179091558b840151928c01519351928216939091169185917f67a6208cfcc0801d50f6cbe764733f4fddf66ac0b04442061a8a8c0cb6b63f6291a4604051339083907f798f9f13695f8f045aa5f80ed8efebb695f3c7fe65da381969f2f28bf3c60b9790600090a35050505050505050565b603e54604080517f4086d18300000000000000000000000000000000000000000000000000000000815290516000926001600160a01b031691634086d1839160048083019260209291908290030181865afa1580156119a6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614a77565b603754604080517f5c975abb00000000000000000000000000000000000000000000000000000000815290516000926001600160a01b031691635c975abb9160048083019260209291908290030181865afa158015611a2d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e291906149bd565b60008281526039602090815260408083206001600160a01b0385811685529083528184208251808401845290549182168082527401000000000000000000000000000000000000000090920467ffffffffffffffff1681850152868552603390935292205490919060ff1615611af3576040517f730a107400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b816020015167ffffffffffffffff16600003611b3b576040517fcca6afda00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611b7c816001600160a01b031663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115f9573d6000803e3d6000fd5b67ffffffffffffffff16826020015167ffffffffffffffff1611611bcc576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000826020015167ffffffffffffffff1642611c079190614b97565b11611c3e576040517fd9bc01be00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f6c4f44670000000000000000000000000000000000000000000000000000000081526001600160a01b03838116600483015290911690636c4f446790602401602060405180830381865afa158015611ca1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611cc591906149bd565b610fa2576040517f332a57f800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b611d078133610c52565b50565b603e54604080517f952b279700000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163952b27979160048083019260209291908290030181865afa158015611d6d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614afa565b6000611d9e826028614bae565b61109390615208614bde565b603c6020528160005260406000208181548110611dc657600080fd5b6000918252602090912001546001600160a01b03169150829050565b6000611dec610a6e565b6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109be573d6000803e3d6000fd5b603e54604080517ff2b4e61700000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163f2b4e6179160048083019260209291908290030181865afa1580156109be573d6000803e3d6000fd5b6037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f435553544f4d5f4741535f544f4b454e0000000000000000000000000000000060048201526000916001600160a01b0316906347af267b90602401602060405180830381865afa158015611a2d573d6000803e3d6000fd5b6037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f4554485f4c4f434b424f5800000000000000000000000000000000000000000060048201526000916001600160a01b0316906347af267b90602401602060405180830381865afa158015611f90573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611fb491906149bd565b80156109e2575050603f546001600160a01b0316151590565b6000323303611fdc5750600190565b333b60170361200457604051602081016040526020600082333c5160e81c62ef010014905090565b50600090565b600154600090612040907801000000000000000000000000000000000000000000000000900467ffffffffffffffff1643614b97565b9050600061204c61276a565b90506000816020015160ff16826000015163ffffffff1661206d9190614c39565b905082156121a4576001546000906120a4908390700100000000000000000000000000000000900467ffffffffffffffff16614c83565b90506000836040015160ff16836120bb9190614cf7565b6001546120db9084906fffffffffffffffffffffffffffffffff16614cf7565b6120e59190614c39565b6001549091506000906121369061210f9084906fffffffffffffffffffffffffffffffff16614db3565b866060015163ffffffff168760a001516fffffffffffffffffffffffffffffffff16612823565b905060018611156121655761216261210f82876040015160ff1660018a61215d9190614b97565b612842565b90505b6fffffffffffffffffffffffffffffffff16780100000000000000000000000000000000000000000000000067ffffffffffffffff4316021760015550505b600180548691906010906121d7908490700100000000000000000000000000000000900467ffffffffffffffff16614bde565b92506101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550816000015163ffffffff16600160000160109054906101000a900467ffffffffffffffff1667ffffffffffffffff161315612264576040517f77ebef4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600154600090612290906fffffffffffffffffffffffffffffffff1667ffffffffffffffff881661499e565b905060006122a248633b9aca00612897565b6122ac9083614e27565b905060005a6122bb9088614b97565b9050808211156122d7576122d76122d28284614b97565b6128ae565b5050505050505050565b6122e96119ca565b15611cfb576040517fb9c3c2ef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001600160a01b038216301480611093575050603f546001600160a01b0390811691161490565b80516020808301516040808501516060860151608087015160a08801519351600097612379979096959101614e3b565b604051602081830303815290604052805190602001209050919050565b60008060006123a68660006128dc565b9050806123dc576308c379a06000526020805278185361666543616c6c3a204e6f7420656e6f756768206761736058526064601cfd5b600080855160208701888b5af1979650505050505050565b336123fd610a6e565b6001600160a01b031614158015612424575033612418611de2565b6001600160a01b031614155b15611cfb576040517fc4050a2600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f4554485f4c4f434b424f5800000000000000000000000000000000000000000060048201526001600160a01b03909116906347af267b90602401602060405180830381865afa1580156124dc573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061250091906149bd565b80156125155750603f546001600160a01b0316155b806125d757506037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f4554485f4c4f434b424f5800000000000000000000000000000000000000000060048201526001600160a01b03909116906347af267b90602401602060405180830381865afa15801561259c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125c091906149bd565b1580156125d75750603f546001600160a01b031615155b15611cfb576040517f9c46cd7900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600054610100900460ff1661268b5760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401611147565b6001547801000000000000000000000000000000000000000000000000900467ffffffffffffffff16600003611cfb5760408051606081018252633b9aca00808252600060208301524367ffffffffffffffff169190920181905278010000000000000000000000000000000000000000000000000217600155565b60008160000151826020015183604001518460600151604051602001612379949392919093845260208401929092526040830152606082015260800190565b600080612752866128fa565b90506127608186868661292c565b9695505050505050565b6040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a0810191909152603754604080517fcc731b0200000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163cc731b029160048083019260c09291908290030181865afa1580156127ff573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110939190614e9b565b6000612838612832858561295c565b8361296c565b90505b9392505050565b6000670de0b6b3a764000061288361285a8583614c39565b61286c90670de0b6b3a7640000614c83565b61287e85670de0b6b3a7640000614cf7565b61297b565b61288d9086614cf7565b6128389190614c39565b6000818310156128a7578161283b565b5090919050565b6000805a90505b825a6128c19083614b97565b10156128d7576128d082614f57565b91506128b5565b505050565b600080603f83619c4001026040850201603f5a021015949350505050565b6060818051906020012060405160200161291691815260200190565b6040516020818303038152906040529050919050565b60006129538461293d8786866129ac565b8051602091820120825192909101919091201490565b95945050505050565b6000818312156128a7578161283b565b60008183126128a7578161283b565b600061283b670de0b6b3a764000083612993866132be565b61299d9190614cf7565b6129a79190614c39565b6134e8565b606060008451116129ff5760405162461bcd60e51b815260206004820152601560248201527f4d65726b6c65547269653a20656d707479206b657900000000000000000000006044820152606401611147565b6000612a0a8461370d565b90506000612a17866137f9565b9050600084604051602001612a2e91815260200190565b60405160208183030381529060405290506000805b845181101561324f576000858281518110612a6057612a60614f71565b602002602001015190508451831115612ae15760405162461bcd60e51b815260206004820152602e60248201527f4d65726b6c65547269653a206b657920696e646578206578636565647320746f60448201527f74616c206b6579206c656e6774680000000000000000000000000000000000006064820152608401611147565b82600003612b805780518051602091820120604051612b2f92612b0992910190815260200190565b604051602081830303815290604052858051602091820120825192909101919091201490565b612b7b5760405162461bcd60e51b815260206004820152601d60248201527f4d65726b6c65547269653a20696e76616c696420726f6f7420686173680000006044820152606401611147565b612ca3565b805151602011612c1c5780518051602091820120604051612baa92612b0992910190815260200190565b612b7b5760405162461bcd60e51b815260206004820152602760248201527f4d65726b6c65547269653a20696e76616c6964206c6172676520696e7465726e60448201527f616c2068617368000000000000000000000000000000000000000000000000006064820152608401611147565b805184516020808701919091208251919092012014612ca35760405162461bcd60e51b815260206004820152602660248201527f4d65726b6c65547269653a20696e76616c696420696e7465726e616c206e6f6460448201527f65206861736800000000000000000000000000000000000000000000000000006064820152608401611147565b612caf60106001614fa0565b81602001515103612e575784518303612def57612ce98160200151601081518110612cdc57612cdc614f71565b602002602001015161385c565b96506000875111612d625760405162461bcd60e51b815260206004820152603b60248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286272616e63682900000000006064820152608401611147565b60018651612d709190614b97565b8214612de45760405162461bcd60e51b815260206004820152603a60248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286272616e6368290000000000006064820152608401611147565b50505050505061283b565b6000858481518110612e0357612e03614f71565b602001015160f81c60f81b60f81c9050600082602001518260ff1681518110612e2e57612e2e614f71565b60200260200101519050612e4181613910565b9550612e4e600186614fa0565b9450505061323c565b6002816020015151036131ce576000612e6f82613935565b9050600081600081518110612e8657612e86614f71565b016020015160f81c90506000612e9d600283614fb8565b612ea8906002614fda565b90506000612eb9848360ff16613959565b90506000612ec78a89613959565b90506000612ed5838361398f565b905080835114612f4d5760405162461bcd60e51b815260206004820152603a60248201527f4d65726b6c65547269653a20706174682072656d61696e646572206d7573742060448201527f736861726520616c6c206e6962626c65732077697468206b65790000000000006064820152608401611147565b60ff851660021480612f62575060ff85166003145b156131035780825114612fdd5760405162461bcd60e51b815260206004820152603d60248201527f4d65726b6c65547269653a206b65792072656d61696e646572206d757374206260448201527f65206964656e746963616c20746f20706174682072656d61696e6465720000006064820152608401611147565b612ff78760200151600181518110612cdc57612cdc614f71565b9c5060008d51116130705760405162461bcd60e51b815260206004820152603960248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286c65616629000000000000006064820152608401611147565b60018c5161307e9190614b97565b88146130f25760405162461bcd60e51b815260206004820152603860248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286c6561662900000000000000006064820152608401611147565b50505050505050505050505061283b565b60ff85161580613116575060ff85166001145b1561315557613142876020015160018151811061313557613135614f71565b6020026020010151613910565b995061314e818a614fa0565b98506131c3565b60405162461bcd60e51b815260206004820152603260248201527f4d65726b6c65547269653a2072656365697665642061206e6f6465207769746860448201527f20616e20756e6b6e6f776e2070726566697800000000000000000000000000006064820152608401611147565b50505050505061323c565b60405162461bcd60e51b815260206004820152602860248201527f4d65726b6c65547269653a20726563656976656420616e20756e70617273656160448201527f626c65206e6f64650000000000000000000000000000000000000000000000006064820152608401611147565b508061324781614f57565b915050612a43565b5060405162461bcd60e51b815260206004820152602560248201527f4d65726b6c65547269653a2072616e206f7574206f662070726f6f6620656c6560448201527f6d656e74730000000000000000000000000000000000000000000000000000006064820152608401611147565b600080821361330f5760405162461bcd60e51b815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611147565b6000606061331c84613a43565b03609f8181039490941b90931c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b393909302929092017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdb731c958f34d94c1821361351957506000919050565b680755bf798b4a1bf1e582126135715760405162461bcd60e51b815260206004820152600c60248201527f4558505f4f564552464c4f5700000000000000000000000000000000000000006044820152606401611147565b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b80516060908067ffffffffffffffff81111561372b5761372b6143e9565b60405190808252806020026020018201604052801561377057816020015b60408051808201909152606080825260208201528152602001906001900390816137495790505b50915060005b818110156137f257604051806040016040528085838151811061379b5761379b614f71565b602002602001015181526020016137ca8684815181106137bd576137bd614f71565b6020026020010151613aff565b8152508382815181106137df576137df614f71565b6020908102919091010152600101613776565b5050919050565b606080604051905082518060011b603f8101601f1916830160405280835250602084016020830160005b83811015613851578060011b82018184015160001a8060041c8253600f811660018301535050600101613823565b509295945050505050565b6060600080600061386c85613b12565b91945092509050600081600181111561388757613887614a27565b146138be576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6138c88284614fa0565b855114613901576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61295385602001518484613fb0565b6060602082600001511061392c576139278261385c565b611093565b61109382614044565b60606110936139548360200151600081518110612cdc57612cdc614f71565b6137f9565b6060825182106139785750604080516020810190915260008152611093565b61283b838384865161398a9190614b97565b61405a565b60008082518451106139a25782516139a5565b83515b90505b8082108015613a2c57508282815181106139c4576139c4614f71565b602001015160f81c60f81b7effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916848381518110613a0357613a03614f71565b01602001517fff0000000000000000000000000000000000000000000000000000000000000016145b15613a3c578160010191506139a8565b5092915050565b6000808211613a945760405162461bcd60e51b815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611147565b5060016fffffffffffffffffffffffffffffffff821160071b82811c67ffffffffffffffff1060061b1782811c63ffffffff1060051b1782811c61ffff1060041b1782811c60ff10600390811b90911783811c600f1060021b1783811c909110821b1791821c111790565b6060611093613b0d836141c6565b614233565b60008060008360000151600003613b55576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f8111613b7a576000600160009450945094505050613fa9565b60b78111613c90576000613b8f608083614b97565b905080876000015111613bce576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff00000000000000000000000000000000000000000000000000000000000000169082148015613c4657507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b15613c7d576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5060019550935060009250613fa9915050565b60bf8111613dee576000613ca560b783614b97565b905080876000015111613ce4576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003613d46576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111613d8e576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613d988184614fa0565b895111613dd1576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613ddc836001614fa0565b9750955060009450613fa99350505050565b60f78111613e53576000613e0360c083614b97565b905080876000015111613e42576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600195509350849250613fa9915050565b6000613e6060f783614b97565b905080876000015111613e9f576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003613f01576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111613f49576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613f538184614fa0565b895111613f8c576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613f97836001614fa0565b9750955060019450613fa99350505050565b9193909250565b60608167ffffffffffffffff811115613fcb57613fcb6143e9565b6040519080825280601f01601f191660200182016040528015613ff5576020820181803683370190505b509050811561283b57600061400a8486614fa0565b90506020820160005b8481101561402b578281015182820152602001614013565b8481111561403a576000858301525b5050509392505050565b6060611093826020015160008460000151613fb0565b60608182601f0110156140af5760405162461bcd60e51b815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611147565b8282840110156141015760405162461bcd60e51b815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611147565b818301845110156141545760405162461bcd60e51b815260206004820152601160248201527f736c6963655f6f75744f66426f756e64730000000000000000000000000000006044820152606401611147565b60608215801561417357604051915060008252602082016040526141bd565b6040519150601f8416801560200281840101858101878315602002848b0101015b818310156141ac578051835260209283019201614194565b5050858452601f01601f1916604052505b50949350505050565b60408051808201909152600080825260208201528151600003614215576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b6060600080600061424385613b12565b91945092509050600181600181111561425e5761425e614a27565b14614295576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b84516142a18385614fa0565b146142d8576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b60408051808201909152600080825260208201528152602001906001900390816142ef5790505093506000835b86518110156143dd576000806143626040518060400160405280858c600001516143469190614b97565b8152602001858c6020015161435b9190614fa0565b9052613b12565b50915091506040518060400160405280838361437e9190614fa0565b8152602001848b602001516143939190614fa0565b8152508885815181106143a8576143a8614f71565b60209081029190910101526143be600185614fa0565b93506143ca8183614fa0565b6143d49084614fa0565b9250505061431c565b50845250919392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715614441576144416143e9565b604052919050565b6001600160a01b0381168114611d0757600080fd5b600082601f83011261446f57600080fd5b813567ffffffffffffffff811115614489576144896143e9565b61449c6020601f19601f84011601614418565b8181528460208386010111156144b157600080fd5b816020850160208301376000918101602001919091529392505050565b600060c082840312156144e057600080fd5b60405160c0810167ffffffffffffffff8282108183111715614504576145046143e9565b81604052829350843583526020850135915061451f82614449565b8160208401526040850135915061453582614449565b816040840152606085013560608401526080850135608084015260a085013591508082111561456357600080fd5b506145708582860161445e565b60a0830152505092915050565b6000806040838503121561459057600080fd5b823567ffffffffffffffff8111156145a757600080fd5b6145b3858286016144ce565b92505060208301356145c481614449565b809150509250929050565b6000602082840312156145e157600080fd5b813561283b81614449565b600080604083850312156145ff57600080fd5b823561460a81614449565b915060208301356145c481614449565b600080600080600085870360e081121561463357600080fd5b863567ffffffffffffffff8082111561464b57600080fd5b6146578a838b016144ce565b97506020890135965060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08401121561469057600080fd5b60408901955060c08901359250808311156146aa57600080fd5b828901925089601f8401126146be57600080fd5b82359150808211156146cf57600080fd5b508860208260051b84010111156146e557600080fd5b959894975092955050506020019190565b60006020828403121561470857600080fd5b5035919050565b60005b8381101561472a578181015183820152602001614712565b83811115610fa25750506000910152565b6000815180845261475381602086016020860161470f565b601f01601f19169290920160200192915050565b60208152600061283b602083018461473b565b6000806040838503121561478d57600080fd5b8235915060208301356145c481614449565b6000602082840312156147b157600080fd5b813567ffffffffffffffff8111156147c857600080fd5b6147d4848285016144ce565b949350505050565b67ffffffffffffffff81168114611d0757600080fd5b60006020828403121561480457600080fd5b813561283b816147dc565b6000806040838503121561482257600080fd5b50508035926020909101359150565b8015158114611d0757600080fd5b600080600080600060a0868803121561485757600080fd5b853561486281614449565b9450602086013593506040860135614879816147dc565b9250606086013561488981614831565b9150608086013567ffffffffffffffff8111156148a557600080fd5b6148b18882890161445e565b9150509295509295909350565b8581528460208201527fffffffffffffffff0000000000000000000000000000000000000000000000008460c01b16604082015282151560f81b60488201526000825161491281604985016020870161470f565b919091016049019695505050505050565b60006020828403121561493557600080fd5b815161283b81614449565b63ffffffff81168114611d0757600080fd5b60006020828403121561496457600080fd5b815161283b81614940565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008160001904831182151516156149b8576149b861496f565b500290565b6000602082840312156149cf57600080fd5b815161283b81614831565b6000806000606084860312156149ef57600080fd5b83516149fa81614940565b6020850151909350614a0b816147dc565b6040850151909250614a1c81614449565b809150509250925092565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b600060208284031215614a6857600080fd5b81516003811061283b57600080fd5b600060208284031215614a8957600080fd5b815161283b816147dc565b600060808284031215614aa657600080fd5b6040516080810181811067ffffffffffffffff82111715614ac957614ac96143e9565b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b600060208284031215614b0c57600080fd5b5051919050565b600067ffffffffffffffff80841115614b2e57614b2e6143e9565b8360051b6020614b3f818301614418565b868152918501918181019036841115614b5757600080fd5b865b84811015614b8b57803586811115614b715760008081fd5b614b7d36828b0161445e565b845250918301918301614b59565b50979650505050505050565b600082821015614ba957614ba961496f565b500390565b600067ffffffffffffffff80831681851681830481118215151615614bd557614bd561496f565b02949350505050565b600067ffffffffffffffff808316818516808303821115614c0157614c0161496f565b01949350505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b600082614c4857614c48614c0a565b60001983147f800000000000000000000000000000000000000000000000000000000000000083141615614c7e57614c7e61496f565b500590565b6000808312837f800000000000000000000000000000000000000000000000000000000000000001831281151615614cbd57614cbd61496f565b837f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff018313811615614cf157614cf161496f565b50500390565b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615614d3857614d3861496f565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615614d7357614d7361496f565b60008712925087820587128484161615614d8f57614d8f61496f565b87850587128184161615614da557614da561496f565b505050929093029392505050565b6000808212827f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03841381151615614ded57614ded61496f565b827f8000000000000000000000000000000000000000000000000000000000000000038412811615614e2157614e2161496f565b50500190565b600082614e3657614e36614c0a565b500490565b86815260006001600160a01b03808816602084015280871660408401525084606083015283608083015260c060a0830152614e7960c083018461473b565b98975050505050505050565b805160ff81168114614e9657600080fd5b919050565b600060c08284031215614ead57600080fd5b60405160c0810181811067ffffffffffffffff82111715614ed057614ed06143e9565b6040528251614ede81614940565b8152614eec60208401614e85565b6020820152614efd60408401614e85565b60408201526060830151614f1081614940565b60608201526080830151614f2381614940565b608082015260a08301516fffffffffffffffffffffffffffffffff81168114614f4b57600080fd5b60a08201529392505050565b60006000198203614f6a57614f6a61496f565b5060010190565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b60008219821115614fb357614fb361496f565b500190565b600060ff831680614fcb57614fcb614c0a565b8060ff84160691505092915050565b600060ff821660ff841680821015614ff457614ff461496f565b9003939250505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0Qi8\x03\x80b\0Qi\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x11V[`\x03`\x80R`\xA0\x81\x90Rb\0\0Hb\0\0OV[Pb\0\x01+V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x0FW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01$W`\0\x80\xFD[PQ\x91\x90PV[`\x80Q`\xA0QaP\nb\0\x01_`\09`\0\x81\x81a\x05\xFC\x01Ra\x1B\xCE\x01R`\0\x81\x81a\x02c\x01Ra\x10\x9B\x01RaP\n`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xD1W`\x005`\xE0\x1C\x80cq\xC1Vn\x11a\0\xF7W\x80c\xA3\x86\x0FH\x11a\0\x95W\x80c\xCF\xF0\xAB\x96\x11a\0dW\x80c\xCF\xF0\xAB\x96\x14a\x06 W\x80c\xDA\xD5D\xE0\x14a\x06\xC1W\x80c\xE9\xE0\\B\x14a\x06\xD6W\x80c\xF2\xB4\xE6\x17\x14a\x06\xE9W`\0\x80\xFD[\x80c\xA3\x86\x0FH\x14a\x05 W\x80c\xB6\x82\xC4D\x14a\x05@W\x80c\xBB,r~\x14a\x05`W\x80c\xBFe:\\\x14a\x05\xEDW`\0\x80\xFD[\x80c\x95+'\x97\x11a\0\xD1W\x80c\x95+'\x97\x14a\x04\x9BW\x80c\x9B\xF6-\x82\x14a\x04\xB0W\x80c\xA1B8\xE7\x14a\x04\xD0W\x80c\xA3]\x99\xDF\x14a\x05\0W`\0\x80\xFD[\x80cq\xC1Vn\x14a\x04[W\x80c\x8BL@\xB0\x14a\x01\xF6W\x80c\x8C1R\xE9\x14a\x04{W`\0\x80\xFD[\x80cE\x88M2\x11a\x01oW\x80cQ7G\xAB\x11a\x01>W\x80cQ7G\xAB\x14a\x03\x9FW\x80cT\xFDMP\x14a\x03\xDAW\x80c\\\x0C\xBA3\x14a\x04&W\x80c\\\x97Z\xBB\x14a\x04FW`\0\x80\xFD[\x80cE\x88M2\x14a\x03\x01W\x80cH\\\xC9U\x14a\x031W\x80cHpIo\x14a\x03QW\x80cO\xD0CL\x14a\x03qW`\0\x80\xFD[\x80c<\x9F9|\x11a\x01\xABW\x80c<\x9F9|\x14a\x02\x8DW\x80c>G\x15\x8C\x14a\x02\xB7W\x80cC\xCA\x1CP\x14a\x02\xCCW\x80cE*\x93 \x14a\x02\xECW`\0\x80\xFD[\x80c3\xD7\xE2\xBD\x14a\x01\xFDW\x80c5\xE8\n\xB3\x14a\x02:W\x80c8\xD3\x8C\x97\x14a\x02OW`\0\x80\xFD[6a\x01\xF8Wa\x01\xF634b\x01\x86\xA0`\0`@Q\x80` \x01`@R\x80`\0\x81RPa\x06\xFEV[\0[`\0\x80\xFD[4\x80\x15a\x02\tW`\0\x80\xFD[P`7Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02FW`\0\x80\xFD[Pa\x02\x1Da\t[V[4\x80\x15a\x02[W`\0\x80\xFD[P`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x021V[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xA2a\t\xE7V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x021V[4\x80\x15a\x02\xC3W`\0\x80\xFD[Pa\x02\x1Da\nnV[4\x80\x15a\x02\xD8W`\0\x80\xFD[Pa\x01\xF6a\x02\xE76`\x04aE}V[a\x0CRV[4\x80\x15a\x02\xF8W`\0\x80\xFD[Pa\x02\x1Da\x0F\xA8V[4\x80\x15a\x03\rW`\0\x80\xFD[Pa\x03!a\x03\x1C6`\x04aE\xCFV[a\x10\x0BV[`@Q\x90\x15\x15\x81R` \x01a\x021V[4\x80\x15a\x03=W`\0\x80\xFD[Pa\x01\xF6a\x03L6`\x04aE\xECV[a\x10\x99V[4\x80\x15a\x03]W`\0\x80\xFD[Pa\x01\xF6a\x03l6`\x04aF\x1AV[a\x12\x81V[4\x80\x15a\x03}W`\0\x80\xFD[Pa\x03\x86a\x19CV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x021V[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x03\xCCa\x03\xBA6`\x04aF\xF6V[`\0\x90\x81R`<` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x021V[4\x80\x15a\x03\xE6W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\x05\x81R\x7F5.2.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x021\x91\x90aGgV[4\x80\x15a\x042W`\0\x80\xFD[P`>Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04RW`\0\x80\xFD[Pa\x03!a\x19\xCAV[4\x80\x15a\x04gW`\0\x80\xFD[Pa\x01\xF6a\x04v6`\x04aGzV[a\x1AQV[4\x80\x15a\x04\x87W`\0\x80\xFD[Pa\x01\xF6a\x04\x966`\x04aG\x9FV[a\x1C\xFDV[4\x80\x15a\x04\xA7W`\0\x80\xFD[Pa\x03\xCCa\x1D\nV[4\x80\x15a\x04\xBCW`\0\x80\xFD[P`2Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xDCW`\0\x80\xFD[Pa\x03!a\x04\xEB6`\x04aF\xF6V[`3` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\x0CW`\0\x80\xFD[Pa\x03\x86a\x05\x1B6`\x04aG\xF2V[a\x1D\x91V[4\x80\x15a\x05,W`\0\x80\xFD[Pa\x02\x1Da\x05;6`\x04aH\x0FV[a\x1D\xAAV[4\x80\x15a\x05LW`\0\x80\xFD[P`?Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05lW`\0\x80\xFD[Pa\x05\xC5a\x05{6`\x04aGzV[`9` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x81\x16\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x021V[4\x80\x15a\x05\xF9W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xCCV[4\x80\x15a\x06,W`\0\x80\xFD[P`\x01Ta\x06\x88\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x021V[4\x80\x15a\x06\xCDW`\0\x80\xFD[Pa\x02\x1Da\x1D\xE2V[a\x01\xF6a\x06\xE46`\x04aH?V[a\x06\xFEV[4\x80\x15a\x06\xF5W`\0\x80\xFD[Pa\x02\x1Da\x1E)V[\x82`\0Z\x90Pa\x07\x0Ca\x1E\x8CV[\x15a\x07IW4\x15a\x07IW`@Q\x7F\xBDX\xE0\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07Qa\x1F\x0EV[\x15a\x07\xC6W4\x15a\x07\xC6W`?`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xE1\x16\xBF4`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x07\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xC0W=`\0\x80>=`\0\xFD[PPPPP[\x83\x80\x15a\x07\xDBWP`\x01`\x01`\xA0\x1B\x03\x87\x16\x15\x15[\x15a\x08\x12W`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x1C\x83Qa\x1D\x91V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x08iW`@Q\x7Fp\xC8\xBD\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x01\xD4\xC0\x83Q\x11\x15a\x08\xA7W`@Q\x7FZ\xA3\xBA\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x08\xB0a\x1F\xCDV[a\x08\xCDWP3s\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\x01[`\x004\x88\x88\x88\x88`@Q` \x01a\x08\xE8\x95\x94\x93\x92\x91\x90aH\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB3\x815h\xD9\x99\x1F\xC9Q\x96\x1F\xCBLxH\x93WB@\xA2\x89%`M\t\xFCW|U\xBB|2\x84`@Qa\t>\x91\x90aGgV[`@Q\x80\x91\x03\x90\xA4PPa\tR\x82\x82a \nV[PPPPPPPV[`7T`@\x80Q\x7F5\xE8\n\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c5\xE8\n\xB3\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aI#V[\x90P\x90V[`>T`@\x80Q\x7F<\x9F9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c<\x9F9|\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\nJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aIRV[`\0\x80a\n\x99\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\n\xAFW\x91\x90PV[`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x81RPQ`\x02a\n\xF2\x91\x90aI\x9EV[`@\x80Q0` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x91\x90\x91\x17\x90a\x0BM\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 T\x90V[\x14a\x0B\x84W`@Q\x7FT\xE43\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R`\0\x90a\x0B\xA6\x90``\x01a\x0B3V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0C W\x80`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x19\x91\x90aI#V[\x92PPP\x90V[`@Q\x7F3!D\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0CZa\"\xE1V[a\x0Cba\x1E\x8CV[\x15a\x0C\xA3W``\x82\x01Q\x15a\x0C\xA3W`@Q\x7F\xBDX\xE0\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`2T`\x01`\x01`\xA0\x1B\x03\x16a\xDE\xAD\x14a\x0C\xE9W`@Q\x7F\xDF\xEA\xAE\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xF6\x82`@\x01Qa# V[\x15a\r-W`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r8\x83a#IV[\x90Pa\rD\x81\x83a\x1AQV[`\0\x81\x81R`3` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\r\x83a\x1F\x0EV[\x15a\x0E\x15W``\x83\x01Q\x15a\x0E\x15W`?T``\x84\x01Q`@Q\x7F\x8DD[\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8DD[\xD0\x91a\r\xE2\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x10W=`\0\x80>=`\0\xFD[PPPP[\x82` \x01Q`2`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\0a\x0E^\x84`@\x01Q\x85`\x80\x01Q\x86``\x01Q\x87`\xA0\x01Qa#\x96V[`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U`@Q\x90\x91P\x82\x90\x7F\xDB\\vR\x85z\xA1c\xDA\xAD\xD6p\xE1\x16b\x8F\xB4.\x86\x9D\x8A\xC4%\x1E\xF8\x97\x1D\x9EW'\xDF\x1B\x90a\x0E\xC3\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x0E\xD3a\x1F\x0EV[\x15a\x0F]W\x80\x15\x80\x15a\x0E\xEAWP`\0\x84``\x01Q\x11[\x15a\x0F]W`?`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xE1\x16\xBF\x85``\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x0FCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0FWW=`\0\x80>=`\0\xFD[PPPPP[\x80\x15\x80\x15a\x0FkWP2`\x01\x14[\x15a\x0F\xA2W`@Q\x7F\xABX\x106\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`7T`@\x80Q\x7FE*\x93 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cE*\x93 \x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[`>T`@Q\x7FE\x88M2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x92\x16\x90cE\x88M2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x93\x91\x90aI\xBDV[\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x10\xD9WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x11PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x11\x89a#\xF4V[`7\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`>\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90Ua\x11\xD9a$[V[`2T`\x01`\x01`\xA0\x1B\x03\x16a\x12\x16W`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U[a\x12\x1Ea&\x0EV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[a\x12\x89a\"\xE1V[a\x12\x96\x85`@\x01Qa# V[\x15a\x12\xCDW`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xD5a\x1E\x8CV[\x15a\x13\x16W``\x85\x01Q\x15a\x13\x16W`@Q\x7F\xBDX\xE0\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13 a\x1E)V[`\x01`\x01`\xA0\x1B\x03\x16c\xBB\x8A\xA1\xFC\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13M\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x8E\x91\x90aI\xDAV[`>T`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x95P\x91\x16\x92PcIk\x9C\x16\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x19\x91\x90aI\xBDV[a\x14OW`@Q\x7F\xF3\x95$\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7F\x04\xE5\x0F\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\x04\xE5\x0F\xED\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xD6\x91\x90aI\xBDV[a\x15\x0CW`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01`\x01`\xA0\x1B\x03\x16c \r.\xD2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15p\x91\x90aJVV[`\x02\x81\x11\x15a\x15\x81Wa\x15\x81aJ'V[\x03a\x15\xB8W`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16*\x81`\x01`\x01`\xA0\x1B\x03\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x1D\x91\x90aJwV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11a\x16lW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x83a\x16~6\x86\x90\x03\x86\x01\x86aJ\x94V[a'\x07V[a\x16\xEB\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE8\x91\x90aJ\xFAV[\x90V[\x14a\x17\"W`@Q\x7FBaI\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x17-\x87a#IV[\x90P`\0\x81`\0`@Q` \x01a\x17N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92Pa\x17\xC5\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82\x01\x90\x91R`\x01\x82R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R\x90a\x17\xBB\x87\x89aK\x13V[\x89`@\x015a'FV[\x15\x15`\0\x03a\x18\0W`@Q\x7F.W\xEF:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16` \x80\x85\x01\x91\x82R`\0\x88\x81R`9\x82R\x86\x81 3\x80\x83R\x90\x83R\x87\x82 \x96Q\x87T\x94Q\x90\x95\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x94\x86\x16\x94\x90\x94\x17\x92\x90\x92\x17\x90\x94U\x86\x81R`<\x84R\x84\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x84\x82 \x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x92\x17\x90\x91U\x8B\x84\x01Q\x92\x8C\x01Q\x93Q\x92\x82\x16\x93\x90\x91\x16\x91\x85\x91\x7Fg\xA6 \x8C\xFC\xC0\x80\x1DP\xF6\xCB\xE7ds?O\xDD\xF6j\xC0\xB0DB\x06\x1A\x8A\x8C\x0C\xB6\xB6?b\x91\xA4`@Q3\x90\x83\x90\x7Fy\x8F\x9F\x13i_\x8F\x04Z\xA5\xF8\x0E\xD8\xEF\xEB\xB6\x95\xF3\xC7\xFEe\xDA8\x19i\xF2\xF2\x8B\xF3\xC6\x0B\x97\x90`\0\x90\xA3PPPPPPPPV[`>T`@\x80Q\x7F@\x86\xD1\x83\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c@\x86\xD1\x83\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x19\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aJwV[`7T`@\x80Q\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\\\x97Z\xBB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aI\xBDV[`\0\x82\x81R`9` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x85R\x90\x83R\x81\x84 \x82Q\x80\x84\x01\x84R\x90T\x91\x82\x16\x80\x82Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x85\x01R\x86\x85R`3\x90\x93R\x92 T\x90\x91\x90`\xFF\x16\x15a\x1A\xF3W`@Q\x7Fs\n\x10t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B;W`@Q\x7F\xCC\xA6\xAF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B|\x81`\x01`\x01`\xA0\x1B\x03\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xF9W=`\0\x80>=`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1B\xCCW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x1C\x07\x91\x90aK\x97V[\x11a\x1C>W`@Q\x7F\xD9\xBC\x01\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7FlODg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90clODg\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xC5\x91\x90aI\xBDV[a\x0F\xA2W`@Q\x7F3*W\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x1D\x07\x813a\x0CRV[PV[`>T`@\x80Q\x7F\x95+'\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95+'\x97\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1DmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aJ\xFAV[`\0a\x1D\x9E\x82`(aK\xAEV[a\x10\x93\x90aR\x08aK\xDEV[`<` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x1D\xC6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`\0a\x1D\xECa\nnV[`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[`>T`@\x80Q\x7F\xF2\xB4\xE6\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2\xB4\xE6\x17\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FCUSTOM_GAS_TOKEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A-W=`\0\x80>=`\0\xFD[`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xB4\x91\x90aI\xBDV[\x80\x15a\t\xE2WPP`?T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15\x90V[`\x0023\x03a\x1F\xDCWP`\x01\x90V[3;`\x17\x03a \x04W`@Q` \x81\x01`@R` `\0\x823<Q`\xE8\x1Cb\xEF\x01\0\x14\x90P\x90V[P`\0\x90V[`\x01T`\0\x90a @\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16CaK\x97V[\x90P`\0a La'jV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a m\x91\x90aL9V[\x90P\x82\x15a!\xA4W`\x01T`\0\x90a \xA4\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aL\x83V[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a \xBB\x91\x90aL\xF7V[`\x01Ta \xDB\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aL\xF7V[a \xE5\x91\x90aL9V[`\x01T\x90\x91P`\0\x90a!6\x90a!\x0F\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aM\xB3V[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(#V[\x90P`\x01\x86\x11\x15a!eWa!ba!\x0F\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa!]\x91\x90aK\x97V[a(BV[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a!\xD7\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aK\xDEV[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\"dW`@Q\x7Fw\xEB\xEFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\0\x90a\"\x90\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16aI\x9EV[\x90P`\0a\"\xA2Hc;\x9A\xCA\0a(\x97V[a\"\xAC\x90\x83aN'V[\x90P`\0Za\"\xBB\x90\x88aK\x97V[\x90P\x80\x82\x11\x15a\"\xD7Wa\"\xD7a\"\xD2\x82\x84aK\x97V[a(\xAEV[PPPPPPPPV[a\"\xE9a\x19\xCAV[\x15a\x1C\xFBW`@Q\x7F\xB9\xC3\xC2\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x80a\x10\x93WPP`?T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a#y\x97\x90\x96\x95\x91\x01aN;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a#\xA6\x86`\0a(\xDCV[\x90P\x80a#\xDCWc\x08\xC3y\xA0`\0R` \x80Rx\x18SafeCall: Not enough gas`XR`d`\x1C\xFD[`\0\x80\x85Q` \x87\x01\x88\x8BZ\xF1\x97\x96PPPPPPPV[3a#\xFDa\nnV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a$$WP3a$\x18a\x1D\xE2V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x1C\xFBW`@Q\x7F\xC4\x05\n&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\0\x91\x90aI\xBDV[\x80\x15a%\x15WP`?T`\x01`\x01`\xA0\x1B\x03\x16\x15[\x80a%\xD7WP`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xC0\x91\x90aI\xBDV[\x15\x80\x15a%\xD7WP`?T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[\x15a\x1C\xFBW`@Q\x7F\x9CF\xCDy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16a&\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\x01Tx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1C\xFBW`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90\x92\x01\x81\x90Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x17`\x01UV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a#y\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`\0\x80a'R\x86a(\xFAV[\x90Pa'`\x81\x86\x86\x86a),V[\x96\x95PPPPPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R`7T`@\x80Q\x7F\xCCs\x1B\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xCCs\x1B\x02\x91`\x04\x80\x83\x01\x92`\xC0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x93\x91\x90aN\x9BV[`\0a(8a(2\x85\x85a)\\V[\x83a)lV[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a(\x83a(Z\x85\x83aL9V[a(l\x90g\r\xE0\xB6\xB3\xA7d\0\0aL\x83V[a(~\x85g\r\xE0\xB6\xB3\xA7d\0\0aL\xF7V[a){V[a(\x8D\x90\x86aL\xF7V[a(8\x91\x90aL9V[`\0\x81\x83\x10\x15a(\xA7W\x81a(;V[P\x90\x91\x90PV[`\0\x80Z\x90P[\x82Za(\xC1\x90\x83aK\x97V[\x10\x15a(\xD7Wa(\xD0\x82aOWV[\x91Pa(\xB5V[PPPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[``\x81\x80Q\x90` \x01 `@Q` \x01a)\x16\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a)S\x84a)=\x87\x86\x86a)\xACV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[\x95\x94PPPPPV[`\0\x81\x83\x12\x15a(\xA7W\x81a(;V[`\0\x81\x83\x12a(\xA7W\x81a(;V[`\0a(;g\r\xE0\xB6\xB3\xA7d\0\0\x83a)\x93\x86a2\xBEV[a)\x9D\x91\x90aL\xF7V[a)\xA7\x91\x90aL9V[a4\xE8V[```\0\x84Q\x11a)\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FMerkleTrie: empty key\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[`\0a*\n\x84a7\rV[\x90P`\0a*\x17\x86a7\xF9V[\x90P`\0\x84`@Q` \x01a*.\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80[\x84Q\x81\x10\x15a2OW`\0\x85\x82\x81Q\x81\x10a*`Wa*`aOqV[` \x02` \x01\x01Q\x90P\x84Q\x83\x11\x15a*\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FMerkleTrie: key index exceeds to`D\x82\x01R\x7Ftal key length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[\x82`\0\x03a+\x80W\x80Q\x80Q` \x91\x82\x01 `@Qa+/\x92a+\t\x92\x91\x01\x90\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[a+{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FMerkleTrie: invalid root hash\0\0\0`D\x82\x01R`d\x01a\x11GV[a,\xA3V[\x80QQ` \x11a,\x1CW\x80Q\x80Q` \x91\x82\x01 `@Qa+\xAA\x92a+\t\x92\x91\x01\x90\x81R` \x01\x90V[a+{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FMerkleTrie: invalid large intern`D\x82\x01R\x7Fal hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[\x80Q\x84Q` \x80\x87\x01\x91\x90\x91 \x82Q\x91\x90\x92\x01 \x14a,\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FMerkleTrie: invalid internal nod`D\x82\x01R\x7Fe hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[a,\xAF`\x10`\x01aO\xA0V[\x81` \x01QQ\x03a.WW\x84Q\x83\x03a-\xEFWa,\xE9\x81` \x01Q`\x10\x81Q\x81\x10a,\xDCWa,\xDCaOqV[` \x02` \x01\x01Qa8\\V[\x96P`\0\x87Q\x11a-bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (branch)\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\x01\x86Qa-p\x91\x90aK\x97V[\x82\x14a-\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (branch)\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[PPPPPPa(;V[`\0\x85\x84\x81Q\x81\x10a.\x03Wa.\x03aOqV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\0\x82` \x01Q\x82`\xFF\x16\x81Q\x81\x10a..Wa..aOqV[` \x02` \x01\x01Q\x90Pa.A\x81a9\x10V[\x95Pa.N`\x01\x86aO\xA0V[\x94PPPa2<V[`\x02\x81` \x01QQ\x03a1\xCEW`\0a.o\x82a95V[\x90P`\0\x81`\0\x81Q\x81\x10a.\x86Wa.\x86aOqV[\x01` \x01Q`\xF8\x1C\x90P`\0a.\x9D`\x02\x83aO\xB8V[a.\xA8\x90`\x02aO\xDAV[\x90P`\0a.\xB9\x84\x83`\xFF\x16a9YV[\x90P`\0a.\xC7\x8A\x89a9YV[\x90P`\0a.\xD5\x83\x83a9\x8FV[\x90P\x80\x83Q\x14a/MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: path remainder must `D\x82\x01R\x7Fshare all nibbles with key\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\xFF\x85\x16`\x02\x14\x80a/bWP`\xFF\x85\x16`\x03\x14[\x15a1\x03W\x80\x82Q\x14a/\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FMerkleTrie: key remainder must b`D\x82\x01R\x7Fe identical to path remainder\0\0\0`d\x82\x01R`\x84\x01a\x11GV[a/\xF7\x87` \x01Q`\x01\x81Q\x81\x10a,\xDCWa,\xDCaOqV[\x9CP`\0\x8DQ\x11a0pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (leaf)\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\x01\x8CQa0~\x91\x90aK\x97V[\x88\x14a0\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (leaf)\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[PPPPPPPPPPPPa(;V[`\xFF\x85\x16\x15\x80a1\x16WP`\xFF\x85\x16`\x01\x14[\x15a1UWa1B\x87` \x01Q`\x01\x81Q\x81\x10a15Wa15aOqV[` \x02` \x01\x01Qa9\x10V[\x99Pa1N\x81\x8AaO\xA0V[\x98Pa1\xC3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FMerkleTrie: received a node with`D\x82\x01R\x7F an unknown prefix\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[PPPPPPa2<V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMerkleTrie: received an unparsea`D\x82\x01R\x7Fble node\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[P\x80a2G\x81aOWV[\x91PPa*CV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FMerkleTrie: ran out of proof ele`D\x82\x01R\x7Fments\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\0\x80\x82\x13a3\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[`\0``a3\x1C\x84a:CV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a5\x19WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a5qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x80Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7+Wa7+aC\xE9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7pW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a7IW\x90P[P\x91P`\0[\x81\x81\x10\x15a7\xF2W`@Q\x80`@\x01`@R\x80\x85\x83\x81Q\x81\x10a7\x9BWa7\x9BaOqV[` \x02` \x01\x01Q\x81R` \x01a7\xCA\x86\x84\x81Q\x81\x10a7\xBDWa7\xBDaOqV[` \x02` \x01\x01Qa:\xFFV[\x81RP\x83\x82\x81Q\x81\x10a7\xDFWa7\xDFaOqV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a7vV[PP\x91\x90PV[``\x80`@Q\x90P\x82Q\x80`\x01\x1B`?\x81\x01`\x1F\x19\x16\x83\x01`@R\x80\x83RP` \x84\x01` \x83\x01`\0[\x83\x81\x10\x15a8QW\x80`\x01\x1B\x82\x01\x81\x84\x01Q`\0\x1A\x80`\x04\x1C\x82S`\x0F\x81\x16`\x01\x83\x01SPP`\x01\x01a8#V[P\x92\x95\x94PPPPPV[```\0\x80`\0a8l\x85a;\x12V[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15a8\x87Wa8\x87aJ'V[\x14a8\xBEW`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a8\xC8\x82\x84aO\xA0V[\x85Q\x14a9\x01W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)S\x85` \x01Q\x84\x84a?\xB0V[``` \x82`\0\x01Q\x10a9,Wa9'\x82a8\\V[a\x10\x93V[a\x10\x93\x82a@DV[``a\x10\x93a9T\x83` \x01Q`\0\x81Q\x81\x10a,\xDCWa,\xDCaOqV[a7\xF9V[``\x82Q\x82\x10a9xWP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\x93V[a(;\x83\x83\x84\x86Qa9\x8A\x91\x90aK\x97V[a@ZV[`\0\x80\x82Q\x84Q\x10a9\xA2W\x82Qa9\xA5V[\x83Q[\x90P[\x80\x82\x10\x80\x15a:,WP\x82\x82\x81Q\x81\x10a9\xC4Wa9\xC4aOqV[` \x01\x01Q`\xF8\x1C`\xF8\x1B~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x84\x83\x81Q\x81\x10a:\x03Wa:\x03aOqV[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[\x15a:<W\x81`\x01\x01\x91Pa9\xA8V[P\x92\x91PPV[`\0\x80\x82\x11a:\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[``a\x10\x93a;\r\x83aA\xC6V[aB3V[`\0\x80`\0\x83`\0\x01Q`\0\x03a;UW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11a;zW`\0`\x01`\0\x94P\x94P\x94PPPa?\xA9V[`\xB7\x81\x11a<\x90W`\0a;\x8F`\x80\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a;\xCEW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15a<FWP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15a<}W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92Pa?\xA9\x91PPV[`\xBF\x81\x11a=\xEEW`\0a<\xA5`\xB7\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a<\xE4W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03a=FW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11a=\x8EW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\x98\x81\x84aO\xA0V[\x89Q\x11a=\xD1W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\xDC\x83`\x01aO\xA0V[\x97P\x95P`\0\x94Pa?\xA9\x93PPPPV[`\xF7\x81\x11a>SW`\0a>\x03`\xC0\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a>BW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92Pa?\xA9\x91PPV[`\0a>``\xF7\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a>\x9FW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03a?\x01W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11a?IW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a?S\x81\x84aO\xA0V[\x89Q\x11a?\x8CW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a?\x97\x83`\x01aO\xA0V[\x97P\x95P`\x01\x94Pa?\xA9\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?\xCBWa?\xCBaC\xE9V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a?\xF5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15a(;W`\0a@\n\x84\x86aO\xA0V[\x90P` \x82\x01`\0[\x84\x81\x10\x15a@+W\x82\x81\x01Q\x82\x82\x01R` \x01a@\x13V[\x84\x81\x11\x15a@:W`\0\x85\x83\x01R[PPP\x93\x92PPPV[``a\x10\x93\x82` \x01Q`\0\x84`\0\x01Qa?\xB0V[``\x81\x82`\x1F\x01\x10\x15a@\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[\x82\x82\x84\x01\x10\x15aA\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[\x81\x83\x01\x84Q\x10\x15aATW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Fslice_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[``\x82\x15\x80\x15aAsW`@Q\x91P`\0\x82R` \x82\x01`@RaA\xBDV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15aA\xACW\x80Q\x83R` \x92\x83\x01\x92\x01aA\x94V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03aB\x15W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0aBC\x85a;\x12V[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15aB^WaB^aJ'V[\x14aB\x95W`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84QaB\xA1\x83\x85aO\xA0V[\x14aB\xD8W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81aB\xEFW\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15aC\xDDW`\0\x80aCb`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01QaCF\x91\x90aK\x97V[\x81R` \x01\x85\x8C` \x01QaC[\x91\x90aO\xA0V[\x90Ra;\x12V[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83aC~\x91\x90aO\xA0V[\x81R` \x01\x84\x8B` \x01QaC\x93\x91\x90aO\xA0V[\x81RP\x88\x85\x81Q\x81\x10aC\xA8WaC\xA8aOqV[` \x90\x81\x02\x91\x90\x91\x01\x01RaC\xBE`\x01\x85aO\xA0V[\x93PaC\xCA\x81\x83aO\xA0V[aC\xD4\x90\x84aO\xA0V[\x92PPPaC\x1CV[P\x84RP\x91\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aDAWaDAaC\xE9V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1D\x07W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aDoW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x89WaD\x89aC\xE9V[aD\x9C` `\x1F\x19`\x1F\x84\x01\x16\x01aD\x18V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aD\xB1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15aD\xE0W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aE\x04WaE\x04aC\xE9V[\x81`@R\x82\x93P\x845\x83R` \x85\x015\x91PaE\x1F\x82aDIV[\x81` \x84\x01R`@\x85\x015\x91PaE5\x82aDIV[\x81`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15aEcW`\0\x80\xFD[PaEp\x85\x82\x86\x01aD^V[`\xA0\x83\x01RPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aE\x90W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xA7W`\0\x80\xFD[aE\xB3\x85\x82\x86\x01aD\xCEV[\x92PP` \x83\x015aE\xC4\x81aDIV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aE\xE1W`\0\x80\xFD[\x815a(;\x81aDIV[`\0\x80`@\x83\x85\x03\x12\x15aE\xFFW`\0\x80\xFD[\x825aF\n\x81aDIV[\x91P` \x83\x015aE\xC4\x81aDIV[`\0\x80`\0\x80`\0\x85\x87\x03`\xE0\x81\x12\x15aF3W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aFKW`\0\x80\xFD[aFW\x8A\x83\x8B\x01aD\xCEV[\x97P` \x89\x015\x96P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x01\x12\x15aF\x90W`\0\x80\xFD[`@\x89\x01\x95P`\xC0\x89\x015\x92P\x80\x83\x11\x15aF\xAAW`\0\x80\xFD[\x82\x89\x01\x92P\x89`\x1F\x84\x01\x12aF\xBEW`\0\x80\xFD[\x825\x91P\x80\x82\x11\x15aF\xCFW`\0\x80\xFD[P\x88` \x82`\x05\x1B\x84\x01\x01\x11\x15aF\xE5W`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95PPP` \x01\x91\x90V[`\0` \x82\x84\x03\x12\x15aG\x08W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15aG*W\x81\x81\x01Q\x83\x82\x01R` \x01aG\x12V[\x83\x81\x11\x15a\x0F\xA2WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaGS\x81` \x86\x01` \x86\x01aG\x0FV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a(;` \x83\x01\x84aG;V[`\0\x80`@\x83\x85\x03\x12\x15aG\x8DW`\0\x80\xFD[\x825\x91P` \x83\x015aE\xC4\x81aDIV[`\0` \x82\x84\x03\x12\x15aG\xB1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xC8W`\0\x80\xFD[aG\xD4\x84\x82\x85\x01aD\xCEV[\x94\x93PPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1D\x07W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aH\x04W`\0\x80\xFD[\x815a(;\x81aG\xDCV[`\0\x80`@\x83\x85\x03\x12\x15aH\"W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x1D\x07W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aHWW`\0\x80\xFD[\x855aHb\x81aDIV[\x94P` \x86\x015\x93P`@\x86\x015aHy\x81aG\xDCV[\x92P``\x86\x015aH\x89\x81aH1V[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xA5W`\0\x80\xFD[aH\xB1\x88\x82\x89\x01aD^V[\x91PP\x92\x95P\x92\x95\x90\x93PV[\x85\x81R\x84` \x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xC0\x1B\x16`@\x82\x01R\x82\x15\x15`\xF8\x1B`H\x82\x01R`\0\x82QaI\x12\x81`I\x85\x01` \x87\x01aG\x0FV[\x91\x90\x91\x01`I\x01\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aI5W`\0\x80\xFD[\x81Qa(;\x81aDIV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1D\x07W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aIdW`\0\x80\xFD[\x81Qa(;\x81aI@V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aI\xB8WaI\xB8aIoV[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aI\xCFW`\0\x80\xFD[\x81Qa(;\x81aH1V[`\0\x80`\0``\x84\x86\x03\x12\x15aI\xEFW`\0\x80\xFD[\x83QaI\xFA\x81aI@V[` \x85\x01Q\x90\x93PaJ\x0B\x81aG\xDCV[`@\x85\x01Q\x90\x92PaJ\x1C\x81aDIV[\x80\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aJhW`\0\x80\xFD[\x81Q`\x03\x81\x10a(;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aJ\x89W`\0\x80\xFD[\x81Qa(;\x81aG\xDCV[`\0`\x80\x82\x84\x03\x12\x15aJ\xA6W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aJ\xC9WaJ\xC9aC\xE9V[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aK\x0CW`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15aK.WaK.aC\xE9V[\x83`\x05\x1B` aK?\x81\x83\x01aD\x18V[\x86\x81R\x91\x85\x01\x91\x81\x81\x01\x906\x84\x11\x15aKWW`\0\x80\xFD[\x86[\x84\x81\x10\x15aK\x8BW\x805\x86\x81\x11\x15aKqW`\0\x80\x81\xFD[aK}6\x82\x8B\x01aD^V[\x84RP\x91\x83\x01\x91\x83\x01aKYV[P\x97\x96PPPPPPPV[`\0\x82\x82\x10\x15aK\xA9WaK\xA9aIoV[P\x03\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aK\xD5WaK\xD5aIoV[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aL\x01WaL\x01aIoV[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aLHWaLHaL\nV[`\0\x19\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aL~WaL~aIoV[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15aL\xBDWaL\xBDaIoV[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15aL\xF1WaL\xF1aIoV[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15aM8WaM8aIoV[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15aMsWaMsaIoV[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15aM\x8FWaM\x8FaIoV[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15aM\xA5WaM\xA5aIoV[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15aM\xEDWaM\xEDaIoV[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15aN!WaN!aIoV[PP\x01\x90V[`\0\x82aN6WaN6aL\nV[P\x04\x90V[\x86\x81R`\0`\x01`\x01`\xA0\x1B\x03\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01RaNy`\xC0\x83\x01\x84aG;V[\x98\x97PPPPPPPPV[\x80Q`\xFF\x81\x16\x81\x14aN\x96W`\0\x80\xFD[\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15aN\xADW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aN\xD0WaN\xD0aC\xE9V[`@R\x82QaN\xDE\x81aI@V[\x81RaN\xEC` \x84\x01aN\x85V[` \x82\x01RaN\xFD`@\x84\x01aN\x85V[`@\x82\x01R``\x83\x01QaO\x10\x81aI@V[``\x82\x01R`\x80\x83\x01QaO#\x81aI@V[`\x80\x82\x01R`\xA0\x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aOKW`\0\x80\xFD[`\xA0\x82\x01R\x93\x92PPPV[`\0`\0\x19\x82\x03aOjWaOjaIoV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aO\xB3WaO\xB3aIoV[P\x01\x90V[`\0`\xFF\x83\x16\x80aO\xCBWaO\xCBaL\nV[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15aO\xF4WaO\xF4aIoV[\x90\x03\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101d15760003560e01c806371c1566e116100f7578063a3860f4811610095578063cff0ab9611610064578063cff0ab9614610620578063dad544e0146106c1578063e9e05c42146106d6578063f2b4e617146106e957600080fd5b8063a3860f4814610520578063b682c44414610540578063bb2c727e14610560578063bf653a5c146105ed57600080fd5b8063952b2797116100d1578063952b27971461049b5780639bf62d82146104b0578063a14238e7146104d0578063a35d99df1461050057600080fd5b806371c1566e1461045b5780638b4c40b0146101f65780638c3152e91461047b57600080fd5b806345884d321161016f578063513747ab1161013e578063513747ab1461039f57806354fd4d50146103da5780635c0cba33146104265780635c975abb1461044657600080fd5b806345884d3214610301578063485cc955146103315780634870496f146103515780634fd0434c1461037157600080fd5b80633c9f397c116101ab5780633c9f397c1461028d5780633e47158c146102b757806343ca1c50146102cc578063452a9320146102ec57600080fd5b806333d7e2bd146101fd57806335e80ab31461023a57806338d38c971461024f57600080fd5b366101f8576101f63334620186a06000604051806020016040528060008152506106fe565b005b600080fd5b34801561020957600080fd5b5060375461021d906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561024657600080fd5b5061021d61095b565b34801561025b57600080fd5b5060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152602001610231565b34801561029957600080fd5b506102a26109e7565b60405163ffffffff9091168152602001610231565b3480156102c357600080fd5b5061021d610a6e565b3480156102d857600080fd5b506101f66102e736600461457d565b610c52565b3480156102f857600080fd5b5061021d610fa8565b34801561030d57600080fd5b5061032161031c3660046145cf565b61100b565b6040519015158152602001610231565b34801561033d57600080fd5b506101f661034c3660046145ec565b611099565b34801561035d57600080fd5b506101f661036c36600461461a565b611281565b34801561037d57600080fd5b50610386611943565b60405167ffffffffffffffff9091168152602001610231565b3480156103ab57600080fd5b506103cc6103ba3660046146f6565b6000908152603c602052604090205490565b604051908152602001610231565b3480156103e657600080fd5b50604080518082018252600581527f352e322e30000000000000000000000000000000000000000000000000000000602082015290516102319190614767565b34801561043257600080fd5b50603e5461021d906001600160a01b031681565b34801561045257600080fd5b506103216119ca565b34801561046757600080fd5b506101f661047636600461477a565b611a51565b34801561048757600080fd5b506101f661049636600461479f565b611cfd565b3480156104a757600080fd5b506103cc611d0a565b3480156104bc57600080fd5b5060325461021d906001600160a01b031681565b3480156104dc57600080fd5b506103216104eb3660046146f6565b60336020526000908152604090205460ff1681565b34801561050c57600080fd5b5061038661051b3660046147f2565b611d91565b34801561052c57600080fd5b5061021d61053b36600461480f565b611daa565b34801561054c57600080fd5b50603f5461021d906001600160a01b031681565b34801561056c57600080fd5b506105c561057b36600461477a565b60396020908152600092835260408084209091529082529020546001600160a01b0381169074010000000000000000000000000000000000000000900467ffffffffffffffff1682565b604080516001600160a01b03909316835267ffffffffffffffff909116602083015201610231565b3480156105f957600080fd5b507f00000000000000000000000000000000000000000000000000000000000000006103cc565b34801561062c57600080fd5b50600154610688906fffffffffffffffffffffffffffffffff81169067ffffffffffffffff7001000000000000000000000000000000008204811691780100000000000000000000000000000000000000000000000090041683565b604080516fffffffffffffffffffffffffffffffff909416845267ffffffffffffffff9283166020850152911690820152606001610231565b3480156106cd57600080fd5b5061021d611de2565b6101f66106e436600461483f565b6106fe565b3480156106f557600080fd5b5061021d611e29565b8260005a905061070c611e8c565b15610749573415610749576040517fbd58e0a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610751611f0e565b156107c65734156107c657603f60009054906101000a90046001600160a01b03166001600160a01b0316631ee116bf346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156107ac57600080fd5b505af11580156107c0573d6000803e3d6000fd5b50505050505b8380156107db57506001600160a01b03871615155b15610812576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61081c8351611d91565b67ffffffffffffffff168567ffffffffffffffff161015610869576040517f70c8bdbd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6201d4c0835111156108a7576040517f5aa3bac900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336108b0611fcd565b6108cd575033731111000000000000000000000000000000001111015b600034888888886040516020016108e89594939291906148be565b60405160208183030381529060405290506000896001600160a01b0316836001600160a01b03167fb3813568d9991fc951961fcb4c784893574240a28925604d09fc577c55bb7c328460405161093e9190614767565b60405180910390a45050610952828261200a565b50505050505050565b603754604080517f35e80ab300000000000000000000000000000000000000000000000000000000815290516000926001600160a01b0316916335e80ab39160048083019260209291908290030181865afa1580156109be573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614923565b905090565b603e54604080517f3c9f397c00000000000000000000000000000000000000000000000000000000815290516000926001600160a01b031691633c9f397c9160048083019260209291908290030181865afa158015610a4a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614952565b600080610a997fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035490565b90506001600160a01b03811615610aaf57919050565b6040518060400160405280601a81526020017f4f564d5f4c3143726f7373446f6d61696e4d657373656e676572000000000000815250516002610af2919061499e565b604080513060208201526000918101919091527f4f564d5f4c3143726f7373446f6d61696e4d657373656e6765720000000000009190911790610b4d906060015b604051602081830303815290604052805190602001205490565b14610b84576040517f54e433cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408051306020820152600191810191909152600090610ba690606001610b33565b90506001600160a01b03811615610c2057806001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bf5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c199190614923565b9250505090565b6040517f332144db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610c5a6122e1565b610c62611e8c565b15610ca357606082015115610ca3576040517fbd58e0a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6032546001600160a01b031661dead14610ce9576040517fdfeaaeb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610cf68260400151612320565b15610d2d576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610d3883612349565b9050610d448183611a51565b600081815260336020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055610d83611f0e565b15610e1557606083015115610e1557603f5460608401516040517f8d445bd00000000000000000000000000000000000000000000000000000000081526001600160a01b0390921691638d445bd091610de29160040190815260200190565b600060405180830381600087803b158015610dfc57600080fd5b505af1158015610e10573d6000803e3d6000fd5b505050505b8260200151603260006101000a8154816001600160a01b0302191690836001600160a01b031602179055506000610e5e8460400151856080015186606001518760a00151612396565b603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead17905560405190915082907fdb5c7652857aa163daadd670e116628fb42e869d8ac4251ef8971d9e5727df1b90610ec390841515815260200190565b60405180910390a2610ed3611f0e565b15610f5d5780158015610eea575060008460600151115b15610f5d57603f60009054906101000a90046001600160a01b03166001600160a01b0316631ee116bf85606001516040518263ffffffff1660e01b81526004016000604051808303818588803b158015610f4357600080fd5b505af1158015610f57573d6000803e3d6000fd5b50505050505b80158015610f6b5750326001145b15610fa2576040517fab58103600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505050565b603754604080517f452a932000000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163452a93209160048083019260209291908290030181865afa1580156109be573d6000803e3d6000fd5b603e546040517f45884d320000000000000000000000000000000000000000000000000000000081526001600160a01b03838116600483015260009216906345884d3290602401602060405180830381865afa15801561106f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061109391906149bd565b92915050565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff161580156110d9575060005460ff8083169116105b6111505760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a656400000000000000000000000000000000000060648201526084015b60405180910390fd5b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff8316176101001790556111896123f4565b603780546001600160a01b038086167fffffffffffffffffffffffff000000000000000000000000000000000000000092831617909255603e8054928516929091169190911790556111d961245b565b6032546001600160a01b031661121657603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead1790555b61121e61260e565b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b6112896122e1565b6112968560400151612320565b156112cd576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6112d5611e8c565b1561131657606085015115611316576040517fbd58e0a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611320611e29565b6001600160a01b031663bb8aa1fc866040518263ffffffff1660e01b815260040161134d91815260200190565b606060405180830381865afa15801561136a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061138e91906149da565b603e546040517f496b9c160000000000000000000000000000000000000000000000000000000081526001600160a01b0380841660048301529295509116925063496b9c169150602401602060405180830381865afa1580156113f5573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061141991906149bd565b61144f576040517ff395240e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f04e50fed0000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152909116906304e50fed90602401602060405180830381865afa1580156114b2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114d691906149bd565b61150c576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001816001600160a01b031663200d2ed26040518163ffffffff1660e01b8152600401602060405180830381865afa15801561154c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115709190614a56565b600281111561158157611581614a27565b036115b8576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61162a816001600160a01b031663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115f9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061161d9190614a77565b67ffffffffffffffff1690565b67ffffffffffffffff16421161166c576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61168361167e36869003860186614a94565b612707565b6116eb826001600160a01b031663bcef3b556040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116e89190614afa565b90565b14611722576040517f426149af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600061172d87612349565b9050600081600060405160200161174e929190918252602082015260400190565b60408051601f19818403018152828252805160209182012090830181905292506117c5910160408051601f19818403018152828201909152600182527f01000000000000000000000000000000000000000000000000000000000000006020830152906117bb8789614b13565b8960400135612746565b1515600003611800576040517f2e57ef3a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820182526001600160a01b03808616825267ffffffffffffffff42811660208085019182526000888152603982528681203380835290835287822096518754945190951674010000000000000000000000000000000000000000027fffffffff000000000000000000000000000000000000000000000000000000009094169486169490941792909217909455868152603c845284812080546001810182559082528482200180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169092179091558b840151928c01519351928216939091169185917f67a6208cfcc0801d50f6cbe764733f4fddf66ac0b04442061a8a8c0cb6b63f6291a4604051339083907f798f9f13695f8f045aa5f80ed8efebb695f3c7fe65da381969f2f28bf3c60b9790600090a35050505050505050565b603e54604080517f4086d18300000000000000000000000000000000000000000000000000000000815290516000926001600160a01b031691634086d1839160048083019260209291908290030181865afa1580156119a6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614a77565b603754604080517f5c975abb00000000000000000000000000000000000000000000000000000000815290516000926001600160a01b031691635c975abb9160048083019260209291908290030181865afa158015611a2d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e291906149bd565b60008281526039602090815260408083206001600160a01b0385811685529083528184208251808401845290549182168082527401000000000000000000000000000000000000000090920467ffffffffffffffff1681850152868552603390935292205490919060ff1615611af3576040517f730a107400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b816020015167ffffffffffffffff16600003611b3b576040517fcca6afda00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611b7c816001600160a01b031663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115f9573d6000803e3d6000fd5b67ffffffffffffffff16826020015167ffffffffffffffff1611611bcc576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000826020015167ffffffffffffffff1642611c079190614b97565b11611c3e576040517fd9bc01be00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f6c4f44670000000000000000000000000000000000000000000000000000000081526001600160a01b03838116600483015290911690636c4f446790602401602060405180830381865afa158015611ca1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611cc591906149bd565b610fa2576040517f332a57f800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b611d078133610c52565b50565b603e54604080517f952b279700000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163952b27979160048083019260209291908290030181865afa158015611d6d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e29190614afa565b6000611d9e826028614bae565b61109390615208614bde565b603c6020528160005260406000208181548110611dc657600080fd5b6000918252602090912001546001600160a01b03169150829050565b6000611dec610a6e565b6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109be573d6000803e3d6000fd5b603e54604080517ff2b4e61700000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163f2b4e6179160048083019260209291908290030181865afa1580156109be573d6000803e3d6000fd5b6037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f435553544f4d5f4741535f544f4b454e0000000000000000000000000000000060048201526000916001600160a01b0316906347af267b90602401602060405180830381865afa158015611a2d573d6000803e3d6000fd5b6037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f4554485f4c4f434b424f5800000000000000000000000000000000000000000060048201526000916001600160a01b0316906347af267b90602401602060405180830381865afa158015611f90573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611fb491906149bd565b80156109e2575050603f546001600160a01b0316151590565b6000323303611fdc5750600190565b333b60170361200457604051602081016040526020600082333c5160e81c62ef010014905090565b50600090565b600154600090612040907801000000000000000000000000000000000000000000000000900467ffffffffffffffff1643614b97565b9050600061204c61276a565b90506000816020015160ff16826000015163ffffffff1661206d9190614c39565b905082156121a4576001546000906120a4908390700100000000000000000000000000000000900467ffffffffffffffff16614c83565b90506000836040015160ff16836120bb9190614cf7565b6001546120db9084906fffffffffffffffffffffffffffffffff16614cf7565b6120e59190614c39565b6001549091506000906121369061210f9084906fffffffffffffffffffffffffffffffff16614db3565b866060015163ffffffff168760a001516fffffffffffffffffffffffffffffffff16612823565b905060018611156121655761216261210f82876040015160ff1660018a61215d9190614b97565b612842565b90505b6fffffffffffffffffffffffffffffffff16780100000000000000000000000000000000000000000000000067ffffffffffffffff4316021760015550505b600180548691906010906121d7908490700100000000000000000000000000000000900467ffffffffffffffff16614bde565b92506101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550816000015163ffffffff16600160000160109054906101000a900467ffffffffffffffff1667ffffffffffffffff161315612264576040517f77ebef4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600154600090612290906fffffffffffffffffffffffffffffffff1667ffffffffffffffff881661499e565b905060006122a248633b9aca00612897565b6122ac9083614e27565b905060005a6122bb9088614b97565b9050808211156122d7576122d76122d28284614b97565b6128ae565b5050505050505050565b6122e96119ca565b15611cfb576040517fb9c3c2ef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006001600160a01b038216301480611093575050603f546001600160a01b0390811691161490565b80516020808301516040808501516060860151608087015160a08801519351600097612379979096959101614e3b565b604051602081830303815290604052805190602001209050919050565b60008060006123a68660006128dc565b9050806123dc576308c379a06000526020805278185361666543616c6c3a204e6f7420656e6f756768206761736058526064601cfd5b600080855160208701888b5af1979650505050505050565b336123fd610a6e565b6001600160a01b031614158015612424575033612418611de2565b6001600160a01b031614155b15611cfb576040517fc4050a2600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f4554485f4c4f434b424f5800000000000000000000000000000000000000000060048201526001600160a01b03909116906347af267b90602401602060405180830381865afa1580156124dc573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061250091906149bd565b80156125155750603f546001600160a01b0316155b806125d757506037546040517f47af267b0000000000000000000000000000000000000000000000000000000081527f4554485f4c4f434b424f5800000000000000000000000000000000000000000060048201526001600160a01b03909116906347af267b90602401602060405180830381865afa15801561259c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125c091906149bd565b1580156125d75750603f546001600160a01b031615155b15611cfb576040517f9c46cd7900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600054610100900460ff1661268b5760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401611147565b6001547801000000000000000000000000000000000000000000000000900467ffffffffffffffff16600003611cfb5760408051606081018252633b9aca00808252600060208301524367ffffffffffffffff169190920181905278010000000000000000000000000000000000000000000000000217600155565b60008160000151826020015183604001518460600151604051602001612379949392919093845260208401929092526040830152606082015260800190565b600080612752866128fa565b90506127608186868661292c565b9695505050505050565b6040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a0810191909152603754604080517fcc731b0200000000000000000000000000000000000000000000000000000000815290516000926001600160a01b03169163cc731b029160048083019260c09291908290030181865afa1580156127ff573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110939190614e9b565b6000612838612832858561295c565b8361296c565b90505b9392505050565b6000670de0b6b3a764000061288361285a8583614c39565b61286c90670de0b6b3a7640000614c83565b61287e85670de0b6b3a7640000614cf7565b61297b565b61288d9086614cf7565b6128389190614c39565b6000818310156128a7578161283b565b5090919050565b6000805a90505b825a6128c19083614b97565b10156128d7576128d082614f57565b91506128b5565b505050565b600080603f83619c4001026040850201603f5a021015949350505050565b6060818051906020012060405160200161291691815260200190565b6040516020818303038152906040529050919050565b60006129538461293d8786866129ac565b8051602091820120825192909101919091201490565b95945050505050565b6000818312156128a7578161283b565b60008183126128a7578161283b565b600061283b670de0b6b3a764000083612993866132be565b61299d9190614cf7565b6129a79190614c39565b6134e8565b606060008451116129ff5760405162461bcd60e51b815260206004820152601560248201527f4d65726b6c65547269653a20656d707479206b657900000000000000000000006044820152606401611147565b6000612a0a8461370d565b90506000612a17866137f9565b9050600084604051602001612a2e91815260200190565b60405160208183030381529060405290506000805b845181101561324f576000858281518110612a6057612a60614f71565b602002602001015190508451831115612ae15760405162461bcd60e51b815260206004820152602e60248201527f4d65726b6c65547269653a206b657920696e646578206578636565647320746f60448201527f74616c206b6579206c656e6774680000000000000000000000000000000000006064820152608401611147565b82600003612b805780518051602091820120604051612b2f92612b0992910190815260200190565b604051602081830303815290604052858051602091820120825192909101919091201490565b612b7b5760405162461bcd60e51b815260206004820152601d60248201527f4d65726b6c65547269653a20696e76616c696420726f6f7420686173680000006044820152606401611147565b612ca3565b805151602011612c1c5780518051602091820120604051612baa92612b0992910190815260200190565b612b7b5760405162461bcd60e51b815260206004820152602760248201527f4d65726b6c65547269653a20696e76616c6964206c6172676520696e7465726e60448201527f616c2068617368000000000000000000000000000000000000000000000000006064820152608401611147565b805184516020808701919091208251919092012014612ca35760405162461bcd60e51b815260206004820152602660248201527f4d65726b6c65547269653a20696e76616c696420696e7465726e616c206e6f6460448201527f65206861736800000000000000000000000000000000000000000000000000006064820152608401611147565b612caf60106001614fa0565b81602001515103612e575784518303612def57612ce98160200151601081518110612cdc57612cdc614f71565b602002602001015161385c565b96506000875111612d625760405162461bcd60e51b815260206004820152603b60248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286272616e63682900000000006064820152608401611147565b60018651612d709190614b97565b8214612de45760405162461bcd60e51b815260206004820152603a60248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286272616e6368290000000000006064820152608401611147565b50505050505061283b565b6000858481518110612e0357612e03614f71565b602001015160f81c60f81b60f81c9050600082602001518260ff1681518110612e2e57612e2e614f71565b60200260200101519050612e4181613910565b9550612e4e600186614fa0565b9450505061323c565b6002816020015151036131ce576000612e6f82613935565b9050600081600081518110612e8657612e86614f71565b016020015160f81c90506000612e9d600283614fb8565b612ea8906002614fda565b90506000612eb9848360ff16613959565b90506000612ec78a89613959565b90506000612ed5838361398f565b905080835114612f4d5760405162461bcd60e51b815260206004820152603a60248201527f4d65726b6c65547269653a20706174682072656d61696e646572206d7573742060448201527f736861726520616c6c206e6962626c65732077697468206b65790000000000006064820152608401611147565b60ff851660021480612f62575060ff85166003145b156131035780825114612fdd5760405162461bcd60e51b815260206004820152603d60248201527f4d65726b6c65547269653a206b65792072656d61696e646572206d757374206260448201527f65206964656e746963616c20746f20706174682072656d61696e6465720000006064820152608401611147565b612ff78760200151600181518110612cdc57612cdc614f71565b9c5060008d51116130705760405162461bcd60e51b815260206004820152603960248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286c65616629000000000000006064820152608401611147565b60018c5161307e9190614b97565b88146130f25760405162461bcd60e51b815260206004820152603860248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286c6561662900000000000000006064820152608401611147565b50505050505050505050505061283b565b60ff85161580613116575060ff85166001145b1561315557613142876020015160018151811061313557613135614f71565b6020026020010151613910565b995061314e818a614fa0565b98506131c3565b60405162461bcd60e51b815260206004820152603260248201527f4d65726b6c65547269653a2072656365697665642061206e6f6465207769746860448201527f20616e20756e6b6e6f776e2070726566697800000000000000000000000000006064820152608401611147565b50505050505061323c565b60405162461bcd60e51b815260206004820152602860248201527f4d65726b6c65547269653a20726563656976656420616e20756e70617273656160448201527f626c65206e6f64650000000000000000000000000000000000000000000000006064820152608401611147565b508061324781614f57565b915050612a43565b5060405162461bcd60e51b815260206004820152602560248201527f4d65726b6c65547269653a2072616e206f7574206f662070726f6f6620656c6560448201527f6d656e74730000000000000000000000000000000000000000000000000000006064820152608401611147565b600080821361330f5760405162461bcd60e51b815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611147565b6000606061331c84613a43565b03609f8181039490941b90931c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b393909302929092017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdb731c958f34d94c1821361351957506000919050565b680755bf798b4a1bf1e582126135715760405162461bcd60e51b815260206004820152600c60248201527f4558505f4f564552464c4f5700000000000000000000000000000000000000006044820152606401611147565b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b80516060908067ffffffffffffffff81111561372b5761372b6143e9565b60405190808252806020026020018201604052801561377057816020015b60408051808201909152606080825260208201528152602001906001900390816137495790505b50915060005b818110156137f257604051806040016040528085838151811061379b5761379b614f71565b602002602001015181526020016137ca8684815181106137bd576137bd614f71565b6020026020010151613aff565b8152508382815181106137df576137df614f71565b6020908102919091010152600101613776565b5050919050565b606080604051905082518060011b603f8101601f1916830160405280835250602084016020830160005b83811015613851578060011b82018184015160001a8060041c8253600f811660018301535050600101613823565b509295945050505050565b6060600080600061386c85613b12565b91945092509050600081600181111561388757613887614a27565b146138be576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6138c88284614fa0565b855114613901576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61295385602001518484613fb0565b6060602082600001511061392c576139278261385c565b611093565b61109382614044565b60606110936139548360200151600081518110612cdc57612cdc614f71565b6137f9565b6060825182106139785750604080516020810190915260008152611093565b61283b838384865161398a9190614b97565b61405a565b60008082518451106139a25782516139a5565b83515b90505b8082108015613a2c57508282815181106139c4576139c4614f71565b602001015160f81c60f81b7effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916848381518110613a0357613a03614f71565b01602001517fff0000000000000000000000000000000000000000000000000000000000000016145b15613a3c578160010191506139a8565b5092915050565b6000808211613a945760405162461bcd60e51b815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611147565b5060016fffffffffffffffffffffffffffffffff821160071b82811c67ffffffffffffffff1060061b1782811c63ffffffff1060051b1782811c61ffff1060041b1782811c60ff10600390811b90911783811c600f1060021b1783811c909110821b1791821c111790565b6060611093613b0d836141c6565b614233565b60008060008360000151600003613b55576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f8111613b7a576000600160009450945094505050613fa9565b60b78111613c90576000613b8f608083614b97565b905080876000015111613bce576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff00000000000000000000000000000000000000000000000000000000000000169082148015613c4657507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b15613c7d576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5060019550935060009250613fa9915050565b60bf8111613dee576000613ca560b783614b97565b905080876000015111613ce4576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003613d46576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111613d8e576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613d988184614fa0565b895111613dd1576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613ddc836001614fa0565b9750955060009450613fa99350505050565b60f78111613e53576000613e0360c083614b97565b905080876000015111613e42576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600195509350849250613fa9915050565b6000613e6060f783614b97565b905080876000015111613e9f576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003613f01576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111613f49576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613f538184614fa0565b895111613f8c576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b613f97836001614fa0565b9750955060019450613fa99350505050565b9193909250565b60608167ffffffffffffffff811115613fcb57613fcb6143e9565b6040519080825280601f01601f191660200182016040528015613ff5576020820181803683370190505b509050811561283b57600061400a8486614fa0565b90506020820160005b8481101561402b578281015182820152602001614013565b8481111561403a576000858301525b5050509392505050565b6060611093826020015160008460000151613fb0565b60608182601f0110156140af5760405162461bcd60e51b815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611147565b8282840110156141015760405162461bcd60e51b815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611147565b818301845110156141545760405162461bcd60e51b815260206004820152601160248201527f736c6963655f6f75744f66426f756e64730000000000000000000000000000006044820152606401611147565b60608215801561417357604051915060008252602082016040526141bd565b6040519150601f8416801560200281840101858101878315602002848b0101015b818310156141ac578051835260209283019201614194565b5050858452601f01601f1916604052505b50949350505050565b60408051808201909152600080825260208201528151600003614215576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b6060600080600061424385613b12565b91945092509050600181600181111561425e5761425e614a27565b14614295576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b84516142a18385614fa0565b146142d8576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b60408051808201909152600080825260208201528152602001906001900390816142ef5790505093506000835b86518110156143dd576000806143626040518060400160405280858c600001516143469190614b97565b8152602001858c6020015161435b9190614fa0565b9052613b12565b50915091506040518060400160405280838361437e9190614fa0565b8152602001848b602001516143939190614fa0565b8152508885815181106143a8576143a8614f71565b60209081029190910101526143be600185614fa0565b93506143ca8183614fa0565b6143d49084614fa0565b9250505061431c565b50845250919392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715614441576144416143e9565b604052919050565b6001600160a01b0381168114611d0757600080fd5b600082601f83011261446f57600080fd5b813567ffffffffffffffff811115614489576144896143e9565b61449c6020601f19601f84011601614418565b8181528460208386010111156144b157600080fd5b816020850160208301376000918101602001919091529392505050565b600060c082840312156144e057600080fd5b60405160c0810167ffffffffffffffff8282108183111715614504576145046143e9565b81604052829350843583526020850135915061451f82614449565b8160208401526040850135915061453582614449565b816040840152606085013560608401526080850135608084015260a085013591508082111561456357600080fd5b506145708582860161445e565b60a0830152505092915050565b6000806040838503121561459057600080fd5b823567ffffffffffffffff8111156145a757600080fd5b6145b3858286016144ce565b92505060208301356145c481614449565b809150509250929050565b6000602082840312156145e157600080fd5b813561283b81614449565b600080604083850312156145ff57600080fd5b823561460a81614449565b915060208301356145c481614449565b600080600080600085870360e081121561463357600080fd5b863567ffffffffffffffff8082111561464b57600080fd5b6146578a838b016144ce565b97506020890135965060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08401121561469057600080fd5b60408901955060c08901359250808311156146aa57600080fd5b828901925089601f8401126146be57600080fd5b82359150808211156146cf57600080fd5b508860208260051b84010111156146e557600080fd5b959894975092955050506020019190565b60006020828403121561470857600080fd5b5035919050565b60005b8381101561472a578181015183820152602001614712565b83811115610fa25750506000910152565b6000815180845261475381602086016020860161470f565b601f01601f19169290920160200192915050565b60208152600061283b602083018461473b565b6000806040838503121561478d57600080fd5b8235915060208301356145c481614449565b6000602082840312156147b157600080fd5b813567ffffffffffffffff8111156147c857600080fd5b6147d4848285016144ce565b949350505050565b67ffffffffffffffff81168114611d0757600080fd5b60006020828403121561480457600080fd5b813561283b816147dc565b6000806040838503121561482257600080fd5b50508035926020909101359150565b8015158114611d0757600080fd5b600080600080600060a0868803121561485757600080fd5b853561486281614449565b9450602086013593506040860135614879816147dc565b9250606086013561488981614831565b9150608086013567ffffffffffffffff8111156148a557600080fd5b6148b18882890161445e565b9150509295509295909350565b8581528460208201527fffffffffffffffff0000000000000000000000000000000000000000000000008460c01b16604082015282151560f81b60488201526000825161491281604985016020870161470f565b919091016049019695505050505050565b60006020828403121561493557600080fd5b815161283b81614449565b63ffffffff81168114611d0757600080fd5b60006020828403121561496457600080fd5b815161283b81614940565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008160001904831182151516156149b8576149b861496f565b500290565b6000602082840312156149cf57600080fd5b815161283b81614831565b6000806000606084860312156149ef57600080fd5b83516149fa81614940565b6020850151909350614a0b816147dc565b6040850151909250614a1c81614449565b809150509250925092565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b600060208284031215614a6857600080fd5b81516003811061283b57600080fd5b600060208284031215614a8957600080fd5b815161283b816147dc565b600060808284031215614aa657600080fd5b6040516080810181811067ffffffffffffffff82111715614ac957614ac96143e9565b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b600060208284031215614b0c57600080fd5b5051919050565b600067ffffffffffffffff80841115614b2e57614b2e6143e9565b8360051b6020614b3f818301614418565b868152918501918181019036841115614b5757600080fd5b865b84811015614b8b57803586811115614b715760008081fd5b614b7d36828b0161445e565b845250918301918301614b59565b50979650505050505050565b600082821015614ba957614ba961496f565b500390565b600067ffffffffffffffff80831681851681830481118215151615614bd557614bd561496f565b02949350505050565b600067ffffffffffffffff808316818516808303821115614c0157614c0161496f565b01949350505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b600082614c4857614c48614c0a565b60001983147f800000000000000000000000000000000000000000000000000000000000000083141615614c7e57614c7e61496f565b500590565b6000808312837f800000000000000000000000000000000000000000000000000000000000000001831281151615614cbd57614cbd61496f565b837f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff018313811615614cf157614cf161496f565b50500390565b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615614d3857614d3861496f565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615614d7357614d7361496f565b60008712925087820587128484161615614d8f57614d8f61496f565b87850587128184161615614da557614da561496f565b505050929093029392505050565b6000808212827f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03841381151615614ded57614ded61496f565b827f8000000000000000000000000000000000000000000000000000000000000000038412811615614e2157614e2161496f565b50500190565b600082614e3657614e36614c0a565b500490565b86815260006001600160a01b03808816602084015280871660408401525084606083015283608083015260c060a0830152614e7960c083018461473b565b98975050505050505050565b805160ff81168114614e9657600080fd5b919050565b600060c08284031215614ead57600080fd5b60405160c0810181811067ffffffffffffffff82111715614ed057614ed06143e9565b6040528251614ede81614940565b8152614eec60208401614e85565b6020820152614efd60408401614e85565b60408201526060830151614f1081614940565b60608201526080830151614f2381614940565b608082015260a08301516fffffffffffffffffffffffffffffffff81168114614f4b57600080fd5b60a08201529392505050565b60006000198203614f6a57614f6a61496f565b5060010190565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b60008219821115614fb357614fb361496f565b500190565b600060ff831680614fcb57614fcb614c0a565b8060ff84160691505092915050565b600060ff821660ff841680821015614ff457614ff461496f565b9003939250505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xD1W`\x005`\xE0\x1C\x80cq\xC1Vn\x11a\0\xF7W\x80c\xA3\x86\x0FH\x11a\0\x95W\x80c\xCF\xF0\xAB\x96\x11a\0dW\x80c\xCF\xF0\xAB\x96\x14a\x06 W\x80c\xDA\xD5D\xE0\x14a\x06\xC1W\x80c\xE9\xE0\\B\x14a\x06\xD6W\x80c\xF2\xB4\xE6\x17\x14a\x06\xE9W`\0\x80\xFD[\x80c\xA3\x86\x0FH\x14a\x05 W\x80c\xB6\x82\xC4D\x14a\x05@W\x80c\xBB,r~\x14a\x05`W\x80c\xBFe:\\\x14a\x05\xEDW`\0\x80\xFD[\x80c\x95+'\x97\x11a\0\xD1W\x80c\x95+'\x97\x14a\x04\x9BW\x80c\x9B\xF6-\x82\x14a\x04\xB0W\x80c\xA1B8\xE7\x14a\x04\xD0W\x80c\xA3]\x99\xDF\x14a\x05\0W`\0\x80\xFD[\x80cq\xC1Vn\x14a\x04[W\x80c\x8BL@\xB0\x14a\x01\xF6W\x80c\x8C1R\xE9\x14a\x04{W`\0\x80\xFD[\x80cE\x88M2\x11a\x01oW\x80cQ7G\xAB\x11a\x01>W\x80cQ7G\xAB\x14a\x03\x9FW\x80cT\xFDMP\x14a\x03\xDAW\x80c\\\x0C\xBA3\x14a\x04&W\x80c\\\x97Z\xBB\x14a\x04FW`\0\x80\xFD[\x80cE\x88M2\x14a\x03\x01W\x80cH\\\xC9U\x14a\x031W\x80cHpIo\x14a\x03QW\x80cO\xD0CL\x14a\x03qW`\0\x80\xFD[\x80c<\x9F9|\x11a\x01\xABW\x80c<\x9F9|\x14a\x02\x8DW\x80c>G\x15\x8C\x14a\x02\xB7W\x80cC\xCA\x1CP\x14a\x02\xCCW\x80cE*\x93 \x14a\x02\xECW`\0\x80\xFD[\x80c3\xD7\xE2\xBD\x14a\x01\xFDW\x80c5\xE8\n\xB3\x14a\x02:W\x80c8\xD3\x8C\x97\x14a\x02OW`\0\x80\xFD[6a\x01\xF8Wa\x01\xF634b\x01\x86\xA0`\0`@Q\x80` \x01`@R\x80`\0\x81RPa\x06\xFEV[\0[`\0\x80\xFD[4\x80\x15a\x02\tW`\0\x80\xFD[P`7Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02FW`\0\x80\xFD[Pa\x02\x1Da\t[V[4\x80\x15a\x02[W`\0\x80\xFD[P`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x021V[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xA2a\t\xE7V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x021V[4\x80\x15a\x02\xC3W`\0\x80\xFD[Pa\x02\x1Da\nnV[4\x80\x15a\x02\xD8W`\0\x80\xFD[Pa\x01\xF6a\x02\xE76`\x04aE}V[a\x0CRV[4\x80\x15a\x02\xF8W`\0\x80\xFD[Pa\x02\x1Da\x0F\xA8V[4\x80\x15a\x03\rW`\0\x80\xFD[Pa\x03!a\x03\x1C6`\x04aE\xCFV[a\x10\x0BV[`@Q\x90\x15\x15\x81R` \x01a\x021V[4\x80\x15a\x03=W`\0\x80\xFD[Pa\x01\xF6a\x03L6`\x04aE\xECV[a\x10\x99V[4\x80\x15a\x03]W`\0\x80\xFD[Pa\x01\xF6a\x03l6`\x04aF\x1AV[a\x12\x81V[4\x80\x15a\x03}W`\0\x80\xFD[Pa\x03\x86a\x19CV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x021V[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x03\xCCa\x03\xBA6`\x04aF\xF6V[`\0\x90\x81R`<` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x021V[4\x80\x15a\x03\xE6W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\x05\x81R\x7F5.2.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x021\x91\x90aGgV[4\x80\x15a\x042W`\0\x80\xFD[P`>Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04RW`\0\x80\xFD[Pa\x03!a\x19\xCAV[4\x80\x15a\x04gW`\0\x80\xFD[Pa\x01\xF6a\x04v6`\x04aGzV[a\x1AQV[4\x80\x15a\x04\x87W`\0\x80\xFD[Pa\x01\xF6a\x04\x966`\x04aG\x9FV[a\x1C\xFDV[4\x80\x15a\x04\xA7W`\0\x80\xFD[Pa\x03\xCCa\x1D\nV[4\x80\x15a\x04\xBCW`\0\x80\xFD[P`2Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xDCW`\0\x80\xFD[Pa\x03!a\x04\xEB6`\x04aF\xF6V[`3` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\x0CW`\0\x80\xFD[Pa\x03\x86a\x05\x1B6`\x04aG\xF2V[a\x1D\x91V[4\x80\x15a\x05,W`\0\x80\xFD[Pa\x02\x1Da\x05;6`\x04aH\x0FV[a\x1D\xAAV[4\x80\x15a\x05LW`\0\x80\xFD[P`?Ta\x02\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05lW`\0\x80\xFD[Pa\x05\xC5a\x05{6`\x04aGzV[`9` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x81\x16\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x021V[4\x80\x15a\x05\xF9W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xCCV[4\x80\x15a\x06,W`\0\x80\xFD[P`\x01Ta\x06\x88\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x021V[4\x80\x15a\x06\xCDW`\0\x80\xFD[Pa\x02\x1Da\x1D\xE2V[a\x01\xF6a\x06\xE46`\x04aH?V[a\x06\xFEV[4\x80\x15a\x06\xF5W`\0\x80\xFD[Pa\x02\x1Da\x1E)V[\x82`\0Z\x90Pa\x07\x0Ca\x1E\x8CV[\x15a\x07IW4\x15a\x07IW`@Q\x7F\xBDX\xE0\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07Qa\x1F\x0EV[\x15a\x07\xC6W4\x15a\x07\xC6W`?`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xE1\x16\xBF4`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x07\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xC0W=`\0\x80>=`\0\xFD[PPPPP[\x83\x80\x15a\x07\xDBWP`\x01`\x01`\xA0\x1B\x03\x87\x16\x15\x15[\x15a\x08\x12W`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x1C\x83Qa\x1D\x91V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x08iW`@Q\x7Fp\xC8\xBD\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x01\xD4\xC0\x83Q\x11\x15a\x08\xA7W`@Q\x7FZ\xA3\xBA\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x08\xB0a\x1F\xCDV[a\x08\xCDWP3s\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\x01[`\x004\x88\x88\x88\x88`@Q` \x01a\x08\xE8\x95\x94\x93\x92\x91\x90aH\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB3\x815h\xD9\x99\x1F\xC9Q\x96\x1F\xCBLxH\x93WB@\xA2\x89%`M\t\xFCW|U\xBB|2\x84`@Qa\t>\x91\x90aGgV[`@Q\x80\x91\x03\x90\xA4PPa\tR\x82\x82a \nV[PPPPPPPV[`7T`@\x80Q\x7F5\xE8\n\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c5\xE8\n\xB3\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aI#V[\x90P\x90V[`>T`@\x80Q\x7F<\x9F9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c<\x9F9|\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\nJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aIRV[`\0\x80a\n\x99\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\n\xAFW\x91\x90PV[`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x81RPQ`\x02a\n\xF2\x91\x90aI\x9EV[`@\x80Q0` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x91\x90\x91\x17\x90a\x0BM\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 T\x90V[\x14a\x0B\x84W`@Q\x7FT\xE43\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R`\0\x90a\x0B\xA6\x90``\x01a\x0B3V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0C W\x80`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x19\x91\x90aI#V[\x92PPP\x90V[`@Q\x7F3!D\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0CZa\"\xE1V[a\x0Cba\x1E\x8CV[\x15a\x0C\xA3W``\x82\x01Q\x15a\x0C\xA3W`@Q\x7F\xBDX\xE0\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`2T`\x01`\x01`\xA0\x1B\x03\x16a\xDE\xAD\x14a\x0C\xE9W`@Q\x7F\xDF\xEA\xAE\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xF6\x82`@\x01Qa# V[\x15a\r-W`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\r8\x83a#IV[\x90Pa\rD\x81\x83a\x1AQV[`\0\x81\x81R`3` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90Ua\r\x83a\x1F\x0EV[\x15a\x0E\x15W``\x83\x01Q\x15a\x0E\x15W`?T``\x84\x01Q`@Q\x7F\x8DD[\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8DD[\xD0\x91a\r\xE2\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x10W=`\0\x80>=`\0\xFD[PPPP[\x82` \x01Q`2`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\0a\x0E^\x84`@\x01Q\x85`\x80\x01Q\x86``\x01Q\x87`\xA0\x01Qa#\x96V[`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U`@Q\x90\x91P\x82\x90\x7F\xDB\\vR\x85z\xA1c\xDA\xAD\xD6p\xE1\x16b\x8F\xB4.\x86\x9D\x8A\xC4%\x1E\xF8\x97\x1D\x9EW'\xDF\x1B\x90a\x0E\xC3\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x0E\xD3a\x1F\x0EV[\x15a\x0F]W\x80\x15\x80\x15a\x0E\xEAWP`\0\x84``\x01Q\x11[\x15a\x0F]W`?`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xE1\x16\xBF\x85``\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x0FCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0FWW=`\0\x80>=`\0\xFD[PPPPP[\x80\x15\x80\x15a\x0FkWP2`\x01\x14[\x15a\x0F\xA2W`@Q\x7F\xABX\x106\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`7T`@\x80Q\x7FE*\x93 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cE*\x93 \x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[`>T`@Q\x7FE\x88M2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x92\x16\x90cE\x88M2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x93\x91\x90aI\xBDV[\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x10\xD9WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x11PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x11\x89a#\xF4V[`7\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`>\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90Ua\x11\xD9a$[V[`2T`\x01`\x01`\xA0\x1B\x03\x16a\x12\x16W`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U[a\x12\x1Ea&\x0EV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[a\x12\x89a\"\xE1V[a\x12\x96\x85`@\x01Qa# V[\x15a\x12\xCDW`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xD5a\x1E\x8CV[\x15a\x13\x16W``\x85\x01Q\x15a\x13\x16W`@Q\x7F\xBDX\xE0\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13 a\x1E)V[`\x01`\x01`\xA0\x1B\x03\x16c\xBB\x8A\xA1\xFC\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13M\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x8E\x91\x90aI\xDAV[`>T`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x95P\x91\x16\x92PcIk\x9C\x16\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x19\x91\x90aI\xBDV[a\x14OW`@Q\x7F\xF3\x95$\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7F\x04\xE5\x0F\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\x04\xE5\x0F\xED\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xD6\x91\x90aI\xBDV[a\x15\x0CW`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01`\x01`\xA0\x1B\x03\x16c \r.\xD2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15p\x91\x90aJVV[`\x02\x81\x11\x15a\x15\x81Wa\x15\x81aJ'V[\x03a\x15\xB8W`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16*\x81`\x01`\x01`\xA0\x1B\x03\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x1D\x91\x90aJwV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11a\x16lW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x83a\x16~6\x86\x90\x03\x86\x01\x86aJ\x94V[a'\x07V[a\x16\xEB\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE8\x91\x90aJ\xFAV[\x90V[\x14a\x17\"W`@Q\x7FBaI\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x17-\x87a#IV[\x90P`\0\x81`\0`@Q` \x01a\x17N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92Pa\x17\xC5\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82\x01\x90\x91R`\x01\x82R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01R\x90a\x17\xBB\x87\x89aK\x13V[\x89`@\x015a'FV[\x15\x15`\0\x03a\x18\0W`@Q\x7F.W\xEF:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16` \x80\x85\x01\x91\x82R`\0\x88\x81R`9\x82R\x86\x81 3\x80\x83R\x90\x83R\x87\x82 \x96Q\x87T\x94Q\x90\x95\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x94\x86\x16\x94\x90\x94\x17\x92\x90\x92\x17\x90\x94U\x86\x81R`<\x84R\x84\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x84\x82 \x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x92\x17\x90\x91U\x8B\x84\x01Q\x92\x8C\x01Q\x93Q\x92\x82\x16\x93\x90\x91\x16\x91\x85\x91\x7Fg\xA6 \x8C\xFC\xC0\x80\x1DP\xF6\xCB\xE7ds?O\xDD\xF6j\xC0\xB0DB\x06\x1A\x8A\x8C\x0C\xB6\xB6?b\x91\xA4`@Q3\x90\x83\x90\x7Fy\x8F\x9F\x13i_\x8F\x04Z\xA5\xF8\x0E\xD8\xEF\xEB\xB6\x95\xF3\xC7\xFEe\xDA8\x19i\xF2\xF2\x8B\xF3\xC6\x0B\x97\x90`\0\x90\xA3PPPPPPPPV[`>T`@\x80Q\x7F@\x86\xD1\x83\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c@\x86\xD1\x83\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x19\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aJwV[`7T`@\x80Q\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\\\x97Z\xBB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aI\xBDV[`\0\x82\x81R`9` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x85R\x90\x83R\x81\x84 \x82Q\x80\x84\x01\x84R\x90T\x91\x82\x16\x80\x82Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x85\x01R\x86\x85R`3\x90\x93R\x92 T\x90\x91\x90`\xFF\x16\x15a\x1A\xF3W`@Q\x7Fs\n\x10t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B;W`@Q\x7F\xCC\xA6\xAF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B|\x81`\x01`\x01`\xA0\x1B\x03\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xF9W=`\0\x80>=`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1B\xCCW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x1C\x07\x91\x90aK\x97V[\x11a\x1C>W`@Q\x7F\xD9\xBC\x01\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7FlODg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90clODg\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xC5\x91\x90aI\xBDV[a\x0F\xA2W`@Q\x7F3*W\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x1D\x07\x813a\x0CRV[PV[`>T`@\x80Q\x7F\x95+'\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95+'\x97\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1DmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE2\x91\x90aJ\xFAV[`\0a\x1D\x9E\x82`(aK\xAEV[a\x10\x93\x90aR\x08aK\xDEV[`<` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x1D\xC6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`\0a\x1D\xECa\nnV[`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[`>T`@\x80Q\x7F\xF2\xB4\xE6\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2\xB4\xE6\x17\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\t\xBEW=`\0\x80>=`\0\xFD[`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FCUSTOM_GAS_TOKEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A-W=`\0\x80>=`\0\xFD[`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xB4\x91\x90aI\xBDV[\x80\x15a\t\xE2WPP`?T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15\x90V[`\x0023\x03a\x1F\xDCWP`\x01\x90V[3;`\x17\x03a \x04W`@Q` \x81\x01`@R` `\0\x823<Q`\xE8\x1Cb\xEF\x01\0\x14\x90P\x90V[P`\0\x90V[`\x01T`\0\x90a @\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16CaK\x97V[\x90P`\0a La'jV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a m\x91\x90aL9V[\x90P\x82\x15a!\xA4W`\x01T`\0\x90a \xA4\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aL\x83V[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a \xBB\x91\x90aL\xF7V[`\x01Ta \xDB\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aL\xF7V[a \xE5\x91\x90aL9V[`\x01T\x90\x91P`\0\x90a!6\x90a!\x0F\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aM\xB3V[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(#V[\x90P`\x01\x86\x11\x15a!eWa!ba!\x0F\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa!]\x91\x90aK\x97V[a(BV[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a!\xD7\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aK\xDEV[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\"dW`@Q\x7Fw\xEB\xEFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\0\x90a\"\x90\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16aI\x9EV[\x90P`\0a\"\xA2Hc;\x9A\xCA\0a(\x97V[a\"\xAC\x90\x83aN'V[\x90P`\0Za\"\xBB\x90\x88aK\x97V[\x90P\x80\x82\x11\x15a\"\xD7Wa\"\xD7a\"\xD2\x82\x84aK\x97V[a(\xAEV[PPPPPPPPV[a\"\xE9a\x19\xCAV[\x15a\x1C\xFBW`@Q\x7F\xB9\xC3\xC2\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x80a\x10\x93WPP`?T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a#y\x97\x90\x96\x95\x91\x01aN;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a#\xA6\x86`\0a(\xDCV[\x90P\x80a#\xDCWc\x08\xC3y\xA0`\0R` \x80Rx\x18SafeCall: Not enough gas`XR`d`\x1C\xFD[`\0\x80\x85Q` \x87\x01\x88\x8BZ\xF1\x97\x96PPPPPPPV[3a#\xFDa\nnV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a$$WP3a$\x18a\x1D\xE2V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x1C\xFBW`@Q\x7F\xC4\x05\n&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\0\x91\x90aI\xBDV[\x80\x15a%\x15WP`?T`\x01`\x01`\xA0\x1B\x03\x16\x15[\x80a%\xD7WP`7T`@Q\x7FG\xAF&{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FETH_LOCKBOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cG\xAF&{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xC0\x91\x90aI\xBDV[\x15\x80\x15a%\xD7WP`?T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[\x15a\x1C\xFBW`@Q\x7F\x9CF\xCDy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16a&\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\x01Tx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x1C\xFBW`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90\x92\x01\x81\x90Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x17`\x01UV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a#y\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`\0\x80a'R\x86a(\xFAV[\x90Pa'`\x81\x86\x86\x86a),V[\x96\x95PPPPPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R`7T`@\x80Q\x7F\xCCs\x1B\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xCCs\x1B\x02\x91`\x04\x80\x83\x01\x92`\xC0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x93\x91\x90aN\x9BV[`\0a(8a(2\x85\x85a)\\V[\x83a)lV[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a(\x83a(Z\x85\x83aL9V[a(l\x90g\r\xE0\xB6\xB3\xA7d\0\0aL\x83V[a(~\x85g\r\xE0\xB6\xB3\xA7d\0\0aL\xF7V[a){V[a(\x8D\x90\x86aL\xF7V[a(8\x91\x90aL9V[`\0\x81\x83\x10\x15a(\xA7W\x81a(;V[P\x90\x91\x90PV[`\0\x80Z\x90P[\x82Za(\xC1\x90\x83aK\x97V[\x10\x15a(\xD7Wa(\xD0\x82aOWV[\x91Pa(\xB5V[PPPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[``\x81\x80Q\x90` \x01 `@Q` \x01a)\x16\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a)S\x84a)=\x87\x86\x86a)\xACV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[\x95\x94PPPPPV[`\0\x81\x83\x12\x15a(\xA7W\x81a(;V[`\0\x81\x83\x12a(\xA7W\x81a(;V[`\0a(;g\r\xE0\xB6\xB3\xA7d\0\0\x83a)\x93\x86a2\xBEV[a)\x9D\x91\x90aL\xF7V[a)\xA7\x91\x90aL9V[a4\xE8V[```\0\x84Q\x11a)\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FMerkleTrie: empty key\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[`\0a*\n\x84a7\rV[\x90P`\0a*\x17\x86a7\xF9V[\x90P`\0\x84`@Q` \x01a*.\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80[\x84Q\x81\x10\x15a2OW`\0\x85\x82\x81Q\x81\x10a*`Wa*`aOqV[` \x02` \x01\x01Q\x90P\x84Q\x83\x11\x15a*\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FMerkleTrie: key index exceeds to`D\x82\x01R\x7Ftal key length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[\x82`\0\x03a+\x80W\x80Q\x80Q` \x91\x82\x01 `@Qa+/\x92a+\t\x92\x91\x01\x90\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[a+{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FMerkleTrie: invalid root hash\0\0\0`D\x82\x01R`d\x01a\x11GV[a,\xA3V[\x80QQ` \x11a,\x1CW\x80Q\x80Q` \x91\x82\x01 `@Qa+\xAA\x92a+\t\x92\x91\x01\x90\x81R` \x01\x90V[a+{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FMerkleTrie: invalid large intern`D\x82\x01R\x7Fal hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[\x80Q\x84Q` \x80\x87\x01\x91\x90\x91 \x82Q\x91\x90\x92\x01 \x14a,\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FMerkleTrie: invalid internal nod`D\x82\x01R\x7Fe hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[a,\xAF`\x10`\x01aO\xA0V[\x81` \x01QQ\x03a.WW\x84Q\x83\x03a-\xEFWa,\xE9\x81` \x01Q`\x10\x81Q\x81\x10a,\xDCWa,\xDCaOqV[` \x02` \x01\x01Qa8\\V[\x96P`\0\x87Q\x11a-bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (branch)\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\x01\x86Qa-p\x91\x90aK\x97V[\x82\x14a-\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (branch)\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[PPPPPPa(;V[`\0\x85\x84\x81Q\x81\x10a.\x03Wa.\x03aOqV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\0\x82` \x01Q\x82`\xFF\x16\x81Q\x81\x10a..Wa..aOqV[` \x02` \x01\x01Q\x90Pa.A\x81a9\x10V[\x95Pa.N`\x01\x86aO\xA0V[\x94PPPa2<V[`\x02\x81` \x01QQ\x03a1\xCEW`\0a.o\x82a95V[\x90P`\0\x81`\0\x81Q\x81\x10a.\x86Wa.\x86aOqV[\x01` \x01Q`\xF8\x1C\x90P`\0a.\x9D`\x02\x83aO\xB8V[a.\xA8\x90`\x02aO\xDAV[\x90P`\0a.\xB9\x84\x83`\xFF\x16a9YV[\x90P`\0a.\xC7\x8A\x89a9YV[\x90P`\0a.\xD5\x83\x83a9\x8FV[\x90P\x80\x83Q\x14a/MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: path remainder must `D\x82\x01R\x7Fshare all nibbles with key\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\xFF\x85\x16`\x02\x14\x80a/bWP`\xFF\x85\x16`\x03\x14[\x15a1\x03W\x80\x82Q\x14a/\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FMerkleTrie: key remainder must b`D\x82\x01R\x7Fe identical to path remainder\0\0\0`d\x82\x01R`\x84\x01a\x11GV[a/\xF7\x87` \x01Q`\x01\x81Q\x81\x10a,\xDCWa,\xDCaOqV[\x9CP`\0\x8DQ\x11a0pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (leaf)\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\x01\x8CQa0~\x91\x90aK\x97V[\x88\x14a0\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (leaf)\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[PPPPPPPPPPPPa(;V[`\xFF\x85\x16\x15\x80a1\x16WP`\xFF\x85\x16`\x01\x14[\x15a1UWa1B\x87` \x01Q`\x01\x81Q\x81\x10a15Wa15aOqV[` \x02` \x01\x01Qa9\x10V[\x99Pa1N\x81\x8AaO\xA0V[\x98Pa1\xC3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FMerkleTrie: received a node with`D\x82\x01R\x7F an unknown prefix\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[PPPPPPa2<V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMerkleTrie: received an unparsea`D\x82\x01R\x7Fble node\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[P\x80a2G\x81aOWV[\x91PPa*CV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FMerkleTrie: ran out of proof ele`D\x82\x01R\x7Fments\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11GV[`\0\x80\x82\x13a3\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[`\0``a3\x1C\x84a:CV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a5\x19WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a5qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x80Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7+Wa7+aC\xE9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7pW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a7IW\x90P[P\x91P`\0[\x81\x81\x10\x15a7\xF2W`@Q\x80`@\x01`@R\x80\x85\x83\x81Q\x81\x10a7\x9BWa7\x9BaOqV[` \x02` \x01\x01Q\x81R` \x01a7\xCA\x86\x84\x81Q\x81\x10a7\xBDWa7\xBDaOqV[` \x02` \x01\x01Qa:\xFFV[\x81RP\x83\x82\x81Q\x81\x10a7\xDFWa7\xDFaOqV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a7vV[PP\x91\x90PV[``\x80`@Q\x90P\x82Q\x80`\x01\x1B`?\x81\x01`\x1F\x19\x16\x83\x01`@R\x80\x83RP` \x84\x01` \x83\x01`\0[\x83\x81\x10\x15a8QW\x80`\x01\x1B\x82\x01\x81\x84\x01Q`\0\x1A\x80`\x04\x1C\x82S`\x0F\x81\x16`\x01\x83\x01SPP`\x01\x01a8#V[P\x92\x95\x94PPPPPV[```\0\x80`\0a8l\x85a;\x12V[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15a8\x87Wa8\x87aJ'V[\x14a8\xBEW`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a8\xC8\x82\x84aO\xA0V[\x85Q\x14a9\x01W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)S\x85` \x01Q\x84\x84a?\xB0V[``` \x82`\0\x01Q\x10a9,Wa9'\x82a8\\V[a\x10\x93V[a\x10\x93\x82a@DV[``a\x10\x93a9T\x83` \x01Q`\0\x81Q\x81\x10a,\xDCWa,\xDCaOqV[a7\xF9V[``\x82Q\x82\x10a9xWP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\x93V[a(;\x83\x83\x84\x86Qa9\x8A\x91\x90aK\x97V[a@ZV[`\0\x80\x82Q\x84Q\x10a9\xA2W\x82Qa9\xA5V[\x83Q[\x90P[\x80\x82\x10\x80\x15a:,WP\x82\x82\x81Q\x81\x10a9\xC4Wa9\xC4aOqV[` \x01\x01Q`\xF8\x1C`\xF8\x1B~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x84\x83\x81Q\x81\x10a:\x03Wa:\x03aOqV[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[\x15a:<W\x81`\x01\x01\x91Pa9\xA8V[P\x92\x91PPV[`\0\x80\x82\x11a:\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[``a\x10\x93a;\r\x83aA\xC6V[aB3V[`\0\x80`\0\x83`\0\x01Q`\0\x03a;UW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11a;zW`\0`\x01`\0\x94P\x94P\x94PPPa?\xA9V[`\xB7\x81\x11a<\x90W`\0a;\x8F`\x80\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a;\xCEW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15a<FWP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15a<}W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92Pa?\xA9\x91PPV[`\xBF\x81\x11a=\xEEW`\0a<\xA5`\xB7\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a<\xE4W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03a=FW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11a=\x8EW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\x98\x81\x84aO\xA0V[\x89Q\x11a=\xD1W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a=\xDC\x83`\x01aO\xA0V[\x97P\x95P`\0\x94Pa?\xA9\x93PPPPV[`\xF7\x81\x11a>SW`\0a>\x03`\xC0\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a>BW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92Pa?\xA9\x91PPV[`\0a>``\xF7\x83aK\x97V[\x90P\x80\x87`\0\x01Q\x11a>\x9FW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03a?\x01W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11a?IW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a?S\x81\x84aO\xA0V[\x89Q\x11a?\x8CW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a?\x97\x83`\x01aO\xA0V[\x97P\x95P`\x01\x94Pa?\xA9\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?\xCBWa?\xCBaC\xE9V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a?\xF5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15a(;W`\0a@\n\x84\x86aO\xA0V[\x90P` \x82\x01`\0[\x84\x81\x10\x15a@+W\x82\x81\x01Q\x82\x82\x01R` \x01a@\x13V[\x84\x81\x11\x15a@:W`\0\x85\x83\x01R[PPP\x93\x92PPPV[``a\x10\x93\x82` \x01Q`\0\x84`\0\x01Qa?\xB0V[``\x81\x82`\x1F\x01\x10\x15a@\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[\x82\x82\x84\x01\x10\x15aA\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[\x81\x83\x01\x84Q\x10\x15aATW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Fslice_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x11GV[``\x82\x15\x80\x15aAsW`@Q\x91P`\0\x82R` \x82\x01`@RaA\xBDV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15aA\xACW\x80Q\x83R` \x92\x83\x01\x92\x01aA\x94V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03aB\x15W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0aBC\x85a;\x12V[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15aB^WaB^aJ'V[\x14aB\x95W`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84QaB\xA1\x83\x85aO\xA0V[\x14aB\xD8W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81aB\xEFW\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15aC\xDDW`\0\x80aCb`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01QaCF\x91\x90aK\x97V[\x81R` \x01\x85\x8C` \x01QaC[\x91\x90aO\xA0V[\x90Ra;\x12V[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83aC~\x91\x90aO\xA0V[\x81R` \x01\x84\x8B` \x01QaC\x93\x91\x90aO\xA0V[\x81RP\x88\x85\x81Q\x81\x10aC\xA8WaC\xA8aOqV[` \x90\x81\x02\x91\x90\x91\x01\x01RaC\xBE`\x01\x85aO\xA0V[\x93PaC\xCA\x81\x83aO\xA0V[aC\xD4\x90\x84aO\xA0V[\x92PPPaC\x1CV[P\x84RP\x91\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aDAWaDAaC\xE9V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1D\x07W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aDoW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x89WaD\x89aC\xE9V[aD\x9C` `\x1F\x19`\x1F\x84\x01\x16\x01aD\x18V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aD\xB1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15aD\xE0W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aE\x04WaE\x04aC\xE9V[\x81`@R\x82\x93P\x845\x83R` \x85\x015\x91PaE\x1F\x82aDIV[\x81` \x84\x01R`@\x85\x015\x91PaE5\x82aDIV[\x81`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15aEcW`\0\x80\xFD[PaEp\x85\x82\x86\x01aD^V[`\xA0\x83\x01RPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aE\x90W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xA7W`\0\x80\xFD[aE\xB3\x85\x82\x86\x01aD\xCEV[\x92PP` \x83\x015aE\xC4\x81aDIV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aE\xE1W`\0\x80\xFD[\x815a(;\x81aDIV[`\0\x80`@\x83\x85\x03\x12\x15aE\xFFW`\0\x80\xFD[\x825aF\n\x81aDIV[\x91P` \x83\x015aE\xC4\x81aDIV[`\0\x80`\0\x80`\0\x85\x87\x03`\xE0\x81\x12\x15aF3W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aFKW`\0\x80\xFD[aFW\x8A\x83\x8B\x01aD\xCEV[\x97P` \x89\x015\x96P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x01\x12\x15aF\x90W`\0\x80\xFD[`@\x89\x01\x95P`\xC0\x89\x015\x92P\x80\x83\x11\x15aF\xAAW`\0\x80\xFD[\x82\x89\x01\x92P\x89`\x1F\x84\x01\x12aF\xBEW`\0\x80\xFD[\x825\x91P\x80\x82\x11\x15aF\xCFW`\0\x80\xFD[P\x88` \x82`\x05\x1B\x84\x01\x01\x11\x15aF\xE5W`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95PPP` \x01\x91\x90V[`\0` \x82\x84\x03\x12\x15aG\x08W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15aG*W\x81\x81\x01Q\x83\x82\x01R` \x01aG\x12V[\x83\x81\x11\x15a\x0F\xA2WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaGS\x81` \x86\x01` \x86\x01aG\x0FV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a(;` \x83\x01\x84aG;V[`\0\x80`@\x83\x85\x03\x12\x15aG\x8DW`\0\x80\xFD[\x825\x91P` \x83\x015aE\xC4\x81aDIV[`\0` \x82\x84\x03\x12\x15aG\xB1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xC8W`\0\x80\xFD[aG\xD4\x84\x82\x85\x01aD\xCEV[\x94\x93PPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1D\x07W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aH\x04W`\0\x80\xFD[\x815a(;\x81aG\xDCV[`\0\x80`@\x83\x85\x03\x12\x15aH\"W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x1D\x07W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aHWW`\0\x80\xFD[\x855aHb\x81aDIV[\x94P` \x86\x015\x93P`@\x86\x015aHy\x81aG\xDCV[\x92P``\x86\x015aH\x89\x81aH1V[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xA5W`\0\x80\xFD[aH\xB1\x88\x82\x89\x01aD^V[\x91PP\x92\x95P\x92\x95\x90\x93PV[\x85\x81R\x84` \x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xC0\x1B\x16`@\x82\x01R\x82\x15\x15`\xF8\x1B`H\x82\x01R`\0\x82QaI\x12\x81`I\x85\x01` \x87\x01aG\x0FV[\x91\x90\x91\x01`I\x01\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aI5W`\0\x80\xFD[\x81Qa(;\x81aDIV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1D\x07W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aIdW`\0\x80\xFD[\x81Qa(;\x81aI@V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aI\xB8WaI\xB8aIoV[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aI\xCFW`\0\x80\xFD[\x81Qa(;\x81aH1V[`\0\x80`\0``\x84\x86\x03\x12\x15aI\xEFW`\0\x80\xFD[\x83QaI\xFA\x81aI@V[` \x85\x01Q\x90\x93PaJ\x0B\x81aG\xDCV[`@\x85\x01Q\x90\x92PaJ\x1C\x81aDIV[\x80\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aJhW`\0\x80\xFD[\x81Q`\x03\x81\x10a(;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aJ\x89W`\0\x80\xFD[\x81Qa(;\x81aG\xDCV[`\0`\x80\x82\x84\x03\x12\x15aJ\xA6W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aJ\xC9WaJ\xC9aC\xE9V[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aK\x0CW`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15aK.WaK.aC\xE9V[\x83`\x05\x1B` aK?\x81\x83\x01aD\x18V[\x86\x81R\x91\x85\x01\x91\x81\x81\x01\x906\x84\x11\x15aKWW`\0\x80\xFD[\x86[\x84\x81\x10\x15aK\x8BW\x805\x86\x81\x11\x15aKqW`\0\x80\x81\xFD[aK}6\x82\x8B\x01aD^V[\x84RP\x91\x83\x01\x91\x83\x01aKYV[P\x97\x96PPPPPPPV[`\0\x82\x82\x10\x15aK\xA9WaK\xA9aIoV[P\x03\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aK\xD5WaK\xD5aIoV[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aL\x01WaL\x01aIoV[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aLHWaLHaL\nV[`\0\x19\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aL~WaL~aIoV[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15aL\xBDWaL\xBDaIoV[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15aL\xF1WaL\xF1aIoV[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15aM8WaM8aIoV[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15aMsWaMsaIoV[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15aM\x8FWaM\x8FaIoV[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15aM\xA5WaM\xA5aIoV[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15aM\xEDWaM\xEDaIoV[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15aN!WaN!aIoV[PP\x01\x90V[`\0\x82aN6WaN6aL\nV[P\x04\x90V[\x86\x81R`\0`\x01`\x01`\xA0\x1B\x03\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01RaNy`\xC0\x83\x01\x84aG;V[\x98\x97PPPPPPPPV[\x80Q`\xFF\x81\x16\x81\x14aN\x96W`\0\x80\xFD[\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15aN\xADW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aN\xD0WaN\xD0aC\xE9V[`@R\x82QaN\xDE\x81aI@V[\x81RaN\xEC` \x84\x01aN\x85V[` \x82\x01RaN\xFD`@\x84\x01aN\x85V[`@\x82\x01R``\x83\x01QaO\x10\x81aI@V[``\x82\x01R`\x80\x83\x01QaO#\x81aI@V[`\x80\x82\x01R`\xA0\x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aOKW`\0\x80\xFD[`\xA0\x82\x01R\x93\x92PPPV[`\0`\0\x19\x82\x03aOjWaOjaIoV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aO\xB3WaO\xB3aIoV[P\x01\x90V[`\0`\xFF\x83\x16\x80aO\xCBWaO\xCBaL\nV[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15aO\xF4WaO\xF4aIoV[\x90\x03\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
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
    /**Custom error with signature `OptimismPortal_AlreadyFinalized()` and selector `0x730a1074`.
```solidity
error OptimismPortal_AlreadyFinalized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_AlreadyFinalized;
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
        impl ::core::convert::From<OptimismPortal_AlreadyFinalized>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_AlreadyFinalized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_AlreadyFinalized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_AlreadyFinalized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_AlreadyFinalized()";
            const SELECTOR: [u8; 4] = [115u8, 10u8, 16u8, 116u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_BadTarget()` and selector `0xc5defbad`.
```solidity
error OptimismPortal_BadTarget();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_BadTarget;
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
        impl ::core::convert::From<OptimismPortal_BadTarget>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_BadTarget) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_BadTarget {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_BadTarget {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_BadTarget()";
            const SELECTOR: [u8; 4] = [197u8, 222u8, 251u8, 173u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_CallPaused()` and selector `0xb9c3c2ef`.
```solidity
error OptimismPortal_CallPaused();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_CallPaused;
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
        impl ::core::convert::From<OptimismPortal_CallPaused>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_CallPaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_CallPaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_CallPaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_CallPaused()";
            const SELECTOR: [u8; 4] = [185u8, 195u8, 194u8, 239u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_CalldataTooLarge()` and selector `0x5aa3bac9`.
```solidity
error OptimismPortal_CalldataTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_CalldataTooLarge;
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
        impl ::core::convert::From<OptimismPortal_CalldataTooLarge>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_CalldataTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_CalldataTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_CalldataTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_CalldataTooLarge()";
            const SELECTOR: [u8; 4] = [90u8, 163u8, 186u8, 201u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_GasEstimation()` and selector `0xab581036`.
```solidity
error OptimismPortal_GasEstimation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_GasEstimation;
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
        impl ::core::convert::From<OptimismPortal_GasEstimation>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_GasEstimation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_GasEstimation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_GasEstimation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_GasEstimation()";
            const SELECTOR: [u8; 4] = [171u8, 88u8, 16u8, 54u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_GasLimitTooLow()` and selector `0x70c8bdbd`.
```solidity
error OptimismPortal_GasLimitTooLow();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_GasLimitTooLow;
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
        impl ::core::convert::From<OptimismPortal_GasLimitTooLow>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_GasLimitTooLow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_GasLimitTooLow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_GasLimitTooLow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_GasLimitTooLow()";
            const SELECTOR: [u8; 4] = [112u8, 200u8, 189u8, 189u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_ImproperDisputeGame()` and selector `0xf395240e`.
```solidity
error OptimismPortal_ImproperDisputeGame();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_ImproperDisputeGame;
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
        impl ::core::convert::From<OptimismPortal_ImproperDisputeGame>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_ImproperDisputeGame) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_ImproperDisputeGame {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_ImproperDisputeGame {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_ImproperDisputeGame()";
            const SELECTOR: [u8; 4] = [243u8, 149u8, 36u8, 14u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidDisputeGame()` and selector `0xe29927ed`.
```solidity
error OptimismPortal_InvalidDisputeGame();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidDisputeGame;
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
        impl ::core::convert::From<OptimismPortal_InvalidDisputeGame>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidDisputeGame) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidDisputeGame {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidDisputeGame {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidDisputeGame()";
            const SELECTOR: [u8; 4] = [226u8, 153u8, 39u8, 237u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidLockboxState()` and selector `0x9c46cd79`.
```solidity
error OptimismPortal_InvalidLockboxState();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidLockboxState;
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
        impl ::core::convert::From<OptimismPortal_InvalidLockboxState>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidLockboxState) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidLockboxState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidLockboxState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidLockboxState()";
            const SELECTOR: [u8; 4] = [156u8, 70u8, 205u8, 121u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidMerkleProof()` and selector `0x2e57ef3a`.
```solidity
error OptimismPortal_InvalidMerkleProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidMerkleProof;
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
        impl ::core::convert::From<OptimismPortal_InvalidMerkleProof>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidMerkleProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidMerkleProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidMerkleProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidMerkleProof()";
            const SELECTOR: [u8; 4] = [46u8, 87u8, 239u8, 58u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidOutputRootProof()` and selector `0x426149af`.
```solidity
error OptimismPortal_InvalidOutputRootProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidOutputRootProof;
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
        impl ::core::convert::From<OptimismPortal_InvalidOutputRootProof>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidOutputRootProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidOutputRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidOutputRootProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidOutputRootProof()";
            const SELECTOR: [u8; 4] = [66u8, 97u8, 73u8, 175u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidProofTimestamp()` and selector `0xb4caa4e5`.
```solidity
error OptimismPortal_InvalidProofTimestamp();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidProofTimestamp;
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
        impl ::core::convert::From<OptimismPortal_InvalidProofTimestamp>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidProofTimestamp) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidProofTimestamp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidProofTimestamp {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidProofTimestamp()";
            const SELECTOR: [u8; 4] = [180u8, 202u8, 164u8, 229u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidRootClaim()` and selector `0x332a57f8`.
```solidity
error OptimismPortal_InvalidRootClaim();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidRootClaim;
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
        impl ::core::convert::From<OptimismPortal_InvalidRootClaim>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidRootClaim) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidRootClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidRootClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidRootClaim()";
            const SELECTOR: [u8; 4] = [51u8, 42u8, 87u8, 248u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_NoReentrancy()` and selector `0xdfeaaeb8`.
```solidity
error OptimismPortal_NoReentrancy();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_NoReentrancy;
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
        impl ::core::convert::From<OptimismPortal_NoReentrancy>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_NoReentrancy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_NoReentrancy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_NoReentrancy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_NoReentrancy()";
            const SELECTOR: [u8; 4] = [223u8, 234u8, 174u8, 184u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_NotAllowedOnCGTMode()` and selector `0xbd58e0a2`.
```solidity
error OptimismPortal_NotAllowedOnCGTMode();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_NotAllowedOnCGTMode;
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
        impl ::core::convert::From<OptimismPortal_NotAllowedOnCGTMode>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_NotAllowedOnCGTMode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_NotAllowedOnCGTMode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_NotAllowedOnCGTMode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_NotAllowedOnCGTMode()";
            const SELECTOR: [u8; 4] = [189u8, 88u8, 224u8, 162u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_ProofNotOldEnough()` and selector `0xd9bc01be`.
```solidity
error OptimismPortal_ProofNotOldEnough();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_ProofNotOldEnough;
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
        impl ::core::convert::From<OptimismPortal_ProofNotOldEnough>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_ProofNotOldEnough) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_ProofNotOldEnough {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_ProofNotOldEnough {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_ProofNotOldEnough()";
            const SELECTOR: [u8; 4] = [217u8, 188u8, 1u8, 190u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_Unproven()` and selector `0xcca6afda`.
```solidity
error OptimismPortal_Unproven();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_Unproven;
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
        impl ::core::convert::From<OptimismPortal_Unproven> for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_Unproven) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OptimismPortal_Unproven {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_Unproven {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_Unproven()";
            const SELECTOR: [u8; 4] = [204u8, 166u8, 175u8, 218u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OutOfGas()` and selector `0x77ebef4d`.
```solidity
error OutOfGas();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfGas;
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
        impl ::core::convert::From<OutOfGas> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfGas) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfGas {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfGas {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfGas()";
            const SELECTOR: [u8; 4] = [119u8, 235u8, 239u8, 77u8];
            #[inline]
            fn new<'a>(
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
    /**Event with signature `TransactionDeposited(address,address,uint256,bytes)` and selector `0xb3813568d9991fc951961fcb4c784893574240a28925604d09fc577c55bb7c32`.
```solidity
event TransactionDeposited(address indexed from, address indexed to, uint256 indexed version, bytes opaqueData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionDeposited {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub opaqueData: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for TransactionDeposited {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "TransactionDeposited(address,address,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                179u8, 129u8, 53u8, 104u8, 217u8, 153u8, 31u8, 201u8, 81u8, 150u8, 31u8,
                203u8, 76u8, 120u8, 72u8, 147u8, 87u8, 66u8, 64u8, 162u8, 137u8, 37u8,
                96u8, 77u8, 9u8, 252u8, 87u8, 124u8, 85u8, 187u8, 124u8, 50u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    to: topics.2,
                    version: topics.3,
                    opaqueData: data.0,
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
                        &self.opaqueData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.from.clone(),
                    self.to.clone(),
                    self.version.clone(),
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
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.version);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionDeposited {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionDeposited> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionDeposited) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `WithdrawalFinalized(bytes32,bool)` and selector `0xdb5c7652857aa163daadd670e116628fb42e869d8ac4251ef8971d9e5727df1b`.
```solidity
event WithdrawalFinalized(bytes32 indexed withdrawalHash, bool success);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalFinalized {
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub success: bool,
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
        impl alloy_sol_types::SolEvent for WithdrawalFinalized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "WithdrawalFinalized(bytes32,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                219u8, 92u8, 118u8, 82u8, 133u8, 122u8, 161u8, 99u8, 218u8, 173u8, 214u8,
                112u8, 225u8, 22u8, 98u8, 143u8, 180u8, 46u8, 134u8, 157u8, 138u8, 196u8,
                37u8, 30u8, 248u8, 151u8, 29u8, 158u8, 87u8, 39u8, 223u8, 27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawalHash: topics.1,
                    success: data.0,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.success,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.withdrawalHash.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.withdrawalHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalFinalized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalFinalized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalFinalized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `WithdrawalProven(bytes32,address,address)` and selector `0x67a6208cfcc0801d50f6cbe764733f4fddf66ac0b04442061a8a8c0cb6b63f62`.
```solidity
event WithdrawalProven(bytes32 indexed withdrawalHash, address indexed from, address indexed to);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalProven {
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for WithdrawalProven {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "WithdrawalProven(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                103u8, 166u8, 32u8, 140u8, 252u8, 192u8, 128u8, 29u8, 80u8, 246u8, 203u8,
                231u8, 100u8, 115u8, 63u8, 79u8, 221u8, 246u8, 106u8, 192u8, 176u8, 68u8,
                66u8, 6u8, 26u8, 138u8, 140u8, 12u8, 182u8, 182u8, 63u8, 98u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawalHash: topics.1,
                    from: topics.2,
                    to: topics.3,
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
                    self.withdrawalHash.clone(),
                    self.from.clone(),
                    self.to.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.withdrawalHash);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalProven {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalProven> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalProven) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `WithdrawalProvenExtension1(bytes32,address)` and selector `0x798f9f13695f8f045aa5f80ed8efebb695f3c7fe65da381969f2f28bf3c60b97`.
```solidity
event WithdrawalProvenExtension1(bytes32 indexed withdrawalHash, address indexed proofSubmitter);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalProvenExtension1 {
        #[allow(missing_docs)]
        pub withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proofSubmitter: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for WithdrawalProvenExtension1 {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "WithdrawalProvenExtension1(bytes32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                121u8, 143u8, 159u8, 19u8, 105u8, 95u8, 143u8, 4u8, 90u8, 165u8, 248u8,
                14u8, 216u8, 239u8, 235u8, 182u8, 149u8, 243u8, 199u8, 254u8, 101u8,
                218u8, 56u8, 25u8, 105u8, 242u8, 242u8, 139u8, 243u8, 198u8, 11u8, 151u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawalHash: topics.1,
                    proofSubmitter: topics.2,
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
                    self.withdrawalHash.clone(),
                    self.proofSubmitter.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.withdrawalHash);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.proofSubmitter,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalProvenExtension1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalProvenExtension1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &WithdrawalProvenExtension1,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(uint256 _proofMaturityDelaySeconds);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _proofMaturityDelaySeconds: alloy::sol_types::private::primitives::aliases::U256,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._proofMaturityDelaySeconds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _proofMaturityDelaySeconds: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._proofMaturityDelaySeconds,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `anchorStateRegistry()` and selector `0x5c0cba33`.
```solidity
function anchorStateRegistry() external view returns (address);
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for anchorStateRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                        let r: anchorStateRegistryReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkWithdrawal(bytes32,address)` and selector `0x71c1566e`.
```solidity
function checkWithdrawal(bytes32 _withdrawalHash, address _proofSubmitter) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkWithdrawalCall {
        #[allow(missing_docs)]
        pub _withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _proofSubmitter: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`checkWithdrawal(bytes32,address)`](checkWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkWithdrawalReturn {}
    #[allow(
        non_camel_case_types,
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<checkWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkWithdrawalCall) -> Self {
                    (value._withdrawalHash, value._proofSubmitter)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _withdrawalHash: tuple.0,
                        _proofSubmitter: tuple.1,
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
            impl ::core::convert::From<checkWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl checkWithdrawalReturn {
            fn _tokenize(
                &self,
            ) -> <checkWithdrawalCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkWithdrawal(bytes32,address)";
            const SELECTOR: [u8; 4] = [113u8, 193u8, 86u8, 110u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._withdrawalHash),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._proofSubmitter,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                checkWithdrawalReturn::_tokenize(ret)
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
    /**Function with signature `depositTransaction(address,uint256,uint64,bool,bytes)` and selector `0xe9e05c42`.
```solidity
function depositTransaction(address _to, uint256 _value, uint64 _gasLimit, bool _isCreation, bytes memory _data) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositTransactionCall {
        #[allow(missing_docs)]
        pub _to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _gasLimit: u64,
        #[allow(missing_docs)]
        pub _isCreation: bool,
        #[allow(missing_docs)]
        pub _data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`depositTransaction(address,uint256,uint64,bool,bytes)`](depositTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositTransactionReturn {}
    #[allow(
        non_camel_case_types,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                u64,
                bool,
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
            impl ::core::convert::From<depositTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositTransactionCall) -> Self {
                    (
                        value._to,
                        value._value,
                        value._gasLimit,
                        value._isCreation,
                        value._data,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _to: tuple.0,
                        _value: tuple.1,
                        _gasLimit: tuple.2,
                        _isCreation: tuple.3,
                        _data: tuple.4,
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
            impl ::core::convert::From<depositTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl depositTransactionReturn {
            fn _tokenize(
                &self,
            ) -> <depositTransactionCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositTransactionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositTransaction(address,uint256,uint64,bool,bytes)";
            const SELECTOR: [u8; 4] = [233u8, 224u8, 92u8, 66u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._gasLimit),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._isCreation,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._data,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                depositTransactionReturn::_tokenize(ret)
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
    /**Function with signature `disputeGameBlacklist(address)` and selector `0x45884d32`.
```solidity
function disputeGameBlacklist(address _disputeGame) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameBlacklistCall {
        #[allow(missing_docs)]
        pub _disputeGame: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`disputeGameBlacklist(address)`](disputeGameBlacklistCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameBlacklistReturn {
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
            impl ::core::convert::From<disputeGameBlacklistCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameBlacklistCall) -> Self {
                    (value._disputeGame,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameBlacklistCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _disputeGame: tuple.0 }
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
            impl ::core::convert::From<disputeGameBlacklistReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameBlacklistReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameBlacklistReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disputeGameBlacklistCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disputeGameBlacklist(address)";
            const SELECTOR: [u8; 4] = [69u8, 136u8, 77u8, 50u8];
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
                        &self._disputeGame,
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
                        let r: disputeGameBlacklistReturn = r.into();
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
                        let r: disputeGameBlacklistReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disputeGameFactory()` and selector `0xf2b4e617`.
```solidity
function disputeGameFactory() external view returns (address);
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameFactoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                        let r: disputeGameFactoryReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disputeGameFinalityDelaySeconds()` and selector `0x952b2797`.
```solidity
function disputeGameFinalityDelaySeconds() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameFinalityDelaySecondsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`disputeGameFinalityDelaySeconds()`](disputeGameFinalityDelaySecondsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeGameFinalityDelaySecondsReturn {
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
            impl ::core::convert::From<disputeGameFinalityDelaySecondsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameFinalityDelaySecondsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameFinalityDelaySecondsCall {
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
            impl ::core::convert::From<disputeGameFinalityDelaySecondsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disputeGameFinalityDelaySecondsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disputeGameFinalityDelaySecondsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disputeGameFinalityDelaySecondsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disputeGameFinalityDelaySeconds()";
            const SELECTOR: [u8; 4] = [149u8, 43u8, 39u8, 151u8];
            #[inline]
            fn new<'a>(
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
                        let r: disputeGameFinalityDelaySecondsReturn = r.into();
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
                        let r: disputeGameFinalityDelaySecondsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `donateETH()` and selector `0x8b4c40b0`.
```solidity
function donateETH() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct donateETHCall;
    ///Container type for the return parameters of the [`donateETH()`](donateETHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct donateETHReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<donateETHCall> for UnderlyingRustTuple<'_> {
                fn from(value: donateETHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for donateETHCall {
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
            impl ::core::convert::From<donateETHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: donateETHReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for donateETHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl donateETHReturn {
            fn _tokenize(
                &self,
            ) -> <donateETHCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for donateETHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = donateETHReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "donateETH()";
            const SELECTOR: [u8; 4] = [139u8, 76u8, 64u8, 176u8];
            #[inline]
            fn new<'a>(
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
                donateETHReturn::_tokenize(ret)
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
    /**Function with signature `ethLockbox()` and selector `0xb682c444`.
```solidity
function ethLockbox() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethLockboxCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ethLockbox()`](ethLockboxCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethLockboxReturn {
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
            impl ::core::convert::From<ethLockboxCall> for UnderlyingRustTuple<'_> {
                fn from(value: ethLockboxCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ethLockboxCall {
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
            impl ::core::convert::From<ethLockboxReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ethLockboxReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ethLockboxReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ethLockboxCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ethLockbox()";
            const SELECTOR: [u8; 4] = [182u8, 130u8, 196u8, 68u8];
            #[inline]
            fn new<'a>(
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
                        let r: ethLockboxReturn = r.into();
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
                        let r: ethLockboxReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `finalizeWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes))` and selector `0x8c3152e9`.
```solidity
function finalizeWithdrawalTransaction(Types.WithdrawalTransaction memory _tx) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeWithdrawalTransactionCall {
        #[allow(missing_docs)]
        pub _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`finalizeWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes))`](finalizeWithdrawalTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeWithdrawalTransactionReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Types::WithdrawalTransaction,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<finalizeWithdrawalTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeWithdrawalTransactionCall) -> Self {
                    (value._tx,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeWithdrawalTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _tx: tuple.0 }
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
            impl ::core::convert::From<finalizeWithdrawalTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeWithdrawalTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeWithdrawalTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl finalizeWithdrawalTransactionReturn {
            fn _tokenize(
                &self,
            ) -> <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizeWithdrawalTransactionCall {
            type Parameters<'a> = (Types::WithdrawalTransaction,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizeWithdrawalTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizeWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes))";
            const SELECTOR: [u8; 4] = [140u8, 49u8, 82u8, 233u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Types::WithdrawalTransaction as alloy_sol_types::SolType>::tokenize(
                        &self._tx,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                finalizeWithdrawalTransactionReturn::_tokenize(ret)
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
    /**Function with signature `finalizeWithdrawalTransactionExternalProof((uint256,address,address,uint256,uint256,bytes),address)` and selector `0x43ca1c50`.
```solidity
function finalizeWithdrawalTransactionExternalProof(Types.WithdrawalTransaction memory _tx, address _proofSubmitter) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeWithdrawalTransactionExternalProofCall {
        #[allow(missing_docs)]
        pub _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _proofSubmitter: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`finalizeWithdrawalTransactionExternalProof((uint256,address,address,uint256,uint256,bytes),address)`](finalizeWithdrawalTransactionExternalProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeWithdrawalTransactionExternalProofReturn {}
    #[allow(
        non_camel_case_types,
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
                Types::WithdrawalTransaction,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<finalizeWithdrawalTransactionExternalProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeWithdrawalTransactionExternalProofCall) -> Self {
                    (value._tx, value._proofSubmitter)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeWithdrawalTransactionExternalProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _tx: tuple.0,
                        _proofSubmitter: tuple.1,
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
            impl ::core::convert::From<finalizeWithdrawalTransactionExternalProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: finalizeWithdrawalTransactionExternalProofReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeWithdrawalTransactionExternalProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl finalizeWithdrawalTransactionExternalProofReturn {
            fn _tokenize(
                &self,
            ) -> <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for finalizeWithdrawalTransactionExternalProofCall {
            type Parameters<'a> = (
                Types::WithdrawalTransaction,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizeWithdrawalTransactionExternalProofReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizeWithdrawalTransactionExternalProof((uint256,address,address,uint256,uint256,bytes),address)";
            const SELECTOR: [u8; 4] = [67u8, 202u8, 28u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Types::WithdrawalTransaction as alloy_sol_types::SolType>::tokenize(
                        &self._tx,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._proofSubmitter,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                finalizeWithdrawalTransactionExternalProofReturn::_tokenize(ret)
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
    /**Function with signature `finalizedWithdrawals(bytes32)` and selector `0xa14238e7`.
```solidity
function finalizedWithdrawals(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizedWithdrawalsCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`finalizedWithdrawals(bytes32)`](finalizedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizedWithdrawalsReturn {
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
            impl ::core::convert::From<finalizedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizedWithdrawalsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizedWithdrawalsCall {
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
            impl ::core::convert::From<finalizedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizedWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizedWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizedWithdrawals(bytes32)";
            const SELECTOR: [u8; 4] = [161u8, 66u8, 56u8, 231u8];
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
                        let r: finalizedWithdrawalsReturn = r.into();
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
                        let r: finalizedWithdrawalsReturn = r.into();
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
    /**Function with signature `initialize(address,address)` and selector `0x485cc955`.
```solidity
function initialize(address _systemConfig, address _anchorStateRegistry) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _systemConfig: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _anchorStateRegistry: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._systemConfig, value._anchorStateRegistry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _systemConfig: tuple.0,
                        _anchorStateRegistry: tuple.1,
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
            const SIGNATURE: &'static str = "initialize(address,address)";
            const SELECTOR: [u8; 4] = [72u8, 92u8, 201u8, 85u8];
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
                        &self._systemConfig,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._anchorStateRegistry,
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
    /**Function with signature `l2Sender()` and selector `0x9bf62d82`.
```solidity
function l2Sender() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2SenderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`l2Sender()`](l2SenderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct l2SenderReturn {
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
            impl ::core::convert::From<l2SenderCall> for UnderlyingRustTuple<'_> {
                fn from(value: l2SenderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2SenderCall {
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
            impl ::core::convert::From<l2SenderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: l2SenderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for l2SenderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for l2SenderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "l2Sender()";
            const SELECTOR: [u8; 4] = [155u8, 246u8, 45u8, 130u8];
            #[inline]
            fn new<'a>(
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
                        let r: l2SenderReturn = r.into();
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
                        let r: l2SenderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `minimumGasLimit(uint64)` and selector `0xa35d99df`.
```solidity
function minimumGasLimit(uint64 _byteCount) external pure returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumGasLimitCall {
        #[allow(missing_docs)]
        pub _byteCount: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`minimumGasLimit(uint64)`](minimumGasLimitCall) function.
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
            impl ::core::convert::From<minimumGasLimitCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumGasLimitCall) -> Self {
                    (value._byteCount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumGasLimitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _byteCount: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumGasLimit(uint64)";
            const SELECTOR: [u8; 4] = [163u8, 93u8, 153u8, 223u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._byteCount),
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
    /**Function with signature `numProofSubmitters(bytes32)` and selector `0x513747ab`.
```solidity
function numProofSubmitters(bytes32 _withdrawalHash) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numProofSubmittersCall {
        #[allow(missing_docs)]
        pub _withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`numProofSubmitters(bytes32)`](numProofSubmittersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numProofSubmittersReturn {
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
            impl ::core::convert::From<numProofSubmittersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: numProofSubmittersCall) -> Self {
                    (value._withdrawalHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for numProofSubmittersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _withdrawalHash: tuple.0 }
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
            impl ::core::convert::From<numProofSubmittersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: numProofSubmittersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for numProofSubmittersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numProofSubmittersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "numProofSubmitters(bytes32)";
            const SELECTOR: [u8; 4] = [81u8, 55u8, 71u8, 171u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._withdrawalHash),
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
                        let r: numProofSubmittersReturn = r.into();
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
                        let r: numProofSubmittersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `params()` and selector `0xcff0ab96`.
```solidity
function params() external view returns (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paramsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`params()`](paramsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paramsReturn {
        #[allow(missing_docs)]
        pub prevBaseFee: u128,
        #[allow(missing_docs)]
        pub prevBoughtGas: u64,
        #[allow(missing_docs)]
        pub prevBlockNum: u64,
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
            impl ::core::convert::From<paramsCall> for UnderlyingRustTuple<'_> {
                fn from(value: paramsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paramsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128, u64, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paramsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: paramsReturn) -> Self {
                    (value.prevBaseFee, value.prevBoughtGas, value.prevBlockNum)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paramsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        prevBaseFee: tuple.0,
                        prevBoughtGas: tuple.1,
                        prevBlockNum: tuple.2,
                    }
                }
            }
        }
        impl paramsReturn {
            fn _tokenize(
                &self,
            ) -> <paramsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevBaseFee),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevBoughtGas),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevBlockNum),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paramsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paramsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "params()";
            const SELECTOR: [u8; 4] = [207u8, 240u8, 171u8, 150u8];
            #[inline]
            fn new<'a>(
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
                paramsReturn::_tokenize(ret)
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
    /**Function with signature `proofMaturityDelaySeconds()` and selector `0xbf653a5c`.
```solidity
function proofMaturityDelaySeconds() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofMaturityDelaySecondsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proofMaturityDelaySeconds()`](proofMaturityDelaySecondsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofMaturityDelaySecondsReturn {
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
            impl ::core::convert::From<proofMaturityDelaySecondsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofMaturityDelaySecondsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofMaturityDelaySecondsCall {
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
            impl ::core::convert::From<proofMaturityDelaySecondsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofMaturityDelaySecondsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofMaturityDelaySecondsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofMaturityDelaySecondsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proofMaturityDelaySeconds()";
            const SELECTOR: [u8; 4] = [191u8, 101u8, 58u8, 92u8];
            #[inline]
            fn new<'a>(
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
                        let r: proofMaturityDelaySecondsReturn = r.into();
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
                        let r: proofMaturityDelaySecondsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proofSubmitters(bytes32,uint256)` and selector `0xa3860f48`.
```solidity
function proofSubmitters(bytes32, uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofSubmittersCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proofSubmitters(bytes32,uint256)`](proofSubmittersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofSubmittersReturn {
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
            impl ::core::convert::From<proofSubmittersCall> for UnderlyingRustTuple<'_> {
                fn from(value: proofSubmittersCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proofSubmittersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<proofSubmittersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofSubmittersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofSubmittersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofSubmittersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proofSubmitters(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [163u8, 134u8, 15u8, 72u8];
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
                        let r: proofSubmittersReturn = r.into();
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
                        let r: proofSubmittersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proveWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes),uint256,(bytes32,bytes32,bytes32,bytes32),bytes[])` and selector `0x4870496f`.
```solidity
function proveWithdrawalTransaction(Types.WithdrawalTransaction memory _tx, uint256 _disputeGameIndex, Types.OutputRootProof memory _outputRootProof, bytes[] memory _withdrawalProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proveWithdrawalTransactionCall {
        #[allow(missing_docs)]
        pub _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _disputeGameIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _withdrawalProof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
    }
    ///Container type for the return parameters of the [`proveWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes),uint256,(bytes32,bytes32,bytes32,bytes32),bytes[])`](proveWithdrawalTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proveWithdrawalTransactionReturn {}
    #[allow(
        non_camel_case_types,
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
                Types::WithdrawalTransaction,
                alloy::sol_types::sol_data::Uint<256>,
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<proveWithdrawalTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proveWithdrawalTransactionCall) -> Self {
                    (
                        value._tx,
                        value._disputeGameIndex,
                        value._outputRootProof,
                        value._withdrawalProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proveWithdrawalTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _tx: tuple.0,
                        _disputeGameIndex: tuple.1,
                        _outputRootProof: tuple.2,
                        _withdrawalProof: tuple.3,
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
            impl ::core::convert::From<proveWithdrawalTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proveWithdrawalTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proveWithdrawalTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proveWithdrawalTransactionReturn {
            fn _tokenize(
                &self,
            ) -> <proveWithdrawalTransactionCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proveWithdrawalTransactionCall {
            type Parameters<'a> = (
                Types::WithdrawalTransaction,
                alloy::sol_types::sol_data::Uint<256>,
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proveWithdrawalTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proveWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes),uint256,(bytes32,bytes32,bytes32,bytes32),bytes[])";
            const SELECTOR: [u8; 4] = [72u8, 112u8, 73u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Types::WithdrawalTransaction as alloy_sol_types::SolType>::tokenize(
                        &self._tx,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._disputeGameIndex),
                    <Types::OutputRootProof as alloy_sol_types::SolType>::tokenize(
                        &self._outputRootProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self._withdrawalProof),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                proveWithdrawalTransactionReturn::_tokenize(ret)
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
    /**Function with signature `provenWithdrawals(bytes32,address)` and selector `0xbb2c727e`.
```solidity
function provenWithdrawals(bytes32, address) external view returns (address disputeGameProxy, uint64 timestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct provenWithdrawalsCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`provenWithdrawals(bytes32,address)`](provenWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct provenWithdrawalsReturn {
        #[allow(missing_docs)]
        pub disputeGameProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timestamp: u64,
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<provenWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: provenWithdrawalsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for provenWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<provenWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: provenWithdrawalsReturn) -> Self {
                    (value.disputeGameProxy, value.timestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for provenWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        disputeGameProxy: tuple.0,
                        timestamp: tuple.1,
                    }
                }
            }
        }
        impl provenWithdrawalsReturn {
            fn _tokenize(
                &self,
            ) -> <provenWithdrawalsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.disputeGameProxy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for provenWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = provenWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "provenWithdrawals(bytes32,address)";
            const SELECTOR: [u8; 4] = [187u8, 44u8, 114u8, 126u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                provenWithdrawalsReturn::_tokenize(ret)
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
    /**Function with signature `respectedGameType()` and selector `0x3c9f397c`.
```solidity
function respectedGameType() external view returns (GameType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respectedGameTypeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`respectedGameType()`](respectedGameTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respectedGameTypeReturn {
        #[allow(missing_docs)]
        pub _0: <GameType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<respectedGameTypeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: respectedGameTypeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for respectedGameTypeCall {
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
            impl ::core::convert::From<respectedGameTypeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: respectedGameTypeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for respectedGameTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respectedGameTypeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GameType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GameType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respectedGameType()";
            const SELECTOR: [u8; 4] = [60u8, 159u8, 57u8, 124u8];
            #[inline]
            fn new<'a>(
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
                        let r: respectedGameTypeReturn = r.into();
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
                        let r: respectedGameTypeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `respectedGameTypeUpdatedAt()` and selector `0x4fd0434c`.
```solidity
function respectedGameTypeUpdatedAt() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respectedGameTypeUpdatedAtCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`respectedGameTypeUpdatedAt()`](respectedGameTypeUpdatedAtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respectedGameTypeUpdatedAtReturn {
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
            impl ::core::convert::From<respectedGameTypeUpdatedAtCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: respectedGameTypeUpdatedAtCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for respectedGameTypeUpdatedAtCall {
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
            impl ::core::convert::From<respectedGameTypeUpdatedAtReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: respectedGameTypeUpdatedAtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for respectedGameTypeUpdatedAtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respectedGameTypeUpdatedAtCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respectedGameTypeUpdatedAt()";
            const SELECTOR: [u8; 4] = [79u8, 208u8, 67u8, 76u8];
            #[inline]
            fn new<'a>(
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
                        let r: respectedGameTypeUpdatedAtReturn = r.into();
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
                        let r: respectedGameTypeUpdatedAtReturn = r.into();
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
    /**Function with signature `systemConfig()` and selector `0x33d7e2bd`.
```solidity
function systemConfig() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct systemConfigCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`systemConfig()`](systemConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct systemConfigReturn {
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
            impl ::core::convert::From<systemConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: systemConfigCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for systemConfigCall {
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
            impl ::core::convert::From<systemConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: systemConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for systemConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for systemConfigCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "systemConfig()";
            const SELECTOR: [u8; 4] = [51u8, 215u8, 226u8, 189u8];
            #[inline]
            fn new<'a>(
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
                        let r: systemConfigReturn = r.into();
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
                        let r: systemConfigReturn = r.into();
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
    ///Container for all the [`OptimismPortal2`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum OptimismPortal2Calls {
        #[allow(missing_docs)]
        anchorStateRegistry(anchorStateRegistryCall),
        #[allow(missing_docs)]
        checkWithdrawal(checkWithdrawalCall),
        #[allow(missing_docs)]
        depositTransaction(depositTransactionCall),
        #[allow(missing_docs)]
        disputeGameBlacklist(disputeGameBlacklistCall),
        #[allow(missing_docs)]
        disputeGameFactory(disputeGameFactoryCall),
        #[allow(missing_docs)]
        disputeGameFinalityDelaySeconds(disputeGameFinalityDelaySecondsCall),
        #[allow(missing_docs)]
        donateETH(donateETHCall),
        #[allow(missing_docs)]
        ethLockbox(ethLockboxCall),
        #[allow(missing_docs)]
        finalizeWithdrawalTransaction(finalizeWithdrawalTransactionCall),
        #[allow(missing_docs)]
        finalizeWithdrawalTransactionExternalProof(
            finalizeWithdrawalTransactionExternalProofCall,
        ),
        #[allow(missing_docs)]
        finalizedWithdrawals(finalizedWithdrawalsCall),
        #[allow(missing_docs)]
        guardian(guardianCall),
        #[allow(missing_docs)]
        initVersion(initVersionCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        l2Sender(l2SenderCall),
        #[allow(missing_docs)]
        minimumGasLimit(minimumGasLimitCall),
        #[allow(missing_docs)]
        numProofSubmitters(numProofSubmittersCall),
        #[allow(missing_docs)]
        params(paramsCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        proofMaturityDelaySeconds(proofMaturityDelaySecondsCall),
        #[allow(missing_docs)]
        proofSubmitters(proofSubmittersCall),
        #[allow(missing_docs)]
        proveWithdrawalTransaction(proveWithdrawalTransactionCall),
        #[allow(missing_docs)]
        provenWithdrawals(provenWithdrawalsCall),
        #[allow(missing_docs)]
        proxyAdmin(proxyAdminCall),
        #[allow(missing_docs)]
        proxyAdminOwner(proxyAdminOwnerCall),
        #[allow(missing_docs)]
        respectedGameType(respectedGameTypeCall),
        #[allow(missing_docs)]
        respectedGameTypeUpdatedAt(respectedGameTypeUpdatedAtCall),
        #[allow(missing_docs)]
        superchainConfig(superchainConfigCall),
        #[allow(missing_docs)]
        systemConfig(systemConfigCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl OptimismPortal2Calls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [51u8, 215u8, 226u8, 189u8],
            [53u8, 232u8, 10u8, 179u8],
            [56u8, 211u8, 140u8, 151u8],
            [60u8, 159u8, 57u8, 124u8],
            [62u8, 71u8, 21u8, 140u8],
            [67u8, 202u8, 28u8, 80u8],
            [69u8, 42u8, 147u8, 32u8],
            [69u8, 136u8, 77u8, 50u8],
            [72u8, 92u8, 201u8, 85u8],
            [72u8, 112u8, 73u8, 111u8],
            [79u8, 208u8, 67u8, 76u8],
            [81u8, 55u8, 71u8, 171u8],
            [84u8, 253u8, 77u8, 80u8],
            [92u8, 12u8, 186u8, 51u8],
            [92u8, 151u8, 90u8, 187u8],
            [113u8, 193u8, 86u8, 110u8],
            [139u8, 76u8, 64u8, 176u8],
            [140u8, 49u8, 82u8, 233u8],
            [149u8, 43u8, 39u8, 151u8],
            [155u8, 246u8, 45u8, 130u8],
            [161u8, 66u8, 56u8, 231u8],
            [163u8, 93u8, 153u8, 223u8],
            [163u8, 134u8, 15u8, 72u8],
            [182u8, 130u8, 196u8, 68u8],
            [187u8, 44u8, 114u8, 126u8],
            [191u8, 101u8, 58u8, 92u8],
            [207u8, 240u8, 171u8, 150u8],
            [218u8, 213u8, 68u8, 224u8],
            [233u8, 224u8, 92u8, 66u8],
            [242u8, 180u8, 230u8, 23u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(systemConfig),
            ::core::stringify!(superchainConfig),
            ::core::stringify!(initVersion),
            ::core::stringify!(respectedGameType),
            ::core::stringify!(proxyAdmin),
            ::core::stringify!(finalizeWithdrawalTransactionExternalProof),
            ::core::stringify!(guardian),
            ::core::stringify!(disputeGameBlacklist),
            ::core::stringify!(initialize),
            ::core::stringify!(proveWithdrawalTransaction),
            ::core::stringify!(respectedGameTypeUpdatedAt),
            ::core::stringify!(numProofSubmitters),
            ::core::stringify!(version),
            ::core::stringify!(anchorStateRegistry),
            ::core::stringify!(paused),
            ::core::stringify!(checkWithdrawal),
            ::core::stringify!(donateETH),
            ::core::stringify!(finalizeWithdrawalTransaction),
            ::core::stringify!(disputeGameFinalityDelaySeconds),
            ::core::stringify!(l2Sender),
            ::core::stringify!(finalizedWithdrawals),
            ::core::stringify!(minimumGasLimit),
            ::core::stringify!(proofSubmitters),
            ::core::stringify!(ethLockbox),
            ::core::stringify!(provenWithdrawals),
            ::core::stringify!(proofMaturityDelaySeconds),
            ::core::stringify!(params),
            ::core::stringify!(proxyAdminOwner),
            ::core::stringify!(depositTransaction),
            ::core::stringify!(disputeGameFactory),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <systemConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <superchainConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <respectedGameTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxyAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::SIGNATURE,
            <guardianCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeGameBlacklistCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proveWithdrawalTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <numProofSubmittersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <anchorStateRegistryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkWithdrawalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <donateETHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2SenderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <minimumGasLimitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proofSubmittersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ethLockboxCall as alloy_sol_types::SolCall>::SIGNATURE,
            <provenWithdrawalsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <paramsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxyAdminOwnerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <depositTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeGameFactoryCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OptimismPortal2Calls {
        const NAME: &'static str = "OptimismPortal2Calls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::anchorStateRegistry(_) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkWithdrawal(_) => {
                    <checkWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositTransaction(_) => {
                    <depositTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disputeGameBlacklist(_) => {
                    <disputeGameBlacklistCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disputeGameFactory(_) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disputeGameFinalityDelaySeconds(_) => {
                    <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::donateETH(_) => {
                    <donateETHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ethLockbox(_) => {
                    <ethLockboxCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizeWithdrawalTransaction(_) => {
                    <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizeWithdrawalTransactionExternalProof(_) => {
                    <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizedWithdrawals(_) => {
                    <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::guardian(_) => <guardianCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initVersion(_) => {
                    <initVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::l2Sender(_) => <l2SenderCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::minimumGasLimit(_) => {
                    <minimumGasLimitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::numProofSubmitters(_) => {
                    <numProofSubmittersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::params(_) => <paramsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proofMaturityDelaySeconds(_) => {
                    <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proofSubmitters(_) => {
                    <proofSubmittersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proveWithdrawalTransaction(_) => {
                    <proveWithdrawalTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::provenWithdrawals(_) => {
                    <provenWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxyAdmin(_) => {
                    <proxyAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxyAdminOwner(_) => {
                    <proxyAdminOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respectedGameType(_) => {
                    <respectedGameTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respectedGameTypeUpdatedAt(_) => {
                    <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::superchainConfig(_) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::systemConfig(_) => {
                    <systemConfigCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OptimismPortal2Calls>] = &[
                {
                    fn systemConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <systemConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::systemConfig)
                    }
                    systemConfig
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn initVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <initVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::initVersion)
                    }
                    initVersion
                },
                {
                    fn respectedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <respectedGameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::respectedGameType)
                    }
                    respectedGameType
                },
                {
                    fn proxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::proxyAdmin)
                    }
                    proxyAdmin
                },
                {
                    fn finalizeWithdrawalTransactionExternalProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Calls::finalizeWithdrawalTransactionExternalProof,
                            )
                    }
                    finalizeWithdrawalTransactionExternalProof
                },
                {
                    fn guardian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <guardianCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortal2Calls::guardian)
                    }
                    guardian
                },
                {
                    fn disputeGameBlacklist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <disputeGameBlacklistCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::disputeGameBlacklist)
                    }
                    disputeGameBlacklist
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::initialize)
                    }
                    initialize
                },
                {
                    fn proveWithdrawalTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proveWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::proveWithdrawalTransaction)
                    }
                    proveWithdrawalTransaction
                },
                {
                    fn respectedGameTypeUpdatedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::respectedGameTypeUpdatedAt)
                    }
                    respectedGameTypeUpdatedAt
                },
                {
                    fn numProofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <numProofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::numProofSubmitters)
                    }
                    numProofSubmitters
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortal2Calls::version)
                    }
                    version
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortal2Calls::paused)
                    }
                    paused
                },
                {
                    fn checkWithdrawal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <checkWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::checkWithdrawal)
                    }
                    checkWithdrawal
                },
                {
                    fn donateETH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <donateETHCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortal2Calls::donateETH)
                    }
                    donateETH
                },
                {
                    fn finalizeWithdrawalTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::finalizeWithdrawalTransaction)
                    }
                    finalizeWithdrawalTransaction
                },
                {
                    fn disputeGameFinalityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::disputeGameFinalityDelaySeconds)
                    }
                    disputeGameFinalityDelaySeconds
                },
                {
                    fn l2Sender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <l2SenderCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortal2Calls::l2Sender)
                    }
                    l2Sender
                },
                {
                    fn finalizedWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::finalizedWithdrawals)
                    }
                    finalizedWithdrawals
                },
                {
                    fn minimumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::minimumGasLimit)
                    }
                    minimumGasLimit
                },
                {
                    fn proofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::proofSubmitters)
                    }
                    proofSubmitters
                },
                {
                    fn ethLockbox(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <ethLockboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::ethLockbox)
                    }
                    ethLockbox
                },
                {
                    fn provenWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <provenWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::provenWithdrawals)
                    }
                    provenWithdrawals
                },
                {
                    fn proofMaturityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::proofMaturityDelaySeconds)
                    }
                    proofMaturityDelaySeconds
                },
                {
                    fn params(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <paramsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortal2Calls::params)
                    }
                    params
                },
                {
                    fn proxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::proxyAdminOwner)
                    }
                    proxyAdminOwner
                },
                {
                    fn depositTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <depositTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::depositTransaction)
                    }
                    depositTransaction
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Calls::disputeGameFactory)
                    }
                    disputeGameFactory
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
            ) -> alloy_sol_types::Result<OptimismPortal2Calls>] = &[
                {
                    fn systemConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <systemConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::systemConfig)
                    }
                    systemConfig
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn initVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <initVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::initVersion)
                    }
                    initVersion
                },
                {
                    fn respectedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <respectedGameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::respectedGameType)
                    }
                    respectedGameType
                },
                {
                    fn proxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::proxyAdmin)
                    }
                    proxyAdmin
                },
                {
                    fn finalizeWithdrawalTransactionExternalProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Calls::finalizeWithdrawalTransactionExternalProof,
                            )
                    }
                    finalizeWithdrawalTransactionExternalProof
                },
                {
                    fn guardian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <guardianCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::guardian)
                    }
                    guardian
                },
                {
                    fn disputeGameBlacklist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <disputeGameBlacklistCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::disputeGameBlacklist)
                    }
                    disputeGameBlacklist
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::initialize)
                    }
                    initialize
                },
                {
                    fn proveWithdrawalTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proveWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::proveWithdrawalTransaction)
                    }
                    proveWithdrawalTransaction
                },
                {
                    fn respectedGameTypeUpdatedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::respectedGameTypeUpdatedAt)
                    }
                    respectedGameTypeUpdatedAt
                },
                {
                    fn numProofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <numProofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::numProofSubmitters)
                    }
                    numProofSubmitters
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::version)
                    }
                    version
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::paused)
                    }
                    paused
                },
                {
                    fn checkWithdrawal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <checkWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::checkWithdrawal)
                    }
                    checkWithdrawal
                },
                {
                    fn donateETH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <donateETHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::donateETH)
                    }
                    donateETH
                },
                {
                    fn finalizeWithdrawalTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::finalizeWithdrawalTransaction)
                    }
                    finalizeWithdrawalTransaction
                },
                {
                    fn disputeGameFinalityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::disputeGameFinalityDelaySeconds)
                    }
                    disputeGameFinalityDelaySeconds
                },
                {
                    fn l2Sender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <l2SenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::l2Sender)
                    }
                    l2Sender
                },
                {
                    fn finalizedWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::finalizedWithdrawals)
                    }
                    finalizedWithdrawals
                },
                {
                    fn minimumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::minimumGasLimit)
                    }
                    minimumGasLimit
                },
                {
                    fn proofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::proofSubmitters)
                    }
                    proofSubmitters
                },
                {
                    fn ethLockbox(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <ethLockboxCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::ethLockbox)
                    }
                    ethLockbox
                },
                {
                    fn provenWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <provenWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::provenWithdrawals)
                    }
                    provenWithdrawals
                },
                {
                    fn proofMaturityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::proofMaturityDelaySeconds)
                    }
                    proofMaturityDelaySeconds
                },
                {
                    fn params(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <paramsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::params)
                    }
                    params
                },
                {
                    fn proxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::proxyAdminOwner)
                    }
                    proxyAdminOwner
                },
                {
                    fn depositTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <depositTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::depositTransaction)
                    }
                    depositTransaction
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Calls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Calls::disputeGameFactory)
                    }
                    disputeGameFactory
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
                Self::anchorStateRegistry(inner) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkWithdrawal(inner) => {
                    <checkWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositTransaction(inner) => {
                    <depositTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disputeGameBlacklist(inner) => {
                    <disputeGameBlacklistCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disputeGameFactory(inner) => {
                    <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disputeGameFinalityDelaySeconds(inner) => {
                    <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::donateETH(inner) => {
                    <donateETHCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::ethLockbox(inner) => {
                    <ethLockboxCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::finalizeWithdrawalTransaction(inner) => {
                    <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizeWithdrawalTransactionExternalProof(inner) => {
                    <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizedWithdrawals(inner) => {
                    <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::l2Sender(inner) => {
                    <l2SenderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::minimumGasLimit(inner) => {
                    <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::numProofSubmitters(inner) => {
                    <numProofSubmittersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::params(inner) => {
                    <paramsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proofMaturityDelaySeconds(inner) => {
                    <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proofSubmitters(inner) => {
                    <proofSubmittersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proveWithdrawalTransaction(inner) => {
                    <proveWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::provenWithdrawals(inner) => {
                    <provenWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proxyAdmin(inner) => {
                    <proxyAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxyAdminOwner(inner) => {
                    <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respectedGameType(inner) => {
                    <respectedGameTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respectedGameTypeUpdatedAt(inner) => {
                    <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::superchainConfig(inner) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::systemConfig(inner) => {
                    <systemConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::anchorStateRegistry(inner) => {
                    <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkWithdrawal(inner) => {
                    <checkWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositTransaction(inner) => {
                    <depositTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disputeGameBlacklist(inner) => {
                    <disputeGameBlacklistCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::disputeGameFinalityDelaySeconds(inner) => {
                    <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::donateETH(inner) => {
                    <donateETHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ethLockbox(inner) => {
                    <ethLockboxCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizeWithdrawalTransaction(inner) => {
                    <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizeWithdrawalTransactionExternalProof(inner) => {
                    <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizedWithdrawals(inner) => {
                    <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::l2Sender(inner) => {
                    <l2SenderCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::numProofSubmitters(inner) => {
                    <numProofSubmittersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::params(inner) => {
                    <paramsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proofMaturityDelaySeconds(inner) => {
                    <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proofSubmitters(inner) => {
                    <proofSubmittersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proveWithdrawalTransaction(inner) => {
                    <proveWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::provenWithdrawals(inner) => {
                    <provenWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::respectedGameType(inner) => {
                    <respectedGameTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::respectedGameTypeUpdatedAt(inner) => {
                    <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::systemConfig(inner) => {
                    <systemConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`OptimismPortal2`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OptimismPortal2Errors {
        #[allow(missing_docs)]
        ContentLengthMismatch(ContentLengthMismatch),
        #[allow(missing_docs)]
        EmptyItem(EmptyItem),
        #[allow(missing_docs)]
        InvalidDataRemainder(InvalidDataRemainder),
        #[allow(missing_docs)]
        InvalidHeader(InvalidHeader),
        #[allow(missing_docs)]
        OptimismPortal_AlreadyFinalized(OptimismPortal_AlreadyFinalized),
        #[allow(missing_docs)]
        OptimismPortal_BadTarget(OptimismPortal_BadTarget),
        #[allow(missing_docs)]
        OptimismPortal_CallPaused(OptimismPortal_CallPaused),
        #[allow(missing_docs)]
        OptimismPortal_CalldataTooLarge(OptimismPortal_CalldataTooLarge),
        #[allow(missing_docs)]
        OptimismPortal_GasEstimation(OptimismPortal_GasEstimation),
        #[allow(missing_docs)]
        OptimismPortal_GasLimitTooLow(OptimismPortal_GasLimitTooLow),
        #[allow(missing_docs)]
        OptimismPortal_ImproperDisputeGame(OptimismPortal_ImproperDisputeGame),
        #[allow(missing_docs)]
        OptimismPortal_InvalidDisputeGame(OptimismPortal_InvalidDisputeGame),
        #[allow(missing_docs)]
        OptimismPortal_InvalidLockboxState(OptimismPortal_InvalidLockboxState),
        #[allow(missing_docs)]
        OptimismPortal_InvalidMerkleProof(OptimismPortal_InvalidMerkleProof),
        #[allow(missing_docs)]
        OptimismPortal_InvalidOutputRootProof(OptimismPortal_InvalidOutputRootProof),
        #[allow(missing_docs)]
        OptimismPortal_InvalidProofTimestamp(OptimismPortal_InvalidProofTimestamp),
        #[allow(missing_docs)]
        OptimismPortal_InvalidRootClaim(OptimismPortal_InvalidRootClaim),
        #[allow(missing_docs)]
        OptimismPortal_NoReentrancy(OptimismPortal_NoReentrancy),
        #[allow(missing_docs)]
        OptimismPortal_NotAllowedOnCGTMode(OptimismPortal_NotAllowedOnCGTMode),
        #[allow(missing_docs)]
        OptimismPortal_ProofNotOldEnough(OptimismPortal_ProofNotOldEnough),
        #[allow(missing_docs)]
        OptimismPortal_Unproven(OptimismPortal_Unproven),
        #[allow(missing_docs)]
        OutOfGas(OutOfGas),
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
        UnexpectedList(UnexpectedList),
        #[allow(missing_docs)]
        UnexpectedString(UnexpectedString),
    }
    impl OptimismPortal2Errors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 92u8, 67u8, 20u8],
            [31u8, 249u8, 178u8, 228u8],
            [46u8, 87u8, 239u8, 58u8],
            [51u8, 33u8, 68u8, 219u8],
            [51u8, 42u8, 87u8, 248u8],
            [66u8, 97u8, 73u8, 175u8],
            [75u8, 156u8, 106u8, 190u8],
            [84u8, 228u8, 51u8, 205u8],
            [90u8, 163u8, 186u8, 201u8],
            [90u8, 180u8, 88u8, 251u8],
            [92u8, 85u8, 55u8, 184u8],
            [102u8, 201u8, 68u8, 133u8],
            [112u8, 200u8, 189u8, 189u8],
            [115u8, 10u8, 16u8, 116u8],
            [119u8, 235u8, 239u8, 77u8],
            [127u8, 18u8, 198u8, 75u8],
            [155u8, 1u8, 175u8, 237u8],
            [156u8, 70u8, 205u8, 121u8],
            [171u8, 88u8, 16u8, 54u8],
            [180u8, 202u8, 164u8, 229u8],
            [185u8, 195u8, 194u8, 239u8],
            [186u8, 187u8, 1u8, 221u8],
            [189u8, 88u8, 224u8, 162u8],
            [196u8, 5u8, 10u8, 38u8],
            [197u8, 222u8, 251u8, 173u8],
            [204u8, 166u8, 175u8, 218u8],
            [217u8, 188u8, 1u8, 190u8],
            [223u8, 234u8, 174u8, 184u8],
            [226u8, 153u8, 39u8, 237u8],
            [232u8, 24u8, 220u8, 195u8],
            [243u8, 149u8, 36u8, 14u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ProxyAdminOwnedBase_NotSharedProxyAdminOwner),
            ::core::stringify!(UnexpectedList),
            ::core::stringify!(OptimismPortal_InvalidMerkleProof),
            ::core::stringify!(ProxyAdminOwnedBase_ProxyAdminNotFound),
            ::core::stringify!(OptimismPortal_InvalidRootClaim),
            ::core::stringify!(OptimismPortal_InvalidOutputRootProof),
            ::core::stringify!(UnexpectedString),
            ::core::stringify!(ProxyAdminOwnedBase_NotResolvedDelegateProxy),
            ::core::stringify!(OptimismPortal_CalldataTooLarge),
            ::core::stringify!(EmptyItem),
            ::core::stringify!(InvalidDataRemainder),
            ::core::stringify!(ContentLengthMismatch),
            ::core::stringify!(OptimismPortal_GasLimitTooLow),
            ::core::stringify!(OptimismPortal_AlreadyFinalized),
            ::core::stringify!(OutOfGas),
            ::core::stringify!(ProxyAdminOwnedBase_NotProxyAdminOwner),
            ::core::stringify!(ReinitializableBase_ZeroInitVersion),
            ::core::stringify!(OptimismPortal_InvalidLockboxState),
            ::core::stringify!(OptimismPortal_GasEstimation),
            ::core::stringify!(OptimismPortal_InvalidProofTimestamp),
            ::core::stringify!(OptimismPortal_CallPaused),
            ::core::stringify!(InvalidHeader),
            ::core::stringify!(OptimismPortal_NotAllowedOnCGTMode),
            ::core::stringify!(ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner),
            ::core::stringify!(OptimismPortal_BadTarget),
            ::core::stringify!(OptimismPortal_Unproven),
            ::core::stringify!(OptimismPortal_ProofNotOldEnough),
            ::core::stringify!(OptimismPortal_NoReentrancy),
            ::core::stringify!(OptimismPortal_InvalidDisputeGame),
            ::core::stringify!(ProxyAdminOwnedBase_NotProxyAdmin),
            ::core::stringify!(OptimismPortal_ImproperDisputeGame),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedList as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedString as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyItem as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDataRemainder as alloy_sol_types::SolError>::SIGNATURE,
            <ContentLengthMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::SIGNATURE,
            <OutOfGas as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidLockboxState as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_CallPaused as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHeader as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_NotAllowedOnCGTMode as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_BadTarget as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_Unproven as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OptimismPortal2Errors {
        const NAME: &'static str = "OptimismPortal2Errors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 31usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ContentLengthMismatch(_) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyItem(_) => <EmptyItem as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidDataRemainder(_) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHeader(_) => {
                    <InvalidHeader as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_AlreadyFinalized(_) => {
                    <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_BadTarget(_) => {
                    <OptimismPortal_BadTarget as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_CallPaused(_) => {
                    <OptimismPortal_CallPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_CalldataTooLarge(_) => {
                    <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_GasEstimation(_) => {
                    <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_GasLimitTooLow(_) => {
                    <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_ImproperDisputeGame(_) => {
                    <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidDisputeGame(_) => {
                    <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidLockboxState(_) => {
                    <OptimismPortal_InvalidLockboxState as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidMerkleProof(_) => {
                    <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidOutputRootProof(_) => {
                    <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidProofTimestamp(_) => {
                    <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidRootClaim(_) => {
                    <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_NoReentrancy(_) => {
                    <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_NotAllowedOnCGTMode(_) => {
                    <OptimismPortal_NotAllowedOnCGTMode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_ProofNotOldEnough(_) => {
                    <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_Unproven(_) => {
                    <OptimismPortal_Unproven as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfGas(_) => <OutOfGas as alloy_sol_types::SolError>::SELECTOR,
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
                Self::UnexpectedList(_) => {
                    <UnexpectedList as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnexpectedString(_) => {
                    <UnexpectedString as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<OptimismPortal2Errors>] = &[
                {
                    fn ProxyAdminOwnedBase_NotSharedProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotSharedProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotSharedProxyAdminOwner
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn OptimismPortal_InvalidMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidMerkleProof,
                            )
                    }
                    OptimismPortal_InvalidMerkleProof
                },
                {
                    fn ProxyAdminOwnedBase_ProxyAdminNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_ProxyAdminNotFound,
                            )
                    }
                    ProxyAdminOwnedBase_ProxyAdminNotFound
                },
                {
                    fn OptimismPortal_InvalidRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_InvalidRootClaim)
                    }
                    OptimismPortal_InvalidRootClaim
                },
                {
                    fn OptimismPortal_InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidOutputRootProof,
                            )
                    }
                    OptimismPortal_InvalidOutputRootProof
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn ProxyAdminOwnedBase_NotResolvedDelegateProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotResolvedDelegateProxy,
                            )
                    }
                    ProxyAdminOwnedBase_NotResolvedDelegateProxy
                },
                {
                    fn OptimismPortal_CalldataTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_CalldataTooLarge)
                    }
                    OptimismPortal_CalldataTooLarge
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OptimismPortal2Errors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn OptimismPortal_GasLimitTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_GasLimitTooLow)
                    }
                    OptimismPortal_GasLimitTooLow
                },
                {
                    fn OptimismPortal_AlreadyFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_AlreadyFinalized)
                    }
                    OptimismPortal_AlreadyFinalized
                },
                {
                    fn OutOfGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OutOfGas as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OptimismPortal2Errors::OutOfGas)
                    }
                    OutOfGas
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOwner
                },
                {
                    fn ReinitializableBase_ZeroInitVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ReinitializableBase_ZeroInitVersion,
                            )
                    }
                    ReinitializableBase_ZeroInitVersion
                },
                {
                    fn OptimismPortal_InvalidLockboxState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidLockboxState as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidLockboxState,
                            )
                    }
                    OptimismPortal_InvalidLockboxState
                },
                {
                    fn OptimismPortal_GasEstimation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_GasEstimation)
                    }
                    OptimismPortal_GasEstimation
                },
                {
                    fn OptimismPortal_InvalidProofTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidProofTimestamp,
                            )
                    }
                    OptimismPortal_InvalidProofTimestamp
                },
                {
                    fn OptimismPortal_CallPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_CallPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_CallPaused)
                    }
                    OptimismPortal_CallPaused
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn OptimismPortal_NotAllowedOnCGTMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_NotAllowedOnCGTMode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_NotAllowedOnCGTMode,
                            )
                    }
                    OptimismPortal_NotAllowedOnCGTMode
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner
                },
                {
                    fn OptimismPortal_BadTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_BadTarget as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_BadTarget)
                    }
                    OptimismPortal_BadTarget
                },
                {
                    fn OptimismPortal_Unproven(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_Unproven as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_Unproven)
                    }
                    OptimismPortal_Unproven
                },
                {
                    fn OptimismPortal_ProofNotOldEnough(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_ProofNotOldEnough)
                    }
                    OptimismPortal_ProofNotOldEnough
                },
                {
                    fn OptimismPortal_NoReentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_NoReentrancy)
                    }
                    OptimismPortal_NoReentrancy
                },
                {
                    fn OptimismPortal_InvalidDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidDisputeGame,
                            )
                    }
                    OptimismPortal_InvalidDisputeGame
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotProxyAdmin,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdmin
                },
                {
                    fn OptimismPortal_ImproperDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_ImproperDisputeGame,
                            )
                    }
                    OptimismPortal_ImproperDisputeGame
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
            ) -> alloy_sol_types::Result<OptimismPortal2Errors>] = &[
                {
                    fn ProxyAdminOwnedBase_NotSharedProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotSharedProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotSharedProxyAdminOwner
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn OptimismPortal_InvalidMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidMerkleProof,
                            )
                    }
                    OptimismPortal_InvalidMerkleProof
                },
                {
                    fn ProxyAdminOwnedBase_ProxyAdminNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_ProxyAdminNotFound,
                            )
                    }
                    ProxyAdminOwnedBase_ProxyAdminNotFound
                },
                {
                    fn OptimismPortal_InvalidRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_InvalidRootClaim)
                    }
                    OptimismPortal_InvalidRootClaim
                },
                {
                    fn OptimismPortal_InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidOutputRootProof,
                            )
                    }
                    OptimismPortal_InvalidOutputRootProof
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn ProxyAdminOwnedBase_NotResolvedDelegateProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotResolvedDelegateProxy,
                            )
                    }
                    ProxyAdminOwnedBase_NotResolvedDelegateProxy
                },
                {
                    fn OptimismPortal_CalldataTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_CalldataTooLarge)
                    }
                    OptimismPortal_CalldataTooLarge
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn OptimismPortal_GasLimitTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_GasLimitTooLow)
                    }
                    OptimismPortal_GasLimitTooLow
                },
                {
                    fn OptimismPortal_AlreadyFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_AlreadyFinalized)
                    }
                    OptimismPortal_AlreadyFinalized
                },
                {
                    fn OutOfGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OutOfGas as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OutOfGas)
                    }
                    OutOfGas
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOwner
                },
                {
                    fn ReinitializableBase_ZeroInitVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ReinitializableBase_ZeroInitVersion,
                            )
                    }
                    ReinitializableBase_ZeroInitVersion
                },
                {
                    fn OptimismPortal_InvalidLockboxState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidLockboxState as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidLockboxState,
                            )
                    }
                    OptimismPortal_InvalidLockboxState
                },
                {
                    fn OptimismPortal_GasEstimation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_GasEstimation)
                    }
                    OptimismPortal_GasEstimation
                },
                {
                    fn OptimismPortal_InvalidProofTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidProofTimestamp,
                            )
                    }
                    OptimismPortal_InvalidProofTimestamp
                },
                {
                    fn OptimismPortal_CallPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_CallPaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_CallPaused)
                    }
                    OptimismPortal_CallPaused
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn OptimismPortal_NotAllowedOnCGTMode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_NotAllowedOnCGTMode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_NotAllowedOnCGTMode,
                            )
                    }
                    OptimismPortal_NotAllowedOnCGTMode
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner
                },
                {
                    fn OptimismPortal_BadTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_BadTarget as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_BadTarget)
                    }
                    OptimismPortal_BadTarget
                },
                {
                    fn OptimismPortal_Unproven(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_Unproven as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_Unproven)
                    }
                    OptimismPortal_Unproven
                },
                {
                    fn OptimismPortal_ProofNotOldEnough(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_ProofNotOldEnough)
                    }
                    OptimismPortal_ProofNotOldEnough
                },
                {
                    fn OptimismPortal_NoReentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortal2Errors::OptimismPortal_NoReentrancy)
                    }
                    OptimismPortal_NoReentrancy
                },
                {
                    fn OptimismPortal_InvalidDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_InvalidDisputeGame,
                            )
                    }
                    OptimismPortal_InvalidDisputeGame
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::ProxyAdminOwnedBase_NotProxyAdmin,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdmin
                },
                {
                    fn OptimismPortal_ImproperDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortal2Errors> {
                        <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortal2Errors::OptimismPortal_ImproperDisputeGame,
                            )
                    }
                    OptimismPortal_ImproperDisputeGame
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
                Self::ContentLengthMismatch(inner) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyItem(inner) => {
                    <EmptyItem as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidDataRemainder(inner) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHeader(inner) => {
                    <InvalidHeader as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OptimismPortal_AlreadyFinalized(inner) => {
                    <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_BadTarget(inner) => {
                    <OptimismPortal_BadTarget as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_CallPaused(inner) => {
                    <OptimismPortal_CallPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_CalldataTooLarge(inner) => {
                    <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_GasEstimation(inner) => {
                    <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_GasLimitTooLow(inner) => {
                    <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_ImproperDisputeGame(inner) => {
                    <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidDisputeGame(inner) => {
                    <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidLockboxState(inner) => {
                    <OptimismPortal_InvalidLockboxState as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidMerkleProof(inner) => {
                    <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidOutputRootProof(inner) => {
                    <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidProofTimestamp(inner) => {
                    <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidRootClaim(inner) => {
                    <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_NoReentrancy(inner) => {
                    <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_NotAllowedOnCGTMode(inner) => {
                    <OptimismPortal_NotAllowedOnCGTMode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_ProofNotOldEnough(inner) => {
                    <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_Unproven(inner) => {
                    <OptimismPortal_Unproven as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfGas(inner) => {
                    <OutOfGas as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
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
                Self::UnexpectedList(inner) => {
                    <UnexpectedList as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnexpectedString(inner) => {
                    <UnexpectedString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ContentLengthMismatch(inner) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyItem(inner) => {
                    <EmptyItem as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidDataRemainder(inner) => {
                    <InvalidDataRemainder as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OptimismPortal_AlreadyFinalized(inner) => {
                    <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_BadTarget(inner) => {
                    <OptimismPortal_BadTarget as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_CallPaused(inner) => {
                    <OptimismPortal_CallPaused as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_CalldataTooLarge(inner) => {
                    <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_GasEstimation(inner) => {
                    <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_GasLimitTooLow(inner) => {
                    <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_ImproperDisputeGame(inner) => {
                    <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidDisputeGame(inner) => {
                    <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidLockboxState(inner) => {
                    <OptimismPortal_InvalidLockboxState as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidMerkleProof(inner) => {
                    <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidOutputRootProof(inner) => {
                    <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidProofTimestamp(inner) => {
                    <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidRootClaim(inner) => {
                    <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_NoReentrancy(inner) => {
                    <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_NotAllowedOnCGTMode(inner) => {
                    <OptimismPortal_NotAllowedOnCGTMode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_ProofNotOldEnough(inner) => {
                    <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_Unproven(inner) => {
                    <OptimismPortal_Unproven as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfGas(inner) => {
                    <OutOfGas as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
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
                Self::UnexpectedList(inner) => {
                    <UnexpectedList as alloy_sol_types::SolError>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`OptimismPortal2`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OptimismPortal2Events {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        TransactionDeposited(TransactionDeposited),
        #[allow(missing_docs)]
        WithdrawalFinalized(WithdrawalFinalized),
        #[allow(missing_docs)]
        WithdrawalProven(WithdrawalProven),
        #[allow(missing_docs)]
        WithdrawalProvenExtension1(WithdrawalProvenExtension1),
    }
    impl OptimismPortal2Events {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                103u8, 166u8, 32u8, 140u8, 252u8, 192u8, 128u8, 29u8, 80u8, 246u8, 203u8,
                231u8, 100u8, 115u8, 63u8, 79u8, 221u8, 246u8, 106u8, 192u8, 176u8, 68u8,
                66u8, 6u8, 26u8, 138u8, 140u8, 12u8, 182u8, 182u8, 63u8, 98u8,
            ],
            [
                121u8, 143u8, 159u8, 19u8, 105u8, 95u8, 143u8, 4u8, 90u8, 165u8, 248u8,
                14u8, 216u8, 239u8, 235u8, 182u8, 149u8, 243u8, 199u8, 254u8, 101u8,
                218u8, 56u8, 25u8, 105u8, 242u8, 242u8, 139u8, 243u8, 198u8, 11u8, 151u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                179u8, 129u8, 53u8, 104u8, 217u8, 153u8, 31u8, 201u8, 81u8, 150u8, 31u8,
                203u8, 76u8, 120u8, 72u8, 147u8, 87u8, 66u8, 64u8, 162u8, 137u8, 37u8,
                96u8, 77u8, 9u8, 252u8, 87u8, 124u8, 85u8, 187u8, 124u8, 50u8,
            ],
            [
                219u8, 92u8, 118u8, 82u8, 133u8, 122u8, 161u8, 99u8, 218u8, 173u8, 214u8,
                112u8, 225u8, 22u8, 98u8, 143u8, 180u8, 46u8, 134u8, 157u8, 138u8, 196u8,
                37u8, 30u8, 248u8, 151u8, 29u8, 158u8, 87u8, 39u8, 223u8, 27u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(WithdrawalProven),
            ::core::stringify!(WithdrawalProvenExtension1),
            ::core::stringify!(Initialized),
            ::core::stringify!(TransactionDeposited),
            ::core::stringify!(WithdrawalFinalized),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <WithdrawalProven as alloy_sol_types::SolEvent>::SIGNATURE,
            <WithdrawalProvenExtension1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionDeposited as alloy_sol_types::SolEvent>::SIGNATURE,
            <WithdrawalFinalized as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for OptimismPortal2Events {
        const NAME: &'static str = "OptimismPortal2Events";
        const COUNT: usize = 5usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <TransactionDeposited as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionDeposited as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TransactionDeposited)
                }
                Some(
                    <WithdrawalFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <WithdrawalFinalized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::WithdrawalFinalized)
                }
                Some(<WithdrawalProven as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <WithdrawalProven as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::WithdrawalProven)
                }
                Some(
                    <WithdrawalProvenExtension1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <WithdrawalProvenExtension1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::WithdrawalProvenExtension1)
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
    impl alloy_sol_types::private::IntoLogData for OptimismPortal2Events {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionDeposited(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalProven(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalProvenExtension1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionDeposited(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalProven(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalProvenExtension1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OptimismPortal2`](self) contract instance.

See the [wrapper's documentation](`OptimismPortal2Instance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OptimismPortal2Instance<P, N> {
        OptimismPortal2Instance::<P, N>::new(address, __provider)
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
        _proofMaturityDelaySeconds: alloy::sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OptimismPortal2Instance<P, N>>,
    > {
        OptimismPortal2Instance::<P, N>::deploy(__provider, _proofMaturityDelaySeconds)
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
        _proofMaturityDelaySeconds: alloy::sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        OptimismPortal2Instance::<
            P,
            N,
        >::deploy_builder(__provider, _proofMaturityDelaySeconds)
    }
    /**A [`OptimismPortal2`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OptimismPortal2`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OptimismPortal2Instance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OptimismPortal2Instance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OptimismPortal2Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OptimismPortal2Instance<P, N> {
        /**Creates a new wrapper around an on-chain [`OptimismPortal2`](self) contract instance.

See the [wrapper's documentation](`OptimismPortal2Instance`) for more details.*/
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
            _proofMaturityDelaySeconds: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::Result<OptimismPortal2Instance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                _proofMaturityDelaySeconds,
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
            _proofMaturityDelaySeconds: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _proofMaturityDelaySeconds,
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
    impl<P: ::core::clone::Clone, N> OptimismPortal2Instance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OptimismPortal2Instance<P, N> {
            OptimismPortal2Instance {
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
    > OptimismPortal2Instance<P, N> {
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
        ///Creates a new call builder for the [`anchorStateRegistry`] function.
        pub fn anchorStateRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, anchorStateRegistryCall, N> {
            self.call_builder(&anchorStateRegistryCall)
        }
        ///Creates a new call builder for the [`checkWithdrawal`] function.
        pub fn checkWithdrawal(
            &self,
            _withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
            _proofSubmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, checkWithdrawalCall, N> {
            self.call_builder(
                &checkWithdrawalCall {
                    _withdrawalHash,
                    _proofSubmitter,
                },
            )
        }
        ///Creates a new call builder for the [`depositTransaction`] function.
        pub fn depositTransaction(
            &self,
            _to: alloy::sol_types::private::Address,
            _value: alloy::sol_types::private::primitives::aliases::U256,
            _gasLimit: u64,
            _isCreation: bool,
            _data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, depositTransactionCall, N> {
            self.call_builder(
                &depositTransactionCall {
                    _to,
                    _value,
                    _gasLimit,
                    _isCreation,
                    _data,
                },
            )
        }
        ///Creates a new call builder for the [`disputeGameBlacklist`] function.
        pub fn disputeGameBlacklist(
            &self,
            _disputeGame: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, disputeGameBlacklistCall, N> {
            self.call_builder(
                &disputeGameBlacklistCall {
                    _disputeGame,
                },
            )
        }
        ///Creates a new call builder for the [`disputeGameFactory`] function.
        pub fn disputeGameFactory(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, disputeGameFactoryCall, N> {
            self.call_builder(&disputeGameFactoryCall)
        }
        ///Creates a new call builder for the [`disputeGameFinalityDelaySeconds`] function.
        pub fn disputeGameFinalityDelaySeconds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, disputeGameFinalityDelaySecondsCall, N> {
            self.call_builder(&disputeGameFinalityDelaySecondsCall)
        }
        ///Creates a new call builder for the [`donateETH`] function.
        pub fn donateETH(&self) -> alloy_contract::SolCallBuilder<&P, donateETHCall, N> {
            self.call_builder(&donateETHCall)
        }
        ///Creates a new call builder for the [`ethLockbox`] function.
        pub fn ethLockbox(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ethLockboxCall, N> {
            self.call_builder(&ethLockboxCall)
        }
        ///Creates a new call builder for the [`finalizeWithdrawalTransaction`] function.
        pub fn finalizeWithdrawalTransaction(
            &self,
            _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, finalizeWithdrawalTransactionCall, N> {
            self.call_builder(
                &finalizeWithdrawalTransactionCall {
                    _tx,
                },
            )
        }
        ///Creates a new call builder for the [`finalizeWithdrawalTransactionExternalProof`] function.
        pub fn finalizeWithdrawalTransactionExternalProof(
            &self,
            _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
            _proofSubmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            finalizeWithdrawalTransactionExternalProofCall,
            N,
        > {
            self.call_builder(
                &finalizeWithdrawalTransactionExternalProofCall {
                    _tx,
                    _proofSubmitter,
                },
            )
        }
        ///Creates a new call builder for the [`finalizedWithdrawals`] function.
        pub fn finalizedWithdrawals(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, finalizedWithdrawalsCall, N> {
            self.call_builder(&finalizedWithdrawalsCall(_0))
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
            _systemConfig: alloy::sol_types::private::Address,
            _anchorStateRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _systemConfig,
                    _anchorStateRegistry,
                },
            )
        }
        ///Creates a new call builder for the [`l2Sender`] function.
        pub fn l2Sender(&self) -> alloy_contract::SolCallBuilder<&P, l2SenderCall, N> {
            self.call_builder(&l2SenderCall)
        }
        ///Creates a new call builder for the [`minimumGasLimit`] function.
        pub fn minimumGasLimit(
            &self,
            _byteCount: u64,
        ) -> alloy_contract::SolCallBuilder<&P, minimumGasLimitCall, N> {
            self.call_builder(&minimumGasLimitCall { _byteCount })
        }
        ///Creates a new call builder for the [`numProofSubmitters`] function.
        pub fn numProofSubmitters(
            &self,
            _withdrawalHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, numProofSubmittersCall, N> {
            self.call_builder(
                &numProofSubmittersCall {
                    _withdrawalHash,
                },
            )
        }
        ///Creates a new call builder for the [`params`] function.
        pub fn params(&self) -> alloy_contract::SolCallBuilder<&P, paramsCall, N> {
            self.call_builder(&paramsCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`proofMaturityDelaySeconds`] function.
        pub fn proofMaturityDelaySeconds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proofMaturityDelaySecondsCall, N> {
            self.call_builder(&proofMaturityDelaySecondsCall)
        }
        ///Creates a new call builder for the [`proofSubmitters`] function.
        pub fn proofSubmitters(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, proofSubmittersCall, N> {
            self.call_builder(&proofSubmittersCall { _0, _1 })
        }
        ///Creates a new call builder for the [`proveWithdrawalTransaction`] function.
        pub fn proveWithdrawalTransaction(
            &self,
            _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
            _disputeGameIndex: alloy::sol_types::private::primitives::aliases::U256,
            _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
            _withdrawalProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, proveWithdrawalTransactionCall, N> {
            self.call_builder(
                &proveWithdrawalTransactionCall {
                    _tx,
                    _disputeGameIndex,
                    _outputRootProof,
                    _withdrawalProof,
                },
            )
        }
        ///Creates a new call builder for the [`provenWithdrawals`] function.
        pub fn provenWithdrawals(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, provenWithdrawalsCall, N> {
            self.call_builder(&provenWithdrawalsCall { _0, _1 })
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
        ///Creates a new call builder for the [`respectedGameType`] function.
        pub fn respectedGameType(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, respectedGameTypeCall, N> {
            self.call_builder(&respectedGameTypeCall)
        }
        ///Creates a new call builder for the [`respectedGameTypeUpdatedAt`] function.
        pub fn respectedGameTypeUpdatedAt(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, respectedGameTypeUpdatedAtCall, N> {
            self.call_builder(&respectedGameTypeUpdatedAtCall)
        }
        ///Creates a new call builder for the [`superchainConfig`] function.
        pub fn superchainConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, superchainConfigCall, N> {
            self.call_builder(&superchainConfigCall)
        }
        ///Creates a new call builder for the [`systemConfig`] function.
        pub fn systemConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, systemConfigCall, N> {
            self.call_builder(&systemConfigCall)
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
    > OptimismPortal2Instance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`TransactionDeposited`] event.
        pub fn TransactionDeposited_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionDeposited, N> {
            self.event_filter::<TransactionDeposited>()
        }
        ///Creates a new event filter for the [`WithdrawalFinalized`] event.
        pub fn WithdrawalFinalized_filter(
            &self,
        ) -> alloy_contract::Event<&P, WithdrawalFinalized, N> {
            self.event_filter::<WithdrawalFinalized>()
        }
        ///Creates a new event filter for the [`WithdrawalProven`] event.
        pub fn WithdrawalProven_filter(
            &self,
        ) -> alloy_contract::Event<&P, WithdrawalProven, N> {
            self.event_filter::<WithdrawalProven>()
        }
        ///Creates a new event filter for the [`WithdrawalProvenExtension1`] event.
        pub fn WithdrawalProvenExtension1_filter(
            &self,
        ) -> alloy_contract::Event<&P, WithdrawalProvenExtension1, N> {
            self.event_filter::<WithdrawalProvenExtension1>()
        }
    }
}

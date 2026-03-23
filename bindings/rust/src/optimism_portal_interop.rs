///Module containing a contract's types and functions.
/**

```solidity
library Types {
    struct OutputRootProof { bytes32 version; bytes32 stateRoot; bytes32 messagePasserStorageRoot; bytes32 latestBlockhash; }
    struct OutputRootWithChainId { uint256 chainId; bytes32 root; }
    struct SuperRootProof { bytes1 version; uint64 timestamp; OutputRootWithChainId[] outputRoots; }
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
struct OutputRootWithChainId { uint256 chainId; bytes32 root; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutputRootWithChainId {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<OutputRootWithChainId> for UnderlyingRustTuple<'_> {
            fn from(value: OutputRootWithChainId) -> Self {
                (value.chainId, value.root)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutputRootWithChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    root: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OutputRootWithChainId {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OutputRootWithChainId {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
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
        impl alloy_sol_types::SolType for OutputRootWithChainId {
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
        impl alloy_sol_types::SolStruct for OutputRootWithChainId {
            const NAME: &'static str = "OutputRootWithChainId";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OutputRootWithChainId(uint256 chainId,bytes32 root)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.chainId)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.root)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OutputRootWithChainId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.root)
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
                    &rust.chainId,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.root,
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
struct SuperRootProof { bytes1 version; uint64 timestamp; OutputRootWithChainId[] outputRoots; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SuperRootProof {
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub timestamp: u64,
        #[allow(missing_docs)]
        pub outputRoots: alloy::sol_types::private::Vec<
            <OutputRootWithChainId as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::FixedBytes<1>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Array<OutputRootWithChainId>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<1>,
            u64,
            alloy::sol_types::private::Vec<
                <OutputRootWithChainId as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<SuperRootProof> for UnderlyingRustTuple<'_> {
            fn from(value: SuperRootProof) -> Self {
                (value.version, value.timestamp, value.outputRoots)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SuperRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    version: tuple.0,
                    timestamp: tuple.1,
                    outputRoots: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SuperRootProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SuperRootProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        1,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                    <alloy::sol_types::sol_data::Array<
                        OutputRootWithChainId,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputRoots),
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
        impl alloy_sol_types::SolType for SuperRootProof {
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
        impl alloy_sol_types::SolStruct for SuperRootProof {
            const NAME: &'static str = "SuperRootProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SuperRootProof(bytes1 version,uint64 timestamp,OutputRootWithChainId[] outputRoots)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <OutputRootWithChainId as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OutputRootWithChainId as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        1,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.version)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timestamp)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OutputRootWithChainId,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.outputRoots)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SuperRootProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        1,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.version,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timestamp,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OutputRootWithChainId,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputRoots,
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
                    1,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.version,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OutputRootWithChainId,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outputRoots,
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
    struct OutputRootWithChainId {
        uint256 chainId;
        bytes32 root;
    }
    struct SuperRootProof {
        bytes1 version;
        uint64 timestamp;
        OutputRootWithChainId[] outputRoots;
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

interface OptimismPortalInterop {
    type GameType is uint32;

    error ContentLengthMismatch();
    error EmptyItem();
    error Encoding_EmptySuperRoot();
    error Encoding_InvalidSuperRootVersion();
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
    error OptimismPortal_InvalidMerkleProof();
    error OptimismPortal_InvalidOutputRootChainId();
    error OptimismPortal_InvalidOutputRootIndex();
    error OptimismPortal_InvalidOutputRootProof();
    error OptimismPortal_InvalidProofTimestamp();
    error OptimismPortal_InvalidRootClaim();
    error OptimismPortal_InvalidSuperRootProof();
    error OptimismPortal_MigratingToSameRegistry();
    error OptimismPortal_NoReentrancy();
    error OptimismPortal_ProofNotOldEnough();
    error OptimismPortal_Unproven();
    error OptimismPortal_WrongProofMethod();
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

    event ETHMigrated(address indexed lockbox, uint256 ethBalance);
    event Initialized(uint8 version);
    event PortalMigrated(address oldLockbox, address newLockbox, address oldAnchorStateRegistry, address newAnchorStateRegistry);
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
    function initialize(address _systemConfig, address _anchorStateRegistry, address _ethLockbox) external;
    function l2Sender() external view returns (address);
    function migrateLiquidity() external;
    function migrateToSuperRoots(address _newLockbox, address _newAnchorStateRegistry) external;
    function minimumGasLimit(uint64 _byteCount) external pure returns (uint64);
    function numProofSubmitters(bytes32 _withdrawalHash) external view returns (uint256);
    function params() external view returns (uint128 prevBaseFee, uint64 prevBoughtGas, uint64 prevBlockNum);
    function paused() external view returns (bool);
    function proofMaturityDelaySeconds() external view returns (uint256);
    function proofSubmitters(bytes32, uint256) external view returns (address);
    function proveWithdrawalTransaction(Types.WithdrawalTransaction memory _tx, uint256 _disputeGameIndex, Types.OutputRootProof memory _outputRootProof, bytes[] memory _withdrawalProof) external;
    function proveWithdrawalTransaction(Types.WithdrawalTransaction memory _tx, address _disputeGameProxy, uint256 _outputRootIndex, Types.SuperRootProof memory _superRootProof, Types.OutputRootProof memory _outputRootProof, bytes[] memory _withdrawalProof) external;
    function provenWithdrawals(bytes32, address) external view returns (address disputeGameProxy, uint64 timestamp);
    function proxyAdmin() external view returns (address);
    function proxyAdminOwner() external view returns (address);
    function respectedGameType() external view returns (GameType);
    function respectedGameTypeUpdatedAt() external view returns (uint64);
    function superRootsActive() external view returns (bool);
    function superchainConfig() external view returns (address);
    function systemConfig() external view returns (address);
    function upgrade(address _anchorStateRegistry, address _ethLockbox) external;
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
      },
      {
        "name": "_ethLockbox",
        "type": "address",
        "internalType": "contract IETHLockbox"
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
    "name": "migrateLiquidity",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "migrateToSuperRoots",
    "inputs": [
      {
        "name": "_newLockbox",
        "type": "address",
        "internalType": "contract IETHLockbox"
      },
      {
        "name": "_newAnchorStateRegistry",
        "type": "address",
        "internalType": "contract IAnchorStateRegistry"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
        "name": "_disputeGameProxy",
        "type": "address",
        "internalType": "contract IDisputeGame"
      },
      {
        "name": "_outputRootIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_superRootProof",
        "type": "tuple",
        "internalType": "struct Types.SuperRootProof",
        "components": [
          {
            "name": "version",
            "type": "bytes1",
            "internalType": "bytes1"
          },
          {
            "name": "timestamp",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "outputRoots",
            "type": "tuple[]",
            "internalType": "struct Types.OutputRootWithChainId[]",
            "components": [
              {
                "name": "chainId",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "root",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
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
    "name": "superRootsActive",
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
    "name": "upgrade",
    "inputs": [
      {
        "name": "_anchorStateRegistry",
        "type": "address",
        "internalType": "contract IAnchorStateRegistry"
      },
      {
        "name": "_ethLockbox",
        "type": "address",
        "internalType": "contract IETHLockbox"
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
    "stateMutability": "pure"
  },
  {
    "type": "event",
    "name": "ETHMigrated",
    "inputs": [
      {
        "name": "lockbox",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "ethBalance",
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
    "name": "PortalMigrated",
    "inputs": [
      {
        "name": "oldLockbox",
        "type": "address",
        "indexed": false,
        "internalType": "contract IETHLockbox"
      },
      {
        "name": "newLockbox",
        "type": "address",
        "indexed": false,
        "internalType": "contract IETHLockbox"
      },
      {
        "name": "oldAnchorStateRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IAnchorStateRegistry"
      },
      {
        "name": "newAnchorStateRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IAnchorStateRegistry"
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
    "name": "Encoding_EmptySuperRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Encoding_InvalidSuperRootVersion",
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
    "name": "OptimismPortal_InvalidMerkleProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidOutputRootChainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_InvalidOutputRootIndex",
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
    "name": "OptimismPortal_InvalidSuperRootProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_MigratingToSameRegistry",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OptimismPortal_NoReentrancy",
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
    "name": "OptimismPortal_WrongProofMethod",
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
pub mod OptimismPortalInterop {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c06040523480156200001157600080fd5b506040516200602838038062006028833981016040819052620000349162000111565b600460805260a0819052620000486200004f565b506200012b565b600054610100900460ff1615620000bc5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156200010f576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6000602082840312156200012457600080fd5b5051919050565b60805160a051615ec26200016660003960008181610713015261179d01526000818161030401528181611a000152611d020152615ec26000f3fe6080604052600436106102385760003560e01c80638c3152e911610138578063bb2c727e116100b0578063cff0ab961161007f578063dad544e011610064578063dad544e01461082a578063e9e05c421461083f578063f2b4e6171461085257600080fd5b8063cff0ab9614610757578063d325d3bf146107f857600080fd5b8063bb2c727e14610648578063bda204bb146106ef578063bf653a5c14610704578063c0c53b8b1461073757600080fd5b80639bf62d8211610107578063a35d99df116100ec578063a35d99df146105db578063a3860f48146105fb578063b682c4441461061b57600080fd5b80639bf62d821461057e578063a14238e7146105ab57600080fd5b80638c3152e9146105095780638c90dd6514610529578063952b27971461054957806399a88ec41461055e57600080fd5b806345884d32116101cb57806354fd4d501161019a5780635c975abb1161017f5780635c975abb146104d457806371c1566e146104e95780638b4c40b01461025d57600080fd5b806354fd4d501461045b5780635c0cba33146104a757600080fd5b806345884d32146103a25780634870496f146103d25780634fd0434c146103f2578063513747ab1461042057600080fd5b80633c9f397c116102075780633c9f397c1461032e5780633e47158c1461035857806343ca1c501461036d578063452a93201461038d57600080fd5b80632152f2be1461026457806333d7e2bd1461028457806335e80ab3146102db57806338d38c97146102f057600080fd5b3661025f5761025d3334620186a0600060405180602001604052806000815250610867565b005b600080fd5b34801561027057600080fd5b5061025d61027f366004614f47565b610ab3565b34801561029057600080fd5b506037546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b3480156102e757600080fd5b506102b1610bf9565b3480156102fc57600080fd5b5060405160ff7f00000000000000000000000000000000000000000000000000000000000000001681526020016102d2565b34801561033a57600080fd5b50610343610c92565b60405163ffffffff90911681526020016102d2565b34801561036457600080fd5b506102b1610d26565b34801561037957600080fd5b5061025d610388366004615187565b610f31565b34801561039957600080fd5b506102b1611272565b3480156103ae57600080fd5b506103c26103bd3660046151ce565b6112e2565b60405190151581526020016102d2565b3480156103de57600080fd5b5061025d6103ed36600461524f565b61137d565b3480156103fe57600080fd5b506104076114ad565b60405167ffffffffffffffff90911681526020016102d2565b34801561042c57600080fd5b5061044d61043b3660046152d5565b6000908152603c602052604090205490565b6040519081526020016102d2565b34801561046757600080fd5b50604080518082018252600d81527f352e312e302b696e7465726f7000000000000000000000000000000000000000602082015290516102d29190615364565b3480156104b357600080fd5b50603e546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b3480156104e057600080fd5b506103c2611541565b3480156104f557600080fd5b5061025d610504366004615377565b6115d5565b34801561051557600080fd5b5061025d61052436600461539c565b6118d9565b34801561053557600080fd5b5061025d6105443660046153d9565b6118e6565b34801561055557600080fd5b5061044d61196a565b34801561056a57600080fd5b5061025d610579366004614f47565b6119fe565b34801561058a57600080fd5b506032546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b3480156105b757600080fd5b506103c26105c63660046152d5565b60336020526000908152604090205460ff1681565b3480156105e757600080fd5b506104076105f63660046154b9565b611bc0565b34801561060757600080fd5b506102b16106163660046154d6565b611bd9565b34801561062757600080fd5b50603f546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b34801561065457600080fd5b506106ba610663366004615377565b603960209081526000928352604080842090915290825290205473ffffffffffffffffffffffffffffffffffffffff81169074010000000000000000000000000000000000000000900467ffffffffffffffff1682565b6040805173ffffffffffffffffffffffffffffffffffffffff909316835267ffffffffffffffff9091166020830152016102d2565b3480156106fb57600080fd5b5061025d611c1e565b34801561071057600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061044d565b34801561074357600080fd5b5061025d6107523660046154f8565b611d00565b34801561076357600080fd5b506001546107bf906fffffffffffffffffffffffffffffffff81169067ffffffffffffffff7001000000000000000000000000000000008204811691780100000000000000000000000000000000000000000000000090041683565b604080516fffffffffffffffffffffffffffffffff909416845267ffffffffffffffff92831660208501529116908201526060016102d2565b34801561080457600080fd5b50603f546103c29074010000000000000000000000000000000000000000900460ff1681565b34801561083657600080fd5b506102b1611efd565b61025d61084d366004615551565b610867565b34801561085e57600080fd5b506102b1611f51565b8260005a905034156108f757603f60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16631ee116bf346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156108dd57600080fd5b505af11580156108f1573d6000803e3d6000fd5b50505050505b838015610919575073ffffffffffffffffffffffffffffffffffffffff871615155b15610950576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61095a8351611bc0565b67ffffffffffffffff168567ffffffffffffffff1610156109a7576040517f70c8bdbd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6201d4c0835111156109e5576040517f5aa3bac900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336109ee611fc1565b610a0b575033731111000000000000000000000000000000001111015b60003488888888604051602001610a269594939291906155d0565b604051602081830303815290604052905060008973ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff167fb3813568d9991fc951961fcb4c784893574240a28925604d09fc577c55bb7c3284604051610a969190615364565b60405180910390a45050610aaa8282611fff565b50505050505050565b610abb6122d6565b610ac3612315565b603e5473ffffffffffffffffffffffffffffffffffffffff808316911603610b17576040517f785df91100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603f8054603e805473ffffffffffffffffffffffffffffffffffffffff8581167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681179093557fffffffffffffffffffffff0000000000000000000000000000000000000000008416878216908117740100000000000000000000000000000000000000001790955560408051948216808652602086019690965291169083018190526060830191909152907f9e5368471a58d81987e5dc7d6374dd5ed5e756cc95a79ff726903423bce0060d906080015b60405180910390a150505050565b603754604080517f35e80ab3000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff16916335e80ab39160048083019260209291908290030181865afa158015610c69573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d9190615635565b905090565b603e54604080517f3c9f397c000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff1691633c9f397c9160048083019260209291908290030181865afa158015610d02573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d9190615664565b600080610d517fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035490565b905073ffffffffffffffffffffffffffffffffffffffff811615610d7457919050565b6040518060400160405280601a81526020017f4f564d5f4c3143726f7373446f6d61696e4d657373656e676572000000000000815250516002610db791906156b0565b604080513060208201526000918101919091527f4f564d5f4c3143726f7373446f6d61696e4d657373656e6765720000000000009190911790610e12906060015b604051602081830303815290604052805190602001205490565b14610e49576040517f54e433cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408051306020820152600191810191909152600090610e6b90606001610df8565b905073ffffffffffffffffffffffffffffffffffffffff811615610eff578073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ed4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ef89190615635565b9250505090565b6040517f332144db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610f396122d6565b60325473ffffffffffffffffffffffffffffffffffffffff1661dead14610f8c576040517fdfeaaeb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610f99826040015161236b565b15610fd0576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610fdb836123ae565b9050610fe781836115d5565b600081815260336020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790556060830151156110b857603f5460608401516040517f8d445bd000000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90921691638d445bd0916110859160040190815260200190565b600060405180830381600087803b15801561109f57600080fd5b505af11580156110b3573d6000803e3d6000fd5b505050505b8260200151603260006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600061111b8460400151856080015186606001518760a001516123fb565b603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead17905560405190915082907fdb5c7652857aa163daadd670e116628fb42e869d8ac4251ef8971d9e5727df1b9061118090841515815260200190565b60405180910390a28015801561119a575060008460600151115b1561122757603f60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16631ee116bf85606001516040518263ffffffff1660e01b81526004016000604051808303818588803b15801561120d57600080fd5b505af1158015611221573d6000803e3d6000fd5b50505050505b801580156112355750326001145b1561126c576040517fab58103600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505050565b603754604080517f452a9320000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163452a93209160048083019260209291908290030181865afa158015610c69573d6000803e3d6000fd5b603e546040517f45884d3200000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff838116600483015260009216906345884d3290602401602060405180830381865afa158015611353573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061137791906156ed565b92915050565b6113856122d6565b603f5474010000000000000000000000000000000000000000900460ff16156113da576040517f5e74b54200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006113e4611f51565b73ffffffffffffffffffffffffffffffffffffffff1663bb8aa1fc866040518263ffffffff1660e01b815260040161141e91815260200190565b606060405180830381865afa15801561143b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061145f919061570a565b6040805160608082018352600080835260208301529181019190915290935091506114879050565b610aaa878360008461149e368b90038b018b61574c565b6114a8898b6157d6565b612459565b603e54604080517f4086d183000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff1691634086d1839160048083019260209291908290030181865afa15801561151d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d919061584f565b603754604080517f5c975abb000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff1691635c975abb9160048083019260209291908290030181865afa1580156115b1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d91906156ed565b600082815260396020908152604080832073ffffffffffffffffffffffffffffffffffffffff85811685529083528184208251808401845290549182168082527401000000000000000000000000000000000000000090920467ffffffffffffffff1681850152868552603390935292205490919060ff1615611684576040517f730a107400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b816020015167ffffffffffffffff166000036116cc576040517fcca6afda00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61174b8173ffffffffffffffffffffffffffffffffffffffff1663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561171a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061173e919061584f565b67ffffffffffffffff1690565b67ffffffffffffffff16826020015167ffffffffffffffff161161179b576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000826020015167ffffffffffffffff16426117d6919061586c565b1161180d576040517fd9bc01be00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f6c4f446700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff838116600483015290911690636c4f446790602401602060405180830381865afa15801561187d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118a191906156ed565b61126c576040517f332a57f800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6118e38133610f31565b50565b6118ee6122d6565b603f5474010000000000000000000000000000000000000000900460ff16611942576040517f5e74b54200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610aaa87878761195188615883565b6119603689900389018961574c565b6114a887896157d6565b603e54604080517f952b2797000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163952b27979160048083019260209291908290030181865afa1580156119da573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d919061598d565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff16158015611a3e575060005460ff8083169116105b611acf576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a656400000000000000000000000000000000000060648201526084015b60405180910390fd5b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff831617610100179055611b08612cb4565b603e805473ffffffffffffffffffffffffffffffffffffffff8581167fffffffffffffffffffffffff000000000000000000000000000000000000000092831617909255603f805492851692909116919091179055600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b6000611bcd8260286159a6565b611377906152086159d6565b603c6020528160005260406000208181548110611bf557600080fd5b60009182526020909120015473ffffffffffffffffffffffffffffffffffffffff169150829050565b611c26612315565b603f54604080517f1ee116bf0000000000000000000000000000000000000000000000000000000081529051479273ffffffffffffffffffffffffffffffffffffffff1691631ee116bf91849160048082019260009290919082900301818588803b158015611c9457600080fd5b505af1158015611ca8573d6000803e3d6000fd5b5050603f5460405185815273ffffffffffffffffffffffffffffffffffffffff90911693507fd893f630c6867fa43689da9ae949ebf04cac24aad3b45c759d442ed3c32e3a379250602001905060405180910390a250565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff16158015611d40575060005460ff8083169116105b611dcc576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a65640000000000000000000000000000000000006064820152608401611ac6565b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff831617610100179055611e05612cb4565b6037805473ffffffffffffffffffffffffffffffffffffffff8087167fffffffffffffffffffffffff000000000000000000000000000000000000000092831617909255603e8054868416908316179055603f8054858416921691909117905560325416611e9a57603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead1790555b611ea2612d35565b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602001610beb565b6000611f07610d26565b73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c69573d6000803e3d6000fd5b603e54604080517ff2b4e617000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163f2b4e6179160048083019260209291908290030181865afa158015610c69573d6000803e3d6000fd5b6000323303611fd05750600190565b333b601703611ff857604051602081016040526020600082333c5160e81c62ef010014905090565b5060005b90565b600154600090612035907801000000000000000000000000000000000000000000000000900467ffffffffffffffff164361586c565b90506000612041612e48565b90506000816020015160ff16826000015163ffffffff166120629190615a31565b9050821561219957600154600090612099908390700100000000000000000000000000000000900467ffffffffffffffff16615a99565b90506000836040015160ff16836120b09190615b0d565b6001546120d09084906fffffffffffffffffffffffffffffffff16615b0d565b6120da9190615a31565b60015490915060009061212b906121049084906fffffffffffffffffffffffffffffffff16615bc9565b866060015163ffffffff168760a001516fffffffffffffffffffffffffffffffff16612f0e565b9050600186111561215a5761215761210482876040015160ff1660018a612152919061586c565b612f2d565b90505b6fffffffffffffffffffffffffffffffff16780100000000000000000000000000000000000000000000000067ffffffffffffffff4316021760015550505b600180548691906010906121cc908490700100000000000000000000000000000000900467ffffffffffffffff166159d6565b92506101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550816000015163ffffffff16600160000160109054906101000a900467ffffffffffffffff1667ffffffffffffffff161315612259576040517f77ebef4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600154600090612285906fffffffffffffffffffffffffffffffff1667ffffffffffffffff88166156b0565b9050600061229748633b9aca00612f82565b6122a19083615c3d565b905060005a6122b0908861586c565b9050808211156122cc576122cc6122c7828461586c565b612f99565b5050505050505050565b6122de611541565b156118d7576040517fb9c3c2ef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3361231e611efd565b73ffffffffffffffffffffffffffffffffffffffff16146118d7576040517f7f12c64b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff8216301480611377575050603f5473ffffffffffffffffffffffffffffffffffffffff90811691161490565b80516020808301516040808501516060860151608087015160a088015193516000976123de979096959101615c51565b604051602081830303815290604052805190602001209050919050565b600080600061240b866000612fc7565b905080612441576308c379a06000526020805278185361666543616c6c3a204e6f7420656e6f756768206761736058526064601cfd5b600080855160208701888b5af1979650505050505050565b612466866040015161236b565b1561249d576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f496b9c1600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff87811660048301529091169063496b9c1690602401602060405180830381865afa15801561250d573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061253191906156ed565b612567576040517ff395240e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f04e50fed00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8781166004830152909116906304e50fed90602401602060405180830381865afa1580156125d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125fb91906156ed565b612631576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018573ffffffffffffffffffffffffffffffffffffffff1663200d2ed26040518163ffffffff1660e01b8152600401602060405180830381865afa15801561267e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126a29190615cd7565b60028111156126b3576126b3615ca8565b036126ea576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6127388573ffffffffffffffffffffffffffffffffffffffff1663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561171a573d6000803e3d6000fd5b67ffffffffffffffff16421161277a576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603f5474010000000000000000000000000000000000000000900460ff16156129c4576127a683612fe5565b6128188673ffffffffffffffffffffffffffffffffffffffff1663bcef3b556040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127f4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ffc919061598d565b1461284f576040517f2b1a9a6600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826040015151841061288d576040517f32dc285c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000836040015185815181106128a5576128a5615cf8565b60200260200101519050603760009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d6ae3cd56040518163ffffffff1660e01b8152600401602060405180830381865afa15801561291c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612940919061598d565b815114612979576040517f7cc2f31b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61298283612ffe565b8160200151146129be576040517f426149af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50612a52565b6129cd82612ffe565b612a1b8673ffffffffffffffffffffffffffffffffffffffff1663bcef3b556040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127f4573d6000803e3d6000fd5b14612a52576040517f426149af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612a5d876123ae565b90506000816000604051602001612a7e929190918252602082015260400190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081840301815282825280516020918201209083018190529250612b299101604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181528282018252600183527f0100000000000000000000000000000000000000000000000000000000000000602084015290870151909190869061303d565b1515600003612b64576040517f2e57ef3a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408051808201825273ffffffffffffffffffffffffffffffffffffffff808a16825267ffffffffffffffff42811660208085019182526000888152603982528681203380835290835287822096518754945190951674010000000000000000000000000000000000000000027fffffffff000000000000000000000000000000000000000000000000000000009094169486169490941792909217909455868152603c845284812080546001810182559082528482200180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169092179091558b840151928c01519351928216939091169185917f67a6208cfcc0801d50f6cbe764733f4fddf66ac0b04442061a8a8c0cb6b63f6291a4604051339083907f798f9f13695f8f045aa5f80ed8efebb695f3c7fe65da381969f2f28bf3c60b9790600090a35050505050505050565b33612cbd610d26565b73ffffffffffffffffffffffffffffffffffffffff1614158015612cfe575033612ce5611efd565b73ffffffffffffffffffffffffffffffffffffffff1614155b156118d7576040517fc4050a2600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600054610100900460ff16612dcc576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401611ac6565b6001547801000000000000000000000000000000000000000000000000900467ffffffffffffffff166000036118d75760408051606081018252633b9aca00808252600060208301524367ffffffffffffffff169190920181905278010000000000000000000000000000000000000000000000000217600155565b6040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a0810191909152603754604080517fcc731b02000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163cc731b029160048083019260c09291908290030181865afa158015612eea573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113779190615d3d565b6000612f23612f1d8585613061565b83613071565b90505b9392505050565b6000670de0b6b3a7640000612f6e612f458583615a31565b612f5790670de0b6b3a7640000615a99565b612f6985670de0b6b3a7640000615b0d565b613080565b612f789086615b0d565b612f239190615a31565b600081831015612f925781612f26565b5090919050565b6000805a90505b825a612fac908361586c565b1015612fc257612fbb82615df9565b9150612fa0565b505050565b600080603f83619c4001026040850201603f5a021015949350505050565b6000612ff0826130b1565b805190602001209050919050565b600081600001518260200151836040015184606001516040516020016123de949392919093845260208401929092526040830152606082015260800190565b60008061304986613267565b905061305781868686613299565b9695505050505050565b600081831215612f925781612f26565b6000818312612f925781612f26565b6000612f26670de0b6b3a764000083613098866132c9565b6130a29190615b0d565b6130ac9190615a31565b61350d565b80516060907fff00000000000000000000000000000000000000000000000000000000000000167f010000000000000000000000000000000000000000000000000000000000000014613130576040517fc06b523800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81604001515160000361316f576040517f9103e7cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008260000151836020015160c01b6040516020016131de9291907fff000000000000000000000000000000000000000000000000000000000000009290921682527fffffffffffffffff00000000000000000000000000000000000000000000000016600182015260090190565b604051602081830303815290604052905060005b8360400151518110156132605760008460400151828151811061321757613217615cf8565b60209081029190910181015180518183015160405192945061323b93879301615e31565b604051602081830303815290604052925050808061325890615df9565b9150506131f2565b5092915050565b6060818051906020012060405160200161328391815260200190565b6040516020818303038152906040529050919050565b60006132c0846132aa87868661374c565b8051602091820120825192909101919091201490565b95945050505050565b6000808213613334576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611ac6565b60006060613341846141ca565b03609f8181039490941b90931c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b393909302929092017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdb731c958f34d94c1821361353e57506000919050565b680755bf798b4a1bf1e582126135b0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f4558505f4f564552464c4f5700000000000000000000000000000000000000006044820152606401611ac6565b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b606060008451116137b9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601560248201527f4d65726b6c65547269653a20656d707479206b657900000000000000000000006044820152606401611ac6565b60006137c4846142a0565b905060006137d18661438c565b90506000846040516020016137e891815260200190565b60405160208183030381529060405290506000805b845181101561414157600085828151811061381a5761381a615cf8565b6020026020010151905084518311156138b5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f4d65726b6c65547269653a206b657920696e646578206578636565647320746f60448201527f74616c206b6579206c656e6774680000000000000000000000000000000000006064820152608401611ac6565b8260000361396e5780518051602091820120604051613903926138dd92910190815260200190565b604051602081830303815290604052858051602091820120825192909101919091201490565b613969576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f4d65726b6c65547269653a20696e76616c696420726f6f7420686173680000006044820152606401611ac6565b613ac5565b805151602011613a245780518051602091820120604051613998926138dd92910190815260200190565b613969576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602760248201527f4d65726b6c65547269653a20696e76616c6964206c6172676520696e7465726e60448201527f616c2068617368000000000000000000000000000000000000000000000000006064820152608401611ac6565b805184516020808701919091208251919092012014613ac5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4d65726b6c65547269653a20696e76616c696420696e7465726e616c206e6f6460448201527f65206861736800000000000000000000000000000000000000000000000000006064820152608401611ac6565b613ad160106001615e58565b81602001515103613cad5784518303613c4557613b0b8160200151601081518110613afe57613afe615cf8565b60200260200101516143ef565b96506000875111613b9e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603b60248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286272616e63682900000000006064820152608401611ac6565b60018651613bac919061586c565b8214613c3a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286272616e6368290000000000006064820152608401611ac6565b505050505050612f26565b6000858481518110613c5957613c59615cf8565b602001015160f81c60f81b60f81c9050600082602001518260ff1681518110613c8457613c84615cf8565b60200260200101519050613c97816144a3565b9550613ca4600186615e58565b9450505061412e565b6002816020015151036140a6576000613cc5826144c8565b9050600081600081518110613cdc57613cdc615cf8565b016020015160f81c90506000613cf3600283615e70565b613cfe906002615e92565b90506000613d0f848360ff166144ec565b90506000613d1d8a896144ec565b90506000613d2b8383614522565b905080835114613dbd576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f4d65726b6c65547269653a20706174682072656d61696e646572206d7573742060448201527f736861726520616c6c206e6962626c65732077697468206b65790000000000006064820152608401611ac6565b60ff851660021480613dd2575060ff85166003145b15613fc15780825114613e67576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603d60248201527f4d65726b6c65547269653a206b65792072656d61696e646572206d757374206260448201527f65206964656e746963616c20746f20706174682072656d61696e6465720000006064820152608401611ac6565b613e818760200151600181518110613afe57613afe615cf8565b9c5060008d5111613f14576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603960248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286c65616629000000000000006064820152608401611ac6565b60018c51613f22919061586c565b8814613fb0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286c6561662900000000000000006064820152608401611ac6565b505050505050505050505050612f26565b60ff85161580613fd4575060ff85166001145b15614013576140008760200151600181518110613ff357613ff3615cf8565b60200260200101516144a3565b995061400c818a615e58565b985061409b565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603260248201527f4d65726b6c65547269653a2072656365697665642061206e6f6465207769746860448201527f20616e20756e6b6e6f776e2070726566697800000000000000000000000000006064820152608401611ac6565b50505050505061412e565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602860248201527f4d65726b6c65547269653a20726563656976656420616e20756e70617273656160448201527f626c65206e6f64650000000000000000000000000000000000000000000000006064820152608401611ac6565b508061413981615df9565b9150506137fd565b506040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f4d65726b6c65547269653a2072616e206f7574206f662070726f6f6620656c6560448201527f6d656e74730000000000000000000000000000000000000000000000000000006064820152608401611ac6565b6000808211614235576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611ac6565b5060016fffffffffffffffffffffffffffffffff821160071b82811c67ffffffffffffffff1060061b1782811c63ffffffff1060051b1782811c61ffff1060041b1782811c60ff10600390811b90911783811c600f1060021b1783811c909110821b1791821c111790565b80516060908067ffffffffffffffff8111156142be576142be614f80565b60405190808252806020026020018201604052801561430357816020015b60408051808201909152606080825260208201528152602001906001900390816142dc5790505b50915060005b8181101561438557604051806040016040528085838151811061432e5761432e615cf8565b6020026020010151815260200161435d86848151811061435057614350615cf8565b60200260200101516145cf565b81525083828151811061437257614372615cf8565b6020908102919091010152600101614309565b5050919050565b606080604051905082518060011b603f8101601f1916830160405280835250602084016020830160005b838110156143e4578060011b82018184015160001a8060041c8253600f8116600183015350506001016143b6565b509295945050505050565b606060008060006143ff856145e2565b91945092509050600081600181111561441a5761441a615ca8565b14614451576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61445b8284615e58565b855114614494576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6132c085602001518484614a80565b606060208260000151106144bf576144ba826143ef565b611377565b61137782614b14565b60606113776144e78360200151600081518110613afe57613afe615cf8565b61438c565b60608251821061450b5750604080516020810190915260008152611377565b612f26838384865161451d919061586c565b614b2a565b6000808251845110614535578251614538565b83515b90505b80821080156145bf575082828151811061455757614557615cf8565b602001015160f81c60f81b7effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191684838151811061459657614596615cf8565b01602001517fff0000000000000000000000000000000000000000000000000000000000000016145b156132605781600101915061453b565b60606113776145dd83614d02565b614d6f565b60008060008360000151600003614625576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f811161464a576000600160009450945094505050614a79565b60b7811161476057600061465f60808361586c565b90508087600001511161469e576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff0000000000000000000000000000000000000000000000000000000000000016908214801561471657507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b1561474d576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5060019550935060009250614a79915050565b60bf81116148be57600061477560b78361586c565b9050808760000151116147b4576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614816576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c6037811161485e576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6148688184615e58565b8951116148a1576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6148ac836001615e58565b9750955060009450614a799350505050565b60f781116149235760006148d360c08361586c565b905080876000015111614912576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600195509350849250614a79915050565b600061493060f78361586c565b90508087600001511161496f576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff000000000000000000000000000000000000000000000000000000000000001660008190036149d1576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614a19576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614a238184615e58565b895111614a5c576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614a67836001615e58565b9750955060019450614a799350505050565b9193909250565b60608167ffffffffffffffff811115614a9b57614a9b614f80565b6040519080825280601f01601f191660200182016040528015614ac5576020820181803683370190505b5090508115612f26576000614ada8486615e58565b90506020820160005b84811015614afb578281015182820152602001614ae3565b84811115614b0a576000858301525b5050509392505050565b6060611377826020015160008460000151614a80565b60608182601f011015614b99576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611ac6565b828284011015614c05576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611ac6565b81830184511015614c72576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f736c6963655f6f75744f66426f756e64730000000000000000000000000000006044820152606401611ac6565b606082158015614c915760405191506000825260208201604052614cf9565b6040519150601f8416801560200281840101858101878315602002848b0101015b81831015614cca578051835260209283019201614cb2565b5050858452601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016604052505b50949350505050565b60408051808201909152600080825260208201528151600003614d51576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b60606000806000614d7f856145e2565b919450925090506001816001811115614d9a57614d9a615ca8565b14614dd1576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8451614ddd8385615e58565b14614e14576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b6040805180820190915260008082526020820152815260200190600190039081614e2b5790505093506000835b8651811015614f1957600080614e9e6040518060400160405280858c60000151614e82919061586c565b8152602001858c60200151614e979190615e58565b90526145e2565b509150915060405180604001604052808383614eba9190615e58565b8152602001848b60200151614ecf9190615e58565b815250888581518110614ee457614ee4615cf8565b6020908102919091010152614efa600185615e58565b9350614f068183615e58565b614f109084615e58565b92505050614e58565b50845250919392505050565b73ffffffffffffffffffffffffffffffffffffffff811681146118e357600080fd5b60008060408385031215614f5a57600080fd5b8235614f6581614f25565b91506020830135614f7581614f25565b809150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040516060810167ffffffffffffffff81118282101715614fd257614fd2614f80565b60405290565b6040805190810167ffffffffffffffff81118282101715614fd257614fd2614f80565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff8111828210171561504257615042614f80565b604052919050565b600082601f83011261505b57600080fd5b813567ffffffffffffffff81111561507557615075614f80565b6150a660207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601614ffb565b8181528460208386010111156150bb57600080fd5b816020850160208301376000918101602001919091529392505050565b600060c082840312156150ea57600080fd5b60405160c0810167ffffffffffffffff828210818311171561510e5761510e614f80565b81604052829350843583526020850135915061512982614f25565b8160208401526040850135915061513f82614f25565b816040840152606085013560608401526080850135608084015260a085013591508082111561516d57600080fd5b5061517a8582860161504a565b60a0830152505092915050565b6000806040838503121561519a57600080fd5b823567ffffffffffffffff8111156151b157600080fd5b6151bd858286016150d8565b9250506020830135614f7581614f25565b6000602082840312156151e057600080fd5b8135612f2681614f25565b6000608082840312156151fd57600080fd5b50919050565b60008083601f84011261521557600080fd5b50813567ffffffffffffffff81111561522d57600080fd5b6020830191508360208260051b850101111561524857600080fd5b9250929050565b600080600080600060e0868803121561526757600080fd5b853567ffffffffffffffff8082111561527f57600080fd5b61528b89838a016150d8565b9650602088013595506152a18960408a016151eb565b945060c08801359150808211156152b757600080fd5b506152c488828901615203565b969995985093965092949392505050565b6000602082840312156152e757600080fd5b5035919050565b60005b838110156153095781810151838201526020016152f1565b8381111561126c5750506000910152565b600081518084526153328160208601602086016152ee565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b602081526000612f26602083018461531a565b6000806040838503121561538a57600080fd5b823591506020830135614f7581614f25565b6000602082840312156153ae57600080fd5b813567ffffffffffffffff8111156153c557600080fd5b6153d1848285016150d8565b949350505050565b6000806000806000806000610120888a0312156153f557600080fd5b873567ffffffffffffffff8082111561540d57600080fd5b6154198b838c016150d8565b985060208a0135915061542b82614f25565b909650604089013595506060890135908082111561544857600080fd5b908901906060828c03121561545c57600080fd5b81955061546c8b60808c016151eb565b94506101008a013591508082111561548357600080fd5b506154908a828b01615203565b989b979a50959850939692959293505050565b67ffffffffffffffff811681146118e357600080fd5b6000602082840312156154cb57600080fd5b8135612f26816154a3565b600080604083850312156154e957600080fd5b50508035926020909101359150565b60008060006060848603121561550d57600080fd5b833561551881614f25565b9250602084013561552881614f25565b9150604084013561553881614f25565b809150509250925092565b80151581146118e357600080fd5b600080600080600060a0868803121561556957600080fd5b853561557481614f25565b945060208601359350604086013561558b816154a3565b9250606086013561559b81615543565b9150608086013567ffffffffffffffff8111156155b757600080fd5b6155c38882890161504a565b9150509295509295909350565b8581528460208201527fffffffffffffffff0000000000000000000000000000000000000000000000008460c01b16604082015282151560f81b6048820152600082516156248160498501602087016152ee565b919091016049019695505050505050565b60006020828403121561564757600080fd5b8151612f2681614f25565b63ffffffff811681146118e357600080fd5b60006020828403121561567657600080fd5b8151612f2681615652565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156156e8576156e8615681565b500290565b6000602082840312156156ff57600080fd5b8151612f2681615543565b60008060006060848603121561571f57600080fd5b835161572a81615652565b602085015190935061573b816154a3565b604085015190925061553881614f25565b60006080828403121561575e57600080fd5b6040516080810181811067ffffffffffffffff8211171561578157615781614f80565b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b600067ffffffffffffffff8211156157cc576157cc614f80565b5060051b60200190565b60006157e96157e4846157b2565b614ffb565b80848252602080830192508560051b85013681111561580757600080fd5b855b8181101561584357803567ffffffffffffffff8111156158295760008081fd5b61583536828a0161504a565b865250938201938201615809565b50919695505050505050565b60006020828403121561586157600080fd5b8151612f26816154a3565b60008282101561587e5761587e615681565b500390565b60006060823603121561589557600080fd5b61589d614faf565b82357fff00000000000000000000000000000000000000000000000000000000000000811681146158cd57600080fd5b81526020838101356158de816154a3565b8282015260408481013567ffffffffffffffff8111156158fd57600080fd5b850136601f82011261590e57600080fd5b803561591c6157e4826157b2565b81815260069190911b8201840190848101903683111561593b57600080fd5b928501925b8284101561597b578484360312156159585760008081fd5b615960614fd8565b84358152868501358782015282529284019290850190615940565b93860193909352509295945050505050565b60006020828403121561599f57600080fd5b5051919050565b600067ffffffffffffffff808316818516818304811182151516156159cd576159cd615681565b02949350505050565b600067ffffffffffffffff8083168185168083038211156159f9576159f9615681565b01949350505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b600082615a4057615a40615a02565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83147f800000000000000000000000000000000000000000000000000000000000000083141615615a9457615a94615681565b500590565b6000808312837f800000000000000000000000000000000000000000000000000000000000000001831281151615615ad357615ad3615681565b837f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff018313811615615b0757615b07615681565b50500390565b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615615b4e57615b4e615681565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615615b8957615b89615681565b60008712925087820587128484161615615ba557615ba5615681565b87850587128184161615615bbb57615bbb615681565b505050929093029392505050565b6000808212827f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03841381151615615c0357615c03615681565b827f8000000000000000000000000000000000000000000000000000000000000000038412811615615c3757615c37615681565b50500190565b600082615c4c57615c4c615a02565b500490565b868152600073ffffffffffffffffffffffffffffffffffffffff808816602084015280871660408401525084606083015283608083015260c060a0830152615c9c60c083018461531a565b98975050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b600060208284031215615ce957600080fd5b815160038110612f2657600080fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b805160ff81168114615d3857600080fd5b919050565b600060c08284031215615d4f57600080fd5b60405160c0810181811067ffffffffffffffff82111715615d7257615d72614f80565b6040528251615d8081615652565b8152615d8e60208401615d27565b6020820152615d9f60408401615d27565b60408201526060830151615db281615652565b60608201526080830151615dc581615652565b608082015260a08301516fffffffffffffffffffffffffffffffff81168114615ded57600080fd5b60a08201529392505050565b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203615e2a57615e2a615681565b5060010190565b60008451615e438184602089016152ee565b91909101928352506020820152604001919050565b60008219821115615e6b57615e6b615681565b500190565b600060ff831680615e8357615e83615a02565b8060ff84160691505092915050565b600060ff821660ff841680821015615eac57615eac615681565b9003939250505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0`(8\x03\x80b\0`(\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x11V[`\x04`\x80R`\xA0\x81\x90Rb\0\0Hb\0\0OV[Pb\0\x01+V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x0FW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01$W`\0\x80\xFD[PQ\x91\x90PV[`\x80Q`\xA0Qa^\xC2b\0\x01f`\09`\0\x81\x81a\x07\x13\x01Ra\x17\x9D\x01R`\0\x81\x81a\x03\x04\x01R\x81\x81a\x1A\0\x01Ra\x1D\x02\x01Ra^\xC2`\0\xF3\xFE`\x80`@R`\x046\x10a\x028W`\x005`\xE0\x1C\x80c\x8C1R\xE9\x11a\x018W\x80c\xBB,r~\x11a\0\xB0W\x80c\xCF\xF0\xAB\x96\x11a\0\x7FW\x80c\xDA\xD5D\xE0\x11a\0dW\x80c\xDA\xD5D\xE0\x14a\x08*W\x80c\xE9\xE0\\B\x14a\x08?W\x80c\xF2\xB4\xE6\x17\x14a\x08RW`\0\x80\xFD[\x80c\xCF\xF0\xAB\x96\x14a\x07WW\x80c\xD3%\xD3\xBF\x14a\x07\xF8W`\0\x80\xFD[\x80c\xBB,r~\x14a\x06HW\x80c\xBD\xA2\x04\xBB\x14a\x06\xEFW\x80c\xBFe:\\\x14a\x07\x04W\x80c\xC0\xC5;\x8B\x14a\x077W`\0\x80\xFD[\x80c\x9B\xF6-\x82\x11a\x01\x07W\x80c\xA3]\x99\xDF\x11a\0\xECW\x80c\xA3]\x99\xDF\x14a\x05\xDBW\x80c\xA3\x86\x0FH\x14a\x05\xFBW\x80c\xB6\x82\xC4D\x14a\x06\x1BW`\0\x80\xFD[\x80c\x9B\xF6-\x82\x14a\x05~W\x80c\xA1B8\xE7\x14a\x05\xABW`\0\x80\xFD[\x80c\x8C1R\xE9\x14a\x05\tW\x80c\x8C\x90\xDDe\x14a\x05)W\x80c\x95+'\x97\x14a\x05IW\x80c\x99\xA8\x8E\xC4\x14a\x05^W`\0\x80\xFD[\x80cE\x88M2\x11a\x01\xCBW\x80cT\xFDMP\x11a\x01\x9AW\x80c\\\x97Z\xBB\x11a\x01\x7FW\x80c\\\x97Z\xBB\x14a\x04\xD4W\x80cq\xC1Vn\x14a\x04\xE9W\x80c\x8BL@\xB0\x14a\x02]W`\0\x80\xFD[\x80cT\xFDMP\x14a\x04[W\x80c\\\x0C\xBA3\x14a\x04\xA7W`\0\x80\xFD[\x80cE\x88M2\x14a\x03\xA2W\x80cHpIo\x14a\x03\xD2W\x80cO\xD0CL\x14a\x03\xF2W\x80cQ7G\xAB\x14a\x04 W`\0\x80\xFD[\x80c<\x9F9|\x11a\x02\x07W\x80c<\x9F9|\x14a\x03.W\x80c>G\x15\x8C\x14a\x03XW\x80cC\xCA\x1CP\x14a\x03mW\x80cE*\x93 \x14a\x03\x8DW`\0\x80\xFD[\x80c!R\xF2\xBE\x14a\x02dW\x80c3\xD7\xE2\xBD\x14a\x02\x84W\x80c5\xE8\n\xB3\x14a\x02\xDBW\x80c8\xD3\x8C\x97\x14a\x02\xF0W`\0\x80\xFD[6a\x02_Wa\x02]34b\x01\x86\xA0`\0`@Q\x80` \x01`@R\x80`\0\x81RPa\x08gV[\0[`\0\x80\xFD[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x02]a\x02\x7F6`\x04aOGV[a\n\xB3V[4\x80\x15a\x02\x90W`\0\x80\xFD[P`7Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xE7W`\0\x80\xFD[Pa\x02\xB1a\x0B\xF9V[4\x80\x15a\x02\xFCW`\0\x80\xFD[P`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x02\xD2V[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x03Ca\x0C\x92V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xD2V[4\x80\x15a\x03dW`\0\x80\xFD[Pa\x02\xB1a\r&V[4\x80\x15a\x03yW`\0\x80\xFD[Pa\x02]a\x03\x886`\x04aQ\x87V[a\x0F1V[4\x80\x15a\x03\x99W`\0\x80\xFD[Pa\x02\xB1a\x12rV[4\x80\x15a\x03\xAEW`\0\x80\xFD[Pa\x03\xC2a\x03\xBD6`\x04aQ\xCEV[a\x12\xE2V[`@Q\x90\x15\x15\x81R` \x01a\x02\xD2V[4\x80\x15a\x03\xDEW`\0\x80\xFD[Pa\x02]a\x03\xED6`\x04aROV[a\x13}V[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x04\x07a\x14\xADV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xD2V[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x04Ma\x04;6`\x04aR\xD5V[`\0\x90\x81R`<` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\xD2V[4\x80\x15a\x04gW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\r\x81R\x7F5.1.0+interop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x02\xD2\x91\x90aSdV[4\x80\x15a\x04\xB3W`\0\x80\xFD[P`>Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x03\xC2a\x15AV[4\x80\x15a\x04\xF5W`\0\x80\xFD[Pa\x02]a\x05\x046`\x04aSwV[a\x15\xD5V[4\x80\x15a\x05\x15W`\0\x80\xFD[Pa\x02]a\x05$6`\x04aS\x9CV[a\x18\xD9V[4\x80\x15a\x055W`\0\x80\xFD[Pa\x02]a\x05D6`\x04aS\xD9V[a\x18\xE6V[4\x80\x15a\x05UW`\0\x80\xFD[Pa\x04Ma\x19jV[4\x80\x15a\x05jW`\0\x80\xFD[Pa\x02]a\x05y6`\x04aOGV[a\x19\xFEV[4\x80\x15a\x05\x8AW`\0\x80\xFD[P`2Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x03\xC2a\x05\xC66`\x04aR\xD5V[`3` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xE7W`\0\x80\xFD[Pa\x04\x07a\x05\xF66`\x04aT\xB9V[a\x1B\xC0V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x02\xB1a\x06\x166`\x04aT\xD6V[a\x1B\xD9V[4\x80\x15a\x06'W`\0\x80\xFD[P`?Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x06TW`\0\x80\xFD[Pa\x06\xBAa\x06c6`\x04aSwV[`9` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xD2V[4\x80\x15a\x06\xFBW`\0\x80\xFD[Pa\x02]a\x1C\x1EV[4\x80\x15a\x07\x10W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04MV[4\x80\x15a\x07CW`\0\x80\xFD[Pa\x02]a\x07R6`\x04aT\xF8V[a\x1D\0V[4\x80\x15a\x07cW`\0\x80\xFD[P`\x01Ta\x07\xBF\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x02\xD2V[4\x80\x15a\x08\x04W`\0\x80\xFD[P`?Ta\x03\xC2\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x086W`\0\x80\xFD[Pa\x02\xB1a\x1E\xFDV[a\x02]a\x08M6`\x04aUQV[a\x08gV[4\x80\x15a\x08^W`\0\x80\xFD[Pa\x02\xB1a\x1FQV[\x82`\0Z\x90P4\x15a\x08\xF7W`?`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1E\xE1\x16\xBF4`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x08\xDDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xF1W=`\0\x80>=`\0\xFD[PPPPP[\x83\x80\x15a\t\x19WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x15\x15[\x15a\tPW`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tZ\x83Qa\x1B\xC0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\t\xA7W`@Q\x7Fp\xC8\xBD\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x01\xD4\xC0\x83Q\x11\x15a\t\xE5W`@Q\x7FZ\xA3\xBA\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\t\xEEa\x1F\xC1V[a\n\x0BWP3s\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\x01[`\x004\x88\x88\x88\x88`@Q` \x01a\n&\x95\x94\x93\x92\x91\x90aU\xD0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB3\x815h\xD9\x99\x1F\xC9Q\x96\x1F\xCBLxH\x93WB@\xA2\x89%`M\t\xFCW|U\xBB|2\x84`@Qa\n\x96\x91\x90aSdV[`@Q\x80\x91\x03\x90\xA4PPa\n\xAA\x82\x82a\x1F\xFFV[PPPPPPPV[a\n\xBBa\"\xD6V[a\n\xC3a#\x15V[`>Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x03a\x0B\x17W`@Q\x7Fx]\xF9\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`?\x80T`>\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x87\x82\x16\x90\x81\x17t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90\x95U`@\x80Q\x94\x82\x16\x80\x86R` \x86\x01\x96\x90\x96R\x91\x16\x90\x83\x01\x81\x90R``\x83\x01\x91\x90\x91R\x90\x7F\x9EShG\x1AX\xD8\x19\x87\xE5\xDC}ct\xDD^\xD5\xE7V\xCC\x95\xA7\x9F\xF7&\x904#\xBC\xE0\x06\r\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA1PPPPV[`7T`@\x80Q\x7F5\xE8\n\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c5\xE8\n\xB3\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aV5V[\x90P\x90V[`>T`@\x80Q\x7F<\x9F9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c<\x9F9|\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aVdV[`\0\x80a\rQ\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03T\x90V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\rtW\x91\x90PV[`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x81RPQ`\x02a\r\xB7\x91\x90aV\xB0V[`@\x80Q0` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x91\x90\x91\x17\x90a\x0E\x12\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 T\x90V[\x14a\x0EIW`@Q\x7FT\xE43\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R`\0\x90a\x0Ek\x90``\x01a\r\xF8V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\x0E\xFFW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF8\x91\x90aV5V[\x92PPP\x90V[`@Q\x7F3!D\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F9a\"\xD6V[`2Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\xDE\xAD\x14a\x0F\x8CW`@Q\x7F\xDF\xEA\xAE\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x99\x82`@\x01Qa#kV[\x15a\x0F\xD0W`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0F\xDB\x83a#\xAEV[\x90Pa\x0F\xE7\x81\x83a\x15\xD5V[`\0\x81\x81R`3` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U``\x83\x01Q\x15a\x10\xB8W`?T``\x84\x01Q`@Q\x7F\x8DD[\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91c\x8DD[\xD0\x91a\x10\x85\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x9FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xB3W=`\0\x80>=`\0\xFD[PPPP[\x82` \x01Q`2`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0a\x11\x1B\x84`@\x01Q\x85`\x80\x01Q\x86``\x01Q\x87`\xA0\x01Qa#\xFBV[`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U`@Q\x90\x91P\x82\x90\x7F\xDB\\vR\x85z\xA1c\xDA\xAD\xD6p\xE1\x16b\x8F\xB4.\x86\x9D\x8A\xC4%\x1E\xF8\x97\x1D\x9EW'\xDF\x1B\x90a\x11\x80\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x80\x15\x80\x15a\x11\x9AWP`\0\x84``\x01Q\x11[\x15a\x12'W`?`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1E\xE1\x16\xBF\x85``\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12!W=`\0\x80>=`\0\xFD[PPPPP[\x80\x15\x80\x15a\x125WP2`\x01\x14[\x15a\x12lW`@Q\x7F\xABX\x106\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`7T`@\x80Q\x7FE*\x93 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91cE*\x93 \x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[`>T`@Q\x7FE\x88M2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x92\x16\x90cE\x88M2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13w\x91\x90aV\xEDV[\x92\x91PPV[a\x13\x85a\"\xD6V[`?Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x13\xDAW`@Q\x7F^t\xB5B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\xE4a\x1FQV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBB\x8A\xA1\xFC\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x1E\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14_\x91\x90aW\nV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R\x90\x93P\x91Pa\x14\x87\x90PV[a\n\xAA\x87\x83`\0\x84a\x14\x9E6\x8B\x90\x03\x8B\x01\x8BaWLV[a\x14\xA8\x89\x8BaW\xD6V[a$YV[`>T`@\x80Q\x7F@\x86\xD1\x83\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c@\x86\xD1\x83\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x15\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aXOV[`7T`@\x80Q\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\\\x97Z\xBB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x15\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aV\xEDV[`\0\x82\x81R`9` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x85R\x90\x83R\x81\x84 \x82Q\x80\x84\x01\x84R\x90T\x91\x82\x16\x80\x82Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x85\x01R\x86\x85R`3\x90\x93R\x92 T\x90\x91\x90`\xFF\x16\x15a\x16\x84W`@Q\x7Fs\n\x10t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x16\xCCW`@Q\x7F\xCC\xA6\xAF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17K\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17>\x91\x90aXOV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x17\x9BW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x17\xD6\x91\x90aXlV[\x11a\x18\rW`@Q\x7F\xD9\xBC\x01\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7FlODg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90clODg\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA1\x91\x90aV\xEDV[a\x12lW`@Q\x7F3*W\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x18\xE3\x813a\x0F1V[PV[a\x18\xEEa\"\xD6V[`?Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x19BW`@Q\x7F^t\xB5B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xAA\x87\x87\x87a\x19Q\x88aX\x83V[a\x19`6\x89\x90\x03\x89\x01\x89aWLV[a\x14\xA8\x87\x89aW\xD6V[`>T`@\x80Q\x7F\x95+'\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x95+'\x97\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x19\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aY\x8DV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x1A>WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x1A\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x1B\x08a,\xB4V[`>\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`?\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x1B\xCD\x82`(aY\xA6V[a\x13w\x90aR\x08aY\xD6V[`<` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x1B\xF5W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x82\x90PV[a\x1C&a#\x15V[`?T`@\x80Q\x7F\x1E\xE1\x16\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90QG\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x1E\xE1\x16\xBF\x91\x84\x91`\x04\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x85\x88\x80;\x15\x80\x15a\x1C\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xA8W=`\0\x80>=`\0\xFD[PP`?T`@Q\x85\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x93P\x7F\xD8\x93\xF60\xC6\x86\x7F\xA46\x89\xDA\x9A\xE9I\xEB\xF0L\xAC$\xAA\xD3\xB4\\u\x9DD.\xD3\xC3.:7\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA2PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x1D@WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x1D\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x1E\x05a,\xB4V[`7\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`>\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`?\x80T\x85\x84\x16\x92\x16\x91\x90\x91\x17\x90U`2T\x16a\x1E\x9AW`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U[a\x1E\xA2a-5V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x0B\xEBV[`\0a\x1F\x07a\r&V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[`>T`@\x80Q\x7F\xF2\xB4\xE6\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xF2\xB4\xE6\x17\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[`\x0023\x03a\x1F\xD0WP`\x01\x90V[3;`\x17\x03a\x1F\xF8W`@Q` \x81\x01`@R` `\0\x823<Q`\xE8\x1Cb\xEF\x01\0\x14\x90P\x90V[P`\0[\x90V[`\x01T`\0\x90a 5\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16CaXlV[\x90P`\0a Aa.HV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a b\x91\x90aZ1V[\x90P\x82\x15a!\x99W`\x01T`\0\x90a \x99\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZ\x99V[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a \xB0\x91\x90a[\rV[`\x01Ta \xD0\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\rV[a \xDA\x91\x90aZ1V[`\x01T\x90\x91P`\0\x90a!+\x90a!\x04\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\xC9V[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a/\x0EV[\x90P`\x01\x86\x11\x15a!ZWa!Wa!\x04\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa!R\x91\x90aXlV[a/-V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a!\xCC\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aY\xD6V[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\"YW`@Q\x7Fw\xEB\xEFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\0\x90a\"\x85\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16aV\xB0V[\x90P`\0a\"\x97Hc;\x9A\xCA\0a/\x82V[a\"\xA1\x90\x83a\\=V[\x90P`\0Za\"\xB0\x90\x88aXlV[\x90P\x80\x82\x11\x15a\"\xCCWa\"\xCCa\"\xC7\x82\x84aXlV[a/\x99V[PPPPPPPPV[a\"\xDEa\x15AV[\x15a\x18\xD7W`@Q\x7F\xB9\xC3\xC2\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a#\x1Ea\x1E\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xD7W`@Q\x7F\x7F\x12\xC6K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14\x80a\x13wWPP`?Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a#\xDE\x97\x90\x96\x95\x91\x01a\\QV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a$\x0B\x86`\0a/\xC7V[\x90P\x80a$AWc\x08\xC3y\xA0`\0R` \x80Rx\x18SafeCall: Not enough gas`XR`d`\x1C\xFD[`\0\x80\x85Q` \x87\x01\x88\x8BZ\xF1\x97\x96PPPPPPPV[a$f\x86`@\x01Qa#kV[\x15a$\x9DW`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%1\x91\x90aV\xEDV[a%gW`@Q\x7F\xF3\x95$\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7F\x04\xE5\x0F\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\x04\xE5\x0F\xED\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xFB\x91\x90aV\xEDV[a&1W`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c \r.\xD2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xA2\x91\x90a\\\xD7V[`\x02\x81\x11\x15a&\xB3Wa&\xB3a\\\xA8V[\x03a&\xEAW`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'8\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x1AW=`\0\x80>=`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11a'zW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`?Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a)\xC4Wa'\xA6\x83a/\xE5V[a(\x18\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xFC\x91\x90aY\x8DV[\x14a(OW`@Q\x7F+\x1A\x9Af\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`@\x01QQ\x84\x10a(\x8DW`@Q\x7F2\xDC(\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83`@\x01Q\x85\x81Q\x81\x10a(\xA5Wa(\xA5a\\\xF8V[` \x02` \x01\x01Q\x90P`7`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\xAE<\xD5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)@\x91\x90aY\x8DV[\x81Q\x14a)yW`@Q\x7F|\xC2\xF3\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\x82\x83a/\xFEV[\x81` \x01Q\x14a)\xBEW`@Q\x7FBaI\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa*RV[a)\xCD\x82a/\xFEV[a*\x1B\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xF4W=`\0\x80>=`\0\xFD[\x14a*RW`@Q\x7FBaI\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a*]\x87a#\xAEV[\x90P`\0\x81`\0`@Q` \x01a*~\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92Pa+)\x91\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x82\x82\x01\x82R`\x01\x83R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x90\x87\x01Q\x90\x91\x90\x86\x90a0=V[\x15\x15`\0\x03a+dW`@Q\x7F.W\xEF:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16` \x80\x85\x01\x91\x82R`\0\x88\x81R`9\x82R\x86\x81 3\x80\x83R\x90\x83R\x87\x82 \x96Q\x87T\x94Q\x90\x95\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x94\x86\x16\x94\x90\x94\x17\x92\x90\x92\x17\x90\x94U\x86\x81R`<\x84R\x84\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x84\x82 \x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x92\x17\x90\x91U\x8B\x84\x01Q\x92\x8C\x01Q\x93Q\x92\x82\x16\x93\x90\x91\x16\x91\x85\x91\x7Fg\xA6 \x8C\xFC\xC0\x80\x1DP\xF6\xCB\xE7ds?O\xDD\xF6j\xC0\xB0DB\x06\x1A\x8A\x8C\x0C\xB6\xB6?b\x91\xA4`@Q3\x90\x83\x90\x7Fy\x8F\x9F\x13i_\x8F\x04Z\xA5\xF8\x0E\xD8\xEF\xEB\xB6\x95\xF3\xC7\xFEe\xDA8\x19i\xF2\xF2\x8B\xF3\xC6\x0B\x97\x90`\0\x90\xA3PPPPPPPPV[3a,\xBDa\r&V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a,\xFEWP3a,\xE5a\x1E\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x18\xD7W`@Q\x7F\xC4\x05\n&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16a-\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\x01Tx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x18\xD7W`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90\x92\x01\x81\x90Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x17`\x01UV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R`7T`@\x80Q\x7F\xCCs\x1B\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xCCs\x1B\x02\x91`\x04\x80\x83\x01\x92`\xC0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a.\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13w\x91\x90a]=V[`\0a/#a/\x1D\x85\x85a0aV[\x83a0qV[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a/na/E\x85\x83aZ1V[a/W\x90g\r\xE0\xB6\xB3\xA7d\0\0aZ\x99V[a/i\x85g\r\xE0\xB6\xB3\xA7d\0\0a[\rV[a0\x80V[a/x\x90\x86a[\rV[a/#\x91\x90aZ1V[`\0\x81\x83\x10\x15a/\x92W\x81a/&V[P\x90\x91\x90PV[`\0\x80Z\x90P[\x82Za/\xAC\x90\x83aXlV[\x10\x15a/\xC2Wa/\xBB\x82a]\xF9V[\x91Pa/\xA0V[PPPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[`\0a/\xF0\x82a0\xB1V[\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a#\xDE\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`\0\x80a0I\x86a2gV[\x90Pa0W\x81\x86\x86\x86a2\x99V[\x96\x95PPPPPPV[`\0\x81\x83\x12\x15a/\x92W\x81a/&V[`\0\x81\x83\x12a/\x92W\x81a/&V[`\0a/&g\r\xE0\xB6\xB3\xA7d\0\0\x83a0\x98\x86a2\xC9V[a0\xA2\x91\x90a[\rV[a0\xAC\x91\x90aZ1V[a5\rV[\x80Q``\x90\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a10W`@Q\x7F\xC0kR8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`@\x01QQ`\0\x03a1oW`@Q\x7F\x91\x03\xE7\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`\0\x01Q\x83` \x01Q`\xC0\x1B`@Q` \x01a1\xDE\x92\x91\x90\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x90\x92\x16\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01\x82\x01R`\t\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x83`@\x01QQ\x81\x10\x15a2`W`\0\x84`@\x01Q\x82\x81Q\x81\x10a2\x17Wa2\x17a\\\xF8V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x80Q\x81\x83\x01Q`@Q\x92\x94Pa2;\x93\x87\x93\x01a^1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PP\x80\x80a2X\x90a]\xF9V[\x91PPa1\xF2V[P\x92\x91PPV[``\x81\x80Q\x90` \x01 `@Q` \x01a2\x83\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a2\xC0\x84a2\xAA\x87\x86\x86a7LV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[\x95\x94PPPPPV[`\0\x80\x82\x13a34W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[`\0``a3A\x84aA\xCAV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a5>WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a5\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[```\0\x84Q\x11a7\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FMerkleTrie: empty key\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[`\0a7\xC4\x84aB\xA0V[\x90P`\0a7\xD1\x86aC\x8CV[\x90P`\0\x84`@Q` \x01a7\xE8\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80[\x84Q\x81\x10\x15aAAW`\0\x85\x82\x81Q\x81\x10a8\x1AWa8\x1Aa\\\xF8V[` \x02` \x01\x01Q\x90P\x84Q\x83\x11\x15a8\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FMerkleTrie: key index exceeds to`D\x82\x01R\x7Ftal key length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[\x82`\0\x03a9nW\x80Q\x80Q` \x91\x82\x01 `@Qa9\x03\x92a8\xDD\x92\x91\x01\x90\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[a9iW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FMerkleTrie: invalid root hash\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[a:\xC5V[\x80QQ` \x11a:$W\x80Q\x80Q` \x91\x82\x01 `@Qa9\x98\x92a8\xDD\x92\x91\x01\x90\x81R` \x01\x90V[a9iW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FMerkleTrie: invalid large intern`D\x82\x01R\x7Fal hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[\x80Q\x84Q` \x80\x87\x01\x91\x90\x91 \x82Q\x91\x90\x92\x01 \x14a:\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FMerkleTrie: invalid internal nod`D\x82\x01R\x7Fe hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[a:\xD1`\x10`\x01a^XV[\x81` \x01QQ\x03a<\xADW\x84Q\x83\x03a<EWa;\x0B\x81` \x01Q`\x10\x81Q\x81\x10a:\xFEWa:\xFEa\\\xF8V[` \x02` \x01\x01QaC\xEFV[\x96P`\0\x87Q\x11a;\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (branch)\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\x01\x86Qa;\xAC\x91\x90aXlV[\x82\x14a<:W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (branch)\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[PPPPPPa/&V[`\0\x85\x84\x81Q\x81\x10a<YWa<Ya\\\xF8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\0\x82` \x01Q\x82`\xFF\x16\x81Q\x81\x10a<\x84Wa<\x84a\\\xF8V[` \x02` \x01\x01Q\x90Pa<\x97\x81aD\xA3V[\x95Pa<\xA4`\x01\x86a^XV[\x94PPPaA.V[`\x02\x81` \x01QQ\x03a@\xA6W`\0a<\xC5\x82aD\xC8V[\x90P`\0\x81`\0\x81Q\x81\x10a<\xDCWa<\xDCa\\\xF8V[\x01` \x01Q`\xF8\x1C\x90P`\0a<\xF3`\x02\x83a^pV[a<\xFE\x90`\x02a^\x92V[\x90P`\0a=\x0F\x84\x83`\xFF\x16aD\xECV[\x90P`\0a=\x1D\x8A\x89aD\xECV[\x90P`\0a=+\x83\x83aE\"V[\x90P\x80\x83Q\x14a=\xBDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: path remainder must `D\x82\x01R\x7Fshare all nibbles with key\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\xFF\x85\x16`\x02\x14\x80a=\xD2WP`\xFF\x85\x16`\x03\x14[\x15a?\xC1W\x80\x82Q\x14a>gW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FMerkleTrie: key remainder must b`D\x82\x01R\x7Fe identical to path remainder\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[a>\x81\x87` \x01Q`\x01\x81Q\x81\x10a:\xFEWa:\xFEa\\\xF8V[\x9CP`\0\x8DQ\x11a?\x14W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (leaf)\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\x01\x8CQa?\"\x91\x90aXlV[\x88\x14a?\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (leaf)\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[PPPPPPPPPPPPa/&V[`\xFF\x85\x16\x15\x80a?\xD4WP`\xFF\x85\x16`\x01\x14[\x15a@\x13Wa@\0\x87` \x01Q`\x01\x81Q\x81\x10a?\xF3Wa?\xF3a\\\xF8V[` \x02` \x01\x01QaD\xA3V[\x99Pa@\x0C\x81\x8Aa^XV[\x98Pa@\x9BV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FMerkleTrie: received a node with`D\x82\x01R\x7F an unknown prefix\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[PPPPPPaA.V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMerkleTrie: received an unparsea`D\x82\x01R\x7Fble node\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[P\x80aA9\x81a]\xF9V[\x91PPa7\xFDV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FMerkleTrie: ran out of proof ele`D\x82\x01R\x7Fments\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\0\x80\x82\x11aB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[\x80Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xBEWaB\xBEaO\x80V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aC\x03W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81aB\xDCW\x90P[P\x91P`\0[\x81\x81\x10\x15aC\x85W`@Q\x80`@\x01`@R\x80\x85\x83\x81Q\x81\x10aC.WaC.a\\\xF8V[` \x02` \x01\x01Q\x81R` \x01aC]\x86\x84\x81Q\x81\x10aCPWaCPa\\\xF8V[` \x02` \x01\x01QaE\xCFV[\x81RP\x83\x82\x81Q\x81\x10aCrWaCra\\\xF8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01aC\tV[PP\x91\x90PV[``\x80`@Q\x90P\x82Q\x80`\x01\x1B`?\x81\x01`\x1F\x19\x16\x83\x01`@R\x80\x83RP` \x84\x01` \x83\x01`\0[\x83\x81\x10\x15aC\xE4W\x80`\x01\x1B\x82\x01\x81\x84\x01Q`\0\x1A\x80`\x04\x1C\x82S`\x0F\x81\x16`\x01\x83\x01SPP`\x01\x01aC\xB6V[P\x92\x95\x94PPPPPV[```\0\x80`\0aC\xFF\x85aE\xE2V[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15aD\x1AWaD\x1Aa\\\xA8V[\x14aDQW`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aD[\x82\x84a^XV[\x85Q\x14aD\x94W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2\xC0\x85` \x01Q\x84\x84aJ\x80V[``` \x82`\0\x01Q\x10aD\xBFWaD\xBA\x82aC\xEFV[a\x13wV[a\x13w\x82aK\x14V[``a\x13waD\xE7\x83` \x01Q`\0\x81Q\x81\x10a:\xFEWa:\xFEa\\\xF8V[aC\x8CV[``\x82Q\x82\x10aE\x0BWP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x13wV[a/&\x83\x83\x84\x86QaE\x1D\x91\x90aXlV[aK*V[`\0\x80\x82Q\x84Q\x10aE5W\x82QaE8V[\x83Q[\x90P[\x80\x82\x10\x80\x15aE\xBFWP\x82\x82\x81Q\x81\x10aEWWaEWa\\\xF8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x84\x83\x81Q\x81\x10aE\x96WaE\x96a\\\xF8V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[\x15a2`W\x81`\x01\x01\x91PaE;V[``a\x13waE\xDD\x83aM\x02V[aMoV[`\0\x80`\0\x83`\0\x01Q`\0\x03aF%W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11aFJW`\0`\x01`\0\x94P\x94P\x94PPPaJyV[`\xB7\x81\x11aG`W`\0aF_`\x80\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aF\x9EW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15aG\x16WP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15aGMW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92PaJy\x91PPV[`\xBF\x81\x11aH\xBEW`\0aGu`\xB7\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aG\xB4W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aH\x16W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aH^W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aHh\x81\x84a^XV[\x89Q\x11aH\xA1W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aH\xAC\x83`\x01a^XV[\x97P\x95P`\0\x94PaJy\x93PPPPV[`\xF7\x81\x11aI#W`\0aH\xD3`\xC0\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aI\x12W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92PaJy\x91PPV[`\0aI0`\xF7\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aIoW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aI\xD1W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aJ\x19W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aJ#\x81\x84a^XV[\x89Q\x11aJ\\W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aJg\x83`\x01a^XV[\x97P\x95P`\x01\x94PaJy\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\x9BWaJ\x9BaO\x80V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aJ\xC5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15a/&W`\0aJ\xDA\x84\x86a^XV[\x90P` \x82\x01`\0[\x84\x81\x10\x15aJ\xFBW\x82\x81\x01Q\x82\x82\x01R` \x01aJ\xE3V[\x84\x81\x11\x15aK\nW`\0\x85\x83\x01R[PPP\x93\x92PPPV[``a\x13w\x82` \x01Q`\0\x84`\0\x01QaJ\x80V[``\x81\x82`\x1F\x01\x10\x15aK\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[\x82\x82\x84\x01\x10\x15aL\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[\x81\x83\x01\x84Q\x10\x15aLrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Fslice_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[``\x82\x15\x80\x15aL\x91W`@Q\x91P`\0\x82R` \x82\x01`@RaL\xF9V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15aL\xCAW\x80Q\x83R` \x92\x83\x01\x92\x01aL\xB2V[PP\x85\x84R`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16`@RP[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03aMQW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0aM\x7F\x85aE\xE2V[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15aM\x9AWaM\x9Aa\\\xA8V[\x14aM\xD1W`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84QaM\xDD\x83\x85a^XV[\x14aN\x14W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81aN+W\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15aO\x19W`\0\x80aN\x9E`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01QaN\x82\x91\x90aXlV[\x81R` \x01\x85\x8C` \x01QaN\x97\x91\x90a^XV[\x90RaE\xE2V[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83aN\xBA\x91\x90a^XV[\x81R` \x01\x84\x8B` \x01QaN\xCF\x91\x90a^XV[\x81RP\x88\x85\x81Q\x81\x10aN\xE4WaN\xE4a\\\xF8V[` \x90\x81\x02\x91\x90\x91\x01\x01RaN\xFA`\x01\x85a^XV[\x93PaO\x06\x81\x83a^XV[aO\x10\x90\x84a^XV[\x92PPPaNXV[P\x84RP\x91\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xE3W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aOZW`\0\x80\xFD[\x825aOe\x81aO%V[\x91P` \x83\x015aOu\x81aO%V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aO\xD2WaO\xD2aO\x80V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aO\xD2WaO\xD2aO\x80V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aPBWaPBaO\x80V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aP[W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aPuWaPuaO\x80V[aP\xA6` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01aO\xFBV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aP\xBBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15aP\xEAW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aQ\x0EWaQ\x0EaO\x80V[\x81`@R\x82\x93P\x845\x83R` \x85\x015\x91PaQ)\x82aO%V[\x81` \x84\x01R`@\x85\x015\x91PaQ?\x82aO%V[\x81`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15aQmW`\0\x80\xFD[PaQz\x85\x82\x86\x01aPJV[`\xA0\x83\x01RPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x9AW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xB1W`\0\x80\xFD[aQ\xBD\x85\x82\x86\x01aP\xD8V[\x92PP` \x83\x015aOu\x81aO%V[`\0` \x82\x84\x03\x12\x15aQ\xE0W`\0\x80\xFD[\x815a/&\x81aO%V[`\0`\x80\x82\x84\x03\x12\x15aQ\xFDW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aR\x15W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR-W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aRHW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\xE0\x86\x88\x03\x12\x15aRgW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aR\x7FW`\0\x80\xFD[aR\x8B\x89\x83\x8A\x01aP\xD8V[\x96P` \x88\x015\x95PaR\xA1\x89`@\x8A\x01aQ\xEBV[\x94P`\xC0\x88\x015\x91P\x80\x82\x11\x15aR\xB7W`\0\x80\xFD[PaR\xC4\x88\x82\x89\x01aR\x03V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aR\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15aS\tW\x81\x81\x01Q\x83\x82\x01R` \x01aR\xF1V[\x83\x81\x11\x15a\x12lWPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaS2\x81` \x86\x01` \x86\x01aR\xEEV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a/&` \x83\x01\x84aS\x1AV[`\0\x80`@\x83\x85\x03\x12\x15aS\x8AW`\0\x80\xFD[\x825\x91P` \x83\x015aOu\x81aO%V[`\0` \x82\x84\x03\x12\x15aS\xAEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xC5W`\0\x80\xFD[aS\xD1\x84\x82\x85\x01aP\xD8V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0a\x01 \x88\x8A\x03\x12\x15aS\xF5W`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aT\rW`\0\x80\xFD[aT\x19\x8B\x83\x8C\x01aP\xD8V[\x98P` \x8A\x015\x91PaT+\x82aO%V[\x90\x96P`@\x89\x015\x95P``\x89\x015\x90\x80\x82\x11\x15aTHW`\0\x80\xFD[\x90\x89\x01\x90``\x82\x8C\x03\x12\x15aT\\W`\0\x80\xFD[\x81\x95PaTl\x8B`\x80\x8C\x01aQ\xEBV[\x94Pa\x01\0\x8A\x015\x91P\x80\x82\x11\x15aT\x83W`\0\x80\xFD[PaT\x90\x8A\x82\x8B\x01aR\x03V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xE3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aT\xCBW`\0\x80\xFD[\x815a/&\x81aT\xA3V[`\0\x80`@\x83\x85\x03\x12\x15aT\xE9W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15aU\rW`\0\x80\xFD[\x835aU\x18\x81aO%V[\x92P` \x84\x015aU(\x81aO%V[\x91P`@\x84\x015aU8\x81aO%V[\x80\x91PP\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x18\xE3W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aUiW`\0\x80\xFD[\x855aUt\x81aO%V[\x94P` \x86\x015\x93P`@\x86\x015aU\x8B\x81aT\xA3V[\x92P``\x86\x015aU\x9B\x81aUCV[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xB7W`\0\x80\xFD[aU\xC3\x88\x82\x89\x01aPJV[\x91PP\x92\x95P\x92\x95\x90\x93PV[\x85\x81R\x84` \x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xC0\x1B\x16`@\x82\x01R\x82\x15\x15`\xF8\x1B`H\x82\x01R`\0\x82QaV$\x81`I\x85\x01` \x87\x01aR\xEEV[\x91\x90\x91\x01`I\x01\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aVGW`\0\x80\xFD[\x81Qa/&\x81aO%V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xE3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aVvW`\0\x80\xFD[\x81Qa/&\x81aVRV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aV\xE8WaV\xE8aV\x81V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aV\xFFW`\0\x80\xFD[\x81Qa/&\x81aUCV[`\0\x80`\0``\x84\x86\x03\x12\x15aW\x1FW`\0\x80\xFD[\x83QaW*\x81aVRV[` \x85\x01Q\x90\x93PaW;\x81aT\xA3V[`@\x85\x01Q\x90\x92PaU8\x81aO%V[`\0`\x80\x82\x84\x03\x12\x15aW^W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aW\x81WaW\x81aO\x80V[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aW\xCCWaW\xCCaO\x80V[P`\x05\x1B` \x01\x90V[`\0aW\xE9aW\xE4\x84aW\xB2V[aO\xFBV[\x80\x84\x82R` \x80\x83\x01\x92P\x85`\x05\x1B\x85\x016\x81\x11\x15aX\x07W`\0\x80\xFD[\x85[\x81\x81\x10\x15aXCW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX)W`\0\x80\x81\xFD[aX56\x82\x8A\x01aPJV[\x86RP\x93\x82\x01\x93\x82\x01aX\tV[P\x91\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aXaW`\0\x80\xFD[\x81Qa/&\x81aT\xA3V[`\0\x82\x82\x10\x15aX~WaX~aV\x81V[P\x03\x90V[`\0``\x826\x03\x12\x15aX\x95W`\0\x80\xFD[aX\x9DaO\xAFV[\x825\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14aX\xCDW`\0\x80\xFD[\x81R` \x83\x81\x015aX\xDE\x81aT\xA3V[\x82\x82\x01R`@\x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\xFDW`\0\x80\xFD[\x85\x016`\x1F\x82\x01\x12aY\x0EW`\0\x80\xFD[\x805aY\x1CaW\xE4\x82aW\xB2V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x84\x01\x90\x84\x81\x01\x906\x83\x11\x15aY;W`\0\x80\xFD[\x92\x85\x01\x92[\x82\x84\x10\x15aY{W\x84\x846\x03\x12\x15aYXW`\0\x80\x81\xFD[aY`aO\xD8V[\x845\x81R\x86\x85\x015\x87\x82\x01R\x82R\x92\x84\x01\x92\x90\x85\x01\x90aY@V[\x93\x86\x01\x93\x90\x93RP\x92\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aY\x9FW`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aY\xCDWaY\xCDaV\x81V[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aY\xF9WaY\xF9aV\x81V[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aZ@WaZ@aZ\x02V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aZ\x94WaZ\x94aV\x81V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15aZ\xD3WaZ\xD3aV\x81V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a[\x07Wa[\x07aV\x81V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a[NWa[NaV\x81V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a[\x89Wa[\x89aV\x81V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a[\xA5Wa[\xA5aV\x81V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a[\xBBWa[\xBBaV\x81V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\\\x03Wa\\\x03aV\x81V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\\7Wa\\7aV\x81V[PP\x01\x90V[`\0\x82a\\LWa\\LaZ\x02V[P\x04\x90V[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\\\x9C`\xC0\x83\x01\x84aS\x1AV[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\\\xE9W`\0\x80\xFD[\x81Q`\x03\x81\x10a/&W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x80Q`\xFF\x81\x16\x81\x14a]8W`\0\x80\xFD[\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a]OW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a]rWa]raO\x80V[`@R\x82Qa]\x80\x81aVRV[\x81Ra]\x8E` \x84\x01a]'V[` \x82\x01Ra]\x9F`@\x84\x01a]'V[`@\x82\x01R``\x83\x01Qa]\xB2\x81aVRV[``\x82\x01R`\x80\x83\x01Qa]\xC5\x81aVRV[`\x80\x82\x01R`\xA0\x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a]\xEDW`\0\x80\xFD[`\xA0\x82\x01R\x93\x92PPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a^*Wa^*aV\x81V[P`\x01\x01\x90V[`\0\x84Qa^C\x81\x84` \x89\x01aR\xEEV[\x91\x90\x91\x01\x92\x83RP` \x82\x01R`@\x01\x91\x90PV[`\0\x82\x19\x82\x11\x15a^kWa^kaV\x81V[P\x01\x90V[`\0`\xFF\x83\x16\x80a^\x83Wa^\x83aZ\x02V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a^\xACWa^\xACaV\x81V[\x90\x03\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106102385760003560e01c80638c3152e911610138578063bb2c727e116100b0578063cff0ab961161007f578063dad544e011610064578063dad544e01461082a578063e9e05c421461083f578063f2b4e6171461085257600080fd5b8063cff0ab9614610757578063d325d3bf146107f857600080fd5b8063bb2c727e14610648578063bda204bb146106ef578063bf653a5c14610704578063c0c53b8b1461073757600080fd5b80639bf62d8211610107578063a35d99df116100ec578063a35d99df146105db578063a3860f48146105fb578063b682c4441461061b57600080fd5b80639bf62d821461057e578063a14238e7146105ab57600080fd5b80638c3152e9146105095780638c90dd6514610529578063952b27971461054957806399a88ec41461055e57600080fd5b806345884d32116101cb57806354fd4d501161019a5780635c975abb1161017f5780635c975abb146104d457806371c1566e146104e95780638b4c40b01461025d57600080fd5b806354fd4d501461045b5780635c0cba33146104a757600080fd5b806345884d32146103a25780634870496f146103d25780634fd0434c146103f2578063513747ab1461042057600080fd5b80633c9f397c116102075780633c9f397c1461032e5780633e47158c1461035857806343ca1c501461036d578063452a93201461038d57600080fd5b80632152f2be1461026457806333d7e2bd1461028457806335e80ab3146102db57806338d38c97146102f057600080fd5b3661025f5761025d3334620186a0600060405180602001604052806000815250610867565b005b600080fd5b34801561027057600080fd5b5061025d61027f366004614f47565b610ab3565b34801561029057600080fd5b506037546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b3480156102e757600080fd5b506102b1610bf9565b3480156102fc57600080fd5b5060405160ff7f00000000000000000000000000000000000000000000000000000000000000001681526020016102d2565b34801561033a57600080fd5b50610343610c92565b60405163ffffffff90911681526020016102d2565b34801561036457600080fd5b506102b1610d26565b34801561037957600080fd5b5061025d610388366004615187565b610f31565b34801561039957600080fd5b506102b1611272565b3480156103ae57600080fd5b506103c26103bd3660046151ce565b6112e2565b60405190151581526020016102d2565b3480156103de57600080fd5b5061025d6103ed36600461524f565b61137d565b3480156103fe57600080fd5b506104076114ad565b60405167ffffffffffffffff90911681526020016102d2565b34801561042c57600080fd5b5061044d61043b3660046152d5565b6000908152603c602052604090205490565b6040519081526020016102d2565b34801561046757600080fd5b50604080518082018252600d81527f352e312e302b696e7465726f7000000000000000000000000000000000000000602082015290516102d29190615364565b3480156104b357600080fd5b50603e546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b3480156104e057600080fd5b506103c2611541565b3480156104f557600080fd5b5061025d610504366004615377565b6115d5565b34801561051557600080fd5b5061025d61052436600461539c565b6118d9565b34801561053557600080fd5b5061025d6105443660046153d9565b6118e6565b34801561055557600080fd5b5061044d61196a565b34801561056a57600080fd5b5061025d610579366004614f47565b6119fe565b34801561058a57600080fd5b506032546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b3480156105b757600080fd5b506103c26105c63660046152d5565b60336020526000908152604090205460ff1681565b3480156105e757600080fd5b506104076105f63660046154b9565b611bc0565b34801561060757600080fd5b506102b16106163660046154d6565b611bd9565b34801561062757600080fd5b50603f546102b19073ffffffffffffffffffffffffffffffffffffffff1681565b34801561065457600080fd5b506106ba610663366004615377565b603960209081526000928352604080842090915290825290205473ffffffffffffffffffffffffffffffffffffffff81169074010000000000000000000000000000000000000000900467ffffffffffffffff1682565b6040805173ffffffffffffffffffffffffffffffffffffffff909316835267ffffffffffffffff9091166020830152016102d2565b3480156106fb57600080fd5b5061025d611c1e565b34801561071057600080fd5b507f000000000000000000000000000000000000000000000000000000000000000061044d565b34801561074357600080fd5b5061025d6107523660046154f8565b611d00565b34801561076357600080fd5b506001546107bf906fffffffffffffffffffffffffffffffff81169067ffffffffffffffff7001000000000000000000000000000000008204811691780100000000000000000000000000000000000000000000000090041683565b604080516fffffffffffffffffffffffffffffffff909416845267ffffffffffffffff92831660208501529116908201526060016102d2565b34801561080457600080fd5b50603f546103c29074010000000000000000000000000000000000000000900460ff1681565b34801561083657600080fd5b506102b1611efd565b61025d61084d366004615551565b610867565b34801561085e57600080fd5b506102b1611f51565b8260005a905034156108f757603f60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16631ee116bf346040518263ffffffff1660e01b81526004016000604051808303818588803b1580156108dd57600080fd5b505af11580156108f1573d6000803e3d6000fd5b50505050505b838015610919575073ffffffffffffffffffffffffffffffffffffffff871615155b15610950576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61095a8351611bc0565b67ffffffffffffffff168567ffffffffffffffff1610156109a7576040517f70c8bdbd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6201d4c0835111156109e5576040517f5aa3bac900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336109ee611fc1565b610a0b575033731111000000000000000000000000000000001111015b60003488888888604051602001610a269594939291906155d0565b604051602081830303815290604052905060008973ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff167fb3813568d9991fc951961fcb4c784893574240a28925604d09fc577c55bb7c3284604051610a969190615364565b60405180910390a45050610aaa8282611fff565b50505050505050565b610abb6122d6565b610ac3612315565b603e5473ffffffffffffffffffffffffffffffffffffffff808316911603610b17576040517f785df91100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603f8054603e805473ffffffffffffffffffffffffffffffffffffffff8581167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681179093557fffffffffffffffffffffff0000000000000000000000000000000000000000008416878216908117740100000000000000000000000000000000000000001790955560408051948216808652602086019690965291169083018190526060830191909152907f9e5368471a58d81987e5dc7d6374dd5ed5e756cc95a79ff726903423bce0060d906080015b60405180910390a150505050565b603754604080517f35e80ab3000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff16916335e80ab39160048083019260209291908290030181865afa158015610c69573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d9190615635565b905090565b603e54604080517f3c9f397c000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff1691633c9f397c9160048083019260209291908290030181865afa158015610d02573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d9190615664565b600080610d517fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035490565b905073ffffffffffffffffffffffffffffffffffffffff811615610d7457919050565b6040518060400160405280601a81526020017f4f564d5f4c3143726f7373446f6d61696e4d657373656e676572000000000000815250516002610db791906156b0565b604080513060208201526000918101919091527f4f564d5f4c3143726f7373446f6d61696e4d657373656e6765720000000000009190911790610e12906060015b604051602081830303815290604052805190602001205490565b14610e49576040517f54e433cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408051306020820152600191810191909152600090610e6b90606001610df8565b905073ffffffffffffffffffffffffffffffffffffffff811615610eff578073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ed4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ef89190615635565b9250505090565b6040517f332144db00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610f396122d6565b60325473ffffffffffffffffffffffffffffffffffffffff1661dead14610f8c576040517fdfeaaeb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610f99826040015161236b565b15610fd0576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000610fdb836123ae565b9050610fe781836115d5565b600081815260336020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011790556060830151156110b857603f5460608401516040517f8d445bd000000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90921691638d445bd0916110859160040190815260200190565b600060405180830381600087803b15801561109f57600080fd5b505af11580156110b3573d6000803e3d6000fd5b505050505b8260200151603260006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600061111b8460400151856080015186606001518760a001516123fb565b603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead17905560405190915082907fdb5c7652857aa163daadd670e116628fb42e869d8ac4251ef8971d9e5727df1b9061118090841515815260200190565b60405180910390a28015801561119a575060008460600151115b1561122757603f60009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16631ee116bf85606001516040518263ffffffff1660e01b81526004016000604051808303818588803b15801561120d57600080fd5b505af1158015611221573d6000803e3d6000fd5b50505050505b801580156112355750326001145b1561126c576040517fab58103600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505050565b603754604080517f452a9320000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163452a93209160048083019260209291908290030181865afa158015610c69573d6000803e3d6000fd5b603e546040517f45884d3200000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff838116600483015260009216906345884d3290602401602060405180830381865afa158015611353573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061137791906156ed565b92915050565b6113856122d6565b603f5474010000000000000000000000000000000000000000900460ff16156113da576040517f5e74b54200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006113e4611f51565b73ffffffffffffffffffffffffffffffffffffffff1663bb8aa1fc866040518263ffffffff1660e01b815260040161141e91815260200190565b606060405180830381865afa15801561143b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061145f919061570a565b6040805160608082018352600080835260208301529181019190915290935091506114879050565b610aaa878360008461149e368b90038b018b61574c565b6114a8898b6157d6565b612459565b603e54604080517f4086d183000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff1691634086d1839160048083019260209291908290030181865afa15801561151d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d919061584f565b603754604080517f5c975abb000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff1691635c975abb9160048083019260209291908290030181865afa1580156115b1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d91906156ed565b600082815260396020908152604080832073ffffffffffffffffffffffffffffffffffffffff85811685529083528184208251808401845290549182168082527401000000000000000000000000000000000000000090920467ffffffffffffffff1681850152868552603390935292205490919060ff1615611684576040517f730a107400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b816020015167ffffffffffffffff166000036116cc576040517fcca6afda00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61174b8173ffffffffffffffffffffffffffffffffffffffff1663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561171a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061173e919061584f565b67ffffffffffffffff1690565b67ffffffffffffffff16826020015167ffffffffffffffff161161179b576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f0000000000000000000000000000000000000000000000000000000000000000826020015167ffffffffffffffff16426117d6919061586c565b1161180d576040517fd9bc01be00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f6c4f446700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff838116600483015290911690636c4f446790602401602060405180830381865afa15801561187d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118a191906156ed565b61126c576040517f332a57f800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6118e38133610f31565b50565b6118ee6122d6565b603f5474010000000000000000000000000000000000000000900460ff16611942576040517f5e74b54200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610aaa87878761195188615883565b6119603689900389018961574c565b6114a887896157d6565b603e54604080517f952b2797000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163952b27979160048083019260209291908290030181865afa1580156119da573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c8d919061598d565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff16158015611a3e575060005460ff8083169116105b611acf576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a656400000000000000000000000000000000000060648201526084015b60405180910390fd5b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff831617610100179055611b08612cb4565b603e805473ffffffffffffffffffffffffffffffffffffffff8581167fffffffffffffffffffffffff000000000000000000000000000000000000000092831617909255603f805492851692909116919091179055600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b6000611bcd8260286159a6565b611377906152086159d6565b603c6020528160005260406000208181548110611bf557600080fd5b60009182526020909120015473ffffffffffffffffffffffffffffffffffffffff169150829050565b611c26612315565b603f54604080517f1ee116bf0000000000000000000000000000000000000000000000000000000081529051479273ffffffffffffffffffffffffffffffffffffffff1691631ee116bf91849160048082019260009290919082900301818588803b158015611c9457600080fd5b505af1158015611ca8573d6000803e3d6000fd5b5050603f5460405185815273ffffffffffffffffffffffffffffffffffffffff90911693507fd893f630c6867fa43689da9ae949ebf04cac24aad3b45c759d442ed3c32e3a379250602001905060405180910390a250565b7f0000000000000000000000000000000000000000000000000000000000000000600054610100900460ff16158015611d40575060005460ff8083169116105b611dcc576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201527f647920696e697469616c697a65640000000000000000000000000000000000006064820152608401611ac6565b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00001660ff831617610100179055611e05612cb4565b6037805473ffffffffffffffffffffffffffffffffffffffff8087167fffffffffffffffffffffffff000000000000000000000000000000000000000092831617909255603e8054868416908316179055603f8054858416921691909117905560325416611e9a57603280547fffffffffffffffffffffffff00000000000000000000000000000000000000001661dead1790555b611ea2612d35565b600080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602001610beb565b6000611f07610d26565b73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c69573d6000803e3d6000fd5b603e54604080517ff2b4e617000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163f2b4e6179160048083019260209291908290030181865afa158015610c69573d6000803e3d6000fd5b6000323303611fd05750600190565b333b601703611ff857604051602081016040526020600082333c5160e81c62ef010014905090565b5060005b90565b600154600090612035907801000000000000000000000000000000000000000000000000900467ffffffffffffffff164361586c565b90506000612041612e48565b90506000816020015160ff16826000015163ffffffff166120629190615a31565b9050821561219957600154600090612099908390700100000000000000000000000000000000900467ffffffffffffffff16615a99565b90506000836040015160ff16836120b09190615b0d565b6001546120d09084906fffffffffffffffffffffffffffffffff16615b0d565b6120da9190615a31565b60015490915060009061212b906121049084906fffffffffffffffffffffffffffffffff16615bc9565b866060015163ffffffff168760a001516fffffffffffffffffffffffffffffffff16612f0e565b9050600186111561215a5761215761210482876040015160ff1660018a612152919061586c565b612f2d565b90505b6fffffffffffffffffffffffffffffffff16780100000000000000000000000000000000000000000000000067ffffffffffffffff4316021760015550505b600180548691906010906121cc908490700100000000000000000000000000000000900467ffffffffffffffff166159d6565b92506101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550816000015163ffffffff16600160000160109054906101000a900467ffffffffffffffff1667ffffffffffffffff161315612259576040517f77ebef4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600154600090612285906fffffffffffffffffffffffffffffffff1667ffffffffffffffff88166156b0565b9050600061229748633b9aca00612f82565b6122a19083615c3d565b905060005a6122b0908861586c565b9050808211156122cc576122cc6122c7828461586c565b612f99565b5050505050505050565b6122de611541565b156118d7576040517fb9c3c2ef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b3361231e611efd565b73ffffffffffffffffffffffffffffffffffffffff16146118d7576040517f7f12c64b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff8216301480611377575050603f5473ffffffffffffffffffffffffffffffffffffffff90811691161490565b80516020808301516040808501516060860151608087015160a088015193516000976123de979096959101615c51565b604051602081830303815290604052805190602001209050919050565b600080600061240b866000612fc7565b905080612441576308c379a06000526020805278185361666543616c6c3a204e6f7420656e6f756768206761736058526064601cfd5b600080855160208701888b5af1979650505050505050565b612466866040015161236b565b1561249d576040517fc5defbad00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f496b9c1600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff87811660048301529091169063496b9c1690602401602060405180830381865afa15801561250d573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061253191906156ed565b612567576040517ff395240e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603e546040517f04e50fed00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8781166004830152909116906304e50fed90602401602060405180830381865afa1580156125d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125fb91906156ed565b612631576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018573ffffffffffffffffffffffffffffffffffffffff1663200d2ed26040518163ffffffff1660e01b8152600401602060405180830381865afa15801561267e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126a29190615cd7565b60028111156126b3576126b3615ca8565b036126ea576040517fe29927ed00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6127388573ffffffffffffffffffffffffffffffffffffffff1663cf09e0d06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561171a573d6000803e3d6000fd5b67ffffffffffffffff16421161277a576040517fb4caa4e500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603f5474010000000000000000000000000000000000000000900460ff16156129c4576127a683612fe5565b6128188673ffffffffffffffffffffffffffffffffffffffff1663bcef3b556040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127f4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ffc919061598d565b1461284f576040517f2b1a9a6600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826040015151841061288d576040517f32dc285c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000836040015185815181106128a5576128a5615cf8565b60200260200101519050603760009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d6ae3cd56040518163ffffffff1660e01b8152600401602060405180830381865afa15801561291c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612940919061598d565b815114612979576040517f7cc2f31b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61298283612ffe565b8160200151146129be576040517f426149af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50612a52565b6129cd82612ffe565b612a1b8673ffffffffffffffffffffffffffffffffffffffff1663bcef3b556040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127f4573d6000803e3d6000fd5b14612a52576040517f426149af00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000612a5d876123ae565b90506000816000604051602001612a7e929190918252602082015260400190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081840301815282825280516020918201209083018190529250612b299101604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181528282018252600183527f0100000000000000000000000000000000000000000000000000000000000000602084015290870151909190869061303d565b1515600003612b64576040517f2e57ef3a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408051808201825273ffffffffffffffffffffffffffffffffffffffff808a16825267ffffffffffffffff42811660208085019182526000888152603982528681203380835290835287822096518754945190951674010000000000000000000000000000000000000000027fffffffff000000000000000000000000000000000000000000000000000000009094169486169490941792909217909455868152603c845284812080546001810182559082528482200180547fffffffffffffffffffffffff0000000000000000000000000000000000000000169092179091558b840151928c01519351928216939091169185917f67a6208cfcc0801d50f6cbe764733f4fddf66ac0b04442061a8a8c0cb6b63f6291a4604051339083907f798f9f13695f8f045aa5f80ed8efebb695f3c7fe65da381969f2f28bf3c60b9790600090a35050505050505050565b33612cbd610d26565b73ffffffffffffffffffffffffffffffffffffffff1614158015612cfe575033612ce5611efd565b73ffffffffffffffffffffffffffffffffffffffff1614155b156118d7576040517fc4050a2600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600054610100900460ff16612dcc576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201527f6e697469616c697a696e670000000000000000000000000000000000000000006064820152608401611ac6565b6001547801000000000000000000000000000000000000000000000000900467ffffffffffffffff166000036118d75760408051606081018252633b9aca00808252600060208301524367ffffffffffffffff169190920181905278010000000000000000000000000000000000000000000000000217600155565b6040805160c081018252600080825260208201819052918101829052606081018290526080810182905260a0810191909152603754604080517fcc731b02000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163cc731b029160048083019260c09291908290030181865afa158015612eea573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113779190615d3d565b6000612f23612f1d8585613061565b83613071565b90505b9392505050565b6000670de0b6b3a7640000612f6e612f458583615a31565b612f5790670de0b6b3a7640000615a99565b612f6985670de0b6b3a7640000615b0d565b613080565b612f789086615b0d565b612f239190615a31565b600081831015612f925781612f26565b5090919050565b6000805a90505b825a612fac908361586c565b1015612fc257612fbb82615df9565b9150612fa0565b505050565b600080603f83619c4001026040850201603f5a021015949350505050565b6000612ff0826130b1565b805190602001209050919050565b600081600001518260200151836040015184606001516040516020016123de949392919093845260208401929092526040830152606082015260800190565b60008061304986613267565b905061305781868686613299565b9695505050505050565b600081831215612f925781612f26565b6000818312612f925781612f26565b6000612f26670de0b6b3a764000083613098866132c9565b6130a29190615b0d565b6130ac9190615a31565b61350d565b80516060907fff00000000000000000000000000000000000000000000000000000000000000167f010000000000000000000000000000000000000000000000000000000000000014613130576040517fc06b523800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81604001515160000361316f576040517f9103e7cd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008260000151836020015160c01b6040516020016131de9291907fff000000000000000000000000000000000000000000000000000000000000009290921682527fffffffffffffffff00000000000000000000000000000000000000000000000016600182015260090190565b604051602081830303815290604052905060005b8360400151518110156132605760008460400151828151811061321757613217615cf8565b60209081029190910181015180518183015160405192945061323b93879301615e31565b604051602081830303815290604052925050808061325890615df9565b9150506131f2565b5092915050565b6060818051906020012060405160200161328391815260200190565b6040516020818303038152906040529050919050565b60006132c0846132aa87868661374c565b8051602091820120825192909101919091201490565b95945050505050565b6000808213613334576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611ac6565b60006060613341846141ca565b03609f8181039490941b90931c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506027d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b393909302929092017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d92915050565b60007ffffffffffffffffffffffffffffffffffffffffffffffffdb731c958f34d94c1821361353e57506000919050565b680755bf798b4a1bf1e582126135b0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f4558505f4f564552464c4f5700000000000000000000000000000000000000006044820152606401611ac6565b6503782dace9d9604e83901b059150600060606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b606060008451116137b9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601560248201527f4d65726b6c65547269653a20656d707479206b657900000000000000000000006044820152606401611ac6565b60006137c4846142a0565b905060006137d18661438c565b90506000846040516020016137e891815260200190565b60405160208183030381529060405290506000805b845181101561414157600085828151811061381a5761381a615cf8565b6020026020010151905084518311156138b5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602e60248201527f4d65726b6c65547269653a206b657920696e646578206578636565647320746f60448201527f74616c206b6579206c656e6774680000000000000000000000000000000000006064820152608401611ac6565b8260000361396e5780518051602091820120604051613903926138dd92910190815260200190565b604051602081830303815290604052858051602091820120825192909101919091201490565b613969576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f4d65726b6c65547269653a20696e76616c696420726f6f7420686173680000006044820152606401611ac6565b613ac5565b805151602011613a245780518051602091820120604051613998926138dd92910190815260200190565b613969576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602760248201527f4d65726b6c65547269653a20696e76616c6964206c6172676520696e7465726e60448201527f616c2068617368000000000000000000000000000000000000000000000000006064820152608401611ac6565b805184516020808701919091208251919092012014613ac5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4d65726b6c65547269653a20696e76616c696420696e7465726e616c206e6f6460448201527f65206861736800000000000000000000000000000000000000000000000000006064820152608401611ac6565b613ad160106001615e58565b81602001515103613cad5784518303613c4557613b0b8160200151601081518110613afe57613afe615cf8565b60200260200101516143ef565b96506000875111613b9e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603b60248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286272616e63682900000000006064820152608401611ac6565b60018651613bac919061586c565b8214613c3a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286272616e6368290000000000006064820152608401611ac6565b505050505050612f26565b6000858481518110613c5957613c59615cf8565b602001015160f81c60f81b60f81c9050600082602001518260ff1681518110613c8457613c84615cf8565b60200260200101519050613c97816144a3565b9550613ca4600186615e58565b9450505061412e565b6002816020015151036140a6576000613cc5826144c8565b9050600081600081518110613cdc57613cdc615cf8565b016020015160f81c90506000613cf3600283615e70565b613cfe906002615e92565b90506000613d0f848360ff166144ec565b90506000613d1d8a896144ec565b90506000613d2b8383614522565b905080835114613dbd576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f4d65726b6c65547269653a20706174682072656d61696e646572206d7573742060448201527f736861726520616c6c206e6962626c65732077697468206b65790000000000006064820152608401611ac6565b60ff851660021480613dd2575060ff85166003145b15613fc15780825114613e67576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603d60248201527f4d65726b6c65547269653a206b65792072656d61696e646572206d757374206260448201527f65206964656e746963616c20746f20706174682072656d61696e6465720000006064820152608401611ac6565b613e818760200151600181518110613afe57613afe615cf8565b9c5060008d5111613f14576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603960248201527f4d65726b6c65547269653a2076616c7565206c656e677468206d75737420626560448201527f2067726561746572207468616e207a65726f20286c65616629000000000000006064820152608401611ac6565b60018c51613f22919061586c565b8814613fb0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603860248201527f4d65726b6c65547269653a2076616c7565206e6f6465206d757374206265206c60448201527f617374206e6f646520696e2070726f6f6620286c6561662900000000000000006064820152608401611ac6565b505050505050505050505050612f26565b60ff85161580613fd4575060ff85166001145b15614013576140008760200151600181518110613ff357613ff3615cf8565b60200260200101516144a3565b995061400c818a615e58565b985061409b565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603260248201527f4d65726b6c65547269653a2072656365697665642061206e6f6465207769746860448201527f20616e20756e6b6e6f776e2070726566697800000000000000000000000000006064820152608401611ac6565b50505050505061412e565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602860248201527f4d65726b6c65547269653a20726563656976656420616e20756e70617273656160448201527f626c65206e6f64650000000000000000000000000000000000000000000000006064820152608401611ac6565b508061413981615df9565b9150506137fd565b506040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f4d65726b6c65547269653a2072616e206f7574206f662070726f6f6620656c6560448201527f6d656e74730000000000000000000000000000000000000000000000000000006064820152608401611ac6565b6000808211614235576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600960248201527f554e444546494e454400000000000000000000000000000000000000000000006044820152606401611ac6565b5060016fffffffffffffffffffffffffffffffff821160071b82811c67ffffffffffffffff1060061b1782811c63ffffffff1060051b1782811c61ffff1060041b1782811c60ff10600390811b90911783811c600f1060021b1783811c909110821b1791821c111790565b80516060908067ffffffffffffffff8111156142be576142be614f80565b60405190808252806020026020018201604052801561430357816020015b60408051808201909152606080825260208201528152602001906001900390816142dc5790505b50915060005b8181101561438557604051806040016040528085838151811061432e5761432e615cf8565b6020026020010151815260200161435d86848151811061435057614350615cf8565b60200260200101516145cf565b81525083828151811061437257614372615cf8565b6020908102919091010152600101614309565b5050919050565b606080604051905082518060011b603f8101601f1916830160405280835250602084016020830160005b838110156143e4578060011b82018184015160001a8060041c8253600f8116600183015350506001016143b6565b509295945050505050565b606060008060006143ff856145e2565b91945092509050600081600181111561441a5761441a615ca8565b14614451576040517f1ff9b2e400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61445b8284615e58565b855114614494576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6132c085602001518484614a80565b606060208260000151106144bf576144ba826143ef565b611377565b61137782614b14565b60606113776144e78360200151600081518110613afe57613afe615cf8565b61438c565b60608251821061450b5750604080516020810190915260008152611377565b612f26838384865161451d919061586c565b614b2a565b6000808251845110614535578251614538565b83515b90505b80821080156145bf575082828151811061455757614557615cf8565b602001015160f81c60f81b7effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191684838151811061459657614596615cf8565b01602001517fff0000000000000000000000000000000000000000000000000000000000000016145b156132605781600101915061453b565b60606113776145dd83614d02565b614d6f565b60008060008360000151600003614625576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6020840151805160001a607f811161464a576000600160009450945094505050614a79565b60b7811161476057600061465f60808361586c565b90508087600001511161469e576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001838101517fff0000000000000000000000000000000000000000000000000000000000000016908214801561471657507f80000000000000000000000000000000000000000000000000000000000000007fff000000000000000000000000000000000000000000000000000000000000008216105b1561474d576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5060019550935060009250614a79915050565b60bf81116148be57600061477560b78361586c565b9050808760000151116147b4576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff00000000000000000000000000000000000000000000000000000000000000166000819003614816576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c6037811161485e576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6148688184615e58565b8951116148a1576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6148ac836001615e58565b9750955060009450614a799350505050565b60f781116149235760006148d360c08361586c565b905080876000015111614912576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600195509350849250614a79915050565b600061493060f78361586c565b90508087600001511161496f576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60018301517fff000000000000000000000000000000000000000000000000000000000000001660008190036149d1576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600184015160088302610100031c60378111614a19576040517fbabb01dd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614a238184615e58565b895111614a5c576040517f66c9448500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b614a67836001615e58565b9750955060019450614a799350505050565b9193909250565b60608167ffffffffffffffff811115614a9b57614a9b614f80565b6040519080825280601f01601f191660200182016040528015614ac5576020820181803683370190505b5090508115612f26576000614ada8486615e58565b90506020820160005b84811015614afb578281015182820152602001614ae3565b84811115614b0a576000858301525b5050509392505050565b6060611377826020015160008460000151614a80565b60608182601f011015614b99576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611ac6565b828284011015614c05576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f736c6963655f6f766572666c6f770000000000000000000000000000000000006044820152606401611ac6565b81830184511015614c72576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f736c6963655f6f75744f66426f756e64730000000000000000000000000000006044820152606401611ac6565b606082158015614c915760405191506000825260208201604052614cf9565b6040519150601f8416801560200281840101858101878315602002848b0101015b81831015614cca578051835260209283019201614cb2565b5050858452601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016604052505b50949350505050565b60408051808201909152600080825260208201528151600003614d51576040517f5ab458fb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50604080518082019091528151815260209182019181019190915290565b60606000806000614d7f856145e2565b919450925090506001816001811115614d9a57614d9a615ca8565b14614dd1576040517f4b9c6abe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8451614ddd8385615e58565b14614e14576040517f5c5537b800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b604080516020808252610420820190925290816020015b6040805180820190915260008082526020820152815260200190600190039081614e2b5790505093506000835b8651811015614f1957600080614e9e6040518060400160405280858c60000151614e82919061586c565b8152602001858c60200151614e979190615e58565b90526145e2565b509150915060405180604001604052808383614eba9190615e58565b8152602001848b60200151614ecf9190615e58565b815250888581518110614ee457614ee4615cf8565b6020908102919091010152614efa600185615e58565b9350614f068183615e58565b614f109084615e58565b92505050614e58565b50845250919392505050565b73ffffffffffffffffffffffffffffffffffffffff811681146118e357600080fd5b60008060408385031215614f5a57600080fd5b8235614f6581614f25565b91506020830135614f7581614f25565b809150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040516060810167ffffffffffffffff81118282101715614fd257614fd2614f80565b60405290565b6040805190810167ffffffffffffffff81118282101715614fd257614fd2614f80565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff8111828210171561504257615042614f80565b604052919050565b600082601f83011261505b57600080fd5b813567ffffffffffffffff81111561507557615075614f80565b6150a660207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601614ffb565b8181528460208386010111156150bb57600080fd5b816020850160208301376000918101602001919091529392505050565b600060c082840312156150ea57600080fd5b60405160c0810167ffffffffffffffff828210818311171561510e5761510e614f80565b81604052829350843583526020850135915061512982614f25565b8160208401526040850135915061513f82614f25565b816040840152606085013560608401526080850135608084015260a085013591508082111561516d57600080fd5b5061517a8582860161504a565b60a0830152505092915050565b6000806040838503121561519a57600080fd5b823567ffffffffffffffff8111156151b157600080fd5b6151bd858286016150d8565b9250506020830135614f7581614f25565b6000602082840312156151e057600080fd5b8135612f2681614f25565b6000608082840312156151fd57600080fd5b50919050565b60008083601f84011261521557600080fd5b50813567ffffffffffffffff81111561522d57600080fd5b6020830191508360208260051b850101111561524857600080fd5b9250929050565b600080600080600060e0868803121561526757600080fd5b853567ffffffffffffffff8082111561527f57600080fd5b61528b89838a016150d8565b9650602088013595506152a18960408a016151eb565b945060c08801359150808211156152b757600080fd5b506152c488828901615203565b969995985093965092949392505050565b6000602082840312156152e757600080fd5b5035919050565b60005b838110156153095781810151838201526020016152f1565b8381111561126c5750506000910152565b600081518084526153328160208601602086016152ee565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169290920160200192915050565b602081526000612f26602083018461531a565b6000806040838503121561538a57600080fd5b823591506020830135614f7581614f25565b6000602082840312156153ae57600080fd5b813567ffffffffffffffff8111156153c557600080fd5b6153d1848285016150d8565b949350505050565b6000806000806000806000610120888a0312156153f557600080fd5b873567ffffffffffffffff8082111561540d57600080fd5b6154198b838c016150d8565b985060208a0135915061542b82614f25565b909650604089013595506060890135908082111561544857600080fd5b908901906060828c03121561545c57600080fd5b81955061546c8b60808c016151eb565b94506101008a013591508082111561548357600080fd5b506154908a828b01615203565b989b979a50959850939692959293505050565b67ffffffffffffffff811681146118e357600080fd5b6000602082840312156154cb57600080fd5b8135612f26816154a3565b600080604083850312156154e957600080fd5b50508035926020909101359150565b60008060006060848603121561550d57600080fd5b833561551881614f25565b9250602084013561552881614f25565b9150604084013561553881614f25565b809150509250925092565b80151581146118e357600080fd5b600080600080600060a0868803121561556957600080fd5b853561557481614f25565b945060208601359350604086013561558b816154a3565b9250606086013561559b81615543565b9150608086013567ffffffffffffffff8111156155b757600080fd5b6155c38882890161504a565b9150509295509295909350565b8581528460208201527fffffffffffffffff0000000000000000000000000000000000000000000000008460c01b16604082015282151560f81b6048820152600082516156248160498501602087016152ee565b919091016049019695505050505050565b60006020828403121561564757600080fd5b8151612f2681614f25565b63ffffffff811681146118e357600080fd5b60006020828403121561567657600080fd5b8151612f2681615652565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156156e8576156e8615681565b500290565b6000602082840312156156ff57600080fd5b8151612f2681615543565b60008060006060848603121561571f57600080fd5b835161572a81615652565b602085015190935061573b816154a3565b604085015190925061553881614f25565b60006080828403121561575e57600080fd5b6040516080810181811067ffffffffffffffff8211171561578157615781614f80565b8060405250823581526020830135602082015260408301356040820152606083013560608201528091505092915050565b600067ffffffffffffffff8211156157cc576157cc614f80565b5060051b60200190565b60006157e96157e4846157b2565b614ffb565b80848252602080830192508560051b85013681111561580757600080fd5b855b8181101561584357803567ffffffffffffffff8111156158295760008081fd5b61583536828a0161504a565b865250938201938201615809565b50919695505050505050565b60006020828403121561586157600080fd5b8151612f26816154a3565b60008282101561587e5761587e615681565b500390565b60006060823603121561589557600080fd5b61589d614faf565b82357fff00000000000000000000000000000000000000000000000000000000000000811681146158cd57600080fd5b81526020838101356158de816154a3565b8282015260408481013567ffffffffffffffff8111156158fd57600080fd5b850136601f82011261590e57600080fd5b803561591c6157e4826157b2565b81815260069190911b8201840190848101903683111561593b57600080fd5b928501925b8284101561597b578484360312156159585760008081fd5b615960614fd8565b84358152868501358782015282529284019290850190615940565b93860193909352509295945050505050565b60006020828403121561599f57600080fd5b5051919050565b600067ffffffffffffffff808316818516818304811182151516156159cd576159cd615681565b02949350505050565b600067ffffffffffffffff8083168185168083038211156159f9576159f9615681565b01949350505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b600082615a4057615a40615a02565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83147f800000000000000000000000000000000000000000000000000000000000000083141615615a9457615a94615681565b500590565b6000808312837f800000000000000000000000000000000000000000000000000000000000000001831281151615615ad357615ad3615681565b837f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff018313811615615b0757615b07615681565b50500390565b60007f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600084136000841385830485118282161615615b4e57615b4e615681565b7f80000000000000000000000000000000000000000000000000000000000000006000871286820588128184161615615b8957615b89615681565b60008712925087820587128484161615615ba557615ba5615681565b87850587128184161615615bbb57615bbb615681565b505050929093029392505050565b6000808212827f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03841381151615615c0357615c03615681565b827f8000000000000000000000000000000000000000000000000000000000000000038412811615615c3757615c37615681565b50500190565b600082615c4c57615c4c615a02565b500490565b868152600073ffffffffffffffffffffffffffffffffffffffff808816602084015280871660408401525084606083015283608083015260c060a0830152615c9c60c083018461531a565b98975050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b600060208284031215615ce957600080fd5b815160038110612f2657600080fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b805160ff81168114615d3857600080fd5b919050565b600060c08284031215615d4f57600080fd5b60405160c0810181811067ffffffffffffffff82111715615d7257615d72614f80565b6040528251615d8081615652565b8152615d8e60208401615d27565b6020820152615d9f60408401615d27565b60408201526060830151615db281615652565b60608201526080830151615dc581615652565b608082015260a08301516fffffffffffffffffffffffffffffffff81168114615ded57600080fd5b60a08201529392505050565b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203615e2a57615e2a615681565b5060010190565b60008451615e438184602089016152ee565b91909101928352506020820152604001919050565b60008219821115615e6b57615e6b615681565b500190565b600060ff831680615e8357615e83615a02565b8060ff84160691505092915050565b600060ff821660ff841680821015615eac57615eac615681565b9003939250505056fea164736f6c634300080f000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x028W`\x005`\xE0\x1C\x80c\x8C1R\xE9\x11a\x018W\x80c\xBB,r~\x11a\0\xB0W\x80c\xCF\xF0\xAB\x96\x11a\0\x7FW\x80c\xDA\xD5D\xE0\x11a\0dW\x80c\xDA\xD5D\xE0\x14a\x08*W\x80c\xE9\xE0\\B\x14a\x08?W\x80c\xF2\xB4\xE6\x17\x14a\x08RW`\0\x80\xFD[\x80c\xCF\xF0\xAB\x96\x14a\x07WW\x80c\xD3%\xD3\xBF\x14a\x07\xF8W`\0\x80\xFD[\x80c\xBB,r~\x14a\x06HW\x80c\xBD\xA2\x04\xBB\x14a\x06\xEFW\x80c\xBFe:\\\x14a\x07\x04W\x80c\xC0\xC5;\x8B\x14a\x077W`\0\x80\xFD[\x80c\x9B\xF6-\x82\x11a\x01\x07W\x80c\xA3]\x99\xDF\x11a\0\xECW\x80c\xA3]\x99\xDF\x14a\x05\xDBW\x80c\xA3\x86\x0FH\x14a\x05\xFBW\x80c\xB6\x82\xC4D\x14a\x06\x1BW`\0\x80\xFD[\x80c\x9B\xF6-\x82\x14a\x05~W\x80c\xA1B8\xE7\x14a\x05\xABW`\0\x80\xFD[\x80c\x8C1R\xE9\x14a\x05\tW\x80c\x8C\x90\xDDe\x14a\x05)W\x80c\x95+'\x97\x14a\x05IW\x80c\x99\xA8\x8E\xC4\x14a\x05^W`\0\x80\xFD[\x80cE\x88M2\x11a\x01\xCBW\x80cT\xFDMP\x11a\x01\x9AW\x80c\\\x97Z\xBB\x11a\x01\x7FW\x80c\\\x97Z\xBB\x14a\x04\xD4W\x80cq\xC1Vn\x14a\x04\xE9W\x80c\x8BL@\xB0\x14a\x02]W`\0\x80\xFD[\x80cT\xFDMP\x14a\x04[W\x80c\\\x0C\xBA3\x14a\x04\xA7W`\0\x80\xFD[\x80cE\x88M2\x14a\x03\xA2W\x80cHpIo\x14a\x03\xD2W\x80cO\xD0CL\x14a\x03\xF2W\x80cQ7G\xAB\x14a\x04 W`\0\x80\xFD[\x80c<\x9F9|\x11a\x02\x07W\x80c<\x9F9|\x14a\x03.W\x80c>G\x15\x8C\x14a\x03XW\x80cC\xCA\x1CP\x14a\x03mW\x80cE*\x93 \x14a\x03\x8DW`\0\x80\xFD[\x80c!R\xF2\xBE\x14a\x02dW\x80c3\xD7\xE2\xBD\x14a\x02\x84W\x80c5\xE8\n\xB3\x14a\x02\xDBW\x80c8\xD3\x8C\x97\x14a\x02\xF0W`\0\x80\xFD[6a\x02_Wa\x02]34b\x01\x86\xA0`\0`@Q\x80` \x01`@R\x80`\0\x81RPa\x08gV[\0[`\0\x80\xFD[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x02]a\x02\x7F6`\x04aOGV[a\n\xB3V[4\x80\x15a\x02\x90W`\0\x80\xFD[P`7Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xE7W`\0\x80\xFD[Pa\x02\xB1a\x0B\xF9V[4\x80\x15a\x02\xFCW`\0\x80\xFD[P`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x02\xD2V[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x03Ca\x0C\x92V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xD2V[4\x80\x15a\x03dW`\0\x80\xFD[Pa\x02\xB1a\r&V[4\x80\x15a\x03yW`\0\x80\xFD[Pa\x02]a\x03\x886`\x04aQ\x87V[a\x0F1V[4\x80\x15a\x03\x99W`\0\x80\xFD[Pa\x02\xB1a\x12rV[4\x80\x15a\x03\xAEW`\0\x80\xFD[Pa\x03\xC2a\x03\xBD6`\x04aQ\xCEV[a\x12\xE2V[`@Q\x90\x15\x15\x81R` \x01a\x02\xD2V[4\x80\x15a\x03\xDEW`\0\x80\xFD[Pa\x02]a\x03\xED6`\x04aROV[a\x13}V[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x04\x07a\x14\xADV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xD2V[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x04Ma\x04;6`\x04aR\xD5V[`\0\x90\x81R`<` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\xD2V[4\x80\x15a\x04gW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\r\x81R\x7F5.1.0+interop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x02\xD2\x91\x90aSdV[4\x80\x15a\x04\xB3W`\0\x80\xFD[P`>Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x03\xC2a\x15AV[4\x80\x15a\x04\xF5W`\0\x80\xFD[Pa\x02]a\x05\x046`\x04aSwV[a\x15\xD5V[4\x80\x15a\x05\x15W`\0\x80\xFD[Pa\x02]a\x05$6`\x04aS\x9CV[a\x18\xD9V[4\x80\x15a\x055W`\0\x80\xFD[Pa\x02]a\x05D6`\x04aS\xD9V[a\x18\xE6V[4\x80\x15a\x05UW`\0\x80\xFD[Pa\x04Ma\x19jV[4\x80\x15a\x05jW`\0\x80\xFD[Pa\x02]a\x05y6`\x04aOGV[a\x19\xFEV[4\x80\x15a\x05\x8AW`\0\x80\xFD[P`2Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x03\xC2a\x05\xC66`\x04aR\xD5V[`3` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xE7W`\0\x80\xFD[Pa\x04\x07a\x05\xF66`\x04aT\xB9V[a\x1B\xC0V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x02\xB1a\x06\x166`\x04aT\xD6V[a\x1B\xD9V[4\x80\x15a\x06'W`\0\x80\xFD[P`?Ta\x02\xB1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x06TW`\0\x80\xFD[Pa\x06\xBAa\x06c6`\x04aSwV[`9` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xD2V[4\x80\x15a\x06\xFBW`\0\x80\xFD[Pa\x02]a\x1C\x1EV[4\x80\x15a\x07\x10W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04MV[4\x80\x15a\x07CW`\0\x80\xFD[Pa\x02]a\x07R6`\x04aT\xF8V[a\x1D\0V[4\x80\x15a\x07cW`\0\x80\xFD[P`\x01Ta\x07\xBF\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x02\xD2V[4\x80\x15a\x08\x04W`\0\x80\xFD[P`?Ta\x03\xC2\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[4\x80\x15a\x086W`\0\x80\xFD[Pa\x02\xB1a\x1E\xFDV[a\x02]a\x08M6`\x04aUQV[a\x08gV[4\x80\x15a\x08^W`\0\x80\xFD[Pa\x02\xB1a\x1FQV[\x82`\0Z\x90P4\x15a\x08\xF7W`?`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1E\xE1\x16\xBF4`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x08\xDDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xF1W=`\0\x80>=`\0\xFD[PPPPP[\x83\x80\x15a\t\x19WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x15\x15[\x15a\tPW`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tZ\x83Qa\x1B\xC0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\t\xA7W`@Q\x7Fp\xC8\xBD\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x01\xD4\xC0\x83Q\x11\x15a\t\xE5W`@Q\x7FZ\xA3\xBA\xC9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\t\xEEa\x1F\xC1V[a\n\x0BWP3s\x11\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x11\x01[`\x004\x88\x88\x88\x88`@Q` \x01a\n&\x95\x94\x93\x92\x91\x90aU\xD0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB3\x815h\xD9\x99\x1F\xC9Q\x96\x1F\xCBLxH\x93WB@\xA2\x89%`M\t\xFCW|U\xBB|2\x84`@Qa\n\x96\x91\x90aSdV[`@Q\x80\x91\x03\x90\xA4PPa\n\xAA\x82\x82a\x1F\xFFV[PPPPPPPV[a\n\xBBa\"\xD6V[a\n\xC3a#\x15V[`>Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x03a\x0B\x17W`@Q\x7Fx]\xF9\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`?\x80T`>\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x87\x82\x16\x90\x81\x17t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90\x95U`@\x80Q\x94\x82\x16\x80\x86R` \x86\x01\x96\x90\x96R\x91\x16\x90\x83\x01\x81\x90R``\x83\x01\x91\x90\x91R\x90\x7F\x9EShG\x1AX\xD8\x19\x87\xE5\xDC}ct\xDD^\xD5\xE7V\xCC\x95\xA7\x9F\xF7&\x904#\xBC\xE0\x06\r\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA1PPPPV[`7T`@\x80Q\x7F5\xE8\n\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c5\xE8\n\xB3\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aV5V[\x90P\x90V[`>T`@\x80Q\x7F<\x9F9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c<\x9F9|\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aVdV[`\0\x80a\rQ\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03T\x90V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\rtW\x91\x90PV[`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x81RPQ`\x02a\r\xB7\x91\x90aV\xB0V[`@\x80Q0` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x7FOVM_L1CrossDomainMessenger\0\0\0\0\0\0\x91\x90\x91\x17\x90a\x0E\x12\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 T\x90V[\x14a\x0EIW`@Q\x7FT\xE43\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R`\0\x90a\x0Ek\x90``\x01a\r\xF8V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a\x0E\xFFW\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF8\x91\x90aV5V[\x92PPP\x90V[`@Q\x7F3!D\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F9a\"\xD6V[`2Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\xDE\xAD\x14a\x0F\x8CW`@Q\x7F\xDF\xEA\xAE\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x99\x82`@\x01Qa#kV[\x15a\x0F\xD0W`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0F\xDB\x83a#\xAEV[\x90Pa\x0F\xE7\x81\x83a\x15\xD5V[`\0\x81\x81R`3` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U``\x83\x01Q\x15a\x10\xB8W`?T``\x84\x01Q`@Q\x7F\x8DD[\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91c\x8DD[\xD0\x91a\x10\x85\x91`\x04\x01\x90\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x9FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xB3W=`\0\x80>=`\0\xFD[PPPP[\x82` \x01Q`2`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0a\x11\x1B\x84`@\x01Q\x85`\x80\x01Q\x86``\x01Q\x87`\xA0\x01Qa#\xFBV[`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U`@Q\x90\x91P\x82\x90\x7F\xDB\\vR\x85z\xA1c\xDA\xAD\xD6p\xE1\x16b\x8F\xB4.\x86\x9D\x8A\xC4%\x1E\xF8\x97\x1D\x9EW'\xDF\x1B\x90a\x11\x80\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x80\x15\x80\x15a\x11\x9AWP`\0\x84``\x01Q\x11[\x15a\x12'W`?`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1E\xE1\x16\xBF\x85``\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12!W=`\0\x80>=`\0\xFD[PPPPP[\x80\x15\x80\x15a\x125WP2`\x01\x14[\x15a\x12lW`@Q\x7F\xABX\x106\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`7T`@\x80Q\x7FE*\x93 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91cE*\x93 \x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[`>T`@Q\x7FE\x88M2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x92\x16\x90cE\x88M2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13w\x91\x90aV\xEDV[\x92\x91PPV[a\x13\x85a\"\xD6V[`?Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x13\xDAW`@Q\x7F^t\xB5B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\xE4a\x1FQV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBB\x8A\xA1\xFC\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x1E\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14_\x91\x90aW\nV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R\x90\x93P\x91Pa\x14\x87\x90PV[a\n\xAA\x87\x83`\0\x84a\x14\x9E6\x8B\x90\x03\x8B\x01\x8BaWLV[a\x14\xA8\x89\x8BaW\xD6V[a$YV[`>T`@\x80Q\x7F@\x86\xD1\x83\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c@\x86\xD1\x83\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x15\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aXOV[`7T`@\x80Q\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\\\x97Z\xBB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x15\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aV\xEDV[`\0\x82\x81R`9` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x85R\x90\x83R\x81\x84 \x82Q\x80\x84\x01\x84R\x90T\x91\x82\x16\x80\x82Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x85\x01R\x86\x85R`3\x90\x93R\x92 T\x90\x91\x90`\xFF\x16\x15a\x16\x84W`@Q\x7Fs\n\x10t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x16\xCCW`@Q\x7F\xCC\xA6\xAF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17K\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17>\x91\x90aXOV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x17\x9BW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x17\xD6\x91\x90aXlV[\x11a\x18\rW`@Q\x7F\xD9\xBC\x01\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7FlODg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90clODg\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA1\x91\x90aV\xEDV[a\x12lW`@Q\x7F3*W\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x18\xE3\x813a\x0F1V[PV[a\x18\xEEa\"\xD6V[`?Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x19BW`@Q\x7F^t\xB5B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xAA\x87\x87\x87a\x19Q\x88aX\x83V[a\x19`6\x89\x90\x03\x89\x01\x89aWLV[a\x14\xA8\x87\x89aW\xD6V[`>T`@\x80Q\x7F\x95+'\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x95+'\x97\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x19\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90aY\x8DV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x1A>WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x1A\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x1B\x08a,\xB4V[`>\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`?\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x1B\xCD\x82`(aY\xA6V[a\x13w\x90aR\x08aY\xD6V[`<` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x1B\xF5W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x82\x90PV[a\x1C&a#\x15V[`?T`@\x80Q\x7F\x1E\xE1\x16\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90QG\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x1E\xE1\x16\xBF\x91\x84\x91`\x04\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x85\x88\x80;\x15\x80\x15a\x1C\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xA8W=`\0\x80>=`\0\xFD[PP`?T`@Q\x85\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x93P\x7F\xD8\x93\xF60\xC6\x86\x7F\xA46\x89\xDA\x9A\xE9I\xEB\xF0L\xAC$\xAA\xD3\xB4\\u\x9DD.\xD3\xC3.:7\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA2PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x1D@WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x1D\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x1E\x05a,\xB4V[`7\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`>\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`?\x80T\x85\x84\x16\x92\x16\x91\x90\x91\x17\x90U`2T\x16a\x1E\x9AW`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U[a\x1E\xA2a-5V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x0B\xEBV[`\0a\x1F\x07a\r&V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[`>T`@\x80Q\x7F\xF2\xB4\xE6\x17\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xF2\xB4\xE6\x17\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[`\x0023\x03a\x1F\xD0WP`\x01\x90V[3;`\x17\x03a\x1F\xF8W`@Q` \x81\x01`@R` `\0\x823<Q`\xE8\x1Cb\xEF\x01\0\x14\x90P\x90V[P`\0[\x90V[`\x01T`\0\x90a 5\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16CaXlV[\x90P`\0a Aa.HV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a b\x91\x90aZ1V[\x90P\x82\x15a!\x99W`\x01T`\0\x90a \x99\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aZ\x99V[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a \xB0\x91\x90a[\rV[`\x01Ta \xD0\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\rV[a \xDA\x91\x90aZ1V[`\x01T\x90\x91P`\0\x90a!+\x90a!\x04\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a[\xC9V[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a/\x0EV[\x90P`\x01\x86\x11\x15a!ZWa!Wa!\x04\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa!R\x91\x90aXlV[a/-V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a!\xCC\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aY\xD6V[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\"YW`@Q\x7Fw\xEB\xEFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\0\x90a\"\x85\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16aV\xB0V[\x90P`\0a\"\x97Hc;\x9A\xCA\0a/\x82V[a\"\xA1\x90\x83a\\=V[\x90P`\0Za\"\xB0\x90\x88aXlV[\x90P\x80\x82\x11\x15a\"\xCCWa\"\xCCa\"\xC7\x82\x84aXlV[a/\x99V[PPPPPPPPV[a\"\xDEa\x15AV[\x15a\x18\xD7W`@Q\x7F\xB9\xC3\xC2\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a#\x1Ea\x1E\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xD7W`@Q\x7F\x7F\x12\xC6K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14\x80a\x13wWPP`?Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a#\xDE\x97\x90\x96\x95\x91\x01a\\QV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a$\x0B\x86`\0a/\xC7V[\x90P\x80a$AWc\x08\xC3y\xA0`\0R` \x80Rx\x18SafeCall: Not enough gas`XR`d`\x1C\xFD[`\0\x80\x85Q` \x87\x01\x88\x8BZ\xF1\x97\x96PPPPPPPV[a$f\x86`@\x01Qa#kV[\x15a$\x9DW`@Q\x7F\xC5\xDE\xFB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7FIk\x9C\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x90cIk\x9C\x16\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%1\x91\x90aV\xEDV[a%gW`@Q\x7F\xF3\x95$\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@Q\x7F\x04\xE5\x0F\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\x04\xE5\x0F\xED\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xFB\x91\x90aV\xEDV[a&1W`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c \r.\xD2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xA2\x91\x90a\\\xD7V[`\x02\x81\x11\x15a&\xB3Wa&\xB3a\\\xA8V[\x03a&\xEAW`@Q\x7F\xE2\x99'\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'8\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCF\t\xE0\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x1AW=`\0\x80>=`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11a'zW`@Q\x7F\xB4\xCA\xA4\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`?Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a)\xC4Wa'\xA6\x83a/\xE5V[a(\x18\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xFC\x91\x90aY\x8DV[\x14a(OW`@Q\x7F+\x1A\x9Af\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`@\x01QQ\x84\x10a(\x8DW`@Q\x7F2\xDC(\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83`@\x01Q\x85\x81Q\x81\x10a(\xA5Wa(\xA5a\\\xF8V[` \x02` \x01\x01Q\x90P`7`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\xAE<\xD5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)@\x91\x90aY\x8DV[\x81Q\x14a)yW`@Q\x7F|\xC2\xF3\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\x82\x83a/\xFEV[\x81` \x01Q\x14a)\xBEW`@Q\x7FBaI\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa*RV[a)\xCD\x82a/\xFEV[a*\x1B\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xF4W=`\0\x80>=`\0\xFD[\x14a*RW`@Q\x7FBaI\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a*]\x87a#\xAEV[\x90P`\0\x81`\0`@Q` \x01a*~\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92Pa+)\x91\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x82\x82\x01\x82R`\x01\x83R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x90\x87\x01Q\x90\x91\x90\x86\x90a0=V[\x15\x15`\0\x03a+dW`@Q\x7F.W\xEF:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16` \x80\x85\x01\x91\x82R`\0\x88\x81R`9\x82R\x86\x81 3\x80\x83R\x90\x83R\x87\x82 \x96Q\x87T\x94Q\x90\x95\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x94\x86\x16\x94\x90\x94\x17\x92\x90\x92\x17\x90\x94U\x86\x81R`<\x84R\x84\x81 \x80T`\x01\x81\x01\x82U\x90\x82R\x84\x82 \x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x92\x17\x90\x91U\x8B\x84\x01Q\x92\x8C\x01Q\x93Q\x92\x82\x16\x93\x90\x91\x16\x91\x85\x91\x7Fg\xA6 \x8C\xFC\xC0\x80\x1DP\xF6\xCB\xE7ds?O\xDD\xF6j\xC0\xB0DB\x06\x1A\x8A\x8C\x0C\xB6\xB6?b\x91\xA4`@Q3\x90\x83\x90\x7Fy\x8F\x9F\x13i_\x8F\x04Z\xA5\xF8\x0E\xD8\xEF\xEB\xB6\x95\xF3\xC7\xFEe\xDA8\x19i\xF2\xF2\x8B\xF3\xC6\x0B\x97\x90`\0\x90\xA3PPPPPPPPV[3a,\xBDa\r&V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a,\xFEWP3a,\xE5a\x1E\xFDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x18\xD7W`@Q\x7F\xC4\x05\n&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16a-\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\x01Tx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\x18\xD7W`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90\x92\x01\x81\x90Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x17`\x01UV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R`7T`@\x80Q\x7F\xCCs\x1B\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q`\0\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xCCs\x1B\x02\x91`\x04\x80\x83\x01\x92`\xC0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a.\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13w\x91\x90a]=V[`\0a/#a/\x1D\x85\x85a0aV[\x83a0qV[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a/na/E\x85\x83aZ1V[a/W\x90g\r\xE0\xB6\xB3\xA7d\0\0aZ\x99V[a/i\x85g\r\xE0\xB6\xB3\xA7d\0\0a[\rV[a0\x80V[a/x\x90\x86a[\rV[a/#\x91\x90aZ1V[`\0\x81\x83\x10\x15a/\x92W\x81a/&V[P\x90\x91\x90PV[`\0\x80Z\x90P[\x82Za/\xAC\x90\x83aXlV[\x10\x15a/\xC2Wa/\xBB\x82a]\xF9V[\x91Pa/\xA0V[PPPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[`\0a/\xF0\x82a0\xB1V[\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a#\xDE\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`\0\x80a0I\x86a2gV[\x90Pa0W\x81\x86\x86\x86a2\x99V[\x96\x95PPPPPPV[`\0\x81\x83\x12\x15a/\x92W\x81a/&V[`\0\x81\x83\x12a/\x92W\x81a/&V[`\0a/&g\r\xE0\xB6\xB3\xA7d\0\0\x83a0\x98\x86a2\xC9V[a0\xA2\x91\x90a[\rV[a0\xAC\x91\x90aZ1V[a5\rV[\x80Q``\x90\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a10W`@Q\x7F\xC0kR8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`@\x01QQ`\0\x03a1oW`@Q\x7F\x91\x03\xE7\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`\0\x01Q\x83` \x01Q`\xC0\x1B`@Q` \x01a1\xDE\x92\x91\x90\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x90\x92\x16\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01\x82\x01R`\t\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x83`@\x01QQ\x81\x10\x15a2`W`\0\x84`@\x01Q\x82\x81Q\x81\x10a2\x17Wa2\x17a\\\xF8V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x80Q\x81\x83\x01Q`@Q\x92\x94Pa2;\x93\x87\x93\x01a^1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PP\x80\x80a2X\x90a]\xF9V[\x91PPa1\xF2V[P\x92\x91PPV[``\x81\x80Q\x90` \x01 `@Q` \x01a2\x83\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a2\xC0\x84a2\xAA\x87\x86\x86a7LV[\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[\x95\x94PPPPPV[`\0\x80\x82\x13a34W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[`\0``a3A\x84aA\xCAV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a5>WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a5\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[```\0\x84Q\x11a7\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FMerkleTrie: empty key\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[`\0a7\xC4\x84aB\xA0V[\x90P`\0a7\xD1\x86aC\x8CV[\x90P`\0\x84`@Q` \x01a7\xE8\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80[\x84Q\x81\x10\x15aAAW`\0\x85\x82\x81Q\x81\x10a8\x1AWa8\x1Aa\\\xF8V[` \x02` \x01\x01Q\x90P\x84Q\x83\x11\x15a8\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FMerkleTrie: key index exceeds to`D\x82\x01R\x7Ftal key length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[\x82`\0\x03a9nW\x80Q\x80Q` \x91\x82\x01 `@Qa9\x03\x92a8\xDD\x92\x91\x01\x90\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x80Q` \x91\x82\x01 \x82Q\x92\x90\x91\x01\x91\x90\x91 \x14\x90V[a9iW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FMerkleTrie: invalid root hash\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[a:\xC5V[\x80QQ` \x11a:$W\x80Q\x80Q` \x91\x82\x01 `@Qa9\x98\x92a8\xDD\x92\x91\x01\x90\x81R` \x01\x90V[a9iW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FMerkleTrie: invalid large intern`D\x82\x01R\x7Fal hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[\x80Q\x84Q` \x80\x87\x01\x91\x90\x91 \x82Q\x91\x90\x92\x01 \x14a:\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FMerkleTrie: invalid internal nod`D\x82\x01R\x7Fe hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[a:\xD1`\x10`\x01a^XV[\x81` \x01QQ\x03a<\xADW\x84Q\x83\x03a<EWa;\x0B\x81` \x01Q`\x10\x81Q\x81\x10a:\xFEWa:\xFEa\\\xF8V[` \x02` \x01\x01QaC\xEFV[\x96P`\0\x87Q\x11a;\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (branch)\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\x01\x86Qa;\xAC\x91\x90aXlV[\x82\x14a<:W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (branch)\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[PPPPPPa/&V[`\0\x85\x84\x81Q\x81\x10a<YWa<Ya\\\xF8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\0\x82` \x01Q\x82`\xFF\x16\x81Q\x81\x10a<\x84Wa<\x84a\\\xF8V[` \x02` \x01\x01Q\x90Pa<\x97\x81aD\xA3V[\x95Pa<\xA4`\x01\x86a^XV[\x94PPPaA.V[`\x02\x81` \x01QQ\x03a@\xA6W`\0a<\xC5\x82aD\xC8V[\x90P`\0\x81`\0\x81Q\x81\x10a<\xDCWa<\xDCa\\\xF8V[\x01` \x01Q`\xF8\x1C\x90P`\0a<\xF3`\x02\x83a^pV[a<\xFE\x90`\x02a^\x92V[\x90P`\0a=\x0F\x84\x83`\xFF\x16aD\xECV[\x90P`\0a=\x1D\x8A\x89aD\xECV[\x90P`\0a=+\x83\x83aE\"V[\x90P\x80\x83Q\x14a=\xBDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMerkleTrie: path remainder must `D\x82\x01R\x7Fshare all nibbles with key\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\xFF\x85\x16`\x02\x14\x80a=\xD2WP`\xFF\x85\x16`\x03\x14[\x15a?\xC1W\x80\x82Q\x14a>gW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FMerkleTrie: key remainder must b`D\x82\x01R\x7Fe identical to path remainder\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[a>\x81\x87` \x01Q`\x01\x81Q\x81\x10a:\xFEWa:\xFEa\\\xF8V[\x9CP`\0\x8DQ\x11a?\x14W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FMerkleTrie: value length must be`D\x82\x01R\x7F greater than zero (leaf)\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\x01\x8CQa?\"\x91\x90aXlV[\x88\x14a?\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FMerkleTrie: value node must be l`D\x82\x01R\x7Fast node in proof (leaf)\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[PPPPPPPPPPPPa/&V[`\xFF\x85\x16\x15\x80a?\xD4WP`\xFF\x85\x16`\x01\x14[\x15a@\x13Wa@\0\x87` \x01Q`\x01\x81Q\x81\x10a?\xF3Wa?\xF3a\\\xF8V[` \x02` \x01\x01QaD\xA3V[\x99Pa@\x0C\x81\x8Aa^XV[\x98Pa@\x9BV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FMerkleTrie: received a node with`D\x82\x01R\x7F an unknown prefix\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[PPPPPPaA.V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMerkleTrie: received an unparsea`D\x82\x01R\x7Fble node\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[P\x80aA9\x81a]\xF9V[\x91PPa7\xFDV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FMerkleTrie: ran out of proof ele`D\x82\x01R\x7Fments\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xC6V[`\0\x80\x82\x11aB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[\x80Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xBEWaB\xBEaO\x80V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aC\x03W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81aB\xDCW\x90P[P\x91P`\0[\x81\x81\x10\x15aC\x85W`@Q\x80`@\x01`@R\x80\x85\x83\x81Q\x81\x10aC.WaC.a\\\xF8V[` \x02` \x01\x01Q\x81R` \x01aC]\x86\x84\x81Q\x81\x10aCPWaCPa\\\xF8V[` \x02` \x01\x01QaE\xCFV[\x81RP\x83\x82\x81Q\x81\x10aCrWaCra\\\xF8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01aC\tV[PP\x91\x90PV[``\x80`@Q\x90P\x82Q\x80`\x01\x1B`?\x81\x01`\x1F\x19\x16\x83\x01`@R\x80\x83RP` \x84\x01` \x83\x01`\0[\x83\x81\x10\x15aC\xE4W\x80`\x01\x1B\x82\x01\x81\x84\x01Q`\0\x1A\x80`\x04\x1C\x82S`\x0F\x81\x16`\x01\x83\x01SPP`\x01\x01aC\xB6V[P\x92\x95\x94PPPPPV[```\0\x80`\0aC\xFF\x85aE\xE2V[\x91\x94P\x92P\x90P`\0\x81`\x01\x81\x11\x15aD\x1AWaD\x1Aa\\\xA8V[\x14aDQW`@Q\x7F\x1F\xF9\xB2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aD[\x82\x84a^XV[\x85Q\x14aD\x94W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2\xC0\x85` \x01Q\x84\x84aJ\x80V[``` \x82`\0\x01Q\x10aD\xBFWaD\xBA\x82aC\xEFV[a\x13wV[a\x13w\x82aK\x14V[``a\x13waD\xE7\x83` \x01Q`\0\x81Q\x81\x10a:\xFEWa:\xFEa\\\xF8V[aC\x8CV[``\x82Q\x82\x10aE\x0BWP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x13wV[a/&\x83\x83\x84\x86QaE\x1D\x91\x90aXlV[aK*V[`\0\x80\x82Q\x84Q\x10aE5W\x82QaE8V[\x83Q[\x90P[\x80\x82\x10\x80\x15aE\xBFWP\x82\x82\x81Q\x81\x10aEWWaEWa\\\xF8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x84\x83\x81Q\x81\x10aE\x96WaE\x96a\\\xF8V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[\x15a2`W\x81`\x01\x01\x91PaE;V[``a\x13waE\xDD\x83aM\x02V[aMoV[`\0\x80`\0\x83`\0\x01Q`\0\x03aF%W`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x84\x01Q\x80Q`\0\x1A`\x7F\x81\x11aFJW`\0`\x01`\0\x94P\x94P\x94PPPaJyV[`\xB7\x81\x11aG`W`\0aF_`\x80\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aF\x9EW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x82\x14\x80\x15aG\x16WP\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x10[\x15aGMW`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x95P\x93P`\0\x92PaJy\x91PPV[`\xBF\x81\x11aH\xBEW`\0aGu`\xB7\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aG\xB4W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aH\x16W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aH^W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aHh\x81\x84a^XV[\x89Q\x11aH\xA1W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aH\xAC\x83`\x01a^XV[\x97P\x95P`\0\x94PaJy\x93PPPPV[`\xF7\x81\x11aI#W`\0aH\xD3`\xC0\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aI\x12W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x95P\x93P\x84\x92PaJy\x91PPV[`\0aI0`\xF7\x83aXlV[\x90P\x80\x87`\0\x01Q\x11aIoW`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x81\x90\x03aI\xD1W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x84\x01Q`\x08\x83\x02a\x01\0\x03\x1C`7\x81\x11aJ\x19W`@Q\x7F\xBA\xBB\x01\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aJ#\x81\x84a^XV[\x89Q\x11aJ\\W`@Q\x7Ff\xC9D\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aJg\x83`\x01a^XV[\x97P\x95P`\x01\x94PaJy\x93PPPPV[\x91\x93\x90\x92PV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\x9BWaJ\x9BaO\x80V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aJ\xC5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x15a/&W`\0aJ\xDA\x84\x86a^XV[\x90P` \x82\x01`\0[\x84\x81\x10\x15aJ\xFBW\x82\x81\x01Q\x82\x82\x01R` \x01aJ\xE3V[\x84\x81\x11\x15aK\nW`\0\x85\x83\x01R[PPP\x93\x92PPPV[``a\x13w\x82` \x01Q`\0\x84`\0\x01QaJ\x80V[``\x81\x82`\x1F\x01\x10\x15aK\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[\x82\x82\x84\x01\x10\x15aL\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[\x81\x83\x01\x84Q\x10\x15aLrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Fslice_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1A\xC6V[``\x82\x15\x80\x15aL\x91W`@Q\x91P`\0\x82R` \x82\x01`@RaL\xF9V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15aL\xCAW\x80Q\x83R` \x92\x83\x01\x92\x01aL\xB2V[PP\x85\x84R`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16`@RP[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q`\0\x03aMQW`@Q\x7FZ\xB4X\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R\x81Q\x81R` \x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[```\0\x80`\0aM\x7F\x85aE\xE2V[\x91\x94P\x92P\x90P`\x01\x81`\x01\x81\x11\x15aM\x9AWaM\x9Aa\\\xA8V[\x14aM\xD1W`@Q\x7FK\x9Cj\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84QaM\xDD\x83\x85a^XV[\x14aN\x14W`@Q\x7F\\U7\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81aN+W\x90PP\x93P`\0\x83[\x86Q\x81\x10\x15aO\x19W`\0\x80aN\x9E`@Q\x80`@\x01`@R\x80\x85\x8C`\0\x01QaN\x82\x91\x90aXlV[\x81R` \x01\x85\x8C` \x01QaN\x97\x91\x90a^XV[\x90RaE\xE2V[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83aN\xBA\x91\x90a^XV[\x81R` \x01\x84\x8B` \x01QaN\xCF\x91\x90a^XV[\x81RP\x88\x85\x81Q\x81\x10aN\xE4WaN\xE4a\\\xF8V[` \x90\x81\x02\x91\x90\x91\x01\x01RaN\xFA`\x01\x85a^XV[\x93PaO\x06\x81\x83a^XV[aO\x10\x90\x84a^XV[\x92PPPaNXV[P\x84RP\x91\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xE3W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aOZW`\0\x80\xFD[\x825aOe\x81aO%V[\x91P` \x83\x015aOu\x81aO%V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aO\xD2WaO\xD2aO\x80V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aO\xD2WaO\xD2aO\x80V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aPBWaPBaO\x80V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aP[W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aPuWaPuaO\x80V[aP\xA6` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01aO\xFBV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aP\xBBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15aP\xEAW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aQ\x0EWaQ\x0EaO\x80V[\x81`@R\x82\x93P\x845\x83R` \x85\x015\x91PaQ)\x82aO%V[\x81` \x84\x01R`@\x85\x015\x91PaQ?\x82aO%V[\x81`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15aQmW`\0\x80\xFD[PaQz\x85\x82\x86\x01aPJV[`\xA0\x83\x01RPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x9AW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xB1W`\0\x80\xFD[aQ\xBD\x85\x82\x86\x01aP\xD8V[\x92PP` \x83\x015aOu\x81aO%V[`\0` \x82\x84\x03\x12\x15aQ\xE0W`\0\x80\xFD[\x815a/&\x81aO%V[`\0`\x80\x82\x84\x03\x12\x15aQ\xFDW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aR\x15W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR-W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aRHW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\xE0\x86\x88\x03\x12\x15aRgW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aR\x7FW`\0\x80\xFD[aR\x8B\x89\x83\x8A\x01aP\xD8V[\x96P` \x88\x015\x95PaR\xA1\x89`@\x8A\x01aQ\xEBV[\x94P`\xC0\x88\x015\x91P\x80\x82\x11\x15aR\xB7W`\0\x80\xFD[PaR\xC4\x88\x82\x89\x01aR\x03V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aR\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15aS\tW\x81\x81\x01Q\x83\x82\x01R` \x01aR\xF1V[\x83\x81\x11\x15a\x12lWPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaS2\x81` \x86\x01` \x86\x01aR\xEEV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a/&` \x83\x01\x84aS\x1AV[`\0\x80`@\x83\x85\x03\x12\x15aS\x8AW`\0\x80\xFD[\x825\x91P` \x83\x015aOu\x81aO%V[`\0` \x82\x84\x03\x12\x15aS\xAEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xC5W`\0\x80\xFD[aS\xD1\x84\x82\x85\x01aP\xD8V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0a\x01 \x88\x8A\x03\x12\x15aS\xF5W`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aT\rW`\0\x80\xFD[aT\x19\x8B\x83\x8C\x01aP\xD8V[\x98P` \x8A\x015\x91PaT+\x82aO%V[\x90\x96P`@\x89\x015\x95P``\x89\x015\x90\x80\x82\x11\x15aTHW`\0\x80\xFD[\x90\x89\x01\x90``\x82\x8C\x03\x12\x15aT\\W`\0\x80\xFD[\x81\x95PaTl\x8B`\x80\x8C\x01aQ\xEBV[\x94Pa\x01\0\x8A\x015\x91P\x80\x82\x11\x15aT\x83W`\0\x80\xFD[PaT\x90\x8A\x82\x8B\x01aR\x03V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xE3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aT\xCBW`\0\x80\xFD[\x815a/&\x81aT\xA3V[`\0\x80`@\x83\x85\x03\x12\x15aT\xE9W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15aU\rW`\0\x80\xFD[\x835aU\x18\x81aO%V[\x92P` \x84\x015aU(\x81aO%V[\x91P`@\x84\x015aU8\x81aO%V[\x80\x91PP\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x18\xE3W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aUiW`\0\x80\xFD[\x855aUt\x81aO%V[\x94P` \x86\x015\x93P`@\x86\x015aU\x8B\x81aT\xA3V[\x92P``\x86\x015aU\x9B\x81aUCV[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xB7W`\0\x80\xFD[aU\xC3\x88\x82\x89\x01aPJV[\x91PP\x92\x95P\x92\x95\x90\x93PV[\x85\x81R\x84` \x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xC0\x1B\x16`@\x82\x01R\x82\x15\x15`\xF8\x1B`H\x82\x01R`\0\x82QaV$\x81`I\x85\x01` \x87\x01aR\xEEV[\x91\x90\x91\x01`I\x01\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aVGW`\0\x80\xFD[\x81Qa/&\x81aO%V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xE3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aVvW`\0\x80\xFD[\x81Qa/&\x81aVRV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15aV\xE8WaV\xE8aV\x81V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aV\xFFW`\0\x80\xFD[\x81Qa/&\x81aUCV[`\0\x80`\0``\x84\x86\x03\x12\x15aW\x1FW`\0\x80\xFD[\x83QaW*\x81aVRV[` \x85\x01Q\x90\x93PaW;\x81aT\xA3V[`@\x85\x01Q\x90\x92PaU8\x81aO%V[`\0`\x80\x82\x84\x03\x12\x15aW^W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aW\x81WaW\x81aO\x80V[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aW\xCCWaW\xCCaO\x80V[P`\x05\x1B` \x01\x90V[`\0aW\xE9aW\xE4\x84aW\xB2V[aO\xFBV[\x80\x84\x82R` \x80\x83\x01\x92P\x85`\x05\x1B\x85\x016\x81\x11\x15aX\x07W`\0\x80\xFD[\x85[\x81\x81\x10\x15aXCW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX)W`\0\x80\x81\xFD[aX56\x82\x8A\x01aPJV[\x86RP\x93\x82\x01\x93\x82\x01aX\tV[P\x91\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aXaW`\0\x80\xFD[\x81Qa/&\x81aT\xA3V[`\0\x82\x82\x10\x15aX~WaX~aV\x81V[P\x03\x90V[`\0``\x826\x03\x12\x15aX\x95W`\0\x80\xFD[aX\x9DaO\xAFV[\x825\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14aX\xCDW`\0\x80\xFD[\x81R` \x83\x81\x015aX\xDE\x81aT\xA3V[\x82\x82\x01R`@\x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX\xFDW`\0\x80\xFD[\x85\x016`\x1F\x82\x01\x12aY\x0EW`\0\x80\xFD[\x805aY\x1CaW\xE4\x82aW\xB2V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x84\x01\x90\x84\x81\x01\x906\x83\x11\x15aY;W`\0\x80\xFD[\x92\x85\x01\x92[\x82\x84\x10\x15aY{W\x84\x846\x03\x12\x15aYXW`\0\x80\x81\xFD[aY`aO\xD8V[\x845\x81R\x86\x85\x015\x87\x82\x01R\x82R\x92\x84\x01\x92\x90\x85\x01\x90aY@V[\x93\x86\x01\x93\x90\x93RP\x92\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aY\x9FW`\0\x80\xFD[PQ\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aY\xCDWaY\xCDaV\x81V[\x02\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aY\xF9WaY\xF9aV\x81V[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aZ@WaZ@aZ\x02V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aZ\x94WaZ\x94aV\x81V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15aZ\xD3WaZ\xD3aV\x81V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a[\x07Wa[\x07aV\x81V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a[NWa[NaV\x81V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a[\x89Wa[\x89aV\x81V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a[\xA5Wa[\xA5aV\x81V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a[\xBBWa[\xBBaV\x81V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\\\x03Wa\\\x03aV\x81V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\\7Wa\\7aV\x81V[PP\x01\x90V[`\0\x82a\\LWa\\LaZ\x02V[P\x04\x90V[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\\\x9C`\xC0\x83\x01\x84aS\x1AV[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\\\xE9W`\0\x80\xFD[\x81Q`\x03\x81\x10a/&W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x80Q`\xFF\x81\x16\x81\x14a]8W`\0\x80\xFD[\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a]OW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a]rWa]raO\x80V[`@R\x82Qa]\x80\x81aVRV[\x81Ra]\x8E` \x84\x01a]'V[` \x82\x01Ra]\x9F`@\x84\x01a]'V[`@\x82\x01R``\x83\x01Qa]\xB2\x81aVRV[``\x82\x01R`\x80\x83\x01Qa]\xC5\x81aVRV[`\x80\x82\x01R`\xA0\x83\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a]\xEDW`\0\x80\xFD[`\xA0\x82\x01R\x93\x92PPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a^*Wa^*aV\x81V[P`\x01\x01\x90V[`\0\x84Qa^C\x81\x84` \x89\x01aR\xEEV[\x91\x90\x91\x01\x92\x83RP` \x82\x01R`@\x01\x91\x90PV[`\0\x82\x19\x82\x11\x15a^kWa^kaV\x81V[P\x01\x90V[`\0`\xFF\x83\x16\x80a^\x83Wa^\x83aZ\x02V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a^\xACWa^\xACaV\x81V[\x90\x03\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x0F\0\n",
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
    /**Custom error with signature `Encoding_EmptySuperRoot()` and selector `0x9103e7cd`.
```solidity
error Encoding_EmptySuperRoot();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Encoding_EmptySuperRoot;
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
        impl ::core::convert::From<Encoding_EmptySuperRoot> for UnderlyingRustTuple<'_> {
            fn from(value: Encoding_EmptySuperRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Encoding_EmptySuperRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Encoding_EmptySuperRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Encoding_EmptySuperRoot()";
            const SELECTOR: [u8; 4] = [145u8, 3u8, 231u8, 205u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `Encoding_InvalidSuperRootVersion()` and selector `0xc06b5238`.
```solidity
error Encoding_InvalidSuperRootVersion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Encoding_InvalidSuperRootVersion;
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
        impl ::core::convert::From<Encoding_InvalidSuperRootVersion>
        for UnderlyingRustTuple<'_> {
            fn from(value: Encoding_InvalidSuperRootVersion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for Encoding_InvalidSuperRootVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Encoding_InvalidSuperRootVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Encoding_InvalidSuperRootVersion()";
            const SELECTOR: [u8; 4] = [192u8, 107u8, 82u8, 56u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidOutputRootChainId()` and selector `0x7cc2f31b`.
```solidity
error OptimismPortal_InvalidOutputRootChainId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidOutputRootChainId;
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
        impl ::core::convert::From<OptimismPortal_InvalidOutputRootChainId>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidOutputRootChainId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidOutputRootChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidOutputRootChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidOutputRootChainId()";
            const SELECTOR: [u8; 4] = [124u8, 194u8, 243u8, 27u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidOutputRootIndex()` and selector `0x32dc285c`.
```solidity
error OptimismPortal_InvalidOutputRootIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidOutputRootIndex;
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
        impl ::core::convert::From<OptimismPortal_InvalidOutputRootIndex>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidOutputRootIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidOutputRootIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidOutputRootIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidOutputRootIndex()";
            const SELECTOR: [u8; 4] = [50u8, 220u8, 40u8, 92u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_InvalidSuperRootProof()` and selector `0x2b1a9a66`.
```solidity
error OptimismPortal_InvalidSuperRootProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_InvalidSuperRootProof;
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
        impl ::core::convert::From<OptimismPortal_InvalidSuperRootProof>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_InvalidSuperRootProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_InvalidSuperRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_InvalidSuperRootProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_InvalidSuperRootProof()";
            const SELECTOR: [u8; 4] = [43u8, 26u8, 154u8, 102u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_MigratingToSameRegistry()` and selector `0x785df911`.
```solidity
error OptimismPortal_MigratingToSameRegistry();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_MigratingToSameRegistry;
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
        impl ::core::convert::From<OptimismPortal_MigratingToSameRegistry>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_MigratingToSameRegistry) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_MigratingToSameRegistry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_MigratingToSameRegistry {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_MigratingToSameRegistry()";
            const SELECTOR: [u8; 4] = [120u8, 93u8, 249u8, 17u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `OptimismPortal_WrongProofMethod()` and selector `0x5e74b542`.
```solidity
error OptimismPortal_WrongProofMethod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OptimismPortal_WrongProofMethod;
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
        impl ::core::convert::From<OptimismPortal_WrongProofMethod>
        for UnderlyingRustTuple<'_> {
            fn from(value: OptimismPortal_WrongProofMethod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OptimismPortal_WrongProofMethod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OptimismPortal_WrongProofMethod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OptimismPortal_WrongProofMethod()";
            const SELECTOR: [u8; 4] = [94u8, 116u8, 181u8, 66u8];
            #[inline]
            fn new<'a>(
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
    /**Event with signature `ETHMigrated(address,uint256)` and selector `0xd893f630c6867fa43689da9ae949ebf04cac24aad3b45c759d442ed3c32e3a37`.
```solidity
event ETHMigrated(address indexed lockbox, uint256 ethBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ETHMigrated {
        #[allow(missing_docs)]
        pub lockbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ethBalance: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ETHMigrated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ETHMigrated(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                216u8, 147u8, 246u8, 48u8, 198u8, 134u8, 127u8, 164u8, 54u8, 137u8,
                218u8, 154u8, 233u8, 73u8, 235u8, 240u8, 76u8, 172u8, 36u8, 170u8, 211u8,
                180u8, 92u8, 117u8, 157u8, 68u8, 46u8, 211u8, 195u8, 46u8, 58u8, 55u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    lockbox: topics.1,
                    ethBalance: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.ethBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.lockbox.clone())
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
                    &self.lockbox,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ETHMigrated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ETHMigrated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ETHMigrated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `PortalMigrated(address,address,address,address)` and selector `0x9e5368471a58d81987e5dc7d6374dd5ed5e756cc95a79ff726903423bce0060d`.
```solidity
event PortalMigrated(address oldLockbox, address newLockbox, address oldAnchorStateRegistry, address newAnchorStateRegistry);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PortalMigrated {
        #[allow(missing_docs)]
        pub oldLockbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newLockbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldAnchorStateRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newAnchorStateRegistry: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PortalMigrated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PortalMigrated(address,address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                158u8, 83u8, 104u8, 71u8, 26u8, 88u8, 216u8, 25u8, 135u8, 229u8, 220u8,
                125u8, 99u8, 116u8, 221u8, 94u8, 213u8, 231u8, 86u8, 204u8, 149u8, 167u8,
                159u8, 247u8, 38u8, 144u8, 52u8, 35u8, 188u8, 224u8, 6u8, 13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldLockbox: data.0,
                    newLockbox: data.1,
                    oldAnchorStateRegistry: data.2,
                    newAnchorStateRegistry: data.3,
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
                        &self.oldLockbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newLockbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldAnchorStateRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newAnchorStateRegistry,
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
        impl alloy_sol_types::private::IntoLogData for PortalMigrated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PortalMigrated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PortalMigrated) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `initialize(address,address,address)` and selector `0xc0c53b8b`.
```solidity
function initialize(address _systemConfig, address _anchorStateRegistry, address _ethLockbox) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _systemConfig: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _anchorStateRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _ethLockbox: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._systemConfig, value._anchorStateRegistry, value._ethLockbox)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _systemConfig: tuple.0,
                        _anchorStateRegistry: tuple.1,
                        _ethLockbox: tuple.2,
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
            const SIGNATURE: &'static str = "initialize(address,address,address)";
            const SELECTOR: [u8; 4] = [192u8, 197u8, 59u8, 139u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ethLockbox,
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
    /**Function with signature `migrateLiquidity()` and selector `0xbda204bb`.
```solidity
function migrateLiquidity() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateLiquidityCall;
    ///Container type for the return parameters of the [`migrateLiquidity()`](migrateLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateLiquidityReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<migrateLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateLiquidityCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateLiquidityCall {
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
            impl ::core::convert::From<migrateLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateLiquidityReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl migrateLiquidityReturn {
            fn _tokenize(
                &self,
            ) -> <migrateLiquidityCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateLiquidityCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateLiquidityReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrateLiquidity()";
            const SELECTOR: [u8; 4] = [189u8, 162u8, 4u8, 187u8];
            #[inline]
            fn new<'a>(
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
                migrateLiquidityReturn::_tokenize(ret)
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
    /**Function with signature `migrateToSuperRoots(address,address)` and selector `0x2152f2be`.
```solidity
function migrateToSuperRoots(address _newLockbox, address _newAnchorStateRegistry) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateToSuperRootsCall {
        #[allow(missing_docs)]
        pub _newLockbox: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _newAnchorStateRegistry: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`migrateToSuperRoots(address,address)`](migrateToSuperRootsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateToSuperRootsReturn {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<migrateToSuperRootsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateToSuperRootsCall) -> Self {
                    (value._newLockbox, value._newAnchorStateRegistry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateToSuperRootsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newLockbox: tuple.0,
                        _newAnchorStateRegistry: tuple.1,
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
            impl ::core::convert::From<migrateToSuperRootsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateToSuperRootsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateToSuperRootsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl migrateToSuperRootsReturn {
            fn _tokenize(
                &self,
            ) -> <migrateToSuperRootsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateToSuperRootsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateToSuperRootsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrateToSuperRoots(address,address)";
            const SELECTOR: [u8; 4] = [33u8, 82u8, 242u8, 190u8];
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
                        &self._newLockbox,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._newAnchorStateRegistry,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                migrateToSuperRootsReturn::_tokenize(ret)
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
    pub struct proveWithdrawalTransaction_0Call {
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
    ///Container type for the return parameters of the [`proveWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes),uint256,(bytes32,bytes32,bytes32,bytes32),bytes[])`](proveWithdrawalTransaction_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proveWithdrawalTransaction_0Return {}
    #[allow(
        non_camel_case_types,
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
            impl ::core::convert::From<proveWithdrawalTransaction_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: proveWithdrawalTransaction_0Call) -> Self {
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
            for proveWithdrawalTransaction_0Call {
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
            impl ::core::convert::From<proveWithdrawalTransaction_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: proveWithdrawalTransaction_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proveWithdrawalTransaction_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proveWithdrawalTransaction_0Return {
            fn _tokenize(
                &self,
            ) -> <proveWithdrawalTransaction_0Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proveWithdrawalTransaction_0Call {
            type Parameters<'a> = (
                Types::WithdrawalTransaction,
                alloy::sol_types::sol_data::Uint<256>,
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proveWithdrawalTransaction_0Return;
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
                proveWithdrawalTransaction_0Return::_tokenize(ret)
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
    #[derive()]
    /**Function with signature `proveWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes),address,uint256,(bytes1,uint64,(uint256,bytes32)[]),(bytes32,bytes32,bytes32,bytes32),bytes[])` and selector `0x8c90dd65`.
```solidity
function proveWithdrawalTransaction(Types.WithdrawalTransaction memory _tx, address _disputeGameProxy, uint256 _outputRootIndex, Types.SuperRootProof memory _superRootProof, Types.OutputRootProof memory _outputRootProof, bytes[] memory _withdrawalProof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proveWithdrawalTransaction_1Call {
        #[allow(missing_docs)]
        pub _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _disputeGameProxy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _outputRootIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _superRootProof: <Types::SuperRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _withdrawalProof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
    }
    ///Container type for the return parameters of the [`proveWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes),address,uint256,(bytes1,uint64,(uint256,bytes32)[]),(bytes32,bytes32,bytes32,bytes32),bytes[])`](proveWithdrawalTransaction_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proveWithdrawalTransaction_1Return {}
    #[allow(
        non_camel_case_types,
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
                alloy::sol_types::sol_data::Uint<256>,
                Types::SuperRootProof,
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <Types::SuperRootProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<proveWithdrawalTransaction_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: proveWithdrawalTransaction_1Call) -> Self {
                    (
                        value._tx,
                        value._disputeGameProxy,
                        value._outputRootIndex,
                        value._superRootProof,
                        value._outputRootProof,
                        value._withdrawalProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proveWithdrawalTransaction_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _tx: tuple.0,
                        _disputeGameProxy: tuple.1,
                        _outputRootIndex: tuple.2,
                        _superRootProof: tuple.3,
                        _outputRootProof: tuple.4,
                        _withdrawalProof: tuple.5,
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
            impl ::core::convert::From<proveWithdrawalTransaction_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: proveWithdrawalTransaction_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proveWithdrawalTransaction_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proveWithdrawalTransaction_1Return {
            fn _tokenize(
                &self,
            ) -> <proveWithdrawalTransaction_1Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proveWithdrawalTransaction_1Call {
            type Parameters<'a> = (
                Types::WithdrawalTransaction,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Types::SuperRootProof,
                Types::OutputRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proveWithdrawalTransaction_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proveWithdrawalTransaction((uint256,address,address,uint256,uint256,bytes),address,uint256,(bytes1,uint64,(uint256,bytes32)[]),(bytes32,bytes32,bytes32,bytes32),bytes[])";
            const SELECTOR: [u8; 4] = [140u8, 144u8, 221u8, 101u8];
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
                        &self._disputeGameProxy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._outputRootIndex),
                    <Types::SuperRootProof as alloy_sol_types::SolType>::tokenize(
                        &self._superRootProof,
                    ),
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
                proveWithdrawalTransaction_1Return::_tokenize(ret)
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
    /**Function with signature `superRootsActive()` and selector `0xd325d3bf`.
```solidity
function superRootsActive() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct superRootsActiveCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`superRootsActive()`](superRootsActiveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct superRootsActiveReturn {
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
            impl ::core::convert::From<superRootsActiveCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: superRootsActiveCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for superRootsActiveCall {
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
            impl ::core::convert::From<superRootsActiveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: superRootsActiveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for superRootsActiveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for superRootsActiveCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "superRootsActive()";
            const SELECTOR: [u8; 4] = [211u8, 37u8, 211u8, 191u8];
            #[inline]
            fn new<'a>(
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
                        let r: superRootsActiveReturn = r.into();
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
                        let r: superRootsActiveReturn = r.into();
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
    /**Function with signature `upgrade(address,address)` and selector `0x99a88ec4`.
```solidity
function upgrade(address _anchorStateRegistry, address _ethLockbox) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeCall {
        #[allow(missing_docs)]
        pub _anchorStateRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _ethLockbox: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`upgrade(address,address)`](upgradeCall) function.
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
            impl ::core::convert::From<upgradeCall> for UnderlyingRustTuple<'_> {
                fn from(value: upgradeCall) -> Self {
                    (value._anchorStateRegistry, value._ethLockbox)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for upgradeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _anchorStateRegistry: tuple.0,
                        _ethLockbox: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgrade(address,address)";
            const SELECTOR: [u8; 4] = [153u8, 168u8, 142u8, 196u8];
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
                        &self._anchorStateRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ethLockbox,
                    ),
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
    ///Container for all the [`OptimismPortalInterop`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum OptimismPortalInteropCalls {
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
        migrateLiquidity(migrateLiquidityCall),
        #[allow(missing_docs)]
        migrateToSuperRoots(migrateToSuperRootsCall),
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
        proveWithdrawalTransaction_0(proveWithdrawalTransaction_0Call),
        #[allow(missing_docs)]
        proveWithdrawalTransaction_1(proveWithdrawalTransaction_1Call),
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
        superRootsActive(superRootsActiveCall),
        #[allow(missing_docs)]
        superchainConfig(superchainConfigCall),
        #[allow(missing_docs)]
        systemConfig(systemConfigCall),
        #[allow(missing_docs)]
        upgrade(upgradeCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    impl OptimismPortalInteropCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [33u8, 82u8, 242u8, 190u8],
            [51u8, 215u8, 226u8, 189u8],
            [53u8, 232u8, 10u8, 179u8],
            [56u8, 211u8, 140u8, 151u8],
            [60u8, 159u8, 57u8, 124u8],
            [62u8, 71u8, 21u8, 140u8],
            [67u8, 202u8, 28u8, 80u8],
            [69u8, 42u8, 147u8, 32u8],
            [69u8, 136u8, 77u8, 50u8],
            [72u8, 112u8, 73u8, 111u8],
            [79u8, 208u8, 67u8, 76u8],
            [81u8, 55u8, 71u8, 171u8],
            [84u8, 253u8, 77u8, 80u8],
            [92u8, 12u8, 186u8, 51u8],
            [92u8, 151u8, 90u8, 187u8],
            [113u8, 193u8, 86u8, 110u8],
            [139u8, 76u8, 64u8, 176u8],
            [140u8, 49u8, 82u8, 233u8],
            [140u8, 144u8, 221u8, 101u8],
            [149u8, 43u8, 39u8, 151u8],
            [153u8, 168u8, 142u8, 196u8],
            [155u8, 246u8, 45u8, 130u8],
            [161u8, 66u8, 56u8, 231u8],
            [163u8, 93u8, 153u8, 223u8],
            [163u8, 134u8, 15u8, 72u8],
            [182u8, 130u8, 196u8, 68u8],
            [187u8, 44u8, 114u8, 126u8],
            [189u8, 162u8, 4u8, 187u8],
            [191u8, 101u8, 58u8, 92u8],
            [192u8, 197u8, 59u8, 139u8],
            [207u8, 240u8, 171u8, 150u8],
            [211u8, 37u8, 211u8, 191u8],
            [218u8, 213u8, 68u8, 224u8],
            [233u8, 224u8, 92u8, 66u8],
            [242u8, 180u8, 230u8, 23u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(migrateToSuperRoots),
            ::core::stringify!(systemConfig),
            ::core::stringify!(superchainConfig),
            ::core::stringify!(initVersion),
            ::core::stringify!(respectedGameType),
            ::core::stringify!(proxyAdmin),
            ::core::stringify!(finalizeWithdrawalTransactionExternalProof),
            ::core::stringify!(guardian),
            ::core::stringify!(disputeGameBlacklist),
            ::core::stringify!(proveWithdrawalTransaction_0),
            ::core::stringify!(respectedGameTypeUpdatedAt),
            ::core::stringify!(numProofSubmitters),
            ::core::stringify!(version),
            ::core::stringify!(anchorStateRegistry),
            ::core::stringify!(paused),
            ::core::stringify!(checkWithdrawal),
            ::core::stringify!(donateETH),
            ::core::stringify!(finalizeWithdrawalTransaction),
            ::core::stringify!(proveWithdrawalTransaction_1),
            ::core::stringify!(disputeGameFinalityDelaySeconds),
            ::core::stringify!(upgrade),
            ::core::stringify!(l2Sender),
            ::core::stringify!(finalizedWithdrawals),
            ::core::stringify!(minimumGasLimit),
            ::core::stringify!(proofSubmitters),
            ::core::stringify!(ethLockbox),
            ::core::stringify!(provenWithdrawals),
            ::core::stringify!(migrateLiquidity),
            ::core::stringify!(proofMaturityDelaySeconds),
            ::core::stringify!(initialize),
            ::core::stringify!(params),
            ::core::stringify!(superRootsActive),
            ::core::stringify!(proxyAdminOwner),
            ::core::stringify!(depositTransaction),
            ::core::stringify!(disputeGameFactory),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <migrateToSuperRootsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <systemConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <superchainConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <respectedGameTypeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxyAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::SIGNATURE,
            <guardianCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeGameBlacklistCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proveWithdrawalTransaction_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <numProofSubmittersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <versionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <anchorStateRegistryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <checkWithdrawalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <donateETHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proveWithdrawalTransaction_1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <l2SenderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <minimumGasLimitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proofSubmittersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ethLockboxCall as alloy_sol_types::SolCall>::SIGNATURE,
            <provenWithdrawalsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <migrateLiquidityCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <paramsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <superRootsActiveCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OptimismPortalInteropCalls {
        const NAME: &'static str = "OptimismPortalInteropCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
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
                Self::migrateLiquidity(_) => {
                    <migrateLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrateToSuperRoots(_) => {
                    <migrateToSuperRootsCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::proveWithdrawalTransaction_0(_) => {
                    <proveWithdrawalTransaction_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proveWithdrawalTransaction_1(_) => {
                    <proveWithdrawalTransaction_1Call as alloy_sol_types::SolCall>::SELECTOR
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
                Self::superRootsActive(_) => {
                    <superRootsActiveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::superchainConfig(_) => {
                    <superchainConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::systemConfig(_) => {
                    <systemConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgrade(_) => <upgradeCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<OptimismPortalInteropCalls>] = &[
                {
                    fn migrateToSuperRoots(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <migrateToSuperRootsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::migrateToSuperRoots)
                    }
                    migrateToSuperRoots
                },
                {
                    fn systemConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <systemConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::systemConfig)
                    }
                    systemConfig
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn initVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <initVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::initVersion)
                    }
                    initVersion
                },
                {
                    fn respectedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <respectedGameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::respectedGameType)
                    }
                    respectedGameType
                },
                {
                    fn proxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proxyAdmin)
                    }
                    proxyAdmin
                },
                {
                    fn finalizeWithdrawalTransactionExternalProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::finalizeWithdrawalTransactionExternalProof,
                            )
                    }
                    finalizeWithdrawalTransactionExternalProof
                },
                {
                    fn guardian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <guardianCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortalInteropCalls::guardian)
                    }
                    guardian
                },
                {
                    fn disputeGameBlacklist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <disputeGameBlacklistCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::disputeGameBlacklist)
                    }
                    disputeGameBlacklist
                },
                {
                    fn proveWithdrawalTransaction_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proveWithdrawalTransaction_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::proveWithdrawalTransaction_0,
                            )
                    }
                    proveWithdrawalTransaction_0
                },
                {
                    fn respectedGameTypeUpdatedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::respectedGameTypeUpdatedAt)
                    }
                    respectedGameTypeUpdatedAt
                },
                {
                    fn numProofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <numProofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::numProofSubmitters)
                    }
                    numProofSubmitters
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortalInteropCalls::version)
                    }
                    version
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortalInteropCalls::paused)
                    }
                    paused
                },
                {
                    fn checkWithdrawal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <checkWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::checkWithdrawal)
                    }
                    checkWithdrawal
                },
                {
                    fn donateETH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <donateETHCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortalInteropCalls::donateETH)
                    }
                    donateETH
                },
                {
                    fn finalizeWithdrawalTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::finalizeWithdrawalTransaction,
                            )
                    }
                    finalizeWithdrawalTransaction
                },
                {
                    fn proveWithdrawalTransaction_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proveWithdrawalTransaction_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::proveWithdrawalTransaction_1,
                            )
                    }
                    proveWithdrawalTransaction_1
                },
                {
                    fn disputeGameFinalityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::disputeGameFinalityDelaySeconds,
                            )
                    }
                    disputeGameFinalityDelaySeconds
                },
                {
                    fn upgrade(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <upgradeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortalInteropCalls::upgrade)
                    }
                    upgrade
                },
                {
                    fn l2Sender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <l2SenderCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortalInteropCalls::l2Sender)
                    }
                    l2Sender
                },
                {
                    fn finalizedWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::finalizedWithdrawals)
                    }
                    finalizedWithdrawals
                },
                {
                    fn minimumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::minimumGasLimit)
                    }
                    minimumGasLimit
                },
                {
                    fn proofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proofSubmitters)
                    }
                    proofSubmitters
                },
                {
                    fn ethLockbox(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <ethLockboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::ethLockbox)
                    }
                    ethLockbox
                },
                {
                    fn provenWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <provenWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::provenWithdrawals)
                    }
                    provenWithdrawals
                },
                {
                    fn migrateLiquidity(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <migrateLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::migrateLiquidity)
                    }
                    migrateLiquidity
                },
                {
                    fn proofMaturityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proofMaturityDelaySeconds)
                    }
                    proofMaturityDelaySeconds
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::initialize)
                    }
                    initialize
                },
                {
                    fn params(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <paramsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OptimismPortalInteropCalls::params)
                    }
                    params
                },
                {
                    fn superRootsActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <superRootsActiveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::superRootsActive)
                    }
                    superRootsActive
                },
                {
                    fn proxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proxyAdminOwner)
                    }
                    proxyAdminOwner
                },
                {
                    fn depositTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <depositTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::depositTransaction)
                    }
                    depositTransaction
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::disputeGameFactory)
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
            ) -> alloy_sol_types::Result<OptimismPortalInteropCalls>] = &[
                {
                    fn migrateToSuperRoots(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <migrateToSuperRootsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::migrateToSuperRoots)
                    }
                    migrateToSuperRoots
                },
                {
                    fn systemConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <systemConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::systemConfig)
                    }
                    systemConfig
                },
                {
                    fn superchainConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <superchainConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::superchainConfig)
                    }
                    superchainConfig
                },
                {
                    fn initVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <initVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::initVersion)
                    }
                    initVersion
                },
                {
                    fn respectedGameType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <respectedGameTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::respectedGameType)
                    }
                    respectedGameType
                },
                {
                    fn proxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proxyAdmin)
                    }
                    proxyAdmin
                },
                {
                    fn finalizeWithdrawalTransactionExternalProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <finalizeWithdrawalTransactionExternalProofCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::finalizeWithdrawalTransactionExternalProof,
                            )
                    }
                    finalizeWithdrawalTransactionExternalProof
                },
                {
                    fn guardian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <guardianCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::guardian)
                    }
                    guardian
                },
                {
                    fn disputeGameBlacklist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <disputeGameBlacklistCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::disputeGameBlacklist)
                    }
                    disputeGameBlacklist
                },
                {
                    fn proveWithdrawalTransaction_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proveWithdrawalTransaction_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::proveWithdrawalTransaction_0,
                            )
                    }
                    proveWithdrawalTransaction_0
                },
                {
                    fn respectedGameTypeUpdatedAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <respectedGameTypeUpdatedAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::respectedGameTypeUpdatedAt)
                    }
                    respectedGameTypeUpdatedAt
                },
                {
                    fn numProofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <numProofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::numProofSubmitters)
                    }
                    numProofSubmitters
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::version)
                    }
                    version
                },
                {
                    fn anchorStateRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <anchorStateRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::anchorStateRegistry)
                    }
                    anchorStateRegistry
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::paused)
                    }
                    paused
                },
                {
                    fn checkWithdrawal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <checkWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::checkWithdrawal)
                    }
                    checkWithdrawal
                },
                {
                    fn donateETH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <donateETHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::donateETH)
                    }
                    donateETH
                },
                {
                    fn finalizeWithdrawalTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <finalizeWithdrawalTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::finalizeWithdrawalTransaction,
                            )
                    }
                    finalizeWithdrawalTransaction
                },
                {
                    fn proveWithdrawalTransaction_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proveWithdrawalTransaction_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::proveWithdrawalTransaction_1,
                            )
                    }
                    proveWithdrawalTransaction_1
                },
                {
                    fn disputeGameFinalityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <disputeGameFinalityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropCalls::disputeGameFinalityDelaySeconds,
                            )
                    }
                    disputeGameFinalityDelaySeconds
                },
                {
                    fn upgrade(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <upgradeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::upgrade)
                    }
                    upgrade
                },
                {
                    fn l2Sender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <l2SenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::l2Sender)
                    }
                    l2Sender
                },
                {
                    fn finalizedWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <finalizedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::finalizedWithdrawals)
                    }
                    finalizedWithdrawals
                },
                {
                    fn minimumGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <minimumGasLimitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::minimumGasLimit)
                    }
                    minimumGasLimit
                },
                {
                    fn proofSubmitters(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proofSubmittersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proofSubmitters)
                    }
                    proofSubmitters
                },
                {
                    fn ethLockbox(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <ethLockboxCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::ethLockbox)
                    }
                    ethLockbox
                },
                {
                    fn provenWithdrawals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <provenWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::provenWithdrawals)
                    }
                    provenWithdrawals
                },
                {
                    fn migrateLiquidity(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <migrateLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::migrateLiquidity)
                    }
                    migrateLiquidity
                },
                {
                    fn proofMaturityDelaySeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proofMaturityDelaySecondsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proofMaturityDelaySeconds)
                    }
                    proofMaturityDelaySeconds
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::initialize)
                    }
                    initialize
                },
                {
                    fn params(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <paramsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::params)
                    }
                    params
                },
                {
                    fn superRootsActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <superRootsActiveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::superRootsActive)
                    }
                    superRootsActive
                },
                {
                    fn proxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <proxyAdminOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::proxyAdminOwner)
                    }
                    proxyAdminOwner
                },
                {
                    fn depositTransaction(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <depositTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::depositTransaction)
                    }
                    depositTransaction
                },
                {
                    fn disputeGameFactory(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropCalls> {
                        <disputeGameFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropCalls::disputeGameFactory)
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
                Self::migrateLiquidity(inner) => {
                    <migrateLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::migrateToSuperRoots(inner) => {
                    <migrateToSuperRootsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::proveWithdrawalTransaction_0(inner) => {
                    <proveWithdrawalTransaction_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proveWithdrawalTransaction_1(inner) => {
                    <proveWithdrawalTransaction_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::superRootsActive(inner) => {
                    <superRootsActiveCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::upgrade(inner) => {
                    <upgradeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::migrateLiquidity(inner) => {
                    <migrateLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::migrateToSuperRoots(inner) => {
                    <migrateToSuperRootsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::proveWithdrawalTransaction_0(inner) => {
                    <proveWithdrawalTransaction_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proveWithdrawalTransaction_1(inner) => {
                    <proveWithdrawalTransaction_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::superRootsActive(inner) => {
                    <superRootsActiveCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::upgrade(inner) => {
                    <upgradeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`OptimismPortalInterop`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OptimismPortalInteropErrors {
        #[allow(missing_docs)]
        ContentLengthMismatch(ContentLengthMismatch),
        #[allow(missing_docs)]
        EmptyItem(EmptyItem),
        #[allow(missing_docs)]
        Encoding_EmptySuperRoot(Encoding_EmptySuperRoot),
        #[allow(missing_docs)]
        Encoding_InvalidSuperRootVersion(Encoding_InvalidSuperRootVersion),
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
        OptimismPortal_InvalidMerkleProof(OptimismPortal_InvalidMerkleProof),
        #[allow(missing_docs)]
        OptimismPortal_InvalidOutputRootChainId(OptimismPortal_InvalidOutputRootChainId),
        #[allow(missing_docs)]
        OptimismPortal_InvalidOutputRootIndex(OptimismPortal_InvalidOutputRootIndex),
        #[allow(missing_docs)]
        OptimismPortal_InvalidOutputRootProof(OptimismPortal_InvalidOutputRootProof),
        #[allow(missing_docs)]
        OptimismPortal_InvalidProofTimestamp(OptimismPortal_InvalidProofTimestamp),
        #[allow(missing_docs)]
        OptimismPortal_InvalidRootClaim(OptimismPortal_InvalidRootClaim),
        #[allow(missing_docs)]
        OptimismPortal_InvalidSuperRootProof(OptimismPortal_InvalidSuperRootProof),
        #[allow(missing_docs)]
        OptimismPortal_MigratingToSameRegistry(OptimismPortal_MigratingToSameRegistry),
        #[allow(missing_docs)]
        OptimismPortal_NoReentrancy(OptimismPortal_NoReentrancy),
        #[allow(missing_docs)]
        OptimismPortal_ProofNotOldEnough(OptimismPortal_ProofNotOldEnough),
        #[allow(missing_docs)]
        OptimismPortal_Unproven(OptimismPortal_Unproven),
        #[allow(missing_docs)]
        OptimismPortal_WrongProofMethod(OptimismPortal_WrongProofMethod),
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
    impl OptimismPortalInteropErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 92u8, 67u8, 20u8],
            [31u8, 249u8, 178u8, 228u8],
            [43u8, 26u8, 154u8, 102u8],
            [46u8, 87u8, 239u8, 58u8],
            [50u8, 220u8, 40u8, 92u8],
            [51u8, 33u8, 68u8, 219u8],
            [51u8, 42u8, 87u8, 248u8],
            [66u8, 97u8, 73u8, 175u8],
            [75u8, 156u8, 106u8, 190u8],
            [84u8, 228u8, 51u8, 205u8],
            [90u8, 163u8, 186u8, 201u8],
            [90u8, 180u8, 88u8, 251u8],
            [92u8, 85u8, 55u8, 184u8],
            [94u8, 116u8, 181u8, 66u8],
            [102u8, 201u8, 68u8, 133u8],
            [112u8, 200u8, 189u8, 189u8],
            [115u8, 10u8, 16u8, 116u8],
            [119u8, 235u8, 239u8, 77u8],
            [120u8, 93u8, 249u8, 17u8],
            [124u8, 194u8, 243u8, 27u8],
            [127u8, 18u8, 198u8, 75u8],
            [145u8, 3u8, 231u8, 205u8],
            [155u8, 1u8, 175u8, 237u8],
            [171u8, 88u8, 16u8, 54u8],
            [180u8, 202u8, 164u8, 229u8],
            [185u8, 195u8, 194u8, 239u8],
            [186u8, 187u8, 1u8, 221u8],
            [192u8, 107u8, 82u8, 56u8],
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
            ::core::stringify!(OptimismPortal_InvalidSuperRootProof),
            ::core::stringify!(OptimismPortal_InvalidMerkleProof),
            ::core::stringify!(OptimismPortal_InvalidOutputRootIndex),
            ::core::stringify!(ProxyAdminOwnedBase_ProxyAdminNotFound),
            ::core::stringify!(OptimismPortal_InvalidRootClaim),
            ::core::stringify!(OptimismPortal_InvalidOutputRootProof),
            ::core::stringify!(UnexpectedString),
            ::core::stringify!(ProxyAdminOwnedBase_NotResolvedDelegateProxy),
            ::core::stringify!(OptimismPortal_CalldataTooLarge),
            ::core::stringify!(EmptyItem),
            ::core::stringify!(InvalidDataRemainder),
            ::core::stringify!(OptimismPortal_WrongProofMethod),
            ::core::stringify!(ContentLengthMismatch),
            ::core::stringify!(OptimismPortal_GasLimitTooLow),
            ::core::stringify!(OptimismPortal_AlreadyFinalized),
            ::core::stringify!(OutOfGas),
            ::core::stringify!(OptimismPortal_MigratingToSameRegistry),
            ::core::stringify!(OptimismPortal_InvalidOutputRootChainId),
            ::core::stringify!(ProxyAdminOwnedBase_NotProxyAdminOwner),
            ::core::stringify!(Encoding_EmptySuperRoot),
            ::core::stringify!(ReinitializableBase_ZeroInitVersion),
            ::core::stringify!(OptimismPortal_GasEstimation),
            ::core::stringify!(OptimismPortal_InvalidProofTimestamp),
            ::core::stringify!(OptimismPortal_CallPaused),
            ::core::stringify!(InvalidHeader),
            ::core::stringify!(Encoding_InvalidSuperRootVersion),
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
            <OptimismPortal_InvalidSuperRootProof as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidOutputRootIndex as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::SIGNATURE,
            <UnexpectedString as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyItem as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidDataRemainder as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_WrongProofMethod as alloy_sol_types::SolError>::SIGNATURE,
            <ContentLengthMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::SIGNATURE,
            <OutOfGas as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_MigratingToSameRegistry as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidOutputRootChainId as alloy_sol_types::SolError>::SIGNATURE,
            <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::SIGNATURE,
            <Encoding_EmptySuperRoot as alloy_sol_types::SolError>::SIGNATURE,
            <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::SIGNATURE,
            <OptimismPortal_CallPaused as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHeader as alloy_sol_types::SolError>::SIGNATURE,
            <Encoding_InvalidSuperRootVersion as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OptimismPortalInteropErrors {
        const NAME: &'static str = "OptimismPortalInteropErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 36usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ContentLengthMismatch(_) => {
                    <ContentLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyItem(_) => <EmptyItem as alloy_sol_types::SolError>::SELECTOR,
                Self::Encoding_EmptySuperRoot(_) => {
                    <Encoding_EmptySuperRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Encoding_InvalidSuperRootVersion(_) => {
                    <Encoding_InvalidSuperRootVersion as alloy_sol_types::SolError>::SELECTOR
                }
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
                Self::OptimismPortal_InvalidMerkleProof(_) => {
                    <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidOutputRootChainId(_) => {
                    <OptimismPortal_InvalidOutputRootChainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_InvalidOutputRootIndex(_) => {
                    <OptimismPortal_InvalidOutputRootIndex as alloy_sol_types::SolError>::SELECTOR
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
                Self::OptimismPortal_InvalidSuperRootProof(_) => {
                    <OptimismPortal_InvalidSuperRootProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_MigratingToSameRegistry(_) => {
                    <OptimismPortal_MigratingToSameRegistry as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_NoReentrancy(_) => {
                    <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_ProofNotOldEnough(_) => {
                    <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_Unproven(_) => {
                    <OptimismPortal_Unproven as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OptimismPortal_WrongProofMethod(_) => {
                    <OptimismPortal_WrongProofMethod as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<OptimismPortalInteropErrors>] = &[
                {
                    fn ProxyAdminOwnedBase_NotSharedProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotSharedProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotSharedProxyAdminOwner
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn OptimismPortal_InvalidSuperRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidSuperRootProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidSuperRootProof,
                            )
                    }
                    OptimismPortal_InvalidSuperRootProof
                },
                {
                    fn OptimismPortal_InvalidMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidMerkleProof,
                            )
                    }
                    OptimismPortal_InvalidMerkleProof
                },
                {
                    fn OptimismPortal_InvalidOutputRootIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidOutputRootIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidOutputRootIndex,
                            )
                    }
                    OptimismPortal_InvalidOutputRootIndex
                },
                {
                    fn ProxyAdminOwnedBase_ProxyAdminNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_ProxyAdminNotFound,
                            )
                    }
                    ProxyAdminOwnedBase_ProxyAdminNotFound
                },
                {
                    fn OptimismPortal_InvalidRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidRootClaim,
                            )
                    }
                    OptimismPortal_InvalidRootClaim
                },
                {
                    fn OptimismPortal_InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidOutputRootProof,
                            )
                    }
                    OptimismPortal_InvalidOutputRootProof
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn ProxyAdminOwnedBase_NotResolvedDelegateProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotResolvedDelegateProxy,
                            )
                    }
                    ProxyAdminOwnedBase_NotResolvedDelegateProxy
                },
                {
                    fn OptimismPortal_CalldataTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_CalldataTooLarge,
                            )
                    }
                    OptimismPortal_CalldataTooLarge
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OptimismPortalInteropErrors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn OptimismPortal_WrongProofMethod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_WrongProofMethod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_WrongProofMethod,
                            )
                    }
                    OptimismPortal_WrongProofMethod
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn OptimismPortal_GasLimitTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_GasLimitTooLow,
                            )
                    }
                    OptimismPortal_GasLimitTooLow
                },
                {
                    fn OptimismPortal_AlreadyFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_AlreadyFinalized,
                            )
                    }
                    OptimismPortal_AlreadyFinalized
                },
                {
                    fn OutOfGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OutOfGas as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OptimismPortalInteropErrors::OutOfGas)
                    }
                    OutOfGas
                },
                {
                    fn OptimismPortal_MigratingToSameRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_MigratingToSameRegistry as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_MigratingToSameRegistry,
                            )
                    }
                    OptimismPortal_MigratingToSameRegistry
                },
                {
                    fn OptimismPortal_InvalidOutputRootChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidOutputRootChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidOutputRootChainId,
                            )
                    }
                    OptimismPortal_InvalidOutputRootChainId
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOwner
                },
                {
                    fn Encoding_EmptySuperRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <Encoding_EmptySuperRoot as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::Encoding_EmptySuperRoot)
                    }
                    Encoding_EmptySuperRoot
                },
                {
                    fn ReinitializableBase_ZeroInitVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ReinitializableBase_ZeroInitVersion,
                            )
                    }
                    ReinitializableBase_ZeroInitVersion
                },
                {
                    fn OptimismPortal_GasEstimation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_GasEstimation,
                            )
                    }
                    OptimismPortal_GasEstimation
                },
                {
                    fn OptimismPortal_InvalidProofTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidProofTimestamp,
                            )
                    }
                    OptimismPortal_InvalidProofTimestamp
                },
                {
                    fn OptimismPortal_CallPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_CallPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::OptimismPortal_CallPaused)
                    }
                    OptimismPortal_CallPaused
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn Encoding_InvalidSuperRootVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <Encoding_InvalidSuperRootVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::Encoding_InvalidSuperRootVersion,
                            )
                    }
                    Encoding_InvalidSuperRootVersion
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner
                },
                {
                    fn OptimismPortal_BadTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_BadTarget as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::OptimismPortal_BadTarget)
                    }
                    OptimismPortal_BadTarget
                },
                {
                    fn OptimismPortal_Unproven(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_Unproven as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::OptimismPortal_Unproven)
                    }
                    OptimismPortal_Unproven
                },
                {
                    fn OptimismPortal_ProofNotOldEnough(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_ProofNotOldEnough,
                            )
                    }
                    OptimismPortal_ProofNotOldEnough
                },
                {
                    fn OptimismPortal_NoReentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_NoReentrancy,
                            )
                    }
                    OptimismPortal_NoReentrancy
                },
                {
                    fn OptimismPortal_InvalidDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidDisputeGame,
                            )
                    }
                    OptimismPortal_InvalidDisputeGame
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotProxyAdmin,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdmin
                },
                {
                    fn OptimismPortal_ImproperDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_ImproperDisputeGame,
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
            ) -> alloy_sol_types::Result<OptimismPortalInteropErrors>] = &[
                {
                    fn ProxyAdminOwnedBase_NotSharedProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotSharedProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotSharedProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotSharedProxyAdminOwner
                },
                {
                    fn UnexpectedList(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <UnexpectedList as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::UnexpectedList)
                    }
                    UnexpectedList
                },
                {
                    fn OptimismPortal_InvalidSuperRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidSuperRootProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidSuperRootProof,
                            )
                    }
                    OptimismPortal_InvalidSuperRootProof
                },
                {
                    fn OptimismPortal_InvalidMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidMerkleProof,
                            )
                    }
                    OptimismPortal_InvalidMerkleProof
                },
                {
                    fn OptimismPortal_InvalidOutputRootIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidOutputRootIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidOutputRootIndex,
                            )
                    }
                    OptimismPortal_InvalidOutputRootIndex
                },
                {
                    fn ProxyAdminOwnedBase_ProxyAdminNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_ProxyAdminNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_ProxyAdminNotFound,
                            )
                    }
                    ProxyAdminOwnedBase_ProxyAdminNotFound
                },
                {
                    fn OptimismPortal_InvalidRootClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidRootClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidRootClaim,
                            )
                    }
                    OptimismPortal_InvalidRootClaim
                },
                {
                    fn OptimismPortal_InvalidOutputRootProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidOutputRootProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidOutputRootProof,
                            )
                    }
                    OptimismPortal_InvalidOutputRootProof
                },
                {
                    fn UnexpectedString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <UnexpectedString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::UnexpectedString)
                    }
                    UnexpectedString
                },
                {
                    fn ProxyAdminOwnedBase_NotResolvedDelegateProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotResolvedDelegateProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotResolvedDelegateProxy,
                            )
                    }
                    ProxyAdminOwnedBase_NotResolvedDelegateProxy
                },
                {
                    fn OptimismPortal_CalldataTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_CalldataTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_CalldataTooLarge,
                            )
                    }
                    OptimismPortal_CalldataTooLarge
                },
                {
                    fn EmptyItem(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <EmptyItem as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::EmptyItem)
                    }
                    EmptyItem
                },
                {
                    fn InvalidDataRemainder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <InvalidDataRemainder as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::InvalidDataRemainder)
                    }
                    InvalidDataRemainder
                },
                {
                    fn OptimismPortal_WrongProofMethod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_WrongProofMethod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_WrongProofMethod,
                            )
                    }
                    OptimismPortal_WrongProofMethod
                },
                {
                    fn ContentLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ContentLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::ContentLengthMismatch)
                    }
                    ContentLengthMismatch
                },
                {
                    fn OptimismPortal_GasLimitTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_GasLimitTooLow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_GasLimitTooLow,
                            )
                    }
                    OptimismPortal_GasLimitTooLow
                },
                {
                    fn OptimismPortal_AlreadyFinalized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_AlreadyFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_AlreadyFinalized,
                            )
                    }
                    OptimismPortal_AlreadyFinalized
                },
                {
                    fn OutOfGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OutOfGas as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::OutOfGas)
                    }
                    OutOfGas
                },
                {
                    fn OptimismPortal_MigratingToSameRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_MigratingToSameRegistry as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_MigratingToSameRegistry,
                            )
                    }
                    OptimismPortal_MigratingToSameRegistry
                },
                {
                    fn OptimismPortal_InvalidOutputRootChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidOutputRootChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidOutputRootChainId,
                            )
                    }
                    OptimismPortal_InvalidOutputRootChainId
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOwner
                },
                {
                    fn Encoding_EmptySuperRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <Encoding_EmptySuperRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::Encoding_EmptySuperRoot)
                    }
                    Encoding_EmptySuperRoot
                },
                {
                    fn ReinitializableBase_ZeroInitVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ReinitializableBase_ZeroInitVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ReinitializableBase_ZeroInitVersion,
                            )
                    }
                    ReinitializableBase_ZeroInitVersion
                },
                {
                    fn OptimismPortal_GasEstimation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_GasEstimation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_GasEstimation,
                            )
                    }
                    OptimismPortal_GasEstimation
                },
                {
                    fn OptimismPortal_InvalidProofTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidProofTimestamp as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidProofTimestamp,
                            )
                    }
                    OptimismPortal_InvalidProofTimestamp
                },
                {
                    fn OptimismPortal_CallPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_CallPaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::OptimismPortal_CallPaused)
                    }
                    OptimismPortal_CallPaused
                },
                {
                    fn InvalidHeader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <InvalidHeader as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::InvalidHeader)
                    }
                    InvalidHeader
                },
                {
                    fn Encoding_InvalidSuperRootVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <Encoding_InvalidSuperRootVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::Encoding_InvalidSuperRootVersion,
                            )
                    }
                    Encoding_InvalidSuperRootVersion
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdminOrProxyAdminOwner
                },
                {
                    fn OptimismPortal_BadTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_BadTarget as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::OptimismPortal_BadTarget)
                    }
                    OptimismPortal_BadTarget
                },
                {
                    fn OptimismPortal_Unproven(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_Unproven as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OptimismPortalInteropErrors::OptimismPortal_Unproven)
                    }
                    OptimismPortal_Unproven
                },
                {
                    fn OptimismPortal_ProofNotOldEnough(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_ProofNotOldEnough as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_ProofNotOldEnough,
                            )
                    }
                    OptimismPortal_ProofNotOldEnough
                },
                {
                    fn OptimismPortal_NoReentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_NoReentrancy,
                            )
                    }
                    OptimismPortal_NoReentrancy
                },
                {
                    fn OptimismPortal_InvalidDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_InvalidDisputeGame as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_InvalidDisputeGame,
                            )
                    }
                    OptimismPortal_InvalidDisputeGame
                },
                {
                    fn ProxyAdminOwnedBase_NotProxyAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <ProxyAdminOwnedBase_NotProxyAdmin as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::ProxyAdminOwnedBase_NotProxyAdmin,
                            )
                    }
                    ProxyAdminOwnedBase_NotProxyAdmin
                },
                {
                    fn OptimismPortal_ImproperDisputeGame(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OptimismPortalInteropErrors> {
                        <OptimismPortal_ImproperDisputeGame as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OptimismPortalInteropErrors::OptimismPortal_ImproperDisputeGame,
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
                Self::Encoding_EmptySuperRoot(inner) => {
                    <Encoding_EmptySuperRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Encoding_InvalidSuperRootVersion(inner) => {
                    <Encoding_InvalidSuperRootVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::OptimismPortal_InvalidMerkleProof(inner) => {
                    <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidOutputRootChainId(inner) => {
                    <OptimismPortal_InvalidOutputRootChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_InvalidOutputRootIndex(inner) => {
                    <OptimismPortal_InvalidOutputRootIndex as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::OptimismPortal_InvalidSuperRootProof(inner) => {
                    <OptimismPortal_InvalidSuperRootProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_MigratingToSameRegistry(inner) => {
                    <OptimismPortal_MigratingToSameRegistry as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OptimismPortal_NoReentrancy(inner) => {
                    <OptimismPortal_NoReentrancy as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::OptimismPortal_WrongProofMethod(inner) => {
                    <OptimismPortal_WrongProofMethod as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::Encoding_EmptySuperRoot(inner) => {
                    <Encoding_EmptySuperRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Encoding_InvalidSuperRootVersion(inner) => {
                    <Encoding_InvalidSuperRootVersion as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OptimismPortal_InvalidMerkleProof(inner) => {
                    <OptimismPortal_InvalidMerkleProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidOutputRootChainId(inner) => {
                    <OptimismPortal_InvalidOutputRootChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_InvalidOutputRootIndex(inner) => {
                    <OptimismPortal_InvalidOutputRootIndex as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OptimismPortal_InvalidSuperRootProof(inner) => {
                    <OptimismPortal_InvalidSuperRootProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OptimismPortal_MigratingToSameRegistry(inner) => {
                    <OptimismPortal_MigratingToSameRegistry as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OptimismPortal_WrongProofMethod(inner) => {
                    <OptimismPortal_WrongProofMethod as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`OptimismPortalInterop`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OptimismPortalInteropEvents {
        #[allow(missing_docs)]
        ETHMigrated(ETHMigrated),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        PortalMigrated(PortalMigrated),
        #[allow(missing_docs)]
        TransactionDeposited(TransactionDeposited),
        #[allow(missing_docs)]
        WithdrawalFinalized(WithdrawalFinalized),
        #[allow(missing_docs)]
        WithdrawalProven(WithdrawalProven),
        #[allow(missing_docs)]
        WithdrawalProvenExtension1(WithdrawalProvenExtension1),
    }
    impl OptimismPortalInteropEvents {
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
                158u8, 83u8, 104u8, 71u8, 26u8, 88u8, 216u8, 25u8, 135u8, 229u8, 220u8,
                125u8, 99u8, 116u8, 221u8, 94u8, 213u8, 231u8, 86u8, 204u8, 149u8, 167u8,
                159u8, 247u8, 38u8, 144u8, 52u8, 35u8, 188u8, 224u8, 6u8, 13u8,
            ],
            [
                179u8, 129u8, 53u8, 104u8, 217u8, 153u8, 31u8, 201u8, 81u8, 150u8, 31u8,
                203u8, 76u8, 120u8, 72u8, 147u8, 87u8, 66u8, 64u8, 162u8, 137u8, 37u8,
                96u8, 77u8, 9u8, 252u8, 87u8, 124u8, 85u8, 187u8, 124u8, 50u8,
            ],
            [
                216u8, 147u8, 246u8, 48u8, 198u8, 134u8, 127u8, 164u8, 54u8, 137u8,
                218u8, 154u8, 233u8, 73u8, 235u8, 240u8, 76u8, 172u8, 36u8, 170u8, 211u8,
                180u8, 92u8, 117u8, 157u8, 68u8, 46u8, 211u8, 195u8, 46u8, 58u8, 55u8,
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
            ::core::stringify!(PortalMigrated),
            ::core::stringify!(TransactionDeposited),
            ::core::stringify!(ETHMigrated),
            ::core::stringify!(WithdrawalFinalized),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <WithdrawalProven as alloy_sol_types::SolEvent>::SIGNATURE,
            <WithdrawalProvenExtension1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <PortalMigrated as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionDeposited as alloy_sol_types::SolEvent>::SIGNATURE,
            <ETHMigrated as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for OptimismPortalInteropEvents {
        const NAME: &'static str = "OptimismPortalInteropEvents";
        const COUNT: usize = 7usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ETHMigrated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ETHMigrated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ETHMigrated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(<PortalMigrated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PortalMigrated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PortalMigrated)
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
    impl alloy_sol_types::private::IntoLogData for OptimismPortalInteropEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ETHMigrated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PortalMigrated(inner) => {
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
                Self::ETHMigrated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PortalMigrated(inner) => {
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
    /**Creates a new wrapper around an on-chain [`OptimismPortalInterop`](self) contract instance.

See the [wrapper's documentation](`OptimismPortalInteropInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OptimismPortalInteropInstance<P, N> {
        OptimismPortalInteropInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<OptimismPortalInteropInstance<P, N>>,
    > {
        OptimismPortalInteropInstance::<
            P,
            N,
        >::deploy(__provider, _proofMaturityDelaySeconds)
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
        OptimismPortalInteropInstance::<
            P,
            N,
        >::deploy_builder(__provider, _proofMaturityDelaySeconds)
    }
    /**A [`OptimismPortalInterop`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OptimismPortalInterop`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OptimismPortalInteropInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OptimismPortalInteropInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OptimismPortalInteropInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OptimismPortalInteropInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OptimismPortalInterop`](self) contract instance.

See the [wrapper's documentation](`OptimismPortalInteropInstance`) for more details.*/
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
        ) -> alloy_contract::Result<OptimismPortalInteropInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> OptimismPortalInteropInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OptimismPortalInteropInstance<P, N> {
            OptimismPortalInteropInstance {
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
    > OptimismPortalInteropInstance<P, N> {
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
            _ethLockbox: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _systemConfig,
                    _anchorStateRegistry,
                    _ethLockbox,
                },
            )
        }
        ///Creates a new call builder for the [`l2Sender`] function.
        pub fn l2Sender(&self) -> alloy_contract::SolCallBuilder<&P, l2SenderCall, N> {
            self.call_builder(&l2SenderCall)
        }
        ///Creates a new call builder for the [`migrateLiquidity`] function.
        pub fn migrateLiquidity(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, migrateLiquidityCall, N> {
            self.call_builder(&migrateLiquidityCall)
        }
        ///Creates a new call builder for the [`migrateToSuperRoots`] function.
        pub fn migrateToSuperRoots(
            &self,
            _newLockbox: alloy::sol_types::private::Address,
            _newAnchorStateRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, migrateToSuperRootsCall, N> {
            self.call_builder(
                &migrateToSuperRootsCall {
                    _newLockbox,
                    _newAnchorStateRegistry,
                },
            )
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
        ///Creates a new call builder for the [`proveWithdrawalTransaction_0`] function.
        pub fn proveWithdrawalTransaction_0(
            &self,
            _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
            _disputeGameIndex: alloy::sol_types::private::primitives::aliases::U256,
            _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
            _withdrawalProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, proveWithdrawalTransaction_0Call, N> {
            self.call_builder(
                &proveWithdrawalTransaction_0Call {
                    _tx,
                    _disputeGameIndex,
                    _outputRootProof,
                    _withdrawalProof,
                },
            )
        }
        ///Creates a new call builder for the [`proveWithdrawalTransaction_1`] function.
        pub fn proveWithdrawalTransaction_1(
            &self,
            _tx: <Types::WithdrawalTransaction as alloy::sol_types::SolType>::RustType,
            _disputeGameProxy: alloy::sol_types::private::Address,
            _outputRootIndex: alloy::sol_types::private::primitives::aliases::U256,
            _superRootProof: <Types::SuperRootProof as alloy::sol_types::SolType>::RustType,
            _outputRootProof: <Types::OutputRootProof as alloy::sol_types::SolType>::RustType,
            _withdrawalProof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, proveWithdrawalTransaction_1Call, N> {
            self.call_builder(
                &proveWithdrawalTransaction_1Call {
                    _tx,
                    _disputeGameProxy,
                    _outputRootIndex,
                    _superRootProof,
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
        ///Creates a new call builder for the [`superRootsActive`] function.
        pub fn superRootsActive(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, superRootsActiveCall, N> {
            self.call_builder(&superRootsActiveCall)
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
        ///Creates a new call builder for the [`upgrade`] function.
        pub fn upgrade(
            &self,
            _anchorStateRegistry: alloy::sol_types::private::Address,
            _ethLockbox: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, upgradeCall, N> {
            self.call_builder(
                &upgradeCall {
                    _anchorStateRegistry,
                    _ethLockbox,
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
    > OptimismPortalInteropInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ETHMigrated`] event.
        pub fn ETHMigrated_filter(&self) -> alloy_contract::Event<&P, ETHMigrated, N> {
            self.event_filter::<ETHMigrated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`PortalMigrated`] event.
        pub fn PortalMigrated_filter(
            &self,
        ) -> alloy_contract::Event<&P, PortalMigrated, N> {
            self.event_filter::<PortalMigrated>()
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

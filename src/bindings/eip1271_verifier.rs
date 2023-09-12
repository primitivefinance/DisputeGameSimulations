/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod eip1271_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAttestTypeHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAttestTypeHash"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDomainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDomainSeparator"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getName"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRevokeTypeHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRevokeTypeHash"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("increaseNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NonceIncreased"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NonceIncreased"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidNonce"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EIP1271VERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct EIP1271Verifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EIP1271Verifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EIP1271Verifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EIP1271Verifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EIP1271Verifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EIP1271Verifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EIP1271Verifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EIP1271VERIFIER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getAttestTypeHash` (0x12b11a17) function
        pub fn get_attest_type_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([18, 177, 26, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDomainSeparator` (0xed24911d) function
        pub fn get_domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([237, 36, 145, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getName` (0x17d7de7c) function
        pub fn get_name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([23, 215, 222, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x2d0335ab) function
        pub fn get_nonce(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 3, 53, 171], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRevokeTypeHash` (0xb83010d3) function
        pub fn get_revoke_type_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 48, 16, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseNonce` (0x79f7573a) function
        pub fn increase_nonce(
            &self,
            new_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 247, 87, 58], new_nonce)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NonceIncreased` event
        pub fn nonce_increased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NonceIncreasedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NonceIncreasedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EIP1271Verifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidNonce` with signature `InvalidNonce()` and selector `0x756688fe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidNonce", abi = "InvalidNonce()")]
    pub struct InvalidNonce;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "NonceIncreased", abi = "NonceIncreased(uint256,uint256)")]
    pub struct NonceIncreasedFilter {
        pub old_nonce: ::ethers::core::types::U256,
        pub new_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAttestTypeHash` function with signature `getAttestTypeHash()` and selector `0x12b11a17`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAttestTypeHash", abi = "getAttestTypeHash()")]
    pub struct GetAttestTypeHashCall;
    ///Container type for all input parameters for the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getDomainSeparator", abi = "getDomainSeparator()")]
    pub struct GetDomainSeparatorCall;
    ///Container type for all input parameters for the `getName` function with signature `getName()` and selector `0x17d7de7c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getName", abi = "getName()")]
    pub struct GetNameCall;
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getRevokeTypeHash` function with signature `getRevokeTypeHash()` and selector `0xb83010d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRevokeTypeHash", abi = "getRevokeTypeHash()")]
    pub struct GetRevokeTypeHashCall;
    ///Container type for all input parameters for the `increaseNonce` function with signature `increaseNonce(uint256)` and selector `0x79f7573a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "increaseNonce", abi = "increaseNonce(uint256)")]
    pub struct IncreaseNonceCall {
        pub new_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EIP1271VerifierCalls {
        GetAttestTypeHash(GetAttestTypeHashCall),
        GetDomainSeparator(GetDomainSeparatorCall),
        GetName(GetNameCall),
        GetNonce(GetNonceCall),
        GetRevokeTypeHash(GetRevokeTypeHashCall),
        IncreaseNonce(IncreaseNonceCall),
    }
    impl ::ethers::core::abi::AbiDecode for EIP1271VerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetAttestTypeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAttestTypeHash(decoded));
            }
            if let Ok(decoded) = <GetDomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDomainSeparator(decoded));
            }
            if let Ok(decoded) = <GetNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetName(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) = <GetRevokeTypeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRevokeTypeHash(decoded));
            }
            if let Ok(decoded) = <IncreaseNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseNonce(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EIP1271VerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAttestTypeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetName(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRevokeTypeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EIP1271VerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAttestTypeHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDomainSeparator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetName(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRevokeTypeHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseNonce(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetAttestTypeHashCall> for EIP1271VerifierCalls {
        fn from(value: GetAttestTypeHashCall) -> Self {
            Self::GetAttestTypeHash(value)
        }
    }
    impl ::core::convert::From<GetDomainSeparatorCall> for EIP1271VerifierCalls {
        fn from(value: GetDomainSeparatorCall) -> Self {
            Self::GetDomainSeparator(value)
        }
    }
    impl ::core::convert::From<GetNameCall> for EIP1271VerifierCalls {
        fn from(value: GetNameCall) -> Self {
            Self::GetName(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for EIP1271VerifierCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetRevokeTypeHashCall> for EIP1271VerifierCalls {
        fn from(value: GetRevokeTypeHashCall) -> Self {
            Self::GetRevokeTypeHash(value)
        }
    }
    impl ::core::convert::From<IncreaseNonceCall> for EIP1271VerifierCalls {
        fn from(value: IncreaseNonceCall) -> Self {
            Self::IncreaseNonce(value)
        }
    }
    ///Container type for all return fields from the `getAttestTypeHash` function with signature `getAttestTypeHash()` and selector `0x12b11a17`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAttestTypeHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetDomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getName` function with signature `getName()` and selector `0x17d7de7c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRevokeTypeHash` function with signature `getRevokeTypeHash()` and selector `0xb83010d3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRevokeTypeHashReturn(pub [u8; 32]);
}

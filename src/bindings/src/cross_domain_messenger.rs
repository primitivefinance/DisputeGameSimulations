pub use cross_domain_messenger::*;
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
pub mod cross_domain_messenger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MESSAGE_VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MESSAGE_VERSION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MIN_GAS_CALLDATA_OVERHEAD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MIN_GAS_CALLDATA_OVERHEAD",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OTHER_MESSENGER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("OTHER_MESSENGER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RELAY_CALL_OVERHEAD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RELAY_CALL_OVERHEAD",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RELAY_CONSTANT_OVERHEAD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RELAY_CONSTANT_OVERHEAD",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RELAY_GAS_CHECK_BUFFER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RELAY_GAS_CHECK_BUFFER",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RELAY_RESERVED_GAS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RELAY_RESERVED_GAS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("baseGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("baseGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failedMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedMessages"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("messageNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("messageNonce"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("relayMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relayMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("successfulMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("successfulMessages"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("xDomainMessageSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "xDomainMessageSender",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FailedRelayedMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FailedRelayedMessage",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayedMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RelayedMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SentMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SentMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasLimit"),
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
                (
                    ::std::borrow::ToOwned::to_owned("SentMessageExtension1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SentMessageExtension1",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CROSSDOMAINMESSENGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct CrossDomainMessenger<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CrossDomainMessenger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CrossDomainMessenger<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CrossDomainMessenger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CrossDomainMessenger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CrossDomainMessenger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CrossDomainMessenger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CROSSDOMAINMESSENGER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `MESSAGE_VERSION` (0x3f827a5a) function
        pub fn message_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([63, 130, 122, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MIN_GAS_CALLDATA_OVERHEAD` (0x028f85f7) function
        pub fn min_gas_calldata_overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([2, 143, 133, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR` (0x0c568498) function
        pub fn min_gas_dynamic_overhead_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([12, 86, 132, 152], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR` (0x2828d7e8) function
        pub fn min_gas_dynamic_overhead_numerator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([40, 40, 215, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OTHER_MESSENGER` (0x9fce812c) function
        pub fn other_messenger(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([159, 206, 129, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELAY_CALL_OVERHEAD` (0x4c1d6a69) function
        pub fn relay_call_overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([76, 29, 106, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELAY_CONSTANT_OVERHEAD` (0x83a74074) function
        pub fn relay_constant_overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([131, 167, 64, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELAY_GAS_CHECK_BUFFER` (0x5644cfdf) function
        pub fn relay_gas_check_buffer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([86, 68, 207, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELAY_RESERVED_GAS` (0x8cbeeef2) function
        pub fn relay_reserved_gas(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([140, 190, 238, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `baseGas` (0xb28ade25) function
        pub fn base_gas(
            &self,
            message: ::ethers::core::types::Bytes,
            min_gas_limit: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([178, 138, 222, 37], (message, min_gas_limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMessages` (0xa4e7f8bd) function
        pub fn failed_messages(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 231, 248, 189], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messageNonce` (0xecc70428) function
        pub fn message_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 199, 4, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayMessage` (0xd764ad0b) function
        pub fn relay_message(
            &self,
            nonce: ::ethers::core::types::U256,
            sender: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            min_gas_limit: ::ethers::core::types::U256,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 100, 173, 11],
                    (nonce, sender, target, value, min_gas_limit, message),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendMessage` (0x3dbb202b) function
        pub fn send_message(
            &self,
            target: ::ethers::core::types::Address,
            message: ::ethers::core::types::Bytes,
            min_gas_limit: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 187, 32, 43], (target, message, min_gas_limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `successfulMessages` (0xb1b1b209) function
        pub fn successful_messages(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 177, 178, 9], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `xDomainMessageSender` (0x6e296e45) function
        pub fn x_domain_message_sender(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([110, 41, 110, 69], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FailedRelayedMessage` event
        pub fn failed_relayed_message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FailedRelayedMessageFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayedMessage` event
        pub fn relayed_message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayedMessageFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SentMessage` event
        pub fn sent_message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SentMessageFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SentMessageExtension1` event
        pub fn sent_message_extension_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SentMessageExtension1Filter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CrossDomainMessengerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CrossDomainMessenger<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    #[ethevent(name = "FailedRelayedMessage", abi = "FailedRelayedMessage(bytes32)")]
    pub struct FailedRelayedMessageFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
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
    #[ethevent(name = "RelayedMessage", abi = "RelayedMessage(bytes32)")]
    pub struct RelayedMessageFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
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
    #[ethevent(
        name = "SentMessage",
        abi = "SentMessage(address,address,bytes,uint256,uint256)"
    )]
    pub struct SentMessageFilter {
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
        pub sender: ::ethers::core::types::Address,
        pub message: ::ethers::core::types::Bytes,
        pub message_nonce: ::ethers::core::types::U256,
        pub gas_limit: ::ethers::core::types::U256,
    }
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
    #[ethevent(
        name = "SentMessageExtension1",
        abi = "SentMessageExtension1(address,uint256)"
    )]
    pub struct SentMessageExtension1Filter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CrossDomainMessengerEvents {
        FailedRelayedMessageFilter(FailedRelayedMessageFilter),
        InitializedFilter(InitializedFilter),
        RelayedMessageFilter(RelayedMessageFilter),
        SentMessageFilter(SentMessageFilter),
        SentMessageExtension1Filter(SentMessageExtension1Filter),
    }
    impl ::ethers::contract::EthLogDecode for CrossDomainMessengerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FailedRelayedMessageFilter::decode_log(log) {
                return Ok(
                    CrossDomainMessengerEvents::FailedRelayedMessageFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(CrossDomainMessengerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = RelayedMessageFilter::decode_log(log) {
                return Ok(CrossDomainMessengerEvents::RelayedMessageFilter(decoded));
            }
            if let Ok(decoded) = SentMessageFilter::decode_log(log) {
                return Ok(CrossDomainMessengerEvents::SentMessageFilter(decoded));
            }
            if let Ok(decoded) = SentMessageExtension1Filter::decode_log(log) {
                return Ok(
                    CrossDomainMessengerEvents::SentMessageExtension1Filter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CrossDomainMessengerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FailedRelayedMessageFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayedMessageFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SentMessageFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SentMessageExtension1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FailedRelayedMessageFilter>
    for CrossDomainMessengerEvents {
        fn from(value: FailedRelayedMessageFilter) -> Self {
            Self::FailedRelayedMessageFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for CrossDomainMessengerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<RelayedMessageFilter> for CrossDomainMessengerEvents {
        fn from(value: RelayedMessageFilter) -> Self {
            Self::RelayedMessageFilter(value)
        }
    }
    impl ::core::convert::From<SentMessageFilter> for CrossDomainMessengerEvents {
        fn from(value: SentMessageFilter) -> Self {
            Self::SentMessageFilter(value)
        }
    }
    impl ::core::convert::From<SentMessageExtension1Filter>
    for CrossDomainMessengerEvents {
        fn from(value: SentMessageExtension1Filter) -> Self {
            Self::SentMessageExtension1Filter(value)
        }
    }
    ///Container type for all input parameters for the `MESSAGE_VERSION` function with signature `MESSAGE_VERSION()` and selector `0x3f827a5a`
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
    #[ethcall(name = "MESSAGE_VERSION", abi = "MESSAGE_VERSION()")]
    pub struct MessageVersionCall;
    ///Container type for all input parameters for the `MIN_GAS_CALLDATA_OVERHEAD` function with signature `MIN_GAS_CALLDATA_OVERHEAD()` and selector `0x028f85f7`
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
    #[ethcall(name = "MIN_GAS_CALLDATA_OVERHEAD", abi = "MIN_GAS_CALLDATA_OVERHEAD()")]
    pub struct MinGasCalldataOverheadCall;
    ///Container type for all input parameters for the `MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR` function with signature `MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR()` and selector `0x0c568498`
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
    #[ethcall(
        name = "MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR",
        abi = "MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR()"
    )]
    pub struct MinGasDynamicOverheadDenominatorCall;
    ///Container type for all input parameters for the `MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR` function with signature `MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR()` and selector `0x2828d7e8`
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
    #[ethcall(
        name = "MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR",
        abi = "MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR()"
    )]
    pub struct MinGasDynamicOverheadNumeratorCall;
    ///Container type for all input parameters for the `OTHER_MESSENGER` function with signature `OTHER_MESSENGER()` and selector `0x9fce812c`
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
    #[ethcall(name = "OTHER_MESSENGER", abi = "OTHER_MESSENGER()")]
    pub struct OtherMessengerCall;
    ///Container type for all input parameters for the `RELAY_CALL_OVERHEAD` function with signature `RELAY_CALL_OVERHEAD()` and selector `0x4c1d6a69`
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
    #[ethcall(name = "RELAY_CALL_OVERHEAD", abi = "RELAY_CALL_OVERHEAD()")]
    pub struct RelayCallOverheadCall;
    ///Container type for all input parameters for the `RELAY_CONSTANT_OVERHEAD` function with signature `RELAY_CONSTANT_OVERHEAD()` and selector `0x83a74074`
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
    #[ethcall(name = "RELAY_CONSTANT_OVERHEAD", abi = "RELAY_CONSTANT_OVERHEAD()")]
    pub struct RelayConstantOverheadCall;
    ///Container type for all input parameters for the `RELAY_GAS_CHECK_BUFFER` function with signature `RELAY_GAS_CHECK_BUFFER()` and selector `0x5644cfdf`
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
    #[ethcall(name = "RELAY_GAS_CHECK_BUFFER", abi = "RELAY_GAS_CHECK_BUFFER()")]
    pub struct RelayGasCheckBufferCall;
    ///Container type for all input parameters for the `RELAY_RESERVED_GAS` function with signature `RELAY_RESERVED_GAS()` and selector `0x8cbeeef2`
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
    #[ethcall(name = "RELAY_RESERVED_GAS", abi = "RELAY_RESERVED_GAS()")]
    pub struct RelayReservedGasCall;
    ///Container type for all input parameters for the `baseGas` function with signature `baseGas(bytes,uint32)` and selector `0xb28ade25`
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
    #[ethcall(name = "baseGas", abi = "baseGas(bytes,uint32)")]
    pub struct BaseGasCall {
        pub message: ::ethers::core::types::Bytes,
        pub min_gas_limit: u32,
    }
    ///Container type for all input parameters for the `failedMessages` function with signature `failedMessages(bytes32)` and selector `0xa4e7f8bd`
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
    #[ethcall(name = "failedMessages", abi = "failedMessages(bytes32)")]
    pub struct FailedMessagesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `messageNonce` function with signature `messageNonce()` and selector `0xecc70428`
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
    #[ethcall(name = "messageNonce", abi = "messageNonce()")]
    pub struct MessageNonceCall;
    ///Container type for all input parameters for the `relayMessage` function with signature `relayMessage(uint256,address,address,uint256,uint256,bytes)` and selector `0xd764ad0b`
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
    #[ethcall(
        name = "relayMessage",
        abi = "relayMessage(uint256,address,address,uint256,uint256,bytes)"
    )]
    pub struct RelayMessageCall {
        pub nonce: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub min_gas_limit: ::ethers::core::types::U256,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,bytes,uint32)` and selector `0x3dbb202b`
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
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,bytes,uint32)")]
    pub struct SendMessageCall {
        pub target: ::ethers::core::types::Address,
        pub message: ::ethers::core::types::Bytes,
        pub min_gas_limit: u32,
    }
    ///Container type for all input parameters for the `successfulMessages` function with signature `successfulMessages(bytes32)` and selector `0xb1b1b209`
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
    #[ethcall(name = "successfulMessages", abi = "successfulMessages(bytes32)")]
    pub struct SuccessfulMessagesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `xDomainMessageSender` function with signature `xDomainMessageSender()` and selector `0x6e296e45`
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
    #[ethcall(name = "xDomainMessageSender", abi = "xDomainMessageSender()")]
    pub struct XdomainMessageSenderCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CrossDomainMessengerCalls {
        MessageVersion(MessageVersionCall),
        MinGasCalldataOverhead(MinGasCalldataOverheadCall),
        MinGasDynamicOverheadDenominator(MinGasDynamicOverheadDenominatorCall),
        MinGasDynamicOverheadNumerator(MinGasDynamicOverheadNumeratorCall),
        OtherMessenger(OtherMessengerCall),
        RelayCallOverhead(RelayCallOverheadCall),
        RelayConstantOverhead(RelayConstantOverheadCall),
        RelayGasCheckBuffer(RelayGasCheckBufferCall),
        RelayReservedGas(RelayReservedGasCall),
        BaseGas(BaseGasCall),
        FailedMessages(FailedMessagesCall),
        MessageNonce(MessageNonceCall),
        RelayMessage(RelayMessageCall),
        SendMessage(SendMessageCall),
        SuccessfulMessages(SuccessfulMessagesCall),
        XdomainMessageSender(XdomainMessageSenderCall),
    }
    impl ::ethers::core::abi::AbiDecode for CrossDomainMessengerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MessageVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MessageVersion(decoded));
            }
            if let Ok(decoded) = <MinGasCalldataOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinGasCalldataOverhead(decoded));
            }
            if let Ok(decoded) = <MinGasDynamicOverheadDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinGasDynamicOverheadDenominator(decoded));
            }
            if let Ok(decoded) = <MinGasDynamicOverheadNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinGasDynamicOverheadNumerator(decoded));
            }
            if let Ok(decoded) = <OtherMessengerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OtherMessenger(decoded));
            }
            if let Ok(decoded) = <RelayCallOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayCallOverhead(decoded));
            }
            if let Ok(decoded) = <RelayConstantOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayConstantOverhead(decoded));
            }
            if let Ok(decoded) = <RelayGasCheckBufferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayGasCheckBuffer(decoded));
            }
            if let Ok(decoded) = <RelayReservedGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayReservedGas(decoded));
            }
            if let Ok(decoded) = <BaseGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseGas(decoded));
            }
            if let Ok(decoded) = <FailedMessagesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedMessages(decoded));
            }
            if let Ok(decoded) = <MessageNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MessageNonce(decoded));
            }
            if let Ok(decoded) = <RelayMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayMessage(decoded));
            }
            if let Ok(decoded) = <SendMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendMessage(decoded));
            }
            if let Ok(decoded) = <SuccessfulMessagesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SuccessfulMessages(decoded));
            }
            if let Ok(decoded) = <XdomainMessageSenderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::XdomainMessageSender(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CrossDomainMessengerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MessageVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinGasCalldataOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinGasDynamicOverheadDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinGasDynamicOverheadNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OtherMessenger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayCallOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayConstantOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayGasCheckBuffer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayReservedGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseGas(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MessageNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SuccessfulMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::XdomainMessageSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CrossDomainMessengerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MessageVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinGasCalldataOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinGasDynamicOverheadDenominator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinGasDynamicOverheadNumerator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OtherMessenger(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayCallOverhead(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayConstantOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayGasCheckBuffer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayReservedGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SuccessfulMessages(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::XdomainMessageSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MessageVersionCall> for CrossDomainMessengerCalls {
        fn from(value: MessageVersionCall) -> Self {
            Self::MessageVersion(value)
        }
    }
    impl ::core::convert::From<MinGasCalldataOverheadCall>
    for CrossDomainMessengerCalls {
        fn from(value: MinGasCalldataOverheadCall) -> Self {
            Self::MinGasCalldataOverhead(value)
        }
    }
    impl ::core::convert::From<MinGasDynamicOverheadDenominatorCall>
    for CrossDomainMessengerCalls {
        fn from(value: MinGasDynamicOverheadDenominatorCall) -> Self {
            Self::MinGasDynamicOverheadDenominator(value)
        }
    }
    impl ::core::convert::From<MinGasDynamicOverheadNumeratorCall>
    for CrossDomainMessengerCalls {
        fn from(value: MinGasDynamicOverheadNumeratorCall) -> Self {
            Self::MinGasDynamicOverheadNumerator(value)
        }
    }
    impl ::core::convert::From<OtherMessengerCall> for CrossDomainMessengerCalls {
        fn from(value: OtherMessengerCall) -> Self {
            Self::OtherMessenger(value)
        }
    }
    impl ::core::convert::From<RelayCallOverheadCall> for CrossDomainMessengerCalls {
        fn from(value: RelayCallOverheadCall) -> Self {
            Self::RelayCallOverhead(value)
        }
    }
    impl ::core::convert::From<RelayConstantOverheadCall> for CrossDomainMessengerCalls {
        fn from(value: RelayConstantOverheadCall) -> Self {
            Self::RelayConstantOverhead(value)
        }
    }
    impl ::core::convert::From<RelayGasCheckBufferCall> for CrossDomainMessengerCalls {
        fn from(value: RelayGasCheckBufferCall) -> Self {
            Self::RelayGasCheckBuffer(value)
        }
    }
    impl ::core::convert::From<RelayReservedGasCall> for CrossDomainMessengerCalls {
        fn from(value: RelayReservedGasCall) -> Self {
            Self::RelayReservedGas(value)
        }
    }
    impl ::core::convert::From<BaseGasCall> for CrossDomainMessengerCalls {
        fn from(value: BaseGasCall) -> Self {
            Self::BaseGas(value)
        }
    }
    impl ::core::convert::From<FailedMessagesCall> for CrossDomainMessengerCalls {
        fn from(value: FailedMessagesCall) -> Self {
            Self::FailedMessages(value)
        }
    }
    impl ::core::convert::From<MessageNonceCall> for CrossDomainMessengerCalls {
        fn from(value: MessageNonceCall) -> Self {
            Self::MessageNonce(value)
        }
    }
    impl ::core::convert::From<RelayMessageCall> for CrossDomainMessengerCalls {
        fn from(value: RelayMessageCall) -> Self {
            Self::RelayMessage(value)
        }
    }
    impl ::core::convert::From<SendMessageCall> for CrossDomainMessengerCalls {
        fn from(value: SendMessageCall) -> Self {
            Self::SendMessage(value)
        }
    }
    impl ::core::convert::From<SuccessfulMessagesCall> for CrossDomainMessengerCalls {
        fn from(value: SuccessfulMessagesCall) -> Self {
            Self::SuccessfulMessages(value)
        }
    }
    impl ::core::convert::From<XdomainMessageSenderCall> for CrossDomainMessengerCalls {
        fn from(value: XdomainMessageSenderCall) -> Self {
            Self::XdomainMessageSender(value)
        }
    }
    ///Container type for all return fields from the `MESSAGE_VERSION` function with signature `MESSAGE_VERSION()` and selector `0x3f827a5a`
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
    pub struct MessageVersionReturn(pub u16);
    ///Container type for all return fields from the `MIN_GAS_CALLDATA_OVERHEAD` function with signature `MIN_GAS_CALLDATA_OVERHEAD()` and selector `0x028f85f7`
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
    pub struct MinGasCalldataOverheadReturn(pub u64);
    ///Container type for all return fields from the `MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR` function with signature `MIN_GAS_DYNAMIC_OVERHEAD_DENOMINATOR()` and selector `0x0c568498`
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
    pub struct MinGasDynamicOverheadDenominatorReturn(pub u64);
    ///Container type for all return fields from the `MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR` function with signature `MIN_GAS_DYNAMIC_OVERHEAD_NUMERATOR()` and selector `0x2828d7e8`
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
    pub struct MinGasDynamicOverheadNumeratorReturn(pub u64);
    ///Container type for all return fields from the `OTHER_MESSENGER` function with signature `OTHER_MESSENGER()` and selector `0x9fce812c`
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
    pub struct OtherMessengerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `RELAY_CALL_OVERHEAD` function with signature `RELAY_CALL_OVERHEAD()` and selector `0x4c1d6a69`
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
    pub struct RelayCallOverheadReturn(pub u64);
    ///Container type for all return fields from the `RELAY_CONSTANT_OVERHEAD` function with signature `RELAY_CONSTANT_OVERHEAD()` and selector `0x83a74074`
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
    pub struct RelayConstantOverheadReturn(pub u64);
    ///Container type for all return fields from the `RELAY_GAS_CHECK_BUFFER` function with signature `RELAY_GAS_CHECK_BUFFER()` and selector `0x5644cfdf`
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
    pub struct RelayGasCheckBufferReturn(pub u64);
    ///Container type for all return fields from the `RELAY_RESERVED_GAS` function with signature `RELAY_RESERVED_GAS()` and selector `0x8cbeeef2`
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
    pub struct RelayReservedGasReturn(pub u64);
    ///Container type for all return fields from the `baseGas` function with signature `baseGas(bytes,uint32)` and selector `0xb28ade25`
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
    pub struct BaseGasReturn(pub u64);
    ///Container type for all return fields from the `failedMessages` function with signature `failedMessages(bytes32)` and selector `0xa4e7f8bd`
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
    pub struct FailedMessagesReturn(pub bool);
    ///Container type for all return fields from the `messageNonce` function with signature `messageNonce()` and selector `0xecc70428`
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
    pub struct MessageNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `successfulMessages` function with signature `successfulMessages(bytes32)` and selector `0xb1b1b209`
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
    pub struct SuccessfulMessagesReturn(pub bool);
    ///Container type for all return fields from the `xDomainMessageSender` function with signature `xDomainMessageSender()` and selector `0x6e296e45`
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
    pub struct XdomainMessageSenderReturn(pub ::ethers::core::types::Address);
}

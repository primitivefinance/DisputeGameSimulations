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
pub mod l2_cross_domain_messenger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_l1CrossDomainMessenger",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("l1CrossDomainMessenger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l1CrossDomainMessenger",
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
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("version"),
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
    pub static L2CROSSDOMAINMESSENGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0-O8\x03\x80b\0-O\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\x02AV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R`\x01`\xA0\x81\x90R`\x05`\xC0R`\xE0Rb\0\0\xA8b\0\0\xAFV[Pb\0\x02\xBEV[`\0T`\x02\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15\x80\x15b\0\0\xDBWP`\0T`\xFF\x80\x83\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x10[b\0\x01DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF`\xA8\x1B\x19`\xFF\x84\x16`\x01`\xA0\x1B\x02\x16a\xFF\xFF`\xA0\x1B\x19\x90\x91\x16\x17`\x01`\xA8\x1B\x17\x90Ub\0\x01vb\0\x01\xBCV[`\0\x80T`\xFF`\xA8\x1B\x19\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0T`\x01`\xA8\x1B\x90\x04`\xFF\x16b\0\x02+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01;V[`\xCC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16a\xDE\xAD\x17\x90UV[`\0` \x82\x84\x03\x12\x15b\0\x02\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\xB7W`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa*Bb\0\x03\r`\09`\0a\x0Fe\x01R`\0a\x0F<\x01R`\0a\x0F\x13\x01R`\0\x81\x81a\x08\xC1\x01R\x81\x81a\n6\x01R\x81\x81a\x0C\xAD\x01Ra\x15\0\x01Ra*B`\0\xF3\xFE`\x80`@R`\x046\x10a\x01DW`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xC0W\x80c\xA7\x11\x98i\x11a\0tW\x80c\xB2\x8A\xDE%\x11a\0YW\x80c\xB2\x8A\xDE%\x14a\x0B\x0CW\x80c\xD7d\xAD\x0B\x14a\x0B\xAEW\x80c\xEC\xC7\x04(\x14a\x0B\xC1Wa\x01DV[\x80c\xA7\x11\x98i\x14a\t\xA5W\x80c\xB1\xB1\xB2\t\x14a\nZWa\x01DV[\x80c\x8C\xBE\xEE\xF2\x11a\0\xA5W\x80c\x8C\xBE\xEE\xF2\x14a\x04mW\x80c\x9F\xCE\x81,\x14a\x08-W\x80c\xA4\xE7\xF8\xBD\x14a\x08\xE3Wa\x01DV[\x80c\x81)\xFC\x1C\x14a\x06\xFDW\x80c\x83\xA7@t\x14a\x07\x94Wa\x01DV[\x80c?\x82zZ\x11a\x01\x17W\x80cT\xFDMP\x11a\0\xFCW\x80cT\xFDMP\x14a\x05\x05W\x80cVD\xCF\xDF\x14a\x05\xA9W\x80cn)nE\x14a\x06AWa\x01DV[\x80c?\x82zZ\x14a\x03\xC3W\x80cL\x1Dji\x14a\x04mWa\x01DV[\x80c\x02\x8F\x85\xF7\x14a\x01\xCBW\x80c\x0CV\x84\x98\x14a\x02\x80W\x80c((\xD7\xE8\x14a\x03\x17W\x80c=\xBB +\x14a\x03\xAEW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x02YW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02b`\x10\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02b`?\x81V[4\x80\x15a\x03\xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02b`@\x81V[a\x03\xC1a\x03\xBC6`\x04a#\xE6V[a\x0C\xA8V[\0[4\x80\x15a\x04QW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04Z`\x01\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02wV[4\x80\x15a\x04\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ba\x9C@\x81V[4\x80\x15a\x05\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x05\x9Ca\x0F\x0CV[`@Qa\x02w\x91\x90a$\xCBV[4\x80\x15a\x067W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ba\x13\x88\x81V[4\x80\x15a\x06\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x06\xD8a\x0F\xAFV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02wV[4\x80\x15a\x07\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xC1a\x10\x9BV[4\x80\x15a\x08\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02bb\x03\r@\x81V[4\x80\x15a\x08\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x06\xD8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\tqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\t\x95a\t\x806`\x04a$\xE5V[`\xCE` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02wV[4\x80\x15a\n3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xD8V[4\x80\x15a\n\xE8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\t\x95a\n\xF76`\x04a$\xE5V[`\xCB` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ba\x0B\xA96`\x04a%\x01V[a\x12^V[a\x03\xC1a\x0B\xBC6`\x04a%[V[a\x12\xCCV[4\x80\x15a\x0COW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x0C\x9A`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[`@Q\x90\x81R` \x01a\x02wV[a\r\xE1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C\xD7\x85\x85\x85a\x12^V[4\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\rC`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[3\x8A4\x89\x8C\x8C`@Q`$\x01a\r_\x97\x96\x95\x94\x93\x92\x91\x90a&,V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1B\xA9V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\x0F\x7F\xFDx\xF9\xAE\xE4z$\x8F\xAE\x8D\xB1\x81\xDBn\xEE\x8309\x12>\x02m\xCB\xFFR\x95\"\xE5*3\x85\x85a\x0Ef`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x86`@Qa\x0Ex\x95\x94\x93\x92\x91\x90a&\x8BV[`@Q\x80\x91\x03\x90\xA2`@Q4\x81R3\x90\x7F\x8E\xBB.\xC2F[\xDB*\x06\xA6o\xC3z\tc\xAF\x8A*j\x14y\xD8\x1DV\xFD\xB8\xCB\xB9\x80\x96\xD5F\x90` \x01`@Q\x80\x91\x03\x90\xA2PP`\xCD\x80T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01\x01\x16\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPV[``a\x0F7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1C\xB9V[a\x0F`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1C\xB9V[a\x0F\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1C\xB9V[`@Q` \x01a\x0F\x9B\x93\x92\x91\x90a&\xD9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\xCCT`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!S\x01a\x10~W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FCrossDomainMessenger: xDomainMes`D\x82\x01R\x7FsageSender is not set\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[P`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\0T`\x02\x90u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15\x80\x15a\x10\xE9WP`\0T`\xFF\x80\x83\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04\x16\x10[a\x11uW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x84\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ua\x11\xFDa\x1D\xEEV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0a\x13\x88a\x9C@\x80`?a\x12z`@c\xFF\xFF\xFF\xFF\x88\x16a'~V[a\x12\x84\x91\x90a'\xDDV[a\x12\x8F`\x10\x88a'~V[a\x12\x9C\x90b\x03\r@a(\x04V[a\x12\xA6\x91\x90a(\x04V[a\x12\xB0\x91\x90a(\x04V[a\x12\xBA\x91\x90a(\x04V[a\x12\xC4\x91\x90a(\x04V[\x94\x93PPPPV[`\xF0\x87\x90\x1C`\x02\x81\x10a\x13\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FCrossDomainMessenger: only versi`D\x82\x01R\x7Fon 0 or 1 messages are supported`d\x82\x01R\x7F at this time\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x10uV[\x80a\xFF\xFF\x16`\0\x03a\x14|W`\0a\x13\xD8\x87\x89\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8F\x92Pa\x1E\xC7\x91PPV[`\0\x81\x81R`\xCB` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x14zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FCrossDomainMessenger: legacy wit`D\x82\x01R\x7Fhdrawal already relayed\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[P[`\0a\x14\xC2\x89\x89\x89\x89\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xE6\x92PPPV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEE\xEE\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEE\xEF3\x01\x81\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x03a\x15ZW\x854\x14a\x156Wa\x156a(0V[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16\x15a\x15UWa\x15Ua(0V[a\x16\xACV[4\x15a\x16\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FCrossDomainMessenger: value must`D\x82\x01R\x7F be zero unless message is from `d\x82\x01R\x7Fa system address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x10uV[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16a\x16\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FCrossDomainMessenger: message ca`D\x82\x01R\x7Fnnot be replayed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[a\x16\xB5\x87a\x1F\tV[\x15a\x17hW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FCrossDomainMessenger: cannot sen`D\x82\x01R\x7Fd message to blocked system addr`d\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x10uV[`\0\x81\x81R`\xCB` R`@\x90 T`\xFF\x16\x15a\x18\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FCrossDomainMessenger: message ha`D\x82\x01R\x7Fs already been relayed\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[a\x18(\x85a\x18\x19a\x13\x88a\x9C@a(\x04V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F^V[\x15\x80a\x18NWP`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\xDE\xAD\x14\x15[\x15a\x19gW`\0\x81\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x82\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x19`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[PPa\x1B\xA0V[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x17\x90U`\0a\x19\xF8\x88a\x9C@Za\x19\xBB\x91\x90a(_V[\x89\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1F|\x92PPPV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U\x90P\x80\x15a\x1A\x8FW`\0\x82\x81R`\xCB` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7FFA\xDFJ\x96 q\xE1'\x19\xD8\xC8\xC8\xE5\xAC\x7F\xC4\xD9{\x92sF\xA3\xD7\xA35\xB1\xF7Q~\x13<\x91\xA2a\x1B\x9CV[`\0\x82\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x1B\x9CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[PPP[PPPPPPPV[`@Q\x7F\xC2\xB3\xE5\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC2\xB3\xE5\xAC\x90\x84\x90a\x1B\xFF\x90\x88\x90\x88\x90\x87\x90`\x04\x01a(vV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1C\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C\xAEW=`\0\x80>=`\0\xFD[PPPPPPPPPV[``\x81`\0\x03a\x1C\xFCWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x1D&W\x80a\x1D\x10\x81a(\xBEV[\x91Pa\x1D\x1F\x90P`\n\x83a(\xF6V[\x91Pa\x1D\0V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DAWa\x1DAa)\nV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1DkW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x12\xC4Wa\x1D\x80`\x01\x83a(_V[\x91Pa\x1D\x8D`\n\x86a)9V[a\x1D\x98\x90`0a)MV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x1D\xADWa\x1D\xADa)eV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x1D\xE7`\n\x86a(\xF6V[\x94Pa\x1DoV[`\0Tu\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x1E\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90UV[`\0a\x1E\xD5\x85\x85\x85\x85a\x1F\x96V[\x80Q\x90` \x01 \x90P\x94\x93PPPPV[`\0a\x1E\xF6\x87\x87\x87\x87\x87\x87a /V[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14\x80a\x1FXWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[\x92\x91PPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[``\x84\x84\x84\x84`@Q`$\x01a\x1F\xAF\x94\x93\x92\x91\x90a)\x94V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xD4\xEC\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x94\x93PPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a L\x96\x95\x94\x93\x92\x91\x90a)\xDEV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\xFCW`\0\x80\xFD[\x91\x90PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80\x83`\x1F\x84\x01\x12a#\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a#\xCBWa#\xCBa\"\x01V[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\xFCW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a#\xFFWa#\xFFa \xCEV[a$\x08\x85a!\xD8V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$'Wa$'a!SV[a$3\x87\x82\x88\x01a\"\x86V[\x90\x94P\x92Pa$F\x90P`@\x86\x01a#\xD2V[\x90P\x92\x95\x91\x94P\x92PV[`\0[\x83\x81\x10\x15a$lW\x81\x81\x01Q\x83\x82\x01R` \x01a$TV[\x83\x81\x11\x15a${W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra$\x99\x81` \x86\x01` \x86\x01a$QV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a$\xDE` \x83\x01\x84a$\x81V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a$\xFAWa$\xFAa \xCEV[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\x19Wa%\x19a \xCEV[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%3Wa%3a!SV[a%?\x86\x82\x87\x01a\"\x86V[\x90\x94P\x92Pa%R\x90P` \x85\x01a#\xD2V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a%yWa%ya \xCEV[\x875\x96Pa%\x89` \x89\x01a!\xD8V[\x95Pa%\x97`@\x89\x01a!\xD8V[\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xC4Wa%\xC4a!SV[a%\xD0\x8A\x82\x8B\x01a\"\x86V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[\x87\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16` \x84\x01R\x80\x88\x16`@\x84\x01RP\x85``\x83\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra&~`\xC0\x83\x01\x84\x86a%\xE3V[\x99\x98PPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a&\xBB`\x80\x83\x01\x86\x88a%\xE3V[\x90P\x83`@\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16``\x83\x01R\x96\x95PPPPPPV[`\0\x84Qa&\xEB\x81\x84` \x89\x01a$QV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa''\x81`\x01\x85\x01` \x8A\x01a$QV[`\x01\x92\x01\x91\x82\x01R\x83Qa'B\x81`\x02\x84\x01` \x88\x01a$QV[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a'\xA5Wa'\xA5a'OV[\x02\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a'\xF8Wa'\xF8a'\xAEV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a('Wa('a'OV[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a(qWa(qa'OV[P\x03\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a(\xB5``\x83\x01\x84a$\x81V[\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a(\xEFWa(\xEFa'OV[P`\x01\x01\x90V[`\0\x82a)\x05Wa)\x05a'\xAEV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82a)HWa)Ha'\xAEV[P\x06\x90V[`\0\x82\x19\x82\x11\x15a)`Wa)`a'OV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra)\xCD`\x80\x83\x01\x85a$\x81V[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra*)`\xC0\x83\x01\x84a$\x81V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L2CROSSDOMAINMESSENGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01DW`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0\xC0W\x80c\xA7\x11\x98i\x11a\0tW\x80c\xB2\x8A\xDE%\x11a\0YW\x80c\xB2\x8A\xDE%\x14a\x0B\x0CW\x80c\xD7d\xAD\x0B\x14a\x0B\xAEW\x80c\xEC\xC7\x04(\x14a\x0B\xC1Wa\x01DV[\x80c\xA7\x11\x98i\x14a\t\xA5W\x80c\xB1\xB1\xB2\t\x14a\nZWa\x01DV[\x80c\x8C\xBE\xEE\xF2\x11a\0\xA5W\x80c\x8C\xBE\xEE\xF2\x14a\x04mW\x80c\x9F\xCE\x81,\x14a\x08-W\x80c\xA4\xE7\xF8\xBD\x14a\x08\xE3Wa\x01DV[\x80c\x81)\xFC\x1C\x14a\x06\xFDW\x80c\x83\xA7@t\x14a\x07\x94Wa\x01DV[\x80c?\x82zZ\x11a\x01\x17W\x80cT\xFDMP\x11a\0\xFCW\x80cT\xFDMP\x14a\x05\x05W\x80cVD\xCF\xDF\x14a\x05\xA9W\x80cn)nE\x14a\x06AWa\x01DV[\x80c?\x82zZ\x14a\x03\xC3W\x80cL\x1Dji\x14a\x04mWa\x01DV[\x80c\x02\x8F\x85\xF7\x14a\x01\xCBW\x80c\x0CV\x84\x98\x14a\x02\x80W\x80c((\xD7\xE8\x14a\x03\x17W\x80c=\xBB +\x14a\x03\xAEW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x02YW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02b`\x10\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02b`?\x81V[4\x80\x15a\x03\xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02b`@\x81V[a\x03\xC1a\x03\xBC6`\x04a#\xE6V[a\x0C\xA8V[\0[4\x80\x15a\x04QW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04Z`\x01\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02wV[4\x80\x15a\x04\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ba\x9C@\x81V[4\x80\x15a\x05\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x05\x9Ca\x0F\x0CV[`@Qa\x02w\x91\x90a$\xCBV[4\x80\x15a\x067W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ba\x13\x88\x81V[4\x80\x15a\x06\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x06\xD8a\x0F\xAFV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02wV[4\x80\x15a\x07\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xC1a\x10\x9BV[4\x80\x15a\x08\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02bb\x03\r@\x81V[4\x80\x15a\x08\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x06\xD8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\tqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\t\x95a\t\x806`\x04a$\xE5V[`\xCE` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02wV[4\x80\x15a\n3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xD8V[4\x80\x15a\n\xE8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\t\x95a\n\xF76`\x04a$\xE5V[`\xCB` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ba\x0B\xA96`\x04a%\x01V[a\x12^V[a\x03\xC1a\x0B\xBC6`\x04a%[V[a\x12\xCCV[4\x80\x15a\x0COW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x0C\x9A`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[`@Q\x90\x81R` \x01a\x02wV[a\r\xE1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C\xD7\x85\x85\x85a\x12^V[4\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\rC`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[3\x8A4\x89\x8C\x8C`@Q`$\x01a\r_\x97\x96\x95\x94\x93\x92\x91\x90a&,V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1B\xA9V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\x0F\x7F\xFDx\xF9\xAE\xE4z$\x8F\xAE\x8D\xB1\x81\xDBn\xEE\x8309\x12>\x02m\xCB\xFFR\x95\"\xE5*3\x85\x85a\x0Ef`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x86`@Qa\x0Ex\x95\x94\x93\x92\x91\x90a&\x8BV[`@Q\x80\x91\x03\x90\xA2`@Q4\x81R3\x90\x7F\x8E\xBB.\xC2F[\xDB*\x06\xA6o\xC3z\tc\xAF\x8A*j\x14y\xD8\x1DV\xFD\xB8\xCB\xB9\x80\x96\xD5F\x90` \x01`@Q\x80\x91\x03\x90\xA2PP`\xCD\x80T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01\x01\x16\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPV[``a\x0F7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1C\xB9V[a\x0F`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1C\xB9V[a\x0F\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1C\xB9V[`@Q` \x01a\x0F\x9B\x93\x92\x91\x90a&\xD9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\xCCT`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!S\x01a\x10~W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FCrossDomainMessenger: xDomainMes`D\x82\x01R\x7FsageSender is not set\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[P`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\0T`\x02\x90u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15\x80\x15a\x10\xE9WP`\0T`\xFF\x80\x83\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04\x16\x10[a\x11uW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x84\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ua\x11\xFDa\x1D\xEEV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0a\x13\x88a\x9C@\x80`?a\x12z`@c\xFF\xFF\xFF\xFF\x88\x16a'~V[a\x12\x84\x91\x90a'\xDDV[a\x12\x8F`\x10\x88a'~V[a\x12\x9C\x90b\x03\r@a(\x04V[a\x12\xA6\x91\x90a(\x04V[a\x12\xB0\x91\x90a(\x04V[a\x12\xBA\x91\x90a(\x04V[a\x12\xC4\x91\x90a(\x04V[\x94\x93PPPPV[`\xF0\x87\x90\x1C`\x02\x81\x10a\x13\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FCrossDomainMessenger: only versi`D\x82\x01R\x7Fon 0 or 1 messages are supported`d\x82\x01R\x7F at this time\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x10uV[\x80a\xFF\xFF\x16`\0\x03a\x14|W`\0a\x13\xD8\x87\x89\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8F\x92Pa\x1E\xC7\x91PPV[`\0\x81\x81R`\xCB` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x14zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FCrossDomainMessenger: legacy wit`D\x82\x01R\x7Fhdrawal already relayed\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[P[`\0a\x14\xC2\x89\x89\x89\x89\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xE6\x92PPPV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEE\xEE\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEE\xEF3\x01\x81\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x03a\x15ZW\x854\x14a\x156Wa\x156a(0V[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16\x15a\x15UWa\x15Ua(0V[a\x16\xACV[4\x15a\x16\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FCrossDomainMessenger: value must`D\x82\x01R\x7F be zero unless message is from `d\x82\x01R\x7Fa system address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x10uV[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16a\x16\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FCrossDomainMessenger: message ca`D\x82\x01R\x7Fnnot be replayed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[a\x16\xB5\x87a\x1F\tV[\x15a\x17hW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FCrossDomainMessenger: cannot sen`D\x82\x01R\x7Fd message to blocked system addr`d\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x10uV[`\0\x81\x81R`\xCB` R`@\x90 T`\xFF\x16\x15a\x18\x07W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FCrossDomainMessenger: message ha`D\x82\x01R\x7Fs already been relayed\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[a\x18(\x85a\x18\x19a\x13\x88a\x9C@a(\x04V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F^V[\x15\x80a\x18NWP`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\xDE\xAD\x14\x15[\x15a\x19gW`\0\x81\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x82\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x19`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[PPa\x1B\xA0V[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x17\x90U`\0a\x19\xF8\x88a\x9C@Za\x19\xBB\x91\x90a(_V[\x89\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1F|\x92PPPV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U\x90P\x80\x15a\x1A\x8FW`\0\x82\x81R`\xCB` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7FFA\xDFJ\x96 q\xE1'\x19\xD8\xC8\xC8\xE5\xAC\x7F\xC4\xD9{\x92sF\xA3\xD7\xA35\xB1\xF7Q~\x13<\x91\xA2a\x1B\x9CV[`\0\x82\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x1B\x9CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[PPP[PPPPPPPV[`@Q\x7F\xC2\xB3\xE5\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC2\xB3\xE5\xAC\x90\x84\x90a\x1B\xFF\x90\x88\x90\x88\x90\x87\x90`\x04\x01a(vV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1C\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1C\xAEW=`\0\x80>=`\0\xFD[PPPPPPPPPV[``\x81`\0\x03a\x1C\xFCWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x1D&W\x80a\x1D\x10\x81a(\xBEV[\x91Pa\x1D\x1F\x90P`\n\x83a(\xF6V[\x91Pa\x1D\0V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DAWa\x1DAa)\nV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1DkW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x12\xC4Wa\x1D\x80`\x01\x83a(_V[\x91Pa\x1D\x8D`\n\x86a)9V[a\x1D\x98\x90`0a)MV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x1D\xADWa\x1D\xADa)eV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x1D\xE7`\n\x86a(\xF6V[\x94Pa\x1DoV[`\0Tu\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x1E\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10uV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90UV[`\0a\x1E\xD5\x85\x85\x85\x85a\x1F\x96V[\x80Q\x90` \x01 \x90P\x94\x93PPPPV[`\0a\x1E\xF6\x87\x87\x87\x87\x87\x87a /V[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14\x80a\x1FXWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[\x92\x91PPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[``\x84\x84\x84\x84`@Q`$\x01a\x1F\xAF\x94\x93\x92\x91\x90a)\x94V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xD4\xEC\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x94\x93PPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a L\x96\x95\x94\x93\x92\x91\x90a)\xDEV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\xFCW`\0\x80\xFD[\x91\x90PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80\x83`\x1F\x84\x01\x12a#\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a#\xCBWa#\xCBa\"\x01V[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\xFCW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a#\xFFWa#\xFFa \xCEV[a$\x08\x85a!\xD8V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$'Wa$'a!SV[a$3\x87\x82\x88\x01a\"\x86V[\x90\x94P\x92Pa$F\x90P`@\x86\x01a#\xD2V[\x90P\x92\x95\x91\x94P\x92PV[`\0[\x83\x81\x10\x15a$lW\x81\x81\x01Q\x83\x82\x01R` \x01a$TV[\x83\x81\x11\x15a${W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra$\x99\x81` \x86\x01` \x86\x01a$QV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a$\xDE` \x83\x01\x84a$\x81V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a$\xFAWa$\xFAa \xCEV[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\x19Wa%\x19a \xCEV[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%3Wa%3a!SV[a%?\x86\x82\x87\x01a\"\x86V[\x90\x94P\x92Pa%R\x90P` \x85\x01a#\xD2V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a%yWa%ya \xCEV[\x875\x96Pa%\x89` \x89\x01a!\xD8V[\x95Pa%\x97`@\x89\x01a!\xD8V[\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xC4Wa%\xC4a!SV[a%\xD0\x8A\x82\x8B\x01a\"\x86V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[\x87\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16` \x84\x01R\x80\x88\x16`@\x84\x01RP\x85``\x83\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra&~`\xC0\x83\x01\x84\x86a%\xE3V[\x99\x98PPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a&\xBB`\x80\x83\x01\x86\x88a%\xE3V[\x90P\x83`@\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16``\x83\x01R\x96\x95PPPPPPV[`\0\x84Qa&\xEB\x81\x84` \x89\x01a$QV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa''\x81`\x01\x85\x01` \x8A\x01a$QV[`\x01\x92\x01\x91\x82\x01R\x83Qa'B\x81`\x02\x84\x01` \x88\x01a$QV[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a'\xA5Wa'\xA5a'OV[\x02\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a'\xF8Wa'\xF8a'\xAEV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a('Wa('a'OV[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a(qWa(qa'OV[P\x03\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a(\xB5``\x83\x01\x84a$\x81V[\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a(\xEFWa(\xEFa'OV[P`\x01\x01\x90V[`\0\x82a)\x05Wa)\x05a'\xAEV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82a)HWa)Ha'\xAEV[P\x06\x90V[`\0\x82\x19\x82\x11\x15a)`Wa)`a'OV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra)\xCD`\x80\x83\x01\x85a$\x81V[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra*)`\xC0\x83\x01\x84a$\x81V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L2CROSSDOMAINMESSENGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L2CrossDomainMessenger<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L2CrossDomainMessenger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L2CrossDomainMessenger<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L2CrossDomainMessenger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L2CrossDomainMessenger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L2CrossDomainMessenger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L2CrossDomainMessenger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L2CROSSDOMAINMESSENGER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                L2CROSSDOMAINMESSENGER_ABI.clone(),
                L2CROSSDOMAINMESSENGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1CrossDomainMessenger` (0xa7119869) function
        pub fn l_1_cross_domain_messenger(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([167, 17, 152, 105], ())
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
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
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
            L2CrossDomainMessengerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L2CrossDomainMessenger<M> {
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
    pub enum L2CrossDomainMessengerEvents {
        FailedRelayedMessageFilter(FailedRelayedMessageFilter),
        InitializedFilter(InitializedFilter),
        RelayedMessageFilter(RelayedMessageFilter),
        SentMessageFilter(SentMessageFilter),
        SentMessageExtension1Filter(SentMessageExtension1Filter),
    }
    impl ::ethers::contract::EthLogDecode for L2CrossDomainMessengerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FailedRelayedMessageFilter::decode_log(log) {
                return Ok(
                    L2CrossDomainMessengerEvents::FailedRelayedMessageFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(L2CrossDomainMessengerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = RelayedMessageFilter::decode_log(log) {
                return Ok(L2CrossDomainMessengerEvents::RelayedMessageFilter(decoded));
            }
            if let Ok(decoded) = SentMessageFilter::decode_log(log) {
                return Ok(L2CrossDomainMessengerEvents::SentMessageFilter(decoded));
            }
            if let Ok(decoded) = SentMessageExtension1Filter::decode_log(log) {
                return Ok(
                    L2CrossDomainMessengerEvents::SentMessageExtension1Filter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L2CrossDomainMessengerEvents {
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
    for L2CrossDomainMessengerEvents {
        fn from(value: FailedRelayedMessageFilter) -> Self {
            Self::FailedRelayedMessageFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for L2CrossDomainMessengerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<RelayedMessageFilter> for L2CrossDomainMessengerEvents {
        fn from(value: RelayedMessageFilter) -> Self {
            Self::RelayedMessageFilter(value)
        }
    }
    impl ::core::convert::From<SentMessageFilter> for L2CrossDomainMessengerEvents {
        fn from(value: SentMessageFilter) -> Self {
            Self::SentMessageFilter(value)
        }
    }
    impl ::core::convert::From<SentMessageExtension1Filter>
    for L2CrossDomainMessengerEvents {
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
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
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `l1CrossDomainMessenger` function with signature `l1CrossDomainMessenger()` and selector `0xa7119869`
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
    #[ethcall(name = "l1CrossDomainMessenger", abi = "l1CrossDomainMessenger()")]
    pub struct L1CrossDomainMessengerCall;
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
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
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
    pub enum L2CrossDomainMessengerCalls {
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
        Initialize(InitializeCall),
        L1CrossDomainMessenger(L1CrossDomainMessengerCall),
        MessageNonce(MessageNonceCall),
        RelayMessage(RelayMessageCall),
        SendMessage(SendMessageCall),
        SuccessfulMessages(SuccessfulMessagesCall),
        Version(VersionCall),
        XdomainMessageSender(XdomainMessageSenderCall),
    }
    impl ::ethers::core::abi::AbiDecode for L2CrossDomainMessengerCalls {
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
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <L1CrossDomainMessengerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1CrossDomainMessenger(decoded));
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
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <XdomainMessageSenderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::XdomainMessageSender(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L2CrossDomainMessengerCalls {
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
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1CrossDomainMessenger(element) => {
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
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::XdomainMessageSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for L2CrossDomainMessengerCalls {
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
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1CrossDomainMessenger(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MessageNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SuccessfulMessages(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::XdomainMessageSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MessageVersionCall> for L2CrossDomainMessengerCalls {
        fn from(value: MessageVersionCall) -> Self {
            Self::MessageVersion(value)
        }
    }
    impl ::core::convert::From<MinGasCalldataOverheadCall>
    for L2CrossDomainMessengerCalls {
        fn from(value: MinGasCalldataOverheadCall) -> Self {
            Self::MinGasCalldataOverhead(value)
        }
    }
    impl ::core::convert::From<MinGasDynamicOverheadDenominatorCall>
    for L2CrossDomainMessengerCalls {
        fn from(value: MinGasDynamicOverheadDenominatorCall) -> Self {
            Self::MinGasDynamicOverheadDenominator(value)
        }
    }
    impl ::core::convert::From<MinGasDynamicOverheadNumeratorCall>
    for L2CrossDomainMessengerCalls {
        fn from(value: MinGasDynamicOverheadNumeratorCall) -> Self {
            Self::MinGasDynamicOverheadNumerator(value)
        }
    }
    impl ::core::convert::From<OtherMessengerCall> for L2CrossDomainMessengerCalls {
        fn from(value: OtherMessengerCall) -> Self {
            Self::OtherMessenger(value)
        }
    }
    impl ::core::convert::From<RelayCallOverheadCall> for L2CrossDomainMessengerCalls {
        fn from(value: RelayCallOverheadCall) -> Self {
            Self::RelayCallOverhead(value)
        }
    }
    impl ::core::convert::From<RelayConstantOverheadCall>
    for L2CrossDomainMessengerCalls {
        fn from(value: RelayConstantOverheadCall) -> Self {
            Self::RelayConstantOverhead(value)
        }
    }
    impl ::core::convert::From<RelayGasCheckBufferCall> for L2CrossDomainMessengerCalls {
        fn from(value: RelayGasCheckBufferCall) -> Self {
            Self::RelayGasCheckBuffer(value)
        }
    }
    impl ::core::convert::From<RelayReservedGasCall> for L2CrossDomainMessengerCalls {
        fn from(value: RelayReservedGasCall) -> Self {
            Self::RelayReservedGas(value)
        }
    }
    impl ::core::convert::From<BaseGasCall> for L2CrossDomainMessengerCalls {
        fn from(value: BaseGasCall) -> Self {
            Self::BaseGas(value)
        }
    }
    impl ::core::convert::From<FailedMessagesCall> for L2CrossDomainMessengerCalls {
        fn from(value: FailedMessagesCall) -> Self {
            Self::FailedMessages(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for L2CrossDomainMessengerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<L1CrossDomainMessengerCall>
    for L2CrossDomainMessengerCalls {
        fn from(value: L1CrossDomainMessengerCall) -> Self {
            Self::L1CrossDomainMessenger(value)
        }
    }
    impl ::core::convert::From<MessageNonceCall> for L2CrossDomainMessengerCalls {
        fn from(value: MessageNonceCall) -> Self {
            Self::MessageNonce(value)
        }
    }
    impl ::core::convert::From<RelayMessageCall> for L2CrossDomainMessengerCalls {
        fn from(value: RelayMessageCall) -> Self {
            Self::RelayMessage(value)
        }
    }
    impl ::core::convert::From<SendMessageCall> for L2CrossDomainMessengerCalls {
        fn from(value: SendMessageCall) -> Self {
            Self::SendMessage(value)
        }
    }
    impl ::core::convert::From<SuccessfulMessagesCall> for L2CrossDomainMessengerCalls {
        fn from(value: SuccessfulMessagesCall) -> Self {
            Self::SuccessfulMessages(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L2CrossDomainMessengerCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<XdomainMessageSenderCall>
    for L2CrossDomainMessengerCalls {
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
    ///Container type for all return fields from the `l1CrossDomainMessenger` function with signature `l1CrossDomainMessenger()` and selector `0xa7119869`
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
    pub struct L1CrossDomainMessengerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::std::string::String);
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

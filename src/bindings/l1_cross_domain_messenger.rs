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
pub mod l1_cross_domain_messenger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("PORTAL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PORTAL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract OptimismPortal"),
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
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_portal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract OptimismPortal"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("portal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("portal"),
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
    pub static L1CROSSDOMAINMESSENGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[PsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07`\x80R`\x01`\xA0\x81\x90R`\x05`\xC0R`\xE0Rb\0\0\x93`\0b\0\0\x99V[b\0\x02GV[`\0T`\x02\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15\x80\x15b\0\0\xC5WP`\0T`\xFF\x80\x83\x16`\x01`\xA0\x1B\x90\x92\x04\x16\x10[b\0\x01.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\xA8\x1Ba\xFF\xFF`\xA0\x1B\x19\x90\x91\x16`\x01`\xA0\x1B`\xFF\x85\x16\x02`\xFF`\xA8\x1B\x19\x16\x17\x17\x90U`\xF9\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ub\0\x01{b\0\x01\xC2V[`\0\x80T`\xFF`\xA8\x1B\x19\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\0T`\x01`\xA8\x1B\x90\x04`\xFF\x16b\0\x021W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01%V[`\xCC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16a\xDE\xAD\x17\x90UV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa,\xD9b\0\x02\x8F`\09`\0a\x102\x01R`\0a\x10\t\x01R`\0a\x0F\xE0\x01R`\0\x81\x81a\t\xA1\x01R\x81\x81a\rz\x01Ra 9\x01Ra,\xD9`\0\xF3\xFE`\x80`@R`\x046\x10a\x01_W`\x005`\xE0\x1C\x80cn)nE\x11a\0\xC0W\x80c\xB1\xB1\xB2\t\x11a\0tW\x80c\xC4\xD6m\xE8\x11a\0YW\x80c\xC4\xD6m\xE8\x14a\x0B\xD9W\x80c\xD7d\xAD\x0B\x14a\x0C{W\x80c\xEC\xC7\x04(\x14a\x0C\x8EWa\x01_V[\x80c\xB1\xB1\xB2\t\x14a\n\x85W\x80c\xB2\x8A\xDE%\x14a\x0B7Wa\x01_V[\x80c\x8C\xBE\xEE\xF2\x11a\0\xA5W\x80c\x8C\xBE\xEE\xF2\x14a\x05\\W\x80c\x9F\xCE\x81,\x14a\t\rW\x80c\xA4\xE7\xF8\xBD\x14a\t\xC3Wa\x01_V[\x80cn)nE\x14a\x07\xDDW\x80c\x83\xA7@t\x14a\x08tWa\x01_V[\x80c?\x82zZ\x11a\x01\x17W\x80cT\xFDMP\x11a\0\xFCW\x80cT\xFDMP\x14a\x05\xF4W\x80cVD\xCF\xDF\x14a\x06\x98W\x80cd%fk\x14a\x070Wa\x01_V[\x80c?\x82zZ\x14a\x04\xB2W\x80cL\x1Dji\x14a\x05\\Wa\x01_V[\x80c\x0F\xF7T\xEA\x11a\x01HW\x80c\x0F\xF7T\xEA\x14a\x032W\x80c((\xD7\xE8\x14a\x04\x06W\x80c=\xBB +\x14a\x04\x9DWa\x01_V[\x80c\x02\x8F\x85\xF7\x14a\x01\xE6W\x80c\x0CV\x84\x98\x14a\x02\x9BW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x02tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}`\x10\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03)W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}`?\x81V[4\x80\x15a\x03\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\xF9Ta\x03\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x92V[4\x80\x15a\x04\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}`@\x81V[a\x04\xB0a\x04\xAB6`\x04a&'V[a\ruV[\0[4\x80\x15a\x05@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x05I`\x01\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x92V[4\x80\x15a\x05\xEAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}a\x9C@\x81V[4\x80\x15a\x06\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x06\x8Ba\x0F\xD9V[`@Qa\x02\x92\x91\x90a'\x0EV[4\x80\x15a\x07&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}a\x13\x88\x81V[4\x80\x15a\x07\xBEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\xF9Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xE1V[4\x80\x15a\x08kW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xE1a\x10|V[4\x80\x15a\t\x02W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}b\x03\r@\x81V[4\x80\x15a\t\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xE1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\nQW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\nua\n`6`\x04a'(V[`\xCE` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x92V[4\x80\x15a\x0B\x13W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\nua\x0B\"6`\x04a'(V[`\xCB` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}a\x0B\xD46`\x04a'DV[a\x11hV[4\x80\x15a\x0CgW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04\xB0a\x0Cv6`\x04a'\x9EV[a\x11\xD6V[a\x04\xB0a\x0C\x896`\x04a'\xBEV[a\x13\xDAV[4\x80\x15a\r\x1CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\rg`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[`@Q\x90\x81R` \x01a\x02\x92V[a\x0E\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xA4\x85\x85\x85a\x11hV[4\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0E\x10`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[3\x8A4\x89\x8C\x8C`@Q`$\x01a\x0E,\x97\x96\x95\x94\x93\x92\x91\x90a(\x93V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1CfV[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\x0F\x7F\xFDx\xF9\xAE\xE4z$\x8F\xAE\x8D\xB1\x81\xDBn\xEE\x8309\x12>\x02m\xCB\xFFR\x95\"\xE5*3\x85\x85a\x0F3`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x86`@Qa\x0FE\x95\x94\x93\x92\x91\x90a(\xF2V[`@Q\x80\x91\x03\x90\xA2`@Q4\x81R3\x90\x7F\x8E\xBB.\xC2F[\xDB*\x06\xA6o\xC3z\tc\xAF\x8A*j\x14y\xD8\x1DV\xFD\xB8\xCB\xB9\x80\x96\xD5F\x90` \x01`@Q\x80\x91\x03\x90\xA2PP`\xCD\x80T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01\x01\x16\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPV[``a\x10\x04\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x81V[a\x10-\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x81V[a\x10V\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x81V[`@Q` \x01a\x10h\x93\x92\x91\x90a)@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\xCCT`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!S\x01a\x11KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FCrossDomainMessenger: xDomainMes`D\x82\x01R\x7FsageSender is not set\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[P`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\0a\x13\x88a\x9C@\x80`?a\x11\x84`@c\xFF\xFF\xFF\xFF\x88\x16a)\xE5V[a\x11\x8E\x91\x90a*DV[a\x11\x99`\x10\x88a)\xE5V[a\x11\xA6\x90b\x03\r@a*kV[a\x11\xB0\x91\x90a*kV[a\x11\xBA\x91\x90a*kV[a\x11\xC4\x91\x90a*kV[a\x11\xCE\x91\x90a*kV[\x94\x93PPPPV[`\0T`\x02\x90u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15\x80\x15a\x12$WP`\0T`\xFF\x80\x83\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04\x16\x10[a\x12\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[`\0\x80Tu\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x85\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x17\x90U`\xF9\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x17\x90Ua\x13xa\x1E\xB6V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\xF0\x87\x90\x1C`\x02\x81\x10a\x14\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FCrossDomainMessenger: only versi`D\x82\x01R\x7Fon 0 or 1 messages are supported`d\x82\x01R\x7F at this time\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11BV[\x80a\xFF\xFF\x16`\0\x03a\x15\x8AW`\0a\x14\xE6\x87\x89\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8F\x92Pa\x1F\x8F\x91PPV[`\0\x81\x81R`\xCB` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x15\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FCrossDomainMessenger: legacy wit`D\x82\x01R\x7Fhdrawal already relayed\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[P[`\0a\x15\xD0\x89\x89\x89\x89\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1F\xAE\x92PPPV[\x90Pa\x15\xDAa\x1F\xD1V[\x15a\x16\x12W\x854\x14a\x15\xEEWa\x15\xEEa*\x97V[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16\x15a\x16\rWa\x16\ra*\x97V[a\x17dV[4\x15a\x16\xC6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FCrossDomainMessenger: value must`D\x82\x01R\x7F be zero unless message is from `d\x82\x01R\x7Fa system address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11BV[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16a\x17dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FCrossDomainMessenger: message ca`D\x82\x01R\x7Fnnot be replayed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[a\x17m\x87a!XV[\x15a\x18 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FCrossDomainMessenger: cannot sen`D\x82\x01R\x7Fd message to blocked system addr`d\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11BV[`\0\x81\x81R`\xCB` R`@\x90 T`\xFF\x16\x15a\x18\xBFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FCrossDomainMessenger: message ha`D\x82\x01R\x7Fs already been relayed\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[a\x18\xE0\x85a\x18\xD1a\x13\x88a\x9C@a*kV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!\x9EV[\x15\x80a\x19\x06WP`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\xDE\xAD\x14\x15[\x15a\x1A\x1FW`\0\x81\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x82\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x1A\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[PPa\x1CXV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x17\x90U`\0a\x1A\xB0\x88a\x9C@Za\x1As\x91\x90a*\xC6V[\x89\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa!\xBC\x92PPPV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U\x90P\x80\x15a\x1BGW`\0\x82\x81R`\xCB` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7FFA\xDFJ\x96 q\xE1'\x19\xD8\xC8\xC8\xE5\xAC\x7F\xC4\xD9{\x92sF\xA3\xD7\xA35\xB1\xF7Q~\x13<\x91\xA2a\x1CTV[`\0\x82\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x1CTW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[PPP[PPPPPPPV[\x90P\x90V[`\xF9T`@Q\x7F\xE9\xE0\\B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xE9\xE0\\B\x90\x84\x90a\x1C\xC7\x90\x88\x90\x83\x90\x89\x90`\0\x90\x89\x90`\x04\x01a*\xDDV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1DbW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1DvW=`\0\x80>=`\0\xFD[PPPPPPPPPV[``\x81`\0\x03a\x1D\xC4WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x1D\xEEW\x80a\x1D\xD8\x81a+5V[\x91Pa\x1D\xE7\x90P`\n\x83a+mV[\x91Pa\x1D\xC8V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\tWa\x1E\ta+\x81V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1E3W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x11\xCEWa\x1EH`\x01\x83a*\xC6V[\x91Pa\x1EU`\n\x86a+\xB0V[a\x1E`\x90`0a+\xC4V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x1EuWa\x1Eua+\xDCV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x1E\xAF`\n\x86a+mV[\x94Pa\x1E7V[`\0Tu\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x1FaW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90UV[`\0a\x1F\x9D\x85\x85\x85\x85a!\xD6V[\x80Q\x90` \x01 \x90P\x94\x93PPPPV[`\0a\x1F\xBE\x87\x87\x87\x87\x87\x87a\"oV[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[`\xF9T`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\x1CaWP`\xF9T`@\x80Q\x7F\x9B\xF6-\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x16\x91c\x9B\xF6-\x82\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a!\x04W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a!\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!<\x91\x90a,\x0BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x90P\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14\x80a!\x98WP`\xF9Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x91\x16\x14[\x92\x91PPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[``\x84\x84\x84\x84`@Q`$\x01a!\xEF\x94\x93\x92\x91\x90a,+V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xD4\xEC\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x94\x93PPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a\"\x8C\x96\x95\x94\x93\x92\x91\x90a,uV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$:W`\0\x80\xFD[PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80\x83`\x1F\x84\x01\x12a%TW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xECW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a&\x07Wa&\x07a$=V[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a&\"W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a&@Wa&@a#\x0EV[\x845a&K\x81a$\x18V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&jWa&ja#\x93V[a&v\x87\x82\x88\x01a$\xC2V[\x90\x94P\x92Pa&\x89\x90P`@\x86\x01a&\x0EV[\x90P\x92\x95\x91\x94P\x92PV[`\0[\x83\x81\x10\x15a&\xAFW\x81\x81\x01Q\x83\x82\x01R` \x01a&\x97V[\x83\x81\x11\x15a&\xBEW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra&\xDC\x81` \x86\x01` \x86\x01a&\x94V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a'!` \x83\x01\x84a&\xC4V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'=Wa'=a#\x0EV[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a'\\Wa'\\a#\x0EV[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'vWa'va#\x93V[a'\x82\x86\x82\x87\x01a$\xC2V[\x90\x94P\x92Pa'\x95\x90P` \x85\x01a&\x0EV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a'\xB3Wa'\xB3a#\x0EV[\x815a'!\x81a$\x18V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a'\xDCWa'\xDCa#\x0EV[\x875\x96P` \x88\x015a'\xEE\x81a$\x18V[\x95P`@\x88\x015a'\xFE\x81a$\x18V[\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(+Wa(+a#\x93V[a(7\x8A\x82\x8B\x01a$\xC2V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[\x87\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16` \x84\x01R\x80\x88\x16`@\x84\x01RP\x85``\x83\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra(\xE5`\xC0\x83\x01\x84\x86a(JV[\x99\x98PPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a)\"`\x80\x83\x01\x86\x88a(JV[\x90P\x83`@\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16``\x83\x01R\x96\x95PPPPPPV[`\0\x84Qa)R\x81\x84` \x89\x01a&\x94V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa)\x8E\x81`\x01\x85\x01` \x8A\x01a&\x94V[`\x01\x92\x01\x91\x82\x01R\x83Qa)\xA9\x81`\x02\x84\x01` \x88\x01a&\x94V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a*\x0CWa*\x0Ca)\xB6V[\x02\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a*_Wa*_a*\x15V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a*\x8EWa*\x8Ea)\xB6V[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a*\xD8Wa*\xD8a)\xB6V[P\x03\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R\x84` \x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R\x82\x15\x15``\x82\x01R`\xA0`\x80\x82\x01R`\0a+*`\xA0\x83\x01\x84a&\xC4V[\x97\x96PPPPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a+fWa+fa)\xB6V[P`\x01\x01\x90V[`\0\x82a+|Wa+|a*\x15V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82a+\xBFWa+\xBFa*\x15V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a+\xD7Wa+\xD7a)\xB6V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a, Wa, a#\x0EV[\x81Qa'!\x81a$\x18V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra,d`\x80\x83\x01\x85a&\xC4V[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra,\xC0`\xC0\x83\x01\x84a&\xC4V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L1CROSSDOMAINMESSENGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01_W`\x005`\xE0\x1C\x80cn)nE\x11a\0\xC0W\x80c\xB1\xB1\xB2\t\x11a\0tW\x80c\xC4\xD6m\xE8\x11a\0YW\x80c\xC4\xD6m\xE8\x14a\x0B\xD9W\x80c\xD7d\xAD\x0B\x14a\x0C{W\x80c\xEC\xC7\x04(\x14a\x0C\x8EWa\x01_V[\x80c\xB1\xB1\xB2\t\x14a\n\x85W\x80c\xB2\x8A\xDE%\x14a\x0B7Wa\x01_V[\x80c\x8C\xBE\xEE\xF2\x11a\0\xA5W\x80c\x8C\xBE\xEE\xF2\x14a\x05\\W\x80c\x9F\xCE\x81,\x14a\t\rW\x80c\xA4\xE7\xF8\xBD\x14a\t\xC3Wa\x01_V[\x80cn)nE\x14a\x07\xDDW\x80c\x83\xA7@t\x14a\x08tWa\x01_V[\x80c?\x82zZ\x11a\x01\x17W\x80cT\xFDMP\x11a\0\xFCW\x80cT\xFDMP\x14a\x05\xF4W\x80cVD\xCF\xDF\x14a\x06\x98W\x80cd%fk\x14a\x070Wa\x01_V[\x80c?\x82zZ\x14a\x04\xB2W\x80cL\x1Dji\x14a\x05\\Wa\x01_V[\x80c\x0F\xF7T\xEA\x11a\x01HW\x80c\x0F\xF7T\xEA\x14a\x032W\x80c((\xD7\xE8\x14a\x04\x06W\x80c=\xBB +\x14a\x04\x9DWa\x01_V[\x80c\x02\x8F\x85\xF7\x14a\x01\xE6W\x80c\x0CV\x84\x98\x14a\x02\x9BW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x02tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}`\x10\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03)W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}`?\x81V[4\x80\x15a\x03\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\xF9Ta\x03\xE1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x92V[4\x80\x15a\x04\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}`@\x81V[a\x04\xB0a\x04\xAB6`\x04a&'V[a\ruV[\0[4\x80\x15a\x05@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x05I`\x01\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x92V[4\x80\x15a\x05\xEAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}a\x9C@\x81V[4\x80\x15a\x06\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x06\x8Ba\x0F\xD9V[`@Qa\x02\x92\x91\x90a'\x0EV[4\x80\x15a\x07&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}a\x13\x88\x81V[4\x80\x15a\x07\xBEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\xF9Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xE1V[4\x80\x15a\x08kW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xE1a\x10|V[4\x80\x15a\t\x02W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}b\x03\r@\x81V[4\x80\x15a\t\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xE1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\nQW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\nua\n`6`\x04a'(V[`\xCE` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x92V[4\x80\x15a\x0B\x13W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\nua\x0B\"6`\x04a'(V[`\xCB` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x0B\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02}a\x0B\xD46`\x04a'DV[a\x11hV[4\x80\x15a\x0CgW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04\xB0a\x0Cv6`\x04a'\x9EV[a\x11\xD6V[a\x04\xB0a\x0C\x896`\x04a'\xBEV[a\x13\xDAV[4\x80\x15a\r\x1CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\rg`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[`@Q\x90\x81R` \x01a\x02\x92V[a\x0E\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xA4\x85\x85\x85a\x11hV[4\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0E\x10`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[3\x8A4\x89\x8C\x8C`@Q`$\x01a\x0E,\x97\x96\x95\x94\x93\x92\x91\x90a(\x93V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1CfV[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\x0F\x7F\xFDx\xF9\xAE\xE4z$\x8F\xAE\x8D\xB1\x81\xDBn\xEE\x8309\x12>\x02m\xCB\xFFR\x95\"\xE5*3\x85\x85a\x0F3`\xCDT}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x86`@Qa\x0FE\x95\x94\x93\x92\x91\x90a(\xF2V[`@Q\x80\x91\x03\x90\xA2`@Q4\x81R3\x90\x7F\x8E\xBB.\xC2F[\xDB*\x06\xA6o\xC3z\tc\xAF\x8A*j\x14y\xD8\x1DV\xFD\xB8\xCB\xB9\x80\x96\xD5F\x90` \x01`@Q\x80\x91\x03\x90\xA2PP`\xCD\x80T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01\x01\x16\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPV[``a\x10\x04\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x81V[a\x10-\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x81V[a\x10V\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x81V[`@Q` \x01a\x10h\x93\x92\x91\x90a)@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\xCCT`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!S\x01a\x11KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FCrossDomainMessenger: xDomainMes`D\x82\x01R\x7FsageSender is not set\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[P`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\0a\x13\x88a\x9C@\x80`?a\x11\x84`@c\xFF\xFF\xFF\xFF\x88\x16a)\xE5V[a\x11\x8E\x91\x90a*DV[a\x11\x99`\x10\x88a)\xE5V[a\x11\xA6\x90b\x03\r@a*kV[a\x11\xB0\x91\x90a*kV[a\x11\xBA\x91\x90a*kV[a\x11\xC4\x91\x90a*kV[a\x11\xCE\x91\x90a*kV[\x94\x93PPPPV[`\0T`\x02\x90u\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15\x80\x15a\x12$WP`\0T`\xFF\x80\x83\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04\x16\x10[a\x12\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[`\0\x80Tu\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x85\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x17\x90U`\xF9\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x17\x90Ua\x13xa\x1E\xB6V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\xF0\x87\x90\x1C`\x02\x81\x10a\x14\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FCrossDomainMessenger: only versi`D\x82\x01R\x7Fon 0 or 1 messages are supported`d\x82\x01R\x7F at this time\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11BV[\x80a\xFF\xFF\x16`\0\x03a\x15\x8AW`\0a\x14\xE6\x87\x89\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8F\x92Pa\x1F\x8F\x91PPV[`\0\x81\x81R`\xCB` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x15\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FCrossDomainMessenger: legacy wit`D\x82\x01R\x7Fhdrawal already relayed\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[P[`\0a\x15\xD0\x89\x89\x89\x89\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1F\xAE\x92PPPV[\x90Pa\x15\xDAa\x1F\xD1V[\x15a\x16\x12W\x854\x14a\x15\xEEWa\x15\xEEa*\x97V[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16\x15a\x16\rWa\x16\ra*\x97V[a\x17dV[4\x15a\x16\xC6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FCrossDomainMessenger: value must`D\x82\x01R\x7F be zero unless message is from `d\x82\x01R\x7Fa system address\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11BV[`\0\x81\x81R`\xCE` R`@\x90 T`\xFF\x16a\x17dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FCrossDomainMessenger: message ca`D\x82\x01R\x7Fnnot be replayed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[a\x17m\x87a!XV[\x15a\x18 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FCrossDomainMessenger: cannot sen`D\x82\x01R\x7Fd message to blocked system addr`d\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11BV[`\0\x81\x81R`\xCB` R`@\x90 T`\xFF\x16\x15a\x18\xBFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FCrossDomainMessenger: message ha`D\x82\x01R\x7Fs already been relayed\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[a\x18\xE0\x85a\x18\xD1a\x13\x88a\x9C@a*kV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!\x9EV[\x15\x80a\x19\x06WP`\xCCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\xDE\xAD\x14\x15[\x15a\x1A\x1FW`\0\x81\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x82\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x1A\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[PPa\x1CXV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x17\x90U`\0a\x1A\xB0\x88a\x9C@Za\x1As\x91\x90a*\xC6V[\x89\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa!\xBC\x92PPPV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90U\x90P\x80\x15a\x1BGW`\0\x82\x81R`\xCB` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7FFA\xDFJ\x96 q\xE1'\x19\xD8\xC8\xC8\xE5\xAC\x7F\xC4\xD9{\x92sF\xA3\xD7\xA35\xB1\xF7Q~\x13<\x91\xA2a\x1CTV[`\0\x82\x81R`\xCE` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x83\x91\x7F\x99\xD0\xE0HHK\xAA\x1B\x15@\xB16|\xB1(\xAC\xD7\xAB)F\xD1\xED\x91\xEC\x10\xE3\xC8^K\xF5\x1B\x8F\x91\xA2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF2\x01a\x1CTW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FCrossDomainMessenger: failed to `D\x82\x01R\x7Frelay message\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[PPP[PPPPPPPV[\x90P\x90V[`\xF9T`@Q\x7F\xE9\xE0\\B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xE9\xE0\\B\x90\x84\x90a\x1C\xC7\x90\x88\x90\x83\x90\x89\x90`\0\x90\x89\x90`\x04\x01a*\xDDV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1DbW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1DvW=`\0\x80>=`\0\xFD[PPPPPPPPPV[``\x81`\0\x03a\x1D\xC4WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x1D\xEEW\x80a\x1D\xD8\x81a+5V[\x91Pa\x1D\xE7\x90P`\n\x83a+mV[\x91Pa\x1D\xC8V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\tWa\x1E\ta+\x81V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1E3W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x11\xCEWa\x1EH`\x01\x83a*\xC6V[\x91Pa\x1EU`\n\x86a+\xB0V[a\x1E`\x90`0a+\xC4V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x1EuWa\x1Eua+\xDCV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x1E\xAF`\n\x86a+mV[\x94Pa\x1E7V[`\0Tu\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x1FaW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x11BV[`\xCC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\xDE\xAD\x17\x90UV[`\0a\x1F\x9D\x85\x85\x85\x85a!\xD6V[\x80Q\x90` \x01 \x90P\x94\x93PPPPV[`\0a\x1F\xBE\x87\x87\x87\x87\x87\x87a\"oV[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[`\xF9T`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\x1CaWP`\xF9T`@\x80Q\x7F\x9B\xF6-\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x16\x91c\x9B\xF6-\x82\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a!\x04W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a!\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!<\x91\x90a,\x0BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x90P\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14\x80a!\x98WP`\xF9Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x91\x16\x14[\x92\x91PPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[``\x84\x84\x84\x84`@Q`$\x01a!\xEF\x94\x93\x92\x91\x90a,+V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xD4\xEC\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x94\x93PPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a\"\x8C\x96\x95\x94\x93\x92\x91\x90a,uV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$:W`\0\x80\xFD[PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80\x83`\x1F\x84\x01\x12a%TW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xECW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a&\x07Wa&\x07a$=V[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a&\"W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a&@Wa&@a#\x0EV[\x845a&K\x81a$\x18V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&jWa&ja#\x93V[a&v\x87\x82\x88\x01a$\xC2V[\x90\x94P\x92Pa&\x89\x90P`@\x86\x01a&\x0EV[\x90P\x92\x95\x91\x94P\x92PV[`\0[\x83\x81\x10\x15a&\xAFW\x81\x81\x01Q\x83\x82\x01R` \x01a&\x97V[\x83\x81\x11\x15a&\xBEW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra&\xDC\x81` \x86\x01` \x86\x01a&\x94V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a'!` \x83\x01\x84a&\xC4V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'=Wa'=a#\x0EV[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a'\\Wa'\\a#\x0EV[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'vWa'va#\x93V[a'\x82\x86\x82\x87\x01a$\xC2V[\x90\x94P\x92Pa'\x95\x90P` \x85\x01a&\x0EV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a'\xB3Wa'\xB3a#\x0EV[\x815a'!\x81a$\x18V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a'\xDCWa'\xDCa#\x0EV[\x875\x96P` \x88\x015a'\xEE\x81a$\x18V[\x95P`@\x88\x015a'\xFE\x81a$\x18V[\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(+Wa(+a#\x93V[a(7\x8A\x82\x8B\x01a$\xC2V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[\x87\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16` \x84\x01R\x80\x88\x16`@\x84\x01RP\x85``\x83\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra(\xE5`\xC0\x83\x01\x84\x86a(JV[\x99\x98PPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\x80` \x82\x01R`\0a)\"`\x80\x83\x01\x86\x88a(JV[\x90P\x83`@\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16``\x83\x01R\x96\x95PPPPPPV[`\0\x84Qa)R\x81\x84` \x89\x01a&\x94V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa)\x8E\x81`\x01\x85\x01` \x8A\x01a&\x94V[`\x01\x92\x01\x91\x82\x01R\x83Qa)\xA9\x81`\x02\x84\x01` \x88\x01a&\x94V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a*\x0CWa*\x0Ca)\xB6V[\x02\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a*_Wa*_a*\x15V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a*\x8EWa*\x8Ea)\xB6V[\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a*\xD8Wa*\xD8a)\xB6V[P\x03\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R\x84` \x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R\x82\x15\x15``\x82\x01R`\xA0`\x80\x82\x01R`\0a+*`\xA0\x83\x01\x84a&\xC4V[\x97\x96PPPPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a+fWa+fa)\xB6V[P`\x01\x01\x90V[`\0\x82a+|Wa+|a*\x15V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82a+\xBFWa+\xBFa*\x15V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a+\xD7Wa+\xD7a)\xB6V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a, Wa, a#\x0EV[\x81Qa'!\x81a$\x18V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra,d`\x80\x83\x01\x85a&\xC4V[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra,\xC0`\xC0\x83\x01\x84a&\xC4V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L1CROSSDOMAINMESSENGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L1CrossDomainMessenger<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L1CrossDomainMessenger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L1CrossDomainMessenger<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L1CrossDomainMessenger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L1CrossDomainMessenger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L1CrossDomainMessenger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L1CrossDomainMessenger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L1CROSSDOMAINMESSENGER_ABI.clone(),
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
                L1CROSSDOMAINMESSENGER_ABI.clone(),
                L1CROSSDOMAINMESSENGER_BYTECODE.clone().into(),
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
        ///Calls the contract's `PORTAL` (0x0ff754ea) function
        pub fn PORTAL(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([15, 247, 84, 234], ())
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
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            portal: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], portal)
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
        ///Calls the contract's `portal` (0x6425666b) function
        pub fn portal(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([100, 37, 102, 107], ())
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
            L1CrossDomainMessengerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L1CrossDomainMessenger<M> {
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
    pub enum L1CrossDomainMessengerEvents {
        FailedRelayedMessageFilter(FailedRelayedMessageFilter),
        InitializedFilter(InitializedFilter),
        RelayedMessageFilter(RelayedMessageFilter),
        SentMessageFilter(SentMessageFilter),
        SentMessageExtension1Filter(SentMessageExtension1Filter),
    }
    impl ::ethers::contract::EthLogDecode for L1CrossDomainMessengerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FailedRelayedMessageFilter::decode_log(log) {
                return Ok(
                    L1CrossDomainMessengerEvents::FailedRelayedMessageFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(L1CrossDomainMessengerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = RelayedMessageFilter::decode_log(log) {
                return Ok(L1CrossDomainMessengerEvents::RelayedMessageFilter(decoded));
            }
            if let Ok(decoded) = SentMessageFilter::decode_log(log) {
                return Ok(L1CrossDomainMessengerEvents::SentMessageFilter(decoded));
            }
            if let Ok(decoded) = SentMessageExtension1Filter::decode_log(log) {
                return Ok(
                    L1CrossDomainMessengerEvents::SentMessageExtension1Filter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L1CrossDomainMessengerEvents {
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
    for L1CrossDomainMessengerEvents {
        fn from(value: FailedRelayedMessageFilter) -> Self {
            Self::FailedRelayedMessageFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for L1CrossDomainMessengerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<RelayedMessageFilter> for L1CrossDomainMessengerEvents {
        fn from(value: RelayedMessageFilter) -> Self {
            Self::RelayedMessageFilter(value)
        }
    }
    impl ::core::convert::From<SentMessageFilter> for L1CrossDomainMessengerEvents {
        fn from(value: SentMessageFilter) -> Self {
            Self::SentMessageFilter(value)
        }
    }
    impl ::core::convert::From<SentMessageExtension1Filter>
    for L1CrossDomainMessengerEvents {
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
    ///Container type for all input parameters for the `PORTAL` function with signature `PORTAL()` and selector `0x0ff754ea`
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
    #[ethcall(name = "PORTAL", abi = "PORTAL()")]
    pub struct PORTALCall;
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub portal: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `portal` function with signature `portal()` and selector `0x6425666b`
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
    #[ethcall(name = "portal", abi = "portal()")]
    pub struct portalCall;
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
    pub enum L1CrossDomainMessengerCalls {
        MessageVersion(MessageVersionCall),
        MinGasCalldataOverhead(MinGasCalldataOverheadCall),
        MinGasDynamicOverheadDenominator(MinGasDynamicOverheadDenominatorCall),
        MinGasDynamicOverheadNumerator(MinGasDynamicOverheadNumeratorCall),
        OtherMessenger(OtherMessengerCall),
        PORTAL(PORTALCall),
        RelayCallOverhead(RelayCallOverheadCall),
        RelayConstantOverhead(RelayConstantOverheadCall),
        RelayGasCheckBuffer(RelayGasCheckBufferCall),
        RelayReservedGas(RelayReservedGasCall),
        BaseGas(BaseGasCall),
        FailedMessages(FailedMessagesCall),
        Initialize(InitializeCall),
        MessageNonce(MessageNonceCall),
        portal(portalCall),
        RelayMessage(RelayMessageCall),
        SendMessage(SendMessageCall),
        SuccessfulMessages(SuccessfulMessagesCall),
        Version(VersionCall),
        XdomainMessageSender(XdomainMessageSenderCall),
    }
    impl ::ethers::core::abi::AbiDecode for L1CrossDomainMessengerCalls {
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
            if let Ok(decoded) = <PORTALCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PORTAL(decoded));
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
            if let Ok(decoded) = <MessageNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MessageNonce(decoded));
            }
            if let Ok(decoded) = <portalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::portal(decoded));
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
    impl ::ethers::core::abi::AbiEncode for L1CrossDomainMessengerCalls {
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
                Self::PORTAL(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::MessageNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::portal(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
    impl ::core::fmt::Display for L1CrossDomainMessengerCalls {
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
                Self::PORTAL(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::MessageNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::portal(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<MessageVersionCall> for L1CrossDomainMessengerCalls {
        fn from(value: MessageVersionCall) -> Self {
            Self::MessageVersion(value)
        }
    }
    impl ::core::convert::From<MinGasCalldataOverheadCall>
    for L1CrossDomainMessengerCalls {
        fn from(value: MinGasCalldataOverheadCall) -> Self {
            Self::MinGasCalldataOverhead(value)
        }
    }
    impl ::core::convert::From<MinGasDynamicOverheadDenominatorCall>
    for L1CrossDomainMessengerCalls {
        fn from(value: MinGasDynamicOverheadDenominatorCall) -> Self {
            Self::MinGasDynamicOverheadDenominator(value)
        }
    }
    impl ::core::convert::From<MinGasDynamicOverheadNumeratorCall>
    for L1CrossDomainMessengerCalls {
        fn from(value: MinGasDynamicOverheadNumeratorCall) -> Self {
            Self::MinGasDynamicOverheadNumerator(value)
        }
    }
    impl ::core::convert::From<OtherMessengerCall> for L1CrossDomainMessengerCalls {
        fn from(value: OtherMessengerCall) -> Self {
            Self::OtherMessenger(value)
        }
    }
    impl ::core::convert::From<PORTALCall> for L1CrossDomainMessengerCalls {
        fn from(value: PORTALCall) -> Self {
            Self::PORTAL(value)
        }
    }
    impl ::core::convert::From<RelayCallOverheadCall> for L1CrossDomainMessengerCalls {
        fn from(value: RelayCallOverheadCall) -> Self {
            Self::RelayCallOverhead(value)
        }
    }
    impl ::core::convert::From<RelayConstantOverheadCall>
    for L1CrossDomainMessengerCalls {
        fn from(value: RelayConstantOverheadCall) -> Self {
            Self::RelayConstantOverhead(value)
        }
    }
    impl ::core::convert::From<RelayGasCheckBufferCall> for L1CrossDomainMessengerCalls {
        fn from(value: RelayGasCheckBufferCall) -> Self {
            Self::RelayGasCheckBuffer(value)
        }
    }
    impl ::core::convert::From<RelayReservedGasCall> for L1CrossDomainMessengerCalls {
        fn from(value: RelayReservedGasCall) -> Self {
            Self::RelayReservedGas(value)
        }
    }
    impl ::core::convert::From<BaseGasCall> for L1CrossDomainMessengerCalls {
        fn from(value: BaseGasCall) -> Self {
            Self::BaseGas(value)
        }
    }
    impl ::core::convert::From<FailedMessagesCall> for L1CrossDomainMessengerCalls {
        fn from(value: FailedMessagesCall) -> Self {
            Self::FailedMessages(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for L1CrossDomainMessengerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MessageNonceCall> for L1CrossDomainMessengerCalls {
        fn from(value: MessageNonceCall) -> Self {
            Self::MessageNonce(value)
        }
    }
    impl ::core::convert::From<portalCall> for L1CrossDomainMessengerCalls {
        fn from(value: portalCall) -> Self {
            Self::portal(value)
        }
    }
    impl ::core::convert::From<RelayMessageCall> for L1CrossDomainMessengerCalls {
        fn from(value: RelayMessageCall) -> Self {
            Self::RelayMessage(value)
        }
    }
    impl ::core::convert::From<SendMessageCall> for L1CrossDomainMessengerCalls {
        fn from(value: SendMessageCall) -> Self {
            Self::SendMessage(value)
        }
    }
    impl ::core::convert::From<SuccessfulMessagesCall> for L1CrossDomainMessengerCalls {
        fn from(value: SuccessfulMessagesCall) -> Self {
            Self::SuccessfulMessages(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L1CrossDomainMessengerCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<XdomainMessageSenderCall>
    for L1CrossDomainMessengerCalls {
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
    ///Container type for all return fields from the `PORTAL` function with signature `PORTAL()` and selector `0x0ff754ea`
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
    pub struct PORTALReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `portal` function with signature `portal()` and selector `0x6425666b`
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
    pub struct portalReturn(pub ::ethers::core::types::Address);
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

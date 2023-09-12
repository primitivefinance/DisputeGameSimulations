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
pub mod resource_metering_invariant {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedContracts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("excludedSenders_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failed"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "invariant_high_usage_raise_baseFee",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invariant_high_usage_raise_baseFee",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "invariant_low_usage_lower_baseFee",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invariant_low_usage_lower_baseFee",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "invariant_never_above_max_gas_limit",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invariant_never_above_max_gas_limit",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "invariant_never_below_min_baseFee",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invariant_never_below_min_baseFee",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "invariant_never_exceed_max_decrease",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invariant_never_exceed_max_decrease",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "invariant_never_exceed_max_increase",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invariant_never_exceed_max_increase",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invariant_never_underflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invariant_never_underflow",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifactSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "targetArtifactSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifactSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedContracts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSelectors"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetedSenders_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_int",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_uint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
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
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("logs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
    pub static RESOURCEMETERING_INVARIANT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa84\x80a\0\x8A`\09`\0\xF3\xFE`\x80`@R4\x80\x15b\0\0\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10b\0\x01\xD6W`\x005`\xE0\x1C\x80c\x8B9\x9F\x95\x11b\0\x01EW\x80c\xC2\x16]\xC6\x11b\0\x01\x0FW\x80c\xE2\x0C\x9Fq\x11b\0\0\xF1W\x80c\xE2\x0C\x9Fq\x14b\0\x03<W\x80c\xF1I\x17\x95\x14b\0\x03FW\x80c\xFAv&\xD4\x14b\0\x03PWb\0\x01\xD6V[\x80c\xC2\x16]\xC6\x14b\0\x03(W\x80c\xD2\xF60\x9A\x14b\0\x032Wb\0\x01\xD6V[\x80c\x8B9\x9F\x95\x14b\0\x02\xEFW\x80c\x91j\x17\xC6\x14b\0\x02\xF9W\x80c\xB5P\x8A\xA9\x14b\0\x03\x03W\x80c\xBAAO\xA6\x14b\0\x03\rWb\0\x01\xD6V[\x80c?r\x86\xF4\x11b\0\x01\xA5W\x80cf\xD9\xA9\xA0\x11b\0\x01\x87W\x80cf\xD9\xA9\xA0\x14b\0\x02\xB3W\x80ci \xCE\xA4\x14b\0\x02\xCCW\x80c\x85\"l\x81\x14b\0\x02\xD6Wb\0\x01\xD6V[\x80c?r\x86\xF4\x14b\0\x02\x9FW\x80cc\x99\x9A\r\x14b\0\x02\xA9Wb\0\x01\xD6V[\x80c\x04$\xAF\xC3\x14b\0\x02]W\x80c\n\x92T\xE4\x14b\0\x02iW\x80c\x1E\xD7\x83\x1C\x14b\0\x02sW\x80c>^<#\x14b\0\x02\x95W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[b\0\x02gb\0\x03^V[\0[b\0\x02gb\0\x04\x91V[b\0\x02}b\0\x05\xE9V[`@Qb\0\x02\x8C\x91\x90b\0\x16!V[`@Q\x80\x91\x03\x90\xF3[b\0\x02}b\0\x06ZV[b\0\x02}b\0\x06\xC9V[b\0\x02gb\0\x078V[b\0\x02\xBDb\0\x08(V[`@Qb\0\x02\x8C\x91\x90b\0\x16}V[b\0\x02gb\0\t=V[b\0\x02\xE0b\0\n-V[`@Qb\0\x02\x8C\x91\x90b\0\x17\xA6V[b\0\x02gb\0\x0B\x07V[b\0\x02\xBDb\0\x0B\xF7V[b\0\x02\xE0b\0\r\x03V[b\0\x03\x17b\0\r\xDDV[`@Q\x90\x15\x15\x81R` \x01b\0\x02\x8CV[b\0\x02gb\0\x0FGV[b\0\x02gb\0\x107V[b\0\x02}b\0\x11'V[b\0\x02gb\0\x11\x96V[`\0Tb\0\x03\x17\x90`\xFF\x16\x81V[`\x1BT`@\x80Q\x7F\xE4\x12@#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xE4\x12@#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x04cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04\x89\x91\x90b\0\x18`V[b\0\x12\x86V[V[`@Qb\0\x04\x9F\x90b\0\x15IV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x04\xBCW=`\0\x80>=`\0\xFD[P`\x1B\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x91U`\x0E\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x01\x80T\x90\x92\x16\x17\x90U`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90Pc\x9F\xACh\xCB`\xE0\x1B\x81`\0\x81Q\x81\x10b\0\x05\x7FWb\0\x05\x7Fb\0\x19\x0BV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1BTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R\x90\x81\x01\x82\x90Rb\0\x05\xE5\x81b\0\x12\x95V[PPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$WPPPPP\x90P\x90V[`\x1BT`@\x80Q\x7F\xB8_\x9B\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xB8_\x9B\xAA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\t\x1BW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x08\xC7W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x08LV[PPPP\x90P\x90V[`\x1BT`@\x80Q\x7F\x8EKI\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x8EKI\xB3\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\ns\x90b\0\x19:V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\n\xA1\x90b\0\x19:V[\x80\x15b\0\n\xF2W\x80`\x1F\x10b\0\n\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\n\xF2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\n\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\nQV[`\x1BT`@\x80Q\x7F\x1B\xEB\xFF\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x1B\xEB\xFF\t\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\x0C\xEAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x0C\x96W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x0C\x1BV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\rI\x90b\0\x19:V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\rw\x90b\0\x19:V[\x80\x15b\0\r\xC8W\x80`\x1F\x10b\0\r\x9CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\r\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\r\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\r'V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15b\0\r\xFEWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15b\0\x0FBW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91b\0\x0E\xA6\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01b\0\x19\x8FV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x0E\xE0\x91b\0\x19\xD9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x0F\x1FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x0F$V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90b\0\x0F>\x91\x90b\0\x18`V[\x91PP[\x91\x90PV[`\x1BT`@\x80Q\x7Fd\xB4\xB12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91cd\xB4\xB12\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x1BT`@\x80Q\x7F\xDCu\xBE\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xDCu\xBE\xC7\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$WPPPPP\x90P\x90V[`\x1BT`@\x80Q\x7FGY\xC2\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91cGY\xC2\xFB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[b\0\x12\x92\x81\x15b\0\x13TV[PV[`\x13\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x81Q\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x90`\x02\x90\x92\x02\x91\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x81U` \x80\x84\x01Q\x80Q\x85\x94b\0\x13N\x93\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x91\x90\x91\x01\x92\x01\x90b\0\x15WV[PPPPV[\x80b\0\x12\x92W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qb\0\x13\xBA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1b\0\x12\x92sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15b\0\x15\x1BW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x14\x98\x92\x91` \x01b\0\x19\x8FV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x14\xD2\x91b\0\x19\xD9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x15\x11W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x15\x16V[``\x91P[PPPP[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90UV[a\x1E0\x80b\0\x19\xF8\x839\x01\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90`\x07\x01`\x08\x90\x04\x81\x01\x92\x82\x15b\0\x15\xF8W\x91` \x02\x82\x01`\0[\x83\x82\x11\x15b\0\x15\xC4W\x83Q\x83\x82a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`\xE0\x1C\x02\x17\x90UP\x92` \x01\x92`\x04\x01` \x81`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x02b\0\x15\x81V[\x80\x15b\0\x15\xF6W\x82\x81a\x01\0\n\x81T\x90c\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x04\x01` \x81`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x02b\0\x15\xC4V[P[Pb\0\x16\x06\x92\x91Pb\0\x16\nV[P\x90V[[\x80\x82\x11\x15b\0\x16\x06W`\0\x81U`\x01\x01b\0\x16\x0BV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0\x16qW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0\x16=V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15b\0\x17hW\x89\x84\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x01\x86R\x82Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15b\0\x17RW\x83Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90b\0\x17\x0EV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01b\0\x16\xA5V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15b\0\x17\x94W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x17zV[\x83\x81\x11\x15b\0\x13NWPP`\0\x91\x01RV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15b\0\x18SW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84R\x81Q\x80Q\x80\x87Rb\0\x18\x15\x81\x89\x89\x01\x8A\x85\x01b\0\x17wV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01b\0\x17\xCDV[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15b\0\x18\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x19\x04W`\0\x80\xFD[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x19OW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x19\x89W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R`\0\x82Qb\0\x19\xCB\x81`\x04\x85\x01` \x87\x01b\0\x17wV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qb\0\x19\xED\x81\x84` \x87\x01b\0\x17wV[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pb\0\0ib\0\0oV[b\0\x02SV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\0\x90WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\0\xC0WPb\0\0\xAD0b\0\x01\xA1` \x1Bb\0\x08\xFE\x17` \x1CV[\x15\x80\x15b\0\0\xC0WP`\0T`\xFF\x16`\x01\x14[b\0\x01)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x01MW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x01Wb\0\x01\xB0V[\x80\x15b\0\x01\x9EW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x02\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01 V[`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RC`\x01`\x01`@\x1B\x03\x16\x91\x90\x92\x01\x81\x90R`\x01`\xC0\x1B\x02\x17`\x01UV[a\x1B\xCD\x80b\0\x02c`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01@W`\x005`\xE0\x1C\x80c\xB8_\x9B\xAA\x11a\0\xF8W\x80c\xCF\xF0\xAB\x96\x11a\0\xDDW\x80c\xCF\xF0\xAB\x96\x14a\x02\xC4W\x80c\xDCu\xBE\xC7\x14a\x03XW\x80c\xE4\x12@#\x14a\x03oWa\x01@V[\x80c\xB8_\x9B\xAA\x14a\x02:W\x80c\xCCs\x1B\x02\x14a\x02LWa\x01@V[\x80cd\xB4\xB12\x11a\x01)W\x80cd\xB4\xB12\x14a\x02\x03W\x80c\x8EKI\xB3\x14a\x02\x10W\x80c\x9F\xACh\xCB\x14a\x02%Wa\x01@V[\x80c\x1B\xEB\xFF\t\x14a\x01\xC7W\x80cGY\xC2\xFB\x14a\x01\xF0W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`2Ta\x01\xDB\x90c\x01\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`2Ta\x01\xDB\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90`\xFF\x16\x81V[`2Ta\x01\xDB\x90d\x01\0\0\0\0\x90\x04`\xFF\x16\x81V[a\x028a\x0236`\x04a\x17\x04V[a\x03\x85V[\0[`2Ta\x01\xDB\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x02Ta\x08\xBFV[`@Qa\x01\xE7\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`\x01Ta\x03\x1F\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01\xE7V[`2Ta\x01\xDB\x90f\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90e\x01\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16`\0a\x03\xE3a\x08\xBFV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x04\x04\x91\x90a\x18\x17V[\x90P\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x85\x10\x15a\x04GW`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\x16c\x01\0\0\0\x17\x90U[\x81Qc\xFF\xFF\xFF\xFF\x16\x84\x11\x15a\x04\x82W`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[`\0\x86\x15a\x04\xA6Wa\x04\x9F\x88\x83\x85`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\t\x1AV[\x90Pa\x04\xB5V[a\x04\xB2\x88`\0\x84a\t\x1AV[\x90P[a\x04\xBE\x81a\toV[`\0\x83`@\x01Q`\xFF\x16\x87a\x04\xD3\x91\x90a\x18\x17V[\x90P\x82\x86\x11\x80\x15a\x05\x17WP`\x01Ta\x05\x13\x90\x86\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[`\x01\x14[\x15a\x05\xEBW`2Ta\x01\0\x90\x04`\xFF\x16\x80a\x05FWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10\x15[`2\x80T\x91\x15\x15a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ud\x01\0\0\0\0\x90\x04`\xFF\x16\x80a\x05\xB2WP`\x01T\x81\x90a\x05\xB0\x90\x89\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x10[`2\x80T\x91\x15\x15d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x82\x86\x10\x80a\x06+WP`\x01\x80Ta\x06)\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11[\x15a\x08\xB4W`2Tb\x01\0\0\x90\x04`\xFF\x16\x80a\x06ZWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10[`2\x80T\x91\x15\x15b\x01\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x01Ta\x06\xC5\x90\x86\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16a\x18+V[`\x01\x03a\x07CW`2Te\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x07\x05WP`\x01T\x81\x90a\x07\x02\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xB4V[`\x01\x80Ta\x07x\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11\x15a\x08\xB4W`@\x84\x01Q`\x01Ta\x07\xE9\x91a\x07\xC2\x91\x8A\x91`\xFF\x16\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x90\x03a\t\x84V[\x85``\x01Qc\xFF\xFF\xFF\xFF\x16\x86`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[`2T\x90\x88\x03\x91Pf\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08\x08WP\x86\x81\x11[`2\x80T\x91\x15\x15f\x01\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ue\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08zWP`\x01T\x81\x90a\x08w\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[PPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x08\xF9a\t\xF6V[\x90P\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0a\t'\x84\x84\x84a\n\xB1V[\x90Pa\th`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x0C\xEEV[\x93\x92PPPV[\x80`\0Z\x90Pa\t\x7F\x82\x82a\r\xEEV[PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\t\xC5a\t\x9C\x85\x83a\x18BV[a\t\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xAAV[a\t\xC0\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x1EV[a\x11\x1BV[a\t\xCF\x90\x86a\x19\x1EV[a\t\xD9\x91\x90a\x18BV[\x94\x93PPPPV[`\0a\t\xD9a\t\xF0\x85\x85a\x11LV[\x83a\x11cV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\n\xAB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[\x92\x91PPV[`\0\x81\x83\x11\x15a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0BXWP\x81\x84\x11\x15[\x15a\x0BdWP\x82a\thV[`\0a\x0Bp\x84\x84a\x18+V[a\x0B{\x90`\x01a\x19\xDAV[\x90P`\x03\x85\x11\x15\x80\x15a\x0B\x8DWP\x84\x81\x11[\x15a\x0B\xA4Wa\x0B\x9C\x85\x85a\x19\xDAV[\x91PPa\thV[a\x0B\xCF`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x85\x10\x15\x80\x15a\x0C\x06WPa\x0C\x03\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x81\x11[\x15a\x0C?Wa\x0C5\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[a\x0B\x9C\x90\x84a\x18+V[\x82\x85\x11\x15a\x0C\x95W`\0a\x0CS\x84\x87a\x18+V[\x90P`\0a\x0Ca\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0CvW\x84\x93PPPPa\thV[`\x01a\x0C\x82\x82\x88a\x19\xDAV[a\x0C\x8C\x91\x90a\x18+V[\x93PPPa\x0C\xE6V[\x83\x85\x10\x15a\x0C\xE6W`\0a\x0C\xA9\x86\x86a\x18+V[\x90P`\0a\x0C\xB7\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0C\xCCW\x85\x93PPPPa\thV[a\x0C\xD6\x81\x86a\x18+V[a\x0C\xE1\x90`\x01a\x19\xDAV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\r%\x92\x91\x90a\x1A6V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\r\xA6\x91\x90a\x1A\x8FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\xE1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xE6V[``\x91P[PPPPPPV[`\x01T`\0\x90a\x0E$\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ca\x18+V[\x90P`\0a\x0E0a\t\xF6V[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x0EQ\x91\x90a\x18BV[\x90P\x82\x15a\x0F\x88W`\x01T`\0\x90a\x0E\x88\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\xAAV[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a\x0E\x9F\x91\x90a\x19\x1EV[`\x01Ta\x0E\xBF\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\x1EV[a\x0E\xC9\x91\x90a\x18BV[`\x01T\x90\x91P`\0\x90a\x0F\x1A\x90a\x0E\xF3\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xABV[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[\x90P`\x01\x86\x11\x15a\x0FIWa\x0FFa\x0E\xF3\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa\x0FA\x91\x90a\x18+V[a\t\x84V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a\x0F\xBB\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1B\x1FV[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\x10\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FResourceMetering: cannot buy mor`D\x82\x01R\x7Fe gas than available gas limit\0\0`d\x82\x01R`\x84\x01a\x0B?V[`\x01T`\0\x90a\x10\xCA\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16a\x1BKV[\x90P`\0a\x10\xDCHc;\x9A\xCA\0a\x11rV[a\x10\xE6\x90\x83a\x18\x17V[\x90P`\0Za\x10\xF5\x90\x88a\x18+V[\x90P\x80\x82\x11\x15a\x11\x11Wa\x11\x11a\x11\x0C\x82\x84a\x18+V[a\x11\x82V[PPPPPPPPV[`\0a\thg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x113\x86a\x11\xABV[a\x11=\x91\x90a\x19\x1EV[a\x11G\x91\x90a\x18BV[a\x13\xEFV[`\0\x81\x83\x12\x15a\x11\\W\x81a\thV[P\x90\x91\x90PV[`\0\x81\x83\x12a\x11\\W\x81a\thV[`\0\x81\x83\x10\x15a\x11\\W\x81a\thV[`\0\x80Z\x90P[\x82Za\x11\x95\x90\x83a\x18+V[\x10\x15a\t\x7FWa\x11\xA4\x82a\x1B\x88V[\x91Pa\x11\x89V[`\0\x80\x82\x13a\x12\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[`\0``a\x12#\x84a\x16.V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a\x14 WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x16\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x97W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x825\x91P` \x83\x015\x80\x15\x15\x81\x14a\x17\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x18&Wa\x18&a\x17\xB9V[P\x04\x90V[`\0\x82\x82\x10\x15a\x18=Wa\x18=a\x17\xE8V[P\x03\x90V[`\0\x82a\x18QWa\x18Qa\x17\xB9V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\x18\xA5Wa\x18\xA5a\x17\xE8V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15a\x18\xE4Wa\x18\xE4a\x17\xE8V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a\x19\x18Wa\x19\x18a\x17\xE8V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x19_Wa\x19_a\x17\xE8V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\x19\x9AWa\x19\x9Aa\x17\xE8V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x19\xB6Wa\x19\xB6a\x17\xE8V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x19\xCCWa\x19\xCCa\x17\xE8V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a\x19\xEDWa\x19\xEDa\x17\xE8V[P\x01\x90V[`\0\x82a\x1A\x01Wa\x1A\x01a\x17\xB9V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1A!W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A\tV[\x83\x81\x11\x15a\x1A0W`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x1AU\x81``\x85\x01` \x88\x01a\x1A\x06V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x1A\xA1\x81\x84` \x87\x01a\x1A\x06V[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\x1A\xE5Wa\x1A\xE5a\x17\xE8V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\x1B\x19Wa\x1B\x19a\x17\xE8V[PP\x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1BBWa\x1BBa\x17\xE8V[\x01\x94\x93PPPPV[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x1B\x83Wa\x1B\x83a\x17\xE8V[P\x02\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1B\xB9Wa\x1B\xB9a\x17\xE8V[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static RESOURCEMETERING_INVARIANT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10b\0\x01\xD6W`\x005`\xE0\x1C\x80c\x8B9\x9F\x95\x11b\0\x01EW\x80c\xC2\x16]\xC6\x11b\0\x01\x0FW\x80c\xE2\x0C\x9Fq\x11b\0\0\xF1W\x80c\xE2\x0C\x9Fq\x14b\0\x03<W\x80c\xF1I\x17\x95\x14b\0\x03FW\x80c\xFAv&\xD4\x14b\0\x03PWb\0\x01\xD6V[\x80c\xC2\x16]\xC6\x14b\0\x03(W\x80c\xD2\xF60\x9A\x14b\0\x032Wb\0\x01\xD6V[\x80c\x8B9\x9F\x95\x14b\0\x02\xEFW\x80c\x91j\x17\xC6\x14b\0\x02\xF9W\x80c\xB5P\x8A\xA9\x14b\0\x03\x03W\x80c\xBAAO\xA6\x14b\0\x03\rWb\0\x01\xD6V[\x80c?r\x86\xF4\x11b\0\x01\xA5W\x80cf\xD9\xA9\xA0\x11b\0\x01\x87W\x80cf\xD9\xA9\xA0\x14b\0\x02\xB3W\x80ci \xCE\xA4\x14b\0\x02\xCCW\x80c\x85\"l\x81\x14b\0\x02\xD6Wb\0\x01\xD6V[\x80c?r\x86\xF4\x14b\0\x02\x9FW\x80cc\x99\x9A\r\x14b\0\x02\xA9Wb\0\x01\xD6V[\x80c\x04$\xAF\xC3\x14b\0\x02]W\x80c\n\x92T\xE4\x14b\0\x02iW\x80c\x1E\xD7\x83\x1C\x14b\0\x02sW\x80c>^<#\x14b\0\x02\x95W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[b\0\x02gb\0\x03^V[\0[b\0\x02gb\0\x04\x91V[b\0\x02}b\0\x05\xE9V[`@Qb\0\x02\x8C\x91\x90b\0\x16!V[`@Q\x80\x91\x03\x90\xF3[b\0\x02}b\0\x06ZV[b\0\x02}b\0\x06\xC9V[b\0\x02gb\0\x078V[b\0\x02\xBDb\0\x08(V[`@Qb\0\x02\x8C\x91\x90b\0\x16}V[b\0\x02gb\0\t=V[b\0\x02\xE0b\0\n-V[`@Qb\0\x02\x8C\x91\x90b\0\x17\xA6V[b\0\x02gb\0\x0B\x07V[b\0\x02\xBDb\0\x0B\xF7V[b\0\x02\xE0b\0\r\x03V[b\0\x03\x17b\0\r\xDDV[`@Q\x90\x15\x15\x81R` \x01b\0\x02\x8CV[b\0\x02gb\0\x0FGV[b\0\x02gb\0\x107V[b\0\x02}b\0\x11'V[b\0\x02gb\0\x11\x96V[`\0Tb\0\x03\x17\x90`\xFF\x16\x81V[`\x1BT`@\x80Q\x7F\xE4\x12@#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xE4\x12@#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x04cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04\x89\x91\x90b\0\x18`V[b\0\x12\x86V[V[`@Qb\0\x04\x9F\x90b\0\x15IV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x04\xBCW=`\0\x80>=`\0\xFD[P`\x1B\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x91U`\x0E\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x01\x80T\x90\x92\x16\x17\x90U`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90Pc\x9F\xACh\xCB`\xE0\x1B\x81`\0\x81Q\x81\x10b\0\x05\x7FWb\0\x05\x7Fb\0\x19\x0BV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1BTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R\x90\x81\x01\x82\x90Rb\0\x05\xE5\x81b\0\x12\x95V[PPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$WPPPPP\x90P\x90V[`\x1BT`@\x80Q\x7F\xB8_\x9B\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xB8_\x9B\xAA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\t\x1BW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x08\xC7W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x08LV[PPPP\x90P\x90V[`\x1BT`@\x80Q\x7F\x8EKI\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x8EKI\xB3\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\ns\x90b\0\x19:V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\n\xA1\x90b\0\x19:V[\x80\x15b\0\n\xF2W\x80`\x1F\x10b\0\n\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\n\xF2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\n\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\nQV[`\x1BT`@\x80Q\x7F\x1B\xEB\xFF\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x1B\xEB\xFF\t\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\x0C\xEAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x0C\x96W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x0C\x1BV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\t4W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\rI\x90b\0\x19:V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\rw\x90b\0\x19:V[\x80\x15b\0\r\xC8W\x80`\x1F\x10b\0\r\x9CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\r\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\r\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\r'V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15b\0\r\xFEWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15b\0\x0FBW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91b\0\x0E\xA6\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01b\0\x19\x8FV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x0E\xE0\x91b\0\x19\xD9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x0F\x1FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x0F$V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90b\0\x0F>\x91\x90b\0\x18`V[\x91PP[\x91\x90PV[`\x1BT`@\x80Q\x7Fd\xB4\xB12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91cd\xB4\xB12\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x1BT`@\x80Q\x7F\xDCu\xBE\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\xDCu\xBE\xC7\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x06PW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x06$WPPPPP\x90P\x90V[`\x1BT`@\x80Q\x7FGY\xC2\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qb\0\x04\x8F\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91cGY\xC2\xFB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x04NW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[b\0\x12\x92\x81\x15b\0\x13TV[PV[`\x13\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x81Q\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x90`\x02\x90\x92\x02\x91\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x81U` \x80\x84\x01Q\x80Q\x85\x94b\0\x13N\x93\x7Ff\xDE\x8F\xFD\xA7\x97\xE3\xDE\x9C\x05\xE8\xFCW\xB3\xBF\x0E\xC2\x8A\x93\r@\xB0\xD2\x85\xD9<\x06P\x1C\xF6\xA0\x91\x90\x91\x01\x92\x01\x90b\0\x15WV[PPPPV[\x80b\0\x12\x92W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qb\0\x13\xBA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1b\0\x12\x92sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15b\0\x15\x1BW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x14\x98\x92\x91` \x01b\0\x19\x8FV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x14\xD2\x91b\0\x19\xD9V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x15\x11W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x15\x16V[``\x91P[PPPP[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90UV[a\x1E0\x80b\0\x19\xF8\x839\x01\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90`\x07\x01`\x08\x90\x04\x81\x01\x92\x82\x15b\0\x15\xF8W\x91` \x02\x82\x01`\0[\x83\x82\x11\x15b\0\x15\xC4W\x83Q\x83\x82a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`\xE0\x1C\x02\x17\x90UP\x92` \x01\x92`\x04\x01` \x81`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x02b\0\x15\x81V[\x80\x15b\0\x15\xF6W\x82\x81a\x01\0\n\x81T\x90c\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x04\x01` \x81`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x02b\0\x15\xC4V[P[Pb\0\x16\x06\x92\x91Pb\0\x16\nV[P\x90V[[\x80\x82\x11\x15b\0\x16\x06W`\0\x81U`\x01\x01b\0\x16\x0BV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0\x16qW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0\x16=V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15b\0\x17hW\x89\x84\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x01\x86R\x82Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15b\0\x17RW\x83Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90b\0\x17\x0EV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01b\0\x16\xA5V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15b\0\x17\x94W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x17zV[\x83\x81\x11\x15b\0\x13NWPP`\0\x91\x01RV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15b\0\x18SW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84R\x81Q\x80Q\x80\x87Rb\0\x18\x15\x81\x89\x89\x01\x8A\x85\x01b\0\x17wV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01b\0\x17\xCDV[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15b\0\x18\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x19\x04W`\0\x80\xFD[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x19OW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x19\x89W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R`\0\x82Qb\0\x19\xCB\x81`\x04\x85\x01` \x87\x01b\0\x17wV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qb\0\x19\xED\x81\x84` \x87\x01b\0\x17wV[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pb\0\0ib\0\0oV[b\0\x02SV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\0\x90WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\0\xC0WPb\0\0\xAD0b\0\x01\xA1` \x1Bb\0\x08\xFE\x17` \x1CV[\x15\x80\x15b\0\0\xC0WP`\0T`\xFF\x16`\x01\x14[b\0\x01)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x01MW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x01Wb\0\x01\xB0V[\x80\x15b\0\x01\x9EW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x02\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01 V[`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RC`\x01`\x01`@\x1B\x03\x16\x91\x90\x92\x01\x81\x90R`\x01`\xC0\x1B\x02\x17`\x01UV[a\x1B\xCD\x80b\0\x02c`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01@W`\x005`\xE0\x1C\x80c\xB8_\x9B\xAA\x11a\0\xF8W\x80c\xCF\xF0\xAB\x96\x11a\0\xDDW\x80c\xCF\xF0\xAB\x96\x14a\x02\xC4W\x80c\xDCu\xBE\xC7\x14a\x03XW\x80c\xE4\x12@#\x14a\x03oWa\x01@V[\x80c\xB8_\x9B\xAA\x14a\x02:W\x80c\xCCs\x1B\x02\x14a\x02LWa\x01@V[\x80cd\xB4\xB12\x11a\x01)W\x80cd\xB4\xB12\x14a\x02\x03W\x80c\x8EKI\xB3\x14a\x02\x10W\x80c\x9F\xACh\xCB\x14a\x02%Wa\x01@V[\x80c\x1B\xEB\xFF\t\x14a\x01\xC7W\x80cGY\xC2\xFB\x14a\x01\xF0W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`2Ta\x01\xDB\x90c\x01\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`2Ta\x01\xDB\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90`\xFF\x16\x81V[`2Ta\x01\xDB\x90d\x01\0\0\0\0\x90\x04`\xFF\x16\x81V[a\x028a\x0236`\x04a\x17\x04V[a\x03\x85V[\0[`2Ta\x01\xDB\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x02Ta\x08\xBFV[`@Qa\x01\xE7\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`\x01Ta\x03\x1F\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01\xE7V[`2Ta\x01\xDB\x90f\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90e\x01\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16`\0a\x03\xE3a\x08\xBFV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x04\x04\x91\x90a\x18\x17V[\x90P\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x85\x10\x15a\x04GW`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\x16c\x01\0\0\0\x17\x90U[\x81Qc\xFF\xFF\xFF\xFF\x16\x84\x11\x15a\x04\x82W`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[`\0\x86\x15a\x04\xA6Wa\x04\x9F\x88\x83\x85`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\t\x1AV[\x90Pa\x04\xB5V[a\x04\xB2\x88`\0\x84a\t\x1AV[\x90P[a\x04\xBE\x81a\toV[`\0\x83`@\x01Q`\xFF\x16\x87a\x04\xD3\x91\x90a\x18\x17V[\x90P\x82\x86\x11\x80\x15a\x05\x17WP`\x01Ta\x05\x13\x90\x86\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[`\x01\x14[\x15a\x05\xEBW`2Ta\x01\0\x90\x04`\xFF\x16\x80a\x05FWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10\x15[`2\x80T\x91\x15\x15a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ud\x01\0\0\0\0\x90\x04`\xFF\x16\x80a\x05\xB2WP`\x01T\x81\x90a\x05\xB0\x90\x89\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x10[`2\x80T\x91\x15\x15d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x82\x86\x10\x80a\x06+WP`\x01\x80Ta\x06)\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11[\x15a\x08\xB4W`2Tb\x01\0\0\x90\x04`\xFF\x16\x80a\x06ZWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10[`2\x80T\x91\x15\x15b\x01\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x01Ta\x06\xC5\x90\x86\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16a\x18+V[`\x01\x03a\x07CW`2Te\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x07\x05WP`\x01T\x81\x90a\x07\x02\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xB4V[`\x01\x80Ta\x07x\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11\x15a\x08\xB4W`@\x84\x01Q`\x01Ta\x07\xE9\x91a\x07\xC2\x91\x8A\x91`\xFF\x16\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x90\x03a\t\x84V[\x85``\x01Qc\xFF\xFF\xFF\xFF\x16\x86`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[`2T\x90\x88\x03\x91Pf\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08\x08WP\x86\x81\x11[`2\x80T\x91\x15\x15f\x01\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ue\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08zWP`\x01T\x81\x90a\x08w\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[PPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x08\xF9a\t\xF6V[\x90P\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0a\t'\x84\x84\x84a\n\xB1V[\x90Pa\th`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x0C\xEEV[\x93\x92PPPV[\x80`\0Z\x90Pa\t\x7F\x82\x82a\r\xEEV[PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\t\xC5a\t\x9C\x85\x83a\x18BV[a\t\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xAAV[a\t\xC0\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x1EV[a\x11\x1BV[a\t\xCF\x90\x86a\x19\x1EV[a\t\xD9\x91\x90a\x18BV[\x94\x93PPPPV[`\0a\t\xD9a\t\xF0\x85\x85a\x11LV[\x83a\x11cV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\n\xAB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[\x92\x91PPV[`\0\x81\x83\x11\x15a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0BXWP\x81\x84\x11\x15[\x15a\x0BdWP\x82a\thV[`\0a\x0Bp\x84\x84a\x18+V[a\x0B{\x90`\x01a\x19\xDAV[\x90P`\x03\x85\x11\x15\x80\x15a\x0B\x8DWP\x84\x81\x11[\x15a\x0B\xA4Wa\x0B\x9C\x85\x85a\x19\xDAV[\x91PPa\thV[a\x0B\xCF`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x85\x10\x15\x80\x15a\x0C\x06WPa\x0C\x03\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x81\x11[\x15a\x0C?Wa\x0C5\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[a\x0B\x9C\x90\x84a\x18+V[\x82\x85\x11\x15a\x0C\x95W`\0a\x0CS\x84\x87a\x18+V[\x90P`\0a\x0Ca\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0CvW\x84\x93PPPPa\thV[`\x01a\x0C\x82\x82\x88a\x19\xDAV[a\x0C\x8C\x91\x90a\x18+V[\x93PPPa\x0C\xE6V[\x83\x85\x10\x15a\x0C\xE6W`\0a\x0C\xA9\x86\x86a\x18+V[\x90P`\0a\x0C\xB7\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0C\xCCW\x85\x93PPPPa\thV[a\x0C\xD6\x81\x86a\x18+V[a\x0C\xE1\x90`\x01a\x19\xDAV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\r%\x92\x91\x90a\x1A6V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\r\xA6\x91\x90a\x1A\x8FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\xE1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xE6V[``\x91P[PPPPPPV[`\x01T`\0\x90a\x0E$\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ca\x18+V[\x90P`\0a\x0E0a\t\xF6V[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x0EQ\x91\x90a\x18BV[\x90P\x82\x15a\x0F\x88W`\x01T`\0\x90a\x0E\x88\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\xAAV[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a\x0E\x9F\x91\x90a\x19\x1EV[`\x01Ta\x0E\xBF\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\x1EV[a\x0E\xC9\x91\x90a\x18BV[`\x01T\x90\x91P`\0\x90a\x0F\x1A\x90a\x0E\xF3\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xABV[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[\x90P`\x01\x86\x11\x15a\x0FIWa\x0FFa\x0E\xF3\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa\x0FA\x91\x90a\x18+V[a\t\x84V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a\x0F\xBB\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1B\x1FV[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\x10\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FResourceMetering: cannot buy mor`D\x82\x01R\x7Fe gas than available gas limit\0\0`d\x82\x01R`\x84\x01a\x0B?V[`\x01T`\0\x90a\x10\xCA\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16a\x1BKV[\x90P`\0a\x10\xDCHc;\x9A\xCA\0a\x11rV[a\x10\xE6\x90\x83a\x18\x17V[\x90P`\0Za\x10\xF5\x90\x88a\x18+V[\x90P\x80\x82\x11\x15a\x11\x11Wa\x11\x11a\x11\x0C\x82\x84a\x18+V[a\x11\x82V[PPPPPPPPV[`\0a\thg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x113\x86a\x11\xABV[a\x11=\x91\x90a\x19\x1EV[a\x11G\x91\x90a\x18BV[a\x13\xEFV[`\0\x81\x83\x12\x15a\x11\\W\x81a\thV[P\x90\x91\x90PV[`\0\x81\x83\x12a\x11\\W\x81a\thV[`\0\x81\x83\x10\x15a\x11\\W\x81a\thV[`\0\x80Z\x90P[\x82Za\x11\x95\x90\x83a\x18+V[\x10\x15a\t\x7FWa\x11\xA4\x82a\x1B\x88V[\x91Pa\x11\x89V[`\0\x80\x82\x13a\x12\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[`\0``a\x12#\x84a\x16.V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a\x14 WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x16\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x97W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x825\x91P` \x83\x015\x80\x15\x15\x81\x14a\x17\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x18&Wa\x18&a\x17\xB9V[P\x04\x90V[`\0\x82\x82\x10\x15a\x18=Wa\x18=a\x17\xE8V[P\x03\x90V[`\0\x82a\x18QWa\x18Qa\x17\xB9V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\x18\xA5Wa\x18\xA5a\x17\xE8V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15a\x18\xE4Wa\x18\xE4a\x17\xE8V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a\x19\x18Wa\x19\x18a\x17\xE8V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x19_Wa\x19_a\x17\xE8V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\x19\x9AWa\x19\x9Aa\x17\xE8V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x19\xB6Wa\x19\xB6a\x17\xE8V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x19\xCCWa\x19\xCCa\x17\xE8V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a\x19\xEDWa\x19\xEDa\x17\xE8V[P\x01\x90V[`\0\x82a\x1A\x01Wa\x1A\x01a\x17\xB9V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1A!W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A\tV[\x83\x81\x11\x15a\x1A0W`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x1AU\x81``\x85\x01` \x88\x01a\x1A\x06V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x1A\xA1\x81\x84` \x87\x01a\x1A\x06V[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\x1A\xE5Wa\x1A\xE5a\x17\xE8V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\x1B\x19Wa\x1B\x19a\x17\xE8V[PP\x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1BBWa\x1BBa\x17\xE8V[\x01\x94\x93PPPPV[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x1B\x83Wa\x1B\x83a\x17\xE8V[P\x02\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1B\xB9Wa\x1B\xB9a\x17\xE8V[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static RESOURCEMETERING_INVARIANT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ResourceMetering_Invariant<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ResourceMetering_Invariant<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ResourceMetering_Invariant<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ResourceMetering_Invariant<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ResourceMetering_Invariant<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ResourceMetering_Invariant))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ResourceMetering_Invariant<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESOURCEMETERING_INVARIANT_ABI.clone(),
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
                RESOURCEMETERING_INVARIANT_ABI.clone(),
                RESOURCEMETERING_INVARIANT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_TEST` (0xfa7626d4) function
        pub fn is_test(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeArtifacts` (0xb5508aa9) function
        pub fn exclude_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([181, 80, 138, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeContracts` (0xe20c9f71) function
        pub fn exclude_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([226, 12, 159, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeSenders` (0x1ed7831c) function
        pub fn exclude_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([30, 215, 131, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant_high_usage_raise_baseFee` (0x63999a0d) function
        pub fn invariant_high_usage_raise_base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 153, 154, 13], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant_low_usage_lower_baseFee` (0xf1491795) function
        pub fn invariant_low_usage_lower_base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 73, 23, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant_never_above_max_gas_limit` (0xc2165dc6) function
        pub fn invariant_never_above_max_gas_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 22, 93, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant_never_below_min_baseFee` (0x8b399f95) function
        pub fn invariant_never_below_min_base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 57, 159, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant_never_exceed_max_decrease` (0x0424afc3) function
        pub fn invariant_never_exceed_max_decrease(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 36, 175, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant_never_exceed_max_increase` (0x6920cea4) function
        pub fn invariant_never_exceed_max_increase(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 32, 206, 164], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariant_never_underflow` (0xd2f6309a) function
        pub fn invariant_never_underflow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 246, 48, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUp` (0x0a9254e4) function
        pub fn set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function
        pub fn target_artifact_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifacts` (0x85226c81) function
        pub fn target_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([133, 34, 108, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetContracts` (0x3f7286f4) function
        pub fn target_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 114, 134, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSelectors` (0x916a17c6) function
        pub fn target_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([145, 106, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSenders` (0x3e5e3c23) function
        pub fn target_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([62, 94, 60, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_int` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_named_address` event
        pub fn log_named_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_uint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        ///Gets the contract's `logs` event
        pub fn logs_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ResourceMetering_InvariantEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ResourceMetering_Invariant<M> {
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ::ethers::core::types::Address);
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
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ::ethers::core::types::Bytes);
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
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
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
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub ::ethers::core::types::I256);
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
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: ::std::string::String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
        pub decimals: ::ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: ::std::string::String,
        pub val: ::std::string::String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ::ethers::core::types::U256);
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
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ResourceMetering_InvariantEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ::ethers::contract::EthLogDecode for ResourceMetering_InvariantEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedArray1Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedArray2Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedArray3Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedBytesFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedBytes32Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedDecimalIntFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedDecimalUintFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(
                    ResourceMetering_InvariantEvents::LogNamedStringFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(ResourceMetering_InvariantEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ResourceMetering_InvariantEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytes32Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalIntFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for ResourceMetering_InvariantEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for ResourceMetering_InvariantEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for ResourceMetering_InvariantEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for ResourceMetering_InvariantEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter>
    for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for ResourceMetering_InvariantEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    #[ethcall(name = "excludeArtifacts", abi = "excludeArtifacts()")]
    pub struct ExcludeArtifactsCall;
    ///Container type for all input parameters for the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
    ///Container type for all input parameters for the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    #[ethcall(name = "excludeSenders", abi = "excludeSenders()")]
    pub struct ExcludeSendersCall;
    ///Container type for all input parameters for the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `invariant_high_usage_raise_baseFee` function with signature `invariant_high_usage_raise_baseFee()` and selector `0x63999a0d`
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
        name = "invariant_high_usage_raise_baseFee",
        abi = "invariant_high_usage_raise_baseFee()"
    )]
    pub struct InvariantHighUsageRaiseBaseFeeCall;
    ///Container type for all input parameters for the `invariant_low_usage_lower_baseFee` function with signature `invariant_low_usage_lower_baseFee()` and selector `0xf1491795`
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
        name = "invariant_low_usage_lower_baseFee",
        abi = "invariant_low_usage_lower_baseFee()"
    )]
    pub struct InvariantLowUsageLowerBaseFeeCall;
    ///Container type for all input parameters for the `invariant_never_above_max_gas_limit` function with signature `invariant_never_above_max_gas_limit()` and selector `0xc2165dc6`
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
        name = "invariant_never_above_max_gas_limit",
        abi = "invariant_never_above_max_gas_limit()"
    )]
    pub struct InvariantNeverAboveMaxGasLimitCall;
    ///Container type for all input parameters for the `invariant_never_below_min_baseFee` function with signature `invariant_never_below_min_baseFee()` and selector `0x8b399f95`
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
        name = "invariant_never_below_min_baseFee",
        abi = "invariant_never_below_min_baseFee()"
    )]
    pub struct InvariantNeverBelowMinBaseFeeCall;
    ///Container type for all input parameters for the `invariant_never_exceed_max_decrease` function with signature `invariant_never_exceed_max_decrease()` and selector `0x0424afc3`
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
        name = "invariant_never_exceed_max_decrease",
        abi = "invariant_never_exceed_max_decrease()"
    )]
    pub struct InvariantNeverExceedMaxDecreaseCall;
    ///Container type for all input parameters for the `invariant_never_exceed_max_increase` function with signature `invariant_never_exceed_max_increase()` and selector `0x6920cea4`
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
        name = "invariant_never_exceed_max_increase",
        abi = "invariant_never_exceed_max_increase()"
    )]
    pub struct InvariantNeverExceedMaxIncreaseCall;
    ///Container type for all input parameters for the `invariant_never_underflow` function with signature `invariant_never_underflow()` and selector `0xd2f6309a`
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
    #[ethcall(name = "invariant_never_underflow", abi = "invariant_never_underflow()")]
    pub struct InvariantNeverUnderflowCall;
    ///Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `0x0a9254e4`
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
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    ///Container type for all input parameters for the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    #[ethcall(name = "targetArtifactSelectors", abi = "targetArtifactSelectors()")]
    pub struct TargetArtifactSelectorsCall;
    ///Container type for all input parameters for the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    #[ethcall(name = "targetArtifacts", abi = "targetArtifacts()")]
    pub struct TargetArtifactsCall;
    ///Container type for all input parameters for the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    #[ethcall(name = "targetContracts", abi = "targetContracts()")]
    pub struct TargetContractsCall;
    ///Container type for all input parameters for the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    #[ethcall(name = "targetSelectors", abi = "targetSelectors()")]
    pub struct TargetSelectorsCall;
    ///Container type for all input parameters for the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ResourceMetering_InvariantCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        InvariantHighUsageRaiseBaseFee(InvariantHighUsageRaiseBaseFeeCall),
        InvariantLowUsageLowerBaseFee(InvariantLowUsageLowerBaseFeeCall),
        InvariantNeverAboveMaxGasLimit(InvariantNeverAboveMaxGasLimitCall),
        InvariantNeverBelowMinBaseFee(InvariantNeverBelowMinBaseFeeCall),
        InvariantNeverExceedMaxDecrease(InvariantNeverExceedMaxDecreaseCall),
        InvariantNeverExceedMaxIncrease(InvariantNeverExceedMaxIncreaseCall),
        InvariantNeverUnderflow(InvariantNeverUnderflowCall),
        SetUp(SetUpCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
    }
    impl ::ethers::core::abi::AbiDecode for ResourceMetering_InvariantCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) = <ExcludeArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) = <ExcludeContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeContracts(decoded));
            }
            if let Ok(decoded) = <ExcludeSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <InvariantHighUsageRaiseBaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantHighUsageRaiseBaseFee(decoded));
            }
            if let Ok(decoded) = <InvariantLowUsageLowerBaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantLowUsageLowerBaseFee(decoded));
            }
            if let Ok(decoded) = <InvariantNeverAboveMaxGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantNeverAboveMaxGasLimit(decoded));
            }
            if let Ok(decoded) = <InvariantNeverBelowMinBaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantNeverBelowMinBaseFee(decoded));
            }
            if let Ok(decoded) = <InvariantNeverExceedMaxDecreaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantNeverExceedMaxDecrease(decoded));
            }
            if let Ok(decoded) = <InvariantNeverExceedMaxIncreaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantNeverExceedMaxIncrease(decoded));
            }
            if let Ok(decoded) = <InvariantNeverUnderflowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantNeverUnderflow(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUp(decoded));
            }
            if let Ok(decoded) = <TargetArtifactSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) = <TargetArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifacts(decoded));
            }
            if let Ok(decoded) = <TargetContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetContracts(decoded));
            }
            if let Ok(decoded) = <TargetSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSelectors(decoded));
            }
            if let Ok(decoded) = <TargetSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSenders(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ResourceMetering_InvariantCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvariantHighUsageRaiseBaseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvariantLowUsageLowerBaseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvariantNeverAboveMaxGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvariantNeverBelowMinBaseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvariantNeverExceedMaxDecrease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvariantNeverExceedMaxIncrease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvariantNeverUnderflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ResourceMetering_InvariantCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvariantHighUsageRaiseBaseFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvariantLowUsageLowerBaseFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvariantNeverAboveMaxGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvariantNeverBelowMinBaseFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvariantNeverExceedMaxDecrease(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvariantNeverExceedMaxIncrease(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvariantNeverUnderflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for ResourceMetering_InvariantCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for ResourceMetering_InvariantCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for ResourceMetering_InvariantCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<InvariantHighUsageRaiseBaseFeeCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: InvariantHighUsageRaiseBaseFeeCall) -> Self {
            Self::InvariantHighUsageRaiseBaseFee(value)
        }
    }
    impl ::core::convert::From<InvariantLowUsageLowerBaseFeeCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: InvariantLowUsageLowerBaseFeeCall) -> Self {
            Self::InvariantLowUsageLowerBaseFee(value)
        }
    }
    impl ::core::convert::From<InvariantNeverAboveMaxGasLimitCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: InvariantNeverAboveMaxGasLimitCall) -> Self {
            Self::InvariantNeverAboveMaxGasLimit(value)
        }
    }
    impl ::core::convert::From<InvariantNeverBelowMinBaseFeeCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: InvariantNeverBelowMinBaseFeeCall) -> Self {
            Self::InvariantNeverBelowMinBaseFee(value)
        }
    }
    impl ::core::convert::From<InvariantNeverExceedMaxDecreaseCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: InvariantNeverExceedMaxDecreaseCall) -> Self {
            Self::InvariantNeverExceedMaxDecrease(value)
        }
    }
    impl ::core::convert::From<InvariantNeverExceedMaxIncreaseCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: InvariantNeverExceedMaxIncreaseCall) -> Self {
            Self::InvariantNeverExceedMaxIncrease(value)
        }
    }
    impl ::core::convert::From<InvariantNeverUnderflowCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: InvariantNeverUnderflowCall) -> Self {
            Self::InvariantNeverUnderflow(value)
        }
    }
    impl ::core::convert::From<SetUpCall> for ResourceMetering_InvariantCalls {
        fn from(value: SetUpCall) -> Self {
            Self::SetUp(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall>
    for ResourceMetering_InvariantCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for ResourceMetering_InvariantCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for ResourceMetering_InvariantCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for ResourceMetering_InvariantCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for ResourceMetering_InvariantCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    pub struct IsTestReturn(pub bool);
    ///Container type for all return fields from the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    pub struct ExcludeArtifactsReturn {
        pub excluded_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    pub struct ExcludeSendersReturn {
        pub excluded_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    pub struct FailedReturn(pub bool);
    ///Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    pub struct TargetArtifactsReturn {
        pub targeted_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    pub struct TargetContractsReturn {
        pub targeted_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    pub struct TargetSelectorsReturn {
        pub targeted_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
}

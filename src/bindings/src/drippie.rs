pub use drippie::*;
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
pub mod drippie {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CALL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CALL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DELEGATECALL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DELEGATECALL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("create"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("create"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Drippie.DripConfig",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("drip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("drip"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("drips"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("drips"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Drippie.DripStatus"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Drippie.DripConfig",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("last"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
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
                    ::std::borrow::ToOwned::to_owned("executable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executable"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("setOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("status"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("status"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Drippie.DripStatus"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawERC721"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawERC721"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC721"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_id"),
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
                (
                    ::std::borrow::ToOwned::to_owned("withdrawETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
                    ::std::borrow::ToOwned::to_owned("DripCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DripCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nameref"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DripExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DripExecuted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nameref"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("executor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("DripStatusUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DripStatusUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nameref"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnerUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OwnerUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReceivedETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReceivedETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrewERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrewERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrewERC721"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrewERC721"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrewETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrewETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DRIPPIE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0.\r8\x03\x80b\0.\r\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\x8CV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x83\x92\x83\x92\x83\x92\x90\x91\x90\x7F\x82\x92\xFC\xE1\x8F\xA6\x9E\xDFM\xB7\xB9N\xA2\xE5\x82A\xDF\n\xE5\x7F\x97\xE0\xA6\xC9\xB2\x90g\x02\x8B\xF9-v\x90\x82\x90\xA3PPPPb\0\0\xBEV[`\0` \x82\x84\x03\x12\x15b\0\0\x9FW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xB7W`\0\x80\xFD[\x93\x92PPPV[a-?\x80b\0\0\xCE`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xE1W`\x005`\xE0\x1C\x80cn-D\xAE\x11a\0\x7FW\x80c\x9B\xC9M\x01\x11a\0YW\x80c\x9B\xC9M\x01\x14a\x02\xB0W\x80c\xE5Q\xCD\xAA\x14a\x02\xD0W\x80c\xED\xEEb9\x14a\x02\xF0W\x80c\xFC>>\xBA\x14a\x03\x03W`\0\x80\xFD[\x80cn-D\xAE\x14a\x02\x1DW\x80c\x8D\xA5\xCB[\x14a\x02>W\x80c\x94V\xFB\xCC\x14a\x02\x90W`\0\x80\xFD[\x80cG\x82\xF7y\x11a\0\xBBW\x80cG\x82\xF7y\x14a\x01\x84W\x80cM\x7F\xBAn\x14a\x01\xA4W\x80cg\x14\x8C\xD2\x14a\x01\xDDW\x80ci\r\x83 \x14a\x01\xFDW`\0\x80\xFD[\x80c\x13\xAF@5\x14a\x01\"W\x80c@%\xFE\xB2\x14a\x01DW\x80cD\0L\xC1\x14a\x01dW`\0\x80\xFD[6a\x01\x1DW`@Q4\x81R3\x90\x7FA\x03%~\xAA\xC9\x83\xCAy\xA7\r(\xF9\r\xFCO\xA1ka\x9B\xB0\xC1~\xE7\xCA\xB0\xD4\x03L'\x96$\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[`\0\x80\xFD[4\x80\x15a\x01.W`\0\x80\xFD[Pa\x01Ba\x01=6`\x04a\x1D2V[a\x033V[\0[4\x80\x15a\x01PW`\0\x80\xFD[Pa\x01Ba\x01_6`\x04a\x1DVV[a\x04)V[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01Ba\x01\x7F6`\x04a\x1DVV[a\x05\xBBV[4\x80\x15a\x01\x90W`\0\x80\xFD[Pa\x01Ba\x01\x9F6`\x04a\x1D\x97V[a\x07LV[4\x80\x15a\x01\xB0W`\0\x80\xFD[Pa\x01\xC4a\x01\xBF6`\x04a\x1E\x86V[a\x08\x9CV[`@Qa\x01\xD4\x94\x93\x92\x91\x90a\x1F\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01Ba\x01\xF86`\x04a!:V[a\n\xCEV[4\x80\x15a\x02\tW`\0\x80\xFD[Pa\x01Ba\x02\x186`\x04a\x1D2V[a\x0C\xF1V[a\x020a\x02+6`\x04a!\x9CV[a\r\x7FV[`@Qa\x01\xD4\x92\x91\x90a!\xF5V[4\x80\x15a\x02JW`\0\x80\xFD[P`\0Ta\x02k\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xD4V[4\x80\x15a\x02\x9CW`\0\x80\xFD[Pa\x01Ba\x02\xAB6`\x04a\"\x10V[a\x0EyV[4\x80\x15a\x02\xBCW`\0\x80\xFD[Pa\x01Ba\x02\xCB6`\x04a\"IV[a\x0F\x94V[4\x80\x15a\x02\xDCW`\0\x80\xFD[Pa\x01Ba\x02\xEB6`\x04a\"\xA4V[a\x14\xCFV[a\x020a\x02\xFE6`\x04a#\tV[a\x19SV[4\x80\x15a\x03\x0FW`\0\x80\xFD[Pa\x03#a\x03\x1E6`\x04a!:V[a\x1AIV[`@Q\x90\x15\x15\x81R` \x01a\x01\xD4V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x82\x92\xFC\xE1\x8F\xA6\x9E\xDFM\xB7\xB9N\xA2\xE5\x82A\xDF\n\xE5\x7F\x97\xE0\xA6\xC9\xB2\x90g\x02\x8B\xF9-v\x91\x90\xA3PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R\x84\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05 W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x054W=`\0\x80>=`\0\xFD[PPPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F0\xB4x\xA5\xE1\x96\xE5X\x86\"\x8A\xA8{\xA7J}\xFE\xBAe^\nM{\xA2u\xEA\xBF\xC2*\xAB\xB7\xA8\x84`@Qa\x05\xAE\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD5\x91\x90a#gV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fk\0\xF1\xC7\x88?\x05;\xA8>\x90\x7F\xD1\x96[\"\xFF\xFE<A\x118>r_\x04c\x8AVl\xDB\xFA\x84`@Qa\x05\xAE\x91\x81R` \x01\x90V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\xCDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08'W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08,V[``\x91P[PP\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1F\x12\xAA\x8BmI-\xD9\xB9\x8E+\0\xB0\xB2\x080\xC2\xA7\xDE\xD6Z\xFA\xC1;`\xD1i\xA04\xAE\x90\xBC\x84`@Qa\x08\x8F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x01\x80\x83R\x93\x83\x01\x94\x83\x01\x94\x90\x94 \x93\x90R\x82T`@\x80Q`\xA0\x81\x01\x82R\x93\x85\x01\x80T`\xFF\x90\x81\x16\x15\x15\x86R`\x02\x87\x01T\x94\x86\x01\x94\x90\x94R`\x03\x86\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x85\x01\x91\x90\x91R`\x04\x85\x01\x80T\x93\x90\x92\x16\x94\x93\x92\x90\x91``\x84\x01\x91a\t\x1E\x90a#\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tJ\x90a#\x84V[\x80\x15a\t\x97W\x80`\x1F\x10a\tlWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tzW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\n\xB6W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x90\x91R`\x03\x85\x02\x90\x91\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\n\x1B\x90a#\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nG\x90a#\x84V[\x80\x15a\n\x94W\x80`\x1F\x10a\niWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x94V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nwW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\t\xC5V[PPP\x91RPP`\x06\x82\x01T`\x07\x90\x92\x01T\x90\x91\x90\x84V[`\0`\x01\x83\x83`@Qa\n\xE2\x92\x91\x90a#\xD7V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90Pa\n\xFC\x83\x83a\x1AIV[PB`\x06\x82\x01U`\x07\x81\x01\x80T\x90`\0a\x0B\x15\x83a$\x16V[\x90\x91UPP`\x05\x81\x01T`\0[\x81\x81\x10\x15a\x0C\x95W`\0\x83`\x01\x01`\x04\x01\x82\x81T\x81\x10a\x0BDWa\x0BDa$NV[`\0\x91\x82R` \x82 `\x03\x90\x91\x02\x01\x80T`\x02\x82\x01T`@Q\x92\x94Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x0B\x87\x90`\x01\x86\x01\x90a$}V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xC9V[``\x91P[PP\x90P\x80a\x0C\x80W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FDrippie: drip was unsuccessful, `D\x82\x01R\x7Fplease check your configuration `d\x82\x01R\x7Ffor mistakes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[PP\x80\x80a\x0C\x8D\x90a$\x16V[\x91PPa\x0B\"V[P\x83\x83`@Qa\x0C\xA6\x92\x91\x90a#\xD7V[`@Q\x80\x91\x03\x90 \x7F\xEA!CT\x19\xAA\xD9\xC5J\x9D\x90\xE2R+o`\xBDVd\x01\xF3o\xCE\xF6a\xF5\xF5\xA2\x8C\xC0\xD2\xC6\x85\x853B`@Qa\x0C\xE3\x94\x93\x92\x91\x90a%ZV[`@Q\x80\x91\x03\x90\xA2PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\rrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[a\r|\x81Ga\x07LV[PV[`\0\x80T``\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\x04W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x85`@Qa\x0E*\x91\x90a%\x97V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0EgW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0ElV[``\x91P[P\x90\x96\x90\x95P\x93PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\xFAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Ra\x0F\x90\x90\x83\x90\x83\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FlW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x7F\x91\x90a%\xB3V[PPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x10\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`\0\x81`\x03\x81\x11\x15a\x10)Wa\x10)a\x1E\xD7V[\x03a\x10\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDrippie: drip status can never b`D\x82\x01R\x7Fe set back to NONE after creatio`d\x82\x01R\x7Fn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\0`\x01\x84\x84`@Qa\x10\xF0\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x90P`\0\x81`\x03\x81\x11\x15a\x11\x19Wa\x11\x19a\x1E\xD7V[\x03a\x11\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDrippie: drip with that name doe`D\x82\x01R\x7Fs not exist and cannot be update`d\x82\x01R\x7Fd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\x03\x81`\x03\x81\x11\x15a\x11\xE0Wa\x11\xE0a\x1E\xD7V[\x03a\x12\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDrippie: drip with that name has\x90\x82\x01R\x7F been archived and cannot be upd`d\x82\x01R\x7Fated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[\x81`\x03\x81\x11\x15a\x12\xA6Wa\x12\xA6a\x1E\xD7V[\x81`\x03\x81\x11\x15a\x12\xB8Wa\x12\xB8a\x1E\xD7V[\x03a\x13kW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDrippie: cannot set drip status `D\x82\x01R\x7Fto the same status as its curren`d\x82\x01R\x7Ft status\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\x03\x82`\x03\x81\x11\x15a\x13\x7FWa\x13\x7Fa\x1E\xD7V[\x03a\x14%W`\x01\x81`\x03\x81\x11\x15a\x13\x98Wa\x13\x98a\x1E\xD7V[\x14a\x14%W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDrippie: drip must first be paus`D\x82\x01R\x7Fed before being archived\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[\x81`\x01\x85\x85`@Qa\x148\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x03\x81\x11\x15a\x14\x7FWa\x14\x7Fa\x1E\xD7V[\x02\x17\x90UP\x83\x83`@Qa\x14\x94\x92\x91\x90a#\xD7V[`@Q\x80\x91\x03\x90 \x7F@|\xB3\xAD\x05\xE6\x0E\xC4\x98\xFB9A|zOk\x82\xD5\xBA\x80\xF8/\xE5\x12\xA3{\x02\xC91\x81\xA2\xA1\x85\x85\x85`@Qa\x0C\xE3\x93\x92\x91\x90a%\xCCV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x15PW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`\0`\x01\x84\x84`@Qa\x15d\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16`\x03\x81\x11\x15a\x15\x88Wa\x15\x88a\x1E\xD7V[\x14a\x16\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FDrippie: drip with that name alr`D\x82\x01R\x7Feady exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[a\x16\"` \x82\x01\x82a%\xEFV[\x15a\x16\xBEW` \x81\x015\x15a\x16\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FDrippie: if allowing reentrant d`D\x82\x01R\x7Frip, must set interval to zero\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[a\x17yV[`\0\x81` \x015\x11a\x17yW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDrippie: interval must be greate\x90\x82\x01R\x7Fr than zero if drip is not reent`d\x82\x01R\x7Frant\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\0`\x01\x84\x84`@Qa\x17\x8D\x92\x91\x90a#\xD7V[\x90\x81R`@Q` \x91\x81\x90\x03\x82\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x81U\x91Pa\x17\xD4\x90\x83\x01\x83a%\xEFV[`\x01\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U` \x82\x015`\x02\x82\x01Ua\x18\x1F``\x83\x01`@\x84\x01a\x1D2V[`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x18s``\x83\x01\x83a&\x0CV[`\x04\x83\x01\x91a\x18\x83\x91\x90\x83a&\xC0V[P`\0[a\x18\x94`\x80\x84\x01\x84a'\xDBV[\x90P\x81\x10\x15a\x19\x07W`\x05\x82\x01a\x18\xAE`\x80\x85\x01\x85a'\xDBV[\x83\x81\x81\x10a\x18\xBEWa\x18\xBEa$NV[\x90P` \x02\x81\x01\x90a\x18\xD0\x91\x90a(CV[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x90\x92 \x90\x91`\x03\x02\x01a\x18\xF2\x82\x82a(wV[PP\x80\x80a\x18\xFF\x90a$\x16V[\x91PPa\x18\x87V[P\x83\x83`@Qa\x19\x18\x92\x91\x90a#\xD7V[`@Q\x80\x91\x03\x90 \x7F\xE3\x8D\x8D\x98\xE6\xCCf\xF6\xF5 \xD4\x83\xC6\xC5\xA8\x92\x89h\x1F\x89w\x99\xC4\xC2\x9Dv|\xF5~v\xD9\xA6\x85\x85\x85`@Qa\x0C\xE3\x93\x92\x91\x90a+jV[`\0\x80T``\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x19\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\x19\xFD\x91\x90a%\x97V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1A8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A=V[``\x91P[P\x90\x95\x90\x94P\x92PPPV[`\0\x80`\x01\x84\x84`@Qa\x1A^\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x90P`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x1A\x87Wa\x1A\x87a\x1E\xD7V[\x14a\x1B\x16W`@\x80Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FDrippie: selected drip does not `D\x82\x01R\x7Fexist or is not currently active`d\x82\x01R`\x84\x01a\x03\xB0V[`\x02\x81\x01T`\x06\x82\x01TB\x91a\x1B+\x91a,qV[\x11\x15a\x1B\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FDrippie: drip interval has not e`D\x82\x01R\x7Flapsed since last drip\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[`\x03\x81\x01T`@Q\x7F\xC6K;\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xC6K;\xB5\x90a\x1C\x13\x90`\x04\x80\x86\x01\x91\x01a,\x89V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CT\x91\x90a#gV[a\x1D\x06W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FDrippie: dripcheck failed so dri`D\x82\x01R\x7Fp is not yet ready to be trigger`d\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[P`\x01\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r|W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1DDW`\0\x80\xFD[\x815a\x1DO\x81a\x1D\x10V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1DkW`\0\x80\xFD[\x835a\x1Dv\x81a\x1D\x10V[\x92P` \x84\x015a\x1D\x86\x81a\x1D\x10V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xAAW`\0\x80\xFD[\x825a\x1D\xB5\x81a\x1D\x10V[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x1E\rWa\x1E\ra\x1D\xC3V[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1ESWa\x1ESa\x1D\xC3V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x1ElW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1E\x98W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xAFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1E\xC0W`\0\x80\xFD[a\x1E\xCF\x84\x825` \x84\x01a\x1D\xF2V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x04\x81\x10a\x1F=W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[`\0[\x83\x81\x10\x15a\x1F\\W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FDV[\x83\x81\x11\x15a\x1FkW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x1F\x89\x81` \x86\x01` \x86\x01a\x1FAV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[a\x1F\xC5\x81\x86a\x1F\x06V[`\0` `\x80\x81\x84\x01R\x85Q\x15\x15`\x80\x84\x01R\x80\x86\x01Q`\xA0\x84\x01R`@\x80\x87\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\xC0\x87\x01R``\x91P\x81\x89\x01Q`\xA0`\xE0\x88\x01Ra !a\x01 \x88\x01\x82a\x1FqV[`\x80\x8B\x01Q\x88\x82\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x01a\x01\0\x8A\x01R\x80Q\x80\x83R\x91\x92P\x86\x01\x90\x86\x83\x01\x90`\x05\x81\x90\x1B\x84\x01\x88\x01`\0[\x82\x81\x10\x15a \xD5W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x83\x03\x01\x84R\x84Q\x87\x81Q\x16\x83R\x8A\x81\x01Q\x89\x8C\x85\x01Ra \xB8\x8A\x85\x01\x82a\x1FqV[\x91\x8B\x01Q\x93\x8B\x01\x93\x90\x93R\x94\x8A\x01\x94\x93\x8A\x01\x93\x91P`\x01\x01a lV[P\x96\x8A\x01\x9B\x90\x9BRPPPP\x90\x93\x01\x93\x90\x93RP\x94\x93PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a!\x03W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\x1BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a!MW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!dW`\0\x80\xFD[a!p\x85\x82\x86\x01a \xF1V[\x90\x96\x90\x95P\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a!\x8DW`\0\x80\xFD[a\x1DO\x83\x835` \x85\x01a\x1D\xF2V[`\0\x80`\0``\x84\x86\x03\x12\x15a!\xB1W`\0\x80\xFD[\x835a!\xBC\x81a\x1D\x10V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xD8W`\0\x80\xFD[a!\xE4\x86\x82\x87\x01a!|V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x1E\xCF`@\x83\x01\x84a\x1FqV[`\0\x80`@\x83\x85\x03\x12\x15a\"#W`\0\x80\xFD[\x825a\".\x81a\x1D\x10V[\x91P` \x83\x015a\">\x81a\x1D\x10V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"^W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"uW`\0\x80\xFD[a\"\x81\x86\x82\x87\x01a \xF1V[\x90\x94P\x92PP` \x84\x015`\x04\x81\x10a\"\x99W`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\xB9W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD1W`\0\x80\xFD[a\"\xDD\x87\x83\x88\x01a \xF1V[\x90\x95P\x93P` \x86\x015\x91P\x80\x82\x11\x15a\"\xF6W`\0\x80\xFD[P\x84\x01`\xA0\x81\x87\x03\x12\x15a\"\x99W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a#\x1CW`\0\x80\xFD[\x825a#'\x81a\x1D\x10V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#CW`\0\x80\xFD[a#O\x85\x82\x86\x01a!|V[\x91PP\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\r|W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#yW`\0\x80\xFD[\x81Qa\x1DO\x81a#YV[`\x01\x81\x81\x1C\x90\x82\x16\x80a#\x98W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a#\xD1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a$GWa$Ga#\xE7V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x80\x83Ta$\x8B\x81a#\x84V[`\x01\x82\x81\x16\x80\x15a$\xA3W`\x01\x81\x14a$\xD6Wa%\x05V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa%\x05V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a$\xFCW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a$\xE3V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0a%n``\x83\x01\x86\x88a%\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16` \x83\x01RP`@\x01R\x92\x91PPV[`\0\x82Qa%\xA9\x81\x84` \x87\x01a\x1FAV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a%\xC5W`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a%\xE0`@\x83\x01\x85\x87a%\x11V[\x90Pa\x1E\xCF` \x83\x01\x84a\x1F\x06V[`\0` \x82\x84\x03\x12\x15a&\x01W`\0\x80\xFD[\x815a\x1DO\x81a#YV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a&AW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\\W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a!3W`\0\x80\xFD[`\x1F\x82\x11\x15a&\xBBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a&\x98WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a&\xB7W\x82\x81U`\x01\x01a&\xA4V[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a&\xD8Wa&\xD8a\x1D\xC3V[a&\xEC\x83a&\xE6\x83Ta#\x84V[\x83a&qV[`\0`\x1F\x84\x11`\x01\x81\x14a'>W`\0\x85\x15a'\x08WP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua'\xD4V[`\0\x83\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x90\x83[\x82\x81\x10\x15a'\x8DW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a'mV[P\x86\x82\x10\x15a'\xC8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a(\x10W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(+W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a!3W`\0\x80\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x836\x03\x01\x81\x12a%\xA9W`\0\x80\xFD[\x815a(\x82\x81a\x1D\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x16\x17\x82UP`\x01\x80\x82\x01` \x80\x85\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x866\x03\x01\x81\x12a(\xFCW`\0\x80\xFD[\x85\x01\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x15W`\0\x80\xFD[\x806\x03\x83\x83\x01\x13\x15a)&W`\0\x80\xFD[a):\x81a)4\x86Ta#\x84V[\x86a&qV[`\0`\x1F\x82\x11`\x01\x81\x14a)\x8EW`\0\x83\x15a)XWP\x83\x82\x01\x85\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x86Ua*#V[`\0\x86\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x84\x16\x90\x83[\x82\x81\x10\x15a)\xDCW\x86\x85\x01\x88\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a)\xBDV[P\x84\x82\x10\x15a*\x19W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x86`\x03\x1B\x16\x1C\x19\x87\x85\x88\x01\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPP`@\x82\x015`\x02\x82\x01UPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a*mW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\x8DW`\0\x80\xFD[\x806\x03\x82\x13\x15a!3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15a+\\W\x83\x85\x03\x8AR\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x896\x03\x01\x81\x12a*\xF5W\x82\x83\xFD[\x88\x01``\x815a+\x04\x81a\x1D\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87Ra+(\x82\x89\x01\x83a*8V[\x82\x8A\x8A\x01Ra+:\x83\x8A\x01\x82\x84a%\x11V[`@\x94\x85\x015\x99\x90\x94\x01\x98\x90\x98RPP\x99\x86\x01\x99\x94P\x91\x85\x01\x91`\x01\x01a*\xB7V[P\x92\x98\x97PPPPPPPPV[`@\x81R`\0a+~`@\x83\x01\x85\x87a%\x11V[\x82\x81\x03` \x84\x01R\x835a+\x91\x81a#YV[\x15\x15\x81R` \x84\x81\x015\x90\x82\x01R`@\x84\x015a+\xAD\x81a\x1D\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x82\x01Ra+\xD5``\x85\x01\x85a*8V[`\xA0``\x84\x01Ra+\xEA`\xA0\x84\x01\x82\x84a%\x11V[\x91PP`\x80\x85\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x866\x03\x01\x81\x12a,\"W`\0\x80\xFD[\x85\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,?W`\0\x80\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a,QW`\0\x80\xFD[\x83\x83\x03`\x80\x85\x01Ra,d\x83\x82\x84a*\x9CV[\x99\x98PPPPPPPPPV[`\0\x82\x19\x82\x11\x15a,\x84Wa,\x84a#\xE7V[P\x01\x90V[`\0` \x80\x83R`\0\x84Ta,\x9D\x81a#\x84V[\x80\x84\x87\x01R`@`\x01\x80\x84\x16`\0\x81\x14a,\xBEW`\x01\x81\x14a,\xF6Wa-$V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x83\x8A\x01R\x82\x84\x15\x15`\x05\x1B\x8A\x01\x01\x95Pa-$V[\x89`\0R\x86`\0 `\0[\x85\x81\x10\x15a-\x1CW\x81T\x8B\x82\x01\x86\x01R\x90\x83\x01\x90\x88\x01a-\x01V[\x8A\x01\x84\x01\x96PP[P\x93\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static DRIPPIE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xE1W`\x005`\xE0\x1C\x80cn-D\xAE\x11a\0\x7FW\x80c\x9B\xC9M\x01\x11a\0YW\x80c\x9B\xC9M\x01\x14a\x02\xB0W\x80c\xE5Q\xCD\xAA\x14a\x02\xD0W\x80c\xED\xEEb9\x14a\x02\xF0W\x80c\xFC>>\xBA\x14a\x03\x03W`\0\x80\xFD[\x80cn-D\xAE\x14a\x02\x1DW\x80c\x8D\xA5\xCB[\x14a\x02>W\x80c\x94V\xFB\xCC\x14a\x02\x90W`\0\x80\xFD[\x80cG\x82\xF7y\x11a\0\xBBW\x80cG\x82\xF7y\x14a\x01\x84W\x80cM\x7F\xBAn\x14a\x01\xA4W\x80cg\x14\x8C\xD2\x14a\x01\xDDW\x80ci\r\x83 \x14a\x01\xFDW`\0\x80\xFD[\x80c\x13\xAF@5\x14a\x01\"W\x80c@%\xFE\xB2\x14a\x01DW\x80cD\0L\xC1\x14a\x01dW`\0\x80\xFD[6a\x01\x1DW`@Q4\x81R3\x90\x7FA\x03%~\xAA\xC9\x83\xCAy\xA7\r(\xF9\r\xFCO\xA1ka\x9B\xB0\xC1~\xE7\xCA\xB0\xD4\x03L'\x96$\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[`\0\x80\xFD[4\x80\x15a\x01.W`\0\x80\xFD[Pa\x01Ba\x01=6`\x04a\x1D2V[a\x033V[\0[4\x80\x15a\x01PW`\0\x80\xFD[Pa\x01Ba\x01_6`\x04a\x1DVV[a\x04)V[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01Ba\x01\x7F6`\x04a\x1DVV[a\x05\xBBV[4\x80\x15a\x01\x90W`\0\x80\xFD[Pa\x01Ba\x01\x9F6`\x04a\x1D\x97V[a\x07LV[4\x80\x15a\x01\xB0W`\0\x80\xFD[Pa\x01\xC4a\x01\xBF6`\x04a\x1E\x86V[a\x08\x9CV[`@Qa\x01\xD4\x94\x93\x92\x91\x90a\x1F\xBBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01Ba\x01\xF86`\x04a!:V[a\n\xCEV[4\x80\x15a\x02\tW`\0\x80\xFD[Pa\x01Ba\x02\x186`\x04a\x1D2V[a\x0C\xF1V[a\x020a\x02+6`\x04a!\x9CV[a\r\x7FV[`@Qa\x01\xD4\x92\x91\x90a!\xF5V[4\x80\x15a\x02JW`\0\x80\xFD[P`\0Ta\x02k\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xD4V[4\x80\x15a\x02\x9CW`\0\x80\xFD[Pa\x01Ba\x02\xAB6`\x04a\"\x10V[a\x0EyV[4\x80\x15a\x02\xBCW`\0\x80\xFD[Pa\x01Ba\x02\xCB6`\x04a\"IV[a\x0F\x94V[4\x80\x15a\x02\xDCW`\0\x80\xFD[Pa\x01Ba\x02\xEB6`\x04a\"\xA4V[a\x14\xCFV[a\x020a\x02\xFE6`\x04a#\tV[a\x19SV[4\x80\x15a\x03\x0FW`\0\x80\xFD[Pa\x03#a\x03\x1E6`\x04a!:V[a\x1AIV[`@Q\x90\x15\x15\x81R` \x01a\x01\xD4V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x82\x92\xFC\xE1\x8F\xA6\x9E\xDFM\xB7\xB9N\xA2\xE5\x82A\xDF\n\xE5\x7F\x97\xE0\xA6\xC9\xB2\x90g\x02\x8B\xF9-v\x91\x90\xA3PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R\x84\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05 W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x054W=`\0\x80>=`\0\xFD[PPPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F0\xB4x\xA5\xE1\x96\xE5X\x86\"\x8A\xA8{\xA7J}\xFE\xBAe^\nM{\xA2u\xEA\xBF\xC2*\xAB\xB7\xA8\x84`@Qa\x05\xAE\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD5\x91\x90a#gV[P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fk\0\xF1\xC7\x88?\x05;\xA8>\x90\x7F\xD1\x96[\"\xFF\xFE<A\x118>r_\x04c\x8AVl\xDB\xFA\x84`@Qa\x05\xAE\x91\x81R` \x01\x90V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\xCDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08'W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08,V[``\x91P[PP\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1F\x12\xAA\x8BmI-\xD9\xB9\x8E+\0\xB0\xB2\x080\xC2\xA7\xDE\xD6Z\xFA\xC1;`\xD1i\xA04\xAE\x90\xBC\x84`@Qa\x08\x8F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x01\x80\x83R\x93\x83\x01\x94\x83\x01\x94\x90\x94 \x93\x90R\x82T`@\x80Q`\xA0\x81\x01\x82R\x93\x85\x01\x80T`\xFF\x90\x81\x16\x15\x15\x86R`\x02\x87\x01T\x94\x86\x01\x94\x90\x94R`\x03\x86\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x85\x01\x91\x90\x91R`\x04\x85\x01\x80T\x93\x90\x92\x16\x94\x93\x92\x90\x91``\x84\x01\x91a\t\x1E\x90a#\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tJ\x90a#\x84V[\x80\x15a\t\x97W\x80`\x1F\x10a\tlWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tzW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\n\xB6W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x90\x91R`\x03\x85\x02\x90\x91\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\n\x1B\x90a#\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nG\x90a#\x84V[\x80\x15a\n\x94W\x80`\x1F\x10a\niWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x94V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nwW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\t\xC5V[PPP\x91RPP`\x06\x82\x01T`\x07\x90\x92\x01T\x90\x91\x90\x84V[`\0`\x01\x83\x83`@Qa\n\xE2\x92\x91\x90a#\xD7V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x90Pa\n\xFC\x83\x83a\x1AIV[PB`\x06\x82\x01U`\x07\x81\x01\x80T\x90`\0a\x0B\x15\x83a$\x16V[\x90\x91UPP`\x05\x81\x01T`\0[\x81\x81\x10\x15a\x0C\x95W`\0\x83`\x01\x01`\x04\x01\x82\x81T\x81\x10a\x0BDWa\x0BDa$NV[`\0\x91\x82R` \x82 `\x03\x90\x91\x02\x01\x80T`\x02\x82\x01T`@Q\x92\x94Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x0B\x87\x90`\x01\x86\x01\x90a$}V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xC9V[``\x91P[PP\x90P\x80a\x0C\x80W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FDrippie: drip was unsuccessful, `D\x82\x01R\x7Fplease check your configuration `d\x82\x01R\x7Ffor mistakes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[PP\x80\x80a\x0C\x8D\x90a$\x16V[\x91PPa\x0B\"V[P\x83\x83`@Qa\x0C\xA6\x92\x91\x90a#\xD7V[`@Q\x80\x91\x03\x90 \x7F\xEA!CT\x19\xAA\xD9\xC5J\x9D\x90\xE2R+o`\xBDVd\x01\xF3o\xCE\xF6a\xF5\xF5\xA2\x8C\xC0\xD2\xC6\x85\x853B`@Qa\x0C\xE3\x94\x93\x92\x91\x90a%ZV[`@Q\x80\x91\x03\x90\xA2PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\rrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[a\r|\x81Ga\x07LV[PV[`\0\x80T``\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\x04W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x85`@Qa\x0E*\x91\x90a%\x97V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0EgW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0ElV[``\x91P[P\x90\x96\x90\x95P\x93PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\xFAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Ra\x0F\x90\x90\x83\x90\x83\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FlW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x7F\x91\x90a%\xB3V[PPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x10\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`\0\x81`\x03\x81\x11\x15a\x10)Wa\x10)a\x1E\xD7V[\x03a\x10\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDrippie: drip status can never b`D\x82\x01R\x7Fe set back to NONE after creatio`d\x82\x01R\x7Fn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\0`\x01\x84\x84`@Qa\x10\xF0\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x90P`\0\x81`\x03\x81\x11\x15a\x11\x19Wa\x11\x19a\x1E\xD7V[\x03a\x11\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDrippie: drip with that name doe`D\x82\x01R\x7Fs not exist and cannot be update`d\x82\x01R\x7Fd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\x03\x81`\x03\x81\x11\x15a\x11\xE0Wa\x11\xE0a\x1E\xD7V[\x03a\x12\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDrippie: drip with that name has\x90\x82\x01R\x7F been archived and cannot be upd`d\x82\x01R\x7Fated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[\x81`\x03\x81\x11\x15a\x12\xA6Wa\x12\xA6a\x1E\xD7V[\x81`\x03\x81\x11\x15a\x12\xB8Wa\x12\xB8a\x1E\xD7V[\x03a\x13kW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDrippie: cannot set drip status `D\x82\x01R\x7Fto the same status as its curren`d\x82\x01R\x7Ft status\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\x03\x82`\x03\x81\x11\x15a\x13\x7FWa\x13\x7Fa\x1E\xD7V[\x03a\x14%W`\x01\x81`\x03\x81\x11\x15a\x13\x98Wa\x13\x98a\x1E\xD7V[\x14a\x14%W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDrippie: drip must first be paus`D\x82\x01R\x7Fed before being archived\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[\x81`\x01\x85\x85`@Qa\x148\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x03\x81\x11\x15a\x14\x7FWa\x14\x7Fa\x1E\xD7V[\x02\x17\x90UP\x83\x83`@Qa\x14\x94\x92\x91\x90a#\xD7V[`@Q\x80\x91\x03\x90 \x7F@|\xB3\xAD\x05\xE6\x0E\xC4\x98\xFB9A|zOk\x82\xD5\xBA\x80\xF8/\xE5\x12\xA3{\x02\xC91\x81\xA2\xA1\x85\x85\x85`@Qa\x0C\xE3\x93\x92\x91\x90a%\xCCV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x15PW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[`\0`\x01\x84\x84`@Qa\x15d\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16`\x03\x81\x11\x15a\x15\x88Wa\x15\x88a\x1E\xD7V[\x14a\x16\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FDrippie: drip with that name alr`D\x82\x01R\x7Feady exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[a\x16\"` \x82\x01\x82a%\xEFV[\x15a\x16\xBEW` \x81\x015\x15a\x16\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FDrippie: if allowing reentrant d`D\x82\x01R\x7Frip, must set interval to zero\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[a\x17yV[`\0\x81` \x015\x11a\x17yW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDrippie: interval must be greate\x90\x82\x01R\x7Fr than zero if drip is not reent`d\x82\x01R\x7Frant\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[`\0`\x01\x84\x84`@Qa\x17\x8D\x92\x91\x90a#\xD7V[\x90\x81R`@Q` \x91\x81\x90\x03\x82\x01\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x81U\x91Pa\x17\xD4\x90\x83\x01\x83a%\xEFV[`\x01\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U` \x82\x015`\x02\x82\x01Ua\x18\x1F``\x83\x01`@\x84\x01a\x1D2V[`\x03\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x18s``\x83\x01\x83a&\x0CV[`\x04\x83\x01\x91a\x18\x83\x91\x90\x83a&\xC0V[P`\0[a\x18\x94`\x80\x84\x01\x84a'\xDBV[\x90P\x81\x10\x15a\x19\x07W`\x05\x82\x01a\x18\xAE`\x80\x85\x01\x85a'\xDBV[\x83\x81\x81\x10a\x18\xBEWa\x18\xBEa$NV[\x90P` \x02\x81\x01\x90a\x18\xD0\x91\x90a(CV[\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x90\x92 \x90\x91`\x03\x02\x01a\x18\xF2\x82\x82a(wV[PP\x80\x80a\x18\xFF\x90a$\x16V[\x91PPa\x18\x87V[P\x83\x83`@Qa\x19\x18\x92\x91\x90a#\xD7V[`@Q\x80\x91\x03\x90 \x7F\xE3\x8D\x8D\x98\xE6\xCCf\xF6\xF5 \xD4\x83\xC6\xC5\xA8\x92\x89h\x1F\x89w\x99\xC4\xC2\x9Dv|\xF5~v\xD9\xA6\x85\x85\x85`@Qa\x0C\xE3\x93\x92\x91\x90a+jV[`\0\x80T``\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x19\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\x19\xFD\x91\x90a%\x97V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1A8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A=V[``\x91P[P\x90\x95\x90\x94P\x92PPPV[`\0\x80`\x01\x84\x84`@Qa\x1A^\x92\x91\x90a#\xD7V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x90P`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x1A\x87Wa\x1A\x87a\x1E\xD7V[\x14a\x1B\x16W`@\x80Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FDrippie: selected drip does not `D\x82\x01R\x7Fexist or is not currently active`d\x82\x01R`\x84\x01a\x03\xB0V[`\x02\x81\x01T`\x06\x82\x01TB\x91a\x1B+\x91a,qV[\x11\x15a\x1B\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FDrippie: drip interval has not e`D\x82\x01R\x7Flapsed since last drip\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\xB0V[`\x03\x81\x01T`@Q\x7F\xC6K;\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xC6K;\xB5\x90a\x1C\x13\x90`\x04\x80\x86\x01\x91\x01a,\x89V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CT\x91\x90a#gV[a\x1D\x06W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FDrippie: dripcheck failed so dri`D\x82\x01R\x7Fp is not yet ready to be trigger`d\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\xB0V[P`\x01\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r|W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1DDW`\0\x80\xFD[\x815a\x1DO\x81a\x1D\x10V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1DkW`\0\x80\xFD[\x835a\x1Dv\x81a\x1D\x10V[\x92P` \x84\x015a\x1D\x86\x81a\x1D\x10V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1D\xAAW`\0\x80\xFD[\x825a\x1D\xB5\x81a\x1D\x10V[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x1E\rWa\x1E\ra\x1D\xC3V[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1ESWa\x1ESa\x1D\xC3V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x1ElW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1E\x98W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xAFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1E\xC0W`\0\x80\xFD[a\x1E\xCF\x84\x825` \x84\x01a\x1D\xF2V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x04\x81\x10a\x1F=W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[`\0[\x83\x81\x10\x15a\x1F\\W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FDV[\x83\x81\x11\x15a\x1FkW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x1F\x89\x81` \x86\x01` \x86\x01a\x1FAV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[a\x1F\xC5\x81\x86a\x1F\x06V[`\0` `\x80\x81\x84\x01R\x85Q\x15\x15`\x80\x84\x01R\x80\x86\x01Q`\xA0\x84\x01R`@\x80\x87\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\xC0\x87\x01R``\x91P\x81\x89\x01Q`\xA0`\xE0\x88\x01Ra !a\x01 \x88\x01\x82a\x1FqV[`\x80\x8B\x01Q\x88\x82\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x01a\x01\0\x8A\x01R\x80Q\x80\x83R\x91\x92P\x86\x01\x90\x86\x83\x01\x90`\x05\x81\x90\x1B\x84\x01\x88\x01`\0[\x82\x81\x10\x15a \xD5W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x83\x03\x01\x84R\x84Q\x87\x81Q\x16\x83R\x8A\x81\x01Q\x89\x8C\x85\x01Ra \xB8\x8A\x85\x01\x82a\x1FqV[\x91\x8B\x01Q\x93\x8B\x01\x93\x90\x93R\x94\x8A\x01\x94\x93\x8A\x01\x93\x91P`\x01\x01a lV[P\x96\x8A\x01\x9B\x90\x9BRPPPP\x90\x93\x01\x93\x90\x93RP\x94\x93PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a!\x03W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\x1BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a!MW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!dW`\0\x80\xFD[a!p\x85\x82\x86\x01a \xF1V[\x90\x96\x90\x95P\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a!\x8DW`\0\x80\xFD[a\x1DO\x83\x835` \x85\x01a\x1D\xF2V[`\0\x80`\0``\x84\x86\x03\x12\x15a!\xB1W`\0\x80\xFD[\x835a!\xBC\x81a\x1D\x10V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xD8W`\0\x80\xFD[a!\xE4\x86\x82\x87\x01a!|V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x1E\xCF`@\x83\x01\x84a\x1FqV[`\0\x80`@\x83\x85\x03\x12\x15a\"#W`\0\x80\xFD[\x825a\".\x81a\x1D\x10V[\x91P` \x83\x015a\">\x81a\x1D\x10V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"^W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"uW`\0\x80\xFD[a\"\x81\x86\x82\x87\x01a \xF1V[\x90\x94P\x92PP` \x84\x015`\x04\x81\x10a\"\x99W`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\xB9W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD1W`\0\x80\xFD[a\"\xDD\x87\x83\x88\x01a \xF1V[\x90\x95P\x93P` \x86\x015\x91P\x80\x82\x11\x15a\"\xF6W`\0\x80\xFD[P\x84\x01`\xA0\x81\x87\x03\x12\x15a\"\x99W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a#\x1CW`\0\x80\xFD[\x825a#'\x81a\x1D\x10V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#CW`\0\x80\xFD[a#O\x85\x82\x86\x01a!|V[\x91PP\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\r|W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#yW`\0\x80\xFD[\x81Qa\x1DO\x81a#YV[`\x01\x81\x81\x1C\x90\x82\x16\x80a#\x98W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a#\xD1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a$GWa$Ga#\xE7V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x80\x83Ta$\x8B\x81a#\x84V[`\x01\x82\x81\x16\x80\x15a$\xA3W`\x01\x81\x14a$\xD6Wa%\x05V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa%\x05V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a$\xFCW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a$\xE3V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R`\0a%n``\x83\x01\x86\x88a%\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16` \x83\x01RP`@\x01R\x92\x91PPV[`\0\x82Qa%\xA9\x81\x84` \x87\x01a\x1FAV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a%\xC5W`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a%\xE0`@\x83\x01\x85\x87a%\x11V[\x90Pa\x1E\xCF` \x83\x01\x84a\x1F\x06V[`\0` \x82\x84\x03\x12\x15a&\x01W`\0\x80\xFD[\x815a\x1DO\x81a#YV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a&AW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\\W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a!3W`\0\x80\xFD[`\x1F\x82\x11\x15a&\xBBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a&\x98WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a&\xB7W\x82\x81U`\x01\x01a&\xA4V[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a&\xD8Wa&\xD8a\x1D\xC3V[a&\xEC\x83a&\xE6\x83Ta#\x84V[\x83a&qV[`\0`\x1F\x84\x11`\x01\x81\x14a'>W`\0\x85\x15a'\x08WP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua'\xD4V[`\0\x83\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x90\x83[\x82\x81\x10\x15a'\x8DW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a'mV[P\x86\x82\x10\x15a'\xC8W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a(\x10W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(+W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a!3W`\0\x80\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x836\x03\x01\x81\x12a%\xA9W`\0\x80\xFD[\x815a(\x82\x81a\x1D\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83T\x16\x17\x82UP`\x01\x80\x82\x01` \x80\x85\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x866\x03\x01\x81\x12a(\xFCW`\0\x80\xFD[\x85\x01\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x15W`\0\x80\xFD[\x806\x03\x83\x83\x01\x13\x15a)&W`\0\x80\xFD[a):\x81a)4\x86Ta#\x84V[\x86a&qV[`\0`\x1F\x82\x11`\x01\x81\x14a)\x8EW`\0\x83\x15a)XWP\x83\x82\x01\x85\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x86Ua*#V[`\0\x86\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x84\x16\x90\x83[\x82\x81\x10\x15a)\xDCW\x86\x85\x01\x88\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a)\xBDV[P\x84\x82\x10\x15a*\x19W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x86`\x03\x1B\x16\x1C\x19\x87\x85\x88\x01\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPP`@\x82\x015`\x02\x82\x01UPPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a*mW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\x8DW`\0\x80\xFD[\x806\x03\x82\x13\x15a!3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15a+\\W\x83\x85\x03\x8AR\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x896\x03\x01\x81\x12a*\xF5W\x82\x83\xFD[\x88\x01``\x815a+\x04\x81a\x1D\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87Ra+(\x82\x89\x01\x83a*8V[\x82\x8A\x8A\x01Ra+:\x83\x8A\x01\x82\x84a%\x11V[`@\x94\x85\x015\x99\x90\x94\x01\x98\x90\x98RPP\x99\x86\x01\x99\x94P\x91\x85\x01\x91`\x01\x01a*\xB7V[P\x92\x98\x97PPPPPPPPV[`@\x81R`\0a+~`@\x83\x01\x85\x87a%\x11V[\x82\x81\x03` \x84\x01R\x835a+\x91\x81a#YV[\x15\x15\x81R` \x84\x81\x015\x90\x82\x01R`@\x84\x015a+\xAD\x81a\x1D\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x82\x01Ra+\xD5``\x85\x01\x85a*8V[`\xA0``\x84\x01Ra+\xEA`\xA0\x84\x01\x82\x84a%\x11V[\x91PP`\x80\x85\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x866\x03\x01\x81\x12a,\"W`\0\x80\xFD[\x85\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,?W`\0\x80\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a,QW`\0\x80\xFD[\x83\x83\x03`\x80\x85\x01Ra,d\x83\x82\x84a*\x9CV[\x99\x98PPPPPPPPPV[`\0\x82\x19\x82\x11\x15a,\x84Wa,\x84a#\xE7V[P\x01\x90V[`\0` \x80\x83R`\0\x84Ta,\x9D\x81a#\x84V[\x80\x84\x87\x01R`@`\x01\x80\x84\x16`\0\x81\x14a,\xBEW`\x01\x81\x14a,\xF6Wa-$V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x83\x8A\x01R\x82\x84\x15\x15`\x05\x1B\x8A\x01\x01\x95Pa-$V[\x89`\0R\x86`\0 `\0[\x85\x81\x10\x15a-\x1CW\x81T\x8B\x82\x01\x86\x01R\x90\x83\x01\x90\x88\x01a-\x01V[\x8A\x01\x84\x01\x96PP[P\x93\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static DRIPPIE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Drippie<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Drippie<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Drippie<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Drippie<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Drippie<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Drippie)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Drippie<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DRIPPIE_ABI.clone(),
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
                DRIPPIE_ABI.clone(),
                DRIPPIE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CALL` (0x6e2d44ae) function
        pub fn call(
            &self,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([110, 45, 68, 174], (target, data, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DELEGATECALL` (0xedee6239) function
        pub fn delegatecall(
            &self,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([237, 238, 98, 57], (target, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `create` (0xe551cdaa) function
        pub fn create(
            &self,
            name: ::std::string::String,
            config: DripConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 81, 205, 170], (name, config))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `drip` (0x67148cd2) function
        pub fn drip(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 20, 140, 210], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `drips` (0x4d7fba6e) function
        pub fn drips(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, DripConfig, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([77, 127, 186, 110], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executable` (0xfc3e3eba) function
        pub fn executable(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 62, 62, 186], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOwner` (0x13af4035) function
        pub fn set_owner(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `status` (0x9bc94d01) function
        pub fn status(
            &self,
            name: ::std::string::String,
            status: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 201, 77, 1], (name, status))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC20` (0x44004cc1) function
        pub fn withdraw_erc_20_with_asset_and_to(
            &self,
            asset: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 0, 76, 193], (asset, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC20` (0x9456fbcc) function
        pub fn withdraw_erc20(
            &self,
            asset: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 86, 251, 204], (asset, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC721` (0x4025feb2) function
        pub fn withdraw_erc721(
            &self,
            asset: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 37, 254, 178], (asset, to, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawETH` (0x4782f779) function
        pub fn withdraw_eth_with_amount(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 130, 247, 121], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawETH` (0x690d8320) function
        pub fn withdraw_eth(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 13, 131, 32], to)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DripCreated` event
        pub fn drip_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DripCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DripExecuted` event
        pub fn drip_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DripExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DripStatusUpdated` event
        pub fn drip_status_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DripStatusUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnerUpdated` event
        pub fn owner_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReceivedETH` event
        pub fn received_eth_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReceivedETHFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrewERC20` event
        pub fn withdrew_erc20_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrewERC20Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrewERC721` event
        pub fn withdrew_erc721_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrewERC721Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrewETH` event
        pub fn withdrew_eth_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrewETHFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DrippieEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Drippie<M> {
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
    #[ethevent(
        name = "DripCreated",
        abi = "DripCreated(string,string,(bool,uint256,address,bytes,(address,bytes,uint256)[]))"
    )]
    pub struct DripCreatedFilter {
        #[ethevent(indexed)]
        pub nameref: ::ethers::core::types::H256,
        pub name: ::std::string::String,
        pub config: DripConfig,
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
        name = "DripExecuted",
        abi = "DripExecuted(string,string,address,uint256)"
    )]
    pub struct DripExecutedFilter {
        #[ethevent(indexed)]
        pub nameref: ::ethers::core::types::H256,
        pub name: ::std::string::String,
        pub executor: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
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
        name = "DripStatusUpdated",
        abi = "DripStatusUpdated(string,string,uint8)"
    )]
    pub struct DripStatusUpdatedFilter {
        #[ethevent(indexed)]
        pub nameref: ::ethers::core::types::H256,
        pub name: ::std::string::String,
        pub status: u8,
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
    #[ethevent(name = "OwnerUpdated", abi = "OwnerUpdated(address,address)")]
    pub struct OwnerUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "ReceivedETH", abi = "ReceivedETH(address,uint256)")]
    pub struct ReceivedETHFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "WithdrewERC20",
        abi = "WithdrewERC20(address,address,address,uint256)"
    )]
    pub struct WithdrewERC20Filter {
        #[ethevent(indexed)]
        pub withdrawer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "WithdrewERC721",
        abi = "WithdrewERC721(address,address,address,uint256)"
    )]
    pub struct WithdrewERC721Filter {
        #[ethevent(indexed)]
        pub withdrawer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
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
    #[ethevent(name = "WithdrewETH", abi = "WithdrewETH(address,address,uint256)")]
    pub struct WithdrewETHFilter {
        #[ethevent(indexed)]
        pub withdrawer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DrippieEvents {
        DripCreatedFilter(DripCreatedFilter),
        DripExecutedFilter(DripExecutedFilter),
        DripStatusUpdatedFilter(DripStatusUpdatedFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        ReceivedETHFilter(ReceivedETHFilter),
        WithdrewERC20Filter(WithdrewERC20Filter),
        WithdrewERC721Filter(WithdrewERC721Filter),
        WithdrewETHFilter(WithdrewETHFilter),
    }
    impl ::ethers::contract::EthLogDecode for DrippieEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DripCreatedFilter::decode_log(log) {
                return Ok(DrippieEvents::DripCreatedFilter(decoded));
            }
            if let Ok(decoded) = DripExecutedFilter::decode_log(log) {
                return Ok(DrippieEvents::DripExecutedFilter(decoded));
            }
            if let Ok(decoded) = DripStatusUpdatedFilter::decode_log(log) {
                return Ok(DrippieEvents::DripStatusUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(DrippieEvents::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ReceivedETHFilter::decode_log(log) {
                return Ok(DrippieEvents::ReceivedETHFilter(decoded));
            }
            if let Ok(decoded) = WithdrewERC20Filter::decode_log(log) {
                return Ok(DrippieEvents::WithdrewERC20Filter(decoded));
            }
            if let Ok(decoded) = WithdrewERC721Filter::decode_log(log) {
                return Ok(DrippieEvents::WithdrewERC721Filter(decoded));
            }
            if let Ok(decoded) = WithdrewETHFilter::decode_log(log) {
                return Ok(DrippieEvents::WithdrewETHFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DrippieEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DripCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DripExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DripStatusUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReceivedETHFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrewERC20Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrewERC721Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrewETHFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DripCreatedFilter> for DrippieEvents {
        fn from(value: DripCreatedFilter) -> Self {
            Self::DripCreatedFilter(value)
        }
    }
    impl ::core::convert::From<DripExecutedFilter> for DrippieEvents {
        fn from(value: DripExecutedFilter) -> Self {
            Self::DripExecutedFilter(value)
        }
    }
    impl ::core::convert::From<DripStatusUpdatedFilter> for DrippieEvents {
        fn from(value: DripStatusUpdatedFilter) -> Self {
            Self::DripStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnerUpdatedFilter> for DrippieEvents {
        fn from(value: OwnerUpdatedFilter) -> Self {
            Self::OwnerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ReceivedETHFilter> for DrippieEvents {
        fn from(value: ReceivedETHFilter) -> Self {
            Self::ReceivedETHFilter(value)
        }
    }
    impl ::core::convert::From<WithdrewERC20Filter> for DrippieEvents {
        fn from(value: WithdrewERC20Filter) -> Self {
            Self::WithdrewERC20Filter(value)
        }
    }
    impl ::core::convert::From<WithdrewERC721Filter> for DrippieEvents {
        fn from(value: WithdrewERC721Filter) -> Self {
            Self::WithdrewERC721Filter(value)
        }
    }
    impl ::core::convert::From<WithdrewETHFilter> for DrippieEvents {
        fn from(value: WithdrewETHFilter) -> Self {
            Self::WithdrewETHFilter(value)
        }
    }
    ///Container type for all input parameters for the `CALL` function with signature `CALL(address,bytes,uint256)` and selector `0x6e2d44ae`
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
    #[ethcall(name = "CALL", abi = "CALL(address,bytes,uint256)")]
    pub struct CallCall {
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `DELEGATECALL` function with signature `DELEGATECALL(address,bytes)` and selector `0xedee6239`
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
    #[ethcall(name = "DELEGATECALL", abi = "DELEGATECALL(address,bytes)")]
    pub struct DelegatecallCall {
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `create` function with signature `create(string,(bool,uint256,address,bytes,(address,bytes,uint256)[]))` and selector `0xe551cdaa`
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
        name = "create",
        abi = "create(string,(bool,uint256,address,bytes,(address,bytes,uint256)[]))"
    )]
    pub struct CreateCall {
        pub name: ::std::string::String,
        pub config: DripConfig,
    }
    ///Container type for all input parameters for the `drip` function with signature `drip(string)` and selector `0x67148cd2`
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
    #[ethcall(name = "drip", abi = "drip(string)")]
    pub struct DripCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `drips` function with signature `drips(string)` and selector `0x4d7fba6e`
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
    #[ethcall(name = "drips", abi = "drips(string)")]
    pub struct DripsCall(pub ::std::string::String);
    ///Container type for all input parameters for the `executable` function with signature `executable(string)` and selector `0xfc3e3eba`
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
    #[ethcall(name = "executable", abi = "executable(string)")]
    pub struct ExecutableCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `0x13af4035`
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
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `status` function with signature `status(string,uint8)` and selector `0x9bc94d01`
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
    #[ethcall(name = "status", abi = "status(string,uint8)")]
    pub struct StatusCall {
        pub name: ::std::string::String,
        pub status: u8,
    }
    ///Container type for all input parameters for the `withdrawERC20` function with signature `withdrawERC20(address,address,uint256)` and selector `0x44004cc1`
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
    #[ethcall(name = "withdrawERC20", abi = "withdrawERC20(address,address,uint256)")]
    pub struct WithdrawErc20WithAssetAndToCall {
        pub asset: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawERC20` function with signature `withdrawERC20(address,address)` and selector `0x9456fbcc`
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
    #[ethcall(name = "withdrawERC20", abi = "withdrawERC20(address,address)")]
    pub struct WithdrawERC20Call {
        pub asset: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawERC721` function with signature `withdrawERC721(address,address,uint256)` and selector `0x4025feb2`
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
    #[ethcall(name = "withdrawERC721", abi = "withdrawERC721(address,address,uint256)")]
    pub struct WithdrawERC721Call {
        pub asset: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawETH` function with signature `withdrawETH(address,uint256)` and selector `0x4782f779`
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
    #[ethcall(name = "withdrawETH", abi = "withdrawETH(address,uint256)")]
    pub struct WithdrawEthWithAmountCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawETH` function with signature `withdrawETH(address)` and selector `0x690d8320`
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
    #[ethcall(name = "withdrawETH", abi = "withdrawETH(address)")]
    pub struct WithdrawETHCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DrippieCalls {
        Call(CallCall),
        Delegatecall(DelegatecallCall),
        Create(CreateCall),
        Drip(DripCall),
        Drips(DripsCall),
        Executable(ExecutableCall),
        Owner(OwnerCall),
        SetOwner(SetOwnerCall),
        Status(StatusCall),
        WithdrawErc20WithAssetAndTo(WithdrawErc20WithAssetAndToCall),
        WithdrawERC20(WithdrawERC20Call),
        WithdrawERC721(WithdrawERC721Call),
        WithdrawEthWithAmount(WithdrawEthWithAmountCall),
        WithdrawETH(WithdrawETHCall),
    }
    impl ::ethers::core::abi::AbiDecode for DrippieCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Call(decoded));
            }
            if let Ok(decoded) = <DelegatecallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Delegatecall(decoded));
            }
            if let Ok(decoded) = <CreateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Create(decoded));
            }
            if let Ok(decoded) = <DripCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Drip(decoded));
            }
            if let Ok(decoded) = <DripsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Drips(decoded));
            }
            if let Ok(decoded) = <ExecutableCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Executable(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOwner(decoded));
            }
            if let Ok(decoded) = <StatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Status(decoded));
            }
            if let Ok(decoded) = <WithdrawErc20WithAssetAndToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawErc20WithAssetAndTo(decoded));
            }
            if let Ok(decoded) = <WithdrawERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawERC20(decoded));
            }
            if let Ok(decoded) = <WithdrawERC721Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawERC721(decoded));
            }
            if let Ok(decoded) = <WithdrawEthWithAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawEthWithAmount(decoded));
            }
            if let Ok(decoded) = <WithdrawETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DrippieCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Call(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Delegatecall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Create(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Drip(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Drips(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Executable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Status(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawErc20WithAssetAndTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawERC721(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawEthWithAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DrippieCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Call(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegatecall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Create(element) => ::core::fmt::Display::fmt(element, f),
                Self::Drip(element) => ::core::fmt::Display::fmt(element, f),
                Self::Drips(element) => ::core::fmt::Display::fmt(element, f),
                Self::Executable(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Status(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawErc20WithAssetAndTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawERC721(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawEthWithAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawETH(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CallCall> for DrippieCalls {
        fn from(value: CallCall) -> Self {
            Self::Call(value)
        }
    }
    impl ::core::convert::From<DelegatecallCall> for DrippieCalls {
        fn from(value: DelegatecallCall) -> Self {
            Self::Delegatecall(value)
        }
    }
    impl ::core::convert::From<CreateCall> for DrippieCalls {
        fn from(value: CreateCall) -> Self {
            Self::Create(value)
        }
    }
    impl ::core::convert::From<DripCall> for DrippieCalls {
        fn from(value: DripCall) -> Self {
            Self::Drip(value)
        }
    }
    impl ::core::convert::From<DripsCall> for DrippieCalls {
        fn from(value: DripsCall) -> Self {
            Self::Drips(value)
        }
    }
    impl ::core::convert::From<ExecutableCall> for DrippieCalls {
        fn from(value: ExecutableCall) -> Self {
            Self::Executable(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DrippieCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetOwnerCall> for DrippieCalls {
        fn from(value: SetOwnerCall) -> Self {
            Self::SetOwner(value)
        }
    }
    impl ::core::convert::From<StatusCall> for DrippieCalls {
        fn from(value: StatusCall) -> Self {
            Self::Status(value)
        }
    }
    impl ::core::convert::From<WithdrawErc20WithAssetAndToCall> for DrippieCalls {
        fn from(value: WithdrawErc20WithAssetAndToCall) -> Self {
            Self::WithdrawErc20WithAssetAndTo(value)
        }
    }
    impl ::core::convert::From<WithdrawERC20Call> for DrippieCalls {
        fn from(value: WithdrawERC20Call) -> Self {
            Self::WithdrawERC20(value)
        }
    }
    impl ::core::convert::From<WithdrawERC721Call> for DrippieCalls {
        fn from(value: WithdrawERC721Call) -> Self {
            Self::WithdrawERC721(value)
        }
    }
    impl ::core::convert::From<WithdrawEthWithAmountCall> for DrippieCalls {
        fn from(value: WithdrawEthWithAmountCall) -> Self {
            Self::WithdrawEthWithAmount(value)
        }
    }
    impl ::core::convert::From<WithdrawETHCall> for DrippieCalls {
        fn from(value: WithdrawETHCall) -> Self {
            Self::WithdrawETH(value)
        }
    }
    ///Container type for all return fields from the `CALL` function with signature `CALL(address,bytes,uint256)` and selector `0x6e2d44ae`
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
    pub struct CallReturn {
        pub success: bool,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `DELEGATECALL` function with signature `DELEGATECALL(address,bytes)` and selector `0xedee6239`
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
    pub struct DelegatecallReturn {
        pub success: bool,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `drips` function with signature `drips(string)` and selector `0x4d7fba6e`
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
    pub struct DripsReturn {
        pub status: u8,
        pub config: DripConfig,
        pub last: ::ethers::core::types::U256,
        pub count: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `executable` function with signature `executable(string)` and selector `0xfc3e3eba`
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
    pub struct ExecutableReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///`DripAction(address,bytes,uint256)`
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
    pub struct DripAction {
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub value: ::ethers::core::types::U256,
    }
    ///`DripConfig(bool,uint256,address,bytes,(address,bytes,uint256)[])`
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
    pub struct DripConfig {
        pub reentrant: bool,
        pub interval: ::ethers::core::types::U256,
        pub dripcheck: ::ethers::core::types::Address,
        pub checkparams: ::ethers::core::types::Bytes,
        pub actions: ::std::vec::Vec<DripAction>,
    }
}

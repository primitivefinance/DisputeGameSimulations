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
pub mod i_gnosis_safe {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VERSION"),
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
                    ::std::borrow::ToOwned::to_owned("addOwnerWithThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addOwnerWithThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("approveHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approveHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hashToApprove"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("approvedHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvedHashes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("changeThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeThreshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("checkNSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkNSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requiredSignatures",
                                    ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("disableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("disableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevModule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
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
                    ::std::borrow::ToOwned::to_owned("enableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                    ::std::borrow::ToOwned::to_owned("encodeTransactionData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeTransactionData",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execTransaction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execTransaction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execTransactionFromModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModule",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
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
                        "execTransactionFromModuleReturnData",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModuleReturnData",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainId"),
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
                    ::std::borrow::ToOwned::to_owned("getModulesPaginated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getModulesPaginated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pageSize"),
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
                                    name: ::std::borrow::ToOwned::to_owned("array"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("next"),
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
                    ::std::borrow::ToOwned::to_owned("getOwners"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOwners"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getStorageAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStorageAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("offset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getThreshold"),
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
                    ::std::borrow::ToOwned::to_owned("getTransactionHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTransactionHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                    ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("isOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                    ::std::borrow::ToOwned::to_owned("removeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("requiredTxGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requiredTxGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFallbackHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFallbackHandler"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("handler"),
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
                    ::std::borrow::ToOwned::to_owned("setGuard"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGuard"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("guard"),
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
                    ::std::borrow::ToOwned::to_owned("setup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owners"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackHandler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentReceiver"),
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
                    ::std::borrow::ToOwned::to_owned("signedMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signedMessages"),
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
                    ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldataPayload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("swapOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddedOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApproveHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApproveHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approvedHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedFallbackHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangedFallbackHandler",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("handler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedGuard"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangedGuard"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("guard"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangedThreshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
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
                    ::std::borrow::ToOwned::to_owned("DisabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DisabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionFailure"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleFailure",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionSuccess"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("RemovedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemovedOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SafeReceived"),
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
                (
                    ::std::borrow::ToOwned::to_owned("SafeSetup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SafeSetup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owners"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackHandler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SignMsg"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IGNOSISSAFE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IGnosisSafe<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IGnosisSafe<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IGnosisSafe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IGnosisSafe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IGnosisSafe<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IGnosisSafe))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IGnosisSafe<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IGNOSISSAFE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOwnerWithThreshold` (0x0d582f13) function
        pub fn add_owner_with_threshold(
            &self,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 88, 47, 19], (owner, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveHash` (0xd4d9bdcd) function
        pub fn approve_hash(
            &self,
            hash_to_approve: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 217, 189, 205], hash_to_approve)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvedHashes` (0x7d832974) function
        pub fn approved_hashes(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 131, 41, 116], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeThreshold` (0x694e80c3) function
        pub fn change_threshold(
            &self,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 78, 128, 195], threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkNSignatures` (0x12fb68e0) function
        pub fn check_n_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
            required_signatures: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 251, 104, 224],
                    (data_hash, data, signatures, required_signatures),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x934f3a11) function
        pub fn check_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 79, 58, 17], (data_hash, data, signatures))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableModule` (0xe009cfde) function
        pub fn disable_module(
            &self,
            prev_module: ::ethers::core::types::Address,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 9, 207, 222], (prev_module, module))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableModule` (0x610b5925) function
        pub fn enable_module(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 11, 89, 37], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeTransactionData` (0xe86637db) function
        pub fn encode_transaction_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [232, 102, 55, 219],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransaction` (0x6a761202) function
        pub fn exec_transaction(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [106, 118, 18, 2],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        signatures,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModule` (0x468721a7) function
        pub fn exec_transaction_from_module(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 135, 33, 167], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModuleReturnData` (0x5229073f) function
        pub fn exec_transaction_from_module_return_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([82, 41, 7, 63], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModulesPaginated` (0xcc2f8452) function
        pub fn get_modules_paginated(
            &self,
            start: ::ethers::core::types::Address,
            page_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([204, 47, 132, 82], (start, page_size))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOwners` (0xa0e67e2b) function
        pub fn get_owners(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([160, 230, 126, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStorageAt` (0x5624b25b) function
        pub fn get_storage_at(
            &self,
            offset: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([86, 36, 178, 91], (offset, length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getThreshold` (0xe75235b8) function
        pub fn get_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 82, 53, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTransactionHash` (0xd8d11f78) function
        pub fn get_transaction_hash(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [216, 209, 31, 120],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isModuleEnabled` (0x2d9ad53d) function
        pub fn is_module_enabled(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 154, 213, 61], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOwner` (0x2f54bf6e) function
        pub fn is_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 84, 191, 110], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeOwner` (0xf8dc5dd9) function
        pub fn remove_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 220, 93, 217], (prev_owner, owner, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requiredTxGas` (0xc4ca3a9c) function
        pub fn required_tx_gas(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 202, 58, 156], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackHandler` (0xf08a0323) function
        pub fn set_fallback_handler(
            &self,
            handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 138, 3, 35], handler)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGuard` (0xe19a9dd9) function
        pub fn set_guard(
            &self,
            guard: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 157, 217], guard)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setup` (0xb63e800d) function
        pub fn setup(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
            threshold: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            fallback_handler: ::ethers::core::types::Address,
            payment_token: ::ethers::core::types::Address,
            payment: ::ethers::core::types::U256,
            payment_receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [182, 62, 128, 13],
                    (
                        owners,
                        threshold,
                        to,
                        data,
                        fallback_handler,
                        payment_token,
                        payment,
                        payment_receiver,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedMessages` (0x5ae6bd37) function
        pub fn signed_messages(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 230, 189, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateAndRevert` (0xb4faba09) function
        pub fn simulate_and_revert(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 250, 186, 9], (target_contract, calldata_payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapOwner` (0xe318b52b) function
        pub fn swap_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            old_owner: ::ethers::core::types::Address,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 24, 181, 43], (prev_owner, old_owner, new_owner))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedOwner` event
        pub fn added_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApproveHash` event
        pub fn approve_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApproveHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedFallbackHandler` event
        pub fn changed_fallback_handler_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedFallbackHandlerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedGuard` event
        pub fn changed_guard_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedGuardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedThreshold` event
        pub fn changed_threshold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedThresholdFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DisabledModule` event
        pub fn disabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnabledModule` event
        pub fn enabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFailure` event
        pub fn execution_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleFailure` event
        pub fn execution_from_module_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleSuccess` event
        pub fn execution_from_module_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionSuccess` event
        pub fn execution_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemovedOwner` event
        pub fn removed_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemovedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeReceived` event
        pub fn safe_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeSetup` event
        pub fn safe_setup_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeSetupFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SignMsg` event
        pub fn sign_msg_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignMsgFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IGnosisSafeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IGnosisSafe<M> {
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
    #[ethevent(name = "AddedOwner", abi = "AddedOwner(address)")]
    pub struct AddedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "ApproveHash", abi = "ApproveHash(bytes32,address)")]
    pub struct ApproveHashFilter {
        #[ethevent(indexed)]
        pub approved_hash: [u8; 32],
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedFallbackHandler", abi = "ChangedFallbackHandler(address)")]
    pub struct ChangedFallbackHandlerFilter {
        pub handler: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedGuard", abi = "ChangedGuard(address)")]
    pub struct ChangedGuardFilter {
        pub guard: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedThreshold", abi = "ChangedThreshold(uint256)")]
    pub struct ChangedThresholdFilter {
        pub threshold: ::ethers::core::types::U256,
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
    #[ethevent(name = "DisabledModule", abi = "DisabledModule(address)")]
    pub struct DisabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "EnabledModule", abi = "EnabledModule(address)")]
    pub struct EnabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "ExecutionFailure", abi = "ExecutionFailure(bytes32,uint256)")]
    pub struct ExecutionFailureFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
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
        name = "ExecutionFromModuleFailure",
        abi = "ExecutionFromModuleFailure(address)"
    )]
    pub struct ExecutionFromModuleFailureFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
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
        name = "ExecutionFromModuleSuccess",
        abi = "ExecutionFromModuleSuccess(address)"
    )]
    pub struct ExecutionFromModuleSuccessFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "ExecutionSuccess", abi = "ExecutionSuccess(bytes32,uint256)")]
    pub struct ExecutionSuccessFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
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
    #[ethevent(name = "RemovedOwner", abi = "RemovedOwner(address)")]
    pub struct RemovedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "SafeReceived", abi = "SafeReceived(address,uint256)")]
    pub struct SafeReceivedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        name = "SafeSetup",
        abi = "SafeSetup(address,address[],uint256,address,address)"
    )]
    pub struct SafeSetupFilter {
        #[ethevent(indexed)]
        pub initiator: ::ethers::core::types::Address,
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub initializer: ::ethers::core::types::Address,
        pub fallback_handler: ::ethers::core::types::Address,
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
    #[ethevent(name = "SignMsg", abi = "SignMsg(bytes32)")]
    pub struct SignMsgFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IGnosisSafeEvents {
        AddedOwnerFilter(AddedOwnerFilter),
        ApproveHashFilter(ApproveHashFilter),
        ChangedFallbackHandlerFilter(ChangedFallbackHandlerFilter),
        ChangedGuardFilter(ChangedGuardFilter),
        ChangedThresholdFilter(ChangedThresholdFilter),
        DisabledModuleFilter(DisabledModuleFilter),
        EnabledModuleFilter(EnabledModuleFilter),
        ExecutionFailureFilter(ExecutionFailureFilter),
        ExecutionFromModuleFailureFilter(ExecutionFromModuleFailureFilter),
        ExecutionFromModuleSuccessFilter(ExecutionFromModuleSuccessFilter),
        ExecutionSuccessFilter(ExecutionSuccessFilter),
        RemovedOwnerFilter(RemovedOwnerFilter),
        SafeReceivedFilter(SafeReceivedFilter),
        SafeSetupFilter(SafeSetupFilter),
        SignMsgFilter(SignMsgFilter),
    }
    impl ::ethers::contract::EthLogDecode for IGnosisSafeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedOwnerFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::AddedOwnerFilter(decoded));
            }
            if let Ok(decoded) = ApproveHashFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ApproveHashFilter(decoded));
            }
            if let Ok(decoded) = ChangedFallbackHandlerFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ChangedFallbackHandlerFilter(decoded));
            }
            if let Ok(decoded) = ChangedGuardFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ChangedGuardFilter(decoded));
            }
            if let Ok(decoded) = ChangedThresholdFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ChangedThresholdFilter(decoded));
            }
            if let Ok(decoded) = DisabledModuleFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::DisabledModuleFilter(decoded));
            }
            if let Ok(decoded) = EnabledModuleFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::EnabledModuleFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFailureFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ExecutionFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleFailureFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ExecutionFromModuleFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleSuccessFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ExecutionFromModuleSuccessFilter(decoded));
            }
            if let Ok(decoded) = ExecutionSuccessFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::ExecutionSuccessFilter(decoded));
            }
            if let Ok(decoded) = RemovedOwnerFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::RemovedOwnerFilter(decoded));
            }
            if let Ok(decoded) = SafeReceivedFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::SafeReceivedFilter(decoded));
            }
            if let Ok(decoded) = SafeSetupFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::SafeSetupFilter(decoded));
            }
            if let Ok(decoded) = SignMsgFilter::decode_log(log) {
                return Ok(IGnosisSafeEvents::SignMsgFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IGnosisSafeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedOwnerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveHashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangedFallbackHandlerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedGuardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedThresholdFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DisabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovedOwnerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeSetupFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignMsgFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedOwnerFilter> for IGnosisSafeEvents {
        fn from(value: AddedOwnerFilter) -> Self {
            Self::AddedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<ApproveHashFilter> for IGnosisSafeEvents {
        fn from(value: ApproveHashFilter) -> Self {
            Self::ApproveHashFilter(value)
        }
    }
    impl ::core::convert::From<ChangedFallbackHandlerFilter> for IGnosisSafeEvents {
        fn from(value: ChangedFallbackHandlerFilter) -> Self {
            Self::ChangedFallbackHandlerFilter(value)
        }
    }
    impl ::core::convert::From<ChangedGuardFilter> for IGnosisSafeEvents {
        fn from(value: ChangedGuardFilter) -> Self {
            Self::ChangedGuardFilter(value)
        }
    }
    impl ::core::convert::From<ChangedThresholdFilter> for IGnosisSafeEvents {
        fn from(value: ChangedThresholdFilter) -> Self {
            Self::ChangedThresholdFilter(value)
        }
    }
    impl ::core::convert::From<DisabledModuleFilter> for IGnosisSafeEvents {
        fn from(value: DisabledModuleFilter) -> Self {
            Self::DisabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<EnabledModuleFilter> for IGnosisSafeEvents {
        fn from(value: EnabledModuleFilter) -> Self {
            Self::EnabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFailureFilter> for IGnosisSafeEvents {
        fn from(value: ExecutionFailureFilter) -> Self {
            Self::ExecutionFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleFailureFilter> for IGnosisSafeEvents {
        fn from(value: ExecutionFromModuleFailureFilter) -> Self {
            Self::ExecutionFromModuleFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleSuccessFilter> for IGnosisSafeEvents {
        fn from(value: ExecutionFromModuleSuccessFilter) -> Self {
            Self::ExecutionFromModuleSuccessFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionSuccessFilter> for IGnosisSafeEvents {
        fn from(value: ExecutionSuccessFilter) -> Self {
            Self::ExecutionSuccessFilter(value)
        }
    }
    impl ::core::convert::From<RemovedOwnerFilter> for IGnosisSafeEvents {
        fn from(value: RemovedOwnerFilter) -> Self {
            Self::RemovedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SafeReceivedFilter> for IGnosisSafeEvents {
        fn from(value: SafeReceivedFilter) -> Self {
            Self::SafeReceivedFilter(value)
        }
    }
    impl ::core::convert::From<SafeSetupFilter> for IGnosisSafeEvents {
        fn from(value: SafeSetupFilter) -> Self {
            Self::SafeSetupFilter(value)
        }
    }
    impl ::core::convert::From<SignMsgFilter> for IGnosisSafeEvents {
        fn from(value: SignMsgFilter) -> Self {
            Self::SignMsgFilter(value)
        }
    }
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `addOwnerWithThreshold` function with signature `addOwnerWithThreshold(address,uint256)` and selector `0x0d582f13`
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
        name = "addOwnerWithThreshold",
        abi = "addOwnerWithThreshold(address,uint256)"
    )]
    pub struct AddOwnerWithThresholdCall {
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approveHash` function with signature `approveHash(bytes32)` and selector `0xd4d9bdcd`
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
    #[ethcall(name = "approveHash", abi = "approveHash(bytes32)")]
    pub struct ApproveHashCall {
        pub hash_to_approve: [u8; 32],
    }
    ///Container type for all input parameters for the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `0x7d832974`
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
    #[ethcall(name = "approvedHashes", abi = "approvedHashes(address,bytes32)")]
    pub struct ApprovedHashesCall(pub ::ethers::core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `changeThreshold` function with signature `changeThreshold(uint256)` and selector `0x694e80c3`
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
    #[ethcall(name = "changeThreshold", abi = "changeThreshold(uint256)")]
    pub struct ChangeThresholdCall {
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkNSignatures` function with signature `checkNSignatures(bytes32,bytes,bytes,uint256)` and selector `0x12fb68e0`
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
        name = "checkNSignatures",
        abi = "checkNSignatures(bytes32,bytes,bytes,uint256)"
    )]
    pub struct CheckNSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
        pub required_signatures: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,bytes)` and selector `0x934f3a11`
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
    #[ethcall(name = "checkSignatures", abi = "checkSignatures(bytes32,bytes,bytes)")]
    pub struct CheckSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `disableModule` function with signature `disableModule(address,address)` and selector `0xe009cfde`
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
    #[ethcall(name = "disableModule", abi = "disableModule(address,address)")]
    pub struct DisableModuleCall {
        pub prev_module: ::ethers::core::types::Address,
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `enableModule` function with signature `enableModule(address)` and selector `0x610b5925`
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
    #[ethcall(name = "enableModule", abi = "enableModule(address)")]
    pub struct EnableModuleCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xe86637db`
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
        name = "encodeTransactionData",
        abi = "encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct EncodeTransactionDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `0x6a761202`
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
        name = "execTransaction",
        abi = "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)"
    )]
    pub struct ExecTransactionCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
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
        name = "execTransactionFromModule",
        abi = "execTransactionFromModule(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
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
        name = "execTransactionFromModuleReturnData",
        abi = "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleReturnDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
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
        name = "getModulesPaginated",
        abi = "getModulesPaginated(address,uint256)"
    )]
    pub struct GetModulesPaginatedCall {
        pub start: ::ethers::core::types::Address,
        pub page_size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getOwners` function with signature `getOwners()` and selector `0xa0e67e2b`
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
    #[ethcall(name = "getOwners", abi = "getOwners()")]
    pub struct GetOwnersCall;
    ///Container type for all input parameters for the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
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
    #[ethcall(name = "getStorageAt", abi = "getStorageAt(uint256,uint256)")]
    pub struct GetStorageAtCall {
        pub offset: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getThreshold` function with signature `getThreshold()` and selector `0xe75235b8`
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
    #[ethcall(name = "getThreshold", abi = "getThreshold()")]
    pub struct GetThresholdCall;
    ///Container type for all input parameters for the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xd8d11f78`
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
        name = "getTransactionHash",
        abi = "getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct GetTransactionHashCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    #[ethcall(name = "isModuleEnabled", abi = "isModuleEnabled(address)")]
    pub struct IsModuleEnabledCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
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
    #[ethcall(name = "isOwner", abi = "isOwner(address)")]
    pub struct IsOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `removeOwner` function with signature `removeOwner(address,address,uint256)` and selector `0xf8dc5dd9`
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
    #[ethcall(name = "removeOwner", abi = "removeOwner(address,address,uint256)")]
    pub struct RemoveOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `0xc4ca3a9c`
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
        name = "requiredTxGas",
        abi = "requiredTxGas(address,uint256,bytes,uint8)"
    )]
    pub struct RequiredTxGasCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `setFallbackHandler` function with signature `setFallbackHandler(address)` and selector `0xf08a0323`
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
    #[ethcall(name = "setFallbackHandler", abi = "setFallbackHandler(address)")]
    pub struct SetFallbackHandlerCall {
        pub handler: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGuard` function with signature `setGuard(address)` and selector `0xe19a9dd9`
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
    #[ethcall(name = "setGuard", abi = "setGuard(address)")]
    pub struct SetGuardCall {
        pub guard: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setup` function with signature `setup(address[],uint256,address,bytes,address,address,uint256,address)` and selector `0xb63e800d`
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
        name = "setup",
        abi = "setup(address[],uint256,address,bytes,address,address,uint256,address)"
    )]
    pub struct SetupCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub fallback_handler: ::ethers::core::types::Address,
        pub payment_token: ::ethers::core::types::Address,
        pub payment: ::ethers::core::types::U256,
        pub payment_receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `signedMessages` function with signature `signedMessages(bytes32)` and selector `0x5ae6bd37`
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
    #[ethcall(name = "signedMessages", abi = "signedMessages(bytes32)")]
    pub struct SignedMessagesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `simulateAndRevert` function with signature `simulateAndRevert(address,bytes)` and selector `0xb4faba09`
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
    #[ethcall(name = "simulateAndRevert", abi = "simulateAndRevert(address,bytes)")]
    pub struct SimulateAndRevertCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `swapOwner` function with signature `swapOwner(address,address,address)` and selector `0xe318b52b`
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
    #[ethcall(name = "swapOwner", abi = "swapOwner(address,address,address)")]
    pub struct SwapOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub old_owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IGnosisSafeCalls {
        Version(VersionCall),
        AddOwnerWithThreshold(AddOwnerWithThresholdCall),
        ApproveHash(ApproveHashCall),
        ApprovedHashes(ApprovedHashesCall),
        ChangeThreshold(ChangeThresholdCall),
        CheckNSignatures(CheckNSignaturesCall),
        CheckSignatures(CheckSignaturesCall),
        DisableModule(DisableModuleCall),
        DomainSeparator(DomainSeparatorCall),
        EnableModule(EnableModuleCall),
        EncodeTransactionData(EncodeTransactionDataCall),
        ExecTransaction(ExecTransactionCall),
        ExecTransactionFromModule(ExecTransactionFromModuleCall),
        ExecTransactionFromModuleReturnData(ExecTransactionFromModuleReturnDataCall),
        GetChainId(GetChainIdCall),
        GetModulesPaginated(GetModulesPaginatedCall),
        GetOwners(GetOwnersCall),
        GetStorageAt(GetStorageAtCall),
        GetThreshold(GetThresholdCall),
        GetTransactionHash(GetTransactionHashCall),
        IsModuleEnabled(IsModuleEnabledCall),
        IsOwner(IsOwnerCall),
        Nonce(NonceCall),
        RemoveOwner(RemoveOwnerCall),
        RequiredTxGas(RequiredTxGasCall),
        SetFallbackHandler(SetFallbackHandlerCall),
        SetGuard(SetGuardCall),
        Setup(SetupCall),
        SignedMessages(SignedMessagesCall),
        SimulateAndRevert(SimulateAndRevertCall),
        SwapOwner(SwapOwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for IGnosisSafeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <AddOwnerWithThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddOwnerWithThreshold(decoded));
            }
            if let Ok(decoded) = <ApproveHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproveHash(decoded));
            }
            if let Ok(decoded) = <ApprovedHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApprovedHashes(decoded));
            }
            if let Ok(decoded) = <ChangeThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeThreshold(decoded));
            }
            if let Ok(decoded) = <CheckNSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckNSignatures(decoded));
            }
            if let Ok(decoded) = <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <DisableModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DisableModule(decoded));
            }
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <EnableModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnableModule(decoded));
            }
            if let Ok(decoded) = <EncodeTransactionDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EncodeTransactionData(decoded));
            }
            if let Ok(decoded) = <ExecTransactionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecTransaction(decoded));
            }
            if let Ok(decoded) = <ExecTransactionFromModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecTransactionFromModule(decoded));
            }
            if let Ok(decoded) = <ExecTransactionFromModuleReturnDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecTransactionFromModuleReturnData(decoded));
            }
            if let Ok(decoded) = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded) = <GetModulesPaginatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetModulesPaginated(decoded));
            }
            if let Ok(decoded) = <GetOwnersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOwners(decoded));
            }
            if let Ok(decoded) = <GetStorageAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStorageAt(decoded));
            }
            if let Ok(decoded) = <GetThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetThreshold(decoded));
            }
            if let Ok(decoded) = <GetTransactionHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTransactionHash(decoded));
            }
            if let Ok(decoded) = <IsModuleEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsModuleEnabled(decoded));
            }
            if let Ok(decoded) = <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <RemoveOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveOwner(decoded));
            }
            if let Ok(decoded) = <RequiredTxGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequiredTxGas(decoded));
            }
            if let Ok(decoded) = <SetFallbackHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFallbackHandler(decoded));
            }
            if let Ok(decoded) = <SetGuardCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetGuard(decoded));
            }
            if let Ok(decoded) = <SetupCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Setup(decoded));
            }
            if let Ok(decoded) = <SignedMessagesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SignedMessages(decoded));
            }
            if let Ok(decoded) = <SimulateAndRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateAndRevert(decoded));
            }
            if let Ok(decoded) = <SwapOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IGnosisSafeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddOwnerWithThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApprovedHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckNSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DisableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeTransactionData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetModulesPaginated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOwners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStorageAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTransactionHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsModuleEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequiredTxGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGuard(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Setup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAndRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IGnosisSafeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOwnerWithThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApproveHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovedHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckNSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeTransactionData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecTransactionFromModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetModulesPaginated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOwners(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStorageAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTransactionHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsModuleEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequiredTxGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFallbackHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGuard(element) => ::core::fmt::Display::fmt(element, f),
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateAndRevert(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapOwner(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<VersionCall> for IGnosisSafeCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<AddOwnerWithThresholdCall> for IGnosisSafeCalls {
        fn from(value: AddOwnerWithThresholdCall) -> Self {
            Self::AddOwnerWithThreshold(value)
        }
    }
    impl ::core::convert::From<ApproveHashCall> for IGnosisSafeCalls {
        fn from(value: ApproveHashCall) -> Self {
            Self::ApproveHash(value)
        }
    }
    impl ::core::convert::From<ApprovedHashesCall> for IGnosisSafeCalls {
        fn from(value: ApprovedHashesCall) -> Self {
            Self::ApprovedHashes(value)
        }
    }
    impl ::core::convert::From<ChangeThresholdCall> for IGnosisSafeCalls {
        fn from(value: ChangeThresholdCall) -> Self {
            Self::ChangeThreshold(value)
        }
    }
    impl ::core::convert::From<CheckNSignaturesCall> for IGnosisSafeCalls {
        fn from(value: CheckNSignaturesCall) -> Self {
            Self::CheckNSignatures(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for IGnosisSafeCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<DisableModuleCall> for IGnosisSafeCalls {
        fn from(value: DisableModuleCall) -> Self {
            Self::DisableModule(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for IGnosisSafeCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EnableModuleCall> for IGnosisSafeCalls {
        fn from(value: EnableModuleCall) -> Self {
            Self::EnableModule(value)
        }
    }
    impl ::core::convert::From<EncodeTransactionDataCall> for IGnosisSafeCalls {
        fn from(value: EncodeTransactionDataCall) -> Self {
            Self::EncodeTransactionData(value)
        }
    }
    impl ::core::convert::From<ExecTransactionCall> for IGnosisSafeCalls {
        fn from(value: ExecTransactionCall) -> Self {
            Self::ExecTransaction(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleCall> for IGnosisSafeCalls {
        fn from(value: ExecTransactionFromModuleCall) -> Self {
            Self::ExecTransactionFromModule(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleReturnDataCall>
    for IGnosisSafeCalls {
        fn from(value: ExecTransactionFromModuleReturnDataCall) -> Self {
            Self::ExecTransactionFromModuleReturnData(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for IGnosisSafeCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetModulesPaginatedCall> for IGnosisSafeCalls {
        fn from(value: GetModulesPaginatedCall) -> Self {
            Self::GetModulesPaginated(value)
        }
    }
    impl ::core::convert::From<GetOwnersCall> for IGnosisSafeCalls {
        fn from(value: GetOwnersCall) -> Self {
            Self::GetOwners(value)
        }
    }
    impl ::core::convert::From<GetStorageAtCall> for IGnosisSafeCalls {
        fn from(value: GetStorageAtCall) -> Self {
            Self::GetStorageAt(value)
        }
    }
    impl ::core::convert::From<GetThresholdCall> for IGnosisSafeCalls {
        fn from(value: GetThresholdCall) -> Self {
            Self::GetThreshold(value)
        }
    }
    impl ::core::convert::From<GetTransactionHashCall> for IGnosisSafeCalls {
        fn from(value: GetTransactionHashCall) -> Self {
            Self::GetTransactionHash(value)
        }
    }
    impl ::core::convert::From<IsModuleEnabledCall> for IGnosisSafeCalls {
        fn from(value: IsModuleEnabledCall) -> Self {
            Self::IsModuleEnabled(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for IGnosisSafeCalls {
        fn from(value: IsOwnerCall) -> Self {
            Self::IsOwner(value)
        }
    }
    impl ::core::convert::From<NonceCall> for IGnosisSafeCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<RemoveOwnerCall> for IGnosisSafeCalls {
        fn from(value: RemoveOwnerCall) -> Self {
            Self::RemoveOwner(value)
        }
    }
    impl ::core::convert::From<RequiredTxGasCall> for IGnosisSafeCalls {
        fn from(value: RequiredTxGasCall) -> Self {
            Self::RequiredTxGas(value)
        }
    }
    impl ::core::convert::From<SetFallbackHandlerCall> for IGnosisSafeCalls {
        fn from(value: SetFallbackHandlerCall) -> Self {
            Self::SetFallbackHandler(value)
        }
    }
    impl ::core::convert::From<SetGuardCall> for IGnosisSafeCalls {
        fn from(value: SetGuardCall) -> Self {
            Self::SetGuard(value)
        }
    }
    impl ::core::convert::From<SetupCall> for IGnosisSafeCalls {
        fn from(value: SetupCall) -> Self {
            Self::Setup(value)
        }
    }
    impl ::core::convert::From<SignedMessagesCall> for IGnosisSafeCalls {
        fn from(value: SignedMessagesCall) -> Self {
            Self::SignedMessages(value)
        }
    }
    impl ::core::convert::From<SimulateAndRevertCall> for IGnosisSafeCalls {
        fn from(value: SimulateAndRevertCall) -> Self {
            Self::SimulateAndRevert(value)
        }
    }
    impl ::core::convert::From<SwapOwnerCall> for IGnosisSafeCalls {
        fn from(value: SwapOwnerCall) -> Self {
            Self::SwapOwner(value)
        }
    }
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    ///Container type for all return fields from the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `0x7d832974`
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
    pub struct ApprovedHashesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xe86637db`
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
    pub struct EncodeTransactionDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `0x6a761202`
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
    pub struct ExecTransactionReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
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
    pub struct ExecTransactionFromModuleReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
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
    pub struct ExecTransactionFromModuleReturnDataReturn {
        pub success: bool,
        pub return_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
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
    pub struct GetModulesPaginatedReturn {
        pub array: ::std::vec::Vec<::ethers::core::types::Address>,
        pub next: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getOwners` function with signature `getOwners()` and selector `0xa0e67e2b`
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
    pub struct GetOwnersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
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
    pub struct GetStorageAtReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getThreshold` function with signature `getThreshold()` and selector `0xe75235b8`
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
    pub struct GetThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xd8d11f78`
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
    pub struct GetTransactionHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    pub struct IsModuleEnabledReturn(pub bool);
    ///Container type for all return fields from the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
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
    pub struct IsOwnerReturn(pub bool);
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `0xc4ca3a9c`
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
    pub struct RequiredTxGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `signedMessages` function with signature `signedMessages(bytes32)` and selector `0x5ae6bd37`
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
    pub struct SignedMessagesReturn(pub ::ethers::core::types::U256);
}

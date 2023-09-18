pub use l2_standard_bridge::*;
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
pub mod l2_standard_bridge {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_otherBridge"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract StandardBridge"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MESSENGER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MESSENGER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract CrossDomainMessenger",
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
                    ::std::borrow::ToOwned::to_owned("OTHER_BRIDGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("OTHER_BRIDGE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract StandardBridge"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bridgeERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("bridgeERC20To"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeERC20To"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("bridgeETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("bridgeETHTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeETHTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("deposits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposits"),
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
                    ::std::borrow::ToOwned::to_owned("finalizeBridgeERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "finalizeBridgeERC20",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("finalizeBridgeETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("finalizeBridgeETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("finalizeDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("finalizeDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l1Token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2Token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("l1TokenBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1TokenBridge"),
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
                    ::std::borrow::ToOwned::to_owned("messenger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("messenger"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract CrossDomainMessenger",
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
                    ::std::borrow::ToOwned::to_owned("otherBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("otherBridge"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract StandardBridge"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2Token"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2Token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DepositFinalized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DepositFinalized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l1Token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l2Token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20BridgeFinalized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20BridgeFinalized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20BridgeInitiated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20BridgeInitiated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ETHBridgeFinalized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ETHBridgeFinalized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ETHBridgeInitiated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ETHBridgeInitiated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("WithdrawalInitiated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawalInitiated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l1Token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l2Token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static L2STANDARDBRIDGE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0,\x858\x03\x80b\0,\x85\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xE4V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Rb\0\0Kb\0\0RV[Pb\0\x02\x16V[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15b\0\0uWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[b\0\0\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\xFF\xFF\x19\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ub\0\x01\x11sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07b\0\x01UV[`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x01\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\0\xD5V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0` \x82\x84\x03\x12\x15b\0\x01\xF7W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\x0FW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa*0b\0\x02U`\09`\0\x81\x81a\x02N\x01R\x81\x81a\x03`\x01R\x81\x81a\x05\x8E\x01R\x81\x81a\n#\x01R\x81\x81a\x13@\x01Ra\x16\x82\x01Ra*0`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x12W`\x005`\xE0\x1C\x80cf*c:\x11a\0\xA5W\x80c\x8F`\x1Ff\x11a\0tW\x80c\xA3\xA7\x95H\x11a\0YW\x80c\xA3\xA7\x95H\x14a\x04(W\x80c\xC8\x97\x01\xA2\x14a\x02?W\x80c\xE1\x10\x13\xDD\x14a\x04;W`\0\x80\xFD[\x80c\x8F`\x1Ff\x14a\x03\xB7W\x80c\x92~\xDE-\x14a\x03\xFDW`\0\x80\xFD[\x80cf*c:\x14a\x03;W\x80c\x7FF\xDD\xB2\x14a\x03NW\x80c\x81)\xFC\x1C\x14a\x03\x82W\x80c\x87\x08v#\x14a\x03\x97W`\0\x80\xFD[\x80c6\xC7\x17\xC1\x11a\0\xE1W\x80c6\xC7\x17\xC1\x14a\x02?W\x80c<\xB7G\xBF\x14a\x02\x98W\x80cT\n\xBFs\x14a\x02\xC5W\x80cT\xFDMP\x14a\x02\xE5W`\0\x80\xFD[\x80c\x01f\xA0z\x14a\x01\xE6W\x80c\t\xFC\x88C\x14a\x02\x06W\x80c\x165\xF5\xFD\x14a\x02\x19W\x80c2\xB7\0m\x14a\x02,W`\0\x80\xFD[6a\x01\xE1W3;\x15a\x01\xABW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xDFs\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\x00334b\x03\r@`@Q\x80` \x01`@R\x80`\0\x81RPa\x04NV[\0[`\0\x80\xFD[4\x80\x15a\x01\xF2W`\0\x80\xFD[Pa\x01\xDFa\x02\x016`\x04a$@V[a\x05)V[a\x01\xDFa\x02\x146`\x04a$\xF1V[a\x08\xE7V[a\x01\xDFa\x02'6`\x04a%DV[a\t\xBEV[a\x01\xDFa\x02:6`\x04a%\xB7V[a\x0E,V[4\x80\x15a\x02KW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA4W`\0\x80\xFD[P`\x03Ta\x02n\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\xD1W`\0\x80\xFD[Pa\x01\xDFa\x02\xE06`\x04a&\x0BV[a\x0F\x06V[4\x80\x15a\x02\xF1W`\0\x80\xFD[Pa\x03.`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.3.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x02\x8F\x91\x90a&\xF8V[a\x01\xDFa\x03I6`\x04a$@V[a\x0FKV[4\x80\x15a\x03ZW`\0\x80\xFD[Pa\x02n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x8EW`\0\x80\xFD[Pa\x01\xDFa\x0F\xBEV[4\x80\x15a\x03\xA3W`\0\x80\xFD[Pa\x01\xDFa\x03\xB26`\x04a'\x0BV[a\x11\x1BV[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x03\xEFa\x03\xD26`\x04a'\x8EV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x02\x8FV[4\x80\x15a\x04\tW`\0\x80\xFD[P`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02nV[a\x01\xDFa\x0466`\x04a'\x0BV[a\x11\xEFV[a\x01\xDFa\x04I6`\x04a'\xC7V[a\x123V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!R!R!R!R!R!R!R!R!S\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x01a\x04\x9DWa\x04\x98\x85\x85\x85\x85\x85a\x12|V[a\x05!V[`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC0\x1E\x1B\xD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0E\x91\x90a(*V[\x90Pa\x05\x1F\x87\x82\x88\x88\x88\x88\x88a\x14bV[P[PPPPPPV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\x06\x18WP`\x03T`@\x80Q\x7Fn)nE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x16\x91cn)nE\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\0\x91\x90a(*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x06\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from the other bridg`d\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[a\x06\xD3\x87a\x17\xAAV[\x15a\x08!Wa\x06\xE2\x87\x87a\x18\x0CV[a\x07\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FStandardBridge: wrong remote tok`D\x82\x01R\x7Fen for Optimism Mintable ERC20 l`d\x82\x01R\x7Focal token\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x18W=`\0\x80>=`\0\xFD[PPPPa\x08\xA3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R Ta\x08_\x90\x84\x90a(vV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x94\x8C\x16\x83R\x93\x90R\x91\x90\x91 \x91\x90\x91Ua\x08\xA3\x90\x85\x85a\x19,V[a\x05\x1F\x87\x87\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\0\x92PPPV[3;\x15a\tvW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\t\xB9334\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x12|\x92PPPV[PPPV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\n\xADWP`\x03T`@\x80Q\x7Fn)nE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x16\x91cn)nE\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\nqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x95\x91\x90a(*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x0B_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from the other bridg`d\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[\x824\x14a\x0B\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FStandardBridge: amount sent does`D\x82\x01R\x7F not match amount required\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x03a\x0C\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FStandardBridge: cannot send to s`D\x82\x01R\x7Felf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x85\x16\x03a\r>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FStandardBridge: cannot send to m`D\x82\x01R\x7Fessenger\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\r\x80\x85\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\x8E\x92PPPV[`\0a\r\x9D\x85Z\x86`@Q\x80` \x01`@R\x80`\0\x81RPa\x1B/V[\x90P\x80a\x05!W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FStandardBridge: ETH transfer fai`D\x82\x01R\x7Fled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[3;\x15a\x0E\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\x0E\xFF\x8533\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04N\x92PPPV[PPPPPV[a\x05\x1F\x87\x873\x88\x88\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14b\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x15\x80\x15a\x0F\x98WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\0\x14[\x15a\x0F\xAFWa\x0F\xAA\x85\x85\x85\x85\x85a\t\xBEV[a\x05\x1FV[a\x05\x1F\x86\x88\x87\x87\x87\x87\x87a\x05)V[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x0F\xE0WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x10lW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x10\xBAsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07a\x1BIV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[3;\x15a\x11\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\x05!\x86\x8633\x88\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14b\x92PPPV[a\x05!\x863\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04N\x92PPPV[a\x12v3\x854\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x12|\x92PPPV[PPPPV[\x824\x14a\x13\x0BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStandardBridge: bridging ETH mus`D\x82\x01R\x7Ft include sufficient ETH value\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\x13\x17\x85\x85\x85\x84a\x1C'V[`\x03T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c=\xBB +\x90\x85\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x7F\x165\xF5\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x13\x96\x90\x8B\x90\x8B\x90\x86\x90\x8A\x90`$\x01a(\x8DV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x94\x85\x16\x17\x90RQ`\xE0\x86\x90\x1B\x90\x92\x16\x82Ra\x14)\x92\x91\x88\x90`\x04\x01a(\xD6V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x14BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14VW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[a\x14k\x87a\x17\xAAV[\x15a\x15\xB9Wa\x14z\x87\x87a\x18\x0CV[a\x15,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FStandardBridge: wrong remote tok`D\x82\x01R\x7Fen for Optimism Mintable ERC20 l`d\x82\x01R\x7Focal token\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[`@Q\x7F\x9D\xC2\x9F\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xB0W=`\0\x80>=`\0\xFD[PPPPa\x16MV[a\x15\xDBs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x860\x86a\x1C\xC8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R Ta\x16\x19\x90\x84\x90a)\x1BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R U[a\x16[\x87\x87\x87\x87\x87\x86a\x1D&V[`\x03T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c=\xBB +\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x7F\x01f\xA0z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x16\xDC\x90\x8B\x90\x8D\x90\x8C\x90\x8C\x90\x8C\x90\x8B\x90`$\x01a)3V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x94\x85\x16\x17\x90RQ`\xE0\x85\x90\x1B\x90\x92\x16\x82Ra\x17o\x92\x91\x87\x90`\x04\x01a(\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x89W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x9DW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`\0a\x17\xD6\x82\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\xB4V[\x80a\x18\x06WPa\x18\x06\x82\x7F\xECO\xC8\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\xB4V[\x92\x91PPV[`\0a\x188\x83\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\xB4V[\x15a\x18\xE1W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC0\x1E\x1B\xD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xAC\x91\x90a(*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x90Pa\x18\x06V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\xC0\xB2\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x88W=`\0\x80>=`\0\xFD[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\t\xB9\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1D\xD7V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB0DE#&\x87\x17\xA0&\x98\xBEG\xD0\x80:\xA7F\x8C\0\xAC\xBE\xD2\xF8\xBD\x93\xA0E\x9C\xDEa\xDD\x89\x86\x86\x86`@Qa\x1Ax\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x05!\x86\x86\x86\x86\x86\x86a\x1E\xE3V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB0DE#&\x87\x17\xA0&\x98\xBEG\xD0\x80:\xA7F\x8C\0\xAC\xBE\xD2\xF8\xBD\x93\xA0E\x9C\xDEa\xDD\x89\x86\x86\x86`@Qa\x1B\x1B\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x12v\x84\x84\x84\x84a\x1FkV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1B\xE0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fs\xD1p\x91\n\xBA\x9EmP\xB1\x02\xDBR+\x1D\xBC\xD7\x96!oQ(\xB4E\xAA!5'(\x86I~\x86\x86\x86`@Qa\x1C\xB4\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x12v\x84\x84\x84\x84a\x1F\xD8V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x12v\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01a\x19~V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fs\xD1p\x91\n\xBA\x9EmP\xB1\x02\xDBR+\x1D\xBC\xD7\x96!oQ(\xB4E\xAA!5'(\x86I~\x86\x86\x86`@Qa\x1D\x9E\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x05!\x86\x86\x86\x86\x86\x86a 7V[`\0a\x1D\xBF\x83a \xAFV[\x80\x15a\x1D\xD0WPa\x1D\xD0\x83\x83a!\x13V[\x93\x92PPPV[`\0a\x1E9\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!\xE2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\t\xB9W\x80\x80` \x01\x90Q\x81\x01\x90a\x1EW\x91\x90a)\xCCV[a\t\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD5\x9Ce\xB3TE\"X5\xC8?P\xB6\xED\xE0j{\xE0G\xD2.5ps\xE2P\xD9\xAFSu\x18\xCD\x86\x86\x86`@Qa\x1F[\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4PPPPPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F1\xB2\x16o\xF6\x04\xFCVr\xEA]\xF0\x8Ax\x08\x1D+\xC6\xD7F\xCA\xDC\xE8\x80t\x7F6C\xD8\x19\xE8=\x84\x84`@Qa\x1F\xCA\x92\x91\x90a)\xEEV[`@Q\x80\x91\x03\x90\xA3PPPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F(I\xB40t\t:\x059ko*\x93}\xEE\x85e\xB1ZH\xA7\xB3\xD4\xBF\xFBs*P\x178\n\xF5\x84\x84`@Qa\x1F\xCA\x92\x91\x90a)\xEEV[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x7F\xF1&\xDB\x80$BK\xBF\xD9\x82n\x8A\xB8/\xF5\x916(\x9E\xA4@\xB0K9\xA0\xDF\x1B\x03\xB9\xCA\xBF\x86\x86\x86`@Qa\x1F[\x93\x92\x91\x90a)\x8EV[`\0a \xDB\x82\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a!\x13V[\x80\x15a\x18\x06WPa!\x0C\x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a!\x13V[\x15\x92\x91PPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x81R\x82Q`\0\x93\x92\x84\x92\x83\x92\x83\x92\x91\x83\x91\x90\x8Aau0\xFA\x92P=\x91P`\0Q\x90P\x82\x80\x15a!\xCBWP` \x82\x10\x15[\x80\x15a!\xD7WP`\0\x81\x11[\x97\x96PPPPPPPV[``a!\xF1\x84\x84`\0\x85a!\xF9V[\x94\x93PPPPV[``\x82G\x10\x15a\"\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a#\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\xA2V[`\0\x80\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x87`@Qa#2\x91\x90a*\x07V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a#oW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#tV[``\x91P[P\x91P\x91Pa!\xD7\x82\x82\x86``\x83\x15a#\x8EWP\x81a\x1D\xD0V[\x82Q\x15a#\x9EW\x82Q\x80\x84` \x01\xFD[\x81`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xA2\x91\x90a&\xF8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#\xF4W`\0\x80\xFD[PV[`\0\x80\x83`\x1F\x84\x01\x12a$\tW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a$9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a$[W`\0\x80\xFD[\x875a$f\x81a#\xD2V[\x96P` \x88\x015a$v\x81a#\xD2V[\x95P`@\x88\x015a$\x86\x81a#\xD2V[\x94P``\x88\x015a$\x96\x81a#\xD2V[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xB9W`\0\x80\xFD[a$\xC5\x8A\x82\x8B\x01a#\xF7V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$\xECW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\x06W`\0\x80\xFD[a%\x0F\x84a$\xD8V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%+W`\0\x80\xFD[a%7\x86\x82\x87\x01a#\xF7V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a%\\W`\0\x80\xFD[\x855a%g\x81a#\xD2V[\x94P` \x86\x015a%w\x81a#\xD2V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x9AW`\0\x80\xFD[a%\xA6\x88\x82\x89\x01a#\xF7V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a%\xCFW`\0\x80\xFD[\x855a%\xDA\x81a#\xD2V[\x94P` \x86\x015\x93Pa%\xEF`@\x87\x01a$\xD8V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x9AW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a&&W`\0\x80\xFD[\x875a&1\x81a#\xD2V[\x96P` \x88\x015a&A\x81a#\xD2V[\x95P`@\x88\x015a&Q\x81a#\xD2V[\x94P``\x88\x015\x93Pa&f`\x80\x89\x01a$\xD8V[\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xB9W`\0\x80\xFD[`\0[\x83\x81\x10\x15a&\x9DW\x81\x81\x01Q\x83\x82\x01R` \x01a&\x85V[\x83\x81\x11\x15a\x12vWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\xC6\x81` \x86\x01` \x86\x01a&\x82V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1D\xD0` \x83\x01\x84a&\xAEV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a'$W`\0\x80\xFD[\x865a'/\x81a#\xD2V[\x95P` \x87\x015a'?\x81a#\xD2V[\x94P`@\x87\x015\x93Pa'T``\x88\x01a$\xD8V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'pW`\0\x80\xFD[a'|\x89\x82\x8A\x01a#\xF7V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\xA1W`\0\x80\xFD[\x825a'\xAC\x81a#\xD2V[\x91P` \x83\x015a'\xBC\x81a#\xD2V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a'\xDDW`\0\x80\xFD[\x845a'\xE8\x81a#\xD2V[\x93Pa'\xF6` \x86\x01a$\xD8V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x12W`\0\x80\xFD[a(\x1E\x87\x82\x88\x01a#\xF7V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a(<W`\0\x80\xFD[\x81Qa\x1D\xD0\x81a#\xD2V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a(\x88Wa(\x88a(GV[P\x03\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP\x83`@\x83\x01R`\x80``\x83\x01Ra(\xCC`\x80\x83\x01\x84a&\xAEV[\x96\x95PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0a)\x05``\x83\x01\x85a&\xAEV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15a).Wa).a(GV[P\x01\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x83R\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01R\x80\x86\x16``\x84\x01RP\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra)\x82`\xC0\x83\x01\x84a&\xAEV[\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a)\xC3``\x83\x01\x84a&\xAEV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a)\xDEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1D\xD0W`\0\x80\xFD[\x82\x81R`@` \x82\x01R`\0a!\xF1`@\x83\x01\x84a&\xAEV[`\0\x82Qa*\x19\x81\x84` \x87\x01a&\x82V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L2STANDARDBRIDGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x12W`\x005`\xE0\x1C\x80cf*c:\x11a\0\xA5W\x80c\x8F`\x1Ff\x11a\0tW\x80c\xA3\xA7\x95H\x11a\0YW\x80c\xA3\xA7\x95H\x14a\x04(W\x80c\xC8\x97\x01\xA2\x14a\x02?W\x80c\xE1\x10\x13\xDD\x14a\x04;W`\0\x80\xFD[\x80c\x8F`\x1Ff\x14a\x03\xB7W\x80c\x92~\xDE-\x14a\x03\xFDW`\0\x80\xFD[\x80cf*c:\x14a\x03;W\x80c\x7FF\xDD\xB2\x14a\x03NW\x80c\x81)\xFC\x1C\x14a\x03\x82W\x80c\x87\x08v#\x14a\x03\x97W`\0\x80\xFD[\x80c6\xC7\x17\xC1\x11a\0\xE1W\x80c6\xC7\x17\xC1\x14a\x02?W\x80c<\xB7G\xBF\x14a\x02\x98W\x80cT\n\xBFs\x14a\x02\xC5W\x80cT\xFDMP\x14a\x02\xE5W`\0\x80\xFD[\x80c\x01f\xA0z\x14a\x01\xE6W\x80c\t\xFC\x88C\x14a\x02\x06W\x80c\x165\xF5\xFD\x14a\x02\x19W\x80c2\xB7\0m\x14a\x02,W`\0\x80\xFD[6a\x01\xE1W3;\x15a\x01\xABW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xDFs\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\x00334b\x03\r@`@Q\x80` \x01`@R\x80`\0\x81RPa\x04NV[\0[`\0\x80\xFD[4\x80\x15a\x01\xF2W`\0\x80\xFD[Pa\x01\xDFa\x02\x016`\x04a$@V[a\x05)V[a\x01\xDFa\x02\x146`\x04a$\xF1V[a\x08\xE7V[a\x01\xDFa\x02'6`\x04a%DV[a\t\xBEV[a\x01\xDFa\x02:6`\x04a%\xB7V[a\x0E,V[4\x80\x15a\x02KW`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA4W`\0\x80\xFD[P`\x03Ta\x02n\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\xD1W`\0\x80\xFD[Pa\x01\xDFa\x02\xE06`\x04a&\x0BV[a\x0F\x06V[4\x80\x15a\x02\xF1W`\0\x80\xFD[Pa\x03.`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.3.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x02\x8F\x91\x90a&\xF8V[a\x01\xDFa\x03I6`\x04a$@V[a\x0FKV[4\x80\x15a\x03ZW`\0\x80\xFD[Pa\x02n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x8EW`\0\x80\xFD[Pa\x01\xDFa\x0F\xBEV[4\x80\x15a\x03\xA3W`\0\x80\xFD[Pa\x01\xDFa\x03\xB26`\x04a'\x0BV[a\x11\x1BV[4\x80\x15a\x03\xC3W`\0\x80\xFD[Pa\x03\xEFa\x03\xD26`\x04a'\x8EV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x02\x8FV[4\x80\x15a\x04\tW`\0\x80\xFD[P`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02nV[a\x01\xDFa\x0466`\x04a'\x0BV[a\x11\xEFV[a\x01\xDFa\x04I6`\x04a'\xC7V[a\x123V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!R!R!R!R!R!R!R!R!S\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x01a\x04\x9DWa\x04\x98\x85\x85\x85\x85\x85a\x12|V[a\x05!V[`\0\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC0\x1E\x1B\xD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0E\x91\x90a(*V[\x90Pa\x05\x1F\x87\x82\x88\x88\x88\x88\x88a\x14bV[P[PPPPPPV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\x06\x18WP`\x03T`@\x80Q\x7Fn)nE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x16\x91cn)nE\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\0\x91\x90a(*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x06\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from the other bridg`d\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[a\x06\xD3\x87a\x17\xAAV[\x15a\x08!Wa\x06\xE2\x87\x87a\x18\x0CV[a\x07\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FStandardBridge: wrong remote tok`D\x82\x01R\x7Fen for Optimism Mintable ERC20 l`d\x82\x01R\x7Focal token\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x18W=`\0\x80>=`\0\xFD[PPPPa\x08\xA3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R Ta\x08_\x90\x84\x90a(vV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x94\x8C\x16\x83R\x93\x90R\x91\x90\x91 \x91\x90\x91Ua\x08\xA3\x90\x85\x85a\x19,V[a\x05\x1F\x87\x87\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\0\x92PPPV[3;\x15a\tvW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\t\xB9334\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x12|\x92PPPV[PPPV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\n\xADWP`\x03T`@\x80Q\x7Fn)nE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x16\x91cn)nE\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\nqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x95\x91\x90a(*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x0B_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from the other bridg`d\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[\x824\x14a\x0B\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FStandardBridge: amount sent does`D\x82\x01R\x7F not match amount required\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x03a\x0C\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FStandardBridge: cannot send to s`D\x82\x01R\x7Felf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x85\x16\x03a\r>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FStandardBridge: cannot send to m`D\x82\x01R\x7Fessenger\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\r\x80\x85\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\x8E\x92PPPV[`\0a\r\x9D\x85Z\x86`@Q\x80` \x01`@R\x80`\0\x81RPa\x1B/V[\x90P\x80a\x05!W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FStandardBridge: ETH transfer fai`D\x82\x01R\x7Fled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[3;\x15a\x0E\xBBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\x0E\xFF\x8533\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04N\x92PPPV[PPPPPV[a\x05\x1F\x87\x873\x88\x88\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14b\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x15\x80\x15a\x0F\x98WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\0\x14[\x15a\x0F\xAFWa\x0F\xAA\x85\x85\x85\x85\x85a\t\xBEV[a\x05\x1FV[a\x05\x1F\x86\x88\x87\x87\x87\x87\x87a\x05)V[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x0F\xE0WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x10lW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x10\xBAsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07a\x1BIV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[3;\x15a\x11\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStandardBridge: function can onl`D\x82\x01R\x7Fy be called from an EOA\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\x05!\x86\x8633\x88\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14b\x92PPPV[a\x05!\x863\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04N\x92PPPV[a\x12v3\x854\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x12|\x92PPPV[PPPPV[\x824\x14a\x13\x0BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStandardBridge: bridging ETH mus`D\x82\x01R\x7Ft include sufficient ETH value\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[a\x13\x17\x85\x85\x85\x84a\x1C'V[`\x03T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c=\xBB +\x90\x85\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x7F\x165\xF5\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x13\x96\x90\x8B\x90\x8B\x90\x86\x90\x8A\x90`$\x01a(\x8DV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x94\x85\x16\x17\x90RQ`\xE0\x86\x90\x1B\x90\x92\x16\x82Ra\x14)\x92\x91\x88\x90`\x04\x01a(\xD6V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x14BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14VW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[a\x14k\x87a\x17\xAAV[\x15a\x15\xB9Wa\x14z\x87\x87a\x18\x0CV[a\x15,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FStandardBridge: wrong remote tok`D\x82\x01R\x7Fen for Optimism Mintable ERC20 l`d\x82\x01R\x7Focal token\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x01\xA2V[`@Q\x7F\x9D\xC2\x9F\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xB0W=`\0\x80>=`\0\xFD[PPPPa\x16MV[a\x15\xDBs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x860\x86a\x1C\xC8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R Ta\x16\x19\x90\x84\x90a)\x1BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R U[a\x16[\x87\x87\x87\x87\x87\x86a\x1D&V[`\x03T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c=\xBB +\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x7F\x01f\xA0z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x16\xDC\x90\x8B\x90\x8D\x90\x8C\x90\x8C\x90\x8C\x90\x8B\x90`$\x01a)3V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x94\x85\x16\x17\x90RQ`\xE0\x85\x90\x1B\x90\x92\x16\x82Ra\x17o\x92\x91\x87\x90`\x04\x01a(\xD6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x89W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x9DW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`\0a\x17\xD6\x82\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\xB4V[\x80a\x18\x06WPa\x18\x06\x82\x7F\xECO\xC8\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\xB4V[\x92\x91PPV[`\0a\x188\x83\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\xB4V[\x15a\x18\xE1W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC0\x1E\x1B\xD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xAC\x91\x90a(*V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x90Pa\x18\x06V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD6\xC0\xB2\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x88W=`\0\x80>=`\0\xFD[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\t\xB9\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1D\xD7V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB0DE#&\x87\x17\xA0&\x98\xBEG\xD0\x80:\xA7F\x8C\0\xAC\xBE\xD2\xF8\xBD\x93\xA0E\x9C\xDEa\xDD\x89\x86\x86\x86`@Qa\x1Ax\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x05!\x86\x86\x86\x86\x86\x86a\x1E\xE3V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB0DE#&\x87\x17\xA0&\x98\xBEG\xD0\x80:\xA7F\x8C\0\xAC\xBE\xD2\xF8\xBD\x93\xA0E\x9C\xDEa\xDD\x89\x86\x86\x86`@Qa\x1B\x1B\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x12v\x84\x84\x84\x84a\x1FkV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1B\xE0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fs\xD1p\x91\n\xBA\x9EmP\xB1\x02\xDBR+\x1D\xBC\xD7\x96!oQ(\xB4E\xAA!5'(\x86I~\x86\x86\x86`@Qa\x1C\xB4\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x12v\x84\x84\x84\x84a\x1F\xD8V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x12v\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01a\x19~V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fs\xD1p\x91\n\xBA\x9EmP\xB1\x02\xDBR+\x1D\xBC\xD7\x96!oQ(\xB4E\xAA!5'(\x86I~\x86\x86\x86`@Qa\x1D\x9E\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4a\x05!\x86\x86\x86\x86\x86\x86a 7V[`\0a\x1D\xBF\x83a \xAFV[\x80\x15a\x1D\xD0WPa\x1D\xD0\x83\x83a!\x13V[\x93\x92PPPV[`\0a\x1E9\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!\xE2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\t\xB9W\x80\x80` \x01\x90Q\x81\x01\x90a\x1EW\x91\x90a)\xCCV[a\t\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD5\x9Ce\xB3TE\"X5\xC8?P\xB6\xED\xE0j{\xE0G\xD2.5ps\xE2P\xD9\xAFSu\x18\xCD\x86\x86\x86`@Qa\x1F[\x93\x92\x91\x90a)\x8EV[`@Q\x80\x91\x03\x90\xA4PPPPPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F1\xB2\x16o\xF6\x04\xFCVr\xEA]\xF0\x8Ax\x08\x1D+\xC6\xD7F\xCA\xDC\xE8\x80t\x7F6C\xD8\x19\xE8=\x84\x84`@Qa\x1F\xCA\x92\x91\x90a)\xEEV[`@Q\x80\x91\x03\x90\xA3PPPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F(I\xB40t\t:\x059ko*\x93}\xEE\x85e\xB1ZH\xA7\xB3\xD4\xBF\xFBs*P\x178\n\xF5\x84\x84`@Qa\x1F\xCA\x92\x91\x90a)\xEEV[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x7F\xF1&\xDB\x80$BK\xBF\xD9\x82n\x8A\xB8/\xF5\x916(\x9E\xA4@\xB0K9\xA0\xDF\x1B\x03\xB9\xCA\xBF\x86\x86\x86`@Qa\x1F[\x93\x92\x91\x90a)\x8EV[`\0a \xDB\x82\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a!\x13V[\x80\x15a\x18\x06WPa!\x0C\x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a!\x13V[\x15\x92\x91PPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x81R\x82Q`\0\x93\x92\x84\x92\x83\x92\x83\x92\x91\x83\x91\x90\x8Aau0\xFA\x92P=\x91P`\0Q\x90P\x82\x80\x15a!\xCBWP` \x82\x10\x15[\x80\x15a!\xD7WP`\0\x81\x11[\x97\x96PPPPPPPV[``a!\xF1\x84\x84`\0\x85a!\xF9V[\x94\x93PPPPV[``\x82G\x10\x15a\"\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xA2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a#\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\xA2V[`\0\x80\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x87`@Qa#2\x91\x90a*\x07V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a#oW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#tV[``\x91P[P\x91P\x91Pa!\xD7\x82\x82\x86``\x83\x15a#\x8EWP\x81a\x1D\xD0V[\x82Q\x15a#\x9EW\x82Q\x80\x84` \x01\xFD[\x81`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xA2\x91\x90a&\xF8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#\xF4W`\0\x80\xFD[PV[`\0\x80\x83`\x1F\x84\x01\x12a$\tW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a$9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a$[W`\0\x80\xFD[\x875a$f\x81a#\xD2V[\x96P` \x88\x015a$v\x81a#\xD2V[\x95P`@\x88\x015a$\x86\x81a#\xD2V[\x94P``\x88\x015a$\x96\x81a#\xD2V[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xB9W`\0\x80\xFD[a$\xC5\x8A\x82\x8B\x01a#\xF7V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$\xECW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\x06W`\0\x80\xFD[a%\x0F\x84a$\xD8V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%+W`\0\x80\xFD[a%7\x86\x82\x87\x01a#\xF7V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a%\\W`\0\x80\xFD[\x855a%g\x81a#\xD2V[\x94P` \x86\x015a%w\x81a#\xD2V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x9AW`\0\x80\xFD[a%\xA6\x88\x82\x89\x01a#\xF7V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a%\xCFW`\0\x80\xFD[\x855a%\xDA\x81a#\xD2V[\x94P` \x86\x015\x93Pa%\xEF`@\x87\x01a$\xD8V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x9AW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a&&W`\0\x80\xFD[\x875a&1\x81a#\xD2V[\x96P` \x88\x015a&A\x81a#\xD2V[\x95P`@\x88\x015a&Q\x81a#\xD2V[\x94P``\x88\x015\x93Pa&f`\x80\x89\x01a$\xD8V[\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xB9W`\0\x80\xFD[`\0[\x83\x81\x10\x15a&\x9DW\x81\x81\x01Q\x83\x82\x01R` \x01a&\x85V[\x83\x81\x11\x15a\x12vWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\xC6\x81` \x86\x01` \x86\x01a&\x82V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1D\xD0` \x83\x01\x84a&\xAEV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a'$W`\0\x80\xFD[\x865a'/\x81a#\xD2V[\x95P` \x87\x015a'?\x81a#\xD2V[\x94P`@\x87\x015\x93Pa'T``\x88\x01a$\xD8V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'pW`\0\x80\xFD[a'|\x89\x82\x8A\x01a#\xF7V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\xA1W`\0\x80\xFD[\x825a'\xAC\x81a#\xD2V[\x91P` \x83\x015a'\xBC\x81a#\xD2V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a'\xDDW`\0\x80\xFD[\x845a'\xE8\x81a#\xD2V[\x93Pa'\xF6` \x86\x01a$\xD8V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x12W`\0\x80\xFD[a(\x1E\x87\x82\x88\x01a#\xF7V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a(<W`\0\x80\xFD[\x81Qa\x1D\xD0\x81a#\xD2V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a(\x88Wa(\x88a(GV[P\x03\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP\x83`@\x83\x01R`\x80``\x83\x01Ra(\xCC`\x80\x83\x01\x84a&\xAEV[\x96\x95PPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0a)\x05``\x83\x01\x85a&\xAEV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15a).Wa).a(GV[P\x01\x90V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x83R\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01R\x80\x86\x16``\x84\x01RP\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra)\x82`\xC0\x83\x01\x84a&\xAEV[\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a)\xC3``\x83\x01\x84a&\xAEV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a)\xDEW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1D\xD0W`\0\x80\xFD[\x82\x81R`@` \x82\x01R`\0a!\xF1`@\x83\x01\x84a&\xAEV[`\0\x82Qa*\x19\x81\x84` \x87\x01a&\x82V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L2STANDARDBRIDGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L2StandardBridge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L2StandardBridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L2StandardBridge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L2StandardBridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L2StandardBridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L2StandardBridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L2StandardBridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L2STANDARDBRIDGE_ABI.clone(),
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
                L2STANDARDBRIDGE_ABI.clone(),
                L2STANDARDBRIDGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MESSENGER` (0x927ede2d) function
        pub fn MESSENGER(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([146, 126, 222, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OTHER_BRIDGE` (0x7f46ddb2) function
        pub fn OTHER_BRIDGE(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([127, 70, 221, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeERC20` (0x87087623) function
        pub fn bridge_erc20(
            &self,
            local_token: ::ethers::core::types::Address,
            remote_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [135, 8, 118, 35],
                    (local_token, remote_token, amount, min_gas_limit, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeERC20To` (0x540abf73) function
        pub fn bridge_erc20_to(
            &self,
            local_token: ::ethers::core::types::Address,
            remote_token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [84, 10, 191, 115],
                    (local_token, remote_token, to, amount, min_gas_limit, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeETH` (0x09fc8843) function
        pub fn bridge_eth(
            &self,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 252, 136, 67], (min_gas_limit, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeETHTo` (0xe11013dd) function
        pub fn bridge_eth_to(
            &self,
            to: ::ethers::core::types::Address,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 16, 19, 221], (to, min_gas_limit, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposits` (0x8f601f66) function
        pub fn deposits(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 96, 31, 102], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizeBridgeERC20` (0x0166a07a) function
        pub fn finalize_bridge_erc20(
            &self,
            local_token: ::ethers::core::types::Address,
            remote_token: ::ethers::core::types::Address,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [1, 102, 160, 122],
                    (local_token, remote_token, from, to, amount, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizeBridgeETH` (0x1635f5fd) function
        pub fn finalize_bridge_eth(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 53, 245, 253], (from, to, amount, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizeDeposit` (0x662a633a) function
        pub fn finalize_deposit(
            &self,
            l_1_token: ::ethers::core::types::Address,
            l_2_token: ::ethers::core::types::Address,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [102, 42, 99, 58],
                    (l_1_token, l_2_token, from, to, amount, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1TokenBridge` (0x36c717c1) function
        pub fn l_1_token_bridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([54, 199, 23, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messenger` (0x3cb747bf) function
        pub fn messenger(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([60, 183, 71, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `otherBridge` (0xc89701a2) function
        pub fn otherBridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 151, 1, 162], ())
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
        ///Calls the contract's `withdraw` (0x32b7006d) function
        pub fn withdraw(
            &self,
            l_2_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [50, 183, 0, 109],
                    (l_2_token, amount, min_gas_limit, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTo` (0xa3a79548) function
        pub fn withdraw_to(
            &self,
            l_2_token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [163, 167, 149, 72],
                    (l_2_token, to, amount, min_gas_limit, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DepositFinalized` event
        pub fn deposit_finalized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositFinalizedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ERC20BridgeFinalized` event
        pub fn erc20_bridge_finalized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Erc20BridgeFinalizedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ERC20BridgeInitiated` event
        pub fn erc20_bridge_initiated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Erc20BridgeInitiatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ETHBridgeFinalized` event
        pub fn eth_bridge_finalized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EthbridgeFinalizedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ETHBridgeInitiated` event
        pub fn eth_bridge_initiated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EthbridgeInitiatedFilter,
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
        ///Gets the contract's `WithdrawalInitiated` event
        pub fn withdrawal_initiated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalInitiatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            L2StandardBridgeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L2StandardBridge<M> {
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
        name = "DepositFinalized",
        abi = "DepositFinalized(address,address,address,address,uint256,bytes)"
    )]
    pub struct DepositFinalizedFilter {
        #[ethevent(indexed)]
        pub l_1_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub l_2_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
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
        name = "ERC20BridgeFinalized",
        abi = "ERC20BridgeFinalized(address,address,address,address,uint256,bytes)"
    )]
    pub struct Erc20BridgeFinalizedFilter {
        #[ethevent(indexed)]
        pub local_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub remote_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
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
        name = "ERC20BridgeInitiated",
        abi = "ERC20BridgeInitiated(address,address,address,address,uint256,bytes)"
    )]
    pub struct Erc20BridgeInitiatedFilter {
        #[ethevent(indexed)]
        pub local_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub remote_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
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
        name = "ETHBridgeFinalized",
        abi = "ETHBridgeFinalized(address,address,uint256,bytes)"
    )]
    pub struct EthbridgeFinalizedFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
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
        name = "ETHBridgeInitiated",
        abi = "ETHBridgeInitiated(address,address,uint256,bytes)"
    )]
    pub struct EthbridgeInitiatedFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
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
    #[ethevent(
        name = "WithdrawalInitiated",
        abi = "WithdrawalInitiated(address,address,address,address,uint256,bytes)"
    )]
    pub struct WithdrawalInitiatedFilter {
        #[ethevent(indexed)]
        pub l_1_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub l_2_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L2StandardBridgeEvents {
        DepositFinalizedFilter(DepositFinalizedFilter),
        Erc20BridgeFinalizedFilter(Erc20BridgeFinalizedFilter),
        Erc20BridgeInitiatedFilter(Erc20BridgeInitiatedFilter),
        EthbridgeFinalizedFilter(EthbridgeFinalizedFilter),
        EthbridgeInitiatedFilter(EthbridgeInitiatedFilter),
        InitializedFilter(InitializedFilter),
        WithdrawalInitiatedFilter(WithdrawalInitiatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for L2StandardBridgeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFinalizedFilter::decode_log(log) {
                return Ok(L2StandardBridgeEvents::DepositFinalizedFilter(decoded));
            }
            if let Ok(decoded) = Erc20BridgeFinalizedFilter::decode_log(log) {
                return Ok(L2StandardBridgeEvents::Erc20BridgeFinalizedFilter(decoded));
            }
            if let Ok(decoded) = Erc20BridgeInitiatedFilter::decode_log(log) {
                return Ok(L2StandardBridgeEvents::Erc20BridgeInitiatedFilter(decoded));
            }
            if let Ok(decoded) = EthbridgeFinalizedFilter::decode_log(log) {
                return Ok(L2StandardBridgeEvents::EthbridgeFinalizedFilter(decoded));
            }
            if let Ok(decoded) = EthbridgeInitiatedFilter::decode_log(log) {
                return Ok(L2StandardBridgeEvents::EthbridgeInitiatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(L2StandardBridgeEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalInitiatedFilter::decode_log(log) {
                return Ok(L2StandardBridgeEvents::WithdrawalInitiatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L2StandardBridgeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFinalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Erc20BridgeFinalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Erc20BridgeInitiatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EthbridgeFinalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EthbridgeInitiatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalInitiatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DepositFinalizedFilter> for L2StandardBridgeEvents {
        fn from(value: DepositFinalizedFilter) -> Self {
            Self::DepositFinalizedFilter(value)
        }
    }
    impl ::core::convert::From<Erc20BridgeFinalizedFilter> for L2StandardBridgeEvents {
        fn from(value: Erc20BridgeFinalizedFilter) -> Self {
            Self::Erc20BridgeFinalizedFilter(value)
        }
    }
    impl ::core::convert::From<Erc20BridgeInitiatedFilter> for L2StandardBridgeEvents {
        fn from(value: Erc20BridgeInitiatedFilter) -> Self {
            Self::Erc20BridgeInitiatedFilter(value)
        }
    }
    impl ::core::convert::From<EthbridgeFinalizedFilter> for L2StandardBridgeEvents {
        fn from(value: EthbridgeFinalizedFilter) -> Self {
            Self::EthbridgeFinalizedFilter(value)
        }
    }
    impl ::core::convert::From<EthbridgeInitiatedFilter> for L2StandardBridgeEvents {
        fn from(value: EthbridgeInitiatedFilter) -> Self {
            Self::EthbridgeInitiatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for L2StandardBridgeEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalInitiatedFilter> for L2StandardBridgeEvents {
        fn from(value: WithdrawalInitiatedFilter) -> Self {
            Self::WithdrawalInitiatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `MESSENGER` function with signature `MESSENGER()` and selector `0x927ede2d`
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
    #[ethcall(name = "MESSENGER", abi = "MESSENGER()")]
    pub struct MESSENGERCall;
    ///Container type for all input parameters for the `OTHER_BRIDGE` function with signature `OTHER_BRIDGE()` and selector `0x7f46ddb2`
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
    #[ethcall(name = "OTHER_BRIDGE", abi = "OTHER_BRIDGE()")]
    pub struct OTHER_BRIDGECall;
    ///Container type for all input parameters for the `bridgeERC20` function with signature `bridgeERC20(address,address,uint256,uint32,bytes)` and selector `0x87087623`
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
        name = "bridgeERC20",
        abi = "bridgeERC20(address,address,uint256,uint32,bytes)"
    )]
    pub struct BridgeERC20Call {
        pub local_token: ::ethers::core::types::Address,
        pub remote_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `bridgeERC20To` function with signature `bridgeERC20To(address,address,address,uint256,uint32,bytes)` and selector `0x540abf73`
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
        name = "bridgeERC20To",
        abi = "bridgeERC20To(address,address,address,uint256,uint32,bytes)"
    )]
    pub struct BridgeERC20ToCall {
        pub local_token: ::ethers::core::types::Address,
        pub remote_token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `bridgeETH` function with signature `bridgeETH(uint32,bytes)` and selector `0x09fc8843`
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
    #[ethcall(name = "bridgeETH", abi = "bridgeETH(uint32,bytes)")]
    pub struct BridgeETHCall {
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `bridgeETHTo` function with signature `bridgeETHTo(address,uint32,bytes)` and selector `0xe11013dd`
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
    #[ethcall(name = "bridgeETHTo", abi = "bridgeETHTo(address,uint32,bytes)")]
    pub struct BridgeETHToCall {
        pub to: ::ethers::core::types::Address,
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deposits` function with signature `deposits(address,address)` and selector `0x8f601f66`
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
    #[ethcall(name = "deposits", abi = "deposits(address,address)")]
    pub struct DepositsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `finalizeBridgeERC20` function with signature `finalizeBridgeERC20(address,address,address,address,uint256,bytes)` and selector `0x0166a07a`
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
        name = "finalizeBridgeERC20",
        abi = "finalizeBridgeERC20(address,address,address,address,uint256,bytes)"
    )]
    pub struct FinalizeBridgeERC20Call {
        pub local_token: ::ethers::core::types::Address,
        pub remote_token: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `finalizeBridgeETH` function with signature `finalizeBridgeETH(address,address,uint256,bytes)` and selector `0x1635f5fd`
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
        name = "finalizeBridgeETH",
        abi = "finalizeBridgeETH(address,address,uint256,bytes)"
    )]
    pub struct FinalizeBridgeETHCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `finalizeDeposit` function with signature `finalizeDeposit(address,address,address,address,uint256,bytes)` and selector `0x662a633a`
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
        name = "finalizeDeposit",
        abi = "finalizeDeposit(address,address,address,address,uint256,bytes)"
    )]
    pub struct FinalizeDepositCall {
        pub l_1_token: ::ethers::core::types::Address,
        pub l_2_token: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
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
    ///Container type for all input parameters for the `l1TokenBridge` function with signature `l1TokenBridge()` and selector `0x36c717c1`
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
    #[ethcall(name = "l1TokenBridge", abi = "l1TokenBridge()")]
    pub struct L1TokenBridgeCall;
    ///Container type for all input parameters for the `messenger` function with signature `messenger()` and selector `0x3cb747bf`
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
    #[ethcall(name = "messenger", abi = "messenger()")]
    pub struct messengerCall;
    ///Container type for all input parameters for the `otherBridge` function with signature `otherBridge()` and selector `0xc89701a2`
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
    #[ethcall(name = "otherBridge", abi = "otherBridge()")]
    pub struct otherBridgeCall;
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
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256,uint32,bytes)` and selector `0x32b7006d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256,uint32,bytes)")]
    pub struct WithdrawCall {
        pub l_2_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdrawTo` function with signature `withdrawTo(address,address,uint256,uint32,bytes)` and selector `0xa3a79548`
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
        name = "withdrawTo",
        abi = "withdrawTo(address,address,uint256,uint32,bytes)"
    )]
    pub struct WithdrawToCall {
        pub l_2_token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L2StandardBridgeCalls {
        MESSENGER(MESSENGERCall),
        OTHER_BRIDGE(OTHER_BRIDGECall),
        BridgeERC20(BridgeERC20Call),
        BridgeERC20To(BridgeERC20ToCall),
        BridgeETH(BridgeETHCall),
        BridgeETHTo(BridgeETHToCall),
        Deposits(DepositsCall),
        FinalizeBridgeERC20(FinalizeBridgeERC20Call),
        FinalizeBridgeETH(FinalizeBridgeETHCall),
        FinalizeDeposit(FinalizeDepositCall),
        Initialize(InitializeCall),
        L1TokenBridge(L1TokenBridgeCall),
        messenger(messengerCall),
        otherBridge(otherBridgeCall),
        Version(VersionCall),
        Withdraw(WithdrawCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ::ethers::core::abi::AbiDecode for L2StandardBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MESSENGERCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MESSENGER(decoded));
            }
            if let Ok(decoded) = <OTHER_BRIDGECall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OTHER_BRIDGE(decoded));
            }
            if let Ok(decoded) = <BridgeERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BridgeERC20(decoded));
            }
            if let Ok(decoded) = <BridgeERC20ToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BridgeERC20To(decoded));
            }
            if let Ok(decoded) = <BridgeETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BridgeETH(decoded));
            }
            if let Ok(decoded) = <BridgeETHToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BridgeETHTo(decoded));
            }
            if let Ok(decoded) = <DepositsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposits(decoded));
            }
            if let Ok(decoded) = <FinalizeBridgeERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FinalizeBridgeERC20(decoded));
            }
            if let Ok(decoded) = <FinalizeBridgeETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FinalizeBridgeETH(decoded));
            }
            if let Ok(decoded) = <FinalizeDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FinalizeDeposit(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <L1TokenBridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1TokenBridge(decoded));
            }
            if let Ok(decoded) = <messengerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::messenger(decoded));
            }
            if let Ok(decoded) = <otherBridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::otherBridge(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L2StandardBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MESSENGER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OTHER_BRIDGE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeERC20To(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeETHTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizeBridgeERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizeBridgeETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizeDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1TokenBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::messenger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::otherBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for L2StandardBridgeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MESSENGER(element) => ::core::fmt::Display::fmt(element, f),
                Self::OTHER_BRIDGE(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeERC20To(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeETHTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizeBridgeERC20(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FinalizeBridgeETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizeDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1TokenBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::messenger(element) => ::core::fmt::Display::fmt(element, f),
                Self::otherBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MESSENGERCall> for L2StandardBridgeCalls {
        fn from(value: MESSENGERCall) -> Self {
            Self::MESSENGER(value)
        }
    }
    impl ::core::convert::From<OTHER_BRIDGECall> for L2StandardBridgeCalls {
        fn from(value: OTHER_BRIDGECall) -> Self {
            Self::OTHER_BRIDGE(value)
        }
    }
    impl ::core::convert::From<BridgeERC20Call> for L2StandardBridgeCalls {
        fn from(value: BridgeERC20Call) -> Self {
            Self::BridgeERC20(value)
        }
    }
    impl ::core::convert::From<BridgeERC20ToCall> for L2StandardBridgeCalls {
        fn from(value: BridgeERC20ToCall) -> Self {
            Self::BridgeERC20To(value)
        }
    }
    impl ::core::convert::From<BridgeETHCall> for L2StandardBridgeCalls {
        fn from(value: BridgeETHCall) -> Self {
            Self::BridgeETH(value)
        }
    }
    impl ::core::convert::From<BridgeETHToCall> for L2StandardBridgeCalls {
        fn from(value: BridgeETHToCall) -> Self {
            Self::BridgeETHTo(value)
        }
    }
    impl ::core::convert::From<DepositsCall> for L2StandardBridgeCalls {
        fn from(value: DepositsCall) -> Self {
            Self::Deposits(value)
        }
    }
    impl ::core::convert::From<FinalizeBridgeERC20Call> for L2StandardBridgeCalls {
        fn from(value: FinalizeBridgeERC20Call) -> Self {
            Self::FinalizeBridgeERC20(value)
        }
    }
    impl ::core::convert::From<FinalizeBridgeETHCall> for L2StandardBridgeCalls {
        fn from(value: FinalizeBridgeETHCall) -> Self {
            Self::FinalizeBridgeETH(value)
        }
    }
    impl ::core::convert::From<FinalizeDepositCall> for L2StandardBridgeCalls {
        fn from(value: FinalizeDepositCall) -> Self {
            Self::FinalizeDeposit(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for L2StandardBridgeCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<L1TokenBridgeCall> for L2StandardBridgeCalls {
        fn from(value: L1TokenBridgeCall) -> Self {
            Self::L1TokenBridge(value)
        }
    }
    impl ::core::convert::From<messengerCall> for L2StandardBridgeCalls {
        fn from(value: messengerCall) -> Self {
            Self::messenger(value)
        }
    }
    impl ::core::convert::From<otherBridgeCall> for L2StandardBridgeCalls {
        fn from(value: otherBridgeCall) -> Self {
            Self::otherBridge(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L2StandardBridgeCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for L2StandardBridgeCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawToCall> for L2StandardBridgeCalls {
        fn from(value: WithdrawToCall) -> Self {
            Self::WithdrawTo(value)
        }
    }
    ///Container type for all return fields from the `MESSENGER` function with signature `MESSENGER()` and selector `0x927ede2d`
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
    pub struct MESSENGERReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `OTHER_BRIDGE` function with signature `OTHER_BRIDGE()` and selector `0x7f46ddb2`
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
    pub struct OTHER_BRIDGEReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deposits` function with signature `deposits(address,address)` and selector `0x8f601f66`
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
    pub struct DepositsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l1TokenBridge` function with signature `l1TokenBridge()` and selector `0x36c717c1`
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
    pub struct L1TokenBridgeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `messenger` function with signature `messenger()` and selector `0x3cb747bf`
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
    pub struct messengerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `otherBridge` function with signature `otherBridge()` and selector `0xc89701a2`
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
    pub struct otherBridgeReturn(pub ::ethers::core::types::Address);
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
}

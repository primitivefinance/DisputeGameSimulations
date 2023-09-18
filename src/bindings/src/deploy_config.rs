pub use deploy_config::*;
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
pub mod deploy_config {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_path"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
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
                    ::std::borrow::ToOwned::to_owned("baseFeeVaultRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "baseFeeVaultRecipient",
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
                    ::std::borrow::ToOwned::to_owned("batchInboxAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchInboxAddress"),
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
                    ::std::borrow::ToOwned::to_owned("batchSenderAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchSenderAddress"),
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
                    ::std::borrow::ToOwned::to_owned("channelTimeout"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("channelTimeout"),
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
                    ::std::borrow::ToOwned::to_owned("eip1559Denominator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eip1559Denominator"),
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
                    ::std::borrow::ToOwned::to_owned("eip1559Elasticity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eip1559Elasticity"),
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
                    ::std::borrow::ToOwned::to_owned("faultGameAbsolutePrestate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "faultGameAbsolutePrestate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("faultGameMaxDepth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("faultGameMaxDepth"),
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
                    ::std::borrow::ToOwned::to_owned("faultGameMaxDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "faultGameMaxDuration",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("finalSystemOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("finalSystemOwner"),
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
                    ::std::borrow::ToOwned::to_owned("finalizationPeriodSeconds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "finalizationPeriodSeconds",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("gasPriceOracleOverhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "gasPriceOracleOverhead",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("gasPriceOracleScalar"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "gasPriceOracleScalar",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("governanceTokenName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "governanceTokenName",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("governanceTokenOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "governanceTokenOwner",
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
                    ::std::borrow::ToOwned::to_owned("governanceTokenSymbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "governanceTokenSymbol",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("l1ChainID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1ChainID"),
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
                    ::std::borrow::ToOwned::to_owned("l1FeeVaultRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l1FeeVaultRecipient",
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
                    ::std::borrow::ToOwned::to_owned("l1StartingBlockTag"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1StartingBlockTag"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("l2BlockTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l2BlockTime"),
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
                    ::std::borrow::ToOwned::to_owned("l2ChainID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l2ChainID"),
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
                    ::std::borrow::ToOwned::to_owned("l2GenesisBlockBaseFeePerGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l2GenesisBlockBaseFeePerGas",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("l2GenesisBlockGasLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l2GenesisBlockGasLimit",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("l2OutputOracleChallenger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l2OutputOracleChallenger",
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
                    ::std::borrow::ToOwned::to_owned("l2OutputOracleProposer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l2OutputOracleProposer",
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
                    ::std::borrow::ToOwned::to_owned(
                        "l2OutputOracleStartingBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l2OutputOracleStartingBlockNumber",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("l2OutputOracleStartingTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l2OutputOracleStartingTimestamp",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("l2OutputOracleSubmissionInterval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l2OutputOracleSubmissionInterval",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("maxSequencerDrift"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxSequencerDrift"),
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
                    ::std::borrow::ToOwned::to_owned("p2pSequencerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "p2pSequencerAddress",
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
                    ::std::borrow::ToOwned::to_owned("portalGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("portalGuardian"),
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
                    ::std::borrow::ToOwned::to_owned("proxyAdminOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxyAdminOwner"),
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
                    ::std::borrow::ToOwned::to_owned("recommendedProtocolVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "recommendedProtocolVersion",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("requiredProtocolVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requiredProtocolVersion",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("sequencerFeeVaultRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sequencerFeeVaultRecipient",
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
                    ::std::borrow::ToOwned::to_owned("sequencerWindowSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sequencerWindowSize",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("systemConfigStartBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "systemConfigStartBlock",
                            ),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEPLOYCONFIG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x04\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0C\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15b\0\0.W`\0\x80\xFD[P`@Qb\0A&8\x03\x80b\0A&\x839\x81\x01`@\x81\x90Rb\0\0Q\x91b\0%tV[b\0\0\x9D`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7FDeployConfig: reading file %s\0\0\0\x81RP\x82b\0\"\x94` \x1Bb\0\x0C}\x17` \x1CV[`@Qc`\xF9\xBB\x11`\xE0\x1B\x81R`\0\x80Q` b\0@R\x839\x81Q\x91R\x90c`\xF9\xBB\x11\x90b\0\0\xD1\x90\x84\x90`\x04\x01b\0&ZV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15b\0\x01\x12WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x01\x0F\x91\x90\x81\x01\x90b\0%tV[`\x01[b\0\x01LWb\0\x01F`@Q\x80`\x80\x01`@R\x80`N\x81R` \x01b\0@\xD8`N\x919b\0\"\xE5` \x1Bb\0\r\x12\x17` \x1CV[b\0\"\x8DV[`\rb\0\x01Z\x82\x82b\0&\xFEV[PPb\0\x02/`\r\x80Tb\0\x01o\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x01\x9D\x90b\0&oV[\x80\x15b\0\x01\xEEW\x80`\x1F\x10b\0\x01\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x01\xEEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x01\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x12\x1734\xB70\xB6)\xBC\xB9\xBA2\xB6\xA7\xBB\xB72\xB9`q\x1B\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x0E`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x03&`\r\x80Tb\0\x02h\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x02\x96\x90b\0&oV[\x80\x15b\0\x02\xE7W\x80`\x1F\x10b\0\x02\xBBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x02\xE7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x02\xC9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x12\x1787\xB9:0\xB6#\xBA\xB0\xB924\xB0\xB7`\x81\x1B\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x0F`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x04\x18`\r\x80Tb\0\x03_\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x03\x8D\x90b\0&oV[\x80\x15b\0\x03\xDEW\x80`\x1F\x10b\0\x03\xB2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x03\xDEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x03\xC0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\t\x0B\x9B\x0CP\xDA\x18Z[\x92Q`\xAA\x1B\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x10U`\r\x80Tb\0\x04\xE9\x91\x90b\0\x040\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x04^\x90b\0&oV[\x80\x15b\0\x04\xAFW\x80`\x1F\x10b\0\x04\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x04\xAFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x04\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\t\x0B\x9B\x0C\x90\xDA\x18Z[\x92Q`\xAA\x1B\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x11U`\r\x80Tb\0\x05\xBC\x91\x90b\0\x05\x01\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x05/\x90b\0&oV[\x80\x15b\0\x05\x80W\x80`\x1F\x10b\0\x05TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x05\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x05bW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l$.l2BlockTime`\x98\x1B\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x12U`\r\x80Tb\0\x06\x9F\x91\x90b\0\x05\xD4\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x06\x02\x90b\0&oV[\x80\x15b\0\x06SW\x80`\x1F\x10b\0\x06'Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x06SV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x065W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7F$.maxSequencerDrift\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x13U`\r\x80Tb\0\x07\x82\x91\x90b\0\x06\xB7\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x06\xE5\x90b\0&oV[\x80\x15b\0\x076W\x80`\x1F\x10b\0\x07\nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x076V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x07\x18W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7F$.sequencerWindowSize\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x14U`\r\x80Tb\0\x08X\x91\x90b\0\x07\x9A\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x07\xC8\x90b\0&oV[\x80\x15b\0\x08\x19W\x80`\x1F\x10b\0\x07\xEDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x08\x19V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x07\xFBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\t\x0B\x98\xDA\x18[\x9B\x99[\x15\x1A[Y[\xDD]`\x82\x1B\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x15U`\r\x80Tb\0\t;\x91\x90b\0\x08p\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x08\x9E\x90b\0&oV[\x80\x15b\0\x08\xEFW\x80`\x1F\x10b\0\x08\xC3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x08\xEFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x08\xD1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7F$.p2pSequencerAddress\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x16`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\n?`\r\x80Tb\0\tt\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\t\xA2\x90b\0&oV[\x80\x15b\0\t\xF3W\x80`\x1F\x10b\0\t\xC7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\t\xF3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\t\xD5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7F$.batchInboxAddress\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x17`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x0BC`\r\x80Tb\0\nx\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\n\xA6\x90b\0&oV[\x80\x15b\0\n\xF7W\x80`\x1F\x10b\0\n\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\n\xF7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\n\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7F$.batchSenderAddress\0\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x18`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x0C+`\r\x80Tb\0\x0B|\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x0B\xAA\x90b\0&oV[\x80\x15b\0\x0B\xFBW\x80`\x1F\x10b\0\x0B\xCFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x0B\xFBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x0B\xDDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80``\x01`@R\x80`\"\x81R` \x01b\0@\xB6`\"\x919b\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x19U`\r\x80Tb\0\x0C\xF2\x91\x90b\0\x0CC\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x0Cq\x90b\0&oV[\x80\x15b\0\x0C\xC2W\x80`\x1F\x10b\0\x0C\x96Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x0C\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x0C\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80``\x01`@R\x80`!\x81R` \x01b\0@r`!\x919b\0$8` \x1Bb\0\x0E\xDB\x17` \x1CV[`\x1AU`\r\x80Tb\0\r\xB9\x91\x90b\0\r\n\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\r8\x90b\0&oV[\x80\x15b\0\r\x89W\x80`\x1F\x10b\0\r]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\r\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\rkW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80``\x01`@R\x80`#\x81R` \x01b\0@\x93`#\x919b\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x1BU`\r\x80Tb\0\x0E\x9C\x91\x90b\0\r\xD1\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\r\xFF\x90b\0&oV[\x80\x15b\0\x0EPW\x80`\x1F\x10b\0\x0E$Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x0EPV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x0E2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F$.l2OutputOracleProposer\0\0\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x1C`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x0F\xA0`\r\x80Tb\0\x0E\xD5\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x0F\x03\x90b\0&oV[\x80\x15b\0\x0FTW\x80`\x1F\x10b\0\x0F(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x0FTV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x0F6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7F$.l2OutputOracleChallenger\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x1D`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x10\xA4`\r\x80Tb\0\x0F\xD9\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x10\x07\x90b\0&oV[\x80\x15b\0\x10XW\x80`\x1F\x10b\0\x10,Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x10XV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x10:W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7F$.finalizationPeriodSeconds\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`\x1EU`\r\x80Tb\0\x11{\x91\x90b\0\x10\xBC\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x10\xEA\x90b\0&oV[\x80\x15b\0\x11;W\x80`\x1F\x10b\0\x11\x0FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x11;V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x11\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x12\x17897\xBC<\xA0\xB26\xB4\xB7'\xBB\xB72\xB9`y\x1B\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\x1F`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x12\x7F`\r\x80Tb\0\x11\xB4\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x11\xE2\x90b\0&oV[\x80\x15b\0\x123W\x80`\x1F\x10b\0\x12\x07Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x123V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x12\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F$.baseFeeVaultRecipient\0\0\0\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[` `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x13\x83`\r\x80Tb\0\x12\xB8\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x12\xE6\x90b\0&oV[\x80\x15b\0\x137W\x80`\x1F\x10b\0\x13\x0BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x137V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x13\x19W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7F$.l1FeeVaultRecipient\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`!`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x14\x87`\r\x80Tb\0\x13\xBC\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x13\xEA\x90b\0&oV[\x80\x15b\0\x14;W\x80`\x1F\x10b\0\x14\x0FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x14;V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x14\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F$.sequencerFeeVaultRecipient\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`\"`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x15\x8B`\r\x80Tb\0\x14\xC0\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x14\xEE\x90b\0&oV[\x80\x15b\0\x15?W\x80`\x1F\x10b\0\x15\x13Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x15?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x15!W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7F$.governanceTokenName\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0$q` \x1Bb\0\x0F0\x17` \x1CV[`#\x90b\0\x15\x9A\x90\x82b\0&\xFEV[Pb\0\x16y`\r\x80Tb\0\x15\xAE\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x15\xDC\x90b\0&oV[\x80\x15b\0\x16-W\x80`\x1F\x10b\0\x16\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x16-V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x16\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F$.governanceTokenSymbol\0\0\0\0\0\0\0\0\0\x81RPb\0$q` \x1Bb\0\x0F0\x17` \x1CV[`$\x90b\0\x16\x88\x90\x82b\0&\xFEV[Pb\0\x17g`\r\x80Tb\0\x16\x9C\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x16\xCA\x90b\0&oV[\x80\x15b\0\x17\x1BW\x80`\x1F\x10b\0\x16\xEFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x17\x1BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x16\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7F$.governanceTokenOwner\0\0\0\0\0\0\0\0\0\0\x81RPb\0#3` \x1Bb\0\r\xA4\x17` \x1CV[`%`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPb\0\x18k`\r\x80Tb\0\x17\xA0\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x17\xCE\x90b\0&oV[\x80\x15b\0\x18\x1FW\x80`\x1F\x10b\0\x17\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x18\x1FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x18\x01W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F$.l2GenesisBlockGasLimit\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`&U`\r\x80Tb\0\x19N\x91\x90b\0\x18\x83\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x18\xB1\x90b\0&oV[\x80\x15b\0\x19\x02W\x80`\x1F\x10b\0\x18\xD6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x19\x02V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x18\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7F$.l2GenesisBlockBaseFeePerGas\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`'U`\r\x80Tb\0\x1A1\x91\x90b\0\x19f\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x19\x94\x90b\0&oV[\x80\x15b\0\x19\xE5W\x80`\x1F\x10b\0\x19\xB9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x19\xE5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x19\xC7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F$.gasPriceOracleOverhead\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`(U`\r\x80Tb\0\x1B\x14\x91\x90b\0\x1AI\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1Aw\x90b\0&oV[\x80\x15b\0\x1A\xC8W\x80`\x1F\x10b\0\x1A\x9CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1A\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1A\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7F$.gasPriceOracleScalar\0\0\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`)U`\r\x80Tb\0\x1B\xF7\x91\x90b\0\x1B,\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1BZ\x90b\0&oV[\x80\x15b\0\x1B\xABW\x80`\x1F\x10b\0\x1B\x7FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1B\xABV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1B\x8DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7F$.eip1559Denominator\0\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`*U`\r\x80Tb\0\x1C\xDA\x91\x90b\0\x1C\x0F\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1C=\x90b\0&oV[\x80\x15b\0\x1C\x8EW\x80`\x1F\x10b\0\x1CbWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1C\x8EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1CpW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7F$.eip1559Elasticity\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`+U`\r\x80Tb\0\x1D\xBD\x91\x90b\0\x1C\xF2\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1D \x90b\0&oV[\x80\x15b\0\x1DqW\x80`\x1F\x10b\0\x1DEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1DqV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1DSW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F$.systemConfigStartBlock\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`/UFa\x03\x84\x14\x80b\0\x1D\xD2WPa\x059F\x14[\x15b\0\"FWb\0\x1E\xB6`\r\x80Tb\0\x1D\xEB\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1E\x19\x90b\0&oV[\x80\x15b\0\x1EjW\x80`\x1F\x10b\0\x1E>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1EjV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1ELW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7F$.faultGameAbsolutePrestate\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`,U`\r\x80Tb\0\x1F\x99\x91\x90b\0\x1E\xCE\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1E\xFC\x90b\0&oV[\x80\x15b\0\x1FMW\x80`\x1F\x10b\0\x1F!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1FMV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1F/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7F$.faultGameMaxDepth\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`-U`\r\x80Tb\0 |\x91\x90b\0\x1F\xB1\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1F\xDF\x90b\0&oV[\x80\x15b\0 0W\x80`\x1F\x10b\0 \x04Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0 0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0 \x12W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7F$.faultGameMaxDuration\0\0\0\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`.U`\r\x80Tb\0!_\x91\x90b\0 \x94\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0 \xC2\x90b\0&oV[\x80\x15b\0!\x13W\x80`\x1F\x10b\0 \xE7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0!\x13V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0 \xF5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F$.requiredProtocolVersion\0\0\0\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`0U`\r\x80Tb\0\"B\x91\x90b\0!w\x90b\0&oV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0!\xA5\x90b\0&oV[\x80\x15b\0!\xF6W\x80`\x1F\x10b\0!\xCAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0!\xF6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0!\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F$.recommendedProtocolVersion\0\0\0\0\x81RPb\0#\xB9` \x1Bb\0\x0EC\x17` \x1CV[`1U[`\x05F\x14\x80b\0\"XWPb\xAA6\xA7F\x14[\x15b\0\"\x8DWb\0\"q`\r\x80Tb\0 \x94\x90b\0&oV[`0U`\r\x80Tb\0\"\x89\x91\x90b\0!w\x90b\0&oV[`1U[Pb\0(AV[b\0\"\xE1\x82\x82`@Q`$\x01b\0\"\xAD\x92\x91\x90b\0'\xCAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x81\x16cK\\Bw`\xE0\x1B\x17\x90\x91Rb\0$\xF4\x16V[PPV[b\0#0\x81`@Q`$\x01b\0\"\xFC\x91\x90b\0&ZV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x81\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90\x91Rb\0$\xF4\x16V[PV[`@Qc\x1E\x19\xE6W`\xE0\x1B\x81R`\0\x90`\0\x80Q` b\0@R\x839\x81Q\x91R\x90c\x1E\x19\xE6W\x90b\0#l\x90\x86\x90\x86\x90`\x04\x01b\0'\xCAV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0#\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0#\xB2\x91\x90b\0'\xFCV[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R`\0\x90`\0\x80Q` b\0@R\x839\x81Q\x91R\x90c\xAD\xDD\xE2\xB6\x90b\0#\xF2\x90\x86\x90\x86\x90`\x04\x01b\0'\xCAV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0$\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0#\xB2\x91\x90b\0('V[`@Qc{\x04\x8C\xCD`\xE0\x1B\x81R`\0\x90`\0\x80Q` b\0@R\x839\x81Q\x91R\x90c{\x04\x8C\xCD\x90b\0#\xF2\x90\x86\x90\x86\x90`\x04\x01b\0'\xCAV[`@Qc\t8\x9FY`\xE3\x1B\x81R``\x90`\0\x80Q` b\0@R\x839\x81Q\x91R\x90cI\xC4\xFA\xC8\x90b\0$\xAA\x90\x86\x90\x86\x90`\x04\x01b\0'\xCAV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0$\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0#\xB2\x91\x90\x81\x01\x90b\0%tV[b\0#0\x81b\0%\n` \x1Bb\0\x0F\xEA\x17` \x1CV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0%^W\x81\x81\x01Q\x83\x82\x01R` \x01b\0%DV[\x83\x81\x11\x15b\0%nW`\0\x84\x84\x01R[PPPPV[`\0` \x82\x84\x03\x12\x15b\0%\x87W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0%\x9FW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12b\0%\xB4W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0%\xC9Wb\0%\xC9b\0%+V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0%\xF4Wb\0%\xF4b\0%+V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15b\0&\x0EW`\0\x80\xFD[b\0&!\x83` \x83\x01` \x88\x01b\0%AV[\x97\x96PPPPPPPV[`\0\x81Q\x80\x84Rb\0&F\x81` \x86\x01` \x86\x01b\0%AV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0#\xB2` \x83\x01\x84b\0&,V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0&\x84W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0&\xA5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0&\xF9W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0&\xD4WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0&\xF5W\x82\x81U`\x01\x01b\0&\xE0V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0'\x1AWb\0'\x1Ab\0%+V[b\0'2\x81b\0'+\x84Tb\0&oV[\x84b\0&\xABV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0'jW`\0\x84\x15b\0'QWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0&\xF5V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0'\x9BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0'zV[P\x85\x82\x10\x15b\0'\xBAW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R`\0b\0'\xDF`@\x83\x01\x85b\0&,V[\x82\x81\x03` \x84\x01Rb\0'\xF3\x81\x85b\0&,V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15b\0(\x0FW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0#\xB2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15b\0(:W`\0\x80\xFD[PQ\x91\x90PV[a\x18\x01\x80b\0(Q`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x92W`\x005`\xE0\x1C\x80c|\xF4\x8B@\x11a\x01`W\x80c\xAD\xD6\xCED\x11a\0\xD8W\x80c\xD25O \x11a\0\x8CW\x80c\xE3\x1E\xA1\x08\x11a\0qW\x80c\xE3\x1E\xA1\x08\x14a\x05\x10W\x80c\xE7\xD6\xCDB\x14a\x05\x19W\x80c\xF8\xCC\xBFG\x14a\x059W`\0\x80\xFD[\x80c\xD25O \x14a\x04\xD0W\x80c\xDA\xD5D\xE0\x14a\x04\xF0W`\0\x80\xFD[\x80c\xCE]\xB8\xD6\x11a\0\xBDW\x80c\xCE]\xB8\xD6\x14a\x04\xB5W\x80c\xCF{\xFE\xF5\x14a\x04\xBEW\x80c\xD2 \xA9\xE0\x14a\x04\xC7W`\0\x80\xFD[\x80c\xAD\xD6\xCED\x14a\x04\xA3W\x80c\xC9\xFF-\x16\x14a\x04\xACW`\0\x80\xFD[\x80c\x98\xF3M\xF5\x11a\x01/W\x80c\x9C\x166\x0F\x11a\x01\x14W\x80c\x9C\x166\x0F\x14a\x04qW\x80c\xA2\xAF\r\x1F\x14a\x04\x91W\x80c\xA9\xDD\xE7\xD9\x14a\x04\x9AW`\0\x80\xFD[\x80c\x98\xF3M\xF5\x14a\x04IW\x80c\x9BK\xB4\x8F\x14a\x04QW`\0\x80\xFD[\x80c|\xF4\x8B@\x14a\x04\x0EW\x80c\x81\x14:Y\x14a\x04.W\x80c\x93^8)\x14a\x047W\x80c\x93\x99\x1A\xF3\x14a\x04@W`\0\x80\xFD[\x80c2d\x95\xFB\x11a\x02\x0EW\x80cU\xD6+}\x11a\x01\xC2W\x80cd\xFBu\x80\x11a\x01\xA7W\x80cd\xFBu\x80\x14a\x03\xDDW\x80cg\xCE\xF4F\x14a\x03\xE5W\x80ch\xEA*C\x14a\x03\xEEW`\0\x80\xFD[\x80cU\xD6+}\x14a\x03\xB4W\x80c]EF\xA0\x14a\x03\xD4W`\0\x80\xFD[\x80cB\x0E\x17(\x11a\x01\xF3W\x80cB\x0E\x17(\x14a\x03vW\x80cB\xC8\x02\x95\x14a\x03\x7FW\x80cR\xF89\x8E\x14a\x03\x9FW`\0\x80\xFD[\x80c2d\x95\xFB\x14a\x03eW\x80c8\xC2qY\x14a\x03mW`\0\x80\xFD[\x80c\x10,\x9A\xA4\x11a\x02eW\x80c'kez\x11a\x02JW\x80c'kez\x14a\x03JW\x80c-\xDE6\xF5\x14a\x03SW\x80c.\xF2\xD5^\x14a\x03\\W`\0\x80\xFD[\x80c\x10,\x9A\xA4\x14a\x03!W\x80c$\x1E-~\x14a\x03*W`\0\x80\xFD[\x80c\x08\xCB\x82-\x14a\x02\x97W\x80c\t\x08\x94\xDC\x14a\x02\xE1W\x80c\n\xC7{\xB5\x14a\x03\x01W\x80c\x0B{\x1B0\x14a\x03\x18W[`\0\x80\xFD[`\x1DTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`%Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`)T\x81V[`@Q\x90\x81R` \x01a\x02\xD8V[a\x03\n`-T\x81V[a\x03\n`/T\x81V[`\x1CTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x19T\x81V[a\x03\n`&T\x81V[a\x03\n`\x15T\x81V[a\x03\na\x05VV[a\x03\n`0T\x81V[a\x03\n`.T\x81V[`\"Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xA7a\x08\xECV[`@Qa\x02\xD8\x91\x90a\x12\xBDV[`!Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x1BT\x81V[a\x03\xA7a\tzV[a\x03\n`1T\x81V[`\x18Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[` Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`(T\x81V[a\x03\n`,T\x81V[a\x03\n`\x12T\x81V[a\x03\na\t\x87V[`\x0FTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x17Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x11T\x81V[a\x03\n`'T\x81V[a\x03\n`\x14T\x81V[a\x03\n`+T\x81V[a\x03\n`\x1ET\x81V[a\x03\n`\x10T\x81V[a\x03\n`*T\x81V[`\x0ETa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x1FTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x13T\x81V[`\x16Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x0CTa\x05F\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xD8V[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x05\xAA\x90`\r\x90`\x04\x01a\x13#V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x06\x03WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x06\0\x91\x81\x01\x90a\x149V[`\x01[a\x085W`@Q\x7FI\xC4\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cI\xC4\xFA\xC8\x90a\x06X\x90`\r\x90`\x04\x01a\x13#V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x06\xB6WP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x06\xB3\x91\x90\x81\x01\x90a\x15\x13V[`\x01[a\x08,W`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x07\x0B\x90`\r\x90`\x04\x01a\x13#V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x07dWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x07a\x91\x81\x01\x90a\x149V[`\x01[\x15a\x08'W`@Q\x7Fi\0\xA3\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90Ra\x08!\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\x1C\x91\x90\x81\x01\x90a\x15\x13V[a\x10\x0BV[\x91PP\x90V[a\x08:V[a\x08!\x81a\x10\x0BV[\x91\x90PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`Z`$\x82\x01R\x7Fl1StartingBlockTag must be a byt`D\x82\x01R\x7Fes32, string or uint256 or canno`d\x82\x01R\x7Ft fetch l1StartingBlockTag\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[`#\x80Ta\x08\xF9\x90a\x12\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t%\x90a\x12\xD0V[\x80\x15a\trW\x80`\x1F\x10a\tGWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\trV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tUW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`$\x80Ta\x08\xF9\x90a\x12\xD0V[`\0\x80`\x1AT\x12\x15a\x0CvW`\0a\t\x9Da\x05VV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x91\x92P`\0\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xB9W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Fbash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\0\x81Q\x81\x10a\n\x19Wa\n\x19a\x15\\V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7F-c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\x01\x81Q\x81\x10a\nmWa\nma\x15\\V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Q\x7F\xB1\x1A\x19\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xB1\x1A\x19\xE8\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0B'\x91\x90\x81\x01\x90a\x15\x13V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7Fjq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q` \x01a\x0Bn\x92\x91\x90a\x15\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x02\x81Q\x81\x10a\x0B\x90Wa\x0B\x90a\x15\\V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Q\x7F\x89\x16\x04g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x0B\xEE\x90\x85\x90`\x04\x01a\x166V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0CS\x91\x90\x81\x01\x90a\x15\x13V[\x90Pa\x0Cn\x81`@Q\x80` \x01`@R\x80`\0\x81RPa\x0ECV[\x93PPPP\x90V[P`\x1AT\x90V[a\r\x0E\x82\x82`@Q`$\x01a\x0C\x93\x92\x91\x90a\x16\xB6V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FK\\Bw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x120V[PPV[a\r\xA1\x81`@Q`$\x01a\r&\x91\x90a\x12\xBDV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x120V[PV[`@Q\x7F\x1E\x19\xE6W\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x1E\x19\xE6W\x90a\r\xF9\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E<\x91\x90a\x16\xE4V[\x93\x92PPPV[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x0E\x98\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E<\x91\x90a\x149V[`@Q\x7F{\x04\x8C\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c{\x04\x8C\xCD\x90a\x0E\x98\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[`@Q\x7FI\xC4\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cI\xC4\xFA\xC8\x90a\x0F\x85\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0E<\x91\x90\x81\x01\x90a\x15\x13V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R`\0\x91\x82\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10&W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Fbash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\0\x81Q\x81\x10a\x10\x86Wa\x10\x86a\x15\\V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7F-c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\x01\x81Q\x81\x10a\x10\xDAWa\x10\xDAa\x15\\V[` \x02` \x01\x01\x81\x90RP\x82`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7Fjq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q` \x01a\x11-\x92\x91\x90a\x17\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x02\x81Q\x81\x10a\x11OWa\x11Oa\x15\\V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Q\x7F\x89\x16\x04g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x11\xAD\x90\x85\x90`\x04\x01a\x166V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x12\x12\x91\x90\x81\x01\x90a\x15\x13V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x12(\x91\x90a\x149V[\x94\x93PPPPV[a\r\xA1\x81a\x0F\xEAV[a\x12Aa\x17\xC5V[V[`\0[\x83\x81\x10\x15a\x12^W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12FV[\x83\x81\x11\x15a\x12mW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x12\x8B\x81` \x86\x01` \x86\x01a\x12CV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0E<` \x83\x01\x84a\x12sV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x12\xE4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\x1DW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x83T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x13CW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x13{W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x13\x9AW`\x01\x81\x14a\x13\xCEWa\x13\xF9V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x13\xF9V[`\0\x8B\x81R` \x90 `\0[\x87\x81\x10\x15a\x13\xF3W\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x13\xDAV[\x83\x01\x98PP[PP\x87\x86\x03\x90\x88\x01RPP`\x14\x83RPP\x7F$.l1StartingBlockTag\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@\x81\x01a\x12(V[`\0` \x82\x84\x03\x12\x15a\x14KW`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x14\x9CWa\x14\x9Ca\x14RV[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x14\xE2Wa\x14\xE2a\x14RV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x14\xFBW`\0\x80\xFD[a\x15\t\x86` \x83\x01\x87a\x12CV[PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x15%W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15<W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x15MW`\0\x80\xFD[a\x12(\x84\x82Q` \x84\x01a\x14\x81V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7Fcast block \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x15\xC3\x81`\x0B\x85\x01` \x88\x01a\x12CV[\x7F --json | \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x91\x84\x01\x91\x82\x01R\x83Qa\x16\0\x81`\x15\x84\x01` \x88\x01a\x12CV[\x7F .timestamp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x15\x92\x90\x91\x01\x91\x82\x01R` \x01\x94\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x16\xA9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra\x16\x97\x85\x83Qa\x12sV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x16]V[P\x92\x97\x96PPPPPPPV[`@\x81R`\0a\x16\xC9`@\x83\x01\x85a\x12sV[\x82\x81\x03` \x84\x01Ra\x16\xDB\x81\x85a\x12sV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x16\xF6W`\0\x80\xFD[\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E<W`\0\x80\xFD[\x7Fcast block \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x17R\x81`\x0B\x85\x01` \x88\x01a\x12CV[\x7F --json | \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x91\x84\x01\x91\x82\x01R\x83Qa\x17\x8F\x81`\x15\x84\x01` \x88\x01a\x12CV[\x7F -r .hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x15\x92\x90\x91\x01\x91\x82\x01R`\x1E\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`Q`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-$.l2OutputOracleStartingTimestamp$.l2OutputOracleStartingBlockNumber$.l2OutputOracleSubmissionIntervalWarning: unable to read config. Do not deploy unless you are not using config.";
    /// The bytecode of the contract.
    pub static DEPLOYCONFIG_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x92W`\x005`\xE0\x1C\x80c|\xF4\x8B@\x11a\x01`W\x80c\xAD\xD6\xCED\x11a\0\xD8W\x80c\xD25O \x11a\0\x8CW\x80c\xE3\x1E\xA1\x08\x11a\0qW\x80c\xE3\x1E\xA1\x08\x14a\x05\x10W\x80c\xE7\xD6\xCDB\x14a\x05\x19W\x80c\xF8\xCC\xBFG\x14a\x059W`\0\x80\xFD[\x80c\xD25O \x14a\x04\xD0W\x80c\xDA\xD5D\xE0\x14a\x04\xF0W`\0\x80\xFD[\x80c\xCE]\xB8\xD6\x11a\0\xBDW\x80c\xCE]\xB8\xD6\x14a\x04\xB5W\x80c\xCF{\xFE\xF5\x14a\x04\xBEW\x80c\xD2 \xA9\xE0\x14a\x04\xC7W`\0\x80\xFD[\x80c\xAD\xD6\xCED\x14a\x04\xA3W\x80c\xC9\xFF-\x16\x14a\x04\xACW`\0\x80\xFD[\x80c\x98\xF3M\xF5\x11a\x01/W\x80c\x9C\x166\x0F\x11a\x01\x14W\x80c\x9C\x166\x0F\x14a\x04qW\x80c\xA2\xAF\r\x1F\x14a\x04\x91W\x80c\xA9\xDD\xE7\xD9\x14a\x04\x9AW`\0\x80\xFD[\x80c\x98\xF3M\xF5\x14a\x04IW\x80c\x9BK\xB4\x8F\x14a\x04QW`\0\x80\xFD[\x80c|\xF4\x8B@\x14a\x04\x0EW\x80c\x81\x14:Y\x14a\x04.W\x80c\x93^8)\x14a\x047W\x80c\x93\x99\x1A\xF3\x14a\x04@W`\0\x80\xFD[\x80c2d\x95\xFB\x11a\x02\x0EW\x80cU\xD6+}\x11a\x01\xC2W\x80cd\xFBu\x80\x11a\x01\xA7W\x80cd\xFBu\x80\x14a\x03\xDDW\x80cg\xCE\xF4F\x14a\x03\xE5W\x80ch\xEA*C\x14a\x03\xEEW`\0\x80\xFD[\x80cU\xD6+}\x14a\x03\xB4W\x80c]EF\xA0\x14a\x03\xD4W`\0\x80\xFD[\x80cB\x0E\x17(\x11a\x01\xF3W\x80cB\x0E\x17(\x14a\x03vW\x80cB\xC8\x02\x95\x14a\x03\x7FW\x80cR\xF89\x8E\x14a\x03\x9FW`\0\x80\xFD[\x80c2d\x95\xFB\x14a\x03eW\x80c8\xC2qY\x14a\x03mW`\0\x80\xFD[\x80c\x10,\x9A\xA4\x11a\x02eW\x80c'kez\x11a\x02JW\x80c'kez\x14a\x03JW\x80c-\xDE6\xF5\x14a\x03SW\x80c.\xF2\xD5^\x14a\x03\\W`\0\x80\xFD[\x80c\x10,\x9A\xA4\x14a\x03!W\x80c$\x1E-~\x14a\x03*W`\0\x80\xFD[\x80c\x08\xCB\x82-\x14a\x02\x97W\x80c\t\x08\x94\xDC\x14a\x02\xE1W\x80c\n\xC7{\xB5\x14a\x03\x01W\x80c\x0B{\x1B0\x14a\x03\x18W[`\0\x80\xFD[`\x1DTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`%Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`)T\x81V[`@Q\x90\x81R` \x01a\x02\xD8V[a\x03\n`-T\x81V[a\x03\n`/T\x81V[`\x1CTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x19T\x81V[a\x03\n`&T\x81V[a\x03\n`\x15T\x81V[a\x03\na\x05VV[a\x03\n`0T\x81V[a\x03\n`.T\x81V[`\"Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\xA7a\x08\xECV[`@Qa\x02\xD8\x91\x90a\x12\xBDV[`!Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x1BT\x81V[a\x03\xA7a\tzV[a\x03\n`1T\x81V[`\x18Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[` Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`(T\x81V[a\x03\n`,T\x81V[a\x03\n`\x12T\x81V[a\x03\na\t\x87V[`\x0FTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x17Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x11T\x81V[a\x03\n`'T\x81V[a\x03\n`\x14T\x81V[a\x03\n`+T\x81V[a\x03\n`\x1ET\x81V[a\x03\n`\x10T\x81V[a\x03\n`*T\x81V[`\x0ETa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x1FTa\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x03\n`\x13T\x81V[`\x16Ta\x02\xB7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x0CTa\x05F\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xD8V[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x05\xAA\x90`\r\x90`\x04\x01a\x13#V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x06\x03WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x06\0\x91\x81\x01\x90a\x149V[`\x01[a\x085W`@Q\x7FI\xC4\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cI\xC4\xFA\xC8\x90a\x06X\x90`\r\x90`\x04\x01a\x13#V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x06\xB6WP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x06\xB3\x91\x90\x81\x01\x90a\x15\x13V[`\x01[a\x08,W`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x07\x0B\x90`\r\x90`\x04\x01a\x13#V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x07dWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x07a\x91\x81\x01\x90a\x149V[`\x01[\x15a\x08'W`@Q\x7Fi\0\xA3\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90Ra\x08!\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\x1C\x91\x90\x81\x01\x90a\x15\x13V[a\x10\x0BV[\x91PP\x90V[a\x08:V[a\x08!\x81a\x10\x0BV[\x91\x90PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`Z`$\x82\x01R\x7Fl1StartingBlockTag must be a byt`D\x82\x01R\x7Fes32, string or uint256 or canno`d\x82\x01R\x7Ft fetch l1StartingBlockTag\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[`#\x80Ta\x08\xF9\x90a\x12\xD0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t%\x90a\x12\xD0V[\x80\x15a\trW\x80`\x1F\x10a\tGWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\trV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tUW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`$\x80Ta\x08\xF9\x90a\x12\xD0V[`\0\x80`\x1AT\x12\x15a\x0CvW`\0a\t\x9Da\x05VV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x91\x92P`\0\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xB9W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Fbash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\0\x81Q\x81\x10a\n\x19Wa\n\x19a\x15\\V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7F-c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\x01\x81Q\x81\x10a\nmWa\nma\x15\\V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Q\x7F\xB1\x1A\x19\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xB1\x1A\x19\xE8\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0B'\x91\x90\x81\x01\x90a\x15\x13V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7Fjq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q` \x01a\x0Bn\x92\x91\x90a\x15\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x02\x81Q\x81\x10a\x0B\x90Wa\x0B\x90a\x15\\V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Q\x7F\x89\x16\x04g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x0B\xEE\x90\x85\x90`\x04\x01a\x166V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0CS\x91\x90\x81\x01\x90a\x15\x13V[\x90Pa\x0Cn\x81`@Q\x80` \x01`@R\x80`\0\x81RPa\x0ECV[\x93PPPP\x90V[P`\x1AT\x90V[a\r\x0E\x82\x82`@Q`$\x01a\x0C\x93\x92\x91\x90a\x16\xB6V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FK\\Bw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x120V[PPV[a\r\xA1\x81`@Q`$\x01a\r&\x91\x90a\x12\xBDV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x120V[PV[`@Q\x7F\x1E\x19\xE6W\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x1E\x19\xE6W\x90a\r\xF9\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E<\x91\x90a\x16\xE4V[\x93\x92PPPV[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x0E\x98\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E<\x91\x90a\x149V[`@Q\x7F{\x04\x8C\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c{\x04\x8C\xCD\x90a\x0E\x98\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[`@Q\x7FI\xC4\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cI\xC4\xFA\xC8\x90a\x0F\x85\x90\x86\x90\x86\x90`\x04\x01a\x16\xB6V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0E<\x91\x90\x81\x01\x90a\x15\x13V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R`\0\x91\x82\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10&W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Fbash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\0\x81Q\x81\x10a\x10\x86Wa\x10\x86a\x15\\V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7F-c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81`\x01\x81Q\x81\x10a\x10\xDAWa\x10\xDAa\x15\\V[` \x02` \x01\x01\x81\x90RP\x82`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01\x7Fjq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q` \x01a\x11-\x92\x91\x90a\x17\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x02\x81Q\x81\x10a\x11OWa\x11Oa\x15\\V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Q\x7F\x89\x16\x04g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x11\xAD\x90\x85\x90`\x04\x01a\x166V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x12\x12\x91\x90\x81\x01\x90a\x15\x13V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x12(\x91\x90a\x149V[\x94\x93PPPPV[a\r\xA1\x81a\x0F\xEAV[a\x12Aa\x17\xC5V[V[`\0[\x83\x81\x10\x15a\x12^W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12FV[\x83\x81\x11\x15a\x12mW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x12\x8B\x81` \x86\x01` \x86\x01a\x12CV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0E<` \x83\x01\x84a\x12sV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x12\xE4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\x1DW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`@\x81R`\0\x80\x83T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x13CW`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03a\x13{W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86R`\"`\x04R`$\x86\xFD[`@\x88\x01\x84\x90R``\x88\x01\x82\x80\x15a\x13\x9AW`\x01\x81\x14a\x13\xCEWa\x13\xF9V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x87\x16\x82R\x85\x15\x15`\x05\x1B\x82\x01\x97Pa\x13\xF9V[`\0\x8B\x81R` \x90 `\0[\x87\x81\x10\x15a\x13\xF3W\x81T\x84\x82\x01R\x90\x86\x01\x90\x84\x01a\x13\xDAV[\x83\x01\x98PP[PP\x87\x86\x03\x90\x88\x01RPP`\x14\x83RPP\x7F$.l1StartingBlockTag\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@\x81\x01a\x12(V[`\0` \x82\x84\x03\x12\x15a\x14KW`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x14\x9CWa\x14\x9Ca\x14RV[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x14\xE2Wa\x14\xE2a\x14RV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x14\xFBW`\0\x80\xFD[a\x15\t\x86` \x83\x01\x87a\x12CV[PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x15%W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15<W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x15MW`\0\x80\xFD[a\x12(\x84\x82Q` \x84\x01a\x14\x81V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7Fcast block \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x15\xC3\x81`\x0B\x85\x01` \x88\x01a\x12CV[\x7F --json | \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x91\x84\x01\x91\x82\x01R\x83Qa\x16\0\x81`\x15\x84\x01` \x88\x01a\x12CV[\x7F .timestamp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x15\x92\x90\x91\x01\x91\x82\x01R` \x01\x94\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x16\xA9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra\x16\x97\x85\x83Qa\x12sV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x16]V[P\x92\x97\x96PPPPPPPV[`@\x81R`\0a\x16\xC9`@\x83\x01\x85a\x12sV[\x82\x81\x03` \x84\x01Ra\x16\xDB\x81\x85a\x12sV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x16\xF6W`\0\x80\xFD[\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E<W`\0\x80\xFD[\x7Fcast block \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x17R\x81`\x0B\x85\x01` \x88\x01a\x12CV[\x7F --json | \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x91\x84\x01\x91\x82\x01R\x83Qa\x17\x8F\x81`\x15\x84\x01` \x88\x01a\x12CV[\x7F -r .hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x15\x92\x90\x91\x01\x91\x82\x01R`\x1E\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`Q`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static DEPLOYCONFIG_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DeployConfig<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DeployConfig<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DeployConfig<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DeployConfig<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DeployConfig<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DeployConfig))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployConfig<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPLOYCONFIG_ABI.clone(),
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
                DEPLOYCONFIG_ABI.clone(),
                DEPLOYCONFIG_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `baseFeeVaultRecipient` (0x7cf48b40) function
        pub fn base_fee_vault_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([124, 244, 139, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchInboxAddress` (0x9c16360f) function
        pub fn batch_inbox_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([156, 22, 54, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchSenderAddress` (0x68ea2a43) function
        pub fn batch_sender_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([104, 234, 42, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `channelTimeout` (0x2ef2d55e) function
        pub fn channel_timeout(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 242, 213, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip1559Denominator` (0xd220a9e0) function
        pub fn eip_1559_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([210, 32, 169, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip1559Elasticity` (0xc9ff2d16) function
        pub fn eip_1559_elasticity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 255, 45, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `faultGameAbsolutePrestate` (0x935e3829) function
        pub fn fault_game_absolute_prestate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 94, 56, 41], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `faultGameMaxDepth` (0x0b7b1b30) function
        pub fn fault_game_max_depth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 123, 27, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `faultGameMaxDuration` (0x420e1728) function
        pub fn fault_game_max_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 14, 23, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalSystemOwner` (0xd2354f20) function
        pub fn final_system_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 53, 79, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizationPeriodSeconds` (0xce5db8d6) function
        pub fn finalization_period_seconds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 93, 184, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gasPriceOracleOverhead` (0x81143a59) function
        pub fn gas_price_oracle_overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([129, 20, 58, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gasPriceOracleScalar` (0x0ac77bb5) function
        pub fn gas_price_oracle_scalar(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 199, 123, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `governanceTokenName` (0x52f8398e) function
        pub fn governance_token_name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([82, 248, 57, 142], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `governanceTokenOwner` (0x090894dc) function
        pub fn governance_token_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([9, 8, 148, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `governanceTokenSymbol` (0x64fb7580) function
        pub fn governance_token_symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([100, 251, 117, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1ChainID` (0xcf7bfef5) function
        pub fn l_1_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 123, 254, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1FeeVaultRecipient` (0x55d62b7d) function
        pub fn l_1_fee_vault_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([85, 214, 43, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1StartingBlockTag` (0x326495fb) function
        pub fn l_1_starting_block_tag(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([50, 100, 149, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2BlockTime` (0x93991af3) function
        pub fn l_2_block_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 153, 26, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2ChainID` (0xa2af0d1f) function
        pub fn l_2_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([162, 175, 13, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2GenesisBlockBaseFeePerGas` (0xa9dde7d9) function
        pub fn l_2_genesis_block_base_fee_per_gas(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([169, 221, 231, 217], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2GenesisBlockGasLimit` (0x2dde36f5) function
        pub fn l_2_genesis_block_gas_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 222, 54, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2OutputOracleChallenger` (0x08cb822d) function
        pub fn l_2_output_oracle_challenger(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 203, 130, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2OutputOracleProposer` (0x241e2d7e) function
        pub fn l_2_output_oracle_proposer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([36, 30, 45, 126], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2OutputOracleStartingBlockNumber` (0x5d4546a0) function
        pub fn l_2_output_oracle_starting_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([93, 69, 70, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2OutputOracleStartingTimestamp` (0x98f34df5) function
        pub fn l_2_output_oracle_starting_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 243, 77, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2OutputOracleSubmissionInterval` (0x276b657a) function
        pub fn l_2_output_oracle_submission_interval(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 107, 101, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxSequencerDrift` (0xe31ea108) function
        pub fn max_sequencer_drift(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 30, 161, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `p2pSequencerAddress` (0xe7d6cd42) function
        pub fn p_2p_sequencer_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 214, 205, 66], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `portalGuardian` (0x9b4bb48f) function
        pub fn portal_guardian(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([155, 75, 180, 143], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyAdminOwner` (0xdad544e0) function
        pub fn proxy_admin_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([218, 213, 68, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recommendedProtocolVersion` (0x67cef446) function
        pub fn recommended_protocol_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 206, 244, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requiredProtocolVersion` (0x38c27159) function
        pub fn required_protocol_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([56, 194, 113, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sequencerFeeVaultRecipient` (0x42c80295) function
        pub fn sequencer_fee_vault_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([66, 200, 2, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sequencerWindowSize` (0xadd6ce44) function
        pub fn sequencer_window_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 214, 206, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `systemConfigStartBlock` (0x102c9aa4) function
        pub fn system_config_start_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([16, 44, 154, 164], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DeployConfig<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `baseFeeVaultRecipient` function with signature `baseFeeVaultRecipient()` and selector `0x7cf48b40`
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
    #[ethcall(name = "baseFeeVaultRecipient", abi = "baseFeeVaultRecipient()")]
    pub struct BaseFeeVaultRecipientCall;
    ///Container type for all input parameters for the `batchInboxAddress` function with signature `batchInboxAddress()` and selector `0x9c16360f`
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
    #[ethcall(name = "batchInboxAddress", abi = "batchInboxAddress()")]
    pub struct BatchInboxAddressCall;
    ///Container type for all input parameters for the `batchSenderAddress` function with signature `batchSenderAddress()` and selector `0x68ea2a43`
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
    #[ethcall(name = "batchSenderAddress", abi = "batchSenderAddress()")]
    pub struct BatchSenderAddressCall;
    ///Container type for all input parameters for the `channelTimeout` function with signature `channelTimeout()` and selector `0x2ef2d55e`
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
    #[ethcall(name = "channelTimeout", abi = "channelTimeout()")]
    pub struct ChannelTimeoutCall;
    ///Container type for all input parameters for the `eip1559Denominator` function with signature `eip1559Denominator()` and selector `0xd220a9e0`
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
    #[ethcall(name = "eip1559Denominator", abi = "eip1559Denominator()")]
    pub struct Eip1559DenominatorCall;
    ///Container type for all input parameters for the `eip1559Elasticity` function with signature `eip1559Elasticity()` and selector `0xc9ff2d16`
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
    #[ethcall(name = "eip1559Elasticity", abi = "eip1559Elasticity()")]
    pub struct Eip1559ElasticityCall;
    ///Container type for all input parameters for the `faultGameAbsolutePrestate` function with signature `faultGameAbsolutePrestate()` and selector `0x935e3829`
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
    #[ethcall(name = "faultGameAbsolutePrestate", abi = "faultGameAbsolutePrestate()")]
    pub struct FaultGameAbsolutePrestateCall;
    ///Container type for all input parameters for the `faultGameMaxDepth` function with signature `faultGameMaxDepth()` and selector `0x0b7b1b30`
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
    #[ethcall(name = "faultGameMaxDepth", abi = "faultGameMaxDepth()")]
    pub struct FaultGameMaxDepthCall;
    ///Container type for all input parameters for the `faultGameMaxDuration` function with signature `faultGameMaxDuration()` and selector `0x420e1728`
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
    #[ethcall(name = "faultGameMaxDuration", abi = "faultGameMaxDuration()")]
    pub struct FaultGameMaxDurationCall;
    ///Container type for all input parameters for the `finalSystemOwner` function with signature `finalSystemOwner()` and selector `0xd2354f20`
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
    #[ethcall(name = "finalSystemOwner", abi = "finalSystemOwner()")]
    pub struct FinalSystemOwnerCall;
    ///Container type for all input parameters for the `finalizationPeriodSeconds` function with signature `finalizationPeriodSeconds()` and selector `0xce5db8d6`
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
    #[ethcall(name = "finalizationPeriodSeconds", abi = "finalizationPeriodSeconds()")]
    pub struct FinalizationPeriodSecondsCall;
    ///Container type for all input parameters for the `gasPriceOracleOverhead` function with signature `gasPriceOracleOverhead()` and selector `0x81143a59`
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
    #[ethcall(name = "gasPriceOracleOverhead", abi = "gasPriceOracleOverhead()")]
    pub struct GasPriceOracleOverheadCall;
    ///Container type for all input parameters for the `gasPriceOracleScalar` function with signature `gasPriceOracleScalar()` and selector `0x0ac77bb5`
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
    #[ethcall(name = "gasPriceOracleScalar", abi = "gasPriceOracleScalar()")]
    pub struct GasPriceOracleScalarCall;
    ///Container type for all input parameters for the `governanceTokenName` function with signature `governanceTokenName()` and selector `0x52f8398e`
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
    #[ethcall(name = "governanceTokenName", abi = "governanceTokenName()")]
    pub struct GovernanceTokenNameCall;
    ///Container type for all input parameters for the `governanceTokenOwner` function with signature `governanceTokenOwner()` and selector `0x090894dc`
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
    #[ethcall(name = "governanceTokenOwner", abi = "governanceTokenOwner()")]
    pub struct GovernanceTokenOwnerCall;
    ///Container type for all input parameters for the `governanceTokenSymbol` function with signature `governanceTokenSymbol()` and selector `0x64fb7580`
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
    #[ethcall(name = "governanceTokenSymbol", abi = "governanceTokenSymbol()")]
    pub struct GovernanceTokenSymbolCall;
    ///Container type for all input parameters for the `l1ChainID` function with signature `l1ChainID()` and selector `0xcf7bfef5`
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
    #[ethcall(name = "l1ChainID", abi = "l1ChainID()")]
    pub struct L1ChainIDCall;
    ///Container type for all input parameters for the `l1FeeVaultRecipient` function with signature `l1FeeVaultRecipient()` and selector `0x55d62b7d`
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
    #[ethcall(name = "l1FeeVaultRecipient", abi = "l1FeeVaultRecipient()")]
    pub struct L1FeeVaultRecipientCall;
    ///Container type for all input parameters for the `l1StartingBlockTag` function with signature `l1StartingBlockTag()` and selector `0x326495fb`
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
    #[ethcall(name = "l1StartingBlockTag", abi = "l1StartingBlockTag()")]
    pub struct L1StartingBlockTagCall;
    ///Container type for all input parameters for the `l2BlockTime` function with signature `l2BlockTime()` and selector `0x93991af3`
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
    #[ethcall(name = "l2BlockTime", abi = "l2BlockTime()")]
    pub struct L2BlockTimeCall;
    ///Container type for all input parameters for the `l2ChainID` function with signature `l2ChainID()` and selector `0xa2af0d1f`
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
    #[ethcall(name = "l2ChainID", abi = "l2ChainID()")]
    pub struct L2ChainIDCall;
    ///Container type for all input parameters for the `l2GenesisBlockBaseFeePerGas` function with signature `l2GenesisBlockBaseFeePerGas()` and selector `0xa9dde7d9`
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
        name = "l2GenesisBlockBaseFeePerGas",
        abi = "l2GenesisBlockBaseFeePerGas()"
    )]
    pub struct L2GenesisBlockBaseFeePerGasCall;
    ///Container type for all input parameters for the `l2GenesisBlockGasLimit` function with signature `l2GenesisBlockGasLimit()` and selector `0x2dde36f5`
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
    #[ethcall(name = "l2GenesisBlockGasLimit", abi = "l2GenesisBlockGasLimit()")]
    pub struct L2GenesisBlockGasLimitCall;
    ///Container type for all input parameters for the `l2OutputOracleChallenger` function with signature `l2OutputOracleChallenger()` and selector `0x08cb822d`
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
    #[ethcall(name = "l2OutputOracleChallenger", abi = "l2OutputOracleChallenger()")]
    pub struct L2OutputOracleChallengerCall;
    ///Container type for all input parameters for the `l2OutputOracleProposer` function with signature `l2OutputOracleProposer()` and selector `0x241e2d7e`
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
    #[ethcall(name = "l2OutputOracleProposer", abi = "l2OutputOracleProposer()")]
    pub struct L2OutputOracleProposerCall;
    ///Container type for all input parameters for the `l2OutputOracleStartingBlockNumber` function with signature `l2OutputOracleStartingBlockNumber()` and selector `0x5d4546a0`
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
        name = "l2OutputOracleStartingBlockNumber",
        abi = "l2OutputOracleStartingBlockNumber()"
    )]
    pub struct L2OutputOracleStartingBlockNumberCall;
    ///Container type for all input parameters for the `l2OutputOracleStartingTimestamp` function with signature `l2OutputOracleStartingTimestamp()` and selector `0x98f34df5`
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
        name = "l2OutputOracleStartingTimestamp",
        abi = "l2OutputOracleStartingTimestamp()"
    )]
    pub struct L2OutputOracleStartingTimestampCall;
    ///Container type for all input parameters for the `l2OutputOracleSubmissionInterval` function with signature `l2OutputOracleSubmissionInterval()` and selector `0x276b657a`
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
        name = "l2OutputOracleSubmissionInterval",
        abi = "l2OutputOracleSubmissionInterval()"
    )]
    pub struct L2OutputOracleSubmissionIntervalCall;
    ///Container type for all input parameters for the `maxSequencerDrift` function with signature `maxSequencerDrift()` and selector `0xe31ea108`
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
    #[ethcall(name = "maxSequencerDrift", abi = "maxSequencerDrift()")]
    pub struct MaxSequencerDriftCall;
    ///Container type for all input parameters for the `p2pSequencerAddress` function with signature `p2pSequencerAddress()` and selector `0xe7d6cd42`
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
    #[ethcall(name = "p2pSequencerAddress", abi = "p2pSequencerAddress()")]
    pub struct P2PsequencerAddressCall;
    ///Container type for all input parameters for the `portalGuardian` function with signature `portalGuardian()` and selector `0x9b4bb48f`
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
    #[ethcall(name = "portalGuardian", abi = "portalGuardian()")]
    pub struct PortalGuardianCall;
    ///Container type for all input parameters for the `proxyAdminOwner` function with signature `proxyAdminOwner()` and selector `0xdad544e0`
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
    #[ethcall(name = "proxyAdminOwner", abi = "proxyAdminOwner()")]
    pub struct ProxyAdminOwnerCall;
    ///Container type for all input parameters for the `recommendedProtocolVersion` function with signature `recommendedProtocolVersion()` and selector `0x67cef446`
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
    #[ethcall(name = "recommendedProtocolVersion", abi = "recommendedProtocolVersion()")]
    pub struct RecommendedProtocolVersionCall;
    ///Container type for all input parameters for the `requiredProtocolVersion` function with signature `requiredProtocolVersion()` and selector `0x38c27159`
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
    #[ethcall(name = "requiredProtocolVersion", abi = "requiredProtocolVersion()")]
    pub struct RequiredProtocolVersionCall;
    ///Container type for all input parameters for the `sequencerFeeVaultRecipient` function with signature `sequencerFeeVaultRecipient()` and selector `0x42c80295`
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
    #[ethcall(name = "sequencerFeeVaultRecipient", abi = "sequencerFeeVaultRecipient()")]
    pub struct SequencerFeeVaultRecipientCall;
    ///Container type for all input parameters for the `sequencerWindowSize` function with signature `sequencerWindowSize()` and selector `0xadd6ce44`
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
    #[ethcall(name = "sequencerWindowSize", abi = "sequencerWindowSize()")]
    pub struct SequencerWindowSizeCall;
    ///Container type for all input parameters for the `systemConfigStartBlock` function with signature `systemConfigStartBlock()` and selector `0x102c9aa4`
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
    #[ethcall(name = "systemConfigStartBlock", abi = "systemConfigStartBlock()")]
    pub struct SystemConfigStartBlockCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DeployConfigCalls {
        IsScript(IsScriptCall),
        BaseFeeVaultRecipient(BaseFeeVaultRecipientCall),
        BatchInboxAddress(BatchInboxAddressCall),
        BatchSenderAddress(BatchSenderAddressCall),
        ChannelTimeout(ChannelTimeoutCall),
        Eip1559Denominator(Eip1559DenominatorCall),
        Eip1559Elasticity(Eip1559ElasticityCall),
        FaultGameAbsolutePrestate(FaultGameAbsolutePrestateCall),
        FaultGameMaxDepth(FaultGameMaxDepthCall),
        FaultGameMaxDuration(FaultGameMaxDurationCall),
        FinalSystemOwner(FinalSystemOwnerCall),
        FinalizationPeriodSeconds(FinalizationPeriodSecondsCall),
        GasPriceOracleOverhead(GasPriceOracleOverheadCall),
        GasPriceOracleScalar(GasPriceOracleScalarCall),
        GovernanceTokenName(GovernanceTokenNameCall),
        GovernanceTokenOwner(GovernanceTokenOwnerCall),
        GovernanceTokenSymbol(GovernanceTokenSymbolCall),
        L1ChainID(L1ChainIDCall),
        L1FeeVaultRecipient(L1FeeVaultRecipientCall),
        L1StartingBlockTag(L1StartingBlockTagCall),
        L2BlockTime(L2BlockTimeCall),
        L2ChainID(L2ChainIDCall),
        L2GenesisBlockBaseFeePerGas(L2GenesisBlockBaseFeePerGasCall),
        L2GenesisBlockGasLimit(L2GenesisBlockGasLimitCall),
        L2OutputOracleChallenger(L2OutputOracleChallengerCall),
        L2OutputOracleProposer(L2OutputOracleProposerCall),
        L2OutputOracleStartingBlockNumber(L2OutputOracleStartingBlockNumberCall),
        L2OutputOracleStartingTimestamp(L2OutputOracleStartingTimestampCall),
        L2OutputOracleSubmissionInterval(L2OutputOracleSubmissionIntervalCall),
        MaxSequencerDrift(MaxSequencerDriftCall),
        P2PsequencerAddress(P2PsequencerAddressCall),
        PortalGuardian(PortalGuardianCall),
        ProxyAdminOwner(ProxyAdminOwnerCall),
        RecommendedProtocolVersion(RecommendedProtocolVersionCall),
        RequiredProtocolVersion(RequiredProtocolVersionCall),
        SequencerFeeVaultRecipient(SequencerFeeVaultRecipientCall),
        SequencerWindowSize(SequencerWindowSizeCall),
        SystemConfigStartBlock(SystemConfigStartBlockCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployConfigCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded) = <BaseFeeVaultRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseFeeVaultRecipient(decoded));
            }
            if let Ok(decoded) = <BatchInboxAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchInboxAddress(decoded));
            }
            if let Ok(decoded) = <BatchSenderAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchSenderAddress(decoded));
            }
            if let Ok(decoded) = <ChannelTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChannelTimeout(decoded));
            }
            if let Ok(decoded) = <Eip1559DenominatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eip1559Denominator(decoded));
            }
            if let Ok(decoded) = <Eip1559ElasticityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eip1559Elasticity(decoded));
            }
            if let Ok(decoded) = <FaultGameAbsolutePrestateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FaultGameAbsolutePrestate(decoded));
            }
            if let Ok(decoded) = <FaultGameMaxDepthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FaultGameMaxDepth(decoded));
            }
            if let Ok(decoded) = <FaultGameMaxDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FaultGameMaxDuration(decoded));
            }
            if let Ok(decoded) = <FinalSystemOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FinalSystemOwner(decoded));
            }
            if let Ok(decoded) = <FinalizationPeriodSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FinalizationPeriodSeconds(decoded));
            }
            if let Ok(decoded) = <GasPriceOracleOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GasPriceOracleOverhead(decoded));
            }
            if let Ok(decoded) = <GasPriceOracleScalarCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GasPriceOracleScalar(decoded));
            }
            if let Ok(decoded) = <GovernanceTokenNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GovernanceTokenName(decoded));
            }
            if let Ok(decoded) = <GovernanceTokenOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GovernanceTokenOwner(decoded));
            }
            if let Ok(decoded) = <GovernanceTokenSymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GovernanceTokenSymbol(decoded));
            }
            if let Ok(decoded) = <L1ChainIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1ChainID(decoded));
            }
            if let Ok(decoded) = <L1FeeVaultRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1FeeVaultRecipient(decoded));
            }
            if let Ok(decoded) = <L1StartingBlockTagCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1StartingBlockTag(decoded));
            }
            if let Ok(decoded) = <L2BlockTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2BlockTime(decoded));
            }
            if let Ok(decoded) = <L2ChainIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2ChainID(decoded));
            }
            if let Ok(decoded) = <L2GenesisBlockBaseFeePerGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2GenesisBlockBaseFeePerGas(decoded));
            }
            if let Ok(decoded) = <L2GenesisBlockGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2GenesisBlockGasLimit(decoded));
            }
            if let Ok(decoded) = <L2OutputOracleChallengerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2OutputOracleChallenger(decoded));
            }
            if let Ok(decoded) = <L2OutputOracleProposerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2OutputOracleProposer(decoded));
            }
            if let Ok(decoded) = <L2OutputOracleStartingBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2OutputOracleStartingBlockNumber(decoded));
            }
            if let Ok(decoded) = <L2OutputOracleStartingTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2OutputOracleStartingTimestamp(decoded));
            }
            if let Ok(decoded) = <L2OutputOracleSubmissionIntervalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2OutputOracleSubmissionInterval(decoded));
            }
            if let Ok(decoded) = <MaxSequencerDriftCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxSequencerDrift(decoded));
            }
            if let Ok(decoded) = <P2PsequencerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::P2PsequencerAddress(decoded));
            }
            if let Ok(decoded) = <PortalGuardianCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PortalGuardian(decoded));
            }
            if let Ok(decoded) = <ProxyAdminOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxyAdminOwner(decoded));
            }
            if let Ok(decoded) = <RecommendedProtocolVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecommendedProtocolVersion(decoded));
            }
            if let Ok(decoded) = <RequiredProtocolVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequiredProtocolVersion(decoded));
            }
            if let Ok(decoded) = <SequencerFeeVaultRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SequencerFeeVaultRecipient(decoded));
            }
            if let Ok(decoded) = <SequencerWindowSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SequencerWindowSize(decoded));
            }
            if let Ok(decoded) = <SystemConfigStartBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SystemConfigStartBlock(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployConfigCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseFeeVaultRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchInboxAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchSenderAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChannelTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip1559Denominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip1559Elasticity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FaultGameAbsolutePrestate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FaultGameMaxDepth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FaultGameMaxDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalSystemOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizationPeriodSeconds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GasPriceOracleOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GasPriceOracleScalar(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GovernanceTokenName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GovernanceTokenOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GovernanceTokenSymbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1ChainID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1FeeVaultRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1StartingBlockTag(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2BlockTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2ChainID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2GenesisBlockBaseFeePerGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2GenesisBlockGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2OutputOracleChallenger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2OutputOracleProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2OutputOracleStartingBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2OutputOracleStartingTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2OutputOracleSubmissionInterval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxSequencerDrift(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::P2PsequencerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PortalGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyAdminOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecommendedProtocolVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequiredProtocolVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SequencerFeeVaultRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SequencerWindowSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SystemConfigStartBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DeployConfigCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseFeeVaultRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchInboxAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchSenderAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChannelTimeout(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip1559Denominator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Eip1559Elasticity(element) => ::core::fmt::Display::fmt(element, f),
                Self::FaultGameAbsolutePrestate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FaultGameMaxDepth(element) => ::core::fmt::Display::fmt(element, f),
                Self::FaultGameMaxDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FinalSystemOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizationPeriodSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GasPriceOracleOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GasPriceOracleScalar(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GovernanceTokenName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GovernanceTokenOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GovernanceTokenSymbol(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L1ChainID(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1FeeVaultRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L1StartingBlockTag(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2BlockTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2ChainID(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2GenesisBlockBaseFeePerGas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2GenesisBlockGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2OutputOracleChallenger(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2OutputOracleProposer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2OutputOracleStartingBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2OutputOracleStartingTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2OutputOracleSubmissionInterval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxSequencerDrift(element) => ::core::fmt::Display::fmt(element, f),
                Self::P2PsequencerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PortalGuardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyAdminOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecommendedProtocolVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequiredProtocolVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerFeeVaultRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerWindowSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SystemConfigStartBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for DeployConfigCalls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<BaseFeeVaultRecipientCall> for DeployConfigCalls {
        fn from(value: BaseFeeVaultRecipientCall) -> Self {
            Self::BaseFeeVaultRecipient(value)
        }
    }
    impl ::core::convert::From<BatchInboxAddressCall> for DeployConfigCalls {
        fn from(value: BatchInboxAddressCall) -> Self {
            Self::BatchInboxAddress(value)
        }
    }
    impl ::core::convert::From<BatchSenderAddressCall> for DeployConfigCalls {
        fn from(value: BatchSenderAddressCall) -> Self {
            Self::BatchSenderAddress(value)
        }
    }
    impl ::core::convert::From<ChannelTimeoutCall> for DeployConfigCalls {
        fn from(value: ChannelTimeoutCall) -> Self {
            Self::ChannelTimeout(value)
        }
    }
    impl ::core::convert::From<Eip1559DenominatorCall> for DeployConfigCalls {
        fn from(value: Eip1559DenominatorCall) -> Self {
            Self::Eip1559Denominator(value)
        }
    }
    impl ::core::convert::From<Eip1559ElasticityCall> for DeployConfigCalls {
        fn from(value: Eip1559ElasticityCall) -> Self {
            Self::Eip1559Elasticity(value)
        }
    }
    impl ::core::convert::From<FaultGameAbsolutePrestateCall> for DeployConfigCalls {
        fn from(value: FaultGameAbsolutePrestateCall) -> Self {
            Self::FaultGameAbsolutePrestate(value)
        }
    }
    impl ::core::convert::From<FaultGameMaxDepthCall> for DeployConfigCalls {
        fn from(value: FaultGameMaxDepthCall) -> Self {
            Self::FaultGameMaxDepth(value)
        }
    }
    impl ::core::convert::From<FaultGameMaxDurationCall> for DeployConfigCalls {
        fn from(value: FaultGameMaxDurationCall) -> Self {
            Self::FaultGameMaxDuration(value)
        }
    }
    impl ::core::convert::From<FinalSystemOwnerCall> for DeployConfigCalls {
        fn from(value: FinalSystemOwnerCall) -> Self {
            Self::FinalSystemOwner(value)
        }
    }
    impl ::core::convert::From<FinalizationPeriodSecondsCall> for DeployConfigCalls {
        fn from(value: FinalizationPeriodSecondsCall) -> Self {
            Self::FinalizationPeriodSeconds(value)
        }
    }
    impl ::core::convert::From<GasPriceOracleOverheadCall> for DeployConfigCalls {
        fn from(value: GasPriceOracleOverheadCall) -> Self {
            Self::GasPriceOracleOverhead(value)
        }
    }
    impl ::core::convert::From<GasPriceOracleScalarCall> for DeployConfigCalls {
        fn from(value: GasPriceOracleScalarCall) -> Self {
            Self::GasPriceOracleScalar(value)
        }
    }
    impl ::core::convert::From<GovernanceTokenNameCall> for DeployConfigCalls {
        fn from(value: GovernanceTokenNameCall) -> Self {
            Self::GovernanceTokenName(value)
        }
    }
    impl ::core::convert::From<GovernanceTokenOwnerCall> for DeployConfigCalls {
        fn from(value: GovernanceTokenOwnerCall) -> Self {
            Self::GovernanceTokenOwner(value)
        }
    }
    impl ::core::convert::From<GovernanceTokenSymbolCall> for DeployConfigCalls {
        fn from(value: GovernanceTokenSymbolCall) -> Self {
            Self::GovernanceTokenSymbol(value)
        }
    }
    impl ::core::convert::From<L1ChainIDCall> for DeployConfigCalls {
        fn from(value: L1ChainIDCall) -> Self {
            Self::L1ChainID(value)
        }
    }
    impl ::core::convert::From<L1FeeVaultRecipientCall> for DeployConfigCalls {
        fn from(value: L1FeeVaultRecipientCall) -> Self {
            Self::L1FeeVaultRecipient(value)
        }
    }
    impl ::core::convert::From<L1StartingBlockTagCall> for DeployConfigCalls {
        fn from(value: L1StartingBlockTagCall) -> Self {
            Self::L1StartingBlockTag(value)
        }
    }
    impl ::core::convert::From<L2BlockTimeCall> for DeployConfigCalls {
        fn from(value: L2BlockTimeCall) -> Self {
            Self::L2BlockTime(value)
        }
    }
    impl ::core::convert::From<L2ChainIDCall> for DeployConfigCalls {
        fn from(value: L2ChainIDCall) -> Self {
            Self::L2ChainID(value)
        }
    }
    impl ::core::convert::From<L2GenesisBlockBaseFeePerGasCall> for DeployConfigCalls {
        fn from(value: L2GenesisBlockBaseFeePerGasCall) -> Self {
            Self::L2GenesisBlockBaseFeePerGas(value)
        }
    }
    impl ::core::convert::From<L2GenesisBlockGasLimitCall> for DeployConfigCalls {
        fn from(value: L2GenesisBlockGasLimitCall) -> Self {
            Self::L2GenesisBlockGasLimit(value)
        }
    }
    impl ::core::convert::From<L2OutputOracleChallengerCall> for DeployConfigCalls {
        fn from(value: L2OutputOracleChallengerCall) -> Self {
            Self::L2OutputOracleChallenger(value)
        }
    }
    impl ::core::convert::From<L2OutputOracleProposerCall> for DeployConfigCalls {
        fn from(value: L2OutputOracleProposerCall) -> Self {
            Self::L2OutputOracleProposer(value)
        }
    }
    impl ::core::convert::From<L2OutputOracleStartingBlockNumberCall>
    for DeployConfigCalls {
        fn from(value: L2OutputOracleStartingBlockNumberCall) -> Self {
            Self::L2OutputOracleStartingBlockNumber(value)
        }
    }
    impl ::core::convert::From<L2OutputOracleStartingTimestampCall>
    for DeployConfigCalls {
        fn from(value: L2OutputOracleStartingTimestampCall) -> Self {
            Self::L2OutputOracleStartingTimestamp(value)
        }
    }
    impl ::core::convert::From<L2OutputOracleSubmissionIntervalCall>
    for DeployConfigCalls {
        fn from(value: L2OutputOracleSubmissionIntervalCall) -> Self {
            Self::L2OutputOracleSubmissionInterval(value)
        }
    }
    impl ::core::convert::From<MaxSequencerDriftCall> for DeployConfigCalls {
        fn from(value: MaxSequencerDriftCall) -> Self {
            Self::MaxSequencerDrift(value)
        }
    }
    impl ::core::convert::From<P2PsequencerAddressCall> for DeployConfigCalls {
        fn from(value: P2PsequencerAddressCall) -> Self {
            Self::P2PsequencerAddress(value)
        }
    }
    impl ::core::convert::From<PortalGuardianCall> for DeployConfigCalls {
        fn from(value: PortalGuardianCall) -> Self {
            Self::PortalGuardian(value)
        }
    }
    impl ::core::convert::From<ProxyAdminOwnerCall> for DeployConfigCalls {
        fn from(value: ProxyAdminOwnerCall) -> Self {
            Self::ProxyAdminOwner(value)
        }
    }
    impl ::core::convert::From<RecommendedProtocolVersionCall> for DeployConfigCalls {
        fn from(value: RecommendedProtocolVersionCall) -> Self {
            Self::RecommendedProtocolVersion(value)
        }
    }
    impl ::core::convert::From<RequiredProtocolVersionCall> for DeployConfigCalls {
        fn from(value: RequiredProtocolVersionCall) -> Self {
            Self::RequiredProtocolVersion(value)
        }
    }
    impl ::core::convert::From<SequencerFeeVaultRecipientCall> for DeployConfigCalls {
        fn from(value: SequencerFeeVaultRecipientCall) -> Self {
            Self::SequencerFeeVaultRecipient(value)
        }
    }
    impl ::core::convert::From<SequencerWindowSizeCall> for DeployConfigCalls {
        fn from(value: SequencerWindowSizeCall) -> Self {
            Self::SequencerWindowSize(value)
        }
    }
    impl ::core::convert::From<SystemConfigStartBlockCall> for DeployConfigCalls {
        fn from(value: SystemConfigStartBlockCall) -> Self {
            Self::SystemConfigStartBlock(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    pub struct IsScriptReturn(pub bool);
    ///Container type for all return fields from the `baseFeeVaultRecipient` function with signature `baseFeeVaultRecipient()` and selector `0x7cf48b40`
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
    pub struct BaseFeeVaultRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `batchInboxAddress` function with signature `batchInboxAddress()` and selector `0x9c16360f`
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
    pub struct BatchInboxAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `batchSenderAddress` function with signature `batchSenderAddress()` and selector `0x68ea2a43`
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
    pub struct BatchSenderAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `channelTimeout` function with signature `channelTimeout()` and selector `0x2ef2d55e`
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
    pub struct ChannelTimeoutReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `eip1559Denominator` function with signature `eip1559Denominator()` and selector `0xd220a9e0`
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
    pub struct Eip1559DenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `eip1559Elasticity` function with signature `eip1559Elasticity()` and selector `0xc9ff2d16`
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
    pub struct Eip1559ElasticityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `faultGameAbsolutePrestate` function with signature `faultGameAbsolutePrestate()` and selector `0x935e3829`
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
    pub struct FaultGameAbsolutePrestateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `faultGameMaxDepth` function with signature `faultGameMaxDepth()` and selector `0x0b7b1b30`
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
    pub struct FaultGameMaxDepthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `faultGameMaxDuration` function with signature `faultGameMaxDuration()` and selector `0x420e1728`
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
    pub struct FaultGameMaxDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `finalSystemOwner` function with signature `finalSystemOwner()` and selector `0xd2354f20`
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
    pub struct FinalSystemOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `finalizationPeriodSeconds` function with signature `finalizationPeriodSeconds()` and selector `0xce5db8d6`
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
    pub struct FinalizationPeriodSecondsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gasPriceOracleOverhead` function with signature `gasPriceOracleOverhead()` and selector `0x81143a59`
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
    pub struct GasPriceOracleOverheadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gasPriceOracleScalar` function with signature `gasPriceOracleScalar()` and selector `0x0ac77bb5`
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
    pub struct GasPriceOracleScalarReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `governanceTokenName` function with signature `governanceTokenName()` and selector `0x52f8398e`
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
    pub struct GovernanceTokenNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `governanceTokenOwner` function with signature `governanceTokenOwner()` and selector `0x090894dc`
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
    pub struct GovernanceTokenOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `governanceTokenSymbol` function with signature `governanceTokenSymbol()` and selector `0x64fb7580`
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
    pub struct GovernanceTokenSymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `l1ChainID` function with signature `l1ChainID()` and selector `0xcf7bfef5`
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
    pub struct L1ChainIDReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l1FeeVaultRecipient` function with signature `l1FeeVaultRecipient()` and selector `0x55d62b7d`
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
    pub struct L1FeeVaultRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `l1StartingBlockTag` function with signature `l1StartingBlockTag()` and selector `0x326495fb`
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
    pub struct L1StartingBlockTagReturn(pub [u8; 32]);
    ///Container type for all return fields from the `l2BlockTime` function with signature `l2BlockTime()` and selector `0x93991af3`
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
    pub struct L2BlockTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2ChainID` function with signature `l2ChainID()` and selector `0xa2af0d1f`
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
    pub struct L2ChainIDReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2GenesisBlockBaseFeePerGas` function with signature `l2GenesisBlockBaseFeePerGas()` and selector `0xa9dde7d9`
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
    pub struct L2GenesisBlockBaseFeePerGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2GenesisBlockGasLimit` function with signature `l2GenesisBlockGasLimit()` and selector `0x2dde36f5`
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
    pub struct L2GenesisBlockGasLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2OutputOracleChallenger` function with signature `l2OutputOracleChallenger()` and selector `0x08cb822d`
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
    pub struct L2OutputOracleChallengerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `l2OutputOracleProposer` function with signature `l2OutputOracleProposer()` and selector `0x241e2d7e`
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
    pub struct L2OutputOracleProposerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `l2OutputOracleStartingBlockNumber` function with signature `l2OutputOracleStartingBlockNumber()` and selector `0x5d4546a0`
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
    pub struct L2OutputOracleStartingBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2OutputOracleStartingTimestamp` function with signature `l2OutputOracleStartingTimestamp()` and selector `0x98f34df5`
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
    pub struct L2OutputOracleStartingTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2OutputOracleSubmissionInterval` function with signature `l2OutputOracleSubmissionInterval()` and selector `0x276b657a`
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
    pub struct L2OutputOracleSubmissionIntervalReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxSequencerDrift` function with signature `maxSequencerDrift()` and selector `0xe31ea108`
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
    pub struct MaxSequencerDriftReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `p2pSequencerAddress` function with signature `p2pSequencerAddress()` and selector `0xe7d6cd42`
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
    pub struct P2PsequencerAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `portalGuardian` function with signature `portalGuardian()` and selector `0x9b4bb48f`
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
    pub struct PortalGuardianReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxyAdminOwner` function with signature `proxyAdminOwner()` and selector `0xdad544e0`
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
    pub struct ProxyAdminOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `recommendedProtocolVersion` function with signature `recommendedProtocolVersion()` and selector `0x67cef446`
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
    pub struct RecommendedProtocolVersionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `requiredProtocolVersion` function with signature `requiredProtocolVersion()` and selector `0x38c27159`
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
    pub struct RequiredProtocolVersionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sequencerFeeVaultRecipient` function with signature `sequencerFeeVaultRecipient()` and selector `0x42c80295`
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
    pub struct SequencerFeeVaultRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `sequencerWindowSize` function with signature `sequencerWindowSize()` and selector `0xadd6ce44`
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
    pub struct SequencerWindowSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `systemConfigStartBlock` function with signature `systemConfigStartBlock()` and selector `0x102c9aa4`
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
    pub struct SystemConfigStartBlockReturn(pub ::ethers::core::types::U256);
}

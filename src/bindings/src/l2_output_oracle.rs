pub use l2_output_oracle::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::all,
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod l2_output_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_submissionInterval"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_l2BlockTime"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_finalizationPeriodSeconds",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CHALLENGER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CHALLENGER"),
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
                    ::std::borrow::ToOwned::to_owned("FINALIZATION_PERIOD_SECONDS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FINALIZATION_PERIOD_SECONDS",
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
                    ::std::borrow::ToOwned::to_owned("L2_BLOCK_TIME"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("L2_BLOCK_TIME"),
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
                    ::std::borrow::ToOwned::to_owned("PROPOSER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PROPOSER"),
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
                    ::std::borrow::ToOwned::to_owned("SUBMISSION_INTERVAL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SUBMISSION_INTERVAL",
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
                    ::std::borrow::ToOwned::to_owned("challenger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("challenger"),
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
                    ::std::borrow::ToOwned::to_owned("computeL2Timestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeL2Timestamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2BlockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("deleteL2Outputs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deleteL2Outputs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2OutputIndex"),
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
                    ::std::borrow::ToOwned::to_owned("getL2Output"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getL2Output"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2OutputIndex"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Types.OutputProposal",
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
                    ::std::borrow::ToOwned::to_owned("getL2OutputAfter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getL2OutputAfter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2BlockNumber"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Types.OutputProposal",
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
                    ::std::borrow::ToOwned::to_owned("getL2OutputIndexAfter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getL2OutputIndexAfter",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2BlockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_startingBlockNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_startingTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proposer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_challenger"),
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
                    ::std::borrow::ToOwned::to_owned("latestBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestBlockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("latestOutputIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestOutputIndex"),
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
                    ::std::borrow::ToOwned::to_owned("nextBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nextBlockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("nextOutputIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nextOutputIndex"),
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
                    ::std::borrow::ToOwned::to_owned("proposeL2Output"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposeL2Output"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_outputRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l2BlockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l1BlockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l1BlockNumber"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposer"),
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
                    ::std::borrow::ToOwned::to_owned("startingBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "startingBlockNumber",
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
                    ::std::borrow::ToOwned::to_owned("startingTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startingTimestamp"),
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
                    ::std::borrow::ToOwned::to_owned("submissionInterval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submissionInterval"),
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
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("OutputProposed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OutputProposed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("outputRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l2OutputIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l2BlockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("l1Timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("OutputsDeleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OutputsDeleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "prevNextOutputIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newNextOutputIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
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
    pub static L2OUTPUTORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x18\xAE8\x03\x80b\0\x18\xAE\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02\xF3V[`\0\x82\x11b\0\0\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FL2OutputOracle: L2 block time mu`D\x82\x01R\x7Fst be greater than 0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83\x11b\0\x01(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FL2OutputOracle: submission inter`D\x82\x01R\x7Fval must be greater than 0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\0\xA7V[`\x80\x83\x90R`\xA0\x82\x90R`\xC0\x81\x90Rb\0\x01F`\0\x80\x80\x80b\0\x01OV[PPPb\0\x03\"V[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15b\0\x01rWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[b\0\x01\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01b\0\0\xA7V[`\0\x80Ta\xFF\xFF\x19\x16`\xFF\x83\x16\x17a\x01\0\x17\x90UB\x84\x11\x15b\0\x02qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FL2OutputOracle: starting L2 time\x90\x82\x01R\x7Fstamp must be less than current `d\x82\x01Rctime`\xE0\x1B`\x84\x82\x01R`\xA4\x01b\0\0\xA7V[`\x02\x84\x90U`\x01\x85\x90U`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x04\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x03\tW`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa\x152b\0\x03|`\09`\0\x81\x81a\x04\xB3\x01R\x81\x81a\x05q\x01Ra\x0B\xEB\x01R`\0\x81\x81a\x01\xA1\x01R\x81\x81a\x03\xB9\x01Ra\x12t\x01R`\0\x81\x81a\x02\x1F\x01R\x81\x81a\x05;\x01Ra\x12\xC2\x01Ra\x152`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x8AW`\x005`\xE0\x1C\x80c\x89\xC4L\xBB\x11a\0\xD6W\x80c\xCE]\xB8\xD6\x11a\0\x7FW\x80c\xDC\xEC3H\x11a\0YW\x80c\xDC\xEC3H\x14a\x05\x17W\x80c\xE1\xA4\x1B\xCF\x14a\x05,W\x80c\xF4\xDA\xA2\x91\x14a\x05_W`\0\x80\xFD[\x80c\xCE]\xB8\xD6\x14a\x04\xA4W\x80c\xCF\x8E\\\xF0\x14a\x04\xD7W\x80c\xD1\xDE\x85l\x14a\x04\xF7W`\0\x80\xFD[\x80c\xA2Z\xE5W\x11a\0\xB0W\x80c\xA2Z\xE5W\x14a\x03\xF0W\x80c\xA8\xE4\xFB\x90\x14a\x04LW\x80c\xBF\xFA\x7F\x0F\x14a\x04yW`\0\x80\xFD[\x80c\x89\xC4L\xBB\x14a\x03\x8AW\x80c\x93\x99\x1A\xF3\x14a\x03\xAAW\x80c\x9A\xAA\xB6H\x14a\x03\xDDW`\0\x80\xFD[\x80ci\xF1n\xEC\x11a\x018W\x80cp\x87*\xA5\x11a\x01\x12W\x80cp\x87*\xA5\x14a\x03>W\x80c\x7F\0d \x14a\x03TW\x80c\x88xbr\x14a\x03tW`\0\x80\xFD[\x80ci\xF1n\xEC\x14a\x02\xE9W\x80cj\xBC\xF5c\x14a\x02\xFEW\x80ckM\x98\xDD\x14a\x03\x13W`\0\x80\xFD[\x80cR\x993\xDF\x11a\x01iW\x80cR\x993\xDF\x14a\x02\rW\x80cSM\xB0\xE2\x14a\x02AW\x80cT\xFDMP\x14a\x02\x93W`\0\x80\xFD[\x80b!4\xCC\x14a\x01\x8FW\x80c\x01\x9E')\x14a\x01\xD6W\x80cE\x99\xC7\x88\x14a\x01\xF8W[`\0\x80\xFD[4\x80\x15a\x01\x9BW`\0\x80\xFD[Pa\x01\xC3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xE2W`\0\x80\xFD[Pa\x01\xF6a\x01\xF16`\x04a\x13\x1CV[a\x05\x93V[\0[4\x80\x15a\x02\x04W`\0\x80\xFD[Pa\x01\xC3a\x07\xF2V[4\x80\x15a\x02\x19W`\0\x80\xFD[Pa\x01\xC3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02MW`\0\x80\xFD[P`\x04Ta\x02n\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xCDV[4\x80\x15a\x02\x9FW`\0\x80\xFD[Pa\x02\xDC`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.5.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xCD\x91\x90a\x13bV[4\x80\x15a\x02\xF5W`\0\x80\xFD[Pa\x01\xC3a\x08eV[4\x80\x15a\x03\nW`\0\x80\xFD[P`\x03Ta\x01\xC3V[4\x80\x15a\x03\x1FW`\0\x80\xFD[P`\x04Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02nV[4\x80\x15a\x03JW`\0\x80\xFD[Pa\x01\xC3`\x01T\x81V[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x01\xC3a\x03o6`\x04a\x13\xD5V[a\x08wV[4\x80\x15a\x03\x80W`\0\x80\xFD[Pa\x01\xC3`\x02T\x81V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x01\xF6a\x03\xA56`\x04a\x13\xD5V[a\n\x8BV[4\x80\x15a\x03\xB6W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xC3V[a\x01\xF6a\x03\xEB6`\x04a\x13\xEEV[a\rCV[4\x80\x15a\x03\xFCW`\0\x80\xFD[Pa\x04\x10a\x04\x0B6`\x04a\x13\xD5V[a\x11\xA4V[`@\x80Q\x82Q\x81R` \x80\x84\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01\xCDV[4\x80\x15a\x04XW`\0\x80\xFD[P`\x05Ta\x02n\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\x85W`\0\x80\xFD[P`\x05Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02nV[4\x80\x15a\x04\xB0W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xC3V[4\x80\x15a\x04\xE3W`\0\x80\xFD[Pa\x04\x10a\x04\xF26`\x04a\x13\xD5V[a\x128V[4\x80\x15a\x05\x03W`\0\x80\xFD[Pa\x01\xC3a\x05\x126`\x04a\x13\xD5V[a\x12pV[4\x80\x15a\x05#W`\0\x80\xFD[Pa\x01\xC3a\x12\xBEV[4\x80\x15a\x058W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xC3V[4\x80\x15a\x05kW`\0\x80\xFD[Pa\x01\xC3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x05\xB5WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x06FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90UB\x84\x11\x15a\x07.W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FL2OutputOracle: starting L2 time\x90\x82\x01R\x7Fstamp must be less than current `d\x82\x01R\x7Ftime\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\x02\x84\x90U`\x01\x85\x90U`\x05\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`\x04\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x03T`\0\x90\x15a\x08\\W`\x03\x80Ta\x08\r\x90`\x01\x90a\x14OV[\x81T\x81\x10a\x08\x1DWa\x08\x1Da\x14fV[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01`\x01\x01Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90PV[`\x01T[\x90P\x90V[`\x03T`\0\x90a\x08`\x90`\x01\x90a\x14OV[`\0a\x08\x81a\x07\xF2V[\x82\x11\x15a\t6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FL2OutputOracle: cannot get outpu`D\x82\x01R\x7Ft for a block that has not been `d\x82\x01R\x7Fproposed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\x03Ta\t\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FL2OutputOracle: cannot get outpu`D\x82\x01R\x7Ft as no outputs have been propos`d\x82\x01R\x7Fed yet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\x03T`\0\x90[\x80\x82\x10\x15a\n\x84W`\0`\x02a\n\x08\x83\x85a\x14\x95V[a\n\x12\x91\x90a\x14\xADV[\x90P\x84`\x03\x82\x81T\x81\x10a\n(Wa\n(a\x14fV[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01`\x01\x01Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\nzWa\ns\x81`\x01a\x14\x95V[\x92Pa\n~V[\x80\x91P[Pa\t\xF2V[P\x92\x91PPV[`\x04Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0B2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FL2OutputOracle: only the challen`D\x82\x01R\x7Fger address can delete outputs\0\0`d\x82\x01R`\x84\x01a\x06=V[`\x03T\x81\x10a\x0B\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FL2OutputOracle: cannot delete ou`D\x82\x01R\x7Ftputs after the latest output in`d\x82\x01R\x7Fdex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03\x82\x81T\x81\x10a\x0C\x1DWa\x0C\x1Da\x14fV[`\0\x91\x82R` \x90\x91 `\x01`\x02\x90\x92\x02\x01\x01Ta\x0CM\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x14OV[\x10a\r\0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FL2OutputOracle: cannot delete ou`D\x82\x01R\x7Ftputs that have already been fin`d\x82\x01R\x7Falized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\0a\r\x0B`\x03T\x90V[\x90P\x81`\x03U\x81\x81\x7FN\xE3z\xC2\xC7\x86\xEC\x85\xE8u\x92\xD3\xC5\xC8\xA1\xDDf\xF8Im\xDA?\x12]\x9E\xA8\xCA_ev)\xB6`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x05Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\x10W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FL2OutputOracle: only the propose`D\x82\x01R\x7Fr address can propose new output`d\x82\x01R\x7Fs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[a\x0E\x18a\x12\xBEV[\x83\x14a\x0E\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FL2OutputOracle: block number mus`D\x82\x01R\x7Ft be equal to next expected bloc`d\x82\x01R\x7Fk number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[Ba\x0E\xD6\x84a\x12pV[\x10a\x0FcW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FL2OutputOracle: cannot propose L`D\x82\x01R\x7F2 output in the future\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06=V[\x83a\x0F\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FL2OutputOracle: L2 output propos`D\x82\x01R\x7Fal cannot be the zero hash\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06=V[\x81\x15a\x10\xACW\x81\x81@\x14a\x10\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FL2OutputOracle: block hash does `D\x82\x01R\x7Fnot match the hash at the expect`d\x82\x01R\x7Fed height\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[\x82a\x10\xB6`\x03T\x90V[\x85\x7F\xA7\xAA\xF2Q'i\xDANDN=\xE2G\xBE%d\"\\.z\x8Ft\xCF\xE5(\xE4n\x17\xD2Hh\xE2B`@Qa\x10\xE8\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`@\x80Q``\x81\x01\x82R\x92\x83Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16` \x85\x01\x90\x81R\x92\x81\x16\x91\x84\x01\x91\x82R`\x03\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x93Q`\x02\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x94\x90\x94U\x91Q\x90Q\x82\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x91\x16\x17\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x90\x91\x01UV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x03\x82\x81T\x81\x10a\x11\xD4Wa\x11\xD4a\x14fV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x02\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x03a\x12`\x83a\x08wV[\x81T\x81\x10a\x11\xD4Wa\x11\xD4a\x14fV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x83a\x12\xA1\x91\x90a\x14OV[a\x12\xAB\x91\x90a\x14\xE8V[`\x02Ta\x12\xB8\x91\x90a\x14\x95V[\x92\x91PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x12\xE9a\x07\xF2V[a\x08`\x91\x90a\x14\x95V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\x17W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x132W`\0\x80\xFD[\x845\x93P` \x85\x015\x92Pa\x13I`@\x86\x01a\x12\xF3V[\x91Pa\x13W``\x86\x01a\x12\xF3V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x13\x8FW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x13sV[\x81\x81\x11\x15a\x13\xA1W`\0`@\x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14\x04W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x14aWa\x14aa\x14 V[P\x03\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x14\xA8Wa\x14\xA8a\x14 V[P\x01\x90V[`\0\x82a\x14\xE3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x15 Wa\x15 a\x14 V[P\x02\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L2OUTPUTORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x8AW`\x005`\xE0\x1C\x80c\x89\xC4L\xBB\x11a\0\xD6W\x80c\xCE]\xB8\xD6\x11a\0\x7FW\x80c\xDC\xEC3H\x11a\0YW\x80c\xDC\xEC3H\x14a\x05\x17W\x80c\xE1\xA4\x1B\xCF\x14a\x05,W\x80c\xF4\xDA\xA2\x91\x14a\x05_W`\0\x80\xFD[\x80c\xCE]\xB8\xD6\x14a\x04\xA4W\x80c\xCF\x8E\\\xF0\x14a\x04\xD7W\x80c\xD1\xDE\x85l\x14a\x04\xF7W`\0\x80\xFD[\x80c\xA2Z\xE5W\x11a\0\xB0W\x80c\xA2Z\xE5W\x14a\x03\xF0W\x80c\xA8\xE4\xFB\x90\x14a\x04LW\x80c\xBF\xFA\x7F\x0F\x14a\x04yW`\0\x80\xFD[\x80c\x89\xC4L\xBB\x14a\x03\x8AW\x80c\x93\x99\x1A\xF3\x14a\x03\xAAW\x80c\x9A\xAA\xB6H\x14a\x03\xDDW`\0\x80\xFD[\x80ci\xF1n\xEC\x11a\x018W\x80cp\x87*\xA5\x11a\x01\x12W\x80cp\x87*\xA5\x14a\x03>W\x80c\x7F\0d \x14a\x03TW\x80c\x88xbr\x14a\x03tW`\0\x80\xFD[\x80ci\xF1n\xEC\x14a\x02\xE9W\x80cj\xBC\xF5c\x14a\x02\xFEW\x80ckM\x98\xDD\x14a\x03\x13W`\0\x80\xFD[\x80cR\x993\xDF\x11a\x01iW\x80cR\x993\xDF\x14a\x02\rW\x80cSM\xB0\xE2\x14a\x02AW\x80cT\xFDMP\x14a\x02\x93W`\0\x80\xFD[\x80b!4\xCC\x14a\x01\x8FW\x80c\x01\x9E')\x14a\x01\xD6W\x80cE\x99\xC7\x88\x14a\x01\xF8W[`\0\x80\xFD[4\x80\x15a\x01\x9BW`\0\x80\xFD[Pa\x01\xC3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xE2W`\0\x80\xFD[Pa\x01\xF6a\x01\xF16`\x04a\x13\x1CV[a\x05\x93V[\0[4\x80\x15a\x02\x04W`\0\x80\xFD[Pa\x01\xC3a\x07\xF2V[4\x80\x15a\x02\x19W`\0\x80\xFD[Pa\x01\xC3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02MW`\0\x80\xFD[P`\x04Ta\x02n\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xCDV[4\x80\x15a\x02\x9FW`\0\x80\xFD[Pa\x02\xDC`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.5.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xCD\x91\x90a\x13bV[4\x80\x15a\x02\xF5W`\0\x80\xFD[Pa\x01\xC3a\x08eV[4\x80\x15a\x03\nW`\0\x80\xFD[P`\x03Ta\x01\xC3V[4\x80\x15a\x03\x1FW`\0\x80\xFD[P`\x04Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02nV[4\x80\x15a\x03JW`\0\x80\xFD[Pa\x01\xC3`\x01T\x81V[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x01\xC3a\x03o6`\x04a\x13\xD5V[a\x08wV[4\x80\x15a\x03\x80W`\0\x80\xFD[Pa\x01\xC3`\x02T\x81V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x01\xF6a\x03\xA56`\x04a\x13\xD5V[a\n\x8BV[4\x80\x15a\x03\xB6W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xC3V[a\x01\xF6a\x03\xEB6`\x04a\x13\xEEV[a\rCV[4\x80\x15a\x03\xFCW`\0\x80\xFD[Pa\x04\x10a\x04\x0B6`\x04a\x13\xD5V[a\x11\xA4V[`@\x80Q\x82Q\x81R` \x80\x84\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01\xCDV[4\x80\x15a\x04XW`\0\x80\xFD[P`\x05Ta\x02n\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\x85W`\0\x80\xFD[P`\x05Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02nV[4\x80\x15a\x04\xB0W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xC3V[4\x80\x15a\x04\xE3W`\0\x80\xFD[Pa\x04\x10a\x04\xF26`\x04a\x13\xD5V[a\x128V[4\x80\x15a\x05\x03W`\0\x80\xFD[Pa\x01\xC3a\x05\x126`\x04a\x13\xD5V[a\x12pV[4\x80\x15a\x05#W`\0\x80\xFD[Pa\x01\xC3a\x12\xBEV[4\x80\x15a\x058W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xC3V[4\x80\x15a\x05kW`\0\x80\xFD[Pa\x01\xC3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x05\xB5WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x06FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90UB\x84\x11\x15a\x07.W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FL2OutputOracle: starting L2 time\x90\x82\x01R\x7Fstamp must be less than current `d\x82\x01R\x7Ftime\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\x02\x84\x90U`\x01\x85\x90U`\x05\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x17\x90\x92U`\x04\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x03T`\0\x90\x15a\x08\\W`\x03\x80Ta\x08\r\x90`\x01\x90a\x14OV[\x81T\x81\x10a\x08\x1DWa\x08\x1Da\x14fV[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01`\x01\x01Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90PV[`\x01T[\x90P\x90V[`\x03T`\0\x90a\x08`\x90`\x01\x90a\x14OV[`\0a\x08\x81a\x07\xF2V[\x82\x11\x15a\t6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FL2OutputOracle: cannot get outpu`D\x82\x01R\x7Ft for a block that has not been `d\x82\x01R\x7Fproposed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\x03Ta\t\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FL2OutputOracle: cannot get outpu`D\x82\x01R\x7Ft as no outputs have been propos`d\x82\x01R\x7Fed yet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\x03T`\0\x90[\x80\x82\x10\x15a\n\x84W`\0`\x02a\n\x08\x83\x85a\x14\x95V[a\n\x12\x91\x90a\x14\xADV[\x90P\x84`\x03\x82\x81T\x81\x10a\n(Wa\n(a\x14fV[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01`\x01\x01Tp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\nzWa\ns\x81`\x01a\x14\x95V[\x92Pa\n~V[\x80\x91P[Pa\t\xF2V[P\x92\x91PPV[`\x04Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0B2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FL2OutputOracle: only the challen`D\x82\x01R\x7Fger address can delete outputs\0\0`d\x82\x01R`\x84\x01a\x06=V[`\x03T\x81\x10a\x0B\xE9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FL2OutputOracle: cannot delete ou`D\x82\x01R\x7Ftputs after the latest output in`d\x82\x01R\x7Fdex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03\x82\x81T\x81\x10a\x0C\x1DWa\x0C\x1Da\x14fV[`\0\x91\x82R` \x90\x91 `\x01`\x02\x90\x92\x02\x01\x01Ta\x0CM\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba\x14OV[\x10a\r\0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FL2OutputOracle: cannot delete ou`D\x82\x01R\x7Ftputs that have already been fin`d\x82\x01R\x7Falized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[`\0a\r\x0B`\x03T\x90V[\x90P\x81`\x03U\x81\x81\x7FN\xE3z\xC2\xC7\x86\xEC\x85\xE8u\x92\xD3\xC5\xC8\xA1\xDDf\xF8Im\xDA?\x12]\x9E\xA8\xCA_ev)\xB6`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x05Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0E\x10W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FL2OutputOracle: only the propose`D\x82\x01R\x7Fr address can propose new output`d\x82\x01R\x7Fs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[a\x0E\x18a\x12\xBEV[\x83\x14a\x0E\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FL2OutputOracle: block number mus`D\x82\x01R\x7Ft be equal to next expected bloc`d\x82\x01R\x7Fk number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[Ba\x0E\xD6\x84a\x12pV[\x10a\x0FcW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FL2OutputOracle: cannot propose L`D\x82\x01R\x7F2 output in the future\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06=V[\x83a\x0F\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FL2OutputOracle: L2 output propos`D\x82\x01R\x7Fal cannot be the zero hash\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06=V[\x81\x15a\x10\xACW\x81\x81@\x14a\x10\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FL2OutputOracle: block hash does `D\x82\x01R\x7Fnot match the hash at the expect`d\x82\x01R\x7Fed height\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06=V[\x82a\x10\xB6`\x03T\x90V[\x85\x7F\xA7\xAA\xF2Q'i\xDANDN=\xE2G\xBE%d\"\\.z\x8Ft\xCF\xE5(\xE4n\x17\xD2Hh\xE2B`@Qa\x10\xE8\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`@\x80Q``\x81\x01\x82R\x92\x83Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x81\x16` \x85\x01\x90\x81R\x92\x81\x16\x91\x84\x01\x91\x82R`\x03\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x93Q`\x02\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x94\x90\x94U\x91Q\x90Q\x82\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x91\x16\x17\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x90\x91\x01UV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x03\x82\x81T\x81\x10a\x11\xD4Wa\x11\xD4a\x14fV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R`\x02\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\x03a\x12`\x83a\x08wV[\x81T\x81\x10a\x11\xD4Wa\x11\xD4a\x14fV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01T\x83a\x12\xA1\x91\x90a\x14OV[a\x12\xAB\x91\x90a\x14\xE8V[`\x02Ta\x12\xB8\x91\x90a\x14\x95V[\x92\x91PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x12\xE9a\x07\xF2V[a\x08`\x91\x90a\x14\x95V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\x17W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x132W`\0\x80\xFD[\x845\x93P` \x85\x015\x92Pa\x13I`@\x86\x01a\x12\xF3V[\x91Pa\x13W``\x86\x01a\x12\xF3V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x13\x8FW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x13sV[\x81\x81\x11\x15a\x13\xA1W`\0`@\x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x14\x04W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x14aWa\x14aa\x14 V[P\x03\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x14\xA8Wa\x14\xA8a\x14 V[P\x01\x90V[`\0\x82a\x14\xE3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x15 Wa\x15 a\x14 V[P\x02\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L2OUTPUTORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L2OutputOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L2OutputOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L2OutputOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L2OutputOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L2OutputOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L2OutputOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L2OutputOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L2OUTPUTORACLE_ABI.clone(),
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
                L2OUTPUTORACLE_ABI.clone(),
                L2OUTPUTORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CHALLENGER` (0x6b4d98dd) function
        pub fn CHALLENGER(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([107, 77, 152, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FINALIZATION_PERIOD_SECONDS` (0xf4daa291) function
        pub fn FINALIZATION_PERIOD_SECONDS(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 218, 162, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `L2_BLOCK_TIME` (0x002134cc) function
        pub fn l2_block_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 33, 52, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROPOSER` (0xbffa7f0f) function
        pub fn PROPOSER(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([191, 250, 127, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SUBMISSION_INTERVAL` (0x529933df) function
        pub fn SUBMISSION_INTERVAL(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([82, 153, 51, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `challenger` (0x534db0e2) function
        pub fn challenger(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([83, 77, 176, 226], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeL2Timestamp` (0xd1de856c) function
        pub fn compute_l2_timestamp(
            &self,
            l_2_block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([209, 222, 133, 108], l_2_block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteL2Outputs` (0x89c44cbb) function
        pub fn delete_l2_outputs(
            &self,
            l_2_output_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 196, 76, 187], l_2_output_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizationPeriodSeconds` (0xce5db8d6) function
        pub fn finalizationPeriodSeconds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 93, 184, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getL2Output` (0xa25ae557) function
        pub fn get_l2_output(
            &self,
            l_2_output_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, OutputProposal> {
            self.0
                .method_hash([162, 90, 229, 87], l_2_output_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getL2OutputAfter` (0xcf8e5cf0) function
        pub fn get_l2_output_after(
            &self,
            l_2_block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, OutputProposal> {
            self.0
                .method_hash([207, 142, 92, 240], l_2_block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getL2OutputIndexAfter` (0x7f006420) function
        pub fn get_l2_output_index_after(
            &self,
            l_2_block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([127, 0, 100, 32], l_2_block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x019e2729) function
        pub fn initialize(
            &self,
            starting_block_number: ::ethers::core::types::U256,
            starting_timestamp: ::ethers::core::types::U256,
            proposer: ::ethers::core::types::Address,
            challenger: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [1, 158, 39, 41],
                    (starting_block_number, starting_timestamp, proposer, challenger),
                )
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
        ///Calls the contract's `latestBlockNumber` (0x4599c788) function
        pub fn latest_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([69, 153, 199, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestOutputIndex` (0x69f16eec) function
        pub fn latest_output_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([105, 241, 110, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextBlockNumber` (0xdcec3348) function
        pub fn next_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 236, 51, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextOutputIndex` (0x6abcf563) function
        pub fn next_output_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 188, 245, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposeL2Output` (0x9aaab648) function
        pub fn propose_l2_output(
            &self,
            output_root: [u8; 32],
            l_2_block_number: ::ethers::core::types::U256,
            l_1_block_hash: [u8; 32],
            l_1_block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [154, 170, 182, 72],
                    (output_root, l_2_block_number, l_1_block_hash, l_1_block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposer` (0xa8e4fb90) function
        pub fn proposer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 228, 251, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startingBlockNumber` (0x70872aa5) function
        pub fn starting_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 135, 42, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startingTimestamp` (0x88786272) function
        pub fn starting_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([136, 120, 98, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submissionInterval` (0xe1a41bcf) function
        pub fn submissionInterval(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([225, 164, 27, 207], ())
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
        ///Gets the contract's `OutputProposed` event
        pub fn output_proposed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OutputProposedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OutputsDeleted` event
        pub fn outputs_deleted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OutputsDeletedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            L2OutputOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L2OutputOracle<M> {
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
        name = "OutputProposed",
        abi = "OutputProposed(bytes32,uint256,uint256,uint256)"
    )]
    pub struct OutputProposedFilter {
        #[ethevent(indexed)]
        pub output_root: [u8; 32],
        #[ethevent(indexed)]
        pub l_2_output_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub l_2_block_number: ::ethers::core::types::U256,
        pub l_1_timestamp: ::ethers::core::types::U256,
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
    #[ethevent(name = "OutputsDeleted", abi = "OutputsDeleted(uint256,uint256)")]
    pub struct OutputsDeletedFilter {
        #[ethevent(indexed)]
        pub prev_next_output_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub new_next_output_index: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L2OutputOracleEvents {
        InitializedFilter(InitializedFilter),
        OutputProposedFilter(OutputProposedFilter),
        OutputsDeletedFilter(OutputsDeletedFilter),
    }
    impl ::ethers::contract::EthLogDecode for L2OutputOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(L2OutputOracleEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OutputProposedFilter::decode_log(log) {
                return Ok(L2OutputOracleEvents::OutputProposedFilter(decoded));
            }
            if let Ok(decoded) = OutputsDeletedFilter::decode_log(log) {
                return Ok(L2OutputOracleEvents::OutputsDeletedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L2OutputOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutputProposedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OutputsDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for L2OutputOracleEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OutputProposedFilter> for L2OutputOracleEvents {
        fn from(value: OutputProposedFilter) -> Self {
            Self::OutputProposedFilter(value)
        }
    }
    impl ::core::convert::From<OutputsDeletedFilter> for L2OutputOracleEvents {
        fn from(value: OutputsDeletedFilter) -> Self {
            Self::OutputsDeletedFilter(value)
        }
    }
    ///Container type for all input parameters for the `CHALLENGER` function with signature `CHALLENGER()` and selector `0x6b4d98dd`
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
    #[ethcall(name = "CHALLENGER", abi = "CHALLENGER()")]
    pub struct CHALLENGERCall;
    ///Container type for all input parameters for the `FINALIZATION_PERIOD_SECONDS` function with signature `FINALIZATION_PERIOD_SECONDS()` and selector `0xf4daa291`
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
        name = "FINALIZATION_PERIOD_SECONDS",
        abi = "FINALIZATION_PERIOD_SECONDS()"
    )]
    pub struct FINALIZATION_PERIOD_SECONDSCall;
    ///Container type for all input parameters for the `L2_BLOCK_TIME` function with signature `L2_BLOCK_TIME()` and selector `0x002134cc`
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
    #[ethcall(name = "L2_BLOCK_TIME", abi = "L2_BLOCK_TIME()")]
    pub struct L2BlockTimeCall;
    ///Container type for all input parameters for the `PROPOSER` function with signature `PROPOSER()` and selector `0xbffa7f0f`
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
    #[ethcall(name = "PROPOSER", abi = "PROPOSER()")]
    pub struct PROPOSERCall;
    ///Container type for all input parameters for the `SUBMISSION_INTERVAL` function with signature `SUBMISSION_INTERVAL()` and selector `0x529933df`
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
    #[ethcall(name = "SUBMISSION_INTERVAL", abi = "SUBMISSION_INTERVAL()")]
    pub struct SUBMISSION_INTERVALCall;
    ///Container type for all input parameters for the `challenger` function with signature `challenger()` and selector `0x534db0e2`
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
    #[ethcall(name = "challenger", abi = "challenger()")]
    pub struct challengerCall;
    ///Container type for all input parameters for the `computeL2Timestamp` function with signature `computeL2Timestamp(uint256)` and selector `0xd1de856c`
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
    #[ethcall(name = "computeL2Timestamp", abi = "computeL2Timestamp(uint256)")]
    pub struct ComputeL2TimestampCall {
        pub l_2_block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deleteL2Outputs` function with signature `deleteL2Outputs(uint256)` and selector `0x89c44cbb`
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
    #[ethcall(name = "deleteL2Outputs", abi = "deleteL2Outputs(uint256)")]
    pub struct DeleteL2OutputsCall {
        pub l_2_output_index: ::ethers::core::types::U256,
    }
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
    pub struct finalizationPeriodSecondsCall;
    ///Container type for all input parameters for the `getL2Output` function with signature `getL2Output(uint256)` and selector `0xa25ae557`
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
    #[ethcall(name = "getL2Output", abi = "getL2Output(uint256)")]
    pub struct GetL2OutputCall {
        pub l_2_output_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getL2OutputAfter` function with signature `getL2OutputAfter(uint256)` and selector `0xcf8e5cf0`
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
    #[ethcall(name = "getL2OutputAfter", abi = "getL2OutputAfter(uint256)")]
    pub struct GetL2OutputAfterCall {
        pub l_2_block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getL2OutputIndexAfter` function with signature `getL2OutputIndexAfter(uint256)` and selector `0x7f006420`
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
    #[ethcall(name = "getL2OutputIndexAfter", abi = "getL2OutputIndexAfter(uint256)")]
    pub struct GetL2OutputIndexAfterCall {
        pub l_2_block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,uint256,address,address)` and selector `0x019e2729`
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
    #[ethcall(name = "initialize", abi = "initialize(uint256,uint256,address,address)")]
    pub struct InitializeCall {
        pub starting_block_number: ::ethers::core::types::U256,
        pub starting_timestamp: ::ethers::core::types::U256,
        pub proposer: ::ethers::core::types::Address,
        pub challenger: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `l2BlockTime` function with signature `l2BlockTime()` and selector `0x93991af3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        // Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    // #[ethcall(name = "l2BlockTime", abi = "l2BlockTime()")]
    // pub struct L2BlockTimeCall;
    // ///Container type for all input parameters for the `latestBlockNumber` function with signature `latestBlockNumber()` and selector `0x4599c788`
    // #[derive(
    //     Clone,
    //     ::ethers::contract::EthCall,
    //     ::ethers::contract::EthDisplay,
    //     Default,
    //     Debug,
    //     PartialEq,
    //     Eq,
    //     Hash
    // )]
    #[ethcall(name = "latestBlockNumber", abi = "latestBlockNumber()")]
    pub struct LatestBlockNumberCall;
    ///Container type for all input parameters for the `latestOutputIndex` function with signature `latestOutputIndex()` and selector `0x69f16eec`
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
    #[ethcall(name = "latestOutputIndex", abi = "latestOutputIndex()")]
    pub struct LatestOutputIndexCall;
    ///Container type for all input parameters for the `nextBlockNumber` function with signature `nextBlockNumber()` and selector `0xdcec3348`
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
    #[ethcall(name = "nextBlockNumber", abi = "nextBlockNumber()")]
    pub struct NextBlockNumberCall;
    ///Container type for all input parameters for the `nextOutputIndex` function with signature `nextOutputIndex()` and selector `0x6abcf563`
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
    #[ethcall(name = "nextOutputIndex", abi = "nextOutputIndex()")]
    pub struct NextOutputIndexCall;
    ///Container type for all input parameters for the `proposeL2Output` function with signature `proposeL2Output(bytes32,uint256,bytes32,uint256)` and selector `0x9aaab648`
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
        name = "proposeL2Output",
        abi = "proposeL2Output(bytes32,uint256,bytes32,uint256)"
    )]
    pub struct ProposeL2OutputCall {
        pub output_root: [u8; 32],
        pub l_2_block_number: ::ethers::core::types::U256,
        pub l_1_block_hash: [u8; 32],
        pub l_1_block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proposer` function with signature `proposer()` and selector `0xa8e4fb90`
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
    #[ethcall(name = "proposer", abi = "proposer()")]
    pub struct proposerCall;
    ///Container type for all input parameters for the `startingBlockNumber` function with signature `startingBlockNumber()` and selector `0x70872aa5`
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
    #[ethcall(name = "startingBlockNumber", abi = "startingBlockNumber()")]
    pub struct StartingBlockNumberCall;
    ///Container type for all input parameters for the `startingTimestamp` function with signature `startingTimestamp()` and selector `0x88786272`
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
    #[ethcall(name = "startingTimestamp", abi = "startingTimestamp()")]
    pub struct StartingTimestampCall;
    ///Container type for all input parameters for the `submissionInterval` function with signature `submissionInterval()` and selector `0xe1a41bcf`
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
    #[ethcall(name = "submissionInterval", abi = "submissionInterval()")]
    pub struct submissionIntervalCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L2OutputOracleCalls {
        CHALLENGER(CHALLENGERCall),
        FINALIZATION_PERIOD_SECONDS(FINALIZATION_PERIOD_SECONDSCall),
        L2BlockTime(L2BlockTimeCall),
        PROPOSER(PROPOSERCall),
        SUBMISSION_INTERVAL(SUBMISSION_INTERVALCall),
        challenger(challengerCall),
        ComputeL2Timestamp(ComputeL2TimestampCall),
        DeleteL2Outputs(DeleteL2OutputsCall),
        finalizationPeriodSeconds(finalizationPeriodSecondsCall),
        GetL2Output(GetL2OutputCall),
        GetL2OutputAfter(GetL2OutputAfterCall),
        GetL2OutputIndexAfter(GetL2OutputIndexAfterCall),
        Initialize(InitializeCall),
        LatestBlockNumber(LatestBlockNumberCall),
        LatestOutputIndex(LatestOutputIndexCall),
        NextBlockNumber(NextBlockNumberCall),
        NextOutputIndex(NextOutputIndexCall),
        ProposeL2Output(ProposeL2OutputCall),
        proposer(proposerCall),
        StartingBlockNumber(StartingBlockNumberCall),
        StartingTimestamp(StartingTimestampCall),
        submissionInterval(submissionIntervalCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for L2OutputOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CHALLENGERCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CHALLENGER(decoded));
            }
            if let Ok(decoded) = <FINALIZATION_PERIOD_SECONDSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FINALIZATION_PERIOD_SECONDS(decoded));
            }
            if let Ok(decoded) = <L2BlockTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2BlockTime(decoded));
            }
            if let Ok(decoded) = <PROPOSERCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PROPOSER(decoded));
            }
            if let Ok(decoded) = <SUBMISSION_INTERVALCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SUBMISSION_INTERVAL(decoded));
            }
            if let Ok(decoded) = <challengerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::challenger(decoded));
            }
            if let Ok(decoded) = <ComputeL2TimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeL2Timestamp(decoded));
            }
            if let Ok(decoded) = <DeleteL2OutputsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeleteL2Outputs(decoded));
            }
            if let Ok(decoded) = <finalizationPeriodSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::finalizationPeriodSeconds(decoded));
            }
            if let Ok(decoded) = <GetL2OutputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetL2Output(decoded));
            }
            if let Ok(decoded) = <GetL2OutputAfterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetL2OutputAfter(decoded));
            }
            if let Ok(decoded) = <GetL2OutputIndexAfterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetL2OutputIndexAfter(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            // if let Ok(decoded) = <LatestBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
            //     data,
            // ) {
            //     return Ok(Self::LatestBlockNumber(decoded));
            // }
            if let Ok(decoded) = <LatestOutputIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestOutputIndex(decoded));
            }
            if let Ok(decoded) = <NextBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NextBlockNumber(decoded));
            }
            if let Ok(decoded) = <NextOutputIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NextOutputIndex(decoded));
            }
            if let Ok(decoded) = <ProposeL2OutputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposeL2Output(decoded));
            }
            if let Ok(decoded) = <proposerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::proposer(decoded));
            }
            if let Ok(decoded) = <StartingBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartingBlockNumber(decoded));
            }
            if let Ok(decoded) = <StartingTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartingTimestamp(decoded));
            }
            if let Ok(decoded) = <submissionIntervalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::submissionInterval(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L2OutputOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CHALLENGER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FINALIZATION_PERIOD_SECONDS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2BlockTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PROPOSER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUBMISSION_INTERVAL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::challenger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeL2Timestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteL2Outputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::finalizationPeriodSeconds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetL2Output(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetL2OutputAfter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetL2OutputIndexAfter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2BlockTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestOutputIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextOutputIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposeL2Output(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::proposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartingBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartingTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::submissionInterval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for L2OutputOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CHALLENGER(element) => ::core::fmt::Display::fmt(element, f),
                Self::FINALIZATION_PERIOD_SECONDS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2BlockTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::PROPOSER(element) => ::core::fmt::Display::fmt(element, f),
                Self::SUBMISSION_INTERVAL(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::challenger(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeL2Timestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeleteL2Outputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::finalizationPeriodSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetL2Output(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetL2OutputAfter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetL2OutputIndexAfter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2BlockTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestOutputIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextOutputIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposeL2Output(element) => ::core::fmt::Display::fmt(element, f),
                Self::proposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartingBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartingTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::submissionInterval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CHALLENGERCall> for L2OutputOracleCalls {
        fn from(value: CHALLENGERCall) -> Self {
            Self::CHALLENGER(value)
        }
    }
    impl ::core::convert::From<FINALIZATION_PERIOD_SECONDSCall> for L2OutputOracleCalls {
        fn from(value: FINALIZATION_PERIOD_SECONDSCall) -> Self {
            Self::FINALIZATION_PERIOD_SECONDS(value)
        }
    }
    impl ::core::convert::From<L2BlockTimeCall> for L2OutputOracleCalls {
        fn from(value: L2BlockTimeCall) -> Self {
            Self::L2BlockTime(value)
        }
    }
    impl ::core::convert::From<PROPOSERCall> for L2OutputOracleCalls {
        fn from(value: PROPOSERCall) -> Self {
            Self::PROPOSER(value)
        }
    }
    impl ::core::convert::From<SUBMISSION_INTERVALCall> for L2OutputOracleCalls {
        fn from(value: SUBMISSION_INTERVALCall) -> Self {
            Self::SUBMISSION_INTERVAL(value)
        }
    }
    impl ::core::convert::From<challengerCall> for L2OutputOracleCalls {
        fn from(value: challengerCall) -> Self {
            Self::challenger(value)
        }
    }
    impl ::core::convert::From<ComputeL2TimestampCall> for L2OutputOracleCalls {
        fn from(value: ComputeL2TimestampCall) -> Self {
            Self::ComputeL2Timestamp(value)
        }
    }
    impl ::core::convert::From<DeleteL2OutputsCall> for L2OutputOracleCalls {
        fn from(value: DeleteL2OutputsCall) -> Self {
            Self::DeleteL2Outputs(value)
        }
    }
    impl ::core::convert::From<finalizationPeriodSecondsCall> for L2OutputOracleCalls {
        fn from(value: finalizationPeriodSecondsCall) -> Self {
            Self::finalizationPeriodSeconds(value)
        }
    }
    impl ::core::convert::From<GetL2OutputCall> for L2OutputOracleCalls {
        fn from(value: GetL2OutputCall) -> Self {
            Self::GetL2Output(value)
        }
    }
    impl ::core::convert::From<GetL2OutputAfterCall> for L2OutputOracleCalls {
        fn from(value: GetL2OutputAfterCall) -> Self {
            Self::GetL2OutputAfter(value)
        }
    }
    impl ::core::convert::From<GetL2OutputIndexAfterCall> for L2OutputOracleCalls {
        fn from(value: GetL2OutputIndexAfterCall) -> Self {
            Self::GetL2OutputIndexAfter(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for L2OutputOracleCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LatestBlockNumberCall> for L2OutputOracleCalls {
        fn from(value: LatestBlockNumberCall) -> Self {
            Self::LatestBlockNumber(value)
        }
    }
    impl ::core::convert::From<LatestOutputIndexCall> for L2OutputOracleCalls {
        fn from(value: LatestOutputIndexCall) -> Self {
            Self::LatestOutputIndex(value)
        }
    }
    impl ::core::convert::From<NextBlockNumberCall> for L2OutputOracleCalls {
        fn from(value: NextBlockNumberCall) -> Self {
            Self::NextBlockNumber(value)
        }
    }
    impl ::core::convert::From<NextOutputIndexCall> for L2OutputOracleCalls {
        fn from(value: NextOutputIndexCall) -> Self {
            Self::NextOutputIndex(value)
        }
    }
    impl ::core::convert::From<ProposeL2OutputCall> for L2OutputOracleCalls {
        fn from(value: ProposeL2OutputCall) -> Self {
            Self::ProposeL2Output(value)
        }
    }
    impl ::core::convert::From<proposerCall> for L2OutputOracleCalls {
        fn from(value: proposerCall) -> Self {
            Self::proposer(value)
        }
    }
    impl ::core::convert::From<StartingBlockNumberCall> for L2OutputOracleCalls {
        fn from(value: StartingBlockNumberCall) -> Self {
            Self::StartingBlockNumber(value)
        }
    }
    impl ::core::convert::From<StartingTimestampCall> for L2OutputOracleCalls {
        fn from(value: StartingTimestampCall) -> Self {
            Self::StartingTimestamp(value)
        }
    }
    impl ::core::convert::From<submissionIntervalCall> for L2OutputOracleCalls {
        fn from(value: submissionIntervalCall) -> Self {
            Self::submissionInterval(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L2OutputOracleCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `CHALLENGER` function with signature `CHALLENGER()` and selector `0x6b4d98dd`
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
    pub struct CHALLENGERReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `FINALIZATION_PERIOD_SECONDS` function with signature `FINALIZATION_PERIOD_SECONDS()` and selector `0xf4daa291`
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
    pub struct FINALIZATION_PERIOD_SECONDSReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `L2_BLOCK_TIME` function with signature `L2_BLOCK_TIME()` and selector `0x002134cc`
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
    ///Container type for all return fields from the `PROPOSER` function with signature `PROPOSER()` and selector `0xbffa7f0f`
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
    pub struct PROPOSERReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SUBMISSION_INTERVAL` function with signature `SUBMISSION_INTERVAL()` and selector `0x529933df`
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
    pub struct SUBMISSION_INTERVALReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `challenger` function with signature `challenger()` and selector `0x534db0e2`
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
    pub struct challengerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `computeL2Timestamp` function with signature `computeL2Timestamp(uint256)` and selector `0xd1de856c`
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
    pub struct ComputeL2TimestampReturn(pub ::ethers::core::types::U256);
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
    pub struct finalizationPeriodSecondsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getL2Output` function with signature `getL2Output(uint256)` and selector `0xa25ae557`
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
    pub struct GetL2OutputReturn(pub OutputProposal);
    ///Container type for all return fields from the `getL2OutputAfter` function with signature `getL2OutputAfter(uint256)` and selector `0xcf8e5cf0`
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
    pub struct GetL2OutputAfterReturn(pub OutputProposal);
    ///Container type for all return fields from the `getL2OutputIndexAfter` function with signature `getL2OutputIndexAfter(uint256)` and selector `0x7f006420`
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
    pub struct GetL2OutputIndexAfterReturn(pub ::ethers::core::types::U256);
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
    pub struct LatestBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestOutputIndex` function with signature `latestOutputIndex()` and selector `0x69f16eec`
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
    pub struct LatestOutputIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nextBlockNumber` function with signature `nextBlockNumber()` and selector `0xdcec3348`
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
    pub struct NextBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nextOutputIndex` function with signature `nextOutputIndex()` and selector `0x6abcf563`
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
    pub struct NextOutputIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposer` function with signature `proposer()` and selector `0xa8e4fb90`
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
    pub struct proposerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `startingBlockNumber` function with signature `startingBlockNumber()` and selector `0x70872aa5`
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
    pub struct StartingBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `startingTimestamp` function with signature `startingTimestamp()` and selector `0x88786272`
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
    pub struct StartingTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submissionInterval` function with signature `submissionInterval()` and selector `0xe1a41bcf`
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
    pub struct submissionIntervalReturn(pub ::ethers::core::types::U256);
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
    ///`OutputProposal(bytes32,uint128,uint128)`
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
    pub struct OutputProposal {
        pub output_root: [u8; 32],
        pub timestamp: u128,
        pub l_2_block_number: u128,
    }
}

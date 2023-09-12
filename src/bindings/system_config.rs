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
pub mod system_config {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BATCH_INBOX_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BATCH_INBOX_SLOT"),
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
                    ::std::borrow::ToOwned::to_owned("L1_CROSS_DOMAIN_MESSENGER_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "L1_CROSS_DOMAIN_MESSENGER_SLOT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("L1_ERC_721_BRIDGE_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "L1_ERC_721_BRIDGE_SLOT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("L1_STANDARD_BRIDGE_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "L1_STANDARD_BRIDGE_SLOT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("L2_OUTPUT_ORACLE_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "L2_OUTPUT_ORACLE_SLOT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("OPTIMISM_PORTAL_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMISM_PORTAL_SLOT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("UNSAFE_BLOCK_SIGNER_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNSAFE_BLOCK_SIGNER_SLOT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VERSION"),
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
                    ::std::borrow::ToOwned::to_owned("batchInbox"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchInbox"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("batcherHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batcherHash"),
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
                    ::std::borrow::ToOwned::to_owned("gasLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gasLimit"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_overhead"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_scalar"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_batcherHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_unsafeBlockSigner",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ResourceMetering.ResourceConfig",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_startBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_batchInbox"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_addresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SystemConfig.Addresses",
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
                    ::std::borrow::ToOwned::to_owned("l1CrossDomainMessenger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "l1CrossDomainMessenger",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("l1ERC721Bridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1ERC721Bridge"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("l1StandardBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1StandardBridge"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("l2OutputOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l2OutputOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("minimumGasLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minimumGasLimit"),
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
                    ::std::borrow::ToOwned::to_owned("optimismMintableERC20Factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "optimismMintableERC20Factory",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("optimismPortal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("optimismPortal"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("overhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("overhead"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resourceConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resourceConfig"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ResourceMetering.ResourceConfig",
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
                    ::std::borrow::ToOwned::to_owned("scalar"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scalar"),
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
                    ::std::borrow::ToOwned::to_owned("setBatcherHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBatcherHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_batcherHash"),
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
                    ::std::borrow::ToOwned::to_owned("setGasConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGasConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_overhead"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_scalar"),
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
                    ::std::borrow::ToOwned::to_owned("setGasLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGasLimit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("setResourceConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setResourceConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ResourceMetering.ResourceConfig",
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
                    ::std::borrow::ToOwned::to_owned("setUnsafeBlockSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setUnsafeBlockSigner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_unsafeBlockSigner",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("startBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startBlock"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
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
                    ::std::borrow::ToOwned::to_owned("unsafeBlockSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unsafeBlockSigner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_"),
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
                    ::std::borrow::ToOwned::to_owned("ConfigUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ConfigUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("updateType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SYSTEMCONFIG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x01`\x80\x81\x81R`\x06`\xA0\x90\x81R`\0`\xC0\x81\x81R`@\x80Q\x80\x83\x01\x82R\x86\x81R` \x80\x82\x01\x88\x90R`\x02\x82\x84\x01R``\x80\x83\x01\x86\x90R\x82\x88\x01\x86\x90R\x82\x87\x01\x86\x90R\x83Q\x94\x85\x01\x84R\x85\x85R\x90\x84\x01\x85\x90R\x91\x83\x01\x84\x90R\x90\x82\x01\x83\x90R\x93\x81\x01\x82\x90R\x91\x82\x01\x81\x90Rb\0\0\xE9\x93a\xDE\xAD\x93\x91\x92\x83\x92\x83\x92\x91\x83\x91\x90`\0\x19\x90\x83\x90b\0\0\xEFV[b\0\x0C\xEBV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15b\0\x01\x12WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[b\0\x01{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\xFF\xFF\x19\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ub\0\x01\x99b\0\x04\rV[b\0\x01\xA4\x8Bb\0\x04uV[b\0\x01\xAF\x88b\0\x04\xF4V[b\0\x01\xBB\x8A\x8Ab\0\x05FV[b\0\x01\xC6\x87b\0\x05\xAAV[b\0\x01\xD1\x86b\0\x06GV[b\0\x02\x06\x83b\0\x02\x03`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98b\0\x0B\xC8V[UV[\x81Qb\0\x02:\x90b\0\x02\x03`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067b\0\x0B\xC8V[` \x82\x01Qb\0\x02q\x90b\0\x02\x03`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8b\0\x0B\xC8V[`@\x82\x01Qb\0\x02\xA8\x90b\0\x02\x03`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwb\0\x0B\xC8V[``\x82\x01Qb\0\x02\xDF\x90b\0\x02\x03`\x01\x7F\xE5*f\x7Fq\xECv\x1B\x9B8\x1C{v\xCA\x9B\x85*\xDF~\x89\x05\xDA\x0E\n\xD4\x99\x86\xA0\xA6\x87\x18\x16b\0\x0B\xC8V[`\x80\x82\x01Qb\0\x03\x16\x90b\0\x02\x03`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADb\0\x0B\xC8V[`\xA0\x82\x01Qb\0\x03M\x90b\0\x02\x03`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\rb\0\x0B\xC8V[b\0\x03X\x84b\0\x06\xA1V[b\0\x03c\x85b\0\x07,V[b\0\x03mb\0\npV[`\x01`\x01`@\x1B\x03\x16\x87`\x01`\x01`@\x1B\x03\x16\x10\x15b\0\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R`\0\x80Q` b\0-\xF3\x839\x81Q\x91R`D\x82\x01R`d\x01b\0\x01rV[`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x04iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R`\0\x80Q` b\0.3\x839\x81Q\x91R`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01rV[b\0\x04sb\0\n\x9DV[V[b\0\x04\x7Fb\0\x0B\x04V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x04\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x01rV[b\0\x04\xF1\x81b\0\x0B`V[PV[`g\x81\x90U`@\x80Q` \x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x90\x82\x01\x90\x91R`\0[`\0`\0\x80Q` b\0.\x13\x839\x81Q\x91R\x83`@Qb\0\x05:\x91\x90b\0\x0B\xE2V[`@Q\x80\x91\x03\x90\xA3PPV[`e\x82\x90U`f\x81\x90U`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\0\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x01`\0`\0\x80Q` b\0.\x13\x839\x81Q\x91R\x83`@Qb\0\x05\x9D\x91\x90b\0\x0B\xE2V[`@Q\x80\x91\x03\x90\xA3PPPV[b\0\x05\xB4b\0\npV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x10\x15b\0\x06\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R`\0\x80Q` b\0-\xF3\x839\x81Q\x91R`D\x82\x01R`d\x01b\0\x01rV[`h\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x02b\0\x05\x18V[b\0\x06p\x81\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x03b\0\x05\x18V[`jT\x15b\0\x07\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FSystemConfig: cannot override an`D\x82\x01R\x7F already set start block\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x01rV[\x80\x15b\0\x07%W`jUV[C`jUPV[\x80`\xA0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15b\0\x07\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FSystemConfig: min base fee must `D\x82\x01R\x7Fbe less than max base\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x01rV[`\x01\x81`@\x01Q`\xFF\x16\x11b\0\x08,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: denominator must b`D\x82\x01Rne larger than 1`\x88\x1B`d\x82\x01R`\x84\x01b\0\x01rV[`hT`\x80\x82\x01Q\x82Q`\x01`\x01`@\x1B\x03\x90\x92\x16\x91b\0\x08N\x91\x90b\0\x0C:V[c\xFF\xFF\xFF\xFF\x16\x11\x15b\0\x08\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R`\0\x80Q` b\0-\xF3\x839\x81Q\x91R`D\x82\x01R`d\x01b\0\x01rV[`\0\x81` \x01Q`\xFF\x16\x11b\0\t\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: elasticity multipl`D\x82\x01Rn\x06\x96W\"\x066\x16\xE6\xE6\xF7B\x06&R\x03`\x8C\x1B`d\x82\x01R`\x84\x01b\0\x01rV[\x80Q` \x82\x01Qc\xFF\xFF\xFF\xFF\x82\x16\x91`\xFF\x90\x91\x16\x90b\0\t&\x90\x82\x90b\0\x0CeV[b\0\t2\x91\x90b\0\x0C\x97V[c\xFF\xFF\xFF\xFF\x16\x14b\0\t\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FSystemConfig: precision loss wit`D\x82\x01R\x7Fh target resource limit\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x01rV[\x80Q`i\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17d\xFF\xFF\xFF\xFF\xFF`(\x1B\x19\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02c\xFF\xFF\xFF\xFF`0\x1B\x19\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17`\x01`P\x1B`\x01`\xF0\x1B\x03\x19\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02`\x01`p\x1B`\x01`\xF0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`p\x1B`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x02\x17\x90UV[`iT`\0\x90b\0\n\x98\x90c\xFF\xFF\xFF\xFFj\x01\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16b\0\x0C\xC6V[\x90P\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\n\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R`\0\x80Q` b\0.3\x839\x81Q\x91R`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01rV[b\0\x04s3b\0\x0B`V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x04sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x01rV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15b\0\x0B\xDDWb\0\x0B\xDDb\0\x0B\xB2V[P\x03\x90V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15b\0\x0C\x11W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01b\0\x0B\xF3V[\x81\x81\x11\x15b\0\x0C$W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15b\0\x0C\\Wb\0\x0C\\b\0\x0B\xB2V[\x01\x94\x93PPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80b\0\x0C\x8BWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15b\0\x0C\xBDWb\0\x0C\xBDb\0\x0B\xB2V[\x02\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x81\x16\x84\x82\x16\x80\x83\x03\x82\x11\x15b\0\x0C\\Wb\0\x0C\\b\0\x0B\xB2V[`\x80Q`\xA0Q`\xC0Qa \xD8b\0\r\x1B`\09`\0a\x0B\x8F\x01R`\0a\x0Bf\x01R`\0a\x0B=\x01Ra \xD8`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\xA8W`\x005`\xE0\x1C\x80c\x93_\x02\x9E\x11a\x01\xACW\x80c\xCCs\x1B\x02\x11a\x01?W\x80c\xF4^e\xD8\x11a\x01\x0EW\x80c\xF8\xC6\x8D\xE0\x11a\0\xF3W\x80c\xF8\xC6\x8D\xE0\x14a\x06EW\x80c\xFD2\xAA\x0F\x14a\x06MW\x80c\xFF\xA1\xADt\x14a\x06UWa\x02\xA8V[\x80c\xF4^e\xD8\x14a\x06(W\x80c\xF6\x80\x16\xB7\x14a\x061Wa\x02\xA8V[\x80c\xCCs\x1B\x02\x14a\x04\xD0W\x80c\xDA\xC6\xE6:\x14a\x06\x04W\x80c\xE8\x1B,m\x14a\x06\x0CW\x80c\xF2\xFD\xE3\x8B\x14a\x06\x15Wa\x02\xA8V[\x80c\xBCI\xCE_\x11a\x01{W\x80c\xBCI\xCE_\x14a\x04\x9AW\x80c\xC4\xE8\xDD\xFA\x14a\x04\xA2W\x80c\xC7\x19s\xF6\x14a\x04\xAAW\x80c\xC9\xB2oa\x14a\x04\xBDWa\x02\xA8V[\x80c\x93_\x02\x9E\x14a\x04dW\x80c\x9B}\x7F\n\x14a\x04wW\x80c\xA7\x11\x98i\x14a\x04\x7FW\x80c\xB4\n\x81|\x14a\x04\x87Wa\x02\xA8V[\x80cJ\xDD2\x1D\x11a\x02?W\x80cT\xFDMP\x11a\x02\x0EW\x80ca\xD1Wh\x11a\x01\xF3W\x80ca\xD1Wh\x14a\x046W\x80cqP\x18\xA6\x14a\x04>W\x80c\x8D\xA5\xCB[\x14a\x04FWa\x02\xA8V[\x80cT\xFDMP\x14a\x04\x19W\x80c]s6\x9C\x14a\x04.Wa\x02\xA8V[\x80cJ\xDD2\x1D\x14a\x03\xB6W\x80cM\x9F\x15Y\x14a\x03\xD7W\x80cO\x16T\x0B\x14a\x03\xDFW\x80cR(\xA6\xAC\x14a\x04\x06Wa\x02\xA8V[\x80c\x18\xD19\x18\x11a\x02{W\x80c\x18\xD19\x18\x14a\x03\x88W\x80c\x19\xF5\xCE\xA8\x14a\x03\x9DW\x80c\x1F\xD1\x9E\xE1\x14a\x03\xA5W\x80cH\xCDL\xB1\x14a\x03\xADWa\x02\xA8V[\x80c\x06\xC9&W\x14a\x03/W\x80c\x07\x8F)\xCF\x14a\x03JW\x80c\nI\xCB\x03\x14a\x03wW\x80c\x0C\x18\xC1b\x14a\x03\x7FW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x037a\x06]V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Ra\x06\x8BV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03AV[a\x03Ra\x06\xC4V[a\x037`eT\x81V[a\x03\x9Ba\x03\x966`\x04a\x1A\x0CV[a\x06\xF4V[\0[a\x037a\x07\x08V[a\x03Ra\x073V[a\x037`jT\x81V[a\x03\xBEa\x07]V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03AV[a\x03Ra\x07\x83V[a\x037\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x81V[a\x03\x9Ba\x04\x146`\x04a\x1CVV[a\x07\xB3V[a\x04!a\x0B6V[`@Qa\x03A\x91\x90a\x1E\x0EV[a\x037a\x0B\xD9V[a\x037a\x0C\x04V[a\x03\x9Ba\x0C/V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03RV[a\x03\x9Ba\x04r6`\x04a\x1E!V[a\x0CCV[a\x03Ra\x0CYV[a\x03Ra\x0C\x89V[a\x03\x9Ba\x04\x956`\x04a\x1EFV[a\x0C\xB9V[a\x037a\x0C\xCAV[a\x03Ra\x0C\xF5V[a\x03\x9Ba\x04\xB86`\x04a\x1EdV[a\r%V[a\x03\x9Ba\x04\xCB6`\x04a\x1E\x83V[a\r6V[a\x05\x94`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`iTc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[`@Qa\x03A\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[a\x03Ra\rGV[a\x037`gT\x81V[a\x03\x9Ba\x06#6`\x04a\x1A\x0CV[a\rwV[a\x037`fT\x81V[`hTa\x03\xBE\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x037a\x0E+V[a\x037a\x0EVV[a\x037`\0\x81V[a\x06\x88`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra\x1E\xCEV[\x81V[`\0a\x06\xBFa\x06\xBB`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa\x1E\xCEV[T\x90V[\x90P\x90V[`\0a\x06\xBFa\x06\xBB`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa\x1E\xCEV[a\x06\xFCa\x0E\x81V[a\x07\x05\x81a\x0F\x02V[PV[a\x06\x88`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a\x1E\xCEV[`\0a\x06\xBF\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08T\x90V[`iT`\0\x90a\x06\xBF\x90c\xFF\xFF\xFF\xFFj\x01\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16a\x1E\xE5V[`\0a\x06\xBFa\x06\xBB`\x01\x7F\xE5*f\x7Fq\xECv\x1B\x9B8\x1C{v\xCA\x9B\x85*\xDF~\x89\x05\xDA\x0E\n\xD4\x99\x86\xA0\xA6\x87\x18\x16a\x1E\xCEV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x07\xD5WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x08fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x08\x9Fa\x0F\xBEV[a\x08\xA8\x8Ba\rwV[a\x08\xB1\x88a\x10]V[a\x08\xBB\x8A\x8Aa\x10\x85V[a\x08\xC4\x87a\x11\x16V[a\x08\xCD\x86a\x0F\x02V[a\x08\xFF\x83a\x08\xFC`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a\x1E\xCEV[UV[\x81Qa\t0\x90a\x08\xFC`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a\x1E\xCEV[` \x82\x01Qa\td\x90a\x08\xFC`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a\x1E\xCEV[`@\x82\x01Qa\t\x98\x90a\x08\xFC`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa\x1E\xCEV[``\x82\x01Qa\t\xCC\x90a\x08\xFC`\x01\x7F\xE5*f\x7Fq\xECv\x1B\x9B8\x1C{v\xCA\x9B\x85*\xDF~\x89\x05\xDA\x0E\n\xD4\x99\x86\xA0\xA6\x87\x18\x16a\x1E\xCEV[`\x80\x82\x01Qa\n\0\x90a\x08\xFC`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa\x1E\xCEV[`\xA0\x82\x01Qa\n4\x90a\x08\xFC`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra\x1E\xCEV[a\n=\x84a\x11\xF4V[a\nF\x85a\x12\x96V[a\nNa\x07]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\n\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x08]V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[``a\x0Ba\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17\nV[a\x0B\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17\nV[a\x0B\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17\nV[`@Q` \x01a\x0B\xC5\x93\x92\x91\x90a\x1F\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\x06\x88`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a\x1E\xCEV[a\x06\x88`\x01\x7F\xE5*f\x7Fq\xECv\x1B\x9B8\x1C{v\xCA\x9B\x85*\xDF~\x89\x05\xDA\x0E\n\xD4\x99\x86\xA0\xA6\x87\x18\x16a\x1E\xCEV[a\x0C7a\x0E\x81V[a\x0CA`\0a\x18GV[V[a\x0CKa\x0E\x81V[a\x0CU\x82\x82a\x10\x85V[PPV[`\0a\x06\xBFa\x06\xBB`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra\x1E\xCEV[`\0a\x06\xBFa\x06\xBB`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a\x1E\xCEV[a\x0C\xC1a\x0E\x81V[a\x07\x05\x81a\x11\x16V[a\x06\x88`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a\x1E\xCEV[`\0a\x06\xBFa\x06\xBB`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a\x1E\xCEV[a\r-a\x0E\x81V[a\x07\x05\x81a\x12\x96V[a\r>a\x0E\x81V[a\x07\x05\x81a\x10]V[`\0a\x06\xBFa\x06\xBB`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a\x1E\xCEV[a\r\x7Fa\x0E\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0E\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[a\x07\x05\x81a\x18GV[a\x06\x88`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa\x1E\xCEV[a\x06\x88`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa\x1E\xCEV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0CAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08]V[a\x0F*\x81\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08UV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R`\0\x91\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x03[`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x0F\xB2\x91\x90a\x1E\x0EV[`@Q\x80\x91\x03\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x10UW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[a\x0CAa\x18\xBEV[`g\x81\x90U`@\x80Q` \x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x90\x82\x01\x90\x91R`\0a\x0F\x81V[`e\x82\x90U`f\x81\x90U`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\0\x90``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x01`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x11\t\x91\x90a\x1E\x0EV[`@Q\x80\x91\x03\x90\xA3PPPV[a\x11\x1Ea\x07]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x11\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x08]V[`h\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x02a\x0F\x81V[`jT\x15a\x12\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FSystemConfig: cannot override an`D\x82\x01R\x7F already set start block\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[\x80\x15a\x12\x8FW`jUV[C`jUPV[\x80`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FSystemConfig: min base fee must `D\x82\x01R\x7Fbe less than max base\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[`\x01\x81`@\x01Q`\xFF\x16\x11a\x13\xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: denominator must b`D\x82\x01R\x7Fe larger than 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[`hT`\x80\x82\x01Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91a\x13\xFE\x91\x90a\x1F\x87V[c\xFF\xFF\xFF\xFF\x16\x11\x15a\x14lW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x08]V[`\0\x81` \x01Q`\xFF\x16\x11a\x15\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: elasticity multipl`D\x82\x01R\x7Fier cannot be 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[\x80Q` \x82\x01Qc\xFF\xFF\xFF\xFF\x82\x16\x91`\xFF\x90\x91\x16\x90a\x15#\x90\x82\x90a\x1F\xD5V[a\x15-\x91\x90a\x1F\xF8V[c\xFF\xFF\xFF\xFF\x16\x14a\x15\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FSystemConfig: precision loss wit`D\x82\x01R\x7Fh target resource limit\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[\x80Q`i\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x02\x17\x90UV[``\x81`\0\x03a\x17MWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x17wW\x80a\x17a\x81a $V[\x91Pa\x17p\x90P`\n\x83a \\V[\x91Pa\x17QV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x92Wa\x17\x92a\x1A\xCEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17\xBCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x18?Wa\x17\xD1`\x01\x83a\x1E\xCEV[\x91Pa\x17\xDE`\n\x86a pV[a\x17\xE9\x90`0a \x84V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x17\xFEWa\x17\xFEa \x9CV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x188`\n\x86a \\V[\x94Pa\x17\xC0V[\x94\x93PPPPV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x19UW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[a\x0CA3a\x18GV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A!Wa\x1A!a\x19^V[a\x1A*\x82a\x19\xE3V[\x93\x92PPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1BGW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\x1B\x87Wa\x1B\x87a\x1AIV[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B\xD1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80a\x1B\xE0\x83a\x1BMV[\x81Ra\x1B\xEE` \x84\x01a\x1BaV[` \x82\x01Ra\x1B\xFF`@\x84\x01a\x1BaV[`@\x82\x01Ra\x1C\x10``\x84\x01a\x1BMV[``\x82\x01Ra\x1C!`\x80\x84\x01a\x1BMV[`\x80\x82\x01R`\xA0\x83\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1CIW`\0\x80\xFD[`\xA0\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80\x8A\x8C\x03a\x02\x80\x81\x12\x15a\x1CzWa\x1Cza\x19^V[a\x1C\x83\x8Ca\x19\xE3V[\x9AP` \x8C\x015\x99P`@\x8C\x015\x98P``\x8C\x015\x97Pa\x1C\xA6`\x80\x8D\x01a\x1A1V[\x96Pa\x1C\xB4`\xA0\x8D\x01a\x19\xE3V[\x95Pa\x1C\xC3\x8D`\xC0\x8E\x01a\x1BrV[\x94Pa\x01\x80\x8C\x015\x93Pa\x1C\xDAa\x01\xA0\x8D\x01a\x19\xE3V[\x92P`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE@\x82\x01\x12\x15a\x1D\x0FWa\x1D\x0Fa\x1AIV[Pa\x1D\x18a\x1A\xFDV[a\x1D%a\x01\xC0\x8D\x01a\x19\xE3V[\x81Ra\x1D4a\x01\xE0\x8D\x01a\x19\xE3V[` \x82\x01Ra\x1DFa\x02\0\x8D\x01a\x19\xE3V[`@\x82\x01Ra\x1DXa\x02 \x8D\x01a\x19\xE3V[``\x82\x01Ra\x1Dja\x02@\x8D\x01a\x19\xE3V[`\x80\x82\x01Ra\x1D|a\x02`\x8D\x01a\x19\xE3V[`\xA0\x82\x01R\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0[\x83\x81\x10\x15a\x1D\xAFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D\x97V[\x83\x81\x11\x15a\x1D\xBEW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x1D\xDC\x81` \x86\x01` \x86\x01a\x1D\x94V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1A*` \x83\x01\x84a\x1D\xC4V[`\0\x80`@\x83\x85\x03\x12\x15a\x1E7Wa\x1E7a\x19^V[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x1E[Wa\x1E[a\x19^V[a\x1A*\x82a\x1A1V[`\0`\xC0\x82\x84\x03\x12\x15a\x1EyWa\x1Eya\x19^V[a\x1A*\x83\x83a\x1BrV[`\0` \x82\x84\x03\x12\x15a\x1E\x98Wa\x1E\x98a\x19^V[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1E\xE0Wa\x1E\xE0a\x1E\x9FV[P\x03\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\x08Wa\x1F\x08a\x1E\x9FV[\x01\x94\x93PPPPV[`\0\x84Qa\x1F#\x81\x84` \x89\x01a\x1D\x94V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x1F_\x81`\x01\x85\x01` \x8A\x01a\x1D\x94V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x1Fz\x81`\x02\x84\x01` \x88\x01a\x1D\x94V[\x01`\x02\x01\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\x08Wa\x1F\x08a\x1E\x9FV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x1F\xECWa\x1F\xECa\x1F\xA6V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a \x1BWa \x1Ba\x1E\x9FV[\x02\x94\x93PPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a UWa Ua\x1E\x9FV[P`\x01\x01\x90V[`\0\x82a kWa ka\x1F\xA6V[P\x04\x90V[`\0\x82a \x7FWa \x7Fa\x1F\xA6V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a \x97Wa \x97a\x1E\x9FV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\nSystemConfig: gas limit too low\0\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBEInitializable: contract is not i";
    /// The bytecode of the contract.
    pub static SYSTEMCONFIG_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x02\xA8W`\x005`\xE0\x1C\x80c\x93_\x02\x9E\x11a\x01\xACW\x80c\xCCs\x1B\x02\x11a\x01?W\x80c\xF4^e\xD8\x11a\x01\x0EW\x80c\xF8\xC6\x8D\xE0\x11a\0\xF3W\x80c\xF8\xC6\x8D\xE0\x14a\x06EW\x80c\xFD2\xAA\x0F\x14a\x06MW\x80c\xFF\xA1\xADt\x14a\x06UWa\x02\xA8V[\x80c\xF4^e\xD8\x14a\x06(W\x80c\xF6\x80\x16\xB7\x14a\x061Wa\x02\xA8V[\x80c\xCCs\x1B\x02\x14a\x04\xD0W\x80c\xDA\xC6\xE6:\x14a\x06\x04W\x80c\xE8\x1B,m\x14a\x06\x0CW\x80c\xF2\xFD\xE3\x8B\x14a\x06\x15Wa\x02\xA8V[\x80c\xBCI\xCE_\x11a\x01{W\x80c\xBCI\xCE_\x14a\x04\x9AW\x80c\xC4\xE8\xDD\xFA\x14a\x04\xA2W\x80c\xC7\x19s\xF6\x14a\x04\xAAW\x80c\xC9\xB2oa\x14a\x04\xBDWa\x02\xA8V[\x80c\x93_\x02\x9E\x14a\x04dW\x80c\x9B}\x7F\n\x14a\x04wW\x80c\xA7\x11\x98i\x14a\x04\x7FW\x80c\xB4\n\x81|\x14a\x04\x87Wa\x02\xA8V[\x80cJ\xDD2\x1D\x11a\x02?W\x80cT\xFDMP\x11a\x02\x0EW\x80ca\xD1Wh\x11a\x01\xF3W\x80ca\xD1Wh\x14a\x046W\x80cqP\x18\xA6\x14a\x04>W\x80c\x8D\xA5\xCB[\x14a\x04FWa\x02\xA8V[\x80cT\xFDMP\x14a\x04\x19W\x80c]s6\x9C\x14a\x04.Wa\x02\xA8V[\x80cJ\xDD2\x1D\x14a\x03\xB6W\x80cM\x9F\x15Y\x14a\x03\xD7W\x80cO\x16T\x0B\x14a\x03\xDFW\x80cR(\xA6\xAC\x14a\x04\x06Wa\x02\xA8V[\x80c\x18\xD19\x18\x11a\x02{W\x80c\x18\xD19\x18\x14a\x03\x88W\x80c\x19\xF5\xCE\xA8\x14a\x03\x9DW\x80c\x1F\xD1\x9E\xE1\x14a\x03\xA5W\x80cH\xCDL\xB1\x14a\x03\xADWa\x02\xA8V[\x80c\x06\xC9&W\x14a\x03/W\x80c\x07\x8F)\xCF\x14a\x03JW\x80c\nI\xCB\x03\x14a\x03wW\x80c\x0C\x18\xC1b\x14a\x03\x7FW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x037a\x06]V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Ra\x06\x8BV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03AV[a\x03Ra\x06\xC4V[a\x037`eT\x81V[a\x03\x9Ba\x03\x966`\x04a\x1A\x0CV[a\x06\xF4V[\0[a\x037a\x07\x08V[a\x03Ra\x073V[a\x037`jT\x81V[a\x03\xBEa\x07]V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03AV[a\x03Ra\x07\x83V[a\x037\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08\x81V[a\x03\x9Ba\x04\x146`\x04a\x1CVV[a\x07\xB3V[a\x04!a\x0B6V[`@Qa\x03A\x91\x90a\x1E\x0EV[a\x037a\x0B\xD9V[a\x037a\x0C\x04V[a\x03\x9Ba\x0C/V[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03RV[a\x03\x9Ba\x04r6`\x04a\x1E!V[a\x0CCV[a\x03Ra\x0CYV[a\x03Ra\x0C\x89V[a\x03\x9Ba\x04\x956`\x04a\x1EFV[a\x0C\xB9V[a\x037a\x0C\xCAV[a\x03Ra\x0C\xF5V[a\x03\x9Ba\x04\xB86`\x04a\x1EdV[a\r%V[a\x03\x9Ba\x04\xCB6`\x04a\x1E\x83V[a\r6V[a\x05\x94`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`iTc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[`@Qa\x03A\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[a\x03Ra\rGV[a\x037`gT\x81V[a\x03\x9Ba\x06#6`\x04a\x1A\x0CV[a\rwV[a\x037`fT\x81V[`hTa\x03\xBE\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x037a\x0E+V[a\x037a\x0EVV[a\x037`\0\x81V[a\x06\x88`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra\x1E\xCEV[\x81V[`\0a\x06\xBFa\x06\xBB`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa\x1E\xCEV[T\x90V[\x90P\x90V[`\0a\x06\xBFa\x06\xBB`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa\x1E\xCEV[a\x06\xFCa\x0E\x81V[a\x07\x05\x81a\x0F\x02V[PV[a\x06\x88`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a\x1E\xCEV[`\0a\x06\xBF\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08T\x90V[`iT`\0\x90a\x06\xBF\x90c\xFF\xFF\xFF\xFFj\x01\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91\x16a\x1E\xE5V[`\0a\x06\xBFa\x06\xBB`\x01\x7F\xE5*f\x7Fq\xECv\x1B\x9B8\x1C{v\xCA\x9B\x85*\xDF~\x89\x05\xDA\x0E\n\xD4\x99\x86\xA0\xA6\x87\x18\x16a\x1E\xCEV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x07\xD5WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x08fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\x08\x9Fa\x0F\xBEV[a\x08\xA8\x8Ba\rwV[a\x08\xB1\x88a\x10]V[a\x08\xBB\x8A\x8Aa\x10\x85V[a\x08\xC4\x87a\x11\x16V[a\x08\xCD\x86a\x0F\x02V[a\x08\xFF\x83a\x08\xFC`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a\x1E\xCEV[UV[\x81Qa\t0\x90a\x08\xFC`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a\x1E\xCEV[` \x82\x01Qa\td\x90a\x08\xFC`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a\x1E\xCEV[`@\x82\x01Qa\t\x98\x90a\x08\xFC`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa\x1E\xCEV[``\x82\x01Qa\t\xCC\x90a\x08\xFC`\x01\x7F\xE5*f\x7Fq\xECv\x1B\x9B8\x1C{v\xCA\x9B\x85*\xDF~\x89\x05\xDA\x0E\n\xD4\x99\x86\xA0\xA6\x87\x18\x16a\x1E\xCEV[`\x80\x82\x01Qa\n\0\x90a\x08\xFC`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa\x1E\xCEV[`\xA0\x82\x01Qa\n4\x90a\x08\xFC`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra\x1E\xCEV[a\n=\x84a\x11\xF4V[a\nF\x85a\x12\x96V[a\nNa\x07]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\n\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x08]V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[``a\x0Ba\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17\nV[a\x0B\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17\nV[a\x0B\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17\nV[`@Q` \x01a\x0B\xC5\x93\x92\x91\x90a\x1F\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\x06\x88`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a\x1E\xCEV[a\x06\x88`\x01\x7F\xE5*f\x7Fq\xECv\x1B\x9B8\x1C{v\xCA\x9B\x85*\xDF~\x89\x05\xDA\x0E\n\xD4\x99\x86\xA0\xA6\x87\x18\x16a\x1E\xCEV[a\x0C7a\x0E\x81V[a\x0CA`\0a\x18GV[V[a\x0CKa\x0E\x81V[a\x0CU\x82\x82a\x10\x85V[PPV[`\0a\x06\xBFa\x06\xBB`\x01\x7F\xA0L[\xB98\xCAo\xC4m\x95U:\xBF\nv4\\\xE3\xE7\"\xA3\x0B\xF4\xF7I(\xB8\xE7\xD8R2\ra\x1E\xCEV[`\0a\x06\xBFa\x06\xBB`\x01\x7F8?)\x18\x19\xE6\xD5@s\xBC\x9Ad\x82Q\xD9t!\x07k\xDD\x10\x193\xC0\xC0\"!\x9C\xE9X\x067a\x1E\xCEV[a\x0C\xC1a\x0E\x81V[a\x07\x05\x81a\x11\x16V[a\x06\x88`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a\x1E\xCEV[`\0a\x06\xBFa\x06\xBB`\x01\x7FF\xAD\xCB\xEB\xC6\xBE\x8C\xE5Qt\x0C)\xC4|\x87\x98!\x0F#\xF7\xF4\x08lAu)D5%h\xD5\xA8a\x1E\xCEV[a\r-a\x0E\x81V[a\x07\x05\x81a\x12\x96V[a\r>a\x0E\x81V[a\x07\x05\x81a\x10]V[`\0a\x06\xBFa\x06\xBB`\x01\x7Fq\xAC\x12\x82\x9Df\xEEs\xD8\xD9[\xFFP\xB3X\x97E\xCEW\xED\xAEp\xA3\xFB\x11\x1A#BFM\xC5\x98a\x1E\xCEV[a\r\x7Fa\x0E\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0E\"W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[a\x07\x05\x81a\x18GV[a\x06\x88`\x01\x7F\x99\x04\xBA\x90\xDD\xE5il\xDA\x05\xC9\xE0\xDA\xB5\xCB\xAA\x0F\xEA\0Z\xCEM\x11!\x8A\x02\xACf\x8D\xADcwa\x1E\xCEV[a\x06\x88`\x01\x7FKlt\xF9\xE6\x88\xCB9\x80\x1F!\x12\xC1J\x8CW#*?\xC5 .\x14D\x12mK\xCE\x86\xEB\x19\xADa\x1E\xCEV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0CAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08]V[a\x0F*\x81\x7Fe\xA7\xEDT/\xB3\x7F\xE27\xFD\xFB\xDDp\xB3\x15\x98R?\xE5\xB3(y\xE3\x07\xBA\xE2z\x0B\xD9X\x1C\x08UV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R`\0\x91\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x03[`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x0F\xB2\x91\x90a\x1E\x0EV[`@Q\x80\x91\x03\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x10UW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[a\x0CAa\x18\xBEV[`g\x81\x90U`@\x80Q` \x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x90\x82\x01\x90\x91R`\0a\x0F\x81V[`e\x82\x90U`f\x81\x90U`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\0\x90``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x01`\0\x7F\x1D+\x0B\xDA!\xD5k\x8B\xD1-O\x94\xEB\xAC\xFF\xDF\xB3_^\"o\x84\xB4a\x10;\xB8\xBE\xABcS\xBE\x83`@Qa\x11\t\x91\x90a\x1E\x0EV[`@Q\x80\x91\x03\x90\xA3PPPV[a\x11\x1Ea\x07]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x11\x9BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x08]V[`h\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@\x80Q` \x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R\x81\x01\x90R`\x02a\x0F\x81V[`jT\x15a\x12\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FSystemConfig: cannot override an`D\x82\x01R\x7F already set start block\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[\x80\x15a\x12\x8FW`jUV[C`jUPV[\x80`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FSystemConfig: min base fee must `D\x82\x01R\x7Fbe less than max base\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[`\x01\x81`@\x01Q`\xFF\x16\x11a\x13\xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: denominator must b`D\x82\x01R\x7Fe larger than 1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[`hT`\x80\x82\x01Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91a\x13\xFE\x91\x90a\x1F\x87V[c\xFF\xFF\xFF\xFF\x16\x11\x15a\x14lW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSystemConfig: gas limit too low\0`D\x82\x01R`d\x01a\x08]V[`\0\x81` \x01Q`\xFF\x16\x11a\x15\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FSystemConfig: elasticity multipl`D\x82\x01R\x7Fier cannot be 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[\x80Q` \x82\x01Qc\xFF\xFF\xFF\xFF\x82\x16\x91`\xFF\x90\x91\x16\x90a\x15#\x90\x82\x90a\x1F\xD5V[a\x15-\x91\x90a\x1F\xF8V[c\xFF\xFF\xFF\xFF\x16\x14a\x15\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FSystemConfig: precision loss wit`D\x82\x01R\x7Fh target resource limit\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[\x80Q`i\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x02\x17\x90UV[``\x81`\0\x03a\x17MWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x17wW\x80a\x17a\x81a $V[\x91Pa\x17p\x90P`\n\x83a \\V[\x91Pa\x17QV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x92Wa\x17\x92a\x1A\xCEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17\xBCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x18?Wa\x17\xD1`\x01\x83a\x1E\xCEV[\x91Pa\x17\xDE`\n\x86a pV[a\x17\xE9\x90`0a \x84V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x17\xFEWa\x17\xFEa \x9CV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x188`\n\x86a \\V[\x94Pa\x17\xC0V[\x94\x93PPPPV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x19UW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08]V[a\x0CA3a\x18GV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A!Wa\x1A!a\x19^V[a\x1A*\x82a\x19\xE3V[\x93\x92PPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1BGW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x1A\x07W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\x1B\x87Wa\x1B\x87a\x1AIV[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B\xD1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80a\x1B\xE0\x83a\x1BMV[\x81Ra\x1B\xEE` \x84\x01a\x1BaV[` \x82\x01Ra\x1B\xFF`@\x84\x01a\x1BaV[`@\x82\x01Ra\x1C\x10``\x84\x01a\x1BMV[``\x82\x01Ra\x1C!`\x80\x84\x01a\x1BMV[`\x80\x82\x01R`\xA0\x83\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1CIW`\0\x80\xFD[`\xA0\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80\x8A\x8C\x03a\x02\x80\x81\x12\x15a\x1CzWa\x1Cza\x19^V[a\x1C\x83\x8Ca\x19\xE3V[\x9AP` \x8C\x015\x99P`@\x8C\x015\x98P``\x8C\x015\x97Pa\x1C\xA6`\x80\x8D\x01a\x1A1V[\x96Pa\x1C\xB4`\xA0\x8D\x01a\x19\xE3V[\x95Pa\x1C\xC3\x8D`\xC0\x8E\x01a\x1BrV[\x94Pa\x01\x80\x8C\x015\x93Pa\x1C\xDAa\x01\xA0\x8D\x01a\x19\xE3V[\x92P`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE@\x82\x01\x12\x15a\x1D\x0FWa\x1D\x0Fa\x1AIV[Pa\x1D\x18a\x1A\xFDV[a\x1D%a\x01\xC0\x8D\x01a\x19\xE3V[\x81Ra\x1D4a\x01\xE0\x8D\x01a\x19\xE3V[` \x82\x01Ra\x1DFa\x02\0\x8D\x01a\x19\xE3V[`@\x82\x01Ra\x1DXa\x02 \x8D\x01a\x19\xE3V[``\x82\x01Ra\x1Dja\x02@\x8D\x01a\x19\xE3V[`\x80\x82\x01Ra\x1D|a\x02`\x8D\x01a\x19\xE3V[`\xA0\x82\x01R\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0[\x83\x81\x10\x15a\x1D\xAFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D\x97V[\x83\x81\x11\x15a\x1D\xBEW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x1D\xDC\x81` \x86\x01` \x86\x01a\x1D\x94V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1A*` \x83\x01\x84a\x1D\xC4V[`\0\x80`@\x83\x85\x03\x12\x15a\x1E7Wa\x1E7a\x19^V[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x1E[Wa\x1E[a\x19^V[a\x1A*\x82a\x1A1V[`\0`\xC0\x82\x84\x03\x12\x15a\x1EyWa\x1Eya\x19^V[a\x1A*\x83\x83a\x1BrV[`\0` \x82\x84\x03\x12\x15a\x1E\x98Wa\x1E\x98a\x19^V[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1E\xE0Wa\x1E\xE0a\x1E\x9FV[P\x03\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\x08Wa\x1F\x08a\x1E\x9FV[\x01\x94\x93PPPPV[`\0\x84Qa\x1F#\x81\x84` \x89\x01a\x1D\x94V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x1F_\x81`\x01\x85\x01` \x8A\x01a\x1D\x94V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x1Fz\x81`\x02\x84\x01` \x88\x01a\x1D\x94V[\x01`\x02\x01\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\x08Wa\x1F\x08a\x1E\x9FV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x1F\xECWa\x1F\xECa\x1F\xA6V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a \x1BWa \x1Ba\x1E\x9FV[\x02\x94\x93PPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a UWa Ua\x1E\x9FV[P`\x01\x01\x90V[`\0\x82a kWa ka\x1F\xA6V[P\x04\x90V[`\0\x82a \x7FWa \x7Fa\x1F\xA6V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a \x97Wa \x97a\x1E\x9FV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static SYSTEMCONFIG_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SystemConfig<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SystemConfig<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SystemConfig<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SystemConfig<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SystemConfig<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SystemConfig))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SystemConfig<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SYSTEMCONFIG_ABI.clone(),
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
                SYSTEMCONFIG_ABI.clone(),
                SYSTEMCONFIG_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BATCH_INBOX_SLOT` (0xbc49ce5f) function
        pub fn batch_inbox_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([188, 73, 206, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `L1_CROSS_DOMAIN_MESSENGER_SLOT` (0x5d73369c) function
        pub fn l1_cross_domain_messenger_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([93, 115, 54, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `L1_ERC_721_BRIDGE_SLOT` (0x19f5cea8) function
        pub fn l1_erc_721_bridge_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([25, 245, 206, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `L1_STANDARD_BRIDGE_SLOT` (0xf8c68de0) function
        pub fn l1_standard_bridge_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([248, 198, 141, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `L2_OUTPUT_ORACLE_SLOT` (0x61d15768) function
        pub fn l2_output_oracle_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([97, 209, 87, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT` (0x06c92657) function
        pub fn optimism_mintable_erc20_factory_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([6, 201, 38, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMISM_PORTAL_SLOT` (0xfd32aa0f) function
        pub fn optimism_portal_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([253, 50, 170, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNSAFE_BLOCK_SIGNER_SLOT` (0x4f16540b) function
        pub fn unsafe_block_signer_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([79, 22, 84, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn VERSION(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchInbox` (0xdac6e63a) function
        pub fn batch_inbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([218, 198, 230, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batcherHash` (0xe81b2c6d) function
        pub fn batcher_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 27, 44, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gasLimit` (0xf68016b7) function
        pub fn gas_limit(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([246, 128, 22, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x5228a6ac) function
        pub fn initialize(
            &self,
            owner: ::ethers::core::types::Address,
            overhead: ::ethers::core::types::U256,
            scalar: ::ethers::core::types::U256,
            batcher_hash: [u8; 32],
            gas_limit: u64,
            unsafe_block_signer: ::ethers::core::types::Address,
            config: ResourceConfig,
            start_block: ::ethers::core::types::U256,
            batch_inbox: ::ethers::core::types::Address,
            addresses: Addresses,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [82, 40, 166, 172],
                    (
                        owner,
                        overhead,
                        scalar,
                        batcher_hash,
                        gas_limit,
                        unsafe_block_signer,
                        config,
                        start_block,
                        batch_inbox,
                        addresses,
                    ),
                )
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
        ///Calls the contract's `l1ERC721Bridge` (0xc4e8ddfa) function
        pub fn l_1erc721_bridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 232, 221, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1StandardBridge` (0x078f29cf) function
        pub fn l_1_standard_bridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([7, 143, 41, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2OutputOracle` (0x4d9f1559) function
        pub fn l_2_output_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([77, 159, 21, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumGasLimit` (0x4add321d) function
        pub fn minimum_gas_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([74, 221, 50, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `optimismMintableERC20Factory` (0x9b7d7f0a) function
        pub fn optimism_mintable_erc20_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([155, 125, 127, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `optimismPortal` (0x0a49cb03) function
        pub fn optimism_portal(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([10, 73, 203, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `overhead` (0x0c18c162) function
        pub fn overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 24, 193, 98], ())
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resourceConfig` (0xcc731b02) function
        pub fn resource_config(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ResourceConfig> {
            self.0
                .method_hash([204, 115, 27, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `scalar` (0xf45e65d8) function
        pub fn scalar(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 94, 101, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBatcherHash` (0xc9b26f61) function
        pub fn set_batcher_hash(
            &self,
            batcher_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 178, 111, 97], batcher_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGasConfig` (0x935f029e) function
        pub fn set_gas_config(
            &self,
            overhead: ::ethers::core::types::U256,
            scalar: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 95, 2, 158], (overhead, scalar))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGasLimit` (0xb40a817c) function
        pub fn set_gas_limit(
            &self,
            gas_limit: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 10, 129, 124], gas_limit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setResourceConfig` (0xc71973f6) function
        pub fn set_resource_config(
            &self,
            config: ResourceConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 25, 115, 246], (config,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUnsafeBlockSigner` (0x18d13918) function
        pub fn set_unsafe_block_signer(
            &self,
            unsafe_block_signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 209, 57, 24], unsafe_block_signer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBlock` (0x48cd4cb1) function
        pub fn start_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([72, 205, 76, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsafeBlockSigner` (0x1fd19ee1) function
        pub fn unsafe_block_signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([31, 209, 158, 225], ())
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
        ///Gets the contract's `ConfigUpdate` event
        pub fn config_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ConfigUpdateFilter,
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SystemConfigEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SystemConfig<M> {
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
    #[ethevent(name = "ConfigUpdate", abi = "ConfigUpdate(uint256,uint8,bytes)")]
    pub struct ConfigUpdateFilter {
        #[ethevent(indexed)]
        pub version: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub update_type: u8,
        pub data: ::ethers::core::types::Bytes,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SystemConfigEvents {
        ConfigUpdateFilter(ConfigUpdateFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for SystemConfigEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ConfigUpdateFilter::decode_log(log) {
                return Ok(SystemConfigEvents::ConfigUpdateFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SystemConfigEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(SystemConfigEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SystemConfigEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConfigUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ConfigUpdateFilter> for SystemConfigEvents {
        fn from(value: ConfigUpdateFilter) -> Self {
            Self::ConfigUpdateFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for SystemConfigEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for SystemConfigEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `BATCH_INBOX_SLOT` function with signature `BATCH_INBOX_SLOT()` and selector `0xbc49ce5f`
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
    #[ethcall(name = "BATCH_INBOX_SLOT", abi = "BATCH_INBOX_SLOT()")]
    pub struct BatchInboxSlotCall;
    ///Container type for all input parameters for the `L1_CROSS_DOMAIN_MESSENGER_SLOT` function with signature `L1_CROSS_DOMAIN_MESSENGER_SLOT()` and selector `0x5d73369c`
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
        name = "L1_CROSS_DOMAIN_MESSENGER_SLOT",
        abi = "L1_CROSS_DOMAIN_MESSENGER_SLOT()"
    )]
    pub struct L1CrossDomainMessengerSlotCall;
    ///Container type for all input parameters for the `L1_ERC_721_BRIDGE_SLOT` function with signature `L1_ERC_721_BRIDGE_SLOT()` and selector `0x19f5cea8`
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
    #[ethcall(name = "L1_ERC_721_BRIDGE_SLOT", abi = "L1_ERC_721_BRIDGE_SLOT()")]
    pub struct L1Erc721BridgeSlotCall;
    ///Container type for all input parameters for the `L1_STANDARD_BRIDGE_SLOT` function with signature `L1_STANDARD_BRIDGE_SLOT()` and selector `0xf8c68de0`
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
    #[ethcall(name = "L1_STANDARD_BRIDGE_SLOT", abi = "L1_STANDARD_BRIDGE_SLOT()")]
    pub struct L1StandardBridgeSlotCall;
    ///Container type for all input parameters for the `L2_OUTPUT_ORACLE_SLOT` function with signature `L2_OUTPUT_ORACLE_SLOT()` and selector `0x61d15768`
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
    #[ethcall(name = "L2_OUTPUT_ORACLE_SLOT", abi = "L2_OUTPUT_ORACLE_SLOT()")]
    pub struct L2OutputOracleSlotCall;
    ///Container type for all input parameters for the `OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT` function with signature `OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT()` and selector `0x06c92657`
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
        name = "OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT",
        abi = "OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT()"
    )]
    pub struct OptimismMintableErc20FactorySlotCall;
    ///Container type for all input parameters for the `OPTIMISM_PORTAL_SLOT` function with signature `OPTIMISM_PORTAL_SLOT()` and selector `0xfd32aa0f`
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
    #[ethcall(name = "OPTIMISM_PORTAL_SLOT", abi = "OPTIMISM_PORTAL_SLOT()")]
    pub struct OptimismPortalSlotCall;
    ///Container type for all input parameters for the `UNSAFE_BLOCK_SIGNER_SLOT` function with signature `UNSAFE_BLOCK_SIGNER_SLOT()` and selector `0x4f16540b`
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
    #[ethcall(name = "UNSAFE_BLOCK_SIGNER_SLOT", abi = "UNSAFE_BLOCK_SIGNER_SLOT()")]
    pub struct UnsafeBlockSignerSlotCall;
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
    pub struct VERSIONCall;
    ///Container type for all input parameters for the `batchInbox` function with signature `batchInbox()` and selector `0xdac6e63a`
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
    #[ethcall(name = "batchInbox", abi = "batchInbox()")]
    pub struct BatchInboxCall;
    ///Container type for all input parameters for the `batcherHash` function with signature `batcherHash()` and selector `0xe81b2c6d`
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
    #[ethcall(name = "batcherHash", abi = "batcherHash()")]
    pub struct BatcherHashCall;
    ///Container type for all input parameters for the `gasLimit` function with signature `gasLimit()` and selector `0xf68016b7`
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
    #[ethcall(name = "gasLimit", abi = "gasLimit()")]
    pub struct GasLimitCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,uint256,uint256,bytes32,uint64,address,(uint32,uint8,uint8,uint32,uint32,uint128),uint256,address,(address,address,address,address,address,address))` and selector `0x5228a6ac`
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
        name = "initialize",
        abi = "initialize(address,uint256,uint256,bytes32,uint64,address,(uint32,uint8,uint8,uint32,uint32,uint128),uint256,address,(address,address,address,address,address,address))"
    )]
    pub struct InitializeCall {
        pub owner: ::ethers::core::types::Address,
        pub overhead: ::ethers::core::types::U256,
        pub scalar: ::ethers::core::types::U256,
        pub batcher_hash: [u8; 32],
        pub gas_limit: u64,
        pub unsafe_block_signer: ::ethers::core::types::Address,
        pub config: ResourceConfig,
        pub start_block: ::ethers::core::types::U256,
        pub batch_inbox: ::ethers::core::types::Address,
        pub addresses: Addresses,
    }
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
    ///Container type for all input parameters for the `l1ERC721Bridge` function with signature `l1ERC721Bridge()` and selector `0xc4e8ddfa`
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
    #[ethcall(name = "l1ERC721Bridge", abi = "l1ERC721Bridge()")]
    pub struct L1Erc721BridgeCall;
    ///Container type for all input parameters for the `l1StandardBridge` function with signature `l1StandardBridge()` and selector `0x078f29cf`
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
    #[ethcall(name = "l1StandardBridge", abi = "l1StandardBridge()")]
    pub struct L1StandardBridgeCall;
    ///Container type for all input parameters for the `l2OutputOracle` function with signature `l2OutputOracle()` and selector `0x4d9f1559`
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
    #[ethcall(name = "l2OutputOracle", abi = "l2OutputOracle()")]
    pub struct L2OutputOracleCall;
    ///Container type for all input parameters for the `minimumGasLimit` function with signature `minimumGasLimit()` and selector `0x4add321d`
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
    #[ethcall(name = "minimumGasLimit", abi = "minimumGasLimit()")]
    pub struct MinimumGasLimitCall;
    ///Container type for all input parameters for the `optimismMintableERC20Factory` function with signature `optimismMintableERC20Factory()` and selector `0x9b7d7f0a`
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
        name = "optimismMintableERC20Factory",
        abi = "optimismMintableERC20Factory()"
    )]
    pub struct OptimismMintableERC20FactoryCall;
    ///Container type for all input parameters for the `optimismPortal` function with signature `optimismPortal()` and selector `0x0a49cb03`
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
    #[ethcall(name = "optimismPortal", abi = "optimismPortal()")]
    pub struct OptimismPortalCall;
    ///Container type for all input parameters for the `overhead` function with signature `overhead()` and selector `0x0c18c162`
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
    #[ethcall(name = "overhead", abi = "overhead()")]
    pub struct OverheadCall;
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `resourceConfig` function with signature `resourceConfig()` and selector `0xcc731b02`
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
    #[ethcall(name = "resourceConfig", abi = "resourceConfig()")]
    pub struct ResourceConfigCall;
    ///Container type for all input parameters for the `scalar` function with signature `scalar()` and selector `0xf45e65d8`
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
    #[ethcall(name = "scalar", abi = "scalar()")]
    pub struct ScalarCall;
    ///Container type for all input parameters for the `setBatcherHash` function with signature `setBatcherHash(bytes32)` and selector `0xc9b26f61`
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
    #[ethcall(name = "setBatcherHash", abi = "setBatcherHash(bytes32)")]
    pub struct SetBatcherHashCall {
        pub batcher_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `setGasConfig` function with signature `setGasConfig(uint256,uint256)` and selector `0x935f029e`
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
    #[ethcall(name = "setGasConfig", abi = "setGasConfig(uint256,uint256)")]
    pub struct SetGasConfigCall {
        pub overhead: ::ethers::core::types::U256,
        pub scalar: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setGasLimit` function with signature `setGasLimit(uint64)` and selector `0xb40a817c`
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
    #[ethcall(name = "setGasLimit", abi = "setGasLimit(uint64)")]
    pub struct SetGasLimitCall {
        pub gas_limit: u64,
    }
    ///Container type for all input parameters for the `setResourceConfig` function with signature `setResourceConfig((uint32,uint8,uint8,uint32,uint32,uint128))` and selector `0xc71973f6`
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
        name = "setResourceConfig",
        abi = "setResourceConfig((uint32,uint8,uint8,uint32,uint32,uint128))"
    )]
    pub struct SetResourceConfigCall {
        pub config: ResourceConfig,
    }
    ///Container type for all input parameters for the `setUnsafeBlockSigner` function with signature `setUnsafeBlockSigner(address)` and selector `0x18d13918`
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
    #[ethcall(name = "setUnsafeBlockSigner", abi = "setUnsafeBlockSigner(address)")]
    pub struct SetUnsafeBlockSignerCall {
        pub unsafe_block_signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `startBlock` function with signature `startBlock()` and selector `0x48cd4cb1`
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
    #[ethcall(name = "startBlock", abi = "startBlock()")]
    pub struct StartBlockCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unsafeBlockSigner` function with signature `unsafeBlockSigner()` and selector `0x1fd19ee1`
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
    #[ethcall(name = "unsafeBlockSigner", abi = "unsafeBlockSigner()")]
    pub struct UnsafeBlockSignerCall;
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
    pub struct versionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SystemConfigCalls {
        BatchInboxSlot(BatchInboxSlotCall),
        L1CrossDomainMessengerSlot(L1CrossDomainMessengerSlotCall),
        L1Erc721BridgeSlot(L1Erc721BridgeSlotCall),
        L1StandardBridgeSlot(L1StandardBridgeSlotCall),
        L2OutputOracleSlot(L2OutputOracleSlotCall),
        OptimismMintableErc20FactorySlot(OptimismMintableErc20FactorySlotCall),
        OptimismPortalSlot(OptimismPortalSlotCall),
        UnsafeBlockSignerSlot(UnsafeBlockSignerSlotCall),
        VERSION(VERSIONCall),
        BatchInbox(BatchInboxCall),
        BatcherHash(BatcherHashCall),
        GasLimit(GasLimitCall),
        Initialize(InitializeCall),
        L1CrossDomainMessenger(L1CrossDomainMessengerCall),
        L1Erc721Bridge(L1Erc721BridgeCall),
        L1StandardBridge(L1StandardBridgeCall),
        L2OutputOracle(L2OutputOracleCall),
        MinimumGasLimit(MinimumGasLimitCall),
        OptimismMintableERC20Factory(OptimismMintableERC20FactoryCall),
        OptimismPortal(OptimismPortalCall),
        Overhead(OverheadCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        ResourceConfig(ResourceConfigCall),
        Scalar(ScalarCall),
        SetBatcherHash(SetBatcherHashCall),
        SetGasConfig(SetGasConfigCall),
        SetGasLimit(SetGasLimitCall),
        SetResourceConfig(SetResourceConfigCall),
        SetUnsafeBlockSigner(SetUnsafeBlockSignerCall),
        StartBlock(StartBlockCall),
        TransferOwnership(TransferOwnershipCall),
        UnsafeBlockSigner(UnsafeBlockSignerCall),
        version(versionCall),
    }
    impl ::ethers::core::abi::AbiDecode for SystemConfigCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BatchInboxSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchInboxSlot(decoded));
            }
            if let Ok(decoded) = <L1CrossDomainMessengerSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1CrossDomainMessengerSlot(decoded));
            }
            if let Ok(decoded) = <L1Erc721BridgeSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1Erc721BridgeSlot(decoded));
            }
            if let Ok(decoded) = <L1StandardBridgeSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1StandardBridgeSlot(decoded));
            }
            if let Ok(decoded) = <L2OutputOracleSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2OutputOracleSlot(decoded));
            }
            if let Ok(decoded) = <OptimismMintableErc20FactorySlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimismMintableErc20FactorySlot(decoded));
            }
            if let Ok(decoded) = <OptimismPortalSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimismPortalSlot(decoded));
            }
            if let Ok(decoded) = <UnsafeBlockSignerSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeBlockSignerSlot(decoded));
            }
            if let Ok(decoded) = <VERSIONCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VERSION(decoded));
            }
            if let Ok(decoded) = <BatchInboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchInbox(decoded));
            }
            if let Ok(decoded) = <BatcherHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatcherHash(decoded));
            }
            if let Ok(decoded) = <GasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GasLimit(decoded));
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
            if let Ok(decoded) = <L1Erc721BridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1Erc721Bridge(decoded));
            }
            if let Ok(decoded) = <L1StandardBridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1StandardBridge(decoded));
            }
            if let Ok(decoded) = <L2OutputOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2OutputOracle(decoded));
            }
            if let Ok(decoded) = <MinimumGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumGasLimit(decoded));
            }
            if let Ok(decoded) = <OptimismMintableERC20FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimismMintableERC20Factory(decoded));
            }
            if let Ok(decoded) = <OptimismPortalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimismPortal(decoded));
            }
            if let Ok(decoded) = <OverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Overhead(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <ResourceConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResourceConfig(decoded));
            }
            if let Ok(decoded) = <ScalarCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Scalar(decoded));
            }
            if let Ok(decoded) = <SetBatcherHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBatcherHash(decoded));
            }
            if let Ok(decoded) = <SetGasConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetGasConfig(decoded));
            }
            if let Ok(decoded) = <SetGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetGasLimit(decoded));
            }
            if let Ok(decoded) = <SetResourceConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetResourceConfig(decoded));
            }
            if let Ok(decoded) = <SetUnsafeBlockSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUnsafeBlockSigner(decoded));
            }
            if let Ok(decoded) = <StartBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBlock(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnsafeBlockSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeBlockSigner(decoded));
            }
            if let Ok(decoded) = <versionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SystemConfigCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BatchInboxSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1CrossDomainMessengerSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1Erc721BridgeSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1StandardBridgeSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2OutputOracleSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimismMintableErc20FactorySlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimismPortalSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeBlockSignerSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VERSION(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BatchInbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatcherHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1CrossDomainMessenger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1Erc721Bridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1StandardBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2OutputOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimismMintableERC20Factory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimismPortal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Overhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResourceConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Scalar(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBatcherHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGasConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetResourceConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUnsafeBlockSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeBlockSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SystemConfigCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BatchInboxSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1CrossDomainMessengerSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L1Erc721BridgeSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L1StandardBridgeSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2OutputOracleSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimismMintableErc20FactorySlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimismPortalSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsafeBlockSignerSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VERSION(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchInbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatcherHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1CrossDomainMessenger(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L1Erc721Bridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1StandardBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2OutputOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumGasLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::OptimismMintableERC20Factory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimismPortal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Overhead(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResourceConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Scalar(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBatcherHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGasConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGasLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetResourceConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUnsafeBlockSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeBlockSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BatchInboxSlotCall> for SystemConfigCalls {
        fn from(value: BatchInboxSlotCall) -> Self {
            Self::BatchInboxSlot(value)
        }
    }
    impl ::core::convert::From<L1CrossDomainMessengerSlotCall> for SystemConfigCalls {
        fn from(value: L1CrossDomainMessengerSlotCall) -> Self {
            Self::L1CrossDomainMessengerSlot(value)
        }
    }
    impl ::core::convert::From<L1Erc721BridgeSlotCall> for SystemConfigCalls {
        fn from(value: L1Erc721BridgeSlotCall) -> Self {
            Self::L1Erc721BridgeSlot(value)
        }
    }
    impl ::core::convert::From<L1StandardBridgeSlotCall> for SystemConfigCalls {
        fn from(value: L1StandardBridgeSlotCall) -> Self {
            Self::L1StandardBridgeSlot(value)
        }
    }
    impl ::core::convert::From<L2OutputOracleSlotCall> for SystemConfigCalls {
        fn from(value: L2OutputOracleSlotCall) -> Self {
            Self::L2OutputOracleSlot(value)
        }
    }
    impl ::core::convert::From<OptimismMintableErc20FactorySlotCall>
    for SystemConfigCalls {
        fn from(value: OptimismMintableErc20FactorySlotCall) -> Self {
            Self::OptimismMintableErc20FactorySlot(value)
        }
    }
    impl ::core::convert::From<OptimismPortalSlotCall> for SystemConfigCalls {
        fn from(value: OptimismPortalSlotCall) -> Self {
            Self::OptimismPortalSlot(value)
        }
    }
    impl ::core::convert::From<UnsafeBlockSignerSlotCall> for SystemConfigCalls {
        fn from(value: UnsafeBlockSignerSlotCall) -> Self {
            Self::UnsafeBlockSignerSlot(value)
        }
    }
    impl ::core::convert::From<VERSIONCall> for SystemConfigCalls {
        fn from(value: VERSIONCall) -> Self {
            Self::VERSION(value)
        }
    }
    impl ::core::convert::From<BatchInboxCall> for SystemConfigCalls {
        fn from(value: BatchInboxCall) -> Self {
            Self::BatchInbox(value)
        }
    }
    impl ::core::convert::From<BatcherHashCall> for SystemConfigCalls {
        fn from(value: BatcherHashCall) -> Self {
            Self::BatcherHash(value)
        }
    }
    impl ::core::convert::From<GasLimitCall> for SystemConfigCalls {
        fn from(value: GasLimitCall) -> Self {
            Self::GasLimit(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SystemConfigCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<L1CrossDomainMessengerCall> for SystemConfigCalls {
        fn from(value: L1CrossDomainMessengerCall) -> Self {
            Self::L1CrossDomainMessenger(value)
        }
    }
    impl ::core::convert::From<L1Erc721BridgeCall> for SystemConfigCalls {
        fn from(value: L1Erc721BridgeCall) -> Self {
            Self::L1Erc721Bridge(value)
        }
    }
    impl ::core::convert::From<L1StandardBridgeCall> for SystemConfigCalls {
        fn from(value: L1StandardBridgeCall) -> Self {
            Self::L1StandardBridge(value)
        }
    }
    impl ::core::convert::From<L2OutputOracleCall> for SystemConfigCalls {
        fn from(value: L2OutputOracleCall) -> Self {
            Self::L2OutputOracle(value)
        }
    }
    impl ::core::convert::From<MinimumGasLimitCall> for SystemConfigCalls {
        fn from(value: MinimumGasLimitCall) -> Self {
            Self::MinimumGasLimit(value)
        }
    }
    impl ::core::convert::From<OptimismMintableERC20FactoryCall> for SystemConfigCalls {
        fn from(value: OptimismMintableERC20FactoryCall) -> Self {
            Self::OptimismMintableERC20Factory(value)
        }
    }
    impl ::core::convert::From<OptimismPortalCall> for SystemConfigCalls {
        fn from(value: OptimismPortalCall) -> Self {
            Self::OptimismPortal(value)
        }
    }
    impl ::core::convert::From<OverheadCall> for SystemConfigCalls {
        fn from(value: OverheadCall) -> Self {
            Self::Overhead(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SystemConfigCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for SystemConfigCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ResourceConfigCall> for SystemConfigCalls {
        fn from(value: ResourceConfigCall) -> Self {
            Self::ResourceConfig(value)
        }
    }
    impl ::core::convert::From<ScalarCall> for SystemConfigCalls {
        fn from(value: ScalarCall) -> Self {
            Self::Scalar(value)
        }
    }
    impl ::core::convert::From<SetBatcherHashCall> for SystemConfigCalls {
        fn from(value: SetBatcherHashCall) -> Self {
            Self::SetBatcherHash(value)
        }
    }
    impl ::core::convert::From<SetGasConfigCall> for SystemConfigCalls {
        fn from(value: SetGasConfigCall) -> Self {
            Self::SetGasConfig(value)
        }
    }
    impl ::core::convert::From<SetGasLimitCall> for SystemConfigCalls {
        fn from(value: SetGasLimitCall) -> Self {
            Self::SetGasLimit(value)
        }
    }
    impl ::core::convert::From<SetResourceConfigCall> for SystemConfigCalls {
        fn from(value: SetResourceConfigCall) -> Self {
            Self::SetResourceConfig(value)
        }
    }
    impl ::core::convert::From<SetUnsafeBlockSignerCall> for SystemConfigCalls {
        fn from(value: SetUnsafeBlockSignerCall) -> Self {
            Self::SetUnsafeBlockSigner(value)
        }
    }
    impl ::core::convert::From<StartBlockCall> for SystemConfigCalls {
        fn from(value: StartBlockCall) -> Self {
            Self::StartBlock(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for SystemConfigCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnsafeBlockSignerCall> for SystemConfigCalls {
        fn from(value: UnsafeBlockSignerCall) -> Self {
            Self::UnsafeBlockSigner(value)
        }
    }
    impl ::core::convert::From<versionCall> for SystemConfigCalls {
        fn from(value: versionCall) -> Self {
            Self::version(value)
        }
    }
    ///Container type for all return fields from the `BATCH_INBOX_SLOT` function with signature `BATCH_INBOX_SLOT()` and selector `0xbc49ce5f`
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
    pub struct BatchInboxSlotReturn(pub [u8; 32]);
    ///Container type for all return fields from the `L1_CROSS_DOMAIN_MESSENGER_SLOT` function with signature `L1_CROSS_DOMAIN_MESSENGER_SLOT()` and selector `0x5d73369c`
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
    pub struct L1CrossDomainMessengerSlotReturn(pub [u8; 32]);
    ///Container type for all return fields from the `L1_ERC_721_BRIDGE_SLOT` function with signature `L1_ERC_721_BRIDGE_SLOT()` and selector `0x19f5cea8`
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
    pub struct L1Erc721BridgeSlotReturn(pub [u8; 32]);
    ///Container type for all return fields from the `L1_STANDARD_BRIDGE_SLOT` function with signature `L1_STANDARD_BRIDGE_SLOT()` and selector `0xf8c68de0`
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
    pub struct L1StandardBridgeSlotReturn(pub [u8; 32]);
    ///Container type for all return fields from the `L2_OUTPUT_ORACLE_SLOT` function with signature `L2_OUTPUT_ORACLE_SLOT()` and selector `0x61d15768`
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
    pub struct L2OutputOracleSlotReturn(pub [u8; 32]);
    ///Container type for all return fields from the `OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT` function with signature `OPTIMISM_MINTABLE_ERC20_FACTORY_SLOT()` and selector `0x06c92657`
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
    pub struct OptimismMintableErc20FactorySlotReturn(pub [u8; 32]);
    ///Container type for all return fields from the `OPTIMISM_PORTAL_SLOT` function with signature `OPTIMISM_PORTAL_SLOT()` and selector `0xfd32aa0f`
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
    pub struct OptimismPortalSlotReturn(pub [u8; 32]);
    ///Container type for all return fields from the `UNSAFE_BLOCK_SIGNER_SLOT` function with signature `UNSAFE_BLOCK_SIGNER_SLOT()` and selector `0x4f16540b`
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
    pub struct UnsafeBlockSignerSlotReturn(pub [u8; 32]);
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
    pub struct VERSIONReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `batchInbox` function with signature `batchInbox()` and selector `0xdac6e63a`
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
    pub struct BatchInboxReturn {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `batcherHash` function with signature `batcherHash()` and selector `0xe81b2c6d`
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
    pub struct BatcherHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `gasLimit` function with signature `gasLimit()` and selector `0xf68016b7`
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
    pub struct GasLimitReturn(pub u64);
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
    pub struct L1CrossDomainMessengerReturn {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `l1ERC721Bridge` function with signature `l1ERC721Bridge()` and selector `0xc4e8ddfa`
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
    pub struct L1Erc721BridgeReturn {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `l1StandardBridge` function with signature `l1StandardBridge()` and selector `0x078f29cf`
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
    pub struct L1StandardBridgeReturn {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `l2OutputOracle` function with signature `l2OutputOracle()` and selector `0x4d9f1559`
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
    pub struct L2OutputOracleReturn {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `minimumGasLimit` function with signature `minimumGasLimit()` and selector `0x4add321d`
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
    pub struct MinimumGasLimitReturn(pub u64);
    ///Container type for all return fields from the `optimismMintableERC20Factory` function with signature `optimismMintableERC20Factory()` and selector `0x9b7d7f0a`
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
    pub struct OptimismMintableERC20FactoryReturn {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `optimismPortal` function with signature `optimismPortal()` and selector `0x0a49cb03`
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
    pub struct OptimismPortalReturn {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `overhead` function with signature `overhead()` and selector `0x0c18c162`
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
    pub struct OverheadReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `resourceConfig` function with signature `resourceConfig()` and selector `0xcc731b02`
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
    pub struct ResourceConfigReturn(pub ResourceConfig);
    ///Container type for all return fields from the `scalar` function with signature `scalar()` and selector `0xf45e65d8`
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
    pub struct ScalarReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `startBlock` function with signature `startBlock()` and selector `0x48cd4cb1`
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
    pub struct StartBlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `unsafeBlockSigner` function with signature `unsafeBlockSigner()` and selector `0x1fd19ee1`
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
    pub struct UnsafeBlockSignerReturn {
        pub addr: ::ethers::core::types::Address,
    }
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
    pub struct versionReturn(pub ::std::string::String);
    ///`Addresses(address,address,address,address,address,address)`
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
    pub struct Addresses {
        pub l_1_cross_domain_messenger: ::ethers::core::types::Address,
        pub l_1erc721_bridge: ::ethers::core::types::Address,
        pub l_1_standard_bridge: ::ethers::core::types::Address,
        pub l_2_output_oracle: ::ethers::core::types::Address,
        pub optimism_portal: ::ethers::core::types::Address,
        pub optimism_mintable_erc20_factory: ::ethers::core::types::Address,
    }
}

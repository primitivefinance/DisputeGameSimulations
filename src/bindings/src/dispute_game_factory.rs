pub use dispute_game_factory::*;
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
pub mod dispute_game_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("create"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("create"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rootClaim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gameAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameAtIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_index"),
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
                                    name: ::std::borrow::ToOwned::to_owned("gameType_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Timestamp"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gameCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameCount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gameCount_"),
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
                    ::std::borrow::ToOwned::to_owned("gameImpls"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameImpls"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("games"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("games"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rootClaim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Timestamp"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGameUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGameUUID"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rootClaim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uuid_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Hash"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("setImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setImplementation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_impl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
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
                    ::std::borrow::ToOwned::to_owned("DisputeGameCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DisputeGameCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("disputeProxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rootClaim"),
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
                    ::std::borrow::ToOwned::to_owned("ImplementationSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ImplementationSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("impl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GameAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("GameAlreadyExists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uuid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Hash"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoImplementation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DISPUTEGAMEFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\0`\x80\x81\x90R`\xA0\x81\x90R`\x05`\xC0Rb\0\0.\x90b\0\x004V[b\0\x02\x7FV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\0UWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\0qWP0;\x15\x80\x15b\0\0qWP`\0T`\xFF\x16`\x01\x14[b\0\0\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\0\xFEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x01\x08b\0\x01^V[b\0\x01\x13\x82b\0\x01\xC6V[\x80\x15b\0\x01ZW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x01\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R`\0\x80Q` b\0\x15\xD0\x839\x81Q\x91R`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\0\xD1V[b\0\x01\xC4b\0\x02\x18V[V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x02tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R`\0\x80Q` b\0\x15\xD0\x839\x81Q\x91R`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\0\xD1V[b\0\x01\xC43b\0\x01\xC6V[`\x80Q`\xA0Q`\xC0Qa\x13!b\0\x02\xAF`\09`\0a\x06R\x01R`\0a\x06)\x01R`\0a\x06\0\x01Ra\x13!`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xD4W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x81W\x80c\xC4\xD6m\xE8\x11a\0[W\x80c\xC4\xD6m\xE8\x14a\x02\xB4W\x80c\xDF\xA1b\xD3\x14a\x02\xC7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xFDW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\xFDW\x80c\xBB\x8A\xA1\xFC\x14a\x02\x1BW\x80c\xC4\x9DRq\x14a\x02lW`\0\x80\xFD[\x80cM\x19u\xB4\x11a\0\xB2W\x80cM\x19u\xB4\x14a\x01\xD8W\x80cT\xFDMP\x14a\x01\xE0W\x80cqP\x18\xA6\x14a\x01\xF5W`\0\x80\xFD[\x80c&\xDA\xAF\xBE\x14a\0\xD9W\x80c1B\xE5^\x14a\x01\x8BW\x80cEX;z\x14a\x01\xC3W[`\0\x80\xFD[a\x01xa\0\xE76`\x04a\x0E\xD5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x81\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x83\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x86\x01\x80Q\x98\x86R\x96\x83R``\x87R\x94Q`\x9F\x01\x90\x94\x16\x83 \x91\x90\x92R\x91\x90R\x91\x90R\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Ea\x01\x996`\x04a\x0F\xBEV[a\x03\x10V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x82V[a\x01\xD6a\x01\xD16`\x04a\x10gV[a\x05rV[\0[`gTa\x01xV[a\x01\xE8a\x05\xF9V[`@Qa\x01\x82\x91\x90a\x10\xC2V[a\x01\xD6a\x06\x9CV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x9EV[a\x02.a\x02)6`\x04a\x11\x13V[a\x06\xB0V[`@\x80Q`\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x01\x82V[a\x02\x7Fa\x02z6`\x04a\x0F\xBEV[a\x07\x12V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\x82V[a\x01\xD6a\x02\xC26`\x04a\x11,V[a\x07\x9AV[a\x01\x9Ea\x02\xD56`\x04a\x11PV[`e` R`\0\x90\x81R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xD6a\x03\x0B6`\x04a\x11,V[a\t6V[`\xFF\x84\x16`\0\x90\x81R`e` R`@\x81 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x03zW`@Q\x7FD&]o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x87\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xDD\x85\x85\x85`@Q` \x01a\x03\x92\x93\x92\x91\x90a\x11kV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\t\xEDV[\x91P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x81)\xFC\x1C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04'W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04;W=`\0\x80>=`\0\xFD[PPPP`\0a\x04\x82\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\0\xE7\x92PPPV[`\0\x81\x81R`f` R`@\x90 T\x90\x91P\x15a\x04\xCEW`@Q\x7F\x01Oo\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03qV[`\0B`\xB8\x1B`\xF8\x89\x90\x1B\x17\x84\x17`\0\x83\x81R`f` R`@\x80\x82 \x83\x90U`g\x80T`\x01\x81\x01\x82U\x90\x83R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE\x01\x83\x90UQ\x91\x92P\x88\x91`\xFF\x8B\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x91\x7F\xFA\xD0Y\x9F\xF4I\xD8\xD9h^\xAD\xEC\xCA\x8C\xB9\xE0\t$\xC5\xFD\x83g\xC1\xC0\x94i\x82I9\xE1\xFF\xEC\x91\x90\xA4PPP\x94\x93PPPPV[a\x05za\x0B!V[`\xFF\x82\x16`\0\x81\x81R`e` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U\x90Q\x90\x91\x7Fb7\x13\xF7/nBz\x80D\xBB\x8B;\xD6\x83CW\xCF(]\xEC\xBA\xA2\x1B\xCCs\xC1\xD0c,M\x84\x91\xA3PPV[``a\x06$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xA2V[a\x06M\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xA2V[a\x06v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xA2V[`@Q` \x01a\x06\x88\x93\x92\x91\x90a\x11\x85V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\x06\xA4a\x0B!V[a\x06\xAE`\0a\x0C\xDFV[V[`\0\x80`\0a\x07\x05`g\x85\x81T\x81\x10a\x06\xCBWa\x06\xCBa\x11\xFBV[\x90`\0R` `\0 \x01T`\xF8\x81\x90\x1C\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xB8\x83\x90\x1C\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91\x96\x90\x95P\x90\x93P\x91PPV[`\0\x80`\0a\x07X\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\0\xE7\x92PPPV[`\0\x90\x81R`f` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\xB8\x91\x90\x91\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x97P\x95PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x07\xBAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x07\xD4WP0;\x15\x80\x15a\x07\xD4WP`\0T`\xFF\x16`\x01\x14[a\x08`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x08\xBEW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x08\xC6a\rVV[a\x08\xCF\x82a\x0C\xDFV[\x80\x15a\t2W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\t>a\x0B!V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\t\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[a\t\xEA\x81a\x0C\xDFV[PV[`\0`\x02\x82Q\x01`?\x81\x01`\n\x81\x03`@Q\x83`X\x1B\x82`\xE8\x1B\x17\x7Fa\0\0=\x81`\n=9\xF36==7====a\0\0\x80`5696\x01=s\0\0\x17\x81R\x86``\x1B`\x1E\x82\x01R\x7FZ\xF4==\x93\x80>`3W\xFD[\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`2\x82\x01R\x85Q\x91P`?\x81\x01` \x87\x01[` \x84\x10a\n\xA5W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x93\x01\x92` \x91\x82\x01\x91\x01a\nhV[Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x90\x03`\x03\x1B\x1B\x16\x81R`\xF0\x85\x90\x1B\x90\x83\x01R\x82\x81`\0\xF0\x94P\x84a\x0B\x12W\x7F\xEB\xFE\xF1\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R` `\0\xFD[\x90\x91\x01`@RP\x90\x93\x92PPPV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06\xAEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03qV[``\x81`\0\x03a\x0B\xE5WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x0C\x0FW\x80a\x0B\xF9\x81a\x12YV[\x91Pa\x0C\x08\x90P`\n\x83a\x12\xC0V[\x91Pa\x0B\xE9V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C*Wa\x0C*a\x0E\xA6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0CTW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x0C\xD7Wa\x0Ci`\x01\x83a\x12\xD4V[\x91Pa\x0Cv`\n\x86a\x12\xEDV[a\x0C\x81\x90`0a\x13\x01V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x0C\x96Wa\x0C\x96a\x11\xFBV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0C\xD0`\n\x86a\x12\xC0V[\x94Pa\x0CXV[\x94\x93PPPPV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\r\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[a\x06\xAE`\0Ta\x01\0\x90\x04`\xFF\x16a\x0E\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[a\x06\xAE3a\x0C\xDFV[\x805`\xFF\x81\x16\x81\x14a\x0E\xA1W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xEAW`\0\x80\xFD[a\x0E\xF3\x84a\x0E\x90V[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x17W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F+W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F=Wa\x0F=a\x0E\xA6V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0F\x83Wa\x0F\x83a\x0E\xA6V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x0F\x9CW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0F\xD4W`\0\x80\xFD[a\x0F\xDD\x85a\x0E\x90V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\x01W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x10\x15W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10$W`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x106W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xEAW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10zW`\0\x80\xFD[a\x10\x83\x83a\x0E\x90V[\x91P` \x83\x015a\x10\x93\x81a\x10EV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x10\xB9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\xA1V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x10\xE1\x81`@\x85\x01` \x87\x01a\x10\x9EV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11%W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x11>W`\0\x80\xFD[\x815a\x11I\x81a\x10EV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11bW`\0\x80\xFD[a\x11I\x82a\x0E\x90V[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0\x84Qa\x11\x97\x81\x84` \x89\x01a\x10\x9EV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x11\xD3\x81`\x01\x85\x01` \x8A\x01a\x10\x9EV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x11\xEE\x81`\x02\x84\x01` \x88\x01a\x10\x9EV[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x12\x8AWa\x12\x8Aa\x12*V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x12\xCFWa\x12\xCFa\x12\x91V[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x12\xE7Wa\x12\xE7a\x12*V[\x92\x91PPV[`\0\x82a\x12\xFCWa\x12\xFCa\x12\x91V[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x12\xE7Wa\x12\xE7a\x12*V\xFE\xA1dsolcC\0\x08\x15\0\nInitializable: contract is not i";
    /// The bytecode of the contract.
    pub static DISPUTEGAMEFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xD4W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x81W\x80c\xC4\xD6m\xE8\x11a\0[W\x80c\xC4\xD6m\xE8\x14a\x02\xB4W\x80c\xDF\xA1b\xD3\x14a\x02\xC7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xFDW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\xFDW\x80c\xBB\x8A\xA1\xFC\x14a\x02\x1BW\x80c\xC4\x9DRq\x14a\x02lW`\0\x80\xFD[\x80cM\x19u\xB4\x11a\0\xB2W\x80cM\x19u\xB4\x14a\x01\xD8W\x80cT\xFDMP\x14a\x01\xE0W\x80cqP\x18\xA6\x14a\x01\xF5W`\0\x80\xFD[\x80c&\xDA\xAF\xBE\x14a\0\xD9W\x80c1B\xE5^\x14a\x01\x8BW\x80cEX;z\x14a\x01\xC3W[`\0\x80\xFD[a\x01xa\0\xE76`\x04a\x0E\xD5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x81\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x83\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x86\x01\x80Q\x98\x86R\x96\x83R``\x87R\x94Q`\x9F\x01\x90\x94\x16\x83 \x91\x90\x92R\x91\x90R\x91\x90R\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Ea\x01\x996`\x04a\x0F\xBEV[a\x03\x10V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x82V[a\x01\xD6a\x01\xD16`\x04a\x10gV[a\x05rV[\0[`gTa\x01xV[a\x01\xE8a\x05\xF9V[`@Qa\x01\x82\x91\x90a\x10\xC2V[a\x01\xD6a\x06\x9CV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x9EV[a\x02.a\x02)6`\x04a\x11\x13V[a\x06\xB0V[`@\x80Q`\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x01\x82V[a\x02\x7Fa\x02z6`\x04a\x0F\xBEV[a\x07\x12V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\x82V[a\x01\xD6a\x02\xC26`\x04a\x11,V[a\x07\x9AV[a\x01\x9Ea\x02\xD56`\x04a\x11PV[`e` R`\0\x90\x81R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xD6a\x03\x0B6`\x04a\x11,V[a\t6V[`\xFF\x84\x16`\0\x90\x81R`e` R`@\x81 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x03zW`@Q\x7FD&]o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x87\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xDD\x85\x85\x85`@Q` \x01a\x03\x92\x93\x92\x91\x90a\x11kV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\t\xEDV[\x91P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x81)\xFC\x1C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04'W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04;W=`\0\x80>=`\0\xFD[PPPP`\0a\x04\x82\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\0\xE7\x92PPPV[`\0\x81\x81R`f` R`@\x90 T\x90\x91P\x15a\x04\xCEW`@Q\x7F\x01Oo\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03qV[`\0B`\xB8\x1B`\xF8\x89\x90\x1B\x17\x84\x17`\0\x83\x81R`f` R`@\x80\x82 \x83\x90U`g\x80T`\x01\x81\x01\x82U\x90\x83R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE\x01\x83\x90UQ\x91\x92P\x88\x91`\xFF\x8B\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x91\x7F\xFA\xD0Y\x9F\xF4I\xD8\xD9h^\xAD\xEC\xCA\x8C\xB9\xE0\t$\xC5\xFD\x83g\xC1\xC0\x94i\x82I9\xE1\xFF\xEC\x91\x90\xA4PPP\x94\x93PPPPV[a\x05za\x0B!V[`\xFF\x82\x16`\0\x81\x81R`e` R`@\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U\x90Q\x90\x91\x7Fb7\x13\xF7/nBz\x80D\xBB\x8B;\xD6\x83CW\xCF(]\xEC\xBA\xA2\x1B\xCCs\xC1\xD0c,M\x84\x91\xA3PPV[``a\x06$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xA2V[a\x06M\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xA2V[a\x06v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\xA2V[`@Q` \x01a\x06\x88\x93\x92\x91\x90a\x11\x85V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\x06\xA4a\x0B!V[a\x06\xAE`\0a\x0C\xDFV[V[`\0\x80`\0a\x07\x05`g\x85\x81T\x81\x10a\x06\xCBWa\x06\xCBa\x11\xFBV[\x90`\0R` `\0 \x01T`\xF8\x81\x90\x1C\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xB8\x83\x90\x1C\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91\x96\x90\x95P\x90\x93P\x91PPV[`\0\x80`\0a\x07X\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\0\xE7\x92PPPV[`\0\x90\x81R`f` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\xB8\x91\x90\x91\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x97P\x95PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x07\xBAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x07\xD4WP0;\x15\x80\x15a\x07\xD4WP`\0T`\xFF\x16`\x01\x14[a\x08`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x08\xBEW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x08\xC6a\rVV[a\x08\xCF\x82a\x0C\xDFV[\x80\x15a\t2W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\t>a\x0B!V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\t\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[a\t\xEA\x81a\x0C\xDFV[PV[`\0`\x02\x82Q\x01`?\x81\x01`\n\x81\x03`@Q\x83`X\x1B\x82`\xE8\x1B\x17\x7Fa\0\0=\x81`\n=9\xF36==7====a\0\0\x80`5696\x01=s\0\0\x17\x81R\x86``\x1B`\x1E\x82\x01R\x7FZ\xF4==\x93\x80>`3W\xFD[\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`2\x82\x01R\x85Q\x91P`?\x81\x01` \x87\x01[` \x84\x10a\n\xA5W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x93\x01\x92` \x91\x82\x01\x91\x01a\nhV[Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x90\x03`\x03\x1B\x1B\x16\x81R`\xF0\x85\x90\x1B\x90\x83\x01R\x82\x81`\0\xF0\x94P\x84a\x0B\x12W\x7F\xEB\xFE\xF1\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R` `\0\xFD[\x90\x91\x01`@RP\x90\x93\x92PPPV[`3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x06\xAEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03qV[``\x81`\0\x03a\x0B\xE5WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x0C\x0FW\x80a\x0B\xF9\x81a\x12YV[\x91Pa\x0C\x08\x90P`\n\x83a\x12\xC0V[\x91Pa\x0B\xE9V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C*Wa\x0C*a\x0E\xA6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0CTW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x0C\xD7Wa\x0Ci`\x01\x83a\x12\xD4V[\x91Pa\x0Cv`\n\x86a\x12\xEDV[a\x0C\x81\x90`0a\x13\x01V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x0C\x96Wa\x0C\x96a\x11\xFBV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0C\xD0`\n\x86a\x12\xC0V[\x94Pa\x0CXV[\x94\x93PPPPV[`3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\r\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[a\x06\xAE`\0Ta\x01\0\x90\x04`\xFF\x16a\x0E\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03qV[a\x06\xAE3a\x0C\xDFV[\x805`\xFF\x81\x16\x81\x14a\x0E\xA1W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xEAW`\0\x80\xFD[a\x0E\xF3\x84a\x0E\x90V[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x17W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F+W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F=Wa\x0F=a\x0E\xA6V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0F\x83Wa\x0F\x83a\x0E\xA6V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x0F\x9CW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0F\xD4W`\0\x80\xFD[a\x0F\xDD\x85a\x0E\x90V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\x01W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x10\x15W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10$W`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x106W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xEAW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10zW`\0\x80\xFD[a\x10\x83\x83a\x0E\x90V[\x91P` \x83\x015a\x10\x93\x81a\x10EV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x10\xB9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\xA1V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x10\xE1\x81`@\x85\x01` \x87\x01a\x10\x9EV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11%W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x11>W`\0\x80\xFD[\x815a\x11I\x81a\x10EV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11bW`\0\x80\xFD[a\x11I\x82a\x0E\x90V[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0\x84Qa\x11\x97\x81\x84` \x89\x01a\x10\x9EV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x11\xD3\x81`\x01\x85\x01` \x8A\x01a\x10\x9EV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x11\xEE\x81`\x02\x84\x01` \x88\x01a\x10\x9EV[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x12\x8AWa\x12\x8Aa\x12*V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x12\xCFWa\x12\xCFa\x12\x91V[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x12\xE7Wa\x12\xE7a\x12*V[\x92\x91PPV[`\0\x82a\x12\xFCWa\x12\xFCa\x12\x91V[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x12\xE7Wa\x12\xE7a\x12*V\xFE\xA1dsolcC\0\x08\x15\0\n";
    /// The deployed bytecode of the contract.
    pub static DISPUTEGAMEFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DisputeGameFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DisputeGameFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DisputeGameFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DisputeGameFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DisputeGameFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DisputeGameFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DisputeGameFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DISPUTEGAMEFACTORY_ABI.clone(),
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
                DISPUTEGAMEFACTORY_ABI.clone(),
                DISPUTEGAMEFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `create` (0x3142e55e) function
        pub fn create(
            &self,
            game_type: u8,
            root_claim: [u8; 32],
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([49, 66, 229, 94], (game_type, root_claim, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameAtIndex` (0xbb8aa1fc) function
        pub fn game_at_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, u64, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([187, 138, 161, 252], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameCount` (0x4d1975b4) function
        pub fn game_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 25, 117, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameImpls` (0xdfa162d3) function
        pub fn game_impls(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 161, 98, 211], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `games` (0xc49d5271) function
        pub fn games(
            &self,
            game_type: u8,
            root_claim: [u8; 32],
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u64),
        > {
            self.0
                .method_hash([196, 157, 82, 113], (game_type, root_claim, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGameUUID` (0x26daafbe) function
        pub fn get_game_uuid(
            &self,
            game_type: u8,
            root_claim: [u8; 32],
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([38, 218, 175, 190], (game_type, root_claim, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], owner)
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
        ///Calls the contract's `setImplementation` (0x45583b7a) function
        pub fn set_implementation(
            &self,
            game_type: u8,
            impl_: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 88, 59, 122], (game_type, impl_))
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
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DisputeGameCreated` event
        pub fn dispute_game_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisputeGameCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ImplementationSet` event
        pub fn implementation_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ImplementationSetFilter,
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
            DisputeGameFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DisputeGameFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `GameAlreadyExists` with signature `GameAlreadyExists(bytes32)` and selector `0x014f6fe5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "GameAlreadyExists", abi = "GameAlreadyExists(bytes32)")]
    pub struct GameAlreadyExists {
        pub uuid: [u8; 32],
    }
    ///Custom Error type `NoImplementation` with signature `NoImplementation(uint8)` and selector `0x44265d6f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NoImplementation", abi = "NoImplementation(uint8)")]
    pub struct NoImplementation {
        pub game_type: u8,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DisputeGameFactoryErrors {
        GameAlreadyExists(GameAlreadyExists),
        NoImplementation(NoImplementation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DisputeGameFactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <GameAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameAlreadyExists(decoded));
            }
            if let Ok(decoded) = <NoImplementation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoImplementation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DisputeGameFactoryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::GameAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DisputeGameFactoryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <GameAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DisputeGameFactoryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GameAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoImplementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DisputeGameFactoryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<GameAlreadyExists> for DisputeGameFactoryErrors {
        fn from(value: GameAlreadyExists) -> Self {
            Self::GameAlreadyExists(value)
        }
    }
    impl ::core::convert::From<NoImplementation> for DisputeGameFactoryErrors {
        fn from(value: NoImplementation) -> Self {
            Self::NoImplementation(value)
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
        name = "DisputeGameCreated",
        abi = "DisputeGameCreated(address,uint8,bytes32)"
    )]
    pub struct DisputeGameCreatedFilter {
        #[ethevent(indexed)]
        pub dispute_proxy: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub game_type: u8,
        #[ethevent(indexed)]
        pub root_claim: [u8; 32],
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
    #[ethevent(name = "ImplementationSet", abi = "ImplementationSet(address,uint8)")]
    pub struct ImplementationSetFilter {
        #[ethevent(indexed)]
        pub impl_: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub game_type: u8,
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
    pub enum DisputeGameFactoryEvents {
        DisputeGameCreatedFilter(DisputeGameCreatedFilter),
        ImplementationSetFilter(ImplementationSetFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for DisputeGameFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DisputeGameCreatedFilter::decode_log(log) {
                return Ok(DisputeGameFactoryEvents::DisputeGameCreatedFilter(decoded));
            }
            if let Ok(decoded) = ImplementationSetFilter::decode_log(log) {
                return Ok(DisputeGameFactoryEvents::ImplementationSetFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(DisputeGameFactoryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DisputeGameFactoryEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DisputeGameFactoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DisputeGameCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ImplementationSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DisputeGameCreatedFilter> for DisputeGameFactoryEvents {
        fn from(value: DisputeGameCreatedFilter) -> Self {
            Self::DisputeGameCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ImplementationSetFilter> for DisputeGameFactoryEvents {
        fn from(value: ImplementationSetFilter) -> Self {
            Self::ImplementationSetFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for DisputeGameFactoryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for DisputeGameFactoryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `create` function with signature `create(uint8,bytes32,bytes)` and selector `0x3142e55e`
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
    #[ethcall(name = "create", abi = "create(uint8,bytes32,bytes)")]
    pub struct CreateCall {
        pub game_type: u8,
        pub root_claim: [u8; 32],
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `gameAtIndex` function with signature `gameAtIndex(uint256)` and selector `0xbb8aa1fc`
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
    #[ethcall(name = "gameAtIndex", abi = "gameAtIndex(uint256)")]
    pub struct GameAtIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `gameCount` function with signature `gameCount()` and selector `0x4d1975b4`
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
    #[ethcall(name = "gameCount", abi = "gameCount()")]
    pub struct GameCountCall;
    ///Container type for all input parameters for the `gameImpls` function with signature `gameImpls(uint8)` and selector `0xdfa162d3`
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
    #[ethcall(name = "gameImpls", abi = "gameImpls(uint8)")]
    pub struct GameImplsCall(pub u8);
    ///Container type for all input parameters for the `games` function with signature `games(uint8,bytes32,bytes)` and selector `0xc49d5271`
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
    #[ethcall(name = "games", abi = "games(uint8,bytes32,bytes)")]
    pub struct GamesCall {
        pub game_type: u8,
        pub root_claim: [u8; 32],
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getGameUUID` function with signature `getGameUUID(uint8,bytes32,bytes)` and selector `0x26daafbe`
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
    #[ethcall(name = "getGameUUID", abi = "getGameUUID(uint8,bytes32,bytes)")]
    pub struct GetGameUUIDCall {
        pub game_type: u8,
        pub root_claim: [u8; 32],
        pub extra_data: ::ethers::core::types::Bytes,
    }
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
        pub owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setImplementation` function with signature `setImplementation(uint8,address)` and selector `0x45583b7a`
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
    #[ethcall(name = "setImplementation", abi = "setImplementation(uint8,address)")]
    pub struct SetImplementationCall {
        pub game_type: u8,
        pub impl_: ::ethers::core::types::Address,
    }
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
    pub enum DisputeGameFactoryCalls {
        Create(CreateCall),
        GameAtIndex(GameAtIndexCall),
        GameCount(GameCountCall),
        GameImpls(GameImplsCall),
        Games(GamesCall),
        GetGameUUID(GetGameUUIDCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetImplementation(SetImplementationCall),
        TransferOwnership(TransferOwnershipCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for DisputeGameFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CreateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Create(decoded));
            }
            if let Ok(decoded) = <GameAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameAtIndex(decoded));
            }
            if let Ok(decoded) = <GameCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameCount(decoded));
            }
            if let Ok(decoded) = <GameImplsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameImpls(decoded));
            }
            if let Ok(decoded) = <GamesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Games(decoded));
            }
            if let Ok(decoded) = <GetGameUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGameUUID(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
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
            if let Ok(decoded) = <SetImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetImplementation(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DisputeGameFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Create(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GameAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GameCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GameImpls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Games(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetGameUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DisputeGameFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Create(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameImpls(element) => ::core::fmt::Display::fmt(element, f),
                Self::Games(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGameUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetImplementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateCall> for DisputeGameFactoryCalls {
        fn from(value: CreateCall) -> Self {
            Self::Create(value)
        }
    }
    impl ::core::convert::From<GameAtIndexCall> for DisputeGameFactoryCalls {
        fn from(value: GameAtIndexCall) -> Self {
            Self::GameAtIndex(value)
        }
    }
    impl ::core::convert::From<GameCountCall> for DisputeGameFactoryCalls {
        fn from(value: GameCountCall) -> Self {
            Self::GameCount(value)
        }
    }
    impl ::core::convert::From<GameImplsCall> for DisputeGameFactoryCalls {
        fn from(value: GameImplsCall) -> Self {
            Self::GameImpls(value)
        }
    }
    impl ::core::convert::From<GamesCall> for DisputeGameFactoryCalls {
        fn from(value: GamesCall) -> Self {
            Self::Games(value)
        }
    }
    impl ::core::convert::From<GetGameUUIDCall> for DisputeGameFactoryCalls {
        fn from(value: GetGameUUIDCall) -> Self {
            Self::GetGameUUID(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for DisputeGameFactoryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DisputeGameFactoryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DisputeGameFactoryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetImplementationCall> for DisputeGameFactoryCalls {
        fn from(value: SetImplementationCall) -> Self {
            Self::SetImplementation(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DisputeGameFactoryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<VersionCall> for DisputeGameFactoryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `create` function with signature `create(uint8,bytes32,bytes)` and selector `0x3142e55e`
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
    pub struct CreateReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `gameAtIndex` function with signature `gameAtIndex(uint256)` and selector `0xbb8aa1fc`
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
    pub struct GameAtIndexReturn {
        pub game_type: u8,
        pub timestamp: u64,
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `gameCount` function with signature `gameCount()` and selector `0x4d1975b4`
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
    pub struct GameCountReturn {
        pub game_count: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `gameImpls` function with signature `gameImpls(uint8)` and selector `0xdfa162d3`
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
    pub struct GameImplsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `games` function with signature `games(uint8,bytes32,bytes)` and selector `0xc49d5271`
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
    pub struct GamesReturn {
        pub proxy: ::ethers::core::types::Address,
        pub timestamp: u64,
    }
    ///Container type for all return fields from the `getGameUUID` function with signature `getGameUUID(uint8,bytes32,bytes)` and selector `0x26daafbe`
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
    pub struct GetGameUUIDReturn {
        pub uuid: [u8; 32],
    }
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

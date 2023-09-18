pub use faucet::*;
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
pub mod faucet {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADMIN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADMIN"),
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
                    ::std::borrow::ToOwned::to_owned("configure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("configure"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IFaucetAuthModule",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Faucet.ModuleConfig",
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
                                    name: ::std::borrow::ToOwned::to_owned("_params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Faucet.DripParameters",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_auth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Faucet.AuthParameters",
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
                    ::std::borrow::ToOwned::to_owned("modules"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("modules"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IFaucetAuthModule",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ttl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
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
                    ::std::borrow::ToOwned::to_owned("timeouts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("timeouts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IFaucetAuthModule",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
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
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Drip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Drip"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authModule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("userId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FAUCET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x12!8\x03\x80a\x12!\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x11\x88a\0\x99`\09`\0\x81\x81a\x01\x04\x01R\x81\x81a\x02\x19\x01Ra\t\x87\x01Ra\x11\x88`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c8u{\xFD\x11a\0NW\x80c8u{\xFD\x14a\x01KW\x80c\xA8\xEEI\xFE\x14a\x01\x91W\x80c\xB9`\xBC+\x14a\x01\xC1W\x80c\xF3\xFE\xF3\xA3\x14a\x01\xE1W`\0\x80\xFD[\x80c\x1C\xB8\x1B\x88\x14a\0\x80W\x80c\x1D]&\xBC\x14a\0\xA2W\x80c*\n\xCCj\x14a\0\xF2W`\0\x80\xFD[6a\0{W\0[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x0B\xF3V[a\x02\x01V[\0[4\x80\x15a\0\xAEW`\0\x80\xFD[Pa\0\xDDa\0\xBD6`\x04a\x0C\xB4V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xFEW`\0\x80\xFD[Pa\x01&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE9V[4\x80\x15a\x01WW`\0\x80\xFD[Pa\x01\x83a\x01f6`\x04a\x0C\xD6V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xE9V[4\x80\x15a\x01\x9DW`\0\x80\xFD[Pa\x01\xB1a\x01\xAC6`\x04a\r\x02V[a\x03RV[`@Qa\0\xE9\x94\x93\x92\x91\x90a\r\x9CV[4\x80\x15a\x01\xCDW`\0\x80\xFD[Pa\0\xA0a\x01\xDC6`\x04a\r\xCBV[a\x04\x08V[4\x80\x15a\x01\xEDW`\0\x80\xFD[Pa\0\xA0a\x01\xFC6`\x04a\x0C\xD6V[a\toV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFaucet: function can only be cal`D\x82\x01R\x7Fled by admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81Q\x82\x91\x90\x81\x90a\x03\x01\x90\x82a\x0FGV[P` \x82\x01Q`\x01\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01UPPV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T\x81\x90a\x03m\x90a\x0E\xA5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x99\x90a\x0E\xA5V[\x80\x15a\x03\xE6W\x80`\x1F\x10a\x03\xBBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xE6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xC9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x01\x83\x01T`\x02\x84\x01T`\x03\x90\x94\x01T\x92\x93`\xFF\x90\x91\x16\x92\x90\x91P\x84V[\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x81Q`\x80\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x04G\x90a\x0E\xA5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04s\x90a\x0E\xA5V[\x80\x15a\x04\xC0W\x80`\x1F\x10a\x04\x95Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xC0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x01\x82\x01T`\xFF\x16\x15\x15` \x80\x83\x01\x91\x90\x91R`\x02\x83\x01T`@\x83\x01R`\x03\x90\x92\x01T``\x90\x91\x01R\x81\x01Q\x90\x91Pa\x05\x83W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FFaucet: provided auth module is `D\x82\x01R\x7Fnot supported by this faucet\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xC2V[` \x80\x83\x01Q`\0\x90\x81R`\x02\x82R`@\x80\x82 \x86\x84\x01Q\x83R\x90\x92R T`\xFF\x16\x15a\x062W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FFaucet: nonce has already been u`D\x82\x01R\x7Fsed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xC2V[\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x82\x86\x01Q\x84R\x90\x91R\x90 TB\x11a\x06\xF5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FFaucet: auth cannot be used yet `D\x82\x01R\x7Fbecause timeout has not elapsed\0`d\x82\x01R`\x84\x01a\x02\xC2V[\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\xF5C\x1F\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92c\xF5C\x1F\xFA\x92a\x07T\x92\x88\x92`\x04\x01a\x10aV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x95\x91\x90a\x10\xAAV[a\x08#W`@\x80Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FFaucet: drip parameters could no`D\x82\x01R\x7Ft be verified by security module`d\x82\x01R`\x84\x01a\x02\xC2V[`@\x81\x01Qa\x082\x90Ba\x10\xC7V[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x82\x88\x01\x80Q\x86R\x90\x83R\x81\x85 \x95\x90\x95U\x93Q\x83R`\x02\x81R\x83\x83 \x87\x82\x01Q\x84R\x90R\x90\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90\x91\x17\x90U``\x82\x01Q\x84Q\x91Q\x90\x91\x90a\x08\xC1\x90a\n\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\x08\xFBW=`\0\x80>=`\0\xFD[PP\x82Q` \x83\x01Q\x82Q`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92a\t,\x91\x90a\x11\x06V[`@Q\x90\x81\x90\x03\x81 ``\x85\x01Q\x82R\x90\x7F,\xEB\xDF\x1C\xC7\x06\xA5\x0E\x1B(\xBF/\xC5\xCF\xBDr\x04tz;\x82C\x9B\x85r\x1AGM\xF3\xA3U\xA4\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFaucet: function can only be cal`D\x82\x01R\x7Fled by admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xC2V[\x80\x82`@Qa\nB\x90a\n\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\n|W=`\0\x80>=`\0\xFD[PPPPV[`Y\x80a\x11#\x839\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\xB0W`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x05Wa\x0B\x05a\n\xB3V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x05Wa\x0B\x05a\n\xB3V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x05Wa\x0B\x05a\n\xB3V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x0BlWa\x0Bla\n\xB3V[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xB2Wa\x0B\xB2a\n\xB3V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x0B\xCBW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[\x80\x15\x15\x81\x14a\n\xB0W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x06W`\0\x80\xFD[\x825a\x0C\x11\x81a\n\x8EV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C.W`\0\x80\xFD[\x90\x84\x01\x90`\x80\x82\x87\x03\x12\x15a\x0CBW`\0\x80\xFD[a\x0CJa\n\xE2V[\x825\x82\x81\x11\x15a\x0CYW`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x0ClW`\0\x80\xFD[a\x0C{\x87\x835` \x85\x01a\x0BQV[\x81R` \x83\x015\x91Pa\x0C\x8D\x82a\x0B\xE5V[\x81` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xC7W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xE9W`\0\x80\xFD[\x825a\x0C\xF4\x81a\n\x8EV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\r\x14W`\0\x80\xFD[\x815a\r\x1F\x81a\n\x8EV[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\rAW\x81\x81\x01Q\x83\x82\x01R` \x01a\r)V[\x83\x81\x11\x15a\n|WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rj\x81` \x86\x01` \x86\x01a\r&V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x80\x81R`\0a\r\xAF`\x80\x83\x01\x87a\rRV[\x94\x15\x15` \x83\x01RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x91\x90PV[`\0\x80\x82\x84\x03``\x81\x12\x15a\r\xDFW`\0\x80\xFD[`@\x81\x12\x15a\r\xEDW`\0\x80\xFD[Pa\r\xF6a\x0B\x0BV[\x835a\x0E\x01\x81a\n\x8EV[\x81R` \x84\x81\x015\x90\x82\x01R\x91P`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E*W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x0E>W`\0\x80\xFD[a\x0EFa\x0B.V[\x825a\x0EQ\x81a\n\x8EV[\x81R` \x83\x81\x015\x90\x82\x01R`@\x83\x015\x82\x81\x11\x15a\x0EoW`\0\x80\xFD[\x80\x84\x01\x93PP\x86`\x1F\x84\x01\x12a\x0E\x84W`\0\x80\xFD[a\x0E\x93\x87\x845` \x86\x01a\x0BQV[`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\xB9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\xF2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0FBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0F\x1FWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F>W\x82\x81U`\x01\x01a\x0F+V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FaWa\x0Faa\n\xB3V[a\x0Fu\x81a\x0Fo\x84Ta\x0E\xA5V[\x84a\x0E\xF8V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0F\xC8W`\0\x84\x15a\x0F\x92WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0F>V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x10\x15W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\xF6V[P\x85\x82\x10\x15a\x10QW\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84Q\x16\x81R` \x84\x01Q` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x10\xA1`\x80\x83\x01\x84a\rRV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x10\xBCW`\0\x80\xFD[\x81Qa\r\x1F\x81a\x0B\xE5V[`\0\x82\x19\x82\x11\x15a\x11\x01W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\0\x82Qa\x11\x18\x81\x84` \x87\x01a\r&V[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R`@Q`Y8\x03\x80`Y\x839\x81\x01`@\x81\x90R`\x1E\x91`*V[\x80`\x01`\x01`\xA0\x1B\x03\x16\xFF[`\0` \x82\x84\x03\x12\x15`;W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`QW`\0\x80\xFD[\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static FAUCET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c8u{\xFD\x11a\0NW\x80c8u{\xFD\x14a\x01KW\x80c\xA8\xEEI\xFE\x14a\x01\x91W\x80c\xB9`\xBC+\x14a\x01\xC1W\x80c\xF3\xFE\xF3\xA3\x14a\x01\xE1W`\0\x80\xFD[\x80c\x1C\xB8\x1B\x88\x14a\0\x80W\x80c\x1D]&\xBC\x14a\0\xA2W\x80c*\n\xCCj\x14a\0\xF2W`\0\x80\xFD[6a\0{W\0[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x0B\xF3V[a\x02\x01V[\0[4\x80\x15a\0\xAEW`\0\x80\xFD[Pa\0\xDDa\0\xBD6`\x04a\x0C\xB4V[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xFEW`\0\x80\xFD[Pa\x01&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE9V[4\x80\x15a\x01WW`\0\x80\xFD[Pa\x01\x83a\x01f6`\x04a\x0C\xD6V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xE9V[4\x80\x15a\x01\x9DW`\0\x80\xFD[Pa\x01\xB1a\x01\xAC6`\x04a\r\x02V[a\x03RV[`@Qa\0\xE9\x94\x93\x92\x91\x90a\r\x9CV[4\x80\x15a\x01\xCDW`\0\x80\xFD[Pa\0\xA0a\x01\xDC6`\x04a\r\xCBV[a\x04\x08V[4\x80\x15a\x01\xEDW`\0\x80\xFD[Pa\0\xA0a\x01\xFC6`\x04a\x0C\xD6V[a\toV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFaucet: function can only be cal`D\x82\x01R\x7Fled by admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81Q\x82\x91\x90\x81\x90a\x03\x01\x90\x82a\x0FGV[P` \x82\x01Q`\x01\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90U`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01UPPV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T\x81\x90a\x03m\x90a\x0E\xA5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x99\x90a\x0E\xA5V[\x80\x15a\x03\xE6W\x80`\x1F\x10a\x03\xBBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xE6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xC9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x01\x83\x01T`\x02\x84\x01T`\x03\x90\x94\x01T\x92\x93`\xFF\x90\x91\x16\x92\x90\x91P\x84V[\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x81Q`\x80\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x04G\x90a\x0E\xA5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04s\x90a\x0E\xA5V[\x80\x15a\x04\xC0W\x80`\x1F\x10a\x04\x95Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xC0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x01\x82\x01T`\xFF\x16\x15\x15` \x80\x83\x01\x91\x90\x91R`\x02\x83\x01T`@\x83\x01R`\x03\x90\x92\x01T``\x90\x91\x01R\x81\x01Q\x90\x91Pa\x05\x83W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FFaucet: provided auth module is `D\x82\x01R\x7Fnot supported by this faucet\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xC2V[` \x80\x83\x01Q`\0\x90\x81R`\x02\x82R`@\x80\x82 \x86\x84\x01Q\x83R\x90\x92R T`\xFF\x16\x15a\x062W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FFaucet: nonce has already been u`D\x82\x01R\x7Fsed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xC2V[\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x82\x86\x01Q\x84R\x90\x91R\x90 TB\x11a\x06\xF5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FFaucet: auth cannot be used yet `D\x82\x01R\x7Fbecause timeout has not elapsed\0`d\x82\x01R`\x84\x01a\x02\xC2V[\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\xF5C\x1F\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92c\xF5C\x1F\xFA\x92a\x07T\x92\x88\x92`\x04\x01a\x10aV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x95\x91\x90a\x10\xAAV[a\x08#W`@\x80Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FFaucet: drip parameters could no`D\x82\x01R\x7Ft be verified by security module`d\x82\x01R`\x84\x01a\x02\xC2V[`@\x81\x01Qa\x082\x90Ba\x10\xC7V[\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x82\x88\x01\x80Q\x86R\x90\x83R\x81\x85 \x95\x90\x95U\x93Q\x83R`\x02\x81R\x83\x83 \x87\x82\x01Q\x84R\x90R\x90\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90\x91\x17\x90U``\x82\x01Q\x84Q\x91Q\x90\x91\x90a\x08\xC1\x90a\n\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\x08\xFBW=`\0\x80>=`\0\xFD[PP\x82Q` \x83\x01Q\x82Q`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92a\t,\x91\x90a\x11\x06V[`@Q\x90\x81\x90\x03\x81 ``\x85\x01Q\x82R\x90\x7F,\xEB\xDF\x1C\xC7\x06\xA5\x0E\x1B(\xBF/\xC5\xCF\xBDr\x04tz;\x82C\x9B\x85r\x1AGM\xF3\xA3U\xA4\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFaucet: function can only be cal`D\x82\x01R\x7Fled by admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xC2V[\x80\x82`@Qa\nB\x90a\n\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\n|W=`\0\x80>=`\0\xFD[PPPPV[`Y\x80a\x11#\x839\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\xB0W`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x05Wa\x0B\x05a\n\xB3V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x05Wa\x0B\x05a\n\xB3V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x05Wa\x0B\x05a\n\xB3V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x0BlWa\x0Bla\n\xB3V[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xB2Wa\x0B\xB2a\n\xB3V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x0B\xCBW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[\x80\x15\x15\x81\x14a\n\xB0W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x06W`\0\x80\xFD[\x825a\x0C\x11\x81a\n\x8EV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C.W`\0\x80\xFD[\x90\x84\x01\x90`\x80\x82\x87\x03\x12\x15a\x0CBW`\0\x80\xFD[a\x0CJa\n\xE2V[\x825\x82\x81\x11\x15a\x0CYW`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x0ClW`\0\x80\xFD[a\x0C{\x87\x835` \x85\x01a\x0BQV[\x81R` \x83\x015\x91Pa\x0C\x8D\x82a\x0B\xE5V[\x81` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xC7W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xE9W`\0\x80\xFD[\x825a\x0C\xF4\x81a\n\x8EV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\r\x14W`\0\x80\xFD[\x815a\r\x1F\x81a\n\x8EV[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\rAW\x81\x81\x01Q\x83\x82\x01R` \x01a\r)V[\x83\x81\x11\x15a\n|WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rj\x81` \x86\x01` \x86\x01a\r&V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x80\x81R`\0a\r\xAF`\x80\x83\x01\x87a\rRV[\x94\x15\x15` \x83\x01RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x91\x90PV[`\0\x80\x82\x84\x03``\x81\x12\x15a\r\xDFW`\0\x80\xFD[`@\x81\x12\x15a\r\xEDW`\0\x80\xFD[Pa\r\xF6a\x0B\x0BV[\x835a\x0E\x01\x81a\n\x8EV[\x81R` \x84\x81\x015\x90\x82\x01R\x91P`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E*W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x0E>W`\0\x80\xFD[a\x0EFa\x0B.V[\x825a\x0EQ\x81a\n\x8EV[\x81R` \x83\x81\x015\x90\x82\x01R`@\x83\x015\x82\x81\x11\x15a\x0EoW`\0\x80\xFD[\x80\x84\x01\x93PP\x86`\x1F\x84\x01\x12a\x0E\x84W`\0\x80\xFD[a\x0E\x93\x87\x845` \x86\x01a\x0BQV[`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\xB9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\xF2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0FBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0F\x1FWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F>W\x82\x81U`\x01\x01a\x0F+V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FaWa\x0Faa\n\xB3V[a\x0Fu\x81a\x0Fo\x84Ta\x0E\xA5V[\x84a\x0E\xF8V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0F\xC8W`\0\x84\x15a\x0F\x92WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0F>V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x10\x15W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\xF6V[P\x85\x82\x10\x15a\x10QW\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84Q\x16\x81R` \x84\x01Q` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x10\xA1`\x80\x83\x01\x84a\rRV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x10\xBCW`\0\x80\xFD[\x81Qa\r\x1F\x81a\x0B\xE5V[`\0\x82\x19\x82\x11\x15a\x11\x01W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\0\x82Qa\x11\x18\x81\x84` \x87\x01a\r&V[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R`@Q`Y8\x03\x80`Y\x839\x81\x01`@\x81\x90R`\x1E\x91`*V[\x80`\x01`\x01`\xA0\x1B\x03\x16\xFF[`\0` \x82\x84\x03\x12\x15`;W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`QW`\0\x80\xFD[\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static FAUCET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Faucet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Faucet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Faucet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Faucet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Faucet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Faucet)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Faucet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FAUCET_ABI.clone(),
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
                FAUCET_ABI.clone(),
                FAUCET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ADMIN` (0x2a0acc6a) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([42, 10, 204, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configure` (0x1cb81b88) function
        pub fn configure(
            &self,
            module: ::ethers::core::types::Address,
            config: ModuleConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 184, 27, 136], (module, config))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `drip` (0xb960bc2b) function
        pub fn drip(
            &self,
            params: DripParameters,
            auth: AuthParameters,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 96, 188, 43], (params, auth))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modules` (0xa8ee49fe) function
        pub fn modules(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::string::String,
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([168, 238, 73, 254], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x1d5d26bc) function
        pub fn nonces(
            &self,
            p0: [u8; 32],
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 93, 38, 188], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timeouts` (0x38757bfd) function
        pub fn timeouts(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([56, 117, 123, 253], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xf3fef3a3) function
        pub fn withdraw(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 254, 243, 163], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Drip` event
        pub fn drip_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DripFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DripFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Faucet<M> {
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
    #[ethevent(name = "Drip", abi = "Drip(string,bytes32,uint256,address)")]
    pub struct DripFilter {
        #[ethevent(indexed)]
        pub auth_module: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub user_id: [u8; 32],
        pub amount: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `ADMIN` function with signature `ADMIN()` and selector `0x2a0acc6a`
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
    #[ethcall(name = "ADMIN", abi = "ADMIN()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `configure` function with signature `configure(address,(string,bool,uint256,uint256))` and selector `0x1cb81b88`
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
        name = "configure",
        abi = "configure(address,(string,bool,uint256,uint256))"
    )]
    pub struct ConfigureCall {
        pub module: ::ethers::core::types::Address,
        pub config: ModuleConfig,
    }
    ///Container type for all input parameters for the `drip` function with signature `drip((address,bytes32),(address,bytes32,bytes))` and selector `0xb960bc2b`
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
    #[ethcall(name = "drip", abi = "drip((address,bytes32),(address,bytes32,bytes))")]
    pub struct DripCall {
        pub params: DripParameters,
        pub auth: AuthParameters,
    }
    ///Container type for all input parameters for the `modules` function with signature `modules(address)` and selector `0xa8ee49fe`
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
    #[ethcall(name = "modules", abi = "modules(address)")]
    pub struct ModulesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `nonces` function with signature `nonces(bytes32,bytes32)` and selector `0x1d5d26bc`
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
    #[ethcall(name = "nonces", abi = "nonces(bytes32,bytes32)")]
    pub struct NoncesCall(pub [u8; 32], pub [u8; 32]);
    ///Container type for all input parameters for the `timeouts` function with signature `timeouts(address,bytes32)` and selector `0x38757bfd`
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
    #[ethcall(name = "timeouts", abi = "timeouts(address,bytes32)")]
    pub struct TimeoutsCall(pub ::ethers::core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256)` and selector `0xf3fef3a3`
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
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256)")]
    pub struct WithdrawCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FaucetCalls {
        Admin(AdminCall),
        Configure(ConfigureCall),
        Drip(DripCall),
        Modules(ModulesCall),
        Nonces(NoncesCall),
        Timeouts(TimeoutsCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for FaucetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) = <ConfigureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Configure(decoded));
            }
            if let Ok(decoded) = <DripCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Drip(decoded));
            }
            if let Ok(decoded) = <ModulesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Modules(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <TimeoutsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Timeouts(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FaucetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Configure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Drip(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Modules(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Timeouts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FaucetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Configure(element) => ::core::fmt::Display::fmt(element, f),
                Self::Drip(element) => ::core::fmt::Display::fmt(element, f),
                Self::Modules(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Timeouts(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminCall> for FaucetCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<ConfigureCall> for FaucetCalls {
        fn from(value: ConfigureCall) -> Self {
            Self::Configure(value)
        }
    }
    impl ::core::convert::From<DripCall> for FaucetCalls {
        fn from(value: DripCall) -> Self {
            Self::Drip(value)
        }
    }
    impl ::core::convert::From<ModulesCall> for FaucetCalls {
        fn from(value: ModulesCall) -> Self {
            Self::Modules(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for FaucetCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<TimeoutsCall> for FaucetCalls {
        fn from(value: TimeoutsCall) -> Self {
            Self::Timeouts(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for FaucetCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `ADMIN` function with signature `ADMIN()` and selector `0x2a0acc6a`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `modules` function with signature `modules(address)` and selector `0xa8ee49fe`
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
    pub struct ModulesReturn {
        pub name: ::std::string::String,
        pub enabled: bool,
        pub ttl: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `nonces` function with signature `nonces(bytes32,bytes32)` and selector `0x1d5d26bc`
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
    pub struct NoncesReturn(pub bool);
    ///Container type for all return fields from the `timeouts` function with signature `timeouts(address,bytes32)` and selector `0x38757bfd`
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
    pub struct TimeoutsReturn(pub ::ethers::core::types::U256);
    ///`AuthParameters(address,bytes32,bytes)`
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
    pub struct AuthParameters {
        pub module: ::ethers::core::types::Address,
        pub id: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
    }
    ///`ModuleConfig(string,bool,uint256,uint256)`
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
    pub struct ModuleConfig {
        pub name: ::std::string::String,
        pub enabled: bool,
        pub ttl: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
}

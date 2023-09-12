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
pub mod resource_metering_user {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasToBurn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_raiseBaseFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("failedLowerBaseFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedLowerBaseFee"),
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
                    ::std::borrow::ToOwned::to_owned("failedMaxGasPerBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedMaxGasPerBlock",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("failedMaxLowerBaseFeePerBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedMaxLowerBaseFeePerBlock",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("failedMaxRaiseBaseFeePerBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedMaxRaiseBaseFeePerBlock",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("failedNeverBelowMinBaseFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedNeverBelowMinBaseFee",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("failedRaiseBaseFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedRaiseBaseFee"),
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
                    ::std::borrow::ToOwned::to_owned("params"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevBaseFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevBoughtGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevBlockNum"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("underflow"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RESOURCEMETERING_USER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pb\0\0ib\0\0oV[b\0\x02SV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\0\x90WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\0\xC0WPb\0\0\xAD0b\0\x01\xA1` \x1Bb\0\x08\xFE\x17` \x1CV[\x15\x80\x15b\0\0\xC0WP`\0T`\xFF\x16`\x01\x14[b\0\x01)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x01MW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x01Wb\0\x01\xB0V[\x80\x15b\0\x01\x9EW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x02\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01 V[`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RC`\x01`\x01`@\x1B\x03\x16\x91\x90\x92\x01\x81\x90R`\x01`\xC0\x1B\x02\x17`\x01UV[a\x1B\xCD\x80b\0\x02c`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01@W`\x005`\xE0\x1C\x80c\xB8_\x9B\xAA\x11a\0\xF8W\x80c\xCF\xF0\xAB\x96\x11a\0\xDDW\x80c\xCF\xF0\xAB\x96\x14a\x02\xC4W\x80c\xDCu\xBE\xC7\x14a\x03XW\x80c\xE4\x12@#\x14a\x03oWa\x01@V[\x80c\xB8_\x9B\xAA\x14a\x02:W\x80c\xCCs\x1B\x02\x14a\x02LWa\x01@V[\x80cd\xB4\xB12\x11a\x01)W\x80cd\xB4\xB12\x14a\x02\x03W\x80c\x8EKI\xB3\x14a\x02\x10W\x80c\x9F\xACh\xCB\x14a\x02%Wa\x01@V[\x80c\x1B\xEB\xFF\t\x14a\x01\xC7W\x80cGY\xC2\xFB\x14a\x01\xF0W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`2Ta\x01\xDB\x90c\x01\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`2Ta\x01\xDB\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90`\xFF\x16\x81V[`2Ta\x01\xDB\x90d\x01\0\0\0\0\x90\x04`\xFF\x16\x81V[a\x028a\x0236`\x04a\x17\x04V[a\x03\x85V[\0[`2Ta\x01\xDB\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x02Ta\x08\xBFV[`@Qa\x01\xE7\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`\x01Ta\x03\x1F\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01\xE7V[`2Ta\x01\xDB\x90f\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90e\x01\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16`\0a\x03\xE3a\x08\xBFV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x04\x04\x91\x90a\x18\x17V[\x90P\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x85\x10\x15a\x04GW`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\x16c\x01\0\0\0\x17\x90U[\x81Qc\xFF\xFF\xFF\xFF\x16\x84\x11\x15a\x04\x82W`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[`\0\x86\x15a\x04\xA6Wa\x04\x9F\x88\x83\x85`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\t\x1AV[\x90Pa\x04\xB5V[a\x04\xB2\x88`\0\x84a\t\x1AV[\x90P[a\x04\xBE\x81a\toV[`\0\x83`@\x01Q`\xFF\x16\x87a\x04\xD3\x91\x90a\x18\x17V[\x90P\x82\x86\x11\x80\x15a\x05\x17WP`\x01Ta\x05\x13\x90\x86\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[`\x01\x14[\x15a\x05\xEBW`2Ta\x01\0\x90\x04`\xFF\x16\x80a\x05FWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10\x15[`2\x80T\x91\x15\x15a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ud\x01\0\0\0\0\x90\x04`\xFF\x16\x80a\x05\xB2WP`\x01T\x81\x90a\x05\xB0\x90\x89\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x10[`2\x80T\x91\x15\x15d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x82\x86\x10\x80a\x06+WP`\x01\x80Ta\x06)\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11[\x15a\x08\xB4W`2Tb\x01\0\0\x90\x04`\xFF\x16\x80a\x06ZWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10[`2\x80T\x91\x15\x15b\x01\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x01Ta\x06\xC5\x90\x86\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16a\x18+V[`\x01\x03a\x07CW`2Te\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x07\x05WP`\x01T\x81\x90a\x07\x02\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xB4V[`\x01\x80Ta\x07x\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11\x15a\x08\xB4W`@\x84\x01Q`\x01Ta\x07\xE9\x91a\x07\xC2\x91\x8A\x91`\xFF\x16\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x90\x03a\t\x84V[\x85``\x01Qc\xFF\xFF\xFF\xFF\x16\x86`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[`2T\x90\x88\x03\x91Pf\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08\x08WP\x86\x81\x11[`2\x80T\x91\x15\x15f\x01\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ue\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08zWP`\x01T\x81\x90a\x08w\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[PPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x08\xF9a\t\xF6V[\x90P\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0a\t'\x84\x84\x84a\n\xB1V[\x90Pa\th`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x0C\xEEV[\x93\x92PPPV[\x80`\0Z\x90Pa\t\x7F\x82\x82a\r\xEEV[PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\t\xC5a\t\x9C\x85\x83a\x18BV[a\t\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xAAV[a\t\xC0\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x1EV[a\x11\x1BV[a\t\xCF\x90\x86a\x19\x1EV[a\t\xD9\x91\x90a\x18BV[\x94\x93PPPPV[`\0a\t\xD9a\t\xF0\x85\x85a\x11LV[\x83a\x11cV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\n\xAB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[\x92\x91PPV[`\0\x81\x83\x11\x15a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0BXWP\x81\x84\x11\x15[\x15a\x0BdWP\x82a\thV[`\0a\x0Bp\x84\x84a\x18+V[a\x0B{\x90`\x01a\x19\xDAV[\x90P`\x03\x85\x11\x15\x80\x15a\x0B\x8DWP\x84\x81\x11[\x15a\x0B\xA4Wa\x0B\x9C\x85\x85a\x19\xDAV[\x91PPa\thV[a\x0B\xCF`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x85\x10\x15\x80\x15a\x0C\x06WPa\x0C\x03\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x81\x11[\x15a\x0C?Wa\x0C5\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[a\x0B\x9C\x90\x84a\x18+V[\x82\x85\x11\x15a\x0C\x95W`\0a\x0CS\x84\x87a\x18+V[\x90P`\0a\x0Ca\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0CvW\x84\x93PPPPa\thV[`\x01a\x0C\x82\x82\x88a\x19\xDAV[a\x0C\x8C\x91\x90a\x18+V[\x93PPPa\x0C\xE6V[\x83\x85\x10\x15a\x0C\xE6W`\0a\x0C\xA9\x86\x86a\x18+V[\x90P`\0a\x0C\xB7\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0C\xCCW\x85\x93PPPPa\thV[a\x0C\xD6\x81\x86a\x18+V[a\x0C\xE1\x90`\x01a\x19\xDAV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\r%\x92\x91\x90a\x1A6V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\r\xA6\x91\x90a\x1A\x8FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\xE1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xE6V[``\x91P[PPPPPPV[`\x01T`\0\x90a\x0E$\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ca\x18+V[\x90P`\0a\x0E0a\t\xF6V[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x0EQ\x91\x90a\x18BV[\x90P\x82\x15a\x0F\x88W`\x01T`\0\x90a\x0E\x88\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\xAAV[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a\x0E\x9F\x91\x90a\x19\x1EV[`\x01Ta\x0E\xBF\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\x1EV[a\x0E\xC9\x91\x90a\x18BV[`\x01T\x90\x91P`\0\x90a\x0F\x1A\x90a\x0E\xF3\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xABV[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[\x90P`\x01\x86\x11\x15a\x0FIWa\x0FFa\x0E\xF3\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa\x0FA\x91\x90a\x18+V[a\t\x84V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a\x0F\xBB\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1B\x1FV[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\x10\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FResourceMetering: cannot buy mor`D\x82\x01R\x7Fe gas than available gas limit\0\0`d\x82\x01R`\x84\x01a\x0B?V[`\x01T`\0\x90a\x10\xCA\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16a\x1BKV[\x90P`\0a\x10\xDCHc;\x9A\xCA\0a\x11rV[a\x10\xE6\x90\x83a\x18\x17V[\x90P`\0Za\x10\xF5\x90\x88a\x18+V[\x90P\x80\x82\x11\x15a\x11\x11Wa\x11\x11a\x11\x0C\x82\x84a\x18+V[a\x11\x82V[PPPPPPPPV[`\0a\thg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x113\x86a\x11\xABV[a\x11=\x91\x90a\x19\x1EV[a\x11G\x91\x90a\x18BV[a\x13\xEFV[`\0\x81\x83\x12\x15a\x11\\W\x81a\thV[P\x90\x91\x90PV[`\0\x81\x83\x12a\x11\\W\x81a\thV[`\0\x81\x83\x10\x15a\x11\\W\x81a\thV[`\0\x80Z\x90P[\x82Za\x11\x95\x90\x83a\x18+V[\x10\x15a\t\x7FWa\x11\xA4\x82a\x1B\x88V[\x91Pa\x11\x89V[`\0\x80\x82\x13a\x12\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[`\0``a\x12#\x84a\x16.V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a\x14 WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x16\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x97W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x825\x91P` \x83\x015\x80\x15\x15\x81\x14a\x17\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x18&Wa\x18&a\x17\xB9V[P\x04\x90V[`\0\x82\x82\x10\x15a\x18=Wa\x18=a\x17\xE8V[P\x03\x90V[`\0\x82a\x18QWa\x18Qa\x17\xB9V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\x18\xA5Wa\x18\xA5a\x17\xE8V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15a\x18\xE4Wa\x18\xE4a\x17\xE8V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a\x19\x18Wa\x19\x18a\x17\xE8V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x19_Wa\x19_a\x17\xE8V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\x19\x9AWa\x19\x9Aa\x17\xE8V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x19\xB6Wa\x19\xB6a\x17\xE8V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x19\xCCWa\x19\xCCa\x17\xE8V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a\x19\xEDWa\x19\xEDa\x17\xE8V[P\x01\x90V[`\0\x82a\x1A\x01Wa\x1A\x01a\x17\xB9V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1A!W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A\tV[\x83\x81\x11\x15a\x1A0W`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x1AU\x81``\x85\x01` \x88\x01a\x1A\x06V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x1A\xA1\x81\x84` \x87\x01a\x1A\x06V[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\x1A\xE5Wa\x1A\xE5a\x17\xE8V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\x1B\x19Wa\x1B\x19a\x17\xE8V[PP\x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1BBWa\x1BBa\x17\xE8V[\x01\x94\x93PPPPV[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x1B\x83Wa\x1B\x83a\x17\xE8V[P\x02\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1B\xB9Wa\x1B\xB9a\x17\xE8V[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static RESOURCEMETERING_USER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01@W`\x005`\xE0\x1C\x80c\xB8_\x9B\xAA\x11a\0\xF8W\x80c\xCF\xF0\xAB\x96\x11a\0\xDDW\x80c\xCF\xF0\xAB\x96\x14a\x02\xC4W\x80c\xDCu\xBE\xC7\x14a\x03XW\x80c\xE4\x12@#\x14a\x03oWa\x01@V[\x80c\xB8_\x9B\xAA\x14a\x02:W\x80c\xCCs\x1B\x02\x14a\x02LWa\x01@V[\x80cd\xB4\xB12\x11a\x01)W\x80cd\xB4\xB12\x14a\x02\x03W\x80c\x8EKI\xB3\x14a\x02\x10W\x80c\x9F\xACh\xCB\x14a\x02%Wa\x01@V[\x80c\x1B\xEB\xFF\t\x14a\x01\xC7W\x80cGY\xC2\xFB\x14a\x01\xF0W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`2Ta\x01\xDB\x90c\x01\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`2Ta\x01\xDB\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90`\xFF\x16\x81V[`2Ta\x01\xDB\x90d\x01\0\0\0\0\x90\x04`\xFF\x16\x81V[a\x028a\x0236`\x04a\x17\x04V[a\x03\x85V[\0[`2Ta\x01\xDB\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x02Ta\x08\xBFV[`@Qa\x01\xE7\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`\x01Ta\x03\x1F\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01\xE7V[`2Ta\x01\xDB\x90f\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`2Ta\x01\xDB\x90e\x01\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16`\0a\x03\xE3a\x08\xBFV[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x04\x04\x91\x90a\x18\x17V[\x90P\x81``\x01Qc\xFF\xFF\xFF\xFF\x16\x85\x10\x15a\x04GW`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\x16c\x01\0\0\0\x17\x90U[\x81Qc\xFF\xFF\xFF\xFF\x16\x84\x11\x15a\x04\x82W`2\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[`\0\x86\x15a\x04\xA6Wa\x04\x9F\x88\x83\x85`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\t\x1AV[\x90Pa\x04\xB5V[a\x04\xB2\x88`\0\x84a\t\x1AV[\x90P[a\x04\xBE\x81a\toV[`\0\x83`@\x01Q`\xFF\x16\x87a\x04\xD3\x91\x90a\x18\x17V[\x90P\x82\x86\x11\x80\x15a\x05\x17WP`\x01Ta\x05\x13\x90\x86\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[`\x01\x14[\x15a\x05\xEBW`2Ta\x01\0\x90\x04`\xFF\x16\x80a\x05FWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10\x15[`2\x80T\x91\x15\x15a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ud\x01\0\0\0\0\x90\x04`\xFF\x16\x80a\x05\xB2WP`\x01T\x81\x90a\x05\xB0\x90\x89\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x10[`2\x80T\x91\x15\x15d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x82\x86\x10\x80a\x06+WP`\x01\x80Ta\x06)\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11[\x15a\x08\xB4W`2Tb\x01\0\0\x90\x04`\xFF\x16\x80a\x06ZWP`\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x10[`2\x80T\x91\x15\x15b\x01\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\x01Ta\x06\xC5\x90\x86\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16a\x18+V[`\x01\x03a\x07CW`2Te\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x07\x05WP`\x01T\x81\x90a\x07\x02\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xB4V[`\x01\x80Ta\x07x\x90\x87\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18+V[\x11\x15a\x08\xB4W`@\x84\x01Q`\x01Ta\x07\xE9\x91a\x07\xC2\x91\x8A\x91`\xFF\x16\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x90\x03a\t\x84V[\x85``\x01Qc\xFF\xFF\xFF\xFF\x16\x86`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[`2T\x90\x88\x03\x91Pf\x01\0\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08\x08WP\x86\x81\x11[`2\x80T\x91\x15\x15f\x01\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90\x81\x90Ue\x01\0\0\0\0\0\x90\x04`\xFF\x16\x80a\x08zWP`\x01T\x81\x90a\x08w\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a\x18+V[\x11\x15[`2\x80T\x91\x15\x15e\x01\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[PPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x08\xF9a\t\xF6V[\x90P\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0a\t'\x84\x84\x84a\n\xB1V[\x90Pa\th`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x0C\xEEV[\x93\x92PPPV[\x80`\0Z\x90Pa\t\x7F\x82\x82a\r\xEEV[PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\t\xC5a\t\x9C\x85\x83a\x18BV[a\t\xAE\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xAAV[a\t\xC0\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x1EV[a\x11\x1BV[a\t\xCF\x90\x86a\x19\x1EV[a\t\xD9\x91\x90a\x18BV[\x94\x93PPPPV[`\0a\t\xD9a\t\xF0\x85\x85a\x11LV[\x83a\x11cV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\n\xAB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[\x92\x91PPV[`\0\x81\x83\x11\x15a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0BXWP\x81\x84\x11\x15[\x15a\x0BdWP\x82a\thV[`\0a\x0Bp\x84\x84a\x18+V[a\x0B{\x90`\x01a\x19\xDAV[\x90P`\x03\x85\x11\x15\x80\x15a\x0B\x8DWP\x84\x81\x11[\x15a\x0B\xA4Wa\x0B\x9C\x85\x85a\x19\xDAV[\x91PPa\thV[a\x0B\xCF`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x85\x10\x15\x80\x15a\x0C\x06WPa\x0C\x03\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[\x81\x11[\x15a\x0C?Wa\x0C5\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x18+V[a\x0B\x9C\x90\x84a\x18+V[\x82\x85\x11\x15a\x0C\x95W`\0a\x0CS\x84\x87a\x18+V[\x90P`\0a\x0Ca\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0CvW\x84\x93PPPPa\thV[`\x01a\x0C\x82\x82\x88a\x19\xDAV[a\x0C\x8C\x91\x90a\x18+V[\x93PPPa\x0C\xE6V[\x83\x85\x10\x15a\x0C\xE6W`\0a\x0C\xA9\x86\x86a\x18+V[\x90P`\0a\x0C\xB7\x83\x83a\x19\xF2V[\x90P\x80`\0\x03a\x0C\xCCW\x85\x93PPPPa\thV[a\x0C\xD6\x81\x86a\x18+V[a\x0C\xE1\x90`\x01a\x19\xDAV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\r%\x92\x91\x90a\x1A6V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\r\xA6\x91\x90a\x1A\x8FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\xE1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xE6V[``\x91P[PPPPPPV[`\x01T`\0\x90a\x0E$\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ca\x18+V[\x90P`\0a\x0E0a\t\xF6V[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\x0EQ\x91\x90a\x18BV[\x90P\x82\x15a\x0F\x88W`\x01T`\0\x90a\x0E\x88\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\xAAV[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a\x0E\x9F\x91\x90a\x19\x1EV[`\x01Ta\x0E\xBF\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\x1EV[a\x0E\xC9\x91\x90a\x18BV[`\x01T\x90\x91P`\0\x90a\x0F\x1A\x90a\x0E\xF3\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xABV[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xE1V[\x90P`\x01\x86\x11\x15a\x0FIWa\x0FFa\x0E\xF3\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa\x0FA\x91\x90a\x18+V[a\t\x84V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a\x0F\xBB\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1B\x1FV[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\x10\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FResourceMetering: cannot buy mor`D\x82\x01R\x7Fe gas than available gas limit\0\0`d\x82\x01R`\x84\x01a\x0B?V[`\x01T`\0\x90a\x10\xCA\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16a\x1BKV[\x90P`\0a\x10\xDCHc;\x9A\xCA\0a\x11rV[a\x10\xE6\x90\x83a\x18\x17V[\x90P`\0Za\x10\xF5\x90\x88a\x18+V[\x90P\x80\x82\x11\x15a\x11\x11Wa\x11\x11a\x11\x0C\x82\x84a\x18+V[a\x11\x82V[PPPPPPPPV[`\0a\thg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x113\x86a\x11\xABV[a\x11=\x91\x90a\x19\x1EV[a\x11G\x91\x90a\x18BV[a\x13\xEFV[`\0\x81\x83\x12\x15a\x11\\W\x81a\thV[P\x90\x91\x90PV[`\0\x81\x83\x12a\x11\\W\x81a\thV[`\0\x81\x83\x10\x15a\x11\\W\x81a\thV[`\0\x80Z\x90P[\x82Za\x11\x95\x90\x83a\x18+V[\x10\x15a\t\x7FWa\x11\xA4\x82a\x1B\x88V[\x91Pa\x11\x89V[`\0\x80\x82\x13a\x12\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[`\0``a\x12#\x84a\x16.V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a\x14 WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x16\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B?V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x97W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x825\x91P` \x83\x015\x80\x15\x15\x81\x14a\x17\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x18&Wa\x18&a\x17\xB9V[P\x04\x90V[`\0\x82\x82\x10\x15a\x18=Wa\x18=a\x17\xE8V[P\x03\x90V[`\0\x82a\x18QWa\x18Qa\x17\xB9V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\x18\xA5Wa\x18\xA5a\x17\xE8V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15a\x18\xE4Wa\x18\xE4a\x17\xE8V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a\x19\x18Wa\x19\x18a\x17\xE8V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x19_Wa\x19_a\x17\xE8V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\x19\x9AWa\x19\x9Aa\x17\xE8V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x19\xB6Wa\x19\xB6a\x17\xE8V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x19\xCCWa\x19\xCCa\x17\xE8V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a\x19\xEDWa\x19\xEDa\x17\xE8V[P\x01\x90V[`\0\x82a\x1A\x01Wa\x1A\x01a\x17\xB9V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1A!W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A\tV[\x83\x81\x11\x15a\x1A0W`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x1AU\x81``\x85\x01` \x88\x01a\x1A\x06V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x1A\xA1\x81\x84` \x87\x01a\x1A\x06V[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\x1A\xE5Wa\x1A\xE5a\x17\xE8V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\x1B\x19Wa\x1B\x19a\x17\xE8V[PP\x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1BBWa\x1BBa\x17\xE8V[\x01\x94\x93PPPPV[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x1B\x83Wa\x1B\x83a\x17\xE8V[P\x02\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1B\xB9Wa\x1B\xB9a\x17\xE8V[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static RESOURCEMETERING_USER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ResourceMetering_User<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ResourceMetering_User<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ResourceMetering_User<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ResourceMetering_User<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ResourceMetering_User<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ResourceMetering_User))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ResourceMetering_User<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESOURCEMETERING_USER_ABI.clone(),
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
                RESOURCEMETERING_USER_ABI.clone(),
                RESOURCEMETERING_USER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `burn` (0x9fac68cb) function
        pub fn burn(
            &self,
            gas_to_burn: ::ethers::core::types::U256,
            raise_base_fee: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 172, 104, 203], (gas_to_burn, raise_base_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedLowerBaseFee` (0x4759c2fb) function
        pub fn failed_lower_base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([71, 89, 194, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMaxGasPerBlock` (0x64b4b132) function
        pub fn failed_max_gas_per_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([100, 180, 177, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMaxLowerBaseFeePerBlock` (0xe4124023) function
        pub fn failed_max_lower_base_fee_per_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 18, 64, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMaxRaiseBaseFeePerBlock` (0x8e4b49b3) function
        pub fn failed_max_raise_base_fee_per_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 75, 73, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedNeverBelowMinBaseFee` (0x1bebff09) function
        pub fn failed_never_below_min_base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 235, 255, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedRaiseBaseFee` (0xb85f9baa) function
        pub fn failed_raise_base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([184, 95, 155, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `params` (0xcff0ab96) function
        pub fn params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u64, u64)> {
            self.0
                .method_hash([207, 240, 171, 150], ())
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
        ///Calls the contract's `underflow` (0xdc75bec7) function
        pub fn underflow(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([220, 117, 190, 199], ())
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ResourceMetering_User<M> {
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
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256,bool)` and selector `0x9fac68cb`
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
    #[ethcall(name = "burn", abi = "burn(uint256,bool)")]
    pub struct BurnCall {
        pub gas_to_burn: ::ethers::core::types::U256,
        pub raise_base_fee: bool,
    }
    ///Container type for all input parameters for the `failedLowerBaseFee` function with signature `failedLowerBaseFee()` and selector `0x4759c2fb`
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
    #[ethcall(name = "failedLowerBaseFee", abi = "failedLowerBaseFee()")]
    pub struct FailedLowerBaseFeeCall;
    ///Container type for all input parameters for the `failedMaxGasPerBlock` function with signature `failedMaxGasPerBlock()` and selector `0x64b4b132`
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
    #[ethcall(name = "failedMaxGasPerBlock", abi = "failedMaxGasPerBlock()")]
    pub struct FailedMaxGasPerBlockCall;
    ///Container type for all input parameters for the `failedMaxLowerBaseFeePerBlock` function with signature `failedMaxLowerBaseFeePerBlock()` and selector `0xe4124023`
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
        name = "failedMaxLowerBaseFeePerBlock",
        abi = "failedMaxLowerBaseFeePerBlock()"
    )]
    pub struct FailedMaxLowerBaseFeePerBlockCall;
    ///Container type for all input parameters for the `failedMaxRaiseBaseFeePerBlock` function with signature `failedMaxRaiseBaseFeePerBlock()` and selector `0x8e4b49b3`
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
        name = "failedMaxRaiseBaseFeePerBlock",
        abi = "failedMaxRaiseBaseFeePerBlock()"
    )]
    pub struct FailedMaxRaiseBaseFeePerBlockCall;
    ///Container type for all input parameters for the `failedNeverBelowMinBaseFee` function with signature `failedNeverBelowMinBaseFee()` and selector `0x1bebff09`
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
    #[ethcall(name = "failedNeverBelowMinBaseFee", abi = "failedNeverBelowMinBaseFee()")]
    pub struct FailedNeverBelowMinBaseFeeCall;
    ///Container type for all input parameters for the `failedRaiseBaseFee` function with signature `failedRaiseBaseFee()` and selector `0xb85f9baa`
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
    #[ethcall(name = "failedRaiseBaseFee", abi = "failedRaiseBaseFee()")]
    pub struct FailedRaiseBaseFeeCall;
    ///Container type for all input parameters for the `params` function with signature `params()` and selector `0xcff0ab96`
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
    #[ethcall(name = "params", abi = "params()")]
    pub struct ParamsCall;
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
    ///Container type for all input parameters for the `underflow` function with signature `underflow()` and selector `0xdc75bec7`
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
    #[ethcall(name = "underflow", abi = "underflow()")]
    pub struct UnderflowCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ResourceMetering_UserCalls {
        Burn(BurnCall),
        FailedLowerBaseFee(FailedLowerBaseFeeCall),
        FailedMaxGasPerBlock(FailedMaxGasPerBlockCall),
        FailedMaxLowerBaseFeePerBlock(FailedMaxLowerBaseFeePerBlockCall),
        FailedMaxRaiseBaseFeePerBlock(FailedMaxRaiseBaseFeePerBlockCall),
        FailedNeverBelowMinBaseFee(FailedNeverBelowMinBaseFeeCall),
        FailedRaiseBaseFee(FailedRaiseBaseFeeCall),
        Params(ParamsCall),
        ResourceConfig(ResourceConfigCall),
        Underflow(UnderflowCall),
    }
    impl ::ethers::core::abi::AbiDecode for ResourceMetering_UserCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <FailedLowerBaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedLowerBaseFee(decoded));
            }
            if let Ok(decoded) = <FailedMaxGasPerBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedMaxGasPerBlock(decoded));
            }
            if let Ok(decoded) = <FailedMaxLowerBaseFeePerBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedMaxLowerBaseFeePerBlock(decoded));
            }
            if let Ok(decoded) = <FailedMaxRaiseBaseFeePerBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedMaxRaiseBaseFeePerBlock(decoded));
            }
            if let Ok(decoded) = <FailedNeverBelowMinBaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedNeverBelowMinBaseFee(decoded));
            }
            if let Ok(decoded) = <FailedRaiseBaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedRaiseBaseFee(decoded));
            }
            if let Ok(decoded) = <ParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Params(decoded));
            }
            if let Ok(decoded) = <ResourceConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResourceConfig(decoded));
            }
            if let Ok(decoded) = <UnderflowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Underflow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ResourceMetering_UserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedLowerBaseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedMaxGasPerBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedMaxLowerBaseFeePerBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedMaxRaiseBaseFeePerBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedNeverBelowMinBaseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedRaiseBaseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Params(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResourceConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Underflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ResourceMetering_UserCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedLowerBaseFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedMaxGasPerBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedMaxLowerBaseFeePerBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedMaxRaiseBaseFeePerBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedNeverBelowMinBaseFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedRaiseBaseFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Params(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResourceConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Underflow(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnCall> for ResourceMetering_UserCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<FailedLowerBaseFeeCall> for ResourceMetering_UserCalls {
        fn from(value: FailedLowerBaseFeeCall) -> Self {
            Self::FailedLowerBaseFee(value)
        }
    }
    impl ::core::convert::From<FailedMaxGasPerBlockCall> for ResourceMetering_UserCalls {
        fn from(value: FailedMaxGasPerBlockCall) -> Self {
            Self::FailedMaxGasPerBlock(value)
        }
    }
    impl ::core::convert::From<FailedMaxLowerBaseFeePerBlockCall>
    for ResourceMetering_UserCalls {
        fn from(value: FailedMaxLowerBaseFeePerBlockCall) -> Self {
            Self::FailedMaxLowerBaseFeePerBlock(value)
        }
    }
    impl ::core::convert::From<FailedMaxRaiseBaseFeePerBlockCall>
    for ResourceMetering_UserCalls {
        fn from(value: FailedMaxRaiseBaseFeePerBlockCall) -> Self {
            Self::FailedMaxRaiseBaseFeePerBlock(value)
        }
    }
    impl ::core::convert::From<FailedNeverBelowMinBaseFeeCall>
    for ResourceMetering_UserCalls {
        fn from(value: FailedNeverBelowMinBaseFeeCall) -> Self {
            Self::FailedNeverBelowMinBaseFee(value)
        }
    }
    impl ::core::convert::From<FailedRaiseBaseFeeCall> for ResourceMetering_UserCalls {
        fn from(value: FailedRaiseBaseFeeCall) -> Self {
            Self::FailedRaiseBaseFee(value)
        }
    }
    impl ::core::convert::From<ParamsCall> for ResourceMetering_UserCalls {
        fn from(value: ParamsCall) -> Self {
            Self::Params(value)
        }
    }
    impl ::core::convert::From<ResourceConfigCall> for ResourceMetering_UserCalls {
        fn from(value: ResourceConfigCall) -> Self {
            Self::ResourceConfig(value)
        }
    }
    impl ::core::convert::From<UnderflowCall> for ResourceMetering_UserCalls {
        fn from(value: UnderflowCall) -> Self {
            Self::Underflow(value)
        }
    }
    ///Container type for all return fields from the `failedLowerBaseFee` function with signature `failedLowerBaseFee()` and selector `0x4759c2fb`
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
    pub struct FailedLowerBaseFeeReturn(pub bool);
    ///Container type for all return fields from the `failedMaxGasPerBlock` function with signature `failedMaxGasPerBlock()` and selector `0x64b4b132`
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
    pub struct FailedMaxGasPerBlockReturn(pub bool);
    ///Container type for all return fields from the `failedMaxLowerBaseFeePerBlock` function with signature `failedMaxLowerBaseFeePerBlock()` and selector `0xe4124023`
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
    pub struct FailedMaxLowerBaseFeePerBlockReturn(pub bool);
    ///Container type for all return fields from the `failedMaxRaiseBaseFeePerBlock` function with signature `failedMaxRaiseBaseFeePerBlock()` and selector `0x8e4b49b3`
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
    pub struct FailedMaxRaiseBaseFeePerBlockReturn(pub bool);
    ///Container type for all return fields from the `failedNeverBelowMinBaseFee` function with signature `failedNeverBelowMinBaseFee()` and selector `0x1bebff09`
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
    pub struct FailedNeverBelowMinBaseFeeReturn(pub bool);
    ///Container type for all return fields from the `failedRaiseBaseFee` function with signature `failedRaiseBaseFee()` and selector `0xb85f9baa`
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
    pub struct FailedRaiseBaseFeeReturn(pub bool);
    ///Container type for all return fields from the `params` function with signature `params()` and selector `0xcff0ab96`
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
    pub struct ParamsReturn {
        pub prev_base_fee: u128,
        pub prev_bought_gas: u64,
        pub prev_block_num: u64,
    }
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
    ///Container type for all return fields from the `underflow` function with signature `underflow()` and selector `0xdc75bec7`
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
    pub struct UnderflowReturn(pub bool);
}

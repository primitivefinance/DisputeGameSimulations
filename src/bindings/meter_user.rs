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
pub mod meter_user {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("innerConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("innerConfig"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxResourceLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "elasticityMultiplier",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "baseFeeMaxChangeDenominator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumBaseFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("systemTxMaxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maximumBaseFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_prevBaseFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_prevBoughtGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_prevBlockNum"),
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
                    ::std::borrow::ToOwned::to_owned("setParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newConfig"),
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
                    ::std::borrow::ToOwned::to_owned("use"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("use"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
    pub static METERUSER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pb\0\0ib\0\x01EV[b\0\0~b\0\x02w` \x1Bb\0\x08>\x17` \x1CV[\x80Q`2\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17d\xFF\xFF\xFF\xFF\xFF`(\x1B\x19\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02c\xFF\xFF\xFF\xFF`0\x1B\x19\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17`\x01`P\x1B`\x01`\xF0\x1B\x03\x19\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02`\x01`p\x1B`\x01`\xF0\x1B\x03\x19\x16\x92\x90\x92\x17`\x01`p\x1B`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x02\x17\x90Ub\0\x03\xA0V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x01fWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x01\x96WPb\0\x01\x830b\0\x02\xEE` \x1Bb\0\x08\xBE\x17` \x1CV[\x15\x80\x15b\0\x01\x96WP`\0T`\xFF\x16`\x01\x14[b\0\x01\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x02#W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x02-b\0\x02\xFDV[\x80\x15b\0\x02tW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01R`\x01`\x01`\x80\x1B\x03`\xA0\x82\x01R\x90V[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x03jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01\xF6V[`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RC`\x01`\x01`@\x1B\x03\x16\x91\x90\x92\x01\x81\x90R`\x01`\xC0\x1B\x02\x17`\x01UV[a\x19Z\x80b\0\x03\xB0`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xFFW`\x005`\xE0\x1C\x80c\xB4\xB6O\xBB\x11a\0\xDDW\x80c\xB4\xB6O\xBB\x14a\x03 W\x80c\xB7t)\xA0\x14a\x033W\x80c\xCCs\x1B\x02\x14a\x04\x8BW\x80c\xCF\xF0\xAB\x96\x14a\x05\x03Wa\0\xFFV[\x80co\x16\x96\x06\x14a\x01\x86W\x80c\x81\x17\x1E\xF8\x14a\x02RW\x80c\x81)\xFC\x1C\x14a\x03\x18W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02Pa\x01\x946`\x04a\x14\xC5V[`@\x80Q``\x81\x01\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x80\x85Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01\x81\x90R\x92\x90\x93\x16\x93\x01\x83\x90R`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x92\x17p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x02\x17w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x02\x91\x90\x91\x17\x90UV[\0[`2Ta\x02\xC1\x90c\xFF\xFF\xFF\xFF\x80\x82\x16\x91`\xFFd\x01\0\0\0\0\x82\x04\x81\x16\x92e\x01\0\0\0\0\0\x83\x04\x90\x91\x16\x91f\x01\0\0\0\0\0\0\x81\x04\x82\x16\x91j\x01\0\0\0\0\0\0\0\0\0\0\x82\x04\x16\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x86V[`@\x80Qc\xFF\xFF\xFF\xFF\x97\x88\x16\x81R`\xFF\x96\x87\x16` \x82\x01R\x94\x90\x95\x16\x94\x84\x01\x94\x90\x94R\x90\x84\x16``\x83\x01R\x90\x92\x16`\x80\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01[`@Q\x80\x91\x03\x90\xF3[a\x02Pa\x05\x97V[a\x02Pa\x03.6`\x04a\x15\x0BV[a\x07.V[a\x02Pa\x03A6`\x04a\x15NV[\x80Q`2\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x02\x17\x90UV[a\x04\x93a\x07CV[`@Qa\x03\x0F\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`\x01Ta\x05^\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x03\x0FV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\xB7WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x05\xD1WP0;\x15\x80\x15a\x05\xD1WP`\0T`\xFF\x16`\x01\x14[a\x06bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x06\xC0W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x06\xC8a\x08\xDAV[\x80\x15a\x07+W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[\x80`\0Z\x90Pa\x07>\x82\x82a\t\xBDV[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x089`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`2Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16a\tqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06YV[`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90\x92\x01\x81\x90Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x17`\x01UV[`\x01T`\0\x90a\t\xF3\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ca\x16FV[\x90P`\0a\n\xBB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`2Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\n\xDC\x91\x90a\x16\x8CV[\x90P\x82\x15a\x0C\x13W`\x01T`\0\x90a\x0B\x13\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x16\xF4V[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a\x0B*\x91\x90a\x17hV[`\x01Ta\x0BJ\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17hV[a\x0BT\x91\x90a\x16\x8CV[`\x01T\x90\x91P`\0\x90a\x0B\xA5\x90a\x0B~\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18$V[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\r\xA6V[\x90P`\x01\x86\x11\x15a\x0B\xD4Wa\x0B\xD1a\x0B~\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa\x0B\xCC\x91\x90a\x16FV[a\r\xC3V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a\x0CF\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x98V[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\r)W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FResourceMetering: cannot buy mor`D\x82\x01R\x7Fe gas than available gas limit\0\0`d\x82\x01R`\x84\x01a\x06YV[`\x01T`\0\x90a\rU\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16a\x18\xC4V[\x90P`\0a\rgHc;\x9A\xCA\0a\x0E\x18V[a\rq\x90\x83a\x19\x01V[\x90P`\0Za\r\x80\x90\x88a\x16FV[\x90P\x80\x82\x11\x15a\r\x9CWa\r\x9Ca\r\x97\x82\x84a\x16FV[a\x0E1V[PPPPPPPPV[`\0a\r\xBBa\r\xB5\x85\x85a\x0EZV[\x83a\x0EjV[\x94\x93PPPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x04a\r\xDB\x85\x83a\x16\x8CV[a\r\xED\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xF4V[a\r\xFF\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x17hV[a\x0EyV[a\x0E\x0E\x90\x86a\x17hV[a\r\xBB\x91\x90a\x16\x8CV[`\0\x81\x83\x10\x15a\x0E(W\x81a\x0E*V[\x82[\x93\x92PPPV[`\0\x80Z\x90P[\x82Za\x0ED\x90\x83a\x16FV[\x10\x15a\x07>Wa\x0ES\x82a\x19\x15V[\x91Pa\x0E8V[`\0\x81\x83\x12\x15a\x0E(W\x81a\x0E*V[`\0\x81\x83\x12a\x0E(W\x81a\x0E*V[`\0a\x0E*g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0E\x91\x86a\x0E\xAAV[a\x0E\x9B\x91\x90a\x17hV[a\x0E\xA5\x91\x90a\x16\x8CV[a\x10\xEEV[`\0\x80\x82\x13a\x0F\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06YV[`\0``a\x0F\"\x84a\x13-V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a\x11\x1FWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x11\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06YV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x13\x98W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06YV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xDDWa\x14\xDDa\x14\x03V[a\x14\xE6\x84a\x14\x88V[\x92Pa\x14\xF4` \x85\x01a\x14\xADV[\x91Pa\x15\x02`@\x85\x01a\x14\xADV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x15 Wa\x15 a\x14\x03V[a\x0E*\x82a\x14\xADV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\x15cWa\x15ca\x14\x03V[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x15\xADW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Ra\x15\xB9\x83a\x15)V[\x81Ra\x15\xC7` \x84\x01a\x15=V[` \x82\x01Ra\x15\xD8`@\x84\x01a\x15=V[`@\x82\x01Ra\x15\xE9``\x84\x01a\x15)V[``\x82\x01Ra\x15\xFA`\x80\x84\x01a\x15)V[`\x80\x82\x01Ra\x16\x0B`\xA0\x84\x01a\x14\x88V[`\xA0\x82\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x16XWa\x16Xa\x16\x17V[P\x03\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x16\x9BWa\x16\x9Ba\x16]V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\x16\xEFWa\x16\xEFa\x16\x17V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15a\x17.Wa\x17.a\x16\x17V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a\x17bWa\x17ba\x16\x17V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x17\xA9Wa\x17\xA9a\x16\x17V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\x17\xE4Wa\x17\xE4a\x16\x17V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x18\0Wa\x18\0a\x16\x17V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x18\x16Wa\x18\x16a\x16\x17V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\x18^Wa\x18^a\x16\x17V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\x18\x92Wa\x18\x92a\x16\x17V[PP\x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x18\xBBWa\x18\xBBa\x16\x17V[\x01\x94\x93PPPPV[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x18\xFCWa\x18\xFCa\x16\x17V[P\x02\x90V[`\0\x82a\x19\x10Wa\x19\x10a\x16]V[P\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x19FWa\x19Fa\x16\x17V[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static METERUSER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xFFW`\x005`\xE0\x1C\x80c\xB4\xB6O\xBB\x11a\0\xDDW\x80c\xB4\xB6O\xBB\x14a\x03 W\x80c\xB7t)\xA0\x14a\x033W\x80c\xCCs\x1B\x02\x14a\x04\x8BW\x80c\xCF\xF0\xAB\x96\x14a\x05\x03Wa\0\xFFV[\x80co\x16\x96\x06\x14a\x01\x86W\x80c\x81\x17\x1E\xF8\x14a\x02RW\x80c\x81)\xFC\x1C\x14a\x03\x18W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02Pa\x01\x946`\x04a\x14\xC5V[`@\x80Q``\x81\x01\x82Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x80\x85Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01\x81\x90R\x92\x90\x93\x16\x93\x01\x83\x90R`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x92\x17p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x02\x17w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x02\x91\x90\x91\x17\x90UV[\0[`2Ta\x02\xC1\x90c\xFF\xFF\xFF\xFF\x80\x82\x16\x91`\xFFd\x01\0\0\0\0\x82\x04\x81\x16\x92e\x01\0\0\0\0\0\x83\x04\x90\x91\x16\x91f\x01\0\0\0\0\0\0\x81\x04\x82\x16\x91j\x01\0\0\0\0\0\0\0\0\0\0\x82\x04\x16\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x86V[`@\x80Qc\xFF\xFF\xFF\xFF\x97\x88\x16\x81R`\xFF\x96\x87\x16` \x82\x01R\x94\x90\x95\x16\x94\x84\x01\x94\x90\x94R\x90\x84\x16``\x83\x01R\x90\x92\x16`\x80\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01[`@Q\x80\x91\x03\x90\xF3[a\x02Pa\x05\x97V[a\x02Pa\x03.6`\x04a\x15\x0BV[a\x07.V[a\x02Pa\x03A6`\x04a\x15NV[\x80Q`2\x80T` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x90\x97\x01Qc\xFF\xFF\xFF\xFF\x96\x87\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0`\xFF\x94\x85\x16\x02\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\x16e\x01\0\0\0\0\0\x93\x90\x92\x16\x92\x90\x92\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17f\x01\0\0\0\0\0\0\x91\x85\x16\x91\x90\x91\x02\x17\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16j\x01\0\0\0\0\0\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x02\x17\x90UV[a\x04\x93a\x07CV[`@Qa\x03\x0F\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[`\x01Ta\x05^\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x03\x0FV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\xB7WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x05\xD1WP0;\x15\x80\x15a\x05\xD1WP`\0T`\xFF\x16`\x01\x14[a\x06bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x06\xC0W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x06\xC8a\x08\xDAV[\x80\x15a\x07+W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[\x80`\0Z\x90Pa\x07>\x82\x82a\t\xBDV[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x089`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`2Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16a\tqW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06YV[`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90\x92\x01\x81\x90Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x17`\x01UV[`\x01T`\0\x90a\t\xF3\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ca\x16FV[\x90P`\0a\n\xBB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82R`2Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rd\x01\0\0\0\0\x82\x04`\xFF\x90\x81\x16` \x85\x01Re\x01\0\0\0\0\0\x83\x04\x16\x93\x83\x01\x93\x90\x93Rf\x01\0\0\0\0\0\0\x81\x04\x83\x16``\x83\x01Rj\x01\0\0\0\0\0\0\0\0\0\0\x81\x04\x90\x92\x16`\x80\x82\x01Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R\x90V[\x90P`\0\x81` \x01Q`\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16a\n\xDC\x91\x90a\x16\x8CV[\x90P\x82\x15a\x0C\x13W`\x01T`\0\x90a\x0B\x13\x90\x83\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x16\xF4V[\x90P`\0\x83`@\x01Q`\xFF\x16\x83a\x0B*\x91\x90a\x17hV[`\x01Ta\x0BJ\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17hV[a\x0BT\x91\x90a\x16\x8CV[`\x01T\x90\x91P`\0\x90a\x0B\xA5\x90a\x0B~\x90\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18$V[\x86``\x01Qc\xFF\xFF\xFF\xFF\x16\x87`\xA0\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\r\xA6V[\x90P`\x01\x86\x11\x15a\x0B\xD4Wa\x0B\xD1a\x0B~\x82\x87`@\x01Q`\xFF\x16`\x01\x8Aa\x0B\xCC\x91\x90a\x16FV[a\r\xC3V[\x90P[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16\x02\x17`\x01UPP[`\x01\x80T\x86\x91\x90`\x10\x90a\x0CF\x90\x84\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x98V[\x92Pa\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\0\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x13\x15a\r)W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FResourceMetering: cannot buy mor`D\x82\x01R\x7Fe gas than available gas limit\0\0`d\x82\x01R`\x84\x01a\x06YV[`\x01T`\0\x90a\rU\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16a\x18\xC4V[\x90P`\0a\rgHc;\x9A\xCA\0a\x0E\x18V[a\rq\x90\x83a\x19\x01V[\x90P`\0Za\r\x80\x90\x88a\x16FV[\x90P\x80\x82\x11\x15a\r\x9CWa\r\x9Ca\r\x97\x82\x84a\x16FV[a\x0E1V[PPPPPPPPV[`\0a\r\xBBa\r\xB5\x85\x85a\x0EZV[\x83a\x0EjV[\x94\x93PPPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x04a\r\xDB\x85\x83a\x16\x8CV[a\r\xED\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xF4V[a\r\xFF\x85g\r\xE0\xB6\xB3\xA7d\0\0a\x17hV[a\x0EyV[a\x0E\x0E\x90\x86a\x17hV[a\r\xBB\x91\x90a\x16\x8CV[`\0\x81\x83\x10\x15a\x0E(W\x81a\x0E*V[\x82[\x93\x92PPPV[`\0\x80Z\x90P[\x82Za\x0ED\x90\x83a\x16FV[\x10\x15a\x07>Wa\x0ES\x82a\x19\x15V[\x91Pa\x0E8V[`\0\x81\x83\x12\x15a\x0E(W\x81a\x0E*V[`\0\x81\x83\x12a\x0E(W\x81a\x0E*V[`\0a\x0E*g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0E\x91\x86a\x0E\xAAV[a\x0E\x9B\x91\x90a\x17hV[a\x0E\xA5\x91\x90a\x16\x8CV[a\x10\xEEV[`\0\x80\x82\x13a\x0F\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06YV[`\0``a\x0F\"\x84a\x13-V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xB71\xC9X\xF3M\x94\xC1\x82\x13a\x11\x1FWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x11\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FEXP_OVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06YV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x13\x98W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\t`$\x82\x01R\x7FUNDEFINED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06YV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xDDWa\x14\xDDa\x14\x03V[a\x14\xE6\x84a\x14\x88V[\x92Pa\x14\xF4` \x85\x01a\x14\xADV[\x91Pa\x15\x02`@\x85\x01a\x14\xADV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x15 Wa\x15 a\x14\x03V[a\x0E*\x82a\x14\xADV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x14\xA8W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\x15cWa\x15ca\x14\x03V[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x15\xADW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Ra\x15\xB9\x83a\x15)V[\x81Ra\x15\xC7` \x84\x01a\x15=V[` \x82\x01Ra\x15\xD8`@\x84\x01a\x15=V[`@\x82\x01Ra\x15\xE9``\x84\x01a\x15)V[``\x82\x01Ra\x15\xFA`\x80\x84\x01a\x15)V[`\x80\x82\x01Ra\x16\x0B`\xA0\x84\x01a\x14\x88V[`\xA0\x82\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x16XWa\x16Xa\x16\x17V[P\x03\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x16\x9BWa\x16\x9Ba\x16]V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x14\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15a\x16\xEFWa\x16\xEFa\x16\x17V[P\x05\x90V[`\0\x80\x83\x12\x83\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x83\x12\x81\x15\x16\x15a\x17.Wa\x17.a\x16\x17V[\x83\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x83\x13\x81\x16\x15a\x17bWa\x17ba\x16\x17V[PP\x03\x90V[`\0\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x17\xA9Wa\x17\xA9a\x16\x17V[\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x87\x12\x86\x82\x05\x88\x12\x81\x84\x16\x16\x15a\x17\xE4Wa\x17\xE4a\x16\x17V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x18\0Wa\x18\0a\x16\x17V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x18\x16Wa\x18\x16a\x16\x17V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x80\x82\x12\x82\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x84\x13\x81\x15\x16\x15a\x18^Wa\x18^a\x16\x17V[\x82\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x84\x12\x81\x16\x15a\x18\x92Wa\x18\x92a\x16\x17V[PP\x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x18\xBBWa\x18\xBBa\x16\x17V[\x01\x94\x93PPPPV[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x18\xFCWa\x18\xFCa\x16\x17V[P\x02\x90V[`\0\x82a\x19\x10Wa\x19\x10a\x16]V[P\x04\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x19FWa\x19Fa\x16\x17V[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static METERUSER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MeterUser<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MeterUser<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MeterUser<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MeterUser<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MeterUser<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MeterUser)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MeterUser<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    METERUSER_ABI.clone(),
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
                METERUSER_ABI.clone(),
                METERUSER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `innerConfig` (0x81171ef8) function
        pub fn inner_config(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u32, u8, u8, u32, u32, u128),
        > {
            self.0
                .method_hash([129, 23, 30, 248], ())
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
        ///Calls the contract's `set` (0x6f169606) function
        pub fn set(
            &self,
            prev_base_fee: u128,
            prev_bought_gas: u64,
            prev_block_num: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [111, 22, 150, 6],
                    (prev_base_fee, prev_bought_gas, prev_block_num),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setParams` (0xb77429a0) function
        pub fn set_params(
            &self,
            new_config: ResourceConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 116, 41, 160], (new_config,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `use` (0xb4b64fbb) function
        pub fn use_(
            &self,
            amount: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 182, 79, 187], amount)
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
    for MeterUser<M> {
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
    ///Container type for all input parameters for the `innerConfig` function with signature `innerConfig()` and selector `0x81171ef8`
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
    #[ethcall(name = "innerConfig", abi = "innerConfig()")]
    pub struct InnerConfigCall;
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
    ///Container type for all input parameters for the `set` function with signature `set(uint128,uint64,uint64)` and selector `0x6f169606`
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
    #[ethcall(name = "set", abi = "set(uint128,uint64,uint64)")]
    pub struct SetCall {
        pub prev_base_fee: u128,
        pub prev_bought_gas: u64,
        pub prev_block_num: u64,
    }
    ///Container type for all input parameters for the `setParams` function with signature `setParams((uint32,uint8,uint8,uint32,uint32,uint128))` and selector `0xb77429a0`
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
        name = "setParams",
        abi = "setParams((uint32,uint8,uint8,uint32,uint32,uint128))"
    )]
    pub struct SetParamsCall {
        pub new_config: ResourceConfig,
    }
    ///Container type for all input parameters for the `use` function with signature `use(uint64)` and selector `0xb4b64fbb`
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
    #[ethcall(name = "use", abi = "use(uint64)")]
    pub struct UseCall {
        pub amount: u64,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MeterUserCalls {
        Initialize(InitializeCall),
        InnerConfig(InnerConfigCall),
        Params(ParamsCall),
        ResourceConfig(ResourceConfigCall),
        Set(SetCall),
        SetParams(SetParamsCall),
        Use(UseCall),
    }
    impl ::ethers::core::abi::AbiDecode for MeterUserCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <InnerConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InnerConfig(decoded));
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
            if let Ok(decoded) = <SetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Set(decoded));
            }
            if let Ok(decoded) = <SetParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetParams(decoded));
            }
            if let Ok(decoded) = <UseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Use(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MeterUserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InnerConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Params(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResourceConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Set(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Use(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MeterUserCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::InnerConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Params(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResourceConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Set(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Use(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializeCall> for MeterUserCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InnerConfigCall> for MeterUserCalls {
        fn from(value: InnerConfigCall) -> Self {
            Self::InnerConfig(value)
        }
    }
    impl ::core::convert::From<ParamsCall> for MeterUserCalls {
        fn from(value: ParamsCall) -> Self {
            Self::Params(value)
        }
    }
    impl ::core::convert::From<ResourceConfigCall> for MeterUserCalls {
        fn from(value: ResourceConfigCall) -> Self {
            Self::ResourceConfig(value)
        }
    }
    impl ::core::convert::From<SetCall> for MeterUserCalls {
        fn from(value: SetCall) -> Self {
            Self::Set(value)
        }
    }
    impl ::core::convert::From<SetParamsCall> for MeterUserCalls {
        fn from(value: SetParamsCall) -> Self {
            Self::SetParams(value)
        }
    }
    impl ::core::convert::From<UseCall> for MeterUserCalls {
        fn from(value: UseCall) -> Self {
            Self::Use(value)
        }
    }
    ///Container type for all return fields from the `innerConfig` function with signature `innerConfig()` and selector `0x81171ef8`
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
    pub struct InnerConfigReturn {
        pub max_resource_limit: u32,
        pub elasticity_multiplier: u8,
        pub base_fee_max_change_denominator: u8,
        pub minimum_base_fee: u32,
        pub system_tx_max_gas: u32,
        pub maximum_base_fee: u128,
    }
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
}

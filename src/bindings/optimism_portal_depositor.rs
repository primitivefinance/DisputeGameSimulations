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
pub mod optimism_portal_depositor {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vm"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Vm"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_portal"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract OptimismPortal"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("depositTransactionCompletes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "depositTransactionCompletes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_isCreation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failedToComplete"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedToComplete"),
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
    pub static OPTIMISMPORTAL_DEPOSITOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x18\x108\x03\x80b\0\x18\x10\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x02\xBDV[`2\x80T`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`3\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90Ub\0\0\xBBb\0\0\xC3V[PPb\0\x03GV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\0\xE4WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x01\x14WPb\0\x01\x010b\0\x01\xF5` \x1Bb\0\nu\x17` \x1CV[\x15\x80\x15b\0\x01\x14WP`\0T`\xFF\x16`\x01\x14[b\0\x01}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x01\xA1W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[b\0\x01\xABb\0\x02\x04V[\x80\x15b\0\x01\xF2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16b\0\x02qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x01tV[`@\x80Q``\x81\x01\x82Rc;\x9A\xCA\0\x80\x82R`\0` \x83\x01RC`\x01`\x01`@\x1B\x03\x16\x91\x90\x92\x01\x81\x90R`\x01`\xC0\x1B\x02\x17`\x01UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0\x03\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x82Qb\0\x03)\x81b\0\x02\xA7V[` \x84\x01Q\x90\x92Pb\0\x03<\x81b\0\x02\xA7V[\x80\x91PP\x92P\x92\x90PV[a\x14\xB9\x80b\0\x03W`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80c\x99\xFC\xA4/\x14a\0\xC6W\x80c\xCCs\x1B\x02\x14a\x01\x8FW\x80c\xCF\xF0\xAB\x96\x14a\x02\x96W\x80c\xFC\xF9c\xCD\x14a\x03\xB9W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x01TW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`3Ta\x01z\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02&a\x03\xCEV[`@Qa\x01\x86\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[4\x80\x15a\x03$W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x01Ta\x03\x80\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01\x86V[a\x03\xCCa\x03\xC76`\x04a\x11^V[a\x04\rV[\0[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x04\x08a\n\x91V[\x90P\x90V[`2Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cLc\xE5b\x83\x15\x80a\x04LWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x15[\x80\x15a\x04\\WPb\x01\xD4\xC0\x83Q\x11\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04|\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05*W=`\0\x80>=`\0\xFD[PPPP`\0a\x05N\x85`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16a\x0BLV[`2T`@Q\x7F\xC8\x8A^m\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC8\x8A^m\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06WW=`\0\x80>=`\0\xFD[PPPP`\0G\x90P`\0a\x06n\x83`\0\x84a\x0BLV[\x90P`\0`3`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCF\xF0\xAB\x96`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x94\x91\x90a\x12~V[P\x91PP`\0a\x07\xA2a\x03\xCEV[\x80Q`3T\x88Q`@Q\x7F\xA3]\x99\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\x04\x82\x01R\x93\x94Pc\xFF\xFF\xFF\xFF\x90\x92\x16\x92`\0\x92a\t\x10\x92\x90\x8D\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xA3]\x99\xDF\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xB3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xEB\x91\x90a\x12\xE3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85a\t\x0B\x91\x90a\x132V[a\x0BLV[\x90P`3`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE9\xE0\\B\x86\x8E\x88\x85\x8E\x8E`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tv\x95\x94\x93\x92\x91\x90a\x13\xC3V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\n\x11W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x93PPPP\x80\x15a\n#WP`\x01[a\ngW`3\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U[PPPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\x0BF`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[\x92\x91PPV[`\0a\x0BY\x84\x84\x84a\x0B\xA1V[\x90Pa\x0B\x9A`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\r\xDDV[\x93\x92PPPV[`\0\x81\x83\x11\x15a\x0C7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0CGWP\x81\x84\x11\x15[\x15a\x0CSWP\x82a\x0B\x9AV[`\0a\x0C_\x84\x84a\x132V[a\x0Cj\x90`\x01a\x14\x1BV[\x90P`\x03\x85\x11\x15\x80\x15a\x0C|WP\x84\x81\x11[\x15a\x0C\x93Wa\x0C\x8B\x85\x85a\x14\x1BV[\x91PPa\x0B\x9AV[a\x0C\xBE`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x132V[\x85\x10\x15\x80\x15a\x0C\xF5WPa\x0C\xF2\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x132V[\x81\x11[\x15a\r.Wa\r$\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x132V[a\x0C\x8B\x90\x84a\x132V[\x82\x85\x11\x15a\r\x84W`\0a\rB\x84\x87a\x132V[\x90P`\0a\rP\x83\x83a\x143V[\x90P\x80`\0\x03a\reW\x84\x93PPPPa\x0B\x9AV[`\x01a\rq\x82\x88a\x14\x1BV[a\r{\x91\x90a\x132V[\x93PPPa\r\xD5V[\x83\x85\x10\x15a\r\xD5W`\0a\r\x98\x86\x86a\x132V[\x90P`\0a\r\xA6\x83\x83a\x143V[\x90P\x80`\0\x03a\r\xBBW\x85\x93PPPPa\x0B\x9AV[a\r\xC5\x81\x86a\x132V[a\r\xD0\x90`\x01a\x14\x1BV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x0E\x14\x92\x91\x90a\x14nV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x0E\x95\x91\x90a\x14\x90V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x0E\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\xD5V[``\x91P[PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0FxW`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x10GW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10bWa\x10ba\x0F{V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x10\xA8Wa\x10\xA8a\x0F{V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x11CW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[a\x11T\x84` \x83\x01` \x89\x01a\x0F\xAAV[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x11yWa\x11ya\x0E\xDDV[\x855s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\x9DW`\0\x80\xFD[\x94P` \x86\x015\x93P`@\x86\x015a\x11\xB4\x81a\x0FbV[\x92P``\x86\x015\x80\x15\x15\x81\x14a\x11\xC9W`\0\x80\xFD[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12eW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a\x12q\x88\x82\x89\x01a\x0F\xB6V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x96Wa\x12\x96a\x0E\xDDV[\x83Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xB6W`\0\x80\xFD[` \x85\x01Q\x90\x93Pa\x12\xC7\x81a\x0FbV[`@\x85\x01Q\x90\x92Pa\x12\xD8\x81a\x0FbV[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x12\xF8Wa\x12\xF8a\x0E\xDDV[\x81Qa\x0B\x9A\x81a\x0FbV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x13DWa\x13Da\x13\x03V[P\x03\x90V[`\0[\x83\x81\x10\x15a\x13dW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13LV[\x83\x81\x11\x15a\x13sW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x13\x91\x81` \x86\x01` \x86\x01a\x13IV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R\x84` \x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R\x82\x15\x15``\x82\x01R`\xA0`\x80\x82\x01R`\0a\x14\x10`\xA0\x83\x01\x84a\x13yV[\x97\x96PPPPPPPV[`\0\x82\x19\x82\x11\x15a\x14.Wa\x14.a\x13\x03V[P\x01\x90V[`\0\x82a\x14iW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a\x14\x81`@\x83\x01\x85a\x13yV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82Qa\x14\xA2\x81\x84` \x87\x01a\x13IV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static OPTIMISMPORTAL_DEPOSITOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80c\x99\xFC\xA4/\x14a\0\xC6W\x80c\xCCs\x1B\x02\x14a\x01\x8FW\x80c\xCF\xF0\xAB\x96\x14a\x02\x96W\x80c\xFC\xF9c\xCD\x14a\x03\xB9W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x01TW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`3Ta\x01z\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02&a\x03\xCEV[`@Qa\x01\x86\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R`\xFF` \x85\x01Q\x16` \x84\x01R`\xFF`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01RPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x84\x01Q\x16`\xA0\x83\x01R\x92\x91PPV[4\x80\x15a\x03$W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x01Ta\x03\x80\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04\x81\x16\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x16\x83V[`@\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01\x86V[a\x03\xCCa\x03\xC76`\x04a\x11^V[a\x04\rV[\0[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x04\x08a\n\x91V[\x90P\x90V[`2Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cLc\xE5b\x83\x15\x80a\x04LWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x15[\x80\x15a\x04\\WPb\x01\xD4\xC0\x83Q\x11\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04|\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x05*W=`\0\x80>=`\0\xFD[PPPP`\0a\x05N\x85`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16a\x0BLV[`2T`@Q\x7F\xC8\x8A^m\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC8\x8A^m\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x06WW=`\0\x80>=`\0\xFD[PPPP`\0G\x90P`\0a\x06n\x83`\0\x84a\x0BLV[\x90P`\0`3`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCF\xF0\xAB\x96`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x94\x91\x90a\x12~V[P\x91PP`\0a\x07\xA2a\x03\xCEV[\x80Q`3T\x88Q`@Q\x7F\xA3]\x99\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\x04\x82\x01R\x93\x94Pc\xFF\xFF\xFF\xFF\x90\x92\x16\x92`\0\x92a\t\x10\x92\x90\x8D\x16\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xA3]\x99\xDF\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xB3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xEB\x91\x90a\x12\xE3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85a\t\x0B\x91\x90a\x132V[a\x0BLV[\x90P`3`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE9\xE0\\B\x86\x8E\x88\x85\x8E\x8E`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tv\x95\x94\x93\x92\x91\x90a\x13\xC3V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\n\x11W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x93PPPP\x80\x15a\n#WP`\x01[a\ngW`3\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U[PPPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x15\x15\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\x0BF`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91RP`@\x80Q`\xC0\x81\x01\x82Rc\x011-\0\x81R`\n` \x82\x01R`\x08\x91\x81\x01\x91\x90\x91Rc;\x9A\xCA\0``\x82\x01Rb\x0FB@`\x80\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x82\x01R\x90V[\x92\x91PPV[`\0a\x0BY\x84\x84\x84a\x0B\xA1V[\x90Pa\x0B\x9A`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\r\xDDV[\x93\x92PPPV[`\0\x81\x83\x11\x15a\x0C7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0CGWP\x81\x84\x11\x15[\x15a\x0CSWP\x82a\x0B\x9AV[`\0a\x0C_\x84\x84a\x132V[a\x0Cj\x90`\x01a\x14\x1BV[\x90P`\x03\x85\x11\x15\x80\x15a\x0C|WP\x84\x81\x11[\x15a\x0C\x93Wa\x0C\x8B\x85\x85a\x14\x1BV[\x91PPa\x0B\x9AV[a\x0C\xBE`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x132V[\x85\x10\x15\x80\x15a\x0C\xF5WPa\x0C\xF2\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x132V[\x81\x11[\x15a\r.Wa\r$\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x132V[a\x0C\x8B\x90\x84a\x132V[\x82\x85\x11\x15a\r\x84W`\0a\rB\x84\x87a\x132V[\x90P`\0a\rP\x83\x83a\x143V[\x90P\x80`\0\x03a\reW\x84\x93PPPPa\x0B\x9AV[`\x01a\rq\x82\x88a\x14\x1BV[a\r{\x91\x90a\x132V[\x93PPPa\r\xD5V[\x83\x85\x10\x15a\r\xD5W`\0a\r\x98\x86\x86a\x132V[\x90P`\0a\r\xA6\x83\x83a\x143V[\x90P\x80`\0\x03a\r\xBBW\x85\x93PPPPa\x0B\x9AV[a\r\xC5\x81\x86a\x132V[a\r\xD0\x90`\x01a\x14\x1BV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x0E\x14\x92\x91\x90a\x14nV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x0E\x95\x91\x90a\x14\x90V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x0E\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\xD5V[``\x91P[PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0FxW`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x10GW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10bWa\x10ba\x0F{V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x10\xA8Wa\x10\xA8a\x0F{V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x11CW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[a\x11T\x84` \x83\x01` \x89\x01a\x0F\xAAV[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x11yWa\x11ya\x0E\xDDV[\x855s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\x9DW`\0\x80\xFD[\x94P` \x86\x015\x93P`@\x86\x015a\x11\xB4\x81a\x0FbV[\x92P``\x86\x015\x80\x15\x15\x81\x14a\x11\xC9W`\0\x80\xFD[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12eW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a\x12q\x88\x82\x89\x01a\x0F\xB6V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x96Wa\x12\x96a\x0E\xDDV[\x83Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xB6W`\0\x80\xFD[` \x85\x01Q\x90\x93Pa\x12\xC7\x81a\x0FbV[`@\x85\x01Q\x90\x92Pa\x12\xD8\x81a\x0FbV[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x12\xF8Wa\x12\xF8a\x0E\xDDV[\x81Qa\x0B\x9A\x81a\x0FbV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x13DWa\x13Da\x13\x03V[P\x03\x90V[`\0[\x83\x81\x10\x15a\x13dW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13LV[\x83\x81\x11\x15a\x13sW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x13\x91\x81` \x86\x01` \x86\x01a\x13IV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R\x84` \x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R\x82\x15\x15``\x82\x01R`\xA0`\x80\x82\x01R`\0a\x14\x10`\xA0\x83\x01\x84a\x13yV[\x97\x96PPPPPPPV[`\0\x82\x19\x82\x11\x15a\x14.Wa\x14.a\x13\x03V[P\x01\x90V[`\0\x82a\x14iW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a\x14\x81`@\x83\x01\x85a\x13yV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82Qa\x14\xA2\x81\x84` \x87\x01a\x13IV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static OPTIMISMPORTAL_DEPOSITOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OptimismPortal_Depositor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OptimismPortal_Depositor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OptimismPortal_Depositor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OptimismPortal_Depositor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OptimismPortal_Depositor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OptimismPortal_Depositor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OptimismPortal_Depositor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPTIMISMPORTAL_DEPOSITOR_ABI.clone(),
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
                OPTIMISMPORTAL_DEPOSITOR_ABI.clone(),
                OPTIMISMPORTAL_DEPOSITOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `depositTransactionCompletes` (0xfcf963cd) function
        pub fn deposit_transaction_completes(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            gas_limit: u64,
            is_creation: bool,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [252, 249, 99, 205],
                    (to, value, gas_limit, is_creation, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedToComplete` (0x99fca42f) function
        pub fn failed_to_complete(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([153, 252, 164, 47], ())
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
    for OptimismPortal_Depositor<M> {
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
    ///Container type for all input parameters for the `depositTransactionCompletes` function with signature `depositTransactionCompletes(address,uint256,uint64,bool,bytes)` and selector `0xfcf963cd`
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
        name = "depositTransactionCompletes",
        abi = "depositTransactionCompletes(address,uint256,uint64,bool,bytes)"
    )]
    pub struct DepositTransactionCompletesCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub gas_limit: u64,
        pub is_creation: bool,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `failedToComplete` function with signature `failedToComplete()` and selector `0x99fca42f`
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
    #[ethcall(name = "failedToComplete", abi = "failedToComplete()")]
    pub struct FailedToCompleteCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OptimismPortal_DepositorCalls {
        DepositTransactionCompletes(DepositTransactionCompletesCall),
        FailedToComplete(FailedToCompleteCall),
        Params(ParamsCall),
        ResourceConfig(ResourceConfigCall),
    }
    impl ::ethers::core::abi::AbiDecode for OptimismPortal_DepositorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositTransactionCompletesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositTransactionCompletes(decoded));
            }
            if let Ok(decoded) = <FailedToCompleteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedToComplete(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OptimismPortal_DepositorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositTransactionCompletes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedToComplete(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Params(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResourceConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OptimismPortal_DepositorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositTransactionCompletes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedToComplete(element) => ::core::fmt::Display::fmt(element, f),
                Self::Params(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResourceConfig(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositTransactionCompletesCall>
    for OptimismPortal_DepositorCalls {
        fn from(value: DepositTransactionCompletesCall) -> Self {
            Self::DepositTransactionCompletes(value)
        }
    }
    impl ::core::convert::From<FailedToCompleteCall> for OptimismPortal_DepositorCalls {
        fn from(value: FailedToCompleteCall) -> Self {
            Self::FailedToComplete(value)
        }
    }
    impl ::core::convert::From<ParamsCall> for OptimismPortal_DepositorCalls {
        fn from(value: ParamsCall) -> Self {
            Self::Params(value)
        }
    }
    impl ::core::convert::From<ResourceConfigCall> for OptimismPortal_DepositorCalls {
        fn from(value: ResourceConfigCall) -> Self {
            Self::ResourceConfig(value)
        }
    }
    ///Container type for all return fields from the `failedToComplete` function with signature `failedToComplete()` and selector `0x99fca42f`
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
    pub struct FailedToCompleteReturn(pub bool);
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

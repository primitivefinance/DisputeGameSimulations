pub use honest_player_half_trace::*;
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
pub mod honest_player_half_trace {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_absolutePrestate"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claimAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_traceIndex"),
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
                                    name: ::std::borrow::ToOwned::to_owned("claim_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Position"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claim_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failedToStep"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedToStep"),
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
                    ::std::borrow::ToOwned::to_owned("gameProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameProxy"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract FaultDisputeGame",
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
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameProxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract FaultDisputeGame",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_counterParty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract GamePlayer"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vm"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Vm"),
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
                    ::std::borrow::ToOwned::to_owned("play"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("play"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_parentIndex"),
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
                    ::std::borrow::ToOwned::to_owned("trace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trace"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("traceAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("traceAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Position"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state_"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("traceAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_traceIndex"),
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
                                    name: ::std::borrow::ToOwned::to_owned("state_"),
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
    pub static HONESTPLAYER_HALFTRACE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1B!8\x03\x80b\0\x1B!\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x19V[`\0\x81`\x1F\x81Q\x81\x10b\0\0LWb\0\0Lb\0\x01\xF5V[\x01` \x90\x81\x01Q`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R`\xF8\x92\x90\x92\x1C\x93P`\0\x92\x82\x01\x81\x806\x837\x01\x90PP\x90P`\0[\x81Q\x81`\xFF\x16\x10\x15b\0\0\xEAWb\0\0\x96\x81\x84b\0\x02!V[b\0\0\xA3\x90`\x01b\0\x02!V[`\xF8\x1B\x82\x82`\xFF\x16\x81Q\x81\x10b\0\0\xBEWb\0\0\xBEb\0\x01\xF5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80b\0\0\xE1\x81b\0\x02IV[\x91PPb\0\0}V[P`\x01b\0\0\xF9\x82\x82b\0\x02\xFAV[PPPPb\0\x03\xC6V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15b\0\x01-W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x01ZW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x01oWb\0\x01ob\0\x01\x03V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x01\x9AWb\0\x01\x9Ab\0\x01\x03V[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15b\0\x01\xB3W`\0\x80\xFD[`\0\x93P[\x82\x84\x10\x15b\0\x01\xD7W\x84\x84\x01\x86\x01Q\x81\x85\x01\x87\x01R\x92\x85\x01\x92b\0\x01\xB8V[\x82\x84\x11\x15b\0\x01\xE9W`\0\x86\x84\x83\x01\x01R[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15b\0\x02AWb\0\x02Ab\0\x02\x0BV[\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03b\0\x02bWb\0\x02bb\0\x02\x0BV[`\x01\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02\x80W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02\xA1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\xF5W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\xD0WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\xF1W\x82\x81U`\x01\x01b\0\x02\xDCV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\x16Wb\0\x03\x16b\0\x01\x03V[b\0\x03.\x81b\0\x03'\x84Tb\0\x02kV[\x84b\0\x02\xA7V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03fW`\0\x84\x15b\0\x03MWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\xF1V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03\x97W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03vV[P\x85\x82\x10\x15b\0\x03\xB6W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x17K\x80b\0\x03\xD6`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA3W`\x005`\xE0\x1C\x80cM\x95e\xED\x11a\0vW\x80ch\x98\xF8+\x11a\0[W\x80ch\x98\xF8+\x14a\x01pW\x80c\xAA\xD9j\xD4\x14a\x01\x83W\x80c\xE5*\xFC\xED\x14a\x01\x98W`\0\x80\xFD[\x80cM\x95e\xED\x14a\x01@W\x80cN\x7Fn\xBE\x14a\x01]W`\0\x80\xFD[\x80c\x02\xD0\x15\xFB\x14a\0\xA8W\x80c\x14b\xA5\xAD\x14a\0\xCEW\x80c\x18K\x95Y\x14a\x01\x18W\x80c\x1F\x1D\xAE\t\x14a\x01-W[`\0\x80\xFD[a\0\xBBa\0\xB66`\x04a\x13\xCDV[a\x01\xABV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xF3\x90a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xC5V[a\x01+a\x01&6`\x04a\x14\x13V[a\x01\xDDV[\0[a\0\xBBa\x01;6`\x04a\x14^V[a\x02\xE7V[`\0Ta\x01M\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xC5V[a\0\xBBa\x01k6`\x04a\x13\xCDV[a\x03\xBAV[a\x01+a\x01~6`\x04a\x14^V[a\x03\xE6V[a\x01\x8Ba\x10\x0FV[`@Qa\0\xC5\x91\x90a\x14\xE2V[a\0\xBBa\x01\xA66`\x04a\x14^V[a\x10\x9DV[`\0a\x01\xD7a\x01\xA6`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92\x91PPV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16a\x01\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x02\x92\x90\x92\x17\x90\x92U`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x86\x84\x16\x17\x90\x91U`\x03\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U`@\x80Q\x7FGx\xEF\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90QcGx\xEF\xE8\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x02\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90a\x14\xF5V[`\x04UPPPV[`\0\x80`\x01\x80Ta\x02\xF7\x90a\x15\x0EV[\x90P\x83\x10\x15a\x03\x06W\x82a\x03\x1FV[`\x01\x80\x80Ta\x03\x14\x90a\x15\x0EV[a\x03\x1F\x92\x91Pa\x15\x90V[a\x03(\x84a\x10\x9DV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x93\x92PPPV[`\0a\x01\xD7a\x01;`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91a\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x82\x91\x90a\x15\xA7V[P\x93P\x93PP\x92P`\0\x80`\0c\xFF\xFF\xFF\xFF\x80\x16\x86c\xFF\xFF\xFF\xFF\x16\x03a\x04\xC2WP`\x01\x83\x81\x1Bp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x16\x92Pa\x06\x04V[`\0a\x04\xCD\x85a\x03\xBAV[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x92\x93P\x90\x91\x82\x91a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05p\x91\x90a\x15\xA7V[P\x93P\x93PPP`\0a\x05\x82\x82a\x03\xBAV[\x90P\x88\x84\x14a\x05\xD0Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x89\x90\x1B\x16\x96P\x82\x81\x14a\x05\xC7Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x83\x90\x1B\x16\x95P[`\x01\x94Pa\x05\xFFV[\x88\x84\x14\x80\x15a\x05\xDEWP\x82\x81\x14[\x15a\x05\xFFW`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x81\x17\x90\x1B\x96P[PPPP[`\x04Ta\x06\xA3\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x08\xADW```\0a\x06\xD2\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12ZV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x07\xD0W`\0\x82a\x06\xF9Wa\x06\xF4\x86`\x01a\x16 V[a\x07\x04V[a\x07\x04`\x01\x87a\x16TV[\x90P`\0a\x07#\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13\0V[\x90P\x83\x15a\x07|W`\x04Ta\x07K\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\x11\x9DV[a\x07T\x82a\x01\xABV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92Pa\x07\xC9V[`\x04Ta\x07\x9C\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a\x11\x9DV[a\x07\xA5\x88a\x01\xABV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[PPa\x07\xEEV[`@\x80Q`\x0F` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`\0T`@Q\x7F\xD8\xCC\x1A<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xD8\xCC\x1A<\x90a\x08L\x90\x8B\x90\x86\x90\x86\x90`\x04\x01a\x16\x85V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08fW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x08wWP`\x01[a\x08\xA7W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[Pa\x10\x06V[`\0a\x08\xD6`\x04T\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x08\xE3\x82a\x02\xE7V[\x90P\x82\x15a\r\x08W`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tbW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\tvW=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n$\x91\x90a\x14\xF5V[a\n.\x91\x90a\x15\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\nL\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nfW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\nzW=`\0\x80>=`\0\xFD[PPPP\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14a\r\x03W`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B5\x91\x90a\x15\xA7V[P\x93PPPP`\0a\x0Bfa\x01k`\x01\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12R\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8D\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Pa\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xF8W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0C\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA6\x91\x90a\x14\xF5V[a\x0C\xB0\x91\x90a\x15\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xCE\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xFCW=`\0\x80>=`\0\xFD[PPPPPP[a\x10\x03V[`\x02a\r\xA6\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a\r\xB0\x91\x90a\x16\xC1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x0E_WP`\0`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ES\x91\x90a\x14\xF5V[a\x0E]`\x0Fa\x02\xE7V[\x14[\x15a\x0EoWPPPPPPPPPV[`\0T`@Q\x7F5\xFE\xF5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c5\xFE\xF5g\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xFAW=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0F\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA8\x91\x90a\x14\xF5V[a\x0F\xB2\x91\x90a\x15\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xD0\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xEAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFEW=`\0\x80>=`\0\xFD[PPPP[PP[PPPPPPPV[`\x01\x80Ta\x10\x1C\x90a\x15\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10H\x90a\x15\x0EV[\x80\x15a\x10\x95W\x80`\x1F\x10a\x10jWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x95V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0`\x01\x80Ta\x10\xAC\x90a\x15\x0EV[\x90P\x82\x10\x15a\x11\x1BW`\x01\x82\x81Ta\x10\xC3\x90a\x15\x0EV[\x81\x10a\x10\xD1Wa\x10\xD1a\x17\x0FV[\x81T`\x01\x16\x15a\x10\xF0W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02a\x11\x94V[`\x01\x80\x80\x80Ta\x11*\x90a\x15\x0EV[a\x115\x92\x91Pa\x15\x90V[\x81Ta\x11@\x90a\x15\x0EV[\x81\x10a\x11NWa\x11Na\x17\x0FV[\x81T`\x01\x16\x15a\x11mW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02[`\xF8\x1C\x92\x91PPV[`\0\x80a\x12*\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[\x15\x17`\x01\x1B\x90V[`\0\x80a\x12\xE7\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x81\x19`\x01\x83\x01\x16\x81a\x13\x94\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\xCAW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x13\xDFW`\0\x80\xFD[\x815a\x13\xEA\x81a\x13\xACV[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\xCAW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14(W`\0\x80\xFD[\x835a\x143\x81a\x13\xF1V[\x92P` \x84\x015a\x14C\x81a\x13\xF1V[\x91P`@\x84\x015a\x14S\x81a\x13\xF1V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x14pW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x14\x9DW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x14\x81V[\x81\x81\x11\x15a\x14\xAFW`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\xEA` \x83\x01\x84a\x14wV[`\0` \x82\x84\x03\x12\x15a\x15\x07W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x15\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15[W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x15\xA2Wa\x15\xA2a\x15aV[P\x03\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x15\xBFW`\0\x80\xFD[\x85Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15\xD3W`\0\x80\xFD[` \x87\x01Q\x90\x95P\x80\x15\x15\x81\x14a\x15\xE9W`\0\x80\xFD[`@\x87\x01Q``\x88\x01Q\x91\x95P\x93Pa\x16\x01\x81a\x13\xACV[`\x80\x87\x01Q\x90\x92Pa\x16\x12\x81a\x13\xACV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x16KWa\x16Ka\x15aV[\x01\x94\x93PPPPV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x16}Wa\x16}a\x15aV[\x03\x93\x92PPPV[\x83\x81R\x82\x15\x15` \x82\x01R`\x80`@\x82\x01R`\0a\x16\xA6`\x80\x83\x01\x84a\x14wV[\x82\x81\x03``\x84\x01R`\0\x81R` \x81\x01\x91PP\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x17\x03W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x06\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static HONESTPLAYER_HALFTRACE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA3W`\x005`\xE0\x1C\x80cM\x95e\xED\x11a\0vW\x80ch\x98\xF8+\x11a\0[W\x80ch\x98\xF8+\x14a\x01pW\x80c\xAA\xD9j\xD4\x14a\x01\x83W\x80c\xE5*\xFC\xED\x14a\x01\x98W`\0\x80\xFD[\x80cM\x95e\xED\x14a\x01@W\x80cN\x7Fn\xBE\x14a\x01]W`\0\x80\xFD[\x80c\x02\xD0\x15\xFB\x14a\0\xA8W\x80c\x14b\xA5\xAD\x14a\0\xCEW\x80c\x18K\x95Y\x14a\x01\x18W\x80c\x1F\x1D\xAE\t\x14a\x01-W[`\0\x80\xFD[a\0\xBBa\0\xB66`\x04a\x13\xCDV[a\x01\xABV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xF3\x90a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xC5V[a\x01+a\x01&6`\x04a\x14\x13V[a\x01\xDDV[\0[a\0\xBBa\x01;6`\x04a\x14^V[a\x02\xE7V[`\0Ta\x01M\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xC5V[a\0\xBBa\x01k6`\x04a\x13\xCDV[a\x03\xBAV[a\x01+a\x01~6`\x04a\x14^V[a\x03\xE6V[a\x01\x8Ba\x10\x0FV[`@Qa\0\xC5\x91\x90a\x14\xE2V[a\0\xBBa\x01\xA66`\x04a\x14^V[a\x10\x9DV[`\0a\x01\xD7a\x01\xA6`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92\x91PPV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16a\x01\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x02\x92\x90\x92\x17\x90\x92U`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x86\x84\x16\x17\x90\x91U`\x03\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U`@\x80Q\x7FGx\xEF\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90QcGx\xEF\xE8\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x02\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90a\x14\xF5V[`\x04UPPPV[`\0\x80`\x01\x80Ta\x02\xF7\x90a\x15\x0EV[\x90P\x83\x10\x15a\x03\x06W\x82a\x03\x1FV[`\x01\x80\x80Ta\x03\x14\x90a\x15\x0EV[a\x03\x1F\x92\x91Pa\x15\x90V[a\x03(\x84a\x10\x9DV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x93\x92PPPV[`\0a\x01\xD7a\x01;`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91a\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x82\x91\x90a\x15\xA7V[P\x93P\x93PP\x92P`\0\x80`\0c\xFF\xFF\xFF\xFF\x80\x16\x86c\xFF\xFF\xFF\xFF\x16\x03a\x04\xC2WP`\x01\x83\x81\x1Bp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x16\x92Pa\x06\x04V[`\0a\x04\xCD\x85a\x03\xBAV[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x92\x93P\x90\x91\x82\x91a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05p\x91\x90a\x15\xA7V[P\x93P\x93PPP`\0a\x05\x82\x82a\x03\xBAV[\x90P\x88\x84\x14a\x05\xD0Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x89\x90\x1B\x16\x96P\x82\x81\x14a\x05\xC7Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x83\x90\x1B\x16\x95P[`\x01\x94Pa\x05\xFFV[\x88\x84\x14\x80\x15a\x05\xDEWP\x82\x81\x14[\x15a\x05\xFFW`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x81\x17\x90\x1B\x96P[PPPP[`\x04Ta\x06\xA3\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x08\xADW```\0a\x06\xD2\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12ZV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x07\xD0W`\0\x82a\x06\xF9Wa\x06\xF4\x86`\x01a\x16 V[a\x07\x04V[a\x07\x04`\x01\x87a\x16TV[\x90P`\0a\x07#\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13\0V[\x90P\x83\x15a\x07|W`\x04Ta\x07K\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\x11\x9DV[a\x07T\x82a\x01\xABV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92Pa\x07\xC9V[`\x04Ta\x07\x9C\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a\x11\x9DV[a\x07\xA5\x88a\x01\xABV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[PPa\x07\xEEV[`@\x80Q`\x0F` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`\0T`@Q\x7F\xD8\xCC\x1A<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xD8\xCC\x1A<\x90a\x08L\x90\x8B\x90\x86\x90\x86\x90`\x04\x01a\x16\x85V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08fW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x08wWP`\x01[a\x08\xA7W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[Pa\x10\x06V[`\0a\x08\xD6`\x04T\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x08\xE3\x82a\x02\xE7V[\x90P\x82\x15a\r\x08W`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tbW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\tvW=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n$\x91\x90a\x14\xF5V[a\n.\x91\x90a\x15\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\nL\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nfW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\nzW=`\0\x80>=`\0\xFD[PPPP\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14a\r\x03W`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B5\x91\x90a\x15\xA7V[P\x93PPPP`\0a\x0Bfa\x01k`\x01\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12R\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8D\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Pa\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xF8W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0C\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA6\x91\x90a\x14\xF5V[a\x0C\xB0\x91\x90a\x15\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xCE\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xFCW=`\0\x80>=`\0\xFD[PPPPPP[a\x10\x03V[`\x02a\r\xA6\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a\r\xB0\x91\x90a\x16\xC1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x0E_WP`\0`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ES\x91\x90a\x14\xF5V[a\x0E]`\x0Fa\x02\xE7V[\x14[\x15a\x0EoWPPPPPPPPPV[`\0T`@Q\x7F5\xFE\xF5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c5\xFE\xF5g\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xFAW=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0F\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA8\x91\x90a\x14\xF5V[a\x0F\xB2\x91\x90a\x15\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xD0\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xEAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFEW=`\0\x80>=`\0\xFD[PPPP[PP[PPPPPPPV[`\x01\x80Ta\x10\x1C\x90a\x15\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10H\x90a\x15\x0EV[\x80\x15a\x10\x95W\x80`\x1F\x10a\x10jWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x95V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0`\x01\x80Ta\x10\xAC\x90a\x15\x0EV[\x90P\x82\x10\x15a\x11\x1BW`\x01\x82\x81Ta\x10\xC3\x90a\x15\x0EV[\x81\x10a\x10\xD1Wa\x10\xD1a\x17\x0FV[\x81T`\x01\x16\x15a\x10\xF0W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02a\x11\x94V[`\x01\x80\x80\x80Ta\x11*\x90a\x15\x0EV[a\x115\x92\x91Pa\x15\x90V[\x81Ta\x11@\x90a\x15\x0EV[\x81\x10a\x11NWa\x11Na\x17\x0FV[\x81T`\x01\x16\x15a\x11mW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02[`\xF8\x1C\x92\x91PPV[`\0\x80a\x12*\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[\x15\x17`\x01\x1B\x90V[`\0\x80a\x12\xE7\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x81\x19`\x01\x83\x01\x16\x81a\x13\x94\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\xCAW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x13\xDFW`\0\x80\xFD[\x815a\x13\xEA\x81a\x13\xACV[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\xCAW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14(W`\0\x80\xFD[\x835a\x143\x81a\x13\xF1V[\x92P` \x84\x015a\x14C\x81a\x13\xF1V[\x91P`@\x84\x015a\x14S\x81a\x13\xF1V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x14pW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x14\x9DW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x14\x81V[\x81\x81\x11\x15a\x14\xAFW`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\xEA` \x83\x01\x84a\x14wV[`\0` \x82\x84\x03\x12\x15a\x15\x07W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x15\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15[W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x15\xA2Wa\x15\xA2a\x15aV[P\x03\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x15\xBFW`\0\x80\xFD[\x85Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15\xD3W`\0\x80\xFD[` \x87\x01Q\x90\x95P\x80\x15\x15\x81\x14a\x15\xE9W`\0\x80\xFD[`@\x87\x01Q``\x88\x01Q\x91\x95P\x93Pa\x16\x01\x81a\x13\xACV[`\x80\x87\x01Q\x90\x92Pa\x16\x12\x81a\x13\xACV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x16KWa\x16Ka\x15aV[\x01\x94\x93PPPPV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x16}Wa\x16}a\x15aV[\x03\x93\x92PPPV[\x83\x81R\x82\x15\x15` \x82\x01R`\x80`@\x82\x01R`\0a\x16\xA6`\x80\x83\x01\x84a\x14wV[\x82\x81\x03``\x84\x01R`\0\x81R` \x81\x01\x91PP\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x17\x03W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x06\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static HONESTPLAYER_HALFTRACE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct HonestPlayer_HalfTrace<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HonestPlayer_HalfTrace<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HonestPlayer_HalfTrace<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HonestPlayer_HalfTrace<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HonestPlayer_HalfTrace<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HonestPlayer_HalfTrace))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HonestPlayer_HalfTrace<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HONESTPLAYER_HALFTRACE_ABI.clone(),
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
                HONESTPLAYER_HALFTRACE_ABI.clone(),
                HONESTPLAYER_HALFTRACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `claimAt` (0x1f1dae09) function
        pub fn claim_at(
            &self,
            trace_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([31, 29, 174, 9], trace_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimAt` (0x4e7f6ebe) function
        pub fn claim_at_with_position(
            &self,
            position: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([78, 127, 110, 190], position)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedToStep` (0x4d9565ed) function
        pub fn failed_to_step(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([77, 149, 101, 237], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameProxy` (0x1462a5ad) function
        pub fn game_proxy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([20, 98, 165, 173], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x184b9559) function
        pub fn init(
            &self,
            game_proxy: ::ethers::core::types::Address,
            counter_party: ::ethers::core::types::Address,
            vm: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 75, 149, 89], (game_proxy, counter_party, vm))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `play` (0x6898f82b) function
        pub fn play(
            &self,
            parent_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 152, 248, 43], parent_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trace` (0xaad96ad4) function
        pub fn trace(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([170, 217, 106, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `traceAt` (0x02d015fb) function
        pub fn trace_at(
            &self,
            position: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([2, 208, 21, 251], position)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `traceAt` (0xe52afced) function
        pub fn trace_at_with_trace_index(
            &self,
            trace_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 42, 252, 237], trace_index)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HonestPlayer_HalfTrace<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `claimAt` function with signature `claimAt(uint256)` and selector `0x1f1dae09`
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
    #[ethcall(name = "claimAt", abi = "claimAt(uint256)")]
    pub struct ClaimAtCall {
        pub trace_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `claimAt` function with signature `claimAt(uint128)` and selector `0x4e7f6ebe`
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
    #[ethcall(name = "claimAt", abi = "claimAt(uint128)")]
    pub struct ClaimAtWithPositionCall {
        pub position: u128,
    }
    ///Container type for all input parameters for the `failedToStep` function with signature `failedToStep()` and selector `0x4d9565ed`
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
    #[ethcall(name = "failedToStep", abi = "failedToStep()")]
    pub struct FailedToStepCall;
    ///Container type for all input parameters for the `gameProxy` function with signature `gameProxy()` and selector `0x1462a5ad`
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
    #[ethcall(name = "gameProxy", abi = "gameProxy()")]
    pub struct GameProxyCall;
    ///Container type for all input parameters for the `init` function with signature `init(address,address,address)` and selector `0x184b9559`
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
    #[ethcall(name = "init", abi = "init(address,address,address)")]
    pub struct InitCall {
        pub game_proxy: ::ethers::core::types::Address,
        pub counter_party: ::ethers::core::types::Address,
        pub vm: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `play` function with signature `play(uint256)` and selector `0x6898f82b`
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
    #[ethcall(name = "play", abi = "play(uint256)")]
    pub struct PlayCall {
        pub parent_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `trace` function with signature `trace()` and selector `0xaad96ad4`
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
    #[ethcall(name = "trace", abi = "trace()")]
    pub struct TraceCall;
    ///Container type for all input parameters for the `traceAt` function with signature `traceAt(uint128)` and selector `0x02d015fb`
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
    #[ethcall(name = "traceAt", abi = "traceAt(uint128)")]
    pub struct TraceAtCall {
        pub position: u128,
    }
    ///Container type for all input parameters for the `traceAt` function with signature `traceAt(uint256)` and selector `0xe52afced`
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
    #[ethcall(name = "traceAt", abi = "traceAt(uint256)")]
    pub struct TraceAtWithTraceIndexCall {
        pub trace_index: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum HonestPlayer_HalfTraceCalls {
        ClaimAt(ClaimAtCall),
        ClaimAtWithPosition(ClaimAtWithPositionCall),
        FailedToStep(FailedToStepCall),
        GameProxy(GameProxyCall),
        Init(InitCall),
        Play(PlayCall),
        Trace(TraceCall),
        TraceAt(TraceAtCall),
        TraceAtWithTraceIndex(TraceAtWithTraceIndexCall),
    }
    impl ::ethers::core::abi::AbiDecode for HonestPlayer_HalfTraceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClaimAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimAt(decoded));
            }
            if let Ok(decoded) = <ClaimAtWithPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimAtWithPosition(decoded));
            }
            if let Ok(decoded) = <FailedToStepCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedToStep(decoded));
            }
            if let Ok(decoded) = <GameProxyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameProxy(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <PlayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Play(decoded));
            }
            if let Ok(decoded) = <TraceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Trace(decoded));
            }
            if let Ok(decoded) = <TraceAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TraceAt(decoded));
            }
            if let Ok(decoded) = <TraceAtWithTraceIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TraceAtWithTraceIndex(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HonestPlayer_HalfTraceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimAt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimAtWithPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedToStep(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GameProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Play(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Trace(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TraceAt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TraceAtWithTraceIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for HonestPlayer_HalfTraceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimAtWithPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedToStep(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameProxy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::Play(element) => ::core::fmt::Display::fmt(element, f),
                Self::Trace(element) => ::core::fmt::Display::fmt(element, f),
                Self::TraceAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::TraceAtWithTraceIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ClaimAtCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: ClaimAtCall) -> Self {
            Self::ClaimAt(value)
        }
    }
    impl ::core::convert::From<ClaimAtWithPositionCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: ClaimAtWithPositionCall) -> Self {
            Self::ClaimAtWithPosition(value)
        }
    }
    impl ::core::convert::From<FailedToStepCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: FailedToStepCall) -> Self {
            Self::FailedToStep(value)
        }
    }
    impl ::core::convert::From<GameProxyCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: GameProxyCall) -> Self {
            Self::GameProxy(value)
        }
    }
    impl ::core::convert::From<InitCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<PlayCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: PlayCall) -> Self {
            Self::Play(value)
        }
    }
    impl ::core::convert::From<TraceCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: TraceCall) -> Self {
            Self::Trace(value)
        }
    }
    impl ::core::convert::From<TraceAtCall> for HonestPlayer_HalfTraceCalls {
        fn from(value: TraceAtCall) -> Self {
            Self::TraceAt(value)
        }
    }
    impl ::core::convert::From<TraceAtWithTraceIndexCall>
    for HonestPlayer_HalfTraceCalls {
        fn from(value: TraceAtWithTraceIndexCall) -> Self {
            Self::TraceAtWithTraceIndex(value)
        }
    }
    ///Container type for all return fields from the `claimAt` function with signature `claimAt(uint256)` and selector `0x1f1dae09`
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
    pub struct ClaimAtReturn {
        pub claim: [u8; 32],
    }
    ///Container type for all return fields from the `claimAt` function with signature `claimAt(uint128)` and selector `0x4e7f6ebe`
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
    pub struct ClaimAtWithPositionReturn {
        pub claim: [u8; 32],
    }
    ///Container type for all return fields from the `failedToStep` function with signature `failedToStep()` and selector `0x4d9565ed`
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
    pub struct FailedToStepReturn(pub bool);
    ///Container type for all return fields from the `gameProxy` function with signature `gameProxy()` and selector `0x1462a5ad`
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
    pub struct GameProxyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `trace` function with signature `trace()` and selector `0xaad96ad4`
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
    pub struct TraceReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `traceAt` function with signature `traceAt(uint128)` and selector `0x02d015fb`
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
    pub struct TraceAtReturn {
        pub state: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `traceAt` function with signature `traceAt(uint256)` and selector `0xe52afced`
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
    pub struct TraceAtWithTraceIndexReturn {
        pub state: ::ethers::core::types::U256,
    }
}

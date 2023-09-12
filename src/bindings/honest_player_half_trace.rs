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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0&\x808\x03\x80b\0&\x80\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x02\xD0V[`\0\x81`\x1F\x81Q\x81\x10b\0\0\x99Wb\0\0\x99b\0\x03\xA6V[\x01` \x90\x81\x01Q`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R`\xF8\x92\x90\x92\x1C\x93P`\0\x92\x82\x01\x81\x806\x837\x01\x90PP\x90P`\0[\x81Q\x81`\xFF\x16\x10\x15b\0\x017Wb\0\0\xE3\x81\x84b\0\x03\xD2V[b\0\0\xF0\x90`\x01b\0\x03\xD2V[`\xF8\x1B\x82\x82`\xFF\x16\x81Q\x81\x10b\0\x01\x0BWb\0\x01\x0Bb\0\x03\xA6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80b\0\x01.\x81b\0\x03\xFAV[\x91PPb\0\0\xCAV[P`\x01b\0\x01F\x82\x82b\0\x04\xABV[PPPPb\0\x05wV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x01\x83W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x01iV[\x83\x81\x11\x15b\0\x01\x93W`\0\x84\x84\x01R[PPPPV[`\0\x82`\x1F\x83\x01\x12b\0\x01\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02\x1CWb\0\x02\x1Cb\0\x01PV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x02GWb\0\x02Gb\0\x01PV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x02\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[b\0\x02\xC6\x84` \x83\x01` \x89\x01b\0\x01fV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15b\0\x03.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x03\x9E\x84\x82\x85\x01b\0\x01\x99V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15b\0\x03\xF2Wb\0\x03\xF2b\0\x03\xBCV[\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03b\0\x04\x13Wb\0\x04\x13b\0\x03\xBCV[`\x01\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x041W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x04RWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x04\xA6W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x04\x81WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x04\xA2W\x82\x81U`\x01\x01b\0\x04\x8DV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\xC7Wb\0\x04\xC7b\0\x01PV[b\0\x04\xDF\x81b\0\x04\xD8\x84Tb\0\x04\x1CV[\x84b\0\x04XV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x05\x17W`\0\x84\x15b\0\x04\xFEWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x04\xA2V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x05HW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x05'V[P\x85\x82\x10\x15b\0\x05gW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a \xF9\x80b\0\x05\x87`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01%W`\x005`\xE0\x1C\x80cM\x95e\xED\x11a\0\xF8W\x80ch\x98\xF8+\x11a\0\xDDW\x80ch\x98\xF8+\x14a\x02tW\x80c\xAA\xD9j\xD4\x14a\x02\x87W\x80c\xE5*\xFC\xED\x14a\x02\x9CWa\x01%V[\x80cM\x95e\xED\x14a\x02DW\x80cN\x7Fn\xBE\x14a\x02aWa\x01%V[\x80c\x02\xD0\x15\xFB\x14a\x01\xACW\x80c\x14b\xA5\xAD\x14a\x01\xD2W\x80c\x18K\x95Y\x14a\x02\x1CW\x80c\x1F\x1D\xAE\t\x14a\x021W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xBFa\x01\xBA6`\x04a\x1DlV[a\x02\xAFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\x01\xF7\x90a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xC9V[a\x02/a\x02*6`\x04a\x1D\xB5V[a\x02\xE1V[\0[a\x01\xBFa\x02?6`\x04a\x1E\x03V[a\x04|V[`\0Ta\x02Q\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xC9V[a\x01\xBFa\x02o6`\x04a\x1DlV[a\x05OV[a\x02/a\x02\x826`\x04a\x1E\x03V[a\x05{V[a\x02\x8Fa\x19)V[`@Qa\x01\xC9\x91\x90a\x1E\x8AV[a\x01\xBFa\x02\xAA6`\x04a\x1E\x03V[a\x19\xB7V[`\0a\x02\xDBa\x02\xAA`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xB7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92\x91PPV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16a\x01\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x02\x92\x90\x92\x17\x90\x92U`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x86\x84\x16\x17\x90\x91U`\x03\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U`@\x80Q\x7FGx\xEF\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90QcGx\xEF\xE8\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x04<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04t\x91\x90a\x1E\x9DV[`\x04UPPPV[`\0\x80`\x01\x80Ta\x04\x8C\x90a\x1E\xB9V[\x90P\x83\x10\x15a\x04\x9BW\x82a\x04\xB4V[`\x01\x80\x80Ta\x04\xA9\x90a\x1E\xB9V[a\x04\xB4\x92\x91Pa\x1F;V[a\x04\xBD\x84a\x19\xB7V[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x93\x92PPPV[`\0a\x02\xDBa\x02?`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xB7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91a\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA8\x91\x90a\x1FRV[P\x93P\x93PP\x92P`\0\x80`\0c\xFF\xFF\xFF\xFF\x80\x16\x86c\xFF\xFF\xFF\xFF\x16\x03a\x06\xE8WP`\x01\x83\x81\x1Bp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x16\x92Pa\x08\xBBV[`\0a\x06\xF3\x85a\x05OV[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x92\x93P\x90\x91\x82\x91a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xEFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08'\x91\x90a\x1FRV[P\x93P\x93PPP`\0a\x089\x82a\x05OV[\x90P\x88\x84\x14a\x08\x87Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x89\x90\x1B\x16\x96P\x82\x81\x14a\x08~Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x83\x90\x1B\x16\x95P[`\x01\x94Pa\x08\xB6V[\x88\x84\x14\x80\x15a\x08\x95WP\x82\x81\x14[\x15a\x08\xB6W`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x81\x17\x90\x1B\x96P[PPPP[`\x04Ta\tZ\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xE6W```\0a\t\x89\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1BtV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\n\x87W`\0\x82a\t\xB0Wa\t\xAB\x86`\x01a\x1F\xCEV[a\t\xBBV[a\t\xBB`\x01\x87a \x02V[\x90P`\0a\t\xDA\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C\x1AV[\x90P\x83\x15a\n3W`\x04Ta\n\x02\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\x1A\xB7V[a\n\x0B\x82a\x02\xAFV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92Pa\n\x80V[`\x04Ta\nS\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a\x1A\xB7V[a\n\\\x88a\x02\xAFV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[PPa\n\xA5V[`@\x80Q`\x0F` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`\0T`@Q\x7F\xD8\xCC\x1A<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xD8\xCC\x1A<\x90a\x0B\x03\x90\x8B\x90\x86\x90\x86\x90`\x04\x01a 3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x9FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x0B\xB0WP`\x01[a\x0B\xE0W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[Pa\x19 V[`\0a\x0C\x0F`\x04T\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xB7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0C\x1C\x82a\x04|V[\x90P\x82\x15a\x13\xFCW`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r1W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0ELW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ep\x91\x90a\x1E\x9DV[a\x0Ez\x91\x90a\x1F;V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x98\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0FHW=`\0\x80>=`\0\xFD[PPPP\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14a\x13\xF7W`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x94\x91\x90a\x1FRV[P\x93PPPP`\0a\x10\xC5a\x02o`\x01\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1Bl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8D\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Pa\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xD9W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12\xE0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x18\x91\x90a\x1E\x9DV[a\x13\"\x91\x90a\x1F;V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13@\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\xF0W=`\0\x80>=`\0\xFD[PPPPPP[a\x19\x1DV[`\x02a\x14\x9A\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a\x14\xA4\x91\x90a oV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x15\xE4WP`\0`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD8\x91\x90a\x1E\x9DV[a\x15\xE2`\x0Fa\x04|V[\x14[\x15a\x15\xF4WPPPPPPPPPV[`\0T`@Q\x7F5\xFE\xF5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c5\xFE\xF5g\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17\x01W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x18\x08W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x18\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18@\x91\x90a\x1E\x9DV[a\x18J\x91\x90a\x1F;V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18h\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x04W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\x18W=`\0\x80>=`\0\xFD[PPPP[PP[PPPPPPPV[`\x01\x80Ta\x196\x90a\x1E\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19b\x90a\x1E\xB9V[\x80\x15a\x19\xAFW\x80`\x1F\x10a\x19\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\xAFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0`\x01\x80Ta\x19\xC6\x90a\x1E\xB9V[\x90P\x82\x10\x15a\x1A5W`\x01\x82\x81Ta\x19\xDD\x90a\x1E\xB9V[\x81\x10a\x19\xEBWa\x19\xEBa \xBDV[\x81T`\x01\x16\x15a\x1A\nW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02a\x1A\xAEV[`\x01\x80\x80\x80Ta\x1AD\x90a\x1E\xB9V[a\x1AO\x92\x91Pa\x1F;V[\x81Ta\x1AZ\x90a\x1E\xB9V[\x81\x10a\x1AhWa\x1Aha \xBDV[\x81T`\x01\x16\x15a\x1A\x87W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02[`\xF8\x1C\x92\x91PPV[`\0\x80a\x1BD\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[\x15\x17`\x01\x1B\x90V[`\0\x80a\x1C\x01\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x81\x19`\x01\x83\x01\x16\x81a\x1C\xAE\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1DiW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x1D\x81Wa\x1D\x81a\x1C\xC6V[\x815a\x1D\x8C\x81a\x1DKV[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1DiW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xCDWa\x1D\xCDa\x1C\xC6V[\x835a\x1D\xD8\x81a\x1D\x93V[\x92P` \x84\x015a\x1D\xE8\x81a\x1D\x93V[\x91P`@\x84\x015a\x1D\xF8\x81a\x1D\x93V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1E\x18Wa\x1E\x18a\x1C\xC6V[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1EEW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1E)V[\x81\x81\x11\x15a\x1EWW`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1D\x8C` \x83\x01\x84a\x1E\x1FV[`\0` \x82\x84\x03\x12\x15a\x1E\xB2Wa\x1E\xB2a\x1C\xC6V[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\xCDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1F\x06W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1FMWa\x1FMa\x1F\x0CV[P\x03\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1FmWa\x1Fma\x1C\xC6V[\x85Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F\x81W`\0\x80\xFD[` \x87\x01Q\x90\x95P\x80\x15\x15\x81\x14a\x1F\x97W`\0\x80\xFD[`@\x87\x01Q``\x88\x01Q\x91\x95P\x93Pa\x1F\xAF\x81a\x1DKV[`\x80\x87\x01Q\x90\x92Pa\x1F\xC0\x81a\x1DKV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\xF9Wa\x1F\xF9a\x1F\x0CV[\x01\x94\x93PPPPV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a +Wa +a\x1F\x0CV[\x03\x93\x92PPPV[\x83\x81R\x82\x15\x15` \x82\x01R`\x80`@\x82\x01R`\0a T`\x80\x83\x01\x84a\x1E\x1FV[\x82\x81\x03``\x84\x01R`\0\x81R` \x81\x01\x91PP\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a \xB1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x06\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static HONESTPLAYER_HALFTRACE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01%W`\x005`\xE0\x1C\x80cM\x95e\xED\x11a\0\xF8W\x80ch\x98\xF8+\x11a\0\xDDW\x80ch\x98\xF8+\x14a\x02tW\x80c\xAA\xD9j\xD4\x14a\x02\x87W\x80c\xE5*\xFC\xED\x14a\x02\x9CWa\x01%V[\x80cM\x95e\xED\x14a\x02DW\x80cN\x7Fn\xBE\x14a\x02aWa\x01%V[\x80c\x02\xD0\x15\xFB\x14a\x01\xACW\x80c\x14b\xA5\xAD\x14a\x01\xD2W\x80c\x18K\x95Y\x14a\x02\x1CW\x80c\x1F\x1D\xAE\t\x14a\x021W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xBFa\x01\xBA6`\x04a\x1DlV[a\x02\xAFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\x01\xF7\x90a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xC9V[a\x02/a\x02*6`\x04a\x1D\xB5V[a\x02\xE1V[\0[a\x01\xBFa\x02?6`\x04a\x1E\x03V[a\x04|V[`\0Ta\x02Q\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xC9V[a\x01\xBFa\x02o6`\x04a\x1DlV[a\x05OV[a\x02/a\x02\x826`\x04a\x1E\x03V[a\x05{V[a\x02\x8Fa\x19)V[`@Qa\x01\xC9\x91\x90a\x1E\x8AV[a\x01\xBFa\x02\xAA6`\x04a\x1E\x03V[a\x19\xB7V[`\0a\x02\xDBa\x02\xAA`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xB7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92\x91PPV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16a\x01\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x02\x92\x90\x92\x17\x90\x92U`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x86\x84\x16\x17\x90\x91U`\x03\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U`@\x80Q\x7FGx\xEF\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90QcGx\xEF\xE8\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x04<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04t\x91\x90a\x1E\x9DV[`\x04UPPPV[`\0\x80`\x01\x80Ta\x04\x8C\x90a\x1E\xB9V[\x90P\x83\x10\x15a\x04\x9BW\x82a\x04\xB4V[`\x01\x80\x80Ta\x04\xA9\x90a\x1E\xB9V[a\x04\xB4\x92\x91Pa\x1F;V[a\x04\xBD\x84a\x19\xB7V[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x93\x92PPPV[`\0a\x02\xDBa\x02?`\x04T\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xB7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x82\x91\x82\x91a\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA8\x91\x90a\x1FRV[P\x93P\x93PP\x92P`\0\x80`\0c\xFF\xFF\xFF\xFF\x80\x16\x86c\xFF\xFF\xFF\xFF\x16\x03a\x06\xE8WP`\x01\x83\x81\x1Bp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x16\x92Pa\x08\xBBV[`\0a\x06\xF3\x85a\x05OV[`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x92\x93P\x90\x91\x82\x91a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xEFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08'\x91\x90a\x1FRV[P\x93P\x93PPP`\0a\x089\x82a\x05OV[\x90P\x88\x84\x14a\x08\x87Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x89\x90\x1B\x16\x96P\x82\x81\x14a\x08~Wp\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE`\x01\x83\x90\x1B\x16\x95P[`\x01\x94Pa\x08\xB6V[\x88\x84\x14\x80\x15a\x08\x95WP\x82\x81\x14[\x15a\x08\xB6W`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x81\x17\x90\x1B\x96P[PPPP[`\x04Ta\tZ\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xE6W```\0a\t\x89\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1BtV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\n\x87W`\0\x82a\t\xB0Wa\t\xAB\x86`\x01a\x1F\xCEV[a\t\xBBV[a\t\xBB`\x01\x87a \x02V[\x90P`\0a\t\xDA\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C\x1AV[\x90P\x83\x15a\n3W`\x04Ta\n\x02\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90a\x1A\xB7V[a\n\x0B\x82a\x02\xAFV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92Pa\n\x80V[`\x04Ta\nS\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a\x1A\xB7V[a\n\\\x88a\x02\xAFV[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[PPa\n\xA5V[`@\x80Q`\x0F` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`\0T`@Q\x7F\xD8\xCC\x1A<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xD8\xCC\x1A<\x90a\x0B\x03\x90\x8B\x90\x86\x90\x86\x90`\x04\x01a 3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x9FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x92PPP\x80\x15a\x0B\xB0WP`\x01[a\x0B\xE0W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[Pa\x19 V[`\0a\x0C\x0F`\x04T\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1A\xB7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0C\x1C\x82a\x04|V[\x90P\x82\x15a\x13\xFCW`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r1W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0E8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0ELW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ep\x91\x90a\x1E\x9DV[a\x0Ez\x91\x90a\x1F;V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x98\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0FHW=`\0\x80>=`\0\xFD[PPPP\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14a\x13\xF7W`\0\x80T`@Q\x7F\xC6\xF00\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC6\xF00\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x94\x91\x90a\x1FRV[P\x93PPPP`\0a\x10\xC5a\x02o`\x01\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1Bl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0T`@Q\x7F\xC5\\\xD0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x8D\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Pa\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC5\\\xD0\xC7\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11\xD9W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12\xE0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x18\x91\x90a\x1E\x9DV[a\x13\"\x91\x90a\x1F;V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13@\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x13\xF0W=`\0\x80>=`\0\xFD[PPPPPP[a\x19\x1DV[`\x02a\x14\x9A\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[a\x14\xA4\x91\x90a oV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x15\xE4WP`\0`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBC\xEF;U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x15\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x15\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD8\x91\x90a\x1E\x9DV[a\x15\xE2`\x0Fa\x04|V[\x14[\x15a\x15\xF4WPPPPPPPPPV[`\0T`@Q\x7F5\xFE\xF5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x8B\x90R`$\x81\x01\x83\x90Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c5\xFE\xF5g\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17\x01W=`\0\x80>=`\0\xFD[PP`\x02T`\0T`@\x80Q\x7F\x89\x80\xE0\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pch\x98\xF8+\x94P`\x01\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91c\x89\x80\xE0\xCC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x18\x08W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x18\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18@\x91\x90a\x1E\x9DV[a\x18J\x91\x90a\x1F;V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18h\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x04W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x19\x18W=`\0\x80>=`\0\xFD[PPPP[PP[PPPPPPPV[`\x01\x80Ta\x196\x90a\x1E\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19b\x90a\x1E\xB9V[\x80\x15a\x19\xAFW\x80`\x1F\x10a\x19\x84Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\xAFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0`\x01\x80Ta\x19\xC6\x90a\x1E\xB9V[\x90P\x82\x10\x15a\x1A5W`\x01\x82\x81Ta\x19\xDD\x90a\x1E\xB9V[\x81\x10a\x19\xEBWa\x19\xEBa \xBDV[\x81T`\x01\x16\x15a\x1A\nW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02a\x1A\xAEV[`\x01\x80\x80\x80Ta\x1AD\x90a\x1E\xB9V[a\x1AO\x92\x91Pa\x1F;V[\x81Ta\x1AZ\x90a\x1E\xB9V[\x81\x10a\x1AhWa\x1Aha \xBDV[\x81T`\x01\x16\x15a\x1A\x87W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T\x90\x1A\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02[`\xF8\x1C\x92\x91PPV[`\0\x80a\x1BD\x84~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x83\x03`\x01\x84\x1B`\x01\x80\x83\x1B\x03\x86\x83\x1B\x17\x03\x92PPP\x92\x91PPV[\x15\x17`\x01\x1B\x90V[`\0\x80a\x1C\x01\x83~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x1B\x90\x92\x03\x92\x91PPV[`\0\x81\x19`\x01\x83\x01\x16\x81a\x1C\xAE\x82~\t\x01\n\r\x15\x02\x1D\x0B\x0E\x10\x12\x16\x19\x03\x1E\x08\x0C\x14\x1C\x0F\x11\x18\x07\x13\x1B\x17\x06\x1A\x05\x04\x1F\x7F\x07\xC4\xAC\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11`\x06\x1B\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x92\x83\x1C`\x01\x81\x90\x1C\x17`\x02\x81\x90\x1C\x17`\x04\x81\x90\x1C\x17`\x08\x81\x90\x1C\x17`\x10\x81\x90\x1C\x17\x02`\xFB\x1C\x1A\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x90\x93\x1C\x80\x15\x17\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1DiW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x1D\x81Wa\x1D\x81a\x1C\xC6V[\x815a\x1D\x8C\x81a\x1DKV[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1DiW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xCDWa\x1D\xCDa\x1C\xC6V[\x835a\x1D\xD8\x81a\x1D\x93V[\x92P` \x84\x015a\x1D\xE8\x81a\x1D\x93V[\x91P`@\x84\x015a\x1D\xF8\x81a\x1D\x93V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1E\x18Wa\x1E\x18a\x1C\xC6V[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1EEW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1E)V[\x81\x81\x11\x15a\x1EWW`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1D\x8C` \x83\x01\x84a\x1E\x1FV[`\0` \x82\x84\x03\x12\x15a\x1E\xB2Wa\x1E\xB2a\x1C\xC6V[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\xCDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1F\x06W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1FMWa\x1FMa\x1F\x0CV[P\x03\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1FmWa\x1Fma\x1C\xC6V[\x85Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F\x81W`\0\x80\xFD[` \x87\x01Q\x90\x95P\x80\x15\x15\x81\x14a\x1F\x97W`\0\x80\xFD[`@\x87\x01Q``\x88\x01Q\x91\x95P\x93Pa\x1F\xAF\x81a\x1DKV[`\x80\x87\x01Q\x90\x92Pa\x1F\xC0\x81a\x1DKV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\xF9Wa\x1F\xF9a\x1F\x0CV[\x01\x94\x93PPPPV[`\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a +Wa +a\x1F\x0CV[\x03\x93\x92PPPV[\x83\x81R\x82\x15\x15` \x82\x01R`\x80`@\x82\x01R`\0a T`\x80\x83\x01\x84a\x1E\x1FV[\x82\x81\x03``\x84\x01R`\0\x81R` \x81\x01\x91PP\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a \xB1W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x06\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
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

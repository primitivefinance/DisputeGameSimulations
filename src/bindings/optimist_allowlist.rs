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
pub mod optimist_allowlist {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_attestationStation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract AttestationStation",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_allowlistAttestor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_coinbaseQuestAttestor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_optimistInviter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ALLOWLIST_ATTESTOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ALLOWLIST_ATTESTOR"),
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
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_STATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ATTESTATION_STATION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AttestationStation",
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
                    ::std::borrow::ToOwned::to_owned("COINBASE_QUEST_ATTESTOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "COINBASE_QUEST_ATTESTOR",
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
                        "COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY",
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
                        "OPTIMIST_CAN_MINT_ATTESTATION_KEY",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMIST_CAN_MINT_ATTESTATION_KEY",
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
                    ::std::borrow::ToOwned::to_owned("OPTIMIST_INVITER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("OPTIMIST_INVITER"),
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
                    ::std::borrow::ToOwned::to_owned("isAllowedToMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAllowedToMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_claimer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowed_"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OPTIMISTALLOWLIST_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15a\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x0E\xAB8\x03\x80a\x0E\xAB\x839\x81\x01`@\x81\x90Ra\0}\x91a\0\xC9V[`\x01`\x80R`\0`\xA0R`\x02`\xC0R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\xE0R\x91\x83\x16a\x01\0R\x82\x16a\x01 R\x16a\x01@Ra\x01sV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xC6W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x84Qa\x015\x81a\0\xB1V[` \x86\x01Q\x90\x94Pa\x01F\x81a\0\xB1V[`@\x86\x01Q\x90\x93Pa\x01W\x81a\0\xB1V[``\x86\x01Q\x90\x92Pa\x01h\x81a\0\xB1V[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x0C\xC2a\x01\xE9`\09`\0\x81\x81a\x02\x1F\x01Ra\x04^\x01R`\0\x81\x81a\x01\x96\x01Ra\x04\x11\x01R`\0\x81\x81a\x02\xA2\x01Ra\x03\xC4\x01R`\0\x81\x81a\x02{\x01Ra\x06:\x01R`\0a\x03s\x01R`\0a\x03J\x01R`\0a\x03!\x01Ra\x0C\xC2`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\nW`\x005`\xE0\x1C\x80c\x81\x9F~\x84\x11a\0\xDDW\x80c\x81\x9F~\x84\x14a\x02AW\x80c\xDB\x08=q\x14a\x02vW\x80c\xDB<1c\x14a\x02\x9DW\x80c\xE7\xBD\x80N\x14a\x02\xC4Wa\x01\nV[\x80c:\xC5-\xF7\x14a\x01\x91W\x80cH\x13\xD8\xA6\x14a\x01\xE2W\x80cT\xFDMP\x14a\x02\x05W\x80c^OH\x9A\x14a\x02\x1AW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF5a\x01\xF06`\x04a\x07\xE7V[a\x02\xEBV[`@Q\x90\x15\x15\x81R` \x01a\x01\xD9V[a\x02\ra\x03\x1AV[`@Qa\x01\xD9\x91\x90a\x08WV[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02h\x7Fcoinbase.quest-eligible\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x01\xD9V[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02h\x7Foptimist.can-mint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x02\xF6\x82a\x03\xBDV[\x80a\x03\x05WPa\x03\x05\x82a\x04\nV[\x80a\x03\x14WPa\x03\x14\x82a\x04WV[\x92\x91PPV[``a\x03E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA4V[a\x03n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA4V[a\x03\x97\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA4V[`@Q` \x01a\x03\xA9\x93\x92\x91\x90a\x08\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x03\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x7Foptimist.can-mint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xE1V[`\0a\x03\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x7Fcoinbase.quest-eligible\0\0\0\0\0\0\0\0\0a\x05\xE1V[`\0a\x03\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x7Foptimist.can-mint-from-invite\0\0\0a\x05\xE1V[``\x81`\0\x03a\x04\xE7WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x05\x11W\x80a\x04\xFB\x81a\tMV[\x91Pa\x05\n\x90P`\n\x83a\t\xB4V[\x91Pa\x04\xEBV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05,Wa\x05,a\t\xC8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05VW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x05\xD9Wa\x05k`\x01\x83a\t\xF7V[\x91Pa\x05x`\n\x86a\n\x0EV[a\x05\x83\x90`0a\n\"V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x05\x98Wa\x05\x98a\n:V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x05\xD2`\n\x86a\t\xB4V[\x94Pa\x05ZV[\x94\x93PPPPV[`@Q\x7F)\xB4,\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c)\xB4,\xB5\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x07X\x91\x90\x81\x01\x90a\niV[Q\x11\x94\x93PPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xFCWa\x07\xFCa\x07bV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08 W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x08BW\x81\x81\x01Q\x83\x82\x01R` \x01a\x08*V[\x83\x81\x11\x15a\x08QW`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x08v\x81`@\x85\x01` \x87\x01a\x08'V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x84Qa\x08\xBA\x81\x84` \x89\x01a\x08'V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x08\xF6\x81`\x01\x85\x01` \x8A\x01a\x08'V[`\x01\x92\x01\x91\x82\x01R\x83Qa\t\x11\x81`\x02\x84\x01` \x88\x01a\x08'V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\t~Wa\t~a\t\x1EV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\t\xC3Wa\t\xC3a\t\x85V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\n\tWa\n\ta\t\x1EV[P\x03\x90V[`\0\x82a\n\x1DWa\n\x1Da\t\x85V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\n5Wa\n5a\t\x1EV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\n\x7FWa\n\x7Fa\x07bV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\xA9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a\x0B\xBBWa\x0B\xBBa\t\xC8V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0C\x01Wa\x0C\x01a\t\xC8V[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15a\x0C\x9AW`@Q\x93P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x85`\x04\x85\x01R`'`$\x85\x01R\x7FABI decoding: invalid byte array`D\x85\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x85\x01R`\x84\x84\xFD[a\x0C\xA9\x83\x87\x83\x01\x88\x88\x01a\x08'V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static OPTIMISTALLOWLIST_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\nW`\x005`\xE0\x1C\x80c\x81\x9F~\x84\x11a\0\xDDW\x80c\x81\x9F~\x84\x14a\x02AW\x80c\xDB\x08=q\x14a\x02vW\x80c\xDB<1c\x14a\x02\x9DW\x80c\xE7\xBD\x80N\x14a\x02\xC4Wa\x01\nV[\x80c:\xC5-\xF7\x14a\x01\x91W\x80cH\x13\xD8\xA6\x14a\x01\xE2W\x80cT\xFDMP\x14a\x02\x05W\x80c^OH\x9A\x14a\x02\x1AW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF5a\x01\xF06`\x04a\x07\xE7V[a\x02\xEBV[`@Q\x90\x15\x15\x81R` \x01a\x01\xD9V[a\x02\ra\x03\x1AV[`@Qa\x01\xD9\x91\x90a\x08WV[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02h\x7Fcoinbase.quest-eligible\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x01\xD9V[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02h\x7Foptimist.can-mint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x02\xF6\x82a\x03\xBDV[\x80a\x03\x05WPa\x03\x05\x82a\x04\nV[\x80a\x03\x14WPa\x03\x14\x82a\x04WV[\x92\x91PPV[``a\x03E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA4V[a\x03n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA4V[a\x03\x97\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA4V[`@Q` \x01a\x03\xA9\x93\x92\x91\x90a\x08\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x03\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x7Foptimist.can-mint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xE1V[`\0a\x03\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x7Fcoinbase.quest-eligible\0\0\0\0\0\0\0\0\0a\x05\xE1V[`\0a\x03\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x7Foptimist.can-mint-from-invite\0\0\0a\x05\xE1V[``\x81`\0\x03a\x04\xE7WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x05\x11W\x80a\x04\xFB\x81a\tMV[\x91Pa\x05\n\x90P`\n\x83a\t\xB4V[\x91Pa\x04\xEBV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05,Wa\x05,a\t\xC8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05VW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x05\xD9Wa\x05k`\x01\x83a\t\xF7V[\x91Pa\x05x`\n\x86a\n\x0EV[a\x05\x83\x90`0a\n\"V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x05\x98Wa\x05\x98a\n:V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x05\xD2`\n\x86a\t\xB4V[\x94Pa\x05ZV[\x94\x93PPPPV[`@Q\x7F)\xB4,\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c)\xB4,\xB5\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x07X\x91\x90\x81\x01\x90a\niV[Q\x11\x94\x93PPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xFCWa\x07\xFCa\x07bV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08 W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x08BW\x81\x81\x01Q\x83\x82\x01R` \x01a\x08*V[\x83\x81\x11\x15a\x08QW`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x08v\x81`@\x85\x01` \x87\x01a\x08'V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x84Qa\x08\xBA\x81\x84` \x89\x01a\x08'V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x08\xF6\x81`\x01\x85\x01` \x8A\x01a\x08'V[`\x01\x92\x01\x91\x82\x01R\x83Qa\t\x11\x81`\x02\x84\x01` \x88\x01a\x08'V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\t~Wa\t~a\t\x1EV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\t\xC3Wa\t\xC3a\t\x85V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\n\tWa\n\ta\t\x1EV[P\x03\x90V[`\0\x82a\n\x1DWa\n\x1Da\t\x85V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\n5Wa\n5a\t\x1EV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\n\x7FWa\n\x7Fa\x07bV[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\xA9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81Q\x81\x81\x11\x15a\x0B\xBBWa\x0B\xBBa\t\xC8V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0C\x01Wa\x0C\x01a\t\xC8V[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15a\x0C\x9AW`@Q\x93P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x85`\x04\x85\x01R`'`$\x85\x01R\x7FABI decoding: invalid byte array`D\x85\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x85\x01R`\x84\x84\xFD[a\x0C\xA9\x83\x87\x83\x01\x88\x88\x01a\x08'V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static OPTIMISTALLOWLIST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OptimistAllowlist<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OptimistAllowlist<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OptimistAllowlist<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OptimistAllowlist<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OptimistAllowlist<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OptimistAllowlist))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OptimistAllowlist<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPTIMISTALLOWLIST_ABI.clone(),
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
                OPTIMISTALLOWLIST_ABI.clone(),
                OPTIMISTALLOWLIST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ALLOWLIST_ATTESTOR` (0xdb3c3163) function
        pub fn allowlist_attestor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([219, 60, 49, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ATTESTATION_STATION` (0xdb083d71) function
        pub fn attestation_station(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([219, 8, 61, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COINBASE_QUEST_ATTESTOR` (0x3ac52df7) function
        pub fn coinbase_quest_attestor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([58, 197, 45, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY` (0x819f7e84) function
        pub fn coinbase_quest_eligible_attestation_key(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([129, 159, 126, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMIST_CAN_MINT_ATTESTATION_KEY` (0xe7bd804e) function
        pub fn optimist_can_mint_attestation_key(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([231, 189, 128, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMIST_INVITER` (0x5e4f489a) function
        pub fn optimist_inviter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([94, 79, 72, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAllowedToMint` (0x4813d8a6) function
        pub fn is_allowed_to_mint(
            &self,
            claimer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([72, 19, 216, 166], claimer)
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OptimistAllowlist<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `ALLOWLIST_ATTESTOR` function with signature `ALLOWLIST_ATTESTOR()` and selector `0xdb3c3163`
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
    #[ethcall(name = "ALLOWLIST_ATTESTOR", abi = "ALLOWLIST_ATTESTOR()")]
    pub struct AllowlistAttestorCall;
    ///Container type for all input parameters for the `ATTESTATION_STATION` function with signature `ATTESTATION_STATION()` and selector `0xdb083d71`
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
    #[ethcall(name = "ATTESTATION_STATION", abi = "ATTESTATION_STATION()")]
    pub struct AttestationStationCall;
    ///Container type for all input parameters for the `COINBASE_QUEST_ATTESTOR` function with signature `COINBASE_QUEST_ATTESTOR()` and selector `0x3ac52df7`
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
    #[ethcall(name = "COINBASE_QUEST_ATTESTOR", abi = "COINBASE_QUEST_ATTESTOR()")]
    pub struct CoinbaseQuestAttestorCall;
    ///Container type for all input parameters for the `COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY` function with signature `COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY()` and selector `0x819f7e84`
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
        name = "COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY",
        abi = "COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY()"
    )]
    pub struct CoinbaseQuestEligibleAttestationKeyCall;
    ///Container type for all input parameters for the `OPTIMIST_CAN_MINT_ATTESTATION_KEY` function with signature `OPTIMIST_CAN_MINT_ATTESTATION_KEY()` and selector `0xe7bd804e`
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
        name = "OPTIMIST_CAN_MINT_ATTESTATION_KEY",
        abi = "OPTIMIST_CAN_MINT_ATTESTATION_KEY()"
    )]
    pub struct OptimistCanMintAttestationKeyCall;
    ///Container type for all input parameters for the `OPTIMIST_INVITER` function with signature `OPTIMIST_INVITER()` and selector `0x5e4f489a`
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
    #[ethcall(name = "OPTIMIST_INVITER", abi = "OPTIMIST_INVITER()")]
    pub struct OptimistInviterCall;
    ///Container type for all input parameters for the `isAllowedToMint` function with signature `isAllowedToMint(address)` and selector `0x4813d8a6`
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
    #[ethcall(name = "isAllowedToMint", abi = "isAllowedToMint(address)")]
    pub struct IsAllowedToMintCall {
        pub claimer: ::ethers::core::types::Address,
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
    pub enum OptimistAllowlistCalls {
        AllowlistAttestor(AllowlistAttestorCall),
        AttestationStation(AttestationStationCall),
        CoinbaseQuestAttestor(CoinbaseQuestAttestorCall),
        CoinbaseQuestEligibleAttestationKey(CoinbaseQuestEligibleAttestationKeyCall),
        OptimistCanMintAttestationKey(OptimistCanMintAttestationKeyCall),
        OptimistInviter(OptimistInviterCall),
        IsAllowedToMint(IsAllowedToMintCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for OptimistAllowlistCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowlistAttestorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowlistAttestor(decoded));
            }
            if let Ok(decoded) = <AttestationStationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationStation(decoded));
            }
            if let Ok(decoded) = <CoinbaseQuestAttestorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CoinbaseQuestAttestor(decoded));
            }
            if let Ok(decoded) = <CoinbaseQuestEligibleAttestationKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CoinbaseQuestEligibleAttestationKey(decoded));
            }
            if let Ok(decoded) = <OptimistCanMintAttestationKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimistCanMintAttestationKey(decoded));
            }
            if let Ok(decoded) = <OptimistInviterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimistInviter(decoded));
            }
            if let Ok(decoded) = <IsAllowedToMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsAllowedToMint(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OptimistAllowlistCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllowlistAttestor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationStation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CoinbaseQuestAttestor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CoinbaseQuestEligibleAttestationKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimistCanMintAttestationKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimistInviter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAllowedToMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OptimistAllowlistCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowlistAttestor(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationStation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CoinbaseQuestAttestor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CoinbaseQuestEligibleAttestationKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimistCanMintAttestationKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimistInviter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAllowedToMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowlistAttestorCall> for OptimistAllowlistCalls {
        fn from(value: AllowlistAttestorCall) -> Self {
            Self::AllowlistAttestor(value)
        }
    }
    impl ::core::convert::From<AttestationStationCall> for OptimistAllowlistCalls {
        fn from(value: AttestationStationCall) -> Self {
            Self::AttestationStation(value)
        }
    }
    impl ::core::convert::From<CoinbaseQuestAttestorCall> for OptimistAllowlistCalls {
        fn from(value: CoinbaseQuestAttestorCall) -> Self {
            Self::CoinbaseQuestAttestor(value)
        }
    }
    impl ::core::convert::From<CoinbaseQuestEligibleAttestationKeyCall>
    for OptimistAllowlistCalls {
        fn from(value: CoinbaseQuestEligibleAttestationKeyCall) -> Self {
            Self::CoinbaseQuestEligibleAttestationKey(value)
        }
    }
    impl ::core::convert::From<OptimistCanMintAttestationKeyCall>
    for OptimistAllowlistCalls {
        fn from(value: OptimistCanMintAttestationKeyCall) -> Self {
            Self::OptimistCanMintAttestationKey(value)
        }
    }
    impl ::core::convert::From<OptimistInviterCall> for OptimistAllowlistCalls {
        fn from(value: OptimistInviterCall) -> Self {
            Self::OptimistInviter(value)
        }
    }
    impl ::core::convert::From<IsAllowedToMintCall> for OptimistAllowlistCalls {
        fn from(value: IsAllowedToMintCall) -> Self {
            Self::IsAllowedToMint(value)
        }
    }
    impl ::core::convert::From<VersionCall> for OptimistAllowlistCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `ALLOWLIST_ATTESTOR` function with signature `ALLOWLIST_ATTESTOR()` and selector `0xdb3c3163`
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
    pub struct AllowlistAttestorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ATTESTATION_STATION` function with signature `ATTESTATION_STATION()` and selector `0xdb083d71`
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
    pub struct AttestationStationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `COINBASE_QUEST_ATTESTOR` function with signature `COINBASE_QUEST_ATTESTOR()` and selector `0x3ac52df7`
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
    pub struct CoinbaseQuestAttestorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY` function with signature `COINBASE_QUEST_ELIGIBLE_ATTESTATION_KEY()` and selector `0x819f7e84`
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
    pub struct CoinbaseQuestEligibleAttestationKeyReturn(pub [u8; 32]);
    ///Container type for all return fields from the `OPTIMIST_CAN_MINT_ATTESTATION_KEY` function with signature `OPTIMIST_CAN_MINT_ATTESTATION_KEY()` and selector `0xe7bd804e`
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
    pub struct OptimistCanMintAttestationKeyReturn(pub [u8; 32]);
    ///Container type for all return fields from the `OPTIMIST_INVITER` function with signature `OPTIMIST_INVITER()` and selector `0x5e4f489a`
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
    pub struct OptimistInviterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isAllowedToMint` function with signature `isAllowedToMint(address)` and selector `0x4813d8a6`
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
    pub struct IsAllowedToMintReturn {
        pub allowed: bool,
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
    pub struct VersionReturn(pub ::std::string::String);
}

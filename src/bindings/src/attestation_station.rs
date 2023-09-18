pub use attestation_station::*;
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
pub mod attestation_station {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("attest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_attestations"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AttestationStation.AttestationData[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_about"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("attestations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attestations"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("AttestationCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AttestationCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("creator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("about"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
    pub static ATTESTATIONSTATION_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01`\x80\x81\x90R`\xA0R`\x02`\xC0R`\x80Q`\xA0Q`\xC0Qa\n\xF3a\0O`\09`\0a\x01\xAD\x01R`\0a\x01\x84\x01R`\0a\x01[\x01Ra\n\xF3`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c)\xB4,\xB5\x14a\0QW\x80cT\xFDMP\x14a\0zW\x80c^\xB5\xEA\x10\x14a\0\x82W\x80cp+\x9D\xEE\x14a\0\x97W[`\0\x80\xFD[a\0da\0_6`\x04a\x04lV[a\0\xAAV[`@Qa\0q\x91\x90a\x05\x1EV[`@Q\x80\x91\x03\x90\xF3[a\0da\x01TV[a\0\x95a\0\x906`\x04a\x058V[a\x01\xF7V[\0[a\0\x95a\0\xA56`\x04a\x06\x87V[a\x02ZV[`\0` \x81\x81R\x93\x81R`@\x80\x82 \x85R\x92\x81R\x82\x81 \x90\x93R\x82R\x90 \x80Ta\0\xD3\x90a\x06\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\0\xFF\x90a\x06\xDEV[\x80\x15a\x01LW\x80`\x1F\x10a\x01!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01LV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``a\x01\x7F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x06V[a\x01\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x06V[a\x01\xD1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x06V[`@Q` \x01a\x01\xE3\x93\x92\x91\x90a\x071V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80`\0[\x81\x81\x10\x15a\x02TW`\0\x84\x84\x83\x81\x81\x10a\x02\x17Wa\x02\x17a\x07\xA7V[\x90P` \x02\x81\x01\x90a\x02)\x91\x90a\x07\xD6V[a\x022\x90a\x08\x14V[\x90Pa\x02K\x81`\0\x01Q\x82` \x01Q\x83`@\x01Qa\x02ZV[P`\x01\x01a\x01\xFBV[PPPPV[3`\0\x90\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x82R\x80\x83 \x85\x84R\x90\x91R\x90 a\x02\x9A\x82\x82a\x08\xDFV[P\x81\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F(q\r\xFE\xCA\xB4=\x1E)\xE0*\xA5k.\x1Ea\x0C\x0B\xAE\x19\x13\\\x9C\xF7\xA8:\x1A\xDBm\xF9m\x85\x84`@Qa\x02\xF9\x91\x90a\x05\x1EV[`@Q\x80\x91\x03\x90\xA4PPPV[``\x81`\0\x03a\x03IWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x03sW\x80a\x03]\x81a\n(V[\x91Pa\x03l\x90P`\n\x83a\n\x8FV[\x91Pa\x03MV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x8EWa\x03\x8Ea\x05\xADV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x03\xB8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x04;Wa\x03\xCD`\x01\x83a\n\xA3V[\x91Pa\x03\xDA`\n\x86a\n\xBAV[a\x03\xE5\x90`0a\n\xCEV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x03\xFAWa\x03\xFAa\x07\xA7V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x044`\n\x86a\n\x8FV[\x94Pa\x03\xBCV[\x94\x93PPPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04gW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\x81W`\0\x80\xFD[a\x04\x8A\x84a\x04CV[\x92Pa\x04\x98` \x85\x01a\x04CV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x04\xC3W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xABV[\x83\x81\x11\x15a\x02TWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x04\xEC\x81` \x86\x01` \x86\x01a\x04\xA8V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x051` \x83\x01\x84a\x04\xD4V[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x05KW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05cW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05wW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\x86W`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x05\x9BW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x05\xEDW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\x08Wa\x06\x08a\x05\xADV[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x06NWa\x06Na\x05\xADV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x06gW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06\x9CW`\0\x80\xFD[a\x06\xA5\x84a\x04CV[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC8W`\0\x80\xFD[a\x06\xD4\x86\x82\x87\x01a\x05\xDCV[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xF2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07+W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x84Qa\x07C\x81\x84` \x89\x01a\x04\xA8V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x07\x7F\x81`\x01\x85\x01` \x8A\x01a\x04\xA8V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x07\x9A\x81`\x02\x84\x01` \x88\x01a\x04\xA8V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x836\x03\x01\x81\x12a\x08\nW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0``\x826\x03\x12\x15a\x08&W`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x08JWa\x08Ja\x05\xADV[\x81`@Ra\x08W\x85a\x04CV[\x83R` \x85\x015` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a\x08wW`\0\x80\xFD[Pa\x08\x846\x82\x86\x01a\x05\xDCV[`@\x83\x01RP\x92\x91PPV[`\x1F\x82\x11\x15a\x08\xDAW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x08\xB7WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08\xD6W\x82\x81U`\x01\x01a\x08\xC3V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xF9Wa\x08\xF9a\x05\xADV[a\t\r\x81a\t\x07\x84Ta\x06\xDEV[\x84a\x08\x90V[` \x80`\x1F\x83\x11`\x01\x81\x14a\t`W`\0\x84\x15a\t*WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x08\xD6V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\t\xADW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\t\x8EV[P\x85\x82\x10\x15a\t\xE9W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\nYWa\nYa\t\xF9V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\n\x9EWa\n\x9Ea\n`V[P\x04\x90V[`\0\x82\x82\x10\x15a\n\xB5Wa\n\xB5a\t\xF9V[P\x03\x90V[`\0\x82a\n\xC9Wa\n\xC9a\n`V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\n\xE1Wa\n\xE1a\t\xF9V[P\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static ATTESTATIONSTATION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c)\xB4,\xB5\x14a\0QW\x80cT\xFDMP\x14a\0zW\x80c^\xB5\xEA\x10\x14a\0\x82W\x80cp+\x9D\xEE\x14a\0\x97W[`\0\x80\xFD[a\0da\0_6`\x04a\x04lV[a\0\xAAV[`@Qa\0q\x91\x90a\x05\x1EV[`@Q\x80\x91\x03\x90\xF3[a\0da\x01TV[a\0\x95a\0\x906`\x04a\x058V[a\x01\xF7V[\0[a\0\x95a\0\xA56`\x04a\x06\x87V[a\x02ZV[`\0` \x81\x81R\x93\x81R`@\x80\x82 \x85R\x92\x81R\x82\x81 \x90\x93R\x82R\x90 \x80Ta\0\xD3\x90a\x06\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\0\xFF\x90a\x06\xDEV[\x80\x15a\x01LW\x80`\x1F\x10a\x01!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01LV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``a\x01\x7F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x06V[a\x01\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x06V[a\x01\xD1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x06V[`@Q` \x01a\x01\xE3\x93\x92\x91\x90a\x071V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80`\0[\x81\x81\x10\x15a\x02TW`\0\x84\x84\x83\x81\x81\x10a\x02\x17Wa\x02\x17a\x07\xA7V[\x90P` \x02\x81\x01\x90a\x02)\x91\x90a\x07\xD6V[a\x022\x90a\x08\x14V[\x90Pa\x02K\x81`\0\x01Q\x82` \x01Q\x83`@\x01Qa\x02ZV[P`\x01\x01a\x01\xFBV[PPPPV[3`\0\x90\x81R` \x81\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x82R\x80\x83 \x85\x84R\x90\x91R\x90 a\x02\x9A\x82\x82a\x08\xDFV[P\x81\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F(q\r\xFE\xCA\xB4=\x1E)\xE0*\xA5k.\x1Ea\x0C\x0B\xAE\x19\x13\\\x9C\xF7\xA8:\x1A\xDBm\xF9m\x85\x84`@Qa\x02\xF9\x91\x90a\x05\x1EV[`@Q\x80\x91\x03\x90\xA4PPPV[``\x81`\0\x03a\x03IWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x03sW\x80a\x03]\x81a\n(V[\x91Pa\x03l\x90P`\n\x83a\n\x8FV[\x91Pa\x03MV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x8EWa\x03\x8Ea\x05\xADV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x03\xB8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x04;Wa\x03\xCD`\x01\x83a\n\xA3V[\x91Pa\x03\xDA`\n\x86a\n\xBAV[a\x03\xE5\x90`0a\n\xCEV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x03\xFAWa\x03\xFAa\x07\xA7V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x044`\n\x86a\n\x8FV[\x94Pa\x03\xBCV[\x94\x93PPPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04gW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\x81W`\0\x80\xFD[a\x04\x8A\x84a\x04CV[\x92Pa\x04\x98` \x85\x01a\x04CV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x04\xC3W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xABV[\x83\x81\x11\x15a\x02TWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x04\xEC\x81` \x86\x01` \x86\x01a\x04\xA8V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x051` \x83\x01\x84a\x04\xD4V[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x05KW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05cW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05wW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\x86W`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x05\x9BW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x05\xEDW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\x08Wa\x06\x08a\x05\xADV[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x06NWa\x06Na\x05\xADV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x06gW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06\x9CW`\0\x80\xFD[a\x06\xA5\x84a\x04CV[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC8W`\0\x80\xFD[a\x06\xD4\x86\x82\x87\x01a\x05\xDCV[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xF2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07+W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x84Qa\x07C\x81\x84` \x89\x01a\x04\xA8V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x07\x7F\x81`\x01\x85\x01` \x8A\x01a\x04\xA8V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x07\x9A\x81`\x02\x84\x01` \x88\x01a\x04\xA8V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x836\x03\x01\x81\x12a\x08\nW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0``\x826\x03\x12\x15a\x08&W`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x08JWa\x08Ja\x05\xADV[\x81`@Ra\x08W\x85a\x04CV[\x83R` \x85\x015` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a\x08wW`\0\x80\xFD[Pa\x08\x846\x82\x86\x01a\x05\xDCV[`@\x83\x01RP\x92\x91PPV[`\x1F\x82\x11\x15a\x08\xDAW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x08\xB7WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08\xD6W\x82\x81U`\x01\x01a\x08\xC3V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xF9Wa\x08\xF9a\x05\xADV[a\t\r\x81a\t\x07\x84Ta\x06\xDEV[\x84a\x08\x90V[` \x80`\x1F\x83\x11`\x01\x81\x14a\t`W`\0\x84\x15a\t*WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x08\xD6V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\t\xADW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\t\x8EV[P\x85\x82\x10\x15a\t\xE9W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\nYWa\nYa\t\xF9V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\n\x9EWa\n\x9Ea\n`V[P\x04\x90V[`\0\x82\x82\x10\x15a\n\xB5Wa\n\xB5a\t\xF9V[P\x03\x90V[`\0\x82a\n\xC9Wa\n\xC9a\n`V[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\n\xE1Wa\n\xE1a\t\xF9V[P\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static ATTESTATIONSTATION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AttestationStation<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationStation<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationStation<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationStation<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationStation<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AttestationStation))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationStation<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATTESTATIONSTATION_ABI.clone(),
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
                ATTESTATIONSTATION_ABI.clone(),
                ATTESTATIONSTATION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `attest` (0x5eb5ea10) function
        pub fn attest(
            &self,
            attestations: ::std::vec::Vec<AttestationData>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 181, 234, 16], attestations)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `attest` (0x702b9dee) function
        pub fn attest_with_about(
            &self,
            about: ::ethers::core::types::Address,
            key: [u8; 32],
            val: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 43, 157, 238], (about, key, val))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `attestations` (0x29b42cb5) function
        pub fn attestations(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([41, 180, 44, 181], (p0, p1, p2))
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
        ///Gets the contract's `AttestationCreated` event
        pub fn attestation_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestationCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestationCreatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AttestationStation<M> {
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
    #[ethevent(
        name = "AttestationCreated",
        abi = "AttestationCreated(address,address,bytes32,bytes)"
    )]
    pub struct AttestationCreatedFilter {
        #[ethevent(indexed)]
        pub creator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub about: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub key: [u8; 32],
        pub val: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `attest` function with signature `attest((address,bytes32,bytes)[])` and selector `0x5eb5ea10`
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
    #[ethcall(name = "attest", abi = "attest((address,bytes32,bytes)[])")]
    pub struct AttestCall {
        pub attestations: ::std::vec::Vec<AttestationData>,
    }
    ///Container type for all input parameters for the `attest` function with signature `attest(address,bytes32,bytes)` and selector `0x702b9dee`
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
    #[ethcall(name = "attest", abi = "attest(address,bytes32,bytes)")]
    pub struct AttestWithAboutCall {
        pub about: ::ethers::core::types::Address,
        pub key: [u8; 32],
        pub val: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `attestations` function with signature `attestations(address,address,bytes32)` and selector `0x29b42cb5`
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
    #[ethcall(name = "attestations", abi = "attestations(address,address,bytes32)")]
    pub struct AttestationsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub [u8; 32],
    );
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
    pub enum AttestationStationCalls {
        Attest(AttestCall),
        AttestWithAbout(AttestWithAboutCall),
        Attestations(AttestationsCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationStationCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AttestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Attest(decoded));
            }
            if let Ok(decoded) = <AttestWithAboutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestWithAbout(decoded));
            }
            if let Ok(decoded) = <AttestationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Attestations(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationStationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Attest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AttestWithAbout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Attestations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AttestationStationCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Attest(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestWithAbout(element) => ::core::fmt::Display::fmt(element, f),
                Self::Attestations(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestCall> for AttestationStationCalls {
        fn from(value: AttestCall) -> Self {
            Self::Attest(value)
        }
    }
    impl ::core::convert::From<AttestWithAboutCall> for AttestationStationCalls {
        fn from(value: AttestWithAboutCall) -> Self {
            Self::AttestWithAbout(value)
        }
    }
    impl ::core::convert::From<AttestationsCall> for AttestationStationCalls {
        fn from(value: AttestationsCall) -> Self {
            Self::Attestations(value)
        }
    }
    impl ::core::convert::From<VersionCall> for AttestationStationCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `attestations` function with signature `attestations(address,address,bytes32)` and selector `0x29b42cb5`
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
    pub struct AttestationsReturn(pub ::ethers::core::types::Bytes);
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
    ///`AttestationData(address,bytes32,bytes)`
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
    pub struct AttestationData {
        pub about: ::ethers::core::types::Address,
        pub key: [u8; 32],
        pub val: ::ethers::core::types::Bytes,
    }
}

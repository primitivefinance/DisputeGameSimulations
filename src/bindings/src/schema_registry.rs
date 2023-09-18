pub use schema_registry::*;
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
pub mod schema_registry {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getSchema"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSchema"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uid"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SchemaRecord"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("register"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("schema"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resolver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISchemaResolver"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revocable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("Registered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Registered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("uid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("registerer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("schema"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadyExists"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SCHEMAREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01`\x80R`\x02`\xA0R`\0`\xC0R`\x80Q`\xA0Q`\xC0Qa\x0B1a\0L`\09`\0`\xFE\x01R`\0`\xD5\x01R`\0`\xAC\x01Ra\x0B1`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cT\xFDMP\x14a\0FW\x80c`\xD7\xA2x\x14a\0dW\x80c\xA2\xEA|n\x14a\0\x85W[`\0\x80\xFD[a\0Na\0\xA5V[`@Qa\0[\x91\x90a\x06\x1FV[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x069V[a\x01HV[`@Q\x90\x81R` \x01a\0[V[a\0\x98a\0\x936`\x04a\x06\xEBV[a\x03\x0CV[`@Qa\0[\x91\x90a\x07\x04V[``a\0\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x044V[a\0\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x044V[a\x01\"\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x044V[`@Q` \x01a\x014\x93\x92\x91\x90a\x07UV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0\x80`@Q\x80`\x80\x01`@R\x80`\0\x80\x1B\x81R` \x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x15\x15\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91Pa\x01\xCA\x90P\x82a\x05qV[`\0\x81\x81R` \x81\x90R`@\x90 T\x90\x91P\x15a\x02\x13W`@Q\x7F#6\x9F\xA6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82R`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x84Q\x81U\x90\x84\x01Q`\x01\x82\x01\x80T\x93\x86\x01Q\x15\x15t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x90\x91U``\x83\x01Q\x83\x91\x90`\x02\x82\x01\x90a\x02\xAF\x90\x82a\x08\x9CV[P\x90PP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xD0\xB8hR\xE2\x1F\x9E_\xA4\xBC;\x0C\xFF\x97W\xFF\xE2C\xD5\x0CKC\x96\x8AB !S\xD6Q\xEA^\x84`@Qa\x02\xFA\x91\x90a\x07\x04V[`@Q\x80\x91\x03\x90\xA3\x96\x95PPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x80\x82\x01R`\0\x82\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x93\x82\x01\x93\x90\x93Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04`\xFF\x16\x15\x15\x92\x82\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x92\x91``\x84\x01\x91\x90a\x03\xAB\x90a\x07\xFAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xD7\x90a\x07\xFAV[\x80\x15a\x04$W\x80`\x1F\x10a\x03\xF9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04$V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``\x81`\0\x03a\x04wWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x04\xA1W\x80a\x04\x8B\x81a\t\xE5V[\x91Pa\x04\x9A\x90P`\n\x83a\nLV[\x91Pa\x04{V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xBCWa\x04\xBCa\x07\xCBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x04\xE6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x05iWa\x04\xFB`\x01\x83a\n`V[\x91Pa\x05\x08`\n\x86a\nyV[a\x05\x13\x90`0a\n\x8DV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x05(Wa\x05(a\n\xA0V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x05b`\n\x86a\nLV[\x94Pa\x04\xEAV[\x94\x93PPPPV[`\0\x81``\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a\x05\x94\x93\x92\x91\x90a\n\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x83\x81\x10\x15a\x05\xCCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xB4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05\xED\x81` \x86\x01` \x86\x01a\x05\xB1V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x062` \x83\x01\x84a\x05\xD5V[\x93\x92PPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06OW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06gW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x06{W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\x8AW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x06\x9CW`\0\x80\xFD[` \x92\x83\x01\x96P\x94PP\x85\x015s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xCBW`\0\x80\xFD[\x91P`@\x85\x015\x80\x15\x15\x81\x14a\x06\xE0W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x06\xFDW`\0\x80\xFD[P5\x91\x90PV[` \x81R\x81Q` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x15\x15``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra\x05i`\xA0\x84\x01\x82a\x05\xD5V[`\0\x84Qa\x07g\x81\x84` \x89\x01a\x05\xB1V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x07\xA3\x81`\x01\x85\x01` \x8A\x01a\x05\xB1V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x07\xBE\x81`\x02\x84\x01` \x88\x01a\x05\xB1V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x08\x0EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08GW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x08\x97W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x08tWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08\x93W\x82\x81U`\x01\x01a\x08\x80V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xB6Wa\x08\xB6a\x07\xCBV[a\x08\xCA\x81a\x08\xC4\x84Ta\x07\xFAV[\x84a\x08MV[` \x80`\x1F\x83\x11`\x01\x81\x14a\t\x1DW`\0\x84\x15a\x08\xE7WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x08\x93V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\tjW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\tKV[P\x85\x82\x10\x15a\t\xA6W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\n\x16Wa\n\x16a\t\xB6V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\n[Wa\n[a\n\x1DV[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\nsWa\nsa\t\xB6V[\x92\x91PPV[`\0\x82a\n\x88Wa\n\x88a\n\x1DV[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\nsWa\nsa\t\xB6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x84Qa\n\xE1\x81\x84` \x89\x01a\x05\xB1V[``\x94\x90\x94\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x93\x01\x90\x81R\x90\x15\x15`\xF8\x1B`\x14\x82\x01R`\x15\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The bytecode of the contract.
    pub static SCHEMAREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cT\xFDMP\x14a\0FW\x80c`\xD7\xA2x\x14a\0dW\x80c\xA2\xEA|n\x14a\0\x85W[`\0\x80\xFD[a\0Na\0\xA5V[`@Qa\0[\x91\x90a\x06\x1FV[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x069V[a\x01HV[`@Q\x90\x81R` \x01a\0[V[a\0\x98a\0\x936`\x04a\x06\xEBV[a\x03\x0CV[`@Qa\0[\x91\x90a\x07\x04V[``a\0\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x044V[a\0\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x044V[a\x01\"\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x044V[`@Q` \x01a\x014\x93\x92\x91\x90a\x07UV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0\x80`@Q\x80`\x80\x01`@R\x80`\0\x80\x1B\x81R` \x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x15\x15\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91Pa\x01\xCA\x90P\x82a\x05qV[`\0\x81\x81R` \x81\x90R`@\x90 T\x90\x91P\x15a\x02\x13W`@Q\x7F#6\x9F\xA6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82R`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x84Q\x81U\x90\x84\x01Q`\x01\x82\x01\x80T\x93\x86\x01Q\x15\x15t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x90\x91U``\x83\x01Q\x83\x91\x90`\x02\x82\x01\x90a\x02\xAF\x90\x82a\x08\x9CV[P\x90PP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xD0\xB8hR\xE2\x1F\x9E_\xA4\xBC;\x0C\xFF\x97W\xFF\xE2C\xD5\x0CKC\x96\x8AB !S\xD6Q\xEA^\x84`@Qa\x02\xFA\x91\x90a\x07\x04V[`@Q\x80\x91\x03\x90\xA3\x96\x95PPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x80\x82\x01R`\0\x82\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x93\x82\x01\x93\x90\x93Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04`\xFF\x16\x15\x15\x92\x82\x01\x92\x90\x92R`\x02\x82\x01\x80T\x91\x92\x91``\x84\x01\x91\x90a\x03\xAB\x90a\x07\xFAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xD7\x90a\x07\xFAV[\x80\x15a\x04$W\x80`\x1F\x10a\x03\xF9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04$V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[``\x81`\0\x03a\x04wWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x04\xA1W\x80a\x04\x8B\x81a\t\xE5V[\x91Pa\x04\x9A\x90P`\n\x83a\nLV[\x91Pa\x04{V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xBCWa\x04\xBCa\x07\xCBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x04\xE6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x05iWa\x04\xFB`\x01\x83a\n`V[\x91Pa\x05\x08`\n\x86a\nyV[a\x05\x13\x90`0a\n\x8DV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x05(Wa\x05(a\n\xA0V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x05b`\n\x86a\nLV[\x94Pa\x04\xEAV[\x94\x93PPPPV[`\0\x81``\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a\x05\x94\x93\x92\x91\x90a\n\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x83\x81\x10\x15a\x05\xCCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xB4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05\xED\x81` \x86\x01` \x86\x01a\x05\xB1V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x062` \x83\x01\x84a\x05\xD5V[\x93\x92PPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06OW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06gW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x06{W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\x8AW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x06\x9CW`\0\x80\xFD[` \x92\x83\x01\x96P\x94PP\x85\x015s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xCBW`\0\x80\xFD[\x91P`@\x85\x015\x80\x15\x15\x81\x14a\x06\xE0W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x06\xFDW`\0\x80\xFD[P5\x91\x90PV[` \x81R\x81Q` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x15\x15``\x82\x01R`\0``\x83\x01Q`\x80\x80\x84\x01Ra\x05i`\xA0\x84\x01\x82a\x05\xD5V[`\0\x84Qa\x07g\x81\x84` \x89\x01a\x05\xB1V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x07\xA3\x81`\x01\x85\x01` \x8A\x01a\x05\xB1V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x07\xBE\x81`\x02\x84\x01` \x88\x01a\x05\xB1V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x08\x0EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08GW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x08\x97W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x08tWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08\x93W\x82\x81U`\x01\x01a\x08\x80V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xB6Wa\x08\xB6a\x07\xCBV[a\x08\xCA\x81a\x08\xC4\x84Ta\x07\xFAV[\x84a\x08MV[` \x80`\x1F\x83\x11`\x01\x81\x14a\t\x1DW`\0\x84\x15a\x08\xE7WP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x08\x93V[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\tjW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\tKV[P\x85\x82\x10\x15a\t\xA6W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\n\x16Wa\n\x16a\t\xB6V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\n[Wa\n[a\n\x1DV[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\nsWa\nsa\t\xB6V[\x92\x91PPV[`\0\x82a\n\x88Wa\n\x88a\n\x1DV[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\nsWa\nsa\t\xB6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x84Qa\n\xE1\x81\x84` \x89\x01a\x05\xB1V[``\x94\x90\x94\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x93\x01\x90\x81R\x90\x15\x15`\xF8\x1B`\x14\x82\x01R`\x15\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The deployed bytecode of the contract.
    pub static SCHEMAREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SchemaRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SchemaRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SchemaRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SchemaRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SchemaRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SchemaRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SchemaRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SCHEMAREGISTRY_ABI.clone(),
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
                SCHEMAREGISTRY_ABI.clone(),
                SCHEMAREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getSchema` (0xa2ea7c6e) function
        pub fn get_schema(
            &self,
            uid: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, SchemaRecord> {
            self.0
                .method_hash([162, 234, 124, 110], uid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0x60d7a278) function
        pub fn register(
            &self,
            schema: ::std::string::String,
            resolver: ::ethers::core::types::Address,
            revocable: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([96, 215, 162, 120], (schema, resolver, revocable))
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
        ///Gets the contract's `Registered` event
        pub fn registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegisteredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegisteredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SchemaRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyExists` with signature `AlreadyExists()` and selector `0x23369fa6`
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
    #[etherror(name = "AlreadyExists", abi = "AlreadyExists()")]
    pub struct AlreadyExists;
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
        name = "Registered",
        abi = "Registered(bytes32,address,(bytes32,address,bool,string))"
    )]
    pub struct RegisteredFilter {
        #[ethevent(indexed)]
        pub uid: [u8; 32],
        #[ethevent(indexed)]
        pub registerer: ::ethers::core::types::Address,
        pub schema: SchemaRecord,
    }
    ///Container type for all input parameters for the `getSchema` function with signature `getSchema(bytes32)` and selector `0xa2ea7c6e`
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
    #[ethcall(name = "getSchema", abi = "getSchema(bytes32)")]
    pub struct GetSchemaCall {
        pub uid: [u8; 32],
    }
    ///Container type for all input parameters for the `register` function with signature `register(string,address,bool)` and selector `0x60d7a278`
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
    #[ethcall(name = "register", abi = "register(string,address,bool)")]
    pub struct RegisterCall {
        pub schema: ::std::string::String,
        pub resolver: ::ethers::core::types::Address,
        pub revocable: bool,
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
    pub enum SchemaRegistryCalls {
        GetSchema(GetSchemaCall),
        Register(RegisterCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for SchemaRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetSchemaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSchema(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SchemaRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetSchema(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SchemaRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetSchema(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetSchemaCall> for SchemaRegistryCalls {
        fn from(value: GetSchemaCall) -> Self {
            Self::GetSchema(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for SchemaRegistryCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<VersionCall> for SchemaRegistryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `getSchema` function with signature `getSchema(bytes32)` and selector `0xa2ea7c6e`
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
    pub struct GetSchemaReturn(pub SchemaRecord);
    ///Container type for all return fields from the `register` function with signature `register(string,address,bool)` and selector `0x60d7a278`
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
    pub struct RegisterReturn(pub [u8; 32]);
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

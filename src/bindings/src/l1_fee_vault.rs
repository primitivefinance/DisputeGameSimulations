pub use l1_fee_vault::*;
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
pub mod l1_fee_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_recipient"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_minWithdrawalAmount"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_withdrawalNetwork"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "enum FeeVault.WithdrawalNetwork",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MIN_WITHDRAWAL_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MIN_WITHDRAWAL_AMOUNT",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("RECIPIENT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RECIPIENT"),
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
                    ::std::borrow::ToOwned::to_owned("WITHDRAWAL_NETWORK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WITHDRAWAL_NETWORK"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum FeeVault.WithdrawalNetwork",
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
                    ::std::borrow::ToOwned::to_owned("totalProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalProcessed"),
                            inputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Withdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdrawal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdrawal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalNetwork"),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static L1FEEVAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08\xD18\x03\x80a\x08\xD1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0yV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\xA0R`\x80\x82\x90R\x82\x82\x82\x80`\x01\x81\x11\x15a\0VWa\0Va\0\xCCV[`\xC0\x81`\x01\x81\x11\x15a\0jWa\0ja\0\xCCV[\x81RPPPPPPPPa\0\xE2V[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\x8EW`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xA5W`\0\x80\xFD[` \x85\x01Q`@\x86\x01Q\x91\x94P\x92P`\x02\x81\x10a\0\xC1W`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x80Q`\xA0Q`\xC0Qa\x07\x90a\x01A`\09`\0\x81\x81a\x01v\x01R\x81\x81a\x03\x89\x01Ra\x03\xC4\x01R`\0\x81\x81`\x87\x01R\x81\x81a\x02\xD8\x01R\x81\x81a\x03g\x01R\x81\x81a\x03\xFD\x01Ra\x05d\x01R`\0\x81\x81a\x01\xB7\x01Ra\x01\xDB\x01Ra\x07\x90`\0\xF3\xFE`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\x84A\x1De\x11a\0CW\x80c\x84A\x1De\x14a\x01@W\x80c\xD0\xE1/\x90\x14a\x01dW\x80c\xD3\xE5y+\x14a\x01\xA5W`\0\x80\xFD[\x80c\r\x90\x19\xE1\x14a\0uW\x80c<\xCF\xD6\x0B\x14a\0\xD3W\x80cT\xFDMP\x14a\0\xEAW`\0\x80\xFD[6a\0pW\0[`\0\x80\xFD[4\x80\x15a\0\x81W`\0\x80\xFD[Pa\0\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xDFW`\0\x80\xFD[Pa\0\xE8a\x01\xD9V[\0[4\x80\x15a\0\xF6W`\0\x80\xFD[Pa\x013`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.4.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xCA\x91\x90a\x060V[4\x80\x15a\x01LW`\0\x80\xFD[Pa\x01V`\0T\x81V[`@Q\x90\x81R` \x01a\0\xCAV[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01\x98\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\0\xCA\x91\x90a\x06\xB4V[4\x80\x15a\x01\xB1W`\0\x80\xFD[Pa\x01V\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0G\x10\x15a\x02\xB4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FFeeVault: withdrawal amount must`D\x82\x01R\x7F be greater than minimum withdra`d\x82\x01R\x7Fwal amount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0G\x90P\x80`\0\x80\x82\x82Ta\x02\xCA\x91\x90a\x06\xC8V[\x90\x91UPP`@\x80Q\x82\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R3\x81\x83\x01R\x90Q\x7F\xC8\xA2\x11\xCCd\xB6\xED\x1BPYZ\x9F\xCB\x192\xB6\xD1\xE5\xA6\xE8\xEF\x15\xB6\x0E[\x1F\x98\x8E\xA9\x08k\xBA\x91\x81\x90\x03``\x01\x90\xA1\x7F8\xE0L\xBE\xB8\xC1\x0F\x8FV\x86\x18\xAAu\xBE\x0F\x10\xB6r\x9B\x8BB7t;M\xE2\x0C\xBC\xDE(9\xEE\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qa\x03\xB8\x94\x93\x92\x91\x90a\x07\x07V[`@Q\x80\x91\x03\x90\xA1`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01\x81\x11\x15a\x03\xF4Wa\x03\xF4a\x06JV[\x03a\x05\rW`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04sW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04xV[``\x91P[PP\x90P\x80a\x05\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FFeeVault: failed to send ETH to `D\x82\x01R\x7FL2 fee recipient\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xABV[PPV[`@\x80Q` \x81\x01\x82R`\0\x81R\x90Q\x7F\xE1\x10\x13\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\x91c\xE1\x10\x13\xDD\x91\x84\x91a\x05\x90\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x88\xB8\x91`\x04\x01a\x07HV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x05\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xBDW=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x05\xEBW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05\xCFV[\x81\x81\x11\x15a\x05\xFDW`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06C` \x83\x01\x84a\x05\xC5V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\x06\xB0W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x06\xC2\x82\x84a\x06yV[\x92\x91PPV[`\0\x82\x19\x82\x11\x15a\x07\x02W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[\x84\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16` \x83\x01R\x83\x16`@\x82\x01R`\x80\x81\x01a\x07?``\x83\x01\x84a\x06yV[\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a\x07?``\x83\x01\x84a\x05\xC5V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L1FEEVAULT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\x84A\x1De\x11a\0CW\x80c\x84A\x1De\x14a\x01@W\x80c\xD0\xE1/\x90\x14a\x01dW\x80c\xD3\xE5y+\x14a\x01\xA5W`\0\x80\xFD[\x80c\r\x90\x19\xE1\x14a\0uW\x80c<\xCF\xD6\x0B\x14a\0\xD3W\x80cT\xFDMP\x14a\0\xEAW`\0\x80\xFD[6a\0pW\0[`\0\x80\xFD[4\x80\x15a\0\x81W`\0\x80\xFD[Pa\0\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xDFW`\0\x80\xFD[Pa\0\xE8a\x01\xD9V[\0[4\x80\x15a\0\xF6W`\0\x80\xFD[Pa\x013`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.4.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xCA\x91\x90a\x060V[4\x80\x15a\x01LW`\0\x80\xFD[Pa\x01V`\0T\x81V[`@Q\x90\x81R` \x01a\0\xCAV[4\x80\x15a\x01pW`\0\x80\xFD[Pa\x01\x98\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\0\xCA\x91\x90a\x06\xB4V[4\x80\x15a\x01\xB1W`\0\x80\xFD[Pa\x01V\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0G\x10\x15a\x02\xB4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FFeeVault: withdrawal amount must`D\x82\x01R\x7F be greater than minimum withdra`d\x82\x01R\x7Fwal amount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0G\x90P\x80`\0\x80\x82\x82Ta\x02\xCA\x91\x90a\x06\xC8V[\x90\x91UPP`@\x80Q\x82\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R3\x81\x83\x01R\x90Q\x7F\xC8\xA2\x11\xCCd\xB6\xED\x1BPYZ\x9F\xCB\x192\xB6\xD1\xE5\xA6\xE8\xEF\x15\xB6\x0E[\x1F\x98\x8E\xA9\x08k\xBA\x91\x81\x90\x03``\x01\x90\xA1\x7F8\xE0L\xBE\xB8\xC1\x0F\x8FV\x86\x18\xAAu\xBE\x0F\x10\xB6r\x9B\x8BB7t;M\xE2\x0C\xBC\xDE(9\xEE\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qa\x03\xB8\x94\x93\x92\x91\x90a\x07\x07V[`@Q\x80\x91\x03\x90\xA1`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01\x81\x11\x15a\x03\xF4Wa\x03\xF4a\x06JV[\x03a\x05\rW`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04sW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04xV[``\x91P[PP\x90P\x80a\x05\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FFeeVault: failed to send ETH to `D\x82\x01R\x7FL2 fee recipient\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xABV[PPV[`@\x80Q` \x81\x01\x82R`\0\x81R\x90Q\x7F\xE1\x10\x13\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\x91c\xE1\x10\x13\xDD\x91\x84\x91a\x05\x90\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x88\xB8\x91`\x04\x01a\x07HV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x05\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xBDW=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x05\xEBW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05\xCFV[\x81\x81\x11\x15a\x05\xFDW`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06C` \x83\x01\x84a\x05\xC5V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\x06\xB0W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x06\xC2\x82\x84a\x06yV[\x92\x91PPV[`\0\x82\x19\x82\x11\x15a\x07\x02W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[\x84\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16` \x83\x01R\x83\x16`@\x82\x01R`\x80\x81\x01a\x07?``\x83\x01\x84a\x06yV[\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a\x07?``\x83\x01\x84a\x05\xC5V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L1FEEVAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L1FeeVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L1FeeVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L1FeeVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L1FeeVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L1FeeVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L1FeeVault)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L1FeeVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L1FEEVAULT_ABI.clone(),
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
                L1FEEVAULT_ABI.clone(),
                L1FEEVAULT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MIN_WITHDRAWAL_AMOUNT` (0xd3e5792b) function
        pub fn min_withdrawal_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([211, 229, 121, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RECIPIENT` (0x0d9019e1) function
        pub fn recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 144, 25, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WITHDRAWAL_NETWORK` (0xd0e12f90) function
        pub fn withdrawal_network(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([208, 225, 47, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalProcessed` (0x84411d65) function
        pub fn total_processed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([132, 65, 29, 101], ())
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
        ///Calls the contract's `withdraw` (0x3ccfd60b) function
        pub fn withdraw(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Withdrawal` event
        pub fn withdrawal_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Withdrawal1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdrawal` event
        pub fn withdrawal_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Withdrawal2Filter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            L1FeeVaultEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L1FeeVault<M> {
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
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(uint256,address,address)")]
    pub struct Withdrawal1Filter {
        pub value: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
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
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(uint256,address,address,uint8)")]
    pub struct Withdrawal2Filter {
        pub value: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub withdrawal_network: u8,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L1FeeVaultEvents {
        Withdrawal1Filter(Withdrawal1Filter),
        Withdrawal2Filter(Withdrawal2Filter),
    }
    impl ::ethers::contract::EthLogDecode for L1FeeVaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = Withdrawal1Filter::decode_log(log) {
                return Ok(L1FeeVaultEvents::Withdrawal1Filter(decoded));
            }
            if let Ok(decoded) = Withdrawal2Filter::decode_log(log) {
                return Ok(L1FeeVaultEvents::Withdrawal2Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L1FeeVaultEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Withdrawal1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdrawal2Filter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Withdrawal1Filter> for L1FeeVaultEvents {
        fn from(value: Withdrawal1Filter) -> Self {
            Self::Withdrawal1Filter(value)
        }
    }
    impl ::core::convert::From<Withdrawal2Filter> for L1FeeVaultEvents {
        fn from(value: Withdrawal2Filter) -> Self {
            Self::Withdrawal2Filter(value)
        }
    }
    ///Container type for all input parameters for the `MIN_WITHDRAWAL_AMOUNT` function with signature `MIN_WITHDRAWAL_AMOUNT()` and selector `0xd3e5792b`
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
    #[ethcall(name = "MIN_WITHDRAWAL_AMOUNT", abi = "MIN_WITHDRAWAL_AMOUNT()")]
    pub struct MinWithdrawalAmountCall;
    ///Container type for all input parameters for the `RECIPIENT` function with signature `RECIPIENT()` and selector `0x0d9019e1`
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
    #[ethcall(name = "RECIPIENT", abi = "RECIPIENT()")]
    pub struct RecipientCall;
    ///Container type for all input parameters for the `WITHDRAWAL_NETWORK` function with signature `WITHDRAWAL_NETWORK()` and selector `0xd0e12f90`
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
    #[ethcall(name = "WITHDRAWAL_NETWORK", abi = "WITHDRAWAL_NETWORK()")]
    pub struct WithdrawalNetworkCall;
    ///Container type for all input parameters for the `totalProcessed` function with signature `totalProcessed()` and selector `0x84411d65`
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
    #[ethcall(name = "totalProcessed", abi = "totalProcessed()")]
    pub struct TotalProcessedCall;
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
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw()` and selector `0x3ccfd60b`
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
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L1FeeVaultCalls {
        MinWithdrawalAmount(MinWithdrawalAmountCall),
        Recipient(RecipientCall),
        WithdrawalNetwork(WithdrawalNetworkCall),
        TotalProcessed(TotalProcessedCall),
        Version(VersionCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for L1FeeVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MinWithdrawalAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinWithdrawalAmount(decoded));
            }
            if let Ok(decoded) = <RecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Recipient(decoded));
            }
            if let Ok(decoded) = <WithdrawalNetworkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawalNetwork(decoded));
            }
            if let Ok(decoded) = <TotalProcessedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalProcessed(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L1FeeVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MinWithdrawalAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Recipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalNetwork(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalProcessed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for L1FeeVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinWithdrawalAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Recipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalNetwork(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalProcessed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MinWithdrawalAmountCall> for L1FeeVaultCalls {
        fn from(value: MinWithdrawalAmountCall) -> Self {
            Self::MinWithdrawalAmount(value)
        }
    }
    impl ::core::convert::From<RecipientCall> for L1FeeVaultCalls {
        fn from(value: RecipientCall) -> Self {
            Self::Recipient(value)
        }
    }
    impl ::core::convert::From<WithdrawalNetworkCall> for L1FeeVaultCalls {
        fn from(value: WithdrawalNetworkCall) -> Self {
            Self::WithdrawalNetwork(value)
        }
    }
    impl ::core::convert::From<TotalProcessedCall> for L1FeeVaultCalls {
        fn from(value: TotalProcessedCall) -> Self {
            Self::TotalProcessed(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L1FeeVaultCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for L1FeeVaultCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `MIN_WITHDRAWAL_AMOUNT` function with signature `MIN_WITHDRAWAL_AMOUNT()` and selector `0xd3e5792b`
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
    pub struct MinWithdrawalAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RECIPIENT` function with signature `RECIPIENT()` and selector `0x0d9019e1`
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
    pub struct RecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `WITHDRAWAL_NETWORK` function with signature `WITHDRAWAL_NETWORK()` and selector `0xd0e12f90`
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
    pub struct WithdrawalNetworkReturn(pub u8);
    ///Container type for all return fields from the `totalProcessed` function with signature `totalProcessed()` and selector `0x84411d65`
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
    pub struct TotalProcessedReturn(pub ::ethers::core::types::U256);
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

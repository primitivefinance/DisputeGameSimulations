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
pub mod sequencer_fee_vault {
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
                    ::std::borrow::ToOwned::to_owned("l1FeeWallet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1FeeWallet"),
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
    pub static SEQUENCERFEEVAULT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@R4\x80\x15a\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x11\xB48\x03\x80a\x11\xB4\x839\x81\x01`@\x81\x90Ra\0}\x91a\0\xDCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\xA0R`\x80\x82\x90R`\x01`\x03\x81\x85\x85\x85\x80\x84\x81\x11\x15a\0\xA8Wa\0\xA8a\x01zV[`\xC0\x81`\x01\x81\x11\x15a\0\xBCWa\0\xBCa\x01zV[\x90RPPP`\xE0\x93\x90\x93Ra\x01\0\x91\x90\x91Ra\x01 RPa\x01\x90\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x01<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01SW`\0\x80\xFD[` \x85\x01Q`@\x86\x01Q\x91\x94P\x92P`\x02\x81\x10a\x01oW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x0F\x9Da\x02\x17`\09`\0a\n\xBA\x01R`\0a\n\x91\x01R`\0a\nh\x01R`\0\x81\x81a\x04Y\x01R\x81\x81a\x07\xA3\x01Ra\x07\xDE\x01R`\0\x81\x81a\x01\x96\x01R\x81\x81a\x05\xCF\x01R\x81\x81a\x06\xF2\x01R\x81\x81a\x07\x81\x01R\x81\x81a\x08\x17\x01Ra\t~\x01R`\0\x81\x81a\x05\x1C\x01Ra\x05\xF5\x01Ra\x0F\x9D`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c\x84A\x1De\x11a\0NW\x80c\x84A\x1De\x14a\x03\x1FW\x80c\xD0\xE1/\x90\x14a\x03\xC5W\x80c\xD3\xE5y+\x14a\x04\x88W\x80c\xD4\xFF\x92\x18\x14a\x05>Wa\0{V[\x80c\r\x90\x19\xE1\x14a\x01\x02W\x80c<\xCF\xD6\x0B\x14a\x01\xE2W\x80cT\xFDMP\x14a\x02{Wa\0{V[6a\0{W\0[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01\x90\x81R\x7Fk defined\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x01\x90W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ya\x05\xF3V[\0[4\x80\x15a\x03\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\x12a\naV[`@Qa\x01\xD9\x91\x90a\x0C\xBBV[4\x80\x15a\x03\xADW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xB7`\0T\x81V[`@Q\x90\x81R` \x01a\x01\xD9V[4\x80\x15a\x04SW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\x01\xD9\x91\x90a\r?V[4\x80\x15a\x05\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xB7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0G\x10\x15a\x06\xCEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FFeeVault: withdrawal amount must`D\x82\x01R\x7F be greater than minimum withdra`d\x82\x01R\x7Fwal amount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0G\x90P\x80`\0\x80\x82\x82Ta\x06\xE4\x91\x90a\r\x82V[\x90\x91UPP`@\x80Q\x82\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R3\x81\x83\x01R\x90Q\x7F\xC8\xA2\x11\xCCd\xB6\xED\x1BPYZ\x9F\xCB\x192\xB6\xD1\xE5\xA6\xE8\xEF\x15\xB6\x0E[\x1F\x98\x8E\xA9\x08k\xBA\x91\x81\x90\x03``\x01\x90\xA1\x7F8\xE0L\xBE\xB8\xC1\x0F\x8FV\x86\x18\xAAu\xBE\x0F\x10\xB6r\x9B\x8BB7t;M\xE2\x0C\xBC\xDE(9\xEE\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qa\x07\xD2\x94\x93\x92\x91\x90a\r\x9AV[`@Q\x80\x91\x03\x90\xA1`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01\x81\x11\x15a\x08\x0EWa\x08\x0Ea\x0C\xD5V[\x03a\t'W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08\x8DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x92V[``\x91P[PP\x90P\x80a\t#W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FFeeVault: failed to send ETH to `D\x82\x01R\x7FL2 fee recipient\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xC5V[PPV[`@\x80Q` \x81\x01\x82R`\0\x81R\x90Q\x7F\xE1\x10\x13\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\x91c\xE1\x10\x13\xDD\x91\x84\x91a\t\xAA\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x88\xB8\x91`\x04\x01a\r\xDBV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\nEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\nYW=`\0\x80>=`\0\xFD[PPPPPPV[``a\n\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x04V[a\n\xB5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x04V[a\n\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x04V[`@Q` \x01a\n\xF0\x93\x92\x91\x90a\x0E\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x81`\0\x03a\x0BGWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x0BqW\x80a\x0B[\x81a\x0E\x8CV[\x91Pa\x0Bj\x90P`\n\x83a\x0E\xF3V[\x91Pa\x0BKV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x8CWa\x0B\x8Ca\x0F\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xB6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x0C9Wa\x0B\xCB`\x01\x83a\x0F6V[\x91Pa\x0B\xD8`\n\x86a\x0FMV[a\x0B\xE3\x90`0a\r\x82V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x0B\xF8Wa\x0B\xF8a\x0FaV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0C2`\n\x86a\x0E\xF3V[\x94Pa\x0B\xBAV[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\x0C\\W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0CDV[\x83\x81\x11\x15a\x0CkW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x0C\x89\x81` \x86\x01` \x86\x01a\x0CAV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0C\xCE` \x83\x01\x84a\x0CqV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\r;W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\rM\x82\x84a\r\x04V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\r\x95Wa\r\x95a\rSV[P\x01\x90V[\x84\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16` \x83\x01R\x83\x16`@\x82\x01R`\x80\x81\x01a\r\xD2``\x83\x01\x84a\r\x04V[\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a\r\xD2``\x83\x01\x84a\x0CqV[`\0\x84Qa\x0E(\x81\x84` \x89\x01a\x0CAV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x0Ed\x81`\x01\x85\x01` \x8A\x01a\x0CAV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x0E\x7F\x81`\x02\x84\x01` \x88\x01a\x0CAV[\x01`\x02\x01\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\xBDWa\x0E\xBDa\rSV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x0F\x02Wa\x0F\x02a\x0E\xC4V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x0FHWa\x0FHa\rSV[P\x03\x90V[`\0\x82a\x0F\\Wa\x0F\\a\x0E\xC4V[P\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static SEQUENCERFEEVAULT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c\x84A\x1De\x11a\0NW\x80c\x84A\x1De\x14a\x03\x1FW\x80c\xD0\xE1/\x90\x14a\x03\xC5W\x80c\xD3\xE5y+\x14a\x04\x88W\x80c\xD4\xFF\x92\x18\x14a\x05>Wa\0{V[\x80c\r\x90\x19\xE1\x14a\x01\x02W\x80c<\xCF\xD6\x0B\x14a\x01\xE2W\x80cT\xFDMP\x14a\x02{Wa\0{V[6a\0{W\0[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01\x90\x81R\x7Fk defined\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x01\x90W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x01\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02ya\x05\xF3V[\0[4\x80\x15a\x03\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\x12a\naV[`@Qa\x01\xD9\x91\x90a\x0C\xBBV[4\x80\x15a\x03\xADW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xB7`\0T\x81V[`@Q\x90\x81R` \x01a\x01\xD9V[4\x80\x15a\x04SW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\x01\xD9\x91\x90a\r?V[4\x80\x15a\x05\x16W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xB7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xB8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0G\x10\x15a\x06\xCEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FFeeVault: withdrawal amount must`D\x82\x01R\x7F be greater than minimum withdra`d\x82\x01R\x7Fwal amount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0G\x90P\x80`\0\x80\x82\x82Ta\x06\xE4\x91\x90a\r\x82V[\x90\x91UPP`@\x80Q\x82\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R3\x81\x83\x01R\x90Q\x7F\xC8\xA2\x11\xCCd\xB6\xED\x1BPYZ\x9F\xCB\x192\xB6\xD1\xE5\xA6\xE8\xEF\x15\xB6\x0E[\x1F\x98\x8E\xA9\x08k\xBA\x91\x81\x90\x03``\x01\x90\xA1\x7F8\xE0L\xBE\xB8\xC1\x0F\x8FV\x86\x18\xAAu\xBE\x0F\x10\xB6r\x9B\x8BB7t;M\xE2\x0C\xBC\xDE(9\xEE\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x003\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qa\x07\xD2\x94\x93\x92\x91\x90a\r\x9AV[`@Q\x80\x91\x03\x90\xA1`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01\x81\x11\x15a\x08\x0EWa\x08\x0Ea\x0C\xD5V[\x03a\t'W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08\x8DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x92V[``\x91P[PP\x90P\x80a\t#W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FFeeVault: failed to send ETH to `D\x82\x01R\x7FL2 fee recipient\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xC5V[PPV[`@\x80Q` \x81\x01\x82R`\0\x81R\x90Q\x7F\xE1\x10\x13\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\x91c\xE1\x10\x13\xDD\x91\x84\x91a\t\xAA\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x88\xB8\x91`\x04\x01a\r\xDBV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\nEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\nYW=`\0\x80>=`\0\xFD[PPPPPPV[``a\n\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x04V[a\n\xB5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x04V[a\n\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x04V[`@Q` \x01a\n\xF0\x93\x92\x91\x90a\x0E\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x81`\0\x03a\x0BGWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x0BqW\x80a\x0B[\x81a\x0E\x8CV[\x91Pa\x0Bj\x90P`\n\x83a\x0E\xF3V[\x91Pa\x0BKV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x8CWa\x0B\x8Ca\x0F\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xB6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x0C9Wa\x0B\xCB`\x01\x83a\x0F6V[\x91Pa\x0B\xD8`\n\x86a\x0FMV[a\x0B\xE3\x90`0a\r\x82V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x0B\xF8Wa\x0B\xF8a\x0FaV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0C2`\n\x86a\x0E\xF3V[\x94Pa\x0B\xBAV[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\x0C\\W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0CDV[\x83\x81\x11\x15a\x0CkW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x0C\x89\x81` \x86\x01` \x86\x01a\x0CAV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0C\xCE` \x83\x01\x84a\x0CqV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\r;W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\rM\x82\x84a\r\x04V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\r\x95Wa\r\x95a\rSV[P\x01\x90V[\x84\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16` \x83\x01R\x83\x16`@\x82\x01R`\x80\x81\x01a\r\xD2``\x83\x01\x84a\r\x04V[\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R```@\x82\x01R`\0a\r\xD2``\x83\x01\x84a\x0CqV[`\0\x84Qa\x0E(\x81\x84` \x89\x01a\x0CAV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x0Ed\x81`\x01\x85\x01` \x8A\x01a\x0CAV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x0E\x7F\x81`\x02\x84\x01` \x88\x01a\x0CAV[\x01`\x02\x01\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\xBDWa\x0E\xBDa\rSV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x0F\x02Wa\x0F\x02a\x0E\xC4V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x0FHWa\x0FHa\rSV[P\x03\x90V[`\0\x82a\x0F\\Wa\x0F\\a\x0E\xC4V[P\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static SEQUENCERFEEVAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SequencerFeeVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SequencerFeeVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SequencerFeeVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SequencerFeeVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SequencerFeeVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SequencerFeeVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SequencerFeeVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SEQUENCERFEEVAULT_ABI.clone(),
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
                SEQUENCERFEEVAULT_ABI.clone(),
                SEQUENCERFEEVAULT_BYTECODE.clone().into(),
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
        ///Calls the contract's `l1FeeWallet` (0xd4ff9218) function
        pub fn l_1_fee_wallet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([212, 255, 146, 24], ())
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
            SequencerFeeVaultEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SequencerFeeVault<M> {
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
    pub enum SequencerFeeVaultEvents {
        Withdrawal1Filter(Withdrawal1Filter),
        Withdrawal2Filter(Withdrawal2Filter),
    }
    impl ::ethers::contract::EthLogDecode for SequencerFeeVaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = Withdrawal1Filter::decode_log(log) {
                return Ok(SequencerFeeVaultEvents::Withdrawal1Filter(decoded));
            }
            if let Ok(decoded) = Withdrawal2Filter::decode_log(log) {
                return Ok(SequencerFeeVaultEvents::Withdrawal2Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SequencerFeeVaultEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Withdrawal1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdrawal2Filter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Withdrawal1Filter> for SequencerFeeVaultEvents {
        fn from(value: Withdrawal1Filter) -> Self {
            Self::Withdrawal1Filter(value)
        }
    }
    impl ::core::convert::From<Withdrawal2Filter> for SequencerFeeVaultEvents {
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
    ///Container type for all input parameters for the `l1FeeWallet` function with signature `l1FeeWallet()` and selector `0xd4ff9218`
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
    #[ethcall(name = "l1FeeWallet", abi = "l1FeeWallet()")]
    pub struct L1FeeWalletCall;
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
    pub enum SequencerFeeVaultCalls {
        MinWithdrawalAmount(MinWithdrawalAmountCall),
        Recipient(RecipientCall),
        WithdrawalNetwork(WithdrawalNetworkCall),
        L1FeeWallet(L1FeeWalletCall),
        TotalProcessed(TotalProcessedCall),
        Version(VersionCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for SequencerFeeVaultCalls {
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
            if let Ok(decoded) = <L1FeeWalletCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1FeeWallet(decoded));
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
    impl ::ethers::core::abi::AbiEncode for SequencerFeeVaultCalls {
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
                Self::L1FeeWallet(element) => {
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
    impl ::core::fmt::Display for SequencerFeeVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinWithdrawalAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Recipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalNetwork(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1FeeWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalProcessed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MinWithdrawalAmountCall> for SequencerFeeVaultCalls {
        fn from(value: MinWithdrawalAmountCall) -> Self {
            Self::MinWithdrawalAmount(value)
        }
    }
    impl ::core::convert::From<RecipientCall> for SequencerFeeVaultCalls {
        fn from(value: RecipientCall) -> Self {
            Self::Recipient(value)
        }
    }
    impl ::core::convert::From<WithdrawalNetworkCall> for SequencerFeeVaultCalls {
        fn from(value: WithdrawalNetworkCall) -> Self {
            Self::WithdrawalNetwork(value)
        }
    }
    impl ::core::convert::From<L1FeeWalletCall> for SequencerFeeVaultCalls {
        fn from(value: L1FeeWalletCall) -> Self {
            Self::L1FeeWallet(value)
        }
    }
    impl ::core::convert::From<TotalProcessedCall> for SequencerFeeVaultCalls {
        fn from(value: TotalProcessedCall) -> Self {
            Self::TotalProcessed(value)
        }
    }
    impl ::core::convert::From<VersionCall> for SequencerFeeVaultCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for SequencerFeeVaultCalls {
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
    ///Container type for all return fields from the `l1FeeWallet` function with signature `l1FeeWallet()` and selector `0xd4ff9218`
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
    pub struct L1FeeWalletReturn(pub ::ethers::core::types::Address);
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

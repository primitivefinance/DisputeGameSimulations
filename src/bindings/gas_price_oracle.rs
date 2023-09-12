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
pub mod gas_price_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DECIMALS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DECIMALS"),
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
                    ::std::borrow::ToOwned::to_owned("baseFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("baseFee"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gasPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gasPrice"),
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
                    ::std::borrow::ToOwned::to_owned("getL1Fee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getL1Fee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getL1GasUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getL1GasUsed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("l1BaseFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1BaseFee"),
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
                    ::std::borrow::ToOwned::to_owned("overhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("overhead"),
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
                    ::std::borrow::ToOwned::to_owned("scalar"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scalar"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GASPRICEORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x01`\x80R`\0`\xA0R`\x02`\xC0R`\x80Q`\xA0Q`\xC0Qa\x0E\xC3a\0\x9C`\09`\0a\x04\xF5\x01R`\0a\x04\xCC\x01R`\0a\x04\xA3\x01Ra\x0E\xC3`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01@W`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0\xF8W\x80c\xDE&\xC4\xA1\x11a\0\xDDW\x80c\xDE&\xC4\xA1\x14a\x02'W\x80c\xF4^e\xD8\x14a\x02:W\x80c\xFE\x17;\x97\x14a\x02!Wa\x01@V[\x80cT\xFDMP\x14a\x02\x0CW\x80cn\xF2\\:\x14a\x02!Wa\x01@V[\x80c1<\xE5g\x11a\x01)W\x80c1<\xE5g\x14a\x01\xEAW\x80cI\x94\x8E\x0E\x14a\x01\xF1W\x80cQ\x9BK\xD3\x14a\x02\x04Wa\x01@V[\x80c\x0C\x18\xC1b\x14a\x01\xC7W\x80c.\x0F&%\x14a\x01\xE2W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xCFa\x02BV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCF`\x06\x81V[`\x06a\x01\xCFV[a\x01\xCFa\x01\xFF6`\x04a\x08\xC9V[a\x03]V[a\x01\xCFa\x03\xBEV[a\x02\x14a\x04\x9CV[`@Qa\x01\xD9\x91\x90a\x0BEV[Ha\x01\xCFV[a\x01\xCFa\x0256`\x04a\x08\xC9V[a\x05?V[a\x01\xCFa\x05\xEEV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B#\x9Fs`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x034W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03X\x91\x90a\x0B\x96V[\x90P\x90V[`\0\x80a\x03i\x83a\x05?V[\x90P`\0a\x03ua\x03\xBEV[a\x03\x7F\x90\x83a\x0B\xE1V[\x90P`\0a\x03\x8F`\x06`\na\r@V[\x90P`\0a\x03\x9Ba\x05\xEEV[a\x03\xA5\x90\x84a\x0B\xE1V[\x90P`\0a\x03\xB3\x83\x83a\r\x82V[\x97\x96PPPPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\xF2Ii`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[``a\x04\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xCCV[a\x04\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xCCV[a\x05\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xCCV[`@Q` \x01a\x05+\x93\x92\x91\x90a\r\x96V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80Q`\0\x90\x81\x90\x81[\x81\x81\x10\x15a\x05\xC2W\x84\x81\x81Q\x81\x10a\x05bWa\x05ba\x0E\x0CV[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x03a\x05\xA2Wa\x05\x9B`\x04\x84a\x0E;V[\x92Pa\x05\xB0V[a\x05\xAD`\x10\x84a\x0E;V[\x92P[\x80a\x05\xBA\x81a\x0ESV[\x91PPa\x05HV[P`\0a\x05\xCDa\x02BV[a\x05\xD7\x90\x84a\x0E;V[\x90Pa\x05\xE5\x81a\x04@a\x0E;V[\x95\x94PPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9E\x8CIf`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[``\x81`\0\x03a\x07\x0FWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x079W\x80a\x07#\x81a\x0ESV[\x91Pa\x072\x90P`\n\x83a\r\x82V[\x91Pa\x07\x13V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07TWa\x07Ta\x08\x8EV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x07~W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x08\x01Wa\x07\x93`\x01\x83a\x0E\x8BV[\x91Pa\x07\xA0`\n\x86a\x0E\xA2V[a\x07\xAB\x90`0a\x0E;V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x07\xC0Wa\x07\xC0a\x0E\x0CV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x07\xFA`\n\x86a\r\x82V[\x94Pa\x07\x82V[\x94\x93PPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0` \x80\x83\x85\x03\x12\x15a\x08\xDFWa\x08\xDFa\x08\tV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tvW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\n\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\n\x1BWa\n\x1Ba\x08\x8EV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\naWa\naa\x08\x8EV[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15a\n\xFAW`@Q\x93P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x85`\x04\x85\x01R`'`$\x85\x01R\x7FABI decoding: invalid byte array`D\x85\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x85\x01R`\x84\x84\xFD[a\x0B\t\x83\x87\x83\x01\x88\x88\x01a\x08\xBDV[\x98\x97PPPPPPPPV[`\0[\x83\x81\x10\x15a\x0B0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\x18V[\x83\x81\x11\x15a\x0B?W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0Bd\x81`@\x85\x01` \x87\x01a\x0B\x15V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0B\xABWa\x0B\xABa\x08\tV[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x0C\x19Wa\x0C\x19a\x0B\xB2V[P\x02\x90V[`\x01\x81\x81[\x80\x85\x11\x15a\x0CwW\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x0C]Wa\x0C]a\x0B\xB2V[\x80\x85\x16\x15a\x0CjW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x0C#V[P\x92P\x92\x90PV[`\0\x82a\x0C\x8EWP`\x01a\r:V[\x81a\x0C\x9BWP`\0a\r:V[\x81`\x01\x81\x14a\x0C\xB1W`\x02\x81\x14a\x0C\xBBWa\x0C\xD7V[`\x01\x91PPa\r:V[`\xFF\x84\x11\x15a\x0C\xCCWa\x0C\xCCa\x0B\xB2V[PP`\x01\x82\x1Ba\r:V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0C\xFAWP\x81\x81\na\r:V[a\r\x04\x83\x83a\x0C\x1EV[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\r6Wa\r6a\x0B\xB2V[\x02\x90P[\x92\x91PPV[`\0a\rL\x83\x83a\x0C\x7FV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\r\x91Wa\r\x91a\rSV[P\x04\x90V[`\0\x84Qa\r\xA8\x81\x84` \x89\x01a\x0B\x15V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\r\xE4\x81`\x01\x85\x01` \x8A\x01a\x0B\x15V[`\x01\x92\x01\x91\x82\x01R\x83Qa\r\xFF\x81`\x02\x84\x01` \x88\x01a\x0B\x15V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x0ENWa\x0ENa\x0B\xB2V[P\x01\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\x84Wa\x0E\x84a\x0B\xB2V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x0E\x9DWa\x0E\x9Da\x0B\xB2V[P\x03\x90V[`\0\x82a\x0E\xB1Wa\x0E\xB1a\rSV[P\x06\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static GASPRICEORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01@W`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0\xF8W\x80c\xDE&\xC4\xA1\x11a\0\xDDW\x80c\xDE&\xC4\xA1\x14a\x02'W\x80c\xF4^e\xD8\x14a\x02:W\x80c\xFE\x17;\x97\x14a\x02!Wa\x01@V[\x80cT\xFDMP\x14a\x02\x0CW\x80cn\xF2\\:\x14a\x02!Wa\x01@V[\x80c1<\xE5g\x11a\x01)W\x80c1<\xE5g\x14a\x01\xEAW\x80cI\x94\x8E\x0E\x14a\x01\xF1W\x80cQ\x9BK\xD3\x14a\x02\x04Wa\x01@V[\x80c\x0C\x18\xC1b\x14a\x01\xC7W\x80c.\x0F&%\x14a\x01\xE2W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xCFa\x02BV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCF`\x06\x81V[`\x06a\x01\xCFV[a\x01\xCFa\x01\xFF6`\x04a\x08\xC9V[a\x03]V[a\x01\xCFa\x03\xBEV[a\x02\x14a\x04\x9CV[`@Qa\x01\xD9\x91\x90a\x0BEV[Ha\x01\xCFV[a\x01\xCFa\x0256`\x04a\x08\xC9V[a\x05?V[a\x01\xCFa\x05\xEEV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B#\x9Fs`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x034W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03X\x91\x90a\x0B\x96V[\x90P\x90V[`\0\x80a\x03i\x83a\x05?V[\x90P`\0a\x03ua\x03\xBEV[a\x03\x7F\x90\x83a\x0B\xE1V[\x90P`\0a\x03\x8F`\x06`\na\r@V[\x90P`\0a\x03\x9Ba\x05\xEEV[a\x03\xA5\x90\x84a\x0B\xE1V[\x90P`\0a\x03\xB3\x83\x83a\r\x82V[\x97\x96PPPPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\xF2Ii`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[``a\x04\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xCCV[a\x04\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xCCV[a\x05\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xCCV[`@Q` \x01a\x05+\x93\x92\x91\x90a\r\x96V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80Q`\0\x90\x81\x90\x81[\x81\x81\x10\x15a\x05\xC2W\x84\x81\x81Q\x81\x10a\x05bWa\x05ba\x0E\x0CV[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x03a\x05\xA2Wa\x05\x9B`\x04\x84a\x0E;V[\x92Pa\x05\xB0V[a\x05\xAD`\x10\x84a\x0E;V[\x92P[\x80a\x05\xBA\x81a\x0ESV[\x91PPa\x05HV[P`\0a\x05\xCDa\x02BV[a\x05\xD7\x90\x84a\x0E;V[\x90Pa\x05\xE5\x81a\x04@a\x0E;V[\x95\x94PPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9E\x8CIf`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03 W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[``\x81`\0\x03a\x07\x0FWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x079W\x80a\x07#\x81a\x0ESV[\x91Pa\x072\x90P`\n\x83a\r\x82V[\x91Pa\x07\x13V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07TWa\x07Ta\x08\x8EV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x07~W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x08\x01Wa\x07\x93`\x01\x83a\x0E\x8BV[\x91Pa\x07\xA0`\n\x86a\x0E\xA2V[a\x07\xAB\x90`0a\x0E;V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x07\xC0Wa\x07\xC0a\x0E\x0CV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x07\xFA`\n\x86a\r\x82V[\x94Pa\x07\x82V[\x94\x93PPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0` \x80\x83\x85\x03\x12\x15a\x08\xDFWa\x08\xDFa\x08\tV[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tvW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\n\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\n\x1BWa\n\x1Ba\x08\x8EV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\naWa\naa\x08\x8EV[\x81`@R\x82\x81R\x88\x86\x84\x87\x01\x01\x11\x15a\n\xFAW`@Q\x93P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x85`\x04\x85\x01R`'`$\x85\x01R\x7FABI decoding: invalid byte array`D\x85\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x85\x01R`\x84\x84\xFD[a\x0B\t\x83\x87\x83\x01\x88\x88\x01a\x08\xBDV[\x98\x97PPPPPPPPV[`\0[\x83\x81\x10\x15a\x0B0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\x18V[\x83\x81\x11\x15a\x0B?W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0Bd\x81`@\x85\x01` \x87\x01a\x0B\x15V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0B\xABWa\x0B\xABa\x08\tV[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x0C\x19Wa\x0C\x19a\x0B\xB2V[P\x02\x90V[`\x01\x81\x81[\x80\x85\x11\x15a\x0CwW\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x0C]Wa\x0C]a\x0B\xB2V[\x80\x85\x16\x15a\x0CjW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x0C#V[P\x92P\x92\x90PV[`\0\x82a\x0C\x8EWP`\x01a\r:V[\x81a\x0C\x9BWP`\0a\r:V[\x81`\x01\x81\x14a\x0C\xB1W`\x02\x81\x14a\x0C\xBBWa\x0C\xD7V[`\x01\x91PPa\r:V[`\xFF\x84\x11\x15a\x0C\xCCWa\x0C\xCCa\x0B\xB2V[PP`\x01\x82\x1Ba\r:V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0C\xFAWP\x81\x81\na\r:V[a\r\x04\x83\x83a\x0C\x1EV[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\r6Wa\r6a\x0B\xB2V[\x02\x90P[\x92\x91PPV[`\0a\rL\x83\x83a\x0C\x7FV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\r\x91Wa\r\x91a\rSV[P\x04\x90V[`\0\x84Qa\r\xA8\x81\x84` \x89\x01a\x0B\x15V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\r\xE4\x81`\x01\x85\x01` \x8A\x01a\x0B\x15V[`\x01\x92\x01\x91\x82\x01R\x83Qa\r\xFF\x81`\x02\x84\x01` \x88\x01a\x0B\x15V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x0ENWa\x0ENa\x0B\xB2V[P\x01\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\x84Wa\x0E\x84a\x0B\xB2V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x0E\x9DWa\x0E\x9Da\x0B\xB2V[P\x03\x90V[`\0\x82a\x0E\xB1Wa\x0E\xB1a\rSV[P\x06\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static GASPRICEORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GasPriceOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GasPriceOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GasPriceOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GasPriceOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GasPriceOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GasPriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GasPriceOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GASPRICEORACLE_ABI.clone(),
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
                GASPRICEORACLE_ABI.clone(),
                GASPRICEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DECIMALS` (0x2e0f2625) function
        pub fn DECIMALS(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 15, 38, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `baseFee` (0x6ef25c3a) function
        pub fn base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 242, 92, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gasPrice` (0xfe173b97) function
        pub fn gas_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 23, 59, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getL1Fee` (0x49948e0e) function
        pub fn get_l1_fee(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([73, 148, 142, 14], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getL1GasUsed` (0xde26c4a1) function
        pub fn get_l1_gas_used(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([222, 38, 196, 161], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1BaseFee` (0x519b4bd3) function
        pub fn l_1_base_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 155, 75, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `overhead` (0x0c18c162) function
        pub fn overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 24, 193, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `scalar` (0xf45e65d8) function
        pub fn scalar(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 94, 101, 216], ())
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
    for GasPriceOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `DECIMALS` function with signature `DECIMALS()` and selector `0x2e0f2625`
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
    #[ethcall(name = "DECIMALS", abi = "DECIMALS()")]
    pub struct DECIMALSCall;
    ///Container type for all input parameters for the `baseFee` function with signature `baseFee()` and selector `0x6ef25c3a`
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
    #[ethcall(name = "baseFee", abi = "baseFee()")]
    pub struct BaseFeeCall;
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct decimalsCall;
    ///Container type for all input parameters for the `gasPrice` function with signature `gasPrice()` and selector `0xfe173b97`
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
    #[ethcall(name = "gasPrice", abi = "gasPrice()")]
    pub struct GasPriceCall;
    ///Container type for all input parameters for the `getL1Fee` function with signature `getL1Fee(bytes)` and selector `0x49948e0e`
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
    #[ethcall(name = "getL1Fee", abi = "getL1Fee(bytes)")]
    pub struct GetL1FeeCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getL1GasUsed` function with signature `getL1GasUsed(bytes)` and selector `0xde26c4a1`
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
    #[ethcall(name = "getL1GasUsed", abi = "getL1GasUsed(bytes)")]
    pub struct GetL1GasUsedCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `l1BaseFee` function with signature `l1BaseFee()` and selector `0x519b4bd3`
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
    #[ethcall(name = "l1BaseFee", abi = "l1BaseFee()")]
    pub struct L1BaseFeeCall;
    ///Container type for all input parameters for the `overhead` function with signature `overhead()` and selector `0x0c18c162`
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
    #[ethcall(name = "overhead", abi = "overhead()")]
    pub struct OverheadCall;
    ///Container type for all input parameters for the `scalar` function with signature `scalar()` and selector `0xf45e65d8`
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
    #[ethcall(name = "scalar", abi = "scalar()")]
    pub struct ScalarCall;
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
    pub enum GasPriceOracleCalls {
        DECIMALS(DECIMALSCall),
        BaseFee(BaseFeeCall),
        decimals(decimalsCall),
        GasPrice(GasPriceCall),
        GetL1Fee(GetL1FeeCall),
        GetL1GasUsed(GetL1GasUsedCall),
        L1BaseFee(L1BaseFeeCall),
        Overhead(OverheadCall),
        Scalar(ScalarCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for GasPriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DECIMALSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DECIMALS(decoded));
            }
            if let Ok(decoded) = <BaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseFee(decoded));
            }
            if let Ok(decoded) = <decimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::decimals(decoded));
            }
            if let Ok(decoded) = <GasPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GasPrice(decoded));
            }
            if let Ok(decoded) = <GetL1FeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetL1Fee(decoded));
            }
            if let Ok(decoded) = <GetL1GasUsedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetL1GasUsed(decoded));
            }
            if let Ok(decoded) = <L1BaseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1BaseFee(decoded));
            }
            if let Ok(decoded) = <OverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Overhead(decoded));
            }
            if let Ok(decoded) = <ScalarCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Scalar(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GasPriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DECIMALS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GasPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetL1Fee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetL1GasUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1BaseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Overhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Scalar(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GasPriceOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DECIMALS(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetL1Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetL1GasUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1BaseFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Overhead(element) => ::core::fmt::Display::fmt(element, f),
                Self::Scalar(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DECIMALSCall> for GasPriceOracleCalls {
        fn from(value: DECIMALSCall) -> Self {
            Self::DECIMALS(value)
        }
    }
    impl ::core::convert::From<BaseFeeCall> for GasPriceOracleCalls {
        fn from(value: BaseFeeCall) -> Self {
            Self::BaseFee(value)
        }
    }
    impl ::core::convert::From<decimalsCall> for GasPriceOracleCalls {
        fn from(value: decimalsCall) -> Self {
            Self::decimals(value)
        }
    }
    impl ::core::convert::From<GasPriceCall> for GasPriceOracleCalls {
        fn from(value: GasPriceCall) -> Self {
            Self::GasPrice(value)
        }
    }
    impl ::core::convert::From<GetL1FeeCall> for GasPriceOracleCalls {
        fn from(value: GetL1FeeCall) -> Self {
            Self::GetL1Fee(value)
        }
    }
    impl ::core::convert::From<GetL1GasUsedCall> for GasPriceOracleCalls {
        fn from(value: GetL1GasUsedCall) -> Self {
            Self::GetL1GasUsed(value)
        }
    }
    impl ::core::convert::From<L1BaseFeeCall> for GasPriceOracleCalls {
        fn from(value: L1BaseFeeCall) -> Self {
            Self::L1BaseFee(value)
        }
    }
    impl ::core::convert::From<OverheadCall> for GasPriceOracleCalls {
        fn from(value: OverheadCall) -> Self {
            Self::Overhead(value)
        }
    }
    impl ::core::convert::From<ScalarCall> for GasPriceOracleCalls {
        fn from(value: ScalarCall) -> Self {
            Self::Scalar(value)
        }
    }
    impl ::core::convert::From<VersionCall> for GasPriceOracleCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `DECIMALS` function with signature `DECIMALS()` and selector `0x2e0f2625`
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
    pub struct DECIMALSReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `baseFee` function with signature `baseFee()` and selector `0x6ef25c3a`
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
    pub struct BaseFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct decimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gasPrice` function with signature `gasPrice()` and selector `0xfe173b97`
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
    pub struct GasPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getL1Fee` function with signature `getL1Fee(bytes)` and selector `0x49948e0e`
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
    pub struct GetL1FeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getL1GasUsed` function with signature `getL1GasUsed(bytes)` and selector `0xde26c4a1`
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
    pub struct GetL1GasUsedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l1BaseFee` function with signature `l1BaseFee()` and selector `0x519b4bd3`
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
    pub struct L1BaseFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `overhead` function with signature `overhead()` and selector `0x0c18c162`
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
    pub struct OverheadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `scalar` function with signature `scalar()` and selector `0xf45e65d8`
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
    pub struct ScalarReturn(pub ::ethers::core::types::U256);
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

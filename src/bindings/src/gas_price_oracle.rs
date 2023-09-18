pub use gas_price_oracle::*;
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
            constructor: ::core::option::Option::None,
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\xC0\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0vW\x80c\xDE&\xC4\xA1\x11a\0[W\x80c\xDE&\xC4\xA1\x14a\x01WW\x80c\xF4^e\xD8\x14a\x01jW\x80c\xFE\x17;\x97\x14a\x01QW`\0\x80\xFD[\x80cT\xFDMP\x14a\x01\x08W\x80cn\xF2\\:\x14a\x01QW`\0\x80\xFD[\x80c1<\xE5g\x11a\0\xA7W\x80c1<\xE5g\x14a\0\xE6W\x80cI\x94\x8E\x0E\x14a\0\xEDW\x80cQ\x9BK\xD3\x14a\x01\0W`\0\x80\xFD[\x80c\x0C\x18\xC1b\x14a\0\xC3W\x80c.\x0F&%\x14a\0\xDEW[`\0\x80\xFD[a\0\xCBa\x01rV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCB`\x06\x81V[`\x06a\0\xCBV[a\0\xCBa\0\xFB6`\x04a\x03\xFDV[a\x01\xFCV[a\0\xCBa\x02]V[a\x01D`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.1.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xD5\x91\x90a\x04\xCCV[Ha\0\xCBV[a\0\xCBa\x01e6`\x04a\x03\xFDV[a\x02\xBEV[a\0\xCBa\x03mV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B#\x9Fs`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF7\x91\x90a\x05?V[\x90P\x90V[`\0\x80a\x02\x08\x83a\x02\xBEV[\x90P`\0a\x02\x14a\x02]V[a\x02\x1E\x90\x83a\x05\x87V[\x90P`\0a\x02.`\x06`\na\x06\xE6V[\x90P`\0a\x02:a\x03mV[a\x02D\x90\x84a\x05\x87V[\x90P`\0a\x02R\x83\x83a\x06\xF9V[\x97\x96PPPPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\xF2Ii`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD3W=`\0\x80>=`\0\xFD[\x80Q`\0\x90\x81\x90\x81[\x81\x81\x10\x15a\x03AW\x84\x81\x81Q\x81\x10a\x02\xE1Wa\x02\xE1a\x074V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x03a\x03!Wa\x03\x1A`\x04\x84a\x07cV[\x92Pa\x03/V[a\x03,`\x10\x84a\x07cV[\x92P[\x80a\x039\x81a\x07{V[\x91PPa\x02\xC7V[P`\0a\x03La\x01rV[a\x03V\x90\x84a\x07cV[\x90Pa\x03d\x81a\x04@a\x07cV[\x95\x94PPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9E\x8CIf`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD3W=`\0\x80>=`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x04\x0FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04'W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x04;W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04MWa\x04Ma\x03\xCEV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\x93Wa\x04\x93a\x03\xCEV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x04\xACW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x04\xF9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\xDDV[\x81\x81\x11\x15a\x05\x0BW`\0`@\x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05QW`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x05\xBFWa\x05\xBFa\x05XV[P\x02\x90V[`\x01\x81\x81[\x80\x85\x11\x15a\x06\x1DW\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x06\x03Wa\x06\x03a\x05XV[\x80\x85\x16\x15a\x06\x10W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x05\xC9V[P\x92P\x92\x90PV[`\0\x82a\x064WP`\x01a\x06\xE0V[\x81a\x06AWP`\0a\x06\xE0V[\x81`\x01\x81\x14a\x06WW`\x02\x81\x14a\x06aWa\x06}V[`\x01\x91PPa\x06\xE0V[`\xFF\x84\x11\x15a\x06rWa\x06ra\x05XV[PP`\x01\x82\x1Ba\x06\xE0V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x06\xA0WP\x81\x81\na\x06\xE0V[a\x06\xAA\x83\x83a\x05\xC4V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x06\xDCWa\x06\xDCa\x05XV[\x02\x90P[\x92\x91PPV[`\0a\x06\xF2\x83\x83a\x06%V[\x93\x92PPPV[`\0\x82a\x07/W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x07vWa\x07va\x05XV[P\x01\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x07\xACWa\x07\xACa\x05XV[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static GASPRICEORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0vW\x80c\xDE&\xC4\xA1\x11a\0[W\x80c\xDE&\xC4\xA1\x14a\x01WW\x80c\xF4^e\xD8\x14a\x01jW\x80c\xFE\x17;\x97\x14a\x01QW`\0\x80\xFD[\x80cT\xFDMP\x14a\x01\x08W\x80cn\xF2\\:\x14a\x01QW`\0\x80\xFD[\x80c1<\xE5g\x11a\0\xA7W\x80c1<\xE5g\x14a\0\xE6W\x80cI\x94\x8E\x0E\x14a\0\xEDW\x80cQ\x9BK\xD3\x14a\x01\0W`\0\x80\xFD[\x80c\x0C\x18\xC1b\x14a\0\xC3W\x80c.\x0F&%\x14a\0\xDEW[`\0\x80\xFD[a\0\xCBa\x01rV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCB`\x06\x81V[`\x06a\0\xCBV[a\0\xCBa\0\xFB6`\x04a\x03\xFDV[a\x01\xFCV[a\0\xCBa\x02]V[a\x01D`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.1.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\xD5\x91\x90a\x04\xCCV[Ha\0\xCBV[a\0\xCBa\x01e6`\x04a\x03\xFDV[a\x02\xBEV[a\0\xCBa\x03mV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8B#\x9Fs`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF7\x91\x90a\x05?V[\x90P\x90V[`\0\x80a\x02\x08\x83a\x02\xBEV[\x90P`\0a\x02\x14a\x02]V[a\x02\x1E\x90\x83a\x05\x87V[\x90P`\0a\x02.`\x06`\na\x06\xE6V[\x90P`\0a\x02:a\x03mV[a\x02D\x90\x84a\x05\x87V[\x90P`\0a\x02R\x83\x83a\x06\xF9V[\x97\x96PPPPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\\xF2Ii`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD3W=`\0\x80>=`\0\xFD[\x80Q`\0\x90\x81\x90\x81[\x81\x81\x10\x15a\x03AW\x84\x81\x81Q\x81\x10a\x02\xE1Wa\x02\xE1a\x074V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\0\x03a\x03!Wa\x03\x1A`\x04\x84a\x07cV[\x92Pa\x03/V[a\x03,`\x10\x84a\x07cV[\x92P[\x80a\x039\x81a\x07{V[\x91PPa\x02\xC7V[P`\0a\x03La\x01rV[a\x03V\x90\x84a\x07cV[\x90Pa\x03d\x81a\x04@a\x07cV[\x95\x94PPPPPV[`\0sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9E\x8CIf`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD3W=`\0\x80>=`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x04\x0FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04'W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x04;W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04MWa\x04Ma\x03\xCEV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\x93Wa\x04\x93a\x03\xCEV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x04\xACW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x04\xF9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x04\xDDV[\x81\x81\x11\x15a\x05\x0BW`\0`@\x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05QW`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x05\xBFWa\x05\xBFa\x05XV[P\x02\x90V[`\x01\x81\x81[\x80\x85\x11\x15a\x06\x1DW\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x06\x03Wa\x06\x03a\x05XV[\x80\x85\x16\x15a\x06\x10W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x05\xC9V[P\x92P\x92\x90PV[`\0\x82a\x064WP`\x01a\x06\xE0V[\x81a\x06AWP`\0a\x06\xE0V[\x81`\x01\x81\x14a\x06WW`\x02\x81\x14a\x06aWa\x06}V[`\x01\x91PPa\x06\xE0V[`\xFF\x84\x11\x15a\x06rWa\x06ra\x05XV[PP`\x01\x82\x1Ba\x06\xE0V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x06\xA0WP\x81\x81\na\x06\xE0V[a\x06\xAA\x83\x83a\x05\xC4V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11\x15a\x06\xDCWa\x06\xDCa\x05XV[\x02\x90P[\x92\x91PPV[`\0a\x06\xF2\x83\x83a\x06%V[\x93\x92PPPV[`\0\x82a\x07/W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x07vWa\x07va\x05XV[P\x01\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x07\xACWa\x07\xACa\x05XV[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
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

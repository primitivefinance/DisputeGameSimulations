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
pub mod example_clone {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_argOffset"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addressArg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressArg"),
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
                    ::std::borrow::ToOwned::to_owned("dynBytesArg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dynBytesArg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arrLen"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("fixedBytesArg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fixedBytesArg"),
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
                    ::std::borrow::ToOwned::to_owned("uint64Arg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uint64Arg"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("uint8Arg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uint8Arg"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uintArg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uintArg"),
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
                    ::std::borrow::ToOwned::to_owned("uintArrayArg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uintArrayArg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arrLen"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
    pub static EXAMPLECLONE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x06\xEB8\x03\x80a\x06\xEB\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\x84V[`\0Ua\0\xE8V[`\0` \x82\x84\x03\x12\x15a\0\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[a\x05\xF4\x80a\0\xF7`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xFFW`\x005`\xE0\x1C\x80c\xA6\xC9\xDC\x06\x11a\0\xDDW\x80c\xA6\xC9\xDC\x06\x14a\x02JW\x80c\xAF\xAE\x0Cv\x14a\x02jW\x80c\xBD\x12u\xDD\x14a\x02\x8AW\x80c\xBE\xA9O>\x14a\x01\xCCWa\0\xFFV[\x80c\x01\x1C\xD7C\x14a\x01\x86W\x80c\x7F\xB3\x1A\x14\x14a\x01\xCCW\x80c\xA5\xCA\xAFZ\x14a\x02\tW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\0T`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x90\x91\x015`\xF8\x1C\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x015`@Q\x90\x81R` \x01a\x01\xC3V[`\0T`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x90\x91\x015`\xC0\x1C\x81R` \x01a\x01\xC3V[a\x02]a\x02X6`\x04a\x04\x1DV[a\x02\xCBV[`@Qa\x01\xC3\x91\x90a\x04\xCEV[a\x02}a\x02x6`\x04a\x04\x1DV[a\x02\xDFV[`@Qa\x01\xC3\x91\x90a\x05\x12V[`\0T`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x90\x91\x015``\x1C\x81R` \x01a\x01\xC3V[``a\x02\xD9`\0T\x83a\x02\xEDV[\x92\x91PPV[``a\x02\xD9`\0T\x83a\x03\x86V[```\0a\x03$\x846\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03a\x05~V[\x90P\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03IWa\x03Ia\x05\xB8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03rW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`\x05\x1B\x81` \x84\x017P\x92\x91PPV[```\0a\x03\xBD\x846\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03a\x05~V[\x90P\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xE2Wa\x03\xE2a\x05\xB8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x04\x0CW` \x82\x01\x81\x806\x837\x01\x90P[P\x91P\x82\x81` \x84\x017P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\xAFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xC7W`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x05\x06W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x04\xEAV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x05?W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x05#V[P`\0`@\x82\x86\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xD9W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The bytecode of the contract.
    pub static EXAMPLECLONE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xFFW`\x005`\xE0\x1C\x80c\xA6\xC9\xDC\x06\x11a\0\xDDW\x80c\xA6\xC9\xDC\x06\x14a\x02JW\x80c\xAF\xAE\x0Cv\x14a\x02jW\x80c\xBD\x12u\xDD\x14a\x02\x8AW\x80c\xBE\xA9O>\x14a\x01\xCCWa\0\xFFV[\x80c\x01\x1C\xD7C\x14a\x01\x86W\x80c\x7F\xB3\x1A\x14\x14a\x01\xCCW\x80c\xA5\xCA\xAFZ\x14a\x02\tW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\0T`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x90\x91\x015`\xF8\x1C\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x015`@Q\x90\x81R` \x01a\x01\xC3V[`\0T`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x90\x91\x015`\xC0\x1C\x81R` \x01a\x01\xC3V[a\x02]a\x02X6`\x04a\x04\x1DV[a\x02\xCBV[`@Qa\x01\xC3\x91\x90a\x04\xCEV[a\x02}a\x02x6`\x04a\x04\x1DV[a\x02\xDFV[`@Qa\x01\xC3\x91\x90a\x05\x12V[`\0T`@Q6\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03\x90\x91\x015``\x1C\x81R` \x01a\x01\xC3V[``a\x02\xD9`\0T\x83a\x02\xEDV[\x92\x91PPV[``a\x02\xD9`\0T\x83a\x03\x86V[```\0a\x03$\x846\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03a\x05~V[\x90P\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03IWa\x03Ia\x05\xB8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03rW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`\x05\x1B\x81` \x84\x017P\x92\x91PPV[```\0a\x03\xBD\x846\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x015`\xF0\x1C\x90\x03a\x05~V[\x90P\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xE2Wa\x03\xE2a\x05\xB8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x04\x0CW` \x82\x01\x81\x806\x837\x01\x90P[P\x91P\x82\x81` \x84\x017P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x04\xAFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xC7W`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x05\x06W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x04\xEAV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x05?W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x05#V[P`\0`@\x82\x86\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xD9W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The deployed bytecode of the contract.
    pub static EXAMPLECLONE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ExampleClone<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExampleClone<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExampleClone<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExampleClone<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExampleClone<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExampleClone))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExampleClone<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXAMPLECLONE_ABI.clone(),
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
                EXAMPLECLONE_ABI.clone(),
                EXAMPLECLONE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addressArg` (0xbd1275dd) function
        pub fn address_arg(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([189, 18, 117, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dynBytesArg` (0xafae0c76) function
        pub fn dyn_bytes_arg(
            &self,
            arr_len: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([175, 174, 12, 118], arr_len)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fixedBytesArg` (0x7fb31a14) function
        pub fn fixed_bytes_arg(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([127, 179, 26, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uint64Arg` (0xa5caaf5a) function
        pub fn uint_64_arg(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([165, 202, 175, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uint8Arg` (0x011cd743) function
        pub fn uint_8_arg(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([1, 28, 215, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uintArg` (0xbea94f3e) function
        pub fn uint_arg(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([190, 169, 79, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uintArrayArg` (0xa6c9dc06) function
        pub fn uint_array_arg(
            &self,
            arr_len: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([166, 201, 220, 6], arr_len)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExampleClone<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addressArg` function with signature `addressArg()` and selector `0xbd1275dd`
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
    #[ethcall(name = "addressArg", abi = "addressArg()")]
    pub struct AddressArgCall;
    ///Container type for all input parameters for the `dynBytesArg` function with signature `dynBytesArg(uint64)` and selector `0xafae0c76`
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
    #[ethcall(name = "dynBytesArg", abi = "dynBytesArg(uint64)")]
    pub struct DynBytesArgCall {
        pub arr_len: u64,
    }
    ///Container type for all input parameters for the `fixedBytesArg` function with signature `fixedBytesArg()` and selector `0x7fb31a14`
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
    #[ethcall(name = "fixedBytesArg", abi = "fixedBytesArg()")]
    pub struct FixedBytesArgCall;
    ///Container type for all input parameters for the `uint64Arg` function with signature `uint64Arg()` and selector `0xa5caaf5a`
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
    #[ethcall(name = "uint64Arg", abi = "uint64Arg()")]
    pub struct Uint64ArgCall;
    ///Container type for all input parameters for the `uint8Arg` function with signature `uint8Arg()` and selector `0x011cd743`
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
    #[ethcall(name = "uint8Arg", abi = "uint8Arg()")]
    pub struct Uint8ArgCall;
    ///Container type for all input parameters for the `uintArg` function with signature `uintArg()` and selector `0xbea94f3e`
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
    #[ethcall(name = "uintArg", abi = "uintArg()")]
    pub struct UintArgCall;
    ///Container type for all input parameters for the `uintArrayArg` function with signature `uintArrayArg(uint64)` and selector `0xa6c9dc06`
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
    #[ethcall(name = "uintArrayArg", abi = "uintArrayArg(uint64)")]
    pub struct UintArrayArgCall {
        pub arr_len: u64,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExampleCloneCalls {
        AddressArg(AddressArgCall),
        DynBytesArg(DynBytesArgCall),
        FixedBytesArg(FixedBytesArgCall),
        Uint64Arg(Uint64ArgCall),
        Uint8Arg(Uint8ArgCall),
        UintArg(UintArgCall),
        UintArrayArg(UintArrayArgCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleCloneCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddressArgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressArg(decoded));
            }
            if let Ok(decoded) = <DynBytesArgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DynBytesArg(decoded));
            }
            if let Ok(decoded) = <FixedBytesArgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FixedBytesArg(decoded));
            }
            if let Ok(decoded) = <Uint64ArgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Uint64Arg(decoded));
            }
            if let Ok(decoded) = <Uint8ArgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Uint8Arg(decoded));
            }
            if let Ok(decoded) = <UintArgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UintArg(decoded));
            }
            if let Ok(decoded) = <UintArrayArgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UintArrayArg(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleCloneCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressArg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DynBytesArg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FixedBytesArg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Uint64Arg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Uint8Arg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UintArg(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UintArrayArg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExampleCloneCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressArg(element) => ::core::fmt::Display::fmt(element, f),
                Self::DynBytesArg(element) => ::core::fmt::Display::fmt(element, f),
                Self::FixedBytesArg(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uint64Arg(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uint8Arg(element) => ::core::fmt::Display::fmt(element, f),
                Self::UintArg(element) => ::core::fmt::Display::fmt(element, f),
                Self::UintArrayArg(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressArgCall> for ExampleCloneCalls {
        fn from(value: AddressArgCall) -> Self {
            Self::AddressArg(value)
        }
    }
    impl ::core::convert::From<DynBytesArgCall> for ExampleCloneCalls {
        fn from(value: DynBytesArgCall) -> Self {
            Self::DynBytesArg(value)
        }
    }
    impl ::core::convert::From<FixedBytesArgCall> for ExampleCloneCalls {
        fn from(value: FixedBytesArgCall) -> Self {
            Self::FixedBytesArg(value)
        }
    }
    impl ::core::convert::From<Uint64ArgCall> for ExampleCloneCalls {
        fn from(value: Uint64ArgCall) -> Self {
            Self::Uint64Arg(value)
        }
    }
    impl ::core::convert::From<Uint8ArgCall> for ExampleCloneCalls {
        fn from(value: Uint8ArgCall) -> Self {
            Self::Uint8Arg(value)
        }
    }
    impl ::core::convert::From<UintArgCall> for ExampleCloneCalls {
        fn from(value: UintArgCall) -> Self {
            Self::UintArg(value)
        }
    }
    impl ::core::convert::From<UintArrayArgCall> for ExampleCloneCalls {
        fn from(value: UintArrayArgCall) -> Self {
            Self::UintArrayArg(value)
        }
    }
    ///Container type for all return fields from the `addressArg` function with signature `addressArg()` and selector `0xbd1275dd`
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
    pub struct AddressArgReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `dynBytesArg` function with signature `dynBytesArg(uint64)` and selector `0xafae0c76`
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
    pub struct DynBytesArgReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `fixedBytesArg` function with signature `fixedBytesArg()` and selector `0x7fb31a14`
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
    pub struct FixedBytesArgReturn(pub [u8; 32]);
    ///Container type for all return fields from the `uint64Arg` function with signature `uint64Arg()` and selector `0xa5caaf5a`
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
    pub struct Uint64ArgReturn(pub u64);
    ///Container type for all return fields from the `uint8Arg` function with signature `uint8Arg()` and selector `0x011cd743`
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
    pub struct Uint8ArgReturn(pub u8);
    ///Container type for all return fields from the `uintArg` function with signature `uintArg()` and selector `0xbea94f3e`
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
    pub struct UintArgReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `uintArrayArg` function with signature `uintArrayArg(uint64)` and selector `0xa6c9dc06`
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
    pub struct UintArrayArgReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
}

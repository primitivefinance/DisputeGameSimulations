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
pub mod hash_cross_domain_hasher {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("failedCrossDomainHashHighVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedCrossDomainHashHighVersion",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("failedCrossDomainHashV0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedCrossDomainHashV0",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("failedCrossDomainHashV1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedCrossDomainHashV1",
                            ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "hashCrossDomainMessageHighVersion",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hashCrossDomainMessageHighVersion",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        240usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint240"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
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
                    ::std::borrow::ToOwned::to_owned("hashCrossDomainMessageV0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hashCrossDomainMessageV0",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        240usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint240"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
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
                    ::std::borrow::ToOwned::to_owned("hashCrossDomainMessageV1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hashCrossDomainMessageV1",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        240usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint240"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static HASH_CROSSDOMAINHASHER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x0B\r\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF4W`\x005`\xE0\x1C\x80c\xA2\xC4\xA7\xDC\x11a\0\xD2W\x80c\xA2\xC4\xA7\xDC\x14a\x01\xB6W\x80c\xB3/\xCF\x9A\x14a\x01\xD7W\x80c\xE0\x13b4\x14a\x01\xEAWa\0\xF4V[\x80c\x08\xC5\xBBa\x14a\x01{W\x80c@\xE1^\x04\x14a\x01\x90W\x80c\x93\x1D\xAD\x1A\x14a\x01\xA3W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\x8Ea\x01\x896`\x04a\x08\xCBV[a\x01\xFCV[\0[a\x01\x8Ea\x01\x9E6`\x04a\tmV[a\x02^V[a\x01\x8Ea\x01\xB16`\x04a\tmV[a\x02\xC0V[`\0Ta\x01\xC3\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\x01\xC3\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`\0Ta\x01\xC3\x90a\x01\0\x90\x04`\xFF\x16\x81V[`\0a\x02\n\x87\x89`\xF0\x1B\x17\x90V[\x90Pa\x02\x1A\x81\x87\x87\x87\x87\x87a\x03EV[P`\x01\x88a\xFF\xFF\x16\x11\x15a\x02TW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[PPPPPPPPV[\x85`\0a\x02o\x82\x88\x88\x88\x88\x88a\x03EV[\x90P`\0a\x02\x7F\x87\x89\x86\x86a\x04\x17V[\x90P\x80\x82\x14a\x02\xB5W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[PPPPPPPPPV[~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x17`\0a\x02\xF2\x82\x88\x88\x88\x88\x88a\x03EV[\x90P`\0a\x03\x04\x83\x89\x89\x89\x89\x89a\x046V[\x90P\x80\x82\x14a\x02\xB5W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\x16b\x01\0\0\x17\x90UPPPPPPPPPV[`\0`\xF0\x87\x90\x1C\x80\x82\x03a\x03gWa\x03_\x86\x88\x85\x8Ba\x04\x17V[\x91PPa\x04\rV[\x80a\xFF\xFF\x16`\x01\x03a\x03\x81Wa\x03_\x88\x88\x88\x88\x88\x88a\x046V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FHashing: unknown cross domain me`D\x82\x01R\x7Fssage version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x96\x95PPPPPPV[`\0a\x04%\x85\x85\x85\x85a\x04YV[\x80Q\x90` \x01 \x90P\x94\x93PPPPV[`\0a\x04F\x87\x87\x87\x87\x87\x87a\x04\xF2V[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[``\x84\x84\x84\x84`@Q`$\x01a\x04r\x94\x93\x92\x91\x90a\n_V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xD4\xEC\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x94\x93PPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a\x05\x0F\x96\x95\x94\x93\x92\x91\x90a\n\xA9V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xC9W`\0\x80\xFD[\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xC9W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x07\xBEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xD9Wa\x07\xD9a\x06\xF2V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x08\x1FWa\x08\x1Fa\x06\xF2V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x08\xBAW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[a\x04\r\x84` \x83\x01` \x89\x01a\x07!V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x08\xE9Wa\x08\xE9a\x05\x91V[\x875a\xFF\xFF\x81\x16\x81\x14a\x08\xFBW`\0\x80\xFD[\x96Pa\t\t` \x89\x01a\x06\x9BV[\x95Pa\t\x17`@\x89\x01a\x06\xCEV[\x94Pa\t%``\x89\x01a\x06\xCEV[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tRWa\tRa\x06\x16V[a\t^\x8A\x82\x8B\x01a\x07-V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\t\x89Wa\t\x89a\x05\x91V[a\t\x92\x87a\x06\x9BV[\x95Pa\t\xA0` \x88\x01a\x06\xCEV[\x94Pa\t\xAE`@\x88\x01a\x06\xCEV[\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDBWa\t\xDBa\x06\x16V[a\t\xE7\x89\x82\x8A\x01a\x07-V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\n\x1AW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\t\xFEV[\x81\x81\x11\x15a\n,W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra\n\x98`\x80\x83\x01\x85a\t\xF4V[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\n\xF4`\xC0\x83\x01\x84a\t\xF4V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static HASH_CROSSDOMAINHASHER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF4W`\x005`\xE0\x1C\x80c\xA2\xC4\xA7\xDC\x11a\0\xD2W\x80c\xA2\xC4\xA7\xDC\x14a\x01\xB6W\x80c\xB3/\xCF\x9A\x14a\x01\xD7W\x80c\xE0\x13b4\x14a\x01\xEAWa\0\xF4V[\x80c\x08\xC5\xBBa\x14a\x01{W\x80c@\xE1^\x04\x14a\x01\x90W\x80c\x93\x1D\xAD\x1A\x14a\x01\xA3W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\x8Ea\x01\x896`\x04a\x08\xCBV[a\x01\xFCV[\0[a\x01\x8Ea\x01\x9E6`\x04a\tmV[a\x02^V[a\x01\x8Ea\x01\xB16`\x04a\tmV[a\x02\xC0V[`\0Ta\x01\xC3\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\x01\xC3\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`\0Ta\x01\xC3\x90a\x01\0\x90\x04`\xFF\x16\x81V[`\0a\x02\n\x87\x89`\xF0\x1B\x17\x90V[\x90Pa\x02\x1A\x81\x87\x87\x87\x87\x87a\x03EV[P`\x01\x88a\xFF\xFF\x16\x11\x15a\x02TW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[PPPPPPPPV[\x85`\0a\x02o\x82\x88\x88\x88\x88\x88a\x03EV[\x90P`\0a\x02\x7F\x87\x89\x86\x86a\x04\x17V[\x90P\x80\x82\x14a\x02\xB5W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[PPPPPPPPPV[~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x17`\0a\x02\xF2\x82\x88\x88\x88\x88\x88a\x03EV[\x90P`\0a\x03\x04\x83\x89\x89\x89\x89\x89a\x046V[\x90P\x80\x82\x14a\x02\xB5W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\x16b\x01\0\0\x17\x90UPPPPPPPPPV[`\0`\xF0\x87\x90\x1C\x80\x82\x03a\x03gWa\x03_\x86\x88\x85\x8Ba\x04\x17V[\x91PPa\x04\rV[\x80a\xFF\xFF\x16`\x01\x03a\x03\x81Wa\x03_\x88\x88\x88\x88\x88\x88a\x046V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FHashing: unknown cross domain me`D\x82\x01R\x7Fssage version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x96\x95PPPPPPV[`\0a\x04%\x85\x85\x85\x85a\x04YV[\x80Q\x90` \x01 \x90P\x94\x93PPPPV[`\0a\x04F\x87\x87\x87\x87\x87\x87a\x04\xF2V[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[``\x84\x84\x84\x84`@Q`$\x01a\x04r\x94\x93\x92\x91\x90a\n_V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCB\xD4\xEC\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x94\x93PPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a\x05\x0F\x96\x95\x94\x93\x92\x91\x90a\n\xA9V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xC9W`\0\x80\xFD[\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xC9W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x07\xBEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xD9Wa\x07\xD9a\x06\xF2V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x08\x1FWa\x08\x1Fa\x06\xF2V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x08\xBAW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[a\x04\r\x84` \x83\x01` \x89\x01a\x07!V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x08\xE9Wa\x08\xE9a\x05\x91V[\x875a\xFF\xFF\x81\x16\x81\x14a\x08\xFBW`\0\x80\xFD[\x96Pa\t\t` \x89\x01a\x06\x9BV[\x95Pa\t\x17`@\x89\x01a\x06\xCEV[\x94Pa\t%``\x89\x01a\x06\xCEV[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tRWa\tRa\x06\x16V[a\t^\x8A\x82\x8B\x01a\x07-V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\t\x89Wa\t\x89a\x05\x91V[a\t\x92\x87a\x06\x9BV[\x95Pa\t\xA0` \x88\x01a\x06\xCEV[\x94Pa\t\xAE`@\x88\x01a\x06\xCEV[\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDBWa\t\xDBa\x06\x16V[a\t\xE7\x89\x82\x8A\x01a\x07-V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\n\x1AW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\t\xFEV[\x81\x81\x11\x15a\n,W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra\n\x98`\x80\x83\x01\x85a\t\xF4V[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\n\xF4`\xC0\x83\x01\x84a\t\xF4V[\x98\x97PPPPPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static HASH_CROSSDOMAINHASHER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Hash_CrossDomainHasher<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Hash_CrossDomainHasher<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Hash_CrossDomainHasher<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Hash_CrossDomainHasher<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Hash_CrossDomainHasher<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Hash_CrossDomainHasher))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Hash_CrossDomainHasher<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HASH_CROSSDOMAINHASHER_ABI.clone(),
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
                HASH_CROSSDOMAINHASHER_ABI.clone(),
                HASH_CROSSDOMAINHASHER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `failedCrossDomainHashHighVersion` (0xa2c4a7dc) function
        pub fn failed_cross_domain_hash_high_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([162, 196, 167, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedCrossDomainHashV0` (0xe0136234) function
        pub fn failed_cross_domain_hash_v0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 19, 98, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedCrossDomainHashV1` (0xb32fcf9a) function
        pub fn failed_cross_domain_hash_v1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 47, 207, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashCrossDomainMessageHighVersion` (0x08c5bb61) function
        pub fn hash_cross_domain_message_high_version(
            &self,
            version: u16,
            nonce: ::ethers::core::types::U256,
            sender: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            gas_limit: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [8, 197, 187, 97],
                    (version, nonce, sender, target, value, gas_limit, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashCrossDomainMessageV0` (0x40e15e04) function
        pub fn hash_cross_domain_message_v0(
            &self,
            nonce: ::ethers::core::types::U256,
            sender: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            gas_limit: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [64, 225, 94, 4],
                    (nonce, sender, target, value, gas_limit, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashCrossDomainMessageV1` (0x931dad1a) function
        pub fn hash_cross_domain_message_v1(
            &self,
            nonce: ::ethers::core::types::U256,
            sender: ::ethers::core::types::Address,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            gas_limit: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [147, 29, 173, 26],
                    (nonce, sender, target, value, gas_limit, data),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Hash_CrossDomainHasher<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `failedCrossDomainHashHighVersion` function with signature `failedCrossDomainHashHighVersion()` and selector `0xa2c4a7dc`
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
        name = "failedCrossDomainHashHighVersion",
        abi = "failedCrossDomainHashHighVersion()"
    )]
    pub struct FailedCrossDomainHashHighVersionCall;
    ///Container type for all input parameters for the `failedCrossDomainHashV0` function with signature `failedCrossDomainHashV0()` and selector `0xe0136234`
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
    #[ethcall(name = "failedCrossDomainHashV0", abi = "failedCrossDomainHashV0()")]
    pub struct FailedCrossDomainHashV0Call;
    ///Container type for all input parameters for the `failedCrossDomainHashV1` function with signature `failedCrossDomainHashV1()` and selector `0xb32fcf9a`
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
    #[ethcall(name = "failedCrossDomainHashV1", abi = "failedCrossDomainHashV1()")]
    pub struct FailedCrossDomainHashV1Call;
    ///Container type for all input parameters for the `hashCrossDomainMessageHighVersion` function with signature `hashCrossDomainMessageHighVersion(uint16,uint240,address,address,uint256,uint256,bytes)` and selector `0x08c5bb61`
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
        name = "hashCrossDomainMessageHighVersion",
        abi = "hashCrossDomainMessageHighVersion(uint16,uint240,address,address,uint256,uint256,bytes)"
    )]
    pub struct HashCrossDomainMessageHighVersionCall {
        pub version: u16,
        pub nonce: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub gas_limit: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hashCrossDomainMessageV0` function with signature `hashCrossDomainMessageV0(uint240,address,address,uint256,uint256,bytes)` and selector `0x40e15e04`
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
        name = "hashCrossDomainMessageV0",
        abi = "hashCrossDomainMessageV0(uint240,address,address,uint256,uint256,bytes)"
    )]
    pub struct HashCrossDomainMessageV0Call {
        pub nonce: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub gas_limit: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hashCrossDomainMessageV1` function with signature `hashCrossDomainMessageV1(uint240,address,address,uint256,uint256,bytes)` and selector `0x931dad1a`
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
        name = "hashCrossDomainMessageV1",
        abi = "hashCrossDomainMessageV1(uint240,address,address,uint256,uint256,bytes)"
    )]
    pub struct HashCrossDomainMessageV1Call {
        pub nonce: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub gas_limit: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum Hash_CrossDomainHasherCalls {
        FailedCrossDomainHashHighVersion(FailedCrossDomainHashHighVersionCall),
        FailedCrossDomainHashV0(FailedCrossDomainHashV0Call),
        FailedCrossDomainHashV1(FailedCrossDomainHashV1Call),
        HashCrossDomainMessageHighVersion(HashCrossDomainMessageHighVersionCall),
        HashCrossDomainMessageV0(HashCrossDomainMessageV0Call),
        HashCrossDomainMessageV1(HashCrossDomainMessageV1Call),
    }
    impl ::ethers::core::abi::AbiDecode for Hash_CrossDomainHasherCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FailedCrossDomainHashHighVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedCrossDomainHashHighVersion(decoded));
            }
            if let Ok(decoded) = <FailedCrossDomainHashV0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedCrossDomainHashV0(decoded));
            }
            if let Ok(decoded) = <FailedCrossDomainHashV1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedCrossDomainHashV1(decoded));
            }
            if let Ok(decoded) = <HashCrossDomainMessageHighVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashCrossDomainMessageHighVersion(decoded));
            }
            if let Ok(decoded) = <HashCrossDomainMessageV0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashCrossDomainMessageV0(decoded));
            }
            if let Ok(decoded) = <HashCrossDomainMessageV1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashCrossDomainMessageV1(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Hash_CrossDomainHasherCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FailedCrossDomainHashHighVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedCrossDomainHashV0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedCrossDomainHashV1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashCrossDomainMessageHighVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashCrossDomainMessageV0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashCrossDomainMessageV1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Hash_CrossDomainHasherCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FailedCrossDomainHashHighVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedCrossDomainHashV0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedCrossDomainHashV1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HashCrossDomainMessageHighVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HashCrossDomainMessageV0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HashCrossDomainMessageV1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FailedCrossDomainHashHighVersionCall>
    for Hash_CrossDomainHasherCalls {
        fn from(value: FailedCrossDomainHashHighVersionCall) -> Self {
            Self::FailedCrossDomainHashHighVersion(value)
        }
    }
    impl ::core::convert::From<FailedCrossDomainHashV0Call>
    for Hash_CrossDomainHasherCalls {
        fn from(value: FailedCrossDomainHashV0Call) -> Self {
            Self::FailedCrossDomainHashV0(value)
        }
    }
    impl ::core::convert::From<FailedCrossDomainHashV1Call>
    for Hash_CrossDomainHasherCalls {
        fn from(value: FailedCrossDomainHashV1Call) -> Self {
            Self::FailedCrossDomainHashV1(value)
        }
    }
    impl ::core::convert::From<HashCrossDomainMessageHighVersionCall>
    for Hash_CrossDomainHasherCalls {
        fn from(value: HashCrossDomainMessageHighVersionCall) -> Self {
            Self::HashCrossDomainMessageHighVersion(value)
        }
    }
    impl ::core::convert::From<HashCrossDomainMessageV0Call>
    for Hash_CrossDomainHasherCalls {
        fn from(value: HashCrossDomainMessageV0Call) -> Self {
            Self::HashCrossDomainMessageV0(value)
        }
    }
    impl ::core::convert::From<HashCrossDomainMessageV1Call>
    for Hash_CrossDomainHasherCalls {
        fn from(value: HashCrossDomainMessageV1Call) -> Self {
            Self::HashCrossDomainMessageV1(value)
        }
    }
    ///Container type for all return fields from the `failedCrossDomainHashHighVersion` function with signature `failedCrossDomainHashHighVersion()` and selector `0xa2c4a7dc`
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
    pub struct FailedCrossDomainHashHighVersionReturn(pub bool);
    ///Container type for all return fields from the `failedCrossDomainHashV0` function with signature `failedCrossDomainHashV0()` and selector `0xe0136234`
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
    pub struct FailedCrossDomainHashV0Return(pub bool);
    ///Container type for all return fields from the `failedCrossDomainHashV1` function with signature `failedCrossDomainHashV1()` and selector `0xb32fcf9a`
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
    pub struct FailedCrossDomainHashV1Return(pub bool);
}

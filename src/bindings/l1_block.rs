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
pub mod l1_block {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEPOSITOR_ACCOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEPOSITOR_ACCOUNT"),
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
                    ::std::borrow::ToOwned::to_owned("basefee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("basefee"),
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
                    ::std::borrow::ToOwned::to_owned("batcherHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batcherHash"),
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
                    ::std::borrow::ToOwned::to_owned("hash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hash"),
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
                    ::std::borrow::ToOwned::to_owned("l1FeeOverhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1FeeOverhead"),
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
                    ::std::borrow::ToOwned::to_owned("l1FeeScalar"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1FeeScalar"),
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
                    ::std::borrow::ToOwned::to_owned("number"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("number"),
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
                    ::std::borrow::ToOwned::to_owned("sequenceNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sequenceNumber"),
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
                    ::std::borrow::ToOwned::to_owned("setL1BlockValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setL1BlockValues"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_number"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_basefee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_hash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sequenceNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_batcherHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l1FeeOverhead"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_l1FeeScalar"),
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
                    ::std::borrow::ToOwned::to_owned("timestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
    pub static L1BLOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x01`\x80R`\0`\xA0R`\x02`\xC0R`\x80Q`\xA0Q`\xC0Qa\tZa\0\x9C`\09`\0a\x04u\x01R`\0a\x04L\x01R`\0a\x04#\x01Ra\tZ`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01KW`\x005`\xE0\x1C\x80c\x83\x81\xF5\x8A\x11a\x01\x03W\x80c\xB8\x07w\xEA\x11a\0\xDDW\x80c\xB8\x07w\xEA\x14a\x02tW\x80c\xE5\x91\xB2\x82\x14a\x02\x94W\x80c\xE8\x1B,m\x14a\x02\xD4Wa\x01KV[\x80c\x83\x81\xF5\x8A\x14a\x02NW\x80c\x8B#\x9Fs\x14a\x02bW\x80c\x9E\x8CIf\x14a\x02kWa\x01KV[\x80cT\xFDMP\x11a\x014W\x80cT\xFDMP\x14a\x02\x03W\x80c\\\xF2Ii\x14a\x02\x18W\x80cd\xCA#\xEF\x14a\x02!Wa\x01KV[\x80c\x01]\x8E\xB9\x14a\x01\xD2W\x80c\t\xBDZ`\x14a\x01\xE7W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xE5a\x01\xE06`\x04a\x06\x19V[a\x02\xDDV[\0[a\x01\xF0`\x02T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x0Ba\x04\x1CV[`@Qa\x01\xFA\x91\x90a\x07;V[a\x01\xF0`\x01T\x81V[`\x03Ta\x025\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFAV[`\0Ta\x025\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xF0`\x05T\x81V[a\x01\xF0`\x06T\x81V[`\0Ta\x025\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xAFs\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\x01\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFAV[a\x01\xF0`\x04T\x81V[3s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\x01\x14a\x03\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FL1Block: only the depositor acco`D\x82\x01R\x7Funt can set L1 block values\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x98\x89\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x99\x89\x16\x99\x90\x99\x17\x98\x90\x98\x17\x90\x97U`\x01\x94\x90\x94U`\x02\x92\x90\x92U`\x03\x80T\x91\x90\x94\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x90\x91\x16\x17\x90\x92U`\x04\x91\x90\x91U`\x05U`\x06UV[``a\x04G\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xBFV[a\x04p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xBFV[a\x04\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xBFV[`@Q` \x01a\x04\xAB\x93\x92\x91\x90a\x07\x8CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x81`\0\x03a\x05\x02WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x05,W\x80a\x05\x16\x81a\x081V[\x91Pa\x05%\x90P`\n\x83a\x08\x98V[\x91Pa\x05\x06V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05GWa\x05Ga\x08\xACV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05qW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x05\xF4Wa\x05\x86`\x01\x83a\x08\xDBV[\x91Pa\x05\x93`\n\x86a\x08\xF2V[a\x05\x9E\x90`0a\t\x06V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x05\xB3Wa\x05\xB3a\t\x1EV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x05\xED`\n\x86a\x08\x98V[\x94Pa\x05uV[\x94\x93PPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\x14W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x06\xB6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a\x06\xBF\x89a\x05\xFCV[\x97Pa\x06\xCD` \x8A\x01a\x05\xFCV[\x96P`@\x89\x015\x95P``\x89\x015\x94Pa\x06\xE9`\x80\x8A\x01a\x05\xFCV[\x97\x9A\x96\x99P\x94\x97\x93\x96\x95`\xA0\x85\x015\x95P`\xC0\x85\x015\x94`\xE0\x015\x93P\x91PPV[`\0[\x83\x81\x10\x15a\x07&W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\x0EV[\x83\x81\x11\x15a\x075W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x07Z\x81`@\x85\x01` \x87\x01a\x07\x0BV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x84Qa\x07\x9E\x81\x84` \x89\x01a\x07\x0BV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x07\xDA\x81`\x01\x85\x01` \x8A\x01a\x07\x0BV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x07\xF5\x81`\x02\x84\x01` \x88\x01a\x07\x0BV[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x08bWa\x08ba\x08\x02V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x08\xA7Wa\x08\xA7a\x08iV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x08\xEDWa\x08\xEDa\x08\x02V[P\x03\x90V[`\0\x82a\t\x01Wa\t\x01a\x08iV[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\t\x19Wa\t\x19a\x08\x02V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L1BLOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01KW`\x005`\xE0\x1C\x80c\x83\x81\xF5\x8A\x11a\x01\x03W\x80c\xB8\x07w\xEA\x11a\0\xDDW\x80c\xB8\x07w\xEA\x14a\x02tW\x80c\xE5\x91\xB2\x82\x14a\x02\x94W\x80c\xE8\x1B,m\x14a\x02\xD4Wa\x01KV[\x80c\x83\x81\xF5\x8A\x14a\x02NW\x80c\x8B#\x9Fs\x14a\x02bW\x80c\x9E\x8CIf\x14a\x02kWa\x01KV[\x80cT\xFDMP\x11a\x014W\x80cT\xFDMP\x14a\x02\x03W\x80c\\\xF2Ii\x14a\x02\x18W\x80cd\xCA#\xEF\x14a\x02!Wa\x01KV[\x80c\x01]\x8E\xB9\x14a\x01\xD2W\x80c\t\xBDZ`\x14a\x01\xE7W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xE5a\x01\xE06`\x04a\x06\x19V[a\x02\xDDV[\0[a\x01\xF0`\x02T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x0Ba\x04\x1CV[`@Qa\x01\xFA\x91\x90a\x07;V[a\x01\xF0`\x01T\x81V[`\x03Ta\x025\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFAV[`\0Ta\x025\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xF0`\x05T\x81V[a\x01\xF0`\x06T\x81V[`\0Ta\x025\x90h\x01\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xAFs\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\x01\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFAV[a\x01\xF0`\x04T\x81V[3s\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\xDE\xAD\0\x01\x14a\x03\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FL1Block: only the depositor acco`D\x82\x01R\x7Funt can set L1 block values\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x98\x89\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x99\x89\x16\x99\x90\x99\x17\x98\x90\x98\x17\x90\x97U`\x01\x94\x90\x94U`\x02\x92\x90\x92U`\x03\x80T\x91\x90\x94\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x90\x91\x16\x17\x90\x92U`\x04\x91\x90\x91U`\x05U`\x06UV[``a\x04G\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xBFV[a\x04p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xBFV[a\x04\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xBFV[`@Q` \x01a\x04\xAB\x93\x92\x91\x90a\x07\x8CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x81`\0\x03a\x05\x02WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x05,W\x80a\x05\x16\x81a\x081V[\x91Pa\x05%\x90P`\n\x83a\x08\x98V[\x91Pa\x05\x06V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05GWa\x05Ga\x08\xACV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05qW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x05\xF4Wa\x05\x86`\x01\x83a\x08\xDBV[\x91Pa\x05\x93`\n\x86a\x08\xF2V[a\x05\x9E\x90`0a\t\x06V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x05\xB3Wa\x05\xB3a\t\x1EV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x05\xED`\n\x86a\x08\x98V[\x94Pa\x05uV[\x94\x93PPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\x14W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x06\xB6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a\x06\xBF\x89a\x05\xFCV[\x97Pa\x06\xCD` \x8A\x01a\x05\xFCV[\x96P`@\x89\x015\x95P``\x89\x015\x94Pa\x06\xE9`\x80\x8A\x01a\x05\xFCV[\x97\x9A\x96\x99P\x94\x97\x93\x96\x95`\xA0\x85\x015\x95P`\xC0\x85\x015\x94`\xE0\x015\x93P\x91PPV[`\0[\x83\x81\x10\x15a\x07&W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\x0EV[\x83\x81\x11\x15a\x075W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x07Z\x81`@\x85\x01` \x87\x01a\x07\x0BV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x84Qa\x07\x9E\x81\x84` \x89\x01a\x07\x0BV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x07\xDA\x81`\x01\x85\x01` \x8A\x01a\x07\x0BV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x07\xF5\x81`\x02\x84\x01` \x88\x01a\x07\x0BV[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x08bWa\x08ba\x08\x02V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x08\xA7Wa\x08\xA7a\x08iV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x08\xEDWa\x08\xEDa\x08\x02V[P\x03\x90V[`\0\x82a\t\x01Wa\t\x01a\x08iV[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\t\x19Wa\t\x19a\x08\x02V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L1BLOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L1Block<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L1Block<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L1Block<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L1Block<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L1Block<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L1Block)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L1Block<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L1BLOCK_ABI.clone(),
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
                L1BLOCK_ABI.clone(),
                L1BLOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEPOSITOR_ACCOUNT` (0xe591b282) function
        pub fn depositor_account(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([229, 145, 178, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `basefee` (0x5cf24969) function
        pub fn basefee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 242, 73, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batcherHash` (0xe81b2c6d) function
        pub fn batcher_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 27, 44, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hash` (0x09bd5a60) function
        pub fn hash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([9, 189, 90, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1FeeOverhead` (0x8b239f73) function
        pub fn l_1_fee_overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 35, 159, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1FeeScalar` (0x9e8c4966) function
        pub fn l_1_fee_scalar(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 140, 73, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `number` (0x8381f58a) function
        pub fn number(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([131, 129, 245, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sequenceNumber` (0x64ca23ef) function
        pub fn sequence_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([100, 202, 35, 239], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setL1BlockValues` (0x015d8eb9) function
        pub fn set_l1_block_values(
            &self,
            number: u64,
            timestamp: u64,
            basefee: ::ethers::core::types::U256,
            hash: [u8; 32],
            sequence_number: u64,
            batcher_hash: [u8; 32],
            l_1_fee_overhead: ::ethers::core::types::U256,
            l_1_fee_scalar: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [1, 93, 142, 185],
                    (
                        number,
                        timestamp,
                        basefee,
                        hash,
                        sequence_number,
                        batcher_hash,
                        l_1_fee_overhead,
                        l_1_fee_scalar,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timestamp` (0xb80777ea) function
        pub fn timestamp(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([184, 7, 119, 234], ())
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
    for L1Block<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `DEPOSITOR_ACCOUNT` function with signature `DEPOSITOR_ACCOUNT()` and selector `0xe591b282`
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
    #[ethcall(name = "DEPOSITOR_ACCOUNT", abi = "DEPOSITOR_ACCOUNT()")]
    pub struct DepositorAccountCall;
    ///Container type for all input parameters for the `basefee` function with signature `basefee()` and selector `0x5cf24969`
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
    #[ethcall(name = "basefee", abi = "basefee()")]
    pub struct BasefeeCall;
    ///Container type for all input parameters for the `batcherHash` function with signature `batcherHash()` and selector `0xe81b2c6d`
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
    #[ethcall(name = "batcherHash", abi = "batcherHash()")]
    pub struct BatcherHashCall;
    ///Container type for all input parameters for the `hash` function with signature `hash()` and selector `0x09bd5a60`
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
    #[ethcall(name = "hash", abi = "hash()")]
    pub struct HashCall;
    ///Container type for all input parameters for the `l1FeeOverhead` function with signature `l1FeeOverhead()` and selector `0x8b239f73`
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
    #[ethcall(name = "l1FeeOverhead", abi = "l1FeeOverhead()")]
    pub struct L1FeeOverheadCall;
    ///Container type for all input parameters for the `l1FeeScalar` function with signature `l1FeeScalar()` and selector `0x9e8c4966`
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
    #[ethcall(name = "l1FeeScalar", abi = "l1FeeScalar()")]
    pub struct L1FeeScalarCall;
    ///Container type for all input parameters for the `number` function with signature `number()` and selector `0x8381f58a`
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
    #[ethcall(name = "number", abi = "number()")]
    pub struct NumberCall;
    ///Container type for all input parameters for the `sequenceNumber` function with signature `sequenceNumber()` and selector `0x64ca23ef`
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
    #[ethcall(name = "sequenceNumber", abi = "sequenceNumber()")]
    pub struct SequenceNumberCall;
    ///Container type for all input parameters for the `setL1BlockValues` function with signature `setL1BlockValues(uint64,uint64,uint256,bytes32,uint64,bytes32,uint256,uint256)` and selector `0x015d8eb9`
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
        name = "setL1BlockValues",
        abi = "setL1BlockValues(uint64,uint64,uint256,bytes32,uint64,bytes32,uint256,uint256)"
    )]
    pub struct SetL1BlockValuesCall {
        pub number: u64,
        pub timestamp: u64,
        pub basefee: ::ethers::core::types::U256,
        pub hash: [u8; 32],
        pub sequence_number: u64,
        pub batcher_hash: [u8; 32],
        pub l_1_fee_overhead: ::ethers::core::types::U256,
        pub l_1_fee_scalar: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `timestamp` function with signature `timestamp()` and selector `0xb80777ea`
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
    #[ethcall(name = "timestamp", abi = "timestamp()")]
    pub struct TimestampCall;
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
    pub enum L1BlockCalls {
        DepositorAccount(DepositorAccountCall),
        Basefee(BasefeeCall),
        BatcherHash(BatcherHashCall),
        Hash(HashCall),
        L1FeeOverhead(L1FeeOverheadCall),
        L1FeeScalar(L1FeeScalarCall),
        Number(NumberCall),
        SequenceNumber(SequenceNumberCall),
        SetL1BlockValues(SetL1BlockValuesCall),
        Timestamp(TimestampCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for L1BlockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositorAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositorAccount(decoded));
            }
            if let Ok(decoded) = <BasefeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Basefee(decoded));
            }
            if let Ok(decoded) = <BatcherHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatcherHash(decoded));
            }
            if let Ok(decoded) = <HashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Hash(decoded));
            }
            if let Ok(decoded) = <L1FeeOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1FeeOverhead(decoded));
            }
            if let Ok(decoded) = <L1FeeScalarCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1FeeScalar(decoded));
            }
            if let Ok(decoded) = <NumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Number(decoded));
            }
            if let Ok(decoded) = <SequenceNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SequenceNumber(decoded));
            }
            if let Ok(decoded) = <SetL1BlockValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetL1BlockValues(decoded));
            }
            if let Ok(decoded) = <TimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Timestamp(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L1BlockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositorAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Basefee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BatcherHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Hash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::L1FeeOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1FeeScalar(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Number(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SequenceNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetL1BlockValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Timestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for L1BlockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositorAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Basefee(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatcherHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hash(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1FeeOverhead(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1FeeScalar(element) => ::core::fmt::Display::fmt(element, f),
                Self::Number(element) => ::core::fmt::Display::fmt(element, f),
                Self::SequenceNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetL1BlockValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::Timestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositorAccountCall> for L1BlockCalls {
        fn from(value: DepositorAccountCall) -> Self {
            Self::DepositorAccount(value)
        }
    }
    impl ::core::convert::From<BasefeeCall> for L1BlockCalls {
        fn from(value: BasefeeCall) -> Self {
            Self::Basefee(value)
        }
    }
    impl ::core::convert::From<BatcherHashCall> for L1BlockCalls {
        fn from(value: BatcherHashCall) -> Self {
            Self::BatcherHash(value)
        }
    }
    impl ::core::convert::From<HashCall> for L1BlockCalls {
        fn from(value: HashCall) -> Self {
            Self::Hash(value)
        }
    }
    impl ::core::convert::From<L1FeeOverheadCall> for L1BlockCalls {
        fn from(value: L1FeeOverheadCall) -> Self {
            Self::L1FeeOverhead(value)
        }
    }
    impl ::core::convert::From<L1FeeScalarCall> for L1BlockCalls {
        fn from(value: L1FeeScalarCall) -> Self {
            Self::L1FeeScalar(value)
        }
    }
    impl ::core::convert::From<NumberCall> for L1BlockCalls {
        fn from(value: NumberCall) -> Self {
            Self::Number(value)
        }
    }
    impl ::core::convert::From<SequenceNumberCall> for L1BlockCalls {
        fn from(value: SequenceNumberCall) -> Self {
            Self::SequenceNumber(value)
        }
    }
    impl ::core::convert::From<SetL1BlockValuesCall> for L1BlockCalls {
        fn from(value: SetL1BlockValuesCall) -> Self {
            Self::SetL1BlockValues(value)
        }
    }
    impl ::core::convert::From<TimestampCall> for L1BlockCalls {
        fn from(value: TimestampCall) -> Self {
            Self::Timestamp(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L1BlockCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `DEPOSITOR_ACCOUNT` function with signature `DEPOSITOR_ACCOUNT()` and selector `0xe591b282`
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
    pub struct DepositorAccountReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `basefee` function with signature `basefee()` and selector `0x5cf24969`
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
    pub struct BasefeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `batcherHash` function with signature `batcherHash()` and selector `0xe81b2c6d`
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
    pub struct BatcherHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hash` function with signature `hash()` and selector `0x09bd5a60`
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
    pub struct HashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `l1FeeOverhead` function with signature `l1FeeOverhead()` and selector `0x8b239f73`
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
    pub struct L1FeeOverheadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l1FeeScalar` function with signature `l1FeeScalar()` and selector `0x9e8c4966`
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
    pub struct L1FeeScalarReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `number` function with signature `number()` and selector `0x8381f58a`
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
    pub struct NumberReturn(pub u64);
    ///Container type for all return fields from the `sequenceNumber` function with signature `sequenceNumber()` and selector `0x64ca23ef`
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
    pub struct SequenceNumberReturn(pub u64);
    ///Container type for all return fields from the `timestamp` function with signature `timestamp()` and selector `0xb80777ea`
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
    pub struct TimestampReturn(pub u64);
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

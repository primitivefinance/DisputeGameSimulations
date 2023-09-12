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
pub mod preimage_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("cheat"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cheat"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("partOffset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("part"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("size"),
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
                    ::std::borrow::ToOwned::to_owned("loadKeccak256PreimagePart"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "loadKeccak256PreimagePart",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_partOffset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_preimage"),
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
                    ::std::borrow::ToOwned::to_owned("loadLocalData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("loadLocalData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ident"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_word"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_size"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_partOffset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key_"),
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
                    ::std::borrow::ToOwned::to_owned("preimageLengths"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("preimageLengths"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("preimagePartOk"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("preimagePartOk"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("preimageParts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("preimageParts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("readPreimage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readPreimage"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("_offset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dat_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("datLen_"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PartOffsetOOB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PartOffsetOOB"),
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
    pub static PREIMAGEORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\n<\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xFFW`\x005`\xE0\x1C\x80c\xE01\x10\xE1\x11a\0\xDDW\x80c\xE01\x10\xE1\x14a\x02\x15W\x80c\xE1Y&\x11\x14a\x02=W\x80c\xFEJ\xC0\x8E\x14a\x02RW\x80c\xFE\xF2\xB4\xED\x14a\x02\xC7Wa\0\xFFV[\x80ca#\x8B\xDE\x14a\x01\x86W\x80c\x85B\xCFP\x14a\x01\xC4W\x80c\x9A\x1F^\x7F\x14a\x02\x02W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xB1a\x01\x946`\x04a\x06\xDAV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF2a\x01\xD26`\x04a\x06\xDAV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBBV[a\x01\xB1a\x02\x106`\x04a\x06\xFFV[a\x02\xE7V[a\x02(a\x02#6`\x04a\x06\xDAV[a\x03\xBAV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xBBV[a\x02Pa\x02K6`\x04a\x07\xB9V[a\x04\xABV[\0[a\x02Pa\x02`6`\x04a\x06\xFFV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 \x87\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x86\x84R\x82R\x80\x83 \x96\x83R\x95\x81R\x85\x82 \x93\x90\x93U\x92\x83R\x90\x82\x90R\x91\x90 UV[a\x01\xB1a\x02\xD56`\x04a\t\xB5V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`\0a\x02\xF2\x85a\x05\xB4V[\x90Pa\x02\xFF\x83`\x08a\n\0V[\x82\x11\x80a\x03\x0CWP` \x83\x11[\x15a\x03CW`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x81\x81R`\xC0\x85\x90\x1B\x82R`\x08\x95\x90\x95R\x82Q\x82\x82R`\x02\x86R`@\x80\x83 \x85\x84R\x87R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x87R\x80\x83 \x94\x83R\x93\x86R\x83\x82 U\x81\x81R\x93\x84\x90R\x92 U\x91\x90PV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81\x90`\xFF\x16a\x04CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fpre-image must exist\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P`\0\x83\x81R` \x81\x81R`@\x90\x91 Ta\x04_\x81`\x08a\n\0V[a\x04j\x85` a\n\0V[\x10a\x04\x88W\x83a\x04{\x82`\x08a\n\0V[a\x04\x85\x91\x90a\n\x18V[\x91P[P`\0\x93\x84R`\x01` \x90\x81R`@\x80\x86 \x94\x86R\x93\x90R\x91\x90\x92 T\x92\x90\x91PV[`D5`\0\x80`\x08\x83\x01\x86\x11\x15a\x04\xCAWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x83\x90\x1B`\x80R`\x88\x83\x86\x827\x80\x87\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01Q\x90\x84\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x8B\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x82R\x80\x83 \x9A\x83R\x99\x81R\x89\x82 \x93\x90\x93U\x90\x81R\x90\x81\x90R\x95\x90\x95 \x91\x90\x91UPPPPV[\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x17a\x06O\x81`\0\x90\x81R3` R`@\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x92\x91PPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xF0Wa\x06\xF0a\x06UV[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x07\x18Wa\x07\x18a\x06UV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07\xD1Wa\x07\xD1a\x06UV[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\t\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\t\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x88\x84\x82\x85\x01\x01\x11\x15a\t\xA5Wa\t\xA5a\x074V[\x95\x98\x91\x90\x92\x01\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\t\xCAWa\t\xCAa\x06UV[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\n\x13Wa\n\x13a\t\xD1V[P\x01\x90V[`\0\x82\x82\x10\x15a\n*Wa\n*a\t\xD1V[P\x03\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static PREIMAGEORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xFFW`\x005`\xE0\x1C\x80c\xE01\x10\xE1\x11a\0\xDDW\x80c\xE01\x10\xE1\x14a\x02\x15W\x80c\xE1Y&\x11\x14a\x02=W\x80c\xFEJ\xC0\x8E\x14a\x02RW\x80c\xFE\xF2\xB4\xED\x14a\x02\xC7Wa\0\xFFV[\x80ca#\x8B\xDE\x14a\x01\x86W\x80c\x85B\xCFP\x14a\x01\xC4W\x80c\x9A\x1F^\x7F\x14a\x02\x02W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xB1a\x01\x946`\x04a\x06\xDAV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF2a\x01\xD26`\x04a\x06\xDAV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBBV[a\x01\xB1a\x02\x106`\x04a\x06\xFFV[a\x02\xE7V[a\x02(a\x02#6`\x04a\x06\xDAV[a\x03\xBAV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xBBV[a\x02Pa\x02K6`\x04a\x07\xB9V[a\x04\xABV[\0[a\x02Pa\x02`6`\x04a\x06\xFFV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 \x87\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x86\x84R\x82R\x80\x83 \x96\x83R\x95\x81R\x85\x82 \x93\x90\x93U\x92\x83R\x90\x82\x90R\x91\x90 UV[a\x01\xB1a\x02\xD56`\x04a\t\xB5V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`\0a\x02\xF2\x85a\x05\xB4V[\x90Pa\x02\xFF\x83`\x08a\n\0V[\x82\x11\x80a\x03\x0CWP` \x83\x11[\x15a\x03CW`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x81\x81R`\xC0\x85\x90\x1B\x82R`\x08\x95\x90\x95R\x82Q\x82\x82R`\x02\x86R`@\x80\x83 \x85\x84R\x87R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x87R\x80\x83 \x94\x83R\x93\x86R\x83\x82 U\x81\x81R\x93\x84\x90R\x92 U\x91\x90PV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81\x90`\xFF\x16a\x04CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fpre-image must exist\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P`\0\x83\x81R` \x81\x81R`@\x90\x91 Ta\x04_\x81`\x08a\n\0V[a\x04j\x85` a\n\0V[\x10a\x04\x88W\x83a\x04{\x82`\x08a\n\0V[a\x04\x85\x91\x90a\n\x18V[\x91P[P`\0\x93\x84R`\x01` \x90\x81R`@\x80\x86 \x94\x86R\x93\x90R\x91\x90\x92 T\x92\x90\x91PV[`D5`\0\x80`\x08\x83\x01\x86\x11\x15a\x04\xCAWc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x83\x90\x1B`\x80R`\x88\x83\x86\x827\x80\x87\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01Q\x90\x84\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x8B\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x82R\x80\x83 \x9A\x83R\x99\x81R\x89\x82 \x93\x90\x93U\x90\x81R\x90\x81\x90R\x95\x90\x95 \x91\x90\x91UPPPPV[\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x17a\x06O\x81`\0\x90\x81R3` R`@\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x92\x91PPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xF0Wa\x06\xF0a\x06UV[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x07\x18Wa\x07\x18a\x06UV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07\xD1Wa\x07\xD1a\x06UV[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\t\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\t\x91W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x88\x84\x82\x85\x01\x01\x11\x15a\t\xA5Wa\t\xA5a\x074V[\x95\x98\x91\x90\x92\x01\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\t\xCAWa\t\xCAa\x06UV[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\n\x13Wa\n\x13a\t\xD1V[P\x01\x90V[`\0\x82\x82\x10\x15a\n*Wa\n*a\t\xD1V[P\x03\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static PREIMAGEORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PreimageOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PreimageOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PreimageOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PreimageOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PreimageOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PreimageOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PreimageOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PREIMAGEORACLE_ABI.clone(),
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
                PREIMAGEORACLE_ABI.clone(),
                PREIMAGEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `cheat` (0xfe4ac08e) function
        pub fn cheat(
            &self,
            part_offset: ::ethers::core::types::U256,
            key: [u8; 32],
            part: [u8; 32],
            size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 74, 192, 142], (part_offset, key, part, size))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `loadKeccak256PreimagePart` (0xe1592611) function
        pub fn load_keccak_256_preimage_part(
            &self,
            part_offset: ::ethers::core::types::U256,
            preimage: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 89, 38, 17], (part_offset, preimage))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `loadLocalData` (0x9a1f5e7f) function
        pub fn load_local_data(
            &self,
            ident: ::ethers::core::types::U256,
            word: [u8; 32],
            size: ::ethers::core::types::U256,
            part_offset: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([154, 31, 94, 127], (ident, word, size, part_offset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `preimageLengths` (0xfef2b4ed) function
        pub fn preimage_lengths(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 242, 180, 237], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `preimagePartOk` (0x8542cf50) function
        pub fn preimage_part_ok(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([133, 66, 207, 80], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `preimageParts` (0x61238bde) function
        pub fn preimage_parts(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([97, 35, 139, 222], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readPreimage` (0xe03110e1) function
        pub fn read_preimage(
            &self,
            key: [u8; 32],
            offset: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([224, 49, 16, 225], (key, offset))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PreimageOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `PartOffsetOOB` with signature `PartOffsetOOB()` and selector `0xfe254987`
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
    #[etherror(name = "PartOffsetOOB", abi = "PartOffsetOOB()")]
    pub struct PartOffsetOOB;
    ///Container type for all input parameters for the `cheat` function with signature `cheat(uint256,bytes32,bytes32,uint256)` and selector `0xfe4ac08e`
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
    #[ethcall(name = "cheat", abi = "cheat(uint256,bytes32,bytes32,uint256)")]
    pub struct CheatCall {
        pub part_offset: ::ethers::core::types::U256,
        pub key: [u8; 32],
        pub part: [u8; 32],
        pub size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `loadKeccak256PreimagePart` function with signature `loadKeccak256PreimagePart(uint256,bytes)` and selector `0xe1592611`
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
        name = "loadKeccak256PreimagePart",
        abi = "loadKeccak256PreimagePart(uint256,bytes)"
    )]
    pub struct LoadKeccak256PreimagePartCall {
        pub part_offset: ::ethers::core::types::U256,
        pub preimage: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `loadLocalData` function with signature `loadLocalData(uint256,bytes32,uint256,uint256)` and selector `0x9a1f5e7f`
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
        name = "loadLocalData",
        abi = "loadLocalData(uint256,bytes32,uint256,uint256)"
    )]
    pub struct LoadLocalDataCall {
        pub ident: ::ethers::core::types::U256,
        pub word: [u8; 32],
        pub size: ::ethers::core::types::U256,
        pub part_offset: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `preimageLengths` function with signature `preimageLengths(bytes32)` and selector `0xfef2b4ed`
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
    #[ethcall(name = "preimageLengths", abi = "preimageLengths(bytes32)")]
    pub struct PreimageLengthsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `preimagePartOk` function with signature `preimagePartOk(bytes32,uint256)` and selector `0x8542cf50`
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
    #[ethcall(name = "preimagePartOk", abi = "preimagePartOk(bytes32,uint256)")]
    pub struct PreimagePartOkCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `preimageParts` function with signature `preimageParts(bytes32,uint256)` and selector `0x61238bde`
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
    #[ethcall(name = "preimageParts", abi = "preimageParts(bytes32,uint256)")]
    pub struct PreimagePartsCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `readPreimage` function with signature `readPreimage(bytes32,uint256)` and selector `0xe03110e1`
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
    #[ethcall(name = "readPreimage", abi = "readPreimage(bytes32,uint256)")]
    pub struct ReadPreimageCall {
        pub key: [u8; 32],
        pub offset: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PreimageOracleCalls {
        Cheat(CheatCall),
        LoadKeccak256PreimagePart(LoadKeccak256PreimagePartCall),
        LoadLocalData(LoadLocalDataCall),
        PreimageLengths(PreimageLengthsCall),
        PreimagePartOk(PreimagePartOkCall),
        PreimageParts(PreimagePartsCall),
        ReadPreimage(ReadPreimageCall),
    }
    impl ::ethers::core::abi::AbiDecode for PreimageOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheatCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cheat(decoded));
            }
            if let Ok(decoded) = <LoadKeccak256PreimagePartCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LoadKeccak256PreimagePart(decoded));
            }
            if let Ok(decoded) = <LoadLocalDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LoadLocalData(decoded));
            }
            if let Ok(decoded) = <PreimageLengthsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreimageLengths(decoded));
            }
            if let Ok(decoded) = <PreimagePartOkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreimagePartOk(decoded));
            }
            if let Ok(decoded) = <PreimagePartsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreimageParts(decoded));
            }
            if let Ok(decoded) = <ReadPreimageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadPreimage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PreimageOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Cheat(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LoadKeccak256PreimagePart(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LoadLocalData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreimageLengths(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreimagePartOk(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreimageParts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadPreimage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PreimageOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Cheat(element) => ::core::fmt::Display::fmt(element, f),
                Self::LoadKeccak256PreimagePart(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LoadLocalData(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreimageLengths(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreimagePartOk(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreimageParts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadPreimage(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheatCall> for PreimageOracleCalls {
        fn from(value: CheatCall) -> Self {
            Self::Cheat(value)
        }
    }
    impl ::core::convert::From<LoadKeccak256PreimagePartCall> for PreimageOracleCalls {
        fn from(value: LoadKeccak256PreimagePartCall) -> Self {
            Self::LoadKeccak256PreimagePart(value)
        }
    }
    impl ::core::convert::From<LoadLocalDataCall> for PreimageOracleCalls {
        fn from(value: LoadLocalDataCall) -> Self {
            Self::LoadLocalData(value)
        }
    }
    impl ::core::convert::From<PreimageLengthsCall> for PreimageOracleCalls {
        fn from(value: PreimageLengthsCall) -> Self {
            Self::PreimageLengths(value)
        }
    }
    impl ::core::convert::From<PreimagePartOkCall> for PreimageOracleCalls {
        fn from(value: PreimagePartOkCall) -> Self {
            Self::PreimagePartOk(value)
        }
    }
    impl ::core::convert::From<PreimagePartsCall> for PreimageOracleCalls {
        fn from(value: PreimagePartsCall) -> Self {
            Self::PreimageParts(value)
        }
    }
    impl ::core::convert::From<ReadPreimageCall> for PreimageOracleCalls {
        fn from(value: ReadPreimageCall) -> Self {
            Self::ReadPreimage(value)
        }
    }
    ///Container type for all return fields from the `loadLocalData` function with signature `loadLocalData(uint256,bytes32,uint256,uint256)` and selector `0x9a1f5e7f`
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
    pub struct LoadLocalDataReturn {
        pub key: [u8; 32],
    }
    ///Container type for all return fields from the `preimageLengths` function with signature `preimageLengths(bytes32)` and selector `0xfef2b4ed`
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
    pub struct PreimageLengthsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `preimagePartOk` function with signature `preimagePartOk(bytes32,uint256)` and selector `0x8542cf50`
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
    pub struct PreimagePartOkReturn(pub bool);
    ///Container type for all return fields from the `preimageParts` function with signature `preimageParts(bytes32,uint256)` and selector `0x61238bde`
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
    pub struct PreimagePartsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `readPreimage` function with signature `readPreimage(bytes32,uint256)` and selector `0xe03110e1`
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
    pub struct ReadPreimageReturn {
        pub dat: [u8; 32],
        pub dat_len: ::ethers::core::types::U256,
    }
}

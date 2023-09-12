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
pub mod faucet_helper {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EIP712_DOMAIN_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EIP712_DOMAIN_TYPEHASH",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PROOF_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PROOF_TYPEHASH"),
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
                    ::std::borrow::ToOwned::to_owned("consumeNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("consumeNonce"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("currentNonce"),
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
                    ::std::borrow::ToOwned::to_owned("getDigestWithEIP712Domain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDigestWithEIP712Domain",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AdminFaucetAuthModule.Proof",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_verifyingContract",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProofStructHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProofStructHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AdminFaucetAuthModule.Proof",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static FAUCETHELPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\t\x86\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF4W`\x005`\xE0\x1C\x80c\xAD\xB6\x10\xA3\x11a\0\xD2W\x80c\xAD\xB6\x10\xA3\x14a\x02FW\x80c\xB1c\xA6\xB8\x14a\x02OW\x80c\xC7\x97{\xE7\x14a\x02bWa\0\xF4V[\x80c4\xB1Q\x18\x14a\x01{W\x80cym\xFB\xFA\x14a\x01\x95W\x80c\x8B>;\xF6\x14a\x02\x1FW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\x83a\x02\x89V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\x83a\x01\xA36`\x04a\x06\xD0V[\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q\x7F\xD4(5\x07\xDCz\x82\x82\xFA\xA6\xB4\xC8\xC1\x8B\xAC\xBBt\xDB\xBA\xB5FsB\xE6\xF5\x81eo5w#n\x81\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84\x86\x01R``\x84\x01\x91\x90\x91R`\x80\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[a\x01\x83\x7F\xD4(5\x07\xDCz\x82\x82\xFA\xA6\xB4\xC8\xC1\x8B\xAC\xBBt\xDB\xBA\xB5FsB\xE6\xF5\x81eo5w#n\x81V[a\x01\x83`\0T\x81V[a\x01\x83a\x02]6`\x04a\x08\x81V[a\x02\xC9V[a\x01\x83\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81V[`\0\x80T\x81\x80a\x02\x98\x83a\t\x1AV[\x91\x90PU`@Q` \x01a\x02\xAE\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x83Q` \x80\x86\x01\x91\x90\x91 \x84Q\x85\x83\x01 `@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x94\x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\xA0\x82\x01R`\0\x90\x81\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x043\x81a\x03\xD8\x89\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q\x7F\xD4(5\x07\xDCz\x82\x82\xFA\xA6\xB4\xC8\xC1\x8B\xAC\xBBt\xDB\xBA\xB5FsB\xE6\xF5\x81eo5w#n\x81\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84\x86\x01R``\x84\x01\x91\x90\x91R`\x80\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x97\x96PPPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xBEWa\x05\xBEa\x05HV[`@R\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xEAW`\0\x80\xFD[\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x06\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\xA4Wa\x06\xA4a\x05HV[`@R\x90P\x80a\x06\xB3\x83a\x05\xC6V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x06\xE5Wa\x06\xE5a\x04>V[a\x06\xEF\x83\x83a\x05\xEFV[\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x07\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xA3Wa\x07\xA3a\x05HV[a\x07\xD3\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01a\x05wV[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15a\x08fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0`\xE0\x86\x88\x03\x12\x15a\x08\x9CWa\x08\x9Ca\x04>V[a\x08\xA6\x87\x87a\x05\xEFV[\x94P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\xC6Wa\x08\xC6a\x04\xC3V[a\x08\xD2\x89\x83\x8A\x01a\x06\xF6V[\x95P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x08\xEBWa\x08\xEBa\x04\xC3V[Pa\x08\xF8\x88\x82\x89\x01a\x06\xF6V[\x93PP`\xA0\x86\x015\x91Pa\t\x0E`\xC0\x87\x01a\x05\xC6V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\trW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static FAUCETHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xF4W`\x005`\xE0\x1C\x80c\xAD\xB6\x10\xA3\x11a\0\xD2W\x80c\xAD\xB6\x10\xA3\x14a\x02FW\x80c\xB1c\xA6\xB8\x14a\x02OW\x80c\xC7\x97{\xE7\x14a\x02bWa\0\xF4V[\x80c4\xB1Q\x18\x14a\x01{W\x80cym\xFB\xFA\x14a\x01\x95W\x80c\x8B>;\xF6\x14a\x02\x1FW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\x83a\x02\x89V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\x83a\x01\xA36`\x04a\x06\xD0V[\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q\x7F\xD4(5\x07\xDCz\x82\x82\xFA\xA6\xB4\xC8\xC1\x8B\xAC\xBBt\xDB\xBA\xB5FsB\xE6\xF5\x81eo5w#n\x81\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84\x86\x01R``\x84\x01\x91\x90\x91R`\x80\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[a\x01\x83\x7F\xD4(5\x07\xDCz\x82\x82\xFA\xA6\xB4\xC8\xC1\x8B\xAC\xBBt\xDB\xBA\xB5FsB\xE6\xF5\x81eo5w#n\x81V[a\x01\x83`\0T\x81V[a\x01\x83a\x02]6`\x04a\x08\x81V[a\x02\xC9V[a\x01\x83\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81V[`\0\x80T\x81\x80a\x02\x98\x83a\t\x1AV[\x91\x90PU`@Q` \x01a\x02\xAE\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x83Q` \x80\x86\x01\x91\x90\x91 \x84Q\x85\x83\x01 `@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x94\x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\xA0\x82\x01R`\0\x90\x81\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x043\x81a\x03\xD8\x89\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q\x7F\xD4(5\x07\xDCz\x82\x82\xFA\xA6\xB4\xC8\xC1\x8B\xAC\xBBt\xDB\xBA\xB5FsB\xE6\xF5\x81eo5w#n\x81\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x94\x16\x84\x86\x01R``\x84\x01\x91\x90\x91R`\x80\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x97\x96PPPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xBEWa\x05\xBEa\x05HV[`@R\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xEAW`\0\x80\xFD[\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x06\x81W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\xA4Wa\x06\xA4a\x05HV[`@R\x90P\x80a\x06\xB3\x83a\x05\xC6V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x06\xE5Wa\x06\xE5a\x04>V[a\x06\xEF\x83\x83a\x05\xEFV[\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x07\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xA3Wa\x07\xA3a\x05HV[a\x07\xD3\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01a\x05wV[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15a\x08fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0`\xE0\x86\x88\x03\x12\x15a\x08\x9CWa\x08\x9Ca\x04>V[a\x08\xA6\x87\x87a\x05\xEFV[\x94P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\xC6Wa\x08\xC6a\x04\xC3V[a\x08\xD2\x89\x83\x8A\x01a\x06\xF6V[\x95P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x08\xEBWa\x08\xEBa\x04\xC3V[Pa\x08\xF8\x88\x82\x89\x01a\x06\xF6V[\x93PP`\xA0\x86\x015\x91Pa\t\x0E`\xC0\x87\x01a\x05\xC6V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\trW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static FAUCETHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FaucetHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FaucetHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FaucetHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FaucetHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FaucetHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FaucetHelper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FaucetHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FAUCETHELPER_ABI.clone(),
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
                FAUCETHELPER_ABI.clone(),
                FAUCETHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `EIP712_DOMAIN_TYPEHASH` (0xc7977be7) function
        pub fn eip712_domain_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([199, 151, 123, 231], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROOF_TYPEHASH` (0x8b3e3bf6) function
        pub fn proof_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([139, 62, 59, 246], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `consumeNonce` (0x34b15118) function
        pub fn consume_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([52, 177, 81, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentNonce` (0xadb610a3) function
        pub fn current_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 182, 16, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDigestWithEIP712Domain` (0xb163a6b8) function
        pub fn get_digest_with_eip712_domain(
            &self,
            proof: Proof,
            name: ::ethers::core::types::Bytes,
            version: ::ethers::core::types::Bytes,
            chainid: ::ethers::core::types::U256,
            verifying_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [177, 99, 166, 184],
                    (proof, name, version, chainid, verifying_contract),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProofStructHash` (0x796dfbfa) function
        pub fn get_proof_struct_hash(
            &self,
            proof: Proof,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 109, 251, 250], (proof,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FaucetHelper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `EIP712_DOMAIN_TYPEHASH` function with signature `EIP712_DOMAIN_TYPEHASH()` and selector `0xc7977be7`
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
    #[ethcall(name = "EIP712_DOMAIN_TYPEHASH", abi = "EIP712_DOMAIN_TYPEHASH()")]
    pub struct Eip712DomainTypehashCall;
    ///Container type for all input parameters for the `PROOF_TYPEHASH` function with signature `PROOF_TYPEHASH()` and selector `0x8b3e3bf6`
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
    #[ethcall(name = "PROOF_TYPEHASH", abi = "PROOF_TYPEHASH()")]
    pub struct ProofTypehashCall;
    ///Container type for all input parameters for the `consumeNonce` function with signature `consumeNonce()` and selector `0x34b15118`
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
    #[ethcall(name = "consumeNonce", abi = "consumeNonce()")]
    pub struct ConsumeNonceCall;
    ///Container type for all input parameters for the `currentNonce` function with signature `currentNonce()` and selector `0xadb610a3`
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
    #[ethcall(name = "currentNonce", abi = "currentNonce()")]
    pub struct CurrentNonceCall;
    ///Container type for all input parameters for the `getDigestWithEIP712Domain` function with signature `getDigestWithEIP712Domain((address,bytes32,bytes32),bytes,bytes,uint256,address)` and selector `0xb163a6b8`
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
        name = "getDigestWithEIP712Domain",
        abi = "getDigestWithEIP712Domain((address,bytes32,bytes32),bytes,bytes,uint256,address)"
    )]
    pub struct GetDigestWithEIP712DomainCall {
        pub proof: Proof,
        pub name: ::ethers::core::types::Bytes,
        pub version: ::ethers::core::types::Bytes,
        pub chainid: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getProofStructHash` function with signature `getProofStructHash((address,bytes32,bytes32))` and selector `0x796dfbfa`
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
        name = "getProofStructHash",
        abi = "getProofStructHash((address,bytes32,bytes32))"
    )]
    pub struct GetProofStructHashCall {
        pub proof: Proof,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FaucetHelperCalls {
        Eip712DomainTypehash(Eip712DomainTypehashCall),
        ProofTypehash(ProofTypehashCall),
        ConsumeNonce(ConsumeNonceCall),
        CurrentNonce(CurrentNonceCall),
        GetDigestWithEIP712Domain(GetDigestWithEIP712DomainCall),
        GetProofStructHash(GetProofStructHashCall),
    }
    impl ::ethers::core::abi::AbiDecode for FaucetHelperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Eip712DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eip712DomainTypehash(decoded));
            }
            if let Ok(decoded) = <ProofTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProofTypehash(decoded));
            }
            if let Ok(decoded) = <ConsumeNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConsumeNonce(decoded));
            }
            if let Ok(decoded) = <CurrentNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentNonce(decoded));
            }
            if let Ok(decoded) = <GetDigestWithEIP712DomainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDigestWithEIP712Domain(decoded));
            }
            if let Ok(decoded) = <GetProofStructHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProofStructHash(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FaucetHelperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Eip712DomainTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConsumeNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDigestWithEIP712Domain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProofStructHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FaucetHelperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Eip712DomainTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProofTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConsumeNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDigestWithEIP712Domain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProofStructHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Eip712DomainTypehashCall> for FaucetHelperCalls {
        fn from(value: Eip712DomainTypehashCall) -> Self {
            Self::Eip712DomainTypehash(value)
        }
    }
    impl ::core::convert::From<ProofTypehashCall> for FaucetHelperCalls {
        fn from(value: ProofTypehashCall) -> Self {
            Self::ProofTypehash(value)
        }
    }
    impl ::core::convert::From<ConsumeNonceCall> for FaucetHelperCalls {
        fn from(value: ConsumeNonceCall) -> Self {
            Self::ConsumeNonce(value)
        }
    }
    impl ::core::convert::From<CurrentNonceCall> for FaucetHelperCalls {
        fn from(value: CurrentNonceCall) -> Self {
            Self::CurrentNonce(value)
        }
    }
    impl ::core::convert::From<GetDigestWithEIP712DomainCall> for FaucetHelperCalls {
        fn from(value: GetDigestWithEIP712DomainCall) -> Self {
            Self::GetDigestWithEIP712Domain(value)
        }
    }
    impl ::core::convert::From<GetProofStructHashCall> for FaucetHelperCalls {
        fn from(value: GetProofStructHashCall) -> Self {
            Self::GetProofStructHash(value)
        }
    }
    ///Container type for all return fields from the `EIP712_DOMAIN_TYPEHASH` function with signature `EIP712_DOMAIN_TYPEHASH()` and selector `0xc7977be7`
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
    pub struct Eip712DomainTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PROOF_TYPEHASH` function with signature `PROOF_TYPEHASH()` and selector `0x8b3e3bf6`
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
    pub struct ProofTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `consumeNonce` function with signature `consumeNonce()` and selector `0x34b15118`
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
    pub struct ConsumeNonceReturn(pub [u8; 32]);
    ///Container type for all return fields from the `currentNonce` function with signature `currentNonce()` and selector `0xadb610a3`
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
    pub struct CurrentNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getDigestWithEIP712Domain` function with signature `getDigestWithEIP712Domain((address,bytes32,bytes32),bytes,bytes,uint256,address)` and selector `0xb163a6b8`
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
    pub struct GetDigestWithEIP712DomainReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getProofStructHash` function with signature `getProofStructHash((address,bytes32,bytes32))` and selector `0x796dfbfa`
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
    pub struct GetProofStructHashReturn(pub [u8; 32]);
    ///`Proof(address,bytes32,bytes32)`
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
    pub struct Proof {
        pub recipient: ::ethers::core::types::Address,
        pub nonce: [u8; 32],
        pub id: [u8; 32],
    }
}

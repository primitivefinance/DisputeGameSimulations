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
pub mod relay_actor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_op"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract OptimismPortal"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_xdm"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract L1CrossDomainMessenger",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vm"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Vm"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_doFail"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("hashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashes"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("numHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numHashes"),
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
                    ::std::borrow::ToOwned::to_owned("relay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
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
                    ::std::borrow::ToOwned::to_owned("reverted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reverted"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RELAYACTOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x02\x80T`\xFF\x19\x16\x90U4\x80\x15b\0\0hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x1A\x868\x03\x80b\0\x1A\x86\x839\x81\x01`@\x81\x90Rb\0\0\x8B\x91b\0\x01\x07V[`\x02\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x02\x17\x90U`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x85\x16\x93\x90\x93\x17\x90\x92U`\x04\x80T\x91\x90\x93\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1B\x91\x15\x15\x91\x90\x91\x02\x17\x90Ub\0\x01\xBEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x04W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x84Qb\0\x01v\x81b\0\0\xEEV[` \x86\x01Q\x90\x94Pb\0\x01\x89\x81b\0\0\xEEV[`@\x86\x01Q\x90\x93Pb\0\x01\x9C\x81b\0\0\xEEV[``\x86\x01Q\x90\x92P\x80\x15\x15\x81\x14b\0\x01\xB3W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[a\x18\xB8\x80b\0\x01\xCE`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c4\x8Aq\xCB\x14a\x01UW\x80cP\x18\x95\xAE\x14a\x01qW\x80c\xBC\n\xF8*\x14a\x01\x84W\x80c\xE8(\xA0\xCE\x14a\x01\xA1W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01^`\0T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01^a\x01\x7F6`\x04a\x12NV[a\x01\xB6V[`\x02Ta\x01\x91\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01hV[a\x01\xB4a\x01\xAF6`\x04a\x12\xBBV[a\x01\xD7V[\0[`\x01\x81\x81T\x81\x10a\x01\xC6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[\x80Q`\x04\x90sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\x90`\0\x90` \x90a\x02\x03\x90`\x1Fa\x15VV[a\x02\r\x91\x90a\x15\x9DV[a\x02\x18\x90`\x03a\x15\xB1V[a\x02#\x90`\x0Fa\x15VV[`\x04T`\x02T`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16` \x83\x01R\x94\x95P\x92\x84\x16\x93cp\xCA\x10\xBB\x93a\x01\0\x90\x93\x04\x16\x91`2\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x02{\x90a\x15\xEEV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03\x84W=`\0\x80>=`\0\xFD[PPPP`\x02\x86a\x03\x95\x91\x90a\x163V[\x95Pa\x03\xA2`\x02\x86a\x163V[`\x04T\x90\x95P`\0\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x05\x03W`\x03T`@Q\x7F\xB2\x8A\xDE%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xB2\x8A\xDE%\x90a\x04\"\x90\x88\x90\x86\x90`\x04\x01a\x16\xCFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF4\x91\x90a\x16\xF7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\x19V[a\x05\x19\x82c\xFF\xFF\xFF\xFF\x16a\xEA`b\x018\x80a\rvV[\x90P`\0a\x05Y~\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF0\x8A\x90\x1B\x16\x85\x87\x8A`\xFF\x16\x87c\xFF\xFF\xFF\xFF\x16\x8Ba\r\xCBV[`\x01\x80T\x80\x82\x01\x82U`\0\x82\x81R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90\x91\x01\x83\x90U\x80T\x92\x93P\x90\x91\x81\x90a\x05\xA2\x90\x84\x90a\x15VV[\x90\x91UPP`\x04\x80T`\x03T`@Q\x7F\xB1\xB1\xB2\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x83\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92cLc\xE5b\x92\x90\x91\x16\x90c\xB1\xB1\xB2\t\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a\x17$V[\x15\x80\x15a\x08\x06WP`\x03T`@Q\x7F\xA4\xE7\xF8\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xA4\xE7\xF8\xBD\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x04\x91\x90a\x17$V[\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08&\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xD4W=`\0\x80>=`\0\xFD[PP`\x04\x80T`\x02T`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\0\x90\x92\x04\x82\x16\x93\x81\x01\x93\x90\x93R\x16\x92Pc\x06D}V\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t\xE4W=`\0\x80>=`\0\xFD[PP`\x04Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x91Pa\x0B\x1C\x90PW`\x04\x80T`@Q\x7F\x08\xE4\xE1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91c\x08\xE4\xE1\x16\x91a\ng\x91\x90\x8B\x90\x88\x90\x8C\x90\x84\x01a\x17IV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\x17W=`\0\x80>=`\0\xFD[PPPP[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD7d\xAD\x0B\x83`\xFF\x8A\x16~\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF0\x8D\x90\x1B\x16\x88\x8A\x8D\x8A\x8E`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x8B\x96\x95\x94\x93\x92\x91\x90a\x17\x97V[`\0`@Q\x80\x83\x03\x81\x85\x89\x80;\x15\x80\x15a\x0C&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P\x88\xF1\x94PPPPP\x80\x15a\x0C9WP`\x01[a\x0CiW`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[`\x04\x80T`@\x80Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92c\x90\xC5\x01;\x92\x82\x82\x01\x92`\0\x92\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\rTW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rhW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0a\r\x83\x84\x84\x84a\r\xEEV[\x90Pa\r\xC4`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x10*V[\x93\x92PPPV[`\0a\r\xDB\x87\x87\x87\x87\x87\x87a\x11*V[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[`\0\x81\x83\x11\x15a\x0E\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0E\x94WP\x81\x84\x11\x15[\x15a\x0E\xA0WP\x82a\r\xC4V[`\0a\x0E\xAC\x84\x84a\x17\xF7V[a\x0E\xB7\x90`\x01a\x15VV[\x90P`\x03\x85\x11\x15\x80\x15a\x0E\xC9WP\x84\x81\x11[\x15a\x0E\xE0Wa\x0E\xD8\x85\x85a\x15VV[\x91PPa\r\xC4V[a\x0F\x0B`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x17\xF7V[\x85\x10\x15\x80\x15a\x0FBWPa\x0F?\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x17\xF7V[\x81\x11[\x15a\x0F{Wa\x0Fq\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x17\xF7V[a\x0E\xD8\x90\x84a\x17\xF7V[\x82\x85\x11\x15a\x0F\xD1W`\0a\x0F\x8F\x84\x87a\x17\xF7V[\x90P`\0a\x0F\x9D\x83\x83a\x18\x0EV[\x90P\x80`\0\x03a\x0F\xB2W\x84\x93PPPPa\r\xC4V[`\x01a\x0F\xBE\x82\x88a\x15VV[a\x0F\xC8\x91\x90a\x17\xF7V[\x93PPPa\x10\"V[\x83\x85\x10\x15a\x10\"W`\0a\x0F\xE5\x86\x86a\x17\xF7V[\x90P`\0a\x0F\xF3\x83\x83a\x18\x0EV[\x90P\x80`\0\x03a\x10\x08W\x85\x93PPPPa\r\xC4V[a\x10\x12\x81\x86a\x17\xF7V[a\x10\x1D\x90`\x01a\x15VV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x10a\x92\x91\x90a\x18\"V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x10\xE2\x91\x90a\x18DV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x11\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\"V[``\x91P[PPPPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a\x11G\x96\x95\x94\x93\x92\x91\x90a\x18`V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x12cWa\x12ca\x11\xC9V[P5\x91\x90PV[\x805`\xFF\x81\x16\x81\x14a\x12{W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xD3Wa\x12\xD3a\x11\xC9V[a\x12\xDC\x84a\x12jV[\x92P` a\x12\xEB\x81\x86\x01a\x12jV[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\x87W\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x14\x19W\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x14+Wa\x14+a\x12\x80V[\x83Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x14pWa\x14pa\x12\x80V[\x81\x86R\x82\x81R\x8B\x87\x84\x87\x01\x01\x11\x15a\x15\x07W\x85Q\x93P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x86`\x04\x85\x01R`'`$\x85\x01R\x7FABI decoding: invalid byte array`D\x85\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x85\x01R`\x84\x84\xFD[a\x15\x16\x83\x88\x83\x01\x89\x88\x01a\x12\xAFV[\x80\x97PPPPPPPP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x15iWa\x15ia\x15'V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x15\xACWa\x15\xACa\x15nV[P\x04\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x15\xE9Wa\x15\xE9a\x15'V[P\x02\x90V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x16-W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\0`\xFF\x83\x16\x80a\x16FWa\x16Fa\x15nV[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0[\x83\x81\x10\x15a\x16pW\x81\x81\x01Q\x83\x82\x01R` \x01a\x16XV[\x83\x81\x11\x15a\x16\x7FW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x16\x9D\x81` \x86\x01` \x86\x01a\x16UV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16\xE2`@\x83\x01\x85a\x16\x85V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17\x0CWa\x17\x0Ca\x11\xC9V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xC4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x179Wa\x179a\x11\xC9V[\x81Q\x80\x15\x15\x81\x14a\r\xC4W`\0\x80\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R`\xFF\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R`\0a\x17\x8D`\x80\x83\x01\x84a\x16\x85V[\x96\x95PPPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP`\xFF\x85\x16``\x83\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x17\xEB`\xC0\x83\x01\x84a\x16\x85V[\x98\x97PPPPPPPPV[`\0\x82\x82\x10\x15a\x18\tWa\x18\ta\x15'V[P\x03\x90V[`\0\x82a\x18\x1DWa\x18\x1Da\x15nV[P\x06\x90V[`@\x81R`\0a\x185`@\x83\x01\x85a\x16\x85V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82Qa\x18V\x81\x84` \x87\x01a\x16UV[\x91\x90\x91\x01\x92\x91PPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x17\xEB`\xC0\x83\x01\x84a\x16\x85V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static RELAYACTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c4\x8Aq\xCB\x14a\x01UW\x80cP\x18\x95\xAE\x14a\x01qW\x80c\xBC\n\xF8*\x14a\x01\x84W\x80c\xE8(\xA0\xCE\x14a\x01\xA1W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01^`\0T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01^a\x01\x7F6`\x04a\x12NV[a\x01\xB6V[`\x02Ta\x01\x91\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01hV[a\x01\xB4a\x01\xAF6`\x04a\x12\xBBV[a\x01\xD7V[\0[`\x01\x81\x81T\x81\x10a\x01\xC6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[\x80Q`\x04\x90sB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\x90`\0\x90` \x90a\x02\x03\x90`\x1Fa\x15VV[a\x02\r\x91\x90a\x15\x9DV[a\x02\x18\x90`\x03a\x15\xB1V[a\x02#\x90`\x0Fa\x15VV[`\x04T`\x02T`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16` \x83\x01R\x94\x95P\x92\x84\x16\x93cp\xCA\x10\xBB\x93a\x01\0\x90\x93\x04\x16\x91`2\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x02{\x90a\x15\xEEV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x03\x84W=`\0\x80>=`\0\xFD[PPPP`\x02\x86a\x03\x95\x91\x90a\x163V[\x95Pa\x03\xA2`\x02\x86a\x163V[`\x04T\x90\x95P`\0\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x05\x03W`\x03T`@Q\x7F\xB2\x8A\xDE%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xB2\x8A\xDE%\x90a\x04\"\x90\x88\x90\x86\x90`\x04\x01a\x16\xCFV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF4\x91\x90a\x16\xF7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\x19V[a\x05\x19\x82c\xFF\xFF\xFF\xFF\x16a\xEA`b\x018\x80a\rvV[\x90P`\0a\x05Y~\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF0\x8A\x90\x1B\x16\x85\x87\x8A`\xFF\x16\x87c\xFF\xFF\xFF\xFF\x16\x8Ba\r\xCBV[`\x01\x80T\x80\x82\x01\x82U`\0\x82\x81R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90\x91\x01\x83\x90U\x80T\x92\x93P\x90\x91\x81\x90a\x05\xA2\x90\x84\x90a\x15VV[\x90\x91UPP`\x04\x80T`\x03T`@Q\x7F\xB1\xB1\xB2\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x83\x01\x84\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x92cLc\xE5b\x92\x90\x91\x16\x90c\xB1\xB1\xB2\t\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a\x17$V[\x15\x80\x15a\x08\x06WP`\x03T`@Q\x7F\xA4\xE7\xF8\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xA4\xE7\xF8\xBD\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x04\x91\x90a\x17$V[\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08&\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xD4W=`\0\x80>=`\0\xFD[PP`\x04\x80T`\x02T`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\0\x90\x92\x04\x82\x16\x93\x81\x01\x93\x90\x93R\x16\x92Pc\x06D}V\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\t\xE4W=`\0\x80>=`\0\xFD[PP`\x04Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x91Pa\x0B\x1C\x90PW`\x04\x80T`@Q\x7F\x08\xE4\xE1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91c\x08\xE4\xE1\x16\x91a\ng\x91\x90\x8B\x90\x88\x90\x8C\x90\x84\x01a\x17IV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\x17W=`\0\x80>=`\0\xFD[PPPP[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD7d\xAD\x0B\x83`\xFF\x8A\x16~\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF0\x8D\x90\x1B\x16\x88\x8A\x8D\x8A\x8E`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x8B\x96\x95\x94\x93\x92\x91\x90a\x17\x97V[`\0`@Q\x80\x83\x03\x81\x85\x89\x80;\x15\x80\x15a\x0C&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P\x88\xF1\x94PPPPP\x80\x15a\x0C9WP`\x01[a\x0CiW`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[`\x04\x80T`@\x80Q\x7F\x90\xC5\x01;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92c\x90\xC5\x01;\x92\x82\x82\x01\x92`\0\x92\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\rTW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\rhW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0a\r\x83\x84\x84\x84a\r\xEEV[\x90Pa\r\xC4`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x10*V[\x93\x92PPPV[`\0a\r\xDB\x87\x87\x87\x87\x87\x87a\x11*V[\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[`\0\x81\x83\x11\x15a\x0E\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0E\x94WP\x81\x84\x11\x15[\x15a\x0E\xA0WP\x82a\r\xC4V[`\0a\x0E\xAC\x84\x84a\x17\xF7V[a\x0E\xB7\x90`\x01a\x15VV[\x90P`\x03\x85\x11\x15\x80\x15a\x0E\xC9WP\x84\x81\x11[\x15a\x0E\xE0Wa\x0E\xD8\x85\x85a\x15VV[\x91PPa\r\xC4V[a\x0F\x0B`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x17\xF7V[\x85\x10\x15\x80\x15a\x0FBWPa\x0F?\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x17\xF7V[\x81\x11[\x15a\x0F{Wa\x0Fq\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x17\xF7V[a\x0E\xD8\x90\x84a\x17\xF7V[\x82\x85\x11\x15a\x0F\xD1W`\0a\x0F\x8F\x84\x87a\x17\xF7V[\x90P`\0a\x0F\x9D\x83\x83a\x18\x0EV[\x90P\x80`\0\x03a\x0F\xB2W\x84\x93PPPPa\r\xC4V[`\x01a\x0F\xBE\x82\x88a\x15VV[a\x0F\xC8\x91\x90a\x17\xF7V[\x93PPPa\x10\"V[\x83\x85\x10\x15a\x10\"W`\0a\x0F\xE5\x86\x86a\x17\xF7V[\x90P`\0a\x0F\xF3\x83\x83a\x18\x0EV[\x90P\x80`\0\x03a\x10\x08W\x85\x93PPPPa\r\xC4V[a\x10\x12\x81\x86a\x17\xF7V[a\x10\x1D\x90`\x01a\x15VV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x10a\x92\x91\x90a\x18\"V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x10\xE2\x91\x90a\x18DV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x11\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\"V[``\x91P[PPPPPPV[``\x86\x86\x86\x86\x86\x86`@Q`$\x01a\x11G\x96\x95\x94\x93\x92\x91\x90a\x18`V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD7d\xAD\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90P\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x12cWa\x12ca\x11\xC9V[P5\x91\x90PV[\x805`\xFF\x81\x16\x81\x14a\x12{W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xD3Wa\x12\xD3a\x11\xC9V[a\x12\xDC\x84a\x12jV[\x92P` a\x12\xEB\x81\x86\x01a\x12jV[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\x87W\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x14\x19W\x82Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x14+Wa\x14+a\x12\x80V[\x83Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x14pWa\x14pa\x12\x80V[\x81\x86R\x82\x81R\x8B\x87\x84\x87\x01\x01\x11\x15a\x15\x07W\x85Q\x93P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x86`\x04\x85\x01R`'`$\x85\x01R\x7FABI decoding: invalid byte array`D\x85\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x85\x01R`\x84\x84\xFD[a\x15\x16\x83\x88\x83\x01\x89\x88\x01a\x12\xAFV[\x80\x97PPPPPPPP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x15iWa\x15ia\x15'V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x15\xACWa\x15\xACa\x15nV[P\x04\x90V[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x15\xE9Wa\x15\xE9a\x15'V[P\x02\x90V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x16-W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\0`\xFF\x83\x16\x80a\x16FWa\x16Fa\x15nV[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0[\x83\x81\x10\x15a\x16pW\x81\x81\x01Q\x83\x82\x01R` \x01a\x16XV[\x83\x81\x11\x15a\x16\x7FW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x16\x9D\x81` \x86\x01` \x86\x01a\x16UV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16\xE2`@\x83\x01\x85a\x16\x85V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17\x0CWa\x17\x0Ca\x11\xC9V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xC4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x179Wa\x179a\x11\xC9V[\x81Q\x80\x15\x15\x81\x14a\r\xC4W`\0\x80\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R`\xFF\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R`\0a\x17\x8D`\x80\x83\x01\x84a\x16\x85V[\x96\x95PPPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP`\xFF\x85\x16``\x83\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x17\xEB`\xC0\x83\x01\x84a\x16\x85V[\x98\x97PPPPPPPPV[`\0\x82\x82\x10\x15a\x18\tWa\x18\ta\x15'V[P\x03\x90V[`\0\x82a\x18\x1DWa\x18\x1Da\x15nV[P\x06\x90V[`@\x81R`\0a\x185`@\x83\x01\x85a\x16\x85V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x82Qa\x18V\x81\x84` \x87\x01a\x16UV[\x91\x90\x91\x01\x92\x91PPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x17\xEB`\xC0\x83\x01\x84a\x16\x85V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static RELAYACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RelayActor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RelayActor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RelayActor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RelayActor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RelayActor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RelayActor)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RelayActor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RELAYACTOR_ABI.clone(),
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
                RELAYACTOR_ABI.clone(),
                RELAYACTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `hashes` (0x501895ae) function
        pub fn hashes(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([80, 24, 149, 174], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numHashes` (0x348a71cb) function
        pub fn num_hashes(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 138, 113, 203], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relay` (0xe828a0ce) function
        pub fn relay(
            &self,
            version: u8,
            value: u8,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 40, 160, 206], (version, value, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reverted` (0xbc0af82a) function
        pub fn reverted(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([188, 10, 248, 42], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RelayActor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `hashes` function with signature `hashes(uint256)` and selector `0x501895ae`
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
    #[ethcall(name = "hashes", abi = "hashes(uint256)")]
    pub struct HashesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `numHashes` function with signature `numHashes()` and selector `0x348a71cb`
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
    #[ethcall(name = "numHashes", abi = "numHashes()")]
    pub struct NumHashesCall;
    ///Container type for all input parameters for the `relay` function with signature `relay(uint8,uint8,bytes)` and selector `0xe828a0ce`
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
    #[ethcall(name = "relay", abi = "relay(uint8,uint8,bytes)")]
    pub struct RelayCall {
        pub version: u8,
        pub value: u8,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `reverted` function with signature `reverted()` and selector `0xbc0af82a`
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
    #[ethcall(name = "reverted", abi = "reverted()")]
    pub struct RevertedCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RelayActorCalls {
        Hashes(HashesCall),
        NumHashes(NumHashesCall),
        Relay(RelayCall),
        Reverted(RevertedCall),
    }
    impl ::ethers::core::abi::AbiDecode for RelayActorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Hashes(decoded));
            }
            if let Ok(decoded) = <NumHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumHashes(decoded));
            }
            if let Ok(decoded) = <RelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Relay(decoded));
            }
            if let Ok(decoded) = <RevertedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reverted(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RelayActorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Hashes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Relay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Reverted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RelayActorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Hashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Relay(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reverted(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HashesCall> for RelayActorCalls {
        fn from(value: HashesCall) -> Self {
            Self::Hashes(value)
        }
    }
    impl ::core::convert::From<NumHashesCall> for RelayActorCalls {
        fn from(value: NumHashesCall) -> Self {
            Self::NumHashes(value)
        }
    }
    impl ::core::convert::From<RelayCall> for RelayActorCalls {
        fn from(value: RelayCall) -> Self {
            Self::Relay(value)
        }
    }
    impl ::core::convert::From<RevertedCall> for RelayActorCalls {
        fn from(value: RevertedCall) -> Self {
            Self::Reverted(value)
        }
    }
    ///Container type for all return fields from the `hashes` function with signature `hashes(uint256)` and selector `0x501895ae`
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
    pub struct HashesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `numHashes` function with signature `numHashes()` and selector `0x348a71cb`
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
    pub struct NumHashesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reverted` function with signature `reverted()` and selector `0xbc0af82a`
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
    pub struct RevertedReturn(pub bool);
}

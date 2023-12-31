pub use optimist_inviter_helper::*;
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
pub mod optimist_inviter_helper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_optimistInviter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract OptimistInviter"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CLAIMABLE_INVITE_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CLAIMABLE_INVITE_TYPEHASH",
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
                    ::std::borrow::ToOwned::to_owned("getClaimableInviteStructHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getClaimableInviteStructHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_claimableInvite"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OptimistInviter.ClaimableInvite",
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
                (
                    ::std::borrow::ToOwned::to_owned("getClaimableInviteWithNewNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getClaimableInviteWithNewNonce",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_issuer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OptimistInviter.ClaimableInvite",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDigest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDigest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_claimableInvite"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OptimistInviter.ClaimableInvite",
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
                                    name: ::std::borrow::ToOwned::to_owned("_claimableInvite"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OptimistInviter.ClaimableInvite",
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
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
                    ::std::borrow::ToOwned::to_owned("optimistInviter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("optimistInviter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract OptimistInviter"),
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
    pub static OPTIMISTINVITERHELPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\r18\x03\x80b\0\r1\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0|V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\x01b\0\0]\x82\x82b\0\x02\x0BV[PPPb\0\x02\xD7V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\x90W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xA8W`\0\x80\xFD[` \x84\x81\x01Q\x91\x93P\x90`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\xC8W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12b\0\0\xDDW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\0\xF2Wb\0\0\xF2b\0\0fV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x01\x1DWb\0\x01\x1Db\0\0fV[\x81`@R\x82\x81R\x89\x86\x84\x87\x01\x01\x11\x15b\0\x016W`\0\x80\xFD[`\0\x93P[\x82\x84\x10\x15b\0\x01ZW\x84\x84\x01\x86\x01Q\x81\x85\x01\x87\x01R\x92\x85\x01\x92b\0\x01;V[\x82\x84\x11\x15b\0\x01lW`\0\x86\x84\x83\x01\x01R[\x80\x96PPPPPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01\x91W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01\xB2WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\x06W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\xE1WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\x02W\x82\x81U`\x01\x01b\0\x01\xEDV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02'Wb\0\x02'b\0\0fV[b\0\x02?\x81b\0\x028\x84Tb\0\x01|V[\x84b\0\x01\xB8V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02wW`\0\x84\x15b\0\x02^WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\x02V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02\xA8W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x02\x87V[P\x85\x82\x10\x15b\0\x02\xC7W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\nJ\x80b\0\x02\xE7`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80c\xAD\xB6\x10\xA3\x11a\0vW\x80c\xC7\x97{\xE7\x11a\0[W\x80c\xC7\x97{\xE7\x14a\x01\x8DW\x80c\xD1\x82\x99\x07\x14a\x01\xB4W\x80c\xEB\x1D\xF6&\x14a\x01\xF9W`\0\x80\xFD[\x80c\xAD\xB6\x10\xA3\x14a\x01]W\x80c\xC4\xFCE=\x14a\x01fW`\0\x80\xFD[\x80c\x1Cn\x7F%\x11a\0\xA7W\x80c\x1Cn\x7F%\x14a\0\xFEW\x80c4\xB1Q\x18\x14a\x01\x11W\x80c\x8E\xBE$\xE5\x14a\x01\x19W`\0\x80\xFD[\x80c\x01\x9AOI\x14a\0\xC3W\x80c\x06\xFD\xDE\x03\x14a\0\xE9W[`\0\x80\xFD[a\0\xD6a\0\xD16`\x04a\x07tV[a\x02\x0CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF1a\x03\x15V[`@Qa\0\xE0\x91\x90a\x084V[a\0\xD6a\x01\x0C6`\x04a\x08\x85V[a\x03\xA3V[a\0\xD6a\x05\x0CV[a\x01,a\x01'6`\x04a\x08\xA8V[a\x05NV[`@\x80Q\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xE0V[a\0\xD6`\x02T\x81V[a\0\xD6\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCE\x81V[a\0\xD6\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81V[`\0Ta\x01\xD4\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE0V[a\0\xD6a\x02\x076`\x04a\x08\xC3V[a\x05\x99V[\x83Q` \x80\x86\x01\x91\x90\x91 \x84Q\x85\x83\x01 `@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x94\x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\xA0\x82\x01R`\0\x90\x81\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x03\n\x81a\x02\xAF\x89\x806\x03\x81\x01\x90a\x02\x07\x91\x90a\x08\xC3V[`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x97\x96PPPPPPPV[`\x01\x80Ta\x03\"\x90a\t\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03N\x90a\t\x1AV[\x80\x15a\x03\x9BW\x80`\x1F\x10a\x03pWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x9BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03~W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x05\x06\x82`\x01\x80Ta\x03\xB6\x90a\t\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xE2\x90a\t\x1AV[\x80\x15a\x04/W\x80`\x1F\x10a\x04\x04Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04/V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x12W\x82\x90\x03`\x1F\x16\x82\x01\x91[PP`\0\x80T`@\x80Q\x7F\xEC\xCE\xC5\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x95Pc\xEC\xCE\xC5\xA8\x94P`\x04\x80\x82\x01\x94P\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04\xE6\x91\x90\x81\x01\x90a\tgV[`\0TF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\x0CV[\x92\x91PPV[`\x02\x80T`\0\x91\x82a\x05\x1D\x83a\t\xDEV[\x91\x90PU`@Q` \x01a\x053\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@Q\x80`@\x01`@R\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x05\x91a\x05\x0CV[\x90R\x92\x91PPV[\x80Q` \x80\x83\x01Q`@Q`\0\x93a\x05\xFC\x93\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCE\x93\x91\x92\x01\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x06+W`\0\x80\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\xA7Wa\x06\xA7a\x061V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\xC9Wa\x06\xC9a\x061V[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x06W`\0\x80\xFD[\x815a\x07\x19a\x07\x14\x82a\x06\xAFV[a\x06`V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07.W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07oW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xC0\x86\x88\x03\x12\x15a\x07\x8CW`\0\x80\xFD[a\x07\x96\x87\x87a\x06\x19V[\x94P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB3W`\0\x80\xFD[a\x07\xBF\x89\x83\x8A\x01a\x06\xF5V[\x95P``\x88\x015\x91P\x80\x82\x11\x15a\x07\xD5W`\0\x80\xFD[Pa\x07\xE2\x88\x82\x89\x01a\x06\xF5V[\x93PP`\x80\x86\x015\x91Pa\x07\xF8`\xA0\x87\x01a\x07KV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0[\x83\x81\x10\x15a\x08\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\x07V[\x83\x81\x11\x15a\x08.W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x08S\x81`@\x85\x01` \x87\x01a\x08\x04V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x08\x97W`\0\x80\xFD[a\x08\xA1\x83\x83a\x06\x19V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xBAW`\0\x80\xFD[a\x08\xA1\x82a\x07KV[`\0`@\x82\x84\x03\x12\x15a\x08\xD5W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x08\xF8Wa\x08\xF8a\x061V[`@Ra\t\x04\x83a\x07KV[\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\t.W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06+W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\tyW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x90W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\t\xA1W`\0\x80\xFD[\x80Qa\t\xAFa\x07\x14\x82a\x06\xAFV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\t\xC4W`\0\x80\xFD[a\t\xD5\x82` \x83\x01` \x86\x01a\x08\x04V[\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\n6W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static OPTIMISTINVITERHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80c\xAD\xB6\x10\xA3\x11a\0vW\x80c\xC7\x97{\xE7\x11a\0[W\x80c\xC7\x97{\xE7\x14a\x01\x8DW\x80c\xD1\x82\x99\x07\x14a\x01\xB4W\x80c\xEB\x1D\xF6&\x14a\x01\xF9W`\0\x80\xFD[\x80c\xAD\xB6\x10\xA3\x14a\x01]W\x80c\xC4\xFCE=\x14a\x01fW`\0\x80\xFD[\x80c\x1Cn\x7F%\x11a\0\xA7W\x80c\x1Cn\x7F%\x14a\0\xFEW\x80c4\xB1Q\x18\x14a\x01\x11W\x80c\x8E\xBE$\xE5\x14a\x01\x19W`\0\x80\xFD[\x80c\x01\x9AOI\x14a\0\xC3W\x80c\x06\xFD\xDE\x03\x14a\0\xE9W[`\0\x80\xFD[a\0\xD6a\0\xD16`\x04a\x07tV[a\x02\x0CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF1a\x03\x15V[`@Qa\0\xE0\x91\x90a\x084V[a\0\xD6a\x01\x0C6`\x04a\x08\x85V[a\x03\xA3V[a\0\xD6a\x05\x0CV[a\x01,a\x01'6`\x04a\x08\xA8V[a\x05NV[`@\x80Q\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xE0V[a\0\xD6`\x02T\x81V[a\0\xD6\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCE\x81V[a\0\xD6\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81V[`\0Ta\x01\xD4\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xE0V[a\0\xD6a\x02\x076`\x04a\x08\xC3V[a\x05\x99V[\x83Q` \x80\x86\x01\x91\x90\x91 \x84Q\x85\x83\x01 `@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x94\x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\xA0\x82\x01R`\0\x90\x81\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x03\n\x81a\x02\xAF\x89\x806\x03\x81\x01\x90a\x02\x07\x91\x90a\x08\xC3V[`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x97\x96PPPPPPPV[`\x01\x80Ta\x03\"\x90a\t\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03N\x90a\t\x1AV[\x80\x15a\x03\x9BW\x80`\x1F\x10a\x03pWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x9BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03~W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0a\x05\x06\x82`\x01\x80Ta\x03\xB6\x90a\t\x1AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xE2\x90a\t\x1AV[\x80\x15a\x04/W\x80`\x1F\x10a\x04\x04Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04/V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x12W\x82\x90\x03`\x1F\x16\x82\x01\x91[PP`\0\x80T`@\x80Q\x7F\xEC\xCE\xC5\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x95Pc\xEC\xCE\xC5\xA8\x94P`\x04\x80\x82\x01\x94P\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04\xE6\x91\x90\x81\x01\x90a\tgV[`\0TF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\x0CV[\x92\x91PPV[`\x02\x80T`\0\x91\x82a\x05\x1D\x83a\t\xDEV[\x91\x90PU`@Q` \x01a\x053\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@Q\x80`@\x01`@R\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x05\x91a\x05\x0CV[\x90R\x92\x91PPV[\x80Q` \x80\x83\x01Q`@Q`\0\x93a\x05\xFC\x93\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCE\x93\x91\x92\x01\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x06+W`\0\x80\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\xA7Wa\x06\xA7a\x061V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\xC9Wa\x06\xC9a\x061V[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x06W`\0\x80\xFD[\x815a\x07\x19a\x07\x14\x82a\x06\xAFV[a\x06`V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07.W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07oW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xC0\x86\x88\x03\x12\x15a\x07\x8CW`\0\x80\xFD[a\x07\x96\x87\x87a\x06\x19V[\x94P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB3W`\0\x80\xFD[a\x07\xBF\x89\x83\x8A\x01a\x06\xF5V[\x95P``\x88\x015\x91P\x80\x82\x11\x15a\x07\xD5W`\0\x80\xFD[Pa\x07\xE2\x88\x82\x89\x01a\x06\xF5V[\x93PP`\x80\x86\x015\x91Pa\x07\xF8`\xA0\x87\x01a\x07KV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0[\x83\x81\x10\x15a\x08\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\x07V[\x83\x81\x11\x15a\x08.W`\0\x84\x84\x01R[PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x08S\x81`@\x85\x01` \x87\x01a\x08\x04V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x08\x97W`\0\x80\xFD[a\x08\xA1\x83\x83a\x06\x19V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\xBAW`\0\x80\xFD[a\x08\xA1\x82a\x07KV[`\0`@\x82\x84\x03\x12\x15a\x08\xD5W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x08\xF8Wa\x08\xF8a\x061V[`@Ra\t\x04\x83a\x07KV[\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\t.W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06+W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\tyW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x90W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\t\xA1W`\0\x80\xFD[\x80Qa\t\xAFa\x07\x14\x82a\x06\xAFV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\t\xC4W`\0\x80\xFD[a\t\xD5\x82` \x83\x01` \x86\x01a\x08\x04V[\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\n6W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static OPTIMISTINVITERHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OptimistInviterHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OptimistInviterHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OptimistInviterHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OptimistInviterHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OptimistInviterHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OptimistInviterHelper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OptimistInviterHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPTIMISTINVITERHELPER_ABI.clone(),
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
                OPTIMISTINVITERHELPER_ABI.clone(),
                OPTIMISTINVITERHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CLAIMABLE_INVITE_TYPEHASH` (0xc4fc453d) function
        pub fn claimable_invite_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([196, 252, 69, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EIP712_DOMAIN_TYPEHASH` (0xc7977be7) function
        pub fn eip712_domain_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([199, 151, 123, 231], ())
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
        ///Calls the contract's `getClaimableInviteStructHash` (0xeb1df626) function
        pub fn get_claimable_invite_struct_hash(
            &self,
            claimable_invite: ClaimableInvite,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([235, 29, 246, 38], (claimable_invite,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getClaimableInviteWithNewNonce` (0x8ebe24e5) function
        pub fn get_claimable_invite_with_new_nonce(
            &self,
            issuer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ClaimableInvite> {
            self.0
                .method_hash([142, 190, 36, 229], issuer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDigest` (0x1c6e7f25) function
        pub fn get_digest(
            &self,
            claimable_invite: ClaimableInvite,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([28, 110, 127, 37], (claimable_invite,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDigestWithEIP712Domain` (0x019a4f49) function
        pub fn get_digest_with_eip712_domain(
            &self,
            claimable_invite: ClaimableInvite,
            name: ::ethers::core::types::Bytes,
            version: ::ethers::core::types::Bytes,
            chainid: ::ethers::core::types::U256,
            verifying_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [1, 154, 79, 73],
                    (claimable_invite, name, version, chainid, verifying_contract),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `optimistInviter` (0xd1829907) function
        pub fn optimist_inviter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([209, 130, 153, 7], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OptimistInviterHelper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `CLAIMABLE_INVITE_TYPEHASH` function with signature `CLAIMABLE_INVITE_TYPEHASH()` and selector `0xc4fc453d`
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
    #[ethcall(name = "CLAIMABLE_INVITE_TYPEHASH", abi = "CLAIMABLE_INVITE_TYPEHASH()")]
    pub struct ClaimableInviteTypehashCall;
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
    ///Container type for all input parameters for the `getClaimableInviteStructHash` function with signature `getClaimableInviteStructHash((address,bytes32))` and selector `0xeb1df626`
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
        name = "getClaimableInviteStructHash",
        abi = "getClaimableInviteStructHash((address,bytes32))"
    )]
    pub struct GetClaimableInviteStructHashCall {
        pub claimable_invite: ClaimableInvite,
    }
    ///Container type for all input parameters for the `getClaimableInviteWithNewNonce` function with signature `getClaimableInviteWithNewNonce(address)` and selector `0x8ebe24e5`
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
        name = "getClaimableInviteWithNewNonce",
        abi = "getClaimableInviteWithNewNonce(address)"
    )]
    pub struct GetClaimableInviteWithNewNonceCall {
        pub issuer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDigest` function with signature `getDigest((address,bytes32))` and selector `0x1c6e7f25`
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
    #[ethcall(name = "getDigest", abi = "getDigest((address,bytes32))")]
    pub struct GetDigestCall {
        pub claimable_invite: ClaimableInvite,
    }
    ///Container type for all input parameters for the `getDigestWithEIP712Domain` function with signature `getDigestWithEIP712Domain((address,bytes32),bytes,bytes,uint256,address)` and selector `0x019a4f49`
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
        abi = "getDigestWithEIP712Domain((address,bytes32),bytes,bytes,uint256,address)"
    )]
    pub struct GetDigestWithEIP712DomainCall {
        pub claimable_invite: ClaimableInvite,
        pub name: ::ethers::core::types::Bytes,
        pub version: ::ethers::core::types::Bytes,
        pub chainid: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `optimistInviter` function with signature `optimistInviter()` and selector `0xd1829907`
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
    #[ethcall(name = "optimistInviter", abi = "optimistInviter()")]
    pub struct OptimistInviterCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OptimistInviterHelperCalls {
        ClaimableInviteTypehash(ClaimableInviteTypehashCall),
        Eip712DomainTypehash(Eip712DomainTypehashCall),
        ConsumeNonce(ConsumeNonceCall),
        CurrentNonce(CurrentNonceCall),
        GetClaimableInviteStructHash(GetClaimableInviteStructHashCall),
        GetClaimableInviteWithNewNonce(GetClaimableInviteWithNewNonceCall),
        GetDigest(GetDigestCall),
        GetDigestWithEIP712Domain(GetDigestWithEIP712DomainCall),
        Name(NameCall),
        OptimistInviter(OptimistInviterCall),
    }
    impl ::ethers::core::abi::AbiDecode for OptimistInviterHelperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClaimableInviteTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimableInviteTypehash(decoded));
            }
            if let Ok(decoded) = <Eip712DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eip712DomainTypehash(decoded));
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
            if let Ok(decoded) = <GetClaimableInviteStructHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetClaimableInviteStructHash(decoded));
            }
            if let Ok(decoded) = <GetClaimableInviteWithNewNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetClaimableInviteWithNewNonce(decoded));
            }
            if let Ok(decoded) = <GetDigestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDigest(decoded));
            }
            if let Ok(decoded) = <GetDigestWithEIP712DomainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDigestWithEIP712Domain(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OptimistInviterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimistInviter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OptimistInviterHelperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimableInviteTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712DomainTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConsumeNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetClaimableInviteStructHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetClaimableInviteWithNewNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDigest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDigestWithEIP712Domain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OptimistInviter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OptimistInviterHelperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimableInviteTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Eip712DomainTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConsumeNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClaimableInviteStructHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetClaimableInviteWithNewNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDigestWithEIP712Domain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OptimistInviter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimableInviteTypehashCall>
    for OptimistInviterHelperCalls {
        fn from(value: ClaimableInviteTypehashCall) -> Self {
            Self::ClaimableInviteTypehash(value)
        }
    }
    impl ::core::convert::From<Eip712DomainTypehashCall> for OptimistInviterHelperCalls {
        fn from(value: Eip712DomainTypehashCall) -> Self {
            Self::Eip712DomainTypehash(value)
        }
    }
    impl ::core::convert::From<ConsumeNonceCall> for OptimistInviterHelperCalls {
        fn from(value: ConsumeNonceCall) -> Self {
            Self::ConsumeNonce(value)
        }
    }
    impl ::core::convert::From<CurrentNonceCall> for OptimistInviterHelperCalls {
        fn from(value: CurrentNonceCall) -> Self {
            Self::CurrentNonce(value)
        }
    }
    impl ::core::convert::From<GetClaimableInviteStructHashCall>
    for OptimistInviterHelperCalls {
        fn from(value: GetClaimableInviteStructHashCall) -> Self {
            Self::GetClaimableInviteStructHash(value)
        }
    }
    impl ::core::convert::From<GetClaimableInviteWithNewNonceCall>
    for OptimistInviterHelperCalls {
        fn from(value: GetClaimableInviteWithNewNonceCall) -> Self {
            Self::GetClaimableInviteWithNewNonce(value)
        }
    }
    impl ::core::convert::From<GetDigestCall> for OptimistInviterHelperCalls {
        fn from(value: GetDigestCall) -> Self {
            Self::GetDigest(value)
        }
    }
    impl ::core::convert::From<GetDigestWithEIP712DomainCall>
    for OptimistInviterHelperCalls {
        fn from(value: GetDigestWithEIP712DomainCall) -> Self {
            Self::GetDigestWithEIP712Domain(value)
        }
    }
    impl ::core::convert::From<NameCall> for OptimistInviterHelperCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OptimistInviterCall> for OptimistInviterHelperCalls {
        fn from(value: OptimistInviterCall) -> Self {
            Self::OptimistInviter(value)
        }
    }
    ///Container type for all return fields from the `CLAIMABLE_INVITE_TYPEHASH` function with signature `CLAIMABLE_INVITE_TYPEHASH()` and selector `0xc4fc453d`
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
    pub struct ClaimableInviteTypehashReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `getClaimableInviteStructHash` function with signature `getClaimableInviteStructHash((address,bytes32))` and selector `0xeb1df626`
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
    pub struct GetClaimableInviteStructHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getClaimableInviteWithNewNonce` function with signature `getClaimableInviteWithNewNonce(address)` and selector `0x8ebe24e5`
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
    pub struct GetClaimableInviteWithNewNonceReturn(pub ClaimableInvite);
    ///Container type for all return fields from the `getDigest` function with signature `getDigest((address,bytes32))` and selector `0x1c6e7f25`
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
    pub struct GetDigestReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getDigestWithEIP712Domain` function with signature `getDigestWithEIP712Domain((address,bytes32),bytes,bytes,uint256,address)` and selector `0x019a4f49`
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
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `optimistInviter` function with signature `optimistInviter()` and selector `0xd1829907`
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
    pub struct OptimistInviterReturn(pub ::ethers::core::types::Address);
}

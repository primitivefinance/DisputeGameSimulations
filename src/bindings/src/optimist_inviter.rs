pub use optimist_inviter::*;
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
pub mod optimist_inviter {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_inviteGranter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_attestationStation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract AttestationStation",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_STATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ATTESTATION_STATION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AttestationStation",
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
                    ::std::borrow::ToOwned::to_owned("CAN_INVITE_ATTESTATION_KEY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CAN_INVITE_ATTESTATION_KEY",
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
                    ::std::borrow::ToOwned::to_owned("EIP712_VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EIP712_VERSION"),
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
                    ::std::borrow::ToOwned::to_owned("INVITE_GRANTER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVITE_GRANTER"),
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
                    ::std::borrow::ToOwned::to_owned("MIN_COMMITMENT_PERIOD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MIN_COMMITMENT_PERIOD",
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
                    ::std::borrow::ToOwned::to_owned("claimInvite"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimInvite"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_claimer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("_signature"),
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
                    ::std::borrow::ToOwned::to_owned("commitInvite"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("commitInvite"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_commitment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("commitmentTimestamps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "commitmentTimestamps",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("inviteCounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("inviteCounts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setInviteCounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setInviteCounts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_accounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_inviteCount"),
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
                    ::std::borrow::ToOwned::to_owned("usedNonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("usedNonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InviteClaimed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("InviteClaimed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("issuer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("claimer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OPTIMISTINVITER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1D\xE08\x03\x80b\0\x1D\xE0\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\0vV[`\x01`\x80R`\0`\xA0R`\x02`\xC0R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xE0R\x16a\x01\0Rb\0\0\xB5V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0sW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\x8AW`\0\x80\xFD[\x82Qb\0\0\x97\x81b\0\0]V[` \x84\x01Q\x90\x92Pb\0\0\xAA\x81b\0\0]V[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x1C\xD2b\0\x01\x0E`\09`\0\x81\x81a\x02W\x01R\x81\x81a\x06Q\x01Ra\x0C\x02\x01R`\0\x81\x81`\xF4\x01Ra\x03\xB4\x01R`\0a\r\xA1\x01R`\0a\rx\x01R`\0a\rO\x01Ra\x1C\xD2`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x91m\xB2/\x11a\0\x8CW\x80c\xDB\x08=q\x11a\0fW\x80c\xDB\x08=q\x14a\x02RW\x80c\xDE-\xD2!\x14a\x02yW\x80c\xEC\xCE\xC5\xA8\x14a\x02\x99W\x80c\xF6-\x18\x88\x14a\x02\xD5W`\0\x80\xFD[\x80c\x91m\xB2/\x14a\x01\xE4W\x80c\xB4$]s\x14a\x02\x0BW\x80c\xC4\xFCE=\x14a\x02+W`\0\x80\xFD[\x80cP\xB4\x14\xE6\x11a\0\xC8W\x80cP\xB4\x14\xE6\x14a\x01hW\x80cP\xEE\xDB\xC2\x14a\x01~W\x80cT\xFDMP\x14a\x01\x91W\x80c_\xDA\x04\xC7\x14a\x01\xA6W`\0\x80\xFD[\x80c\x14\xB4rA\x14a\0\xEFW\x80c\x18~<\xD1\x14a\x01@W\x80c%\xB2z=\x14a\x01UW[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Sa\x01N6`\x04a\x16_V[a\x02\xE8V[\0[a\x01Sa\x01c6`\x04a\x16xV[a\x03\x9CV[a\x01p`<\x81V[`@Q\x90\x81R` \x01a\x017V[a\x01Sa\x01\x8C6`\x04a\x17\xDFV[a\x06\xBFV[a\x01\x99a\rHV[`@Qa\x017\x91\x90a\x18\xF5V[a\x01\xD4a\x01\xB46`\x04a\x19\x08V[`6` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x017V[a\x01p\x7Foptimist.can-invite\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01pa\x02\x196`\x04a\x16_V[`5` R`\0\x90\x81R`@\x90 T\x81V[a\x01p\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCE\x81V[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01pa\x02\x876`\x04a\x192V[`7` R`\0\x90\x81R`@\x90 T\x81V[a\x01\x99`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01Sa\x02\xE36`\x04a\x19MV[a\r\xEBV[`\0\x81\x81R`5` R`@\x90 T\x15a\x03\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FOptimistInviter: commitment alre`D\x82\x01R\x7Fady made\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`5` R`@\x90 B\x90UV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04aW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FOptimistInviter: only invite gra`D\x82\x01R\x7Fnter can grant invites\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[\x81`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04}Wa\x04}a\x17\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xCAW\x81` \x01[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\x9BW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x06\x13W\x83`7`\0\x88\x88\x85\x81\x81\x10a\x04\xEFWa\x04\xEFa\x19\x96V[\x90P` \x02\x01` \x81\x01\x90a\x05\x04\x91\x90a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`@Q\x80``\x01`@R\x80\x87\x87\x84\x81\x81\x10a\x05_Wa\x05_a\x19\x96V[\x90P` \x02\x01` \x81\x01\x90a\x05t\x91\x90a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x7Foptimist.can-invite\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x01`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftrue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81RP\x82\x82\x81Q\x81\x10a\x06\0Wa\x06\0a\x19\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xD0V[P`@Q\x7F^\xB5\xEA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^\xB5\xEA\x10\x90a\x06\x86\x90\x84\x90`\x04\x01a\x19\xC5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0`5`\0\x85\x84`@Q` \x01a\x06\xD8\x92\x91\x90a\x1AxV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x11a\x07\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FOptimistInviter: claimer and sig`D\x82\x01R\x7Fnature have not been committed y`d\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\x80V[Ba\x07\xC1`<\x83a\x1A\xD6V[\x11\x15a\x08OW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FOptimistInviter: minimum commitm`D\x82\x01R\x7Fent period has not elapsed yet\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`\0a\x08\xD2\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCEa\x08\x82` \x87\x01\x87a\x192V[`@\x80Q` \x81\x81\x01\x94\x90\x94Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x90\x82\x01R\x90\x86\x015``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x0F\xB5V[\x90Pa\x08\xEBa\x08\xE4` \x86\x01\x86a\x192V[\x82\x85a\x10$V[a\twW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FOptimistInviter: invalid signatu`D\x82\x01R\x7Fre\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`6`\0a\t\x88` \x87\x01\x87a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x87\x84\x015\x82R\x90\x92R\x90 T`\xFF\x16\x15a\nOW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOptimistInviter: nonce has alrea`D\x82\x01R\x7Fdy been used\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`\x01`6`\0a\nb` \x88\x01\x88a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x88\x84\x01\x805\x83R\x93R\x90\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x93\x15\x15\x93\x90\x93\x17\x90\x92U`7\x90\x82\x90a\n\xD6\x90\x88a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x11a\x0B\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOptimistInviter: issuer has no i`D\x82\x01R\x7Fnvites\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`7`\0a\x0B\xAF` \x87\x01\x87a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x90\x91\x01`\0 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cp+\x9D\xEE\x90\x87\x90\x7Foptimist.can-mint-from-invite\0\0\0\x90a\x0CY\x90\x89\x01\x89a\x192V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x83\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xA8\x93\x92\x91\x90a\x1A\xEEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xC2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xD6W=`\0\x80>=`\0\xFD[PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90Pa\x0C\xFF` \x86\x01\x86a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Ft]<[\xC9*\xB4\x0BA\x80i\xBF\x8F\x8E 0\x80~\xFF\xCE\xB8\x8B\xBA\xA0~\xE0\x15t\xF1k\xE4u`@Q`@Q\x80\x91\x03\x90\xA3PPPPPV[``a\rs\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\xF3V[a\r\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\xF3V[a\r\xC5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\xF3V[`@Q` \x01a\r\xD7\x93\x92\x91\x90a\x1B,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0E\x0BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0E%WP0;\x15\x80\x15a\x0E%WP`\0T`\xFF\x16`\x01\x14[a\x0E\xB1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x0F\x0FW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x0FN\x82`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x130V[\x80\x15a\x0F\xB1W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[`\0a\x10\x1Ea\x0F\xC2a\x13\xD1V[\x83`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x92\x91PPV[`\0\x80`\0a\x103\x85\x85a\x14QV[\x90\x92P\x90P`\0\x81`\x04\x81\x11\x15a\x10LWa\x10La\x1B\xA2V[\x14\x80\x15a\x10\x84WP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x15a\x10\x94W`\x01\x92PPPa\x11\xECV[`\0\x80\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x10\xC9\x92\x91\x90a\x1B\xD1V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x11R\x91\x90a\x1B\xEAV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x11\x8DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\x92V[``\x91P[P\x91P\x91P\x81\x80\x15a\x11\xA5WP\x80Q` \x14[\x80\x15a\x11\xE5WP\x80Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x11\xE3\x90\x83\x01` \x90\x81\x01\x90\x84\x01a\x1C\x06V[\x14[\x94PPPPP[\x93\x92PPPV[``\x81`\0\x03a\x126WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x12`W\x80a\x12J\x81a\x1C\x1FV[\x91Pa\x12Y\x90P`\n\x83a\x1C\x86V[\x91Pa\x12:V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12{Wa\x12{a\x17\x1CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\xA5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x13(Wa\x12\xBA`\x01\x83a\x1C\x9AV[\x91Pa\x12\xC7`\n\x86a\x1C\xB1V[a\x12\xD2\x90`0a\x1A\xD6V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x12\xE7Wa\x12\xE7a\x19\x96V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x13!`\n\x86a\x1C\x86V[\x94Pa\x12\xA9V[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13\xC7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[a\x0F\xB1\x82\x82a\x14\x96V[`\0a\x14L\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x14\0`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[\x90P\x90V[`\0\x80\x82Q`A\x03a\x14\x87W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x14{\x87\x82\x85\x85a\x15GV[\x94P\x94PPPPa\x14\x8FV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x15-W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x15~WP`\0\x90P`\x03a\x16VV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x15\x96WP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x15\xA7WP`\0\x90P`\x04a\x16VV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xFBW=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x16OW`\0`\x01\x92P\x92PPa\x16VV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x16qW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x16\x8DW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xA5W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x16\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16\xC8W`\0\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\xDDW`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17\x17W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x17fWa\x17fa\x17\x1CV[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x17\xACWa\x17\xACa\x17\x1CV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x17\xC5W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`\0\x83\x85\x03`\x80\x81\x12\x15a\x17\xF5W`\0\x80\xFD[a\x17\xFE\x85a\x16\xF3V[\x93P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x01\x12\x15a\x180W`\0\x80\xFD[P` \x84\x01\x91P``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18QW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x18bW`\0\x80\xFD[a\x18q\x86\x825` \x84\x01a\x17KV[\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x18\x96W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18~V[\x83\x81\x11\x15a\x18\xA5W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x18\xC3\x81` \x86\x01` \x86\x01a\x18{V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x11\xEC` \x83\x01\x84a\x18\xABV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\x1BW`\0\x80\xFD[a\x19$\x83a\x16\xF3V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x19DW`\0\x80\xFD[a\x11\xEC\x82a\x16\xF3V[`\0` \x82\x84\x03\x12\x15a\x19_W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19vW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\x87W`\0\x80\xFD[a\x13(\x84\x825` \x84\x01a\x17KV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\x1AjW\x88\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x01\x85R\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x87\x81\x01Q\x88\x85\x01R\x86\x01Q``\x87\x85\x01\x81\x90Ra\x1AV\x81\x86\x01\x83a\x18\xABV[\x96\x89\x01\x96\x94PPP\x90\x86\x01\x90`\x01\x01a\x19\xECV[P\x90\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x13(`@\x83\x01\x84a\x18\xABV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1A\xE9Wa\x1A\xE9a\x1A\xA7V[P\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x1B#``\x83\x01\x84a\x18\xABV[\x95\x94PPPPPV[`\0\x84Qa\x1B>\x81\x84` \x89\x01a\x18{V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x1Bz\x81`\x01\x85\x01` \x8A\x01a\x18{V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x1B\x95\x81`\x02\x84\x01` \x88\x01a\x18{V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@` \x82\x01R`\0a\x13(`@\x83\x01\x84a\x18\xABV[`\0\x82Qa\x1B\xFC\x81\x84` \x87\x01a\x18{V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\x18W`\0\x80\xFD[PQ\x91\x90PV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1CPWa\x1CPa\x1A\xA7V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1C\x95Wa\x1C\x95a\x1CWV[P\x04\x90V[`\0\x82\x82\x10\x15a\x1C\xACWa\x1C\xACa\x1A\xA7V[P\x03\x90V[`\0\x82a\x1C\xC0Wa\x1C\xC0a\x1CWV[P\x06\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static OPTIMISTINVITER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x91m\xB2/\x11a\0\x8CW\x80c\xDB\x08=q\x11a\0fW\x80c\xDB\x08=q\x14a\x02RW\x80c\xDE-\xD2!\x14a\x02yW\x80c\xEC\xCE\xC5\xA8\x14a\x02\x99W\x80c\xF6-\x18\x88\x14a\x02\xD5W`\0\x80\xFD[\x80c\x91m\xB2/\x14a\x01\xE4W\x80c\xB4$]s\x14a\x02\x0BW\x80c\xC4\xFCE=\x14a\x02+W`\0\x80\xFD[\x80cP\xB4\x14\xE6\x11a\0\xC8W\x80cP\xB4\x14\xE6\x14a\x01hW\x80cP\xEE\xDB\xC2\x14a\x01~W\x80cT\xFDMP\x14a\x01\x91W\x80c_\xDA\x04\xC7\x14a\x01\xA6W`\0\x80\xFD[\x80c\x14\xB4rA\x14a\0\xEFW\x80c\x18~<\xD1\x14a\x01@W\x80c%\xB2z=\x14a\x01UW[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Sa\x01N6`\x04a\x16_V[a\x02\xE8V[\0[a\x01Sa\x01c6`\x04a\x16xV[a\x03\x9CV[a\x01p`<\x81V[`@Q\x90\x81R` \x01a\x017V[a\x01Sa\x01\x8C6`\x04a\x17\xDFV[a\x06\xBFV[a\x01\x99a\rHV[`@Qa\x017\x91\x90a\x18\xF5V[a\x01\xD4a\x01\xB46`\x04a\x19\x08V[`6` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x017V[a\x01p\x7Foptimist.can-invite\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01pa\x02\x196`\x04a\x16_V[`5` R`\0\x90\x81R`@\x90 T\x81V[a\x01p\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCE\x81V[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01pa\x02\x876`\x04a\x192V[`7` R`\0\x90\x81R`@\x90 T\x81V[a\x01\x99`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[a\x01Sa\x02\xE36`\x04a\x19MV[a\r\xEBV[`\0\x81\x81R`5` R`@\x90 T\x15a\x03\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FOptimistInviter: commitment alre`D\x82\x01R\x7Fady made\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`5` R`@\x90 B\x90UV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04aW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FOptimistInviter: only invite gra`D\x82\x01R\x7Fnter can grant invites\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[\x81`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04}Wa\x04}a\x17\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xCAW\x81` \x01[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\x9BW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x06\x13W\x83`7`\0\x88\x88\x85\x81\x81\x10a\x04\xEFWa\x04\xEFa\x19\x96V[\x90P` \x02\x01` \x81\x01\x90a\x05\x04\x91\x90a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`@Q\x80``\x01`@R\x80\x87\x87\x84\x81\x81\x10a\x05_Wa\x05_a\x19\x96V[\x90P` \x02\x01` \x81\x01\x90a\x05t\x91\x90a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x7Foptimist.can-invite\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x01`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftrue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81RP\x82\x82\x81Q\x81\x10a\x06\0Wa\x06\0a\x19\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xD0V[P`@Q\x7F^\xB5\xEA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^\xB5\xEA\x10\x90a\x06\x86\x90\x84\x90`\x04\x01a\x19\xC5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0`5`\0\x85\x84`@Q` \x01a\x06\xD8\x92\x91\x90a\x1AxV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x11a\x07\xB5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FOptimistInviter: claimer and sig`D\x82\x01R\x7Fnature have not been committed y`d\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\x80V[Ba\x07\xC1`<\x83a\x1A\xD6V[\x11\x15a\x08OW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FOptimistInviter: minimum commitm`D\x82\x01R\x7Fent period has not elapsed yet\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`\0a\x08\xD2\x7Fe)\xFD\x12\x93Q\xE7%\xD7\xBC\xBCF\x8B\x0B\x0BFuG~V\xB5\x85\x14\xE6\x9A\xB7\xE6m\xDF\xD2\x0F\xCEa\x08\x82` \x87\x01\x87a\x192V[`@\x80Q` \x81\x81\x01\x94\x90\x94Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x90\x82\x01R\x90\x86\x015``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x0F\xB5V[\x90Pa\x08\xEBa\x08\xE4` \x86\x01\x86a\x192V[\x82\x85a\x10$V[a\twW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FOptimistInviter: invalid signatu`D\x82\x01R\x7Fre\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`6`\0a\t\x88` \x87\x01\x87a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x87\x84\x015\x82R\x90\x92R\x90 T`\xFF\x16\x15a\nOW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOptimistInviter: nonce has alrea`D\x82\x01R\x7Fdy been used\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`\x01`6`\0a\nb` \x88\x01\x88a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x88\x84\x01\x805\x83R\x93R\x90\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x93\x15\x15\x93\x90\x93\x17\x90\x92U`7\x90\x82\x90a\n\xD6\x90\x88a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x11a\x0B\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOptimistInviter: issuer has no i`D\x82\x01R\x7Fnvites\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`7`\0a\x0B\xAF` \x87\x01\x87a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x90\x91\x01`\0 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cp+\x9D\xEE\x90\x87\x90\x7Foptimist.can-mint-from-invite\0\0\0\x90a\x0CY\x90\x89\x01\x89a\x192V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x83\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xA8\x93\x92\x91\x90a\x1A\xEEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xC2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xD6W=`\0\x80>=`\0\xFD[PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90Pa\x0C\xFF` \x86\x01\x86a\x192V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Ft]<[\xC9*\xB4\x0BA\x80i\xBF\x8F\x8E 0\x80~\xFF\xCE\xB8\x8B\xBA\xA0~\xE0\x15t\xF1k\xE4u`@Q`@Q\x80\x91\x03\x90\xA3PPPPPV[``a\rs\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\xF3V[a\r\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\xF3V[a\r\xC5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\xF3V[`@Q` \x01a\r\xD7\x93\x92\x91\x90a\x1B,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0E\x0BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0E%WP0;\x15\x80\x15a\x0E%WP`\0T`\xFF\x16`\x01\x14[a\x0E\xB1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x80\x15a\x0F\x0FW`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[a\x0FN\x82`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x130V[\x80\x15a\x0F\xB1W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[`\0a\x10\x1Ea\x0F\xC2a\x13\xD1V[\x83`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x92\x91PPV[`\0\x80`\0a\x103\x85\x85a\x14QV[\x90\x92P\x90P`\0\x81`\x04\x81\x11\x15a\x10LWa\x10La\x1B\xA2V[\x14\x80\x15a\x10\x84WP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x15a\x10\x94W`\x01\x92PPPa\x11\xECV[`\0\x80\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x10\xC9\x92\x91\x90a\x1B\xD1V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x11R\x91\x90a\x1B\xEAV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x11\x8DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11\x92V[``\x91P[P\x91P\x91P\x81\x80\x15a\x11\xA5WP\x80Q` \x14[\x80\x15a\x11\xE5WP\x80Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x11\xE3\x90\x83\x01` \x90\x81\x01\x90\x84\x01a\x1C\x06V[\x14[\x94PPPPP[\x93\x92PPPV[``\x81`\0\x03a\x126WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x12`W\x80a\x12J\x81a\x1C\x1FV[\x91Pa\x12Y\x90P`\n\x83a\x1C\x86V[\x91Pa\x12:V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12{Wa\x12{a\x17\x1CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\xA5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x13(Wa\x12\xBA`\x01\x83a\x1C\x9AV[\x91Pa\x12\xC7`\n\x86a\x1C\xB1V[a\x12\xD2\x90`0a\x1A\xD6V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x12\xE7Wa\x12\xE7a\x19\x96V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x13!`\n\x86a\x1C\x86V[\x94Pa\x12\xA9V[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13\xC7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[a\x0F\xB1\x82\x82a\x14\x96V[`\0a\x14L\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x14\0`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[\x90P\x90V[`\0\x80\x82Q`A\x03a\x14\x87W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x14{\x87\x82\x85\x85a\x15GV[\x94P\x94PPPPa\x14\x8FV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x15-W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x80V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x15~WP`\0\x90P`\x03a\x16VV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x15\x96WP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x15\xA7WP`\0\x90P`\x04a\x16VV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xFBW=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x16OW`\0`\x01\x92P\x92PPa\x16VV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x16qW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x16\x8DW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xA5W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x16\xB9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16\xC8W`\0\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\xDDW`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17\x17W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x17fWa\x17fa\x17\x1CV[`@Q`\x1F\x85\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x17\xACWa\x17\xACa\x17\x1CV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x17\xC5W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`\0\x83\x85\x03`\x80\x81\x12\x15a\x17\xF5W`\0\x80\xFD[a\x17\xFE\x85a\x16\xF3V[\x93P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x01\x12\x15a\x180W`\0\x80\xFD[P` \x84\x01\x91P``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18QW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x18bW`\0\x80\xFD[a\x18q\x86\x825` \x84\x01a\x17KV[\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x18\x96W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18~V[\x83\x81\x11\x15a\x18\xA5W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x18\xC3\x81` \x86\x01` \x86\x01a\x18{V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x11\xEC` \x83\x01\x84a\x18\xABV[`\0\x80`@\x83\x85\x03\x12\x15a\x19\x1BW`\0\x80\xFD[a\x19$\x83a\x16\xF3V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x19DW`\0\x80\xFD[a\x11\xEC\x82a\x16\xF3V[`\0` \x82\x84\x03\x12\x15a\x19_W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19vW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\x87W`\0\x80\xFD[a\x13(\x84\x825` \x84\x01a\x17KV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\x1AjW\x88\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x01\x85R\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x87\x81\x01Q\x88\x85\x01R\x86\x01Q``\x87\x85\x01\x81\x90Ra\x1AV\x81\x86\x01\x83a\x18\xABV[\x96\x89\x01\x96\x94PPP\x90\x86\x01\x90`\x01\x01a\x19\xECV[P\x90\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x13(`@\x83\x01\x84a\x18\xABV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1A\xE9Wa\x1A\xE9a\x1A\xA7V[P\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x1B#``\x83\x01\x84a\x18\xABV[\x95\x94PPPPPV[`\0\x84Qa\x1B>\x81\x84` \x89\x01a\x18{V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x1Bz\x81`\x01\x85\x01` \x8A\x01a\x18{V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x1B\x95\x81`\x02\x84\x01` \x88\x01a\x18{V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@` \x82\x01R`\0a\x13(`@\x83\x01\x84a\x18\xABV[`\0\x82Qa\x1B\xFC\x81\x84` \x87\x01a\x18{V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\x18W`\0\x80\xFD[PQ\x91\x90PV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1CPWa\x1CPa\x1A\xA7V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1C\x95Wa\x1C\x95a\x1CWV[P\x04\x90V[`\0\x82\x82\x10\x15a\x1C\xACWa\x1C\xACa\x1A\xA7V[P\x03\x90V[`\0\x82a\x1C\xC0Wa\x1C\xC0a\x1CWV[P\x06\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static OPTIMISTINVITER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OptimistInviter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OptimistInviter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OptimistInviter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OptimistInviter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OptimistInviter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OptimistInviter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OptimistInviter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPTIMISTINVITER_ABI.clone(),
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
                OPTIMISTINVITER_ABI.clone(),
                OPTIMISTINVITER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ATTESTATION_STATION` (0xdb083d71) function
        pub fn attestation_station(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([219, 8, 61, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CAN_INVITE_ATTESTATION_KEY` (0x916db22f) function
        pub fn can_invite_attestation_key(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([145, 109, 178, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CLAIMABLE_INVITE_TYPEHASH` (0xc4fc453d) function
        pub fn claimable_invite_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([196, 252, 69, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EIP712_VERSION` (0xeccec5a8) function
        pub fn eip712_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([236, 206, 197, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVITE_GRANTER` (0x14b47241) function
        pub fn invite_granter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([20, 180, 114, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MIN_COMMITMENT_PERIOD` (0x50b414e6) function
        pub fn min_commitment_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 180, 20, 230], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimInvite` (0x50eedbc2) function
        pub fn claim_invite(
            &self,
            claimer: ::ethers::core::types::Address,
            claimable_invite: ClaimableInvite,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 238, 219, 194], (claimer, claimable_invite, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitInvite` (0x187e3cd1) function
        pub fn commit_invite(
            &self,
            commitment: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 126, 60, 209], commitment)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitmentTimestamps` (0xb4245d73) function
        pub fn commitment_timestamps(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 36, 93, 115], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf62d1888) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 45, 24, 136], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inviteCounts` (0xde2dd221) function
        pub fn invite_counts(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([222, 45, 210, 33], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setInviteCounts` (0x25b27a3d) function
        pub fn set_invite_counts(
            &self,
            accounts: ::std::vec::Vec<::ethers::core::types::Address>,
            invite_count: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 178, 122, 61], (accounts, invite_count))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usedNonces` (0x5fda04c7) function
        pub fn used_nonces(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([95, 218, 4, 199], (p0, p1))
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
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InviteClaimed` event
        pub fn invite_claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InviteClaimedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OptimistInviterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OptimistInviter<M> {
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
    #[ethevent(name = "InviteClaimed", abi = "InviteClaimed(address,address)")]
    pub struct InviteClaimedFilter {
        #[ethevent(indexed)]
        pub issuer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub claimer: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OptimistInviterEvents {
        InitializedFilter(InitializedFilter),
        InviteClaimedFilter(InviteClaimedFilter),
    }
    impl ::ethers::contract::EthLogDecode for OptimistInviterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OptimistInviterEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = InviteClaimedFilter::decode_log(log) {
                return Ok(OptimistInviterEvents::InviteClaimedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OptimistInviterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InviteClaimedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for OptimistInviterEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<InviteClaimedFilter> for OptimistInviterEvents {
        fn from(value: InviteClaimedFilter) -> Self {
            Self::InviteClaimedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ATTESTATION_STATION` function with signature `ATTESTATION_STATION()` and selector `0xdb083d71`
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
    #[ethcall(name = "ATTESTATION_STATION", abi = "ATTESTATION_STATION()")]
    pub struct AttestationStationCall;
    ///Container type for all input parameters for the `CAN_INVITE_ATTESTATION_KEY` function with signature `CAN_INVITE_ATTESTATION_KEY()` and selector `0x916db22f`
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
    #[ethcall(name = "CAN_INVITE_ATTESTATION_KEY", abi = "CAN_INVITE_ATTESTATION_KEY()")]
    pub struct CanInviteAttestationKeyCall;
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
    ///Container type for all input parameters for the `EIP712_VERSION` function with signature `EIP712_VERSION()` and selector `0xeccec5a8`
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
    #[ethcall(name = "EIP712_VERSION", abi = "EIP712_VERSION()")]
    pub struct Eip712VersionCall;
    ///Container type for all input parameters for the `INVITE_GRANTER` function with signature `INVITE_GRANTER()` and selector `0x14b47241`
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
    #[ethcall(name = "INVITE_GRANTER", abi = "INVITE_GRANTER()")]
    pub struct InviteGranterCall;
    ///Container type for all input parameters for the `MIN_COMMITMENT_PERIOD` function with signature `MIN_COMMITMENT_PERIOD()` and selector `0x50b414e6`
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
    #[ethcall(name = "MIN_COMMITMENT_PERIOD", abi = "MIN_COMMITMENT_PERIOD()")]
    pub struct MinCommitmentPeriodCall;
    ///Container type for all input parameters for the `claimInvite` function with signature `claimInvite(address,(address,bytes32),bytes)` and selector `0x50eedbc2`
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
        name = "claimInvite",
        abi = "claimInvite(address,(address,bytes32),bytes)"
    )]
    pub struct ClaimInviteCall {
        pub claimer: ::ethers::core::types::Address,
        pub claimable_invite: ClaimableInvite,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `commitInvite` function with signature `commitInvite(bytes32)` and selector `0x187e3cd1`
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
    #[ethcall(name = "commitInvite", abi = "commitInvite(bytes32)")]
    pub struct CommitInviteCall {
        pub commitment: [u8; 32],
    }
    ///Container type for all input parameters for the `commitmentTimestamps` function with signature `commitmentTimestamps(bytes32)` and selector `0xb4245d73`
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
    #[ethcall(name = "commitmentTimestamps", abi = "commitmentTimestamps(bytes32)")]
    pub struct CommitmentTimestampsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string)` and selector `0xf62d1888`
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
    #[ethcall(name = "initialize", abi = "initialize(string)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `inviteCounts` function with signature `inviteCounts(address)` and selector `0xde2dd221`
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
    #[ethcall(name = "inviteCounts", abi = "inviteCounts(address)")]
    pub struct InviteCountsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `setInviteCounts` function with signature `setInviteCounts(address[],uint256)` and selector `0x25b27a3d`
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
    #[ethcall(name = "setInviteCounts", abi = "setInviteCounts(address[],uint256)")]
    pub struct SetInviteCountsCall {
        pub accounts: ::std::vec::Vec<::ethers::core::types::Address>,
        pub invite_count: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `usedNonces` function with signature `usedNonces(address,bytes32)` and selector `0x5fda04c7`
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
    #[ethcall(name = "usedNonces", abi = "usedNonces(address,bytes32)")]
    pub struct UsedNoncesCall(pub ::ethers::core::types::Address, pub [u8; 32]);
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
    pub enum OptimistInviterCalls {
        AttestationStation(AttestationStationCall),
        CanInviteAttestationKey(CanInviteAttestationKeyCall),
        ClaimableInviteTypehash(ClaimableInviteTypehashCall),
        Eip712Version(Eip712VersionCall),
        InviteGranter(InviteGranterCall),
        MinCommitmentPeriod(MinCommitmentPeriodCall),
        ClaimInvite(ClaimInviteCall),
        CommitInvite(CommitInviteCall),
        CommitmentTimestamps(CommitmentTimestampsCall),
        Initialize(InitializeCall),
        InviteCounts(InviteCountsCall),
        SetInviteCounts(SetInviteCountsCall),
        UsedNonces(UsedNoncesCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for OptimistInviterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AttestationStationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationStation(decoded));
            }
            if let Ok(decoded) = <CanInviteAttestationKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanInviteAttestationKey(decoded));
            }
            if let Ok(decoded) = <ClaimableInviteTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimableInviteTypehash(decoded));
            }
            if let Ok(decoded) = <Eip712VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eip712Version(decoded));
            }
            if let Ok(decoded) = <InviteGranterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InviteGranter(decoded));
            }
            if let Ok(decoded) = <MinCommitmentPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinCommitmentPeriod(decoded));
            }
            if let Ok(decoded) = <ClaimInviteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimInvite(decoded));
            }
            if let Ok(decoded) = <CommitInviteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CommitInvite(decoded));
            }
            if let Ok(decoded) = <CommitmentTimestampsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CommitmentTimestamps(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <InviteCountsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InviteCounts(decoded));
            }
            if let Ok(decoded) = <SetInviteCountsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetInviteCounts(decoded));
            }
            if let Ok(decoded) = <UsedNoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsedNonces(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OptimistInviterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestationStation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanInviteAttestationKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimableInviteTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Version(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InviteGranter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinCommitmentPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimInvite(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CommitInvite(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CommitmentTimestamps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InviteCounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetInviteCounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedNonces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OptimistInviterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationStation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanInviteAttestationKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimableInviteTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Eip712Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::InviteGranter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinCommitmentPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimInvite(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitInvite(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitmentTimestamps(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::InviteCounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInviteCounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedNonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestationStationCall> for OptimistInviterCalls {
        fn from(value: AttestationStationCall) -> Self {
            Self::AttestationStation(value)
        }
    }
    impl ::core::convert::From<CanInviteAttestationKeyCall> for OptimistInviterCalls {
        fn from(value: CanInviteAttestationKeyCall) -> Self {
            Self::CanInviteAttestationKey(value)
        }
    }
    impl ::core::convert::From<ClaimableInviteTypehashCall> for OptimistInviterCalls {
        fn from(value: ClaimableInviteTypehashCall) -> Self {
            Self::ClaimableInviteTypehash(value)
        }
    }
    impl ::core::convert::From<Eip712VersionCall> for OptimistInviterCalls {
        fn from(value: Eip712VersionCall) -> Self {
            Self::Eip712Version(value)
        }
    }
    impl ::core::convert::From<InviteGranterCall> for OptimistInviterCalls {
        fn from(value: InviteGranterCall) -> Self {
            Self::InviteGranter(value)
        }
    }
    impl ::core::convert::From<MinCommitmentPeriodCall> for OptimistInviterCalls {
        fn from(value: MinCommitmentPeriodCall) -> Self {
            Self::MinCommitmentPeriod(value)
        }
    }
    impl ::core::convert::From<ClaimInviteCall> for OptimistInviterCalls {
        fn from(value: ClaimInviteCall) -> Self {
            Self::ClaimInvite(value)
        }
    }
    impl ::core::convert::From<CommitInviteCall> for OptimistInviterCalls {
        fn from(value: CommitInviteCall) -> Self {
            Self::CommitInvite(value)
        }
    }
    impl ::core::convert::From<CommitmentTimestampsCall> for OptimistInviterCalls {
        fn from(value: CommitmentTimestampsCall) -> Self {
            Self::CommitmentTimestamps(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for OptimistInviterCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InviteCountsCall> for OptimistInviterCalls {
        fn from(value: InviteCountsCall) -> Self {
            Self::InviteCounts(value)
        }
    }
    impl ::core::convert::From<SetInviteCountsCall> for OptimistInviterCalls {
        fn from(value: SetInviteCountsCall) -> Self {
            Self::SetInviteCounts(value)
        }
    }
    impl ::core::convert::From<UsedNoncesCall> for OptimistInviterCalls {
        fn from(value: UsedNoncesCall) -> Self {
            Self::UsedNonces(value)
        }
    }
    impl ::core::convert::From<VersionCall> for OptimistInviterCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `ATTESTATION_STATION` function with signature `ATTESTATION_STATION()` and selector `0xdb083d71`
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
    pub struct AttestationStationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `CAN_INVITE_ATTESTATION_KEY` function with signature `CAN_INVITE_ATTESTATION_KEY()` and selector `0x916db22f`
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
    pub struct CanInviteAttestationKeyReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `EIP712_VERSION` function with signature `EIP712_VERSION()` and selector `0xeccec5a8`
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
    pub struct Eip712VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVITE_GRANTER` function with signature `INVITE_GRANTER()` and selector `0x14b47241`
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
    pub struct InviteGranterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `MIN_COMMITMENT_PERIOD` function with signature `MIN_COMMITMENT_PERIOD()` and selector `0x50b414e6`
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
    pub struct MinCommitmentPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `commitmentTimestamps` function with signature `commitmentTimestamps(bytes32)` and selector `0xb4245d73`
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
    pub struct CommitmentTimestampsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `inviteCounts` function with signature `inviteCounts(address)` and selector `0xde2dd221`
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
    pub struct InviteCountsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `usedNonces` function with signature `usedNonces(address,bytes32)` and selector `0x5fda04c7`
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
    pub struct UsedNoncesReturn(pub bool);
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

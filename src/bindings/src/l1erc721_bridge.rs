pub use l1erc721_bridge::*;
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
pub mod l1erc721_bridge {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MESSENGER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MESSENGER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract CrossDomainMessenger",
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
                    ::std::borrow::ToOwned::to_owned("OTHER_BRIDGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("OTHER_BRIDGE"),
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
                    ::std::borrow::ToOwned::to_owned("bridgeERC721"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeERC721"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("bridgeERC721To"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeERC721To"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("deposits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposits"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("finalizeBridgeERC721"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "finalizeBridgeERC721",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_messenger"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract CrossDomainMessenger",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("messenger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("messenger"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract CrossDomainMessenger",
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
                    ::std::borrow::ToOwned::to_owned("otherBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("otherBridge"),
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
                    ::std::borrow::ToOwned::to_owned("ERC721BridgeFinalized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721BridgeFinalized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721BridgeInitiated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721BridgeInitiated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("localToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static L1ERC721BRIDGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[PsB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a\x004V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0J`\0a\0OV[a\x01\xCBV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\0qWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\0\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\0+V[`\0\x80Ta\xFF\xFF\x19\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\0\xF1\x82a\x016V[`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x01\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\0+V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x80Qa\x12\x11a\x01\xFB`\09`\0\x81\x81a\x01\xCD\x01R\x81\x81a\x02;\x01R\x81\x81a\x035\x01Ra\x0B\xF1\x01Ra\x12\x11`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80c\x7FF\xDD\xB2\x11a\0vW\x80c\xAAUtR\x11a\0[W\x80c\xAAUtR\x14a\x02\x13W\x80c\xC4\xD6m\xE8\x14a\x02&W\x80c\xC8\x97\x01\xA2\x14a\x029W`\0\x80\xFD[\x80c\x7FF\xDD\xB2\x14a\x01\xC8W\x80c\x92~\xDE-\x14a\x01\xEFW`\0\x80\xFD[\x80cT\xFDMP\x11a\0\xA7W\x80cT\xFDMP\x14a\x01(W\x80c]\x93\xA3\xFC\x14a\x01qW\x80cv\x1FD\x93\x14a\x01\xB5W`\0\x80\xFD[\x80c6\x87\x01\x1A\x14a\0\xC3W\x80c<\xB7G\xBF\x14a\0\xD8W[`\0\x80\xFD[a\0\xD6a\0\xD16`\x04a\x0EGV[a\x02_V[\0[`\0Ta\0\xFE\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01d`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.3.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\x1F\x91\x90a\x0F5V[a\x01\xA5a\x01\x7F6`\x04a\x0FOV[`1` \x90\x81R`\0\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1FV[a\0\xD6a\x01\xC36`\x04a\x0F\x90V[a\x03\x0BV[a\0\xFE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\0\xFEV[a\0\xD6a\x02!6`\x04a\x10(V[a\x07vV[a\0\xD6a\x0246`\x04a\x10\x9FV[a\x082V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xFEV[3;\x15a\x02\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC721Bridge: account is not ext`D\x82\x01R\x7Fernally owned\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x03\x86\x8633\x88\x88\x88\x88a\t|V[PPPPPPV[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\x04\x13WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cn)nE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFB\x91\x90a\x10\xBCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x04\x9FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FERC721Bridge: function can only `D\x82\x01R\x7Fbe called from the other bridge\0`d\x82\x01R`\x84\x01a\x02\xEAV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x03a\x05DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FL1ERC721Bridge: local token cann`D\x82\x01R\x7Fot be self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16`\0\x90\x81R`1` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x81R\x82\x82 \x86\x83R\x90R T`\xFF\x16\x15\x15`\x01\x14a\x06\x13W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FL1ERC721Bridge: Token ID is not `D\x82\x01R\x7Fescrowed in the L1 Bridge\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`\0\x81\x81R`1` \x90\x81R`@\x80\x83 \x8B\x86\x16\x84R\x82R\x80\x83 \x88\x84R\x90\x91R\x90\x81\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ\x7FB\x84.\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x91\x86\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90cB\x84.\x0E\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE7W=`\0\x80>=`\0\xFD[PPPP\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1F9\xBFg\x07\xB5\xD6\x08E>\n\xE4\xC0g\xB5b\xBC\xC4\xC8\\\x0FV.\xF5\xD2\xC7t\xD2\xE7\xF11\xAC\x87\x87\x87\x87`@Qa\x07e\x94\x93\x92\x91\x90a\x11\"V[`@Q\x80\x91\x03\x90\xA4PPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x08\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC721Bridge: nft recipient cann`D\x82\x01R\x7Fot be address(0)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[a\x08)\x87\x873\x88\x88\x88\x88\x88a\t|V[PPPPPPPV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x08TWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x08\xE0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\t\x1A\x82a\x0C\xDCV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16a\n\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FL1ERC721Bridge: remote token can`D\x82\x01R\x7Fnot be address(0)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[`\0cv\x1FD\x93`\xE0\x1B\x88\x8A\x89\x89\x89\x88\x88`@Q`$\x01a\nF\x97\x96\x95\x94\x93\x92\x91\x90a\x11bV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x90\x95\x16\x94\x90\x94\x17\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x81\x16`\0\x81\x81R`1\x86R\x83\x81 \x8E\x84\x16\x82R\x86R\x83\x81 \x8B\x82R\x90\x95R\x93\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x90Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x8A\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x92Pc#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x86W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\x9AW=`\0\x80>=`\0\xFD[PP`\0T`@Q\x7F=\xBB +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\x01\0\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92Pc=\xBB +\x91Pa\x0C\x1D\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x89\x90`\x04\x01a\x11\xBFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0CKW=`\0\x80>=`\0\xFD[PPPP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB7F\x0E*\x88\x0F%n\xBE\xF3@a\x16\xFF>\xEE\x0C\xEEQ\xEB\xCC\xDC*@i\x8F\x87\xEB\xB2\xE9\xC1\xA5\x89\x89\x88\x88`@Qa\x0C\xC9\x94\x93\x92\x91\x90a\x11\"V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\rsW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16b\x01\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xE2W`\0\x80\xFD[PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xF9W`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x0E\x10W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E(W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0E@W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x0E`W`\0\x80\xFD[\x865a\x0Ek\x81a\r\xC0V[\x95P` \x87\x015a\x0E{\x81a\r\xC0V[\x94P`@\x87\x015\x93Pa\x0E\x90``\x88\x01a\r\xE5V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xACW`\0\x80\xFD[a\x0E\xB8\x89\x82\x8A\x01a\r\xFEV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0E\xF0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0E\xD4V[\x81\x81\x11\x15a\x0F\x02W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0FH` \x83\x01\x84a\x0E\xCAV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0FdW`\0\x80\xFD[\x835a\x0Fo\x81a\r\xC0V[\x92P` \x84\x015a\x0F\x7F\x81a\r\xC0V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x0F\xABW`\0\x80\xFD[\x875a\x0F\xB6\x81a\r\xC0V[\x96P` \x88\x015a\x0F\xC6\x81a\r\xC0V[\x95P`@\x88\x015a\x0F\xD6\x81a\r\xC0V[\x94P``\x88\x015a\x0F\xE6\x81a\r\xC0V[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\tW`\0\x80\xFD[a\x10\x15\x8A\x82\x8B\x01a\r\xFEV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x10CW`\0\x80\xFD[\x875a\x10N\x81a\r\xC0V[\x96P` \x88\x015a\x10^\x81a\r\xC0V[\x95P`@\x88\x015a\x10n\x81a\r\xC0V[\x94P``\x88\x015\x93Pa\x10\x83`\x80\x89\x01a\r\xE5V[\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\tW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xB1W`\0\x80\xFD[\x815a\x0FH\x81a\r\xC0V[`\0` \x82\x84\x03\x12\x15a\x10\xCEW`\0\x80\xFD[\x81Qa\x0FH\x81a\r\xC0V[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x11X``\x83\x01\x84\x86a\x10\xD9V[\x96\x95PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x83R\x80\x89\x16` \x84\x01R\x80\x88\x16`@\x84\x01R\x80\x87\x16``\x84\x01RP\x84`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x11\xB2`\xC0\x83\x01\x84\x86a\x10\xD9V[\x99\x98PPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0a\x11\xEE``\x83\x01\x85a\x0E\xCAV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L1ERC721BRIDGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80c\x7FF\xDD\xB2\x11a\0vW\x80c\xAAUtR\x11a\0[W\x80c\xAAUtR\x14a\x02\x13W\x80c\xC4\xD6m\xE8\x14a\x02&W\x80c\xC8\x97\x01\xA2\x14a\x029W`\0\x80\xFD[\x80c\x7FF\xDD\xB2\x14a\x01\xC8W\x80c\x92~\xDE-\x14a\x01\xEFW`\0\x80\xFD[\x80cT\xFDMP\x11a\0\xA7W\x80cT\xFDMP\x14a\x01(W\x80c]\x93\xA3\xFC\x14a\x01qW\x80cv\x1FD\x93\x14a\x01\xB5W`\0\x80\xFD[\x80c6\x87\x01\x1A\x14a\0\xC3W\x80c<\xB7G\xBF\x14a\0\xD8W[`\0\x80\xFD[a\0\xD6a\0\xD16`\x04a\x0EGV[a\x02_V[\0[`\0Ta\0\xFE\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01d`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F1.3.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\x1F\x91\x90a\x0F5V[a\x01\xA5a\x01\x7F6`\x04a\x0FOV[`1` \x90\x81R`\0\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1FV[a\0\xD6a\x01\xC36`\x04a\x0F\x90V[a\x03\x0BV[a\0\xFE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\0\xFEV[a\0\xD6a\x02!6`\x04a\x10(V[a\x07vV[a\0\xD6a\x0246`\x04a\x10\x9FV[a\x082V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xFEV[3;\x15a\x02\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC721Bridge: account is not ext`D\x82\x01R\x7Fernally owned\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x03\x86\x8633\x88\x88\x88\x88a\t|V[PPPPPPV[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14\x80\x15a\x04\x13WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cn)nE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFB\x91\x90a\x10\xBCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x04\x9FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FERC721Bridge: function can only `D\x82\x01R\x7Fbe called from the other bridge\0`d\x82\x01R`\x84\x01a\x02\xEAV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x03a\x05DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FL1ERC721Bridge: local token cann`D\x82\x01R\x7Fot be self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16`\0\x90\x81R`1` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x81R\x82\x82 \x86\x83R\x90R T`\xFF\x16\x15\x15`\x01\x14a\x06\x13W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FL1ERC721Bridge: Token ID is not `D\x82\x01R\x7Fescrowed in the L1 Bridge\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`\0\x81\x81R`1` \x90\x81R`@\x80\x83 \x8B\x86\x16\x84R\x82R\x80\x83 \x88\x84R\x90\x91R\x90\x81\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90UQ\x7FB\x84.\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x91\x86\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90cB\x84.\x0E\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE7W=`\0\x80>=`\0\xFD[PPPP\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1F9\xBFg\x07\xB5\xD6\x08E>\n\xE4\xC0g\xB5b\xBC\xC4\xC8\\\x0FV.\xF5\xD2\xC7t\xD2\xE7\xF11\xAC\x87\x87\x87\x87`@Qa\x07e\x94\x93\x92\x91\x90a\x11\"V[`@Q\x80\x91\x03\x90\xA4PPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x08\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC721Bridge: nft recipient cann`D\x82\x01R\x7Fot be address(0)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[a\x08)\x87\x873\x88\x88\x88\x88\x88a\t|V[PPPPPPPV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\x08TWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x08\xE0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16`\xFF\x83\x16\x17a\x01\0\x17\x90Ua\t\x1A\x82a\x0C\xDCV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x90U`@Q`\xFF\x82\x16\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16a\n\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FL1ERC721Bridge: remote token can`D\x82\x01R\x7Fnot be address(0)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[`\0cv\x1FD\x93`\xE0\x1B\x88\x8A\x89\x89\x89\x88\x88`@Q`$\x01a\nF\x97\x96\x95\x94\x93\x92\x91\x90a\x11bV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x90\x95\x16\x94\x90\x94\x17\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x81\x16`\0\x81\x81R`1\x86R\x83\x81 \x8E\x84\x16\x82R\x86R\x83\x81 \x8B\x82R\x90\x95R\x93\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x90Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x8A\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R\x90\x92Pc#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x86W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\x9AW=`\0\x80>=`\0\xFD[PP`\0T`@Q\x7F=\xBB +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rb\x01\0\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92Pc=\xBB +\x91Pa\x0C\x1D\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x89\x90`\x04\x01a\x11\xBFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0CKW=`\0\x80>=`\0\xFD[PPPP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB7F\x0E*\x88\x0F%n\xBE\xF3@a\x16\xFF>\xEE\x0C\xEEQ\xEB\xCC\xDC*@i\x8F\x87\xEB\xB2\xE9\xC1\xA5\x89\x89\x88\x88`@Qa\x0C\xC9\x94\x93\x92\x91\x90a\x11\"V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\rsW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xEAV[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16b\x01\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xE2W`\0\x80\xFD[PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xF9W`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x0E\x10W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E(W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0E@W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x0E`W`\0\x80\xFD[\x865a\x0Ek\x81a\r\xC0V[\x95P` \x87\x015a\x0E{\x81a\r\xC0V[\x94P`@\x87\x015\x93Pa\x0E\x90``\x88\x01a\r\xE5V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xACW`\0\x80\xFD[a\x0E\xB8\x89\x82\x8A\x01a\r\xFEV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0E\xF0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0E\xD4V[\x81\x81\x11\x15a\x0F\x02W`\0` \x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0FH` \x83\x01\x84a\x0E\xCAV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0FdW`\0\x80\xFD[\x835a\x0Fo\x81a\r\xC0V[\x92P` \x84\x015a\x0F\x7F\x81a\r\xC0V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x0F\xABW`\0\x80\xFD[\x875a\x0F\xB6\x81a\r\xC0V[\x96P` \x88\x015a\x0F\xC6\x81a\r\xC0V[\x95P`@\x88\x015a\x0F\xD6\x81a\r\xC0V[\x94P``\x88\x015a\x0F\xE6\x81a\r\xC0V[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\tW`\0\x80\xFD[a\x10\x15\x8A\x82\x8B\x01a\r\xFEV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x10CW`\0\x80\xFD[\x875a\x10N\x81a\r\xC0V[\x96P` \x88\x015a\x10^\x81a\r\xC0V[\x95P`@\x88\x015a\x10n\x81a\r\xC0V[\x94P``\x88\x015\x93Pa\x10\x83`\x80\x89\x01a\r\xE5V[\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\tW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xB1W`\0\x80\xFD[\x815a\x0FH\x81a\r\xC0V[`\0` \x82\x84\x03\x12\x15a\x10\xCEW`\0\x80\xFD[\x81Qa\x0FH\x81a\r\xC0V[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x11X``\x83\x01\x84\x86a\x10\xD9V[\x96\x95PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x83R\x80\x89\x16` \x84\x01R\x80\x88\x16`@\x84\x01R\x80\x87\x16``\x84\x01RP\x84`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x11\xB2`\xC0\x83\x01\x84\x86a\x10\xD9V[\x99\x98PPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0a\x11\xEE``\x83\x01\x85a\x0E\xCAV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L1ERC721BRIDGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L1ERC721Bridge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L1ERC721Bridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L1ERC721Bridge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L1ERC721Bridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L1ERC721Bridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L1ERC721Bridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L1ERC721Bridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L1ERC721BRIDGE_ABI.clone(),
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
                L1ERC721BRIDGE_ABI.clone(),
                L1ERC721BRIDGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MESSENGER` (0x927ede2d) function
        pub fn MESSENGER(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([146, 126, 222, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OTHER_BRIDGE` (0x7f46ddb2) function
        pub fn OTHER_BRIDGE(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([127, 70, 221, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeERC721` (0x3687011a) function
        pub fn bridge_erc721(
            &self,
            local_token: ::ethers::core::types::Address,
            remote_token: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [54, 135, 1, 26],
                    (local_token, remote_token, token_id, min_gas_limit, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeERC721To` (0xaa557452) function
        pub fn bridge_erc721_to(
            &self,
            local_token: ::ethers::core::types::Address,
            remote_token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            min_gas_limit: u32,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [170, 85, 116, 82],
                    (local_token, remote_token, to, token_id, min_gas_limit, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposits` (0x5d93a3fc) function
        pub fn deposits(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([93, 147, 163, 252], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizeBridgeERC721` (0x761f4493) function
        pub fn finalize_bridge_erc721(
            &self,
            local_token: ::ethers::core::types::Address,
            remote_token: ::ethers::core::types::Address,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [118, 31, 68, 147],
                    (local_token, remote_token, from, to, token_id, extra_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            messenger: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], messenger)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messenger` (0x3cb747bf) function
        pub fn messenger(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([60, 183, 71, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `otherBridge` (0xc89701a2) function
        pub fn otherBridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 151, 1, 162], ())
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
        ///Gets the contract's `ERC721BridgeFinalized` event
        pub fn erc721_bridge_finalized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Erc721BridgeFinalizedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ERC721BridgeInitiated` event
        pub fn erc721_bridge_initiated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Erc721BridgeInitiatedFilter,
        > {
            self.0.event()
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            L1ERC721BridgeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L1ERC721Bridge<M> {
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
    #[ethevent(
        name = "ERC721BridgeFinalized",
        abi = "ERC721BridgeFinalized(address,address,address,address,uint256,bytes)"
    )]
    pub struct Erc721BridgeFinalizedFilter {
        #[ethevent(indexed)]
        pub local_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub remote_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
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
    #[ethevent(
        name = "ERC721BridgeInitiated",
        abi = "ERC721BridgeInitiated(address,address,address,address,uint256,bytes)"
    )]
    pub struct Erc721BridgeInitiatedFilter {
        #[ethevent(indexed)]
        pub local_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub remote_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L1ERC721BridgeEvents {
        Erc721BridgeFinalizedFilter(Erc721BridgeFinalizedFilter),
        Erc721BridgeInitiatedFilter(Erc721BridgeInitiatedFilter),
        InitializedFilter(InitializedFilter),
    }
    impl ::ethers::contract::EthLogDecode for L1ERC721BridgeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = Erc721BridgeFinalizedFilter::decode_log(log) {
                return Ok(L1ERC721BridgeEvents::Erc721BridgeFinalizedFilter(decoded));
            }
            if let Ok(decoded) = Erc721BridgeInitiatedFilter::decode_log(log) {
                return Ok(L1ERC721BridgeEvents::Erc721BridgeInitiatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(L1ERC721BridgeEvents::InitializedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L1ERC721BridgeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Erc721BridgeFinalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Erc721BridgeInitiatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Erc721BridgeFinalizedFilter> for L1ERC721BridgeEvents {
        fn from(value: Erc721BridgeFinalizedFilter) -> Self {
            Self::Erc721BridgeFinalizedFilter(value)
        }
    }
    impl ::core::convert::From<Erc721BridgeInitiatedFilter> for L1ERC721BridgeEvents {
        fn from(value: Erc721BridgeInitiatedFilter) -> Self {
            Self::Erc721BridgeInitiatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for L1ERC721BridgeEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    ///Container type for all input parameters for the `MESSENGER` function with signature `MESSENGER()` and selector `0x927ede2d`
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
    #[ethcall(name = "MESSENGER", abi = "MESSENGER()")]
    pub struct MESSENGERCall;
    ///Container type for all input parameters for the `OTHER_BRIDGE` function with signature `OTHER_BRIDGE()` and selector `0x7f46ddb2`
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
    #[ethcall(name = "OTHER_BRIDGE", abi = "OTHER_BRIDGE()")]
    pub struct OTHER_BRIDGECall;
    ///Container type for all input parameters for the `bridgeERC721` function with signature `bridgeERC721(address,address,uint256,uint32,bytes)` and selector `0x3687011a`
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
        name = "bridgeERC721",
        abi = "bridgeERC721(address,address,uint256,uint32,bytes)"
    )]
    pub struct BridgeERC721Call {
        pub local_token: ::ethers::core::types::Address,
        pub remote_token: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `bridgeERC721To` function with signature `bridgeERC721To(address,address,address,uint256,uint32,bytes)` and selector `0xaa557452`
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
        name = "bridgeERC721To",
        abi = "bridgeERC721To(address,address,address,uint256,uint32,bytes)"
    )]
    pub struct BridgeERC721ToCall {
        pub local_token: ::ethers::core::types::Address,
        pub remote_token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub min_gas_limit: u32,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deposits` function with signature `deposits(address,address,uint256)` and selector `0x5d93a3fc`
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
    #[ethcall(name = "deposits", abi = "deposits(address,address,uint256)")]
    pub struct DepositsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `finalizeBridgeERC721` function with signature `finalizeBridgeERC721(address,address,address,address,uint256,bytes)` and selector `0x761f4493`
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
        name = "finalizeBridgeERC721",
        abi = "finalizeBridgeERC721(address,address,address,address,uint256,bytes)"
    )]
    pub struct FinalizeBridgeERC721Call {
        pub local_token: ::ethers::core::types::Address,
        pub remote_token: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub messenger: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `messenger` function with signature `messenger()` and selector `0x3cb747bf`
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
    #[ethcall(name = "messenger", abi = "messenger()")]
    pub struct messengerCall;
    ///Container type for all input parameters for the `otherBridge` function with signature `otherBridge()` and selector `0xc89701a2`
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
    #[ethcall(name = "otherBridge", abi = "otherBridge()")]
    pub struct otherBridgeCall;
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
    pub enum L1ERC721BridgeCalls {
        MESSENGER(MESSENGERCall),
        OTHER_BRIDGE(OTHER_BRIDGECall),
        BridgeERC721(BridgeERC721Call),
        BridgeERC721To(BridgeERC721ToCall),
        Deposits(DepositsCall),
        FinalizeBridgeERC721(FinalizeBridgeERC721Call),
        Initialize(InitializeCall),
        messenger(messengerCall),
        otherBridge(otherBridgeCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for L1ERC721BridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MESSENGERCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MESSENGER(decoded));
            }
            if let Ok(decoded) = <OTHER_BRIDGECall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OTHER_BRIDGE(decoded));
            }
            if let Ok(decoded) = <BridgeERC721Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BridgeERC721(decoded));
            }
            if let Ok(decoded) = <BridgeERC721ToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BridgeERC721To(decoded));
            }
            if let Ok(decoded) = <DepositsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposits(decoded));
            }
            if let Ok(decoded) = <FinalizeBridgeERC721Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FinalizeBridgeERC721(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <messengerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::messenger(decoded));
            }
            if let Ok(decoded) = <otherBridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::otherBridge(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L1ERC721BridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MESSENGER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OTHER_BRIDGE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeERC721(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeERC721To(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizeBridgeERC721(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::messenger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::otherBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for L1ERC721BridgeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MESSENGER(element) => ::core::fmt::Display::fmt(element, f),
                Self::OTHER_BRIDGE(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeERC721(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeERC721To(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizeBridgeERC721(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::messenger(element) => ::core::fmt::Display::fmt(element, f),
                Self::otherBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MESSENGERCall> for L1ERC721BridgeCalls {
        fn from(value: MESSENGERCall) -> Self {
            Self::MESSENGER(value)
        }
    }
    impl ::core::convert::From<OTHER_BRIDGECall> for L1ERC721BridgeCalls {
        fn from(value: OTHER_BRIDGECall) -> Self {
            Self::OTHER_BRIDGE(value)
        }
    }
    impl ::core::convert::From<BridgeERC721Call> for L1ERC721BridgeCalls {
        fn from(value: BridgeERC721Call) -> Self {
            Self::BridgeERC721(value)
        }
    }
    impl ::core::convert::From<BridgeERC721ToCall> for L1ERC721BridgeCalls {
        fn from(value: BridgeERC721ToCall) -> Self {
            Self::BridgeERC721To(value)
        }
    }
    impl ::core::convert::From<DepositsCall> for L1ERC721BridgeCalls {
        fn from(value: DepositsCall) -> Self {
            Self::Deposits(value)
        }
    }
    impl ::core::convert::From<FinalizeBridgeERC721Call> for L1ERC721BridgeCalls {
        fn from(value: FinalizeBridgeERC721Call) -> Self {
            Self::FinalizeBridgeERC721(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for L1ERC721BridgeCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<messengerCall> for L1ERC721BridgeCalls {
        fn from(value: messengerCall) -> Self {
            Self::messenger(value)
        }
    }
    impl ::core::convert::From<otherBridgeCall> for L1ERC721BridgeCalls {
        fn from(value: otherBridgeCall) -> Self {
            Self::otherBridge(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L1ERC721BridgeCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `MESSENGER` function with signature `MESSENGER()` and selector `0x927ede2d`
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
    pub struct MESSENGERReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `OTHER_BRIDGE` function with signature `OTHER_BRIDGE()` and selector `0x7f46ddb2`
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
    pub struct OTHER_BRIDGEReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deposits` function with signature `deposits(address,address,uint256)` and selector `0x5d93a3fc`
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
    pub struct DepositsReturn(pub bool);
    ///Container type for all return fields from the `messenger` function with signature `messenger()` and selector `0x3cb747bf`
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
    pub struct messengerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `otherBridge` function with signature `otherBridge()` and selector `0xc89701a2`
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
    pub struct otherBridgeReturn(pub ::ethers::core::types::Address);
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

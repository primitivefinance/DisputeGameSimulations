pub use proxy_admin::*;
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
pub mod proxy_admin {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addressManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract AddressManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeProxyAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeProxyAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("getProxyAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProxyAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getProxyImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProxyImplementation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("implementationName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("implementationName"),
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
                    ::std::borrow::ToOwned::to_owned("isUpgrading"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isUpgrading"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("proxyType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxyType"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProxyAdmin.ProxyType",
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setAddressManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAddressManager"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract AddressManager"),
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
                    ::std::borrow::ToOwned::to_owned("setImplementationName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setImplementationName",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProxyType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setProxyType"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_type"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProxyAdmin.ProxyType",
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
                    ::std::borrow::ToOwned::to_owned("setUpgrading"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUpgrading"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_upgrading"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("upgrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgrade"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("upgradeAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
    pub static PROXYADMIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1A_8\x03\x80b\0\x1A_\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xA1V[b\0\0?3b\0\0QV[b\0\0J\x81b\0\0QV[Pb\0\0\xD3V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0` \x82\x84\x03\x12\x15b\0\0\xB4W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xCCW`\0\x80\xFD[\x93\x92PPPV[a\x19|\x80b\0\0\xE3`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x0EW`\x005`\xE0\x1C\x80c\x86\x0F|\xDA\x11a\0\xA5W\x80c\x99\xA8\x8E\xC4\x11a\0tW\x80c\xB7\x94rb\x11a\0YW\x80c\xB7\x94rb\x14a\x03)W\x80c\xF2\xFD\xE3\x8B\x14a\x03dW\x80c\xF3\xB7\xDE\xAD\x14a\x03\x84W`\0\x80\xFD[\x80c\x99\xA8\x8E\xC4\x14a\x02\xE9W\x80c\x9B.\xA4\xBD\x14a\x03\tW`\0\x80\xFD[\x80c\x86\x0F|\xDA\x14a\x02kW\x80c\x8DR\xD4\xA0\x14a\x02\x8BW\x80c\x8D\xA5\xCB[\x14a\x02\xABW\x80c\x96#`\x9D\x14a\x02\xD6W`\0\x80\xFD[\x80c:\xB7n\x9F\x11a\0\xE1W\x80c:\xB7n\x9F\x14a\x01\xCCW\x80ck\xD9\xF5\x16\x14a\x01\xF9W\x80cqP\x18\xA6\x14a\x026W\x80c~\xFF'^\x14a\x02KW`\0\x80\xFD[\x80c\x06R\xB5z\x14a\x01\x13W\x80c\x07\xC8\xF7\xB0\x14a\x015W\x80c N\x1Cz\x14a\x01UW\x80c#\x81\x81\xAE\x14a\x01\x9FW[`\0\x80\xFD[4\x80\x15a\x01\x1FW`\0\x80\xFD[Pa\x013a\x01.6`\x04a\x11\xF9V[a\x03\xA4V[\0[4\x80\x15a\x01AW`\0\x80\xFD[Pa\x013a\x01P6`\x04a\x12\x16V[a\x03\xF3V[4\x80\x15a\x01aW`\0\x80\xFD[Pa\x01ua\x01p6`\x04a\x11\xF9V[a\x04EV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\x01\xBFa\x01\xBA6`\x04a\x11\xF9V[a\x06kV[`@Qa\x01\x96\x91\x90a\x12\xAEV[4\x80\x15a\x01\xD8W`\0\x80\xFD[P`\x03Ta\x01u\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\x05W`\0\x80\xFD[Pa\x02)a\x02\x146`\x04a\x11\xF9V[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x01\x96\x91\x90a\x12\xF0V[4\x80\x15a\x02BW`\0\x80\xFD[Pa\x013a\x07\x05V[4\x80\x15a\x02WW`\0\x80\xFD[Pa\x013a\x02f6`\x04a\x131V[a\x07\x19V[4\x80\x15a\x02wW`\0\x80\xFD[Pa\x013a\x02\x866`\x04a\x14\x8CV[a\x08\xCCV[4\x80\x15a\x02\x97W`\0\x80\xFD[Pa\x013a\x02\xA66`\x04a\x14\xDCV[a\t\x03V[4\x80\x15a\x02\xB7W`\0\x80\xFD[P`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01uV[a\x013a\x02\xE46`\x04a\x15\x0EV[a\twV[4\x80\x15a\x02\xF5W`\0\x80\xFD[Pa\x013a\x03\x046`\x04a\x131V[a\x0B\x8EV[4\x80\x15a\x03\x15W`\0\x80\xFD[Pa\x013a\x03$6`\x04a\x15\x84V[a\x0E\x1EV[4\x80\x15a\x035W`\0\x80\xFD[P`\x03Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`@Q\x90\x15\x15\x81R` \x01a\x01\x96V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x013a\x03\x7F6`\x04a\x11\xF9V[a\x0E\xB4V[4\x80\x15a\x03\x90W`\0\x80\xFD[Pa\x01ua\x03\x9F6`\x04a\x11\xF9V[a\x0FkV[a\x03\xACa\x10\xE1V[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03\xFBa\x10\xE1V[`\x03\x80T\x91\x15\x15t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x81\x81`\x02\x81\x11\x15a\x04\x81Wa\x04\x81a\x12\xC1V[\x03a\x04\xFCW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF5\x91\x90a\x15\xCBV[\x93\x92PPPV[`\x01\x81`\x02\x81\x11\x15a\x05\x10Wa\x05\x10a\x12\xC1V[\x03a\x05`W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAA\xF1\x0FB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\x02\x81`\x02\x81\x11\x15a\x05tWa\x05ta\x12\xC1V[\x03a\x05\xFEW`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\0\x90\x81R`\x02` R`@\x90\x81\x90 \x90Q\x7F\xBF@\xFA\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x90\x92\x16\x91c\xBF@\xFA\xC1\x91a\x05\xE1\x91\x90`\x04\x01a\x165V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FProxyAdmin: unknown proxy type\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x91\x90PV[`\x02` R`\0\x90\x81R`@\x90 \x80Ta\x06\x84\x90a\x15\xE8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xB0\x90a\x15\xE8V[\x80\x15a\x06\xFDW\x80`\x1F\x10a\x06\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\x07\ra\x10\xE1V[a\x07\x17`\0a\x11bV[V[a\x07!a\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x90\x81`\x02\x81\x11\x15a\x07]Wa\x07]a\x12\xC1V[\x03a\x07\xE9W`@Q\x7F\x8F(9p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90c\x8F(9p\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xE0W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x01\x81`\x02\x81\x11\x15a\x07\xFDWa\x07\xFDa\x12\xC1V[\x03a\x08VW`@Q\x7F\x13\xAF@5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90c\x13\xAF@5\x90`$\x01a\x07\xB2V[`\x02\x81`\x02\x81\x11\x15a\x08jWa\x08ja\x12\xC1V[\x03a\x05\xFEW`\x03T`@Q\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xF2\xFD\xE3\x8B\x90`$\x01a\x07\xB2V[PPPV[a\x08\xD4a\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x02` R`@\x90 a\x08\xC7\x82\x82a\x17$V[a\t\x0Ba\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\tnWa\tna\x12\xC1V[\x02\x17\x90UPPPV[a\t\x7Fa\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x90\x81`\x02\x81\x11\x15a\t\xBBWa\t\xBBa\x12\xC1V[\x03a\n\x81W`@Q\x7FO\x1E\xF2\x86\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90cO\x1E\xF2\x86\x904\x90a\n\x16\x90\x87\x90\x87\x90`\x04\x01a\x18>V[`\0`@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\n4W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\n{\x91\x90\x81\x01\x90a\x18uV[Pa\x0B\x88V[a\n\x8B\x84\x84a\x0B\x8EV[`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x164\x84`@Qa\n\xB3\x91\x90a\x18\xECV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\xF0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xF5V[``\x91P[PP\x90P\x80a\x0B\x86W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FProxyAdmin: call to proxy after `D\x82\x01R\x7Fupgrade failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\\V[P[PPPPV[a\x0B\x96a\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x90\x81`\x02\x81\x11\x15a\x0B\xD2Wa\x0B\xD2a\x12\xC1V[\x03a\x0C+W`@Q\x7F6Y\xCF\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90c6Y\xCF\xE6\x90`$\x01a\x07\xB2V[`\x01\x81`\x02\x81\x11\x15a\x0C?Wa\x0C?a\x12\xC1V[\x03a\x0C\xBEW`@Q\x7F\x9B\x0B\x0F\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`$\x83\x01R\x84\x16\x90c\x9B\x0B\x0F\xDA\x90`D\x01a\x07\xB2V[`\x02\x81`\x02\x81\x11\x15a\x0C\xD2Wa\x0C\xD2a\x12\xC1V[\x03a\x0E\x16Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x02` R`@\x81 \x80Ta\r\x07\x90a\x15\xE8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r3\x90a\x15\xE8V[\x80\x15a\r\x80W\x80`\x1F\x10a\rUWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\rcW\x82\x90\x03`\x1F\x16\x82\x01\x91[PP`\x03T`@Q\x7F\x9B.\xA4\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x94\x95Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93c\x9B.\xA4\xBD\x93Pa\r\xDE\x92P\x85\x91P\x87\x90`\x04\x01a\x19\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xF8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0CW=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x08\xC7a\x19@V[a\x0E&a\x10\xE1V[`\x03T`@Q\x7F\x9B.\xA4\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x9B.\xA4\xBD\x90a\x0E~\x90\x85\x90\x85\x90`\x04\x01a\x19\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xACW=`\0\x80>=`\0\xFD[PPPPPPV[a\x0E\xBCa\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0F_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\\V[a\x0Fh\x81a\x11bV[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x81\x81`\x02\x81\x11\x15a\x0F\xA7Wa\x0F\xA7a\x12\xC1V[\x03a\x0F\xF7W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF8Q\xA4@`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\x01\x81`\x02\x81\x11\x15a\x10\x0BWa\x10\x0Ba\x12\xC1V[\x03a\x10[W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x89= \xE8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\x02\x81`\x02\x81\x11\x15a\x10oWa\x10oa\x12\xC1V[\x03a\x05\xFEW`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\x17W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\\V[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0FhW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12\x0BW`\0\x80\xFD[\x815a\x04\xF5\x81a\x11\xD7V[`\0` \x82\x84\x03\x12\x15a\x12(W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x04\xF5W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x12SW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12;V[\x83\x81\x11\x15a\x0B\x88WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12|\x81` \x86\x01` \x86\x01a\x128V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xF5` \x83\x01\x84a\x12dV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x13+W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x13DW`\0\x80\xFD[\x825a\x13O\x81a\x11\xD7V[\x91P` \x83\x015a\x13_\x81a\x11\xD7V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\xE0Wa\x13\xE0a\x13jV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\x02Wa\x14\x02a\x13jV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0a\x14Aa\x14<\x84a\x13\xE8V[a\x13\x99V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\x14UW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14}W`\0\x80\xFD[a\x04\xF5\x83\x835` \x85\x01a\x14.V[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x9FW`\0\x80\xFD[\x825a\x14\xAA\x81a\x11\xD7V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xC6W`\0\x80\xFD[a\x14\xD2\x85\x82\x86\x01a\x14lV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xEFW`\0\x80\xFD[\x825a\x14\xFA\x81a\x11\xD7V[\x91P` \x83\x015`\x03\x81\x10a\x13_W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15#W`\0\x80\xFD[\x835a\x15.\x81a\x11\xD7V[\x92P` \x84\x015a\x15>\x81a\x11\xD7V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15ZW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x15kW`\0\x80\xFD[a\x15z\x86\x825` \x84\x01a\x14.V[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x97W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xAEW`\0\x80\xFD[a\x15\xBA\x85\x82\x86\x01a\x14lV[\x92PP` \x83\x015a\x13_\x81a\x11\xD7V[`\0` \x82\x84\x03\x12\x15a\x15\xDDW`\0\x80\xFD[\x81Qa\x04\xF5\x81a\x11\xD7V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x15\xFCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06eW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0` \x80\x83R`\0\x84Ta\x16I\x81a\x15\xE8V[\x80\x84\x87\x01R`@`\x01\x80\x84\x16`\0\x81\x14a\x16jW`\x01\x81\x14a\x16\xA2Wa\x16\xD0V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x83\x8A\x01R\x82\x84\x15\x15`\x05\x1B\x8A\x01\x01\x95Pa\x16\xD0V[\x89`\0R\x86`\0 `\0[\x85\x81\x10\x15a\x16\xC8W\x81T\x8B\x82\x01\x86\x01R\x90\x83\x01\x90\x88\x01a\x16\xADV[\x8A\x01\x84\x01\x96PP[P\x93\x98\x97PPPPPPPPV[`\x1F\x82\x11\x15a\x08\xC7W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x17\x05WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\xACW\x82\x81U`\x01\x01a\x17\x11V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17>Wa\x17>a\x13jV[a\x17R\x81a\x17L\x84Ta\x15\xE8V[\x84a\x16\xDEV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x17\xA5W`\0\x84\x15a\x17oWP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0E\xACV[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x17\xF2W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x17\xD3V[P\x85\x82\x10\x15a\x18.W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x18m`@\x83\x01\x84a\x12dV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x18\x87W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x9EW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x18\xAFW`\0\x80\xFD[\x80Qa\x18\xBDa\x14<\x82a\x13\xE8V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x18\xD2W`\0\x80\xFD[a\x18\xE3\x82` \x83\x01` \x86\x01a\x128V[\x95\x94PPPPPV[`\0\x82Qa\x18\xFE\x81\x84` \x87\x01a\x128V[\x91\x90\x91\x01\x92\x91PPV[`@\x81R`\0a\x19\x1B`@\x83\x01\x85a\x12dV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static PROXYADMIN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x0EW`\x005`\xE0\x1C\x80c\x86\x0F|\xDA\x11a\0\xA5W\x80c\x99\xA8\x8E\xC4\x11a\0tW\x80c\xB7\x94rb\x11a\0YW\x80c\xB7\x94rb\x14a\x03)W\x80c\xF2\xFD\xE3\x8B\x14a\x03dW\x80c\xF3\xB7\xDE\xAD\x14a\x03\x84W`\0\x80\xFD[\x80c\x99\xA8\x8E\xC4\x14a\x02\xE9W\x80c\x9B.\xA4\xBD\x14a\x03\tW`\0\x80\xFD[\x80c\x86\x0F|\xDA\x14a\x02kW\x80c\x8DR\xD4\xA0\x14a\x02\x8BW\x80c\x8D\xA5\xCB[\x14a\x02\xABW\x80c\x96#`\x9D\x14a\x02\xD6W`\0\x80\xFD[\x80c:\xB7n\x9F\x11a\0\xE1W\x80c:\xB7n\x9F\x14a\x01\xCCW\x80ck\xD9\xF5\x16\x14a\x01\xF9W\x80cqP\x18\xA6\x14a\x026W\x80c~\xFF'^\x14a\x02KW`\0\x80\xFD[\x80c\x06R\xB5z\x14a\x01\x13W\x80c\x07\xC8\xF7\xB0\x14a\x015W\x80c N\x1Cz\x14a\x01UW\x80c#\x81\x81\xAE\x14a\x01\x9FW[`\0\x80\xFD[4\x80\x15a\x01\x1FW`\0\x80\xFD[Pa\x013a\x01.6`\x04a\x11\xF9V[a\x03\xA4V[\0[4\x80\x15a\x01AW`\0\x80\xFD[Pa\x013a\x01P6`\x04a\x12\x16V[a\x03\xF3V[4\x80\x15a\x01aW`\0\x80\xFD[Pa\x01ua\x01p6`\x04a\x11\xF9V[a\x04EV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\x01\xBFa\x01\xBA6`\x04a\x11\xF9V[a\x06kV[`@Qa\x01\x96\x91\x90a\x12\xAEV[4\x80\x15a\x01\xD8W`\0\x80\xFD[P`\x03Ta\x01u\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\x05W`\0\x80\xFD[Pa\x02)a\x02\x146`\x04a\x11\xF9V[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x01\x96\x91\x90a\x12\xF0V[4\x80\x15a\x02BW`\0\x80\xFD[Pa\x013a\x07\x05V[4\x80\x15a\x02WW`\0\x80\xFD[Pa\x013a\x02f6`\x04a\x131V[a\x07\x19V[4\x80\x15a\x02wW`\0\x80\xFD[Pa\x013a\x02\x866`\x04a\x14\x8CV[a\x08\xCCV[4\x80\x15a\x02\x97W`\0\x80\xFD[Pa\x013a\x02\xA66`\x04a\x14\xDCV[a\t\x03V[4\x80\x15a\x02\xB7W`\0\x80\xFD[P`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01uV[a\x013a\x02\xE46`\x04a\x15\x0EV[a\twV[4\x80\x15a\x02\xF5W`\0\x80\xFD[Pa\x013a\x03\x046`\x04a\x131V[a\x0B\x8EV[4\x80\x15a\x03\x15W`\0\x80\xFD[Pa\x013a\x03$6`\x04a\x15\x84V[a\x0E\x1EV[4\x80\x15a\x035W`\0\x80\xFD[P`\x03Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16`@Q\x90\x15\x15\x81R` \x01a\x01\x96V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x013a\x03\x7F6`\x04a\x11\xF9V[a\x0E\xB4V[4\x80\x15a\x03\x90W`\0\x80\xFD[Pa\x01ua\x03\x9F6`\x04a\x11\xF9V[a\x0FkV[a\x03\xACa\x10\xE1V[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03\xFBa\x10\xE1V[`\x03\x80T\x91\x15\x15t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x81\x81`\x02\x81\x11\x15a\x04\x81Wa\x04\x81a\x12\xC1V[\x03a\x04\xFCW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF5\x91\x90a\x15\xCBV[\x93\x92PPPV[`\x01\x81`\x02\x81\x11\x15a\x05\x10Wa\x05\x10a\x12\xC1V[\x03a\x05`W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xAA\xF1\x0FB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\x02\x81`\x02\x81\x11\x15a\x05tWa\x05ta\x12\xC1V[\x03a\x05\xFEW`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\0\x90\x81R`\x02` R`@\x90\x81\x90 \x90Q\x7F\xBF@\xFA\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x90\x92\x16\x91c\xBF@\xFA\xC1\x91a\x05\xE1\x91\x90`\x04\x01a\x165V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FProxyAdmin: unknown proxy type\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[P\x91\x90PV[`\x02` R`\0\x90\x81R`@\x90 \x80Ta\x06\x84\x90a\x15\xE8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xB0\x90a\x15\xE8V[\x80\x15a\x06\xFDW\x80`\x1F\x10a\x06\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\x07\ra\x10\xE1V[a\x07\x17`\0a\x11bV[V[a\x07!a\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x90\x81`\x02\x81\x11\x15a\x07]Wa\x07]a\x12\xC1V[\x03a\x07\xE9W`@Q\x7F\x8F(9p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90c\x8F(9p\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xE0W=`\0\x80>=`\0\xFD[PPPPPPPV[`\x01\x81`\x02\x81\x11\x15a\x07\xFDWa\x07\xFDa\x12\xC1V[\x03a\x08VW`@Q\x7F\x13\xAF@5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90c\x13\xAF@5\x90`$\x01a\x07\xB2V[`\x02\x81`\x02\x81\x11\x15a\x08jWa\x08ja\x12\xC1V[\x03a\x05\xFEW`\x03T`@Q\x7F\xF2\xFD\xE3\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xF2\xFD\xE3\x8B\x90`$\x01a\x07\xB2V[PPPV[a\x08\xD4a\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x02` R`@\x90 a\x08\xC7\x82\x82a\x17$V[a\t\x0Ba\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\tnWa\tna\x12\xC1V[\x02\x17\x90UPPPV[a\t\x7Fa\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x90\x81`\x02\x81\x11\x15a\t\xBBWa\t\xBBa\x12\xC1V[\x03a\n\x81W`@Q\x7FO\x1E\xF2\x86\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90cO\x1E\xF2\x86\x904\x90a\n\x16\x90\x87\x90\x87\x90`\x04\x01a\x18>V[`\0`@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\n4W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\n{\x91\x90\x81\x01\x90a\x18uV[Pa\x0B\x88V[a\n\x8B\x84\x84a\x0B\x8EV[`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x164\x84`@Qa\n\xB3\x91\x90a\x18\xECV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n\xF0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xF5V[``\x91P[PP\x90P\x80a\x0B\x86W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FProxyAdmin: call to proxy after `D\x82\x01R\x7Fupgrade failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\\V[P[PPPPV[a\x0B\x96a\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x90\x81`\x02\x81\x11\x15a\x0B\xD2Wa\x0B\xD2a\x12\xC1V[\x03a\x0C+W`@Q\x7F6Y\xCF\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90c6Y\xCF\xE6\x90`$\x01a\x07\xB2V[`\x01\x81`\x02\x81\x11\x15a\x0C?Wa\x0C?a\x12\xC1V[\x03a\x0C\xBEW`@Q\x7F\x9B\x0B\x0F\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`$\x83\x01R\x84\x16\x90c\x9B\x0B\x0F\xDA\x90`D\x01a\x07\xB2V[`\x02\x81`\x02\x81\x11\x15a\x0C\xD2Wa\x0C\xD2a\x12\xC1V[\x03a\x0E\x16Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x02` R`@\x81 \x80Ta\r\x07\x90a\x15\xE8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r3\x90a\x15\xE8V[\x80\x15a\r\x80W\x80`\x1F\x10a\rUWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\rcW\x82\x90\x03`\x1F\x16\x82\x01\x91[PP`\x03T`@Q\x7F\x9B.\xA4\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x94\x95Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93c\x9B.\xA4\xBD\x93Pa\r\xDE\x92P\x85\x91P\x87\x90`\x04\x01a\x19\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xF8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0CW=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x08\xC7a\x19@V[a\x0E&a\x10\xE1V[`\x03T`@Q\x7F\x9B.\xA4\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\x9B.\xA4\xBD\x90a\x0E~\x90\x85\x90\x85\x90`\x04\x01a\x19\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xACW=`\0\x80>=`\0\xFD[PPPPPPV[a\x0E\xBCa\x10\xE1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0F_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\\V[a\x0Fh\x81a\x11bV[PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T`\xFF\x16\x81\x81`\x02\x81\x11\x15a\x0F\xA7Wa\x0F\xA7a\x12\xC1V[\x03a\x0F\xF7W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF8Q\xA4@`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\x01\x81`\x02\x81\x11\x15a\x10\x0BWa\x10\x0Ba\x12\xC1V[\x03a\x10[W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x89= \xE8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\x02\x81`\x02\x81\x11\x15a\x10oWa\x10oa\x12\xC1V[\x03a\x05\xFEW`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD1W=`\0\x80>=`\0\xFD[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\x17W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\\V[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0FhW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12\x0BW`\0\x80\xFD[\x815a\x04\xF5\x81a\x11\xD7V[`\0` \x82\x84\x03\x12\x15a\x12(W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x04\xF5W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x12SW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12;V[\x83\x81\x11\x15a\x0B\x88WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12|\x81` \x86\x01` \x86\x01a\x128V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xF5` \x83\x01\x84a\x12dV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x13+W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x13DW`\0\x80\xFD[\x825a\x13O\x81a\x11\xD7V[\x91P` \x83\x015a\x13_\x81a\x11\xD7V[\x80\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\xE0Wa\x13\xE0a\x13jV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\x02Wa\x14\x02a\x13jV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0a\x14Aa\x14<\x84a\x13\xE8V[a\x13\x99V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\x14UW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14}W`\0\x80\xFD[a\x04\xF5\x83\x835` \x85\x01a\x14.V[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x9FW`\0\x80\xFD[\x825a\x14\xAA\x81a\x11\xD7V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xC6W`\0\x80\xFD[a\x14\xD2\x85\x82\x86\x01a\x14lV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xEFW`\0\x80\xFD[\x825a\x14\xFA\x81a\x11\xD7V[\x91P` \x83\x015`\x03\x81\x10a\x13_W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15#W`\0\x80\xFD[\x835a\x15.\x81a\x11\xD7V[\x92P` \x84\x015a\x15>\x81a\x11\xD7V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15ZW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x15kW`\0\x80\xFD[a\x15z\x86\x825` \x84\x01a\x14.V[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x97W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xAEW`\0\x80\xFD[a\x15\xBA\x85\x82\x86\x01a\x14lV[\x92PP` \x83\x015a\x13_\x81a\x11\xD7V[`\0` \x82\x84\x03\x12\x15a\x15\xDDW`\0\x80\xFD[\x81Qa\x04\xF5\x81a\x11\xD7V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x15\xFCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06eW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0` \x80\x83R`\0\x84Ta\x16I\x81a\x15\xE8V[\x80\x84\x87\x01R`@`\x01\x80\x84\x16`\0\x81\x14a\x16jW`\x01\x81\x14a\x16\xA2Wa\x16\xD0V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x83\x8A\x01R\x82\x84\x15\x15`\x05\x1B\x8A\x01\x01\x95Pa\x16\xD0V[\x89`\0R\x86`\0 `\0[\x85\x81\x10\x15a\x16\xC8W\x81T\x8B\x82\x01\x86\x01R\x90\x83\x01\x90\x88\x01a\x16\xADV[\x8A\x01\x84\x01\x96PP[P\x93\x98\x97PPPPPPPPV[`\x1F\x82\x11\x15a\x08\xC7W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x17\x05WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\xACW\x82\x81U`\x01\x01a\x17\x11V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17>Wa\x17>a\x13jV[a\x17R\x81a\x17L\x84Ta\x15\xE8V[\x84a\x16\xDEV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x17\xA5W`\0\x84\x15a\x17oWP\x85\x83\x01Q[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0E\xACV[`\0\x85\x81R` \x81 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x91[\x82\x81\x10\x15a\x17\xF2W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x17\xD3V[P\x85\x82\x10\x15a\x18.W\x87\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x18m`@\x83\x01\x84a\x12dV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x18\x87W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x9EW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x18\xAFW`\0\x80\xFD[\x80Qa\x18\xBDa\x14<\x82a\x13\xE8V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x18\xD2W`\0\x80\xFD[a\x18\xE3\x82` \x83\x01` \x86\x01a\x128V[\x95\x94PPPPPV[`\0\x82Qa\x18\xFE\x81\x84` \x87\x01a\x128V[\x91\x90\x91\x01\x92\x91PPV[`@\x81R`\0a\x19\x1B`@\x83\x01\x85a\x12dV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x01`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static PROXYADMIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ProxyAdmin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ProxyAdmin<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ProxyAdmin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ProxyAdmin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ProxyAdmin<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ProxyAdmin)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ProxyAdmin<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PROXYADMIN_ABI.clone(),
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
                PROXYADMIN_ABI.clone(),
                PROXYADMIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addressManager` (0x3ab76e9f) function
        pub fn address_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([58, 183, 110, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeProxyAdmin` (0x7eff275e) function
        pub fn change_proxy_admin(
            &self,
            proxy: ::ethers::core::types::Address,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 255, 39, 94], (proxy, new_admin))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProxyAdmin` (0xf3b7dead) function
        pub fn get_proxy_admin(
            &self,
            proxy: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([243, 183, 222, 173], proxy)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProxyImplementation` (0x204e1c7a) function
        pub fn get_proxy_implementation(
            &self,
            proxy: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([32, 78, 28, 122], proxy)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementationName` (0x238181ae) function
        pub fn implementation_name(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([35, 129, 129, 174], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isUpgrading` (0xb7947262) function
        pub fn is_upgrading(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 148, 114, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyType` (0x6bd9f516) function
        pub fn proxy_type(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([107, 217, 245, 22], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAddress` (0x9b2ea4bd) function
        pub fn set_address(
            &self,
            name: ::std::string::String,
            address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 46, 164, 189], (name, address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAddressManager` (0x0652b57a) function
        pub fn set_address_manager(
            &self,
            address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 82, 181, 122], address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setImplementationName` (0x860f7cda) function
        pub fn set_implementation_name(
            &self,
            address: ::ethers::core::types::Address,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 15, 124, 218], (address, name))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProxyType` (0x8d52d4a0) function
        pub fn set_proxy_type(
            &self,
            address: ::ethers::core::types::Address,
            type_: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 82, 212, 160], (address, type_))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUpgrading` (0x07c8f7b0) function
        pub fn set_upgrading(
            &self,
            upgrading: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 200, 247, 176], upgrading)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrade` (0x99a88ec4) function
        pub fn upgrade(
            &self,
            proxy: ::ethers::core::types::Address,
            implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 168, 142, 196], (proxy, implementation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeAndCall` (0x9623609d) function
        pub fn upgrade_and_call(
            &self,
            proxy: ::ethers::core::types::Address,
            implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 35, 96, 157], (proxy, implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ProxyAdmin<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addressManager` function with signature `addressManager()` and selector `0x3ab76e9f`
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
    #[ethcall(name = "addressManager", abi = "addressManager()")]
    pub struct AddressManagerCall;
    ///Container type for all input parameters for the `changeProxyAdmin` function with signature `changeProxyAdmin(address,address)` and selector `0x7eff275e`
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
    #[ethcall(name = "changeProxyAdmin", abi = "changeProxyAdmin(address,address)")]
    pub struct ChangeProxyAdminCall {
        pub proxy: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getProxyAdmin` function with signature `getProxyAdmin(address)` and selector `0xf3b7dead`
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
    #[ethcall(name = "getProxyAdmin", abi = "getProxyAdmin(address)")]
    pub struct GetProxyAdminCall {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getProxyImplementation` function with signature `getProxyImplementation(address)` and selector `0x204e1c7a`
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
    #[ethcall(name = "getProxyImplementation", abi = "getProxyImplementation(address)")]
    pub struct GetProxyImplementationCall {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `implementationName` function with signature `implementationName(address)` and selector `0x238181ae`
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
    #[ethcall(name = "implementationName", abi = "implementationName(address)")]
    pub struct ImplementationNameCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isUpgrading` function with signature `isUpgrading()` and selector `0xb7947262`
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
    #[ethcall(name = "isUpgrading", abi = "isUpgrading()")]
    pub struct IsUpgradingCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `proxyType` function with signature `proxyType(address)` and selector `0x6bd9f516`
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
    #[ethcall(name = "proxyType", abi = "proxyType(address)")]
    pub struct ProxyTypeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setAddress` function with signature `setAddress(string,address)` and selector `0x9b2ea4bd`
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
    #[ethcall(name = "setAddress", abi = "setAddress(string,address)")]
    pub struct SetAddressCall {
        pub name: ::std::string::String,
        pub address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setAddressManager` function with signature `setAddressManager(address)` and selector `0x0652b57a`
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
    #[ethcall(name = "setAddressManager", abi = "setAddressManager(address)")]
    pub struct SetAddressManagerCall {
        pub address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setImplementationName` function with signature `setImplementationName(address,string)` and selector `0x860f7cda`
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
        name = "setImplementationName",
        abi = "setImplementationName(address,string)"
    )]
    pub struct SetImplementationNameCall {
        pub address: ::ethers::core::types::Address,
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `setProxyType` function with signature `setProxyType(address,uint8)` and selector `0x8d52d4a0`
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
    #[ethcall(name = "setProxyType", abi = "setProxyType(address,uint8)")]
    pub struct SetProxyTypeCall {
        pub address: ::ethers::core::types::Address,
        pub type_: u8,
    }
    ///Container type for all input parameters for the `setUpgrading` function with signature `setUpgrading(bool)` and selector `0x07c8f7b0`
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
    #[ethcall(name = "setUpgrading", abi = "setUpgrading(bool)")]
    pub struct SetUpgradingCall {
        pub upgrading: bool,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(address,address)` and selector `0x99a88ec4`
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
    #[ethcall(name = "upgrade", abi = "upgrade(address,address)")]
    pub struct UpgradeCall {
        pub proxy: ::ethers::core::types::Address,
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeAndCall` function with signature `upgradeAndCall(address,address,bytes)` and selector `0x9623609d`
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
    #[ethcall(name = "upgradeAndCall", abi = "upgradeAndCall(address,address,bytes)")]
    pub struct UpgradeAndCallCall {
        pub proxy: ::ethers::core::types::Address,
        pub implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ProxyAdminCalls {
        AddressManager(AddressManagerCall),
        ChangeProxyAdmin(ChangeProxyAdminCall),
        GetProxyAdmin(GetProxyAdminCall),
        GetProxyImplementation(GetProxyImplementationCall),
        ImplementationName(ImplementationNameCall),
        IsUpgrading(IsUpgradingCall),
        Owner(OwnerCall),
        ProxyType(ProxyTypeCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetAddress(SetAddressCall),
        SetAddressManager(SetAddressManagerCall),
        SetImplementationName(SetImplementationNameCall),
        SetProxyType(SetProxyTypeCall),
        SetUpgrading(SetUpgradingCall),
        TransferOwnership(TransferOwnershipCall),
        Upgrade(UpgradeCall),
        UpgradeAndCall(UpgradeAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for ProxyAdminCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddressManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressManager(decoded));
            }
            if let Ok(decoded) = <ChangeProxyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeProxyAdmin(decoded));
            }
            if let Ok(decoded) = <GetProxyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProxyAdmin(decoded));
            }
            if let Ok(decoded) = <GetProxyImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProxyImplementation(decoded));
            }
            if let Ok(decoded) = <ImplementationNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ImplementationName(decoded));
            }
            if let Ok(decoded) = <IsUpgradingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsUpgrading(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxyTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxyType(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAddress(decoded));
            }
            if let Ok(decoded) = <SetAddressManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAddressManager(decoded));
            }
            if let Ok(decoded) = <SetImplementationNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetImplementationName(decoded));
            }
            if let Ok(decoded) = <SetProxyTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProxyType(decoded));
            }
            if let Ok(decoded) = <SetUpgradingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUpgrading(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Upgrade(decoded));
            }
            if let Ok(decoded) = <UpgradeAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ProxyAdminCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeProxyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProxyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProxyImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ImplementationName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsUpgrading(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxyType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAddressManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetImplementationName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProxyType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUpgrading(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ProxyAdminCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeProxyAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProxyAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProxyImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ImplementationName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsUpgrading(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyType(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddressManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetImplementationName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetProxyType(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUpgrading(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressManagerCall> for ProxyAdminCalls {
        fn from(value: AddressManagerCall) -> Self {
            Self::AddressManager(value)
        }
    }
    impl ::core::convert::From<ChangeProxyAdminCall> for ProxyAdminCalls {
        fn from(value: ChangeProxyAdminCall) -> Self {
            Self::ChangeProxyAdmin(value)
        }
    }
    impl ::core::convert::From<GetProxyAdminCall> for ProxyAdminCalls {
        fn from(value: GetProxyAdminCall) -> Self {
            Self::GetProxyAdmin(value)
        }
    }
    impl ::core::convert::From<GetProxyImplementationCall> for ProxyAdminCalls {
        fn from(value: GetProxyImplementationCall) -> Self {
            Self::GetProxyImplementation(value)
        }
    }
    impl ::core::convert::From<ImplementationNameCall> for ProxyAdminCalls {
        fn from(value: ImplementationNameCall) -> Self {
            Self::ImplementationName(value)
        }
    }
    impl ::core::convert::From<IsUpgradingCall> for ProxyAdminCalls {
        fn from(value: IsUpgradingCall) -> Self {
            Self::IsUpgrading(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ProxyAdminCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxyTypeCall> for ProxyAdminCalls {
        fn from(value: ProxyTypeCall) -> Self {
            Self::ProxyType(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ProxyAdminCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetAddressCall> for ProxyAdminCalls {
        fn from(value: SetAddressCall) -> Self {
            Self::SetAddress(value)
        }
    }
    impl ::core::convert::From<SetAddressManagerCall> for ProxyAdminCalls {
        fn from(value: SetAddressManagerCall) -> Self {
            Self::SetAddressManager(value)
        }
    }
    impl ::core::convert::From<SetImplementationNameCall> for ProxyAdminCalls {
        fn from(value: SetImplementationNameCall) -> Self {
            Self::SetImplementationName(value)
        }
    }
    impl ::core::convert::From<SetProxyTypeCall> for ProxyAdminCalls {
        fn from(value: SetProxyTypeCall) -> Self {
            Self::SetProxyType(value)
        }
    }
    impl ::core::convert::From<SetUpgradingCall> for ProxyAdminCalls {
        fn from(value: SetUpgradingCall) -> Self {
            Self::SetUpgrading(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ProxyAdminCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for ProxyAdminCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    impl ::core::convert::From<UpgradeAndCallCall> for ProxyAdminCalls {
        fn from(value: UpgradeAndCallCall) -> Self {
            Self::UpgradeAndCall(value)
        }
    }
    ///Container type for all return fields from the `addressManager` function with signature `addressManager()` and selector `0x3ab76e9f`
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
    pub struct AddressManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getProxyAdmin` function with signature `getProxyAdmin(address)` and selector `0xf3b7dead`
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
    pub struct GetProxyAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getProxyImplementation` function with signature `getProxyImplementation(address)` and selector `0x204e1c7a`
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
    pub struct GetProxyImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `implementationName` function with signature `implementationName(address)` and selector `0x238181ae`
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
    pub struct ImplementationNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `isUpgrading` function with signature `isUpgrading()` and selector `0xb7947262`
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
    pub struct IsUpgradingReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxyType` function with signature `proxyType(address)` and selector `0x6bd9f516`
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
    pub struct ProxyTypeReturn(pub u8);
}

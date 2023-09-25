pub use optimism_mintable_erc20_factory::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
    non_snake_case,
)]
pub mod optimism_mintable_erc20_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BRIDGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BRIDGE"),
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
                    ::std::borrow::ToOwned::to_owned("bridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridge"),
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
                    ::std::borrow::ToOwned::to_owned("createOptimismMintableERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createOptimismMintableERC20",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "createOptimismMintableERC20WithDecimals",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createOptimismMintableERC20WithDecimals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createStandardL2Token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createStandardL2Token",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteToken"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_bridge"),
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
                    ::std::borrow::ToOwned::to_owned("OptimismMintableERC20Created"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OptimismMintableERC20Created",
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
                                    name: ::std::borrow::ToOwned::to_owned("deployer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StandardL2TokenCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StandardL2TokenCreated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remoteToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("localToken"),
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
    pub static OPTIMISMMINTABLEERC20FACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x01`\x80R`\x04`\xA0R`\0`\xC0\x81\x90Ra\0+\x90a\x000V[a\x01(V[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\0RWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\0\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x01\0`\xFF\x84\x16a\xFF\xFF\x19\x90\x92\x16\x82\x17\x17a\x01\0`\x01`\xB0\x1B\x03\x19\x16a\xFF\0\x19b\x01\0\0`\x01`\x01`\xA0\x1B\x03\x87\x16\x02\x16\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\x80Q`\xA0Q`\xC0Qa'?a\x01W`\09`\0a\x01\xDC\x01R`\0a\x01\xB1\x01R`\0a\x01\x86\x01Ra'?`\0\xF3\xFE`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\0\x87W`\x005`\xE0\x1C\x80c\xC4\xD6m\xE8\x11b\0\0bW\x80c\xC4\xD6m\xE8\x14b\0\x01\x02W\x80c\xCEZ\xC9\x0F\x14b\0\x01\x1BW\x80c\xE7\x8C\xEA\x92\x14b\0\x012W\x80c\xEE\x9A1\xA2\x14b\0\x01YW`\0\x80\xFD[\x80cT\xFDMP\x14b\0\0\x8CW\x80c\x89o\x93\xD1\x14b\0\0\xAEW\x80c\x8C\xF0b\x9C\x14b\0\0\xEBW[`\0\x80\xFD[b\0\0\x96b\0\x01~V[`@Qb\0\0\xA5\x91\x90b\0\x07\xA5V[`@Q\x80\x91\x03\x90\xF3[b\0\0\xC5b\0\0\xBF6`\x04b\0\x08\xCDV[b\0\x02)V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01b\0\0\xA5V[b\0\0\xC5b\0\0\xFC6`\x04b\0\tJV[b\0\x02@V[b\0\x01\x19b\0\x01\x136`\x04b\0\t\xE1V[b\0\x04<V[\0[b\0\0\xC5b\0\x01,6`\x04b\0\x08\xCDV[b\0\x05\xBAV[`\0Tb\0\0\xC5\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\0\xC5V[``b\0\x01\xAB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x05\xCBV[b\0\x01\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x05\xCBV[b\0\x02\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x05\xCBV[`@Q` \x01b\0\x02\x15\x93\x92\x91\x90b\0\t\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0b\0\x028\x84\x84\x84b\0\x05\xBAV[\x94\x93PPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16b\0\x02\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FOptimismMintableERC20Factory: mu`D\x82\x01R\x7Fst provide remote token address\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x85\x85\x85`@Q` \x01b\0\x03\x04\x93\x92\x91\x90b\0\n{V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`\0`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x88\x88\x88`@Qb\0\x03T\x90b\0\x07\x18V[b\0\x03d\x95\x94\x93\x92\x91\x90b\0\n\xCAV[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x03\x85W=`\0\x80>=`\0\xFD[P\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCE\xEB\x8E}R\r\x7F;e\xFC\x11\xA2b\xB9\x10f\x94\x01\x93\xB0]O\x93\xDF\x07\xCF\xDC\xED\x0E\xB5Q\xCF`@Q`@Q\x80\x91\x03\x90\xA3`@Q3\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x91\x90\x83\x16\x90\x7FR\xFE\x89\xDDY0\xF3C\xD2VP\xB6/\xD3g\xBA\xE4p\x88\xBC\xDD\xFF\xD2\xA8\x83P\xA6\xEC\xDDb\x0C\xDB\x90` \x01`@Q\x80\x91\x03\x90\xA3\x96\x95PPPPPPV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15b\0\x04_WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[b\0\x04\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x02\xE2V[`\0\x80Ta\x01\0`\xFF\x84\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x90\x92\x16\x82\x17\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFFb\x01\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x02\x16\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\0b\0\x028\x84\x84\x84`\x12b\0\x02@V[``\x81`\0\x03b\0\x06\x0FWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15b\0\x06?W\x80b\0\x06&\x81b\0\x0B^V[\x91Pb\0\x067\x90P`\n\x83b\0\x0B\xC8V[\x91Pb\0\x06\x13V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x06]Wb\0\x06]b\0\x07\xEBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15b\0\x06\x88W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15b\0\x028Wb\0\x06\xA0`\x01\x83b\0\x0B\xDFV[\x91Pb\0\x06\xAF`\n\x86b\0\x0B\xF9V[b\0\x06\xBC\x90`0b\0\x0C\x10V[`\xF8\x1B\x81\x83\x81Q\x81\x10b\0\x06\xD4Wb\0\x06\xD4b\0\x0C+V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPb\0\x07\x10`\n\x86b\0\x0B\xC8V[\x94Pb\0\x06\x8CV[a\x1A\xD8\x80b\0\x0C[\x839\x01\x90V[`\0[\x83\x81\x10\x15b\0\x07CW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x07)V[\x83\x81\x11\x15b\0\x07SW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\0\x07s\x81` \x86\x01` \x86\x01b\0\x07&V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0\x07\xBA` \x83\x01\x84b\0\x07YV[\x93\x92PPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x07\xE6W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\x08,W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x08JWb\0\x08Jb\0\x07\xEBV[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x08\x93Wb\0\x08\x93b\0\x07\xEBV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x08\xADW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x08\xE3W`\0\x80\xFD[b\0\x08\xEE\x84b\0\x07\xC1V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\t\x0CW`\0\x80\xFD[b\0\t\x1A\x87\x83\x88\x01b\0\x08\x1AV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15b\0\t1W`\0\x80\xFD[Pb\0\t@\x86\x82\x87\x01b\0\x08\x1AV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\taW`\0\x80\xFD[b\0\tl\x85b\0\x07\xC1V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\t\x8AW`\0\x80\xFD[b\0\t\x98\x88\x83\x89\x01b\0\x08\x1AV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\0\t\xAFW`\0\x80\xFD[Pb\0\t\xBE\x87\x82\x88\x01b\0\x08\x1AV[\x92PP``\x85\x015`\xFF\x81\x16\x81\x14b\0\t\xD6W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15b\0\t\xF4W`\0\x80\xFD[b\0\x07\xBA\x82b\0\x07\xC1V[`\0\x84Qb\0\n\x13\x81\x84` \x89\x01b\0\x07&V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qb\0\nQ\x81`\x01\x85\x01` \x8A\x01b\0\x07&V[`\x01\x92\x01\x91\x82\x01R\x83Qb\0\nn\x81`\x02\x84\x01` \x88\x01b\0\x07&V[\x01`\x02\x01\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0b\0\n\xAC``\x83\x01\x85b\0\x07YV[\x82\x81\x03`@\x84\x01Rb\0\n\xC0\x81\x85b\0\x07YV[\x96\x95PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x83R\x80\x87\x16` \x84\x01RP`\xA0`@\x83\x01Rb\0\x0B\x05`\xA0\x83\x01\x86b\0\x07YV[\x82\x81\x03``\x84\x01Rb\0\x0B\x19\x81\x86b\0\x07YV[\x91PP`\xFF\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03b\0\x0B\x92Wb\0\x0B\x92b\0\x0B/V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82b\0\x0B\xDAWb\0\x0B\xDAb\0\x0B\x99V[P\x04\x90V[`\0\x82\x82\x10\x15b\0\x0B\xF4Wb\0\x0B\xF4b\0\x0B/V[P\x03\x90V[`\0\x82b\0\x0C\x0BWb\0\x0C\x0Bb\0\x0B\x99V[P\x06\x90V[`\0\x82\x19\x82\x11\x15b\0\x0C&Wb\0\x0C&b\0\x0B/V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFEa\x01@`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1A\xD88\x03\x80b\0\x1A\xD8\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01xV[`\x01`\x02`\0\x85\x85`\x03b\0\0K\x83\x82b\0\x02\xB3V[P`\x04b\0\0Z\x82\x82b\0\x02\xB3V[PPP`\x80\x92\x90\x92R`\xA0R`\xC0R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\xE0R\x93\x90\x92\x16a\x01\0RPP`\xFF\x16a\x01 Rb\0\x03\x7FV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xA6W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\0\xD3W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\xF0Wb\0\0\xF0b\0\0\xABV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x01\x1BWb\0\x01\x1Bb\0\0\xABV[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\x018W`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x01\\W\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\x01=V[\x83\x82\x11\x15b\0\x01nW`\0\x83\x85\x83\x01\x01R[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\x91W`\0\x80\xFD[b\0\x01\x9C\x86b\0\0\x8EV[\x94Pb\0\x01\xAC` \x87\x01b\0\0\x8EV[`@\x87\x01Q\x90\x94P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01\xCAW`\0\x80\xFD[b\0\x01\xD8\x89\x83\x8A\x01b\0\0\xC1V[\x94P``\x88\x01Q\x91P\x80\x82\x11\x15b\0\x01\xEFW`\0\x80\xFD[Pb\0\x01\xFE\x88\x82\x89\x01b\0\0\xC1V[\x92PP`\x80\x86\x01Q`\xFF\x81\x16\x81\x14b\0\x02\x16W`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x029W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02ZWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\xAEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\x89WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\xAAW\x82\x81U`\x01\x01b\0\x02\x95V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02\xCFWb\0\x02\xCFb\0\0\xABV[b\0\x02\xE7\x81b\0\x02\xE0\x84Tb\0\x02$V[\x84b\0\x02`V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03\x1FW`\0\x84\x15b\0\x03\x06WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\xAAV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03PW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03/V[P\x85\x82\x10\x15b\0\x03oW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x16\xEDb\0\x03\xEB`\09`\0a\x02D\x01R`\0\x81\x81a\x03\x17\x01R\x81\x81a\x03\xAC\x01R\x81\x81a\x05\xF1\x01Ra\x07\xCB\x01R`\0\x81\x81a\x01\xA9\x01Ra\x03=\x01R`\0a\x07Z\x01R`\0a\x071\x01R`\0a\x07\x08\x01Ra\x16\xED`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01wW`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\xD8W\x80c\xAE\x1Fj\xAF\x11a\0\x8CW\x80c\xDDb\xED>\x11a\0fW\x80c\xDDb\xED>\x14a\x03aW\x80c\xE7\x8C\xEA\x92\x14a\x03\x15W\x80c\xEE\x9A1\xA2\x14a\x03\xA7W`\0\x80\xFD[\x80c\xAE\x1Fj\xAF\x14a\x03\x15W\x80c\xC0\x1E\x1B\xD6\x14a\x03;W\x80c\xD6\xC0\xB2\xC4\x14a\x03;W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x11a\0\xBDW\x80c\x9D\xC2\x9F\xAC\x14a\x02\xDCW\x80c\xA4W\xC2\xD7\x14a\x02\xEFW\x80c\xA9\x05\x9C\xBB\x14a\x03\x02W`\0\x80\xFD[\x80cp\xA0\x821\x14a\x02\x9EW\x80c\x95\xD8\x9BA\x14a\x02\xD4W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01/W\x80c9P\x93Q\x11a\x01\x14W\x80c9P\x93Q\x14a\x02nW\x80c@\xC1\x0F\x19\x14a\x02\x81W\x80cT\xFDMP\x14a\x02\x96W`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x02*W\x80c1<\xE5g\x14a\x02=W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x11a\x01`W\x80c\x06\xFD\xDE\x03\x14a\x01\xF0W\x80c\t^\xA7\xB3\x14a\x02\x05W\x80c\x18\x16\r\xDD\x14a\x02\x18W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01|W\x80c\x039d\xBE\x14a\x01\xA4W[`\0\x80\xFD[a\x01\x8Fa\x01\x8A6`\x04a\x13)V[a\x03\xCEV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x9BV[a\x01\xF8a\x04\xBFV[`@Qa\x01\x9B\x91\x90a\x13\x9EV[a\x01\x8Fa\x02\x136`\x04a\x14\x18V[a\x05QV[`\x02T[`@Q\x90\x81R` \x01a\x01\x9BV[a\x01\x8Fa\x0286`\x04a\x14BV[a\x05iV[`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x01\x9BV[a\x01\x8Fa\x02|6`\x04a\x14\x18V[a\x05\x8DV[a\x02\x94a\x02\x8F6`\x04a\x14\x18V[a\x05\xD9V[\0[a\x01\xF8a\x07\x01V[a\x02\x1Ca\x02\xAC6`\x04a\x14~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\xF8a\x07\xA4V[a\x02\x94a\x02\xEA6`\x04a\x14\x18V[a\x07\xB3V[a\x01\x8Fa\x02\xFD6`\x04a\x14\x18V[a\x08\xCAV[a\x01\x8Fa\x03\x106`\x04a\x14\x18V[a\t\x9BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xCBV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xCBV[a\x02\x1Ca\x03o6`\x04a\x14\x99V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x01\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xECO\xC8\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x83\x14\x80a\x04\x87WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x83\x16\x14[\x80a\x04\xB6WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x82\x16\x14[\x95\x94PPPPPV[```\x03\x80Ta\x04\xCE\x90a\x14\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xFA\x90a\x14\xCCV[\x80\x15a\x05GW\x80`\x1F\x10a\x05\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05GV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05*W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x05_\x81\x85\x85a\t\xA9V[P`\x01\x93\x92PPPV[`\x003a\x05w\x85\x82\x85a\x0B]V[a\x05\x82\x85\x85\x85a\x0C4V[P`\x01\x94\x93PPPPV[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90a\x05_\x90\x82\x90\x86\x90a\x05\xD4\x90\x87\x90a\x15NV[a\t\xA9V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\xAD\x82\x82a\x0E\xE7V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x82`@Qa\x06\xF5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[``a\x07,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x07V[a\x07U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x07V[a\x07~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x07V[`@Q` \x01a\x07\x90\x93\x92\x91\x90a\x15fV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[```\x04\x80Ta\x04\xCE\x90a\x14\xCCV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[a\x08\x82\x82\x82a\x11DV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x82`@Qa\x06\xF5\x91\x81R` \x01\x90V[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90\x83\x81\x10\x15a\t\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01R\x7F zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[a\x05\x82\x82\x86\x86\x84\x03a\t\xA9V[`\x003a\x05_\x81\x85\x85a\x0C4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\nKW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01R\x7Fress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\n\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01R\x7Fss\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x0C.W\x81\x81\x10\x15a\x0C!W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x06\x9AV[a\x0C.\x84\x84\x84\x84\x03a\t\xA9V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x0C\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01R\x7Fdress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\rzW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0E0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01R\x7Falance\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x85\x85\x03\x90U\x91\x85\x16\x81R\x90\x81 \x80T\x84\x92\x90a\x0Et\x90\x84\x90a\x15NV[\x92PP\x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x0E\xDA\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x0C.V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0FdW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x06\x9AV[\x80`\x02`\0\x82\x82Ta\x0Fv\x91\x90a\x15NV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x83\x92\x90a\x0F\xB0\x90\x84\x90a\x15NV[\x90\x91UPP`@Q\x81\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[``\x81`\0\x03a\x10JWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x10tW\x80a\x10^\x81a\x15\xDCV[\x91Pa\x10m\x90P`\n\x83a\x16CV[\x91Pa\x10NV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x8FWa\x10\x8Fa\x16WV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10\xB9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x11<Wa\x10\xCE`\x01\x83a\x16\x86V[\x91Pa\x10\xDB`\n\x86a\x16\x9DV[a\x10\xE6\x90`0a\x15NV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x10\xFBWa\x10\xFBa\x16\xB1V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x115`\n\x86a\x16CV[\x94Pa\x10\xBDV[\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x11\xE7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R\x7Fs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x12\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01R\x7Fce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x83\x83\x03\x90U`\x02\x80T\x84\x92\x90a\x12\xD9\x90\x84\x90a\x16\x86V[\x90\x91UPP`@Q\x82\x81R`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\x0BPV[`\0` \x82\x84\x03\x12\x15a\x13;W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x13kW`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x13\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13uV[\x83\x81\x11\x15a\x0C.WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x13\xBD\x81`@\x85\x01` \x87\x01a\x13rV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\x13W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14+W`\0\x80\xFD[a\x144\x83a\x13\xEFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14WW`\0\x80\xFD[a\x14`\x84a\x13\xEFV[\x92Pa\x14n` \x85\x01a\x13\xEFV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x14\x90W`\0\x80\xFD[a\x13k\x82a\x13\xEFV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xACW`\0\x80\xFD[a\x14\xB5\x83a\x13\xEFV[\x91Pa\x14\xC3` \x84\x01a\x13\xEFV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x14\xE0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\x19W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x15aWa\x15aa\x15\x1FV[P\x01\x90V[`\0\x84Qa\x15x\x81\x84` \x89\x01a\x13rV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x15\xB4\x81`\x01\x85\x01` \x8A\x01a\x13rV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x15\xCF\x81`\x02\x84\x01` \x88\x01a\x13rV[\x01`\x02\x01\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x16\rWa\x16\ra\x15\x1FV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x16RWa\x16Ra\x16\x14V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x16\x98Wa\x16\x98a\x15\x1FV[P\x03\x90V[`\0\x82a\x16\xACWa\x16\xACa\x16\x14V[P\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static OPTIMISMMINTABLEERC20FACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\0\x87W`\x005`\xE0\x1C\x80c\xC4\xD6m\xE8\x11b\0\0bW\x80c\xC4\xD6m\xE8\x14b\0\x01\x02W\x80c\xCEZ\xC9\x0F\x14b\0\x01\x1BW\x80c\xE7\x8C\xEA\x92\x14b\0\x012W\x80c\xEE\x9A1\xA2\x14b\0\x01YW`\0\x80\xFD[\x80cT\xFDMP\x14b\0\0\x8CW\x80c\x89o\x93\xD1\x14b\0\0\xAEW\x80c\x8C\xF0b\x9C\x14b\0\0\xEBW[`\0\x80\xFD[b\0\0\x96b\0\x01~V[`@Qb\0\0\xA5\x91\x90b\0\x07\xA5V[`@Q\x80\x91\x03\x90\xF3[b\0\0\xC5b\0\0\xBF6`\x04b\0\x08\xCDV[b\0\x02)V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01b\0\0\xA5V[b\0\0\xC5b\0\0\xFC6`\x04b\0\tJV[b\0\x02@V[b\0\x01\x19b\0\x01\x136`\x04b\0\t\xE1V[b\0\x04<V[\0[b\0\0\xC5b\0\x01,6`\x04b\0\x08\xCDV[b\0\x05\xBAV[`\0Tb\0\0\xC5\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\0\xC5V[``b\0\x01\xAB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x05\xCBV[b\0\x01\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x05\xCBV[b\0\x02\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x05\xCBV[`@Q` \x01b\0\x02\x15\x93\x92\x91\x90b\0\t\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0b\0\x028\x84\x84\x84b\0\x05\xBAV[\x94\x93PPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16b\0\x02\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FOptimismMintableERC20Factory: mu`D\x82\x01R\x7Fst provide remote token address\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x85\x85\x85`@Q` \x01b\0\x03\x04\x93\x92\x91\x90b\0\n{V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`\0`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x88\x88\x88`@Qb\0\x03T\x90b\0\x07\x18V[b\0\x03d\x95\x94\x93\x92\x91\x90b\0\n\xCAV[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x03\x85W=`\0\x80>=`\0\xFD[P\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCE\xEB\x8E}R\r\x7F;e\xFC\x11\xA2b\xB9\x10f\x94\x01\x93\xB0]O\x93\xDF\x07\xCF\xDC\xED\x0E\xB5Q\xCF`@Q`@Q\x80\x91\x03\x90\xA3`@Q3\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x91\x90\x83\x16\x90\x7FR\xFE\x89\xDDY0\xF3C\xD2VP\xB6/\xD3g\xBA\xE4p\x88\xBC\xDD\xFF\xD2\xA8\x83P\xA6\xEC\xDDb\x0C\xDB\x90` \x01`@Q\x80\x91\x03\x90\xA3\x96\x95PPPPPPV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15b\0\x04_WP`\0T`\xFF\x80\x83\x16\x91\x16\x10[b\0\x04\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x02\xE2V[`\0\x80Ta\x01\0`\xFF\x84\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x90\x92\x16\x82\x17\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFFb\x01\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x02\x16\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\0b\0\x028\x84\x84\x84`\x12b\0\x02@V[``\x81`\0\x03b\0\x06\x0FWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15b\0\x06?W\x80b\0\x06&\x81b\0\x0B^V[\x91Pb\0\x067\x90P`\n\x83b\0\x0B\xC8V[\x91Pb\0\x06\x13V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x06]Wb\0\x06]b\0\x07\xEBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15b\0\x06\x88W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15b\0\x028Wb\0\x06\xA0`\x01\x83b\0\x0B\xDFV[\x91Pb\0\x06\xAF`\n\x86b\0\x0B\xF9V[b\0\x06\xBC\x90`0b\0\x0C\x10V[`\xF8\x1B\x81\x83\x81Q\x81\x10b\0\x06\xD4Wb\0\x06\xD4b\0\x0C+V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPb\0\x07\x10`\n\x86b\0\x0B\xC8V[\x94Pb\0\x06\x8CV[a\x1A\xD8\x80b\0\x0C[\x839\x01\x90V[`\0[\x83\x81\x10\x15b\0\x07CW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x07)V[\x83\x81\x11\x15b\0\x07SW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\0\x07s\x81` \x86\x01` \x86\x01b\0\x07&V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0\x07\xBA` \x83\x01\x84b\0\x07YV[\x93\x92PPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x07\xE6W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\x08,W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x08JWb\0\x08Jb\0\x07\xEBV[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x08\x93Wb\0\x08\x93b\0\x07\xEBV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x08\xADW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x08\xE3W`\0\x80\xFD[b\0\x08\xEE\x84b\0\x07\xC1V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\t\x0CW`\0\x80\xFD[b\0\t\x1A\x87\x83\x88\x01b\0\x08\x1AV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15b\0\t1W`\0\x80\xFD[Pb\0\t@\x86\x82\x87\x01b\0\x08\x1AV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\taW`\0\x80\xFD[b\0\tl\x85b\0\x07\xC1V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\t\x8AW`\0\x80\xFD[b\0\t\x98\x88\x83\x89\x01b\0\x08\x1AV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\0\t\xAFW`\0\x80\xFD[Pb\0\t\xBE\x87\x82\x88\x01b\0\x08\x1AV[\x92PP``\x85\x015`\xFF\x81\x16\x81\x14b\0\t\xD6W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15b\0\t\xF4W`\0\x80\xFD[b\0\x07\xBA\x82b\0\x07\xC1V[`\0\x84Qb\0\n\x13\x81\x84` \x89\x01b\0\x07&V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qb\0\nQ\x81`\x01\x85\x01` \x8A\x01b\0\x07&V[`\x01\x92\x01\x91\x82\x01R\x83Qb\0\nn\x81`\x02\x84\x01` \x88\x01b\0\x07&V[\x01`\x02\x01\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0b\0\n\xAC``\x83\x01\x85b\0\x07YV[\x82\x81\x03`@\x84\x01Rb\0\n\xC0\x81\x85b\0\x07YV[\x96\x95PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x83R\x80\x87\x16` \x84\x01RP`\xA0`@\x83\x01Rb\0\x0B\x05`\xA0\x83\x01\x86b\0\x07YV[\x82\x81\x03``\x84\x01Rb\0\x0B\x19\x81\x86b\0\x07YV[\x91PP`\xFF\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03b\0\x0B\x92Wb\0\x0B\x92b\0\x0B/V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82b\0\x0B\xDAWb\0\x0B\xDAb\0\x0B\x99V[P\x04\x90V[`\0\x82\x82\x10\x15b\0\x0B\xF4Wb\0\x0B\xF4b\0\x0B/V[P\x03\x90V[`\0\x82b\0\x0C\x0BWb\0\x0C\x0Bb\0\x0B\x99V[P\x06\x90V[`\0\x82\x19\x82\x11\x15b\0\x0C&Wb\0\x0C&b\0\x0B/V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFEa\x01@`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1A\xD88\x03\x80b\0\x1A\xD8\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01xV[`\x01`\x02`\0\x85\x85`\x03b\0\0K\x83\x82b\0\x02\xB3V[P`\x04b\0\0Z\x82\x82b\0\x02\xB3V[PPP`\x80\x92\x90\x92R`\xA0R`\xC0R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\xE0R\x93\x90\x92\x16a\x01\0RPP`\xFF\x16a\x01 Rb\0\x03\x7FV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xA6W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\0\xD3W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\0\xF0Wb\0\0\xF0b\0\0\xABV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x01\x1BWb\0\x01\x1Bb\0\0\xABV[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\x018W`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x01\\W\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\x01=V[\x83\x82\x11\x15b\0\x01nW`\0\x83\x85\x83\x01\x01R[\x96\x95PPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\x91W`\0\x80\xFD[b\0\x01\x9C\x86b\0\0\x8EV[\x94Pb\0\x01\xAC` \x87\x01b\0\0\x8EV[`@\x87\x01Q\x90\x94P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01\xCAW`\0\x80\xFD[b\0\x01\xD8\x89\x83\x8A\x01b\0\0\xC1V[\x94P``\x88\x01Q\x91P\x80\x82\x11\x15b\0\x01\xEFW`\0\x80\xFD[Pb\0\x01\xFE\x88\x82\x89\x01b\0\0\xC1V[\x92PP`\x80\x86\x01Q`\xFF\x81\x16\x81\x14b\0\x02\x16W`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x029W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02ZWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\xAEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\x89WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\xAAW\x82\x81U`\x01\x01b\0\x02\x95V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02\xCFWb\0\x02\xCFb\0\0\xABV[b\0\x02\xE7\x81b\0\x02\xE0\x84Tb\0\x02$V[\x84b\0\x02`V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03\x1FW`\0\x84\x15b\0\x03\x06WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\xAAV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03PW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03/V[P\x85\x82\x10\x15b\0\x03oW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x16\xEDb\0\x03\xEB`\09`\0a\x02D\x01R`\0\x81\x81a\x03\x17\x01R\x81\x81a\x03\xAC\x01R\x81\x81a\x05\xF1\x01Ra\x07\xCB\x01R`\0\x81\x81a\x01\xA9\x01Ra\x03=\x01R`\0a\x07Z\x01R`\0a\x071\x01R`\0a\x07\x08\x01Ra\x16\xED`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01wW`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\0\xD8W\x80c\xAE\x1Fj\xAF\x11a\0\x8CW\x80c\xDDb\xED>\x11a\0fW\x80c\xDDb\xED>\x14a\x03aW\x80c\xE7\x8C\xEA\x92\x14a\x03\x15W\x80c\xEE\x9A1\xA2\x14a\x03\xA7W`\0\x80\xFD[\x80c\xAE\x1Fj\xAF\x14a\x03\x15W\x80c\xC0\x1E\x1B\xD6\x14a\x03;W\x80c\xD6\xC0\xB2\xC4\x14a\x03;W`\0\x80\xFD[\x80c\x9D\xC2\x9F\xAC\x11a\0\xBDW\x80c\x9D\xC2\x9F\xAC\x14a\x02\xDCW\x80c\xA4W\xC2\xD7\x14a\x02\xEFW\x80c\xA9\x05\x9C\xBB\x14a\x03\x02W`\0\x80\xFD[\x80cp\xA0\x821\x14a\x02\x9EW\x80c\x95\xD8\x9BA\x14a\x02\xD4W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01/W\x80c9P\x93Q\x11a\x01\x14W\x80c9P\x93Q\x14a\x02nW\x80c@\xC1\x0F\x19\x14a\x02\x81W\x80cT\xFDMP\x14a\x02\x96W`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x02*W\x80c1<\xE5g\x14a\x02=W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x11a\x01`W\x80c\x06\xFD\xDE\x03\x14a\x01\xF0W\x80c\t^\xA7\xB3\x14a\x02\x05W\x80c\x18\x16\r\xDD\x14a\x02\x18W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01|W\x80c\x039d\xBE\x14a\x01\xA4W[`\0\x80\xFD[a\x01\x8Fa\x01\x8A6`\x04a\x13)V[a\x03\xCEV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x9BV[a\x01\xF8a\x04\xBFV[`@Qa\x01\x9B\x91\x90a\x13\x9EV[a\x01\x8Fa\x02\x136`\x04a\x14\x18V[a\x05QV[`\x02T[`@Q\x90\x81R` \x01a\x01\x9BV[a\x01\x8Fa\x0286`\x04a\x14BV[a\x05iV[`@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x01\x9BV[a\x01\x8Fa\x02|6`\x04a\x14\x18V[a\x05\x8DV[a\x02\x94a\x02\x8F6`\x04a\x14\x18V[a\x05\xD9V[\0[a\x01\xF8a\x07\x01V[a\x02\x1Ca\x02\xAC6`\x04a\x14~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x01\xF8a\x07\xA4V[a\x02\x94a\x02\xEA6`\x04a\x14\x18V[a\x07\xB3V[a\x01\x8Fa\x02\xFD6`\x04a\x14\x18V[a\x08\xCAV[a\x01\x8Fa\x03\x106`\x04a\x14\x18V[a\t\x9BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xCBV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xCBV[a\x02\x1Ca\x03o6`\x04a\x14\x99V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x01\xCB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xECO\xC8\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x83\x14\x80a\x04\x87WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x83\x16\x14[\x80a\x04\xB6WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x82\x16\x14[\x95\x94PPPPPV[```\x03\x80Ta\x04\xCE\x90a\x14\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xFA\x90a\x14\xCCV[\x80\x15a\x05GW\x80`\x1F\x10a\x05\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05GV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05*W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x05_\x81\x85\x85a\t\xA9V[P`\x01\x93\x92PPPV[`\x003a\x05w\x85\x82\x85a\x0B]V[a\x05\x82\x85\x85\x85a\x0C4V[P`\x01\x94\x93PPPPV[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90a\x05_\x90\x82\x90\x86\x90a\x05\xD4\x90\x87\x90a\x15NV[a\t\xA9V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xA3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\xAD\x82\x82a\x0E\xE7V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x82`@Qa\x06\xF5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[``a\x07,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x07V[a\x07U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x07V[a\x07~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\x07V[`@Q` \x01a\x07\x90\x93\x92\x91\x90a\x15fV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[```\x04\x80Ta\x04\xCE\x90a\x14\xCCV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[a\x08\x82\x82\x82a\x11DV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x82`@Qa\x06\xF5\x91\x81R` \x01\x90V[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90\x83\x81\x10\x15a\t\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01R\x7F zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[a\x05\x82\x82\x86\x86\x84\x03a\t\xA9V[`\x003a\x05_\x81\x85\x85a\x0C4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\nKW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01R\x7Fress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\n\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01R\x7Fss\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x0C.W\x81\x81\x10\x15a\x0C!W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x06\x9AV[a\x0C.\x84\x84\x84\x84\x03a\t\xA9V[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x0C\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01R\x7Fdress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\rzW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0E0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01R\x7Falance\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x85\x85\x03\x90U\x91\x85\x16\x81R\x90\x81 \x80T\x84\x92\x90a\x0Et\x90\x84\x90a\x15NV[\x92PP\x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x0E\xDA\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x0C.V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0FdW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x06\x9AV[\x80`\x02`\0\x82\x82Ta\x0Fv\x91\x90a\x15NV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x83\x92\x90a\x0F\xB0\x90\x84\x90a\x15NV[\x90\x91UPP`@Q\x81\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[``\x81`\0\x03a\x10JWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x10tW\x80a\x10^\x81a\x15\xDCV[\x91Pa\x10m\x90P`\n\x83a\x16CV[\x91Pa\x10NV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x8FWa\x10\x8Fa\x16WV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10\xB9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x11<Wa\x10\xCE`\x01\x83a\x16\x86V[\x91Pa\x10\xDB`\n\x86a\x16\x9DV[a\x10\xE6\x90`0a\x15NV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x10\xFBWa\x10\xFBa\x16\xB1V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x115`\n\x86a\x16CV[\x94Pa\x10\xBDV[\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x11\xE7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R\x7Fs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x12\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01R\x7Fce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x83\x83\x03\x90U`\x02\x80T\x84\x92\x90a\x12\xD9\x90\x84\x90a\x16\x86V[\x90\x91UPP`@Q\x82\x81R`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\x0BPV[`\0` \x82\x84\x03\x12\x15a\x13;W`\0\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x13kW`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x13\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13uV[\x83\x81\x11\x15a\x0C.WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x13\xBD\x81`@\x85\x01` \x87\x01a\x13rV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x14\x13W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14+W`\0\x80\xFD[a\x144\x83a\x13\xEFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14WW`\0\x80\xFD[a\x14`\x84a\x13\xEFV[\x92Pa\x14n` \x85\x01a\x13\xEFV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x14\x90W`\0\x80\xFD[a\x13k\x82a\x13\xEFV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xACW`\0\x80\xFD[a\x14\xB5\x83a\x13\xEFV[\x91Pa\x14\xC3` \x84\x01a\x13\xEFV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x14\xE0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\x19W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x15aWa\x15aa\x15\x1FV[P\x01\x90V[`\0\x84Qa\x15x\x81\x84` \x89\x01a\x13rV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x15\xB4\x81`\x01\x85\x01` \x8A\x01a\x13rV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x15\xCF\x81`\x02\x84\x01` \x88\x01a\x13rV[\x01`\x02\x01\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x16\rWa\x16\ra\x15\x1FV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x16RWa\x16Ra\x16\x14V[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x16\x98Wa\x16\x98a\x15\x1FV[P\x03\x90V[`\0\x82a\x16\xACWa\x16\xACa\x16\x14V[P\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static OPTIMISMMINTABLEERC20FACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OptimismMintableERC20Factory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OptimismMintableERC20Factory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OptimismMintableERC20Factory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OptimismMintableERC20Factory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OptimismMintableERC20Factory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OptimismMintableERC20Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OptimismMintableERC20Factory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPTIMISMMINTABLEERC20FACTORY_ABI.clone(),
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
                OPTIMISMMINTABLEERC20FACTORY_ABI.clone(),
                OPTIMISMMINTABLEERC20FACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BRIDGE` (0xee9a31a2) function
        pub fn BRIDGE(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([238, 154, 49, 162], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridge` (0xe78cea92) function
        pub fn bridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 140, 234, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createOptimismMintableERC20` (0xce5ac90f) function
        pub fn create_optimism_mintable_erc20(
            &self,
            remote_token: ::ethers::core::types::Address,
            name: ::std::string::String,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([206, 90, 201, 15], (remote_token, name, symbol))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createOptimismMintableERC20WithDecimals` (0x8cf0629c) function
        pub fn create_optimism_mintable_erc20_with_decimals(
            &self,
            remote_token: ::ethers::core::types::Address,
            name: ::std::string::String,
            symbol: ::std::string::String,
            decimals: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([140, 240, 98, 156], (remote_token, name, symbol, decimals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createStandardL2Token` (0x896f93d1) function
        pub fn create_standard_l2_token(
            &self,
            remote_token: ::ethers::core::types::Address,
            name: ::std::string::String,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([137, 111, 147, 209], (remote_token, name, symbol))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            bridge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], bridge)
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
        ///Gets the contract's `OptimismMintableERC20Created` event
        pub fn optimism_mintable_erc20_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OptimismMintableERC20CreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StandardL2TokenCreated` event
        pub fn standard_l2_token_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StandardL2TokenCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OptimismMintableERC20FactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OptimismMintableERC20Factory<M> {
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
    #[ethevent(
        name = "OptimismMintableERC20Created",
        abi = "OptimismMintableERC20Created(address,address,address)"
    )]
    pub struct OptimismMintableERC20CreatedFilter {
        #[ethevent(indexed)]
        pub local_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub remote_token: ::ethers::core::types::Address,
        pub deployer: ::ethers::core::types::Address,
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
        name = "StandardL2TokenCreated",
        abi = "StandardL2TokenCreated(address,address)"
    )]
    pub struct StandardL2TokenCreatedFilter {
        #[ethevent(indexed)]
        pub remote_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub local_token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OptimismMintableERC20FactoryEvents {
        InitializedFilter(InitializedFilter),
        OptimismMintableERC20CreatedFilter(OptimismMintableERC20CreatedFilter),
        StandardL2TokenCreatedFilter(StandardL2TokenCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for OptimismMintableERC20FactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(
                    OptimismMintableERC20FactoryEvents::InitializedFilter(decoded),
                );
            }
            if let Ok(decoded) = OptimismMintableERC20CreatedFilter::decode_log(log) {
                return Ok(
                    OptimismMintableERC20FactoryEvents::OptimismMintableERC20CreatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = StandardL2TokenCreatedFilter::decode_log(log) {
                return Ok(
                    OptimismMintableERC20FactoryEvents::StandardL2TokenCreatedFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OptimismMintableERC20FactoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OptimismMintableERC20CreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StandardL2TokenCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter>
    for OptimismMintableERC20FactoryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OptimismMintableERC20CreatedFilter>
    for OptimismMintableERC20FactoryEvents {
        fn from(value: OptimismMintableERC20CreatedFilter) -> Self {
            Self::OptimismMintableERC20CreatedFilter(value)
        }
    }
    impl ::core::convert::From<StandardL2TokenCreatedFilter>
    for OptimismMintableERC20FactoryEvents {
        fn from(value: StandardL2TokenCreatedFilter) -> Self {
            Self::StandardL2TokenCreatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `BRIDGE` function with signature `BRIDGE()` and selector `0xee9a31a2`
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
    #[ethcall(name = "BRIDGE", abi = "BRIDGE()")]
    pub struct BRIDGECall;
    ///Container type for all input parameters for the `bridge` function with signature `bridge()` and selector `0xe78cea92`
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
    #[ethcall(name = "bridge", abi = "bridge()")]
    pub struct bridgeCall;
    ///Container type for all input parameters for the `createOptimismMintableERC20` function with signature `createOptimismMintableERC20(address,string,string)` and selector `0xce5ac90f`
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
        name = "createOptimismMintableERC20",
        abi = "createOptimismMintableERC20(address,string,string)"
    )]
    pub struct CreateOptimismMintableERC20Call {
        pub remote_token: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `createOptimismMintableERC20WithDecimals` function with signature `createOptimismMintableERC20WithDecimals(address,string,string,uint8)` and selector `0x8cf0629c`
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
        name = "createOptimismMintableERC20WithDecimals",
        abi = "createOptimismMintableERC20WithDecimals(address,string,string,uint8)"
    )]
    pub struct CreateOptimismMintableERC20WithDecimalsCall {
        pub remote_token: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
        pub decimals: u8,
    }
    ///Container type for all input parameters for the `createStandardL2Token` function with signature `createStandardL2Token(address,string,string)` and selector `0x896f93d1`
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
        name = "createStandardL2Token",
        abi = "createStandardL2Token(address,string,string)"
    )]
    pub struct CreateStandardL2TokenCall {
        pub remote_token: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
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
        pub bridge: ::ethers::core::types::Address,
    }
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
    pub enum OptimismMintableERC20FactoryCalls {
        BRIDGE(BRIDGECall),
        bridge(bridgeCall),
        CreateOptimismMintableERC20(CreateOptimismMintableERC20Call),
        CreateOptimismMintableERC20WithDecimals(
            CreateOptimismMintableERC20WithDecimalsCall,
        ),
        CreateStandardL2Token(CreateStandardL2TokenCall),
        Initialize(InitializeCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for OptimismMintableERC20FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BRIDGECall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BRIDGE(decoded));
            }
            if let Ok(decoded) = <bridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::bridge(decoded));
            }
            if let Ok(decoded) = <CreateOptimismMintableERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateOptimismMintableERC20(decoded));
            }
            if let Ok(decoded) = <CreateOptimismMintableERC20WithDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateOptimismMintableERC20WithDecimals(decoded));
            }
            if let Ok(decoded) = <CreateStandardL2TokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateStandardL2Token(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OptimismMintableERC20FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BRIDGE(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::bridge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateOptimismMintableERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateOptimismMintableERC20WithDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateStandardL2Token(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OptimismMintableERC20FactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BRIDGE(element) => ::core::fmt::Display::fmt(element, f),
                Self::bridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateOptimismMintableERC20(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateOptimismMintableERC20WithDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateStandardL2Token(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BRIDGECall> for OptimismMintableERC20FactoryCalls {
        fn from(value: BRIDGECall) -> Self {
            Self::BRIDGE(value)
        }
    }
    impl ::core::convert::From<bridgeCall> for OptimismMintableERC20FactoryCalls {
        fn from(value: bridgeCall) -> Self {
            Self::bridge(value)
        }
    }
    impl ::core::convert::From<CreateOptimismMintableERC20Call>
    for OptimismMintableERC20FactoryCalls {
        fn from(value: CreateOptimismMintableERC20Call) -> Self {
            Self::CreateOptimismMintableERC20(value)
        }
    }
    impl ::core::convert::From<CreateOptimismMintableERC20WithDecimalsCall>
    for OptimismMintableERC20FactoryCalls {
        fn from(value: CreateOptimismMintableERC20WithDecimalsCall) -> Self {
            Self::CreateOptimismMintableERC20WithDecimals(value)
        }
    }
    impl ::core::convert::From<CreateStandardL2TokenCall>
    for OptimismMintableERC20FactoryCalls {
        fn from(value: CreateStandardL2TokenCall) -> Self {
            Self::CreateStandardL2Token(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for OptimismMintableERC20FactoryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<VersionCall> for OptimismMintableERC20FactoryCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `BRIDGE` function with signature `BRIDGE()` and selector `0xee9a31a2`
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
    pub struct BRIDGEReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `bridge` function with signature `bridge()` and selector `0xe78cea92`
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
    pub struct bridgeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `createOptimismMintableERC20` function with signature `createOptimismMintableERC20(address,string,string)` and selector `0xce5ac90f`
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
    pub struct CreateOptimismMintableERC20Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `createOptimismMintableERC20WithDecimals` function with signature `createOptimismMintableERC20WithDecimals(address,string,string,uint8)` and selector `0x8cf0629c`
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
    pub struct CreateOptimismMintableERC20WithDecimalsReturn(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `createStandardL2Token` function with signature `createStandardL2Token(address,string,string)` and selector `0x896f93d1`
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
    pub struct CreateStandardL2TokenReturn(pub ::ethers::core::types::Address);
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

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
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x01`\x80R`\x03`\xA0R`\0`\xC0\x81\x90Ra\0x\x90a\0}V[a\x01uV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15a\0\x9FWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[a\x01\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x01\0`\xFF\x84\x16a\xFF\xFF\x19\x90\x92\x16\x82\x17\x17a\x01\0`\x01`\xB0\x1B\x03\x19\x16a\xFF\0\x19b\x01\0\0`\x01`\x01`\xA0\x1B\x03\x87\x16\x02\x16\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\x80Q`\xA0Q`\xC0Qa,sa\x01\xA4`\09`\0a\x02\xBE\x01R`\0a\x02\x93\x01R`\0a\x02h\x01Ra,s`\0\xF3\xFE`\x80`@R4\x80\x15b\0\0\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10b\0\0\xFEW`\x005`\xE0\x1C\x80c\xCEZ\xC9\x0F\x11b\0\0\xD9W\x80c\xCEZ\xC9\x0F\x14b\0\x01\xFDW\x80c\xE7\x8C\xEA\x92\x14b\0\x02\x14W\x80c\xEE\x9A1\xA2\x14b\0\x02;Wb\0\0\xFEV[\x80cT\xFDMP\x14b\0\x01\x85W\x80c\x89o\x93\xD1\x14b\0\x01\xA7W\x80c\xC4\xD6m\xE8\x14b\0\x01\xE4W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[b\0\x01\x8Fb\0\x02`V[`@Qb\0\x01\x9E\x91\x90b\0\x08sV[`@Q\x80\x91\x03\x90\xF3[b\0\x01\xBEb\0\x01\xB86`\x04b\0\x0B\xB0V[b\0\x03\x0BV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01b\0\x01\x9EV[b\0\x01\xFBb\0\x01\xF56`\x04b\0\x0C<V[b\0\x03\"V[\0[b\0\x01\xBEb\0\x02\x0E6`\x04b\0\x0B\xB0V[b\0\x04\xA4V[`\0Tb\0\x01\xBE\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\x01\xBEV[``b\0\x02\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x06\x99V[b\0\x02\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x06\x99V[b\0\x02\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x06\x99V[`@Q` \x01b\0\x02\xF7\x93\x92\x91\x90b\0\x0C_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0b\0\x03\x1A\x84\x84\x84b\0\x04\xA4V[\x94\x93PPPPV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15b\0\x03EWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[b\0\x03\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x01\0`\xFF\x84\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x90\x92\x16\x82\x17\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFFb\x01\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x02\x16\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16b\0\x05KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FOptimismMintableERC20Factory: mu`D\x82\x01R\x7Fst provide remote token address\0`d\x82\x01R`\x84\x01b\0\x03\xCEV[`\0\x84\x84\x84`@Q` \x01b\0\x05d\x93\x92\x91\x90b\0\x0C\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`\0`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87\x87`@Qb\0\x05\xB3\x90b\0\x07\xE6V[b\0\x05\xC2\x94\x93\x92\x91\x90b\0\r V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x05\xE3W=`\0\x80>=`\0\xFD[P\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCE\xEB\x8E}R\r\x7F;e\xFC\x11\xA2b\xB9\x10f\x94\x01\x93\xB0]O\x93\xDF\x07\xCF\xDC\xED\x0E\xB5Q\xCF`@Q`@Q\x80\x91\x03\x90\xA3`@Q3\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x90\x83\x16\x90\x7FR\xFE\x89\xDDY0\xF3C\xD2VP\xB6/\xD3g\xBA\xE4p\x88\xBC\xDD\xFF\xD2\xA8\x83P\xA6\xEC\xDDb\x0C\xDB\x90` \x01`@Q\x80\x91\x03\x90\xA3\x95\x94PPPPPV[``\x81`\0\x03b\0\x06\xDDWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15b\0\x07\rW\x80b\0\x06\xF4\x81b\0\r\xA9V[\x91Pb\0\x07\x05\x90P`\n\x83b\0\x0E\x13V[\x91Pb\0\x06\xE1V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x07+Wb\0\x07+b\0\t\xC3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15b\0\x07VW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15b\0\x03\x1AWb\0\x07n`\x01\x83b\0\x0E*V[\x91Pb\0\x07}`\n\x86b\0\x0EDV[b\0\x07\x8A\x90`0b\0\x0E[V[`\xF8\x1B\x81\x83\x81Q\x81\x10b\0\x07\xA2Wb\0\x07\xA2b\0\x0EvV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPb\0\x07\xDE`\n\x86b\0\x0E\x13V[\x94Pb\0\x07ZV[a\x1D\xC1\x80b\0\x0E\xA6\x839\x01\x90V[`\0[\x83\x81\x10\x15b\0\x08\x11W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x07\xF7V[\x83\x81\x11\x15b\0\x08!W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\0\x08A\x81` \x86\x01` \x86\x01b\0\x07\xF4V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0\x08\x88` \x83\x01\x84b\0\x08'V[\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\t\xBEW`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12b\0\n\x90W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\n\xAEWb\0\n\xAEb\0\t\xC3V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\n\xF7Wb\0\n\xF7b\0\t\xC3V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x0B\x93W`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[b\0\x0B\xA6\x84` \x83\x01` \x89\x01b\0\t\xF2V[\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x0B\xCBWb\0\x0B\xCBb\0\x08\x8FV[b\0\x0B\xD6\x84b\0\t\x99V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x0B\xF9Wb\0\x0B\xF9b\0\t\x14V[b\0\x0C\x07\x87\x83\x88\x01b\0\t\xFEV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15b\0\x0C#Wb\0\x0C#b\0\t\x14V[Pb\0\x0C2\x86\x82\x87\x01b\0\t\xFEV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x0CTWb\0\x0CTb\0\x08\x8FV[b\0\x08\x88\x82b\0\t\x99V[`\0\x84Qb\0\x0Cs\x81\x84` \x89\x01b\0\x07\xF4V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qb\0\x0C\xB1\x81`\x01\x85\x01` \x8A\x01b\0\x07\xF4V[`\x01\x92\x01\x91\x82\x01R\x83Qb\0\x0C\xCE\x81`\x02\x84\x01` \x88\x01b\0\x07\xF4V[\x01`\x02\x01\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0b\0\r\x0C``\x83\x01\x85b\0\x08'V[\x82\x81\x03`@\x84\x01Rb\0\x0B\xA6\x81\x85b\0\x08'V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Rb\0\r[`\x80\x83\x01\x85b\0\x08'V[\x82\x81\x03``\x84\x01Rb\0\ro\x81\x85b\0\x08'V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03b\0\r\xDDWb\0\r\xDDb\0\rzV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82b\0\x0E%Wb\0\x0E%b\0\r\xE4V[P\x04\x90V[`\0\x82\x82\x10\x15b\0\x0E?Wb\0\x0E?b\0\rzV[P\x03\x90V[`\0\x82b\0\x0EVWb\0\x0EVb\0\r\xE4V[P\x06\x90V[`\0\x82\x19\x82\x11\x15b\0\x0EqWb\0\x0Eqb\0\rzV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFEa\x01 `@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x1D\xC18\x03\x80b\0\x1D\xC1\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\x02\xBDV[`\x01\x80`\0\x84\x84`\x03b\0\0\x97\x83\x82b\0\x041V[P`\x04b\0\0\xA6\x82\x82b\0\x041V[PPP`\x80\x92\x90\x92R`\xA0R`\xC0RPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xE0R\x16a\x01\0Rb\0\x04\xFDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x018W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x01pW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x01VV[\x83\x81\x11\x15b\0\x01\x80W`\0\x84\x84\x01R[PPPPV[`\0\x82`\x1F\x83\x01\x12b\0\x01\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02\tWb\0\x02\tb\0\x01=V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x024Wb\0\x024b\0\x01=V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x02\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[b\0\x02\xB3\x84` \x83\x01` \x89\x01b\0\x01SV[\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x03\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x03*\x85b\0\x01 V[\x93Pb\0\x03:` \x86\x01b\0\x01 V[`@\x86\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03]Wb\0\x03]b\0\0\xD0V[b\0\x03k\x88\x83\x89\x01b\0\x01\x86V[\x93P``\x87\x01Q\x91P\x80\x82\x11\x15b\0\x03\x87Wb\0\x03\x87b\0\0\xD0V[Pb\0\x03\x96\x87\x82\x88\x01b\0\x01\x86V[\x91PP\x92\x95\x91\x94P\x92PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03\xB7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\xD8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x04,W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x04\x07WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x04(W\x82\x81U`\x01\x01b\0\x04\x13V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04MWb\0\x04Mb\0\x01=V[b\0\x04e\x81b\0\x04^\x84Tb\0\x03\xA2V[\x84b\0\x03\xDEV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x04\x9DW`\0\x84\x15b\0\x04\x84WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x04(V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x04\xCEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x04\xADV[P\x85\x82\x10\x15b\0\x04\xEDW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x18cb\0\x05^`\09`\0\x81\x81a\x03\xF9\x01R\x81\x81a\x04\x8E\x01R\x81\x81a\x06\xD3\x01Ra\x08\xAD\x01R`\0\x81\x81a\x02\xAD\x01Ra\x04\x1F\x01R`\0a\x08<\x01R`\0a\x08\x13\x01R`\0a\x07\xEA\x01Ra\x18c`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xF9W`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\x01ZW\x80c\xAE\x1Fj\xAF\x11a\x01\x0EW\x80c\xDDb\xED>\x11a\0\xE8W\x80c\xDDb\xED>\x14a\x04CW\x80c\xE7\x8C\xEA\x92\x14a\x03\xF7W\x80c\xEE\x9A1\xA2\x14a\x04\x89Wa\x01\xF9V[\x80c\xAE\x1Fj\xAF\x14a\x03\xF7W\x80c\xC0\x1E\x1B\xD6\x14a\x04\x1DW\x80c\xD6\xC0\xB2\xC4\x14a\x04\x1DWa\x01\xF9V[\x80c\x9D\xC2\x9F\xAC\x11a\x01?W\x80c\x9D\xC2\x9F\xAC\x14a\x03\xBEW\x80c\xA4W\xC2\xD7\x14a\x03\xD1W\x80c\xA9\x05\x9C\xBB\x14a\x03\xE4Wa\x01\xF9V[\x80cp\xA0\x821\x14a\x03\x80W\x80c\x95\xD8\x9BA\x14a\x03\xB6Wa\x01\xF9V[\x80c#\xB8r\xDD\x11a\x01\xB1W\x80c9P\x93Q\x11a\x01\x96W\x80c9P\x93Q\x14a\x03PW\x80c@\xC1\x0F\x19\x14a\x03cW\x80cT\xFDMP\x14a\x03xWa\x01\xF9V[\x80c#\xB8r\xDD\x14a\x03.W\x80c1<\xE5g\x14a\x03AWa\x01\xF9V[\x80c\x06\xFD\xDE\x03\x11a\x01\xE2W\x80c\x06\xFD\xDE\x03\x14a\x02\xF4W\x80c\t^\xA7\xB3\x14a\x03\tW\x80c\x18\x16\r\xDD\x14a\x03\x1CWa\x01\xF9V[\x80c\x01\xFF\xC9\xA7\x14a\x02\x80W\x80c\x039d\xBE\x14a\x02\xA8W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02\x93a\x02\x8E6`\x04a\x14\x90V[a\x04\xB0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x9FV[a\x02\xFCa\x05\xA1V[`@Qa\x02\x9F\x91\x90a\x15\x08V[a\x02\x93a\x03\x176`\x04a\x15\x82V[a\x063V[`\x02T[`@Q\x90\x81R` \x01a\x02\x9FV[a\x02\x93a\x03<6`\x04a\x15\xAFV[a\x06KV[`@Q`\x12\x81R` \x01a\x02\x9FV[a\x02\x93a\x03^6`\x04a\x15\x82V[a\x06oV[a\x03va\x03q6`\x04a\x15\x82V[a\x06\xBBV[\0[a\x02\xFCa\x07\xE3V[a\x03 a\x03\x8E6`\x04a\x15\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02\xFCa\x08\x86V[a\x03va\x03\xCC6`\x04a\x15\x82V[a\x08\x95V[a\x02\x93a\x03\xDF6`\x04a\x15\x82V[a\t\xACV[a\x02\x93a\x03\xF26`\x04a\x15\x82V[a\n}V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xCFV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xCFV[a\x03 a\x04Q6`\x04a\x16\x0CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x02\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xECO\xC8\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x83\x14\x80a\x05iWP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x83\x16\x14[\x80a\x05\x98WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x82\x16\x14[\x95\x94PPPPPV[```\x03\x80Ta\x05\xB0\x90a\x16BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xDC\x90a\x16BV[\x80\x15a\x06)W\x80`\x1F\x10a\x05\xFEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06)V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06A\x81\x85\x85a\n\x8BV[P`\x01\x93\x92PPPV[`\x003a\x06Y\x85\x82\x85a\x0C?V[a\x06d\x85\x85\x85a\r\x16V[P`\x01\x94\x93PPPPV[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90a\x06A\x90\x82\x90\x86\x90a\x06\xB6\x90\x87\x90a\x16\xC4V[a\n\x8BV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\x85W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\x8F\x82\x82a\x0F\xC9V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x82`@Qa\x07\xD7\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[``a\x08\x0E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xE9V[a\x087\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xE9V[a\x08`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xE9V[`@Q` \x01a\x08r\x93\x92\x91\x90a\x16\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[```\x04\x80Ta\x05\xB0\x90a\x16BV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\tZW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[a\td\x82\x82a\x12&V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x82`@Qa\x07\xD7\x91\x81R` \x01\x90V[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90\x83\x81\x10\x15a\npW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01R\x7F zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[a\x06d\x82\x86\x86\x84\x03a\n\x8BV[`\x003a\x06A\x81\x85\x85a\r\x16V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x0B-W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01R\x7Fress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0B\xD0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01R\x7Fss\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\r\x10W\x81\x81\x10\x15a\r\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07|V[a\r\x10\x84\x84\x84\x84\x03a\n\x8BV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\r\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01R\x7Fdress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0E\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0F\x12W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01R\x7Falance\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x85\x85\x03\x90U\x91\x85\x16\x81R\x90\x81 \x80T\x84\x92\x90a\x0FV\x90\x84\x90a\x16\xC4V[\x92PP\x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x0F\xBC\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\r\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x10FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07|V[\x80`\x02`\0\x82\x82Ta\x10X\x91\x90a\x16\xC4V[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x83\x92\x90a\x10\x92\x90\x84\x90a\x16\xC4V[\x90\x91UPP`@Q\x81\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[``\x81`\0\x03a\x11,WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x11VW\x80a\x11@\x81a\x17RV[\x91Pa\x11O\x90P`\n\x83a\x17\xB9V[\x91Pa\x110V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11qWa\x11qa\x17\xCDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x11\x9BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x12\x1EWa\x11\xB0`\x01\x83a\x17\xFCV[\x91Pa\x11\xBD`\n\x86a\x18\x13V[a\x11\xC8\x90`0a\x16\xC4V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x11\xDDWa\x11\xDDa\x18'V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x12\x17`\n\x86a\x17\xB9V[\x94Pa\x11\x9FV[\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x12\xC9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R\x7Fs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x13\x7FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01R\x7Fce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x83\x83\x03\x90U`\x02\x80T\x84\x92\x90a\x13\xBB\x90\x84\x90a\x17\xFCV[\x90\x91UPP`@Q\x82\x81R`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\x0C2V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x14\xA5Wa\x14\xA5a\x14\x0BV[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x14\xD5W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x14\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xDFV[\x83\x81\x11\x15a\r\x10WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x15'\x81`@\x85\x01` \x87\x01a\x14\xDCV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15}W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x98Wa\x15\x98a\x14\x0BV[a\x15\xA1\x83a\x15YV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15\xC7Wa\x15\xC7a\x14\x0BV[a\x15\xD0\x84a\x15YV[\x92Pa\x15\xDE` \x85\x01a\x15YV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x16\x03Wa\x16\x03a\x14\x0BV[a\x14\xD5\x82a\x15YV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\"Wa\x16\"a\x14\x0BV[a\x16+\x83a\x15YV[\x91Pa\x169` \x84\x01a\x15YV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x16VW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x16\x8FW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x16\xD7Wa\x16\xD7a\x16\x95V[P\x01\x90V[`\0\x84Qa\x16\xEE\x81\x84` \x89\x01a\x14\xDCV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x17*\x81`\x01\x85\x01` \x8A\x01a\x14\xDCV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x17E\x81`\x02\x84\x01` \x88\x01a\x14\xDCV[\x01`\x02\x01\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x17\x83Wa\x17\x83a\x16\x95V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x17\xC8Wa\x17\xC8a\x17\x8AV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x18\x0EWa\x18\x0Ea\x16\x95V[P\x03\x90V[`\0\x82a\x18\"Wa\x18\"a\x17\x8AV[P\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static OPTIMISMMINTABLEERC20FACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10b\0\0\xFEW`\x005`\xE0\x1C\x80c\xCEZ\xC9\x0F\x11b\0\0\xD9W\x80c\xCEZ\xC9\x0F\x14b\0\x01\xFDW\x80c\xE7\x8C\xEA\x92\x14b\0\x02\x14W\x80c\xEE\x9A1\xA2\x14b\0\x02;Wb\0\0\xFEV[\x80cT\xFDMP\x14b\0\x01\x85W\x80c\x89o\x93\xD1\x14b\0\x01\xA7W\x80c\xC4\xD6m\xE8\x14b\0\x01\xE4W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[b\0\x01\x8Fb\0\x02`V[`@Qb\0\x01\x9E\x91\x90b\0\x08sV[`@Q\x80\x91\x03\x90\xF3[b\0\x01\xBEb\0\x01\xB86`\x04b\0\x0B\xB0V[b\0\x03\x0BV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01b\0\x01\x9EV[b\0\x01\xFBb\0\x01\xF56`\x04b\0\x0C<V[b\0\x03\"V[\0[b\0\x01\xBEb\0\x02\x0E6`\x04b\0\x0B\xB0V[b\0\x04\xA4V[`\0Tb\0\x01\xBE\x90b\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0Tb\x01\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\x01\xBEV[``b\0\x02\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x06\x99V[b\0\x02\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x06\x99V[b\0\x02\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\0\x06\x99V[`@Q` \x01b\0\x02\xF7\x93\x92\x91\x90b\0\x0C_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0b\0\x03\x1A\x84\x84\x84b\0\x04\xA4V[\x94\x93PPPPV[`\0T`\x02\x90a\x01\0\x90\x04`\xFF\x16\x15\x80\x15b\0\x03EWP`\0T`\xFF\x80\x83\x16\x91\x16\x10[b\0\x03\xD7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x01\0`\xFF\x84\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x90\x92\x16\x82\x17\x17\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFFb\x01\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x02\x16\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16b\0\x05KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FOptimismMintableERC20Factory: mu`D\x82\x01R\x7Fst provide remote token address\0`d\x82\x01R`\x84\x01b\0\x03\xCEV[`\0\x84\x84\x84`@Q` \x01b\0\x05d\x93\x92\x91\x90b\0\x0C\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`\0`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87\x87`@Qb\0\x05\xB3\x90b\0\x07\xE6V[b\0\x05\xC2\x94\x93\x92\x91\x90b\0\r V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x05\xE3W=`\0\x80>=`\0\xFD[P\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCE\xEB\x8E}R\r\x7F;e\xFC\x11\xA2b\xB9\x10f\x94\x01\x93\xB0]O\x93\xDF\x07\xCF\xDC\xED\x0E\xB5Q\xCF`@Q`@Q\x80\x91\x03\x90\xA3`@Q3\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x90\x83\x16\x90\x7FR\xFE\x89\xDDY0\xF3C\xD2VP\xB6/\xD3g\xBA\xE4p\x88\xBC\xDD\xFF\xD2\xA8\x83P\xA6\xEC\xDDb\x0C\xDB\x90` \x01`@Q\x80\x91\x03\x90\xA3\x95\x94PPPPPV[``\x81`\0\x03b\0\x06\xDDWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15b\0\x07\rW\x80b\0\x06\xF4\x81b\0\r\xA9V[\x91Pb\0\x07\x05\x90P`\n\x83b\0\x0E\x13V[\x91Pb\0\x06\xE1V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x07+Wb\0\x07+b\0\t\xC3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15b\0\x07VW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15b\0\x03\x1AWb\0\x07n`\x01\x83b\0\x0E*V[\x91Pb\0\x07}`\n\x86b\0\x0EDV[b\0\x07\x8A\x90`0b\0\x0E[V[`\xF8\x1B\x81\x83\x81Q\x81\x10b\0\x07\xA2Wb\0\x07\xA2b\0\x0EvV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPb\0\x07\xDE`\n\x86b\0\x0E\x13V[\x94Pb\0\x07ZV[a\x1D\xC1\x80b\0\x0E\xA6\x839\x01\x90V[`\0[\x83\x81\x10\x15b\0\x08\x11W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x07\xF7V[\x83\x81\x11\x15b\0\x08!W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Rb\0\x08A\x81` \x86\x01` \x86\x01b\0\x07\xF4V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0\x08\x88` \x83\x01\x84b\0\x08'V[\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\t\xBEW`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12b\0\n\x90W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\n\xAEWb\0\n\xAEb\0\t\xC3V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\n\xF7Wb\0\n\xF7b\0\t\xC3V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x0B\x93W`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[b\0\x0B\xA6\x84` \x83\x01` \x89\x01b\0\t\xF2V[\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x0B\xCBWb\0\x0B\xCBb\0\x08\x8FV[b\0\x0B\xD6\x84b\0\t\x99V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0\x0B\xF9Wb\0\x0B\xF9b\0\t\x14V[b\0\x0C\x07\x87\x83\x88\x01b\0\t\xFEV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15b\0\x0C#Wb\0\x0C#b\0\t\x14V[Pb\0\x0C2\x86\x82\x87\x01b\0\t\xFEV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x0CTWb\0\x0CTb\0\x08\x8FV[b\0\x08\x88\x82b\0\t\x99V[`\0\x84Qb\0\x0Cs\x81\x84` \x89\x01b\0\x07\xF4V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qb\0\x0C\xB1\x81`\x01\x85\x01` \x8A\x01b\0\x07\xF4V[`\x01\x92\x01\x91\x82\x01R\x83Qb\0\x0C\xCE\x81`\x02\x84\x01` \x88\x01b\0\x07\xF4V[\x01`\x02\x01\x95\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R``` \x82\x01R`\0b\0\r\x0C``\x83\x01\x85b\0\x08'V[\x82\x81\x03`@\x84\x01Rb\0\x0B\xA6\x81\x85b\0\x08'V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Rb\0\r[`\x80\x83\x01\x85b\0\x08'V[\x82\x81\x03``\x84\x01Rb\0\ro\x81\x85b\0\x08'V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03b\0\r\xDDWb\0\r\xDDb\0\rzV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82b\0\x0E%Wb\0\x0E%b\0\r\xE4V[P\x04\x90V[`\0\x82\x82\x10\x15b\0\x0E?Wb\0\x0E?b\0\rzV[P\x03\x90V[`\0\x82b\0\x0EVWb\0\x0EVb\0\r\xE4V[P\x06\x90V[`\0\x82\x19\x82\x11\x15b\0\x0EqWb\0\x0Eqb\0\rzV[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFEa\x01 `@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x1D\xC18\x03\x80b\0\x1D\xC1\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\x02\xBDV[`\x01\x80`\0\x84\x84`\x03b\0\0\x97\x83\x82b\0\x041V[P`\x04b\0\0\xA6\x82\x82b\0\x041V[PPP`\x80\x92\x90\x92R`\xA0R`\xC0RPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xE0R\x16a\x01\0Rb\0\x04\xFDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x018W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x01pW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x01VV[\x83\x81\x11\x15b\0\x01\x80W`\0\x84\x84\x01R[PPPPV[`\0\x82`\x1F\x83\x01\x12b\0\x01\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02\tWb\0\x02\tb\0\x01=V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x024Wb\0\x024b\0\x01=V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x02\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[b\0\x02\xB3\x84` \x83\x01` \x89\x01b\0\x01SV[\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x03\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x03*\x85b\0\x01 V[\x93Pb\0\x03:` \x86\x01b\0\x01 V[`@\x86\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03]Wb\0\x03]b\0\0\xD0V[b\0\x03k\x88\x83\x89\x01b\0\x01\x86V[\x93P``\x87\x01Q\x91P\x80\x82\x11\x15b\0\x03\x87Wb\0\x03\x87b\0\0\xD0V[Pb\0\x03\x96\x87\x82\x88\x01b\0\x01\x86V[\x91PP\x92\x95\x91\x94P\x92PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03\xB7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\xD8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x04,W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x04\x07WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x04(W\x82\x81U`\x01\x01b\0\x04\x13V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04MWb\0\x04Mb\0\x01=V[b\0\x04e\x81b\0\x04^\x84Tb\0\x03\xA2V[\x84b\0\x03\xDEV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x04\x9DW`\0\x84\x15b\0\x04\x84WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x04(V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x04\xCEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x04\xADV[P\x85\x82\x10\x15b\0\x04\xEDW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x18cb\0\x05^`\09`\0\x81\x81a\x03\xF9\x01R\x81\x81a\x04\x8E\x01R\x81\x81a\x06\xD3\x01Ra\x08\xAD\x01R`\0\x81\x81a\x02\xAD\x01Ra\x04\x1F\x01R`\0a\x08<\x01R`\0a\x08\x13\x01R`\0a\x07\xEA\x01Ra\x18c`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xF9W`\x005`\xE0\x1C\x80cp\xA0\x821\x11a\x01ZW\x80c\xAE\x1Fj\xAF\x11a\x01\x0EW\x80c\xDDb\xED>\x11a\0\xE8W\x80c\xDDb\xED>\x14a\x04CW\x80c\xE7\x8C\xEA\x92\x14a\x03\xF7W\x80c\xEE\x9A1\xA2\x14a\x04\x89Wa\x01\xF9V[\x80c\xAE\x1Fj\xAF\x14a\x03\xF7W\x80c\xC0\x1E\x1B\xD6\x14a\x04\x1DW\x80c\xD6\xC0\xB2\xC4\x14a\x04\x1DWa\x01\xF9V[\x80c\x9D\xC2\x9F\xAC\x11a\x01?W\x80c\x9D\xC2\x9F\xAC\x14a\x03\xBEW\x80c\xA4W\xC2\xD7\x14a\x03\xD1W\x80c\xA9\x05\x9C\xBB\x14a\x03\xE4Wa\x01\xF9V[\x80cp\xA0\x821\x14a\x03\x80W\x80c\x95\xD8\x9BA\x14a\x03\xB6Wa\x01\xF9V[\x80c#\xB8r\xDD\x11a\x01\xB1W\x80c9P\x93Q\x11a\x01\x96W\x80c9P\x93Q\x14a\x03PW\x80c@\xC1\x0F\x19\x14a\x03cW\x80cT\xFDMP\x14a\x03xWa\x01\xF9V[\x80c#\xB8r\xDD\x14a\x03.W\x80c1<\xE5g\x14a\x03AWa\x01\xF9V[\x80c\x06\xFD\xDE\x03\x11a\x01\xE2W\x80c\x06\xFD\xDE\x03\x14a\x02\xF4W\x80c\t^\xA7\xB3\x14a\x03\tW\x80c\x18\x16\r\xDD\x14a\x03\x1CWa\x01\xF9V[\x80c\x01\xFF\xC9\xA7\x14a\x02\x80W\x80c\x039d\xBE\x14a\x02\xA8W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02\x93a\x02\x8E6`\x04a\x14\x90V[a\x04\xB0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x9FV[a\x02\xFCa\x05\xA1V[`@Qa\x02\x9F\x91\x90a\x15\x08V[a\x02\x93a\x03\x176`\x04a\x15\x82V[a\x063V[`\x02T[`@Q\x90\x81R` \x01a\x02\x9FV[a\x02\x93a\x03<6`\x04a\x15\xAFV[a\x06KV[`@Q`\x12\x81R` \x01a\x02\x9FV[a\x02\x93a\x03^6`\x04a\x15\x82V[a\x06oV[a\x03va\x03q6`\x04a\x15\x82V[a\x06\xBBV[\0[a\x02\xFCa\x07\xE3V[a\x03 a\x03\x8E6`\x04a\x15\xEEV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x02\xFCa\x08\x86V[a\x03va\x03\xCC6`\x04a\x15\x82V[a\x08\x95V[a\x02\x93a\x03\xDF6`\x04a\x15\x82V[a\t\xACV[a\x02\x93a\x03\xF26`\x04a\x15\x82V[a\n}V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xCFV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xCFV[a\x03 a\x04Q6`\x04a\x16\x0CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x02\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x1D\x1D\x8Bc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xECO\xC8\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x83\x14\x80a\x05iWP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x83\x16\x14[\x80a\x05\x98WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x81\x16\x90\x82\x16\x14[\x95\x94PPPPPV[```\x03\x80Ta\x05\xB0\x90a\x16BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xDC\x90a\x16BV[\x80\x15a\x06)W\x80`\x1F\x10a\x05\xFEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06)V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06A\x81\x85\x85a\n\x8BV[P`\x01\x93\x92PPPV[`\x003a\x06Y\x85\x82\x85a\x0C?V[a\x06d\x85\x85\x85a\r\x16V[P`\x01\x94\x93PPPPV[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90a\x06A\x90\x82\x90\x86\x90a\x06\xB6\x90\x87\x90a\x16\xC4V[a\n\x8BV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\x85W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x07\x8F\x82\x82a\x0F\xC9V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x82`@Qa\x07\xD7\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[``a\x08\x0E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xE9V[a\x087\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xE9V[a\x08`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xE9V[`@Q` \x01a\x08r\x93\x92\x91\x90a\x16\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[```\x04\x80Ta\x05\xB0\x90a\x16BV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\tZW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FOptimismMintableERC20: only brid`D\x82\x01R\x7Fge can mint and burn\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[a\td\x82\x82a\x12&V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x82`@Qa\x07\xD7\x91\x81R` \x01\x90V[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x81 T\x90\x91\x90\x83\x81\x10\x15a\npW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01R\x7F zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[a\x06d\x82\x86\x86\x84\x03a\n\x8BV[`\x003a\x06A\x81\x85\x85a\r\x16V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x0B-W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01R\x7Fress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0B\xD0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01R\x7Fss\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\r\x10W\x81\x81\x10\x15a\r\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07|V[a\r\x10\x84\x84\x84\x84\x03a\n\x8BV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\r\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01R\x7Fdress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0E\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01R\x7Fess\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x0F\x12W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01R\x7Falance\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x85\x85\x03\x90U\x91\x85\x16\x81R\x90\x81 \x80T\x84\x92\x90a\x0FV\x90\x84\x90a\x16\xC4V[\x92PP\x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x0F\xBC\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\r\x10V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x10FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07|V[\x80`\x02`\0\x82\x82Ta\x10X\x91\x90a\x16\xC4V[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x83\x92\x90a\x10\x92\x90\x84\x90a\x16\xC4V[\x90\x91UPP`@Q\x81\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[``\x81`\0\x03a\x11,WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x11VW\x80a\x11@\x81a\x17RV[\x91Pa\x11O\x90P`\n\x83a\x17\xB9V[\x91Pa\x110V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11qWa\x11qa\x17\xCDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x11\x9BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x12\x1EWa\x11\xB0`\x01\x83a\x17\xFCV[\x91Pa\x11\xBD`\n\x86a\x18\x13V[a\x11\xC8\x90`0a\x16\xC4V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x11\xDDWa\x11\xDDa\x18'V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x12\x17`\n\x86a\x17\xB9V[\x94Pa\x11\x9FV[\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x12\xC9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R\x7Fs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x13\x7FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01R\x7Fce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x83\x83\x03\x90U`\x02\x80T\x84\x92\x90a\x13\xBB\x90\x84\x90a\x17\xFCV[\x90\x91UPP`@Q\x82\x81R`\0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\x0C2V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x14\xA5Wa\x14\xA5a\x14\x0BV[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x14\xD5W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x14\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xDFV[\x83\x81\x11\x15a\r\x10WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x15'\x81`@\x85\x01` \x87\x01a\x14\xDCV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15}W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x98Wa\x15\x98a\x14\x0BV[a\x15\xA1\x83a\x15YV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15\xC7Wa\x15\xC7a\x14\x0BV[a\x15\xD0\x84a\x15YV[\x92Pa\x15\xDE` \x85\x01a\x15YV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x16\x03Wa\x16\x03a\x14\x0BV[a\x14\xD5\x82a\x15YV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\"Wa\x16\"a\x14\x0BV[a\x16+\x83a\x15YV[\x91Pa\x169` \x84\x01a\x15YV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x16VW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x16\x8FW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x16\xD7Wa\x16\xD7a\x16\x95V[P\x01\x90V[`\0\x84Qa\x16\xEE\x81\x84` \x89\x01a\x14\xDCV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x17*\x81`\x01\x85\x01` \x8A\x01a\x14\xDCV[`\x01\x92\x01\x91\x82\x01R\x83Qa\x17E\x81`\x02\x84\x01` \x88\x01a\x14\xDCV[\x01`\x02\x01\x95\x94PPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x17\x83Wa\x17\x83a\x16\x95V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x17\xC8Wa\x17\xC8a\x17\x8AV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x18\x0EWa\x18\x0Ea\x16\x95V[P\x03\x90V[`\0\x82a\x18\"Wa\x18\"a\x17\x8AV[P\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x0F\0\n\xA1dsolcC\0\x08\x0F\0\n";
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

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
pub mod example_clone_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("implementation_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("createAddressClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createAddressClone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arg"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createClone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("randomCalldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createDynBytesClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createDynBytesClone",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arg"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createFixedBytesClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createFixedBytesClone",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arg"),
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
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createUint64Clone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createUint64Clone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arg"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createUint8Clone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createUint8Clone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arg"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createUintArrayClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createUintArrayClone",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arg"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createUintClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createUintClone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arg"),
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
                                    name: ::std::borrow::ToOwned::to_owned("clone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("implementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ExampleClone"),
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
    pub static EXAMPLECLONEFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x0B\xE28\x03\x80a\x0B\xE2\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\xA1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x01\x1CV[`\0` \x82\x84\x03\x12\x15a\0\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x15W`\0\x80\xFD[\x93\x92PPPV[a\n\xB7\x80a\x01+`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01%W`\x005`\xE0\x1C\x80c\x8F%\x83*\x11a\0\xF8W\x80c\xC4x\x9Cd\x11a\0\xDDW\x80c\xC4x\x9Cd\x14a\x02TW\x80c\xD68\xC1\xD4\x14a\x02AW\x80c\xE3u\x19C\x14a\x02gWa\x01%V[\x80c\x8F%\x83*\x14a\x02.W\x80c\x9A\xEAG\xB1\x14a\x02AWa\x01%V[\x80c\x1C.Q\r\x14a\x01\xACW\x80cK8\x96\xDE\x14a\x01\xE8W\x80cZ\xD7\x87s\x14a\x01\xFBW\x80c\\`\xDA\x1B\x14a\x02\x0EW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xBFa\x01\xBA6`\x04a\x072V[a\x02zV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\xBFa\x01\xF66`\x04a\x072V[a\x02\xE9V[a\x01\xBFa\x02\t6`\x04a\x08mV[a\x03\x13V[`\0Ta\x01\xBF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xBFa\x02<6`\x04a\x08\x9AV[a\x03OV[a\x01\xBFa\x02O6`\x04a\x08\xD3V[a\x03\x8BV[a\x01\xBFa\x02b6`\x04a\x08\xEFV[a\x03\xA1V[a\x01\xBFa\x02u6`\x04a\t\x15V[a\x03\xDDV[`\0\x80\x82`@Q` \x01a\x02\x8E\x91\x90a\nEV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R`\0T\x90\x91Pa\x02\xE2\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82a\x03\xF1V[\x93\x92PPPV[`\0\x80Ta\x03\r\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83a\x03\xF1V[\x92\x91PPV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x83\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`(\x01a\x02\x8EV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`4\x01a\x02\x8EV[`\0\x80\x82`@Q` \x01a\x02\x8E\x91\x81R` \x01\x90V[`@Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x83\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`!\x01a\x02\x8EV[`\0\x80\x82`@Q` \x01a\x02\x8E\x91\x90a\ntV[`\0`\x02\x82Q\x01`?\x81\x01`\n\x81\x03`@Q\x83`X\x1B\x82`\xE8\x1B\x17\x7Fa\0\0=\x81`\n=9\xF36==7====a\0\0\x80`5696\x01=s\0\0\x17\x81R\x86``\x1B`\x1E\x82\x01R\x7FZ\xF4==\x93\x80>`3W\xFD[\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`2\x82\x01R\x85Q\x91P`?\x81\x01` \x87\x01[` \x84\x10a\x04\xA9W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x93\x01\x92` \x91\x82\x01\x91\x01a\x04lV[Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x90\x03`\x03\x1B\x1B\x16\x81R`\xF0\x85\x90\x1B\x90\x83\x01R\x82\x81`\0\xF0\x94P\x84a\x05\x16W\x7F\xEB\xFE\xF1\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R` `\0\xFD[\x90\x91\x01`@RP\x90\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07*Wa\x07*a\x06\xB4V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x07HWa\x07Ha\x05%V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07cWa\x07ca\x05\xAAV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07zWa\x07za\x06/V[\x815\x81\x81\x11\x15a\x07\x8CWa\x07\x8Ca\x06\xB4V[a\x07\xBC\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x06\xE3V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x08QW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x82Wa\x08\x82a\x05%V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xE2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xAFWa\x08\xAFa\x05%V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xE2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xE8Wa\x08\xE8a\x05%V[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\t\x04Wa\t\x04a\x05%V[\x815`\xFF\x81\x16\x81\x14a\x02\xE2W`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\t+Wa\t+a\x05%V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tFWa\tFa\x05\xAAV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t]Wa\t]a\x06/V[\x815\x81\x81\x11\x15a\toWa\toa\x06\xB4V[\x80`\x05\x1B\x91Pa\t\x80\x84\x83\x01a\x06\xE3V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\n\x1BW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x85`\x04\x84\x01R`+`$\x84\x01R\x7FABI decoding: invalid calldata a`D\x84\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\n9W\x845\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\n V[\x98\x97PPPPPPPPV[`\0\x82Q`\0[\x81\x81\x10\x15a\nfW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\nLV[P`\0\x92\x01\x91\x82RP\x91\x90PV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a\n\x9EW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a\n\x82V[P\x92\x96\x95PPPPPPV\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The bytecode of the contract.
    pub static EXAMPLECLONEFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01%W`\x005`\xE0\x1C\x80c\x8F%\x83*\x11a\0\xF8W\x80c\xC4x\x9Cd\x11a\0\xDDW\x80c\xC4x\x9Cd\x14a\x02TW\x80c\xD68\xC1\xD4\x14a\x02AW\x80c\xE3u\x19C\x14a\x02gWa\x01%V[\x80c\x8F%\x83*\x14a\x02.W\x80c\x9A\xEAG\xB1\x14a\x02AWa\x01%V[\x80c\x1C.Q\r\x14a\x01\xACW\x80cK8\x96\xDE\x14a\x01\xE8W\x80cZ\xD7\x87s\x14a\x01\xFBW\x80c\\`\xDA\x1B\x14a\x02\x0EW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xBFa\x01\xBA6`\x04a\x072V[a\x02zV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\xBFa\x01\xF66`\x04a\x072V[a\x02\xE9V[a\x01\xBFa\x02\t6`\x04a\x08mV[a\x03\x13V[`\0Ta\x01\xBF\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xBFa\x02<6`\x04a\x08\x9AV[a\x03OV[a\x01\xBFa\x02O6`\x04a\x08\xD3V[a\x03\x8BV[a\x01\xBFa\x02b6`\x04a\x08\xEFV[a\x03\xA1V[a\x01\xBFa\x02u6`\x04a\t\x15V[a\x03\xDDV[`\0\x80\x82`@Q` \x01a\x02\x8E\x91\x90a\nEV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R`\0T\x90\x91Pa\x02\xE2\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82a\x03\xF1V[\x93\x92PPPV[`\0\x80Ta\x03\r\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83a\x03\xF1V[\x92\x91PPV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x83\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`(\x01a\x02\x8EV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`4\x01a\x02\x8EV[`\0\x80\x82`@Q` \x01a\x02\x8E\x91\x81R` \x01\x90V[`@Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x83\x90\x1B\x16` \x82\x01R`\0\x90\x81\x90`!\x01a\x02\x8EV[`\0\x80\x82`@Q` \x01a\x02\x8E\x91\x90a\ntV[`\0`\x02\x82Q\x01`?\x81\x01`\n\x81\x03`@Q\x83`X\x1B\x82`\xE8\x1B\x17\x7Fa\0\0=\x81`\n=9\xF36==7====a\0\0\x80`5696\x01=s\0\0\x17\x81R\x86``\x1B`\x1E\x82\x01R\x7FZ\xF4==\x93\x80>`3W\xFD[\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`2\x82\x01R\x85Q\x91P`?\x81\x01` \x87\x01[` \x84\x10a\x04\xA9W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x93\x01\x92` \x91\x82\x01\x91\x01a\x04lV[Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x90\x03`\x03\x1B\x1B\x16\x81R`\xF0\x85\x90\x1B\x90\x83\x01R\x82\x81`\0\xF0\x94P\x84a\x05\x16W\x7F\xEB\xFE\xF1\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R` `\0\xFD[\x90\x91\x01`@RP\x90\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07*Wa\x07*a\x06\xB4V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x07HWa\x07Ha\x05%V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07cWa\x07ca\x05\xAAV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07zWa\x07za\x06/V[\x815\x81\x81\x11\x15a\x07\x8CWa\x07\x8Ca\x06\xB4V[a\x07\xBC\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x06\xE3V[\x91P\x80\x82R\x86\x84\x82\x85\x01\x01\x11\x15a\x08QW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x90\x82\x01\x90\x93\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x82Wa\x08\x82a\x05%V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xE2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xAFWa\x08\xAFa\x05%V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xE2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xE8Wa\x08\xE8a\x05%V[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\t\x04Wa\t\x04a\x05%V[\x815`\xFF\x81\x16\x81\x14a\x02\xE2W`\0\x80\xFD[`\0` \x80\x83\x85\x03\x12\x15a\t+Wa\t+a\x05%V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tFWa\tFa\x05\xAAV[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t]Wa\t]a\x06/V[\x815\x81\x81\x11\x15a\toWa\toa\x06\xB4V[\x80`\x05\x1B\x91Pa\t\x80\x84\x83\x01a\x06\xE3V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a\n\x1BW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x85`\x04\x84\x01R`+`$\x84\x01R\x7FABI decoding: invalid calldata a`D\x84\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\n9W\x845\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\n V[\x98\x97PPPPPPPPV[`\0\x82Q`\0[\x81\x81\x10\x15a\nfW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\nLV[P`\0\x92\x01\x91\x82RP\x91\x90PV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a\n\x9EW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a\n\x82V[P\x92\x96\x95PPPPPPV\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The deployed bytecode of the contract.
    pub static EXAMPLECLONEFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ExampleCloneFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExampleCloneFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExampleCloneFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExampleCloneFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExampleCloneFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExampleCloneFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExampleCloneFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXAMPLECLONEFACTORY_ABI.clone(),
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
                EXAMPLECLONEFACTORY_ABI.clone(),
                EXAMPLECLONEFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `createAddressClone` (0x8f25832a) function
        pub fn create_address_clone(
            &self,
            arg: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([143, 37, 131, 42], arg)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createClone` (0x4b3896de) function
        pub fn create_clone(
            &self,
            random_calldata: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([75, 56, 150, 222], random_calldata)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createDynBytesClone` (0x1c2e510d) function
        pub fn create_dyn_bytes_clone(
            &self,
            arg: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([28, 46, 81, 13], arg)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFixedBytesClone` (0xd638c1d4) function
        pub fn create_fixed_bytes_clone(
            &self,
            arg: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([214, 56, 193, 212], arg)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createUint64Clone` (0x5ad78773) function
        pub fn create_uint_64_clone(
            &self,
            arg: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([90, 215, 135, 115], arg)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createUint8Clone` (0xc4789c64) function
        pub fn create_uint_8_clone(
            &self,
            arg: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 120, 156, 100], arg)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createUintArrayClone` (0xe3751943) function
        pub fn create_uint_array_clone(
            &self,
            arg: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([227, 117, 25, 67], arg)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createUintClone` (0x9aea47b1) function
        pub fn create_uint_clone(
            &self,
            arg: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([154, 234, 71, 177], arg)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementation` (0x5c60da1b) function
        pub fn implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExampleCloneFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `createAddressClone` function with signature `createAddressClone(address)` and selector `0x8f25832a`
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
    #[ethcall(name = "createAddressClone", abi = "createAddressClone(address)")]
    pub struct CreateAddressCloneCall {
        pub arg: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createClone` function with signature `createClone(bytes)` and selector `0x4b3896de`
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
    #[ethcall(name = "createClone", abi = "createClone(bytes)")]
    pub struct CreateCloneCall {
        pub random_calldata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `createDynBytesClone` function with signature `createDynBytesClone(bytes)` and selector `0x1c2e510d`
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
    #[ethcall(name = "createDynBytesClone", abi = "createDynBytesClone(bytes)")]
    pub struct CreateDynBytesCloneCall {
        pub arg: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `createFixedBytesClone` function with signature `createFixedBytesClone(bytes32)` and selector `0xd638c1d4`
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
    #[ethcall(name = "createFixedBytesClone", abi = "createFixedBytesClone(bytes32)")]
    pub struct CreateFixedBytesCloneCall {
        pub arg: [u8; 32],
    }
    ///Container type for all input parameters for the `createUint64Clone` function with signature `createUint64Clone(uint64)` and selector `0x5ad78773`
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
    #[ethcall(name = "createUint64Clone", abi = "createUint64Clone(uint64)")]
    pub struct CreateUint64CloneCall {
        pub arg: u64,
    }
    ///Container type for all input parameters for the `createUint8Clone` function with signature `createUint8Clone(uint8)` and selector `0xc4789c64`
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
    #[ethcall(name = "createUint8Clone", abi = "createUint8Clone(uint8)")]
    pub struct CreateUint8CloneCall {
        pub arg: u8,
    }
    ///Container type for all input parameters for the `createUintArrayClone` function with signature `createUintArrayClone(uint256[])` and selector `0xe3751943`
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
    #[ethcall(name = "createUintArrayClone", abi = "createUintArrayClone(uint256[])")]
    pub struct CreateUintArrayCloneCall {
        pub arg: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `createUintClone` function with signature `createUintClone(uint256)` and selector `0x9aea47b1`
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
    #[ethcall(name = "createUintClone", abi = "createUintClone(uint256)")]
    pub struct CreateUintCloneCall {
        pub arg: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExampleCloneFactoryCalls {
        CreateAddressClone(CreateAddressCloneCall),
        CreateClone(CreateCloneCall),
        CreateDynBytesClone(CreateDynBytesCloneCall),
        CreateFixedBytesClone(CreateFixedBytesCloneCall),
        CreateUint64Clone(CreateUint64CloneCall),
        CreateUint8Clone(CreateUint8CloneCall),
        CreateUintArrayClone(CreateUintArrayCloneCall),
        CreateUintClone(CreateUintCloneCall),
        Implementation(ImplementationCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleCloneFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CreateAddressCloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateAddressClone(decoded));
            }
            if let Ok(decoded) = <CreateCloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateClone(decoded));
            }
            if let Ok(decoded) = <CreateDynBytesCloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateDynBytesClone(decoded));
            }
            if let Ok(decoded) = <CreateFixedBytesCloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFixedBytesClone(decoded));
            }
            if let Ok(decoded) = <CreateUint64CloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateUint64Clone(decoded));
            }
            if let Ok(decoded) = <CreateUint8CloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateUint8Clone(decoded));
            }
            if let Ok(decoded) = <CreateUintArrayCloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateUintArrayClone(decoded));
            }
            if let Ok(decoded) = <CreateUintCloneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateUintClone(decoded));
            }
            if let Ok(decoded) = <ImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Implementation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleCloneFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreateAddressClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateDynBytesClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateFixedBytesClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateUint64Clone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateUint8Clone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateUintArrayClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateUintClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Implementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExampleCloneFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreateAddressClone(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateClone(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateDynBytesClone(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateFixedBytesClone(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateUint64Clone(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateUint8Clone(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateUintArrayClone(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateUintClone(element) => ::core::fmt::Display::fmt(element, f),
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateAddressCloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateAddressCloneCall) -> Self {
            Self::CreateAddressClone(value)
        }
    }
    impl ::core::convert::From<CreateCloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateCloneCall) -> Self {
            Self::CreateClone(value)
        }
    }
    impl ::core::convert::From<CreateDynBytesCloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateDynBytesCloneCall) -> Self {
            Self::CreateDynBytesClone(value)
        }
    }
    impl ::core::convert::From<CreateFixedBytesCloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateFixedBytesCloneCall) -> Self {
            Self::CreateFixedBytesClone(value)
        }
    }
    impl ::core::convert::From<CreateUint64CloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateUint64CloneCall) -> Self {
            Self::CreateUint64Clone(value)
        }
    }
    impl ::core::convert::From<CreateUint8CloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateUint8CloneCall) -> Self {
            Self::CreateUint8Clone(value)
        }
    }
    impl ::core::convert::From<CreateUintArrayCloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateUintArrayCloneCall) -> Self {
            Self::CreateUintArrayClone(value)
        }
    }
    impl ::core::convert::From<CreateUintCloneCall> for ExampleCloneFactoryCalls {
        fn from(value: CreateUintCloneCall) -> Self {
            Self::CreateUintClone(value)
        }
    }
    impl ::core::convert::From<ImplementationCall> for ExampleCloneFactoryCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    ///Container type for all return fields from the `createAddressClone` function with signature `createAddressClone(address)` and selector `0x8f25832a`
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
    pub struct CreateAddressCloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createClone` function with signature `createClone(bytes)` and selector `0x4b3896de`
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
    pub struct CreateCloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createDynBytesClone` function with signature `createDynBytesClone(bytes)` and selector `0x1c2e510d`
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
    pub struct CreateDynBytesCloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createFixedBytesClone` function with signature `createFixedBytesClone(bytes32)` and selector `0xd638c1d4`
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
    pub struct CreateFixedBytesCloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createUint64Clone` function with signature `createUint64Clone(uint64)` and selector `0x5ad78773`
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
    pub struct CreateUint64CloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createUint8Clone` function with signature `createUint8Clone(uint8)` and selector `0xc4789c64`
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
    pub struct CreateUint8CloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createUintArrayClone` function with signature `createUintArrayClone(uint256[])` and selector `0xe3751943`
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
    pub struct CreateUintArrayCloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createUintClone` function with signature `createUintClone(uint256)` and selector `0x9aea47b1`
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
    pub struct CreateUintCloneReturn {
        pub clone: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    pub struct ImplementationReturn(pub ::ethers::core::types::Address);
}

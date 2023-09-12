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
pub mod l2_to_l1_message_passer {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MESSAGE_VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MESSAGE_VERSION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initiateWithdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initiateWithdrawal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                (
                    ::std::borrow::ToOwned::to_owned("messageNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("messageNonce"),
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
                    ::std::borrow::ToOwned::to_owned("sentMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sentMessages"),
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
                    ::std::borrow::ToOwned::to_owned("MessagePassed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MessagePassed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawerBalanceBurnt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawerBalanceBurnt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static L2TOL1MESSAGEPASSER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x01`\x80R`\0`\xA0R`\x02`\xC0R`\x80Q`\xA0Q`\xC0Qa\x0FYa\0\x9C`\09`\0a\x07\x0F\x01R`\0a\x06\xE6\x01R`\0a\x06\xBD\x01Ra\x0FY`\0\xF3\xFE`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\x82\xE3p-\x11a\0CW\x80c\x82\xE3p-\x14a\x02\xFEW\x80c\xC2\xB3\xE5\xAC\x14a\x03\xC0W\x80c\xEC\xC7\x04(\x14a\x03\xD3Wa\0\x8DV[\x80c?\x82zZ\x14a\x01\x14W\x80cD\xDF\x8Ep\x14a\x01\xC3W\x80cT\xFDMP\x14a\x02ZWa\0\x8DV[6a\0\x8DWa\0\x8B3b\x01\x86\xA0`@Q\x80` \x01`@R\x80`\0\x81RPa\x04\xBAV[\0[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01\x90\x81R\x7Fk defined\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x01\xA2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x01\xAB`\x01\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02QW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\0\x8Ba\x06~V[4\x80\x15a\x02\xE8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02\xF1a\x06\xB6V[`@Qa\x01\xBA\x91\x90a\t\x98V[4\x80\x15a\x03\x8CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xB0a\x03\x9B6`\x04a\n7V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBAV[a\0\x8Ba\x03\xCE6`\x04a\x0C6V[a\x04\xBAV[4\x80\x15a\x04aW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04\xAC`\x01T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[`@Q\x90\x81R` \x01a\x01\xBAV[`\0a\x05P`@Q\x80`\xC0\x01`@R\x80a\x05\x14`\x01T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x81R3` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`@\x82\x01R4``\x82\x01R`\x80\x81\x01\x86\x90R`\xA0\x01\x84\x90Ra\x07YV[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163a\x05\xEB`\x01T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x7F\x02\xA5#g\xD1\x07B\xD8\x03'\x12\xC1\xBB\x8E\x01D\xFF\x1E\xC5\xFF\xDA\x1E\xD7\xD7\x0B\xB0Z'D\x95PT4\x87\x87\x87`@Qa\x06 \x94\x93\x92\x91\x90a\r+V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x80T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83\x01\x16\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPV[Ga\x06\x88\x81a\x07\xA6V[`@Q\x81\x90\x7Fyg\xDEazZ\xC1\xCC~\xBA-o7W\n\x015\xAF\xA9P\xD8\xBBw\xCD\xD3_\r\x0BN\x85\xA1o\x90`\0\x90\xA2PV[``a\x06\xE1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xD5V[a\x07\n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xD5V[a\x073\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xD5V[`@Q` \x01a\x07E\x93\x92\x91\x90a\r[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a\x07\x89\x97\x90\x96\x95\x91\x01a\r\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80`@Qa\x07\xB3\x90a\t\x12V[`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\x07\xD0W=`\0\x80>=`\0\xFD[PPPV[``\x81`\0\x03a\x08\x18WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x08BW\x80a\x08,\x81a\x0EWV[\x91Pa\x08;\x90P`\n\x83a\x0E\xBEV[\x91Pa\x08\x1CV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08]Wa\x08]a\nSV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x08\x87W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\t\nWa\x08\x9C`\x01\x83a\x0E\xD2V[\x91Pa\x08\xA9`\n\x86a\x0E\xE9V[a\x08\xB4\x90`0a\x0E\xFDV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x08\xC9Wa\x08\xC9a\x0F\x15V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\t\x03`\n\x86a\x0E\xBEV[\x94Pa\x08\x8BV[\x94\x93PPPPV[`\x08\x80a\x0FE\x839\x01\x90V[`\0[\x83\x81\x10\x15a\t9W\x81\x81\x01Q\x83\x82\x01R` \x01a\t!V[\x83\x81\x11\x15a\tHW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\tf\x81` \x86\x01` \x86\x01a\t\x1EV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\t\xAB` \x83\x01\x84a\tNV[\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\nLWa\nLa\t\xB2V[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x0B\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B:Wa\x0B:a\nSV[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\x80Wa\x0B\x80a\nSV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0C\x1BW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[a\x0C,\x84` \x83\x01` \x89\x01a\n\x82V[\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0CNWa\x0CNa\t\xB2V[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CrW`\0\x80\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a\r!\x86\x82\x87\x01a\n\x8EV[\x91PP\x92P\x92P\x92V[\x84\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0a\rJ`\x80\x83\x01\x85a\tNV[\x90P\x82``\x83\x01R\x95\x94PPPPPV[`\0\x84Qa\rm\x81\x84` \x89\x01a\t\x1EV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\r\xA9\x81`\x01\x85\x01` \x8A\x01a\t\x1EV[`\x01\x92\x01\x91\x82\x01R\x83Qa\r\xC4\x81`\x02\x84\x01` \x88\x01a\t\x1EV[\x01`\x02\x01\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x0E\x1C`\xC0\x83\x01\x84a\tNV[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\x88Wa\x0E\x88a\x0E(V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x0E\xCDWa\x0E\xCDa\x0E\x8FV[P\x04\x90V[`\0\x82\x82\x10\x15a\x0E\xE4Wa\x0E\xE4a\x0E(V[P\x03\x90V[`\0\x82a\x0E\xF8Wa\x0E\xF8a\x0E\x8FV[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\x0F\x10Wa\x0F\x10a\x0E(V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE`\x80`@R0\xFF\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static L2TOL1MESSAGEPASSER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\x82\xE3p-\x11a\0CW\x80c\x82\xE3p-\x14a\x02\xFEW\x80c\xC2\xB3\xE5\xAC\x14a\x03\xC0W\x80c\xEC\xC7\x04(\x14a\x03\xD3Wa\0\x8DV[\x80c?\x82zZ\x14a\x01\x14W\x80cD\xDF\x8Ep\x14a\x01\xC3W\x80cT\xFDMP\x14a\x02ZWa\0\x8DV[6a\0\x8DWa\0\x8B3b\x01\x86\xA0`@Q\x80` \x01`@R\x80`\0\x81RPa\x04\xBAV[\0[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FUnknown signature and no fallbac`D\x82\x01\x90\x81R\x7Fk defined\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[4\x80\x15a\x01\xA2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x01\xAB`\x01\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02QW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\0\x8Ba\x06~V[4\x80\x15a\x02\xE8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x02\xF1a\x06\xB6V[`@Qa\x01\xBA\x91\x90a\t\x98V[4\x80\x15a\x03\x8CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x03\xB0a\x03\x9B6`\x04a\n7V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBAV[a\0\x8Ba\x03\xCE6`\x04a\x0C6V[a\x04\xBAV[4\x80\x15a\x04aW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[Pa\x04\xAC`\x01T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[`@Q\x90\x81R` \x01a\x01\xBAV[`\0a\x05P`@Q\x80`\xC0\x01`@R\x80a\x05\x14`\x01T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x81R3` \x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`@\x82\x01R4``\x82\x01R`\x80\x81\x01\x86\x90R`\xA0\x01\x84\x90Ra\x07YV[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163a\x05\xEB`\x01T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16~\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x7F\x02\xA5#g\xD1\x07B\xD8\x03'\x12\xC1\xBB\x8E\x01D\xFF\x1E\xC5\xFF\xDA\x1E\xD7\xD7\x0B\xB0Z'D\x95PT4\x87\x87\x87`@Qa\x06 \x94\x93\x92\x91\x90a\r+V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x80T}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83\x01\x16\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90UPPV[Ga\x06\x88\x81a\x07\xA6V[`@Q\x81\x90\x7Fyg\xDEazZ\xC1\xCC~\xBA-o7W\n\x015\xAF\xA9P\xD8\xBBw\xCD\xD3_\r\x0BN\x85\xA1o\x90`\0\x90\xA2PV[``a\x06\xE1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xD5V[a\x07\n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xD5V[a\x073\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xD5V[`@Q` \x01a\x07E\x93\x92\x91\x90a\r[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a\x07\x89\x97\x90\x96\x95\x91\x01a\r\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80`@Qa\x07\xB3\x90a\t\x12V[`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\x07\xD0W=`\0\x80>=`\0\xFD[PPPV[``\x81`\0\x03a\x08\x18WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81`\0[\x81\x15a\x08BW\x80a\x08,\x81a\x0EWV[\x91Pa\x08;\x90P`\n\x83a\x0E\xBEV[\x91Pa\x08\x1CV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08]Wa\x08]a\nSV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x08\x87W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\t\nWa\x08\x9C`\x01\x83a\x0E\xD2V[\x91Pa\x08\xA9`\n\x86a\x0E\xE9V[a\x08\xB4\x90`0a\x0E\xFDV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x08\xC9Wa\x08\xC9a\x0F\x15V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\t\x03`\n\x86a\x0E\xBEV[\x94Pa\x08\x8BV[\x94\x93PPPPV[`\x08\x80a\x0FE\x839\x01\x90V[`\0[\x83\x81\x10\x15a\t9W\x81\x81\x01Q\x83\x82\x01R` \x01a\t!V[\x83\x81\x11\x15a\tHW`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\tf\x81` \x86\x01` \x86\x01a\t\x1EV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\t\xAB` \x83\x01\x84a\tNV[\x93\x92PPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\nLWa\nLa\t\xB2V[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x82\x81\x837P`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a\x0B\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B:Wa\x0B:a\nSV[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\x80Wa\x0B\x80a\nSV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0C\x1BW`@Q\x92P\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` `\x04\x84\x01R`'`$\x84\x01R\x7FABI decoding: invalid byte array`D\x84\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x84\x01R`\x84\x83\xFD[a\x0C,\x84` \x83\x01` \x89\x01a\n\x82V[\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0CNWa\x0CNa\t\xB2V[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CrW`\0\x80\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a\r!\x86\x82\x87\x01a\n\x8EV[\x91PP\x92P\x92P\x92V[\x84\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0a\rJ`\x80\x83\x01\x85a\tNV[\x90P\x82``\x83\x01R\x95\x94PPPPPV[`\0\x84Qa\rm\x81\x84` \x89\x01a\t\x1EV[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\r\xA9\x81`\x01\x85\x01` \x8A\x01a\t\x1EV[`\x01\x92\x01\x91\x82\x01R\x83Qa\r\xC4\x81`\x02\x84\x01` \x88\x01a\t\x1EV[\x01`\x02\x01\x95\x94PPPPPV[\x86\x81R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16` \x84\x01R\x80\x87\x16`@\x84\x01RP\x84``\x83\x01R\x83`\x80\x83\x01R`\xC0`\xA0\x83\x01Ra\x0E\x1C`\xC0\x83\x01\x84a\tNV[\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0E\x88Wa\x0E\x88a\x0E(V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x0E\xCDWa\x0E\xCDa\x0E\x8FV[P\x04\x90V[`\0\x82\x82\x10\x15a\x0E\xE4Wa\x0E\xE4a\x0E(V[P\x03\x90V[`\0\x82a\x0E\xF8Wa\x0E\xF8a\x0E\x8FV[P\x06\x90V[`\0\x82\x19\x82\x11\x15a\x0F\x10Wa\x0F\x10a\x0E(V[P\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE`\x80`@R0\xFF\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static L2TOL1MESSAGEPASSER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L2ToL1MessagePasser<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L2ToL1MessagePasser<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L2ToL1MessagePasser<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L2ToL1MessagePasser<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L2ToL1MessagePasser<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L2ToL1MessagePasser))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L2ToL1MessagePasser<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L2TOL1MESSAGEPASSER_ABI.clone(),
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
                L2TOL1MESSAGEPASSER_ABI.clone(),
                L2TOL1MESSAGEPASSER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MESSAGE_VERSION` (0x3f827a5a) function
        pub fn message_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([63, 130, 122, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x44df8e70) function
        pub fn burn(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 223, 142, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initiateWithdrawal` (0xc2b3e5ac) function
        pub fn initiate_withdrawal(
            &self,
            target: ::ethers::core::types::Address,
            gas_limit: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 179, 229, 172], (target, gas_limit, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messageNonce` (0xecc70428) function
        pub fn message_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 199, 4, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sentMessages` (0x82e3702d) function
        pub fn sent_messages(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([130, 227, 112, 45], p0)
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
        ///Gets the contract's `MessagePassed` event
        pub fn message_passed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MessagePassedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawerBalanceBurnt` event
        pub fn withdrawer_balance_burnt_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawerBalanceBurntFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            L2ToL1MessagePasserEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L2ToL1MessagePasser<M> {
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
        name = "MessagePassed",
        abi = "MessagePassed(uint256,address,address,uint256,uint256,bytes,bytes32)"
    )]
    pub struct MessagePassedFilter {
        #[ethevent(indexed)]
        pub nonce: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub gas_limit: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub withdrawal_hash: [u8; 32],
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
    #[ethevent(name = "WithdrawerBalanceBurnt", abi = "WithdrawerBalanceBurnt(uint256)")]
    pub struct WithdrawerBalanceBurntFilter {
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L2ToL1MessagePasserEvents {
        MessagePassedFilter(MessagePassedFilter),
        WithdrawerBalanceBurntFilter(WithdrawerBalanceBurntFilter),
    }
    impl ::ethers::contract::EthLogDecode for L2ToL1MessagePasserEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MessagePassedFilter::decode_log(log) {
                return Ok(L2ToL1MessagePasserEvents::MessagePassedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawerBalanceBurntFilter::decode_log(log) {
                return Ok(
                    L2ToL1MessagePasserEvents::WithdrawerBalanceBurntFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L2ToL1MessagePasserEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MessagePassedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawerBalanceBurntFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MessagePassedFilter> for L2ToL1MessagePasserEvents {
        fn from(value: MessagePassedFilter) -> Self {
            Self::MessagePassedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawerBalanceBurntFilter>
    for L2ToL1MessagePasserEvents {
        fn from(value: WithdrawerBalanceBurntFilter) -> Self {
            Self::WithdrawerBalanceBurntFilter(value)
        }
    }
    ///Container type for all input parameters for the `MESSAGE_VERSION` function with signature `MESSAGE_VERSION()` and selector `0x3f827a5a`
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
    #[ethcall(name = "MESSAGE_VERSION", abi = "MESSAGE_VERSION()")]
    pub struct MessageVersionCall;
    ///Container type for all input parameters for the `burn` function with signature `burn()` and selector `0x44df8e70`
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
    #[ethcall(name = "burn", abi = "burn()")]
    pub struct BurnCall;
    ///Container type for all input parameters for the `initiateWithdrawal` function with signature `initiateWithdrawal(address,uint256,bytes)` and selector `0xc2b3e5ac`
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
        name = "initiateWithdrawal",
        abi = "initiateWithdrawal(address,uint256,bytes)"
    )]
    pub struct InitiateWithdrawalCall {
        pub target: ::ethers::core::types::Address,
        pub gas_limit: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `messageNonce` function with signature `messageNonce()` and selector `0xecc70428`
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
    #[ethcall(name = "messageNonce", abi = "messageNonce()")]
    pub struct MessageNonceCall;
    ///Container type for all input parameters for the `sentMessages` function with signature `sentMessages(bytes32)` and selector `0x82e3702d`
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
    #[ethcall(name = "sentMessages", abi = "sentMessages(bytes32)")]
    pub struct SentMessagesCall(pub [u8; 32]);
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
    pub enum L2ToL1MessagePasserCalls {
        MessageVersion(MessageVersionCall),
        Burn(BurnCall),
        InitiateWithdrawal(InitiateWithdrawalCall),
        MessageNonce(MessageNonceCall),
        SentMessages(SentMessagesCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for L2ToL1MessagePasserCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MessageVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MessageVersion(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <InitiateWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitiateWithdrawal(decoded));
            }
            if let Ok(decoded) = <MessageNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MessageNonce(decoded));
            }
            if let Ok(decoded) = <SentMessagesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SentMessages(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L2ToL1MessagePasserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MessageVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitiateWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MessageNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SentMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for L2ToL1MessagePasserCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MessageVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitiateWithdrawal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MessageNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::SentMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MessageVersionCall> for L2ToL1MessagePasserCalls {
        fn from(value: MessageVersionCall) -> Self {
            Self::MessageVersion(value)
        }
    }
    impl ::core::convert::From<BurnCall> for L2ToL1MessagePasserCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<InitiateWithdrawalCall> for L2ToL1MessagePasserCalls {
        fn from(value: InitiateWithdrawalCall) -> Self {
            Self::InitiateWithdrawal(value)
        }
    }
    impl ::core::convert::From<MessageNonceCall> for L2ToL1MessagePasserCalls {
        fn from(value: MessageNonceCall) -> Self {
            Self::MessageNonce(value)
        }
    }
    impl ::core::convert::From<SentMessagesCall> for L2ToL1MessagePasserCalls {
        fn from(value: SentMessagesCall) -> Self {
            Self::SentMessages(value)
        }
    }
    impl ::core::convert::From<VersionCall> for L2ToL1MessagePasserCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `MESSAGE_VERSION` function with signature `MESSAGE_VERSION()` and selector `0x3f827a5a`
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
    pub struct MessageVersionReturn(pub u16);
    ///Container type for all return fields from the `messageNonce` function with signature `messageNonce()` and selector `0xecc70428`
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
    pub struct MessageNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sentMessages` function with signature `sentMessages(bytes32)` and selector `0x82e3702d`
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
    pub struct SentMessagesReturn(pub bool);
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

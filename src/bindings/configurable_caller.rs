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
pub mod configurable_caller {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("call"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("call"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDoRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDoRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_doRevert"),
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
                    ::std::borrow::ToOwned::to_owned("setPayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPayload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payload"),
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
                    ::std::borrow::ToOwned::to_owned("setTarget"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTarget"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WhatHappened"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WhatHappened"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("returndata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONFIGURABLECALLER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\xFF\x19\x16`\x01\x17\x90U4\x80\x15a\0jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\t\xA7\x80a\0z`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c(\xB5\xE3+\x14a\x01BW\x80cwm\x1A\x01\x14a\x01JW\x80cx\x1D\xD7P\x14a\x01\xA4W\x80c\xDE\xB3\xCD\xF2\x14a\x01\xE3W[`\0T`\xFF\x16\x15a\x01@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FConfigurableCaller: revert\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\0[a\x01@a\x01\xF6V[a\x01@a\x01X6`\x04a\x03\xC2V[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x01@a\x01\xB26`\x04a\x04\x02V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[a\x01@a\x01\xF16`\x04a\x04\xACV[a\x030V[`\0T`\xFF\x16\x15a\x02cW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FConfigurableCaller: revert\0\0\0\0\0\0`D\x82\x01R`d\x01a\x017V[`\0\x80T`@Q\x82\x91a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a\x02\x94\x90`\x01\x90a\x06\xF1V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x02\xD1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xD6V[``\x91P[P\x91P\x91P\x7F!\x06\xB1l\x04\xED|(\x10.\x8F[\xAE*\x81\xF8\xF9i4n\xFA=9\x7F\x8D\xD4\x88yC\x85\xD4\x14\x82\x82`@Qa\x03\x0C\x92\x91\x90a\x07\x85V[`@Q\x80\x91\x03\x90\xA1\x81\x80\x15a\x03#W\x81Q` \x83\x01\xF3[\x81Q` \x83\x01\xFD[PPPV[`\x01a\x03+\x82\x84\x83a\x08\x7FV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xD7Wa\x03\xD7a\x03=V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xFBW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x04\x17Wa\x04\x17a\x03=V[\x815\x80\x15\x15\x81\x14a\x03\xFBW`\0\x80\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x80\x84\x86\x03\x12\x15a\x04\xC3Wa\x04\xC3a\x03=V[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05ZW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x06{W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x87\x84\x82\x85\x01\x01\x11\x15a\x06\x8FWa\x06\x8Fa\x04'V[\x91\x90\x92\x01\x96\x90\x95P\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xB2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xEBW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80\x83Ta\x06\xFF\x81a\x06\x9EV[`\x01\x82\x81\x16\x80\x15a\x07\x17W`\x01\x81\x14a\x07JWa\x07yV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x07yV[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x07pW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x07WV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x82\x15\x15\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x07\xBBW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x07\x9FV[\x81\x81\x11\x15a\x07\xCDW`\0``\x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x03+W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x08XWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08wW\x82\x81U`\x01\x01a\x08dV[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x08\x97Wa\x08\x97a\x08\x02V[a\x08\xAB\x83a\x08\xA5\x83Ta\x06\x9EV[\x83a\x081V[`\0`\x1F\x84\x11`\x01\x81\x14a\x08\xFDW`\0\x85\x15a\x08\xC7WP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\t\x93V[`\0\x83\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x90\x83[\x82\x81\x10\x15a\tLW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\t,V[P\x86\x82\x10\x15a\t\x87W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static CONFIGURABLECALLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80c(\xB5\xE3+\x14a\x01BW\x80cwm\x1A\x01\x14a\x01JW\x80cx\x1D\xD7P\x14a\x01\xA4W\x80c\xDE\xB3\xCD\xF2\x14a\x01\xE3W[`\0T`\xFF\x16\x15a\x01@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FConfigurableCaller: revert\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\0[a\x01@a\x01\xF6V[a\x01@a\x01X6`\x04a\x03\xC2V[`\0\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16a\x01\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x01@a\x01\xB26`\x04a\x04\x02V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[a\x01@a\x01\xF16`\x04a\x04\xACV[a\x030V[`\0T`\xFF\x16\x15a\x02cW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FConfigurableCaller: revert\0\0\0\0\0\0`D\x82\x01R`d\x01a\x017V[`\0\x80T`@Q\x82\x91a\x01\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a\x02\x94\x90`\x01\x90a\x06\xF1V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x02\xD1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xD6V[``\x91P[P\x91P\x91P\x7F!\x06\xB1l\x04\xED|(\x10.\x8F[\xAE*\x81\xF8\xF9i4n\xFA=9\x7F\x8D\xD4\x88yC\x85\xD4\x14\x82\x82`@Qa\x03\x0C\x92\x91\x90a\x07\x85V[`@Q\x80\x91\x03\x90\xA1\x81\x80\x15a\x03#W\x81Q` \x83\x01\xF3[\x81Q` \x83\x01\xFD[PPPV[`\x01a\x03+\x82\x84\x83a\x08\x7FV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xD7Wa\x03\xD7a\x03=V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xFBW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x04\x17Wa\x04\x17a\x03=V[\x815\x80\x15\x15\x81\x14a\x03\xFBW`\0\x80\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray stride\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0\x80` \x80\x84\x86\x03\x12\x15a\x04\xC3Wa\x04\xC3a\x03=V[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05ZW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815\x81\x81\x11\x15a\x06{W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x84`\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x87\x84\x82\x85\x01\x01\x11\x15a\x06\x8FWa\x06\x8Fa\x04'V[\x91\x90\x92\x01\x96\x90\x95P\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xB2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xEBW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80\x83Ta\x06\xFF\x81a\x06\x9EV[`\x01\x82\x81\x16\x80\x15a\x07\x17W`\x01\x81\x14a\x07JWa\x07yV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x07yV[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x07pW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x07WV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x82\x15\x15\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x07\xBBW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x07\x9FV[\x81\x81\x11\x15a\x07\xCDW`\0``\x83\x87\x01\x01R[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x03+W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x08XWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x08wW\x82\x81U`\x01\x01a\x08dV[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x08\x97Wa\x08\x97a\x08\x02V[a\x08\xAB\x83a\x08\xA5\x83Ta\x06\x9EV[\x83a\x081V[`\0`\x1F\x84\x11`\x01\x81\x14a\x08\xFDW`\0\x85\x15a\x08\xC7WP\x83\x82\x015[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\t\x93V[`\0\x83\x81R` \x90 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x16\x90\x83[\x82\x81\x10\x15a\tLW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\t,V[P\x86\x82\x10\x15a\t\x87W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static CONFIGURABLECALLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ConfigurableCaller<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConfigurableCaller<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConfigurableCaller<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConfigurableCaller<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConfigurableCaller<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ConfigurableCaller))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConfigurableCaller<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONFIGURABLECALLER_ABI.clone(),
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
                CONFIGURABLECALLER_ABI.clone(),
                CONFIGURABLECALLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `call` (0x28b5e32b) function
        pub fn call(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 181, 227, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDoRevert` (0x781dd750) function
        pub fn set_do_revert(
            &self,
            do_revert: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 29, 215, 80], do_revert)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPayload` (0xdeb3cdf2) function
        pub fn set_payload(
            &self,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 179, 205, 242], payload)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTarget` (0x776d1a01) function
        pub fn set_target(
            &self,
            target: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 109, 26, 1], target)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `WhatHappened` event
        pub fn what_happened_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WhatHappenedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WhatHappenedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConfigurableCaller<M> {
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
    #[ethevent(name = "WhatHappened", abi = "WhatHappened(bool,bytes)")]
    pub struct WhatHappenedFilter {
        pub success: bool,
        pub returndata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `call` function with signature `call()` and selector `0x28b5e32b`
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
    #[ethcall(name = "call", abi = "call()")]
    pub struct CallCall;
    ///Container type for all input parameters for the `setDoRevert` function with signature `setDoRevert(bool)` and selector `0x781dd750`
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
    #[ethcall(name = "setDoRevert", abi = "setDoRevert(bool)")]
    pub struct SetDoRevertCall {
        pub do_revert: bool,
    }
    ///Container type for all input parameters for the `setPayload` function with signature `setPayload(bytes)` and selector `0xdeb3cdf2`
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
    #[ethcall(name = "setPayload", abi = "setPayload(bytes)")]
    pub struct SetPayloadCall {
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setTarget` function with signature `setTarget(address)` and selector `0x776d1a01`
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
    #[ethcall(name = "setTarget", abi = "setTarget(address)")]
    pub struct SetTargetCall {
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConfigurableCallerCalls {
        Call(CallCall),
        SetDoRevert(SetDoRevertCall),
        SetPayload(SetPayloadCall),
        SetTarget(SetTargetCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConfigurableCallerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Call(decoded));
            }
            if let Ok(decoded) = <SetDoRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDoRevert(decoded));
            }
            if let Ok(decoded) = <SetPayloadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPayload(decoded));
            }
            if let Ok(decoded) = <SetTargetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTarget(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConfigurableCallerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Call(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDoRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTarget(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConfigurableCallerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Call(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDoRevert(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTarget(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CallCall> for ConfigurableCallerCalls {
        fn from(value: CallCall) -> Self {
            Self::Call(value)
        }
    }
    impl ::core::convert::From<SetDoRevertCall> for ConfigurableCallerCalls {
        fn from(value: SetDoRevertCall) -> Self {
            Self::SetDoRevert(value)
        }
    }
    impl ::core::convert::From<SetPayloadCall> for ConfigurableCallerCalls {
        fn from(value: SetPayloadCall) -> Self {
            Self::SetPayload(value)
        }
    }
    impl ::core::convert::From<SetTargetCall> for ConfigurableCallerCalls {
        fn from(value: SetTargetCall) -> Self {
            Self::SetTarget(value)
        }
    }
}

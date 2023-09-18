pub use safe_caller_actor::*;
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
pub mod safe_caller_actor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vm"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Vm"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_fails"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("numCalls"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numCalls"),
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
                    ::std::borrow::ToOwned::to_owned("performSafeCallMinGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "performSafeCallMinGas",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SAFECALLER_ACTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\n\xF08\x03\x80a\n\xF0\x839\x81\x01`@\x81\x90Ra\0/\x91a\0ZV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x15\x15`\x80Ra\0\xA5V[`\0\x80`@\x83\x85\x03\x12\x15a\0mW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x84W`\0\x80\xFD[` \x84\x01Q\x90\x92P\x80\x15\x15\x81\x14a\0\x9AW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\x80Qa\n\"a\0\xCE`\09`\0\x81\x81a\x011\x01R\x81\x81a\x03L\x01Ra\x03\x89\x01Ra\n\"`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x82\xF1a\xCA\x14a\0;W\x80c\xE0\xF8\x03[\x14a\0VW[`\0\x80\xFD[a\0D`\x01T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ia\0d6`\x04a\x07\x9DV[a\0kV[\0[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90cLc\xE5b\x90\x84\x16;\x15\x80\x15a\0\xBEWPjconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x14\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDE\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\0\xF6W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x01\nW=`\0\x80>=`\0\xFD[PPPPa\x01-\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xC4e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16a\x03\xD5V[\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x01\x94Wa\x01\x8Dg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16\x90\x85\x16`?a\x01t\x87`@a\x08CV[a\x01~\x91\x90a\x08\xA2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xD5V[\x93Pa\x01\xEEV[a\x01\xEBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`?a\x01\xAF\x86`@a\x08CV[a\x01\xB9\x91\x90a\x08\xA2V[a\x01\xC5\x90a\x9C@a\x08\xC9V[a\x01\xD1\x90a\x03\xE8a\x08\xC9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16a\x03\xD5V[\x93P[`\0\x80T`@Q\x7F\x08\xE4\xE1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x04\x83\x01R`\xFF\x85\x16`$\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`D\x83\x01R`\x80`d\x83\x01R`\x84\x82\x01\x93\x90\x93R\x91\x16\x90c\x08\xE4\xE1\x16\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x97W=`\0\x80>=`\0\xFD[PP`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`D\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x90\x92R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97A\xE3\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\0\x93Pa\x03@\x92P3\x91\x88\x16\x90`\xFF\x86\x16\x90a\x04*V[\x90P\x80\x80\x15a\x03lWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x15a\x03\x87W`\x01\x80T\x90`\0a\x03\x81\x83a\x08\xF5V[\x91\x90PUP[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15\x80\x15a\x03\xB3WP\x80\x15[\x15a\x03\xCEW`\x01\x80T\x90`\0a\x03\xC8\x83a\x08\xF5V[\x91\x90PUP[PPPPPV[`\0a\x03\xE2\x84\x84\x84a\x04DV[\x90Pa\x04#`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x06\x80V[\x93\x92PPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[`\0\x81\x83\x11\x15a\x04\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x04\xEAWP\x81\x84\x11\x15[\x15a\x04\xF6WP\x82a\x04#V[`\0a\x05\x02\x84\x84a\t-V[a\x05\r\x90`\x01a\tDV[\x90P`\x03\x85\x11\x15\x80\x15a\x05\x1FWP\x84\x81\x11[\x15a\x056Wa\x05.\x85\x85a\tDV[\x91PPa\x04#V[a\x05a`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t-V[\x85\x10\x15\x80\x15a\x05\x98WPa\x05\x95\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t-V[\x81\x11[\x15a\x05\xD1Wa\x05\xC7\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t-V[a\x05.\x90\x84a\t-V[\x82\x85\x11\x15a\x06'W`\0a\x05\xE5\x84\x87a\t-V[\x90P`\0a\x05\xF3\x83\x83a\t\\V[\x90P\x80`\0\x03a\x06\x08W\x84\x93PPPPa\x04#V[`\x01a\x06\x14\x82\x88a\tDV[a\x06\x1E\x91\x90a\t-V[\x93PPPa\x06xV[\x83\x85\x10\x15a\x06xW`\0a\x06;\x86\x86a\t-V[\x90P`\0a\x06I\x83\x83a\t\\V[\x90P\x80`\0\x03a\x06^W\x85\x93PPPPa\x04#V[a\x06h\x81\x86a\t-V[a\x06s\x90`\x01a\tDV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x06\xB7\x92\x91\x90a\t\xA0V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x078\x91\x90a\t\xF9V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x07sW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07xV[``\x91P[PPPPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x98W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x07\xB3W`\0\x80\xFD[a\x07\xBC\x85a\x07\x80V[\x93Pa\x07\xCA` \x86\x01a\x07\x80V[\x92P`@\x85\x015s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xF3W`\0\x80\xFD[\x91P``\x85\x015`\xFF\x81\x16\x81\x14a\x08\tW`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a\x08jWa\x08ja\x08\x14V[\x02\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x08\xBDWa\x08\xBDa\x08sV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x08\xECWa\x08\xECa\x08\x14V[\x01\x94\x93PPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\t&Wa\t&a\x08\x14V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\t?Wa\t?a\x08\x14V[P\x03\x90V[`\0\x82\x19\x82\x11\x15a\tWWa\tWa\x08\x14V[P\x01\x90V[`\0\x82a\tkWa\tka\x08sV[P\x06\x90V[`\0[\x83\x81\x10\x15a\t\x8BW\x81\x81\x01Q\x83\x82\x01R` \x01a\tsV[\x83\x81\x11\x15a\t\x9AW`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\t\xBF\x81``\x85\x01` \x88\x01a\tpV[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\n\x0B\x81\x84` \x87\x01a\tpV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static SAFECALLER_ACTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x82\xF1a\xCA\x14a\0;W\x80c\xE0\xF8\x03[\x14a\0VW[`\0\x80\xFD[a\0D`\x01T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ia\0d6`\x04a\x07\x9DV[a\0kV[\0[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90cLc\xE5b\x90\x84\x16;\x15\x80\x15a\0\xBEWPjconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x14\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDE\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\0\xF6W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x01\nW=`\0\x80>=`\0\xFD[PPPPa\x01-\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xC4e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16a\x03\xD5V[\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x01\x94Wa\x01\x8Dg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16\x90\x85\x16`?a\x01t\x87`@a\x08CV[a\x01~\x91\x90a\x08\xA2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xD5V[\x93Pa\x01\xEEV[a\x01\xEBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`?a\x01\xAF\x86`@a\x08CV[a\x01\xB9\x91\x90a\x08\xA2V[a\x01\xC5\x90a\x9C@a\x08\xC9V[a\x01\xD1\x90a\x03\xE8a\x08\xC9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16a\x03\xD5V[\x93P[`\0\x80T`@Q\x7F\x08\xE4\xE1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x04\x83\x01R`\xFF\x85\x16`$\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`D\x83\x01R`\x80`d\x83\x01R`\x84\x82\x01\x93\x90\x93R\x91\x16\x90c\x08\xE4\xE1\x16\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x97W=`\0\x80>=`\0\xFD[PP`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16`D\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x90\x92R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97A\xE3\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\0\x93Pa\x03@\x92P3\x91\x88\x16\x90`\xFF\x86\x16\x90a\x04*V[\x90P\x80\x80\x15a\x03lWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x15a\x03\x87W`\x01\x80T\x90`\0a\x03\x81\x83a\x08\xF5V[\x91\x90PUP[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15\x80\x15a\x03\xB3WP\x80\x15[\x15a\x03\xCEW`\x01\x80T\x90`\0a\x03\xC8\x83a\x08\xF5V[\x91\x90PUP[PPPPPV[`\0a\x03\xE2\x84\x84\x84a\x04DV[\x90Pa\x04#`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x06\x80V[\x93\x92PPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[`\0\x81\x83\x11\x15a\x04\xDAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x04\xEAWP\x81\x84\x11\x15[\x15a\x04\xF6WP\x82a\x04#V[`\0a\x05\x02\x84\x84a\t-V[a\x05\r\x90`\x01a\tDV[\x90P`\x03\x85\x11\x15\x80\x15a\x05\x1FWP\x84\x81\x11[\x15a\x056Wa\x05.\x85\x85a\tDV[\x91PPa\x04#V[a\x05a`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t-V[\x85\x10\x15\x80\x15a\x05\x98WPa\x05\x95\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t-V[\x81\x11[\x15a\x05\xD1Wa\x05\xC7\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\t-V[a\x05.\x90\x84a\t-V[\x82\x85\x11\x15a\x06'W`\0a\x05\xE5\x84\x87a\t-V[\x90P`\0a\x05\xF3\x83\x83a\t\\V[\x90P\x80`\0\x03a\x06\x08W\x84\x93PPPPa\x04#V[`\x01a\x06\x14\x82\x88a\tDV[a\x06\x1E\x91\x90a\t-V[\x93PPPa\x06xV[\x83\x85\x10\x15a\x06xW`\0a\x06;\x86\x86a\t-V[\x90P`\0a\x06I\x83\x83a\t\\V[\x90P\x80`\0\x03a\x06^W\x85\x93PPPPa\x04#V[a\x06h\x81\x86a\t-V[a\x06s\x90`\x01a\tDV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x06\xB7\x92\x91\x90a\t\xA0V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x078\x91\x90a\t\xF9V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x07sW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07xV[``\x91P[PPPPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x98W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x07\xB3W`\0\x80\xFD[a\x07\xBC\x85a\x07\x80V[\x93Pa\x07\xCA` \x86\x01a\x07\x80V[\x92P`@\x85\x015s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xF3W`\0\x80\xFD[\x91P``\x85\x015`\xFF\x81\x16\x81\x14a\x08\tW`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a\x08jWa\x08ja\x08\x14V[\x02\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80a\x08\xBDWa\x08\xBDa\x08sV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x08\xECWa\x08\xECa\x08\x14V[\x01\x94\x93PPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\t&Wa\t&a\x08\x14V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\t?Wa\t?a\x08\x14V[P\x03\x90V[`\0\x82\x19\x82\x11\x15a\tWWa\tWa\x08\x14V[P\x01\x90V[`\0\x82a\tkWa\tka\x08sV[P\x06\x90V[`\0[\x83\x81\x10\x15a\t\x8BW\x81\x81\x01Q\x83\x82\x01R` \x01a\tsV[\x83\x81\x11\x15a\t\x9AW`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\t\xBF\x81``\x85\x01` \x88\x01a\tpV[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\n\x0B\x81\x84` \x87\x01a\tpV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static SAFECALLER_ACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SafeCaller_Actor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SafeCaller_Actor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SafeCaller_Actor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SafeCaller_Actor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SafeCaller_Actor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SafeCaller_Actor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SafeCaller_Actor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SAFECALLER_ACTOR_ABI.clone(),
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
                SAFECALLER_ACTOR_ABI.clone(),
                SAFECALLER_ACTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `numCalls` (0x82f161ca) function
        pub fn num_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([130, 241, 97, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `performSafeCallMinGas` (0xe0f8035b) function
        pub fn perform_safe_call_min_gas(
            &self,
            gas: u64,
            min_gas: u64,
            to: ::ethers::core::types::Address,
            value: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 248, 3, 91], (gas, min_gas, to, value))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SafeCaller_Actor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `numCalls` function with signature `numCalls()` and selector `0x82f161ca`
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
    #[ethcall(name = "numCalls", abi = "numCalls()")]
    pub struct NumCallsCall;
    ///Container type for all input parameters for the `performSafeCallMinGas` function with signature `performSafeCallMinGas(uint64,uint64,address,uint8)` and selector `0xe0f8035b`
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
        name = "performSafeCallMinGas",
        abi = "performSafeCallMinGas(uint64,uint64,address,uint8)"
    )]
    pub struct PerformSafeCallMinGasCall {
        pub gas: u64,
        pub min_gas: u64,
        pub to: ::ethers::core::types::Address,
        pub value: u8,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SafeCaller_ActorCalls {
        NumCalls(NumCallsCall),
        PerformSafeCallMinGas(PerformSafeCallMinGasCall),
    }
    impl ::ethers::core::abi::AbiDecode for SafeCaller_ActorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <NumCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumCalls(decoded));
            }
            if let Ok(decoded) = <PerformSafeCallMinGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PerformSafeCallMinGas(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SafeCaller_ActorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::NumCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PerformSafeCallMinGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SafeCaller_ActorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NumCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerformSafeCallMinGas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<NumCallsCall> for SafeCaller_ActorCalls {
        fn from(value: NumCallsCall) -> Self {
            Self::NumCalls(value)
        }
    }
    impl ::core::convert::From<PerformSafeCallMinGasCall> for SafeCaller_ActorCalls {
        fn from(value: PerformSafeCallMinGasCall) -> Self {
            Self::PerformSafeCallMinGas(value)
        }
    }
    ///Container type for all return fields from the `numCalls` function with signature `numCalls()` and selector `0x82f161ca`
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
    pub struct NumCallsReturn(pub ::ethers::core::types::U256);
}

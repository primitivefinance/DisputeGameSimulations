pub use alphabet_vm::*;
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
pub mod alphabet_vm {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_absolutePrestate"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                            32usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("Claim"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("oracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPreimageOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("step"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("step"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stateData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("postState_"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ALPHABETVM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\n\xDD8\x03\x80a\n\xDD\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x90V[`\x80\x81\x90R`@Qa\0@\x90a\0\x83V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\0\\W=`\0\x80>=`\0\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa\0\xA9V[a\x06\xC5\x80a\x04\x18\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\0\xA2W`\0\x80\xFD[PQ\x91\x90PV[`\x80Qa\x03Ua\0\xC3`\09`\0`\xAF\x01Ra\x03U`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c}\xC0\xD1\xD0\x14a\0;W\x80c\xF8\xE0\xCB\x96\x14a\0\x85W[`\0\x80\xFD[`\0Ta\0[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x98a\0\x936`\x04a\x02\x12V[a\0\xA6V[`@Q\x90\x81R` \x01a\0|V[`\0\x80`\0`\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x1B`\x08\x88\x88`@Qa\0\xE2\x92\x91\x90a\x02~V[`@Q\x80\x91\x03\x90 \x90\x1B\x03a\x01\x08W`\0\x91Pa\x01\x01\x86\x88\x01\x88a\x02\x8EV[\x90Pa\x01'V[a\x01\x14\x86\x88\x01\x88a\x02\xA7V[\x90\x92P\x90P\x81a\x01#\x81a\x02\xF8V[\x92PP[\x81a\x013\x82`\x01a\x030V[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x97\x96PPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x01\xDBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xF3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x02\x0BW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x02(W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02@W`\0\x80\xFD[a\x02L\x88\x83\x89\x01a\x01\xC9V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x02eW`\0\x80\xFD[Pa\x02r\x87\x82\x88\x01a\x01\xC9V[\x95\x98\x94\x97P\x95PPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xA0W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xBAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x03)Wa\x03)a\x02\xC9V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\x03CWa\x03Ca\x02\xC9V[P\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x06\xA5\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xE01\x10\xE1\x11a\0[W\x80c\xE01\x10\xE1\x14a\x01\x11W\x80c\xE1Y&\x11\x14a\x019W\x80c\xFEJ\xC0\x8E\x14a\x01NW\x80c\xFE\xF2\xB4\xED\x14a\x01\xC3W`\0\x80\xFD[\x80ca#\x8B\xDE\x14a\0\x82W\x80c\x85B\xCFP\x14a\0\xC0W\x80c\x9A\x1F^\x7F\x14a\0\xFEW[`\0\x80\xFD[a\0\xADa\0\x906`\x04a\x05QV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEEa\0\xCE6`\x04a\x05QV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xB7V[a\0\xADa\x01\x0C6`\x04a\x05sV[a\x01\xE3V[a\x01$a\x01\x1F6`\x04a\x05QV[a\x02\xB6V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xB7V[a\x01La\x01G6`\x04a\x05\xA5V[a\x03\xA7V[\0[a\x01La\x01\\6`\x04a\x05sV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 \x87\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x86\x84R\x82R\x80\x83 \x96\x83R\x95\x81R\x85\x82 \x93\x90\x93U\x92\x83R\x90\x82\x90R\x91\x90 UV[a\0\xADa\x01\xD16`\x04a\x06!V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`\0a\x01\xEE\x85a\x04\xB0V[\x90Pa\x01\xFB\x83`\x08a\x06iV[\x82\x11\x80a\x02\x08WP` \x83\x11[\x15a\x02?W`@Q\x7F\xFE%I\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x81\x81R`\xC0\x85\x90\x1B\x82R`\x08\x95\x90\x95R\x82Q\x82\x82R`\x02\x86R`@\x80\x83 \x85\x84R\x87R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x87R\x80\x83 \x94\x83R\x93\x86R\x83\x82 U\x81\x81R\x93\x84\x90R\x92 U\x91\x90PV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T\x81\x90`\xFF\x16a\x03?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Fpre-image must exist\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P`\0\x83\x81R` \x81\x81R`@\x90\x91 Ta\x03[\x81`\x08a\x06iV[a\x03f\x85` a\x06iV[\x10a\x03\x84W\x83a\x03w\x82`\x08a\x06iV[a\x03\x81\x91\x90a\x06\x81V[\x91P[P`\0\x93\x84R`\x01` \x90\x81R`@\x80\x86 \x94\x86R\x93\x90R\x91\x90\x92 T\x92\x90\x91PV[`D5`\0\x80`\x08\x83\x01\x86\x11\x15a\x03\xC6Wc\xFE%I\x87`\0R`\x04`\x1C\xFD[`\xC0\x83\x90\x1B`\x80R`\x88\x83\x86\x827\x80\x87\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01Q\x90\x84\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x8B\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x90\x81\x17\x90\x91U\x84\x84R\x82R\x80\x83 \x9A\x83R\x99\x81R\x89\x82 \x93\x90\x93U\x90\x81R\x90\x81\x90R\x95\x90\x95 \x91\x90\x91UPPPPV[\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x17a\x05K\x81`\0\x90\x81R3` R`@\x90 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05dW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\x89W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x05\xBAW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xD9W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05\xEDW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xFCW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x06\x0EW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x063W`\0\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x06|Wa\x06|a\x06:V[P\x01\x90V[`\0\x82\x82\x10\x15a\x06\x93Wa\x06\x93a\x06:V[P\x03\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static ALPHABETVM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c}\xC0\xD1\xD0\x14a\0;W\x80c\xF8\xE0\xCB\x96\x14a\0\x85W[`\0\x80\xFD[`\0Ta\0[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x98a\0\x936`\x04a\x02\x12V[a\0\xA6V[`@Q\x90\x81R` \x01a\0|V[`\0\x80`\0`\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x1B`\x08\x88\x88`@Qa\0\xE2\x92\x91\x90a\x02~V[`@Q\x80\x91\x03\x90 \x90\x1B\x03a\x01\x08W`\0\x91Pa\x01\x01\x86\x88\x01\x88a\x02\x8EV[\x90Pa\x01'V[a\x01\x14\x86\x88\x01\x88a\x02\xA7V[\x90\x92P\x90P\x81a\x01#\x81a\x02\xF8V[\x92PP[\x81a\x013\x82`\x01a\x030V[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 ~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x97\x96PPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x01\xDBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xF3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x02\x0BW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x02(W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02@W`\0\x80\xFD[a\x02L\x88\x83\x89\x01a\x01\xC9V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x02eW`\0\x80\xFD[Pa\x02r\x87\x82\x88\x01a\x01\xC9V[\x95\x98\x94\x97P\x95PPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xA0W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xBAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x03)Wa\x03)a\x02\xC9V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\x03CWa\x03Ca\x02\xC9V[P\x01\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static ALPHABETVM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AlphabetVM<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AlphabetVM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AlphabetVM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AlphabetVM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AlphabetVM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AlphabetVM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AlphabetVM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ALPHABETVM_ABI.clone(),
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
                ALPHABETVM_ABI.clone(),
                ALPHABETVM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `oracle` (0x7dc0d1d0) function
        pub fn oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `step` (0xf8e0cb96) function
        pub fn step(
            &self,
            state_data: ::ethers::core::types::Bytes,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([248, 224, 203, 150], (state_data, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AlphabetVM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    ///Container type for all input parameters for the `step` function with signature `step(bytes,bytes)` and selector `0xf8e0cb96`
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
    #[ethcall(name = "step", abi = "step(bytes,bytes)")]
    pub struct StepCall {
        pub state_data: ::ethers::core::types::Bytes,
        pub p1: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AlphabetVMCalls {
        Oracle(OracleCall),
        Step(StepCall),
    }
    impl ::ethers::core::abi::AbiDecode for AlphabetVMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <OracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Oracle(decoded));
            }
            if let Ok(decoded) = <StepCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Step(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AlphabetVMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Oracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Step(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AlphabetVMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Oracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::Step(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OracleCall> for AlphabetVMCalls {
        fn from(value: OracleCall) -> Self {
            Self::Oracle(value)
        }
    }
    impl ::core::convert::From<StepCall> for AlphabetVMCalls {
        fn from(value: StepCall) -> Self {
            Self::Step(value)
        }
    }
    ///Container type for all return fields from the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    pub struct OracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `step` function with signature `step(bytes,bytes)` and selector `0xf8e0cb96`
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
    pub struct StepReturn {
        pub post_state: [u8; 32],
    }
}

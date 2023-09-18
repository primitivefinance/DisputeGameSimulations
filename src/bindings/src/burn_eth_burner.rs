pub use burn_eth_burner::*;
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
pub mod burn_eth_burner {
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("burnEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnEth"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
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
                    ::std::borrow::ToOwned::to_owned("failedEthBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedEthBurn"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BURN_ETHBURNER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07e8\x03\x80a\x07e\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x06\xD2\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80ch\x0B7j\x14a\0;W\x80c\x9A\x99\xF4\xDE\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x05jV[a\0\x89V[\0[`\0Ta\0u\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0a\0\xA6\x82\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\x9EV[`\0T`@Q\x7F\xC8\x8A^m\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC8\x8A^m\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01-W=`\0\x80>=`\0\xFD[PPPP`\0G\x90P`\0a\x01D\x83`\0\x84a\x01\x9EV[\x90Pa\x01O\x81a\x01\xF3V[\x80\x82\x03G\x14a\x01\x98W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U[PPPPV[`\0a\x01\xAB\x84\x84\x84a\x02\"V[\x90Pa\x01\xEC`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x04^V[\x93\x92PPPV[\x80`@Qa\x02\0\x90a\x05^V[`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\x02\x1DW=`\0\x80>=`\0\xFD[PPPV[`\0\x81\x83\x11\x15a\x02\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x02\xC8WP\x81\x84\x11\x15[\x15a\x02\xD4WP\x82a\x01\xECV[`\0a\x02\xE0\x84\x84a\x05\xB2V[a\x02\xEB\x90`\x01a\x05\xC9V[\x90P`\x03\x85\x11\x15\x80\x15a\x02\xFDWP\x84\x81\x11[\x15a\x03\x14Wa\x03\x0C\x85\x85a\x05\xC9V[\x91PPa\x01\xECV[a\x03?`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\xB2V[\x85\x10\x15\x80\x15a\x03vWPa\x03s\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\xB2V[\x81\x11[\x15a\x03\xAFWa\x03\xA5\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\xB2V[a\x03\x0C\x90\x84a\x05\xB2V[\x82\x85\x11\x15a\x04\x05W`\0a\x03\xC3\x84\x87a\x05\xB2V[\x90P`\0a\x03\xD1\x83\x83a\x05\xE1V[\x90P\x80`\0\x03a\x03\xE6W\x84\x93PPPPa\x01\xECV[`\x01a\x03\xF2\x82\x88a\x05\xC9V[a\x03\xFC\x91\x90a\x05\xB2V[\x93PPPa\x04VV[\x83\x85\x10\x15a\x04VW`\0a\x04\x19\x86\x86a\x05\xB2V[\x90P`\0a\x04'\x83\x83a\x05\xE1V[\x90P\x80`\0\x03a\x04<W\x85\x93PPPPa\x01\xECV[a\x04F\x81\x86a\x05\xB2V[a\x04Q\x90`\x01a\x05\xC9V[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x04\x95\x92\x91\x90a\x06HV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x05\x16\x91\x90a\x06\xA1V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x05QW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05VV[``\x91P[PPPPPPV[`\x08\x80a\x06\xBE\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x05|W`\0\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x05\xC4Wa\x05\xC4a\x05\x83V[P\x03\x90V[`\0\x82\x19\x82\x11\x15a\x05\xDCWa\x05\xDCa\x05\x83V[P\x01\x90V[`\0\x82a\x06\x17W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x067W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\x1FV[\x83\x81\x11\x15a\x01\x98WPP`\0\x91\x01RV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x06g\x81``\x85\x01` \x88\x01a\x06\x1CV[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x06\xB3\x81\x84` \x87\x01a\x06\x1CV[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R0\xFF\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static BURN_ETHBURNER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80ch\x0B7j\x14a\0;W\x80c\x9A\x99\xF4\xDE\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x05jV[a\0\x89V[\0[`\0Ta\0u\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0a\0\xA6\x82\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\x9EV[`\0T`@Q\x7F\xC8\x8A^m\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xC8\x8A^m\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01-W=`\0\x80>=`\0\xFD[PPPP`\0G\x90P`\0a\x01D\x83`\0\x84a\x01\x9EV[\x90Pa\x01O\x81a\x01\xF3V[\x80\x82\x03G\x14a\x01\x98W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U[PPPPV[`\0a\x01\xAB\x84\x84\x84a\x02\"V[\x90Pa\x01\xEC`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x04^V[\x93\x92PPPV[\x80`@Qa\x02\0\x90a\x05^V[`@Q\x80\x91\x03\x90\x82\xF0\x90P\x80\x15\x80\x15a\x02\x1DW=`\0\x80>=`\0\xFD[PPPV[`\0\x81\x83\x11\x15a\x02\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x02\xC8WP\x81\x84\x11\x15[\x15a\x02\xD4WP\x82a\x01\xECV[`\0a\x02\xE0\x84\x84a\x05\xB2V[a\x02\xEB\x90`\x01a\x05\xC9V[\x90P`\x03\x85\x11\x15\x80\x15a\x02\xFDWP\x84\x81\x11[\x15a\x03\x14Wa\x03\x0C\x85\x85a\x05\xC9V[\x91PPa\x01\xECV[a\x03?`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\xB2V[\x85\x10\x15\x80\x15a\x03vWPa\x03s\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\xB2V[\x81\x11[\x15a\x03\xAFWa\x03\xA5\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\xB2V[a\x03\x0C\x90\x84a\x05\xB2V[\x82\x85\x11\x15a\x04\x05W`\0a\x03\xC3\x84\x87a\x05\xB2V[\x90P`\0a\x03\xD1\x83\x83a\x05\xE1V[\x90P\x80`\0\x03a\x03\xE6W\x84\x93PPPPa\x01\xECV[`\x01a\x03\xF2\x82\x88a\x05\xC9V[a\x03\xFC\x91\x90a\x05\xB2V[\x93PPPa\x04VV[\x83\x85\x10\x15a\x04VW`\0a\x04\x19\x86\x86a\x05\xB2V[\x90P`\0a\x04'\x83\x83a\x05\xE1V[\x90P\x80`\0\x03a\x04<W\x85\x93PPPPa\x01\xECV[a\x04F\x81\x86a\x05\xB2V[a\x04Q\x90`\x01a\x05\xC9V[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x04\x95\x92\x91\x90a\x06HV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x05\x16\x91\x90a\x06\xA1V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x05QW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05VV[``\x91P[PPPPPPV[`\x08\x80a\x06\xBE\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x05|W`\0\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x05\xC4Wa\x05\xC4a\x05\x83V[P\x03\x90V[`\0\x82\x19\x82\x11\x15a\x05\xDCWa\x05\xDCa\x05\x83V[P\x01\x90V[`\0\x82a\x06\x17W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x067W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\x1FV[\x83\x81\x11\x15a\x01\x98WPP`\0\x91\x01RV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x06g\x81``\x85\x01` \x88\x01a\x06\x1CV[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x06\xB3\x81\x84` \x87\x01a\x06\x1CV[\x91\x90\x91\x01\x92\x91PPV\xFE`\x80`@R0\xFF\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static BURN_ETHBURNER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Burn_EthBurner<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Burn_EthBurner<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Burn_EthBurner<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Burn_EthBurner<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Burn_EthBurner<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Burn_EthBurner))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Burn_EthBurner<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BURN_ETHBURNER_ABI.clone(),
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
                BURN_ETHBURNER_ABI.clone(),
                BURN_ETHBURNER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `burnEth` (0x680b376a) function
        pub fn burn_eth(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 11, 55, 106], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedEthBurn` (0x9a99f4de) function
        pub fn failed_eth_burn(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([154, 153, 244, 222], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Burn_EthBurner<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `burnEth` function with signature `burnEth(uint256)` and selector `0x680b376a`
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
    #[ethcall(name = "burnEth", abi = "burnEth(uint256)")]
    pub struct BurnEthCall {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `failedEthBurn` function with signature `failedEthBurn()` and selector `0x9a99f4de`
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
    #[ethcall(name = "failedEthBurn", abi = "failedEthBurn()")]
    pub struct FailedEthBurnCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum Burn_EthBurnerCalls {
        BurnEth(BurnEthCall),
        FailedEthBurn(FailedEthBurnCall),
    }
    impl ::ethers::core::abi::AbiDecode for Burn_EthBurnerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BurnEthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnEth(decoded));
            }
            if let Ok(decoded) = <FailedEthBurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedEthBurn(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Burn_EthBurnerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BurnEth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedEthBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Burn_EthBurnerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BurnEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedEthBurn(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnEthCall> for Burn_EthBurnerCalls {
        fn from(value: BurnEthCall) -> Self {
            Self::BurnEth(value)
        }
    }
    impl ::core::convert::From<FailedEthBurnCall> for Burn_EthBurnerCalls {
        fn from(value: FailedEthBurnCall) -> Self {
            Self::FailedEthBurn(value)
        }
    }
    ///Container type for all return fields from the `failedEthBurn` function with signature `failedEthBurn()` and selector `0x9a99f4de`
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
    pub struct FailedEthBurnReturn(pub bool);
}

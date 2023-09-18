pub use burn_gas_burner::*;
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
pub mod burn_gas_burner {
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
                    ::std::borrow::ToOwned::to_owned("burnGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnGas"),
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
                    ::std::borrow::ToOwned::to_owned("failedGasBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedGasBurn"),
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
    pub static BURN_GASBURNER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x06\xFC8\x03\x80a\x06\xFC\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x06i\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cJ\xD5\xD1o\x14a\0;W\x80cy\xDF$Z\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x04\xCDV[a\0\x89V[\0[`\0Ta\0u\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[bz\x12\0`\0a\0\x9A\x83\x82\x84a\x01\x0EV[\x90P`\0Z\x90Pa\0\xAA\x82a\x01cV[`\0Z\x90P\x82\x82\x82\x03\x11\x15\x80\x15a\0\xC2WP\x81\x83\x83\x03\x11[\x15a\x01\x07W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U[PPPPPV[`\0a\x01\x1B\x84\x84\x84a\x01\x91V[\x90Pa\x01\\`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x03\xCDV[\x93\x92PPPV[`\0\x80Z\x90P[\x82Za\x01v\x90\x83a\x05\x15V[\x10\x15a\x01\x8CWa\x01\x85\x82a\x05,V[\x91Pa\x01jV[PPPV[`\0\x81\x83\x11\x15a\x02'W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x027WP\x81\x84\x11\x15[\x15a\x02CWP\x82a\x01\\V[`\0a\x02O\x84\x84a\x05\x15V[a\x02Z\x90`\x01a\x05dV[\x90P`\x03\x85\x11\x15\x80\x15a\x02lWP\x84\x81\x11[\x15a\x02\x83Wa\x02{\x85\x85a\x05dV[\x91PPa\x01\\V[a\x02\xAE`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x15V[\x85\x10\x15\x80\x15a\x02\xE5WPa\x02\xE2\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x15V[\x81\x11[\x15a\x03\x1EWa\x03\x14\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x15V[a\x02{\x90\x84a\x05\x15V[\x82\x85\x11\x15a\x03tW`\0a\x032\x84\x87a\x05\x15V[\x90P`\0a\x03@\x83\x83a\x05|V[\x90P\x80`\0\x03a\x03UW\x84\x93PPPPa\x01\\V[`\x01a\x03a\x82\x88a\x05dV[a\x03k\x91\x90a\x05\x15V[\x93PPPa\x03\xC5V[\x83\x85\x10\x15a\x03\xC5W`\0a\x03\x88\x86\x86a\x05\x15V[\x90P`\0a\x03\x96\x83\x83a\x05|V[\x90P\x80`\0\x03a\x03\xABW\x85\x93PPPPa\x01\\V[a\x03\xB5\x81\x86a\x05\x15V[a\x03\xC0\x90`\x01a\x05dV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x04\x04\x92\x91\x90a\x05\xE7V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x04\x85\x91\x90a\x06@V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x04\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xC5V[``\x91P[PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xDFW`\0\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x05'Wa\x05'a\x04\xE6V[P\x03\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x05]Wa\x05]a\x04\xE6V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\x05wWa\x05wa\x04\xE6V[P\x01\x90V[`\0\x82a\x05\xB2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x05\xD2W\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xBAV[\x83\x81\x11\x15a\x05\xE1W`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x06\x06\x81``\x85\x01` \x88\x01a\x05\xB7V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x06R\x81\x84` \x87\x01a\x05\xB7V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static BURN_GASBURNER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cJ\xD5\xD1o\x14a\0;W\x80cy\xDF$Z\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x04\xCDV[a\0\x89V[\0[`\0Ta\0u\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[bz\x12\0`\0a\0\x9A\x83\x82\x84a\x01\x0EV[\x90P`\0Z\x90Pa\0\xAA\x82a\x01cV[`\0Z\x90P\x82\x82\x82\x03\x11\x15\x80\x15a\0\xC2WP\x81\x83\x83\x03\x11[\x15a\x01\x07W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U[PPPPPV[`\0a\x01\x1B\x84\x84\x84a\x01\x91V[\x90Pa\x01\\`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound Result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82a\x03\xCDV[\x93\x92PPPV[`\0\x80Z\x90P[\x82Za\x01v\x90\x83a\x05\x15V[\x10\x15a\x01\x8CWa\x01\x85\x82a\x05,V[\x91Pa\x01jV[PPPV[`\0\x81\x83\x11\x15a\x02'W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x027WP\x81\x84\x11\x15[\x15a\x02CWP\x82a\x01\\V[`\0a\x02O\x84\x84a\x05\x15V[a\x02Z\x90`\x01a\x05dV[\x90P`\x03\x85\x11\x15\x80\x15a\x02lWP\x84\x81\x11[\x15a\x02\x83Wa\x02{\x85\x85a\x05dV[\x91PPa\x01\\V[a\x02\xAE`\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x15V[\x85\x10\x15\x80\x15a\x02\xE5WPa\x02\xE2\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x15V[\x81\x11[\x15a\x03\x1EWa\x03\x14\x85\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x05\x15V[a\x02{\x90\x84a\x05\x15V[\x82\x85\x11\x15a\x03tW`\0a\x032\x84\x87a\x05\x15V[\x90P`\0a\x03@\x83\x83a\x05|V[\x90P\x80`\0\x03a\x03UW\x84\x93PPPPa\x01\\V[`\x01a\x03a\x82\x88a\x05dV[a\x03k\x91\x90a\x05\x15V[\x93PPPa\x03\xC5V[\x83\x85\x10\x15a\x03\xC5W`\0a\x03\x88\x86\x86a\x05\x15V[\x90P`\0a\x03\x96\x83\x83a\x05|V[\x90P\x80`\0\x03a\x03\xABW\x85\x93PPPPa\x01\\V[a\x03\xB5\x81\x86a\x05\x15V[a\x03\xC0\x90`\x01a\x05dV[\x93PPP[P\x93\x92PPPV[`\0jconsole.logs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83`@Q`$\x01a\x04\x04\x92\x91\x90a\x05\xE7V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa\x04\x85\x91\x90a\x06@V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x04\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xC5V[``\x91P[PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xDFW`\0\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x05'Wa\x05'a\x04\xE6V[P\x03\x90V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x05]Wa\x05]a\x04\xE6V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\x05wWa\x05wa\x04\xE6V[P\x01\x90V[`\0\x82a\x05\xB2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x05\xD2W\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\xBAV[\x83\x81\x11\x15a\x05\xE1W`\0\x84\x84\x01R[PPPPV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x06\x06\x81``\x85\x01` \x88\x01a\x05\xB7V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x06R\x81\x84` \x87\x01a\x05\xB7V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static BURN_GASBURNER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Burn_GasBurner<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Burn_GasBurner<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Burn_GasBurner<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Burn_GasBurner<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Burn_GasBurner<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Burn_GasBurner))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Burn_GasBurner<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BURN_GASBURNER_ABI.clone(),
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
                BURN_GASBURNER_ABI.clone(),
                BURN_GASBURNER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `burnGas` (0x4ad5d16f) function
        pub fn burn_gas(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 213, 209, 111], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedGasBurn` (0x79df245a) function
        pub fn failed_gas_burn(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([121, 223, 36, 90], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Burn_GasBurner<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `burnGas` function with signature `burnGas(uint256)` and selector `0x4ad5d16f`
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
    #[ethcall(name = "burnGas", abi = "burnGas(uint256)")]
    pub struct BurnGasCall {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `failedGasBurn` function with signature `failedGasBurn()` and selector `0x79df245a`
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
    #[ethcall(name = "failedGasBurn", abi = "failedGasBurn()")]
    pub struct FailedGasBurnCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum Burn_GasBurnerCalls {
        BurnGas(BurnGasCall),
        FailedGasBurn(FailedGasBurnCall),
    }
    impl ::ethers::core::abi::AbiDecode for Burn_GasBurnerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BurnGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnGas(decoded));
            }
            if let Ok(decoded) = <FailedGasBurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedGasBurn(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Burn_GasBurnerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BurnGas(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedGasBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Burn_GasBurnerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BurnGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedGasBurn(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnGasCall> for Burn_GasBurnerCalls {
        fn from(value: BurnGasCall) -> Self {
            Self::BurnGas(value)
        }
    }
    impl ::core::convert::From<FailedGasBurnCall> for Burn_GasBurnerCalls {
        fn from(value: FailedGasBurnCall) -> Self {
            Self::FailedGasBurn(value)
        }
    }
    ///Container type for all return fields from the `failedGasBurn` function with signature `failedGasBurn()` and selector `0x79df245a`
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
    pub struct FailedGasBurnReturn(pub bool);
}

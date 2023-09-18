pub use resolved_delegate_proxy::*;
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
pub mod resolved_delegate_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_addressManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract AddressManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_implementationName"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RESOLVEDDELEGATEPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x05\xF08\x03\x80a\x05\xF0\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x88V[0`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x16\x17\x90U\x90\x82\x90R\x90 a\0j\x82\x82a\x02\x03V[PPPa\x02\xC2V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\0\x9BW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB2W`\0\x80\xFD[` \x84\x81\x01Q\x91\x93P\x90`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\0\xD1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\0\xE5W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\0\xF7Wa\0\xF7a\0rV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01\x1FWa\x01\x1Fa\0rV[\x81`@R\x82\x81R\x89\x86\x84\x87\x01\x01\x11\x15a\x017W`\0\x80\xFD[`\0\x93P[\x82\x84\x10\x15a\x01YW\x84\x84\x01\x86\x01Q\x81\x85\x01\x87\x01R\x92\x85\x01\x92a\x01<V[\x82\x84\x11\x15a\x01jW`\0\x86\x84\x83\x01\x01R[\x80\x96PPPPPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x8EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xAEWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x01\xFEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x01\xDBWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xFAW\x82\x81U`\x01\x01a\x01\xE7V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x1CWa\x02\x1Ca\0rV[a\x020\x81a\x02*\x84Ta\x01zV[\x84a\x01\xB4V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x02eW`\0\x84\x15a\x02MWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x01\xFAV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x02\x94W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x02uV[P\x85\x82\x10\x15a\x02\xB2W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x03\x1F\x80a\x02\xD1`\09`\0\xF3\xFE`\x80`@\x81\x81R0`\0\x90\x81R`\x01` \x90\x81R\x82\x82 T\x90\x82\x90R\x91\x81 \x7F\xBF@\xFA\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93R\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xBF@\xFA\xC1\x90a\0m\x90`\x84a\x01\xE2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAE\x91\x90a\x02\xC5V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x01WW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FResolvedDelegateProxy: target ad`D\x82\x01R\x7Fdress must be initialized\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x006`@Qa\x01\x82\x92\x91\x90a\x03\x02V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xBDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xC2V[``\x91P[P\x90\x92P\x90P\x81\x15\x15`\x01\x03a\x01\xDAW\x80Q` \x82\x01\xF3[\x80Q` \x82\x01\xFD[`\0` \x80\x83R`\0\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x02\x04W`\x7F\x83\x16\x92P[\x85\x83\x10\x81\x03a\x02:W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\"`\x04R`$\x85\xFD[\x87\x86\x01\x83\x81R` \x01\x81\x80\x15a\x02WW`\x01\x81\x14a\x02\x8BWa\x02\xB6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x86\x16\x82R\x84\x15\x15`\x05\x1B\x82\x01\x96Pa\x02\xB6V[`\0\x8B\x81R` \x90 `\0[\x86\x81\x10\x15a\x02\xB0W\x81T\x84\x82\x01R\x90\x85\x01\x90\x89\x01a\x02\x97V[\x83\x01\x97PP[P\x94\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x02\xD7W`\0\x80\xFD[\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xFBW`\0\x80\xFD[\x93\x92PPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static RESOLVEDDELEGATEPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R0`\0\x90\x81R`\x01` \x90\x81R\x82\x82 T\x90\x82\x90R\x91\x81 \x7F\xBF@\xFA\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93R\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xBF@\xFA\xC1\x90a\0m\x90`\x84a\x01\xE2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAE\x91\x90a\x02\xC5V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x01WW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FResolvedDelegateProxy: target ad`D\x82\x01R\x7Fdress must be initialized\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x006`@Qa\x01\x82\x92\x91\x90a\x03\x02V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xBDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xC2V[``\x91P[P\x90\x92P\x90P\x81\x15\x15`\x01\x03a\x01\xDAW\x80Q` \x82\x01\xF3[\x80Q` \x82\x01\xFD[`\0` \x80\x83R`\0\x84T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80a\x02\x04W`\x7F\x83\x16\x92P[\x85\x83\x10\x81\x03a\x02:W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\"`\x04R`$\x85\xFD[\x87\x86\x01\x83\x81R` \x01\x81\x80\x15a\x02WW`\x01\x81\x14a\x02\x8BWa\x02\xB6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x86\x16\x82R\x84\x15\x15`\x05\x1B\x82\x01\x96Pa\x02\xB6V[`\0\x8B\x81R` \x90 `\0[\x86\x81\x10\x15a\x02\xB0W\x81T\x84\x82\x01R\x90\x85\x01\x90\x89\x01a\x02\x97V[\x83\x01\x97PP[P\x94\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x02\xD7W`\0\x80\xFD[\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xFBW`\0\x80\xFD[\x93\x92PPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static RESOLVEDDELEGATEPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ResolvedDelegateProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ResolvedDelegateProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ResolvedDelegateProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ResolvedDelegateProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ResolvedDelegateProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ResolvedDelegateProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ResolvedDelegateProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESOLVEDDELEGATEPROXY_ABI.clone(),
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
                RESOLVEDDELEGATEPROXY_ABI.clone(),
                RESOLVEDDELEGATEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ResolvedDelegateProxy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}

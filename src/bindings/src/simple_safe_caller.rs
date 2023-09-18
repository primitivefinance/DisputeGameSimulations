pub use simple_safe_caller::*;
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
pub mod simple_safe_caller {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("a"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("a"),
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
                    ::std::borrow::ToOwned::to_owned("makeSafeCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makeSafeCall"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("makeSafeCallMinGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makeSafeCallMinGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setA"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_a"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SIMPLESAFECALLER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\xF7\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x0C'$n\x14a\0QW\x80c\r\xBEg\x1F\x14a\0yW\x80c\xDE\xBC\xF5\xCC\x14a\0\x90W\x80c\xEE\x91\x9DP\x14a\0\xA3W[`\0\x80\xFD[a\0da\0_6`\x04a\x02\x83V[a\0\xB8V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x82`\0T\x81V[`@Q\x90\x81R` \x01a\0pV[a\0da\0\x9E6`\x04a\x02\x9EV[a\x01CV[a\0\xB6a\0\xB16`\x04a\x02\xD1V[`\0UV[\0[`@\x80Q`\x01`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xEE\x91\x9DP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\0\x90a\x01=\x900\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x84\x90a\x01\xD0V[\x92\x91PPV[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`$\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`D\x90\x92\x01\x90\x92R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0C'$n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\0\x91a\x01\xC9\x910\x91\x86\x16\x90\x84\x90a\x02.V[\x93\x92PPPV[`\0\x80`\0a\x01\xE0\x86`\0a\x02HV[\x90P\x80a\x02\x16Wc\x08\xC3y\xA0`\0R` \x80Rx\x18SafeCall: Not enough gas`XR`d`\x1C\xFD[`\0\x80\x85Q` \x87\x01\x88\x8BZ\xF1\x97\x96PPPPPPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02~W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\x95W`\0\x80\xFD[a\x01\xC9\x82a\x02fV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xB1W`\0\x80\xFD[a\x02\xBA\x83a\x02fV[\x91Pa\x02\xC8` \x84\x01a\x02fV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xE3W`\0\x80\xFD[P5\x91\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static SIMPLESAFECALLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x0C'$n\x14a\0QW\x80c\r\xBEg\x1F\x14a\0yW\x80c\xDE\xBC\xF5\xCC\x14a\0\x90W\x80c\xEE\x91\x9DP\x14a\0\xA3W[`\0\x80\xFD[a\0da\0_6`\x04a\x02\x83V[a\0\xB8V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x82`\0T\x81V[`@Q\x90\x81R` \x01a\0pV[a\0da\0\x9E6`\x04a\x02\x9EV[a\x01CV[a\0\xB6a\0\xB16`\x04a\x02\xD1V[`\0UV[\0[`@\x80Q`\x01`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xEE\x91\x9DP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\0\x90a\x01=\x900\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90\x84\x90a\x01\xD0V[\x92\x91PPV[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`$\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`D\x90\x92\x01\x90\x92R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0C'$n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\0\x91a\x01\xC9\x910\x91\x86\x16\x90\x84\x90a\x02.V[\x93\x92PPPV[`\0\x80`\0a\x01\xE0\x86`\0a\x02HV[\x90P\x80a\x02\x16Wc\x08\xC3y\xA0`\0R` \x80Rx\x18SafeCall: Not enough gas`XR`d`\x1C\xFD[`\0\x80\x85Q` \x87\x01\x88\x8BZ\xF1\x97\x96PPPPPPPV[`\0\x80`\0\x80\x84Q` \x86\x01\x87\x8A\x8A\xF1\x96\x95PPPPPPV[`\0\x80`?\x83a\x9C@\x01\x02`@\x85\x02\x01`?Z\x02\x10\x15\x94\x93PPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02~W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\x95W`\0\x80\xFD[a\x01\xC9\x82a\x02fV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xB1W`\0\x80\xFD[a\x02\xBA\x83a\x02fV[\x91Pa\x02\xC8` \x84\x01a\x02fV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xE3W`\0\x80\xFD[P5\x91\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static SIMPLESAFECALLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SimpleSafeCaller<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleSafeCaller<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SimpleSafeCaller<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SimpleSafeCaller<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SimpleSafeCaller<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SimpleSafeCaller))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleSafeCaller<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SIMPLESAFECALLER_ABI.clone(),
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
                SIMPLESAFECALLER_ABI.clone(),
                SIMPLESAFECALLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `a` (0x0dbe671f) function
        pub fn a(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([13, 190, 103, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makeSafeCall` (0xdebcf5cc) function
        pub fn make_safe_call(
            &self,
            gas: u64,
            min_gas: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([222, 188, 245, 204], (gas, min_gas))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makeSafeCallMinGas` (0x0c27246e) function
        pub fn make_safe_call_min_gas(
            &self,
            min_gas: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([12, 39, 36, 110], min_gas)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setA` (0xee919d50) function
        pub fn set_a(
            &self,
            a: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 145, 157, 80], a)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SimpleSafeCaller<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `a` function with signature `a()` and selector `0x0dbe671f`
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
    #[ethcall(name = "a", abi = "a()")]
    pub struct ACall;
    ///Container type for all input parameters for the `makeSafeCall` function with signature `makeSafeCall(uint64,uint64)` and selector `0xdebcf5cc`
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
    #[ethcall(name = "makeSafeCall", abi = "makeSafeCall(uint64,uint64)")]
    pub struct MakeSafeCallCall {
        pub gas: u64,
        pub min_gas: u64,
    }
    ///Container type for all input parameters for the `makeSafeCallMinGas` function with signature `makeSafeCallMinGas(uint64)` and selector `0x0c27246e`
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
    #[ethcall(name = "makeSafeCallMinGas", abi = "makeSafeCallMinGas(uint64)")]
    pub struct MakeSafeCallMinGasCall {
        pub min_gas: u64,
    }
    ///Container type for all input parameters for the `setA` function with signature `setA(uint256)` and selector `0xee919d50`
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
    #[ethcall(name = "setA", abi = "setA(uint256)")]
    pub struct SetACall {
        pub a: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimpleSafeCallerCalls {
        A(ACall),
        MakeSafeCall(MakeSafeCallCall),
        MakeSafeCallMinGas(MakeSafeCallMinGasCall),
        SetA(SetACall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleSafeCallerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ACall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::A(decoded));
            }
            if let Ok(decoded) = <MakeSafeCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakeSafeCall(decoded));
            }
            if let Ok(decoded) = <MakeSafeCallMinGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakeSafeCallMinGas(decoded));
            }
            if let Ok(decoded) = <SetACall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetA(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleSafeCallerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::A(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MakeSafeCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakeSafeCallMinGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetA(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SimpleSafeCallerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::A(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakeSafeCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakeSafeCallMinGas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetA(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ACall> for SimpleSafeCallerCalls {
        fn from(value: ACall) -> Self {
            Self::A(value)
        }
    }
    impl ::core::convert::From<MakeSafeCallCall> for SimpleSafeCallerCalls {
        fn from(value: MakeSafeCallCall) -> Self {
            Self::MakeSafeCall(value)
        }
    }
    impl ::core::convert::From<MakeSafeCallMinGasCall> for SimpleSafeCallerCalls {
        fn from(value: MakeSafeCallMinGasCall) -> Self {
            Self::MakeSafeCallMinGas(value)
        }
    }
    impl ::core::convert::From<SetACall> for SimpleSafeCallerCalls {
        fn from(value: SetACall) -> Self {
            Self::SetA(value)
        }
    }
    ///Container type for all return fields from the `a` function with signature `a()` and selector `0x0dbe671f`
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
    pub struct AReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `makeSafeCall` function with signature `makeSafeCall(uint64,uint64)` and selector `0xdebcf5cc`
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
    pub struct MakeSafeCallReturn(pub bool);
    ///Container type for all return fields from the `makeSafeCallMinGas` function with signature `makeSafeCallMinGas(uint64)` and selector `0x0c27246e`
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
    pub struct MakeSafeCallMinGasReturn(pub bool);
}

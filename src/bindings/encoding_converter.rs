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
pub mod encoding_converter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("convertRoundTripAToB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "convertRoundTripAToB",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        240usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint240"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("convertRoundTripBToA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "convertRoundTripBToA",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_versionedNonce"),
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
                    ::std::borrow::ToOwned::to_owned("failedRoundtripAToB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedRoundtripAToB",
                            ),
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
                (
                    ::std::borrow::ToOwned::to_owned("failedRoundtripBToA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "failedRoundtripBToA",
                            ),
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
    pub static ENCODING_CONVERTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x03\xD2\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80cT\xC0?%\x14a\x01UW\x80c^5R9\x14a\x01vW\x80c\x87\x1C\x9F\xBC\x14a\x01\x88W\x80c\xFA\xC4\x9B\xD5\x14a\x01\x9DW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\0Ta\x01b\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\x01b\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x01\x9Ba\x01\x966`\x04a\x03CV[a\x01\xB0V[\0[a\x01\x9Ba\x01\xAB6`\x04a\x03_V[a\x025V[}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\xF0\x82\x90\x1C\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x82\x17\x80\x84\x14a\x02/W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[PPPPV[`\0a\x02C\x83\x83`\xF0\x1B\x17\x90V[\x90P}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x90`\xF0\x83\x90\x1C\x90\x85\x16\x82\x14\x15\x80a\x02\x86WP\x83a\xFF\xFF\x16\x81a\xFF\xFF\x16\x14\x15[\x15a\x02\xB7W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[PPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x03XWa\x03Xa\x02\xBEV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03uWa\x03ua\x02\xBEV[\x825}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xA3W`\0\x80\xFD[\x91P` \x83\x015a\xFF\xFF\x81\x16\x81\x14a\x03\xBAW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static ENCODING_CONVERTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80cT\xC0?%\x14a\x01UW\x80c^5R9\x14a\x01vW\x80c\x87\x1C\x9F\xBC\x14a\x01\x88W\x80c\xFA\xC4\x9B\xD5\x14a\x01\x9DW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\0Ta\x01b\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\x01b\x90a\x01\0\x90\x04`\xFF\x16\x81V[a\x01\x9Ba\x01\x966`\x04a\x03CV[a\x01\xB0V[\0[a\x01\x9Ba\x01\xAB6`\x04a\x03_V[a\x025V[}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\xF0\x82\x90\x1C\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x82\x17\x80\x84\x14a\x02/W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90U[PPPPV[`\0a\x02C\x83\x83`\xF0\x1B\x17\x90V[\x90P}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x90`\xF0\x83\x90\x1C\x90\x85\x16\x82\x14\x15\x80a\x02\x86WP\x83a\xFF\xFF\x16\x81a\xFF\xFF\x16\x14\x15[\x15a\x02\xB7W`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U[PPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x03XWa\x03Xa\x02\xBEV[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03uWa\x03ua\x02\xBEV[\x825}\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xA3W`\0\x80\xFD[\x91P` \x83\x015a\xFF\xFF\x81\x16\x81\x14a\x03\xBAW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static ENCODING_CONVERTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Encoding_Converter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Encoding_Converter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Encoding_Converter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Encoding_Converter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Encoding_Converter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Encoding_Converter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Encoding_Converter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ENCODING_CONVERTER_ABI.clone(),
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
                ENCODING_CONVERTER_ABI.clone(),
                ENCODING_CONVERTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `convertRoundTripAToB` (0xfac49bd5) function
        pub fn convert_round_trip_a_to_b(
            &self,
            nonce: ::ethers::core::types::U256,
            version: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 196, 155, 213], (nonce, version))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertRoundTripBToA` (0x871c9fbc) function
        pub fn convert_round_trip_b_to_a(
            &self,
            versioned_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 28, 159, 188], versioned_nonce)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedRoundtripAToB` (0x54c03f25) function
        pub fn failed_roundtrip_a_to_b(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([84, 192, 63, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedRoundtripBToA` (0x5e355239) function
        pub fn failed_roundtrip_b_to_a(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([94, 53, 82, 57], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Encoding_Converter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `convertRoundTripAToB` function with signature `convertRoundTripAToB(uint240,uint16)` and selector `0xfac49bd5`
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
        name = "convertRoundTripAToB",
        abi = "convertRoundTripAToB(uint240,uint16)"
    )]
    pub struct ConvertRoundTripAToBCall {
        pub nonce: ::ethers::core::types::U256,
        pub version: u16,
    }
    ///Container type for all input parameters for the `convertRoundTripBToA` function with signature `convertRoundTripBToA(uint256)` and selector `0x871c9fbc`
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
    #[ethcall(name = "convertRoundTripBToA", abi = "convertRoundTripBToA(uint256)")]
    pub struct ConvertRoundTripBToACall {
        pub versioned_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `failedRoundtripAToB` function with signature `failedRoundtripAToB()` and selector `0x54c03f25`
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
    #[ethcall(name = "failedRoundtripAToB", abi = "failedRoundtripAToB()")]
    pub struct FailedRoundtripAToBCall;
    ///Container type for all input parameters for the `failedRoundtripBToA` function with signature `failedRoundtripBToA()` and selector `0x5e355239`
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
    #[ethcall(name = "failedRoundtripBToA", abi = "failedRoundtripBToA()")]
    pub struct FailedRoundtripBToACall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum Encoding_ConverterCalls {
        ConvertRoundTripAToB(ConvertRoundTripAToBCall),
        ConvertRoundTripBToA(ConvertRoundTripBToACall),
        FailedRoundtripAToB(FailedRoundtripAToBCall),
        FailedRoundtripBToA(FailedRoundtripBToACall),
    }
    impl ::ethers::core::abi::AbiDecode for Encoding_ConverterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ConvertRoundTripAToBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertRoundTripAToB(decoded));
            }
            if let Ok(decoded) = <ConvertRoundTripBToACall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertRoundTripBToA(decoded));
            }
            if let Ok(decoded) = <FailedRoundtripAToBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedRoundtripAToB(decoded));
            }
            if let Ok(decoded) = <FailedRoundtripBToACall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedRoundtripBToA(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Encoding_ConverterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConvertRoundTripAToB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertRoundTripBToA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedRoundtripAToB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedRoundtripBToA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Encoding_ConverterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConvertRoundTripAToB(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConvertRoundTripBToA(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedRoundtripAToB(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedRoundtripBToA(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ConvertRoundTripAToBCall> for Encoding_ConverterCalls {
        fn from(value: ConvertRoundTripAToBCall) -> Self {
            Self::ConvertRoundTripAToB(value)
        }
    }
    impl ::core::convert::From<ConvertRoundTripBToACall> for Encoding_ConverterCalls {
        fn from(value: ConvertRoundTripBToACall) -> Self {
            Self::ConvertRoundTripBToA(value)
        }
    }
    impl ::core::convert::From<FailedRoundtripAToBCall> for Encoding_ConverterCalls {
        fn from(value: FailedRoundtripAToBCall) -> Self {
            Self::FailedRoundtripAToB(value)
        }
    }
    impl ::core::convert::From<FailedRoundtripBToACall> for Encoding_ConverterCalls {
        fn from(value: FailedRoundtripBToACall) -> Self {
            Self::FailedRoundtripBToA(value)
        }
    }
    ///Container type for all return fields from the `failedRoundtripAToB` function with signature `failedRoundtripAToB()` and selector `0x54c03f25`
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
    pub struct FailedRoundtripAToBReturn(pub bool);
    ///Container type for all return fields from the `failedRoundtripBToA` function with signature `failedRoundtripBToA()` and selector `0x5e355239`
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
    pub struct FailedRoundtripBToAReturn(pub bool);
}

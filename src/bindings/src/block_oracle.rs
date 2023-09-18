pub use block_oracle::*;
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
pub mod block_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkpoint"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("load"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("load"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_blockNumber"),
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
                                    name: ::std::borrow::ToOwned::to_owned("blockInfo_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BlockOracle.BlockInfo",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("Checkpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Checkpoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("childTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BlockHashNotPresent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BlockHashNotPresent",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BLOCKORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\x1C\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x99\xD5H\xAA\x14a\0;W\x80c\xC2\xC4\xC5\xC1\x14a\0xW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\xB8V[a\0\x8EV[`@\x80Q\x82Q\x81R` \x92\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\x80a\x01\rV[`@Q\x90\x81R` \x01a\0oV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01\x81\x90R\x83\x81R\x80\x82R\x82\x81 \x83Q\x80\x85\x01\x90\x94R\x80T\x80\x85R`\x01\x90\x91\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x84\x01\x92\x90\x92R\x03a\x01\x08W`@Q\x7F7\xCF'\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\0a\x01\x1A`\x01Ca\x01\xD1V[`@\x80Q\x80\x82\x01\x82R\x82@\x80\x82RBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16` \x80\x86\x01\x82\x81R`\0\x89\x81R\x91\x82\x90R\x87\x82 \x96Q\x87UQ`\x01\x90\x96\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x96\x90\x93\x16\x95\x90\x95\x17\x90\x91U\x93Q\x94\x95P\x90\x93\x90\x92\x91\x84\x91\x86\x91\x7F\xB6\x7F\xF5\x8B3\x06\x0F\xD3q\xA3Z\xE2\xD9\xF1\xC3\xCD\xAE\xC9\xB8\x19yi\xF6\xEF\xE2YJ\x1F\xF4\xBAh\xC6\x91\xA4PP\x90V[`\0` \x82\x84\x03\x12\x15a\x01\xCAW`\0\x80\xFD[P5\x91\x90PV[`\0\x82\x82\x10\x15a\x02\nW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The bytecode of the contract.
    pub static BLOCKORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x99\xD5H\xAA\x14a\0;W\x80c\xC2\xC4\xC5\xC1\x14a\0xW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\xB8V[a\0\x8EV[`@\x80Q\x82Q\x81R` \x92\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\x80a\x01\rV[`@Q\x90\x81R` \x01a\0oV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01\x81\x90R\x83\x81R\x80\x82R\x82\x81 \x83Q\x80\x85\x01\x90\x94R\x80T\x80\x85R`\x01\x90\x91\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x84\x01\x92\x90\x92R\x03a\x01\x08W`@Q\x7F7\xCF'\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[`\0a\x01\x1A`\x01Ca\x01\xD1V[`@\x80Q\x80\x82\x01\x82R\x82@\x80\x82RBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16` \x80\x86\x01\x82\x81R`\0\x89\x81R\x91\x82\x90R\x87\x82 \x96Q\x87UQ`\x01\x90\x96\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x96\x90\x93\x16\x95\x90\x95\x17\x90\x91U\x93Q\x94\x95P\x90\x93\x90\x92\x91\x84\x91\x86\x91\x7F\xB6\x7F\xF5\x8B3\x06\x0F\xD3q\xA3Z\xE2\xD9\xF1\xC3\xCD\xAE\xC9\xB8\x19yi\xF6\xEF\xE2YJ\x1F\xF4\xBAh\xC6\x91\xA4PP\x90V[`\0` \x82\x84\x03\x12\x15a\x01\xCAW`\0\x80\xFD[P5\x91\x90PV[`\0\x82\x82\x10\x15a\x02\nW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V\xFE\xA1dsolcC\0\x08\x0F\0\n";
    /// The deployed bytecode of the contract.
    pub static BLOCKORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BlockOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BlockOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BlockOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BlockOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BlockOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BlockOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BlockOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLOCKORACLE_ABI.clone(),
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
                BLOCKORACLE_ABI.clone(),
                BLOCKORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkpoint` (0xc2c4c5c1) function
        pub fn checkpoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 196, 197, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `load` (0x99d548aa) function
        pub fn load(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, BlockInfo> {
            self.0
                .method_hash([153, 213, 72, 170], block_number)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Checkpoint` event
        pub fn checkpoint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CheckpointFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CheckpointFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BlockOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BlockHashNotPresent` with signature `BlockHashNotPresent()` and selector `0x37cf2705`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BlockHashNotPresent", abi = "BlockHashNotPresent()")]
    pub struct BlockHashNotPresent;
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
    #[ethevent(name = "Checkpoint", abi = "Checkpoint(uint256,bytes32,uint64)")]
    pub struct CheckpointFilter {
        #[ethevent(indexed)]
        pub block_number: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub block_hash: [u8; 32],
        #[ethevent(indexed)]
        pub child_timestamp: u64,
    }
    ///Container type for all input parameters for the `checkpoint` function with signature `checkpoint()` and selector `0xc2c4c5c1`
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
    #[ethcall(name = "checkpoint", abi = "checkpoint()")]
    pub struct CheckpointCall;
    ///Container type for all input parameters for the `load` function with signature `load(uint256)` and selector `0x99d548aa`
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
    #[ethcall(name = "load", abi = "load(uint256)")]
    pub struct LoadCall {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BlockOracleCalls {
        Checkpoint(CheckpointCall),
        Load(LoadCall),
    }
    impl ::ethers::core::abi::AbiDecode for BlockOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckpointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Checkpoint(decoded));
            }
            if let Ok(decoded) = <LoadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Load(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BlockOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Checkpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Load(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BlockOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Checkpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Load(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckpointCall> for BlockOracleCalls {
        fn from(value: CheckpointCall) -> Self {
            Self::Checkpoint(value)
        }
    }
    impl ::core::convert::From<LoadCall> for BlockOracleCalls {
        fn from(value: LoadCall) -> Self {
            Self::Load(value)
        }
    }
    ///Container type for all return fields from the `checkpoint` function with signature `checkpoint()` and selector `0xc2c4c5c1`
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
    pub struct CheckpointReturn {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `load` function with signature `load(uint256)` and selector `0x99d548aa`
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
    pub struct LoadReturn {
        pub block_info: BlockInfo,
    }
    ///`BlockInfo(bytes32,uint64)`
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
    pub struct BlockInfo {
        pub hash: [u8; 32],
        pub child_timestamp: u64,
    }
}

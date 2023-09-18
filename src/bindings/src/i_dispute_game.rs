pub use i_dispute_game::*;
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
pub mod i_dispute_game {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("bondManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bondManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bondManager_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBondManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createdAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createdAt"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("createdAt_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Timestamp"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("extraData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("extraData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extraData_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gameData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gameType_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootClaim_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extraData_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gameType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameType"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gameType_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resolve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resolve"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum GameStatus"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rootClaim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rootClaim"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootClaim_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("status"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("status"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum GameStatus"),
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
                    ::std::borrow::ToOwned::to_owned("Resolved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Resolved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IDISPUTEGAME_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IDisputeGame<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDisputeGame<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDisputeGame<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDisputeGame<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDisputeGame<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDisputeGame))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDisputeGame<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDISPUTEGAME_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `bondManager` (0x363cc427) function
        pub fn bond_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([54, 60, 196, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createdAt` (0xcf09e0d0) function
        pub fn created_at(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([207, 9, 224, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `extraData` (0x609d3334) function
        pub fn extra_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([96, 157, 51, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameData` (0xfa24f743) function
        pub fn game_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, [u8; 32], ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([250, 36, 247, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameType` (0xbbdc02db) function
        pub fn game_type(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([187, 220, 2, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resolve` (0x2810e1d6) function
        pub fn resolve(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([40, 16, 225, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rootClaim` (0xbcef3b55) function
        pub fn root_claim(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([188, 239, 59, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `status` (0x200d2ed2) function
        pub fn status(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([32, 13, 46, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Resolved` event
        pub fn resolved_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ResolvedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ResolvedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDisputeGame<M> {
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
    #[ethevent(name = "Resolved", abi = "Resolved(uint8)")]
    pub struct ResolvedFilter {
        #[ethevent(indexed)]
        pub status: u8,
    }
    ///Container type for all input parameters for the `bondManager` function with signature `bondManager()` and selector `0x363cc427`
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
    #[ethcall(name = "bondManager", abi = "bondManager()")]
    pub struct BondManagerCall;
    ///Container type for all input parameters for the `createdAt` function with signature `createdAt()` and selector `0xcf09e0d0`
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
    #[ethcall(name = "createdAt", abi = "createdAt()")]
    pub struct CreatedAtCall;
    ///Container type for all input parameters for the `extraData` function with signature `extraData()` and selector `0x609d3334`
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
    #[ethcall(name = "extraData", abi = "extraData()")]
    pub struct ExtraDataCall;
    ///Container type for all input parameters for the `gameData` function with signature `gameData()` and selector `0xfa24f743`
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
    #[ethcall(name = "gameData", abi = "gameData()")]
    pub struct GameDataCall;
    ///Container type for all input parameters for the `gameType` function with signature `gameType()` and selector `0xbbdc02db`
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
    #[ethcall(name = "gameType", abi = "gameType()")]
    pub struct GameTypeCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
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
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `resolve` function with signature `resolve()` and selector `0x2810e1d6`
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
    #[ethcall(name = "resolve", abi = "resolve()")]
    pub struct ResolveCall;
    ///Container type for all input parameters for the `rootClaim` function with signature `rootClaim()` and selector `0xbcef3b55`
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
    #[ethcall(name = "rootClaim", abi = "rootClaim()")]
    pub struct RootClaimCall;
    ///Container type for all input parameters for the `status` function with signature `status()` and selector `0x200d2ed2`
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
    #[ethcall(name = "status", abi = "status()")]
    pub struct StatusCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IDisputeGameCalls {
        BondManager(BondManagerCall),
        CreatedAt(CreatedAtCall),
        ExtraData(ExtraDataCall),
        GameData(GameDataCall),
        GameType(GameTypeCall),
        Initialize(InitializeCall),
        Resolve(ResolveCall),
        RootClaim(RootClaimCall),
        Status(StatusCall),
    }
    impl ::ethers::core::abi::AbiDecode for IDisputeGameCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BondManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BondManager(decoded));
            }
            if let Ok(decoded) = <CreatedAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatedAt(decoded));
            }
            if let Ok(decoded) = <ExtraDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExtraData(decoded));
            }
            if let Ok(decoded) = <GameDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameData(decoded));
            }
            if let Ok(decoded) = <GameTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameType(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <ResolveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Resolve(decoded));
            }
            if let Ok(decoded) = <RootClaimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RootClaim(decoded));
            }
            if let Ok(decoded) = <StatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Status(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDisputeGameCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BondManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatedAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExtraData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GameData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GameType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Resolve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RootClaim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Status(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IDisputeGameCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BondManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatedAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtraData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameType(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resolve(element) => ::core::fmt::Display::fmt(element, f),
                Self::RootClaim(element) => ::core::fmt::Display::fmt(element, f),
                Self::Status(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BondManagerCall> for IDisputeGameCalls {
        fn from(value: BondManagerCall) -> Self {
            Self::BondManager(value)
        }
    }
    impl ::core::convert::From<CreatedAtCall> for IDisputeGameCalls {
        fn from(value: CreatedAtCall) -> Self {
            Self::CreatedAt(value)
        }
    }
    impl ::core::convert::From<ExtraDataCall> for IDisputeGameCalls {
        fn from(value: ExtraDataCall) -> Self {
            Self::ExtraData(value)
        }
    }
    impl ::core::convert::From<GameDataCall> for IDisputeGameCalls {
        fn from(value: GameDataCall) -> Self {
            Self::GameData(value)
        }
    }
    impl ::core::convert::From<GameTypeCall> for IDisputeGameCalls {
        fn from(value: GameTypeCall) -> Self {
            Self::GameType(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IDisputeGameCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ResolveCall> for IDisputeGameCalls {
        fn from(value: ResolveCall) -> Self {
            Self::Resolve(value)
        }
    }
    impl ::core::convert::From<RootClaimCall> for IDisputeGameCalls {
        fn from(value: RootClaimCall) -> Self {
            Self::RootClaim(value)
        }
    }
    impl ::core::convert::From<StatusCall> for IDisputeGameCalls {
        fn from(value: StatusCall) -> Self {
            Self::Status(value)
        }
    }
    ///Container type for all return fields from the `bondManager` function with signature `bondManager()` and selector `0x363cc427`
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
    pub struct BondManagerReturn {
        pub bond_manager: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createdAt` function with signature `createdAt()` and selector `0xcf09e0d0`
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
    pub struct CreatedAtReturn {
        pub created_at: u64,
    }
    ///Container type for all return fields from the `extraData` function with signature `extraData()` and selector `0x609d3334`
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
    pub struct ExtraDataReturn {
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `gameData` function with signature `gameData()` and selector `0xfa24f743`
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
    pub struct GameDataReturn {
        pub game_type: u8,
        pub root_claim: [u8; 32],
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `gameType` function with signature `gameType()` and selector `0xbbdc02db`
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
    pub struct GameTypeReturn {
        pub game_type: u8,
    }
    ///Container type for all return fields from the `resolve` function with signature `resolve()` and selector `0x2810e1d6`
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
    pub struct ResolveReturn {
        pub status: u8,
    }
    ///Container type for all return fields from the `rootClaim` function with signature `rootClaim()` and selector `0xbcef3b55`
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
    pub struct RootClaimReturn {
        pub root_claim: [u8; 32],
    }
    ///Container type for all return fields from the `status` function with signature `status()` and selector `0x200d2ed2`
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
    pub struct StatusReturn {
        pub status: u8,
    }
}

pub use i_dispute_game_factory::*;
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
pub mod i_dispute_game_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("create"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("create"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rootClaim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gameAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameAtIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_index"),
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
                                    name: ::std::borrow::ToOwned::to_owned("gameType_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Timestamp"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gameCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameCount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gameCount_"),
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
                    ::std::borrow::ToOwned::to_owned("gameImpls"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gameImpls"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("impl_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("games"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("games"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rootClaim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp_"),
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
                    ::std::borrow::ToOwned::to_owned("getGameUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGameUUID"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rootClaim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_extraData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uuid_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Hash"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setImplementation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("GameType"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_impl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDisputeGame"),
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
                    ::std::borrow::ToOwned::to_owned("DisputeGameCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DisputeGameCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("disputeProxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gameType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rootClaim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ImplementationSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ImplementationSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("impl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gameType"),
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
    pub static IDISPUTEGAMEFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IDisputeGameFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDisputeGameFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDisputeGameFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDisputeGameFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDisputeGameFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDisputeGameFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDisputeGameFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDISPUTEGAMEFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `create` (0x3142e55e) function
        pub fn create(
            &self,
            game_type: u8,
            root_claim: [u8; 32],
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([49, 66, 229, 94], (game_type, root_claim, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameAtIndex` (0xbb8aa1fc) function
        pub fn game_at_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, u64, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([187, 138, 161, 252], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameCount` (0x4d1975b4) function
        pub fn game_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 25, 117, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gameImpls` (0xdfa162d3) function
        pub fn game_impls(
            &self,
            game_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 161, 98, 211], game_type)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `games` (0xc49d5271) function
        pub fn games(
            &self,
            game_type: u8,
            root_claim: [u8; 32],
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u64),
        > {
            self.0
                .method_hash([196, 157, 82, 113], (game_type, root_claim, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGameUUID` (0x26daafbe) function
        pub fn get_game_uuid(
            &self,
            game_type: u8,
            root_claim: [u8; 32],
            extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([38, 218, 175, 190], (game_type, root_claim, extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setImplementation` (0x45583b7a) function
        pub fn set_implementation(
            &self,
            game_type: u8,
            impl_: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 88, 59, 122], (game_type, impl_))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DisputeGameCreated` event
        pub fn dispute_game_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisputeGameCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ImplementationSet` event
        pub fn implementation_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ImplementationSetFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IDisputeGameFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDisputeGameFactory<M> {
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
    #[ethevent(
        name = "DisputeGameCreated",
        abi = "DisputeGameCreated(address,uint8,bytes32)"
    )]
    pub struct DisputeGameCreatedFilter {
        #[ethevent(indexed)]
        pub dispute_proxy: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub game_type: u8,
        #[ethevent(indexed)]
        pub root_claim: [u8; 32],
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
    #[ethevent(name = "ImplementationSet", abi = "ImplementationSet(address,uint8)")]
    pub struct ImplementationSetFilter {
        #[ethevent(indexed)]
        pub impl_: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub game_type: u8,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IDisputeGameFactoryEvents {
        DisputeGameCreatedFilter(DisputeGameCreatedFilter),
        ImplementationSetFilter(ImplementationSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for IDisputeGameFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DisputeGameCreatedFilter::decode_log(log) {
                return Ok(IDisputeGameFactoryEvents::DisputeGameCreatedFilter(decoded));
            }
            if let Ok(decoded) = ImplementationSetFilter::decode_log(log) {
                return Ok(IDisputeGameFactoryEvents::ImplementationSetFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IDisputeGameFactoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DisputeGameCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ImplementationSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DisputeGameCreatedFilter> for IDisputeGameFactoryEvents {
        fn from(value: DisputeGameCreatedFilter) -> Self {
            Self::DisputeGameCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ImplementationSetFilter> for IDisputeGameFactoryEvents {
        fn from(value: ImplementationSetFilter) -> Self {
            Self::ImplementationSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `create` function with signature `create(uint8,bytes32,bytes)` and selector `0x3142e55e`
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
    #[ethcall(name = "create", abi = "create(uint8,bytes32,bytes)")]
    pub struct CreateCall {
        pub game_type: u8,
        pub root_claim: [u8; 32],
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `gameAtIndex` function with signature `gameAtIndex(uint256)` and selector `0xbb8aa1fc`
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
    #[ethcall(name = "gameAtIndex", abi = "gameAtIndex(uint256)")]
    pub struct GameAtIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `gameCount` function with signature `gameCount()` and selector `0x4d1975b4`
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
    #[ethcall(name = "gameCount", abi = "gameCount()")]
    pub struct GameCountCall;
    ///Container type for all input parameters for the `gameImpls` function with signature `gameImpls(uint8)` and selector `0xdfa162d3`
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
    #[ethcall(name = "gameImpls", abi = "gameImpls(uint8)")]
    pub struct GameImplsCall {
        pub game_type: u8,
    }
    ///Container type for all input parameters for the `games` function with signature `games(uint8,bytes32,bytes)` and selector `0xc49d5271`
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
    #[ethcall(name = "games", abi = "games(uint8,bytes32,bytes)")]
    pub struct GamesCall {
        pub game_type: u8,
        pub root_claim: [u8; 32],
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getGameUUID` function with signature `getGameUUID(uint8,bytes32,bytes)` and selector `0x26daafbe`
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
    #[ethcall(name = "getGameUUID", abi = "getGameUUID(uint8,bytes32,bytes)")]
    pub struct GetGameUUIDCall {
        pub game_type: u8,
        pub root_claim: [u8; 32],
        pub extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setImplementation` function with signature `setImplementation(uint8,address)` and selector `0x45583b7a`
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
    #[ethcall(name = "setImplementation", abi = "setImplementation(uint8,address)")]
    pub struct SetImplementationCall {
        pub game_type: u8,
        pub impl_: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IDisputeGameFactoryCalls {
        Create(CreateCall),
        GameAtIndex(GameAtIndexCall),
        GameCount(GameCountCall),
        GameImpls(GameImplsCall),
        Games(GamesCall),
        GetGameUUID(GetGameUUIDCall),
        SetImplementation(SetImplementationCall),
    }
    impl ::ethers::core::abi::AbiDecode for IDisputeGameFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CreateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Create(decoded));
            }
            if let Ok(decoded) = <GameAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameAtIndex(decoded));
            }
            if let Ok(decoded) = <GameCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameCount(decoded));
            }
            if let Ok(decoded) = <GameImplsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GameImpls(decoded));
            }
            if let Ok(decoded) = <GamesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Games(decoded));
            }
            if let Ok(decoded) = <GetGameUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGameUUID(decoded));
            }
            if let Ok(decoded) = <SetImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetImplementation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDisputeGameFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Create(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GameAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GameCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GameImpls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Games(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetGameUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IDisputeGameFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Create(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameImpls(element) => ::core::fmt::Display::fmt(element, f),
                Self::Games(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGameUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetImplementation(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateCall> for IDisputeGameFactoryCalls {
        fn from(value: CreateCall) -> Self {
            Self::Create(value)
        }
    }
    impl ::core::convert::From<GameAtIndexCall> for IDisputeGameFactoryCalls {
        fn from(value: GameAtIndexCall) -> Self {
            Self::GameAtIndex(value)
        }
    }
    impl ::core::convert::From<GameCountCall> for IDisputeGameFactoryCalls {
        fn from(value: GameCountCall) -> Self {
            Self::GameCount(value)
        }
    }
    impl ::core::convert::From<GameImplsCall> for IDisputeGameFactoryCalls {
        fn from(value: GameImplsCall) -> Self {
            Self::GameImpls(value)
        }
    }
    impl ::core::convert::From<GamesCall> for IDisputeGameFactoryCalls {
        fn from(value: GamesCall) -> Self {
            Self::Games(value)
        }
    }
    impl ::core::convert::From<GetGameUUIDCall> for IDisputeGameFactoryCalls {
        fn from(value: GetGameUUIDCall) -> Self {
            Self::GetGameUUID(value)
        }
    }
    impl ::core::convert::From<SetImplementationCall> for IDisputeGameFactoryCalls {
        fn from(value: SetImplementationCall) -> Self {
            Self::SetImplementation(value)
        }
    }
    ///Container type for all return fields from the `create` function with signature `create(uint8,bytes32,bytes)` and selector `0x3142e55e`
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
    pub struct CreateReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `gameAtIndex` function with signature `gameAtIndex(uint256)` and selector `0xbb8aa1fc`
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
    pub struct GameAtIndexReturn {
        pub game_type: u8,
        pub timestamp: u64,
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `gameCount` function with signature `gameCount()` and selector `0x4d1975b4`
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
    pub struct GameCountReturn {
        pub game_count: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `gameImpls` function with signature `gameImpls(uint8)` and selector `0xdfa162d3`
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
    pub struct GameImplsReturn {
        pub impl_: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `games` function with signature `games(uint8,bytes32,bytes)` and selector `0xc49d5271`
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
    pub struct GamesReturn {
        pub proxy: ::ethers::core::types::Address,
        pub timestamp: u64,
    }
    ///Container type for all return fields from the `getGameUUID` function with signature `getGameUUID(uint8,bytes32,bytes)` and selector `0x26daafbe`
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
    pub struct GetGameUUIDReturn {
        pub uuid: [u8; 32],
    }
}

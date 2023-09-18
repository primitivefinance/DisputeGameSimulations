pub use i_fault_dispute_game::*;
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
pub mod i_fault_dispute_game {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addLocalData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLocalData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ident"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_partOffset"),
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
                    ::std::borrow::ToOwned::to_owned("attack"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attack"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_parentIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_claim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("defend"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defend"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_parentIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_claim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Claim"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("l1BlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1BlockNumber"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("l1BlockNumber_"),
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
                    ::std::borrow::ToOwned::to_owned("l1Head"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l1Head"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("l1Head_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Hash"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("l2BlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("l2BlockNumber"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("l2BlockNumber_"),
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
                (
                    ::std::borrow::ToOwned::to_owned("step"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("step"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_claimIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_isAttack"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stateData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("Move"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Move"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("parentIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("claim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("claimant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
    pub static IFAULTDISPUTEGAME_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IFaultDisputeGame<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IFaultDisputeGame<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IFaultDisputeGame<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IFaultDisputeGame<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IFaultDisputeGame<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IFaultDisputeGame))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IFaultDisputeGame<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IFAULTDISPUTEGAME_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addLocalData` (0x1e27052a) function
        pub fn add_local_data(
            &self,
            ident: ::ethers::core::types::U256,
            part_offset: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 39, 5, 42], (ident, part_offset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `attack` (0xc55cd0c7) function
        pub fn attack(
            &self,
            parent_index: ::ethers::core::types::U256,
            claim: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 92, 208, 199], (parent_index, claim))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `defend` (0x35fef567) function
        pub fn defend(
            &self,
            parent_index: ::ethers::core::types::U256,
            claim: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 254, 245, 103], (parent_index, claim))
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
        ///Calls the contract's `l1BlockNumber` (0x298c9005) function
        pub fn l_1_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([41, 140, 144, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l1Head` (0x6361506d) function
        pub fn l_1_head(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([99, 97, 80, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2BlockNumber` (0x8b85902b) function
        pub fn l_2_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 133, 144, 43], ())
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
        ///Calls the contract's `step` (0xd8cc1a3c) function
        pub fn step(
            &self,
            claim_index: ::ethers::core::types::U256,
            is_attack: bool,
            state_data: ::ethers::core::types::Bytes,
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [216, 204, 26, 60],
                    (claim_index, is_attack, state_data, proof),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Move` event
        pub fn move_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MoveFilter> {
            self.0.event()
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
            IFaultDisputeGameEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IFaultDisputeGame<M> {
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
    #[ethevent(name = "Move", abi = "Move(uint256,bytes32,address)")]
    pub struct MoveFilter {
        #[ethevent(indexed)]
        pub parent_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub claim: [u8; 32],
        #[ethevent(indexed)]
        pub claimant: ::ethers::core::types::Address,
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IFaultDisputeGameEvents {
        MoveFilter(MoveFilter),
        ResolvedFilter(ResolvedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IFaultDisputeGameEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MoveFilter::decode_log(log) {
                return Ok(IFaultDisputeGameEvents::MoveFilter(decoded));
            }
            if let Ok(decoded) = ResolvedFilter::decode_log(log) {
                return Ok(IFaultDisputeGameEvents::ResolvedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IFaultDisputeGameEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MoveFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResolvedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MoveFilter> for IFaultDisputeGameEvents {
        fn from(value: MoveFilter) -> Self {
            Self::MoveFilter(value)
        }
    }
    impl ::core::convert::From<ResolvedFilter> for IFaultDisputeGameEvents {
        fn from(value: ResolvedFilter) -> Self {
            Self::ResolvedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addLocalData` function with signature `addLocalData(uint256,uint256)` and selector `0x1e27052a`
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
    #[ethcall(name = "addLocalData", abi = "addLocalData(uint256,uint256)")]
    pub struct AddLocalDataCall {
        pub ident: ::ethers::core::types::U256,
        pub part_offset: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `attack` function with signature `attack(uint256,bytes32)` and selector `0xc55cd0c7`
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
    #[ethcall(name = "attack", abi = "attack(uint256,bytes32)")]
    pub struct AttackCall {
        pub parent_index: ::ethers::core::types::U256,
        pub claim: [u8; 32],
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
    ///Container type for all input parameters for the `defend` function with signature `defend(uint256,bytes32)` and selector `0x35fef567`
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
    #[ethcall(name = "defend", abi = "defend(uint256,bytes32)")]
    pub struct DefendCall {
        pub parent_index: ::ethers::core::types::U256,
        pub claim: [u8; 32],
    }
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
    ///Container type for all input parameters for the `l1BlockNumber` function with signature `l1BlockNumber()` and selector `0x298c9005`
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
    #[ethcall(name = "l1BlockNumber", abi = "l1BlockNumber()")]
    pub struct L1BlockNumberCall;
    ///Container type for all input parameters for the `l1Head` function with signature `l1Head()` and selector `0x6361506d`
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
    #[ethcall(name = "l1Head", abi = "l1Head()")]
    pub struct L1HeadCall;
    ///Container type for all input parameters for the `l2BlockNumber` function with signature `l2BlockNumber()` and selector `0x8b85902b`
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
    #[ethcall(name = "l2BlockNumber", abi = "l2BlockNumber()")]
    pub struct L2BlockNumberCall;
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
    ///Container type for all input parameters for the `step` function with signature `step(uint256,bool,bytes,bytes)` and selector `0xd8cc1a3c`
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
    #[ethcall(name = "step", abi = "step(uint256,bool,bytes,bytes)")]
    pub struct StepCall {
        pub claim_index: ::ethers::core::types::U256,
        pub is_attack: bool,
        pub state_data: ::ethers::core::types::Bytes,
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IFaultDisputeGameCalls {
        AddLocalData(AddLocalDataCall),
        Attack(AttackCall),
        BondManager(BondManagerCall),
        CreatedAt(CreatedAtCall),
        Defend(DefendCall),
        ExtraData(ExtraDataCall),
        GameData(GameDataCall),
        GameType(GameTypeCall),
        Initialize(InitializeCall),
        L1BlockNumber(L1BlockNumberCall),
        L1Head(L1HeadCall),
        L2BlockNumber(L2BlockNumberCall),
        Resolve(ResolveCall),
        RootClaim(RootClaimCall),
        Status(StatusCall),
        Step(StepCall),
    }
    impl ::ethers::core::abi::AbiDecode for IFaultDisputeGameCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddLocalDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLocalData(decoded));
            }
            if let Ok(decoded) = <AttackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Attack(decoded));
            }
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
            if let Ok(decoded) = <DefendCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Defend(decoded));
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
            if let Ok(decoded) = <L1BlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1BlockNumber(decoded));
            }
            if let Ok(decoded) = <L1HeadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L1Head(decoded));
            }
            if let Ok(decoded) = <L2BlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::L2BlockNumber(decoded));
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
            if let Ok(decoded) = <StepCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Step(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IFaultDisputeGameCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddLocalData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Attack(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BondManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatedAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Defend(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::L1BlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L1Head(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::L2BlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Resolve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RootClaim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Status(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Step(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IFaultDisputeGameCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLocalData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Attack(element) => ::core::fmt::Display::fmt(element, f),
                Self::BondManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatedAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::Defend(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtraData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GameType(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1BlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::L1Head(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2BlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resolve(element) => ::core::fmt::Display::fmt(element, f),
                Self::RootClaim(element) => ::core::fmt::Display::fmt(element, f),
                Self::Status(element) => ::core::fmt::Display::fmt(element, f),
                Self::Step(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddLocalDataCall> for IFaultDisputeGameCalls {
        fn from(value: AddLocalDataCall) -> Self {
            Self::AddLocalData(value)
        }
    }
    impl ::core::convert::From<AttackCall> for IFaultDisputeGameCalls {
        fn from(value: AttackCall) -> Self {
            Self::Attack(value)
        }
    }
    impl ::core::convert::From<BondManagerCall> for IFaultDisputeGameCalls {
        fn from(value: BondManagerCall) -> Self {
            Self::BondManager(value)
        }
    }
    impl ::core::convert::From<CreatedAtCall> for IFaultDisputeGameCalls {
        fn from(value: CreatedAtCall) -> Self {
            Self::CreatedAt(value)
        }
    }
    impl ::core::convert::From<DefendCall> for IFaultDisputeGameCalls {
        fn from(value: DefendCall) -> Self {
            Self::Defend(value)
        }
    }
    impl ::core::convert::From<ExtraDataCall> for IFaultDisputeGameCalls {
        fn from(value: ExtraDataCall) -> Self {
            Self::ExtraData(value)
        }
    }
    impl ::core::convert::From<GameDataCall> for IFaultDisputeGameCalls {
        fn from(value: GameDataCall) -> Self {
            Self::GameData(value)
        }
    }
    impl ::core::convert::From<GameTypeCall> for IFaultDisputeGameCalls {
        fn from(value: GameTypeCall) -> Self {
            Self::GameType(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IFaultDisputeGameCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<L1BlockNumberCall> for IFaultDisputeGameCalls {
        fn from(value: L1BlockNumberCall) -> Self {
            Self::L1BlockNumber(value)
        }
    }
    impl ::core::convert::From<L1HeadCall> for IFaultDisputeGameCalls {
        fn from(value: L1HeadCall) -> Self {
            Self::L1Head(value)
        }
    }
    impl ::core::convert::From<L2BlockNumberCall> for IFaultDisputeGameCalls {
        fn from(value: L2BlockNumberCall) -> Self {
            Self::L2BlockNumber(value)
        }
    }
    impl ::core::convert::From<ResolveCall> for IFaultDisputeGameCalls {
        fn from(value: ResolveCall) -> Self {
            Self::Resolve(value)
        }
    }
    impl ::core::convert::From<RootClaimCall> for IFaultDisputeGameCalls {
        fn from(value: RootClaimCall) -> Self {
            Self::RootClaim(value)
        }
    }
    impl ::core::convert::From<StatusCall> for IFaultDisputeGameCalls {
        fn from(value: StatusCall) -> Self {
            Self::Status(value)
        }
    }
    impl ::core::convert::From<StepCall> for IFaultDisputeGameCalls {
        fn from(value: StepCall) -> Self {
            Self::Step(value)
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
    ///Container type for all return fields from the `l1BlockNumber` function with signature `l1BlockNumber()` and selector `0x298c9005`
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
    pub struct L1BlockNumberReturn {
        pub l_1_block_number: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `l1Head` function with signature `l1Head()` and selector `0x6361506d`
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
    pub struct L1HeadReturn {
        pub l_1_head: [u8; 32],
    }
    ///Container type for all return fields from the `l2BlockNumber` function with signature `l2BlockNumber()` and selector `0x8b85902b`
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
    pub struct L2BlockNumberReturn {
        pub l_2_block_number: ::ethers::core::types::U256,
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

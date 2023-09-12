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
pub mod deployer {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
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
                    ::std::borrow::ToOwned::to_owned("get"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Deployment"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("has"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("has"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mustGetAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mustGetAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("newDeployments"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("newDeployments"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Deployment[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("save"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("save"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_deployed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sync"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sync"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DeploymentDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeploymentDoesNotExist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDeployment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidDeployment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEPLOYER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Deployer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Deployer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Deployer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Deployer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Deployer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Deployer)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Deployer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPLOYER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get` (0x693ec85e) function
        pub fn get(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, Deployment> {
            self.0
                .method_hash([105, 62, 200, 94], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddress` (0xbf40fac1) function
        pub fn get_address(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([191, 64, 250, 193], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `has` (0xa8e52294) function
        pub fn has(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([168, 229, 34, 148], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mustGetAddress` (0x3fb90271) function
        pub fn must_get_address(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 185, 2, 113], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `newDeployments` (0xe8727781) function
        pub fn new_deployments(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Deployment>> {
            self.0
                .method_hash([232, 114, 119, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `save` (0x4b64367e) function
        pub fn save(
            &self,
            name: ::std::string::String,
            deployed: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 100, 54, 126], (name, deployed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUp` (0x0a9254e4) function
        pub fn set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sync` (0xfff6cae9) function
        pub fn sync(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 246, 202, 233], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Deployer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DeploymentDoesNotExist` with signature `DeploymentDoesNotExist(string)` and selector `0xb2f5b290`
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
    #[etherror(name = "DeploymentDoesNotExist", abi = "DeploymentDoesNotExist(string)")]
    pub struct DeploymentDoesNotExist(pub ::std::string::String);
    ///Custom Error type `InvalidDeployment` with signature `InvalidDeployment(string)` and selector `0xdfc8467d`
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
    #[etherror(name = "InvalidDeployment", abi = "InvalidDeployment(string)")]
    pub struct InvalidDeployment(pub ::std::string::String);
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DeployerErrors {
        DeploymentDoesNotExist(DeploymentDoesNotExist),
        InvalidDeployment(InvalidDeployment),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DeployerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DeploymentDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeploymentDoesNotExist(decoded));
            }
            if let Ok(decoded) = <InvalidDeployment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDeployment(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeploymentDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDeployment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DeployerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DeploymentDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDeployment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DeployerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeploymentDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDeployment(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DeployerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeploymentDoesNotExist> for DeployerErrors {
        fn from(value: DeploymentDoesNotExist) -> Self {
            Self::DeploymentDoesNotExist(value)
        }
    }
    impl ::core::convert::From<InvalidDeployment> for DeployerErrors {
        fn from(value: InvalidDeployment) -> Self {
            Self::InvalidDeployment(value)
        }
    }
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `get` function with signature `get(string)` and selector `0x693ec85e`
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
    #[ethcall(name = "get", abi = "get(string)")]
    pub struct GetCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `getAddress` function with signature `getAddress(string)` and selector `0xbf40fac1`
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
    #[ethcall(name = "getAddress", abi = "getAddress(string)")]
    pub struct GetAddressCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `has` function with signature `has(string)` and selector `0xa8e52294`
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
    #[ethcall(name = "has", abi = "has(string)")]
    pub struct HasCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `mustGetAddress` function with signature `mustGetAddress(string)` and selector `0x3fb90271`
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
    #[ethcall(name = "mustGetAddress", abi = "mustGetAddress(string)")]
    pub struct MustGetAddressCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `newDeployments` function with signature `newDeployments()` and selector `0xe8727781`
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
    #[ethcall(name = "newDeployments", abi = "newDeployments()")]
    pub struct NewDeploymentsCall;
    ///Container type for all input parameters for the `save` function with signature `save(string,address)` and selector `0x4b64367e`
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
    #[ethcall(name = "save", abi = "save(string,address)")]
    pub struct SaveCall {
        pub name: ::std::string::String,
        pub deployed: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `0x0a9254e4`
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
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    ///Container type for all input parameters for the `sync` function with signature `sync()` and selector `0xfff6cae9`
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
    #[ethcall(name = "sync", abi = "sync()")]
    pub struct SyncCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DeployerCalls {
        IsScript(IsScriptCall),
        Get(GetCall),
        GetAddress(GetAddressCall),
        Has(HasCall),
        MustGetAddress(MustGetAddressCall),
        Name(NameCall),
        NewDeployments(NewDeploymentsCall),
        Save(SaveCall),
        SetUp(SetUpCall),
        Sync(SyncCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <GetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddress(decoded));
            }
            if let Ok(decoded) = <HasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Has(decoded));
            }
            if let Ok(decoded) = <MustGetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MustGetAddress(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NewDeploymentsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NewDeployments(decoded));
            }
            if let Ok(decoded) = <SaveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Save(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUp(decoded));
            }
            if let Ok(decoded) = <SyncCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sync(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Has(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MustGetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NewDeployments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Save(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sync(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DeployerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Has(element) => ::core::fmt::Display::fmt(element, f),
                Self::MustGetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewDeployments(element) => ::core::fmt::Display::fmt(element, f),
                Self::Save(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sync(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for DeployerCalls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<GetCall> for DeployerCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<GetAddressCall> for DeployerCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    impl ::core::convert::From<HasCall> for DeployerCalls {
        fn from(value: HasCall) -> Self {
            Self::Has(value)
        }
    }
    impl ::core::convert::From<MustGetAddressCall> for DeployerCalls {
        fn from(value: MustGetAddressCall) -> Self {
            Self::MustGetAddress(value)
        }
    }
    impl ::core::convert::From<NameCall> for DeployerCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NewDeploymentsCall> for DeployerCalls {
        fn from(value: NewDeploymentsCall) -> Self {
            Self::NewDeployments(value)
        }
    }
    impl ::core::convert::From<SaveCall> for DeployerCalls {
        fn from(value: SaveCall) -> Self {
            Self::Save(value)
        }
    }
    impl ::core::convert::From<SetUpCall> for DeployerCalls {
        fn from(value: SetUpCall) -> Self {
            Self::SetUp(value)
        }
    }
    impl ::core::convert::From<SyncCall> for DeployerCalls {
        fn from(value: SyncCall) -> Self {
            Self::Sync(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    pub struct IsScriptReturn(pub bool);
    ///Container type for all return fields from the `get` function with signature `get(string)` and selector `0x693ec85e`
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
    pub struct GetReturn(pub Deployment);
    ///Container type for all return fields from the `getAddress` function with signature `getAddress(string)` and selector `0xbf40fac1`
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
    pub struct GetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `has` function with signature `has(string)` and selector `0xa8e52294`
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
    pub struct HasReturn(pub bool);
    ///Container type for all return fields from the `mustGetAddress` function with signature `mustGetAddress(string)` and selector `0x3fb90271`
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
    pub struct MustGetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `newDeployments` function with signature `newDeployments()` and selector `0xe8727781`
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
    pub struct NewDeploymentsReturn(pub ::std::vec::Vec<Deployment>);
}

///`Attestation(bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Attestation {
    pub uid: [u8; 32],
    pub schema: [u8; 32],
    pub time: u64,
    pub expiration_time: u64,
    pub revocation_time: u64,
    pub ref_uid: [u8; 32],
    pub recipient: ::ethers::core::types::Address,
    pub attester: ::ethers::core::types::Address,
    pub revocable: bool,
    pub data: ::ethers::core::types::Bytes,
}
///`AttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct AttestationRequest {
    pub schema: [u8; 32],
    pub data: AttestationRequestData,
}
///`AttestationRequestData(address,uint64,bool,bytes32,bytes,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct AttestationRequestData {
    pub recipient: ::ethers::core::types::Address,
    pub expiration_time: u64,
    pub revocable: bool,
    pub ref_uid: [u8; 32],
    pub data: ::ethers::core::types::Bytes,
    pub value: ::ethers::core::types::U256,
}
///`DelegatedAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DelegatedAttestationRequest {
    pub schema: [u8; 32],
    pub data: AttestationRequestData,
    pub signature: Signature,
    pub attester: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`DelegatedRevocationRequest(bytes32,(bytes32,uint256),(uint8,bytes32,bytes32),address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DelegatedRevocationRequest {
    pub schema: [u8; 32],
    pub data: RevocationRequestData,
    pub signature: Signature,
    pub revoker: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`Deployment(string,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Deployment {
    pub name: ::std::string::String,
    pub addr: ::ethers::core::types::Address,
}
///`Checkpoint(uint32,uint224)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Checkpoint {
    pub from_block: u32,
    pub votes: ::ethers::core::types::U256,
}
///`DripParameters(address,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DripParameters {
    pub recipient: ::ethers::core::types::Address,
    pub nonce: [u8; 32],
}
///`MultiAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256)[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MultiAttestationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<AttestationRequestData>,
}
///`MultiDelegatedAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MultiDelegatedAttestationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<AttestationRequestData>,
    pub signatures: ::std::vec::Vec<Signature>,
    pub attester: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`MultiDelegatedRevocationRequest(bytes32,(bytes32,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MultiDelegatedRevocationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<RevocationRequestData>,
    pub signatures: ::std::vec::Vec<Signature>,
    pub revoker: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`MultiRevocationRequest(bytes32,(bytes32,uint256)[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MultiRevocationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<RevocationRequestData>,
}
///`ClaimableInvite(address,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ClaimableInvite {
    pub issuer: ::ethers::core::types::Address,
    pub nonce: [u8; 32],
}
///`ResourceConfig(uint32,uint8,uint8,uint32,uint32,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ResourceConfig {
    pub max_resource_limit: u32,
    pub elasticity_multiplier: u8,
    pub base_fee_max_change_denominator: u8,
    pub minimum_base_fee: u32,
    pub system_tx_max_gas: u32,
    pub maximum_base_fee: u128,
}
///`RevocationRequest(bytes32,(bytes32,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RevocationRequest {
    pub schema: [u8; 32],
    pub data: RevocationRequestData,
}
///`RevocationRequestData(bytes32,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct RevocationRequestData {
    pub uid: [u8; 32],
    pub value: ::ethers::core::types::U256,
}
///`SchemaRecord(bytes32,address,bool,string)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SchemaRecord {
    pub uid: [u8; 32],
    pub resolver: ::ethers::core::types::Address,
    pub revocable: bool,
    pub schema: ::std::string::String,
}
///`Signature(uint8,bytes32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Signature {
    pub v: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
///`WithdrawalTransaction(uint256,address,address,uint256,uint256,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct WithdrawalTransaction {
    pub nonce: ::ethers::core::types::U256,
    pub sender: ::ethers::core::types::Address,
    pub target: ::ethers::core::types::Address,
    pub value: ::ethers::core::types::U256,
    pub gas_limit: ::ethers::core::types::U256,
    pub data: ::ethers::core::types::Bytes,
}

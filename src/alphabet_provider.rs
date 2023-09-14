use crate::bindings::types::types::Types;
use std::error::Error;
use ethabi::Hash;
use num_bigint::BigUint;
use tokio::task;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Index is larger than the maximum index")]
    ErrIndexTooLarge,
    // Add other error types here
}

pub struct AlphabetTraceProvider {
    state: Vec<String>,
    max_len: u64,
}

impl AlphabetTraceProvider {
    pub fn new(state: String, depth: u64) -> Self {
        Self {
            state: state.chars().map(|c| c.to_string()).collect(),
            max_len: 1 << depth,
        }
    }

    pub async fn get_step_data(&self, i: u64) -> Box<Result<(Vec<u8>, Vec<u8>, Option<Vec<u8>>), dyn Error>> {
        if i == 0 {
            let prestate = self.absolute_pre_state().await?;
            return Ok((prestate, vec![], None));
        }
        let i = i - 1;
        if i >= self.max_len {
            return Err(Box::new(MyError::ErrIndexTooLarge));
        }
        if i >= self.state.len() as u64 {
            return self.get_step_data(i as u64).await;
        }
        Ok((build_alphabet_preimage(i, &self.state[i as usize]), vec![], None))
    }

    pub async fn get(&self, i: u64) -> Result<Hash, Box<dyn Error>> {
        let (claim_bytes, _, _,) = self.get_step_data(i + 1).await?;
        let hash = alphabet_state_hash(&claim_bytes);
        Ok(hash)
    }

    pub async fn absolute_pre_state(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Convert hex to bytes
        let pre_state = hex::decode("0000000000000000000000000000000000000000000000000000000000000060")?;
        Ok(pre_state)
    }
}

pub fn build_alphabet_preimage(i: u64, letter: &str) -> Vec<u8> {
    let mut result = index_to_bytes(i);
    result.extend(letter_to_bytes(letter));
    result
}

pub fn alphabet_state_hash(state: &[u8]) -> Hash {
    let mut keccak = Keccak::v256();
    keccak.update(state);
    let mut res: [u8; 32] = [0; 32];
    keccak.finalize(&mut res);
    Hash::from_slice(&res)
}

pub fn index_to_bytes(i: u64) -> Vec<u8> {
    let big = BigUint::from(i);
    let mut out = vec![0u8; 32];
    big.to_big_endian(&mut out);
    out
}

pub fn letter_to_bytes(letter: &str) -> Vec<u8> {
    let mut out = vec![0u8; 32];
    out[31] = letter.as_bytes()[0];
    out
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let alphabet_trace_provider = AlphabetTraceProvider::new("abc".to_string(), 4);

    // Use the methods here
    let (step_data, _, _) = alphabet_trace_provider.get_step_data(2).await?;
    println!("{:?}", step_data);

    Ok(())
}



use std::sync::Arc;

use alloy_primitives::Address;
use durin_fault::{FaultDisputeSolver};


use arbiter_core::middleware::RevmMiddleware;
use durin_primitives::Claim;
use foundry_contracts::{fault_dispute_game::FaultDisputeGame, alphabet_vm::AlphabetVM, block_oracle::BlockOracle, l2_output_oracle::L2OutputOracle};

pub struct HonestPlayer {
    // pub fault: FaultDisputeSolver<RevmMiddleware, RevmMiddleware, Claim>,
    pub address: Address,
    pub client: Arc<RevmMiddleware>,
    pub vm: AlphabetVM<RevmMiddleware>,
    pub oracle: L2OutputOracle<RevmMiddleware>,
    pub block_oracle: BlockOracle<RevmMiddleware>,
    pub disputegame: FaultDisputeGame<RevmMiddleware>,
}

impl HonestPlayer {
    pub fn new() -> Self {
        todo!()
    }

    pub fn start_game_with_honest_dispute() {
        todo!()
    }

    pub fn generate_honest_dispute() {
        // probably needs to use cannon and or durin here
        todo!()
    }
}

pub struct DishonestPlayer{
    pub name: String,
    pub address: String,
    pub client: RevmMiddleware,
    pub vm: AlphabetVM<RevmMiddleware>,
    pub oracle: L2OutputOracle<RevmMiddleware>,
    pub block_oracle: BlockOracle<RevmMiddleware>,
    pub disputegame: FaultDisputeGame<RevmMiddleware>,
}

impl DishonestPlayer {
    pub fn new() -> Self {
        todo!()
    }

    pub fn generate_dishonest_dispute() {
        todo!()
    }

    pub fn start_game_with_dishonest_dispute() {
        todo!()
    }
}
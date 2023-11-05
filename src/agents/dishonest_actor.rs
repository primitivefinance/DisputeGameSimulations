use super::*;

pub struct DishonestPlayer {
    pub address: Address,
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

    pub fn step() {
        // randomly chose a move to make: attack, defend, or step
        // randomly select a value in some range to has as move's claim
        todo!()
    }
}

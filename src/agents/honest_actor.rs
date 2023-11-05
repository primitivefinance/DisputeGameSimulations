use super::*;

/// actors can call
/// Move (alias for step) is a bisection step, pass in challenge index
/// (index of claim trying to counter, called claim data), only children point to parents for the dag
/// attack (alias for move)
/// deffend (alias for move)
/// step
///
///
/// might need to make a new trait for the solver of off this
/// /Users/shufflebottomhogwood/Code/DisputeGameSimulations/lib/optimism/op-challenger/game/fault/solver/solver.go
/// make player generic over solver generics to be able to sub out any solver
pub struct HonestPlayer {
    pub fault: FaultDisputeSolver<
        [u8; 1],
        AlphabetTraceProvider,
        AlphaClaimSolver<[u8; 1], AlphabetTraceProvider>,
    >, // need to make alphaclaim solver public
    pub state: FaultDisputeState, // <- needs to increment with every new state
    pub address: Address,
    pub client: Arc<RevmMiddleware>,
    pub vm: AlphabetVM<RevmMiddleware>,
    pub oracle: L2OutputOracle<RevmMiddleware>,
    pub block_oracle: BlockOracle<RevmMiddleware>,
    pub disputegame: FaultDisputeGame<RevmMiddleware>,
}

impl HonestPlayer {
    pub fn new() -> Self {
        // pull state to update state with the initial claim
        // subscribe to events on the contract
        todo!()
    }

    pub fn start_game_with_honest_dispute() {
        todo!()
    }

    pub fn generate_honest_dispute() {
        // probably needs to use cannon and or durin here
        todo!()
    }

    pub fn step() {
        // every time we get an event call this on new state
        // update local state
        // ask solver if we need to make any moves self.fault.available_moves() -> return array of moves to make
        // itterate through moves if not skip then return call data and submit move, might need to do some type magic here
        todo!()
    }
}


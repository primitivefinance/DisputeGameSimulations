use alloy_primitives::U256;
use alloy_sol_types::{self, sol};
use anyhow::Result;
use arbiter_core::middleware::RevmMiddleware;
use ethers::utils::keccak256 as ekeccak256;
use foundry_contracts::{
    alphabet_vm::AlphabetVM, block_oracle::block_oracle::BlockOracle,
    fault_dispute_game::fault_dispute_game::FaultDisputeGame,
    l2_output_oracle::l2_output_oracle::L2OutputOracle,
};
use std::{error::Error, sync::Arc};
mod startup;
mod actors;

const ENV_LABEL: &str = "OPTIMISM_FRAUD_PROOF";
const SUBMISSION_INTERVAL: f64 = 1800.0; // output every 30minutes
const L2_BLOCK_TIME: f64 = 2.0;
const FINALIZATION_PERIOD_SECONDS: f64 = 700.0;
// address internal proposer = 0x000000000000000000000000000000000000AbBa;
// address internal owner = 0x000000000000000000000000000000000000ACDC;
// uint256 internal submissionInterval = 1800;
// uint256 internal l2BlockTime = 2;
// uint256 internal startingBlockNumber = 200;
// uint256 internal startingTimestamp = 1000;
// uint256 internal finalizationPeriodSeconds = 7 days;
// https://github.com/ethereum-optimism/optimism/tree/develop/op-challenger

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let (mut _manager, admin, _alice, _bob, _multisig) = startup::set_up_agents().await?;

    let _contracts = startup::deploy_contracts(admin).await?;

    // make our agents 

    // main sim loop 
    // check for event 
    // make action depending on whos turns

    // Things to note:

    Ok(())
}

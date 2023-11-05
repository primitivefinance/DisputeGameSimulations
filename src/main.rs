use alloy_primitives::U256;
use alloy_sol_types::{self, sol};
use anyhow::Result;
use arbiter_core::middleware::RevmMiddleware;
use ethers::utils::keccak256 as ekeccak256;
use foundry_contracts::{
    alphabet_vm::AlphabetVM, block_oracle::block_oracle::BlockOracle,
    dispute_game_factory::DisputeGameFactory,
    fault_dispute_game::fault_dispute_game::FaultDisputeGame,
    l2_output_oracle::l2_output_oracle::L2OutputOracle,
};

use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use std::time::Instant;

pub mod agents;
pub mod settings;
pub mod simulations;

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

/// Represents command-line arguments passed to this binary.
#[derive(Parser)]
#[clap(name = "Excalibur")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Simulation driven development.", long_about = None)]
#[clap(author)]
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true, required = false, action = ArgAction::Count, value_parser(
        clap::value_parser!(u8)))]
    verbose: Option<u8>,
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// Represents the `Bind` subcommand.
    Simulate {
        #[clap(index = 1, default_value = "src/config/dispute_game.toml")]
        config_path: String,
    },
}

// #[tokio::main]
// pub async fn main() -> Result<(), Box<dyn Error>> {
//     let (mut _manager, admin, _alice, _bob, _multisig) = startup::set_up_agents().await?;

//     let _contracts = startup::deploy_contracts(admin).await?;

//     // make our agents

//     // main sim loop
//     // check for event
//     // make action depending on whos turns

//     // Things to note:

//     Ok(())
// }

/// The entry point for the simulation tool.
///
/// This binary provides a command-line interface for the simulation-driven development.
/// It allows users to run simulations by specifying configuration paths, with detailed command-line
/// feedback provided through the `clap` crate.
///
/// # Usage
/// Run the binary without arguments to see available commands and options.
/// Example usage for running simulations:
/// ```
/// $ cargo run simulate [path_to_config]
/// ```
///
/// By default, if no configuration path is provided, it will read from "src/config/gbm.toml".
///
/// These simulations are performed in Arbiter's in memory revm instance and with the exposed RevmMiddleware.
fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Simulate { config_path }) => {
            println!("Reading from config path: {}", config_path);
            let start = Instant::now();
            // This is the entry point for the simulation
            simulations::batch(config_path)?;
            let duration = start.elapsed();
            println!("Total duration of simulations: {:?}", duration);
        }
        None => Args::command().print_long_help()?,
    }
    Ok(())
}

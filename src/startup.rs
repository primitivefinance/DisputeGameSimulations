use arbiter_core::environment::{self, builder::EnvironmentBuilder, cheatcodes::Cheatcodes, Environment};

use super::*;

// pub async fn initialize() -> Result<()> {
//     let (mut _manager, admin, _alice, _bob, _multisig) = startup::set_up().await?;

//     todo!()
// }

/// All the possible contracts that this simulation will actively use, but not
/// all that are deployed!
/// Each is bound to a `Client` and can be used to interact with the contract.
/// The client in this case will be the admin.
#[derive(Clone, Debug)]
pub struct SimulationContracts {
    /// The `L2OutputOracle` contract.
    pub l2_output_oracle: L2OutputOracle<RevmMiddleware>,

    /// The `BlockOracle` contract.
    pub block_oracle: BlockOracle<RevmMiddleware>,

    /// The `AlphabetVM` contract.
    pub alphabet_vm: AlphabetVM<RevmMiddleware>,
}

pub async fn set_up_agents() -> Result<(
    Environment,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
)> {
    let environment = EnvironmentBuilder::new().build();

    let alice = RevmMiddleware::new(
        &environment,
        Some(ENV_LABEL)).unwrap();

    let bob = RevmMiddleware::new(
        &environment,
        Some(ENV_LABEL)).unwrap();

    let multisig = RevmMiddleware::new(
        &environment,
        Some(ENV_LABEL)).unwrap();

    let admin = RevmMiddleware::new(
        &environment,
        Some(ENV_LABEL)).unwrap();

    println!("admin at address {}", admin.address());
    println!("alice at address {}", alice.address());
    println!("bob at address {}", bob.address());
    println!("multisig at address {}", multisig.address());

    alice.apply_cheatcode(Cheatcodes::Deal {
            address: alice.address(),
            amount: U256::MAX.into(),
        })
        .await
        .unwrap();
    bob.apply_cheatcode(Cheatcodes::Deal {
        address: bob.address(),
        amount: U256::MAX.into(),
    })
    .await
    .unwrap();
    multisig.apply_cheatcode(Cheatcodes::Deal {
        address: multisig.address(),
        amount: U256::MAX.into(),
    })
    .await
    .unwrap();
    Ok((environment, admin, alice, bob, multisig))
}

pub async fn deploy_contracts(admin: Arc<RevmMiddleware>) -> Result<SimulationContracts> {
    let sub_interval = eU256::from(SUBMISSION_INTERVAL as u64);
    let l2_block_time = eU256::from(L2_BLOCK_TIME as u64);
    let finalization_period = eU256::from(FINALIZATION_PERIOD_SECONDS as u64);

    let l2_output_oracle = L2OutputOracle::deploy(
        admin.clone(),
        (sub_interval, l2_block_time, finalization_period),
    )?
    .send()
    .await?;
    println!("L2OutputOracle address: {}", l2_output_oracle.address());


    let block_oracle = BlockOracle::deploy(admin.clone(), ())?
    .send()
    .await?;

    println!("BlockOracle address: {}", block_oracle.address());


    sol! {
        type MyValueType is uint256;
    }

    // UDTs are encoded as their underlying type
    let mvt = MyValueType::from(U256::from(1));
    // let data: U256 = "0x0000000000000000000000000000000000000000000000000000000000000001".parse().unwrap();
    let root_claim = ekeccak256(mvt.encode_single()); // replace with the actual root claim

    let alphabet_vm = AlphabetVM::deploy(admin.clone(), root_claim)?
        .send()
        .await?;

    println!("AlphabetVM address: {}", alphabet_vm.address());

    // deploy the dispute game
    // need: 
        // GameType _gameType,
        // Claim _absolutePrestate,
        // uint256 _maxGameDepth,
        // Duration _gameDuration,
    // Have:
        // IBigStepper _vm,
        // L2OutputOracle _l2oo,
        // BlockOracle _blockOracle

    Ok(SimulationContracts {
        l2_output_oracle,
        block_oracle,
        alphabet_vm,})
}
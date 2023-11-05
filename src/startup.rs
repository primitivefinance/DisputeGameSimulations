use alloy_primitives::Bytes;
use arbiter_core::environment::{builder::EnvironmentBuilder, cheatcodes::Cheatcodes, Environment};
use ethers::types::U256 as eU256;

use super::*;

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

    /// The `FaultDisputeGame` contract.
    pub disputegame: FaultDisputeGame<RevmMiddleware>,

    /// The `FaultDisputeGameFactory` contract.
    pub factory: DisputeGameFactory<RevmMiddleware>,
}

pub async fn set_up_agents() -> Result<(
    Environment,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
)> {
    let environment = EnvironmentBuilder::new().label(ENV_LABEL).build();

    let alice = RevmMiddleware::new(&environment, Some("0")).unwrap();

    let bob = RevmMiddleware::new(&environment, Some("1")).unwrap();

    let multisig = RevmMiddleware::new(&environment, Some("2")).unwrap();

    let admin = RevmMiddleware::new(&environment, Some("3")).unwrap();

    println!("admin at address {}", admin.address());
    println!("alice at address {}", alice.address());
    println!("bob at address {}", bob.address());
    println!("multisig at address {}", multisig.address());

    alice
        .apply_cheatcode(Cheatcodes::Deal {
            address: alice.address(),
            amount: eU256::MAX,
        })
        .await
        .unwrap();
    bob.apply_cheatcode(Cheatcodes::Deal {
        address: bob.address(),
        amount: eU256::MAX,
    })
    .await
    .unwrap();
    multisig
        .apply_cheatcode(Cheatcodes::Deal {
            address: multisig.address(),
            amount: eU256::MAX,
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

    // l2_output_oracle
    // details for this are in the proposals.md
    // can produce BS here for alphabet but then need valid ones for mips game
    // https://github.com/ethereum-optimism/optimism/blob/develop/specs/proposals.md#l2-output-commitment-construction
    // need to propose some outputs for game to run
    // one in initial block and one next block
    let result = admin.update_block(1, 32)?;
    println!("result: {:?}", result);
    let call = l2_output_oracle
        .propose_l2_output(
            ekeccak256("Asdfa"),
            eU256::from(1),
            ekeccak256(1 as i32),
            eU256::from(1),
        )
        .send()
        .await?
        .await?;

    let result = call.unwrap();
    println!("result: {:?}", result);
    let block_oracle = BlockOracle::deploy(admin.clone(), ())?.send().await?;

    // checkpoint a point a block after the preposals at least 1 block after the preposals
    println!("BlockOracle address: {}", block_oracle.address());

    sol! {
        type AbiEncodableU256 is uint256;
        type GameType is uint8;
        type Claim is bytes32;
        type Duration is uint64;
    }

    // UDTs are encoded as their underlying type
    let mvt = AbiEncodableU256::from(U256::from(1));
    let root_claim = ekeccak256(AbiEncodableU256::abi_encode(&mvt));

    let alphabet_vm = AlphabetVM::deploy(admin.clone(), root_claim)?
        .send()
        .await?;

    println!("AlphabetVM address: {}", alphabet_vm.address());
    let game_type = GameType::from(0);
    let mut claim = ekeccak256("A");
    claim[0] = 0x01; // need to set zeroith byte
    let depth = eU256::from(4); // 16 letters / numbers supports 2^(depth). Average cannon trace is 2^40 = 40ish B
    let duration = Duration::from(60);

    let disputegame = FaultDisputeGame::deploy(
        admin.clone(),
        (
            game_type.into(),
            claim,
            depth,
            duration.into(),
            alphabet_vm.address(),
            l2_output_oracle.address(),
            block_oracle.address(),
        ),
    )?
    .send()
    .await?;

    println!("FaultDisputeGame address: {}", disputegame.address());

    // GameType _gameType,
    // Claim _absolutePrestate,
    // uint256 _maxGameDepth,
    // Duration _gameDuration,
    // IBigStepper _vm,
    // L2OutputOracle _l2oo,
    // BlockOracle _blockOracle

    // deploy factory contract
    let factory = DisputeGameFactory::deploy(admin.clone(), ())?
        .send()
        .await?;
    println!("DisputeGameFactory address: {}", factory.address());

    // use the factory and call the set implementation on the particular game we have deployed
    // because they need the clones of immutable data
    // bytes calldata _extraData // abi.encode(l2 blocknumber being disputed + l1  block number (checkpointed in block oracle))

    // let new_game = factory.create(game_type.into(), root_claim, );
    let call = factory.set_implementation(game_type.into(), disputegame.address());
    let reciept = call.send().await?.await?.unwrap();
    println!("reciept: {:?}", reciept);

    Ok(SimulationContracts {
        l2_output_oracle,
        block_oracle,
        alphabet_vm,
        disputegame,
        factory,
    })
}

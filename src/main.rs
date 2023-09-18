use anyhow::Result;
use arbiter_core::{
    environment::EnvironmentParameters,
    environment::{BlockSettings, GasSettings},
    manager::Manager,
    middleware::RevmMiddleware,
    math::float_to_wad,
};
use ethers::types::U256;
use std::{error::Error, sync::Arc};
use foundry_contracts::{l2_output_oracle::l2_output_oracle::L2OutputOracle, dispute_game_factory::dispute_game_factory::DisputeGameFactory, proxy::proxy::Proxy};

const ENV_LABEL: &str = "OPTIMISM_FRAUD_PROOF";
const SUBMISSION_INTERVAL: f64 = 1800.0;
const L2_BLOCK_TIME: f64 = 2.0;
const FINALIZATION_PERIOD_SECONDS: f64 = 7.0;
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
    let (mut _manager, admin, alice, _bob, _multisig) = set_up().await?;

    // deploy proxy from admin
    let proxy = Proxy::deploy(admin.clone(), admin.address())?
        .send()
        .await?;

    println!("Proxy address: {}", proxy.address());
    
    let sub_interval = U256::from(SUBMISSION_INTERVAL as u64);
    let l2_block_time = U256::from(L2_BLOCK_TIME as u64);
    let finalization_period = U256::from(FINALIZATION_PERIOD_SECONDS as u64);
    let l2_output_args = (
        float_to_wad(SUBMISSION_INTERVAL),
        float_to_wad(L2_BLOCK_TIME),
        float_to_wad(FINALIZATION_PERIOD_SECONDS),
    );
    println!(" {:?}", proxy.methods.values());
    // Constructor is not defined in abi
    let l2_output_oracle = L2OutputOracle::deploy(admin.clone(), (sub_interval, l2_block_time, finalization_period))?.send().await?;

    println!("L2OutputOracle address: {}", l2_output_oracle.address());

    let factory = DisputeGameFactory::deploy(admin.clone(), ())?
        .send()
        .await?;

    println!("Factory address: {}", factory.address());
    Ok(())
}

async fn set_up() -> Result<(
    Manager,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
    Arc<RevmMiddleware>,
)> {
    let mut manager = Manager::new();
    let _ = manager.add_environment(EnvironmentParameters {
        label: ENV_LABEL.to_owned(),
        block_settings: BlockSettings::UserControlled,
        gas_settings: GasSettings::UserControlled,
    });
    let _ = manager.start_environment(ENV_LABEL);
    let alice = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        None,
    )?);

    let bob = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        None,
    )?);

    let multisig = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        None,
    )?);

    let admin = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        None,
    )?);

    println!("admin at address {}", admin.address());
    println!("alice at address {}", alice.address());
    println!("bob at address {}", bob.address());
    println!("multisig at address {}", multisig.address());

    alice
        .deal(alice.address(), ethers::types::U256::from(1 << 16))
        .await
        .unwrap();
    bob.deal(bob.address(), ethers::types::U256::from(1 << 16))
        .await
        .unwrap();
    multisig
        .deal(multisig.address(), ethers::types::U256::from(1 << 16))
        .await
        .unwrap();
    Ok((manager, admin, alice, bob, multisig))
}

// in DisputeGameFactory.t.sol
// function setUp() public virtual override {
//     super.setUp(); // Calls the function below

//     Proxy proxy = new Proxy(address(this));
//     DisputeGameFactory impl = new DisputeGameFactory();

//     proxy.upgradeToAndCall({
//         _implementation: address(impl),
//         _data: abi.encodeCall(impl.initialize, (address(this)))
//     });
//     factory = DisputeGameFactory(address(proxy));
//     vm.label(address(factory), "DisputeGameFactoryProxy");

//     fakeClone = new FakeClone();
// }

// which is define here in L
//    function setUp() public virtual override {
//     super.setUp();
//     guardian = makeAddr("guardian");

//     // By default the first block has timestamp and number zero, which will cause underflows in the
//     // tests, so we'll move forward to these block values.
//     initL1Time = startingTimestamp + 1;
//     vm.warp(initL1Time);
//     vm.roll(startingBlockNumber);
//     // Deploy the L2OutputOracle and transfer owernship to the proposer
//     oracleImpl = new L2OutputOracle({
//         _submissionInterval: submissionInterval,
//         _l2BlockTime: l2BlockTime,
//         _finalizationPeriodSeconds: finalizationPeriodSeconds
//     });
//     Proxy proxy = new Proxy(multisig);
//     vm.prank(multisig);
//     proxy.upgradeToAndCall(
//         address(oracleImpl),
//         abi.encodeCall(L2OutputOracle.initialize, (startingBlockNumber, startingTimestamp, proposer, owner))
//     );
//     oracle = L2OutputOracle(address(proxy));
//     vm.label(address(oracle), "L2OutputOracle");

//     // Set the L2ToL1MessagePasser at the correct address
//     vm.etch(Predeploys.L2_TO_L1_MESSAGE_PASSER, address(new L2ToL1MessagePasser()).code);

//     vm.label(Predeploys.L2_TO_L1_MESSAGE_PASSER, "L2ToL1MessagePasser");
// }

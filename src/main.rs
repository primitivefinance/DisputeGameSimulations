use anyhow::Result;
use arbiter_core::{
    environment::EnvironmentParameters,
    environment::{BlockSettings, GasSettings},
    manager::Manager,
    middleware::RevmMiddleware,
};
use std::{error::Error, sync::Arc};
mod bindings;
use crate::bindings::{
    dispute_game_factory::dispute_game_factory::DisputeGameFactory, proxy::proxy::Proxy,
};

const ENV_LABEL: &str = "OPTIMISM_FRAUD_PROOF";
// https://github.com/ethereum-optimism/optimism/tree/develop/op-challenger

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let (mut _manager, alice, _bob, _multisig) = set_up().await?;

    // deploy proxy from alice
    let proxy = Proxy::deploy(alice.clone(), alice.address())
        .unwrap()
        .send()
        .await
        .unwrap();

    println!("Proxy address: {}", proxy.address());

    let factory = DisputeGameFactory::deploy(alice.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();

    println!("Factory address: {}", factory.address());
    Ok(())
}

async fn set_up() -> Result<(
    Manager,
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
    Ok((manager, alice, bob, multisig))
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

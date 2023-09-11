use arbiter_core::{
    environment::EnvironmentParameters, manager::Manager, middleware::RevmMiddleware,
};
use ethers::providers::Middleware;
use std::{error::Error, sync::Arc};
mod bindings;
use crate::bindings::dispute_game_factory::dispute_game_factory::DisputeGameFactory;

const TEST_ENV_LABEL: &str = "test_env";
// https://github.com/ethereum-optimism/optimism/tree/develop/op-challenger

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = Manager::new();

    let _ = manager.add_environment(
        TEST_ENV_LABEL,
        EnvironmentParameters {
            block_rate: 1.0,
            seed: 1,
        },
    );
    let _ = manager.start_environment(TEST_ENV_LABEL);
    let client_with_signer = Arc::new(RevmMiddleware::new(
        manager.environments.get(TEST_ENV_LABEL).unwrap(),
        None,
    ));
    println!("client at address {}", client_with_signer.default_sender().unwrap());

    let factory = DisputeGameFactory::deploy(client_with_signer.clone(),())
        .unwrap()
        .send()
        .await
        .unwrap();

    println!("Factory address: {}", factory.address());
    Ok(())
}

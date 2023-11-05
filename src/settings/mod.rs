pub mod parameters;

use parameters::*;

use crate::simulations::SimulationType;
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

/// Defines the configuration for a simulation.
///
/// This struct holds all the necessary parameters and configurations needed to run a simulation.
/// It encompasses several sub-configurations such as `TrajectoryParameters` and `GBMParameters`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// The type of simulation to run, defined by an enum `SimulationType`.
    pub simulation: SimulationType,

    /// Maximum number of parallel simulations to run.
    pub max_parallel: Option<usize>,

    /// Directory where the simulation output will be stored.
    pub output_directory: String,

    /// Name of the file where the simulation results will be written.
    pub output_file_name: Option<String>,

    /// Parameters related to block configurations.
    pub block: BlockParameters,
}

impl SimulationConfig {
    /// Creates a new `SimulationConfig` instance from a configuration file.
    ///
    /// Reads the specified configuration file and deserializes it into a `SimulationConfig` object.
    /// The `config_path` is the path to the configuration file in question.
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()?;
        s.try_deserialize()
    }
}
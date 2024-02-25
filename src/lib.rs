#![doc = include_str!("../README.md")]

pub mod atoc;
pub mod setup;
pub mod utils;

use setup::Config;
use std::error::Error;

/// Execute the complete `atoc2gtfs` pipeline using the [Config] setup:
/// 1. Unzip ATOC input and check for expected contents.
///
/// # Arguments
///
/// - `config`: An instance of [Config].
///
/// # Example
/// ```
/// use atoc2gtfs::{setup::Config, run};
/// let input_atoc_path = "./tests/data/empty_atoc.zip";                     // input ATOC.CIF path
/// let output_gtfs_path = "./data/output_gtfs.zip";                         // output GTFS path
/// let config = Config::new(&input_atoc_path, &output_gtfs_path).unwrap();  // setup a new Config
/// assert!(run(config).is_ok());                                            // run the pipeline
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Converting {:?} to GTFS...", config.input_path);

    Config::unzip_atoc_and_check(config)?;

    Ok(())
}

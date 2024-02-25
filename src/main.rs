use atoc2gtfs::{run, setup::Config};
use std::env;
use std::process;

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build_from_cli(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {:?}", err);
        process::exit(1);
    });

    // execute atoc2gtfs to convert input to GTFS
    if let Err(e) = run(config) {
        eprintln!("atoc2gtfs error: {:?}", e);
        process::exit(1);
    }
}

use atoc2gtfs::cli;
use std::env;
use std::process;

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    let config: cli::Config = cli::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Converting {:?} to GTFS...", config.input_path)
}

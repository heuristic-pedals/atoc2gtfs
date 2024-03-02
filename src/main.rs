use atoc2gtfs::{run, setup::Config};
use std::env;
use std::process;

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();

    // handle case where help is requested `atoc2gtfs -h` or `atoc2gtfs --help`
    if (args.len() == 2) && (args[1] == "-h" || args[1] == "--help") {
        println!("{}", cli_help_msg());
        process::exit(0);
    }

    // parse CLI arguments
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

// provide help/usage information
fn cli_help_msg() -> &'static str {
    concat!(
        "ATOC.cif to GTFS conversion tool.\n\n",
        "Usage: atoc2gtfs [ARGS]...\n\n",
        "Arguments:\n",
        "    ATOC_INPUT_PATH:\tpath to the input ATOC file (.zip file)\n",
        "    GTFS_OUTPUT_PATH:\tpath to the output GTFS file (.zip file)\n",
    )
}

#[cfg(test)]
mod tests {
    use crate::cli_help_msg;

    #[test]
    fn cli_help_msg_on_pass() {
        let msg = cli_help_msg();
        assert!(
            msg.contains("Usage:"),
            "Help message does not contain usage instructions"
        );
        assert!(
            msg.contains("Arguments:"),
            "Help message does not contain Arguments section"
        );
        assert!(
            msg.contains("ATOC_INPUT_PATH:"),
            "Does not contain ATOC_INPUT_PATH info"
        );
        assert!(
            msg.contains("GTFS_OUTPUT_PATH:"),
            "Does not contain GTFS_OUTPUT_PATH info"
        );
    }
}

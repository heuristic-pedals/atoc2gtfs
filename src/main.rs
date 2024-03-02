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

    // handle case where version info is requested `atoc2gtfs -v` or `atoc2gtfs --version`
    if (args.len() == 2) && (args[1] == "-v" || args[1] == "--version") {
        println!("{}", cli_version_msg());
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
        "`atoc2gtfs`: an ATOC to GTFS conversion tool.\n\n",
        "Usage: atoc2gtfs [OPTIONS] [ARGS]...\n",
        "\nOptions:\n",
        "    -h:\tdisplay this help message (or --help)\n",
        "    -v:\tdisplay the version number (or --version)\n",
        "\nArguments:\n",
        "    ATOC_INPUT_PATH:\tpath to the input ATOC file (.zip file)\n",
        "    GTFS_OUTPUT_PATH:\tpath to the output GTFS file (.zip file)\n",
    )
}

// provide version details - use option_env! to handle case where cargo is not used to build
// advice take from: https://stackoverflow.com/questions/27840394/how-can-a-rust-program-access-meta
// data-from-its-cargo-package
fn cli_version_msg() -> &'static str {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("unknown")
}

#[cfg(test)]
mod tests {
    use crate::cli_help_msg;

    // check core parts of help message
    #[test]
    fn cli_help_msg_on_pass() {
        let msg = cli_help_msg();
        let checks: [&str; 7] = [
            "Usage:",
            "Options:",
            "-h:",
            "-v:",
            "Arguments:",
            "ATOC_INPUT_PATH:",
            "GTFS_OUTPUT_PATH:",
        ];
        for check in checks {
            assert!(
                msg.contains(check),
                "Help message did not contain '{}'",
                check
            );
        }
    }
}

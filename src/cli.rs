use std::cmp::Ordering;

/// Capture and collect the runtime configuration
pub struct Config {
    /// path to the input file
    pub input_path: String,
    /// path to write output file
    pub output_path: String,
}

impl Config {
    pub fn build(parsed_args: &Vec<String>) -> Result<Config, &'static str> {
        match parsed_args.len().cmp(&3) {
            Ordering::Greater => return Err("Too many argument provided."),
            Ordering::Less => return Err("Too few arguments provided."),
            Ordering::Equal => (),
        }
        let input_path: String = parsed_args[1].clone();
        let output_path: String = parsed_args[2].clone();
        Ok(Config {
            input_path,
            output_path,
        })
    }
}

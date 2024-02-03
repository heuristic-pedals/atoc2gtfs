use std::cmp::Ordering;
use std::path::Path;

/// Capture and collect the runtime configuration
pub struct Config<'a> {
    /// path to the input file
    pub input_path: &'a Path,
    /// path to write output file
    pub output_path: &'a Path,
}

impl<'a> Config<'a> {
    pub fn build(parsed_args: &Vec<String>) -> Result<Config, &'static str> {
        match parsed_args.len().cmp(&3) {
            Ordering::Greater => return Err("Too many argument provided."),
            Ordering::Less => return Err("Too few arguments provided."),
            Ordering::Equal => (),
        }

        let input_path: &Path = Path::new(&parsed_args[1]);
        let output_path: &Path = Path::new(&parsed_args[2]);

        Ok(Config {
            input_path,
            output_path,
        })
    }
}

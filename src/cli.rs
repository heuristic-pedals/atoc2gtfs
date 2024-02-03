use std::cmp::Ordering;
use std::ffi::OsStr;
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

        const ACCEPT_ZIP_EXTS: [&str; 2] = ["zip", "ZIP"];

        match parsed_args.len().cmp(&3) {
            Ordering::Greater => return Err("Too many argument provided."),
            Ordering::Less => return Err("Too few arguments provided."),
            Ordering::Equal => (),
        }

        let input_path: &Path = Path::new(&parsed_args[1]);
        let output_path: &Path = Path::new(&parsed_args[2]);

        if !input_path.exists() {
            return Err("Provided input path does not exist.")
        }
        if !input_path.is_file() {
            return Err("Provided input is not a file.")
        }
        
        match input_path.extension().and_then(OsStr::to_str) {
            Some(ext) => if !ACCEPT_ZIP_EXTS.contains(&ext) {
                return Err("Provided input is not a zip file.")
            },
            None => return Err("Unable to determine the input file extension."),
        };

        Ok(Config {
            input_path,
            output_path,
        })
    }
}

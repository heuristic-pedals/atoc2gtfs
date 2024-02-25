//! Handles runtime command line parsing for the binary crate.
use crate::atoc::AtocError;
use crate::utils;
use std::cmp::Ordering;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

/// Capture and collect the runtime configuration
pub struct Config<'a> {
    /// path to the input file
    pub input_path: &'a Path,
    /// path to write output file
    pub output_path: &'a Path,
}

impl<'a> Config<'a> {
    /// Builds an instance of `Config` after parsing CLI inputs. Used to collect input
    /// and output paths, check the input exists and is a file, and that both both the
    /// input and outpus are zip file. Both `.zip` and `.ZIP` are valid extensions.
    ///
    /// # Arguments
    ///
    /// * `parsed_args` - A vector of strings denoting the parsed inputs.
    /// Expecting the format: [BINARY_NAME, ATOC_PATH, OUTPUT_PATH] (this
    /// is the result of calling `std::env::args().collect()`)
    ///
    /// # Examples
    ///
    /// ```
    /// use atoc2gtfs::setup::Config;
    /// let dummy_parsed_args = vec![
    ///     "".to_string(),                 // empty dummy binary name (not used)
    ///     "./tests/data/dummy_empty.zip".to_string(),  // dummy sub-string query
    ///     "dummy_output.zip".to_string(),    // dummy file path
    /// ];
    /// let config = Config::build_from_cli(&dummy_parsed_args);
    /// assert!(config.is_ok());
    /// ```
    /// # See Also
    ///
    /// - [Config::new] - Create a `Config` instance by supplying arguments directly.
    pub fn build_from_cli(parsed_args: &[String]) -> Result<Config, String> {
        const NUM_REQ_ARGS: usize = 2;
        let num_inputted_req_args: usize = parsed_args.len() - 1;
        let req_arg_err_msg: String = format!(
            " required arguments provided. Expected {}, got {}.",
            NUM_REQ_ARGS, num_inputted_req_args
        );
        match num_inputted_req_args.cmp(&NUM_REQ_ARGS) {
            Ordering::Greater => return Err("Too many".to_string() + &req_arg_err_msg),
            Ordering::Less => return Err("Too few".to_string() + &req_arg_err_msg),
            Ordering::Equal => (),
        }

        Config::new(&parsed_args[1], &parsed_args[2])
    }

    /// Create an instance of `Config`. Used to collect input and output paths, check the
    /// input exists and is a file, and that both both the input and outpus are zip file.
    /// Both `.zip` and `.ZIP` are valid extensions.
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the input ATOC zip file to be converted, as a str.
    /// * `output_path` - Path to user to save the converted GTFS file, as a str.
    ///
    /// # Examples
    ///
    /// ```
    /// use atoc2gtfs::setup::Config;
    /// let input_path = "./tests/data/dummy_empty.zip";
    /// let output_path = "dummy_output.zip";
    /// let config = Config::new(&input_path, &output_path);
    /// assert!(config.is_ok());
    /// ```
    ///
    /// # See Also
    ///
    /// - [Config::build_from_cli] - Building `Config` by parsing CLI arguments.
    pub fn new(input_path: &'a str, output_path: &'a str) -> Result<Config<'a>, String> {
        let input_path: &Path = Path::new(input_path);
        let output_path: &Path = Path::new(output_path);

        if !input_path.exists() {
            return Err(format!("{:?} does not exist.", input_path));
        }
        if !input_path.is_file() {
            return Err(format!("{:?} is not a file.", input_path));
        }

        let accept_zip_exts: Vec<&str> = vec!["zip", "ZIP"];
        utils::io::check_extension(input_path, &accept_zip_exts)?;
        utils::io::check_extension(output_path, &accept_zip_exts)?;

        Ok(Config {
            input_path,
            output_path,
        })
    }
    pub fn unzip_atoc_and_check(config: Config) -> Result<(), Box<dyn Error>> {
        const EXPECPTED_ATOC_EXTS: [&str; 2] = ["mca", "msn"];
        let mut expected_atoc_exts = EXPECPTED_ATOC_EXTS
            .iter()
            .map(OsStr::new)
            .collect::<Vec<&OsStr>>();

        // open the zipped ATOC.CIF file for reading.
        let file = File::open(config.input_path)?;
        let mut archive = ZipArchive::new(file)?;

        // Iterate through all the files in the ZIP archive.
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            let extension = Path::new(file.name()).extension();
            if let Some(ext) = extension {
                if expected_atoc_exts.contains(&ext) {
                    expected_atoc_exts.retain(|e| *e != ext);
                }
            }
        }

        // raise an error if expected extensions can't be found
        if !expected_atoc_exts.is_empty() {
            let atoc_error = AtocError {
                code: 1,
                add_message: format!("{:?}", expected_atoc_exts),
            };
            return Err(atoc_error.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_from_cli_on_pass() {
        let dummy_input_path = "./tests/data/dummy_empty.zip";
        let dummy_output_path = "dummy_output.zip";

        let dummy_parsed_args = vec![
            "".to_string(),
            dummy_input_path.to_string(),
            dummy_output_path.to_string(),
        ];
        let config = Config::build_from_cli(&dummy_parsed_args);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(
            config.input_path.to_str().unwrap(),
            dummy_input_path,
            "Unexpected `input_path` value {:?}",
            config.input_path
        );
        assert_eq!(
            config.output_path.to_str().unwrap(),
            dummy_output_path,
            "Unexpected `file_path` value {:?}",
            config.output_path
        );
    }

    #[test]
    fn config_build_from_cli_too_many_req_args() {
        let numerous_args = vec!["".to_string(); 4];
        let config = Config::build_from_cli(&numerous_args);
        // split out assertions to imporve test debug messages
        assert!(config.is_err(), "Too many arguments case was not detected.");
        assert!(
            // check equality since err message not expected to change
            config.is_err_and(|err| err.contains("Too many required arguments provided")),
            "Unexpected error message when passing too many arguments."
        );
    }

    #[test]
    fn config_build_from_cli_too_few_req_args() {
        let sparse_args = vec!["".to_string(); 2];
        let config = Config::build_from_cli(&sparse_args);
        // split out assertions to imporve test debug messages
        assert!(config.is_err(), "Too few arguments case was not detected.");
        assert!(
            // check equality since err message not expected to change
            config.is_err_and(|err| err.contains("Too few required arguments provided")),
            "Unexpected error message when passing too few arguments."
        );
    }

    #[test]
    fn config_new_on_pass() {
        let dummy_input_path = "./tests/data/dummy_empty.zip";
        let dummy_output_path = "dummy_output.zip";

        let config = Config::new(&dummy_input_path, &dummy_output_path);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(
            config.input_path.to_str().unwrap(),
            dummy_input_path,
            "Unexpected `input_path` value {:?}",
            config.input_path
        );
        assert_eq!(
            config.output_path.to_str().unwrap(),
            dummy_output_path,
            "Unexpected `file_path` value {:?}",
            config.output_path
        );
    }

    #[test]
    fn config_new_input_does_not_exist() {
        let config = Config::new("does_not_exist.zip", "dummy_output.zip");
        assert!(
            config.is_err(),
            "No error raised when provided non-existent input."
        );
        assert!(
            config.is_err_and(|err| err.contains("does not exist")),
            "Unexpected error message when passing a non-existent input."
        );
    }

    #[test]
    fn config_new_input_not_a_file() {
        let config = Config::new("./tests/data/", "dummy_output.zip");
        assert!(
            config.is_err(),
            "No error raised when provided a folder path as an input."
        );
        assert!(
            config.is_err_and(|err| err.contains("is not a file")),
            "Unexpected error message when passing a folder path as an input."
        );
    }

    #[test]
    fn config_new_input_not_a_zip() {
        let config = Config::new("./tests/data/dummy_empty.txt", "dummy_output.zip");
        assert!(
            config.is_err(),
            "No error raised when provided a text file as input."
        );
    }

    #[test]
    fn config_new_output_not_a_zip() {
        let config = Config::new(&"./tests/data/dummy_empty.zip", "dummy_output.text");
        assert!(
            config.is_err(),
            "No error raised when provided a text file as output."
        );
    }

    #[test]
    fn unzip_atoc_and_check_on_pass() {
        let config = Config::new("./tests/data/empty_atoc.zip", "output.zip").unwrap();
        let result = Config::unzip_atoc_and_check(config);
        assert!(result.is_ok());
    }

    #[test]
    fn unzip_atoc_and_check_no_atoc_files() {
        let config = Config::new("./tests/data/dummy_empty.zip", "output.zip").unwrap();
        let result = Config::unzip_atoc_and_check(config);
        assert!(
            result.is_err(),
            "No error raised when unzipping ATOC with only a txt file inside."
        );
        assert!(
            result.is_err_and(|err| format!("{}", err)
                .contains("AtocError code 1: ATOC input does not contain expected file type(s):")),
            "Unexpected error message unzipping ATOC with only a txt file inside."
        );
    }
}

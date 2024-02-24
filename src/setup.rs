//! Handles runtime command line parsing for the binary crate.
use crate::utils;
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
    /// Builds an instance of `Config` after parsing CLI inputs
    ///
    /// # Arguments
    ///
    /// * `parsed_args` - A vector of strings denoting the parsed inputs.
    /// Expecting the format: [BINARY_NAME, ATOC_PATH, OUTPUT_PATH] (this
    /// is the result of calling `std::env::args().collect()`)
    ///
    /// > Note: the format of `parsed_args` is currently 'awkward' to use. A
    /// future feature will be to implement `Config::new()` such that use cases
    /// that don't require a cli can be catered for.
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
    /// let config = Config::build(&dummy_parsed_args);
    /// assert!(config.is_ok());
    /// ```
    pub fn build(parsed_args: &[String]) -> Result<Config, String> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_on_pass() {
        let dummy_input_path = "./tests/data/dummy_empty.zip";
        let dummy_output_path = "dummy_output.zip";

        let dummy_parsed_args = vec![
            "".to_string(),
            dummy_input_path.to_string(),
            dummy_output_path.to_string(),
        ];
        let config = Config::build(&dummy_parsed_args);
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
    fn config_too_many_req_args() {
        let numerous_args = vec!["".to_string(); 4];
        let config = Config::build(&numerous_args);
        // split out assertions to imporve test debug messages
        assert!(config.is_err(), "Too many arguments case was not detected.");
        assert!(
            // check equality since err message not expected to change
            config.is_err_and(|err| err.contains("Too many required arguments provided")),
            "Unexpected error message when passing too many arguments."
        );
    }

    #[test]
    fn config_too_few_req_args() {
        let sparse_args = vec!["".to_string(); 2];
        let config = Config::build(&sparse_args);
        // split out assertions to imporve test debug messages
        assert!(config.is_err(), "Too few arguments case was not detected.");
        assert!(
            // check equality since err message not expected to change
            config.is_err_and(|err| err.contains("Too few required arguments provided")),
            "Unexpected error message when passing too few arguments."
        );
    }

    #[test]
    fn config_input_does_not_exist() {
        let non_exist_input = vec![
            "".to_string(),
            "does_not_exist.zip".to_string(),
            "dummy_output.zip".to_string(),
        ];
        let config = Config::build(&non_exist_input);
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
    fn config_input_not_a_file() {
        let folder_input = vec![
            "".to_string(),
            "./tests/data/".to_string(),
            "dummy_output.zip".to_string(),
        ];
        let config = Config::build(&folder_input);
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
    fn config_input_not_a_zip() {
        let text_input = vec![
            "".to_string(),
            "./tests/data/dummy_empty.txt".to_string(),
            "dummy_output.zip".to_string(),
        ];
        let config = Config::build(&text_input);
        assert!(
            config.is_err(),
            "No error raised when provided a text file as input."
        );
    }

    #[test]
    fn config_output_not_a_zip() {
        let test_output = vec![
            "".to_string(),
            "./tests/data/dummy_empty.zip".to_string(),
            "dummy_output.text".to_string(),
        ];
        let config = Config::build(&test_output);
        assert!(
            config.is_err(),
            "No error raised when provided a text file as output."
        );
    }
}

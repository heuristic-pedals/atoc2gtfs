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
    pub fn build(parsed_args: &Vec<String>) -> Result<Config, String> {
        let args: &Vec<String> = &parsed_args[1..].to_vec();

        const NUM_REQ_ARGS: usize = 2;
        let num_inputted_req_args: usize = args.len();
        let req_arg_err_msg: String = format!(
            " required arguments provided. Expected {}, got {}.",
            NUM_REQ_ARGS, num_inputted_req_args
        );
        match num_inputted_req_args.cmp(&NUM_REQ_ARGS) {
            Ordering::Greater => return Err("Too many".to_string() + &req_arg_err_msg),
            Ordering::Less => return Err("Too few".to_string() + &req_arg_err_msg),
            Ordering::Equal => (),
        }

        let input_path: &Path = Path::new(&parsed_args[1]);
        let output_path: &Path = Path::new(&parsed_args[2]);

        if !input_path.exists() {
            return Err(format!("{:?} does not exist.", input_path));
        }
        if !input_path.is_file() {
            return Err(format!("{:?} is not a file.", input_path));
        }

        Config::check_zip(input_path)?;
        Config::check_zip(output_path)?;

        Ok(Config {
            input_path,
            output_path,
        })
    }

    fn check_zip(path: &Path) -> Result<(), String> {
        const ACCEPT_ZIP_EXTS: [&str; 2] = ["zip", "ZIP"];

        match path.extension().and_then(OsStr::to_str) {
            Some(ext) => {
                if !ACCEPT_ZIP_EXTS.contains(&ext) {
                    return Err(format!("{:?} is not a zip file.", path));
                }
            }
            None => return Err(format!("Unable to determine file extension for {:?}", path)),
        };

        Ok(())
    }
}

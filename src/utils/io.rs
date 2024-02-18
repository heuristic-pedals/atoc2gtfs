//! A set of utility methods related to inputs and outputs.
use std::ffi::OsStr;
use std::path::Path;

/// Checks if the provded input path has the expected file extension.
///
/// Returns a `Result` enmum as follows:
/// - `OK(())` when the provided path has the expected file extension.
/// - `Er()` in all other cases. This could be due to the path having no file extension
/// (say a directory) or it is not a zip file (say .txt). The `Er()` will contain
/// a `String` error message with more details.
///
/// > Note: this function wraps [`std::path::Path::extension`]. See this method's
/// documentation for more details.
///
/// # Arguments
///
/// * `path` - Path to input file to test.
/// * `extensions` - A vector of `&str` types containing valid extensions.
///
/// # Example (checking for a ZIP archive file)
/// ```
/// use atoc2gtfs::utils::io;
/// use std::path::Path;
///
/// let zip_example = Path::new("example.zip");             // is a zip file
/// let text_example = Path::new("example.txt");            // is not a zip file
/// let accept_zip_exts: Vec<&str> = vec!["zip", "ZIP"];    // valid extensions
/// assert!(io::check_extension(&zip_example, &accept_zip_exts).is_ok());
/// assert!(io::check_extension(&text_example, &accept_zip_exts).is_err());
/// ```
pub fn check_extension(path: &Path, extensions: &Vec<&str>) -> Result<(), String> {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) => {
            if !extensions.contains(&ext) {
                return Err(format!(
                    "{:?} is not the expected file type. Expected one of: {:?}",
                    path, extensions
                ));
            }
        }
        None => return Err(format!("Unable to determine file extension for {:?}", path)),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn check_extension_on_pass() {
        let dummy_inputs: [&Path; 2] = [
            Path::new("./tests/data/dummy_empty.zip"),
            Path::new("./tests/data/dummy_empty_capital.ZIP"),
        ];
        let accept_zip_exts: Vec<&str> = vec!["zip", "ZIP"];
        for dummy_input in dummy_inputs {
            assert!(check_extension(&dummy_input, &accept_zip_exts).is_ok());
        }
    }

    #[test]
    fn check_extension_not_a_zip() {
        let text_input = Path::new("./tests/data/dummy_empty.txt");
        let accept_zip_exts: Vec<&str> = vec!["zip", "ZIP"];
        let result = check_extension(&text_input, &accept_zip_exts);
        assert!(
            result.is_err(),
            "Did not raise error when provided text input."
        );
        assert!(
            result.is_err_and(|err| err
                .contains("is not the expected file type. Expected one of: [\"zip\", \"ZIP\"]")),
            "Unexpected error message when passing a text file as an input."
        )
    }

    #[test]
    fn check_extension_no_file_extension() {
        let no_file_ext = Path::new("./tests/data/dummy_empty");
        let accept_zip_exts: Vec<&str> = vec!["zip", "ZIP"];
        let result = check_extension(&no_file_ext, &accept_zip_exts);
        assert!(
            result.is_err(),
            "Did not raise error when input with no extension was provided."
        );
        assert!(
            result.is_err_and(|err| err.contains("Unable to determine file extension")),
            "Unexpected error message when passing an input with no file extension."
        )
    }
}

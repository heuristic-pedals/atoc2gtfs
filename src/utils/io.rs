//! A set of utility methods related to inputs and outputs.
use std::ffi::OsStr;
use std::path::Path;

/// Checks if the provded input path is a zip file based on its extension.
///
/// Both `.zip` and `.ZIP` file exentions are considered zip files. Returns a
/// `Result` enmum as follows:
/// - `OK(())` when the provided path has a zip file extension.
/// - `Er()` when no zip file extension is found. This could be due to the path
/// having no file extension (say a directory) or it is not a zip file (say .txt).
/// The `Er()` will contain a `String` error message with more details.
///
/// > Note: this function wraps [`std::path::Path::extension`]. See this method's
/// documentation for more details.
///
/// # Arguments
///
/// * `path` - Path to input file to test.
///
/// # Example Usage
/// ```
/// use atoc2gtfs::utils::io;
/// use std::path::Path;
///
/// // is a zip file
/// let zip_example = Path::new("example.zip");
/// assert!(io::check_zip(&zip_example).is_ok());
///
/// // is not a zip file
/// let text_example = Path::new("example.txt");
/// assert!(io::check_zip(&text_example).is_err());
/// ```
pub fn check_zip(path: &Path) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn check_zip_on_pass() {
        let dummy_inputs: [&Path; 2] = [
            Path::new("./tests/data/dummy_empty.zip"),
            Path::new("./tests/data/dummy_empty_capital.ZIP"),
        ];
        for dummy_input in dummy_inputs {
            assert!(check_zip(&dummy_input).is_ok());
        }
    }

    #[test]
    fn check_zip_not_a_zip() {
        let text_input = Path::new("./tests/data/dummy_empty.txt");
        let result = check_zip(&text_input);
        assert!(
            result.is_err(),
            "Did not raise error when provided text input."
        );
        assert!(
            result.is_err_and(|err| err.contains("is not a zip file")),
            "Unexpected error message when passing a text file as an input."
        )
    }

    #[test]
    fn check_zip_no_file_extension() {
        let no_file_ext = Path::new("./tests/data/dummy_empty");
        let result = check_zip(&no_file_ext);
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

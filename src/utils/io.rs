use std::ffi::OsStr;
use std::path::Path;

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

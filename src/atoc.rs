//! ATOC parsing, decoding, and erorr handling.

use std::{error, fmt};

// define an atoc error struct
pub struct AtocError {
    pub code: usize,
    pub add_message: String,
}

// different ATOC related error messages according to code
impl fmt::Display for AtocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            1 => "ATOC input does not contain expected file type(s):",
            _ => "",
        };

        write!(
            f,
            "AtocError code {}: {} {}",
            self.code, err_msg, self.add_message
        )
    }
}

// debug trait required, but duplicate the display (no need for speicifc debug method here)
// adopted from https://stackoverflow.com/questions/69964338/define-fmtdisplay-and-fmtdebug-together
impl fmt::Debug for AtocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl error::Error for AtocError {}

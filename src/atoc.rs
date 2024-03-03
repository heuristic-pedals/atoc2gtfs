//! ATOC parsing, decoding, and erorr handling.

pub mod msn;

use crate::atoc::msn::{Msn, MsnRecord};
use crate::setup::Config;
use std::{error, fmt};

#[derive(Debug)]
pub struct Atoc<'a> {
    pub msn: Vec<MsnRecord<'a>>,
}

impl<'a> Atoc<'a> {
    pub fn parse_input(config: Config) -> Atoc {
        Atoc {
            msn: Msn::parse(config.input_path),
        }
    }
}

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

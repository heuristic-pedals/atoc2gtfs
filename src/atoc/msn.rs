//! ATOC msn file parsing, decoding, and erorr handling.

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

#[derive(Debug)]
pub struct Msn {}

#[derive(Debug)]
pub struct MsnRecord<'a> {
    pub station_name: &'a str,
    pub tiploc_code: &'a str,
    pub crs_code: &'a str,
    pub easting: usize,
    pub northing: usize,
    pub latitude: f64,
    pub longitude: f64,
}

impl Msn {
    pub fn parse(input_zip_path: &Path) -> Result<Vec<MsnRecord>, Box<dyn Error>> {
        dbg!(input_zip_path);

        // Open the ZIP file for reading.
        let file = File::open(input_zip_path)?;
        let archive = ZipArchive::new(&file)?;
        let archived_msn = archive
            .file_names()
            .filter(|fname| fname.ends_with(".msn"))
            .collect::<Vec<&str>>();

        let msn_fname = match archived_msn.len() {
            1 => archived_msn[0],
            _ => return Err("Unable to find the .msn file in the .zip".into()),
        };

        let mut archive = ZipArchive::new(&file)?;
        let mut msn_file = archive.by_name(msn_fname)?;

        let mut buffer = Vec::new();
        msn_file.read_to_end(&mut buffer)?;
        let msn_contents = String::from_utf8(buffer)?;
        for line in msn_contents.lines() {
            dbg!(line);
        }

        Ok(vec![MsnRecord {
            station_name: "Test",
            tiploc_code: "Test",
            crs_code: "Test",
            easting: 0,
            northing: 0,
            latitude: 0.0,
            longitude: 0.0,
        }])
    }
    pub fn parse_line(msn_line: &str) -> Option<MsnRecord> {
        if msn_line[..1].ne("A") || (msn_line[..1].eq("A") && msn_line[4..5].eq(" ")) {
            return None;
        }

        Some(MsnRecord {
            station_name: msn_line[4..31].trim(),
            tiploc_code: "Test",
            crs_code: "Test",
            easting: 0,
            northing: 0,
            latitude: 0.0,
            longitude: 0.0,
        })
    }
}

//! ATOC msn file parsing, decoding, and erorr handling.

use std::path::Path;

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
    pub fn parse(msn_path: &Path) -> Vec<MsnRecord> {
        dbg!(msn_path);
        vec![MsnRecord {
            station_name: "Test",
            tiploc_code: "Test",
            crs_code: "Test",
            easting: 0,
            northing: 0,
            latitude: 0.0,
            longitude: 0.0,
        }]
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

use atoc2gtfs::{run, setup::Config};

#[test]
fn run_atoc2gtfs() {
    let config = Config::new("./tests/data/dummy_atoc.zip", "./data/output_gtfs.zip").unwrap();
    assert!(run(config).is_ok());
}

## run llvm-cov and open coverage report
coverage:
	cargo llvm-cov --all-features --workspace --ignore-filename-regex main --open

## make and open docs
docs:
	cargo doc --no-deps
	open ./target/doc/atoc2gtfs/index.html
## run llvm-cov and open coverage report
coverage:
	cargo llvm-cov --all-features --workspace --ignore-filename-regex main --open
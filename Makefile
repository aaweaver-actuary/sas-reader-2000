
coverage:
	cargo llvm-cov clean
	cargo llvm-cov --lcov --output-path lcov.info
	cargo llvm-cov --output-dir target/llvm-cov --html

release:
	cargo build --release
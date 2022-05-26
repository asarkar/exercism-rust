# exercism-rust
[Exercism's Rust Track](https://exercism.org/tracks/rust)

[![](https://github.com/asarkar/exercism-rust/workflows/CI/badge.svg)](https://github.com/asarkar/exercism-rust/actions)

## Running tests
```
RUST_BACKTRACE=1 cargo test --manifest-path </path/to/Cargo.toml> \
	--all-features -- --include-ignored --nocapture <test_name>
```

This actually runs all tests with names beginning with `test_name`.
For exact match, use `--exact` flag.

To run a test in file `src/bitset.rs`, under module `tests`, use the path 
`bitset::tests::test_bits` instead of test name. To run all tests in that module,
use the path `bitset::tests`.
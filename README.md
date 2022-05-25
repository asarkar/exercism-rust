# exercism-rust
[Exercism's Rust Track](https://exercism.org/tracks/rust)

[![](https://github.com/asarkar/exercism-rust/workflows/CI/badge.svg)](https://github.com/asarkar/exercism-rust/actions)

To run a single test:
```
RUST_BACKTRACE=1 cargo test --manifest-path </path/to/Cargo.toml> \
	--all-features -- --ignored --show-output <test_name>
```

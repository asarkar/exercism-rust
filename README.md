# exercism-rust
My solutions for [Exercism Rust Track](https://exercism.org/tracks/rust).
Feel free to open issues for questions, comments, or suggestions.

[![](https://github.com/asarkar/exercism-rust/workflows/CI/badge.svg)](https://github.com/asarkar/exercism-rust/actions)

## Running tests
```
RUST_BACKTRACE=1 cargo test --release --manifest-path </path/to/Cargo.toml> \
	--all-features -- --include-ignored --nocapture --exact <test_name>
```

To run a test `test_bits` in file `src/bitset.rs`, under module `tests`, use the path 
`bitset::tests::test_bits`. To run all tests in that module, use the path `bitset::tests`.

To run a test `test_bits` in file `src/lib.rs`, under module `tests`, use the path 
`tests::test_bits`.

## References

* https://www.philipdaniels.com/tags/rust/

## License

Released under [Apache License v2.0](LICENSE).
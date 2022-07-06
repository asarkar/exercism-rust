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

* [Phil's Blog](https://www.philipdaniels.com/tags/rust/)
* [Tom McGurl/YouTube](https://www.youtube.com/c/TomMcGurl/videos)
* [Rust Programming Exercises/YouTube](https://www.youtube.com/playlist?list=PLb1VOxJqFzDdS-xV9OkKKPfXvtQ8y1Wzk)
* [Tensor Programming/Youtube](https://www.youtube.com/c/TensorProgramming/search)
* [CS Honors @ Illinois/YouTube](https://www.youtube.com/channel/UCRA18QWPzB7FYVyg0WFKC6g/search?query=rust)
* [Exercism supported Crates](https://github.com/exercism/rust-test-runner/blob/main/local-registry/supported_crates)

## License

Released under [Apache License v2.0](LICENSE).
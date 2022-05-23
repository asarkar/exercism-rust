# exercism-rust
[Rust track](https://exercism.org/tracks/rust)

[![](https://github.com/asarkar/exercism-rust/workflows/CI/badge.svg)](https://github.com/asarkar/exercism-rust/actions)

### zsh-compatible commands
```
for toml (**/Cargo.toml(N.)) cargo test --manifest-path $toml -- --ignored
for toml (**/Cargo.toml(N.)) cargo fmt --manifest-path $toml -- --check
for toml (**/Cargo.toml(N.)) cargo clippy --manifest-path $toml -- -D warnings
```

> To have rustfmt modify the files, use `-l` instead of `--check`.

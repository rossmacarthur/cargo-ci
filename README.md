# cargo-ci

[![Crates.io Version](https://img.shields.io/crates/v/cargo-ci.svg?style=flat-square)][crates]

Manages running common CI tasks with Cargo.

## Getting started

Install it using

```bash
cargo install cargo-ci
```

Then you can run cargo commands using

```
cargo ci --only nightly fmt
cargo ci --skip nightly clippy --all --all-targets --all-features -- -D warnings
cargo ci --skip nightly test
```

Arbitrary commands are also allowed, so something like this will work

```
cargo ci --only stable git diff --exit-code
```

## License

This project is dual licensed under the Apache 2.0 License and the MIT License.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for more
details.

[crates]: https://crates.io/crates/cargo-ci

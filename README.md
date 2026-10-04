# `simple-bitset` Rust Crate<br>[![Crates.io](https://img.shields.io/crates/v/simple-bitset.svg)](https://crates.io/crates/simple-bitset) [![Documentation](https://docs.rs/simple-bitset/badge.svg)](https://docs.rs/simple-bitset) [![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT) ![open source](https://badgen.net/badge/open/source/blue?icon=github)

Simple `no-std` compatible 64-bit and 128-bit bitsets for embedded applications".

1. `BitSet64` - for storing up to 64 bits.
2. `BitSet128` - for storing up to 128 bits.

`BitSet64` uses a `(u64)` singlet for its underlying storage.<br>
`BitSet128` uses a `(u64, u64)` duplet for its underlying storage.

This crate is `no_std`, `no alloc`, and the Minimum Supported Rust Version (MSRV) is `Rust 1.73`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

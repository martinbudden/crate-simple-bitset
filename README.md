# simple-bitset Crate ![license](https://img.shields.io/badge/license-MIT-green) [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) ![open source](https://badgen.net/badge/open/source/blue?icon=github)

Simple no-std compatible 64-bit and 128-bit bitsets for embedded applications".

1. `BitSet64` - for storing up to 64 bits.
2. `BitSet128` - for storing up to 128 bits.

`BitSet64` uses a `(u64)` singlet for its underlying storage.<br>
`BitSet128` uses a `(u64, u64)` duplet for its underlying storage.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

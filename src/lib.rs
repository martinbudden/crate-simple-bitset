#![doc = include_str!("../README.md")]
#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![warn(unused_results)]
#![warn(clippy::pedantic)]
#![warn(clippy::doc_paragraphs_missing_punctuation)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]

mod bitset128;
mod bitset64;

pub use bitset64::{BitSet64, BitSet64Iter};
pub use bitset128::{BitSet128, BitSet128Iter};

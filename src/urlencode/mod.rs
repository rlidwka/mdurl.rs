//! Percent-encoding and decoding that gracefully handles invalid input.
//!
//! **You should use [percent_encoding](https://crates.io/crates/percent-encoding) crate instead.**
//!
//! The purpose of this implementation is to keep user input intact in corner cases,
//! for example if url is already encoded, we try to keep it as is.
//!
//! You probably don't need it, so use crates designed to work with urlencoding specifically,
//! as they are better tested and maintained.
//!
//! This is not a part of public api, but still accessible. Please tell me
//! if someone finds it useful enough to make it public.
//!

mod asciiset;
pub use asciiset::AsciiSet;

mod decode;
pub use decode::*;

mod encode;
pub use encode::*;

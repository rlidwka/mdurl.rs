//! URL parsing utils that gracefully handle invalid input.
#![forbid(unsafe_code)]
#![warn(clippy::manual_assert)]
#![warn(clippy::semicolon_if_nothing_returned)]

mod asciiset;
pub use asciiset::AsciiSet;

mod encode;
pub use encode::percent_encode;

mod decode;
pub use decode::percent_decode;

mod parse;
pub use parse::parse_url;

mod url;
pub use url::Url;

mod format;
pub use format::format_url_for_computers;
pub use format::format_url_for_humans;

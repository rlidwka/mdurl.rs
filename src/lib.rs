//! URL parsing utils that gracefully handle invalid input.
#![forbid(unsafe_code)]
#![warn(clippy::manual_assert)]
#![warn(clippy::semicolon_if_nothing_returned)]

#[doc(hidden)]
// not part of official API, see comments in that module
pub mod urlencode;

mod parse;
pub use parse::parse_url;

mod url;
pub use url::Url;

mod format;
pub use format::format_url_for_computers;
pub use format::format_url_for_humans;

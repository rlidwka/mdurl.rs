//! URL parser and formatter that gracefully handles invalid input.
//!
//! This is a tool for pretty-printing user-supplied urls plus a
//! url parser that makes it possible.
//!
//! ```rust
//! use mdurl::format_url_for_humans as format;
//! let url = "https://www.reddit.com/r/programming/comments/vxttiq/\
//! comment/ifyqsqt/?utm_source=reddit&utm_medium=web2x&context=3";
//!
//! assert_eq!(format(url, 20), "reddit.com/…/ifyqsqt…");
//! assert_eq!(format(url, 30), "www.reddit.com/r/…/ifyqsqt/?ut…");
//! assert_eq!(format(url, 50), "www.reddit.com/r/programming/comments/…/ifyqsqt/?u…");
//! ```
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

# mdurl

[<img alt="web demo" src="https://img.shields.io/badge/demo-8da0cb?style=for-the-badge&labelColor=555555&logo=webpack&logoColor=white" height="20">](https://rlidwka.github.io/mdurl.rs/)
[<img alt="github" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rlidwka/mdurl.rs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs-8da0cb?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/mdurl)
[<img alt="crates.io" src="https://img.shields.io/crates/v/mdurl.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/mdurl)
[<img alt="coverage" src="https://img.shields.io/codecov/c/github/rlidwka/mdurl.rs?style=for-the-badge" height="20">](https://app.codecov.io/gh/rlidwka/mdurl.rs)

URL parser and formatter that gracefully handles invalid input.

It is a rust port of [mdurl.js](https://github.com/markdown-it/mdurl) library, created specifically
for url rendering in [markdown-it](https://github.com/rlidwka/markdown-it.rs) parser.

### URL formatter

This function takes URL, decodes it, and fits it into N characters, replacing the rest with
"…" symbol (that's called "url elision").

This is similar to what Chromium would show you in status bar when you hover your mouse over a link.

```rust
use mdurl::format_url_for_humans as format;
let url = "https://www.reddit.com/r/programming/comments/vxttiq/\
comment/ifyqsqt/?utm_source=reddit&utm_medium=web2x&context=3";

assert_eq!(format(url, 20), "reddit.com/…/ifyqsq…");
assert_eq!(format(url, 30), "www.reddit.com/r/…/ifyqsqt/?u…");
assert_eq!(format(url, 50), "www.reddit.com/r/programming/comments/…/ifyqsqt/?…");
```

Check out [this demo](https://rlidwka.github.io/mdurl.rs/) to play around with different URLs
and lengths.

[humanize-url](https://crates.io/crates/humanize-url) crate tries to achieve similar goals,
let me know if there are others.

### URL parser

In order to achieve the task above, a new url parser had to be created, so here it is:

```rust
let url = "https://www.reddit.com/r/programming/comments/vxttiq/\
comment/ifyqsqt/?utm_source=reddit&utm_medium=web2x&context=3";
let u = mdurl::parse_url(url);

assert_eq!(u.hostname, Some("www.reddit.com".into()));
assert_eq!(u.pathname, Some("/r/programming/comments/vxttiq/comment/ifyqsqt/".into()));
assert_eq!(u.search, Some("?utm_source=reddit&utm_medium=web2x&context=3".into()));
```

This function uses a non-standard parsing algorithm derived from node.js legacy URL parser.

You should probably be using [rust-url](https://crates.io/crates/url) crate instead.
Unfortunately, it isn't suitable for the task of pretty-printing urls because
you can't customize parts of Url returned by that library (for example, rust-url
will always encode non-ascii hostname with punycode, this implementation will not).

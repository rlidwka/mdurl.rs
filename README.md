# mdurl

[<img alt="web demo" src="https://img.shields.io/badge/demo-8da0cb?style=for-the-badge&labelColor=555555&logo=webpack&logoColor=white" height="20">](https://rlidwka.github.io/mdurl.rs/)
[<img alt="github" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rlidwka/mdurl.rs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs-8da0cb?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/mdurl)
[<img alt="crates.io" src="https://img.shields.io/crates/v/mdurl.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/mdurl)
[<img alt="coverage" src="https://img.shields.io/codecov/c/github/rlidwka/mdurl.rs?style=for-the-badge" height="20">](https://app.codecov.io/gh/rlidwka/mdurl.rs)

URL parser and formatter that gracefully handles invalid input.
Rust port of [mdurl.js](https://github.com/markdown-it/mdurl) library.

This is a tool for pretty-printing user-supplied urls plus a url parser that makes it possible.

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

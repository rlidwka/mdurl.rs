use once_cell::sync::Lazy;
use regex::Regex;
use crate::Url;
use crate::urlencode::{DECODE_DEFAULT_CHARS, ENCODE_DEFAULT_CHARS};

static HTTPS_OR_MAILTO : Lazy<Regex> = Lazy::new(||
    Regex::new("(?i)^(https?:|mailto:)$").unwrap()
);

static IP_HOST_CHECK : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"\.\d"#).unwrap()
);

/// Normalize and encode url.
///
///  - hostname is punycode-encoded and lowercased
///  - all parts of url that aren't already percent-encoded will be
///
pub fn format_url_for_computers(url: &str) -> String {
    let mut parsed = crate::parse_url(url);

    if let Some(protocol) = parsed.protocol.as_ref() {
        parsed.protocol = Some(protocol.to_ascii_lowercase());
    }

    if let Some(hostname) = parsed.hostname.as_ref() {
        // Encode hostnames in urls like:
        // `http://host/`, `https://host/`, `mailto:user@host`, `//host/`
        //
        // We don't encode unknown schemas, because it's likely that we encode
        // something we shouldn't (e.g. `skype:name` treated as `skype:host`)
        //
        if parsed.protocol.is_none() || HTTPS_OR_MAILTO.is_match(parsed.protocol.as_ref().unwrap()) {
            if let Ok(x) = idna::domain_to_ascii(hostname) {
                parsed.hostname = Some(x);
            }
        }
    }

    let encode = |s: String| {
        crate::urlencode::encode(&s, ENCODE_DEFAULT_CHARS, true).to_string()
    };

    parsed.auth = parsed.auth.map(encode);
    parsed.hash = parsed.hash.map(encode);
    parsed.search = parsed.search.map(encode);
    parsed.pathname = parsed.pathname.map(encode);

    parsed.to_string()
}


// if string char length > max then truncate string and add "..."
fn elide_text(mut text: String, max: usize) -> String {
    for (count, (offset, _)) in text.char_indices().enumerate() {
        if count + 1 >= max {
            text.truncate(offset);
            if !text.ends_with('…') {
                text.push('…');
            }
            break;
        }
    }
    text
}

fn elide_url(mut url: Url, max: usize) -> String {
    let mut url_str = url.to_string();
    let query_length = url.search.as_ref().map(|s| s.len()).unwrap_or_default() +
                       url.hash.as_ref().map(|s| s.len()).unwrap_or_default();

    // Maximum length of url without query+hash part
    //
    let max_path_length = max.saturating_add(query_length);
    let max_path_length = max_path_length.saturating_sub(2);

    // Here and below this `if` condition means:
    //
    // Assume that we can safely truncate querystring at anytime without
    // readability loss up to "?".
    //
    // So if url without hash/search fits, return it, eliding the end
    // e.g. "example.org/path/file?q=12345" -> "example.org/path/file?q=12..."
    //
    if url_str.chars().count() <= max_path_length {
        return elide_text(url_str, max);
    }

    // Try to elide path, e.g. "/foo/bar/baz/quux" -> "/foo/.../quux"
    //
    if let Some(pathname) = url.pathname.clone() {
        let mut components = pathname.split('/').collect::<Vec<_>>();
        let mut filename = components.pop().unwrap_or_default().to_owned();

        if filename.is_empty() && !components.is_empty() {
            filename = components.pop().unwrap().to_owned();
            filename.push('/');
        }

        while components.len() > 1 {
            components.pop();
            let new_pathname = format!("{}/…/{}", components.join("/"), filename);
            url.pathname = Some(new_pathname);
            url_str = url.to_string();

            if url_str.chars().count() <= max_path_length {
                return elide_text(url_str, max);
            }
        }
    }

    // Elide subdomains up to 2nd level,
    // e.g. "foo.bar.example.org" -> "...bar.example.org",
    //
    // Do NOT elide IP addresses here
    //
    if let Some(hostname) = url.hostname.clone() {
        if !IP_HOST_CHECK.is_match(&hostname) {
            let mut subdomains = hostname.split('.').collect::<Vec<_>>();
            let mut was_elided = false;

            // If it starts with "www", just remove it
            //
            if subdomains.first() == Some(&"www") && subdomains.len() > 2 {
                subdomains.remove(0);
                let new_hostname = subdomains.join(".");
                url.hostname = Some(new_hostname);
                url_str = url.to_string();

                if url_str.chars().count() <= max_path_length {
                    return elide_text(url_str, max);
                }
            }

            loop {
                // truncate up to 2nd level domain, e.g. `example.com`
                if subdomains.len() <= 2 { break; }

                // if 2nd level is short enough, truncate up to 3rd level, e.g. `example.co.uk`
                // (ideally, we'd use https://publicsuffix.org/list/public_suffix_list.dat,
                // but the list is too large)
                if subdomains.len() == 3 && subdomains.get(1).unwrap().len() < 3 {
                    break;
                }

                // if 3rd level is short enough (1-4 characters), keep it as is
                if !was_elided && subdomains.len() == 3 && subdomains.first().unwrap().len() <= 4 {
                    break;
                }

                subdomains.remove(0);
                let new_hostname = format!("…{}", subdomains.join("."));
                url.hostname = Some(new_hostname);
                url_str = url.to_string();
                was_elided = true;

                if url_str.chars().count() <= max_path_length {
                    return elide_text(url_str, max);
                }
            }
        }
    }

    elide_text(url_str, max)
}


/// Pretty-print url and fit it into N characters (url elision).
///
/// Result of this function is intended to be viewed by humans only,
/// and it's not guaranteed to stay a valid url anymore.
///
/// This function takes `max_length` argument, which is maximum allowed
/// character count for this url. If `url` is longer than this, less
/// relevant parts of it will be replaced with `…` character.
/// Use `usize::MAX` to disable this feature.
///
/// The elision algorithm is similar to one used in chromium:
/// <https://chromium.googlesource.com/chromium/src/+/refs/heads/main/components/url_formatter/elide_url.cc>
///
/// It reads as follows:
///
///  1. Chop off path, e.g.
///
///     "/foo/bar/baz/quux" -> "/foo/bar/…/quux" -> "/foo/…/quux" -> "/…/quux"
///
///  2. Get rid of 2+ level subdomains, e.g.
///
///     "foo.bar.baz.example.com" -> "…bar.baz.example.com" ->
///     "…baz.example.com" -> "…example.com"
///
///     Exception 1: if 2nd level domain is 1-3 letters, truncate to 3rd level:
///
///     "foo.bar.baz.co.uk" -> ... -> "…baz.co.uk"
///
///     Exception 2: don't change if it is 3rd level domain which has short (1-4 characters) 3rd level
///     "foo.example.org" -> "foo.example.org"
///     "bar.foo.example.org" -> "…example.org"
///
///  3. Truncate the rest of the url if needed
///
pub fn format_url_for_humans(url: &str, max_length: usize) -> String {
    //if max_length == 0 { max_length = usize::MAX; }
    let mut parsed = crate::parse_url(url);
    let url_with_slashes;

    // urls without host and protocol, e.g. "example.org/foo"
    if parsed.protocol.is_none() && !parsed.slashes && parsed.hostname.is_none() {
        url_with_slashes = format!("//{url}");
        parsed = crate::parse_url(&url_with_slashes);
    }

    if let Some(hostname) = parsed.hostname.as_ref() {
        // Encode hostnames in urls like:
        // `http://host/`, `https://host/`, `mailto:user@host`, `//host/`
        //
        // We don't encode unknown schemas, because it's likely that we encode
        // something we shouldn't (e.g. `skype:name` treated as `skype:host`)
        //
        #[allow(clippy::collapsible_if)]
        if parsed.protocol.is_none() || HTTPS_OR_MAILTO.is_match(parsed.protocol.as_ref().unwrap()) {
            if hostname.starts_with("xn--") {
                let (x, _) = idna::domain_to_unicode(hostname);
                parsed.hostname = Some(x);
            }
        }
    }

    let decode = |s: String| {
        // Decode url-encoded characters
        //
        // add '%' to exclude list because of https://github.com/markdown-it/markdown-it/issues/720
        crate::urlencode::decode(&s, DECODE_DEFAULT_CHARS.add(b'%')).to_string()
    };

    parsed.auth = parsed.auth.map(decode);
    parsed.hash = parsed.hash.map(decode);
    parsed.search = parsed.search.map(decode);
    parsed.pathname = parsed.pathname.map(decode);


    // Remove trailing slash: http://example.org/ → http://example.org
    //
    if let Some(pathname) = parsed.pathname.as_ref() {
        if pathname == "/" && parsed.search.is_none() && parsed.hash.is_none() {
            parsed.pathname = Some(String::new());
        }
    }

    // Omit protocol if it's http, https or mailto
    //
    if parsed.protocol.is_some() {
        if HTTPS_OR_MAILTO.is_match(parsed.protocol.as_ref().unwrap()) {
            parsed.protocol = None;
            parsed.slashes = false;
        }
    } else {
        parsed.slashes = false;
    }

    elide_url(parsed, max_length)
}


#[cfg(test)]
mod tests {
    use super::*;

    mod format_url_for_computers {
        use super::*;

        #[test]
        fn encode_should_punycode_domains() {
            let source = "https://ουτοπία.δπθ.gr/";
            let expected = "https://xn--kxae4bafwg.xn--pxaix.gr/";
            assert_eq!(format_url_for_computers(source), expected);
        }

        #[test]
        fn encode_should_lowercase_protocol_and_domain() {
            let source = "HTTP://GOOGLE.COM/";
            let expected = "http://google.com/";
            assert_eq!(format_url_for_computers(source), expected);
        }

        #[test]
        fn encode_should_urlencode_nonascii_parts() {
            let source = "http://example.org/✔️?q=❤️";
            let expected = "http://example.org/%E2%9C%94%EF%B8%8F?q=%E2%9D%A4%EF%B8%8F";
            assert_eq!(format_url_for_computers(source), expected);
        }

        #[test]
        fn encode_should_skip_already_encoded_sequences() {
            let source = "http://example.org/%20%25";
            let expected = "http://example.org/%20%25";
            assert_eq!(format_url_for_computers(source), expected);
        }
    }

    mod format_url_for_humans {
        use super::*;

        #[test]
        fn should_decode_punycode_domains() {
            let source = "https://xn--kxae4bafwg.xn--pxaix.gr/";
            let expected = "ουτοπία.δπθ.gr";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }

        #[test]
        fn should_keep_uppercase_domain() {
            let source = "HTTP://GOOGLE.COM/";
            let expected = "GOOGLE.COM";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }

        #[test]
        fn should_keep_uppercase_protocol() {
            let source = "JAVASCRIPT:alert(1)";
            let expected = "JAVASCRIPT:alert(1)";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }

        #[test]
        fn encode_should_urlencode_nonascii_parts() {
            let source = "http://example.org/%E2%9C%94%EF%B8%8F?q=%E2%9D%A4%EF%B8%8F";
            let expected = "example.org/✔️?q=❤️";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }

        #[test]
        fn should_keep_encoded_sequences() {
            // https://github.com/markdown-it/markdown-it/issues/720
            let source = "https://www.google.com/search?q=hello%252Fhello";
            let expected = "www.google.com/search?q=hello%252Fhello";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }

        #[test]
        fn url_without_protocol_slashes() {
            let source = "//www.google.com/foobar";
            let expected = "www.google.com/foobar";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }

        #[test]
        fn url_without_protocol_no_slashes() {
            let source = "www.google.com/foobar";
            let expected = "www.google.com/foobar";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }

        #[test]
        fn should_omit_mailto() {
            let source = "mailto:foo@example.org";
            let expected = "foo@example.org";
            assert_eq!(format_url_for_humans(source, usize::MAX), expected);
        }
    }

    mod elide_url {
        use super::*;

        #[test]
        fn should_truncate_domains() {
            let source = "https://whatever.example.com/foobarbazquux?query=string";
            let expected = "…example.com/foobarb…";
            assert_eq!(format_url_for_humans(source, 21), expected);
        }

        #[test]
        fn should_show_common_2nd_level_domains() {
            let source = "https://whatever.example.co.uk/foobarbazquux?query=string";
            let expected = "…example.co.uk/fooba…";
            assert_eq!(format_url_for_humans(source, 21), expected);
        }

        #[test]
        fn should_show_4_letter_3rd_level_domains() {
            let source = "https://blog.chromium.org/2019/10/no-more-mixed-messages-about-https.html";
            let expected = "blog.chromium.org/…/no-more-mixed-messag…";
            assert_eq!(format_url_for_humans(source, 41), expected);
        }

        #[test]
        fn should_work_with_0_or_1_char() {
            let source = "https://blog.chromium.org/2019/10/no-more-mixed-messages-about-https.html";
            assert_eq!(format_url_for_humans(source, 0), "…");
            assert_eq!(format_url_for_humans(source, 1), "…");
        }

        #[test]
        fn should_elide_middle_of_the_path() {
            let source = "https://www.reddit.com/r/programming/comments/vxttiq/comment/ifyqsqt/?utm_source=reddit&utm_medium=web2x&context=3";
            let expected = "www.reddit.com/r/programming/…/ifyqsqt/?u…";
            assert_eq!(format_url_for_humans(source, 42), expected);
        }

        #[test]
        fn should_elide_if_path_ends_with_slash() {
            let source = "https://example.org/foo/bar/baz/";
            let expected = "example.org/…/baz/";
            assert_eq!(format_url_for_humans(source, 23), expected);
        }

        #[test]
        fn should_not_have_consecutive_elides() {
            let source = "https://example.org/foo/bar/baz/";
            let expected = "example.org/…";
            assert_eq!(format_url_for_humans(source, 14), expected);
        }

        #[test]
        fn should_elide_domains_from_the_front() {
            let source = "https://foo.bar.baz.example.org";
            let expected = "…baz.example.org";
            assert_eq!(format_url_for_humans(source, 20), expected);
        }

        #[test]
        fn should_elide_ip_addresses_from_the_back() {
            let source = "https://127.123.123.234/";
            let expected = "127.123.12…";
            assert_eq!(format_url_for_humans(source, 11), expected);
        }

        #[test]
        fn remove_www_without_eliding() {
            let source = "https://www.google.com/foobar";
            let expected = "google.com/foob…";
            assert_eq!(format_url_for_humans(source, 16), expected);
        }

        #[test]
        fn remove_www_without_eliding2() {
            let source = "https://www.google.com";
            let expected = "google.com";
            assert_eq!(format_url_for_humans(source, 13), expected);
        }

        #[test]
        fn second_level_www() {
            let source = "https://www.com/foobar";
            let expected = "www.com/…";
            assert_eq!(format_url_for_humans(source, 9), expected);
        }
    }
}

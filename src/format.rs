use once_cell::sync::Lazy;
use regex::Regex;
use crate::{AsciiSet, Url};

static HTTPS_OR_MAILTO : Lazy<Regex> = Lazy::new(||
    Regex::new("(?i)^(https?:|mailto:)$").unwrap()
);

static IP_HOST_CHECK : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"\.\d"#).unwrap()
);

// Decode hostname/path and trim url
//  - url_str    - url to decode
//  - max_length - maximum allowed length for this url
//
pub fn format_url_for_computers(url: &str) -> String {
    let mut parsed = crate::parse_url(url, true);

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
        const SET : AsciiSet = crate::AsciiSet::from(";/?:@&=+$,-_.!~*'()#");
        crate::percent_encode(&s, SET, true).to_string()
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
        if count >= max {
            text.truncate(offset);
            text.push('…');
            break;
        }
    }
    text
}


// Replace long parts of the urls with elisions.
//
// This algorithm is similar to one used in chromium:
// https://chromium.googlesource.com/chromium/src.git/+/master/chrome/browser/ui/elide_url.cc
//
//  1. Chop off path, e.g.
//
//     "/foo/bar/baz/quux" -> "/foo/bar/…/quux" -> "/foo/…/quux" -> "/…/quux"
//
//  2. Get rid of 2+ level subdomains, e.g.
//
//     "foo.bar.baz.example.com" -> "…bar.baz.example.com" ->
//     "…baz.example.com" -> "…example.com"
//
//     Exception 1: if 2nd level domain is 1-3 letters, truncate to 3rd level:
//
//     "foo.bar.baz.co.uk" -> ... -> "…baz.co.uk"
//
//     Exception 2: don't change if it is 3rd level domain which has short (1-4 characters) 3rd level
//     "foo.example.org" -> "foo.example.org"
//     "bar.foo.example.org" -> "…example.org"
//
//  3. Truncate the rest of the url
//
// If at any point of the time url becomes small enough, return it
//
fn elide_url(mut url: Url, max: usize) -> String {
    let mut url_str = url.to_string();
    let query_length = url.search.as_ref().map(|s| s.len()).unwrap_or_default() +
                       url.hash.as_ref().map(|s| s.len()).unwrap_or_default();

    // Maximum length of url without query+hash part
    //
    let max_path_length = max + query_length;
    let max_path_length = if max_path_length < 2 { 0 } else { max_path_length - 2 };

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


// Decode hostname/path and trim url
//  - url        - url to decode
//  - max_length - maximum allowed length for this url
//
pub fn format_url_for_humans(url: &str, max_length: usize) -> String {
    //if max_length == 0 { max_length = usize::MAX; }
    let mut parsed = crate::parse_url(url, true);
    let url_with_slashes;

    // urls without host and protocol, e.g. "example.org/foo"
    if parsed.protocol.is_none() && !parsed.slashes && parsed.hostname.is_none() {
        url_with_slashes = format!("//{url}");
        parsed = crate::parse_url(&url_with_slashes, true);
    }

    if let Some(hostname) = parsed.hostname.as_ref() {
        // Encode hostnames in urls like:
        // `http://host/`, `https://host/`, `mailto:user@host`, `//host/`
        //
        // We don't encode unknown schemas, because it's likely that we encode
        // something we shouldn't (e.g. `skype:name` treated as `skype:host`)
        //
        if parsed.protocol.is_none() || HTTPS_OR_MAILTO.is_match(parsed.protocol.as_ref().unwrap()) {
            let (x, _) = idna::domain_to_unicode(hostname);
            parsed.hostname = Some(x);
        }
    }

    let decode = |s: String| {
        // Decode url-encoded characters
        //
        // add '%' to exclude list because of https://github.com/markdown-it/markdown-it/issues/720
        const SET : AsciiSet = crate::AsciiSet::from(";/?:@&=+$,#").add(b'%');
        crate::percent_decode(&s, SET).to_string()
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
    use super::format_url_for_humans;

    #[test]
    fn should_truncate_domains() {
        let source = "https://whatever.example.com/foobarbazquux?query=string";
        let expected = "…example.com/foobarb…";
        assert_eq!(format_url_for_humans(source, 20), expected);
    }

    #[test]
    fn should_show_common_2nd_level_domains() {
        let source = "https://whatever.example.co.uk/foobarbazquux?query=string";
        let expected = "…example.co.uk/fooba…";
        assert_eq!(format_url_for_humans(source, 20), expected);
    }

    #[test]
    fn should_show_4_letter_3rd_level_domains() {
        let source = "https://blog.chromium.org/2019/10/no-more-mixed-messages-about-https.html";
        let expected = "blog.chromium.org/…/no-more-mixed-messag…";
        assert_eq!(format_url_for_humans(source, 40), expected);
    }
}

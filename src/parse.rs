// Copyright Joyent, Inc. and other Node contributors.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit
// persons to whom the Software is furnished to do so, subject to the
// following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
// NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.


//
// Changes from joyent/node:
//
// 1. No leading slash in paths,
//    e.g. in `url.parse('http://foo?bar')` pathname is ``, not `/`
//
// 2. Backslashes are not replaced with slashes,
//    so `http:\\example.org\` is treated like a relative path
//
// 3. Trailing colon is treated like a part of the path,
//    i.e. in `http://example.org:foo` pathname is `:foo`
//
// 4. Nothing is URL-encoded in the resulting object,
//    (in joyent/node some chars in auth and paths are encoded)
//
// 5. `url.parse()` does not have `parseQueryString` argument
//
// 6. Removed extraneous result properties: `host`, `path`, `query`, etc.,
//    which can be constructed using other parts of the url.
//

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use crate::Url;

// Reference: RFC 3986, RFC 1808, RFC 2396

// define these here so at least they only have to be
// compiled once on the first module load.
static PROTOCOL_PATTERN : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"(?i)^([a-z0-9.+-]+:)"#).unwrap()
);

static PORT_PATTERN : Lazy<Regex> = Lazy::new(||
    Regex::new(r#":[0-9]*$"#).unwrap()
);

static HOST_PATTERN : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"^//[^@/]+@[^@/]+"#).unwrap()
);

// Special case for a simple path URL
static SIMPLE_PATH_PATTERN : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"^(//?[^/\?\s]?[^\?\s]*)(\?[^\s]*)?$"#).unwrap()
);

const NON_HOST_CHARS : [ char; 8 + 6 + 1 + 5 ] = [
    // RFC 2396: characters reserved for delimiting URLs.
    // We actually just auto-escape these.
    '<', '>', '"', '`', ' ', '\r', '\n', '\t', // DELIMS

    // RFC 2396: characters not allowed for various reasons.
    '{', '}', '|', '\\', '^', '`', // UNWISE

    // Allowed by RFCs, but cause of XSS attacks.  Always escape these.
    '\'', // AUTO_ESCAPE

    // Characters that are never ever allowed in a hostname.
    // Note that any invalid chars are also handled, but these
    // are the ones that are *expected* to be seen, so we fast-path
    // them.
    '%', '/', '?', ';', '#', // NON_HOST_CHARS
];

const HOST_ENDING_CHARS : [ char; 3 ] = [ '/', '?', '#' ];

const HOSTNAME_MAX_LEN : usize = 255;

static HOSTNAME_PART_PATTERN : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"^[+a-z0-9A-Z_-]{0,63}$"#).unwrap()
);

static HOSTNAME_PART_START : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"^([+a-z0-9A-Z_-]{0,63})(.*)$"#).unwrap()
);

// protocols that can allow "unsafe" and "unwise" chars.
// protocols that never have a hostname.
static HOSTLESS_PROTOCOL : Lazy<HashSet<&'static str>> = Lazy::new(||
    HashSet::from_iter([
        "javascript",
        "javascript:",
    ].iter().copied())
);

// protocols that always contain a // bit.
static SLASHED_PROTOCOL : Lazy<HashSet<&'static str>> = Lazy::new(||
    HashSet::from_iter([
        "http",
        "https",
        "ftp",
        "gopher",
        "file",
        "http:",
        "https:",
        "ftp:",
        "gopher:",
        "file:",
    ].iter().copied())
);


/// Parse URL string and return a [Url] object.
///
/// This function uses a non-standard parsing algorithm derived from node.js
/// legacy URL parser.
///
/// There are three major differences from [url](https://crates.io/crates/url) crate:
///
///  - It returns `Url`, not `Result<Url>`. We try our best to parse any urls,
///    but given nonsensical input it may produce nonsensical output.
///
///  - No normalization of any kind is provided, and no information is lost during
///    parsing. For example, hostname may be lowercase or uppercase, punycoded or not.
///    It is up to you to normalize these.
///
///  - You can freely modify resulting `Url` object, setting any values that you want.
///    `Url::to_string()` will reflect your changes as is.
///
pub fn parse_url(url: &str) -> Url {
    // - slashes_denote_host - If `true`, the first token after the literal
    //   string `//` and preceding the next `/` will be interpreted as the `host`.
    //   For instance, given `//foo/bar`, the result would be
    //   `{host: 'foo', pathname: '/bar'}` rather than `{pathname: '//foo/bar'}`.
    //
    // Rust doesn't allow optional arguments, so this option in original algoritm
    // was really getting in the way. Tell me if it is a useful option to have.
    //
    const SLASHES_DENOTE_HOST: bool = true;

    let mut this = Url::default();
    let mut rest = url;

    // trim before proceeding.
    // This is to support parse stuff like "  http://foo.com  \n"
    rest = rest.trim();

    if !SLASHES_DENOTE_HOST && !url.contains('#') {
        // Try fast path regexp
        if let Some(simple_path) = SIMPLE_PATH_PATTERN.captures(rest) {
            this.pathname = Some(simple_path.get(1).unwrap().as_str().into());
            this.search = simple_path.get(2).map(|x| x.as_str().into());
            return this;
        }
    }

    if let Some(proto_match) = PROTOCOL_PATTERN.captures(rest) {
        let proto = Some(proto_match.get(0).unwrap().as_str());
        //let lower_proto = proto.map(|s| s.to_ascii_lowercase());
        this.protocol = proto.map(|s| s.into());
        rest = &rest[proto.unwrap().len()..];
    }

    // figure out if it's got a host
    // user@server is *always* interpreted as a hostname, and url
    // resolution will treat //foo/bar as host=foo,path=bar because that's
    // how the browser resolves relative URLs.
    if SLASHES_DENOTE_HOST || this.protocol.is_some() || HOST_PATTERN.is_match(rest) {
        let slashes = rest.starts_with("//");
        if slashes && !(this.protocol.is_some() && HOSTLESS_PROTOCOL.contains(this.protocol.as_ref().unwrap().as_str())) {
            rest = &rest[2..];
            this.slashes = true;
        }
    }

    if (this.protocol.is_none() || !HOSTLESS_PROTOCOL.contains(this.protocol.as_ref().unwrap().as_str())) &&
        (this.slashes || (this.protocol.is_some() && !SLASHED_PROTOCOL.contains(this.protocol.as_ref().unwrap().as_str()))) {

        // there's a hostname.
        // the first instance of /, ?, ;, or # ends the host.
        //
        // If there is an @ in the hostname, then non-host chars *are* allowed
        // to the left of the last @ sign, unless some host-ending character
        // comes *before* the @-sign.
        // URLs are obnoxious.
        //
        // ex:
        // http://a@b@c/ => user:a@b host:c
        // http://a@b?@c => user:a host:c path:/?@c

        // v0.12 TODO(isaacs): This is not quite how Chrome does things.
        // Review our test case against browsers more comprehensively.

        // find the first instance of any hostEndingChars
        let host_end = rest.find(HOST_ENDING_CHARS);

        // at this point, either we have an explicit point where the
        // auth portion cannot go past, or the last @ char is the decider.
        let at_sign = if let Some(host_end) = host_end {
            // atSign must be in auth portion.
            // http://a@b/c@d => host:b auth:a path:/c@d
            rest[..host_end].rfind('@')
        } else {
            // atSign can be anywhere.
            rest.rfind('@')
        };

        // Now we have a portion which is definitely the auth.
        // Pull that off.
        if let Some(at_sign) = at_sign {
            this.auth = Some(rest[..at_sign].into());
            rest = &rest[at_sign+1..];
        }

        // the host is the remaining to the left of the first non-host char
        let host_end = rest.find(NON_HOST_CHARS);
        // if we still have not hit it, then the entire thing is a host.
        let mut host_end = host_end.unwrap_or(rest.len());

        if rest[..host_end].ends_with(':') { host_end -= 1; }
        let mut host = &rest[..host_end];
        rest = &rest[host_end..];

        // pull out port.
        if let Some(port_match) = PORT_PATTERN.captures(host) {
            let port = port_match.get(0).unwrap().as_str();
            if port != ":" {
                this.port = Some(port[1..].into());
            }
            host = &host[..host.len()-port.len()];
        }
        this.hostname = Some(host.into());

        // if hostname begins with [ and ends with ]
        // assume that it's an IPv6 address.
        let check_hostname = this.hostname.as_ref().unwrap().as_str();
        let ipv6_hostname = check_hostname.starts_with('[') && check_hostname.ends_with(']');

        // validate a little.
        if !ipv6_hostname {
            let hostparts = this.hostname.as_ref().unwrap().split('.').collect::<Vec<_>>();
            for (i, part) in hostparts.iter().enumerate() {
                if part.is_empty() { continue; }
                if !HOSTNAME_PART_PATTERN.is_match(part) {
                    // we replace non-ASCII char with a temporary placeholder
                    // we need this to make sure size of hostname is not
                    // broken by replacing non-ASCII by nothing
                    let newpart = part.chars()
                        .map(|c| if c as u32 > 127 { 'x' } else { c })
                        .collect::<String>();
                    // we test again with ASCII char only
                    if !HOSTNAME_PART_PATTERN.is_match(&newpart) {
                        let mut valid_parts = hostparts[..i].to_vec();
                        let mut not_host = hostparts[i+1..].to_vec();
                        if let Some(bit) = HOSTNAME_PART_START.captures(part) {
                            valid_parts.push(bit.get(1).unwrap().as_str());
                            not_host.push(bit.get(2).unwrap().as_str());
                        }
                        if !not_host.is_empty() {
                            // same as:
                            //rest = not_host.join(".") + rest;
                            rest = &url[url.len()-rest.len()-not_host.join(".").len()..];
                        }
                        this.hostname = Some(valid_parts.join("."));
                        //this.hostname = Some(&url[url.len()-rest.len()-valid_parts.join(".").len()..url.len()-rest.len()]);
                        break;
                    }
                }
            }
        }

        if this.hostname.as_ref().unwrap().len() > HOSTNAME_MAX_LEN {
            this.hostname = Some(String::new());
        }

        // strip [ and ] from the hostname
        // the host field still retains them, though
        if ipv6_hostname {
            let hostname = this.hostname.as_ref().unwrap().as_str();
            this.hostname = Some(hostname[1..hostname.len()-1].into());
        }
    }

    // chop off from the tail first.
    if let Some(hash) = rest.find('#') {
        // got a fragment string.
        this.hash = Some(rest[hash..].into());
        rest = &rest[0..hash];
    }
    if let Some(qm) = rest.find('?') {
        this.search = Some(rest[qm..].into());
        rest = &rest[0..qm];
    }
    if !rest.is_empty() {
        this.pathname = Some(rest.into());
    }
    if this.protocol.is_some() &&
            SLASHED_PROTOCOL.contains(this.protocol.as_ref().unwrap().to_ascii_lowercase().as_str()) &&
            this.hostname.is_some() && !this.hostname.as_ref().unwrap().is_empty() &&
            this.pathname.is_none() {
        this.pathname = Some(String::new());
    }

    this
}

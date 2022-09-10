use crate::Url;

/// Return a formatted URL string derived from [Url] object.
///
/// It simply concatenates whatever is in the input, and does no validation
/// or escaping of any kind.
///
/// Round-trip is guaranteed, so `format(parse(str))` always equals to `str`,
/// but if you write malformed data to `url`, you may get broken url as the output.
///
pub fn format(url: Url) -> String {
    let mut result = String::new();

    if let Some(s) = url.protocol {
        result.push_str(s);
    }

    if url.slashes {
        result.push_str("//");
    }

    if let Some(s) = url.auth {
        result.push_str(s);
        result.push('@');
    }

    if let Some(s) = url.hostname {
        if s.contains(':') {
            // ipv6 address
            result.push('[');
            result.push_str(s);
            result.push(']');
        } else {
            result.push_str(s);
        }
    }

    if let Some(s) = url.port {
        result.push(':');
        result.push_str(s);
    }

    if let Some(s) = url.pathname {
        result.push_str(s);
    }

    if let Some(s) = url.search {
        result.push_str(s);
    }

    if let Some(s) = url.hash {
        result.push_str(s);
    }

    result
}


#[cfg(test)]
mod tests {
    use crate::parse;
    use super::format;

    const FIXTURES : [ &str; 87 ] = [
        "//some_path",
        "HTTP://www.example.com/",
        "HTTP://www.example.com",
        "http://www.ExAmPlE.com/",
        "http://user:pw@www.ExAmPlE.com/",
        "http://USER:PW@www.ExAmPlE.com/",
        "http://user@www.example.com/",
        "http://user%3Apw@www.example.com/",
        "http://x.com/path?that\'s#all, folks",
        "HTTP://X.COM/Y",
        "http://x.y.com+a/b/c",
        "HtTp://x.y.cOm;a/b/c?d=e#f g<h>i",
        "HtTp://x.y.cOm;A/b/c?d=e#f g<h>i",
        "http://x...y...#p",
        "http://x/p/\"quoted\"",
        "<http://goo.corn/bread> Is a URL!",
        "http://www.narwhaljs.org/blog/categories?id=news",
        "http://mt0.google.com/vt/lyrs=m@114&hl=en&src=api&x=2&y=2&z=3&s=",
        "http://mt0.google.com/vt/lyrs=m@114???&hl=en&src=api&x=2&y=2&z=3&s=",
        "http://user:pass@mt0.google.com/vt/lyrs=m@114???&hl=en&src=api&x=2&y=2&z=3&s=",
        "file:///etc/passwd",
        "file://localhost/etc/passwd",
        "file://foo/etc/passwd",
        "file:///etc/node/",
        "file://localhost/etc/node/",
        "file://foo/etc/node/",
        "http:/baz/../foo/bar",
        "http://user:pass@example.com:8000/foo/bar?baz=quux#frag",
        "//user:pass@example.com:8000/foo/bar?baz=quux#frag",
        "/foo/bar?baz=quux#frag",
        "http:/foo/bar?baz=quux#frag",
        "mailto:foo@bar.com?subject=hello",
        "javascript:alert(\'hello\');",
        "xmpp:isaacschlueter@jabber.org",
        "http://atpass:foo%40bar@127.0.0.1:8080/path?search=foo#bar",
        "svn+ssh://foo/bar",
        "dash-test://foo/bar",
        "dash-test:foo/bar",
        "dot.test://foo/bar",
        "dot.test:foo/bar",
        "http://www.日本語.com/",
        "http://example.Bücher.com/",
        "http://www.Äffchen.com/",
        "http://www.Äffchen.cOm;A/b/c?d=e#f g<h>i",
        "http://SÉLIER.COM/",
        "http://ﻞﻴﻬﻣﺎﺒﺘﻜﻠﻣﻮﺸﻋﺮﺒﻳ؟.ﻱ؟/",
        "http://➡.ws/➡",
        "http://bucket_name.s3.amazonaws.com/image.jpg",
        "git+http://github.com/joyent/node.git",
        "local1@domain1",
        "www.example.com",
        "[fe80::1]",
        "coap://[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]",
        "coap://[1080:0:0:0:8:800:200C:417A]:61616/",
        "http://user:password@[3ffe:2a00:100:7031::1]:8080",
        "coap://u:p@[::192.9.5.5]:61616/.well-known/r?n=Temperature",
        "http://example.com:",
        "http://example.com:/a/b.html",
        "http://example.com:?a=b",
        "http://example.com:#abc",
        "http://[fe80::1]:/a/b?a=b#abc",
        "http://-lovemonsterz.tumblr.com/rss",
        "http://-lovemonsterz.tumblr.com:80/rss",
        "http://user:pass@-lovemonsterz.tumblr.com/rss",
        "http://user:pass@-lovemonsterz.tumblr.com:80/rss",
        "http://_jabber._tcp.google.com/test",
        "http://user:pass@_jabber._tcp.google.com/test",
        "http://_jabber._tcp.google.com:80/test",
        "http://user:pass@_jabber._tcp.google.com:80/test",
        "http://x:1/' <>\"`/{}|\\^~`/",
        "http://a@b@c/",
        "http://a@b?@c",
        "http://a\r\" \t\n<'b:b@c\r\nd/e?f",
        "git+ssh://git@github.com:npm/npm",
        "http://example.com?foo=bar#frag",
        "http://example.com?foo=@bar#frag",
        "http://example.com?foo=/bar/#frag",
        "http://example.com?foo=?bar/#frag",
        "http://example.com#frag=?bar/#frag",
        "http://google.com\" onload=\"alert(42)/",
        "http://a.com/a/b/c?s#h",
        "http://atpass:foo%40bar@127.0.0.1/",
        "http://atslash%2F%40:%2F%40@foo/",
        "coap:u:p@[::1]:61616/.well-known/r?n=Temperature",
        "coap:[fedc:ba98:7654:3210:fedc:ba98:7654:3210]:61616/s/stopButton",
        "http://ex.com/foo%3F100%m%23r?abc=the%231?&foo=bar#frag",
        "http://ex.com/fooA100%mBr?abc=the%231?&foo=bar#frag",
    ];

    #[test]
    fn round_trip() {
        for str in FIXTURES {
            let url = parse(str, false);
            assert_eq!(format(url), str);
        }
    }
}

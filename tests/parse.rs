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

#![allow(clippy::needless_update)]
use mdurl::Url;
use mdurl::parse;

#[test]
fn simple_path() {
    assert_eq!(
        parse("//some_path", false),
        Url {
            pathname: Some("//some_path"),
            ..Default::default()
        }
    );
}

#[test]
fn test1() {
    assert_eq!(
        parse("HTTP://www.example.com/", false),
        Url {
            protocol: Some("HTTP:"),
            slashes: true,
            hostname: Some("www.example.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn test2() {
    assert_eq!(
        parse("HTTP://www.example.com", false),
        Url {
            protocol: Some("HTTP:"),
            slashes: true,
            hostname: Some("www.example.com"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn test3() {
    assert_eq!(
        parse("http://www.ExAmPlE.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("www.ExAmPlE.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn testpw1() {
    assert_eq!(
        parse("http://user:pw@www.ExAmPlE.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user:pw"),
            hostname: Some("www.ExAmPlE.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn testpw2() {
    assert_eq!(
        parse("http://USER:PW@www.ExAmPlE.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("USER:PW"),
            hostname: Some("www.ExAmPlE.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn testauth() {
    assert_eq!(
        parse("http://user@www.example.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user"),
            hostname: Some("www.example.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn testauth3a() {
    assert_eq!(
        parse("http://user%3Apw@www.example.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user%3Apw"),
            hostname: Some("www.example.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn thatsall() {
    assert_eq!(
        parse("http://x.com/path?that\'s#all, folks", false),
        Url {
            protocol: Some("http:"),
            hostname: Some("x.com"),
            slashes: true,
            search: Some("?that\'s"),
            pathname: Some("/path"),
            hash: Some("#all, folks"),
            ..Default::default()
        }
    );
}

#[test]
fn testupper() {
    assert_eq!(
        parse("HTTP://X.COM/Y", false),
        Url {
            protocol: Some("HTTP:"),
            slashes: true,
            hostname: Some("X.COM"),
            pathname: Some("/Y"),
            ..Default::default()
        }
    );
}

#[test]
// + not an invalid host character
// per https://url.spec.whatwg.org/#host-parsing
fn testplus() {
    assert_eq!(
        parse("http://x.y.com+a/b/c", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("x.y.com+a"),
            pathname: Some("/b/c"),
            ..Default::default()
        }
    );
}

#[test]
// an unexpected invalid char in the hostname.
fn invalid_char_in_hostname() {
    assert_eq!(
        parse("HtTp://x.y.cOm;a/b/c?d=e#f g<h>i", false),
        Url {
            protocol: Some("HtTp:"),
            slashes: true,
            hostname: Some("x.y.cOm"),
            pathname: Some(";a/b/c"),
            search: Some("?d=e"),
            hash: Some("#f g<h>i"),
            ..Default::default()
        }
    );
}

#[test]
// make sure that we don't accidentally lcast the path parts.
fn testlcast() {
    assert_eq!(
        parse("HtTp://x.y.cOm;A/b/c?d=e#f g<h>i", false),
        Url {
            protocol: Some("HtTp:"),
            slashes: true,
            hostname: Some("x.y.cOm"),
            pathname: Some(";A/b/c"),
            search: Some("?d=e"),
            hash: Some("#f g<h>i"),
            ..Default::default()
        }
    );
}

#[test]
fn testdots() {
    assert_eq!(
        parse("http://x...y...#p", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("x...y..."),
            hash: Some("#p"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn testquoted() {
    assert_eq!(
        parse("http://x/p/\"quoted\"", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("x"),
            pathname: Some("/p/\"quoted\""),
            ..Default::default()
        }
    );
}

#[test]
fn testangled() {
    assert_eq!(
        parse("<http://goo.corn/bread> Is a URL!", false),
        Url {
            pathname: Some("<http://goo.corn/bread> Is a URL!"),
            ..Default::default()
        }
    );
}

#[test]
fn testnarwhaljs() {
    assert_eq!(
        parse("http://www.narwhaljs.org/blog/categories?id=news", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("www.narwhaljs.org"),
            search: Some("?id=news"),
            pathname: Some("/blog/categories"),
            ..Default::default()
        }
    );
}

#[test]
fn testgoog1() {
    assert_eq!(
        parse("http://mt0.google.com/vt/lyrs=m@114&hl=en&src=api&x=2&y=2&z=3&s=", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("mt0.google.com"),
            pathname: Some("/vt/lyrs=m@114&hl=en&src=api&x=2&y=2&z=3&s="),
            ..Default::default()
        }
    );
}

#[test]
fn testgoog2() {
    assert_eq!(
        parse("http://mt0.google.com/vt/lyrs=m@114???&hl=en&src=api&x=2&y=2&z=3&s=", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("mt0.google.com"),
            search: Some("???&hl=en&src=api&x=2&y=2&z=3&s="),
            pathname: Some("/vt/lyrs=m@114"),
            ..Default::default()
        }
    );
}

#[test]
fn testgoog3() {
    assert_eq!(
        parse("http://user:pass@mt0.google.com/vt/lyrs=m@114???&hl=en&src=api&x=2&y=2&z=3&s=", false),
        Url {
                protocol: Some("http:"),
                slashes: true,
                auth: Some("user:pass"),
                hostname: Some("mt0.google.com"),
                search: Some("???&hl=en&src=api&x=2&y=2&z=3&s="),
                pathname: Some("/vt/lyrs=m@114"),
                ..Default::default()
        }
    );
}

#[test]
fn etcpasswd() {
    assert_eq!(
        parse("file:///etc/passwd", false),
        Url {
            slashes: true,
            protocol: Some("file:"),
            pathname: Some("/etc/passwd"),
            hostname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn etcpasswd2() {
    assert_eq!(
        parse("file://localhost/etc/passwd", false),
        Url {
            protocol: Some("file:"),
            slashes: true,
            pathname: Some("/etc/passwd"),
            hostname: Some("localhost"),
            ..Default::default()
        }
    );
}

#[test]
fn etcpasswd3() {
    assert_eq!(
        parse("file://foo/etc/passwd", false),
        Url {
            protocol: Some("file:"),
            slashes: true,
            pathname: Some("/etc/passwd"),
            hostname: Some("foo"),
            ..Default::default()
        }
    );
}

#[test]
fn etcnode() {
    assert_eq!(
        parse("file:///etc/node/", false),
        Url {
            slashes: true,
            protocol: Some("file:"),
            pathname: Some("/etc/node/"),
            hostname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn etcnode2() {
    assert_eq!(
        parse("file://localhost/etc/node/", false),
        Url {
            protocol: Some("file:"),
            slashes: true,
            pathname: Some("/etc/node/"),
            hostname: Some("localhost"),
            ..Default::default()
        }
    );
}

#[test]
fn etcnode3() {
    assert_eq!(
        parse("file://foo/etc/node/", false),
        Url {
            protocol: Some("file:"),
            slashes: true,
            pathname: Some("/etc/node/"),
            hostname: Some("foo"),
            ..Default::default()
        }
    );
}

#[test]
fn testdotdot() {
    assert_eq!(
        parse("http:/baz/../foo/bar", false),
        Url {
            protocol: Some("http:"),
            pathname: Some("/baz/../foo/bar"),
            ..Default::default()
        }
    );
}

#[test]
fn testfullurl() {
    assert_eq!(
        parse("http://user:pass@example.com:8000/foo/bar?baz=quux#frag", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user:pass"),
            port: Some("8000"),
            hostname: Some("example.com"),
            hash: Some("#frag"),
            search: Some("?baz=quux"),
            pathname: Some("/foo/bar"),
            ..Default::default()
        }
    );
}

#[test]
fn testnoproto() {
    assert_eq!(
        parse("//user:pass@example.com:8000/foo/bar?baz=quux#frag", false),
        Url {
            slashes: true,
            auth: Some("user:pass"),
            port: Some("8000"),
            hostname: Some("example.com"),
            hash: Some("#frag"),
            search: Some("?baz=quux"),
            pathname: Some("/foo/bar"),
            ..Default::default()
        }
    );
}

#[test]
fn testnohost() {
    assert_eq!(
        parse("/foo/bar?baz=quux#frag", false),
        Url {
            hash: Some("#frag"),
            search: Some("?baz=quux"),
            pathname: Some("/foo/bar"),
            ..Default::default()
        }
    );
}

#[test]
fn oneslash() {
    assert_eq!(
        parse("http:/foo/bar?baz=quux#frag", false),
        Url {
            protocol: Some("http:"),
            hash: Some("#frag"),
            search: Some("?baz=quux"),
            pathname: Some("/foo/bar"),
            ..Default::default()
        }
    );
}

#[test]
fn mailto() {
    assert_eq!(
        parse("mailto:foo@bar.com?subject=hello", false),
        Url {
            protocol: Some("mailto:"),
            auth: Some("foo"),
            hostname: Some("bar.com"),
            search: Some("?subject=hello"),
            ..Default::default()
        }
    );
}

#[test]
fn javascript() {
    assert_eq!(
        parse("javascript:alert(\'hello\');", false),
        Url {
            protocol: Some("javascript:"),
            pathname: Some("alert(\'hello\');"),
            ..Default::default()
        }
    );
}

#[test]
fn xmpp() {
    assert_eq!(
        parse("xmpp:isaacschlueter@jabber.org", false),
        Url {
            protocol: Some("xmpp:"),
            auth: Some("isaacschlueter"),
            hostname: Some("jabber.org"),
            ..Default::default()
        }
    );
}

#[test]
fn testatpass() {
    assert_eq!(
        parse("http://atpass:foo%40bar@127.0.0.1:8080/path?search=foo#bar", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("atpass:foo%40bar"),
            hostname: Some("127.0.0.1"),
            port: Some("8080"),
            pathname: Some("/path"),
            search: Some("?search=foo"),
            hash: Some("#bar"),
            ..Default::default()
        }
    );
}

#[test]
fn svnssh() {
    assert_eq!(
        parse("svn+ssh://foo/bar", false),
        Url {
            hostname: Some("foo"),
            protocol: Some("svn+ssh:"),
            pathname: Some("/bar"),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn dashtest() {
    assert_eq!(
        parse("dash-test://foo/bar", false),
        Url {
            hostname: Some("foo"),
            protocol: Some("dash-test:"),
            pathname: Some("/bar"),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn dashtest2() {
    assert_eq!(
        parse("dash-test:foo/bar", false),
        Url {
            hostname: Some("foo"),
            protocol: Some("dash-test:"),
            pathname: Some("/bar"),
            ..Default::default()
        }
    );
}

#[test]
fn dottest() {
    assert_eq!(
        parse("dot.test://foo/bar", false),
        Url {
            hostname: Some("foo"),
            protocol: Some("dot.test:"),
            pathname: Some("/bar"),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn dottest2() {
    assert_eq!(
        parse("dot.test:foo/bar", false),
        Url {
            hostname: Some("foo"),
            protocol: Some("dot.test:"),
            pathname: Some("/bar"),
            ..Default::default()
        }
    );
}

#[test]
// IDNA tests
fn idna1() {
    assert_eq!(
        parse("http://www.日本語.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("www.日本語.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn idna2() {
    assert_eq!(
        parse("http://example.Bücher.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.Bücher.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn idna3() {
    assert_eq!(
        parse("http://www.Äffchen.com/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("www.Äffchen.com"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn idna4() {
    assert_eq!(
        parse("http://www.Äffchen.cOm;A/b/c?d=e#f g<h>i", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("www.Äffchen.cOm"),
            pathname: Some(";A/b/c"),
            search: Some("?d=e"),
            hash: Some("#f g<h>i"),
            ..Default::default()
        }
    );
}

#[test]
fn idna5() {
    assert_eq!(
        parse("http://SÉLIER.COM/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("SÉLIER.COM"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn idna6() {
    assert_eq!(
        parse("http://ﻞﻴﻬﻣﺎﺒﺘﻜﻠﻣﻮﺸﻋﺮﺒﻳ؟.ﻱ؟/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("ﻞﻴﻬﻣﺎﺒﺘﻜﻠﻣﻮﺸﻋﺮﺒﻳ؟.ﻱ؟"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn idna7() {
    assert_eq!(
        parse("http://➡.ws/➡", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("➡.ws"),
            pathname: Some("/➡"),
            ..Default::default()
        }
    );
}

#[test]
fn amazon() {
    assert_eq!(
        parse("http://bucket_name.s3.amazonaws.com/image.jpg", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("bucket_name.s3.amazonaws.com"),
            pathname: Some("/image.jpg"),
            ..Default::default()
        }
    );
}

#[test]
fn githttp() {
    assert_eq!(
        parse("git+http://github.com/joyent/node.git", false),
        Url {
            protocol: Some("git+http:"),
            slashes: true,
            hostname: Some("github.com"),
            pathname: Some("/joyent/node.git"),
            ..Default::default()
        }
    );
}

#[test]
// if local1@domain1 is uses as a relative URL it may
// be parse into auth@hostname, but here there is no
// way to make it work in url.parse, I add the test to be explicit
fn local1domain1() {
    assert_eq!(
        parse("local1@domain1", false),
        Url {
            pathname: Some("local1@domain1"),
            ..Default::default()
        }
    );
}

#[test]
// While this may seem counter-intuitive, a browser will parse
// <a href='www.google.com'> as a path.
fn bare_domain() {
    assert_eq!(
        parse("www.example.com", false),
        Url {
            pathname: Some("www.example.com"),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_1() {
    assert_eq!(
        parse("[fe80::1]", false),
        Url {
            pathname: Some("[fe80::1]"),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_2() {
    assert_eq!(
        parse("coap://[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]", false),
        Url {
            protocol: Some("coap:"),
            slashes: true,
            hostname: Some("FEDC:BA98:7654:3210:FEDC:BA98:7654:3210"),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_3() {
    assert_eq!(
        parse("coap://[1080:0:0:0:8:800:200C:417A]:61616/", false),
        Url {
            protocol: Some("coap:"),
            slashes: true,
            port: Some("61616"),
            hostname: Some("1080:0:0:0:8:800:200C:417A"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_4() {
    assert_eq!(
        parse("http://user:password@[3ffe:2a00:100:7031::1]:8080", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user:password"),
            port: Some("8080"),
            hostname: Some("3ffe:2a00:100:7031::1"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_5() {
    assert_eq!(
        parse("coap://u:p@[::192.9.5.5]:61616/.well-known/r?n=Temperature", false),
        Url {
            protocol: Some("coap:"),
            slashes: true,
            auth: Some("u:p"),
            port: Some("61616"),
            hostname: Some("::192.9.5.5"),
            search: Some("?n=Temperature"),
            pathname: Some("/.well-known/r"),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port1() {
    assert_eq!(
        parse("http://example.com:", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            pathname: Some(":"),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port2() {
    assert_eq!(
        parse("http://example.com:/a/b.html", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            pathname: Some(":/a/b.html"),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port3() {
    assert_eq!(
        parse("http://example.com:?a=b", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            search: Some("?a=b"),
            pathname: Some(":"),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port4() {
    assert_eq!(
        parse("http://example.com:#abc", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            hash: Some("#abc"),
            pathname: Some(":"),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port5() {
    assert_eq!(
        parse("http://[fe80::1]:/a/b?a=b#abc", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("fe80::1"),
            search: Some("?a=b"),
            hash: Some("#abc"),
            pathname: Some(":/a/b"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash1() {
    assert_eq!(
        parse("http://-lovemonsterz.tumblr.com/rss", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("-lovemonsterz.tumblr.com"),
            pathname: Some("/rss"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash2() {
    assert_eq!(
        parse("http://-lovemonsterz.tumblr.com:80/rss", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            port: Some("80"),
            hostname: Some("-lovemonsterz.tumblr.com"),
            pathname: Some("/rss"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash3() {
    assert_eq!(
        parse("http://user:pass@-lovemonsterz.tumblr.com/rss", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user:pass"),
            hostname: Some("-lovemonsterz.tumblr.com"),
            pathname: Some("/rss"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash4() {
    assert_eq!(
        parse("http://user:pass@-lovemonsterz.tumblr.com:80/rss", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user:pass"),
            port: Some("80"),
            hostname: Some("-lovemonsterz.tumblr.com"),
            pathname: Some("/rss"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund1() {
    assert_eq!(
        parse("http://_jabber._tcp.google.com/test", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("_jabber._tcp.google.com"),
            pathname: Some("/test"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund2() {
    assert_eq!(
        parse("http://user:pass@_jabber._tcp.google.com/test", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user:pass"),
            hostname: Some("_jabber._tcp.google.com"),
            pathname: Some("/test"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund3() {
    assert_eq!(
        parse("http://_jabber._tcp.google.com:80/test", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            port: Some("80"),
            hostname: Some("_jabber._tcp.google.com"),
            pathname: Some("/test"),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund4() {
    assert_eq!(
        parse("http://user:pass@_jabber._tcp.google.com:80/test", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("user:pass"),
            port: Some("80"),
            hostname: Some("_jabber._tcp.google.com"),
            pathname: Some("/test"),
            ..Default::default()
        }
    );
}

#[test]
fn testpuncts() {
    assert_eq!(
        parse("http://x:1/' <>\"`/{}|\\^~`/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            port: Some("1"),
            hostname: Some("x"),
            pathname: Some("/' <>\"`/{}|\\^~`/"),
            ..Default::default()
        }
    );
}

#[test]
fn testat1() {
    assert_eq!(
        parse("http://a@b@c/", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("a@b"),
            hostname: Some("c"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn testat2() {
    assert_eq!(
        parse("http://a@b?@c", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("a"),
            hostname: Some("b"),
            pathname: Some(""),
            search: Some("?@c"),
            ..Default::default()
        }
    );
}

#[test]
fn testspecials() {
    assert_eq!(
        parse("http://a\r\" \t\n<'b:b@c\r\nd/e?f", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            auth: Some("a\r\" \t\n<'b:b"),
            hostname: Some("c"),
            search: Some("?f"),
            pathname: Some("\r\nd/e"),
            ..Default::default()
        }
    );
}

#[test]
// git urls used by npm
fn giturls() {
    assert_eq!(
        parse("git+ssh://git@github.com:npm/npm", false),
        Url {
            protocol: Some("git+ssh:"),
            slashes: true,
            auth: Some("git"),
            hostname: Some("github.com"),
            pathname: Some(":npm/npm"),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag1() {
    assert_eq!(
        parse("http://example.com?foo=bar#frag", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            hash: Some("#frag"),
            search: Some("?foo=bar"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag2() {
    assert_eq!(
        parse("http://example.com?foo=@bar#frag", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            hash: Some("#frag"),
            search: Some("?foo=@bar"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag3() {
    assert_eq!(
        parse("http://example.com?foo=/bar/#frag", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            hash: Some("#frag"),
            search: Some("?foo=/bar/"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag4() {
    assert_eq!(
        parse("http://example.com?foo=?bar/#frag", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            hash: Some("#frag"),
            search: Some("?foo=?bar/"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag5() {
    assert_eq!(
        parse("http://example.com#frag=?bar/#frag", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            hostname: Some("example.com"),
            hash: Some("#frag=?bar/#frag"),
            pathname: Some(""),
            ..Default::default()
        }
    );
}

#[test]
fn testxss() {
    assert_eq!(
        parse("http://google.com\" onload=\"alert(42)/", false),
        Url {
            hostname: Some("google.com"),
            protocol: Some("http:"),
            slashes: true,
            pathname: Some("\" onload=\"alert(42)/"),
            ..Default::default()
        }
    );
}

#[test]
fn acom() {
    assert_eq!(
        parse("http://a.com/a/b/c?s#h", false),
        Url {
            protocol: Some("http:"),
            slashes: true,
            pathname: Some("/a/b/c"),
            hostname: Some("a.com"),
            hash: Some("#h"),
            search: Some("?s"),
            ..Default::default()
        }
    );
}

#[test]
fn test127001() {
    assert_eq!(
        parse("http://atpass:foo%40bar@127.0.0.1/", false),
        Url {
            auth: Some("atpass:foo%40bar"),
            slashes: true,
            hostname: Some("127.0.0.1"),
            protocol: Some("http:"),
            pathname: Some("/"),
            ..Default::default()
        }
    );
}

#[test]
fn testescaped() {
    assert_eq!(
        parse("http://atslash%2F%40:%2F%40@foo/", false),
        Url {
            auth: Some("atslash%2F%40:%2F%40"),
            hostname: Some("foo"),
            protocol: Some("http:"),
            pathname: Some("/"),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_a() {
    assert_eq!(
        parse("coap:u:p@[::1]:61616/.well-known/r?n=Temperature", false),
        Url {
            protocol: Some("coap:"),
            auth: Some("u:p"),
            hostname: Some("::1"),
            port: Some("61616"),
            pathname: Some("/.well-known/r"),
            search: Some("?n=Temperature"),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_b() {
    assert_eq!(
        parse("coap:[fedc:ba98:7654:3210:fedc:ba98:7654:3210]:61616/s/stopButton", false),
        Url {
            hostname: Some("fedc:ba98:7654:3210:fedc:ba98:7654:3210"),
            port: Some("61616"),
            protocol: Some("coap:"),
            pathname: Some("/s/stopButton"),
            ..Default::default()
        }
    );
}

#[test]
// encode context-specific delimiters in path and query, but do not touch
// other non-delimiter chars like `%`.
// <https://github.com/joyent/node/issues/4082>
fn delims() {
    // `?` and `#` in path and search
    assert_eq!(
        parse("http://ex.com/foo%3F100%m%23r?abc=the%231?&foo=bar#frag", false),
        Url {
            protocol: Some("http:"),
            hostname: Some("ex.com"),
            hash: Some("#frag"),
            search: Some("?abc=the%231?&foo=bar"),
            pathname: Some("/foo%3F100%m%23r"),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn delims2() {
    // `?` and `#` in search only
    assert_eq!(
        parse("http://ex.com/fooA100%mBr?abc=the%231?&foo=bar#frag", false),
        Url {
            protocol: Some("http:"),
            hostname: Some("ex.com"),
            hash: Some("#frag"),
            search: Some("?abc=the%231?&foo=bar"),
            pathname: Some("/fooA100%mBr"),
            slashes: true,
            ..Default::default()
        }
    );
}

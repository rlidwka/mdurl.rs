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
use mdurl::parse_url;

/*#[test]
fn simple_path() {
    assert_eq!(
        parse_url("//some_path"),
        Url {
            pathname: Some("//some_path".into()),
            ..Default::default()
        }
    );
}*/

#[test]
fn test1() {
    assert_eq!(
        parse_url("HTTP://www.example.com/"),
        Url {
            protocol: Some("HTTP:".into()),
            slashes: true,
            hostname: Some("www.example.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn test2() {
    assert_eq!(
        parse_url("HTTP://www.example.com"),
        Url {
            protocol: Some("HTTP:".into()),
            slashes: true,
            hostname: Some("www.example.com".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn test3() {
    assert_eq!(
        parse_url("http://www.ExAmPlE.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("www.ExAmPlE.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testpw1() {
    assert_eq!(
        parse_url("http://user:pw@www.ExAmPlE.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user:pw".into()),
            hostname: Some("www.ExAmPlE.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testpw2() {
    assert_eq!(
        parse_url("http://USER:PW@www.ExAmPlE.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("USER:PW".into()),
            hostname: Some("www.ExAmPlE.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testauth() {
    assert_eq!(
        parse_url("http://user@www.example.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user".into()),
            hostname: Some("www.example.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testauth3a() {
    assert_eq!(
        parse_url("http://user%3Apw@www.example.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user%3Apw".into()),
            hostname: Some("www.example.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn thatsall() {
    assert_eq!(
        parse_url("http://x.com/path?that\'s#all, folks"),
        Url {
            protocol: Some("http:".into()),
            hostname: Some("x.com".into()),
            slashes: true,
            search: Some("?that\'s".into()),
            pathname: Some("/path".into()),
            hash: Some("#all, folks".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testupper() {
    assert_eq!(
        parse_url("HTTP://X.COM/Y"),
        Url {
            protocol: Some("HTTP:".into()),
            slashes: true,
            hostname: Some("X.COM".into()),
            pathname: Some("/Y".into()),
            ..Default::default()
        }
    );
}

#[test]
// + not an invalid host character
// per https://url.spec.whatwg.org/#host-parsing
fn testplus() {
    assert_eq!(
        parse_url("http://x.y.com+a/b/c"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("x.y.com+a".into()),
            pathname: Some("/b/c".into()),
            ..Default::default()
        }
    );
}

#[test]
// an unexpected invalid char in the hostname.
fn invalid_char_in_hostname() {
    assert_eq!(
        parse_url("HtTp://x.y.cOm;a/b/c?d=e#f g<h>i"),
        Url {
            protocol: Some("HtTp:".into()),
            slashes: true,
            hostname: Some("x.y.cOm".into()),
            pathname: Some(";a/b/c".into()),
            search: Some("?d=e".into()),
            hash: Some("#f g<h>i".into()),
            ..Default::default()
        }
    );
}

#[test]
// make sure that we don't accidentally lcast the path parts.
fn testlcast() {
    assert_eq!(
        parse_url("HtTp://x.y.cOm;A/b/c?d=e#f g<h>i"),
        Url {
            protocol: Some("HtTp:".into()),
            slashes: true,
            hostname: Some("x.y.cOm".into()),
            pathname: Some(";A/b/c".into()),
            search: Some("?d=e".into()),
            hash: Some("#f g<h>i".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testdots() {
    assert_eq!(
        parse_url("http://x...y...#p"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("x...y...".into()),
            hash: Some("#p".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testquoted() {
    assert_eq!(
        parse_url("http://x/p/\"quoted\""),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("x".into()),
            pathname: Some("/p/\"quoted\"".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testangled() {
    assert_eq!(
        parse_url("<http://goo.corn/bread> Is a URL!"),
        Url {
            pathname: Some("<http://goo.corn/bread> Is a URL!".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testnarwhaljs() {
    assert_eq!(
        parse_url("http://www.narwhaljs.org/blog/categories?id=news"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("www.narwhaljs.org".into()),
            search: Some("?id=news".into()),
            pathname: Some("/blog/categories".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testgoog1() {
    assert_eq!(
        parse_url("http://mt0.google.com/vt/lyrs=m@114&hl=en&src=api&x=2&y=2&z=3&s="),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("mt0.google.com".into()),
            pathname: Some("/vt/lyrs=m@114&hl=en&src=api&x=2&y=2&z=3&s=".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testgoog2() {
    assert_eq!(
        parse_url("http://mt0.google.com/vt/lyrs=m@114???&hl=en&src=api&x=2&y=2&z=3&s="),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("mt0.google.com".into()),
            search: Some("???&hl=en&src=api&x=2&y=2&z=3&s=".into()),
            pathname: Some("/vt/lyrs=m@114".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testgoog3() {
    assert_eq!(
        parse_url("http://user:pass@mt0.google.com/vt/lyrs=m@114???&hl=en&src=api&x=2&y=2&z=3&s="),
        Url {
                protocol: Some("http:".into()),
                slashes: true,
                auth: Some("user:pass".into()),
                hostname: Some("mt0.google.com".into()),
                search: Some("???&hl=en&src=api&x=2&y=2&z=3&s=".into()),
                pathname: Some("/vt/lyrs=m@114".into()),
                ..Default::default()
        }
    );
}

#[test]
fn etcpasswd() {
    assert_eq!(
        parse_url("file:///etc/passwd"),
        Url {
            slashes: true,
            protocol: Some("file:".into()),
            pathname: Some("/etc/passwd".into()),
            hostname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn etcpasswd2() {
    assert_eq!(
        parse_url("file://localhost/etc/passwd"),
        Url {
            protocol: Some("file:".into()),
            slashes: true,
            pathname: Some("/etc/passwd".into()),
            hostname: Some("localhost".into()),
            ..Default::default()
        }
    );
}

#[test]
fn etcpasswd3() {
    assert_eq!(
        parse_url("file://foo/etc/passwd"),
        Url {
            protocol: Some("file:".into()),
            slashes: true,
            pathname: Some("/etc/passwd".into()),
            hostname: Some("foo".into()),
            ..Default::default()
        }
    );
}

#[test]
fn etcnode() {
    assert_eq!(
        parse_url("file:///etc/node/"),
        Url {
            slashes: true,
            protocol: Some("file:".into()),
            pathname: Some("/etc/node/".into()),
            hostname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn etcnode2() {
    assert_eq!(
        parse_url("file://localhost/etc/node/"),
        Url {
            protocol: Some("file:".into()),
            slashes: true,
            pathname: Some("/etc/node/".into()),
            hostname: Some("localhost".into()),
            ..Default::default()
        }
    );
}

#[test]
fn etcnode3() {
    assert_eq!(
        parse_url("file://foo/etc/node/"),
        Url {
            protocol: Some("file:".into()),
            slashes: true,
            pathname: Some("/etc/node/".into()),
            hostname: Some("foo".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testdotdot() {
    assert_eq!(
        parse_url("http:/baz/../foo/bar"),
        Url {
            protocol: Some("http:".into()),
            pathname: Some("/baz/../foo/bar".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testfullurl() {
    assert_eq!(
        parse_url("http://user:pass@example.com:8000/foo/bar?baz=quux#frag"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user:pass".into()),
            port: Some("8000".into()),
            hostname: Some("example.com".into()),
            hash: Some("#frag".into()),
            search: Some("?baz=quux".into()),
            pathname: Some("/foo/bar".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testnoproto() {
    assert_eq!(
        parse_url("//user:pass@example.com:8000/foo/bar?baz=quux#frag"),
        Url {
            slashes: true,
            auth: Some("user:pass".into()),
            port: Some("8000".into()),
            hostname: Some("example.com".into()),
            hash: Some("#frag".into()),
            search: Some("?baz=quux".into()),
            pathname: Some("/foo/bar".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testnohost() {
    assert_eq!(
        parse_url("/foo/bar?baz=quux#frag"),
        Url {
            hash: Some("#frag".into()),
            search: Some("?baz=quux".into()),
            pathname: Some("/foo/bar".into()),
            ..Default::default()
        }
    );
}

#[test]
fn oneslash() {
    assert_eq!(
        parse_url("http:/foo/bar?baz=quux#frag"),
        Url {
            protocol: Some("http:".into()),
            hash: Some("#frag".into()),
            search: Some("?baz=quux".into()),
            pathname: Some("/foo/bar".into()),
            ..Default::default()
        }
    );
}

#[test]
fn mailto() {
    assert_eq!(
        parse_url("mailto:foo@bar.com?subject=hello"),
        Url {
            protocol: Some("mailto:".into()),
            auth: Some("foo".into()),
            hostname: Some("bar.com".into()),
            search: Some("?subject=hello".into()),
            ..Default::default()
        }
    );
}

#[test]
fn javascript() {
    assert_eq!(
        parse_url("javascript:alert(\'hello\');"),
        Url {
            protocol: Some("javascript:".into()),
            pathname: Some("alert(\'hello\');".into()),
            ..Default::default()
        }
    );
}

#[test]
fn xmpp() {
    assert_eq!(
        parse_url("xmpp:isaacschlueter@jabber.org"),
        Url {
            protocol: Some("xmpp:".into()),
            auth: Some("isaacschlueter".into()),
            hostname: Some("jabber.org".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testatpass() {
    assert_eq!(
        parse_url("http://atpass:foo%40bar@127.0.0.1:8080/path?search=foo#bar"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("atpass:foo%40bar".into()),
            hostname: Some("127.0.0.1".into()),
            port: Some("8080".into()),
            pathname: Some("/path".into()),
            search: Some("?search=foo".into()),
            hash: Some("#bar".into()),
            ..Default::default()
        }
    );
}

#[test]
fn svnssh() {
    assert_eq!(
        parse_url("svn+ssh://foo/bar"),
        Url {
            hostname: Some("foo".into()),
            protocol: Some("svn+ssh:".into()),
            pathname: Some("/bar".into()),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn dashtest() {
    assert_eq!(
        parse_url("dash-test://foo/bar"),
        Url {
            hostname: Some("foo".into()),
            protocol: Some("dash-test:".into()),
            pathname: Some("/bar".into()),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn dashtest2() {
    assert_eq!(
        parse_url("dash-test:foo/bar"),
        Url {
            hostname: Some("foo".into()),
            protocol: Some("dash-test:".into()),
            pathname: Some("/bar".into()),
            ..Default::default()
        }
    );
}

#[test]
fn dottest() {
    assert_eq!(
        parse_url("dot.test://foo/bar"),
        Url {
            hostname: Some("foo".into()),
            protocol: Some("dot.test:".into()),
            pathname: Some("/bar".into()),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn dottest2() {
    assert_eq!(
        parse_url("dot.test:foo/bar"),
        Url {
            hostname: Some("foo".into()),
            protocol: Some("dot.test:".into()),
            pathname: Some("/bar".into()),
            ..Default::default()
        }
    );
}

#[test]
// IDNA tests
fn idna1() {
    assert_eq!(
        parse_url("http://www.日本語.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("www.日本語.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn idna2() {
    assert_eq!(
        parse_url("http://example.Bücher.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.Bücher.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn idna3() {
    assert_eq!(
        parse_url("http://www.Äffchen.com/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("www.Äffchen.com".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn idna4() {
    assert_eq!(
        parse_url("http://www.Äffchen.cOm;A/b/c?d=e#f g<h>i"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("www.Äffchen.cOm".into()),
            pathname: Some(";A/b/c".into()),
            search: Some("?d=e".into()),
            hash: Some("#f g<h>i".into()),
            ..Default::default()
        }
    );
}

#[test]
fn idna5() {
    assert_eq!(
        parse_url("http://SÉLIER.COM/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("SÉLIER.COM".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn idna6() {
    assert_eq!(
        parse_url("http://ﻞﻴﻬﻣﺎﺒﺘﻜﻠﻣﻮﺸﻋﺮﺒﻳ؟.ﻱ؟/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("ﻞﻴﻬﻣﺎﺒﺘﻜﻠﻣﻮﺸﻋﺮﺒﻳ؟.ﻱ؟".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn idna7() {
    assert_eq!(
        parse_url("http://➡.ws/➡"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("➡.ws".into()),
            pathname: Some("/➡".into()),
            ..Default::default()
        }
    );
}

#[test]
fn amazon() {
    assert_eq!(
        parse_url("http://bucket_name.s3.amazonaws.com/image.jpg"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("bucket_name.s3.amazonaws.com".into()),
            pathname: Some("/image.jpg".into()),
            ..Default::default()
        }
    );
}

#[test]
fn githttp() {
    assert_eq!(
        parse_url("git+http://github.com/joyent/node.git"),
        Url {
            protocol: Some("git+http:".into()),
            slashes: true,
            hostname: Some("github.com".into()),
            pathname: Some("/joyent/node.git".into()),
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
        parse_url("local1@domain1"),
        Url {
            pathname: Some("local1@domain1".into()),
            ..Default::default()
        }
    );
}

#[test]
// While this may seem counter-intuitive, a browser will parse
// <a href='www.google.com'> as a path.
fn bare_domain() {
    assert_eq!(
        parse_url("www.example.com"),
        Url {
            pathname: Some("www.example.com".into()),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_1() {
    assert_eq!(
        parse_url("[fe80::1]"),
        Url {
            pathname: Some("[fe80::1]".into()),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_2() {
    assert_eq!(
        parse_url("coap://[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]"),
        Url {
            protocol: Some("coap:".into()),
            slashes: true,
            hostname: Some("FEDC:BA98:7654:3210:FEDC:BA98:7654:3210".into()),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_3() {
    assert_eq!(
        parse_url("coap://[1080:0:0:0:8:800:200C:417A]:61616/"),
        Url {
            protocol: Some("coap:".into()),
            slashes: true,
            port: Some("61616".into()),
            hostname: Some("1080:0:0:0:8:800:200C:417A".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_4() {
    assert_eq!(
        parse_url("http://user:password@[3ffe:2a00:100:7031::1]:8080"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user:password".into()),
            port: Some("8080".into()),
            hostname: Some("3ffe:2a00:100:7031::1".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_5() {
    assert_eq!(
        parse_url("coap://u:p@[::192.9.5.5]:61616/.well-known/r?n=Temperature"),
        Url {
            protocol: Some("coap:".into()),
            slashes: true,
            auth: Some("u:p".into()),
            port: Some("61616".into()),
            hostname: Some("::192.9.5.5".into()),
            search: Some("?n=Temperature".into()),
            pathname: Some("/.well-known/r".into()),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port1() {
    assert_eq!(
        parse_url("http://example.com:"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            pathname: Some(":".into()),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port2() {
    assert_eq!(
        parse_url("http://example.com:/a/b.html"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            pathname: Some(":/a/b.html".into()),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port3() {
    assert_eq!(
        parse_url("http://example.com:?a=b"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            search: Some("?a=b".into()),
            pathname: Some(":".into()),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port4() {
    assert_eq!(
        parse_url("http://example.com:#abc"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            hash: Some("#abc".into()),
            pathname: Some(":".into()),
            ..Default::default()
        }
    );
}

#[test]
fn empty_port5() {
    assert_eq!(
        parse_url("http://[fe80::1]:/a/b?a=b#abc"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("fe80::1".into()),
            search: Some("?a=b".into()),
            hash: Some("#abc".into()),
            pathname: Some(":/a/b".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash1() {
    assert_eq!(
        parse_url("http://-lovemonsterz.tumblr.com/rss"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("-lovemonsterz.tumblr.com".into()),
            pathname: Some("/rss".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash2() {
    assert_eq!(
        parse_url("http://-lovemonsterz.tumblr.com:80/rss"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            port: Some("80".into()),
            hostname: Some("-lovemonsterz.tumblr.com".into()),
            pathname: Some("/rss".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash3() {
    assert_eq!(
        parse_url("http://user:pass@-lovemonsterz.tumblr.com/rss"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user:pass".into()),
            hostname: Some("-lovemonsterz.tumblr.com".into()),
            pathname: Some("/rss".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingdash4() {
    assert_eq!(
        parse_url("http://user:pass@-lovemonsterz.tumblr.com:80/rss"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user:pass".into()),
            port: Some("80".into()),
            hostname: Some("-lovemonsterz.tumblr.com".into()),
            pathname: Some("/rss".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund1() {
    assert_eq!(
        parse_url("http://_jabber._tcp.google.com/test"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("_jabber._tcp.google.com".into()),
            pathname: Some("/test".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund2() {
    assert_eq!(
        parse_url("http://user:pass@_jabber._tcp.google.com/test"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user:pass".into()),
            hostname: Some("_jabber._tcp.google.com".into()),
            pathname: Some("/test".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund3() {
    assert_eq!(
        parse_url("http://_jabber._tcp.google.com:80/test"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            port: Some("80".into()),
            hostname: Some("_jabber._tcp.google.com".into()),
            pathname: Some("/test".into()),
            ..Default::default()
        }
    );
}

#[test]
fn leadingund4() {
    assert_eq!(
        parse_url("http://user:pass@_jabber._tcp.google.com:80/test"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("user:pass".into()),
            port: Some("80".into()),
            hostname: Some("_jabber._tcp.google.com".into()),
            pathname: Some("/test".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testpuncts() {
    assert_eq!(
        parse_url("http://x:1/' <>\"`/{}|\\^~`/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            port: Some("1".into()),
            hostname: Some("x".into()),
            pathname: Some("/' <>\"`/{}|\\^~`/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testat1() {
    assert_eq!(
        parse_url("http://a@b@c/"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("a@b".into()),
            hostname: Some("c".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testat2() {
    assert_eq!(
        parse_url("http://a@b?@c"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("a".into()),
            hostname: Some("b".into()),
            pathname: Some("".into()),
            search: Some("?@c".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testspecials() {
    assert_eq!(
        parse_url("http://a\r\" \t\n<'b:b@c\r\nd/e?f"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            auth: Some("a\r\" \t\n<'b:b".into()),
            hostname: Some("c".into()),
            search: Some("?f".into()),
            pathname: Some("\r\nd/e".into()),
            ..Default::default()
        }
    );
}

#[test]
// git urls used by npm
fn giturls() {
    assert_eq!(
        parse_url("git+ssh://git@github.com:npm/npm"),
        Url {
            protocol: Some("git+ssh:".into()),
            slashes: true,
            auth: Some("git".into()),
            hostname: Some("github.com".into()),
            pathname: Some(":npm/npm".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag1() {
    assert_eq!(
        parse_url("http://example.com?foo=bar#frag"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            hash: Some("#frag".into()),
            search: Some("?foo=bar".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag2() {
    assert_eq!(
        parse_url("http://example.com?foo=@bar#frag"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            hash: Some("#frag".into()),
            search: Some("?foo=@bar".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag3() {
    assert_eq!(
        parse_url("http://example.com?foo=/bar/#frag"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            hash: Some("#frag".into()),
            search: Some("?foo=/bar/".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag4() {
    assert_eq!(
        parse_url("http://example.com?foo=?bar/#frag"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            hash: Some("#frag".into()),
            search: Some("?foo=?bar/".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testfrag5() {
    assert_eq!(
        parse_url("http://example.com#frag=?bar/#frag"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            hostname: Some("example.com".into()),
            hash: Some("#frag=?bar/#frag".into()),
            pathname: Some("".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testxss() {
    assert_eq!(
        parse_url("http://google.com\" onload=\"alert(42)/"),
        Url {
            hostname: Some("google.com".into()),
            protocol: Some("http:".into()),
            slashes: true,
            pathname: Some("\" onload=\"alert(42)/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn acom() {
    assert_eq!(
        parse_url("http://a.com/a/b/c?s#h"),
        Url {
            protocol: Some("http:".into()),
            slashes: true,
            pathname: Some("/a/b/c".into()),
            hostname: Some("a.com".into()),
            hash: Some("#h".into()),
            search: Some("?s".into()),
            ..Default::default()
        }
    );
}

#[test]
fn test127001() {
    assert_eq!(
        parse_url("http://atpass:foo%40bar@127.0.0.1/"),
        Url {
            auth: Some("atpass:foo%40bar".into()),
            slashes: true,
            hostname: Some("127.0.0.1".into()),
            protocol: Some("http:".into()),
            pathname: Some("/".into()),
            ..Default::default()
        }
    );
}

#[test]
fn testescaped() {
    assert_eq!(
        parse_url("http://atslash%2F%40:%2F%40@foo/"),
        Url {
            auth: Some("atslash%2F%40:%2F%40".into()),
            hostname: Some("foo".into()),
            protocol: Some("http:".into()),
            pathname: Some("/".into()),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_a() {
    assert_eq!(
        parse_url("coap:u:p@[::1]:61616/.well-known/r?n=Temperature"),
        Url {
            protocol: Some("coap:".into()),
            auth: Some("u:p".into()),
            hostname: Some("::1".into()),
            port: Some("61616".into()),
            pathname: Some("/.well-known/r".into()),
            search: Some("?n=Temperature".into()),
            ..Default::default()
        }
    );
}

#[test]
fn ipv6_b() {
    assert_eq!(
        parse_url("coap:[fedc:ba98:7654:3210:fedc:ba98:7654:3210]:61616/s/stopButton"),
        Url {
            hostname: Some("fedc:ba98:7654:3210:fedc:ba98:7654:3210".into()),
            port: Some("61616".into()),
            protocol: Some("coap:".into()),
            pathname: Some("/s/stopButton".into()),
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
        parse_url("http://ex.com/foo%3F100%m%23r?abc=the%231?&foo=bar#frag"),
        Url {
            protocol: Some("http:".into()),
            hostname: Some("ex.com".into()),
            hash: Some("#frag".into()),
            search: Some("?abc=the%231?&foo=bar".into()),
            pathname: Some("/foo%3F100%m%23r".into()),
            slashes: true,
            ..Default::default()
        }
    );
}

#[test]
fn delims2() {
    // `?` and `#` in search only
    assert_eq!(
        parse_url("http://ex.com/fooA100%mBr?abc=the%231?&foo=bar#frag"),
        Url {
            protocol: Some("http:".into()),
            hostname: Some("ex.com".into()),
            hash: Some("#frag".into()),
            search: Some("?abc=the%231?&foo=bar".into()),
            pathname: Some("/fooA100%mBr".into()),
            slashes: true,
            ..Default::default()
        }
    );
}

use once_cell::sync::Lazy;
use regex::Regex;
use super::AsciiSet;

static URLENCODED_SEQUENCE : Lazy<Regex> = Lazy::new(||
    Regex::new(r#"(%[a-fA-F0-9]{2})+"#).unwrap()
);

/// Decode percent-encoded string.
///
pub fn decode(string: &str, exclude: AsciiSet) -> String {
    URLENCODED_SEQUENCE.replace_all(string, |caps: &regex::Captures| -> String {
        let mut result = Vec::new();
        let mut bytes = caps.get(0).unwrap().as_str().as_bytes().iter();

        while bytes.next().is_some() { // skips '%'
            let byte1 = *bytes.next().unwrap();
            let byte2 = *bytes.next().unwrap();
            let decoded = (parse_hex_digit(byte1) << 4) + parse_hex_digit(byte2);

            if decoded < 0x80 && exclude.has(decoded) {
                result.push(b'%');
                result.push(byte1);
                result.push(byte2);
            } else {
                result.push(decoded);
            }
        }

        String::from_utf8_lossy(&result).into()
    }).to_string()
}

fn parse_hex_digit(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'F' => byte - b'A' + 10,
        b'a'..=b'f' => byte - b'a' + 10,
        _ => unreachable!(),
    }
}


#[cfg(test)]
mod tests {
    use super::decode;
    use super::AsciiSet;
    const SET : AsciiSet = AsciiSet::from_empty(";/?:@&=+$,#");

    #[test]
    fn should_decode_xx() {
        assert_eq!(decode("x%20xx%20%2520", SET), "x xx %20");
    }

    #[test]
    fn should_not_decode_invalid_sequences() {
        assert_eq!(decode("%2g%z1%%", SET), "%2g%z1%%");
    }

    #[test]
    fn should_not_decode_reserved_set() {
        assert_eq!(decode("%20%25%20", AsciiSet::from_empty("%")), " %25 ");
        assert_eq!(decode("%20%25%20", AsciiSet::from_empty(" ")), "%20%%20");
        assert_eq!(decode("%20%25%20", AsciiSet::from_empty(" %")), "%20%25%20");
    }

    #[test]
    fn should_deal_with_utf8() {
        assert_eq!(decode("%80", SET), "\u{fffd}");
        assert_eq!(decode("%bf", SET), "\u{fffd}");
        assert_eq!(decode("%00", SET), "\u{0}");
        assert_eq!(decode("%55", SET), "\u{55}");
        assert_eq!(decode("%7f", SET), "\u{7f}");
        assert_eq!(decode("%c7%55", SET), "\u{fffd}\u{55}");
        assert_eq!(decode("%e3%55", SET), "\u{fffd}\u{55}");
        assert_eq!(decode("%f1%55", SET), "\u{fffd}\u{55}");
        assert_eq!(decode("%c7%c0", SET), "\u{fffd}\u{fffd}");
        assert_eq!(decode("%e3%c0", SET), "\u{fffd}\u{fffd}");
        assert_eq!(decode("%f1%c0", SET), "\u{fffd}\u{fffd}");
        assert_eq!(decode("%e3%95%55", SET), "\u{fffd}\u{55}"); // js: \u{fffd}\u{fffd}\u{55}
        assert_eq!(decode("%f1%95%55", SET), "\u{fffd}\u{55}"); // js: \u{fffd}\u{fffd}\u{55}
        assert_eq!(decode("%f1%95%95%55", SET), "\u{fffd}\u{55}"); // js: \u{fffd}\u{fffd}\u{fffd}\u{55}
        assert_eq!(decode("%c7%aa", SET), "\u{1ea}");
        assert_eq!(decode("%e3%aa%aa", SET), "\u{3aaa}");
        assert_eq!(decode("%f1%aa%aa%aa", SET), "\u{6aaaa}");
        assert_eq!(decode("%c2%80", SET), "\u{80}");
        assert_eq!(decode("%e0%a0%80", SET), "\u{800}");
        assert_eq!(decode("%c1%bf", SET), "\u{fffd}\u{fffd}");
        assert_eq!(decode("%e0%9f%bf", SET), "\u{fffd}\u{fffd}\u{fffd}");
        assert_eq!(decode("%c1%80", SET), "\u{fffd}\u{fffd}");
        assert_eq!(decode("%e0%90%80", SET), "\u{fffd}\u{fffd}\u{fffd}");
        assert_eq!(decode("%df%bf", SET), "\u{7ff}");
        assert_eq!(decode("%ef%bf%bf", SET), "\u{ffff}");
        assert_eq!(decode("%f0%90%80%80", SET), "\u{10000}");
        assert_eq!(decode("%f0%90%8f%8f", SET), "\u{103cf}");
        assert_eq!(decode("%f4%8f%b0%80", SET), "\u{10fc00}");
        assert_eq!(decode("%f4%8f%bf%bf", SET), "\u{10ffff}");
        assert_eq!(decode("%f0%8f%bf%bf", SET), "\u{fffd}\u{fffd}\u{fffd}\u{fffd}");
        assert_eq!(decode("%f4%90%80%80", SET), "\u{fffd}\u{fffd}\u{fffd}\u{fffd}");
        assert_eq!(decode("%f4%9f%bf%bf", SET), "\u{fffd}\u{fffd}\u{fffd}\u{fffd}");
        assert_eq!(decode("%ed%9f%bf", SET), "\u{d7ff}");
        assert_eq!(decode("%ed%a0%80", SET), "\u{fffd}\u{fffd}\u{fffd}");
        assert_eq!(decode("%ed%bf%bf", SET), "\u{fffd}\u{fffd}\u{fffd}");
        assert_eq!(decode("%ee%80%80", SET), "\u{e000}");
    }
}

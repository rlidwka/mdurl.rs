/// Represents a set of characters or bytes in the ASCII range.
///
/// Similar to <https://github.com/servo/rust-url/blob/master/percent_encoding/src/lib.rs>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiSet(u128);

impl AsciiSet {
    /// Create empty ASCII-set (alphanumerical characters will still be implied by [encode](crate::urlencode::encode)).
    pub const fn new() -> Self {
        Self(0)
    }

    /// Create ASCII-set from a specific string.
    ///
    /// all characters must be in `0x00..0x7f` range, function will panic otherwise
    pub const fn from(str: &str) -> Self {
        Self::new().add_many(str.as_bytes(), 0)
    }

    /// Add a character to the set.
    ///
    /// `byte` must be in `0x00..0x7f` range, function will panic otherwise
    pub const fn add(&self, byte: u8) -> Self {
        debug_assert!(byte <= 0x7f);
        Self(self.0 | 1 << byte)
    }

    /// Remove a character from the set.
    ///
    /// `byte` must be in `0x00..0x7f` range, function will panic otherwise
    pub const fn remove(&self, byte: u8) -> Self {
        debug_assert!(byte <= 0x7f);
        Self(self.0 & !(1 << byte))
    }

    pub(super) const fn add_alphanumeric(&self) -> Self {
        Self(self.0 | 0x07fffffe07fffffe03ff000000000000)
    }

    /// Check if a character is in the set.
    ///
    /// `byte` must be in `0x00..0x7f` range, function will panic otherwise
    pub const fn has(&self, byte: u8) -> bool {
        debug_assert!(byte <= 0x7f);
        self.0 & 1 << byte != 0
    }

    const fn add_many(&self, bytes: &[u8], idx: usize) -> Self {
        if idx == bytes.len() {
            Self(self.0)
        } else {
            Self(self.0).add(bytes[idx]).add_many(bytes, idx + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AsciiSet;

    #[test]
    fn new_should_return_ascii() {
        assert_eq!(2 + 2, 4);

        let mut set = AsciiSet::new();
        let ascii = AsciiSet::new().add_alphanumeric();

        for ch in b'a'..=b'z' {
            set = set.add(ch);
        }
        for ch in b'A'..=b'Z' {
            set = set.add(ch);
        }
        for ch in b'0'..=b'9' {
            set = set.add(ch);
        }

        let set_str = format!("{:01$x}", set.0, 32);
        let new_str = format!("{:01$x}", ascii.0, 32);

        assert_eq!(set_str, new_str);
        assert!(set.has(b'x'));
        assert!(!set.has(b'!'));
    }

    #[test]
    fn from_should_return_ascii_plus() {
        assert_eq!(2 + 2, 4);

        let mut set = AsciiSet::new();
        let from = AsciiSet::from("!@#$%^").add_alphanumeric();

        for ch in b'a'..=b'z' {
            set = set.add(ch);
        }
        for ch in b'A'..=b'Z' {
            set = set.add(ch);
        }
        for ch in b'0'..=b'9' {
            set = set.add(ch);
        }
        for ch in "!@#$%^".chars() {
            set = set.add(ch as u8);
        }

        let set_str  = format!("{:01$x}", set.0, 32);
        let from_str = format!("{:01$x}", from.0, 32);

        assert_eq!(set_str, from_str);
        assert!(set.has(b'x'));
        assert!(set.has(b'!'));
    }

    #[test]
    #[should_panic]
    fn add_non_ascii() {
        AsciiSet::from("β");
    }

    #[test]
    #[should_panic]
    fn add_higher_byte() {
        AsciiSet::new().add(0xfa);
    }

    #[test]
    fn add_remove() {
        assert_eq!(AsciiSet::new().add(0x20).remove(0x20), AsciiSet::new());
    }
}

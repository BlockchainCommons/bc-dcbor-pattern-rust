use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching byte string values in dCBOR.
#[derive(Debug, Clone)]
pub enum ByteStringPattern {
    /// Matches any byte string.
    Any,
    /// Matches the specific byte string.
    Value(Vec<u8>),
    /// Matches the binary regex for a byte string.
    Regex(regex::bytes::Regex),
}

impl PartialEq for ByteStringPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ByteStringPattern::Any, ByteStringPattern::Any) => true,
            (ByteStringPattern::Value(a), ByteStringPattern::Value(b)) => {
                a == b
            }
            (ByteStringPattern::Regex(a), ByteStringPattern::Regex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for ByteStringPattern {}

impl std::hash::Hash for ByteStringPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ByteStringPattern::Any => {
                0u8.hash(state);
            }
            ByteStringPattern::Value(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            ByteStringPattern::Regex(regex) => {
                2u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl ByteStringPattern {
    /// Creates a new `ByteStringPattern` that matches any byte string.
    pub fn any() -> Self { ByteStringPattern::Any }

    /// Creates a new `ByteStringPattern` that matches the specific byte string.
    pub fn value(value: impl AsRef<[u8]>) -> Self {
        ByteStringPattern::Value(value.as_ref().to_vec())
    }

    /// Creates a new `ByteStringPattern` that matches the binary regex for a
    /// byte string.
    pub fn regex(regex: regex::bytes::Regex) -> Self {
        ByteStringPattern::Regex(regex)
    }
}

impl Matcher for ByteStringPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        let is_hit = cbor.as_byte_string().is_some_and(|bytes| match self {
            ByteStringPattern::Any => true,
            ByteStringPattern::Value(want) => bytes == want,
            ByteStringPattern::Regex(regex) => regex.is_match(bytes),
        });

        if is_hit {
            vec![vec![cbor.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Value(
            crate::pattern::ValuePattern::ByteString(self.clone()),
        ));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for ByteStringPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteStringPattern::Any => write!(f, "BSTR"),
            ByteStringPattern::Value(value) => {
                write!(f, "BSTR(h'{}')", hex::encode(value))
            }
            ByteStringPattern::Regex(regex) => {
                write!(f, "BSTR(/{}/)", regex.as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_string_pattern_display() {
        assert_eq!(ByteStringPattern::any().to_string(), "BSTR");
        assert_eq!(
            ByteStringPattern::value(vec![1, 2, 3]).to_string(),
            r#"BSTR(h'010203')"#
        );
        assert_eq!(
            ByteStringPattern::regex(
                regex::bytes::Regex::new(r"^\d+$").unwrap()
            )
            .to_string(),
            r#"BSTR(/^\d+$/)"#
        );
    }

    #[test]
    fn test_byte_string_pattern_matching() {
        let hello_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let hello_cbor = CBOR::to_byte_string(hello_bytes.clone());
        let world_bytes = vec![0x57, 0x6f, 0x72, 0x6c, 0x64]; // "World"
        let world_cbor = CBOR::to_byte_string(world_bytes.clone());
        let digits_bytes = vec![0x31, 0x32, 0x33, 0x34, 0x35]; // "12345"
        let digits_cbor = CBOR::to_byte_string(digits_bytes.clone());
        let mixed_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33]; // "Hello123"
        let mixed_cbor = CBOR::to_byte_string(mixed_bytes.clone());
        let text_cbor = "Hello".to_cbor();

        // Test Any pattern
        let any_pattern = ByteStringPattern::any();
        assert!(any_pattern.matches(&hello_cbor));
        assert!(any_pattern.matches(&world_cbor));
        assert!(any_pattern.matches(&digits_cbor));
        assert!(any_pattern.matches(&mixed_cbor));
        assert!(!any_pattern.matches(&text_cbor));

        // Test specific value patterns
        let hello_pattern = ByteStringPattern::value(hello_bytes.clone());
        assert!(hello_pattern.matches(&hello_cbor));
        assert!(!hello_pattern.matches(&world_cbor));
        assert!(!hello_pattern.matches(&text_cbor));

        // Test regex patterns
        let digits_regex = regex::bytes::Regex::new(r"^\d+$").unwrap();
        let digits_pattern = ByteStringPattern::regex(digits_regex);
        assert!(!digits_pattern.matches(&hello_cbor));
        assert!(!digits_pattern.matches(&world_cbor));
        assert!(digits_pattern.matches(&digits_cbor));
        assert!(!digits_pattern.matches(&mixed_cbor));
        assert!(!digits_pattern.matches(&text_cbor));

        let alpha_regex = regex::bytes::Regex::new(r"^[A-Za-z]+$").unwrap();
        let alpha_pattern = ByteStringPattern::regex(alpha_regex);
        assert!(alpha_pattern.matches(&hello_cbor));
        assert!(alpha_pattern.matches(&world_cbor));
        assert!(!alpha_pattern.matches(&digits_cbor));
        assert!(!alpha_pattern.matches(&mixed_cbor));
        assert!(!alpha_pattern.matches(&text_cbor));
    }

    #[test]
    fn test_byte_string_pattern_paths() {
        let hello_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let hello_cbor = CBOR::to_byte_string(hello_bytes.clone());
        let text_cbor = "Hello".to_cbor();

        let any_pattern = ByteStringPattern::any();
        let hello_paths = any_pattern.paths(&hello_cbor);
        assert_eq!(hello_paths.len(), 1);
        assert_eq!(hello_paths[0].len(), 1);
        assert_eq!(hello_paths[0][0], hello_cbor);

        let text_paths = any_pattern.paths(&text_cbor);
        assert_eq!(text_paths.len(), 0);

        let hello_pattern = ByteStringPattern::value(hello_bytes.clone());
        let paths = hello_pattern.paths(&hello_cbor);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 1);
        assert_eq!(paths[0][0], hello_cbor);

        let no_match_paths = hello_pattern.paths(&text_cbor);
        assert_eq!(no_match_paths.len(), 0);
    }

    #[test]
    fn test_byte_string_pattern_equality() {
        let any1 = ByteStringPattern::any();
        let any2 = ByteStringPattern::any();
        let value1 = ByteStringPattern::value(vec![1, 2, 3]);
        let value2 = ByteStringPattern::value(vec![1, 2, 3]);
        let value3 = ByteStringPattern::value(vec![4, 5, 6]);
        let regex1 =
            ByteStringPattern::regex(regex::bytes::Regex::new(r"\d+").unwrap());
        let regex2 =
            ByteStringPattern::regex(regex::bytes::Regex::new(r"\d+").unwrap());
        let regex3 = ByteStringPattern::regex(
            regex::bytes::Regex::new(r"[a-z]+").unwrap(),
        );

        // Test equality
        assert_eq!(any1, any2);
        assert_eq!(value1, value2);
        assert_eq!(regex1, regex2);

        // Test inequality
        assert_ne!(any1, value1);
        assert_ne!(value1, value3);
        assert_ne!(regex1, regex3);
        assert_ne!(value1, regex1);
    }

    #[test]
    fn test_byte_string_pattern_regex_complex() {
        // Test with binary data that looks like an email pattern
        let email_bytes = b"test@example.com";
        let email_cbor = CBOR::to_byte_string(email_bytes);
        let not_email_bytes = b"not_an_email";
        let not_email_cbor = CBOR::to_byte_string(not_email_bytes);

        // Simple email regex pattern
        let email_regex =
            regex::bytes::Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
        let email_pattern = ByteStringPattern::regex(email_regex);

        assert!(email_pattern.matches(&email_cbor));
        assert!(!email_pattern.matches(&not_email_cbor));
    }

    #[test]
    fn test_byte_string_pattern_binary_data() {
        // Test with actual binary data (not text)
        let binary_data = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD];
        let binary_cbor = CBOR::to_byte_string(binary_data.clone());

        let any_pattern = ByteStringPattern::any();
        assert!(any_pattern.matches(&binary_cbor));

        let exact_pattern = ByteStringPattern::value(binary_data.clone());
        assert!(exact_pattern.matches(&binary_cbor));

        let different_pattern =
            ByteStringPattern::value(vec![0x00, 0x01, 0x02]);
        assert!(!different_pattern.matches(&binary_cbor));

        // Test regex that matches any bytes starting with 0x00
        let starts_with_zero_regex =
            regex::bytes::Regex::new(r"^\x00").unwrap();
        let starts_with_zero_pattern =
            ByteStringPattern::regex(starts_with_zero_regex);
        assert!(starts_with_zero_pattern.matches(&binary_cbor));

        // Test regex that doesn't match
        let starts_with_one_regex = regex::bytes::Regex::new(r"^\x01").unwrap();
        let starts_with_one_pattern =
            ByteStringPattern::regex(starts_with_one_regex);
        assert!(!starts_with_one_pattern.matches(&binary_cbor));
    }
}

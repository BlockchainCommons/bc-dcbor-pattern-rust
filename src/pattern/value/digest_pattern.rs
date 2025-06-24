use bc_components::{Digest, tags};
use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching dCBOR digest values (CBOR tag 40001).
#[derive(Debug, Clone)]
pub enum DigestPattern {
    /// Matches the exact digest.
    Digest(Digest),
    /// Matches the prefix of a digest (case insensitive).
    Prefix(Vec<u8>),
    /// Matches the binary regular expression for a digest.
    BinaryRegex(regex::bytes::Regex),
}

impl PartialEq for DigestPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DigestPattern::Digest(a), DigestPattern::Digest(b)) => a == b,
            (DigestPattern::Prefix(a), DigestPattern::Prefix(b)) => {
                a.eq_ignore_ascii_case(b)
            }
            (DigestPattern::BinaryRegex(a), DigestPattern::BinaryRegex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for DigestPattern {}

impl std::hash::Hash for DigestPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DigestPattern::Digest(a) => {
                0u8.hash(state);
                a.hash(state);
            }
            DigestPattern::Prefix(prefix) => {
                1u8.hash(state);
                prefix.hash(state);
            }
            DigestPattern::BinaryRegex(regex) => {
                2u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl DigestPattern {
    /// Creates a new `DigestPattern` that matches the exact digest.
    pub fn digest(digest: Digest) -> Self { DigestPattern::Digest(digest) }

    /// Creates a new `DigestPattern` that matches the prefix of a digest.
    pub fn prefix(prefix: impl AsRef<[u8]>) -> Self {
        DigestPattern::Prefix(prefix.as_ref().to_vec())
    }

    /// Creates a new `DigestPattern` that matches the binary regex for a
    /// digest.
    pub fn binary_regex(regex: regex::bytes::Regex) -> Self {
        DigestPattern::BinaryRegex(regex)
    }
}

impl Matcher for DigestPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // Check if the CBOR value is a tagged digest (tag 40001)
        if let CBORCase::Tagged(tag, content) = cbor.as_case() {
            if tag.value() == tags::TAG_DIGEST {
                // Try to extract the digest from the tagged content
                match CBOR::try_into_byte_string(content.clone()) {
                    Ok(digest_bytes) => {
                        if digest_bytes.len() == Digest::DIGEST_SIZE {
                            let is_hit = match self {
                                DigestPattern::Digest(pattern_digest) => {
                                    digest_bytes == pattern_digest.data()
                                }
                                DigestPattern::Prefix(prefix) => {
                                    digest_bytes.starts_with(prefix)
                                }
                                DigestPattern::BinaryRegex(regex) => {
                                    regex.is_match(&digest_bytes)
                                }
                            };

                            if is_hit {
                                return vec![vec![cbor.clone()]];
                            }
                        }
                    }
                    Err(_) => {
                        // Not a byte string, no match
                    }
                }
            }
        }

        vec![]
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Value(crate::pattern::ValuePattern::Digest(
            self.clone(),
        )));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for DigestPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DigestPattern::Digest(digest) => write!(f, "DIGEST({})", digest),
            DigestPattern::Prefix(prefix) => {
                write!(f, "DIGEST({})", hex::encode(prefix))
            }
            DigestPattern::BinaryRegex(regex) => {
                write!(f, "DIGEST(/{}/)", regex)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bc_components::{Digest, DigestProvider};

    use super::*;

    fn test_digest() -> Digest {
        let data: &[u8] = b"test data";
        data.digest().into_owned()
    }

    fn create_digest_cbor(digest: &Digest) -> CBOR { digest.to_cbor() }

    #[test]
    fn test_digest_pattern_display() {
        let digest = test_digest();
        let pattern = DigestPattern::digest(digest.clone());
        assert_eq!(format!("{}", pattern), format!("DIGEST({})", digest));

        let prefix = vec![0x74, 0x65, 0x73]; // "tes"
        let pattern = DigestPattern::prefix(prefix.clone());
        assert_eq!(
            format!("{}", pattern),
            format!("DIGEST({})", hex::encode(&prefix))
        );

        let regex = regex::bytes::Regex::new(r"^te.*").unwrap();
        let pattern = DigestPattern::binary_regex(regex.clone());
        assert_eq!(format!("{}", pattern), format!("DIGEST(/{}/)", regex));
    }

    #[test]
    fn test_digest_pattern_exact_match() {
        let digest = test_digest();
        let digest_cbor = create_digest_cbor(&digest);
        let pattern = DigestPattern::digest(digest.clone());

        assert!(pattern.matches(&digest_cbor));
        assert_eq!(
            pattern.paths(&digest_cbor),
            vec![vec![digest_cbor.clone()]]
        );

        // Test with different digest
        let other_data: &[u8] = b"other data";
        let other_digest = other_data.digest().into_owned();
        let other_digest_cbor = create_digest_cbor(&other_digest);
        assert!(!pattern.matches(&other_digest_cbor));
        assert!(pattern.paths(&other_digest_cbor).is_empty());
    }

    #[test]
    fn test_digest_pattern_prefix_match() {
        let digest = test_digest();
        let digest_cbor = create_digest_cbor(&digest);

        // Test prefix matching - use first 4 bytes of the digest
        let prefix = digest.data()[..4].to_vec();
        let pattern = DigestPattern::prefix(prefix);

        assert!(pattern.matches(&digest_cbor));
        assert_eq!(
            pattern.paths(&digest_cbor),
            vec![vec![digest_cbor.clone()]]
        );

        // Test with digest that doesn't match the prefix
        let other_data: &[u8] = b"completely different data";
        let other_digest = other_data.digest().into_owned();
        let other_digest_cbor = create_digest_cbor(&other_digest);

        // Only match if the other digest happens to have the same prefix
        // (unlikely)
        let matches = pattern.matches(&other_digest_cbor);
        assert_eq!(
            matches,
            other_digest.data().starts_with(&digest.data()[..4])
        );
    }

    #[test]
    fn test_digest_pattern_regex_match() {
        let digest = test_digest();
        let digest_cbor = create_digest_cbor(&digest);

        // Create a regex that matches any binary data (any sequence of bytes)
        let match_all_regex = regex::bytes::Regex::new(r".*").unwrap();
        let pattern = DigestPattern::binary_regex(match_all_regex);

        assert!(pattern.matches(&digest_cbor));
        assert_eq!(
            pattern.paths(&digest_cbor),
            vec![vec![digest_cbor.clone()]]
        );

        // Test with a regex that definitely won't match - look for bytes that
        // don't exist
        let no_match_regex =
            regex::bytes::Regex::new(r"^\xFF\xFF\xFF\xFF").unwrap();
        let no_match_pattern = DigestPattern::binary_regex(no_match_regex);

        // This should only match if the digest happens to start with 0xFFFFFFFF
        // (very unlikely)
        let matches = no_match_pattern.matches(&digest_cbor);
        assert_eq!(
            matches,
            digest.data().starts_with(&[0xFF, 0xFF, 0xFF, 0xFF])
        );
    }

    #[test]
    fn test_digest_pattern_non_digest_cbor() {
        let pattern = DigestPattern::digest(test_digest());

        // Test with non-digest CBOR values
        let text_cbor = "hello".to_cbor();
        let number_cbor = 42.to_cbor();
        let array_cbor = vec![1, 2, 3].to_cbor();

        assert!(!pattern.matches(&text_cbor));
        assert!(!pattern.matches(&number_cbor));
        assert!(!pattern.matches(&array_cbor));

        assert!(pattern.paths(&text_cbor).is_empty());
        assert!(pattern.paths(&number_cbor).is_empty());
        assert!(pattern.paths(&array_cbor).is_empty());
    }

    #[test]
    fn test_digest_pattern_equality() {
        let digest1 = test_digest();
        let digest2 = test_digest();
        assert_eq!(digest1, digest2); // Same data produces same digest

        let pattern1 = DigestPattern::digest(digest1);
        let pattern2 = DigestPattern::digest(digest2);
        assert_eq!(pattern1, pattern2);

        let prefix = vec![0x12, 0x34];
        let prefix_pattern1 = DigestPattern::prefix(&prefix);
        let prefix_pattern2 = DigestPattern::prefix(&prefix);
        assert_eq!(prefix_pattern1, prefix_pattern2);

        let regex = regex::bytes::Regex::new(r"^test").unwrap();
        let regex_pattern1 = DigestPattern::binary_regex(regex.clone());
        let regex_pattern2 = DigestPattern::binary_regex(regex);
        assert_eq!(regex_pattern1, regex_pattern2);
    }
}

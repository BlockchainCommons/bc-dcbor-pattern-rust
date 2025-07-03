use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching text values in dCBOR.
#[derive(Debug, Clone)]
pub enum TextPattern {
    /// Matches any text.
    Any,
    /// Matches the specific text.
    Value(String),
    /// Matches the regex for a text.
    Regex(regex::Regex),
}

impl PartialEq for TextPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TextPattern::Any, TextPattern::Any) => true,
            (TextPattern::Value(a), TextPattern::Value(b)) => a == b,
            (TextPattern::Regex(a), TextPattern::Regex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for TextPattern {}

impl std::hash::Hash for TextPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            TextPattern::Any => {
                0u8.hash(state);
            }
            TextPattern::Value(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            TextPattern::Regex(regex) => {
                2u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl TextPattern {
    /// Creates a new `TextPattern` that matches any text.
    pub fn any() -> Self { TextPattern::Any }

    /// Creates a new `TextPattern` that matches the specific text.
    pub fn value<T: Into<String>>(value: T) -> Self {
        TextPattern::Value(value.into())
    }

    /// Creates a new `TextPattern` that matches the regex for a text.
    pub fn regex(regex: regex::Regex) -> Self { TextPattern::Regex(regex) }
}

impl Matcher for TextPattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        let is_hit = haystack.as_text().is_some_and(|value| match self {
            TextPattern::Any => true,
            TextPattern::Value(want) => value == *want,
            TextPattern::Regex(regex) => regex.is_match(value),
        });

        if is_hit {
            vec![vec![haystack.clone()]]
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
        literals.push(Pattern::Value(crate::pattern::ValuePattern::Text(
            self.clone(),
        )));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for TextPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextPattern::Any => write!(f, "text"),
            TextPattern::Value(value) => {
                let escaped = value.replace("\\", "\\\\").replace("\"", "\\\"");
                write!(f, "\"{}\"", escaped)
            }
            TextPattern::Regex(regex) => write!(f, "/{}/", regex),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_pattern_display() {
        assert_eq!(TextPattern::any().to_string(), "text");
        assert_eq!(TextPattern::value("Hello").to_string(), "\"Hello\"");
        assert_eq!(
            TextPattern::regex(regex::Regex::new(r"^\d+$").unwrap())
                .to_string(),
            "/^\\d+$/"
        );
    }

    #[test]
    fn test_text_pattern_matching() {
        let hello_cbor = "Hello".to_cbor();
        let world_cbor = "World".to_cbor();
        let digits_cbor = "12345".to_cbor();
        let mixed_cbor = "Hello123".to_cbor();
        let number_cbor = 42.to_cbor();

        // Test Any pattern
        let any_pattern = TextPattern::any();
        assert!(any_pattern.matches(&hello_cbor));
        assert!(any_pattern.matches(&world_cbor));
        assert!(any_pattern.matches(&digits_cbor));
        assert!(any_pattern.matches(&mixed_cbor));
        assert!(!any_pattern.matches(&number_cbor));

        // Test specific value patterns
        let hello_pattern = TextPattern::value("Hello");
        assert!(hello_pattern.matches(&hello_cbor));
        assert!(!hello_pattern.matches(&world_cbor));
        assert!(!hello_pattern.matches(&number_cbor));

        // Test regex patterns
        let digits_regex = regex::Regex::new(r"^\d+$").unwrap();
        let digits_pattern = TextPattern::regex(digits_regex);
        assert!(!digits_pattern.matches(&hello_cbor));
        assert!(!digits_pattern.matches(&world_cbor));
        assert!(digits_pattern.matches(&digits_cbor));
        assert!(!digits_pattern.matches(&mixed_cbor));
        assert!(!digits_pattern.matches(&number_cbor));

        let word_regex = regex::Regex::new(r"^[A-Za-z]+$").unwrap();
        let word_pattern = TextPattern::regex(word_regex);
        assert!(word_pattern.matches(&hello_cbor));
        assert!(word_pattern.matches(&world_cbor));
        assert!(!word_pattern.matches(&digits_cbor));
        assert!(!word_pattern.matches(&mixed_cbor));
        assert!(!word_pattern.matches(&number_cbor));
    }

    #[test]
    fn test_text_pattern_paths() {
        let hello_cbor = "Hello".to_cbor();
        let number_cbor = 42.to_cbor();

        let any_pattern = TextPattern::any();
        let hello_paths = any_pattern.paths(&hello_cbor);
        assert_eq!(hello_paths.len(), 1);
        assert_eq!(hello_paths[0].len(), 1);
        assert_eq!(hello_paths[0][0], hello_cbor);

        let number_paths = any_pattern.paths(&number_cbor);
        assert_eq!(number_paths.len(), 0);

        let hello_pattern = TextPattern::value("Hello");
        let paths = hello_pattern.paths(&hello_cbor);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 1);
        assert_eq!(paths[0][0], hello_cbor);

        let no_match_paths = hello_pattern.paths(&number_cbor);
        assert_eq!(no_match_paths.len(), 0);
    }

    #[test]
    fn test_text_pattern_equality() {
        let any1 = TextPattern::any();
        let any2 = TextPattern::any();
        let value1 = TextPattern::value("test");
        let value2 = TextPattern::value("test");
        let value3 = TextPattern::value("different");
        let regex1 = TextPattern::regex(regex::Regex::new(r"\d+").unwrap());
        let regex2 = TextPattern::regex(regex::Regex::new(r"\d+").unwrap());
        let regex3 = TextPattern::regex(regex::Regex::new(r"[a-z]+").unwrap());

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
    fn test_text_pattern_regex_complex() {
        let email_cbor = "test@example.com".to_cbor();
        let not_email_cbor = "not_an_email".to_cbor();

        // Simple email regex pattern
        let email_regex = regex::Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
        let email_pattern = TextPattern::regex(email_regex);

        assert!(email_pattern.matches(&email_cbor));
        assert!(!email_pattern.matches(&not_email_cbor));
    }
}

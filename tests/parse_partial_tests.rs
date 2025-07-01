#[cfg(test)]
mod parse_partial_tests {
    use dcbor_pattern::{Pattern, Error};

    #[test]
    fn test_parse_partial_basic() {
        let (pattern, consumed) = Pattern::parse_partial("true rest").unwrap();
        assert_eq!(pattern, Pattern::bool(true));
        assert_eq!(consumed, 5); // "true ".len() - includes whitespace that is skipped
    }

    #[test]
    fn test_parse_partial_with_whitespace() {
        let (pattern, consumed) = Pattern::parse_partial("42    more stuff").unwrap();
        assert_eq!(pattern, Pattern::number(42));
        assert_eq!(consumed, 6); // "42    ".len() - includes whitespace that is skipped
    }

    #[test]
    fn test_parse_partial_complete_input() {
        let (pattern, consumed) = Pattern::parse_partial("false").unwrap();
        assert_eq!(pattern, Pattern::bool(false));
        assert_eq!(consumed, 5); // "false".len()
    }

    #[test]
    fn test_parse_partial_complex_pattern() {
        let (_pattern, consumed) = Pattern::parse_partial("number | text additional").unwrap();
        // Should parse "number | text" and stop before "additional"
        assert!(consumed > 10); // At least "number | text".len()
        assert!(consumed < "number | text additional".len()); // But not the full string
    }

    #[test]
    fn test_parse_full_compatibility() {
        // Existing behavior should still work
        assert!(Pattern::parse("true").is_ok());

        // Should still return error for extra data (backward compatibility)
        let result = Pattern::parse("true extra");
        match result {
            Err(Error::ExtraData(_)) => (), // Expected
            other => panic!("Expected ExtraData error, got: {:?}", other),
        }
    }

    #[test]
    fn test_parse_partial_with_valid_following_token() {
        let (pattern, consumed) = Pattern::parse_partial("true false").unwrap();
        assert_eq!(pattern, Pattern::bool(true));
        assert_eq!(consumed, 5); // "true ".len() - includes whitespace

        // Should be able to parse the rest
        let remaining = &"true false"[consumed..];
        let (pattern2, consumed2) = Pattern::parse_partial(remaining).unwrap();
        assert_eq!(pattern2, Pattern::bool(false));
        assert_eq!(consumed2, 5); // "false".len() (no trailing whitespace)
    }

    #[test]
    fn test_parse_partial_error_cases() {
        // Invalid pattern should still error
        let result = Pattern::parse_partial("invalid_pattern");
        assert!(result.is_err());

        // Empty input should error
        let result = Pattern::parse_partial("");
        assert!(result.is_err());
    }
}

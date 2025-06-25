use crate::{ArrayPattern, Error, Pattern, Result, parse::Token};

/// Parse an ARRAY pattern.
///
/// Supports the following syntax:
/// - `ARRAY` - matches any array
/// - `ARRAY({n})` - matches array with exactly n elements
/// - `ARRAY({n,m})` - matches array with n to m elements (inclusive)
/// - `ARRAY({n,})` - matches array with at least n elements
/// - `ARRAY(pattern)` - matches array with elements matching the given pattern
pub(crate) fn parse_array(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // Consume the '(' token
            lexer.next();

            // Peek at the next token to determine what we're parsing
            let mut second_lookahead = lexer.clone();
            match second_lookahead.next() {
                Some(Ok(Token::Range(res))) => {
                    // This is a quantifier syntax: ARRAY({n}), ARRAY({n,m}),
                    // etc.
                    let quantifier = res?;
                    lexer.next(); // consume the Range token

                    // Convert quantifier to appropriate ArrayPattern
                    let pattern = if let Some(max) = quantifier.max() {
                        if quantifier.min() == max {
                            // Exact count: {n}
                            ArrayPattern::with_length(quantifier.min())
                        } else {
                            // Range: {n,m}
                            ArrayPattern::with_length_range(
                                quantifier.min()..=max,
                            )
                        }
                    } else {
                        // Open-ended range: {n,}
                        ArrayPattern::with_length_range(
                            quantifier.min()..=usize::MAX,
                        )
                    };

                    // Expect closing parenthesis
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(Pattern::Structure(
                            crate::pattern::StructurePattern::Array(pattern),
                        )),
                        Some(Ok(token)) => Err(Error::UnexpectedToken(
                            Box::new(token),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                _ => {
                    // This is a pattern syntax: ARRAY(pattern)
                    // Parse the inner pattern using the full pattern syntax
                    let element_pattern = super::super::meta::parse_or(lexer)?;
                    let pattern = ArrayPattern::with_elements(element_pattern);

                    // Expect closing parenthesis
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(Pattern::Structure(
                            crate::pattern::StructurePattern::Array(pattern),
                        )),
                        Some(Ok(token)) => Err(Error::UnexpectedToken(
                            Box::new(token),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
            }
        }
        _ => {
            // No parentheses, just "ARRAY" - matches any array
            Ok(Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::any(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_array_any() {
        let pattern = Pattern::parse("ARRAY").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::any()
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY");
    }

    #[test]
    fn test_parse_array_exact_count() {
        let pattern = Pattern::parse("ARRAY({3})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length(3)
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY({3})");
    }

    #[test]
    fn test_parse_array_range() {
        let pattern = Pattern::parse("ARRAY({2,5})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length_range(2..=5)
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY({2,5})");
    }

    #[test]
    fn test_parse_array_open_range() {
        let pattern = Pattern::parse("ARRAY({2,})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length_range(2..=usize::MAX)
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY({2,})");
    }

    #[test]
    fn test_parse_array_error_missing_close_paren() {
        let result = Pattern::parse("ARRAY({3}");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_array_error_invalid_range() {
        let result = Pattern::parse("ARRAY(invalid)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_array_with_pattern_simple() {
        let pattern = Pattern::parse("ARRAY(NUMBER(42))").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(Pattern::number(42))
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY(NUMBER(42))");
    }

    #[test]
    fn test_parse_array_with_pattern_text() {
        let pattern = Pattern::parse(r#"ARRAY(TEXT("hello"))"#).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(Pattern::text("hello"))
            ))
        );
        assert_eq!(pattern.to_string(), r#"ARRAY(TEXT("hello"))"#);
    }

    #[test]
    fn test_array_pattern_matching() {
        use dcbor_parse::parse_dcbor_item;

        use crate::pattern::Matcher;

        // Test 1: Simple single element matching
        let pattern =
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(Pattern::number(42)),
            ));

        // Should match array with exactly one element: 42
        let cbor_single = parse_dcbor_item("[42]").unwrap();
        assert!(
            pattern.matches(&cbor_single),
            "ARRAY(NUMBER(42)) should match [42]"
        );

        // Should NOT match array with multiple elements including 42
        let cbor_multi = parse_dcbor_item("[1, 42, 3]").unwrap();
        assert!(
            !pattern.matches(&cbor_multi),
            "ARRAY(NUMBER(42)) should NOT match [1, 42, 3]"
        );

        // Should NOT match array without 42
        let cbor_no_match = parse_dcbor_item("[1, 2, 3]").unwrap();
        assert!(
            !pattern.matches(&cbor_no_match),
            "ARRAY(NUMBER(42)) should NOT match [1, 2, 3]"
        );

        // Should NOT match empty array
        let cbor_empty = parse_dcbor_item("[]").unwrap();
        assert!(
            !pattern.matches(&cbor_empty),
            "ARRAY(NUMBER(42)) should NOT match []"
        );

        // Test 2: Sequence matching
        let sequence_pattern =
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(Pattern::sequence(vec![
                    Pattern::text("a"),
                    Pattern::text("b"),
                ])),
            ));

        // Should match exact sequence
        let cbor_seq_match = parse_dcbor_item(r#"["a", "b"]"#).unwrap();
        assert!(
            sequence_pattern.matches(&cbor_seq_match),
            r#"ARRAY(TEXT("a") > TEXT("b")) should match ["a", "b"]"#
        );

        // Should NOT match wrong order
        let cbor_seq_wrong = parse_dcbor_item(r#"["b", "a"]"#).unwrap();
        assert!(
            !sequence_pattern.matches(&cbor_seq_wrong),
            r#"ARRAY(TEXT("a") > TEXT("b")) should NOT match ["b", "a"]"#
        );

        // Should NOT match partial sequence
        let cbor_seq_partial = parse_dcbor_item(r#"["a"]"#).unwrap();
        assert!(
            !sequence_pattern.matches(&cbor_seq_partial),
            r#"ARRAY(TEXT("a") > TEXT("b")) should NOT match ["a"]"#
        );

        // Should NOT match longer sequence
        let cbor_seq_long = parse_dcbor_item(r#"["a", "b", "c"]"#).unwrap();
        assert!(
            !sequence_pattern.matches(&cbor_seq_long),
            r#"ARRAY(TEXT("a") > TEXT("b")) should NOT match ["a", "b", "c"]"#
        );

        // Test 3: Or pattern (should still work with individual element
        // matching)
        let or_pattern =
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(Pattern::or(vec![
                    Pattern::any_number(),
                    Pattern::any_text(),
                ])),
            ));

        // This should work with the fallback logic for meta patterns
        let cbor_or_match = parse_dcbor_item("[42]").unwrap();
        let or_matches = or_pattern.matches(&cbor_or_match);
        println!("ARRAY(NUMBER | TEXT) matches [42]: {}", or_matches);
    }

    #[test]
    fn test_parse_array_pattern_precedence() {
        // Test that quantifier syntax takes precedence over pattern syntax
        let quantifier_pattern = Pattern::parse("ARRAY({3})").unwrap();
        assert_eq!(
            quantifier_pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length(3)
            ))
        );

        // Test that non-quantifier patterns are parsed as element patterns
        let element_pattern = Pattern::parse("ARRAY(NUMBER)").unwrap();
        assert_eq!(
            element_pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(Pattern::any_number())
            ))
        );
    }

    #[test]
    fn test_parse_array_pattern_with_parentheses() {
        let pattern = Pattern::parse("ARRAY((NUMBER))").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(Pattern::any_number())
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY(NUMBER)");
    }

    #[test]
    fn test_parse_array_error_unclosed_parentheses() {
        let result = Pattern::parse("ARRAY(NUMBER");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_array_error_empty_parentheses() {
        let result = Pattern::parse("ARRAY()");
        assert!(result.is_err());
    }

    #[test]
    fn test_repeat_pattern_support() {
        use dcbor_parse::parse_dcbor_item;

        use crate::pattern::Matcher;

        // Test 1: Can we create repeat patterns programmatically?
        let any_star = Pattern::repeat(
            Pattern::any(),
            crate::Quantifier::new(0..=usize::MAX, crate::Reluctance::Greedy),
        );
        println!("✅ Can create (ANY)* pattern: {}", any_star);

        // Test 2: Can we create sequences with repeats?
        let sequence_with_repeats = Pattern::sequence(vec![
            any_star.clone(),
            Pattern::number(42),
            any_star.clone(),
        ]);
        println!(
            "✅ Can create sequence with repeats: {}",
            sequence_with_repeats
        );

        // Test 3: Can we create ARRAY patterns with sequences containing
        // repeats?
        let array_with_repeats =
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_elements(sequence_with_repeats.clone()),
            ));
        println!(
            "✅ Can create ARRAY with repeat sequence: {}",
            array_with_repeats
        );

        // Test 4: Test matching behavior (this will show current limitations)
        let test_cases = [
            ("[42]", "Just 42"),
            ("[1, 42]", "42 at end"),
            ("[42, 1]", "42 at start"),
            ("[1, 42, 3]", "42 in middle"),
            ("[1, 2, 3]", "No 42"),
            ("[]", "Empty array"),
        ];

        println!("\n--- Testing ARRAY((ANY)*>NUMBER(42)>(ANY)*) matching ---");

        for (cbor_text, description) in &test_cases {
            let cbor = parse_dcbor_item(cbor_text).unwrap();
            let matches = array_with_repeats.matches(&cbor);
            println!(
                "{} ({}): {}",
                cbor_text,
                description,
                if matches { "✅ MATCH" } else { "❌ NO MATCH" }
            );
        }

        // Note: This test documents current behavior, not necessarily desired
        // behavior In the unified syntax,
        // ARRAY((ANY)*>NUMBER(42)>(ANY)*) should match any array containing 42
        // But our current implementation may not support this yet
    }
}

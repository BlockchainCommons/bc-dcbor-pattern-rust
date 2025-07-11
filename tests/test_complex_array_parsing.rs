#[cfg(test)]
mod tests {
    use dcbor_parse::parse_dcbor_item;
    use dcbor_pattern::{Matcher, Pattern};

    #[test]
    fn test_complex_array_pattern_text_parsing() {
        // Test if complex array pattern parsing works from text
        let pattern_text = r#"[(*)*, 42, (*)*]"#;

        let pattern =
            Pattern::parse(pattern_text).expect("Should parse complex pattern");
        println!("✅ Successfully parsed: {}", pattern);

        // Test matching
        let test_cases = [
            ("[42]", "Just 42", true),
            ("[1, 42]", "42 at end", true),
            ("[42, 1]", "42 at start", true),
            ("[1, 42, 3]", "42 in middle", true),
            ("[1, 2, 3]", "No 42", false),
            ("[]", "Empty array", false),
        ];

        println!("\n--- Testing parsed pattern matching ---");
        for (cbor_text, _description, expected_match) in &test_cases {
            let cbor = parse_dcbor_item(cbor_text).unwrap();
            let matches = pattern.matches(&cbor);
            // println!(
            //     "{} ({}): {}",
            //     cbor_text,
            //     description,
            //     if matches { "✅ MATCH" } else { "❌ NO MATCH" }
            // );

            assert_eq!(
                matches, *expected_match,
                "Pattern matching for {} should be {}",
                cbor_text, expected_match
            );
        }
    }

    #[test]
    fn test_various_repeat_quantifiers_in_arrays() {
        let test_patterns = [
            ("[(*)+]", "One or more *", "[1]", true),
            ("[(*)+]", "One or more * empty", "[]", false),
            ("[(*)?]", "Zero or one *", "[]", true),
            ("[(*)?]", "Zero or one * single", "[1]", true),
            ("[(*)?]", "Zero or one * multiple", "[1,2]", false),
            ("[(number)*]", "Zero or more numbers", "[]", true),
            (
                "[(number)*]",
                "Zero or more numbers with nums",
                "[1,2,3]",
                true,
            ),
            (
                "[(number)*]",
                "Zero or more numbers with text",
                r#"["hello"]"#,
                false,
            ),
        ];

        for (pattern_text, description, cbor_text, expected_match) in
            &test_patterns
        {
            println!("Testing: {} - {}", description, pattern_text);
            let pattern =
                Pattern::parse(pattern_text).expect("Should parse pattern");
            let cbor = parse_dcbor_item(cbor_text).unwrap();
            let matches = pattern.matches(&cbor);

            assert_eq!(
                matches, *expected_match,
                "Pattern '{}' for {} should be {}",
                pattern_text, cbor_text, expected_match
            );
        }
    }

    #[test]
    fn test_nested_array_patterns_with_repeats() {
        // Test nested patterns with complex repeats
        let pattern_text = r#"[[(number)*], (*)*]"#;
        let pattern =
            Pattern::parse(pattern_text).expect("Should parse nested pattern");

        let test_cases = [
            ("[[1,2,3]]", "Single number array", true),
            ("[[1,2,3], 42]", "Number array followed by number", true),
            (
                "[[1,2,3], \"hello\"]",
                "Number array followed by text",
                true,
            ),
            ("[[], 42]", "Empty array followed by number", true),
            (r#"[["hello"], 42]"#, "Text array followed by number", false), /* First element has text */
        ];

        for (cbor_text, description, expected_match) in &test_cases {
            let cbor = parse_dcbor_item(cbor_text).unwrap();
            let matches = pattern.matches(&cbor);
            println!(
                "{} ({}): {}",
                cbor_text,
                description,
                if matches { "✅ MATCH" } else { "❌ NO MATCH" }
            );

            assert_eq!(
                matches, *expected_match,
                "Nested pattern for {} should be {}",
                cbor_text, expected_match
            );
        }
    }

    #[test]
    fn test_simple_array_patterns_still_work() {
        // Ensure simple patterns still work after our changes
        let test_patterns = [
            ("[*]", "[]", false), /* [*] now means single element, not any
                                   * array */
            ("[*]", "[1]", true), // [*] matches single element arrays
            ("[*]", "[1,2,3]", false), // [*] doesn't match multi-element arrays
            ("array", "[]", true), // array keyword matches any array
            ("array", "[1,2,3]", true), // array keyword matches any array
            ("[{3}]", "[1,2,3]", true),
            ("[{3}]", "[1,2]", false),
            ("[number]", "[42]", true),
            ("[number]", "[42,43]", false), // Single element only
            ("[text]", r#"["hello"]"#, true),
        ];

        for (pattern_text, cbor_text, expected_match) in &test_patterns {
            let pattern = Pattern::parse(pattern_text)
                .expect("Should parse simple pattern");
            let cbor = parse_dcbor_item(cbor_text).unwrap();
            let matches = pattern.matches(&cbor);

            assert_eq!(
                matches, *expected_match,
                "Simple pattern '{}' for {} should be {}",
                pattern_text, cbor_text, expected_match
            );
        }
    }
}

mod common;

use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR { parse_dcbor_item(s).unwrap() }

#[test]
fn test_any_pattern() {
    let pattern = Pattern::any();

    // Should match all types of CBOR values
    let number_cbor = cbor("42");
    let paths = pattern.paths(&number_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    let text_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&text_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    let bool_cbor = cbor("true");
    let paths = pattern.paths(&bool_cbor);
    let expected = "true";
    assert_actual_expected!(format_paths(&paths), expected);

    let array_cbor = cbor("[1, 2, 3]");
    let paths = pattern.paths(&array_cbor);
    let expected = "[1, 2, 3]";
    assert_actual_expected!(format_paths(&paths), expected);

    let null_cbor = cbor("null");
    let paths = pattern.paths(&null_cbor);
    let expected = "null";
    assert_actual_expected!(format_paths(&paths), expected);

    // Display should show ANY
    assert_eq!(pattern.to_string(), "ANY");
}

#[test]
fn test_none_pattern() {
    let pattern = Pattern::none();

    // Should never match any CBOR value
    assert!(!pattern.matches(&cbor("42")));
    assert!(!pattern.matches(&cbor(r#""hello""#)));
    assert!(!pattern.matches(&cbor("true")));
    assert!(!pattern.matches(&cbor("[1, 2, 3]")));
    assert!(!pattern.matches(&cbor("null")));

    // Display should show NONE
    assert_eq!(pattern.to_string(), "NONE");
}

#[test]
fn test_and_pattern() {
    let pattern = Pattern::and(vec![
        Pattern::number_greater_than(5),
        Pattern::number_less_than(10),
    ]);

    // Should match values that satisfy all conditions
    let seven_cbor = cbor("7");
    let paths = pattern.paths(&seven_cbor);
    let expected = "7";
    assert_actual_expected!(format_paths(&paths), expected);

    let six_cbor = cbor("6");
    let paths = pattern.paths(&six_cbor);
    let expected = "6";
    assert_actual_expected!(format_paths(&paths), expected);

    let nine_cbor = cbor("9");
    let paths = pattern.paths(&nine_cbor);
    let expected = "9";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match values that fail any condition
    assert!(!pattern.matches(&cbor("3"))); // < 5
    assert!(!pattern.matches(&cbor("12"))); // > 10
    assert!(!pattern.matches(&cbor(r#""hello""#))); // not a number

    // Display should use & operator
    assert_eq!(pattern.to_string(), ">5&<10");
}

#[test]
fn test_or_pattern() {
    let pattern = Pattern::or(vec![
        Pattern::number(5),
        Pattern::text("hello"),
        Pattern::bool(true),
    ]);

    // Should match values that satisfy any condition
    let five_cbor = cbor("5");
    let paths = pattern.paths(&five_cbor);
    let expected = "5";
    assert_actual_expected!(format_paths(&paths), expected);

    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    let true_cbor = cbor("true");
    let paths = pattern.paths(&true_cbor);
    let expected = "true";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match values that don't satisfy any condition
    assert!(!pattern.matches(&cbor("42")));
    assert!(!pattern.matches(&cbor(r#""world""#)));
    assert!(!pattern.matches(&cbor("false")));

    // Display should use | operator
    assert_eq!(pattern.to_string(), r#"5|"hello"|true"#);
}

#[test]
fn test_not_pattern() {
    let pattern = Pattern::not_matching(Pattern::number(5));

    // Should match values that don't match the inner pattern
    let forty_two_cbor = cbor("42");
    let paths = pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    let true_cbor = cbor("true");
    let paths = pattern.paths(&true_cbor);
    let expected = "true";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match the exact value
    assert!(!pattern.matches(&cbor("5")));

    // Display should use ! operator
    assert_eq!(pattern.to_string(), "!5");
}

#[test]
fn test_not_pattern_complex() {
    let inner = Pattern::and(vec![
        Pattern::number_greater_than(5),
        Pattern::number_less_than(10),
    ]);
    let pattern = Pattern::not_matching(inner);

    // Should match values outside the range
    let three_cbor = cbor("3");
    let paths = pattern.paths(&three_cbor);
    let expected = "3";
    assert_actual_expected!(format_paths(&paths), expected);

    let twelve_cbor = cbor("12");
    let paths = pattern.paths(&twelve_cbor);
    let expected = "12";
    assert_actual_expected!(format_paths(&paths), expected);

    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match values in the range
    assert!(!pattern.matches(&cbor("7")));

    // Display should wrap complex patterns in parentheses
    assert_eq!(pattern.to_string(), "!(>5&<10)");
}

#[test]
fn test_nested_meta_patterns() {
    // (number > 5 AND number < 10) OR text = "hello"
    let pattern = Pattern::or(vec![
        Pattern::and(vec![
            Pattern::number_greater_than(5),
            Pattern::number_less_than(10),
        ]),
        Pattern::text("hello"),
    ]);

    // Should match numbers in range
    let seven_cbor = cbor("7");
    let paths = pattern.paths(&seven_cbor);
    let expected = "7";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match the specific text
    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match numbers outside range or other text
    assert!(!pattern.matches(&cbor("3")));
    assert!(!pattern.matches(&cbor("12")));
    assert!(!pattern.matches(&cbor(r#""world""#)));

    // Display should properly nest the operators
    assert_eq!(
        pattern.to_string(),
        r#">5&<10|"hello""#
    );
}

#[test]
fn test_empty_and_pattern() {
    let pattern = Pattern::and(vec![]);

    // Empty AND should match everything (vacuous truth)
    assert!(pattern.matches(&cbor("42")));
    assert!(pattern.matches(&cbor(r#""hello""#)));

    // Display should be empty string
    assert_eq!(pattern.to_string(), "");
}

#[test]
fn test_empty_or_pattern() {
    let pattern = Pattern::or(vec![]);

    // Empty OR should match nothing
    assert!(!pattern.matches(&cbor("42")));
    assert!(!pattern.matches(&cbor(r#""hello""#)));

    // Display should be empty string
    assert_eq!(pattern.to_string(), "");
}

#[test]
fn test_capture_pattern_basic() {
    let pattern = Pattern::capture("test", Pattern::number(42));

    // Should match the same things as the inner pattern
    let forty_two_cbor = cbor("42");
    let paths = pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other values
    assert!(!pattern.matches(&cbor("43")));
    assert!(!pattern.matches(&cbor(r#""hello""#)));

    // Display should show capture syntax
    assert_eq!(pattern.to_string(), "@test(42)");
}

#[test]
fn test_capture_pattern_text() {
    let pattern = Pattern::capture("name", Pattern::text("hello"));

    // Should match the same things as the inner pattern
    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other values
    assert!(!pattern.matches(&cbor(r#""world""#)));
    assert!(!pattern.matches(&cbor("42")));

    // Display should show capture syntax
    assert_eq!(pattern.to_string(), r#"@name("hello")"#);
}

#[test]
fn test_capture_pattern_any() {
    let pattern = Pattern::capture("anything", Pattern::any());

    // Should match anything since inner pattern is ANY
    let forty_two_cbor = cbor("42");
    let paths = pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    let true_cbor = cbor("true");
    let paths = pattern.paths(&true_cbor);
    let expected = "true";
    assert_actual_expected!(format_paths(&paths), expected);

    let array_cbor = cbor("[1, 2, 3]");
    let paths = pattern.paths(&array_cbor);
    let expected = "[1, 2, 3]";
    assert_actual_expected!(format_paths(&paths), expected);

    // Display should show capture syntax
    assert_eq!(pattern.to_string(), "@anything(ANY)");
}

#[test]
fn test_capture_pattern_none() {
    let pattern = Pattern::capture("nothing", Pattern::none());

    // Should never match since inner pattern is NONE
    assert!(!pattern.matches(&cbor("42")));
    assert!(!pattern.matches(&cbor(r#""hello""#)));
    assert!(!pattern.matches(&cbor("true")));

    // Display should show capture syntax
    assert_eq!(pattern.to_string(), "@nothing(NONE)");
}

#[test]
fn test_capture_pattern_complex() {
    let pattern = Pattern::capture(
        "range",
        Pattern::and(vec![
            Pattern::number_greater_than(5),
            Pattern::number_less_than(10),
        ]),
    );

    // Should match numbers in range 5 < x < 10
    let seven_cbor = cbor("7");
    let paths = pattern.paths(&seven_cbor);
    let expected = "7";
    assert_actual_expected!(format_paths(&paths), expected);

    let six_cbor = cbor("6");
    let paths = pattern.paths(&six_cbor);
    let expected = "6";
    assert_actual_expected!(format_paths(&paths), expected);

    let nine_cbor = cbor("9");
    let paths = pattern.paths(&nine_cbor);
    let expected = "9";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match values outside range
    assert!(!pattern.matches(&cbor("5")));
    assert!(!pattern.matches(&cbor("10")));
    assert!(!pattern.matches(&cbor("15")));

    // Display should show capture syntax with complex inner pattern
    let display = pattern.to_string();
    assert!(display.starts_with("@range("));
    assert!(display.contains("&"));
    assert!(display.ends_with(")"));
}

#[test]
fn test_nested_capture_patterns() {
    let pattern = Pattern::capture(
        "outer",
        Pattern::or(vec![
            Pattern::capture("inner1", Pattern::number(42)),
            Pattern::capture("inner2", Pattern::text("hello")),
        ]),
    );

    // Should match either captured pattern
    let forty_two_cbor = cbor("42");
    let paths = pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other values
    assert!(!pattern.matches(&cbor("43")));
    assert!(!pattern.matches(&cbor(r#""world""#)));

    // Display should show nested capture syntax
    let display = pattern.to_string();
    assert!(display.starts_with("@outer("));
    assert!(display.contains("@inner1"));
    assert!(display.contains("@inner2"));
    assert!(display.contains("|"));
    assert!(display.ends_with(")"));
}

#[test]
fn test_capture_pattern_name_access() {
    let inner_pattern = Pattern::number(42);
    let pattern = Pattern::capture("test_name", inner_pattern.clone());

    // Test that we can access the capture pattern internals
    if let dcbor_pattern::Pattern::Meta(dcbor_pattern::MetaPattern::Capture(
        capture,
    )) = &pattern
    {
        assert_eq!(capture.name(), "test_name");
        assert_eq!(capture.pattern(), &inner_pattern);
    } else {
        panic!("Expected capture pattern");
    }
}

#[test]
fn test_capture_pattern_is_complex() {
    // Simple capture should not be complex if inner pattern isn't complex
    let simple = Pattern::capture("simple", Pattern::number(42));
    assert!(!simple.is_complex());

    // Complex capture should be complex if inner pattern is complex
    let complex = Pattern::capture(
        "complex",
        Pattern::and(vec![Pattern::number(1), Pattern::number(2)]),
    );
    assert!(complex.is_complex());
}

#[test]
fn test_repeat_pattern_basic() {
    // Test exact match (default quantifier)
    let pattern = Pattern::group(Pattern::number(42));

    let forty_two_cbor = cbor("42");
    let paths = pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other values
    assert!(!pattern.matches(&cbor("41")));
    assert!(!pattern.matches(&cbor(r#""hello""#)));

    // Display should show pattern with {1} quantifier
    assert_eq!(pattern.to_string(), "(42){1}");
}

#[test]
fn test_repeat_pattern_with_quantifier() {
    use dcbor_pattern::{Quantifier, Reluctance};

    // Test optional pattern (0 or 1 match)
    let optional_pattern = Pattern::repeat(
        Pattern::number(42),
        Quantifier::new(0..=1, Reluctance::Greedy),
    );

    // Should match the number or succeed without it
    let forty_two_cbor = cbor("42");
    let paths = optional_pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    // Display should show pattern with ? quantifier
    assert_eq!(optional_pattern.to_string(), "(42)?");
}

#[test]
fn test_repeat_pattern_zero_or_more() {
    use dcbor_pattern::{Quantifier, Reluctance};

    // Test zero or more pattern
    let star_pattern = Pattern::repeat(
        Pattern::number(42),
        Quantifier::new(0.., Reluctance::Greedy),
    );

    // Should always succeed with 0 matches or with the actual match
    let forty_two_cbor = cbor("42");
    let paths = star_pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    // Also succeeds with 0 matches for non-matching values - tested with
    // matches()
    assert!(star_pattern.matches(&cbor("41"))); // Succeeds with 0 matches

    // Display should show pattern with * quantifier
    assert_eq!(star_pattern.to_string(), "(42)*");
}

#[test]
fn test_repeat_pattern_one_or_more() {
    use dcbor_pattern::{Quantifier, Reluctance};

    // Test one or more pattern
    let plus_pattern = Pattern::repeat(
        Pattern::number(42),
        Quantifier::new(1.., Reluctance::Greedy),
    );

    // Should match the number but not other values
    let forty_two_cbor = cbor("42");
    let paths = plus_pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other values
    assert!(!plus_pattern.matches(&cbor("41")));

    // Display should show pattern with + quantifier
    assert_eq!(plus_pattern.to_string(), "(42)+");
}

#[test]
fn test_repeat_pattern_exact_count() {
    use dcbor_pattern::{Quantifier, Reluctance};

    // Test exact count pattern
    let exact_pattern = Pattern::repeat(
        Pattern::number(42),
        Quantifier::new(3..=3, Reluctance::Greedy),
    );

    // For single values, this should fail if count > 1
    assert!(!exact_pattern.matches(&cbor("42")));

    // Display should show pattern with {3} quantifier
    assert_eq!(exact_pattern.to_string(), "(42){3}");
}

#[test]
fn test_repeat_pattern_display_with_reluctance() {
    use dcbor_pattern::{Quantifier, Reluctance};

    // Test lazy quantifier
    let lazy_pattern = Pattern::repeat(
        Pattern::text("test"),
        Quantifier::new(0..=1, Reluctance::Lazy),
    );

    assert_eq!(lazy_pattern.to_string(), r#"("test")??"#);

    // Test possessive quantifier
    let possessive_pattern = Pattern::repeat(
        Pattern::text("test"),
        Quantifier::new(1.., Reluctance::Possessive),
    );

    assert_eq!(possessive_pattern.to_string(), r#"("test")++"#);
}

#[test]
fn test_search_pattern_basic() {
    let pattern = Pattern::search(Pattern::number(42));

    // Test with a flat CBOR value containing the number
    let forty_two_cbor = cbor("42");
    let paths = pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other flat values
    assert!(!pattern.matches(&cbor("43")));

    // Test with nested structure containing the number
    let array_cbor = cbor("[1, 42, 3]");
    let paths = pattern.paths(&array_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [1, 42, 3]
            42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let map_cbor = cbor("{1: 42}");
    let paths = pattern.paths(&map_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        {1: 42}
            42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let nested_cbor = cbor("{\"key\": [1, 2, 42]}");
    let paths = pattern.paths(&nested_cbor);
    #[rustfmt::skip]
    let expected = r#"{"key": [1, 2, 42]}
    [1, 2, 42]
        42"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test that it doesn't match when the value is not present
    assert!(!pattern.matches(&cbor("[1, 2, 3]")));
    assert!(!pattern.matches(&cbor("{1: 2}")));

    // Display should show SEARCH(...)
    assert_eq!(pattern.to_string(), "SEARCH(42)");
}

#[test]
fn test_search_pattern_text() {
    let pattern = Pattern::search(Pattern::text("hello"));

    // Test with nested structures
    let array_cbor = cbor(r#"["hello", "world"]"#);
    let paths = pattern.paths(&array_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        ["hello", "world"]
            "hello"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let map_cbor = cbor(r#"{"greeting": "hello"}"#);
    let paths = pattern.paths(&map_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        {"greeting": "hello"}
            "hello"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let nested_cbor = cbor(r#"[{"nested": ["hello"]}]"#);
    let paths = pattern.paths(&nested_cbor);
    #[rustfmt::skip]
    let expected = r#"[{"nested": ["hello"]}]
    {"nested": ["hello"]}
        ["hello"]
            "hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test that it doesn't match when the text is not present
    assert!(!pattern.matches(&cbor(r#"["goodbye", "world"]"#)));

    // Display should show SEARCH(...)
    assert_eq!(pattern.to_string(), r#"SEARCH("hello")"#);
}

#[test]
fn test_search_pattern_any() {
    let pattern = Pattern::search(Pattern::any());

    // Should match any CBOR value because ANY matches everything
    let forty_two_cbor = cbor("42");
    let paths = pattern.paths(&forty_two_cbor);
    let expected = "42";
    assert_actual_expected!(format_paths(&paths), expected);

    let hello_cbor = cbor(r#""hello""#);
    let paths = pattern.paths(&hello_cbor);
    let expected = r#""hello""#;
    assert_actual_expected!(format_paths(&paths), expected);

    let array_cbor = cbor("[1, 2, 3]");
    let paths = pattern.paths(&array_cbor);
    // ANY matches everything, so this should match all nodes in the tree
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [1, 2, 3]
        [1, 2, 3]
            1
        [1, 2, 3]
            2
        [1, 2, 3]
            3
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let empty_map_cbor = cbor("{}");
    let paths = pattern.paths(&empty_map_cbor);
    let expected = "{}";
    assert_actual_expected!(format_paths(&paths), expected);

    // Display should show SEARCH(ANY)
    assert_eq!(pattern.to_string(), "SEARCH(ANY)");
}

#[test]
fn test_search_pattern_complex() {
    // Search for arrays containing the number 5
    let pattern = Pattern::search(Pattern::number(5));

    let test_data = cbor(
        r#"
    {
        "data": [
            {"values": [1, 2, 3]},
            {"values": [4, 5, 6]},
            {"other": "text"}
        ],
        "meta": {
            "count": 5,
            "items": [7, 8, 9]
        }
    }
    "#,
    );

    // Should match because the structure contains the number 5 in multiple
    // places
    let paths = pattern.paths(&test_data);
    #[rustfmt::skip]
    let expected = r#"{"data": [{"values": [1, 2, 3]}, {"values": [4, 5, 6]}, {"other": "text"}], "meta": {"count": 5, "items": [7, 8, 9]}}
    [{"values": [1, 2, 3]}, {"values": [4, 5, 6]}, {"other": "text"}]
        {"values": [4, 5, 6]}
            [4, 5, 6]
                5
{"data": [{"values": [1, 2, 3]}, {"values": [4, 5, 6]}, {"other": "text"}], "meta": {"count": 5, "items": [7, 8, 9]}}
    {"count": 5, "items": [7, 8, 9]}
        5"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Check specific paths are found
    assert!(!paths.is_empty());
}

#[test]
fn test_search_pattern_with_captures() {
    // Create a search pattern that captures what it finds
    let inner_pattern = Pattern::capture("found", Pattern::number(42));
    let pattern = Pattern::search(inner_pattern);

    // Test with a nested structure
    let data = cbor(r#"[1, {"key": 42}, 3]"#);
    let paths = pattern.paths(&data);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [1, {"key": 42}, 3]
            {"key": 42}
                42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Display should show the capture in the search
    assert_eq!(pattern.to_string(), "SEARCH(@found(42))");
}

#[test]
fn test_search_pattern_paths() {
    let pattern = Pattern::search(Pattern::text("target"));

    #[rustfmt::skip]
    let data = cbor(r#"
        {
            "level1": {
                "level2": ["target", "other"]
            },
            "another": "target"
        }
    "#);

    let paths = pattern.paths(&data);

    // Should find multiple paths to "target"
    assert!(paths.len() >= 2);

    // All paths should be valid (non-empty)
    for path in &paths {
        assert!(!path.is_empty());
    }
}

#[test]
fn test_search_pattern_edge_cases() {
    let pattern = Pattern::search(Pattern::number(1));

    // Test with empty structures
    assert!(!pattern.matches(&cbor("[]")));
    assert!(!pattern.matches(&cbor("{}")));

    // Test with null
    assert!(!pattern.matches(&cbor("null")));

    // Test with deeply nested structure containing the target
    assert!(pattern.matches(&cbor("[[[[1]]]]")));
}

#[test]
fn test_search_pattern_with_structure_pattern() {
    // Search for any array
    let pattern = Pattern::search(Pattern::parse("[*]").unwrap());

    #[rustfmt::skip]
    let data = cbor(r#"
        {
            "arrays": [[1, 2], [3, 4]],
            "not_array": 42
        }
    "#);

    let paths = pattern.paths(&data);
    // Should find the outer arrays structure and the inner arrays
    #[rustfmt::skip]
    let expected = indoc! {r#"
        {"arrays": [[1, 2], [3, 4]], "not_array": 42}
            [[1, 2], [3, 4]]
        {"arrays": [[1, 2], [3, 4]], "not_array": 42}
            [[1, 2], [3, 4]]
                [1, 2]
        {"arrays": [[1, 2], [3, 4]], "not_array": 42}
            [[1, 2], [3, 4]]
                [3, 4]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(paths.len() >= 3); // The "arrays" value plus the two inner arrays
}

#[test]
fn test_search_array_order() {
    let data = cbor(r#"[[1, 2, 3], [4, 5, 6]]"#);
    let pattern = Pattern::parse("SEARCH([*])").unwrap();

    let paths = pattern.paths(&data);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [[1, 2, 3], [4, 5, 6]]
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let pattern = Pattern::parse("SEARCH(number)").unwrap();
    let paths = pattern.paths(&data);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
                1
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
                2
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
                3
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
                4
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
                5
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
                6
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

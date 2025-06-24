use dcbor::prelude::*;
use dcbor_pattern::{Matcher, Pattern};
use dcbor_parse::parse_dcbor_item;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR {
    parse_dcbor_item(s).unwrap()
}

#[test]
fn test_any_pattern() {
    let pattern = Pattern::any();

    // Should match all types of CBOR values
    assert!(pattern.matches(&cbor("42")));
    assert!(pattern.matches(&cbor(r#""hello""#)));
    assert!(pattern.matches(&cbor("true")));
    assert!(pattern.matches(&cbor("[1, 2, 3]")));
    assert!(pattern.matches(&cbor("null")));

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
    assert!(pattern.matches(&cbor("7")));
    assert!(pattern.matches(&cbor("6")));
    assert!(pattern.matches(&cbor("9")));

    // Should not match values that fail any condition
    assert!(!pattern.matches(&cbor("3"))); // < 5
    assert!(!pattern.matches(&cbor("12"))); // > 10
    assert!(!pattern.matches(&cbor(r#""hello""#))); // not a number

    // Display should use & operator
    assert_eq!(pattern.to_string(), "NUMBER(>5)&NUMBER(<10)");
}

#[test]
fn test_or_pattern() {
    let pattern = Pattern::or(vec![
        Pattern::number(5),
        Pattern::text("hello"),
        Pattern::bool(true),
    ]);

    // Should match values that satisfy any condition
    assert!(pattern.matches(&cbor("5")));
    assert!(pattern.matches(&cbor(r#""hello""#)));
    assert!(pattern.matches(&cbor("true")));

    // Should not match values that don't satisfy any condition
    assert!(!pattern.matches(&cbor("42")));
    assert!(!pattern.matches(&cbor(r#""world""#)));
    assert!(!pattern.matches(&cbor("false")));

    // Display should use | operator
    assert_eq!(pattern.to_string(), r#"NUMBER(5)|TEXT("hello")|BOOL(true)"#);
}

#[test]
fn test_not_pattern() {
    let pattern = Pattern::not(Pattern::number(5));

    // Should match values that don't match the inner pattern
    assert!(pattern.matches(&cbor("42")));
    assert!(pattern.matches(&cbor(r#""hello""#)));
    assert!(pattern.matches(&cbor("true")));

    // Should not match the exact value
    assert!(!pattern.matches(&cbor("5")));

    // Display should use ! operator
    assert_eq!(pattern.to_string(), "!NUMBER(5)");
}

#[test]
fn test_not_pattern_complex() {
    let inner = Pattern::and(vec![
        Pattern::number_greater_than(5),
        Pattern::number_less_than(10),
    ]);
    let pattern = Pattern::not(inner);

    // Should match values outside the range
    assert!(pattern.matches(&cbor("3"))); // < 5
    assert!(pattern.matches(&cbor("12"))); // > 10
    assert!(pattern.matches(&cbor(r#""hello""#))); // not a number

    // Should not match values in the range
    assert!(!pattern.matches(&cbor("7")));

    // Display should wrap complex patterns in parentheses
    assert_eq!(pattern.to_string(), "!(NUMBER(>5)&NUMBER(<10))");
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
    assert!(pattern.matches(&cbor("7")));

    // Should match the specific text
    assert!(pattern.matches(&cbor(r#""hello""#)));

    // Should not match numbers outside range or other text
    assert!(!pattern.matches(&cbor("3")));
    assert!(!pattern.matches(&cbor("12")));
    assert!(!pattern.matches(&cbor(r#""world""#)));

    // Display should properly nest the operators
    assert_eq!(
        pattern.to_string(),
        r#"NUMBER(>5)&NUMBER(<10)|TEXT("hello")"#
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
    assert!(pattern.matches(&cbor("42")));
    assert!(!pattern.matches(&cbor("43")));
    assert!(!pattern.matches(&cbor(r#""hello""#)));

    // Display should show capture syntax
    assert_eq!(pattern.to_string(), "@test(NUMBER(42))");
}

#[test]
fn test_capture_pattern_text() {
    let pattern = Pattern::capture("name", Pattern::text("hello"));

    // Should match the same things as the inner pattern
    assert!(pattern.matches(&cbor(r#""hello""#)));
    assert!(!pattern.matches(&cbor(r#""world""#)));
    assert!(!pattern.matches(&cbor("42")));

    // Display should show capture syntax
    assert_eq!(pattern.to_string(), r#"@name(TEXT("hello"))"#);
}

#[test]
fn test_capture_pattern_any() {
    let pattern = Pattern::capture("anything", Pattern::any());

    // Should match anything since inner pattern is ANY
    assert!(pattern.matches(&cbor("42")));
    assert!(pattern.matches(&cbor(r#""hello""#)));
    assert!(pattern.matches(&cbor("true")));
    assert!(pattern.matches(&cbor("[1, 2, 3]")));

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
        ])
    );

    // Should match numbers in range 5 < x < 10
    assert!(pattern.matches(&cbor("7")));
    assert!(pattern.matches(&cbor("6")));
    assert!(pattern.matches(&cbor("9")));
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
        ])
    );

    // Should match either captured pattern
    assert!(pattern.matches(&cbor("42")));
    assert!(pattern.matches(&cbor(r#""hello""#)));
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
    if let dcbor_pattern::Pattern::Meta(dcbor_pattern::MetaPattern::Capture(capture)) = &pattern {
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
    let complex = Pattern::capture("complex", Pattern::and(vec![
        Pattern::number(1),
        Pattern::number(2),
    ]));
    assert!(complex.is_complex());
}

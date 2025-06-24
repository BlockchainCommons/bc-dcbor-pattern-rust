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

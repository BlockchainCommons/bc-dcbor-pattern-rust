use dcbor::prelude::*;
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn test_bool_pattern_any() {
    let pattern = Pattern::any_bool();

    // Should match true
    let true_cbor = true.to_cbor();
    assert!(pattern.matches(&true_cbor));
    let paths = pattern.paths(&true_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], true_cbor);

    // Should match false
    let false_cbor = false.to_cbor();
    assert!(pattern.matches(&false_cbor));
    let paths = pattern.paths(&false_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], false_cbor);

    // Should not match non-boolean
    let number_cbor = 42.to_cbor();
    assert!(!pattern.matches(&number_cbor));
    let paths = pattern.paths(&number_cbor);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_bool_pattern_specific() {
    let true_pattern = Pattern::bool(true);
    let false_pattern = Pattern::bool(false);

    let true_cbor = true.to_cbor();
    let false_cbor = false.to_cbor();
    let number_cbor = 42.to_cbor();

    // true pattern tests
    assert!(true_pattern.matches(&true_cbor));
    assert!(!true_pattern.matches(&false_cbor));
    assert!(!true_pattern.matches(&number_cbor));

    // false pattern tests
    assert!(!false_pattern.matches(&true_cbor));
    assert!(false_pattern.matches(&false_cbor));
    assert!(!false_pattern.matches(&number_cbor));
}

#[test]
fn test_bool_pattern_display() {
    assert_eq!(Pattern::any_bool().to_string(), "BOOL");
    assert_eq!(Pattern::bool(true).to_string(), "BOOL(true)");
    assert_eq!(Pattern::bool(false).to_string(), "BOOL(false)");
}

#[test]
fn test_text_pattern_any() {
    let pattern = Pattern::any_text();

    // Should match any text
    let hello_cbor = "Hello".to_cbor();
    assert!(pattern.matches(&hello_cbor));
    let paths = pattern.paths(&hello_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], hello_cbor);

    let empty_cbor = "".to_cbor();
    assert!(pattern.matches(&empty_cbor));
    let paths = pattern.paths(&empty_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], empty_cbor);

    // Should not match non-text
    let number_cbor = 42.to_cbor();
    assert!(!pattern.matches(&number_cbor));
    let paths = pattern.paths(&number_cbor);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_text_pattern_specific() {
    let hello_pattern = Pattern::text("Hello");
    let world_pattern = Pattern::text("World");

    let hello_cbor = "Hello".to_cbor();
    let world_cbor = "World".to_cbor();
    let number_cbor = 42.to_cbor();

    // hello pattern tests
    assert!(hello_pattern.matches(&hello_cbor));
    assert!(!hello_pattern.matches(&world_cbor));
    assert!(!hello_pattern.matches(&number_cbor));

    // world pattern tests
    assert!(!world_pattern.matches(&hello_cbor));
    assert!(world_pattern.matches(&world_cbor));
    assert!(!world_pattern.matches(&number_cbor));
}

#[test]
fn test_text_pattern_regex() {
    let digits_regex = regex::Regex::new(r"^\d+$").unwrap();
    let digits_pattern = Pattern::text_regex(digits_regex);

    let digits_cbor = "12345".to_cbor();
    let letters_cbor = "Hello".to_cbor();
    let mixed_cbor = "Hello123".to_cbor();
    let number_cbor = 42.to_cbor();

    // Should match pure digits
    assert!(digits_pattern.matches(&digits_cbor));
    let paths = digits_pattern.paths(&digits_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], digits_cbor);

    // Should not match letters, mixed content, or non-text
    assert!(!digits_pattern.matches(&letters_cbor));
    assert!(!digits_pattern.matches(&mixed_cbor));
    assert!(!digits_pattern.matches(&number_cbor));
}

#[test]
fn test_text_pattern_display() {
    assert_eq!(Pattern::any_text().to_string(), "TEXT");
    assert_eq!(Pattern::text("Hello").to_string(), r#"TEXT("Hello")"#);

    let regex_pattern =
        Pattern::text_regex(regex::Regex::new(r"^\d+$").unwrap());
    assert_eq!(regex_pattern.to_string(), r#"TEXT(/^\d+$/)"#);
}

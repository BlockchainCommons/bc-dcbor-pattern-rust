use dcbor::prelude::*;
use dcbor_pattern::{Pattern, Matcher};

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

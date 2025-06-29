use dcbor_pattern::{Pattern, Matcher};
use dcbor::prelude::*;

#[test]
fn test_infinity_pattern_integration() {
    // Test parsing and matching of infinity patterns

    // Parse NUMBER(Infinity) pattern
    let inf_pattern = Pattern::parse("NUMBER(Infinity)").unwrap();
    assert_eq!(inf_pattern.to_string(), "NUMBER(Infinity)");

    // Parse NUMBER(-Infinity) pattern
    let neg_inf_pattern = Pattern::parse("NUMBER(-Infinity)").unwrap();
    assert_eq!(neg_inf_pattern.to_string(), "NUMBER(-Infinity)");

    // Create CBOR values
    let inf_cbor = f64::INFINITY.to_cbor();
    let neg_inf_cbor = f64::NEG_INFINITY.to_cbor();
    let nan_cbor = f64::NAN.to_cbor();
    let regular_cbor = 42.0.to_cbor();

    // Test positive infinity pattern matching
    assert!(inf_pattern.matches(&inf_cbor));
    assert!(!inf_pattern.matches(&neg_inf_cbor));
    assert!(!inf_pattern.matches(&nan_cbor));
    assert!(!inf_pattern.matches(&regular_cbor));

    // Test negative infinity pattern matching
    assert!(!neg_inf_pattern.matches(&inf_cbor));
    assert!(neg_inf_pattern.matches(&neg_inf_cbor));
    assert!(!neg_inf_pattern.matches(&nan_cbor));
    assert!(!neg_inf_pattern.matches(&regular_cbor));

    // Test parsing still works for NaN
    let nan_pattern = Pattern::parse("NUMBER(NaN)").unwrap();
    assert_eq!(nan_pattern.to_string(), "NUMBER(NaN)");
    assert!(!nan_pattern.matches(&inf_cbor));
    assert!(!nan_pattern.matches(&neg_inf_cbor));
    assert!(nan_pattern.matches(&nan_cbor));
    assert!(!nan_pattern.matches(&regular_cbor));
}

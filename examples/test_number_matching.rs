use std::f64;

use dcbor::prelude::*;
use dcbor_pattern::{Matcher, Pattern};

fn main() {
    // Create some test dCBOR values
    let test_values = [
        5.0_f64.to_cbor(),
        42.0_f64.to_cbor(),
        (-10.0_f64).to_cbor(),
        std::f64::consts::PI.to_cbor(),
        f64::NAN.to_cbor(),
        100.0_f64.to_cbor(),
    ];

    let test_patterns = [
        ("NUMBER", "any number"),
        ("NUMBER(42)", "exact 42"),
        ("NUMBER(-10)", "exact -10"),
        ("NUMBER(1...50)", "range 1 to 50"),
        ("NUMBER(>40)", "greater than 40"),
        ("NUMBER(<=10)", "less than or equal 10"),
        ("NUMBER(NaN)", "NaN only"),
    ];

    for (pattern_str, description) in &test_patterns {
        println!("\nTesting {}: {}", pattern_str, description);
        let pattern = Pattern::parse(pattern_str).unwrap();

        for (i, value) in test_values.iter().enumerate() {
            let matches = pattern.matches(value);
            let extracted: Result<f64, _> = f64::try_from(value.clone());
            let value_desc = match extracted {
                Ok(f) if f.is_nan() => "NaN".to_string(),
                Ok(f) => f.to_string(),
                Err(_) => "non-number".to_string(),
            };
            println!(
                "  Value {}: {} {}",
                i,
                value_desc,
                if matches { "✓" } else { "✗" }
            );
        }
    }
}

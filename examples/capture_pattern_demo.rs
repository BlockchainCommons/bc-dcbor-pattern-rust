//! Demonstration of capture pattern functionality
//!
//! This example shows how capture patterns work with the basic matching system.

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

fn main() {
    println!("=== Capture Pattern Demo ===");

    // Create a capture pattern
    let pattern = Pattern::capture("my_number", Pattern::number(42));

    println!("Pattern: {}", pattern);

    // Test data
    let cbor = parse_dcbor_item("42").unwrap();
    let wrong_cbor = parse_dcbor_item("43").unwrap();

    // Test basic matching
    println!("Pattern matches 42: {}", pattern.matches(&cbor));
    println!("Pattern matches 43: {}", pattern.matches(&wrong_cbor));

    // Show paths
    println!("Paths for 42: {:?}", pattern.paths(&cbor));
    println!("Paths for 43: {:?}", pattern.paths(&wrong_cbor));

    // Show that nested captures work
    println!("\n=== Nested Capture Demo ===");
    let nested = Pattern::capture(
        "outer",
        Pattern::and(vec![
            Pattern::capture("inner", Pattern::number_greater_than(40)),
            Pattern::number_less_than(50),
        ]),
    );

    println!("Nested pattern: {}", nested);
    println!("Matches 42: {}", nested.matches(&cbor));
    println!("Paths for nested pattern: {:?}", nested.paths(&cbor));

    // Test complexity detection
    println!("\n=== Complexity Detection ===");
    let simple = Pattern::capture("simple", Pattern::number(42));
    let complex = Pattern::capture(
        "complex",
        Pattern::and(vec![Pattern::number(1), Pattern::number(2)]),
    );

    println!("Simple capture is complex: {}", simple.is_complex());
    println!("Complex capture is complex: {}", complex.is_complex());
}

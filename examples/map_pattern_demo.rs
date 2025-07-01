use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

fn main() {
    println!("MAP Pattern Parsing Demo");
    println!("=======================");

    // Test data
    let empty_map = cbor("{}");
    let single_item = cbor(r#"{"key": "value"}"#);
    let three_items = cbor(r#"{"a": 1, "b": 2, "c": 3}"#);

    // Parse different MAP patterns and test them
    let patterns = vec![
        "{*}",     // Any map
        "{{0}}",   // Empty map
        "{{1}}",   // Single item map
        "{{3}}",   // Three item map
        "{{1,5}}", // Range: 1-5 items
        "{{2,}}",  // At least 2 items
    ];

    for pattern_str in patterns {
        println!("\nPattern: {}", pattern_str);
        let pattern = Pattern::parse(pattern_str).unwrap();
        println!("  Parsed: {}", pattern);

        println!("  Matches empty map: {}", pattern.matches(&empty_map));
        println!("  Matches single item: {}", pattern.matches(&single_item));
        println!("  Matches three items: {}", pattern.matches(&three_items));
    }

    println!("\nPattern round-trip test:");
    for pattern_str in &["{*}", "{{0}}", "{{3}}", "{{2,8}}", "{{5,}}"] {
        let pattern = Pattern::parse(pattern_str).unwrap();
        let displayed = pattern.to_string();
        println!("  {} -> {}", pattern_str, displayed);
        assert_eq!(pattern_str, &displayed);
    }
    println!("✅ All round-trip tests passed!");
}

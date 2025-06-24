/// Demonstration of how dcbor-parse integration simplifies CBOR object
/// creation for testing
use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    ArrayPattern, MapPattern, Matcher, Pattern, TaggedPattern,
};

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR { parse_dcbor_item(s).unwrap() }

fn main() {
    println!("=== dCBOR Parse Integration Demo ===\n");

    // Example 1: Simple value patterns with dcbor-parse
    println!("1. Simple Values:");
    let number_pattern = Pattern::number(42);
    let number_cbor = cbor("42"); // Much simpler than 42.to_cbor()
    println!("   Pattern: {}", number_pattern);
    println!("   Matches 42: {}", number_pattern.matches(&number_cbor));

    let text_pattern = Pattern::text("hello");
    let text_cbor = cbor(r#""hello""#); // Clearer than "hello".to_cbor()
    println!("   Pattern: {}", text_pattern);
    println!("   Matches \"hello\": {}", text_pattern.matches(&text_cbor));
    println!();

    // Example 2: Complex structures with dcbor-parse
    println!("2. Complex Structures:");

    // Arrays are much simpler to create
    let array_cbor = cbor("[1, 2, 3, \"hello\", true]");
    println!("   Array: {}", array_cbor);
    let any_array_pattern = ArrayPattern::any();
    println!(
        "   Array pattern matches: {}",
        any_array_pattern.matches(&array_cbor)
    );

    // Maps are much cleaner
    let map_cbor = cbor(r#"{"name": "Alice", "age": 30, "active": true}"#);
    println!("   Map: {}", map_cbor);
    let any_map_pattern = MapPattern::any();
    println!(
        "   Map pattern matches: {}",
        any_map_pattern.matches(&map_cbor)
    );
    println!();

    // Example 3: Byte strings in hex notation
    println!("3. Byte Strings:");
    let byte_string_cbor = cbor("h'deadbeef'"); // Much clearer than CBOR::to_byte_string(vec![0xDE, 0xAD, 0xBE, 0xEF])
    println!("   Byte string: {}", byte_string_cbor);
    let byte_pattern = Pattern::any_byte_string();
    println!(
        "   Byte string pattern matches: {}",
        byte_pattern.matches(&byte_string_cbor)
    );
    println!();

    // Example 4: Tagged values
    println!("4. Tagged Values:");
    let tagged_cbor = cbor(r#"1234("tagged content")"#); // Much simpler than CBORCase::Tagged construction
    println!("   Tagged value: {}", tagged_cbor);
    let tagged_pattern = TaggedPattern::any();
    println!(
        "   Tagged pattern matches: {}",
        tagged_pattern.matches(&tagged_cbor)
    );
    println!();

    // Example 5: Complex nested structures
    println!("5. Complex Nested Structures:");
    let complex_cbor = cbor(
        r#"{
        "users": [
            {"name": "Alice", "data": h'010203'},
            {"name": "Bob", "data": h'040506'}
        ],
        "metadata": {
            "version": 1,
            "timestamp": 1234("2023-12-25T00:00:00Z")
        }
    }"#,
    );
    println!("   Complex structure: {}", complex_cbor);
    println!("   Structure is valid CBOR: {}", complex_cbor.is_map());
    println!();

    // Example 6: Before and after comparison
    println!("6. Before vs After Comparison:");
    println!("   BEFORE (manual construction):");
    println!("   let mut map = Map::new();");
    println!("   map.insert(\"key\", \"value\");");
    println!("   let cbor = CBOR::from(map);");
    println!();
    println!("   AFTER (dcbor-parse):");
    println!("   let cbor = cbor(r#\"{{\"key\": \"value\"}}\"#);");
    println!();

    // Example 7: Testing patterns against various values
    println!("7. Pattern Matching Examples:");
    let or_pattern = Pattern::or(vec![
        Pattern::number(42),
        Pattern::text("hello"),
        Pattern::bool(true),
    ]);

    let test_values = vec![
        ("42", cbor("42")),
        ("\"hello\"", cbor(r#""hello""#)),
        ("true", cbor("true")),
        ("false", cbor("false")),
        ("[1, 2, 3]", cbor("[1, 2, 3]")),
    ];

    println!("   Pattern: {}", or_pattern);
    for (desc, value) in test_values {
        println!("   {} matches: {}", desc, or_pattern.matches(&value));
    }

    println!("\n=== Demo Complete ===");
}

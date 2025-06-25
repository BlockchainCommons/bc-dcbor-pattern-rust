// Temporary test file to check repeat pattern support

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, Quantifier, Reluctance};

fn main() {
    // Test 1: Can we create repeat patterns programmatically?
    let any_star = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=usize::MAX, Reluctance::Greedy),
    );
    println!("✅ Can create (ANY)* pattern: {}", any_star);

    // Test 2: Can we parse repeat patterns from text?
    let parsed_repeat = Pattern::parse("(ANY)*");
    match parsed_repeat {
        Ok(pattern) => println!("✅ Can parse (ANY)* from text: {}", pattern),
        Err(e) => println!("❌ Cannot parse (ANY)* from text: {}", e),
    }

    // Test 3: Can we create sequences with repeats?
    let sequence_with_repeats = Pattern::sequence(vec![
        any_star.clone(),
        Pattern::number(42),
        any_star.clone(),
    ]);
    println!(
        "✅ Can create sequence with repeats: {}",
        sequence_with_repeats
    );

    // Test 4: Can we create ARRAY patterns with sequences containing repeats?
    let array_with_repeats =
        Pattern::Structure(dcbor_pattern::StructurePattern::Array(
            dcbor_pattern::ArrayPattern::with_elements(
                sequence_with_repeats.clone(),
            ),
        ));
    println!(
        "✅ Can create ARRAY with repeat sequence: {}",
        array_with_repeats
    );

    // Test 5: Let's test some matching behavior
    let test_arrays = [
        "[42]",       // Just 42
        "[1, 42]",    // 42 at end
        "[42, 1]",    // 42 at start
        "[1, 42, 3]", // 42 in middle
        "[1, 2, 3]",  // No 42
        "[]",         // Empty array
    ];

    println!("\n--- Testing array matching with (ANY)*>NUMBER(42)>(ANY)* ---");

    for cbor_text in &test_arrays {
        let cbor = parse_dcbor_item(cbor_text).unwrap();
        let matches = array_with_repeats.matches(&cbor);
        println!(
            "{}: {}",
            cbor_text,
            if matches { "✅ MATCH" } else { "❌ NO MATCH" }
        );
    }

    // Test 6: Test simple repeat pattern matching
    println!("\n--- Testing simple repeat pattern (ANY)* ---");
    let simple_repeat = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=usize::MAX, Reluctance::Greedy),
    );

    for cbor_text in &["42", "\"hello\"", "[]", "{}"] {
        let cbor = parse_dcbor_item(cbor_text).unwrap();
        let matches = simple_repeat.matches(&cbor);
        println!(
            "{}: {}",
            cbor_text,
            if matches { "✅ MATCH" } else { "❌ NO MATCH" }
        );
    }
}

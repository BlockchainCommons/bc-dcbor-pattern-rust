use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

fn main() {
    println!("Testing pattern parsing for unified syntax...\n");

    // Test parsing of the documented syntax from PatternSyntax.md
    let test_patterns = [
        "ARRAY(NUMBER(42))",
        "ARRAY(TEXT(\"a\") > TEXT(\"b\") > TEXT(\"c\"))",
        // Note: This complex pattern may not parse yet, but the programmatic
        // version works
    ];

    for pattern_str in &test_patterns {
        match Pattern::parse(pattern_str) {
            Ok(pattern) => {
                println!("✅ Parsed: {}", pattern_str);
                println!("   Result: {}", pattern);

                // Test some basic matching
                if pattern_str.contains("NUMBER(42)") {
                    let test_array = parse_dcbor_item("[42]").unwrap();
                    println!(
                        "   Matches [42]: {}",
                        pattern.matches(&test_array)
                    );
                }
            }
            Err(e) => {
                println!("❌ Failed to parse: {}", pattern_str);
                println!("   Error: {}", e);
            }
        }
        println!();
    }

    // Test the programmatic API that we know works
    println!("Testing programmatic pattern creation:");

    let any_star = Pattern::repeat(
        Pattern::any(),
        dcbor_pattern::Quantifier::new(
            0..=usize::MAX,
            dcbor_pattern::Reluctance::Greedy,
        ),
    );

    let complex_sequence = Pattern::sequence(vec![
        any_star.clone(),
        Pattern::number(42),
        any_star.clone(),
    ]);

    let array_pattern =
        dcbor_pattern::ArrayPattern::with_elements(complex_sequence);
    let full_pattern = Pattern::Structure(
        dcbor_pattern::StructurePattern::Array(array_pattern),
    );

    println!("✅ Complex pattern created: {}", full_pattern);

    // Test against various arrays
    let test_cases = [
        ("[42]", "Just 42"),
        ("[1, 42]", "42 at end"),
        ("[42, 1]", "42 at start"),
        ("[1, 42, 3]", "42 in middle"),
        ("[1, 2, 3]", "No 42"),
    ];

    for (array_str, description) in &test_cases {
        let test_array = parse_dcbor_item(array_str).unwrap();
        let matches = full_pattern.matches(&test_array);
        println!(
            "   {} ({}): {}",
            array_str,
            description,
            if matches { "✅ MATCH" } else { "❌ NO MATCH" }
        );
    }
}

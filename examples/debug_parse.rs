use dcbor_pattern::Pattern;

fn main() {
    let test_patterns = vec![
        // Basic bracket array patterns
        "[*]",                              // Any array
        "[]",                               // Empty array
        "[{3}]",                            // Exactly 3 elements
        "[{2,5}]",                          // 2 to 5 elements
        "[{3,}]",                           // At least 3 elements

        // Single element patterns
        "[NUMBER(42)]",                     // Array with specific number
        "[TEXT(\"hello\")]",                // Array with specific text
        "[ANY]",                            // Array with any single element

        // Multiple element patterns (comma-separated)
        "[NUMBER(1), NUMBER(2)]",           // Exact sequence
        "[TEXT(\"a\"), TEXT(\"b\"), TEXT(\"c\")]", // Multiple texts

        // Capture patterns
        "[@item(NUMBER(42))]",              // Capture specific number
        "[@first(NUMBER), @second(TEXT)]",  // Multiple captures
        "[@any_item(ANY)]",                 // Capture any item

        // Complex patterns with repeats
        "[(ANY)*]",                         // Any number of any elements
        "[(ANY)*, NUMBER(42)]",             // Any elements followed by 42
        "[NUMBER(42), (ANY)*]",             // 42 followed by any elements
        "[(ANY)*, NUMBER(42), (ANY)*]",     // 42 anywhere in array
    ];

    println!("Testing bracket array syntax:\n");

    for pattern_str in test_patterns {
        println!("Pattern: {}", pattern_str);
        match Pattern::parse(pattern_str) {
            Ok(pattern) => {
                println!("  ✓ Parsed successfully");
                println!("  ✓ Display: {}", pattern);
            },
            Err(e) => {
                println!("  ✗ Failed to parse: {:?}", e);
            },
        }
        println!();
    }
}

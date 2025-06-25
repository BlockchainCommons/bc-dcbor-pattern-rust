use dcbor_pattern::Pattern;

#[test]
fn test_range_syntax_parsing() {
    // Test if range syntax is already implemented for parsing
    let tests = vec![
        "ARRAY({1,10})",
        "MAP({2,8})",
        "ARRAY({1,})",
        "MAP({0,5})",
        "ARRAY({3})",
        "MAP({5})",
    ];

    for test_pattern in tests {
        match Pattern::parse(test_pattern) {
            Ok(pattern) => println!("✅ {} -> {}", test_pattern, pattern),
            Err(e) => println!("❌ {} -> Error: {}", test_pattern, e),
        }
    }
}

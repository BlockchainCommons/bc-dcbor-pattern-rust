use dcbor_pattern::*;

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern {
    Pattern::parse(s).unwrap()
}

#[test]
fn test_search_with_partial_array_capture() {
    // Test the exact patterns from the user's examples
    let cbor1 = dcbor_parse::parse_dcbor_item("[1, 2, 3]").unwrap();
    let cbor2 = dcbor_parse::parse_dcbor_item("[1]").unwrap();

    // This is the pattern that was failing: search([@a(*), @rest((*)*)])
    let pattern_str = "search([@a(*), @rest((*)*)])";
    let pattern = parse(pattern_str);

    println!("Testing search pattern: {}", pattern_str);

    // Test case 1: [1, 2, 3]
    let (paths1, captures1) = pattern.paths_with_captures(&cbor1);
    println!("\nTest case 1 - [1, 2, 3]:");
    println!("Paths: {:?}", paths1);
    println!("Captures: {:?}", captures1);

    // Should have captures for @a and @rest
    assert!(!captures1.is_empty(), "Should have captures for [1, 2, 3]");
    assert!(captures1.contains_key("a"), "Should have capture @a");
    assert!(captures1.contains_key("rest"), "Should have capture @rest");

    // Test case 2: [1]
    let (paths2, captures2) = pattern.paths_with_captures(&cbor2);
    println!("\nTest case 2 - [1]:");
    println!("Paths: {:?}", paths2);
    println!("Captures: {:?}", captures2);

    // Should have captures for @a and @rest
    assert!(!captures2.is_empty(), "Should have captures for [1]");
    assert!(captures2.contains_key("a"), "Should have capture @a");
    // @rest should capture empty array, so it might or might not be in the captures map
    // depending on implementation
}

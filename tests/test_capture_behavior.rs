#[cfg(test)]
mod test_capture_behavior {
    use dcbor_parse::parse_dcbor_item;
    use dcbor_pattern::{Matcher, Pattern};

    #[test]
    fn test_capture_deduplication_behavior() {
        // Test with array that has duplicate values
        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();
        let pattern = Pattern::parse("ARRAY(@item(NUMBER(42)))").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        println!("=== PATHS ===");
        for (i, path) in paths.iter().enumerate() {
            println!("{}: {:?}", i, path);
        }

        println!("\n=== CAPTURES ===");
        for (name, captured_paths) in &captures {
            println!("@{}: {} paths", name, captured_paths.len());
            for (i, path) in captured_paths.iter().enumerate() {
                println!("  {}: {:?}", i, path);
            }
        }

        // The key question: should we capture the value 42 twice (since it appears twice in different positions)
        // or should we deduplicate and only capture it once?

        // Based on the user's request, captures should be unique paths
        // But the paths to [array, 42_at_index_0] and [array, 42_at_index_2] are DIFFERENT paths
        // So both should be captured!

        if let Some(item_captures) = captures.get("item") {
            // These should be two different paths: [array, 42] where 42 is at different indices
            // But in CBOR, we don't have index information in the path - just the values
            // So the paths might actually be identical: [array, 42]
            println!("Number of captures: {}", item_captures.len());
            for (i, path) in item_captures.iter().enumerate() {
                println!("Capture {}: {:?}", i, path);
            }
        }
    }

    #[test]
    fn test_what_makes_paths_unique() {
        // Let's understand what makes paths unique in this context
        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();
        let pattern = Pattern::parse("ARRAY(@item(ANY))").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        println!("=== PATHS ===");
        for (i, path) in paths.iter().enumerate() {
            println!("{}: {:?}", i, path);
        }

        println!("\n=== CAPTURES ===");
        for (name, captured_paths) in &captures {
            println!("@{}: {} paths", name, captured_paths.len());
            for (i, path) in captured_paths.iter().enumerate() {
                println!("  {}: {:?}", i, path);
            }
        }
    }
}

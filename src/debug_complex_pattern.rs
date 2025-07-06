#[cfg(test)]
mod debug_complex_pattern {
    use dcbor_parse::parse_dcbor_item;
    use crate::{Matcher, Pattern, format_paths_with_captures, FormatPathsOpts};

    /// Helper function to parse CBOR diagnostic notation into CBOR objects
    fn cbor(s: &str) -> dcbor::prelude::CBOR { parse_dcbor_item(s).unwrap() }

    /// Helper function to parse pattern text into Pattern objects
    fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

    #[test]
    fn test_complex_pattern_without_search() {
        println!("Testing complex pattern WITHOUT search...\n");

        // Test the pattern without search to see what it should return
        let pattern = parse(r#"@found({"id": @id_value(number)})"#);
        let cbor_data = cbor(r#"{"id": 1, "name": "Alice"}"#);

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);
        println!("Pattern: @found({{\"id\": @id_value(number)}})");
        println!("Data: {{\"id\": 1, \"name\": \"Alice\"}}");
        println!("Result without search:");
        println!("{}", format_paths_with_captures(&paths, &captures, FormatPathsOpts::default()));
        println!();

        // Debug: show the raw capture paths
        for (name, cap_paths) in &captures {
            println!("Capture '{}': {:?}", name, cap_paths);
        }
        println!();
    }

    #[test]
    fn test_array_pattern_without_search() {
        println!("Testing array pattern WITHOUT search...\n");

        // Test the array pattern without search to see what it should return
        let pattern = parse(r#"[@a(*), @b(*), @c(*)]"#);
        let cbor_data = cbor(r#"[1, 2, 3]"#);

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);
        println!("Pattern: [@a(*), @b(*), @c(*)]");
        println!("Data: [1, 2, 3]");
        println!("Result without search:");
        println!("{}", format_paths_with_captures(&paths, &captures, FormatPathsOpts::default()));
        println!();

        // Debug: show the raw capture paths
        for (name, cap_paths) in &captures {
            println!("Capture '{}': {:?}", name, cap_paths);
        }
        println!();
    }

    #[test]
    fn test_complex_pattern_with_search() {
        println!("Testing complex pattern WITH search...\n");

        // Test the pattern with search
        let pattern = parse(r#"search(@found({"id": @id_value(number)}))"#);
        let cbor_data = cbor(r#"{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}"#);

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);
        println!("Pattern: search(@found({{\"id\": @id_value(number)}}))");
        println!("Data: {{\"users\": [...]}}");
        println!("Result with search:");
        println!("{}", format_paths_with_captures(&paths, &captures, FormatPathsOpts::default()));
        println!();

        // Debug: show the raw capture paths
        for (name, cap_paths) in &captures {
            println!("Capture '{}': {:?}", name, cap_paths);
        }
        println!();
    }

    #[test]
    fn test_array_pattern_with_search() {
        println!("Testing array pattern WITH search...\n");

        // Test the array pattern with search
        let pattern = parse(r#"search([@a(*), @b(*), @c(*)])"#);
        let cbor_data = cbor(r#"[1, 2, 3]"#);

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);
        println!("Pattern: search([@a(*), @b(*), @c(*)])");
        println!("Data: [1, 2, 3]");
        println!("Result with search:");
        println!("{}", format_paths_with_captures(&paths, &captures, FormatPathsOpts::default()));
        println!();

        // Debug: show the raw capture paths
        for (name, cap_paths) in &captures {
            println!("Capture '{}': {:?}", name, cap_paths);
        }
        println!();
    }
}

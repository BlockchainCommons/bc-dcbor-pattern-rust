#[cfg(test)]
mod test_search_issue {
    use dcbor_parse::parse_dcbor_item;
    use crate::{Matcher, Pattern, format_paths_with_captures, FormatPathsOpts};

    /// Helper function to parse CBOR diagnostic notation into CBOR objects
    fn cbor(s: &str) -> dcbor::prelude::CBOR { parse_dcbor_item(s).unwrap() }

    /// Helper function to parse pattern text into Pattern objects
    fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

    #[test]
    fn test_search_captures_issue() {
        println!("Testing search pattern with captures...\n");

        // Test case 1: Without search
        let pattern1 = parse("[@a(*), @b(*), @c(*)]");
        let cbor_data = cbor("[1, 2, 3]");

        let (paths1, captures1) = pattern1.paths_with_captures(&cbor_data);
        println!("Pattern: [@a(*), @b(*), @c(*)]");
        println!("Result without search:");
        println!("{}", format_paths_with_captures(&paths1, &captures1, FormatPathsOpts::default()));
        println!();

        // Test case 2: With search
        let pattern2 = parse("search([@a(*), @b(*), @c(*)])");
        let (paths2, captures2) = pattern2.paths_with_captures(&cbor_data);
        println!("Pattern: search([@a(*), @b(*), @c(*)])");
        println!("Result with search:");
        println!("{}", format_paths_with_captures(&paths2, &captures2, FormatPathsOpts::default()));
        println!();

        // Test case 3: Simple search capture (should work correctly)
        let pattern3 = parse("search(@found(42))");
        let cbor_data3 = cbor("[1, [2, 42], 3]");
        let (paths3, captures3) = pattern3.paths_with_captures(&cbor_data3);
        println!("Pattern: search(@found(42))");
        println!("Result simple search:");
        println!("{}", format_paths_with_captures(&paths3, &captures3, FormatPathsOpts::default()));
        println!();

        // Test case 4: Let's see what the simple capture pattern returns
        let simple_pattern = parse("@found(42)");
        let simple_cbor = cbor("42");
        let (_simple_paths, simple_captures) = simple_pattern.paths_with_captures(&simple_cbor);
        println!("Simple capture pattern @found(42) on 42:");
        for (name, cap_paths) in &simple_captures {
            println!("  {}: {:?}", name, cap_paths);
        }
        println!();

        // Test case 4: Let's see what the inner pattern actually returns
        let inner_pattern = parse("[@a(*), @b(*), @c(*)]");
        let (_inner_paths, inner_captures) = inner_pattern.paths_with_captures(&cbor_data);
        println!("Inner pattern captures:");
        for (name, cap_paths) in &inner_captures {
            println!("  {}: {:?}", name, cap_paths);
        }
        println!();

        // The issue: captures should include the individual elements for array captures
        // But with search, we're getting the entire array path for all captures

        // For debugging, print raw capture details
        for name in ["a", "b", "c"] {
            if let (Some(cap1), Some(cap2)) = (captures1.get(name), captures2.get(name)) {
                println!("Capture '{}' without search: {:?}", name, cap1);
                println!("Capture '{}' with search: {:?}", name, cap2);
                println!();
            }
        }
    }

    #[test]
    fn test_array_pattern_captures_directly() {
        // Test just the array pattern with captures without search to understand
        // the baseline behavior
        let pattern = parse("[@a(*), @b(*), @c(*)]");
        let cbor_data = cbor("[1, 2, 3]");

        let (_paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Check what the array pattern actually captures
        println!("Array pattern captures:");
        for (name, cap_paths) in &captures {
            println!("  {}: {:?}", name, cap_paths);
        }

        // The expected behavior:
        // @a should capture [1, 2, 3] -> 1
        // @b should capture [1, 2, 3] -> 2
        // @c should capture [1, 2, 3] -> 3

        assert_eq!(captures.len(), 3);
        assert!(captures.contains_key("a"));
        assert!(captures.contains_key("b"));
        assert!(captures.contains_key("c"));

        // Each capture should have exactly one path
        for name in ["a", "b", "c"] {
            assert_eq!(captures[name].len(), 1);
        }
    }
}

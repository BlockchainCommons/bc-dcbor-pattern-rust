#[cfg(test)]
mod validate_tagged_syntax {
    use dcbor_pattern::Pattern;

    #[test]
    fn test_tagged_syntax_examples_from_agents_md() {
        println!("Validating Tagged Pattern Syntax Update...\n");

        // Test the tagged syntax examples from AGENTS.md
        let test_cases = vec![
            ("tagged", "Matches any CBOR tagged value"),
            (
                "tagged(1234, text)",
                "Matches tagged value with specific u64 tag and content pattern",
            ),
            (
                "tagged(myTag, number)",
                "Matches tagged value with named tag and content pattern",
            ),
            (
                "tagged(/test.*/, text)",
                "Matches tagged value with tag name matching regex and content pattern",
            ),
        ];

        for (pattern_str, description) in test_cases {
            match Pattern::parse(pattern_str) {
                Ok(pattern) => {
                    println!("✓ SUCCESS: `{}` - {}", pattern_str, description);
                    println!("   Parsed as: {}", pattern);
                }
                Err(e) => {
                    panic!("✗ FAILED: `{}` - {:?}", pattern_str, e);
                }
            }
            println!();
        }

        println!("🎉 All tagged pattern syntax tests passed!");
        println!(
            "Tagged pattern syntax has been successfully updated to the new lowercase format."
        );
    }
}

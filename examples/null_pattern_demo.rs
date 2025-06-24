use dcbor::prelude::*;
use dcbor_pattern::{Matcher, Pattern};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Null Pattern Demo ===\n");

    // Create a null pattern
    let null_pattern = Pattern::null();
    println!("Pattern: {}", null_pattern);

    // Create test CBOR values
    let null_cbor = CBOR::null();
    let true_cbor = true.to_cbor();
    let false_cbor = false.to_cbor();
    let number_cbor = 42.to_cbor();
    let text_cbor = "hello".to_cbor();
    let array_cbor = vec![1, 2, 3].to_cbor();

    println!("\nTesting against various CBOR values:");

    // Test null value
    println!(
        "  null           -> matches: {}",
        null_pattern.matches(&null_cbor)
    );

    // Test other values
    println!(
        "  true           -> matches: {}",
        null_pattern.matches(&true_cbor)
    );
    println!(
        "  false          -> matches: {}",
        null_pattern.matches(&false_cbor)
    );
    println!(
        "  42             -> matches: {}",
        null_pattern.matches(&number_cbor)
    );
    println!(
        "  \"hello\"        -> matches: {}",
        null_pattern.matches(&text_cbor)
    );
    println!(
        "  [1, 2, 3]      -> matches: {}",
        null_pattern.matches(&array_cbor)
    );

    println!("\nPaths for matching values:");
    let paths = null_pattern.paths(&null_cbor);
    println!("  null -> paths: {} path(s)", paths.len());
    if !paths.is_empty() {
        println!("    First path length: {}", paths[0].len());
        println!("    First path element: {}", paths[0][0].diagnostic());
    }

    // Test parsing
    println!("\nParsing 'NULL' pattern:");
    let parsed_pattern = Pattern::parse("NULL")?;
    println!("  Parsed: {}", parsed_pattern);
    println!("  Equals original: {}", parsed_pattern == null_pattern);

    Ok(())
}

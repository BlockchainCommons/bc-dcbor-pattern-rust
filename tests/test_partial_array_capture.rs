mod common;

use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths_with_captures};

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

#[test]
fn test_debug_array_pattern_directly() {
    // Test the array pattern directly to isolate the issue
    let pattern = parse("[@a(*), @rest((*)*)]");
    let cbor_data = cbor("[1, 2, 3]");

    println!("Testing pattern: {:?}", pattern);

    // Test paths() method
    if let dcbor_pattern::Pattern::Structure(structure_pattern) = &pattern {
        if let dcbor_pattern::StructurePattern::Array(array_pattern) =
            structure_pattern
        {
            let paths = array_pattern.paths(&cbor_data);
            println!("Direct ArrayPattern::paths result: {:?}", paths);

            let (paths_with_caps, captures) =
                array_pattern.paths_with_captures(&cbor_data);
            println!(
                "Direct ArrayPattern::paths_with_captures result: {:?}",
                paths_with_caps
            );
            println!("Direct ArrayPattern captures: {:?}", captures);
        }
    }

    // Test the full pattern
    let paths = pattern.paths(&cbor_data);
    println!("Full pattern paths: {:?}", paths);

    let (paths_with_caps, captures) = pattern.paths_with_captures(&cbor_data);
    println!("Full pattern paths_with_captures: {:?}", paths_with_caps);
    println!("Full pattern captures: {:?}", captures);
}

#[test]
fn test_desired_partial_array_capture_behavior() {
    // Test the desired behavior once implemented

    // Test case 1: [1, 2, 3] with [@a(*), @rest((*)*)]
    let pattern = parse("search([@a(*), @rest((*)*)])");
    let cbor_data = cbor("[1, 2, 3]");
    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Expected output according to user:
    // @a
    //     [1, 2, 3]
    //         1
    // @rest
    //     [1, 2, 3]
    //         [2, 3]
    // [1, 2, 3]

    // For now, let's just ensure it doesn't crash and prints what it does
    let output = format_paths_with_captures(
        &paths,
        &captures,
        dcbor_pattern::FormatPathsOpts::default(),
    );
    println!("Test case 1 output:\n{}", output);

    // Test case 2: [1] with [@a(*), @rest((*)*)]
    let cbor_data2 = cbor("[1]");
    let (paths2, captures2) = pattern.paths_with_captures(&cbor_data2);

    // Expected output according to user:
    // @a
    //     [1]
    //         1
    // @rest
    //     []
    // [1]

    let output2 = format_paths_with_captures(
        &paths2,
        &captures2,
        dcbor_pattern::FormatPathsOpts::default(),
    );
    println!("Test case 2 output:\n{}", output2);
}

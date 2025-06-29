mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn debug_exact_count_behavior() {
    let single_element = parse_dcbor_item("[42]").unwrap();
    let multiple_elements = parse_dcbor_item("[1, 2]").unwrap();

    // Test [{1}] - should match exactly one element
    let exact_one_pattern = Pattern::parse("[{1}]").unwrap();
    println!("Pattern [{{1}}]:");
    println!("  Single element [42]: {}", exact_one_pattern.matches(&single_element));
    println!("  Multiple elements [1, 2]: {}", exact_one_pattern.matches(&multiple_elements));

    // Test [(ANY){1}] - should match exactly one ANY element
    let any_exact_one_pattern = Pattern::parse("[(ANY){1}]").unwrap();
    println!("Pattern [(ANY){{1}}]:");
    println!("  Single element [42]: {}", any_exact_one_pattern.matches(&single_element));
    println!("  Multiple elements [1, 2]: {}", any_exact_one_pattern.matches(&multiple_elements));
}

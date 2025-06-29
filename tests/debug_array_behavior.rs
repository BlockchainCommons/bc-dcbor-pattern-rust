mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn debug_array_single_element_behavior() {
    let single_element = parse_dcbor_item("[42]").unwrap();
    let multiple_elements = parse_dcbor_item("[1, 2]").unwrap();

    // Test [NUMBER] - should match single, not multiple
    let number_pattern = Pattern::parse("[NUMBER]").unwrap();
    println!("Pattern [NUMBER]:");
    println!(
        "  Single element [42]: {}",
        number_pattern.matches(&single_element)
    );
    println!(
        "  Multiple elements [1, 2]: {}",
        number_pattern.matches(&multiple_elements)
    );

    // Test [(ANY)] - should match single, not multiple
    let any_pattern = Pattern::parse("[(ANY)]").unwrap();
    println!("Pattern [(ANY)]:");
    println!(
        "  Single element [42]: {}",
        any_pattern.matches(&single_element)
    );
    println!(
        "  Multiple elements [1, 2]: {}",
        any_pattern.matches(&multiple_elements)
    );

    // Test [ANY] - what happens with this?
    let any_no_parens_pattern = Pattern::parse("[ANY]").unwrap();
    println!("Pattern [ANY]:");
    println!(
        "  Single element [42]: {}",
        any_no_parens_pattern.matches(&single_element)
    );
    println!(
        "  Multiple elements [1, 2]: {}",
        any_no_parens_pattern.matches(&multiple_elements)
    );
}

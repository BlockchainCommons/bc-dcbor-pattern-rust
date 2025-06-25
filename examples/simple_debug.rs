use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

fn main() {
    println!("Testing simple repeat pattern...");

    // Create a simple repeat pattern without maximum bounds
    let any_star = Pattern::repeat(
        Pattern::any(),
        dcbor_pattern::Quantifier::new(
            0..=2,
            dcbor_pattern::Reluctance::Greedy,
        ), // Limit to max 2 for safety
    );

    println!("Repeat pattern: {}", any_star);

    // Test against empty array
    let empty_array = parse_dcbor_item("[]").unwrap();
    println!("Empty array []: {}", any_star.matches(&empty_array));

    // Test against single element array
    let single_array = parse_dcbor_item("[42]").unwrap();
    println!("Single array [42]: {}", any_star.matches(&single_array));

    // Simple fixed sequence: exactly 3 patterns
    let simple_sequence = Pattern::sequence(vec![
        Pattern::any(),      // Match any first element
        Pattern::number(42), // Match 42 in middle
        Pattern::any(),      // Match any last element
    ]);

    println!("Simple sequence: {}", simple_sequence);

    // Test the simple sequence directly (not in array yet)
    let three_elem_array = parse_dcbor_item("[1, 42, 3]").unwrap();
    println!(
        "Sequence matches [1, 42, 3]: {}",
        simple_sequence.matches(&three_elem_array)
    );
}

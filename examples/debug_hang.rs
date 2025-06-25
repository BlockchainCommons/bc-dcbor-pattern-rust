// Temporary debug program to understand the hanging issue

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::*;

fn main() {
    println!("Testing simple cases first...");

    // Create pattern step by step
    let any_star = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=usize::MAX, Reluctance::Greedy),
    );
    println!("✅ Created any_star: {}", any_star);

    let number_42 = Pattern::number(42);
    println!("✅ Created number_42: {}", number_42);

    let sequence = Pattern::sequence(vec![
        any_star.clone(),
        number_42.clone(),
        any_star.clone(),
    ]);
    println!("✅ Created sequence: {}", sequence);

    let array_pattern = Pattern::Structure(StructurePattern::Array(
        ArrayPattern::with_elements(sequence),
    ));
    println!("✅ Created array pattern: {}", array_pattern);

    // Test simple case: [42]
    let test_array = parse_dcbor_item("[42]").unwrap();
    println!("Test array: {}", test_array);

    // Test individual components first
    println!(
        "Does any_star match 42? {}",
        any_star.matches(&parse_dcbor_item("42").unwrap())
    );
    println!(
        "Does number_42 match 42? {}",
        number_42.matches(&parse_dcbor_item("42").unwrap())
    );

    println!("About to test array pattern matching...");
    let result = array_pattern.matches(&test_array);
    println!("Result: {}", result);
}

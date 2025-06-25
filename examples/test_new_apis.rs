use dcbor_pattern::{Matcher, Pattern};

fn main() {
    // Test the new convenience methods
    let array_pattern = Pattern::any_array();
    let map_pattern = Pattern::any_map();
    let tagged_pattern = Pattern::any_tagged();

    println!("Array pattern: {}", array_pattern);
    println!("Map pattern: {}", map_pattern);
    println!("Tagged pattern: {}", tagged_pattern);

    // Test the new sequence pattern
    let sequence = Pattern::sequence(vec![
        Pattern::text("first"),
        Pattern::text("second"),
        Pattern::text("third"),
    ]);

    println!("Sequence pattern: {}", sequence);

    // Test sequence properties
    let mut capture_names = Vec::new();
    sequence.collect_capture_names(&mut capture_names);
    println!("Sequence has {} captures", capture_names.len());
    println!("Sequence is complex: {}", sequence.is_complex());
}

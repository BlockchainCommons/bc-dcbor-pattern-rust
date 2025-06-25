use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{ArrayPattern, Matcher, Pattern, StructurePattern};

fn test_with_timeout<F>(test_fn: F, timeout_secs: u64, test_name: &str) -> bool
where
    F: FnOnce() -> bool + Send + 'static,
{
    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();

    let handle = thread::spawn(move || {
        let test_result = test_fn();
        *result_clone.lock().unwrap() = Some(test_result);
    });

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(timeout_secs) {
        if let Ok(mut guard) = result.try_lock() {
            if let Some(res) = *guard {
                println!("✅ {} completed: {}", test_name, res);
                return res;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

    println!("⏰ {} timed out after {} seconds", test_name, timeout_secs);
    false
}

fn main() {
    println!("Testing array pattern matching with timeout protection...\n");

    // Test the exact pattern that's failing
    let any_star = Pattern::repeat(
        Pattern::any(),
        dcbor_pattern::Quantifier::new(
            0..=usize::MAX,
            dcbor_pattern::Reluctance::Greedy,
        ),
    );

    let sequence_with_repeats = Pattern::sequence(vec![
        any_star.clone(),
        Pattern::number(42),
        any_star.clone(),
    ]);

    let array_with_repeats = Pattern::Structure(StructurePattern::Array(
        ArrayPattern::with_elements(sequence_with_repeats.clone()),
    ));

    println!("Pattern: {}", array_with_repeats);

    // Test case that should work: [42] - with timeout
    let test_cbor = parse_dcbor_item("[42]").unwrap();
    println!("Testing [42]:");
    println!("  CBOR: {}", test_cbor);

    let cbor_clone = test_cbor.clone();
    let pattern_clone = array_with_repeats.clone();

    let matches = test_with_timeout(
        move || pattern_clone.matches(&cbor_clone),
        2, // 2 second timeout
        "[42] match test",
    );

    if !matches {
        println!("❌ Test failed or timed out");
    }

    // Debug: test individual components (these should be fast)
    println!("\nDebugging components:");
    println!(
        "  Pattern number(42) matches 42: {}",
        Pattern::number(42).matches(&parse_dcbor_item("42").unwrap())
    );
    println!(
        "  Pattern any() matches 42: {}",
        Pattern::any().matches(&parse_dcbor_item("42").unwrap())
    );

    // Test simpler patterns first
    println!("\nTesting simpler patterns:");

    // Test simple array pattern (no repeats)
    let simple_array = Pattern::Structure(StructurePattern::Array(
        ArrayPattern::with_elements(Pattern::number(42)),
    ));

    let simple_matches = test_with_timeout(
        move || simple_array.matches(&test_cbor),
        1, // 1 second timeout
        "Simple ARRAY(NUMBER(42)) test",
    );

    println!("Simple array pattern result: {}", simple_matches);
}

use dcbor_pattern::{Pattern, Matcher};
use dcbor::prelude::*;

fn main() {
    // Test sequence pattern compilation and matching
    let sequence = Pattern::sequence(vec![
        Pattern::text("hello"),
        Pattern::number(42),
        Pattern::any_bool(),
    ]);

    println!("Sequence pattern: {}", sequence);

    // Compile the pattern to see the VM code
    let mut code = Vec::new();
    let mut literals = Vec::new();
    let mut captures = Vec::new();

    sequence.compile(&mut code, &mut literals, &mut captures);

    println!("Generated {} VM instructions", code.len());
    println!("Generated {} literals", literals.len());
    println!("Has {} captures", captures.len());

    // Test with some CBOR data
    let test_cbor = vec!["hello".to_cbor(), 42.to_cbor(), true.to_cbor()];

    println!("Testing with {} CBOR values:", test_cbor.len());
    for (i, cbor) in test_cbor.iter().enumerate() {
        let paths = sequence.paths(cbor);
        println!("  Value {}: {} paths found", i, paths.len());
    }

    // Test the new convenience methods with CBOR
    let array_cbor = vec![1, 2, 3].to_cbor();
    let any_array = Pattern::any_array();
    let array_paths = any_array.paths(&array_cbor);
    println!("Array pattern found {} paths in array CBOR", array_paths.len());

    let map_cbor = vec![("key", "value")].into_iter().collect::<std::collections::BTreeMap<_, _>>().to_cbor();
    let any_map = Pattern::any_map();
    let map_paths = any_map.paths(&map_cbor);
    println!("Map pattern found {} paths in map CBOR", map_paths.len());
}

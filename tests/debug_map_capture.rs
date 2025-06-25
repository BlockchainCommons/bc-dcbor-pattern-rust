use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn debug_map_capture() {
    let pattern =
        Pattern::parse(r#"MAP(@key(TEXT("name")): @value(TEXT("Alice")))"#)
            .unwrap();
    let cbor = parse_dcbor_item(r#"{"name": "Alice"}"#).unwrap();

    println!("Pattern: {}", pattern);
    println!("CBOR: {:?}", cbor);

    // Test regular match first
    let paths = pattern.paths(&cbor);
    println!("Regular paths: {:?}", paths);

    // Check if pattern has captures
    let mut capture_names = Vec::new();
    pattern.collect_capture_names(&mut capture_names);
    println!("Capture names: {:?}", capture_names);

    // Test if we can extract the structure pattern and test it directly
    if let dcbor_pattern::Pattern::Structure(
        dcbor_pattern::StructurePattern::Map(map_pattern),
    ) = &pattern
    {
        println!("Map pattern variant: {:?}", map_pattern);
        let (direct_paths, direct_captures) =
            map_pattern.paths_with_captures(&cbor);
        println!("Direct map paths: {:?}", direct_paths);
        println!("Direct map captures: {:?}", direct_captures);
    }

    // Test with captures using the main API
    let (capture_paths, captures) = pattern.paths_with_captures(&cbor);
    println!("API capture paths: {:?}", capture_paths);
    println!("API captures: {:?}", captures);
}

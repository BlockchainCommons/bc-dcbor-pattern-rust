use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn debug_search_capture() {
    let pattern = Pattern::parse("SEARCH(@found(NUMBER(42)))").unwrap();
    let cbor = parse_dcbor_item(r#"[1, [2, 42], 3]"#).unwrap();

    println!("Pattern: {}", pattern);
    println!("CBOR: {:?}", cbor);

    // Test regular match first
    let paths = pattern.paths(&cbor);
    println!("Regular paths: {:?}", paths);

    // Check if pattern has captures
    let mut capture_names = Vec::new();
    pattern.collect_capture_names(&mut capture_names);
    println!("Capture names: {:?}", capture_names);

    // Test with captures using direct method
    let (capture_paths, captures) = pattern.paths_with_captures(&cbor);
    println!("Direct capture paths: {:?}", capture_paths);
    println!("Direct captures: {:?}", captures);

    // Test with public API
    let (api_paths, api_captures) = pattern.paths_with_captures(&cbor);
    println!("API capture paths: {:?}", api_paths);
    println!("API captures: {:?}", api_captures);
}

use dcbor_pattern::{Pattern, Matcher};
use dcbor_parse::parse_dcbor_item;

#[test]
fn debug_tagged_capture() {
    let pattern = Pattern::parse("TAG(1, @content(NUMBER(42)))").unwrap();
    let cbor = parse_dcbor_item("1(42)").unwrap();

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

    // Check the capture details
    if let Some(content_paths) = captures.get("content") {
        for (i, path) in content_paths.iter().enumerate() {
            println!("Capture 'content' path {}: length={}, elements={:?}", i, path.len(), path);
        }
    }
}

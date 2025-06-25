// Debug array pattern paths_with_captures implementation

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::*;

#[test]
fn debug_array_pattern_paths_with_captures() {
    // Parse the inner capture pattern directly
    let inner_pattern = Pattern::parse("@item(NUMBER(42))").unwrap();
    let cbor = parse_dcbor_item("[42]").unwrap();

    println!("Inner pattern: {}", inner_pattern);

    // Test the inner pattern directly on the array
    let (inner_paths, inner_captures) =
        inner_pattern.match_with_captures(&cbor);
    println!(
        "Inner pattern on array - paths: {:?}, captures: {:?}",
        inner_paths, inner_captures
    );

    // Test the inner pattern on the array element
    let element = parse_dcbor_item("42").unwrap();
    let (element_paths, element_captures) =
        inner_pattern.match_with_captures(&element);
    println!(
        "Inner pattern on element - paths: {:?}, captures: {:?}",
        element_paths, element_captures
    );

    // Now test what happens when we call paths() on the inner pattern with the
    // array
    let pattern_paths = inner_pattern.paths(&cbor);
    println!("Inner pattern.paths(&array): {:?}", pattern_paths);
}

#[test]
fn debug_array_element_traversal() {
    use dcbor::CBORCase;

    let cbor = parse_dcbor_item("[42]").unwrap();

    if let CBORCase::Array(arr) = cbor.as_case() {
        println!("Array elements: {:?}", arr);

        for (i, element) in arr.iter().enumerate() {
            println!("Element {}: {:?}", i, element);

            let pattern = Pattern::parse("@item(NUMBER(42))").unwrap();
            let (paths, captures) = pattern.match_with_captures(element);
            println!(
                "Pattern on element {} - paths: {:?}, captures: {:?}",
                i, paths, captures
            );
        }
    }
}

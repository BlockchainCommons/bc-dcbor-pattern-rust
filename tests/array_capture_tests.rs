mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

#[test]
fn test_array_capture_basic() {
    let pattern = parse("ARRAY(@item(NUMBER(42)))");
    let cbor_data = cbor("[42]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Verify capture
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("item"));
    let captured_paths = &captures["item"];
    assert_eq!(captured_paths.len(), 1);
    // Capture contains path from array to element: [array, element]
    assert_eq!(captured_paths[0], vec![cbor("[42]"), cbor("42")]);

    // Also test normal paths for comparison
    let normal_paths = pattern.paths(&cbor_data);
    assert_actual_expected!(format_paths(&normal_paths), expected_paths);
}

#[test]
fn test_array_capture_multiple_items() {
    let pattern = parse("ARRAY(@first(NUMBER) > @second(NUMBER))");
    let cbor_data = cbor("[42, 100]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [42, 100]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Verify captures
    assert_eq!(captures.len(), 2);
    assert!(captures.contains_key("first"));
    assert!(captures.contains_key("second"));
    let first_captured = &captures["first"];
    let second_captured = &captures["second"];
    assert_eq!(first_captured.len(), 1);
    assert_eq!(second_captured.len(), 1);
    // Each capture contains path from array to element
    assert_eq!(first_captured[0], vec![cbor("[42, 100]"), cbor("42")]);
    assert_eq!(second_captured[0], vec![cbor("[42, 100]"), cbor("100")]);
}

#[test]
fn test_array_capture_with_any_pattern() {
    let pattern = parse("ARRAY(@any_item(ANY))");
    let cbor_data = cbor("[\"hello\"]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Array patterns may match the array itself and its elements
    assert!(!paths.is_empty(), "Should have at least one path");

    // Verify capture - when using ANY, it might capture all elements
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("any_item"));
    let captured_paths = &captures["any_item"];
    assert!(!captured_paths.is_empty(), "Should have captured something");
    // Check that one of the captures is the element
    assert!(
        captured_paths
            .iter()
            .any(|path| path.len() == 2 && path[1] == cbor("\"hello\""))
    );
}

#[test]
fn test_array_vm_compilation_and_execution() {
    let pattern = parse("ARRAY(@item(NUMBER(42)))");

    let mut code = Vec::new();
    let mut literals = Vec::new();
    let mut captures = Vec::new();

    pattern.compile(&mut code, &mut literals, &mut captures);
    code.push(dcbor_pattern::Instr::Accept);

    // Verify compilation structure
    assert!(!code.is_empty(), "Code should not be empty");
    assert!(!captures.is_empty(), "Should have at least one capture name");
    assert!(
        captures.contains(&"item".to_string()),
        "Should contain 'item' capture"
    );

    let program =
        dcbor_pattern::Program { code, literals, capture_names: captures };

    let cbor_data = cbor("[42]");
    let (vm_paths, vm_captures) = dcbor_pattern::run(&program, &cbor_data);

    #[rustfmt::skip]
    let expected_vm_paths = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&vm_paths), expected_vm_paths);

    // Verify VM captures
    assert_eq!(vm_captures.len(), 1);
    assert!(vm_captures.contains_key("item"));
    let vm_captured_paths = &vm_captures["item"];
    assert_eq!(vm_captured_paths.len(), 1);
    assert_eq!(vm_captured_paths[0], vec![cbor("[42]"), cbor("42")]);
}

#[test]
fn test_array_nested_capture() {
    let pattern = parse("@arr(ARRAY(@item(NUMBER)))");
    let cbor_data = cbor("[99]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [99]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Verify captures
    assert_eq!(captures.len(), 2);

    assert!(captures.contains_key("arr"));
    let arr_captured = &captures["arr"];
    assert_eq!(arr_captured.len(), 1);
    assert_eq!(arr_captured[0], vec![cbor("[99]")]);

    assert!(captures.contains_key("item"));
    let item_captured = &captures["item"];
    assert_eq!(item_captured.len(), 1);
    // Item capture is nested: [outer_array, inner_element]
    assert_eq!(item_captured[0], vec![cbor("[99]"), cbor("99")]);
}

#[test]
fn test_array_capture_non_matching() {
    let pattern = parse("ARRAY(@item(NUMBER(42)))");
    let cbor_data = cbor("[100]"); // Different number

    // Should not match
    assert!(!pattern.matches(&cbor_data));

    let paths = pattern.paths(&cbor_data);
    assert!(
        paths.is_empty(),
        "No paths should be returned for non-matching pattern"
    );
}

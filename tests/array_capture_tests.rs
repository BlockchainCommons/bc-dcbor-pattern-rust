mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    Matcher, Pattern, format_paths, format_paths_with_captures,
};
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

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @item
            [42]
                42
        [42]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    // Also test normal paths for comparison
    let normal_paths = pattern.paths(&cbor_data);
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&normal_paths), expected_paths);
}

#[test]
fn test_array_capture_multiple_items() {
    let pattern = parse("ARRAY(@first(NUMBER), @second(NUMBER))");
    let cbor_data = cbor("[42, 100]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @first
            [42, 100]
                42
        @second
            [42, 100]
                100
        [42, 100]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );
}

#[test]
fn test_array_capture_with_any_pattern() {
    let pattern = parse("ARRAY(@any_item(ANY))");
    let cbor_data = cbor("[\"hello\"]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @any_item
            ["hello"]
                "hello"
        ["hello"]
            "hello"
        ["hello"]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
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
    assert!(
        !captures.is_empty(),
        "Should have at least one capture name"
    );
    assert!(
        captures.contains(&"item".to_string()),
        "Should contain 'item' capture"
    );

    let program =
        dcbor_pattern::Program { code, literals, capture_names: captures };

    let cbor_data = cbor("[42]");
    let (vm_paths, vm_captures) = dcbor_pattern::run(&program, &cbor_data);

    // Validate VM execution with formatted output
    #[rustfmt::skip]
    let expected_vm_output = indoc! {r#"
        @item
            [42]
                42
        [42]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &vm_paths,
            &vm_captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_vm_output
    );
}

#[test]
fn test_array_nested_capture() {
    let pattern = parse("@arr(ARRAY(@item(NUMBER)))");
    let cbor_data = cbor("[99]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Validate formatted output with nested captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @arr
            [99]
        @item
            [99]
                99
        [99]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );
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

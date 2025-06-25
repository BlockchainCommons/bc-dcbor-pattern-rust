mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, Result, format_paths};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

#[test]
fn test_simple_pattern_without_capture() -> Result<()> {
    let pattern = parse("NUMBER(42)");
    let cbor_data = cbor("42");

    let paths = pattern.paths(&cbor_data);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    Ok(())
}

#[test]
fn test_simple_pattern_with_capture() -> Result<()> {
    let pattern = parse("@num(NUMBER(42))");
    let cbor_data = cbor("42");

    // Test normal paths
    let paths = pattern.paths(&cbor_data);
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Test paths with captures
    let (vm_paths, captures) = pattern.paths_with_captures(&cbor_data);
    #[rustfmt::skip]
    let expected_vm_paths = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(format_paths(&vm_paths), expected_vm_paths);

    // Verify capture is present
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("num"));
    let captured_paths = &captures["num"];
    assert_eq!(captured_paths.len(), 1);
    // Simple capture just contains the element itself
    assert_eq!(captured_paths[0], vec![cbor_data]);

    Ok(())
}

#[test]
fn test_vm_compilation_and_execution() -> Result<()> {
    let pattern = parse("@num(NUMBER(42))");

    let mut code = Vec::new();
    let mut literals = Vec::new();
    let mut captures = Vec::new();

    pattern.compile(&mut code, &mut literals, &mut captures);
    code.push(dcbor_pattern::Instr::Accept);

    // Verify compilation produces expected structure
    assert!(!code.is_empty(), "Code should not be empty");
    assert_eq!(captures.len(), 1, "Should have one capture name");
    assert_eq!(captures[0], "num", "Capture name should be 'num'");

    let program =
        dcbor_pattern::Program { code, literals, capture_names: captures };

    let cbor_data = cbor("42");
    let (vm_paths, vm_captures) = dcbor_pattern::run(&program, &cbor_data);

    // Verify VM execution results
    #[rustfmt::skip]
    let expected_vm_paths = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(format_paths(&vm_paths), expected_vm_paths);

    // Verify captures
    assert_eq!(vm_captures.len(), 1);
    assert!(vm_captures.contains_key("num"));
    let captured_paths = &vm_captures["num"];
    assert_eq!(captured_paths.len(), 1);
    // Simple capture just contains the element itself
    assert_eq!(captured_paths[0], vec![cbor_data]);

    Ok(())
}

#[test]
fn test_capture_with_array_pattern() -> Result<()> {
    let pattern = parse("@arr(ARRAY(NUMBER(42)))");
    let cbor_data = cbor("[42]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("arr"));
    let captured_paths = &captures["arr"];
    assert_eq!(captured_paths.len(), 1);
    assert_eq!(captured_paths[0], vec![cbor_data]);

    Ok(())
}

#[test]
fn test_capture_with_nested_pattern() -> Result<()> {
    let pattern = parse("@outer(ARRAY(@inner(NUMBER(42))))");
    let cbor_data = cbor("[42]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    assert_eq!(captures.len(), 2);

    // Check outer capture
    assert!(captures.contains_key("outer"));
    let outer_captured = &captures["outer"];
    assert_eq!(outer_captured.len(), 1);
    assert_eq!(outer_captured[0], vec![cbor("[42]")]);

    // Check inner capture
    assert!(captures.contains_key("inner"));
    let inner_captured = &captures["inner"];
    assert_eq!(inner_captured.len(), 1);
    // Inner capture includes the path from the root: [array, 42]
    assert_eq!(inner_captured[0], vec![cbor("[42]"), cbor("42")]);

    Ok(())
}

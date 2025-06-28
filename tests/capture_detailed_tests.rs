mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    FormatPathsOpts, Matcher, Pattern, Result, format_paths,
    format_paths_with_captures,
};
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

    // Test paths with captures using the proper rubric
    let (vm_paths, captures) = pattern.paths_with_captures(&cbor_data);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        @num
            42
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &vm_paths,
            &captures,
            FormatPathsOpts::default()
        ),
        expected
    );

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

    // Verify VM execution results using the proper rubric
    #[rustfmt::skip]
    let expected = indoc! {r#"
        @num
            42
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &vm_paths,
            &vm_captures,
            FormatPathsOpts::default()
        ),
        expected
    );

    Ok(())
}

#[test]
fn test_capture_with_array_pattern() -> Result<()> {
    let pattern = parse("@arr([NUMBER(42)])");
    let cbor_data = cbor("[42]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @arr
            [42]
        [42]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            FormatPathsOpts::default()
        ),
        expected
    );

    Ok(())
}

#[test]
fn test_capture_with_nested_pattern() -> Result<()> {
    let pattern = parse("@outer([@inner(NUMBER(42)]))");
    let cbor_data = cbor("[42]");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @inner
            [42]
                42
        @outer
            [42]
        [42]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            FormatPathsOpts::default()
        ),
        expected
    );

    Ok(())
}

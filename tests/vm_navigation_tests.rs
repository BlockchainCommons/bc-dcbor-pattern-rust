mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{format_paths, *};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

#[test]
fn test_vm_array_navigation() {
    // Test how PushAxis(ArrayElement) works
    let cbor_data = cbor("[42]");

    // Create a simple program that navigates to array elements and captures
    // them
    let code = vec![
        Instr::PushAxis(Axis::ArrayElement), // Navigate to array elements
        Instr::CaptureStart(0),              // Start capture
        Instr::MatchPredicate(0),            // Match NUMBER(42)
        Instr::CaptureEnd(0),                // End capture
        Instr::Accept,                       // Accept the match
    ];

    let literals = vec![
        Pattern::number(42), // Pattern to match
    ];

    let capture_names = vec!["item".to_string()];

    let program = Program { code, literals, capture_names };

    let (vm_paths, vm_captures) = run(&program, &cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [42]
            42
    "#}.trim();
    assert_actual_expected!(format_paths(&vm_paths), expected_paths);

    // Verify capture
    assert_eq!(vm_captures.len(), 1);
    assert!(vm_captures.contains_key("item"));
    let captured_paths = &vm_captures["item"];
    assert_eq!(captured_paths.len(), 1);
    assert_eq!(captured_paths[0], vec![cbor("[42]"), cbor("42")]);
}

#[test]
fn test_vm_map_navigation() {
    // Test how PushAxis(MapValue) works
    let cbor_data = cbor(r#"{"key": "value"}"#);

    let code = vec![
        Instr::PushAxis(Axis::MapValue), // Navigate to map values
        Instr::CaptureStart(0),          // Start capture
        Instr::MatchPredicate(0),        // Match "value"
        Instr::CaptureEnd(0),            // End capture
        Instr::Accept,                   // Accept the match
    ];

    let literals = vec![
        Pattern::text("value"), // Pattern to match
    ];

    let capture_names = vec!["value".to_string()];

    let program = Program { code, literals, capture_names };

    let (vm_paths, vm_captures) = run(&program, &cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        {"key": "value"}
            "value"
    "#}.trim();
    assert_actual_expected!(format_paths(&vm_paths), expected_paths);

    // Verify capture
    assert_eq!(vm_captures.len(), 1);
    assert!(vm_captures.contains_key("value"));
    let captured_paths = &vm_captures["value"];
    assert_eq!(captured_paths.len(), 1);
    assert_eq!(
        captured_paths[0],
        vec![cbor(r#"{"key": "value"}"#), cbor(r#""value""#)]
    );
}

#[test]
fn test_vm_nested_navigation() {
    // Test navigation through nested structures
    let cbor_data = cbor(r#"[{"inner": 42}]"#);

    let code = vec![
        Instr::PushAxis(Axis::ArrayElement), // Navigate to array elements
        Instr::PushAxis(Axis::MapValue),     // Navigate to map values
        Instr::CaptureStart(0),              // Start capture
        Instr::MatchPredicate(0),            // Match NUMBER(42)
        Instr::CaptureEnd(0),                // End capture
        Instr::Accept,                       // Accept the match
    ];

    let literals = vec![
        Pattern::number(42), // Pattern to match
    ];

    let capture_names = vec!["nested".to_string()];

    let program = Program { code, literals, capture_names };

    let (vm_paths, vm_captures) = run(&program, &cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [{"inner": 42}]
            {"inner": 42}
                42
    "#}.trim();
    assert_actual_expected!(format_paths(&vm_paths), expected_paths);

    // Verify capture
    assert_eq!(vm_captures.len(), 1);
    assert!(vm_captures.contains_key("nested"));
    let captured_paths = &vm_captures["nested"];
    assert_eq!(captured_paths.len(), 1);
    assert_eq!(
        captured_paths[0],
        vec![
            cbor(r#"[{"inner": 42}]"#),
            cbor(r#"{"inner": 42}"#),
            cbor("42")
        ]
    );
}

#[test]
fn test_vm_multiple_captures() {
    // Test multiple captures in sequence
    let cbor_data = cbor("[42, 100]");

    let code = vec![
        Instr::PushAxis(Axis::ArrayElement), // Navigate to array elements
        Instr::CaptureStart(0),              // Start first capture
        Instr::MatchPredicate(0),            // Match ANY
        Instr::CaptureEnd(0),                // End first capture
        Instr::Accept,                       // Accept the match
    ];

    let literals = vec![
        Pattern::any(), // Pattern to match any element
    ];

    let capture_names = vec!["element".to_string()];

    let program = Program { code, literals, capture_names };

    let (vm_paths, vm_captures) = run(&program, &cbor_data);

    // Should capture both elements
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [42, 100]
            100
        [42, 100]
            42
    "#}.trim();
    assert_actual_expected!(format_paths(&vm_paths), expected_paths);

    // Note: VM captures may contain multiple entries for the same name
    // when matching multiple elements
    assert!(!vm_captures.is_empty());
}

#[test]
fn test_vm_no_match_navigation() {
    // Test navigation when no match is found
    let cbor_data = cbor("[100]"); // Different number

    let code = vec![
        Instr::PushAxis(Axis::ArrayElement), // Navigate to array elements
        Instr::CaptureStart(0),              // Start capture
        Instr::MatchPredicate(0),            // Match NUMBER(42) - won't match
        Instr::CaptureEnd(0),                // End capture
        Instr::Accept,                       // Accept the match
    ];

    let literals = vec![
        Pattern::number(42), // Pattern that won't match
    ];

    let capture_names = vec!["item".to_string()];

    let program = Program { code, literals, capture_names };

    let (vm_paths, vm_captures) = run(&program, &cbor_data);

    // Should have no paths or captures when no match
    assert!(
        vm_paths.is_empty(),
        "No paths should be returned for non-matching pattern"
    );
    assert!(
        vm_captures.is_empty(),
        "No captures should be returned for non-matching pattern"
    );
}

// Debug VM array navigation

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::*;

#[test]
fn debug_vm_array_navigation() {
    // Test how PushAxis(ArrayElement) works
    let cbor = parse_dcbor_item("[42]").unwrap();

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

    let (vm_paths, vm_captures) = run(&program, &cbor);

    println!("VM paths: {:?}", vm_paths);
    println!("VM captures: {:?}", vm_captures);
}

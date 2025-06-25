// Debug array captures

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::*;

#[test]
fn debug_array_capture() {
    let pattern = Pattern::parse("ARRAY(@item(NUMBER(42)))").unwrap();
    let cbor = parse_dcbor_item("[42]").unwrap();

    println!("Pattern: {}", pattern);
    println!("CBOR: {}", cbor);

    let (paths, captures) = pattern.match_with_captures(&cbor);

    println!("Paths: {:?}", paths);
    println!("Captures: {:?}", captures);

    // Let's also test the normal paths to see if the pattern matches at all
    let normal_paths = pattern.paths(&cbor);
    println!("Normal paths: {:?}", normal_paths);
}

#[test]
fn debug_array_vm_compilation() {
    let pattern = Pattern::parse("ARRAY(@item(NUMBER(42)))").unwrap();

    let mut code = Vec::new();
    let mut literals = Vec::new();
    let mut captures = Vec::new();

    pattern.compile(&mut code, &mut literals, &mut captures);
    code.push(Instr::Accept);

    println!("Code: {:?}", code);
    println!("Literals: {:?}", literals);
    println!("Captures: {:?}", captures);

    let program = Program { code, literals, capture_names: captures };

    let cbor = parse_dcbor_item("[42]").unwrap();
    let (vm_paths, vm_captures) = run(&program, &cbor);

    println!("VM paths: {:?}", vm_paths);
    println!("VM captures: {:?}", vm_captures);
}

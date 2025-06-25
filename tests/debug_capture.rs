// Simple debug test to understand capture implementation

use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::*;

#[test]
fn test_simple_pattern_without_capture() -> Result<()> {
    let pattern = Pattern::parse("NUMBER(42)")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let paths = pattern.paths(&cbor);
    println!("Without capture - paths: {:?}", paths);

    Ok(())
}

#[test]
fn test_simple_pattern_with_capture() -> Result<()> {
    let pattern = Pattern::parse("@num(NUMBER(42))")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let paths = pattern.paths(&cbor);
    println!("With capture - normal paths: {:?}", paths);

    let (vm_paths, captures) = pattern.match_with_captures(&cbor);
    println!("With capture - VM paths: {:?}", vm_paths);
    println!("With capture - captures: {:?}", captures);

    Ok(())
}

#[test]
fn test_vm_compilation() -> Result<()> {
    let pattern = Pattern::parse("@num(NUMBER(42))")?;

    let mut code = Vec::new();
    let mut literals = Vec::new();
    let mut captures = Vec::new();

    pattern.compile(&mut code, &mut literals, &mut captures);
    code.push(Instr::Accept);

    println!("Code: {:?}", code);
    println!("Literals: {:?}", literals);
    println!("Captures: {:?}", captures);

    let program = Program { code, literals, capture_names: captures };

    let cbor = parse_dcbor_item("42").unwrap();
    let (vm_paths, vm_captures) = run(&program, &cbor);

    println!("VM paths: {:?}", vm_paths);
    println!("VM captures: {:?}", vm_captures);

    Ok(())
}

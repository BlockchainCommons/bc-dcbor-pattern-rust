use dcbor::prelude::*;
use dcbor_pattern::{Error, Matcher, Pattern};

fn main() -> Result<(), Error> {
    println!("Testing NUMBER pattern syntax from documentation...\n");

    // Test values - mix of integers and floats as they would appear in dCBOR
    let test_values = [
        5.to_cbor(),
        42.to_cbor(),
        (-10).to_cbor(),
        3.2222.to_cbor(),
        f64::NAN.to_cbor(),
        100.to_cbor(),
        0.to_cbor(),
        (-1).to_cbor(),
        1.5.to_cbor(),
        50.to_cbor(),
    ];

    let value_descriptions = ["5", "42", "-10", "3.2222", "NaN", "100", "0", "-1", "1.5", "50"];

    // Test NUMBER - matches any number
    println!("Testing 'NUMBER' (any number):");
    let pattern = Pattern::parse("NUMBER")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    // Test NUMBER(42) - exact value
    println!("Testing 'NUMBER(42)' (exact value):");
    let pattern = Pattern::parse("NUMBER(42)")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    // Test NUMBER(1...50) - range
    println!("Testing 'NUMBER(1...50)' (range):");
    let pattern = Pattern::parse("NUMBER(1...50)")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    // Test NUMBER(>=10) - greater than or equal
    println!("Testing 'NUMBER(>=10)' (greater than or equal):");
    let pattern = Pattern::parse("NUMBER(>=10)")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    // Test NUMBER(<=10) - less than or equal
    println!("Testing 'NUMBER(<=10)' (less than or equal):");
    let pattern = Pattern::parse("NUMBER(<=10)")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    // Test NUMBER(>0) - greater than
    println!("Testing 'NUMBER(>0)' (greater than):");
    let pattern = Pattern::parse("NUMBER(>0)")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    // Test NUMBER(<0) - less than
    println!("Testing 'NUMBER(<0)' (less than):");
    let pattern = Pattern::parse("NUMBER(<0)")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    // Test NUMBER(NaN) - NaN only
    println!("Testing 'NUMBER(NaN)' (NaN only):");
    let pattern = Pattern::parse("NUMBER(NaN)")?;
    for (i, value) in test_values.iter().enumerate() {
        let matches = pattern.matches(value);
        println!(
            "  {} -> {}",
            value_descriptions[i],
            if matches { "✓" } else { "✗" }
        );
    }
    println!();

    println!("All NUMBER pattern syntax tests completed successfully!");
    Ok(())
}

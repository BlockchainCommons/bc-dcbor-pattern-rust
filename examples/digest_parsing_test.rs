use dcbor_pattern::Pattern;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bc_components::register_tags();

    // Test parsing DIGEST without parentheses
    let pattern1 = Pattern::parse("DIGEST")?;
    println!("Parsed 'DIGEST' -> {}", pattern1);

    // Test parsing DIGEST with hex prefix
    let pattern2 = Pattern::parse("DIGEST(a1b2c3)")?;
    println!("Parsed 'DIGEST(a1b2c3)' -> {}", pattern2);

    // Test parsing DIGEST with full hex
    let pattern3 = Pattern::parse(
        "DIGEST(4d303dac9eed63573f6190e9c4191be619e03a7b3c21e9bb3d27ac1a55971e6b)",
    )?;
    println!(
        "Parsed 'DIGEST(4d303dac9eed63573f6190e9c4191be619e03a7b3c21e9bb3d27ac1a55971e6b)' -> {}",
        pattern3
    );

    println!("All digest parsing tests passed!");
    Ok(())
}

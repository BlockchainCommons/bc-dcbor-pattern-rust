use dcbor_pattern::Pattern;

fn main() {
    // Test known value regex parsing
    match Pattern::parse("KNOWN(/^is.*/)") {
        Ok(pattern) => {
            println!("✅ Known value regex parsing works: {}", pattern)
        }
        Err(e) => println!("❌ Known value regex parsing failed: {}", e),
    }

    // Test date range parsing
    match Pattern::parse("DATE(2023-01-01...2023-12-31)") {
        Ok(pattern) => println!("✅ Date range parsing works: {}", pattern),
        Err(e) => println!("❌ Date range parsing failed: {}", e),
    }

    // Test date regex parsing
    match Pattern::parse("DATE(/^2023-/)") {
        Ok(pattern) => println!("✅ Date regex parsing works: {}", pattern),
        Err(e) => println!("❌ Date regex parsing failed: {}", e),
    }
}

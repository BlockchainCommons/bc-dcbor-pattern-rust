use dcbor_pattern::Pattern;

fn main() {
    // Test 1: Valid pattern with no extra data
    match Pattern::parse("true") {
        Ok(pattern) => println!("✓ 'true' parsed successfully: {}", pattern),
        Err(e) => println!("✗ 'true' failed: {}", e),
    }

    // Test 2: Valid pattern followed by extra data
    match Pattern::parse("true extra") {
        Ok(pattern) => {
            println!("✓ 'true extra' parsed successfully: {}", pattern)
        }
        Err(e) => println!("✗ 'true extra' failed: {}", e),
    }

    // Test 3: Valid pattern followed by another pattern
    match Pattern::parse("true false") {
        Ok(pattern) => {
            println!("✓ 'true false' parsed successfully: {}", pattern)
        }
        Err(e) => println!("✗ 'true false' failed: {}", e),
    }

    // Test 4: Valid pattern followed by whitespace and more
    match Pattern::parse("42    more stuff") {
        Ok(pattern) => {
            println!("✓ '42    more stuff' parsed successfully: {}", pattern)
        }
        Err(e) => println!("✗ '42    more stuff' failed: {}", e),
    }
}

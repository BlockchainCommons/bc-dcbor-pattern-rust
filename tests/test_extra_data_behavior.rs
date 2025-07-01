#[cfg(test)]
mod extra_data_tests {
    use dcbor_pattern::{Error, Pattern};

    #[test]
    fn test_current_behavior_with_extra_data() {
        // Test 1: Valid pattern with no extra data
        let result = Pattern::parse("true");
        assert!(result.is_ok(), "Should parse 'true' successfully");

        // Test 2: Valid pattern followed by extra data
        let result = Pattern::parse("true extra");
        match result {
            Ok(_) => panic!("Expected error for 'true extra', but got success"),
            Err(Error::ExtraData(_)) => {
                println!("✓ 'true extra' correctly returns ExtraData error")
            }
            Err(Error::UnrecognizedToken(_)) => println!(
                "✓ 'true extra' returns UnrecognizedToken error (not ExtraData)"
            ),
            Err(e) => panic!(
                "Expected ExtraData or UnrecognizedToken error, got: {}",
                e
            ),
        }

        // Test 3: Valid pattern followed by another pattern
        let result = Pattern::parse("true false");
        match result {
            Ok(_) => panic!("Expected error for 'true false', but got success"),
            Err(Error::ExtraData(_)) => {
                println!("✓ 'true false' correctly returns ExtraData error")
            }
            Err(Error::UnrecognizedToken(_)) => println!(
                "✓ 'true false' returns UnrecognizedToken error (not ExtraData)"
            ),
            Err(e) => panic!(
                "Expected ExtraData or UnrecognizedToken error, got: {}",
                e
            ),
        }

        // Test 4: Valid pattern followed by whitespace and more
        let result = Pattern::parse("42    more stuff");
        match result {
            Ok(_) => {
                panic!("Expected error for '42    more stuff', but got success")
            }
            Err(Error::ExtraData(_)) => println!(
                "✓ '42    more stuff' correctly returns ExtraData error"
            ),
            Err(Error::UnrecognizedToken(_)) => println!(
                "✓ '42    more stuff' returns UnrecognizedToken error (not ExtraData)"
            ),
            Err(e) => panic!(
                "Expected ExtraData or UnrecognizedToken error, got: {}",
                e
            ),
        }

        // Test 5: Valid pattern followed by a valid token (should be ExtraData)
        let result = Pattern::parse("42 |");
        match result {
            Ok(_) => panic!("Expected error for '42 |', but got success"),
            Err(Error::ExtraData(_)) => {
                println!("✓ '42 |' correctly returns ExtraData error")
            }
            Err(Error::UnrecognizedToken(_)) => println!(
                "✓ '42 |' returns UnrecognizedToken error (not ExtraData)"
            ),
            Err(e) => println!("? '42 |' returns: {}", e),
        }
    }
}

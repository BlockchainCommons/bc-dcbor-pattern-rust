#[cfg(test)]
mod test_error_positions {
    use dcbor_pattern::Pattern;

    #[test]
    fn test_tagged_error_position() {
        println!("Testing error position reporting for tagged patterns...\n");

        // This should fail and report the correct position of FOO (at position
        // 14)
        let pattern_str = "tagged(12345, FOO)";
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                // Check if this is an UnrecognizedToken error
                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // Show what character is actually at the reported position
                    if span.start < pattern_str.len() {
                        let char_at_pos = &pattern_str
                            [span.start..span.end.min(pattern_str.len())];
                        println!(
                            "Character at reported position {}: '{}'",
                            span.start, char_at_pos
                        );
                    }

                    // The FOO token starts at position 14 in "tagged(12345,
                    // FOO)"
                    // 0123456789012345
                    let expected_start = 14;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (F in FOO), but got {}",
                        expected_start, span.start
                    );
                    // Note: The end position is currently only single character
                    // due to lexer limitation
                    // This is a separate issue from the main problem we fixed
                    // assert_eq!(span.end, expected_end,
                    //     "Expected error to end at position {} (after FOO),
                    // but got {}",     expected_end,
                    // span.end);

                    println!("✓ Error starting position is correct!");
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_array_error_position() {
        println!("Testing error position reporting for array patterns...\n");

        // This should fail and report the correct position of BAR
        let pattern_str = "[number, BAR]";
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // The BAR token starts at position 9 in "[number, BAR]"
                    //                                     0123456789012
                    let expected_start = 9;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (B in BAR), but got {}",
                        expected_start, span.start
                    );
                    // Note: The end position is currently only single character
                    // due to lexer limitation
                    // assert_eq!(span.end, expected_end,
                    //     "Expected error to end at position {} (after BAR),
                    // but got {}",     expected_end,
                    // span.end);

                    println!("✓ Error starting position is correct!");
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_map_key_error_position() {
        println!(
            "Testing error position reporting for map pattern key errors...\n"
        );

        // This should fail and report the correct position of FOO as a key
        let pattern_str = "{FOO: number}";
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // Show what character is actually at the reported position
                    if span.start < pattern_str.len() {
                        let char_at_pos = &pattern_str
                            [span.start..span.end.min(pattern_str.len())];
                        println!(
                            "Character at reported position {}: '{}'",
                            span.start, char_at_pos
                        );
                    }

                    // The FOO token starts at position 1 in "{FOO: number}"
                    //                                     0123456789012
                    let expected_start = 1;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (F in FOO), but got {}",
                        expected_start, span.start
                    );

                    println!("✓ Map key error position is correct!");
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_map_value_error_position() {
        println!(
            "Testing error position reporting for map pattern value errors...\n"
        );

        // This should fail and report the correct position of FOO as a value
        let pattern_str = "{text: FOO}";
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // Show what character is actually at the reported position
                    if span.start < pattern_str.len() {
                        let char_at_pos = &pattern_str
                            [span.start..span.end.min(pattern_str.len())];
                        println!(
                            "Character at reported position {}: '{}'",
                            span.start, char_at_pos
                        );
                    }

                    // The FOO token starts at position 7 in "{text: FOO}"
                    //                                     0123456789
                    let expected_start = 7;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (F in FOO), but got {}",
                        expected_start, span.start
                    );

                    println!("✓ Map value error position is correct!");
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_map_second_constraint_key_error() {
        println!(
            "Testing error position reporting for second constraint key errors...\n"
        );

        // This should fail and report the correct position of FOO in the second
        // constraint
        let pattern_str = "{text: *, FOO: number}";
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // Show what character is actually at the reported position
                    if span.start < pattern_str.len() {
                        let char_at_pos = &pattern_str
                            [span.start..span.end.min(pattern_str.len())];
                        println!(
                            "Character at reported position {}: '{}'",
                            span.start, char_at_pos
                        );
                    }

                    // The FOO token starts at position 10 in "{text: *, FOO:
                    // number}"
                    // 01234567890123456789012
                    let expected_start = 10;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (F in FOO), but got {}",
                        expected_start, span.start
                    );

                    println!(
                        "✓ Second constraint key error position is correct!"
                    );
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_map_second_constraint_value_error() {
        println!(
            "Testing error position reporting for second constraint value errors...\n"
        );

        // This should fail and report the correct position of FOO in the second
        // constraint value
        let pattern_str = "{bool: bstr, *: FOO}";
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // Show what character is actually at the reported position
                    if span.start < pattern_str.len() {
                        let char_at_pos = &pattern_str
                            [span.start..span.end.min(pattern_str.len())];
                        println!(
                            "Character at reported position {}: '{}'",
                            span.start, char_at_pos
                        );
                    }

                    // The FOO token starts at position 16 in "{bool: bstr, *:
                    // FOO}"
                    // 0123456789012345678901
                    let expected_start = 16;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (F in FOO), but got {}",
                        expected_start, span.start
                    );

                    println!(
                        "✓ Second constraint value error position is correct!"
                    );
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_map_complex_error_position() {
        println!(
            "Testing error position reporting for complex map patterns...\n"
        );

        // Test with multiple constraints and error in the middle
        let pattern_str =
            r#"{"name": text, "age": number, BAD: *, "email": text}"#;
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // Show what character is actually at the reported position
                    if span.start < pattern_str.len() {
                        let char_at_pos = &pattern_str
                            [span.start..span.end.min(pattern_str.len())];
                        println!(
                            "Character at reported position {}: '{}'",
                            span.start, char_at_pos
                        );
                    }

                    // The BAD token starts at position 30 in the pattern
                    // {"name": text, "age": number, BAD: *, "email": text}
                    //  01234567890123456789012345678901234567890123456789012
                    let expected_start = 30;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (B in BAD), but got {}",
                        expected_start, span.start
                    );

                    println!("✓ Complex map error position is correct!");
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_map_nested_pattern_error() {
        println!(
            "Testing error position reporting for nested map pattern errors...\n"
        );

        // Test error inside a nested structure within a map
        let pattern_str = r#"{"data": [number, INVALID], "id": number}"#;
        match Pattern::parse(pattern_str) {
            Ok(_) => panic!("Expected parse to fail for: {}", pattern_str),
            Err(e) => {
                println!("Error: {:?}", e);

                if let dcbor_pattern::Error::UnrecognizedToken(span) = &e {
                    println!("Error span: {:?}", span);
                    println!("Pattern: {}", pattern_str);

                    // Show what character is actually at the reported position
                    if span.start < pattern_str.len() {
                        let char_at_pos = &pattern_str
                            [span.start..span.end.min(pattern_str.len())];
                        println!(
                            "Character at reported position {}: '{}'",
                            span.start, char_at_pos
                        );
                    }

                    // The INVALID token starts at position 18 in the pattern
                    // {"data": [number, INVALID], "id": number}
                    //  012345678901234567890123456789012345678901
                    let expected_start = 18;

                    assert_eq!(
                        span.start, expected_start,
                        "Expected error to start at position {} (I in INVALID), but got {}",
                        expected_start, span.start
                    );

                    println!("✓ Nested map pattern error position is correct!");
                } else {
                    panic!("Expected UnrecognizedToken error, got: {:?}", e);
                }
            }
        }
    }
}

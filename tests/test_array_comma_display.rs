#[cfg(test)]
mod test_array_comma_display {
    use dcbor_pattern::Pattern;

    #[test]
    fn test_array_sequence_display_format() {
        // Test that array sequences display with commas
        let pattern = Pattern::parse(r#"ARRAY(TEXT("a"), TEXT("b"))"#).unwrap();
        let display = pattern.to_string();

        println!("Pattern display: {}", display);

        // Should contain commas, not >
        assert!(display.contains(","));
        assert!(!display.contains("TEXT(\"a\")>TEXT(\"b\")"));
    }

    #[test]
    fn test_complex_array_sequence_display() {
        let pattern = Pattern::parse(r#"ARRAY((ANY)*, NUMBER(42), (ANY)*)"#).unwrap();
        let display = pattern.to_string();

        println!("Complex pattern display: {}", display);

        // Should contain commas for array sequences
        assert!(display.contains(","));
    }
}

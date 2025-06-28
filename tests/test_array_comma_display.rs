#[cfg(test)]
mod test_array_comma_display {
    use dcbor_pattern::Pattern;

    #[test]
    fn test_array_sequence_display_format() {
        let pattern = Pattern::parse(r#"[TEXT("a"), TEXT("b")]"#).unwrap();
        let display = pattern.to_string();
        assert_eq!(display, r#"[TEXT("a"), TEXT("b")]"#);
    }

    #[test]
    fn test_complex_array_sequence_display() {
        let pattern = Pattern::parse(r#"[(ANY)*, NUMBER(42), (ANY)*]"#).unwrap();
        let display = pattern.to_string();
        assert_eq!(display, "[(ANY)*, NUMBER(42), (ANY)*]");
    }
}

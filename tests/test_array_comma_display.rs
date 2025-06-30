#[cfg(test)]
mod test_array_comma_display {
    use dcbor_pattern::Pattern;

    #[test]
    fn test_array_sequence_display_format() {
        let pattern = Pattern::parse(r#"["a", "b"]"#).unwrap();
        let display = pattern.to_string();
        assert_eq!(display, r#"["a", "b"]"#);
    }

    #[test]
    fn test_complex_array_sequence_display() {
        let pattern = Pattern::parse(r#"[(*)*, 42, (*)*]"#).unwrap();
        let display = pattern.to_string();
        assert_eq!(display, "[(*)*, 42, (*)*]");
    }
}

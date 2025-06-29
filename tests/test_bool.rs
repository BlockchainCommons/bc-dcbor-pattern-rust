#[cfg(test)]
mod test_new_bool_syntax {
    use dcbor::prelude::*;
    use dcbor_pattern::{Pattern, Matcher};

    #[test]
    fn test_bool_pattern_parsing() {
        // Test bool pattern (matches any boolean)
        let bool_pattern = Pattern::parse("bool").unwrap();
        assert_eq!(bool_pattern.to_string(), "bool");
        
        // Test true pattern
        let true_pattern = Pattern::parse("true").unwrap();
        assert_eq!(true_pattern.to_string(), "true");
        
        // Test false pattern
        let false_pattern = Pattern::parse("false").unwrap();
        assert_eq!(false_pattern.to_string(), "false");
    }

    #[test]
    fn test_bool_pattern_matching() {
        let bool_pattern = Pattern::parse("bool").unwrap();
        let true_pattern = Pattern::parse("true").unwrap();
        let false_pattern = Pattern::parse("false").unwrap();
        
        let true_cbor = true.to_cbor();
        let false_cbor = false.to_cbor();
        let number_cbor = 42.to_cbor();
        
        // Test bool pattern matching
        assert!(bool_pattern.matches(&true_cbor));
        assert!(bool_pattern.matches(&false_cbor));
        assert!(!bool_pattern.matches(&number_cbor));
        
        // Test true pattern matching
        assert!(true_pattern.matches(&true_cbor));
        assert!(!true_pattern.matches(&false_cbor));
        assert!(!true_pattern.matches(&number_cbor));
        
        // Test false pattern matching
        assert!(!false_pattern.matches(&true_cbor));
        assert!(false_pattern.matches(&false_cbor));
        assert!(!false_pattern.matches(&number_cbor));
    }

    #[test]
    fn test_bool_combinations() {
        // Test OR combinations
        let true_or_false = Pattern::parse("true | false").unwrap();
        let true_cbor = true.to_cbor();
        let false_cbor = false.to_cbor();
        let number_cbor = 42.to_cbor();
        
        assert!(true_or_false.matches(&true_cbor));
        assert!(true_or_false.matches(&false_cbor));
        assert!(!true_or_false.matches(&number_cbor));
        
        // Test with other patterns
        let bool_or_number = Pattern::parse("bool | NUMBER").unwrap();
        assert!(bool_or_number.matches(&true_cbor));
        assert!(bool_or_number.matches(&false_cbor));
        assert!(bool_or_number.matches(&number_cbor));
    }
}

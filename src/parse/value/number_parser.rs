use crate::{Pattern, Result, parse::Token};

pub(crate) fn parse_number(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // For the keyword "number", just return any number pattern
    Ok(Pattern::any_number())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_any() {
        let pattern = Pattern::parse("number").unwrap();
        assert_eq!(pattern.to_string(), "number");
    }

    #[test]
    fn test_number_exact() {
        let pattern = Pattern::parse("42").unwrap();
        assert_eq!(pattern.to_string(), "42");

        let pattern = Pattern::parse("3.2222").unwrap();
        assert_eq!(pattern.to_string(), "3.2222");

        let pattern = Pattern::parse("-10").unwrap();
        assert_eq!(pattern.to_string(), "-10");
    }

    #[test]
    fn test_number_range() {
        let pattern = Pattern::parse("1...10").expect("Failed to parse 1...10");
        assert_eq!(pattern.to_string(), "1...10");
        let pattern =
            Pattern::parse("-5.5...5.5").expect("Failed to parse -5.5...5.5");
        assert_eq!(pattern.to_string(), "-5.5...5.5");
    }

    #[test]
    fn test_number_comparisons() {
        let pattern = Pattern::parse(">5").expect("Failed to parse >5");
        assert_eq!(pattern.to_string(), ">5");

        let pattern = Pattern::parse(">=10").expect("Failed to parse >=10");
        assert_eq!(pattern.to_string(), ">=10");

        let pattern = Pattern::parse("<100").expect("Failed to parse <100");
        assert_eq!(pattern.to_string(), "<100");

        let pattern = Pattern::parse("<=50").expect("Failed to parse <=50");
        assert_eq!(pattern.to_string(), "<=50");
    }

    #[test]
    fn test_scientific_notation() {
        let pattern = Pattern::parse("1e10").expect("Failed to parse 1e10");
        assert_eq!(pattern.to_string(), "10000000000");
        let pattern =
            Pattern::parse("3.2222e-2").expect("Failed to parse 3.2222e-2");
        assert_eq!(pattern.to_string(), "0.032222");
    }

    #[test]
    fn test_number_infinity_parsing() {
        let pattern =
            Pattern::parse("NaN").expect("NaN should parse successfully");
        assert_eq!(pattern.to_string(), "NaN");
        let pattern = Pattern::parse("Infinity")
            .expect("Infinity should parse successfully");
        assert_eq!(pattern.to_string(), "Infinity");
        let pattern = Pattern::parse("-Infinity")
            .expect("-Infinity should parse successfully");
        assert_eq!(pattern.to_string(), "-Infinity");
    }
}

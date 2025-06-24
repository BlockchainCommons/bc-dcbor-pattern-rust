use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_number(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            parse_number_inner(lexer)
        }
        _ => Ok(Pattern::any_number()),
    }
}

fn parse_number_inner(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // Check for NaN first
    if let Some(Ok(Token::NaN)) = lexer.clone().next() {
        lexer.next(); // consume the NaN token
        match lexer.next() {
            Some(Ok(Token::ParenClose)) => return Ok(Pattern::number_nan()),
            Some(Ok(t)) => {
                return Err(Error::UnexpectedToken(Box::new(t), lexer.span()));
            }
            Some(Err(e)) => return Err(e),
            None => return Err(Error::ExpectedCloseParen(lexer.span())),
        }
    }

    // Check for comparison operators
    let comparison = match lexer.clone().next() {
        Some(Ok(Token::GreaterThanOrEqual)) => {
            lexer.next();
            Some("ge")
        }
        Some(Ok(Token::LessThanOrEqual)) => {
            lexer.next();
            Some("le")
        }
        Some(Ok(Token::GreaterThan)) | Some(Ok(Token::Sequence)) => {
            lexer.next();
            Some("gt")
        }
        Some(Ok(Token::LessThan)) => {
            lexer.next();
            Some("lt")
        }
        _ => None,
    };

    if let Some(op) = comparison {
        let value = parse_float_number(lexer)?;
        match lexer.next() {
            Some(Ok(Token::ParenClose)) => {
                let pattern = match op {
                    "ge" => Pattern::number_greater_than_or_equal(value),
                    "le" => Pattern::number_less_than_or_equal(value),
                    "gt" => Pattern::number_greater_than(value),
                    "lt" => Pattern::number_less_than(value),
                    _ => unreachable!(),
                };
                return Ok(pattern);
            }
            Some(Ok(t)) => {
                return Err(Error::UnexpectedToken(Box::new(t), lexer.span()));
            }
            Some(Err(e)) => return Err(e),
            None => return Err(Error::ExpectedCloseParen(lexer.span())),
        }
    }

    // Parse first number
    let first = parse_float_number(lexer)?;

    // Check for range operator
    match lexer.clone().next() {
        Some(Ok(Token::Ellipsis)) => {
            lexer.next(); // consume ellipsis
            let second = parse_float_number(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    Ok(Pattern::number_range(first..=second))
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        Some(Ok(Token::ParenClose)) => {
            lexer.next(); // consume closing paren
            Ok(Pattern::number(first))
        }
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::ExpectedCloseParen(lexer.span())),
    }
}

fn parse_float_number(lexer: &mut logos::Lexer<Token>) -> Result<f64> {
    match lexer.next() {
        Some(Ok(Token::NumberLiteral(Ok(value)))) => Ok(value),
        Some(Ok(Token::NumberLiteral(Err(e)))) => Err(e),
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    fn test_parse(input: &str) -> Result<Pattern> {
        let mut lexer = Token::lexer(input);
        match lexer.next() {
            Some(Ok(Token::Number)) => parse_number(&mut lexer),
            Some(Ok(token)) => {
                Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
            }
            Some(Err(e)) => Err(e),
            None => Err(Error::EmptyInput),
        }
    }

    #[test]
    fn test_number_any() {
        let pattern = test_parse("NUMBER").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER");
    }

    #[test]
    fn test_number_exact() {
        let pattern = test_parse("NUMBER(42)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(42)");

        let pattern = test_parse("NUMBER(3.14)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(3.14)");

        let pattern = test_parse("NUMBER(-10)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(-10)");
    }

    #[test]
    fn test_number_range() {
        match test_parse("NUMBER(1...10)") {
            Ok(pattern) => {
                assert_eq!(pattern.to_string(), "NUMBER(1...10)");
            }
            Err(e) => {
                panic!("Failed to parse NUMBER(1...10): {:?}", e);
            }
        }

        // Debug negative range parsing
        match test_parse("NUMBER(-5.5...5.5)") {
            Ok(pattern) => {
                assert_eq!(pattern.to_string(), "NUMBER(-5.5...5.5)");
            }
            Err(e) => {
                panic!("Failed to parse NUMBER(-5.5...5.5): {:?}", e);
            }
        }
    }

    #[test]
    fn test_number_comparisons() {
        let pattern = test_parse("NUMBER(>5)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(>5)");

        let pattern = test_parse("NUMBER(>=10)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(>=10)");

        let pattern = test_parse("NUMBER(<100)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(<100)");

        let pattern = test_parse("NUMBER(<=50)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(<=50)");
    }

    #[test]
    fn test_number_nan() {
        let pattern = test_parse("NUMBER(NaN)").unwrap();
        assert_eq!(pattern.to_string(), "NUMBER(NaN)");
    }

    #[test]
    fn test_scientific_notation() {
        match test_parse("NUMBER(1e10)") {
            Ok(pattern) => {
                assert_eq!(pattern.to_string(), "NUMBER(10000000000)")
            }
            Err(e) => panic!("Failed to parse NUMBER(1e10): {:?}", e),
        }

        match test_parse("NUMBER(3.14e-2)") {
            Ok(pattern) => assert_eq!(pattern.to_string(), "NUMBER(0.0314)"),
            Err(e) => panic!("Failed to parse NUMBER(3.14e-2): {:?}", e),
        }
    }
}

use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_number(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // For the keyword "number", just return any number pattern
    Ok(Pattern::any_number())
}

fn parse_number_inner(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // Check for special values first
    match lexer.clone().next() {
        Some(Ok(Token::NaN)) => {
            lexer.next(); // consume the NaN token
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => return Ok(Pattern::number_nan()),
                Some(Ok(t)) => {
                    return Err(Error::UnexpectedToken(
                        Box::new(t),
                        lexer.span(),
                    ));
                }
                Some(Err(e)) => return Err(e),
                None => return Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        Some(Ok(Token::Infinity)) => {
            lexer.next(); // consume the Infinity token
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    return Ok(Pattern::number_infinity());
                }
                Some(Ok(t)) => {
                    return Err(Error::UnexpectedToken(
                        Box::new(t),
                        lexer.span(),
                    ));
                }
                Some(Err(e)) => return Err(e),
                None => return Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        Some(Ok(Token::NegInfinity)) => {
            lexer.next(); // consume the -Infinity token
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    return Ok(Pattern::number_neg_infinity());
                }
                Some(Ok(t)) => {
                    return Err(Error::UnexpectedToken(
                        Box::new(t),
                        lexer.span(),
                    ));
                }
                Some(Err(e)) => return Err(e),
                None => return Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => {} // Continue with normal parsing
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
        Some(Ok(Token::GreaterThan)) => {
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
        let pattern = Pattern::parse("-5.5...5.5").expect("Failed to parse -5.5...5.5");
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
        let pattern = Pattern::parse("3.2222e-2").expect("Failed to parse 3.2222e-2");
        assert_eq!(pattern.to_string(), "0.032222");
    }

    #[test]
    fn test_number_infinity_parsing() {
        let pattern = Pattern::parse("NaN").expect("NaN should parse successfully");
        assert_eq!(pattern.to_string(), "NaN");
        let pattern = Pattern::parse("Infinity").expect("Infinity should parse successfully");
        assert_eq!(pattern.to_string(), "Infinity");
        let pattern = Pattern::parse("-Infinity").expect("-Infinity should parse successfully");
        assert_eq!(pattern.to_string(), "-Infinity");
    }
}

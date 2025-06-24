//! Parser for repeat patterns (quantifiers).
//!
//! This module handles parsing of quantifier syntax like *, +, ?, {n,m}
//! that can follow grouped patterns in parentheses.

use super::super::Token;
use crate::{Error, Pattern, Quantifier, Reluctance, Result};

/// Parse quantifier tokens that follow a grouped pattern.
///
/// This function assumes that a pattern has been parsed and we're now
/// looking for quantifier operators like *, +, ?, or {n,m}.
///
/// # Arguments
/// * `pattern` - The pattern to apply the quantifier to
/// * `lexer` - The lexer positioned after the pattern
///
/// # Returns
/// * `Ok(Pattern)` - The pattern wrapped with the appropriate quantifier
/// * `Err(Error)` - If quantifier parsing fails
pub(crate) fn parse_quantifier(
    pattern: Pattern,
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // Look ahead to see if there's a quantifier
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(tok)) => match tok {
            Token::RepeatZeroOrMore => {
                lexer.next(); // consume the token
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(0.., Reluctance::Greedy),
                ))
            }
            Token::RepeatZeroOrMoreLazy => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(0.., Reluctance::Lazy),
                ))
            }
            Token::RepeatZeroOrMorePossessive => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(0.., Reluctance::Possessive),
                ))
            }
            Token::RepeatOneOrMore => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(1.., Reluctance::Greedy),
                ))
            }
            Token::RepeatOneOrMoreLazy => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(1.., Reluctance::Lazy),
                ))
            }
            Token::RepeatOneOrMorePossessive => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(1.., Reluctance::Possessive),
                ))
            }
            Token::RepeatZeroOrOne => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(0..=1, Reluctance::Greedy),
                ))
            }
            Token::RepeatZeroOrOneLazy => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(0..=1, Reluctance::Lazy),
                ))
            }
            Token::RepeatZeroOrOnePossessive => {
                lexer.next();
                Ok(Pattern::repeat(
                    pattern,
                    Quantifier::new(0..=1, Reluctance::Possessive),
                ))
            }
            Token::Range(res) => {
                lexer.next(); // consume the token
                let q = res?;
                let pat = if let Some(max) = q.max() {
                    Pattern::repeat(
                        pattern,
                        Quantifier::new(q.min()..=max, q.reluctance()),
                    )
                } else {
                    Pattern::repeat(
                        pattern,
                        Quantifier::new(q.min().., q.reluctance()),
                    )
                };
                Ok(pat)
            }
            _ => {
                // No quantifier found, return pattern as-is
                Ok(pattern)
            }
        },
        _ => {
            // No quantifier found, return pattern as-is
            Ok(pattern)
        }
    }
}

/// Parse a grouped pattern with optional quantifier.
///
/// This function would be used when parsing "(pattern)" followed by
/// quantifiers. Currently a placeholder until the full parsing infrastructure
/// is implemented.
pub(crate) fn parse_group(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // TODO: Implement when OR parser and primary pattern parsing is ready
    // This would:
    // 1. Parse the inner pattern using parse_or() or similar
    // 2. Expect a closing parenthesis
    // 3. Parse any quantifiers using parse_quantifier()
    todo!(
        "Group parsing not yet implemented - waiting for full parse infrastructure"
    )
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_quantifier_star() {
        let mut lexer = Token::lexer("*");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer).unwrap();

        // Should be a repeat pattern with 0.. quantifier
        assert_eq!(result.to_string(), "(NUMBER(42))*");
    }

    #[test]
    fn test_parse_quantifier_plus() {
        let mut lexer = Token::lexer("+");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer).unwrap();

        assert_eq!(result.to_string(), "(NUMBER(42))+");
    }

    #[test]
    fn test_parse_quantifier_question() {
        let mut lexer = Token::lexer("?");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer).unwrap();

        assert_eq!(result.to_string(), "(NUMBER(42))?");
    }

    #[test]
    fn test_parse_quantifier_lazy() {
        let mut lexer = Token::lexer("*?");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer).unwrap();

        assert_eq!(result.to_string(), "(NUMBER(42))*?");
    }

    #[test]
    fn test_parse_quantifier_possessive() {
        let mut lexer = Token::lexer("++");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer).unwrap();

        assert_eq!(result.to_string(), "(NUMBER(42))++");
    }

    #[test]
    fn test_parse_quantifier_range() {
        let mut lexer = Token::lexer("{3,5}");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer).unwrap();

        assert_eq!(result.to_string(), "(NUMBER(42)){3,5}");
    }

    #[test]
    fn test_parse_quantifier_no_quantifier() {
        let mut lexer = Token::lexer("OTHER");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer).unwrap();

        // Should return the pattern unchanged
        assert_eq!(result.to_string(), "NUMBER(42)");
    }
}

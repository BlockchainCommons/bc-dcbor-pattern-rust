//! Parser for repeat patterns (quantifiers).
//!
//! This module handles parsing of quantifier syntax like *, +, ?, {n,m}
//! that can follow grouped patterns in parentheses.

use super::super::Token;
use crate::{Pattern, Quantifier, Reluctance, Result};

/// Parse quantifier tokens that follow a grouped pattern.
///
/// This function assumes that a pattern has been parsed and we're now
/// looking for quantifier operators like *, +, ?, or {n,m}.
///
/// # Arguments
/// * `pattern` - The pattern to apply the quantifier to
/// * `lexer` - The lexer positioned after the pattern
/// * `force_repeat` - If true, always wrap in RepeatPattern even without
///   explicit quantifier
///
/// # Returns
/// * `Ok(Pattern)` - The pattern wrapped with the appropriate quantifier
/// * `Err(Error)` - If quantifier parsing fails
pub(crate) fn parse_quantifier(
    pattern: Pattern,
    lexer: &mut logos::Lexer<Token>,
    force_repeat: bool,
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
                // No quantifier found - behavior depends on force_repeat flag
                if force_repeat {
                    // Parentheses always create a RepeatPattern with "exactly
                    // one"
                    Ok(Pattern::repeat(pattern, Quantifier::default()))
                } else {
                    // Return pattern unchanged for general use
                    Ok(pattern)
                }
            }
        },
        _ => {
            // No quantifier found - behavior depends on force_repeat flag
            if force_repeat {
                // Parentheses always create a RepeatPattern with "exactly one"
                Ok(Pattern::repeat(pattern, Quantifier::default()))
            } else {
                // Return pattern unchanged for general use
                Ok(pattern)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_quantifier_star() {
        let mut lexer = Token::lexer("*");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, false).unwrap();

        // Should be a repeat pattern with 0.. quantifier
        assert_eq!(result.to_string(), "(42)*");
    }

    #[test]
    fn test_parse_quantifier_plus() {
        let mut lexer = Token::lexer("+");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, false).unwrap();

        assert_eq!(result.to_string(), "(42)+");
    }

    #[test]
    fn test_parse_quantifier_question() {
        let mut lexer = Token::lexer("?");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, false).unwrap();

        assert_eq!(result.to_string(), "(42)?");
    }

    #[test]
    fn test_parse_quantifier_lazy() {
        let mut lexer = Token::lexer("*?");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, false).unwrap();

        assert_eq!(result.to_string(), "(42)*?");
    }

    #[test]
    fn test_parse_quantifier_possessive() {
        let mut lexer = Token::lexer("++");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, false).unwrap();

        assert_eq!(result.to_string(), "(42)++");
    }

    #[test]
    fn test_parse_quantifier_range() {
        let mut lexer = Token::lexer("{3,5}");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, false).unwrap();

        assert_eq!(result.to_string(), "(42){3,5}");
    }

    #[test]
    fn test_parse_quantifier_no_quantifier() {
        let mut lexer = Token::lexer("OTHER");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, false).unwrap();

        // Should return the pattern unchanged
        assert_eq!(result.to_string(), "42");
    }

    #[test]
    fn test_parse_quantifier_force_repeat() {
        let mut lexer = Token::lexer("OTHER");
        let pattern = Pattern::number(42);
        let result = parse_quantifier(pattern, &mut lexer, true).unwrap();

        // Should always create a RepeatPattern when force_repeat is true
        assert_eq!(result.to_string(), "(42){1}");
    }
}

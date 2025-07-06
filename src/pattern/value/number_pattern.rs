use std::ops::RangeInclusive;

use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching number values in dCBOR.
#[derive(Debug, Clone)]
pub enum NumberPattern {
    /// Matches any number.
    Any,
    /// Matches the exact number.
    Value(f64),
    /// Matches numbers within a range, inclusive (..=).
    Range(RangeInclusive<f64>),
    /// Matches numbers that are greater than the specified value.
    GreaterThan(f64),
    /// Matches numbers that are greater than or equal to the specified value.
    GreaterThanOrEqual(f64),
    /// Matches numbers that are less than the specified value.
    LessThan(f64),
    /// Matches numbers that are less than or equal to the specified value.
    LessThanOrEqual(f64),
    /// Matches numbers that are NaN (Not a Number).
    NaN,
    /// Matches positive infinity.
    Infinity,
    /// Matches negative infinity.
    NegInfinity,
}

impl std::hash::Hash for NumberPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            NumberPattern::Any => 0u8.hash(state),
            NumberPattern::Value(value) => {
                1u8.hash(state);
                value.to_bits().hash(state);
            }
            NumberPattern::Range(range) => {
                2u8.hash(state);
                range.start().to_bits().hash(state);
                range.end().to_bits().hash(state);
            }
            NumberPattern::GreaterThan(value) => {
                3u8.hash(state);
                value.to_bits().hash(state);
            }
            NumberPattern::GreaterThanOrEqual(value) => {
                4u8.hash(state);
                value.to_bits().hash(state);
            }
            NumberPattern::LessThan(value) => {
                5u8.hash(state);
                value.to_bits().hash(state);
            }
            NumberPattern::LessThanOrEqual(value) => {
                6u8.hash(state);
                value.to_bits().hash(state);
            }
            NumberPattern::NaN => 7u8.hash(state),
            NumberPattern::Infinity => 8u8.hash(state),
            NumberPattern::NegInfinity => 9u8.hash(state),
        }
    }
}

impl PartialEq for NumberPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumberPattern::Any, NumberPattern::Any) => true,
            (NumberPattern::Value(a), NumberPattern::Value(b)) => a == b,
            (NumberPattern::Range(a), NumberPattern::Range(b)) => a == b,
            (NumberPattern::GreaterThan(a), NumberPattern::GreaterThan(b)) => {
                a == b
            }
            (
                NumberPattern::GreaterThanOrEqual(a),
                NumberPattern::GreaterThanOrEqual(b),
            ) => a == b,
            (NumberPattern::LessThan(a), NumberPattern::LessThan(b)) => a == b,
            (
                NumberPattern::LessThanOrEqual(a),
                NumberPattern::LessThanOrEqual(b),
            ) => a == b,
            (NumberPattern::NaN, NumberPattern::NaN) => true,
            (NumberPattern::Infinity, NumberPattern::Infinity) => true,
            (NumberPattern::NegInfinity, NumberPattern::NegInfinity) => true,
            _ => false,
        }
    }
}

impl Eq for NumberPattern {}

impl NumberPattern {
    /// Creates a new `NumberPattern` that matches any number.
    pub fn any() -> Self { NumberPattern::Any }

    /// Creates a new `NumberPattern` that matches the exact number.
    pub fn value<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        NumberPattern::Value(value.into())
    }

    /// Creates a new `NumberPattern` that matches numbers within the specified
    /// range.
    pub fn range<A>(range: RangeInclusive<A>) -> Self
    where
        A: Into<f64> + Copy,
    {
        let start = (*range.start()).into();
        let end = (*range.end()).into();
        NumberPattern::Range(RangeInclusive::new(start, end))
    }

    /// Creates a new `NumberPattern` that matches numbers greater than the
    /// specified value.
    pub fn greater_than<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        NumberPattern::GreaterThan(value.into())
    }

    /// Creates a new `NumberPattern` that matches numbers greater than or
    /// equal to the specified value.
    pub fn greater_than_or_equal<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        NumberPattern::GreaterThanOrEqual(value.into())
    }

    /// Creates a new `NumberPattern` that matches numbers less than the
    /// specified value.
    pub fn less_than<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        NumberPattern::LessThan(value.into())
    }

    /// Creates a new `NumberPattern` that matches numbers less than or equal
    /// to the specified value.
    pub fn less_than_or_equal<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        NumberPattern::LessThanOrEqual(value.into())
    }

    /// Creates a new `NumberPattern` that matches NaN values.
    pub fn nan() -> Self { NumberPattern::NaN }

    /// Creates a new `NumberPattern` that matches positive infinity.
    pub fn infinity() -> Self { NumberPattern::Infinity }

    /// Creates a new `NumberPattern` that matches negative infinity.
    pub fn neg_infinity() -> Self { NumberPattern::NegInfinity }
}

impl Matcher for NumberPattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        let is_hit = match self {
            NumberPattern::Any => haystack.is_number(),
            NumberPattern::Value(want) => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value == *want
                } else {
                    false
                }
            }
            NumberPattern::Range(want) => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    want.contains(&value)
                } else {
                    false
                }
            }
            NumberPattern::GreaterThan(want) => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value > *want
                } else {
                    false
                }
            }
            NumberPattern::GreaterThanOrEqual(want) => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value >= *want
                } else {
                    false
                }
            }
            NumberPattern::LessThan(want) => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value < *want
                } else {
                    false
                }
            }
            NumberPattern::LessThanOrEqual(want) => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value <= *want
                } else {
                    false
                }
            }
            NumberPattern::NaN => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value.is_nan()
                } else {
                    false
                }
            }
            NumberPattern::Infinity => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value.is_infinite() && value.is_sign_positive()
                } else {
                    false
                }
            }
            NumberPattern::NegInfinity => {
                if let Ok(value) = f64::try_from_cbor(haystack) {
                    value.is_infinite() && value.is_sign_negative()
                } else {
                    false
                }
            }
        };

        if is_hit {
            vec![vec![haystack.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Value(crate::pattern::ValuePattern::Number(
            self.clone(),
        )));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for NumberPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberPattern::Any => write!(f, "number"),
            NumberPattern::Value(value) => write!(f, "{}", value),
            NumberPattern::Range(range) => {
                write!(f, "{}...{}", range.start(), range.end())
            }
            NumberPattern::GreaterThan(value) => {
                write!(f, ">{}", value)
            }
            NumberPattern::GreaterThanOrEqual(value) => {
                write!(f, ">={}", value)
            }
            NumberPattern::LessThan(value) => write!(f, "<{}", value),
            NumberPattern::LessThanOrEqual(value) => {
                write!(f, "<={}", value)
            }
            NumberPattern::NaN => write!(f, "NaN"),
            NumberPattern::Infinity => write!(f, "Infinity"),
            NumberPattern::NegInfinity => write!(f, "-Infinity"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_pattern_display() {
        assert_eq!(NumberPattern::any().to_string(), "number");
        assert_eq!(NumberPattern::value(42.0).to_string(), "42");
        assert_eq!(NumberPattern::range(1.0..=10.0).to_string(), "1...10");
        assert_eq!(NumberPattern::greater_than(5.0).to_string(), ">5");
        assert_eq!(
            NumberPattern::greater_than_or_equal(5.0).to_string(),
            ">=5"
        );
        assert_eq!(NumberPattern::less_than(5.0).to_string(), "<5");
        assert_eq!(NumberPattern::less_than_or_equal(5.0).to_string(), "<=5");
        assert_eq!(NumberPattern::nan().to_string(), "NaN");
        assert_eq!(NumberPattern::infinity().to_string(), "Infinity");
        assert_eq!(NumberPattern::neg_infinity().to_string(), "-Infinity");
    }

    #[test]
    fn test_number_pattern_matching() {
        let int_cbor = 42.to_cbor();
        let float_cbor = 3.2222.to_cbor();
        let negative_cbor = (-10).to_cbor();
        let nan_cbor = f64::NAN.to_cbor();
        let text_cbor = "not a number".to_cbor();

        // Test Any pattern
        let any_pattern = NumberPattern::any();
        assert!(any_pattern.matches(&int_cbor));
        assert!(any_pattern.matches(&float_cbor));
        assert!(any_pattern.matches(&negative_cbor));
        assert!(any_pattern.matches(&nan_cbor));
        assert!(!any_pattern.matches(&text_cbor));

        // Test exact patterns
        let exact_pattern = NumberPattern::value(42.0);
        assert!(exact_pattern.matches(&int_cbor));
        assert!(!exact_pattern.matches(&float_cbor));
        assert!(!exact_pattern.matches(&text_cbor));

        // Test range pattern
        let range_pattern = NumberPattern::range(1.0..=50.0);
        assert!(range_pattern.matches(&int_cbor));
        assert!(range_pattern.matches(&float_cbor));
        assert!(!range_pattern.matches(&negative_cbor));
        assert!(!range_pattern.matches(&text_cbor));

        // Test comparison patterns
        let gt_pattern = NumberPattern::greater_than(10.0);
        assert!(gt_pattern.matches(&int_cbor));
        assert!(!gt_pattern.matches(&float_cbor));
        assert!(!gt_pattern.matches(&negative_cbor));

        let lt_pattern = NumberPattern::less_than(50.0);
        assert!(lt_pattern.matches(&int_cbor));
        assert!(lt_pattern.matches(&float_cbor));
        assert!(lt_pattern.matches(&negative_cbor));

        // Test NaN pattern
        let nan_pattern = NumberPattern::nan();
        assert!(!nan_pattern.matches(&int_cbor));
        assert!(!nan_pattern.matches(&float_cbor));
        assert!(nan_pattern.matches(&nan_cbor));
        assert!(!nan_pattern.matches(&text_cbor));
    }

    #[test]
    fn test_number_pattern_paths() {
        let int_cbor = 42.to_cbor();
        let text_cbor = "not a number".to_cbor();

        let any_pattern = NumberPattern::any();
        let int_paths = any_pattern.paths(&int_cbor);
        assert_eq!(int_paths.len(), 1);
        assert_eq!(int_paths[0].len(), 1);
        assert_eq!(int_paths[0][0], int_cbor);

        let text_paths = any_pattern.paths(&text_cbor);
        assert_eq!(text_paths.len(), 0);

        let exact_pattern = NumberPattern::value(42.0);
        let paths = exact_pattern.paths(&int_cbor);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 1);
        assert_eq!(paths[0][0], int_cbor);

        let no_match_paths = exact_pattern.paths(&text_cbor);
        assert_eq!(no_match_paths.len(), 0);
    }

    #[test]
    fn test_number_conversion() {
        let int_cbor = 42.to_cbor();
        let float_cbor = 3.2222.to_cbor();
        let negative_cbor = (-10).to_cbor();
        let text_cbor = "not a number".to_cbor();

        // Test direct conversion using try_from_cbor
        assert_eq!(f64::try_from_cbor(&int_cbor).ok(), Some(42.0));
        assert_eq!(f64::try_from_cbor(&float_cbor).ok(), Some(3.2222));
        assert_eq!(f64::try_from_cbor(&negative_cbor).ok(), Some(-10.0));
        assert_eq!(f64::try_from_cbor(&text_cbor).ok(), None);
    }

    #[test]
    fn test_infinity_patterns() {
        let inf_cbor = f64::INFINITY.to_cbor();
        let neg_inf_cbor = f64::NEG_INFINITY.to_cbor();
        let nan_cbor = f64::NAN.to_cbor();
        let regular_cbor = 42.0.to_cbor();
        let text_cbor = "not a number".to_cbor();

        // Test positive infinity pattern
        let inf_pattern = NumberPattern::infinity();
        assert!(inf_pattern.matches(&inf_cbor));
        assert!(!inf_pattern.matches(&neg_inf_cbor));
        assert!(!inf_pattern.matches(&nan_cbor));
        assert!(!inf_pattern.matches(&regular_cbor));
        assert!(!inf_pattern.matches(&text_cbor));

        // Test negative infinity pattern
        let neg_inf_pattern = NumberPattern::neg_infinity();
        assert!(!neg_inf_pattern.matches(&inf_cbor));
        assert!(neg_inf_pattern.matches(&neg_inf_cbor));
        assert!(!neg_inf_pattern.matches(&nan_cbor));
        assert!(!neg_inf_pattern.matches(&regular_cbor));
        assert!(!neg_inf_pattern.matches(&text_cbor));

        // Test that any pattern matches all number types including infinities
        let any_pattern = NumberPattern::any();
        assert!(any_pattern.matches(&inf_cbor));
        assert!(any_pattern.matches(&neg_inf_cbor));
        assert!(any_pattern.matches(&nan_cbor));
        assert!(any_pattern.matches(&regular_cbor));
        assert!(!any_pattern.matches(&text_cbor));

        // Test display formatting
        assert_eq!(inf_pattern.to_string(), "Infinity");
        assert_eq!(neg_inf_pattern.to_string(), "-Infinity");
    }
}

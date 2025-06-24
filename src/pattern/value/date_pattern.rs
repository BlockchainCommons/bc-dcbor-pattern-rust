use std::ops::RangeInclusive;

use dcbor::{Date, prelude::*};

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching date values in dCBOR.
#[derive(Debug, Clone)]
pub enum DatePattern {
    /// Matches any date.
    Any,
    /// Matches a specific date.
    Value(Date),
    /// Matches dates within a range (inclusive).
    Range(RangeInclusive<Date>),
    /// Matches dates that are on or after the specified date.
    Earliest(Date),
    /// Matches dates that are on or before the specified date.
    Latest(Date),
    /// Matches a date by its ISO-8601 string representation.
    Iso8601(String),
    /// Matches dates whose ISO-8601 string representation matches the given
    /// regex pattern.
    Regex(regex::Regex),
}

impl PartialEq for DatePattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DatePattern::Any, DatePattern::Any) => true,
            (DatePattern::Value(a), DatePattern::Value(b)) => a == b,
            (DatePattern::Range(a), DatePattern::Range(b)) => a == b,
            (DatePattern::Earliest(a), DatePattern::Earliest(b)) => a == b,
            (DatePattern::Latest(a), DatePattern::Latest(b)) => a == b,
            (DatePattern::Iso8601(a), DatePattern::Iso8601(b)) => a == b,
            (DatePattern::Regex(a), DatePattern::Regex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for DatePattern {}

impl std::hash::Hash for DatePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DatePattern::Any => {
                0u8.hash(state);
            }
            DatePattern::Value(date) => {
                1u8.hash(state);
                date.hash(state);
            }
            DatePattern::Range(range) => {
                2u8.hash(state);
                range.start().hash(state);
                range.end().hash(state);
            }
            DatePattern::Earliest(date) => {
                3u8.hash(state);
                date.hash(state);
            }
            DatePattern::Latest(date) => {
                4u8.hash(state);
                date.hash(state);
            }
            DatePattern::Iso8601(iso_string) => {
                5u8.hash(state);
                iso_string.hash(state);
            }
            DatePattern::Regex(regex) => {
                6u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl DatePattern {
    /// Creates a new `DatePattern` that matches any date.
    pub fn any() -> Self { DatePattern::Any }

    /// Creates a new `DatePattern` that matches a specific date.
    pub fn value(date: Date) -> Self { DatePattern::Value(date) }

    /// Creates a new `DatePattern` that matches dates within a range
    /// (inclusive).
    pub fn range(range: RangeInclusive<Date>) -> Self {
        DatePattern::Range(range)
    }

    /// Creates a new `DatePattern` that matches dates that are on or after the
    /// specified date.
    pub fn earliest(date: Date) -> Self { DatePattern::Earliest(date) }

    /// Creates a new `DatePattern` that matches dates that are on or before the
    /// specified date.
    pub fn latest(date: Date) -> Self { DatePattern::Latest(date) }

    /// Creates a new `DatePattern` that matches a date by its ISO-8601 string
    /// representation.
    pub fn iso8601(iso_string: impl Into<String>) -> Self {
        DatePattern::Iso8601(iso_string.into())
    }

    /// Creates a new `DatePattern` that matches dates whose ISO-8601 string
    /// representation matches the given regex pattern.
    pub fn regex(regex: regex::Regex) -> Self { DatePattern::Regex(regex) }
}

impl Matcher for DatePattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // Check if the CBOR is a tagged value with date tag (tag 1)
        if let CBORCase::Tagged(tag, _) = cbor.as_case() {
            // Check if this is a date tag (tag 1)
            if tag.value() == 1 {
                // Try to extract the date
                if let Ok(date) = Date::try_from(cbor.clone()) {
                    let is_hit = match self {
                        DatePattern::Any => true,
                        DatePattern::Value(expected_date) => {
                            date == *expected_date
                        }
                        DatePattern::Range(range) => range.contains(&date),
                        DatePattern::Earliest(earliest) => date >= *earliest,
                        DatePattern::Latest(latest) => date <= *latest,
                        DatePattern::Iso8601(expected_string) => {
                            date.to_string() == *expected_string
                        }
                        DatePattern::Regex(regex) => {
                            regex.is_match(&date.to_string())
                        }
                    };

                    if is_hit {
                        vec![vec![cbor.clone()]]
                    } else {
                        vec![]
                    }
                } else {
                    // Tagged with date tag but couldn't be parsed as date
                    vec![]
                }
            } else {
                // Not a date tag
                vec![]
            }
        } else {
            // Not tagged
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
        literals.push(Pattern::Value(crate::pattern::ValuePattern::Date(self.clone())));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for DatePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatePattern::Any => write!(f, "DATE"),
            DatePattern::Value(date) => write!(f, "DATE({})", date),
            DatePattern::Range(range) => {
                write!(f, "DATE({}...{})", range.start(), range.end())
            }
            DatePattern::Earliest(date) => write!(f, "DATE({}...)", date),
            DatePattern::Latest(date) => write!(f, "DATE(...{})", date),
            DatePattern::Iso8601(iso_string) => {
                write!(f, "DATE({})", iso_string)
            }
            DatePattern::Regex(regex) => {
                write!(f, "DATE(/{}/)", regex.as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_pattern_any() {
        // Create a date CBOR value
        let date = Date::from_ymd(2023, 12, 25);
        let cbor = CBOR::from(date);

        let pattern = DatePattern::any();
        let paths = pattern.paths(&cbor);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![cbor.clone()]);

        // Test with non-date CBOR
        let text_cbor = CBOR::from("test");
        let paths = pattern.paths(&text_cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_value() {
        let date = Date::from_ymd(2023, 12, 25);
        let cbor = CBOR::from(date.clone());

        // Test matching date
        let pattern = DatePattern::value(date.clone());
        let paths = pattern.paths(&cbor);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![cbor.clone()]);

        // Test non-matching date
        let other_date = Date::from_ymd(2024, 1, 1);
        let pattern = DatePattern::value(other_date);
        let paths = pattern.paths(&cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_range() {
        let date1 = Date::from_ymd(2023, 12, 20);
        let date2 = Date::from_ymd(2023, 12, 25);
        let date3 = Date::from_ymd(2023, 12, 30);

        let test_date = date2.clone();
        let cbor = CBOR::from(test_date);

        // Test date within range
        let pattern = DatePattern::range(date1.clone()..=date3.clone());
        let paths = pattern.paths(&cbor);
        assert_eq!(paths.len(), 1);

        // Test date outside range
        let early_date = Date::from_ymd(2023, 12, 10);
        let _late_date = Date::from_ymd(2024, 1, 10);
        let pattern = DatePattern::range(early_date..=date1);
        let paths = pattern.paths(&cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_earliest() {
        let early_date = Date::from_ymd(2023, 12, 20);
        let test_date = Date::from_ymd(2023, 12, 25);
        let cbor = CBOR::from(test_date);

        // Test date after earliest
        let pattern = DatePattern::earliest(early_date);
        let paths = pattern.paths(&cbor);
        assert_eq!(paths.len(), 1);

        // Test date before earliest
        let late_date = Date::from_ymd(2023, 12, 30);
        let pattern = DatePattern::earliest(late_date);
        let paths = pattern.paths(&cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_latest() {
        let late_date = Date::from_ymd(2023, 12, 30);
        let test_date = Date::from_ymd(2023, 12, 25);
        let cbor = CBOR::from(test_date);

        // Test date before latest
        let pattern = DatePattern::latest(late_date);
        let paths = pattern.paths(&cbor);
        assert_eq!(paths.len(), 1);

        // Test date after latest
        let early_date = Date::from_ymd(2023, 12, 20);
        let pattern = DatePattern::latest(early_date);
        let paths = pattern.paths(&cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_iso8601() {
        let date = Date::from_ymd(2023, 12, 25);
        let cbor = CBOR::from(date.clone());
        let iso_string = date.to_string();

        // Test matching ISO string
        let pattern = DatePattern::iso8601(iso_string.clone());
        let paths = pattern.paths(&cbor);
        assert_eq!(paths.len(), 1);

        // Test non-matching ISO string
        let pattern = DatePattern::iso8601("2024-01-01T00:00:00Z");
        let paths = pattern.paths(&cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_regex() {
        let date = Date::from_ymd(2023, 12, 25);
        let cbor = CBOR::from(date);

        // Test matching regex (year 2023)
        let regex = regex::Regex::new(r"2023-.*").unwrap();
        let pattern = DatePattern::regex(regex);
        let paths = pattern.paths(&cbor);
        assert_eq!(paths.len(), 1);

        // Test non-matching regex (year 2024)
        let regex = regex::Regex::new(r"2024-.*").unwrap();
        let pattern = DatePattern::regex(regex);
        let paths = pattern.paths(&cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_display() {
        assert_eq!(DatePattern::any().to_string(), "DATE");

        let date = Date::from_ymd(2023, 12, 25);
        assert_eq!(
            DatePattern::value(date.clone()).to_string(),
            format!("DATE({})", date)
        );

        let date1 = Date::from_ymd(2023, 12, 20);
        let date2 = Date::from_ymd(2023, 12, 30);
        assert_eq!(
            DatePattern::range(date1.clone()..=date2.clone()).to_string(),
            format!("DATE({}...{})", date1, date2)
        );

        assert_eq!(
            DatePattern::earliest(date.clone()).to_string(),
            format!("DATE({}...)", date)
        );
        assert_eq!(
            DatePattern::latest(date.clone()).to_string(),
            format!("DATE(...{})", date)
        );

        assert_eq!(
            DatePattern::iso8601("2023-12-25T00:00:00Z").to_string(),
            "DATE(2023-12-25T00:00:00Z)"
        );

        let regex = regex::Regex::new(r"2023-.*").unwrap();
        assert_eq!(DatePattern::regex(regex).to_string(), "DATE(/2023-.*/)");
    }
}

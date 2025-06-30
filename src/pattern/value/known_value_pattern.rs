use dcbor::{CBOR, CBORCase, Tag};
use known_values::{KNOWN_VALUES, KnownValue};

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

// Known value tag is 40000 as defined in BCR-2020-006
const KNOWN_VALUE_TAG: Tag = Tag::with_value(40000);

/// Pattern for matching known values.
#[derive(Debug, Clone)]
pub enum KnownValuePattern {
    /// Matches any known value.
    Any,
    /// Matches the specific known value.
    Value(KnownValue),
    /// Matches the name of a known value.
    Named(String),
    /// Matches the regex for a known value name.
    Regex(regex::Regex),
}

impl PartialEq for KnownValuePattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KnownValuePattern::Any, KnownValuePattern::Any) => true,
            (KnownValuePattern::Value(a), KnownValuePattern::Value(b)) => {
                a == b
            }
            (KnownValuePattern::Named(a), KnownValuePattern::Named(b)) => {
                a == b
            }
            (KnownValuePattern::Regex(a), KnownValuePattern::Regex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for KnownValuePattern {}

impl std::hash::Hash for KnownValuePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            KnownValuePattern::Any => {
                0u8.hash(state);
            }
            KnownValuePattern::Value(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            KnownValuePattern::Named(name) => {
                2u8.hash(state);
                name.hash(state);
            }
            KnownValuePattern::Regex(regex) => {
                3u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl KnownValuePattern {
    /// Creates a new `KnownValuePattern` that matches any known value.
    pub fn any() -> Self { KnownValuePattern::Any }

    /// Creates a new `KnownValuePattern` that matches a specific known value.
    pub fn value(value: KnownValue) -> Self { KnownValuePattern::Value(value) }

    /// Creates a new `KnownValuePattern` that matches a known value by name.
    pub fn named(name: impl Into<String>) -> Self {
        KnownValuePattern::Named(name.into())
    }

    /// Creates a new `KnownValuePattern` that matches the regex for a known
    /// value name.
    pub fn regex(regex: regex::Regex) -> Self {
        KnownValuePattern::Regex(regex)
    }
}

impl Matcher for KnownValuePattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // Known values are represented as tagged values with tag 40000
        // (KNOWN_VALUE)
        if let CBORCase::Tagged(tag, content) = cbor.as_case() {
            if *tag == KNOWN_VALUE_TAG {
                if let CBORCase::Unsigned(value) = content.as_case() {
                    let known_value = KnownValue::new(*value);
                    match self {
                        KnownValuePattern::Any => vec![vec![cbor.clone()]],
                        KnownValuePattern::Value(expected) => {
                            if known_value == *expected {
                                vec![vec![cbor.clone()]]
                            } else {
                                vec![]
                            }
                        }
                        KnownValuePattern::Named(name) => {
                            // Look up the known value by name in the global
                            // registry
                            let binding = KNOWN_VALUES.get();
                            if let Some(known_values_store) = binding.as_ref() {
                                if let Some(expected_value) =
                                    known_values_store.known_value_named(name)
                                {
                                    if known_value == *expected_value {
                                        vec![vec![cbor.clone()]]
                                    } else {
                                        vec![]
                                    }
                                } else {
                                    // Name not found in registry, no match
                                    vec![]
                                }
                            } else {
                                // Registry not initialized, no match
                                vec![]
                            }
                        }
                        KnownValuePattern::Regex(regex) => {
                            // Check if the known value's name matches the regex
                            // Use the global registry to get the proper name
                            let name = {
                                let binding = KNOWN_VALUES.get();
                                if let Some(known_values_store) =
                                    binding.as_ref()
                                {
                                    known_values_store.name(known_value.clone())
                                } else {
                                    known_value.name()
                                }
                            };

                            if regex.is_match(&name) {
                                vec![vec![cbor.clone()]]
                            } else {
                                vec![]
                            }
                        }
                    }
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
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
        literals.push(Pattern::Value(
            crate::pattern::ValuePattern::KnownValue(self.clone()),
        ));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for KnownValuePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnownValuePattern::Any => write!(f, "known"),
            KnownValuePattern::Value(value) => {
                write!(f, "'{}'", value.name())
            }
            KnownValuePattern::Named(name) => write!(f, "'{}'", name),
            KnownValuePattern::Regex(regex) => {
                write!(f, "'/{}/'", regex.as_str())
            }
        }
    }
}

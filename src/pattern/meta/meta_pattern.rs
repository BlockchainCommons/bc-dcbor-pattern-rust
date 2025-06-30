use dcbor::prelude::*;

use super::{
    AndPattern, AnyPattern, CapturePattern, NotPattern, OrPattern,
    RepeatPattern, SearchPattern, SequencePattern,
};
use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for combining and modifying other patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaPattern {
    /// Always matches.
    Any(AnyPattern),
    /// Matches if all contained patterns match.
    And(AndPattern),
    /// Matches if any contained pattern matches.
    Or(OrPattern),
    /// Matches if the inner pattern does not match.
    Not(NotPattern),
    /// Matches with repetition.
    Repeat(RepeatPattern),
    /// Captures a pattern match.
    Capture(CapturePattern),
    /// Searches the entire dCBOR tree for matches.
    Search(SearchPattern),
    /// Matches a sequence of patterns in order.
    Sequence(SequencePattern),
}

impl Matcher for MetaPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        match self {
            MetaPattern::Any(pattern) => pattern.paths(cbor),
            MetaPattern::And(pattern) => pattern.paths(cbor),
            MetaPattern::Or(pattern) => pattern.paths(cbor),
            MetaPattern::Not(pattern) => pattern.paths(cbor),
            MetaPattern::Repeat(pattern) => pattern.paths(cbor),
            MetaPattern::Capture(pattern) => pattern.paths(cbor),
            MetaPattern::Search(pattern) => pattern.paths(cbor),
            MetaPattern::Sequence(pattern) => pattern.paths(cbor),
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            MetaPattern::Any(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::And(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::Or(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::Not(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::Repeat(pattern) => {
                pattern.compile(code, lits, captures)
            }
            MetaPattern::Capture(pattern) => {
                pattern.compile(code, lits, captures)
            }
            MetaPattern::Search(pattern) => {
                pattern.compile(code, lits, captures)
            }
            MetaPattern::Sequence(pattern) => {
                pattern.compile(code, lits, captures)
            }
        }
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        match self {
            MetaPattern::Any(pattern) => pattern.collect_capture_names(names),
            MetaPattern::And(pattern) => pattern.collect_capture_names(names),
            MetaPattern::Or(pattern) => pattern.collect_capture_names(names),
            MetaPattern::Not(pattern) => pattern.collect_capture_names(names),
            MetaPattern::Repeat(pattern) => {
                pattern.collect_capture_names(names)
            }
            MetaPattern::Capture(pattern) => {
                pattern.collect_capture_names(names)
            }
            MetaPattern::Search(pattern) => {
                pattern.collect_capture_names(names)
            }
            MetaPattern::Sequence(pattern) => {
                pattern.collect_capture_names(names)
            }
        }
    }

    fn is_complex(&self) -> bool {
        match self {
            MetaPattern::Any(pattern) => pattern.is_complex(),
            MetaPattern::And(pattern) => pattern.is_complex(),
            MetaPattern::Or(pattern) => pattern.is_complex(),
            MetaPattern::Not(pattern) => pattern.is_complex(),
            MetaPattern::Repeat(pattern) => pattern.is_complex(),
            MetaPattern::Capture(pattern) => pattern.is_complex(),
            MetaPattern::Search(pattern) => pattern.is_complex(),
            MetaPattern::Sequence(pattern) => pattern.is_complex(),
        }
    }

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        match self {
            MetaPattern::Any(pattern) => pattern.paths_with_captures(cbor),
            MetaPattern::And(pattern) => pattern.paths_with_captures(cbor),
            MetaPattern::Or(pattern) => pattern.paths_with_captures(cbor),
            MetaPattern::Not(pattern) => pattern.paths_with_captures(cbor),
            MetaPattern::Repeat(pattern) => pattern.paths_with_captures(cbor),
            MetaPattern::Capture(pattern) => pattern.paths_with_captures(cbor),
            MetaPattern::Search(pattern) => pattern.paths_with_captures(cbor),
            MetaPattern::Sequence(pattern) => pattern.paths_with_captures(cbor),
        }
    }
}

impl std::fmt::Display for MetaPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaPattern::Any(pattern) => pattern.fmt(f),
            MetaPattern::And(pattern) => pattern.fmt(f),
            MetaPattern::Or(pattern) => pattern.fmt(f),
            MetaPattern::Not(pattern) => pattern.fmt(f),
            MetaPattern::Repeat(pattern) => pattern.fmt(f),
            MetaPattern::Capture(pattern) => pattern.fmt(f),
            MetaPattern::Search(pattern) => pattern.fmt(f),
            MetaPattern::Sequence(pattern) => pattern.fmt(f),
        }
    }
}

use std::collections::HashMap;

use dcbor::prelude::*;

use crate::pattern::{Pattern, vm::Instr};

/// A sequence of `CBOR` that match a pattern, starting from the root of the
/// dCBOR item.
pub type Path = Vec<CBOR>;

#[doc(hidden)]
pub trait Matcher: std::fmt::Debug + std::fmt::Display + Clone {
    /// Return all matching paths along with any named captures.
    fn paths_with_captures(
        &self,
        _haystack: &CBOR,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        unimplemented!(
            "Matcher::paths_with_captures not implemented for {:?}",
            self
        )
    }

    /// Return only the matching paths, discarding any captures.
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        self.paths_with_captures(haystack).0
    }

    fn matches(&self, haystack: &CBOR) -> bool { !self.paths(haystack).is_empty() }

    fn compile(
        &self,
        _code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        unimplemented!("Matcher::compile not implemented for {:?}", self);
    }

    /// Recursively collect all capture names from this pattern.
    fn collect_capture_names(&self, _names: &mut Vec<String>) {
        // Default implementation does nothing - only capture patterns
        // and patterns containing them need to override this
    }

    /// Should return true if the Display of the matcher is *complex*,
    /// i.e. contains nested patterns or other complex structures
    /// that require its text rendering to be surrounded by grouping
    /// parentheses.
    fn is_complex(&self) -> bool { false }
}

use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that captures matches.
///
/// Capture patterns wrap another pattern and assign a name to the paths
/// that match. The captured paths can be retrieved when pattern matching
/// is performed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePattern {
    name: String,
    pattern: Box<Pattern>,
}

impl CapturePattern {
    /// Creates a new `CapturePattern` with the given name and pattern.
    pub fn new(name: impl AsRef<str>, pattern: Pattern) -> Self {
        CapturePattern {
            name: name.as_ref().to_string(),
            pattern: Box::new(pattern),
        }
    }

    /// Returns the name of the capture.
    pub fn name(&self) -> &str { &self.name }

    /// Returns the inner pattern.
    pub fn pattern(&self) -> &Pattern { &self.pattern }
}

impl Matcher for CapturePattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        // For the basic paths() method, we just return the paths from the inner
        // pattern The capture functionality is handled by the VM during
        // compilation/execution
        self.pattern.paths(haystack)
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        // Register this capture name and get its index
        let capture_idx = captures.len();
        captures.push(self.name.clone());

        // Emit capture start instruction
        code.push(Instr::CaptureStart(capture_idx));

        // Compile the inner pattern
        self.pattern.compile(code, literals, captures);

        // Emit capture end instruction
        code.push(Instr::CaptureEnd(capture_idx));
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        // Add this capture's name
        names.push(self.name.clone());

        // Recursively collect from the inner pattern
        self.pattern.collect_capture_names(names);
    }

    fn is_complex(&self) -> bool {
        // A capture pattern is complex if its inner pattern is complex
        self.pattern.is_complex()
    }

    fn paths_with_captures(
        &self,
        haystack: &CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // Get paths from the inner pattern
        let (paths, mut captures) = self.pattern.paths_with_captures(haystack);

        // For all paths that match, add them as captures for this capture name
        if !paths.is_empty() {
            captures.insert(self.name.clone(), paths.clone());
        }

        (paths, captures)
    }
}

impl std::fmt::Display for CapturePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}({})", self.name, self.pattern)
    }
}

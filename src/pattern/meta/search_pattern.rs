use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that searches the entire dCBOR tree for matches.
///
/// This pattern recursively traverses the dCBOR tree and applies the inner
/// pattern at each node, returning all matching paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPattern(Box<Pattern>);

impl SearchPattern {
    /// Creates a new `SearchPattern` that searches for the given pattern.
    pub fn new(pattern: Pattern) -> Self { SearchPattern(Box::new(pattern)) }

    /// Returns a reference to the inner pattern.
    pub fn pattern(&self) -> &Pattern { &self.0 }

    // Helper method to recursively search through CBOR tree
    fn search_recursive(
        &self,
        cbor: &CBOR,
        path: Vec<CBOR>,
        results: &mut Vec<Path>,
    ) {
        // Test the pattern against this node
        let pattern_paths = self.0.paths(cbor);

        // If the pattern matches, add the current path to results
        if !pattern_paths.is_empty() {
            results.push(path.clone());
        }

        // Recursively search children based on CBOR type
        match cbor.as_case() {
            CBORCase::Array(arr) => {
                for child in arr.iter() {
                    let mut new_path = path.clone();
                    new_path.push(child.clone());
                    self.search_recursive(child, new_path, results);
                }
            }
            CBORCase::Map(map) => {
                for (key, value) in map.iter() {
                    // Search both keys and values
                    let mut key_path = path.clone();
                    key_path.push(key.clone());
                    self.search_recursive(key, key_path, results);

                    let mut value_path = path.clone();
                    value_path.push(value.clone());
                    self.search_recursive(value, value_path, results);
                }
            }
            CBORCase::Tagged(_, content) => {
                let mut new_path = path.clone();
                new_path.push(content.clone());
                self.search_recursive(content, new_path, results);
            }
            _ => {
                // Leaf nodes (primitives) - no children to search
            }
        }
    }

    // Helper method to recursively search through CBOR tree with capture
    // support
    fn search_recursive_with_captures(
        &self,
        cbor: &CBOR,
        path: Vec<CBOR>,
        results: &mut Vec<Path>,
        all_captures: &mut std::collections::HashMap<String, Vec<Path>>,
    ) {
        // Test the pattern against this node with captures
        let (pattern_paths, captures) = self.0.paths_with_captures(cbor);

        // If the pattern matches, add the current path to results and handle
        // captures
        if !pattern_paths.is_empty() {
            results.push(path.clone());

            // For search patterns, the captured paths should be based on the
            // current search location, not the inner pattern's
            // paths
            for (name, _capture_paths) in captures {
                // The capture should be the path to the location where the
                // match occurred
                all_captures.entry(name).or_default().push(path.clone());
            }
        }

        // Recursively search children based on CBOR type
        match cbor.as_case() {
            CBORCase::Array(arr) => {
                for child in arr.iter() {
                    let mut new_path = path.clone();
                    new_path.push(child.clone());
                    self.search_recursive_with_captures(
                        child,
                        new_path,
                        results,
                        all_captures,
                    );
                }
            }
            CBORCase::Map(map) => {
                for (key, value) in map.iter() {
                    // Search both keys and values
                    let mut key_path = path.clone();
                    key_path.push(key.clone());
                    self.search_recursive_with_captures(
                        key,
                        key_path,
                        results,
                        all_captures,
                    );

                    let mut value_path = path.clone();
                    value_path.push(value.clone());
                    self.search_recursive_with_captures(
                        value,
                        value_path,
                        results,
                        all_captures,
                    );
                }
            }
            CBORCase::Tagged(_, content) => {
                let mut tagged_path = path.clone();
                tagged_path.push(content.clone());
                self.search_recursive_with_captures(
                    content,
                    tagged_path,
                    results,
                    all_captures,
                );
            }
            _ => {
                // For primitive types, no further recursion
            }
        }
    }
}

impl Default for SearchPattern {
    fn default() -> Self {
        // Create a default search pattern that matches any value
        Self::new(Pattern::any())
    }
}

impl Matcher for SearchPattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        let mut result_paths = Vec::new();
        self.search_recursive(haystack, vec![haystack.clone()], &mut result_paths);

        // Remove duplicates based on CBOR values in the path
        let mut seen = std::collections::HashSet::new();
        let mut unique = Vec::new();
        for path in result_paths {
            // Create a unique key based on the path's CBOR values
            let path_key: Vec<_> = path
                .iter()
                .map(|cbor| cbor.to_cbor_data()) // Use serialized form as key
                .collect();
            if seen.insert(path_key) {
                unique.push(path);
            }
        }

        unique
    }

    fn paths_with_captures(
        &self,
        haystack: &CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        let mut result_paths = Vec::new();
        let mut all_captures = std::collections::HashMap::new();

        self.search_recursive_with_captures(
            haystack,
            vec![haystack.clone()],
            &mut result_paths,
            &mut all_captures,
        );

        // Remove duplicates from result paths
        let mut seen = std::collections::HashSet::new();
        let mut unique_paths = Vec::new();
        for path in result_paths {
            let path_key: Vec<_> =
                path.iter().map(|cbor| cbor.to_cbor_data()).collect();
            if seen.insert(path_key) {
                unique_paths.push(path);
            }
        }

        (unique_paths, all_captures)
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        // Delegate to the inner pattern to collect its capture names
        self.0.collect_capture_names(names);
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push((*self.0).clone());

        // Collect capture names from the inner pattern
        let mut inner_names = Vec::new();
        self.0.collect_capture_names(&mut inner_names);
        let mut capture_map = Vec::new();

        for name in inner_names {
            let pos = if let Some(i) = captures.iter().position(|n| n == &name)
            {
                i
            } else {
                let i = captures.len();
                captures.push(name.clone());
                i
            };
            capture_map.push((name, pos));
        }

        code.push(Instr::Search { pat_idx: idx, capture_map });
    }
}

impl std::fmt::Display for SearchPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "search({})", self.pattern())
    }
}

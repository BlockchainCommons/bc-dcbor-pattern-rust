use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that matches a sequence of patterns in order.
///
/// This pattern is used to match multiple patterns consecutively,
/// which is particularly useful for array elements or other sequential
/// data structures.
///
/// # Examples
///
/// ```
/// use dcbor_pattern::Pattern;
///
/// // Match a sequence of three specific text values
/// let pattern = Pattern::sequence(vec![
///     Pattern::text("first"),
///     Pattern::text("second"),
///     Pattern::text("third"),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequencePattern(Vec<Pattern>);

impl SequencePattern {
    /// Creates a new sequence pattern with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Self { Self(patterns) }

    /// Returns a reference to the patterns in this sequence.
    pub fn patterns(&self) -> &[Pattern] { &self.0 }

    /// Returns true if this sequence is empty.
    pub fn is_empty(&self) -> bool { self.patterns().is_empty() }

    /// Returns the number of patterns in this sequence.
    pub fn len(&self) -> usize { self.patterns().len() }
}

impl Matcher for SequencePattern {
    fn paths(&self, _cbor: &CBOR) -> Vec<Path> {
        // For a sequence pattern, we need to find paths where all patterns
        // match consecutively. This is primarily used within array patterns
        // or other structural contexts.
        //
        // Since sequence patterns are typically used as part of other patterns
        // (like arrays), and not as standalone matchers against arbitrary CBOR,
        // we return empty paths when used directly.
        //
        // The actual sequence matching logic is handled by the VM through
        // SequenceStart and SequenceNext instructions.
        vec![]
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        if self.patterns().is_empty() {
            // Empty sequence always matches
            return;
        }

        if self.patterns().len() == 1 {
            // Single pattern, just compile it directly
            self.patterns()[0].compile(code, literals, captures);
            return;
        }

        // Multiple patterns in sequence - use ExtendSequence and
        // CombineSequence to implement proper sequence semantics in the
        // VM
        for (i, pattern) in self.patterns().iter().enumerate() {
            if i > 0 {
                // For patterns after the first, extend the sequence to move to
                // next element
                code.push(Instr::ExtendSequence);
            }

            // Compile the pattern to match current element
            pattern.compile(code, literals, captures);

            if i > 0 {
                // Combine the sequence after matching (except for the first
                // pattern)
                code.push(Instr::CombineSequence);
            }
        }
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        for pattern in self.patterns() {
            pattern.collect_capture_names(names);
        }
    }

    fn is_complex(&self) -> bool {
        // A sequence is complex if it contains multiple patterns or
        // if any of its patterns are complex
        self.patterns().len() > 1 || self.patterns().iter().any(|p| p.is_complex())
    }

    fn paths_with_captures(
        &self,
        cbor: &CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // For sequence patterns, the capture logic is handled by the
        // VM when compiled by the main Pattern. When called directly,
        // we use the basic implementation.
        (self.paths(cbor), std::collections::HashMap::new())
    }
}

impl std::fmt::Display for SequencePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.patterns().is_empty() {
            write!(f, "()")
        } else {
            let patterns_str: Vec<String> =
                self.patterns().iter().map(|p| p.to_string()).collect();
            write!(f, "{}", patterns_str.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use dcbor_parse::parse_dcbor_item;

    use super::*;

    #[test]
    fn test_sequence_pattern_new() {
        let patterns = vec![Pattern::text("first"), Pattern::text("second")];
        let sequence = SequencePattern::new(patterns.clone());
        assert_eq!(sequence.patterns(), &patterns);
    }

    #[test]
    fn test_sequence_pattern_empty() {
        let sequence = SequencePattern::new(vec![]);
        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn test_sequence_pattern_len() {
        let patterns =
            vec![Pattern::text("a"), Pattern::text("b"), Pattern::text("c")];
        let sequence = SequencePattern::new(patterns);
        assert!(!sequence.is_empty());
        assert_eq!(sequence.len(), 3);
    }

    #[test]
    fn test_sequence_pattern_display() {
        let patterns = vec![
            Pattern::text("first"),
            Pattern::text("second"),
            Pattern::text("third"),
        ];
        let sequence = SequencePattern::new(patterns);
        let display = sequence.to_string();
        assert!(display.contains("first"));
        assert!(display.contains("second"));
        assert!(display.contains("third"));
        assert!(display.contains(", "));
    }

    #[test]
    fn test_sequence_pattern_display_empty() {
        let sequence = SequencePattern::new(vec![]);
        assert_eq!(sequence.to_string(), "()");
    }

    #[test]
    fn test_sequence_pattern_is_complex() {
        // Empty sequence is not complex
        let empty_sequence = SequencePattern::new(vec![]);
        assert!(!empty_sequence.is_complex());

        // Single simple pattern is not complex
        let single_sequence = SequencePattern::new(vec![Pattern::text("test")]);
        assert!(!single_sequence.is_complex());

        // Multiple patterns are complex
        let multi_sequence = SequencePattern::new(vec![
            Pattern::text("first"),
            Pattern::text("second"),
        ]);
        assert!(multi_sequence.is_complex());
    }

    #[test]
    fn test_sequence_pattern_compile() {
        let patterns = vec![Pattern::text("first"), Pattern::text("second")];
        let sequence = SequencePattern::new(patterns);

        let mut code = Vec::new();
        let mut literals = Vec::new();
        let mut captures = Vec::new();

        sequence.compile(&mut code, &mut literals, &mut captures);

        // Should compile both patterns sequentially
        assert!(!code.is_empty());
        // Should have two patterns in literals (one for each text pattern)
        assert_eq!(literals.len(), 2);
    }

    #[test]
    fn test_sequence_pattern_compile_empty() {
        let sequence = SequencePattern::new(vec![]);

        let mut code = Vec::new();
        let mut literals = Vec::new();
        let mut captures = Vec::new();

        sequence.compile(&mut code, &mut literals, &mut captures);

        // Empty sequence should not add any instructions
        assert!(code.is_empty());
    }

    #[test]
    fn test_sequence_pattern_collect_capture_names() {
        let patterns = vec![
            Pattern::capture("first", Pattern::text("a")),
            Pattern::text("b"),
            Pattern::capture("third", Pattern::text("c")),
        ];
        let sequence = SequencePattern::new(patterns);

        let mut names = Vec::new();
        sequence.collect_capture_names(&mut names);

        assert_eq!(names.len(), 2);
        assert!(names.contains(&"first".to_string()));
        assert!(names.contains(&"third".to_string()));
    }

    #[test]
    fn test_sequence_pattern_paths() {
        let patterns = vec![Pattern::text("a"), Pattern::text("b")];
        let sequence = SequencePattern::new(patterns);

        let cbor = "test".to_cbor();
        let paths = sequence.paths(&cbor);

        // Sequence patterns return empty paths when used directly
        assert!(paths.is_empty());
    }

    #[test]
    fn test_sequence_pattern_with_array() {
        // Test sequence pattern within an array context using parse_dcbor_item
        let _array_cbor =
            parse_dcbor_item(r#"["first", "second", "third"]"#).unwrap();

        // Create a sequence pattern that matches the array elements
        let sequence = SequencePattern::new(vec![
            Pattern::text("first"),
            Pattern::text("second"),
            Pattern::text("third"),
        ]);

        // Verify the sequence pattern structure
        assert_eq!(sequence.len(), 3);
        assert!(!sequence.is_empty());
        assert!(sequence.is_complex()); // Multiple patterns make it complex

        // Test display format
        let display = sequence.to_string();
        assert!(display.contains("first"));
        assert!(display.contains("second"));
        assert!(display.contains("third"));
        assert!(display.contains(", "));
    }

    #[test]
    fn test_sequence_pattern_with_mixed_types() {
        // Test sequence with different CBOR types
        let _mixed_array =
            parse_dcbor_item(r#"[42, "hello", true, null]"#).unwrap();

        let sequence = SequencePattern::new(vec![
            Pattern::number(42),
            Pattern::text("hello"),
            Pattern::bool(true),
            Pattern::null(),
        ]);

        // Verify the sequence pattern properties
        assert_eq!(sequence.len(), 4);
        assert!(sequence.is_complex());

        // Test compilation
        let mut code = Vec::new();
        let mut literals = Vec::new();
        let mut captures = Vec::new();
        sequence.compile(&mut code, &mut literals, &mut captures);

        assert!(!code.is_empty());
        assert_eq!(literals.len(), 4); // One literal for each pattern
    }

    #[test]
    fn test_sequence_pattern_partial_match() {
        // Test sequence that should match part of a larger array
        let _large_array =
            parse_dcbor_item(r#"["start", "middle1", "middle2", "end"]"#)
                .unwrap();

        // Create a sequence pattern for the middle elements
        let sequence = SequencePattern::new(vec![
            Pattern::text("middle1"),
            Pattern::text("middle2"),
        ]);

        // Verify the sequence properties
        assert_eq!(sequence.len(), 2);
        assert!(!sequence.is_empty());
        assert!(sequence.is_complex());

        let display = sequence.to_string();
        assert!(display.contains("middle1"));
        assert!(display.contains("middle2"));
        assert!(display.contains(", "));
    }

    #[test]
    fn test_sequence_pattern_with_captures() {
        // Test sequence pattern with capture groups
        let sequence = SequencePattern::new(vec![
            Pattern::capture("first_value", Pattern::text("hello")),
            Pattern::capture("second_value", Pattern::number(42)),
            Pattern::text("world"),
        ]);

        let mut names = Vec::new();
        sequence.collect_capture_names(&mut names);

        assert_eq!(names.len(), 2);
        assert!(names.contains(&"first_value".to_string()));
        assert!(names.contains(&"second_value".to_string()));

        // Test the display format includes captures
        let display = sequence.to_string();
        assert!(display.contains("@first_value"));
        assert!(display.contains("@second_value"));
        assert!(display.contains(", "));
    }

    #[test]
    fn test_sequence_pattern_with_simple_types() {
        // Test sequence with various simple CBOR types
        let _simple_array = parse_dcbor_item(r#"["text", 123, true]"#).unwrap();

        let sequence = SequencePattern::new(vec![
            Pattern::text("text"),
            Pattern::number(123),
            Pattern::bool(true),
        ]);

        // Verify sequence structure
        assert_eq!(sequence.len(), 3);
        assert!(sequence.is_complex());

        // Test compilation
        let mut code = Vec::new();
        let mut literals = Vec::new();
        let mut captures = Vec::new();
        sequence.compile(&mut code, &mut literals, &mut captures);

        assert!(!code.is_empty());
        assert_eq!(literals.len(), 3); // One literal for each pattern
    }

    #[test]
    fn test_sequence_pattern_with_byte_strings() {
        // Test sequence with byte strings
        let _bytes_array =
            parse_dcbor_item(r#"[h'deadbeef', h'cafebabe', "text"]"#).unwrap();

        let sequence = SequencePattern::new(vec![
            Pattern::byte_string(hex::decode("deadbeef").unwrap()),
            Pattern::byte_string(hex::decode("cafebabe").unwrap()),
            Pattern::text("text"),
        ]);

        // Verify the sequence structure
        assert_eq!(sequence.len(), 3);
        assert!(sequence.is_complex());

        // Check that each pattern in the sequence is correctly represented
        let patterns = sequence.patterns();
        assert_eq!(patterns.len(), 3);

        // Test display formatting
        let display = sequence.to_string();
        assert!(display.contains(", "));
    }
}

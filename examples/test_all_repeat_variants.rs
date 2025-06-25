use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    ArrayPattern, Matcher, Pattern, Quantifier, Reluctance, StructurePattern,
};

fn test_repeat_variant(
    name: &str,
    pattern: Pattern,
    test_cases: &[(&str, bool)],
) {
    println!("Testing {}: {}", name, pattern);

    let array_pattern = Pattern::Structure(StructurePattern::Array(
        ArrayPattern::with_elements(pattern),
    ));

    for (array_str, expected) in test_cases {
        let test_array = parse_dcbor_item(array_str).unwrap();
        let matches = array_pattern.matches(&test_array);
        let status = if matches == *expected { "✅" } else { "❌" };
        println!(
            "  {} {} -> {} (expected {})",
            status,
            array_str,
            if matches { "MATCH" } else { "NO MATCH" },
            if *expected { "MATCH" } else { "NO MATCH" }
        );
    }
    println!();
}

fn main() {
    println!("Testing all repeat pattern variants with array matching...\n");

    // Test data: arrays to test against
    let test_cases = [
        ("[]", "Empty array"),
        ("[42]", "Single element"),
        ("[1, 2]", "Two elements"),
        ("[1, 2, 3]", "Three elements"),
        ("[1, 2, 3, 4, 5]", "Five elements"),
    ];

    println!("Test arrays:");
    for (array_str, description) in &test_cases {
        println!("  {} - {}", array_str, description);
    }
    println!();

    // 1. Test basic quantifiers with greedy reluctance
    println!("=== Basic Quantifiers (Greedy) ===");

    // (ANY)* - 0 or more
    let any_star = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=usize::MAX, Reluctance::Greedy),
    );
    test_repeat_variant(
        "(ANY)*",
        any_star,
        &[
            ("[]", true),              // 0 elements - should match
            ("[42]", true),            // 1 element - should match
            ("[1, 2]", true),          // 2 elements - should match
            ("[1, 2, 3]", true),       // 3 elements - should match
            ("[1, 2, 3, 4, 5]", true), // 5 elements - should match
        ],
    );

    // (ANY)+ - 1 or more
    let any_plus = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(1..=usize::MAX, Reluctance::Greedy),
    );
    test_repeat_variant(
        "(ANY)+",
        any_plus,
        &[
            ("[]", false),             // 0 elements - should NOT match
            ("[42]", true),            // 1 element - should match
            ("[1, 2]", true),          // 2 elements - should match
            ("[1, 2, 3]", true),       // 3 elements - should match
            ("[1, 2, 3, 4, 5]", true), // 5 elements - should match
        ],
    );

    // (ANY)? - 0 or 1
    let any_question = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=1, Reluctance::Greedy),
    );
    test_repeat_variant(
        "(ANY)?",
        any_question,
        &[
            ("[]", true),               // 0 elements - should match
            ("[42]", true),             // 1 element - should match
            ("[1, 2]", false),          // 2 elements - should NOT match
            ("[1, 2, 3]", false),       // 3 elements - should NOT match
            ("[1, 2, 3, 4, 5]", false), // 5 elements - should NOT match
        ],
    );

    // (ANY){2,4} - between 2 and 4
    let any_range = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(2..=4, Reluctance::Greedy),
    );
    test_repeat_variant(
        "(ANY){2,4}",
        any_range,
        &[
            ("[]", false),              // 0 elements - should NOT match
            ("[42]", false),            // 1 element - should NOT match
            ("[1, 2]", true),           // 2 elements - should match
            ("[1, 2, 3]", true),        // 3 elements - should match
            ("[1, 2, 3, 4, 5]", false), /* 5 elements - should NOT match
                                         * (exceeds max) */
        ],
    );

    // 2. Test lazy quantifiers
    println!("=== Lazy Quantifiers ===");

    // (ANY)*? - 0 or more, lazy
    let any_star_lazy = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=usize::MAX, Reluctance::Lazy),
    );
    test_repeat_variant(
        "(ANY)*?",
        any_star_lazy,
        &[
            ("[]", true),              // Should match (prefers fewer)
            ("[42]", true),            // Should match
            ("[1, 2]", true),          // Should match
            ("[1, 2, 3]", true),       // Should match
            ("[1, 2, 3, 4, 5]", true), // Should match
        ],
    );

    // (ANY)+? - 1 or more, lazy
    let any_plus_lazy = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(1..=usize::MAX, Reluctance::Lazy),
    );
    test_repeat_variant(
        "(ANY)+?",
        any_plus_lazy,
        &[
            ("[]", false),             // Should NOT match (needs at least 1)
            ("[42]", true),            // Should match
            ("[1, 2]", true),          // Should match
            ("[1, 2, 3]", true),       // Should match
            ("[1, 2, 3, 4, 5]", true), // Should match
        ],
    );

    // (ANY)?? - 0 or 1, lazy
    let any_question_lazy = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=1, Reluctance::Lazy),
    );
    test_repeat_variant(
        "(ANY)??",
        any_question_lazy,
        &[
            ("[]", true),               // Should match (prefers 0)
            ("[42]", true),             // Should match
            ("[1, 2]", false),          // Should NOT match (exceeds max)
            ("[1, 2, 3]", false),       // Should NOT match (exceeds max)
            ("[1, 2, 3, 4, 5]", false), // Should NOT match (exceeds max)
        ],
    );

    // 3. Test possessive quantifiers
    println!("=== Possessive Quantifiers ===");

    // (ANY)*+ - 0 or more, possessive
    let any_star_possessive = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(0..=usize::MAX, Reluctance::Possessive),
    );
    test_repeat_variant(
        "(ANY)*+",
        any_star_possessive,
        &[
            ("[]", true),              // Should match
            ("[42]", true),            // Should match
            ("[1, 2]", true),          // Should match
            ("[1, 2, 3]", true),       // Should match
            ("[1, 2, 3, 4, 5]", true), // Should match
        ],
    );

    // (ANY)++ - 1 or more, possessive
    let any_plus_possessive = Pattern::repeat(
        Pattern::any(),
        Quantifier::new(1..=usize::MAX, Reluctance::Possessive),
    );
    test_repeat_variant(
        "(ANY)++",
        any_plus_possessive,
        &[
            ("[]", false),             // Should NOT match
            ("[42]", true),            // Should match
            ("[1, 2]", true),          // Should match
            ("[1, 2, 3]", true),       // Should match
            ("[1, 2, 3, 4, 5]", true), // Should match
        ],
    );

    // 4. Test complex sequence patterns with different quantifiers
    println!("=== Complex Sequence Patterns ===");

    // (ANY)*>NUMBER(42)>(ANY)* - find 42 anywhere (greedy)
    let find_42_greedy = Pattern::sequence(vec![
        Pattern::repeat(
            Pattern::any(),
            Quantifier::new(0..=usize::MAX, Reluctance::Greedy),
        ),
        Pattern::number(42),
        Pattern::repeat(
            Pattern::any(),
            Quantifier::new(0..=usize::MAX, Reluctance::Greedy),
        ),
    ]);

    let find_42_array = Pattern::Structure(StructurePattern::Array(
        ArrayPattern::with_elements(find_42_greedy),
    ));

    println!("Testing complex sequence (ANY)*>NUMBER(42)>(ANY)*:");
    let complex_test_cases = [
        ("[42]", true),       // Just 42
        ("[1, 42]", true),    // 42 at end
        ("[42, 1]", true),    // 42 at start
        ("[1, 42, 3]", true), // 42 in middle
        ("[1, 2, 3]", false), // No 42
        ("[]", false),        // Empty (can't match NUMBER(42))
    ];

    for (array_str, expected) in &complex_test_cases {
        let test_array = parse_dcbor_item(array_str).unwrap();
        let matches = find_42_array.matches(&test_array);
        let status = if matches == *expected { "✅" } else { "❌" };
        println!(
            "  {} {} -> {} (expected {})",
            status,
            array_str,
            if matches { "MATCH" } else { "NO MATCH" },
            if *expected { "MATCH" } else { "NO MATCH" }
        );
    }
}

use dcbor::{Date, prelude::*};
use dcbor_pattern::{Matcher, Pattern};

fn main() {
    println!("Testing DatePattern with CBOR tag verification");

    // Create a date and convert to CBOR
    let date = Date::from_ymd(2023, 12, 25);
    let date_cbor = date.to_cbor();

    // Verify the CBOR structure
    if let CBORCase::Tagged(tag, _) = date_cbor.as_case() {
        println!("Date CBOR has tag: {}", tag.value());
        assert_eq!(tag.value(), 1, "Date should have tag 1");
    } else {
        panic!("Date CBOR should be tagged");
    }

    // Test different date patterns
    let patterns = vec![
        ("Any date", Pattern::any_date()),
        ("Specific date", Pattern::date(date.clone())),
        (
            "Date range",
            Pattern::date_range(
                Date::from_ymd(2023, 12, 20)..=Date::from_ymd(2023, 12, 30),
            ),
        ),
        (
            "Earliest date",
            Pattern::date_earliest(Date::from_ymd(2023, 12, 1)),
        ),
        (
            "Latest date",
            Pattern::date_latest(Date::from_ymd(2023, 12, 31)),
        ),
        ("ISO 8601 match", Pattern::date_iso8601(date.to_string())),
    ];

    for (name, pattern) in patterns {
        let matches = pattern.matches(&date_cbor);
        let paths = pattern.paths(&date_cbor);
        println!("{}: matches={}, paths_count={}", name, matches, paths.len());
        assert!(matches, "{} should match", name);
        assert_eq!(paths.len(), 1, "{} should have one path", name);
    }

    // Test with regex pattern
    let regex = regex::Regex::new(r"^2023-12-").unwrap();
    let regex_pattern = Pattern::date_regex(regex);
    assert!(
        regex_pattern.matches(&date_cbor),
        "Regex pattern should match"
    );

    // Test with non-matching patterns
    let non_matching_patterns = vec![
        ("Different date", Pattern::date(Date::from_ymd(2024, 1, 1))),
        (
            "Earlier earliest",
            Pattern::date_earliest(Date::from_ymd(2024, 1, 1)),
        ),
        (
            "Later latest",
            Pattern::date_latest(Date::from_ymd(2023, 12, 1)),
        ),
        (
            "Non-matching ISO",
            Pattern::date_iso8601("2024-01-01T00:00:00Z"),
        ),
    ];

    for (name, pattern) in non_matching_patterns {
        let matches = pattern.matches(&date_cbor);
        let paths = pattern.paths(&date_cbor);
        println!("{}: matches={}, paths_count={}", name, matches, paths.len());
        assert!(!matches, "{} should not match", name);
        assert_eq!(paths.len(), 0, "{} should have no paths", name);
    }

    // Test with non-date CBOR
    let text_cbor = "2023-12-25".to_cbor();
    let number_cbor = 1703462400.to_cbor(); // Unix timestamp

    let any_date_pattern = Pattern::any_date();
    assert!(
        !any_date_pattern.matches(&text_cbor),
        "Should not match text"
    );
    assert!(
        !any_date_pattern.matches(&number_cbor),
        "Should not match number"
    );

    println!("All DatePattern tests passed!");
}

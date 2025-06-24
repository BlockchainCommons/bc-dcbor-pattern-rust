use dcbor::{Date, prelude::*};
use dcbor_pattern::{Matcher, Pattern};

fn main() {
    // Test creating different date patterns
    let date = Date::from_ymd(2023, 12, 25);

    // Any date pattern
    let any_pattern = Pattern::any_date();
    println!("Any date pattern: {}", any_pattern);

    // Specific date pattern
    let specific_pattern = Pattern::date(date.clone());
    println!("Specific date pattern: {}", specific_pattern);

    // Range pattern
    let start = Date::from_ymd(2023, 12, 20);
    let end = Date::from_ymd(2023, 12, 30);
    let range_pattern = Pattern::date_range(start..=end);
    println!("Date range pattern: {}", range_pattern);

    // Earliest pattern
    let earliest_pattern = Pattern::date_earliest(date.clone());
    println!("Earliest pattern: {}", earliest_pattern);

    // Test matching
    let date_cbor = date.to_cbor();
    println!(
        "Does any pattern match date CBOR? {}",
        any_pattern.matches(&date_cbor)
    );

    // Test with non-date CBOR
    let text_cbor = "2023-12-25".to_cbor();
    println!(
        "Does any pattern match text CBOR? {}",
        any_pattern.matches(&text_cbor)
    );
}

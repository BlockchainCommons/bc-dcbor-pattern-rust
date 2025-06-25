// Example demonstrating the new MAP key-value constraints functionality

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{MapPattern, Matcher, Pattern};

fn main() {
    // Test data - a user profile map
    let user_profile = parse_dcbor_item(
        r#"{
        "name": "Alice Smith",
        "age": 30,
        "email": "alice@example.com",
        "active": true,
        "preferences": {
            "theme": "dark",
            "notifications": true
        }
    }"#,
    )
    .unwrap();

    // Example 1: Single key-value constraint using the new API
    let pattern1 = MapPattern::with_key_value_constraints(vec![(
        Pattern::text("name"),
        Pattern::any_text(),
    )]);
    println!(
        "Single constraint matches: {}",
        pattern1.matches(&user_profile)
    );

    // Example 2: Multiple key-value constraints using the new API
    let pattern2 = MapPattern::with_key_value_constraints(vec![
        (Pattern::text("name"), Pattern::any_text()),
        (Pattern::text("age"), Pattern::any_number()),
        (Pattern::text("active"), Pattern::bool(true)),
    ]);
    println!(
        "Multiple constraints match: {}",
        pattern2.matches(&user_profile)
    );

    // Example 3: Using the new text syntax parser
    let pattern3 =
        Pattern::parse(r#"MAP(TEXT("name"):TEXT, TEXT("age"):NUMBER)"#)
            .unwrap();
    println!("Text syntax matches: {}", pattern3.matches(&user_profile));
    println!("Pattern display: {}", pattern3);

    // Example 4: Complex constraints with specific values
    let pattern4 =
        Pattern::parse(r#"MAP(TEXT("active"):BOOL(true), ANY:TEXT)"#).unwrap();
    println!(
        "Complex constraint matches: {}",
        pattern4.matches(&user_profile)
    );

    // Example 5: Testing against a different map
    let simple_map =
        parse_dcbor_item(r#"{"status": "active", "count": 5}"#).unwrap();
    let pattern5 = Pattern::parse(
        r#"MAP(TEXT("status"):TEXT("active"), TEXT("count"):NUMBER)"#,
    )
    .unwrap();
    println!("Simple map matches: {}", pattern5.matches(&simple_map));
}

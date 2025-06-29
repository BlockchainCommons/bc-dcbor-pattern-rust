use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

#[test]
fn parse_bool_any() {
    let src = "bool";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_bool());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_bool_true() {
    let src = "true";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(true));
    assert_eq!(p.to_string(), src);

    // Test that the standalone 'true' pattern works correctly
    let parsed_standalone = Pattern::parse("true").unwrap();
    assert_eq!(parsed_standalone, Pattern::bool(true));
    assert_eq!(parsed_standalone.to_string(), "true");
}

#[test]
fn parse_bool_false() {
    let src = "false";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(false));
    assert_eq!(p.to_string(), src);

    // Test that the standalone 'false' pattern works correctly
    let parsed_standalone = Pattern::parse("false").unwrap();
    assert_eq!(parsed_standalone, Pattern::bool(false));
    assert_eq!(parsed_standalone.to_string(), "false");
}

#[test]
fn parse_bool_patterns_round_trip() {
    let patterns = vec![
        Pattern::any_bool(),
        Pattern::bool(true),
        Pattern::bool(false),
    ];

    for pattern in patterns {
        let string_repr = pattern.to_string();
        let parsed = Pattern::parse(&string_repr).unwrap();
        assert_eq!(parsed, pattern);
    }
}

#[test]
fn parse_null() {
    let src = "NULL";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::null());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_null_pattern_round_trip() {
    let pattern = Pattern::null();
    let string_repr = pattern.to_string();
    let parsed = Pattern::parse(&string_repr).unwrap();
    assert_eq!(parsed, pattern);
}

#[test]
fn parse_date_any() {
    let src = "DATE";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_date());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_date_value() {
    let src = "DATE(2023-12-25)";
    let p = Pattern::parse(src).unwrap();

    // Create expected date using dcbor-parse
    let date_cbor = cbor("2023-12-25");
    let expected_date = dcbor::Date::try_from(date_cbor).unwrap();

    assert_eq!(p, Pattern::date(expected_date.clone()));
    // The display format may be different than input due to date normalization
    assert!(p.to_string().starts_with("DATE("));
}

#[test]
fn parse_date_with_time() {
    let src = "DATE(2023-12-25T15:30:45Z)";
    let p = Pattern::parse(src).unwrap();

    // Create expected date using dcbor-parse
    let date_cbor = cbor("2023-12-25T15:30:45Z");
    let expected_date = dcbor::Date::try_from(date_cbor).unwrap();

    assert_eq!(p, Pattern::date(expected_date));
    assert!(p.to_string().starts_with("DATE("));
}

#[test]
fn parse_date_range() {
    let src = "DATE(2023-12-24...2023-12-26)";
    let p = Pattern::parse(src).unwrap();

    // Create expected dates using dcbor-parse
    let start_cbor = cbor("2023-12-24");
    let end_cbor = cbor("2023-12-26");
    let start_date = dcbor::Date::try_from(start_cbor).unwrap();
    let end_date = dcbor::Date::try_from(end_cbor).unwrap();

    assert_eq!(p, Pattern::date_range(start_date..=end_date));
    assert!(p.to_string().contains("..."));
}

#[test]
fn parse_date_earliest() {
    let src = "DATE(2023-12-24...)";
    let p = Pattern::parse(src).unwrap();

    // Create expected date using dcbor-parse
    let date_cbor = cbor("2023-12-24");
    let expected_date = dcbor::Date::try_from(date_cbor).unwrap();

    assert_eq!(p, Pattern::date_earliest(expected_date));
    assert!(p.to_string().ends_with("...)"));
}

#[test]
fn parse_date_latest() {
    let src = "DATE(...2023-12-26)";
    let p = Pattern::parse(src).unwrap();

    // Create expected date using dcbor-parse
    let date_cbor = cbor("2023-12-26");
    let expected_date = dcbor::Date::try_from(date_cbor).unwrap();

    assert_eq!(p, Pattern::date_latest(expected_date));
    assert!(p.to_string().starts_with("DATE(..."));
}

#[test]
fn parse_date_regex() {
    let src = "DATE(/2023-.*/)";
    let p = Pattern::parse(src).unwrap();

    let regex = regex::Regex::new("2023-.*").unwrap();
    assert_eq!(p, Pattern::date_regex(regex));
    assert_eq!(p.to_string(), "DATE(/2023-.*/)");
}

#[test]
fn parse_date_patterns_round_trip() {
    // Create some date patterns and test round-trip parsing
    let date1 = dcbor::Date::try_from(cbor("2023-05-15")).unwrap();
    let date2 = dcbor::Date::try_from(cbor("2023-12-25")).unwrap();

    let patterns = vec![
        Pattern::any_date(),
        Pattern::date(date1.clone()),
        Pattern::date_range(date1..=date2.clone()),
        Pattern::date_earliest(date2.clone()),
        Pattern::date_latest(date2),
    ];

    for pattern in patterns {
        let string_repr = pattern.to_string();
        let parsed = Pattern::parse(&string_repr).unwrap();
        assert_eq!(parsed, pattern);
    }
}

#[test]
fn parse_date_spaced() {
    let spaced = "DATE ( 2023-12-25 )";
    let p = Pattern::parse(spaced).unwrap();

    let date_cbor = cbor("2023-12-25");
    let expected_date = dcbor::Date::try_from(date_cbor).unwrap();

    assert_eq!(p, Pattern::date(expected_date));
}

#[test]
fn parse_text_any() {
    let src = "TEXT";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_text());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_text_literal() {
    let src = r#"TEXT("hello")"#;
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::text("hello"));
    assert_eq!(p.to_string(), src);

    let spaced = r#"TEXT ( "hello" )"#;
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::text("hello"));
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_text_literal_with_spaces() {
    let src = r#"TEXT("hello world")"#;
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::text("hello world"));
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_text_literal_with_escapes() {
    let src = r#"TEXT("say \"hello\"")"#;
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::text(r#"say "hello""#));
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_text_regex() {
    let src = r"TEXT(/h.*o/)";
    let p = Pattern::parse(src).unwrap();
    let regex = regex::Regex::new("h.*o").unwrap();
    assert_eq!(p, Pattern::text_regex(regex));
    assert_eq!(p.to_string(), src);

    let spaced = r"TEXT ( /h.*o/ )";
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::text_regex(regex::Regex::new("h.*o").unwrap())
    );
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_text_regex_digits() {
    let src = r"TEXT(/^\d+$/)";
    let p = Pattern::parse(src).unwrap();
    let regex = regex::Regex::new(r"^\d+$").unwrap();
    assert_eq!(p, Pattern::text_regex(regex));
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_text_patterns_round_trip() {
    let patterns = vec![
        Pattern::any_text(),
        Pattern::text("hello"),
        Pattern::text("hello world"),
        Pattern::text(r#"say "hello""#),
        Pattern::text_regex(regex::Regex::new(r"^\d+$").unwrap()),
        Pattern::text_regex(regex::Regex::new("h.*o").unwrap()),
    ];

    for pattern in patterns {
        let string_repr = pattern.to_string();
        let parsed = Pattern::parse(&string_repr).unwrap();
        assert_eq!(
            pattern, parsed,
            "Round trip failed for pattern: {}",
            string_repr
        );
    }
}

#[test]
fn parse_text_edge_cases() {
    // Empty string
    let src = r#"TEXT("")"#;
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::text(""));
    assert_eq!(p.to_string(), src);

    // String with newlines and special characters
    let src = r#"TEXT("Hello\nWorld")"#;
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::text("Hello\\nWorld"));

    // Regex with special characters
    let src = r"TEXT(/[a-zA-Z]+/)";
    let p = Pattern::parse(src).unwrap();
    let regex = regex::Regex::new("[a-zA-Z]+").unwrap();
    assert_eq!(p, Pattern::text_regex(regex));
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_bytestring_any() {
    let src = "BSTR";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_byte_string());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_bytestring_hex() {
    let src = r#"BSTR(h'010203')"#;
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::byte_string(vec![1, 2, 3]));
    assert_eq!(p.to_string(), src);

    let spaced = r#"BSTR ( h'010203' )"#;
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::byte_string(vec![1, 2, 3]));
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_bytestring_regex() {
    let src = r"BSTR(/^[0-9]+$/)";
    let p = Pattern::parse(src).unwrap();
    let regex = regex::bytes::Regex::new(r"^[0-9]+$").unwrap();
    assert_eq!(p, Pattern::byte_string_regex(regex));
    assert_eq!(p.to_string(), src);

    // Test pattern matching
    let pattern = Pattern::parse("BSTR(/abc/)").unwrap();
    assert!(pattern.matches(&cbor(r#"h'616263'"#))); // "abc" in hex
    assert!(!pattern.matches(&cbor(r#"h'646566'"#))); // "def" in hex
}

#[test]
fn parse_bytestring_patterns_round_trip() {
    let cases = vec![
        "BSTR",
        r#"BSTR(h'deadbeef')"#,
        r#"BSTR(h'')"#,
        r"BSTR(/^[a-f0-9]+$/)",
        r"BSTR(/test/)",
    ];

    for case in cases {
        let p = Pattern::parse(case).unwrap();
        let round_trip = Pattern::parse(&p.to_string()).unwrap();
        assert_eq!(p, round_trip, "Round trip failed for: {}", case);
    }
}

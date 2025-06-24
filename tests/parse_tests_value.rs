use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::Pattern;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

#[test]
fn parse_bool_any() {
    let src = "BOOL";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_bool());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_bool_true() {
    let src = "BOOL(true)";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(true));
    assert_eq!(p.to_string(), src);

    let spaced = "BOOL ( true )";
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::bool(true));
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_bool_false() {
    let src = "BOOL(false)";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(false));
    assert_eq!(p.to_string(), src);

    let spaced = "BOOL ( false )";
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::bool(false));
    assert_eq!(p_spaced.to_string(), src);
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

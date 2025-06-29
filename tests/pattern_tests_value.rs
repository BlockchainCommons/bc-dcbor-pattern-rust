mod common;

use dcbor::{Date, prelude::*};
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

#[test]
fn test_bool_pattern_any() {
    let pattern = parse("bool");

    // Should match true
    let true_cbor = cbor("true");
    let paths = pattern.paths(&true_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        true
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match false
    let false_cbor = cbor("false");
    let paths = pattern.paths(&false_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        false
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-boolean
    let number_cbor = cbor("42");
    assert!(!pattern.matches(&number_cbor));
}

#[test]
fn test_bool_pattern_specific() {
    let true_pattern = parse("true");
    let false_pattern = parse("false");

    let true_cbor = cbor("true");
    let false_cbor = cbor("false");
    let number_cbor = cbor("42");

    // true pattern tests
    let paths = true_pattern.paths(&true_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        true
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!true_pattern.matches(&false_cbor));
    assert!(!true_pattern.matches(&number_cbor));

    // false pattern tests
    assert!(!false_pattern.matches(&true_cbor));
    let paths = false_pattern.paths(&false_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        false
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!false_pattern.matches(&number_cbor));
}

#[test]
fn test_bool_pattern_display() {
    assert_eq!(parse("bool").to_string(), "bool");
    assert_eq!(parse("true").to_string(), "true");
    assert_eq!(parse("false").to_string(), "false");
}

#[test]
fn test_text_pattern_any() {
    let pattern = parse("TEXT");

    // Should match any text
    let hello_cbor = cbor(r#""Hello""#);
    let paths = pattern.paths(&hello_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        "Hello"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let empty_cbor = cbor(r#""""#);
    let paths = pattern.paths(&empty_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        ""
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-text
    let number_cbor = cbor("42");
    assert!(!pattern.matches(&number_cbor));
}

#[test]
fn test_text_pattern_specific() {
    let hello_pattern = parse(r#"TEXT("Hello")"#);
    let world_pattern = parse(r#"TEXT("World")"#);

    let hello_cbor = cbor(r#""Hello""#);
    let world_cbor = cbor(r#""World""#);
    let number_cbor = cbor("42");

    // hello pattern tests
    let paths = hello_pattern.paths(&hello_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        "Hello"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!hello_pattern.matches(&world_cbor));
    assert!(!hello_pattern.matches(&number_cbor));

    // world pattern tests
    assert!(!world_pattern.matches(&hello_cbor));
    let paths = world_pattern.paths(&world_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        "World"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!world_pattern.matches(&number_cbor));
}

#[test]
fn test_text_pattern_regex() {
    let digits_regex = regex::Regex::new(r"^\d+$").unwrap();
    let digits_pattern = Pattern::text_regex(digits_regex);

    let digits_cbor = cbor(r#""12345""#);
    let letters_cbor = cbor(r#""Hello""#);
    let mixed_cbor = cbor(r#""Hello123""#);
    let number_cbor = cbor("42");

    // Should match pure digits
    let paths = digits_pattern.paths(&digits_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        "12345"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match letters, mixed content, or non-text
    assert!(!digits_pattern.matches(&letters_cbor));
    assert!(!digits_pattern.matches(&mixed_cbor));
    assert!(!digits_pattern.matches(&number_cbor));
}

#[test]
fn test_text_pattern_display() {
    assert_eq!(parse("TEXT").to_string(), "TEXT");
    assert_eq!(parse(r#"TEXT("Hello")"#).to_string(), r#"TEXT("Hello")"#);

    let regex_pattern =
        Pattern::text_regex(regex::Regex::new(r"^\d+$").unwrap());
    assert_eq!(regex_pattern.to_string(), r#"TEXT(/^\d+$/)"#);
}

#[test]
fn test_number_pattern_any() {
    let pattern = parse("NUMBER");

    // Should match integers
    let int_cbor = cbor("42");
    let paths = pattern.paths(&int_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match floats
    let float_cbor = cbor("3.2222");
    let paths = pattern.paths(&float_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        3.2222
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match negative numbers
    let neg_cbor = cbor("-5");
    let paths = pattern.paths(&neg_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        -5
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-numbers
    let text_cbor = cbor(r#""42""#);
    assert!(!pattern.matches(&text_cbor));
}

#[test]
fn test_number_pattern_specific() {
    let int_pattern = parse("NUMBER(42)");
    let float_pattern = parse("NUMBER(3.2222)");

    let int_cbor = cbor("42");
    let float_cbor = cbor("3.2222");
    let different_int_cbor = cbor("24");
    let text_cbor = cbor(r#""42""#);

    // int pattern tests
    let paths = int_pattern.paths(&int_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!int_pattern.matches(&float_cbor));
    assert!(!int_pattern.matches(&different_int_cbor));
    assert!(!int_pattern.matches(&text_cbor));

    // float pattern tests
    assert!(!float_pattern.matches(&int_cbor));
    let paths = float_pattern.paths(&float_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        3.2222
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!float_pattern.matches(&text_cbor));
}

#[test]
fn test_number_pattern_range() {
    let range_pattern = parse("NUMBER(10...20)");

    let in_range_cbor = cbor("15");
    let boundary_low_cbor = cbor("10");
    let boundary_high_cbor = cbor("20");
    let below_range_cbor = cbor("5");
    let above_range_cbor = cbor("25");
    let text_cbor = cbor(r#""15""#);

    // Should match numbers in range
    let paths = range_pattern.paths(&in_range_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        15
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let paths = range_pattern.paths(&boundary_low_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        10
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let paths = range_pattern.paths(&boundary_high_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        20
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match numbers outside range
    assert!(!range_pattern.matches(&below_range_cbor));
    assert!(!range_pattern.matches(&above_range_cbor));
    assert!(!range_pattern.matches(&text_cbor));
}

#[test]
fn test_number_pattern_comparisons() {
    let gt_pattern = parse("NUMBER(>10)");
    let gte_pattern = parse("NUMBER(>=10)");
    let lt_pattern = parse("NUMBER(<10)");
    let lte_pattern = parse("NUMBER(<=10)");

    let equal_cbor = cbor("10");
    let greater_cbor = cbor("15");
    let lesser_cbor = cbor("5");

    // Greater than tests
    assert!(!gt_pattern.matches(&equal_cbor));
    let paths = gt_pattern.paths(&greater_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        15
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    assert!(!gt_pattern.matches(&lesser_cbor));

    // Greater than or equal tests
    let paths = gte_pattern.paths(&equal_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        10
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    assert!(gte_pattern.matches(&greater_cbor));
    assert!(!gte_pattern.matches(&lesser_cbor));

    // Less than tests
    assert!(!lt_pattern.matches(&equal_cbor));
    assert!(!lt_pattern.matches(&greater_cbor));
    let paths = lt_pattern.paths(&lesser_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        5
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Less than or equal tests
    assert!(lte_pattern.matches(&equal_cbor));
    assert!(!lte_pattern.matches(&greater_cbor));
    assert!(lte_pattern.matches(&lesser_cbor));
}

#[test]
fn test_number_pattern_nan() {
    let nan_pattern = parse("NUMBER(NaN)");

    let nan_cbor = cbor("NaN");
    let number_cbor = cbor("42");
    let text_cbor = cbor(r#""NaN""#);

    // Should match NaN
    let paths = nan_pattern.paths(&nan_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        NaN
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match regular numbers or text
    assert!(!nan_pattern.matches(&number_cbor));
    assert!(!nan_pattern.matches(&text_cbor));
}

#[test]
fn test_number_pattern_display() {
    assert_eq!(parse("NUMBER").to_string(), "NUMBER");
    assert_eq!(parse("NUMBER(42)").to_string(), "NUMBER(42)");
    assert_eq!(parse("NUMBER(3.2222)").to_string(), "NUMBER(3.2222)");
    assert_eq!(parse("NUMBER(10...20)").to_string(), "NUMBER(10...20)");
    assert_eq!(parse("NUMBER(>10)").to_string(), "NUMBER(>10)");
    assert_eq!(parse("NUMBER(>=10)").to_string(), "NUMBER(>=10)");
    assert_eq!(parse("NUMBER(<10)").to_string(), "NUMBER(<10)");
    assert_eq!(parse("NUMBER(<=10)").to_string(), "NUMBER(<=10)");
    assert_eq!(parse("NUMBER(NaN)").to_string(), "NUMBER(NaN)");
}

#[test]
fn test_byte_string_pattern_any() {
    let pattern = parse("BSTR");

    // Should match any byte string
    let cbor_bytes = cbor("h'01020304'");
    let paths = pattern.paths(&cbor_bytes);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h'01020304'
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let empty_cbor = cbor("h''");
    let paths = pattern.paths(&empty_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h''
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-byte-string
    let text_cbor = cbor(r#""hello""#);
    assert!(!pattern.matches(&text_cbor));
}

#[test]
fn test_byte_string_pattern_specific() {
    let exact_pattern = parse("BSTR(h'01020304')");
    let different_pattern = parse("BSTR(h'0506')");

    let cbor_bytes = cbor("h'01020304'");
    let different_cbor = cbor("h'0506'");
    let text_cbor = cbor(r#""hello""#);

    // exact pattern tests
    let paths = exact_pattern.paths(&cbor_bytes);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h'01020304'
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!exact_pattern.matches(&different_cbor));
    assert!(!exact_pattern.matches(&text_cbor));

    // different pattern tests
    assert!(!different_pattern.matches(&cbor_bytes));
    let paths = different_pattern.paths(&different_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h'0506'
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!different_pattern.matches(&text_cbor));
}

#[test]
fn test_byte_string_pattern_regex() {
    // Test with binary data that looks like digits
    let digits_regex = regex::bytes::Regex::new(r"^\d+$").unwrap();
    let digits_pattern = Pattern::byte_string_regex(digits_regex);

    let digits_cbor = cbor("h'3132333435'"); // "12345" in hex
    let letters_cbor = cbor("h'48656c6c6f'"); // "Hello" in hex
    let mixed_cbor = cbor("h'48656c6c6f313233'"); // "Hello123" in hex
    let text_cbor = cbor(r#""12345""#);

    // Should match byte strings with digits
    let paths = digits_pattern.paths(&digits_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h'3132333435'
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match letters, mixed content, or text strings
    assert!(!digits_pattern.matches(&letters_cbor));
    assert!(!digits_pattern.matches(&mixed_cbor));
    assert!(!digits_pattern.matches(&text_cbor));
}

#[test]
fn test_byte_string_pattern_binary_data() {
    let pattern = parse("BSTR");

    // Test with actual binary data (not text)
    let binary_cbor = cbor("h'00010203fffefd'");

    let paths = pattern.paths(&binary_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h'00010203fffefd'
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let exact_pattern = parse("BSTR(h'00010203fffefd')");
    let paths = exact_pattern.paths(&binary_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h'00010203fffefd'
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let different_pattern = parse("BSTR(h'000102')");
    assert!(!different_pattern.matches(&binary_cbor));

    // Test regex that matches any bytes starting with 0x00
    let starts_with_zero_regex = regex::bytes::Regex::new(r"^\x00").unwrap();
    let starts_with_zero_pattern =
        Pattern::byte_string_regex(starts_with_zero_regex);
    let paths = starts_with_zero_pattern.paths(&binary_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        h'00010203fffefd'
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Test regex that doesn't match
    let starts_with_one_regex = regex::bytes::Regex::new(r"^\x01").unwrap();
    let starts_with_one_pattern =
        Pattern::byte_string_regex(starts_with_one_regex);
    assert!(!starts_with_one_pattern.matches(&binary_cbor));
}

#[test]
fn test_byte_string_pattern_display() {
    assert_eq!(parse("BSTR").to_string(), "BSTR");
    assert_eq!(parse("BSTR(h'deadbeef')").to_string(), "BSTR(h'deadbeef')");

    let regex = regex::bytes::Regex::new(r"^test.*").unwrap();
    let regex_pattern = Pattern::byte_string_regex(regex);
    assert_eq!(regex_pattern.to_string(), "BSTR(/^test.*/)");
}

#[test]
fn test_date_pattern_any() {
    let pattern = parse("DATE");

    // Should match any date
    let date = Date::from_ymd(2023, 12, 25);
    let date_cbor = date.to_cbor();
    let paths = pattern.paths(&date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-date
    let text_cbor = cbor(r#""2023-12-25""#);
    assert!(!pattern.matches(&text_cbor));

    let number_cbor = cbor("1703462400"); // Unix timestamp for 2023-12-25
    assert!(!pattern.matches(&number_cbor));
}

#[test]
fn test_date_pattern_specific() {
    let date = Date::from_ymd(2023, 12, 25);
    let pattern = Pattern::date(date.clone());

    // Should match the specific date
    let date_cbor = date.to_cbor();
    let paths = pattern.paths(&date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match a different date
    let other_date = Date::from_ymd(2024, 1, 1);
    let other_date_cbor = other_date.to_cbor();
    assert!(!pattern.matches(&other_date_cbor));

    // Should not match non-date
    let text_cbor = cbor(r#""2023-12-25""#);
    assert!(!pattern.matches(&text_cbor));
}

#[test]
fn test_date_pattern_range() {
    let start_date = Date::from_ymd(2023, 12, 20);
    let end_date = Date::from_ymd(2023, 12, 30);
    let pattern = Pattern::date_range(start_date.clone()..=end_date.clone());

    // Should match date within range
    let middle_date = Date::from_ymd(2023, 12, 25);
    let middle_date_cbor = middle_date.to_cbor();
    let paths = pattern.paths(&middle_date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match date at start of range
    let start_date_cbor = start_date.to_cbor();
    let paths = pattern.paths(&start_date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703030400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match date at end of range
    let end_date_cbor = end_date.to_cbor();
    let paths = pattern.paths(&end_date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703894400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match date before range
    let early_date = Date::from_ymd(2023, 12, 15);
    let early_date_cbor = early_date.to_cbor();
    assert!(!pattern.matches(&early_date_cbor));

    // Should not match date after range
    let late_date = Date::from_ymd(2024, 1, 5);
    let late_date_cbor = late_date.to_cbor();
    assert!(!pattern.matches(&late_date_cbor));
}

#[test]
fn test_date_pattern_earliest() {
    let earliest_date = Date::from_ymd(2023, 12, 20);
    let pattern = Pattern::date_earliest(earliest_date.clone());

    // Should match date equal to earliest
    let earliest_date_cbor = earliest_date.to_cbor();
    let paths = pattern.paths(&earliest_date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703030400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match date after earliest
    let later_date = Date::from_ymd(2023, 12, 25);
    let later_date_cbor = later_date.to_cbor();
    let paths = pattern.paths(&later_date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match date before earliest
    let earlier_date = Date::from_ymd(2023, 12, 15);
    let earlier_date_cbor = earlier_date.to_cbor();
    assert!(!pattern.matches(&earlier_date_cbor));
}

#[test]
fn test_date_pattern_latest() {
    let latest_date = Date::from_ymd(2023, 12, 30);
    let pattern = Pattern::date_latest(latest_date.clone());

    // Should match date equal to latest
    let latest_date_cbor = latest_date.to_cbor();
    let paths = pattern.paths(&latest_date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703894400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match date before latest
    let earlier_date = Date::from_ymd(2023, 12, 25);
    let earlier_date_cbor = earlier_date.to_cbor();
    let paths = pattern.paths(&earlier_date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match date after latest
    let later_date = Date::from_ymd(2024, 1, 5);
    let later_date_cbor = later_date.to_cbor();
    assert!(!pattern.matches(&later_date_cbor));
}

#[test]
fn test_date_pattern_iso8601() {
    let date = Date::from_ymd(2023, 12, 25);
    let iso_string = date.to_string();
    let pattern = Pattern::date_iso8601(iso_string.clone());

    // Should match date with matching ISO string
    let date_cbor = date.to_cbor();
    let paths = pattern.paths(&date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match date with different ISO string
    let other_date = Date::from_ymd(2024, 1, 1);
    let other_date_cbor = other_date.to_cbor();
    assert!(!pattern.matches(&other_date_cbor));
}

#[test]
fn test_date_pattern_regex() {
    // Pattern to match any date in 2023
    let regex = regex::Regex::new(r"^2023-").unwrap();
    let pattern = Pattern::date_regex(regex);

    // Should match date in 2023
    let date_2023 = Date::from_ymd(2023, 12, 25);
    let date_2023_cbor = date_2023.to_cbor();
    let paths = pattern.paths(&date_2023_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match date in 2024
    let date_2024 = Date::from_ymd(2024, 1, 1);
    let date_2024_cbor = date_2024.to_cbor();
    assert!(!pattern.matches(&date_2024_cbor));

    // Test with more specific regex (December dates)
    let december_regex = regex::Regex::new(r"-12-").unwrap();
    let december_pattern = Pattern::date_regex(december_regex);

    // Should match December date
    let paths = december_pattern.paths(&date_2023_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match January date
    let january_date = Date::from_ymd(2023, 1, 15);
    let january_date_cbor = january_date.to_cbor();
    assert!(!december_pattern.matches(&january_date_cbor));
}

#[test]
fn test_date_pattern_with_time() {
    // Test with dates that include time components
    let datetime = Date::from_timestamp(1703462400.0); // 2023-12-25 00:00:00 UTC
    let pattern = parse("DATE");

    let datetime_cbor = datetime.to_cbor();
    let paths = pattern.paths(&datetime_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Test specific time matching
    let specific_pattern = Pattern::date(datetime.clone());
    let paths = specific_pattern.paths(&datetime_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Test with fractional seconds
    let datetime_with_millis = Date::from_timestamp(1703462400.123);
    let datetime_with_millis_cbor = datetime_with_millis.to_cbor();
    let paths = pattern.paths(&datetime_with_millis_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1(1703462400.123)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn test_date_pattern_display() {
    assert_eq!(parse("DATE").to_string(), "DATE");

    let date = Date::from_ymd(2023, 12, 25);
    assert_eq!(
        Pattern::date(date.clone()).to_string(),
        format!("DATE({})", date)
    );

    let start_date = Date::from_ymd(2023, 12, 20);
    let end_date = Date::from_ymd(2023, 12, 30);
    assert_eq!(
        Pattern::date_range(start_date.clone()..=end_date.clone()).to_string(),
        format!("DATE({}...{})", start_date, end_date)
    );

    assert_eq!(
        Pattern::date_earliest(date.clone()).to_string(),
        format!("DATE({}...)", date)
    );

    assert_eq!(
        Pattern::date_latest(date.clone()).to_string(),
        format!("DATE(...{})", date)
    );

    assert_eq!(
        Pattern::date_iso8601("2023-12-25T00:00:00Z").to_string(),
        "DATE(2023-12-25T00:00:00Z)"
    );

    let regex = regex::Regex::new(r"^2023-").unwrap();
    assert_eq!(Pattern::date_regex(regex).to_string(), "DATE(/^2023-/)");
}

#[test]
fn test_null_pattern() {
    let pattern = parse("NULL");

    // Should match null
    let null_cbor = cbor("null");
    let paths = pattern.paths(&null_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        null
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-null values
    let true_cbor = cbor("true");
    assert!(!pattern.matches(&true_cbor));

    let false_cbor = cbor("false");
    assert!(!pattern.matches(&false_cbor));

    let number_cbor = cbor("42");
    assert!(!pattern.matches(&number_cbor));

    let text_cbor = cbor(r#""hello""#);
    assert!(!pattern.matches(&text_cbor));

    let array_cbor = cbor("[1, 2, 3]");
    assert!(!pattern.matches(&array_cbor));
}

#[test]
fn test_null_pattern_display() {
    assert_eq!(parse("NULL").to_string(), "NULL");
}

#[test]
fn test_known_value_pattern_any() {
    let pattern = parse("KNOWN");

    // Test with known values represented as tagged values with tag 40000
    let known_value_cbor = cbor("'1'"); // This represents known_values::IS_A as 40000(1)
    let paths = pattern.paths(&known_value_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(1)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Test with another known value
    let date_value_cbor = cbor("'16'"); // This represents known_values::DATE as 40000(16)
    let paths = pattern.paths(&date_value_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(16)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Test with custom known value
    let custom_value_cbor = cbor("'12345'");
    let paths = pattern.paths(&custom_value_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(12345)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match plain unsigned integers (these are NOT known values)
    let plain_int_cbor = cbor("1");
    assert!(!pattern.matches(&plain_int_cbor));

    let text_cbor = cbor(r#""hello""#);
    assert!(!pattern.matches(&text_cbor));

    let negative_cbor = cbor("-1");
    assert!(!pattern.matches(&negative_cbor));
}

#[test]
fn test_known_value_pattern_specific() {
    let is_a_pattern = parse("KNOWN('isA')");
    let date_pattern = parse("KNOWN('date')");

    let is_a_cbor = cbor("'1'"); // IS_A value as 40000(1)
    let date_cbor = cbor("'16'"); // DATE value as 40000(16)
    let other_cbor = cbor("'42'"); // Some other known value as 40000(42)
    let plain_int_cbor = cbor("1"); // Plain integer, NOT a known value
    let text_cbor = cbor(r#""hello""#);

    // is_a pattern tests
    let paths = is_a_pattern.paths(&is_a_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(1)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!is_a_pattern.matches(&date_cbor));
    assert!(!is_a_pattern.matches(&other_cbor));
    assert!(!is_a_pattern.matches(&plain_int_cbor)); // Should NOT match plain integers
    assert!(!is_a_pattern.matches(&text_cbor));

    // date pattern tests
    assert!(!date_pattern.matches(&is_a_cbor));
    let paths = date_pattern.paths(&date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(16)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!date_pattern.matches(&other_cbor));
    assert!(!date_pattern.matches(&plain_int_cbor)); // Should NOT match plain integers
    assert!(!date_pattern.matches(&text_cbor));
}

#[test]
fn test_known_value_pattern_named() {
    let is_a_pattern = parse("KNOWN('isA')");
    let date_pattern = parse("KNOWN('date')");
    let unknown_pattern = parse("KNOWN('unknownValue')");

    let is_a_cbor = cbor("'1'"); // IS_A value as 40000(1)
    let date_cbor = cbor("'16'"); // DATE value as 40000(16)
    let other_cbor = cbor("'42'"); // Some other known value as 40000(42)
    let plain_int_cbor = cbor("1"); // Plain integer, NOT a known value
    let text_cbor = cbor(r#""hello""#);

    // is_a pattern tests
    let paths = is_a_pattern.paths(&is_a_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(1)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!is_a_pattern.matches(&date_cbor));
    assert!(!is_a_pattern.matches(&other_cbor));
    assert!(!is_a_pattern.matches(&plain_int_cbor)); // Should NOT match plain integers
    assert!(!is_a_pattern.matches(&text_cbor));

    // date pattern tests
    assert!(!date_pattern.matches(&is_a_cbor));
    let paths = date_pattern.paths(&date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(16)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!date_pattern.matches(&other_cbor));
    assert!(!date_pattern.matches(&plain_int_cbor)); // Should NOT match plain integers
    assert!(!date_pattern.matches(&text_cbor));

    // unknown pattern tests (should not match anything)
    assert!(!unknown_pattern.matches(&is_a_cbor));
    assert!(!unknown_pattern.matches(&date_cbor));
    assert!(!unknown_pattern.matches(&other_cbor));
    assert!(!unknown_pattern.matches(&plain_int_cbor));
    assert!(!unknown_pattern.matches(&text_cbor));
}

#[test]
fn test_known_value_pattern_regex() {
    // Test regex that matches names starting with "is"
    let is_pattern =
        Pattern::known_value_regex(regex::Regex::new(r"^is.*").unwrap());

    // Test regex that matches names ending with "te"
    let te_pattern =
        Pattern::known_value_regex(regex::Regex::new(r".*te$").unwrap());

    // Test regex that doesn't match any known value names
    let no_match_pattern =
        Pattern::known_value_regex(regex::Regex::new(r"^xyz.*").unwrap());

    let is_a_cbor = cbor("'1'"); // IS_A value (name: "isA") as 40000(1)
    let date_cbor = cbor("'16'"); // DATE value (name: "date") as 40000(16)
    let note_cbor = cbor("'4'"); // NOTE value (name: "note") as 40000(4)
    let other_cbor = cbor("'42'"); // Some other known value as 40000(42)
    let plain_int_cbor = cbor("1"); // Plain integer, NOT a known value
    let text_cbor = cbor(r#""hello""#);

    // is pattern tests (should match IS_A which starts with "is")
    let paths = is_pattern.paths(&is_a_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(1)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!is_pattern.matches(&date_cbor));
    assert!(!is_pattern.matches(&note_cbor));
    assert!(!is_pattern.matches(&other_cbor));
    assert!(!is_pattern.matches(&plain_int_cbor)); // Should NOT match plain integers
    assert!(!is_pattern.matches(&text_cbor));

    // te pattern tests (should match DATE and NOTE which end with "te")
    assert!(!te_pattern.matches(&is_a_cbor));
    let paths = te_pattern.paths(&date_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(16)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let paths = te_pattern.paths(&note_cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        40000(4)
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert!(!te_pattern.matches(&other_cbor));
    assert!(!te_pattern.matches(&plain_int_cbor)); // Should NOT match plain integers
    assert!(!te_pattern.matches(&text_cbor));

    // no match pattern tests
    assert!(!no_match_pattern.matches(&is_a_cbor));
    assert!(!no_match_pattern.matches(&date_cbor));
    assert!(!no_match_pattern.matches(&note_cbor));
    assert!(!no_match_pattern.matches(&other_cbor));
    assert!(!no_match_pattern.matches(&plain_int_cbor));
    assert!(!no_match_pattern.matches(&text_cbor));
}

#[test]
fn test_known_value_pattern_display() {
    let any_pattern = parse("KNOWN");
    assert_eq!(any_pattern.to_string(), "KNOWN");

    let is_a_pattern = Pattern::known_value(known_values::IS_A);
    assert_eq!(is_a_pattern.to_string(), "KNOWN('isA')");

    let date_pattern = Pattern::known_value(known_values::DATE);
    assert_eq!(date_pattern.to_string(), "KNOWN('date')");

    let named_pattern = Pattern::known_value_named("customName");
    assert_eq!(named_pattern.to_string(), "KNOWN('customName')");

    let regex_pattern =
        Pattern::known_value_regex(regex::Regex::new(r"^is.*").unwrap());
    assert_eq!(regex_pattern.to_string(), "KNOWN(/^is.*/)");
}

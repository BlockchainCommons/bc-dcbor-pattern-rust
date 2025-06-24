use dcbor::{prelude::*, Date};
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn test_bool_pattern_any() {
    let pattern = Pattern::any_bool();

    // Should match true
    let true_cbor = true.to_cbor();
    assert!(pattern.matches(&true_cbor));
    let paths = pattern.paths(&true_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], true_cbor);

    // Should match false
    let false_cbor = false.to_cbor();
    assert!(pattern.matches(&false_cbor));
    let paths = pattern.paths(&false_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], false_cbor);

    // Should not match non-boolean
    let number_cbor = 42.to_cbor();
    assert!(!pattern.matches(&number_cbor));
    let paths = pattern.paths(&number_cbor);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_bool_pattern_specific() {
    let true_pattern = Pattern::bool(true);
    let false_pattern = Pattern::bool(false);

    let true_cbor = true.to_cbor();
    let false_cbor = false.to_cbor();
    let number_cbor = 42.to_cbor();

    // true pattern tests
    assert!(true_pattern.matches(&true_cbor));
    assert!(!true_pattern.matches(&false_cbor));
    assert!(!true_pattern.matches(&number_cbor));

    // false pattern tests
    assert!(!false_pattern.matches(&true_cbor));
    assert!(false_pattern.matches(&false_cbor));
    assert!(!false_pattern.matches(&number_cbor));
}

#[test]
fn test_bool_pattern_display() {
    assert_eq!(Pattern::any_bool().to_string(), "BOOL");
    assert_eq!(Pattern::bool(true).to_string(), "BOOL(true)");
    assert_eq!(Pattern::bool(false).to_string(), "BOOL(false)");
}

#[test]
fn test_text_pattern_any() {
    let pattern = Pattern::any_text();

    // Should match any text
    let hello_cbor = "Hello".to_cbor();
    assert!(pattern.matches(&hello_cbor));
    let paths = pattern.paths(&hello_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], hello_cbor);

    let empty_cbor = "".to_cbor();
    assert!(pattern.matches(&empty_cbor));
    let paths = pattern.paths(&empty_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], empty_cbor);

    // Should not match non-text
    let number_cbor = 42.to_cbor();
    assert!(!pattern.matches(&number_cbor));
    let paths = pattern.paths(&number_cbor);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_text_pattern_specific() {
    let hello_pattern = Pattern::text("Hello");
    let world_pattern = Pattern::text("World");

    let hello_cbor = "Hello".to_cbor();
    let world_cbor = "World".to_cbor();
    let number_cbor = 42.to_cbor();

    // hello pattern tests
    assert!(hello_pattern.matches(&hello_cbor));
    assert!(!hello_pattern.matches(&world_cbor));
    assert!(!hello_pattern.matches(&number_cbor));

    // world pattern tests
    assert!(!world_pattern.matches(&hello_cbor));
    assert!(world_pattern.matches(&world_cbor));
    assert!(!world_pattern.matches(&number_cbor));
}

#[test]
fn test_text_pattern_regex() {
    let digits_regex = regex::Regex::new(r"^\d+$").unwrap();
    let digits_pattern = Pattern::text_regex(digits_regex);

    let digits_cbor = "12345".to_cbor();
    let letters_cbor = "Hello".to_cbor();
    let mixed_cbor = "Hello123".to_cbor();
    let number_cbor = 42.to_cbor();

    // Should match pure digits
    assert!(digits_pattern.matches(&digits_cbor));
    let paths = digits_pattern.paths(&digits_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], digits_cbor);

    // Should not match letters, mixed content, or non-text
    assert!(!digits_pattern.matches(&letters_cbor));
    assert!(!digits_pattern.matches(&mixed_cbor));
    assert!(!digits_pattern.matches(&number_cbor));
}

#[test]
fn test_text_pattern_display() {
    assert_eq!(Pattern::any_text().to_string(), "TEXT");
    assert_eq!(Pattern::text("Hello").to_string(), r#"TEXT("Hello")"#);

    let regex_pattern =
        Pattern::text_regex(regex::Regex::new(r"^\d+$").unwrap());
    assert_eq!(regex_pattern.to_string(), r#"TEXT(/^\d+$/)"#);
}

#[test]
fn test_number_pattern_any() {
    let pattern = Pattern::any_number();

    // Should match integers
    let int_cbor = 42.to_cbor();
    assert!(pattern.matches(&int_cbor));
    let paths = pattern.paths(&int_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], int_cbor);

    // Should match floats
    let float_cbor = 3.2222.to_cbor();
    assert!(pattern.matches(&float_cbor));

    // Should match negative numbers
    let neg_cbor = (-5).to_cbor();
    assert!(pattern.matches(&neg_cbor));

    // Should not match non-numbers
    let text_cbor = "42".to_cbor();
    assert!(!pattern.matches(&text_cbor));
    let paths = pattern.paths(&text_cbor);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_number_pattern_specific() {
    let int_pattern = Pattern::number(42);
    let float_pattern = Pattern::number(3.2222);

    let int_cbor = 42.to_cbor();
    let float_cbor = 3.2222.to_cbor();
    let different_int_cbor = 24.to_cbor();
    let text_cbor = "42".to_cbor();

    // int pattern tests
    assert!(int_pattern.matches(&int_cbor));
    assert!(!int_pattern.matches(&float_cbor));
    assert!(!int_pattern.matches(&different_int_cbor));
    assert!(!int_pattern.matches(&text_cbor));

    // float pattern tests
    assert!(!float_pattern.matches(&int_cbor));
    assert!(float_pattern.matches(&float_cbor));
    assert!(!float_pattern.matches(&text_cbor));
}

#[test]
fn test_number_pattern_range() {
    let range_pattern = Pattern::number_range(10..=20);

    let in_range_cbor = 15.to_cbor();
    let boundary_low_cbor = 10.to_cbor();
    let boundary_high_cbor = 20.to_cbor();
    let below_range_cbor = 5.to_cbor();
    let above_range_cbor = 25.to_cbor();
    let text_cbor = "15".to_cbor();

    // Should match numbers in range
    assert!(range_pattern.matches(&in_range_cbor));
    assert!(range_pattern.matches(&boundary_low_cbor));
    assert!(range_pattern.matches(&boundary_high_cbor));

    // Should not match numbers outside range
    assert!(!range_pattern.matches(&below_range_cbor));
    assert!(!range_pattern.matches(&above_range_cbor));
    assert!(!range_pattern.matches(&text_cbor));
}

#[test]
fn test_number_pattern_comparisons() {
    let gt_pattern = Pattern::number_greater_than(10);
    let gte_pattern = Pattern::number_greater_than_or_equal(10);
    let lt_pattern = Pattern::number_less_than(10);
    let lte_pattern = Pattern::number_less_than_or_equal(10);

    let equal_cbor = 10.to_cbor();
    let greater_cbor = 15.to_cbor();
    let lesser_cbor = 5.to_cbor();

    // Greater than tests
    assert!(!gt_pattern.matches(&equal_cbor));
    assert!(gt_pattern.matches(&greater_cbor));
    assert!(!gt_pattern.matches(&lesser_cbor));

    // Greater than or equal tests
    assert!(gte_pattern.matches(&equal_cbor));
    assert!(gte_pattern.matches(&greater_cbor));
    assert!(!gte_pattern.matches(&lesser_cbor));

    // Less than tests
    assert!(!lt_pattern.matches(&equal_cbor));
    assert!(!lt_pattern.matches(&greater_cbor));
    assert!(lt_pattern.matches(&lesser_cbor));

    // Less than or equal tests
    assert!(lte_pattern.matches(&equal_cbor));
    assert!(!lte_pattern.matches(&greater_cbor));
    assert!(lte_pattern.matches(&lesser_cbor));
}

#[test]
fn test_number_pattern_nan() {
    let nan_pattern = Pattern::number_nan();

    let nan_cbor = f64::NAN.to_cbor();
    let number_cbor = 42.to_cbor();
    let text_cbor = "NaN".to_cbor();

    // Should match NaN
    assert!(nan_pattern.matches(&nan_cbor));

    // Should not match regular numbers or text
    assert!(!nan_pattern.matches(&number_cbor));
    assert!(!nan_pattern.matches(&text_cbor));
}

#[test]
fn test_number_pattern_display() {
    assert_eq!(Pattern::any_number().to_string(), "NUMBER");
    assert_eq!(Pattern::number(42).to_string(), "NUMBER(42)");
    assert_eq!(Pattern::number(3.2222).to_string(), "NUMBER(3.2222)");
    assert_eq!(
        Pattern::number_range(10..=20).to_string(),
        "NUMBER(10...20)"
    );
    assert_eq!(Pattern::number_greater_than(10).to_string(), "NUMBER(>10)");
    assert_eq!(
        Pattern::number_greater_than_or_equal(10).to_string(),
        "NUMBER(>=10)"
    );
    assert_eq!(Pattern::number_less_than(10).to_string(), "NUMBER(<10)");
    assert_eq!(
        Pattern::number_less_than_or_equal(10).to_string(),
        "NUMBER(<=10)"
    );
    assert_eq!(Pattern::number_nan().to_string(), "NUMBER(NaN)");
}

#[test]
fn test_byte_string_pattern_any() {
    let pattern = Pattern::any_byte_string();

    // Should match any byte string
    let binary_data = vec![0x01, 0x02, 0x03, 0x04];
    let cbor_bytes = CBOR::to_byte_string(binary_data);
    assert!(pattern.matches(&cbor_bytes));
    let paths = pattern.paths(&cbor_bytes);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], cbor_bytes);

    let empty_bytes = vec![];
    let empty_cbor = CBOR::to_byte_string(empty_bytes);
    assert!(pattern.matches(&empty_cbor));
    let paths = pattern.paths(&empty_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], empty_cbor);

    // Should not match non-byte-string
    let text_cbor = "hello".to_cbor();
    assert!(!pattern.matches(&text_cbor));
    let paths = pattern.paths(&text_cbor);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_byte_string_pattern_specific() {
    let binary_data = vec![0x01, 0x02, 0x03, 0x04];
    let exact_pattern = Pattern::byte_string(&binary_data);
    let different_pattern = Pattern::byte_string(vec![0x05, 0x06]);

    let cbor_bytes = CBOR::to_byte_string(binary_data.clone());
    let different_cbor = CBOR::to_byte_string(vec![0x05, 0x06]);
    let text_cbor = "hello".to_cbor();

    // exact pattern tests
    assert!(exact_pattern.matches(&cbor_bytes));
    assert!(!exact_pattern.matches(&different_cbor));
    assert!(!exact_pattern.matches(&text_cbor));

    // different pattern tests
    assert!(!different_pattern.matches(&cbor_bytes));
    assert!(different_pattern.matches(&different_cbor));
    assert!(!different_pattern.matches(&text_cbor));
}

#[test]
fn test_byte_string_pattern_regex() {
    // Test with binary data that looks like digits
    let digits_regex = regex::bytes::Regex::new(r"^\d+$").unwrap();
    let digits_pattern = Pattern::byte_string_regex(digits_regex);

    let digits_bytes = b"12345";
    let digits_cbor = CBOR::to_byte_string(digits_bytes);
    let letters_bytes = b"Hello";
    let letters_cbor = CBOR::to_byte_string(letters_bytes);
    let mixed_bytes = b"Hello123";
    let mixed_cbor = CBOR::to_byte_string(mixed_bytes);
    let text_cbor = "12345".to_cbor();

    // Should match byte strings with digits
    assert!(digits_pattern.matches(&digits_cbor));
    let paths = digits_pattern.paths(&digits_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], digits_cbor);

    // Should not match letters, mixed content, or text strings
    assert!(!digits_pattern.matches(&letters_cbor));
    assert!(!digits_pattern.matches(&mixed_cbor));
    assert!(!digits_pattern.matches(&text_cbor));
}

#[test]
fn test_byte_string_pattern_binary_data() {
    let pattern = Pattern::any_byte_string();

    // Test with actual binary data (not text)
    let binary_data = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD];
    let binary_cbor = CBOR::to_byte_string(binary_data.clone());

    assert!(pattern.matches(&binary_cbor));

    let exact_pattern = Pattern::byte_string(binary_data.clone());
    assert!(exact_pattern.matches(&binary_cbor));

    let different_pattern = Pattern::byte_string(vec![0x00, 0x01, 0x02]);
    assert!(!different_pattern.matches(&binary_cbor));

    // Test regex that matches any bytes starting with 0x00
    let starts_with_zero_regex = regex::bytes::Regex::new(r"^\x00").unwrap();
    let starts_with_zero_pattern =
        Pattern::byte_string_regex(starts_with_zero_regex);
    assert!(starts_with_zero_pattern.matches(&binary_cbor));

    // Test regex that doesn't match
    let starts_with_one_regex = regex::bytes::Regex::new(r"^\x01").unwrap();
    let starts_with_one_pattern =
        Pattern::byte_string_regex(starts_with_one_regex);
    assert!(!starts_with_one_pattern.matches(&binary_cbor));
}

#[test]
fn test_byte_string_pattern_display() {
    assert_eq!(Pattern::any_byte_string().to_string(), "BSTR");
    assert_eq!(
        Pattern::byte_string(vec![0xde, 0xad, 0xbe, 0xef]).to_string(),
        "BSTR(h'deadbeef')"
    );

    let regex = regex::bytes::Regex::new(r"^test.*").unwrap();
    let regex_pattern = Pattern::byte_string_regex(regex);
    assert_eq!(regex_pattern.to_string(), "BSTR(/^test.*/)");
}

#[test]
fn test_date_pattern_any() {
    let pattern = Pattern::any_date();

    // Should match any date
    let date = Date::from_ymd(2023, 12, 25);
    let date_cbor = date.to_cbor();
    assert!(pattern.matches(&date_cbor));
    let paths = pattern.paths(&date_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], date_cbor);

    // Should not match non-date
    let text_cbor = "2023-12-25".to_cbor();
    assert!(!pattern.matches(&text_cbor));
    let paths = pattern.paths(&text_cbor);
    assert_eq!(paths.len(), 0);

    let number_cbor = 1703462400.to_cbor(); // Unix timestamp for 2023-12-25
    assert!(!pattern.matches(&number_cbor));
}

#[test]
fn test_date_pattern_specific() {
    let date = Date::from_ymd(2023, 12, 25);
    let pattern = Pattern::date(date.clone());

    // Should match the specific date
    let date_cbor = date.to_cbor();
    assert!(pattern.matches(&date_cbor));

    // Should not match a different date
    let other_date = Date::from_ymd(2024, 1, 1);
    let other_date_cbor = other_date.to_cbor();
    assert!(!pattern.matches(&other_date_cbor));

    // Should not match non-date
    let text_cbor = "2023-12-25".to_cbor();
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
    assert!(pattern.matches(&middle_date_cbor));

    // Should match date at start of range
    let start_date_cbor = start_date.to_cbor();
    assert!(pattern.matches(&start_date_cbor));

    // Should match date at end of range
    let end_date_cbor = end_date.to_cbor();
    assert!(pattern.matches(&end_date_cbor));

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
    assert!(pattern.matches(&earliest_date_cbor));

    // Should match date after earliest
    let later_date = Date::from_ymd(2023, 12, 25);
    let later_date_cbor = later_date.to_cbor();
    assert!(pattern.matches(&later_date_cbor));

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
    assert!(pattern.matches(&latest_date_cbor));

    // Should match date before latest
    let earlier_date = Date::from_ymd(2023, 12, 25);
    let earlier_date_cbor = earlier_date.to_cbor();
    assert!(pattern.matches(&earlier_date_cbor));

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
    assert!(pattern.matches(&date_cbor));

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
    assert!(pattern.matches(&date_2023_cbor));

    // Should not match date in 2024
    let date_2024 = Date::from_ymd(2024, 1, 1);
    let date_2024_cbor = date_2024.to_cbor();
    assert!(!pattern.matches(&date_2024_cbor));

    // Test with more specific regex (December dates)
    let december_regex = regex::Regex::new(r"-12-").unwrap();
    let december_pattern = Pattern::date_regex(december_regex);

    // Should match December date
    assert!(december_pattern.matches(&date_2023_cbor));

    // Should not match January date
    let january_date = Date::from_ymd(2023, 1, 15);
    let january_date_cbor = january_date.to_cbor();
    assert!(!december_pattern.matches(&january_date_cbor));
}

#[test]
fn test_date_pattern_with_time() {
    // Test with dates that include time components
    let datetime = Date::from_timestamp(1703462400.0); // 2023-12-25 00:00:00 UTC
    let pattern = Pattern::any_date();

    let datetime_cbor = datetime.to_cbor();
    assert!(pattern.matches(&datetime_cbor));

    // Test specific time matching
    let specific_pattern = Pattern::date(datetime.clone());
    assert!(specific_pattern.matches(&datetime_cbor));

    // Test with fractional seconds
    let datetime_with_millis = Date::from_timestamp(1703462400.123);
    let datetime_with_millis_cbor = datetime_with_millis.to_cbor();
    assert!(pattern.matches(&datetime_with_millis_cbor));
}

#[test]
fn test_date_pattern_display() {
    assert_eq!(Pattern::any_date().to_string(), "DATE");

    let date = Date::from_ymd(2023, 12, 25);
    assert_eq!(Pattern::date(date.clone()).to_string(), format!("DATE({})", date));

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
    assert_eq!(
        Pattern::date_regex(regex).to_string(),
        "DATE(/^2023-/)"
    );
}

#[test]
fn test_null_pattern() {
    let pattern = Pattern::null();

    // Should match null
    let null_cbor = dcbor::CBOR::null();
    assert!(pattern.matches(&null_cbor));
    let paths = pattern.paths(&null_cbor);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].len(), 1);
    assert_eq!(paths[0][0], null_cbor);

    // Should not match non-null values
    let true_cbor = true.to_cbor();
    assert!(!pattern.matches(&true_cbor));
    let paths = pattern.paths(&true_cbor);
    assert_eq!(paths.len(), 0);

    let false_cbor = false.to_cbor();
    assert!(!pattern.matches(&false_cbor));
    let paths = pattern.paths(&false_cbor);
    assert_eq!(paths.len(), 0);

    let number_cbor = 42.to_cbor();
    assert!(!pattern.matches(&number_cbor));
    let paths = pattern.paths(&number_cbor);
    assert_eq!(paths.len(), 0);

    let text_cbor = "hello".to_cbor();
    assert!(!pattern.matches(&text_cbor));
    let paths = pattern.paths(&text_cbor);
    assert_eq!(paths.len(), 0);

    let array_cbor = vec![1, 2, 3].to_cbor();
    assert!(!pattern.matches(&array_cbor));
    let paths = pattern.paths(&array_cbor);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_null_pattern_display() {
    assert_eq!(Pattern::null().to_string(), "NULL");
}

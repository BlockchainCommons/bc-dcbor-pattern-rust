use bc_components::Digest;
use bc_ur::UREncodable;
use dcbor::CBOREncodable;
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn test_digest_pattern_parsing_any() {
    bc_components::register_tags();

    let src = "digest";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_digest());
    assert_eq!(p.to_string(), src);
}

#[test]
fn test_digest_pattern_parsing_hex_prefix() {
    bc_components::register_tags();

    let src = "digest'a1b2c3'";
    let p = Pattern::parse(src).unwrap();
    let expected_bytes = hex::decode("a1b2c3").unwrap();
    assert_eq!(p, Pattern::digest_prefix(expected_bytes));
    assert_eq!(p.to_string(), src);
}

#[test]
fn test_digest_pattern_parsing_full_hex() {
    bc_components::register_tags();

    let full_digest_hex =
        "4d303dac9eed63573f6190e9c4191be619e03a7b3c21e9bb3d27ac1a55971e6b";
    let src = format!("digest'{}'", full_digest_hex);
    let p = Pattern::parse(&src).unwrap();
    let expected_bytes = hex::decode(full_digest_hex).unwrap();
    assert_eq!(p, Pattern::digest_prefix(expected_bytes));
    assert_eq!(p.to_string(), src);
}

#[test]
fn test_digest_pattern_parsing_ur_string() {
    bc_components::register_tags();

    let digest = Digest::from_image(b"hello world");
    let ur_string = digest.ur_string();
    let src = format!("digest'{}'", ur_string);
    let p = Pattern::parse(&src).unwrap();
    assert_eq!(p, Pattern::digest(digest.clone()));
    assert_eq!(p.to_string(), src);
}

#[test]
fn test_digest_pattern_matching() {
    bc_components::register_tags();

    let digest = Digest::from_image(b"test data");
    let digest_cbor = digest.to_cbor();

    // Test any digest pattern
    let any_pattern = Pattern::parse("digest").unwrap();
    assert!(any_pattern.matches(&digest_cbor));

    // Test specific digest pattern
    let ur_string = digest.ur_string();
    let specific_pattern =
        Pattern::parse(&format!("digest'{}'", ur_string)).unwrap();
    assert!(specific_pattern.matches(&digest_cbor));

    // Test prefix pattern
    let prefix_hex = hex::encode(&digest.data()[..4]);
    let prefix_pattern =
        Pattern::parse(&format!("digest'{}'", prefix_hex)).unwrap();
    assert!(prefix_pattern.matches(&digest_cbor));

    // Test non-matching digest
    let other_digest = Digest::from_image(b"other data");
    let other_digest_cbor = other_digest.to_cbor();
    assert!(!specific_pattern.matches(&other_digest_cbor));
}

#[test]
fn test_digest_pattern_round_trip() {
    bc_components::register_tags();

    let patterns = vec![
        Pattern::any_digest(),
        Pattern::digest_prefix(hex::decode("deadbeef").unwrap()),
        Pattern::digest(Digest::from_image(b"test")),
    ];

    for pattern in patterns {
        let string_repr = pattern.to_string();
        let parsed_back = Pattern::parse(&string_repr).unwrap();
        assert_eq!(
            pattern, parsed_back,
            "Round trip failed for: {}",
            string_repr
        );
    }
}

#[test]
fn test_digest_pattern_errors() {
    bc_components::register_tags();

    // Test unterminated quote
    assert!(Pattern::parse("digest'unclosed").is_err());

    // Test invalid hex (odd length)
    assert!(Pattern::parse("digest'abc'").is_err());

    // Test invalid hex characters
    assert!(Pattern::parse("digest'xyz'").is_err());

    // Test invalid UR
    assert!(Pattern::parse("digest'ur:invalid/data'").is_err());

    // Test empty content
    assert!(Pattern::parse("digest''").is_err());
}

use dcbor_pattern::Pattern;

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

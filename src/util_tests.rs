use super::*;

#[test]
fn filter_input_replaces_control_delimiters_with_spaces() {
    assert_eq!(
        filter_input("a\nb\u{000b}c\u{000c}d\re\u{000e}f"),
        "a b c d e f"
    );
}

#[test]
fn null_or_empty_matches_java_trim_semantics() {
    assert!(is_null_or_empty(None));
    assert!(is_null_or_empty(Some(" \t ")));
    assert!(is_null_or_empty(Some("\u{0}\u{20}")));
    assert!(!is_null_or_empty(Some("roseau")));
    assert!(!is_null_or_empty(Some("\u{a0}")));
}

#[test]
fn ip_validation_accepts_ipv4_octets_only() {
    assert!(has_valid_ip_address("127.0.0.1"));
    assert!(has_valid_ip_address("0.0.0.0"));
    assert!(has_valid_ip_address("255.255.255.255"));
    assert!(!has_valid_ip_address("256.0.0.1"));
    assert!(!has_valid_ip_address("localhost"));
    assert!(!has_valid_ip_address("127.0.0"));
    assert!(!has_valid_ip_address("01.2.3.4"));
    assert!(!has_valid_ip_address("1.02.3.4"));
    assert!(!has_valid_ip_address("1.2.003.4"));
}

#[test]
fn rounding_matches_java_two_decimal_helper() {
    assert_eq!(round_to_two_places(1.234), 1.23);
    assert_eq!(round_to_two_places(1.235), 1.24);
    assert_eq!(round_to_two_places(-1.234), -1.23);
    assert_eq!(round_to_two_places(-1.125), -1.12);
    assert_eq!(round_to_two_places(-1.235), -1.24);
    assert_eq!(round_to_two_places(-1.236), -1.24);
    assert_eq!(display_two_place_value(1.2), "1.2");
    assert_eq!(display_two_place_value(1.0), "1");
}

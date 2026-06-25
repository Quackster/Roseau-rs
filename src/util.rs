pub fn filter_input(input: &str) -> String {
    input
        .chars()
        .map(|c| match c as u32 {
            10..=14 => ' ',
            _ => c,
        })
        .collect()
}

pub fn is_null_or_empty(value: Option<&str>) -> bool {
    value.map(is_java_trim_empty).unwrap_or(true)
}

fn is_java_trim_empty(value: &str) -> bool {
    value.trim_matches(|c| c <= '\u{20}').is_empty()
}

pub fn has_valid_ip_address(ip: &str) -> bool {
    let mut parts = ip.split('.');

    let valid = (0..4).all(|_| parts.next().is_some_and(is_valid_ipv4_octet));

    valid && parts.next().is_none()
}

fn is_valid_ipv4_octet(part: &str) -> bool {
    if part.is_empty()
        || part.len() > 3
        || (part.len() > 1 && part.starts_with('0'))
        || !part.chars().all(|c| c.is_ascii_digit())
    {
        return false;
    }

    part.parse::<u16>().is_ok_and(|value| value <= 255)
}

pub fn round_to_two_places(value: f64) -> f64 {
    ((value * 100.0) + 0.5).floor() / 100.0
}

pub fn display_two_place_value(value: f64) -> String {
    let formatted = format!("{:.2}", round_to_two_places(value));
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

#[cfg(test)]
mod tests {
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
}

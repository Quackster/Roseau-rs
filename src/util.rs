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
    value.map(|s| s.trim().is_empty()).unwrap_or(true)
}

pub fn has_valid_ip_address(ip: &str) -> bool {
    let mut parts = ip.split('.');

    let valid = (0..4).all(|_| {
        parts
            .next()
            .and_then(|part| {
                if part.is_empty() || part.len() > 3 {
                    None
                } else {
                    part.parse::<u8>().ok()
                }
            })
            .is_some()
    });

    valid && parts.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_input_replaces_control_delimiters_with_spaces() {
        assert_eq!(filter_input("a\nb\u{000b}c\u{000c}d\re\u{000e}f"), "a b c d e f");
    }

    #[test]
    fn null_or_empty_matches_java_trim_semantics() {
        assert!(is_null_or_empty(None));
        assert!(is_null_or_empty(Some(" \t ")));
        assert!(!is_null_or_empty(Some("roseau")));
    }

    #[test]
    fn ip_validation_accepts_ipv4_octets_only() {
        assert!(has_valid_ip_address("127.0.0.1"));
        assert!(has_valid_ip_address("0.0.0.0"));
        assert!(has_valid_ip_address("255.255.255.255"));
        assert!(!has_valid_ip_address("256.0.0.1"));
        assert!(!has_valid_ip_address("localhost"));
        assert!(!has_valid_ip_address("127.0.0"));
    }
}

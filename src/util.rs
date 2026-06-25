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
#[path = "util_tests.rs"]
mod tests;

use super::date_time::*;

#[test]
fn formats_epoch_with_existing_log_pattern() {
    assert_eq!(DateTime::format_millis(0), "01-01-1970 12:01:00");
    assert_eq!(
        DateTime::format_millis(1_609_459_205_000),
        "01-01-2021 12:01:05"
    );
    assert_eq!(
        DateTime::format_millis(1_614_556_861_000),
        "01-03-2021 12:03:01"
    );
}

#[test]
fn calculates_elapsed_whole_days() {
    assert_eq!(DateTime::days_elapsed_since(0, 86_400_000), 1);
    assert_eq!(DateTime::days_elapsed_since(0, 86_399_999), 0);
}

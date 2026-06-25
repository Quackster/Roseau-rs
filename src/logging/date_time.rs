use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime;

impl DateTime {
    pub fn current_time_millis() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or(0)
    }

    pub fn format_now() -> String {
        Self::format_millis(Self::current_time_millis())
    }

    pub fn format_millis(timestamp_millis: i64) -> String {
        let total_seconds = timestamp_millis.div_euclid(1000);
        let days = total_seconds.div_euclid(86_400);
        let seconds_of_day = total_seconds.rem_euclid(86_400);
        let (year, month, day) = civil_from_days(days);
        let hour_24 = seconds_of_day / 3_600;
        let hour = match hour_24 % 12 {
            0 => 12,
            value => value,
        };
        let second = seconds_of_day % 60;

        format!("{day:02}-{month:02}-{year:04} {hour:02}:{month:02}:{second:02}")
    }

    pub fn days_elapsed_since(timestamp_millis: i64, now_millis: i64) -> i64 {
        (now_millis - timestamp_millis) / 86_400_000
    }
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };

    if month <= 2 {
        year += 1;
    }

    (year, month, day)
}

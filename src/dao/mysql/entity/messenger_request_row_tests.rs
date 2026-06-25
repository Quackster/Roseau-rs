use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_messenger_request_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(4)),
        ("to_id", SqlValue::Integer(2)),
        ("from_id", SqlValue::Integer(1)),
    ]);

    assert_eq!(
        MessengerRequestRow::try_from(&row).unwrap(),
        MessengerRequestRow::new(4, 2, 1)
    );
}

#[test]
fn reports_invalid_messenger_request_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(4)),
        ("to_id", SqlValue::Integer(2)),
    ]);

    assert_eq!(
        MessengerRequestRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `from_id` as i32"
    );
}

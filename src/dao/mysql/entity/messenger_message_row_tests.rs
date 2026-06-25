use super::messenger_message_row::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_messenger_message_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(5)),
        ("from_id", SqlValue::Integer(1)),
        ("to_id", SqlValue::Integer(2)),
        ("time_sent", SqlValue::Long(1234)),
        ("message", SqlValue::Text("hello".to_owned())),
        ("unread", SqlValue::Integer(1)),
    ]);

    assert_eq!(
        MessengerMessageRow::try_from(&row).unwrap(),
        MessengerMessageRow::new(5, 1, 2, 1234, "hello", true)
    );
}

#[test]
fn reports_invalid_messenger_message_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(5)),
        ("from_id", SqlValue::Integer(1)),
        ("to_id", SqlValue::Integer(2)),
    ]);

    assert_eq!(
        MessengerMessageRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `time_sent` as i64"
    );
}

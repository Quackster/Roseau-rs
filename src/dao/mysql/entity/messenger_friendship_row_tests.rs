use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_messenger_friendship_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("sender", SqlValue::Integer(2)),
        ("receiver", SqlValue::Integer(3)),
    ]);

    assert_eq!(
        MessengerFriendshipRow::try_from(&row).unwrap(),
        MessengerFriendshipRow::new(1, 2, 3)
    );
}

#[test]
fn reports_invalid_messenger_friendship_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("sender", SqlValue::Integer(2)),
    ]);

    assert_eq!(
        MessengerFriendshipRow::try_from(&row)
            .unwrap_err()
            .message(),
        "Missing or invalid SQL column `receiver` as i32"
    );
}

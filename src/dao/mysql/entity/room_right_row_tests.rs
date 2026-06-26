use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_room_right_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("user_id", SqlValue::Integer(2)),
        ("room_id", SqlValue::Integer(3)),
    ]);

    assert_eq!(
        RoomRightRow::try_from(&row).unwrap(),
        RoomRightRow::new(1, 2, 3)
    );
}

#[test]
fn reports_invalid_room_right_columns() {
    let row = SqlRow::new([("id", SqlValue::Integer(1))]);

    assert_eq!(
        RoomRightRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `user_id` as i32"
    );
}

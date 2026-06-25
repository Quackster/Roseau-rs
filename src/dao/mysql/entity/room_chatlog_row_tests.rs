use super::room_chatlog_row::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_room_chatlog_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("user", SqlValue::Text("alice".to_owned())),
        ("room_id", SqlValue::Integer(2)),
        ("timestamp", SqlValue::Long(12345)),
        ("message_type", SqlValue::Integer(0)),
        ("message", SqlValue::Text("hello".to_owned())),
    ]);

    assert_eq!(
        RoomChatlogRow::try_from(&row).unwrap(),
        RoomChatlogRow::new(1, "alice", 2, 12345, 0, "hello")
    );
}

#[test]
fn reports_invalid_room_chatlog_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("user", SqlValue::Text("alice".to_owned())),
    ]);

    assert_eq!(
        RoomChatlogRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `room_id` as i32"
    );
}

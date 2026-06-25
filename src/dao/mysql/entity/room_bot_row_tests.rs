use super::room_bot_row::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_room_bot_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("room_id", SqlValue::Integer(2)),
        ("name", SqlValue::Text("Guide".to_owned())),
        ("figure", SqlValue::Text("hr-100".to_owned())),
        ("motto", SqlValue::Text("Welcome".to_owned())),
        ("start_x", SqlValue::Integer(3)),
        ("start_y", SqlValue::Integer(4)),
        ("start_z", SqlValue::Integer(0)),
        ("start_rotation", SqlValue::Integer(2)),
        ("walk_to", SqlValue::Text("5,6".to_owned())),
        ("messages", SqlValue::Text("hi".to_owned())),
        ("triggers", SqlValue::Text("hello".to_owned())),
        ("responses", SqlValue::Text("welcome".to_owned())),
    ]);

    assert_eq!(
        RoomBotRow::try_from(&row).unwrap(),
        RoomBotRow::new(
            1, 2, "Guide", "hr-100", "Welcome", 3, 4, 0, 2, "5,6", "hi", "hello", "welcome",
        )
    );
}

#[test]
fn reports_invalid_room_bot_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("room_id", SqlValue::Integer(2)),
    ]);

    assert_eq!(
        RoomBotRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `name` as String"
    );
}

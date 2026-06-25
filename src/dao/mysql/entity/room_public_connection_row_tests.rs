use super::room_public_connection_row::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_room_public_connection_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(10)),
        ("room_id", SqlValue::Integer(20)),
        ("to_id", SqlValue::Integer(30)),
        ("coordinates", SqlValue::Text("5,6".to_owned())),
        ("door_x", SqlValue::Integer(7)),
        ("door_y", SqlValue::Integer(8)),
        ("door_z", SqlValue::Integer(1)),
        ("door_rotation", SqlValue::Integer(2)),
    ]);

    assert_eq!(
        RoomPublicConnectionRow::try_from(&row).unwrap(),
        RoomPublicConnectionRow::new(10, 20, 30, "5,6", 7, 8, 1, 2)
    );
}

#[test]
fn reports_invalid_room_public_connection_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(10)),
        ("room_id", SqlValue::Integer(20)),
    ]);

    assert_eq!(
        RoomPublicConnectionRow::try_from(&row)
            .unwrap_err()
            .message(),
        "Missing or invalid SQL column `to_id` as i32"
    );
}

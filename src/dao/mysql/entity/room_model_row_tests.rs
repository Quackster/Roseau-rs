use super::room_model_row::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_room_model_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Text("model_a".to_owned())),
        ("door_x", SqlValue::Integer(1)),
        ("door_y", SqlValue::Integer(2)),
        ("door_z", SqlValue::Integer(0)),
        ("door_dir", SqlValue::Integer(4)),
        ("heightmap", SqlValue::Text("xxx".to_owned())),
        ("has_pool", SqlValue::Integer(1)),
        ("disable_height_check", SqlValue::Integer(0)),
    ]);

    assert_eq!(
        RoomModelRow::try_from(&row).unwrap(),
        RoomModelRow::new("model_a", 1, 2, 0, 4, "xxx", true, false)
    );
}

#[test]
fn reports_invalid_room_model_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Text("model_a".to_owned())),
        ("door_x", SqlValue::Integer(1)),
    ]);

    assert_eq!(
        RoomModelRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `door_y` as i32"
    );
}

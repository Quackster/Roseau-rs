use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_room_public_item_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(20)),
        ("model", SqlValue::Text("pool_a".to_owned())),
        ("x", SqlValue::Text("4".to_owned())),
        ("y", SqlValue::Integer(5)),
        ("z", SqlValue::Float(1.0)),
        ("rotation", SqlValue::Integer(2)),
        ("definitionid", SqlValue::Integer(8)),
        ("object", SqlValue::Text("chair".to_owned())),
        ("data", SqlValue::Text("ON".to_owned())),
    ]);

    assert_eq!(
        RoomPublicItemRow::try_from(&row).unwrap(),
        RoomPublicItemRow::new(20, "pool_a", "4", 5, 1.0, 2, 8, "chair", "ON")
    );
}

#[test]
fn builds_room_public_item_row_when_x_is_numeric_sql_value() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(20)),
        ("model", SqlValue::Text("pool_a".to_owned())),
        ("x", SqlValue::Integer(4)),
        ("y", SqlValue::Integer(5)),
        ("z", SqlValue::Float(1.0)),
        ("rotation", SqlValue::Integer(2)),
        ("definitionid", SqlValue::Integer(8)),
        ("object", SqlValue::Text("chair".to_owned())),
        ("data", SqlValue::Text("ON".to_owned())),
    ]);

    assert_eq!(
        RoomPublicItemRow::try_from(&row).unwrap(),
        RoomPublicItemRow::new(20, "pool_a", "4", 5, 1.0, 2, 8, "chair", "ON")
    );
}

#[test]
fn defaults_nullable_room_public_item_text_columns_to_empty_strings() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(20)),
        ("model", SqlValue::Text("pool_a".to_owned())),
        ("x", SqlValue::Integer(4)),
        ("y", SqlValue::Integer(5)),
        ("z", SqlValue::Float(1.0)),
        ("rotation", SqlValue::Integer(2)),
        ("definitionid", SqlValue::Integer(8)),
        ("object", SqlValue::Null),
        ("data", SqlValue::Null),
    ]);

    assert_eq!(
        RoomPublicItemRow::try_from(&row).unwrap(),
        RoomPublicItemRow::new(20, "pool_a", "4", 5, 1.0, 2, 8, "", "")
    );
}

#[test]
fn reports_invalid_room_public_item_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(20)),
        ("model", SqlValue::Text("pool_a".to_owned())),
    ]);

    assert_eq!(
        RoomPublicItemRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `x` as String"
    );
}

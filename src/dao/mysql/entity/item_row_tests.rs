use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_item_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(10)),
        ("user_id", SqlValue::Integer(3)),
        ("item_id", SqlValue::Integer(5)),
        ("room_id", SqlValue::Integer(7)),
        ("x", SqlValue::Text("1".to_owned())),
        ("y", SqlValue::Integer(2)),
        ("z", SqlValue::Float(0.5)),
        ("rotation", SqlValue::Integer(4)),
        ("extra_data", SqlValue::Text("ON".to_owned())),
    ]);

    assert_eq!(
        ItemRow::try_from(&row).unwrap(),
        ItemRow::new(10, 3, 5, 7, "1", 2, 0.5, 4, "ON")
    );
}

#[test]
fn builds_item_row_when_x_is_numeric_sql_value() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(10)),
        ("user_id", SqlValue::Integer(3)),
        ("item_id", SqlValue::Integer(5)),
        ("room_id", SqlValue::Integer(7)),
        ("x", SqlValue::Integer(1)),
        ("y", SqlValue::Integer(2)),
        ("z", SqlValue::Float(0.5)),
        ("rotation", SqlValue::Integer(4)),
        ("extra_data", SqlValue::Text("ON".to_owned())),
    ]);

    assert_eq!(
        ItemRow::try_from(&row).unwrap(),
        ItemRow::new(10, 3, 5, 7, "1", 2, 0.5, 4, "ON")
    );
}

#[test]
fn builds_item_row_from_legacy_numeric_text_and_float_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(10)),
        ("user_id", SqlValue::Integer(3)),
        ("item_id", SqlValue::Integer(5)),
        ("room_id", SqlValue::Integer(7)),
        ("x", SqlValue::Text("1".to_owned())),
        ("y", SqlValue::Text("2".to_owned())),
        ("z", SqlValue::Text("0.5".to_owned())),
        ("rotation", SqlValue::Float(4.0)),
        ("extra_data", SqlValue::Null),
    ]);

    assert_eq!(
        ItemRow::try_from(&row).unwrap(),
        ItemRow::new(10, 3, 5, 7, "1", 2, 0.5, 4, "")
    );
}

#[test]
fn reports_invalid_item_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(10)),
        ("user_id", SqlValue::Integer(3)),
    ]);

    assert_eq!(
        ItemRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `item_id` as i32"
    );
}

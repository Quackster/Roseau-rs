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

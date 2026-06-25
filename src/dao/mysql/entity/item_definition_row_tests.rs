use super::item_definition_row::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_item_definition_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(5)),
        ("sprite", SqlValue::Text("chair".to_owned())),
        ("color", SqlValue::Text("red".to_owned())),
        ("length", SqlValue::Integer(1)),
        ("width", SqlValue::Integer(2)),
        ("height", SqlValue::Float(1.5)),
        ("dataclass", SqlValue::Text("Item".to_owned())),
        ("behaviour", SqlValue::Text("chair".to_owned())),
        ("name", SqlValue::Text("Chair".to_owned())),
        ("description", SqlValue::Text("A chair".to_owned())),
    ]);

    assert_eq!(
        ItemDefinitionRow::try_from(&row).unwrap(),
        ItemDefinitionRow::new(5, "chair", "red", 1, 2, 1.5, "Item", "chair", "Chair", "A chair")
    );
}

#[test]
fn reports_invalid_item_definition_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(5)),
        ("sprite", SqlValue::Text("chair".to_owned())),
    ]);

    assert_eq!(
        ItemDefinitionRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `color` as String"
    );
}

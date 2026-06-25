use super::*;
use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_catalogue_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("definition_id", SqlValue::Integer(5)),
        ("call_id", SqlValue::Text("chair".to_owned())),
        ("credits", SqlValue::Integer(10)),
    ]);

    assert_eq!(
        CatalogueRow::try_from(&row).unwrap(),
        CatalogueRow::new(1, 5, "chair", 10)
    );
}

#[test]
fn reports_invalid_catalogue_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("definition_id", SqlValue::Integer(5)),
        ("call_id", SqlValue::Text("chair".to_owned())),
    ]);

    assert_eq!(
        CatalogueRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `credits` as i32"
    );
}

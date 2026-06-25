use super::*;
use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_catalogue_deal_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(2)),
        ("call_id", SqlValue::Text("bundle".to_owned())),
        ("products", SqlValue::Text("chair,table".to_owned())),
        ("cost", SqlValue::Integer(20)),
    ]);

    assert_eq!(
        CatalogueDealRow::try_from(&row).unwrap(),
        CatalogueDealRow::new(2, "bundle", "chair,table", 20)
    );
}

#[test]
fn reports_invalid_catalogue_deal_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(2)),
        ("call_id", SqlValue::Text("bundle".to_owned())),
        ("products", SqlValue::Text("chair,table".to_owned())),
    ]);

    assert_eq!(
        CatalogueDealRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `cost` as i32"
    );
}

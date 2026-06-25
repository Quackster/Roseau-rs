use super::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_user_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(7)),
        ("username", SqlValue::Text("alice".to_owned())),
        ("password", SqlValue::Text("hash".to_owned())),
        ("rank", SqlValue::Integer(4)),
        ("mission", SqlValue::Text("hello".to_owned())),
        ("figure", SqlValue::Text("hd-100".to_owned())),
        ("pool_figure", SqlValue::Text("pool".to_owned())),
        ("email", SqlValue::Text("alice@example.test".to_owned())),
        ("credits", SqlValue::Integer(55)),
        ("sex", SqlValue::Text("F".to_owned())),
        ("country", SqlValue::Text("UK".to_owned())),
        ("badge", SqlValue::Text("ADM".to_owned())),
        ("birthday", SqlValue::Text("1990-01-01".to_owned())),
        ("join_date", SqlValue::Long(1000)),
        ("last_online", SqlValue::Long(2000)),
        ("personal_greeting", SqlValue::Text("welcome".to_owned())),
        ("tickets", SqlValue::Integer(8)),
    ]);

    assert_eq!(
        UserRow::try_from(&row).unwrap(),
        UserRow::new(
            7,
            "alice",
            "hash",
            4,
            "hello",
            "hd-100",
            "pool",
            "alice@example.test",
            55,
            "F",
            "UK",
            "ADM",
            "1990-01-01",
            1000,
            2000,
            "welcome",
            8,
        )
    );
}

#[test]
fn reports_invalid_user_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(7)),
        ("username", SqlValue::Text("alice".to_owned())),
    ]);

    assert_eq!(
        UserRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `password` as String"
    );
}

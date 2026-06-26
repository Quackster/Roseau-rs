use super::*;
use crate::dao::mysql::DatabaseEngine;

#[test]
fn converts_sql_parameters_to_mysql_params() {
    assert_eq!(positional_params(&[]), Params::Empty);
    assert_eq!(
        positional_params(&[
            SqlParameter::Null,
            SqlParameter::Bool(true),
            SqlParameter::Integer(7),
            SqlParameter::Long(8),
            SqlParameter::Float(1.5),
            SqlParameter::Text("alice".to_owned()),
        ]),
        Params::Positional(vec![
            Value::NULL,
            Value::Int(1),
            Value::Int(7),
            Value::Int(8),
            Value::Double(1.5),
            Value::Bytes(b"alice".to_vec()),
        ])
    );
}

#[test]
fn converts_mysql_values_to_driver_neutral_values() {
    assert_eq!(sql_value_from_mysql_value(&Value::NULL), SqlValue::Null);
    assert_eq!(
        sql_value_from_mysql_value(&Value::Bytes(b"alice".to_vec())),
        SqlValue::Text("alice".to_owned())
    );
    assert_eq!(
        sql_value_from_mysql_value(&Value::Int(7)),
        SqlValue::Integer(7)
    );
    assert_eq!(
        sql_value_from_mysql_value(&Value::Int(i64::from(i32::MAX) + 1)),
        SqlValue::Long(i64::from(i32::MAX) + 1)
    );
    assert_eq!(
        sql_value_from_mysql_value(&Value::UInt(u64::MAX)),
        SqlValue::Text(u64::MAX.to_string())
    );
    assert_eq!(
        sql_value_from_mysql_value(&Value::Float(1.25)),
        SqlValue::Float(1.25)
    );
    assert_eq!(
        sql_value_from_mysql_value(&Value::Double(1.5)),
        SqlValue::Float(1.5)
    );
    assert_eq!(
        sql_value_from_mysql_value(&Value::Date(2024, 5, 6, 7, 8, 9, 10)),
        SqlValue::Text("2024-05-06 07:08:09.000010".to_owned())
    );
    assert_eq!(
        sql_value_from_mysql_value(&Value::Time(true, 1, 2, 3, 4, 5)),
        SqlValue::Text("-1 02:03:04.000005".to_owned())
    );
}

#[test]
fn rejects_non_mysql_storage_urls() {
    let storage = Storage::new(
        DatabaseEngine::Postgres,
        "db",
        5432,
        "roseau",
        "secret",
        "hotel",
        "",
        "",
    );

    assert_eq!(
        MySqlDriver::connect_storage(&storage)
            .unwrap_err()
            .message(),
        "MySQL driver cannot connect postgres storage"
    );
}

use super::*;

#[test]
fn reads_required_typed_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(7)),
        ("name", SqlValue::Text("alice".to_owned())),
        ("last_online", SqlValue::Long(1234)),
        ("height", SqlValue::Float(1.25)),
        ("enabled", SqlValue::Bool(true)),
        ("hidden", SqlValue::Integer(0)),
    ]);

    assert_eq!(row.required_i32("id").unwrap(), 7);
    assert_eq!(row.required_i64("last_online").unwrap(), 1234);
    assert_eq!(row.required_f64("height").unwrap(), 1.25);
    assert_eq!(row.required_string("name").unwrap(), "alice");
    assert!(row.required_bool("enabled").unwrap());
    assert!(!row.required_bool("hidden").unwrap());
}

#[test]
fn reads_legacy_numeric_columns_with_compatible_accessors() {
    let row = SqlRow::new([
        ("text_i32", SqlValue::Text("7".to_owned())),
        ("float_i32", SqlValue::Float(4.0)),
        ("text_f64", SqlValue::Text("1.25".to_owned())),
        ("int_f64", SqlValue::Integer(2)),
    ]);

    assert_eq!(row.required_i32_compatible("text_i32").unwrap(), 7);
    assert_eq!(row.required_i32_compatible("float_i32").unwrap(), 4);
    assert_eq!(row.required_f64_compatible("text_f64").unwrap(), 1.25);
    assert_eq!(row.required_f64_compatible("int_f64").unwrap(), 2.0);
}

#[test]
fn reports_missing_or_invalid_columns() {
    let row = SqlRow::new([("id", SqlValue::Text("not a number".to_owned()))]);

    assert_eq!(
        row.required_i32("id").unwrap_err().message(),
        "Missing or invalid SQL column `id` as i32"
    );
    assert_eq!(
        row.required_string("name").unwrap_err().message(),
        "Missing or invalid SQL column `name` as String"
    );
    assert_eq!(
        row.required_bool("name").unwrap_err().message(),
        "Missing or invalid SQL column `name` as bool"
    );
}

#[test]
fn reads_nullable_typed_columns() {
    let row = SqlRow::new([
        ("id", SqlValue::Null),
        ("owner_id", SqlValue::Integer(7)),
        ("height", SqlValue::Float(1.5)),
        ("name", SqlValue::Text("alice".to_owned())),
        ("enabled", SqlValue::Long(1)),
        ("invalid", SqlValue::Text("not a number".to_owned())),
    ]);

    assert_eq!(row.optional_i32("id").unwrap(), None);
    assert_eq!(row.optional_i32("owner_id").unwrap(), Some(7));
    assert_eq!(row.optional_i64("owner_id").unwrap(), Some(7));
    assert_eq!(row.optional_f64("height").unwrap(), Some(1.5));
    assert_eq!(
        row.optional_string("name").unwrap(),
        Some("alice".to_owned())
    );
    assert_eq!(row.optional_bool("enabled").unwrap(), Some(true));
    assert_eq!(
        row.optional_i32("invalid").unwrap_err().message(),
        "Missing or invalid SQL column `invalid` as i32"
    );
    assert_eq!(
        row.optional_string("missing").unwrap_err().message(),
        "Missing or invalid SQL column `missing` as String"
    );
}

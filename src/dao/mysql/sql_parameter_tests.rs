use super::*;

#[test]
fn exposes_typed_sql_parameters() {
    assert_eq!(SqlParameter::from(7).as_i32(), Some(7));
    assert_eq!(SqlParameter::from(7).as_i64(), Some(7));
    assert_eq!(SqlParameter::from(7_i64).as_i64(), Some(7));
    assert_eq!(SqlParameter::from(1.5).as_f64(), Some(1.5));
    assert_eq!(SqlParameter::from("alice").as_str(), Some("alice"));
    assert_eq!(SqlParameter::from(true).as_bool(), Some(true));
    assert_eq!(SqlParameter::from(1).as_bool(), Some(true));
    assert_eq!(SqlParameter::from(0).as_bool(), Some(false));
    assert!(SqlParameter::Null.is_null());
}

#[test]
fn converts_parameters_to_driver_neutral_sql_values() {
    assert_eq!(SqlParameter::Null.to_value(), SqlValue::Null);
    assert_eq!(SqlParameter::from(true).to_value(), SqlValue::Bool(true));
    assert_eq!(SqlParameter::from(7).to_value(), SqlValue::Integer(7));
    assert_eq!(SqlParameter::from(7_i64).to_value(), SqlValue::Long(7));
    assert_eq!(SqlParameter::from(1.5).to_value(), SqlValue::Float(1.5));
    assert_eq!(
        SqlParameter::from("alice").to_value(),
        SqlValue::Text("alice".to_owned())
    );
}

#[test]
fn redacts_text_parameters_for_diagnostics() {
    assert_eq!(SqlParameter::Null.redacted_display(), "NULL");
    assert_eq!(SqlParameter::from(true).redacted_display(), "true");
    assert_eq!(SqlParameter::from(7).redacted_display(), "7");
    assert_eq!(SqlParameter::from(7_i64).redacted_display(), "7");
    assert_eq!(SqlParameter::from(1.5).redacted_display(), "1.5");
    assert_eq!(SqlParameter::from("secret").redacted_display(), "<text>");
}

#[test]
fn converts_optional_values_to_nullable_parameters() {
    assert_eq!(
        SqlParameter::from(Some("hello")),
        SqlParameter::Text("hello".to_owned())
    );
    assert_eq!(SqlParameter::from(Some(7)), SqlParameter::Integer(7));
    assert_eq!(SqlParameter::from(Option::<i32>::None), SqlParameter::Null);
}

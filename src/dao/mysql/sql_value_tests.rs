use super::*;

#[test]
fn exposes_typed_sql_values() {
    assert_eq!(SqlValue::from(7).as_i32(), Some(7));
    assert_eq!(SqlValue::from(7).as_i64(), Some(7));
    assert_eq!(SqlValue::from(7_i64).as_i64(), Some(7));
    assert_eq!(SqlValue::from(1.5).as_f64(), Some(1.5));
    assert_eq!(SqlValue::from("alice").as_str(), Some("alice"));
    assert_eq!(SqlValue::from(true).as_bool(), Some(true));
    assert_eq!(SqlValue::from(1).as_bool(), Some(true));
    assert_eq!(SqlValue::from(0).as_bool(), Some(false));
    assert!(SqlValue::Null.is_null());
}

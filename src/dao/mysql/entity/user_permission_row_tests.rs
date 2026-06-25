use super::user_permission_row::*;
use crate::dao::mysql::SqlValue;

#[test]
fn builds_user_permission_row_from_sql_row() {
    let row = SqlRow::new([
        ("id", SqlValue::Integer(3)),
        ("rank", SqlValue::Integer(4)),
        ("permission", SqlValue::Text("room_admin".to_owned())),
        ("inheritable", SqlValue::Integer(1)),
    ]);

    assert_eq!(
        UserPermissionRow::try_from(&row).unwrap(),
        UserPermissionRow::new(3, 4, "room_admin", true)
    );
}

#[test]
fn reports_invalid_user_permission_columns() {
    let row = SqlRow::new([("id", SqlValue::Integer(3)), ("rank", SqlValue::Integer(4))]);

    assert_eq!(
        UserPermissionRow::try_from(&row).unwrap_err().message(),
        "Missing or invalid SQL column `permission` as String"
    );
}

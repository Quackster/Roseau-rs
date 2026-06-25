use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPermissionRow {
    pub id: i32,
    pub rank: i32,
    pub permission: String,
    pub inheritable: bool,
}

impl UserPermissionRow {
    pub const TABLE: &'static str = "users_permissions";

    pub fn new(id: i32, rank: i32, permission: impl Into<String>, inheritable: bool) -> Self {
        Self {
            id,
            rank,
            permission: permission.into(),
            inheritable,
        }
    }
}

impl TryFrom<&SqlRow> for UserPermissionRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("rank")?,
            row.required_string("permission")?,
            row.required_bool("inheritable")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}

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
#[path = "user_permission_row_tests.rs"]
mod tests;

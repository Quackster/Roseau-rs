use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomRightRow {
    pub id: i32,
    pub user_id: i32,
    pub room_id: i32,
}

impl RoomRightRow {
    pub const TABLE: &'static str = "room_rights";

    pub const fn new(id: i32, user_id: i32, room_id: i32) -> Self {
        Self {
            id,
            user_id,
            room_id,
        }
    }
}

impl TryFrom<&SqlRow> for RoomRightRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("user_id")?,
            row.required_i32("room_id")?,
        ))
    }
}

#[cfg(test)]
#[path = "room_right_row_tests.rs"]
mod tests;

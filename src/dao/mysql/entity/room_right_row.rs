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
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_room_right_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("user_id", SqlValue::Integer(2)),
            ("room_id", SqlValue::Integer(3)),
        ]);

        assert_eq!(
            RoomRightRow::try_from(&row).unwrap(),
            RoomRightRow::new(1, 2, 3)
        );
    }

    #[test]
    fn reports_invalid_room_right_columns() {
        let row = SqlRow::new([("id", SqlValue::Integer(1))]);

        assert_eq!(
            RoomRightRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `user_id` as i32"
        );
    }
}

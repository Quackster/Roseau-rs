use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomChatlogRow {
    pub id: i32,
    pub user: String,
    pub room_id: i32,
    pub timestamp: i64,
    pub message_type: i32,
    pub message: String,
}

impl RoomChatlogRow {
    pub const TABLE: &'static str = "room_chatlogs";

    pub fn new(
        id: i32,
        user: impl Into<String>,
        room_id: i32,
        timestamp: i64,
        message_type: i32,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            user: user.into(),
            room_id,
            timestamp,
            message_type,
            message: message.into(),
        }
    }
}

impl TryFrom<&SqlRow> for RoomChatlogRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_string("user")?,
            row.required_i32("room_id")?,
            row.required_i64("timestamp")?,
            row.required_i32("message_type")?,
            row.required_string("message")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_room_chatlog_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("user", SqlValue::Text("alice".to_owned())),
            ("room_id", SqlValue::Integer(2)),
            ("timestamp", SqlValue::Long(12345)),
            ("message_type", SqlValue::Integer(0)),
            ("message", SqlValue::Text("hello".to_owned())),
        ]);

        assert_eq!(
            RoomChatlogRow::try_from(&row).unwrap(),
            RoomChatlogRow::new(1, "alice", 2, 12345, 0, "hello")
        );
    }

    #[test]
    fn reports_invalid_room_chatlog_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("user", SqlValue::Text("alice".to_owned())),
        ]);

        assert_eq!(
            RoomChatlogRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `room_id` as i32"
        );
    }
}

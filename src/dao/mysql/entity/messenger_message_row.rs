use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessengerMessageRow {
    pub id: i32,
    pub from_id: i32,
    pub to_id: i32,
    pub time_sent: i64,
    pub message: String,
    pub unread: bool,
}

impl MessengerMessageRow {
    pub const TABLE: &'static str = "messenger_messages";

    pub fn new(
        id: i32,
        from_id: i32,
        to_id: i32,
        time_sent: i64,
        message: impl Into<String>,
        unread: bool,
    ) -> Self {
        Self {
            id,
            from_id,
            to_id,
            time_sent,
            message: message.into(),
            unread,
        }
    }
}

impl TryFrom<&SqlRow> for MessengerMessageRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("from_id")?,
            row.required_i32("to_id")?,
            row.required_i64("time_sent")?,
            row.required_string("message")?,
            row.required_bool("unread")?,
        ))
    }
}

#[cfg(test)]
#[path = "messenger_message_row_tests.rs"]
mod tests;

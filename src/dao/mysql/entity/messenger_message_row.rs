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
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_messenger_message_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(5)),
            ("from_id", SqlValue::Integer(1)),
            ("to_id", SqlValue::Integer(2)),
            ("time_sent", SqlValue::Long(1234)),
            ("message", SqlValue::Text("hello".to_owned())),
            ("unread", SqlValue::Integer(1)),
        ]);

        assert_eq!(
            MessengerMessageRow::try_from(&row).unwrap(),
            MessengerMessageRow::new(5, 1, 2, 1234, "hello", true)
        );
    }

    #[test]
    fn reports_invalid_messenger_message_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(5)),
            ("from_id", SqlValue::Integer(1)),
            ("to_id", SqlValue::Integer(2)),
        ]);

        assert_eq!(
            MessengerMessageRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `time_sent` as i64"
        );
    }
}

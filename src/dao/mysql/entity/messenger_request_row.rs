use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessengerRequestRow {
    pub id: i32,
    pub to_id: i32,
    pub from_id: i32,
}

impl MessengerRequestRow {
    pub const TABLE: &'static str = "messenger_requests";

    pub const fn new(id: i32, to_id: i32, from_id: i32) -> Self {
        Self { id, to_id, from_id }
    }
}

impl TryFrom<&SqlRow> for MessengerRequestRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("to_id")?,
            row.required_i32("from_id")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_messenger_request_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(4)),
            ("to_id", SqlValue::Integer(2)),
            ("from_id", SqlValue::Integer(1)),
        ]);

        assert_eq!(
            MessengerRequestRow::try_from(&row).unwrap(),
            MessengerRequestRow::new(4, 2, 1)
        );
    }

    #[test]
    fn reports_invalid_messenger_request_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(4)),
            ("to_id", SqlValue::Integer(2)),
        ]);

        assert_eq!(
            MessengerRequestRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `from_id` as i32"
        );
    }
}

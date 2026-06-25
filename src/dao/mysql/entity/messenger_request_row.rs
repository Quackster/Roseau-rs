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
#[path = "messenger_request_row_tests.rs"]
mod tests;

use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueRow {
    pub id: i32,
    pub definition_id: i32,
    pub call_id: String,
    pub credits: i32,
}

impl CatalogueRow {
    pub const TABLE: &'static str = "catalogue";

    pub fn new(id: i32, definition_id: i32, call_id: impl Into<String>, credits: i32) -> Self {
        Self {
            id,
            definition_id,
            call_id: call_id.into(),
            credits,
        }
    }
}

impl TryFrom<&SqlRow> for CatalogueRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("definition_id")?,
            row.required_string("call_id")?,
            row.required_i32("credits")?,
        ))
    }
}

#[cfg(test)]
#[path = "catalogue_row_tests.rs"]
mod tests;

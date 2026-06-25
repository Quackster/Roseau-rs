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
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_catalogue_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("definition_id", SqlValue::Integer(5)),
            ("call_id", SqlValue::Text("chair".to_owned())),
            ("credits", SqlValue::Integer(10)),
        ]);

        assert_eq!(
            CatalogueRow::try_from(&row).unwrap(),
            CatalogueRow::new(1, 5, "chair", 10)
        );
    }

    #[test]
    fn reports_invalid_catalogue_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(1)),
            ("definition_id", SqlValue::Integer(5)),
            ("call_id", SqlValue::Text("chair".to_owned())),
        ]);

        assert_eq!(
            CatalogueRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `credits` as i32"
        );
    }
}

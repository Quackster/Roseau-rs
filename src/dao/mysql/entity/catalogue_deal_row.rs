use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueDealRow {
    pub id: i32,
    pub call_id: String,
    pub products: String,
    pub cost: i32,
}

impl CatalogueDealRow {
    pub const TABLE: &'static str = "catalogue_deals";

    pub fn new(
        id: i32,
        call_id: impl Into<String>,
        products: impl Into<String>,
        cost: i32,
    ) -> Self {
        Self {
            id,
            call_id: call_id.into(),
            products: products.into(),
            cost,
        }
    }
}

impl TryFrom<&SqlRow> for CatalogueDealRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_string("call_id")?,
            row.required_string("products")?,
            row.required_i32("cost")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_catalogue_deal_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(2)),
            ("call_id", SqlValue::Text("bundle".to_owned())),
            ("products", SqlValue::Text("chair,table".to_owned())),
            ("cost", SqlValue::Integer(20)),
        ]);

        assert_eq!(
            CatalogueDealRow::try_from(&row).unwrap(),
            CatalogueDealRow::new(2, "bundle", "chair,table", 20)
        );
    }

    #[test]
    fn reports_invalid_catalogue_deal_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(2)),
            ("call_id", SqlValue::Text("bundle".to_owned())),
            ("products", SqlValue::Text("chair,table".to_owned())),
        ]);

        assert_eq!(
            CatalogueDealRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `cost` as i32"
        );
    }
}

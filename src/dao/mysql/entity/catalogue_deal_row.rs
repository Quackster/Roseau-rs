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
#[path = "catalogue_deal_row_tests.rs"]
mod tests;

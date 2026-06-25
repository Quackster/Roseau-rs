use crate::dao::mysql::entity::{CatalogueDealRow, CatalogueRow};
use crate::dao::mysql::SqlQuery;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogueQueries;

impl CatalogueQueries {
    pub fn buyable_items() -> SqlQuery {
        SqlQuery::select_all(CatalogueRow::TABLE)
    }

    pub fn item_deals() -> SqlQuery {
        SqlQuery::select_all(CatalogueDealRow::TABLE)
    }
}

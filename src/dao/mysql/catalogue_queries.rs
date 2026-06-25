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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_java_catalogue_table_reads() {
        assert_eq!(
            CatalogueQueries::buyable_items().sql(),
            "SELECT * FROM catalogue"
        );
        assert_eq!(
            CatalogueQueries::item_deals().sql(),
            "SELECT * FROM catalogue_deals"
        );
        assert!(CatalogueQueries::buyable_items().parameters().is_empty());
    }
}

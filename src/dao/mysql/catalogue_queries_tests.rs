use super::catalogue_queries::*;

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

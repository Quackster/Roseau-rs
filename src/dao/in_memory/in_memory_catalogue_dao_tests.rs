use super::*;
use super::*;

#[test]
fn stores_catalogue_items_and_deals_by_call_id() {
    let dao = InMemoryCatalogueDao::new();
    dao.insert_item(CatalogueItem::new("chair", 5, 10));
    dao.insert_deal(CatalogueDeal::new("bundle", ["chair"], 8));

    assert_eq!(
        dao.buyable_items().unwrap().get("chair").unwrap().credits(),
        10
    );
    assert_eq!(dao.item_deals().unwrap().get("bundle").unwrap().cost(), 8);
}

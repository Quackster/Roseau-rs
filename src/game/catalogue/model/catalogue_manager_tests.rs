use super::catalogue_manager::*;

#[test]
fn finds_catalogue_items_and_deals_by_call_id() {
    let item = CatalogueItem::new("chair", 2, 5);
    let deal = CatalogueDeal::new("bundle", ["chair"], 4);
    let manager = CatalogueManager::with_items_and_deals([item], [deal]);

    assert_eq!(manager.get_item_by_call("chair").unwrap().credits(), 5);
    assert_eq!(manager.get_deal_by_call("bundle").unwrap().cost(), 4);
    assert!(manager.get_item_by_call("missing").is_none());
}

#[test]
fn resolves_items_for_known_deal() {
    let item = CatalogueItem::new("chair", 2, 5);
    let deal = CatalogueDeal::new("bundle", ["chair|green"], 4);
    let manager = CatalogueManager::with_items_and_deals([item], [deal]);
    let resolved = manager.resolve_deal_items("bundle").unwrap().unwrap();

    assert_eq!(resolved[0].extra_data(), Some("green"));
}

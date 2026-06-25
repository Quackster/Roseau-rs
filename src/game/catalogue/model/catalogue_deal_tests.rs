use super::catalogue_deal::*;

#[test]
fn resolves_deal_items_and_applies_extra_data() {
    let mut items = HashMap::new();
    items.insert("poster".to_owned(), CatalogueItem::new("poster", 1, 2));
    items.insert("chair".to_owned(), CatalogueItem::new("chair", 2, 5));
    let deal = CatalogueDeal::new("bundle", ["poster|red", "chair"], 6);

    let resolved = deal.resolve_items(&items).unwrap();

    assert_eq!(resolved[0].call_id(), "poster");
    assert_eq!(resolved[0].extra_data(), Some("red"));
    assert_eq!(resolved[1].call_id(), "chair");
    assert_eq!(resolved[1].extra_data(), None);
}

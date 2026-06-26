use super::*;

#[test]
fn stores_catalogue_item_fields() {
    let item = CatalogueItem::new("chair_blue", 7, 3).with_extra_data("blue");

    assert_eq!(item.call_id(), "chair_blue");
    assert_eq!(item.definition_id(), 7);
    assert_eq!(item.credits(), 3);
    assert_eq!(item.extra_data(), Some("blue"));
}

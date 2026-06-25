use super::purchase_add_strip_item::*;

#[test]
fn composes_purchase_add_strip_item_packet() {
    let mut response = PurchaseAddStripItem.compose();

    assert_eq!(response.get(), "#ADDSTRIPITEM##");
}

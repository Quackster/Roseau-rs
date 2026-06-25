use super::purchase_ok::*;

#[test]
fn composes_purchase_ok_packet() {
    let mut response = PurchaseOk.compose();

    assert_eq!(response.get(), "#PURCHASE_OK##");
}

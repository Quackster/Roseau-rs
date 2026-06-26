use super::*;

#[test]
fn maps_successful_purchase_to_add_strip_item_packet() {
    let effects = CataloguePurchaseNetworkPlan::plan(&CataloguePurchaseOutcome::AddedStripItem, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#ADDSTRIPITEM##".to_owned(),
        }]
    );
}

#[test]
fn maps_insufficient_credits_to_system_broadcast_packet() {
    let effects =
        CataloguePurchaseNetworkPlan::plan(&CataloguePurchaseOutcome::NotEnoughCredits, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#SYSTEMBROADCAST\rYou don't have enough credits to purchase this item!##"
                .to_owned(),
        }]
    );
}

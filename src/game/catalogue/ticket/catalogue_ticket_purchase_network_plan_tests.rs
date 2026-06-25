use super::catalogue_ticket_purchase_network_plan::*;

#[test]
fn maps_self_purchase_to_buyer_alert() {
    let effects = CatalogueTicketPurchaseNetworkPlan::plan(
        &CatalogueTicketPurchaseOutcome::BoughtForSelf,
        42,
        None,
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#SYSTEMBROADCAST\rYou have bought 10 game tickets!##".to_owned(),
        }]
    );
}

#[test]
fn maps_cross_player_purchase_to_buyer_and_target_alerts() {
    let effects = CatalogueTicketPurchaseNetworkPlan::plan(
        &CatalogueTicketPurchaseOutcome::BoughtForOther {
            buyer_username: "alice".to_owned(),
            target_username: "bob".to_owned(),
        },
        42,
        Some(77),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#SYSTEMBROADCAST\rYou have bought 10 game tickets for bob##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 77,
                packet: "#SYSTEMBROADCAST\ralice has bought 10 game tickets for you!##".to_owned(),
            },
        ]
    );
}

#[test]
fn omits_target_alert_when_target_connection_is_missing() {
    let effects = CatalogueTicketPurchaseNetworkPlan::plan(
        &CatalogueTicketPurchaseOutcome::BoughtForOther {
            buyer_username: "alice".to_owned(),
            target_username: "bob".to_owned(),
        },
        42,
        None,
    );

    assert_eq!(effects.len(), 1);
}

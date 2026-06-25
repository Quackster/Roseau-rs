use super::catalogue_ticket_purchase_outcome::*;
use crate::messages::OutgoingMessage;

#[test]
fn builds_insufficient_credit_alert() {
    let mut alert = CatalogueTicketPurchaseOutcome::InsufficientCredits
        .buyer_alert()
        .compose();

    assert_eq!(
        alert.get(),
        "#SYSTEMBROADCAST\rSorry, but you do not have enough Credits to purchase this.##"
    );
}

#[test]
fn builds_missing_target_alert() {
    let mut alert = CatalogueTicketPurchaseOutcome::MissingTarget {
        target_username: "bob".to_owned(),
    }
    .buyer_alert()
    .compose();

    assert_eq!(
        alert.get(),
        "#SYSTEMBROADCAST\rThe player 'bob' cannot be found.##"
    );
}

#[test]
fn builds_self_purchase_alert() {
    let outcome = CatalogueTicketPurchaseOutcome::BoughtForSelf;

    assert_eq!(
        outcome.buyer_alert().compose().get(),
        "#SYSTEMBROADCAST\rYou have bought 10 game tickets!##"
    );
    assert!(outcome.target_alert().is_none());
}

#[test]
fn builds_cross_player_purchase_alerts() {
    let outcome = CatalogueTicketPurchaseOutcome::BoughtForOther {
        buyer_username: "alice".to_owned(),
        target_username: "bob".to_owned(),
    };

    assert_eq!(
        outcome.buyer_alert().compose().get(),
        "#SYSTEMBROADCAST\rYou have bought 10 game tickets for bob##"
    );
    assert_eq!(
        outcome.target_alert().unwrap().compose().get(),
        "#SYSTEMBROADCAST\ralice has bought 10 game tickets for you!##"
    );
}

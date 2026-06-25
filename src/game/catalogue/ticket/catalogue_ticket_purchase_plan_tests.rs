use super::*;

#[test]
fn rejects_non_ticket_purchase_calls() {
    assert_eq!(CatalogueTicketPurchasePlan::resolve("chair", 50), None);
}

#[test]
fn rejects_ticket_purchase_below_java_credit_threshold() {
    assert_eq!(
        CatalogueTicketPurchasePlan::resolve("x hyppy alice", 9),
        None
    );
}

#[test]
fn resolves_ticket_purchase_target_and_amounts() {
    let plan = CatalogueTicketPurchasePlan::resolve("x hyppy alice", 10).unwrap();

    assert_eq!(plan.target_username(), "alice");
    assert_eq!(plan.credited_tickets(), 10);
    assert_eq!(plan.charged_credits(), 5);
}

#[test]
fn filters_ticket_purchase_target_like_java_input() {
    let plan = CatalogueTicketPurchasePlan::resolve("x hyppy al\nice", 10).unwrap();

    assert_eq!(plan.target_username(), "al ice");
}

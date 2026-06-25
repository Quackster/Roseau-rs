use super::catalogue_purchase_queries::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};
use crate::game::catalogue::{
    CataloguePurchaseItemPlan, CataloguePurchasePlan, CatalogueTicketPurchasePlan,
};

#[test]
fn maps_purchase_plan_to_inventory_insert_and_credit_update_plans() {
    let purchase = CataloguePurchasePlan::new(5, [CataloguePurchaseItemPlan::new(7, "red", false)]);
    let plans = CataloguePurchaseQueries::plan(&purchase, 42, 20).unwrap();

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(
        plans[0].sql(),
        "INSERT INTO items (user_id, item_id, room_id, x, extra_data) VALUES (?, ?, ?, ?, ?)"
    );
    assert_eq!(
        plans[0].parameters(),
        &[
            SqlParameter::Integer(42),
            SqlParameter::Integer(7),
            SqlParameter::Integer(0),
            SqlParameter::Text("0".to_owned()),
            SqlParameter::Text("red".to_owned()),
        ]
    );
    assert_eq!(plans[1].kind(), SqlExecutionKind::Execute);
    assert_eq!(plans[1].sql(), "UPDATE users SET credits = ? WHERE id = ?");
    assert_eq!(
        plans[1].parameters(),
        &[SqlParameter::Integer(15), SqlParameter::Integer(42)]
    );
}

#[test]
fn maps_deal_purchase_items_before_credit_update() {
    let purchase = CataloguePurchasePlan::new(
        6,
        [
            CataloguePurchaseItemPlan::new(7, "", false),
            CataloguePurchaseItemPlan::new(8, "green", false),
        ],
    );
    let plans = CataloguePurchaseQueries::plan(&purchase, 42, 20).unwrap();

    assert_eq!(plans.len(), 3);
    assert_eq!(plans[0].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(plans[1].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(plans[2].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[1].parameters(),
        &[
            SqlParameter::Integer(42),
            SqlParameter::Integer(8),
            SqlParameter::Integer(0),
            SqlParameter::Text("0".to_owned()),
            SqlParameter::Text("green".to_owned()),
        ]
    );
    assert_eq!(
        plans[2].parameters(),
        &[SqlParameter::Integer(14), SqlParameter::Integer(42)]
    );
}

#[test]
fn defers_teleporter_pair_purchases_until_generated_ids_can_be_linked() {
    let purchase = CataloguePurchasePlan::new(5, [CataloguePurchaseItemPlan::new(7, "", true)]);

    assert_eq!(CataloguePurchaseQueries::plan(&purchase, 42, 20), None);
}

#[test]
fn maps_teleporter_pair_purchase_to_two_insert_returning_id_plans() {
    let purchase = CataloguePurchasePlan::new(5, [CataloguePurchaseItemPlan::new(7, "", true)]);
    let plans = CataloguePurchaseQueries::teleporter_pair_insert_plans(&purchase, 42).unwrap();

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(plans[1].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(
        plans[0].parameters(),
        &[
            SqlParameter::Integer(42),
            SqlParameter::Integer(7),
            SqlParameter::Integer(0),
            SqlParameter::Text("0".to_owned()),
            SqlParameter::Text(String::new()),
        ]
    );
    assert_eq!(plans[0].parameters(), plans[1].parameters());
}

#[test]
fn maps_generated_teleporter_ids_to_cross_link_updates_and_credit_update() {
    let purchase = CataloguePurchasePlan::new(5, [CataloguePurchaseItemPlan::new(7, "", true)]);
    let plans = CataloguePurchaseQueries::teleporter_pair_link_plans(100, 101, &purchase, 42, 20);

    assert_eq!(plans.len(), 3);
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[0].sql(),
        "UPDATE items SET extra_data = ? WHERE id = ?"
    );
    assert_eq!(
        plans[0].parameters(),
        &[
            SqlParameter::Text("101".to_owned()),
            SqlParameter::Integer(100)
        ]
    );
    assert_eq!(
        plans[1].parameters(),
        &[
            SqlParameter::Text("100".to_owned()),
            SqlParameter::Integer(101)
        ]
    );
    assert_eq!(plans[2].sql(), "UPDATE users SET credits = ? WHERE id = ?");
    assert_eq!(
        plans[2].parameters(),
        &[SqlParameter::Integer(15), SqlParameter::Integer(42)]
    );
}

#[test]
fn maps_ticket_purchase_to_target_ticket_and_buyer_credit_updates() {
    let purchase = CatalogueTicketPurchasePlan::resolve("x hyppy bob", 10).unwrap();
    let plans = CataloguePurchaseQueries::ticket_purchase_plans(&purchase, 42, 20, 84, 3);

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(plans[0].sql(), "UPDATE users SET tickets = ? WHERE id = ?");
    assert_eq!(
        plans[0].parameters(),
        &[SqlParameter::Integer(13), SqlParameter::Integer(84)]
    );
    assert_eq!(plans[1].sql(), "UPDATE users SET credits = ? WHERE id = ?");
    assert_eq!(
        plans[1].parameters(),
        &[SqlParameter::Integer(15), SqlParameter::Integer(42)]
    );
}

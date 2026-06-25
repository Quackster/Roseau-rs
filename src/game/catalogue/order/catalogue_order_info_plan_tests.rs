use super::catalogue_order_info_plan::*;
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

fn manager() -> CatalogueManager {
    CatalogueManager::with_items_and_deals(
        [
            CatalogueItem::new("chair", 2, 5),
            CatalogueItem::new("poster", 3, 4),
        ],
        [CatalogueDeal::new("bundle", ["chair", "poster"], 7)],
    )
}

#[test]
fn resolves_deal_order_info_before_item_order_info() {
    let plan = CatalogueOrderInfoPlan::resolve(&manager(), "bundle", None).unwrap();

    assert_eq!(plan.call_id(), "bundle");
    assert_eq!(plan.credits(), 7);
}

#[test]
fn resolves_item_order_info_and_strips_player_name() {
    let plan = CatalogueOrderInfoPlan::resolve(&manager(), "/chair alice", Some("alice")).unwrap();

    assert_eq!(plan.call_id(), "chair");
    assert_eq!(plan.credits(), 5);
}

#[test]
fn preserves_decoration_extra_data_in_order_name() {
    let plan = CatalogueOrderInfoPlan::resolve(&manager(), "poster L red", None).unwrap();

    assert_eq!(plan.call_id(), "poster L");
    assert_eq!(plan.credits(), 4);
}

#[test]
fn preserves_java_literal_space_split_for_decoration_order_info() {
    let plan = CatalogueOrderInfoPlan::resolve(&manager(), "poster  L red", None).unwrap();

    assert_eq!(plan.call_id(), "poster ");
}

#[test]
fn ignores_unknown_order_info_calls() {
    assert_eq!(
        CatalogueOrderInfoPlan::resolve(&manager(), "missing", None),
        None
    );
}

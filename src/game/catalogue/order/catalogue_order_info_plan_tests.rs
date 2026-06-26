use super::*;
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

fn manager() -> CatalogueManager {
    CatalogueManager::with_items_and_deals(
        [
            CatalogueItem::new("S2S", 1, 3),
            CatalogueItem::new("PEN", 4, 2),
            CatalogueItem::new("STN", 5, 3),
            CatalogueItem::new("PSN", 6, 4),
            CatalogueItem::new("PYN", 7, 2),
            CatalogueItem::new("T", 8, 2),
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
fn resolves_private_room_order_info_shape_and_strips_player_name() {
    let plan = CatalogueOrderInfoPlan::resolve(&manager(), "S2S Alex", Some("Alex")).unwrap();

    assert_eq!(plan.call_id(), "S2S");
    assert_eq!(plan.credits(), 3);
}

#[test]
fn resolves_private_room_poster_order_info_shape_and_strips_player_name() {
    let plan = CatalogueOrderInfoPlan::resolve(&manager(), "PEN Alex", Some("Alex")).unwrap();

    assert_eq!(plan.call_id(), "PEN");
    assert_eq!(plan.credits(), 2);
}

#[test]
fn resolves_java_catalogue_page_order_info_shapes_and_strips_player_name() {
    for (raw_call_id, expected_call_id, expected_credits) in [
        ("STN Alex", "STN", 3),
        ("PSN Alex", "PSN", 4),
        ("PYN Alex", "PYN", 2),
        ("T 101 Alex", "T 101", 2),
    ] {
        let plan = CatalogueOrderInfoPlan::resolve(&manager(), raw_call_id, Some("Alex")).unwrap();

        assert_eq!(plan.call_id(), expected_call_id);
        assert_eq!(plan.credits(), expected_credits);
    }
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

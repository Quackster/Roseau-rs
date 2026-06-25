use super::*;
use crate::game::item::ItemDefinition;

fn item(id: i32, custom_data: Option<String>) -> Item {
    Item::new(
        id,
        1,
        7,
        "1",
        2,
        0.0,
        0,
        ItemDefinition::new(id, "teleport", "", 1, 1, 0.0, "SFX", "", "", "DOOROPEN"),
        "",
        custom_data,
    )
    .unwrap()
}

#[test]
fn applies_runtime_custom_data_to_matching_item() {
    let mut items = vec![item(7, Some("FALSE".to_owned())), item(8, None)];

    let updated = ItemInteractionEffectItemExecutor::apply(
        &mut items,
        &ItemInteractionEffect::SetItemCustomData {
            item_id: 7,
            custom_data: "TRUE".to_owned(),
        },
    );

    assert_eq!(items[0].custom_data(), Some("TRUE"));
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].id(), 7);
    assert_eq!(updated[0].custom_data(), Some("TRUE"));
    assert_eq!(items[1].custom_data(), None);
}

#[test]
fn ignores_missing_items_and_other_boundaries() {
    let mut items = vec![item(7, Some("FALSE".to_owned()))];

    let updated = ItemInteractionEffectItemExecutor::apply_all(
        &mut items,
        &[
            ItemInteractionEffect::SetItemCustomData {
                item_id: 99,
                custom_data: "TRUE".to_owned(),
            },
            ItemInteractionEffect::UpdateItemStatus { item_id: 7 },
            ItemInteractionEffect::SavePlayer,
        ],
    );

    assert!(updated.is_empty());
    assert_eq!(items[0].custom_data(), Some("FALSE"));
}

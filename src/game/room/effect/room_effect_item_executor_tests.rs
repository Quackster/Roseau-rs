use super::room_effect_item_executor::*;
use crate::dao::in_memory::InMemoryItemDao;
use crate::game::item::ItemDefinition;
use crate::game::room::model::RoomModel;

fn definition(id: i32, sprite: &str) -> ItemDefinition {
    ItemDefinition::new(id, sprite, "", 1, 1, 1.0, "SF", "", "", "")
}

fn item(id: i32, room_id: i32, model_name: &str, x: &str) -> Item {
    Item::new(
        id,
        room_id,
        0,
        x,
        1,
        0.0,
        0,
        definition(id, "chair"),
        model_name,
        None,
    )
    .unwrap()
}

fn mapping() -> RoomMapping {
    RoomMapping::new(RoomModel::new("model", "000 000 000", 0, 0, 0, 0, true, false).unwrap())
}

#[test]
fn loads_public_room_passive_objects_from_item_dao() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(item(10, 7, "pool_b", "1"));
    dao.insert_item(item(11, 8, "pool_b", "2"));
    dao.insert_item(item(12, 7, "bar_b", "3"));
    let mut items = vec![item(10, 7, "old", "1")];
    let mut mapping = mapping();

    let loaded = RoomEffectItemExecutor::apply(
        &mut items,
        &mut mapping,
        &dao,
        &RoomEffect::LoadPassiveObjects {
            model_name: "pool_b".to_owned(),
            room_id: 7,
        },
    )
    .unwrap();

    assert_eq!(loaded.iter().map(Item::id).collect::<Vec<_>>(), vec![10]);
    assert_eq!(items.iter().map(Item::id).collect::<Vec<_>>(), vec![10]);
    assert_eq!(items[0].item_data(), "pool_b");
}

#[test]
fn regenerates_collision_map_from_loaded_items() {
    let dao = InMemoryItemDao::new();
    let mut items = vec![item(10, 7, "pool_b", "1")];
    let mut mapping = mapping();

    assert_eq!(mapping.highest_item_id(1, 1), None);

    RoomEffectItemExecutor::apply(
        &mut items,
        &mut mapping,
        &dao,
        &RoomEffect::RegenerateCollisionMaps,
    )
    .unwrap();

    assert_eq!(mapping.highest_item_id(1, 1), Some(10));
}

#[test]
fn ignores_non_item_room_effects() {
    let dao = InMemoryItemDao::new();
    let mut items = vec![item(10, 7, "pool_b", "1")];
    let mut mapping = mapping();

    let loaded = RoomEffectItemExecutor::apply_all(
        &mut items,
        &mut mapping,
        &dao,
        &[
            RoomEffect::SendOwnerPrivileges { user_id: 7 },
            RoomEffect::LoadBots { room_id: 7 },
        ],
    )
    .unwrap();

    assert!(loaded.is_empty());
    assert_eq!(items.len(), 1);
    assert_eq!(mapping.highest_item_id(1, 1), None);
}

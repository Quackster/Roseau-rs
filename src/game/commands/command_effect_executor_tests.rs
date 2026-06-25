use super::*;
use crate::dao::in_memory::InMemoryItemDao;
use crate::game::item::ItemDefinition;

fn definition(id: i32, sprite: &str) -> ItemDefinition {
    ItemDefinition::new(id, sprite, "red", 1, 1, 1.0, "SFC", "Chair", "", "")
}

#[test]
fn reloads_item_definitions_from_dao() {
    let dao = InMemoryItemDao::new();
    dao.insert_definition(definition(2, "table"));
    let mut item_manager = ItemManager::with_definitions([definition(1, "chair")]);

    CommandEffectExecutor::apply(
        &mut item_manager,
        &dao,
        &CommandEffect::ReloadItemDefinitions,
    )
    .unwrap();

    assert!(item_manager.get_definition(1).is_none());
    assert_eq!(item_manager.get_definition(2).unwrap().sprite(), "table");
}

#[test]
fn ignores_non_runtime_item_effects() {
    let dao = InMemoryItemDao::new();
    dao.insert_definition(definition(2, "table"));
    let mut item_manager = ItemManager::with_definitions([definition(1, "chair")]);

    CommandEffectExecutor::apply_all(
        &mut item_manager,
        &dao,
        &[
            CommandEffect::SendAlert("hello".to_owned()),
            CommandEffect::RemoveRoomStatus {
                key: "sit".to_owned(),
            },
            CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1".to_owned(),
                infinite: true,
                duration: -1,
            },
            CommandEffect::MarkRoomNeedsUpdate,
        ],
    )
    .unwrap();

    assert_eq!(item_manager.get_definition(1).unwrap().sprite(), "chair");
    assert!(item_manager.get_definition(2).is_none());
}

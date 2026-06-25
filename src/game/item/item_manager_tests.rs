use super::*;

fn definition(id: i32) -> ItemDefinition {
    ItemDefinition::new(id, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "")
}

#[test]
fn loads_and_replaces_definitions() {
    let mut manager = ItemManager::with_definitions([definition(1)]);
    assert!(manager.get_definition(1).is_some());

    manager.load_definitions([definition(2)]);

    assert!(manager.get_definition(1).is_none());
    assert_eq!(manager.get_definition(2).unwrap().sprite(), "chair");
}

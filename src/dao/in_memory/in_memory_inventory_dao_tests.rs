use super::*;
use super::*;
use crate::game::item::ItemDefinition;

fn chair_definition() -> ItemDefinition {
    ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "SWITCHON")
}

#[test]
fn creates_inventory_items_from_definitions() {
    let item_dao = InMemoryItemDao::new();
    item_dao.insert_definition(chair_definition());
    let inventory = InMemoryInventoryDao::new(item_dao);

    let item = inventory.new_item(5, 7, "ON").unwrap();

    assert_eq!(item.owner_id(), 7);
    assert_eq!(item.room_id(), 0);
    assert_eq!(inventory.inventory_items(7).unwrap().len(), 1);
    assert_eq!(
        inventory.item(item.id() as i64).unwrap().unwrap().id(),
        item.id()
    );
}

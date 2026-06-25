use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::dao::{DaoError, ItemDao};
use crate::game::item::{Item, ItemDefinition};

#[derive(Debug, Default)]
pub struct InMemoryItemDao {
    definitions: RefCell<HashMap<i32, ItemDefinition>>,
    items: RefCell<HashMap<i32, Item>>,
    next_item_instance_id: Cell<i32>,
}

impl InMemoryItemDao {
    pub fn new() -> Self {
        Self {
            definitions: RefCell::new(HashMap::new()),
            items: RefCell::new(HashMap::new()),
            next_item_instance_id: Cell::new(1),
        }
    }

    pub fn insert_definition(&self, definition: ItemDefinition) {
        self.definitions
            .borrow_mut()
            .insert(definition.id(), definition);
    }

    pub fn insert_item(&self, item: Item) {
        self.items.borrow_mut().insert(item.id(), item);
    }

    pub fn next_instance_id(&self) -> i32 {
        let id = self.next_item_instance_id.get();
        self.next_item_instance_id.set(id + 1);
        id
    }

    pub fn definition(&self, definition_id: i32) -> Option<ItemDefinition> {
        self.definitions.borrow().get(&definition_id).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.borrow().is_empty() && self.items.borrow().is_empty()
    }
}

impl ItemDao for InMemoryItemDao {
    fn definitions(&self) -> Result<HashMap<i32, ItemDefinition>, DaoError> {
        Ok(self.definitions.borrow().clone())
    }

    fn public_room_items(&self, model: &str, room_id: i32) -> Result<HashMap<i32, Item>, DaoError> {
        Ok(self
            .items
            .borrow()
            .iter()
            .filter(|(_, item)| item.room_id() == room_id && item.item_data() == model)
            .map(|(id, item)| (*id, item.clone()))
            .collect())
    }

    fn room_items(&self, room_id: i32) -> Result<HashMap<i32, Item>, DaoError> {
        Ok(self
            .items
            .borrow()
            .iter()
            .filter(|(_, item)| item.room_id() == room_id)
            .map(|(id, item)| (*id, item.clone()))
            .collect())
    }

    fn save_item(&self, item: &Item) -> Result<(), DaoError> {
        self.items.borrow_mut().insert(item.id(), item.clone());
        Ok(())
    }

    fn delete_item(&self, id: i64) -> Result<(), DaoError> {
        self.items.borrow_mut().remove(&(id as i32));
        Ok(())
    }

    fn item(&self, item_id: i32) -> Result<Option<Item>, DaoError> {
        Ok(self.items.borrow().get(&item_id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn chair_definition() -> ItemDefinition {
        ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "SWITCHON")
    }

    pub fn chair_item(id: i32, room_id: i32, owner_id: i32) -> Item {
        Item::new(
            id,
            room_id,
            owner_id,
            "1",
            2,
            0.0,
            4,
            chair_definition(),
            "",
            Some("ON".to_owned()),
        )
        .unwrap()
    }

    #[test]
    fn stores_definitions_and_room_items() {
        let dao = InMemoryItemDao::new();
        dao.insert_definition(chair_definition());
        dao.insert_item(chair_item(10, 7, 3));
        dao.insert_item(chair_item(11, 0, 3));

        assert_eq!(dao.definitions().unwrap().len(), 1);
        assert_eq!(dao.room_items(7).unwrap().len(), 1);
        assert_eq!(dao.item(10).unwrap().unwrap().owner_id(), 3);
    }

    #[test]
    fn saves_and_deletes_items() {
        let dao = InMemoryItemDao::new();
        let mut item = chair_item(10, 7, 3);
        dao.save_item(&item).unwrap();

        item.set_room_id(0);
        dao.save_item(&item).unwrap();

        assert_eq!(dao.item(10).unwrap().unwrap().room_id(), 0);
        dao.delete_item(10).unwrap();
        assert!(dao.item(10).unwrap().is_none());
    }
}

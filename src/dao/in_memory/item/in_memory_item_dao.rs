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
#[path = "in_memory_item_dao_tests.rs"]
mod tests;

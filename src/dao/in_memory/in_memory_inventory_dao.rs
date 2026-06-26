use std::cell::RefCell;
use std::rc::Rc;

use crate::dao::in_memory::InMemoryItemDao;
use crate::dao::{DaoError, InventoryDao, ItemDao};
use crate::game::item::Item;

#[derive(Debug, Clone)]
pub struct InMemoryInventoryDao {
    item_dao: Rc<InMemoryItemDao>,
    created_items: RefCell<Vec<i32>>,
}

impl InMemoryInventoryDao {
    pub fn new(item_dao: InMemoryItemDao) -> Self {
        Self {
            item_dao: Rc::new(item_dao),
            created_items: RefCell::new(Vec::new()),
        }
    }

    pub fn shared(item_dao: Rc<InMemoryItemDao>) -> Self {
        Self {
            item_dao,
            created_items: RefCell::new(Vec::new()),
        }
    }

    pub fn item_dao(&self) -> Rc<InMemoryItemDao> {
        Rc::clone(&self.item_dao)
    }
}

impl Default for InMemoryInventoryDao {
    fn default() -> Self {
        Self::new(InMemoryItemDao::new())
    }
}

impl InventoryDao for InMemoryInventoryDao {
    fn inventory_items(&self, user_id: i32) -> Result<Vec<Item>, DaoError> {
        Ok(self
            .item_dao
            .room_items(0)?
            .into_values()
            .filter(|item| item.owner_id() == user_id)
            .collect())
    }

    fn item(&self, id: i64) -> Result<Option<Item>, DaoError> {
        self.item_dao.item(id as i32)
    }

    fn new_item(&self, item_id: i32, owner_id: i32, extra_data: &str) -> Result<Item, DaoError> {
        let definition = self
            .item_dao
            .definition(item_id)
            .ok_or_else(|| DaoError::new(format!("missing item definition {item_id}")))?;
        let id = self.item_dao.next_instance_id();
        let item = Item::new(
            id,
            0,
            owner_id,
            "0",
            0,
            0.0,
            0,
            definition,
            "",
            Some(extra_data.to_owned()),
        )
        .map_err(|error| DaoError::new(error.to_string()))?;

        self.item_dao.save_item(&item)?;
        self.created_items.borrow_mut().push(id);
        Ok(item)
    }
}

#[cfg(test)]
#[path = "in_memory_inventory_dao_tests.rs"]
mod tests;

use std::collections::HashMap;

use crate::dao::DaoError;
use crate::game::item::{Item, ItemDefinition};

pub trait ItemDao {
    fn definitions(&self) -> Result<HashMap<i32, ItemDefinition>, DaoError>;
    fn public_room_items(&self, model: &str, room_id: i32) -> Result<HashMap<i32, Item>, DaoError>;
    fn room_items(&self, room_id: i32) -> Result<HashMap<i32, Item>, DaoError>;
    fn save_item(&self, item: &Item) -> Result<(), DaoError>;
    fn delete_item(&self, id: i64) -> Result<(), DaoError>;
    fn item(&self, item_id: i32) -> Result<Option<Item>, DaoError>;
}

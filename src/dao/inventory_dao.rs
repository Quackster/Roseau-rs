use crate::dao::DaoError;
use crate::game::item::Item;

pub trait InventoryDao {
    fn inventory_items(&self, user_id: i32) -> Result<Vec<Item>, DaoError>;
    fn item(&self, id: i64) -> Result<Option<Item>, DaoError>;
    fn new_item(&self, item_id: i32, owner_id: i32, extra_data: &str) -> Result<Item, DaoError>;
}

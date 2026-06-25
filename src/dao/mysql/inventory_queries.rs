use crate::dao::mysql::entity::ItemRow;
use crate::dao::mysql::{SqlParameter, SqlQuery};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryQueries;

impl InventoryQueries {
    pub fn inventory_items(user_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM items WHERE room_id = ? AND user_id = ?",
            [SqlParameter::Integer(0), SqlParameter::Integer(user_id)],
        )
    }

    pub fn item(id: i64) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM items WHERE id = ? LIMIT 1",
            [SqlParameter::Long(id)],
        )
    }

    pub fn create_item(item_id: i32, owner_id: i32, extra_data: impl Into<String>) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO items (user_id, item_id, room_id, x, extra_data) VALUES (?, ?, ?, ?, ?)",
            [
                SqlParameter::Integer(owner_id),
                SqlParameter::Integer(item_id),
                SqlParameter::Integer(0),
                SqlParameter::Text("0".to_owned()),
                SqlParameter::Text(extra_data.into()),
            ],
        )
    }

    pub fn update_extra_data(item_id: i32, extra_data: impl Into<String>) -> SqlQuery {
        SqlQuery::new(
            "UPDATE items SET extra_data = ? WHERE id = ?",
            [
                SqlParameter::Text(extra_data.into()),
                SqlParameter::Integer(item_id),
            ],
        )
    }

    pub fn table() -> &'static str {
        ItemRow::TABLE
    }
}

#[cfg(test)]
#[path = "inventory_queries_tests.rs"]
mod tests;

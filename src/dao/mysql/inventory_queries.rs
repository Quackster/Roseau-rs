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
mod tests {
    use super::*;

    #[test]
    fn builds_inventory_filter_for_unplaced_user_items() {
        let query = InventoryQueries::inventory_items(7);

        assert_eq!(
            query.sql(),
            "SELECT * FROM items WHERE room_id = ? AND user_id = ?"
        );
        assert_eq!(
            query.parameters(),
            &[SqlParameter::Integer(0), SqlParameter::Integer(7)]
        );
    }

    #[test]
    fn builds_single_item_lookup_and_insert_queries() {
        let item = InventoryQueries::item(42);
        let create = InventoryQueries::create_item(5, 7, "red");
        let update_extra_data = InventoryQueries::update_extra_data(42, "84");

        assert_eq!(item.sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
        assert_eq!(item.parameters(), &[SqlParameter::Long(42)]);
        assert_eq!(
            create.sql(),
            "INSERT INTO items (user_id, item_id, room_id, x, extra_data) VALUES (?, ?, ?, ?, ?)"
        );
        assert_eq!(
            create.parameters(),
            &[
                SqlParameter::Integer(7),
                SqlParameter::Integer(5),
                SqlParameter::Integer(0),
                SqlParameter::Text("0".to_owned()),
                SqlParameter::Text("red".to_owned()),
            ]
        );
        assert_eq!(
            update_extra_data.sql(),
            "UPDATE items SET extra_data = ? WHERE id = ?"
        );
        assert_eq!(
            update_extra_data.parameters(),
            &[
                SqlParameter::Text("84".to_owned()),
                SqlParameter::Integer(42)
            ]
        );
        assert_eq!(InventoryQueries::table(), "items");
    }
}

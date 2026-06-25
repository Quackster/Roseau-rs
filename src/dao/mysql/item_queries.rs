use crate::dao::mysql::entity::{ItemDefinitionRow, ItemRow};
use crate::dao::mysql::{SqlParameter, SqlQuery};
use crate::game::item::Item;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemQueries;

impl ItemQueries {
    pub fn definitions() -> SqlQuery {
        SqlQuery::select_all(ItemDefinitionRow::TABLE)
    }

    pub fn public_room_items(model: &str) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM room_public_items WHERE model = ?",
            [SqlParameter::Text(model.to_owned())],
        )
    }

    pub fn room_items(room_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM items WHERE room_id = ?",
            [SqlParameter::Integer(room_id)],
        )
    }

    pub fn item(item_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM items WHERE id = ? LIMIT 1",
            [SqlParameter::Integer(item_id)],
        )
    }

    pub fn save_item(item: &Item) -> SqlQuery {
        let position = item.position();
        SqlQuery::new(
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?",
            [
                SqlParameter::Text(Self::save_extra_data(item)),
                SqlParameter::Text(Self::save_x(item)),
                SqlParameter::Integer(position.y()),
                SqlParameter::Float(position.z()),
                SqlParameter::Integer(position.rotation()),
                SqlParameter::Integer(item.room_id()),
                SqlParameter::Integer(item.owner_id()),
                SqlParameter::Integer(item.id()),
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

    pub fn place_wall_item(
        item_id: i32,
        room_id: i32,
        owner_id: i32,
        wall_position: impl Into<String>,
        extra_data: impl Into<String>,
    ) -> SqlQuery {
        SqlQuery::new(
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?",
            [
                SqlParameter::Text(extra_data.into()),
                SqlParameter::Text(wall_position.into()),
                SqlParameter::Integer(0),
                SqlParameter::Float(0.0),
                SqlParameter::Integer(0),
                SqlParameter::Integer(room_id),
                SqlParameter::Integer(owner_id),
                SqlParameter::Integer(item_id),
            ],
        )
    }

    pub fn place_floor_item(
        item_id: i32,
        room_id: i32,
        owner_id: i32,
        x: i32,
        y: i32,
        z: f64,
        rotation: i32,
        extra_data: impl Into<String>,
    ) -> SqlQuery {
        SqlQuery::new(
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?",
            [
                SqlParameter::Text(extra_data.into()),
                SqlParameter::Text(x.to_string()),
                SqlParameter::Integer(y),
                SqlParameter::Float(z),
                SqlParameter::Integer(rotation),
                SqlParameter::Integer(room_id),
                SqlParameter::Integer(owner_id),
                SqlParameter::Integer(item_id),
            ],
        )
    }

    pub fn move_floor_item(
        item_id: i32,
        x: i32,
        y: i32,
        z: f64,
        rotation: i32,
        extra_data: impl Into<String>,
    ) -> SqlQuery {
        SqlQuery::new(
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ? WHERE id = ?",
            [
                SqlParameter::Text(extra_data.into()),
                SqlParameter::Text(x.to_string()),
                SqlParameter::Integer(y),
                SqlParameter::Float(z),
                SqlParameter::Integer(rotation),
                SqlParameter::Integer(item_id),
            ],
        )
    }

    pub fn return_floor_item_to_inventory(item_id: i32, owner_id: i32) -> SqlQuery {
        SqlQuery::new(
            "UPDATE items SET x = ?, y = ?, room_id = ?, user_id = ? WHERE id = ?",
            [
                SqlParameter::Text("-1".to_owned()),
                SqlParameter::Integer(-1),
                SqlParameter::Integer(0),
                SqlParameter::Integer(owner_id),
                SqlParameter::Integer(item_id),
            ],
        )
    }

    pub fn return_item_to_inventory(item_id: i32) -> SqlQuery {
        SqlQuery::new(
            "UPDATE items SET room_id = ? WHERE id = ?",
            [SqlParameter::Integer(0), SqlParameter::Integer(item_id)],
        )
    }

    pub fn delete_item(id: i64) -> SqlQuery {
        SqlQuery::new("DELETE FROM items WHERE id = ?", [SqlParameter::Long(id)])
    }

    pub fn save_extra_data(item: &Item) -> String {
        if item.definition().behaviour().is_teleporter() {
            if let Some(custom_data) = item.custom_data() {
                if custom_data.parse::<i32>().is_ok() {
                    return custom_data.to_owned();
                }
            }

            return item.target_teleporter_id().to_string();
        }

        item.custom_data().unwrap_or_default().to_owned()
    }

    fn save_x(item: &Item) -> String {
        if item.definition().behaviour().is_on_wall() {
            item.wall_position().unwrap_or_default().to_owned()
        } else {
            item.position().x().to_string()
        }
    }

    pub fn item_table() -> &'static str {
        ItemRow::TABLE
    }
}

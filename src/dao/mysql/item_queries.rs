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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn definition(flags: &str) -> ItemDefinition {
        ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, flags, "Chair", "", "")
    }

    fn item(flags: &str, custom_data: Option<String>) -> Item {
        Item::new(10, 7, 3, "1", 2, 0.5, 4, definition(flags), "", custom_data).unwrap()
    }

    #[test]
    fn builds_definition_public_room_and_room_item_reads() {
        assert_eq!(
            ItemQueries::definitions().sql(),
            "SELECT * FROM item_definitions"
        );

        let public_items = ItemQueries::public_room_items("pool_a");
        assert_eq!(
            public_items.sql(),
            "SELECT * FROM room_public_items WHERE model = ?"
        );
        assert_eq!(
            public_items.parameters(),
            &[SqlParameter::Text("pool_a".to_owned())]
        );

        let room_items = ItemQueries::room_items(8);
        assert_eq!(room_items.sql(), "SELECT * FROM items WHERE room_id = ?");
        assert_eq!(room_items.parameters(), &[SqlParameter::Integer(8)]);
        assert_eq!(ItemQueries::item_table(), "items");
    }

    #[test]
    fn builds_single_item_delete_and_save_update() {
        let lookup = ItemQueries::item(10);
        let delete = ItemQueries::delete_item(10);
        let save = ItemQueries::save_item(&item("", Some("ON".to_owned())));

        assert_eq!(lookup.sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
        assert_eq!(delete.sql(), "DELETE FROM items WHERE id = ?");
        assert_eq!(
            save.sql(),
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?"
        );
        assert_eq!(
            save.parameters(),
            &[
                SqlParameter::Text("ON".to_owned()),
                SqlParameter::Text("1".to_owned()),
                SqlParameter::Integer(2),
                SqlParameter::Float(0.5),
                SqlParameter::Integer(4),
                SqlParameter::Integer(7),
                SqlParameter::Integer(3),
                SqlParameter::Integer(10),
            ]
        );
    }

    #[test]
    fn builds_focused_item_mutation_queries() {
        let extra_data = ItemQueries::update_extra_data(42, "ON");
        let wall = ItemQueries::place_wall_item(42, 7, 9, ":w=1,1", "paper");
        let floor = ItemQueries::place_floor_item(43, 7, 9, 2, 3, 1.5, 4, "");
        let moved = ItemQueries::move_floor_item(44, 5, 6, 2.25, 3, "3");
        let return_floor = ItemQueries::return_floor_item_to_inventory(43, 9);
        let return_wall = ItemQueries::return_item_to_inventory(42);

        assert_eq!(
            extra_data.sql(),
            "UPDATE items SET extra_data = ? WHERE id = ?"
        );
        assert_eq!(
            extra_data.parameters(),
            &[
                SqlParameter::Text("ON".to_owned()),
                SqlParameter::Integer(42)
            ]
        );
        assert_eq!(
            wall.sql(),
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?"
        );
        assert_eq!(
            wall.parameters(),
            &[
                SqlParameter::Text("paper".to_owned()),
                SqlParameter::Text(":w=1,1".to_owned()),
                SqlParameter::Integer(0),
                SqlParameter::Float(0.0),
                SqlParameter::Integer(0),
                SqlParameter::Integer(7),
                SqlParameter::Integer(9),
                SqlParameter::Integer(42),
            ]
        );
        assert_eq!(
            floor.parameters(),
            &[
                SqlParameter::Text(String::new()),
                SqlParameter::Text("2".to_owned()),
                SqlParameter::Integer(3),
                SqlParameter::Float(1.5),
                SqlParameter::Integer(4),
                SqlParameter::Integer(7),
                SqlParameter::Integer(9),
                SqlParameter::Integer(43),
            ]
        );
        assert_eq!(
            moved.sql(),
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ? WHERE id = ?"
        );
        assert_eq!(
            moved.parameters(),
            &[
                SqlParameter::Text("3".to_owned()),
                SqlParameter::Text("5".to_owned()),
                SqlParameter::Integer(6),
                SqlParameter::Float(2.25),
                SqlParameter::Integer(3),
                SqlParameter::Integer(44),
            ]
        );
        assert_eq!(
            return_floor.sql(),
            "UPDATE items SET x = ?, y = ?, room_id = ?, user_id = ? WHERE id = ?"
        );
        assert_eq!(
            return_floor.parameters(),
            &[
                SqlParameter::Text("-1".to_owned()),
                SqlParameter::Integer(-1),
                SqlParameter::Integer(0),
                SqlParameter::Integer(9),
                SqlParameter::Integer(43),
            ]
        );
        assert_eq!(
            return_wall.parameters(),
            &[SqlParameter::Integer(0), SqlParameter::Integer(42)]
        );
    }

    #[test]
    fn save_extra_data_matches_java_teleporter_fallback() {
        let numeric = item("X", Some("22".to_owned()));
        let fallback = item("X", Some("room-link".to_owned()));

        assert_eq!(ItemQueries::save_extra_data(&numeric), "22");
        assert_eq!(ItemQueries::save_extra_data(&fallback), "0");
    }
}

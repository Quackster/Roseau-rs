use crate::dao::mysql::{ItemQueries, SqlExecutionPlan};
use crate::game::item::Item;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemCommandQueries;

impl ItemCommandQueries {
    pub fn read_plan(effect: &IncomingExecutionEffect) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::MoveStuff { item_id, .. }
            | IncomingExecutionEffect::ApplyDecoration { item_id, .. }
            | IncomingExecutionEffect::RemoveItem { item_id }
            | IncomingExecutionEffect::SetItemData { item_id, .. }
            | IncomingExecutionEffect::SetStuffData { item_id, .. }
            | IncomingExecutionEffect::UseStripItem { item_id }
            | IncomingExecutionEffect::ReturnItemToInventory { item_id }
            | IncomingExecutionEffect::PlaceWallItemFromInventory { item_id, .. }
            | IncomingExecutionEffect::PlaceFloorItemFromInventory { item_id, .. } => {
                Some(ItemQueries::item(*item_id).read_plan())
            }
            _ => None,
        }
    }

    pub fn plan(effect: &IncomingExecutionEffect) -> Vec<SqlExecutionPlan> {
        match effect {
            _ => Vec::new(),
        }
    }

    pub fn set_item_data_plan(
        item_id: i32,
        current_data: Option<&str>,
        new_data: &str,
    ) -> Option<SqlExecutionPlan> {
        if !new_data.starts_with(current_data.unwrap_or_default()) {
            return None;
        }

        Some(ItemQueries::update_extra_data(item_id, new_data).execute_plan())
    }

    pub fn set_stuff_data_plan(
        item_id: i32,
        item_data_class: &str,
        custom_data: &str,
    ) -> Option<SqlExecutionPlan> {
        let extra_data = Self::normalise_stuff_data(item_data_class, custom_data)?;
        Some(ItemQueries::update_extra_data(item_id, extra_data).execute_plan())
    }

    pub fn use_post_it_plan(item_id: i32, current_amount: i32) -> Option<SqlExecutionPlan> {
        let new_amount = current_amount.checked_sub(1)?;

        if new_amount > 0 {
            Some(ItemQueries::update_extra_data(item_id, new_amount.to_string()).execute_plan())
        } else {
            Some(ItemQueries::delete_item(i64::from(item_id)).execute_plan())
        }
    }

    pub fn remove_item_plan(item_id: i32) -> SqlExecutionPlan {
        ItemQueries::delete_item(i64::from(item_id)).execute_plan()
    }

    pub fn place_wall_item_plan(
        item_id: i32,
        room_id: i32,
        owner_id: i32,
        wall_position: impl Into<String>,
        extra_data: impl Into<String>,
    ) -> SqlExecutionPlan {
        ItemQueries::place_wall_item(item_id, room_id, owner_id, wall_position, extra_data)
            .execute_plan()
    }

    pub fn place_floor_item_plan(
        item_id: i32,
        room_id: i32,
        owner_id: i32,
        x: i32,
        y: i32,
        z: f64,
        rotation: i32,
        extra_data: impl Into<String>,
    ) -> SqlExecutionPlan {
        ItemQueries::place_floor_item(item_id, room_id, owner_id, x, y, z, rotation, extra_data)
            .execute_plan()
    }

    pub fn move_floor_item_plan(
        item_id: i32,
        x: i32,
        y: i32,
        z: f64,
        rotation: i32,
        extra_data: impl Into<String>,
    ) -> SqlExecutionPlan {
        ItemQueries::move_floor_item(item_id, x, y, z, rotation, extra_data).execute_plan()
    }

    pub fn moved_item_plan(item: &Item) -> Option<SqlExecutionPlan> {
        if !item.definition().behaviour().is_on_floor() {
            return None;
        }

        let position = item.position();
        Some(Self::move_floor_item_plan(
            item.id(),
            position.x(),
            position.y(),
            position.z(),
            position.rotation(),
            ItemQueries::save_extra_data(item),
        ))
    }

    pub fn return_floor_item_plan(item_id: i32, owner_id: i32) -> SqlExecutionPlan {
        ItemQueries::return_floor_item_to_inventory(item_id, owner_id).execute_plan()
    }

    pub fn return_wall_item_plan(item_id: i32) -> SqlExecutionPlan {
        ItemQueries::return_item_to_inventory(item_id).execute_plan()
    }

    fn normalise_stuff_data(item_data_class: &str, custom_data: &str) -> Option<String> {
        match item_data_class {
            "NULL" | "DIR" | "DOOROPEN" => None,
            "SWITCHON" | "FIREON" => Some(if custom_data == "ON" { "ON" } else { "OFF" }),
            "STATUS" => Some(if custom_data == "O" { "O" } else { "C" }),
            _ => Some(custom_data),
        }
        .map(str::to_owned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlParameter};
    use crate::game::item::ItemDefinition;

    #[test]
    fn maps_set_item_data_effect_to_current_item_read() {
        let read_plan = ItemCommandQueries::read_plan(&IncomingExecutionEffect::SetItemData {
            item_id: 42,
            data: "sticky note".to_owned(),
        })
        .unwrap();

        assert_eq!(read_plan.kind(), SqlExecutionKind::ReadRows);
        assert_eq!(read_plan.sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
        assert_eq!(read_plan.parameters(), &[SqlParameter::Integer(42)]);
    }

    #[test]
    fn maps_item_command_effects_to_current_item_reads() {
        let move_plan = ItemCommandQueries::read_plan(&IncomingExecutionEffect::MoveStuff {
            item_id: 42,
            x: 5,
            y: 6,
            rotation: Some(2),
        })
        .unwrap();
        let set_data_plan = ItemCommandQueries::read_plan(&IncomingExecutionEffect::SetStuffData {
            item_id: 43,
            data_class: "SWITCHON".to_owned(),
            custom_data: "ON".to_owned(),
        })
        .unwrap();
        let return_plan =
            ItemCommandQueries::read_plan(&IncomingExecutionEffect::ReturnItemToInventory {
                item_id: 44,
            })
            .unwrap();

        assert_eq!(move_plan.kind(), SqlExecutionKind::ReadRows);
        assert_eq!(move_plan.sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
        assert_eq!(move_plan.parameters(), &[SqlParameter::Integer(42)]);
        assert_eq!(set_data_plan.parameters(), &[SqlParameter::Integer(43)]);
        assert_eq!(return_plan.parameters(), &[SqlParameter::Integer(44)]);
        assert_eq!(
            ItemCommandQueries::read_plan(&IncomingExecutionEffect::GoAway),
            None
        );
    }

    #[test]
    fn ignores_item_effects_that_need_runtime_context() {
        assert!(ItemCommandQueries::plan(&IncomingExecutionEffect::GoAway).is_empty());
    }

    #[test]
    fn maps_set_stuff_data_to_java_compatible_extra_data_updates() {
        let item_data =
            ItemCommandQueries::set_item_data_plan(42, Some("old"), "old plus").unwrap();

        assert_eq!(
            item_data.parameters(),
            &[
                SqlParameter::Text("old plus".to_owned()),
                SqlParameter::Integer(42)
            ]
        );
        assert_eq!(
            ItemCommandQueries::set_item_data_plan(42, Some("old"), "new"),
            None
        );

        let switch_plan = ItemCommandQueries::set_stuff_data_plan(42, "SWITCHON", "BROKEN")
            .expect("switch data should persist");
        let status_plan = ItemCommandQueries::set_stuff_data_plan(43, "STATUS", "BROKEN")
            .expect("status data should persist");

        assert_eq!(
            switch_plan.parameters(),
            &[
                SqlParameter::Text("OFF".to_owned()),
                SqlParameter::Integer(42),
            ]
        );
        assert_eq!(
            status_plan.parameters(),
            &[
                SqlParameter::Text("C".to_owned()),
                SqlParameter::Integer(43)
            ]
        );
        assert_eq!(
            ItemCommandQueries::set_stuff_data_plan(44, "DOOROPEN", "TRUE"),
            None
        );
        assert_eq!(
            ItemCommandQueries::set_stuff_data_plan(45, "DIR", "3"),
            None
        );
    }

    #[test]
    fn maps_post_it_use_to_amount_update_or_delete() {
        let update = ItemCommandQueries::use_post_it_plan(42, 3).unwrap();
        let delete = ItemCommandQueries::use_post_it_plan(42, 1).unwrap();
        let remove = ItemCommandQueries::remove_item_plan(43);

        assert_eq!(
            update.parameters(),
            &[
                SqlParameter::Text("2".to_owned()),
                SqlParameter::Integer(42)
            ]
        );
        assert_eq!(delete.sql(), "DELETE FROM items WHERE id = ?");
        assert_eq!(delete.parameters(), &[SqlParameter::Long(42)]);
        assert_eq!(remove.sql(), "DELETE FROM items WHERE id = ?");
        assert_eq!(remove.parameters(), &[SqlParameter::Long(43)]);
    }

    #[test]
    fn maps_inventory_place_and_return_context_to_item_updates() {
        let wall = ItemCommandQueries::place_wall_item_plan(42, 7, 9, ":w=1,1", "paper");
        let floor = ItemCommandQueries::place_floor_item_plan(43, 7, 9, 2, 3, 1.5, 4, "");
        let moved = ItemCommandQueries::move_floor_item_plan(44, 5, 6, 2.25, 3, "3");
        let return_floor = ItemCommandQueries::return_floor_item_plan(43, 9);
        let return_wall = ItemCommandQueries::return_wall_item_plan(42);

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
            return_wall.sql(),
            "UPDATE items SET room_id = ? WHERE id = ?"
        );
    }

    #[test]
    fn maps_moved_floor_item_to_persisted_position_and_custom_data() {
        let item = Item::new(
            44,
            7,
            9,
            "5",
            6,
            2.25,
            3,
            ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SIF", "Chair", "", ""),
            "",
            Some("3".to_owned()),
        )
        .unwrap();
        let wall_item = Item::new(
            45,
            7,
            9,
            ":w=1,1",
            0,
            0.0,
            0,
            ItemDefinition::new(6, "poster", "red", 1, 1, 0.0, "SIW", "Poster", "", ""),
            "",
            Some("paper".to_owned()),
        )
        .unwrap();

        let plan = ItemCommandQueries::moved_item_plan(&item).unwrap();

        assert_eq!(
            plan.sql(),
            "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ? WHERE id = ?"
        );
        assert_eq!(
            plan.parameters(),
            &[
                SqlParameter::Text("3".to_owned()),
                SqlParameter::Text("5".to_owned()),
                SqlParameter::Integer(6),
                SqlParameter::Float(2.25),
                SqlParameter::Integer(3),
                SqlParameter::Integer(44),
            ]
        );
        assert_eq!(ItemCommandQueries::moved_item_plan(&wall_item), None);
    }
}

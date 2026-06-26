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

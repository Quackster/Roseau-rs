use crate::dao::{DaoError, ItemDao};
use crate::game::item::{Item, ItemCommandExecution};
use crate::game::room::model::Position;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemCommandPlacementExecutor;

impl ItemCommandPlacementExecutor {
    pub fn place_wall_item(
        item_dao: &dyn ItemDao,
        item_id: i32,
        room_id: i32,
        room_owner_id: i32,
        wall_position: &str,
        has_rights: bool,
        all_super_user: bool,
    ) -> Result<ItemCommandExecution, DaoError> {
        if !has_rights && !all_super_user {
            return Ok(ItemCommandExecution::Ignored);
        }

        let Some(mut item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };
        if !item.definition().behaviour().is_on_wall() {
            return Ok(ItemCommandExecution::Ignored);
        }

        let wall_position = item
            .custom_data()
            .map(|custom_data| wall_position.replace(&format!("/{custom_data}"), ""))
            .unwrap_or_else(|| wall_position.to_owned());

        item.set_room_id(room_id);
        item.set_owner_id(room_owner_id);
        item.set_wall_position(wall_position);
        apply_dir_custom_data(&mut item);
        item_dao.save_item(&item)?;
        Ok(ItemCommandExecution::RoomItemPlaced(item))
    }

    pub fn place_floor_item(
        item_dao: &dyn ItemDao,
        item_id: i32,
        room_id: i32,
        room_owner_id: i32,
        x: i32,
        y: i32,
        has_rights: bool,
        all_super_user: bool,
    ) -> Result<ItemCommandExecution, DaoError> {
        if !has_rights && !all_super_user {
            return Ok(ItemCommandExecution::Ignored);
        }

        let Some(mut item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };
        if !item.definition().behaviour().is_on_floor() {
            return Ok(ItemCommandExecution::Ignored);
        }

        let rotation = if item.definition().behaviour().is_teleporter() {
            4
        } else {
            0
        };
        item.set_room_id(room_id);
        item.set_owner_id(room_owner_id);
        *item.position_mut() = Position::with_rotation(x, y, item.position().z(), rotation);
        apply_dir_custom_data(&mut item);
        item_dao.save_item(&item)?;
        Ok(ItemCommandExecution::RoomItemPlaced(item))
    }

    pub fn move_stuff(
        item_dao: &dyn ItemDao,
        item_id: i32,
        x: i32,
        y: i32,
        rotation: Option<i32>,
        dir_rotation: Option<i32>,
        room_id: i32,
        has_rights: bool,
        all_super_user: bool,
    ) -> Result<ItemCommandExecution, DaoError> {
        if !has_rights && !all_super_user {
            return Ok(ItemCommandExecution::Ignored);
        }

        let Some(mut item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };
        if item.room_id() != room_id {
            return Ok(ItemCommandExecution::Ignored);
        }

        let rotation = rotation.unwrap_or_else(|| item.position().rotation());
        *item.position_mut() = Position::with_rotation(x, y, item.position().z(), rotation);
        item.set_custom_data("");
        apply_movement_dir_custom_data(&mut item, dir_rotation);
        item_dao.save_item(&item)?;
        Ok(ItemCommandExecution::RoomItemMoved(item))
    }
}

fn apply_dir_custom_data(item: &mut Item) {
    if item.definition().data_class() == "DIR" {
        item.set_custom_data(item.position().rotation().to_string());
    }
}

fn apply_movement_dir_custom_data(item: &mut Item, dir_rotation: Option<i32>) {
    if item.definition().data_class() == "DIR" {
        let rotation = dir_rotation
            .unwrap_or_else(|| item.position().rotation())
            .clamp(0, 6);
        *item.position_mut() = Position::with_rotation(
            item.position().x(),
            item.position().y(),
            item.position().z(),
            rotation,
        );
        item.set_custom_data(rotation.to_string());
    }
}

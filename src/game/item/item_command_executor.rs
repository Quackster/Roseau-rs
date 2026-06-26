use crate::dao::{DaoError, ItemDao};
use crate::game::item::{Item, ItemCommandPlacementExecutor};
use crate::game::room::model::Position;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemCommandExecutor;

impl ItemCommandExecutor {
    pub fn set_item_data(
        item_dao: &dyn ItemDao,
        item_id: i32,
        room_id: i32,
        data: &str,
        has_rights: bool,
    ) -> Result<ItemCommandExecution, DaoError> {
        if !has_rights {
            return Ok(ItemCommandExecution::Ignored);
        }

        let Some(mut item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };
        if item.room_id() != room_id {
            return Ok(ItemCommandExecution::Ignored);
        }

        let current_data = item.custom_data().unwrap_or_default();
        if !data.starts_with(current_data) {
            return Ok(ItemCommandExecution::Ignored);
        }

        item.set_custom_data(data);
        item_dao.save_item(&item)?;
        Ok(ItemCommandExecution::Updated(item))
    }

    pub fn set_stuff_data(
        item_dao: &dyn ItemDao,
        item_id: i32,
        room_id: i32,
        _data_class: &str,
        custom_data: &str,
    ) -> Result<ItemCommandExecution, DaoError> {
        let Some(mut item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };
        if item.room_id() != room_id {
            return Ok(ItemCommandExecution::Ignored);
        }
        let Some(extra_data) = normalise_stuff_data(item.definition().data_class(), custom_data)
        else {
            return Ok(ItemCommandExecution::Ignored);
        };

        item.set_custom_data(extra_data);
        if item.definition().data_class() == "DOOROPEN" {
            return Ok(ItemCommandExecution::RuntimeUpdated(item));
        }

        item_dao.save_item(&item)?;
        Ok(ItemCommandExecution::StuffDataUpdated(item))
    }

    pub fn use_strip_item(
        item_dao: &dyn ItemDao,
        item_id: i32,
    ) -> Result<ItemCommandExecution, DaoError> {
        let Some(mut item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };

        if !item.definition().behaviour().is_post_it() {
            return Ok(ItemCommandExecution::Ignored);
        }

        let current_amount = item
            .custom_data()
            .and_then(|amount| amount.parse::<i32>().ok())
            .unwrap_or(0);
        let Some(new_amount) = current_amount.checked_sub(1) else {
            return Ok(ItemCommandExecution::Ignored);
        };

        if new_amount > 0 {
            item.set_custom_data(new_amount.to_string());
            item_dao.save_item(&item)?;
            Ok(ItemCommandExecution::Updated(item))
        } else {
            item_dao.delete_item(i64::from(item_id))?;
            Ok(ItemCommandExecution::Deleted { item_id })
        }
    }

    pub fn remove_item(
        item_dao: &dyn ItemDao,
        item_id: i32,
        room_id: i32,
        has_owner_rights: bool,
    ) -> Result<ItemCommandExecution, DaoError> {
        if !has_owner_rights {
            return Ok(ItemCommandExecution::Ignored);
        }

        let Some(item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };
        if item.room_id() != room_id {
            return Ok(ItemCommandExecution::Ignored);
        }

        item_dao.delete_item(i64::from(item_id))?;
        Ok(ItemCommandExecution::RoomItemDeleted(item))
    }

    pub fn return_item_to_inventory(
        item_dao: &dyn ItemDao,
        item_id: i32,
        room_id: i32,
        owner_id: i32,
        has_owner_rights: bool,
    ) -> Result<ItemCommandExecution, DaoError> {
        if !has_owner_rights {
            return Ok(ItemCommandExecution::Ignored);
        }

        let Some(mut item) = item_dao.item(item_id)? else {
            return Ok(ItemCommandExecution::Ignored);
        };
        if item.room_id() != room_id {
            return Ok(ItemCommandExecution::Ignored);
        }

        if item.definition().behaviour().is_on_floor() {
            item.set_room_id(0);
            item.set_owner_id(owner_id);
            *item.position_mut() =
                Position::with_rotation(-1, -1, item.position().z(), item.position().rotation());
        } else if item.definition().behaviour().is_on_wall() {
            item.set_room_id(0);
        } else {
            return Ok(ItemCommandExecution::Ignored);
        }

        item_dao.save_item(&item)?;
        Ok(ItemCommandExecution::RoomItemReturned(item))
    }

    pub fn place_wall_item(
        item_dao: &dyn ItemDao,
        item_id: i32,
        room_id: i32,
        room_owner_id: i32,
        wall_position: &str,
        has_rights: bool,
        all_super_user: bool,
    ) -> Result<ItemCommandExecution, DaoError> {
        ItemCommandPlacementExecutor::place_wall_item(
            item_dao,
            item_id,
            room_id,
            room_owner_id,
            wall_position,
            has_rights,
            all_super_user,
        )
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
        ItemCommandPlacementExecutor::place_floor_item(
            item_dao,
            item_id,
            room_id,
            room_owner_id,
            x,
            y,
            has_rights,
            all_super_user,
        )
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
        ItemCommandPlacementExecutor::move_stuff(
            item_dao,
            item_id,
            x,
            y,
            rotation,
            dir_rotation,
            room_id,
            has_rights,
            all_super_user,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemCommandExecution {
    Updated(Item),
    StuffDataUpdated(Item),
    RuntimeUpdated(Item),
    RoomItemPlaced(Item),
    RoomItemMoved(Item),
    RoomItemDeleted(Item),
    RoomItemReturned(Item),
    Deleted { item_id: i32 },
    Ignored,
}

fn normalise_stuff_data(data_class: &str, custom_data: &str) -> Option<String> {
    match data_class {
        "NULL" | "DIR" => None,
        "DOOROPEN" => Some(if custom_data == "TRUE" {
            "TRUE"
        } else {
            "FALSE"
        }),
        "SWITCHON" | "FIREON" => Some(if custom_data == "ON" { "ON" } else { "OFF" }),
        "STATUS" => Some(if custom_data == "O" { "O" } else { "C" }),
        _ => Some(custom_data),
    }
    .map(str::to_owned)
}

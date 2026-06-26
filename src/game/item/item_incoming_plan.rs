use crate::dao::{DaoError, ItemDao};
use crate::game::item::{ItemCommandExecution, ItemCommandExecutor};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemIncomingPlan;

impl ItemIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        item_dao: &dyn ItemDao,
        room_id: i32,
        room_owner_id: i32,
        inventory_owner_id: i32,
        has_rights: bool,
        has_owner_rights: bool,
        all_super_user: bool,
        move_dir_rotation: Option<i32>,
    ) -> Result<Vec<ItemCommandExecution>, DaoError> {
        let execution = match effect {
            IncomingExecutionEffect::SetItemData { item_id, data } => {
                ItemCommandExecutor::set_item_data(item_dao, *item_id, room_id, data, has_rights)?
            }
            IncomingExecutionEffect::SetStuffData {
                item_id,
                data_class,
                custom_data,
            } => ItemCommandExecutor::set_stuff_data(
                item_dao,
                *item_id,
                room_id,
                data_class,
                custom_data,
            )?,
            IncomingExecutionEffect::UseStripItem { item_id } => {
                ItemCommandExecutor::use_strip_item(item_dao, *item_id)?
            }
            IncomingExecutionEffect::RemoveItem { item_id } => {
                ItemCommandExecutor::remove_item(item_dao, *item_id, room_id, has_owner_rights)?
            }
            IncomingExecutionEffect::ReturnItemToInventory { item_id } => {
                ItemCommandExecutor::return_item_to_inventory(
                    item_dao,
                    *item_id,
                    room_id,
                    inventory_owner_id,
                    has_owner_rights,
                )?
            }
            IncomingExecutionEffect::PlaceWallItemFromInventory {
                item_id,
                wall_position,
            } => ItemCommandExecutor::place_wall_item(
                item_dao,
                *item_id,
                room_id,
                room_owner_id,
                wall_position,
                has_rights,
                all_super_user,
            )?,
            IncomingExecutionEffect::PlaceFloorItemFromInventory { item_id, x, y, .. } => {
                ItemCommandExecutor::place_floor_item(
                    item_dao,
                    *item_id,
                    room_id,
                    room_owner_id,
                    *x,
                    *y,
                    has_rights,
                    all_super_user,
                )?
            }
            IncomingExecutionEffect::MoveStuff {
                item_id,
                x,
                y,
                rotation,
            } => ItemCommandExecutor::move_stuff(
                item_dao,
                *item_id,
                *x,
                *y,
                *rotation,
                move_dir_rotation,
                room_id,
                has_rights,
                all_super_user,
            )?,
            _ => return Ok(Vec::new()),
        };

        Ok(vec![execution])
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        item_dao: &dyn ItemDao,
        room_id: i32,
        room_owner_id: i32,
        inventory_owner_id: i32,
        has_rights: bool,
        has_owner_rights: bool,
        all_super_user: bool,
        move_dir_rotation: Option<i32>,
    ) -> Result<Vec<ItemCommandExecution>, DaoError> {
        let mut executions = Vec::new();

        for effect in effects {
            executions.extend(Self::plan(
                effect,
                item_dao,
                room_id,
                room_owner_id,
                inventory_owner_id,
                has_rights,
                has_owner_rights,
                all_super_user,
                move_dir_rotation,
            )?);
        }

        Ok(executions)
    }
}

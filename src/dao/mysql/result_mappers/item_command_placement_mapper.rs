use std::collections::HashMap;

use crate::dao::mysql::{
    ItemCommandQueries, ItemResultMapper, SqlExecutionPlan, SqlExecutionResult,
};
use crate::dao::DaoError;
use crate::game::item::{Item, ItemDefinition};
use crate::game::room::model::Position;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemCommandPlacementMapper;

impl ItemCommandPlacementMapper {
    pub fn move_stuff_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
        dir_rotation: Option<i32>,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        let IncomingExecutionEffect::MoveStuff { x, y, rotation, .. } = effect else {
            return Ok(None);
        };

        let Some(mut item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(None);
        };
        if !item.definition().behaviour().is_on_floor() {
            return Ok(None);
        }

        apply_move_stuff(&mut item, *x, *y, *rotation, dir_rotation);
        Ok(ItemCommandQueries::moved_item_plan(&item))
    }

    pub fn place_item_from_inventory_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
        room_id: i32,
        owner_id: i32,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        let Some(mut item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(None);
        };

        match effect {
            IncomingExecutionEffect::PlaceWallItemFromInventory { wall_position, .. } => {
                if !item.definition().behaviour().is_on_wall() {
                    return Ok(None);
                }

                let wall_position = item
                    .custom_data()
                    .map(|custom_data| wall_position.replace(&format!("/{custom_data}"), ""))
                    .unwrap_or_else(|| wall_position.clone());
                item.set_wall_position(wall_position);
                item.set_room_id(room_id);
                item.set_owner_id(owner_id);
                apply_dir_custom_data(&mut item);

                Ok(Some(ItemCommandQueries::place_wall_item_plan(
                    item.id(),
                    room_id,
                    owner_id,
                    item.wall_position().unwrap_or_default(),
                    item.custom_data().unwrap_or_default(),
                )))
            }
            IncomingExecutionEffect::PlaceFloorItemFromInventory { x, y, rotation, .. } => {
                if !item.definition().behaviour().is_on_floor() {
                    return Ok(None);
                }

                item.set_room_id(room_id);
                item.set_owner_id(owner_id);
                *item.position_mut() =
                    Position::with_rotation(*x, *y, item.position().z(), *rotation);
                apply_dir_custom_data(&mut item);

                Ok(Some(ItemCommandQueries::place_floor_item_plan(
                    item.id(),
                    room_id,
                    owner_id,
                    item.position().x(),
                    item.position().y(),
                    item.position().z(),
                    item.position().rotation(),
                    item.custom_data().unwrap_or_default(),
                )))
            }
            _ => Ok(None),
        }
    }
}

fn apply_dir_custom_data(item: &mut Item) {
    if item.definition().data_class() == "DIR" {
        item.set_custom_data(item.position().rotation().to_string());
    }
}

fn apply_move_stuff(
    item: &mut Item,
    x: i32,
    y: i32,
    rotation: Option<i32>,
    dir_rotation: Option<i32>,
) {
    let rotation = rotation.unwrap_or_else(|| item.position().rotation());
    *item.position_mut() = Position::with_rotation(x, y, item.position().z(), rotation);
    item.set_custom_data("");

    if item.definition().data_class() == "DIR" {
        let rotation = dir_rotation.unwrap_or(rotation).clamp(0, 6);
        *item.position_mut() = Position::with_rotation(
            item.position().x(),
            item.position().y(),
            item.position().z(),
            rotation,
        );
        item.set_custom_data(rotation.to_string());
    }
}

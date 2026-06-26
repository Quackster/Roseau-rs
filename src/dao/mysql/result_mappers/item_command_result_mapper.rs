use std::collections::HashMap;

use crate::dao::mysql::{
    ItemCommandDataMapper, ItemCommandPlacementMapper, ItemCommandQueries, ItemResultMapper,
    SqlExecutionPlan, SqlExecutionResult,
};
use crate::dao::DaoError;
use crate::game::item::ItemDefinition;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemCommandResultMapper;

impl ItemCommandResultMapper {
    pub fn move_stuff_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
        dir_rotation: Option<i32>,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        ItemCommandPlacementMapper::move_stuff_plan(result, definitions, effect, dir_rotation)
    }

    pub fn use_strip_item_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        ItemCommandDataMapper::use_strip_item_plan(result, definitions, effect)
    }

    pub fn set_item_data_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        ItemCommandDataMapper::set_item_data_plan(result, definitions, effect)
    }

    pub fn set_stuff_data_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        ItemCommandDataMapper::set_stuff_data_plan(result, definitions, effect)
    }

    pub fn remove_item_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        if !matches!(effect, IncomingExecutionEffect::RemoveItem { .. }) {
            return Ok(None);
        }

        let Some(item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(None);
        };

        Ok(Some(ItemCommandQueries::remove_item_plan(item.id())))
    }

    pub fn return_item_to_inventory_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
        owner_id: i32,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        if !matches!(
            effect,
            IncomingExecutionEffect::ReturnItemToInventory { .. }
        ) {
            return Ok(None);
        }

        let Some(item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(None);
        };

        if item.definition().behaviour().is_on_floor() {
            return Ok(Some(ItemCommandQueries::return_floor_item_plan(
                item.id(),
                owner_id,
            )));
        }

        if item.definition().behaviour().is_on_wall() {
            return Ok(Some(ItemCommandQueries::return_wall_item_plan(item.id())));
        }

        Ok(None)
    }

    pub fn place_item_from_inventory_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
        room_id: i32,
        owner_id: i32,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        ItemCommandPlacementMapper::place_item_from_inventory_plan(
            result,
            definitions,
            effect,
            room_id,
            owner_id,
        )
    }
}

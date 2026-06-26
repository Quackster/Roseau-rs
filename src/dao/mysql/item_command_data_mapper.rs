use std::collections::HashMap;

use crate::dao::mysql::{
    ItemCommandQueries, ItemResultMapper, SqlExecutionPlan, SqlExecutionResult,
};
use crate::dao::DaoError;
use crate::game::item::ItemDefinition;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemCommandDataMapper;

impl ItemCommandDataMapper {
    pub fn use_strip_item_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        if !matches!(effect, IncomingExecutionEffect::UseStripItem { .. }) {
            return Ok(None);
        }

        let Some(item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(None);
        };
        if !item.definition().behaviour().is_post_it() {
            return Ok(None);
        }

        let current_amount = item
            .custom_data()
            .and_then(|amount| amount.parse::<i32>().ok())
            .unwrap_or(0);
        Ok(ItemCommandQueries::use_post_it_plan(
            item.id(),
            current_amount,
        ))
    }

    pub fn set_item_data_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        let IncomingExecutionEffect::SetItemData { data, .. } = effect else {
            return Ok(None);
        };

        let Some(item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(None);
        };

        Ok(ItemCommandQueries::set_item_data_plan(
            item.id(),
            item.custom_data(),
            data,
        ))
    }

    pub fn set_stuff_data_plan(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
    ) -> Result<Option<SqlExecutionPlan>, DaoError> {
        let IncomingExecutionEffect::SetStuffData { custom_data, .. } = effect else {
            return Ok(None);
        };

        let Some(item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(None);
        };

        Ok(ItemCommandQueries::set_stuff_data_plan(
            item.id(),
            item.definition().data_class(),
            custom_data,
        ))
    }
}

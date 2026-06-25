use std::collections::HashMap;

use crate::dao::mysql::{
    ItemCommandQueries, ItemResultMapper, RoomCommandQueries, SqlExecutionPlan, SqlExecutionResult,
};
use crate::dao::DaoError;
use crate::game::item::ItemDefinition;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecorationCommandResultMapper;

impl DecorationCommandResultMapper {
    pub fn plans(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
        effect: &IncomingExecutionEffect,
        room_id: i32,
    ) -> Result<Vec<SqlExecutionPlan>, DaoError> {
        if !matches!(effect, IncomingExecutionEffect::ApplyDecoration { .. }) {
            return Ok(Vec::new());
        }

        let Some(item) = ItemResultMapper::optional_item(result, definitions)? else {
            return Ok(Vec::new());
        };
        if !item.definition().behaviour().is_decoration() {
            return Ok(Vec::new());
        }

        let Some(decoration_plan) = RoomCommandQueries::apply_decoration_plan(
            effect,
            room_id,
            item.custom_data().unwrap_or_default(),
        ) else {
            return Ok(Vec::new());
        };

        Ok(vec![
            ItemCommandQueries::remove_item_plan(item.id()),
            decoration_plan,
        ])
    }
}

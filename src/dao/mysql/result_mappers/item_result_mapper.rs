use std::collections::HashMap;

use crate::dao::mysql::entity::{ItemDefinitionRow, ItemRow, RoomPublicItemRow};
use crate::dao::mysql::mapper::{item_definition_from_row, item_from_row, public_item_from_row};
use crate::dao::mysql::SqlExecutionResult;
use crate::dao::DaoError;
use crate::game::item::{Item, ItemDefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemResultMapper;

impl ItemResultMapper {
    pub fn definitions(
        result: SqlExecutionResult,
    ) -> Result<HashMap<i32, ItemDefinition>, DaoError> {
        Ok(result
            .map_rows(|row| {
                let definition_row = ItemDefinitionRow::try_from(row)?;
                let definition = item_definition_from_row(&definition_row);
                Ok((definition.id(), definition))
            })?
            .into_iter()
            .collect())
    }

    pub fn room_items(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
    ) -> Result<HashMap<i32, Item>, DaoError> {
        Ok(result
            .map_rows(|row| {
                let item_row = ItemRow::try_from(row)?;
                let definition = definitions
                    .get(&item_row.item_id)
                    .cloned()
                    .ok_or_else(|| missing_definition(item_row.item_id))?;
                let item = item_from_row(&item_row, definition)
                    .map_err(|error| DaoError::new(error.to_string()))?;
                Ok((item.id(), item))
            })?
            .into_iter()
            .collect())
    }

    pub fn public_room_items(
        result: SqlExecutionResult,
        room_id: i32,
        definitions: &HashMap<i32, ItemDefinition>,
    ) -> Result<HashMap<i32, Item>, DaoError> {
        let mut items = HashMap::new();

        for row in result.require_rows()? {
            let item_row = RoomPublicItemRow::try_from(&row)?;
            let Some(definition) = definitions.get(&item_row.definition_id).cloned() else {
                continue;
            };
            let Ok(item) = public_item_from_row(&item_row, room_id, definition) else {
                continue;
            };
            items.insert(item.id(), item);
        }

        Ok(items)
    }

    pub fn optional_item(
        result: SqlExecutionResult,
        definitions: &HashMap<i32, ItemDefinition>,
    ) -> Result<Option<Item>, DaoError> {
        let mut items = Self::room_items(result, definitions)?
            .into_values()
            .collect::<Vec<_>>();
        items.sort_by_key(Item::id);
        Ok(items.into_iter().next())
    }
}

fn missing_definition(definition_id: i32) -> DaoError {
    DaoError::new(format!("Missing item definition {definition_id}"))
}

#[cfg(test)]
#[path = "item_result_mapper_tests.rs"]
mod tests;

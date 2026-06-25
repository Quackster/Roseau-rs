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
        Ok(result
            .map_rows(|row| {
                let item_row = RoomPublicItemRow::try_from(row)?;
                let definition = definitions
                    .get(&item_row.definition_id)
                    .cloned()
                    .ok_or_else(|| missing_definition(item_row.definition_id))?;
                let item = public_item_from_row(&item_row, room_id, definition)
                    .map_err(|error| DaoError::new(error.to_string()))?;
                Ok((item.id(), item))
            })?
            .into_iter()
            .collect())
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
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlRow, SqlValue};

    fn definition_row(id: i32, sprite: &str, behaviour: &str) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(id)),
            ("sprite", SqlValue::Text(sprite.to_owned())),
            ("color", SqlValue::Text("red".to_owned())),
            ("length", SqlValue::Integer(1)),
            ("width", SqlValue::Integer(1)),
            ("height", SqlValue::Float(1.0)),
            ("dataclass", SqlValue::Text(String::new())),
            ("behaviour", SqlValue::Text(behaviour.to_owned())),
            ("name", SqlValue::Text("Name".to_owned())),
            ("description", SqlValue::Text("Desc".to_owned())),
        ])
    }

    fn item_row(id: i32, definition_id: i32, x: &str) -> SqlRow {
        SqlRow::new([
            ("id", SqlValue::Integer(id)),
            ("user_id", SqlValue::Integer(7)),
            ("item_id", SqlValue::Integer(definition_id)),
            ("room_id", SqlValue::Integer(42)),
            ("x", SqlValue::Text(x.to_owned())),
            ("y", SqlValue::Integer(2)),
            ("z", SqlValue::Float(0.5)),
            ("rotation", SqlValue::Integer(4)),
            ("extra_data", SqlValue::Text("ON".to_owned())),
        ])
    }

    #[test]
    fn maps_definition_rows_to_id_map() {
        let result = SqlExecutionResult::rows([
            definition_row(5, "chair", "SIT"),
            definition_row(6, "table", ""),
        ]);

        let definitions = ItemResultMapper::definitions(result).unwrap();

        assert_eq!(definitions.len(), 2);
        assert_eq!(definitions[&5].sprite(), "chair");
        assert_eq!(definitions[&6].id(), 6);
    }

    #[test]
    fn maps_room_item_rows_using_definitions() {
        let definitions =
            ItemResultMapper::definitions(SqlExecutionResult::rows([definition_row(
                5, "chair", "",
            )]))
            .unwrap();
        let result = SqlExecutionResult::rows([item_row(10, 5, "1")]);

        let items = ItemResultMapper::room_items(result, &definitions).unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[&10].definition().id(), 5);
        assert_eq!(items[&10].position().x(), 1);
    }

    #[test]
    fn maps_public_room_item_rows_using_room_id_and_definitions() {
        let definitions =
            ItemResultMapper::definitions(SqlExecutionResult::rows([definition_row(
                8, "chair", "",
            )]))
            .unwrap();
        let result = SqlExecutionResult::rows([SqlRow::new([
            ("id", SqlValue::Integer(20)),
            ("model", SqlValue::Text("pool_a".to_owned())),
            ("x", SqlValue::Text("4".to_owned())),
            ("y", SqlValue::Integer(5)),
            ("z", SqlValue::Float(1.0)),
            ("rotation", SqlValue::Integer(2)),
            ("definitionid", SqlValue::Integer(8)),
            ("object", SqlValue::Text("chair".to_owned())),
            ("data", SqlValue::Text("ON".to_owned())),
        ])]);

        let items = ItemResultMapper::public_room_items(result, 77, &definitions).unwrap();

        assert_eq!(items[&20].room_id(), 77);
        assert_eq!(items[&20].custom_data(), Some("ON"));
    }

    #[test]
    fn maps_optional_item_from_first_sorted_row() {
        let definitions =
            ItemResultMapper::definitions(SqlExecutionResult::rows([definition_row(
                5, "chair", "",
            )]))
            .unwrap();
        let result = SqlExecutionResult::rows([item_row(11, 5, "2"), item_row(10, 5, "1")]);

        let item = ItemResultMapper::optional_item(result, &definitions)
            .unwrap()
            .unwrap();

        assert_eq!(item.id(), 10);
    }

    #[test]
    fn reports_missing_definitions_and_invalid_item_coordinates() {
        let missing_definition = ItemResultMapper::room_items(
            SqlExecutionResult::rows([item_row(10, 99, "1")]),
            &HashMap::new(),
        )
        .unwrap_err();
        assert_eq!(missing_definition.message(), "Missing item definition 99");

        let definitions =
            ItemResultMapper::definitions(SqlExecutionResult::rows([definition_row(
                5, "chair", "",
            )]))
            .unwrap();
        let invalid_coordinate = ItemResultMapper::room_items(
            SqlExecutionResult::rows([item_row(10, 5, "x")]),
            &definitions,
        )
        .unwrap_err();
        assert_eq!(invalid_coordinate.message(), "item x coordinate is invalid");
    }
}

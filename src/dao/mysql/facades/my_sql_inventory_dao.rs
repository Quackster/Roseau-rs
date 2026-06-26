use std::collections::HashMap;

use crate::dao::mysql::{
    InventoryQueries, ItemResultMapper, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::{DaoError, InventoryDao};
use crate::game::item::{Item, ItemDefinition};

#[derive(Debug)]
pub struct MySqlInventoryDao<E> {
    executor: E,
    definitions: HashMap<i32, ItemDefinition>,
}

impl<E> MySqlInventoryDao<E> {
    pub fn new(executor: E, definitions: HashMap<i32, ItemDefinition>) -> Self {
        Self {
            executor,
            definitions,
        }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn definitions(&self) -> &HashMap<i32, ItemDefinition> {
        &self.definitions
    }
}

impl<E: SqlExecutor> MySqlInventoryDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }

    fn definition(&self, item_id: i32) -> Result<ItemDefinition, DaoError> {
        self.definitions
            .get(&item_id)
            .cloned()
            .ok_or_else(|| DaoError::new(format!("Missing item definition {item_id}")))
    }
}

impl<E: SqlExecutor> InventoryDao for MySqlInventoryDao<E> {
    fn inventory_items(&self, user_id: i32) -> Result<Vec<Item>, DaoError> {
        let result = self.execute_plan(InventoryQueries::inventory_items(user_id).read_plan())?;
        let mut items = ItemResultMapper::room_items(result, &self.definitions)?
            .into_values()
            .collect::<Vec<_>>();
        items.sort_by_key(Item::id);
        Ok(items)
    }

    fn item(&self, id: i64) -> Result<Option<Item>, DaoError> {
        let result = self.execute_plan(InventoryQueries::item(id).read_plan())?;
        ItemResultMapper::optional_item(result, &self.definitions)
    }

    fn new_item(&self, item_id: i32, owner_id: i32, extra_data: &str) -> Result<Item, DaoError> {
        let definition = self.definition(item_id)?;
        let result = self.execute_plan(
            InventoryQueries::create_item(item_id, owner_id, extra_data).insert_returning_id_plan(),
        )?;
        let id = result.require_i32_insert_id("inventory item id")?;

        Item::new(
            id,
            0,
            owner_id,
            "0",
            0,
            0.0,
            0,
            definition,
            "",
            Some(extra_data.to_owned()),
        )
        .map_err(|error| DaoError::new(error.to_string()))
    }
}

#[cfg(test)]
#[path = "my_sql_inventory_dao_tests.rs"]
mod tests;

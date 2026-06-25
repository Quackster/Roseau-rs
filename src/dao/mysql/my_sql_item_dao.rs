use std::collections::HashMap;

use crate::dao::mysql::{
    ItemQueries, ItemResultMapper, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::{DaoError, ItemDao};
use crate::game::item::{Item, ItemDefinition};

#[derive(Debug)]
pub struct MySqlItemDao<E> {
    executor: E,
    definitions: HashMap<i32, ItemDefinition>,
}

impl<E> MySqlItemDao<E> {
    pub fn new(executor: E, definitions: HashMap<i32, ItemDefinition>) -> Self {
        Self {
            executor,
            definitions,
        }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn definitions_cache(&self) -> &HashMap<i32, ItemDefinition> {
        &self.definitions
    }
}

impl<E: SqlExecutor> MySqlItemDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }

    fn execute_mutation(&self, plan: SqlExecutionPlan) -> Result<(), DaoError> {
        self.execute_plan(plan)?.require_mutation()
    }
}

impl<E: SqlExecutor> ItemDao for MySqlItemDao<E> {
    fn definitions(&self) -> Result<HashMap<i32, ItemDefinition>, DaoError> {
        let result = self.execute_plan(ItemQueries::definitions().read_plan())?;
        ItemResultMapper::definitions(result)
    }

    fn public_room_items(&self, model: &str, room_id: i32) -> Result<HashMap<i32, Item>, DaoError> {
        let result = self.execute_plan(ItemQueries::public_room_items(model).read_plan())?;
        ItemResultMapper::public_room_items(result, room_id, &self.definitions)
    }

    fn room_items(&self, room_id: i32) -> Result<HashMap<i32, Item>, DaoError> {
        let result = self.execute_plan(ItemQueries::room_items(room_id).read_plan())?;
        ItemResultMapper::room_items(result, &self.definitions)
    }

    fn save_item(&self, item: &Item) -> Result<(), DaoError> {
        self.execute_mutation(ItemQueries::save_item(item).execute_plan())
    }

    fn delete_item(&self, id: i64) -> Result<(), DaoError> {
        self.execute_mutation(ItemQueries::delete_item(id).execute_plan())
    }

    fn item(&self, item_id: i32) -> Result<Option<Item>, DaoError> {
        let result = self.execute_plan(ItemQueries::item(item_id).read_plan())?;
        ItemResultMapper::optional_item(result, &self.definitions)
    }
}

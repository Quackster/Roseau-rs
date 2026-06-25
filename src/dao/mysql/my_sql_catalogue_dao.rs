use std::collections::HashMap;

use crate::dao::mysql::{
    CatalogueQueries, CatalogueResultMapper, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::{CatalogueDao, DaoError};
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

#[derive(Debug)]
pub struct MySqlCatalogueDao<E> {
    executor: E,
}

impl<E> MySqlCatalogueDao<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }
}

impl<E: SqlExecutor> MySqlCatalogueDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }
}

impl<E: SqlExecutor> CatalogueDao for MySqlCatalogueDao<E> {
    fn buyable_items(&self) -> Result<HashMap<String, CatalogueItem>, DaoError> {
        let result = self.execute_plan(CatalogueQueries::buyable_items().read_plan())?;
        CatalogueResultMapper::buyable_items(result)
    }

    fn item_deals(&self) -> Result<HashMap<String, CatalogueDeal>, DaoError> {
        let result = self.execute_plan(CatalogueQueries::item_deals().read_plan())?;
        CatalogueResultMapper::item_deals(result)
    }
}

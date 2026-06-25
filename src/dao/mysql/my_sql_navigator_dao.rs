use crate::dao::mysql::{
    NavigatorQueries, NavigatorResultMapper, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::{DaoError, NavigatorDao};
use crate::game::room::RoomData;

#[derive(Debug)]
pub struct MySqlNavigatorDao<E> {
    executor: E,
    owner_name: String,
}

impl<E> MySqlNavigatorDao<E> {
    pub fn new(executor: E, owner_name: impl Into<String>) -> Self {
        Self {
            executor,
            owner_name: owner_name.into(),
        }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn owner_name(&self) -> &str {
        &self.owner_name
    }

    pub fn set_owner_name(&mut self, owner_name: impl Into<String>) {
        self.owner_name = owner_name.into();
    }
}

impl<E: SqlExecutor> MySqlNavigatorDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }
}

impl<E: SqlExecutor> NavigatorDao for MySqlNavigatorDao<E> {
    fn rooms_by_like_name(&self, name: &str) -> Result<Vec<RoomData>, DaoError> {
        let result = self.execute_plan(NavigatorQueries::rooms_by_like_name(name).read_plan())?;
        NavigatorResultMapper::rooms_by_like_name(result, &self.owner_name)
    }
}

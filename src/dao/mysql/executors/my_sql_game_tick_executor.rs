use crate::dao::mysql::{
    GameTickQueries, SqlBatchExecutor, SqlExecutionBatchResult, SqlExecutionPlan, SqlExecutor,
};
use crate::dao::DaoError;
use crate::game::GameTickEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySqlGameTickExecutor<E> {
    batch_executor: SqlBatchExecutor<E>,
}

impl<E> MySqlGameTickExecutor<E> {
    pub fn new(executor: E) -> Self {
        Self {
            batch_executor: SqlBatchExecutor::new(executor),
        }
    }

    pub fn batch_executor(&self) -> &SqlBatchExecutor<E> {
        &self.batch_executor
    }

    pub fn into_executor(self) -> E {
        self.batch_executor.into_executor()
    }
}

impl<E: SqlExecutor> MySqlGameTickExecutor<E> {
    pub fn execution_plans(&self, effects: &[GameTickEffect]) -> Vec<SqlExecutionPlan> {
        effects.iter().filter_map(GameTickQueries::plan).collect()
    }

    pub fn execute_effects(
        &self,
        effects: &[GameTickEffect],
    ) -> Result<SqlExecutionBatchResult, DaoError> {
        let plans = self.execution_plans(effects);
        let result = self.batch_executor.execute_all(&plans)?;
        result
            .validate_for(&plans)
            .map(SqlExecutionBatchResult::new)
    }
}

#[cfg(test)]
#[path = "my_sql_game_tick_executor_tests.rs"]
mod tests;

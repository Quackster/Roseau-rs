use crate::dao::mysql::{SqlExecutionBatchResult, SqlExecutionPlan, SqlExecutor};
use crate::dao::DaoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlBatchExecutor<E> {
    executor: E,
}

impl<E> SqlBatchExecutor<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn into_executor(self) -> E {
        self.executor
    }
}

impl<E: SqlExecutor> SqlBatchExecutor<E> {
    pub fn execute_all(
        &self,
        plans: &[SqlExecutionPlan],
    ) -> Result<SqlExecutionBatchResult, DaoError> {
        let mut results = Vec::with_capacity(plans.len());
        for plan in plans {
            results.push(self.executor.execute(plan.clone())?);
        }

        Ok(SqlExecutionBatchResult::new(results))
    }
}

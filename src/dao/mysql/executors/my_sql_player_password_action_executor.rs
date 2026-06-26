use crate::dao::mysql::{
    MySqlPlayerPasswordActionExecutionReport, MySqlPlayerPasswordActionReport, SqlBatchExecutor,
    SqlExecutionBatchResult, SqlExecutor,
};
use crate::dao::DaoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySqlPlayerPasswordActionExecutor<E> {
    batch_executor: SqlBatchExecutor<E>,
}

impl<E> MySqlPlayerPasswordActionExecutor<E> {
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

impl<E: SqlExecutor> MySqlPlayerPasswordActionExecutor<E> {
    pub fn execute_report(
        &self,
        report: &MySqlPlayerPasswordActionReport,
    ) -> Result<SqlExecutionBatchResult, DaoError> {
        let result = self
            .batch_executor
            .execute_all(report.persistence_plans())?;
        result
            .validate_for(report.persistence_plans())
            .map(SqlExecutionBatchResult::new)
    }

    pub fn execute_password_report(
        &self,
        report: MySqlPlayerPasswordActionReport,
    ) -> Result<MySqlPlayerPasswordActionExecutionReport, DaoError> {
        let database_result = self.execute_report(&report)?;
        Ok(MySqlPlayerPasswordActionExecutionReport::new(
            report,
            database_result,
        ))
    }
}

#[cfg(test)]
#[path = "my_sql_player_password_action_executor_tests.rs"]
mod tests;

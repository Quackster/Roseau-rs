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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionResult, SqlParameter, SqlQuery};
    use std::cell::RefCell;
    use std::collections::VecDeque;

    #[derive(Debug, Default)]
    struct RecordingExecutor {
        plans: RefCell<Vec<SqlExecutionPlan>>,
        results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
    }

    impl RecordingExecutor {
        fn push_result(&self, result: SqlExecutionResult) {
            self.results.borrow_mut().push_back(Ok(result));
        }

        fn push_error(&self, message: &str) {
            self.results
                .borrow_mut()
                .push_back(Err(DaoError::new(message)));
        }

        fn plans(&self) -> Vec<SqlExecutionPlan> {
            self.plans.borrow().clone()
        }
    }

    impl SqlExecutor for RecordingExecutor {
        fn execute(
            &self,
            plan: SqlExecutionPlan,
        ) -> Result<crate::dao::mysql::SqlExecutionResult, DaoError> {
            self.plans.borrow_mut().push(plan);
            self.results
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| Err(DaoError::new("missing executor result")))
        }
    }

    fn insert_plan(id: i32) -> SqlExecutionPlan {
        SqlExecutionPlan::insert_returning_id(SqlQuery::new(
            "INSERT INTO items (item_id) VALUES (?)",
            [SqlParameter::Integer(id)],
        ))
    }

    fn update_plan() -> SqlExecutionPlan {
        SqlExecutionPlan::execute(SqlQuery::new("UPDATE users SET credits = 0", []))
    }

    #[test]
    fn executes_plans_in_order() {
        let executor = RecordingExecutor::default();
        executor.push_result(SqlExecutionResult::insert_id(100));
        executor.push_result(SqlExecutionResult::affected_rows(1));
        let batch = SqlBatchExecutor::new(executor);
        let plans = [insert_plan(7), update_plan()];

        let result = batch.execute_all(&plans).unwrap();

        assert_eq!(batch.executor().plans(), plans);
        assert_eq!(
            result.into_results(),
            vec![
                SqlExecutionResult::InsertId(100),
                SqlExecutionResult::AffectedRows(1)
            ]
        );
    }

    #[test]
    fn stops_on_first_executor_error() {
        let executor = RecordingExecutor::default();
        executor.push_result(SqlExecutionResult::insert_id(100));
        executor.push_error("write failed");
        let batch = SqlBatchExecutor::new(executor);
        let plans = [insert_plan(7), insert_plan(8), update_plan()];

        let error = batch.execute_all(&plans).unwrap_err();

        assert_eq!(error.message(), "write failed");
        assert_eq!(batch.executor().plans(), plans[..2]);
    }

    #[test]
    fn exposes_inner_executor_for_facade_ownership() {
        let executor = RecordingExecutor::default();
        let batch = SqlBatchExecutor::new(executor);

        assert_eq!(
            batch.into_executor().plans(),
            Vec::<SqlExecutionPlan>::new()
        );
    }
}

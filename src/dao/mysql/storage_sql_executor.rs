use crate::dao::mysql::{SqlDriver, SqlExecutionPlan, SqlExecutionResult, SqlExecutor};
use crate::dao::DaoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageSqlExecutor<D> {
    driver: D,
}

impl<D> StorageSqlExecutor<D> {
    pub fn new(driver: D) -> Self {
        Self { driver }
    }

    pub fn driver(&self) -> &D {
        &self.driver
    }

    pub fn into_driver(self) -> D {
        self.driver
    }
}

impl<D: SqlDriver> SqlExecutor for StorageSqlExecutor<D> {
    fn execute(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        plan.validate_parameters()?;
        let result = self.driver.execute_plan(&plan)?;
        plan.validate_result(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlParameter, SqlQuery, SqlRow, SqlValue};
    use std::cell::RefCell;
    use std::collections::VecDeque;

    #[derive(Debug, Default)]
    struct RecordingDriver {
        plans: RefCell<Vec<SqlExecutionPlan>>,
        results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
    }

    impl RecordingDriver {
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

    impl SqlDriver for RecordingDriver {
        fn execute_plan(&self, plan: &SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
            self.plans.borrow_mut().push(plan.clone());
            self.results
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| Err(DaoError::new("missing driver result")))
        }
    }

    #[test]
    fn forwards_owned_execution_plan_to_driver() {
        let driver = RecordingDriver::default();
        driver.push_result(SqlExecutionResult::rows([SqlRow::new([(
            "id",
            SqlValue::Integer(7),
        )])]));
        let executor = StorageSqlExecutor::new(driver);
        let plan = SqlExecutionPlan::read_rows(SqlQuery::new(
            "SELECT * FROM users WHERE id = ?",
            [SqlParameter::Integer(7)],
        ));

        let result = executor.execute(plan.clone()).unwrap();

        assert_eq!(executor.driver().plans(), vec![plan]);
        assert_eq!(
            result.require_rows().unwrap()[0]
                .required_i32("id")
                .unwrap(),
            7
        );
    }

    #[test]
    fn rejects_driver_result_that_does_not_match_plan_kind() {
        let driver = RecordingDriver::default();
        driver.push_result(SqlExecutionResult::insert_id(9));
        let executor = StorageSqlExecutor::new(driver);
        let plan = SqlExecutionPlan::execute(SqlQuery::new(
            "DELETE FROM users WHERE id = ?",
            [SqlParameter::Integer(7)],
        ));

        let error = executor.execute(plan).unwrap_err();

        assert_eq!(
            error.message(),
            "SQL execution kind Execute returned insert id result"
        );
    }

    #[test]
    fn preserves_driver_errors() {
        let driver = RecordingDriver::default();
        driver.push_error("database unavailable");
        let executor = StorageSqlExecutor::new(driver);
        let plan = SqlExecutionPlan::insert_returning_id(SqlQuery::new(
            "INSERT INTO users (username) VALUES (?)",
            [SqlParameter::Text("alice".to_owned())],
        ));

        assert_eq!(
            executor.execute(plan).unwrap_err().message(),
            "database unavailable"
        );
    }

    #[test]
    fn rejects_parameter_mismatches_before_calling_driver() {
        let driver = RecordingDriver::default();
        let executor = StorageSqlExecutor::new(driver);
        let plan = SqlExecutionPlan::execute(SqlQuery::new(
            "UPDATE users SET credits = ? WHERE id = ?",
            [SqlParameter::Integer(100)],
        ));

        let error = executor.execute(plan).unwrap_err();

        assert_eq!(
            error.message(),
            "SQL parameter count mismatch: `UPDATE users SET credits = ? WHERE id = ?` has 2 placeholders but 1 parameters"
        );
        assert!(executor.driver().plans().is_empty());
    }

    #[test]
    fn exposes_inner_driver_for_runtime_adapter_ownership() {
        let driver = RecordingDriver::default();
        let executor = StorageSqlExecutor::new(driver);

        assert_eq!(
            executor.into_driver().plans(),
            Vec::<SqlExecutionPlan>::new()
        );
    }
}

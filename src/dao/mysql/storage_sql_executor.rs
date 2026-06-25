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

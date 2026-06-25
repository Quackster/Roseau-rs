use crate::dao::mysql::{SqlDriver, SqlExecutionPlan, SqlExecutionResult};
use crate::dao::DaoError;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UnconfiguredSqlDriver;

impl UnconfiguredSqlDriver {
    pub fn new() -> Self {
        Self
    }
}

impl SqlDriver for UnconfiguredSqlDriver {
    fn execute_plan(&self, plan: &SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        Err(DaoError::new(format!(
            "SQL driver is not configured for {}",
            plan.redacted_diagnostic()
        )))
    }
}

#[cfg(test)]
#[path = "unconfigured_sql_driver_tests.rs"]
mod tests;

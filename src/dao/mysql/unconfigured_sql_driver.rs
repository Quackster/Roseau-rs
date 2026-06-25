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
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionPlan, SqlParameter, SqlQuery};

    #[test]
    fn reports_missing_database_driver_for_sql_execution() {
        let error = UnconfiguredSqlDriver::new()
            .execute_plan(&SqlExecutionPlan::execute(SqlQuery::new(
                "UPDATE users SET password = ?, credits = ? WHERE id = ?",
                [
                    SqlParameter::Text("hash".to_owned()),
                    SqlParameter::Integer(100),
                    SqlParameter::Long(7),
                ],
            )))
            .unwrap_err();

        assert_eq!(
            error.message(),
            "SQL driver is not configured for Execute `UPDATE users SET password = ?, credits = ? WHERE id = ?` with parameters [<text>, 100, 7]"
        );
    }
}

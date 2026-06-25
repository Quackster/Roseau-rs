use crate::dao::mysql::{MySqlPlayerPasswordActionReport, SqlExecutionBatchResult};

#[derive(Debug, Clone, PartialEq)]
pub struct MySqlPlayerPasswordActionExecutionReport {
    password_report: MySqlPlayerPasswordActionReport,
    database_result: SqlExecutionBatchResult,
}

impl MySqlPlayerPasswordActionExecutionReport {
    pub fn new(
        password_report: MySqlPlayerPasswordActionReport,
        database_result: SqlExecutionBatchResult,
    ) -> Self {
        Self {
            password_report,
            database_result,
        }
    }

    pub fn password_report(&self) -> &MySqlPlayerPasswordActionReport {
        &self.password_report
    }

    pub fn database_result(&self) -> &SqlExecutionBatchResult {
        &self.database_result
    }
}

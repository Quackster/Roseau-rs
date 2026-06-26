use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
use crate::dao::DaoError;

pub trait SqlDriver {
    fn execute_plan(&self, plan: &SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError>;
}

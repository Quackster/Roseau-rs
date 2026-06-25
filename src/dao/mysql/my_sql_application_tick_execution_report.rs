use crate::dao::mysql::SqlExecutionBatchResult;
use crate::game::{GameTickEffect, GameTickRuntimeEffect};

#[derive(Debug, Clone, PartialEq)]
pub struct MySqlApplicationTickExecutionReport {
    database_result: SqlExecutionBatchResult,
    runtime_effects: Vec<GameTickEffect>,
}

impl MySqlApplicationTickExecutionReport {
    pub fn new(
        database_result: SqlExecutionBatchResult,
        runtime_effects: impl Into<Vec<GameTickEffect>>,
    ) -> Self {
        Self {
            database_result,
            runtime_effects: runtime_effects.into(),
        }
    }

    pub fn database_result(&self) -> &SqlExecutionBatchResult {
        &self.database_result
    }

    pub fn runtime_effects(&self) -> &[GameTickEffect] {
        &self.runtime_effects
    }

    pub fn runtime_actions(&self) -> Vec<GameTickRuntimeEffect> {
        GameTickRuntimeEffect::collect(&self.runtime_effects)
    }
}

#[cfg(test)]
#[path = "my_sql_application_tick_execution_report_tests.rs"]
mod tests;

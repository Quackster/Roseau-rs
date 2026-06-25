use crate::dao::mysql::{
    MySqlApplicationTickExecutionReport, MySqlGameTickExecutor, SqlExecutionBatchResult,
    SqlExecutor,
};
use crate::dao::DaoError;
use crate::game::{GameTickEffect, GameTickRuntimeEffect};
use crate::runtime::RoseauApplicationTickOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySqlApplicationTickExecutor<E> {
    game_tick_executor: MySqlGameTickExecutor<E>,
}

impl<E> MySqlApplicationTickExecutor<E> {
    pub fn new(executor: E) -> Self {
        Self {
            game_tick_executor: MySqlGameTickExecutor::new(executor),
        }
    }

    pub fn game_tick_executor(&self) -> &MySqlGameTickExecutor<E> {
        &self.game_tick_executor
    }

    pub fn into_executor(self) -> E {
        self.game_tick_executor.into_executor()
    }
}

impl<E: SqlExecutor> MySqlApplicationTickExecutor<E> {
    pub fn execute_tick(
        &self,
        outcome: &RoseauApplicationTickOutcome,
    ) -> Result<SqlExecutionBatchResult, DaoError> {
        self.game_tick_executor
            .execute_effects(outcome.game_effects())
    }

    pub fn execute_tick_report(
        &self,
        outcome: &RoseauApplicationTickOutcome,
    ) -> Result<MySqlApplicationTickExecutionReport, DaoError> {
        let database_result = self.execute_tick(outcome)?;
        let runtime_effects = outcome
            .game_effects()
            .iter()
            .filter(|effect| GameTickRuntimeEffect::from_tick_effect(effect).is_some())
            .cloned()
            .collect::<Vec<GameTickEffect>>();

        Ok(MySqlApplicationTickExecutionReport::new(
            database_result,
            runtime_effects,
        ))
    }
}

use crate::dao::mysql::{
    GameTickQueries, SqlBatchExecutor, SqlExecutionBatchResult, SqlExecutionPlan, SqlExecutor,
};
use crate::dao::DaoError;
use crate::game::GameTickEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySqlGameTickExecutor<E> {
    batch_executor: SqlBatchExecutor<E>,
}

impl<E> MySqlGameTickExecutor<E> {
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

impl<E: SqlExecutor> MySqlGameTickExecutor<E> {
    pub fn execution_plans(&self, effects: &[GameTickEffect]) -> Vec<SqlExecutionPlan> {
        effects.iter().filter_map(GameTickQueries::plan).collect()
    }

    pub fn execute_effects(
        &self,
        effects: &[GameTickEffect],
    ) -> Result<SqlExecutionBatchResult, DaoError> {
        let plans = self.execution_plans(effects);
        let result = self.batch_executor.execute_all(&plans)?;
        result
            .validate_for(&plans)
            .map(SqlExecutionBatchResult::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionResult, SqlParameter};
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

        fn plans(&self) -> Vec<SqlExecutionPlan> {
            self.plans.borrow().clone()
        }
    }

    impl SqlExecutor for RecordingExecutor {
        fn execute(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
            self.plans.borrow_mut().push(plan);
            self.results
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| Err(DaoError::new("missing executor result")))
        }
    }

    #[test]
    fn executes_only_persistent_game_tick_effects() {
        let executor = RecordingExecutor::default();
        executor.push_result(SqlExecutionResult::affected_rows(1));
        let game_tick_executor = MySqlGameTickExecutor::new(executor);
        let effects = [
            GameTickEffect::AwardCredits {
                user_id: 7,
                amount: 10,
                new_balance: 125,
            },
            GameTickEffect::SavePlayer { user_id: 7 },
            GameTickEffect::ResolveServerIp,
            GameTickEffect::KickAfkUser { user_id: 8 },
        ];

        let result = game_tick_executor.execute_effects(&effects).unwrap();
        let executor = game_tick_executor.into_executor();

        assert_eq!(result.results(), &[SqlExecutionResult::AffectedRows(1)]);
        assert_eq!(executor.plans().len(), 1);
        assert_eq!(
            executor.plans()[0].sql(),
            "UPDATE users SET credits = ? WHERE id = ?"
        );
        assert_eq!(
            executor.plans()[0].parameters(),
            &[SqlParameter::Integer(125), SqlParameter::Integer(7)]
        );
    }

    #[test]
    fn returns_empty_batch_for_non_persistent_tick_effects() {
        let executor = RecordingExecutor::default();
        let game_tick_executor = MySqlGameTickExecutor::new(executor);
        let effects = [
            GameTickEffect::SavePlayer { user_id: 7 },
            GameTickEffect::ResolveServerIp,
        ];

        let result = game_tick_executor.execute_effects(&effects).unwrap();
        let executor = game_tick_executor.into_executor();

        assert!(result.results().is_empty());
        assert!(executor.plans().is_empty());
    }
}
